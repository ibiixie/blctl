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
      rev = "028638a99b68cb068ae7a446e662712eb1cec713";
      sha256 = "sha256-LMAICQrChwsWWmM633w21VFY2lf5gFFJ07C3fwp638I=";
    };

    cargoLock = {
      lockFile = ../Cargo.lock;
    };

    /*
    installPhase = ''
      mkdir -p $out/bin
      cp blctl $out/bin

      mkdir -p $out/sbin
      cp blctld $out/sbin
    '';
    */

    meta = with pkgs.lib; {
      description = "A minimal backlight control daemon for Linux";
      homepage = "https://github.com/imxela/blctl";
      license = with licenses; [ asl20 mit ];
      maintainers = with maintainers; [ imxela ];
      mainProgram = "blctl";
    };
  }
