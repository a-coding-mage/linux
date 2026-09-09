// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * STMicroelectronics TPM I2C Linux driver for TPM ST33ZP24
 * Copyright (C) 2009 - 2016 STMicroelectronics
 */

// Kernel and sibling-module dependencies supplied externally.

const TPM_DUMMY_BYTE: u8 = 0xAA;

#[repr(C)]
struct St33zp24I2cPhy {
    client: *mut i2c_client,
    buf: [u8; ST33ZP24_BUFSIZE + 1],
}

/*
 * write8_reg
 * Send byte to the TIS register according to the ST33ZP24 I2C protocol.
 */
unsafe fn write8_reg(phy_id: *mut core::ffi::c_void, tpm_register: u8,
                     tpm_data: *mut u8, tpm_size: i32) -> i32 {
    let phy = &mut *(phy_id as *mut St33zp24I2cPhy);
    (*phy).buf[0] = tpm_register;
    core::ptr::copy_nonoverlapping(tpm_data, (*phy).buf.as_mut_ptr().add(1),
                                   tpm_size as usize);
    i2c_master_send((*phy).client, (*phy).buf.as_mut_ptr(), tpm_size + 1)
} /* write8_reg() */

/*
 * read8_reg
 * Recv byte from the TIS register according to the ST33ZP24 I2C protocol.
 */
unsafe fn read8_reg(phy_id: *mut core::ffi::c_void, tpm_register: u8,
                    tpm_data: *mut u8, tpm_size: i32) -> i32 {
    let phy = &mut *(phy_id as *mut St33zp24I2cPhy);
    let mut status: u8 = 0;
    let mut data = TPM_DUMMY_BYTE;

    status = write8_reg(phy as *mut _ as *mut core::ffi::c_void,
                        tpm_register, &mut data, 1) as u8;
    if status == 2 {
        status = i2c_master_recv((*phy).client, tpm_data, tpm_size) as u8;
    }
    status as i32
} /* read8_reg() */

/* Send byte to the TIS register according to the ST33ZP24 I2C protocol. */
unsafe fn st33zp24_i2c_send(phy_id: *mut core::ffi::c_void, tpm_register: u8,
                             tpm_data: *mut u8, tpm_size: i32) -> i32 {
    write8_reg(phy_id, tpm_register | TPM_WRITE_DIRECTION, tpm_data, tpm_size)
}

/* Recv byte from the TIS register according to the ST33ZP24 I2C protocol. */
unsafe fn st33zp24_i2c_recv(phy_id: *mut core::ffi::c_void, tpm_register: u8,
                             tpm_data: *mut u8, tpm_size: i32) -> i32 {
    read8_reg(phy_id, tpm_register, tpm_data, tpm_size)
}

static I2C_PHY_OPS: st33zp24_phy_ops = st33zp24_phy_ops {
    send: Some(st33zp24_i2c_send),
    recv: Some(st33zp24_i2c_recv),
};

/* st33zp24_i2c_probe initialize the TPM device. */
unsafe fn st33zp24_i2c_probe(client: *mut i2c_client) -> i32 {
    if !i2c_check_functionality((*client).adapter, I2C_FUNC_I2C) {
        dev_info(&mut (*client).dev, "client not i2c capable\n");
        return -ENODEV;
    }

    let phy = devm_kzalloc(&mut (*client).dev,
                           core::mem::size_of::<St33zp24I2cPhy>(), GFP_KERNEL)
        as *mut St33zp24I2cPhy;
    if phy.is_null() {
        return -ENOMEM;
    }

    (*phy).client = client;
    st33zp24_probe(phy as *mut core::ffi::c_void, &I2C_PHY_OPS,
                   &mut (*client).dev, (*client).irq)
}

/* st33zp24_i2c_remove remove the TPM device. */
unsafe fn st33zp24_i2c_remove(client: *mut i2c_client) {
    let chip = i2c_get_clientdata(client);
    st33zp24_remove(chip);
}

static ST33ZP24_I2C_ID: [i2c_device_id; 2] = [
    i2c_device_id { name: TPM_ST33_I2C },
    i2c_device_id { name: 0 },
];

static OF_ST33ZP24_I2C_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: "st,st33zp24-i2c" },
    of_device_id { compatible: "" },
];

static ST33ZP24_I2C_ACPI_MATCH: [acpi_device_id; 2] = [
    acpi_device_id { id: "SMO3324" },
    acpi_device_id { id: "" },
];

static ST33ZP24_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: driver {
        name: TPM_ST33_I2C,
        pm: &ST33ZP24_I2C_OPS,
        of_match_table: OF_ST33ZP24_I2C_MATCH.as_ptr(),
        acpi_match_table: ST33ZP24_I2C_ACPI_MATCH.as_ptr(),
    },
    probe: Some(st33zp24_i2c_probe),
    remove: Some(st33zp24_i2c_remove),
    id_table: ST33ZP24_I2C_ID.as_ptr(),
};

// Equivalent of SIMPLE_DEV_PM_OPS and module_i2c_driver declarations.
static ST33ZP24_I2C_OPS: dev_pm_ops = dev_pm_ops {
    suspend: Some(st33zp24_pm_suspend),
    resume: Some(st33zp24_pm_resume),
};

// MODULE_AUTHOR("TPM support <TPMsupport@list.st.com>");
// MODULE_DESCRIPTION("STM TPM 1.2 I2C ST33 Driver");
// MODULE_VERSION("1.3.0");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
