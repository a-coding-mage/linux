// SPDX-License-Identifier: GPL-2.0-or-later
// C dependencies: <linux/xz.h>, "compress.h"

#[repr(C)]
pub struct z_erofs_lzma {
    pub next: *mut z_erofs_lzma,
    pub state: *mut xz_dec_microlzma,
    pub bounce: [u8; PAGE_SIZE],
}

/* considering the LZMA performance, no need to use a lockless list for now */
static mut Z_EROFS_LZMA_LOCK: DEFINE_SPINLOCK_TYPE = DEFINE_SPINLOCK_INIT;
static mut Z_EROFS_LZMA_MAX_DICTSIZE: u32 = 0;
static mut Z_EROFS_LZMA_NSTRMS: u32 = 0;
static mut Z_EROFS_LZMA_AVAIL_STRMS: u32 = 0;
static mut Z_EROFS_LZMA_HEAD: *mut z_erofs_lzma = core::ptr::null_mut();
static mut Z_EROFS_LZMA_WQ: DECLARE_WAIT_QUEUE_HEAD_TYPE = DECLARE_WAIT_QUEUE_HEAD_INIT;

// module_param_named(lzma_streams, z_erofs_lzma_nstrms, uint, 0444);

unsafe fn z_erofs_lzma_exit() {
    /* there should be no running fs instance */
    while Z_EROFS_LZMA_AVAIL_STRMS != 0 {
        let strm: *mut z_erofs_lzma;

        spin_lock(&raw mut Z_EROFS_LZMA_LOCK);
        strm = Z_EROFS_LZMA_HEAD;
        if strm.is_null() {
            spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
            DBG_BUGON(1);
            return;
        }
        Z_EROFS_LZMA_HEAD = core::ptr::null_mut();
        spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);

        let mut current = strm;
        while !current.is_null() {
            let n = (*current).next;

            if !(*current).state.is_null() {
                xz_dec_microlzma_end((*current).state);
            }
            kfree(current);
            Z_EROFS_LZMA_AVAIL_STRMS -= 1;
            current = n;
        }
    }
}

unsafe fn z_erofs_lzma_init() -> i32 {
    let mut i: u32;

    /* by default, use # of possible CPUs instead */
    if Z_EROFS_LZMA_NSTRMS == 0 {
        Z_EROFS_LZMA_NSTRMS = min_t(
            num_possible_cpus(),
            CONFIG_EROFS_FS_ZIP_LZMA_DEFAULT_MAX_STREAMS,
        );
    }

    i = 0;
    while i < Z_EROFS_LZMA_NSTRMS {
        let strm: *mut z_erofs_lzma = kzalloc_obj();

        if strm.is_null() {
            z_erofs_lzma_exit();
            return -ENOMEM;
        }
        spin_lock(&raw mut Z_EROFS_LZMA_LOCK);
        (*strm).next = Z_EROFS_LZMA_HEAD;
        Z_EROFS_LZMA_HEAD = strm;
        spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
        Z_EROFS_LZMA_AVAIL_STRMS += 1;
        i += 1;
    }
    0
}

unsafe fn z_erofs_load_lzma_config(
    sb: *mut super_block,
    _dsb: *mut erofs_super_block,
    data: *mut core::ffi::c_void,
    size: i32,
) -> i32 {
    static mut LZMA_RESIZE_MUTEX: DEFINE_MUTEX_TYPE = DEFINE_MUTEX_INIT;
    let lzma = data as *mut z_erofs_lzma_cfgs;
    let mut dict_size: u32;
    let mut i: u32;
    let mut strm: *mut z_erofs_lzma;
    let mut head: *mut z_erofs_lzma = core::ptr::null_mut();
    let mut err: i32;

    if lzma.is_null() || (size as usize) < core::mem::size_of::<z_erofs_lzma_cfgs>() {
        erofs_err(sb, "invalid lzma cfgs, size=%u", size);
        return -EINVAL;
    }
    if (*lzma).format != 0 {
        erofs_err(sb, "unidentified lzma format %x, please check kernel version", le16_to_cpu((*lzma).format));
        return -EINVAL;
    }
    dict_size = le32_to_cpu((*lzma).dict_size);
    if dict_size > Z_EROFS_LZMA_MAX_DICT_SIZE || dict_size < 4096 {
        erofs_err(sb, "unsupported lzma dictionary size %u", dict_size);
        return -EINVAL;
    }

    /* in case 2 z_erofs_load_lzma_config() race to avoid deadlock */
    mutex_lock(&raw mut LZMA_RESIZE_MUTEX);

    if Z_EROFS_LZMA_MAX_DICTSIZE >= dict_size {
        mutex_unlock(&raw mut LZMA_RESIZE_MUTEX);
        return 0;
    }

    /* 1. collect/isolate all streams for the following check */
    i = 0;
    while i < Z_EROFS_LZMA_AVAIL_STRMS {
        let mut last: *mut z_erofs_lzma;

        'again: loop {
            spin_lock(&raw mut Z_EROFS_LZMA_LOCK);
            strm = Z_EROFS_LZMA_HEAD;
            if strm.is_null() {
                spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
                wait_event(&raw mut Z_EROFS_LZMA_WQ, READ_ONCE(Z_EROFS_LZMA_HEAD));
                continue 'again;
            }
            Z_EROFS_LZMA_HEAD = core::ptr::null_mut();
            spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
            break;
        }

        last = strm;
        while !(*last).next.is_null() {
            last = (*last).next;
            i += 1;
        }
        (*last).next = head;
        head = strm;
        i += 1;
    }

    err = 0;
    /* 2. walk each isolated stream and grow max dict_size if needed */
    strm = head;
    while !strm.is_null() {
        if !(*strm).state.is_null() {
            xz_dec_microlzma_end((*strm).state);
        }
        (*strm).state = xz_dec_microlzma_alloc(XZ_PREALLOC, dict_size);
        if (*strm).state.is_null() {
            err = -ENOMEM;
        }
        strm = (*strm).next;
    }

    /* 3. push back all to the global list and update max dict_size */
    spin_lock(&raw mut Z_EROFS_LZMA_LOCK);
    DBG_BUGON(!Z_EROFS_LZMA_HEAD.is_null());
    Z_EROFS_LZMA_HEAD = head;
    spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
    wake_up_all(&raw mut Z_EROFS_LZMA_WQ);

    Z_EROFS_LZMA_MAX_DICTSIZE = dict_size;
    mutex_unlock(&raw mut LZMA_RESIZE_MUTEX);
    err
}

unsafe fn z_erofs_lzma_decompress(
    rq: *mut z_erofs_decompress_req,
    pgpl: *mut *mut page,
) -> *const core::ffi::c_char {
    let sb = (*rq).sb;
    let mut dctx = z_erofs_stream_dctx { rq, no: -1, ni: 0, ..core::mem::zeroed() };
    let mut buf: xz_buf = core::mem::zeroed();
    let mut strm: *mut z_erofs_lzma;
    let mut xz_err: xz_ret;
    let mut reason: *const core::ffi::c_char;

    /* 1. get the exact LZMA compressed size */
    dctx.kin = kmap_local_page(*(*rq).in_);
    reason = z_erofs_fixup_insize(rq, dctx.kin.add((*rq).pageofs_in), min((*rq).inputsize, (*sb).s_blocksize - (*rq).pageofs_in));
    if !reason.is_null() {
        kunmap_local(dctx.kin);
        return reason;
    }

    /* 2. get an available lzma context */
    'again: loop {
        spin_lock(&raw mut Z_EROFS_LZMA_LOCK);
        strm = Z_EROFS_LZMA_HEAD;
        if strm.is_null() {
            spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
            wait_event(&raw mut Z_EROFS_LZMA_WQ, READ_ONCE(Z_EROFS_LZMA_HEAD));
            continue 'again;
        }
        Z_EROFS_LZMA_HEAD = (*strm).next;
        spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
        break;
    }

    /* 3. multi-call decompress */
    xz_dec_microlzma_reset((*strm).state, (*rq).inputsize, (*rq).outputsize, !(*rq).partial_decoding);
    buf.in_size = min((*rq).inputsize, PAGE_SIZE - (*rq).pageofs_in);
    (*rq).inputsize -= buf.in_size;
    buf.in = dctx.kin.add((*rq).pageofs_in);
    dctx.bounce = (*strm).bounce.as_mut_ptr();
    loop {
        dctx.avail_out = buf.out_size - buf.out_pos;
        dctx.inbuf_sz = buf.in_size;
        dctx.inbuf_pos = buf.in_pos;
        reason = z_erofs_stream_switch_bufs(&mut dctx, &mut buf.out, &mut buf.in, pgpl);
        if !reason.is_null() { break; }
        if buf.out_size == buf.out_pos {
            buf.out_size = dctx.avail_out;
            buf.out_pos = 0;
        }
        buf.in_size = dctx.inbuf_sz;
        buf.in_pos = dctx.inbuf_pos;
        xz_err = xz_dec_microlzma_run((*strm).state, &mut buf);
        DBG_BUGON(buf.out_pos > buf.out_size);
        DBG_BUGON(buf.in_pos > buf.in_size);
        if xz_err != XZ_OK {
            if xz_err == XZ_STREAM_END && (*rq).outputsize == 0 { break; }
            reason = if xz_err == XZ_DATA_ERROR { c"corrupted compressed data".as_ptr() } else { c"unexpected end of stream".as_ptr() };
            break;
        }
    }

    if !dctx.kout.is_null() { kunmap_local(dctx.kout); }
    kunmap_local(dctx.kin);
    /* 4. push back LZMA stream context to the global list */
    spin_lock(&raw mut Z_EROFS_LZMA_LOCK);
    (*strm).next = Z_EROFS_LZMA_HEAD;
    Z_EROFS_LZMA_HEAD = strm;
    spin_unlock(&raw mut Z_EROFS_LZMA_LOCK);
    wake_up(&raw mut Z_EROFS_LZMA_WQ);
    reason
}

#[no_mangle]
pub static z_erofs_lzma_decomp: z_erofs_decompressor = z_erofs_decompressor {
    config: Some(z_erofs_load_lzma_config),
    decompress: Some(z_erofs_lzma_decompress),
    init: Some(z_erofs_lzma_init),
    exit: Some(z_erofs_lzma_exit),
    name: c"lzma".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
