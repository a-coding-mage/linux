/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Copyright © 2001-2007 Red Hat, Inc.
 * Copyright © 2004-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * Created by David Woodhouse <dwmw2@infradead.org>
 *
 * For licensing information, see the file 'LICENCE' in this directory.
 */

// Kernel/eCos-only implementation; the removed userspace configuration is intentionally omitted.
// Dependencies supplied by the surrounding translation unit: kernel, zlib, nodelist, and compr APIs.

/* Plan: call deflate() with avail_in == *sourcelen, avail_out == *dstlen - 12
 * and flush == Z_FINISH. If it doesn't manage to finish, call it again with
 * avail_in == 0 and avail_out set to the remaining 12 bytes for cleanup.
 */
const STREAM_END_SPACE: u32 = 12;

static mut deflate_mutex: Mutex = Mutex::new();
static mut inflate_mutex: Mutex = Mutex::new();
static mut inf_strm: z_stream = z_stream::default();
static mut def_strm: z_stream = z_stream::default();

#[cfg(feature = "kernel")]
unsafe fn alloc_workspaces() -> i32 {
    def_strm.workspace = vmalloc(zlib_deflate_workspacesize(MAX_WBITS, MAX_MEM_LEVEL));
    if def_strm.workspace.is_null() {
        return -ENOMEM;
    }

    jffs2_dbg(1, "Allocated %d bytes for deflate workspace\n", zlib_deflate_workspacesize(MAX_WBITS, MAX_MEM_LEVEL));
    inf_strm.workspace = vmalloc(zlib_inflate_workspacesize());
    if inf_strm.workspace.is_null() {
        vfree(def_strm.workspace);
        return -ENOMEM;
    }
    jffs2_dbg(1, "Allocated %d bytes for inflate workspace\n", zlib_inflate_workspacesize());
    0
}

#[cfg(not(feature = "kernel"))]
unsafe fn alloc_workspaces() -> i32 { 0 }

#[cfg(feature = "kernel")]
unsafe fn free_workspaces() {
    vfree(def_strm.workspace);
    vfree(inf_strm.workspace);
}

#[cfg(not(feature = "kernel"))]
unsafe fn free_workspaces() {}

unsafe fn jffs2_zlib_compress(
    data_in: *mut u8, cpage_out: *mut u8,
    sourcelen: *mut u32, dstlen: *mut u32,
) -> i32 {
    let mut ret: i32;
    if *dstlen <= STREAM_END_SPACE { return -1; }
    mutex_lock(&mut deflate_mutex);
    if zlib_deflateInit(&mut def_strm, 3) != Z_OK {
        pr_warn("deflateInit failed\n");
        mutex_unlock(&mut deflate_mutex);
        return -1;
    }
    def_strm.next_in = data_in;
    def_strm.total_in = 0;
    def_strm.next_out = cpage_out;
    def_strm.total_out = 0;
    while def_strm.total_out < (*dstlen - STREAM_END_SPACE) && def_strm.total_in < *sourcelen {
        def_strm.avail_out = *dstlen - (def_strm.total_out + STREAM_END_SPACE);
        def_strm.avail_in = core::cmp::min(*sourcelen - def_strm.total_in, def_strm.avail_out);
        jffs2_dbg(1, "calling deflate with avail_in %ld, avail_out %ld\n", def_strm.avail_in, def_strm.avail_out);
        ret = zlib_deflate(&mut def_strm, Z_PARTIAL_FLUSH);
        jffs2_dbg(1, "deflate returned with avail_in %ld, avail_out %ld, total_in %ld, total_out %ld\n", def_strm.avail_in, def_strm.avail_out, def_strm.total_in, def_strm.total_out);
        if ret != Z_OK {
            jffs2_dbg(1, "deflate in loop returned %d\n", ret);
            zlib_deflateEnd(&mut def_strm);
            mutex_unlock(&mut deflate_mutex);
            return -1;
        }
    }
    def_strm.avail_out += STREAM_END_SPACE;
    def_strm.avail_in = 0;
    ret = zlib_deflate(&mut def_strm, Z_FINISH);
    zlib_deflateEnd(&mut def_strm);
    if ret != Z_STREAM_END { jffs2_dbg(1, "final deflate returned %d\n", ret); ret = -1; mutex_unlock(&mut deflate_mutex); return ret; }
    if def_strm.total_out >= def_strm.total_in { jffs2_dbg(1, "zlib compressed %ld bytes into %ld; failing\n", def_strm.total_in, def_strm.total_out); ret = -1; mutex_unlock(&mut deflate_mutex); return ret; }
    jffs2_dbg(1, "zlib compressed %ld bytes into %ld\n", def_strm.total_in, def_strm.total_out);
    *dstlen = def_strm.total_out;
    *sourcelen = def_strm.total_in;
    mutex_unlock(&mut deflate_mutex);
    0
}

unsafe fn jffs2_zlib_decompress(data_in: *mut u8, cpage_out: *mut u8, srclen: u32, destlen: u32) -> i32 {
    let mut ret: i32;
    let mut wbits = MAX_WBITS;
    mutex_lock(&mut inflate_mutex);
    inf_strm.next_in = data_in; inf_strm.avail_in = srclen; inf_strm.total_in = 0;
    inf_strm.next_out = cpage_out; inf_strm.avail_out = destlen; inf_strm.total_out = 0;
    if srclen > 2 && (*data_in.add(1) & PRESET_DICT) == 0 && (*data_in & 0x0f) == Z_DEFLATED && ((((*data_in as i32) << 8) + *data_in.add(1) as i32) % 31) == 0 {
        jffs2_dbg(2, "inflate skipping adler32\n");
        wbits = -((*data_in >> 4) as i32 + 8);
        inf_strm.next_in = inf_strm.next_in.add(2); inf_strm.avail_in -= 2;
    } else { jffs2_dbg(1, "inflate not skipping adler32\n"); }
    if zlib_inflateInit2(&mut inf_strm, wbits) != Z_OK { pr_warn("inflateInit failed\n"); mutex_unlock(&mut inflate_mutex); return 1; }
    while { ret = zlib_inflate(&mut inf_strm, Z_FINISH); ret == Z_OK } {}
    if ret != Z_STREAM_END { pr_notice("inflate returned %d\n", ret); }
    zlib_inflateEnd(&mut inf_strm); mutex_unlock(&mut inflate_mutex); 0
}

static mut jffs2_zlib_comp: jffs2_compressor = jffs2_compressor {
    priority: JFFS2_ZLIB_PRIORITY, name: "zlib", compr: JFFS2_COMPR_ZLIB,
    compress: Some(jffs2_zlib_compress), decompress: Some(jffs2_zlib_decompress),
    disabled: if cfg!(JFFS2_ZLIB_DISABLED) { 1 } else { 0 },
};

unsafe fn jffs2_zlib_init() -> i32 {
    let ret = alloc_workspaces();
    if ret != 0 { return ret; }
    let ret = jffs2_register_compressor(&mut jffs2_zlib_comp);
    if ret != 0 { free_workspaces(); }
    ret
}

unsafe fn jffs2_zlib_exit() {
    jffs2_unregister_compressor(&mut jffs2_zlib_comp);
    free_workspaces();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
