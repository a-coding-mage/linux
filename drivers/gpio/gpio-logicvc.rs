// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2019 Bootlin
 * Author: Paul Kocialkowski <paul.kocialkowski@bootlin.com>
 */

// C dependencies supplied by the kernel and other translation units are intentionally
// referenced here without reimplementing them.

const LOGICVC_CTRL_REG: u32 = 0x40;
const LOGICVC_CTRL_GPIO_SHIFT: u32 = 11;
const LOGICVC_CTRL_GPIO_BITS: u32 = 5;

const LOGICVC_POWER_CTRL_REG: u32 = 0x78;
const LOGICVC_POWER_CTRL_GPIO_SHIFT: u32 = 0;
const LOGICVC_POWER_CTRL_GPIO_BITS: u32 = 4;

#[repr(C)]
struct logicvc_gpio {
    chip: gpio_chip,
    regmap: *mut regmap,
}

unsafe fn logicvc_gpio_offset(
    logicvc: *mut logicvc_gpio,
    mut offset: u32,
    reg: *mut u32,
    bit: *mut u32,
) {
    let _ = logicvc;
    if offset >= LOGICVC_CTRL_GPIO_BITS {
        *reg = LOGICVC_POWER_CTRL_REG;

        /* To the (virtual) power ctrl offset. */
        offset -= LOGICVC_CTRL_GPIO_BITS;
        /* To the actual bit offset in reg. */
        offset += LOGICVC_POWER_CTRL_GPIO_SHIFT;
    } else {
        *reg = LOGICVC_CTRL_REG;

        /* To the actual bit offset in reg. */
        offset += LOGICVC_CTRL_GPIO_SHIFT;
    }

    *bit = 1u32 << offset;
}

unsafe extern "C" fn logicvc_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let logicvc: *mut logicvc_gpio = gpiochip_get_data(chip);
    let mut reg: u32 = 0;
    let mut bit: u32 = 0;
    let mut value: u32 = 0;
    let ret: i32;

    logicvc_gpio_offset(logicvc, offset, &mut reg, &mut bit);

    ret = regmap_read((*logicvc).regmap, reg, &mut value);
    if ret != 0 {
        return ret;
    }

    if value & bit != 0 { 1 } else { 0 }
}

unsafe extern "C" fn logicvc_gpio_set(
    chip: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    let logicvc: *mut logicvc_gpio = gpiochip_get_data(chip);
    let mut reg: u32 = 0;
    let mut bit: u32 = 0;

    logicvc_gpio_offset(logicvc, offset, &mut reg, &mut bit);

    regmap_update_bits((*logicvc).regmap, reg, bit, if value != 0 { bit } else { 0 })
}

unsafe extern "C" fn logicvc_gpio_direction_output(
    chip: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    /* Pins are always configured as output, so just set the value. */
    logicvc_gpio_set(chip, offset, value)
}

static mut logicvc_gpio_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    name: "logicvc-gpio\0".as_ptr() as *const i8,
    max_register: 0,
};

unsafe extern "C" fn logicvc_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let of_node: *mut device_node = (*dev).of_node;
    let logicvc: *mut logicvc_gpio;
    let ret: i32;

    logicvc = devm_kzalloc(dev, core::mem::size_of::<logicvc_gpio>(), GFP_KERNEL);
    if logicvc.is_null() {
        return -ENOMEM;
    }

    /* Try to get regmap from parent first. */
    (*logicvc).regmap = syscon_node_to_regmap((*of_node).parent);

    /* Grab our own regmap if that fails. */
    if IS_ERR((*logicvc).regmap) {
        let mut res: resource = core::mem::zeroed();
        let base: *mut core::ffi::c_void;

        ret = of_address_to_resource(of_node, 0, &mut res);
        if ret != 0 {
            dev_err(dev, "Failed to get resource from address\n");
            return ret;
        }

        base = devm_ioremap_resource(dev, &res);
        if IS_ERR(base) {
            return PTR_ERR(base);
        }

        logicvc_gpio_regmap_config.max_register = resource_size(&res) -
            logicvc_gpio_regmap_config.reg_stride;

        (*logicvc).regmap = devm_regmap_init_mmio(
            dev,
            base,
            &logicvc_gpio_regmap_config,
        );
        if IS_ERR((*logicvc).regmap) {
            dev_err(dev, "Failed to create regmap for I/O\n");
            return PTR_ERR((*logicvc).regmap);
        }
    }

    (*logicvc).chip.parent = dev;
    (*logicvc).chip.owner = THIS_MODULE;
    (*logicvc).chip.label = dev_name(dev);
    (*logicvc).chip.base = -1;
    (*logicvc).chip.ngpio = LOGICVC_CTRL_GPIO_BITS + LOGICVC_POWER_CTRL_GPIO_BITS;
    (*logicvc).chip.get = Some(logicvc_gpio_get);
    (*logicvc).chip.set = Some(logicvc_gpio_set);
    (*logicvc).chip.direction_output = Some(logicvc_gpio_direction_output);

    devm_gpiochip_add_data(dev, &mut (*logicvc).chip, logicvc)
}

static logicivc_gpio_of_table: [of_device_id; 2] = [
    of_device_id {
        compatible: "xylon,logicvc-3.02.a-gpio\0".as_ptr() as *const i8,
    },
    of_device_id { compatible: core::ptr::null() },
];

static mut logicvc_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "gpio-logicvc\0".as_ptr() as *const i8,
        of_match_table: logicivc_gpio_of_table.as_ptr(),
    },
    probe: Some(logicvc_gpio_probe),
};

// MODULE_DEVICE_TABLE(of, logicivc_gpio_of_table);
// module_platform_driver(logicvc_gpio_driver);
// MODULE_AUTHOR("Paul Kocialkowski <paul.kocialkowski@bootlin.com>");
// MODULE_DESCRIPTION("Xylon LogiCVC GPIO driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
