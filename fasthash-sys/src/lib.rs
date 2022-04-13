#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    deref_nullptr,
    rustdoc::broken_intra_doc_links
)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "gen")] {
        include!(concat!(env!("OUT_DIR"), "/fasthash.rs"));
    } else {
        mod fasthash;

        pub use self::fasthash::*;
    }
}
