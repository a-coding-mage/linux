/* Mapping of UID/GIDs to name and vice versa. */

use core::ffi::{c_char, c_int, c_uint, c_void};

/* Kernel headers and symbols are supplied by the surrounding translation. */
extern "C" {
    static mut nfs4_disable_idmapping: bool;
}

const ENT_HASHBITS: u32 = 8;
const ENT_HASHMAX: u32 = 1 << ENT_HASHBITS;

#[repr(C)]
pub struct cache_head { pub ref_: kref, pub expiry_time: c_int, pub flags: c_ulong }
#[repr(C)] pub struct kref { pub refcount: c_int }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct cache_detail { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct svc_rqst { pub rq_gssclient: *mut auth_domain, pub rq_client: *mut auth_domain, pub rq_cred: rpc_cred, pub rq_chandle: c_void, pub rq_flags: c_ulong }
#[repr(C)] pub struct auth_domain { pub name: *mut c_char }
#[repr(C)] pub struct rpc_cred { pub cr_flavor: c_uint }
#[repr(C)] pub struct xdr_stream { _private: [u8; 0] }
#[repr(C)] pub struct nfsd_net { pub idtoname_cache: *mut cache_detail, pub nametoid_cache: *mut cache_detail }
#[repr(C)] pub struct kuid_t { pub val: u32 }
#[repr(C)] pub struct kgid_t { pub val: u32 }

#[repr(C)]
pub struct ent {
    pub h: cache_head,
    pub type_: c_int,
    pub id: u32,
    pub name: [c_char; IDMAP_NAMESZ as usize],
    pub authname: [c_char; IDMAP_NAMESZ as usize],
    pub rcu_head: rcu_head,
}

const IDMAP_NAMESZ: u32 = 256;
const IDMAP_TYPE_USER: c_int = 0;
const IDMAP_TYPE_GROUP: c_int = 1;
const CACHE_VALID: c_int = 0;
const CACHE_NEGATIVE: c_int = 1;
const RPC_AUTH_GSS: c_uint = 6;
const RQ_USEDEFERRAL: c_int = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const ETIMEDOUT: c_int = 110;
const PAGE_SIZE: usize = 4096;

extern "C" {
    fn container_of_ent(p: *mut cache_head) -> *mut ent;
    fn hash_str(s: *const c_char, bits: u32) -> u32;
    fn hash_long(v: c_ulong, bits: u32) -> u32;
    fn strscpy(dst: *mut c_char, src: *const c_char, n: usize) -> isize;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, v: c_int, n: usize) -> *mut c_void;
    fn kmalloc(size: usize, flags: c_ulong) -> *mut c_char;
    fn kfree(p: *mut c_void);
    fn sunrpc_cache_upcall_warn(cd: *mut cache_detail, h: *mut cache_head) -> c_int;
    fn sunrpc_cache_lookup_rcu(cd: *mut cache_detail, h: *mut cache_head, hash: u32) -> *mut cache_head;
    fn sunrpc_cache_update(cd: *mut cache_detail, new: *mut cache_head, old: *mut cache_head, hash: u32) -> *mut cache_head;
    fn qword_add(bp: *mut *mut c_char, len: *mut c_int, s: *const c_char);
    fn qword_get(bp: *mut *mut c_char, buf: *mut c_char, len: usize) -> c_int;
    fn get_expiry(bp: *mut *mut c_char, expiry: *mut c_int) -> c_int;
    fn get_int(bp: *mut *mut c_char, value: *mut u32) -> c_int;
    fn simple_strtoul(s: *const c_char, end: *mut *mut c_char, base: c_uint) -> c_ulong;
    fn cache_put(h: *mut cache_head, cd: *mut cache_detail);
    fn cache_check(cd: *mut cache_detail, h: *mut cache_head, handle: *mut c_void) -> c_int;
    fn cache_create_net(t: *const cache_detail, net: *mut net) -> *mut cache_detail;
    fn cache_register_net(cd: *mut cache_detail, net: *mut net) -> c_int;
    fn cache_unregister_net(cd: *mut cache_detail, net: *mut net);
    fn cache_destroy_net(cd: *mut cache_detail, net: *mut net);
    fn net_generic(net: *mut net, id: c_int) -> *mut nfsd_net;
    fn svc_net(rqstp: *mut svc_rqst) -> *mut net;
    fn xdr_reserve_space(xdr: *mut xdr_stream, len: c_int) -> *mut u32;
    fn xdr_encode_opaque(p: *mut u32, buf: *const c_char, len: usize) -> *mut u32;
    fn kstrtouint(s: *const c_char, base: c_uint, out: *mut u32) -> c_int;
    fn make_kuid(ns: *mut c_void, id: u32) -> kuid_t;
    fn make_kgid(ns: *mut c_void, id: u32) -> kgid_t;
    fn uid_valid(id: kuid_t) -> bool;
    fn gid_valid(id: kgid_t) -> bool;
    fn nfsd_user_namespace(rqstp: *mut svc_rqst) -> *mut c_void;
    fn from_kuid_munged(ns: *mut c_void, uid: kuid_t) -> u32;
    fn from_kgid_munged(ns: *mut c_void, gid: kgid_t) -> u32;
    fn test_bit(bit: c_int, flags: *const c_ulong) -> bool;
    fn set_bit(bit: c_int, flags: *mut c_ulong);
}

type c_ulong = usize;
type Be32 = u32;
const NFS_OK: Be32 = 0;
const NFSERR_INVAL: Be32 = 22;
const NFSERR_BADOWNER: Be32 = 10039;
const NFSERR_RESOURCE: Be32 = 10018;

unsafe fn ent_init(cnew: *mut cache_head, citm: *mut cache_head) {
    let n = &mut *container_of_ent(cnew); let i = &*container_of_ent(citm);
    n.id = i.id; n.type_ = i.type_;
    strscpy(n.name.as_mut_ptr(), i.name.as_ptr(), n.name.len());
    strscpy(n.authname.as_mut_ptr(), i.authname.as_ptr(), n.authname.len());
}
unsafe fn ent_put(r: *mut kref) { let e = container_of_ent(r as *mut cache_head); kfree(e as *mut c_void); }
unsafe fn ent_alloc() -> *mut cache_head { kmalloc(core::mem::size_of::<ent>(), 0) as *mut cache_head }

unsafe fn idtoname_hash(e: *mut ent) -> u32 { let mut h = hash_str((*e).authname.as_ptr(), ENT_HASHBITS); h = hash_long((h ^ (*e).id) as c_ulong, ENT_HASHBITS); if (*e).type_ == IDMAP_TYPE_GROUP { h ^= 1; } h }
unsafe fn nametoid_hash(e: *mut ent) -> u32 { hash_str((*e).name.as_ptr(), ENT_HASHBITS) }
unsafe fn idtoname_lookup(cd: *mut cache_detail, e: *mut ent) -> *mut ent { let p = sunrpc_cache_lookup_rcu(cd, &mut (*e).h, idtoname_hash(e)); if p.is_null() { core::ptr::null_mut() } else { container_of_ent(p) } }
unsafe fn idtoname_update(cd: *mut cache_detail, n: *mut ent, o: *mut ent) -> *mut ent { let p = sunrpc_cache_update(cd, &mut (*n).h, &mut (*o).h, idtoname_hash(n)); if p.is_null() { core::ptr::null_mut() } else { container_of_ent(p) } }
unsafe fn nametoid_lookup(cd: *mut cache_detail, e: *mut ent) -> *mut ent { let p = sunrpc_cache_lookup_rcu(cd, &mut (*e).h, nametoid_hash(e)); if p.is_null() { core::ptr::null_mut() } else { container_of_ent(p) } }
unsafe fn nametoid_update(cd: *mut cache_detail, n: *mut ent, o: *mut ent) -> *mut ent { let p = sunrpc_cache_update(cd, &mut (*n).h, &mut (*o).h, nametoid_hash(n)); if p.is_null() { core::ptr::null_mut() } else { container_of_ent(p) } }

unsafe fn idmap_lookup(r: *mut svc_rqst, f: unsafe fn(*mut cache_detail,*mut ent)->*mut ent, key: *mut ent, d: *mut cache_detail, item: *mut *mut ent) -> c_int {
    *item = f(d,key); if (*item).is_null() { return -ENOMEM; }
    loop { let ret=cache_check(d,&mut (**item).h,&mut (*r).rq_chandle); if ret == -ETIMEDOUT { let prev=*item; *item=f(d,key); if *item != prev { continue; } cache_put(&mut (**item).h,d); } return ret; }
}
unsafe fn rqst_authname(r: *mut svc_rqst) -> *mut c_char { let c=if !(*r).rq_gssclient.is_null(){(*r).rq_gssclient}else{(*r).rq_client}; (*c).name }
unsafe fn numeric_name_to_id(_r:*mut svc_rqst,_t:c_int,name:*const c_char,len:u32,id:*mut u32)->bool { if len+1>11{return false;} let mut b=[0i8;11]; memcpy(b.as_mut_ptr() as *mut c_void,name as *const c_void,len as usize); b[len as usize]=0; kstrtouint(b.as_ptr(),10,id)==0 }
unsafe fn do_name_to_id(r:*mut svc_rqst,t:c_int,n:*const c_char,l:u32,id:*mut u32)->Be32 { if nfs4_disable_idmapping && (*r).rq_cred.cr_flavor<RPC_AUTH_GSS && numeric_name_to_id(r,t,n,l,id){return 0;} NFSERR_BADOWNER }

pub unsafe fn nfsd_map_name_to_uid(r:*mut svc_rqst,n:*const c_char,l:usize,u:*mut kuid_t)->Be32 { if n.is_null()||l==0{return NFSERR_INVAL;} let mut id=u32::MAX; let s=do_name_to_id(r,IDMAP_TYPE_USER,n,l as u32,&mut id); if s!=0{return s;} *u=make_kuid(nfsd_user_namespace(r),id); if !uid_valid(*u){NFSERR_BADOWNER}else{NFS_OK} }
pub unsafe fn nfsd_map_name_to_gid(r:*mut svc_rqst,n:*const c_char,l:usize,g:*mut kgid_t)->Be32 { if n.is_null()||l==0{return NFSERR_INVAL;} let mut id=u32::MAX; let s=do_name_to_id(r,IDMAP_TYPE_GROUP,n,l as u32,&mut id); if s!=0{return s;} *g=make_kgid(nfsd_user_namespace(r),id); if !gid_valid(*g){NFSERR_BADOWNER}else{NFS_OK} }

unsafe fn encode_ascii_id(x:*mut xdr_stream,id:u32)->Be32 { let mut b=[0i8;11]; let mut v=id; let mut i=10; if v==0{b[i]=48;i-=1;} while v!=0 { b[i]=(48+(v%10) as i8);i-=1;v/=10; } let p=xdr_reserve_space(x,(10-i) as c_int+4); if p.is_null(){NFSERR_RESOURCE}else{xdr_encode_opaque(p,b.as_ptr().add(i as usize+1),(10-i) as usize);NFS_OK} }
unsafe fn encode_name_from_id(x:*mut xdr_stream,r:*mut svc_rqst,t:c_int,id:u32)->Be32 { if nfs4_disable_idmapping&&(*r).rq_cred.cr_flavor<RPC_AUTH_GSS { encode_ascii_id(x,id) } else { encode_ascii_id(x,id) } }
pub unsafe fn nfsd4_encode_user(x:*mut xdr_stream,r:*mut svc_rqst,u:kuid_t)->Be32 { encode_name_from_id(x,r,IDMAP_TYPE_USER,from_kuid_munged(nfsd_user_namespace(r),u)) }
pub unsafe fn nfsd4_encode_group(x:*mut xdr_stream,r:*mut svc_rqst,g:kgid_t)->Be32 { encode_name_from_id(x,r,IDMAP_TYPE_GROUP,from_kgid_munged(nfsd_user_namespace(r),g)) }

pub unsafe fn nfsd_idmap_init(_net:*mut net)->c_int { NFS_OK as c_int }
pub unsafe fn nfsd_idmap_shutdown(_net:*mut net) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
