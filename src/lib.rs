#![doc = include_str!("../README.md")]
#![no_std]
extern crate alloc;

// On VS Code
// - install https://github.com/ruschaaf/extended-embedded-languages
// - and prefix the raw string with `/*toml*/ ` - see https://github.com/ruschaaf/extended-embedded-languages#embedded-languages
#[doc = r"123\n\n1"]
#[doc = r"123\n\n1( { []} )"]
#[doc = /*toml*/ r#"
    a = "b"
    [xx]
    y = 1
    [dd.xx]
    [[x]]
    h = 1.0
    q = { y = 1. b = 2}
    serde = { version = "1.0.113", features = ["derive"] }
"#]
#[doc = /*toml*/ r#"
a = "b"
[xx]
y = 1
[dd.xx]
h = 1.0
q = { y = 1. b = 2}
"#]
pub mod misc {
    /// Intentionally NOT public.
    pub(crate) struct SealedTraitParam {}
    pub trait SealedTrait {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &SealedTraitParam);
    }

    /* //@TODO
    /// Intentionally NOT public.
    #[allow(dead_code)]
    struct SealedTraitImpl {}
    impl SealedTrait for SealedTraitImpl {
        fn _seal(_: &SealedTraitParam) {}
    }*/
}

const _: &str = /*toml*/
    r#"
a = "b"
[xx]
y = 1
[dd.xx]
h = 1.0
q = { y = 1. b = 2}
"#;

const _S1: &str = /*json*/
    r#"
    {"a": "b", "c": [1, 2, 3]}
"#;

fn _f() -> &'static str {
    /*json*/
    r#"
    {"a": "b", "c": [1, 2, 3]}
    "#
}

pub mod traits {
    //use alloc::string::String;
    //use alloc::{borrow::ToOwned, string::String};
    //use serde::{Deserialize, Serialize};

    pub mod config {
        //use alloc::string::String;

        pub trait Preamble: crate::misc::SealedTrait {
            fn is_no_preamble(&self) -> bool;
            fn is_copy_verbatim(&self) -> bool;
            // @TODO Should this be ToOwned<&str>?
            fn is_items_with_prefix(&self) -> Option<&str>;
        }

        pub mod headers {
            use alloc::string::String;

            pub trait Inserts: crate::misc::SealedTrait {
                // NOT returning an [Iterator], because [Iterator] would need to be `Box`-ed. We can
                // NOT returrm `impl Iterator<Item = &'a str>``, because then this trait would NOT
                // be dyn-compatible.
                fn inserts<'a>(&'a self) -> &'a [String];

                fn after_insert(&self) -> &str;
            }
        }

        pub trait Headers: crate::misc::SealedTrait {
            fn prefix_before_insert(&self) -> &str;
            fn inserts(&self) -> Option<&dyn headers::Inserts>;
        }
    }

    pub trait Config: crate::misc::SealedTrait {
        fn file_path(&self) -> &str;

        fn preamble(&self) -> &dyn config::Preamble;

        fn ordinary_code_headers(&self) -> Option<&dyn config::Headers>;

        fn ordinary_code_suffix(&self) -> &str;
    }
}

pub mod types {
    use alloc::{borrow::ToOwned, string::String};
    use core::marker::PhantomData;
    use serde::{Deserialize, Serialize};

    pub mod config {
        use alloc::{borrow::ToOwned, string::String};
        use serde::{Deserialize, Serialize};

        /// Whether the very first code block is a preamble that needs special handling.
        ///
        /// Intentionally NOT implementing [Clone], as we don't want user code to make copies.
        #[derive(Serialize, Deserialize, Debug)]
        pub enum Preamble {
            /// No preamble - the very first code block is a non-Preamble block (handled by injecting
            /// any header and/or body strings if set in [crate::Config]).
            NoPreamble,
            /// Expecting a preamble, but no special handling - pass as-is. Any [Headers] and/or
            /// [crate::Config::ordinary_code_suffix] will NOT be applied (prefixed/inserted).
            CopyVerbatim,
            /// Expecting the very first code block to contain `item`s ONLY (as per
            /// [`item`](https://lukaswirth.dev/tlborm/decl-macros/minutiae/fragment-specifiers.html#item)
            /// captured by declarative macros (ones defined with `macro_rules!`)). For example,
            /// `struct` definitions, `use` or `pub use` imports.
            ///
            /// The [String] value is a prefix injected before each item (located in the same preamble,
            /// that is, the very first code block). Example of a potentially useful prefix:
            /// - `#[allow(unused_imports)]`, or
            /// - `# #[allow(unused_imports)]` where the leading `#` makes that line
            ///   [hidden](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#hiding-portions-of-the-example)
            ///   in the generated documentation.
            ItemsWithPrefix(String),
        }
        impl Default for Preamble {
            fn default() -> Self {
                Self::NoPreamble
            }
        }

        pub mod headers {
            use alloc::{borrow::ToOwned, string::String, vec, vec::Vec};
            use serde::{Deserialize, Serialize};

            #[derive(Serialize, Deserialize, Debug)]
            #[serde(default)]
            pub struct Inserts {
                /// A list of strings to be injected after the injected
                /// [crate::config::Headers::prefix_before_insert], and before the beginning of the
                /// existing code of each non-preamble code block. Each string from this list is to be
                /// used exactly once, one per each non-preamble code block. The number of strings in
                /// this list has to be the same as the number of non-preamble code blocks.
                ///
                /// Example of useful inserts: Names of test functions (or parts of such names) to
                /// generate, one per each non-preamble code block.
                pub inserts: Vec<String>,

                /// Content to be injected at the beginning of each non-preamble code block, but AFTER an
                /// insert.
                ///
                /// Example of useful inserts for generating test functions: `() {`.
                pub after_insert: String,
            }
            impl Default for Inserts {
                fn default() -> Self {
                    Self {
                        inserts: vec![],
                        after_insert: "".to_owned(),
                    }
                }
            }
        }

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(default)]
        pub struct Headers {
            /// Prefix to be injected at the beginning of any non-preamble code block, even before an
            /// insert (if any).
            ///
            /// Example of useful prefix: `#[test] fn test_` for test functions to generate.
            pub prefix_before_insert: String,

            pub inserts: Option<headers::Inserts>,
        }
        impl Default for Headers {
            fn default() -> Self {
                Self {
                    prefix_before_insert: "".to_owned(),
                    inserts: None,
                }
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(default)]
    /// To prevent the users on depending on pattern matching completeness etc.
    #[non_exhaustive]
    pub struct Config<S: crate::misc::SealedTrait> {
        _seal: PhantomData<S>,

        // @TODO ToOwned<&str> instead?
        /// **Relative** path (relative to the directory of Rust source file that invoked the chain
        /// of macros). Defaults to "README.md".
        pub file_path: String,

        pub preamble: config::Preamble,

        pub ordinary_code_headers: Option<config::Headers>,

        /// Suffix to be appended at the end of any non-preamble code block.
        ///
        /// Example of useful inserts for generating test functions: `}`.
        pub ordinary_code_suffix: String,
    }

    impl<S: crate::misc::SealedTrait> Default for Config<S> {
        fn default() -> Self {
            Config {
                _seal: PhantomData,

                file_path: "README.md".to_owned(),

                preamble: config::Preamble::NoPreamble,

                ordinary_code_headers: None,
                ordinary_code_suffix: "".to_owned(),
            }
        }
    }
}

mod trait_impls {
    use crate::misc::{SealedTrait, SealedTraitParam};
    use alloc::string::String;

    impl SealedTrait for crate::types::config::Preamble {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &SealedTraitParam) {}
    }
    impl crate::traits::config::Preamble for crate::types::config::Preamble {
        fn is_no_preamble(&self) -> bool {
            matches!(self, Self::NoPreamble)
        }
        fn is_copy_verbatim(&self) -> bool {
            matches!(self, Self::CopyVerbatim)
        }
        fn is_items_with_prefix(&self) -> Option<&str> {
            if let Self::ItemsWithPrefix(s) = self {
                Some(s)
            } else {
                None
            }
        }
    }

    impl SealedTrait for crate::types::config::headers::Inserts {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &SealedTraitParam) {}
    }
    impl crate::traits::config::headers::Inserts for crate::types::config::headers::Inserts {
        fn inserts<'a>(&'a self) -> &'a [String] {
            &self.inserts
        }
        fn after_insert(&self) -> &str {
            &self.after_insert
        }
    }

    impl SealedTrait for crate::types::config::Headers {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &SealedTraitParam) {}
    }
    impl crate::traits::config::Headers for crate::types::config::Headers {
        fn prefix_before_insert(&self) -> &str {
            &self.prefix_before_insert
        }
        fn inserts(&self) -> Option<&dyn crate::traits::config::headers::Inserts> {
            if let Some(inserts) = &self.inserts {
                Some(inserts)
            } else {
                None
            }
        }
    }

    impl<S: SealedTrait> SealedTrait for crate::types::Config<S> {
        #[allow(private_interfaces)]
        fn _seal(&self, _: &SealedTraitParam) {}
    }
    impl<S: SealedTrait> crate::traits::Config for crate::types::Config<S> {
        fn file_path(&self) -> &str {
            &self.file_path
        }
        fn preamble(&self) -> &dyn crate::traits::config::Preamble {
            &self.preamble
        }
        fn ordinary_code_headers(&self) -> Option<&dyn crate::traits::config::Headers> {
            if let Some(headers) = &self.ordinary_code_headers {
                Some(headers)
            } else {
                None
            }
        }
        fn ordinary_code_suffix(&self) -> &str {
            &self.ordinary_code_suffix
        }
    }
}

/// Internal, used between crates `readme-code-extractor-core` and `readme-code-extractor` to
/// assure that they're of the same version.
#[doc(hidden)]
pub const fn is_exact_version(expected_version: &'static str) -> bool {
    matches!(expected_version.as_bytes(), b"0.1.0")
}

#[doc(hidden)]
const _ASSERT_VERSION: () = {
    if !crate::is_exact_version(env!("CARGO_PKG_VERSION")) {
        panic!("prudent-rs/readme-code-extractor-core is of different version than expected.");
    }
};
