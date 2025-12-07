# noti-rs

A simple CLI tool for sending desktop notifications on macOS and Linux.

## Installation

```bash
cargo install noti-rs
```

## Usage

```bash
# Send notification with title and message
noti --title "Build Complete" --message "Your project has been built successfully"

# Short form
noti -t "Build Complete" -m "Success"

# Title only
noti -t "Done"

# Message only (title defaults to "noti")
noti -m "Task completed"
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--title` | `-t` | Notification title |
| `--message` | `-m` | Notification message |
| `--help` | `-h` | Show help |

## License

MIT
