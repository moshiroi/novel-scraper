{
  description = "Flake for general novel web scraper";

  inputs = {

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        rustBuildInputs  = with pkgs; [
            rustc
            rustfmt
            cargo
            rust-analyzer
            pkg-config
            openssl
        ];

      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            chromium
            chromedriver
          ] ++ rustBuildInputs;
        };

      }
    );

}

