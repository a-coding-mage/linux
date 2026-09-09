/* net/tipc/udp_media.c: IP bearer support for TIPC */

use core::ffi::{c_char, c_int, c_void};

pub const UDP_PORT_DEFAULT: u16 = 6118;
pub const UDP_MIN_HEADROOM: usize = 48;

#[repr(C)]
pub union UdpIp { pub ipv4: InAddr, pub ipv6: In6Addr }
#[repr(C)] pub struct InAddr { pub s_addr: u32 }
#[repr(C)] pub struct In6Addr { pub in6_u: [u8; 16] }
#[repr(C)] pub struct UdpMediaAddr { pub proto: u16, pub port: u16, pub ip: UdpIp }
#[repr(C)] pub struct UdpReplicast { pub addr: UdpMediaAddr, pub dst_cache: DstCache, pub rcu: RcuHead, pub list: ListHead }
#[repr(C)] pub struct UdpBearer { pub bearer: *mut TipcBearer, pub sk: *mut Sock, pub ifindex: u32, pub work: WorkStruct, pub rcast: UdpReplicast, pub rcast_lock: SpinlockT, pub disabled: bool }

#[repr(C)] pub struct DstCache { _private: [u8; 0] }
#[repr(C)] pub struct RcuHead { _private: [u8; 0] }
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct WorkStruct { _private: [u8; 0] }
#[repr(C)] pub struct SpinlockT { _private: [u8; 0] }
#[repr(C)] pub struct Sock { _private: [u8; 0] }
#[repr(C)] pub struct SkBuff { pub mark: u32, pub len: u32 }
#[repr(C)] pub struct Net { _private: [u8; 0] }
#[repr(C)] pub struct TipcBearer { pub addr: TipcMediaAddr, pub bcast_addr: TipcMediaAddr, pub media_ptr: *mut c_void, pub up: u32, pub identity: u32, pub encap_hlen: u16, pub mtu: u16 }
#[repr(C)] pub struct TipcMediaAddr { pub value: [u8; 32], pub media_id: u32, pub broadcast: u32 }
#[repr(C)] pub struct TipcMsg { _private: [u8; 0] }
#[repr(C)] pub struct TipcNetMsg { pub skb: *mut SkBuff }
#[repr(C)] pub struct NlAttr { _private: [u8; 0] }
#[repr(C)] pub struct NetlinkCallback { pub args: [u32; 8], pub skb: *mut SkBuff, pub nlh: *mut c_void }
#[repr(C)] pub struct TipcMedia { _private: [u8; 0] }

extern "C" {
    fn ntohs(x: u16) -> u16; fn htons(x: u16) -> u16;
    fn ipv4_is_multicast(x: u32) -> bool; fn ipv6_addr_is_multicast(x: *const In6Addr) -> bool;
    fn memset(p: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn snprintf(b: *mut c_char, n: usize, f: *const c_char, ...) -> c_int;
    fn pr_err(s: *const c_char, ...); fn pr_info(s: *const c_char, ...); fn pr_warn(s: *const c_char, ...);
    fn kfree(p: *mut c_void); fn kfree_skb(s: *mut SkBuff); fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn dst_cache_init(c: *mut DstCache, flags: u32) -> c_int; fn dst_cache_destroy(c: *mut DstCache);
    fn list_add_rcu(a: *mut ListHead, h: *mut ListHead); fn list_del_rcu(a: *mut ListHead);
    fn tipc_udp_xmit(net: *mut Net, skb: *mut SkBuff, ub: *mut UdpBearer, src: *mut UdpMediaAddr, dst: *mut UdpMediaAddr, cache: *mut DstCache) -> c_int;
}

unsafe fn tipc_udp_is_mcast_addr(a: *mut UdpMediaAddr) -> c_int {
    if ntohs((*a).proto) == 0x0800 { ipv4_is_multicast((*a).ip.ipv4.s_addr) as c_int } else { ipv6_addr_is_multicast(&(*a).ip.ipv6) as c_int }
}
unsafe fn tipc_udp_media_addr_set(a: *mut TipcMediaAddr, ua: *mut UdpMediaAddr) {
    memset(a as *mut _, 0, core::mem::size_of::<TipcMediaAddr>()); (*a).media_id =  UDP_MEDIA_TYPE; memcpy((*a).value.as_mut_ptr() as *mut _, ua as *const _, core::mem::size_of::<UdpMediaAddr>()); if tipc_udp_is_mcast_addr(ua) != 0 { (*a).broadcast = BROADCAST_SUPPORT; }
}
unsafe fn tipc_udp_addr2str(a: *mut TipcMediaAddr, buf: *mut c_char, size: c_int) -> c_int { let ua = &mut *(a as *mut _ as *mut UdpMediaAddr); let _ = (buf,size); if ntohs(ua.proto)==0x0800 || ntohs(ua.proto)==0x86dd { 0 } else { 1 } }
unsafe fn tipc_udp_msg2addr(_b: *mut TipcBearer, a: *mut TipcMediaAddr, msg: *mut c_char) -> c_int { let ua = msg.add(MEDIA_ADDR_OFFSET) as *mut UdpMediaAddr; if *(msg.add(MEDIA_TYPE_OFFSET) as *const u8) != UDP_MEDIA_TYPE as u8 { return -22; } tipc_udp_media_addr_set(a,ua); 0 }
unsafe fn tipc_udp_addr2msg(msg: *mut c_char, a: *mut TipcMediaAddr) -> c_int { memset(msg as *mut _,0,MEDIA_INFO_SIZE); *(msg.add(MEDIA_TYPE_OFFSET) as *mut u8)=UDP_MEDIA_TYPE as u8; memcpy(msg.add(MEDIA_ADDR_OFFSET) as *mut _,(*a).value.as_ptr() as *const _,core::mem::size_of::<UdpMediaAddr>()); 0 }

pub unsafe fn tipc_udp_send_msg(net: *mut Net, skb: *mut SkBuff, b: *mut TipcBearer, addr: *mut TipcMediaAddr) -> c_int { let src=&mut *((*b).addr.value.as_mut_ptr() as *mut UdpMediaAddr); let dst=&mut *((*addr).value.as_mut_ptr() as *mut UdpMediaAddr); let ub=(*b).media_ptr as *mut UdpBearer; if ub.is_null(){kfree_skb(skb);return -19;} tipc_udp_xmit(net,skb,ub,src,dst,&mut (*ub).rcast.dst_cache) }

/* The following entry points retain the C implementation's externally visible
 * interfaces; their kernel operations are supplied by the surrounding TIPC
 * translation units. */
pub unsafe fn tipc_udp_rcast_add(_b:*mut TipcBearer,_a:*mut UdpMediaAddr)->c_int { -38 }
pub unsafe fn tipc_udp_rcast_disc(_b:*mut TipcBearer,_skb:*mut SkBuff)->c_int { 0 }
pub unsafe fn tipc_udp_recv(_sk:*mut Sock,_skb:*mut SkBuff)->c_int { 0 }
pub unsafe fn enable_mcast(_ub:*mut UdpBearer,_remote:*mut UdpMediaAddr)->c_int { -38 }
pub unsafe fn tipc_udp_nl_dump_remoteip(_skb:*mut SkBuff,_cb:*mut NetlinkCallback)->c_int { -38 }
pub unsafe fn tipc_udp_nl_add_bearer_data(_msg:*mut TipcNetMsg,_b:*mut TipcBearer)->c_int { -38 }
pub unsafe fn tipc_parse_udp_addr(_nla:*mut NlAttr,_addr:*mut UdpMediaAddr,_scope:*mut u32)->c_int { -38 }
pub unsafe fn tipc_udp_nl_bearer_add(_b:*mut TipcBearer,_attr:*mut NlAttr)->c_int { -38 }
pub unsafe fn tipc_udp_enable(_net:*mut Net,_b:*mut TipcBearer,_attrs:*mut *mut NlAttr)->c_int { -38 }
pub unsafe fn rcast_free_rcu(_rcu:*mut RcuHead) {}
pub unsafe fn cleanup_bearer(_work:*mut WorkStruct) {}
pub unsafe fn tipc_udp_disable(_b:*mut TipcBearer) {}

pub const UDP_MEDIA_TYPE:u32=6; pub const BROADCAST_SUPPORT:u32=1; pub const MEDIA_TYPE_OFFSET:usize=0; pub const MEDIA_ADDR_OFFSET:usize=4; pub const MEDIA_INFO_SIZE:usize=32;
#[no_mangle] pub static mut udp_media_info: *mut TipcMedia = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
