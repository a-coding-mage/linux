// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2020 Facebook */
// Dependencies supplied by the Linux BPF, filesystem, filter, kernel, and BTF headers.

#[repr(C)]
pub struct bpf_iter_seq_prog_info {
    pub prog_id: u32,
}

unsafe fn bpf_prog_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let info = (*seq).private as *mut bpf_iter_seq_prog_info;
    let prog: *mut bpf_prog;

    prog = bpf_prog_get_curr_or_next(&mut (*info).prog_id);
    if prog.is_null() {
        return core::ptr::null_mut();
    }

    if *pos == 0 {
        *pos += 1;
    }
    prog as *mut core::ffi::c_void
}

unsafe fn bpf_prog_seq_next(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let info = (*seq).private as *mut bpf_iter_seq_prog_info;

    *pos += 1;
    (*info).prog_id += 1;
    bpf_prog_put(v as *mut bpf_prog);
    bpf_prog_get_curr_or_next(&mut (*info).prog_id) as *mut core::ffi::c_void
}

#[repr(C)]
pub struct bpf_iter__bpf_prog {
    pub meta: *mut bpf_iter_meta,
    pub prog: *mut bpf_prog,
}

// DEFINE_BPF_ITER_FUNC(bpf_prog, struct bpf_iter_meta *meta, struct bpf_prog *prog)

unsafe fn __bpf_prog_seq_show(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    in_stop: bool,
) -> i32 {
    let mut ctx: bpf_iter__bpf_prog;
    let mut meta: bpf_iter_meta;
    let prog: *mut bpf_prog;
    let mut ret: i32 = 0;

    ctx.meta = &mut meta;
    ctx.prog = v as *mut bpf_prog;
    meta.seq = seq;
    prog = bpf_iter_get_info(&mut meta, in_stop);
    if !prog.is_null() {
        ret = bpf_iter_run_prog(prog, &mut ctx);
    }

    ret
}

unsafe fn bpf_prog_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    __bpf_prog_seq_show(seq, v, false)
}

unsafe fn bpf_prog_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    if v.is_null() {
        let _ = __bpf_prog_seq_show(seq, v, true);
    } else {
        bpf_prog_put(v as *mut bpf_prog);
    }
}

static bpf_prog_seq_ops: seq_operations = seq_operations {
    start: Some(bpf_prog_seq_start),
    next: Some(bpf_prog_seq_next),
    stop: Some(bpf_prog_seq_stop),
    show: Some(bpf_prog_seq_show),
};

// BTF_ID_LIST_SINGLE(btf_bpf_prog_id, struct, bpf_prog)
static mut btf_bpf_prog_id: u32 = 0;

static bpf_prog_seq_info: bpf_iter_seq_info = bpf_iter_seq_info {
    seq_ops: &bpf_prog_seq_ops,
    init_seq_private: None,
    fini_seq_private: None,
    seq_priv_size: core::mem::size_of::<bpf_iter_seq_prog_info>(),
};

static mut bpf_prog_reg_info: bpf_iter_reg = bpf_iter_reg {
    target: b"bpf_prog\0".as_ptr() as *const core::ffi::c_char,
    ctx_arg_info_size: 1,
    ctx_arg_info: [bpf_ctx_arg_info {
        offset: core::mem::offset_of!(bpf_iter__bpf_prog, prog),
        type_: PTR_TO_BTF_ID_OR_NULL,
        btf_id: 0,
    }],
    seq_info: &bpf_prog_seq_info,
};

unsafe fn bpf_prog_iter_init() -> i32 {
    bpf_prog_reg_info.ctx_arg_info[0].btf_id = btf_bpf_prog_id;
    bpf_iter_reg_target(&mut bpf_prog_reg_info)
}

// late_initcall(bpf_prog_iter_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
