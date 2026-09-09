/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2007 Nokia Corporation. All rights reserved.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by Richard Purdie <rpurdie@openedhand.com>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Dependencies supplied by the surrounding kernel/JFFS2 environment:
// linux/kernel.h, linux/sched.h, linux/vmalloc.h, linux/init.h,
// linux/lzo.h, and "compr.h".

use core::ffi::c_void;

extern "C" {
    fn vmalloc(size: usize) -> *mut c_void;
    fn vfree(addr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn lzo1x_worst_compress(size: usize) -> usize;
    fn lzo1x_1_compress(
        src: *const u8,
        src_len: usize,
        dst: *mut c_void,
        dst_len: *mut usize,
        wrkmem: *mut c_void,
    ) -> i32;
    fn lzo1x_decompress_safe(
        src: *const u8,
        src_len: usize,
        dst: *mut u8,
        dst_len: *mut usize,
    ) -> i32;
    fn jffs2_register_compressor(comp: *mut jffs2_compressor) -> i32;
    fn jffs2_unregister_compressor(comp: *mut jffs2_compressor);
    static mut deflate_mutex: c_void;
}

// These constants are provided by the kernel headers.
extern "C" {
    static LZO1X_MEM_COMPRESS: usize;
    static LZO_E_OK: i32;
    static PAGE_SIZE: usize;
    static JFFS2_LZO_PRIORITY: i32;
    static JFFS2_COMPR_LZO: i32;
}

#[repr(C)]
struct jffs2_compressor {
    priority: i32,
    name: *const u8,
    compr: i32,
    compress: Option<unsafe extern "C" fn(*mut u8, *mut u8, *mut u32, *mut u32) -> i32>,
    decompress: Option<unsafe extern "C" fn(*mut u8, *mut u8, u32, u32) -> i32>,
    disabled: i32,
}

static mut lzo_mem: *mut c_void = core::ptr::null_mut();
static mut lzo_compress_buf: *mut c_void = core::ptr::null_mut();

unsafe fn free_workspace() {
    vfree(lzo_mem);
    vfree(lzo_compress_buf);
}

unsafe fn alloc_workspace() -> i32 {
    lzo_mem = vmalloc(LZO1X_MEM_COMPRESS);
    lzo_compress_buf = vmalloc(lzo1x_worst_compress(PAGE_SIZE));

    if lzo_mem.is_null() || lzo_compress_buf.is_null() {
        free_workspace();
        return -12; // -ENOMEM
    }

    0
}

unsafe extern "C" fn jffs2_lzo_compress(
    data_in: *mut u8,
    cpage_out: *mut u8,
    sourcelen: *mut u32,
    dstlen: *mut u32,
) -> i32 {
    let mut compress_size: usize = 0;
    let ret: i32;

    mutex_lock(&mut deflate_mutex as *mut c_void);
    ret = lzo1x_1_compress(
        data_in,
        *sourcelen as usize,
        lzo_compress_buf,
        &mut compress_size,
        lzo_mem,
    );
    if ret != LZO_E_OK {
        mutex_unlock(&mut deflate_mutex as *mut c_void);
        return -1;
    }

    if compress_size > *dstlen as usize {
        mutex_unlock(&mut deflate_mutex as *mut c_void);
        return -1;
    }

    memcpy(cpage_out as *mut c_void, lzo_compress_buf, compress_size);
    mutex_unlock(&mut deflate_mutex as *mut c_void);

    *dstlen = compress_size as u32;
    0
}

unsafe extern "C" fn jffs2_lzo_decompress(
    data_in: *mut u8,
    cpage_out: *mut u8,
    srclen: u32,
    destlen: u32,
) -> i32 {
    let mut dl = destlen as usize;
    let ret = lzo1x_decompress_safe(data_in, srclen as usize, cpage_out, &mut dl);

    if ret != LZO_E_OK || dl != destlen as usize {
        return -1;
    }

    0
}

static mut jffs2_lzo_comp: jffs2_compressor = jffs2_compressor {
    priority: 0,
    name: b"lzo\0".as_ptr(),
    compr: 0,
    compress: Some(jffs2_lzo_compress),
    decompress: Some(jffs2_lzo_decompress),
    disabled: 0,
};

#[no_mangle]
pub unsafe extern "C" fn jffs2_lzo_init() -> i32 {
    let mut ret = alloc_workspace();
    if ret < 0 {
        return ret;
    }

    ret = jffs2_register_compressor(&mut jffs2_lzo_comp);
    if ret != 0 {
        free_workspace();
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn jffs2_lzo_exit() {
    jffs2_unregister_compressor(&mut jffs2_lzo_comp);
    free_workspace();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
