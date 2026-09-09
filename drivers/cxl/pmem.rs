// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2021 Intel Corporation. All rights reserved. */

// Linux headers and CXL dependencies are supplied by the surrounding translation.

static mut EXCLUSIVE_CMDS: [usize; CXL_MEM_COMMAND_ID_MAX as usize] = [0; CXL_MEM_COMMAND_ID_MAX as usize];

pub unsafe fn devm_cxl_add_nvdimm_bridge(host: *mut device, port: *mut cxl_port) -> *mut cxl_nvdimm_bridge {
    __devm_cxl_add_nvdimm_bridge(host, port)
}

unsafe fn clear_exclusive(mds: *mut core::ffi::c_void) {
    clear_exclusive_cxl_commands(mds, EXCLUSIVE_CMDS.as_mut_ptr());
}

unsafe fn unregister_nvdimm(nvdimm: *mut core::ffi::c_void) { nvdimm_delete(nvdimm); }

unsafe fn provider_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let nvdimm = to_nvdimm(dev);
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    sysfs_emit(buf, "%s\n", dev_name(&mut (*cxl_nvd).dev))
}

unsafe fn id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let nvdimm = to_nvdimm(dev);
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    let cxlds = (*(*cxl_nvd).cxlmd).cxlds;
    sysfs_emit(buf, "%llu\n", (*cxlds).serial)
}

unsafe fn dirty_shutdown_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let nvdimm = to_nvdimm(dev);
    let cxl_nvd = nvdimm_provider_data(nvdimm);
    sysfs_emit(buf, "%llu\n", (*cxl_nvd).dirty_shutdowns)
}

static mut CXL_DIMM_ATTRIBUTES: [*mut attribute; 4] = [
    &mut dev_attr_id.attr, &mut dev_attr_provider.attr, &mut dev_attr_dirty_shutdown.attr, core::ptr::null_mut(),
];
const CXL_INVALID_DIRTY_SHUTDOWN_COUNT: u64 = u64::MAX;

unsafe fn cxl_dimm_visible(kobj: *mut kobject, a: *mut attribute, _n: i32) -> umode_t {
    if a == &mut dev_attr_dirty_shutdown.attr {
        let dev = kobj_to_dev(kobj);
        let cxl_nvd = nvdimm_provider_data(to_nvdimm(dev));
        if (*cxl_nvd).dirty_shutdowns == CXL_INVALID_DIRTY_SHUTDOWN_COUNT { return 0; }
    }
    (*a).mode
}

unsafe fn cxl_nvdimm_arm_dirty_shutdown_tracking(cxl_nvd: *mut cxl_nvdimm) {
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxlds = (*cxlmd).cxlds;
    let mds = to_cxl_memdev_state(cxlds);
    let dev = &mut (*cxl_nvd).dev;
    let mut count: u32 = 0;
    (*cxl_nvd).dirty_shutdowns = CXL_INVALID_DIRTY_SHUTDOWN_COUNT;
    if cxl_arm_dirty_shutdown(mds) != 0 { dev_warn(dev, "GPF: could not set dirty shutdown state\n"); return; }
    if cxl_gpf_get_dvsec((*cxlds).dev).is_null() { return; }
    if cxl_get_dirty_count(mds, &mut count) != 0 { dev_warn(dev, "GPF: could not retrieve dirty count\n"); return; }
    (*cxl_nvd).dirty_shutdowns = count as u64;
}

unsafe fn cxl_nvdimm_probe(dev: *mut device) -> i32 {
    let cxl_nvd = to_cxl_nvdimm(dev);
    let cxlmd = (*cxl_nvd).cxlmd;
    let cxl_nvb = (*cxlmd).cxl_nvb;
    let mds = to_cxl_memdev_state((*cxlmd).cxlds);
    let mut flags: usize = 0; let mut cmd_mask: usize = 0;
    if test_bit(CXL_NVD_F_INVALIDATED, &(*cxl_nvd).flags) != 0 { return -EBUSY; }
    set_exclusive_cxl_commands(mds, EXCLUSIVE_CMDS.as_mut_ptr());
    let rc = devm_add_action_or_reset(dev, clear_exclusive, mds as *mut _);
    if rc != 0 { return rc; }
    set_bit(NDD_LABELING, &mut flags); set_bit(NDD_REGISTER_SYNC, &mut flags);
    set_bit(ND_CMD_GET_CONFIG_SIZE, &mut cmd_mask); set_bit(ND_CMD_GET_CONFIG_DATA, &mut cmd_mask); set_bit(ND_CMD_SET_CONFIG_DATA, &mut cmd_mask);
    cxl_nvdimm_arm_dirty_shutdown_tracking(cxl_nvd);
    let nvdimm = __nvdimm_create((*cxl_nvb).nvdimm_bus, cxl_nvd, CXL_DIMM_ATTRIBUTE_GROUPS.as_ptr(), flags, cmd_mask, 0, core::ptr::null_mut(), (*cxl_nvd).dev_id, cxl_security_ops, core::ptr::null_mut());
    if nvdimm.is_null() { return -ENOMEM; }
    dev_set_drvdata(dev, nvdimm);
    devm_add_action_or_reset(dev, unregister_nvdimm, nvdimm as *mut _)
}

unsafe fn cxl_pmem_get_config_size(mds: *mut cxl_memdev_state, cmd: *mut nd_cmd_get_config_size, buf_len: u32) -> i32 {
    let mbox = &mut (*(*mds).cxlds).cxl_mbox;
    if core::mem::size_of::<nd_cmd_get_config_size>() as u32 > buf_len { return -EINVAL; }
    (*cmd).config_size = (*mds).lsa_size;
    (*cmd).max_xfer = (*mbox).payload_size - core::mem::size_of::<cxl_mbox_set_lsa>(); 0
}

unsafe fn cxl_pmem_get_config_data(mds: *mut cxl_memdev_state, cmd: *mut nd_cmd_get_config_data_hdr, buf_len: u32) -> i32 {
    let mbox = &mut (*(*mds).cxlds).cxl_mbox;
    if core::mem::size_of::<nd_cmd_get_config_data_hdr>() as u32 > buf_len { return -EINVAL; }
    if struct_size(cmd, out_buf, (*cmd).in_length) > buf_len { return -EINVAL; }
    let mut get_lsa = cxl_mbox_get_lsa { offset: cpu_to_le32((*cmd).in_offset), length: cpu_to_le32((*cmd).in_length) };
    let mut mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_GET_LSA, payload_in: &mut get_lsa as *mut _, size_in: core::mem::size_of_val(&get_lsa), size_out: (*cmd).in_length, payload_out: (*cmd).out_buf };
    let rc = cxl_internal_send_cmd(mbox, &mut mbox_cmd); (*cmd).status = 0; rc
}

unsafe fn cxl_pmem_set_config_data(mds: *mut cxl_memdev_state, cmd: *mut nd_cmd_set_config_hdr, buf_len: u32) -> i32 {
    let mbox = &mut (*(*mds).cxlds).cxl_mbox;
    if core::mem::size_of::<nd_cmd_set_config_hdr>() as u32 > buf_len { return -EINVAL; }
    if size_add(struct_size(cmd, in_buf, (*cmd).in_length), 4) > buf_len { return -EINVAL; }
    let set_lsa = kvzalloc_flex::<cxl_mbox_set_lsa>((*cmd).in_length);
    if set_lsa.is_null() { return -ENOMEM; }
    (*set_lsa).offset = cpu_to_le32((*cmd).in_offset);
    core::ptr::copy_nonoverlapping((*cmd).in_buf, (*set_lsa).data.as_mut_ptr(), (*cmd).in_length as usize);
    let mut mbox_cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_SET_LSA, payload_in: set_lsa as *mut _, size_in: struct_size(set_lsa, data, (*cmd).in_length), size_out: 0, payload_out: core::ptr::null_mut() };
    let rc = cxl_internal_send_cmd(mbox, &mut mbox_cmd);
    put_unaligned(0u32, (*cmd).in_buf.add((*cmd).in_length as usize) as *mut u32); kvfree(set_lsa as *mut _); rc
}

unsafe fn cxl_pmem_nvdimm_ctl(nvdimm: *mut nvdimm, cmd: u32, buf: *mut core::ffi::c_void, len: u32) -> i32 {
    let cxl_nvd = nvdimm_provider_data(nvdimm); let mask = nvdimm_cmd_mask(nvdimm); let mds = to_cxl_memdev_state((*(*cxl_nvd).cxlmd).cxlds);
    if test_bit(cmd, &mask) == 0 { return -ENOTTY; }
    match cmd { ND_CMD_GET_CONFIG_SIZE => cxl_pmem_get_config_size(mds, buf as *mut _, len), ND_CMD_GET_CONFIG_DATA => cxl_pmem_get_config_data(mds, buf as *mut _, len), ND_CMD_SET_CONFIG_DATA => cxl_pmem_set_config_data(mds, buf as *mut _, len), _ => -ENOTTY }
}

unsafe fn cxl_pmem_ctl(_desc: *mut nvdimm_bus_descriptor, nvdimm: *mut nvdimm, cmd: u32, buf: *mut core::ffi::c_void, len: u32, cmd_rc: *mut i32) -> i32 {
    *cmd_rc = 0; if nvdimm.is_null() { -ENOTTY } else { cxl_pmem_nvdimm_ctl(nvdimm, cmd, buf, len) }
}

#[repr(C)]
struct cxl_pmem_region_info { offset: u64, serial: u64 }

unsafe fn unregister_nvdimm_region(region: *mut core::ffi::c_void) { nvdimm_region_delete(region); }
unsafe fn cxlr_pmem_remove_resource(res: *mut resource) { remove_resource(res); }

unsafe fn cxl_pmem_region_probe(dev: *mut device) -> i32 {
    let cxlr_pmem = to_cxl_pmem_region(dev); let cxlr = (*cxlr_pmem).cxlr;
    let cxl_nvb = (*cxlr).cxl_nvb; let mut mappings = [core::mem::zeroed::<nd_mapping_desc>(); CXL_DECODER_MAX_INTERLEAVE as usize];
    let mut desc: nd_region_desc = core::mem::zeroed();
    let res = devm_kzalloc(dev, core::mem::size_of::<resource>(), GFP_KERNEL);
    if res.is_null() { return -ENOMEM; }
    (*res).name = "Persistent Memory\0".as_ptr() as *mut i8; (*res).start = (*cxlr_pmem).hpa_range.start; (*res).end = (*cxlr_pmem).hpa_range.end; (*res).flags = IORESOURCE_MEM; (*res).desc = IORES_DESC_PERSISTENT_MEMORY;
    let mut rc = insert_resource(&mut iomem_resource, res); if rc != 0 { return rc; }
    rc = devm_add_action_or_reset(dev, cxlr_pmem_remove_resource, res); if rc != 0 { return rc; }
    desc.res = res; desc.provider_data = cxlr_pmem as *mut _; desc.numa_node = memory_add_physaddr_to_nid((*res).start); desc.target_node = phys_to_target_node((*res).start);
    if desc.target_node == NUMA_NO_NODE { desc.target_node = desc.numa_node; dev_dbg(&mut (*cxlr).dev, "changing target node from %d to %d\n", NUMA_NO_NODE, desc.target_node); }
    let nd_set = devm_kzalloc(dev, core::mem::size_of::<nd_interleave_set>()) as *mut nd_interleave_set; if nd_set.is_null() { return -ENOMEM; }
    desc.memregion = (*cxlr_pmem).id; set_bit(ND_REGION_CXL, &mut desc.flags); set_bit(ND_REGION_PERSIST_MEMCTRL, &mut desc.flags);
    let info = kmalloc_objs::<cxl_pmem_region_info>((*cxlr_pmem).nr_mappings); if info.is_null() { return -ENOMEM; }
    for i in 0..(*cxlr_pmem).nr_mappings as usize { let m = &mut *(*cxlr_pmem).mapping.add(i); let cxlmd = m.cxlmd; let cxlds = (*cxlmd).cxlds; let cxl_nvd = (*cxlmd).cxl_nvd; let nvdimm = dev_get_drvdata(&mut (*cxl_nvd).dev); if nvdimm.is_null() { kfree(info as *mut _); return -ENODEV; } if (*cxlds).serial == 0 { kfree(info as *mut _); return -ENXIO; } (*info.add(i)).serial = (*cxlds).serial; (*info.add(i)).offset = m.start; m.cxl_nvd = cxl_nvd; mappings[i] = nd_mapping_desc { nvdimm, start: m.start, size: m.size, position: i as u64 }; }
    desc.num_mappings = (*cxlr_pmem).nr_mappings; desc.mapping = mappings.as_mut_ptr(); (*nd_set).cookie1 = nd_fletcher64(info as *mut _, core::mem::size_of::<cxl_pmem_region_info>() * (*cxlr_pmem).nr_mappings as usize, 0); (*nd_set).cookie2 = (*nd_set).cookie1; desc.nd_set = nd_set;
    (*cxlr_pmem).nd_region = nvdimm_pmem_region_create((*cxl_nvb).nvdimm_bus, &mut desc); if (*cxlr_pmem).nd_region.is_null() { kfree(info as *mut _); return -ENOMEM; } rc = devm_add_action_or_reset(dev, unregister_nvdimm_region, (*cxlr_pmem).nd_region as *mut _); kfree(info as *mut _); rc
}

unsafe fn cxl_pmem_init() -> i32 { set_bit(CXL_MEM_COMMAND_ID_SET_SHUTDOWN_STATE, EXCLUSIVE_CMDS.as_mut_ptr()); set_bit(CXL_MEM_COMMAND_ID_SET_LSA, EXCLUSIVE_CMDS.as_mut_ptr()); let mut rc = cxl_driver_register(&mut cxl_nvdimm_bridge_driver); if rc != 0 { return rc; } rc = cxl_driver_register(&mut cxl_nvdimm_driver); if rc != 0 { cxl_driver_unregister(&mut cxl_nvdimm_bridge_driver); return rc; } rc = cxl_driver_register(&mut cxl_pmem_region_driver); if rc != 0 { cxl_driver_unregister(&mut cxl_nvdimm_driver); cxl_driver_unregister(&mut cxl_nvdimm_bridge_driver); } rc }
unsafe fn cxl_pmem_exit() { cxl_driver_unregister(&mut cxl_pmem_region_driver); cxl_driver_unregister(&mut cxl_nvdimm_driver); cxl_driver_unregister(&mut cxl_nvdimm_bridge_driver); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
