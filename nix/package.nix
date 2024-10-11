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
      rev = "dae4445e06ae25e99030eebed70b5aee5abfbdec";
      sha256 = "sha256-5J8rDVbruBfHsssbzTaY4uZaZMh8jJBYkgkpDnzcC0w=";
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
