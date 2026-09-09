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

// C dependencies supplied by the surrounding translation unit.

pub const HPD0_DC_HPD_INT_STATUS__DC_HPD_SENSE_DELAYED_MASK: u32 = 0x00000010;
pub const HPD0_DC_HPD_INT_CONTROL__DC_HPD_INT_POLARITY_MASK: u32 = 0x00000100;
pub const HPD0_DC_HPD_INT_STATUS__DC_HPD_SENSE_DELAYED__SHIFT: u32 = 0x4;
pub const HPD0_DC_HPD_INT_CONTROL__DC_HPD_INT_POLARITY__SHIFT: u32 = 0x8;
pub const DC_HPD1_INT_STATUS__DC_HPD1_SENSE_DELAYED_MASK: u32 = 0x10;
pub const DC_HPD1_INT_STATUS__DC_HPD1_SENSE_DELAYED__SHIFT: u32 = 0x4;
pub const DC_HPD1_INT_CONTROL__DC_HPD1_INT_POLARITY_MASK: u32 = 0x100;
pub const DC_HPD1_INT_CONTROL__DC_HPD1_INT_POLARITY__SHIFT: u32 = 0x8;

pub unsafe fn dal_irq_service_construct(
    irq_service: *mut irq_service,
    init_data: *mut irq_service_init_data,
) {
    if init_data.is_null() || (*init_data).ctx.is_null() {
        BREAK_TO_DEBUGGER!();
        return;
    }

    (*irq_service).ctx = (*init_data).ctx;
}

pub unsafe fn dal_irq_service_destroy(irq_service: *mut *mut irq_service) {
    if irq_service.is_null() || (*irq_service).is_null() {
        BREAK_TO_DEBUGGER!();
        return;
    }

    kfree(*irq_service);
    *irq_service = core::ptr::null_mut();
}

unsafe fn find_irq_source_info(
    irq_service: *mut irq_service,
    source: dc_irq_source,
) -> *const irq_source_info {
    if source >= DAL_IRQ_SOURCES_NUMBER {
        return core::ptr::null();
    }

    &(*irq_service).info[source as usize]
}

pub unsafe fn dal_irq_service_set_generic(
    irq_service: *mut irq_service,
    info: *const irq_source_info,
    enable: bool,
) {
    let addr: u32 = (*info).enable_reg;
    let mut value: u32 = dm_read_reg((*irq_service).ctx, addr);

    value = (value & !(*info).enable_mask)
        | ((*info).enable_value[if enable { 0 } else { 1 }] & (*info).enable_mask);
    dm_write_reg((*irq_service).ctx, addr, value);
}

pub unsafe fn dal_irq_service_set(
    irq_service: *mut irq_service,
    source: dc_irq_source,
    enable: bool,
) -> bool {
    let info = find_irq_source_info(irq_service, source);

    if info.is_null() {
        DC_LOG_ERROR!("%s: cannot find irq info table entry for %d\n", "dal_irq_service_set", source);
        return false;
    }

    dal_irq_service_ack(irq_service, source);

    if !(*info).funcs.is_null() && !(*(*info).funcs).set.is_none() {
        if (*(*info).funcs).set == Some(dal_irq_service_dummy_set) {
            DC_LOG_WARNING!("%s: src: %d, st: %d\n", "dal_irq_service_set", source, enable);
            ASSERT!(false);
        }
        return ((*(*info).funcs).set.unwrap())(irq_service, info, enable);
    }

    dal_irq_service_set_generic(irq_service, info, enable);
    true
}

pub unsafe fn dal_irq_service_ack_generic(
    irq_service: *mut irq_service,
    info: *const irq_source_info,
) {
    let addr: u32 = (*info).ack_reg;
    let mut value: u32 = dm_read_reg((*irq_service).ctx, addr);

    value = (value & !(*info).ack_mask) | ((*info).ack_value & (*info).ack_mask);
    dm_write_reg((*irq_service).ctx, addr, value);
}

pub unsafe fn dal_irq_service_ack(
    irq_service: *mut irq_service,
    source: dc_irq_source,
) -> bool {
    let info = find_irq_source_info(irq_service, source);

    if info.is_null() {
        DC_LOG_ERROR!("%s: cannot find irq info table entry for %d\n", "dal_irq_service_ack", source);
        return false;
    }

    if !(*info).funcs.is_null() && !(*(*info).funcs).ack.is_none() {
        if (*(*info).funcs).ack == Some(dal_irq_service_dummy_ack) {
            DC_LOG_WARNING!("%s: src: %d\n", "dal_irq_service_ack", source);
            ASSERT!(false);
        }
        return ((*(*info).funcs).ack.unwrap())(irq_service, info);
    }

    dal_irq_service_ack_generic(irq_service, info);
    true
}

pub unsafe fn dal_irq_service_to_irq_source(
    irq_service: *mut irq_service,
    src_id: u32,
    ext_id: u32,
) -> dc_irq_source {
    ((*irq_service).funcs.to_dal_irq_source)(irq_service, src_id, ext_id)
}

pub unsafe fn hpd0_ack(
    irq_service: *mut irq_service,
    info: *const irq_source_info,
) -> bool {
    let addr = (*info).status_reg;
    let mut value = dm_read_reg((*irq_service).ctx, addr);
    let current_status = get_reg_field_value!(value, HPD0_DC_HPD_INT_STATUS, DC_HPD_SENSE_DELAYED);

    dal_irq_service_ack_generic(irq_service, info);
    value = dm_read_reg((*irq_service).ctx, (*info).enable_reg);
    set_reg_field_value!(value, if current_status != 0 { 0 } else { 1 }, HPD0_DC_HPD_INT_CONTROL, DC_HPD_INT_POLARITY);
    dm_write_reg((*irq_service).ctx, (*info).enable_reg, value);
    true
}

pub unsafe fn hpd1_ack(
    irq_service: *mut irq_service,
    info: *const irq_source_info,
) -> bool {
    let addr = (*info).status_reg;
    let mut value = dm_read_reg((*irq_service).ctx, addr);
    let current_status = get_reg_field_value!(value, DC_HPD1_INT_STATUS, DC_HPD1_SENSE_DELAYED);

    dal_irq_service_ack_generic(irq_service, info);
    value = dm_read_reg((*irq_service).ctx, (*info).enable_reg);
    set_reg_field_value!(value, if current_status != 0 { 0 } else { 1 }, DC_HPD1_INT_CONTROL, DC_HPD1_INT_POLARITY);
    dm_write_reg((*irq_service).ctx, (*info).enable_reg, value);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
