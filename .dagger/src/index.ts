import {
  argument,
  Container,
  File,
  dag,
  Directory,
  func,
  object,
  Secret,
} from "@dagger.io/dagger";

@object()
export class MyceliumBuild {
  source: Directory;

  constructor(
    @argument({
      defaultPath: ".",
      ignore: [
        "**/node_modules",
        "**/.git",
        "**/.github",
        "**/.husky",
        "**/.vscode",
        // rust
        "**/target",
        "**/artifact",
        // browser
        "**/.swc",
        "**/.netlify",
        // e2e
        "**/test-results",
        "**/template-tests",
        "**/playwright-report",
        "**/tmp",
        "**/.temp",
        "**/.DS_Store",
        "**/.vscode",
        "**/dist",
        "**/assets_tmp",
        "**/build",
        "**/.env",
        "**/.envrc",
      ],
    }) source: Directory,
  ) {
    this.source = source;
  }

  /**
   * Container for building the Scala backend
   */
  containerBackend(): Container {
    const src = this.source;

    return dag
      .container()
      .from("sbtscala/scala-sbt:eclipse-temurin-alpine-24.0.1_9_1.11.7_3.7.4")
      .withMountedCache("/root/.sbt", dag.cacheVolume("sbt-cache"))
      .withMountedCache("/root/.ivy2", dag.cacheVolume("ivy2-cache"))
      .withMountedCache("/root/.cache/coursier", dag.cacheVolume("scala-coursier-cache"))
      .withDirectory("/workspace", src.directory("backend").filter({ include: ["project/build.properties", "project/plugins.sbt", "project/*.scala", "src/**", ".sbtopts", ".scalafmt.conf", "build.sbt"]}))
      .withWorkdir("/workspace");
  }

  /**
   * Build the Scala backend
   */
  async buildBackend_(backendJar: File, arch: string): Promise<Container> {
    const src = this.source;
    const backendNative = dag
      .container({ platform: arch as any })
      .from("ghcr.io/graalvm/native-image-community:25-muslib")
      .withWorkdir("/workspace")
      .withFile("/workspace/backend.jar", backendJar)
      .withFile("/workspace/reflection-config.json", src.file("backend/reflection-config.json"))
      .withExec([
        "native-image",
        "--no-fallback",
        "--static",
        "--libc=musl",
        "--install-exit-handlers",
        "--initialize-at-build-time",
        "--report-unsupported-elements-at-runtime",
        "-H:IncludeResources=META-INF/services/.*,placeholder\\.png,logback\\.xml",
        "-H:ReflectionConfigurationFiles=reflection-config.json",
        "--enable-url-protocols=http,https",
        "--add-opens=java.base/java.nio=ALL-UNNAMED",
        "--add-opens=java.base/jdk.internal.misc=ALL-UNNAMED",
        "--add-opens=java.base/jdk.internal.ref=ALL-UNNAMED",
        "--trace-class-initialization=ch.qos.logback.classic.Logger",
        "--trace-object-instantiation=ch.qos.logback.core.AsyncAppenderBase$Worker,java.util.Random",
        "--initialize-at-run-time=org.postgresql,org.postgresql.Driver,org.postgresql.jdbc",
        "-cp",
        "/workspace/backend.jar",
        "co.mycelium.Main",
        "/workspace/backend"
      ])
      .withExec(["chmod", "+x", "/workspace/backend"]);

    return dag
      .container({ platform: arch as any })
      .from("gcr.io/distroless/cc")
      .withWorkdir("/app")
      .withExposedPort(8080)
      .withFile("/app/backend", backendNative.file("/workspace/backend"))
      .withEntrypoint(["/app/backend"])
      .sync();
  }

  @func()
  async publishBackendGraal(password: Secret, tag?: string): Promise<string> {
      // const platforms = ["linux/amd64", "linux/arm64"];
      const platforms = ["linux/arm64"];
      const backendJar = await this.containerBackend()
        .withExec(["sbt", "assembly"])
        .file("target/scala-3.7.4/backend-assembly-1.0.jar");

      const containers = await Promise.all(platforms.map(x => this.buildBackend_(backendJar, x)));

      return dag
        .container()
        .withRegistryAuth("docker.io", "markdj", password)
        .publish(`markdj/mycelium-backend:${tag ?? "latest"}`, { platformVariants: containers });
  }

  @func()
  publishBackend(password: Secret, tag?: string): Promise<string> {
    return this
      .containerBackend()
      .withEnvVariable("JIB_TARGET_IMAGE_USERNAME", "markdj")
      .withSecretVariable("JIB_TARGET_IMAGE_PASSWORD", password)
      .withExec(["sbt", `-DimageTag=${tag ?? "latest"}`, "jibImageBuild"])
      .stdout();
  }

  @func()
  createClient(generator: string, name: string): Directory {
  
    const openapi = this.containerBackend().withExec(["sbt", 'runMain co.mycelium.OpenApiGenerator']).file("openapi.json");
    const generated = dag.container().from("openapitools/openapi-generator-cli:v7.14.0")
      .withFile("/tmp/openapi.json", openapi)
      .withExec(["mkdir", "-p", "/out"])
      .withExec([
        "/usr/local/bin/docker-entrypoint.sh", "generate",
          "-i", "/tmp/openapi.json",     // input file
          "-g", generator,
          `--additional-properties=packageName=${name},supportMiddleware=true`,
          "-o", "/out"             // output directory inside container
      ]);

    return generated.directory("/out");
  }

  /**
   * Test the Scala backend
   */
  @func()
  async testBackend(): Promise<string> {
    return this.containerBackend()
      .withExec(["sbt", "test"])
      .stdout();
  }

  /**
   * Test the Scala backend
   */
  @func()
  async buildBackend(): Promise<string> {
    return this.containerBackend()
      .withExec(["sbt", "compile"])
      .stdout();
  }

  /**
   * Container for building the Tauri app
   */
  containerApp(): Container {
    const src = this.source;

    let container = dag
      .container()
      .from("rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0")
      .withExec(["sh", "-c", "echo 'deb [check-valid-until=no] http://snapshot.debian.org/archive/debian/20240701T000000Z bookworm main' > /etc/apt/sources.list"])
      .withExec(["sh", "-c", "apt-get install -y curl=7.88.1-10+deb12u12 ca-certificates=20230311+deb12u1 gnupg=2.2.40-1.1"])
      .withExec(["sh", "-c", "curl -fsSL https://deb.nodesource.com/setup_20.x | bash -"])
      .withExec([
        "sh", "-c",
        "apt-get update && apt-get install -y libwebkit2gtk-4.1-dev build-essential wget file libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev xdg-utils nodejs"
      ])
      .withMountedCache("/usr/local/cargo/registry", dag.cacheVolume("app-cargo-registry"))
      .withMountedCache("/usr/local/cargo/git", dag.cacheVolume("app-cargo-git"))
      .withMountedCache("/root/.npm", dag.cacheVolume("app-npm"))
      .withMountedCache("/workspace/node_modules", dag.cacheVolume("app-node-modules"))
      .withMountedCache("/workspace/src-tauri/target", dag.cacheVolume("app-tauri-target"))
      .withDirectory("/workspace", src.directory("app").filter({include: ["public/**", "src/**", "src-tauri/capabilities/**", "src-tauri/icons/**", "src-tauri/src/**", "src-tauri/build.rs", "src-tauri/Cargo.toml", "src-tauri/Cargo.lock", "src-tauri/tauri.conf.json", "index.html", "package-lock.json", "package.json", "postcss.config.js", "tailwind.config.js", "tsconfig.json", "tsconfig.node.json", "vite.config.ts"]}))
      .withWorkdir("/workspace");

    return container;
  }

  /**
   * Build the Tauri app for a specific platform
   */
  @func()
  async buildApp(): Promise<string> {
    return this.containerApp()
      .withExec(["bash", "-c", "npm install && npm run tauri build"])
      .stdout();
  }

  /**
   * Container for building the central component with dbus support
   */
  containerCentral(arch: string): Container {
    const src = this.source;

    return dag
    .container({ platform: arch as any })
      .from("rust:1.88-bookworm")
      .withExec(["sh", "-c", "echo 'deb [check-valid-until=no] http://snapshot.debian.org/archive/debian/20240701T000000Z bookworm main' > /etc/apt/sources.list"])
      .withExec(["sh", "-c", "apt-get update && apt-get install -y libdbus-1-3=1.14.10-1~deb12u1 libdbus-1-dev=1.14.10-1~deb12u1 dbus=1.14.10-1~deb12u1 pkg-config=1.8.1-1"])
      .withMountedCache("/root", dag.cacheVolume("edge-central-root"))
      .withMountedCache("/usr/local/cargo/registry", dag.cacheVolume("edge-central-cargo-registry"))
      .withMountedCache("/usr/local/cargo/git", dag.cacheVolume("edge-central-cargo-git"))
      .withMountedCache("/workspace/edge-protocol/target", dag.cacheVolume("edge-protocol-target"))
      .withDirectory("/workspace/edge-central", src.directory("edge-central").filter({include: ["src/**", "migrations/**", "Cargo.toml", "Cargo.lock"]}))
      .withDirectory("/workspace/edge-client-backend", src.directory("edge-client-backend").filter({include: ["src/**", "Cargo.toml", "Cargo.lock"]}))
      .withDirectory("/workspace/edge-protocol", src.directory("edge-protocol").filter({include: ["src/**", "Cargo.toml", "Cargo.lock"]}))
      .withWorkdir("/workspace/edge-central");
  }

  /**
   * Container for building the peripheral component with xtensa toolchain
   */
  containerPeripheral(arch: string): Container {
    const src = this.source;

    return dag
      .container({ platform: arch as any })
      .from("rust:1.88-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0")
      .withExec([
        "sh", "-c",
        "apt-get update && apt-get install -y gcc build-essential curl pkg-config"
      ])
      .withMountedCache("/root", dag.cacheVolume("edge-peripheral-root"))
      .withMountedCache("/usr/local/cargo/registry", dag.cacheVolume("edge-peripheral-cargo-registry"))
      .withMountedCache("/usr/local/cargo/git", dag.cacheVolume("edge-peripheral-cargo-git"))
      .withMountedCache("/usr/local/rustup/toolchains/esp", dag.cacheVolume("edge-peripheral-rustup-toolchain-esp"))
      .withMountedCache("/workspace/edge-peripheral/target", dag.cacheVolume("edge-peripheral-target-folder"))
      .withExec(["sh", "-c", "cargo install espup --locked --version 0.15.1"])
      .withExec(["sh", "-c", "espup install -t esp32"])
      .withDirectory("/workspace/edge-peripheral", src.directory("edge-peripheral").filter({include: [".cargo/config.toml", "src/**", "build.rs", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "template.yaml"]}))
      .withDirectory("/workspace/edge-protocol", src.directory("edge-protocol").filter({include: ["src/**", "Cargo.toml", "Cargo.lock"]}))
      .withWorkdir("/workspace/edge-peripheral")
  }

  /**
   * Execute a command in the peripheral container with ESP environment
   */
  execPeripheralWithEnv(arch: string, command: string): Container {
    return this.containerPeripheral(arch)
      .withExec(["bash", "-c", `source /root/export-esp.sh && ${command}`]);
  }

  /**
   * Build the central component
   */
  @func()
  buildCentral(@argument() arch: string = "linux/arm64"): File {
    return this.containerCentral(arch)
      .withExec(["cargo", "build", "--release"]).file("target/release/main");
  }

  /**
   * Test the central component
   */
  @func()
  async testCentral(@argument() arch: string = "linux/arm64"): Promise<string> {
    return this.containerCentral(arch)
      .withExec(["cargo", "test"]).stdout();
  }

  /**
   * Build the peripheral component for ESP32
   */
  @func()
  async buildPeripheral(@argument() arch: string = "linux/amd64"): Promise<string> {
    return this.execPeripheralWithEnv(arch, "cargo build --release").stdout();
  }

  @func()
  async ci(@argument() arch: string = "linux/amd64") {

    await Promise.all([
      this.buildPeripheral(arch),
      this.testCentral(),
      this.buildBackend()
    ]);

    return "CI pipeline completed successfully";
  }
}