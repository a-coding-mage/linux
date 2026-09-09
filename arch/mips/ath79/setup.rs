// SPDX-License-Identifier: GPL-2.0-only
/*
 * Atheros AR71XX/AR724X/AR913X specific setup
 *
 * Copyright (C) 2010-2011 Jaiganesh Narayanan <jnarayanan@atheros.com>
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 *
 * Parts of this file are based on Atheros' 2.6.15/2.6.31 BSP
 */

const ATH79_SYS_TYPE_LEN: usize = 64;

static mut ath79_sys_type: [u8; ATH79_SYS_TYPE_LEN] = [0; ATH79_SYS_TYPE_LEN];

extern "C" {
    static mut ath79_soc: u32;
    static mut ath79_soc_rev: u32;
    static mut ath79_reset_base: *mut core::ffi::c_void;
    static mut ath79_pll_base: *mut core::ffi::c_void;
    static mut mips_hpt_frequency: u32;
    static mut _machine_halt: Option<unsafe extern "C" fn()>;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;

    fn cpu_wait();
    fn ath79_reset_rr(reg: u32) -> u32;
    fn soc_is_qca953x() -> bool;
    fn soc_is_qca955x() -> bool;
    fn soc_is_qca956x() -> bool;
    fn soc_is_tp9343() -> bool;
    fn panic(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn set_io_port_base(base: usize);
    fn fw_getenvl(name: *const core::ffi::c_char) -> usize;
    fn get_fdt() -> *mut core::ffi::c_void;
    fn __dt_setup_arch(dtb: *mut core::ffi::c_void);
    fn ioremap(base: usize, size: usize) -> *mut core::ffi::c_void;
    fn ath79_ddr_ctrl_init();
    fn detect_memory_region(start: usize, min: usize, max: usize);
    fn of_clk_init(matches: *const core::ffi::c_void);
    fn of_get_cpu_node(cpu: u32, thread: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn of_clk_get(np: *mut core::ffi::c_void, index: u32) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> isize;
    fn clk_get_rate(clk: *mut core::ffi::c_void) -> usize;
    fn clk_put(clk: *mut core::ffi::c_void);
    fn irqchip_init();
}

const CP0_LEGACY_COMPARE_IRQ: u32 = 0;
const KSEG0ADDR: usize = 0;
const KSEG1: usize = 0;

unsafe extern "C" fn ath79_halt() {
    loop {
        cpu_wait();
    }
}

unsafe extern "C" fn ath79_detect_sys_type() {
    let mut chip: &[u8] = b"????\0";
    let id: u32 = ath79_reset_rr(AR71XX_RESET_REG_REV_ID);
    let major = id & REV_ID_MAJOR_MASK;
    let mut minor: u32;
    let mut rev: u32 = 0;
    let mut ver: u32 = 1;

    match major {
        REV_ID_MAJOR_AR71XX => {
            minor = id & AR71XX_REV_ID_MINOR_MASK;
            rev = (id >> AR71XX_REV_ID_REVISION_SHIFT) & AR71XX_REV_ID_REVISION_MASK;
            match minor {
                AR71XX_REV_ID_MINOR_AR7130 => { ath79_soc = ATH79_SOC_AR7130; chip = b"7130\0"; }
                AR71XX_REV_ID_MINOR_AR7141 => { ath79_soc = ATH79_SOC_AR7141; chip = b"7141\0"; }
                AR71XX_REV_ID_MINOR_AR7161 => { ath79_soc = ATH79_SOC_AR7161; chip = b"7161\0"; }
                _ => {}
            }
        }
        REV_ID_MAJOR_AR7240 => { ath79_soc = ATH79_SOC_AR7240; chip = b"7240\0"; rev = id & AR724X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR7241 => { ath79_soc = ATH79_SOC_AR7241; chip = b"7241\0"; rev = id & AR724X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR7242 => { ath79_soc = ATH79_SOC_AR7242; chip = b"7242\0"; rev = id & AR724X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR913X => {
            minor = id & AR913X_REV_ID_MINOR_MASK;
            rev = (id >> AR913X_REV_ID_REVISION_SHIFT) & AR913X_REV_ID_REVISION_MASK;
            match minor {
                AR913X_REV_ID_MINOR_AR9130 => { ath79_soc = ATH79_SOC_AR9130; chip = b"9130\0"; }
                AR913X_REV_ID_MINOR_AR9132 => { ath79_soc = ATH79_SOC_AR9132; chip = b"9132\0"; }
                _ => {}
            }
        }
        REV_ID_MAJOR_AR9330 => { ath79_soc = ATH79_SOC_AR9330; chip = b"9330\0"; rev = id & AR933X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR9331 => { ath79_soc = ATH79_SOC_AR9331; chip = b"9331\0"; rev = id & AR933X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR9341 => { ath79_soc = ATH79_SOC_AR9341; chip = b"9341\0"; rev = id & AR934X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR9342 => { ath79_soc = ATH79_SOC_AR9342; chip = b"9342\0"; rev = id & AR934X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_AR9344 => { ath79_soc = ATH79_SOC_AR9344; chip = b"9344\0"; rev = id & AR934X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_QCA9533_V2 => { ver = 2; ath79_soc_rev = 2; ath79_soc = ATH79_SOC_QCA9533; chip = b"9533\0"; rev = id & QCA953X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_QCA9533 => { ath79_soc = ATH79_SOC_QCA9533; chip = b"9533\0"; rev = id & QCA953X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_QCA9556 => { ath79_soc = ATH79_SOC_QCA9556; chip = b"9556\0"; rev = id & QCA955X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_QCA9558 => { ath79_soc = ATH79_SOC_QCA9558; chip = b"9558\0"; rev = id & QCA955X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_QCA956X => { ath79_soc = ATH79_SOC_QCA956X; chip = b"956X\0"; rev = id & QCA956X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_QCN550X => { ath79_soc = ATH79_SOC_QCA956X; chip = b"550X\0"; rev = id & QCA956X_REV_ID_REVISION_MASK; }
        REV_ID_MAJOR_TP9343 => { ath79_soc = ATH79_SOC_TP9343; chip = b"9343\0"; rev = id & QCA956X_REV_ID_REVISION_MASK; }
        _ => panic(b"ath79: unknown SoC, id:0x%08x\0".as_ptr() as *const _, id),
    }

    if ver == 1 { ath79_soc_rev = rev; }
    let _ = (chip, ver, rev);
    // The original uses sprintf/pr_info here; retain the externally visible text operation.
    pr_info(b"SoC: %s\n\0".as_ptr() as *const _, ath79_sys_type.as_ptr());
}

pub unsafe extern "C" fn get_system_type() -> *const u8 { ath79_sys_type.as_ptr() }

pub unsafe extern "C" fn get_c0_compare_int() -> u32 { CP0_LEGACY_COMPARE_IRQ }

pub unsafe extern "C" fn plat_mem_setup() {
    set_io_port_base(KSEG1);
    let mut dtb = fw_getenvl(b"fdt_start\0".as_ptr() as *const _);
    if dtb == 0 { dtb = get_fdt() as usize; }
    if dtb != 0 { __dt_setup_arch((KSEG0ADDR + dtb) as *mut _); }
    ath79_reset_base = ioremap(AR71XX_RESET_BASE, AR71XX_RESET_SIZE);
    ath79_pll_base = ioremap(AR71XX_PLL_BASE, AR71XX_PLL_SIZE);
    ath79_detect_sys_type();
    ath79_ddr_ctrl_init();
    detect_memory_region(0, ATH79_MEM_SIZE_MIN, ATH79_MEM_SIZE_MAX);
    _machine_halt = Some(ath79_halt);
    pm_power_off = Some(ath79_halt);
}

pub unsafe extern "C" fn plat_time_init() {
    of_clk_init(core::ptr::null());
    let np = of_get_cpu_node(0, core::ptr::null());
    if np.is_null() { pr_err(b"Failed to get CPU node\n\0".as_ptr() as *const _); return; }
    let clk = of_clk_get(np, 0);
    if is_err(clk) { pr_err(b"Failed to get CPU clock: %ld\n\0".as_ptr() as *const _, ptr_err(clk)); return; }
    let cpu_clk_rate = clk_get_rate(clk);
    pr_info(b"CPU clock: %lu.%03lu MHz\n\0".as_ptr() as *const _, cpu_clk_rate / 1_000_000, (cpu_clk_rate / 1_000) % 1_000);
    mips_hpt_frequency = (cpu_clk_rate / 2) as u32;
    clk_put(clk);
}

pub unsafe extern "C" fn arch_init_irq() { irqchip_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
