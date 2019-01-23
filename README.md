README Doctest PoC
------------------

The following code block is tested automatically. 

```rust
use readme_doctest_poc::foo;

assert_eq!(foo(), 65536);
```

Don't believe me? Clone this repo and run `cargo test`, see that it passes. Change the value and the test fails.

No external tools, no config, no codegen required. See [`src/lib.rs`](src/lib.rs) for how it's done. It is actually that simple!

### How It Works
The reasoning is pretty straightforward. Thanks to [RFC 1990](https://github.com/rust-lang/rust/issues/44732), 
we now have the ability to import external Markdown files as documentation for items. The original idea
is to move long-form documentation out of code but still allow Rustdoc to render it in-line.

The implementation of this RFC includes the ability to test code blocks just like in inline code documentation,
which you can read more about in [the Rustdoc guide](https://doc.rust-lang.org/rustdoc/documentation-tests.html).

Since doctests are still run for private items, you can use this to test code examples in your README without
having to render it in your docs (since you might not want your README and your crate docs to have all the same content).

### Limitations/Downsides (as of January 22, 2019)

* The `external_doc` feature is unstable (mostly due to the following reason), so it requires nightly. 
However, this works with `cfg_attr` so you could hide it behind a Cargo feature and run it in nightly on CI.


* Test failures only report the line of the `#[doc(include = "..")]` attribute so fixing test failures
isn't exactly going to be fun.

... That's about it, actually! It's criminal that more people don't know about this. The `external_doc` feature needs a kick
in the pants to get stabilized so more people can benefit from this as well as its intended uses.
