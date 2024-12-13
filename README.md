# pinghub

Get push notifications from Github without checking the Email.

## Setup

1. Install [Docker](https://docs.docker.com/engine/install/debian/)
2. Copy [`.env.example`](.env.example) to `.env` and fill the values.
3. Run

```console
docker compose up -d
```

## Build

```console
docker build -t pinghub
docker tag pinghub:latest
docker run pinghub
```
