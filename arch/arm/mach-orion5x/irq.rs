// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/irq.c
 *
 * Core IRQ functions for Marvell Orion System On Chip
 *
 * Maintainer: Tzachi Perelstein <tzachi@marvell.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/gpio/consumer.h, linux/kernel.h, linux/irq.h, linux/io.h,
// plat/orion-gpio.h, plat/irq.h, asm/exception.h, bridge-regs.h, common.h

extern "C" {
    pub type pt_regs;

    fn readl_relaxed(addr: *const core::ffi::c_void) -> u32;
    fn __fls(word: u32) -> u32;
    fn handle_IRQ(hwirq: u32, regs: *mut pt_regs);
    fn set_handle_irq(handler: unsafe extern "C" fn(*mut pt_regs));
    fn orion_irq_init(first: u32, mask: *const core::ffi::c_void);
    fn orion_gpio_init(
        gpio_base: u32,
        count: u32,
        virt_base: *const core::ffi::c_void,
        irq_base: u32,
        irq_start: u32,
        gpio_irqs: *const i32,
    );
}

extern "C" {
    static MAIN_IRQ_CAUSE: *const core::ffi::c_void;
    static MAIN_IRQ_MASK: *const core::ffi::c_void;
    static GPIO_VIRT_BASE: *const core::ffi::c_void;
}

// __initdata
static mut gpio0_irqs: [i32; 4] = [
    IRQ_ORION5X_GPIO_0_7,
    IRQ_ORION5X_GPIO_8_15,
    IRQ_ORION5X_GPIO_16_23,
    IRQ_ORION5X_GPIO_24_31,
];

unsafe extern "C" fn orion5x_legacy_handle_irq(regs: *mut pt_regs) {
    let mut stat: u32;

    stat = readl_relaxed(MAIN_IRQ_CAUSE);
    stat &= readl_relaxed(MAIN_IRQ_MASK);
    if stat != 0 {
        let hwirq: u32 = 1 + __fls(stat);
        handle_IRQ(hwirq, regs);
        return;
    }
}

pub unsafe extern "C" fn orion5x_init_irq() {
    orion_irq_init(1, MAIN_IRQ_MASK);

    set_handle_irq(orion5x_legacy_handle_irq);

    /*
     * Initialize gpiolib for GPIOs 0-31.
     */
    orion_gpio_init(
        0,
        32,
        GPIO_VIRT_BASE,
        0,
        IRQ_ORION5X_GPIO_START,
        gpio0_irqs.as_ptr(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
