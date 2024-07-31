<div align="center">

<img src="assets/BigBlobhajHug.svg" width="128" height="128" />

# Blåhaj

</div>

Blahaj: A blazingly fast Discord bot written in Rust. 🚀🚀.

## Building

Try it using nix:

```sh
nix shell github:isabelroses/blahaj
```

Or build manually:

```sh
git clone https://github.com/isabelroses/blahaj
cd blahaj
cargo build
```

> [!WARNING]
> Minimum supported rust version: `1.78.0`.

## Usage

Before starting the bot, create an appliction in the [Discord Developer Portal](https://discord.com/developers/applications).

In the appliction, go to the 'Bot' tab and click 'Reset Token' to get your token.

**Make sure the bot has all three intents enabled**.

To run the bot, a couple of environment variables need to be set
(These can also be set in a `.env` file):

| Env Var | Optional | Description |
| ------- | -------- | ----------- |
| `$DISCORD_TOKEN` | No | The token for the Discord bot that you just created. |
| `$NIXPKGS_URL` | Yes | The url to the version of nixpkgs you want to use for the `/nixpkgs` command. |
| `$GITHUB_TOKEN` | Yes | Github API token. |
| `$DATABASE_PATH` | Yes (unused) if built without `database` feature flag | Path to sqlite database. |

## Thanks

Thanks to this [u/heatherhorns on Reddit](https://www.reddit.com/r/BLAHAJ/comments/s91n8d/some_blahaj_emojis/) for the icon.
