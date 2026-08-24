{
  description = "Reproducible development environment for ogc-rs (libogc bindings)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        # Nightly + rust-src match the project README (custom powerpc target).
        rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
          extensions = [ "rust-src" "rustfmt" "clippy" ];
        };
      in {
        devShells.default = pkgs.mkShell {
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
            echo "ogc-rs dev shell"
            echo "  rustc: $(rustc --version)"
            echo "  clang: $(clang --version | head -n1)"
            echo ""
            echo "Host checks: cargo check -p ogc-rs (or cargo check --workspace)"
            echo "Full Wii cross builds still need the devkitPro toolchain outside Nix;"
            echo "see README.md for powerpc-unknown-eabi setup."
          '';
        };
      });
}
