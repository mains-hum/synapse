{
  description = "Synapce";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };

      runtimeDeps = with pkgs; [
        alsa-lib
        fontconfig
      ];
      buildDeps = with pkgs; [
        pkg-config
        (rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
        })
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = buildDeps ++ runtimeDeps;
      };

      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "synapse";
        version = "0.1.0";
        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = runtimeDeps;

        doCheck = false;
      };

      apps.${system}.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/synapce";
      };
    };
}
