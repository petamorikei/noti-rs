# noti

A simple CLI tool for sending desktop notifications on macOS and Linux.

## Installation

```bash
cargo install noti
```

## Usage

```bash
# Send notification with title and body
noti --title "Build Complete" --body "Your project has been built successfully"

# Short form
noti -t "Build Complete" -b "Success"

# Title only
noti -t "Done"

# Body only (title defaults to "noti")
noti -b "Task completed"
```

## Options

| Option | Short | Description |
|--------|-------|-------------|
| `--title` | `-t` | Notification title |
| `--body` | `-b` | Notification body |
| `--help` | `-h` | Show help |

## License

MIT
