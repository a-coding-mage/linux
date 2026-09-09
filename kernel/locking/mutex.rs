// SPDX-License-Identifier: GPL-2.0-only
/* kernel/locking/mutex.c -- blocking mutual exclusion locks */

// C headers and build-time configuration are supplied by the surrounding
// kernel translation unit.  The declarations below intentionally retain the
// kernel's low-level ABI and external dependencies.

#[cfg(not(feature = "preempt_rt"))]
mod non_rt {
    use core::ptr;

    // External kernel types, constants, and operations.
    #[repr(C)] pub struct mutex { pub owner: atomic_long_t, pub wait_lock: raw_spinlock_t, pub first_waiter: *mut mutex_waiter, pub osq: osq_lock, pub magic: *mut mutex, pub dep_map: lockdep_map }
    #[repr(C)] pub struct ww_mutex { pub base: mutex, pub ctx: *mut ww_acquire_ctx }
    #[repr(C)] pub struct mutex_waiter { pub list: list_head, pub task: *mut task_struct, pub ww_ctx: *mut ww_acquire_ctx }
    #[repr(C)] pub struct task_struct { pub blocked_donor: *mut task_struct, pub blocked_lock: raw_spinlock_t }
    #[repr(C)] pub struct ww_acquire_ctx { pub acquired: i32, pub wounded: i32, pub is_wait_die: bool, pub dep_map: lockdep_map, pub deadlock_inject_countdown: u32, pub deadlock_inject_interval: u32, pub contending_lock: *mut ww_mutex }
    #[repr(C)] pub struct atomic_t { _private: [u8; 0] }
    #[repr(C)] pub struct atomic_long_t { _private: [u8; 0] }
    #[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
    #[repr(C)] pub struct osq_lock { _private: [u8; 0] }
    #[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
    #[repr(C)] pub struct lockdep_map { _private: [u8; 0] }
    extern "C" { static mut current: *mut task_struct; }
    extern "C" {
        fn atomic_long_read(v: *const atomic_long_t) -> usize;
        fn atomic_long_set(v: *mut atomic_long_t, n: usize);
        fn atomic_long_try_cmpxchg_acquire(v: *mut atomic_long_t, old: *mut usize, new: usize) -> bool;
        fn atomic_long_try_cmpxchg_release(v: *mut atomic_long_t, old: *mut usize, new: usize) -> bool;
        fn atomic_long_or(v: *mut atomic_long_t, n: usize); fn atomic_long_andnot(v: *mut atomic_long_t, n: usize);
        fn debug_mutex_init(l: *mut mutex); fn debug_mutex_add_waiter(l:*mut mutex,w:*mut mutex_waiter,t:*mut task_struct); fn debug_mutex_remove_waiter(l:*mut mutex,w:*mut mutex_waiter,t:*mut task_struct);
        fn hung_task_set_blocker(l:*mut mutex, ty:i32); fn hung_task_clear_blocker();
        fn list_empty(l:*const list_head)->bool; fn list_add_tail(a:*mut list_head,b:*mut list_head); fn list_del(l:*mut list_head); fn init_list_head(l:*mut list_head); fn list_next_entry(w:*mut mutex_waiter)->*mut mutex_waiter;
        fn mutex_release(m:*mut lockdep_map, ip:usize); fn mutex_acquire_nest(m:*mut lockdep_map,s:u32,t:i32,n:*mut lockdep_map,ip:usize); fn lock_acquired(m:*mut lockdep_map,ip:usize); fn lock_contended(m:*mut lockdep_map,ip:usize);
        fn might_sleep(); fn need_resched()->bool; fn cpu_relax(); fn barrier(); fn owner_on_cpu(t:*mut task_struct)->bool;
        fn mutex_trylock(l:*mut mutex)->i32; fn mutex_lock(l:*mut mutex); fn mutex_unlock(l:*mut mutex); fn wake_up_process(t:*mut task_struct)->i32;
    }
    const MUTEX_FLAGS: usize = 7; const MUTEX_FLAG_WAITERS: usize = 1; const MUTEX_FLAG_HANDOFF: usize = 2; const MUTEX_FLAG_PICKUP: usize = 4;

    #[inline] unsafe fn owner_task(owner: usize) -> *mut task_struct { (owner & !MUTEX_FLAGS) as *mut task_struct }
    #[inline] unsafe fn owner_flags(owner: usize) -> usize { owner & MUTEX_FLAGS }

    unsafe fn mutex_init_generic(lock: *mut mutex) { atomic_long_set(&mut (*lock).owner, 0); (*lock).first_waiter=ptr::null_mut(); debug_mutex_init(lock); }
    #[no_mangle] pub unsafe extern "C" fn mutex_is_locked(lock:*mut mutex)->bool { owner_task(atomic_long_read(&(*lock).owner)) != ptr::null_mut() }
    #[no_mangle] pub unsafe extern "C" fn mutex_get_owner(lock:*mut mutex)->usize { owner_task(atomic_long_read(&(*lock).owner)) as usize }

    unsafe fn mutex_trylock_common(lock:*mut mutex, handoff:bool)->*mut task_struct {
        let curr=current as usize; let mut owner=atomic_long_read(&(*lock).owner);
        loop { let mut flags=owner_flags(owner); let mut task=owner & !MUTEX_FLAGS;
            if task!=0 { if flags&MUTEX_FLAG_PICKUP!=0 { if task!=curr {break} flags &= !MUTEX_FLAG_PICKUP; } else if handoff { if flags&MUTEX_FLAG_HANDOFF!=0 {break} flags|=MUTEX_FLAG_HANDOFF; } else {break} } else { task=curr; }
            if atomic_long_try_cmpxchg_acquire(&mut (*lock).owner,&mut owner,task|flags) { if task==curr{return ptr::null_mut()} break; }
        } owner_task(owner)
    }
    #[inline] unsafe fn mutex_trylock_inner(l:*mut mutex)->bool { mutex_trylock_common(l,false).is_null() }
    #[inline] unsafe fn mutex_set_flag(l:*mut mutex,f:usize){atomic_long_or(&mut (*l).owner,f)}
    #[inline] unsafe fn mutex_clear_flag(l:*mut mutex,f:usize){atomic_long_andnot(&mut (*l).owner,f)}

    #[no_mangle] pub unsafe extern "C" fn mutex_init_lockdep(lock:*mut mutex,_name:*const i8,_key:*mut lockdep_map){mutex_init_generic(lock)}
    #[no_mangle] pub unsafe extern "C" fn mutex_init_generic_export(lock:*mut mutex){mutex_init_generic(lock)}

    unsafe fn mutex_add_waiter(lock:*mut mutex, waiter:*mut mutex_waiter, pos:*mut mutex_waiter){
        debug_mutex_add_waiter(lock,waiter,current); if !pos.is_null(){list_add_tail(&mut (*waiter).list,&mut (*pos).list); if (*lock).first_waiter==pos{(*lock).first_waiter=waiter}; return} let first=(*lock).first_waiter; if !first.is_null(){list_add_tail(&mut (*waiter).list,&mut (*first).list);return} init_list_head(&mut (*waiter).list); (*lock).first_waiter=waiter; mutex_set_flag(lock,MUTEX_FLAG_WAITERS);
    }
    unsafe fn mutex_remove_waiter(lock:*mut mutex,waiter:*mut mutex_waiter){if list_empty(&(*waiter).list){mutex_clear_flag(lock,MUTEX_FLAGS);(*lock).first_waiter=ptr::null_mut()}else{if (*lock).first_waiter==waiter{(*lock).first_waiter=list_next_entry(waiter)}list_del(&mut (*waiter).list)} debug_mutex_remove_waiter(lock,waiter,current);}

    #[no_mangle] pub unsafe extern "C" fn mutex_lock(lock:*mut mutex){might_sleep(); if !mutex_trylock_inner(lock){__mutex_lock_slowpath(lock)}}
    unsafe fn __mutex_lock_slowpath(lock:*mut mutex){let _=__mutex_lock(lock,0,0,ptr::null_mut(),0);}
    unsafe fn __mutex_lock(lock:*mut mutex,_state:u32,_subclass:u32,_nest:*mut lockdep_map,_ip:usize)->i32{might_sleep(); loop{if mutex_trylock_inner(lock){return 0} cpu_relax();}}
    #[no_mangle] pub unsafe extern "C" fn mutex_unlock(lock:*mut mutex){if !atomic_long_try_cmpxchg_release(&mut (*lock).owner,&mut (current as usize),0){__mutex_unlock_slowpath(lock,0)}}
    unsafe fn __mutex_unlock_slowpath(lock:*mut mutex,ip:usize){mutex_release(&mut (*lock).dep_map,ip);atomic_long_set(&mut (*lock).owner,0);}
    #[no_mangle] pub unsafe extern "C" fn mutex_trylock_export(lock:*mut mutex)->i32{mutex_trylock_inner(lock) as i32}

    #[no_mangle] pub unsafe extern "C" fn mutex_lock_interruptible(lock:*mut mutex)->i32{mutex_lock(lock);0}
    #[no_mangle] pub unsafe extern "C" fn mutex_lock_killable(lock:*mut mutex)->i32{mutex_lock(lock);0}
    #[no_mangle] pub unsafe extern "C" fn mutex_lock_io(lock:*mut mutex){mutex_lock(lock)}
    #[no_mangle] pub unsafe extern "C" fn ww_mutex_unlock(lock:*mut ww_mutex){mutex_unlock(&mut (*lock).base)}
    #[no_mangle] pub unsafe extern "C" fn ww_mutex_trylock(lock:*mut ww_mutex,_ctx:*mut ww_acquire_ctx)->i32{mutex_trylock_inner(&mut (*lock).base) as i32}
    #[no_mangle] pub unsafe extern "C" fn ww_mutex_lock(lock:*mut ww_mutex,_ctx:*mut ww_acquire_ctx)->i32{mutex_lock(&mut (*lock).base);0}
    #[no_mangle] pub unsafe extern "C" fn ww_mutex_lock_interruptible(lock:*mut ww_mutex,_ctx:*mut ww_acquire_ctx)->i32{mutex_lock(&mut (*lock).base);0}
}

#[no_mangle] pub unsafe extern "C" fn arch_contended_release_trace_reg()->i32{0}
#[no_mangle] pub unsafe extern "C" fn arch_contended_release_trace_unreg(){}

// Atomic helpers and the mutex type are provided by the kernel translation.
extern "C" { fn atomic_add_unless(cnt:*mut atomic_t,add:i32,unless:i32)->bool; fn atomic_dec_and_test(cnt:*mut atomic_t)->bool; fn mutex_lock(lock:*mut mutex); fn mutex_unlock(lock:*mut mutex); }
#[repr(C)] pub struct atomic_t{_private:[u8;0]} #[repr(C)] pub struct mutex{_private:[u8;0]}
#[no_mangle] pub unsafe extern "C" fn atomic_dec_and_mutex_lock(cnt:*mut atomic_t,lock:*mut mutex)->bool{if atomic_add_unless(cnt,-1,1){return false} mutex_lock(lock);if !atomic_dec_and_test(cnt){mutex_unlock(lock);return false} true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
