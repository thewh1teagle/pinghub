# pinghub

Get push notifications from Github without checking the Email.

# Setup

1. Copy [`.env.example`](.env.example) to `.env` and fill the values.
2. Create cron job for `notirust`:

Open edit mode with
```console
crontab -e
```

Paste the following rule (Run every 5 minutes)
```
*/1 * * * * /home/pi/pinghub/target/release/pinghub
```

# Build

```console
cargo build --release
```

# Build for RPI4

```console
rustup target add aarch64-unknown-linux-gnu
```