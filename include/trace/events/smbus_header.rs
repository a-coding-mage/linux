/* SPDX-License-Identifier: GPL-2.0-or-later */
/* SMBUS message transfer tracepoints */

// The C header includes <linux/i2c.h> and <linux/tracepoint.h>.  Their types,
// constants, and tracepoint registration machinery are supplied externally.

#[repr(C)]
pub struct SmbusWriteEntry {
    pub adapter_nr: i32,
    pub addr: u16,
    pub flags: u16,
    pub command: u8,
    pub len: u8,
    pub protocol: u32,
    pub buf: [u8; I2C_SMBUS_BLOCK_MAX + 2],
}

#[repr(C)]
pub struct SmbusReadEntry {
    pub adapter_nr: i32,
    pub flags: u16,
    pub addr: u16,
    pub command: u8,
    pub protocol: u32,
    pub buf: [u8; I2C_SMBUS_BLOCK_MAX + 2],
}

#[repr(C)]
pub struct SmbusReplyEntry {
    pub adapter_nr: i32,
    pub addr: u16,
    pub flags: u16,
    pub command: u8,
    pub len: u8,
    pub protocol: u32,
    pub buf: [u8; I2C_SMBUS_BLOCK_MAX + 2],
}

#[repr(C)]
pub struct SmbusResultEntry {
    pub adapter_nr: i32,
    pub addr: u16,
    pub flags: u16,
    pub read_write: u8,
    pub command: u8,
    pub res: i16,
    pub protocol: u32,
}

/* i2c_smbus_xfer() write data or procedure call request. */
pub unsafe fn smbus_write(
    adap: *const i2c_adapter,
    addr: u16,
    flags: u16,
    read_write: i8,
    command: u8,
    protocol: i32,
    data: *const i2c_smbus_data,
) -> Option<SmbusWriteEntry> {
    if !(read_write == I2C_SMBUS_WRITE
        || protocol == I2C_SMBUS_PROC_CALL
        || protocol == I2C_SMBUS_BLOCK_PROC_CALL)
    {
        return None;
    }

    let mut entry = SmbusWriteEntry {
        adapter_nr: (*adap).nr,
        addr,
        flags,
        command,
        len: 0,
        protocol: protocol as u32,
        buf: [0; I2C_SMBUS_BLOCK_MAX + 2],
    };

    match protocol {
        I2C_SMBUS_BYTE_DATA => {
            entry.len = 1;
            core::ptr::copy_nonoverlapping((*data).block.as_ptr(), entry.buf.as_mut_ptr(), entry.len as usize);
        }
        I2C_SMBUS_WORD_DATA | I2C_SMBUS_PROC_CALL => {
            entry.len = 2;
            core::ptr::copy_nonoverlapping((*data).block.as_ptr(), entry.buf.as_mut_ptr(), entry.len as usize);
        }
        I2C_SMBUS_BLOCK_DATA | I2C_SMBUS_BLOCK_PROC_CALL | I2C_SMBUS_I2C_BLOCK_DATA => {
            entry.len = (*data).block[0].wrapping_add(1);
            core::ptr::copy_nonoverlapping((*data).block.as_ptr(), entry.buf.as_mut_ptr(), entry.len as usize);
        }
        I2C_SMBUS_QUICK | I2C_SMBUS_BYTE | I2C_SMBUS_I2C_BLOCK_BROKEN => {}
        _ => {}
    }
    Some(entry)
}

/* i2c_smbus_xfer() read data request. */
pub unsafe fn smbus_read(
    adap: *const i2c_adapter, addr: u16, flags: u16, read_write: i8,
    command: u8, protocol: i32,
) -> Option<SmbusReadEntry> {
    if read_write == I2C_SMBUS_WRITE || protocol == I2C_SMBUS_PROC_CALL || protocol == I2C_SMBUS_BLOCK_PROC_CALL { return None; }
    Some(SmbusReadEntry { adapter_nr: (*adap).nr, flags, addr, command, protocol: protocol as u32, buf: [0; I2C_SMBUS_BLOCK_MAX + 2] })
}

/* i2c_smbus_xfer() read data or procedure call reply. */
pub unsafe fn smbus_reply(
    adap: *const i2c_adapter, addr: u16, flags: u16, read_write: i8,
    command: u8, protocol: i32, data: *const i2c_smbus_data, res: i32,
) -> Option<SmbusReplyEntry> {
    if res < 0 || read_write != I2C_SMBUS_READ { return None; }
    let mut entry = SmbusReplyEntry { adapter_nr: (*adap).nr, addr, flags, command, len: 0, protocol: protocol as u32, buf: [0; I2C_SMBUS_BLOCK_MAX + 2] };
    match protocol {
        I2C_SMBUS_BYTE | I2C_SMBUS_BYTE_DATA => entry.len = 1,
        I2C_SMBUS_WORD_DATA | I2C_SMBUS_PROC_CALL => entry.len = 2,
        I2C_SMBUS_BLOCK_DATA | I2C_SMBUS_BLOCK_PROC_CALL | I2C_SMBUS_I2C_BLOCK_DATA => entry.len = (*data).block[0].wrapping_add(1),
        I2C_SMBUS_QUICK | I2C_SMBUS_I2C_BLOCK_BROKEN => {}
        _ => {}
    }
    if entry.len != 0 { core::ptr::copy_nonoverlapping((*data).block.as_ptr(), entry.buf.as_mut_ptr(), entry.len as usize); }
    Some(entry)
}

/* i2c_smbus_xfer() result. */
pub fn smbus_result(adap: *const i2c_adapter, addr: u16, flags: u16, read_write: i8, command: u8, protocol: i32, res: i32) -> SmbusResultEntry {
    unsafe { SmbusResultEntry { adapter_nr: (*adap).nr, addr, flags, read_write: read_write as u8, command, res: res as i16, protocol: protocol as u32 } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
