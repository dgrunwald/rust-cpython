# Change Log

## [Unreleased]

[Unreleased]: https://github.com/dgrunwald/rust-cpython/compare/0.5.1...HEAD

## 0.5.1 - 2020-09-08
- [ignore trailing comma in plist parsing][220] (PR by [@lausek])
- [make fields of initialization config public and implement Default][219] (PR by [@indygreg])
- [fix macros without $crate:: access][234] (PR by [@markbt])

[@lausek]: https://github.com/lausek
[220]: https://github.com/dgrunwald/rust-cpython/pull/220
[219]: https://github.com/dgrunwald/rust-cpython/pull/219
[234]: https://github.com/dgrunwald/rust-cpython/pull/234

## 0.5.0 - 2020-04-08
- [properties (attributes with getter/setters defined in Rust][208] (PR by [@markbt])
- [adoption of 2018 edition and general code modernization][204] (PR by [@markbt])
- [reference extraction for slot functions and optional reference extraction][207] (PR by [@markbt])
- [PEP-587 initialization APIs (python3-sys for Python≥3.8)][211] (PR by [@indygreg])
- [more import APIs (python3-sys)][210] (PR by [@indygreg])

[208]: https://github.com/dgrunwald/rust-cpython/pull/208
[204]: https://github.com/dgrunwald/rust-cpython/pull/204
[207]: https://github.com/dgrunwald/rust-cpython/pull/207
[211]: https://github.com/dgrunwald/rust-cpython/pull/211
[210]: https://github.com/dgrunwald/rust-cpython/pull/210

## 0.4.1 - 2020-02-03
- [link-time inconsistency with build config][135] (original PR by [@svevang] adapted as [202])
- [missing `pub` classifier][206] in `PySharedRef` example. (PR by [@Alphare])
- README updates: copyright years, version number in examples

[135]: https://github.com/dgrunwald/rust-cpython/pull/135
[@svevang]: https://github.com/svevang
[202]: https://github.com/dgrunwald/rust-cpython/pull/202
[206]: https://github.com/dgrunwald/rust-cpython/pull/206
[@Alphare]: https://github.com/Alphare

## 0.4.0 - 2020-01-27
- The 0.4.x series is planned to be the last that will support Rust 2015.
- Added [Python 3.8][187] support.
- [Type errors][199] during downcasts now explain what the expected and actual types are. (PR by [@markbt])
- Data items can now be shared between Python objects (e.g. for iterators) using [`PySharedRef`][189]. (PR by [@yuja])
- [`PyList` can now be appended to with `append`][197].  The method for inserting items is renamed to `insert` for consistency.  (PR by [@binh-vu])

[199]: https://github.com/dgrunwald/rust-cpython/pull/199
[197]: https://github.com/dgrunwald/rust-cpython/pull/197
[@binh-vu]: https://github.com/binh-vu
[189]: https://github.com/dgrunwald/rust-cpython/pull/189
[@yuja]: https://github.com/yuja
[187]: https://github.com/dgrunwald/rust-cpython/pull/187

## 0.3.0 - 2019-08-01
- Minumum Rust compiler version is now 1.30.
- Added [Capsule support][169] (PR by [@gracinet])
- Added [Rust Docstring support for instance methods][166], [static methods and class methods][179] (PRs by [@quark-zju] and [@AdamRzepka])
- [Made macros work with Rust 2018][167] (PR by [@derekdreery])
- [Support Rust raw identifiers for method and keyword names][183] (PR by [@quark-zju])
- Added `impl RefFromPyObject for [u8]`. This allows using `&[u8]` as parameter type in `py_fn!`.
  When passing a `bytes` object from Python, this allows accessing the data without a copy.
  (other mutable Python objects will use a defensive copy)

[166]: https://github.com/dgrunwald/rust-cpython/pull/166
[183]: https://github.com/dgrunwald/rust-cpython/pull/183
[@quark-zju]: https://github.com/quark-zju
[167]: https://github.com/dgrunwald/rust-cpython/pull/167
[@derekdreery]: https://github.com/derekdreery
[169]: https://github.com/dgrunwald/rust-cpython/pull/169
[@gracinet]: https://github.com/gracinet
[179]: https://github.com/dgrunwald/rust-cpython/pull/179
[@AdamRzepka]: https://github.com/AdamRzepkaA

## 0.2.1 - 2018-09-28
- Added Python 3.7 support

## 0.2.0 - 2018-02-27
- Added `pub` modifier to `py_class!` syntax: `py_class!(pub class ClassName |py| ...)`
- Changed `obj.extract::<Vec<T>>(py)` to work with any object implementing the sequence protocol; not just lists.
- Added the `buffer` module, which allows safe access to the [buffer protocol](https://docs.python.org/3/c-api/buffer.html).
  This allows zero-copy access to numpy arrays.
- When building with `--feature nightly`, `extract::<Vec<PrimitiveType>>` will try to use the buffer protocol
  before falling back to the sequence protocol.
- [Added support for optional parameters][81] to `py_argparse!`, `py_fn!` and `py_class!` macros. (PR by [@Luthaf])

  Example: `py_fn!(py, function(i: i32 = 0))`
- Made `ObjectProtocol::compare()` available on Python 3.
- Added `ObjectProtocol::rich_compare()`.
- Fixed [non-deterministic segfault][115] in extension modules using `py_class!` (PR by [@markbt])
- Fixed python27-sys [compiler error on ARM][114] (PR by [@ostrosco])
- [Export path to Python interpreter as Cargo variable][119] (PR by [@indygreg])

[81]: https://github.com/dgrunwald/rust-cpython/pull/81
[@Luthaf]: https://github.com/Luthaf
[115]: https://github.com/dgrunwald/rust-cpython/pull/115
[@markbt]: https://github.com/markbt
[114]: https://github.com/dgrunwald/rust-cpython/pull/114
[@ostrosco]: https://github.com/ostrosco
[119]: https://github.com/dgrunwald/rust-cpython/pull/119
[@indygreg]: https://github.com/indygreg

## 0.1.0 - 2016-12-17
- First release that works on stable Rust.

