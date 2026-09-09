// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for GE FPGA based GPIO
 *
 * Author: Martyn Welch <martyn.welch@ge.com>
 *
 * 2008 (c) GE Intelligent Platforms Embedded Systems, Inc.
 */

/*
 * TODO:
 *
 * Configuration of output modes (totem-pole/open-drain).
 * Interrupt configuration - interrupts are always generated, the FPGA relies
 * on the I/O interrupt controllers mask to stop them from being propagated.
 */

const GEF_GPIO_DIRECT: usize = 0x00;
const GEF_GPIO_IN: usize = 0x04;
const GEF_GPIO_OUT: usize = 0x08;
const GEF_GPIO_TRIG: usize = 0x0C;
const GEF_GPIO_POLAR_A: usize = 0x10;
const GEF_GPIO_POLAR_B: usize = 0x14;
const GEF_GPIO_INT_STAT: usize = 0x18;
const GEF_GPIO_OVERRUN: usize = 0x1C;
const GEF_GPIO_MODE: usize = 0x20;

static gef_gpio_ids: [of_device_id; 4] = [
    of_device_id {
        compatible: c"gef,sbc610-gpio".as_ptr(),
        data: 19 as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: c"gef,sbc310-gpio".as_ptr(),
        data: 6 as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: c"ge,imp3a-gpio".as_ptr(),
        data: 16 as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

unsafe extern "C" {
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn gpio_generic_chip_init(
        chip: *mut gpio_generic_chip,
        config: *mut gpio_generic_chip_config,
    ) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const core::ffi::c_char) -> i32;
    fn devm_kasprintf(
        dev: *mut device,
        flags: gfp_t,
        fmt: *const core::ffi::c_char,
        ...,
    ) -> *mut core::ffi::c_char;
    fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    fn devm_gpiochip_add_data(
        dev: *mut device,
        gc: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
}

#[init]
unsafe fn gef_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config = core::mem::zeroed();
    let dev: *mut device = &mut (*pdev).dev;
    let chip: *mut gpio_generic_chip;
    let gc: *mut gpio_chip;
    let regs: *mut core::ffi::c_void;
    let ret: i32;

    chip = devm_kzalloc(dev, core::mem::size_of::<gpio_generic_chip>(), GFP_KERNEL);
    if chip.is_null() {
        return -ENOMEM;
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    config = gpio_generic_chip_config {
        dev,
        sz: 4,
        dat: (regs as *mut u8).add(GEF_GPIO_IN),
        set: (regs as *mut u8).add(GEF_GPIO_OUT),
        dirin: (regs as *mut u8).add(GEF_GPIO_DIRECT),
        flags: GPIO_GENERIC_BIG_ENDIAN_BYTE_ORDER,
    };

    ret = gpio_generic_chip_init(chip, &mut config);
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            c"failed to initialize the generic GPIO chip\n".as_ptr(),
        );
    }

    gc = &mut (*chip).gc;

    /* Setup pointers to chip functions */
    (*gc).label = devm_kasprintf(dev, GFP_KERNEL, c"%pfw".as_ptr(), dev_fwnode(dev));
    if (*gc).label.is_null() {
        return -ENOMEM;
    }

    (*gc).base = -1;
    (*gc).ngpio = device_get_match_data(dev) as usize;

    /* This function adds a memory mapped GPIO chip */
    ret = devm_gpiochip_add_data(dev, gc, core::ptr::null_mut());
    if ret != 0 {
        return dev_err_probe(dev, ret, c"GPIO chip registration failed\n".as_ptr());
    }

    0
}

static mut gef_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"gef-gpio".as_ptr(),
        of_match_table: gef_gpio_ids.as_ptr(),
    },
};

module_platform_driver_probe!(gef_gpio_driver, gef_gpio_probe);

module_description!("GE I/O FPGA GPIO driver");
module_author!("Martyn Welch <martyn.welch@ge.com>");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
