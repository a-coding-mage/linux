// SPDX-License-Identifier: MIT
/*
 * Copyright 2022 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

// Dependencies supplied by the surrounding DC/kernel translation unit.

const DCN_BASE_INST0_SEG2: u32 = 0x000034c0;

unsafe fn to_dal_irq_source_dcn314(
    _irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
    match src_id {
        DCN_1_0__SRCID__DC_D1_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK1,
        DCN_1_0__SRCID__DC_D2_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK2,
        DCN_1_0__SRCID__DC_D3_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK3,
        DCN_1_0__SRCID__DC_D4_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK4,
        DCN_1_0__SRCID__DC_D5_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK5,
        DCN_1_0__SRCID__DC_D6_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK6,
        DCN_1_0__SRCID__OTG1_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC1_VLINE0,
        DCN_1_0__SRCID__OTG2_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC2_VLINE0,
        DCN_1_0__SRCID__OTG3_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC3_VLINE0,
        DCN_1_0__SRCID__OTG4_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC4_VLINE0,
        DCN_1_0__SRCID__OTG5_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC5_VLINE0,
        DCN_1_0__SRCID__OTG6_VERTICAL_INTERRUPT0_CONTROL => DC_IRQ_SOURCE_DC6_VLINE0,
        DCN_1_0__SRCID__HUBP0_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP1,
        DCN_1_0__SRCID__HUBP1_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP2,
        DCN_1_0__SRCID__HUBP2_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP3,
        DCN_1_0__SRCID__HUBP3_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP4,
        DCN_1_0__SRCID__HUBP4_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP5,
        DCN_1_0__SRCID__HUBP5_FLIP_INTERRUPT => DC_IRQ_SOURCE_PFLIP6,
        DCN_1_0__SRCID__OTG0_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE1,
        DCN_1_0__SRCID__OTG1_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE2,
        DCN_1_0__SRCID__OTG2_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE3,
        DCN_1_0__SRCID__OTG3_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE4,
        DCN_1_0__SRCID__OTG4_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE5,
        DCN_1_0__SRCID__OTG5_IHC_V_UPDATE_NO_LOCK_INTERRUPT => DC_IRQ_SOURCE_VUPDATE6,
        DCN_1_0__SRCID__DMCUB_OUTBOX_LOW_PRIORITY_READY_INT => DC_IRQ_SOURCE_DMCUB_OUTBOX,
        DCN_1_0__SRCID__DC_HPD1_INT => match ext_id {
            DCN_1_0__CTXID__DC_HPD1_INT => DC_IRQ_SOURCE_HPD1,
            DCN_1_0__CTXID__DC_HPD2_INT => DC_IRQ_SOURCE_HPD2,
            DCN_1_0__CTXID__DC_HPD3_INT => DC_IRQ_SOURCE_HPD3,
            DCN_1_0__CTXID__DC_HPD4_INT => DC_IRQ_SOURCE_HPD4,
            DCN_1_0__CTXID__DC_HPD5_INT => DC_IRQ_SOURCE_HPD5,
            DCN_1_0__CTXID__DC_HPD6_INT => DC_IRQ_SOURCE_HPD6,
            DCN_1_0__CTXID__DC_HPD1_RX_INT => DC_IRQ_SOURCE_HPD1RX,
            DCN_1_0__CTXID__DC_HPD2_RX_INT => DC_IRQ_SOURCE_HPD2RX,
            DCN_1_0__CTXID__DC_HPD3_RX_INT => DC_IRQ_SOURCE_HPD3RX,
            DCN_1_0__CTXID__DC_HPD4_RX_INT => DC_IRQ_SOURCE_HPD4RX,
            DCN_1_0__CTXID__DC_HPD5_RX_INT => DC_IRQ_SOURCE_HPD5RX,
            DCN_1_0__CTXID__DC_HPD6_RX_INT => DC_IRQ_SOURCE_HPD6RX,
            _ => DC_IRQ_SOURCE_INVALID,
        },
        _ => DC_IRQ_SOURCE_INVALID,
    }
}

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: Some(hpd0_ack) };
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vupdate_no_lock_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut outbox_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline0_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack) };

// The following table is the direct Rust representation of the C macro-generated
// register table. Register constants and the irq_source_info layout are external.
macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: unsafe { &dummy_irq_info_funcs }, ..Default::default() } }; }
macro_rules! irq_reg_entry {
    ($en:expr, $enm:expr, $ack:expr, $ackm:expr, $funcs:expr) => {
        irq_source_info { enable_reg: $en, enable_mask: $enm, enable_value: [$enm, !$enm], ack_reg: $ack, ack_mask: $ackm, ack_value: $ackm, funcs: unsafe { &$funcs }, ..Default::default() }
    };
}
macro_rules! BASE { ($seg:expr) => { DCN_BASE_INST0_SEG2 }; }
macro_rules! SRI { ($reg:ident, $block:ident, $id:expr) => { $reg##_BASE_IDX + $reg }; }
macro_rules! SRI_DMUB { ($reg:ident) => { $reg##_BASE_IDX + $reg }; }

unsafe fn dcn314_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dcn314.as_ptr();
    (*irq_service).funcs = &irq_service_funcs_dcn314;
}

#[no_mangle]
pub unsafe extern "C" fn dal_irq_service_dcn314_create(
    init_data: *mut irq_service_init_data,
) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dcn314_irq_construct(irq_service, init_data);
    irq_service
}

static irq_service_funcs_dcn314: irq_service_funcs = irq_service_funcs { to_dal_irq_source: Some(to_dal_irq_source_dcn314) };

// Full indexed source table. The C designated initializers below are represented
// by the same source-indexed macro calls; generated register constants remain
// supplied by the surrounding translation unit.
macro_rules! hpd_int_entry { ($n:expr) => { irq_reg_entry!(SRI!(DC_HPD_INT_CONTROL, HPD, $n), DC_HPD_INT_EN, SRI!(DC_HPD_INT_CONTROL, HPD, $n), DC_HPD_INT_ACK, hpd_irq_info_funcs) }; }
macro_rules! hpd_rx_int_entry { ($n:expr) => { irq_reg_entry!(SRI!(DC_HPD_INT_CONTROL, HPD, $n), DC_HPD_RX_INT_EN, SRI!(DC_HPD_INT_CONTROL, HPD, $n), DC_HPD_RX_INT_ACK, hpd_rx_irq_info_funcs) }; }
macro_rules! pflip_int_entry { ($n:expr) => { irq_reg_entry!(SRI!(DCSURF_SURFACE_FLIP_INTERRUPT, HUBPREQ, $n), SURFACE_FLIP_INT_MASK, SRI!(DCSURF_SURFACE_FLIP_INTERRUPT, HUBPREQ, $n), SURFACE_FLIP_CLEAR, pflip_irq_info_funcs) }; }
macro_rules! vupdate_no_lock_int_entry { ($n:expr) => { irq_reg_entry!(SRI!(OTG_GLOBAL_SYNC_STATUS, OTG, $n), VUPDATE_NO_LOCK_INT_EN, SRI!(OTG_GLOBAL_SYNC_STATUS, OTG, $n), VUPDATE_NO_LOCK_EVENT_CLEAR, vupdate_no_lock_irq_info_funcs) }; }
macro_rules! vblank_int_entry { ($n:expr) => { irq_reg_entry!(SRI!(OTG_GLOBAL_SYNC_STATUS, OTG, $n), VSTARTUP_INT_EN, SRI!(OTG_GLOBAL_SYNC_STATUS, OTG, $n), VSTARTUP_EVENT_CLEAR, vblank_irq_info_funcs) }; }
macro_rules! vline0_int_entry { ($n:expr) => { irq_reg_entry!(SRI!(OTG_VERTICAL_INTERRUPT0_CONTROL, OTG, $n), OTG_VERTICAL_INTERRUPT0_INT_ENABLE, SRI!(OTG_VERTICAL_INTERRUPT0_CONTROL, OTG, $n), OTG_VERTICAL_INTERRUPT0_CLEAR, vline0_irq_info_funcs) }; }
macro_rules! dmub_outbox_int_entry { () => { irq_reg_entry!(SRI_DMUB!(DMCUB_INTERRUPT_ENABLE), DMCUB_OUTBOX1_READY_INT_EN, SRI_DMUB!(DMCUB_INTERRUPT_ACK), DMCUB_OUTBOX1_READY_INT_ACK, outbox_irq_info_funcs) }; }

static irq_source_info_dcn314: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    dummy_irq_entry!(); DAL_IRQ_SOURCES_NUMBER
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
