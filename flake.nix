{
  description = "Godot development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    devshell = {
      url = "github:numtide/devshell";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        fenix.follows = "fenix";
      };
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devshell.flakeModule ];
      systems = [ "x86_64-linux" ];
      perSystem =
        { pkgs, system, ... }:
        let
          inherit (inputs.fenix.packages.${system}.stable) toolchain rust-src;
          naersk' = pkgs.callPackage inputs.naersk {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        {
          packages.default = naersk'.buildPackage {
            pname = "vector-implementation";
            src = ./.;
          };

          devshells.default = {
            packages = [ toolchain ];
            devshell.motd = "\\";
            env = [
              {
                name = "RUST_SRC_PATH";
                value = "${rust-src}/lib/rustlib/src/rust/library";
              }
            ];
          };
        };
    };
}
