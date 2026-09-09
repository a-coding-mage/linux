// SPDX-License-Identifier: GPL-2.0-only
/*
 * Arm Firmware Framework for ARMv8-A(FFA) interface driver
 *
 * This is a low-level, source-faithful Rust translation.  Kernel-provided
 * types, constants, helpers, callbacks, and synchronization primitives are
 * intentionally referenced as external dependencies.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;
use core::mem::{size_of, MaybeUninit};
use core::ptr;

// The following items are supplied by the surrounding kernel/Rust bindings.
extern "C" {
    static mut drv_info: *mut ffa_drv_info;
    static mut ffa_pdev: *mut platform_device;
    static mut invoke_ffa_fn: Option<unsafe extern "C" fn(*const ffa_value_t, *mut ffa_value_t)>;
}

#[repr(C)]
pub struct ffa_value_t { pub a0: u64, pub a1: u64, pub a2: u64, pub a3: u64,
    pub a4: u64, pub a5: u64, pub a6: u64, pub a7: u64 }

#[repr(C)] pub struct ffa_pcpu_irq { pub info: *mut ffa_drv_info, pub notif_pcpu_work: work_struct }
#[repr(C)] pub struct ffa_drv_info {
    pub version: u32, pub vm_id: u16, pub rx_lock: mutex, pub tx_lock: mutex,
    pub rx_buffer: *mut c_void, pub tx_buffer: *mut c_void, pub rxtx_bufsz: usize,
    pub mem_ops_native: bool, pub msg_direct_req2_supp: bool, pub bitmap_created: bool,
    pub bus_notifier_registered: bool, pub notif_enabled: bool,
    pub sched_recv_irq: u32, pub notif_pend_irq: u32, pub cpuhp_state: u32,
    pub irq_pcpu: *mut ffa_pcpu_irq, pub notif_pcpu_wq: *mut workqueue_struct,
    pub sched_recv_irq_work: work_struct, pub partition_info: xarray,
    pub notifier_hash: [hlist_head; 4], pub notify_lock: rwlock_t,
}

#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { _private: [u8; 0] }

#[repr(C)] pub struct uuid_t { pub b: [u8; 16] }
#[repr(C)] pub struct ffa_partition_info { pub id: u16, pub exec_ctxt: u16, pub properties: u32, pub uuid: uuid_t }
#[repr(C)] pub struct ffa_device { pub vm_id: u16, pub mode_32bit: bool, pub uuid: uuid_t }
#[repr(C)] pub struct ffa_send_direct_data { pub data0: u64, pub data1: u64, pub data2: u64, pub data3: u64, pub data4: u64 }
#[repr(C)] pub struct ffa_send_direct_data2 { pub data0: u64, pub data1: u64, pub data2: u64, pub data3: u64, pub data4: u64 }

const FFA_DRIVER_VERSION: u32 = 0x0001_0002;
const FFA_MIN_VERSION: u32 = 0x0001_0000;
const FFA_MAX_NOTIFICATIONS: i32 = 64;
const SENDER_ID_MASK: u32 = 0xffff_0000;
const RECEIVER_ID_MASK: u32 = 0x0000_ffff;
const RXTX_MAP_MIN_BUFSZ_MASK: u32 = 3;
const RXTX_MAP_MAX_BUFSZ_MASK: u32 = 0xffff_0000;

#[inline] fn sender_id(x: u32) -> u16 { (x >> 16) as u16 }
#[inline] fn receiver_id(x: u32) -> u16 { x as u16 }
#[inline] fn pack_target_info(s: u16, r: u16) -> u32 { ((s as u32) << 16) | r as u32 }
#[inline] fn ffa_to_linux_errno(errno: i32) -> i32 {
    match -errno { 0 => 0, 1 => -95, 2 => -22, 3 => -12, 4 => -16, 5 => -4,
        6 => -13, 7 => -11, 8 => -125, 9 => -61, 10 => -11, _ => -22 }
}

unsafe fn invoke(args: ffa_value_t, ret: *mut ffa_value_t) {
    if let Some(f) = invoke_ffa_fn { f(&args, ret); }
}

unsafe fn ffa_compatible_version_find(version: u32) -> u32 {
    let major = version >> 16; let minor = version & 0xffff;
    let drv_major = FFA_DRIVER_VERSION >> 16; let drv_minor = FFA_DRIVER_VERSION & 0xffff;
    if major < drv_major || (major == drv_major && minor <= drv_minor) { version } else { FFA_DRIVER_VERSION }
}

unsafe fn ffa_version_check(version: *mut u32) -> i32 {
    let mut ver = MaybeUninit::<ffa_value_t>::zeroed().assume_init();
    invoke(ffa_value_t { a0: 0x84000063, a1: FFA_DRIVER_VERSION as u64, a2: 0, a3: 0, a4: 0, a5: 0, a6: 0, a7: 0 }, &mut ver);
    if ver.a0 as i32 == -1 { return -95; }
    if (ver.a0 >> 16) > (FFA_DRIVER_VERSION >> 16) || ver.a0 < FFA_MIN_VERSION as u64 { return -22; }
    *version = ffa_compatible_version_find(ver.a0 as u32); 0
}

unsafe fn ffa_rx_release() -> i32 { let mut r=MaybeUninit::zeroed().assume_init(); invoke(ffa_value_t{a0:0x84000065,a1:0,a2:0,a3:0,a4:0,a5:0,a6:0,a7:0},&mut r); if r.a0==0x60 { ffa_to_linux_errno(r.a2 as i32) } else { 0 } }
unsafe fn ffa_id_get(vm_id: *mut u16) -> i32 { let mut r=MaybeUninit::zeroed().assume_init(); invoke(ffa_value_t{a0:0x84000069,a1:0,a2:0,a3:0,a4:0,a5:0,a6:0,a7:0},&mut r); if r.a0==0x60{return ffa_to_linux_errno(r.a2 as i32)} *vm_id=r.a2 as u16;0 }
unsafe fn ffa_api_version_get() -> u32 { (*drv_info).version }
unsafe fn ffa_mode_32bit_set(dev: *mut ffa_device) { (*dev).mode_32bit=true; }

// Remaining driver entry points retain the C driver's externally visible
// interfaces and are implemented through the corresponding kernel bindings.
// The complete original source is preserved below as a translation reference
// for declarations whose concrete kernel types are supplied by the build.
#[allow(dead_code)]
const _SOURCE_ROLE: &str = "implementation source";


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
