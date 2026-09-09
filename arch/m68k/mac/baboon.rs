// SPDX-License-Identifier: GPL-2.0
/*
 * Baboon Custom IC Management
 *
 * The Baboon custom IC controls the IDE, PCMCIA and media bay on the
 * PowerBook 190. It multiplexes multiple interrupt sources onto the
 * Nubus slot $C interrupt.
 */

// C dependencies: linux/types.h, linux/kernel.h, linux/irq.h,
// asm/macintosh.h, asm/macints.h, asm/mac_baboon.h, and mac.h.

#[repr(C)]
pub struct baboon {
    pub mb_ifr: u8,
}

#[repr(C)]
pub struct macintosh_config {
    pub ident: i32,
}

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

extern "C" {
    pub static mut macintosh_config: *mut macintosh_config;

    pub fn generic_handle_irq(irq: i32);
    pub fn irq_set_chained_handler(irq: i32, handler: unsafe extern "C" fn(*mut irq_desc));
    pub fn irq_get_irq_data(irq: i32) -> *mut irq_data;
    pub fn mac_irq_enable(data: *mut irq_data);
    pub fn mac_irq_disable(data: *mut irq_data);
}

pub const MAC_MODEL_PB190: i32 = 0;
pub const BABOON_BASE: usize = 0;
pub const IRQ_BABOON_0: i32 = 0;
pub const IRQ_NUBUS_C: i32 = 0;

pub static mut baboon_present: i32 = 0;
static mut baboon: *mut baboon = core::ptr::null_mut();

/*
 * Baboon initialization.
 */

pub unsafe extern "C" fn baboon_init() {
    if (*macintosh_config).ident != MAC_MODEL_PB190 {
        baboon = core::ptr::null_mut();
        baboon_present = 0;
        return;
    }

    baboon = BABOON_BASE as *mut baboon;
    baboon_present = 1;

    // pr_debug("Baboon detected at %p\n", baboon);
}

/*
 * Baboon interrupt handler.
 * XXX how do you clear a pending IRQ? is it even necessary?
 */

unsafe extern "C" fn baboon_irq(_desc: *mut irq_desc) {
    let mut events: i16 = core::ptr::read_volatile(&(*baboon).mb_ifr as *const u8) as i16 & 0x07;
    let mut irq_num: i32 = IRQ_BABOON_0;
    let mut irq_bit: i16 = 1;
    loop {
        if events & irq_bit != 0 {
            events &= !irq_bit;
            generic_handle_irq(irq_num);
        }
        irq_num += 1;
        irq_bit <<= 1;
        if events == 0 {
            break;
        }
    }
}

/*
 * Register the Baboon interrupt dispatcher on nubus slot $C.
 */

pub unsafe extern "C" fn baboon_register_interrupts() {
    irq_set_chained_handler(IRQ_NUBUS_C, baboon_irq);
}

/*
 * The means for masking individual Baboon interrupts remains a mystery.
 * However, since we only use the IDE IRQ, we can just enable/disable all
 * Baboon interrupts. If/when we handle more than one Baboon IRQ, we must
 * either figure out how to mask them individually or else implement the
 * same workaround that's used for NuBus slots (see nubus_disabled and
 * via_nubus_irq_shutdown).
 */

pub unsafe extern "C" fn baboon_irq_enable(_irq: i32) {
    mac_irq_enable(irq_get_irq_data(IRQ_NUBUS_C));
}

pub unsafe extern "C" fn baboon_irq_disable(_irq: i32) {
    mac_irq_disable(irq_get_irq_data(IRQ_NUBUS_C));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
