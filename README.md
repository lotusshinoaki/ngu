# Installation

```
curl -OL https://github.com/lotusshinoaki/retry/releases/download/0.1.1/x86_64-unknown-linux-gnu.zip
unzip x86_64-unknown-linux-gnu.zip
cp x86_64-unknown-linux-gnu/retry </path/to/bin>
```

# Usage

## Retry up to 5 times
```
retry --tries 5 -- <commands_that_often_fail> <parameters> ...
```

## Wait 2 seconds before retrying
```
retry --delay 2 -- <commands_that_often_fail> <parameters> ...
```

## Do exponential backoff
```
retry --delay 2 --backoff 2 --max-delay 64 -- <commands_that_often_fail> <parameters> ...
```

# Build

```
TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu
```
