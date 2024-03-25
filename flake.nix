{
  description = "Rust development environment for bunbun";
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs-unstable.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs-unstable, utils, fenix, ... }:
    utils.lib.eachDefaultSystem
    (
      system: let
        pkgs = import nixpkgs-unstable { inherit system; };
      in rec
      {
        packages.default =
          (pkgs.makeRustPlatform.buildRustPackage {
            pname = "bunbun";
            version = "1.3.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };
        apps.default = utils.lib.mkApp {drv = packages.default;};
      }
    );
}
