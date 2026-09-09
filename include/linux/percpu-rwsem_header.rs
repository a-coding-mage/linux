/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct percpu_rw_semaphore {
    pub rss: rcu_sync,
    pub read_count: *mut core::ffi::c_uint,
    pub writer: rcuwait,
    pub waiters: wait_queue_head_t,
    pub block: atomic_t,
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    pub dep_map: lockdep_map,
}

#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
macro_rules! __PERCPU_RWSEM_DEP_MAP_INIT {
    ($lockname:ident) => { dep_map: lockdep_map { name: stringify!($lockname) }, };
}
#[cfg(not(CONFIG_DEBUG_LOCK_ALLOC))]
macro_rules! __PERCPU_RWSEM_DEP_MAP_INIT {
    ($lockname:ident) => {};
}

// DEFINE_PER_CPU and the kernel initializer macros are represented as
// declaration-style macros; their referenced definitions are supplied by
// other translated kernel headers.
macro_rules! __DEFINE_PERCPU_RWSEM {
    ($name:ident, $is_static:tt) => {
        static mut __percpu_rwsem_rc_$name: core::ffi::c_uint = 0;
        $is_static mut $name: percpu_rw_semaphore = percpu_rw_semaphore {
            rss: __RCU_SYNC_INITIALIZER!($name.rss),
            read_count: &raw mut __percpu_rwsem_rc_$name,
            writer: __RCUWAIT_INITIALIZER!($name.writer),
            waiters: __WAIT_QUEUE_HEAD_INITIALIZER!($name.waiters),
            block: ATOMIC_INIT!(0),
            __PERCPU_RWSEM_DEP_MAP_INIT!($name)
        };
    };
}

macro_rules! DEFINE_PERCPU_RWSEM {
    ($name:ident) => { __DEFINE_PERCPU_RWSEM!($name, ); };
}
macro_rules! DEFINE_STATIC_PERCPU_RWSEM {
    ($name:ident) => { __DEFINE_PERCPU_RWSEM!($name, static); };
}

extern "C" {
    pub fn __percpu_down_read(sem: *mut percpu_rw_semaphore, try_: bool, freezable: bool) -> bool;
}

#[inline]
pub unsafe fn percpu_down_read_internal(sem: *mut percpu_rw_semaphore, freezable: bool) {
    might_sleep();
    rwsem_acquire_read(&mut (*sem).dep_map, 0, 0, _RET_IP_!());
    preempt_disable();
    /*
     * We are in an RCU-sched read-side critical section, so the writer
     * cannot both change sem->state from readers_fast and start checking
     * counters while we are here. So if we see !sem->state, we know that
     * the writer won't be checking until we're past the preempt_enable()
     * and that once the synchronize_rcu() is done, the writer will see
     * anything we did within this RCU-sched read-size critical section.
     */
    if likely!(rcu_sync_is_idle(&(*sem).rss)) {
        this_cpu_inc!((*sem).read_count);
    } else {
        __percpu_down_read(sem, false, freezable); /* Unconditional memory barrier */
    }
    /* The preempt_enable() prevents the compiler from bleeding the critical section out. */
    preempt_enable();
}

#[inline]
pub unsafe fn percpu_down_read(sem: *mut percpu_rw_semaphore) { percpu_down_read_internal(sem, false); }

#[inline]
pub unsafe fn percpu_down_read_freezable(sem: *mut percpu_rw_semaphore, freeze: bool) {
    percpu_down_read_internal(sem, freeze);
}

#[inline]
pub unsafe fn percpu_down_read_trylock(sem: *mut percpu_rw_semaphore) -> bool {
    let mut ret = true;
    preempt_disable();
    if likely!(rcu_sync_is_idle(&(*sem).rss)) {
        this_cpu_inc!((*sem).read_count);
    } else {
        ret = __percpu_down_read(sem, true, false); /* Unconditional memory barrier */
    }
    preempt_enable();
    if ret { rwsem_acquire_read(&mut (*sem).dep_map, 0, 1, _RET_IP_!()); }
    ret
}

extern "C" {
    pub fn __percpu_up_read(sem: *mut percpu_rw_semaphore);
    pub fn percpu_is_read_locked(sem: *mut percpu_rw_semaphore) -> bool;
    pub fn percpu_down_write(sem: *mut percpu_rw_semaphore);
    pub fn percpu_up_write(sem: *mut percpu_rw_semaphore);
}

DEFINE_GUARD!(percpu_read, *mut percpu_rw_semaphore, percpu_down_read(_T), percpu_up_read(_T));
DEFINE_GUARD_COND!(percpu_read, _try, percpu_down_read_trylock(_T));
DEFINE_GUARD!(percpu_write, *mut percpu_rw_semaphore, percpu_down_write(_T), percpu_up_write(_T));

#[inline]
pub unsafe fn percpu_up_read(sem: *mut percpu_rw_semaphore) {
    rwsem_release(&mut (*sem).dep_map, _RET_IP_!());
    preempt_disable();
    if likely!(rcu_sync_is_idle(&(*sem).rss)) { this_cpu_dec!((*sem).read_count); }
    else { __percpu_up_read(sem); }
    preempt_enable();
}

#[inline]
pub unsafe fn percpu_is_write_locked(sem: *mut percpu_rw_semaphore) -> bool {
    atomic_read(&(*sem).block)
}

extern "C" {
    pub fn __percpu_init_rwsem(sem: *mut percpu_rw_semaphore, name: *const core::ffi::c_char, key: *mut lock_class_key) -> core::ffi::c_int;
    pub fn percpu_free_rwsem(sem: *mut percpu_rw_semaphore);
}

macro_rules! percpu_init_rwsem {
    ($sem:expr) => {{
        static mut rwsem_key: lock_class_key = lock_class_key {};
        __percpu_init_rwsem($sem, stringify!($sem).as_ptr() as *const core::ffi::c_char, &raw mut rwsem_key)
    }};
}
macro_rules! percpu_rwsem_is_write_held { ($sem:expr) => { lockdep_is_held_type!($sem, 0) }; }
macro_rules! percpu_rwsem_is_held { ($sem:expr) => { lockdep_is_held!($sem) }; }
macro_rules! percpu_rwsem_assert_held { ($sem:expr) => { lockdep_assert_held!($sem) }; }

#[inline]
pub unsafe fn percpu_rwsem_release(sem: *mut percpu_rw_semaphore, ip: c_ulong) { lock_release(&mut (*sem).dep_map, ip); }

#[inline]
pub unsafe fn percpu_rwsem_acquire(sem: *mut percpu_rw_semaphore, read: bool, ip: c_ulong) {
    lock_acquire(&mut (*sem).dep_map, 0, 1, read, 1, core::ptr::null_mut(), ip);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
