// SPDX-License-Identifier: GPL-2.0-or-later
// Dependencies supplied by the kernel, zlib, and compress.h are external.

#[repr(C)]
struct z_erofs_deflate {
    next: *mut z_erofs_deflate,
    z: z_stream_s,
    bounce: [u8; PAGE_SIZE],
}

static mut z_erofs_deflate_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut z_erofs_deflate_nstrms: c_uint = 0;
static mut z_erofs_deflate_avail_strms: c_uint = 0;
static mut z_erofs_deflate_head: *mut z_erofs_deflate = core::ptr::null_mut();
static mut z_erofs_deflate_wq: wait_queue_head_t = DECLARE_WAIT_QUEUE_HEAD!();

// module_param_named(deflate_streams, z_erofs_deflate_nstrms, uint, 0444)

unsafe fn z_erofs_deflate_exit() {
    /* there should be no running fs instance */
    while z_erofs_deflate_avail_strms != 0 {
        let mut strm: *mut z_erofs_deflate;

        spin_lock(&raw mut z_erofs_deflate_lock);
        strm = z_erofs_deflate_head;
        if strm.is_null() {
            spin_unlock(&raw mut z_erofs_deflate_lock);
            continue;
        }
        z_erofs_deflate_head = core::ptr::null_mut();
        spin_unlock(&raw mut z_erofs_deflate_lock);

        while !strm.is_null() {
            let n = (*strm).next;

            vfree((*strm).z.workspace);
            kfree(strm);
            z_erofs_deflate_avail_strms -= 1;
            strm = n;
        }
    }
}

unsafe fn z_erofs_deflate_init() -> c_int {
    /* by default, use # of possible CPUs instead */
    if z_erofs_deflate_nstrms == 0 {
        z_erofs_deflate_nstrms = num_possible_cpus();
    }
    0
}

unsafe fn z_erofs_load_deflate_config(
    sb: *mut super_block,
    _dsb: *mut erofs_super_block,
    data: *mut core::ffi::c_void,
    size: c_int,
) -> c_int {
    let dfl = data as *mut z_erofs_deflate_cfgs;
    static mut deflate_resize_mutex: mutex = DEFINE_MUTEX!();
    static mut inited: bool = false;

    if dfl.is_null() || size < core::mem::size_of::<z_erofs_deflate_cfgs>() as c_int {
        erofs_err!(sb, "invalid deflate cfgs, size=%u", size);
        return -EINVAL;
    }

    if (*dfl).windowbits > MAX_WBITS {
        erofs_err!(sb, "unsupported windowbits %u", (*dfl).windowbits);
        return -EOPNOTSUPP;
    }
    mutex_lock(&raw mut deflate_resize_mutex);
    if !inited {
        while z_erofs_deflate_avail_strms < z_erofs_deflate_nstrms {
            let strm = kzalloc_obj::<z_erofs_deflate>();
            if strm.is_null() {
                mutex_unlock(&raw mut deflate_resize_mutex);
                z_erofs_deflate_exit();
                return -ENOMEM;
            }
            /* XXX: in-kernel zlib cannot customize windowbits */
            (*strm).z.workspace = vmalloc(zlib_inflate_workspacesize());
            if (*strm).z.workspace.is_null() {
                kfree(strm);
                mutex_unlock(&raw mut deflate_resize_mutex);
                z_erofs_deflate_exit();
                return -ENOMEM;
            }

            spin_lock(&raw mut z_erofs_deflate_lock);
            (*strm).next = z_erofs_deflate_head;
            z_erofs_deflate_head = strm;
            spin_unlock(&raw mut z_erofs_deflate_lock);
            z_erofs_deflate_avail_strms += 1;
        }
        inited = true;
    }
    mutex_unlock(&raw mut deflate_resize_mutex);
    0
}

unsafe fn __z_erofs_deflate_decompress(
    rq: *mut z_erofs_decompress_req,
    pgpl: *mut *mut page,
) -> *const c_char {
    let sb = (*rq).sb;
    let mut dctx = z_erofs_stream_dctx { rq, no: -1, ni: 0, ..core::mem::zeroed() };
    let mut strm: *mut z_erofs_deflate;
    let mut reason: *const c_char;
    let mut zerr: c_int;

    /* 1. get the exact DEFLATE compressed size */
    dctx.kin = kmap_local_page(*(*rq).input);
    reason = z_erofs_fixup_insize(rq, dctx.kin.add((*rq).pageofs_in),
        core::cmp::min((*rq).inputsize, (*sb).s_blocksize - (*rq).pageofs_in));
    if !reason.is_null() {
        kunmap_local(dctx.kin);
        return reason;
    }

    /* 2. get an available DEFLATE context */
    loop {
        spin_lock(&raw mut z_erofs_deflate_lock);
        strm = z_erofs_deflate_head;
        if !strm.is_null() {
            z_erofs_deflate_head = (*strm).next;
            spin_unlock(&raw mut z_erofs_deflate_lock);
            break;
        }
        spin_unlock(&raw mut z_erofs_deflate_lock);
        wait_event!(z_erofs_deflate_wq, READ_ONCE!(z_erofs_deflate_head));
    }

    /* 3. multi-call decompress */
    zerr = zlib_inflateInit2(&mut (*strm).z, -MAX_WBITS);
    if zerr != Z_OK {
        reason = ERR_PTR(-EINVAL);
        kunmap_local(dctx.kin);
        spin_lock(&raw mut z_erofs_deflate_lock);
        (*strm).next = z_erofs_deflate_head;
        z_erofs_deflate_head = strm;
        spin_unlock(&raw mut z_erofs_deflate_lock);
        wake_up(&raw mut z_erofs_deflate_wq);
        return reason;
    }

    (*rq).fillgaps = true;
    (*strm).z.avail_in = core::cmp::min((*rq).inputsize, PAGE_SIZE - (*rq).pageofs_in);
    (*rq).inputsize -= (*strm).z.avail_in;
    (*strm).z.next_in = dctx.kin.add((*rq).pageofs_in);
    (*strm).z.avail_out = 0;
    dctx.bounce = (*strm).bounce.as_mut_ptr();

    loop {
        dctx.avail_out = (*strm).z.avail_out;
        dctx.inbuf_sz = (*strm).z.avail_in;
        reason = z_erofs_stream_switch_bufs(&mut dctx, &mut (*strm).z.next_out,
            &mut (*strm).z.next_in, pgpl);
        if !reason.is_null() { break; }
        (*strm).z.avail_out = dctx.avail_out;
        (*strm).z.avail_in = dctx.inbuf_sz;
        zerr = zlib_inflate(&mut (*strm).z, Z_SYNC_FLUSH);
        if zerr != Z_OK || (*rq).outputsize + (*strm).z.avail_out == 0 {
            if zerr == Z_OK && (*rq).partial_decoding { break; }
            if zerr == Z_STREAM_END && (*rq).outputsize == 0 { break; }
            reason = if zerr == Z_DATA_ERROR { c_str!("corrupted compressed data") } else { c_str!("unexpected end of stream") };
            break;
        }
    }
    if zlib_inflateEnd(&mut (*strm).z) != Z_OK && reason.is_null() { reason = ERR_PTR(-EIO); }
    if !dctx.kout.is_null() { kunmap_local(dctx.kout); }
    kunmap_local(dctx.kin);
    spin_lock(&raw mut z_erofs_deflate_lock);
    (*strm).next = z_erofs_deflate_head;
    z_erofs_deflate_head = strm;
    spin_unlock(&raw mut z_erofs_deflate_lock);
    wake_up(&raw mut z_erofs_deflate_wq);
    reason
}

unsafe fn z_erofs_deflate_decompress(rq: *mut z_erofs_decompress_req, pgpl: *mut *mut page) -> *const c_char {
    // CONFIG_EROFS_FS_ZIP_ACCEL conditional code is supplied by the build configuration.
    __z_erofs_deflate_decompress(rq, pgpl)
}

const z_erofs_deflate_decomp: z_erofs_decompressor = z_erofs_decompressor {
    config: Some(z_erofs_load_deflate_config),
    decompress: Some(z_erofs_deflate_decompress),
    init: Some(z_erofs_deflate_init),
    exit: Some(z_erofs_deflate_exit),
    name: c_str!("deflate"),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
