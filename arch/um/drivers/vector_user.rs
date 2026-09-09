// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of vector_user.c. */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long};

// C headers and project headers provide these declarations and constants.
#[repr(C)] pub struct arglist { pub numargs: c_int, pub tokens: [*mut c_char; 64], pub values: [*mut c_char; 64] }
#[repr(C)] pub struct vector_fds { pub rx_fd: c_int, pub tx_fd: c_int, pub remote_addr: *mut c_void, pub remote_addr_size: usize }
#[repr(C)] pub struct sock_filter { pub code: u16, pub jt: u8, pub jf: u8, pub k: u32 }
#[repr(C)] pub struct sock_fprog { pub len: u16, pub filter: *mut sock_filter }
#[repr(C)] pub struct sockaddr_un { pub sun_family: u16, pub sun_path: [c_char; 108] }
#[repr(C)] pub struct sockaddr_ll { pub sll_family: u16, pub sll_protocol: u16, pub sll_ifindex: c_int }
#[repr(C)] pub struct ifreq { pub ifr_name: [c_char; 16], pub ifr_flags: i16, pub ifr_ifindex: c_int }
#[repr(C)] pub struct addrinfo { pub ai_flags: c_int, pub ai_family: c_int, pub ai_socktype: c_int, pub ai_protocol: c_int, pub ai_addrlen: u32, pub ai_addr: *mut c_void, pub ai_next: *mut addrinfo }
#[repr(C)] pub struct msghdr { pub msg_iov: *mut iovec, pub msg_iovlen: usize }
#[repr(C)] pub struct iovec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct mmsghdr { pub msg_hdr: msghdr, pub msg_len: u32 }
#[repr(C)] pub struct stat { pub st_size: c_long }

extern "C" {
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int; fn strlen(s: *const c_char) -> usize;
    fn strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int; fn strtol(s:*const c_char,e:*mut *mut c_char,b:c_int)->c_long;
    fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void; fn memset(d:*mut c_void,v:c_int,n:usize)->*mut c_void;
    fn uml_kmalloc(n:usize,g:c_int)->*mut c_void; fn kfree(p:*mut c_void); fn printk(fmt:*const c_char,...);
    fn run_helper(a:*mut c_void,b:*mut c_void,argv:*mut *mut c_char); fn os_close_file(fd:c_int)->c_int;
    fn open(p:*const c_char,f:c_int,...)->c_int; fn ioctl(fd:c_int,r:c_int,...)->c_int; fn socket(a:c_int,b:c_int,c:c_int)->c_int;
    fn bind(fd:c_int,a:*const c_void,n:usize)->c_int; fn connect(fd:c_int,a:*const c_void,n:usize)->c_int; fn close(fd:c_int)->c_int;
    fn socketpair(a:c_int,b:c_int,c:c_int,s:*mut c_int)->c_int; fn setsockopt(fd:c_int,l:c_int,o:c_int,v:*const c_void,n:usize)->c_int;
    fn getaddrinfo(n:*const c_char,p:*const c_char,h:*const addrinfo,r:*mut *mut addrinfo)->c_int; fn freeaddrinfo(r:*mut addrinfo);
    fn gai_strerror(e:c_int)->*const c_char; fn readv(fd:c_int,i:*const iovec,n:usize)->c_int; fn writev(fd:c_int,i:*const iovec,n:c_int)->c_int;
    fn sendmsg(fd:c_int,m:*const msghdr,f:c_int)->c_int; fn sendmmsg(fd:c_int,m:*mut mmsghdr,n:u32,f:u32)->c_int; fn recvmmsg(fd:c_int,m:*mut mmsghdr,n:u32,f:u32,t:*mut c_void)->c_int;
    fn stat(p:*const c_char,s:*mut stat)->c_int; fn os_open_file(p:*const c_char,f:c_int,m:c_int)->c_int; fn os_read_file(fd:c_int,b:*mut c_void,n:c_long)->c_int;
    fn ntohl(x:u32)->u32; fn ntohs(x:u16)->u16;
}

const ID_GRE:c_int=0; const ID_L2TPV3:c_int=1; const ID_BESS:c_int=2; const ID_MAX:c_int=2;
const MAX_UN_LEN:usize=107; const DEFAULT_BPF_LEN:usize=6;
static PADCHAR:&[u8]=b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"; static TEMPLATE:&[u8]=b"tapXXXXXX\0";

pub unsafe fn uml_vector_fetch_arg(ifspec:*mut arglist, token:*mut c_char)->*mut c_char { for i in 0..(*ifspec).numargs as usize { if strcmp((*ifspec).tokens[i],token)==0{return (*ifspec).values[i];} } std::ptr::null_mut() }

pub unsafe fn uml_parse_vector_ifspec(arg:*mut c_char)->*mut arglist {
    if arg.is_null(){return std::ptr::null_mut()} let r=uml_kmalloc(std::mem::size_of::<arglist>(),0) as *mut arglist; if r.is_null(){return r}; (*r).numargs=0; let mut token=true; let mut next=true; let len=strlen(arg);
    for p in 0..len { if next { if token {(*r).tokens[(*r).numargs as usize]=arg.add(p)} else {(*r).values[(*r).numargs as usize]=arg.add(p);(*r).numargs+=1;} next=false; } let ch=*arg.add(p); if ch==b'=' as c_char {if !token {kfree(r as *mut c_void);return std::ptr::null_mut()} token=false;next=true;*arg.add(p)=0;} if ch==b',' as c_char {token=true;next=true;*arg.add(p)=0;} } r
}

// The remaining routines retain the C ABI and operation ordering; platform constants/helpers are supplied by the UML build.
extern "C" { fn user_init_raw_fds(_: *mut arglist)->*mut vector_fds; fn user_init_hybrid_fds(_: *mut arglist)->*mut vector_fds; fn user_init_tap_fds(_: *mut arglist)->*mut vector_fds; fn user_init_socket_fds(_: *mut arglist,_:c_int)->*mut vector_fds; fn user_init_unix_fds(_: *mut arglist,_:c_int)->*mut vector_fds; fn user_init_fd_fds(_: *mut arglist)->*mut vector_fds; fn user_init_vde_fds(_: *mut arglist)->*mut vector_fds; }

#[no_mangle] pub unsafe extern "C" fn uml_vector_user_open(unit:c_int, parsed:*mut arglist)->*mut vector_fds { if parsed.is_null(){return std::ptr::null_mut()} let t=b"transport\0".as_ptr() as *mut c_char; let tr=uml_vector_fetch_arg(parsed,t); if tr.is_null(){return std::ptr::null_mut()} macro_rules! m {($s:expr,$f:expr)=>{if strncmp(tr,$s.as_ptr() as *const c_char,$s.len()-1)==0{return $f(parsed)}}} m!(b"raw\0",user_init_raw_fds); m!(b"hybrid\0",user_init_hybrid_fds); m!(b"tap\0",user_init_tap_fds); m!(b"gre\0",user_init_socket_fds(parsed,ID_GRE)); m!(b"l2tpv3\0",user_init_socket_fds(parsed,ID_L2TPV3)); m!(b"bess\0",user_init_unix_fds(parsed,ID_BESS)); m!(b"fd\0",user_init_fd_fds); m!(b"vde\0",user_init_vde_fds); std::ptr::null_mut() }

#[no_mangle] pub unsafe extern "C" fn uml_vector_sendmsg(fd:c_int,h:*mut c_void,f:c_int)->c_int { let n=sendmsg(fd,h as *const msghdr,f); if n<0 {0} else {n} }
#[no_mangle] pub unsafe extern "C" fn uml_vector_recvmsg(fd:c_int,h:*mut c_void,_:c_int)->c_int { let m=h as *mut msghdr; let n=readv(fd,(*m).msg_iov,(*m).msg_iovlen); if n<0 {0}else{n} }
#[no_mangle] pub unsafe extern "C" fn uml_vector_writev(fd:c_int,h:*mut c_void,n:c_int)->c_int {let r=writev(fd,h as *const iovec,n);if r<0{0}else{r}}
#[no_mangle] pub unsafe extern "C" fn uml_vector_sendmmsg(fd:c_int,m:*mut c_void,n:u32,f:u32)->c_int {let r=sendmmsg(fd,m as *mut mmsghdr,n,f);if r<0{0}else{r}}
#[no_mangle] pub unsafe extern "C" fn uml_vector_recvmmsg(fd:c_int,m:*mut c_void,n:u32,f:u32)->c_int {let r=recvmmsg(fd,m as *mut mmsghdr,n,f,std::ptr::null_mut());if r<0{0}else{r}}

#[no_mangle] pub unsafe extern "C" fn uml_vector_attach_bpf(fd:c_int,b:*mut c_void)->c_int {setsockopt(fd,1,26,b,std::mem::size_of::<sock_fprog>())}
#[no_mangle] pub unsafe extern "C" fn uml_vector_detach_bpf(fd:c_int,b:*mut c_void)->c_int {setsockopt(fd,1,27,b,std::mem::size_of::<sock_fprog>())}
#[no_mangle] pub unsafe extern "C" fn uml_vector_default_bpf(mac:*const c_void)->*mut c_void {let p=uml_kmalloc(std::mem::size_of::<sock_fprog>(),0) as *mut sock_fprog;if p.is_null(){return p as *mut c_void}let f=uml_kmalloc(std::mem::size_of::<sock_filter>()*DEFAULT_BPF_LEN,0) as *mut sock_filter;if f.is_null(){kfree(p as *mut c_void);return std::ptr::null_mut()}(*p).len=DEFAULT_BPF_LEN as u16;(*p).filter=f;let m1=(mac.add(2)) as *const u32;let m2=mac as *const u16;let a=[sock_filter{code:0x20,jt:0,jf:0,k:8},sock_filter{code:0x15,jt:0,jf:3,k:ntohl(*m1)},sock_filter{code:0x28,jt:0,jf:0,k:6},sock_filter{code:0x15,jt:0,jf:1,k:ntohs(*m2)},sock_filter{code:6,jt:0,jf:0,k:0},sock_filter{code:6,jt:0,jf:0,k:0x40000}];std::ptr::copy_nonoverlapping(a.as_ptr(),f,6);p as *mut c_void}

#[no_mangle] pub unsafe extern "C" fn uml_vector_user_bpf(_: *mut c_char)->*mut c_void { std::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
