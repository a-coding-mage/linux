// SPDX-License-Identifier: GPL-2.0
// Rust-owned static state from irqdesc.c. Layouts/alignments and nonzero
// initializer scalars come from the original configured native headers.
macro_rules! irq_raw_spin_initializer {
    ($name:expr) => {{
        let mut lock: raw_spinlock_t = unsafe { zeroed() };
        // Native HAVE_RUST SMP lock representations use the all-zero unlocked
        // value; generic UP DEBUG_SPINLOCK has the header-derived scalar below.
        #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
        {
            lock.raw_lock.slock = LUPOS_IRQ_UP_UNLOCKED as _;
        }
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lock.magic = LUPOS_IRQ_SPIN_MAGIC;
            lock.owner_cpu = LUPOS_IRQ_SPIN_OWNER_CPU;
            lock.owner = LUPOS_IRQ_SPIN_OWNER as *mut c_void;
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = LUPOS_IRQ_LD_WAIT_SPIN as _;
        }
        lock
    }};
}
macro_rules! irq_spin_initializer {
    ($name:expr) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            let mut raw = irq_raw_spin_initializer!($name);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                raw.dep_map.wait_type_inner = LUPOS_IRQ_LD_WAIT_CONFIG as _;
            }
            unsafe {
                addr_of_mut!(lock).cast::<raw_spinlock_t>().write(raw);
            }
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.lock.wait_lock = irq_raw_spin_initializer!(concat!($name, ".lock.wait_lock"));
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
                lock.dep_map.wait_type_inner = LUPOS_IRQ_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

#[no_mangle]
pub static mut lupos_irq_sparse_mutex: mutex = {
    let mut lock: mutex = unsafe { zeroed() };
    #[cfg(not(CONFIG_PREEMPT_RT))]
    {
        lock.wait_lock = irq_raw_spin_initializer!("sparse_irq_lock.wait_lock");
        // owner=ATOMIC_LONG_INIT(0), first_waiter=NULL, and osq={0} are zero.
        #[cfg(CONFIG_DEBUG_MUTEXES)]
        {
            lock.magic = addr_of_mut!(lupos_irq_sparse_mutex).cast();
        }
    }
    #[cfg(CONFIG_PREEMPT_RT)]
    {
        lock.rtmutex.wait_lock = irq_raw_spin_initializer!("sparse_irq_lock.rtmutex.wait_lock");
        // RB_ROOT_CACHED and owner=NULL are all-zero, exactly as rtmutex.h.
    }
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    {
        lock.dep_map.name = b"sparse_irq_lock\0".as_ptr().cast();
        lock.dep_map.wait_type_inner = LUPOS_IRQ_LD_WAIT_SLEEP as _;
    }
    lock
};
#[no_mangle]
pub static mut lupos_irq_sparse_irqs: maple_tree = {
    let mut tree: maple_tree = unsafe { zeroed() };
    #[cfg(CONFIG_LOCKDEP)]
    {
        // Native MTREE_INIT_EXT initializes the union's external-map pointer.
        unsafe {
            addr_of_mut!(tree)
                .cast::<*mut lockdep_map>()
                .write(addr_of_mut!(lupos_irq_sparse_mutex.dep_map));
        }
    }
    #[cfg(not(CONFIG_LOCKDEP))]
    {
        // MTREE_INIT_EXT becomes MTREE_INIT in this configuration.
        unsafe {
            addr_of_mut!(tree)
                .cast::<spinlock_t>()
                .write(irq_spin_initializer!("(sparse_irqs).ma_lock"));
        }
    }
    tree.ma_flags = LUPOS_IRQ_MAPLE_FLAGS;
    tree
};

#[cfg(not(CONFIG_SPARSE_IRQ))]
#[export_name = "irq_desc"]
#[cfg_attr(
    all(CONFIG_SMP, not(CONFIG_X86_VSMP)),
    link_section = ".data..cacheline_aligned"
)]
#[cfg_attr(all(CONFIG_SMP, CONFIG_X86_VSMP), link_section = ".data..page_aligned")]
pub static mut lupos_irq_flat_desc: lupos_irq_flat_desc_storage = lupos_irq_flat_desc_storage {
    descs: [const {
        let mut desc: irq_desc = unsafe { zeroed() };
        desc.handle_irq = Some(handle_bad_irq);
        desc.depth = 1;
        desc.lock = irq_raw_spin_initializer!("irq_desc->lock");
        desc
    }; LUPOS_IRQ_NR_IRQS as usize],
};

const _: () = {
    assert!(size_of::<mutex>() == LUPOS_IRQ_MUTEX_SIZE as usize);
    assert!(core::mem::align_of::<mutex>() == LUPOS_IRQ_MUTEX_ALIGN as usize);
    assert!(size_of::<raw_spinlock_t>() == LUPOS_IRQ_RAW_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == LUPOS_IRQ_RAW_LOCK_ALIGN as usize);
    assert!(size_of::<spinlock_t>() == LUPOS_IRQ_SPIN_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<spinlock_t>() == LUPOS_IRQ_SPIN_LOCK_ALIGN as usize);
    assert!(size_of::<maple_tree>() == LUPOS_IRQ_MAPLE_SIZE as usize);
    assert!(core::mem::align_of::<maple_tree>() == LUPOS_IRQ_MAPLE_ALIGN as usize);
    assert!(LUPOS_IRQ_MAPLE_LOCK_OFFSET == 0);
};
#[cfg(not(CONFIG_PREEMPT_RT))]
const _: () = {
    assert!(LUPOS_IRQ_SPIN_RAW_OFFSET == 0);
    assert!(size_of::<spinlock_t>() == size_of::<raw_spinlock_t>());
    assert!(core::mem::align_of::<spinlock_t>() == core::mem::align_of::<raw_spinlock_t>());
};
#[cfg(CONFIG_LOCKDEP)]
const _: () = {
    assert!(LUPOS_IRQ_MAPLE_EXTERNAL_OFFSET == 0);
};
#[cfg(not(CONFIG_SPARSE_IRQ))]
const _: () = {
    assert!(size_of::<lupos_irq_flat_desc_storage>() == LUPOS_IRQ_FLAT_STORAGE_SIZE as usize);
    assert!(size_of::<lupos_irq_flat_desc_storage>() == LUPOS_IRQ_FLAT_ARRAY_SIZE as usize);
    assert!(core::mem::align_of::<lupos_irq_flat_desc_storage>() == LUPOS_IRQ_FLAT_ALIGN as usize);
    assert!(offset_of!(lupos_irq_flat_desc_storage, descs) == 0);
    assert!(
        size_of::<[irq_desc; LUPOS_IRQ_NR_IRQS as usize]>() == LUPOS_IRQ_FLAT_ARRAY_SIZE as usize
    );
};
