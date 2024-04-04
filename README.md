# Artist DB
Database of artists' social media links

# General dev stuff
## Rust
Requires [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) to be installed

Must be compiled first before running/developing the Nuxt app
- `bun run build-tool`: the tool to generate files for separate artists from one .toml file and watch for changes
- `bun run build-bridge`: the wasm bridge to communicate between the generated files and the Nuxt app

## Nuxt
- `bun run i`, `bun run --bun dev`, `bun run --bun generate`, `bun run --bun lint`

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

## 📃 Custom format, no more wasm

An effort to simplify the process.

### Backend format

```
<username>,<display name>[,<avatar || _>,<alias 1>,<alias 2>...]
<*name@code || social link>[,<display name>]
...

```

- For each artist, the first line is the main info, the rest is their socials
- Information line:
    - `<username>`: the artist's username, use internally for indexing
    - `<display name>`: the text to be render on the frontend, a.k.a the display name and the flag. This eliminates the need for a separate field for the flag
    - `<avatar>` (optional): the artist's avatar, if not provided (or use `_` if need to specify alias), will be inferred from the socials
    - `<alias 1>`, `<alias 2>`, ... (optional): the artist's alias, if any
- Socials:
    - 1st parameter:
        - `*`: render the link as special on the frontend
        - 1st case: if contains `@`, infer as `username@social_code`
        - 2nd case: if starts with `//`, infer as a link
        - everything else: throw a warning and ignore
    - `<display name>` (optional): the text to be render on the frontend, should be provided if the 1st parameter is a link
- Each artist's info is separated by a newline
- To use `,` in the display name or `*` in username, escape it with `\`

### Frontend format
Will looks near identical to the backend format, with few differences

```
<display name>,<avatar url || _>
<*social link>,<display name>
...

With this custom format, there's no need for bitcode, wasm or anything fancy in the middle
```

- All aliases are removed
- `<display name>` always be provided
- `<avatar>` transformed into `<avatar url>`; use `_` to use the default `/avatar.svg` instead. If the url starts with
- `username@social` transformed into `<social link>`

## ✅ Small updates
Improved TOML structure
```toml
[loremipsum] # will be lowercased
name = "Mr. Lorem Ipsum"
avatar = "loremipsum@google" # or url, starts with `http://` or `https://`, using http will results in a warning.
flag = "💡" # doesn't matter what it is, will be formatted into `<name> <flag>`
alias = ["lorem", "ipsum"] # optional, will be lowercased, if one exists as another artist's name, or in another artist's alias, it will be removed

# case: social code is supported
[loremipsum.social]
code = "google" # get
name = "LoremIpsum"
desc = "How to contact me" # will be formatted as `Google | How to contact me`, if empty then just `Google`
link = "https://google.com/profile/loremipsum" # will be removed

# case: social code not supported, or too lazy to add support
[loremipsum.social]
# code = "???" # warn "social not supported" if specify; omit to infer this as personal website
name = "LoremIpsum" # I guess it doesn't use for anything
url = "http://example.com/user/loremipsum" # warn if doesn't specify
special = false # default: true if this was infer as personal website OR it's linktr.ee, carrd.co,..., it will have special styling on the frontend
desc = "🤷"

# a cleaner way to write the above, also will be how the file will be formatted
# socials = [
#     { code = "google", name = "Lipsum", desc = "Msg me", special = false },
#     # ...
# ]

```

## 🦀 Some tricks I discovered while building the `artist-2-bincode-rs`

> Plan 1: file-based watcher

```
watch `.toml` file -> modified -> (read -> process -> format) -> write to `.toml`
```

this would cause an infinite loop

> Plan 2: hash-based watcher

```
let prev_hash;

watch -> modified -> hash(new) ->  hash changed -> (RPF) -> write -> update prev_hash
```

sure but what about the formatter of `Even Better TOML`?
- EBT allows breaking arrays into multiple lines if it's too long
- `toml_edit` won't

> Plan 3: content-based watcher
```
let prev_bincode: Vec<u8>;

watch -> modified -> bincode(new) ->  bincode changed -> (RPF) -> write -> update prev_bincode
```

this way I can ignore the format, and only focus on the content, but there's a bug and an improvement I can make
- instead of storing the previous `bincode` & comparing 2 `bincode`, pre-`murmurhash` them first
- there's a post-processing step after parsing the `.toml`, what if the content DOES change pre-process, but stays the same post-process?

> Plan 4: hashed-content-based + pre-post comparison

```
let preprocess_hash;
let postprocess_hash;

let content = parse(toml)
preprocess_hash = hash(bincode(content))

let processed_content = process(content)
postprocess_hash = hash(bincode(processed_content))

let content_changed_after_post_process = preprocess_hash != postprocess_hash
```

## ⚒️ Refactor again?

Seems like Nuxt allows developers to specify the `srcDir`, I'm thinking about `src-nuxt` and `src-rust` to replace the root for Nuxt and `__rust__` for Rust.

## 💖 rust analyzer

By creating another rust library and importing it in both the builder and the wasm, I can refactor and rename any shared structs, enums, and functions in any of the 3 projects, and the changes will be reflected in the other 2.

## ⚡ wasm is back, no more json

After watching [this video](https://youtu.be/MuCK81q1edU) from ThePrimeagen, I switch from `json` to `bincode` to se/deserialize the artist's data.

## 🔨 Restructure "artists.toml" (again)

Using arbitrary `key/value` as `socialcode:description/username` wasn't a good idea. Here's the re-design, TOML-indented

```toml
[artist]
"flag" = "🇻🇳"
"avatar" = "https://example.com/avatar.jpg"
"alias" = ["alias1", "alias2"]

[[artist.social]]
code = "instagram"
description = "Life"
username = "username"
link = "https://instagram.com/username"

[[artist.social]]
code = "deviantart"
description = "Art"
username = "username"
link = "https://twitter.com/username"
```

Does it look more verbose, less ambiguous, more structured, easier to read, easier to parse, and easier to modify? Yes.

Am I reconsidering my life decisions? Yes.

Using inline tables makes it look cleaner

```toml
[artist]
...
socials = [
  { code = "instagram", description = "Life", username = "username", link = "https://instagram.com/username" },
  { code = "deviantart", description = "Art", username = "username", link = "https://twitter.com/username" }
]
```

## ✔️ Write tests when possible

There are 7 total scenarios when parsing the socials:

|              | Social + Description                             | Social               | Description                |
|--------------|--------------------------------------------------|----------------------|----------------------------|
| username     | social, desc, username, profile link (best case) | no desc              | no social, no profile link |
| profile link | no username                                      | no desc, no username | no social, no username     |

Edge case: nothing.

Missing data must be represented by a `None`, got a few cases where I forgot to handle empty strings before writing the tests.

## 🔁 Refactoring

Fuzzy matching string is something I would do if I don't control the input, a.k.a. writing software for others, but in this case, I can. So I'm replacing the `supported_socials` and `unavatar_socials` Vectors with HashMaps.

## ❓ Re-evaluate my decision

Realizing I could leverage the power of the build step with `artist-json-builder` and produce standardized JSON files, I removed the wasm part.


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