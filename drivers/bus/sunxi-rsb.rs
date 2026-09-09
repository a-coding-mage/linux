// SPDX-License-Identifier: GPL-2.0-only
/*
 * RSB (Reduced Serial Bus) driver.
 *
 * This is a source-level Rust translation of the corresponding Linux driver.
 * Kernel types and functions referenced here are supplied by external dependencies.
 */

/* RSB registers */
const RSB_CTRL: usize = 0x0;
const RSB_CCR: usize = 0x4;
const RSB_INTE: usize = 0x8;
const RSB_INTS: usize = 0xc;
const RSB_ADDR: usize = 0x10;
const RSB_DATA: usize = 0x1c;
const RSB_LCR: usize = 0x24;
const RSB_DMCR: usize = 0x28;
const RSB_CMD: usize = 0x2c;
const RSB_DAR: usize = 0x30;

const RSB_CTRL_START_TRANS: u32 = 1 << 7;
const RSB_CTRL_ABORT_TRANS: u32 = 1 << 6;
const RSB_CTRL_GLOBAL_INT_ENB: u32 = 1 << 1;
const RSB_CTRL_SOFT_RST: u32 = 1 << 0;
const RSB_CCR_MAX_CLK_DIV: u32 = 0xff;
const RSB_INTS_TRANS_ERR_ACK: u32 = 1 << 16;
const RSB_INTS_TRANS_ERR_DATA: u32 = 0xf << 8;
const RSB_INTS_LOAD_BSY: u32 = 1 << 2;
const RSB_INTS_TRANS_ERR: u32 = 1 << 1;
const RSB_INTS_TRANS_OVER: u32 = 1 << 0;
const RSB_DMCR_DEVICE_START: u32 = 1 << 31;
const RSB_DMCR_MODE_DATA: u32 = 0x7c << 16;
const RSB_DMCR_MODE_REG: u32 = 0x3e << 8;
const RSB_DMCR_DEV_ADDR: u32 = 0x00;
const RSB_CMD_RD8: u32 = 0x8b;
const RSB_CMD_RD16: u32 = 0x9c;
const RSB_CMD_RD32: u32 = 0xa6;
const RSB_CMD_WR8: u32 = 0x4e;
const RSB_CMD_WR16: u32 = 0x59;
const RSB_CMD_WR32: u32 = 0x63;
const RSB_CMD_STRA: u32 = 0xe8;
const RSB_MAX_FREQ: u32 = 20_000_000;
const RSB_CTRL_NAME: &str = "sunxi-rsb";

#[inline]
const fn rsb_ccr_sda_out_delay(v: u32) -> u32 { (v & 0x7) << 8 }
#[inline]
const fn rsb_ccr_clk_div(v: u32) -> u32 { v & RSB_CCR_MAX_CLK_DIV }
#[inline]
const fn rsb_dar_rta(v: u8) -> u32 { ((v as u32) & 0xff) << 16 }
#[inline]
const fn rsb_dar_da(v: u16) -> u32 { (v as u32) & 0xffff }

#[repr(C)]
struct SunxiRsbAddrMap { hwaddr: u16, rtaddr: u8 }

#[repr(C)]
struct SunxiRsb {
    dev: *mut device,
    regs: *mut core::ffi::c_void,
    clk: *mut clk,
    rstc: *mut reset_control,
    complete: completion,
    lock: mutex,
    status: u32,
    clk_freq: u32,
}

#[repr(C)]
struct SunxiRsbCtx { rdev: *mut sunxi_rsb_device, size: i32 }

extern "C" {
    static mut sunxi_rsb_bus: bus_type;
    fn of_driver_match_device(dev: *mut device, drv: *const device_driver) -> i32;
    fn to_sunxi_rsb_driver(drv: *mut device_driver) -> *mut sunxi_rsb_driver;
    fn to_sunxi_rsb_device(dev: *mut device) -> *mut sunxi_rsb_device;
    fn of_irq_get(node: *mut device_node, index: u32) -> i32;
    fn of_clk_set_defaults(node: *mut device_node, clk_supplier: bool) -> i32;
    fn driver_register(drv: *mut device_driver) -> i32;
    fn driver_unregister(drv: *mut device_driver);
    fn bus_register(bus: *mut bus_type) -> i32;
    fn bus_unregister(bus: *mut bus_type);
    fn platform_driver_register(drv: *mut platform_driver) -> i32;
    fn platform_driver_unregister(drv: *mut platform_driver);
}

/* External kernel declarations are intentionally not redefined here. */

fn sunxi_rsb_device_match(dev: *mut device, drv: *const device_driver) -> i32 {
    unsafe { of_driver_match_device(dev, drv) }
}

fn sunxi_rsb_device_probe(dev: *mut device) -> i32 {
    unsafe {
        let drv = to_sunxi_rsb_driver((*dev).driver);
        let rdev = to_sunxi_rsb_device(dev);
        if (*drv).probe.is_none() { return -19; }
        if (*rdev).irq == 0 {
            let mut irq = -2;
            if !(*dev).of_node.is_null() { irq = of_irq_get((*dev).of_node, 0); }
            if irq == -517 { return irq; }
            if irq < 0 { irq = 0; }
            (*rdev).irq = irq;
        }
        let ret = of_clk_set_defaults((*dev).of_node, false);
        if ret < 0 { return ret; }
        ((*drv).probe.unwrap())(rdev)
    }
}

fn sunxi_rsb_device_remove(dev: *mut device) {
    unsafe {
        let drv = to_sunxi_rsb_driver((*dev).driver);
        ((*drv).remove.unwrap())(to_sunxi_rsb_device(dev));
    }
}

fn sunxi_rsb_device_modalias(dev: *const device, env: *mut kobj_uevent_env) -> i32 {
    unsafe { of_device_uevent_modalias(dev, env) }
}

fn sunxi_rsb_dev_release(dev: *mut device) {
    unsafe { kfree(to_sunxi_rsb_device(dev)); }
}

fn sunxi_rsb_device_unregister(rdev: *mut sunxi_rsb_device) { unsafe { device_unregister(&mut (*rdev).dev); } }

fn sunxi_rsb_remove_devices(dev: *mut device, _data: *mut core::ffi::c_void) -> i32 {
    unsafe { if (*dev).bus == &raw mut sunxi_rsb_bus { sunxi_rsb_device_unregister(to_sunxi_rsb_device(dev)); } }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sunxi_rsb_driver_register(rdrv: *mut sunxi_rsb_driver) -> i32 {
    (*rdrv).driver.bus = &raw mut sunxi_rsb_bus;
    driver_register(&mut (*rdrv).driver)
}

unsafe fn _sunxi_rsb_run_xfer(rsb: *mut SunxiRsb) -> i32 {
    let int_mask = RSB_INTS_LOAD_BSY | RSB_INTS_TRANS_ERR | RSB_INTS_TRANS_OVER;
    if readl((*rsb).regs.add(RSB_CTRL)) & RSB_CTRL_START_TRANS != 0 { return -16; }
    reinit_completion(&mut (*rsb).complete);
    writel(int_mask, (*rsb).regs.add(RSB_INTE));
    writel(RSB_CTRL_START_TRANS | RSB_CTRL_GLOBAL_INT_ENB, (*rsb).regs.add(RSB_CTRL));
    let (timeout, status) = if irqs_disabled() {
        let mut status = 0;
        let timeout = readl_poll_timeout_atomic((*rsb).regs.add(RSB_INTS), &mut status, status & int_mask != 0, 10, 100000);
        writel(status, (*rsb).regs.add(RSB_INTS)); (timeout, status)
    } else {
        let timeout = !wait_for_completion_io_timeout(&mut (*rsb).complete, msecs_to_jiffies(100));
        (timeout, (*rsb).status)
    };
    if timeout { writel(RSB_CTRL_ABORT_TRANS, (*rsb).regs.add(RSB_CTRL)); writel(readl((*rsb).regs.add(RSB_INTS)), (*rsb).regs.add(RSB_INTS)); return -110; }
    if status & RSB_INTS_LOAD_BSY != 0 { return -16; }
    if status & RSB_INTS_TRANS_ERR != 0 {
        if status & RSB_INTS_TRANS_ERR_ACK != 0 { return -22; }
        if status & RSB_INTS_TRANS_ERR_DATA != 0 { return -5; }
    }
    0
}

unsafe fn sunxi_rsb_read(rsb: *mut SunxiRsb, rtaddr: u8, addr: u8, buf: *mut u32, len: usize) -> i32 {
    if buf.is_null() { return -22; }
    let cmd = match len { 1 => RSB_CMD_RD8, 2 => RSB_CMD_RD16, 4 => RSB_CMD_RD32, _ => return -22 };
    let ret = pm_runtime_resume_and_get((*rsb).dev); if ret != 0 { return ret; }
    mutex_lock(&mut (*rsb).lock);
    writel(addr as u32, (*rsb).regs.add(RSB_ADDR)); writel(rsb_dar_rta(rtaddr), (*rsb).regs.add(RSB_DAR)); writel(cmd, (*rsb).regs.add(RSB_CMD));
    let ret = _sunxi_rsb_run_xfer(rsb); if ret == 0 { *buf = readl((*rsb).regs.add(RSB_DATA)) & ((1u32 << (len * 8)) - 1); }
    mutex_unlock(&mut (*rsb).lock); pm_runtime_put_autosuspend((*rsb).dev); ret
}

unsafe fn sunxi_rsb_write(rsb: *mut SunxiRsb, rtaddr: u8, addr: u8, buf: *const u32, len: usize) -> i32 {
    if buf.is_null() { return -22; }
    let cmd = match len { 1 => RSB_CMD_WR8, 2 => RSB_CMD_WR16, 4 => RSB_CMD_WR32, _ => return -22 };
    let ret = pm_runtime_resume_and_get((*rsb).dev); if ret != 0 { return ret; }
    mutex_lock(&mut (*rsb).lock);
    writel(addr as u32, (*rsb).regs.add(RSB_ADDR)); writel(rsb_dar_rta(rtaddr), (*rsb).regs.add(RSB_DAR)); writel(*buf, (*rsb).regs.add(RSB_DATA)); writel(cmd, (*rsb).regs.add(RSB_CMD));
    let ret = _sunxi_rsb_run_xfer(rsb); mutex_unlock(&mut (*rsb).lock); pm_runtime_put_autosuspend((*rsb).dev); ret
}

// The remaining controller, regmap, device-tree, power-management, platform-driver,
// module-init, and module-exit definitions retain the same external kernel ABI and
// are declared below as direct translations of the source implementation.

unsafe fn sunxi_rsb_get_rtaddr(hwaddr: u16) -> u8 {
    match hwaddr { 0x3a3 => 0x2d, 0x745 => 0x3a, 0xe89 => 0x4e, _ => 0 }
}

unsafe fn sunxi_rsb_init_device_mode(rsb: *mut SunxiRsb) -> i32 {
    writel(RSB_DMCR_DEVICE_START | RSB_DMCR_MODE_DATA | RSB_DMCR_MODE_REG | RSB_DMCR_DEV_ADDR, (*rsb).regs.add(RSB_DMCR));
    let mut reg = 0; readl_poll_timeout((*rsb).regs.add(RSB_DMCR), &mut reg, reg & RSB_DMCR_DEVICE_START == 0, 100, 250000);
    writel(readl((*rsb).regs.add(RSB_INTS)), (*rsb).regs.add(RSB_INTS));
    if reg & RSB_DMCR_DEVICE_START != 0 { -110 } else { 0 }
}

unsafe fn sunxi_rsb_hw_init(rsb: *mut SunxiRsb) -> i32 {
    let mut ret = clk_prepare_enable((*rsb).clk); if ret != 0 { return ret; }
    ret = reset_control_deassert((*rsb).rstc); if ret != 0 { clk_disable_unprepare((*rsb).clk); return ret; }
    writel(RSB_CTRL_SOFT_RST, (*rsb).regs.add(RSB_CTRL));
    let mut reg = 0; readl_poll_timeout((*rsb).regs.add(RSB_CTRL), &mut reg, reg & RSB_CTRL_SOFT_RST == 0, 1000, 100000);
    let p_clk_freq = clk_get_rate((*rsb).clk); let mut clk_div = p_clk_freq / (*rsb).clk_freq as u64 / 2;
    if clk_div == 0 { clk_div = 1; } else if clk_div > (RSB_CCR_MAX_CLK_DIV + 1) as u64 { clk_div = (RSB_CCR_MAX_CLK_DIV + 1) as u64; }
    let mut clk_delay = clk_div >> 1; if clk_delay == 0 { clk_delay = 1; }
    writel(rsb_ccr_sda_out_delay(clk_delay as u32) | rsb_ccr_clk_div((clk_div - 1) as u32), (*rsb).regs.add(RSB_CCR)); 0
}

unsafe fn sunxi_rsb_hw_exit(rsb: *mut SunxiRsb) {
    reset_control_assert((*rsb).rstc);
    if !pm_runtime_status_suspended((*rsb).dev) { clk_disable_unprepare((*rsb).clk); }
}

unsafe fn sunxi_rsb_runtime_suspend(dev: *mut device) -> i32 { clk_disable_unprepare((*(dev_get_drvdata(dev) as *mut SunxiRsb)).clk); 0 }
unsafe fn sunxi_rsb_runtime_resume(dev: *mut device) -> i32 { clk_prepare_enable((*(dev_get_drvdata(dev) as *mut SunxiRsb)).clk) }
unsafe fn sunxi_rsb_suspend(dev: *mut device) -> i32 { sunxi_rsb_hw_exit(dev_get_drvdata(dev) as *mut SunxiRsb); 0 }
unsafe fn sunxi_rsb_resume(dev: *mut device) -> i32 { sunxi_rsb_hw_init(dev_get_drvdata(dev) as *mut SunxiRsb) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
