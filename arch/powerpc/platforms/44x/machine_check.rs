// SPDX-License-Identifier: GPL-2.0-or-later
/*
 */

// C headers omitted; referenced kernel and architecture symbols are supplied
// by the surrounding translation unit.

pub unsafe fn machine_check_4xx(regs: *mut pt_regs) -> i32 {
    let reason: ::core::ffi::c_ulong = (*regs).esr;

    if reason & ESR_IMCP != 0 {
        printk(c"Instruction".as_ptr());
        mtspr(SPRN_ESR, reason & !ESR_IMCP);
    } else {
        printk(c"Data".as_ptr());
    }

    printk(c" machine check in kernel mode.\n".as_ptr());

    0
}

pub unsafe fn machine_check_440A(regs: *mut pt_regs) -> i32 {
    let reason: ::core::ffi::c_ulong = (*regs).esr;

    printk(c"Machine check in kernel mode.\n".as_ptr());
    if reason & ESR_IMCP != 0 {
        printk(c"Instruction Synchronous Machine Check exception\n".as_ptr());
        mtspr(SPRN_ESR, reason & !ESR_IMCP);
    } else {
        let mcsr: u32 = mfspr(SPRN_MCSR);
        if mcsr & MCSR_IB != 0 {
            printk(c"Instruction Read PLB Error\n".as_ptr());
        }
        if mcsr & MCSR_DRB != 0 {
            printk(c"Data Read PLB Error\n".as_ptr());
        }
        if mcsr & MCSR_DWB != 0 {
            printk(c"Data Write PLB Error\n".as_ptr());
        }
        if mcsr & MCSR_TLBP != 0 {
            printk(c"TLB Parity Error\n".as_ptr());
        }
        if mcsr & MCSR_ICP != 0 {
            flush_instruction_cache();
            printk(c"I-Cache Parity Error\n".as_ptr());
        }
        if mcsr & MCSR_DCSP != 0 {
            printk(c"D-Cache Search Parity Error\n".as_ptr());
        }
        if mcsr & MCSR_DCFP != 0 {
            printk(c"D-Cache Flush Parity Error\n".as_ptr());
        }
        if mcsr & MCSR_IMPE != 0 {
            printk(c"Machine Check exception is imprecise\n".as_ptr());
        }

        /* Clear MCSR */
        mtspr(SPRN_MCSR, mcsr);
    }
    0
}

// #ifdef CONFIG_PPC_47x
pub unsafe fn machine_check_47x(regs: *mut pt_regs) -> i32 {
    let reason: ::core::ffi::c_ulong = (*regs).esr;
    let mcsr: u32;

    printk(KERN_ERR.as_ptr());
    printk(c"Machine check in kernel mode.\n".as_ptr());
    if reason & ESR_IMCP != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"Instruction Synchronous Machine Check exception\n".as_ptr());
        mtspr(SPRN_ESR, reason & !ESR_IMCP);
        return 0;
    }
    mcsr = mfspr(SPRN_MCSR);
    if mcsr & MCSR_IB != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"Instruction Read PLB Error\n".as_ptr());
    }
    if mcsr & MCSR_DRB != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"Data Read PLB Error\n".as_ptr());
    }
    if mcsr & MCSR_DWB != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"Data Write PLB Error\n".as_ptr());
    }
    if mcsr & MCSR_TLBP != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"TLB Parity Error\n".as_ptr());
    }
    if mcsr & MCSR_ICP != 0 {
        flush_instruction_cache();
        printk(KERN_ERR.as_ptr());
        printk(c"I-Cache Parity Error\n".as_ptr());
    }
    if mcsr & MCSR_DCSP != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"D-Cache Search Parity Error\n".as_ptr());
    }
    if mcsr & PPC47x_MCSR_GPR != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"GPR Parity Error\n".as_ptr());
    }
    if mcsr & PPC47x_MCSR_FPR != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"FPR Parity Error\n".as_ptr());
    }
    if mcsr & PPC47x_MCSR_IPR != 0 {
        printk(KERN_ERR.as_ptr());
        printk(c"Machine Check exception is imprecise\n".as_ptr());
    }

    /* Clear MCSR */
    mtspr(SPRN_MCSR, mcsr);

    0
}
// #endif /* CONFIG_PPC_47x */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
