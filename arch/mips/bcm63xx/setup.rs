/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 */

// Linux and architecture headers from the original source provide the
// declarations and constants referenced below.

use core::ffi::{c_char, c_void};

extern "C" {
    static mut mips_hpt_frequency: u32;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut _machine_restart: Option<unsafe extern "C" fn(*mut c_char)>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
    static mut ioport_resource: IoportResource;

    fn pr_info(fmt: *const c_char, ...);
    fn mdelay(ms: u32);
    fn bcm_perf_readl(reg: u32) -> u32;
    fn bcm_perf_writel(value: u32, reg: u32);
    fn bcm_wdt_writel(value: u32, reg: u32);
    fn bcm63xx_get_cpu_id() -> u32;
    fn bcm63xx_get_cpu_rev() -> u32;
    fn bcm63xx_get_cpu_freq() -> u32;
    fn bcm63xx_get_memory_size() -> usize;
    fn board_get_name() -> *const c_char;
    fn board_setup();
    fn board_register_devices() -> i32;
    fn bcm63xx_gpio_init();
    fn set_c0_status(value: u32);
    fn change_c0_config(mask: u32, value: u32);
    fn __flush_cache_all();
    fn write_c0_wired(value: u32);
    fn set_io_port_base(value: usize);
    fn memblock_add(base: usize, size: usize);
}

#[repr(C)]
pub struct IoportResource {
    pub start: usize,
    pub end: usize,
}

#[no_mangle]
pub static mut bmips_cbr_addr: *mut c_void = core::ptr::null_mut();

pub unsafe extern "C" fn bcm63xx_machine_halt() {
    pr_info(b"System halted\n\0".as_ptr() as *const c_char);
    loop {}
}

unsafe fn bcm6348_a1_reboot() {
    let mut reg: u32;

    pr_info(b"soft-resetting all blocks ...\n\0".as_ptr() as *const c_char);
    reg = bcm_perf_readl(PERF_SOFTRESET_REG);
    reg &= !SOFTRESET_6348_ALL;
    bcm_perf_writel(reg, PERF_SOFTRESET_REG);
    mdelay(10);

    reg = bcm_perf_readl(PERF_SOFTRESET_REG);
    reg |= SOFTRESET_6348_ALL;
    bcm_perf_writel(reg, PERF_SOFTRESET_REG);
    mdelay(10);

    pr_info(b"jumping to reset vector.\n\0".as_ptr() as *const c_char);
    set_c0_status(ST0_BEV | ST0_ERL);
    change_c0_config(CONF_CM_CMASK, CONF_CM_UNCACHED);
    __flush_cache_all();
    write_c0_wired(0);
    core::arch::asm!("jr $0xbfc00000", options(noreturn));
}

pub unsafe extern "C" fn bcm63xx_machine_reboot() {
    let mut reg: u32;
    let mut perf_regs = [0u32; 2];

    match bcm63xx_get_cpu_id() {
        BCM3368_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_3368,
        BCM6328_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_6328,
        BCM6338_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_6338,
        BCM6345_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_6345,
        BCM6348_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_6348,
        BCM6358_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_6358,
        BCM6362_CPU_ID => perf_regs[0] = PERF_EXTIRQ_CFG_REG_6362,
        _ => {}
    }

    for i in 0..2 {
        if perf_regs[i] == 0 { break; }
        reg = bcm_perf_readl(perf_regs[i]);
        if BCMCPU_IS_6348() {
            reg &= !EXTIRQ_CFG_MASK_ALL_6348;
            reg |= EXTIRQ_CFG_CLEAR_ALL_6348;
        } else {
            reg &= !EXTIRQ_CFG_MASK_ALL;
            reg |= EXTIRQ_CFG_CLEAR_ALL;
        }
        bcm_perf_writel(reg, perf_regs[i]);
    }

    if BCMCPU_IS_6348() && bcm63xx_get_cpu_rev() == 0xa1 {
        bcm6348_a1_reboot();
    }

    pr_info(b"triggering watchdog soft-reset...\n\0".as_ptr() as *const c_char);
    if BCMCPU_IS_6328() {
        bcm_wdt_writel(1, WDT_SOFTRESET_REG);
    } else {
        reg = bcm_perf_readl(PERF_SYS_PLL_CTL_REG);
        reg |= SYS_PLL_SOFT_RESET;
        bcm_perf_writel(reg, PERF_SYS_PLL_CTL_REG);
    }
    loop {}
}

unsafe extern "C" fn __bcm63xx_machine_reboot(_p: *mut c_char) {
    bcm63xx_machine_reboot();
}

pub unsafe extern "C" fn get_system_type() -> *const c_char {
    static mut BUF: [c_char; 128] = [0; 128];
    // Equivalent formatting is supplied by the platform's C-compatible helper.
    snprintf(
        BUF.as_mut_ptr(), 128, b"bcm63xx/%s (0x%04x/0x%02X)\0".as_ptr() as *const c_char,
        board_get_name(), bcm63xx_get_cpu_id(), bcm63xx_get_cpu_rev(),
    );
    BUF.as_ptr()
}

pub unsafe extern "C" fn plat_time_init() {
    mips_hpt_frequency = bcm63xx_get_cpu_freq() / 2;
}

pub unsafe extern "C" fn plat_mem_setup() {
    memblock_add(0, bcm63xx_get_memory_size());
    _machine_halt = Some(bcm63xx_machine_halt);
    _machine_restart = Some(__bcm63xx_machine_reboot);
    pm_power_off = Some(bcm63xx_machine_halt);
    set_io_port_base(0);
    ioport_resource.start = 0;
    ioport_resource.end = usize::MAX;
    board_setup();
}

unsafe extern "C" fn bcm63xx_register_devices() -> i32 {
    bcm63xx_gpio_init();
    board_register_devices()
}

extern "C" {
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ... ) -> i32;
}

// Constants below are supplied by the included BCM63xx headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
