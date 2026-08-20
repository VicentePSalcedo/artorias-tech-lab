# Artorias Tech Lab

The official website and client portal for **Artorias Tech Lab** — an IT consultancy specializing in high-performance web systems, custom workflow automation, and managed IT infrastructure for small to medium-sized businesses.

## Tech Stack

This project is built using an elite, custom Rust-backed infrastructure:

* **Frontend & Backend:** [Leptos](https://leptos.dev/) (Full-stack Rust web framework using Server-Side Rendering and WebAssembly hydration)
* **Styling:** [Tailwind CSS](https://tailwindcss.com/)
* **Environment Management:** [Nix](https://nixos.org/) (Flakes)
* **Build Tool:** `cargo-leptos`
* **Deployment Automation:** [Nushell](https://www.nushell.sh/)

## Development Workflow

This repository uses a Nix Flake to ensure a completely reproducible development environment. You do not need to install Rust or Node.js manually on your system.

### 1. Enter the Dev Shell
Enter the development environment which provides `cargo`, `rustc`, `cargo-leptos`, `npm`, and `binaryen` (for WASM optimization).
```bash
nix develop
```

### 2. Install CSS Dependencies
Install the Tailwind CSS dependencies (only required once):
```bash
npm install
```

### 3. Run the Development Server
Start the development server with hot-reloading enabled. This will watch your `.rs` files and `main.scss` for changes.
```bash
cargo leptos watch
```
The site will be available locally at `http://localhost:3000`.

## Infrastructure & Deployment

The application runs as a bare-metal binary on an AWS Lightsail Ubuntu instance. It binds to `127.0.0.1:3000` and is reverse-proxied by **Caddy** which automatically handles HTTPS certificates via Let's Encrypt. 

The server is connected to a private Tailscale VPN network via Headscale, ensuring secure, private access to infrastructure.

### Deployment Pipeline

Deployment is handled entirely by a custom Nushell script (`scripts/deploy.nu`). The script automates building the release binary, optimizing the WASM output, syncing assets via SSH/rsync, patching the ELF interpreter for Ubuntu compatibility, and restarting the `systemd` service.

To deploy to production, run:
```bash
nu scripts/deploy.nu
```

*Note: The deployment script assumes you have `id_ed25519` SSH key access to the `ubuntu@100.31.228.128` server.*

### Bleeding Edge Updates

To forcefully update your entire tech stack (Nix environment, Rust crates, and NPM packages) to the absolute newest versions, run:
```bash
nu scripts/update.nu
```

## Application Architecture

- **`src/app.rs`**: Global application router and HTML shell, including the smooth-scrolling `Lenis` integration.
- **`src/pages/`**: Contains the primary route views (`index.rs`, `services.rs`, `founder.rs`, `contact.rs`).
- **`src/components/`**: Reusable UI components like the Bento Grid layout and navigation header.
- **`style/main.scss`**: Global stylesheet containing custom animations and Tailwind imports.
