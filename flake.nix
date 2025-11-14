{
  description = "Rust development environment for crate-template";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };

      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-src" "rust-analyzer"];
      };
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = (pkgs.lib.importTOML ./Cargo.toml).package.name;
        version = (pkgs.lib.importTOML ./Cargo.toml).package.version;
        src = self;
        cargoLock.lockFile = ./Cargo.lock;
      };

      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          cargo-edit
          clippy
          rustfmt
        ];

        shellHook = ''
          echo "Rust development environment loaded"
          echo "Rust version: $(rustc --version)"
        '';
      };
    });
}
