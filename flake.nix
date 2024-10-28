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

        # novelScraper = pkgs.stdenv.mkDerivation {
        #   name = "novel-scraper";
        #   src = ./.;

        #   unpackPhase = ":";

        #   # Specify the build inputs (include chromedriver and Python)
        #   buildInputs = with pkgs; [
        #     which # Add which to ensure it's available
        #     chromedriver
        #     python3
        #     python3Packages.lxml
        #     python3Packages.selenium
        #     python3Packages.pyyaml
        #     python3Packages.loguru
        #     python3Packages.beautifulsoup4
        #     python3Packages.undetected-chromedriver
        #   ];

        #   buildPhase = ''
        #     # Do the necessary setup here
        #     echo "Build phase complete, nothing to run here"
        #     touch $out
        #   '';

        #   # Define the script to copy chromedriver and run the Python file
        #   installPhase = ''
        #     echo "Install phase complete, nothing to run here"
        #       TMP_OUTPUT_DIR=$(mktemp -d)
        #     # mkdir $out
        #   '';

        #   shellHook = ''
        #     echo "Scraper is ready to run. Set OUTPUT_DIR and run with nix develop --impure"

        #     if [ -z "$OUTPUT_DIR" ]; then
        #       echo "Please set the OUTPUT_DIR environment variable to your desired output path."
        #     else
        #       mkdir -p "$OUTPUT_DIR"  # Create the output directory if it doesn't exist
        #       echo "Running the script and writing output to $OUTPUT_DIR"
        #       CHROMEDRIVER_PATH=$(which chromedriver)

        #       # Create a temporary directory and copy chromedriver
        #       TMP_CHROMEDRIVER_DIR=$(mktemp -d)
        #       TMP_OUTPUT_DIR=$(mktemp -d)
        #       cp $CHROMEDRIVER_PATH $TMP_CHROMEDRIVER_DIR/

        #       echo "Chromedriver copied to $TMP_CHROMEDRIVER_DIR"

        #       # Run the Python script
        #       python3 ./main.py 
        #         ./config.yaml
        #       } $TMP_CHROMEDRIVER_PATH/chromedriver $OUTPUT_DIR
        #     fi
        #   '';

        # };

        novelScraper = pkgs.python3Packages.buildPythonPackage {

          pname = "novel-scraper";
          version = "1.0.0";

          src = ./.;

          # Add your Python dependencies here
          propagatedBuildInputs = with pkgs.python3Packages; [
            requests
            beautifulsoup4
            lxml
            selenium
            pyyaml
            loguru
            beautifulsoup4
            undetected-chromedriver
            setuptools
          ];

          # Define your script or entry point
          installPhase = ''
            mkdir -p $out/bin
            cp -r . $out/bin/
            chmod +x $out/bin/main.py  # Make your script executable
          '';

          meta = { description = "A Python scraper tool"; };
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
            rustc
            cargo
            rust-analyzer
          ];
        };

        packages.default = novelScraper;
      }

    );

}

