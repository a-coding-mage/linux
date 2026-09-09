// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2026, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/driver code are intentionally
// referenced here rather than reimplemented.

const PSP_STATUS_READY: u32 = 1u32 << 31;

/* PSP commands */
const PSP_VALIDATE: u32 = 1;
const PSP_START: u32 = 2;
const PSP_RELEASE_TMR: u32 = 3;
const PSP_VALIDATE_CERT: u32 = 4;

/* PSP special arguments */
const PSP_START_COPY_FW: u32 = 1;

/* PSP response error code */
const PSP_ERROR_CANCEL: u32 = 0xFFFF0002;
const PSP_ERROR_BAD_STATE: u32 = 0xFFFF0007;

const PSP_FW_ALIGN: u32 = 0x10000;
const PSP_CFW_ALIGN: u32 = 0x8000;
const PSP_POLL_INTERVAL: u32 = 20000; /* us */
const PSP_POLL_TIMEOUT: u32 = 1000000; /* us */

#[repr(C)]
pub struct PspDevice {
    pub ddev: *mut drm_device,
    pub conf: psp_config,
    pub fw_buf_sz: u32,
    pub fw_paddr: u64,
    pub fw_buffer: *mut core::ffi::c_void,
    pub certfw_buf_sz: u32,
    pub certfw_paddr: u64,
    pub certfw_buffer: *mut core::ffi::c_void,
}

unsafe fn psp_reg(psp: *mut PspDevice, reg: usize) -> *mut u32 {
    (*psp).conf.psp_regs[reg]
}

unsafe fn psp_set_cmd(psp: *mut PspDevice, regs: *mut u32, cmd: u32,
                      arg0: u32, arg1: u32, arg2: u32) {
    *regs.add(0) = cmd;
    *regs.add(1) = arg0;
    *regs.add(2) = arg1;
    *regs.add(3) = (arg2 | (cmd << 24)) & (*psp).conf.arg2_mask;
}

unsafe fn psp_exec(psp: *mut PspDevice, reg_vals: *mut u32) -> i32 {
    let mut resp_code: u32;
    let mut ready: u32 = 0;
    let mut ret: i32;

    /* Check for PSP ready before any write */
    ret = readx_poll_timeout(readl, psp_reg(psp, PSP_STATUS_REG), &mut ready,
                             (ready & PSP_STATUS_READY) != 0,
                             PSP_POLL_INTERVAL, PSP_POLL_TIMEOUT);
    if ret != 0 {
        drm_err((*psp).ddev, "PSP is not ready, ret 0x%x", ret);
        return ret;
    }

    /* Write command and argument registers */
    for i in 0..PSP_NUM_IN_REGS {
        writel(*reg_vals.add(i), psp_reg(psp, i));
    }

    /* clear and set PSP INTR register to kick off */
    writel(0, psp_reg(psp, PSP_INTR_REG));
    writel((*psp).conf.notify_val, psp_reg(psp, PSP_INTR_REG));

    /* PSP should be busy. Wait for ready, so we know task is done. */
    ret = readx_poll_timeout(readl, psp_reg(psp, PSP_STATUS_REG), &mut ready,
                             (ready & PSP_STATUS_READY) != 0,
                             PSP_POLL_INTERVAL, PSP_POLL_TIMEOUT);
    if ret != 0 {
        drm_err((*psp).ddev, "PSP is not ready, ret 0x%x", ret);
        return ret;
    }

    resp_code = readl(psp_reg(psp, PSP_RESP_REG));
    if resp_code != 0 {
        drm_err((*psp).ddev, "fw return error 0x%x", resp_code);
        return -EIO;
    }
    0
}

pub unsafe fn aie_psp_waitmode_poll(psp: *mut PspDevice) -> i32 {
    let xdna = to_xdna_dev((*psp).ddev);
    let mut mode_reg: u32 = 0;
    let ret = readx_poll_timeout(readl, psp_reg(psp, PSP_PWAITMODE_REG),
                                 &mut mode_reg, (mode_reg & 0x1) == 1,
                                 PSP_POLL_INTERVAL, PSP_POLL_TIMEOUT);
    if ret != 0 {
        XDNA_ERR(xdna, "fw waitmode reg error, ret %d", ret);
    }
    ret
}

pub unsafe fn aie_psp_stop(psp: *mut PspDevice) {
    let mut reg_vals = [0u32; PSP_NUM_IN_REGS];
    psp_set_cmd(psp, reg_vals.as_mut_ptr(), PSP_RELEASE_TMR, 0, 0, 0);
    let ret = psp_exec(psp, reg_vals.as_mut_ptr());
    if ret != 0 {
        drm_err((*psp).ddev, "release tmr failed, ret %d", ret);
    }
}

unsafe fn psp_validate_fw(psp: *mut PspDevice, cmd: u8, paddr: u64,
                          buf_sz: u32) -> i32 {
    let mut reg_vals = [0u32; PSP_NUM_IN_REGS];
    psp_set_cmd(psp, reg_vals.as_mut_ptr(), cmd as u32, paddr as u32,
                (paddr >> 32) as u32, buf_sz);
    let ret = psp_exec(psp, reg_vals.as_mut_ptr());
    if ret != 0 {
        drm_err((*psp).ddev, "failed to validate fw, ret %d", ret);
    }
    ret
}

unsafe fn psp_start(psp: *mut PspDevice) -> i32 {
    let mut reg_vals = [0u32; PSP_NUM_IN_REGS];
    psp_set_cmd(psp, reg_vals.as_mut_ptr(), PSP_START, PSP_START_COPY_FW, 0, 0);
    let ret = psp_exec(psp, reg_vals.as_mut_ptr());
    if ret != 0 {
        drm_err((*psp).ddev, "failed to start fw, ret %d", ret);
    }
    ret
}

pub unsafe fn aie_psp_start(psp: *mut PspDevice) -> i32 {
    let mut ret = psp_validate_fw(psp, PSP_VALIDATE as u8, (*psp).fw_paddr,
                                  (*psp).fw_buf_sz);
    if ret != 0 { return ret; }
    if (*psp).certfw_buf_sz != 0 {
        ret = psp_validate_fw(psp, PSP_VALIDATE_CERT as u8, (*psp).certfw_paddr,
                              (*psp).certfw_buf_sz);
        if ret != 0 { return ret; }
    }
    psp_start(psp)
}

/*
 * PSP requires host physical address to load firmware.
 * Allocate a buffer, obtain its physical address, align, and copy data in.
 */
unsafe fn psp_alloc_fw_buf(psp: *mut PspDevice, fw_data: *const core::ffi::c_void,
                           fw_size: u32, align: u32, buf_sz: *mut u32,
                           paddr: *mut u64) -> *mut core::ffi::c_void {
    *buf_sz = (fw_size + align - 1) & !(align - 1);
    let alloc_sz = *buf_sz + align;
    let buffer = drmm_kmalloc((*psp).ddev, alloc_sz, GFP_KERNEL);
    if buffer.is_null() { return core::ptr::null_mut(); }
    *paddr = virt_to_phys(buffer);
    let offset = ((*paddr + align as u64 - 1) & !(align as u64 - 1)) - *paddr;
    *paddr += offset;
    memcpy((buffer as *mut u8).add(offset as usize) as *mut core::ffi::c_void,
           fw_data, fw_size as usize);
    buffer
}

pub unsafe fn aiem_psp_create(ddev: *mut drm_device,
                              conf: *mut psp_config) -> *mut PspDevice {
    let psp = drmm_kzalloc(ddev, core::mem::size_of::<PspDevice>(), GFP_KERNEL)
        as *mut PspDevice;
    if psp.is_null() { return core::ptr::null_mut(); }
    (*psp).ddev = ddev;
    (*psp).fw_buffer = psp_alloc_fw_buf(psp, (*conf).fw_buf, (*conf).fw_size,
        PSP_FW_ALIGN, &mut (*psp).fw_buf_sz, &mut (*psp).fw_paddr);
    if (*psp).fw_buffer.is_null() { return core::ptr::null_mut(); }
    if (*conf).certfw_size == 0 {
        drm_dbg(ddev, "no cert fw");
    } else {
        (*psp).certfw_buffer = psp_alloc_fw_buf(psp, (*conf).certfw_buf,
            (*conf).certfw_size, PSP_CFW_ALIGN, &mut (*psp).certfw_buf_sz,
            &mut (*psp).certfw_paddr);
        if (*psp).certfw_buffer.is_null() {
            drm_err(ddev, "no memory for cert fw buffer");
            return core::ptr::null_mut();
        }
    }
    memcpy(&mut (*psp).conf as *mut psp_config as *mut core::ffi::c_void,
           conf as *const core::ffi::c_void, core::mem::size_of::<psp_config>());
    psp
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
