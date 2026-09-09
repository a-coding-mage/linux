// SPDX-License-Identifier: GPL-2.0-or-later
/* APM X-Gene SoC RNG Driver */

// Linux kernel dependencies are supplied by the surrounding Rust environment.

const RNG_MAX_DATUM: u32 = 4;
const MAX_TRY: u32 = 100;
const XGENE_RNG_RETRY_COUNT: u32 = 20;
const XGENE_RNG_RETRY_INTERVAL: u32 = 10;

const RNG_INOUT_0: usize = 0x00;
const RNG_INTR_STS_ACK: usize = 0x10;
const RNG_CONTROL: usize = 0x14;
const RNG_CONFIG: usize = 0x18;
const RNG_ALARMCNT: usize = 0x1c;
const RNG_FROENABLE: usize = 0x20;
const RNG_FRODETUNE: usize = 0x24;
const RNG_ALARMMASK: usize = 0x28;
const RNG_ALARMSTOP: usize = 0x2c;
const RNG_OPTIONS: usize = 0x78;
const RNG_EIP_REV: usize = 0x7c;

const MONOBIT_FAIL_MASK: u32 = 1 << 7;
const POKER_FAIL_MASK: u32 = 1 << 6;
const LONG_RUN_FAIL_MASK: u32 = 1 << 5;
const RUN_FAIL_MASK: u32 = 1 << 4;
const NOISE_FAIL_MASK: u32 = 1 << 3;
const STUCK_OUT_MASK: u32 = 1 << 2;
const SHUTDOWN_OFLO_MASK: u32 = 1 << 1;
const READY_MASK: u32 = 1;

#[repr(C)]
pub struct xgene_rng_dev {
    pub irq: u32,
    pub csr_base: *mut core::ffi::c_void,
    pub revision: u32,
    pub datum_size: u32,
    pub failure_cnt: u32,
    pub failure_ts: usize,
    pub failure_timer: timer_list,
    pub dev: *mut device,
}

#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct hwrng { pub priv_: usize }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct clk { _private: [u8; 0] }

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn udelay(usecs: u32);
    fn disable_irq(irq: u32);
    fn enable_irq(irq: u32);
    fn timer_delete(timer: *mut timer_list);
    fn add_timer(timer: *mut timer_list);
    fn timer_setup(timer: *mut timer_list, function: unsafe extern "C" fn(*mut timer_list), flags: u32);
    fn jiffies() -> usize;
    fn time_after(a: usize, b: usize) -> bool;
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
}

unsafe fn xgene_rng_expired_timer(t: *mut timer_list) {
    let ctx = (t as *mut u8).sub(core::mem::offset_of!(xgene_rng_dev, failure_timer)) as *mut xgene_rng_dev;
    disable_irq((*ctx).irq);
    (*ctx).failure_cnt = 0;
    timer_delete(&mut (*ctx).failure_timer);
    enable_irq((*ctx).irq);
}

unsafe fn xgene_rng_start_timer(ctx: *mut xgene_rng_dev) {
    add_timer(&mut (*ctx).failure_timer);
}

unsafe fn xgene_rng_init_fro(ctx: *mut xgene_rng_dev, fro_val: u32) {
    let base = (*ctx).csr_base as *mut u8;
    writel(fro_val, base.add(RNG_FRODETUNE) as *mut _);
    writel(0, base.add(RNG_ALARMMASK) as *mut _);
    writel(0, base.add(RNG_ALARMSTOP) as *mut _);
    writel(0xffff_ffff, base.add(RNG_FROENABLE) as *mut _);
}

unsafe fn xgene_rng_chk_overflow(ctx: *mut xgene_rng_dev) {
    let val = readl(((*ctx).csr_base as *mut u8).add(RNG_INTR_STS_ACK) as *mut _);
    if val & MONOBIT_FAIL_MASK != 0 { dev_err((*ctx).dev, b"test monobit failure error 0x%08X\0".as_ptr(), val); }
    if val & POKER_FAIL_MASK != 0 { dev_err((*ctx).dev, b"test poker failure error 0x%08X\0".as_ptr(), val); }
    if val & LONG_RUN_FAIL_MASK != 0 { dev_err((*ctx).dev, b"test long run failure error 0x%08X\0".as_ptr(), val); }
    if val & RUN_FAIL_MASK != 0 { dev_err((*ctx).dev, b"test run failure error 0x%08X\0".as_ptr(), val); }
    if val & NOISE_FAIL_MASK != 0 { dev_err((*ctx).dev, b"noise failure error 0x%08X\0".as_ptr(), val); }
    if val & STUCK_OUT_MASK != 0 { dev_err((*ctx).dev, b"stuck out failure error 0x%08X\0".as_ptr(), val); }
    if val & SHUTDOWN_OFLO_MASK != 0 {
        let frostopped = readl(((*ctx).csr_base as *mut u8).add(RNG_ALARMSTOP) as *mut _);
        (*ctx).failure_cnt = (*ctx).failure_cnt.wrapping_add(1);
        xgene_rng_init_fro(ctx, frostopped);
        if (*ctx).failure_cnt == 1 { (*ctx).failure_ts = jiffies(); xgene_rng_start_timer(ctx); }
        else if time_after((*ctx).failure_ts + 60 * 1000, jiffies()) { dev_err((*ctx).dev, b"FRO shutdown failure error 0x%08X\0".as_ptr(), val); }
        else { (*ctx).failure_ts = jiffies(); (*ctx).failure_cnt = 1; xgene_rng_start_timer(ctx); }
    }
    writel(val, ((*ctx).csr_base as *mut u8).add(RNG_INTR_STS_ACK) as *mut _);
}

unsafe extern "C" fn xgene_rng_irq_handler(_irq: i32, id: *mut core::ffi::c_void) -> i32 {
    xgene_rng_chk_overflow(id as *mut xgene_rng_dev); 1
}

// The remaining driver registration and platform glue mirror the C interfaces.
// They depend on kernel-provided types and helpers from the surrounding tree.

unsafe fn xgene_rng_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let ctx = (*rng).priv_ as *mut xgene_rng_dev;
    let mut val = 0;
    for _ in 0..XGENE_RNG_RETRY_COUNT {
        val = readl(((*ctx).csr_base as *mut u8).add(RNG_INTR_STS_ACK) as *mut _);
        if val & READY_MASK != 0 || wait == 0 { break; }
        udelay(XGENE_RNG_RETRY_INTERVAL);
    }
    (val & READY_MASK) as i32
}

unsafe fn xgene_rng_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    let ctx = (*rng).priv_ as *mut xgene_rng_dev;
    for i in 0..(*ctx).datum_size as usize {
        *data.add(i) = readl(((*ctx).csr_base as *mut u8).add(RNG_INOUT_0 + i * 4) as *mut _);
    }
    writel(READY_MASK, ((*ctx).csr_base as *mut u8).add(RNG_INTR_STS_ACK) as *mut _);
    ((*ctx).datum_size) as i32 * 4
}

unsafe fn xgene_rng_init_internal(ctx: *mut xgene_rng_dev) {
    let base = (*ctx).csr_base as *mut u8;
    writel(0, base.add(RNG_CONTROL) as *mut _);
    let mut val = (10u32 << 16) | 10;
    writel(val, base.add(RNG_CONFIG) as *mut _);
    val = 0xff;
    writel(val, base.add(RNG_ALARMCNT) as *mut _);
    xgene_rng_init_fro(ctx, 0);
    writel(MONOBIT_FAIL_MASK | POKER_FAIL_MASK | LONG_RUN_FAIL_MASK | RUN_FAIL_MASK |
        NOISE_FAIL_MASK | STUCK_OUT_MASK | SHUTDOWN_OFLO_MASK | READY_MASK,
        base.add(RNG_INTR_STS_ACK) as *mut _);
    val = (1 << 10) | (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3) | (1 << 2) | (1 << 1);
    writel(val, base.add(RNG_CONTROL) as *mut _);
}

unsafe fn xgene_rng_init(rng: *mut hwrng) -> i32 {
    let ctx = (*rng).priv_ as *mut xgene_rng_dev;
    (*ctx).failure_cnt = 0;
    timer_setup(&mut (*ctx).failure_timer, xgene_rng_expired_timer, 0);
    (*ctx).revision = readl(((*ctx).csr_base as *mut u8).add(RNG_EIP_REV) as *mut _);
    xgene_rng_init_internal(ctx);
    (*ctx).datum_size = RNG_MAX_DATUM;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
