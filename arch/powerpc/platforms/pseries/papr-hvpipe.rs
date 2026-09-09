// SPDX-License-Identifier: GPL-2.0-only

// Translated from papr-hvpipe.c. Kernel and architecture supplied types,
// constants, macros, globals, and functions remain external dependencies.

use core::ffi::c_void;

extern "C" {
    static mut hvpipe_src_list_lock: c_void;
    static mut hvpipe_src_list: c_void;
    static mut hvpipe_ras_buf: [u8; RTAS_ERROR_LOG_MAX];
    static mut papr_hvpipe_wq: *mut workqueue_struct;
    static mut papr_hvpipe_work: *mut work_struct;
    static mut hvpipe_check_exception_token: i32;
    static mut hvpipe_feature: bool;
}

// The definitions below intentionally retain the kernel ABI-oriented layout
// and external operations used by the original implementation.
#[repr(C)] pub struct rtas_work_area { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct poll_table_struct { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct papr_sysparm_buf { pub len: u16, pub val: [u8; 1] }
#[repr(C)] pub struct hvpipe_source_info {
    pub list: c_void,
    pub srcID: u32,
    pub hvpipe_status: u32,
    pub recv_wqh: c_void,
}
#[repr(C)] pub struct papr_hvpipe_hdr { pub version: u8, pub flags: u8 }
#[repr(C)] pub struct hvpipe_event_buf { pub srcID: u32, pub event_type: u32 }
#[repr(C)] pub struct pseries_errorlog { pub data: *mut u8 }
#[repr(C)] pub struct rtas_error_log { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct miscdevice { _private: [u8; 0] }

extern "C" {
    fn rtas_function_token(x: i32) -> i32;
    fn rtas_call(token: i32, nargs: i32, nret: i32, rets: *mut i32, ...) -> i32;
    fn rtas_busy_delay(rc: i32) -> bool;
    fn rtas_work_area_phys(a: *mut rtas_work_area) -> u64;
    fn rtas_work_area_size(a: *mut rtas_work_area) -> u64;
    fn rtas_work_area_alloc(size: usize) -> *mut rtas_work_area;
    fn rtas_work_area_raw_buf(a: *mut rtas_work_area) -> *mut u8;
    fn rtas_work_area_free(a: *mut rtas_work_area);
    fn copy_to_user(dst: *mut u8, src: *const u8, n: usize) -> usize;
    fn copy_from_user(dst: *mut u8, src: *const u8, n: usize) -> usize;
    fn hvpipe_find_source(id: u32) -> *mut hvpipe_source_info;
}

// RTAS and kernel constants are supplied by the surrounding translation unit.
extern "C" { static RTAS_ERROR_LOG_MAX: usize; }

unsafe fn rtas_ibm_receive_hvpipe_msg(area: *mut rtas_work_area, srcID: *mut u32, bytesw: *mut u32) -> i32 {
    let token = rtas_function_token(RTAS_FN_IBM_RECEIVE_HVPIPE_MSG);
    let mut rets = [0i32; 2];
    if token == RTAS_UNKNOWN_SERVICE { return -ENOENT; }
    let fwrc;
    loop {
        fwrc = rtas_call(token, 2, 3, rets.as_mut_ptr(), rtas_work_area_phys(area), rtas_work_area_size(area));
        if !rtas_busy_delay(fwrc) { break; }
    }
    match fwrc {
        RTAS_SUCCESS => { *srcID = rets[0] as u32; *bytesw = rets[1] as u32; 0 }
        RTAS_HARDWARE_ERROR => -EIO,
        RTAS_INVALID_PARAMETER => -EINVAL,
        RTAS_FUNC_NOT_SUPPORTED => -EOPNOTSUPP,
        _ => -EIO,
    }
}

unsafe fn rtas_ibm_send_hvpipe_msg(area: *mut rtas_work_area, srcID: u32) -> i32 {
    let token = rtas_function_token(RTAS_FN_IBM_SEND_HVPIPE_MSG);
    if token == RTAS_UNKNOWN_SERVICE { return -ENOENT; }
    let fwrc;
    loop {
        fwrc = rtas_call(token, 2, 1, core::ptr::null_mut(), srcID, rtas_work_area_phys(area));
        if !rtas_busy_delay(fwrc) { break; }
    }
    match fwrc {
        RTAS_SUCCESS => 0,
        RTAS_HARDWARE_ERROR => -EIO,
        RTAS_INVALID_PARAMETER => -EINVAL,
        RTAS_HVPIPE_CLOSED => -EPIPE,
        RTAS_FUNC_NOT_SUPPORTED => -EOPNOTSUPP,
        _ => -EIO,
    }
}

unsafe fn hvpipe_rtas_recv_msg(buf: *mut u8, mut size: i32) -> i32 {
    let area = rtas_work_area_alloc(SZ_4K);
    if area.is_null() { return -ENOMEM; }
    let mut srcID = 0u32; let mut bytes_written = 0u32;
    let mut ret = rtas_ibm_receive_hvpipe_msg(area, &mut srcID, &mut bytes_written);
    if ret == 0 && !buf.is_null() {
        if size < bytes_written as i32 { size = size.max(0); bytes_written = size as u32; }
        ret = if copy_to_user(buf, rtas_work_area_raw_buf(area), bytes_written as usize) != 0 { -EFAULT } else { bytes_written as i32 };
    }
    rtas_work_area_free(area); ret
}

unsafe extern "C" fn papr_hvpipe_handle_write(file: *mut file, buf: *const u8, mut size: usize, _off: *mut i64) -> isize {
    if !hvpipe_feature { return -ENXIO as isize; }
    let src_info = (*file).private_data as *mut hvpipe_source_info;
    if src_info.is_null() { return -EIO as isize; }
    if size > HVPIPE_HDR_LEN + HVPIPE_MAX_WRITE_BUFFER_SIZE || size <= HVPIPE_HDR_LEN { return -EINVAL as isize; }
    size -= HVPIPE_HDR_LEN; let buf = buf.add(HVPIPE_HDR_LEN);
    let area = rtas_work_area_alloc(SZ_4K); if area.is_null() { return -ENOMEM as isize; }
    let work_buf = rtas_work_area_alloc(SZ_4K); if work_buf.is_null() { rtas_work_area_free(area); return -ENOMEM as isize; }
    let p = rtas_work_area_raw_buf(area) as *mut u64;
    *p.add(0) = ((3 * core::mem::size_of::<u64>()) as u64).to_be();
    *p.add(1) = rtas_work_area_phys(work_buf).to_be(); *p.add(2) = (size as u64).to_be();
    let mut ret = if copy_from_user(rtas_work_area_raw_buf(work_buf), buf, size) == 0 { rtas_ibm_send_hvpipe_msg(area, (*src_info).srcID) } else { -EPERM };
    if ret == 0 { ret = (size + HVPIPE_HDR_LEN) as i32; }
    rtas_work_area_free(work_buf); rtas_work_area_free(area); ret as isize
}

unsafe extern "C" fn papr_hvpipe_handle_read(file: *mut file, buf: *mut u8, size: usize, _off: *mut i64) -> isize {
    if !hvpipe_feature { return -ENXIO as isize; }
    let src = (*file).private_data as *mut hvpipe_source_info; if src.is_null() { return -EIO as isize; }
    if size > HVPIPE_HDR_LEN + HVPIPE_MAX_WRITE_BUFFER_SIZE || size < HVPIPE_HDR_LEN { return -EINVAL as isize; }
    let status = (*src).hvpipe_status; if status == 0 { return 0; }
    let flags = if status & HVPIPE_MSG_AVAILABLE != 0 { HVPIPE_MSG_AVAILABLE } else if status & HVPIPE_LOST_CONNECTION != 0 { HVPIPE_LOST_CONNECTION } else { return -EIO as isize };
    let hdr = papr_hvpipe_hdr { version: 0, flags: flags as u8 };
    if copy_to_user(buf, &hdr as *const _ as *const u8, HVPIPE_HDR_LEN) != 0 { return -EFAULT as isize; }
    let mut ret = if flags & HVPIPE_MSG_AVAILABLE != 0 { hvpipe_rtas_recv_msg(buf.add(HVPIPE_HDR_LEN), (size - HVPIPE_HDR_LEN) as i32) } else { (*src).hvpipe_status &= !HVPIPE_LOST_CONNECTION; 0 };
    if flags & HVPIPE_MSG_AVAILABLE != 0 && (ret >= 0 || ret == -EFAULT) { (*src).hvpipe_status &= !HVPIPE_MSG_AVAILABLE; }
    if ret >= 0 { ret += HVPIPE_HDR_LEN as i32; } ret as isize
}

unsafe extern "C" fn papr_hvpipe_handle_release(_inode: *mut inode, file: *mut file) -> i32 {
    let src = (*file).private_data as *mut hvpipe_source_info; (*file).private_data = core::ptr::null_mut();
    if !src.is_null() && (*src).hvpipe_status & HVPIPE_MSG_AVAILABLE != 0 { (*src).hvpipe_status = 0; hvpipe_rtas_recv_msg(core::ptr::null_mut(), 0); }
    0
}

unsafe extern "C" fn papr_hvpipe_handle_poll(_filp: *mut file, _wait: *mut poll_table_struct) -> u32 {
    if !hvpipe_feature { return POLLRDHUP; } 0
}

unsafe extern "C" fn papr_hvpipe_dev_create_handle(srcID: u32) -> i32 {
    let src = kzalloc_hvpipe_source();
    if src.is_null() { return -ENOMEM; }
    (*src).srcID = srcID;
    if !hvpipe_find_source(srcID).is_null() { kfree_hvpipe_source(src); return -EALREADY; }
    list_add_hvpipe_source(src);
    let fd = anon_inode_fd(src);
    if fd < 0 { list_del_hvpipe_source(src); kfree_hvpipe_source(src); }
    fd
}

unsafe extern "C" fn papr_hvpipe_dev_ioctl(_filp: *mut file, ioctl: u32, arg: usize) -> isize {
    if !hvpipe_feature { return -ENXIO as isize; }
    let srcID = *(arg as *const u32);
    if srcID & HVPIPE_HMC_ID_MASK == 0 { return -EINVAL as isize; }
    if ioctl == PAPR_HVPIPE_IOC_CREATE_HANDLE { papr_hvpipe_dev_create_handle(srcID) as isize } else { -ENOIOCTLCMD as isize }
}

unsafe extern "C" fn papr_hvpipe_work_fn(_work: *mut work_struct) { hvpipe_rtas_recv_msg(core::ptr::null_mut(), 0); }

unsafe extern "C" fn hvpipe_event_interrupt(_irq: i32, _dev_id: *mut c_void) -> i32 {
    let rc = rtas_call(hvpipe_check_exception_token, 6, 1, core::ptr::null_mut(), RTAS_VECTOR_EXTERNAL_INTERRUPT, 0, RTAS_HVPIPE_MSG_EVENTS, 1, hvpipe_ras_buf.as_ptr(), rtas_get_error_log_max());
    if rc != 0 { return IRQ_HANDLED; }
    let event = hvpipe_event_from_ras(hvpipe_ras_buf.as_ptr());
    let src = hvpipe_find_source(u32::from_be((*event).srcID));
    if !src.is_null() {
        if (*event).event_type & HVPIPE_LOST_CONNECTION != 0 { (*src).hvpipe_status |= HVPIPE_LOST_CONNECTION; }
        else if (*event).event_type & HVPIPE_MSG_AVAILABLE != 0 { (*src).hvpipe_status |= HVPIPE_MSG_AVAILABLE; }
        wake_up_source(src);
    } else if (*event).event_type & HVPIPE_MSG_AVAILABLE != 0 { queue_work(papr_hvpipe_wq, papr_hvpipe_work); }
    IRQ_HANDLED
}

unsafe fn set_hvpipe_sys_param(val: u8) -> i32 {
    let buf = papr_sysparm_buf_alloc(); if buf.is_null() { return -ENOMEM; }
    (*buf).len = 1u16.to_be(); (*buf).val[0] = val;
    let ret = papr_sysparm_set(PAPR_SYSPARM_HVPIPE_ENABLE, buf); papr_sysparm_buf_free(buf); ret
}

unsafe extern "C" fn enable_hvpipe_IRQ() -> i32 {
    hvpipe_check_exception_token = rtas_function_token(RTAS_FN_CHECK_EXCEPTION);
    if hvpipe_check_exception_token == RTAS_UNKNOWN_SERVICE { return -ENODEV; }
    let np = of_find_node_by_path(c"/event-sources/ibm,hvpipe-msg-events".as_ptr());
    if np.is_null() { return -ENODEV; }
    request_event_sources_irqs(np, hvpipe_event_interrupt, c"HPIPE_EVENT".as_ptr()); of_node_put(np); 0
}

unsafe extern "C" fn papr_hvpipe_init() -> i32 {
    if !rtas_hvpipe_capable() || !rtas_function_implemented(RTAS_FN_IBM_SEND_HVPIPE_MSG) || !rtas_function_implemented(RTAS_FN_IBM_RECEIVE_HVPIPE_MSG) { return -ENODEV; }
    papr_hvpipe_work = alloc_work(papr_hvpipe_work_fn); if papr_hvpipe_work.is_null() { return -ENOMEM; }
    papr_hvpipe_wq = alloc_ordered_workqueue(); if papr_hvpipe_wq.is_null() { papr_hvpipe_work = core::ptr::null_mut(); return -ENOMEM; }
    let ret = enable_hvpipe_IRQ(); if ret != 0 { return ret; }
    let ret = set_hvpipe_sys_param(1); if ret == 0 { hvpipe_feature = true; } ret
}

#[no_mangle] pub unsafe extern "C" fn hvpipe_migration_handler(action: i32) {
    if papr_hvpipe_work.is_null() { return; }
    if action == HVPIPE_SUSPEND { hvpipe_feature = false; } else if action == HVPIPE_RESUME { hvpipe_feature = true; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
