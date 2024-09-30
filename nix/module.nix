{
  config,
  lib,
  pkgs,
  ...
}: let
  
in {
  options.services.blctl = {
    enable = lib.mkEnableOption "Whether to enable the blctl daemon";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./package.nix { };
    };

    restore = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Whether to enable the blctl store/restore procedure";
    };
  };

  config = lib.mkIf config.services.blctl.enable {
    environment.systemPackages = [ config.services.blctl.package ];
    
    systemd = {
      services.blctl = {
	      serviceConfig = {
          ExecStart = "${config.services.blctl.package}/bin/blctld";
	      };
        wantedBy = [ "multi-user.target" ];
      };
    };
  };
}
