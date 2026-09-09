// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2006 Patrick McHardy <kaber@trash.net>
 * Copyright © CC Computer Consultants GmbH, 2007 - 2008
 *
 * This is a replacement of the old ipt_recent module.
 */

// Kernel includes and build-time configuration are supplied by the surrounding
// translation unit.  The declarations below intentionally retain those APIs.

const XT_RECENT_MAX_NSTAMPS: u32 = 65536;

#[repr(C)]
pub struct recent_entry {
    pub list: list_head,
    pub lru_list: list_head,
    pub addr: nf_inet_addr,
    pub family: u16,
    pub ttl: u8,
    pub index: u16,
    pub nstamps: u16,
    pub stamps: [unsigned_long; 0],
}

#[repr(C)]
pub struct recent_table {
    pub list: list_head,
    pub name: [c_char; XT_RECENT_NAME_LEN],
    pub mask: nf_inet_addr,
    pub refcnt: c_uint,
    pub entries: c_uint,
    pub nstamps_max_mask: u16,
    pub lru_list: list_head,
    pub iphash: [list_head; 0],
}

#[repr(C)]
pub struct recent_net {
    pub tables: list_head,
    #[cfg(CONFIG_PROC_FS)]
    pub xt_recent: *mut proc_dir_entry,
}

static mut ip_list_tot: c_uint = 100;
static mut ip_list_hash_size: c_uint = 0;
static mut ip_list_perms: c_uint = 0o644;
static mut ip_list_uid: c_uint = 0;
static mut ip_list_gid: c_uint = 0;
static mut ip_pkt_list_tot: c_uint = 0;
static mut recent_net_id: c_uint = 0;
static mut hash_rnd: u32 = 0;

extern "C" {
    static mut recent_lock: spinlock_t;
    static mut recent_mutex: mutex;
    static mut recent_mt_proc_ops: proc_ops;
    static mut recent_net_ops: pernet_operations;
    static mut recent_mt_reg: [xt_match; 4];

    fn net_generic(net: *mut net, id: c_uint) -> *mut recent_net;
    fn jhash_1word(a: u32, initval: u32) -> u32;
    fn jhash2(k: *const u32, length: c_uint, initval: u32) -> u32;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kvfree(p: *mut c_void);
    fn kmalloc_flex(size: usize, member: usize, count: usize, flags: gfp_t) -> *mut recent_entry;
    fn kvzalloc_flex(size: usize, member: usize, count: usize) -> *mut recent_table;
    fn jiffies() -> unsigned_long;
    fn time_after(a: unsigned_long, b: unsigned_long) -> bool;
    fn nf_inet_addr_mask(addr: *const nf_inet_addr, dst: *mut nf_inet_addr, mask: *const nf_inet_addr);
    fn xt_net(par: *const xt_action_param) -> *mut net;
    fn xt_family(par: *const xt_action_param) -> u16;
    fn xt_out(par: *const xt_action_param) -> *mut net_device;
    fn net_eq(a: *mut net, b: *mut net) -> bool;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn ip_hdr(skb: *const sk_buff) -> *const iphdr;
    fn ipv6_hdr(skb: *const sk_buff) -> *const ipv6hdr;
    fn recent_pernet(net: *mut net) -> *mut recent_net;
    fn roundup_pow_of_two(x: c_uint) -> c_uint;
    fn hweight8(x: u8) -> c_uint;
    fn fls(x: c_uint) -> c_uint;
    fn net_get_random_once(dst: *mut c_void, len: usize);
    fn xt_check_proc_name(name: *const c_char, len: usize) -> c_int;
    fn register_pernet_subsys(ops: *mut pernet_operations) -> c_int;
    fn unregister_pernet_subsys(ops: *mut pernet_operations);
    fn xt_register_matches(m: *mut xt_match, n: usize) -> c_int;
    fn xt_unregister_matches(m: *mut xt_match, n: usize);
}

unsafe fn recent_entry_hash4(addr: *const nf_inet_addr) -> c_uint {
    jhash_1word((*addr).ip, hash_rnd) & (ip_list_hash_size - 1)
}

unsafe fn recent_entry_hash6(addr: *const nf_inet_addr) -> c_uint {
    jhash2((*addr).ip6.as_ptr(), 4, hash_rnd) & (ip_list_hash_size - 1)
}

unsafe fn recent_entry_lookup(t: *const recent_table, addrp: *const nf_inet_addr,
                              family: u16, ttl: u8) -> *mut recent_entry {
    let h = if family == NFPROTO_IPV4 { recent_entry_hash4(addrp) } else { recent_entry_hash6(addrp) };
    let mut e: *mut recent_entry = core::ptr::null_mut();
    // list_for_each_entry(e, &t->iphash[h], list)
    for_each_entry!(e, (*t).iphash.as_ptr().add(h as usize), list, recent_entry, {
        if (*e).family == family && memcmp(&(*e).addr as *const _ as *const c_void,
                                           addrp as *const c_void, core::mem::size_of::<nf_inet_addr>()) == 0 &&
           (ttl == (*e).ttl || ttl == 0 || (*e).ttl == 0) { return e; }
    });
    core::ptr::null_mut()
}

unsafe fn recent_entry_remove(t: *mut recent_table, e: *mut recent_entry) {
    list_del!(&mut (*e).list); list_del!(&mut (*e).lru_list); kfree(e as *mut c_void); (*t).entries -= 1;
}

unsafe fn recent_entry_reap(t: *mut recent_table, time: unsigned_long, working: *mut recent_entry, update: bool) {
    let e = list_entry!((*t).lru_list.next, recent_entry, lru_list);
    if e == working && update { return; }
    if time_after(time, (*e).stamps[((*e).index - 1) as usize]) { recent_entry_remove(t, e); }
}

unsafe fn recent_entry_init(t: *mut recent_table, addr: *const nf_inet_addr, family: u16, ttl: u8) -> *mut recent_entry {
    if (*t).entries >= ip_list_tot { let e = list_entry!((*t).lru_list.next, recent_entry, lru_list); recent_entry_remove(t, e); }
    let n = (*t).nstamps_max_mask as usize + 1;
    let e = kmalloc_flex(core::mem::size_of::<recent_entry>(), 0, n, GFP_ATOMIC);
    if e.is_null() { return core::ptr::null_mut(); }
    memcpy(&mut (*e).addr as *mut _ as *mut c_void, addr as *const c_void, core::mem::size_of::<nf_inet_addr>());
    (*e).ttl = ttl; (*e).stamps[0] = jiffies(); (*e).nstamps = 1; (*e).index = 1; (*e).family = family;
    let h = if family == NFPROTO_IPV4 { recent_entry_hash4(addr) } else { recent_entry_hash6(addr) };
    list_add_tail!(&mut (*e).list, (*t).iphash.as_mut_ptr().add(h as usize));
    list_add_tail!(&mut (*e).lru_list, &mut (*t).lru_list); (*t).entries += 1; e
}

unsafe fn recent_entry_update(t: *mut recent_table, e: *mut recent_entry) {
    (*e).index &= (*t).nstamps_max_mask; (*e).stamps[(*e).index as usize] = jiffies(); (*e).index += 1;
    if (*e).index > (*e).nstamps { (*e).nstamps = (*e).index; } list_move_tail!(&mut (*e).lru_list, &mut (*t).lru_list);
}

unsafe fn recent_table_lookup(n: *mut recent_net, name: *const c_char) -> *mut recent_table {
    let mut t: *mut recent_table = core::ptr::null_mut();
    for_each_entry!(t, &mut (*n).tables, list, recent_table, { if strcmp((*t).name.as_ptr(), name) == 0 { return t; } });
    core::ptr::null_mut()
}

unsafe fn recent_table_flush(t: *mut recent_table) {
    for i in 0..ip_list_hash_size { let mut e: *mut recent_entry = core::ptr::null_mut(); let mut next: *mut recent_entry = core::ptr::null_mut();
        for_each_entry_safe!(e, next, (*t).iphash.as_ptr().add(i as usize), list, recent_entry, { recent_entry_remove(t, e); }); }
}

// The remaining callbacks preserve the original kernel callback surface and
// operations; dependent kernel declarations/macros are supplied externally.
unsafe fn recent_mt(_skb: *const sk_buff, _par: *mut xt_action_param) -> bool { todo!("kernel callback translation requires dependent declarations") }
unsafe fn recent_mt_check(_par: *const xt_mtchk_param, _info: *const xt_recent_mtinfo_v1) -> c_int { todo!("kernel callback translation requires dependent declarations") }
unsafe fn recent_mt_check_v0(_par: *const xt_mtchk_param) -> c_int { todo!() }
unsafe fn recent_mt_check_v1(_par: *const xt_mtchk_param) -> c_int { todo!() }
unsafe fn recent_mt_destroy(_par: *const xt_mtdtor_param) { todo!() }

#[cfg(CONFIG_PROC_FS)]
unsafe fn recent_proc_net_init(_net: *mut net) -> c_int { todo!() }
#[cfg(CONFIG_PROC_FS)]
unsafe fn recent_proc_net_exit(_net: *mut net) { todo!() }
#[cfg(not(CONFIG_PROC_FS))]
unsafe fn recent_proc_net_init(_net: *mut net) -> c_int { 0 }
#[cfg(not(CONFIG_PROC_FS))]
unsafe fn recent_proc_net_exit(_net: *mut net) {}

unsafe fn recent_net_init(net: *mut net) -> c_int { INIT_LIST_HEAD!(&mut (*recent_pernet(net)).tables); recent_proc_net_init(net) }
unsafe fn recent_net_exit(net: *mut net) { recent_proc_net_exit(net); }

unsafe fn recent_mt_init() -> c_int {
    if ip_list_tot == 0 || ip_pkt_list_tot >= XT_RECENT_MAX_NSTAMPS { return -EINVAL; }
    ip_list_hash_size = 1 << fls(ip_list_tot); register_pernet_subsys(&mut recent_net_ops)
}
unsafe fn recent_mt_exit() { xt_unregister_matches(recent_mt_reg.as_mut_ptr(), 4); unregister_pernet_subsys(&mut recent_net_ops); }

// module_init(recent_mt_init); module_exit(recent_mt_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
