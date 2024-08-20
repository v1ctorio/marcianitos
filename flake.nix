{
  description = "A Nix-flake-based bevy development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self , nixpkgs ,... }: let
    system = "x86_64-linux";
  in {
    devShells."${system}".default = let
      pkgs = import nixpkgs {
        inherit system;
      };
    in pkgs.mkShell {
      packages = with pkgs; [
        rustc
        rust-analyzer
        rustfmt
        cargo
        git
        alsa-lib
        pkg-config
        udev
        
        vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers

      ];

      shellHook = ''
        echo "Rust 🦀 `${pkgs.rustc}/bin/rustc --version`"
      '';
    };
  };
}
