// Translated from linux/mroute_base.h. External kernel types and functions are
// supplied by the surrounding translation unit.

#[repr(C)]
pub struct vif_device {
    pub dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub bytes_in: c_ulong,
    pub bytes_out: c_ulong,
    pub pkt_in: c_ulong,
    pub pkt_out: c_ulong,
    pub rate_limit: c_ulong,
    pub threshold: u8,
    pub flags: u16,
    pub link: c_int,
    pub dev_parent_id: netdev_phys_item_id,
    pub local: __be32,
    pub remote: __be32,
}

#[repr(C)]
pub struct vif_entry_notifier_info {
    pub info: fib_notifier_info,
    pub dev: *mut net_device,
    pub vif_index: u16,
    pub vif_flags: u16,
    pub tb_id: u32,
}

pub unsafe fn mr_call_vif_notifier(
    nb: *mut notifier_block, family: u16, event_type: fib_event_type,
    vif: *mut vif_device, vif_dev: *mut net_device, vif_index: u16,
    tb_id: u32, extack: *mut netlink_ext_ack,
) -> c_int {
    let info = vif_entry_notifier_info {
        info: fib_notifier_info { family, extack },
        dev: vif_dev,
        vif_index,
        vif_flags: (*vif).flags,
        tb_id,
    };
    call_fib_notifier(nb, event_type, &info.info)
}

pub unsafe fn mr_call_vif_notifiers(
    net: *mut net, family: u16, event_type: fib_event_type,
    vif: *mut vif_device, vif_dev: *mut net_device, vif_index: u16,
    tb_id: u32, ipmr_seq: *mut atomic_t,
) -> c_int {
    let info = vif_entry_notifier_info {
        info: fib_notifier_info { family, extack: core::ptr::null_mut() },
        dev: vif_dev,
        vif_index,
        vif_flags: (*vif).flags,
        tb_id,
    };
    ASSERT_RTNL();
    atomic_inc(ipmr_seq);
    call_fib_notifiers(net, event_type, &info.info)
}

pub const MAXVIFS: usize = 32;
pub unsafe fn VIF_EXISTS(_mrt: *mut mr_table, _idx: usize) -> bool {
    !rcu_access_pointer((*_mrt).vif_table[_idx].dev).is_null()
}

pub const MFC_STATIC: c_int = BIT(0);
pub const MFC_OFFLOAD: c_int = BIT(1);

#[repr(C)]
pub union mr_mfc_mfc_un {
    pub unres: mr_mfc_unres,
    pub res: mr_mfc_res,
}
#[repr(C)]
pub struct mr_mfc_unres { pub expires: c_ulong, pub unresolved: sk_buff_head }
#[repr(C)]
pub struct mr_mfc_res {
    pub last_assert: c_ulong,
    pub minvif: c_int,
    pub maxvif: c_int,
    pub bytes: atomic_long_t,
    pub pkt: atomic_long_t,
    pub wrong_if: atomic_long_t,
    pub lastuse: c_ulong,
    pub ttls: [u8; MAXVIFS],
    pub refcount: refcount_t,
}

#[repr(C)]
pub struct mr_mfc {
    pub mnode: rhlist_head,
    pub mfc_parent: u16,
    pub mfc_flags: c_int,
    pub mfc_un: mr_mfc_mfc_un,
    pub list: list_head,
    pub rcu: rcu_head,
    pub free: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

pub unsafe fn mr_cache_put(c: *mut mr_mfc) {
    if refcount_dec_and_test(&mut (*c).mfc_un.res.refcount) {
        call_rcu(&mut (*c).rcu, (*c).free);
    }
}
pub unsafe fn mr_cache_hold(c: *mut mr_mfc) { refcount_inc(&mut (*c).mfc_un.res.refcount); }

#[repr(C)]
pub struct mfc_entry_notifier_info { pub info: fib_notifier_info, pub mfc: *mut mr_mfc, pub tb_id: u32 }

pub unsafe fn mr_call_mfc_notifier(nb: *mut notifier_block, family: u16,
    event_type: fib_event_type, mfc: *mut mr_mfc, tb_id: u32,
    extack: *mut netlink_ext_ack) -> c_int {
    let info = mfc_entry_notifier_info { info: fib_notifier_info { family, extack }, mfc, tb_id };
    call_fib_notifier(nb, event_type, &info.info)
}

pub unsafe fn mr_call_mfc_notifiers(net: *mut net, family: u16,
    event_type: fib_event_type, mfc: *mut mr_mfc, tb_id: u32,
    ipmr_seq: *mut atomic_t) -> c_int {
    let info = mfc_entry_notifier_info { info: fib_notifier_info { family, extack: core::ptr::null_mut() }, mfc, tb_id };
    atomic_inc(ipmr_seq);
    call_fib_notifiers(net, event_type, &info.info)
}

pub struct mr_table;
#[repr(C)]
pub struct mr_table_ops { pub rht_params: *const rhashtable_params, pub cmparg_any: *mut c_void }

#[repr(C)]
pub struct mr_table {
    pub work: rcu_work, pub list: list_head, pub net: possible_net_t,
    pub ops: mr_table_ops, pub id: u32, pub mroute_sk: *mut sock,
    pub ipmr_expire_timer: timer_list, pub mfc_unres_queue: list_head,
    pub vif_table: [vif_device; MAXVIFS], pub mfc_hash: rhltable,
    pub mfc_cache_list: list_head, pub maxvif: c_int,
    pub cache_resolve_queue_len: u32, pub mroute_do_assert: bool,
    pub mroute_do_pim: bool, pub mroute_do_wrvifwhole: bool,
    pub mroute_reg_vif_num: c_int,
}

pub unsafe fn mr_can_free_table(net: *mut net) -> bool { !check_net(net) || !net_initialized(net) }

// CONFIG_IP_MROUTE_COMMON declarations and fallback inline stubs.
extern "C" {
    pub fn vif_device_init(v: *mut vif_device, dev: *mut net_device, rate_limit: c_ulong,
        threshold: u8, flags: u16, get_iflink_mask: u16);
    pub fn mr_table_free(mrt: *mut mr_table);
    pub fn mr_table_alloc(net: *mut net, id: u32, ops: *mut mr_table_ops,
        expire_func: Option<unsafe extern "C" fn(*mut timer_list)>,
        table_set: Option<unsafe extern "C" fn(*mut mr_table, *mut net)>) -> *mut mr_table;
    pub fn mr_mfc_find_parent(mrt: *mut mr_table, hasharg: *mut c_void, parent: c_int) -> *mut c_void;
    pub fn mr_mfc_find_any_parent(mrt: *mut mr_table, vifi: c_int) -> *mut c_void;
    pub fn mr_mfc_find_any(mrt: *mut mr_table, vifi: c_int, hasharg: *mut c_void) -> *mut c_void;
    pub fn mr_fill_mroute(mrt: *mut mr_table, skb: *mut sk_buff, c: *mut mr_mfc, rtm: *mut rtmsg) -> c_int;
    pub fn mr_table_dump(mrt: *mut mr_table, skb: *mut sk_buff, cb: *mut netlink_callback, fill: *mut c_void, lock: *mut spinlock_t, filter: *mut fib_dump_filter) -> c_int;
    pub fn mr_rtm_dumproute(skb: *mut sk_buff, cb: *mut netlink_callback, iter: *mut c_void, fill: *mut c_void, lock: *mut spinlock_t, filter: *mut fib_dump_filter) -> c_int;
    pub fn mr_dump(net: *mut net, nb: *mut notifier_block, family: u16, rules_dump: *mut c_void, mr_iter: *mut c_void, extack: *mut netlink_ext_ack) -> c_int;
}

pub unsafe fn mr_mfc_find(mrt: *mut mr_table, hasharg: *mut c_void) -> *mut c_void {
    mr_mfc_find_parent(mrt, hasharg, -1)
}

#[repr(C)] pub struct mr_vif_iter { pub p: seq_net_private, pub mrt: *mut mr_table, pub ct: c_int }
#[repr(C)] pub struct mr_mfc_iter { pub p: seq_net_private, pub mrt: *mut mr_table, pub cache: *mut list_head, pub lock: *mut spinlock_t }

// CONFIG_PROC_FS and CONFIG_IP_MROUTE_COMMON provide the sequence helpers
// declared by this header; their exact external signatures are preserved here.
extern "C" {
    pub fn mr_vif_seq_idx(net: *mut net, iter: *mut mr_vif_iter, pos: loff_t) -> *mut c_void;
    pub fn mr_vif_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void;
    pub fn mr_mfc_seq_idx(net: *mut net, it: *mut mr_mfc_iter, pos: loff_t) -> *mut c_void;
    pub fn mr_mfc_seq_next(seq: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
