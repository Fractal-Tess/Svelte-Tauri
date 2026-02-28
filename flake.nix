{
  description = "Tauri development shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
  };

  outputs = { nixpkgs, systems, ... }@inputs:
    let
      eachSystem = f:
        nixpkgs.lib.genAttrs (import systems)
          (system: f nixpkgs.legacyPackages.${system});

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
        ];
    in
    {
      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          buildInputs = packages pkgs;

          shellHook = ''
            echo "
              ______                   
             /_  __/___ ___  _______(_)
              / / / __ \`/ / / / ___/ / 
             / / / /_/ / /_/ / /  / /  
            /_/  \__,_/\__,_/_/  /_/   
            Tauri Development Environment
            Bun - $(${pkgs.bun}/bin/bun --version)
            Rustc - $(${pkgs.rustc}/bin/rustc --version)
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
