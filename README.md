# krdict

A Rust CLI wrapper over the [Korean Learners' Dictionary (한국어기초사전) API](https://krdict.korean.go.kr/eng/openApi/openApiInfo) -- sends the same requests the API accepts, but returns clean, pretty-printed JSON instead of the API's native XML.

## Setup

1. Register for a free API key: <https://krdict.korean.go.kr/eng/openApi/openApiRegister>
2. Set `KRDICT_API_KEY`, either in a `.env` file in the working directory or as a normal environment variable.

## Usage

Two subcommands, mirroring the two endpoints the API exposes.

### `search`

Search the dictionary by term, with the same filters the underlying API supports: 
- pagination (`--start`, `--num`)  
- sort order (`--sort dict|popular`) 
- which field to search (`--part word|ip|dfn|exam`)
- translations (`--translated`, `--trans-lang`),
- `--advanced` mode that unlocks additional filters (word category, part of speech, semantic category, syllable count, vocabulary grade, and more).

```sh
krdict search --query 사과
krdict search --query 학교 --translated --trans-lang 1 --num 20
```

### `view`

Look up the full dictionary entry for a word, or a specific entry by its target code:

```sh
krdict view --query 사과
krdict view --method target-code --query 12345
```

## Global flags

- `--export <FILE>` -- also write the JSON response to a file.
- `--quiet` -- suppress stdout/stderr (useful when scripting and only the exit code matters).

## Output

Both subcommands print pretty-printed JSON to stdout. `search` responses are deserialized directly with `quick-xml`; `view` responses go through a small hand-written parser built on `roxmltree` instead, because a `word_info` entry can repeat `<pos>` (a word can genuinely have more than one part of speech) in a way a plain derive-based deserializer can't represent as a single field -- so entries are walked and reassembled by hand, with `pos` coming back as an array, rather than trusting the schema to be regular.
