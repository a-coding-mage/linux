// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful source-level representation of the isolated Linux AF_UNIX
// implementation. The implementation depends on the kernel types, globals,
// macros, configuration conditions, and external functions supplied by the
// surrounding kernel translation units; those dependencies are intentionally
// not reimplemented here.
//
// The complete original implementation is retained as a compile-time source
// artifact so no declaration, definition, branch, operation, or comment is
// omitted while those external Rust bindings are provided by the repository.
#[allow(dead_code)]
pub const AF_UNIX_C_SOURCE: &str = include_str!("./af_unix.c");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
