// SPDX-License-Identifier: GPL-2.0-only
/* FTP extension for connection tracking. */
/* (C) 1999-2001 Paul `Rusty' Russell
 * (C) 2002-2004 Netfilter Core Team <coreteam@netfilter.org>
 * (C) 2003,2004 USAGI/WIDE Project <http://www.linux-ipv6.org>
 * (C) 2006-2012 Patrick McHardy <kaber@trash.net>
 */

/* Kernel dependencies supplied by the surrounding translation unit. */

const HELPER_NAME: &str = "ftp";

static mut loose: bool = false;
static mut nf_nat_ftp_hook: *mut nf_nat_ftp_hook_fn = core::ptr::null_mut();

type nf_nat_ftp_hook_fn = unsafe extern "C" fn(
    *mut sk_buff, *mut nf_conn, nf_conntrack_info, nf_ct_ftp_type,
    c_uint, c_uint, c_uint, *mut nf_conntrack_expect,
) -> c_int;

extern "C" {
    fn in6_pton(src: *const c_char, srclen: c_int, dst: *mut u8, delim: c_int,
                end: *mut *const c_char) -> c_int;
    fn htonl(x: u32) -> u32;
    fn htons(x: u16) -> u16;
    fn ntohl(x: u32) -> u32;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> c_int;
    fn strncasecmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
    fn isdigit(c: c_int) -> c_int;
    fn skb_linearize(skb: *mut sk_buff) -> c_int;
    fn skb_header_pointer(skb: *mut sk_buff, offset: c_uint, len: usize,
                          buffer: *mut c_void) -> *const tcphdr;
    fn nf_ct_l3num(ct: *mut nf_conn) -> u16;
    fn nfct_help_data(ct: *mut nf_conn) -> *mut nf_ct_ftp_master;
    fn nf_ct_expect_alloc(ct: *mut nf_conn) -> *mut nf_conntrack_expect;
    fn nf_ct_expect_put(exp: *mut nf_conntrack_expect);
    fn nf_ct_expect_init(exp: *mut nf_conntrack_expect, class: c_uint,
                         l3num: u16, src: *const nf_inet_addr,
                         dst: *const nf_inet_addr, proto: u8,
                         src_port: *const u16, dst_port: *const u16);
    fn nf_ct_expect_related(exp: *mut nf_conntrack_expect, flags: c_uint) -> c_int;
    fn nf_ct_helper_log(skb: *mut sk_buff, ct: *mut nf_conn, fmt: *const c_char, ...);
    fn nf_conntrack_helper_register(h: *mut nf_conntrack_helper,
                                    ptr: *mut *mut nf_conntrack_helper) -> c_int;
    fn nf_conntrack_helper_unregister(h: *mut nf_conntrack_helper);
    fn nf_ct_helper_init(h: *mut nf_conntrack_helper, l3num: u16, proto: u8,
                         name: *const c_char, policy: *const nf_conntrack_expect_policy,
                         flags: c_uint, help: Option<unsafe extern "C" fn(*mut sk_buff,c_uint,*mut nf_conn,nf_conntrack_info)->c_int>,
                         from_nlattr: Option<unsafe extern "C" fn(*mut nlattr,*mut nf_conn)->c_int>,
                         module: *mut c_void);
    fn spin_lock_bh(lock: *mut c_void);
    fn spin_unlock_bh(lock: *mut c_void);
    fn rcu_dereference<T>(p: *mut T) -> *mut T;
}

type c_char = i8; type c_int = i32; type c_uint = u32; type c_void = core::ffi::c_void;
type nf_conntrack_info = u32;

#[repr(C)] pub struct nf_inet_addr { pub all: [u32; 4] }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub union nf_conntrack_man_u3 { pub ip: u32, pub ip6: [u32; 4], pub all: [u32; 4] }
#[repr(C)] pub union nf_conntrack_man_u { pub tcp: nf_conntrack_man_tcp }
#[repr(C)] pub struct nf_conntrack_man_tcp { pub port: u16 }
#[repr(C)] pub struct nf_conntrack_man { pub l3num: u16, pub u3: nf_conntrack_man_u3, pub u: nf_conntrack_man_u }
#[repr(C)] pub struct nf_ct_ftp_master { pub seq_aft_nl: [[u32; 2]; 2], pub seq_aft_nl_num: [u32; 2], pub flags: [u32; 2] }
#[repr(C)] pub struct tcphdr { pub doff: u16, pub seq: u32 }
#[repr(C)] pub struct sk_buff { pub data: *mut c_char, pub len: u32 }
#[repr(C)] pub struct nf_conn { pub status: u32, pub tuplehash: [nf_conntrack_tuplehash; 2] }
#[repr(C)] pub struct nf_conntrack_tuplehash { pub tuple: nf_conntrack_tuple }
#[repr(C)] pub struct nf_conntrack_tuple { pub src: nf_conntrack_man, pub dst: nf_conntrack_man }
#[repr(C)] pub struct nf_conntrack_expect { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_helper { _private: [u8; 0] }
#[repr(C)] pub struct nf_conntrack_expect_policy { pub max_expected: u32, pub timeout: u32 }

#[repr(C)] #[derive(Copy, Clone)] pub enum nf_ct_ftp_type { NF_CT_FTP_PORT, NF_CT_FTP_PASV, NF_CT_FTP_EPRT, NF_CT_FTP_EPSV }

static mut search: [[ftp_search; 2]; 2] = unsafe { core::mem::zeroed() };
#[repr(C)] struct ftp_search { pattern: *const c_char, plen: usize, skip: c_char, term: c_char, ftptype: nf_ct_ftp_type, getnum: Option<unsafe extern "C" fn(*const c_char,usize,*mut nf_conntrack_man,c_char,*mut c_uint)->c_int> }

unsafe extern "C" fn get_ipv6_addr(src:*const c_char,dlen:usize,dst:*mut in6_addr,term:u8)->c_int { let mut end=core::ptr::null(); let ret=in6_pton(src,core::cmp::min(dlen,0xffff) as c_int,dst as *mut u8,term as c_int,&mut end); if ret>0 {(end as isize-src as isize) as c_int} else {0} }
unsafe extern "C" fn try_number(mut data:*const c_char,dlen:usize,array:*mut u32,array_size:c_int,sep:c_char,term:c_char)->c_int { memset(array as *mut c_void,0,(4*array_size) as usize); let(mut i,mut len)=(0,0); while len<dlen && i<array_size as usize { let ch=*data as u8; if ch>=b'0'&&ch<=b'9' { *array.add(i)=(*array.add(i))*10+(ch-b'0') as u32; if *array.add(i)>255{return 0} } else if *data==sep {i+=1} else {if (*data==term||term==0)&&i==array_size as usize-1{return len as c_int} return 0} len+=1;data=data.add(1) } 0 }
unsafe extern "C" fn try_rfc959(data:*const c_char,dlen:usize,cmd:*mut nf_conntrack_man,term:c_char,_:*mut c_uint)->c_int { let mut a=[0u32;6]; let l=try_number(data,dlen,a.as_mut_ptr(),6,b',',term); if l==0{return 0} (*cmd).u3.ip=htonl((a[0]<<24)|(a[1]<<16)|(a[2]<<8)|a[3]); (*cmd).u.tcp.port=htons(((a[4]<<8)|a[5]) as u16); l }
unsafe extern "C" fn try_rfc1123(data:*const c_char,dlen:usize,cmd:*mut nf_conntrack_man,term:c_char,offset:*mut c_uint)->c_int { let mut i=0; while i<dlen && isdigit(*data.add(i) as c_int)==0{i+=1} if i==dlen{return 0} *offset+=i as u32; try_rfc959(data.add(i),dlen-i,cmd,0,offset) }
unsafe extern "C" fn get_port(data:*const c_char,start:c_int,dlen:usize,delim:c_char,port:*mut u16)->c_int { let(mut p,mut i)=(0u32,start as usize); while i<dlen {let c=*data.add(i); if c==delim {if p==0{break} *port=htons(p as u16);return (i+1) as c_int} else if c>=b'0' as i8&&c<=b'9' as i8 {p=p*10+(c-b'0' as i8) as u32;if p>65535{break}} else {break} i+=1} 0 }
unsafe extern "C" fn try_eprt(data:*const c_char,dlen:usize,cmd:*mut nf_conntrack_man,_:c_char,_:*mut c_uint)->c_int {if dlen<=3{return 0} let delim=*data;if isdigit(delim as c_int)!=0||delim<33||delim>126||*data.add(2)!=delim{return 0} let mut a=[0u32;4];let l=if *data.add(1)==b'1' as i8{let x=try_number(data.add(3),dlen-3,a.as_mut_ptr(),4,b'.',delim);if x!=0{(*cmd).u3.ip=htonl((a[0]<<24)|(a[1]<<16)|(a[2]<<8)|a[3])}x}else{get_ipv6_addr(data.add(3),dlen-3,&mut *( (*cmd).u3.ip6.as_mut_ptr() as *mut in6_addr),delim as u8)};if l==0{0}else{get_port(data,4+l,dlen,delim,&mut (*cmd).u.tcp.port)} }
unsafe extern "C" fn try_epsv_response(data:*const c_char,dlen:usize,cmd:*mut nf_conntrack_man,_:c_char,_:*mut c_uint)->c_int {if dlen<=3{return 0}let d=*data;if isdigit(d as c_int)!=0||d<33||d>126||*data.add(1)!=d||*data.add(2)!=d{return 0}get_port(data,3,dlen,d,&mut (*cmd).u.tcp.port)}

unsafe extern "C" fn find_pattern(data:*const c_char,dlen:usize,pattern:*const c_char,plen:usize,skip:c_char,term:c_char,numoff:*mut c_uint,numlen:*mut c_uint,cmd:*mut nf_conntrack_man,getnum:Option<unsafe extern "C" fn(*const c_char,usize,*mut nf_conntrack_man,c_char,*mut c_uint)->c_int>)->c_int { let mut i=plen;if dlen<=plen{return if strncasecmp(data,pattern,dlen)==0{-1}else{0}} if strncasecmp(data,pattern,plen)!=0{return 0} if skip!=0 {while *data.add(i)!=skip {if i==dlen-1{return -1}i+=1}i+=1}*numoff=i as u32;*numlen=getnum.unwrap()(data.add(i),dlen-i,cmd,term,numoff) as u32;if *numlen==0{-1}else{1} }
unsafe extern "C" fn find_nl_seq(seq:u32,info:*const nf_ct_ftp_master,dir:usize)->c_int {for i in 0..(*info).seq_aft_nl_num[dir] as usize{if (*info).seq_aft_nl[dir][i]==seq{return 1}}0}
unsafe extern "C" fn update_nl_seq(_ct:*mut nf_conn,nl:u32,info:*mut nf_ct_ftp_master,dir:usize,_skb:*mut sk_buff){for i in 0..(*info).seq_aft_nl_num[dir] as usize{if (*info).seq_aft_nl[dir][i]==nl{return}}let n=(*info).seq_aft_nl_num[dir] as usize;if n<2{(*info).seq_aft_nl[dir][n]=nl;(*info).seq_aft_nl_num[dir]+=1}else{(*info).seq_aft_nl[dir][0]=nl}}

unsafe extern "C" fn help(skb:*mut sk_buff,_protoff:c_uint,ct:*mut nf_conn,ctinfo:nf_conntrack_info)->c_int { let info=nfct_help_data(ct);if info.is_null(){return 0} if ctinfo!=ESTABLISHED&&ctinfo!=ESTABLISHED_REPLY{return 1} if skb_linearize(skb)!=0{return 0} 1 }
unsafe extern "C" fn nf_ct_ftp_from_nlattr(_attr:*mut nlattr,ct:*mut nf_conn)->c_int {let ftp=nfct_help_data(ct);if ftp.is_null(){return -2}(*ftp).flags[0]|=1;(*ftp).flags[1]|=1;0}
const ESTABLISHED:nf_conntrack_info=1;const ESTABLISHED_REPLY:nf_conntrack_info=2;

static mut ftp: *mut nf_conntrack_helper = core::ptr::null_mut();
static mut ftp_ptr: *mut nf_conntrack_helper = core::ptr::null_mut();
static ftp_exp_policy:nf_conntrack_expect_policy=nf_conntrack_expect_policy{max_expected:1,timeout:5*60};
unsafe extern "C" fn nf_conntrack_ftp_fini(){nf_conntrack_helper_unregister(ftp_ptr)}
unsafe extern "C" fn nf_conntrack_ftp_init()->c_int {nf_conntrack_helper_register(ftp, &mut ftp_ptr)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
