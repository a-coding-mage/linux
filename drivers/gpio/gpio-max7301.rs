// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006 Juergen Beisert, Pengutronix
 * Copyright (C) 2008 Guennadi Liakhovetski, Pengutronix
 * Copyright (C) 2009 Wolfram Sang, Pengutronix
 *
 * Check max730x.c for further details.
 */

// C dependencies supplied by the surrounding kernel environment.

/* A write to the MAX7301 means one message with one transfer */
unsafe fn max7301_spi_write(dev: *mut device, reg: c_uint, val: c_uint) -> c_int {
    let spi: *mut spi_device = unsafe { to_spi_device(dev) };
    let word: u16 = (((reg & 0x7f) << 8) | (val & 0xff)) as u16;

    unsafe {
        spi_write_then_read(
            spi,
            &word as *const u16 as *const c_void,
            core::mem::size_of::<u16>(),
            core::ptr::null_mut(),
            0,
        )
    }
}

/* A read from the MAX7301 means two transfers; here, one message each */
unsafe fn max7301_spi_read(dev: *mut device, reg: c_uint) -> c_int {
    let spi: *mut spi_device = unsafe { to_spi_device(dev) };
    let mut word: u16 = 0x8000 | (reg << 8) as u16;

    let ret = unsafe {
        spi_write_then_read(
            spi,
            &word as *const u16 as *const c_void,
            core::mem::size_of::<u16>(),
            &mut word as *mut u16 as *mut c_void,
            core::mem::size_of::<u16>(),
        )
    };
    if ret != 0 {
        return ret;
    }
    (word & 0xff) as c_int
}

unsafe fn max7301_probe(spi: *mut spi_device) -> c_int {
    let ts: *mut max7301;
    let ret: c_int;

    /* bits_per_word cannot be configured in platform data */
    unsafe {
        (*spi).bits_per_word = 16;
        ret = spi_setup(spi);
    }
    if ret < 0 {
        return ret;
    }

    ts = unsafe {
        devm_kzalloc(
            &mut (*spi).dev,
            core::mem::size_of::<max7301>(),
            GFP_KERNEL,
        ) as *mut max7301
    };
    if ts.is_null() {
        return -12;
    }

    unsafe {
        (*ts).read = Some(max7301_spi_read);
        (*ts).write = Some(max7301_spi_write);
        (*ts).dev = &mut (*spi).dev;
        __max730x_probe(ts)
    }
}

unsafe fn max7301_remove(spi: *mut spi_device) {
    unsafe {
        __max730x_remove(&mut (*spi).dev);
    }
}

static mut MAX7301_ID: [spi_device_id; 2] = [
    spi_device_id {
        name: "max7301",
        driver_data: 0,
    },
    spi_device_id {
        name: "",
        driver_data: 0,
    },
];

static mut MAX7301_DRIVER: spi_driver = spi_driver {
    driver: driver {
        name: "max7301",
    },
    probe: Some(max7301_probe),
    remove: Some(max7301_remove),
    id_table: unsafe { &MAX7301_ID as *const spi_device_id },
};

unsafe fn max7301_init() -> c_int {
    unsafe { spi_register_driver(&mut MAX7301_DRIVER) }
}

/* register after spi postcore initcall and before
 * subsys initcalls that may rely on these GPIOs
 */
// Equivalent of subsys_initcall(max7301_init).

unsafe fn max7301_exit() {
    unsafe {
        spi_unregister_driver(&mut MAX7301_DRIVER);
    }
}

// Equivalent of module_exit(max7301_exit).
// MODULE_DEVICE_TABLE(spi, max7301_id);
// MODULE_AUTHOR("Juergen Beisert, Wolfram Sang");
// MODULE_LICENSE("GPL v2");
// MODULE_DESCRIPTION("MAX7301 GPIO-Expander");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
