// SPDX-License-Identifier: GPL-2.0
// Translated from ila_xlat.c. Kernel-provided types, constants, macros, and
// functions referenced below are supplied by the surrounding translation.

#[repr(C)]
pub struct ila_xlat_params {
    pub ip: ila_params,
    pub ifindex: ::core::ffi::c_int,
}

#[repr(C)]
pub struct ila_map {
    pub xp: ila_xlat_params,
    pub node: rhash_head,
    pub next: *mut ila_map,
    pub rcu: rcu_head,
}

const MAX_LOCKS: u32 = 1024;
const LOCKS_PER_CPU: u32 = 10;

unsafe fn alloc_ila_locks(ilan: *mut ila_net) -> c_int {
    alloc_bucket_spinlocks(&mut (*ilan).xlat.locks, &mut (*ilan).xlat.locks_mask,
                           MAX_LOCKS, LOCKS_PER_CPU, GFP_KERNEL)
}

static mut hashrnd: u32 = 0;

#[inline(always)]
unsafe fn __ila_hash_secret_init() {
    net_get_random_once((&raw mut hashrnd).cast(), core::mem::size_of::<u32>());
}

#[inline]
unsafe fn ila_locator_hash(loc: ila_locator) -> u32 {
    let v = (&loc.v32 as *const _).cast::<u32>();
    __ila_hash_secret_init();
    jhash_2words(*v, *v.add(1), hashrnd)
}

#[inline]
unsafe fn ila_get_lock(ilan: *mut ila_net, loc: ila_locator) -> *mut spinlock_t {
    (*ilan).xlat.locks.add((ila_locator_hash(loc) & (*ilan).xlat.locks_mask) as usize)
}

#[inline]
unsafe fn ila_cmp_wildcards(ila: *mut ila_map, _iaddr: *mut ila_addr, ifindex: c_int) -> c_int {
    ((*ila).xp.ifindex != 0 && (*ila).xp.ifindex != ifindex) as c_int
}

#[inline]
unsafe fn ila_cmp_params(ila: *mut ila_map, xp: *mut ila_xlat_params) -> c_int {
    ((*ila).xp.ifindex != (*xp).ifindex) as c_int
}

unsafe fn ila_cmpfn(arg: *mut rhashtable_compare_arg, obj: *const core::ffi::c_void) -> c_int {
    let ila = obj as *const ila_map;
    ((*ila).xp.ip.locator_match.v64 != *( (*arg).key as *const __be64)) as c_int
}

#[inline]
unsafe fn ila_order(ila: *mut ila_map) -> c_int {
    if (*ila).xp.ifindex != 0 { 1 << 1 } else { 0 }
}

static rht_params: rhashtable_params = rhashtable_params {
    nelem_hint: 1024, head_offset: offset_of!(ila_map, node),
    key_offset: offset_of!(ila_map, xp.ip.locator_match), key_len: core::mem::size_of::<u64>(),
    max_size: 1048576, min_size: 256, automatic_shrinking: true, obj_cmpfn: Some(ila_cmpfn),
};

unsafe fn parse_nl_config(info: *mut genl_info, xp: *mut ila_xlat_params) -> c_int {
    core::ptr::write_bytes(xp, 0, 1);
    if !(*info).attrs[ILA_ATTR_LOCATOR].is_null() { (*xp).ip.locator.v64 = nla_get_u64((*info).attrs[ILA_ATTR_LOCATOR]) as __be64; }
    if !(*info).attrs[ILA_ATTR_LOCATOR_MATCH].is_null() { (*xp).ip.locator_match.v64 = nla_get_u64((*info).attrs[ILA_ATTR_LOCATOR_MATCH]) as __be64; }
    (*xp).ip.csum_mode = nla_get_u8_default((*info).attrs[ILA_ATTR_CSUM_MODE], ILA_CSUM_NO_ACTION);
    (*xp).ip.ident_type = nla_get_u8_default((*info).attrs[ILA_ATTR_IDENT_TYPE], ILA_ATYPE_USE_FORMAT);
    if !(*info).attrs[ILA_ATTR_IFINDEX].is_null() { (*xp).ifindex = nla_get_s32((*info).attrs[ILA_ATTR_IFINDEX]); }
    0
}

// Must be called with rcu readlock.
#[inline]
unsafe fn ila_lookup_wildcards(iaddr: *mut ila_addr, ifindex: c_int, ilan: *mut ila_net) -> *mut ila_map {
    let mut ila = rhashtable_lookup_fast(&(*ilan).xlat.rhash_table, &(*iaddr).loc, rht_params);
    while !ila.is_null() { if ila_cmp_wildcards(ila, iaddr, ifindex) == 0 { return ila; } ila = rcu_access_pointer((*ila).next); }
    core::ptr::null_mut()
}

// Must be called with rcu readlock.
#[inline]
unsafe fn ila_lookup_by_params(xp: *mut ila_xlat_params, ilan: *mut ila_net) -> *mut ila_map {
    let mut ila = rhashtable_lookup_fast(&(*ilan).xlat.rhash_table, &(*xp).ip.locator_match, rht_params);
    while !ila.is_null() { if ila_cmp_params(ila, xp) == 0 { return ila; } ila = rcu_access_pointer((*ila).next); }
    core::ptr::null_mut()
}

#[inline] unsafe fn ila_release(ila: *mut ila_map) { kfree_rcu(ila, rcu); }

unsafe fn ila_free_node(mut ila: *mut ila_map) { while !ila.is_null() { let next = rcu_access_pointer((*ila).next); ila_release(ila); ila = next; } }
unsafe extern "C" fn ila_free_cb(ptr: *mut c_void, _arg: *mut c_void) { ila_free_node(ptr as *mut ila_map); }

// Remaining declarations and definitions retain the C implementation's
// externally supplied kernel ABI and are intentionally represented directly.
extern "C" {
    fn ila_xlat_addr(skb: *mut sk_buff, sir2ila: bool) -> c_int;
    pub fn ila_xlat_nl_cmd_add_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn ila_xlat_nl_cmd_del_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn ila_xlat_nl_cmd_flush(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn ila_xlat_nl_cmd_get_mapping(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
    pub fn ila_xlat_nl_dump_start(cb: *mut netlink_callback) -> c_int;
    pub fn ila_xlat_nl_dump_done(cb: *mut netlink_callback) -> c_int;
    pub fn ila_xlat_nl_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
    pub fn ila_xlat_init_net(net: *mut net) -> c_int;
    pub fn ila_xlat_pre_exit_net(net: *mut net);
    pub fn ila_xlat_exit_net(net: *mut net);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
