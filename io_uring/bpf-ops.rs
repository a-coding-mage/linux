/* SPDX-License-Identifier: GPL-2.0 */

// Linux kernel and io_uring dependencies supplied by the surrounding build.
// C preprocessor annotations/macros such as __bpf_kfunc, BTF_KFUNCS_* and
// __initcall are intentionally represented by comments or Rust attributes
// where no file-local Rust equivalent can be defined.

static mut IO_BPF_CTRL_MUTEX: core::mem::MaybeUninit<Mutex> = core::mem::MaybeUninit::uninit();
static mut LOOP_PARAMS_TYPE: *const btf_type = core::ptr::null();

// __bpf_kfunc_start_defs();

pub unsafe fn bpf_io_uring_submit_sqes(loop_ctx: *mut iou_ctx, nr: u32) -> i32 {
    let ctx: *mut io_ring_ctx = io_loop_demangle_ctx(loop_ctx);
    io_submit_sqes(ctx, nr)
}

pub unsafe fn bpf_io_uring_get_region(
    loop_ctx: *mut iou_ctx,
    region_id: u32,
    rdwr_buf_size: usize,
) -> *mut u8 {
    let ctx: *mut io_ring_ctx = io_loop_demangle_ctx(loop_ctx);
    let r: *mut io_mapped_region;

    lockdep_assert_held(&(*ctx).uring_lock);

    r = match region_id {
        IOU_REGION_MEM => &mut (*ctx).param_region,
        IOU_REGION_CQ => &mut (*ctx).ring_region,
        IOU_REGION_SQ => &mut (*ctx).sq_region,
        _ => return core::ptr::null_mut(),
    };

    if rdwr_buf_size > io_region_size(r) {
        return core::ptr::null_mut();
    }
    io_region_get_ptr(r)
}

// __bpf_kfunc_end_defs();

// BTF_KFUNCS_START(io_uring_kfunc_set)
// BTF_ID_FLAGS(func, bpf_io_uring_submit_sqes, KF_SLEEPABLE)
// BTF_ID_FLAGS(func, bpf_io_uring_get_region, KF_RET_NULL)
// BTF_KFUNCS_END(io_uring_kfunc_set)
static mut IO_URING_KFUNC_SET: btf_kfunc_id_set = btf_kfunc_id_set {
    owner: THIS_MODULE,
    set: &io_uring_kfunc_set,
};

unsafe fn io_bpf_ops__loop_step(_ctx: *mut iou_ctx, _lp: *mut iou_loop_params) -> i32 {
    IOU_LOOP_STOP
}

static mut IO_BPF_OPS_STUBS: io_uring_bpf_ops = io_uring_bpf_ops {
    loop_step: Some(io_bpf_ops__loop_step),
};

unsafe fn bpf_io_is_valid_access(
    off: i32,
    size: i32,
    access_type: bpf_access_type,
    prog: *const bpf_prog,
    info: *mut bpf_insn_access_aux,
) -> bool {
    if access_type != BPF_READ { return false; }
    if off < 0 || off >= (core::mem::size_of::<u64>() * MAX_BPF_FUNC_ARGS) as i32 { return false; }
    if off % size != 0 { return false; }
    btf_ctx_access(off, size, access_type, prog, info)
}

unsafe fn bpf_io_btf_struct_access(
    _log: *mut bpf_verifier_log,
    reg: *const bpf_reg_state,
    off: i32,
    size: i32,
) -> i32 {
    let t = btf_type_by_id((*reg).btf, (*reg).btf_id);
    if t == LOOP_PARAMS_TYPE {
        if off + size <= offsetofend_iou_loop_params_cq_wait_idx() { return SCALAR_VALUE; }
    }
    -EACCES
}

static mut BPF_IO_VERIFIER_OPS: bpf_verifier_ops = bpf_verifier_ops {
    get_func_proto: Some(bpf_base_func_proto),
    is_valid_access: Some(bpf_io_is_valid_access),
    btf_struct_access: Some(bpf_io_btf_struct_access),
};

unsafe fn io_lookup_struct_type(btf: *mut btf, name: *const core::ffi::c_char) -> *const btf_type {
    let type_id: i32 = btf_find_by_name_kind(btf, name, BTF_KIND_STRUCT);
    if type_id < 0 { return core::ptr::null(); }
    btf_type_by_id(btf, type_id)
}

unsafe fn bpf_io_init(btf: *mut btf) -> i32 {
    LOOP_PARAMS_TYPE = io_lookup_struct_type(btf, c"iou_loop_params".as_ptr());
    if LOOP_PARAMS_TYPE.is_null() {
        pr_err(c"io_uring: Failed to locate iou_loop_params\n".as_ptr());
        return -EINVAL;
    }
    let ret = register_btf_kfunc_id_set(BPF_PROG_TYPE_STRUCT_OPS, &BPF_IO_URING_KFUNC_SET);
    if ret != 0 {
        pr_err(c"io_uring: Failed to register kfuncs (%d)\n".as_ptr(), ret);
        return ret;
    }
    0
}

unsafe fn bpf_io_check_member(_t: *const btf_type, _member: *const btf_member, _prog: *const bpf_prog) -> i32 { 0 }

unsafe fn bpf_io_init_member(
    t: *const btf_type, member: *const btf_member, kdata: *mut core::ffi::c_void,
    udata: *const core::ffi::c_void,
) -> i32 {
    let moff = __btf_member_bit_offset(t, member) / 8;
    let uops = udata as *const io_uring_bpf_ops;
    let ops = kdata as *mut io_uring_bpf_ops;
    match moff {
        OFFSETOF_IO_URING_BPF_OPS_RING_FD => { (*ops).ring_fd = (*uops).ring_fd; 1 }
        _ => 0,
    }
}

unsafe fn io_install_bpf(ctx: *mut io_ring_ctx, ops: *mut io_uring_bpf_ops) -> i32 {
    if (*ctx).flags & (IORING_SETUP_SQPOLL | IORING_SETUP_IOPOLL) != 0 { return -EOPNOTSUPP; }
    if (*ctx).flags & IORING_SETUP_DEFER_TASKRUN == 0 { return -EOPNOTSUPP; }
    if !(*ctx).bpf_ops.is_null() || !(*ops).priv_.is_null() { return -EBUSY; }
    if (*ops).loop_step.is_none() { return -EINVAL; }
    (*ops).priv_ = ctx as *mut _;
    (*ctx).bpf_ops = ops;
    (*ctx).loop_step = (*ops).loop_step;
    0
}

unsafe fn bpf_io_reg(kdata: *mut core::ffi::c_void, _link: *mut bpf_link) -> i32 {
    let ops = kdata as *mut io_uring_bpf_ops;
    let file = io_uring_ctx_get_file((*ops).ring_fd, false);
    if IS_ERR(file) { return PTR_ERR(file); }
    let ctx = (*file).private_data as *mut io_ring_ctx;
    // scoped_guard(mutex, &io_bpf_ctrl_mutex) and guard(mutex)(&ctx->uring_lock)
    let ret = io_install_bpf(ctx, ops);
    fput(file);
    ret
}

unsafe fn io_eject_bpf(ctx: *mut io_ring_ctx) {
    let ops = (*ctx).bpf_ops;
    if ops.is_null() || (*ops).priv_ != ctx as *mut _ { return; }
    (*ops).priv_ = core::ptr::null_mut();
    (*ctx).bpf_ops = core::ptr::null_mut();
    (*ctx).loop_step = None;
}

unsafe fn bpf_io_unreg(kdata: *mut core::ffi::c_void, _link: *mut bpf_link) {
    let ops = kdata as *mut io_uring_bpf_ops;
    let ctx = (*ops).priv_ as *mut io_ring_ctx;
    if !ctx.is_null() && (*ctx).bpf_ops == ops { io_eject_bpf(ctx); }
}

pub unsafe fn io_unregister_bpf_ops(ctx: *mut io_ring_ctx) {
    // ->bpf_ops is write protected by io_bpf_ctrl_mutex and uring_lock,
    // and read protected by either. Try to avoid taking the global lock
    // for rings that never had any bpf installed.
    if (*ctx).bpf_ops.is_null() { return; }
    if !(*ctx).bpf_ops.is_null() { io_eject_bpf(ctx); }
}

static mut BPF_RING_OPS: bpf_struct_ops = bpf_struct_ops {
    verifier_ops: &BPF_IO_VERIFIER_OPS,
    reg: Some(bpf_io_reg),
    unreg: Some(bpf_io_unreg),
    check_member: Some(bpf_io_check_member),
    init_member: Some(bpf_io_init_member),
    init: Some(bpf_io_init),
    cfi_stubs: &IO_BPF_OPS_STUBS,
    name: c"io_uring_bpf_ops".as_ptr(),
    owner: THIS_MODULE,
};

unsafe fn io_uring_bpf_init() -> i32 {
    let ret = register_bpf_struct_ops(&mut BPF_RING_OPS, io_uring_bpf_ops);
    if ret != 0 {
        pr_err(c"io_uring: Failed to register struct_ops (%d)\n".as_ptr(), ret);
        return ret;
    }
    0
}

// __initcall(io_uring_bpf_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
