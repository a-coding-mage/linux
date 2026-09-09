// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-mv78xx0/irq.c
 *
 * MV78xx0 IRQ handling.
 */

// External dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

extern "C" {
    static IRQ_VIRT_BASE: *mut c_void;
    static GPIO_VIRT_BASE: *mut c_void;

    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn __fls(word: u32) -> u32;
    fn handle_IRQ(hwirq: u32, regs: *mut pt_regs);
    fn orion_irq_init(irq: u32, mask: *mut c_void);
    fn set_handle_irq(handler: unsafe extern "C" fn(*mut pt_regs));
    fn orion_gpio_init(
        gpio_base: u32,
        ngpio: u32,
        base: *mut c_void,
        irq_offset: u32,
        irq_start: u32,
        irqs: *mut u32,
    );
    fn mv78xx0_core_index() -> u32;
}

extern "C" {
    static IRQ_CAUSE_LOW_OFF: usize;
    static IRQ_MASK_LOW_OFF: usize;
    static IRQ_CAUSE_HIGH_OFF: usize;
    static IRQ_MASK_HIGH_OFF: usize;
    static IRQ_CAUSE_ERR_OFF: usize;
    static IRQ_MASK_ERR_OFF: usize;
    static IRQ_MV78XX0_GPIO_0_7: u32;
    static IRQ_MV78XX0_GPIO_8_15: u32;
    static IRQ_MV78XX0_GPIO_16_23: u32;
    static IRQ_MV78XX0_GPIO_24_31: u32;
    static IRQ_MV78XX0_GPIO_START: u32;
}

static mut gpio0_irqs: [u32; 4] = [
    unsafe { IRQ_MV78XX0_GPIO_0_7 },
    unsafe { IRQ_MV78XX0_GPIO_8_15 },
    unsafe { IRQ_MV78XX0_GPIO_16_23 },
    unsafe { IRQ_MV78XX0_GPIO_24_31 },
];

static mut mv78xx0_irq_base: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn mv78xx0_legacy_handle_irq(regs: *mut pt_regs) {
    let mut stat: u32;

    stat = readl_relaxed(mv78xx0_irq_base.add(IRQ_CAUSE_LOW_OFF));
    stat &= readl_relaxed(mv78xx0_irq_base.add(IRQ_MASK_LOW_OFF));
    if stat != 0 {
        let hwirq: u32 = __fls(stat);
        handle_IRQ(hwirq, regs);
        return;
    }
    stat = readl_relaxed(mv78xx0_irq_base.add(IRQ_CAUSE_HIGH_OFF));
    stat &= readl_relaxed(mv78xx0_irq_base.add(IRQ_MASK_HIGH_OFF));
    if stat != 0 {
        let hwirq: u32 = 32 + __fls(stat);
        handle_IRQ(hwirq, regs);
        return;
    }
    stat = readl_relaxed(mv78xx0_irq_base.add(IRQ_CAUSE_ERR_OFF));
    stat &= readl_relaxed(mv78xx0_irq_base.add(IRQ_MASK_ERR_OFF));
    if stat != 0 {
        let hwirq: u32 = 64 + __fls(stat);
        handle_IRQ(hwirq, regs);
        return;
    }
}

pub unsafe extern "C" fn mv78xx0_init_irq() {
    orion_irq_init(0, IRQ_VIRT_BASE.add(IRQ_MASK_LOW_OFF));
    orion_irq_init(32, IRQ_VIRT_BASE.add(IRQ_MASK_HIGH_OFF));
    orion_irq_init(64, IRQ_VIRT_BASE.add(IRQ_MASK_ERR_OFF));

    set_handle_irq(mv78xx0_legacy_handle_irq);

    /*
     * Initialize gpiolib for GPIOs 0-31.  (The GPIO interrupt mask
     * registers for core #1 are at an offset of 0x18 from those of
     * core #0.)
     */
    orion_gpio_init(
        0,
        32,
        GPIO_VIRT_BASE,
        if mv78xx0_core_index() != 0 { 0x18 } else { 0 },
        IRQ_MV78XX0_GPIO_START,
        gpio0_irqs.as_mut_ptr(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
