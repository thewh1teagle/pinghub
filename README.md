# pinghub

Get push notifications from Github without checking the Email.

# Setup

1. Create bot in [`@BotFather`](https://t.me/BotFather)
2. Copy [`.env.example`](.env.example) to `.env` and fill the values.
3. Create cron job for `pinghub`:

Open edit mode with
```console
crontab -e
```

Paste the following rule (Run every 5 minutes)
```
*/5 * * * * /home/pi/pinghub/target/release/pinghub
```

# Build

```console
cargo build --release
```

# Cross compile for Linux from macOS

```console
rustup target add x86_64-unknown-linux-gnu
brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=x86_64-unknown-linux-gnu-gcc cargo build --target=x86_64-unknown-linux-gnu --release
```