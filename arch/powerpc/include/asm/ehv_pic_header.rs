/*
 * EHV_PIC private definitions and structure.
 *
 * Copyright 2008-2010 Freescale Semiconductor, Inc.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty
 * of any kind, whether express or implied.
 */

// Dependency supplied by the Linux IRQ subsystem: `irq_domain` and `irq_chip`.

pub const NR_EHV_PIC_INTS: u32 = 1024;

#[macro_export]
macro_rules! EHV_PIC_INFO {
    ($name:ident) => {
        concat!("EHV_PIC_", stringify!($name))
    };
}

pub const EHV_PIC_VECPRI_POLARITY_NEGATIVE: u32 = 0;
pub const EHV_PIC_VECPRI_POLARITY_POSITIVE: u32 = 1;
pub const EHV_PIC_VECPRI_SENSE_EDGE: u32 = 0;
pub const EHV_PIC_VECPRI_SENSE_LEVEL: u32 = 0x2;
pub const EHV_PIC_VECPRI_POLARITY_MASK: u32 = 0x1;
pub const EHV_PIC_VECPRI_SENSE_MASK: u32 = 0x2;

#[repr(C)]
pub struct ehv_pic {
    /* The remapper for this EHV_PIC */
    pub irqhost: *mut irq_domain,

    /* The "linux" controller struct */
    pub hc_irq: irq_chip,

    /* core int flag */
    pub coreint_flag: ::core::ffi::c_int,
}

unsafe extern "C" {
    pub fn ehv_pic_init();
    pub fn ehv_pic_get_irq() -> ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
