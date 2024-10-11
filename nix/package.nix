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
      rev = "cdc88e6ab1fb33ffd0e1f3c6f4d91c1eba61419f";
      sha256 = "sha256-S0VAuGF2KzrlX/NQ/S1I1+n4Y9CoX1QxXo0VZ1HY380=";
    };

    cargoLock = {
      lockFile = ../Cargo.lock;
    };

    meta = with pkgs.lib; {
      description = "A minimal backlight control daemon for Linux";
      homepage = "https://github.com/imxela/blctl";
      license = with licenses; [ asl20 mit ];
      maintainers = with maintainers; [ imxela ];
      mainProgram = "blctl";
    };
  }
