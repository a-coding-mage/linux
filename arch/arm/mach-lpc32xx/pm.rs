// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-lpc32xx/pm.c
 *
 * Original authors: Vitaly Wool, Dmitry Chigirev <source@mvista.com>
 * Modified by Kevin Wells <kevin.wells@nxp.com>
 *
 * 2005 (c) MontaVista Software, Inc.
 */

/*
 * LPC32XX CPU and system power management
 *
 * The LPC32XX has three CPU modes for controlling system power: run,
 * direct-run, and halt modes. When switching between halt and run modes,
 * the CPU transistions through direct-run mode. For Linux, direct-run
 * mode is not used in normal operation. Halt mode is used when the
 * system is fully suspended.
 *
 * Run mode:
 * The ARM CPU clock (HCLK_PLL), HCLK bus clock, and PCLK bus clocks are
 * derived from the HCLK PLL. The HCLK and PCLK bus rates are divided from
 * the HCLK_PLL rate. Linux runs in this mode.
 *
 * Direct-run mode:
 * The ARM CPU clock, HCLK bus clock, and PCLK bus clocks are driven from
 * SYSCLK. SYSCLK is usually around 13MHz, but may vary based on SYSCLK
 * source or the frequency of the main oscillator. In this mode, the
 * HCLK_PLL can be safely enabled, changed, or disabled.
 *
 * Halt mode:
 * SYSCLK is gated off and the CPU and system clocks are halted.
 * Peripherals based on the 32KHz oscillator clock (ie, RTC, touch,
 * key scanner, etc.) still operate if enabled. In this state, an enabled
 * system event (ie, GPIO state change, RTC match, key press, etc.) will
 * wake the system up back into direct-run mode.
 *
 * DRAM refresh
 * DRAM clocking and refresh are slightly different for systems with DDR
 * DRAM or regular SDRAM devices. If SDRAM is used in the system, the
 * SDRAM will still be accessible in direct-run mode. In DDR based systems,
 * a transition to direct-run mode will stop all DDR accesses (no clocks).
 * Because of this, the code to switch power modes and the code to enter
 * and exit DRAM self-refresh modes must not be executed in DRAM. A small
 * section of IRAM is used instead for this.
 *
 * Suspend is handled with the following logic:
 *  Backup a small area of IRAM used for the suspend code
 *  Copy suspend code to IRAM
 *  Transfer control to code in IRAM
 *  Places DRAMs in self-refresh mode
 *  Enter direct-run mode
 *  Save state of HCLK_PLL PLL
 *  Disable HCLK_PLL PLL
 *  Enter halt mode - CPU and buses will stop
 *  System enters direct-run mode when an enabled event occurs
 *  HCLK PLL state is restored
 *  Run mode is entered
 *  DRAMS are placed back into normal mode
 *  Code execution returns from IRAM
 *  IRAM code are used for suspend is restored
 *  Suspend mode is exited
 */

// C headers and symbols supplied by the surrounding kernel translation.

type SuspendState = u32;

#[repr(C)]
struct PlatformSuspendOps {
    valid: Option<unsafe extern "C" fn() -> bool>,
    enter: Option<unsafe extern "C" fn(SuspendState) -> i32>,
}

extern "C" {
    static lpc32xx_sys_suspend: u8;
    static lpc32xx_sys_suspend_sz: usize;
    static EMC_BASE: usize;

    fn kmemdup(src: *const core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> *mut core::ffi::c_void;
    fn flush_icache_range(start: usize, end: usize);
    fn flush_cache_all();
    fn kfree(ptr: *mut core::ffi::c_void);
    fn __raw_readl(addr: *const u32) -> u32;
    fn __raw_writel(value: u32, addr: *mut u32);
    fn suspend_valid_only_mem() -> bool;
    fn suspend_set_ops(ops: *const PlatformSuspendOps);
}

const TEMP_IRAM_AREA: usize = 0; // IO_ADDRESS(LPC32XX_IRAM_BASE)
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

/*
 * Both STANDBY and MEM suspend states are handled the same with no
 * loss of CPU or memory state
 */
unsafe extern "C" fn lpc32xx_pm_enter(_state: SuspendState) -> i32 {
    let mut lpc32xx_suspend_ptr: Option<unsafe extern "C" fn() -> i32> = None;
    let iram_swap_area: *mut core::ffi::c_void;

    /* Allocate some space for temporary IRAM storage */
    iram_swap_area = kmemdup(
        TEMP_IRAM_AREA as *const core::ffi::c_void,
        lpc32xx_sys_suspend_sz,
        GFP_KERNEL,
    );
    if iram_swap_area.is_null() {
        return -ENOMEM;
    }

    /*
     * Copy code to suspend system into IRAM. The suspend code
     * needs to run from IRAM as DRAM may no longer be available
     * when the PLL is stopped.
     */
    memcpy(
        TEMP_IRAM_AREA as *mut core::ffi::c_void,
        (&lpc32xx_sys_suspend as *const u8).cast(),
        lpc32xx_sys_suspend_sz,
    );
    flush_icache_range(TEMP_IRAM_AREA, TEMP_IRAM_AREA + lpc32xx_sys_suspend_sz);

    /* Transfer to suspend code in IRAM */
    lpc32xx_suspend_ptr = Some(core::mem::transmute(TEMP_IRAM_AREA));
    flush_cache_all();
    (lpc32xx_suspend_ptr.unwrap())();

    /* Restore original IRAM contents */
    memcpy(
        TEMP_IRAM_AREA as *mut core::ffi::c_void,
        iram_swap_area,
        lpc32xx_sys_suspend_sz,
    );

    kfree(iram_swap_area);

    0
}

static LPC32XX_PM_OPS: PlatformSuspendOps = PlatformSuspendOps {
    valid: Some(suspend_valid_only_mem),
    enter: Some(lpc32xx_pm_enter),
};

const EMC_DYN_MEM_CTRL_OFS: usize = 0x20;
const EMC_SRMMC: u32 = 1 << 3;
const EMC_CTRL_REG: *mut u32 = (EMC_BASE + EMC_DYN_MEM_CTRL_OFS) as *mut u32;

pub unsafe extern "C" fn lpc32xx_pm_init() {
    /*
     * Setup SDRAM self-refresh clock to automatically disable o
     * start of self-refresh. This only needs to be done once.
     */
    __raw_writel(__raw_readl(EMC_CTRL_REG) | EMC_SRMMC, EMC_CTRL_REG);

    suspend_set_ops(&LPC32XX_PM_OPS);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
