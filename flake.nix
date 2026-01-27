{
  description = "tano";

  nixConfig = {
    extra-substituters = [
      "https://tuisic.cachix.org"
      "https://nix-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "tuisic.cachix.org-1:XEafqLtuNhcRjCZERfPbKp+xSAqDC9Gfd0O7gzFpaSY="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
    ];
  };

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    ...
  } @ inputs: let
    systems = ["x86_64-linux" "aarch64-linux"];

    forAllSystems = nixpkgs.lib.genAttrs systems;
  in {
    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);

    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      tano = pkgs.callPackage ./nix/tano.nix {inherit inputs;};
      default = self.packages.${system}.tano;
    });

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      default = pkgs.callPackage ./nix/shell.nix {inherit inputs;};
    });

    homeModules = {
      default = import ./nix/home-manager.nix {inherit inputs self;};
      tano = self.homeModules.default;
    };
  };
}
