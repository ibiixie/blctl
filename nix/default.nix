{
  pkgs ? import <nixpkgs> { } 
}:
let
  manifest = (pkgs.lib.importTOML ../Cargo.toml).package;
in 
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;

    src = pkgs.fetchFromGitHub {
      owner = "imxela";
      repo = "blctl";
      rev = "d6be5431e6631a4e8c3ca4c6ae151368f2cddb40";
      sha256 = pkgs.lib.fakeHash;
    };

    cargoLock = {
      lockFile = ../Cargo.lock;
    };

    cargoDeps = rustPlatform.importCargoLock {
      lockFile = ../Cargo.lock;
    };
  }
