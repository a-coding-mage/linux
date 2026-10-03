// SPDX-License-Identifier: GPL-2.0-only
// Const translations of the initializers used by init/init_task.c. All storage
// types are generated from the configured native headers, never hand-laid out.

macro_rules! lock_name {
    ($root:ident $(.$field:ident)*) => {
        concat!(stringify!($root), $(".", stringify!($field),)* "\0").as_ptr().cast()
    };
}

macro_rules! LIST_HEAD_INIT {
    ($head:expr) => {{
        let p = unsafe { &raw mut $head };
        list_head { next: p, prev: p }
    }};
}
macro_rules! HLIST_HEAD_INIT {
    () => { hlist_head { first: core::ptr::null_mut() } };
}
macro_rules! ATOMIC_INIT {
    ($n:expr) => { atomic_t { counter: $n as _ } };
}
macro_rules! REFCOUNT_INIT {
    ($n:expr) => { refcount_t { refs: ATOMIC_INIT!($n) } };
}
macro_rules! RB_ROOT_CACHED {
    () => { rb_root_cached {
        rb_root: rb_root { rb_node: core::ptr::null_mut() },
        rb_leftmost: core::ptr::null_mut(),
    } };
}

// SMP qspinlock (x86_64 and arm64) is ATOMIC_INIT(0); UP debug spinlocks
// use a distinct one-valued slock. Keep this distinction even on tiny configs.
macro_rules! __RAW_SPIN_LOCK_UNLOCKED {
    ($root:ident $(.$field:ident)*) => {{
        let mut lock: raw_spinlock_t = unsafe { zeroed() };
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
            lock.dep_map.name = lock_name!($root $(.$field)*);
            lock.dep_map.wait_type_inner = RUST_INIT_TASK_LD_WAIT_SPIN as _;
        }
        lock
    }};
}

macro_rules! __RT_MUTEX_BASE_INITIALIZER {
    ($root:ident $(.$field:ident)*) => {{
        let mut lock: rt_mutex_base = unsafe { zeroed() };
        lock.wait_lock = __RAW_SPIN_LOCK_UNLOCKED!($root $(.$field)* .wait_lock);
        lock.waiters = RB_ROOT_CACHED!();
        lock
    }};
}

macro_rules! __SPIN_LOCK_UNLOCKED {
    ($root:ident $(.$field:ident)*) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            let mut raw = __RAW_SPIN_LOCK_UNLOCKED!($root $(.$field)*);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            { raw.dep_map.wait_type_inner = RUST_INIT_TASK_LD_WAIT_CONFIG as _; }
            // UP non-debug raw_spinlock is zero-sized, so bindgen represents
            // the union arm with __BindgenUnionField instead of a Rust union.
            // The native size/alignment/zero-offset assertions cover both
            // representations. Write the actual native arm at that address.
            unsafe { (&raw mut lock).cast::<raw_spinlock_t>().write(raw) };
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.lock = __RT_MUTEX_BASE_INITIALIZER!($root $(.$field)* .lock);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                lock.dep_map.name = lock_name!($root $(.$field)*);
                lock.dep_map.wait_type_inner = RUST_INIT_TASK_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

macro_rules! __WAIT_QUEUE_HEAD_INITIALIZER {
    ($root:ident $(.$field:ident)*) => {{
        wait_queue_head {
            lock: __SPIN_LOCK_UNLOCKED!($root $(.$field)* .lock),
            head: LIST_HEAD_INIT!($root $(.$field)* .head),
        }
    }};
}

macro_rules! __MUTEX_INITIALIZER {
    ($root:ident $(.$field:ident)*) => {{
        let mut lock: mutex = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            lock.wait_lock = __RAW_SPIN_LOCK_UNLOCKED!($root $(.$field)* .wait_lock);
            // owner=ATOMIC_LONG_INIT(0), first_waiter=NULL.
            #[cfg(CONFIG_DEBUG_MUTEXES)]
            { lock.magic = unsafe { (&raw mut $root $(.$field)*).cast() }; }
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        { lock.rtmutex = __RT_MUTEX_BASE_INITIALIZER!($root $(.$field)* .rtmutex); }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = lock_name!($root $(.$field)*);
            lock.dep_map.wait_type_inner = RUST_INIT_TASK_LD_WAIT_SLEEP as _;
        }
        lock
    }};
}

macro_rules! __RWSEM_INITIALIZER {
    ($root:ident $(.$field:ident)*) => {{
        let mut lock: rw_semaphore = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            // count=RWSEM_UNLOCKED_VALUE(0), owner=0, osq.tail=0,
            // first_waiter=NULL, just as the native initializer.
            lock.wait_lock = __RAW_SPIN_LOCK_UNLOCKED!($root $(.$field)* .wait_lock);
            #[cfg(CONFIG_DEBUG_RWSEMS)]
            { lock.magic = unsafe { (&raw mut $root $(.$field)*).cast() }; }
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.rwbase.readers = ATOMIC_INIT!(READER_BIAS);
            // __RWBASE_INITIALIZER(name) stringifies name.rtmutex, without
            // a rwbase component; retain that exact native lockdep name.
            lock.rwbase.rtmutex = __RT_MUTEX_BASE_INITIALIZER!($root $(.$field)* .rtmutex);
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = lock_name!($root $(.$field)*);
            lock.dep_map.wait_type_inner = RUST_INIT_TASK_LD_WAIT_SLEEP as _;
        }
        lock
    }};
}

macro_rules! SEQCNT_ZERO {
    ($root:ident $(.$field:ident)*) => {{
        let mut seq: seqcount_t = unsafe { zeroed() };
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        { seq.dep_map.name = lock_name!($root $(.$field)*); }
        seq
    }};
}
macro_rules! SEQCNT_SPINLOCK_ZERO {
    ($root:ident $(.$field:ident)*, $lock:expr) => {{
        let mut seq: seqcount_spinlock_t = unsafe { zeroed() };
        seq.seqcount = SEQCNT_ZERO!($root $(.$field)* .seqcount);
        #[cfg(any(CONFIG_LOCKDEP, CONFIG_PREEMPT_RT))]
        { seq.lock = unsafe { $lock }; }
        seq
    }};
}

macro_rules! PLIST_NODE_INIT {
    ($root:ident $(.$field:ident)*, $prio:expr) => {
        plist_node {
            prio: $prio as _,
            prio_list: LIST_HEAD_INIT!($root $(.$field)* .prio_list),
            node_list: LIST_HEAD_INIT!($root $(.$field)* .node_list),
        }
    };
}
macro_rules! INIT_CPUTIME_ATOMIC {
    () => { task_cputime_atomic {
        utime: atomic64_t { counter: 0 },
        stime: atomic64_t { counter: 0 },
        sum_exec_runtime: atomic64_t { counter: 0 },
    } };
}
macro_rules! INIT_USER {
    () => { unsafe { &raw mut root_user } };
}

const fn cpu_mask_all() -> cpumask {
    let mut mask: cpumask = unsafe { zeroed() };
    let mut word = 0;
    while word < mask.bits.len() {
        mask.bits[word] = kernel::ffi::c_ulong::MAX;
        word += 1;
    }
    let tail = RUST_INIT_TASK_NR_CPUS as usize % kernel::ffi::c_ulong::BITS as usize;
    if tail != 0 {
        mask.bits[word - 1] = (1 << tail) - 1;
    }
    mask
}
macro_rules! CPU_MASK_ALL { () => { cpu_mask_all() }; }

// asm-generic/resource.h supplies INIT_RLIMITS on x86_64 and arm64. Keep
// individual indices rather than relying on their numeric order.
const fn init_rlimits() -> [rlimit; RLIM_NLIMITS as usize] {
    let mut limits: [rlimit; RLIM_NLIMITS as usize] = unsafe { zeroed() };
    let infinity = RUST_INIT_TASK_RLIM_INFINITY as kernel::ffi::c_ulong;
    let unlimited = rlimit { rlim_cur: infinity, rlim_max: infinity };
    limits[RLIMIT_CPU as usize] = unlimited;
    limits[RLIMIT_FSIZE as usize] = unlimited;
    limits[RLIMIT_DATA as usize] = unlimited;
    limits[RLIMIT_STACK as usize] = rlimit { rlim_cur: RUST_INIT_TASK_STACK_LIMIT, rlim_max: infinity };
    limits[RLIMIT_CORE as usize] = rlimit { rlim_cur: 0, rlim_max: infinity };
    limits[RLIMIT_RSS as usize] = unlimited;
    limits[RLIMIT_NOFILE as usize] = rlimit { rlim_cur: INR_OPEN_CUR as _, rlim_max: INR_OPEN_MAX as _ };
    limits[RLIMIT_MEMLOCK as usize] = rlimit { rlim_cur: RUST_INIT_TASK_MLOCK_LIMIT, rlim_max: RUST_INIT_TASK_MLOCK_LIMIT };
    limits[RLIMIT_AS as usize] = unlimited;
    limits[RLIMIT_LOCKS as usize] = unlimited;
    limits[RLIMIT_MSGQUEUE as usize] = rlimit { rlim_cur: MQ_BYTES_MAX as _, rlim_max: MQ_BYTES_MAX as _ };
    limits[RLIMIT_RTTIME as usize] = unlimited;
    // NPROC, SIGPENDING, NICE and RTPRIO explicitly have {0,0} defaults.
    limits
}
macro_rules! INIT_RLIMITS { () => { init_rlimits() }; }

const GLOBAL_ROOT_UID: kuid_t = kuid_t { val: 0 };
const GLOBAL_ROOT_GID: kgid_t = kgid_t { val: 0 };
#[cfg(CONFIG_AUDIT)]
const INVALID_UID: kuid_t = kuid_t { val: u32::MAX };
const CAP_EMPTY_SET: kernel_cap_t = kernel_cap_t { val: 0 };
const CAP_FULL_SET: kernel_cap_t = kernel_cap_t { val: RUST_INIT_TASK_CAP_VALID_MASK as _ };

macro_rules! INIT_THREAD_INFO {
    ($task:ident) => {{
        let mut info: thread_info = unsafe { zeroed() };
        #[cfg(CONFIG_ARM64)]
        {
            info.flags = RUST_INIT_TASK_THREAD_FLAGS;
            info.__bindgen_anon_1.preempt_count = RUST_INIT_TASK_PREEMPT_COUNT as _;
            #[cfg(CONFIG_SHADOW_CALL_STACK)]
            {
                info.scs_base = unsafe { (&raw mut init_shadow_call_stack).cast() };
                info.scs_sp = unsafe { (&raw mut init_shadow_call_stack).cast() };
            }
        }
        info
    }};
}
macro_rules! INIT_THREAD {
    () => {{
        let mut thread: thread_struct = unsafe { zeroed() };
        #[cfg(CONFIG_X86_64)]
        { thread.sp = unsafe { (&raw mut __top_init_kernel_stack).cast() }; }
        #[cfg(CONFIG_ARM64)]
        { thread.fpsimd_cpu = RUST_INIT_TASK_NR_CPUS; }
        thread
    }};
}
