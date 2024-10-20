{
  description = "Flake for general novel web scraper";

  inputs = {

    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
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
            # python3Packages.undetected-chromedriver
            chromium # Chrome for Selenium (unfree, allow unfree in your config)
            chromedriver # Chrome WebDriver for Selenium

            # Test for chrome driver stuff
            #   pkgs.glib # Provides libglib-2.0.so.0
            #   pkgs.nss # Provides libnss3.so, libnssutil3.so
            #   pkgs.nspr # Provides libnspr4.so, libplds4.so, libplc4.so
            #   xorg.libpthreadstubs
            #   pkgs.gcc # Provides libgcc_s.so.1
            #   pkgs.libc # Provides libc.so.6, ld-linux-x86-64.so.2
            #   pkgs.libdl # Provides libdl.so.2
            #   pkgs.pcre2 # Provides libpcre2-8.so.0
            #   pkgs.xorg.libX11 # Provides libxcb.so.1
            #   pkgs.xorg.libXau # Provides libXau.so.6
            #   pkgs.xorg.libXdmcp # Provides libXdmcp.so.6
            #   pkgs.libm # Provides libm.so.6
            #   pkgs.librt # Provides librt.so.1
          ];
        };
      }

    );

}

