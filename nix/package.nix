pkgs:
{
  name,
  src,
  includePaths ? [ ],
}:
pkgs.stdenv.mkDerivation (finalAttrs: {
  inherit name;
  # Import the source files found in the official devkitPro Docker images.
  src = pkgs.dockerTools.pullImage (pkgs.lib.importJSON src);

  nativeBuildInputs = with pkgs; [
    autoPatchelfHook
  ];

  buildInputs = with pkgs; [
    ncurses
    stdenv.cc.cc.lib
  ];

  dontPatchShebangs = true;

  phases = [
    "buildPhase"
    "fixupPhase"
  ];

  buildPhase = ''
    tar -xf $src

    for archive in $(find *.tar)
    do
      tar -xf $archive
    done

    # Remove this directory as some of its files are read-protected and can't be copied.
    rm -rf opt/devkitpro/pacman/var/lib/pacman/local

    mkdir -p $out
    cp -r opt $out/opt
    ln -sf $out/opt/devkitpro/tools/bin $out/bin
    rm $out/opt/devkitpro/pacman/share/pacman/keyrings
  '';

  passthru = rec {
    CPATH = pkgs.lib.makeSearchPath "include" (
      builtins.map (x: "${finalAttrs.finalPackage}/opt/devkitpro/${x}") includePaths
    );

    shellHook = pkgs.lib.warn "shellHook is deprecated, please consider using the stdenv packages instead" ''
      export DEVKITPRO="${finalAttrs.finalPackage}/opt/devkitpro"
      export DEVKITARM="$DEVKITPRO/devkitARM"
      export DEVKITPPC="$DEVKITPRO/devkitPPC"
      export CPATH=${CPATH}
      export PATH="${
        pkgs.lib.makeBinPath [
          "$DEVKITPRO"
          "$DEVKITARM"
          "$DEVKITPPC"
        ]
      }:$PATH"
    '';
  };
})
