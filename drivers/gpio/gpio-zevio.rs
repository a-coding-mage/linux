// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO controller in LSI ZEVIO SoCs.
 *
 * Author: Fabian Vogt <fabian@ritter-vogt.de>
 */

// Linux kernel dependencies are supplied by the surrounding tree.

const ZEVIO_GPIO_SECTION_SIZE: usize = 0x40;

const ZEVIO_GPIO_INT_MASKED_STATUS: usize = 0x00;
const ZEVIO_GPIO_INT_STATUS: usize = 0x04;
const ZEVIO_GPIO_INT_UNMASK: usize = 0x08;
const ZEVIO_GPIO_INT_MASK: usize = 0x0c;
const ZEVIO_GPIO_DIRECTION: usize = 0x10;
const ZEVIO_GPIO_OUTPUT: usize = 0x14;
const ZEVIO_GPIO_INPUT: usize = 0x18;
const ZEVIO_GPIO_INT_STICKY: usize = 0x20;

#[inline]
unsafe fn zevio_gpio_bit(gpio: u32) -> u32 {
    gpio & 7
}

#[repr(C)]
struct zevio_gpio {
    chip: gpio_chip,
    lock: spinlock_t,
    regs: *mut core::ffi::c_void,
}

#[inline]
unsafe fn zevio_gpio_port_get(c: *mut zevio_gpio, pin: u32, port_offset: usize) -> u32 {
    let section_offset = (((pin >> 3) & 3) as usize) * ZEVIO_GPIO_SECTION_SIZE;
    readl((*c).regs.cast::<u8>().add(section_offset + port_offset).cast())
}

#[inline]
unsafe fn zevio_gpio_port_set(
    c: *mut zevio_gpio,
    pin: u32,
    port_offset: usize,
    val: u32,
) {
    let section_offset = (((pin >> 3) & 3) as usize) * ZEVIO_GPIO_SECTION_SIZE;
    writel(val, (*c).regs.cast::<u8>().add(section_offset + port_offset).cast());
}

unsafe extern "C" fn zevio_gpio_get(chip: *mut gpio_chip, pin: u32) -> i32 {
    let controller = gpiochip_get_data(chip) as *mut zevio_gpio;
    let dir;
    let val;

    spin_lock(&mut (*controller).lock);
    dir = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_DIRECTION);
    if dir & (1u32 << zevio_gpio_bit(pin)) != 0 {
        val = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_INPUT);
    } else {
        val = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_OUTPUT);
    }
    spin_unlock(&mut (*controller).lock);

    ((val >> zevio_gpio_bit(pin)) & 0x1) as i32
}

unsafe extern "C" fn zevio_gpio_set(chip: *mut gpio_chip, pin: u32, value: i32) {
    let controller = gpiochip_get_data(chip) as *mut zevio_gpio;
    let mut val;

    spin_lock(&mut (*controller).lock);
    val = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_OUTPUT);
    if value != 0 {
        val |= 1u32 << zevio_gpio_bit(pin);
    } else {
        val &= !(1u32 << zevio_gpio_bit(pin));
    }
    zevio_gpio_port_set(controller, pin, ZEVIO_GPIO_OUTPUT, val);
    spin_unlock(&mut (*controller).lock);
}

unsafe extern "C" fn zevio_gpio_direction_input(chip: *mut gpio_chip, pin: u32) -> i32 {
    let controller = gpiochip_get_data(chip) as *mut zevio_gpio;
    let mut val;

    spin_lock(&mut (*controller).lock);
    val = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_DIRECTION);
    val |= 1u32 << zevio_gpio_bit(pin);
    zevio_gpio_port_set(controller, pin, ZEVIO_GPIO_DIRECTION, val);
    spin_unlock(&mut (*controller).lock);
    0
}

unsafe extern "C" fn zevio_gpio_direction_output(
    chip: *mut gpio_chip,
    pin: u32,
    value: i32,
) -> i32 {
    let controller = gpiochip_get_data(chip) as *mut zevio_gpio;
    let mut val;

    spin_lock(&mut (*controller).lock);
    val = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_OUTPUT);
    if value != 0 {
        val |= 1u32 << zevio_gpio_bit(pin);
    } else {
        val &= !(1u32 << zevio_gpio_bit(pin));
    }
    zevio_gpio_port_set(controller, pin, ZEVIO_GPIO_OUTPUT, val);
    val = zevio_gpio_port_get(controller, pin, ZEVIO_GPIO_DIRECTION);
    val &= !(1u32 << zevio_gpio_bit(pin));
    zevio_gpio_port_set(controller, pin, ZEVIO_GPIO_DIRECTION, val);
    spin_unlock(&mut (*controller).lock);
    0
}

unsafe extern "C" fn zevio_gpio_to_irq(_chip: *mut gpio_chip, _pin: u32) -> i32 {
    /* TODO: Implement IRQs. Not implemented yet due to weird lockups. */
    -ENXIO
}

static zevio_gpio_chip: gpio_chip = gpio_chip {
    direction_input: Some(zevio_gpio_direction_input),
    direction_output: Some(zevio_gpio_direction_output),
    set: Some(zevio_gpio_set),
    get: Some(zevio_gpio_get),
    to_irq: Some(zevio_gpio_to_irq),
    base: 0,
    owner: THIS_MODULE,
    ngpio: 32,
    ..gpio_chip::default()
};

unsafe extern "C" fn zevio_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let controller = devm_kzalloc(dev, core::mem::size_of::<zevio_gpio>(), GFP_KERNEL)
        as *mut zevio_gpio;
    if controller.is_null() {
        return -ENOMEM;
    }

    (*controller).chip = zevio_gpio_chip;
    (*controller).chip.parent = dev;
    (*controller).chip.label = devm_kasprintf(dev, GFP_KERNEL, "%pfw", dev_fwnode(dev));
    if (*controller).chip.label.is_null() {
        return -ENOMEM;
    }

    (*controller).regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*controller).regs) {
        return dev_err_probe(dev, PTR_ERR((*controller).regs), "failed to ioremap memory resource\n");
    }

    let status = devm_gpiochip_add_data(dev, &mut (*controller).chip, controller.cast());
    if status != 0 {
        dev_err(dev, "failed to add gpiochip: %d\n", status);
        return status;
    }

    spin_lock_init(&mut (*controller).lock);
    let mut i = 0;
    while i < (*controller).chip.ngpio {
        zevio_gpio_port_set(controller, i, ZEVIO_GPIO_INT_MASK, 0xff);
        i += 8;
    }
    dev_dbg((*controller).chip.parent, "ZEVIO GPIO controller set up!\n");
    0
}

static zevio_gpio_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "lsi,zevio-gpio\0".as_ptr() },
    of_device_id::default(),
];

static zevio_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "gpio-zevio\0".as_ptr(),
        of_match_table: zevio_gpio_of_match.as_ptr(),
        suppress_bind_attrs: true,
        ..device_driver::default()
    },
    probe: Some(zevio_gpio_probe),
    ..platform_driver::default()
};

// Equivalent to builtin_platform_driver(zevio_gpio_driver).
builtin_platform_driver!(zevio_gpio_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
