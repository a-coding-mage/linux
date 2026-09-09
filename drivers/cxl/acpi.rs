// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2021 Intel Corporation. All rights reserved. */
// Linux, ACPI, PCI, CXL, and allocator symbols are supplied by external dependencies.

static ACPI_CXL_QTG_ID_GUID: guid_t = GUID_INIT!(0xF365F9A6, 0xA7DE, 0x4071,
    0xA6, 0x6A, 0xB4, 0x0C, 0x0B, 0x4F, 0x8E, 0x52);
const HBIW_TO_NR_MAPS_SIZE: usize = CXL_DECODER_MAX_INTERLEAVE + 1;
static HBIW_TO_NR_MAPS: [i32; HBIW_TO_NR_MAPS_SIZE] = [0, 0, 1, 0, 2, 0, 1, 0, 3, 0, 0, 0, 2, 0, 0, 0, 4];
static VALID_HBIW: [i32; 8] = [1, 2, 3, 4, 6, 8, 12, 16];

pub unsafe fn cxl_do_xormap_calc(cximsd: *mut cxl_cxims_data, mut addr: u64, hbiw: i32) -> u64 {
    let mut nr_maps_to_apply = -1;
    let mut pos: i32;
    let mut val: u64;
    for i in 0..VALID_HBIW.len() {
        if VALID_HBIW[i] == hbiw { nr_maps_to_apply = HBIW_TO_NR_MAPS[hbiw as usize]; break; }
    }
    if nr_maps_to_apply == -1 || nr_maps_to_apply > (*cximsd).nr_maps { return u64::MAX; }
    for i in 0..(*cximsd).nr_maps as usize {
        let map = (*cximsd).xormaps[i];
        if map == 0 { continue; }
        pos = __ffs(map) as i32;
        val = (hweight64(addr & map) & 1) as u64;
        addr = (addr & !(1u64 << pos)) | (val << pos);
    }
    addr
}

unsafe fn cxl_apply_xor_maps(cxlrd: *mut cxl_root_decoder, addr: u64) -> u64 {
    let hbiw = (*cxlrd).cxlsd.nr_targets;
    if hbiw == 1 || hbiw == 3 { return addr; }
    cxl_do_xormap_calc((*cxlrd).platform_data, addr, hbiw)
}

#[repr(C)]
struct cxl_cxims_context { dev: *mut device, cxlrd: *mut cxl_root_decoder }

unsafe extern "C" fn cxl_parse_cxims(header: *mut acpi_subtable_headers, arg: *mut c_void, _end: c_ulong) -> i32 {
    let cxims = header as *mut acpi_cedt_cxims;
    let ctx = arg as *mut cxl_cxims_context;
    let cxlrd = (*ctx).cxlrd; let cxld = &mut (*cxlrd).cxlsd.cxld;
    let mut hbig = 0; let rc = eig_to_granularity((*cxims).hbig, &mut hbig);
    if rc != 0 { return rc; }
    if hbig != cxld.interleave_granularity { return 0; }
    let nr_maps = if is_power_of_2(cxld.interleave_ways) { ilog2(cxld.interleave_ways) } else { ilog2(cxld.interleave_ways / 3) };
    if (*cxims).nr_xormaps < nr_maps { dev_dbg!((*ctx).dev, "CXIMS nr_xormaps[%d] expected[%d]\n", (*cxims).nr_xormaps, nr_maps); return -ENXIO; }
    let cximsd = devm_kzalloc!((*ctx).dev, struct_size!(cximsd, xormaps, nr_maps), GFP_KERNEL);
    if cximsd.is_null() { return -ENOMEM; }
    (*cximsd).nr_maps = nr_maps;
    memcpy!((*cximsd).xormaps.as_mut_ptr(), (*cxims).xormap_list.as_ptr(), nr_maps as usize * core::mem::size_of::<u64>());
    (*cxlrd).platform_data = cximsd; 0
}

unsafe fn cfmws_to_decoder_flags(restrictions: i32) -> c_ulong {
    let mut flags = CXL_DECODER_F_ENABLE;
    if restrictions & ACPI_CEDT_CFMWS_RESTRICT_DEVMEM != 0 { flags |= CXL_DECODER_F_TYPE2; }
    if restrictions & ACPI_CEDT_CFMWS_RESTRICT_HOSTONLYMEM != 0 { flags |= CXL_DECODER_F_TYPE3; }
    if restrictions & ACPI_CEDT_CFMWS_RESTRICT_VOLATILE != 0 { flags |= CXL_DECODER_F_RAM; }
    if restrictions & ACPI_CEDT_CFMWS_RESTRICT_PMEM != 0 { flags |= CXL_DECODER_F_PMEM; }
    if restrictions & ACPI_CEDT_CFMWS_RESTRICT_FIXED != 0 { flags |= CXL_DECODER_F_LOCK; }
    flags
}

#[repr(C)] struct cxl_cfmws_context { dev: *mut device, root_port: *mut cxl_port, cxl_res: *mut resource, id: i32 }
#[repr(C)] struct cxl_chbs_context { dev: *mut device, uid: u64, base: resource_size_t, cxl_version: u32, nr_versions: i32, saved_version: u32 }

unsafe fn cxl_acpi_evaluate_qtg_dsm(handle: acpi_handle, coord: *mut access_coordinate, entries: i32, qos_class: *mut i32) -> i32 {
    if entries == 0 { return -EINVAL; }
    let in_array = [acpi_object::integer(ACPI_TYPE_INTEGER, (*coord).read_latency), acpi_object::integer(ACPI_TYPE_INTEGER, (*coord).write_latency), acpi_object::integer(ACPI_TYPE_INTEGER, (*coord).read_bandwidth), acpi_object::integer(ACPI_TYPE_INTEGER, (*coord).write_bandwidth)];
    let in_obj = acpi_object::package(&in_array);
    let out_obj = acpi_evaluate_dsm(handle, &ACPI_CXL_QTG_ID_GUID, 1, 1, &in_obj);
    if out_obj.is_null() { return -ENXIO; }
    let mut rc = 0; let pkg = &(*out_obj).package;
    if (*out_obj).type_ != ACPI_TYPE_PACKAGE || pkg.count < 1 || pkg.elements[0].type_ != ACPI_TYPE_INTEGER { rc = -ENXIO; }
    else { let max_qtg = pkg.elements[0].integer.value as u16; if pkg.count > 1 && pkg.elements[1].type_ == ACPI_TYPE_PACKAGE { let n = core::cmp::min(entries as usize, pkg.elements[1].package.count as usize); for i in 0..n { if pkg.elements[1].package.elements[i].type_ != ACPI_TYPE_INTEGER { rc = -ENXIO; break; } (*qos_class.add(i)) = pkg.elements[1].package.elements[i].integer.value as i32; } if rc == 0 { rc = n as i32; } } }
    ACPI_FREE!(out_obj); rc
}

unsafe fn cxl_acpi_qos_class(root: *mut cxl_root, coord: *mut access_coordinate, entries: i32, qos: *mut i32) -> i32 { let dev = (*root).port.uport_dev; if !dev_is_platform(dev) { return -ENODEV; } let h = ACPI_HANDLE!(dev); if h.is_null() { return -ENODEV; } cxl_acpi_evaluate_qtg_dsm(h, coord, entries, qos) }

unsafe fn cxl_acpi_probe(pdev: *mut platform_device) -> i32 {
    let host = &mut (*pdev).dev; let adev = ACPI_COMPANION!(host); let cxl_res = devm_kzalloc!(host, core::mem::size_of::<resource>(), GFP_KERNEL); if cxl_res.is_null() { return -ENOMEM; }
    (*cxl_res).name = c"CXL mem".as_ptr() as *mut i8; (*cxl_res).start = 0; (*cxl_res).end = u64::MAX; (*cxl_res).flags = IORESOURCE_MEM;
    let root = devm_cxl_add_root!(host); if IS_ERR!(root) { return PTR_ERR!(root); } (*root).ops.qos_class = Some(cxl_acpi_qos_class); cxl_setup_prm_address_translation!(root);
    let rc = bus_for_each_dev!((*adev).dev.bus, core::ptr::null_mut(), &mut (*root).port, add_host_bridge_dport); if rc < 0 { return rc; }
    let mut ctx = cxl_cfmws_context { dev: host, root_port: &mut (*root).port, cxl_res, id: 0 };
    let rc = acpi_table_parse_cedt!(ACPI_CEDT_TYPE_CFMWS, cxl_parse_cfmws, &mut ctx); if rc < 0 { return -ENXIO; }
    add_cxl_resources(cxl_res)
}

// Remaining ACPI driver registration and helper callbacks retain their external kernel interfaces.
extern "C" { fn add_host_bridge_dport(_: *mut device, _: *mut c_void) -> i32; fn cxl_parse_cfmws(_: *mut acpi_subtable_headers, _: *mut c_void, _: c_ulong) -> i32; fn add_cxl_resources(_: *mut resource) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
