// If you want your README.md to double as your crate root docs you can potentially include it with
// `#![doc(include = "../README.md")]`
// and it'll be rendered and its code examples will also be tested.
//
// However, for the meantime you probably don't want to require nightly to render your crate docs.
// That, and you may not always want your README and crate docs to have the exact same
// content; besides styling differences, the README is nowadays more important for showcasing the
// crate's main features while the root docs should focus on showing the user how to actually use 
// the crate (IMO).
#![feature(external_doc)]

pub fn foo() -> u32 { 65536 }

// The kind of item doesn't really matter, though typedefs are nice since they don't appear in the
// binary.
//
// Also it needs to be private so your whole README isn't rendered in your crate docs somewhere.
#[doc(include = "../README.md")]
type _DoctestReadme = ();
