// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68knommu/kernel/setup.c
 *
 *  Copyright (C) 1999-2007  Greg Ungerer (gerg@snapgear.com)
 *  Copyright (C) 1998,1999  D. Jeff Dionne <jeff@uClinux.org>
 *  Copyleft  ()) 2000       James D. Schettine {james@telos-systems.com}
 *  Copyright (C) 1998       Kenneth Albanowski <kjahds@kjahds.com>
 *  Copyright (C) 1995       Hamish Macdonald
 *  Copyright (C) 2000       Lineo Inc. (www.lineo.com)
 *  Copyright (C) 2001       Lineo, Inc. <www.lineo.com>
 *
 *  68VZ328 Fixes/support    Evan Stawnyczy <e@lineo.ca>
 */

/* This file handles the architecture-dependent parts of system setup. */

pub static mut memory_start: c_ulong = 0;
pub static mut memory_end: c_ulong = 0;

pub static mut command_line: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

/* machine dependent timer functions */
pub static mut mach_sched_init: Option<unsafe extern "C" fn()> = None;

/* machine dependent reboot functions */
pub static mut mach_reset: Option<unsafe extern "C" fn()> = None;
pub static mut mach_halt: Option<unsafe extern "C" fn()> = None;

#[cfg(CONFIG_M68328)]
const CPU_NAME: &str = "MC68328";
#[cfg(all(not(CONFIG_M68328), CONFIG_M68EZ328))]
const CPU_NAME: &str = "MC68EZ328";
#[cfg(all(not(CONFIG_M68328), not(CONFIG_M68EZ328), CONFIG_M68VZ328))]
const CPU_NAME: &str = "MC68VZ328";
#[cfg(all(CONFIG_M68000, not(CONFIG_M68328), not(CONFIG_M68EZ328), not(CONFIG_M68VZ328)))]
const CPU_NAME: &str = "MC68000";
#[cfg(not(any(CONFIG_M68328, CONFIG_M68EZ328, CONFIG_M68VZ328, CONFIG_M68000)))]
const CPU_NAME: &str = "UNKNOWN";

/* Different cores have different instruction execution timings. */
#[cfg(not(CPU_INSTR_PER_JIFFY))]
const CPU_INSTR_PER_JIFFY: c_ulong = 16;

pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut c_char) {
    memory_start = PAGE_ALIGN(_ramstart);
    memory_end = _ramend;

    setup_initial_init_mm(_stext, _etext, _edata, core::ptr::null_mut());
    config_BSP(command_line.as_mut_ptr(), command_line.len());

    #[cfg(CONFIG_BOOTPARAM)]
    strscpy(command_line.as_mut_ptr(), CONFIG_BOOTPARAM_STRING, command_line.len());

    process_uboot_commandline(command_line.as_mut_ptr(), command_line.len());
    pr_info!("uClinux with CPU {}\n", CPU_NAME);

    #[cfg(CONFIG_UCDIMM)]
    pr_info!("uCdimm by Lineo, Inc. <www.lineo.com>\n");
    #[cfg(CONFIG_M68328)]
    {
        pr_info!("68328 support D. Jeff Dionne <jeff@uclinux.org>\n");
        pr_info!("68328 support Kenneth Albanowski <kjshds.com>\n");
    }
    #[cfg(CONFIG_M68EZ328)]
    pr_info!("68EZ328 DragonBallEZ support (C) 1999 Rt-Control, Inc\n");
    #[cfg(CONFIG_M68VZ328)]
    {
        pr_info!("M68VZ328 support by Evan Stawnyczy <e@lineo.ca>\n");
        pr_info!("68VZ328 DragonBallVZ support (c) 2001 Lineo, Inc.\n");
    }
    #[cfg(CONFIG_COLDFIRE)]
    {
        pr_info!("COLDFIRE port done by Greg Ungerer, gerg@snapgear.com\n");
        #[cfg(CONFIG_M5307)]
        pr_info!("Modified for M5307 by Dave Miller, dmiller@intellistor.com\n");
        #[cfg(CONFIG_ELITE)]
        pr_info!("Modified for M5206eLITE by Rob Scott, rscott@mtrob.fdns.net\n");
    }
    pr_info!("Flat model support (C) 1998,1999 Kenneth Albanowski, D. Jeff Dionne\n");
    #[cfg(all(CONFIG_PILOT, CONFIG_M68328))]
    {
        pr_info!("68328/Pilot support Bernhard Kuhn <kuhn@lpr.e-technik.tu-muenchen.de>\n");
        pr_info!("TRG SuperPilot FLASH card support <info@trgnet.com>\n");
    }
    #[cfg(all(CONFIG_PILOT, CONFIG_M68EZ328))]
    pr_info!("PalmV support by Lineo Inc. <jeff@uclinux.com>\n");
    #[cfg(CONFIG_DRAGEN2)]
    pr_info!("DragonEngine II board support by Georges Menie\n");
    #[cfg(CONFIG_M5235EVB)]
    pr_info!("Motorola M5235EVB support (C)2005 Syn-tech Systems, Inc. (Jate Sujjavanich)\n");

    pr_debug!("KERNEL -> TEXT=0x{:p}-0x{:p} DATA=0x{:p}-0x{:p} BSS=0x{:p}-0x{:p}\n", _stext, _etext, _sdata, _edata, __bss_start, __bss_stop);
    pr_debug!("MEMORY -> ROMFS=0x{:p}-0x{:06x} MEM=0x{:06x}-0x{:06x}\n", __bss_stop, memory_start, memory_start, memory_end);
    memblock_add(_rambase, memory_end - _rambase);
    memblock_reserve(_rambase, memory_start - _rambase);
    *cmdline_p = command_line.as_mut_ptr();
    strscpy(boot_command_line, command_line.as_ptr(), COMMAND_LINE_SIZE);
    min_low_pfn = PFN_DOWN(memory_start);
    max_pfn = max_low_pfn = PFN_DOWN(memory_end);

    #[cfg(all(CONFIG_UBOOT, CONFIG_BLK_DEV_INITRD))]
    if initrd_start > 0 && initrd_start < initrd_end && initrd_end < memory_end {
        memblock_reserve(initrd_start, initrd_end - initrd_start);
    }
    paging_init();
}

/* Get CPU information for use by the procfs. */
unsafe extern "C" fn show_cpuinfo(m: *mut seq_file, _v: *mut c_void) -> c_int {
    let clockfreq = (loops_per_jiffy * HZ) * CPU_INSTR_PER_JIFFY;
    seq_printf(m, "CPU:\t\t%s\nMMU:\t\t%s\nFPU:\t\t%s\nClocking:\t%lu.%1luMHz\nBogoMips:\t%lu.%02lu\nCalibration:\t%lu loops\n", CPU_NAME, "none", "none", clockfreq / 1_000_000, (clockfreq / 100_000) % 10, (loops_per_jiffy * HZ) / 500_000, ((loops_per_jiffy * HZ) / 5_000) % 100, loops_per_jiffy * HZ);
    0
}

unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut c_void {
    if *pos < NR_CPUS { 0x12345678usize as *mut c_void } else { core::ptr::null_mut() }
}

unsafe extern "C" fn c_next(m: *mut seq_file, _v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    *pos += 1;
    c_start(m, pos)
}

unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut c_void) {}

pub static cpuinfo_op: seq_operations = seq_operations {
    start: Some(c_start),
    next: Some(c_next),
    stop: Some(c_stop),
    show: Some(show_cpuinfo),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
