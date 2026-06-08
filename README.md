This is the master migration and implementation blueprint to move **Artorias Tech Lab** to an elite, custom Rust-backed infrastructure.

Since you are executing this using **Antigravity CLI** and agentic subagents, this plan is explicitly structured as an engineering specification that you can feed directly into your LLM workspace context.

---

## Phase 1: Environment Setup & Agent Configuration

Before writing a line of code, we must configure your agentic tools so they understand the unique primitives of Leptos and can interact with your workspace without hallucinations.

### 1. Initialize the Leptos Stack

Instruct your primary Antigravity agent to spin up a full-stack Leptos project using the `cargo-leptos` build tool with Tailwind CSS integration.

```bash
cargo install cargo-leptos
cargo leptos new --ssr artorias-tech-lab
cd artorias-tech-lab
```

### 2. Configure MCP Servers in Antigravity

To prevent your AI agents from hallucinating outdated JavaScript frameworks or generating invalid Rust view macros (`view! {}`), and to enable Nushell scripting, you must register the MCP servers within your Antigravity orchestration layout.

Add the server definition to your local `mcp_config.json`:

```json
{
  "mcpServers": {
    "leptos-mcp-server": {
      "command": "npx",
      "args": ["-y", "@leptos-mcp/server"],
      "env": {
        "WORKSPACE_ROOT": "./",
        "LEPTOS_VERSION": "0.8"
      },
      "settings": {
        "autoLintOnSave": true,
        "validateMacros": true
      }
    },
    "nushell-mcp": {
      "command": "npx",
      "args": ["-y", "nushell-mcp"]
    }
  }
}
```

> **Agent Directive:** *"Verify that the `leptos-mcp-server` and `nushell-mcp` are active inside the workspace tool counts. Utilize the Leptos server to lint all reactive signals and component properties, and use the Nushell MCP for native scripting execution."*

---

## Phase 2: Infrastructure Provisioning (The Server)

To maintain absolute control over the environment, we will sidestep managed PaaS providers and provision a raw Linux node.

* **Target Environment:** AWS Lightsail instance running Ubuntu 24.04 LTS.
* **The Stack:** Bare-metal binary managed via `systemd`, reverse-proxied by **Nginx** or **Caddy** to handle automatic SSL termination.

### DNS Strategy (Keeping GoDaddy as Registrar)

You will not change your domain registrar. Once your infrastructure server is up:

1. Copy the target server's static public IPv4 address.
2. Log into the GoDaddy DNS Management Console for `artoriastechlab.com`.
3. Update the **@ A Record** to point to your new server IP.
4. Update or add a **www CNAME Record** pointing to `@`.

---

## Phase 3: UI Architecture & Visual Styling

The subagents will build a modern, ultra-sharp dark-mode aesthetic utilizing fine grid layouts instead of heavy image assets.

### 1. Global Tailwind & Theme Configuration

Configure `tailwind.config.js` to handle the specialized developer theme using standard Tailwind CLI (decoupled from the Rust build):

* **Backgrounds:** Deep charcoals and slates (`bg-slate-950`, `bg-zinc-950`).
* **Borders:** Low-contrast grid dividers (`border-slate-800/60`).
* **Accents:** High-vibrancy, precise highlights (`text-cyan-400`, `text-emerald-400`).

### 2. Layout Components to Generate

* `AppLayout`: A global component featuring a sticky, glassmorphism header with background blur (`backdrop-blur-md bg-slate-950/70`).
* `BentoGrid`: An asymmetrical layout component for displaying content blocks seamlessly across screen sizes.
* `TerminalMock`: A pure CSS/Tailwind interactive code block component to simulate real system compilations on the homepage.

---

## Phase 4: Content & Component Engineering Blueprint

```
                     +---------------------------+
                     |  Artorias Tech Lab Index  |
                     +-------------+-------------+
                                   |
         +-------------------------+-------------------------+
         |                         |                         |
+--------v--------+       +--------v--------+       +--------v--------+
|  whoami Bento   |       |  Renivel SaaS   |       | Aggressive Case |
| Chief Architect |       | Dual-Track Card |       |   Study Block   |
+-----------------+       +-----------------+       +-----------------+

```

### 1. The Home Page (`src/pages/index.rs`)

Your agents will build a single, highly impact-driven index page using a high-end Bento Grid format containing the following structural modules:

#### Module A: The "whoami" Chief Architect Card

* **Purpose:** Break down the fake agency facade; sell the solo developer advantage.
* **Copy Tone:** Authoritative, direct, friction-free.
* **Layout:** A prominent bento block featuring a minimalist vector graphic or GitHub contribution lattice alongside your personal engineering pitch.

#### Module B: Dual-Track Renivel SaaS Feature

* **Contractor Facing UI:** Highlight the *Dusty Truck Worker App*, *Real-Time Profitability Financial Health Bar*, and *6-Digit Passwordless Onboarding*.
* **Enterprise Consulting Facing UI:** A toggleable code dropdown labeled `[inspect_infrastructure.nu]` that reveals a clean markdown rendering of the multi-tenant architecture, Terraform pipelines, and AWS Secrets Manager integration.
* **Visual:** A single, clean browser iframe component wrapping a high-res screenshot of the app dashboard.

#### Module C: Aggressive Signs "IT Rescue" Case Study

* **The Contrast:** Highlight the shift from an unresponsive legacy provider (Windows Server 2012) to true operational autonomy via modern local NAS deployment.
* **Visual:** A grayscale SVG placeholder for the Aggressive Signs logo that transitions to full brand color on mouse hover.
* **Testimonial Element:**
> *[Placeholder: Insert Jason's 2-sentence response regarding server ownership and zero scheduling latency here]*



### 2. The Client Services Page (`src/pages/services.rs`)

Consolidate your previous multi-page setup into a single, high-margin consulting menu.

* **Digital Foundation Package ($2,500+):** Full-stack, high-performance web systems optimized for lead conversion.
* **Custom Web Applications ($5,000+):** Bespoke software automation, secure user portals, and internal workflows.
* **Managed Infrastructure & Automation (Retainer):** Strategic Fractional CTO management covering Google Workspace, secure local NAS/VPC management, and custom cross-platform workflow integrations. (Explicitly omits synchronous employee password helpdesk support).

---

## Phase 5: Deployment via Nushell Script

Once your Antigravity agent compiles the local Leptos project successfully through the MCP linting filters, deploy it utilizing a continuous delivery Nushell script.

### 1. Local Build & Optimization

Instruct your agent to write a `deploy.nu` script that will:

1. Compile the target in `--release` mode.
2. Optimize the WASM binary sizes.
3. Sync the `target/site` output alongside the compiled binary using `rsync` or the `aws` CLI.

### 2. Direct Push to Lightsail

The `deploy.nu` script will push updates directly to your VPS and recycle the running `systemd` system service with minimal downtime.

> **Agent Directive:** *"Leverage the `nushell-mcp` execution context to write and test the `deploy.nu` script. Once complete, wait for user authorization before triggering a live push to the AWS Lightsail node."*
