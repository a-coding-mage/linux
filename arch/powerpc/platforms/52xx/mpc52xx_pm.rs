// SPDX-License-Identifier: GPL-2.0
// External Linux/kernel headers and build-time configuration are supplied by
// the surrounding translation unit.

use core::ffi::c_void;

extern "C" {
    fn mpc52xx_deep_sleep(
        sram: *mut c_void,
        sdram_regs: *mut c_void,
        cdm: *mut mpc52xx_cdm,
        intr: *mut mpc52xx_intr,
    );
    static mpc52xx_ds_sram: u8;
    static mpc52xx_ds_sram_size: libc::c_long;
    static mpc52xx_ds_cached: u8;
    static mpc52xx_ds_cached_size: libc::c_long;
}

#[repr(C)]
pub struct mpc52xx_cdm {
    pub ccs_sleep_enable: u8,
    pub osc_sleep_enable: u8,
    pub ccs_qreq_test: u8,
    pub clk_enables: u32,
}

#[repr(C)]
pub struct mpc52xx_intr {
    pub main_mask: u32,
}

#[repr(C)]
pub struct mpc52xx_gpio_wkup {
    pub wkup_gpioe: u8,
    pub wkup_ddr: u8,
    pub wkup_inten: u8,
    pub wkup_itype: u16,
    pub wkup_maste: u8,
}

#[repr(C)]
pub struct mpc52xx_suspend {
    pub board_suspend_prepare: Option<unsafe extern "C" fn(*mut c_void)>,
    pub board_resume_finish: Option<unsafe extern "C" fn(*mut c_void)>,
}

type suspend_state_t = i32;

const PM_SUSPEND_STANDBY: suspend_state_t = 1;
const ENOSYS: i32 = 38;
const ENOMEM: i32 = 12;
const CONFIG_KERNEL_START: usize = 0;
const SPRN_DEC: u32 = 22;
const SPRN_HID0: u32 = 1008;
const MSR_POW: u32 = 0x0004;
const HID0_DOZE: u32 = 0x0000_0020;
const HID0_NAP: u32 = 0x0000_0040;
const HID0_DPM: u32 = 0x0000_0080;
const HID0_SLEEP: u32 = 0x0000_0010;

extern "C" {
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn of_find_matching_node(parent: *mut c_void, ids: *const c_void) -> *mut c_void;
    fn of_address_to_resource(np: *mut c_void, index: u32, res: *mut resource) -> i32;
    fn of_node_put(np: *mut c_void);
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn flush_icache_range(start: usize, stop: usize);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn in_8(addr: *const u8) -> u8;
    fn out_8(addr: *mut u8, value: u8);
    fn in_be16(addr: *const u16) -> u16;
    fn out_be16(addr: *mut u16, value: u16);
    fn in_be32(addr: *const u32) -> u32;
    fn out_be32(addr: *mut u32, value: u32);
    fn mfmsr() -> u32;
    fn mtmsr(value: u32);
    fn mfspr(spr: u32) -> u32;
    fn mtspr(spr: u32, value: u32);
}

#[repr(C)]
struct resource { start: usize }

#[repr(C)]
struct of_device_id { compatible: *const u8, type_: *const u8 }

#[repr(C)]
struct platform_suspend_ops {
    valid: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    prepare: Option<unsafe extern "C" fn() -> i32>,
    enter: Option<unsafe extern "C" fn(suspend_state_t) -> i32>,
    finish: Option<unsafe extern "C" fn()>,
}

static mut mbar: *mut c_void = core::ptr::null_mut();
static mut sdram: *mut c_void = core::ptr::null_mut();
static mut cdm: *mut mpc52xx_cdm = core::ptr::null_mut();
static mut intr: *mut mpc52xx_intr = core::ptr::null_mut();
static mut gpiow: *mut mpc52xx_gpio_wkup = core::ptr::null_mut();
static mut sram: *mut c_void = core::ptr::null_mut();
static mut sram_size: i32 = 0;
pub static mut mpc52xx_suspend: mpc52xx_suspend = mpc52xx_suspend { board_suspend_prepare: None, board_resume_finish: None };

pub unsafe extern "C" fn mpc52xx_pm_valid(state: suspend_state_t) -> i32 {
    if state == PM_SUSPEND_STANDBY { 1 } else { 0 }
}

pub unsafe extern "C" fn mpc52xx_set_wakeup_gpio(pin: u8, level: u8) -> i32 {
    out_8(&mut (*gpiow).wkup_gpioe, in_8(&(*gpiow).wkup_gpioe) | (1u8 << pin));
    out_8(&mut (*gpiow).wkup_ddr, in_8(&(*gpiow).wkup_ddr) & !(1u8 << pin));
    out_8(&mut (*gpiow).wkup_inten, in_8(&(*gpiow).wkup_inten) | (1u8 << pin));
    let mut tmp = in_be16(&(*gpiow).wkup_itype);
    tmp &= !(0x3u16 << (pin * 2));
    tmp |= ((!level as u16 + 1) << (pin * 2));
    out_be16(&mut (*gpiow).wkup_itype, tmp);
    out_8(&mut (*gpiow).wkup_maste, 1);
    0
}

pub unsafe extern "C" fn mpc52xx_pm_prepare() -> i32 {
    let mut res = resource { start: 0 };
    let np = of_find_matching_node(core::ptr::null_mut(), core::ptr::null());
    if of_address_to_resource(np, 0, &mut res) != 0 { of_node_put(np); return -ENOSYS; }
    mbar = ioremap(res.start, 0xc000);
    of_node_put(np);
    if mbar.is_null() { return -ENOSYS; }
    sdram = (mbar as usize + 0x100) as *mut c_void;
    cdm = (mbar as usize + 0x200) as *mut mpc52xx_cdm;
    intr = (mbar as usize + 0x500) as *mut mpc52xx_intr;
    gpiow = (mbar as usize + 0xc00) as *mut mpc52xx_gpio_wkup;
    sram = (mbar as usize + 0x8000) as *mut c_void;
    sram_size = 0x4000;
    if let Some(f) = mpc52xx_suspend.board_suspend_prepare { f(mbar); 0 }
    else { iounmap(mbar); -ENOSYS }
}

pub static mut saved_sram: [u8; 0x4000] = [0; 0x4000];

pub unsafe extern "C" fn mpc52xx_pm_enter(_state: suspend_state_t) -> i32 {
    let mut clk_enables: u32;
    let (msr, hid0): (u32, u32);
    let intr_main_mask: u32;
    let irq_0x500 = (CONFIG_KERNEL_START + 0x500) as *mut c_void;
    let irq_0x500_stop = irq_0x500 as usize + mpc52xx_ds_cached_size as usize;
    let mut saved_0x500 = [0u8; 0x100];
    if mpc52xx_ds_cached_size as usize > saved_0x500.len() { return -ENOMEM; }
    intr_main_mask = in_be32(&(*intr).main_mask);
    out_be32(&mut (*intr).main_mask, intr_main_mask | 0x1ffff);
    mtspr(SPRN_DEC, 0x7fffffff);
    memcpy(saved_sram.as_mut_ptr() as *mut c_void, sram, sram_size as usize);
    memcpy(sram, &mpc52xx_ds_sram as *const u8 as *const c_void, mpc52xx_ds_sram_size as usize);
    out_8(&mut (*cdm).ccs_sleep_enable, 1);
    out_8(&mut (*cdm).osc_sleep_enable, 1);
    out_8(&mut (*cdm).ccs_qreq_test, 1);
    clk_enables = in_be32(&(*cdm).clk_enables);
    out_be32(&mut (*cdm).clk_enables, clk_enables & 0x00088000);
    msr = mfmsr();
    mtmsr(msr & !MSR_POW);
    hid0 = mfspr(SPRN_HID0);
    mtspr(SPRN_HID0, (hid0 & !(HID0_DOZE | HID0_NAP | HID0_DPM)) | HID0_SLEEP);
    memcpy(saved_0x500.as_mut_ptr() as *mut c_void, irq_0x500, mpc52xx_ds_cached_size as usize);
    memcpy(irq_0x500, &mpc52xx_ds_cached as *const u8 as *const c_void, mpc52xx_ds_cached_size as usize);
    flush_icache_range(irq_0x500 as usize, irq_0x500_stop);
    mpc52xx_deep_sleep(sram, sdram, cdm, intr);
    memcpy(irq_0x500, saved_0x500.as_ptr() as *const c_void, mpc52xx_ds_cached_size as usize);
    flush_icache_range(irq_0x500 as usize, irq_0x500_stop);
    mtmsr(msr & !MSR_POW); mtspr(SPRN_HID0, hid0); mtmsr(msr);
    out_be32(&mut (*cdm).clk_enables, clk_enables);
    out_8(&mut (*cdm).ccs_sleep_enable, 0); out_8(&mut (*cdm).osc_sleep_enable, 0);
    memcpy(sram, saved_sram.as_ptr() as *const c_void, sram_size as usize);
    out_be32(&mut (*intr).main_mask, intr_main_mask);
    0
}

pub unsafe extern "C" fn mpc52xx_pm_finish() {
    if let Some(f) = mpc52xx_suspend.board_resume_finish { f(mbar); }
    iounmap(mbar);
}

static mpc52xx_pm_ops: platform_suspend_ops = platform_suspend_ops {
    valid: Some(mpc52xx_pm_valid), prepare: Some(mpc52xx_pm_prepare),
    enter: Some(mpc52xx_pm_enter), finish: Some(mpc52xx_pm_finish),
};

pub unsafe extern "C" fn mpc52xx_pm_init() -> i32 {
    suspend_set_ops(&mpc52xx_pm_ops);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
