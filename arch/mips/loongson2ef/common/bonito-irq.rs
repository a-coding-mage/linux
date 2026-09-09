// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2001 MontaVista Software Inc.
 * Author: Jun Sun, jsun@mvista.com or jsun@junsun.net
 * Copyright (C) 2000, 2001 Ralf Baechle (ralf@gnu.org)
 *
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Declarations supplied by the Linux interrupt/compiler and Loongson headers.
use core::ffi::c_void;

#[repr(C)]
pub struct irq_data {
    pub irq: u32,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

unsafe extern "C" {
    static mut LOONGSON_INTENSET: u32;
    static mut LOONGSON_INTENCLR: u32;
    static LOONGSON_IRQ_BASE: u32;

    fn mmiowb();
    fn irq_set_chip_and_handler(
        irq: u32,
        chip: *mut irq_chip,
        handler: unsafe extern "C" fn(),
    );
    fn handle_level_irq();
    fn request_irq(
        irq: u32,
        handler: unsafe extern "C" fn(),
        flags: u32,
        name: *const u8,
        dev: *mut c_void,
    ) -> i32;
    fn no_action();
    fn pr_err(format: *const u8, ...);
}

unsafe extern "C" fn bonito_irq_enable(d: *mut irq_data) {
    LOONGSON_INTENSET = 1u32 << ((*d).irq.wrapping_sub(LOONGSON_IRQ_BASE));
    mmiowb();
}

unsafe extern "C" fn bonito_irq_disable(d: *mut irq_data) {
    LOONGSON_INTENCLR = 1u32 << ((*d).irq.wrapping_sub(LOONGSON_IRQ_BASE));
    mmiowb();
}

static mut bonito_irq_type: irq_chip = irq_chip {
    name: b"bonito_irq\0".as_ptr(),
    irq_mask: Some(bonito_irq_disable),
    irq_unmask: Some(bonito_irq_enable),
};

pub unsafe extern "C" fn bonito_irq_init() {
    let mut i: u32;

    i = LOONGSON_IRQ_BASE;
    while i < LOONGSON_IRQ_BASE.wrapping_add(32) {
        irq_set_chip_and_handler(i, &raw mut bonito_irq_type, handle_level_irq);
        i = i.wrapping_add(1);
    }

    // Preserved build-time condition: CONFIG_CPU_LOONGSON2E.
    #[cfg(CONFIG_CPU_LOONGSON2E)]
    {
        i = LOONGSON_IRQ_BASE.wrapping_add(10);
        if request_irq(
            i,
            no_action,
            0,
            b"dma_timeout\0".as_ptr(),
            core::ptr::null_mut(),
        ) != 0
        {
            pr_err(b"Failed to request irq %d (dma_timeout)\n\0".as_ptr(), i);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
