/*
 * Copyright 2018 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency declarations supplied by the surrounding driver translation.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct pp_hwmgr {
    pub adev: *mut amdgpu_device,
}

#[repr(C)]
pub struct amdgpu_device {
    pub dev: *mut c_void,
    pub reg_offset: [[[u32; 32]; 32]; 32],
}

#[repr(C)]
pub struct baco_cmd_entry {
    pub cmd: u32,
    pub reg_offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub val: u32,
    pub timeout: u32,
}

#[repr(C)]
pub struct soc15_baco_cmd_entry {
    pub cmd: u32,
    pub hwip: usize,
    pub inst: usize,
    pub seg: usize,
    pub reg_offset: u32,
    pub mask: u32,
    pub shift: u32,
    pub val: u32,
    pub timeout: u32,
}

extern "C" {
    fn msleep(milliseconds: u32);
    fn udelay(microseconds: u32);
    fn RREG32(reg: u32) -> u32;
    fn WREG32(reg: u32, value: u32);
    fn dev_warn(dev: *mut c_void, format: *const c_char, ...);
}

extern "C" {
    static CMD_WRITE: u32;
    static CMD_READMODIFYWRITE: u32;
    static CMD_WAITFOR: u32;
    static CMD_DELAY_MS: u32;
    static CMD_DELAY_US: u32;
}

unsafe fn baco_wait_register(hwmgr: *mut pp_hwmgr, reg: u32, mask: u32, value: u32) -> bool {
    let mut timeout: u32 = 5000;
    let mut data: u32;

    loop {
        msleep(1);
        data = RREG32(reg);
        timeout = timeout.wrapping_sub(1);
        if value == (data & mask) || timeout == 0 {
            break;
        }
    }

    let _ = hwmgr;
    timeout != 0
}

unsafe fn baco_cmd_handler(
    hwmgr: *mut pp_hwmgr,
    command: u32,
    reg: u32,
    mask: u32,
    shift: u32,
    value: u32,
    timeout: u32,
) -> bool {
    let adev = (*hwmgr).adev;
    let mut data: u32;
    let mut ret = true;

    if command == CMD_WRITE {
        WREG32(reg, value << shift);
    } else if command == CMD_READMODIFYWRITE {
        data = RREG32(reg);
        data = (data & !mask) | (value << shift);
        WREG32(reg, data);
    } else if command == CMD_WAITFOR {
        ret = baco_wait_register(hwmgr, reg, mask, value);
    } else if command == CMD_DELAY_MS {
        if timeout != 0 {
            /* Delay in milli Seconds */
            msleep(timeout);
        }
    } else if command == CMD_DELAY_US {
        if timeout != 0 {
            /* Delay in micro Seconds */
            udelay(timeout);
        }
    } else {
        dev_warn((*adev).dev, b"Invalid BACO command.\0".as_ptr() as *const c_char);
        ret = false;
    }

    ret
}

pub unsafe fn baco_program_registers(
    hwmgr: *mut pp_hwmgr,
    entry: *const baco_cmd_entry,
    array_size: u32,
) -> bool {
    let mut reg: u32 = 0;

    for i in 0..array_size {
        let current = &*entry.add(i as usize);
        if current.cmd == CMD_WRITE || current.cmd == CMD_READMODIFYWRITE || current.cmd == CMD_WAITFOR {
            reg = current.reg_offset;
        }
        if !baco_cmd_handler(hwmgr, current.cmd, reg, current.mask, current.shift, current.val, current.timeout) {
            return false;
        }
    }
    true
}

pub unsafe fn soc15_baco_program_registers(
    hwmgr: *mut pp_hwmgr,
    entry: *const soc15_baco_cmd_entry,
    array_size: u32,
) -> bool {
    let adev = (*hwmgr).adev;
    let mut reg: u32 = 0;

    for i in 0..array_size {
        let current = &*entry.add(i as usize);
        if current.cmd == CMD_WRITE || current.cmd == CMD_READMODIFYWRITE || current.cmd == CMD_WAITFOR {
            reg = (*adev).reg_offset[current.hwip][current.inst][current.seg].wrapping_add(current.reg_offset);
        }
        if !baco_cmd_handler(hwmgr, current.cmd, reg, current.mask, current.shift, current.val, current.timeout) {
            return false;
        }
    }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
