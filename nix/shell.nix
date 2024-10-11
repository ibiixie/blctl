{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./package.nix { }) ];

  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
  ];
}
