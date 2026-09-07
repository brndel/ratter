{ pkgs }:
let
  rustShellToolchain = (pkgs.rust-bin.stable."1.97.1".default).override {
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
      hash = "sha256-a7lcXJnnZkYReja+iUO7NqqrWyv3toxnUgQb8s4IS5s=";
    };

    cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
      inherit src;
      inherit (src) pname version;
      hash = "sha256-R1Tas33Ursy8kqsxguAkG0ZhNed2n5uFTAhw1l2qlLY=";
    };
  };
  tools = (with pkgs; [ binaryen ]) ++ [
    rustShellToolchain
    dioxus-cli
    wasm-bindgen-cli
  ];
in
{
  inherit tools;
  rust-platform = pkgs.makeRustPlatform {
    cargo = rustShellToolchain;
    rustc = rustShellToolchain;
  };
  shell = pkgs.mkShell {
    name = "dioxus";
    packages = tools;
  };
}
