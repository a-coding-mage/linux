// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright 2019 Google LLC
 */

// Declarations below are supplied by the surrounding kernel/fsverity code.

extern "C" {
    fn read_mapping_folio(mapping: *mut mapping, index: pgoff_t, context: *mut core::ffi::c_void) -> *mut folio;
    fn folio_file_page(folio: *mut folio, index: pgoff_t) -> *mut page;
    fn __filemap_get_folio(mapping: *mut mapping, index: pgoff_t, fgp: u32, order: u32) -> *mut folio;
    fn page_cache_ra_unbounded(ractl: *mut readahead_control, nr_pages: ulong, lookahead_size: ulong);
    fn folio_put(folio: *mut folio);
    fn folio_test_uptodate(folio: *mut folio) -> bool;
    fn memcpy_to_folio(folio: *mut folio, offset: usize, from: *const u8, len: usize);
    fn is_err<T>(ptr: *mut T) -> bool;
    fn err_cast<T, U>(ptr: *mut T) -> *mut U;
    fn warn_on_once(condition: bool) -> bool;
    fn is_aligned(value: usize, alignment: usize) -> bool;
}

// External kernel types.
enum page {}
enum folio {}
enum mapping {}
enum readahead_control {}
enum inode {}
enum fsverity_info {}
type pgoff_t = usize;
type ulong = usize;

/*
 * generic_read_merkle_tree_page - generic ->read_merkle_tree_page helper
 * @inode: inode containing the Merkle tree
 * @index: 0-based index of the Merkle tree page in the inode
 *
 * The caller needs to adjust @index from the Merkle-tree relative index passed
 * to ->read_merkle_tree_page to the actual index where the Merkle tree is
 * stored in the page cache for @inode.
 */
#[no_mangle]
pub unsafe extern "C" fn generic_read_merkle_tree_page(inode: *mut inode, index: pgoff_t) -> *mut page {
    let folio = read_mapping_folio((*inode).i_mapping, index, core::ptr::null_mut());
    if is_err(folio) {
        return err_cast(folio);
    }
    folio_file_page(folio, index)
}

/* EXPORT_SYMBOL_GPL(generic_read_merkle_tree_page); */

/*
 * generic_readahead_merkle_tree() - generic ->readahead_merkle_tree helper
 * @inode: inode containing the Merkle tree
 * @index: 0-based index of the first Merkle tree page to read ahead in the inode
 * @nr_pages: the number of Merkle tree pages that should be read ahead
 *
 * The caller needs to adjust @index from the Merkle-tree relative index passed
 * to ->read_merkle_tree_page to the actual index where the Merkle tree is
 * stored in the page cache for @inode.
 */
#[no_mangle]
pub unsafe extern "C" fn generic_readahead_merkle_tree(
    inode: *mut inode,
    index: pgoff_t,
    nr_pages: ulong,
) {
    let folio = __filemap_get_folio((*inode).i_mapping, index, FGP_ACCESSED, 0);
    if folio == (-ENOENT as isize as *mut folio)
        || (!is_err(folio) && !folio_test_uptodate(folio))
    {
        // DEFINE_READAHEAD(ractl, NULL, NULL, inode->i_mapping, index);
        let mut ractl = readahead_control {};
        page_cache_ra_unbounded(&mut ractl, nr_pages, 0);
    }
    if !is_err(folio) {
        folio_put(folio);
    }
}

/* EXPORT_SYMBOL_GPL(generic_readahead_merkle_tree); */

/* fsverity_fill_zerohash() - fill folio with hashes of zero data block */
#[no_mangle]
pub unsafe extern "C" fn fsverity_fill_zerohash(
    folio: *mut folio,
    offset: usize,
    len: usize,
    vi: *mut fsverity_info,
) {
    let mut off = offset;

    warn_on_once(!is_aligned(offset, (*vi).tree_params.digest_size));
    warn_on_once(!is_aligned(len, (*vi).tree_params.digest_size));

    while off < offset + len {
        memcpy_to_folio(
            folio,
            off,
            (*vi).tree_params.zero_digest,
            (*vi).tree_params.digest_size,
        );
        off += (*vi).tree_params.digest_size;
    }
}

/* EXPORT_SYMBOL_GPL(fsverity_fill_zerohash); */

// Values supplied by the kernel headers.
const FGP_ACCESSED: u32 = 0;
const ENOENT: i32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
