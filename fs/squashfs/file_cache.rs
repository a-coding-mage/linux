// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

// Linux kernel headers and SquashFS headers are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct folio {
    pub mapping: *mut address_space,
}

#[repr(C)]
pub struct address_space {
    pub host: *mut inode,
}

#[repr(C)]
pub struct inode {
    pub i_sb: *mut super_block,
}

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct squashfs_cache_entry {
    pub error: ::core::ffi::c_int,
}

unsafe extern "C" {
    pub fn squashfs_get_datablock(
        sb: *mut super_block,
        block: u64,
        bsize: ::core::ffi::c_int,
    ) -> *mut squashfs_cache_entry;
    pub fn squashfs_copy_cache(
        folio: *mut folio,
        buffer: *mut squashfs_cache_entry,
        expected: ::core::ffi::c_int,
        offset: ::core::ffi::c_int,
    );
    pub fn squashfs_cache_put(buffer: *mut squashfs_cache_entry);
    pub fn ERROR(format: *const ::core::ffi::c_char, ...);
}

/* Read separately compressed datablock and memcopy into page cache */
pub unsafe fn squashfs_readpage_block(
    folio: *mut folio,
    block: u64,
    bsize: ::core::ffi::c_int,
    expected: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let i = (*(*folio).mapping).host;
    let buffer = squashfs_get_datablock((*i).i_sb, block, bsize);
    let res = (*buffer).error;

    if res != 0 {
        ERROR(
            b"Unable to read page, block %llx, size %x\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            block,
            bsize,
        );
    } else {
        squashfs_copy_cache(folio, buffer, expected, 0);
    }

    squashfs_cache_put(buffer);
    res
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
