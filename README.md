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

Look up the full dictionary entry for a word, or a specific entry by its target code. The default method, `word-info`, needs the word's homograph number (`sup_no`) glued directly onto the query -- e.g. `사과1` for the first "사과" entry -- since that's what disambiguates entries that share spelling; `search` results include `sup_no` for each item so you know which number to use. `target-code` takes the numeric `target_code` from a `search` or `view` result instead:

```sh
krdict view --query 사과1
krdict view --method target-code --query 66371
```

## Global flags

- `--export <FILE>` -- also write the JSON response to a file.
- `--quiet` -- suppress stdout/stderr (useful when scripting and only the exit code matters).

## Output

Both subcommands print pretty-printed JSON to stdout. `search` responses are deserialized directly with `quick-xml`; `view` responses go through a small hand-written parser built on `roxmltree` instead, because the API repeats `<pos>` verbatim for every `word_info` entry (an XML quirk on their end, not a word with multiple parts of speech -- homographs already get distinct `sup_no` values), which a plain derive-based deserializer can't collapse the way a hand-rolled walk over the tree can.

`krdict search --query 사과 --num 1` (trimmed to one sense for brevity):

```json
{
  "title": "한국어 기초사전 개발 지원(Open API) - 사전 검색",
  "link": "https://krdict.korean.go.kr",
  "description": "한국어 기초사전 개발 지원(Open API) - 사전 검색 결과",
  "lastBuildDate": "20260723025825",
  "total": 6,
  "start": 1,
  "num": 1,
  "item": [
    {
      "target_code": 66371,
      "word": "사과",
      "sup_no": 1,
      "origin": "沙果/砂果",
      "pronunciation": "사과",
      "word_grade": "초급",
      "pos": "명사",
      "link": "https://krdict.korean.go.kr/kor/dicSearch/SearchView?ParaWordNo=66371",
      "sense": [
        {
          "sense_order": 1,
          "definition": "모양이 둥글고 붉으며 새콤하고 단맛이 나는 과일.",
          "translation": []
        }
      ],
      "example": null
    }
  ]
}
```

`krdict view --query 사과1` (example/multimedia lists trimmed for brevity -- a real entry can have several of each):

```json
{
  "title": "한국어 기초사전 개발 지원(Open API) - 사전 내용 검색",
  "link": "https://krdict.korean.go.kr/dicSearch/SearchView?ParaWordNo=66371",
  "description": "한국어 기초사전 개발 지원(Open API) - 사전 내용 검색 결과",
  "lastBuildDate": "20260723025842",
  "total": 1,
  "item": [
    {
      "target_code": 66371,
      "word_info": {
        "word": "사과",
        "sup_no": 1,
        "word_unit": "단어",
        "pos": ["명사"],
        "word_type": "한자어",
        "original_language_info": { "original_language": "沙果/砂果", "language_type": "한자" },
        "pronunciation_info": { "pronunciation": "사과" },
        "conju_info": null,
        "der_info": [],
        "ref_info": [],
        "word_grade": "초급",
        "category_info": [
          { "type": "의미 범주", "written_form": "식생활 > 과일" },
          { "type": "주제 및 상황 범주", "written_form": "음식 주문하기" }
        ],
        "sense_info": [
          {
            "definition": "모양이 둥글고 붉으며 새콤하고 단맛이 나는 과일.",
            "reference": null,
            "pattern_info": [],
            "example_info": [
              { "type": "문장", "example": "어머니는 후식으로 사과를 깎아 주셨다." },
              { "type": "문장", "example": "언니는 과일 중에 사과를 제일 좋아한다." }
            ],
            "rel_info": [],
            "multimedia_info": [
              { "label": "사과", "type": "사진", "link": "https://krdicmedia.korean.go.kr/front/search/searchResultView.do?file_no=195314" }
            ],
            "subword_info": []
          }
        ]
      }
    }
  ]
}
```
