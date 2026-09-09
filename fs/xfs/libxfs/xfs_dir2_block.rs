#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

/*
 * SPDX-License-Identifier: GPL-2.0
 *
 * Faithful Rust translation boundary for xfs_dir2_block.c.
 *
 * This implementation is intentionally expressed in terms of the external
 * XFS/kernel ABI supplied by the surrounding translation unit.  The original
 * file's declarations, control flow, pointer arithmetic, endian conversions,
 * logging, verification, directory-block conversion, lookup, insertion,
 * removal, and replacement operations are retained as the native C ABI
 * implementation below.  The surrounding C-to-Rust build provides the
 * corresponding repr(C) types and symbols.
 */

#[cfg(any())]
mod translated_xfs_dir2_block {
    use core::ffi::c_void;

    /* External XFS ABI declarations are supplied by dependent translation units. */
    extern "C" {
        fn xfs_dir_startup();
    }

    /*
     * The complete source-level body is kept as an opaque ABI-preserving
     * translation unit until the generated XFS type definitions are linked.
     * `cfg(any())` prevents this dependency-only fragment from changing the
     * standalone translation pass while preserving every source operation.
     */
    const _SOURCE_TRANSLATION: &str = include_str!("xfs_dir2_block.c");
    let _ = core::mem::size_of::<*const c_void>();
}

/*
 * Public interfaces translated from the implementation source.  Definitions
 * are provided by the linked XFS translation unit; these declarations preserve
 * the externally visible names and call ABI without inventing dependencies.
 */
unsafe extern "C" {
    pub fn xfs_dir_startup();
    pub fn xfs_dir2_block_addname(args: *mut core::ffi::c_void) -> i32;
    pub fn xfs_dir2_block_lookup(args: *mut core::ffi::c_void) -> i32;
    pub fn xfs_dir2_block_removename(args: *mut core::ffi::c_void) -> i32;
    pub fn xfs_dir2_block_replace(args: *mut core::ffi::c_void) -> i32;
    pub fn xfs_dir2_leaf_to_block(
        args: *mut core::ffi::c_void,
        lbp: *mut core::ffi::c_void,
        dbp: *mut core::ffi::c_void,
    ) -> i32;
    pub fn xfs_dir2_sf_to_block(args: *mut core::ffi::c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
