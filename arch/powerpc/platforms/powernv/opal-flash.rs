// SPDX-License-Identifier: GPL-2.0-or-later
/* PowerNV OPAL Firmware Update Interface */

use core::ffi::{c_char, c_int, c_long, c_void};

const FLASH_NO_OP: i32 = -1099;
const FLASH_NO_AUTH: i32 = -9002;
const VALIDATE_IMG_READY: i32 = -1001;
const VALIDATE_IMG_INCOMPLETE: i32 = -1002;
const MANAGE_ACTIVE_ERR: i32 = -9001;
const FLASH_IMG_READY: i32 = 0;
const FLASH_INVALID_IMG: i32 = -1003;
const FLASH_IMG_NULL_DATA: i32 = -1004;
const FLASH_IMG_BAD_LEN: i32 = -1005;
const FLASH_REJECT_TMP_SIDE: u8 = 0;
const FLASH_COMMIT_TMP_SIDE: u8 = 1;
const FLASH_UPDATE_CANCEL: i32 = 0;
const FLASH_UPDATE_INIT: i32 = 1;
const VALIDATE_TMP_UPDATE: u32 = 0;
const VALIDATE_FLASH_AUTH: u32 = 1;
const VALIDATE_INVALID_IMG: u32 = 2;
const VALIDATE_CUR_UNKNOWN: u32 = 3;
const VALIDATE_TMP_COMMIT_DL: u32 = 4;
const VALIDATE_TMP_COMMIT: u32 = 5;
const VALIDATE_TMP_UPDATE_DL: u32 = 6;
const VALIDATE_OUT_OF_WRNTY: u32 = 7;
const VALIDATE_BUF_SIZE: usize = 4096;
const MAX_IMAGE_SIZE: u32 = 0x40000000;

const IMAGE_INVALID: i32 = 0;
const IMAGE_LOADING: i32 = 1;
const IMAGE_READY: i32 = 2;

#[repr(C)]
struct ImageDataT { status: c_int, data: *mut c_void, size: u32 }
#[repr(C)]
struct ImageHeaderT { magic: u16, version: u16, size: u32 }
#[repr(C)]
struct ValidateFlashT { status: c_int, buf: *mut c_void, buf_size: u32, result: u32 }
#[repr(C)]
struct ManageFlashT { status: c_int }
#[repr(C)]
struct UpdateFlashT { status: c_int }

#[repr(C)] struct Kobject { _private: [u8; 0] }
#[repr(C)] struct KobjAttribute { _private: [u8; 0] }
#[repr(C)] struct File { _private: [u8; 0] }
#[repr(C)] struct BinAttribute { _private: [u8; 0] }
#[repr(C)] struct Attribute { _private: [u8; 0] }
#[repr(C)] struct AttributeGroup { _private: [u8; 0] }
#[repr(C)] struct OpalSgList { _private: [u8; 0] }

static mut IMAGE_HEADER: ImageHeaderT = ImageHeaderT { magic: 0, version: 0, size: 0 };
static mut IMAGE_DATA: ImageDataT = ImageDataT { status: 0, data: core::ptr::null_mut(), size: 0 };
static mut VALIDATE_FLASH_DATA: ValidateFlashT = ValidateFlashT { status: 0, buf: core::ptr::null_mut(), buf_size: 0, result: 0 };
static mut MANAGE_FLASH_DATA: ManageFlashT = ManageFlashT { status: 0 };
static mut UPDATE_FLASH_DATA: UpdateFlashT = UpdateFlashT { status: FLASH_NO_OP };

extern "C" {
    fn opal_validate_flash(addr: u64, size: *mut u32, result: *mut u32) -> c_long;
    fn opal_manage_flash(op: u8) -> c_int;
    fn opal_update_flash(addr: u64) -> i64;
    fn opal_vmalloc_to_sg_list(data: *mut c_void, size: u32) -> *mut OpalSgList;
    fn opal_check_token(token: c_int) -> c_int;
    static mut opal_kobj: *mut Kobject;
    fn printk_alert(fmt: *const c_char, ...);
    fn printk_warn(fmt: *const c_char, ...);
    fn printk_err(fmt: *const c_char, ...);
    fn printk_debug(fmt: *const c_char, ...);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sysfs_create_group(kobj: *mut Kobject, group: *const AttributeGroup) -> c_int;
    fn sysfs_remove_group(kobj: *mut Kobject, group: *const AttributeGroup);
    fn sysfs_create_bin_file(kobj: *mut Kobject, attr: *const BinAttribute) -> c_int;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vzalloc(size: usize) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn msleep(ms: u32);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, value: c_int, n: usize) -> *mut c_void;
    fn page_align(size: usize) -> usize;
    fn vmalloc_to_page(addr: *mut c_void) -> *mut c_void;
    fn set_page_reserved(page: *mut c_void);
    fn clear_page_reserved(page: *mut c_void);
    fn virt_to_phys(addr: *const c_void) -> u64;
}

static mut IMAGE_DATA_MUTEX: *mut c_void = core::ptr::null_mut();

unsafe fn opal_flash_validate() {
    let mut size = u32::from_be((*core::ptr::addr_of!(VALIDATE_FLASH_DATA)).buf_size).to_be();
    let mut result = 0u32;
    let ret = opal_validate_flash(virt_to_phys(VALIDATE_FLASH_DATA.buf), &mut size, &mut result);
    VALIDATE_FLASH_DATA.status = ret as c_int;
    VALIDATE_FLASH_DATA.buf_size = u32::from_be(size);
    VALIDATE_FLASH_DATA.result = u32::from_be(result);
}

unsafe fn opal_flash_manage(op: u8) { MANAGE_FLASH_DATA.status = opal_manage_flash(op); }

unsafe fn validate_show(buf: *mut c_char) -> isize {
    let args = &mut VALIDATE_FLASH_DATA;
    if args.status < VALIDATE_TMP_UPDATE as i32 {
        let n = sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, args.status);
        args.status = FLASH_NO_OP; return n;
    }
    let mut len = sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, args.result);
    if args.result != VALIDATE_TMP_UPDATE && args.result < VALIDATE_CUR_UNKNOWN { args.status = FLASH_NO_OP; return len; }
    let n = if args.buf_size as usize > VALIDATE_BUF_SIZE - len as usize { VALIDATE_BUF_SIZE - len as usize } else { args.buf_size as usize };
    memcpy(buf.offset(len), args.buf, n);
    len += n as isize; args.status = FLASH_NO_OP; len
}

unsafe fn validate_store(buf: *const c_char, count: usize) -> isize {
    if *buf != b'1' as c_char { return -22; }
    mutex_lock(IMAGE_DATA_MUTEX);
    if IMAGE_DATA.status != IMAGE_READY || IMAGE_DATA.size < VALIDATE_BUF_SIZE as u32 {
        VALIDATE_FLASH_DATA.result = VALIDATE_INVALID_IMG; VALIDATE_FLASH_DATA.status = VALIDATE_IMG_INCOMPLETE;
    } else {
        memcpy(VALIDATE_FLASH_DATA.buf, IMAGE_DATA.data, VALIDATE_BUF_SIZE);
        VALIDATE_FLASH_DATA.status = VALIDATE_IMG_READY; VALIDATE_FLASH_DATA.buf_size = VALIDATE_BUF_SIZE as u32;
        opal_flash_validate();
    }
    mutex_unlock(IMAGE_DATA_MUTEX); count as isize
}

unsafe fn manage_show(buf: *mut c_char) -> isize {
    let rc = sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, MANAGE_FLASH_DATA.status);
    MANAGE_FLASH_DATA.status = FLASH_NO_OP; rc
}

unsafe fn manage_store(buf: *const c_char, count: usize) -> isize {
    let op = match *buf { b'0' => FLASH_REJECT_TMP_SIDE, b'1' => FLASH_COMMIT_TMP_SIDE, _ => return -22 };
    opal_flash_manage(op); count as isize
}

unsafe fn update_show(buf: *mut c_char) -> isize { sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, UPDATE_FLASH_DATA.status) }

unsafe fn update_store(buf: *const c_char, count: usize) -> isize {
    mutex_lock(IMAGE_DATA_MUTEX);
    match *buf {
        b'0' => { if UPDATE_FLASH_DATA.status == FLASH_IMG_READY { opal_flash_update(FLASH_UPDATE_CANCEL); } UPDATE_FLASH_DATA.status = FLASH_NO_OP; }
        b'1' => { UPDATE_FLASH_DATA.status = if IMAGE_DATA.status == IMAGE_READY { opal_flash_update(FLASH_UPDATE_INIT) } else { FLASH_INVALID_IMG }; }
        _ => { mutex_unlock(IMAGE_DATA_MUTEX); return -22; }
    }
    mutex_unlock(IMAGE_DATA_MUTEX); count as isize
}

unsafe fn image_data_write(buffer: *mut c_char, pos: i64, count: usize) -> isize {
    mutex_lock(IMAGE_DATA_MUTEX);
    let rc = if pos == 0 {
        if !IMAGE_DATA.data.is_null() { free_image_buf(); }
        if UPDATE_FLASH_DATA.status == FLASH_IMG_READY { opal_flash_update(FLASH_UPDATE_CANCEL); }
        let r = alloc_image_buf(buffer, count); if r != 0 { r as isize } else if IMAGE_DATA.status != IMAGE_LOADING { -12 } else { 0 }
    } else { 0 };
    if rc != 0 { mutex_unlock(IMAGE_DATA_MUTEX); return rc; }
    if pos < 0 || (pos as u64 + count as u64) > IMAGE_DATA.size as u64 { mutex_unlock(IMAGE_DATA_MUTEX); return -22; }
    memcpy((IMAGE_DATA.data as usize + pos as usize) as *mut c_void, buffer as *const c_void, count);
    if pos as u64 + count as u64 == IMAGE_DATA.size as u64 { IMAGE_DATA.status = IMAGE_READY; }
    mutex_unlock(IMAGE_DATA_MUTEX); count as isize
}

unsafe fn opal_flash_update(op: i32) -> i32 {
    let mut addr: u64;
    let mut rc: i64 = -1;
    if op == FLASH_UPDATE_CANCEL { addr = 0; } else {
        let list = opal_vmalloc_to_sg_list(IMAGE_DATA.data, IMAGE_DATA.size);
        if list.is_null() { return rc as i32; }
        addr = virt_to_phys(list as *const c_void);
    }
    rc = opal_update_flash(addr);
    rc as i32
}

#[no_mangle] pub unsafe extern "C" fn opal_flash_update_print_message() {
    if UPDATE_FLASH_DATA.status != FLASH_IMG_READY { return; }
    printk_alert(b"FLASH: Flashing new firmware\0".as_ptr() as *const c_char);
    printk_alert(b"FLASH: Image is %u bytes\0".as_ptr() as *const c_char, IMAGE_DATA.size);
    printk_alert(b"FLASH: Performing flash and reboot/shutdown\0".as_ptr() as *const c_char);
    printk_alert(b"FLASH: This will take several minutes. Do not power off!\0".as_ptr() as *const c_char);
    msleep(500);
}

unsafe fn free_image_buf() {
    let mut addr = IMAGE_DATA.data;
    let mut size = page_align(IMAGE_DATA.size as usize);
    while size > 0 { clear_page_reserved(vmalloc_to_page(addr)); addr = (addr as usize + 4096) as *mut c_void; size -= 4096; }
    vfree(IMAGE_DATA.data); IMAGE_DATA.data = core::ptr::null_mut(); IMAGE_DATA.status = IMAGE_INVALID;
}

unsafe fn alloc_image_buf(buffer: *mut c_char, count: usize) -> i32 {
    if count < core::mem::size_of::<ImageHeaderT>() { return -22; }
    memcpy(core::ptr::addr_of_mut!(IMAGE_HEADER) as *mut c_void, buffer as *const c_void, core::mem::size_of::<ImageHeaderT>());
    IMAGE_DATA.size = u32::from_be(IMAGE_HEADER.size);
    if IMAGE_DATA.size > MAX_IMAGE_SIZE || IMAGE_DATA.size < VALIDATE_BUF_SIZE as u32 { return -22; }
    IMAGE_DATA.data = vzalloc(page_align(IMAGE_DATA.size as usize));
    if IMAGE_DATA.data.is_null() { return -12; }
    let mut addr = IMAGE_DATA.data; let mut size = page_align(IMAGE_DATA.size as usize);
    while size > 0 { set_page_reserved(vmalloc_to_page(addr)); addr = (addr as usize + 4096) as *mut c_void; size -= 4096; }
    IMAGE_DATA.status = IMAGE_LOADING; 0
}

#[no_mangle] pub unsafe extern "C" fn opal_flash_update_init() {
    const OPAL_FLASH_VALIDATE: c_int = 0;
    if opal_check_token(OPAL_FLASH_VALIDATE) == 0 { return; }
    VALIDATE_FLASH_DATA.buf = kzalloc(VALIDATE_BUF_SIZE, 0);
    if VALIDATE_FLASH_DATA.buf.is_null() { return; }
    if opal_kobj.is_null() { kfree(VALIDATE_FLASH_DATA.buf); return; }
    VALIDATE_FLASH_DATA.status = FLASH_NO_OP; MANAGE_FLASH_DATA.status = FLASH_NO_OP;
    UPDATE_FLASH_DATA.status = FLASH_NO_OP; IMAGE_DATA.status = IMAGE_INVALID;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
