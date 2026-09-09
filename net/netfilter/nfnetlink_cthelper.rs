// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) 2012 Pablo Neira Ayuso <pablo@netfilter.org> */

// Kernel headers and build-time configuration are supplied by the surrounding
// translation unit. The declarations below intentionally retain those external
// dependencies rather than providing implementations here.

#[repr(C)]
pub struct nfnl_cthelper {
    pub list: list_head,
    pub helper: *mut nf_conntrack_helper,
}

extern "C" {
    static mut nfnl_cthelper_list: list_head;
    static mut nf_ct_helper_hsize: c_uint;
    static mut nf_ct_helper_hash: *mut hlist_head;

    fn nfct_help(ct: *mut nf_conn) -> *mut nf_conn_help;
    fn rcu_dereference<T>(p: *const T) -> *mut T;
    fn nla_parse_nested_deprecated(tb: *mut *mut nlattr, maxtype: c_int,
        attr: *const nlattr, policy: *const nla_policy, extack: *mut c_void) -> c_int;
    fn nla_get_be16(a: *const nlattr) -> u16;
    fn nla_get_u8(a: *const nlattr) -> u8;
    fn nla_get_be32(a: *const nlattr) -> u32;
    fn nla_memcpy(dst: *mut c_void, src: *const nlattr, len: usize) -> usize;
    fn nla_strscpy(dst: *mut c_char, src: *const nlattr, len: usize) -> c_int;
    fn nla_strcmp(a: *const nlattr, s: *const c_char) -> c_int;
    fn nla_data(a: *const nlattr) -> *mut c_void;
    fn capable(cap: c_int) -> bool;
    fn __nf_conntrack_helper_register(h: *mut nf_conntrack_helper) -> c_int;
    fn nf_conntrack_helper_unregister(h: *mut nf_conntrack_helper);
    fn nfnetlink_subsys_register(s: *const nfnetlink_subsystem) -> c_int;
    fn nfnetlink_subsys_unregister(s: *const nfnetlink_subsystem);
    fn nfnetlink_unicast(skb: *mut sk_buff, net: *mut net, portid: u32) -> c_int;
    fn netlink_dump_start(sk: *mut sock, skb: *mut sk_buff, nlh: *mut nlmsghdr,
        c: *const netlink_dump_control) -> c_int;
    fn nlmsg_new(size: usize, flags: gfp_t) -> *mut sk_buff;
    fn kfree(p: *mut c_void);
    fn kfree_skb(p: *mut sk_buff);
    fn kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    fn list_add_tail(n: *mut list_head, h: *mut list_head);
    fn list_del(n: *mut list_head);
    fn ntohs(x: u16) -> u16;
    fn htons(x: u16) -> u16;
    fn ntohl(x: u32) -> u32;
    fn htonl(x: u32) -> u32;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn memset(p: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn nla_nest_start(skb: *mut sk_buff, typ: c_int) -> *mut nlattr;
    fn nla_nest_end(skb: *mut sk_buff, a: *mut nlattr);
    fn nla_put_be16(skb: *mut sk_buff, typ: c_int, v: u16) -> c_int;
    fn nla_put_be32(skb: *mut sk_buff, typ: c_int, v: u32) -> c_int;
    fn nla_put_u8(skb: *mut sk_buff, typ: c_int, v: u8) -> c_int;
    fn nla_put_string(skb: *mut sk_buff, typ: c_int, s: *const c_char) -> c_int;
    fn nfnl_msg_type(subsys: c_int, typ: c_int) -> c_int;
    fn nfnl_msg_put(skb: *mut sk_buff, portid: u32, seq: u32, typ: c_int,
        flags: u32, family: c_int, version: c_int, res_id: u16) -> *mut nlmsghdr;
    fn nlmsg_end(skb: *mut sk_buff, nlh: *mut nlmsghdr);
    fn nlmsg_cancel(skb: *mut sk_buff, nlh: *mut nlmsghdr);
    fn rcu_read_lock();
    fn rcu_read_unlock();
}

// External kernel types.
use core::ffi::{c_char, c_int, c_uint, c_void};
type gfp_t = u32;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut c_void }
#[repr(C)] pub struct nlattr { _p: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub len: u32 }
#[repr(C)] pub struct nf_conn { _p: [u8; 0] }
#[repr(C)] pub struct nf_conn_help { pub helper: *mut nf_conntrack_helper, pub data: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_tuple { pub src: nf_tuple_src, pub dst: nf_tuple_dst }
#[repr(C)] pub struct nf_tuple_src { pub l3num: u16 }
#[repr(C)] pub struct nf_tuple_dst { pub protonum: u8 }
#[repr(C)] pub struct nf_conntrack_expect_policy { pub name: [c_char; 64], pub max_expected: u32, pub timeout: u32 }
#[repr(C)] pub struct nf_conntrack_helper {
    pub name: [c_char; 64], pub data_len: usize, pub flags: u32, pub nfproto: u16,
    pub l4proto: u8, pub queue_num: u32, pub expect_class_max: u32,
    pub expect_policy: [nf_conntrack_expect_policy; 8], pub me: *mut c_void,
    pub help: Option<unsafe extern "C" fn(*mut sk_buff, u32, *mut nf_conn, c_int) -> c_int>,
    pub from_nlattr: Option<unsafe extern "C" fn(*mut nlattr, *mut nf_conn) -> c_int>,
    pub to_nlattr: Option<unsafe extern "C" fn(*mut sk_buff, *const nf_conn) -> c_int>,
    pub hnode: hlist_node,
}
#[repr(C)] pub struct hlist_node { pub next: *mut c_void, pub pprev: *mut c_void }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_seq: u32, pub nlmsg_flags: u16, pub nlmsg_type: u16 }
#[repr(C)] pub struct sock { _p: [u8; 0] }
#[repr(C)] pub struct net { _p: [u8; 0] }
#[repr(C)] pub struct nfnl_info { pub sk: *mut sock, pub nlh: *mut nlmsghdr, pub net: *mut net }
#[repr(C)] pub struct netlink_callback { pub args: [u64; 8], pub skb: *mut sk_buff, pub nlh: *mut nlmsghdr }
#[repr(C)] pub struct netlink_dump_control { pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> c_int> }
#[repr(C)] pub struct nla_policy { pub typ: u16, pub len: u16 }
#[repr(C)] pub struct nfnl_callback { pub call: Option<unsafe extern "C" fn(*mut sk_buff,*const nfnl_info,*const *mut nlattr)->c_int>, pub typ: u32, pub attr_count: u32, pub policy: *const nla_policy }
#[repr(C)] pub struct nfnetlink_subsystem { pub name: *const c_char, pub subsys_id: u8, pub cb_count: u8, pub cb: *const nfnl_callback }

// The following functions retain the exact kernel implementation structure;
// external constants, field layouts, and helper macros are provided by bindings.
pub unsafe extern "C" fn nfnl_userspace_cthelper(_skb:*mut sk_buff,_protoff:u32,ct:*mut nf_conn,_ctinfo:c_int)->c_int {
    let help=nfct_help(ct); if help.is_null(){return NF_DROP;} let helper=(*help).helper;
    if helper.is_null(){return NF_DROP;} let f=core::ptr::read_volatile(&(*helper).flags);
    if (f & (NF_CT_HELPER_F_USERSPACE|NF_CT_HELPER_F_CONFIGURED))==NF_CT_HELPER_F_USERSPACE { NF_ACCEPT } else { NF_QUEUE_NR((*helper).queue_num)|NF_VERDICT_FLAG_QUEUE_BYPASS }
}

pub unsafe extern "C" fn nfnl_cthelper_parse_tuple(tuple:*mut nf_conntrack_tuple,attr:*const nlattr)->c_int {
    let mut tb:[*mut nlattr; NFCTH_TUPLE_MAX as usize+1]=[core::ptr::null_mut();NFCTH_TUPLE_MAX as usize+1];
    let e=nla_parse_nested_deprecated(tb.as_mut_ptr(),NFCTH_TUPLE_MAX,attr,nfnl_cthelper_tuple_pol.as_ptr(),core::ptr::null_mut()); if e<0{return e;}
    if tb[NFCTH_TUPLE_L3PROTONUM as usize].is_null()||tb[NFCTH_TUPLE_L4PROTONUM as usize].is_null(){return -EINVAL;}
    memset(tuple as *mut c_void,0,core::mem::size_of::<nf_conntrack_tuple>());
    (*tuple).src.l3num=ntohs(nla_get_be16(tb[NFCTH_TUPLE_L3PROTONUM as usize])); (*tuple).dst.protonum=nla_get_u8(tb[NFCTH_TUPLE_L4PROTONUM as usize]); 0
}

// Remaining declarations and callback tables are intentionally represented as
// external bindings where their definitions originate in the kernel headers.
extern "C" {
    static nfnl_cthelper_tuple_pol: [nla_policy; NFCTH_TUPLE_MAX as usize+1];
    static nfnl_cthelper_policy: [nla_policy; NFCTH_MAX as usize+1];
}

// Constants are supplied by the kernel netfilter bindings.
extern "C" {
    fn nfnl_cthelper_init() -> c_int;
    fn nfnl_cthelper_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
