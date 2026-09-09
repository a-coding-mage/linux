/* SPDX-License-Identifier: GPL-2.0 */
// Translated from net/neighbour.h. Linux dependencies are supplied externally.

pub const NUD_IN_TIMER: u32 = NUD_INCOMPLETE | NUD_REACHABLE | NUD_DELAY | NUD_PROBE;
pub const NUD_VALID: u32 = NUD_PERMANENT | NUD_NOARP | NUD_REACHABLE | NUD_PROBE | NUD_STALE | NUD_DELAY;
pub const NUD_CONNECTED: u32 = NUD_PERMANENT | NUD_NOARP | NUD_REACHABLE;

pub const NEIGH_VAR_MCAST_PROBES: usize = 0;
pub const NEIGH_VAR_UCAST_PROBES: usize = 1;
pub const NEIGH_VAR_APP_PROBES: usize = 2;
pub const NEIGH_VAR_MCAST_REPROBES: usize = 3;
pub const NEIGH_VAR_RETRANS_TIME: usize = 4;
pub const NEIGH_VAR_BASE_REACHABLE_TIME: usize = 5;
pub const NEIGH_VAR_DELAY_PROBE_TIME: usize = 6;
pub const NEIGH_VAR_INTERVAL_PROBE_TIME_MS: usize = 7;
pub const NEIGH_VAR_GC_STALETIME: usize = 8;
pub const NEIGH_VAR_QUEUE_LEN_BYTES: usize = 9;
pub const NEIGH_VAR_PROXY_QLEN: usize = 10;
pub const NEIGH_VAR_ANYCAST_DELAY: usize = 11;
pub const NEIGH_VAR_PROXY_DELAY: usize = 12;
pub const NEIGH_VAR_LOCKTIME: usize = 13;
pub const NEIGH_VAR_DATA_MAX: usize = NEIGH_VAR_LOCKTIME + 1;
pub const NEIGH_VAR_QUEUE_LEN: usize = 14;
pub const NEIGH_VAR_RETRANS_TIME_MS: usize = 15;
pub const NEIGH_VAR_BASE_REACHABLE_TIME_MS: usize = 16;
pub const NEIGH_VAR_GC_INTERVAL: usize = 17;
pub const NEIGH_VAR_GC_THRESH1: usize = 18;
pub const NEIGH_VAR_GC_THRESH2: usize = 19;
pub const NEIGH_VAR_GC_THRESH3: usize = 20;
pub const NEIGH_VAR_MAX: usize = 21;

#[repr(C)]
pub struct neigh_parms {
    pub net: possible_net_t, pub dev: *mut net_device, pub dev_tracker: netdevice_tracker,
    pub list: list_head, pub neigh_setup: Option<unsafe extern "C" fn(*mut neighbour) -> c_int>,
    pub tbl: *mut neigh_table, pub sysctl_table: *mut c_void, pub dead: c_int,
    pub refcnt: refcount_t, pub rcu_head: rcu_head, pub reachable_time: c_int,
    pub qlen: u32, pub data: [c_int; NEIGH_VAR_DATA_MAX], pub data_state: [c_ulong; 1],
}

#[inline] pub unsafe fn neigh_var_set(p: *mut neigh_parms, index: c_int, val: c_int) { set_bit(index, (*p).data_state.as_mut_ptr()); WRITE_ONCE((*p).data[index as usize], val); }
#[inline] pub unsafe fn neigh_parms_data_state_setall(p: *mut neigh_parms) { bitmap_fill((*p).data_state.as_mut_ptr(), NEIGH_VAR_DATA_MAX); }
#[inline] pub unsafe fn neigh_parms_data_state_cleanall(p: *mut neigh_parms) { bitmap_zero((*p).data_state.as_mut_ptr(), NEIGH_VAR_DATA_MAX); }

#[repr(C)] pub struct neigh_statistics { pub allocs: c_ulong, pub destroys: c_ulong, pub hash_grows: c_ulong, pub res_failed: c_ulong, pub lookups: c_ulong, pub hits: c_ulong, pub rcv_probes_mcast: c_ulong, pub rcv_probes_ucast: c_ulong, pub periodic_gc_runs: c_ulong, pub forced_gc_runs: c_ulong, pub unres_discards: c_ulong, pub table_fulls: c_ulong }

#[repr(C)]
pub struct neighbour {
    pub hash: hlist_node, pub dev_list: hlist_node, pub tbl: *mut neigh_table, pub parms: *mut neigh_parms,
    pub confirmed: c_ulong, pub updated: c_ulong, pub lock: rwlock_t, pub refcnt: refcount_t,
    pub arp_queue_len_bytes: c_uint, pub arp_queue: sk_buff_head, pub timer: timer_list, pub used: c_ulong,
    pub probes: atomic_t, pub nud_state: u8, pub type_: u8, pub dead: u8, pub protocol: u8, pub flags: u32,
    pub ha_lock: seqlock_t, pub ha: [u8; ALIGN(MAX_ADDR_LEN, core::mem::size_of::<c_ulong>())], pub hh: hh_cache,
    pub output: Option<unsafe extern "C" fn(*mut neighbour, *mut sk_buff) -> c_int>, pub ops: *const neigh_ops,
    pub gc_list: list_head, pub managed_list: list_head, pub rcu: rcu_head, pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker, pub primary_key: [u8; 0],
}

#[repr(C)] pub struct neigh_ops { pub family: c_int, pub solicit: Option<unsafe extern "C" fn(*mut neighbour,*mut sk_buff)>, pub error_report: Option<unsafe extern "C" fn(*mut neighbour,*mut sk_buff)>, pub output: Option<unsafe extern "C" fn(*mut neighbour,*mut sk_buff)->c_int>, pub connected_output: Option<unsafe extern "C" fn(*mut neighbour,*mut sk_buff)->c_int> }
#[repr(C)] pub struct pneigh_entry { pub next: *mut pneigh_entry, pub net: possible_net_t, pub dev: *mut net_device, pub dev_tracker: netdevice_tracker, pub free_node: list_head, pub flags: u32, pub protocol: u8, pub permanent: bool, pub key: [u8; 0] }
#[repr(C)] pub struct neigh_hash_table { pub hash_heads: *mut hlist_head, pub hash_shift: c_uint, pub hash_rnd: [u32; NEIGH_NUM_HASH_RND], pub rcu: rcu_head }
pub const NEIGH_NUM_HASH_RND: usize = 4;

#[repr(C)] pub struct neigh_table {
    pub family: c_int, pub entry_size: c_uint, pub key_len: c_uint, pub protocol: __be16,
    pub hash: Option<unsafe extern "C" fn(*const c_void,*const net_device,*mut u32)->u32>,
    pub key_eq: Option<unsafe extern "C" fn(*const neighbour,*const c_void)->bool>,
    pub constructor: Option<unsafe extern "C" fn(*mut neighbour)->c_int>, pub pconstructor: Option<unsafe extern "C" fn(*mut pneigh_entry)->c_int>, pub pdestructor: Option<unsafe extern "C" fn(*mut pneigh_entry)>, pub proxy_redo: Option<unsafe extern "C" fn(*mut sk_buff)>, pub is_multicast: Option<unsafe extern "C" fn(*const c_void)->c_int>, pub allow_add: Option<unsafe extern "C" fn(*const net_device,*mut netlink_ext_ack)->bool>,
    pub id: *mut c_char, pub parms: neigh_parms, pub parms_list: list_head, pub gc_interval: c_int, pub gc_thresh1: c_int, pub gc_thresh2: c_int, pub gc_thresh3: c_int, pub last_flush: c_ulong, pub gc_work: delayed_work, pub managed_work: delayed_work, pub proxy_timer: timer_list, pub proxy_queue: sk_buff_head, pub entries: atomic_t, pub gc_entries: atomic_t, pub gc_list: list_head, pub managed_list: list_head, pub lock: spinlock_t, pub last_rand: c_ulong, pub stats: *mut neigh_statistics, pub nht: *mut neigh_hash_table, pub phash_lock: mutex, pub phash_buckets: *mut *mut pneigh_entry,
}

#[inline] pub unsafe fn neigh_parms_family(p: *const neigh_parms) -> c_int { (*(*p).tbl).family }
pub const NEIGH_PRIV_ALIGN: usize = core::mem::size_of::<c_longlong>();
pub const fn neigh_entry_size(size: usize) -> usize { (size + NEIGH_PRIV_ALIGN - 1) & !(NEIGH_PRIV_ALIGN - 1) }
#[inline] pub unsafe fn neighbour_priv(n: *const neighbour) -> *mut c_void { (n as *const u8).add((*(*n).tbl).entry_size as usize) as *mut c_void }

pub const NEIGH_UPDATE_F_OVERRIDE: u32 = BIT(0); pub const NEIGH_UPDATE_F_WEAK_OVERRIDE: u32 = BIT(1); pub const NEIGH_UPDATE_F_OVERRIDE_ISROUTER: u32 = BIT(2); pub const NEIGH_UPDATE_F_USE: u32 = BIT(3); pub const NEIGH_UPDATE_F_MANAGED: u32 = BIT(4); pub const NEIGH_UPDATE_F_EXT_LEARNED: u32 = BIT(5); pub const NEIGH_UPDATE_F_ISROUTER: u32 = BIT(6); pub const NEIGH_UPDATE_F_ADMIN: u32 = BIT(7); pub const NEIGH_UPDATE_F_EXT_VALIDATED: u32 = BIT(8);
pub const NTF_OLD_MASK: u32 = 0xff; pub const NTF_EXT_SHIFT: u32 = 8; pub const NTF_EXT_MASK: u32 = NTF_EXT_MANAGED | NTF_EXT_EXT_VALIDATED; pub const NTF_MANAGED: u32 = NTF_EXT_MANAGED << NTF_EXT_SHIFT; pub const NTF_EXT_VALIDATED: u32 = NTF_EXT_EXT_VALIDATED << NTF_EXT_SHIFT;

#[inline] pub unsafe fn neigh_key_eq32(n: *const neighbour, pkey: *const c_void) -> bool { *( (*n).primary_key.as_ptr() as *const u32) == *(pkey as *const u32) }
#[inline] pub unsafe fn neigh_key_eq128(n: *const neighbour, pkey: *const c_void) -> bool { let a=(*n).primary_key.as_ptr() as *const u32; let b=pkey as *const u32; ((*a ^ *b)|(*a.add(1)^*b.add(1))|(*a.add(2)^*b.add(2))|(*a.add(3)^*b.add(3))) == 0 }

extern "C" { pub static nda_policy: nla_policy; }
// The remaining declarations and inline functions retain their C ABI and depend on Linux kernel definitions.
extern "C" {
    pub fn neigh_table_init(index: c_int, tbl: *mut neigh_table); pub fn neigh_table_clear(index: c_int, tbl: *mut neigh_table) -> c_int;
    pub fn neigh_lookup(tbl: *mut neigh_table, pkey: *const c_void, dev: *mut net_device) -> *mut neighbour;
    pub fn neigh_destroy(neigh: *mut neighbour); pub fn neigh_update(neigh: *mut neighbour,lladdr:*const u8,new_:u8,flags:u32,nlmsg_pid:u32)->c_int;
    pub fn neigh_resolve_output(neigh:*mut neighbour,skb:*mut sk_buff)->c_int; pub fn neigh_connected_output(neigh:*mut neighbour,skb:*mut sk_buff)->c_int; pub fn neigh_direct_output(neigh:*mut neighbour,skb:*mut sk_buff)->c_int;
    pub fn __neigh_create(tbl:*mut neigh_table,pkey:*const c_void,dev:*mut net_device,want_ref:bool)->*mut neighbour;
    pub fn neigh_release(neigh:*mut neighbour); pub fn neigh_event_send(neigh:*mut neighbour,skb:*mut sk_buff)->c_int;
    pub fn neigh_changeaddr(tbl:*mut neigh_table,dev:*mut net_device); pub fn neigh_ifdown(tbl:*mut neigh_table,dev:*mut net_device)->c_int; pub fn neigh_carrier_down(tbl:*mut neigh_table,dev:*mut net_device)->c_int;
    pub fn neigh_event_ns(tbl:*mut neigh_table,lladdr:*mut u8,saddr:*mut c_void,dev:*mut net_device)->*mut neighbour;
    pub fn neigh_parms_alloc(dev:*mut net_device,tbl:*mut neigh_table)->*mut neigh_parms; pub fn neigh_parms_release(tbl:*mut neigh_table,parms:*mut neigh_parms);
    pub fn neigh_rand_reach_time(base:c_ulong)->c_ulong; pub fn pneigh_enqueue(tbl:*mut neigh_table,p:*mut neigh_parms,skb:*mut sk_buff);
    pub fn pneigh_lookup(tbl:*mut neigh_table,net:*mut net,key:*const c_void,dev:*mut net_device)->*mut pneigh_entry; pub fn pneigh_create(tbl:*mut neigh_table,net:*mut net,key:*const c_void,dev:*mut net_device,flags:u32,protocol:u8,permanent:bool)->c_int; pub fn pneigh_delete(tbl:*mut neigh_table,net:*mut net,key:*const c_void,dev:*mut net_device)->c_int;
    pub fn neigh_app_ns(n:*mut neighbour); pub fn neigh_xmit(fam:c_int,dev:*mut net_device,pkey:*const c_void,skb:*mut sk_buff)->c_int;
    pub fn neigh_seq_start(seq:*mut seq_file,pos:*mut loff_t,tbl:*mut neigh_table,flags:c_uint)->*mut c_void; pub fn neigh_seq_next(seq:*mut seq_file,v:*mut c_void,pos:*mut loff_t)->*mut c_void; pub fn neigh_seq_stop(seq:*mut seq_file,v:*mut c_void);
    pub fn neigh_sysctl_register(dev:*mut net_device,p:*mut neigh_parms,handler:proc_handler)->c_int; pub fn neigh_sysctl_unregister(p:*mut neigh_parms);
}

#[repr(C)] pub struct neigh_seq_state { pub p: seq_net_private, pub tbl:*mut neigh_table, pub nht:*mut neigh_hash_table, pub neigh_sub_iter: Option<unsafe extern "C" fn(*mut neigh_seq_state,*mut neighbour,*mut loff_t)->*mut c_void>, pub bucket:c_uint, pub flags:c_uint }
pub const NEIGH_SEQ_NEIGH_ONLY:u32=1; pub const NEIGH_SEQ_IS_PNEIGH:u32=2; pub const NEIGH_SEQ_SKIP_NOARP:u32=4;
#[repr(C)] pub struct neighbour_cb { pub sched_next:c_ulong, pub flags:c_uint }
pub const LOCALLY_ENQUEUED:u32=1;

// Kernel type aliases and constants referenced above are intentionally unresolved here;
// they are provided by the translated dependency headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
