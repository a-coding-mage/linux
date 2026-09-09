// SPDX-License-Identifier: GPL-2.0
/*
 * IDT Winchip specific Machine Check Exception Reporting
 * (C) Copyright 2002 Alan Cox <alan@lxorguk.ukuu.org.uk>
 */

/* Machine check handler for WinChip C6: */
pub unsafe fn winchip_machine_check(regs: *mut pt_regs) {
    let _ = regs;
    instrumentation_begin();
    pr_emerg!("CPU0: Machine Check Exception.\n");
    add_taint(TAINT_MACHINE_CHECK, LOCKDEP_NOW_UNRELIABLE);
    instrumentation_end();
}

/* Set up machine check reporting on the Winchip C6 series */
pub unsafe fn winchip_mcheck_init(c: *mut cpuinfo_x86) {
    let _ = c;
    let mut val: msr = core::mem::zeroed();

    rdmsrq(MSR_IDT_FCR1, &mut val.q);
    val.l |= 1u32 << 2; /* Enable EIERRINT (int 18 MCE) */
    val.l &= !(1u32 << 4); /* Enable MCE */
    wrmsrq(MSR_IDT_FCR1, val.q);

    cr4_set_bits(X86_CR4_MCE);

    pr_info!("Winchip machine check reporting enabled on CPU#0.\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
