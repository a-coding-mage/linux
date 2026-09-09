// SPDX-License-Identifier: GPL-2.0-only

// External Linux kernel dependencies corresponding to the C includes are
// supplied by the surrounding translation unit.

unsafe fn zl3073x_spi_probe(spi: *mut spi_device) -> i32 {
    let dev: *mut device = unsafe { &mut (*spi).dev };
    let mut zldev: *mut zl3073x_dev;

    zldev = unsafe { zl3073x_devm_alloc(dev) };
    if unsafe { IS_ERR(zldev) } {
        return unsafe { PTR_ERR(zldev) };
    }

    unsafe {
        (*zldev).regmap = devm_regmap_init_spi(spi, &zl3073x_regmap_config);
    }
    if unsafe { IS_ERR((*zldev).regmap) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*zldev).regmap),
                c"Failed to initialize regmap\n".as_ptr(),
            )
        };
    }

    unsafe { zl3073x_dev_probe(zldev) }
}

static ZL3073X_SPI_ID: [spi_device_id; 6] = [
    spi_device_id { name: c"zl30731".as_ptr() },
    spi_device_id { name: c"zl30732".as_ptr() },
    spi_device_id { name: c"zl30733".as_ptr() },
    spi_device_id { name: c"zl30734".as_ptr() },
    spi_device_id { name: c"zl30735".as_ptr() },
    spi_device_id { name: core::ptr::null() }, // sentinel
];

// MODULE_DEVICE_TABLE(spi, zl3073x_spi_id);

static ZL3073X_SPI_OF_MATCH: [of_device_id; 6] = [
    of_device_id { compatible: c"microchip,zl30731".as_ptr() },
    of_device_id { compatible: c"microchip,zl30732".as_ptr() },
    of_device_id { compatible: c"microchip,zl30733".as_ptr() },
    of_device_id { compatible: c"microchip,zl30734".as_ptr() },
    of_device_id { compatible: c"microchip,zl30735".as_ptr() },
    of_device_id { compatible: core::ptr::null() }, // sentinel
];

// MODULE_DEVICE_TABLE(of, zl3073x_spi_of_match);

static mut ZL3073X_SPI_DRIVER: spi_driver = spi_driver {
    driver: device_driver {
        name: c"zl3073x-spi".as_ptr(),
        of_match_table: ZL3073X_SPI_OF_MATCH.as_ptr(),
    },
    probe: Some(zl3073x_spi_probe),
    id_table: ZL3073X_SPI_ID.as_ptr(),
};

// module_spi_driver(zl3073x_spi_driver);

// MODULE_AUTHOR("Ivan Vecera <ivecera@redhat.com>");
// MODULE_DESCRIPTION("Microchip ZL3073x SPI driver");
// MODULE_IMPORT_NS("ZL3073X");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
