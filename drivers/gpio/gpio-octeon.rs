/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2011, 2012 Cavium Inc.
 */

// Kernel and OCTEON declarations are supplied by the surrounding translation.

const RX_DAT: u64 = 0x80;
const TX_SET: u64 = 0x88;
const TX_CLEAR: u64 = 0x90;

/*
 * The address offset of the GPIO configuration register for a given
 * line.
 */
unsafe fn bit_cfg_reg(offset: u32) -> u32 {
    /*
     * The register stride is 8, with a discontinuity after the
     * first 16.
     */
    if offset < 16 {
        8 * offset
    } else {
        8 * (offset - 16) + 0x100
    }
}

#[repr(C)]
pub struct octeon_gpio {
    pub chip: gpio_chip,
    pub register_base: u64,
}

unsafe fn octeon_gpio_dir_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);

    cvmx_write_csr((*gpio).register_base + bit_cfg_reg(offset) as u64, 0);
    0
}

unsafe fn octeon_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let mask: u64 = 1u64 << offset;
    let reg = (*gpio).register_base + if value != 0 { TX_SET } else { TX_CLEAR };
    cvmx_write_csr(reg, mask);

    0
}

unsafe fn octeon_gpio_dir_out(
    chip: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let mut cfgx: cvmx_gpio_bit_cfgx;

    octeon_gpio_set(chip, offset, value);

    cfgx.u64 = 0;
    cfgx.s.tx_oe = 1;

    cvmx_write_csr((*gpio).register_base + bit_cfg_reg(offset) as u64, cfgx.u64);
    0
}

unsafe fn octeon_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let read_bits: u64 = cvmx_read_csr((*gpio).register_base + RX_DAT);

    (((1u64 << offset) & read_bits) != 0) as i32
}

unsafe fn octeon_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut gpio: *mut octeon_gpio;
    let chip: *mut gpio_chip;
    let mut reg_base: *mut core::ffi::c_void;
    let mut err: i32 = 0;

    gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<octeon_gpio>(), GFP_KERNEL);
    if gpio.is_null() {
        return -ENOMEM;
    }
    chip = &mut (*gpio).chip;

    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(reg_base) {
        return PTR_ERR(reg_base);
    }

    (*gpio).register_base = reg_base as u64;
    (*pdev).dev.platform_data = chip as *mut core::ffi::c_void;
    (*chip).label = c"octeon-gpio".as_ptr();
    (*chip).parent = &mut (*pdev).dev;
    (*chip).owner = THIS_MODULE;
    (*chip).base = 0;
    (*chip).can_sleep = false;
    (*chip).ngpio = 20;
    (*chip).direction_input = Some(octeon_gpio_dir_in);
    (*chip).get = Some(octeon_gpio_get);
    (*chip).direction_output = Some(octeon_gpio_dir_out);
    (*chip).set = Some(octeon_gpio_set);
    err = devm_gpiochip_add_data(&mut (*pdev).dev, chip, gpio as *mut core::ffi::c_void);
    if err != 0 {
        return err;
    }

    dev_info(&mut (*pdev).dev, c"OCTEON GPIO driver probed.\n".as_ptr());
    0
}

static mut octeon_gpio_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"cavium,octeon-3860-gpio".as_ptr(),
    },
    of_device_id { compatible: core::ptr::null() },
];

static mut octeon_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"octeon_gpio".as_ptr(),
        of_match_table: unsafe { octeon_gpio_match.as_ptr() },
    },
    probe: Some(octeon_gpio_probe),
};

// MODULE_DEVICE_TABLE(of, octeon_gpio_match);
// module_platform_driver(octeon_gpio_driver);
// MODULE_DESCRIPTION("Cavium Inc. OCTEON GPIO Driver");
// MODULE_AUTHOR("David Daney");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
