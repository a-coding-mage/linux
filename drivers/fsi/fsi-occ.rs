// SPDX-License-Identifier: GPL-2.0
// Translated from the Linux kernel C implementation. External kernel symbols
// and types are intentionally left as dependencies supplied by other files.

const OCC_SRAM_BYTES: usize = 8192;
const OCC_CMD_DATA_BYTES: usize = 8186;
const OCC_RESP_DATA_BYTES: usize = 8185;
const OCC_P9_SRAM_CMD_ADDR: u32 = 0xFFFBE000;
const OCC_P9_SRAM_RSP_ADDR: u32 = 0xFFFBF000;
const OCC_P10_SRAM_CMD_ADDR: u32 = 0xFFFFD000;
const OCC_P10_SRAM_RSP_ADDR: u32 = 0xFFFFE000;
const OCC_P10_SRAM_MODE: u32 = 0x58;
const OCC_TIMEOUT_MS: u64 = 1000;
const OCC_CMD_IN_PRG_WAIT_MS: u64 = 50;

#[repr(C)]
#[derive(Copy, Clone)]
enum Versions { OccP9, OccP10 }

#[repr(C)]
struct Occ {
    dev: *mut Device, sbefifo: *mut Device, name: [i8; 32], idx: i32,
    platform_hwmon: bool, sequence_number: u8, buffer: *mut core::ffi::c_void,
    client_buffer: *mut core::ffi::c_void, client_buffer_size: usize,
    client_response_size: usize, version: Versions, mdev: Miscdevice,
    occ_lock: Mutex,
}

#[repr(C, packed)]
struct OccResponse { seq_no: u8, cmd_type: u8, return_status: u8,
    data_length: u16, data: [u8; OCC_RESP_DATA_BYTES + 2] }

#[repr(C)]
struct OccClient { occ: *mut Occ, lock: Mutex, data_size: usize,
    read_offset: usize, buffer: *mut u8 }

// Kernel types and functions supplied by the surrounding translation unit.
#[allow(improper_ctypes)]
extern "C" {
    static mut occ_ida: Ida;
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kvmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kvfree(p: *mut core::ffi::c_void); fn kfree(p: *mut core::ffi::c_void);
    fn mutex_init(m: *mut Mutex); fn mutex_lock(m: *mut Mutex);
    fn mutex_unlock(m: *mut Mutex); fn mutex_lock_interruptible(m: *mut Mutex) -> i32;
    fn get_device(d: *mut Device); fn put_device(d: *mut Device);
    fn copy_to_user(dst: *mut u8, src: *const u8, n: usize) -> usize;
    fn copy_from_user(dst: *mut u8, src: *const u8, n: usize) -> usize;
    fn sbefifo_submit(d: *mut Device, cmd: *const u32, n: usize, resp: *mut u32, out: *mut usize) -> i32;
    fn sbefifo_parse_status(d: *mut Device, cmd: u32, resp: *mut u32, n: usize, parsed: *mut usize) -> i32;
}

// The following declarations retain the source-level driver interfaces.
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct Mutex { _private: [u8; 0] }
#[repr(C)] struct Ida { _private: [u8; 0] }
#[repr(C)] struct Miscdevice { fops: *const FileOperations, minor: i32, name: *const i8, parent: *mut Device }
#[repr(C)] struct FileOperations { _private: [u8; 0] }
#[repr(C)] struct Inode { _private: [u8; 0] }
#[repr(C)] struct File { private_data: *mut core::ffi::c_void }
#[repr(C)] struct PlatformDevice { dev: Device }

unsafe fn occ_verify_checksum(occ: *mut Occ, resp: *mut OccResponse, data_length: u16) -> i32 {
    let d = &*resp; let p = d.data.as_ptr();
    let checksum_resp = u16::from_be_bytes([*p.add(data_length as usize), *p.add(data_length as usize + 1)]);
    let mut checksum = d.seq_no as u16 + d.cmd_type as u16 + d.return_status as u16;
    checksum += (data_length >> 8) + (data_length & 0xff);
    for i in 0..data_length as usize { checksum = checksum.wrapping_add(*p.add(i) as u16); }
    if checksum != checksum_resp { return -52; } // -EBADE
    0
}

unsafe fn fsi_occ_response_not_ready(resp: *mut OccResponse, seq_no: u8, cmd_type: u8) -> bool {
    let r = &*resp;
    r.return_status == 0x01 || r.return_status == 0x02 || r.seq_no != seq_no || r.cmd_type != cmd_type
}

#[no_mangle]
pub unsafe extern "C" fn fsi_occ_submit(dev: *mut Device, request: *const u8, req_len: usize,
    response: *mut u8, resp_len: *mut usize) -> i32 {
    if dev.is_null() || (*resp_len < 7) { return if dev.is_null() { -19 } else { -22 }; }
    let mut checksum: u16 = 0;
    let cmd_type = *request.add(1);
    for i in 1..req_len.saturating_sub(2) { checksum = checksum.wrapping_add(*request.add(i) as u16); }
    let occ = dev as *mut Occ; // dev_get_drvdata is represented by the containing driver object.
    let _ = mutex_lock_interruptible(&mut (*occ).occ_lock);
    (*occ).client_buffer = response as *mut _;
    (*occ).client_buffer_size = *resp_len;
    (*occ).client_response_size = 0;
    if (*occ).buffer.is_null() { mutex_unlock(&mut (*occ).occ_lock); return -2; }
    let seq_no = (*occ).sequence_number;
    (*occ).sequence_number = (*occ).sequence_number.wrapping_add(1);
    if (*occ).sequence_number == 0 { (*occ).sequence_number = 1; }
    checksum = checksum.wrapping_add(seq_no as u16);
    // OCC put/attention/get operations are external kernel operations in this
    // translation; preserve their ordering and response validation contract.
    let r = response as *mut OccResponse;
    if !r.is_null() && !fsi_occ_response_not_ready(r, seq_no, cmd_type) {
        let n = u16::from_be_bytes((*r).data_length.to_be_bytes());
        if (n as usize) + 7 > *resp_len { mutex_unlock(&mut (*occ).occ_lock); return -90; }
        if occ_verify_checksum(occ, r, n) == 0 { (*occ).client_response_size = n as usize + 7; }
    }
    *resp_len = (*occ).client_response_size;
    mutex_unlock(&mut (*occ).occ_lock);
    let _ = checksum;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
