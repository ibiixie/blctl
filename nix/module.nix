{
  config,
  lib,
  pkgs,
  ...
}: let
  
in {
  options = {
    services.blctl = {
      enable = lib.mkEnableOption "Whether to enable the blctl daemon";

      package = lib.mkOption {
        type = lib.types.package;
        default = pkgs.blctl;
      };

      restore = lib.mkOption {
        type = lib.types.bool;
        default = false;
        description = "Whether to enable the blctl store/restore procedure";
      };
    };
  };

  config = lib.mkIf config.services.blctl.enable {
    # Install the binaries (bin/blctl and sbin/blctld)
    environment.systemPackages = [ pkgs.blctl ];
    
    # Create a systemd service for bin/blctld
    systemd = {
      services.blctl = {
	      serviceConfig = {
          ExecStart = "${config.services.blctl.package}/sbin/blctld";
	      };
        wantedBy = [ "multi-user.target" ];
      };
    };
  };
}
