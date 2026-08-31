// SPDX-License-Identifier: Apache-2.0 OR MIT

// The contents of this file come from the Rust rustc-demangle library, hosted
// in the <https://github.com/rust-lang/rustc-demangle> repository, licensed
// under "Apache-2.0 OR MIT". For copyright details, see
// <https://github.com/rust-lang/rustc-demangle/blob/main/README.md>.
// Please note that the file should be kept as close as possible to upstream.

use std::os::raw::c_char;

// C header dependency: <stddef.h> for size_t.

// Original C condition:
// #if defined(__GNUC__) || defined(__clang__)
// #define DEMANGLE_NODISCARD __attribute__((warn_unused_result))
// #else
// #define DEMANGLE_NODISCARD
// #endif
// Rust translation uses #[must_use] on the corresponding declaration.

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum overflow_status {
    OverflowOk,
    OverflowOverflow,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum demangle_style {
    DemangleStyleUnknown = 0,
    DemangleStyleLegacy,
    DemangleStyleV0,
}

// Not using a union here to make the struct easier to copy-paste if needed.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct demangle {
    pub style: demangle_style,
    // points to the "mangled" part of the name,
    // not including `ZN` or `R` prefixes.
    pub mangled: *const c_char,
    pub mangled_len: usize,
    // In DemangleStyleLegacy, is the number of path elements
    pub elements: usize,
    // while it's called "original", it will not contain `.llvm.9D1C9369@@16` suffixes
    // that are to be ignored.
    pub original: *const c_char,
    pub original_len: usize,
    // Contains the part after the mangled name that is to be outputted,
    // which can be `.exit.i.i` suffixes LLVM sometimes adds.
    pub suffix: *const c_char,
    pub suffix_len: usize,
}

// if the length of the output buffer is less than `output_len-OVERFLOW_MARGIN`,
// the demangler will return `OverflowOverflow` even if there is no overflow.
pub const OVERFLOW_MARGIN: usize = 4;

unsafe extern "C" {
    /// Demangle a C string that refers to a Rust symbol and put the demangle intermediate result in `res`.
    /// Beware that `res` contains references into `s`. If `s` is modified (or free'd) before calling
    /// `rust_demangle_display_demangle` behavior is undefined.
    ///
    /// Use `rust_demangle_display_demangle` to convert it to an actual string.
    pub fn rust_demangle_demangle(s: *const c_char, res: *mut demangle);

    /// Write the string in a `struct demangle` into a buffer.
    ///
    /// Return `OverflowOk` if the output buffer was sufficiently big, `OverflowOverflow` if it wasn't.
    /// This function is `O(n)` in the length of the input + *output* [$], but the demangled output of demangling a symbol can
    /// be exponentially[$$] large, therefore it is recommended to have a sane bound (`rust-demangle`
    /// uses 1,000,000 bytes) on `len`.
    ///
    /// `alternate`, if true, uses the less verbose alternate formatting (Rust `{:#}`) is used, which does not show
    /// symbol hashes and types of constant ints.
    ///
    /// [$] It's `O(n * MAX_DEPTH)`, but `MAX_DEPTH` is a constant 300 and therefore it's `O(n)`
    /// [$$] Technically, bounded by `O(n^MAX_DEPTH)`, but this is practically exponential.
    #[must_use]
    pub fn rust_demangle_display_demangle(
        res: *const demangle,
        out: *mut c_char,
        len: usize,
        alternate: bool,
    ) -> overflow_status;

    /// Returns true if `res` refers to a known valid Rust demangling style, false if it's an unknown style.
    pub fn rust_demangle_is_known(res: *mut demangle) -> bool;
}
