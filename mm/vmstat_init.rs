// SPDX-License-Identifier: GPL-2.0-only
#[cfg(CONFIG_PROC_FS)]
const VMSTAT_TABLE_COUNT: usize =
    if cfg!(CONFIG_SMP) { 2 } else { 0 } + if cfg!(CONFIG_NUMA) { 1 } else { 0 };
#[cfg(CONFIG_PROC_FS)]
#[repr(transparent)]
struct VmstatTable([ctl_table; VMSTAT_TABLE_COUNT]);
#[cfg(CONFIG_PROC_FS)]
// SAFETY: the descriptor array is immutable; writable pointees retain native synchronization.
unsafe impl Sync for VmstatTable {}
#[cfg(all(CONFIG_PROC_FS, any(CONFIG_SMP, CONFIG_NUMA)))]
static VMSTAT_TABLE: VmstatTable = VmstatTable({
    let mut table: [ctl_table; VMSTAT_TABLE_COUNT] = unsafe { zeroed() };
    #[cfg(CONFIG_SMP)]
    {
        table[0].procname = kernel::str::as_char_ptr_in_const_context(c"stat_interval");
        table[0].data = addr_of_mut!(SYSCTL_STAT_INTERVAL).cast();
        table[0].maxlen = size_of::<c_int>() as c_int;
        table[0].mode = 0o644;
        table[0].proc_handler = Some(proc_dointvec_jiffies);
        table[1].procname = kernel::str::as_char_ptr_in_const_context(c"stat_refresh");
        table[1].data = null_mut();
        table[1].maxlen = 0;
        table[1].mode = 0o600;
        table[1].proc_handler = Some(vmstat_refresh);
    }
    #[cfg(CONFIG_NUMA)]
    {
        let index = if cfg!(CONFIG_SMP) { 2 } else { 0 };
        table[index].procname = kernel::str::as_char_ptr_in_const_context(c"numa_stat");
        table[index].data = addr_of_mut!(SYSCTL_VM_NUMA_STAT).cast();
        table[index].maxlen = size_of::<c_int>() as c_int;
        table[index].mode = 0o644;
        table[index].proc_handler = Some(sysctl_vm_numa_stat_handler);
        table[index].extra1 = unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_VMSTAT_SYSCTL_ZERO_INDEX as usize)
                .cast_mut()
                .cast()
        };
        table[index].extra2 = unsafe {
            addr_of!(sysctl_vals)
                .cast::<c_int>()
                .wrapping_add(RUST_VMSTAT_SYSCTL_ONE_INDEX as usize)
                .cast_mut()
                .cast()
        };
    }
    table
});
#[cfg(all(CONFIG_PROC_FS, not(any(CONFIG_SMP, CONFIG_NUMA))))]
static VMSTAT_TABLE: VmstatTable = VmstatTable([]);
#[no_mangle]
#[link_section = ".init.text"]
#[cfg_attr(RUST_VMSTAT_INIT_COLD, cold)]
unsafe extern "C" fn init_mm_internals() {
    // SAFETY: this body preserves the native caller's pointer, locking and CPU-context contract.
    unsafe {
        MM_PERCPU_WQ = rust_vmstat_alloc_workqueue(
            kernel::str::as_char_ptr_in_const_context(c"mm_percpu_wq"),
            RUST_VMSTAT_WQ_MEM_RECLAIM | RUST_VMSTAT_WQ_PERCPU,
            0,
        );
        #[cfg(CONFIG_SMP)]
        {
            let ret = rust_vmstat_cpuhp_setup(
                CPUHP_MM_VMSTAT_DEAD,
                kernel::str::as_char_ptr_in_const_context(c"mm/vmstat:dead"),
                None,
                Some(vmstat_cpu_dead),
            );
            if ret < 0 {
                rust_vmstat_log_dead_error();
            }
            let ret = rust_vmstat_cpuhp_setup(
                CPUHP_AP_ONLINE_DYN,
                kernel::str::as_char_ptr_in_const_context(c"mm/vmstat:online"),
                Some(vmstat_cpu_online),
                Some(vmstat_cpu_down_prep),
            );
            if ret < 0 {
                rust_vmstat_log_online_error();
            }
            rust_vmstat_cpus_read_lock();
            init_cpu_node_state();
            rust_vmstat_cpus_read_unlock();
            start_shepherd_timer();
        }
        #[cfg(CONFIG_PROC_FS)]
        {
            rust_vmstat_proc_create_seq(
                kernel::str::as_char_ptr_in_const_context(c"buddyinfo"),
                0o444,
                null_mut(),
                addr_of!(FRAGMENTATION_OP),
            );
            rust_vmstat_proc_create_seq(
                kernel::str::as_char_ptr_in_const_context(c"pagetypeinfo"),
                0o400,
                null_mut(),
                addr_of!(PAGETYPEINFO_OP),
            );
            rust_vmstat_proc_create_seq(
                kernel::str::as_char_ptr_in_const_context(c"vmstat"),
                0o444,
                null_mut(),
                addr_of!(VMSTAT_OP),
            );
            rust_vmstat_proc_create_seq(
                kernel::str::as_char_ptr_in_const_context(c"zoneinfo"),
                0o444,
                null_mut(),
                addr_of!(ZONEINFO_OP),
            );
            rust_vmstat_register_sysctl(
                kernel::str::as_char_ptr_in_const_context(c"vm"),
                addr_of!(VMSTAT_TABLE.0).cast(),
                kernel::str::as_char_ptr_in_const_context(c"vmstat_table"),
                VMSTAT_TABLE_COUNT,
            );
        }
    }
}
