# hashmash development environment
{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell {
  # Host environment tools
  nativeBuildInputs = with pkgs; [
    # Basic build tools
    rustc
    cargo

    # Interactive development
    rust-analyzer
    rustfmt
    clippy
    nixpkgs-fmt
  ];

  # Build inputs (eg for target system if cross-compiling)
  buildInputs = with pkgs; [
    libiconv
  ];
}
