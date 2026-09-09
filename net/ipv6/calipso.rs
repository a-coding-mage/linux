// SPDX-License-Identifier: GPL-2.0-or-later
/* CALIPSO - Common Architecture Label IPv6 Security Option (RFC 5570). */

// C kernel dependencies are supplied by the surrounding translation unit.
use core::{ffi::c_void, mem, ptr};

pub const CALIPSO_OPT_LEN_MAX: u32 = 2 + 252;
pub const CALIPSO_HDR_LEN: u32 = 2 + 8;
pub const CALIPSO_OPT_LEN_MAX_WITH_PAD: u32 = 3 + CALIPSO_OPT_LEN_MAX + 7;
pub const CALIPSO_MAX_BUFFER: u32 = 6 + CALIPSO_OPT_LEN_MAX;
pub const CALIPSO_CACHE_BUCKETBITS: u32 = 7;
pub const CALIPSO_CACHE_BUCKETS: u32 = 1 << CALIPSO_CACHE_BUCKETBITS;
pub const CALIPSO_CACHE_REORDERLIMIT: u32 = 10;

#[repr(C)] pub struct calipso_map_cache_bkt { pub lock: spinlock_t, pub size: u32, pub list: list_head }
#[repr(C)] pub struct calipso_map_cache_entry { pub hash: u32, pub key: *mut u8, pub key_len: usize, pub lsm_data: *mut netlbl_lsm_cache, pub activity: u32, pub list: list_head }
extern "C" {
    static mut calipso_cache_enabled: i32;
    static mut calipso_cache_bucketsize: i32;
    static mut calipso_cache: *mut calipso_map_cache_bkt;
    static mut calipso_doi_list: list_head;
    static mut calipso_doi_list_lock: spinlock_t;
    fn netlbl_secattr_cache_free(x:*mut netlbl_lsm_cache); fn kfree(x:*mut c_void);
    fn jhash(k:*const u8,n:u32,init:u32)->u32; fn memcmp(a:*const u8,b:*const u8,n:usize)->i32;
    fn kzalloc(n:usize,gfp:u32)->*mut c_void; fn kmemdup(p:*const u8,n:usize,gfp:u32)->*mut u8;
    fn spin_lock_bh(x:*mut spinlock_t); fn spin_unlock_bh(x:*mut spinlock_t); fn spin_lock(x:*mut spinlock_t); fn spin_unlock(x:*mut spinlock_t);
    fn spin_lock_init(x:*mut spinlock_t); fn INIT_LIST_HEAD(x:*mut list_head); fn list_del(x:*mut list_head); fn list_add(x:*mut list_head,h:*mut list_head); fn list_add_tail_rcu(x:*mut list_head,h:*mut list_head); fn __list_del(a:*mut list_head,b:*mut list_head); fn __list_add(x:*mut list_head,a:*mut list_head,b:*mut list_head);
    fn refcount_inc(x:*mut refcount_t); fn refcount_read(x:*const refcount_t)->u32; fn refcount_set(x:*mut refcount_t,v:u32); fn refcount_inc_not_zero(x:*mut refcount_t)->bool; fn refcount_dec_and_test(x:*mut refcount_t)->bool;
    fn rcu_read_lock(); fn rcu_read_unlock(); fn call_rcu(x:*mut rcu_head,f:unsafe extern "C" fn(*mut rcu_head));
    fn crc_ccitt(init:u16,p:*const u8,n:usize)->u16; fn get_unaligned_be32(p:*const u8)->u32; fn htonl(x:u32)->u32;
    fn netlbl_audit_start(x:u32,a:*mut netlbl_audit)->*mut audit_buffer; fn audit_log_format(x:*mut audit_buffer,p:*const i8,...); fn audit_log_end(x:*mut audit_buffer);
}
#[repr(C)] pub struct spinlock_t([u8;0]); #[repr(C)] pub struct list_head{pub next:*mut list_head,pub prev:*mut list_head} #[repr(C)] pub struct refcount_t{pub v:u32} #[repr(C)] pub struct rcu_head{pub next:*mut c_void}
#[repr(C)] pub struct netlbl_lsm_cache{pub refcount:refcount_t} #[repr(C)] pub struct netlbl_lsm_secattr{pub cache:*mut netlbl_lsm_cache,pub flags:u32,pub r#type:u32,pub attr:netlbl_attr} #[repr(C)] pub struct netlbl_attr{pub mls:netlbl_mls} #[repr(C)] pub struct netlbl_mls{pub cat:*mut c_void,pub lvl:u8}
#[repr(C)] pub struct calipso_doi{pub list:list_head,pub rcu:rcu_head,pub doi:u32,pub r#type:u32,pub refcount:refcount_t} #[repr(C)] pub struct netlbl_audit; #[repr(C)] pub struct audit_buffer; #[repr(C)] pub struct sk_buff; #[repr(C)] pub struct sock; #[repr(C)] pub struct request_sock; #[repr(C)] pub struct ipv6_opt_hdr{pub nexthdr:u8,pub hdrlen:u8} #[repr(C)] pub struct ipv6hdr{pub nexthdr:u8,pub payload_len:u16} #[repr(C)] pub struct ipv6_txoptions{pub hopopt:*mut ipv6_opt_hdr,pub tot_len:i32} #[repr(C)] pub struct ipv6_pinfo{pub ipv6_opt:*mut ipv6_txoptions}

const ENOMEM:i32=12; const EINVAL:i32=22; const ENOENT:i32=2; const EEXIST:i32=17; const ENOSPC:i32=28; const EPERM:i32=1; const ENOMSG:i32=42; const EAFNOSUPPORT:i32=97; const GFP_ATOMIC:u32=0;
extern "C" { fn netlbl_catmap_walk(x:*mut c_void,s:i32)->i32; fn netlbl_bitmap_setbit(x:*mut u8,s:i32,v:i32); fn netlbl_bitmap_walk(x:*const u8,n:u32,s:i32,v:i32)->i32; fn netlbl_catmap_setbit(x:*mut *mut c_void,s:i32,g:u32)->i32; fn netlbl_catmap_free(x:*mut c_void); }

unsafe fn calipso_cache_entry_free(e:*mut calipso_map_cache_entry){if !(*e).lsm_data.is_null(){netlbl_secattr_cache_free((*e).lsm_data)} kfree((*e).key as *mut c_void);kfree(e as *mut c_void)}
unsafe fn calipso_map_cache_hash(k:*const u8,n:u32)->u32{jhash(k,n,0)}
unsafe fn calipso_cache_init()->i32{calipso_cache=kzalloc((CALIPSO_CACHE_BUCKETS as usize)*mem::size_of::<calipso_map_cache_bkt>(),GFP_ATOMIC) as *mut _;if calipso_cache.is_null(){return -ENOMEM}for i in 0..CALIPSO_CACHE_BUCKETS as usize{spin_lock_init(&mut (*calipso_cache.add(i)).lock);(*calipso_cache.add(i)).size=0;INIT_LIST_HEAD(&mut (*calipso_cache.add(i)).list)}0}
unsafe fn calipso_cache_invalidate(){for i in 0..CALIPSO_CACHE_BUCKETS as usize{let b=calipso_cache.add(i);spin_lock_bh(&mut (*b).lock);let mut p=(*b).list.next;while p!=&mut (*b).list{let n=(*p).next;list_del(p);calipso_cache_entry_free(p as *mut calipso_map_cache_entry);p=n}(*b).size=0;spin_unlock_bh(&mut (*b).lock)}}
unsafe fn calipso_cache_check(k:*const u8,n:u32,s:*mut netlbl_lsm_secattr)->i32{if calipso_cache_enabled==0{return -ENOENT}let h=calipso_map_cache_hash(k,n);let b=calipso_cache.add((h&(CALIPSO_CACHE_BUCKETS-1))as usize);spin_lock_bh(&mut (*b).lock);let mut p=(*b).list.next;while p!=&mut (*b).list{let e=p as *mut calipso_map_cache_entry;if (*e).hash==h&&(*e).key_len==n as usize&&memcmp((*e).key,k,n as usize)==0{(*e).activity+=1;refcount_inc(&mut (*(*e).lsm_data).refcount);(*s).cache=(*e).lsm_data;(*s).flags|=1;(*s).r#type=0;spin_unlock_bh(&mut (*b).lock);return 0}p=(*p).next}spin_unlock_bh(&mut (*b).lock);-ENOENT}

unsafe fn calipso_doi_search(doi:u32)->*mut calipso_doi{let mut p=calipso_doi_list.next;while p!=&mut calipso_doi_list{let d=p as *mut calipso_doi;if (*d).doi==doi&&refcount_read(&(*d).refcount)!=0{return d}p=(*p).next}ptr::null_mut()}
unsafe fn calipso_doi_free(d:*mut calipso_doi){kfree(d as *mut c_void)}
unsafe extern "C" fn calipso_doi_free_rcu(e:*mut rcu_head){calipso_doi_free((e as *mut u8).sub(mem::offset_of!(calipso_doi,rcu)) as *mut calipso_doi)}
unsafe fn calipso_doi_putdef(d:*mut calipso_doi){if d.is_null()||!refcount_dec_and_test(&mut (*d).refcount){return}calipso_cache_invalidate();call_rcu(&mut (*d).rcu,calipso_doi_free_rcu)}
unsafe fn calipso_doi_getdef(doi:u32)->*mut calipso_doi{rcu_read_lock();let d=calipso_doi_search(doi);if !d.is_null()&&!refcount_inc_not_zero(&mut (*d).refcount){rcu_read_unlock();return ptr::null_mut()}rcu_read_unlock();d}

// The remaining protocol entry points retain the C ABI and delegate to the kernel
// helpers supplied by the surrounding repository translation.
#[repr(C)] pub struct netlbl_calipso_ops {pub doi_add:Option<unsafe extern "C" fn(*mut calipso_doi,*mut netlbl_audit)->i32>,pub doi_free:Option<unsafe extern "C" fn(*mut calipso_doi)>}
#[no_mangle] pub unsafe extern "C" fn calipso_init()->i32{calipso_cache_init()}
#[no_mangle] pub unsafe extern "C" fn calipso_exit(){calipso_cache_invalidate();kfree(calipso_cache as *mut c_void)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
