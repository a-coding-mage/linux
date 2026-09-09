// SPDX-License-Identifier: GPL-2.0
//
// C dependencies: <linux/interrupt.h>, <linux/irq.h>, <asm/traps.h>,
// <asm/apollohw.h>, and "apollo.h".

use core::ffi::c_void;

#[repr(C)]
pub struct irq_data {
    pub irq: u32,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data) -> u32>,
    pub irq_shutdown: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_eoi: Option<unsafe extern "C" fn(*mut irq_data)>,
}

extern "C" {
    static mut pica: *mut c_void;
    static mut picb: *mut c_void;

    static VEC_USER: u32;
    static IRQ_APOLLO: u32;

    fn m68k_setup_user_interrupt(vector: u32, count: u32);
    fn m68k_setup_irq_controller(
        chip: *mut irq_chip,
        handler: unsafe extern "C" fn(*mut irq_data),
        irq: u32,
        count: u32,
    );
    fn handle_fasteoi_irq(data: *mut irq_data);
}

unsafe extern "C" fn apollo_irq_startup(data: *mut irq_data) -> u32 {
    let irq = (*data).irq;

    if irq < 8 {
        let port = (pica as *mut u8).add(1);
        *port &= !(1u8 << irq);
    } else {
        let port = (picb as *mut u8).add(1);
        *port &= !(1u8 << (irq - 8));
    }
    0
}

unsafe extern "C" fn apollo_irq_shutdown(data: *mut irq_data) {
    let irq = (*data).irq;

    if irq < 8 {
        let port = (pica as *mut u8).add(1);
        *port |= 1u8 << irq;
    } else {
        let port = (picb as *mut u8).add(1);
        *port |= 1u8 << (irq - 8);
    }
}

unsafe extern "C" fn apollo_irq_eoi(_data: *mut irq_data) {
    *(pica as *mut u8) = 0x20;
    *(picb as *mut u8) = 0x20;
}

static mut apollo_irq_chip: irq_chip = irq_chip {
    name: b"apollo\0".as_ptr(),
    irq_startup: Some(apollo_irq_startup),
    irq_shutdown: Some(apollo_irq_shutdown),
    irq_eoi: Some(apollo_irq_eoi),
};

// __init
pub unsafe extern "C" fn dn_init_IRQ() {
    m68k_setup_user_interrupt(VEC_USER + 96, 16);
    m68k_setup_irq_controller(
        &mut apollo_irq_chip,
        handle_fasteoi_irq,
        IRQ_APOLLO,
        16,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
