{ pkgs }:
let
  rustShellToolchain = (pkgs.rust-bin.stable."1.97.1".minimal).override {
    extensions = [
      "rust-src"
      "rust-analyzer"
    ];
    targets = [ "wasm32-unknown-unknown" ];
  };

  dioxus-cli = pkgs.dioxus-cli;

  cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);

  wasmBindgen = pkgs.lib.findFirst (
    pkg: pkg.name == "wasm-bindgen"
  ) (throw "Could not find wasm-bindgen package") cargoLock.package;

  wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
    src = pkgs.fetchCrate {
      pname = "wasm-bindgen-cli";
      version = wasmBindgen.version;
      hash = "sha256-di+qBAdd7pENLiIB9CoZoab+W5xeDoByMREcCGTSzWo=";
    };

    cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
      inherit src;
      inherit (src) pname version;
      hash = "sha256-FTv2GZIAQs0ePdIZXIXil7JbZ6kIT05VG6vqC1qNFxQ=";
    };
  };
  tools = [
    rustShellToolchain
    dioxus-cli
    wasm-bindgen-cli
  ];
in
{
  inherit tools;
  shell = pkgs.mkShell {
    name = "dioxus";
    packages = tools;
  };
}
