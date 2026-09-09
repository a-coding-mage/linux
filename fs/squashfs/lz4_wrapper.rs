// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2013, 2014
 * Phillip Lougher <phillip@squashfs.org.uk>
 */

// Kernel and local C dependencies are supplied by the surrounding translation.

const LZ4_LEGACY: u32 = 1;

#[repr(C)]
struct lz4_comp_opts {
    version: __le32,
    flags: __le32,
}

#[repr(C)]
struct squashfs_lz4 {
    input: *mut core::ffi::c_void,
    output: *mut core::ffi::c_void,
}

unsafe fn lz4_comp_opts(
    _msblk: *mut squashfs_sb_info,
    buff: *mut core::ffi::c_void,
    len: i32,
) -> *mut core::ffi::c_void {
    let comp_opts = buff as *mut lz4_comp_opts;

    /* LZ4 compressed filesystems always have compression options */
    if comp_opts.is_null() || len < core::mem::size_of::<lz4_comp_opts>() as i32 {
        return ERR_PTR(-EIO);
    }

    if le32_to_cpu((*comp_opts).version) != LZ4_LEGACY {
        /* LZ4 format currently used by the kernel is the 'legacy'
         * format */
        ERROR!("Unknown LZ4 version\n");
        return ERR_PTR(-EINVAL);
    }

    core::ptr::null_mut()
}

unsafe fn lz4_init(
    msblk: *mut squashfs_sb_info,
    _buff: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let block_size = core::cmp::max((*msblk).block_size, SQUASHFS_METADATA_SIZE);
    let stream: *mut squashfs_lz4 = kzalloc_obj::<squashfs_lz4>();

    if stream.is_null() {
        ERROR!("Failed to initialise LZ4 decompressor\n");
        return ERR_PTR(-ENOMEM);
    }
    (*stream).input = vmalloc(block_size as usize);
    if (*stream).input.is_null() {
        kfree(stream as *mut core::ffi::c_void);
        ERROR!("Failed to initialise LZ4 decompressor\n");
        return ERR_PTR(-ENOMEM);
    }
    (*stream).output = vmalloc(block_size as usize);
    if (*stream).output.is_null() {
        vfree((*stream).input);
        kfree(stream as *mut core::ffi::c_void);
        ERROR!("Failed to initialise LZ4 decompressor\n");
        return ERR_PTR(-ENOMEM);
    }

    stream as *mut core::ffi::c_void
}

unsafe fn lz4_free(strm: *mut core::ffi::c_void) {
    let stream = strm as *mut squashfs_lz4;

    if !stream.is_null() {
        vfree((*stream).input);
        vfree((*stream).output);
    }
    kfree(stream as *mut core::ffi::c_void);
}

unsafe fn lz4_uncompress(
    _msblk: *mut squashfs_sb_info,
    strm: *mut core::ffi::c_void,
    bio: *mut bio,
    mut offset: i32,
    length: i32,
    output: *mut squashfs_page_actor,
) -> i32 {
    let mut iter_all: bvec_iter_all = core::mem::zeroed();
    let bvec = bvec_init_iter_all(&mut iter_all);
    let stream = strm as *mut squashfs_lz4;
    let mut buff = (*stream).input as *mut u8;
    let mut data: *mut u8;
    let mut bytes = length;

    while bio_next_segment(bio, &mut iter_all) {
        let avail = core::cmp::min(bytes, (*bvec).bv_len as i32 - offset);
        data = bvec_virt(bvec);
        core::ptr::copy_nonoverlapping(data.add(offset as usize), buff, avail as usize);
        buff = buff.add(avail as usize);
        bytes -= avail;
        offset = 0;
    }

    let res = LZ4_decompress_safe(
        (*stream).input,
        (*stream).output,
        length,
        (*output).length,
    );

    if res < 0 {
        return -EIO;
    }

    bytes = res;
    data = squashfs_first_page(output);
    buff = (*stream).output as *mut u8;
    while !data.is_null() {
        if bytes <= PAGE_SIZE {
            if !IS_ERR(data) {
                core::ptr::copy_nonoverlapping(data, buff, bytes as usize);
            }
            break;
        }
        if !IS_ERR(data) {
            core::ptr::copy_nonoverlapping(data, buff, PAGE_SIZE as usize);
        }
        buff = buff.add(PAGE_SIZE as usize);
        bytes -= PAGE_SIZE;
        data = squashfs_next_page(output);
    }
    squashfs_finish_page(output);

    res
}

const squashfs_lz4_comp_ops: squashfs_decompressor = squashfs_decompressor {
    init: Some(lz4_init),
    comp_opts: Some(lz4_comp_opts),
    free: Some(lz4_free),
    decompress: Some(lz4_uncompress),
    id: LZ4_COMPRESSION,
    name: b"lz4\0".as_ptr() as *const i8,
    alloc_buffer: 0,
    supported: 1,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
