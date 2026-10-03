// SPDX-License-Identifier: GPL-2.0-only
// Rust owns every softirq.c data definition applicable to the selected target.
// The audited per-CPU ELF section policy here is the built-in x86_64 policy.
#[cfg(not(CONFIG_X86_64))]
compile_error!("Rust softirq per-CPU and lock initializers are audited for x86_64 only");
const _: () = {
    assert!(RUST_SIRQ_ARCH_IRQ_STAT == 1); // x86 owns irq_stat in its arch TU.
    assert!(NR_SOFTIRQS == 10);
    assert!(core::mem::align_of::<rust_softirq_vec_storage>() == RUST_SIRQ_VEC_ALIGN as usize);
    assert!(offset_of!(rust_softirq_vec_storage, value) == RUST_SIRQ_VEC_OFFSET as usize);
    assert!(offset_of!(rust_softirq_names_storage, value) == RUST_SIRQ_NAMES_OFFSET as usize);
};

#[export_name = "rust_softirq_vec"]
#[cfg_attr(
    all(CONFIG_SMP, not(CONFIG_X86_VSMP)),
    link_section = ".data..cacheline_aligned"
)]
#[cfg_attr(all(CONFIG_SMP, CONFIG_X86_VSMP), link_section = ".data..page_aligned")]
static mut SOFTIRQ_VEC: rust_softirq_vec_storage = unsafe { zeroed() };

// This generated native wrapper is immutable after link; its pointers all
// designate immutable NUL-terminated strings. It has native array offset 0.
unsafe impl Sync for rust_softirq_names_storage {}
#[export_name = "softirq_to_name"]
static SOFTIRQ_NAMES: rust_softirq_names_storage = rust_softirq_names_storage {
    value: [
        b"HI\0".as_ptr().cast(),
        b"TIMER\0".as_ptr().cast(),
        b"NET_TX\0".as_ptr().cast(),
        b"NET_RX\0".as_ptr().cast(),
        b"BLOCK\0".as_ptr().cast(),
        b"IRQ_POLL\0".as_ptr().cast(),
        b"TASKLET\0".as_ptr().cast(),
        b"SCHED\0".as_ptr().cast(),
        b"HRTIMER\0".as_ptr().cast(),
        b"RCU\0".as_ptr().cast(),
    ],
};

macro_rules! softirq_percpu {
    ($(#[$meta:meta])* $symbol:literal, $name:ident, $ty:ty, $initial:expr) => {
        $(#[$meta])*
        #[export_name = $symbol]
        #[cfg_attr(CONFIG_SMP, link_section = ".data..percpu")]
        #[cfg_attr(not(CONFIG_SMP), link_section = ".data")]
        static mut $name: $ty = $initial;
    };
}
softirq_percpu!("ksoftirqd", KSOFTIRQD, *mut task_struct, null_mut());
softirq_percpu!(
    "local_interrupt_disable_state",
    INTERRUPT_DISABLE_STATE,
    c_ulong,
    0
);
softirq_percpu!(
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    "hardirqs_enabled",
    HARDIRQS_ENABLED,
    c_int,
    0
);
softirq_percpu!(
    #[cfg(CONFIG_TRACE_IRQFLAGS)]
    "hardirq_context",
    HARDIRQ_CONTEXT,
    c_int,
    0
);
softirq_percpu!(
    #[cfg(not(CONFIG_HAS_SEPARATE_PREEMPT_RESCHED_BITS))]
    "nmi_nesting",
    NMI_NESTING,
    c_uint,
    0
);
softirq_percpu!("rust_tasklet_vec", TASKLET_VEC, tasklet_head, unsafe {
    zeroed()
});
softirq_percpu!(
    "rust_tasklet_hi_vec",
    TASKLET_HI_VEC,
    tasklet_head,
    unsafe { zeroed() }
);
softirq_percpu!(
    #[cfg(CONFIG_IRQ_FORCED_THREADING)]
    "ktimerd",
    KTIMERD,
    *mut task_struct,
    null_mut()
);
softirq_percpu!(
    #[cfg(CONFIG_IRQ_FORCED_THREADING)]
    "pending_timer_softirq",
    PENDING_TIMER_SOFTIRQ,
    c_ulong,
    0
);

#[cfg(CONFIG_PREEMPT_RT)]
macro_rules! softirq_rt_spin {
    ($name:literal, $wait_name:literal, $local:expr) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        // x86_64 queued raw locks are unlocked at zero. In UP debug builds
        // the architecture lock's slock is one, as in __ARCH_SPIN_LOCK_UNLOCKED.
        #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
        {
            lock.lock.wait_lock.raw_lock.slock = 1;
        }
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lock.lock.wait_lock.magic = SPINLOCK_MAGIC;
            lock.lock.wait_lock.owner_cpu = u32::MAX;
            lock.lock.wait_lock.owner = usize::MAX as *mut c_void;
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.lock.wait_lock.dep_map.name = concat!($wait_name, "\0").as_ptr().cast();
            lock.lock.wait_lock.dep_map.wait_type_inner = RUST_SIRQ_LD_WAIT_SPIN as _;
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = RUST_SIRQ_LD_WAIT_CONFIG as _;
            if $local {
                lock.dep_map.lock_type = RUST_SIRQ_LD_LOCK_PERCPU as _;
            }
        }
        // RB_ROOT_CACHED and rtmutex owner are NULL, from zeroed().
        lock
    }};
}
#[cfg(CONFIG_PREEMPT_RT)]
const _: () = {
    assert!(size_of::<raw_spinlock_t>() == RUST_SIRQ_RAW_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == RUST_SIRQ_RAW_LOCK_ALIGN as usize);
    assert!(offset_of!(spinlock_t, lock) == RUST_SIRQ_RT_LOCK_OFFSET as usize);
    assert!(size_of::<local_lock_t>() == RUST_SIRQ_LOCAL_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<local_lock_t>() == RUST_SIRQ_LOCAL_LOCK_ALIGN as usize);
};
softirq_percpu!(
    #[cfg(CONFIG_PREEMPT_RT)]
    "rust_softirq_ctrl",
    SOFTIRQ_CTRL,
    softirq_ctrl,
    softirq_ctrl {
        lock: softirq_rt_spin!(
            "(softirq_ctrl.lock)",
            "(softirq_ctrl.lock).lock.wait_lock",
            true
        ),
        cnt: 0,
    }
);
softirq_percpu!(
    #[cfg(CONFIG_PREEMPT_RT)]
    "rust_tasklet_sync_callback",
    TASKLET_SYNC,
    tasklet_sync_callback,
    tasklet_sync_callback {
        cb_lock: softirq_rt_spin!(
            "tasklet_sync_callback.cb_lock",
            "tasklet_sync_callback.cb_lock.lock.wait_lock",
            false
        ),
        cb_waiters: atomic_t { counter: 0 },
    }
);

#[cfg(all(CONFIG_PREEMPT_RT, CONFIG_DEBUG_LOCK_ALLOC))]
static mut BH_LOCK_KEY: lock_class_key = unsafe { zeroed() };
#[cfg(all(CONFIG_PREEMPT_RT, CONFIG_DEBUG_LOCK_ALLOC))]
#[export_name = "bh_lock_map"]
static mut BH_LOCK_MAP: lockdep_map = {
    let mut map: lockdep_map = unsafe { zeroed() };
    map.name = b"local_bh\0".as_ptr().cast();
    map.key = addr_of_mut!(BH_LOCK_KEY);
    map.wait_type_outer = RUST_SIRQ_LD_WAIT_FREE as _;
    map.wait_type_inner = RUST_SIRQ_LD_WAIT_CONFIG as _;
    map.lock_type = RUST_SIRQ_LD_LOCK_PERCPU as _;
    map
};

static mut SOFTIRQ_THREADS: smp_hotplug_thread = {
    let mut thread: smp_hotplug_thread = unsafe { zeroed() };
    thread.store = addr_of_mut!(KSOFTIRQD);
    thread.thread_should_run = Some(ksoftirqd_should_run);
    thread.thread_fn = Some(run_ksoftirqd);
    thread.thread_comm = b"ksoftirqd/%u\0".as_ptr().cast();
    thread
};
#[cfg(CONFIG_IRQ_FORCED_THREADING)]
static mut TIMER_THREAD: smp_hotplug_thread = {
    let mut thread: smp_hotplug_thread = unsafe { zeroed() };
    thread.store = addr_of_mut!(KTIMERD);
    thread.setup = Some(ktimerd_setup);
    thread.thread_should_run = Some(ktimerd_should_run);
    thread.thread_fn = Some(run_ktimerd);
    thread.thread_comm = b"ktimers/%u\0".as_ptr().cast();
    thread
};
