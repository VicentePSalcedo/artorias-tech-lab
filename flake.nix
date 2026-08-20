{
  description = "Artorias Tech Lab - Leptos Development Environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          # Leptos typically requires nightly Rust
          rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "wasm32-unknown-unknown" ];
          };
        in
        {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              # Rust Toolchain
              rustToolchain

              # Leptos specific tools
              cargo-leptos
              cargo-generate

              # Frontend tools
              nodejs_22
              dart-sass
              binaryen

              # System dependencies often required by Rust crates (like reqwest/axum)
              pkg-config
              openssl
            ] ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

            shellHook = ''
              export PATH=~/.cargo/bin:$PATH
              echo "🦀 Welcome to the Artorias Tech Lab development shell! 🦀"
              echo ""
              echo "To get started:"
              echo "  1. Run 'npm install' to fetch tailwind dependencies."
              echo "  2. Run 'cargo leptos watch' to start the development server."
            '';
          };
        }
      );
    };
}
