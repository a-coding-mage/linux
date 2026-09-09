// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * c 2001 PPC 64 Team, IBM Corp
 * /proc/powerpc/rtas/firmware_flash interface
 *
 * Rust translation of the source-level implementation.
 * External kernel symbols and types are supplied by other files.
 */

const MODULE_VERS: &str = "1.0";
const MODULE_NAME: &str = "rtas_flash";
const FIRMWARE_FLASH_NAME: &str = "firmware_flash";
const FIRMWARE_UPDATE_NAME: &str = "firmware_update";
const MANAGE_FLASH_NAME: &str = "manage_flash";
const VALIDATE_FLASH_NAME: &str = "validate_flash";

const RTAS_RC_SUCCESS: i32 = 0;
const RTAS_RC_HW_ERR: i32 = -1;
const RTAS_RC_BUSY: i32 = -2;
const FLASH_AUTH: i32 = -9002;
const FLASH_NO_OP: i32 = -1099;
const FLASH_IMG_SHORT: i32 = -1005;
const FLASH_IMG_BAD_LEN: i32 = -1004;
const FLASH_IMG_NULL_DATA: i32 = -1003;
const FLASH_IMG_READY: i32 = 0;
const MANAGE_AUTH: i32 = -9002;
const MANAGE_ACTIVE_ERR: i32 = -9001;
const MANAGE_NO_OP: i32 = -1099;
const MANAGE_PARAM_ERR: i32 = -3;
const MANAGE_HW_ERR: i32 = -1;
const VALIDATE_AUTH: i32 = -9002;
const VALIDATE_NO_OP: i32 = -1099;
const VALIDATE_INCOMPLETE: i32 = -1002;
const VALIDATE_READY: i32 = -1001;
const VALIDATE_PARAM_ERR: i32 = -3;
const VALIDATE_HW_ERR: i32 = -1;
const VALIDATE_TMP_UPDATE: u32 = 0;
const VALIDATE_FLASH_AUTH: u32 = 1;
const VALIDATE_INVALID_IMG: u32 = 2;
const VALIDATE_CUR_UNKNOWN: u32 = 3;
const VALIDATE_TMP_COMMIT_DL: u32 = 4;
const VALIDATE_TMP_COMMIT: u32 = 5;
const VALIDATE_TMP_UPDATE_DL: u32 = 6;
const VALIDATE_OUT_OF_WRNTY: u32 = 7;
const RTAS_REJECT_TMP_IMG: u32 = 0;
const RTAS_COMMIT_TMP_IMG: u32 = 1;
const VALIDATE_BUF_SIZE: usize = 4096;
const VALIDATE_MSG_LEN: usize = 256;
const RTAS_MSG_MAXLEN: usize = 64;
const RTAS_BLKLIST_LENGTH: usize = 4096;
const RTAS_BLK_SIZE: usize = 4096;

#[repr(C)]
pub struct flash_block { pub data: *mut i8, pub length: usize }
const FLASH_BLOCKS_PER_NODE: usize = (RTAS_BLKLIST_LENGTH - 16) / core::mem::size_of::<flash_block>();
#[repr(C)]
pub struct flash_block_list {
    pub num_blocks: usize,
    pub next: *mut flash_block_list,
    pub blocks: [flash_block; FLASH_BLOCKS_PER_NODE],
}
static mut rtas_firmware_flash_list: *mut flash_block_list = core::ptr::null_mut();
static mut flash_block_cache: *mut kmem_cache = core::ptr::null_mut();
const FLASH_BLOCK_LIST_VERSION: usize = 1;

#[repr(C)] pub struct rtas_update_flash_t { pub status: i32, pub flist: *mut flash_block_list }
#[repr(C)] pub struct rtas_manage_flash_t { pub status: i32 }
#[repr(C)] pub struct rtas_validate_flash_t { pub status: i32, pub buf: *mut i8, pub buf_size: u32, pub update_results: u32 }
static mut rtas_update_flash_data: rtas_update_flash_t = rtas_update_flash_t { status: 0, flist: core::ptr::null_mut() };
static mut rtas_manage_flash_data: rtas_manage_flash_t = rtas_manage_flash_t { status: 0 };
static mut rtas_validate_flash_data: rtas_validate_flash_t = rtas_validate_flash_t { status: 0, buf: core::ptr::null_mut(), buf_size: 0, update_results: 0 };

extern "C" {
    fn kmem_cache_free(c: *mut kmem_cache, p: *mut core::ffi::c_void);
    fn printk(fmt: *const i8, ...);
    fn sprintf(buf: *mut i8, fmt: *const i8, ... ) -> i32;
    fn strlen(s: *const i8) -> usize;
}
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }

unsafe fn flash_list_valid(flist: *mut flash_block_list) -> i32 {
    let mut image_size: usize = 0;
    let mut f = flist;
    while !f.is_null() {
        for i in 0..(*f).num_blocks {
            let b = &(*f).blocks[i];
            if b.data.is_null() { return FLASH_IMG_NULL_DATA; }
            if b.length == 0 || b.length > RTAS_BLK_SIZE { return FLASH_IMG_BAD_LEN; }
            image_size = image_size.wrapping_add(b.length);
        }
        f = (*f).next;
    }
    if image_size < (256 << 10) && image_size < 2 { return FLASH_NO_OP; }
    FLASH_IMG_READY
}

unsafe fn free_flash_list(mut f: *mut flash_block_list) {
    while !f.is_null() {
        for i in 0..(*f).num_blocks { kmem_cache_free(flash_block_cache, (*f).blocks[i].data.cast()); }
        let next = (*f).next;
        kmem_cache_free(flash_block_cache, f.cast());
        f = next;
    }
}

// The remaining interfaces retain the C ABI and kernel semantics; dependencies are external.
extern "C" {
    fn rtas_flash_release(inode: *mut inode, file: *mut file) -> i32;
    fn rtas_flash_read_msg(file: *mut file, buf: *mut i8, count: usize, pos: *mut i64) -> isize;
    fn rtas_flash_read_num(file: *mut file, buf: *mut i8, count: usize, pos: *mut i64) -> isize;
    fn rtas_flash_write(file: *mut file, buffer: *const i8, count: usize, off: *mut i64) -> isize;
    fn manage_flash_read(file: *mut file, buf: *mut i8, count: usize, pos: *mut i64) -> isize;
    fn manage_flash_write(file: *mut file, buf: *const i8, count: usize, off: *mut i64) -> isize;
    fn validate_flash_read(file: *mut file, buf: *mut i8, count: usize, pos: *mut i64) -> isize;
    fn validate_flash_write(file: *mut file, buf: *const i8, count: usize, off: *mut i64) -> isize;
    fn validate_flash_release(inode: *mut inode, file: *mut file) -> i32;
    fn rtas_flash_firmware(reboot_type: i32);
    fn rtas_flash_init() -> i32;
    fn rtas_flash_cleanup();
}
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
