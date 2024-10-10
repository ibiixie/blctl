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
      rev = "6bcb56bc183d5e7b8d20fb85b2eefd2eaa0b9222";
      sha256 = "sha256-yhBaC3MUx6G68Gx2ozKqxGO+Uq9Fh8PFdhJprmZ2C0E=";
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
