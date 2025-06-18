{
  description = "git-remove-merged-branches";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    nixpkgsFor = forAllSystems (system: import nixpkgs {inherit system;});
  in {
    packages = forAllSystems (system: let
      pkgs = nixpkgsFor.${system};
    in {
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "git-remove-merged-branches";
        version = "0.1.0";

        src = self;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
    });
  };
}
