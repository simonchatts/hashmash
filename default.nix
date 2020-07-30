#
# Build hashmash binary.
#
# Pinned nixpkgs and gitignoreSource function managed by niv.
#

{ pkgs ? import (import ./nix/sources.nix).nixpkgs {} }:

# pull in gitignoreSource function
let
  gitignore = (import ./nix/sources.nix).gitignore;
  gitignoreSource = (import gitignore {}).gitignoreSource;
in
with pkgs;

rustPlatform.buildRustPackage rec {
  pname = "hashmash";
  version = "0.1.0";
  src = gitignoreSource ./.;
  cargoSha256 = "1q384jwgdbrnnzm0jr9wmj44vjmmrg5xxrk53h39hfgg8q9xmg26";
  verifyCargoDeps = true;
}
