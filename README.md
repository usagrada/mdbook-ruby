# mdbook-ruby

mdbookでルビを簡単に振ることができるように作った

`[1文字以上]<<1文字以上>>` のように書かれたものに対しルビを振ることができます。

## Usage

```bash
git clone https://github.com/usagrada/mdbook-ruby.git
cd mdbook-ruby
cargo install --path .
```

Add bool.toml

```toml
[preprocessor.ruby]
```

## Example

[漢字]<<かんじ>>

が以下のようにレンダリングされます

<ruby>漢字<rt>かんじ</rt></ruby>

---
これは mdbook-katex のコードを参考に作っています
