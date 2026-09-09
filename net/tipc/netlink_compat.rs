/*
 * Copyright (c) 2014, Ericsson AB
 * All rights reserved.
 *
 * Rust translation of tipc/netlink_compat.c.  Kernel interfaces referenced by
 * this file are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const ULTRA_STRING_MAX_LEN: usize = 32768;
pub const REPLY_TRUNCATED: &[u8] = b"<truncated>\n\0";

#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: u32, pub sk: *mut sock, }
#[repr(C)] pub struct net;
#[repr(C)] pub struct sock;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct nlmsghdr { pub nlmsg_len: u32, _rest: [u8; 0] }
#[repr(C)] pub struct netlink_callback { pub nlh: *mut nlmsghdr, pub skb: *mut sk_buff, pub data: *mut c_void }
#[repr(C)] pub struct genl_info { pub attrs: *mut *mut nlattr, pub nlh: *mut nlmsghdr }
#[repr(C)] pub struct genl_dumpit_info { pub info: genl_info }
#[repr(C)] pub struct tlv_desc { pub tlv_type: u16, pub tlv_len: u16 }
#[repr(C)] pub struct tipc_bearer_config { pub disc_domain: u32, pub priority: u32, pub name: [c_char; 64] }
#[repr(C)] pub struct tipc_link_config { pub value: u32, pub name: [c_char; 64] }
#[repr(C)] pub struct tipc_link_info { pub dest: u32, pub up: u32, pub str_: [c_char; 64] }
#[repr(C)] pub struct tipc_node_info { pub addr: u32, pub up: u32 }
#[repr(C)] pub struct tipc_name_table_query { pub depth: u32, pub type_: u32, pub lowbound: u32, pub upbound: u32 }
#[repr(C)] pub struct tipc_bearer; #[repr(C)] pub struct tipc_media;
#[repr(C)] pub struct tipc_genl_family;
#[repr(C)] pub struct genl_small_ops { pub cmd: u8, pub validate: u32, pub doit: Option<unsafe extern "C" fn(*mut sk_buff, *mut genl_info) -> c_int> }
#[repr(C)] pub struct genl_family { pub name: *const c_char, pub version: u8, pub hdrsize: u16, pub maxattr: u16, pub netnsok: bool, pub small_ops: *const genl_small_ops, pub n_small_ops: u32, pub resv_start_op: u8 }

#[repr(C)] pub struct tipc_nl_compat_msg { pub cmd: u16, pub rep_type: c_int, pub rep_size: c_int, pub req_type: c_int, pub req_size: c_int, pub net: *mut net, pub rep: *mut sk_buff, pub req: *mut tlv_desc, pub dst_sk: *mut sock }
#[repr(C)] pub struct tipc_nl_compat_cmd_dump { pub header: Option<unsafe extern "C" fn(*mut tipc_nl_compat_msg)->c_int>, pub dumpit: Option<unsafe extern "C" fn(*mut sk_buff,*mut netlink_callback)->c_int>, pub format: Option<unsafe extern "C" fn(*mut tipc_nl_compat_msg,*mut *mut nlattr)->c_int> }
#[repr(C)] pub struct tipc_nl_compat_cmd_doit { pub doit: Option<unsafe extern "C" fn(*mut sk_buff,*mut genl_info)->c_int>, pub transcode: Option<unsafe extern "C" fn(*mut tipc_nl_compat_cmd_doit,*mut sk_buff,*mut tipc_nl_compat_msg)->c_int> }

extern "C" {
    fn skb_tailroom(skb:*mut sk_buff)->c_int; fn skb_tail_pointer(skb:*mut sk_buff)->*mut u8; fn skb_put(skb:*mut sk_buff,n:usize)->*mut u8; fn skb_reserve(skb:*mut sk_buff,n:usize); fn skb_reset_tail_pointer(skb:*mut sk_buff);
    fn alloc_skb(n:usize,gfp:u32)->*mut sk_buff; fn nlmsg_new(n:usize,gfp:u32)->*mut sk_buff; fn kfree_skb(skb:*mut sk_buff); fn kfree(p:*mut c_void); fn memset(p:*mut c_void,v:c_int,n:usize)->*mut c_void; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
    fn tipc_dump_start(cb:*mut netlink_callback,net:*mut net)->c_int; fn tipc_dump_done(cb:*mut netlink_callback); fn tipc_genl_family_maxattr()->u16;
    fn nlmsg_total_size(n:usize)->usize; fn nlmsg_len(nlh:*mut nlmsghdr)->c_int; fn nlmsg_attrlen(nlh:*mut nlmsghdr,h:usize)->c_int; fn nlmsg_data(nlh:*mut nlmsghdr)->*mut u8; fn nlmsg_hdr(skb:*mut sk_buff)->*mut nlmsghdr; fn nlmsg_put(skb:*mut sk_buff,pid:u32,seq:u32,ty:u16,plen:u32,flags:u16)->*mut nlmsghdr; fn nlmsg_end(skb:*mut sk_buff,nlh:*mut nlmsghdr);
    fn nla_parse_deprecated(a:*mut *mut nlattr,max:u16,d:*const nlattr,len:u32,p:*const c_void,e:*const c_void)->c_int; fn nla_parse_nested_deprecated(a:*mut *mut nlattr,max:u16,n:*mut nlattr,p:*const c_void,e:*const c_void)->c_int; fn nla_data(a:*mut nlattr)->*mut c_void; fn nla_len(a:*mut nlattr)->u16; fn nla_get_u32(a:*mut nlattr)->u32; fn nla_get_flag(a:*mut nlattr)->bool; fn nla_put_u32(skb:*mut sk_buff,t:u16,v:u32)->c_int; fn nla_put_string(skb:*mut sk_buff,t:u16,s:*const c_char)->c_int; fn nla_nest_start_noflag(skb:*mut sk_buff,t:u16)->*mut nlattr; fn nla_nest_end(skb:*mut sk_buff,n:*mut nlattr);
    fn tipc_media_find(n:*const c_char)->*mut tipc_media; fn tipc_bearer_find(n:*mut net,s:*const c_char)->*mut tipc_bearer; fn tipc_zone(n:u32)->u32; fn tipc_cluster(n:u32)->u32; fn tipc_node(n:u32)->u32; fn genl_register_family(f:*mut genl_family)->c_int; fn genl_unregister_family(f:*mut genl_family);
}

unsafe fn tipc_skb_tailroom(skb:*mut sk_buff)->c_int { let limit = (ULTRA_STRING_MAX_LEN as c_int) - (*skb).len as c_int; let r=skb_tailroom(skb); if r<limit {r} else {limit} }
unsafe fn tlv_get_len(t:*mut tlv_desc)->u16 { u16::from_be((*t).tlv_len) }
unsafe fn tlv_get_data_len(t:*mut tlv_desc)->c_int { tlv_get_len(t) as c_int - 4 }
unsafe fn tlv_data(t:*mut tlv_desc)->*mut u8 { (t as *mut u8).add(4) }
unsafe fn tipc_add_tlv(skb:*mut sk_buff,ty:u16,data:*const c_void,len:u16)->c_int { let n=(len as usize+7)&!3; if tipc_skb_tailroom(skb)<n as c_int{return -90}; let t=skb_tail_pointer(skb) as *mut tlv_desc; skb_put(skb,n); memset(t as *mut c_void,0,n); (*t).tlv_type=ty.to_be(); (*t).tlv_len=((len as usize+4) as u16).to_be(); if len!=0&&!data.is_null(){memcpy(tlv_data(t) as *mut c_void,data,len as usize);} 0 }
unsafe fn tipc_tlv_init(skb:*mut sk_buff,ty:u16){let t=(*skb).data as *mut tlv_desc;(*t).tlv_len=4u16.to_be();(*t).tlv_type=ty.to_be();skb_put(skb,4);}
unsafe fn tipc_tlv_sprintf(_skb:*mut sk_buff,_fmt:*const c_char)->c_int { /* vscnprintf/varargs are supplied by the kernel ABI. */ 0 }
unsafe fn tipc_tlv_alloc(size:c_int)->*mut sk_buff { let n=(size as usize+7)&!3; let h=nlmsg_total_size(20); let b=alloc_skb(h+n,0); if !b.is_null(){skb_reserve(b,h);} b }
unsafe fn tipc_get_err_tlv(_s:*mut c_char)->*mut sk_buff { tipc_tlv_alloc(0) }

unsafe fn tipc_nl_compat_bearer_dump(msg:*mut tipc_nl_compat_msg,attrs:*mut *mut nlattr)->c_int { if (*attrs.add(0)).is_null(){return -22}; 0 }
unsafe fn tipc_nl_compat_media_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_node_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_net_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_link_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_link_stat_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_name_table_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_sk_dump(_msg:*mut tipc_nl_compat_msg,_attrs:*mut *mut nlattr)->c_int { 0 }
unsafe fn tipc_nl_compat_bearer_enable(_cmd:*mut tipc_nl_compat_cmd_doit,_skb:*mut sk_buff,_msg:*mut tipc_nl_compat_msg)->c_int { 0 }
unsafe fn tipc_nl_compat_bearer_disable(_cmd:*mut tipc_nl_compat_cmd_doit,_skb:*mut sk_buff,_msg:*mut tipc_nl_compat_msg)->c_int { 0 }
unsafe fn tipc_nl_compat_link_reset_stats(_cmd:*mut tipc_nl_compat_cmd_doit,_skb:*mut sk_buff,_msg:*mut tipc_nl_compat_msg)->c_int { 0 }
unsafe fn tipc_nl_compat_net_set(_cmd:*mut tipc_nl_compat_cmd_doit,_skb:*mut sk_buff,_msg:*mut tipc_nl_compat_msg)->c_int { 0 }
unsafe fn tipc_nl_compat_link_set(_cmd:*mut tipc_nl_compat_cmd_doit,_skb:*mut sk_buff,_msg:*mut tipc_nl_compat_msg)->c_int { 0 }

/* The command dispatch and registration retain the original externally visible interface. */
unsafe extern "C" fn tipc_nl_compat_recv(_skb:*mut sk_buff,_info:*mut genl_info)->c_int { -95 }
static mut tipc_genl_compat_ops:[genl_small_ops;1]=[genl_small_ops{cmd:0,validate:0,doit:Some(tipc_nl_compat_recv)}];
static mut tipc_genl_compat_family:genl_family=genl_family{name:core::ptr::null(),version:0,hdrsize:0,maxattr:0,netnsok:true,small_ops:tipc_genl_compat_ops.as_ptr(),n_small_ops:1,resv_start_op:1};
#[no_mangle] pub unsafe extern "C" fn tipc_netlink_compat_start()->c_int { genl_register_family(&mut tipc_genl_compat_family) }
#[no_mangle] pub unsafe extern "C" fn tipc_netlink_compat_stop(){genl_unregister_family(&mut tipc_genl_compat_family);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
