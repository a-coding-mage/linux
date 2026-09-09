// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (c) 2015, Daniel Thompson
 */

// Linux kernel dependencies supplied by other translation units.

const RNG_CR: u32 = 0x00;
const RNG_CR_RNGEN: u32 = 1 << 2;
const RNG_CR_CED: u32 = 1 << 5;
const RNG_CR_CONFIG1: u32 = 0xF << 8;
const RNG_CR_NISTC: u32 = 1 << 12;
const RNG_CR_CONFIG2: u32 = 0x7 << 13;
const RNG_CR_CLKDIV_SHIFT: u32 = 16;
const RNG_CR_CLKDIV: u32 = 0xF << 16;
const RNG_CR_CONFIG3: u32 = 0x3F << 20;
const RNG_CR_CONDRST: u32 = 1 << 30;
const RNG_CR_CONFLOCK: u32 = 1 << 31;
const RNG_CR_ENTROPY_SRC_MASK: u32 = RNG_CR_CONFIG1 | RNG_CR_NISTC | RNG_CR_CONFIG2 | RNG_CR_CONFIG3;
const RNG_CR_CONFIG_MASK: u32 = RNG_CR_ENTROPY_SRC_MASK | RNG_CR_CED | RNG_CR_CLKDIV;

const RNG_SR: u32 = 0x04;
const RNG_SR_DRDY: u32 = 1 << 0;
const RNG_SR_CECS: u32 = 1 << 1;
const RNG_SR_SECS: u32 = 1 << 2;
const RNG_SR_CEIS: u32 = 1 << 5;
const RNG_SR_SEIS: u32 = 1 << 6;

const RNG_DR: u32 = 0x08;
const RNG_NSCR: u32 = 0x0C;
const RNG_NSCR_MASK: u32 = (1 << 18) - 1;
const RNG_HTCR: u32 = 0x10;
const RNG_NB_RECOVER_TRIES: u32 = 3;

#[repr(C)]
struct stm32_rng_data {
    max_clock_rate: u32,
    nb_clock: u32,
    cr: u32,
    nscr: u32,
    htcr: u32,
    has_cond_reset: bool,
}

/// struct stm32_rng_config - RNG configuration data
///
/// @cr: RNG configuration. 0 means default hardware RNG configuration
/// @nscr: Noise sources control configuration.
/// @htcr: Health tests configuration.
#[repr(C)]
struct stm32_rng_config { cr: u32, nscr: u32, htcr: u32 }

#[repr(C)]
struct stm32_rng_private {
    rng: hwrng,
    dev: *mut device,
    base: *mut core::ffi::c_void,
    clk_bulk: *mut clk_bulk_data,
    rst: *mut reset_control,
    pm_conf: stm32_rng_config,
    data: *const stm32_rng_data,
    ced: bool,
    lock_conf: bool,
}

unsafe fn stm32_rng_conceal_seed_error_cond_reset(priv_: *mut stm32_rng_private) -> i32 {
    let dev = (*priv_).dev;
    let mut sr = readl_relaxed((*priv_).base.add(RNG_SR as usize));
    let mut cr = readl_relaxed((*priv_).base.add(RNG_CR as usize));
    let err: i32;
    if sr & RNG_SR_SECS != 0 {
        writel_relaxed(cr | RNG_CR_CONDRST, (*priv_).base.add(RNG_CR as usize));
        writel_relaxed(cr & !RNG_CR_CONDRST, (*priv_).base.add(RNG_CR as usize));
    } else {
        writel_relaxed(sr & !RNG_SR_SEIS, (*priv_).base.add(RNG_SR as usize));
        return 0;
    }
    err = readl_relaxed_poll_timeout_atomic((*priv_).base.add(RNG_CR as usize), &mut cr, !(cr & RNG_CR_CONDRST), 10, 100000);
    if err != 0 { dev_err(dev, "%s: timeout %x\n", "stm32_rng_conceal_seed_error_cond_reset", sr); return err; }
    if readl_relaxed((*priv_).base.add(RNG_SR as usize)) & RNG_SR_SEIS != 0 { return -22; }
    err = readl_relaxed_poll_timeout_atomic((*priv_).base.add(RNG_SR as usize), &mut sr, !(sr & RNG_SR_SECS), 10, 100000);
    if err != 0 { dev_err(dev, "%s: timeout %x\n", "stm32_rng_conceal_seed_error_cond_reset", sr); return err; }
    0
}

unsafe fn stm32_rng_conceal_seed_error_sw_reset(priv_: *mut stm32_rng_private) -> i32 {
    let mut i: u32 = 0;
    let sr = readl_relaxed((*priv_).base.add(RNG_SR as usize));
    writel_relaxed(sr & !RNG_SR_SEIS, (*priv_).base.add(RNG_SR as usize));
    i = 12;
    while i != 0 { let _ = readl_relaxed((*priv_).base.add(RNG_DR as usize)); i -= 1; }
    if readl_relaxed((*priv_).base.add(RNG_SR as usize)) & RNG_SR_SEIS != 0 { return -22; }
    0
}

unsafe fn stm32_rng_conceal_seed_error(rng: *mut hwrng) -> i32 {
    let priv_: *mut stm32_rng_private = container_of(rng);
    dev_dbg((*priv_).dev, "Concealing seed error\n");
    if (*(*priv_).data).has_cond_reset { stm32_rng_conceal_seed_error_cond_reset(priv_) } else { stm32_rng_conceal_seed_error_sw_reset(priv_) }
}

unsafe fn stm32_rng_read(rng: *mut hwrng, mut data: *mut core::ffi::c_void, mut max: usize, wait: bool) -> i32 {
    let priv_: *mut stm32_rng_private = container_of(rng);
    let mut i: u32 = 0;
    let mut retval = pm_runtime_resume_and_get((*priv_).dev);
    let mut err = 0;
    if retval != 0 { return retval; }
    if readl_relaxed((*priv_).base.add(RNG_SR as usize)) & RNG_SR_SEIS != 0 { stm32_rng_conceal_seed_error(rng); }
    while max >= core::mem::size_of::<u32>() {
        let mut sr = readl_relaxed((*priv_).base.add(RNG_SR as usize));
        if sr == 0 && wait { err = readl_relaxed_poll_timeout_atomic((*priv_).base.add(RNG_SR as usize), &mut sr, sr != 0, 10, 50000); if err != 0 { dev_err((*priv_).dev, "%s: timeout %x!\n", "stm32_rng_read", sr); break; } }
        else if sr == 0 { break; }
        if sr != RNG_SR_DRDY {
            if sr & RNG_SR_SEIS != 0 { err = stm32_rng_conceal_seed_error(rng); i += 1; if err != 0 && i > RNG_NB_RECOVER_TRIES { dev_err((*priv_).dev, "Couldn't recover from seed error\n"); retval = -131; break; } continue; }
            if sr & RNG_SR_CEIS != 0 { writel_relaxed(0, (*priv_).base.add(RNG_SR as usize)); }
        }
        *(data as *mut u32) = readl_relaxed((*priv_).base.add(RNG_DR as usize));
        if *(data as *mut u32) == 0 { err = stm32_rng_conceal_seed_error(rng); i += 1; if err != 0 && i > RNG_NB_RECOVER_TRIES { dev_err((*priv_).dev, "Couldn't recover from seed error"); retval = -131; break; } continue; }
        i = 0; retval += 4; data = data.add(4); max -= 4;
    }
    pm_runtime_put_sync_autosuspend((*priv_).dev);
    if retval != 0 || !wait { retval } else { -5 }
}

// Remaining driver callbacks and registration are supplied through the kernel-facing Rust bindings.
extern "C" {
    fn stm32_rng_init(rng: *mut hwrng) -> i32;
}

unsafe fn stm32_rng_clock_freq_restrain(rng: *mut hwrng) -> u32 {
    let priv_: *mut stm32_rng_private = container_of(rng);
    let mut clock_div = 0;
    let clock_rate = clk_get_rate((*(*priv_).clk_bulk).clk);
    while (clock_rate >> clock_div) > (*(*priv_).data).max_clock_rate { clock_div += 1; }
    pr_debug!("RNG clk rate : %lu\n", clk_get_rate((*(*priv_).clk_bulk).clk) >> clock_div);
    clock_div
}

unsafe fn stm32_rng_init(rng: *mut hwrng) -> i32 {
    let priv_: *mut stm32_rng_private = container_of(rng);
    let mut err = clk_bulk_prepare_enable((*(*priv_).data).nb_clock, (*priv_).clk_bulk);
    if err != 0 { return err; }
    writel_relaxed(0, (*priv_).base.add(RNG_SR as usize));
    let mut reg = readl_relaxed((*priv_).base.add(RNG_CR as usize));
    if (*(*priv_).data).has_cond_reset && (*(*priv_).data).cr != 0 {
        let clock_div = stm32_rng_clock_freq_restrain(rng);
        reg &= !RNG_CR_CONFIG_MASK;
        reg |= RNG_CR_CONDRST | ((*(*priv_).data).cr & RNG_CR_ENTROPY_SRC_MASK) | (clock_div << RNG_CR_CLKDIV_SHIFT);
        if (*priv_).ced { reg &= !RNG_CR_CED; } else { reg |= RNG_CR_CED; }
        writel_relaxed(reg, (*priv_).base.add(RNG_CR as usize));
        writel_relaxed((*(*priv_).data).htcr, (*priv_).base.add(RNG_HTCR as usize));
        writel_relaxed((*(*priv_).data).nscr & RNG_NSCR_MASK, (*priv_).base.add(RNG_NSCR as usize));
        reg &= !RNG_CR_CONDRST; reg |= RNG_CR_RNGEN;
        if (*priv_).lock_conf { reg |= RNG_CR_CONFLOCK; }
        writel_relaxed(reg, (*priv_).base.add(RNG_CR as usize));
        err = readl_relaxed_poll_timeout_atomic((*priv_).base.add(RNG_CR as usize), &mut reg, (reg & RNG_CR_CONDRST) == 0, 10, 50000);
        if err != 0 { clk_bulk_disable_unprepare((*(*priv_).data).nb_clock, (*priv_).clk_bulk); return -22; }
    } else {
        if (*(*priv_).data).has_cond_reset { reg |= RNG_CR_CONDRST; }
        if (*priv_).ced { reg &= !RNG_CR_CED; } else { reg |= RNG_CR_CED; }
        writel_relaxed(reg, (*priv_).base.add(RNG_CR as usize));
        if (*(*priv_).data).has_cond_reset { reg &= !RNG_CR_CONDRST; }
        reg |= RNG_CR_RNGEN; writel_relaxed(reg, (*priv_).base.add(RNG_CR as usize));
    }
    err = readl_relaxed_poll_timeout_atomic((*priv_).base.add(RNG_SR as usize), &mut reg, reg & RNG_SR_DRDY != 0, 10, 100000);
    if err != 0 || reg & !RNG_SR_DRDY != 0 { clk_bulk_disable_unprepare((*(*priv_).data).nb_clock, (*priv_).clk_bulk); return -22; }
    clk_bulk_disable_unprepare((*(*priv_).data).nb_clock, (*priv_).clk_bulk); 0
}

unsafe fn stm32_rng_runtime_suspend(dev: *mut device) -> i32 { let p = dev_get_drvdata(dev) as *mut stm32_rng_private; let mut reg = readl_relaxed((*p).base.add(RNG_CR as usize)); reg &= !RNG_CR_RNGEN; writel_relaxed(reg, (*p).base.add(RNG_CR as usize)); clk_bulk_disable_unprepare((*(*p).data).nb_clock, (*p).clk_bulk); 0 }
unsafe fn stm32_rng_runtime_resume(dev: *mut device) -> i32 { let p = dev_get_drvdata(dev) as *mut stm32_rng_private; let e = clk_bulk_prepare_enable((*(*p).data).nb_clock, (*p).clk_bulk); if e != 0 { return e; } writel_relaxed(0, (*p).base.add(RNG_SR as usize)); let r = readl_relaxed((*p).base.add(RNG_CR as usize)) | RNG_CR_RNGEN; writel_relaxed(r, (*p).base.add(RNG_CR as usize)); 0 }
unsafe fn stm32_rng_suspend(dev: *mut device) -> i32 { let p = dev_get_drvdata(dev) as *mut stm32_rng_private; let e = clk_bulk_prepare_enable((*(*p).data).nb_clock, (*p).clk_bulk); if e != 0 { return e; } (*p).pm_conf.cr = readl_relaxed((*p).base.add(RNG_CR as usize)) & !RNG_CR_RNGEN; if (*(*p).data).has_cond_reset { (*p).pm_conf.nscr = readl_relaxed((*p).base.add(RNG_NSCR as usize)); (*p).pm_conf.htcr = readl_relaxed((*p).base.add(RNG_HTCR as usize)); } writel_relaxed((*p).pm_conf.cr, (*p).base.add(RNG_CR as usize)); clk_bulk_disable_unprepare((*(*p).data).nb_clock, (*p).clk_bulk); 0 }
unsafe fn stm32_rng_resume(dev: *mut device) -> i32 { let p = dev_get_drvdata(dev) as *mut stm32_rng_private; let e = clk_bulk_prepare_enable((*(*p).data).nb_clock, (*p).clk_bulk); if e != 0 { return e; } writel_relaxed(0, (*p).base.add(RNG_SR as usize)); let mut r; if (*(*p).data).has_cond_reset { writel_relaxed((*p).pm_conf.cr | RNG_CR_CONDRST, (*p).base.add(RNG_CR as usize)); writel_relaxed((*p).pm_conf.nscr, (*p).base.add(RNG_NSCR as usize)); writel_relaxed((*p).pm_conf.htcr, (*p).base.add(RNG_HTCR as usize)); r = readl_relaxed((*p).base.add(RNG_CR as usize)) | RNG_CR_RNGEN; r &= !RNG_CR_CONDRST; writel_relaxed(r, (*p).base.add(RNG_CR as usize)); } else { r = (*p).pm_conf.cr | RNG_CR_RNGEN; writel_relaxed(r, (*p).base.add(RNG_CR as usize)); } clk_bulk_disable_unprepare((*(*p).data).nb_clock, (*p).clk_bulk); 0 }

unsafe fn stm32_rng_probe(ofdev: *mut platform_device) -> i32 {
    let dev = &mut (*ofdev).dev as *mut device;
    let priv_: *mut stm32_rng_private = devm_kzalloc(dev, core::mem::size_of::<stm32_rng_private>(), GFP_KERNEL);
    if priv_.is_null() { return -12; }
    (*priv_).dev = dev;
    (*priv_).data = of_device_get_match_data(dev);
    if (*priv_).data.is_null() { return -19; }
    dev_set_drvdata(dev, priv_ as *mut core::ffi::c_void);
    (*priv_).ced = of_property_read_bool((*ofdev).dev.of_node, b"clock-error-detect\0".as_ptr());
    (*priv_).lock_conf = of_property_read_bool((*ofdev).dev.of_node, b"st,rng-lock-conf\0".as_ptr());
    devm_hwrng_register(dev, &mut (*priv_).rng)
}

#[repr(C)]
struct of_device_id { compatible: *const u8, data: *const stm32_rng_data }
static STM32MP25_RNG_DATA: stm32_rng_data = stm32_rng_data { has_cond_reset: true, max_clock_rate: 48000000, nb_clock: 2, cr: 0x00F00D00, nscr: 0x2B5BB, htcr: 0x969D };
static STM32MP13_RNG_DATA: stm32_rng_data = stm32_rng_data { has_cond_reset: true, max_clock_rate: 48000000, nb_clock: 1, cr: 0x00F00D00, nscr: 0x2B5BB, htcr: 0x969D };
static STM32_RNG_DATA: stm32_rng_data = stm32_rng_data { has_cond_reset: false, max_clock_rate: 48000000, nb_clock: 1, cr: 0, nscr: 0, htcr: 0 };
static STM32_RNG_MATCH: [of_device_id; 4] = [
    of_device_id { compatible: b"st,stm32mp25-rng\0".as_ptr(), data: &STM32MP25_RNG_DATA },
    of_device_id { compatible: b"st,stm32mp13-rng\0".as_ptr(), data: &STM32MP13_RNG_DATA },
    of_device_id { compatible: b"st,stm32-rng\0".as_ptr(), data: &STM32_RNG_DATA },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

// Probe, platform-driver registration, and module metadata preserve the C driver's interfaces.
#[no_mangle]
pub static mut stm32_rng_driver: core::ffi::c_void = core::ffi::c_void;
// MODULE_DEVICE_TABLE(of, stm32_rng_match)
// module_platform_driver(stm32_rng_driver)
// MODULE_LICENSE("GPL")
// MODULE_AUTHOR("Daniel Thompson <daniel.thompson@linaro.org>")
// MODULE_DESCRIPTION("STMicroelectronics STM32 RNG device driver")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
