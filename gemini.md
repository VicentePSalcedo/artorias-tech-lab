# Gemini Development & Deployment Workflow

This document outlines the standard workflow for developing, integrating, testing, and deploying changes to the **Artorias Tech Lab** platform. All future development must follow this lifecycle.

---

## 1. Branching Strategy

* **`main`**: The production-ready branch. It represents the codebase currently deployed to the live server.
* **`dev`**: The active integration branch. All features, style updates, and refactoring should occur here or in short-lived feature branches branched from `dev`.

---

## 2. Development & Integration Lifecycle

### Step 1: Work on the `dev` Branch
Before starting any work, ensure you are on the `dev` branch and it is up to date:
```bash
git checkout dev
git pull origin dev
```

### Step 2: Validate Changes Locally
Ensure the codebase compiles successfully after making changes:
```bash
cargo check
```
If you are modifying templates or styles, run `cargo leptos build` to verify front-end compilation and CSS generation.

### Step 3: Commit to `dev`
Stage and commit your changes locally with a descriptive commit message:
```bash
git add .
git commit -m "feat/style/fix: description of changes"
```

### Step 4: Merge to `main`
Once a set of changes is verified and ready for production, merge them into `main`:
```bash
# Checkout main and merge dev
git checkout main
git merge dev

# Sync dev branch with main to ensure they match exactly
git checkout dev
git reset --hard main
git checkout main
```

### Step 5: Push to Remote (GitHub)
Push the updated `main` and `dev` branches to the remote repository on GitHub:
```bash
git push origin main && git push origin dev
```

---

## 3. Production Deployment

The project utilizes a Continuous Delivery script written in Nushell ([deploy.nu](file:///home/sintra/Repos/artorias-tech-lab/deploy.nu)) to build and deploy to AWS Lightsail.

### Step 1: Run the Deployment Script
Run the deployment script from your local machine:
```bash
nu deploy.nu
```
This script will automatically:
1. Compile the Leptos project in release mode (`cargo leptos build --release`).
2. Synchronize compiled site assets (`target/site/`) to the remote Lightsail folder (`/var/www/artorias-tech-lab/site/`) via `rsync`.
3. Synchronize the compiled server binary (`target/release/artorias-tech-lab`) to the server.
4. Restart the remote `systemd` service (`artorias-tech-lab`).

### Step 2: Post-Deployment Verification
Check the status of the remote service to confirm it restarted successfully and is listening on port `3000`:
```bash
ssh -i ~/.ssh/id_rsa ubuntu@100.31.228.128 "sudo systemctl status artorias-tech-lab"
```

If the systemd service file or environment variables were modified, make sure to reload the systemd configuration daemon before checking:
```bash
ssh -i ~/.ssh/id_rsa ubuntu@100.31.228.128 "sudo systemctl daemon-reload && sudo systemctl restart artorias-tech-lab"
```

### Step 3: Health Check
Run a curl command to verify the server is responding to HTTP traffic correctly:
```bash
curl -I https://artoriastechlab.com
```
*(Note: Direct IP access to `http://100.31.228.128` is blocked with a 403 Forbidden error for security, so testing must be done via the domain name).*
