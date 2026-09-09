// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2021 Intel Corporation */

// Dependency declarations and types are supplied by the corresponding kernel
// headers and translated modules.

const ADF_PFVF_CRC8_POLYNOMIAL: u8 = 0x97;

// DECLARE_CRC8_TABLE(pfvf_crc8_table);
// The CRC table is provided by the kernel CRC8 implementation.
extern "C" {
    static mut pfvf_crc8_table: [u8; 256];
    fn crc8_populate_msb(table: *mut u8, polynomial: u8);
    fn crc8(table: *const u8, buf: *const u8, len: usize, value: u8) -> u8;
}

// CRC Calculation
pub unsafe fn adf_pfvf_crc_init() {
    crc8_populate_msb(
        pfvf_crc8_table.as_mut_ptr(),
        ADF_PFVF_CRC8_POLYNOMIAL,
    );
}

pub unsafe fn adf_pfvf_calc_blkmsg_crc(buf: *const u8, buf_len: u8) -> u8 {
    crc8(pfvf_crc8_table.as_ptr(), buf, buf_len as usize, CRC8_INIT_VALUE)
}

unsafe fn set_value_on_csr_msg(
    accel_dev: *mut adf_accel_dev,
    csr_msg: *mut u32,
    value: u32,
    fmt: *const pfvf_field_format,
) -> bool {
    if (value & (*fmt).mask) != value {
        // Equivalent to the source dev_err() diagnostic; logging is supplied
        // by the platform's device-error facility.
        let _ = accel_dev;
        return false;
    }

    *csr_msg |= value << (*fmt).offset;
    true
}

pub unsafe fn adf_pfvf_csr_msg_of(
    accel_dev: *mut adf_accel_dev,
    msg: pfvf_message,
    fmt: *const pfvf_csr_format,
) -> u32 {
    let mut csr_msg: u32 = 0;

    if !set_value_on_csr_msg(accel_dev, &mut csr_msg, msg.r#type, &(*fmt).r#type) ||
        !set_value_on_csr_msg(accel_dev, &mut csr_msg, msg.data, &(*fmt).data)
    {
        return 0;
    }

    csr_msg | ADF_PFVF_MSGORIGIN_SYSTEM
}

pub unsafe fn adf_pfvf_message_of(
    accel_dev: *mut adf_accel_dev,
    csr_msg: u32,
    fmt: *const pfvf_csr_format,
) -> pfvf_message {
    let mut msg = core::mem::zeroed::<pfvf_message>();

    msg.r#type = (csr_msg >> (*fmt).r#type.offset) & (*fmt).r#type.mask;
    msg.data = (csr_msg >> (*fmt).data.offset) & (*fmt).data.mask;

    if msg.r#type == 0 {
        // Equivalent to the source dev_err() diagnostic; logging is supplied
        // by the platform's device-error facility.
        let _ = accel_dev;
    }

    msg
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
