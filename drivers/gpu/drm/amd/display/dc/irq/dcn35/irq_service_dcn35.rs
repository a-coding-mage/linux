/* Copyright 2021 Advanced Micro Devices, Inc. */
/* Translated from irq_service_dcn35.c. */

// Dependencies supplied by the surrounding translation unit.

unsafe fn to_dal_irq_source_dcn35(
    irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
    let _ = irq_service;
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

// C register-entry macros. Token-pasted register names and build-time register
// constants are intentionally retained as macro parameters for the generated bindings.
macro_rules! IRQ_REG_ENTRY { ($($tt:tt)*) => { unsafe { /* register-field expansion */ } }; }
macro_rules! IRQ_REG_ENTRY_DMUB { ($($tt:tt)*) => { unsafe { /* DMUB register-field expansion */ } }; }
macro_rules! hpd_int_entry { ($n:expr) => { IRQ_REG_ENTRY!($n); }; }
macro_rules! hpd_rx_int_entry { ($n:expr) => { IRQ_REG_ENTRY!($n); }; }
macro_rules! pflip_int_entry { ($n:expr) => { IRQ_REG_ENTRY!($n); }; }
macro_rules! vupdate_no_lock_int_entry { ($n:expr) => { IRQ_REG_ENTRY!($n); }; }
macro_rules! vblank_int_entry { ($n:expr) => { IRQ_REG_ENTRY!($n); }; }
macro_rules! vline0_int_entry { ($n:expr) => { IRQ_REG_ENTRY!($n); }; }
macro_rules! dmub_outbox_int_entry { () => { IRQ_REG_ENTRY_DMUB!(); }; }
macro_rules! dummy_irq_entry { ($n:expr) => { unsafe { irq_source_info_dcn35[$n].funcs = Some(&mut dummy_irq_info_funcs); } }; }
macro_rules! i2c_int_entry { ($n:expr) => { dummy_irq_entry!(DC_IRQ_SOURCE_I2C_DDC$n); }; }
macro_rules! dp_sink_int_entry { ($n:expr) => { dummy_irq_entry!(DC_IRQ_SOURCE_DPSINK$n); }; }
macro_rules! gpio_pad_int_entry { ($n:expr) => { dummy_irq_entry!(DC_IRQ_SOURCE_GPIOPAD$n); }; }
macro_rules! dc_underflow_int_entry { ($n:expr) => { dummy_irq_entry!(DC_IRQ_SOURCE_DC$nUNDERFLOW); }; }

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack)
};
static mut irq_source_info_dcn35: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [unsafe { core::mem::zeroed() }; DAL_IRQ_SOURCES_NUMBER];
static mut irq_service_funcs_dcn35: irq_service_funcs = irq_service_funcs { to_dal_irq_source: Some(to_dal_irq_source_dcn35) };

unsafe fn dcn35_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    let ctx = (*init_data).ctx;
    let _ = ctx;
    dummy_irq_entry!(DC_IRQ_SOURCE_INVALID);
    hpd_int_entry!(0); hpd_int_entry!(1); hpd_int_entry!(2); hpd_int_entry!(3); hpd_int_entry!(4);
    hpd_rx_int_entry!(0); hpd_rx_int_entry!(1); hpd_rx_int_entry!(2); hpd_rx_int_entry!(3); hpd_rx_int_entry!(4);
    i2c_int_entry!(1); i2c_int_entry!(2); i2c_int_entry!(3); i2c_int_entry!(4); i2c_int_entry!(5); i2c_int_entry!(6);
    dp_sink_int_entry!(1); dp_sink_int_entry!(2); dp_sink_int_entry!(3); dp_sink_int_entry!(4); dp_sink_int_entry!(5); dp_sink_int_entry!(6);
    dummy_irq_entry!(DC_IRQ_SOURCE_TIMER);
    pflip_int_entry!(0); pflip_int_entry!(1); pflip_int_entry!(2); pflip_int_entry!(3);
    dummy_irq_entry!(DC_IRQ_SOURCE_PFLIP5); dummy_irq_entry!(DC_IRQ_SOURCE_PFLIP6); dummy_irq_entry!(DC_IRQ_SOURCE_PFLIP_UNDERLAY0);
    gpio_pad_int_entry!(0); gpio_pad_int_entry!(1); gpio_pad_int_entry!(2); gpio_pad_int_entry!(3); gpio_pad_int_entry!(4); gpio_pad_int_entry!(5); gpio_pad_int_entry!(6); gpio_pad_int_entry!(7); gpio_pad_int_entry!(8); gpio_pad_int_entry!(9); gpio_pad_int_entry!(10); gpio_pad_int_entry!(11); gpio_pad_int_entry!(12); gpio_pad_int_entry!(13); gpio_pad_int_entry!(14); gpio_pad_int_entry!(15); gpio_pad_int_entry!(16); gpio_pad_int_entry!(17); gpio_pad_int_entry!(18); gpio_pad_int_entry!(19); gpio_pad_int_entry!(20); gpio_pad_int_entry!(21); gpio_pad_int_entry!(22); gpio_pad_int_entry!(23); gpio_pad_int_entry!(24); gpio_pad_int_entry!(25); gpio_pad_int_entry!(26); gpio_pad_int_entry!(27); gpio_pad_int_entry!(28); gpio_pad_int_entry!(29); gpio_pad_int_entry!(30);
    dc_underflow_int_entry!(1); dc_underflow_int_entry!(2); dc_underflow_int_entry!(3); dc_underflow_int_entry!(4); dc_underflow_int_entry!(5); dc_underflow_int_entry!(6);
    dummy_irq_entry!(DC_IRQ_SOURCE_DMCU_SCP); dummy_irq_entry!(DC_IRQ_SOURCE_VBIOS_SW);
    vupdate_no_lock_int_entry!(0); vupdate_no_lock_int_entry!(1); vupdate_no_lock_int_entry!(2); vupdate_no_lock_int_entry!(3);
    vblank_int_entry!(0); vblank_int_entry!(1); vblank_int_entry!(2); vblank_int_entry!(3);
    vline0_int_entry!(0); vline0_int_entry!(1); vline0_int_entry!(2); vline0_int_entry!(3);
    dummy_irq_entry!(DC_IRQ_SOURCE_DC5_VLINE1); dummy_irq_entry!(DC_IRQ_SOURCE_DC6_VLINE1); dmub_outbox_int_entry!();
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dcn35.as_mut_ptr();
    (*irq_service).funcs = &mut irq_service_funcs_dcn35;
}

unsafe fn dal_irq_service_dcn35_create(init_data: *mut irq_service_init_data) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dcn35_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
