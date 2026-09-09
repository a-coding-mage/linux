// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RNG driver for Freescale RNGC
 *
 * Copyright (C) 2008-2012 Freescale Semiconductor, Inc.
 * Copyright (C) 2017 Martin Kaiser <martin@kaiser.cx>
 */

// Linux kernel dependencies supplied by other translation units.

const RNGC_VER_ID: usize = 0x0000;
const RNGC_COMMAND: usize = 0x0004;
const RNGC_CONTROL: usize = 0x0008;
const RNGC_STATUS: usize = 0x000C;
const RNGC_ERROR: usize = 0x0010;
const RNGC_FIFO: usize = 0x0014;

const RNG_TYPE: u32 = 0xf000_0000;
const RNGC_VER_MAJ_SHIFT: u32 = 8;

const RNGC_TYPE_RNGB: u32 = 0x1;
const RNGC_TYPE_RNGC: u32 = 0x2;

const RNGC_CMD_CLR_ERR: u32 = 1 << 5;
const RNGC_CMD_CLR_INT: u32 = 1 << 4;
const RNGC_CMD_SEED: u32 = 1 << 1;
const RNGC_CMD_SELF_TEST: u32 = 1 << 0;

const RNGC_CTRL_MASK_ERROR: u32 = 1 << 6;
const RNGC_CTRL_MASK_DONE: u32 = 1 << 5;
const RNGC_CTRL_AUTO_SEED: u32 = 1 << 4;

const RNGC_STATUS_ERROR: u32 = 1 << 16;
const RNGC_STATUS_FIFO_LEVEL_MASK: u32 = 0x00000f00;
const RNGC_STATUS_SEED_DONE: u32 = 1 << 5;
const RNGC_STATUS_ST_DONE: u32 = 1 << 4;

const RNGC_ERROR_STATUS_STAT_ERR: u32 = 0x00000008;
const RNGC_SELFTEST_TIMEOUT: u64 = 2500;
const RNGC_SEED_TIMEOUT: u64 = 200;
const RNGC_PM_TIMEOUT: u64 = 500;

static mut self_test: bool = true;

#[repr(C)]
struct imx_rngc {
    dev: *mut device,
    clk: *mut clk,
    base: *mut core::ffi::c_void,
    rng: hwrng,
    rng_op_done: completion,
    err_reg: u32,
}

unsafe fn imx_rngc_irq_mask_clear(rngc: *mut imx_rngc) {
    let mut ctrl: u32 = readl((*rngc).base.add(RNGC_CONTROL));
    ctrl |= RNGC_CTRL_MASK_DONE | RNGC_CTRL_MASK_ERROR;
    writel(ctrl, (*rngc).base.add(RNGC_CONTROL));

    let mut cmd: u32 = readl((*rngc).base.add(RNGC_COMMAND));
    cmd |= RNGC_CMD_CLR_INT | RNGC_CMD_CLR_ERR;
    writel(cmd, (*rngc).base.add(RNGC_COMMAND));
}

unsafe fn imx_rngc_irq_unmask(rngc: *mut imx_rngc) {
    let mut ctrl: u32 = readl((*rngc).base.add(RNGC_CONTROL));
    ctrl &= !(RNGC_CTRL_MASK_DONE | RNGC_CTRL_MASK_ERROR);
    writel(ctrl, (*rngc).base.add(RNGC_CONTROL));
}

unsafe fn imx_rngc_self_test(rngc: *mut imx_rngc) -> i32 {
    imx_rngc_irq_unmask(rngc);
    let cmd = readl((*rngc).base.add(RNGC_COMMAND));
    writel(cmd | RNGC_CMD_SELF_TEST, (*rngc).base.add(RNGC_COMMAND));
    let ret = wait_for_completion_timeout(&mut (*rngc).rng_op_done, usecs_to_jiffies(RNGC_SELFTEST_TIMEOUT));
    imx_rngc_irq_mask_clear(rngc);
    if ret == 0 { return -ETIMEDOUT; }
    if (*rngc).err_reg != 0 { -EIO } else { 0 }
}

unsafe fn imx_rngc_read(rng: *mut hwrng, data: *mut core::ffi::c_void, mut max: usize, _wait: bool) -> i32 {
    let rngc = container_of!(rng, imx_rngc, rng);
    let err = pm_runtime_resume_and_get((*rngc).dev);
    if err != 0 { return err; }
    let mut data = data as *mut u8;
    let mut retval: i32 = 0;
    while max >= core::mem::size_of::<u32>() {
        let status = readl((*rngc).base.add(RNGC_STATUS));
        if status & RNGC_STATUS_ERROR != 0 { break; }
        if status & RNGC_STATUS_FIFO_LEVEL_MASK != 0 {
            (data as *mut u32).write(readl((*rngc).base.add(RNGC_FIFO)));
            retval += core::mem::size_of::<u32>() as i32;
            data = data.add(core::mem::size_of::<u32>());
            max -= core::mem::size_of::<u32>();
        }
    }
    pm_runtime_mark_last_busy((*rngc).dev);
    pm_runtime_put((*rngc).dev);
    if retval != 0 { retval } else { -EIO }
}

unsafe extern "C" fn imx_rngc_irq(_irq: i32, priv_: *mut core::ffi::c_void) -> irqreturn_t {
    let rngc = priv_ as *mut imx_rngc;
    let status = readl((*rngc).base.add(RNGC_STATUS));
    (*rngc).err_reg = readl((*rngc).base.add(RNGC_ERROR));
    imx_rngc_irq_mask_clear(rngc);
    if status & (RNGC_STATUS_SEED_DONE | RNGC_STATUS_ST_DONE) != 0 {
        complete(&mut (*rngc).rng_op_done);
    }
    IRQ_HANDLED
}

unsafe fn imx_rngc_init(rng: *mut hwrng) -> i32 {
    let rngc = container_of!(rng, imx_rngc, rng);
    let mut err = pm_runtime_resume_and_get((*rngc).dev);
    if err != 0 { return err; }
    let cmd = readl((*rngc).base.add(RNGC_COMMAND));
    writel(cmd | RNGC_CMD_CLR_ERR, (*rngc).base.add(RNGC_COMMAND));
    imx_rngc_irq_unmask(rngc);
    loop {
        let cmd = readl((*rngc).base.add(RNGC_COMMAND));
        writel(cmd | RNGC_CMD_SEED, (*rngc).base.add(RNGC_COMMAND));
        let ret = wait_for_completion_timeout(&mut (*rngc).rng_op_done, msecs_to_jiffies(RNGC_SEED_TIMEOUT));
        if ret == 0 { err = -ETIMEDOUT; break; }
        if (*rngc).err_reg != RNGC_ERROR_STATUS_STAT_ERR { break; }
    }
    if err == 0 && (*rngc).err_reg != 0 { err = -EIO; }
    if err == 0 {
        let mut ctrl = readl((*rngc).base.add(RNGC_CONTROL));
        ctrl |= RNGC_CTRL_AUTO_SEED;
        writel(ctrl, (*rngc).base.add(RNGC_CONTROL));
    } else { imx_rngc_irq_mask_clear(rngc); }
    pm_runtime_put((*rngc).dev);
    err
}

unsafe fn imx_rngc_cleanup(rng: *mut hwrng) {
    let rngc = container_of!(rng, imx_rngc, rng);
    let err = pm_runtime_resume_and_get((*rngc).dev);
    if err == 0 { imx_rngc_irq_mask_clear(rngc); pm_runtime_put((*rngc).dev); }
}

unsafe fn imx_rngc_probe(pdev: *mut platform_device) -> i32 {
    let rngc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<imx_rngc>(), GFP_KERNEL) as *mut imx_rngc;
    if rngc.is_null() { return -ENOMEM; }
    (*rngc).base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*rngc).base) { return ptr_err((*rngc).base); }
    (*rngc).clk = devm_clk_get(&mut (*pdev).dev, core::ptr::null());
    if is_err((*rngc).clk) { return dev_err_probe(&mut (*pdev).dev, ptr_err((*rngc).clk), "Cannot get rng_clk\n"); }
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    clk_prepare_enable((*rngc).clk);
    let ver_id = readl((*rngc).base.add(RNGC_VER_ID));
    let rng_type = ((ver_id & RNG_TYPE) >> 28) as u8;
    if rng_type as u32 != RNGC_TYPE_RNGC && rng_type as u32 != RNGC_TYPE_RNGB {
        clk_disable_unprepare((*rngc).clk); return -ENODEV;
    }
    init_completion(&mut (*rngc).rng_op_done);
    (*rngc).rng.name = (*pdev).name;
    (*rngc).rng.init = Some(imx_rngc_init);
    (*rngc).rng.read = Some(imx_rngc_read);
    (*rngc).rng.cleanup = Some(imx_rngc_cleanup);
    (*rngc).rng.quality = 19;
    (*rngc).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, rngc as *mut core::ffi::c_void);
    imx_rngc_irq_mask_clear(rngc);
    let ret = devm_request_irq(&mut (*pdev).dev, irq, Some(imx_rngc_irq), 0, (*pdev).name, rngc as *mut core::ffi::c_void);
    if ret != 0 { clk_disable_unprepare((*rngc).clk); return ret; }
    if self_test {
        let ret = imx_rngc_self_test(rngc);
        if ret != 0 { clk_disable_unprepare((*rngc).clk); return dev_err_probe(&mut (*pdev).dev, ret, "self test failed\n"); }
    }
    pm_runtime_set_autosuspend_delay(&mut (*pdev).dev, RNGC_PM_TIMEOUT);
    pm_runtime_use_autosuspend(&mut (*pdev).dev);
    pm_runtime_set_active(&mut (*pdev).dev);
    devm_pm_runtime_enable(&mut (*pdev).dev);
    let ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*rngc).rng);
    if ret != 0 { clk_disable_unprepare((*rngc).clk); return dev_err_probe(&mut (*pdev).dev, ret, "hwrng registration failed\n"); }
    dev_info(&mut (*pdev).dev, "Freescale RNG%c registered (HW revision %d.%02d)\n", if rng_type as u32 == RNGC_TYPE_RNGB { 'B' } else { 'C' }, (ver_id >> RNGC_VER_MAJ_SHIFT) & 0xff, ver_id & 0xff);
    0
}

unsafe fn imx_rngc_suspend(dev: *mut device) -> i32 {
    let rngc = dev_get_drvdata(dev) as *mut imx_rngc;
    clk_disable_unprepare((*rngc).clk); 0
}

unsafe fn imx_rngc_resume(dev: *mut device) -> i32 {
    let rngc = dev_get_drvdata(dev) as *mut imx_rngc;
    clk_prepare_enable((*rngc).clk); 0
}

// SYSTEM_SLEEP_PM_OPS, RUNTIME_PM_OPS, MODULE_DEVICE_TABLE, and module_platform_driver_probe
// are Linux build-time registrations represented by the corresponding external kernel macros.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
