{
  description = "Nix development environment for the ogc-rs crate";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-parts
    }@inputs:
      inputs.flake-parts.lib.mkFlake {
        inherit inputs;
      }
      {
        imports = [
          inputs.flake-parts.flakeModules.easyOverlay
        ];
        perSystem = {config, system, final, ...}: {
          # Pull the official devkitPro Docker images and patch the binary files to work with Nix.
          # These are made available as packages to be added to a Nix environment.
          packages =
          let
            pkgs = inputs.nixpkgs.legacyPackages.${system};
          in
          {
            # Create each package from their respective Docker image.
            devkitA64 = pkgs.callPackage ./nix/package.nix pkgs {
              name = "devkitA64";
              src = ./nix/sources/devkita64.json;
              includePaths = [
                "devkitA64"
                "devkitA64/aarch64-none-elf"
                "libnx"
                "portlibs/switch"
              ];
            };
            devkitARM = pkgs.callPackage ./nix/package.nix pkgs {
              name = "devkitARM";
              src = ./nix/sources/devkitarm.json;
              includePaths = [
                "devkitARM"
                "devkitARM/arm-none-eabi"
                "libctru"
                "libgba"
                "libmirko"
                "libnds"
                "liborcus"
                "libtonc"
                "portlibs/3ds"
                "portlibs/armv4t"
                "portlibs/gba"
                "portlibs/gp2x"
                "portlibs/nds"
              ];
            };
            devkitPPC = pkgs.callPackage ./nix/package.nix pkgs {
              name = "devkitPPC";
              src = ./nix/sources/devkitppc.json;
              includePaths = [
                "devkitPPC"
                "devkitPPC/powerpc-eabi"
                "libogc"
                "portlibs/gamecube"
                "portlibs/ppc"
                "portlibs/wii"
                "portlibs/wiiu"
                "wut"
              ];
            };
            # Create an altered stdenv that automatically sets up the devkitPro toolchain.
            stdenvA64 = pkgs.stdenvAdapters.addAttrsToDerivation {
              nativeBuildInputs = [ config.packages.devkitA64 ];
              env.DEVKITPRO = config.packages.devkitA64 + "/opt/devkitpro";
            } pkgs.stdenvNoCC;
            stdenvARM = pkgs.stdenvAdapters.addAttrsToDerivation {
              nativeBuildInputs = [ config.packages.devkitARM ];
              env = rec {
                DEVKITPRO = config.packages.devkitARM + "/opt/devkitpro";
                DEVKITARM = DEVKITPRO + "/devkitARM";
              };
            } pkgs.stdenvNoCC;
            stdenvPPC = pkgs.stdenvAdapters.addAttrsToDerivation {
              nativeBuildInputs = [ config.packages.devkitPPC ];
              env = rec {
                DEVKITPRO = config.packages.devkitPPC + "/opt/devkitpro";
                DEVKITPPC = DEVKITPRO + "/devkitPPC";
              };
            } pkgs.stdenvNoCC;
          };
          # Create a developer environment where both the Rust Nightly toolchain (imported from rust-toolchain.toml) and the devkitPro toolchain are automatically set up.
          devShells =
          let
            pkgs = import inputs.nixpkgs {
              inherit system;
              config = {
                allowUnfree = true;
                allowUnfreePredicate = _: true;
              };
              overlays = [
                inputs.rust-overlay.overlays.default
              ];
            };
            rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          in
          {
            # Default devShell is x86_64.
            default = config.devShells.x86_64;
            x86_64 = pkgs.mkShell.override { stdenv = config.packages.stdenvPPC; } {
              buildInputs =
                with pkgs;
                [
                  rust-nightly
                  llvmPackages.libclang
                ];

              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            };
            aarch64 = pkgs.mkShell.override { stdenv = config.packages.stdenvARM; } {
              buildInputs =
                with pkgs;
                [
                  rust-nightly
                  llvmPackages.libclang
                ];

              LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            };
          };
          # Expose a Nixpkgs Overlay for third party projects to import the devkitPro Toolchain.
          overlayAttrs = {
            inherit (config.packages)
              devkitA64
              devkitARM
              devkitPPC
              stdenvA64
              stdenvARM
              stdenvPPC
              ;
          };
      };
      # This flake will build for these system architectures.
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    };
}
