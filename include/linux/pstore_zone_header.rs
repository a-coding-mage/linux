/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_char;

// Supplied by the kernel module dependency.
pub enum module {}

pub type pstore_zone_read_op =
    Option<unsafe extern "C" fn(*mut c_char, usize, i64) -> isize>;
pub type pstore_zone_write_op =
    Option<unsafe extern "C" fn(*const c_char, usize, i64) -> isize>;
pub type pstore_zone_erase_op = Option<unsafe extern "C" fn(usize, i64) -> isize>;

/**
 * struct pstore_zone_info - pstore/zone back-end driver structure
 *
 * @owner:        Module which is responsible for this back-end driver.
 * @name:         Name of the back-end driver.
 * @total_size: The total size in bytes pstore/zone can use. It must be greater
 *              than 4096 and be multiple of 4096.
 * @kmsg_size:   The size of oops/panic zone. Zero means disabled, otherwise,
 *              it must be multiple of SECTOR_SIZE(512 Bytes).
 * @max_reason: Maximum kmsg dump reason to store.
 * @pmsg_size:  The size of pmsg zone which is the same as @kmsg_size.
 * @console_size:The size of console zone which is the same as @kmsg_size.
 * @ftrace_size:The size of ftrace zone which is the same as @kmsg_size.
 * @read:       The general read operation. Both of the function parameters
 *              @size and @offset are relative value to storage.
 *              On success, the number of bytes should be returned, others
 *              mean error.
 * @write:      The same as @read, but the following error number:
 *              -EBUSY means try to write again later.
 *              -ENOMSG means to try next zone.
 * @erase:      The general erase operation for device with special removing
 *              job. Both of the function parameters @size and @offset are
 *              relative value to storage.
 *              Return 0 on success and others on failure.
 * @panic_write:The write operation only used for panic case. It's optional
 *              if you do not care panic log. The parameters are relative
 *              value to storage.
 *              On success, the number of bytes should be returned, others
 *              excluding -ENOMSG mean error. -ENOMSG means to try next zone.
 */
#[repr(C)]
pub struct pstore_zone_info {
    pub owner: *mut module,
    pub name: *const c_char,

    pub total_size: usize,
    pub kmsg_size: usize,
    pub max_reason: i32,
    pub pmsg_size: usize,
    pub console_size: usize,
    pub ftrace_size: usize,
    pub read: pstore_zone_read_op,
    pub write: pstore_zone_write_op,
    pub erase: pstore_zone_erase_op,
    pub panic_write: pstore_zone_write_op,
}

unsafe extern "C" {
    pub fn register_pstore_zone(info: *mut pstore_zone_info) -> i32;
    pub fn unregister_pstore_zone(info: *mut pstore_zone_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
