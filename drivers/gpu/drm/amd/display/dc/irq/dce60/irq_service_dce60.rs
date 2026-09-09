/*
 * Copyright 2020 Mauro Rossi <issor.oruam@gmail.com>
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

// C dependencies supplied by the surrounding kernel/display implementation.

const VISLANDS30_IV_SRCID_D1_VBLANK: u32 = 1;
const VISLANDS30_IV_SRCID_D2_VBLANK: u32 = 2;
const VISLANDS30_IV_SRCID_D3_VBLANK: u32 = 3;
const VISLANDS30_IV_SRCID_D4_VBLANK: u32 = 4;
const VISLANDS30_IV_SRCID_D5_VBLANK: u32 = 5;
const VISLANDS30_IV_SRCID_D6_VBLANK: u32 = 6;

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: Some(hpd1_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: Some(dce110_vblank_set), ack: None };
static mut vblank_irq_info_funcs_dce60: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };

macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: &dummy_irq_info_funcs, ..Default::default() } }; }
macro_rules! hpd_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmDC_HPD1_INT_CONTROL + $n - 1, enable_mask: DC_HPD1_INT_CONTROL__DC_HPD1_INT_EN_MASK, enable_value: [DC_HPD1_INT_CONTROL__DC_HPD1_INT_EN_MASK, !DC_HPD1_INT_CONTROL__DC_HPD1_INT_EN_MASK], ack_reg: mmDC_HPD1_INT_CONTROL + $n - 1, ack_mask: DC_HPD1_INT_CONTROL__DC_HPD1_INT_ACK_MASK, ack_value: DC_HPD1_INT_CONTROL__DC_HPD1_INT_ACK_MASK, status_reg: mmDC_HPD1_INT_STATUS + $n - 1, funcs: &hpd_irq_info_funcs } }; }
macro_rules! hpd_rx_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmDC_HPD1_INT_CONTROL + $n - 1, enable_mask: DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_EN_MASK, enable_value: [DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_EN_MASK, !DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_EN_MASK], ack_reg: mmDC_HPD1_INT_CONTROL + $n - 1, ack_mask: DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_ACK_MASK, ack_value: DC_HPD1_INT_CONTROL__DC_HPD1_RX_INT_ACK_MASK, status_reg: mmDC_HPD1_INT_STATUS + $n - 1, funcs: &hpd_rx_irq_info_funcs } }; }
macro_rules! pflip_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmDCP0_GRPH_INTERRUPT_CONTROL + $n, enable_mask: GRPH_INTERRUPT_CONTROL__GRPH_PFLIP_INT_MASK_MASK, enable_value: [GRPH_INTERRUPT_CONTROL__GRPH_PFLIP_INT_MASK_MASK, !GRPH_INTERRUPT_CONTROL__GRPH_PFLIP_INT_MASK_MASK], ack_reg: mmDCP0_GRPH_INTERRUPT_STATUS + $n, ack_mask: GRPH_INTERRUPT_STATUS__GRPH_PFLIP_INT_CLEAR_MASK, ack_value: GRPH_INTERRUPT_STATUS__GRPH_PFLIP_INT_CLEAR_MASK, status_reg: mmDCP0_GRPH_INTERRUPT_STATUS + $n, funcs: &pflip_irq_info_funcs } }; }
macro_rules! vupdate_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmCRTC0_CRTC_INTERRUPT_CONTROL + $n, enable_mask: CRTC_INTERRUPT_CONTROL__CRTC_V_UPDATE_INT_MSK_MASK, enable_value: [CRTC_INTERRUPT_CONTROL__CRTC_V_UPDATE_INT_MSK_MASK, !CRTC_INTERRUPT_CONTROL__CRTC_V_UPDATE_INT_MSK_MASK], ack_reg: mmCRTC0_CRTC_V_UPDATE_INT_STATUS + $n, ack_mask: CRTC_V_UPDATE_INT_STATUS__CRTC_V_UPDATE_INT_CLEAR_MASK, ack_value: CRTC_V_UPDATE_INT_STATUS__CRTC_V_UPDATE_INT_CLEAR_MASK, funcs: &vblank_irq_info_funcs } }; }
macro_rules! vblank_int_entry { ($n:expr) => { irq_source_info { enable_reg: mmLB0_INT_MASK + $n, enable_mask: INT_MASK__VBLANK_INT_MASK, enable_value: [INT_MASK__VBLANK_INT_MASK, !INT_MASK__VBLANK_INT_MASK], ack_reg: mmLB0_VBLANK_STATUS + $n, ack_mask: VBLANK_STATUS__VBLANK_ACK_MASK, ack_value: VBLANK_STATUS__VBLANK_ACK_MASK, funcs: &vblank_irq_info_funcs_dce60 } }; }

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack) };

static irq_source_info_dce60: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    /* The C designated initializers are retained as indexed entries below. */
    [DC_IRQ_SOURCE_INVALID] = dummy_irq_entry!(),
    hpd_int_entry!(1), hpd_int_entry!(2), hpd_int_entry!(3), hpd_int_entry!(4), hpd_int_entry!(5), hpd_int_entry!(6),
    hpd_rx_int_entry!(1), hpd_rx_int_entry!(2), hpd_rx_int_entry!(3), hpd_rx_int_entry!(4), hpd_rx_int_entry!(5), hpd_rx_int_entry!(6),
    [DC_IRQ_SOURCE_TIMER] = dummy_irq_entry!(),
    pflip_int_entry!(0), pflip_int_entry!(1), pflip_int_entry!(2), pflip_int_entry!(3), pflip_int_entry!(4), pflip_int_entry!(5),
    vupdate_int_entry!(0), vupdate_int_entry!(1), vupdate_int_entry!(2), vupdate_int_entry!(3), vupdate_int_entry!(4), vupdate_int_entry!(5),
    vblank_int_entry!(0), vblank_int_entry!(1), vblank_int_entry!(2), vblank_int_entry!(3), vblank_int_entry!(4), vblank_int_entry!(5),
];

unsafe fn to_dal_irq_source_dce60(_irq_service: *mut irq_service, src_id: u32, ext_id: u32) -> dc_irq_source {
    match src_id {
        VISLANDS30_IV_SRCID_D1_VBLANK => DC_IRQ_SOURCE_VBLANK1,
        VISLANDS30_IV_SRCID_D2_VBLANK => DC_IRQ_SOURCE_VBLANK2,
        VISLANDS30_IV_SRCID_D3_VBLANK => DC_IRQ_SOURCE_VBLANK3,
        VISLANDS30_IV_SRCID_D4_VBLANK => DC_IRQ_SOURCE_VBLANK4,
        VISLANDS30_IV_SRCID_D5_VBLANK => DC_IRQ_SOURCE_VBLANK5,
        VISLANDS30_IV_SRCID_D6_VBLANK => DC_IRQ_SOURCE_VBLANK6,
        VISLANDS30_IV_SRCID_D1_V_UPDATE_INT => DC_IRQ_SOURCE_VUPDATE1,
        VISLANDS30_IV_SRCID_D2_V_UPDATE_INT => DC_IRQ_SOURCE_VUPDATE2,
        VISLANDS30_IV_SRCID_D3_V_UPDATE_INT => DC_IRQ_SOURCE_VUPDATE3,
        VISLANDS30_IV_SRCID_D4_V_UPDATE_INT => DC_IRQ_SOURCE_VUPDATE4,
        VISLANDS30_IV_SRCID_D5_V_UPDATE_INT => DC_IRQ_SOURCE_VUPDATE5,
        VISLANDS30_IV_SRCID_D6_V_UPDATE_INT => DC_IRQ_SOURCE_VUPDATE6,
        VISLANDS30_IV_SRCID_D1_GRPH_PFLIP => DC_IRQ_SOURCE_PFLIP1,
        VISLANDS30_IV_SRCID_D2_GRPH_PFLIP => DC_IRQ_SOURCE_PFLIP2,
        VISLANDS30_IV_SRCID_D3_GRPH_PFLIP => DC_IRQ_SOURCE_PFLIP3,
        VISLANDS30_IV_SRCID_D4_GRPH_PFLIP => DC_IRQ_SOURCE_PFLIP4,
        VISLANDS30_IV_SRCID_D5_GRPH_PFLIP => DC_IRQ_SOURCE_PFLIP5,
        VISLANDS30_IV_SRCID_D6_GRPH_PFLIP => DC_IRQ_SOURCE_PFLIP6,
        VISLANDS30_IV_SRCID_HOTPLUG_DETECT_A => match ext_id {
            VISLANDS30_IV_EXTID_HOTPLUG_DETECT_A => DC_IRQ_SOURCE_HPD1,
            VISLANDS30_IV_EXTID_HOTPLUG_DETECT_B => DC_IRQ_SOURCE_HPD2,
            VISLANDS30_IV_EXTID_HOTPLUG_DETECT_C => DC_IRQ_SOURCE_HPD3,
            VISLANDS30_IV_EXTID_HOTPLUG_DETECT_D => DC_IRQ_SOURCE_HPD4,
            VISLANDS30_IV_EXTID_HOTPLUG_DETECT_E => DC_IRQ_SOURCE_HPD5,
            VISLANDS30_IV_EXTID_HOTPLUG_DETECT_F => DC_IRQ_SOURCE_HPD6,
            VISLANDS30_IV_EXTID_HPD_RX_A => DC_IRQ_SOURCE_HPD1RX,
            VISLANDS30_IV_EXTID_HPD_RX_B => DC_IRQ_SOURCE_HPD2RX,
            VISLANDS30_IV_EXTID_HPD_RX_C => DC_IRQ_SOURCE_HPD3RX,
            VISLANDS30_IV_EXTID_HPD_RX_D => DC_IRQ_SOURCE_HPD4RX,
            VISLANDS30_IV_EXTID_HPD_RX_E => DC_IRQ_SOURCE_HPD5RX,
            VISLANDS30_IV_EXTID_HPD_RX_F => DC_IRQ_SOURCE_HPD6RX,
            _ => DC_IRQ_SOURCE_INVALID,
        },
        _ => DC_IRQ_SOURCE_INVALID,
    }
}

static irq_service_funcs_dce60: irq_service_funcs = irq_service_funcs { to_dal_irq_source: Some(to_dal_irq_source_dce60) };

unsafe fn dce60_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = &irq_source_info_dce60;
    (*irq_service).funcs = &irq_service_funcs_dce60;
}

unsafe fn dal_irq_service_dce60_create(init_data: *mut irq_service_init_data) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dce60_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
