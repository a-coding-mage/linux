// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-dove/irq.c
 *
 * Dove IRQ handling.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    static IRQ_VIRT_BASE: usize;
    static IRQ_CAUSE_LOW_OFF: usize;
    static IRQ_MASK_LOW_OFF: usize;
    static IRQ_CAUSE_HIGH_OFF: usize;
    static IRQ_MASK_HIGH_OFF: usize;
    static DOVE_GPIO_LO_VIRT_BASE: usize;
    static DOVE_GPIO_HI_VIRT_BASE: usize;
    static DOVE_GPIO2_VIRT_BASE: usize;
    static IRQ_DOVE_GPIO_0_7: i32;
    static IRQ_DOVE_GPIO_8_15: i32;
    static IRQ_DOVE_GPIO_16_23: i32;
    static IRQ_DOVE_GPIO_24_31: i32;
    static IRQ_DOVE_HIGH_GPIO: i32;
    static IRQ_DOVE_GPIO_START: i32;

    fn readl_relaxed(addr: *const c_void) -> u32;
    fn __fls(word: u32) -> u32;
    fn handle_IRQ(hwirq: u32, regs: *mut pt_regs);
    fn orion_irq_init(first_hwirq: u32, mask_addr: usize);
    fn set_handle_irq(handler: unsafe extern "C" fn(*mut pt_regs));
    fn orion_gpio_init(
        gpio_base: u32,
        ngpio: u32,
        base: usize,
        secondary_irq_base: u32,
        irq_base: i32,
        irqs: *const i32,
    );
}

static mut gpio0_irqs: [i32; 4] = [
    unsafe { IRQ_DOVE_GPIO_0_7 },
    unsafe { IRQ_DOVE_GPIO_8_15 },
    unsafe { IRQ_DOVE_GPIO_16_23 },
    unsafe { IRQ_DOVE_GPIO_24_31 },
];

static mut gpio1_irqs: [i32; 4] = [unsafe { IRQ_DOVE_HIGH_GPIO }, 0, 0, 0];

static mut gpio2_irqs: [i32; 4] = [0, 0, 0, 0];

static mut dove_irq_base: *mut u8 = unsafe { IRQ_VIRT_BASE as *mut u8 };

// `asmlinkage` and `__exception_irq_entry` are architecture-specific C attributes.
unsafe extern "C" fn dove_legacy_handle_irq(regs: *mut pt_regs) {
    let mut stat: u32;

    stat = unsafe {
        readl_relaxed(
            dove_irq_base
                .add(IRQ_CAUSE_LOW_OFF)
                .cast::<c_void>(),
        )
    };
    stat &= unsafe {
        readl_relaxed(
            dove_irq_base
                .add(IRQ_MASK_LOW_OFF)
                .cast::<c_void>(),
        )
    };
    if stat != 0 {
        let hwirq: u32 = 1 + unsafe { __fls(stat) };
        unsafe { handle_IRQ(hwirq, regs) };
        return;
    }
    stat = unsafe {
        readl_relaxed(
            dove_irq_base
                .add(IRQ_CAUSE_HIGH_OFF)
                .cast::<c_void>(),
        )
    };
    stat &= unsafe {
        readl_relaxed(
            dove_irq_base
                .add(IRQ_MASK_HIGH_OFF)
                .cast::<c_void>(),
        )
    };
    if stat != 0 {
        let hwirq: u32 = 33 + unsafe { __fls(stat) };
        unsafe { handle_IRQ(hwirq, regs) };
        return;
    }
}

pub unsafe extern "C" fn dove_init_irq() {
    orion_irq_init(1, IRQ_VIRT_BASE + IRQ_MASK_LOW_OFF);
    orion_irq_init(33, IRQ_VIRT_BASE + IRQ_MASK_HIGH_OFF);

    set_handle_irq(dove_legacy_handle_irq);

    /*
     * Initialize gpiolib for GPIOs 0-71.
     */
    orion_gpio_init(
        0,
        32,
        DOVE_GPIO_LO_VIRT_BASE,
        0,
        IRQ_DOVE_GPIO_START,
        gpio0_irqs.as_ptr(),
    );

    orion_gpio_init(
        32,
        32,
        DOVE_GPIO_HI_VIRT_BASE,
        0,
        IRQ_DOVE_GPIO_START + 32,
        gpio1_irqs.as_ptr(),
    );

    orion_gpio_init(
        64,
        8,
        DOVE_GPIO2_VIRT_BASE,
        0,
        IRQ_DOVE_GPIO_START + 64,
        gpio2_irqs.as_ptr(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
