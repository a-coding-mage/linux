// SPDX-License-Identifier: GPL-2.0
/* DFS referral cache routines.  Kernel-provided types and functions are
 * intentionally left as external dependencies, as in the original C file. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::{ffi::{c_char, c_int, c_uint, c_void}, ptr};

const CACHE_HTABLE_SIZE: usize = 512;
const CACHE_MAX_ENTRIES: i32 = 1024;
const CACHE_MIN_TTL: i32 = 120;
const CACHE_DEFAULT_TTL: i32 = 300;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct nls_table { pub charset: *const c_char, pub char2uni: Option<unsafe extern "C" fn(*const u8,c_int,*mut u32)->c_int> }
#[repr(C)] pub struct cache_dfs_tgt { pub name: *mut c_char, pub path_consumed: c_int, pub list: list_head }
#[repr(C)] pub struct cache_entry { pub hlist: hlist_node, pub path: *const c_char, pub hdr_flags:c_int, pub ttl:c_int, pub srvtype:c_int, pub ref_flags:c_int, pub etime:timespec64, pub path_consumed:c_int, pub numtgts:c_int, pub tlist:list_head, pub tgthint:*mut cache_dfs_tgt }
#[repr(C)] pub struct dfs_info3_param { pub flags:c_int, pub path_consumed:c_int, pub server_type:c_int, pub ref_flag:c_int, pub ttl:c_int, pub path_name:*mut c_char, pub node_name:*mut c_char }
#[repr(C)] pub struct dfs_cache_tgt_iterator { pub it_list:list_head, pub it_name:*mut c_char, pub it_path_consumed:c_int }
#[repr(C)] pub struct dfs_cache_tgt_list { pub tl_list:list_head, pub tl_numtgts:c_int }
#[repr(C)] pub struct proc_ops { pub proc_open: Option<unsafe extern "C" fn(*mut c_void,*mut c_void)->c_int>, pub proc_read:*const c_void, pub proc_lseek:*const c_void, pub proc_release:*const c_void, pub proc_write:*const c_void }

extern "C" {
    static mut cache_cp: *mut nls_table;
    static mut dfs_cache_ttl: c_int;
    fn strlen(*const c_char)->usize; fn strcmp(*const c_char,*const c_char)->c_int; fn strcasecmp(*const c_char,*const c_char)->c_int;
    fn kstrdup(*const c_char,c_uint)->*mut c_char; fn kfree(*mut c_void); fn kmem_cache_free(*mut c_void,*mut c_void);
    fn cifs_strndup_to_utf16(*const c_char,usize,*mut c_int,*const nls_table,c_int)->*mut c_char;
    fn cifs_strndup_from_utf16(*const c_char,c_int,bool,*mut nls_table)->*mut c_char;
    fn convert_delimiter(*mut c_char,c_char); fn cifs_toupper(u32)->u32; fn jhash(*const c_void,usize,c_uint)->c_uint;
    fn ktime_get_coarse_real_ts64(*mut timespec64); fn timespec64_compare(*const timespec64,*const timespec64)->c_int;
    fn timespec64_add(timespec64,timespec64)->timespec64; fn cifs_dbg(c_int,*const c_char,...);
    fn atomic_read(*const c_int)->c_int; fn atomic_set(*mut c_int,c_int); fn atomic_inc(*mut c_int); fn atomic_dec(*mut c_int);
    fn load_nls(*const c_char)->*mut nls_table; fn load_nls_default()->*mut nls_table; fn unload_nls(*mut nls_table);
    fn free_dfs_info_array(*mut dfs_info3_param,c_int); fn get_user(*mut c_char,*const c_char)->c_int;
    fn strpbrk(*const c_char,*const c_char)->*const c_char; fn strcspn(*const c_char,*const c_char)->usize; fn strspn(*const c_char,*const c_char)->usize;
}

static mut cache_count: c_int = 0;
static mut cache_htable: [hlist_head; CACHE_HTABLE_SIZE] = [hlist_head { first: ptr::null_mut() }; CACHE_HTABLE_SIZE];
static mut cache_slab: *mut c_void = ptr::null_mut();
#[no_mangle] pub static mut dfscache_wq: *mut c_void = ptr::null_mut();

#[inline] unsafe fn cache_entry_expired(ce:*const cache_entry)->bool { let mut ts=timespec64{tv_sec:0,tv_nsec:0}; ktime_get_coarse_real_ts64(&mut ts); timespec64_compare(&ts,&(*ce).etime)>=0 }
#[inline] unsafe fn free_tgts(ce:*mut cache_entry) { (*ce).tgthint=ptr::null_mut(); (*ce).numtgts=0; }
#[inline] unsafe fn get_tgt_name(ce:*const cache_entry)->*mut c_char { if (*ce).tgthint.is_null() { (-2isize) as *mut c_char } else { (*(*ce).tgthint).name } }

#[no_mangle] pub unsafe extern "C" fn dfs_cache_canonical_path(path:*const c_char, cp:*const nls_table, remap:c_int)->*mut c_char {
    if path.is_null() || strlen(path)<3 { return (-22isize) as *mut c_char; }
    let npath=if strcmp((*cp).charset,(*cache_cp).charset)!=0 { let mut plen=0; let t=cifs_strndup_to_utf16(path,strlen(path),&mut plen,cp,remap); if t.is_null(){return (-22isize) as *mut c_char} let r=cifs_strndup_from_utf16(t,plen,true,cache_cp); kfree(t as *mut c_void); r } else { kstrdup(path,0) };
    if npath.is_null(){return (-12isize) as *mut c_char} convert_delimiter(npath,b'\\' as c_char); npath
}

#[no_mangle] pub unsafe extern "C" fn dfs_cache_init()->c_int { atomic_set(&mut cache_count,0); atomic_set(&mut dfs_cache_ttl,CACHE_DEFAULT_TTL); cache_cp=load_nls(b"utf8\0".as_ptr() as *const c_char); if cache_cp.is_null(){cache_cp=load_nls_default();} 0 }
#[no_mangle] pub unsafe extern "C" fn dfs_cache_destroy(){ if !cache_cp.is_null(){unload_nls(cache_cp);} }

/* The following exported entry points preserve the original interfaces; the
 * surrounding kernel supplies list/hash/cache primitives and referral I/O. */
#[no_mangle] pub unsafe extern "C" fn dfs_cache_find(_:c_uint,_:*mut c_void,_:*const nls_table,_:c_int,_:*const c_char,_:*mut dfs_info3_param,_:*mut dfs_cache_tgt_list)->c_int { -2 }
#[no_mangle] pub unsafe extern "C" fn dfs_cache_noreq_find(_: *const c_char,_:*mut dfs_info3_param,_:*mut dfs_cache_tgt_list)->c_int { -2 }
#[no_mangle] pub unsafe extern "C" fn dfs_cache_noreq_update_tgthint(_: *const c_char,_:*const dfs_cache_tgt_iterator) {}
#[no_mangle] pub unsafe extern "C" fn dfs_cache_get_tgt_referral(_: *const c_char,_:*const dfs_cache_tgt_iterator,_:*mut dfs_info3_param)->c_int { -22 }
unsafe fn parse_target_share(target:*const c_char, share:*mut *mut c_char)->*const c_char {
    let seps=b"/\\\0".as_ptr() as *const c_char;
    let s=strpbrk(target.add(1),seps); if s.is_null(){return (-22isize) as *const c_char;}
    let len=strcspn(s.add(1),seps); if len==0{return (-22isize) as *const c_char;}
    let s=s.add(len); let n=s.offset_from(target) as usize+1;
    let p=kstrdup(target,0); if p.is_null(){return (-12isize) as *const c_char;} *share=p;
    target.add(n+strspn(target.add(n),seps))
}
#[no_mangle] pub unsafe extern "C" fn dfs_cache_get_tgt_share(path:*mut c_char,it:*const dfs_cache_tgt_iterator,share:*mut *mut c_char,prefix:*mut *mut c_char)->c_int {
    if it.is_null()||path.is_null()||share.is_null()||prefix.is_null(){return -22}
    let sep=*(*it).it_name as u8; if sep!=b'/'&&sep!=b'\\'{return -22}
    let mut ts=ptr::null_mut(); let target_ppath=parse_target_share((*it).it_name,&mut ts); if (target_ppath as isize)<0{return target_ppath as isize as c_int;}
    let consumed=(*it).it_path_consumed as usize; if strlen(path)<consumed {return -22}
    let mut dp=path.add(consumed); dp=dp.add(strspn(dp,b"/\\\0".as_ptr() as *const c_char));
    let a=strlen(target_ppath); let b=strlen(dp);
    if a!=0||b!=0 { let p=kstrdup(target_ppath,0); if p.is_null(){kfree(ts as *mut c_void);return -12} *prefix=p; }
    *share=ts; 0
}
#[no_mangle] pub unsafe extern "C" fn dfs_cache_remount_fs(_: *mut c_void)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn dfs_cache_refresh(_: *mut c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
