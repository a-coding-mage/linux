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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding translation unit.

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None,
    ack: Some(hpd1_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dce110_vblank_set), ack: None,
};
static mut vupdate_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };

macro_rules! hpd_int_entry {
    ($n:expr) => { irq_source_info { enable_reg: mmDC_HPD1_INT_CONTROL + $n - 1, enable_mask: DC_HPD1_INT_CONTROL__DC_HPD1_INT_EN_MASK, enable_value: [DC_HPD1_INT_CONTROL__DC_HPD1_INT_EN_MASK, !DC_HPD1_INT_CONTROL__DC_HPD1_INT_EN_MASK], ack_reg: mmDC_HPD1_INT_CONTROL + $n - 1, ack_mask: DC_HPD1_INT_CONTROL__DC_HPD1_INT_ACK_MASK, ack_value: DC_HPD1_INT_CONTROL__DC_HPD1_INT_ACK_MASK, status_reg: mmDC_HPD1_INT_STATUS + $n - 1, funcs: unsafe { &mut hpd_irq_info_funcs }, ..Default::default() } }
}
macro_rules! hpd_rx_int_entry {
    ($n:expr) => { irq_source_info { enable_reg: mmDC_HPD1_INT_CONTROL + $n - 1, enable_mask: DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_EN_MASK, enable_value: [DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_EN_MASK, !DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_EN_MASK], ack_reg: mmDC_HPD1_INT_CONTROL + $n - 1, ack_mask: DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_ACK_MASK, ack_value: DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_ACK_MASK, status_reg: mmDC_HPD1_INT_STATUS + $n - 1, funcs: unsafe { &mut hpd_rx_irq_info_funcs }, ..Default::default() } }
}
macro_rules! pflip_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmDCP1_GRPH_INTERRUPT_CONTROL + $n, enable_mask: GRPH_INTERRUPT_CONTROL__GRPH_PFLIP_INT_MASK_MASK, enable_value: [GRPH_INTERRUPT_CONTROL__GRPH_PFLIP_INT_MASK_MASK, !GRPH_INTERRUPT_CONTROL__GRPH_PFLIP_INT_MASK_MASK], ack_reg: mmDCP1_GRPH_INTERRUPT_STATUS + $n, ack_mask: GRPH_INTERRUPT_STATUS__GRPH_PFLIP_INT_CLEAR_MASK, ack_value: GRPH_INTERRUPT_STATUS__GRPH_PFLIP_INT_CLEAR_MASK, status_reg: mmDCP1_GRPH_INTERRUPT_STATUS + $n, funcs: unsafe { &mut pflip_irq_info_funcs }, ..Default::default() } }; }
macro_rules! vupdate_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmCRTC1_CRTC_INTERRUPT_CONTROL + $n, enable_mask: CRTC_INTERRUPT_CONTROL__CRTC_V_UPDATE_INT_MSK_MASK, enable_value: [CRTC_INTERRUPT_CONTROL__CRTC_V_UPDATE_INT_MSK_MASK, !CRTC_INTERRUPT_CONTROL__CRTC_V_UPDATE_INT_MSK_MASK], ack_reg: mmCRTC1_CRTC_V_UPDATE_INT_STATUS + $n, ack_mask: CRTC_V_UPDATE_INT_STATUS__CRTC_V_UPDATE_INT_CLEAR_MASK, ack_value: CRTC_V_UPDATE_INT_STATUS__CRTC_V_UPDATE_INT_CLEAR_MASK, funcs: unsafe { &mut vupdate_irq_info_funcs }, ..Default::default() } }; }
macro_rules! vblank_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmCRTC1_CRTC_VERTICAL_INTERRUPT0_CONTROL + $n, enable_mask: CRTC_VERTICAL_INTERRUPT0_CONTROL__CRTC_VERTICAL_INTERRUPT0_INT_ENABLE_MASK, enable_value: [CRTC_VERTICAL_INTERRUPT0_CONTROL__CRTC_VERTICAL_INTERRUPT0_INT_ENABLE_MASK, !CRTC_VERTICAL_INTERRUPT0_CONTROL__CRTC_VERTICAL_INTERRUPT0_INT_ENABLE_MASK], ack_reg: mmCRTC1_CRTC_VERTICAL_INTERRUPT0_CONTROL + $n, ack_mask: CRTC_VERTICAL_INTERRUPT0_CONTROL__CRTC_VERTICAL_INTERRUPT0_CLEAR_MASK, ack_value: CRTC_VERTICAL_INTERRUPT0_CONTROL__CRTC_VERTICAL_INTERRUPT0_CLEAR_MASK, funcs: unsafe { &mut vblank_irq_info_funcs }, src_id: VISLANDS30_IV_SRCID_D1_VERTICAL_INTERRUPT0 + $n, ..Default::default() } }; }
macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: unsafe { &mut dummy_irq_info_funcs }, ..Default::default() } }; }

macro_rules! repeated_dummy { ($($x:expr),* $(,)?) => { $($x),* }; }
static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack),
};

// The C designated initializers are represented in source order; register and
// source constants retain the original dependency-provided meanings.
static irq_source_info_dce80: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    dummy_irq_entry!(),
    hpd_int_entry!(1), hpd_int_entry!(2), hpd_int_entry!(3), hpd_int_entry!(4), hpd_int_entry!(5), hpd_int_entry!(6),
    hpd_rx_int_entry!(1), hpd_rx_int_entry!(2), hpd_rx_int_entry!(3), hpd_rx_int_entry!(4), hpd_rx_int_entry!(5), hpd_rx_int_entry!(6),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(),
    pflip_int_entry!(0), pflip_int_entry!(1), pflip_int_entry!(2), pflip_int_entry!(3), pflip_int_entry!(4), pflip_int_entry!(5),
    dummy_irq_entry!(),
    repeated_dummy!(dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!()),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(),
    vupdate_int_entry!(0), vupdate_int_entry!(1), vupdate_int_entry!(2), vupdate_int_entry!(3), vupdate_int_entry!(4), vupdate_int_entry!(5),
    vblank_int_entry!(0), vblank_int_entry!(1), vblank_int_entry!(2), vblank_int_entry!(3), vblank_int_entry!(4), vblank_int_entry!(5),
];

static irq_service_funcs_dce80: irq_service_funcs = irq_service_funcs { to_dal_irq_source: Some(to_dal_irq_source_dce110) };

unsafe fn dce80_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = &irq_source_info_dce80 as *const _;
    (*irq_service).funcs = &irq_service_funcs_dce80;
}

#[no_mangle]
pub unsafe extern "C" fn dal_irq_service_dce80_create(init_data: *mut irq_service_init_data) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dce80_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
