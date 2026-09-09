// SPDX-License-Identifier: GPL-2.0
/*
 * LoongArch specific sysrq operations.
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the kernel and architecture headers.

static mut show_lock: spinlock_t = spinlock_t::new();

/*
 * Dump TLB entries on all CPUs.
 */

unsafe extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn smp_processor_id() -> c_int;
    fn dump_tlb_regs();
    fn dump_tlb_all();
    fn smp_call_function(
        func: unsafe extern "C" fn(*mut c_void),
        info: *mut c_void,
        wait: c_int,
    );
    fn schedule_work(work: *mut work_struct);
    fn register_sysrq_key(key: c_int, op: *mut sysrq_key_op) -> c_int;
}

unsafe extern "C" fn sysrq_tlbdump_single(_dummy: *mut c_void) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&raw mut show_lock, &raw mut flags);

    pr_info!("CPU{}:\n", smp_processor_id());
    dump_tlb_regs();
    pr_info!("\n");
    dump_tlb_all();
    pr_info!("\n");

    spin_unlock_irqrestore(&raw mut show_lock, flags);
}

// CONFIG_SMP
unsafe extern "C" fn sysrq_tlbdump_othercpus(_dummy: *mut work_struct) {
    smp_call_function(sysrq_tlbdump_single, core::ptr::null_mut(), 0);
}

// DECLARE_WORK(sysrq_tlbdump, sysrq_tlbdump_othercpus)
#[cfg(feature = "CONFIG_SMP")]
static mut sysrq_tlbdump: work_struct = work_struct::new(sysrq_tlbdump_othercpus);

unsafe extern "C" fn sysrq_handle_tlbdump(key: u8) {
    let _ = key;
    sysrq_tlbdump_single(core::ptr::null_mut());
    // CONFIG_SMP
    #[cfg(feature = "CONFIG_SMP")]
    schedule_work(&raw mut sysrq_tlbdump);
}

static mut sysrq_tlbdump_op: sysrq_key_op = sysrq_key_op {
    handler: Some(sysrq_handle_tlbdump),
    help_msg: "show-tlbs(x)\0".as_ptr() as *const c_char,
    action_msg: "Show TLB entries\0".as_ptr() as *const c_char,
    enable_mask: SYSRQ_ENABLE_DUMP,
};

unsafe extern "C" fn loongarch_sysrq_init() -> c_int {
    register_sysrq_key(b'x' as c_int, &raw mut sysrq_tlbdump_op)
}

// arch_initcall(loongarch_sysrq_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
