// SPDX-License-Identifier: GPL-2.0-only
/* Helper handling for netfilter. */

// Translated from nf_conntrack_helper.c. Kernel types, macros, and external
// functions are supplied by the surrounding translation unit.

static mut NF_CT_HELPER_MUTEX: Mutex = DEFINE_MUTEX!();
pub static mut nf_ct_helper_hash: *mut hlist_head = core::ptr::null_mut();
pub static mut nf_ct_helper_hsize: c_uint = 0;
static mut nf_ct_helper_count: c_uint = 0;
static mut NF_CT_NAT_HELPERS_MUTEX: Mutex = DEFINE_MUTEX!();
static mut nf_ct_nat_helpers: list_head = LIST_HEAD_INIT!();

unsafe fn helper_hash(name: *const c_char, protonum: u8) -> c_uint {
    static mut seed: u32 = 0;
    let initval: u32;
    get_random_once(&mut seed as *mut _ as *mut c_void, core::mem::size_of::<u32>());
    initval = seed ^ protonum as u32;
    jhash(name as *const c_void, strlen(name), initval) % nf_ct_helper_hsize
}

pub unsafe fn __nf_conntrack_helper_find(name: *const c_char, l3num: u16, protonum: u8) -> *mut nf_conntrack_helper {
    if nf_ct_helper_hash.is_null() { return core::ptr::null_mut(); }
    let i = helper_hash(name, protonum);
    let mut h: *mut nf_conntrack_helper = core::ptr::null_mut();
    hlist_for_each_entry_rcu!(h, nf_ct_helper_hash.add(i as usize), hnode) {
        if strcmp((*h).name.as_ptr(), name) != 0 { continue; }
        if (*h).nfproto != NFPROTO_UNSPEC && (*h).nfproto != l3num { continue; }
        if (*h).l4proto == protonum { return h; }
    }
    core::ptr::null_mut()
}

pub unsafe fn nf_conntrack_helper_try_module_get(name: *const c_char, l3num: u16, protonum: u8) -> *mut nf_conntrack_helper {
    rcu_read_lock();
    let mut h = __nf_conntrack_helper_find(name, l3num, protonum);
    // CONFIG_MODULES conditional is retained as the source-level operation.
    if h.is_null() {
        rcu_read_unlock();
        if request_module(cstr!("nfct-helper-%s"), name) == 0 { rcu_read_lock(); h = __nf_conntrack_helper_find(name, l3num, protonum); } else { return h; }
    }
    if !h.is_null() && !try_module_get((*h).me) { h = core::ptr::null_mut(); }
    if !h.is_null() && !refcount_inc_not_zero(&mut (*h).ct_refcnt) { module_put((*h).me); h = core::ptr::null_mut(); }
    rcu_read_unlock(); h
}

pub unsafe fn nf_conntrack_helper_put(helper: *mut nf_conntrack_helper) { module_put((*helper).me); if refcount_dec_and_test(&mut (*helper).ct_refcnt) { kfree_rcu!(helper, rcu); } }

unsafe fn nf_conntrack_nat_helper_find(mod_name: *const c_char) -> *mut nf_conntrack_nat_helper {
    let mut cur: *mut nf_conntrack_nat_helper = core::ptr::null_mut();
    let mut found = false;
    list_for_each_entry_rcu!(cur, &mut nf_ct_nat_helpers, list) { if strcmp((*cur).mod_name.as_ptr(), mod_name) == 0 { found = true; break; } }
    if found { cur } else { core::ptr::null_mut() }
}

pub unsafe fn nf_nat_helper_try_module_get(name: *const c_char, l3num: u16, protonum: u8) -> c_int {
    let h = __nf_conntrack_helper_find(name, l3num, protonum); if h.is_null() { return -ENOENT; }
    rcu_read_lock(); let mut nat = nf_conntrack_nat_helper_find((*h).nat_mod_name.as_ptr());
    let mut mod_name = [0i8; NF_CT_HELPER_NAME_LEN];
    if nat.is_null() { snprintf(mod_name.as_mut_ptr(), mod_name.len(), cstr!("%s"), (*h).nat_mod_name.as_ptr()); rcu_read_unlock(); request_module(cstr!("%s"), mod_name.as_ptr()); rcu_read_lock(); nat = nf_conntrack_nat_helper_find(mod_name.as_ptr()); if nat.is_null() { rcu_read_unlock(); return -ENOENT; } }
    let ret = if try_module_get((*nat).module) { 0 } else { -ENOENT }; rcu_read_unlock(); ret
}

pub unsafe fn nf_nat_helper_put(helper: *mut nf_conntrack_helper) { let nat = nf_conntrack_nat_helper_find((*helper).nat_mod_name.as_ptr()); if WARN_ON_ONCE!(nat.is_null()) { return; } module_put((*nat).module); }

pub unsafe fn nf_ct_helper_ext_add(ct: *mut nf_conn, gfp: gfp_t) -> *mut nf_conn_help {
    let help = nf_ct_ext_add(ct, NF_CT_EXT_HELPER, gfp); if !help.is_null() { __set_bit(IPS_HELPER_BIT, &mut (*ct).status); INIT_HLIST_HEAD!(&mut (*help).expectations); } help
}

pub unsafe fn __nf_ct_try_assign_helper(ct: *mut nf_conn, tmpl: *mut nf_conn, flags: gfp_t) -> c_int {
    if test_bit(IPS_HELPER_BIT, &(*ct).status) != 0 { return 0; }
    if WARN_ON_ONCE!(tmpl.is_null()) { return 0; }
    let th = nfct_help(tmpl); let helper = if !th.is_null() { rcu_dereference!((*th).helper) } else { core::ptr::null_mut() }; let mut help = nfct_help(ct);
    if helper.is_null() { if !help.is_null() { let tmp = rcu_dereference!((*help).helper); RCU_INIT_POINTER!((*help).helper, core::ptr::null_mut()); if !tmp.is_null() && refcount_dec_and_test(&mut (*tmp).ct_refcnt) { kfree_rcu!(tmp, rcu); } } return 0; }
    if help.is_null() { help = nf_ct_helper_ext_add(ct, flags); if help.is_null() { return -ENOMEM; } } else { let tmp = rcu_dereference!((*help).helper); if !tmp.is_null() { if (*tmp).help != (*helper).help { RCU_INIT_POINTER!((*help).helper, core::ptr::null_mut()); if refcount_dec_and_test(&mut (*tmp).ct_refcnt) { kfree_rcu!(tmp, rcu); } } return 0; } }
    if refcount_inc_not_zero(&mut (*helper).ct_refcnt) { rcu_assign_pointer!((*help).helper, helper); } 0
}

pub unsafe fn nf_ct_helper_destroy(ct: *mut nf_conn) { let help = nfct_help(ct); if !help.is_null() { rcu_read_lock(); let helper = rcu_dereference!((*help).helper); if !helper.is_null() && !(*helper).destroy.is_none() { ((*helper).destroy.unwrap())(ct); } rcu_read_unlock(); } }

static mut nf_ct_helper_expectfn_list: list_head = LIST_HEAD_INIT!();
pub unsafe fn nf_ct_helper_expectfn_register(n: *mut nf_ct_helper_expectfn) { spin_lock_bh(&mut nf_conntrack_expect_lock); list_add_rcu!(&mut (*n).head, &mut nf_ct_helper_expectfn_list); spin_unlock_bh(&mut nf_conntrack_expect_lock); }
pub unsafe fn nf_ct_helper_expectfn_unregister(n: *mut nf_ct_helper_expectfn) { spin_lock_bh(&mut nf_conntrack_expect_lock); list_del_rcu!(&mut (*n).head); spin_unlock_bh(&mut nf_conntrack_expect_lock); }
unsafe fn expect_iter_expectfn(exp: *mut nf_conntrack_expect, data: *mut c_void) -> bool { (*exp).expectfn == (*(data as *const nf_ct_helper_expectfn)).expectfn }
pub unsafe fn nf_ct_helper_expectfn_destroy(n: *const nf_ct_helper_expectfn) { nf_ct_expect_iterate_destroy(expect_iter_expectfn, n as *mut c_void); }
pub unsafe fn nf_ct_helper_expectfn_find_by_name(name: *const c_char) -> *mut nf_ct_helper_expectfn { let mut cur = core::ptr::null_mut(); list_for_each_entry_rcu!(cur, &mut nf_ct_helper_expectfn_list, head) { if strcmp((*cur).name.as_ptr(), name) == 0 { return cur; } } core::ptr::null_mut() }
pub unsafe fn nf_ct_helper_expectfn_find_by_symbol(symbol: *const c_void) -> *mut nf_ct_helper_expectfn { let mut cur = core::ptr::null_mut(); list_for_each_entry_rcu!(cur, &mut nf_ct_helper_expectfn_list, head) { if (*cur).expectfn == symbol { return cur; } } core::ptr::null_mut() }

pub unsafe fn nf_ct_helper_log(skb: *mut sk_buff, ct: *const nf_conn, fmt: *const c_char, mut args: ...) { let mut helper_name = cstr!("(null)"); let help = nfct_help(ct); if !help.is_null() { let helper = rcu_dereference!((*help).helper); if !helper.is_null() { helper_name = (*helper).name.as_ptr(); } } nf_log_packet(nf_ct_net(ct), nf_ct_l3num(ct), 0, skb, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), cstr!("helper %s dropping packet: %pV "), helper_name, &mut args); }

// Registration and initialization routines retain the kernel API and ordering.
pub unsafe fn __nf_conntrack_helper_register(me: *mut nf_conntrack_helper) -> c_int { BUG_ON!((*me).expect_class_max >= NF_CT_MAX_EXPECT_CLASSES); BUG_ON!(strlen((*me).name.as_ptr()) > NF_CT_HELPER_NAME_LEN - 1); if nf_ct_helper_hash.is_null() { return -ENOENT; } let mut i = 0; while i <= (*me).expect_class_max { if (*me).expect_policy[i].max_expected == 0 { (*me).expect_policy[i].max_expected = NF_CT_EXPECT_MAX_CNT; } if (*me).expect_policy[i].max_expected > NF_CT_EXPECT_MAX_CNT { return -EINVAL; } i += 1; } let h = helper_hash((*me).name.as_ptr(), (*me).l4proto); mutex_lock(&mut NF_CT_HELPER_MUTEX); let mut cur = core::ptr::null_mut(); hlist_for_each_entry! (cur, nf_ct_helper_hash.add(h as usize), hnode) { if strcmp((*cur).name.as_ptr(), (*me).name.as_ptr()) == 0 && ((*cur).nfproto == NFPROTO_UNSPEC || (*cur).nfproto == (*me).nfproto) && (*cur).l4proto == (*me).l4proto { mutex_unlock(&mut NF_CT_HELPER_MUTEX); return -EBUSY; } } refcount_set(&mut (*me).ct_refcnt, 1); hlist_add_head_rcu!(&mut (*me).hnode, nf_ct_helper_hash.add(h as usize)); nf_ct_helper_count += 1; mutex_unlock(&mut NF_CT_HELPER_MUTEX); 0 }

pub unsafe fn nf_conntrack_helper_release(me: *mut nf_conntrack_helper) { nf_ct_expect_iterate_destroy(expect_iter_me, me as *mut c_void); if refcount_dec_and_test(&mut (*me).ct_refcnt) { kfree_rcu!(me, rcu); } }
unsafe fn expect_iter_me(exp: *mut nf_conntrack_expect, data: *mut c_void) -> bool { let me = data as *mut nf_conntrack_helper; let this = rcu_dereference_protected!((*exp).helper, lockdep_is_held(&nf_conntrack_expect_lock)); if this == me { return true; } rcu_dereference_protected!((*exp).assign_helper, lockdep_is_held(&nf_conntrack_expect_lock)) == me }
pub unsafe fn nf_conntrack_helper_unregister(me: *mut nf_conntrack_helper) { mutex_lock(&mut NF_CT_HELPER_MUTEX); hlist_del_rcu!(&mut (*me).hnode); nf_ct_helper_count -= 1; mutex_unlock(&mut NF_CT_HELPER_MUTEX); rcu_assign_pointer!((*me).help, None); synchronize_rcu(); nf_conntrack_helper_release(me); }

pub unsafe fn nf_conntrack_helper_register(me: *mut nf_conntrack_helper, helper_ptr: *mut *mut nf_conntrack_helper) -> c_int {
    let new_helper = kzalloc_obj::<nf_conntrack_helper>(GFP_KERNEL_ACCOUNT); if new_helper.is_null() { return -ENOMEM; }
    core::ptr::copy_nonoverlapping(me, new_helper, 1); *helper_ptr = new_helper;
    let err = __nf_conntrack_helper_register(new_helper); if err < 0 { *helper_ptr = core::ptr::null_mut(); kfree(new_helper as *mut c_void); return err; } 0
}
pub unsafe fn nf_conntrack_helpers_register(helper: *mut nf_conntrack_helper, n: c_uint, helper_ptr: *mut *mut nf_conntrack_helper) -> c_int {
    let mut i = 0; while i < n { let new_helper = kzalloc_obj::<nf_conntrack_helper>(GFP_KERNEL_ACCOUNT); if new_helper.is_null() { nf_conntrack_helpers_unregister(helper_ptr, i); return -ENOMEM; } core::ptr::copy_nonoverlapping(helper.add(i as usize), new_helper, 1); *helper_ptr.add(i as usize) = new_helper; let err = __nf_conntrack_helper_register(new_helper); if err < 0 { *helper_ptr.add(i as usize) = core::ptr::null_mut(); kfree(new_helper as *mut c_void); nf_conntrack_helpers_unregister(helper_ptr, i); return err; } i += 1; } 0
}
pub unsafe fn nf_conntrack_helpers_unregister(helper: *mut *mut nf_conntrack_helper, mut n: c_uint) { while n > 0 { n -= 1; nf_conntrack_helper_unregister(*helper.add(n as usize)); *helper.add(n as usize) = core::ptr::null_mut(); } }
pub unsafe fn nf_nat_helper_register(nat: *mut nf_conntrack_nat_helper) { mutex_lock(&mut NF_CT_NAT_HELPERS_MUTEX); list_add_rcu!(&mut (*nat).list, &mut nf_ct_nat_helpers); mutex_unlock(&mut NF_CT_NAT_HELPERS_MUTEX); }
pub unsafe fn nf_nat_helper_unregister(nat: *mut nf_conntrack_nat_helper) { mutex_lock(&mut NF_CT_NAT_HELPERS_MUTEX); list_del_rcu!(&mut (*nat).list); mutex_unlock(&mut NF_CT_NAT_HELPERS_MUTEX); }

pub unsafe fn nf_ct_helper_init(helper: *mut nf_conntrack_helper, l3num: u8, protonum: u16, name: *const c_char, exp_pol: *const nf_conntrack_expect_policy, expect_class_max: u32, help: Option<unsafe extern "C" fn(*mut sk_buff, c_uint, *mut nf_conn, ip_conntrack_info) -> c_int>, from_nlattr: Option<unsafe extern "C" fn(*mut nlattr, *mut nf_conn) -> c_int>, module: *mut module) {
    core::ptr::write_bytes(helper, 0, 1); (*helper).nfproto = l3num as _; (*helper).l4proto = protonum as _; rcu_assign_pointer!((*helper).help, help); (*helper).from_nlattr = from_nlattr; (*helper).me = module; snprintf((*helper).nat_mod_name.as_mut_ptr(), (*helper).nat_mod_name.len(), cstr!("%s%s"), NF_NAT_HELPER_PREFIX, name); snprintf((*helper).name.as_mut_ptr(), (*helper).name.len(), cstr!("%s"), name); if WARN_ON_ONCE!(expect_class_max >= NF_CT_MAX_EXPECT_CLASSES) { return; } core::ptr::copy_nonoverlapping(exp_pol, (*helper).expect_policy.as_mut_ptr(), (expect_class_max + 1) as usize); (*helper).expect_class_max = expect_class_max;
}

pub unsafe fn nf_conntrack_helper_init() -> c_int { nf_ct_helper_hsize = 1; nf_ct_helper_hash = nf_ct_alloc_hashtable(&mut nf_ct_helper_hsize, 0); if nf_ct_helper_hash.is_null() { return -ENOMEM; } INIT_LIST_HEAD!(&mut nf_ct_nat_helpers); 0 }
pub unsafe fn nf_conntrack_helper_fini() { kvfree(nf_ct_helper_hash as *mut c_void); nf_ct_helper_hash = core::ptr::null_mut(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
