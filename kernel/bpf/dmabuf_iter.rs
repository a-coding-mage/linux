// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2025 Google LLC */

// External kernel definitions and macros from the included Linux headers are
// intentionally referenced here but not reimplemented in this translation.

#[repr(C)]
pub struct dmabuf_iter_priv {
    /*
     * If this pointer is non-NULL, the buffer's refcount is elevated to
     * prevent destruction between stop/start. If reading is not resumed and
     * start is never called again, then dmabuf_iter_seq_fini drops the
     * reference when the iterator is released.
     */
    pub dmabuf: *mut dma_buf,
}

extern "C" {
    pub type dma_buf;
    pub type seq_file;
    pub type bpf_prog;
    pub type bpf_iter_meta;
    pub type bpf_iter_aux_info;
    pub type bpf_iter_dmabuf;

    fn dma_buf_iter_begin() -> *mut dma_buf;
    fn dma_buf_iter_next(dmabuf: *mut dma_buf) -> *mut dma_buf;
    fn dma_buf_put(dmabuf: *mut dma_buf);
    fn bpf_iter_get_info(meta: *mut bpf_iter_meta, in_stop: bool) -> *mut bpf_prog;
    fn bpf_iter_run_prog(prog: *mut bpf_prog, ctx: *mut bpf_iter__dmabuf) -> i32;
    fn seq_puts(seq: *mut seq_file, s: *const core::ffi::c_char);
    fn bpf_iter_reg_target(info: *mut bpf_iter_reg) -> i32;
}

#[repr(C)]
pub struct bpf_iter__dmabuf {
    pub meta: *mut bpf_iter_meta,
    pub dmabuf: *mut dma_buf,
}

unsafe fn __dmabuf_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void, in_stop: bool) -> i32 {
    let mut meta = bpf_iter_meta { seq };
    let mut ctx = bpf_iter__dmabuf {
        meta: &mut meta,
        dmabuf: v as *mut dma_buf,
    };
    let prog = bpf_iter_get_info(&mut meta, in_stop);

    if !prog.is_null() {
        return bpf_iter_run_prog(prog, &mut ctx);
    }

    0
}

unsafe extern "C" fn dmabuf_iter_seq_start(
    seq: *mut seq_file,
    pos: *mut i64,
) -> *mut core::ffi::c_void {
    let p = (*seq).private as *mut dmabuf_iter_priv;

    if *pos != 0 {
        let dmabuf = (*p).dmabuf;

        if dmabuf.is_null() {
            return core::ptr::null_mut();
        }

        /* Always resume from where we stopped, regardless of the value of pos. */
        (*p).dmabuf = core::ptr::null_mut();
        return dmabuf as *mut core::ffi::c_void;
    }

    dma_buf_iter_begin() as *mut core::ffi::c_void
}

unsafe extern "C" fn dmabuf_iter_seq_next(
    _seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut i64,
) -> *mut core::ffi::c_void {
    let dmabuf = v as *mut dma_buf;
    *pos += 1;
    dma_buf_iter_next(dmabuf) as *mut core::ffi::c_void
}

unsafe extern "C" fn dmabuf_iter_seq_show(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
) -> i32 {
    __dmabuf_seq_show(seq, v, false)
}

unsafe extern "C" fn dmabuf_iter_seq_stop(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
) {
    let dmabuf = v as *mut dma_buf;

    if !dmabuf.is_null() {
        let p = (*seq).private as *mut dmabuf_iter_priv;
        (*p).dmabuf = dmabuf;
    }
}

#[repr(C)]
pub struct seq_operations {
    pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut core::ffi::c_void>,
    pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void, *mut i64) -> *mut core::ffi::c_void>,
    pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void)>,
    pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32>,
}

static DMABUF_ITER_SEQ_OPS: seq_operations = seq_operations {
    start: Some(dmabuf_iter_seq_start),
    next: Some(dmabuf_iter_seq_next),
    stop: Some(dmabuf_iter_seq_stop),
    show: Some(dmabuf_iter_seq_show),
};

unsafe extern "C" fn bpf_iter_dmabuf_show_fdinfo(
    _aux: *const bpf_iter_aux_info,
    seq: *mut seq_file,
) {
    seq_puts(seq, b"dmabuf iter\n\0".as_ptr() as *const core::ffi::c_char);
}

unsafe extern "C" fn dmabuf_iter_seq_init(
    priv_: *mut core::ffi::c_void,
    _aux: *mut bpf_iter_aux_info,
) -> i32 {
    let p = priv_ as *mut dmabuf_iter_priv;
    (*p).dmabuf = core::ptr::null_mut();
    0
}

unsafe extern "C" fn dmabuf_iter_seq_fini(priv_: *mut core::ffi::c_void) {
    let p = priv_ as *mut dmabuf_iter_priv;

    if !(*p).dmabuf.is_null() {
        dma_buf_put((*p).dmabuf);
    }
}

#[repr(C)]
pub struct bpf_iter_seq_info {
    pub seq_ops: *const seq_operations,
    pub init_seq_private: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut bpf_iter_aux_info) -> i32>,
    pub fini_seq_private: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub seq_priv_size: usize,
}

static DMABUF_ITER_SEQ_INFO: bpf_iter_seq_info = bpf_iter_seq_info {
    seq_ops: &DMABUF_ITER_SEQ_OPS,
    init_seq_private: Some(dmabuf_iter_seq_init),
    fini_seq_private: Some(dmabuf_iter_seq_fini),
    seq_priv_size: core::mem::size_of::<dmabuf_iter_priv>(),
};

#[repr(C)]
pub struct bpf_iter_reg {
    pub target: *const core::ffi::c_char,
    pub feature: u32,
    pub show_fdinfo: Option<unsafe extern "C" fn(*const bpf_iter_aux_info, *mut seq_file)>,
    pub ctx_arg_info_size: u32,
    pub ctx_arg_info: [bpf_iter_arg_info; 1],
    pub seq_info: *const bpf_iter_seq_info,
}

#[repr(C)]
pub struct bpf_iter_arg_info {
    pub offset: usize,
    pub type_: u32,
    pub btf_id: u32,
}

static mut BPF_DMABUF_REG_INFO: bpf_iter_reg = bpf_iter_reg {
    target: b"dmabuf\0".as_ptr() as *const core::ffi::c_char,
    feature: 1, // BPF_ITER_RESCHED
    show_fdinfo: Some(bpf_iter_dmabuf_show_fdinfo),
    ctx_arg_info_size: 1,
    ctx_arg_info: [bpf_iter_arg_info { offset: 0, type_: 0, btf_id: 0 }],
    seq_info: &DMABUF_ITER_SEQ_INFO,
};

unsafe extern "C" fn dmabuf_iter_init() -> i32 {
    BPF_DMABUF_REG_INFO.ctx_arg_info[0].btf_id = BPF_DMABUF_BTF_ID[0];
    bpf_iter_reg_target(&mut BPF_DMABUF_REG_INFO)
}

// DEFINE_BPF_ITER_FUNC(dmabuf, struct bpf_iter_meta *, struct dma_buf *)
// BTF_ID_LIST_SINGLE(bpf_dmabuf_btf_id, struct, dma_buf)
static BPF_DMABUF_BTF_ID: [u32; 1] = [0];

#[repr(C, align(8))]
pub struct bpf_iter_dmabuf_opaque {
    /* opaque iterator state; __u64 preserves correct BTF alignment */
    pub __opaque: [u64; 1],
}

#[repr(C, align(8))]
pub struct bpf_iter_dmabuf_kern {
    pub dmabuf: *mut dma_buf,
}

pub unsafe extern "C" fn bpf_iter_dmabuf_new(it: *mut bpf_iter_dmabuf_opaque) -> i32 {
    let kit = it as *mut bpf_iter_dmabuf_kern;
    (*kit).dmabuf = core::ptr::null_mut();
    0
}

pub unsafe extern "C" fn bpf_iter_dmabuf_next(
    it: *mut bpf_iter_dmabuf_opaque,
) -> *mut dma_buf {
    let kit = it as *mut bpf_iter_dmabuf_kern;

    if !(*kit).dmabuf.is_null() {
        (*kit).dmabuf = dma_buf_iter_next((*kit).dmabuf);
    } else {
        (*kit).dmabuf = dma_buf_iter_begin();
    }

    (*kit).dmabuf
}

pub unsafe extern "C" fn bpf_iter_dmabuf_destroy(it: *mut bpf_iter_dmabuf_opaque) {
    let kit = it as *mut bpf_iter_dmabuf_kern;

    if !(*kit).dmabuf.is_null() {
        dma_buf_put((*kit).dmabuf);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
