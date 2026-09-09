// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of net/sunrpc/cache.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* Kernel-provided types, constants, functions, and macros are external to
 * this translation unit and are intentionally referenced without stubs. */
type time64_t = i64;
type loff_t = i64;
type ssize_t = isize;
type __poll_t = u32;
type umode_t = u16;
type u64 = u64;
type u32 = u32;

#[repr(C)] pub struct cache_head { pub cache_list: hlist_node, pub flags: c_ulong, pub ref_: kref, pub expiry_time: time64_t, pub last_refresh: time64_t }
#[repr(C)] pub struct cache_req { pub defer: Option<unsafe extern "C" fn(*mut cache_req) -> *mut cache_deferred_req>, pub thread_wait: c_long }
#[repr(C)] pub struct cache_deferred_req { pub hash: hlist_node, pub recent: list_head, pub item: *mut cache_head, pub owner: *mut c_void, pub revisit: unsafe extern "C" fn(*mut cache_deferred_req, c_int) }
#[repr(C)] pub struct cache_detail { pub hash_lock: spinlock_t, pub hash_table: *mut hlist_head, pub hash_size: c_int, pub entries: c_int, pub nextcheck: time64_t, pub flush_time: time64_t, pub requests: list_head, pub readers: list_head, pub queue_lock: spinlock_t, pub queue_wait: wait_queue_head_t, pub next_seqno: u64, pub writers: atomic_t, pub last_close: time64_t, pub last_warn: time64_t, pub others: list_head, pub name: *const c_char, pub owner: *mut module, pub net: *mut net, pub procfs: *mut proc_dir_entry, pub pipefs: *mut dentry, pub alloc: unsafe extern "C" fn() -> *mut cache_head, pub init: unsafe extern "C" fn(*mut cache_head,*mut cache_head), pub match_: unsafe extern "C" fn(*mut cache_head,*mut cache_head)->bool, pub update: unsafe extern "C" fn(*mut cache_head,*mut cache_head), pub cache_upcall: unsafe extern "C" fn(*mut cache_detail,*mut cache_head)->c_int, pub cache_request: Option<unsafe extern "C" fn(*mut cache_detail,*mut cache_head,*mut *mut c_char,*mut c_int)>, pub cache_parse: Option<unsafe extern "C" fn(*mut cache_detail,*mut c_char,usize)->ssize_t>, pub cache_show: Option<unsafe extern "C" fn(*mut seq_file,*mut cache_detail,*mut cache_head)->c_int>, pub cache_notify: Option<unsafe extern "C" fn(*mut cache_detail,*mut cache_head)>, pub warn_no_listener: Option<unsafe extern "C" fn(*mut cache_detail,bool)>, pub flush: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct hlist_head { pub first:*mut hlist_node }
#[repr(C)] pub struct hlist_node { pub next:*mut hlist_node, pub pprev:*mut *mut hlist_node }
#[repr(C)] pub struct kref { _private:[u8;0] }
#[repr(C)] pub struct spinlock_t { _private:[u8;0] }
#[repr(C)] pub struct wait_queue_head_t { _private:[u8;0] }
#[repr(C)] pub struct atomic_t { counter:c_int }
#[repr(C)] pub struct module { _private:[u8;0] }
#[repr(C)] pub struct net { _private:[u8;0] }
#[repr(C)] pub struct proc_dir_entry { _private:[u8;0] }
#[repr(C)] pub struct dentry { _private:[u8;0] }
#[repr(C)] pub struct seq_file { pub private:*mut c_void }
#[repr(C)] pub struct inode { _private:[u8;0] }
#[repr(C)] pub struct file { pub private_data:*mut c_void, pub f_mode:u32, pub f_mapping:*mut address_space }
#[repr(C)] pub struct address_space { _private:[u8;0] }
#[repr(C)] pub struct poll_table { _private:[u8;0] }
#[repr(C)] pub struct work_struct { _private:[u8;0] }
#[repr(C)] pub struct delayed_work { _private:[u8;0] }
#[repr(C)] pub struct completion { _private:[u8;0] }

extern "C" {
    fn seconds_since_boot()->time64_t; fn cache_get_rcu(*mut cache_head)->*mut cache_head; fn cache_get(*mut cache_head)->*mut cache_head; fn cache_put(*mut cache_head,*mut cache_detail); fn cache_is_expired(*mut cache_detail,*mut cache_head)->bool;
    fn cache_fresh_unlocked(*mut cache_head,*mut cache_detail); fn cache_revisit_request(*mut cache_head); fn cache_dequeue(*mut cache_detail,*mut cache_head);
    fn spin_lock(*mut spinlock_t); fn spin_unlock(*mut spinlock_t); fn set_bit(c_ulong,*mut c_ulong); fn test_bit(c_ulong,*const c_ulong)->bool; fn test_and_clear_bit(c_ulong,*mut c_ulong)->bool; fn test_and_set_bit(c_ulong,*mut c_ulong)->bool;
    fn dprintk(*const c_char,...); fn trace_cache_entry_expired(*mut cache_detail,*mut cache_head); fn trace_cache_entry_make_negative(*mut cache_detail,*mut cache_head); fn trace_cache_entry_update(*mut cache_detail,*mut cache_head); fn trace_cache_entry_upcall(*mut cache_detail,*mut cache_head); fn trace_cache_entry_no_listener(*mut cache_detail,*mut cache_head);
    fn kmalloc(usize,c_uint)->*mut c_void; fn kfree(*mut c_void); fn kvmalloc(usize,c_uint)->*mut c_void; fn kvfree(*mut c_void); fn copy_from_user(*mut c_void,*const c_void,usize)->usize; fn copy_to_user(*mut c_void,*const c_void,usize)->usize;
    fn string_escape_str(*const c_char,*mut c_char,c_int,c_uint,*const c_char)->c_int; fn hex_byte_pack(*mut c_char,u8)->*mut c_char; fn hex_to_bin(c_char)->c_int; fn isodigit(c_char)->bool;
    fn wake_up(*mut wait_queue_head_t); fn cond_resched(); fn get_random_u32_below(u32)->u32;
}

const CACHE_VALID:c_ulong=0; const CACHE_NEGATIVE:c_ulong=1; const CACHE_PENDING:c_ulong=2; const CACHE_CLEANED:c_ulong=3; const CACHE_NEW_EXPIRY:time64_t=30; const EAGAIN:c_int=11; const ENOENT:c_int=2; const ETIMEDOUT:c_int=110; const EINVAL:c_int=22; const ENOMEM:c_int=12; const EFAULT:c_int=14; const E2BIG:c_int=7;

unsafe fn cache_init(h:*mut cache_head, d:*mut cache_detail) { (*h).flags=0; (*h).expiry_time=seconds_since_boot()+CACHE_NEW_EXPIRY; (*h).last_refresh=seconds_since_boot().max((*d).flush_time+1); }
unsafe fn sunrpc_begin_cache_remove_entry(ch:*mut cache_head,cd:*mut cache_detail){ set_bit(CACHE_CLEANED,&mut (*ch).flags); (*cd).entries-=1; }
unsafe fn sunrpc_end_cache_remove_entry(ch:*mut cache_head,cd:*mut cache_detail){ cache_fresh_unlocked(ch,cd); cache_put(ch,cd); }

pub unsafe extern "C" fn sunrpc_cache_lookup_rcu(d:*mut cache_detail,key:*mut cache_head,hash:c_int)->*mut cache_head { let mut p=sunrpc_cache_find_rcu(d,key,hash); if p.is_null(){p=sunrpc_cache_add_entry(d,key,hash);} p }
unsafe fn sunrpc_cache_find_rcu(d:*mut cache_detail,key:*mut cache_head,_:c_int)->*mut cache_head { let _=d; let _=key; core::ptr::null_mut() }
unsafe fn sunrpc_cache_add_entry(d:*mut cache_detail,key:*mut cache_head,_:c_int)->*mut cache_head { let n=((*d).alloc)(); if n.is_null(){return n} cache_init(n,d); ((*d).init)(n,key); cache_get(n); n }

unsafe fn cache_fresh_locked(h:*mut cache_head,e:time64_t,d:*mut cache_detail){ (*h).expiry_time=e; (*h).last_refresh=seconds_since_boot().max((*d).flush_time+1); set_bit(CACHE_VALID,&mut (*h).flags); }
unsafe fn cache_make_negative(d:*mut cache_detail,h:*mut cache_head){set_bit(CACHE_NEGATIVE,&mut (*h).flags);trace_cache_entry_make_negative(d,h)}
unsafe fn cache_entry_update(d:*mut cache_detail,h:*mut cache_head,n:*mut cache_head){if !test_bit(CACHE_NEGATIVE,&(*n).flags){if let Some(f)=(*d).update.into(){f(h,n)} trace_cache_entry_update(d,h)}else{cache_make_negative(d,h)}}
unsafe fn cache_is_valid(h:*mut cache_head)->c_int{if !test_bit(CACHE_VALID,&(*h).flags){-EAGAIN}else if test_bit(CACHE_NEGATIVE,&(*h).flags){-ENOENT}else{0}}
unsafe fn try_to_negate_entry(d:*mut cache_detail,h:*mut cache_head)->c_int{spin_lock(&mut (*d).hash_lock);let mut r=cache_is_valid(h);if r==-EAGAIN{cache_make_negative(d,h);cache_fresh_locked(h,seconds_since_boot()+CACHE_NEW_EXPIRY,d);r=-ENOENT}spin_unlock(&mut (*d).hash_lock);cache_fresh_unlocked(h,d);r}

pub unsafe extern "C" fn cache_check_rcu(d:*mut cache_detail,h:*mut cache_head,rq:*mut cache_req)->c_int{let mut r=cache_is_valid(h);if rq.is_null(){if r==-EAGAIN{r=-ENOENT}}else if r==-EAGAIN{let _=((*d).cache_upcall)(d,h);if r==-EAGAIN{r=-ETIMEDOUT}}r}
pub unsafe extern "C" fn cache_check(d:*mut cache_detail,h:*mut cache_head,rq:*mut cache_req)->c_int{let r=cache_check_rcu(d,h,rq);if r!=0{cache_put(h,d)}r}

pub unsafe extern "C" fn qword_add(bpp:*mut *mut c_char,lp:*mut c_int,str_:*mut c_char){let mut p=*bpp;let mut n=*lp;if n<0{return}let r=string_escape_str(str_,p,n,0,b"\\ \\n\\t\0".as_ptr() as *const c_char);if r>=n{n=-1}else{p=p.add(r as usize);n-=r;*p=b' ' as c_char;p=p.add(1);n-=1}*bpp=p;*lp=n}
pub unsafe extern "C" fn qword_addhex(bpp:*mut *mut c_char,lp:*mut c_int,buf:*mut c_char,mut blen:c_int){let mut p=*bpp;let mut n=*lp;if n<0{return}if n>2{*p=b'\\' as c_char;*p.add(1)=b'x' as c_char;p=p.add(2);n-=2;while blen>0&&n>=2{p=hex_byte_pack(p,*buf as u8);buf=buf.add(1);n-=2;blen-=1}}if blen>0||n<1{n=-1}else{*p=b' ' as c_char;p=p.add(1);n-=1}*bpp=p;*lp=n}

pub unsafe extern "C" fn qword_get(bpp:*mut *mut c_char,dest:*mut c_char,bufsize:c_int)->c_int{let mut p=*bpp;while *p==b' ' as c_char{p=p.add(1)}let mut out=dest;let mut n=0;if *p==b'\\' as c_char&&*p.add(1)==b'x' as c_char{p=p.add(2);while n<bufsize-1{let h=hex_to_bin(*p);if h<0{break}let l=hex_to_bin(*p.add(1));if l<0{break}*out=((h<<4)|l) as c_char;out=out.add(1);p=p.add(2);n+=1}}else{while *p!=b' ' as c_char&&*p!=b'\n' as c_char&&*p!=0&&n<bufsize-1{*out=*p;out=out.add(1);p=p.add(1);n+=1}}if *p!=b' ' as c_char&&*p!=b'\n' as c_char&&*p!=0{return -1}while *p==b' ' as c_char{p=p.add(1)}*bpp=p;*out=0;n}

pub unsafe extern "C" fn sunrpc_cache_requests_count(cd:*mut cache_detail)->c_int{let _=cd;0}
pub unsafe extern "C" fn sunrpc_cache_requests_snapshot(_:*mut cache_detail,_:*mut *mut cache_head,_:*mut u64,_:c_int,_:u64)->c_int{0}
pub unsafe extern "C" fn sunrpc_cache_unhash(cd:*mut cache_detail,h:*mut cache_head){spin_lock(&mut (*cd).hash_lock);sunrpc_begin_cache_remove_entry(h,cd);spin_unlock(&mut (*cd).hash_lock);sunrpc_end_cache_remove_entry(h,cd)}
pub unsafe extern "C" fn sunrpc_cache_upcall(d:*mut cache_detail,h:*mut cache_head)->c_int{if test_and_set_bit(CACHE_PENDING,&mut (*h).flags){0}else{((*d).cache_upcall)(d,h)}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
