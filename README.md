# DashBoard

A car dashboard built with Tauri, designed for easy deployment on a Raspberry Pi or similar devices.

⚠️ **Note:** In release mode, the app will auto-start and launch in full screen.

## Tech Stack

* **Frontend:** React, TailwindCSS, Shadcn/UI
* **Backend:** Rust, Tokio-Serial, Rumqttc
* **Package Manager:** Bun

## Features

1. Read real-time data from serial ports using `tokio-serial`.
2. Upload driving data to your MQTT server.

## Deployment

```bash
git clone https://github.com/JianLin416/dashboard.git
cd dashboard

# Rename environment template
mv .env.template.bak .env

# Install dependencies
bun install
```

Then edit your settings in the `.env` file.

### Run in Debug Mode

```bash
bun tauri dev
```

### Build Release Package

```bash
bun tauri build
```
