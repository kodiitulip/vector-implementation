{
  description = "Godot development environment";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
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
      systems = [ "x86_64-linux" ];
      perSystem =
        { pkgs, system, ... }:
        let
          inherit (inputs.fenix.packages.${system}.stable) toolchain rust-src;
          naersk' = pkgs.callPackage inputs.naersk {
            cargo = toolchain;
            rustc = toolchain;
          };
          libs = with pkgs; [
            llvmPackages.libclang

            wayland
            # X11 dependencies
            libGL
            libx11
            libx11.dev
            libxcursor
            libxi
            libxinerama
            libxrandr

            alsa-lib
            libpulseaudio
          ];
        in
        {
          packages.default = naersk'.buildPackage {
            pname = "vector-implementation";
            src = ./.;
            nativeBuildInputs =
              with pkgs;
              [
                cmake
                pkg-config
                clang
              ]
              ++ libs;
            env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs =
              with pkgs;
              [
                toolchain
                cmake
                pkg-config
                clang
              ]
              ++ libs;
            env.LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            env.RUST_SRC_PATH = "${rust-src}/lib/rustlib/src/rust/library";
          };
        };
    };
}
