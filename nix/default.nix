{
  pkgs ? import <nixpkgs> { } 
}:
let
  manifest = (pkgs.lib.importTOML ../Cargo.toml).package;
in 
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;

    src = pkgs.lib.fetchFromGitHub {
      owner = "imxela";
      repo = "blctl";
      rev = "";
      sha256 = pkgs.lib.fakeHash;
    };

    cargoLock = {
      lockFile = ../Cargo.lock;
    };
  }
