{
  description = "nix shell from wiki.nixos.org/wiki/Dioxus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";

    # For installing non-standard rustc versions
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";

  };

  outputs =
    {
      self,
      rust-overlay,
      nixpkgs,
      systems,
    }:
    let
      forAllSystems = nixpkgs.lib.genAttrs (import systems);
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            system = system;
            overlays = [
              rust-overlay.overlays.default
            ];
          };
          shell = import ./shell.nix { inherit pkgs; };
          in {
            default = shell.shell;
          }
      );

      nixosModules.default = { config, lib, ... }:
        let
          pkgs = import nixpkgs {
            system = "x86_64-linux";
            overlays = [
              rust-overlay.overlays.default
            ];
          };
        in
        import ./. {
          inherit pkgs lib config;
        };
    };
}
