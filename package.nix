{
  pkgs,
  ...
}:

let
  shell = import ./shell.nix { inherit pkgs; };
in
  shell.rust-platform.buildRustPackage {
  pname = "ratter";
  version = "0.1.0";

  src = pkgs.lib.cleanSourceWith {
    src = ./.;
    filter = path: type: !(pkgs.lib.hasSuffix ".nix" (builtins.baseNameOf path));
  };
  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "matter-bdx-0.3.1" = "sha256-qR3ok9dMxt074+67mPcha1Ss1KirFeFFMxWdz5KiqqA=";
    };
  };

  nativeBuildInputs =
    with pkgs;
    [
      pkg-config
      dbus
    ]
    ++ shell.tools;
  buildInputs = with pkgs; [
    dbus
    openssl
  ];

  doCheck = false;

  buildPhase = ''
    dx bundle --release
  '';

  installPhase = ''
    mkdir -p $out/bin

    cp -r target/dx/ratter-app/release/web/ $out/bin/web/
  '';
}
