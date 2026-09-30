// SPDX-License-Identifier: GPL-2.0-only
//! Final build timestamp owner, linked after the preliminary weak definitions.

#[allow(
    clippy::all, dead_code, missing_docs, non_camel_case_types, non_snake_case,
    non_upper_case_globals, improper_ctypes, unsafe_op_in_unsafe_fn, unreachable_pub
)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/init_version_timestamp_generated.rs"));
}

mod version_data;

/// Initial UTS namespace containing the final build identity.
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut init_uts_ns: bindings::uts_namespace =
    // SAFETY: the pointer names this static for the self-linked namespace lists.
    unsafe { version_data::namespace(core::ptr::addr_of_mut!(init_uts_ns)) };

/// Fixed boot banner consumed as a NUL-terminated C array.
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static linux_banner: [u8; bindings::RUST_VERSION_BANNER.len()] =
    *bindings::RUST_VERSION_BANNER;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
