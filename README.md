# exper-tauri-app

The crate was created [following these instructions](https://tauri.app/v1/guides/getting-started/setup/).
To setup the prerequisites [follow these instructions](https://tauri.app/v1/guides/getting-started/prerequisites)
in particular the [linux prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux).

In my case I'm currently using `cargo` so you should install
create-tauri-app with `cargo install create-tauri-app --locked`
as suggested [by Quick Start and clicking the Cargo](https://tauri.app/v1/guides/getting-started/setup/).

To a project you can use the command line use `-h` to get help:
```bash
wink@3900x 24-05-24T22:40:39.478Z:~/prgs/rust/tauri
$ cargo create-tauri-app -h

cargo create-tauri-app 3.14.0
Tauri Programme within The Commons Conservancy
Rapidly scaffold out a new tauri app project.

USAGE:
  cargo create-tauri-app [OPTIONS] [PROJECTNAME]

ARGS:
  <PROJECTNAME>                 Specify project name which is used for the directory, package.json and Cargo.toml

OPTIONS:
  -m, --manager <MANAGER>       Specify preferred package manager [cargo, pnpm, yarn, npm, bun, dotnet]
  -t, --template <TEMPLATE>     Specify the UI template to use [vanilla, vanilla-ts, vue, vue-ts, svelte, svelte-ts, react, react-ts, solid, solid-ts, yew, leptos, sycamore, angular, preact, preact-ts, blazor]
  -y, --yes                     Skip prompts and use defaults where applicable
  -f, --force                   Force create the directory even if it is not empty.
                    --beta                    Bootstraps a project using tauri@2.0-beta
                    --mobile                  Bootstraps a mobile project too. Only availabe with `--beta` option.
                    --no-mobile               Skip bootstraping a mobile project. Only availabe with `--beta` option.
  -h, --help                    Prints help information
  -v, --version                 Prints version information

wink@3900x 24-05-24T22:43:09.759Z:~/prgs/rust/tauri
```

Then something like:
```bash
wink@3900x 24-05-24T22:44:20.295Z:~/prgs/rust/tauri
$ cargo create-tauri-app -m cargo -t vanilla exper-tauri-app

Template created! To get started run:
  cd exper-tauri-app
  cargo tauri dev

wink@3900x 24-05-24T22:44:25.223Z:~/prgs/rust/tauri
```

If you run `cargo create-tauri-app` without any arguments you'll
be prompted for parameters. The equivalent of the above command
would be:
```bash
wink@3900x 24-05-24T22:49:19.400Z:~/prgs/rust/tauri
$ cargo create-tauri-app
✔ Project name · exper-tauri-app
✔ Choose which language to use for your frontend · Rust - (cargo)
✔ Choose your UI template · Vanilla

Template created! To get started run:
  cd exper-tauri-app
  cargo tauri dev

wink@3900x 24-05-24T22:49:39.478Z:~/prgs/rust/tauri
```

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


