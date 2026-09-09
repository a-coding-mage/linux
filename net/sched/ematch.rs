// SPDX-License-Identifier: GPL-2.0-or-later
/* net/sched/ematch.c - Extended Match API */

// Linux kernel headers and symbols referenced by this translation are supplied
// by the surrounding repository.

static mut ematch_ops: list_head = LIST_HEAD_INIT();
static mut ematch_mod_lock: rwlock_t = __RW_LOCK_UNLOCKED();

unsafe fn tcf_em_lookup(kind: u16) -> *mut tcf_ematch_ops {
    let mut e: *mut tcf_ematch_ops = core::ptr::null_mut();

    read_lock(&raw mut ematch_mod_lock);
    list_for_each_entry(e, &raw mut ematch_ops, link) {
        if kind == (*e).kind {
            if !try_module_get((*e).owner) {
                e = core::ptr::null_mut();
            }
            read_unlock(&raw mut ematch_mod_lock);
            return e;
        }
    }
    read_unlock(&raw mut ematch_mod_lock);
    core::ptr::null_mut()
}

pub unsafe fn tcf_em_register(ops: *mut tcf_ematch_ops) -> i32 {
    let mut err: i32 = -EEXIST;
    let mut e: *mut tcf_ematch_ops = core::ptr::null_mut();

    if (*ops).match_.is_none() {
        return -EINVAL;
    }

    write_lock(&raw mut ematch_mod_lock);
    list_for_each_entry(e, &raw mut ematch_ops, link) {
        if (*ops).kind == (*e).kind {
            goto!(errout);
        }
    }

    list_add_tail(&mut (*ops).link, &raw mut ematch_ops);
    err = 0;
errout:
    write_unlock(&raw mut ematch_mod_lock);
    err
}

pub unsafe fn tcf_em_unregister(ops: *mut tcf_ematch_ops) {
    write_lock(&raw mut ematch_mod_lock);
    list_del(&mut (*ops).link);
    write_unlock(&raw mut ematch_mod_lock);
}

#[inline]
unsafe fn tcf_em_get_match(tree: *mut tcf_ematch_tree, index: i32) -> *mut tcf_ematch {
    (*tree).matches.add(index as usize)
}

unsafe fn tcf_em_validate(
    tp: *mut tcf_proto,
    tree_hdr: *mut tcf_ematch_tree_hdr,
    em: *mut tcf_ematch,
    nla: *mut nlattr,
    idx: i32,
) -> i32 {
    let mut err = -EINVAL;
    let em_hdr = nla_data(nla) as *mut tcf_ematch_hdr;
    let data_len = nla_len(nla) - core::mem::size_of::<tcf_ematch_hdr>() as i32;
    let data = (em_hdr as *mut u8).add(core::mem::size_of::<tcf_ematch_hdr>()) as *mut core::ffi::c_void;
    let net = (*(*(*tp).chain).block).net;

    if !TCF_EM_REL_VALID((*em_hdr).flags) { goto!(errout); }

    if (*em_hdr).kind == TCF_EM_CONTAINER {
        let ref_: u32;
        if data_len < core::mem::size_of::<u32>() as i32 { goto!(errout); }
        ref_ = *(data as *const u32);
        if ref_ >= (*tree_hdr).nmatches { goto!(errout); }
        if ref_ <= idx as u32 { goto!(errout); }
        (*em).data = ref_ as usize;
    } else {
        (*em).ops = tcf_em_lookup((*em_hdr).kind);
        if (*em).ops.is_null() {
            err = -ENOENT;
            #[cfg(CONFIG_MODULES)] {
                __rtnl_unlock();
                request_module(c"ematch-kind-%u", (*em_hdr).kind);
                rtnl_lock();
                (*em).ops = tcf_em_lookup((*em_hdr).kind);
                if !(*em).ops.is_null() {
                    module_put((*(*em).ops).owner);
                    (*em).ops = core::ptr::null_mut();
                    err = -EAGAIN;
                }
            }
            goto!(errout);
        }
        if (*(*em).ops).datalen != 0 && data_len < (*(*em).ops).datalen as i32 { goto!(errout); }
        if let Some(change) = (*(*em).ops).change {
            err = -EINVAL;
            if (*em_hdr).flags & TCF_EM_SIMPLE != 0 { goto!(errout); }
            err = change(net, data, data_len, em);
            if err < 0 { goto!(errout); }
        } else if data_len > 0 {
            if (*em_hdr).flags & TCF_EM_SIMPLE != 0 {
                if (*(*em).ops).datalen > 0 || data_len < core::mem::size_of::<u32>() as i32 { goto!(errout); }
                (*em).data = *(data as *const u32) as usize;
            } else {
                let v = kmemdup(data, data_len as usize, GFP_KERNEL);
                if v.is_null() { err = -ENOBUFS; goto!(errout); }
                (*em).data = v as usize;
            }
            (*em).datalen = data_len as u32;
        }
    }
    (*em).matchid = (*em_hdr).matchid;
    (*em).flags = (*em_hdr).flags;
    (*em).net = net;
    err = 0;
errout:
    err
}

static em_policy: [nla_policy; TCA_EMATCH_TREE_MAX as usize + 1] = [/* designated C initializers */];

pub unsafe fn tcf_em_tree_validate(tp: *mut tcf_proto, nla: *mut nlattr, tree: *mut tcf_ematch_tree) -> i32 {
    let mut idx = 0;
    let mut list_len: i32;
    let mut matches_len: usize;
    let mut err: i32;
    let mut tb: [*mut nlattr; TCA_EMATCH_TREE_MAX as usize + 1] = [core::ptr::null_mut(); TCA_EMATCH_TREE_MAX as usize + 1];
    let mut rt_match: *mut nlattr;
    let rt_hdr: *mut nlattr;
    let rt_list: *mut nlattr;
    let tree_hdr: *mut tcf_ematch_tree_hdr;

    core::ptr::write_bytes(tree, 0, 1);
    if nla.is_null() { return 0; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_EMATCH_TREE_MAX, nla, em_policy.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    err = -EINVAL;
    rt_hdr = tb[TCA_EMATCH_TREE_HDR as usize]; rt_list = tb[TCA_EMATCH_TREE_LIST as usize];
    if rt_hdr.is_null() || rt_list.is_null() { return err; }
    tree_hdr = nla_data(rt_hdr) as *mut tcf_ematch_tree_hdr;
    (*tree).hdr = *tree_hdr;
    rt_match = nla_data(rt_list) as *mut nlattr;
    list_len = nla_len(rt_list);
    matches_len = (*tree_hdr).nmatches as usize * core::mem::size_of::<tcf_ematch>();
    (*tree).matches = kzalloc(matches_len, GFP_KERNEL) as *mut tcf_ematch;
    if (*tree).matches.is_null() { return err; }
    while nla_ok(rt_match, list_len) {
        err = -EINVAL;
        if (*rt_match).nla_type != (idx + 1) as u16 || idx >= (*tree_hdr).nmatches as i32 || nla_len(rt_match) < core::mem::size_of::<tcf_ematch_hdr>() as i32 { goto!(errout_abort); }
        err = tcf_em_validate(tp, tree_hdr, tcf_em_get_match(tree, idx), rt_match, idx);
        if err < 0 { goto!(errout_abort); }
        rt_match = nla_next(rt_match, &mut list_len); idx += 1;
    }
    if idx != (*tree_hdr).nmatches as i32 { err = -EINVAL; goto!(errout_abort); }
    return 0;
errout_abort:
    tcf_em_tree_destroy(tree); err
}

pub unsafe fn tcf_em_tree_destroy(tree: *mut tcf_ematch_tree) {
    if (*tree).matches.is_null() { return; }
    for i in 0..(*tree).hdr.nmatches {
        let em = tcf_em_get_match(tree, i as i32);
        if !(*em).ops.is_null() {
            if let Some(destroy) = (*(*em).ops).destroy { destroy(em); }
            else if !tcf_em_is_simple(em) { kfree((*em).data as *mut core::ffi::c_void); }
            module_put((*(*em).ops).owner);
        }
    }
    (*tree).hdr.nmatches = 0;
    kfree((*tree).matches as *mut core::ffi::c_void);
    (*tree).matches = core::ptr::null_mut();
}

pub unsafe fn tcf_em_tree_dump(skb: *mut sk_buff, tree: *mut tcf_ematch_tree, tlv: i32) -> i32 {
    let top_start = nla_nest_start_noflag(skb, tlv); if top_start.is_null() { return -1; }
    if nla_put(skb, TCA_EMATCH_TREE_HDR, core::mem::size_of_val(&(*tree).hdr) as i32, &(*tree).hdr as *const _ as *const _) != 0 { return -1; }
    let list_start = nla_nest_start_noflag(skb, TCA_EMATCH_TREE_LIST); if list_start.is_null() { return -1; }
    for i in 0..(*tree).hdr.nmatches {
        let em = tcf_em_get_match(tree, i as i32);
        let em_hdr = tcf_ematch_hdr { kind: if (*em).ops.is_null() { TCF_EM_CONTAINER } else { (*(*em).ops).kind }, matchid: (*em).matchid, flags: (*em).flags };
        if nla_put(skb, i + 1, core::mem::size_of::<tcf_ematch_hdr>() as i32, &em_hdr as *const _ as *const _) != 0 { return -1; }
        if !(*em).ops.is_null() { if let Some(dump) = (*(*em).ops).dump { if dump(skb, em) < 0 { return -1; } } else if tcf_em_is_container(em) || tcf_em_is_simple(em) { let u = (*em).data as u32; nla_put_nohdr(skb, 4, &u as *const _ as *const _); } else if (*em).datalen > 0 { nla_put_nohdr(skb, (*em).datalen as i32, (*em).data as *const _); } }
    }
    nla_nest_end(skb, list_start); nla_nest_end(skb, top_start); 0
}

#[inline]
unsafe fn tcf_em_match(skb: *mut sk_buff, em: *mut tcf_ematch, info: *mut tcf_pkt_info) -> i32 {
    let r = ((*(*em).ops).match_)(skb, em, info);
    if tcf_em_is_inverted(em) { !r } else { r }
}

pub unsafe fn __tcf_em_tree_match(skb: *mut sk_buff, tree: *mut tcf_ematch_tree, info: *mut tcf_pkt_info) -> i32 {
    let mut stackp = 0usize; let mut match_idx = 0i32; let mut res = 0i32;
    let mut stack = [0i32; CONFIG_NET_EMATCH_STACK as usize];
    loop {
        while match_idx < (*tree).hdr.nmatches as i32 {
            let cur = tcf_em_get_match(tree, match_idx);
            if tcf_em_is_container(cur) { if stackp >= stack.len() { net_warn_ratelimited(c"tc ematch: local stack overflow, increase NET_EMATCH_STACK\n"); return -1; } stack[stackp] = match_idx; stackp += 1; match_idx = (*cur).data as i32; continue; }
            res = tcf_em_match(skb, cur, info); if tcf_em_early_end(cur, res) { break; } match_idx += 1;
        }
        if stackp == 0 { return res; }
        stackp -= 1; match_idx = stack[stackp]; let cur = tcf_em_get_match(tree, match_idx);
        if tcf_em_is_inverted(cur) { res = !res; }
        if tcf_em_early_end(cur, res) { continue; }
        match_idx += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
