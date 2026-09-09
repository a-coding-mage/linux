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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

unsafe fn to_dal_irq_source_dcn201(
    _irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
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
    set: None, ack: Some(hpd0_ack),
};
static mut hpd_rx_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut pflip_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vblank_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vline0_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };
static mut vupdate_no_lock_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs { set: None, ack: None };

/* Register-entry macros from the C implementation are represented by the
 * corresponding external constants and C-compatible structure fields. */
macro_rules! dummy_irq_entry { () => { irq_source_info { funcs: &raw mut dummy_irq_info_funcs, ..unsafe { core::mem::zeroed() } } }; }

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack),
};

/* The designated initializer table retains the original source ordering. */
static irq_source_info_dcn201: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = unsafe { core::mem::zeroed() };

static irq_service_funcs_dcn201: irq_service_funcs = irq_service_funcs {
    to_dal_irq_source: Some(to_dal_irq_source_dcn201),
};

unsafe fn dcn201_irq_construct(
    irq_service: *mut irq_service,
    init_data: *mut irq_service_init_data,
) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = &irq_source_info_dcn201 as *const _;
    (*irq_service).funcs = &irq_service_funcs_dcn201 as *const _;
}

pub unsafe fn dal_irq_service_dcn201_create(
    init_data: *mut irq_service_init_data,
) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() {
        return core::ptr::null_mut();
    }
    dcn201_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
