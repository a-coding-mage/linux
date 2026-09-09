// SPDX-License-Identifier: GPL-2.0
/*
 * linux/arch/alpha/kernel/err_ev6.c
 *
 * Copyright (C) 2000 Jeff Wiedemeier (Compaq Computer Corporation)
 *
 * Error handling code supporting Alpha systems
 */

unsafe fn ev6_parse_ibox(i_stat: u64, print: i32) -> i32 {
    const EV6_I_STAT_PAR: u64 = 1u64 << 29;
    const EV6_I_STAT_ERRMASK: u64 = EV6_I_STAT_PAR;
    let status: i32 = MCHK_DISPOSITION_REPORT;

    if i_stat & EV6_I_STAT_ERRMASK == 0 {
        return MCHK_DISPOSITION_UNKNOWN_ERROR;
    }
    if print == 0 {
        return status;
    }
    if i_stat & EV6_I_STAT_PAR != 0 {
        printk!("%s    Icache parity error\n", err_print_prefix);
    }
    status
}

unsafe fn ev6_parse_mbox(mm_stat: u64, d_stat: u64, c_stat: u64, print: i32) -> i32 {
    const EV6_MM_STAT_DC_TAG_PERR: u64 = 1u64 << 10;
    const EV6_MM_STAT_ERRMASK: u64 = EV6_MM_STAT_DC_TAG_PERR;
    const EV6_D_STAT_TPERR_P0: u64 = 1u64 << 0;
    const EV6_D_STAT_TPERR_P1: u64 = 1u64 << 1;
    const EV6_D_STAT_ECC_ERR_ST: u64 = 1u64 << 2;
    const EV6_D_STAT_ECC_ERR_LD: u64 = 1u64 << 3;
    const EV6_D_STAT_SEO: u64 = 1u64 << 4;
    const EV6_D_STAT_ERRMASK: u64 = EV6_D_STAT_TPERR_P0 | EV6_D_STAT_TPERR_P1 |
        EV6_D_STAT_ECC_ERR_ST | EV6_D_STAT_ECC_ERR_LD | EV6_D_STAT_SEO;
    let status: i32 = MCHK_DISPOSITION_REPORT;

    if d_stat & EV6_D_STAT_ERRMASK == 0 && mm_stat & EV6_MM_STAT_ERRMASK == 0 {
        return MCHK_DISPOSITION_UNKNOWN_ERROR;
    }
    if print == 0 { return status; }
    if mm_stat & EV6_MM_STAT_DC_TAG_PERR != 0 { printk!("%s    Dcache tag parity error on probe\n", err_print_prefix); }
    if d_stat & EV6_D_STAT_TPERR_P0 != 0 { printk!("%s    Dcache tag parity error - pipe 0\n", err_print_prefix); }
    if d_stat & EV6_D_STAT_TPERR_P1 != 0 { printk!("%s    Dcache tag parity error - pipe 1\n", err_print_prefix); }
    if d_stat & EV6_D_STAT_ECC_ERR_ST != 0 { printk!("%s    ECC error occurred on a store\n", err_print_prefix); }
    if d_stat & EV6_D_STAT_ECC_ERR_LD != 0 {
        printk!("%s    ECC error occurred on a %s load\n", err_print_prefix,
            if c_stat != 0 { "" } else { "speculative " });
    }
    if d_stat & EV6_D_STAT_SEO != 0 { printk!("%s    Dcache second error\n", err_print_prefix); }
    status
}

unsafe fn ev6_parse_cbox(c_addr: u64, c1_syn: u64, c2_syn: u64, c_stat: u64, c_sts: u64, print: i32) -> i32 {
    const SOURCENAME: [&str; 8] = ["UNKNOWN", "UNKNOWN", "UNKNOWN", "MEMORY", "BCACHE", "DCACHE", "BCACHE PROBE", "BCACHE PROBE"];
    const STREAMNAME: [&str; 2] = ["D", "I"];
    const BITSNAME: [&str; 2] = ["SINGLE", "DOUBLE"];
    const BC_PERR: u64 = 0x01; const DC_PERR: u64 = 0x02;
    const PROBE_BC_ERR0: u64 = 0x06; const PROBE_BC_ERR1: u64 = 0x07;
    const SOURCE_MEMORY: u64 = 0x03; const SOURCE_BCACHE: u64 = 0x04;
    const ERRMASK: u64 = 0x1f; const SHARED: u64 = 1 << 0; const DIRTY: u64 = 1 << 1;
    const VALID: u64 = 1 << 2; const PARITY: u64 = 1 << 3;
    let status: i32 = MCHK_DISPOSITION_REPORT;
    if c_stat & ERRMASK == 0 { return MCHK_DISPOSITION_UNKNOWN_ERROR; }
    if print == 0 { return status; }
    let mut source: i32 = EXTRACT!(c_stat, EV6_C_STAT_SOURCE);
    let stream: i32 = EXTRACT!(c_stat, EV6_C_STAT_ISTREAM);
    let bits: i32 = EXTRACT!(c_stat, EV6_C_STAT_DOUBLE);
    if c_stat & BC_PERR != 0 { printk!("%s    Bcache tag parity error\n", err_print_prefix); source = -1; }
    if c_stat & DC_PERR != 0 { printk!("%s    Dcache tag parity error\n", err_print_prefix); source = -1; }
    if c_stat == PROBE_BC_ERR0 || c_stat == PROBE_BC_ERR1 { printk!("%s    Bcache single-bit error on a probe hit\n", err_print_prefix); source = -1; }
    if source != -1 { printk!("%s    %s-STREAM %s-BIT ECC error from %s\n", err_print_prefix, STREAMNAME[stream as usize], BITSNAME[bits as usize], SOURCENAME[source as usize]); }
    printk!("%s    Address: 0x%016llx\n    Syndrome[upper.lower]: %02llx.%02llx\n", err_print_prefix, c_addr, c2_syn, c1_syn);
    if source == SOURCE_MEMORY as i32 || source == SOURCE_BCACHE as i32 {
        printk!("%s    Block status: %s%s%s%s\n", err_print_prefix,
            if c_sts & SHARED != 0 { "SHARED " } else { "" }, if c_sts & DIRTY != 0 { "DIRTY " } else { "" },
            if c_sts & VALID != 0 { "VALID " } else { "" }, if c_sts & PARITY != 0 { "PARITY " } else { "" });
    }
    status
}

pub unsafe fn ev6_register_error_handlers() {
    /* None right now. */
}

pub unsafe fn ev6_process_logout_frame(mchk_header: *mut el_common, print: i32) -> i32 {
    let ev6mchk = mchk_header as *mut el_common_EV6_mcheck;
    let mut status = MCHK_DISPOSITION_UNKNOWN_ERROR;
    status |= ev6_parse_ibox((*ev6mchk).I_STAT, print);
    status |= ev6_parse_mbox((*ev6mchk).MM_STAT, (*ev6mchk).DC_STAT, (*ev6mchk).C_STAT, print);
    status |= ev6_parse_cbox((*ev6mchk).C_ADDR, (*ev6mchk).DC1_SYNDROME, (*ev6mchk).DC0_SYNDROME, (*ev6mchk).C_STAT, (*ev6mchk).C_STS, print);
    if print == 0 { return status; }
    if status != MCHK_DISPOSITION_DISMISS {
        let saved_err_prefix = err_print_prefix;
        printk!("%s    EXC_ADDR: 0x%016lx   IER_CM: 0x%016lx   ISUM: 0x%016lx\n    PAL_BASE: 0x%016lx   I_CTL:  0x%016lx   PCTX: 0x%016lx\n", err_print_prefix, (*ev6mchk).EXC_ADDR, (*ev6mchk).IER_CM, (*ev6mchk).ISUM, (*ev6mchk).PAL_BASE, (*ev6mchk).I_CTL, (*ev6mchk).PCTX);
        if status == MCHK_DISPOSITION_UNKNOWN_ERROR { printk!("%s    UNKNOWN error, frame follows:\n", err_print_prefix); } else { err_print_prefix = KERN_NOTICE; }
        mchk_dump_logout_frame(mchk_header);
        err_print_prefix = saved_err_prefix;
    }
    status
}

pub unsafe fn ev6_machine_check(vector: usize, la_ptr: usize) {
    let mchk_header = la_ptr as *mut el_common;
    mb(); draina();
    if ev6_process_logout_frame(mchk_header, 0) != MCHK_DISPOSITION_DISMISS {
        let saved_err_prefix = err_print_prefix; err_print_prefix = KERN_CRIT;
        printk!("%s*CPU %s Error (Vector 0x%x) reported on CPU %d:\n", err_print_prefix, if vector == SCB_Q_PROCERR { "Correctable" } else { "Uncorrectable" }, vector as u32, smp_processor_id() as i32);
        ev6_process_logout_frame(mchk_header, 1);
        dik_show_regs(get_irq_regs(), core::ptr::null_mut());
        err_print_prefix = saved_err_prefix;
    }
    wrmces(0x7); mb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
