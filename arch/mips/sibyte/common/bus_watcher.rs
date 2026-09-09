// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2002,2003 Broadcom Corporation
 */

/*
 * The Bus Watcher monitors internal bus transactions and maintains
 * counts of transactions with error status, logging details and
 * causing one of several interrupts.  This driver provides a handler
 * for those interrupts which aggregates the counts (to avoid
 * saturating the 8-bit counters) and provides a presence in
 * /proc/bus_watcher if PROC_FS is on.
 */

#[repr(C)]
pub struct BwStatsStruct {
    pub status: u64,
    pub l2_err: u32,
    pub memio_err: u32,
    pub status_printed: i32,
    pub l2_cor_d: libc::c_ulong,
    pub l2_bad_d: libc::c_ulong,
    pub l2_cor_t: libc::c_ulong,
    pub l2_bad_t: libc::c_ulong,
    pub mem_cor_d: libc::c_ulong,
    pub mem_bad_d: libc::c_ulong,
    pub bus_error: libc::c_ulong,
}

pub static mut bw_stats: BwStatsStruct = BwStatsStruct {
    status: 0,
    l2_err: 0,
    memio_err: 0,
    status_printed: 0,
    l2_cor_d: 0,
    l2_bad_d: 0,
    l2_cor_t: 0,
    l2_bad_t: 0,
    mem_cor_d: 0,
    mem_bad_d: 0,
    bus_error: 0,
};

unsafe fn print_summary(status: u32, l2_err: u32, memio_err: u32) {
    printk!("Bus watcher error counters: %08x %08x\n", l2_err, memio_err);
    printk!("\nLast recorded signature:\n");
    printk!("Request %02x from %d, answered by %d with Dcode %d\n",
        (G_SCD_BERR_TID(status) & 0x3f) as u32,
        (G_SCD_BERR_TID(status) >> 6) as i32,
        G_SCD_BERR_RID(status) as i32,
        G_SCD_BERR_DCODE(status) as i32);
}

pub unsafe fn check_bus_watcher() {
    let mut status: u32;
    #[cfg(any(CONFIG_SIBYTE_BCM112X, CONFIG_SIBYTE_SB1250))]
    { status = csr_in32(IOADDR(A_SCD_BUS_ERR_STATUS_DEBUG)); }
    #[cfg(CONFIG_SIBYTE_BCM1x80)]
    { status = csr_in32(IOADDR(A_BCM1480_BUS_ERR_STATUS_DEBUG)); }
    // Unknown Sibyte SOC configurations are rejected by the original preprocessor.
    #[cfg(not(any(CONFIG_SIBYTE_BCM112X, CONFIG_SIBYTE_SB1250, CONFIG_SIBYTE_BCM1x80)))]
    { panic!("bus watcher being built for unknown Sibyte SOC!"); }

    let (l2_err, memio_err);
    if (status & 0x7fffffff) == 0 {
        printk!("Using last values reaped by bus watcher driver\n");
        status = bw_stats.status as u32;
        l2_err = bw_stats.l2_err;
        memio_err = bw_stats.memio_err;
    } else {
        l2_err = csr_in32(IOADDR(A_BUS_L2_ERRORS));
        memio_err = csr_in32(IOADDR(A_BUS_MEM_IO_ERRORS));
    }
    if status & !(1u32 << 31) != 0 { print_summary(status, l2_err, memio_err); }
    else { printk!("Bus watcher indicates no error\n"); }
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn bw_proc_show(m: *mut seq_file, _v: *mut libc::c_void) -> i32 {
    let stats = (*m).private as *mut BwStatsStruct;
    seq_puts!(m, "SiByte Bus Watcher statistics\n");
    seq_puts!(m, "-----------------------------\n");
    seq_printf!(m, "L2-d-cor %8ld\nL2-d-bad %8ld\n", (*stats).l2_cor_d, (*stats).l2_bad_d);
    seq_printf!(m, "L2-t-cor %8ld\nL2-t-bad %8ld\n", (*stats).l2_cor_t, (*stats).l2_bad_t);
    seq_printf!(m, "MC-d-cor %8ld\nMC-d-bad %8ld\n", (*stats).mem_cor_d, (*stats).mem_bad_d);
    seq_printf!(m, "IO-err   %8ld\n", (*stats).bus_error);
    seq_puts!(m, "\nLast recorded signature:\n");
    seq_printf!(m, "Request %02x from %d, answered by %d with Dcode %d\n",
        (G_SCD_BERR_TID((*stats).status) & 0x3f) as u32,
        (G_SCD_BERR_TID((*stats).status) >> 6) as i32,
        G_SCD_BERR_RID((*stats).status) as i32,
        G_SCD_BERR_DCODE((*stats).status) as i32);
    if (*stats).status & M_SCD_BERR_MULTERRS != 0 { seq_puts!(m, "Multiple errors observed since last check.\n"); }
    if (*stats).status_printed != 0 { seq_puts!(m, "(no change since last printing)\n"); }
    else { (*stats).status_printed = 1; }
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe fn create_proc_decoder(stats: *mut BwStatsStruct) {
    let ent = proc_create_single_data!("bus_watcher", S_IWUSR | S_IRUGO, core::ptr::null_mut(), bw_proc_show, stats);
    if ent.is_null() { printk!(KERN_INFO "Unable to initialize bus_watcher /proc entry\n"); }
}

unsafe fn sibyte_bw_int(_irq: i32, data: *mut libc::c_void) -> irqreturn_t {
    let stats = data as *mut BwStatsStruct;
    let cntr: libc::c_ulong;
    #[cfg(CONFIG_SIBYTE_BW_TRACE)]
    {
        csr_out32(M_SCD_TRACE_CFG_FREEZE, IOADDR(A_SCD_TRACE_CFG));
        csr_out32(M_SCD_TRACE_CFG_START_READ, IOADDR(A_SCD_TRACE_CFG));
        for _i in 0..(256 * 6) { printk!("%016llx\n", __raw_readq(IOADDR(A_SCD_TRACE_READ))); }
        csr_out32(M_SCD_TRACE_CFG_RESET, IOADDR(A_SCD_TRACE_CFG));
        csr_out32(M_SCD_TRACE_CFG_START, IOADDR(A_SCD_TRACE_CFG));
    }
    (*stats).status = csr_in32(IOADDR(A_SCD_BUS_ERR_STATUS)) as u64;
    (*stats).status_printed = 0;
    (*stats).l2_err = csr_in32(IOADDR(A_BUS_L2_ERRORS));
    cntr = (*stats).l2_err as libc::c_ulong;
    (*stats).l2_cor_d += G_SCD_L2ECC_CORR_D(cntr);
    (*stats).l2_bad_d += G_SCD_L2ECC_BAD_D(cntr);
    (*stats).l2_cor_t += G_SCD_L2ECC_CORR_T(cntr);
    (*stats).l2_bad_t += G_SCD_L2ECC_BAD_T(cntr);
    csr_out32(0, IOADDR(A_BUS_L2_ERRORS));
    (*stats).memio_err = csr_in32(IOADDR(A_BUS_MEM_IO_ERRORS));
    cntr = (*stats).memio_err as libc::c_ulong;
    (*stats).mem_cor_d += G_SCD_MEM_ECC_CORR(cntr);
    (*stats).mem_bad_d += G_SCD_MEM_ECC_BAD(cntr);
    (*stats).bus_error += G_SCD_MEM_BUSERR(cntr);
    csr_out32(0, IOADDR(A_BUS_MEM_IO_ERRORS));
    IRQ_HANDLED
}

pub unsafe fn sibyte_bus_watcher() -> i32 {
    core::ptr::write_bytes(&mut bw_stats as *mut BwStatsStruct, 0, 1);
    bw_stats.status_printed = 1;
    if request_irq(K_INT_BAD_ECC, sibyte_bw_int, 0, cstr!("Bus watcher"), &mut bw_stats as *mut _ as *mut _) != 0 { printk!("Failed to register bus watcher BAD_ECC irq\n"); return -1; }
    if request_irq(K_INT_COR_ECC, sibyte_bw_int, 0, cstr!("Bus watcher"), &mut bw_stats as *mut _ as *mut _) != 0 { free_irq(K_INT_BAD_ECC, &mut bw_stats as *mut _ as *mut _); printk!("Failed to register bus watcher COR_ECC irq\n"); return -1; }
    if request_irq(K_INT_IO_BUS, sibyte_bw_int, 0, cstr!("Bus watcher"), &mut bw_stats as *mut _ as *mut _) != 0 { free_irq(K_INT_BAD_ECC, &mut bw_stats as *mut _ as *mut _); free_irq(K_INT_COR_ECC, &mut bw_stats as *mut _ as *mut _); printk!("Failed to register bus watcher IO_BUS irq\n"); return -1; }
    #[cfg(CONFIG_PROC_FS)] create_proc_decoder(&mut bw_stats);
    #[cfg(CONFIG_SIBYTE_BW_TRACE)]
    {
        csr_out32(M_SCD_TRSEQ_ASAMPLE | M_SCD_TRSEQ_DSAMPLE | K_SCD_TRSEQ_TRIGGER_ALL, IOADDR(A_SCD_TRACE_SEQUENCE_0));
        csr_out32(M_SCD_TRACE_CFG_RESET, IOADDR(A_SCD_TRACE_CFG));
        csr_out32(M_SCD_TRACE_CFG_START, IOADDR(A_SCD_TRACE_CFG));
    }
    0
}

device_initcall!(sibyte_bus_watcher);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
