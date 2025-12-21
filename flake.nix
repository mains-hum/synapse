{
  description = "Synapse";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          pkg-config
          alsa-lib
          alsa-lib.dev
        ];
        
        PKG_CONFIG_PATH = "${pkgs.alsa-lib.dev}/lib/pkgconfig";
      };
    };
}
