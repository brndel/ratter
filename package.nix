{
  pkgs,
  ...
}:

let
  shell = import ./shell.nix { inherit pkgs; };
in
pkgs.rustPlatform.buildRustPackage {
  pname = "ratter";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = shell.tools;

  buildPhase = ''
    dx bundle --release
  '';

  installPhase = ''
    mkdir $out/bin

    cp target/dx/ratter-app/release/web/ $out/bin/web/
  '';
}
