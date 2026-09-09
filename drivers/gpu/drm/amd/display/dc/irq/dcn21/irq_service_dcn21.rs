/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

// C headers and register definitions are supplied by the surrounding crate.

unsafe fn to_dal_irq_source_dcn21(
    irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
    let _ = irq_service;
    let _ = ext_id;
    match src_id {
        DCN_1_0__SRCID__DC_D1_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK1,
        DCN_1_0__SRCID__DC_D2_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK2,
        DCN_1_0__SRCID__DC_D3_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK3,
        DCN_1_0__SRCID__DC_D4_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK4,
        DCN_1_0__SRCID__DC_D5_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK5,
        DCN_1_0__SRCID__DC_D6_OTG_VSTARTUP => DC_IRQ_SOURCE_VBLANK6,
        DCN_1_0__SRCID__DMCUB_OUTBOX_LOW_PRIORITY_READY_INT => DC_IRQ_SOURCE_DMCUB_OUTBOX,
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

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: Some(hpd0_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vupdate_no_lock_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut dmub_outbox_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline0_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };

// The following macros retain the C preprocessor's compile-time register
// concatenation and designated-entry intent for the generated crate.
macro_rules! base_inner { ($seg:tt) => { DMU_BASE__INST0_SEG$seg }; }
macro_rules! base { ($seg:tt) => { base_inner!($seg) }; }
macro_rules! sri { ($reg:ident, $block:ident, $id:tt) => { base!(mm$block$id_$reg_BASE_IDX) + mm$block$id_$reg }; }
macro_rules! sri_dmuB { ($reg:ident) => { base!(mm$reg_BASE_IDX) + mm$reg }; }
macro_rules! irq_reg_entry {
    ($block:ident, $num:tt, $reg1:ident, $mask1:ident, $reg2:ident, $mask2:ident) => {
        irq_source_info { enable_reg: sri!($reg1, $block, $num), enable_mask: $block$num_$reg1__$mask1_MASK,
            enable_value: [$block$num_$reg1__$mask1_MASK, !($block$num_$reg1__$mask1_MASK as u32)],
            ack_reg: sri!($reg2, $block, $num), ack_mask: $block$num_$reg2__$mask2_MASK,
            ack_value: $block$num_$reg2__$mask2_MASK, ..Default::default() }
    };
}
macro_rules! irq_reg_entry_dmuB {
    ($reg1:ident, $mask1:ident, $reg2:ident, $mask2:ident) => {
        irq_source_info { enable_reg: sri_dmuB!($reg1), enable_mask: $reg1__$mask1_MASK,
            enable_value: [$reg1__$mask1_MASK, !($reg1__$mask1_MASK as u32)],
            ack_reg: sri_dmuB!($reg2), ack_mask: $reg2__$mask2_MASK,
            ack_value: $reg2__$mask2_MASK, ..Default::default() }
    };
}
macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: &dummy_irq_info_funcs, ..Default::default() } }; }
macro_rules! hpd_int_entry { ($n:tt) => { irq_reg_entry!(HPD, $n, DC_HPD_INT_CONTROL, DC_HPD_INT_EN, DC_HPD_INT_CONTROL, DC_HPD_INT_ACK) }; }
macro_rules! hpd_rx_int_entry { ($n:tt) => { irq_reg_entry!(HPD, $n, DC_HPD_INT_CONTROL, DC_HPD_RX_INT_EN, DC_HPD_INT_CONTROL, DC_HPD_RX_INT_ACK) }; }
macro_rules! pflip_int_entry { ($n:tt) => { irq_reg_entry!(HUBPREQ, $n, DCSURF_SURFACE_FLIP_INTERRUPT, SURFACE_FLIP_INT_MASK, DCSURF_SURFACE_FLIP_INTERRUPT, SURFACE_FLIP_CLEAR) }; }
macro_rules! vupdate_no_lock_int_entry { ($n:tt) => { irq_reg_entry!(OTG, $n, OTG_GLOBAL_SYNC_STATUS, VUPDATE_NO_LOCK_INT_EN, OTG_GLOBAL_SYNC_STATUS, VUPDATE_NO_LOCK_EVENT_CLEAR) }; }
macro_rules! vblank_int_entry { ($n:tt) => { irq_reg_entry!(OTG, $n, OTG_GLOBAL_SYNC_STATUS, VSTARTUP_INT_EN, OTG_GLOBAL_SYNC_STATUS, VSTARTUP_EVENT_CLEAR) }; }
macro_rules! vline0_int_entry { ($n:tt) => { irq_reg_entry!(OTG, $n, OTG_VERTICAL_INTERRUPT0_CONTROL, OTG_VERTICAL_INTERRUPT0_INT_ENABLE, OTG_VERTICAL_INTERRUPT0_CONTROL, OTG_VERTICAL_INTERRUPT0_CLEAR) }; }
macro_rules! dmub_outbox_int_entry { () => { irq_reg_entry_dmuB!(DMCUB_INTERRUPT_ENABLE, DMCUB_OUTBOX1_READY_INT_EN, DMCUB_INTERRUPT_ACK, DMCUB_OUTBOX1_READY_INT_ACK) }; }
macro_rules! i2c_int_entry { ($n:tt) => { dummy_irq_entry!() }; }
macro_rules! dp_sink_int_entry { ($n:tt) => { dummy_irq_entry!() }; }
macro_rules! gpio_pad_int_entry { ($n:tt) => { dummy_irq_entry!() }; }
macro_rules! dc_underflow_int_entry { ($n:tt) => { dummy_irq_entry!() }; }

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack),
};

// Register table entries are expanded from the corresponding C macros by the
// target's register-definition layer; the complete sparse table is retained
// here as a typed static for linkage with that layer.
static irq_source_info_dcn21: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    dummy_irq_entry!(),
    hpd_int_entry!(0), hpd_int_entry!(1), hpd_int_entry!(2), hpd_int_entry!(3), hpd_int_entry!(4),
    hpd_rx_int_entry!(0), hpd_rx_int_entry!(1), hpd_rx_int_entry!(2), hpd_rx_int_entry!(3), hpd_rx_int_entry!(4),
    i2c_int_entry!(1), i2c_int_entry!(2), i2c_int_entry!(3), i2c_int_entry!(4), i2c_int_entry!(5), i2c_int_entry!(6),
    dp_sink_int_entry!(1), dp_sink_int_entry!(2), dp_sink_int_entry!(3), dp_sink_int_entry!(4), dp_sink_int_entry!(5), dp_sink_int_entry!(6),
    dummy_irq_entry!(), pflip_int_entry!(0), pflip_int_entry!(1), pflip_int_entry!(2), pflip_int_entry!(3),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    gpio_pad_int_entry!(0), gpio_pad_int_entry!(1), gpio_pad_int_entry!(2), gpio_pad_int_entry!(3), gpio_pad_int_entry!(4), gpio_pad_int_entry!(5), gpio_pad_int_entry!(6), gpio_pad_int_entry!(7), gpio_pad_int_entry!(8), gpio_pad_int_entry!(9), gpio_pad_int_entry!(10), gpio_pad_int_entry!(11), gpio_pad_int_entry!(12), gpio_pad_int_entry!(13), gpio_pad_int_entry!(14), gpio_pad_int_entry!(15), gpio_pad_int_entry!(16), gpio_pad_int_entry!(17), gpio_pad_int_entry!(18), gpio_pad_int_entry!(19), gpio_pad_int_entry!(20), gpio_pad_int_entry!(21), gpio_pad_int_entry!(22), gpio_pad_int_entry!(23), gpio_pad_int_entry!(24), gpio_pad_int_entry!(25), gpio_pad_int_entry!(26), gpio_pad_int_entry!(27), gpio_pad_int_entry!(28), gpio_pad_int_entry!(29), gpio_pad_int_entry!(30),
    dc_underflow_int_entry!(1), dc_underflow_int_entry!(2), dc_underflow_int_entry!(3), dc_underflow_int_entry!(4), dc_underflow_int_entry!(5), dc_underflow_int_entry!(6),
    dummy_irq_entry!(), dummy_irq_entry!(),
    vupdate_no_lock_int_entry!(0), vupdate_no_lock_int_entry!(1), vupdate_no_lock_int_entry!(2), vupdate_no_lock_int_entry!(3), vupdate_no_lock_int_entry!(4), vupdate_no_lock_int_entry!(5),
    vblank_int_entry!(0), vblank_int_entry!(1), vblank_int_entry!(2), vblank_int_entry!(3), vblank_int_entry!(4), vblank_int_entry!(5),
    vline0_int_entry!(0), vline0_int_entry!(1), vline0_int_entry!(2), vline0_int_entry!(3), vline0_int_entry!(4), vline0_int_entry!(5),
    dmub_outbox_int_entry!(),
];

static irq_service_funcs_dcn21: irq_service_funcs = irq_service_funcs {
    to_dal_irq_source: Some(to_dal_irq_source_dcn21),
};

unsafe fn dcn21_irq_construct(
    irq_service: *mut irq_service,
    init_data: *mut irq_service_init_data,
) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dcn21.as_ptr();
    (*irq_service).funcs = &irq_service_funcs_dcn21;
}

unsafe fn dal_irq_service_dcn21_create(
    init_data: *mut irq_service_init_data,
) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() {
        return core::ptr::null_mut();
    }
    dcn21_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
