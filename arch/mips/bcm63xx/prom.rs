/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    static mut bmips_cbr_addr: u32;
    static mut bmips_smp_enabled: i32;
    static mut bmips_booted_mask: core::ffi::c_void;
    static bmips_smp_movevec: u8;

    fn BMIPS_GET_CBR() -> u32;
    fn bcm63xx_cpu_init();
    fn bcm_wdt_writel(value: u32, reg: u32);
    fn bcm_perf_readl(reg: u32) -> u32;
    fn bcm_perf_writel(value: u32, reg: u32);
    fn board_prom_init();
    fn register_bmips_smp_ops() -> i32;
    fn bcm_readl(reg: u32) -> u32;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn __sync();
    fn set_c0_cause(value: u32);
    fn cpumask_set_cpu(cpu: u32, mask: *mut core::ffi::c_void);

    fn BCMCPU_IS_3368() -> bool;
    fn BCMCPU_IS_6328() -> bool;
    fn BCMCPU_IS_6338() -> bool;
    fn BCMCPU_IS_6345() -> bool;
    fn BCMCPU_IS_6348() -> bool;
    fn BCMCPU_IS_6358() -> bool;
    fn BCMCPU_IS_6362() -> bool;
    fn BCMCPU_IS_6368() -> bool;
}

pub unsafe fn prom_init() {
    let mut reg: u32;
    let mask: u32;

    /* Cache CBR addr before CPU/DMA setup */
    bmips_cbr_addr = BMIPS_GET_CBR();

    bcm63xx_cpu_init();

    /* stop any running watchdog */
    bcm_wdt_writel(WDT_STOP_1, WDT_CTL_REG);
    bcm_wdt_writel(WDT_STOP_2, WDT_CTL_REG);

    /* disable all hardware blocks clock for now */
    if BCMCPU_IS_3368() {
        mask = CKCTL_3368_ALL_SAFE_EN;
    } else if BCMCPU_IS_6328() {
        mask = CKCTL_6328_ALL_SAFE_EN;
    } else if BCMCPU_IS_6338() {
        mask = CKCTL_6338_ALL_SAFE_EN;
    } else if BCMCPU_IS_6345() {
        mask = CKCTL_6345_ALL_SAFE_EN;
    } else if BCMCPU_IS_6348() {
        mask = CKCTL_6348_ALL_SAFE_EN;
    } else if BCMCPU_IS_6358() {
        mask = CKCTL_6358_ALL_SAFE_EN;
    } else if BCMCPU_IS_6362() {
        mask = CKCTL_6362_ALL_SAFE_EN;
    } else if BCMCPU_IS_6368() {
        mask = CKCTL_6368_ALL_SAFE_EN;
    } else {
        mask = 0;
    }

    reg = bcm_perf_readl(PERF_CKCTL_REG);
    reg &= !mask;
    bcm_perf_writel(reg, PERF_CKCTL_REG);

    /* do low level board init */
    board_prom_init();

    /* set up SMP */
    if register_bmips_smp_ops() == 0 {
        /*
         * BCM6328 might not have its second CPU enabled, while BCM3368
         * and BCM6358 need special handling for their shared TLB, so
         * disable SMP for now.
         */
        if BCMCPU_IS_6328() {
            reg = bcm_readl(BCM_6328_OTP_BASE + OTP_USER_BITS_6328_REG(3));

            if reg & OTP_6328_REG3_TP1_DISABLED != 0 {
                bmips_smp_enabled = 0;
            }
        } else if BCMCPU_IS_3368() || BCMCPU_IS_6358() {
            bmips_smp_enabled = 0;
        }

        if bmips_smp_enabled == 0 {
            return;
        }

        /*
         * The bootloader has set up the CPU1 reset vector at
         * 0xa000_0200.
         * This conflicts with the special interrupt vector (IV).
         * The bootloader has also set up CPU1 to respond to the wrong
         * IPI interrupt.
         * Here we will start up CPU1 in the background and ask it to
         * reconfigure itself then go back to sleep.
         */
        memcpy(
            0xa0000200usize as *mut core::ffi::c_void,
            &bmips_smp_movevec as *const u8 as *const core::ffi::c_void,
            0x20,
        );
        __sync();
        set_c0_cause(C_SW0);
        cpumask_set_cpu(1, &mut bmips_booted_mask);

        /*
         * FIXME: we really should have some sort of hazard barrier here
         */
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
