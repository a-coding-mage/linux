// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2022 Red Hat, Inc. */
// C dependencies: linux/bpf.h, linux/fs.h, linux/filter.h,
// linux/kernel.h, and linux/btf_ids.h.

#[repr(C)]
struct bpf_iter_seq_link_info {
    link_id: u32,
}

unsafe fn bpf_link_seq_start(seq: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    let info = (*seq).private as *mut bpf_iter_seq_link_info;
    let link: *mut bpf_link;

    link = bpf_link_get_curr_or_next(&mut (*info).link_id);
    if link.is_null() {
        return core::ptr::null_mut();
    }

    if *pos == 0 {
        *pos += 1;
    }
    link as *mut core::ffi::c_void
}

unsafe fn bpf_link_seq_next(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    pos: *mut loff_t,
) -> *mut core::ffi::c_void {
    let info = (*seq).private as *mut bpf_iter_seq_link_info;

    *pos += 1;
    (*info).link_id += 1;
    bpf_link_put(v as *mut bpf_link);
    bpf_link_get_curr_or_next(&mut (*info).link_id) as *mut core::ffi::c_void
}

#[repr(C)]
struct bpf_iter__bpf_link {
    meta: *mut bpf_iter_meta,
    link: *mut bpf_link,
}

// DEFINE_BPF_ITER_FUNC(bpf_link, struct bpf_iter_meta *meta,
//                      struct bpf_link *link)

unsafe fn __bpf_link_seq_show(
    seq: *mut seq_file,
    v: *mut core::ffi::c_void,
    in_stop: bool,
) -> i32 {
    let mut ctx: bpf_iter__bpf_link;
    let mut meta: bpf_iter_meta;
    let prog: *mut bpf_prog;
    let mut ret: i32 = 0;

    ctx.meta = &mut meta;
    ctx.link = v as *mut bpf_link;
    meta.seq = seq;
    prog = bpf_iter_get_info(&mut meta, in_stop);
    if !prog.is_null() {
        ret = bpf_iter_run_prog(prog, &mut ctx);
    }

    ret
}

unsafe fn bpf_link_seq_show(seq: *mut seq_file, v: *mut core::ffi::c_void) -> i32 {
    __bpf_link_seq_show(seq, v, false)
}

unsafe fn bpf_link_seq_stop(seq: *mut seq_file, v: *mut core::ffi::c_void) {
    if v.is_null() {
        let _ = __bpf_link_seq_show(seq, v, true);
    } else {
        bpf_link_put(v as *mut bpf_link);
    }
}

static bpf_link_seq_ops: seq_operations = seq_operations {
    start: Some(bpf_link_seq_start),
    next: Some(bpf_link_seq_next),
    stop: Some(bpf_link_seq_stop),
    show: Some(bpf_link_seq_show),
};

// BTF_ID_LIST_SINGLE(btf_bpf_link_id, struct, bpf_link)
extern "C" {
    static btf_bpf_link_id: u32;
}

static bpf_link_seq_info: bpf_iter_seq_info = bpf_iter_seq_info {
    seq_ops: &bpf_link_seq_ops,
    init_seq_private: None,
    fini_seq_private: None,
    seq_priv_size: core::mem::size_of::<bpf_iter_seq_link_info>(),
};

static mut bpf_link_reg_info: bpf_iter_reg = bpf_iter_reg {
    target: "bpf_link".as_ptr() as *const i8,
    ctx_arg_info_size: 1,
    ctx_arg_info: [bpf_ctx_arg_aux {
        offset: core::mem::offset_of!(bpf_iter__bpf_link, link),
        arg_type: PTR_TO_BTF_ID_OR_NULL,
        btf_id: 0,
    }],
    seq_info: &bpf_link_seq_info,
};

unsafe fn bpf_link_iter_init() -> i32 {
    bpf_link_reg_info.ctx_arg_info[0].btf_id = btf_bpf_link_id;
    bpf_iter_reg_target(&mut bpf_link_reg_info)
}

// late_initcall(bpf_link_iter_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
