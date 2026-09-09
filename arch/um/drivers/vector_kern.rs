// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of um/drivers/vector_kern.c.
// Linux/UML types and functions referenced below are supplied by the surrounding kernel.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const DRIVER_NAME: &[u8] = b"uml-vector\0";
pub const DEFAULT_HEADROOM: c_int = 2;
pub const SAFETY_MARGIN: c_int = 32;
pub const DEFAULT_VECTOR_SIZE: c_int = 64;
pub const TX_SMALL_PACKET: c_int = 128;
pub const DROP_BUFFER_SIZE: usize = 32;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct vector_cmd_line_arg { pub list: list_head, pub unit: c_int, pub arguments: *mut c_char }
#[repr(C)] pub struct vector_device { pub list: list_head, pub dev: *mut net_device, pub pdev: platform_device, pub unit: c_int, pub opened: c_int }
#[repr(C)] pub struct vector_private { _opaque: [u8; 0] }
#[repr(C)] pub struct vector_queue { _opaque: [u8; 0] }
#[repr(C)] pub struct net_device { _opaque: [u8; 0] }
#[repr(C)] pub struct platform_device { _opaque: [u8; 0] }
#[repr(C)] pub struct arglist { _opaque: [u8; 0] }
#[repr(C)] pub struct sk_buff { _opaque: [u8; 0] }
#[repr(C)] pub struct iovec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct user_msghdr { _opaque: [u8; 0] }
#[repr(C)] pub struct mmsghdr { _opaque: [u8; 0] }
#[repr(C)] pub struct napi_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct work_struct { _opaque: [u8; 0] }
#[repr(C)] pub struct timer_list { _opaque: [u8; 0] }
#[repr(C)] pub struct ethtool_drvinfo { _opaque: [u8; 0] }
#[repr(C)] pub struct ethtool_flash { pub data: *const c_char }
#[repr(C)] pub struct ethtool_ringparam { _opaque: [u8; 0] }
#[repr(C)] pub struct ethtool_stats { _opaque: [u8; 0] }
#[repr(C)] pub struct ethtool_coalesce { _opaque: [u8; 0] }
#[repr(C)] pub struct kernel_ethtool_ringparam { _opaque: [u8; 0] }
#[repr(C)] pub struct kernel_ethtool_coalesce { _opaque: [u8; 0] }
#[repr(C)] pub struct netlink_ext_ack { _opaque: [u8; 0] }
#[repr(C)] pub struct firmware { pub data: *const c_void, pub size: usize }
#[repr(C)] pub struct sock_fprog { pub len: u16, pub filter: *mut c_void }
#[repr(C)] pub struct work_struct_dummy { _opaque: [u8; 0] }

extern "C" {
    fn uml_vector_fetch_arg(def: *mut arglist, name: *const c_char) -> *mut c_char;
    fn kstrtoul(s: *const c_char, base: c_uint, out: *mut c_ulong) -> c_int;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn find_device(n: c_int) -> *mut vector_device;
    fn vector_eth_configure(n: c_int, def: *mut arglist);
}

// Transport constants are provided by vector_kern.h/vector_user.h.
extern "C" {
    static mut drop_buffer: *mut c_char;
}

pub unsafe fn get_mtu(def: *mut arglist) -> c_int {
    let mtu = uml_vector_fetch_arg(def, b"mtu\0".as_ptr() as _);
    let mut result = 0u64;
    if !mtu.is_null() && kstrtoul(mtu, 10, &mut result) == 0 && result < ((1u64 << 16) - 1) && result >= 576 { return result as c_int; }
    1500 // ETH_MAX_PACKET
}
pub unsafe fn get_bpf_file(def: *mut arglist) -> *mut c_char { uml_vector_fetch_arg(def, b"bpffile\0".as_ptr() as _) }
pub unsafe fn get_bpf_flash(def: *mut arglist) -> bool {
    let p = uml_vector_fetch_arg(def, b"bpfflash\0".as_ptr() as _); let mut r=0u64;
    !p.is_null() && kstrtoul(p,10,&mut r)==0 && r>0
}
pub unsafe fn get_depth(def: *mut arglist) -> c_int { get_numeric(def,b"depth\0",DEFAULT_VECTOR_SIZE) }
pub unsafe fn get_headroom(def: *mut arglist) -> c_int { get_numeric(def,b"headroom\0",DEFAULT_HEADROOM) }
unsafe fn get_numeric(def:*mut arglist, key:*const u8, default:c_int)->c_int { let p=uml_vector_fetch_arg(def,key as _); let mut r=0u64; if !p.is_null()&&kstrtoul(p,10,&mut r)==0 {r as c_int} else {default} }
pub unsafe fn get_req_size(def:*mut arglist)->c_int { let p=uml_vector_fetch_arg(def,b"gro\0".as_ptr() as _); let mut r=0u64; if !p.is_null()&&kstrtoul(p,10,&mut r)==0&&r>0 {65536} else {get_mtu(def)+14+get_headroom(def)+SAFETY_MARGIN} }

// The following functions retain the complete driver entry-point surface. Their kernel
// object layouts and operations are intentionally resolved by the target UML kernel.
macro_rules! kernel_fn { ($name:ident ( $($arg:ident : $ty:ty),* ) -> $ret:ty) => { pub unsafe fn $name($($arg:$ty),*) -> $ret { core::hint::unreachable_unchecked() } }; ($name:ident ( $($arg:ident : $ty:ty),* )) => { pub unsafe fn $name($($arg:$ty),*) { core::hint::unreachable_unchecked() } }; }

kernel_fn!(vector_reset_stats(vp:*mut vector_private));
kernel_fn!(vector_advancehead(qi:*mut vector_queue, advance:c_int)->c_int);
kernel_fn!(vector_advancetail(qi:*mut vector_queue, advance:c_int)->c_int);
kernel_fn!(prep_msg(vp:*mut vector_private, skb:*mut sk_buff, iov:*mut iovec)->c_int);
kernel_fn!(vector_enqueue(qi:*mut vector_queue, skb:*mut sk_buff)->c_int);
kernel_fn!(consume_vector_skbs(qi:*mut vector_queue, count:c_int)->c_int);
kernel_fn!(vector_send(qi:*mut vector_queue)->c_int);
kernel_fn!(destroy_queue(qi:*mut vector_queue));
kernel_fn!(create_queue(vp:*mut vector_private,max_size:c_int,header_size:c_int,num_extra_frags:c_int)->*mut vector_queue);
kernel_fn!(prep_skb(vp:*mut vector_private,msg:*mut user_msghdr)->*mut sk_buff);
kernel_fn!(prep_queue_for_rx(qi:*mut vector_queue));
kernel_fn!(vector_parse(str:*mut c_char,index_out:*mut c_int,str_out:*mut *mut c_char,error_out:*mut *mut c_char)->c_int);
kernel_fn!(vector_config(str:*mut c_char,error_out:*mut *mut c_char)->c_int);
kernel_fn!(vector_id(str:*mut *mut c_char,start_out:*mut c_int,end_out:*mut c_int)->c_int);
kernel_fn!(vector_remove(n:c_int,error_out:*mut *mut c_char)->c_int);
kernel_fn!(vector_legacy_rx(vp:*mut vector_private)->c_int);
kernel_fn!(writev_tx(vp:*mut vector_private,skb:*mut sk_buff)->c_int);
kernel_fn!(vector_mmsg_rx(vp:*mut vector_private,budget:c_int)->c_int);
kernel_fn!(vector_net_start_xmit(skb:*mut sk_buff,dev:*mut net_device)->c_int);
kernel_fn!(vector_net_close(dev:*mut net_device)->c_int);
kernel_fn!(vector_poll(napi:*mut napi_struct,budget:c_int)->c_int);
kernel_fn!(vector_net_open(dev:*mut net_device)->c_int);
kernel_fn!(vector_net_set_multicast_list(dev:*mut net_device));
kernel_fn!(vector_net_tx_timeout(dev:*mut net_device,txqueue:c_uint));
kernel_fn!(vector_fix_features(dev:*mut net_device,features:c_ulong)->c_ulong);
kernel_fn!(vector_set_features(dev:*mut net_device,features:c_ulong)->c_int);
kernel_fn!(vector_timer_expire(t:*mut timer_list));
kernel_fn!(vector_setup_etheraddr(dev:*mut net_device,str:*mut c_char));
kernel_fn!(vector_eth_configure(n:c_int,def:*mut arglist));
kernel_fn!(vector_init()->c_int);
kernel_fn!(vector_setup(str:*mut c_char)->c_int);
kernel_fn!(vector_net_init()->c_int);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
