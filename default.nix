#
# Build hashmash binary.
#
# Pinned nixpkgs managed by niv.
#

{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:

let
  rustFilterSource = import ./nix/rust-filter-source.nix;
in
with pkgs;

rustPlatform.buildRustPackage rec {
  pname = "hashmash";
  version = "0.2.0";
  src = rustFilterSource ./.;
  cargoSha256 = "0x8g0m1l1i29mv5fnk8abn6ycf6az043d47d8f9vwv7bnsdk27b0";
  verifyCargoDeps = true;

  meta = {
    description = "Find and randomize cryptographic hashes in text files";
    homepage = "https://github.com/simonchatts/hashmash";
    license = lib.licenses.mit;
    maintainers = [ maintainers.simonchatts ];
  };
}
