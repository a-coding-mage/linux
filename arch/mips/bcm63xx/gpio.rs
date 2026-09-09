/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 Maxime Bizon <mbizon@freebox.fr>
 * Copyright (C) 2008-2011 Florian Fainelli <florian@openwrt.org>
 */

// Linux and BCM63xx header dependencies are supplied externally.

static mut GPIO_OUT_LOW_REG: u32 = 0;

unsafe fn bcm63xx_gpio_out_low_reg_init() {
    match bcm63xx_get_cpu_id() {
        BCM6345_CPU_ID => GPIO_OUT_LOW_REG = GPIO_DATA_LO_REG_6345,
        _ => GPIO_OUT_LOW_REG = GPIO_DATA_LO_REG,
    }
}

static mut BCM63XX_GPIO_LOCK: SpinLock = DEFINE_SPINLOCK!();
static mut GPIO_OUT_LOW: u32 = 0;
static mut GPIO_OUT_HIGH: u32 = 0;

unsafe fn bcm63xx_gpio_set(chip: *mut gpio_chip, gpio: c_uint, val: c_int) -> c_int {
    let reg: u32;
    let mask: u32;
    let v: *mut u32;
    let mut flags: c_ulong = 0;

    BUG_ON!(gpio >= (*chip).ngpio);

    if gpio < 32 {
        reg = GPIO_OUT_LOW_REG;
        mask = 1u32 << gpio;
        v = &raw mut GPIO_OUT_LOW;
    } else {
        reg = GPIO_DATA_HI_REG;
        mask = 1u32 << (gpio - 32);
        v = &raw mut GPIO_OUT_HIGH;
    }

    spin_lock_irqsave(&raw mut BCM63XX_GPIO_LOCK, &mut flags);
    if val != 0 {
        *v |= mask;
    } else {
        *v &= !mask;
    }
    bcm_gpio_writel(*v, reg);
    spin_unlock_irqrestore(&raw mut BCM63XX_GPIO_LOCK, flags);

    0
}

unsafe fn bcm63xx_gpio_get(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    let reg: u32;
    let mask: u32;

    BUG_ON!(gpio >= (*chip).ngpio);

    if gpio < 32 {
        reg = GPIO_OUT_LOW_REG;
        mask = 1u32 << gpio;
    } else {
        reg = GPIO_DATA_HI_REG;
        mask = 1u32 << (gpio - 32);
    }

    if (bcm_gpio_readl(reg) & mask) != 0 { 1 } else { 0 }
}

unsafe fn bcm63xx_gpio_set_direction(
    chip: *mut gpio_chip,
    gpio: c_uint,
    dir: c_int,
) -> c_int {
    let reg: u32;
    let mask: u32;
    let mut tmp: u32;
    let mut flags: c_ulong = 0;

    BUG_ON!(gpio >= (*chip).ngpio);

    if gpio < 32 {
        reg = GPIO_CTL_LO_REG;
        mask = 1u32 << gpio;
    } else {
        reg = GPIO_CTL_HI_REG;
        mask = 1u32 << (gpio - 32);
    }

    spin_lock_irqsave(&raw mut BCM63XX_GPIO_LOCK, &mut flags);
    tmp = bcm_gpio_readl(reg);
    if dir == BCM63XX_GPIO_DIR_IN {
        tmp &= !mask;
    } else {
        tmp |= mask;
    }
    bcm_gpio_writel(tmp, reg);
    spin_unlock_irqrestore(&raw mut BCM63XX_GPIO_LOCK, flags);

    0
}

unsafe fn bcm63xx_gpio_direction_input(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    bcm63xx_gpio_set_direction(chip, gpio, BCM63XX_GPIO_DIR_IN)
}

unsafe fn bcm63xx_gpio_direction_output(
    chip: *mut gpio_chip,
    gpio: c_uint,
    value: c_int,
) -> c_int {
    bcm63xx_gpio_set(chip, gpio, value);
    bcm63xx_gpio_set_direction(chip, gpio, BCM63XX_GPIO_DIR_OUT)
}

static mut BCM63XX_GPIO_CHIP: gpio_chip = gpio_chip {
    label: b"bcm63xx-gpio\0".as_ptr() as *const c_char,
    direction_input: Some(bcm63xx_gpio_direction_input),
    direction_output: Some(bcm63xx_gpio_direction_output),
    get: Some(bcm63xx_gpio_get),
    set: Some(bcm63xx_gpio_set),
    base: 0,
    ngpio: 0,
};

unsafe fn bcm63xx_gpio_init() -> c_int {
    bcm63xx_gpio_out_low_reg_init();

    GPIO_OUT_LOW = bcm_gpio_readl(GPIO_OUT_LOW_REG);
    if !BCMCPU_IS_6345() {
        GPIO_OUT_HIGH = bcm_gpio_readl(GPIO_DATA_HI_REG);
    }
    BCM63XX_GPIO_CHIP.ngpio = bcm63xx_gpio_count();
    pr_info!("registering {} GPIOs\n", BCM63XX_GPIO_CHIP.ngpio);

    gpiochip_add_data(&raw mut BCM63XX_GPIO_CHIP, core::ptr::null_mut())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
