// SPDX-License-Identifier: GPL-2.0-only
// Const translations of the native initializers reached by mm/init-mm.c.
// All types and numeric configuration values come from this build's headers.

macro_rules! raw_spin_lock {
    ($name:expr) => {{
        let mut lock: raw_spinlock_t = unsafe { zeroed() };
        // x86_64/arm64 SMP qspinlocks are zero; UP debug uses slock = 1.
        #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
        { lock.raw_lock.slock = 1; }
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lock.magic = SPINLOCK_MAGIC as _;
            lock.owner_cpu = u32::MAX;
            lock.owner = usize::MAX as *mut kernel::ffi::c_void;
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = RUST_INIT_MM_LD_WAIT_SPIN as _;
        }
        lock
    }};
}

macro_rules! rt_mutex_base {
    ($name:expr) => {{
        let mut lock: rt_mutex_base = unsafe { zeroed() };
        lock.wait_lock = raw_spin_lock!(concat!($name, ".wait_lock"));
        // RB_ROOT_CACHED and owner = NULL are entirely zero.
        lock
    }};
}

macro_rules! spin_lock {
    ($name:expr) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            let mut raw = raw_spin_lock!($name);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            { raw.dep_map.wait_type_inner = RUST_INIT_MM_LD_WAIT_CONFIG as _; }
            // Bindgen uses __BindgenUnionField for zero-sized UP locks.
            // Native size/alignment and rlock offset assertions guard this
            // write to the generated union storage in every configuration.
            unsafe { (&raw mut lock).cast::<raw_spinlock_t>().write(raw) };
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.lock = rt_mutex_base!(concat!($name, ".lock"));
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
                lock.dep_map.wait_type_inner = RUST_INIT_MM_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

macro_rules! rwsem {
    ($object:expr, $name:expr) => {{
        let mut lock: rw_semaphore = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            // RWSEM_UNLOCKED_VALUE, owner, OSQ_LOCK_UNLOCKED and
            // first_waiter are all zero in the native initializer.
            lock.wait_lock = raw_spin_lock!(concat!($name, ".wait_lock"));
            #[cfg(CONFIG_DEBUG_RWSEMS)]
            { lock.magic = unsafe { (&raw mut $object).cast() }; }
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.rwbase.readers.counter = READER_BIAS as _;
            // __RWBASE_INITIALIZER uses name.rtmutex (no .rwbase).
            lock.rwbase.rtmutex = rt_mutex_base!(concat!($name, ".rtmutex"));
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = RUST_INIT_MM_LD_WAIT_SLEEP as _;
        }
        lock
    }};
}

macro_rules! mutex {
    ($object:expr, $name:expr) => {{
        let mut lock: mutex = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            lock.wait_lock = raw_spin_lock!(concat!($name, ".wait_lock"));
            #[cfg(CONFIG_DEBUG_MUTEXES)]
            { lock.magic = unsafe { (&raw mut $object).cast() }; }
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        { lock.rtmutex = rt_mutex_base!(concat!($name, ".rtmutex")); }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = RUST_INIT_MM_LD_WAIT_SLEEP as _;
        }
        lock
    }};
}

macro_rules! seqcount {
    ($name:expr) => {{
        let mut seq: seqcount_t = unsafe { zeroed() };
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        { seq.dep_map.name = concat!($name, "\0").as_ptr().cast(); }
        seq
    }};
}
