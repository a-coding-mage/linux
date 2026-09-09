// SPDX-License-Identifier: GPL-2.0
/*
 * ACPI Platform Firmware Runtime Update Device driver
 *
 * Copyright (C) 2021 Intel Corporation
 * Author: Chen Yu <yu.c.chen@intel.com>
 *
 * pfru_update driver is used for Platform Firmware Runtime
 * Update, which includes the code injection and driver update.
 */

const PFRU_FUNC_STANDARD_QUERY: u32 = 0;
const PFRU_FUNC_QUERY_UPDATE_CAP: u32 = 1;
const PFRU_FUNC_QUERY_BUF: u32 = 2;
const PFRU_FUNC_START: u32 = 3;

const PFRU_CODE_INJECT_TYPE: i32 = 1;
const PFRU_DRIVER_UPDATE_TYPE: i32 = 2;

const PFRU_REVID_1: u32 = 1;
const PFRU_REVID_2: u32 = 2;
const PFRU_DEFAULT_REV_ID: u32 = PFRU_REVID_1;

#[repr(u32)]
enum CapIndex {
    CapStatusIdx = 0,
    CapUpdateIdx = 1,
    CapCodeTypeIdx = 2,
    CapFwVerIdx = 3,
    CapCodeRtVerIdx = 4,
    CapDrvTypeIdx = 5,
    CapDrvRtVerIdx = 6,
    CapDrvSvnIdx = 7,
    CapPlatIdIdx = 8,
    CapOemIdIdx = 9,
    CapOemInfoIdx = 10,
    CapNrIdx,
}

#[repr(u32)]
enum BufIndex { BufStatusIdx = 0, BufExtStatusIdx, BufAddrLowIdx, BufAddrHiIdx, BufSizeIdx, BufNrIdx }

#[repr(u32)]
enum UpdateIndex { UpdateStatusIdx = 0, UpdateExtStatusIdx, UpdateAuthTimeLowIdx, UpdateAuthTimeHiIdx, UpdateExecTimeLowIdx, UpdateExecTimeHiIdx, UpdateNrIdx }

#[repr(u32)]
enum PfruStartAction { StartStage = 0, StartActivate = 1, StartStageActivate = 2 }

#[repr(C)]
struct PfruDevice {
    rev_id: u32,
    index: u32,
    parent_dev: *mut device,
    miscdev: miscdevice,
}

static mut PFRU_IDA: ida = ida::default();

/*
 * Manual reference:
 * https://uefi.org/sites/default/files/resources/Intel_MM_OS_Interface_Spec_Rev100.pdf
 *
 * pfru_guid is the parameter for _DSM method
 */
static PFRU_GUID: guid_t = GUID_INIT(0xECF9533B, 0x4A3C, 0x4E89, 0x93, 0x9E, 0xC7, 0x71, 0x12, 0x60, 0x1C, 0x6D);

/* pfru_code_inj_guid is the UUID to identify code injection EFI capsule file */
static PFRU_CODE_INJ_GUID: guid_t = GUID_INIT(0xB2F84B79, 0x7B6E, 0x4E45, 0x88, 0x5F, 0x3F, 0xB9, 0xBB, 0x18, 0x54, 0x02);

/* pfru_drv_update_guid is the UUID to identify driver update EFI capsule file */
static PFRU_DRV_UPDATE_GUID: guid_t = GUID_INIT(0x4569DD8C, 0x75F1, 0x429A, 0xA3, 0xD6, 0x24, 0xDE, 0x80, 0x97, 0xA0, 0xDF);

#[inline]
unsafe fn pfru_valid_revid(id: u32) -> bool { id == PFRU_REVID_1 || id == PFRU_REVID_2 }

#[inline]
unsafe fn to_pfru_dev(file: *mut file) -> *mut PfruDevice {
    container_of((*file).private_data, PfruDevice, miscdev)
}

unsafe fn query_capability(cap_hdr: *mut pfru_update_cap_info, pfru_dev: *mut PfruDevice) -> i32 {
    let handle = ACPI_HANDLE((*pfru_dev).parent_dev);
    let mut out_obj: *mut acpi_object = acpi_evaluate_dsm_typed(handle, &PFRU_GUID, (*pfru_dev).rev_id, PFRU_FUNC_QUERY_UPDATE_CAP, core::ptr::null_mut(), ACPI_TYPE_PACKAGE);
    if out_obj.is_null() { dev_dbg((*pfru_dev).parent_dev, "Query cap failed with no object\n"); return -EINVAL; }
    let elems = (*out_obj).package.elements;
    if (*out_obj).package.count < CapNrIdx as u32 || (*elems.add(CapStatusIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*elems.add(CapUpdateIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*elems.add(CapCodeTypeIdx as usize)).type_ != ACPI_TYPE_BUFFER || (*elems.add(CapFwVerIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*elems.add(CapCodeRtVerIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*elems.add(CapDrvTypeIdx as usize)).type_ != ACPI_TYPE_BUFFER || (*elems.add(CapDrvRtVerIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*elems.add(CapDrvSvnIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*elems.add(CapPlatIdIdx as usize)).type_ != ACPI_TYPE_BUFFER || (*elems.add(CapOemIdIdx as usize)).type_ != ACPI_TYPE_BUFFER || (*elems.add(CapOemInfoIdx as usize)).type_ != ACPI_TYPE_BUFFER { dev_dbg((*pfru_dev).parent_dev, "Query cap failed with invalid package count/type\n"); ACPI_FREE(out_obj); return -EINVAL; }
    (*cap_hdr).status = (*elems.add(CapStatusIdx as usize)).integer.value;
    if (*cap_hdr).status != DSM_SUCCEED { dev_dbg((*pfru_dev).parent_dev, "Query cap Error Status:%d\n", (*cap_hdr).status); ACPI_FREE(out_obj); return -EBUSY; }
    if (*elems.add(CapCodeTypeIdx as usize)).buffer.length > core::mem::size_of_val(&(*cap_hdr).code_type) || (*elems.add(CapDrvTypeIdx as usize)).buffer.length > core::mem::size_of_val(&(*cap_hdr).drv_type) || (*elems.add(CapPlatIdIdx as usize)).buffer.length > core::mem::size_of_val(&(*cap_hdr).platform_id) || (*elems.add(CapOemIdIdx as usize)).buffer.length > core::mem::size_of_val(&(*cap_hdr).oem_id) { ACPI_FREE(out_obj); return -EINVAL; }
    (*cap_hdr).update_cap = (*elems.add(CapUpdateIdx as usize)).integer.value;
    core::ptr::copy_nonoverlapping((*elems.add(CapCodeTypeIdx as usize)).buffer.pointer, &mut (*cap_hdr).code_type as *mut _ as *mut u8, (*elems.add(CapCodeTypeIdx as usize)).buffer.length as usize);
    (*cap_hdr).fw_version = (*elems.add(CapFwVerIdx as usize)).integer.value;
    (*cap_hdr).code_rt_version = (*elems.add(CapCodeRtVerIdx as usize)).integer.value;
    core::ptr::copy_nonoverlapping((*elems.add(CapDrvTypeIdx as usize)).buffer.pointer, &mut (*cap_hdr).drv_type as *mut _ as *mut u8, (*elems.add(CapDrvTypeIdx as usize)).buffer.length as usize);
    (*cap_hdr).drv_rt_version = (*elems.add(CapDrvRtVerIdx as usize)).integer.value;
    (*cap_hdr).drv_svn = (*elems.add(CapDrvSvnIdx as usize)).integer.value;
    core::ptr::copy_nonoverlapping((*elems.add(CapPlatIdIdx as usize)).buffer.pointer, &mut (*cap_hdr).platform_id as *mut _ as *mut u8, (*elems.add(CapPlatIdIdx as usize)).buffer.length as usize);
    core::ptr::copy_nonoverlapping((*elems.add(CapOemIdIdx as usize)).buffer.pointer, &mut (*cap_hdr).oem_id as *mut _ as *mut u8, (*elems.add(CapOemIdIdx as usize)).buffer.length as usize);
    (*cap_hdr).oem_info_len = (*elems.add(CapOemInfoIdx as usize)).buffer.length;
    ACPI_FREE(out_obj); 0
}

unsafe fn query_buffer(info: *mut pfru_com_buf_info, pfru_dev: *mut PfruDevice) -> i32 {
    let out_obj = acpi_evaluate_dsm_typed(ACPI_HANDLE((*pfru_dev).parent_dev), &PFRU_GUID, (*pfru_dev).rev_id, PFRU_FUNC_QUERY_BUF, core::ptr::null_mut(), ACPI_TYPE_PACKAGE);
    if out_obj.is_null() { dev_dbg((*pfru_dev).parent_dev, "Query buf failed with no object\n"); return -EINVAL; }
    let e = (*out_obj).package.elements;
    if (*out_obj).package.count < BufNrIdx as u32 || (*e.add(BufStatusIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*e.add(BufExtStatusIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*e.add(BufAddrLowIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*e.add(BufAddrHiIdx as usize)).type_ != ACPI_TYPE_INTEGER || (*e.add(BufSizeIdx as usize)).type_ != ACPI_TYPE_INTEGER { dev_dbg((*pfru_dev).parent_dev, "Query buf failed with invalid package count/type\n"); ACPI_FREE(out_obj); return -EINVAL; }
    (*info).status = (*e.add(BufStatusIdx as usize)).integer.value; (*info).ext_status = (*e.add(BufExtStatusIdx as usize)).integer.value;
    if (*info).status != DSM_SUCCEED { dev_dbg((*pfru_dev).parent_dev, "Query buf failed with Error Status:%d\n", (*info).status); dev_dbg((*pfru_dev).parent_dev, "Query buf failed with Error Extended Status:%d\n", (*info).ext_status); ACPI_FREE(out_obj); return -EBUSY; }
    (*info).addr_lo = (*e.add(BufAddrLowIdx as usize)).integer.value; (*info).addr_hi = (*e.add(BufAddrHiIdx as usize)).integer.value; (*info).buf_size = (*e.add(BufSizeIdx as usize)).integer.value; ACPI_FREE(out_obj); 0
}

unsafe fn get_image_type(img_hdr: *const efi_manage_capsule_image_header, _pfru_dev: *mut PfruDevice) -> i32 { if guid_equal(&(*img_hdr).image_type_id, &PFRU_CODE_INJ_GUID) { return PFRU_CODE_INJECT_TYPE; } if guid_equal(&(*img_hdr).image_type_id, &PFRU_DRV_UPDATE_GUID) { return PFRU_DRIVER_UPDATE_TYPE; } -EINVAL }

unsafe fn adjust_efi_size(img_hdr: *const efi_manage_capsule_image_header, mut size: i32) -> i32 { size += core::mem::size_of::<efi_manage_capsule_image_header>() as i32; match (*img_hdr).ver { 1 => size - 2 * core::mem::size_of::<u64>() as i32, 2 => size - core::mem::size_of::<u64>() as i32, _ => -EINVAL } }

unsafe fn applicable_image(data: *const u8, cap: *mut pfru_update_cap_info, pfru_dev: *mut PfruDevice) -> bool {
    let cap_hdr = data as *const efi_capsule_header_t; let mut size = (*cap_hdr).headersize as usize; let m_hdr = data.add(size) as *const efi_manage_capsule_header;
    size += core::mem::offset_of!(efi_manage_capsule_header, offset_list) + ((*m_hdr).emb_drv_cnt as usize + (*m_hdr).payload_cnt as usize) * core::mem::size_of::<u64>(); let m_img_hdr = data.add(size) as *const efi_manage_capsule_image_header; let typ = get_image_type(m_img_hdr, pfru_dev); if typ < 0 { dev_dbg((*pfru_dev).parent_dev, "Invalid image type\n"); return false; }
    let size_i = adjust_efi_size(m_img_hdr, size as i32); if size_i < 0 { dev_dbg((*pfru_dev).parent_dev, "Invalid image size\n"); return false; } let auth = data.add(size_i as usize) as *const efi_image_auth; let size2 = size_i as usize + core::mem::size_of::<u64>() + (*auth).auth_info.hdr.len as usize; let payload_hdr = data.add(size2) as *const pfru_payload_hdr;
    if typ == PFRU_CODE_INJECT_TYPE { (*payload_hdr).rt_ver >= (*cap).code_rt_version } else { (*payload_hdr).svn_ver >= (*cap).drv_svn }
}

unsafe fn print_update_debug_info(result: *mut pfru_updated_result, dev: *mut PfruDevice) { dev_dbg((*dev).parent_dev, "Update result:\n"); dev_dbg((*dev).parent_dev, "Authentication Time Low:%lld\n", (*result).low_auth_time); dev_dbg((*dev).parent_dev, "Authentication Time High:%lld\n", (*result).high_auth_time); dev_dbg((*dev).parent_dev, "Execution Time Low:%lld\n", (*result).low_exec_time); dev_dbg((*dev).parent_dev, "Execution Time High:%lld\n", (*result).high_exec_time); }

unsafe fn start_update(action: i32, dev: *mut PfruDevice) -> i32 { let mut in_obj: acpi_object = core::mem::zeroed(); let mut in_buf: acpi_object = core::mem::zeroed(); let mut result: pfru_updated_result = core::mem::zeroed(); in_obj.type_ = ACPI_TYPE_PACKAGE; in_obj.package.count = 1; in_obj.package.elements = &mut in_buf; in_buf.type_ = ACPI_TYPE_INTEGER; in_buf.integer.value = action as u64; let out = acpi_evaluate_dsm_typed(ACPI_HANDLE((*dev).parent_dev), &PFRU_GUID, (*dev).rev_id, PFRU_FUNC_START, &mut in_obj, ACPI_TYPE_PACKAGE); if out.is_null() { dev_dbg((*dev).parent_dev, "Update failed to start with no object\n"); return -EINVAL; } let e = (*out).package.elements; if (*out).package.count < UpdateNrIdx as u32 { ACPI_FREE(out); return -EINVAL; } result.status = (*e.add(UpdateStatusIdx as usize)).integer.value; result.ext_status = (*e.add(UpdateExtStatusIdx as usize)).integer.value; if result.status != DSM_SUCCEED { dev_dbg((*dev).parent_dev, "Update failed with Error Status:%d\n", result.status); ACPI_FREE(out); return -EBUSY; } result.low_auth_time = (*e.add(UpdateAuthTimeLowIdx as usize)).integer.value; result.high_auth_time = (*e.add(UpdateAuthTimeHiIdx as usize)).integer.value; result.low_exec_time = (*e.add(UpdateExecTimeLowIdx as usize)).integer.value; result.high_exec_time = (*e.add(UpdateExecTimeHiIdx as usize)).integer.value; print_update_debug_info(&mut result, dev); ACPI_FREE(out); 0 }

unsafe fn pfru_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize { let dev = to_pfru_dev(file); let p = arg as *mut core::ffi::c_void; match cmd { PFRU_IOC_QUERY_CAP => { let mut cap = core::mem::zeroed(); let ret = query_capability(&mut cap, dev); if ret != 0 { ret as isize } else if copy_to_user(p, &cap, core::mem::size_of_val(&cap)) != 0 { -EFAULT as isize } else { 0 } }, PFRU_IOC_SET_REV => { let mut rev = 0u32; if copy_from_user(&mut rev, p, 4) != 0 { -EFAULT as isize } else if !pfru_valid_revid(rev) { -EINVAL as isize } else { (*dev).rev_id = rev; 0 } }, PFRU_IOC_STAGE => start_update(StartStage as i32, dev) as isize, PFRU_IOC_ACTIVATE => start_update(StartActivate as i32, dev) as isize, PFRU_IOC_STAGE_ACTIVATE => start_update(StartStageActivate as i32, dev) as isize, _ => -ENOTTY as isize } }

unsafe fn pfru_write(file: *mut file, buf: *const u8, len: usize, _ppos: *mut loff_t) -> isize { let dev = to_pfru_dev(file); let mut info = core::mem::zeroed(); let mut cap = core::mem::zeroed(); let ret = query_buffer(&mut info, dev); if ret != 0 { return ret as isize; } if len as u64 > info.buf_size { dev_dbg((*dev).parent_dev, "Capsule image size too large\n"); return -EINVAL as isize; } let phy_addr = ((info.addr_hi << 32) | info.addr_lo) as phys_addr_t; let ptr = memremap(phy_addr, info.buf_size as usize, MEMREMAP_WB); if ptr.is_null() { return -ENOMEM as isize; } if !copy_from_user_full(ptr, buf, len) { memunmap(ptr); return -EINVAL as isize; } let mut r = query_capability(&mut cap, dev); if r == 0 && !applicable_image(ptr as *const u8, &mut cap, dev) { r = -EINVAL; } memunmap(ptr); if r != 0 { r as isize } else { len as isize } }

#[no_mangle]
static ACPI_PFRU_FOPS: file_operations = file_operations { owner: THIS_MODULE, write: Some(pfru_write), unlocked_ioctl: Some(pfru_ioctl), llseek: Some(noop_llseek) };

unsafe fn acpi_pfru_remove(pdev: *mut platform_device) {
    let dev = platform_get_drvdata(pdev) as *mut PfruDevice;
    misc_deregister(&mut (*dev).miscdev);
}

unsafe extern "C" fn pfru_put_idx(data: *mut core::ffi::c_void) {
    let dev = data as *mut PfruDevice;
    ida_free(&mut PFRU_IDA, (*dev).index);
}

unsafe fn acpi_pfru_probe(pdev: *mut platform_device) -> i32 {
    let handle = ACPI_HANDLE(&mut (*pdev).dev);
    if handle.is_null() { return -ENODEV; }
    if !acpi_has_method(handle, cstr!("_DSM")) { dev_dbg(&mut (*pdev).dev, "Missing _DSM\n"); return -ENODEV; }
    let dev = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<PfruDevice>(), GFP_KERNEL) as *mut PfruDevice;
    if dev.is_null() { return -ENOMEM; }
    let mut ret = ida_alloc(&mut PFRU_IDA, GFP_KERNEL);
    if ret < 0 { return ret; }
    (*dev).index = ret as u32;
    ret = devm_add_action_or_reset(&mut (*pdev).dev, Some(pfru_put_idx), dev as *mut _);
    if ret != 0 { return ret; }
    (*dev).rev_id = PFRU_DEFAULT_REV_ID;
    (*dev).parent_dev = &mut (*pdev).dev;
    (*dev).miscdev.minor = MISC_DYNAMIC_MINOR;
    (*dev).miscdev.name = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, "pfru%d", (*dev).index);
    if (*dev).miscdev.name.is_null() { return -ENOMEM; }
    (*dev).miscdev.nodename = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, "acpi_pfr_update%d", (*dev).index);
    if (*dev).miscdev.nodename.is_null() { return -ENOMEM; }
    (*dev).miscdev.fops = &ACPI_PFRU_FOPS;
    (*dev).miscdev.parent = &mut (*pdev).dev;
    ret = misc_register(&mut (*dev).miscdev);
    if ret != 0 { return ret; }
    platform_set_drvdata(pdev, dev as *mut _);
    0
}

#[repr(C)]
static ACPI_PFRU_IDS: [acpi_device_id; 2] = [acpi_device_id { id: *b"INTC1080\0", driver_data: 0 }, acpi_device_id { id: [0; 9], driver_data: 0 }];

#[no_mangle]
static mut ACPI_PFRU_DRIVER: platform_driver = platform_driver {
    driver: device_driver { name: cstr!("pfr_update"), acpi_match_table: ACPI_PFRU_IDS.as_ptr() },
    probe: Some(acpi_pfru_probe), remove: Some(acpi_pfru_remove),
};

module_platform_driver!(ACPI_PFRU_DRIVER);
MODULE_DESCRIPTION!("Platform Firmware Runtime Update device driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
