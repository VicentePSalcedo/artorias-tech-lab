#!/usr/bin/env nu

# Artorias Tech Lab - Continuous Delivery Script for AWS Lightsail
# Usage: nu deploy.nu

def main [] {
    let project_root = ($env.FILE_PWD | path dirname)
    cd $project_root

    print "[*] Starting deployment for Artorias Tech Lab..."

    # 1. Compile the target in --release mode
    print "[*] Compiling Leptos project in release mode..."
    cargo leptos build --release

    if $env.LAST_EXIT_CODE != 0 {
        print "[!] Build failed. Aborting deployment."
        exit 1
    }

    print "[+] Build successful."

    print "[*] Patching ELF interpreter for Ubuntu target..."
    patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 target/release/artorias-tech-lab

    # 2. Configuration for AWS Lightsail
    let remote_user = "ubuntu"
    let remote_host = "100.31.228.128"
    let remote_dir = "/var/www/artorias-tech-lab"
    
    print "[*] Synchronizing files to Lightsail instance..."
    
    # We use rsync to push the binary and the site output.
    # Note: Requires SSH keys to be configured for AWS Lightsail instance.
    
    # Sync site (CSS, JS, WASM assets)
    print "[*] Syncing site assets..."
    ^rsync -avz -e "ssh -i ~/.ssh/id_ed25519 -o StrictHostKeyChecking=no" --delete target/site/ $"($remote_user)@($remote_host):($remote_dir)/site/"
    
    # Sync server binary
    print "[*] Syncing server binary..."
    ^rsync -avz -e "ssh -i ~/.ssh/id_ed25519 -o StrictHostKeyChecking=no" target/release/artorias-tech-lab $"($remote_user)@($remote_host):($remote_dir)/"

    # 3. Recycle the systemd service
    print "[*] Restarting systemd service..."
    ^ssh -i ~/.ssh/id_ed25519 -o StrictHostKeyChecking=no $"($remote_user)@($remote_host)" "sudo systemctl restart artorias-tech-lab"

    print "[+] Deployment complete! Your system is now live."
}
