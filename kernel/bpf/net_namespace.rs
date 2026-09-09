// SPDX-License-Identifier: GPL-2.0

// Translated from net_namespace.c. Kernel declarations supplied by dependencies.

#[repr(C)]
pub struct bpf_netns_link {
    pub link: bpf_link,
    pub net: *mut net,
    pub node: list_head,
    pub netns_type: netns_bpf_attach_type,
}

static mut netns_bpf_mutex: mutex = mutex::new();

unsafe fn netns_bpf_attach_type_unneed(type_: netns_bpf_attach_type) {
    match type_ {
        #[cfg(CONFIG_INET)]
        NETNS_BPF_SK_LOOKUP => static_branch_dec(&mut bpf_sk_lookup_enabled),
        _ => (),
    }
}

unsafe fn netns_bpf_attach_type_need(type_: netns_bpf_attach_type) {
    match type_ {
        #[cfg(CONFIG_INET)]
        NETNS_BPF_SK_LOOKUP => static_branch_inc(&mut bpf_sk_lookup_enabled),
        _ => (),
    }
}

unsafe fn netns_bpf_run_array_detach(net: *mut net, type_: netns_bpf_attach_type) {
    let run_array = rcu_replace_pointer((*net).bpf.run_array[type_], core::ptr::null_mut(), lockdep_is_held(&netns_bpf_mutex));
    bpf_prog_array_free(run_array);
}

unsafe fn link_index(net: *mut net, type_: netns_bpf_attach_type, link: *mut bpf_netns_link) -> i32 {
    let mut i = 0;
    list_for_each_entry!(pos, (*net).bpf.links[type_], node, bpf_netns_link, {
        if pos == link { return i; }
        i += 1;
    });
    -ENOENT
}

unsafe fn link_count(net: *mut net, type_: netns_bpf_attach_type) -> i32 {
    let mut i = 0;
    list_for_each!((*net).bpf.links[type_], { i += 1; });
    i
}

unsafe fn fill_prog_array(net: *mut net, type_: netns_bpf_attach_type, prog_array: *mut bpf_prog_array) {
    let mut i = 0;
    list_for_each_entry!(pos, (*net).bpf.links[type_], node, bpf_netns_link, {
        (*prog_array).items[i].prog = (*pos).link.prog;
        i += 1;
    });
}

unsafe fn bpf_netns_link_release(link: *mut bpf_link) {
    let net_link = container_of!(link, bpf_netns_link, link);
    let type_ = (*net_link).netns_type;
    let mut net: *mut net;
    let mut old_array: *mut bpf_prog_array;
    let mut new_array: *mut bpf_prog_array;
    let mut cnt: i32;
    let mut idx: i32;
    mutex_lock(&mut netns_bpf_mutex);
    net = (*net_link).net;
    if net.is_null() { goto_out_unlock!(net_link, netns_bpf_mutex); }
    netns_bpf_attach_type_unneed(type_);
    idx = link_index(net, type_, net_link);
    list_del(&mut (*net_link).node);
    cnt = link_count(net, type_);
    if cnt == 0 {
        netns_bpf_run_array_detach(net, type_);
        goto_out_unlock!(net_link, netns_bpf_mutex);
    }
    old_array = rcu_dereference_protected((*net).bpf.run_array[type_], lockdep_is_held(&netns_bpf_mutex));
    new_array = bpf_prog_array_alloc(cnt as usize, GFP_KERNEL);
    if new_array.is_null() {
        WARN_ON(bpf_prog_array_delete_safe_at(old_array, idx));
        goto_out_unlock!(net_link, netns_bpf_mutex);
    }
    fill_prog_array(net, type_, new_array);
    rcu_assign_pointer((*net).bpf.run_array[type_], new_array);
    bpf_prog_array_free(old_array);
    (*net_link).net = core::ptr::null_mut();
    mutex_unlock(&mut netns_bpf_mutex);
}

unsafe fn bpf_netns_link_detach(link: *mut bpf_link) -> i32 { bpf_netns_link_release(link); 0 }

unsafe fn bpf_netns_link_dealloc(link: *mut bpf_link) {
    let net_link = container_of!(link, bpf_netns_link, link);
    kfree(net_link);
}

unsafe fn bpf_netns_link_update_prog(link: *mut bpf_link, new_prog: *mut bpf_prog, mut old_prog: *mut bpf_prog) -> i32 {
    let net_link = container_of!(link, bpf_netns_link, link);
    let type_ = (*net_link).netns_type;
    let net = (*net_link).net;
    let run_array;
    let idx;
    guard_mutex!(&mut netns_bpf_mutex);
    if !old_prog.is_null() && old_prog != (*link).prog { return -EPERM; }
    if (*new_prog).type_ != (*link).prog.type_ { return -EINVAL; }
    if net.is_null() || !check_net(net) { return -ENOLINK; }
    run_array = rcu_dereference_protected((*net).bpf.run_array[type_], lockdep_is_held(&netns_bpf_mutex));
    idx = link_index(net, type_, net_link);
    let ret = bpf_prog_array_update_at(run_array, idx, new_prog);
    if ret != 0 { return ret; }
    old_prog = xchg(&mut (*link).prog, new_prog);
    bpf_prog_put(old_prog);
    0
}

unsafe fn bpf_netns_link_fill_info(link: *const bpf_link, info: *mut bpf_link_info) -> i32 {
    let net_link = container_of!(link, bpf_netns_link, link);
    let mut inum = 0;
    mutex_lock(&mut netns_bpf_mutex);
    let net = (*net_link).net;
    if !net.is_null() && check_net(net) { inum = (*net).ns.inum; }
    mutex_unlock(&mut netns_bpf_mutex);
    (*info).netns.netns_ino = inum;
    (*info).netns.attach_type = (*link).attach_type;
    0
}

unsafe fn bpf_netns_link_show_fdinfo(link: *const bpf_link, seq: *mut seq_file) {
    let mut info: bpf_link_info = core::mem::zeroed();
    bpf_netns_link_fill_info(link, &mut info);
    seq_printf!(seq, "netns_ino:\t%u\nattach_type:\t%u\n", info.netns.netns_ino, (*link).attach_type);
}

static bpf_netns_link_ops: bpf_link_ops = bpf_link_ops {
    release: Some(bpf_netns_link_release), dealloc: Some(bpf_netns_link_dealloc),
    detach: Some(bpf_netns_link_detach), update_prog: Some(bpf_netns_link_update_prog),
    fill_link_info: Some(bpf_netns_link_fill_info), show_fdinfo: Some(bpf_netns_link_show_fdinfo),
};

unsafe fn __netns_bpf_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr, net: *mut net, type_: netns_bpf_attach_type) -> i32 {
    let prog_ids = u64_to_user_ptr((*attr).query.prog_ids);
    let run_array = rcu_dereference_protected((*net).bpf.run_array[type_], lockdep_is_held(&netns_bpf_mutex));
    let prog_cnt = if !run_array.is_null() { bpf_prog_array_length(run_array) } else { 0 };
    let flags = 0;
    if copy_to_user(&mut (*uattr).query.attach_flags, &flags, core::mem::size_of_val(&flags)) != 0 { return -EFAULT; }
    if copy_to_user(&mut (*uattr).query.prog_cnt, &prog_cnt, core::mem::size_of_val(&prog_cnt)) != 0 { return -EFAULT; }
    if (*attr).query.prog_cnt == 0 || prog_ids.is_null() || prog_cnt == 0 { return 0; }
    bpf_prog_array_copy_to_user(run_array, prog_ids, (*attr).query.prog_cnt)
}

pub unsafe fn netns_bpf_prog_query(attr: *const bpf_attr, uattr: *mut bpf_attr) -> i32 {
    if (*attr).query.query_flags != 0 { return -EINVAL; }
    let type_ = to_netns_bpf_attach_type((*attr).query.attach_type);
    if type_ < 0 { return -EINVAL; }
    let net = get_net_ns_by_fd((*attr).query.target_fd);
    if IS_ERR(net) { return PTR_ERR(net); }
    mutex_lock(&mut netns_bpf_mutex);
    let ret = __netns_bpf_prog_query(attr, uattr, net, type_);
    mutex_unlock(&mut netns_bpf_mutex);
    put_net(net);
    ret
}

pub unsafe fn netns_bpf_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32 {
    if (*attr).target_fd != 0 || (*attr).attach_flags != 0 || (*attr).replace_bpf_fd != 0 { return -EINVAL; }
    let type_ = to_netns_bpf_attach_type((*attr).attach_type);
    if type_ < 0 { return -EINVAL; }
    let net = (*current).nsproxy.net_ns;
    mutex_lock(&mut netns_bpf_mutex);
    if !list_empty(&(*net).bpf.links[type_]) { mutex_unlock(&mut netns_bpf_mutex); return -EEXIST; }
    let mut ret = match type_ {
        NETNS_BPF_FLOW_DISSECTOR => flow_dissector_bpf_prog_attach_check(net, prog),
        _ => -EINVAL,
    };
    if ret != 0 { mutex_unlock(&mut netns_bpf_mutex); return ret; }
    let attached = (*net).bpf.progs[type_];
    if attached == prog { mutex_unlock(&mut netns_bpf_mutex); return -EINVAL; }
    let mut run_array = rcu_dereference_protected((*net).bpf.run_array[type_], lockdep_is_held(&netns_bpf_mutex));
    if !run_array.is_null() {
        WRITE_ONCE!((*run_array).items[0].prog, prog);
    } else {
        run_array = bpf_prog_array_alloc(1, GFP_KERNEL);
        if run_array.is_null() { mutex_unlock(&mut netns_bpf_mutex); return -ENOMEM; }
        (*run_array).items[0].prog = prog;
        rcu_assign_pointer((*net).bpf.run_array[type_], run_array);
    }
    (*net).bpf.progs[type_] = prog;
    if !attached.is_null() { bpf_prog_put(attached); }
    mutex_unlock(&mut netns_bpf_mutex);
    ret
}

unsafe fn __netns_bpf_prog_detach(net: *mut net, type_: netns_bpf_attach_type, old: *mut bpf_prog) -> i32 {
    if !list_empty(&(*net).bpf.links[type_]) { return -EINVAL; }
    let attached = (*net).bpf.progs[type_];
    if attached.is_null() || attached != old { return -ENOENT; }
    netns_bpf_run_array_detach(net, type_);
    (*net).bpf.progs[type_] = core::ptr::null_mut();
    bpf_prog_put(attached);
    0
}

pub unsafe fn netns_bpf_prog_detach(attr: *const bpf_attr, ptype: bpf_prog_type) -> i32 {
    if (*attr).target_fd != 0 { return -EINVAL; }
    let type_ = to_netns_bpf_attach_type((*attr).attach_type);
    if type_ < 0 { return -EINVAL; }
    let prog = bpf_prog_get_type((*attr).attach_bpf_fd, ptype);
    if IS_ERR(prog) { return PTR_ERR(prog); }
    mutex_lock(&mut netns_bpf_mutex);
    let ret = __netns_bpf_prog_detach((*current).nsproxy.net_ns, type_, prog);
    mutex_unlock(&mut netns_bpf_mutex);
    bpf_prog_put(prog);
    ret
}

unsafe fn netns_bpf_max_progs(type_: netns_bpf_attach_type) -> i32 {
    match type_ { NETNS_BPF_FLOW_DISSECTOR => 1, NETNS_BPF_SK_LOOKUP => 64, _ => 0 }
}

unsafe fn netns_bpf_link_attach(net: *mut net, link: *mut bpf_link, type_: netns_bpf_attach_type) -> i32 {
    let net_link = container_of!(link, bpf_netns_link, link);
    mutex_lock(&mut netns_bpf_mutex);
    let cnt = link_count(net, type_);
    if cnt >= netns_bpf_max_progs(type_) { mutex_unlock(&mut netns_bpf_mutex); return -E2BIG; }
    if !(*net).bpf.progs[type_].is_null() { mutex_unlock(&mut netns_bpf_mutex); return -EEXIST; }
    let err = match type_ {
        NETNS_BPF_FLOW_DISSECTOR => flow_dissector_bpf_prog_attach_check(net, (*link).prog),
        NETNS_BPF_SK_LOOKUP => 0,
        _ => -EINVAL,
    };
    if err != 0 { mutex_unlock(&mut netns_bpf_mutex); return err; }
    let run_array = bpf_prog_array_alloc((cnt + 1) as usize, GFP_KERNEL);
    if run_array.is_null() { mutex_unlock(&mut netns_bpf_mutex); return -ENOMEM; }
    list_add_tail(&mut (*net_link).node, &mut (*net).bpf.links[type_]);
    fill_prog_array(net, type_, run_array);
    let old = rcu_replace_pointer((*net).bpf.run_array[type_], run_array, lockdep_is_held(&netns_bpf_mutex));
    bpf_prog_array_free(old);
    netns_bpf_attach_type_need(type_);
    mutex_unlock(&mut netns_bpf_mutex);
    0
}

pub unsafe fn netns_bpf_link_create(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32 {
    if (*attr).link_create.flags != 0 { return -EINVAL; }
    let type_ = (*attr).link_create.attach_type;
    let netns_type = to_netns_bpf_attach_type(type_);
    if netns_type < 0 { return -EINVAL; }
    let net = get_net_ns_by_fd((*attr).link_create.target_fd);
    if IS_ERR(net) { return PTR_ERR(net); }
    let net_link = kzalloc_obj!(bpf_netns_link, GFP_USER);
    if net_link.is_null() { put_net(net); return -ENOMEM; }
    bpf_link_init(&mut (*net_link).link, BPF_LINK_TYPE_NETNS, &bpf_netns_link_ops, prog, type_);
    (*net_link).net = net;
    (*net_link).netns_type = netns_type;
    let mut link_primer: bpf_link_primer = core::mem::zeroed();
    let mut err = bpf_link_prime(&mut (*net_link).link, &mut link_primer);
    if err != 0 { kfree(net_link); put_net(net); return err; }
    err = netns_bpf_link_attach(net, &mut (*net_link).link, netns_type);
    if err != 0 { bpf_link_cleanup(&mut link_primer); put_net(net); return err; }
    put_net(net);
    bpf_link_settle(&mut link_primer)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
