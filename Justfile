sbc_user := env("SBC_USER", "root")
sbc_host := env("SBC_HOST", "dietpi")

# Load backend/.env into the environment for all recipes (PREPROV_USER_ID, etc.).
# Shell env vars still take precedence over the file.
set dotenv-load
set dotenv-filename := "backend/.env"

backend_image_tag := env("BACKEND_IMAGE_TAG", "dev")
backend_image := "markdj/mycelium-backend:" + backend_image_tag

[doc('Build central for linux/aarch64')]
central-build-aarch64:
    cd edge-central && cargo build --release --target aarch64-unknown-linux-musl

[doc('Build central for local run')]
central-run-local:
    cd edge-central && cargo run

[doc("Build central and transfer it to the target and restart and show its output")]
central-build-roll cfg host:
    just central-build-aarch64
    # just transfer-file {{ cfg }} {{ host }} "edge-central/.env" "~/.env"
    just transfer-file {{ cfg }} {{ host }} "edge-central/target/aarch64-unknown-linux-musl/release/main" "~/central"
    # just execute-remote {{ cfg }} {{ host }} "sudo systemd-run --pty --uid=$(id -u) --gid=$(id -g) --same-dir --setenv RUST_LOG=info --setenv PATH --property "AmbientCapabilities=CAP_NET_ADMIN" ~/central"

[doc("Build central and transfer it to the dietpi")]
central-build-roll-dietpi:
    just central-build-roll "~/.ssh/dietpi-ssh-config" {{ sbc_host }}

[doc("Build central and transfer it to the virtual machine")]
central-build-roll-virtmachine:
    just central-build-roll "/Users/markdejong/.ssh/vagrant-ssh-config" "default"

central-build-roll2 cfg host:
    dagger -c "build-central | export edge-central/target/central"
    just transfer-file {{ cfg }} {{ host }} "edge-central/.env" "~/.env"
    just transfer-file {{ cfg }} {{ host }} "edge-central/target/central" "~/central"

central-build-roll-virtmachine2:
    just central-build-roll2 "/Users/markdejong/.ssh/vagrant-ssh-config" "default"

edge-peripheral-clean:
    espflash erase-flash --port /dev/tty.usbserial-11103

edge-peripheral-build:
    . ~/export-esp.sh && cd edge-peripheral && cargo build --target xtensa-esp32-none-elf --release

edge-peripheral-flash:
    . ~/export-esp.sh && cd edge-peripheral && cargo run --target xtensa-esp32-none-elf --release

execute-remote cfg host cmd:
    ssh -F {{ cfg }} {{ host }} '{{ cmd }}'

transfer-file cfg host from to:
    scp -F {{ cfg }} {{ from }} {{ host }}:{{ to }}

gen-client lang name:
    dagger -c "create-client {{lang}} {{name}} | export clients/{{name}}"

gen-client-rs:
    dagger -c "create-client rust edge-client-backend | export clients/rs"

gen-client-ts-axios:
    just gen-client typescript-axios ts-axios

[doc('Build backend Docker image locally via Jib (loads into local Docker daemon)')]
backend-build tag='dev':
    cd backend && sbt -DimageTag={{tag}} jibImageBuild

[doc('Start local stack: timescaledb, minio, backend, preprov')]
backend-compose-up *args:
    #!/usr/bin/env bash
    set -eo pipefail
    cd backend
    export BACKEND_IMAGE="${BACKEND_IMAGE:-{{backend_image}}}"
    export PREPROV_USER_ID="${PREPROV_USER_ID:-local-dev-user}"
    exec docker compose up --build "$@"

[doc('Build backend via Jib and start local docker-compose stack')]
backend-dev *args:
    #!/usr/bin/env bash
    set -eo pipefail
    tag="dev-$(date -u +%Y%m%d-%H%M%S)"
    echo "Building backend image markdj/mycelium-backend:${tag}"
    cd backend
    sbt -DimageTag="${tag}" jibImageBuild
    export BACKEND_IMAGE="markdj/mycelium-backend:${tag}"
    export PREPROV_USER_ID="${PREPROV_USER_ID:-local-dev-user}"
    exec docker compose up --build --force-recreate "$@"

[doc('Stop local backend stack')]
backend-down *args:
    #!/usr/bin/env bash
    set -eo pipefail
    cd backend
    export BACKEND_IMAGE="${BACKEND_IMAGE:-{{backend_image}}}"
    export PREPROV_USER_ID="${PREPROV_USER_ID:-local-dev-user}"
    exec docker compose down "$@"

