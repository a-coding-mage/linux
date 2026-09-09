// SPDX-License-Identifier: GPL-2.0-only
/*
 * Technologic Systems TS-73xx SBC FPGA loader
 *
 * Copyright (C) 2016 Florian Fainelli <f.fainelli@gmail.com>
 *
 * FPGA Manager Driver for the on-board Altera Cyclone II FPGA found on
 * TS-7300, heavily based on load_fpga.c in their vendor tree.
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const TS73XX_FPGA_DATA_REG: usize = 0;
const TS73XX_FPGA_CONFIG_REG: usize = 1;

const TS73XX_FPGA_WRITE_DONE: u8 = 0x1;
const TS73XX_FPGA_WRITE_DONE_TIMEOUT: u32 = 1000; // us
const TS73XX_FPGA_RESET: u8 = 0x2;
const TS73XX_FPGA_RESET_LOW_DELAY: u32 = 30; // us
const TS73XX_FPGA_RESET_HIGH_DELAY: u32 = 80; // us
const TS73XX_FPGA_LOAD_OK: u8 = 0x4;
const TS73XX_FPGA_CONFIG_LOAD: u8 = 0x8;

#[repr(C)]
struct ts73xx_fpga_priv {
    io_base: *mut core::ffi::c_void,
    dev: *mut device,
}

unsafe fn ts73xx_fpga_write_init(
    mgr: *mut fpga_manager,
    _info: *mut fpga_image_info,
    _buf: *const core::ffi::c_char,
    _count: usize,
) -> i32 {
    let priv_ = (*mgr).priv_ as *mut ts73xx_fpga_priv;

    // Reset the FPGA
    writeb(0, (*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));
    udelay(TS73XX_FPGA_RESET_LOW_DELAY);
    writeb(TS73XX_FPGA_RESET, (*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));
    udelay(TS73XX_FPGA_RESET_HIGH_DELAY);

    0
}

unsafe fn ts73xx_fpga_write(
    mgr: *mut fpga_manager,
    buf: *const core::ffi::c_char,
    mut count: usize,
) -> i32 {
    let priv_ = (*mgr).priv_ as *mut ts73xx_fpga_priv;
    let mut i: usize = 0;
    let mut reg: u8;

    while count != 0 {
        let ret = readb_poll_timeout(
            (*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG),
            &mut reg,
            (reg & TS73XX_FPGA_WRITE_DONE) == 0,
            1,
            TS73XX_FPGA_WRITE_DONE_TIMEOUT,
        );
        if ret < 0 {
            return ret;
        }

        writeb(*(buf as *const u8).add(i), (*priv_).io_base.add(TS73XX_FPGA_DATA_REG));
        i += 1;
        count -= 1;
    }

    0
}

unsafe fn ts73xx_fpga_write_complete(
    mgr: *mut fpga_manager,
    _info: *mut fpga_image_info,
) -> i32 {
    let priv_ = (*mgr).priv_ as *mut ts73xx_fpga_priv;
    let mut reg: u8;

    usleep_range(1000, 2000);
    reg = readb((*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));
    reg |= TS73XX_FPGA_CONFIG_LOAD;
    writeb(reg, (*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));

    usleep_range(1000, 2000);
    reg = readb((*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));
    reg &= !TS73XX_FPGA_CONFIG_LOAD;
    writeb(reg, (*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));

    reg = readb((*priv_).io_base.add(TS73XX_FPGA_CONFIG_REG));
    if (reg & TS73XX_FPGA_LOAD_OK) != TS73XX_FPGA_LOAD_OK {
        return -ETIMEDOUT;
    }

    0
}

#[repr(C)]
struct fpga_manager_ops {
    write_init: Option<unsafe fn(*mut fpga_manager, *mut fpga_image_info, *const core::ffi::c_char, usize) -> i32>,
    write: Option<unsafe fn(*mut fpga_manager, *const core::ffi::c_char, usize) -> i32>,
    write_complete: Option<unsafe fn(*mut fpga_manager, *mut fpga_image_info) -> i32>,
}

static ts73xx_fpga_ops: fpga_manager_ops = fpga_manager_ops {
    write_init: Some(ts73xx_fpga_write_init),
    write: Some(ts73xx_fpga_write),
    write_complete: Some(ts73xx_fpga_write_complete),
};

unsafe fn ts73xx_fpga_probe(pdev: *mut platform_device) -> i32 {
    let kdev = &mut (*pdev).dev;
    let priv_ = devm_kzalloc(kdev, core::mem::size_of::<ts73xx_fpga_priv>(), GFP_KERNEL)
        as *mut ts73xx_fpga_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*priv_).dev = kdev;
    (*priv_).io_base = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*priv_).io_base) {
        return ptr_err((*priv_).io_base);
    }

    let mgr = devm_fpga_mgr_register(kdev, "TS-73xx FPGA Manager", &ts73xx_fpga_ops, priv_);
    ptr_err_or_zero(mgr)
}

#[repr(C)]
struct of_device_id {
    compatible: *const core::ffi::c_char,
}

static ts73xx_fpga_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"technologic,ts7300-fpga".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    driver: driver,
    probe: Option<unsafe fn(*mut platform_device) -> i32>,
}

static mut ts73xx_fpga_driver: platform_driver = platform_driver {
    driver: driver {
        name: c"ts73xx-fpga-mgr".as_ptr(),
        of_match_table: ts73xx_fpga_of_match.as_ptr(),
    },
    probe: Some(ts73xx_fpga_probe),
};

// MODULE_DEVICE_TABLE(of, ts73xx_fpga_of_match);
// module_platform_driver(ts73xx_fpga_driver);
// MODULE_AUTHOR("Florian Fainelli <f.fainelli@gmail.com>");
// MODULE_DESCRIPTION("TS-73xx FPGA Manager driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
