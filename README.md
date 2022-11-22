# Roo - A smart bookmarking search engine for your browser

![Dall-E 2 generated logo](static/roo-logo.png)

## Introduction

Roo is a smart bookmarking and productivity tool that works as a custom search engine in your browser, allowing you to quickly issue commands in your browser's search bar and customize the output. You can use this tool locally or run it on an internal network to share it with your team.

It's a simple server that accepts a search query and redirects to the appropirate URI based on what command your input matches.

Roo is heavily inspired by [bunny1](https://github.com/ccheever/bunny1), a bookmarking tool created at Facebook and opensourced in 2012. It's written in Rust and released under the ... license.

## Installation

### Install from Crates.io

If you are a Rust programmer you can install `roo` with `cargo`

```bash
cargo install roo
```

Here are instructions on how to install Rust and Cargo.

### Install with Homebrew (Mac)

1. Install with Homebrew
1. Run the server

### Other systems

If you are on Mac, installing with homebrew is the recommended choice. However you can also checkout the repository on your machine and run with cargo.

1. Install Rust and Cargo
1. Checkout the repository
1. cargo run

```bash
cargo run
```

Run with custom port:

```bash
cargo run -- --port 8000
```

### Run in the background

On mac you can setup `launchd` to automatically run roo in the background.

1. create a `plist` file in the `~/Library/LaunchAgents folder

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
        <string>/Users/marcocampana/code/roo</string>
        <key>ProgramArguments</key>
        <array>
            <string>/Users/marcocampana/.cargo/bin/cargo</string>
            <string>run</string>
        </array>
    </dict>
</plist>
```

2. Load the configuration and start the server

 ```bash
 launchctl load ~/Library/LaunchAgents/com.marcocampana.roo.plist
 ```

## Built-in commands

Roo comes with a set of default commands. You can fully customize the

| Tool                | Description                 | Command             | Example               |
|---------------------|-----------------------------|---------------------|-----------------------|
| Google              | Search Google.com           | `g <search query>`  | `g Rust language`     |
| Github              | Search Github.com           | `gh <search query>` | `gh marcocmapana/roo` |
| Gmail               | Create a new mail in Gmail  | `newmail`           | `newmail`             |
| Google Docs         | Create a new GDoc           | `newdoc`            | `newdoc`              |
| Twitter             | Go to profile               | `tw <handle>`       | `tw @marcocampana`    |
| Twitter             | Search                      | `tw <search query>` | `tw rust lang`        |
| Amazon              | Search Amazon.com           | `a <search query>`  | `a 4k tvs`            |

## How it works

TODO

## How to contribute

Run unit tests

```bash
cargo test
```

## TODO
- Add Duckduckgo search
- Add Youtube
- Add Pinterest search
- Add Reddit search
- Add gmail create new email to recipient
- Add new GSheet 
- Search google maps
- Add AWS handles