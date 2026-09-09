// SPDX-License-Identifier: GPL-2.0-or-later
/* RxRPC key management */

// Kernel includes and build-time macros are supplied by the surrounding crate.

extern "C" {
    fn rxrpc_u32_to_time64(x: u32) -> i64;
    fn rxrpc_time64_to_u32(x: i64) -> u32;
    fn current_cred() -> *const cred;
    fn key_serial(k: *mut key) -> i32;
    fn key_revoke(k: *mut key);
    fn key_put(k: *mut key);
    fn request_key_net(t: *mut key_type, d: *const i8, n: *mut net, p: *const i8) -> *mut key;
    fn key_alloc(t: *mut key_type, n: *const i8, uid: u32, gid: u32, c: *const cred, perm: u32, flags: u32, q: *const i8) -> *mut key;
    fn key_instantiate_and_link(k: *mut key, data: *const core::ffi::c_void, len: usize, r: *mut key, q: *const i8) -> i32;
    fn memdup_sockptr_nul(p: sockptr_t, n: i32) -> *mut i8;
    fn sock_net(s: *mut sock) -> *mut net;
}

#[repr(C)] pub struct key_type { pub name: *const i8, pub flags: u32, pub preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)->i32>, pub free_preparse: Option<unsafe extern "C" fn(*mut key_preparsed_payload)>, pub instantiate: *const core::ffi::c_void, pub destroy: Option<unsafe extern "C" fn(*mut key)>, pub describe: Option<unsafe extern "C" fn(*const key,*mut seq_file)>, pub read: Option<unsafe extern "C" fn(*const key,*mut i8,usize)->isize> }
#[repr(C)] pub struct key_preparsed_payload { pub data: *const u32, pub datalen: usize, pub quotalen: usize, pub expiry: i64, pub payload: payload }
#[repr(C)] pub struct payload { pub data: [*mut core::ffi::c_void; 2] }
#[repr(C)] pub struct key { pub description: *const i8, pub payload: payload, pub serial: i32 }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct cred;
#[repr(C)] pub struct net;
#[repr(C)] pub struct sock;
#[repr(C)] pub struct rxrpc_sock { pub sk: sock, pub key: *mut key }
#[repr(C)] pub struct rxrpc_connection { pub key: *mut key }
pub type sockptr_t = *mut core::ffi::c_void;

#[repr(C)] pub struct rxrpc_key_token { pub next: *mut rxrpc_key_token, pub security_index: u32, pub no_leak_key: bool, pub kad: *mut rxkad_key, pub rxgk: *mut rxgk_key }
#[repr(C)] pub struct rxkad_key { pub ticket_len:u32, pub vice_id:u32, pub kvno:u32, pub start:u32, pub expiry:u32, pub primary_flag:u32, pub session_key:[u8;8], pub ticket:[u8;0] }
#[repr(C)] pub struct rxgk_key { pub begintime:i64,pub endtime:i64,pub level:i64,pub lifetime:i64,pub bytelife:i64,pub enctype:i64,pub key: bytes,pub ticket: bytes,pub _key:[u8;0] }
#[repr(C)] pub struct bytes { pub len:usize, pub data:*mut u8 }
#[repr(C)] pub struct rxrpc_key_data_v1 { pub security_index:u16,pub ticket_length:u16,pub expiry:u32,pub kvno:u32,pub session_key:[u8;8],pub ticket:[u8;0] }

pub const RXRPC_SECURITY_RXKAD:u32=2; pub const RXRPC_SECURITY_YFS_RXGK:u32=3;
pub static mut key_type_rxrpc: key_type = key_type { name: b"rxrpc\0".as_ptr() as _, flags: 1, preparse: Some(rxrpc_preparse), free_preparse: Some(rxrpc_free_preparse), instantiate: core::ptr::null(), destroy: Some(rxrpc_destroy), describe: Some(rxrpc_describe), read: Some(rxrpc_read) };

unsafe fn rxrpc_preparse_xdr_rxkad(prep:*mut key_preparsed_payload,datalen:usize,xdr:*const u32,toklen:u32)->i32 { if toklen<=32 || toklen<32+u32::from_be(*xdr.add(7)){return -127;} let tktlen=u32::from_be(*xdr.add(7)) as usize; let token=alloc_token(); if token.is_null(){return -12;} (*token).security_index=RXRPC_SECURITY_RXKAD; (*token).kad=alloc_kad(tktlen); if (*token).kad.is_null(){free_token(token);return -12;} (*(*token).kad).ticket_len=tktlen as u32; (*(*token).kad).vice_id=u32::from_be(*xdr); (*(*token).kad).kvno=u32::from_be(*xdr.add(1)); (*(*token).kad).start=u32::from_be(*xdr.add(4)); (*(*token).kad).expiry=u32::from_be(*xdr.add(5)); (*(*token).kad).primary_flag=u32::from_be(*xdr.add(6)); core::ptr::copy_nonoverlapping(xdr.add(2) as *const u8,(*(*token).kad).session_key.as_mut_ptr(),8); core::ptr::copy_nonoverlapping(xdr.add(8) as *const u8,(*(*token).kad).ticket.as_mut_ptr(),tktlen); attach(prep,token); (*prep).quotalen+=datalen+core::mem::size_of::<rxrpc_key_token>()+core::mem::size_of::<rxkad_key>()+tktlen; 0 }
unsafe fn xdr_dec64(x:*const u32)->u64 {(u32::from_be(*x) as u64)<<32|u32::from_be(*x.add(1)) as u64}
fn rxrpc_s64_to_time64(x:i64)->i64 { x/10_000_000 }
unsafe fn rxrpc_preparse_xdr_yfs_rxgk(prep:*mut key_preparsed_payload,datalen:usize,xdr:*const u32,toklen:u32)->i32 { if toklen<52{return -127;} let key=xdr.add(13); let raw_keylen=u32::from_be(*key.sub(1)) as usize; let keylen=(raw_keylen+3)&!3; if raw_keylen>65536 || 52+keylen>toklen as usize{return -127;} let ticket=key.add(keylen/4+1); let raw_tktlen=u32::from_be(*ticket.sub(1)) as usize; let tktlen=(raw_tktlen+3)&!3; if raw_tktlen>65536 || 52+keylen+tktlen!=toklen as usize{return -127;} let token=alloc_token(); if token.is_null(){return -12;} (*token).security_index=RXRPC_SECURITY_YFS_RXGK; (*token).rxgk=alloc_rxgk(raw_keylen); if (*token).rxgk.is_null(){free_token(token);return -12;} let r=(*token).rxgk; (*r).begintime=xdr_dec64(xdr) as i64; (*r).endtime=xdr_dec64(xdr.add(2)) as i64; (*r).level=xdr_dec64(xdr.add(4)) as i64; (*r).lifetime=xdr_dec64(xdr.add(6)) as i64; (*r).bytelife=xdr_dec64(xdr.add(8)) as i64; (*r).enctype=xdr_dec64(xdr.add(10)) as i64; (*r).key.len=raw_keylen; (*r).key.data=(*r)._key.as_mut_ptr(); (*r).ticket.len=raw_tktlen; (*r).ticket.data=alloc_bytes(tktlen); if (*r).ticket.data.is_null(){free_token(token);return -12;} core::ptr::copy_nonoverlapping(key as *const u8,(*r).key.data,raw_keylen); core::ptr::copy_nonoverlapping(ticket as *const u8,(*r).ticket.data,raw_tktlen); attach(prep,token); (*prep).quotalen+=datalen+core::mem::size_of::<rxrpc_key_token>()+core::mem::size_of::<rxgk_key>()+tktlen+keylen; 0 }

unsafe fn rxrpc_preparse_xdr(prep:*mut key_preparsed_payload)->i32 { let d=(*prep).datalen; if d>131072 || d&3!=0{return -71;} let x=(*prep).data; if u32::from_be(*x)!=0{return -71;} let len=u32::from_be(*x.add(1)) as usize; if len<1||len>256{return -71;} let mut off=2+(len+3)/4; if d<off*4+12{return -71;} let n=u32::from_be(*x.add(off)) as usize; off+=1; if n<1||n>16{return -71;} let mut p=off; for _ in 0..n {let l=u32::from_be(*x.add(p)) as usize; if l<20||p*4+4+l>d{return -71;} p+=(l+7)/4;} if p*4!=d{return -71;} let mut ret=-93; let mut q=off; for _ in 0..n {let l=u32::from_be(*x.add(q)) as u32; let sec=u32::from_be(*x.add(q+1)); let body=x.add(q+2); let r=match sec {RXRPC_SECURITY_RXKAD=>rxrpc_preparse_xdr_rxkad(prep,d,body,l-4),RXRPC_SECURITY_YFS_RXGK=>rxrpc_preparse_xdr_yfs_rxgk(prep,d,body,l-4),_=>-93}; if r==0{ret=0}else if r!=-93{return r} q+=(l as usize+7)/4+1;} ret }

unsafe fn rxrpc_preparse(prep:*mut key_preparsed_payload)->i32 { if (*prep).data.is_null()&&(*prep).datalen==0{return 0;} if (*prep).datalen>28 {let r=rxrpc_preparse_xdr(prep);if r!=-71{return r;}} if (*prep).datalen<=4||(*prep).data.is_null(){return -22;} let v=(*prep).data as *const rxrpc_key_data_v1; if (*prep).datalen<core::mem::size_of::<rxrpc_key_data_v1>()+(*v).ticket_length as usize||(*v).security_index!=RXRPC_SECURITY_RXKAD{return -127;} let t=alloc_token();if t.is_null(){return -12;} (*t).security_index=RXRPC_SECURITY_RXKAD;(*t).kad=alloc_kad((*v).ticket_length as usize);if (*t).kad.is_null(){free_token(t);return -12;} core::ptr::copy_nonoverlapping(v as *const u8,(*t).kad as *mut u8,core::mem::size_of::<rxkad_key>()); attach(prep,t);0 }
unsafe fn rxrpc_free_preparse(p:*mut key_preparsed_payload){free_list((*p).payload.data[0] as *mut rxrpc_key_token)} unsafe fn rxrpc_destroy(k:*mut key){free_list((*k).payload.data[0] as *mut rxrpc_key_token)}
unsafe fn rxrpc_describe(k:*const key,m:*mut seq_file){let _=(k,m);} unsafe fn rxrpc_read(_k:*const key,_b:*mut i8,_n:usize)->isize{-95}
unsafe fn alloc_token()->*mut rxrpc_key_token{let p=libc_malloc(core::mem::size_of::<rxrpc_key_token>()) as *mut _;if !p.is_null(){core::ptr::write_bytes(p,0,1);}p} unsafe fn alloc_kad(_n:usize)->*mut rxkad_key{libc_malloc(core::mem::size_of::<rxkad_key>()+_n) as *mut _} unsafe fn alloc_rxgk(_n:usize)->*mut rxgk_key{libc_malloc(core::mem::size_of::<rxgk_key>()+_n) as *mut _} unsafe fn alloc_bytes(n:usize)->*mut u8{libc_malloc(n) as *mut u8} unsafe fn libc_malloc(n:usize)->*mut core::ffi::c_void{extern "C"{fn malloc(n:usize)->*mut core::ffi::c_void;}malloc(n)} unsafe fn free_token(p:*mut rxrpc_key_token){extern "C"{fn free(p:*mut core::ffi::c_void);}free(p as _)} unsafe fn attach(p:*mut key_preparsed_payload,t:*mut rxrpc_key_token){let q=&mut (*p).payload.data[0] as *mut _ as *mut *mut rxrpc_key_token;while !(*q).is_null(){q=&mut (**q).next;}*q=t} unsafe fn free_list(mut p:*mut rxrpc_key_token){extern "C"{fn free(p:*mut core::ffi::c_void);}while !p.is_null(){let n=(*p).next;if !(*p).kad.is_null(){free((*p).kad as _)}if !(*p).rxgk.is_null(){free((*p).rxgk as _)}free(p as _);p=n;}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
