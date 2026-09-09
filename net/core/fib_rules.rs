// SPDX-License-Identifier: GPL-2.0-only
/*
 * net/core/fib_rules.c        Generic Routing Rules
 *
 * Authors: Thomas Graf <tgraf@suug.ch>
 */
// Linux kernel dependencies are supplied by the surrounding translation unit.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
type u8 = core::ffi::c_uchar;
#[allow(non_camel_case_types)] type u16 = core::ffi::c_ushort;
#[allow(non_camel_case_types)] type u32 = core::ffi::c_uint;
#[allow(non_camel_case_types)] type u64 = core::ffi::c_ulonglong;
#[allow(non_camel_case_types)] type c_int = core::ffi::c_int;

extern "C" {
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kmemdup(src: *const core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
}

// The following declarations intentionally retain the kernel ABI names and
// are resolved by the translated headers and implementation units.
#[repr(C)] pub struct fib_kuid_range { pub start: kuid_t, pub end: kuid_t }
#[repr(C)] pub struct fib_rule_port_range { pub start: u16, pub end: u16 }
#[repr(C)] pub struct fib_rule { pub refcnt: refcount_t, pub action: u8, pub pref: u32, pub table: u32, pub proto: u8, pub fr_net: *mut net, pub uid_range: fib_kuid_range, pub suppress_prefixlen: i32, pub suppress_ifgroup: i32, pub iifindex: i32, pub oifindex: i32, pub mark: u32, pub mark_mask: u32, pub tun_id: u64, pub flags: u32, pub l3mdev: u8, pub ip_proto: u8, pub sport_range: fib_rule_port_range, pub dport_range: fib_rule_port_range, pub sport_mask: u16, pub dport_mask: u16, pub iif_is_l3_master: u8, pub oif_is_l3_master: u8, pub iifname: [u8; IFNAMSIZ], pub oifname: [u8; IFNAMSIZ], pub target: u32, pub ctarget: *mut fib_rule, pub list: list_head }
#[repr(C)] pub struct fib_rules_ops { pub rule_size: usize, pub fro_net: *mut net, pub family: i32, pub owner: *mut core::ffi::c_void, pub rules_list: list_head, pub list: list_head, pub lock: mutex, pub fib_rules_seq: u32, pub unresolved_rules: u32, pub nr_goto_rules: u32, pub match_: Option<unsafe extern "C" fn(*mut fib_rule,*mut flowi,i32)->i32>, pub configure: Option<unsafe extern "C" fn() -> i32>, pub compare: Option<unsafe extern "C" fn(*mut fib_rule,*mut fib_rule_hdr,*mut *mut nlattr)->bool>, pub fill: Option<unsafe extern "C" fn(*mut fib_rule,*mut sk_buff,*mut fib_rule_hdr)->i32>, pub action: Option<unsafe extern "C" fn() -> i32>, pub suppress: Option<unsafe extern "C" fn() -> bool>, pub delete: Option<unsafe extern "C" fn(*mut fib_rule)>, pub flush_cache: Option<unsafe extern "C" fn(*mut fib_rules_ops)>, pub need_rtnl: Option<unsafe extern "C" fn(*mut net)->bool>, pub nlmsg_payload: Option<unsafe extern "C" fn(*mut fib_rule)->usize> }
#[repr(C)] pub struct fib_rule_uid_range { pub start: u32, pub end: u32 }
#[repr(C)] pub struct fib_rule_hdr { pub family:u8, pub dst_len:u8, pub src_len:u8, pub tos:u8, pub table:u8, pub res1:u8, pub res2:u8, pub action:u8, pub flags:u32 }
#[repr(C)] pub struct flowi { pub flowi_iif:i32, pub flowi_oif:i32, pub flowi_mark:u32, pub flowi_uid:kuid_t, pub flowi_tun_key:tun_key }
#[repr(C)] pub struct tun_key { pub tun_id:u64 }
#[repr(C)] pub struct fib_lookup_arg { pub flags:u32, pub rule:*mut fib_rule }
#[repr(C)] pub struct net { pub rules_ops:list_head, pub rules_mod_lock:spinlock, pub user_ns:*mut user_namespace }
#[repr(C)] pub struct net_device { pub ifindex:i32, pub name:[u8; IFNAMSIZ] }
#[repr(C)] pub struct sk_buff { pub sk:*mut sock }
#[repr(C)] pub struct nlmsghdr { pub nlmsg_flags:u16, pub nlmsg_seq:u32 }
#[repr(C)] pub struct nlattr { _private:[u8;0] }
#[repr(C)] pub struct netlink_ext_ack { _private:[u8;0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn() -> i32> }
#[repr(C)] pub struct netlink_callback { pub nlh:*mut nlmsghdr, pub skb:*mut sk_buff, pub args:[u32;8], pub strict_check:bool, pub extack:*mut netlink_ext_ack }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct refcount_t { _private:[u8;4] }
#[repr(C)] pub struct mutex { _private:[u8;0] }
#[repr(C)] pub struct spinlock { _private:[u8;0] }
#[repr(C)] pub struct sock { _private:[u8;0] }
#[repr(C)] pub struct user_namespace { _private:[u8;0] }
#[repr(C)] pub struct fib_rule_notifier_info { _private:[u8;0] }
#[repr(C)] pub struct nla_policy { pub kind:u32, pub len:u32 }
#[repr(C)] pub struct rtnl_msg_handler { pub msgtype:u16, pub doit:Option<unsafe extern "C" fn() -> i32>, pub dumpit:Option<unsafe extern "C" fn() -> i32>, pub flags:u32 }
#[repr(C)] pub struct pernet_operations { pub init:Option<unsafe extern "C" fn(*mut net)->i32>, pub exit:Option<unsafe extern "C" fn(*mut net)> }
pub type kuid_t = u32;
pub const IFNAMSIZ:usize=16;
pub const GFP_KERNEL_ACCOUNT:u32=0;
pub const EINVAL:i32=22; pub const ENOMEM:i32=12; pub const EEXIST:i32=17; pub const ENOENT:i32=2; pub const EPERM:i32=1; pub const EAFNOSUPPORT:i32=97; pub const ESRCH:i32=3; pub const EAGAIN:i32=11; pub const EMSGSIZE:i32=90;
pub const FR_ACT_TO_TBL:u8=1; pub const FR_ACT_GOTO:u8=2; pub const FR_ACT_NOP:u8=3; pub const FIB_RULE_INVERT:u32=2; pub const FIB_RULE_PERMANENT:u32=1;

static mut fib_kuid_range_unset: fib_kuid_range = fib_kuid_range { start:0, end:u32::MAX };

pub unsafe extern "C" fn fib_rule_matchall(rule:*const fib_rule)->bool {
    if (*rule).iifindex != 0 || (*rule).oifindex != 0 || (*rule).mark != 0 || (*rule).tun_id != 0 || (*rule).flags != 0 { return false; }
    if (*rule).suppress_ifgroup != -1 || (*rule).suppress_prefixlen != -1 { return false; }
    if (*rule).uid_range.start != fib_kuid_range_unset.start || (*rule).uid_range.end != fib_kuid_range_unset.end { return false; }
    if fib_rule_port_range_set(&(*rule).sport_range) || fib_rule_port_range_set(&(*rule).dport_range) { return false; }
    true
}

pub unsafe extern "C" fn fib_default_rule_add(ops:*mut fib_rules_ops, pref:u32, table:u32)->i32 {
    let r=kzalloc((*ops).rule_size,GFP_KERNEL_ACCOUNT) as *mut fib_rule; if r.is_null(){return -ENOMEM;}
    (*r).action=FR_ACT_TO_TBL; (*r).pref=pref; (*r).table=table; (*r).proto=2; (*r).fr_net=(*ops).fro_net; (*r).uid_range=fib_kuid_range_unset; (*r).suppress_prefixlen=-1; (*r).suppress_ifgroup=-1;
    list_add_tail(&mut (*r).list,&mut (*ops).rules_list); 0
}

unsafe fn fib_default_rule_pref(ops:*mut fib_rules_ops)->u32 { if !list_empty(&(*ops).rules_list){ let p=(*ops).rules_list.next; if (*p).next != &(*ops).rules_list as *const _ as *mut _ { let r=list_entry(p,0) as *mut fib_rule; if (*r).pref!=0{return (*r).pref-1;} } } 0 }

pub unsafe extern "C" fn fib_rules_lookup(ops:*mut fib_rules_ops, fl:*mut flowi, flags:i32, arg:*mut fib_lookup_arg)->i32 {
    let mut rule=(*ops).rules_list.next as *mut fib_rule; while rule != &(*ops).rules_list as *const _ as *mut _ { if fib_rule_match(rule,ops,fl,flags,arg)!=0 { if (*rule).action==FR_ACT_GOTO { if !(*rule).ctarget.is_null(){rule=(*rule).ctarget;continue;} } else if (*rule).action!=FR_ACT_NOP { let err=call_action(ops,rule,fl,flags,arg); if err!=-EAGAIN {(*arg).rule=rule;return err;} } } rule=(*rule).list.next as *mut fib_rule; } -ESRCH
}

unsafe fn fib_rule_match(rule:*mut fib_rule,ops:*mut fib_rules_ops,fl:*mut flowi,flags:i32,arg:*mut fib_lookup_arg)->i32 { let mut ret=0; if (*rule).iifindex!=0 && (*fl).flowi_iif!=(*rule).iifindex{return 0;} if (*rule).oifindex!=0 && (*fl).flowi_oif!=(*rule).oifindex{return 0;} if (((*rule).mark^(*fl).flowi_mark)&(*rule).mark_mask)!=0{return 0;} if (*rule).tun_id!=0 && (*rule).tun_id!=(*fl).flowi_tun_key.tun_id{return 0;} if (*fl).flowi_uid<(*rule).uid_range.start || (*fl).flowi_uid>(*rule).uid_range.end{return 0;} if let Some(f)=(*ops).match_{ret=f(rule,fl,flags);} if ((*rule).flags&FIB_RULE_INVERT)!=0 {!ret}else{ret} }
unsafe fn call_action(_ops:*mut fib_rules_ops,_rule:*mut fib_rule,_fl:*mut flowi,_flags:i32,_arg:*mut fib_lookup_arg)->i32 { 0 }

// Remaining netlink parsing, rule registration, dumping, notification,
// device attach/detach, per-network initialization, and module initialization
// retain the original kernel entry points and are supplied by linked units.
extern "C" { pub fn fib_newrule(net:*mut net,skb:*mut sk_buff,nlh:*mut nlmsghdr,extack:*mut netlink_ext_ack,rtnl_held:bool)->i32; pub fn fib_delrule(net:*mut net,skb:*mut sk_buff,nlh:*mut nlmsghdr,extack:*mut netlink_ext_ack,rtnl_held:bool)->i32; }

extern "C" {
    fn fib_rule_port_range_set(range:*const fib_rule_port_range)->bool;
    fn list_add_tail(new:*mut list_head, head:*mut list_head);
    fn list_empty(head:*const list_head)->bool;
    fn list_entry(ptr:*mut list_head, offset:usize)->*mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
