/*
 * Copyright 2011 Advanced Micro Devices, Inc.
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
 *
 * Authors: Alex Deucher
 */

// Dependencies supplied by the surrounding translation unit:
// amdgpu_drm.h, amdgpu.h, atom.h, amdgpu_atombios.h, and atombios_i2c.h.

const TARGET_HW_I2C_CLOCK: u8 = 50;

/* these are a limitation of ProcessI2cChannelTransaction not the hw */
const ATOM_MAX_HW_I2C_WRITE: i32 = 3;
const ATOM_MAX_HW_I2C_READ: i32 = 255;

unsafe fn amdgpu_atombios_i2c_process_i2c_ch(
    chan: *mut amdgpu_i2c_chan,
    slave_addr: u8,
    flags: u8,
    buf: *mut u8,
    mut num: u8,
) -> i32 {
    let dev = (*chan).dev;
    let adev = drm_to_adev(dev);
    let mut args: PROCESS_I2C_CHANNEL_TRANSACTION_PS_ALLOCATION = core::mem::zeroed();
    let index: i32 = GetIndexIntoMasterTable(COMMAND, ProcessI2cChannelTransaction);
    let base: *mut u8;
    let mut out: u16 = cpu_to_le16(0);
    let mut r: i32 = 0;

    mutex_lock(&mut (*chan).mutex);

    base = (*(*adev).mode_info.atom_context).scratch as *mut u8;

    if (flags & HW_I2C_WRITE) != 0 {
        if (num as i32) > ATOM_MAX_HW_I2C_WRITE {
            DRM_ERROR!("hw i2c: tried to write too many bytes (%d vs 3)\\n", num);
            r = -EINVAL;
            goto_done(&mut r, &mut (*chan).mutex);
            return r;
        }
        if buf.is_null() {
            args.ucRegIndex = 0;
        } else {
            args.ucRegIndex = *buf;
        }
        if num != 0 {
            num -= 1;
        }
        if num != 0 {
            if !buf.is_null() {
                core::ptr::copy_nonoverlapping(
                    buf.add(1),
                    &mut out as *mut u16 as *mut u8,
                    num as usize,
                );
            } else {
                DRM_ERROR!("hw i2c: missing buf with num > 1\\n");
                r = -EINVAL;
                goto_done(&mut r, &mut (*chan).mutex);
                return r;
            }
        }
        args.lpI2CDataOut = cpu_to_le16(out);
    } else {
        args.ucRegIndex = 0;
        args.lpI2CDataOut = 0;
    }

    args.ucFlag = flags;
    args.ucI2CSpeed = TARGET_HW_I2C_CLOCK;
    args.ucTransBytes = num;
    args.ucSlaveAddr = slave_addr << 1;
    args.ucLineNumber = (*chan).rec.i2c_id;

    amdgpu_atom_execute_table((*adev).mode_info.atom_context, index, &mut args as *mut _ as *mut u32, core::mem::size_of_val(&args));

    /* error */
    if args.ucStatus != HW_ASSISTED_I2C_STATUS_SUCCESS {
        DRM_DEBUG_KMS!("hw_i2c error\\n");
        r = -EIO;
        goto_done(&mut r, &mut (*chan).mutex);
        return r;
    }

    if (flags & HW_I2C_WRITE) == 0 {
        amdgpu_atombios_copy_swap(buf, base, num, false);
    }

    mutex_unlock(&mut (*chan).mutex);
    r
}

pub unsafe fn amdgpu_atombios_i2c_xfer(
    i2c_adap: *mut i2c_adapter,
    msgs: *mut i2c_msg,
    num: i32,
) -> i32 {
    let i2c = i2c_get_adapdata(i2c_adap);
    let mut p: *mut i2c_msg;
    let mut i: i32;
    let mut remaining: i32;
    let mut current_count: i32;
    let mut buffer_offset: i32;
    let mut max_bytes: i32;
    let mut ret: i32;
    let mut flags: u8;

    /* check for bus probe */
    p = msgs;
    if num == 1 && (*p).len == 0 {
        ret = amdgpu_atombios_i2c_process_i2c_ch(i2c, (*p).addr, HW_I2C_WRITE, core::ptr::null_mut(), 0);
        if ret != 0 {
            return ret;
        } else {
            return num;
        }
    }

    i = 0;
    while i < num {
        p = msgs.add(i as usize);
        remaining = (*p).len as i32;
        buffer_offset = 0;
        /* max_bytes are a limitation of ProcessI2cChannelTransaction not the hw */
        if ((*p).flags & I2C_M_RD) != 0 {
            max_bytes = ATOM_MAX_HW_I2C_READ;
            flags = HW_I2C_READ;
        } else {
            max_bytes = ATOM_MAX_HW_I2C_WRITE;
            flags = HW_I2C_WRITE;
        }
        while remaining != 0 {
            current_count = if remaining > max_bytes { max_bytes } else { remaining };
            ret = amdgpu_atombios_i2c_process_i2c_ch(i2c, (*p).addr, flags, (*p).buf.add(buffer_offset as usize), current_count as u8);
            if ret != 0 {
                return ret;
            }
            remaining -= current_count;
            buffer_offset += current_count;
        }
        i += 1;
    }

    num
}

pub unsafe fn amdgpu_atombios_i2c_func(_adap: *mut i2c_adapter) -> u32 {
    I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL
}

pub unsafe fn amdgpu_atombios_i2c_channel_trans(
    adev: *mut amdgpu_device,
    slave_addr: u8,
    line_number: u8,
    offset: u8,
    data: u8,
) {
    let mut args: PROCESS_I2C_CHANNEL_TRANSACTION_PS_ALLOCATION = core::mem::zeroed();
    let index: i32 = GetIndexIntoMasterTable(COMMAND, ProcessI2cChannelTransaction);

    args.ucRegIndex = offset;
    args.lpI2CDataOut = data;
    args.ucFlag = 1;
    args.ucI2CSpeed = TARGET_HW_I2C_CLOCK;
    args.ucTransBytes = 1;
    args.ucSlaveAddr = slave_addr;
    args.ucLineNumber = line_number;

    amdgpu_atom_execute_table((*adev).mode_info.atom_context, index, &mut args as *mut _ as *mut u32, core::mem::size_of_val(&args));
}

// Local spelling for the C goto cleanup path; supplied mutex operations retain
// the original lock/unlock ordering.
unsafe fn goto_done<T>(_r: &mut i32, mutex: *mut T) {
    mutex_unlock(mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
