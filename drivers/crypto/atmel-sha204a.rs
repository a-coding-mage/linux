// SPDX-License-Identifier: GPL-2.0
/*
 * Microchip / Atmel SHA204A (I2C) driver.
 *
 * Copyright (c) 2019 Linaro, Ltd. <ard.biesheuvel@linaro.org>
 */

// Linux kernel dependencies and "atmel-i2c.h" are supplied by the surrounding
// translation unit/build environment.

static AT SHA204_QUALITY: u16 = 1;

unsafe fn atmel_sha204a_rng_done(
    work_data: *mut atmel_i2c_work_data,
    areq: *mut core::ffi::c_void,
    status: i32,
) {
    let i2c_priv = (*work_data).ctx as *mut atmel_i2c_client_priv;
    let rng = areq as *mut hwrng;

    if status != 0 {
        dev_warn_ratelimited(&(*(*i2c_priv).client).dev,
                             "i2c transaction failed (%d)\n", status);
        kfree_sensitive(work_data as *mut core::ffi::c_void);
        atomic_dec(&mut (*i2c_priv).tfm_count);
        return;
    }

    (*rng).priv_ = work_data as usize;
    atomic_dec(&mut (*i2c_priv).tfm_count);
}

unsafe fn atmel_sha204a_rng_read_nonblocking(
    rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    mut max: usize,
) -> i32 {
    let i2c_priv = container_of(rng, atmel_i2c_client_priv, hwrng);
    let mut work_data: *mut atmel_i2c_work_data;

    // keep maximum 1 asynchronous read in flight at any time
    if !atomic_add_unless(&mut (*i2c_priv).tfm_count, 1, 1) {
        return 0;
    }

    if (*rng).priv_ != 0 {
        work_data = (*rng).priv_ as *mut atmel_i2c_work_data;
        max = core::cmp::min(RANDOM_RSP_SIZE - CMD_OVERHEAD_SIZE, max);
        memcpy(data, (*work_data).cmd.data.as_ptr().add(RSP_DATA_IDX), max);
        (*rng).priv_ = 0;
    } else {
        work_data = kmalloc_obj::<atmel_i2c_work_data>(GFP_ATOMIC);
        if work_data.is_null() {
            atomic_dec(&mut (*i2c_priv).tfm_count);
            return -ENOMEM;
        }
        (*work_data).ctx = i2c_priv as *mut core::ffi::c_void;
        (*work_data).client = (*i2c_priv).client;
        max = 0;
    }

    atmel_i2c_init_random_cmd(&mut (*work_data).cmd);
    atmel_i2c_enqueue(work_data, atmel_sha204a_rng_done, rng as *mut core::ffi::c_void);
    max as i32
}

unsafe fn atmel_sha204a_rng_read(
    rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    mut max: usize,
    wait: bool,
) -> i32 {
    if !wait {
        return atmel_sha204a_rng_read_nonblocking(rng, data, max);
    }

    let i2c_priv = container_of(rng, atmel_i2c_client_priv, hwrng);
    let mut cmd: atmel_i2c_cmd = core::mem::zeroed();
    atmel_i2c_init_random_cmd(&mut cmd);

    let mut ret = atmel_i2c_send_receive((*i2c_priv).client, &mut cmd);
    if ret == 0 {
        max = core::cmp::min(RANDOM_RSP_SIZE - CMD_OVERHEAD_SIZE, max);
        memcpy(data, cmd.data.as_ptr().add(RSP_DATA_IDX), max);
        ret = max as i32;
    }
    memzero_explicit(&mut cmd as *mut _ as *mut core::ffi::c_void, core::mem::size_of_val(&cmd));
    ret
}

unsafe fn atmel_sha204a_otp_read(client: *mut i2c_client, addr: u16, otp: *mut u8) -> i32 {
    let mut cmd: atmel_i2c_cmd = core::mem::zeroed();
    let mut ret = atmel_i2c_init_read_otp_cmd(&mut cmd, addr);
    if ret < 0 {
        dev_err(&(*client).dev, "failed, invalid otp address %04X\n", addr);
        return ret;
    }
    ret = atmel_i2c_send_receive(client, &mut cmd);
    if ret < 0 {
        dev_err(&(*client).dev, "failed to read otp at %04X\n", addr);
        return ret;
    }
    if cmd.data[0] == 0xff {
        dev_err(&(*client).dev, "failed, device not ready\n");
        return -EIO;
    }
    memcpy(otp, cmd.data.as_ptr().add(1), 4);
    ret
}

unsafe fn otp_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let client = to_i2c_client(dev);
    let mut otp = [0u8; OTP_ZONE_SIZE];
    let mut addr: u16 = 0;
    while addr < (OTP_ZONE_SIZE / 4) as u16 {
        let ret = atmel_sha204a_otp_read(client, addr, otp.as_mut_ptr().add(addr as usize * 4));
        if ret < 0 { dev_err(dev, "failed to read otp zone\n"); return ret as isize; }
        addr += 1;
    }
    let mut len: isize = 0;
    for i in 0..OTP_ZONE_SIZE { len += sysfs_emit_at(buf, len, "%02X", otp[i]); }
    len += sysfs_emit_at(buf, len, "\n");
    len
}

// DEVICE_ATTR_RO(otp)
static mut DEV_ATTR_OTP: device_attribute = device_attribute { show: Some(otp_show) };

static mut ATSHA204A_ATTRS: [*mut attribute; 2] = [
    unsafe { &mut DEV_ATTR_OTP.attr }, core::ptr::null_mut(),
];
static ATSHA204A_GROUPS: attribute_group = attribute_group {
    name: "atsha204a\0".as_ptr() as *const i8,
    attrs: unsafe { ATSHA204A_ATTRS.as_ptr() },
};

unsafe fn atmel_sha204a_probe(client: *mut i2c_client) -> i32 {
    let mut ret = atmel_i2c_probe(client); if ret != 0 { return ret; }
    let i2c_priv = i2c_get_clientdata(client);
    memset(&mut (*i2c_priv).hwrng as *mut _ as *mut core::ffi::c_void, 0, core::mem::size_of::<hwrng>());
    (*i2c_priv).hwrng.name = dev_name(&(*client).dev);
    (*i2c_priv).hwrng.read = Some(atmel_sha204a_rng_read);
    let quality = i2c_get_match_data(client);
    if !quality.is_null() { (*i2c_priv).hwrng.quality = *quality; }
    ret = devm_hwrng_register(&(*client).dev, &mut (*i2c_priv).hwrng);
    if ret != 0 { dev_err(&(*client).dev, "failed to register RNG (%d)\n", ret); return ret; }
    ret = sysfs_create_group(&mut (*client).dev.kobj, &ATSHA204A_GROUPS);
    if ret != 0 { dev_err(&(*client).dev, "failed to create sysfs group (%d)\n", ret); return ret; }
    ret
}

unsafe fn atmel_sha204a_remove(client: *mut i2c_client) {
    let i2c_priv = i2c_get_clientdata(client);
    sysfs_remove_group(&mut (*client).dev.kobj, &ATSHA204A_GROUPS);
    devm_hwrng_unregister(&(*client).dev, &mut (*i2c_priv).hwrng);
    atmel_i2c_flush_queue();
    kfree_sensitive((*i2c_priv).hwrng.priv_ as *mut core::ffi::c_void);
}

// MODULE_DEVICE_TABLE(of, atmel_sha204a_dt_ids)
static ATSHA204A_DT_IDS: [of_device_id; 3] = [
    of_device_id { compatible: "atmel,atsha204\0".as_ptr() as *const i8 },
    of_device_id { compatible: "atmel,atsha204a\0".as_ptr() as *const i8 }, of_device_id { compatible: core::ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, atmel_sha204a_id)
static ATSHA204A_ID: [i2c_device_id; 3] = [
    i2c_device_id { name: "atsha204\0".as_ptr() as *const i8, driver_data: &ATSHA204_QUALITY as *const _ as usize },
    i2c_device_id { name: "atsha204a\0".as_ptr() as *const i8, driver_data: 0 }, i2c_device_id { name: core::ptr::null(), driver_data: 0 },
];
static mut ATSHA204A_DRIVER: i2c_driver = i2c_driver {
    probe: Some(atmel_sha204a_probe), remove: Some(atmel_sha204a_remove), id_table: ATSHA204A_ID.as_ptr(),
    name: "atmel-sha204a\0".as_ptr() as *const i8, of_match_table: ATSHA204A_DT_IDS.as_ptr(),
};

unsafe fn atmel_sha204a_init() -> i32 { i2c_add_driver(&mut ATSHA204A_DRIVER) }
unsafe fn atmel_sha204a_exit() { atmel_i2c_flush_queue(); i2c_del_driver(&mut ATSHA204A_DRIVER); }

// module_init(atmel_sha204a_init); module_exit(atmel_sha204a_exit);
// MODULE_AUTHOR("Ard Biesheuvel <ard.biesheuvel@linaro.org>");
// MODULE_DESCRIPTION("Microchip / Atmel SHA204A (I2C) driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
