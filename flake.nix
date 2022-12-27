{
  description = "Zen microkernel";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        fenix-channel = fenix.packages.${system}.latest;

        fenix-toolchain = (fenix-channel.withComponents [
          "rustc"
          "cargo"
          "clippy"
          "rust-analysis"
          "rust-src"
          "rustfmt"
        ]);

        craneLib = (crane.mkLib pkgs).overrideToolchain fenix-toolchain;

        zen = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;

          # Tests currently need to be run via `cargo wasi` which
          # isn't packaged in nixpkgs yet...
          doCheck = false;

          buildInputs = [];
        };
      in
      {
        checks = {
          inherit zen;
        };

        packages.default = zen;

        apps.default = flake-utils.lib.mkApp {
          drv = pkgs.writeShellScriptBin "my-app" ''
            ${pkgs.wasmtime}/bin/wasmtime run ${zen}/bin/custom-toolchain.wasm
          '';
        };

        devShells.default = pkgs.mkShell {
          inputsFrom = builtins.attrValues self.checks;

          # Extra inputs can be added here
          nativeBuildInputs = with pkgs; [
            fenix-toolchain
            nasm
            rust-analyzer
          ];
        };
      });
}
