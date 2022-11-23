# Roo - A smart bookmarking search engine for your browser

![Dall-E 2 generated logo](static/roo-logo.png)

## Introduction

`Roo` (short for "Kangaroo") is a smart bookmarking and productivity CLI tool that works as a custom search engine for your browser, allowing you to quickly issue commands in your browser's search bar. It's a simple server that accepts a search query and redirects to the appropriate URI based on a set of predefined (and customizable) rules.

`Roo` is useful for personal productivity, but many users run it on company servers to allow all employees to share a common way to quickly access internal and external tools, wikis, documentation and resources.

Roo is heavily inspired by [bunny1](https://github.com/ccheever/bunny1), a bookmarking tool created at Facebook and open-sourced in 2012. It's written in Rust and released under the [MIT license](LICENSE.md).

## Built-in commands

Roo comes with a set of default commands that can be fully customized.

| Tool                | Description                 | Command              | Example                       |
|---------------------|-----------------------------|----------------------|-------------------------------|
| Google              | Search Google.com           | `g <search query>`   | `g Rust language`             |
| Google Docs         | Create a new GDoc           | `newdoc`             | `newdoc`                      |
| Gmail               | Create a new mail in Gmail  | `newmail`            | `newmail`                     |
| GitHub              | Search Github               | `gh <search query>`  | `gh marcocampana/roo`         |
| GitHub              | Search Github Code          | `ghc <search query>` | `ghc hello world!`            |
| GitHub              | Search Github Issues        | `ghi <search query>` | `ghi can't download internet` |
| Twitter             | Go to profile               | `tw <handle>`        | `tw @marcocampana`            |
| Twitter             | Search                      | `tw <search query>`  | `tw rust lang`                |
| Amazon              | Search Amazon.com           | `a <search query>`   | `a 4k tvs`                    |

## Installation

### Install from Crates.io

If you are a Rust programmer you can install `roo` with `cargo`

```bash
cargo install roo
```

### Install with Homebrew (Mac)

COMING SOON

### Check out the repository

If you are on MacOs, installing with homebrew is the recommended choice. However, you can also check out the repository on your machine and run with `cargo`.

1. [Install Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
1. Check out the repository from GitHub

    ```bash
    git checkout git@github.com:marcocampana/roo.git
    ```

1. Run with Cargo

    ```bash
    cargo run
    ```

## How to

### Start the server

If you installed the binary, you can start the server by executing the `roo` binary in your terminal:

```bash
roo
```

or if you using `cargo` you can run:

```bash
cargo run
```

This will start `roo` on `127.0.0.1:3030` and expects a file named `rules.toml` in the current directory.

You can use the `--help` argument to print all the available arguments you can use to run `roo`.

```bash
roo --help
```

### Add `Roo` as your default search engine in Chrome

1. Go to `chrome://settings/searchEngines`
2. Click on `Add` under `Search engines` and use this URL `http://127.0.0.1:3030/?q=%s`
3. Optionally set this as your default search engine for maximum productivity™

### Define your own commands

Roo takes your search query and tries to match it against the list of defined commands. Buil-in commands that ship with `Roo` are found in the [rules.toml](rules.toml) file. You can make your own file and tell `roo` where to find it at startup:

```bash
roo --path ~/my_file.toml
```

A `roo` command looks like this:

```toml
[[command]]
 input = "g (.+)"
 output = "https://www.google.com/search?q={}"
```

the `input` string is a regex that is matched against the input of your browser's search bar. If a match is found the regex group captured in `()` is replaced in the `{}` found on the output string where the browser redirects. In the example above we are defining a command `g` that will match a string `g <search query>` that redirects to Google search to search for `<search query>`.

### Example: JIRA tasks

If you are a JIRA user you know that JIRA task ids have the format `<PROJECT_PREFIX>-<TASK_NUMBER>`. You could write a custom command that allows you to just input the task id in your browser and be redirected to the task details. For example, let's assume that your project prefix is `MP` you could define the following rule:

```toml
[[command]]
 input = "(MP-.+)"
 output = "https://marcocampana.atlassian.net/browse/{}"
```

## Run in the background

On MacOs you can setup `launchd` to automatically run `roo` in the background.

1. create a `plist` file in the `~/Library/LaunchAgents` folder (make sure to specify the absolute path
to the code in your machine instead of`/Users/myuser`)

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
 <dict>
  <key>Label</key>
  <string>com.marcocampana.roo</string>
  <key>RunAtLoad</key>
  <true/>
  
  <key>WorkingDirectory</key>
  <string>/User/myuser/code/roo</string>

  <key>ProgramArguments</key>
     <array>
        <string><PATH_TO>/User/myuser/.cargo/bin/cargo</string>
        <string>run</string>
     </array>
 </dict>
</plist>
```

2. Load the configuration to start the server:

 ```bash
 launchctl load ~/Library/LaunchAgents/com.marcocampana.roo.plist
 ```

## How to contribute

Fork this repo and send a pull request. Make sure to have unit tests for any new functionality

You can run the tests with:

```bash
cargo test
```