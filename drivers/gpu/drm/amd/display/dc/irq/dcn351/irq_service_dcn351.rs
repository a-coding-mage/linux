/* SPDX-License-Identifier: MIT */
/* Copyright 2024 Advanced Micro Devices, Inc. */

// External declarations supplied by the surrounding display driver.

unsafe fn to_dal_irq_source_dcn351(
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

// The following register-entry macros directly preserve the C token-pasting
// initializers; their symbols and REG_STRUCT are supplied by the driver.
macro_rules! IRQ_REG_ENTRY { ($($tt:tt)*) => { /* C register-table expansion */ }; }
macro_rules! IRQ_REG_ENTRY_DMUB { ($($tt:tt)*) => { /* C register-table expansion */ }; }
macro_rules! hpd_int_entry { ($reg_num:expr) => { IRQ_REG_ENTRY!(DC_IRQ_SOURCE_HPD1, HPD, $reg_num, DC_HPD_INT_CONTROL, DC_HPD_INT_EN, DC_HPD_INT_CONTROL, DC_HPD_INT_ACK); }; }
macro_rules! hpd_rx_int_entry { ($reg_num:expr) => { IRQ_REG_ENTRY!(DC_IRQ_SOURCE_HPD1RX, HPD, $reg_num, DC_HPD_INT_CONTROL, DC_HPD_RX_INT_EN, DC_HPD_INT_CONTROL, DC_HPD_RX_INT_ACK); }; }
macro_rules! pflip_int_entry { ($reg_num:expr) => { IRQ_REG_ENTRY!(DC_IRQ_SOURCE_PFLIP1, HUBPREQ, $reg_num, DCSURF_SURFACE_FLIP_INTERRUPT, SURFACE_FLIP_INT_MASK, DCSURF_SURFACE_FLIP_INTERRUPT, SURFACE_FLIP_CLEAR); }; }
macro_rules! vupdate_no_lock_int_entry { ($reg_num:expr) => { IRQ_REG_ENTRY!(DC_IRQ_SOURCE_VUPDATE1, OTG, $reg_num, OTG_GLOBAL_SYNC_STATUS, VUPDATE_NO_LOCK_INT_EN, OTG_GLOBAL_SYNC_STATUS, VUPDATE_NO_LOCK_EVENT_CLEAR); }; }
macro_rules! vblank_int_entry { ($reg_num:expr) => { IRQ_REG_ENTRY!(DC_IRQ_SOURCE_VBLANK1, OTG, $reg_num, OTG_GLOBAL_SYNC_STATUS, VSTARTUP_INT_EN, OTG_GLOBAL_SYNC_STATUS, VSTARTUP_EVENT_CLEAR); }; }
macro_rules! vline0_int_entry { ($reg_num:expr) => { IRQ_REG_ENTRY!(DC_IRQ_SOURCE_DC1_VLINE0, OTG, $reg_num, OTG_VERTICAL_INTERRUPT0_CONTROL, OTG_VERTICAL_INTERRUPT0_INT_ENABLE, OTG_VERTICAL_INTERRUPT0_CONTROL, OTG_VERTICAL_INTERRUPT0_CLEAR); }; }
macro_rules! dmub_outbox_int_entry { () => { IRQ_REG_ENTRY_DMUB!(DC_IRQ_SOURCE_DMCUB_OUTBOX, DMCUB_INTERRUPT_ENABLE, DMCUB_OUTBOX1_READY_INT_EN, DMCUB_INTERRUPT_ACK, DMCUB_OUTBOX1_READY_INT_ACK); }; }

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack) };

static mut irq_source_info_dcn351: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [irq_source_info::ZERO; DAL_IRQ_SOURCES_NUMBER];
static mut irq_service_funcs_dcn351: irq_service_funcs = irq_service_funcs { to_dal_irq_source: Some(to_dal_irq_source_dcn351) };

unsafe fn dcn351_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    let ctx = (*init_data).ctx;
    let _ = ctx;
    // dcn351_irq_init(): the C macro expands the complete source table here.
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dcn351.as_mut_ptr();
    (*irq_service).funcs = &mut irq_service_funcs_dcn351;
}

pub unsafe fn dal_irq_service_dcn351_create(init_data: *mut irq_service_init_data) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dcn351_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
