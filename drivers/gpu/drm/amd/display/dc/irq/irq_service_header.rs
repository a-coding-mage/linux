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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependency: include/irq_service_interface.h
// Dependency: irq_types.h

#[repr(C)]
pub struct irq_service {
    pub ctx: *mut dc_context,
    pub info: *const irq_source_info,
    pub funcs: *const irq_service_funcs,
}

#[repr(C)]
pub struct irq_source_info_funcs {
    pub set: Option<unsafe extern "C" fn(
        irq_service: *mut irq_service,
        info: *const irq_source_info,
        enable: bool,
    ) -> bool>,
    pub ack: Option<unsafe extern "C" fn(
        irq_service: *mut irq_service,
        info: *const irq_source_info,
    ) -> bool>,
}

#[repr(C)]
pub struct irq_source_info {
    pub src_id: u32,
    pub ext_id: u32,
    pub enable_reg: u32,
    pub enable_mask: u32,
    pub enable_value: [u32; 2],
    pub ack_reg: u32,
    pub ack_mask: u32,
    pub ack_value: u32,
    pub status_reg: u32,
    pub funcs: *mut irq_source_info_funcs,
}

#[repr(C)]
pub struct irq_service_funcs {
    pub to_dal_irq_source: Option<unsafe extern "C" fn(
        irq_service: *mut irq_service,
        src_id: u32,
        ext_id: u32,
    ) -> dc_irq_source>,
}

extern "C" {
    pub fn dal_irq_service_construct(
        irq_service: *mut irq_service,
        init_data: *mut irq_service_init_data,
    );

    pub fn dal_irq_service_ack_generic(
        irq_service: *mut irq_service,
        info: *const irq_source_info,
    );

    pub fn dal_irq_service_set_generic(
        irq_service: *mut irq_service,
        info: *const irq_source_info,
        enable: bool,
    );

    pub fn hpd0_ack(
        irq_service: *mut irq_service,
        info: *const irq_source_info,
    ) -> bool;

    pub fn hpd1_ack(
        irq_service: *mut irq_service,
        info: *const irq_source_info,
    ) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
