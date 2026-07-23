# reinze-lib-fun

A set of fun, novelty commands for the [reinze](https://reinze.com) IRC bot. It
builds to a shared library (`.so`) that the `rust-reinze` host loads dynamically
at runtime.

These are meant to be seen by the channel, so they're typically invoked with a
`+` prefix (public output); the same commands also work with `-` for a private
reply. Most take no argument and return a fixed or randomly chosen line; a few
take an argument or fetch from a public API, as noted.

## Commands

### Fortune & Jokes

- `+8ball (question)` — Magic 8-ball. Returns one randomly chosen answer from the
  classic 20-option list (e.g. "It is certain", "Ask again later", "My reply is
  no"), regardless of the question asked.
- `+horoscope (sign)` — Fetches today's daily horoscope for a zodiac sign from a
  public API (alias `+horo`). Requires a recognized sign (full name or a short
  alias like `scorp`, `sag`, `cap`, `aqua`, `pisc`); errors if none is given.
- `+joke` — Fetches a random two-line setup-and-punchline joke from a public API.

### Callouts

- `+liar (nick)` — Accuses a target of lying ("&lt;nick&gt; is a bloody LIAR!").
  Targets the given nick, or the invoking user if none is supplied.
- `+noob (nick)` — Randomly (50/50) declares "&lt;nick&gt; is a silly noob!" or
  "&lt;nick&gt; is not a silly noob!". Targets the given nick, or the invoking
  user if none is supplied.

### Pets & Critters

- `+beaver` — A fixed woodcutting-pet joke on the "how much wood would a woodchuck
  chuck" rhyme.
- `+chinchompa` — A fixed "Squeaka squeaka! KABOOM" line (with color codes).
- `+heron` — A fixed copypasta-style joke about a heron threatening to trash your
  kitchen unless the message is reposted in 10 other channels.
- `+toucan` — A fixed pun: "Toucan play at this game...".

### Flavor & Cameos

- `+golem` — A fixed one-liner: "I'm pretty stoned."
- `+flashbang` — Prints "FLASHBANG" repeated with alternating IRC background/
  foreground color codes, producing a flashing effect in IRC clients.
- `+dra9` — Fixed cameo one-liner: "Dra9 &gt; Sababa".
- `+shrimp` — Fixed cameo one-liner: "SHRIMP DID IT!".
- `+zac` — Fixed cameo quote referencing a `!nick` mishap.

## Building

```sh
cargo build --release
```

This produces `target/release/libreinze_lib_fun.so`. Install it into the
`rust-reinze` host's `plugins/` directory. Install **atomically** — build to a
temp file on the same filesystem, then `mv`/rename it into place — so the host
never loads a partially written library.
