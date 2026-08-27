{
  description = "Reproducible development environment for ogc-rs (libogc bindings)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    # Official devkitPro toolchains (incl. devkitPPC + libogc) as a Nix overlay.
    # https://github.com/bandithedoge/devkitNix
    devkitNix.url = "github:bandithedoge/devkitNix";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, devkitNix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          (import rust-overlay)
          devkitNix.overlays.default
        ];
        pkgs = import nixpkgs { inherit system overlays; };
        # Nightly + rust-src match the project README (custom powerpc target).
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
        # Sets DEVKITPRO / DEVKITPPC and includes the patched toolchain.
        dkpStdenv = pkgs.devkitNix.stdenvPPC;
      in {
        devShells.default = pkgs.mkShell.override { stdenv = dkpStdenv; } {
          name = "ogc-rs-dev";
          packages = with pkgs; [
            rustToolchain
            rust-analyzer
            clang
            llvmPackages.libclang
            pkg-config
            git
          ];

          # libclang is required by bindgen-based crates such as ogc-sys.
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";

          shellHook = ''
            # powerpc-eabi-* live under $DEVKITPPC/bin; stdenvPPC only exports the vars.
            export PATH="$DEVKITPPC/bin:$PATH"
            echo "ogc-rs dev shell"
            echo "  rustc: $(rustc --version)"
            echo "  clang: $(clang --version | head -n1)"
            echo "  DEVKITPRO=$DEVKITPRO"
            echo "  DEVKITPPC=$DEVKITPPC"
            echo ""
            echo "Wii target build: cargo check  (see .cargo/config.toml)"
            echo "Note: devkitNix extracts the official devkitPPC Docker image (Linux hosts)."
          '';
        };
      });
}
