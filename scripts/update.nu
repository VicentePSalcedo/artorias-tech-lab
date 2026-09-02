#!/usr/bin/env nu

# Artorias Tech Lab - Dependency Update Script
# Usage: nix develop -c nu scripts/update.nu
#
# Keeps every dependency ecosystem current:
#   1. Nix flake inputs  (nixpkgs, Rust toolchain, system tools)
#   2. Rust crates       (Cargo.lock)
#   3. NPM packages      (tailwindcss / postcss / autoprefixer toolchain)
#   4. NPM security audit
#
# Pattern borrowed from the `just update-deps` recipe in ~/Repos/iio/Justfile.

def main [] {
    let project_root = (git rev-parse --show-toplevel | str trim)
    cd $project_root

    print "[*] Artorias Tech Lab - dependency update"
    print ""

    # Guard: never bump lockfiles on a dirty tree
    let dirty_count = (git status --porcelain | lines | length)
    if $dirty_count > 0 {
        print $"(ansi red_bold)ERROR(ansi reset): working tree is dirty ($dirty_count) uncommitted files."
        print "Commit or stash your changes first so lockfile bumps stay reviewable."
        exit 1
    }

    print "[*] 1/4 Updating Nix flake inputs (nixpkgs, Rust toolchain, system tools)..."
    nix flake update
    print ""

    print "[*] 2/4 Updating Rust crates (Cargo.lock)..."
    cargo update
    print ""

    print "[*] 3/4 Updating NPM packages (tailwindcss toolchain)..."
    npm update --no-fund --no-audit --silent
    print ""

    print "[*] 4/4 Auditing NPM packages for vulnerabilities..."
    let audit = (do { npm audit --audit-level=moderate } | complete)
    if $audit.exit_code == 0 {
        print $"(ansi green)OK(ansi reset): no known vulnerabilities at moderate severity or higher."
    } else {
        print $"(ansi yellow)WARNING(ansi reset): npm audit reported issues - inspect with `npm audit`."
    }
    print ""

    print "[+] Done. Changed files:"
    git status --porcelain | lines | each { |l| print $"  ($l)" }
    print ""
    print "[+] Next steps:"
    print "    1. `nix develop -c cargo leptos watch` - pull the new toolchain and verify the build"
    print "    2. Review and commit the lockfile bumps"
    print "    3. Deploy with `nix develop -c nu scripts/deploy.nu`"
}
