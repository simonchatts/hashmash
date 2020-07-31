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
  version = "0.1.0";
  src = rustFilterSource ./.;
  cargoSha256 = "1q384jwgdbrnnzm0jr9wmj44vjmmrg5xxrk53h39hfgg8q9xmg26";
  verifyCargoDeps = true;

  meta = {
    description = "Find and randomize cryptographic hashes in text files";
    homepage = "https://github.com/simonchatts/hashmash";
    license = lib.licenses.mit;
    maintainers = [ maintainers.simonchatts ];
  };
}
