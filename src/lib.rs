#![feature(external_doc)]

pub fn foo() -> u32 { 65536 }

#[allow(dead_code)]
#[doc(include = "../README.md")]
type DoctestReadme = ();
