# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Development

At the command line cd to src-tauri/ directory and run. In this
development mode the tauri code will watch for changes in the
code and automatically recompile the program shortening the
code debug cycle:
```bash
wink@3900x 24-05-24T22:03:05.645Z:~/prgs/rust/tauri/exper-tauri-app/src-tauri (main)
$ cargo tauri dev
        Info Watching /home/wink/prgs/rust/tauri/exper-tauri-app/src-tauri for changes...
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.15s

** (tauri-app:157481): WARNING **: 15:03:15.890: webkit_settings_set_enable_offline_web_application_cache is deprecated and does nothing.
```

You should see the Tauri app window open with the following content:
![Image tauri-app](/assets/tauri-app-1.png)

You can enter your name into the input field:
![Image tauri-app](/assets/tauri-app-2.png)

And click the "Greet" button to see the greeting:
![Image tauri-app](/assets/tauri-app-3.png)

## Building and Running

Sometimes you may just want to build the app and then run it from
the command line. To do this cd to the src-tauri/ directory and
run:

```bash
wink@3900x 24-05-24T22:12:44.424Z:~/prgs/rust/tauri/exper-tauri-app/src-tauri (main)
$ cargo tauri build -b none
   Compiling tauri-app v0.0.0 (/home/wink/prgs/rust/tauri/exper-tauri-app/src-tauri)
    Finished `release` profile [optimized] target(s) in 3.30s
wink@3900x 24-05-24T22:13:03.744Z:~/prgs/rust/tauri/exper-tauri-app/src-tauri (main)
$
```

And then run the app:
```bash
wink@3900x 24-05-24T22:14:16.280Z:~/prgs/rust/tauri/exper-tauri-app/src-tauri (main)
$ ./target/release/tauri-app

** (tauri-app:161119): WARNING **: 15:14:34.163: webkit_settings_set_enable_offline_web_application_cache is deprecated and does nothing.
```

And you'll see the app window open as before:
![Image tauri-app](/assets/tauri-app-1.png)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.


