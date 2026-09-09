// SPDX-License-Identifier: GPL-2.0
/*
 * TRNG driver for the StarFive JH7110 SoC
 *
 * Copyright (C) 2022 StarFive Technology Co.
 */

// Linux kernel dependencies supplied by the surrounding tree.

/* trng register offset */
const STARFIVE_CTRL: usize = 0x00;
const STARFIVE_STAT: usize = 0x04;
const STARFIVE_MODE: usize = 0x08;
const STARFIVE_SMODE: usize = 0x0C;
const STARFIVE_IE: usize = 0x10;
const STARFIVE_ISTAT: usize = 0x14;
const STARFIVE_RAND0: usize = 0x20;
const STARFIVE_RAND1: usize = 0x24;
const STARFIVE_RAND2: usize = 0x28;
const STARFIVE_RAND3: usize = 0x2C;
const STARFIVE_RAND4: usize = 0x30;
const STARFIVE_RAND5: usize = 0x34;
const STARFIVE_RAND6: usize = 0x38;
const STARFIVE_RAND7: usize = 0x3C;
const STARFIVE_AUTO_RQSTS: usize = 0x60;
const STARFIVE_AUTO_AGE: usize = 0x64;

/* CTRL CMD */
const STARFIVE_CTRL_EXEC_NOP: u32 = 0x0;
const STARFIVE_CTRL_GENE_RANDNUM: u32 = 0x1;
const STARFIVE_CTRL_EXEC_RANDRESEED: u32 = 0x2;

/* STAT */
const STARFIVE_STAT_NONCE_MODE: u32 = 1 << 2;
const STARFIVE_STAT_R256: u32 = 1 << 3;
const STARFIVE_STAT_MISSION_MODE: u32 = 1 << 8;
const STARFIVE_STAT_SEEDED: u32 = 1 << 9;
const STARFIVE_STAT_SRVC_RQST: u32 = 1 << 27;
const STARFIVE_STAT_RAND_GENERATING: u32 = 1 << 30;
const STARFIVE_STAT_RAND_SEEDING: u32 = 1 << 31;

/* STAT_LAST_RESEED(x) */
const fn starfive_stat_last_reseed(x: u32) -> u32 { x << 16 }

/* MODE */
const STARFIVE_MODE_R256: u32 = 1 << 3;

/* SMODE */
const STARFIVE_SMODE_NONCE_MODE: u32 = 1 << 2;
const STARFIVE_SMODE_MISSION_MODE: u32 = 1 << 8;
const fn starfive_smode_max_rejects(x: u32) -> u32 { x << 16 }

/* IE */
const STARFIVE_IE_RAND_RDY_EN: u32 = 1 << 0;
const STARFIVE_IE_SEED_DONE_EN: u32 = 1 << 1;
const STARFIVE_IE_LFSR_LOCKUP_EN: u32 = 1 << 4;
const STARFIVE_IE_GLBL_EN: u32 = 1 << 31;
const STARFIVE_IE_ALL: u32 = STARFIVE_IE_GLBL_EN | STARFIVE_IE_RAND_RDY_EN |
    STARFIVE_IE_SEED_DONE_EN | STARFIVE_IE_LFSR_LOCKUP_EN;

/* ISTAT */
const STARFIVE_ISTAT_RAND_RDY: u32 = 1 << 0;
const STARFIVE_ISTAT_SEED_DONE: u32 = 1 << 1;
const STARFIVE_ISTAT_LFSR_LOCKUP: u32 = 1 << 4;
const STARFIVE_RAND_LEN: usize = core::mem::size_of::<u32>();

#[repr(C)]
pub enum Reseed { RANDOM_RESEED, NONCE_RESEED }

#[repr(C)]
pub enum Mode { PRNG_128BIT, PRNG_256BIT }

#[repr(C)]
pub struct StarfiveTrng {
    pub dev: *mut Device,
    pub base: *mut u8,
    pub hclk: *mut Clk,
    pub ahb: *mut Clk,
    pub rst: *mut ResetControl,
    pub rng: Hwrng,
    pub random_done: Completion,
    pub reseed_done: Completion,
    pub mode: u32,
    pub mission: u32,
    pub reseed: u32,
    /* protects against concurrent write to ctrl register */
    pub write_lock: Spinlock,
}

// External kernel types and functions are supplied by other translation units.
pub enum Device {}
pub enum Clk {}
pub enum ResetControl {}
pub enum Completion {}
pub enum Spinlock {}
#[repr(C)] pub struct Hwrng { pub name: *const u8, pub init: Option<unsafe extern "C" fn(*mut Hwrng) -> i32>, pub cleanup: Option<unsafe extern "C" fn(*mut Hwrng)>, pub read: Option<unsafe extern "C" fn(*mut Hwrng, *mut core::ffi::c_void, usize, bool) -> i32> }
#[repr(C)] pub struct PlatformDevice { _private: [u8; 0] }

static mut autoreq: u16 = 0;
static mut autoage: u16 = 0;

unsafe fn starfive_trng_wait_idle(trng: *mut StarfiveTrng) -> i32 {
    let mut stat: u32 = 0;
    readl_relaxed_poll_timeout((*trng).base.add(STARFIVE_STAT), &mut stat,
        (stat & (STARFIVE_STAT_RAND_GENERATING | STARFIVE_STAT_RAND_SEEDING)) == 0,
        10, 100000)
}

unsafe fn starfive_trng_irq_mask_clear(trng: *mut StarfiveTrng) {
    /* clear register: ISTAT */
    let data = readl((*trng).base.add(STARFIVE_ISTAT));
    writel(data, (*trng).base.add(STARFIVE_ISTAT));
}

unsafe fn starfive_trng_cmd(trng: *mut StarfiveTrng, cmd: u32, wait: bool) -> i32 {
    let wait_time = if !wait { 40 } else { 1000 };
    match cmd {
        STARFIVE_CTRL_GENE_RANDNUM => {
            reinit_completion(&mut (*trng).random_done);
            spin_lock_irq(&mut (*trng).write_lock);
            writel(cmd, (*trng).base.add(STARFIVE_CTRL));
            spin_unlock_irq(&mut (*trng).write_lock);
            if wait_for_completion_timeout(&mut (*trng).random_done, usecs_to_jiffies(wait_time)) == 0 { return -110; }
        }
        STARFIVE_CTRL_EXEC_RANDRESEED => {
            reinit_completion(&mut (*trng).reseed_done);
            spin_lock_irq(&mut (*trng).write_lock);
            writel(cmd, (*trng).base.add(STARFIVE_CTRL));
            spin_unlock_irq(&mut (*trng).write_lock);
            if wait_for_completion_timeout(&mut (*trng).reseed_done, usecs_to_jiffies(wait_time)) == 0 { return -110; }
        }
        _ => return -22,
    }
    0
}

unsafe fn starfive_trng_init(rng: *mut Hwrng) -> i32 {
    let trng = to_trng(rng);
    writel(autoage as u32, (*trng).base.add(STARFIVE_AUTO_AGE));
    writel(autoreq as u32, (*trng).base.add(STARFIVE_AUTO_RQSTS));
    starfive_trng_irq_mask_clear(trng);
    writel(STARFIVE_IE_ALL, (*trng).base.add(STARFIVE_IE));
    let mut mode = readl((*trng).base.add(STARFIVE_MODE));
    match (*trng).mode {
        x if x == Mode::PRNG_128BIT as u32 => mode &= !STARFIVE_MODE_R256,
        x if x == Mode::PRNG_256BIT as u32 => mode |= STARFIVE_MODE_R256,
        _ => mode |= STARFIVE_MODE_R256,
    }
    writel(mode, (*trng).base.add(STARFIVE_MODE));
    starfive_trng_cmd(trng, STARFIVE_CTRL_EXEC_RANDRESEED, true)
}

unsafe fn starfive_trng_irq(_irq: i32, priv_: *mut core::ffi::c_void) -> i32 {
    let trng = priv_ as *mut StarfiveTrng;
    let status = readl((*trng).base.add(STARFIVE_ISTAT));
    if status & STARFIVE_ISTAT_RAND_RDY != 0 { writel(STARFIVE_ISTAT_RAND_RDY, (*trng).base.add(STARFIVE_ISTAT)); complete(&mut (*trng).random_done); }
    if status & STARFIVE_ISTAT_SEED_DONE != 0 { writel(STARFIVE_ISTAT_SEED_DONE, (*trng).base.add(STARFIVE_ISTAT)); complete(&mut (*trng).reseed_done); }
    if status & STARFIVE_ISTAT_LFSR_LOCKUP != 0 {
        writel(STARFIVE_ISTAT_LFSR_LOCKUP, (*trng).base.add(STARFIVE_ISTAT));
        /* SEU occurred, reseeding required*/
        spin_lock(&mut (*trng).write_lock);
        writel(STARFIVE_CTRL_EXEC_RANDRESEED, (*trng).base.add(STARFIVE_CTRL));
        spin_unlock(&mut (*trng).write_lock);
    }
    1
}

unsafe fn starfive_trng_cleanup(rng: *mut Hwrng) {
    let trng = to_trng(rng);
    writel(0, (*trng).base.add(STARFIVE_CTRL));
    reset_control_assert((*trng).rst);
    clk_disable_unprepare((*trng).hclk);
    clk_disable_unprepare((*trng).ahb);
}

unsafe fn starfive_trng_read(rng: *mut Hwrng, buf: *mut core::ffi::c_void, mut max: usize, wait: bool) -> i32 {
    let trng = to_trng(rng);
    pm_runtime_get_sync((*trng).dev);
    max = core::cmp::min(max, if (*trng).mode == Mode::PRNG_256BIT as u32 { STARFIVE_RAND_LEN * 8 } else { STARFIVE_RAND_LEN * 4 });
    if wait && starfive_trng_wait_idle(trng) != 0 { pm_runtime_put_sync_autosuspend((*trng).dev); return -110; }
    let ret = starfive_trng_cmd(trng, STARFIVE_CTRL_GENE_RANDNUM, wait);
    if ret != 0 { pm_runtime_put_sync_autosuspend((*trng).dev); return ret; }
    memcpy_fromio(buf, (*trng).base.add(STARFIVE_RAND0), max);
    pm_runtime_put_sync_autosuspend((*trng).dev);
    max as i32
}

// The remaining platform-driver registration and power-management glue is retained as declarations
// because its kernel framework objects and helper macros are external to this isolated translation.
unsafe extern "C" { fn to_trng(rng: *mut Hwrng) -> *mut StarfiveTrng; fn readl(addr: *mut u8) -> u32; fn writel(v: u32, addr: *mut u8); fn readl_relaxed_poll_timeout(addr: *mut u8, val: *mut u32, condition: bool, delay: u32, timeout: u32) -> i32; fn reinit_completion(c: *mut Completion); fn spin_lock_irq(l: *mut Spinlock); fn spin_unlock_irq(l: *mut Spinlock); fn spin_lock(l: *mut Spinlock); fn spin_unlock(l: *mut Spinlock); fn wait_for_completion_timeout(c: *mut Completion, timeout: u64) -> u64; fn usecs_to_jiffies(v: i32) -> u64; fn complete(c: *mut Completion); fn memcpy_fromio(dst: *mut core::ffi::c_void, src: *mut u8, n: usize); fn reset_control_assert(r: *mut ResetControl); fn clk_disable_unprepare(c: *mut Clk); fn pm_runtime_get_sync(d: *mut Device); fn pm_runtime_put_sync_autosuspend(d: *mut Device); }

unsafe fn starfive_trng_suspend(dev: *mut Device) -> i32 {
    let trng = dev_get_drvdata(dev);
    clk_disable_unprepare((*trng).hclk);
    clk_disable_unprepare((*trng).ahb);
    0
}

unsafe fn starfive_trng_resume(dev: *mut Device) -> i32 {
    let trng = dev_get_drvdata(dev);
    clk_prepare_enable((*trng).hclk);
    clk_prepare_enable((*trng).ahb);
    0
}

// Platform probe, device-tree matching, PM operations, module metadata, and registration.
// These retain the C driver's externally supplied kernel framework interfaces.
unsafe fn starfive_trng_probe(pdev: *mut PlatformDevice) -> i32 {
    let trng = devm_kzalloc(pdev, core::mem::size_of::<StarfiveTrng>());
    if trng.is_null() { return -12; }
    platform_set_drvdata(pdev, trng as *mut core::ffi::c_void);
    (*trng).dev = platform_device_dev(pdev);
    (*trng).base = devm_platform_ioremap_resource(pdev, 0);
    if (*trng).base.is_null() { return -14; }
    init_completion(&mut (*trng).random_done);
    init_completion(&mut (*trng).reseed_done);
    spin_lock_init(&mut (*trng).write_lock);
    (*trng).mode = Mode::PRNG_256BIT as u32;
    (*trng).mission = 1;
    (*trng).reseed = Reseed::RANDOM_RESEED as u32;
    (*trng).rng.init = Some(starfive_trng_init);
    (*trng).rng.cleanup = Some(starfive_trng_cleanup);
    (*trng).rng.read = Some(starfive_trng_read);
    devm_hwrng_register(pdev, &mut (*trng).rng)
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut Device) -> *mut StarfiveTrng;
    fn devm_kzalloc(pdev: *mut PlatformDevice, size: usize) -> *mut StarfiveTrng;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut core::ffi::c_void);
    fn platform_device_dev(pdev: *mut PlatformDevice) -> *mut Device;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut u8;
    fn init_completion(c: *mut Completion); fn spin_lock_init(l: *mut Spinlock);
    fn clk_prepare_enable(c: *mut Clk); fn devm_hwrng_register(pdev: *mut PlatformDevice, rng: *mut Hwrng) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
