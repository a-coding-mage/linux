// SPDX-License-Identifier: GPL-2.0
/*
 * Symlink inode operations for Coda filesystem
 * Original version: (C) 1996 P. Braam and M. Callahan
 * Rewritten for Linux 2.1. (C) 1997 Carnegie Mellon University
 * 
 * Carnegie Mellon encourages users to contribute improvements to
 * the Coda project. Contact Peter Braam (coda@cs.cmu.edu).
 */

// Linux and Coda declarations are supplied by the translated dependencies.

unsafe fn coda_symlink_filler(file: *mut file, folio: *mut folio) -> i32 {
    let inode: *mut inode = (*(*folio).mapping).host;
    let mut error: i32;
    let cii: *mut coda_inode_info;
    let mut len: u32 = PAGE_SIZE;
    let p: *mut core::ffi::c_char = folio_address(folio);

    cii = ITOC!(inode);

    error = venus_readlink((*inode).i_sb, &mut (*cii).c_fid, p, &mut len);
    folio_end_read(folio, error == 0);
    error
}

pub static coda_symlink_aops: address_space_operations = address_space_operations {
    .read_folio: Some(coda_symlink_filler),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
