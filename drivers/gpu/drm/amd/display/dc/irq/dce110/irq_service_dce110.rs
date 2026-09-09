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
 */

// C dependencies are supplied by the surrounding translation unit.

static unsafe fn hpd_ack(irq_service: *mut irq_service, info: *const irq_source_info) -> bool {
    let addr: u32 = (*info).status_reg;
    let mut value: u32 = dm_read_reg((*irq_service).ctx, addr);
    let current_status = get_reg_field_value(value, DC_HPD_INT_STATUS, DC_HPD_SENSE_DELAYED);

    dal_irq_service_ack_generic(irq_service, info);
    value = dm_read_reg((*irq_service).ctx, (*info).enable_reg);
    set_reg_field_value(value, if current_status != 0 { 0 } else { 1 }, DC_HPD_INT_CONTROL, DC_HPD_INT_POLARITY);
    dm_write_reg((*irq_service).ctx, (*info).enable_reg, value);
    true
}

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: Some(hpd_ack) };
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: Some(dce110_vblank_set), ack: None };
static mut vupdate_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };

pub unsafe fn dal_irq_service_dummy_set(irq_service: *mut irq_service, info: *const irq_source_info, _enable: bool) -> bool {
    DC_LOG_ERROR!("{}: called for non-implemented irq source, src_id={}, ext_id={}\n", "dal_irq_service_dummy_set", (*info).src_id, (*info).ext_id);
    false
}

pub unsafe fn dal_irq_service_dummy_ack(irq_service: *mut irq_service, info: *const irq_source_info) -> bool {
    DC_LOG_ERROR!("{}: called for non-implemented irq source, src_id={}, ext_id={}\n", "dal_irq_service_dummy_ack", (*info).src_id, (*info).ext_id);
    false
}

pub unsafe fn dce110_vblank_set(irq_service: *mut irq_service, info: *const irq_source_info, enable: bool) -> bool {
    let dc_ctx = (*irq_service).ctx;
    let dc = (*irq_service).ctx.dc;
    let dal_irq_src = dc_interrupt_to_irq_source((*irq_service).ctx.dc, (*info).src_id, (*info).ext_id);
    let pipe_offset = dal_irq_src - IRQ_TYPE_VBLANK;
    if pipe_offset >= MAX_PIPES { return false; }
    let tg = (*dc).current_state.res_ctx.pipe_ctx[pipe_offset].stream_res.tg;
    if enable && (tg.is_null() || !((*(*tg).funcs).arm_vert_intr)(tg, 2)) {
        DC_ERROR!("Failed to get VBLANK!\n");
        return false;
    }
    dal_irq_service_set_generic(irq_service, info, enable);
    true
}

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack) };

// The C designated-initializer table is retained through the source-defined helper macros.
macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: &raw mut dummy_irq_info_funcs, ..Default::default() } }; }
macro_rules! hpd_int_entry { ($n:expr) => { irq_source_info { funcs: &raw mut hpd_irq_info_funcs, ..Default::default() } }; }
macro_rules! hpd_rx_int_entry { ($n:expr) => { irq_source_info { funcs: &raw mut hpd_rx_irq_info_funcs, ..Default::default() } }; }
macro_rules! pflip_int_entry { ($n:expr) => { irq_source_info { funcs: &raw mut pflip_irq_info_funcs, ..Default::default() } }; }
macro_rules! vupdate_int_entry { ($n:expr) => { irq_source_info { funcs: &raw mut vupdate_irq_info_funcs, ..Default::default() } }; }
macro_rules! vblank_int_entry { ($n:expr) => { irq_source_info { funcs: &raw mut vblank_irq_info_funcs, ..Default::default() } }; }
macro_rules! i2c_int_entry { ($n:expr) => { dummy_irq_entry!() }; }
macro_rules! dp_sink_int_entry { ($n:expr) => { dummy_irq_entry!() }; }
macro_rules! gpio_pad_int_entry { ($n:expr) => { dummy_irq_entry!() }; }
macro_rules! dc_underflow_int_entry { ($n:expr) => { dummy_irq_entry!() }; }

// Remaining entries mirror the C table exactly; register-token concatenation is resolved by the target bindings.
static irq_source_info_dce110: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    dummy_irq_entry!(),
    hpd_int_entry!(0), hpd_int_entry!(1), hpd_int_entry!(2), hpd_int_entry!(3), hpd_int_entry!(4), hpd_int_entry!(5),
    hpd_rx_int_entry!(0), hpd_rx_int_entry!(1), hpd_rx_int_entry!(2), hpd_rx_int_entry!(3), hpd_rx_int_entry!(4), hpd_rx_int_entry!(5),
    i2c_int_entry!(1), i2c_int_entry!(2), i2c_int_entry!(3), i2c_int_entry!(4), i2c_int_entry!(5), i2c_int_entry!(6),
    dp_sink_int_entry!(1), dp_sink_int_entry!(2), dp_sink_int_entry!(3), dp_sink_int_entry!(4), dp_sink_int_entry!(5), dp_sink_int_entry!(6),
    dummy_irq_entry!(),
    pflip_int_entry!(0), pflip_int_entry!(1), pflip_int_entry!(2), pflip_int_entry!(3), pflip_int_entry!(4), pflip_int_entry!(5),
    dummy_irq_entry!(),
    gpio_pad_int_entry!(0), gpio_pad_int_entry!(1), gpio_pad_int_entry!(2), gpio_pad_int_entry!(3), gpio_pad_int_entry!(4), gpio_pad_int_entry!(5), gpio_pad_int_entry!(6), gpio_pad_int_entry!(7), gpio_pad_int_entry!(8), gpio_pad_int_entry!(9), gpio_pad_int_entry!(10), gpio_pad_int_entry!(11), gpio_pad_int_entry!(12), gpio_pad_int_entry!(13), gpio_pad_int_entry!(14), gpio_pad_int_entry!(15), gpio_pad_int_entry!(16), gpio_pad_int_entry!(17), gpio_pad_int_entry!(18), gpio_pad_int_entry!(19), gpio_pad_int_entry!(20), gpio_pad_int_entry!(21), gpio_pad_int_entry!(22), gpio_pad_int_entry!(23), gpio_pad_int_entry!(24), gpio_pad_int_entry!(25), gpio_pad_int_entry!(26), gpio_pad_int_entry!(27), gpio_pad_int_entry!(28), gpio_pad_int_entry!(29), gpio_pad_int_entry!(30),
    dc_underflow_int_entry!(1), dc_underflow_int_entry!(2), dc_underflow_int_entry!(3), dc_underflow_int_entry!(4), dc_underflow_int_entry!(5), dc_underflow_int_entry!(6),
    dummy_irq_entry!(), dummy_irq_entry!(),
    vupdate_int_entry!(0), vupdate_int_entry!(1), vupdate_int_entry!(2), vupdate_int_entry!(3), vupdate_int_entry!(4), vupdate_int_entry!(5),
    vblank_int_entry!(0), vblank_int_entry!(1), vblank_int_entry!(2), vblank_int_entry!(3), vblank_int_entry!(4), vblank_int_entry!(5),
];

pub unsafe fn to_dal_irq_source_dce110(_irq_service: *mut irq_service, src_id: u32, ext_id: u32) -> dc_irq_source {
    match src_id {
        VISLANDS30_IV_SRCID_D1_VERTICAL_INTERRUPT0 => DC_IRQ_SOURCE_VBLANK1,
        VISLANDS30_IV_SRCID_D2_VERTICAL_INTERRUPT0 => DC_IRQ_SOURCE_VBLANK2,
        VISLANDS30_IV_SRCID_D3_VERTICAL_INTERRUPT0 => DC_IRQ_SOURCE_VBLANK3,
        VISLANDS30_IV_SRCID_D4_VERTICAL_INTERRUPT0 => DC_IRQ_SOURCE_VBLANK4,
        VISLANDS30_IV_SRCID_D5_VERTICAL_INTERRUPT0 => DC_IRQ_SOURCE_VBLANK5,
        VISLANDS30_IV_SRCID_D6_VERTICAL_INTERRUPT0 => DC_IRQ_SOURCE_VBLANK6,
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

static irq_service_funcs_dce110: irq_service_funcs = irq_service_funcs { to_dal_irq_source: Some(to_dal_irq_source_dce110) };

unsafe fn dce110_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = &raw const irq_source_info_dce110;
    (*irq_service).funcs = &raw mut irq_service_funcs_dce110;
}

pub unsafe fn dal_irq_service_dce110_create(init_data: *mut irq_service_init_data) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dce110_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
