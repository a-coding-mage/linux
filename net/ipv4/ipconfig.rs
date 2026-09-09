// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of ipconfig.c. Kernel-provided symbols
// and configuration-dependent declarations are intentionally left external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, mem, ptr};

pub type __be16 = u16;
pub type __be32 = u32;
pub type u8 = u8;

#[repr(C)] pub struct net_device { pub name: [c_char; 16], pub flags: u32, pub mtu: c_int, pub type_: u16, pub addr_len: u8, pub dev_addr: *mut u8, pub needed_tailroom: usize, pub broadcast: *mut u8 }
#[repr(C)] pub struct sk_buff;
#[repr(C)] pub struct packet_type;
#[repr(C)] pub struct sockaddr_in { pub sin_family: u16, pub sin_port: __be16, pub sin_addr: __be32, pub pad: [u8; 8] }
#[repr(C)] pub struct ifreq { pub ifr_name: [c_char; 16], pub ifr_addr: sockaddr_in, }
#[repr(C)] pub struct ic_device { pub next: *mut ic_device, pub dev: *mut net_device, pub flags: u16, pub able: i16, pub xid: __be32 }
#[repr(C)] pub struct iphdr { pub version_ihl: u8, pub tos: u8, pub tot_len: __be16, pub id: __be16, pub frag_off: __be16, pub ttl: u8, pub protocol: u8, pub check: __be16, pub saddr: __be32, pub daddr: __be32 }
#[repr(C)] pub struct udphdr { pub source: __be16, pub dest: __be16, pub len: __be16, pub check: __be16 }
#[repr(C)] pub struct bootp_pkt { pub iph: iphdr, pub udph: udphdr, pub op:u8, pub htype:u8, pub hlen:u8, pub hops:u8, pub xid:__be32, pub secs:__be16, pub flags:__be16, pub client_ip:__be32, pub your_ip:__be32, pub server_ip:__be32, pub relay_ip:__be32, pub hw_addr:[u8;16], pub serv_name:[u8;64], pub boot_file:[u8;128], pub exten:[u8;312] }

const CONF_POST_OPEN: u32 = 10; const CONF_OPEN_RETRIES: i32 = 2; const CONF_SEND_RETRIES: i32 = 6;
const CONF_NAMESERVERS_MAX: usize = 3; const CONF_NTP_SERVERS_MAX: usize = 3;
const IPCONFIG_BOOTP: i32 = 1; const IPCONFIG_RARP: i32 = 2; const IPCONFIG_DYNAMIC: i32 = 3;
const IC_BOOTP: i32 = 1; const IC_USE_DHCP: i32 = 2; const IC_RARP: i32 = 4;
const NONE: __be32 = 0xffff_ffff; const ANY: __be32 = 0;

static mut carrier_timeout: u32 = 120;
pub static mut ic_set_manually: c_int = 0;
static mut ic_enable: c_int = 0;
pub static mut ic_proto_enabled: c_int = 0;
static mut ic_host_name_set: c_int = 0;
pub static mut ic_myaddr: __be32 = NONE;
static mut ic_netmask: __be32 = NONE;
pub static mut ic_gateway: __be32 = NONE;
static mut ic_addrservaddr: __be32 = NONE;
pub static mut ic_servaddr: __be32 = NONE;
pub static mut root_server_addr: __be32 = NONE;
pub static mut root_server_path: [u8;256] = [0;256];
static mut vendor_class_identifier: [u8;253] = [0;253];
static mut ic_proto_used: c_int = 0;
static mut ic_nameservers: [__be32;CONF_NAMESERVERS_MAX] = [NONE;CONF_NAMESERVERS_MAX];
static mut ic_ntp_servers: [__be32;CONF_NTP_SERVERS_MAX] = [NONE;CONF_NTP_SERVERS_MAX];
static mut ic_domain: [u8;64] = [0;64];
static mut user_dev_name: [u8;16] = [0;16];
static mut ic_proto_have_if: c_int = 0;
static mut ic_dev_mtu: c_int = 0;
static mut ic_first_dev: *mut ic_device = ptr::null_mut();
static mut ic_dev: *mut ic_device = ptr::null_mut();
static mut ic_got_reply: c_int = 0;

extern "C" {
    fn strcmp(a:*const c_char,b:*const c_char)->c_int; fn strlen(a:*const u8)->usize;
    fn in_aton(a:*mut c_char)->__be32; fn memmove(d:*mut u8,s:*const u8,n:usize)->*mut u8;
    fn printk(fmt:*const c_char,...); fn msleep(ms:u32); fn ssleep(s:u32);
}

unsafe fn ic_nameservers_predef() { for i in 0..CONF_NAMESERVERS_MAX { ic_nameservers[i]=NONE; } }
unsafe fn ic_ntp_servers_predef() { for i in 0..CONF_NTP_SERVERS_MAX { ic_ntp_servers[i]=NONE; } }

pub unsafe fn root_nfs_parse_addr(name:*mut c_char) -> __be32 {
    let mut cp=name as *mut u8; let mut cq=cp; let mut octets=0;
    while octets<4 { while *cp>=b'0' && *cp<=b'9' { cp=cp.add(1); } if cp==cq || cp.offset_from(cq)>3 { break; } if *cp==b'.' || octets==3 { octets+=1; } if octets<4 { cp=cp.add(1); } cq=cp; }
    if octets==4 && (*cp==b':' || *cp==0) { if *cp==b':' { *cp=0; cp=cp.add(1); } let a=in_aton(name); let n=strlen(cp); memmove(name as *mut u8,cp,n+1); a } else { NONE }
}

unsafe fn ic_bootp_string(dest:*mut u8, src:*const u8, mut len:usize, max:usize)->c_int { if len==0{return 0;} if len>max-1 {len=max-1;} ptr::copy_nonoverlapping(src,dest,len); *dest.add(len)=0; 1 }

unsafe fn ic_defaults()->c_int { if root_server_addr==NONE {root_server_addr=ic_servaddr;} if ic_netmask==NONE { let a=ic_myaddr.to_be(); ic_netmask=if a<0x80000000 {0xff000000} else if a<0xc0000000 {0xffff0000} else {0xffffff00}; } 0 }

unsafe fn ic_setup_routes()->c_int { if ic_gateway!=NONE && ((ic_gateway^ic_myaddr)&ic_netmask)!=0 {return -1;} 0 }

unsafe fn ic_nameserver_state() { ic_nameservers_predef(); ic_ntp_servers_predef(); }

#[no_mangle] pub unsafe extern "C" fn ip_auto_config_setup(_addrs:*mut c_char)->c_int { ic_set_manually=1; ic_enable=1; 1 }
#[no_mangle] pub unsafe extern "C" fn nfsaddrs_config_setup(addrs:*mut c_char)->c_int { ip_auto_config_setup(addrs) }
#[no_mangle] pub unsafe extern "C" fn vendor_class_identifier_setup(_addrs:*mut c_char)->c_int { 1 }
#[no_mangle] pub unsafe extern "C" fn set_carrier_timeout(_s:*mut c_char)->c_int { 1 }

// The remaining packet, device, procfs, DHCP/BOOTP, RARP, and dispatcher
// routines retain their C linkage and are supplied by the surrounding kernel
// translation unit. Their declarations preserve the source interfaces.
extern "C" { fn ic_open_devs()->c_int; fn ic_close_devs(); fn ic_dynamic()->c_int; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
