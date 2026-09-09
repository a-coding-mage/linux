/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding kernel translation.

const EEPROM_PAGE_BITS: u32 = 8;
const EEPROM_PAGE_SIZE: u32 = 1u32 << EEPROM_PAGE_BITS;
const EEPROM_PAGE_MASK: u32 = EEPROM_PAGE_SIZE - 1;
const EEPROM_OFFSET_SIZE: u16 = 2;

const fn make_i2c_addr(aa: u32) -> u16 {
    ((0xA << 3) | ((aa >> 16) & 0xF)) as u16
}

unsafe fn __amdgpu_eeprom_xfer(
    i2c_adap: *mut i2c_adapter,
    mut eeprom_addr: u32,
    mut eeprom_buf: *mut u8,
    mut buf_size: u32,
    read: bool,
) -> i32 {
    let mut eeprom_offset_buf = [0u8; EEPROM_OFFSET_SIZE as usize];
    let mut msgs = [
        i2c_msg {
            flags: 0,
            len: EEPROM_OFFSET_SIZE,
            buf: eeprom_offset_buf.as_mut_ptr(),
            ..core::mem::zeroed()
        },
        i2c_msg {
            flags: if read { I2C_M_RD } else { 0 },
            ..core::mem::zeroed()
        },
    ];
    let p = eeprom_buf;
    let mut r: i32 = 0;
    let mut len: u16 = 0;

    while buf_size > 0 {
        msgs[0].addr = make_i2c_addr(eeprom_addr);
        msgs[1].addr = msgs[0].addr;
        (*msgs[0].buf.add(0)) = (eeprom_addr >> 8) as u8;
        (*msgs[0].buf.add(1)) = eeprom_addr as u8;

        if !read {
            len = core::cmp::min(
                (EEPROM_PAGE_SIZE - (eeprom_addr & EEPROM_PAGE_MASK)) as u16,
                buf_size as u16,
            );
        } else {
            len = core::cmp::min(u16::MAX as u32, buf_size) as u16;
        }
        msgs[1].len = len;
        msgs[1].buf = eeprom_buf;

        r = i2c_transfer(i2c_adap, msgs.as_mut_ptr(), msgs.len() as i32);
        if r != msgs.len() as i32 {
            break;
        }

        if !read {
            // According to EEPROM specs the self-writing cycle is 10 ms.
            // TODO: Use polling on ACK to minimize waiting for the cycle.
            msleep(10);
        }

        buf_size -= len as u32;
        eeprom_addr = eeprom_addr.wrapping_add(len as u32);
        eeprom_buf = eeprom_buf.add(len as usize);
    }

    if r < 0 { r } else { eeprom_buf.offset_from(p) as i32 }
}

/// amdgpu_eeprom_xfer -- Read/write from/to an I2C EEPROM device.
unsafe fn amdgpu_eeprom_xfer(
    i2c_adap: *mut i2c_adapter,
    mut eeprom_addr: u32,
    mut eeprom_buf: *mut u8,
    mut buf_size: u32,
    read: bool,
) -> i32 {
    let quirks = (*i2c_adap).quirks;
    let limit: u16;
    let mut ps: u16;
    let mut res: i32 = 0;

    if quirks.is_null() {
        limit = 0;
    } else if read {
        limit = (*quirks).max_read_len;
    } else {
        limit = (*quirks).max_write_len;
    }

    if limit == 0 {
        return __amdgpu_eeprom_xfer(i2c_adap, eeprom_addr, eeprom_buf, buf_size, read);
    } else if limit <= EEPROM_OFFSET_SIZE {
        dev_err_ratelimited(
            &mut (*i2c_adap).dev,
            "maddr:0x%04X size:0x%02X:quirk max_%s_len must be > %d",
            eeprom_addr, buf_size, str_read_write(read), EEPROM_OFFSET_SIZE,
        );
        return -EINVAL;
    }

    let data_limit = limit - EEPROM_OFFSET_SIZE;
    while buf_size > 0 {
        ps = core::cmp::min(data_limit as u32, buf_size) as u16;
        let r = __amdgpu_eeprom_xfer(i2c_adap, eeprom_addr, eeprom_buf, ps as u32, read);
        if r < 0 { return r; }
        res += r;
        buf_size -= ps as u32;
        eeprom_addr = eeprom_addr.wrapping_add(ps as u32);
        eeprom_buf = eeprom_buf.add(ps as usize);
    }
    res
}

pub unsafe fn amdgpu_eeprom_read(
    i2c_adap: *mut i2c_adapter, eeprom_addr: u32, eeprom_buf: *mut u8, bytes: u32,
) -> i32 {
    amdgpu_eeprom_xfer(i2c_adap, eeprom_addr, eeprom_buf, bytes, true)
}

pub unsafe fn amdgpu_eeprom_write(
    i2c_adap: *mut i2c_adapter, eeprom_addr: u32, eeprom_buf: *mut u8, bytes: u32,
) -> i32 {
    amdgpu_eeprom_xfer(i2c_adap, eeprom_addr, eeprom_buf, bytes, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
