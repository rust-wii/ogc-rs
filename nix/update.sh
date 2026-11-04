#!/usr/bin/env nix-shell
#!nix-shell -i bash -p nix-prefetch-docker fd
# shellcheck shell=bash

function fetch() {
    echo "fetching $1..."
    nix-prefetch-docker --image-name "devkitpro/$1" --json --quiet >"nix/sources/$1.json"
}

fd flake.nix -x nix flake update --flake "{//}"

mkdir -p nix/sources

fetch devkitarm
fetch devkita64
fetch devkitppc
