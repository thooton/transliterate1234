transliterate1234
========

Converts UTF-8 `&str` to ASCII `String`. I stole the data file from the popular npm module [transliteration](https://www.npmjs.com/package/transliteration) and wrote 15 lines.

```rust
use transliterate1234::transliterate;

assert_eq!(transliterate("你好，世界"), "NiHao,ShiJie");
assert_eq!(transliterate("Γεια σας, τον κόσμο"), "Geia sas, ton kosmo");
assert_eq!(transliterate("안녕하세요, 세계"), "annyeonghaseyo, segye");
assert_eq!(transliterate("你好, world!"), "NiHao, world!");
```