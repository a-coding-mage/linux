// SPDX-License-Identifier: GPL-2.0
// C dependencies are supplied by the surrounding kernel translation unit.

unsafe fn nf_hook_run_bpf(
    bpf_prog: *mut core::ffi::c_void,
    skb: *mut sk_buff,
    s: *const nf_hook_state,
) -> u32 {
    let prog = bpf_prog as *const bpf_prog;
    let ctx = bpf_nf_ctx { state: s, skb };
    bpf_prog_run_pin_on_cpu(prog, &ctx)
}

#[repr(C)]
struct bpf_nf_link {
    link: bpf_link,
    hook_ops: nf_hook_ops,
    ns_tracker: netns_tracker,
    net: *mut net,
    dead: u32,
    defrag_hook: *const nf_defrag_hook,
}

// Enabled when CONFIG_NF_DEFRAG_IPV4 or CONFIG_NF_DEFRAG_IPV6 is enabled.
#[cfg(any(CONFIG_NF_DEFRAG_IPV4, CONFIG_NF_DEFRAG_IPV6))]
unsafe fn get_proto_defrag_hook(
    link: *mut bpf_nf_link,
    ptr_global_hook: *const *const nf_defrag_hook,
    mod_name: *const core::ffi::c_char,
) -> *const nf_defrag_hook {
    let mut hook: *const nf_defrag_hook;
    let err: i32;

    // RCU protects us from races against module unloading
    rcu_read_lock();
    hook = rcu_dereference(*ptr_global_hook);
    if hook.is_null() {
        rcu_read_unlock();
        err = request_module(b"%s\0".as_ptr() as *const _, mod_name);
        if err != 0 {
            return err_ptr(if err < 0 { err } else { -EINVAL });
        }

        rcu_read_lock();
        hook = rcu_dereference(*ptr_global_hook);
    }

    if !hook.is_null() && try_module_get((*hook).owner) {
        // Once we have a refcnt on the module, we no longer need RCU
        hook = rcu_pointer_handoff(hook);
    } else {
        WARN_ONCE(hook.is_null(), b"%s has bad registration\0".as_ptr(), mod_name);
        hook = err_ptr(-ENOENT);
    }
    rcu_read_unlock();

    if !is_err(hook) {
        err = ((*hook).enable)((*link).net);
        if err != 0 {
            module_put((*hook).owner);
            hook = err_ptr(err);
        }
    }

    hook
}

unsafe fn bpf_nf_enable_defrag(link: *mut bpf_nf_link) -> i32 {
    let hook: *const nf_defrag_hook;

    match (*link).hook_ops.pf {
        #[cfg(CONFIG_NF_DEFRAG_IPV4)]
        NFPROTO_IPV4 => {
            hook = get_proto_defrag_hook(link, &nf_defrag_v4_hook, b"nf_defrag_ipv4\0".as_ptr() as _);
            if is_err(hook) { return ptr_err(hook); }
            (*link).defrag_hook = hook;
            0
        }
        #[cfg(CONFIG_NF_DEFRAG_IPV6)]
        NFPROTO_IPV6 => {
            hook = get_proto_defrag_hook(link, &nf_defrag_v6_hook, b"nf_defrag_ipv6\0".as_ptr() as _);
            if is_err(hook) { return ptr_err(hook); }
            (*link).defrag_hook = hook;
            0
        }
        _ => -EAFNOSUPPORT,
    }
}

unsafe fn bpf_nf_disable_defrag(link: *mut bpf_nf_link) {
    let hook = (*link).defrag_hook;
    if hook.is_null() { return; }
    ((*hook).disable)((*link).net);
    module_put((*hook).owner);
}

unsafe extern "C" fn bpf_nf_link_release(link: *mut bpf_link) {
    let nf_link = container_of!(link, bpf_nf_link, link);
    if (*nf_link).dead != 0 { return; }
    // do not double release in case .detach was already called
    if cmpxchg(&mut (*nf_link).dead, 0, 1) == 0 {
        nf_unregister_net_hook((*nf_link).net, &mut (*nf_link).hook_ops);
        bpf_nf_disable_defrag(nf_link);
        put_net_track((*nf_link).net, &mut (*nf_link).ns_tracker);
    }
}

unsafe extern "C" fn bpf_nf_link_dealloc(link: *mut bpf_link) {
    let nf_link = container_of!(link, bpf_nf_link, link);
    kfree(nf_link);
}

unsafe extern "C" fn bpf_nf_link_detach(link: *mut bpf_link) -> i32 {
    bpf_nf_link_release(link);
    0
}

unsafe extern "C" fn bpf_nf_link_show_info(link: *const bpf_link, seq: *mut seq_file) {
    let nf_link = container_of!(link, bpf_nf_link, link);
    seq_printf(seq, b"pf:\t%u\thooknum:\t%u\tprio:\t%d\n\0".as_ptr(),
        (*nf_link).hook_ops.pf, (*nf_link).hook_ops.hooknum, (*nf_link).hook_ops.priority);
}

unsafe extern "C" fn bpf_nf_link_fill_link_info(link: *const bpf_link, info: *mut bpf_link_info) -> i32 {
    let nf_link = container_of!(link, bpf_nf_link, link);
    let hook = (*nf_link).defrag_hook;
    (*info).netfilter.pf = (*nf_link).hook_ops.pf;
    (*info).netfilter.hooknum = (*nf_link).hook_ops.hooknum;
    (*info).netfilter.priority = (*nf_link).hook_ops.priority;
    (*info).netfilter.flags = if !hook.is_null() { BPF_F_NETFILTER_IP_DEFRAG } else { 0 };
    0
}

unsafe extern "C" fn bpf_nf_link_update(_: *mut bpf_link, _: *mut bpf_prog, _: *mut bpf_prog) -> i32 {
    -EOPNOTSUPP
}

static BPF_NF_LINK_LOPS: bpf_link_ops = bpf_link_ops {
    release: Some(bpf_nf_link_release),
    dealloc_deferred: Some(bpf_nf_link_dealloc),
    detach: Some(bpf_nf_link_detach),
    show_fdinfo: Some(bpf_nf_link_show_info),
    fill_link_info: Some(bpf_nf_link_fill_link_info),
    update_prog: Some(bpf_nf_link_update),
};

unsafe fn bpf_nf_check_pf_and_hooks(attr: *const bpf_attr) -> i32 {
    let prio: i32;
    match (*attr).link_create.netfilter.pf {
        NFPROTO_IPV4 | NFPROTO_IPV6 => {
            if (*attr).link_create.netfilter.hooknum >= NF_INET_NUMHOOKS { return -EPROTO; }
        }
        _ => return -EAFNOSUPPORT,
    }
    if (*attr).link_create.netfilter.flags & !BPF_F_NETFILTER_IP_DEFRAG != 0 { return -EOPNOTSUPP; }
    // make sure conntrack confirm is always last
    prio = (*attr).link_create.netfilter.priority;
    if prio == NF_IP_PRI_FIRST { return -ERANGE; } // sabotage_in and other warts
    else if prio == NF_IP_PRI_LAST { return -ERANGE; } // e.g. conntrack confirm
    else if (*attr).link_create.netfilter.flags & BPF_F_NETFILTER_IP_DEFRAG != 0 && prio <= NF_IP_PRI_CONNTRACK_DEFRAG { return -ERANGE; } // cannot use defrag if prog runs before nf_defrag
    0
}

pub unsafe extern "C" fn bpf_nf_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> i32 {
    let net = (*current).nsproxy.net_ns;
    let mut link_primer: bpf_link_primer = core::mem::zeroed();
    let link: *mut bpf_nf_link;
    let mut err: i32;
    if (*attr).link_create.flags != 0 { return -EINVAL; }
    err = bpf_nf_check_pf_and_hooks(attr);
    if err != 0 { return err; }
    link = kzalloc_obj!(bpf_nf_link, GFP_USER);
    if link.is_null() { return -ENOMEM; }
    bpf_link_init(&mut (*link).link, BPF_LINK_TYPE_NETFILTER, &BPF_NF_LINK_LOPS, prog, (*attr).link_create.attach_type);
    (*link).hook_ops.hook = Some(nf_hook_run_bpf);
    (*link).hook_ops.hook_ops_type = NF_HOOK_OP_BPF;
    (*link).hook_ops.priv_ = prog as *mut _;
    (*link).hook_ops.pf = (*attr).link_create.netfilter.pf;
    (*link).hook_ops.priority = (*attr).link_create.netfilter.priority;
    (*link).hook_ops.hooknum = (*attr).link_create.netfilter.hooknum;
    (*link).net = net;
    (*link).dead = 0;
    (*link).defrag_hook = core::ptr::null();
    err = bpf_link_prime(&mut (*link).link, &mut link_primer);
    if err != 0 { kfree(link); return err; }
    if (*attr).link_create.netfilter.flags & BPF_F_NETFILTER_IP_DEFRAG != 0 {
        err = bpf_nf_enable_defrag(link);
        if err != 0 { bpf_link_cleanup(&mut link_primer); return err; }
    }
    err = nf_register_net_hook(net, &mut (*link).hook_ops);
    if err != 0 { bpf_nf_disable_defrag(link); bpf_link_cleanup(&mut link_primer); return err; }
    get_net_track(net, &mut (*link).ns_tracker, GFP_KERNEL);
    bpf_link_settle(&mut link_primer)
}

pub static NETFILTER_PROG_OPS: bpf_prog_ops = bpf_prog_ops { test_run: Some(bpf_prog_test_run_nf) };

unsafe fn nf_ptr_to_btf_id(info: *mut bpf_insn_access_aux, name: *const core::ffi::c_char) -> bool {
    let btf = bpf_get_btf_vmlinux();
    if btf.is_null() || is_err(btf) { return false; }
    let type_id = btf_find_by_name_kind(btf, name, BTF_KIND_STRUCT);
    if type_id < 0 { WARN_ON_ONCE(true); return false; }
    (*info).btf = btf; (*info).btf_id = type_id; (*info).reg_type = PTR_TO_BTF_ID | PTR_TRUSTED; true
}

unsafe fn nf_is_valid_access(off: i32, size: i32, ty: bpf_access_type, _: *const bpf_prog, info: *mut bpf_insn_access_aux) -> bool {
    if off < 0 || off as usize >= core::mem::size_of::<bpf_nf_ctx>() || off % size != 0 || ty == BPF_WRITE { return false; }
    match off {
        BPF_CTX_RANGE_SKB => size == core::mem::size_of::<*mut sk_buff>() as i32 && nf_ptr_to_btf_id(info, b"sk_buff\0".as_ptr() as _),
        BPF_CTX_RANGE_STATE => size == core::mem::size_of::<*const nf_hook_state>() as i32 && nf_ptr_to_btf_id(info, b"nf_hook_state\0".as_ptr() as _),
        _ => false,
    }
}

unsafe fn bpf_nf_func_proto(func_id: bpf_func_id, prog: *const bpf_prog) -> *const bpf_func_proto {
    bpf_base_func_proto(func_id, prog)
}

pub static NETFILTER_VERIFIER_OPS: bpf_verifier_ops = bpf_verifier_ops {
    is_valid_access: Some(nf_is_valid_access),
    get_func_proto: Some(bpf_nf_func_proto),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
