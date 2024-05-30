{
  description = "Build a cargo project without extra checks";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, crane, fenix, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        toolchain = fenix.packages.${system}.combine (with fenix.packages.${system}; [
          latest.toolchain
          # targets."x86_64-unknown-linux-musl".latest.rust-std
        ]);
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
      in
      rec {
        packages.default = craneLib.buildPackage {
          src = pkgs.lib.cleanSourceWith {
            src = craneLib.path ./.;
            filter = path: type: 
              (craneLib.filterCargoSources path type) ||
              (builtins.match ".*(ttf)$" path != null);
          };
          strictDeps = true;
          pname = "breakwater";
          cargoExtraArgs = "--package breakwater";

          nativeBuildInputs = with pkgs; [
            clang
            makeWrapper
            pkg-config
          ];
          buildInputs = with pkgs; [
            libvncserver
            stdenv
            libclang
            libvncserver.dev
          ];
          
          postInstall = ''
            wrapProgram $out/bin/breakwater \
              --set LD_LIBRARY_PATH ${pkgs.lib.makeLibraryPath [
                pkgs.libvncserver
              ]}
          '';
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          LIBVNCSERVER_HEADER_FILE = "${pkgs.libvncserver.dev}/include/rfb/rfb.h";
        };

        apps.default = flake-utils.lib.mkApp {
          drv = packages.default;
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            pkg-config
            clang
            libclang
            libvncserver
            libvncserver.dev
          ];

          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          LIBVNCSERVER_HEADER_FILE = "${pkgs.libvncserver.dev}/include/rfb/rfb.h";
        };
      });
}
