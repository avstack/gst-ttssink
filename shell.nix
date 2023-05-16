with import <nixpkgs> {};
mkShell {
  name = "gst-ttssink";
  buildInputs = [
    cargo
    cargo-c
    cmake
    pkg-config
    git
    glib
    gst_all_1.gstreamer
    gst_all_1.gst-plugins-base
  ] ++ (if stdenv.isDarwin then [
    darwin.apple_sdk.frameworks.AppKit
    darwin.apple_sdk.frameworks.AVFoundation
  ] else []);
}
