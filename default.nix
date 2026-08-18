{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.ratter;

  binary = import ./package.nix { inherit pkgs; };
in
{
  options.services.ratter = {
    enable = lib.mkEnableOption "ratter";
    host = lib.mkOption {
      type = lib.types.str;
      default = "0.0.0.0";
    };
    port = lib.mkOption {
      type = lib.types.port;
      default = 34817;
    };
  };

  config = lib.mkIf cfg.enable {
    users.users.ratter = {
      isSystemUser = true;
      group = "ratter";
      home = "/var/lib/ratter";
    };
    users.groups.ratter = { };

    systemd.tmpfiles.rules = [
      "d /var/lib/ratter 0750 ratter ratter -"
    ];

    systemd.services.ratter = {
      description = "Ratter smart home server";
      after = [ "network.target" ];
      wantedBy = [ "multi-user.target" ];

      path = with pkgs; [ openssl cacert ];

      environment = {
        HOST = cfg.host;
        PORT = toString cfg.port;
        # SSL_CERT_FILE = "${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt";
      };

      serviceConfig = {
        User = "ratter";
        Group = "ratter";
        WorkingDirectory = "/var/lib/ratter";
        ExecStart = "${binary}/bin/web/server";

        RestartSec = 5;
        Restart = "always";
      };

      restartIfChanged = true;
    };
  };
}
