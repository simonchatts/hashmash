# production package for hashmash
{ flib, stdenv, lib, rustPlatform, installShellFiles, rustc, cargo, clippy, libiconv }:

let
  cargoToml = flib.readCargoToml ./.;
in
rustPlatform.buildRustPackage {
  # Package the binary
  pname = cargoToml.name;
  version = cargoToml.version;
  src = flib.rustFilterSource ./.;
  cargoLock = {
    lockFile = ./Cargo.lock;
  };
  nativeBuildInputs = [
    rustc
    cargo
    installShellFiles
  ];
  buildInputs = lib.optionals stdenv.isDarwin [
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
