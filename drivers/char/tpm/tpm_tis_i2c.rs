// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2014-2021 Nuvoton Technology corporation
 * Copyright (C) 2019-2022 Infineon Technologies AG
 *
 * This device driver implements the TPM interface as defined in the TCG PC
 * Client Platform TPM Profile (PTP) Specification for TPM 2.0 v1.04
 * Revision 14.
 *
 * It is based on the tpm_tis_spi device driver.
 */

/* Linux kernel dependencies are supplied by the surrounding translation unit. */

const TPM_I2C_LOC_SEL: u8 = 0x00;
const TPM_I2C_ACCESS: u8 = 0x04;
const TPM_I2C_INTERFACE_CAPABILITY: u8 = 0x30;
const TPM_I2C_DEVICE_ADDRESS: u8 = 0x38;
const TPM_I2C_DATA_CSUM_ENABLE: u8 = 0x40;
const TPM_DATA_CSUM: u8 = 0x44;
const TPM_I2C_DID_VID: u8 = 0x48;
const TPM_I2C_RID: u8 = 0x4c;

const TPM_LOC_SEL: u32 = 0x0fff;
const TPM_TIS_REGISTER_MASK: u32 = 0x0fff;
const GUARD_TIME_DEFAULT_MIN: u32 = 250;
const GUARD_TIME_DEFAULT_MAX: u32 = 300;
const GUARD_TIME_ERR_MIN: u32 = 250;
const GUARD_TIME_ERR_MAX: u32 = 300;
const TPM_GUARD_TIME_SR_MASK: u32 = 0x40000000;
const TPM_GUARD_TIME_RR_MASK: u32 = 0x00100000;
const TPM_GUARD_TIME_RW_MASK: u32 = 0x00080000;
const TPM_GUARD_TIME_WR_MASK: u32 = 0x00040000;
const TPM_GUARD_TIME_WW_MASK: u32 = 0x00020000;
const TPM_GUARD_TIME_MIN_MASK: u32 = 0x0001fe00;
const TPM_GUARD_TIME_MIN_SHIFT: u32 = 9;
const TPM_ACCESS_READ_ZERO: u32 = 0x48;
const TPM_INT_ENABLE_ZERO: u32 = 0x7fffff60;
const TPM_STS_READ_ZERO: u32 = 0x23;
const TPM_INTF_CAPABILITY_ZERO: u32 = 0x0ffff000;
const TPM_I2C_INTERFACE_CAPABILITY_ZERO: u32 = 0x80000000;

#[repr(C)]
struct tpm_tis_i2c_phy {
    priv_: tpm_tis_data,
    i2c_client: *mut i2c_client,
    guard_time_read: bool,
    guard_time_write: bool,
    guard_time_min: u16,
    guard_time_max: u16,
    io_buf: *mut u8,
}

#[inline]
unsafe fn to_tpm_tis_i2c_phy(data: *mut tpm_tis_data) -> *mut tpm_tis_i2c_phy {
    container_of!(data, tpm_tis_i2c_phy, priv_)
}

/*
 * tpm_tis_core uses the register addresses as defined in Table 19 "Allocation
 * of Register Space for FIFO TPM Access" of the TCG PC Client PTP
 * Specification. In order for this code to work together with tpm_tis_core,
 * those addresses need to mapped to the registers defined for I2C TPMs in
 * Table 51 "I2C-TPM Register Overview".
 *
 * For most addresses this can be done by simply stripping off the locality
 * information from the address. A few addresses need to be mapped explicitly,
 * since the corresponding I2C registers have been moved around. TPM_LOC_SEL is
 * only defined for I2C TPMs and is also mapped explicitly here to distinguish
 * it from TPM_ACCESS(0).
 *
 * Locality information is ignored, since this driver assumes exclusive access
 * to the TPM and always uses locality 0.
 */
unsafe fn tpm_tis_i2c_address_to_register(mut addr: u32) -> u8 {
    addr &= TPM_TIS_REGISTER_MASK;
    match addr {
        x if x == TPM_ACCESS(0) => TPM_I2C_ACCESS,
        x if x == TPM_LOC_SEL => TPM_I2C_LOC_SEL,
        x if x == TPM_DID_VID(0) => TPM_I2C_DID_VID,
        x if x == TPM_RID(0) => TPM_I2C_RID,
        _ => addr as u8,
    }
}

unsafe fn tpm_tis_i2c_retry_transfer_until_ack(data: *mut tpm_tis_data, msg: *mut i2c_msg) -> i32 {
    let phy = to_tpm_tis_i2c_phy(data);
    let guard_time = if (*msg).flags & I2C_M_RD != 0 {
        (*phy).guard_time_read
    } else {
        (*phy).guard_time_write
    };
    let mut i = 0;
    let mut ret;
    loop {
        ret = i2c_transfer((*(*phy).i2c_client).adapter, msg, 1);
        if ret < 0 {
            usleep_range(GUARD_TIME_ERR_MIN, GUARD_TIME_ERR_MAX);
        } else if guard_time {
            usleep_range((*phy).guard_time_min as u32, (*phy).guard_time_max as u32);
        }
        i += 1;
        if !(ret < 0 && i < TPM_RETRY) {
            break;
        }
    }
    ret
}

unsafe fn tpm_tis_i2c_sanity_check_read(reg: u8, len: u16, buf: *mut u8) -> i32 {
    let value = match len as usize {
        1 => *buf as u32,
        2 => le16_to_cpup(buf as *const __le16) as u32,
        4 => le32_to_cpup(buf as *const __le32),
        _ => return 0,
    };
    let zero_mask = match reg as u32 {
        x if x == TPM_I2C_ACCESS as u32 => TPM_ACCESS_READ_ZERO,
        x if x == (TPM_INT_ENABLE(0) & TPM_TIS_REGISTER_MASK) => TPM_INT_ENABLE_ZERO,
        x if x == (TPM_STS(0) & TPM_TIS_REGISTER_MASK) => TPM_STS_READ_ZERO,
        x if x == (TPM_INTF_CAPS(0) & TPM_TIS_REGISTER_MASK) => TPM_INTF_CAPABILITY_ZERO,
        x if x == TPM_I2C_INTERFACE_CAPABILITY as u32 => TPM_I2C_INTERFACE_CAPABILITY_ZERO,
        _ => return 0,
    };
    if (value & zero_mask) != 0 {
        pr_debug!("TPM I2C read of register 0x{:02x} failed sanity check: 0x{:x}\n", reg, value);
        return -EIO;
    }
    0
}

unsafe fn tpm_tis_i2c_read_bytes(data: *mut tpm_tis_data, addr: u32, len: u16, result: *mut u8, _io_mode: enum_tpm_tis_io_mode) -> i32 {
    let phy = to_tpm_tis_i2c_phy(data);
    let mut msg = i2c_msg { addr: (*(*phy).i2c_client).addr, ..core::mem::zeroed() };
    let reg = tpm_tis_i2c_address_to_register(addr);
    let mut ret = 0;
    for _ in 0..TPM_RETRY {
        let mut read: u16 = 0;
        while read < len {
            msg.len = 1;
            msg.buf = &reg as *const u8 as *mut u8;
            msg.flags = 0;
            ret = tpm_tis_i2c_retry_transfer_until_ack(data, &mut msg);
            if ret < 0 { return ret; }
            msg.buf = result.add(read as usize);
            msg.len = len - read;
            msg.flags = I2C_M_RD;
            if msg.len > I2C_SMBUS_BLOCK_MAX { msg.len = I2C_SMBUS_BLOCK_MAX; }
            ret = tpm_tis_i2c_retry_transfer_until_ack(data, &mut msg);
            if ret < 0 { return ret; }
            read += msg.len;
        }
        ret = tpm_tis_i2c_sanity_check_read(reg, len, result);
        if ret == 0 { return 0; }
        usleep_range(GUARD_TIME_ERR_MIN, GUARD_TIME_ERR_MAX);
    }
    ret
}

unsafe fn tpm_tis_i2c_write_bytes(data: *mut tpm_tis_data, addr: u32, len: u16, value: *const u8, _io_mode: enum_tpm_tis_io_mode) -> i32 {
    let phy = to_tpm_tis_i2c_phy(data);
    let mut msg = i2c_msg { addr: (*(*phy).i2c_client).addr, ..core::mem::zeroed() };
    let reg = tpm_tis_i2c_address_to_register(addr);
    let mut wrote: u16 = 0;
    if len > TPM_BUFSIZE - 1 { return -EIO; }
    *(*phy).io_buf = reg;
    msg.buf = (*phy).io_buf;
    while wrote < len {
        msg.len = 1 + len - wrote;
        if msg.len > I2C_SMBUS_BLOCK_MAX { msg.len = I2C_SMBUS_BLOCK_MAX; }
        core::ptr::copy_nonoverlapping(value.add(wrote as usize), (*phy).io_buf.add(1), (msg.len - 1) as usize);
        let ret = tpm_tis_i2c_retry_transfer_until_ack(data, &mut msg);
        if ret < 0 { return ret; }
        wrote += msg.len - 1;
    }
    0
}

unsafe fn tpm_tis_i2c_verify_crc(data: *mut tpm_tis_data, len: usize, value: *const u8) -> i32 {
    let mut crc_tpm: u16 = 0;
    let rc = tpm_tis_read16(data, TPM_DATA_CSUM as u32, &mut crc_tpm);
    if rc < 0 { return rc; }
    let crc_host = swab16(crc_ccitt(0, value, len));
    if crc_tpm != crc_host { return -EIO; }
    0
}

unsafe fn tpm_tis_i2c_init_guard_time(phy: *mut tpm_tis_i2c_phy) -> i32 {
    let mut i2c_caps: u32 = 0;
    (*phy).guard_time_read = true;
    (*phy).guard_time_write = true;
    (*phy).guard_time_min = GUARD_TIME_DEFAULT_MIN as u16;
    (*phy).guard_time_max = GUARD_TIME_DEFAULT_MAX as u16;
    let ret = tpm_tis_i2c_read_bytes(&mut (*phy).priv_, TPM_I2C_INTERFACE_CAPABILITY as u32, 4, &mut i2c_caps as *mut u32 as *mut u8, TPM_TIS_PHYS_32);
    if ret != 0 { return ret; }
    (*phy).guard_time_read = i2c_caps & (TPM_GUARD_TIME_RR_MASK | TPM_GUARD_TIME_RW_MASK) != 0;
    (*phy).guard_time_write = i2c_caps & (TPM_GUARD_TIME_WR_MASK | TPM_GUARD_TIME_WW_MASK) != 0;
    (*phy).guard_time_min = ((i2c_caps & TPM_GUARD_TIME_MIN_MASK) >> TPM_GUARD_TIME_MIN_SHIFT) as u16;
    (*phy).guard_time_max = (*phy).guard_time_min + (*phy).guard_time_min / 5;
    0
}

static tpm_i2c_phy_ops tpm_i2c_phy_ops = tpm_i2c_phy_ops {
    read_bytes: Some(tpm_tis_i2c_read_bytes),
    write_bytes: Some(tpm_tis_i2c_write_bytes),
    verify_crc: Some(tpm_tis_i2c_verify_crc),
};

unsafe fn tpm_tis_i2c_probe(dev: *mut i2c_client) -> i32 {
    let phy = devm_kzalloc(&mut (*dev).dev, core::mem::size_of::<tpm_tis_i2c_phy>(), GFP_KERNEL) as *mut tpm_tis_i2c_phy;
    if phy.is_null() { return -ENOMEM; }
    (*phy).io_buf = devm_kzalloc(&mut (*dev).dev, TPM_BUFSIZE as usize, GFP_KERNEL) as *mut u8;
    if (*phy).io_buf.is_null() { return -ENOMEM; }
    set_bit(TPM_TIS_DEFAULT_CANCELLATION, &mut (*phy).priv_.flags);
    (*phy).i2c_client = dev;
    let mut ret = tpm_tis_i2c_init_guard_time(phy);
    if ret != 0 { return ret; }
    let locality: u8 = 0;
    ret = tpm_tis_i2c_write_bytes(&mut (*phy).priv_, TPM_LOC_SEL, 1, &locality, TPM_TIS_PHYS_8);
    if ret != 0 { return ret; }
    let crc_enable: u8 = 1;
    ret = tpm_tis_i2c_write_bytes(&mut (*phy).priv_, TPM_I2C_DATA_CSUM_ENABLE as u32, 1, &crc_enable, TPM_TIS_PHYS_8);
    if ret != 0 { return ret; }
    tpm_tis_core_init(&mut (*dev).dev, &mut (*phy).priv_, -1, &tpm_i2c_phy_ops, core::ptr::null_mut())
}

unsafe fn tpm_tis_i2c_remove(client: *mut i2c_client) {
    let chip = i2c_get_clientdata(client);
    tpm_chip_unregister(chip);
    tpm_tis_remove(chip);
}

static tpm_tis_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"tpm_tis_i2c".as_ptr() as *const u8 },
    i2c_device_id { name: core::ptr::null() },
];

#[cfg(CONFIG_OF)]
static of_tis_i2c_match: [of_device_id; 4] = [
    of_device_id { compatible: c"infineon,slb9673".as_ptr() as *const u8 },
    of_device_id { compatible: c"nuvoton,npct75x".as_ptr() as *const u8 },
    of_device_id { compatible: c"tcg,tpm-tis-i2c".as_ptr() as *const u8 },
    of_device_id { compatible: core::ptr::null() },
];

static mut tpm_tis_i2c_driver: i2c_driver = i2c_driver {
    driver: driver { name: c"tpm_tis_i2c".as_ptr() as *const u8, pm: &tpm_tis_pm, of_match_table: of_match_ptr!(of_tis_i2c_match) },
    probe: Some(tpm_tis_i2c_probe),
    remove: Some(tpm_tis_i2c_remove),
    id_table: tpm_tis_i2c_id.as_ptr(),
};

module_i2c_driver!(tpm_tis_i2c_driver);

// MODULE_DESCRIPTION("TPM Driver for native I2C access");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
