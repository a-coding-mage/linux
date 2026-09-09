// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * 74xx MMIO GPIO driver
 *
 *  Copyright (C) 2014 Alexander Shiyan <shc_work@mail.ru>
 */

// Linux dependencies supplied by the surrounding kernel translation.

const MMIO_74XX_DIR_IN: u32 = 1 << 8;
const MMIO_74XX_DIR_OUT: u32 = 1 << 9;

#[inline]
const fn mmio_74xx_bit_cnt(x: u32) -> u32 {
    x & 0xff
}

#[repr(C)]
struct mmio_74xx_gpio_priv {
    gen_gc: gpio_generic_chip,
    flags: u32,
}

static mmio_74xx_gpio_ids: [of_device_id; 13] = [
    of_device_id { compatible: "ti,741g125", data: (MMIO_74XX_DIR_IN | 1) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,742g125", data: (MMIO_74XX_DIR_IN | 2) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,74125", data: (MMIO_74XX_DIR_IN | 4) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,74365", data: (MMIO_74XX_DIR_IN | 6) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,74244", data: (MMIO_74XX_DIR_IN | 8) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,741624", data: (MMIO_74XX_DIR_IN | 16) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,741g74", data: (MMIO_74XX_DIR_OUT | 1) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,7474", data: (MMIO_74XX_DIR_OUT | 2) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,74175", data: (MMIO_74XX_DIR_OUT | 4) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,74174", data: (MMIO_74XX_DIR_OUT | 6) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,74273", data: (MMIO_74XX_DIR_OUT | 8) as usize as *const core::ffi::c_void },
    of_device_id { compatible: "ti,7416374", data: (MMIO_74XX_DIR_OUT | 16) as usize as *const core::ffi::c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

unsafe fn mmio_74xx_get_direction(gc: *mut gpio_chip, _offset: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut mmio_74xx_gpio_priv;
    if (*priv_).flags & MMIO_74XX_DIR_OUT != 0 {
        return GPIO_LINE_DIRECTION_OUT;
    }
    GPIO_LINE_DIRECTION_IN
}

unsafe fn mmio_74xx_dir_in(gc: *mut gpio_chip, _gpio: u32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut mmio_74xx_gpio_priv;
    if (*priv_).flags & MMIO_74XX_DIR_IN != 0 {
        return 0;
    }
    -ENOTSUPP
}

unsafe fn mmio_74xx_dir_out(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    let priv_ = gpiochip_get_data(gc) as *mut mmio_74xx_gpio_priv;
    if (*priv_).flags & MMIO_74XX_DIR_OUT != 0 {
        return gpio_generic_chip_set(&mut (*priv_).gen_gc, gpio, val);
    }
    -ENOTSUPP
}

unsafe fn mmio_74xx_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config = core::mem::zeroed();
    let priv_: *mut mmio_74xx_gpio_priv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mmio_74xx_gpio_priv>(), GFP_KERNEL) as *mut _;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).flags = device_get_match_data(&(*pdev).dev) as usize as u32;
    let dat = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(dat) {
        return PTR_ERR(dat);
    }

    config.dev = &mut (*pdev).dev;
    config.sz = ((mmio_74xx_bit_cnt((*priv_).flags) + 7) / 8) as usize;
    config.dat = dat;

    let err = gpio_generic_chip_init(&mut (*priv_).gen_gc, &mut config);
    if err != 0 {
        return err;
    }

    (*priv_).gen_gc.gc.direction_input = Some(mmio_74xx_dir_in);
    (*priv_).gen_gc.gc.direction_output = Some(mmio_74xx_dir_out);
    (*priv_).gen_gc.gc.get_direction = Some(mmio_74xx_get_direction);
    (*priv_).gen_gc.gc.ngpio = mmio_74xx_bit_cnt((*priv_).flags);
    (*priv_).gen_gc.gc.owner = THIS_MODULE;

    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*priv_).gen_gc.gc, priv_ as *mut core::ffi::c_void)
}

static mut mmio_74xx_gpio_driver: platform_driver = platform_driver {
    driver: driver {
        name: "74xx-mmio-gpio",
        of_match_table: mmio_74xx_gpio_ids.as_ptr(),
    },
    probe: Some(mmio_74xx_gpio_probe),
};

// module_platform_driver(mmio_74xx_gpio_driver);
// MODULE_DEVICE_TABLE(of, mmio_74xx_gpio_ids);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Alexander Shiyan <shc_work@mail.ru>");
// MODULE_DESCRIPTION("74xx MMIO GPIO driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
