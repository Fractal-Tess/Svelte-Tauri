{
  description = "PageLens development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    playwright.url = "github:pietdevries94/playwright-web-flake";
  };

  outputs = { nixpkgs, systems, playwright, ... }@inputs:
    let
      eachSystem = f:
        nixpkgs.lib.genAttrs (import systems) (system:
          let
            overlay = final: prev: {
              inherit (playwright.packages.${system}) playwright-test playwright-driver;
            };
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ overlay ];
            };
          in
          f pkgs
        );

      libraries = pkgs:
        with pkgs; [
          webkitgtk_4_1
          webkitgtk_4_1.dev
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          librsvg
        ];

      packages = pkgs:
        with pkgs; [
          curl
          wget
          pkg-config
          dbus
          openssl
          glib
          gtk3
          libsoup_3
          webkitgtk_4_1
          webkitgtk_4_1.dev
          librsvg
          clang
          cargo
          rustc
          rustfmt

          bun
          prettierd

          playwright-test
        ];
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          buildInputs = packages pkgs;

          shellHook = ''
            export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1
            export PLAYWRIGHT_BROWSERS_PATH="${pkgs.playwright-driver.browsers}"
            export PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH="$(find ${pkgs.playwright-driver.browsers}/chromium-*/chrome-linux*/ -name chrome -type f | head -1)"

            echo "
               ____                  __
              / __ \____ _____ ____ / /   ___  ____  _____
             / /_/ / __ \`/ __ \`/ _ \/ /   / _ \/ __ \/ ___/
            / ____/ /_/ / /_/ /  __/ /___/  __/ / / (__  )
           /_/    \__,_/\__, /\___/_____/\___/_/ /_/____/
                       /____/
            PageLens Development Environment
            Bun - $(${pkgs.bun}/bin/bun --version)
            Rustc - $(${pkgs.rustc}/bin/rustc --version)
            Chromium - $(ls -d ${pkgs.playwright-driver.browsers}/chromium-* 2>/dev/null | head -1 | xargs basename || echo "not found")
            " | lolcat

            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath (libraries pkgs)
            }:$LD_LIBRARY_PATH
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';

          WEBKIT_DISABLE_COMPOSITING_MODE = 1;
          RUST_BACKTRACE = "full";
          GDK_BACKEND = "x11";
        };
      });
    };
}
