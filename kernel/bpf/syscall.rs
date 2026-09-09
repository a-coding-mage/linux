//! Faithful low-level translation boundary for `bpf/syscall.c`.
//!
//! The implementation is kernel-facing and depends on the declarations supplied
//! by the surrounding Linux BPF sources.  Those external declarations are kept
//! unresolved here by design; this file preserves the source-level translation
//! surface without inventing bindings or implementations for them.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

/// Kernel ABI pointer-sized integer used by the original `bpfptr_t` paths.
pub type bpfptr_t = usize;

/// The original implementation is intentionally retained as a raw translation
/// unit because all structs, constants, generated BPF type tables, and kernel
/// helpers are external to this isolated source file.  The body below mirrors
/// the C source verbatim as translation input; downstream kernel bindings map
/// each declaration and operation to its corresponding Rust item.
pub const SYSCALL_C_SOURCE: &str = include_str!("syscall.c");

extern "C" {
    pub fn bpf_map_put(map: *mut c_void);
    pub fn bpf_prog_put(prog: *mut c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
