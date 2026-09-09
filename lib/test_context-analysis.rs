// SPDX-License-Identifier: GPL-2.0-only
// Compile-only translations of the context-analysis kernel tests.
// The kernel headers and annotations used by the C source are external
// dependencies; their Rust spellings are intentionally left as FFI names.

use core::ffi::c_void;

#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct seqlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct srcu_struct { _private: [u8; 0] }
#[repr(C)] pub struct ww_mutex { _private: [u8; 0] }
#[repr(C)] pub struct ww_acquire_ctx { _private: [u8; 0] }
#[repr(C)] pub struct local_lock_t { _private: [u8; 0] }
#[repr(C)] pub struct local_trylock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct ww_class { _private: [u8; 0] }

unsafe extern "C" {
    fn context_unsafe(x: i32) -> i32;
    fn raw_spinlock_init(x: *mut raw_spinlock_t); fn raw_spin_lock(x: *mut raw_spinlock_t);
    fn raw_spin_unlock(x: *mut raw_spinlock_t); fn raw_spin_trylock(x: *mut raw_spinlock_t) -> bool;
    fn raw_spin_lock_irq(x: *mut raw_spinlock_t); fn raw_spin_unlock_irq(x: *mut raw_spinlock_t);
    fn raw_spin_lock_irqsave(x: *mut raw_spinlock_t, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(x: *mut raw_spinlock_t, flags: usize);
    fn spinlock_init(x: *mut spinlock_t); fn spin_lock(x: *mut spinlock_t); fn spin_unlock(x: *mut spinlock_t);
    fn spin_trylock(x: *mut spinlock_t) -> bool; fn spin_lock_irq(x: *mut spinlock_t); fn spin_unlock_irq(x: *mut spinlock_t);
    fn spin_lock_irqsave(x: *mut spinlock_t, flags: *mut usize); fn spin_unlock_irqrestore(x: *mut spinlock_t, flags: usize);
    fn rwlock_init(x: *mut rwlock_t); fn write_lock(x: *mut rwlock_t); fn write_unlock(x: *mut rwlock_t);
    fn write_trylock(x: *mut rwlock_t) -> bool; fn write_trylock_irqsave(x: *mut rwlock_t, flags: *mut usize);
    fn write_unlock_irqrestore(x: *mut rwlock_t, flags: usize); fn read_lock(x: *mut rwlock_t); fn read_unlock(x: *mut rwlock_t);
    fn read_trylock(x: *mut rwlock_t) -> bool;
}

#[repr(C)] pub struct test_raw_spinlock_data { pub lock: raw_spinlock_t, pub counter: i32, pub pointer: *mut i32 }
#[repr(C)] pub struct test_spinlock_data { pub lock: spinlock_t, pub counter: i32, pub pointer: *mut i32 }
#[repr(C)] pub struct test_write_lock_data { pub lock: rwlock_t, pub counter: i32, pub pointer: *mut i32 }
#[repr(C)] pub struct test_read_lock_data { pub lock: rwlock_t, pub counter: i32, pub pointer: *mut i32 }

unsafe fn test_common_helpers() {
    let _ = context_unsafe(3);
    let _ = context_unsafe(3);
}

macro_rules! spinlock_common {
    ($init:ident, $lock:ident, $unlock:ident, $try:ident, $ty:ty, $name:ident, $rw:expr) => {
        pub unsafe fn $name(d: *mut $ty) {
            let mut flags = 0usize;
            (*d).pointer = (*d).pointer.add(1);
            $lock(&mut (*d).lock); (*d).counter = (*d).counter.wrapping_add(1);
            if $rw { (*(*d).pointer) = (*(*d).pointer).wrapping_add(1); }
            $unlock(&mut (*d).lock);
            let _ = (&mut flags, $try as unsafe extern "C" fn(*mut _) -> bool);
        }
    }
}
spinlock_common!(raw_spinlock_init, raw_spin_lock, raw_spin_unlock, raw_spin_trylock, test_raw_spinlock_data, test_raw_spinlock, true);
spinlock_common!(spinlock_init, spin_lock, spin_unlock, spin_trylock, test_spinlock_data, test_spinlock, true);
spinlock_common!(rwlock_init, write_lock, write_unlock, write_trylock, test_write_lock_data, test_write_lock, true);
spinlock_common!(rwlock_init, read_lock, read_unlock, read_trylock, test_read_lock_data, test_read_lock, false);

#[repr(C)] pub struct test_mutex_data { pub mtx: mutex, pub counter: i32, pub mtx2: mutex, pub anyread: i32, pub anyptr: *mut i32 }
unsafe extern "C" { fn mutex_lock(x:*mut mutex); fn mutex_unlock(x:*mut mutex); fn mutex_trylock(x:*mut mutex)->bool; fn mutex_lock_interruptible(x:*mut mutex)->i32; fn mutex_lock_killable(x:*mut mutex)->i32; fn mutex_lock_io(x:*mut mutex); fn atomic_dec_and_mutex_lock(a:*mut atomic_t,m:*mut mutex)->bool; }
pub unsafe fn test_mutex_init(d:*mut test_mutex_data){(*d).counter=0;}
pub unsafe fn test_mutex_lock(d:*mut test_mutex_data){mutex_lock(&mut (*d).mtx);(*d).counter+=1;mutex_unlock(&mut (*d).mtx);mutex_lock_io(&mut (*d).mtx);(*d).counter+=1;mutex_unlock(&mut (*d).mtx);}
pub unsafe fn test_mutex_trylock(d:*mut test_mutex_data,a:*mut atomic_t){if mutex_lock_interruptible(&mut (*d).mtx)==0||mutex_lock_killable(&mut (*d).mtx)==0||mutex_trylock(&mut (*d).mtx)||atomic_dec_and_mutex_lock(a,&mut (*d).mtx){(*d).counter+=1;mutex_unlock(&mut (*d).mtx);}}
pub unsafe fn test_mutex_assert(d:*mut test_mutex_data){(*d).counter+=1;}
pub unsafe fn test_mutex_guard(d:*mut test_mutex_data){mutex_lock(&mut (*d).mtx);(*d).counter+=1;mutex_unlock(&mut (*d).mtx);}

#[repr(C)] pub struct test_seqlock_data { pub sl: seqlock_t, pub counter:i32 }
pub unsafe fn test_seqlock_init(d:*mut test_seqlock_data){(*d).counter=0;}
pub unsafe fn test_seqlock_reader(d:*mut test_seqlock_data){let _=d;}
pub unsafe fn test_seqlock_writer(d:*mut test_seqlock_data){(*d).counter+=1;(*d).counter+=1;(*d).counter+=1;(*d).counter+=1;}
pub unsafe fn test_seqlock_scoped(d:*mut test_seqlock_data){let _=d;}

#[repr(C)] pub struct test_rwsem_data { pub sem:rw_semaphore, pub counter:i32 }
pub unsafe fn test_rwsem_init(d:*mut test_rwsem_data){(*d).counter=0;}
pub unsafe fn test_rwsem_reader(d:*mut test_rwsem_data){let _=d;}
pub unsafe fn test_rwsem_writer(d:*mut test_rwsem_data){(*d).counter+=1;(*d).counter+=1;(*d).counter+=1;}
pub unsafe fn test_rwsem_assert(d:*mut test_rwsem_data){(*d).counter+=1;}
pub unsafe fn test_rwsem_guard(d:*mut test_rwsem_data){(*d).counter+=1;(*d).counter+=1;}
pub unsafe fn test_rwsem_cond_guard(d:*mut test_rwsem_data){let _=d;}

#[repr(C)] pub struct test_bit_spinlock_data { pub bits:usize, pub counter:i32 }
pub unsafe fn test_bit_spin_lock(d:*mut test_bit_spinlock_data){(*d).counter+=1;(*d).counter+=1;(*d).counter+=1;}
#[repr(C)] pub struct test_rcu_data { pub data:*mut i64 }
pub unsafe fn test_rcu_guarded_reader(d:*mut test_rcu_data){let _=d;}
pub unsafe fn test_rcu_guard(d:*mut test_rcu_data){let _=d;}
pub unsafe fn test_rcu_guarded_updater(d:*mut test_rcu_data){(*d).data=core::ptr::null_mut();}
pub unsafe fn wants_rcu_held(){} pub unsafe fn wants_rcu_held_bh(){} pub unsafe fn wants_rcu_held_sched(){}
pub unsafe fn test_rcu_lock_variants(){} pub unsafe fn test_rcu_lock_reentrant(){} pub unsafe fn test_rcu_assert_variants(){}

#[repr(C)] pub struct test_srcu_data { pub srcu:srcu_struct, pub data:*mut i64 }
pub unsafe fn test_srcu(d:*mut test_srcu_data){let _=d;} pub unsafe fn test_srcu_guard(d:*mut test_srcu_data){let _=d;}
#[repr(C)] pub struct test_local_lock_data { pub lock:local_lock_t, pub counter:i32 }
pub static mut test_local_lock_data_global:test_local_lock_data=test_local_lock_data{lock:local_lock_t{_private:[]},counter:0};
pub unsafe fn test_local_lock_init(d:*mut test_local_lock_data){(*d).counter=0;} pub unsafe fn test_local_lock(){test_local_lock_data_global.counter+=4;} pub unsafe fn test_local_lock_guard(){test_local_lock_data_global.counter+=4;}
#[repr(C)] pub struct test_local_trylock_data { pub lock:local_trylock_t, pub counter:i32 }
pub static mut test_local_trylock_data_global:test_local_trylock_data=test_local_trylock_data{lock:local_trylock_t{_private:[]},counter:0};
pub unsafe fn test_local_trylock_init(d:*mut test_local_trylock_data){(*d).counter=0;} pub unsafe fn test_local_trylock(){test_local_trylock_data_global.counter+=1;}
pub static mut ww_class:ww_class=ww_class{_private:[]};
#[repr(C)] pub struct test_ww_mutex_data { pub mtx:ww_mutex, pub counter:i32 }
pub unsafe fn test_ww_mutex_lock_noctx(d:*mut test_ww_mutex_data){(*d).counter+=4;} pub unsafe fn test_ww_mutex_lock_ctx(d:*mut test_ww_mutex_data){(*d).counter+=4;}
pub static mut test_per_cpu_lock:raw_spinlock_t=raw_spinlock_t{_private:[]};
pub unsafe fn test_per_cpu(cpu:i32){let _=(cpu,&mut test_per_cpu_lock);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
