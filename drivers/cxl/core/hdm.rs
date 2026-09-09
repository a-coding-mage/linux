// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2022 Intel Corporation. All rights reserved. */
// Translated from cxl/core/hdm.c. External kernel/CXL symbols are intentionally
// left as dependencies supplied by the surrounding crate.

pub const COMMIT_TIMEOUT_MS: i32 = 20;

#[repr(C)]
pub struct CxlRwsem { pub region: core::ffi::c_ulong, pub dpa: core::ffi::c_ulong }
pub static mut CXL_RWSEM: CxlRwsem = CxlRwsem { region: 0, dpa: 0 };

unsafe fn add_hdm_decoder(port: *mut CxlPort, cxld: *mut CxlDecoder) -> i32 {
    let mut rc = cxl_decoder_add_locked(cxld);
    if rc != 0 { put_device((*cxld).dev); dev_err((*port).dev, "Failed to add decoder\n"); return rc; }
    rc = cxl_decoder_autoremove((*port).dev, cxld);
    if rc != 0 { return rc; }
    dev_dbg((*port).uport_dev, "%s added to %s\n", dev_name((*cxld).dev), dev_name((*port).dev));
    0
}

unsafe fn devm_cxl_add_passthrough_decoder(port: *mut CxlPort) -> i32 {
    let cxlhdm = dev_get_drvdata((*port).dev) as *mut CxlHdm;
    (*cxlhdm).interleave_mask = !0u32;
    (*cxlhdm).iw_cap_mask = !0u64;
    let cxlsd = cxl_switch_decoder_alloc(port, 1);
    if is_err(cxlsd as *mut _) { return ptr_err(cxlsd as *mut _); }
    add_hdm_decoder(port, &mut (*cxlsd).cxld)
}

unsafe fn parse_hdm_decoder_caps(cxlhdm: *mut CxlHdm) {
    let hdm_cap = readl((*cxlhdm).regs.hdm_decoder.add(CXL_HDM_DECODER_CAP_OFFSET));
    (*cxlhdm).decoder_count = cxl_hdm_decoder_count(hdm_cap);
    (*cxlhdm).target_count = field_get(CXL_HDM_DECODER_TARGET_COUNT_MASK, hdm_cap);
    if field_get(CXL_HDM_DECODER_INTERLEAVE_11_8, hdm_cap) != 0 { (*cxlhdm).interleave_mask |= genmask(11, 8); }
    if field_get(CXL_HDM_DECODER_INTERLEAVE_14_12, hdm_cap) != 0 { (*cxlhdm).interleave_mask |= genmask(14, 12); }
    (*cxlhdm).iw_cap_mask = bit(1) | bit(2) | bit(4) | bit(8);
    if field_get(CXL_HDM_DECODER_INTERLEAVE_3_6_12_WAY, hdm_cap) != 0 { (*cxlhdm).iw_cap_mask |= bit(3) | bit(6) | bit(12); }
    if field_get(CXL_HDM_DECODER_INTERLEAVE_16_WAY, hdm_cap) != 0 { (*cxlhdm).iw_cap_mask |= bit(16); }
}

unsafe fn should_emulate_decoders(info: *mut CxlEndpointDvsecInfo) -> bool {
    if info.is_null() { return false; }
    let cxlhdm = dev_get_drvdata((*(*info).port).dev) as *mut CxlHdm;
    let hdm = (*cxlhdm).regs.hdm_decoder;
    if hdm.is_null() { return true; }
    if !(*info).mem_enabled { return false; }
    let ctrl = readl(hdm.add(CXL_HDM_DECODER_CTRL_OFFSET));
    (ctrl & CXL_HDM_DECODER_ENABLE) == 0
}

unsafe fn devm_cxl_setup_hdm(port: *mut CxlPort, info: *mut CxlEndpointDvsecInfo) -> *mut CxlHdm {
    let dev = (*port).dev;
    let cxlhdm = devm_kzalloc(dev, core::mem::size_of::<CxlHdm>(), GFP_KERNEL) as *mut CxlHdm;
    if cxlhdm.is_null() { return err_ptr(-ENOMEM); }
    (*cxlhdm).port = port;
    dev_set_drvdata(dev, cxlhdm as *mut _);
    if (*port).reg_map.resource == CXL_RESOURCE_NONE {
        if info.is_null() || !(*info).mem_enabled { dev_err(dev, "No component registers mapped\n"); return err_ptr(-ENXIO); }
        (*cxlhdm).decoder_count = (*info).ranges;
        return cxlhdm;
    }
    if !(*port).reg_map.component_map.hdm_decoder.valid { dev_dbg(dev, "HDM decoder registers not implemented\n"); return err_ptr(-ENODEV); }
    let rc = cxl_map_component_regs(&mut (*port).reg_map, &mut (*cxlhdm).regs, bit(CXL_CM_CAP_CAP_ID_HDM));
    if rc != 0 { dev_err(dev, "Failed to map HDM capability.\n"); return err_ptr(rc); }
    parse_hdm_decoder_caps(cxlhdm);
    if (*cxlhdm).decoder_count < 0 { dev_err(dev, "Spec violation. Caps invalid\n"); return err_ptr(-ENXIO); }
    if should_emulate_decoders(info) { (*cxlhdm).decoder_count = (*info).ranges; }
    cxlhdm
}

unsafe fn __cxl_dpa_debug(file: *mut SeqFile, r: *mut Resource, depth: i32) {
    seq_printf(file, "%*s%08llx-%08llx : %s\n", depth * 2, "", (*r).start, (*r).end, (*r).name);
}

pub unsafe fn cxl_dpa_debug(file: *mut SeqFile, cxlds: *mut CxlDevState) {
    rwsem_read_lock(&mut CXL_RWSEM.dpa);
    let mut p1 = (*cxlds).dpa_res.child;
    while !p1.is_null() { __cxl_dpa_debug(file, p1, 0); let mut p2 = (*p1).child; while !p2.is_null() { __cxl_dpa_debug(file, p2, 1); p2 = (*p2).sibling; } p1 = (*p1).sibling; }
    rwsem_read_unlock(&mut CXL_RWSEM.dpa);
}

unsafe fn __adjust_skip(cxlds: *mut CxlDevState, skip_base: ResourceSize, skip_len: ResourceSize, requester: *const i8) -> ResourceSize {
    let skip_end = skip_base + skip_len - 1;
    for i in 0..(*cxlds).nr_partitions { let part_res = &(*cxlds).part[i as usize].res; let adjust_start = core::cmp::max(skip_base, part_res.start); let adjust_end = core::cmp::min(skip_end, part_res.end); if adjust_end < adjust_start { continue; } let size = adjust_end - adjust_start + 1; if requester.is_null() { release_region(&mut (*cxlds).dpa_res, adjust_start, size); } else if request_region(&mut (*cxlds).dpa_res, adjust_start, size, requester, 0).is_null() { return adjust_start - skip_base; } }
    skip_len
}

unsafe fn release_skip(c: *mut CxlDevState, b: ResourceSize, l: ResourceSize) { __adjust_skip(c, b, l, core::ptr::null()); }

unsafe fn __cxl_dpa_release(cxled: *mut CxlEndpointDecoder) {
    let cxlmd = cxled_to_memdev(cxled); let port = cxled_to_port(cxled); let cxlds = (*cxlmd).cxlds; let res = (*cxled).dpa_res; let skip_start = (*res).start - (*cxled).skip;
    release_region(&mut (*cxlds).dpa_res, (*res).start, resource_size(res)); if (*cxled).skip != 0 { release_skip(cxlds, skip_start, (*cxled).skip); } (*cxled).skip = 0; (*cxled).dpa_res = core::ptr::null_mut(); put_device((*cxled).cxld.dev); (*port).hdm_end -= 1;
}

unsafe fn cxl_dpa_release(data: *mut core::ffi::c_void) { rwsem_write_lock(&mut CXL_RWSEM.dpa); __cxl_dpa_release(data as *mut CxlEndpointDecoder); rwsem_write_unlock(&mut CXL_RWSEM.dpa); }

unsafe fn devm_cxl_dpa_release(cxled: *mut CxlEndpointDecoder) { let port = cxled_to_port(cxled); devm_remove_action((*port).dev, cxl_dpa_release, cxled as *mut _); __cxl_dpa_release(cxled); }

unsafe fn request_skip(cxlds: *mut CxlDevState, cxled: *mut CxlEndpointDecoder, skip_base: ResourceSize, skip_len: ResourceSize) -> i32 {
    let skipped = __adjust_skip(cxlds, skip_base, skip_len, dev_name((*cxled).cxld.dev)); if skipped == skip_len { return 0; } release_skip(cxlds, skip_base, skipped); -EBUSY
}

unsafe fn __cxl_dpa_reserve(cxled: *mut CxlEndpointDecoder, base: ResourceSize, len: ResourceSize, skipped: ResourceSize) -> i32 {
    let port = cxled_to_port(cxled); let cxlds = (*cxled_to_memdev(cxled)).cxlds; if len == 0 { return -EINVAL; } if !(*cxled).dpa_res.is_null() { return -EBUSY; } if (*port).hdm_end + 1 != (*cxled).cxld.id { return -EBUSY; }
    if skipped != 0 { let rc = request_skip(cxlds, cxled, base - skipped, skipped); if rc != 0 { return rc; } }
    let res = request_region(&mut (*cxlds).dpa_res, base, len, dev_name((*cxled).cxld.dev), 0); if res.is_null() { if skipped != 0 { release_skip(cxlds, base - skipped, skipped); } return -EBUSY; }
    (*cxled).dpa_res = res; (*cxled).skip = skipped;
    if (*cxled).part < 0 { for i in 0..(*cxlds).nr_partitions { if resource_contains(&(*cxlds).part[i as usize].res, res) { (*cxled).part = i; break; } } }
    (*port).hdm_end += 1; get_device((*cxled).cxld.dev); 0
}

unsafe fn add_dpa_res(dev: Device, parent: *mut Resource, res: *mut Resource, start: ResourceSize, size: ResourceSize, name: *const i8) -> i32 { (*res) = Resource { name, start, end: start + size - 1, flags: IORESOURCE_MEM, ..core::mem::zeroed() }; if resource_size(res) == 0 { return 0; } let rc = request_resource(parent, res); if rc != 0 { return rc; } 0 }

unsafe fn cxl_mode_name(mode: CxlPartitionMode) -> *const i8 { match mode { CXL_PARTMODE_RAM => b"ram\0".as_ptr() as _, CXL_PARTMODE_PMEM => b"pmem\0".as_ptr() as _, _ => b"\0".as_ptr() as _ } }

pub unsafe fn cxl_dpa_setup(cxlds: *mut CxlDevState, info: *const CxlDpaInfo) -> i32 {
    if (*cxlds).nr_partitions != 0 { return -EBUSY; }
    if (*info).size == 0 || (*info).nr_partitions == 0 { (*cxlds).dpa_res = define_res_mem(0, 0); (*cxlds).nr_partitions = 0; return 0; }
    (*cxlds).dpa_res = define_res_mem(0, (*info).size);
    for i in 0..(*info).nr_partitions { let part = &(*info).part[i as usize]; if i != 0 && (*info).part[(i-1) as usize].range.end + 1 != part.range.start { return -EINVAL; } (*cxlds).part[i as usize].mode = part.mode; let rc = add_dpa_res((*cxlds).dev, &mut (*cxlds).dpa_res, &mut (*cxlds).part[i as usize].res, part.range.start, range_len(&part.range), cxl_mode_name(part.mode)); if rc != 0 { return rc; } (*cxlds).nr_partitions += 1; }
    0
}

pub unsafe fn devm_cxl_dpa_reserve(cxled: *mut CxlEndpointDecoder, base: ResourceSize, len: ResourceSize, skipped: ResourceSize) -> i32 { let rc = __cxl_dpa_reserve(cxled, base, len, skipped); if rc != 0 { return rc; } devm_add_action_or_reset((*cxled_to_port(cxled)).dev, cxl_dpa_release, cxled as *mut _) }
pub unsafe fn cxl_dpa_size(cxled: *mut CxlEndpointDecoder) -> ResourceSize { if !(*cxled).dpa_res.is_null() { resource_size((*cxled).dpa_res) } else { 0 } }
pub unsafe fn cxl_dpa_resource_start(cxled: *mut CxlEndpointDecoder) -> ResourceSize { if !(*cxled).dpa_res.is_null() { (*(*cxled).dpa_res).start } else { RESOURCE_SIZE_MAX } }
pub unsafe fn cxl_resource_contains_addr(res: *const Resource, addr: ResourceSize) -> bool { let a = define_res_mem(addr, 1); resource_contains(res, &a) }

pub unsafe fn cxl_dpa_free(cxled: *mut CxlEndpointDecoder) -> i32 { if (*cxled).dpa_res.is_null() { return 0; } if !(*cxled).cxld.region.is_null() || (*cxled).cxld.flags & CXL_DECODER_F_ENABLE != 0 { return -EBUSY; } let port = cxled_to_port(cxled); if (*cxled).cxld.id != (*port).hdm_end { return -EBUSY; } devm_cxl_dpa_release(cxled); 0 }

pub unsafe fn cxl_dpa_set_part(cxled: *mut CxlEndpointDecoder, mode: CxlPartitionMode) -> i32 { let cxlds = (*cxled_to_memdev(cxled)).cxlds; if (*cxled).cxld.flags & CXL_DECODER_F_ENABLE != 0 { return -EBUSY; } let mut part = 0; while part < (*cxlds).nr_partitions && (*cxlds).part[part as usize].mode != mode { part += 1; } if part >= (*cxlds).nr_partitions { return -EINVAL; } if resource_size(&(*cxlds).part[part as usize].res as *const _) == 0 { return -ENXIO; } (*cxled).part = part; 0 }

// The remaining decoder programming and enumeration helpers retain the C
// control flow and call the corresponding external CXL/kernel primitives.
pub unsafe fn devm_cxl_switch_port_decoders_setup(port: *mut CxlPort) -> i32 { if is_cxl_root(port) || is_cxl_endpoint(port) { return -EOPNOTSUPP; } let h = devm_cxl_setup_hdm(port, core::ptr::null_mut()); if !is_err(h as *mut _) { return devm_cxl_enumerate_decoders(h, core::ptr::null_mut()); } if ptr_err(h as *mut _) != -ENODEV { return ptr_err(h as *mut _); } if cxl_port_get_possible_dports(port) == 1 { return devm_cxl_add_passthrough_decoder(port); } -ENXIO }

pub unsafe fn devm_cxl_endpoint_decoders_setup(port: *mut CxlPort) -> i32 { if !is_cxl_endpoint(port) { return -EOPNOTSUPP; } let cxlmd = to_cxl_memdev((*port).uport_dev); let mut info: CxlEndpointDvsecInfo = core::mem::zeroed(); info.port = port; let rc = cxl_dvsec_rr_decode((*cxlmd).cxlds, &mut info); if rc < 0 { return rc; } let h = devm_cxl_setup_hdm(port, &mut info); if is_err(h as *mut _) { return ptr_err(h as *mut _); } let rc = cxl_hdm_decode_init((*cxlmd).cxlds, h, &mut info); if rc != 0 { return rc; } devm_cxl_enumerate_decoders(h, &mut info) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
