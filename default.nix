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
  cargoSha256 = "0ja8snfv0x0y913zgqiy68m9vzbgslrc68c824fn88njgv0135qc";
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
