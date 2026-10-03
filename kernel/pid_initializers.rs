// SPDX-License-Identifier: GPL-2.0-only
// Complete Rust-owned data initializers from kernel/pid.c and the native macros
// they reach. Only canonical bindgen types and native-derived values are used.

macro_rules! pid_raw_spin_lock {
    ($name:expr) => {{
        let mut lock: raw_spinlock_t = unsafe { zeroed() };
        // Every current HAVE_RUST SMP architecture's native unlocked value is
        // zero (qspinlock/ticket/simple/ARM/s390). The generic UP debug variant
        // is distinct; its exact scalar comes from __ARCH_SPIN_LOCK_UNLOCKED.
        #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
        {
            lock.raw_lock.slock = LUPOS_PID_UP_UNLOCKED as _;
        }
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lock.magic = SPINLOCK_MAGIC as _;
            lock.owner_cpu = LUPOS_PID_SPIN_OWNER_CPU;
            lock.owner = LUPOS_PID_SPIN_OWNER as *mut kernel::ffi::c_void;
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = LUPOS_PID_LD_WAIT_SPIN as _;
        }
        lock
    }};
}

macro_rules! pid_spin_lock {
    ($name:expr) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            let mut raw = pid_raw_spin_lock!($name);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                raw.dep_map.wait_type_inner = LUPOS_PID_LD_WAIT_CONFIG as _;
            }
            // Canonical assertions below cover the generated zero-sized UP
            // union as well as the ordinary raw-spinlock arm at offset zero.
            unsafe { addr_of_mut!(lock).cast::<raw_spinlock_t>().write(raw) };
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            // __RT_MUTEX_BASE_INITIALIZER: wait_lock, zero RB_ROOT_CACHED,
            // and owner=NULL. Keep the native macro's stringified lock path.
            lock.lock.wait_lock = pid_raw_spin_lock!(concat!($name, ".lock.wait_lock"));
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
                lock.dep_map.wait_type_inner = LUPOS_PID_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

macro_rules! pid_self_list {
    ($field:expr) => {{
        let p = unsafe { addr_of_mut!($field) };
        list_head { next: p, prev: p }
    }};
}

// Bindgen's native FAM generic gives the original static its one trailing
// upid without replacing the flexible member by a pointer or zero-sized tail.
type InitialPid = pid<[upid; 1]>;
const _: () = {
    assert!(core::mem::align_of::<InitialPid>() == core::mem::align_of::<pid>());
    assert!(offset_of!(InitialPid, numbers) == LUPOS_PID_OFFSET_pid_numbers as usize);
    assert!(core::mem::size_of::<InitialPid>() == LUPOS_PID_INITIAL_SIZE as usize);
    assert!(
        core::mem::size_of::<InitialPid>()
            == core::mem::size_of::<pid>() + core::mem::size_of::<upid>()
    );
    assert!(core::mem::size_of::<lupos_pidmap_lock_storage>() == LUPOS_PID_MAP_LOCK_SIZE as usize);
    assert!(
        core::mem::align_of::<lupos_pidmap_lock_storage>() == LUPOS_PID_MAP_LOCK_ALIGN as usize
    );
    assert!(offset_of!(lupos_pidmap_lock_storage, lock) == LUPOS_PID_MAP_LOCK_OFFSET as usize);
    assert!(core::mem::size_of::<raw_spinlock_t>() == LUPOS_PID_RAW_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == LUPOS_PID_RAW_LOCK_ALIGN as usize);
    assert!(core::mem::size_of::<spinlock_t>() == LUPOS_PID_SPIN_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<spinlock_t>() == LUPOS_PID_SPIN_LOCK_ALIGN as usize);
};
#[cfg(not(CONFIG_PREEMPT_RT))]
const _: () = {
    assert!(LUPOS_PID_SPIN_RAW_OFFSET == 0);
    assert!(core::mem::size_of::<spinlock_t>() == core::mem::size_of::<raw_spinlock_t>());
    assert!(core::mem::align_of::<spinlock_t>() == core::mem::align_of::<raw_spinlock_t>());
};

#[no_mangle]
pub static mut init_struct_pid: InitialPid = {
    let mut p: InitialPid = unsafe { zeroed() };
    p.count.refs.counter = 1; // REFCOUNT_INIT(1).
                              // All task heads and the remaining original designated-initializer tail
                              // are zero, including PIDTYPE_SID (not explicitly listed in the C source).
    p.numbers[0].ns = addr_of_mut!(init_pid_ns);
    p
};

#[no_mangle]
pub static mut init_pid_ns: pid_namespace = {
    let mut ns: pid_namespace = unsafe { zeroed() };
    // NS_COMMON_INIT(init_pid_ns).
    ns.ns.__bindgen_anon_1.__ns_ref.refs.counter = 1;
    ns.ns.ns_type = LUPOS_PID_NS_TYPE;
    ns.ns.inum = LUPOS_PID_NS_INUM;
    #[cfg(CONFIG_PID_NS)]
    {
        ns.ns.ops = unsafe { addr_of!(pidns_operations) };
    }
    let mut tree: ns_tree = unsafe { zeroed() };
    tree.ns_id = LUPOS_PID_NS_ID;
    tree.__ns_ref_active.counter = 1;
    tree.ns_unified_node.ns_list_entry = pid_self_list!(
        init_pid_ns
            .ns
            .__bindgen_anon_2
            .tree
            .ns_unified_node
            .ns_list_entry
    );
    tree.ns_tree_node.ns_list_entry = pid_self_list!(
        init_pid_ns
            .ns
            .__bindgen_anon_2
            .tree
            .ns_tree_node
            .ns_list_entry
    );
    tree.ns_owner_node.ns_list_entry = pid_self_list!(
        init_pid_ns
            .ns
            .__bindgen_anon_2
            .tree
            .ns_owner_node
            .ns_list_entry
    );
    tree.ns_owner_root.ns_list_head = pid_self_list!(
        init_pid_ns
            .ns
            .__bindgen_anon_2
            .tree
            .ns_owner_root
            .ns_list_head
    );
    ns.ns.__bindgen_anon_2.tree = tree;
    // IDR_INIT(init_pid_ns.idr) -> XARRAY_INIT uses this exact name, without
    // inserting idr_rt into the stringified lockdep name.
    ns.idr.idr_rt.xa_lock = pid_spin_lock!("init_pid_ns.idr.xa_lock");
    ns.idr.idr_rt.xa_flags = LUPOS_PID_IDR_MARKER;
    ns.pid_allocated = PIDNS_ADDING;
    ns.child_reaper = unsafe { addr_of_mut!(init_task) };
    ns.user_ns = unsafe { addr_of_mut!(init_user_ns) };
    ns.pid_max = LUPOS_PID_MAX_DEFAULT;
    #[cfg(all(CONFIG_SYSCTL, CONFIG_MEMFD_CREATE))]
    {
        ns.memfd_noexec_scope = MEMFD_NOEXEC_SCOPE_EXEC as _;
    }
    ns
};

// Declaration-derived wrapper supplies the original SMP cacheline alignment.
// Only trailing private alignment padding differs from the scalar C lock's
// symbol extent; lock bytes, address alignment, section and lockdep name match.
#[no_mangle]
#[cfg_attr(
    all(CONFIG_SMP, not(CONFIG_X86_VSMP)),
    link_section = ".data..cacheline_aligned"
)]
#[cfg_attr(all(CONFIG_SMP, CONFIG_X86_VSMP), link_section = ".data..page_aligned")]
pub static mut lupos_pidmap_lock_state: lupos_pidmap_lock_storage = lupos_pidmap_lock_storage {
    lock: pid_spin_lock!("pidmap_lock"),
};
