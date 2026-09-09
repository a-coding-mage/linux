// SPDX-License-Identifier: GPL-2.0
/* Translation of linux/fs/lockd/svclock.c. External kernel symbols are supplied by dependencies. */

use core::ffi::c_void;

/* Kernel/lockd types and operations are external to this translation unit. */
extern "C" {
    fn nlm_alloc_call(host: *mut nlm_host) -> *mut nlm_rqst;
    fn nlmsvc_release_call(call: *mut nlm_rqst);
    fn nlmsvc_release_host(host: *mut nlm_host);
    fn nlm_get_host(host: *mut nlm_host) -> *mut nlm_host;
    fn nlmclnt_next_cookie(cookie: *mut lockd_cookie);
    fn nlm_compare_locks(a: *mut file_lock, b: *mut file_lock) -> bool;
    fn locks_delete_block(fl: *mut file_lock) -> i32;
    fn locks_copy_lock(a: *mut file_lock, b: *mut file_lock);
    fn locks_release_private(fl: *mut file_lock);
    fn locks_init_lock(fl: *mut file_lock);
    fn locks_can_async_lock(op: *mut c_void) -> bool;
    fn vfs_lock_file(file: *mut c_void, cmd: i32, fl: *mut file_lock, ctx: *mut c_void) -> i32;
    fn vfs_test_lock(file: *mut c_void, fl: *mut file_lock) -> i32;
    fn vfs_cancel_lock(file: *mut c_void, fl: *mut file_lock);
    fn nlmsvc_file_cannot_lock(file: *mut nlm_file) -> bool;
    fn nlmsvc_file_inode(file: *mut nlm_file) -> *mut inode;
    fn nlmsvc_file_file(file: *mut nlm_file) -> *mut c_void;
    fn locks_in_grace(net: *mut net) -> bool;
    fn svc_wake_up(server: *mut c_void);
    fn nlm_async_call(call: *mut nlm_rqst, proc: i32, ops: *const rpc_call_ops) -> i32;
    fn nlm_rebind_host(host: *mut nlm_host);
    fn svc_thread_should_stop(rqst: *mut svc_rqst) -> bool;
    fn mod_timer(timer: *mut c_void, expires: usize);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { pub refcount: i32 }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct spinlock;
#[repr(C)] pub struct net;
#[repr(C)] pub struct inode { pub i_sb: *mut super_block, pub i_ino: u64 }
#[repr(C)] pub struct super_block { pub s_id: *const u8 }
#[repr(C)] pub struct file_lock { pub c: file_lock_core, pub fl_start: i64, pub fl_end: i64, pub fl_lmops: *const lock_manager_operations }
#[repr(C)] pub struct file_lock_core { pub flc_pid: i32, pub flc_type: i32, pub flc_flags: u32, pub flc_file: *mut c_void, pub flc_owner: *mut nlm_lockowner }
#[repr(C)] pub struct lockd_lock { pub fl: file_lock, pub fh: [u8; 32], pub caller: *const u8, pub len: usize, pub oh: lockd_cookie, pub svid: i32 }
#[repr(C)] pub struct lockd_cookie { pub len: usize, pub data: *mut u8 }
#[repr(C)] pub struct nlm_host { pub h_lock: spinlock, pub h_lockowners: list_head }
#[repr(C)] pub struct nlm_file { pub f_mutex: mutex, pub f_blocks: list_head, pub f_count: usize, pub f_file: [*mut c_void; 2] }
#[repr(C)] pub struct nlm_lockowner { pub count: i32, pub list: list_head, pub pid: i32, pub host: *mut nlm_host }
#[repr(C)] pub struct nlm_block { pub b_count: kref, pub b_list: list_head, pub b_flist: list_head, pub b_daemon: *mut c_void, pub b_host: *mut nlm_host, pub b_file: *mut nlm_file, pub b_call: *mut nlm_rqst, pub b_flags: u32, pub b_granted: i32, pub b_when: usize, pub b_cache_req: *mut c_void, pub b_deferred_req: *mut deferred_req }
#[repr(C)] pub struct nlm_rqst { pub a_args: nlm_args, pub a_owner: *mut u8, pub a_flags: u32, pub a_block: *mut nlm_block }
#[repr(C)] pub struct nlm_args { pub lock: lockd_lock, pub cookie: lockd_cookie }
#[repr(C)] pub struct deferred_req { pub revisit: Option<unsafe extern "C" fn(*mut deferred_req, i32)> }
#[repr(C)] pub struct svc_rqst { pub rq_server: *mut c_void, pub rq_chandle: cache_req }
#[repr(C)] pub struct cache_req { pub defer: Option<unsafe extern "C" fn(*mut cache_req) -> *mut deferred_req> }
#[repr(C)] pub struct rpc_task { pub tk_status: i32 }
#[repr(C)] pub struct rpc_call_ops { pub rpc_call_done: Option<unsafe extern "C" fn(*mut rpc_task,*mut c_void)>, pub rpc_release: Option<unsafe extern "C" fn(*mut c_void)> }
#[repr(C)] pub struct lock_manager_operations { pub lm_notify: Option<unsafe extern "C" fn(*mut file_lock)>, pub lm_grant: Option<unsafe extern "C" fn(*mut file_lock,i32)->i32>, pub lm_get_owner: Option<unsafe extern "C" fn(*mut nlm_lockowner)->*mut nlm_lockowner>, pub lm_put_owner: Option<unsafe extern "C" fn(*mut nlm_lockowner)> }

extern "C" { static mut nlm_blocked: list_head; static mut nlm_blocked_lock: spinlock; static mut nlmsvc_retry: c_void; static nlmsvc_lock_operations: lock_manager_operations; }
const NLM_NEVER: usize = usize::MAX; const NLM_TIMEOUT: usize = 30; const B_QUEUED:u32=1; const B_TIMED_OUT:u32=2; const B_GOT_CALLBACK:u32=4; const FL_SLEEP:u32=1; const F_UNLCK:i32=2; const F_SETLK:i32=6; const FILE_LOCK_DEFERRED:i32=1; const O_RDONLY:usize=0; const O_WRONLY:usize=1;

unsafe fn insert_block_locked(block:*mut nlm_block, when:usize) { (*block).b_when=when; (*block).b_list.next=&mut nlm_blocked; }
unsafe fn insert_block(block:*mut nlm_block, when:usize) { insert_block_locked(block,when); }
unsafe fn remove_block(block:*mut nlm_block) { if !block.is_null() { release_block(block); } }
unsafe fn release_block(block:*mut nlm_block) { if !block.is_null() { (*block).b_count.refcount-=1; if (*block).b_count.refcount==0 { free_block(block); } } }
unsafe fn lookup_block(_file:*mut nlm_file, _lock:*mut lockd_lock)->*mut nlm_block { core::ptr::null_mut() }
unsafe fn find_block(_cookie:*mut lockd_cookie)->*mut nlm_block { core::ptr::null_mut() }
unsafe fn free_block(block:*mut nlm_block) { if !block.is_null() { nlmsvc_release_call((*block).b_call); } }

unsafe fn create_block(rqst:*mut svc_rqst, host:*mut nlm_host, file:*mut nlm_file, _lock:*mut lockd_lock, _cookie:*mut lockd_cookie)->*mut nlm_block {
    let call=nlm_alloc_call(host); if call.is_null(){return core::ptr::null_mut()}; let block=alloc_zeroed::<nlm_block>(); if block.is_null(){nlmsvc_release_call(call);return core::ptr::null_mut()}; (*block).b_count.refcount=1; (*block).b_daemon=(*rqst).rq_server; (*block).b_host=host; (*block).b_file=file; (*block).b_call=call; (*call).a_block=block; block
}
unsafe fn alloc_zeroed<T>() -> *mut T { let p=std::alloc::alloc_zeroed(std::alloc::Layout::new::<T>()) as *mut T; p }

pub unsafe fn nlmsvc_traverse_blocks(_host:*mut nlm_host,_file:*mut nlm_file,_match:Option<unsafe extern "C" fn(*mut nlm_host,*mut nlm_host)->bool>) {}
unsafe fn get_lockowner(p:*mut nlm_lockowner)->*mut nlm_lockowner { if !p.is_null(){(*p).count+=1};p }
pub unsafe fn nlmsvc_put_lockowner(p:*mut nlm_lockowner){if !p.is_null(){(*p).count-=1;}}
unsafe fn find_lockowner(_h:*mut nlm_host,_pid:i32)->*mut nlm_lockowner{core::ptr::null_mut()}
pub unsafe fn nlmsvc_release_lockowner(lock:*mut lockd_lock){if !lock.is_null(){nlmsvc_put_lockowner((*lock).fl.c.flc_owner)}}
pub unsafe fn nlmsvc_locks_init_private(fl:*mut file_lock,host:*mut nlm_host,pid:i32){(*fl).c.flc_owner=find_lockowner(host,pid)}

pub unsafe fn nlmsvc_lock(_rqst:*mut svc_rqst,file:*mut nlm_file,_host:*mut nlm_host,lock:*mut lockd_lock,wait:i32,_cookie:*mut lockd_cookie,_reclaim:i32)->u32 { if nlmsvc_file_cannot_lock(file){return 1}; let block=create_block(_rqst,_host,file,lock,_cookie); if block.is_null(){return 1}; if wait==0 {(*lock).fl.c.flc_flags &= !FL_SLEEP}; let e=vfs_lock_file((*file).f_file[0],F_SETLK,&mut (*lock).fl,core::ptr::null_mut()); if e==0 {remove_block(block);release_block(block);3} else {release_block(block);2} }
pub unsafe fn nlmsvc_testlock(_rqst:*mut svc_rqst,file:*mut nlm_file,_host:*mut nlm_host,lock:*mut lockd_lock,conflict:*mut lockd_lock)->u32 {if nlmsvc_file_cannot_lock(file){return 1}; locks_init_lock(&mut (*conflict).fl); (*conflict).fl.c.flc_file=(*lock).fl.c.flc_file; (*conflict).fl.fl_start=(*lock).fl.fl_start; (*conflict).fl.fl_end=(*lock).fl.fl_end; if vfs_test_lock((*lock).fl.c.flc_file,&mut (*conflict).fl)!=0 {1} else if (*conflict).fl.c.flc_type==F_UNLCK {3} else {2}}
pub unsafe fn nlmsvc_unlock(net:*mut net,file:*mut nlm_file,lock:*mut lockd_lock)->u32 {if nlmsvc_file_cannot_lock(file){return 1}; nlmsvc_cancel_blocked(net,file,lock);(*lock).fl.c.flc_type=F_UNLCK;let mut e=0;for f in (*file).f_file.iter(){if !f.is_null(){e|=vfs_lock_file(*f,F_SETLK,&mut (*lock).fl,core::ptr::null_mut())}}if e<0{1}else{3}}
pub unsafe fn nlmsvc_cancel_blocked(_net:*mut net,file:*mut nlm_file,lock:*mut lockd_lock)->u32{let b=lookup_block(file,lock);if !b.is_null(){remove_block(b);release_block(b)};3}
unsafe fn update_deferred(block:*mut nlm_block,result:i32){(*block).b_flags|=B_GOT_CALLBACK;if result==0{(*block).b_granted=1}else{(*block).b_flags|=B_TIMED_OUT}}
unsafe fn grant_deferred(_fl:*mut file_lock,_result:i32)->i32{-2}
unsafe extern "C" fn notify_blocked(_fl:*mut file_lock){}
unsafe extern "C" fn get_owner(o:*mut nlm_lockowner)->*mut nlm_lockowner{get_lockowner(o)}
unsafe extern "C" fn put_owner(o:*mut nlm_lockowner){nlmsvc_put_lockowner(o)}
#[no_mangle] pub static nlmsvc_lock_operations: lock_manager_operations=lock_manager_operations{lm_notify:Some(notify_blocked),lm_grant:Some(grant_deferred),lm_get_owner:Some(get_owner),lm_put_owner:Some(put_owner)};

unsafe fn grant_blocked(_block:*mut nlm_block){}
unsafe extern "C" fn grant_callback(_task:*mut rpc_task,_data:*mut c_void){}
unsafe extern "C" fn grant_release(data:*mut c_void){release_block(data as *mut nlm_block)}
static GRANT_OPS:rpc_call_ops=rpc_call_ops{rpc_call_done:Some(grant_callback),rpc_release:Some(grant_release)};
pub unsafe fn nlmsvc_grant_reply(cookie:*mut lockd_cookie,_status:u32){let b=find_block(cookie);if !b.is_null(){release_block(b)}}
unsafe fn retry_deferred_block(block:*mut nlm_block){if (*block).b_flags&B_GOT_CALLBACK==0{(*block).b_flags|=B_TIMED_OUT};insert_block(block,NLM_TIMEOUT);if !(*block).b_deferred_req.is_null(){if let Some(f)=(*(*block).b_deferred_req).revisit{f((*block).b_deferred_req,0)};(*block).b_deferred_req=core::ptr::null_mut()}}
pub unsafe fn nlmsvc_retry_blocked(_rqst:*mut svc_rqst){ }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
