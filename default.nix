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
  # Handle the binary
  pname = "hashmash";
  version = "1.0.0";
  src = rustFilterSource ./.;
  cargoSha256 = "0x7lrqiwxwfdkc6ysa3fnp10591q7d46m36bny9csa58k2bjsdx8";
  verifyCargoDeps = true;

  # Handle the man page
  nativeBuildInputs = [ installShellFiles ];
  postInstall = '' installManPage hashmash.1 '';

  # Metadata
  meta = {
    description = "Find and randomize cryptographic hashes in text files";
    homepage = "https://github.com/simonchatts/hashmash";
    license = lib.licenses.mit;
    maintainers = [ maintainers.simonchatts ];
  };
}
