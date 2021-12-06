# production package for hashmash
{ lib, rustPlatform, installShellFiles, rustc, cargo, clippy, libiconv }:

let
  # Just include: Cargo.toml, Cargo.lock, *.1, src/**
  regex = ".*/Cargo\.(lock|toml)|.*\.1|.*/src($|/.*)";
  rustFilterSource = builtins.filterSource (path: _: builtins.match regex path != null);
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in
rustPlatform.buildRustPackage {
  # Package the binary
  pname = cargoToml.package.name;
  version = cargoToml.package.version;
  src = rustFilterSource ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = [
    rustc
    cargo
    installShellFiles
  ];
  buildInputs = [
    libiconv
  ];

  # Package the man page
  postInstall = '' installManPage hashmash.1 '';

  # Enable "cargo test" checks
  doCheck = true;
  copyLibs = true;
  CARGO_BUILD_INCREMENTAL = "false";
  RUST_BACKTRACE = "full";

  # Metadata
  meta = {
    description = "Find and randomize cryptographic hashes in text files";
    homepage = "https://github.com/simonchatts/hashmash";
    license = lib.licenses.mit;
    maintainers = [ lib.maintainers.simonchatts ];
  };
}
