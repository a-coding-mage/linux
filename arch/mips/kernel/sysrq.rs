// SPDX-License-Identifier: GPL-2.0
/*
 * MIPS specific sysrq operations.
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 */

// Dependencies supplied by the kernel headers are referenced below.

/*
 * Dump TLB entries on all CPUs.
 */

static show_lock: SpinLock = DEFINE_SPINLOCK!();

unsafe fn sysrq_tlbdump_single(_dummy: *mut core::ffi::c_void) {
    let mut flags: c_ulong = 0;

    spin_lock_irqsave(&show_lock, &mut flags);

    pr_info!("CPU{}:\n", smp_processor_id());
    dump_tlb_regs();
    pr_info!("\n");
    dump_tlb_all();
    pr_info!("\n");

    spin_unlock_irqrestore(&show_lock, flags);
}

#[cfg(CONFIG_SMP)]
unsafe fn sysrq_tlbdump_othercpus(_dummy: *mut WorkStruct) {
    smp_call_function(
        sysrq_tlbdump_single,
        core::ptr::null_mut(),
        0,
    );
}

#[cfg(CONFIG_SMP)]
static mut sysrq_tlbdump: WorkStruct = DECLARE_WORK!(sysrq_tlbdump_othercpus);

unsafe fn sysrq_handle_tlbdump(_key: u8) {
    sysrq_tlbdump_single(core::ptr::null_mut());
    #[cfg(CONFIG_SMP)]
    schedule_work(&mut sysrq_tlbdump);
}

static sysrq_tlbdump_op: SysrqKeyOp = SysrqKeyOp {
    handler: Some(sysrq_handle_tlbdump),
    help_msg: "show-tlbs(x)",
    action_msg: "Show TLB entries",
    enable_mask: SYSRQ_ENABLE_DUMP,
};

unsafe fn mips_sysrq_init() -> c_int {
    register_sysrq_key(b'x', &sysrq_tlbdump_op)
}

arch_initcall!(mips_sysrq_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
