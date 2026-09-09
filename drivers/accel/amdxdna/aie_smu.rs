// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the corresponding kernel headers and aie.h.

const SMU_RESULT_OK: u32 = 1;

/* SMU commands */
const AIE_SMU_POWER_ON: u32 = 0x3;
const AIE_SMU_POWER_OFF: u32 = 0x4;
const AIE_SMU_SET_MPNPUCLK_FREQ: u32 = 0x5;
const AIE_SMU_SET_HCLK_FREQ: u32 = 0x6;
const AIE_SMU_SET_SOFT_DPMLEVEL: u32 = 0x7;
const AIE_SMU_SET_HARD_DPMLEVEL: u32 = 0x8;

#[repr(C)]
pub struct smu_device {
    pub ddev: *mut drm_device,
    pub conf: smu_config,
    pub smu_regs: [*mut core::ffi::c_void; SMU_MAX_REGS],
}

#[allow(non_camel_case_types)]
pub struct drm_device;
#[allow(non_camel_case_types)]
pub struct smu_config {
    pub smu_regs: [*mut core::ffi::c_void; SMU_MAX_REGS],
}

extern "C" {
    fn drmm_kzalloc(
        ddev: *mut drm_device,
        size: usize,
        flags: u32,
    ) -> *mut core::ffi::c_void;
    fn drm_err(ddev: *mut drm_device, fmt: *const core::ffi::c_char, ...);
    fn readx_poll_timeout(
        read: unsafe extern "C" fn(*mut core::ffi::c_void) -> u32,
        addr: *mut core::ffi::c_void,
        val: *mut u32,
        cond: u32,
        delay: u32,
        timeout: u32,
    ) -> i32;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize);
}

extern "C" {
    static SMU_MAX_REGS: usize;
    static SMU_RESP_REG: usize;
    static SMU_ARG_REG: usize;
    static SMU_CMD_REG: usize;
    static SMU_INTR_REG: usize;
    static SMU_OUT_REG: usize;
    static AIE_INTERVAL: u32;
    static AIE_TIMEOUT: u32;
    static GFP_KERNEL: u32;
}

unsafe fn smu_reg(s: *mut smu_device, reg: usize) -> *mut core::ffi::c_void {
    (*s).smu_regs[reg]
}

unsafe fn aie_smu_exec(
    smu: *mut smu_device,
    reg_cmd: u32,
    reg_arg: u32,
    out: *mut u32,
) -> i32 {
    let mut resp: u32 = 0;
    let ret: i32;

    writel(0, smu_reg(smu, SMU_RESP_REG));
    writel(reg_arg, smu_reg(smu, SMU_ARG_REG));
    writel(reg_cmd, smu_reg(smu, SMU_CMD_REG));

    /* Clear and set SMU_INTR_REG to kick off */
    writel(0, smu_reg(smu, SMU_INTR_REG));
    writel(1, smu_reg(smu, SMU_INTR_REG));

    ret = readx_poll_timeout(
        readl,
        smu_reg(smu, SMU_RESP_REG),
        &mut resp,
        resp,
        AIE_INTERVAL,
        AIE_TIMEOUT,
    );
    if ret != 0 {
        drm_err((*smu).ddev, c"smu cmd %d timed out".as_ptr(), reg_cmd);
        return ret;
    }

    if !out.is_null() {
        *out = readl(smu_reg(smu, SMU_OUT_REG));
    }

    if resp != SMU_RESULT_OK {
        drm_err((*smu).ddev, c"smu cmd %d failed, 0x%x".as_ptr(), reg_cmd, resp);
        return -22;
    }

    0
}

pub unsafe fn aie_smu_init(smu: *mut smu_device) -> i32 {
    let mut ret: i32;

    /*
     * Failing to set power off indicates an unrecoverable hardware or
     * firmware error.
     */
    ret = aie_smu_exec(smu, AIE_SMU_POWER_OFF, 0, core::ptr::null_mut());
    if ret != 0 {
        drm_err((*smu).ddev, c"Access power failed, ret %d".as_ptr(), ret);
        return ret;
    }

    ret = aie_smu_exec(smu, AIE_SMU_POWER_ON, 0, core::ptr::null_mut());
    if ret != 0 {
        drm_err((*smu).ddev, c"Power on failed, ret %d".as_ptr(), ret);
        return ret;
    }

    0
}

pub unsafe fn aie_smu_fini(smu: *mut smu_device) {
    let ret = aie_smu_exec(smu, AIE_SMU_POWER_OFF, 0, core::ptr::null_mut());
    if ret != 0 {
        drm_err((*smu).ddev, c"Power off failed, ret %d".as_ptr(), ret);
    }
}

pub unsafe fn aie_smu_set_clocks(
    smu: *mut smu_device,
    npuclk: *mut u32,
    hclk: *mut u32,
) -> i32 {
    let mut ret: i32;

    if !npuclk.is_null() {
        ret = aie_smu_exec(smu, AIE_SMU_SET_MPNPUCLK_FREQ, *npuclk, npuclk);
        if ret != 0 {
            drm_err((*smu).ddev, c"Set mpnpu clock to %d failed, ret %d".as_ptr(), *npuclk, ret);
            return ret;
        }
    }

    if !hclk.is_null() {
        ret = aie_smu_exec(smu, AIE_SMU_SET_HCLK_FREQ, *hclk, hclk);
        if ret != 0 {
            drm_err((*smu).ddev, c"Set hclock to %d failed, ret %d".as_ptr(), *hclk, ret);
            return ret;
        }
    }

    0
}

pub unsafe fn aie_smu_set_dpm(smu: *mut smu_device, dpm_level: u32) -> i32 {
    let mut ret = aie_smu_exec(smu, AIE_SMU_SET_HARD_DPMLEVEL, dpm_level, core::ptr::null_mut());
    if ret != 0 {
        drm_err((*smu).ddev, c"Set hard dpm level %d failed, ret %d".as_ptr(), dpm_level, ret);
        return ret;
    }

    ret = aie_smu_exec(smu, AIE_SMU_SET_SOFT_DPMLEVEL, dpm_level, core::ptr::null_mut());
    if ret != 0 {
        drm_err((*smu).ddev, c"Set soft dpm level %d failed, ret %d".as_ptr(), dpm_level, ret);
        return ret;
    }

    0
}

pub unsafe fn aiem_smu_create(
    ddev: *mut drm_device,
    conf: *mut smu_config,
) -> *mut smu_device {
    let smu = drmm_kzalloc(ddev, core::mem::size_of::<smu_device>(), GFP_KERNEL)
        as *mut smu_device;
    if smu.is_null() {
        return core::ptr::null_mut();
    }

    (*smu).ddev = ddev;
    memcpy(
        (*smu).smu_regs.as_mut_ptr() as *mut core::ffi::c_void,
        (*conf).smu_regs.as_ptr() as *const core::ffi::c_void,
        core::mem::size_of_val(&(*smu).smu_regs),
    );

    smu
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
