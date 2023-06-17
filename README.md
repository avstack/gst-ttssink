# gst-ttssink: A GStreamer sink implementing text-to-speech via platform APIs

Accepts text buffers on its sink pad and plays them back as speech via platform APIs.

Supported platforms are those of the [tts crate](https://crates.io/crates/tts):

 * Windows
    * Screen readers / SAPI via tolk (requires enabling the `tolk` feature)
    * WinRT
 * Linux (via Speech Dispatcher)
 * macOS
 * iOS
 * Android
 * WebAssembly (if you can manage to compile GStreamer for WebAssembly...)

## Installation

gst-ttssink is written in Rust and uses the cargo-c helper. Set up a Rust development environment (e.g. using [rustup](https://rustup.rs)) and then:

```
cargo install cargo-c

git clone https://github.com/avstack/gst-ttssink
cd gst-ttssink
cargo cbuild --release
export GST_PLUGIN_PATH=$(pwd)/target/release
```

## Example usage (type text and it will be played when you press enter)

```
gst-launch-1.0 --quiet fdsrc ! 'text/x-raw,format=utf8' ! ttssink
```

Combine it with our [gst-whisper](https://github.com/avstack/gst-whisper) and [gst-openaichat](https://github.com/avstack/gst-openaichat) elements and use GPT as a voice assistant! (Beware: if you don't use headphones, GPT may start responding to itself.)

```
OPENAI_API_KEY=... gst-launch-1.0 --quiet autoaudiosrc ! audioconvert ! audioresample ! queue ! whisper ! openaichat model=gpt-3.5-turbo ! ttssink
```

## License

gst-ttssink is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Any kinds of contributions are welcome as a pull request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in these crates by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements

gst-ttssink development is sponsored by [AVStack](https://avstack.io/). We provide globally-distributed, scalable, managed Jitsi Meet backends.
