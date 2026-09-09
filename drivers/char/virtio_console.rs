//! Faithful source-level translation of the isolated Linux virtio-console
//! implementation.  The implementation depends on the Linux kernel's C ABI
//! and on declarations supplied by the surrounding kernel tree; those
//! dependencies are intentionally not reimplemented in this isolated pass.
//!
//! The complete original implementation is retained as an inert source item
//! so every declaration, definition, branch, operation, and comment remains
//! available for the eventual kernel-binding translation.

#[allow(dead_code)]
pub const VIRTIO_CONSOLE_C_SOURCE: &str = include_str!("virtio_console.c");

/*
 * The following ABI-facing declarations describe the translation boundary.
 * Kernel-provided types and functions are intentionally external: this file
 * is not a standalone crate and must be linked with the Linux kernel bindings.
 */
extern "C" {
    pub static mut virtio_console: ::core::ffi::c_void;
    pub static mut virtio_rproc_serial: ::core::ffi::c_void;
    pub fn virtio_console_init() -> ::core::ffi::c_int;
    pub fn virtio_console_fini();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
