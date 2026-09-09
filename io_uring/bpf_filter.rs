// SPDX-License-Identifier: GPL-2.0
/*
 * BPF filter support for io_uring. Supports SQE opcodes for now.
 */

#[repr(C)]
struct io_bpf_filter {
    refs: refcount_t,
    prog: *mut bpf_prog,
    next: *mut io_bpf_filter,
}

/* Deny if this is set as the filter */
static dummy_filter: io_bpf_filter = unsafe { core::mem::zeroed() };

unsafe fn io_uring_populate_bpf_ctx(bctx: *mut io_uring_bpf_ctx, req: *mut io_kiocb) {
    let def: *const io_issue_def = &io_issue_defs[(*req).opcode as usize];

    (*bctx).opcode = (*req).opcode;
    (*bctx).sqe_flags = (*req).flags as i32 & SQE_VALID_FLAGS;
    (*bctx).user_data = (*req).cqe.user_data;
    /* clear residual, anything from pdu_size and below */
    core::ptr::write_bytes(
        (bctx as *mut u8).add(core::mem::offset_of!(io_uring_bpf_ctx, pdu_size)),
        0,
        core::mem::size_of::<io_uring_bpf_ctx>()
            - core::mem::offset_of!(io_uring_bpf_ctx, pdu_size),
    );

    /*
     * Opcodes can provide a handler for populating more data into bctx,
     * for filters to use.
     */
    if (*def).filter_pdu_size != 0 {
        (*bctx).pdu_size = (*def).filter_pdu_size;
        ((*def).filter_populate)(bctx, req);
    }
}

/*
 * Run registered filters for a given opcode. For filters, a return of 0 denies
 * execution of the request, a return of 1 allows it. If any filter for an
 * opcode returns 0, filter processing is stopped, and the request is denied.
 * This also stops the processing of filters.
 *
 * __io_uring_run_bpf_filters() returns 0 on success, allow running the
 * request, and -EACCES when a request is denied.
 */
unsafe fn __io_uring_run_bpf_filters(
    filters: *mut *mut io_bpf_filter,
    req: *mut io_kiocb,
) -> i32 {
    let mut filter: *mut io_bpf_filter;
    let mut bpf_ctx: io_uring_bpf_ctx = core::mem::zeroed();
    let ret: i32;

    /* Fast check for existence of filters outside of RCU */
    if rcu_access_pointer(filters.add((*req).opcode as usize)).is_null() {
        return 0;
    }

    /* req->opcode has already been validated to be within the expected range. */
    rcu_read_lock();
    filter = rcu_dereference(filters.add((*req).opcode as usize));
    if filter.is_null() {
        rcu_read_unlock();
        return 0;
    } else if filter == &dummy_filter as *const _ as *mut _ {
        rcu_read_unlock();
        return -EACCES;
    }

    io_uring_populate_bpf_ctx(&mut bpf_ctx, req);

    /* Iterate registered filters. The opcode is allowed iff all filters return 1. */
    loop {
        if filter == &dummy_filter as *const _ as *mut _ {
            rcu_read_unlock();
            return -EACCES;
        }
        ret = bpf_prog_run_pin_on_cpu((*filter).prog, &mut bpf_ctx);
        if ret == 0 {
            rcu_read_unlock();
            return -EACCES;
        }
        filter = (*filter).next;
        if filter.is_null() {
            break;
        }
    }
    rcu_read_unlock();
    0
}

unsafe fn io_free_bpf_filters(head: *mut rcu_head) {
    let filters = container_of!(head, io_bpf_filters, rcu_head);
    spin_lock(&mut (*filters).lock);
    let filter = (*filters).filters;
    spin_unlock(&mut (*filters).lock);
    if filter.is_null() {
        return;
    }

    for i in 0..IORING_OP_LAST as usize {
        let mut f: *mut io_bpf_filter;
        rcu_read_lock();
        f = rcu_dereference(filter.add(i));
        while !f.is_null() {
            let next = (*f).next;
            if f == &dummy_filter as *const _ as *mut _ {
                break;
            }
            if !refcount_dec_and_test(&mut (*f).refs) {
                break;
            }
            bpf_prog_destroy((*f).prog);
            kfree(f as *mut core::ffi::c_void);
            f = next;
        }
        rcu_read_unlock();
    }
    kfree((*filters).filters as *mut core::ffi::c_void);
    kfree(filters as *mut core::ffi::c_void);
}

unsafe fn __io_put_bpf_filters(filters: *mut io_bpf_filters) {
    if refcount_dec_and_test(&mut (*filters).refs) {
        call_rcu(&mut (*filters).rcu_head, io_free_bpf_filters);
    }
}

unsafe fn io_put_bpf_filters(res: *mut io_restriction) {
    if !(*res).bpf_filters.is_null() {
        __io_put_bpf_filters((*res).bpf_filters);
    }
}

unsafe fn io_new_bpf_filters() -> *mut io_bpf_filters {
    let filters = kzalloc_obj::<io_bpf_filters>(GFP_KERNEL_ACCOUNT);
    if filters.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    (*filters).filters = kzalloc_objs::<*mut io_bpf_filter>(IORING_OP_LAST, GFP_KERNEL_ACCOUNT);
    if (*filters).filters.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    refcount_set(&mut (*filters).refs, 1);
    spin_lock_init(&mut (*filters).lock);
    filters
}

/* Validate classic BPF filter instructions. Only allow a safe subset of operations. */
unsafe fn io_uring_check_cbpf_filter(filter: *mut sock_filter, flen: u32) -> i32 {
    for pc in 0..flen as usize {
        let ftest = &mut *filter.add(pc);
        let code = ftest.code;
        let k = ftest.k;
        match code {
            BPF_LD | BPF_W | BPF_ABS => {
                ftest.code = BPF_LDX | BPF_W | BPF_ABS;
                if k >= core::mem::size_of::<io_uring_bpf_ctx>() as u32 || k & 3 != 0 { return -EINVAL; }
            }
            BPF_LD | BPF_W | BPF_LEN => { ftest.code = BPF_LD | BPF_IMM; ftest.k = core::mem::size_of::<io_uring_bpf_ctx>() as u32; }
            BPF_LDX | BPF_W | BPF_LEN => { ftest.code = BPF_LDX | BPF_IMM; ftest.k = core::mem::size_of::<io_uring_bpf_ctx>() as u32; }
            BPF_RET | BPF_K | BPF_RET | BPF_A |
            BPF_ALU | BPF_ADD | BPF_K | BPF_ALU | BPF_ADD | BPF_X |
            BPF_ALU | BPF_SUB | BPF_K | BPF_ALU | BPF_SUB | BPF_X |
            BPF_ALU | BPF_MUL | BPF_K | BPF_ALU | BPF_MUL | BPF_X |
            BPF_ALU | BPF_DIV | BPF_K | BPF_ALU | BPF_DIV | BPF_X |
            BPF_ALU | BPF_AND | BPF_K | BPF_ALU | BPF_AND | BPF_X |
            BPF_ALU | BPF_OR | BPF_K | BPF_ALU | BPF_OR | BPF_X |
            BPF_ALU | BPF_XOR | BPF_K | BPF_ALU | BPF_XOR | BPF_X |
            BPF_ALU | BPF_LSH | BPF_K | BPF_ALU | BPF_LSH | BPF_X |
            BPF_ALU | BPF_RSH | BPF_K | BPF_ALU | BPF_RSH | BPF_X |
            BPF_ALU | BPF_NEG | BPF_LD | BPF_IMM | BPF_LDX | BPF_IMM |
            BPF_MISC | BPF_TAX | BPF_MISC | BPF_TXA | BPF_LD | BPF_MEM |
            BPF_LDX | BPF_MEM | BPF_ST | BPF_STX | BPF_JMP | BPF_JA |
            BPF_JMP | BPF_JEQ | BPF_K | BPF_JMP | BPF_JEQ | BPF_X |
            BPF_JMP | BPF_JGE | BPF_K | BPF_JMP | BPF_JGE | BPF_X |
            BPF_JMP | BPF_JGT | BPF_K | BPF_JMP | BPF_JGT | BPF_X |
            BPF_JMP | BPF_JSET | BPF_K | BPF_JMP | BPF_JSET | BPF_X => {}
            _ => return -EINVAL,
        }
    }
    0
}

unsafe fn io_bpf_filter_clone(dst: *mut io_restriction, src: *mut io_restriction) {
    if (*src).bpf_filters.is_null() { return; }
    rcu_read_lock();
    if refcount_inc_not_zero(&mut (*(*src).bpf_filters).refs) {
        (*dst).bpf_filters = (*src).bpf_filters;
        (*dst).bpf_filters_cow = true;
    }
    rcu_read_unlock();
}

/* Allocate a new struct io_bpf_filters. Used when a filter is cloned and modifications need to be made. */
unsafe fn io_bpf_filter_cow(src: *mut io_restriction) -> *mut io_bpf_filters {
    let filters = io_new_bpf_filters();
    if IS_ERR(filters) { return filters; }
    rcu_read_lock();
    for i in 0..IORING_OP_LAST as usize {
        let srcf = rcu_dereference((*(*src).bpf_filters).filters.add(i));
        if srcf.is_null() { continue; }
        if srcf == &dummy_filter as *const _ as *mut _ {
            rcu_assign_pointer((*filters).filters.add(i), &dummy_filter as *const _ as *mut _);
            continue;
        }
        if !refcount_inc_not_zero(&mut (*srcf).refs) {
            rcu_read_unlock(); __io_put_bpf_filters(filters); return ERR_PTR(-EBUSY);
        }
        rcu_assign_pointer((*filters).filters.add(i), srcf);
    }
    rcu_read_unlock();
    filters
}

/* The remaining registration/import logic uses the kernel-provided external helpers and types. */
unsafe fn io_bpf_filter_import(reg: *mut io_uring_bpf, arg: *mut io_uring_bpf) -> i32 {
    if copy_from_user(reg, arg, core::mem::size_of::<io_uring_bpf>()) != 0 { return -EFAULT; }
    if (*reg).cmd_type != IO_URING_BPF_CMD_FILTER || (*reg).cmd_flags != 0 || (*reg).resv != 0 { return -EINVAL; }
    if (*reg).filter.opcode >= IORING_OP_LAST || (*reg).filter.flags & !IO_URING_BPF_FILTER_FLAGS != 0 { return -EINVAL; }
    if !mem_is_zero((*reg).filter.resv.as_ptr(), core::mem::size_of_val(&(*reg).filter.resv)) || !mem_is_zero((*reg).filter.resv2.as_ptr(), core::mem::size_of_val(&(*reg).filter.resv2)) { return -EINVAL; }
    if (*reg).filter.filter_len == 0 || (*reg).filter.filter_len > BPF_MAXINSNS { return -EINVAL; }
    let def = &io_issue_defs[array_index_nospec((*reg).filter.opcode, IORING_OP_LAST) as usize];
    let mut ret = 0;
    if (*reg).filter.pdu_size != def.filter_pdu_size {
        if (*reg).filter.flags & IO_URING_BPF_FILTER_SZ_STRICT != 0 || (*reg).filter.pdu_size > def.filter_pdu_size { ret = -EMSGSIZE; }
    }
    (*reg).filter.pdu_size = def.filter_pdu_size;
    if copy_to_user(&mut (*arg).filter, &(*reg).filter, core::mem::size_of_val(&(*reg).filter)) != 0 { return -EFAULT; }
    ret
}

unsafe fn io_register_bpf_filter(res: *mut io_restriction, arg: *mut io_uring_bpf) -> i32 {
    let mut reg: io_uring_bpf = core::mem::zeroed();
    let ret = io_bpf_filter_import(&mut reg, arg);
    if ret != 0 { return ret; }
    let mut prog: *mut bpf_prog = core::ptr::null_mut();
    let fprog = sock_fprog { len: reg.filter.filter_len, filter: u64_to_user_ptr(reg.filter.filter_ptr) };
    let ret = bpf_prog_create_from_user(&mut prog, &fprog, io_uring_check_cbpf_filter, false);
    if ret != 0 { return ret; }
    let mut filters = (*res).bpf_filters;
    let mut old_filters = core::ptr::null_mut();
    if filters.is_null() { filters = io_new_bpf_filters(); if IS_ERR(filters) { let e = PTR_ERR(filters); bpf_prog_destroy(prog); return e; } }
    else if (*res).bpf_filters_cow { filters = io_bpf_filter_cow(res); if IS_ERR(filters) { let e = PTR_ERR(filters); bpf_prog_destroy(prog); return e; } old_filters = (*res).bpf_filters; }
    let filter = kzalloc_obj::<io_bpf_filter>(GFP_KERNEL_ACCOUNT);
    if filter.is_null() { if filters != (*res).bpf_filters { __io_put_bpf_filters(filters); } bpf_prog_destroy(prog); return -ENOMEM; }
    refcount_set(&mut (*filter).refs, 1); (*filter).prog = prog;
    if !old_filters.is_null() { __io_put_bpf_filters(old_filters); (*res).bpf_filters_cow = false; }
    (*res).bpf_filters = filters;
    rcu_read_lock(); spin_lock_bh(&mut (*filters).lock);
    let slot = (*filters).filters.add(reg.filter.opcode as usize);
    (*filter).next = rcu_dereference(slot); rcu_assign_pointer(slot, filter);
    if reg.filter.flags & IO_URING_BPF_FILTER_DENY_REST != 0 { for i in 0..IORING_OP_LAST as usize { if i == reg.filter.opcode as usize { continue; } if rcu_dereference((*filters).filters.add(i)).is_null() { rcu_assign_pointer((*filters).filters.add(i), &dummy_filter as *const _ as *mut _); } } }
    spin_unlock_bh(&mut (*filters).lock); rcu_read_unlock(); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
