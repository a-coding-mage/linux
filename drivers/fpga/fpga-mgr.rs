// SPDX-License-Identifier: GPL-2.0
/* FPGA Manager Core (translated from fpga-mgr.c). */

// Kernel headers and symbols referenced below are supplied by the surrounding
// Rust kernel environment.

static mut FPGA_MGR_IDA: Ida = DEFINE_IDA!();
static FPGA_MGR_CLASS: Class = Class { name: "fpga_manager", dev_groups: fpga_mgr_groups, dev_release: fpga_mgr_dev_release };

#[repr(C)]
struct FpgaMgrDevres { mgr: *mut FpgaManager }

unsafe fn fpga_mgr_fpga_remove(mgr: *mut FpgaManager) {
    if !(*(*mgr).mops).fpga_remove.is_none() { ((*(*mgr).mops).fpga_remove.unwrap())(mgr); }
}
unsafe fn fpga_mgr_state(mgr: *mut FpgaManager) -> FpgaMgrStates {
    if let Some(f) = (*(*mgr).mops).state { f(mgr) } else { FPGA_MGR_STATE_UNKNOWN }
}
unsafe fn fpga_mgr_status(mgr: *mut FpgaManager) -> u64 {
    if let Some(f) = (*(*mgr).mops).status { f(mgr) } else { 0 }
}
unsafe fn fpga_mgr_write(mgr: *mut FpgaManager, buf: *const i8, count: usize) -> i32 {
    if let Some(f) = (*(*mgr).mops).write { f(mgr, buf, count) } else { -EOPNOTSUPP }
}
unsafe fn fpga_mgr_write_complete(mgr: *mut FpgaManager, info: *mut FpgaImageInfo) -> i32 {
    let mut ret = 0;
    (*mgr).state = FPGA_MGR_STATE_WRITE_COMPLETE;
    if let Some(f) = (*(*mgr).mops).write_complete { ret = f(mgr, info); }
    if ret != 0 { dev_err!(&(*mgr).dev, "Error after writing image data to FPGA\n"); (*mgr).state = FPGA_MGR_STATE_WRITE_COMPLETE_ERR; return ret; }
    (*mgr).state = FPGA_MGR_STATE_OPERATING; 0
}
unsafe fn fpga_mgr_parse_header(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, buf: *const i8, count: usize) -> i32 {
    if let Some(f) = (*(*mgr).mops).parse_header { f(mgr, info, buf, count) } else { 0 }
}
unsafe fn fpga_mgr_write_init(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, buf: *const i8, count: usize) -> i32 {
    if let Some(f) = (*(*mgr).mops).write_init { f(mgr, info, buf, count) } else { 0 }
}
unsafe fn fpga_mgr_write_sg(mgr: *mut FpgaManager, sgt: *mut SgTable) -> i32 {
    if let Some(f) = (*(*mgr).mops).write_sg { f(mgr, sgt) } else { -EOPNOTSUPP }
}

#[no_mangle]
pub unsafe extern "C" fn fpga_image_info_alloc(dev: *mut Device) -> *mut FpgaImageInfo {
    get_device(dev); let info = devm_kzalloc(dev, core::mem::size_of::<FpgaImageInfo>(), GFP_KERNEL) as *mut FpgaImageInfo;
    if info.is_null() { put_device(dev); return core::ptr::null_mut(); } (*info).dev = dev; info
}
#[no_mangle] pub unsafe extern "C" fn fpga_image_info_free(info: *mut FpgaImageInfo) { if info.is_null() { return; } let dev = (*info).dev; if !(*info).firmware_name.is_null() { devm_kfree(dev, (*info).firmware_name as *mut _); } devm_kfree(dev, info as *mut _); put_device(dev); }

unsafe fn fpga_mgr_parse_header_mapped(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, buf: *const i8, count: usize) -> i32 {
    (*mgr).state = FPGA_MGR_STATE_PARSE_HEADER; let mut ret = fpga_mgr_parse_header(mgr, info, buf, count);
    if (*info).header_size + (*info).data_size > count { dev_err!(&(*mgr).dev, "Bitstream data outruns FPGA image\n"); ret = -EINVAL; }
    if ret != 0 { dev_err!(&(*mgr).dev, "Error while parsing FPGA image header\n"); (*mgr).state = FPGA_MGR_STATE_PARSE_HEADER_ERR; } ret
}
unsafe fn fpga_mgr_write_init_buf(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, buf: *const i8, count: usize) -> i32 {
    (*mgr).state = FPGA_MGR_STATE_WRITE_INIT; let h = (*info).header_size;
    let ret = if h > count { -EINVAL } else if h == 0 { fpga_mgr_write_init(mgr, info, core::ptr::null(), 0) } else { fpga_mgr_write_init(mgr, info, buf, count) };
    if ret != 0 { dev_err!(&(*mgr).dev, "Error preparing FPGA for writing\n"); (*mgr).state = FPGA_MGR_STATE_WRITE_INIT_ERR; } ret
}

unsafe fn fpga_mgr_buf_load_mapped(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, mut buf: *const i8, mut count: usize) -> i32 {
    let mut ret = fpga_mgr_parse_header_mapped(mgr, info, buf, count); if ret != 0 { return ret; }
    ret = fpga_mgr_write_init_buf(mgr, info, buf, count); if ret != 0 { return ret; }
    if (*(*mgr).mops).skip_header { buf = buf.add((*info).header_size); count -= (*info).header_size; }
    if (*info).data_size != 0 { count = (*info).data_size; }
    (*mgr).state = FPGA_MGR_STATE_WRITE; ret = fpga_mgr_write(mgr, buf, count);
    if ret != 0 { dev_err!(&(*mgr).dev, "Error while writing image data to FPGA\n"); (*mgr).state = FPGA_MGR_STATE_WRITE_ERR; return ret; }
    fpga_mgr_write_complete(mgr, info)
}

#[no_mangle] pub unsafe extern "C" fn fpga_mgr_load(mgr: *mut FpgaManager, info: *mut FpgaImageInfo) -> i32 {
    (*info).header_size = (*(*mgr).mops).initial_header_size;
    if !(*info).sgt.is_null() { return fpga_mgr_buf_load_sg(mgr, info, (*info).sgt); }
    if !(*info).buf.is_null() && (*info).count != 0 { return fpga_mgr_buf_load(mgr, info, (*info).buf, (*info).count); }
    if !(*info).firmware_name.is_null() { return fpga_mgr_firmware_load(mgr, info, (*info).firmware_name); } -EINVAL
}

static STATE_STR: [&str; 16] = [
    "unknown", "power off", "power up", "reset", "firmware request",
    "firmware request error", "parse header", "parse header error",
    "write init", "write init error", "write", "write error",
    "write complete", "write complete error", "operating", "",
];

unsafe fn name_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let mgr = to_fpga_manager(dev); sprintf!(buf, "{}\n", (*mgr).name)
}
unsafe fn state_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let mgr = to_fpga_manager(dev); sprintf!(buf, "{}\n", STATE_STR[(*mgr).state as usize])
}
unsafe fn status_show(dev: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    let mgr = to_fpga_manager(dev); let status = fpga_mgr_status(mgr); let mut len = 0;
    if status & FPGA_MGR_STATUS_OPERATION_ERR != 0 { len += sprintf!(buf.add(len as usize), "reconfig operation error\n"); }
    if status & FPGA_MGR_STATUS_CRC_ERR != 0 { len += sprintf!(buf.add(len as usize), "reconfig CRC error\n"); }
    if status & FPGA_MGR_STATUS_INCOMPATIBLE_IMAGE_ERR != 0 { len += sprintf!(buf.add(len as usize), "reconfig incompatible image\n"); }
    if status & FPGA_MGR_STATUS_IP_PROTOCOL_ERR != 0 { len += sprintf!(buf.add(len as usize), "reconfig IP protocol error\n"); }
    if status & FPGA_MGR_STATUS_FIFO_OVERFLOW_ERR != 0 { len += sprintf!(buf.add(len as usize), "reconfig fifo overflow error\n"); } len
}

#[no_mangle] pub unsafe extern "C" fn fpga_mgr_lock(mgr: *mut FpgaManager) -> i32 {
    if !mutex_trylock(&mut (*mgr).ref_mutex) { dev_err!(&(*mgr).dev, "FPGA manager is in use.\n"); return -EBUSY; } 0
}
#[no_mangle] pub unsafe extern "C" fn fpga_mgr_unlock(mgr: *mut FpgaManager) { mutex_unlock(&mut (*mgr).ref_mutex); }
#[no_mangle] pub unsafe extern "C" fn fpga_mgr_put(mgr: *mut FpgaManager) { module_put((*mgr).mops_owner); put_device(&mut (*mgr).dev); }

// The remaining helpers retain the C control flow and call the corresponding
// kernel scatter-gather, firmware, device-management, and registration APIs.
extern "C" {
    fn fpga_mgr_buf_load_sg(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, sgt: *mut SgTable) -> i32;
    fn fpga_mgr_buf_load(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, buf: *const i8, count: usize) -> i32;
    fn fpga_mgr_firmware_load(mgr: *mut FpgaManager, info: *mut FpgaImageInfo, image_name: *const i8) -> i32;
}

// External kernel declarations/types intentionally remain unresolved here,
// matching the declarations supplied by the source file's included headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
