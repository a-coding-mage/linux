// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2021 Google LLC.
 */

// Kernel headers and "internal.h" are supplied by the surrounding translation unit.

unsafe fn module_extend_max_pages(info: *mut load_info, extent: c_uint) -> c_int {
    let new_max = (*info).max_pages.wrapping_add(extent);
    let new_pages = kvrealloc(
        (*info).pages as *mut c_void,
        size_mul(new_max as usize, core::mem::size_of::<*mut page>()),
        GFP_KERNEL,
    ) as *mut *mut page;
    if new_pages.is_null() {
        return -ENOMEM;
    }
    (*info).pages = new_pages;
    (*info).max_pages = new_max;
    0
}

unsafe fn module_get_next_page(info: *mut load_info) -> *mut page {
    if (*info).max_pages == (*info).used_pages {
        let error = module_extend_max_pages(info, (*info).used_pages);
        if error != 0 {
            return ERR_PTR(error as isize) as *mut page;
        }
    }
    let p = alloc_page(GFP_KERNEL | __GFP_HIGHMEM);
    if p.is_null() {
        return ERR_PTR(-ENOMEM as isize) as *mut page;
    }
    *(*info).pages.add((*info).used_pages as usize) = p;
    (*info).used_pages = (*info).used_pages.wrapping_add(1);
    p
}

#[cfg(CONFIG_MODULE_COMPRESS_GZIP)]
const MODULE_COMPRESSION: &str = "gzip";
#[cfg(CONFIG_MODULE_COMPRESS_XZ)]
const MODULE_COMPRESSION: &str = "xz";
#[cfg(CONFIG_MODULE_COMPRESS_ZSTD)]
const MODULE_COMPRESSION: &str = "zstd";

#[cfg(CONFIG_MODULE_COMPRESS_GZIP)]
const MODULE_DECOMPRESS_FN: unsafe fn(*mut load_info, *const c_void, usize) -> isize = module_gzip_decompress;
#[cfg(CONFIG_MODULE_COMPRESS_XZ)]
const MODULE_DECOMPRESS_FN: unsafe fn(*mut load_info, *const c_void, usize) -> isize = module_xz_decompress;
#[cfg(CONFIG_MODULE_COMPRESS_ZSTD)]
const MODULE_DECOMPRESS_FN: unsafe fn(*mut load_info, *const c_void, usize) -> isize = module_zstd_decompress;

#[cfg(CONFIG_MODULE_COMPRESS_GZIP)]
unsafe fn module_gzip_header_len(buf: *const u8, size: usize) -> usize {
    let signature = [0x1f_u8, 0x8b, 0x08];
    let mut len = 10usize;
    if size < len || core::slice::from_raw_parts(buf, 3) != signature {
        return 0;
    }
    if *buf.add(3) & 0x08 != 0 {
        loop {
            if len == size { return 0; }
            len += 1;
            if *buf.add(len - 1) == 0 { break; }
        }
    }
    len
}

#[cfg(CONFIG_MODULE_COMPRESS_GZIP)]
unsafe fn module_gzip_decompress(info: *mut load_info, buf: *const c_void, size: usize) -> isize {
    let mut s: z_stream_s = core::mem::zeroed();
    let mut new_size = 0usize;
    let hdr_len = module_gzip_header_len(buf as *const u8, size);
    if hdr_len == 0 { pr_err!("not a gzip compressed module\n"); return -EINVAL as isize; }
    s.next_in = (buf as *const u8).add(hdr_len);
    s.avail_in = (size - hdr_len) as u32;
    s.workspace = kvmalloc(zlib_inflate_workspacesize(), GFP_KERNEL);
    if s.workspace.is_null() { return -ENOMEM as isize; }
    let rc = zlib_inflateInit2(&mut s, -MAX_WBITS);
    if rc != Z_OK { pr_err!("failed to initialize decompressor: %d\n", rc); kvfree(s.workspace); return -EINVAL as isize; }
    let mut rc = rc;
    loop {
        let p = module_get_next_page(info);
        if IS_ERR(p) { let r = PTR_ERR(p); zlib_inflateEnd(&mut s); kvfree(s.workspace); return r; }
        s.next_out = kmap_local_page(p);
        s.avail_out = PAGE_SIZE;
        rc = zlib_inflate(&mut s, 0);
        kunmap_local(s.next_out);
        new_size += PAGE_SIZE as usize - s.avail_out as usize;
        if rc != Z_OK { break; }
    }
    zlib_inflateEnd(&mut s);
    kvfree(s.workspace);
    if rc != Z_STREAM_END { pr_err!("decompression failed with status %d\n", rc); return -EINVAL as isize; }
    new_size as isize
}

#[cfg(CONFIG_MODULE_COMPRESS_XZ)]
unsafe fn module_xz_decompress(info: *mut load_info, buf: *const c_void, size: usize) -> isize {
    let signature = [0xfd_u8, b'7', b'z', b'X', b'Z', 0];
    if size < signature.len() || core::slice::from_raw_parts(buf as *const u8, signature.len()) != signature { pr_err!("not an xz compressed module\n"); return -EINVAL as isize; }
    let dec = xz_dec_init(XZ_DYNALLOC, u32::MAX);
    if dec.is_null() { return -ENOMEM as isize; }
    let mut b: xz_buf = core::mem::zeroed(); b.in_size = size; b.in = buf; b.in_pos = 0;
    let mut total = 0usize; let mut ret;
    loop { let p = module_get_next_page(info); if IS_ERR(p) { ret = PTR_ERR(p); break; } b.out = kmap_local_page(p); b.out_pos = 0; b.out_size = PAGE_SIZE as usize; ret = xz_dec_run(dec, &mut b); kunmap_local(b.out); total += b.out_pos; if b.out_pos != PAGE_SIZE as usize || ret != XZ_OK { break; } }
    xz_dec_end(dec); if ret != XZ_STREAM_END { pr_err!("decompression failed with status %d\n", ret); return -EINVAL as isize; } total as isize
}

#[cfg(CONFIG_MODULE_COMPRESS_ZSTD)]
unsafe fn module_zstd_decompress(info: *mut load_info, buf: *const c_void, size: usize) -> isize {
    // Direct translation of the zstd branch; types and functions are provided by zstd.h.
    let signature = [0x28_u8, 0xb5, 0x2f, 0xfd];
    if size < signature.len() || core::slice::from_raw_parts(buf as *const u8, signature.len()) != signature { pr_err!("not a zstd compressed module\n"); return -EINVAL as isize; }
    let mut header: zstd_frame_header = core::mem::zeroed();
    let mut input: ZSTD_inBuffer = ZSTD_inBuffer { src: buf, pos: 0, size };
    let ret = zstd_get_frame_header(&mut header, input.src, input.size);
    if ret != 0 || header.windowSize > (1usize << ZSTD_WINDOWLOG_MAX) { return -EINVAL as isize; }
    let wksp_size = zstd_dstream_workspace_bound(header.windowSize); let wksp = kvmalloc(wksp_size, GFP_KERNEL); if wksp.is_null() { return -ENOMEM as isize; }
    let stream = zstd_init_dstream(header.windowSize, wksp, wksp_size); if stream.is_null() { kvfree(wksp); return -ENOMEM as isize; }
    let mut total = 0usize; loop { let p = module_get_next_page(info); if IS_ERR(p) { kvfree(wksp); return PTR_ERR(p); } let mut out = ZSTD_outBuffer { dst: kmap_local_page(p), pos: 0, size: PAGE_SIZE as usize }; let r = zstd_decompress_stream(stream, &mut out, &mut input); kunmap_local(out.dst); if zstd_get_error_code(r) != 0 { kvfree(wksp); return -EINVAL as isize; } total += out.pos; if out.pos != PAGE_SIZE as usize || r == 0 { break; } }
    kvfree(wksp); total as isize
}

pub unsafe fn module_decompress(info: *mut load_info, buf: *const c_void, size: usize) -> c_int {
    #[cfg(CONFIG_MODULE_STATS)] { (*info).compressed_len = size; }
    let n_pages = DIV_ROUND_UP(size, PAGE_SIZE as usize).wrapping_mul(2) as c_uint;
    let error = module_extend_max_pages(info, n_pages); if error != 0 { return error; }
    let data_size = MODULE_DECOMPRESS_FN(info, buf, size); if data_size < 0 { module_decompress_cleanup(info); return data_size as c_int; }
    (*info).hdr = vmap((*info).pages, (*info).used_pages, VM_MAP, PAGE_KERNEL); if (*info).hdr.is_null() { module_decompress_cleanup(info); return -ENOMEM; }
    (*info).len = data_size as usize; 0
}

pub unsafe fn module_decompress_cleanup(info: *mut load_info) {
    if !(*info).hdr.is_null() { vunmap((*info).hdr); }
    for i in 0..(*info).used_pages { __free_page(*(*info).pages.add(i as usize)); }
    kvfree((*info).pages as *mut c_void); (*info).pages = core::ptr::null_mut(); (*info).max_pages = 0; (*info).used_pages = 0;
}

#[cfg(CONFIG_SYSFS)]
unsafe fn compression_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> isize { sysfs_emit!(buf, "{}\n", MODULE_COMPRESSION) }

#[cfg(CONFIG_SYSFS)]
static mut MODULE_COMPRESSION_ATTR: kobj_attribute = __ATTR_RO!(compression);

#[cfg(CONFIG_SYSFS)]
unsafe fn module_decompress_sysfs_init() -> c_int {
    let error = sysfs_create_file(&mut (*module_kset).kobj, &mut MODULE_COMPRESSION_ATTR.attr);
    if error != 0 { pr_warn!("Failed to create 'compression' attribute"); }
    0
}

// Equivalent of late_initcall(module_decompress_sysfs_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
