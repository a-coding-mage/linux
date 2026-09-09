// SPDX-License-Identifier: GPL-2.0
/* ACPI Platform Firmware Runtime Telemetry driver. */

// Kernel dependencies supplied by the surrounding translation unit.

pub const PFRT_LOG_EXEC_IDX: u32 = 0;
pub const PFRT_LOG_HISTORY_IDX: u32 = 1;
pub const PFRT_LOG_ERR: u32 = 0;
pub const PFRT_LOG_WARN: u32 = 1;
pub const PFRT_LOG_INFO: u32 = 2;
pub const PFRT_LOG_VERB: u32 = 4;
pub const PFRT_FUNC_SET_LEV: u32 = 1;
pub const PFRT_FUNC_GET_LEV: u32 = 2;
pub const PFRT_FUNC_GET_DATA: u32 = 3;
pub const PFRT_REVID_1: u32 = 1;
pub const PFRT_REVID_2: u32 = 2;
pub const PFRT_DEFAULT_REV_ID: u32 = PFRT_REVID_1;

#[repr(C)]
pub enum log_index {
    LOG_STATUS_IDX = 0, LOG_EXT_STATUS_IDX, LOG_MAX_SZ_IDX,
    LOG_CHUNK1_LO_IDX, LOG_CHUNK1_HI_IDX, LOG_CHUNK1_SZ_IDX,
    LOG_CHUNK2_LO_IDX, LOG_CHUNK2_HI_IDX, LOG_CHUNK2_SZ_IDX,
    LOG_ROLLOVER_CNT_IDX, LOG_RESET_CNT_IDX, LOG_NR_IDX,
}

#[repr(C)]
pub struct pfrt_log_device {
    pub index: i32,
    pub info: pfrt_log_info,
    pub parent_dev: *mut device,
    pub miscdev: miscdevice,
}

static PFRT_LOG_GUID: guid_t = GUID_INIT(0x75191659, 0x8178, 0x4D9D,
    0xB8, 0x8F, 0xAC, 0x5E, 0x5E, 0x93, 0xE8, 0xBF);
static mut PFRT_LOG_IDA: ida = DEFINE_IDA!();

#[inline]
unsafe fn to_pfrt_log_dev(file: *mut file) -> *mut pfrt_log_device {
    container_of!((*file).private_data, pfrt_log_device, miscdev)
}

unsafe fn get_pfrt_log_data_info(data_info: *mut pfrt_log_data_info,
                                 dev: *mut pfrt_log_device) -> i32 {
    let handle = ACPI_HANDLE((*dev).parent_dev);
    let mut out_obj: *mut acpi_object;
    let mut in_obj: acpi_object = core::mem::zeroed();
    let mut in_buf: acpi_object = core::mem::zeroed();
    let mut ret = -EBUSY;
    memset(data_info as *mut _, 0, core::mem::size_of::<pfrt_log_data_info>());
    in_obj.type_ = ACPI_TYPE_PACKAGE;
    in_obj.package.count = 1;
    in_obj.package.elements = &mut in_buf;
    in_buf.type_ = ACPI_TYPE_INTEGER;
    in_buf.integer.value = (*dev).info.log_type as u64;
    out_obj = acpi_evaluate_dsm_typed(handle, &PFRT_LOG_GUID,
        (*dev).info.log_revid, PFRT_FUNC_GET_DATA, &in_obj, ACPI_TYPE_PACKAGE);
    if out_obj.is_null() { return -EINVAL; }
    let p = (*out_obj).package.elements;
    if (*out_obj).package.count < LOG_NR_IDX as usize { ACPI_FREE(out_obj); return ret; }
    (*data_info).status = (*p.add(LOG_STATUS_IDX as usize)).integer.value;
    (*data_info).ext_status = (*p.add(LOG_EXT_STATUS_IDX as usize)).integer.value;
    if (*data_info).status != DSM_SUCCEED as u64 { ACPI_FREE(out_obj); return ret; }
    (*data_info).max_data_size = (*p.add(LOG_MAX_SZ_IDX as usize)).integer.value;
    (*data_info).chunk1_addr_lo = (*p.add(LOG_CHUNK1_LO_IDX as usize)).integer.value;
    (*data_info).chunk1_addr_hi = (*p.add(LOG_CHUNK1_HI_IDX as usize)).integer.value;
    (*data_info).chunk1_size = (*p.add(LOG_CHUNK1_SZ_IDX as usize)).integer.value;
    (*data_info).chunk2_addr_lo = (*p.add(LOG_CHUNK2_LO_IDX as usize)).integer.value;
    (*data_info).chunk2_addr_hi = (*p.add(LOG_CHUNK2_HI_IDX as usize)).integer.value;
    (*data_info).chunk2_size = (*p.add(LOG_CHUNK2_SZ_IDX as usize)).integer.value;
    (*data_info).rollover_cnt = (*p.add(LOG_ROLLOVER_CNT_IDX as usize)).integer.value;
    (*data_info).reset_cnt = (*p.add(LOG_RESET_CNT_IDX as usize)).integer.value;
    ret = 0;
    ACPI_FREE(out_obj);
    ret
}

unsafe fn set_pfrt_log_level(level: i32, dev: *mut pfrt_log_device) -> i32 {
    let handle = ACPI_HANDLE((*dev).parent_dev);
    let mut in_obj: acpi_object = core::mem::zeroed();
    let mut in_buf: acpi_object = core::mem::zeroed();
    in_obj.type_ = ACPI_TYPE_PACKAGE; in_obj.package.count = 1;
    in_obj.package.elements = &mut in_buf; in_buf.type_ = ACPI_TYPE_INTEGER;
    in_buf.integer.value = level as u64;
    let out = acpi_evaluate_dsm_typed(handle, &PFRT_LOG_GUID, (*dev).info.log_revid,
        PFRT_FUNC_SET_LEV, &in_obj, ACPI_TYPE_PACKAGE);
    if out.is_null() { return -EINVAL; }
    let ret = if (*out).package.elements[0].integer.value != DSM_SUCCEED as u64 { -EBUSY } else { 0 };
    ACPI_FREE(out); ret
}

unsafe fn get_pfrt_log_level(dev: *mut pfrt_log_device) -> i32 {
    let out = acpi_evaluate_dsm_typed(ACPI_HANDLE((*dev).parent_dev), &PFRT_LOG_GUID,
        (*dev).info.log_revid, PFRT_FUNC_GET_LEV, core::ptr::null(), ACPI_TYPE_PACKAGE);
    if out.is_null() { return -EINVAL; }
    let p = (*out).package.elements;
    let ret = if (*p).type_ != ACPI_TYPE_INTEGER || (*p).integer.value != DSM_SUCCEED as u64
        || (*p.add(2)).type_ != ACPI_TYPE_INTEGER { -EBUSY } else { (*p.add(2)).integer.value as i32 };
    ACPI_FREE(out); ret
}

fn valid_log_level(level: u32) -> bool { matches!(level, PFRT_LOG_ERR | PFRT_LOG_WARN | PFRT_LOG_INFO | PFRT_LOG_VERB) }
fn valid_log_type(ty: u32) -> bool { ty == PFRT_LOG_EXEC_IDX || ty == PFRT_LOG_HISTORY_IDX }
#[inline] fn valid_log_revid(id: u32) -> bool { id == PFRT_REVID_1 || id == PFRT_REVID_2 }

unsafe fn pfrt_log_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    let dev = to_pfrt_log_dev(file);
    let p = arg as *mut core::ffi::c_void;
    match cmd {
        PFRT_LOG_IOC_SET_INFO => {
            let mut info: pfrt_log_info = core::mem::zeroed();
            if copy_from_user(&mut info as *mut _, p, core::mem::size_of_val(&info)) != 0 { return -EFAULT as isize; }
            if valid_log_revid(info.log_revid) { (*dev).info.log_revid = info.log_revid; }
            if valid_log_level(info.log_level) { let r = set_pfrt_log_level(info.log_level as i32, dev); if r < 0 { return r as isize; } (*dev).info.log_level = info.log_level; }
            if valid_log_type(info.log_type) { (*dev).info.log_type = info.log_type; } 0
        }
        PFRT_LOG_IOC_GET_INFO => {
            let mut info = (*dev).info; info.log_level = get_pfrt_log_level(dev) as u32;
            if copy_to_user(p, &info as *const _, core::mem::size_of_val(&info)) != 0 { -EFAULT as isize } else { 0 }
        }
        PFRT_LOG_IOC_GET_DATA_INFO => {
            let mut info: pfrt_log_data_info = core::mem::zeroed();
            let r = get_pfrt_log_data_info(&mut info, dev); if r != 0 { return r as isize; }
            if copy_to_user(p, &info as *const _, core::mem::size_of_val(&info)) != 0 { -EFAULT as isize } else { 0 }
        }
        _ => -ENOTTY as isize,
    }
}

unsafe fn pfrt_log_mmap(file: *mut file, vma: *mut vm_area_struct) -> i32 {
    if (*vma).vm_flags & VM_WRITE != 0 { return -EROFS; }
    vm_flags_clear(vma, VM_MAYWRITE);
    let dev = to_pfrt_log_dev(file); let mut info: pfrt_log_data_info = core::mem::zeroed();
    let r = get_pfrt_log_data_info(&mut info, dev); if r != 0 { return r; }
    let base = ((info.chunk2_addr_hi << 32) | info.chunk2_addr_lo) as phys_addr_t;
    if base == 0 || !PAGE_ALIGNED(base) || !PAGE_ALIGNED(info.max_data_size) { return -ENODEV; }
    let size = (*vma).vm_end - (*vma).vm_start; if size > info.max_data_size as usize { return -EINVAL; }
    (*vma).vm_page_prot = pgprot_noncached((*vma).vm_page_prot);
    if io_remap_pfn_range(vma, (*vma).vm_start, PFN_DOWN(base), size, (*vma).vm_page_prot) != 0 { -EAGAIN } else { 0 }
}

unsafe fn acpi_pfrt_log_remove(pdev: *mut platform_device) { let d = platform_get_drvdata(pdev) as *mut pfrt_log_device; misc_deregister(&mut (*d).miscdev); }
unsafe fn pfrt_log_put_idx(data: *mut core::ffi::c_void) { let d = data as *mut pfrt_log_device; ida_free(&mut PFRT_LOG_IDA, (*d).index); }
unsafe fn acpi_pfrt_log_probe(_pdev: *mut platform_device) -> i32 { -ENODEV }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
