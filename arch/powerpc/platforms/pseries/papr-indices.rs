// SPDX-License-Identifier: GPL-2.0-only

// External kernel and PAPR symbols referenced below are supplied by other translation units.

const RTAS_IBM_DYNAMIC_INDICE_NO_INDICATOR: i32 = -3;

#[repr(C)]
struct RtasGetIndicesParams {
    is_sensor: u8,
    indice_type: u32,
    work_area: *mut RtasWorkArea,
    next: u32,
    status: i32,
}

// Opaque types and external interfaces supplied by the kernel dependencies.
#[repr(C)] struct RtasWorkArea { _private: [u8; 0] }
#[repr(C)] struct PaprRtasSequence { begin: Option<unsafe extern "C" fn(*mut PaprRtasSequence)>, end: Option<unsafe extern "C" fn(*mut PaprRtasSequence)>, work: Option<unsafe extern "C" fn(*mut PaprRtasSequence, *mut usize) -> *const i8>, params: *mut core::ffi::c_void }
#[repr(C)] struct File { private_data: *mut core::ffi::c_void, f_mode: u32 }
#[repr(C)] struct FileOperations { _private: [u8; 0] }
#[repr(C)] struct MiscDevice { _private: [u8; 0] }
#[repr(C)] struct PaprIndicesIoBlock { indices: PaprIndices, dynamic_param: PaprDynamicParam }
#[repr(C)] struct PaprIndices { is_sensor: u8, indice_type: u32 }
#[repr(C)] struct PaprDynamicParam { token: u32, state: u32, location_code_str: [i8; 256] }

extern "C" {
    static rtas_ibm_get_indices_lock: Mutex;
    static rtas_ibm_set_dynamic_indicator_lock: Mutex;
    static rtas_ibm_get_dynamic_sensor_state_lock: Mutex;
    static papr_indices_handle_ops: FileOperations;
    static papr_indices_ops: FileOperations;
    static mut papr_indices_dev: MiscDevice;
}

#[repr(C)] struct Mutex { _private: [u8; 0] }

unsafe extern "C" {
    fn rtas_function_token(function: i32) -> i32;
    fn rtas_call(token: i32, nargs: i32, nret: i32, rets: *mut u32, ...) -> i32;
    fn rtas_busy_delay(status: i32) -> bool;
    fn rtas_work_area_alloc(size: usize) -> *mut RtasWorkArea;
    fn rtas_work_area_free(area: *mut RtasWorkArea);
    fn rtas_work_area_phys(area: *mut RtasWorkArea) -> u64;
    fn rtas_work_area_raw_buf(area: *mut RtasWorkArea) -> *mut u8;
    fn rtas_work_area_size(area: *mut RtasWorkArea) -> usize;
    fn papr_rtas_sequence_should_stop(seq: *mut PaprRtasSequence, status: i32, init: bool) -> bool;
    fn papr_rtas_sequence_set_err(seq: *mut PaprRtasSequence, err: i32) -> bool;
    fn papr_rtas_setup_file_interface(seq: *mut PaprRtasSequence, ops: *const FileOperations, name: *const i8) -> i64;
    fn papr_rtas_common_handle_seek(file: *mut File, off: *mut i64, whence: i32) -> i64;
    fn papr_rtas_common_handle_release(inode: *mut core::ffi::c_void, file: *mut File) -> i32;
    fn misc_register(dev: *mut MiscDevice) -> i32;
}

const RTAS_GET_INDICES_BUF_SIZE: usize = 4096;
const LOC_CODE_SIZE: usize = 256;
const RTAS_UNKNOWN_SERVICE: i32 = -1;
const RTAS_HARDWARE_ERROR: i32 = -1;
const RTAS_INVALID_PARAMETER: i32 = -2;
const RTAS_SEQ_START_OVER: i32 = -3;
const RTAS_SEQ_MORE_DATA: i32 = -4;
const RTAS_SEQ_COMPLETE: i32 = 0;
const RTAS_SUCCESS: i32 = 0;

unsafe fn rtas_ibm_get_indices(params: *mut RtasGetIndicesParams) -> i32 {
    let work_area = (*params).work_area;
    let token = rtas_function_token(RTAS_FN_IBM_GET_INDICES);
    if token == RTAS_UNKNOWN_SERVICE { return -2; }
    mutex_lock(&rtas_ibm_get_indices_lock);
    let mut rets = 0u32;
    let mut fwrc;
    loop {
        fwrc = rtas_call(token, 5, 2, &mut rets, (*params).is_sensor, (*params).indice_type,
            rtas_work_area_phys(work_area), rtas_work_area_size(work_area), (*params).next);
        if !rtas_busy_delay(fwrc) { break; }
    }
    let ret = match fwrc {
        RTAS_HARDWARE_ERROR => -5,
        RTAS_INVALID_PARAMETER => -22,
        RTAS_SEQ_START_OVER => { (*params).next = 1; -11 },
        RTAS_SEQ_MORE_DATA => { (*params).next = rets; 0 },
        RTAS_SEQ_COMPLETE => { (*params).next = 0; 0 },
        _ => -5,
    };
    (*params).status = fwrc;
    ret
}

unsafe extern "C" fn indices_sequence_begin(seq: *mut PaprRtasSequence) {
    let param = (*seq).params as *mut RtasGetIndicesParams;
    mutex_lock(&rtas_ibm_get_indices_lock);
    (*param).work_area = rtas_work_area_alloc(RTAS_GET_INDICES_BUF_SIZE);
    (*param).next = 1;
    (*param).status = 0;
}

unsafe extern "C" fn indices_sequence_end(seq: *mut PaprRtasSequence) {
    let param = (*seq).params as *mut RtasGetIndicesParams;
    rtas_work_area_free((*param).work_area);
    mutex_unlock(&rtas_ibm_get_indices_lock);
}

unsafe extern "C" fn indices_sequence_fill_work_area(seq: *mut PaprRtasSequence, len: *mut usize) -> *const i8 {
    let p = (*seq).params as *mut RtasGetIndicesParams;
    let init_state = (*p).next == 1;
    if papr_rtas_sequence_should_stop(seq, (*p).status, init_state) { return core::ptr::null(); }
    if papr_rtas_sequence_set_err(seq, rtas_ibm_get_indices(p)) { return core::ptr::null(); }
    *len = RTAS_GET_INDICES_BUF_SIZE;
    rtas_work_area_raw_buf((*p).work_area) as *const i8
}

unsafe extern "C" fn papr_indices_handle_read(file: *mut File, buf: *mut i8, size: usize, off: *mut i64) -> isize {
    let blob = (*file).private_data;
    if blob.is_null() { return -5; }
    if size < RTAS_GET_INDICES_BUF_SIZE { return -22; }
    let size = core::cmp::min(size, RTAS_GET_INDICES_BUF_SIZE);
    simple_read_from_buffer(buf, size, off, blob)
}

unsafe fn papr_indices_create_handle(ubuf: *mut PaprIndicesIoBlock) -> i64 {
    let mut params = RtasGetIndicesParams { is_sensor: (*ubuf).indices.is_sensor, indice_type: (*ubuf).indices.indice_type, work_area: core::ptr::null_mut(), next: 0, status: 0 };
    let mut seq = PaprRtasSequence { begin: Some(indices_sequence_begin), end: Some(indices_sequence_end), work: Some(indices_sequence_fill_work_area), params: (&mut params as *mut _) as *mut _ };
    papr_rtas_setup_file_interface(&mut seq, &papr_indices_handle_ops, b"[papr-indices]\0".as_ptr() as *const i8)
}

unsafe extern "C" fn papr_indices_dev_ioctl(filp: *mut File, ioctl: u32, arg: usize) -> i64 {
    match ioctl {
        PAPR_INDICES_IOC_GET => papr_indices_create_handle(arg as *mut _),
        PAPR_DYNAMIC_SENSOR_IOC_GET => papr_dynamic_sensor_ioc_get(arg as *mut _),
        PAPR_DYNAMIC_INDICATOR_IOC_SET => {
            if (*filp).f_mode & FMODE_WRITE != 0 { papr_dynamic_indicator_ioc_set(arg as *mut _) } else { -9 }
        },
        _ => -515,
    }
}

unsafe fn papr_dynamic_indice_buf_from_user(ubuf: *mut PaprIndicesIoBlock, kbuf: *mut PaprIndicesIoBlock) -> *mut RtasWorkArea {
    core::ptr::copy_nonoverlapping(ubuf, kbuf, 1);
    let mut length = 0usize;
    while length < LOC_CODE_SIZE && (*kbuf).dynamic_param.location_code_str[length] != 0 { length += 1; }
    if length == LOC_CODE_SIZE { return core::ptr::null_mut(); }
    length += 1;
    let area = rtas_work_area_alloc(LOC_CODE_SIZE + 4);
    let len_be = (length as u32).to_be_bytes();
    core::ptr::copy_nonoverlapping(len_be.as_ptr(), rtas_work_area_raw_buf(area), 4);
    core::ptr::copy_nonoverlapping((*kbuf).dynamic_param.location_code_str.as_ptr() as *const u8, rtas_work_area_raw_buf(area).add(4), length);
    area
}

unsafe fn papr_dynamic_indicator_ioc_set(ubuf: *mut PaprIndicesIoBlock) -> i64 {
    let token = rtas_function_token(RTAS_FN_IBM_SET_DYNAMIC_INDICATOR);
    if token == RTAS_UNKNOWN_SERVICE { return -2; }
    mutex_lock(&rtas_ibm_set_dynamic_indicator_lock);
    let mut kbuf = core::mem::zeroed();
    let area = papr_dynamic_indice_buf_from_user(ubuf, &mut kbuf);
    if area.is_null() { mutex_unlock(&rtas_ibm_set_dynamic_indicator_lock); return -14; }
    let mut fwrc;
    loop { fwrc = rtas_call(token, 3, 1, core::ptr::null_mut(), kbuf.dynamic_param.token, kbuf.dynamic_param.state, rtas_work_area_phys(area)); if !rtas_busy_delay(fwrc) { break; } }
    rtas_work_area_free(area); mutex_unlock(&rtas_ibm_set_dynamic_indicator_lock);
    match fwrc { RTAS_SUCCESS => 0, RTAS_IBM_DYNAMIC_INDICE_NO_INDICATOR => -95, _ => -5 }
}

unsafe fn papr_dynamic_sensor_ioc_get(ubuf: *mut PaprIndicesIoBlock) -> i64 {
    let token = rtas_function_token(RTAS_FN_IBM_GET_DYNAMIC_SENSOR_STATE);
    if token == RTAS_UNKNOWN_SERVICE { return -2; }
    mutex_lock(&rtas_ibm_get_dynamic_sensor_state_lock);
    let mut kbuf = core::mem::zeroed(); let area = papr_dynamic_indice_buf_from_user(ubuf, &mut kbuf);
    if area.is_null() { mutex_unlock(&rtas_ibm_get_dynamic_sensor_state_lock); return -14; }
    let mut rets = 0u32; let mut fwrc;
    loop { fwrc = rtas_call(token, 2, 2, &mut rets, kbuf.dynamic_param.token, rtas_work_area_phys(area)); if !rtas_busy_delay(fwrc) { break; } }
    rtas_work_area_free(area); mutex_unlock(&rtas_ibm_get_dynamic_sensor_state_lock);
    if fwrc == RTAS_SUCCESS { (*ubuf).dynamic_param.state = rets; 0 } else if fwrc == RTAS_IBM_DYNAMIC_INDICE_NO_INDICATOR { -95 } else { -5 }
}

unsafe fn papr_indices_init() -> i32 {
    if !rtas_function_implemented(RTAS_FN_IBM_GET_INDICES) { return -19; }
    if !rtas_function_implemented(RTAS_FN_IBM_SET_DYNAMIC_INDICATOR) { return -19; }
    if !rtas_function_implemented(RTAS_FN_IBM_GET_DYNAMIC_SENSOR_STATE) { return -19; }
    misc_register(&mut papr_indices_dev)
}

extern "C" {
    fn mutex_lock(lock: *const Mutex);
    fn mutex_unlock(lock: *const Mutex);
    fn simple_read_from_buffer(buf: *mut i8, size: usize, off: *mut i64, blob: *mut core::ffi::c_void) -> isize;
    fn rtas_function_implemented(function: i32) -> bool;
}

const RTAS_FN_IBM_GET_INDICES: i32 = 0;
const RTAS_FN_IBM_SET_DYNAMIC_INDICATOR: i32 = 1;
const RTAS_FN_IBM_GET_DYNAMIC_SENSOR_STATE: i32 = 2;
const PAPR_INDICES_IOC_GET: u32 = 0;
const PAPR_DYNAMIC_SENSOR_IOC_GET: u32 = 1;
const PAPR_DYNAMIC_INDICATOR_IOC_SET: u32 = 2;
const FMODE_WRITE: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
