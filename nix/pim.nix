{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "pim";
  version = "0.1.0";

  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
}