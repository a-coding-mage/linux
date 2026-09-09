/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of linux/netfilter/ipset/ip_set.h. */

/* C header dependencies are supplied by the surrounding kernel translation. */

#[repr(u32)]
pub enum ip_set_feature {
    IPSET_TYPE_IP_FLAG = 0,
    IPSET_TYPE_IP = 1 << 0,
    IPSET_TYPE_PORT_FLAG = 1,
    IPSET_TYPE_PORT = 1 << 1,
    IPSET_TYPE_MAC_FLAG = 2,
    IPSET_TYPE_MAC = 1 << 2,
    IPSET_TYPE_IP2_FLAG = 3,
    IPSET_TYPE_IP2 = 1 << 3,
    IPSET_TYPE_NAME_FLAG = 4,
    IPSET_TYPE_NAME = 1 << 4,
    IPSET_TYPE_IFACE_FLAG = 5,
    IPSET_TYPE_IFACE = 1 << 5,
    IPSET_TYPE_MARK_FLAG = 6,
    IPSET_TYPE_MARK = 1 << 6,
    IPSET_TYPE_NOMATCH_FLAG = 7,
    IPSET_TYPE_NOMATCH = 1 << 7,
    IPSET_DUMP_LAST_FLAG = 8,
    IPSET_DUMP_LAST = 1 << 8,
}

#[repr(u32)]
pub enum ip_set_extension {
    IPSET_EXT_BIT_TIMEOUT = 0,
    IPSET_EXT_TIMEOUT = 1 << 0,
    IPSET_EXT_BIT_COUNTER = 1,
    IPSET_EXT_COUNTER = 1 << 1,
    IPSET_EXT_BIT_COMMENT = 2,
    IPSET_EXT_COMMENT = 1 << 2,
    IPSET_EXT_BIT_SKBINFO = 3,
    IPSET_EXT_SKBINFO = 1 << 3,
    IPSET_EXT_BIT_DESTROY = 7,
    IPSET_EXT_DESTROY = 1 << 7,
}

#[repr(u32)]
pub enum ip_set_ext_id { IPSET_EXT_ID_COUNTER = 0, IPSET_EXT_ID_TIMEOUT, IPSET_EXT_ID_SKBINFO, IPSET_EXT_ID_COMMENT, IPSET_EXT_ID_MAX }

#[repr(C)] pub struct ip_set { pub rwork: rcu_work, pub name: [c_char; IPSET_MAXNAMELEN], pub lock: spinlock_t, pub ref_: u32, pub ref_netlink: u32, pub type_: *mut ip_set_type, pub variant: *const ip_set_type_variant, pub family: u8, pub revision: u8, pub extensions: u8, pub flags: u8, pub timeout: u32, pub elements: u32, pub ext_size: atomic64_t, pub dsize: usize, pub offset: [usize; IPSET_EXT_ID_MAX as usize], pub data: *mut c_void }
#[repr(C)] pub struct ip_set_ext_type { pub destroy: Option<unsafe extern "C" fn(*mut ip_set, *mut c_void)>, pub type_: ip_set_extension, pub flag: ipset_cadt_flags, pub len: u8, pub align: u8 }
#[repr(C)] pub struct ip_set_counter { pub bytes: atomic64_t, pub packets: atomic64_t }
#[repr(C)] pub struct ip_set_comment_rcu { pub rcu: rcu_head, pub str_: [c_char; 0] }
#[repr(C)] pub struct ip_set_comment { pub c: *mut ip_set_comment_rcu }
#[repr(C)] pub struct ip_set_skbinfo { pub skbmark: u32, pub skbmarkmask: u32, pub skbprio: u32, pub skbqueue: u16, pub pad: u16 }
#[repr(C)] pub struct ip_set_ext { pub skbinfo: ip_set_skbinfo, pub packets: u64, pub bytes: u64, pub comment: *mut c_char, pub timeout: u32, pub packets_op: u8, pub bytes_op: u8, pub target: bool }

pub type ipset_adtfn = Option<unsafe extern "C" fn(*mut ip_set, *mut c_void, *const ip_set_ext, *mut ip_set_ext, u32) -> c_int>;
#[repr(C)] pub struct ip_set_adt_opt { pub family: u8, pub dim: u8, pub flags: u8, pub cmdflags: u32, pub ext: ip_set_ext }
#[repr(C)] pub struct ip_set_type_variant {
    pub kadt: Option<unsafe extern "C" fn(*mut ip_set, *const sk_buff, *const xt_action_param, ipset_adt, *mut ip_set_adt_opt) -> c_int>,
    pub uadt: Option<unsafe extern "C" fn(*mut ip_set, *mut *mut nlattr, ipset_adt, *mut u32, u32, bool) -> c_int>,
    pub adt: [ipset_adtfn; IPSET_ADT_MAX as usize],
    pub resize: Option<unsafe extern "C" fn(*mut ip_set, bool) -> c_int>, pub destroy: Option<unsafe extern "C" fn(*mut ip_set)>, pub flush: Option<unsafe extern "C" fn(*mut ip_set)>, pub expire: Option<unsafe extern "C" fn(*mut ip_set)>, pub head: Option<unsafe extern "C" fn(*mut ip_set, *mut sk_buff) -> c_int>, pub list: Option<unsafe extern "C" fn(*const ip_set, *mut sk_buff, *mut netlink_callback) -> c_int>, pub uref: Option<unsafe extern "C" fn(*mut ip_set, *mut netlink_callback, bool)>, pub same_set: Option<unsafe extern "C" fn(*const ip_set, *const ip_set) -> bool>, pub cancel_gc: Option<unsafe extern "C" fn(*mut ip_set)>, pub region_lock: bool,
}
#[repr(C)] pub struct ip_set_region { pub lock: spinlock_t, pub ext_size: usize, pub elements: u32 }
#[repr(C)] pub struct ip_set_type { pub list: list_head, pub name: [c_char; IPSET_MAXNAMELEN], pub protocol: u8, pub dimension: u8, pub family: u8, pub revision_min: u8, pub revision_max: u8, pub create_flags: [u8; (IPSET_REVISION_MAX + 1) as usize], pub features: u16, pub create: Option<unsafe extern "C" fn(*mut net, *mut ip_set, *mut *mut nlattr, u32) -> c_int>, pub create_policy: [nla_policy; (IPSET_ATTR_CREATE_MAX + 1) as usize], pub adt_policy: [nla_policy; (IPSET_ADT_MAX + 1) as usize], pub me: *mut module }

extern "C" { pub static ip_set_extensions: [ip_set_ext_type; IPSET_EXT_ID_MAX as usize]; pub fn ip_set_type_register(*mut ip_set_type) -> c_int; pub fn ip_set_type_unregister(*mut ip_set_type); }

pub const IPSET_MAX_RANGE: u32 = 1 << 14;
pub const IPSET_REVISION_MAX: u32 = 9;
pub const IPSET_GC_TIME: u32 = 3 * 60;
pub const IPSET_ELEM_PERMANENT: u32 = 0;
pub const IPSET_NO_TIMEOUT: u32 = u32::MAX;
pub const IPSET_MAX_TIMEOUT: u32 = (u32::MAX >> 1) / MSEC_PER_SEC;

#[inline] pub unsafe fn ip_set_ext_destroy(set: *mut ip_set, data: *mut c_void) { if ((*set).extensions as u32 & IPSET_EXT_COMMENT as u32) != 0 { let c = ext_comment(data, set); ((*ip_set_extensions.as_ptr().add(IPSET_EXT_ID_COMMENT as usize)).destroy.unwrap())(set, c as *mut c_void); } }
#[inline] pub unsafe fn ip_set_get_hostipaddr4(nla: *mut nlattr, ipaddr: *mut u32) -> c_int { let mut ip: __be32 = 0; let ret = ip_set_get_ipaddr4(nla, &mut ip); if ret != 0 { return ret; } *ipaddr = ntohl(ip); 0 }
#[inline] pub unsafe fn ip_set_eexist(ret: c_int, flags: u32) -> bool { ret == -IPSET_ERR_EXIST && flags & IPSET_FLAG_EXIST != 0 }
#[inline] pub unsafe fn ip_set_attr_netorder(tb: *mut *mut nlattr, ty: c_int) -> bool { !(*tb.add(ty as usize)).is_null() && ((*(*tb.add(ty as usize))).nla_type & NLA_F_NET_BYTEORDER) != 0 }
#[inline] pub unsafe fn ip_set_optattr_netorder(tb: *mut *mut nlattr, ty: c_int) -> bool { (*tb.add(ty as usize)).is_null() || ((*(*tb.add(ty as usize))).nla_type & NLA_F_NET_BYTEORDER) != 0 }

extern "C" { fn ip_set_get_ipaddr4(*mut nlattr, *mut __be32) -> c_int; fn ntohl(__be32) -> u32; fn time_is_before_jiffies(usize) -> bool; fn msecs_to_jiffies(u32) -> usize; static mut jiffies: usize; }

#[inline] pub unsafe fn ip_set_timeout_expired(t: *const usize) -> bool { *t != 0 && time_is_before_jiffies(*t) }
#[inline] pub unsafe fn ip_set_timeout_set(timeout: *mut usize, value: u32) { if value == 0 { *timeout = 0; return; } let mut t = msecs_to_jiffies(value * MSEC_PER_SEC) + jiffies; if t == 0 { t -= 1; } *timeout = t; }
#[inline] pub unsafe fn ip_set_init_counter(counter: *mut ip_set_counter, ext: *const ip_set_ext) { if (*ext).bytes != u64::MAX { atomic64_set(&mut (*counter).bytes, (*ext).bytes as i64); } if (*ext).packets != u64::MAX { atomic64_set(&mut (*counter).packets, (*ext).packets as i64); } }
#[inline] pub unsafe fn ip_set_init_skbinfo(skbinfo: *mut ip_set_skbinfo, ext: *const ip_set_ext) { *skbinfo = (*ext).skbinfo; }

/* External kernel types and constants are intentionally referenced, not reimplemented here. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
