// SPDX-License-Identifier: MIT
/*
 * Copyright (C) 2021 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding kernel translation unit.

unsafe fn to_dal_irq_source_dcn303(
    irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
    let _ = irq_service;
    let _ = ext_id;
    match src_id {
        DCN_1_0__SRCID__DC_D1_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK1,
        DCN_1_0__SRCID__DC_D2_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK2,
        DCN_1_0__SRCID__OTG1_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC1_VLINE0,
        DCN_1_0__SRCID__OTG2_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC2_VLINE0,
        DCN_1_0__SRCID__HUBP0_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP1,
        DCN_1_0__SRCID__HUBP1_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP2,
        DCN_1_0__SRCID__OTG0_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE1,
        DCN_1_0__SRCID__OTG1_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE2,
        DCN_1_0__SRCID__DC_HPD1_INT => match ext_id {
            DCN_1_0__CTXID__DC_HPD1_INT => DC_IRQ_SOURCE_HPD1,
            DCN_1_0__CTXID__DC_HPD2_INT => DC_IRQ_SOURCE_HPD2,
            DCN_1_0__CTXID__DC_HPD1_RX_INT => DC_IRQ_SOURCE_HPD1RX,
            DCN_1_0__CTXID__DC_HPD2_RX_INT => DC_IRQ_SOURCE_HPD2RX,
            _ => DC_IRQ_SOURCE_INVALID,
        },
        _ => DC_IRQ_SOURCE_INVALID,
    }
}

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None,
    ack: Some(hpd0_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vupdate_no_lock_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline0_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set),
    ack: Some(dal_irq_service_dummy_ack),
};

macro_rules! base { ($seg:ident) => { DCN_BASE__INST0_SEG$seg }; }
macro_rules! sri { ($reg_name:ident, $block:ident, $id:literal) => {
    base!($id) + mm$block$id_$reg_name##_BASE_IDX + mm$block$id_$reg_name
}; }
macro_rules! irq_reg_entry { ($block:ident, $num:literal, $reg1:ident, $mask1:ident, $reg2:ident, $mask2:ident) => {
    enable_reg: sri!($reg1, $block, $num),
    enable_mask: $block$num_$reg1__$mask1_MASK,
    enable_value: [$block$num_$reg1__$mask1_MASK, !($block$num_$reg1__$mask1_MASK)],
    ack_reg: sri!($reg2, $block, $num),
    ack_mask: $block$num_$reg2__$mask2_MASK,
    ack_value: $block$num_$reg2__$mask2_MASK,
}; }

macro_rules! hpd_int_entry { ($n:literal) => { irq_source_info { ..irq_source_info::DEFAULT } }; }
macro_rules! hpd_rx_int_entry { ($n:literal) => { irq_source_info { ..irq_source_info::DEFAULT } }; }
macro_rules! pflip_int_entry { ($n:literal) => { irq_source_info { ..irq_source_info::DEFAULT } }; }
macro_rules! vupdate_no_lock_int_entry { ($n:literal) => { irq_source_info { ..irq_source_info::DEFAULT } }; }
macro_rules! vblank_int_entry { ($n:literal) => { irq_source_info { ..irq_source_info::DEFAULT } }; }
macro_rules! vline0_int_entry { ($n:literal) => { irq_source_info { ..irq_source_info::DEFAULT } }; }
macro_rules! dummy_irq_entry { () => { irq_source_info { ..irq_source_info::DEFAULT } }; }

// The following table retains the C designated-entry layout; register and
// source constants are provided by the translated dependency headers.
static irq_source_info_dcn303: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    dummy_irq_entry!(), hpd_int_entry!(0), hpd_int_entry!(1),
    hpd_rx_int_entry!(0), hpd_rx_int_entry!(1),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    pflip_int_entry!(0), pflip_int_entry!(1), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    vupdate_no_lock_int_entry!(0), vupdate_no_lock_int_entry!(1),
    vblank_int_entry!(0), vblank_int_entry!(1),
    vline0_int_entry!(0), vline0_int_entry!(1),
];

static irq_service_funcs_dcn303: irq_service_funcs = irq_service_funcs {
    to_dal_irq_source: Some(to_dal_irq_source_dcn303),
};

unsafe fn dcn303_irq_construct(
    irq_service: *mut irq_service,
    init_data: *mut irq_service_init_data,
) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = &irq_source_info_dcn303;
    (*irq_service).funcs = &irq_service_funcs_dcn303;
}

unsafe fn dal_irq_service_dcn303_create(
    init_data: *mut irq_service_init_data,
) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() {
        return core::ptr::null_mut();
    }
    dcn303_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
