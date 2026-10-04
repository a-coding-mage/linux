#[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
macro_rules! lock_name {
    ($root:ident $(.$field:ident)*) => {
        concat!(stringify!($root), $(".", stringify!($field),)* "\0").as_ptr().cast()
    };
}

#[cfg(CONFIG_PREEMPT_RT)]
macro_rules! ATOMIC_INIT {
    ($n:expr) => {
        atomic_t { counter: $n as _ }
    };
}
#[cfg(CONFIG_PREEMPT_RT)]
macro_rules! RB_ROOT_CACHED {
    () => {
        rb_root_cached {
            rb_root: rb_root {
                rb_node: core::ptr::null_mut(),
            },
            rb_leftmost: core::ptr::null_mut(),
        }
    };
}

// SMP qspinlock (x86_64 and arm64) is ATOMIC_INIT(0); UP debug spinlocks
// use a distinct one-valued slock. Keep this distinction even on tiny configs.
macro_rules! __RAW_SPIN_LOCK_UNLOCKED {
    ($root:ident $(.$field:ident)*) => {{
        let lock: raw_spinlock_t = unsafe { zeroed() };
        #[cfg(any(CONFIG_DEBUG_SPINLOCK, CONFIG_DEBUG_LOCK_ALLOC))]
        let mut lock = lock;
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
            lock.dep_map.wait_type_inner = RUST_FORK_LD_WAIT_SPIN as _;
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
            let raw = __RAW_SPIN_LOCK_UNLOCKED!($root $(.$field)*);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            let mut raw = raw;
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            { raw.dep_map.wait_type_inner = RUST_FORK_LD_WAIT_CONFIG as _; }
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
                lock.dep_map.wait_type_inner = RUST_FORK_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

// Original __RW_LOCK_UNLOCKED; raw architecture locks are zero on SMP x86_64
// and the UP rwlock type is empty. Native generated type/offset assertions below
// prevent silently treating a different layout as this architecture initializer.
macro_rules! FORK_RW_LOCK_UNLOCKED {
    ($name:ident) => {{
        let lock: rwlock_t = unsafe { zeroed() };
        #[cfg(any(CONFIG_DEBUG_SPINLOCK, CONFIG_DEBUG_LOCK_ALLOC, CONFIG_PREEMPT_RT))]
        let mut lock = lock;
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            #[cfg(CONFIG_DEBUG_SPINLOCK)]
            {
                lock.magic = RWLOCK_MAGIC as _;
                lock.owner_cpu = u32::MAX;
                lock.owner = usize::MAX as *mut c_void;
            }
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.rwbase.readers = ATOMIC_INIT!(RUST_FORK_READER_BIAS);
            lock.rwbase.rtmutex = __RT_MUTEX_BASE_INITIALIZER!($name.rtmutex);
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = lock_name!($name);
            lock.dep_map.wait_type_inner = RUST_FORK_LD_WAIT_CONFIG as _;
        }
        lock
    }};
}
const _: () = {
    assert!(RUST_FORK_TASKLIST_OFFSET == 0);
    assert!(RUST_FORK_MMLIST_OFFSET == 0);
    assert!(offset_of!(rust_fork_tasklist_storage, value) == RUST_FORK_TASKLIST_OFFSET as usize);
    assert!(offset_of!(rust_fork_mmlist_storage, value) == RUST_FORK_MMLIST_OFFSET as usize);
    assert!(size_of::<rwlock_t>() == RUST_FORK_RWLOCK_SIZE as usize);
    assert!(size_of::<raw_spinlock_t>() == RUST_FORK_RAW_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == RUST_FORK_RAW_LOCK_ALIGN as usize);
    assert!(size_of::<spinlock_t>() == RUST_FORK_SPIN_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<spinlock_t>() == RUST_FORK_SPIN_LOCK_ALIGN as usize);
};
#[cfg(not(CONFIG_PREEMPT_RT))]
const _: () = {
    assert!(RUST_FORK_SPIN_RAW_OFFSET == 0);
    assert!(size_of::<spinlock_t>() == size_of::<raw_spinlock_t>());
    assert!(core::mem::align_of::<spinlock_t>() == core::mem::align_of::<raw_spinlock_t>());
};
#[cfg(not(CONFIG_X86_64))]
compile_error!("fork lock and per-CPU storage initializers require the audited x86_64 layout");

// Rust alignment applies to a type and rounds its size up. Keep that padding
// private: each public ELF object below names only the actual native lock
// subobject, not the enclosing alignment record. The generated repr(C) records
// give their offset-zero fields the original variable alignment; the static
// attributes retain the original sections.
// Both fields have static mutable storage and retain their typed initializers;
// no references to the records are created. This owner accesses them through
// the native rwlock_t/spinlock_t declarations in sched/task.h. `used` and the
// asm `sym` operands retain the backing records even though Rust never reads
// them; explicit internal linkage keeps those records out of the public ABI.
#[used]
#[linkage = "internal"]
#[link_section = ".data..cacheline_aligned"]
static mut TASKLIST_STORAGE: rust_fork_tasklist_storage = rust_fork_tasklist_storage {
    value: FORK_RW_LOCK_UNLOCKED!(tasklist_lock),
};
#[used]
#[linkage = "internal"]
#[cfg_attr(all(CONFIG_SMP, CONFIG_X86_VSMP), link_section = ".data..page_aligned")]
#[cfg_attr(
    all(CONFIG_SMP, not(CONFIG_X86_VSMP)),
    link_section = ".data..cacheline_aligned"
)]
static mut MMLIST_STORAGE: rust_fork_mmlist_storage = rust_fork_mmlist_storage {
    value: __SPIN_LOCK_UNLOCKED!(mmlist_lock),
};
core::arch::global_asm!(
    ".globl tasklist_lock",
    ".type tasklist_lock,@object",
    ".set tasklist_lock, {tasklist_storage} + {tasklist_offset}",
    ".size tasklist_lock, {tasklist_size}",
    ".globl mmlist_lock",
    ".type mmlist_lock,@object",
    ".set mmlist_lock, {mmlist_storage} + {mmlist_offset}",
    ".size mmlist_lock, {mmlist_size}",
    tasklist_storage = sym TASKLIST_STORAGE,
    tasklist_offset = const offset_of!(rust_fork_tasklist_storage, value),
    tasklist_size = const size_of::<rwlock_t>(),
    mmlist_storage = sym MMLIST_STORAGE,
    mmlist_offset = const offset_of!(rust_fork_mmlist_storage, value),
    mmlist_size = const size_of::<spinlock_t>(),
);
// asm-generic/percpu.h's built-in PER_CPU_BASE_SECTION on x86_64. Access stays
// through native per-CPU addressing instructions; all storage is defined here.
#[export_name = "process_counts"]
#[cfg_attr(CONFIG_SMP, link_section = ".data..percpu")]
#[cfg_attr(not(CONFIG_SMP), link_section = ".data")]
static mut PROCESS_COUNTS: c_ulong = 0;
#[cfg(CONFIG_VMAP_STACK)]
#[export_name = "rust_fork_cached_stacks"]
#[cfg_attr(CONFIG_SMP, link_section = ".data..percpu")]
#[cfg_attr(not(CONFIG_SMP), link_section = ".data")]
static mut CACHED_STACKS: [*mut vm_struct; NR_CACHED_STACKS] = [null_mut(); NR_CACHED_STACKS];

#[cfg(CONFIG_MM_ID)]
#[export_name = "rust_fork_mm_ida_storage"]
static mut MM_IDA: ida = {
    let mut id: ida = unsafe { zeroed() };
    id.xa.xa_lock = __SPIN_LOCK_UNLOCKED!(mm_ida.xa_lock);
    id.xa.xa_flags = RUST_FORK_IDA_INIT_FLAGS as _;
    id
};
static mut coredump_filter: c_ulong = RUST_FORK_MMF_DUMP_FILTER_DEFAULT as c_ulong;

#[repr(transparent)]
struct ForkResidentTypes([*const c_char; NR_MM_COUNTERS as usize]);
// Every pointer targets an immutable NUL-terminated static diagnostic string.
unsafe impl Sync for ForkResidentTypes {}
static resident_page_types: ForkResidentTypes = {
    let mut names = [core::ptr::null(); NR_MM_COUNTERS as usize];
    names[MM_FILEPAGES as usize] = c"MM_FILEPAGES".as_ptr().cast();
    names[MM_ANONPAGES as usize] = c"MM_ANONPAGES".as_ptr().cast();
    names[MM_SWAPENTS as usize] = c"MM_SWAPENTS".as_ptr().cast();
    names[MM_SHMEMPAGES as usize] = c"MM_SHMEMPAGES".as_ptr().cast();
    const {
        assert!(NR_MM_COUNTERS == 4);
    }
    ForkResidentTypes(names)
};

#[repr(transparent)]
struct ForkSysctlTable([ctl_table; 1]);
// The descriptor is immutable; the callback follows the native sysctl contract.
unsafe impl Sync for ForkSysctlTable {}
static fork_sysctl_table: ForkSysctlTable = ForkSysctlTable([ctl_table {
    procname: c"threads-max".as_ptr().cast(),
    data: null_mut(),
    maxlen: size_of::<c_int>() as c_int,
    mode: 0o644,
    proc_handler: Some(rust_fork_sysctl_max_threads),
    poll: null_mut(),
    extra1: null_mut(),
    extra2: null_mut(),
}]);
