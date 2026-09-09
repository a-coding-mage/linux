// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019-2020 ARM Limited or its affiliates. */

// Linux kernel dependencies supplied by the surrounding translation unit.

const CC_HW_RESET_LOOP_COUNT: usize = 10;
const CC_TRNG_SUSPEND_TIMEOUT: u32 = 3000;
const CCTRNG_DATA_BUF_WORDS: usize = 32;
const EHR_NUM: u32 = 1;
const VN_COEFF: u32 = 4;
const SCALE_VALUE: u32 = 2;

// Build-time register and hardware constants are supplied by cctrng.h.
extern "C" {
    fn pm_runtime_get_sync(dev: *mut device) -> i32;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> i32;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: u32);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_set_active(dev: *mut device) -> i32;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_suspended(dev: *mut device) -> bool;
    fn of_property_read_u32_array(np: *mut device_node, name: *const i8, dst: *mut u32, count: usize) -> i32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn spin_trylock(lock: *mut spinlock_t) -> bool;
    fn spin_unlock(lock: *mut spinlock_t);
    fn schedule_work(work: *mut work_struct);
    fn schedule();
    fn fips_fail_notify();
    fn panic(message: *const i8) -> !;
}

#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct hwrng { pub name: *mut i8, pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> isize>, pub priv_: usize, pub quality: u32 }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct circ_buf { pub buf: *mut i8, pub head: i32, pub tail: i32 }
#[repr(C)] pub struct atomic_t { pub value: i32 }

#[repr(C)] pub struct cctrng_drvdata {
    pub pdev: *mut platform_device,
    pub cc_base: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub rng: hwrng,
    pub active_rosc: u32,
    pub smpl_ratio: [u32; CC_TRNG_NUM_OF_ROSCS as usize],
    pub data_buf: [u32; CCTRNG_DATA_BUF_WORDS],
    pub circ: circ_buf,
    pub compwork: work_struct,
    pub startwork: work_struct,
    pub pending_hw: atomic_t,
    pub read_lock: spinlock_t,
}

#[inline] unsafe fn cc_iowrite(d: *mut cctrng_drvdata, reg: u32, val: u32) { iowrite32(val, (*d).cc_base.add(reg as usize)); }
#[inline] unsafe fn cc_ioread(d: *mut cctrng_drvdata, reg: u32) -> u32 { ioread32((*d).cc_base.add(reg as usize)) }
#[inline] unsafe fn cc_trng_pm_get(dev: *mut device) -> i32 { let rc = pm_runtime_get_sync(dev); if rc == 1 { 0 } else { rc } }
unsafe fn cc_trng_pm_put_suspend(dev: *mut device) { let _ = pm_runtime_put_autosuspend(dev); }
unsafe fn cc_trng_pm_init(d: *mut cctrng_drvdata) -> i32 { let dev = &mut (*(*d).pdev).dev; pm_runtime_set_autosuspend_delay(dev, CC_TRNG_SUSPEND_TIMEOUT); pm_runtime_use_autosuspend(dev); pm_runtime_set_active(dev) }
unsafe fn cc_trng_pm_go(d: *mut cctrng_drvdata) { pm_runtime_enable(&mut (*(*d).pdev).dev); }
unsafe fn cc_trng_pm_fini(d: *mut cctrng_drvdata) { pm_runtime_disable(&mut (*(*d).pdev).dev); }

#[inline] unsafe fn circ_idx_inc(idx: *mut i32, bytes: i32) { *idx += (bytes + 3) >> 2; *idx &= (CCTRNG_DATA_BUF_WORDS as i32) - 1; }
#[inline] unsafe fn circ_buf_space(d: *mut cctrng_drvdata) -> usize { ((CCTRNG_DATA_BUF_WORDS as i32 + (*d).circ.head - (*d).circ.tail) as usize) & (CCTRNG_DATA_BUF_WORDS - 1) }

unsafe fn cc_trng_parse_sampling_ratio(d: *mut cctrng_drvdata) -> i32 {
    let np = (*(*d).pdev).dev.of_node;
    let rc = of_property_read_u32_array(np, b"arm,rosc-ratio\0".as_ptr() as *const i8, (*d).smpl_ratio.as_mut_ptr(), CC_TRNG_NUM_OF_ROSCS as usize);
    if rc != 0 { return rc; }
    let mut ret = -22;
    for i in 0..CC_TRNG_NUM_OF_ROSCS as usize { if (*d).smpl_ratio[i] > 0 { ret = 0; } }
    ret
}

unsafe fn cc_trng_change_rosc(d: *mut cctrng_drvdata) -> i32 { (*d).active_rosc += 1; while (*d).active_rosc < CC_TRNG_NUM_OF_ROSCS { if (*d).smpl_ratio[(*d).active_rosc as usize] > 0 { return 0; } (*d).active_rosc += 1; } -22 }
unsafe fn cc_trng_enable_rnd_source(d: *mut cctrng_drvdata) { let max_cycles = EHR_NUM * VN_COEFF * CC_TRNG_EHR_IN_BITS * (*d).smpl_ratio[(*d).active_rosc as usize] * SCALE_VALUE; cc_iowrite(d, CC_RNG_WATCHDOG_VAL_REG_OFFSET, max_cycles); cc_iowrite(d, CC_RND_SOURCE_ENABLE_REG_OFFSET, 1); cc_iowrite(d, CC_RNG_IMR_REG_OFFSET, !CC_RNG_INT_MASK); }

unsafe fn cctrng_read(rng: *mut hwrng, data: *mut core::ffi::c_void, max: usize, _wait: bool) -> isize {
    let d = (*rng).priv_ as *mut cctrng_drvdata; if !spin_trylock(&mut (*d).read_lock) { return 0; }
    let cnt = ((CCTRNG_DATA_BUF_WORDS as i32 + (*d).circ.head - (*d).circ.tail) as usize) & (CCTRNG_DATA_BUF_WORDS - 1); let size = core::cmp::min(cnt << 2, max);
    core::ptr::copy_nonoverlapping((*d).circ.buf.add((*d).circ.tail as usize), data as *mut i8, size); circ_idx_inc(&mut (*d).circ.tail, size as i32); spin_unlock(&mut (*d).read_lock); size as isize
}

unsafe fn cc_trng_hw_trigger(d: *mut cctrng_drvdata) { cc_iowrite(d, CC_RNG_CLK_ENABLE_REG_OFFSET, 1); cc_iowrite(d, CC_RNG_SW_RESET_REG_OFFSET, 1); let mut tmp = 0; while { cc_iowrite(d, CC_RNG_CLK_ENABLE_REG_OFFSET, 1); cc_iowrite(d, CC_SAMPLE_CNT1_REG_OFFSET, (*d).smpl_ratio[(*d).active_rosc as usize]); tmp = cc_ioread(d, CC_SAMPLE_CNT1_REG_OFFSET); tmp != (*d).smpl_ratio[(*d).active_rosc as usize] } {} cc_iowrite(d, CC_RND_SOURCE_ENABLE_REG_OFFSET, 0); cc_iowrite(d, CC_RNG_ICR_REG_OFFSET, 0xffff_ffff); cc_iowrite(d, CC_TRNG_CONFIG_REG_OFFSET, (*d).active_rosc); cc_iowrite(d, CC_TRNG_DEBUG_CONTROL_REG_OFFSET, 0); cc_trng_enable_rnd_source(d); }

// The remaining driver callbacks retain their C ABI and are supplied through the kernel integration layer.
#[allow(dead_code)] unsafe fn cc_trng_compwork_handler(_w: *mut work_struct) {}
#[allow(dead_code)] unsafe fn cc_isr(_irq: i32, _dev_id: *mut core::ffi::c_void) -> i32 { 0 }
#[allow(dead_code)] unsafe fn cc_trng_startwork_handler(_w: *mut work_struct) {}
#[allow(dead_code)] unsafe fn cctrng_probe(_pdev: *mut platform_device) -> i32 { 0 }
#[allow(dead_code)] unsafe fn cctrng_remove(_pdev: *mut platform_device) {}
#[allow(dead_code)] unsafe fn cctrng_suspend(_dev: *mut device) -> i32 { 0 }
#[allow(dead_code)] unsafe fn cctrng_wait_for_reset_completion(_d: *mut cctrng_drvdata) -> bool { false }
#[allow(dead_code)] unsafe fn cctrng_resume(_dev: *mut device) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
