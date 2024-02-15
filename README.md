# Artist DB
Database of artists' social media links

# General dev stuff
## Rust
Requires [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) to be installed

Must be compiled first before running/developing the Nuxt app

Everything related contain in the `__rust__` directory
- `artist-json-builder` | `tool`
  - `artists.toml` -> multiple `<name>.json` files
  - `cargo build --release` and place it in the root directory
  - `-w` to watch file changes, `-i` input toml file, `-o` output directory

- `unavatar` | `wasm`
  - Faster data processing + I want to try `rust-wasm`
  - `import { ... } from "~/composables/unavatar"`
  - `pnpm build:wasm` to (re)build the library
  - `pnpm dev`, `pnpm generate` won't because I don't want to complexify the Github Action script.

## Nuxt
- `pnpm i`, `pnpm dev`, `pnpm generate`, `pnpm lint`

# Motivation
I repost art from various places to my Telegram channel, with full credits, I need to include
- Link to the original post
- Link to artists' various social media

Copying from post to post wasn't a solution
- What if they remove their account from a platform?
- What if they change their username handler?

I want to create a solution that satisfies
- One link - many places, basically a [linktr.ee](https://linktr.ee) clone
- Generate a static site (so I can use Github Pages)
- Can easily modify the DB on my phone

# 🦆
Bottom to top

<samp>

## 🔁 Refactoring

Fuzzy matching string is something I would do if I don't control the input, a.k.a. writing software for others, but in this case, I can. So I'm replacing the `supported_socials` and `unavatar_socials` Vectors with HashMaps.

## ❓ Re-evaluate my decision

Realizing I could leverage the power of the build step with `artist-json-builder` and produce standardized JSON files, I removed the wasm part.

## 🔨 restructure "artists.toml"

Old structure
```toml
[foo]
twitter = "bar"
twitter = "baz" # result in parsing error
```
While implementing the `unavatar` wasm in Rust, realized an artist may have multiple accounts on the same platform, and using only the social codes (lowercased versions of the social names) wasn't sufficient, I came up with a new structure
```toml
[foo]
__flag__ = "<country flag>"
__avatar__ = "<avatar link>"
__alias__ = ["<alias1>", "<alias2>"]
"<social>:<description>" = "<username or link>"
```
Split the key into 2 parts by a colon, the first part will be re-formatted to its correct form (e.g. `TWiTter` -> `Twitter), and the second part will be used as a description, later joined by any delimiter I like. If there's no colon, the whole key will be treated as the social name.

The value now has an extra processing layer, if it starts with `http://` or https://, it'll be considered as a link, username otherwise, and will be mapped to the correct social profile link.

There are also 3 new fields
- `__flag__` for the country flag emoji
- `__avatar__` for the avatar link, or specify the social media code for `unavatar`, or omit for auto search inside `unavatar` supported social media
- `__alias__` for the artist's alias, if any
## 🪨 Resolve `artists.toml` fetching

First, I renamed `process-toml` to `artist-json-builder`, get it? It splits `artists.toml` into multiple small, digestible `<username`>.json` files.

I need a faster language than JS to do this, wondering between Go and Rust, since I already had Rust configured for wasm, why not this one too?

Apparently, [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) is a thing, so I set up a `_`_rust__` directory at the root of the project and got to work.

## 🙂 [unavatar](https://unavatar.io/) and wasm in Rust

Fuzzy matching the supported and provided social names requires comparing every possible combination of the 2 lists. Using HashMap was never an option because this check
```
provided.lower().contains(supported.lower())
```

I decided to give wasm in Rust a try. Found [this comment](https://www.reddit.com/r/rust/comments/uv7zct/comment/i9rftq4/?utm_source=share&utm_medium=web2x&context=3) on Reddit, suggesting the best "language" between wasm and JS is plain string, or as simple as possible, I took notes to simplify the "language" between JS and wasm as much as possible.

Each item of an artist's social list contains 2 elements: the social name and their corresponding username, serde_json is a bit too overkill, so I decided to join them with a separator, and split them in the wasm. What character should I use is the next question.

`|`, `:` and `@` are my first thought, but for future reasons, I can't use them.
- `\n` was perfect. There's absolutely no reason there will be a newline in a username or a social name.

## 🏗️ Simplify the workers

Realizing I could post multiple messages on one `worker.postMessage` call on the main thread, I refactor 2 workers (one for updating the DB, another one for getting artists' info) into one.

The first worker's response is whatever in the DB it found, the second one includes a boolean `needRefresh` and the new data if it's `true`

Existing downside: one small change to `artists.toml` causes the whole DB to refresh, despite it being irrelevant to the requesting artist.

## 🔁 Prevent `artists.toml` re-fetch on subsequent page load

Instead of fetching & hashing the `artists.toml` in a web worker, I wrote a script to generate `artists.hash` on the build stage, the web worker will fetch this small latest hash file, and compare it to the current hash in DB, if diff then update, no need to re-fetch the whole `artists.toml` just to compare hashes.

There's a `--watch` flag to watch for file changes and a `--write` flag to sort and write to the `artists.toml` file, opposed to just printing out the sort step for me to manually sort. After realizing I was making my life harder, I decided to nuke the `--write` flag.

I was considering TS until I decided to give JSDoc a go, it was surprisingly usable.

## 🧵 Web Worker & IndexedDB optimization

Migrating the proof of concept to a web worker is done, now using [murmur3](https://en.wikipedia.org/wiki/MurmurHash) to hash the toml content, preventing re-write on IndexedDB on subsequent page loads, however, all page loads still re-fetch the whole `artists.toml` file.

Also, I created a second worker to read from IndexedDB.

## 🗃️ Database on static site

I planned to use a single `artists.toml` file, fetch-decode-insert to IndexedDB on page load > works as a proof of concept.

I originally chose Nuxt (familiarity, DX), but I want to test out new tools (Biomejs for faster lint/format, Turbopack for faster build), then realize they're at a very early stage (as of Feb 2024) -> go back to Nuxt.

</samp>