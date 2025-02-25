## jpvoca

in progress...


### ideation: menu
without a parameter
```sh
$> jpvoca

1. add a word
2. delete a word
3. review
4. quit
> 
```

with parameters
```sh
$> jpvoca 食べる たべる "to eat"

	食べる (たべる) - "to eat" added

$> _
```

### ideation: init
when first running `jpvoca`, create a hidden folder `.jpvoca` at the root with `vocab.json` inside.

### ideation: adding a word
```
$> jpvoca
> 1

Kanji: 食べる
Kana: たべる
Definition: to eat
Sentence: 朝ごはんを食べました 
```

```json
{
	0: {
		"kanji": "食べる",
		"kana": "たべる",
		"definition": "to eat",
		"sentence": "朝ごはんを食べました",
		"date_added": "2025-02-18"
	}
}
```
