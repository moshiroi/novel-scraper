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

        novelScraper = pkgs.stdenv.mkDerivation {
          name = "novel-scraper";
          src = ./.;

          unpackPhase = ":";

          # Specify the build inputs (include chromedriver and Python)
          buildInputs = with pkgs; [
            which      # Add which to ensure it's available
            chromedriver
            python3
            python3Packages.lxml
            python3Packages.selenium
            python3Packages.pyyaml
            python3Packages.loguru
            python3Packages.beautifulsoup4
            python3Packages.undetected-chromedriver
          ];

          # Define the script to copy chromedriver and run the Python file
          buildPhase = ''
            # Find the path to chromedriver
            # TODO: Fail if chromedriver not present for some reason
            CHROMEDRIVER_PATH=$(which chromedriver)

            # Create a temporary directory and copy chromedriver
            TMP_CHROMEDRIVER_DIR=$(mktemp -d)
            cp $CHROMEDRIVER_PATH $TMP_CHROMEDRIVER_DIR/

            echo "Chromedriver copied to $TMP_CHROMEDRIVER_DIR"

            # Run the Python script
            python3 ${./main.py} $TMP_CHROMEDRIVER_PATH/chromedriver ${./config.yaml}
          '';

        };

      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            python3
            poetry
            python3Packages.lxml
            python3Packages.selenium
            python3Packages.pyyaml
            python3Packages.loguru
            python3Packages.beautifulsoup4
            python3Packages.undetected-chromedriver
            chromium 
            chromedriver 
          ];
        };

        packages.default = novelScraper;
      }

    );

}

