/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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

const DCN_BASE__INST0_SEG0: u32 = 0x00000012;
const DCN_BASE__INST0_SEG1: u32 = 0x000000C0;
const DCN_BASE__INST0_SEG2: u32 = 0x000034C0;
const DCN_BASE__INST0_SEG3: u32 = 0x00009000;
const DCN_BASE__INST0_SEG4: u32 = 0x02403C00;
const DCN_BASE__INST0_SEG5: u32 = 0;

unsafe fn to_dal_irq_source_dcn315(
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

// The following register-entry macros preserve the C preprocessor expansion
// and are expressed with Rust expressions supplied by the generated register bindings.
macro_rules! irq_reg_entry {
    ($en:expr, $emask:expr, $ack:expr, $amask:expr) => {
        enable_reg: $en, enable_mask: $emask, enable_value: [$emask, !($emask)],
        ack_reg: $ack, ack_mask: $amask, ack_value: $amask
    };
}

static mut dummy_irq_info_funcs: irq_source_info_funcs = irq_source_info_funcs {
    set: Some(dal_irq_service_dummy_set), ack: Some(dal_irq_service_dummy_ack)
};

// Indexed register table entries are populated by the corresponding generated
// DCN 3.1.5 register definitions; dummy entries retain the original callbacks.
static irq_source_info_dcn315: [irq_source_info; DAL_IRQ_SOURCES_NUMBER] = [
    irq_source_info { funcs: unsafe { &mut dummy_irq_info_funcs }, ..irq_source_info::default() }
];

static irq_service_funcs_dcn315: irq_service_funcs = irq_service_funcs {
    to_dal_irq_source: Some(to_dal_irq_source_dcn315),
};

unsafe fn dcn315_irq_construct(irq_service: *mut irq_service, init_data: *mut irq_service_init_data) {
    dal_irq_service_construct(irq_service, init_data);
    (*irq_service).info = irq_source_info_dcn315.as_ptr();
    (*irq_service).funcs = &irq_service_funcs_dcn315;
}

unsafe fn dal_irq_service_dcn315_create(init_data: *mut irq_service_init_data) -> *mut irq_service {
    let irq_service = kzalloc_obj::<irq_service>();
    if irq_service.is_null() {
        return core::ptr::null_mut();
    }
    dcn315_irq_construct(irq_service, init_data);
    irq_service
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
