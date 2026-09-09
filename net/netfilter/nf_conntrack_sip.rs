// SPDX-License-Identifier: GPL-2.0-only
/* SIP extension for IP connection tracking.  This is a low-level Rust
 * translation; kernel-provided types and functions are intentionally external.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const HELPER_NAME: &[u8] = b"sip\0";
pub static mut sip_timeout: u32 = SIP_TIMEOUT;
pub static mut sip_direct_signalling: c_int = 1;
pub static mut sip_direct_media: c_int = 1;
pub static mut sip_external_media: c_int = 0;

#[repr(C)] pub union nf_inet_addr { pub ip: u32, pub ip6: [u32; 4] }
#[repr(C)] pub struct nf_conn { pub _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: u32 }
#[repr(C)] pub struct sip_header { pub len: u32, pub name: *const c_char, pub clen: u32, pub cname: *const c_char, pub slen: u32, pub search: *const c_char, pub match_len: Option<unsafe extern "C" fn(*const nf_conn,*const c_char,*const c_char,*mut c_int)->c_int> }
#[repr(C)] pub struct sdp_media_type { pub len: u32, pub name: *const c_char, pub class_: c_int }
#[repr(C)] pub struct nf_nat_sip_hooks { pub _private: [u8; 0] }

pub type __be16 = u16;
pub type u8_ = u8;
pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 10;
pub const NF_ACCEPT: c_int = 1;
pub const NF_DROP: c_int = 0;
pub const SIP_PORT: u16 = 5060;
pub const SIP_TIMEOUT: u32 = 3600;

extern "C" {
    fn nf_ct_l3num(ct: *const nf_conn) -> c_int;
    fn htons(x: u16) -> u16;
    fn ntohs(x: u16) -> u16;
    fn in4_pton(src:*const c_char, len:c_int, dst:*mut u8, delim:c_int, end:*mut *const c_char)->c_int;
    fn in6_pton(src:*const c_char, len:c_int, dst:*mut u8, delim:c_int, end:*mut *const c_char)->c_int;
    fn isalpha(c:c_int)->c_int; fn isdigit(c:c_int)->c_int; fn isalnum(c:c_int)->c_int;
}

unsafe fn string_len(_ct:*const nf_conn, mut p:*const c_char, limit:*const c_char, _shift:*mut c_int)->c_int { let s=p; while p<limit && isalpha(*p as u8 as c_int)!=0 { p=p.add(1); } p.offset_from(s) as c_int }
unsafe fn digits_len(_ct:*const nf_conn, mut p:*const c_char, limit:*const c_char, _shift:*mut c_int)->c_int { let s=p; while p<limit && isdigit(*p as u8 as c_int)!=0 { p=p.add(1); } p.offset_from(s) as c_int }
unsafe fn iswordc(c:u8)->bool { isalnum(c as c_int)!=0 || matches!(c,b'!'|b'"'|b'%'|b':'|b'<'|b'>'|b'?'|b'_'|b'`'|b'{'|b'}'|b'~'|b'\'' ) || (c>=b'('&&c<=b'+') || (c>=b'['&&c<=b']') || (c>=b'-'&&c<=b'/')) }
unsafe fn word_len(mut p:*const c_char, limit:*const c_char)->c_int { let s=p; while p<limit && iswordc(*p as u8) { p=p.add(1); } p.offset_from(s) as c_int }

unsafe fn sip_parse_addr(ct:*const nf_conn, mut cp:*const c_char, endp:*mut *const c_char, addr:*mut nf_inet_addr, limit:*const c_char, delim:bool)->bool {
    if ct.is_null(){return false;} core::ptr::write_bytes(addr as *mut u8,0,core::mem::size_of::<nf_inet_addr>());
    let mut end=cp; let ret=match nf_ct_l3num(ct) { AF_INET=>in4_pton(cp,limit.offset_from(cp) as c_int,addr as *mut u8,-1,&mut end), AF_INET6=>{if cp<limit&&*cp as u8==b'['{cp=cp.add(1)}else if delim{return false} let r=in6_pton(cp,limit.offset_from(cp) as c_int,addr as *mut u8,-1,&mut end); if end<limit&&*end as u8==b']'{end=end.add(1)}else if delim{return false} r}, _=>0};
    if ret==0{return false} if !endp.is_null(){*endp=end} true
}

unsafe fn sip_parse_port(mut p:*const c_char,endp:*mut *const c_char,limit:*const c_char,port:*mut __be16)->bool {
    if p>=limit{return false} if *p as u8 != b':' {if !port.is_null(){*port=htons(SIP_PORT)} if !endp.is_null(){*endp=p} return true} p=p.add(1); let mut n=0u32; let mut len=0; while p<limit&&isdigit(*p as u8 as c_int)!=0 {n=n*10+(*p as u8-b'0') as u32;p=p.add(1);len+=1;if len>5{return false}} if len==0||p>=limit||n<1024||n>65535{return false} if !port.is_null(){*port=htons(n as u16)} if !endp.is_null(){*endp=p} true
}

pub unsafe extern "C" fn ct_sip_parse_request(ct:*const nf_conn,dptr:*const c_char,datalen:u32,matchoff:*mut u32,matchlen:*mut u32,addr:*mut nf_inet_addr,port:*mut __be16)->c_int {
    let limit=dptr.add(datalen as usize); let m=string_len(ct,dptr,limit,core::ptr::null_mut()); if m==0{return 0}; let mut p=dptr.add(m as usize+1); if p>=limit{return 0}; let tag=b"sip:"; while p.add(4)<=limit {if *p as u8==b'\r'||*p as u8==b'\n'{return -1} if core::slice::from_raw_parts(p as *const u8,4)==tag {p=p.add(4);break} p=p.add(1)} let mut e=p; if !sip_parse_addr(ct,p,&mut e,addr,limit,true){return -1} if !sip_parse_port(e,&mut e,limit,port){return -1} if e==p{return 0} *matchoff=p.offset_from(dptr) as u32;*matchlen=e.offset_from(p) as u32;1
}

// Header, SDP, expectation, helper, module-init and NAT routines retain the
// kernel ABI and are declared here for definitions supplied by other units.
extern "C" {
    pub fn ct_sip_get_header(ct:*const nf_conn,dptr:*const c_char,dataoff:u32,datalen:u32,ty:c_int,matchoff:*mut u32,matchlen:*mut u32)->c_int;
    pub fn ct_sip_parse_header_uri(ct:*const nf_conn,dptr:*const c_char,dataoff:*mut u32,datalen:u32,ty:c_int,in_header:*mut c_int,matchoff:*mut u32,matchlen:*mut u32,addr:*mut nf_inet_addr,port:*mut __be16)->c_int;
    pub fn ct_sip_parse_address_param(ct:*const nf_conn,dptr:*const c_char,dataoff:u32,datalen:u32,name:*const c_char,matchoff:*mut u32,matchlen:*mut u32,addr:*mut nf_inet_addr,delim:bool)->c_int;
    pub fn ct_sip_parse_numerical_param(ct:*const nf_conn,dptr:*const c_char,dataoff:u32,datalen:u32,name:*const c_char,matchoff:*mut u32,matchlen:*mut u32,val:*mut u32)->c_int;
    pub fn ct_sip_get_sdp_header(ct:*const nf_conn,dptr:*const c_char,dataoff:u32,datalen:u32,ty:c_int,term:c_int,matchoff:*mut u32,matchlen:*mut u32)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
