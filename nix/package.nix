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
      rev = "af4124f9ac0cb247c8b90791b0acfdb59c55d7ca";
      sha256 = "sha256-9yz+O323ZMTdv3E0kzPfjHUZM4nokdK3PqoBsjwtwJQ=";
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
