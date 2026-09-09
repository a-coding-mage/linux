// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ATMEL I2C TPM AT97SC3204T
 *
 * Copyright (C) 2012 V Lab Technologies
 *  Teddy Reed <teddy@prosauce.org>
 * Copyright (C) 2013, Obsidian Research Corp.
 *  Jason Gunthorpe <jgunthorpe@obsidianresearch.com>
 * Device driver for ATMEL I2C TPMs.
 *
 * Teddy Reed determined the basic I2C command flow, unlike other I2C TPM
 * devices the raw TCG formatted TPM command data is written via I2C and then
 * raw TCG formatted TPM command data is returned via I2C.
 *
 * TGC status/locality/etc functions seen in the LPC implementation do not
 * seem to be present.
 */

// Dependencies supplied by the surrounding kernel translation.

const I2C_DRIVER_NAME: &str = "tpm_i2c_atmel";
const TPM_I2C_SHORT_TIMEOUT: u32 = 750;
const TPM_I2C_LONG_TIMEOUT: u32 = 2000;
const ATMEL_STS_OK: u8 = 1;

#[repr(C)]
struct priv_data {
    len: usize,
    /*
     * This is the amount we read on the first try. 25 was chosen to fit a
     * fair number of read responses in the buffer so a 2nd retry can be
     * avoided in small message cases.
     */
    buffer: [u8; core::mem::size_of::<tpm_header>() + 25],
}

unsafe fn i2c_atmel_send(chip: *mut tpm_chip, buf: *mut u8, _bufsiz: usize, len: usize) -> i32 {
    let priv_data = dev_get_drvdata((*chip).dev) as *mut priv_data;
    let client = to_i2c_client((*chip).dev.parent);
    let mut status: i32;

    (*priv_data).len = 0;

    if len <= 2 {
        return -EIO;
    }

    status = i2c_master_send(client, buf, len);

    dev_dbg(&(*chip).dev, "%s(buf=%*ph len=%0zx) -> sts=%d\n", __func__,
        core::cmp::min(64usize, len) as i32, buf, len, status);

    if status < 0 {
        return status;
    }
    if status as usize != len {
        return -E2BIG;
    }
    0
}

unsafe fn i2c_atmel_recv(chip: *mut tpm_chip, buf: *mut u8, count: usize) -> i32 {
    let priv_data = dev_get_drvdata((*chip).dev) as *mut priv_data;
    let client = to_i2c_client((*chip).dev.parent);
    let hdr = (*priv_data).buffer.as_ptr() as *const tpm_header;
    let expected_len: u32;
    let rc: i32;

    if (*priv_data).len == 0 {
        return -EIO;
    }
    expected_len = u32::from_be((*hdr).length);
    if expected_len as usize > count {
        return -ENOMEM;
    }
    if (*priv_data).len >= expected_len as usize {
        dev_dbg(&(*chip).dev, "%s early(buf=%*ph count=%0zx) -> ret=%d\n", __func__,
            core::cmp::min(64usize, expected_len as usize) as i32, buf, count, expected_len);
        core::ptr::copy_nonoverlapping((*priv_data).buffer.as_ptr(), buf, expected_len as usize);
        return expected_len as i32;
    }
    rc = i2c_master_recv(client, buf, expected_len as usize);
    dev_dbg(&(*chip).dev, "%s reread(buf=%*ph count=%0zx) -> ret=%d\n", __func__,
        core::cmp::min(64usize, expected_len as usize) as i32, buf, count, expected_len);
    rc
}

unsafe fn i2c_atmel_cancel(chip: *mut tpm_chip) {
    dev_err(&(*chip).dev, "TPM operation cancellation was requested, but is not supported");
}

unsafe fn i2c_atmel_read_status(chip: *mut tpm_chip) -> u8 {
    let priv_data = dev_get_drvdata((*chip).dev) as *mut priv_data;
    let client = to_i2c_client((*chip).dev.parent);
    let rc: i32;

    (*priv_data).len = 0;
    (*priv_data).buffer.fill(0);
    rc = i2c_master_recv(client, (*priv_data).buffer.as_mut_ptr(), (*priv_data).buffer.len());
    dev_dbg(&(*chip).dev, "%s: sts=%d", __func__, rc);
    if rc <= 0 {
        return 0;
    }
    (*priv_data).len = rc as usize;
    ATMEL_STS_OK
}

unsafe fn i2c_atmel_req_canceled(_chip: *mut tpm_chip, _status: u8) -> bool { false }

static i2c_atmel: tpm_class_ops = tpm_class_ops {
    flags: TPM_OPS_AUTO_STARTUP,
    status: Some(i2c_atmel_read_status), recv: Some(i2c_atmel_recv), send: Some(i2c_atmel_send),
    cancel: Some(i2c_atmel_cancel), req_complete_mask: ATMEL_STS_OK,
    req_complete_val: ATMEL_STS_OK, req_canceled: Some(i2c_atmel_req_canceled),
};

unsafe fn i2c_atmel_probe(client: *mut i2c_client) -> i32 {
    let dev = &mut (*client).dev as *mut device;
    if !i2c_check_functionality((*client).adapter, I2C_FUNC_I2C) { return -ENODEV; }
    let chip = tpmm_chip_alloc(dev, &i2c_atmel);
    if IS_ERR(chip) { return PTR_ERR(chip); }
    let priv_ptr = devm_kzalloc(dev, core::mem::size_of::<priv_data>(), GFP_KERNEL) as *mut priv_data;
    if priv_ptr.is_null() { return -ENOMEM; }
    (*chip).timeout_a = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    (*chip).timeout_b = msecs_to_jiffies(TPM_I2C_LONG_TIMEOUT);
    (*chip).timeout_c = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    (*chip).timeout_d = msecs_to_jiffies(TPM_I2C_SHORT_TIMEOUT);
    dev_set_drvdata(&mut (*chip).dev, priv_ptr as *mut core::ffi::c_void);
    tpm_chip_register(chip)
}

unsafe fn i2c_atmel_remove(client: *mut i2c_client) {
    let dev = &mut (*client).dev;
    let chip = dev_get_drvdata(dev) as *mut tpm_chip;
    tpm_chip_unregister(chip);
}

static i2c_atmel_id: [i2c_device_id; 2] = [i2c_device_id { name: I2C_DRIVER_NAME }, i2c_device_id { name: "" }];

// CONFIG_OF conditional declarations are preserved here for the surrounding build configuration.
#[cfg(CONFIG_OF)]
static i2c_atmel_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "atmel,at97sc3204t" }, of_device_id { compatible: "" },
];

static i2c_atmel_pm_ops: dev_pm_ops = SIMPLE_DEV_PM_OPS!(tpm_pm_suspend, tpm_pm_resume);
static mut i2c_atmel_driver: i2c_driver = i2c_driver {
    id_table: i2c_atmel_id.as_ptr(), probe: Some(i2c_atmel_probe), remove: Some(i2c_atmel_remove),
    driver: device_driver { name: I2C_DRIVER_NAME, pm: &i2c_atmel_pm_ops,
        of_match_table: of_match_ptr!(i2c_atmel_of_match) },
};

module_i2c_driver!(i2c_atmel_driver);

MODULE_AUTHOR!("Jason Gunthorpe <jgunthorpe@obsidianresearch.com>");
MODULE_DESCRIPTION!("Atmel TPM I2C Driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
