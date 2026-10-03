// SPDX-License-Identifier: GPL-2.0-only
// All panic.c-owned state and policy tables live here, including the state
// originally produced by DEFINE_SPINLOCK / ATOMIC_NOTIFIER_HEAD / __ATTR_RO.

macro_rules! panic_raw_spin_lock {
    ($name:expr) => {{
        let mut lock: raw_spinlock_t = unsafe { zeroed() };
        #[cfg(all(not(CONFIG_SMP), CONFIG_DEBUG_SPINLOCK))]
        { lock.raw_lock.slock = LUPOS_PANIC_UP_UNLOCKED as _; }
        #[cfg(CONFIG_DEBUG_SPINLOCK)]
        {
            lock.magic = SPINLOCK_MAGIC as _;
            lock.owner_cpu = LUPOS_PANIC_SPIN_OWNER_CPU;
            lock.owner = LUPOS_PANIC_SPIN_OWNER as *mut c_void;
        }
        #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
        {
            lock.dep_map.name = cstr!($name);
            lock.dep_map.wait_type_inner = LUPOS_PANIC_LD_WAIT_SPIN as _;
        }
        lock
    }};
}

macro_rules! panic_spin_lock {
    ($name:expr) => {{
        let mut lock: spinlock_t = unsafe { zeroed() };
        #[cfg(not(CONFIG_PREEMPT_RT))]
        {
            let mut raw = panic_raw_spin_lock!($name);
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            { raw.dep_map.wait_type_inner = LUPOS_PANIC_LD_WAIT_CONFIG as _; }
            unsafe { addr_of_mut!(lock).cast::<raw_spinlock_t>().write(raw) };
        }
        #[cfg(CONFIG_PREEMPT_RT)]
        {
            lock.lock.wait_lock = panic_raw_spin_lock!(concat!($name, ".lock.wait_lock"));
            #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
            {
                lock.dep_map.name = cstr!($name);
                lock.dep_map.wait_type_inner = LUPOS_PANIC_LD_WAIT_CONFIG as _;
            }
        }
        lock
    }};
}

// The initial native x86_64 scope has a zero qspinlock unlocked value. UP debug
// uses the explicitly exposed native __ARCH_SPIN_LOCK_UNLOCKED instead.
const _: () = {
    assert!(size_of::<spinlock_t>() == LUPOS_PANIC_SPIN_SIZE as usize);
    assert!(core::mem::align_of::<spinlock_t>() == LUPOS_PANIC_SPIN_ALIGN as usize);
    assert!(size_of::<raw_spinlock_t>() == LUPOS_PANIC_RAW_SIZE as usize);
    assert!(core::mem::align_of::<raw_spinlock_t>() == LUPOS_PANIC_RAW_ALIGN as usize);
    assert!(size_of::<core::ffi::VaListImpl<'static>>() == LUPOS_PANIC_VA_SIZE as usize);
    assert!(core::mem::align_of::<core::ffi::VaListImpl<'static>>() == LUPOS_PANIC_VA_ALIGN as usize);
    assert!(size_of::<core::ffi::VaList<'static, 'static>>() == size_of::<*mut c_void>());
    assert!(size_of::<lupos_panic_csd_storage>() == LUPOS_PANIC_CSD_STORAGE_SIZE as usize);
    assert!(core::mem::align_of::<lupos_panic_csd_storage>() == LUPOS_PANIC_CSD_ALIGN as usize);
    assert!(core::mem::offset_of!(lupos_panic_csd_storage, data) == LUPOS_PANIC_CSD_DATA_OFFSET as usize);
    assert!(size_of::<call_single_data_t>() == LUPOS_PANIC_CSD_SIZE as usize);
};
#[cfg(not(CONFIG_PREEMPT_RT))]
const _: () = {
    assert!(LUPOS_PANIC_SPIN_RAW_OFFSET == 0);
    assert!(size_of::<spinlock_t>() == size_of::<raw_spinlock_t>());
    assert!(core::mem::align_of::<spinlock_t>() == core::mem::align_of::<raw_spinlock_t>());
};

#[cfg(CONFIG_SMP)]
#[link_section = ".data..read_mostly"]
static mut sysctl_oops_all_cpu_backtrace: c_uint = 0;
#[no_mangle]
pub static mut panic_on_oops: c_int = cfg!(CONFIG_PANIC_ON_OOPS) as c_int;
static mut tainted_mask: c_ulong = if cfg!(CONFIG_RANDSTRUCT) {
    (1 as c_ulong) << TAINT_RANDSTRUCT
} else { 0 };
#[export_name = "lupos_panic_pause_on_oops"]
static mut pause_on_oops: c_int = 0;
static mut pause_on_oops_flag: c_int = 0;
static mut pause_on_oops_lock: spinlock_t = panic_spin_lock!("pause_on_oops_lock");
#[no_mangle]
pub static mut crash_kexec_post_notifiers: bool = false;
#[no_mangle]
#[link_section = ".data..read_mostly"]
pub static mut panic_on_warn: c_int = 0;
#[no_mangle]
pub static mut panic_on_taint: c_ulong = 0;
#[no_mangle]
pub static mut panic_on_taint_nousertaint: bool = false;
#[link_section = ".data..read_mostly"]
static mut warn_limit: c_uint = 0;
#[export_name = "lupos_panic_console_replay"]
static mut panic_console_replay: bool = false;
#[no_mangle]
pub static mut panic_triggering_all_cpu_backtrace: bool = false;
static mut panic_this_cpu_backtrace_printed: bool = false;
#[no_mangle]
pub static mut panic_timeout: c_int = LUPOS_PANIC_TIMEOUT;
#[no_mangle]
pub static mut panic_print: c_ulong = 0;
static mut panic_force_cpu: c_int = -1;

#[no_mangle]
pub static mut panic_notifier_list: atomic_notifier_head = atomic_notifier_head {
    lock: panic_spin_lock!("panic_notifier_list.lock"), head: null_mut(),
};
static mut warn_count: atomic_t = atomic_t { counter: 0 };
#[no_mangle]
pub static mut panic_blink: Option<unsafe extern "C" fn(c_int) -> c_long> = None;
#[no_mangle]
pub static mut panic_cpu: atomic_t = atomic_t { counter: PANIC_CPU_INVALID };
#[no_mangle]
pub static mut panic_redirect_cpu: atomic_t = atomic_t { counter: PANIC_CPU_INVALID };
#[cfg(all(CONFIG_SMP, CONFIG_CRASH_DUMP))]
static mut panic_force_buf: *mut c_char = null_mut();

macro_rules! taint_flag_entry {
    ($yes:literal, $no:literal, $name:literal) => {
        taint_flag { c_true: $yes as c_char, c_false: $no as c_char, desc: cstr!($name) }
    };
}

#[no_mangle]
pub static taint_flags: ReadOnly<[taint_flag; TAINT_FLAGS_COUNT as usize]> = ReadOnly({
    // Designated indexing preserves the native TAINT_* values, independently
    // of declaration order. The array length is the native flag count.
    let mut f: [taint_flag; TAINT_FLAGS_COUNT as usize] = unsafe { zeroed() };
    f[TAINT_PROPRIETARY_MODULE as usize] = taint_flag_entry!(b'P', b'G', "PROPRIETARY_MODULE");
    f[TAINT_FORCED_MODULE as usize] = taint_flag_entry!(b'F', b' ', "FORCED_MODULE");
    f[TAINT_CPU_OUT_OF_SPEC as usize] = taint_flag_entry!(b'S', b' ', "CPU_OUT_OF_SPEC");
    f[TAINT_FORCED_RMMOD as usize] = taint_flag_entry!(b'R', b' ', "FORCED_RMMOD");
    f[TAINT_MACHINE_CHECK as usize] = taint_flag_entry!(b'M', b' ', "MACHINE_CHECK");
    f[TAINT_BAD_PAGE as usize] = taint_flag_entry!(b'B', b' ', "BAD_PAGE");
    f[TAINT_USER as usize] = taint_flag_entry!(b'U', b' ', "USER");
    f[TAINT_DIE as usize] = taint_flag_entry!(b'D', b' ', "DIE");
    f[TAINT_OVERRIDDEN_ACPI_TABLE as usize] = taint_flag_entry!(b'A', b' ', "OVERRIDDEN_ACPI_TABLE");
    f[TAINT_WARN as usize] = taint_flag_entry!(b'W', b' ', "WARN");
    f[TAINT_CRAP as usize] = taint_flag_entry!(b'C', b' ', "CRAP");
    f[TAINT_FIRMWARE_WORKAROUND as usize] = taint_flag_entry!(b'I', b' ', "FIRMWARE_WORKAROUND");
    f[TAINT_OOT_MODULE as usize] = taint_flag_entry!(b'O', b' ', "OOT_MODULE");
    f[TAINT_UNSIGNED_MODULE as usize] = taint_flag_entry!(b'E', b' ', "UNSIGNED_MODULE");
    f[TAINT_SOFTLOCKUP as usize] = taint_flag_entry!(b'L', b' ', "SOFTLOCKUP");
    f[TAINT_LIVEPATCH as usize] = taint_flag_entry!(b'K', b' ', "LIVEPATCH");
    f[TAINT_AUX as usize] = taint_flag_entry!(b'X', b' ', "AUX");
    f[TAINT_RANDSTRUCT as usize] = taint_flag_entry!(b'T', b' ', "RANDSTRUCT");
    f[TAINT_TEST as usize] = taint_flag_entry!(b'N', b' ', "TEST");
    f[TAINT_FWCTL as usize] = taint_flag_entry!(b'J', b' ', "FWCTL");
    f
});
const _: () = { assert!(size_of::<[taint_flag; TAINT_FLAGS_COUNT as usize]>()
    == size_of::<taint_flag>() * TAINT_FLAGS_COUNT as usize); };

#[link_section = ".init.data"]
static mut init_taint_buf: [c_char; INIT_TAINT_BUF_MAX] = [0; INIT_TAINT_BUF_MAX];
#[link_section = ".ref.data"]
static mut taint_buf: *mut c_char = addr_of_mut!(init_taint_buf).cast();
static mut taint_buf_size: usize = INIT_TAINT_BUF_MAX;

#[cfg(CONFIG_SYSFS)]
static mut warn_count_attr: kobj_attribute = {
    let mut a: kobj_attribute = unsafe { zeroed() };
    a.attr.name = cstr!("warn_count");
    a.attr.mode = 0o444;
    a.__bindgen_anon_1.show = Some(warn_count_show);
    a
};

macro_rules! panic_ctl {
    ($name:literal, $data:expr, $size:expr, $handler:expr) => {{
        let mut t: ctl_table = unsafe { zeroed() };
        t.procname = cstr!($name);
        t.data = $data;
        t.maxlen = $size as c_int;
        t.mode = 0o644;
        t.proc_handler = Some($handler);
        t
    }};
}

#[cfg(CONFIG_SYSCTL)]
// Bindgen represents the native incomplete extern sysctl_vals[] as [c_int; 0].
// wrapping_add preserves SYSCTL_ONE's symbol-plus-one-int relocation without
// requiring that declaration to describe the definition's complete array size.
static kern_panic_table: ReadOnly<[ctl_table; 7 + cfg!(CONFIG_SMP) as usize
    + cfg!(all(any(CONFIG_X86_32, CONFIG_PARISC), CONFIG_DEBUG_STACKOVERFLOW)) as usize]> = ReadOnly([
    #[cfg(CONFIG_SMP)]
    {
        let mut t = panic_ctl!("oops_all_cpu_backtrace", addr_of_mut!(sysctl_oops_all_cpu_backtrace).cast(), size_of::<c_int>(), proc_dointvec_minmax);
        t.extra1 = unsafe { addr_of!(sysctl_vals).cast::<c_int>().cast_mut().cast() };
        t.extra2 = unsafe { addr_of!(sysctl_vals).cast::<c_int>().wrapping_add(1).cast_mut().cast() };
        t
    },
    panic_ctl!("tainted", null_mut(), size_of::<c_long>(), proc_taint),
    panic_ctl!("panic", addr_of_mut!(panic_timeout).cast(), size_of::<c_int>(), proc_dointvec),
    panic_ctl!("panic_on_oops", addr_of_mut!(panic_on_oops).cast(), size_of::<c_int>(), proc_dointvec),
    panic_ctl!("panic_print", addr_of_mut!(panic_print).cast(), size_of::<c_ulong>(), sysctl_panic_print_handler),
    {
        let mut t = panic_ctl!("panic_on_warn", addr_of_mut!(panic_on_warn).cast(), size_of::<c_int>(), proc_dointvec_minmax);
        t.extra1 = unsafe { addr_of!(sysctl_vals).cast::<c_int>().cast_mut().cast() };
        t.extra2 = unsafe { addr_of!(sysctl_vals).cast::<c_int>().wrapping_add(1).cast_mut().cast() };
        t
    },
    panic_ctl!("warn_limit", addr_of_mut!(warn_limit).cast(), size_of::<c_uint>(), proc_douintvec),
    #[cfg(all(any(CONFIG_X86_32, CONFIG_PARISC), CONFIG_DEBUG_STACKOVERFLOW))]
    panic_ctl!("panic_on_stackoverflow", unsafe { addr_of_mut!(sysctl_panic_on_stackoverflow).cast() }, size_of::<c_int>(), proc_dointvec),
    panic_ctl!("panic_sys_info", addr_of_mut!(panic_print).cast(), size_of::<c_ulong>(), sysctl_sys_info_handler),
]);

#[cfg(CONFIG_BUG)]
static clear_warn_once_fops: ReadOnly<file_operations> = ReadOnly({
    let mut f: file_operations = unsafe { zeroed() };
    // This is a built-in object; THIS_MODULE is NULL, as in the original C TU.
    f.owner = null_mut();
    f.open = Some(clear_warn_once_open);
    f.release = Some(simple_attr_release);
    f.read = Some(lupos_panic_debugfs_attr_read);
    f.write = Some(lupos_panic_debugfs_attr_write);
    f
});

#[no_mangle]
pub static lupos_panic_print_ops: ReadOnly<kernel_param_ops> = ReadOnly({
    let mut ops: kernel_param_ops = unsafe { zeroed() };
    ops.set = Some(panic_print_set);
    ops.get = Some(panic_print_get);
    ops
});
