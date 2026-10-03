// SPDX-License-Identifier: GPL-2.0-only
// Complete Rust-owned data initializers from kernel/exit.c and the native macros
// they reach. Only canonical bindgen types and native-derived values are used.

macro_rules! exit_raw_spin_lock {
    ($name:expr) => {{
        let mut lock: raw_spinlock_t = unsafe { zeroed() };
        // Every current HAVE_RUST SMP architecture's native unlocked value is
        // zero (qspinlock/ticket/simple/ARM/s390). The generic UP debug variant
        // is distinct; its exact scalar comes from __ARCH_SPIN_LOCK_UNLOCKED.
        #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
        {
            lock.raw_lock.slock = RUST_EXIT_UP_UNLOCKED as _;
        }
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lock.magic = SPINLOCK_MAGIC as _;
            lock.owner_cpu = RUST_EXIT_SPIN_OWNER_CPU;
            lock.owner = RUST_EXIT_SPIN_OWNER as *mut kernel::ffi::c_void;
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
            lock.dep_map.wait_type_inner = RUST_EXIT_LD_WAIT_SPIN as _;
        }
        lock
    }};
}

macro_rules! exit_spin_lock {
    ($name:expr) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            let mut raw = exit_raw_spin_lock!($name);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                raw.dep_map.wait_type_inner = RUST_EXIT_LD_WAIT_CONFIG as _;
            }
            // Canonical assertions below cover the generated zero-sized UP
            // union as well as the ordinary raw-spinlock arm at offset zero.
            unsafe { addr_of_mut!(lock).cast::<raw_spinlock_t>().write(raw) };
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            // __RT_MUTEX_BASE_INITIALIZER: wait_lock, zero RB_ROOT_CACHED,
            // and owner=NULL. Keep the native macro's stringified lock path.
            lock.lock.wait_lock = exit_raw_spin_lock!(concat!($name, ".lock.wait_lock"));
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                lock.dep_map.name = concat!($name, "\0").as_ptr().cast();
                lock.dep_map.wait_type_inner = RUST_EXIT_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

static mut oops_limit: c_uint = 10000;
static mut oops_count: atomic_t = atomic_t { counter: 0 };
#[cfg(CONFIG_DEBUG_STACK_USAGE)]
static mut low_water_lock: spinlock_t = exit_spin_lock!("low_water_lock");
#[cfg(CONFIG_DEBUG_STACK_USAGE)]
static mut lowest_to_date: c_int = RUST_EXIT_THREAD_SIZE as c_int;

#[cfg(CONFIG_SYSCTL)]
#[repr(transparent)]
struct ExitSysctlTable([ctl_table; 1]);
#[cfg(CONFIG_SYSCTL)]
unsafe impl Sync for ExitSysctlTable {}
#[cfg(CONFIG_SYSCTL)]
static kern_exit_table: ExitSysctlTable = ExitSysctlTable([{
    let mut table: ctl_table = unsafe { zeroed() };
    table.procname = c"oops_limit".as_ptr().cast();
    table.data = addr_of_mut!(oops_limit).cast();
    table.maxlen = size_of::<c_uint>() as c_int;
    table.mode = 0o644;
    table.proc_handler = Some(b::proc_douintvec);
    table
}]);
#[cfg(CONFIG_SYSCTL)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_exit_kernel_exit_sysctls_init() -> c_int {
    b::__register_sysctl_init(c"kernel".as_ptr().cast(), kern_exit_table.0.as_ptr(), c"kern_exit_table".as_ptr().cast(), 1);
    0
}
#[cfg(CONFIG_SYSFS)]
static mut oops_count_attr: kobj_attribute = {
    let mut attr: kobj_attribute = unsafe { zeroed() };
    attr.attr.name = c"oops_count".as_ptr().cast();
    attr.attr.mode = 0o444;
    attr.__bindgen_anon_1.show = Some(b::rust_exit_oops_count_show_native);
    attr
};
#[cfg(CONFIG_SYSFS)]
#[no_mangle]
pub unsafe extern "C" fn rust_exit_oops_count_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, page: *mut c_char) -> isize {
    b::sysfs_emit(page, c"%d\n".as_ptr().cast(), b::rust_exit_atomic_read(addr_of!(oops_count))) as isize
}
#[cfg(CONFIG_SYSFS)]
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn rust_exit_kernel_exit_sysfs_init() -> c_int {
    b::sysfs_add_file_to_group(b::kernel_kobj, addr_of!(oops_count_attr.attr), core::ptr::null());
    0
}

const _: () = {
    assert!(size_of::<raw_spinlock_t>() == RUST_EXIT_RAW_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == RUST_EXIT_RAW_LOCK_ALIGN as usize);
    assert!(size_of::<spinlock_t>() == RUST_EXIT_SPIN_LOCK_SIZE as usize);
    assert!(core::mem::align_of::<spinlock_t>() == RUST_EXIT_SPIN_LOCK_ALIGN as usize);
    assert!(size_of::<pid_t>() == size_of::<c_uint>());
    assert!(size_of::<uid_t>() == size_of::<c_uint>());
};
#[cfg(not(CONFIG_PREEMPT_RT))]
const _: () = {
    assert!(RUST_EXIT_SPIN_RAW_OFFSET == 0);
    assert!(size_of::<spinlock_t>() == size_of::<raw_spinlock_t>());
    assert!(core::mem::align_of::<spinlock_t>() == core::mem::align_of::<raw_spinlock_t>());
};
