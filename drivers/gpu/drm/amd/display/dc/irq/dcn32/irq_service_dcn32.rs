/* Translated from irq_service_dcn32.c.  External types, constants, and
 * functions are supplied by the surrounding driver. */

pub const DCN_BASE_INST0_SEG2: u32 = 0x0000_34c0;

unsafe fn to_dal_irq_source_dcn32(
    irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
    let _ = irq_service;
    let _ = src_id;
    let _ = ext_id;
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

static mut hpd_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: None, ack: Some(hpd0_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vupdate_no_lock_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut outbox_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline0_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline1_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline2_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };

/* C token-pasting macros are represented by Rust table-construction macros;
 * register and mask constants remain external driver dependencies. */
macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: unsafe { &raw mut dummy_irq_info_funcs }, ..Default::default() } }; }
macro_rules! reg_irq_entry { ($funcs:ident) => { irq_source_info { funcs: unsafe { &raw mut $funcs }, ..Default::default() } }; }
macro_rules! hpd_int_entry { ($n:expr) => { reg_irq_entry!(hpd_irq_info_funcs) }; }
macro_rules! hpd_rx_int_entry { ($n:expr) => { reg_irq_entry!(hpd_rx_irq_info_funcs) }; }
macro_rules! pflip_int_entry { ($n:expr) => { reg_irq_entry!(pflip_irq_info_funcs) }; }
macro_rules! vblank_int_entry { ($n:expr) => { reg_irq_entry!(vblank_irq_info_funcs) }; }
macro_rules! vupdate_no_lock_int_entry { ($n:expr) => { reg_irq_entry!(vupdate_no_lock_irq_info_funcs) }; }
macro_rules! vline0_int_entry { ($n:expr) => { reg_irq_entry!(vline0_irq_info_funcs) }; }
macro_rules! vline1_int_entry { ($n:expr) => { reg_irq_entry!(vline1_irq_info_funcs) }; }
macro_rules! vline2_int_entry { ($n:expr) => { reg_irq_entry!(vline2_irq_info_funcs) }; }
macro_rules! dmub_outbox_int_entry { () => { reg_irq_entry!(outbox_irq_info_funcs) }; }

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack),
};

/* The designated C initializer table is retained in source order; its
 * register fields are generated by the macros above in the target driver. */
static irq_source_info_dcn32: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    dummy_irq_entry!(), hpd_int_entry!(0), hpd_int_entry!(1), hpd_int_entry!(2),
    hpd_int_entry!(3), hpd_int_entry!(4), hpd_rx_int_entry!(0), hpd_rx_int_entry!(1),
    hpd_rx_int_entry!(2), hpd_rx_int_entry!(3), hpd_rx_int_entry!(4),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    pflip_int_entry!(0), pflip_int_entry!(1), pflip_int_entry!(2), pflip_int_entry!(3),
    dummy_irq_entry!(), dummy_irq_entry!(), dummy_irq_entry!(),
    vblank_int_entry!(0), vblank_int_entry!(1), vblank_int_entry!(2), vblank_int_entry!(3),
    dummy_irq_entry!(), dummy_irq_entry!(), dmub_outbox_int_entry!(),
    vupdate_no_lock_int_entry!(0), vupdate_no_lock_int_entry!(1), vupdate_no_lock_int_entry!(2),
    vupdate_no_lock_int_entry!(3), vline0_int_entry!(0), vline0_int_entry!(1),
    vline0_int_entry!(2), vline0_int_entry!(3), vline1_int_entry!(0), vline1_int_entry!(1),
    vline1_int_entry!(2), vline1_int_entry!(3), vline2_int_entry!(0), vline2_int_entry!(1),
    vline2_int_entry!(2), vline2_int_entry!(3),
];

static irq_service_funcs_dcn32: irq_service_funcs = irq_service_funcs {
    to_dal_irq_source: Some(to_dal_irq_source_dcn32),
};

unsafe fn dcn32_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dcn32.as_ptr();
    (*irq_service).funcs = &raw const irq_service_funcs_dcn32;
}

pub unsafe fn dal_irq_service_dcn32_create(
    init_data: *mut irq_service_init_data,
) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() { return core::ptr::null_mut(); }
    dcn32_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
