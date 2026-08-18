{
  pkgs,
  ...
}:

let
  shell = import ./shell.nix { inherit pkgs; };
in
pkgs.stdenv.mkDerivation {
  pname = "ratter";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;

  buildInputs = shell.tools;

  buildPhase = ''
    dx bundle --release
  '';
  installPhase = ''
    mkdir $out/bin

    cp target/dx/ratter-app/release/web/ $out/bin/web/
  '';
}
