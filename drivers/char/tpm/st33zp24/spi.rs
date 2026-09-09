// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * STMicroelectronics TPM SPI Linux driver for TPM ST33ZP24
 * Copyright (C) 2009 - 2016 STMicroelectronics
 */

// Linux kernel and sibling-module dependencies are supplied externally.

const TPM_DATA_FIFO: u8 = 0x24;
const TPM_INTF_CAPABILITY: u8 = 0x14;
const TPM_DUMMY_BYTE: u8 = 0x00;
const MAX_SPI_LATENCY: i32 = 15;
const LOCALITY0: u8 = 0;

const ST33ZP24_OK: u8 = 0x5a;
const ST33ZP24_UNDEFINED_ERR: u8 = 0x80;
const ST33ZP24_BADLOCALITY: u8 = 0x81;
const ST33ZP24_TISREGISTER_UNKNOWN: u8 = 0x82;
const ST33ZP24_LOCALITY_NOT_ACTIVATED: u8 = 0x83;
const ST33ZP24_HASH_END_BEFORE_HASH_START: u8 = 0x84;
const ST33ZP24_BAD_COMMAND_ORDER: u8 = 0x85;
const ST33ZP24_INCORECT_RECEIVED_LENGTH: u8 = 0x86;
const ST33ZP24_TPM_FIFO_OVERFLOW: u8 = 0x89;
const ST33ZP24_UNEXPECTED_READ_FIFO: u8 = 0x8a;
const ST33ZP24_UNEXPECTED_WRITE_FIFO: u8 = 0x8b;
const ST33ZP24_CMDRDY_SET_WHEN_PROCESSING_HASH_END: u8 = 0x90;
const ST33ZP24_DUMMY_BYTES: u8 = 0x00;

const ST33ZP24_SPI_BUFFER_SIZE: usize =
    ST33ZP24_BUFSIZE + (ST33ZP24_BUFSIZE / 2) + MAX_SPI_LATENCY as usize;

#[repr(C)]
struct st33zp24_spi_phy {
    spi_device: *mut spi_device,
    tx_buf: [u8; ST33ZP24_SPI_BUFFER_SIZE],
    rx_buf: [u8; ST33ZP24_SPI_BUFFER_SIZE],
    latency: i32,
}

unsafe fn st33zp24_status_to_errno(code: u8) -> i32 {
    match code {
        ST33ZP24_OK => 0,
        ST33ZP24_UNDEFINED_ERR
        | ST33ZP24_BADLOCALITY
        | ST33ZP24_TISREGISTER_UNKNOWN
        | ST33ZP24_LOCALITY_NOT_ACTIVATED
        | ST33ZP24_HASH_END_BEFORE_HASH_START
        | ST33ZP24_BAD_COMMAND_ORDER
        | ST33ZP24_UNEXPECTED_READ_FIFO
        | ST33ZP24_UNEXPECTED_WRITE_FIFO
        | ST33ZP24_CMDRDY_SET_WHEN_PROCESSING_HASH_END => -EPROTO,
        ST33ZP24_INCORECT_RECEIVED_LENGTH | ST33ZP24_TPM_FIFO_OVERFLOW => -EMSGSIZE,
        ST33ZP24_DUMMY_BYTES => -ENOSYS,
        _ => code as i32,
    }
}

/*
 * st33zp24_spi_send
 * Send byte to the TIS register according to the ST33ZP24 SPI protocol.
 */
unsafe fn st33zp24_spi_send(
    phy_id: *mut core::ffi::c_void,
    tpm_register: u8,
    tpm_data: *mut u8,
    tpm_size: i32,
) -> i32 {
    let mut total_length: usize = 0;
    let mut ret: i32 = 0;
    let phy = &mut *(phy_id as *mut st33zp24_spi_phy);
    let dev = phy.spi_device;
    let mut spi_xfer = spi_transfer {
        tx_buf: phy.tx_buf.as_ptr() as *const core::ffi::c_void,
        rx_buf: phy.rx_buf.as_mut_ptr() as *mut core::ffi::c_void,
        ..core::mem::zeroed()
    };

    phy.tx_buf[total_length] = TPM_WRITE_DIRECTION | LOCALITY0;
    total_length += 1;
    phy.tx_buf[total_length] = tpm_register;
    total_length += 1;

    if tpm_size > 0 && tpm_register == TPM_DATA_FIFO {
        phy.tx_buf[total_length] = (tpm_size >> 8) as u8;
        total_length += 1;
        phy.tx_buf[total_length] = tpm_size as u8;
        total_length += 1;
    }

    core::ptr::copy_nonoverlapping(tpm_data, phy.tx_buf.as_mut_ptr().add(total_length), tpm_size as usize);
    total_length += tpm_size as usize;
    core::ptr::write_bytes(phy.tx_buf.as_mut_ptr().add(total_length), TPM_DUMMY_BYTE, phy.latency as usize);
    spi_xfer.len = total_length + phy.latency as usize;

    ret = spi_sync_transfer(dev, &mut spi_xfer, 1);
    if ret == 0 {
        ret = phy.rx_buf[total_length + phy.latency as usize - 1] as i32;
    }
    st33zp24_status_to_errno(ret as u8)
}

unsafe fn st33zp24_spi_read8_reg(
    phy_id: *mut core::ffi::c_void,
    tpm_register: u8,
    tpm_data: *mut u8,
    tpm_size: i32,
) -> i32 {
    let mut total_length: usize = 0;
    let mut ret: i32;
    let phy = &mut *(phy_id as *mut st33zp24_spi_phy);
    let dev = phy.spi_device;
    let mut spi_xfer: spi_transfer = core::mem::zeroed();
    spi_xfer.tx_buf = phy.tx_buf.as_ptr() as *const core::ffi::c_void;
    spi_xfer.rx_buf = phy.rx_buf.as_mut_ptr() as *mut core::ffi::c_void;

    phy.tx_buf[total_length] = LOCALITY0;
    total_length += 1;
    phy.tx_buf[total_length] = tpm_register;
    total_length += 1;
    core::ptr::write_bytes(phy.tx_buf.as_mut_ptr().add(total_length), TPM_DUMMY_BYTE,
        (phy.latency + tpm_size) as usize);
    spi_xfer.len = total_length + phy.latency as usize + tpm_size as usize;
    ret = spi_sync_transfer(dev, &mut spi_xfer, 1);
    if tpm_size > 0 && ret == 0 {
        ret = phy.rx_buf[total_length + phy.latency as usize - 1] as i32;
        core::ptr::copy_nonoverlapping(phy.rx_buf.as_ptr().add(total_length + phy.latency as usize), tpm_data, tpm_size as usize);
    }
    ret
}

unsafe fn st33zp24_spi_recv(phy_id: *mut core::ffi::c_void, tpm_register: u8,
    tpm_data: *mut u8, tpm_size: i32) -> i32 {
    let ret = st33zp24_spi_read8_reg(phy_id, tpm_register, tpm_data, tpm_size);
    if st33zp24_status_to_errno(ret as u8) == 0 { tpm_size } else { ret }
}

unsafe fn st33zp24_spi_evaluate_latency(phy_id: *mut core::ffi::c_void) -> i32 {
    let phy = &mut *(phy_id as *mut st33zp24_spi_phy);
    let mut latency: i32 = 1;
    let mut status: i32 = 0;
    let mut data: u8 = 0;
    while status == 0 && latency < MAX_SPI_LATENCY {
        phy.latency = latency;
        status = st33zp24_spi_read8_reg(phy_id, TPM_INTF_CAPABILITY, &mut data, 1);
        latency += 1;
    }
    if status < 0 { return status; }
    if latency == MAX_SPI_LATENCY { return -ENODEV; }
    latency - 1
}

static spi_phy_ops: st33zp24_phy_ops = st33zp24_phy_ops {
    send: Some(st33zp24_spi_send),
    recv: Some(st33zp24_spi_recv),
};

unsafe fn st33zp24_spi_probe(dev: *mut spi_device) -> i32 {
    let phy = devm_kzalloc(&mut (*dev).dev, core::mem::size_of::<st33zp24_spi_phy>(), GFP_KERNEL)
        as *mut st33zp24_spi_phy;
    if phy.is_null() { return -ENOMEM; }
    (*phy).spi_device = dev;
    (*phy).latency = st33zp24_spi_evaluate_latency(phy as *mut core::ffi::c_void);
    if (*phy).latency <= 0 { return -ENODEV; }
    st33zp24_probe(phy as *mut core::ffi::c_void, &spi_phy_ops, &mut (*dev).dev, (*dev).irq)
}

unsafe fn st33zp24_spi_remove(dev: *mut spi_device) {
    let chip = spi_get_drvdata(dev);
    st33zp24_remove(chip);
}

static st33zp24_spi_id: [spi_device_id; 2] = [
    spi_device_id { name: TPM_ST33_SPI, driver_data: 0 },
    spi_device_id { name: 0, driver_data: 0 },
];

static of_st33zp24_spi_match: [of_device_id; 2] = [
    of_device_id { compatible: "st,st33zp24-spi", ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];

static st33zp24_spi_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { id: "SMO3324", ..unsafe { core::mem::zeroed() } },
    acpi_device_id { ..unsafe { core::mem::zeroed() } },
];

static st33zp24_spi_ops: dev_pm_ops = dev_pm_ops {
    suspend: Some(st33zp24_pm_suspend),
    resume: Some(st33zp24_pm_resume),
    ..unsafe { core::mem::zeroed() }
};

static mut st33zp24_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: "st33zp24-spi",
        pm: &st33zp24_spi_ops,
        of_match_table: of_st33zp24_spi_match.as_ptr(),
        acpi_match_table: st33zp24_spi_acpi_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(st33zp24_spi_probe),
    remove: Some(st33zp24_spi_remove),
    id_table: st33zp24_spi_id.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

// Equivalent of module_spi_driver(st33zp24_spi_driver).
// MODULE_AUTHOR("TPM support <TPMsupport@list.st.com>");
// MODULE_DESCRIPTION("STM TPM 1.2 SPI ST33 Driver");
// MODULE_VERSION("1.3.0");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
