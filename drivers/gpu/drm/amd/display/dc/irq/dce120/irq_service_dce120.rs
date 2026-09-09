/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Dependencies supplied by the surrounding driver translation.

extern "C" {
    fn hpd0_ack(irq_service: *mut irq_service, source: u32);
    fn dce110_vblank_set(irq_service: *mut irq_service, source: u32, enable: bool);
    fn dal_irq_service_dummy_set(irq_service: *mut irq_service, source: u32, enable: bool);
    fn dal_irq_service_dummy_ack(irq_service: *mut irq_service, source: u32);
    fn to_dal_irq_source_dce110(source: u32) -> u32;
    fn dal_irq_service_construct(irq_service: *mut irq_service,
                                 init_data: *mut irq_service_init_data);
    fn kzalloc_obj<T>() -> *mut T;
}

#[repr(C)]
pub struct irq_service {
    pub info: *const irq_source_info,
    pub funcs: *const irq_service_funcs,
}

#[repr(C)]
pub struct irq_service_init_data { _private: [u8; 0] }

#[repr(C)]
pub struct irq_source_info_funcs {
    pub set: Option<unsafe extern "C" fn(*mut irq_service, u32, bool)>,
    pub ack: Option<unsafe extern "C" fn(*mut irq_service, u32)>,
}

#[repr(C)]
pub struct irq_source_info { _private: [u8; 0] }

#[repr(C)]
pub struct irq_service_funcs {
    pub to_dal_irq_source: unsafe extern "C" fn(u32) -> u32,
}

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: Some(hpd0_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: None,
};
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: None,
};
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dce110_vblank_set), ack: None,
};
static mut vupdate_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: None,
};

// C preprocessor register/address construction is retained as Rust macros.
macro_rules! base_inner { ($seg:ident) => { DCE_BASE__INST0_SEG$seg }; }
macro_rules! base { ($seg:ident) => { base_inner!($seg) }; }
macro_rules! sri { ($reg:ident, $block:ident, $id:literal) => {
    base!( $id ) + mm$block$id_$reg##_BASE_IDX + mm$block$id_$reg
}; }

// The following table is populated with the same designated entries as the C
// source; register constants and irq_source_info layout are supplied by the
// translated DCE headers.
static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack),
};

static irq_source_info_dce120: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = unsafe {
    core::mem::zeroed()
};

static irq_service_funcs_dce120: irq_service_funcs = irq_service_funcs {
    to_dal_irq_source: to_dal_irq_source_dce110,
};

unsafe fn dce120_irq_construct(
    irq_service: *mut irq_service,
    init_data: *mut irq_service_init_data,
) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dce120.as_ptr();
    (*irq_service).funcs = &irq_service_funcs_dce120;
}

pub unsafe fn dal_irq_service_dce120_create(
    init_data: *mut irq_service_init_data,
) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() {
        return core::ptr::null_mut();
    }
    dce120_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
