![Rust Book Spanish Translation](/docs/RustBookLogoSpanish.png)
# The Rust Programming Language [Spanish Ed.]

![Build Status](https://github.com/rust-lang/book/workflows/CI/badge.svg)
[![Telegram Group
](https://img.shields.io/badge/Telegram-Group-blue.svg?logo=telegram)](
https://t.me/rust_book_spanish_translation)
[![Discord](https://img.shields.io/discord/778674594856960012?color=blue&label=discord&logo=discord)](https://discord.gg/4ng5HgmaMg)


This repository contains the source of "The Rust Programming Language" book.

This is an unofficial Spanish translation of the book. The original book is
available in [rust-lang/book].

We do a best effort approach to keep this translation up to date with the
original book, but we can't guarantee that it is 100% accurate. If you find
any errors, please open an issue or a pull request or write us in the
[Telegram Group](https://t.me/rust_book_spanish_translation) or our
[Discord Server](https://discord.gg/4ng5HgmaMg)

[rust-lang/book]: https://doc.rust-lang.org/stable/book/

[The book is available in dead-tree form from No Starch Press][nostarch].

[nostarch]: https://nostarch.com/rust-programming-language-2nd-edition

You can also read the book for free online. Please see the book as shipped with
the latest [stable], [beta], or [nightly] Rust releases. Be aware that issues
in those versions may have been fixed in this repository already, as those
releases are updated less frequently.

[stable]: https://doc.rust-lang.org/stable/book/
[beta]: https://doc.rust-lang.org/beta/book/
[nightly]: https://doc.rust-lang.org/nightly/book/

See the [releases] to download just the code of all the code listings that appear in the book.

[releases]: https://github.com/rust-lang/book/releases

## Requirements

Building the book requires a custom fork of [mdBook]:

[mdBook]: https://github.com/RustLangES/mdBook/

```bash
$ cargo install mdbook --git https://github.com/RustLangES/mdBook.git
```

## Building

To build the book, type:

```bash
$ mdbook build
```

The output will be in the `book` subdirectory. To check it out, open it in
your web browser.

_Firefox:_
```bash
$ firefox book/index.html                       # Linux
$ open -a "Firefox" book/index.html             # OS X
$ Start-Process "firefox.exe" .\book\index.html # Windows (PowerShell)
$ start firefox.exe .\book\index.html           # Windows (Cmd)
```

_Chrome:_
```bash
$ google-chrome book/index.html                 # Linux
$ open -a "Google Chrome" book/index.html       # OS X
$ Start-Process "chrome.exe" .\book\index.html  # Windows (PowerShell)
$ start chrome.exe .\book\index.html            # Windows (Cmd)
```

To run the tests:

```bash
$ mdbook test
```

## Contributing

We'd love your help! Please see [CONTRIBUTING.md][contrib] to learn about the
kinds of contributions we're looking for.

[contrib]: https://github.com/rust-lang/book/blob/main/CONTRIBUTING.md

Because the book is [printed][nostarch], and because we want
to keep the online version of the book close to the print version when
possible, it may take longer than you're used to for us to address your issue
or pull request.

So far, we've been doing a larger revision to coincide with [Rust
Editions](https://doc.rust-lang.org/edition-guide/). Between those larger
revisions, we will only be correcting errors. If your issue or pull request
isn't strictly fixing an error, it might sit until the next time that we're
working on a large revision: expect on the order of months or years. Thank you
for your patience!

### Translations

We'd love help translating the book! See the [Translations] label to join in
efforts that are currently in progress. Open a new issue to start working on
a new language! We're waiting on [mdbook support] for multiple languages
before we merge any in, but feel free to start!

[Translations]: https://github.com/rust-lang/book/issues?q=is%3Aopen+is%3Aissue+label%3ATranslations
[mdbook support]: https://github.com/rust-lang-nursery/mdBook/issues/5

## Spellchecking

To scan source files for spelling errors, you can use the `spellcheck.sh`
script available in the `ci` directory. It needs a dictionary of valid words,
which is provided in `ci/dictionary.txt`. If the script produces a false
positive (say, you used word `BTreeMap` which the script considers invalid),
you need to add this word to `ci/dictionary.txt` (keep the sorted order for
consistency).
