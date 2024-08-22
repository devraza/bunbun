{
  description = "Rust development environment for bunbun";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs-unstable, utils, ... }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs-unstable { inherit system; };
      in rec
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
            pname = "bunbun";
            version = "1.4.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        apps.default = utils.lib.mkApp {drv = packages.default;};
      }
    );
}
