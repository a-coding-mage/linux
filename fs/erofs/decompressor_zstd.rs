// SPDX-License-Identifier: GPL-2.0-or-later
// Dependencies supplied by the surrounding kernel/Rust translation.

#[repr(C)]
pub struct z_erofs_zstd {
    pub next: *mut z_erofs_zstd,
    pub bounce: [u8; PAGE_SIZE],
    pub wksp: *mut core::ffi::c_void,
    pub wkspsz: u32,
}

static mut Z_EROFS_ZSTD_LOCK: SpinLock = SpinLock::new();
static mut z_erofs_zstd_max_dictsize: u32 = 0;
static mut z_erofs_zstd_nstrms: u32 = 0;
static mut z_erofs_zstd_avail_strms: u32 = 0;
static mut z_erofs_zstd_head: *mut z_erofs_zstd = core::ptr::null_mut();
static mut Z_EROFS_ZSTD_WQ: WaitQueueHead = WaitQueueHead::new();

// module_param_named(zstd_streams, z_erofs_zstd_nstrms, uint, 0444)

unsafe fn z_erofs_isolate_strms(all: bool) -> *mut z_erofs_zstd {
    loop {
        spin_lock(&mut Z_EROFS_ZSTD_LOCK);
        let strm = z_erofs_zstd_head;
        if strm.is_null() {
            spin_unlock(&mut Z_EROFS_ZSTD_LOCK);
            wait_event(&mut Z_EROFS_ZSTD_WQ, core::ptr::read_volatile(&z_erofs_zstd_head));
            continue;
        }
        z_erofs_zstd_head = if all { core::ptr::null_mut() } else { (*strm).next };
        spin_unlock(&mut Z_EROFS_ZSTD_LOCK);
        return strm;
    }
}

unsafe fn z_erofs_zstd_exit() {
    while z_erofs_zstd_avail_strms != 0 {
        let mut strm: *mut z_erofs_zstd;
        let mut n: *mut z_erofs_zstd;
        strm = z_erofs_isolate_strms(true);
        while !strm.is_null() {
            n = (*strm).next;
            kvfree((*strm).wksp);
            kfree(strm);
            z_erofs_zstd_avail_strms -= 1;
            strm = n;
        }
    }
}

unsafe fn z_erofs_zstd_init() -> i32 {
    // by default, use # of possible CPUs instead
    if z_erofs_zstd_nstrms == 0 {
        z_erofs_zstd_nstrms = num_possible_cpus();
    }
    while z_erofs_zstd_avail_strms < z_erofs_zstd_nstrms {
        let strm = kzalloc_obj::<z_erofs_zstd>();
        if strm.is_null() {
            z_erofs_zstd_exit();
            return -ENOMEM;
        }
        spin_lock(&mut Z_EROFS_ZSTD_LOCK);
        (*strm).next = z_erofs_zstd_head;
        z_erofs_zstd_head = strm;
        spin_unlock(&mut Z_EROFS_ZSTD_LOCK);
        z_erofs_zstd_avail_strms += 1;
    }
    0
}

unsafe fn z_erofs_load_zstd_config(
    sb: *mut super_block,
    _dsb: *mut erofs_super_block,
    data: *mut core::ffi::c_void,
    size: i32,
) -> i32 {
    static mut zstd_resize_mutex: Mutex = Mutex::new();
    let zstd = data as *mut z_erofs_zstd_cfgs;
    let mut dict_size: u32;
    let mut wkspsz: u32;
    let mut strm: *mut z_erofs_zstd;
    let mut head: *mut z_erofs_zstd = core::ptr::null_mut();
    let mut wksp: *mut core::ffi::c_void;

    if zstd.is_null() || size < core::mem::size_of::<z_erofs_zstd_cfgs>() as i32 || (*zstd).format != 0 {
        erofs_err(sb, "unsupported zstd format, size={}");
        return -EINVAL;
    }
    if (*zstd).windowlog > ilog2(Z_EROFS_ZSTD_MAX_DICT_SIZE) - 10 {
        erofs_err(sb, "unsupported zstd window log {}", (*zstd).windowlog);
        return -EINVAL;
    }
    dict_size = 1u32 << ((*zstd).windowlog + 10);

    // in case 2 z_erofs_load_zstd_config() race to avoid deadlock
    mutex_lock(&mut zstd_resize_mutex);
    if z_erofs_zstd_max_dictsize >= dict_size {
        mutex_unlock(&mut zstd_resize_mutex);
        return 0;
    }

    // 1. collect/isolate all streams for the following check
    while z_erofs_zstd_avail_strms != 0 {
        let mut n: *mut z_erofs_zstd;
        strm = z_erofs_isolate_strms(true);
        while !strm.is_null() {
            n = (*strm).next;
            (*strm).next = head;
            head = strm;
            z_erofs_zstd_avail_strms -= 1;
            strm = n;
        }
    }

    // 2. walk each isolated stream and grow max dict_size if needed
    wkspsz = zstd_dstream_workspace_bound(dict_size);
    strm = head;
    while !strm.is_null() {
        wksp = kvmalloc(wkspsz, GFP_KERNEL);
        if wksp.is_null() { break; }
        kvfree((*strm).wksp);
        (*strm).wksp = wksp;
        (*strm).wkspsz = wkspsz;
        strm = (*strm).next;
    }

    // 3. push back all to the global list and update max dict_size
    spin_lock(&mut Z_EROFS_ZSTD_LOCK);
    DBG_BUGON(!z_erofs_zstd_head.is_null());
    z_erofs_zstd_head = head;
    spin_unlock(&mut Z_EROFS_ZSTD_LOCK);
    z_erofs_zstd_avail_strms = z_erofs_zstd_nstrms;
    wake_up_all(&mut Z_EROFS_ZSTD_WQ);
    if strm.is_null() { z_erofs_zstd_max_dictsize = dict_size; }
    mutex_unlock(&mut zstd_resize_mutex);
    if strm.is_null() { 0 } else { -ENOMEM }
}

unsafe fn z_erofs_zstd_decompress(
    rq: *mut z_erofs_decompress_req,
    pgpl: *mut *mut page,
) -> *const i8 {
    let sb = (*rq).sb;
    let mut dctx = z_erofs_stream_dctx { rq, no: -1, ni: 0, ..core::mem::zeroed() };
    let mut in_buf = zstd_in_buffer { src: core::ptr::null(), size: 0, pos: 0 };
    let mut out_buf = zstd_out_buffer { dst: core::ptr::null_mut(), size: 0, pos: 0 };
    let strm: *mut z_erofs_zstd;
    let stream: *mut zstd_dstream;
    let mut reason: *const i8;
    let mut zerr: i32;

    dctx.kin = kmap_local_page(*(*rq).in_page);
    reason = z_erofs_fixup_insize(rq, dctx.kin.add((*rq).pageofs_in),
        core::cmp::min((*rq).inputsize, (*sb).s_blocksize - (*rq).pageofs_in));
    if !reason.is_null() { kunmap_local(dctx.kin); return reason; }

    strm = z_erofs_isolate_strms(false);
    stream = zstd_init_dstream(z_erofs_zstd_max_dictsize, (*strm).wksp, (*strm).wkspsz);
    if stream.is_null() { reason = ERR_PTR(-ENOMEM); goto failed_zinit; }

    (*rq).fillgaps = true;
    in_buf.size = core::cmp::min((*rq).inputsize as u32, PAGE_SIZE - (*rq).pageofs_in);
    (*rq).inputsize -= in_buf.size;
    in_buf.src = dctx.kin.add((*rq).pageofs_in);
    dctx.bounce = (*strm).bounce.as_mut_ptr();

    loop {
        dctx.inbuf_sz = in_buf.size;
        dctx.inbuf_pos = in_buf.pos;
        reason = z_erofs_stream_switch_bufs(&mut dctx, &mut out_buf.dst, &mut in_buf.src, pgpl);
        if !reason.is_null() { break; }
        if out_buf.size == out_buf.pos { out_buf.size = dctx.avail_out; out_buf.pos = 0; }
        in_buf.size = dctx.inbuf_sz;
        in_buf.pos = dctx.inbuf_pos;
        zerr = zstd_decompress_stream(stream, &mut out_buf, &mut in_buf);
        dctx.avail_out = out_buf.size - out_buf.pos;
        if zstd_is_error(zerr) || (((*rq).outputsize + dctx.avail_out) != 0 &&
            (!zerr.is_positive() || (zerr > 0 && ((*rq).inputsize + in_buf.size - in_buf.pos) == 0))) {
            reason = if zstd_is_error(zerr) { zstd_get_error_name(zerr) } else { c"unexpected end of stream".as_ptr() };
            break;
        }
        if (*rq).outputsize + dctx.avail_out == 0 { break; }
    }
    if !dctx.kout.is_null() { kunmap_local(dctx.kout); }
failed_zinit:
    kunmap_local(dctx.kin);
    spin_lock(&mut Z_EROFS_ZSTD_LOCK);
    (*strm).next = z_erofs_zstd_head;
    z_erofs_zstd_head = strm;
    spin_unlock(&mut Z_EROFS_ZSTD_LOCK);
    wake_up(&mut Z_EROFS_ZSTD_WQ);
    reason
}

pub static z_erofs_zstd_decomp: z_erofs_decompressor = z_erofs_decompressor {
    config: Some(z_erofs_load_zstd_config),
    decompress: Some(z_erofs_zstd_decompress),
    init: Some(z_erofs_zstd_init),
    exit: Some(z_erofs_zstd_exit),
    name: c"zstd".as_ptr(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
