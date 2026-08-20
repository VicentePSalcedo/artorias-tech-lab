#!/usr/bin/env nu

# Artorias Tech Lab - Bleeding Edge Update Script
# Usage: nu scripts/update.nu

def main [] {
    let project_root = ($env.FILE_PWD | path dirname)
    cd $project_root

    print "[*] Initiating Bleeding Edge Update Sequence..."

    print "\n[*] 1. Updating Nix Flake inputs (Pulling latest Nightly Rust, Node, and system tools)..."
    nix flake update

    print "\n[*] 2. Bumping Rust crates to absolute latest semver bounds..."
    cargo update

    print "\n[*] 3. Bumping NPM frontend packages..."
    npm update

    print "\n[+] Updates complete! Run `nix develop` to pull in any new compiler changes, then test with `cargo leptos watch`."
}
