## mycelium

![mycelium logo](/logo-mycelium.jpg)

The mycelium project is a comprehensive plant monitoring system that automatically monitors
environmental conditions in households and gardens using IoT edge devices. The
system consists of five main components:

## Architecture

- **edge-peripheral** - ESP32-based sensor device that collects environmental
  measurements (temperature, humidity, light, battery level, soil moisture, tank levels) using deep sleep
  and Bluetooth Low Energy (BLE) for power optimization
- **edge-central** - Rust-based central hub that continuously scans for peripheral devices,
  collects measurements, manages time synchronization, and handles local data
  persistence with cloud synchronization
- **edge-protocol** - Shared Rust protocol library that defines the communication
  format between peripheral and central devices using protobuf over BLE GATT
- **backend** - Scala-based cloud backend service with REST API, PostgreSQL database,
  Auth0 authentication, and comprehensive plant management features
- **app** - Cross-platform desktop application built with Tauri (Rust + React)
  that provides a user interface for monitoring and managing the plant monitoring
  system

## Getting Started

### Prerequisites

- Rust toolchain with embedded targets (1.88+)
- ESP32 development environment with espup
- Scala 2.13+ and sbt (for backend service)
- PostgreSQL database (for backend)
- Node.js 20+ (for Tauri desktop app)
- Auth0 account for authentication
- Dagger CLI (optional, for local CI testing)

### Configuration

#### Edge Central Configuration

Copy `edge-central/.env.sample` to `edge-central/.env` and configure:

```bash
# Configuration uses APP prefix with hierarchical structure
APP.DATABASE_URL=sqlite://mycelium.db
APP.PERIPHERAL_SYNC_MODE=ble  # or 'random' for testing
APP.ONBOARDING_STRATEGY=local  # or 'ble'
APP.AUTH0.DOMAIN=your-domain.auth0.com
APP.AUTH0.CLIENT_ID=your-client-id
APP.AUTH0.CLIENT_SECRET=your-client-secret
APP.AUTH0.SCOPE=offline_access
APP.AUTH0.AUDIENCE=your-audience
APP.WIFI.SSID=your-wifi-ssid
APP.WIFI.PASSWORD=your-wifi-password
```

The edge-central configuration system uses the `config` crate with environment variable support. All settings use the `APP` prefix with dot notation for hierarchical configuration.

#### Backend Configuration

The Scala backend uses environment variables and application.conf for configuration:

- **Database**: PostgreSQL connection via JDBC
- **Authentication**: Auth0 JWT validation
- **CORS**: Configured for cross-origin requests
- **File Storage**: Local file system for avatar images
- **Logging**: Logback with structured logging

### Auth0 Configuration

The project uses Auth0 for authentication. Default values for Auth0 domain and
client ID are provided in the code, but you should configure your own Auth0
application for production use:

1. Create an Auth0 application in your Auth0 dashboard
2. Configure the application for Device Authorization Flow
3. Set the appropriate audience and scopes
4. Update the `.env` file with your Auth0 credentials

The application uses the dotenv crate to load environment variables from the
`.env` file at runtime.
## Build System & CI

The project uses [Dagger](https://dagger.io/) for containerized builds and CI/CD. The build configuration is defined in TypeScript at `.dagger/src/index.ts`.

### Local Development

```bash
# Build edge-central component
cd edge-central
cargo build --release

# Build edge-peripheral component (requires ESP toolchain)
cd edge-peripheral
# source some paths needed for xtensa toolchain
. ~/export-esp.sh
cargo build --release

# Build backend service
cd backend
sbt compile
sbt run  # Starts server on port 8080

# Build desktop app
cd app
npm install
npm run tauri build

# Run tests
cd edge-central && cargo test
cd backend && sbt test
```

### Dagger CI

The CI pipeline builds all components in isolated containers:

```bash
# Run full CI pipeline locally, you can include --arch=linux/arm64 or --arch=linux/amd64 depending on your platform
dagger call ci

# Build individual components
dagger call build-central
dagger call build-peripheral --arch=linux/arm64
dagger call build-backend
dagger call build-app

# Run tests
dagger call test-central
dagger call test-backend
```

### GitHub Actions

The project uses GitHub Actions with Dagger for CI. The workflow:
1. Builds the edge-central component with dbus support
2. Builds the edge-peripheral component with ESP32 toolchain
3. Builds the Scala backend service
4. Builds the Tauri desktop application
5. Runs all tests across components

The CI pipeline is configured to run on push events, ensuring code quality across all components.

## Project Structure

```
mycelium/
├── edge-central/          # Central hub (Rust)
│   ├── src/bin/          # Binary executables
│   ├── migrations/       # SQLite schema migrations
│   └── Cargo.toml        # Dependencies and configuration
├── edge-peripheral/       # ESP32 sensor device (Rust)
│   ├── src/              # Firmware source code
│   ├── .cargo/           # Cargo configuration for ESP32
│   └── rust-toolchain.toml # Rust toolchain specification
├── edge-protocol/         # Shared protocol library (Rust)
│   └── src/lib.rs        # Protocol definitions and protobuf encoding
├── backend/               # Cloud backend service (Scala)
│   ├── src/main/scala/   # Scala source code
│   ├── src/main/resources/migrations/ # PostgreSQL Flyway migrations
│   └── build.sbt         # SBT build configuration
├── app/                   # Desktop application (Tauri + React)
│   ├── src/              # React frontend source
│   ├── src-tauri/        # Tauri Rust backend
│   └── package.json      # Node.js dependencies
└── .dagger/              # CI/CD pipeline (TypeScript)
    └── src/index.ts      # Dagger build configuration
```

## Development Notes

### Configuration System

The project uses a hierarchical configuration system with the following features:

- **Environment Variables**: All configuration uses `APP.` prefix with dot notation
- **Type Safety**: Configuration is deserialized into strongly-typed structs
- **Validation**: Missing required values cause startup failures
- **Flexibility**: Supports multiple onboarding strategies and sync modes

Configuration enums:
- `OnboardingStrategy`: `ble` or `local` (for testing)
- `PeripheralSyncMode`: `ble` or `random` (for testing)

### Known Issues

- The project requires specific Rust version (1.88+) for ESP32 compatibility
- ESP toolchain setup requires manual environment sourcing
- Some git dependencies may require specific revisions for compatibility

## API Documentation

### Backend REST API

The Scala backend provides a comprehensive REST API for plant monitoring:

**Authentication**: Auth0 JWT Bearer tokens required for all endpoints

**Base URL**: `http://localhost:8080/api` (development)

**Data Models**:
- **Station**: Plant metadata, location, watering schedule
- **StationMeasurement**: Sensor readings (temperature, humidity, lux, soil/tank pF, battery)
- **WateringSchedule**: Interval-based or threshold-based watering logic
- **StationLog**: Event history (watering, schedule changes)

### Edge Protocol

The edge devices communicate using a custom TLV (Type-Length-Value) protocol over BLE:
- Time synchronization between central and peripheral
- Compressed time series data transmission

## Desktop Application

The project includes a cross-platform desktop application built with Tauri that provides a graphical interface for the plant monitoring system.

### Features

- Real-time monitoring of connected edge devices
- Historical data visualization with interactive charts
- Plant management (add, edit, delete stations)
- Watering schedule configuration (interval/threshold based)
- Authentication via Auth0
- Cloud synchronization with backend API

### Development

```bash
cd app
npm install
npm run dev  # Start development server
npm run tauri dev  # Start Tauri development mode
```

### Testing

The project includes comprehensive tests across all components:

```bash
# Run Rust tests (edge-central, edge-protocol)
cargo test

# Run Scala backend tests
cd backend && sbt test

# Run tests for specific component
cargo test -p edge-central
```