/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2013 Jozsef Kadlecsik <kadlec@netfilter.org> */

// Template aliases corresponding to IPSET_TOKEN(MTYPE, suffix).
// The concrete generated names are supplied by the surrounding translation unit.

#[inline]
unsafe fn get_ext(set: *mut ip_set, map: *mut mtype, id: u32) -> *mut core::ffi::c_void {
    ((*map).extensions as *mut u8).add((*set).dsize.wrapping_mul(id as usize)) as *mut core::ffi::c_void
}

unsafe fn mtype_gc_init(set: *mut ip_set, gc: Option<unsafe extern "C" fn(*mut timer_list)>) {
    let map = (*set).data as *mut mtype;
    timer_setup(&mut (*map).gc, gc, 0);
    mod_timer(&mut (*map).gc, jiffies.wrapping_add(IPSET_GC_PERIOD((*set).timeout).wrapping_mul(HZ)));
}

unsafe fn mtype_ext_cleanup(set: *mut ip_set) {
    let map = (*set).data as *mut mtype;
    let mut id: u32 = 0;
    while id < (*map).elements {
        if test_bit(id, (*map).members) {
            ip_set_ext_destroy(set, get_ext(set, map, id));
        }
        id = id.wrapping_add(1);
    }
}

unsafe fn mtype_destroy(set: *mut ip_set) {
    let map = (*set).data as *mut mtype;
    if (*set).dsize != 0 && ((*set).extensions & IPSET_EXT_DESTROY) != 0 {
        mtype_ext_cleanup(set);
    }
    ip_set_free((*map).members);
    ip_set_free(map as *mut core::ffi::c_void);
    (*set).data = core::ptr::null_mut();
}

unsafe fn mtype_flush(set: *mut ip_set) {
    let map = (*set).data as *mut mtype;
    if ((*set).extensions & IPSET_EXT_DESTROY) != 0 {
        mtype_ext_cleanup(set);
    }
    bitmap_zero((*map).members, (*map).elements);
    (*set).elements = 0;
    DEBUG_NET_WARN_ON_ONCE(atomic64_read(&(*set).ext_size) > 0);
}

/* Calculate the actual memory size of the set data */
unsafe fn mtype_memsize(map: *const mtype, dsize: usize) -> usize {
    core::mem::size_of::<mtype>() + (*map).memsize + ((*map).elements as usize).wrapping_mul(dsize)
}

unsafe fn mtype_head(set: *mut ip_set, skb: *mut sk_buff) -> i32 {
    let map = (*set).data as *const mtype;
    let mut nested: *mut nlattr;
    let memsize = mtype_memsize(map, (*set).dsize).wrapping_add(atomic64_read(&(*set).ext_size) as usize);
    nested = nla_nest_start(skb, IPSET_ATTR_DATA);
    if nested.is_null() { return -EMSGSIZE; }
    if mtype_do_head(skb, map) != 0
        || nla_put_net32(skb, IPSET_ATTR_REFERENCES, htonl((*set).ref_)) != 0
        || nla_put_net32(skb, IPSET_ATTR_MEMSIZE, htonl(memsize as u32)) != 0
        || nla_put_net32(skb, IPSET_ATTR_ELEMENTS, htonl((*set).elements)) != 0
        || ip_set_put_flags(skb, set) != 0 {
        return -EMSGSIZE;
    }
    nla_nest_end(skb, nested);
    0
}

unsafe fn mtype_test(set: *mut ip_set, value: *mut core::ffi::c_void, ext: *const ip_set_ext,
                     mext: *mut ip_set_ext, flags: u32) -> i32 {
    let map = (*set).data as *mut mtype;
    let e = value as *const mtype_adt_elem;
    let x = get_ext(set, map, (*e).id);
    let ret = mtype_do_test(e, map, (*set).dsize);
    if ret <= 0 { return ret; }
    ip_set_match_extensions(set, ext, mext, flags, x)
}

unsafe fn mtype_add(set: *mut ip_set, value: *mut core::ffi::c_void, ext: *const ip_set_ext,
                    _mext: *mut ip_set_ext, flags: u32) -> i32 {
    let map = (*set).data as *mut mtype;
    let e = value as *const mtype_adt_elem;
    let x = get_ext(set, map, (*e).id);
    let mut ret = mtype_do_add(e, map, flags, (*set).dsize);
    if ret == IPSET_ADD_FAILED {
        if SET_WITH_TIMEOUT(set) && ip_set_timeout_expired(ext_timeout(x, set)) {
            (*set).elements = (*set).elements.wrapping_sub(1); ret = 0;
        } else if (flags & IPSET_FLAG_EXIST) == 0 {
            set_bit((*e).id, (*map).members); return -IPSET_ERR_EXIST;
        }
        ip_set_ext_destroy(set, x);
    }
    if ret > 0 { (*set).elements = (*set).elements.wrapping_sub(1); }
    if SET_WITH_TIMEOUT(set) { ip_set_timeout_set(ext_timeout(x, set), (*ext).timeout); }
    if SET_WITH_COUNTER(set) { ip_set_init_counter(ext_counter(x, set), ext); }
    if SET_WITH_COMMENT(set) { ip_set_init_comment(set, ext_comment(x, set), ext); }
    if SET_WITH_SKBINFO(set) { ip_set_init_skbinfo(ext_skbinfo(x, set), ext); }
    smp_mb__before_atomic(); set_bit((*e).id, (*map).members); (*set).elements = (*set).elements.wrapping_add(1); 0
}

unsafe fn mtype_del(set: *mut ip_set, value: *mut core::ffi::c_void, _ext: *const ip_set_ext,
                    _mext: *mut ip_set_ext, _flags: u32) -> i32 {
    let map = (*set).data as *mut mtype;
    let e = value as *const mtype_adt_elem;
    let x = get_ext(set, map, (*e).id);
    if mtype_do_del(e, map) != 0 { return -IPSET_ERR_EXIST; }
    ip_set_ext_destroy(set, x); (*set).elements = (*set).elements.wrapping_sub(1);
    if SET_WITH_TIMEOUT(set) && ip_set_timeout_expired(ext_timeout(x, set)) { return -IPSET_ERR_EXIST; } 0
}

#[cfg(not(IP_SET_BITMAP_STORED_TIMEOUT))]
unsafe fn mtype_is_filled(_x: *const mtype_elem) -> bool { true }

unsafe fn mtype_list(set: *const ip_set, skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let map = (*set).data as *mut mtype;
    let adt = nla_nest_start(skb, IPSET_ATTR_ADT);
    if adt.is_null() { return -EMSGSIZE; }
    rcu_read_lock();
    let first = (*cb).args[IPSET_CB_ARG0];
    while (*cb).args[IPSET_CB_ARG0] < (*map).elements {
        cond_resched_rcu();
        let id = (*cb).args[IPSET_CB_ARG0];
        let x = get_ext(set as *mut ip_set, map, id);
        if !test_bit_acquire(id, (*map).members)
            || (SET_WITH_TIMEOUT(set as *mut ip_set) && ip_set_timeout_expired(ext_timeout(x, set as *mut ip_set))) {
            (*cb).args[IPSET_CB_ARG0] = (*cb).args[IPSET_CB_ARG0].wrapping_add(1); continue;
        }
        let nested = nla_nest_start(skb, IPSET_ATTR_DATA);
        if nested.is_null() {
            if id == first { nla_nest_cancel(skb, adt); rcu_read_unlock(); return -EMSGSIZE; }
            break;
        }
        if mtype_do_list(skb, map, id, (*set).dsize) != 0
            || ip_set_put_extensions(skb, set as *mut ip_set, x, mtype_is_filled(x as *const mtype_elem)) != 0 {
            nla_nest_cancel(skb, nested); break;
        }
        nla_nest_end(skb, nested);
        (*cb).args[IPSET_CB_ARG0] = (*cb).args[IPSET_CB_ARG0].wrapping_add(1);
    }
    nla_nest_end(skb, adt); (*cb).args[IPSET_CB_ARG0] = 0; rcu_read_unlock(); 0
}

unsafe fn mtype_gc(t: *mut timer_list) {
    let map = timer_container_of(t, offset_of!(mtype, gc));
    let set = (*map).set; spin_lock_bh(&mut (*set).lock);
    let mut id = 0; while id < (*map).elements {
        if mtype_gc_test(id, map, (*set).dsize) { let x = get_ext(set, map, id);
            if ip_set_timeout_expired(ext_timeout(x, set)) { clear_bit(id, (*map).members); smp_mb__after_atomic(); ip_set_ext_destroy(set, x); (*set).elements = (*set).elements.wrapping_sub(1); }
        } id = id.wrapping_add(1);
    } spin_unlock_bh(&mut (*set).lock); (*map).gc.expires = jiffies.wrapping_add(IPSET_GC_PERIOD((*set).timeout).wrapping_mul(HZ)); add_timer(&mut (*map).gc);
}

unsafe fn mtype_cancel_gc(set: *mut ip_set) { let map = (*set).data as *mut mtype; if SET_WITH_TIMEOUT(set) { timer_delete_sync(&mut (*map).gc); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
