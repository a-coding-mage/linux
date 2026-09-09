/* SPDX-License-Identifier: GPL-2.0 */
//! Source-level Rust translation of Linux `seqlock.h`.
//!
//! Kernel-provided types and operations referenced here are intentionally left
//! as external dependencies.

pub const KCSAN_SEQLOCK_REGION_MAX: usize = 1000;

#[repr(C)]
pub struct seqcount_t { pub sequence: core::ffi::c_uint, pub dep_map: lockdep_map }
#[repr(C)] pub struct seqcount_latch_t { pub seqcount: seqcount_t }
#[repr(C)] pub struct seqlock_t { pub seqcount: seqcount_spinlock_t, pub lock: spinlock_t }
#[repr(C)] pub struct seqcount_raw_spinlock_t { pub seqcount: seqcount_t, pub lock: *mut raw_spinlock_t }
#[repr(C)] pub struct seqcount_spinlock_t { pub seqcount: seqcount_t, pub lock: *mut spinlock_t }
#[repr(C)] pub struct seqcount_rwlock_t { pub seqcount: seqcount_t, pub lock: *mut rwlock_t }
#[repr(C)] pub struct seqcount_mutex_t { pub seqcount: seqcount_t, pub lock: *mut mutex }
#[repr(C)] pub struct lockdep_map { pub name: *const core::ffi::c_char }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }

extern "C" {
    fn lockdep_init_map(m: *mut lockdep_map, name: *const core::ffi::c_char, key: *mut lock_class_key, subclass: u32);
    fn smp_load_acquire(p: *const core::ffi::c_uint) -> core::ffi::c_uint;
    fn read_once(p: *const core::ffi::c_uint) -> core::ffi::c_uint;
    fn smp_rmb(); fn smp_wmb(); fn cpu_relax();
    fn kcsan_atomic_next(n: usize); fn kcsan_nestable_atomic_begin(); fn kcsan_nestable_atomic_end();
    fn preempt_disable(); fn preempt_enable();
    fn spin_lock(l: *mut spinlock_t); fn spin_unlock(l: *mut spinlock_t);
    fn spin_lock_bh(l: *mut spinlock_t); fn spin_unlock_bh(l: *mut spinlock_t);
    fn spin_lock_irq(l: *mut spinlock_t); fn spin_unlock_irq(l: *mut spinlock_t);
    fn spin_lock_irqsave(l: *mut spinlock_t, flags: *mut usize); fn spin_unlock_irqrestore(l: *mut spinlock_t, flags: usize);
    fn lockdep_assert_preemption_disabled(); fn lockdep_assert_held(l: *const core::ffi::c_void);
    fn seqcount_acquire(m: *mut lockdep_map, subclass: i32, trylock: i32, ip: usize);
    fn seqcount_release(m: *mut lockdep_map, ip: usize);
    fn seqcount_acquire_read(m: *mut lockdep_map, subclass: i32, trylock: i32, ip: usize);
    fn seqcount_release_read(m: *mut lockdep_map, ip: usize);
    fn __scoped_seqlock_invalid_target();
    fn __scoped_seqlock_bug();
}

#[inline] pub unsafe fn __seqcount_init(s: *mut seqcount_t, name: *const core::ffi::c_char, key: *mut lock_class_key) { lockdep_init_map(&mut (*s).dep_map, name, key, 0); (*s).sequence = 0; }
#[inline] pub unsafe fn seqcount_init(s: *mut seqcount_t) { __seqcount_init(s, core::ptr::null(), core::ptr::null_mut()) }
#[inline] pub unsafe fn seqcount_lockdep_reader_access(s: *const seqcount_t) { let l=s as *mut seqcount_t; seqcount_acquire_read(&mut (*l).dep_map,0,0,0); seqcount_release_read(&mut (*l).dep_map,0); }

#[inline] pub unsafe fn __seqprop_ptr(s:*mut seqcount_t)->*mut seqcount_t{s}
#[inline] pub unsafe fn __seqprop_const_ptr(s:*const seqcount_t)->*const seqcount_t{s}
#[inline] pub unsafe fn __seqprop_sequence(s:*const seqcount_t)->u32{smp_load_acquire(&(*s).sequence)}
#[inline] pub unsafe fn __seqprop_preemptible(_: *const seqcount_t)->bool{false}
#[inline] pub unsafe fn __seqprop_assert(_: *const seqcount_t){lockdep_assert_preemption_disabled()}

macro_rules! seqcount_lockname { ($n:ident,$t:ty,$p:expr,$base:ident) => {
    #[inline] pub unsafe fn $n##_sequence(s:*const seqcount_##$n##_t)->u32 { smp_load_acquire(&(*s).seqcount.sequence) }
}; }

#[inline] pub unsafe fn raw_read_seqcount_begin(s:*const seqcount_t)->u32 { let mut q; loop { q=__seqprop_sequence(s); if q&1==0{break} cpu_relax(); } kcsan_atomic_next(KCSAN_SEQLOCK_REGION_MAX); q }
#[inline] pub unsafe fn read_seqcount_begin(s:*const seqcount_t)->u32 { seqcount_lockdep_reader_access(s); raw_read_seqcount_begin(s) }
#[inline] pub unsafe fn raw_read_seqcount(s:*const seqcount_t)->u32 { let q=__seqprop_sequence(s); kcsan_atomic_next(KCSAN_SEQLOCK_REGION_MAX); q }
#[inline] pub unsafe fn raw_seqcount_try_begin(s:*const seqcount_t,start:*mut u32)->bool { *start=raw_read_seqcount(s); *start&1==0 }
#[inline] pub unsafe fn raw_seqcount_begin(s:*const seqcount_t)->u32 { raw_read_seqcount(s)&!1 }
#[inline] pub unsafe fn do___read_seqcount_retry(s:*const seqcount_t,start:u32)->i32 { kcsan_atomic_next(0); (read_once(&(*s).sequence)!=start) as i32 }
#[inline] pub unsafe fn __read_seqcount_retry(s:*const seqcount_t,start:u32)->i32 { do___read_seqcount_retry(s,start) }
#[inline] pub unsafe fn read_seqcount_retry(s:*const seqcount_t,start:u32)->i32 { smp_rmb(); do___read_seqcount_retry(s,start) }

#[inline] pub unsafe fn do_raw_write_seqcount_begin(s:*mut seqcount_t){kcsan_nestable_atomic_begin();(*s).sequence=(*s).sequence.wrapping_add(1);smp_wmb()}
#[inline] pub unsafe fn do_raw_write_seqcount_end(s:*mut seqcount_t){smp_wmb();(*s).sequence=(*s).sequence.wrapping_add(1);kcsan_nestable_atomic_end()}
#[inline] pub unsafe fn raw_write_seqcount_begin(s:*mut seqcount_t){do_raw_write_seqcount_begin(s)}
#[inline] pub unsafe fn raw_write_seqcount_end(s:*mut seqcount_t){do_raw_write_seqcount_end(s)}
#[inline] pub unsafe fn do_write_seqcount_begin_nested(s:*mut seqcount_t,subclass:i32){seqcount_acquire(&mut (*s).dep_map,subclass,0,0);do_raw_write_seqcount_begin(s)}
#[inline] pub unsafe fn do_write_seqcount_begin(s:*mut seqcount_t){do_write_seqcount_begin_nested(s,0)}
#[inline] pub unsafe fn do_write_seqcount_end(s:*mut seqcount_t){seqcount_release(&mut (*s).dep_map,0);do_raw_write_seqcount_end(s)}
#[inline] pub unsafe fn write_seqcount_begin(s:*mut seqcount_t){__seqprop_assert(s);do_write_seqcount_begin(s)}
#[inline] pub unsafe fn write_seqcount_begin_nested(s:*mut seqcount_t,n:i32){__seqprop_assert(s);do_write_seqcount_begin_nested(s,n)}
#[inline] pub unsafe fn write_seqcount_end(s:*mut seqcount_t){do_write_seqcount_end(s)}
#[inline] pub unsafe fn do_raw_write_seqcount_barrier(s:*mut seqcount_t){kcsan_nestable_atomic_begin();(*s).sequence=(*s).sequence.wrapping_add(1);smp_wmb();(*s).sequence=(*s).sequence.wrapping_add(1);kcsan_nestable_atomic_end()}
#[inline] pub unsafe fn raw_write_seqcount_barrier(s:*mut seqcount_t){do_raw_write_seqcount_barrier(s)}
#[inline] pub unsafe fn do_write_seqcount_invalidate(s:*mut seqcount_t){smp_wmb();kcsan_nestable_atomic_begin();(*s).sequence=(*s).sequence.wrapping_add(2);kcsan_nestable_atomic_end()}
#[inline] pub unsafe fn write_seqcount_invalidate(s:*mut seqcount_t){do_write_seqcount_invalidate(s)}

#[inline] pub unsafe fn raw_read_seqcount_latch(s:*const seqcount_latch_t)->u32{read_once(&(*s).seqcount.sequence)}
#[inline] pub unsafe fn read_seqcount_latch(s:*const seqcount_latch_t)->u32{kcsan_atomic_next(KCSAN_SEQLOCK_REGION_MAX);raw_read_seqcount_latch(s)}
#[inline] pub unsafe fn raw_read_seqcount_latch_retry(s:*const seqcount_latch_t,start:u32)->i32{smp_rmb();(read_once(&(*s).seqcount.sequence)!=start) as i32}
#[inline] pub unsafe fn read_seqcount_latch_retry(s:*const seqcount_latch_t,start:u32)->i32{kcsan_atomic_next(0);raw_read_seqcount_latch_retry(s,start)}
#[inline] pub unsafe fn raw_write_seqcount_latch(s:*mut seqcount_latch_t){smp_wmb();(*s).seqcount.sequence=(*s).seqcount.sequence.wrapping_add(1);smp_wmb()}
#[inline] pub unsafe fn write_seqcount_latch_begin(s:*mut seqcount_latch_t){kcsan_nestable_atomic_begin();raw_write_seqcount_latch(s)}
#[inline] pub unsafe fn write_seqcount_latch(s:*mut seqcount_latch_t){raw_write_seqcount_latch(s)}
#[inline] pub unsafe fn write_seqcount_latch_end(_: *mut seqcount_latch_t){kcsan_nestable_atomic_end()}

#[inline] pub unsafe fn read_seqbegin(sl:*const seqlock_t)->u32{read_seqcount_begin(&(*sl).seqcount.seqcount)}
#[inline] pub unsafe fn read_seqretry(sl:*const seqlock_t,start:u32)->u32{read_seqcount_retry(&(*sl).seqcount.seqcount,start) as u32}
#[inline] pub unsafe fn write_seqlock(sl:*mut seqlock_t){spin_lock(&mut (*sl).lock);do_write_seqcount_begin(&mut (*sl).seqcount.seqcount)}
#[inline] pub unsafe fn write_sequnlock(sl:*mut seqlock_t){do_write_seqcount_end(&mut (*sl).seqcount.seqcount);spin_unlock(&mut (*sl).lock)}
#[inline] pub unsafe fn write_seqlock_bh(sl:*mut seqlock_t){spin_lock_bh(&mut (*sl).lock);do_write_seqcount_begin(&mut (*sl).seqcount.seqcount)}
#[inline] pub unsafe fn write_sequnlock_bh(sl:*mut seqlock_t){do_write_seqcount_end(&mut (*sl).seqcount.seqcount);spin_unlock_bh(&mut (*sl).lock)}
#[inline] pub unsafe fn write_seqlock_irq(sl:*mut seqlock_t){spin_lock_irq(&mut (*sl).lock);do_write_seqcount_begin(&mut (*sl).seqcount.seqcount)}
#[inline] pub unsafe fn write_sequnlock_irq(sl:*mut seqlock_t){do_write_seqcount_end(&mut (*sl).seqcount.seqcount);spin_unlock_irq(&mut (*sl).lock)}
#[inline] pub unsafe fn __write_seqlock_irqsave(sl:*mut seqlock_t)->usize{let mut f=0;spin_lock_irqsave(&mut (*sl).lock,&mut f);do_write_seqcount_begin(&mut (*sl).seqcount.seqcount);f}
#[inline] pub unsafe fn write_seqlock_irqrestore(sl:*mut seqlock_t,f:usize){do_write_seqcount_end(&mut (*sl).seqcount.seqcount);spin_unlock_irqrestore(&mut (*sl).lock,f)}
#[inline] pub unsafe fn read_seqlock_excl(sl:*mut seqlock_t){spin_lock(&mut (*sl).lock)}
#[inline] pub unsafe fn read_sequnlock_excl(sl:*mut seqlock_t){spin_unlock(&mut (*sl).lock)}
#[inline] pub unsafe fn read_seqlock_excl_bh(sl:*mut seqlock_t){spin_lock_bh(&mut (*sl).lock)}
#[inline] pub unsafe fn read_sequnlock_excl_bh(sl:*mut seqlock_t){spin_unlock_bh(&mut (*sl).lock)}
#[inline] pub unsafe fn read_seqlock_excl_irq(sl:*mut seqlock_t){spin_lock_irq(&mut (*sl).lock)}
#[inline] pub unsafe fn read_sequnlock_excl_irq(sl:*mut seqlock_t){spin_unlock_irq(&mut (*sl).lock)}
#[inline] pub unsafe fn __read_seqlock_excl_irqsave(sl:*mut seqlock_t)->usize{let mut f=0;spin_lock_irqsave(&mut (*sl).lock,&mut f);f}
#[inline] pub unsafe fn read_sequnlock_excl_irqrestore(sl:*mut seqlock_t,f:usize){spin_unlock_irqrestore(&mut (*sl).lock,f)}

pub const ss_done:u32=0; pub const ss_lock:u32=1; pub const ss_lock_irqsave:u32=2; pub const ss_lockless:u32=3;
#[repr(C)] pub struct ss_tmp { pub state:u32, pub data:usize, pub lock:*mut spinlock_t, pub lock_irqsave:*mut spinlock_t }
#[inline] pub unsafe fn __scoped_seqlock_cleanup(s:*mut ss_tmp){if !(*s).lock.is_null(){spin_unlock((*s).lock)} if !(*s).lock_irqsave.is_null(){spin_unlock_irqrestore((*s).lock_irqsave,(*s).data)}}
#[inline] pub unsafe fn read_seqbegin_or_lock(lock:*mut seqlock_t,seq:*mut i32){if *seq&1==0{*seq=read_seqbegin(lock) as i32}else{read_seqlock_excl(lock)}}
#[inline] pub unsafe fn need_seqretry(lock:*mut seqlock_t,seq:i32)->i32{((seq&1)==0 && read_seqretry(lock,seq as u32)!=0) as i32}
#[inline] pub unsafe fn done_seqretry(lock:*mut seqlock_t,seq:i32){if seq&1!=0{read_sequnlock_excl(lock)}}
#[inline] pub unsafe fn read_seqbegin_or_lock_irqsave(lock:*mut seqlock_t,seq:*mut i32)->usize{let mut f=0;if *seq&1==0{*seq=read_seqbegin(lock) as i32}else{f=__read_seqlock_excl_irqsave(lock)}f}
#[inline] pub unsafe fn done_seqretry_irqrestore(lock:*mut seqlock_t,seq:i32,f:usize){if seq&1!=0{read_sequnlock_excl_irqrestore(lock,f)}}

#[no_mangle] pub unsafe extern "C" fn seqlock_init(sl:*mut seqlock_t){seqcount_init(&mut (*sl).seqcount.seqcount)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
