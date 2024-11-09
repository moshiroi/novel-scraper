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
        pythonBuildInputs  = with pkgs; [
            python3
            poetry
            python3Packages.lxml
            python3Packages.selenium
            python3Packages.pyyaml
            python3Packages.loguru
            python3Packages.beautifulsoup4
            python3Packages.undetected-chromedriver
        ];

      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            chromium
            chromedriver
          ] ++ pythonBuildInputs ++ rustBuildInputs;
        };

      }
    );

}

