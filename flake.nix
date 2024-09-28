{
  description = "blctl";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    supportedSystems = [ "x86_64-linux" ];
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    pkgsFor = nixpkgs.legacyPackages;

    blctl = pkgsFor.${system}.callPackage ./nix/default.nix { };
  in {
    packages = forAllSystems (system: {
      inherit blctl;
      default = blctl;
    });
    devShells = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./nix/shell.nix { };
    });
  };
}
