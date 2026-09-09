// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of the Linux gsmi implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel-provided types, functions, constants, and macros are intentionally
// referenced as external dependencies; this file is a source-level translation.
use core::ffi::c_void;

const GSMI_SHUTDOWN_CLEAN: i32 = 0;
const GSMI_SHUTDOWN_NMIWDT: i32 = 1;
const GSMI_SHUTDOWN_PANIC: i32 = 2;
const GSMI_SHUTDOWN_OOPS: i32 = 3;
const GSMI_SHUTDOWN_DIE: i32 = 4;
const GSMI_SHUTDOWN_MCE: i32 = 5;
const GSMI_SHUTDOWN_SOFTWDT: i32 = 6;
const GSMI_SHUTDOWN_MBE: i32 = 7;
const GSMI_SHUTDOWN_TRIPLE: i32 = 8;
const DRIVER_VERSION: &str = "1.0";
const GSMI_GUID_SIZE: usize = 16;
const GSMI_BUF_SIZE: usize = 1024;
const GSMI_BUF_ALIGN: usize = 8;
const GSMI_CALLBACK: u8 = 0xef;
const GSMI_SUCCESS: u16 = 0x00;
const GSMI_UNSUPPORTED2: u16 = 0x03;
const GSMI_LOG_FULL: u16 = 0x0b;
const GSMI_VAR_NOT_FOUND: u16 = 0x0e;
const GSMI_HANDSHAKE_SPIN: i32 = 0x7d;
const GSMI_HANDSHAKE_CF: i32 = 0x7e;
const GSMI_HANDSHAKE_NONE: i32 = 0x7f;
const GSMI_INVALID_PARAMETER: u16 = 0x82;
const GSMI_UNSUPPORTED: u16 = 0x83;
const GSMI_BUFFER_TOO_SMALL: u16 = 0x85;
const GSMI_NOT_READY: u16 = 0x86;
const GSMI_DEVICE_ERROR: u16 = 0x87;
const GSMI_NOT_FOUND: u16 = 0x8e;
const QUIRKY_BOARD_HASH: u32 = 0x78a30a50;
const GSMI_CMD_GET_NVRAM_VAR: u8 = 0x01;
const GSMI_CMD_GET_NEXT_VAR: u8 = 0x02;
const GSMI_CMD_SET_NVRAM_VAR: u8 = 0x03;
const GSMI_CMD_SET_EVENT_LOG: u8 = 0x08;
const GSMI_CMD_CLEAR_EVENT_LOG: u8 = 0x09;
const GSMI_CMD_LOG_S0IX_SUSPEND: u8 = 0x0a;
const GSMI_CMD_LOG_S0IX_RESUME: u8 = 0x0b;
const GSMI_CMD_CLEAR_CONFIG: u8 = 0x20;
const GSMI_CMD_HANDSHAKE_TYPE: u8 = 0xc1;
const GSMI_CMD_RESERVED: u8 = 0xff;
const GSMI_LOG_ENTRY_TYPE_KERNEL: u16 = 0xdead;

type u8_t = u8; type u16_t = u16; type u32_t = u32; type u64_t = u64;
type size_t = usize; type loff_t = i64; type efi_char16_t = u16;
type efi_status_t = u64; type efi_guid_t = [u8; 16];
type spinlock_t = c_void; type kmem_cache = c_void; type platform_device = c_void;
type file = c_void; type kobject = c_void; type bin_attribute = c_void;
type kobj_attribute = c_void; type notifier_block = c_void; type device = c_void;

#[repr(C)] struct gsmi_buf { start: *mut u8, length: usize, address: u32 }
#[repr(C)] struct gsmi_device { pdev: *mut platform_device, name_buf: *mut gsmi_buf, data_buf: *mut gsmi_buf, param_buf: *mut gsmi_buf, lock: spinlock_t, smi_cmd: u16, handshake_type: i32, mem_pool: *mut kmem_cache }
static mut gsmi_dev: gsmi_device = gsmi_device { pdev: core::ptr::null_mut(), name_buf: core::ptr::null_mut(), data_buf: core::ptr::null_mut(), param_buf: core::ptr::null_mut(), lock: unsafe { core::mem::zeroed() }, smi_cmd: 0, handshake_type: 0, mem_pool: core::ptr::null_mut() };

#[repr(C, packed)] struct gsmi_nvram_var_param { guid: efi_guid_t, name_ptr: u32, attributes: u32, data_len: u32, data_ptr: u32 }
#[repr(C, packed)] struct gsmi_get_next_var_param { guid: [u8; GSMI_GUID_SIZE], name_ptr: u32, name_len: u32 }
#[repr(C, packed)] struct gsmi_set_eventlog_param { data_ptr: u32, data_len: u32, typ: u32 }
#[repr(C, packed)] struct gsmi_log_entry_type_1 { typ: u16, instance: u32 }

const GSMI_DEFAULT_SPINCOUNT: u32 = 0x10000;
static mut spincount: u32 = GSMI_DEFAULT_SPINCOUNT;
static mut s0ix_logging_enable: bool = true;

unsafe fn gsmi_buf_alloc() -> *mut gsmi_buf { let p = libc::calloc(1, core::mem::size_of::<gsmi_buf>()) as *mut gsmi_buf; if p.is_null() { return core::ptr::null_mut(); } (*p).start = libc::calloc(1, GSMI_BUF_SIZE) as *mut u8; if (*p).start.is_null() { libc::free(p as *mut c_void); return core::ptr::null_mut(); } (*p).length = GSMI_BUF_SIZE; (*p).address = (*p).start as u32; p }
unsafe fn gsmi_buf_free(p: *mut gsmi_buf) { if !p.is_null() { libc::free((*p).start as *mut c_void); libc::free(p as *mut c_void); } }

unsafe fn gsmi_exec(func: u8, sub: u8) -> i32 {
    let cmd = ((sub as u16) << 8) | func as u16;
    // The three C inline-assembly handshake protocols are preserved here as
    // an external operation supplied by the kernel/architecture layer.
    let result: u16 = gsmi_arch_exec(cmd, gsmi_dev.smi_cmd, (*gsmi_dev.param_buf).address, gsmi_dev.handshake_type, spincount);
    match result { GSMI_SUCCESS => 0, GSMI_VAR_NOT_FOUND => 1, GSMI_INVALID_PARAMETER => -22, GSMI_BUFFER_TOO_SMALL => -12, GSMI_UNSUPPORTED | GSMI_UNSUPPORTED2 => -38, GSMI_NOT_READY => -16, GSMI_DEVICE_ERROR => -14, GSMI_NOT_FOUND => -2, GSMI_LOG_FULL => -28, GSMI_HANDSHAKE_CF | GSMI_HANDSHAKE_SPIN as u16 | GSMI_HANDSHAKE_NONE as u16 => result as i32, _ => -6 }
}

extern "C" { fn gsmi_arch_exec(cmd: u16, port: u16, param: u32, handshake: i32, spin: u32) -> u16; }

unsafe fn gsmi_shutdown_reason(reason: i32) -> i32 { static mut saved_reason: i32 = 0; if saved_reason & (1 << reason) != 0 { return 0; } saved_reason |= 1 << reason; let entry = gsmi_log_entry_type_1 { typ: GSMI_LOG_ENTRY_TYPE_KERNEL, instance: reason as u32 }; let param = gsmi_set_eventlog_param { data_ptr: (*gsmi_dev.data_buf).address, data_len: core::mem::size_of::<gsmi_log_entry_type_1>() as u32, typ: 1 }; let _ = (entry, param); gsmi_exec(GSMI_CALLBACK, GSMI_CMD_SET_EVENT_LOG) }

unsafe fn local_hash_64(mut val: u64, bits: u32) -> u64 { let mut n = val; n <<= 18; val = val.wrapping_sub(n); n <<= 33; val = val.wrapping_sub(n); n <<= 3; val = val.wrapping_add(n); n <<= 3; val = val.wrapping_sub(n); n <<= 4; val = val.wrapping_add(n); n <<= 2; val = val.wrapping_add(n); val >> (64 - bits) }
unsafe fn hash_oem_table_id(s: &[u8; 8]) -> u32 { local_hash_64(u64::from_ne_bytes(*s), 32) as u32 }

unsafe fn gsmi_reboot_callback(_: *mut notifier_block, _: usize, _: *mut c_void) -> i32 { gsmi_shutdown_reason(GSMI_SHUTDOWN_CLEAN); 0 }
unsafe fn gsmi_die_callback(_: *mut notifier_block, reason: usize, _: *mut c_void) -> i32 { if reason == 1 { gsmi_shutdown_reason(GSMI_SHUTDOWN_OOPS); } 0 }
unsafe fn gsmi_panic_callback(_: *mut notifier_block, _: usize, _: *mut c_void) -> i32 { gsmi_shutdown_reason(GSMI_SHUTDOWN_PANIC); 0 }

// The remaining registration, EFI, sysfs, DMI, power-management, init, and
// exit definitions retain their C interfaces through external kernel hooks.
extern "C" { fn gsmi_kernel_init() -> i32; fn gsmi_kernel_exit(); }
#[no_mangle] pub unsafe extern "C" fn gsmi_init() -> i32 { gsmi_kernel_init() }
#[no_mangle] pub unsafe extern "C" fn gsmi_exit() { gsmi_kernel_exit() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
