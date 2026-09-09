// SPDX-License-Identifier: GPL-2.0
/*
 * System Control and Management Interface (SCMI) Voltage Protocol
 *
 * Copyright (C) 2020-2022 ARM Ltd.
 */

// Dependencies supplied by the surrounding SCMI/kernel translation.

const SCMI_PROTOCOL_SUPPORTED_VERSION: u32 = 0x20001;
const VOLTAGE_DOMS_NUM_MASK: u32 = 0xffff;
const REMAINING_LEVELS_MASK: u32 = 0xffff0000;
const RETURNED_LEVELS_MASK: u32 = 0x0fff;

#[repr(u8)]
enum scmi_voltage_protocol_cmd {
    VOLTAGE_DOMAIN_ATTRIBUTES = 0x3,
    VOLTAGE_DESCRIBE_LEVELS = 0x4,
    VOLTAGE_CONFIG_SET = 0x5,
    VOLTAGE_CONFIG_GET = 0x6,
    VOLTAGE_LEVEL_SET = 0x7,
    VOLTAGE_LEVEL_GET = 0x8,
    VOLTAGE_DOMAIN_NAME_GET = 0x09,
}

#[inline]
fn num_voltage_domains(x: u32) -> u16 { (x & VOLTAGE_DOMS_NUM_MASK) as u16 }

#[repr(C)]
struct scmi_msg_resp_domain_attributes {
    attr: __le32,
    name: [u8; SCMI_SHORT_NAME_MAX_SIZE],
}

#[inline]
fn supports_async_level_set(x: u32) -> bool { (x & (1u32 << 31)) != 0 }
#[inline]
fn supports_extended_names(x: u32) -> bool { (x & (1u32 << 30)) != 0 }

#[repr(C)]
struct scmi_msg_cmd_describe_levels { domain_id: __le32, level_index: __le32 }

#[repr(C)]
struct scmi_msg_resp_describe_levels { flags: __le32, voltage: [__le32; 0] }

#[inline]
fn num_remaining_levels(f: u32) -> u16 { ((f & REMAINING_LEVELS_MASK) >> 16) as u16 }
#[inline]
fn num_returned_levels(f: u32) -> u16 { (f & RETURNED_LEVELS_MASK) as u16 }
#[inline]
fn supports_segmented_levels(f: u32) -> bool { (f & (1u32 << 12)) != 0 }

#[repr(C)]
struct scmi_msg_cmd_config_set { domain_id: __le32, config: __le32 }

#[repr(C)]
struct scmi_msg_cmd_level_set { domain_id: __le32, flags: __le32, voltage_level: __le32 }

#[repr(C)]
struct scmi_resp_voltage_level_set_complete { domain_id: __le32, voltage_level: __le32 }

#[repr(C)]
struct voltage_info { num_domains: c_uint, domains: *mut scmi_voltage_info }

unsafe fn scmi_protocol_attributes_get(ph: *const scmi_protocol_handle, vinfo: *mut voltage_info) -> c_int {
    let mut t: *mut scmi_xfer = core::ptr::null_mut();
    let ret = (*(*ph).xops).xfer_get_init(ph, PROTOCOL_ATTRIBUTES, 0, core::mem::size_of::<__le32>(), &mut t);
    if ret != 0 { return ret; }
    let ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 { (*vinfo).num_domains = num_voltage_domains(get_unaligned_le32((*t).rx.buf)); }
    (*(*ph).xops).xfer_put(ph, t);
    ret
}

unsafe fn scmi_init_voltage_levels(dev: *mut device, v: *mut scmi_voltage_info, num_returned: u32, num_remaining: u32, segmented: bool) -> c_int {
    let num_levels = num_returned + num_remaining;
    if num_levels == 0 || (segmented && (num_remaining != 0 || num_returned != 3)) {
        dev_err(dev, "Invalid level descriptor(%d/%d/%d) for voltage dom %d\\n", num_levels, num_returned, num_remaining, (*v).id);
        return -EINVAL;
    }
    (*v).levels_uv = devm_kcalloc(dev, num_levels as usize, core::mem::size_of::<u32>(), GFP_KERNEL);
    if (*v).levels_uv.is_null() { return -ENOMEM; }
    (*v).num_levels = num_levels;
    (*v).segmented = segmented;
    0
}

#[repr(C)]
struct scmi_volt_ipriv { dev: *mut device, v: *mut scmi_voltage_info }

unsafe fn iter_volt_levels_prepare_message(message: *mut c_void, desc_index: c_uint, priv_: *const c_void) {
    let msg = message as *mut scmi_msg_cmd_describe_levels;
    let p = priv_ as *const scmi_volt_ipriv;
    (*msg).domain_id = cpu_to_le32((*(*p).v).id);
    (*msg).level_index = cpu_to_le32(desc_index);
}

unsafe fn iter_volt_levels_update_state(st: *mut scmi_iterator_state, response: *const c_void, priv_: *mut c_void) -> c_int {
    let r = response as *const scmi_msg_resp_describe_levels;
    let p = priv_ as *mut scmi_volt_ipriv;
    let flags = le32_to_cpu((*r).flags);
    (*st).num_returned = num_returned_levels(flags) as u32;
    (*st).num_remaining = num_remaining_levels(flags) as u32;
    if (*(*p).v).num_levels == 0 {
        let ret = scmi_init_voltage_levels((*p).dev, (*p).v, (*st).num_returned, (*st).num_remaining, supports_segmented_levels(flags));
        if ret == 0 { (*st).max_resources = (*(*p).v).num_levels; }
        return ret;
    }
    0
}

unsafe fn iter_volt_levels_process_response(_ph: *const scmi_protocol_handle, response: *const c_void, st: *mut scmi_iterator_state, priv_: *mut c_void) -> c_int {
    let r = response as *const scmi_msg_resp_describe_levels;
    let p = priv_ as *mut scmi_volt_ipriv;
    let val = le32_to_cpu(*((*r).voltage.as_ptr().add((*st).loop_idx as usize))) as i32;
    *(*p).v.levels_uv.add(((*st).desc_index + (*st).loop_idx) as usize) = val as u32;
    if val < 0 { (*(*p).v).negative_volts_allowed = true; }
    0
}

// The remaining protocol operations retain the C implementation's ABI-facing
// behavior and are expressed using the surrounding SCMI operation definitions.
// External kernel types, helpers, and protocol tables are intentionally left as dependencies.

// SAFETY: This file is a direct low-level translation; callers must provide valid SCMI handles.
unsafe fn scmi_voltage_levels_get(ph: *const scmi_protocol_handle, v: *mut scmi_voltage_info) -> c_int {
    let mut ops = scmi_iterator_ops { prepare_message: Some(iter_volt_levels_prepare_message), update_state: Some(iter_volt_levels_update_state), process_response: Some(iter_volt_levels_process_response) };
    let priv_ = scmi_volt_ipriv { dev: (*ph).dev, v };
    let iter = (*(*ph).hops).iter_response_init(ph, &mut ops, (*v).num_levels, VOLTAGE_DESCRIBE_LEVELS as u8, core::mem::size_of::<scmi_msg_cmd_describe_levels>(), &priv_ as *const _ as *const c_void);
    if IS_ERR(iter) { return PTR_ERR(iter); }
    let ret = (*(*ph).hops).iter_response_run(iter);
    if ret != 0 { (*v).num_levels = 0; devm_kfree((*ph).dev, (*v).levels_uv as *mut c_void); }
    ret
}

unsafe fn scmi_voltage_descriptors_get(ph: *const scmi_protocol_handle, vinfo: *mut voltage_info) -> c_int {
    let mut td: *mut scmi_xfer = core::ptr::null_mut();
    let ret = (*(*ph).xops).xfer_get_init(ph, VOLTAGE_DOMAIN_ATTRIBUTES as u8, core::mem::size_of::<__le32>(), core::mem::size_of::<scmi_msg_resp_domain_attributes>(), &mut td);
    if ret != 0 { return ret; }
    let resp = (*td).rx.buf as *const scmi_msg_resp_domain_attributes;
    for dom in 0..(*vinfo).num_domains {
        put_unaligned_le32(dom, (*td).tx.buf);
        if (*(*ph).xops).do_xfer(ph, td) != 0 { (*(*ph).xops).reset_rx_to_maxsz(ph, td); continue; }
        let v = (*vinfo).domains.add(dom as usize);
        (*v).id = dom;
        let attributes = le32_to_cpu((*resp).attr);
        strscpy((*v).name.as_mut_ptr(), (*resp).name.as_ptr(), SCMI_SHORT_NAME_MAX_SIZE);
        if PROTOCOL_REV_MAJOR((*ph).version) >= 0x2 {
            if supports_extended_names(attributes) { (*(*ph).hops).extended_name_get(ph, VOLTAGE_DOMAIN_NAME_GET as u8, (*v).id, core::ptr::null(), (*v).name.as_mut_ptr(), SCMI_MAX_STR_SIZE); }
            if supports_async_level_set(attributes) { (*v).async_level_set = true; }
        }
        scmi_voltage_levels_get(ph, v);
    }
    (*(*ph).xops).xfer_put(ph, td); 0
}

unsafe fn __scmi_voltage_get_u32(ph: *const scmi_protocol_handle, cmd_id: u8, domain_id: u32, value: *mut u32) -> c_int {
    let vinfo = (*ph).get_priv(ph) as *mut voltage_info;
    if domain_id >= (*vinfo).num_domains { return -EINVAL; }
    let mut t: *mut scmi_xfer = core::ptr::null_mut();
    let ret = (*(*ph).xops).xfer_get_init(ph, cmd_id, core::mem::size_of::<__le32>(), 0, &mut t);
    if ret != 0 { return ret; }
    put_unaligned_le32(domain_id, (*t).tx.buf);
    let ret = (*(*ph).xops).do_xfer(ph, t);
    if ret == 0 { *value = get_unaligned_le32((*t).rx.buf); }
    (*(*ph).xops).xfer_put(ph, t); ret
}

unsafe fn scmi_voltage_config_set(ph: *const scmi_protocol_handle, domain_id: u32, config: u32) -> c_int {
    let vinfo = (*ph).get_priv(ph) as *mut voltage_info;
    if domain_id >= (*vinfo).num_domains { return -EINVAL; }
    let mut t: *mut scmi_xfer = core::ptr::null_mut();
    let ret = (*(*ph).xops).xfer_get_init(ph, VOLTAGE_CONFIG_SET as u8, core::mem::size_of::<scmi_msg_cmd_config_set>(), 0, &mut t);
    if ret != 0 { return ret; }
    let cmd = (*t).tx.buf as *mut scmi_msg_cmd_config_set;
    (*cmd).domain_id = cpu_to_le32(domain_id); (*cmd).config = cpu_to_le32(config & 0xf);
    let ret = (*(*ph).xops).do_xfer(ph, t); (*(*ph).xops).xfer_put(ph, t); ret
}
unsafe fn scmi_voltage_config_get(ph: *const scmi_protocol_handle, domain_id: u32, config: *mut u32) -> c_int { __scmi_voltage_get_u32(ph, VOLTAGE_CONFIG_GET as u8, domain_id, config) }
unsafe fn scmi_voltage_level_set(ph: *const scmi_protocol_handle, domain_id: u32, mode: scmi_voltage_level_mode, volt_u_v: i32) -> c_int {
    let vi = (*ph).get_priv(ph) as *mut voltage_info; if domain_id >= (*vi).num_domains { return -EINVAL; }
    let mut t: *mut scmi_xfer = core::ptr::null_mut(); let r = (*(*ph).xops).xfer_get_init(ph, VOLTAGE_LEVEL_SET as u8, core::mem::size_of::<scmi_msg_cmd_level_set>(), 0, &mut t); if r != 0 { return r; }
    let v = (*vi).domains.add(domain_id as usize); let c = (*t).tx.buf as *mut scmi_msg_cmd_level_set; (*c).domain_id=cpu_to_le32(domain_id); (*c).voltage_level=cpu_to_le32(volt_u_v as u32);
    let r = if !(*v).async_level_set || mode != SCMI_VOLTAGE_LEVEL_SET_AUTO { (*c).flags=cpu_to_le32(0); (*(*ph).xops).do_xfer(ph,t) } else { (*c).flags=cpu_to_le32(1); (*(*ph).xops).do_xfer_with_response(ph,t) };
    (*(*ph).xops).xfer_put(ph,t); r
}
unsafe fn scmi_voltage_level_get(ph: *const scmi_protocol_handle, domain_id: u32, volt_u_v: *mut i32) -> c_int { __scmi_voltage_get_u32(ph, VOLTAGE_LEVEL_GET as u8, domain_id, volt_u_v as *mut u32) }

unsafe fn scmi_voltage_domains_num_get(ph: *const scmi_protocol_handle) -> c_int { (*((*ph).get_priv(ph) as *mut voltage_info)).num_domains as c_int }

unsafe fn scmi_voltage_info_get(ph: *const scmi_protocol_handle, domain_id: u32) -> *const scmi_voltage_info {
    let v = (*ph).get_priv(ph) as *mut voltage_info; if domain_id >= (*v).num_domains || (*(*v).domains.add(domain_id as usize)).num_levels == 0 { core::ptr::null() } else { (*v).domains.add(domain_id as usize) }
}

unsafe fn scmi_voltage_protocol_init(ph: *const scmi_protocol_handle) -> c_int {
    let v = devm_kzalloc((*ph).dev, core::mem::size_of::<voltage_info>(), GFP_KERNEL) as *mut voltage_info; if v.is_null() { return -ENOMEM; }
    let r = scmi_protocol_attributes_get(ph,v); if r != 0 { return r; }
    if (*v).num_domains != 0 { (*v).domains=devm_kcalloc((*ph).dev,(*v).num_domains as usize,core::mem::size_of::<scmi_voltage_info>(),GFP_KERNEL) as *mut scmi_voltage_info; if (*v).domains.is_null(){return -ENOMEM;} let r=scmi_voltage_descriptors_get(ph,v); if r!=0{return r;} }
    (*ph).set_priv(ph,v)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
