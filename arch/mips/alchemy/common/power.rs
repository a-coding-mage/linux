/*
 * BRIEF MODULE DESCRIPTION
 *	Au1xx0 Power Management routines.
 *
 * Copyright 2001, 2008 MontaVista Software Inc.
 * Author: MontaVista Software, Inc. <source@mvista.com>
 *
 * Some of the routines are right out of init/main.c, whose copyrights apply
 * here. This file retains the licensing terms of the original source.
 */

// C dependencies: <linux/pm.h>, <linux/sysctl.h>, <linux/jiffies.h>,
// <linux/uaccess.h>, and <asm/mach-au1x00/au1000.h>.

extern "C" {
    fn alchemy_rdsys(reg: u32) -> u32;
    fn alchemy_rdsmem(reg: u32) -> u32;
    fn alchemy_wrsys(value: u32, reg: u32);
    fn alchemy_wrsmem(value: u32, reg: u32);
    fn au1xxx_cpu_has_pll_wo() -> bool;
    fn alchemy_get_cputype() -> i32;
    fn alchemy_sleep_au1000();
    fn alchemy_sleep_au1550();
    fn alchemy_sleep_au1300();
}

/*
 * We need to save/restore a bunch of core registers that are either volatile
 * or reset to some state across a processor sleep. If reading a register
 * doesn't provide a proper result for a later restore, we have to provide a
 * function for loading that register and save a copy.
 *
 * We only have to save/restore registers that aren't otherwise done as part
 * of a driver pm_* function.
 */
static mut SLEEP_SYS_CLOCKS: [u32; 5] = [0; 5];
static mut SLEEP_SYS_PINFUNC: u32 = 0;
static mut SLEEP_STATIC_MEMCTLR: [[u32; 3]; 4] = [[0; 3]; 4];

unsafe fn save_core_regs() {
    /* Clocks and PLLs. */
    SLEEP_SYS_CLOCKS[0] = alchemy_rdsys(AU1000_SYS_FREQCTRL0);
    SLEEP_SYS_CLOCKS[1] = alchemy_rdsys(AU1000_SYS_FREQCTRL1);
    SLEEP_SYS_CLOCKS[2] = alchemy_rdsys(AU1000_SYS_CLKSRC);
    SLEEP_SYS_CLOCKS[3] = alchemy_rdsys(AU1000_SYS_CPUPLL);
    SLEEP_SYS_CLOCKS[4] = alchemy_rdsys(AU1000_SYS_AUXPLL);

    /* pin mux config */
    SLEEP_SYS_PINFUNC = alchemy_rdsys(AU1000_SYS_PINFUNC);

    /* Save the static memory controller configuration. */
    SLEEP_STATIC_MEMCTLR[0][0] = alchemy_rdsmem(AU1000_MEM_STCFG0);
    SLEEP_STATIC_MEMCTLR[0][1] = alchemy_rdsmem(AU1000_MEM_STTIME0);
    SLEEP_STATIC_MEMCTLR[0][2] = alchemy_rdsmem(AU1000_MEM_STADDR0);
    SLEEP_STATIC_MEMCTLR[1][0] = alchemy_rdsmem(AU1000_MEM_STCFG1);
    SLEEP_STATIC_MEMCTLR[1][1] = alchemy_rdsmem(AU1000_MEM_STTIME1);
    SLEEP_STATIC_MEMCTLR[1][2] = alchemy_rdsmem(AU1000_MEM_STADDR1);
    SLEEP_STATIC_MEMCTLR[2][0] = alchemy_rdsmem(AU1000_MEM_STCFG2);
    SLEEP_STATIC_MEMCTLR[2][1] = alchemy_rdsmem(AU1000_MEM_STTIME2);
    SLEEP_STATIC_MEMCTLR[2][2] = alchemy_rdsmem(AU1000_MEM_STADDR2);
    SLEEP_STATIC_MEMCTLR[3][0] = alchemy_rdsmem(AU1000_MEM_STCFG3);
    SLEEP_STATIC_MEMCTLR[3][1] = alchemy_rdsmem(AU1000_MEM_STTIME3);
    SLEEP_STATIC_MEMCTLR[3][2] = alchemy_rdsmem(AU1000_MEM_STADDR3);
}

unsafe fn restore_core_regs() {
    /* Restore clock configuration; writing CPUPLL last stabilizes other clocks. */
    alchemy_wrsys(SLEEP_SYS_CLOCKS[0], AU1000_SYS_FREQCTRL0);
    alchemy_wrsys(SLEEP_SYS_CLOCKS[1], AU1000_SYS_FREQCTRL1);
    alchemy_wrsys(SLEEP_SYS_CLOCKS[2], AU1000_SYS_CLKSRC);
    alchemy_wrsys(SLEEP_SYS_CLOCKS[4], AU1000_SYS_AUXPLL);
    if !au1xxx_cpu_has_pll_wo() {
        alchemy_wrsys(SLEEP_SYS_CLOCKS[3], AU1000_SYS_CPUPLL);
    }

    alchemy_wrsys(SLEEP_SYS_PINFUNC, AU1000_SYS_PINFUNC);

    /* Restore the static memory controller configuration. */
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[0][0], AU1000_MEM_STCFG0);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[0][1], AU1000_MEM_STTIME0);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[0][2], AU1000_MEM_STADDR0);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[1][0], AU1000_MEM_STCFG1);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[1][1], AU1000_MEM_STTIME1);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[1][2], AU1000_MEM_STADDR1);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[2][0], AU1000_MEM_STCFG2);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[2][1], AU1000_MEM_STTIME2);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[2][2], AU1000_MEM_STADDR2);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[3][0], AU1000_MEM_STCFG3);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[3][1], AU1000_MEM_STTIME3);
    alchemy_wrsmem(SLEEP_STATIC_MEMCTLR[3][2], AU1000_MEM_STADDR3);
}

pub unsafe fn au_sleep() {
    save_core_regs();

    match alchemy_get_cputype() {
        ALCHEMY_CPU_AU1000 | ALCHEMY_CPU_AU1500 | ALCHEMY_CPU_AU1100 => {
            alchemy_sleep_au1000();
        }
        ALCHEMY_CPU_AU1550 | ALCHEMY_CPU_AU1200 => {
            alchemy_sleep_au1550();
        }
        ALCHEMY_CPU_AU1300 => {
            alchemy_sleep_au1300();
        }
        _ => {}
    }

    restore_core_regs();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
