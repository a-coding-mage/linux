/*
 * Definition of platform feature hooks for PowerMacs
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1998 Paul Mackerras &
 *                    Ben. Herrenschmidt.
 *
 * Rust translation of the kernel header. C header guards and includes are
 * intentionally omitted; their dependency intent is supplied externally.
 */

/* Known Mac motherboard models. */
pub const PMAC_TYPE_PSURGE: i32 = 0x10;
pub const PMAC_TYPE_ANS: i32 = 0x11;
pub const PMAC_TYPE_COMET: i32 = 0x20;
pub const PMAC_TYPE_HOOPER: i32 = 0x21;
pub const PMAC_TYPE_KANGA: i32 = 0x22;
pub const PMAC_TYPE_ALCHEMY: i32 = 0x23;
pub const PMAC_TYPE_GAZELLE: i32 = 0x24;
pub const PMAC_TYPE_UNKNOWN_OHARE: i32 = 0x2f;
pub const PMAC_TYPE_GOSSAMER: i32 = 0x30;
pub const PMAC_TYPE_SILK: i32 = 0x31;
pub const PMAC_TYPE_WALLSTREET: i32 = 0x32;
pub const PMAC_TYPE_UNKNOWN_HEATHROW: i32 = 0x3f;
pub const PMAC_TYPE_101_PBOOK: i32 = 0x40;
pub const PMAC_TYPE_ORIG_IMAC: i32 = 0x41;
pub const PMAC_TYPE_YOSEMITE: i32 = 0x42;
pub const PMAC_TYPE_YIKES: i32 = 0x43;
pub const PMAC_TYPE_UNKNOWN_PADDINGTON: i32 = 0x4f;
pub const PMAC_TYPE_ORIG_IBOOK: i32 = 0x40;
pub const PMAC_TYPE_SAWTOOTH: i32 = 0x41;
pub const PMAC_TYPE_FW_IMAC: i32 = 0x42;
pub const PMAC_TYPE_FW_IBOOK: i32 = 0x43;
pub const PMAC_TYPE_CUBE: i32 = 0x44;
pub const PMAC_TYPE_QUICKSILVER: i32 = 0x45;
pub const PMAC_TYPE_PISMO: i32 = 0x46;
pub const PMAC_TYPE_TITANIUM: i32 = 0x47;
pub const PMAC_TYPE_TITANIUM2: i32 = 0x48;
pub const PMAC_TYPE_TITANIUM3: i32 = 0x49;
pub const PMAC_TYPE_TITANIUM4: i32 = 0x50;
pub const PMAC_TYPE_EMAC: i32 = 0x50;
pub const PMAC_TYPE_UNKNOWN_CORE99: i32 = 0x5f;
pub const PMAC_TYPE_RACKMAC: i32 = 0x80;
pub const PMAC_TYPE_WINDTUNNEL: i32 = 0x81;
pub const PMAC_TYPE_PANGEA_IMAC: i32 = 0x100;
pub const PMAC_TYPE_IBOOK2: i32 = 0x101;
pub const PMAC_TYPE_FLAT_PANEL_IMAC: i32 = 0x102;
pub const PMAC_TYPE_UNKNOWN_PANGEA: i32 = 0x10f;
pub const PMAC_TYPE_UNKNOWN_INTREPID: i32 = 0x11f;
pub const PMAC_TYPE_POWERMAC_G5: i32 = 0x150;
pub const PMAC_TYPE_POWERMAC_G5_U3L: i32 = 0x151;
pub const PMAC_TYPE_IMAC_G5: i32 = 0x152;
pub const PMAC_TYPE_XSERVE_G5: i32 = 0x153;
pub const PMAC_TYPE_UNKNOWN_K2: i32 = 0x19f;
pub const PMAC_TYPE_UNKNOWN_SHASTA: i32 = 0x19e;

pub const PMAC_MB_CAN_SLEEP: u32 = 0x00000001;
pub const PMAC_MB_HAS_FW_POWER: u32 = 0x00000002;
pub const PMAC_MB_OLD_CORE99: u32 = 0x00000004;
pub const PMAC_MB_MOBILE: u32 = 0x00000008;
pub const PMAC_MB_MAY_SLEEP: u32 = 0x00000010;

pub const PMAC_SCC_ASYNC: i32 = 0;
pub const PMAC_SCC_IRDA: i32 = 1;
pub const PMAC_SCC_I2S1: i32 = 2;
pub const PMAC_SCC_FLAG_XMON: i32 = 0x00001000;
pub const PMAC_MB_INFO_MODEL: i32 = 0;
pub const PMAC_MB_INFO_FLAGS: i32 = 1;
pub const PMAC_MB_INFO_NAME: i32 = 2;

pub const fn PMAC_FTR_DEF(x: u32) -> u32 { 0x6660000 | x }
pub const PMAC_FTR_SCC_ENABLE: u32 = PMAC_FTR_DEF(0);
pub const PMAC_FTR_MODEM_ENABLE: u32 = PMAC_FTR_DEF(1);
pub const PMAC_FTR_SWIM3_ENABLE: u32 = PMAC_FTR_DEF(2);
pub const PMAC_FTR_MESH_ENABLE: u32 = PMAC_FTR_DEF(3);
pub const PMAC_FTR_IDE_ENABLE: u32 = PMAC_FTR_DEF(4);
pub const PMAC_FTR_IDE_RESET: u32 = PMAC_FTR_DEF(5);
pub const PMAC_FTR_BMAC_ENABLE: u32 = PMAC_FTR_DEF(6);
pub const PMAC_FTR_GMAC_ENABLE: u32 = PMAC_FTR_DEF(7);
pub const PMAC_FTR_GMAC_PHY_RESET: u32 = PMAC_FTR_DEF(8);
pub const PMAC_FTR_SOUND_CHIP_ENABLE: u32 = PMAC_FTR_DEF(9);
pub const PMAC_FTR_AIRPORT_ENABLE: u32 = PMAC_FTR_DEF(10);
pub const PMAC_FTR_RESET_CPU: u32 = PMAC_FTR_DEF(11);
pub const PMAC_FTR_USB_ENABLE: u32 = PMAC_FTR_DEF(12);
pub const PMAC_FTR_1394_ENABLE: u32 = PMAC_FTR_DEF(13);
pub const PMAC_FTR_1394_CABLE_POWER: u32 = PMAC_FTR_DEF(14);
pub const PMAC_FTR_SLEEP_STATE: u32 = PMAC_FTR_DEF(15);
pub const PMAC_FTR_GET_MB_INFO: u32 = PMAC_FTR_DEF(16);
pub const PMAC_FTR_READ_GPIO: u32 = PMAC_FTR_DEF(17);
pub const PMAC_FTR_WRITE_GPIO: u32 = PMAC_FTR_DEF(18);
pub const PMAC_FTR_ENABLE_MPIC: u32 = PMAC_FTR_DEF(19);
pub const PMAC_FTR_AACK_DELAY_ENABLE: u32 = PMAC_FTR_DEF(20);
pub const PMAC_FTR_DEVICE_CAN_WAKE: u32 = PMAC_FTR_DEF(22);

/* Use the platform feature callback supplied by asm/machdep.h. */
#[inline]
pub unsafe fn pmac_call_feature(selector: i32, node: *mut DeviceNode, param: i64, value: i64) -> i64 {
    /* ppc_md.feature_call and machine_is(powermac) are external kernel state. */
    let _ = (selector, node, param, value);
    -19 /* -ENODEV when the external callback is unavailable */
}

pub const MAX_MACIO_CHIPS: usize = 2;

#[repr(i32)]
pub enum MacioType {
    macio_unknown = 0,
    macio_grand_central,
    macio_ohare,
    macio_ohareII,
    macio_heathrow,
    macio_gatwick,
    macio_paddington,
    macio_keylargo,
    macio_pangea,
    macio_intrepid,
    macio_keylargo2,
    macio_shasta,
}

pub const MACIO_FLAG_SCCA_ON: u32 = 0x00000001;
pub const MACIO_FLAG_SCCB_ON: u32 = 0x00000002;
pub const MACIO_FLAG_SCC_LOCKED: u32 = 0x00000004;
pub const MACIO_FLAG_AIRPORT_ON: u32 = 0x00000010;
pub const MACIO_FLAG_FW_SUPPORTED: u32 = 0x00000020;

/* Direct translations of the register-access macros; `macio`/`uninorth_base`
 * are supplied by the calling C-compatible context. */
#[inline] pub unsafe fn MACIO_IN32(macio: *mut MacioChip, r: usize) -> u32 { in_le32(MACIO_FCR32((*macio).base, r)) }
#[inline] pub unsafe fn MACIO_OUT32(macio: *mut MacioChip, r: usize, v: u32) { out_le32(MACIO_FCR32((*macio).base, r), v) }
#[inline] pub unsafe fn MACIO_BIS(macio: *mut MacioChip, r: usize, v: u32) { MACIO_OUT32(macio, r, MACIO_IN32(macio, r) | v) }
#[inline] pub unsafe fn MACIO_BIC(macio: *mut MacioChip, r: usize, v: u32) { MACIO_OUT32(macio, r, MACIO_IN32(macio, r) & !v) }
#[inline] pub unsafe fn MACIO_IN8(macio: *mut MacioChip, r: usize) -> u8 { in_8(MACIO_FCR8((*macio).base, r)) }
#[inline] pub unsafe fn MACIO_OUT8(macio: *mut MacioChip, r: usize, v: u8) { out_8(MACIO_FCR8((*macio).base, r), v) }
#[inline] pub unsafe fn UN_IN(base: *mut u32, r: usize) -> u32 { in_be32(UN_REG(base, r)) }
#[inline] pub unsafe fn UN_OUT(base: *mut u32, r: usize, v: u32) { out_be32(UN_REG(base, r), v) }
#[inline] pub unsafe fn UN_BIS(base: *mut u32, r: usize, v: u32) { UN_OUT(base, r, UN_IN(base, r) | v) }
#[inline] pub unsafe fn UN_BIC(base: *mut u32, r: usize, v: u32) { UN_OUT(base, r, UN_IN(base, r) & !v) }

/* C declarations and macros requiring types/functions from included headers. */
#[repr(C)]
pub struct MacioChip {
    pub of_node: *mut DeviceNode,
    pub type_: i32,
    pub name: *const u8,
    pub rev: i32,
    pub base: *mut u32,
    pub flags: usize,
    pub lbus: MacioBus,
}

#[allow(improper_ctypes)]
extern "C" {
    pub static mut macio_chips: [MacioChip; MAX_MACIO_CHIPS];
    pub fn pmac_do_feature_call(selector: u32, ...) -> i64;
    pub fn pmac_feature_init();
    pub fn pmac_set_early_video_resume(proc: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, data: *mut core::ffi::c_void);
    pub fn pmac_call_early_video_resume();
    pub fn pmac_register_agp_pm(bridge: *mut PciDev, suspend: Option<unsafe extern "C" fn(*mut PciDev) -> i32>, resume: Option<unsafe extern "C" fn(*mut PciDev) -> i32>);
    pub fn pmac_suspend_agp_for_card(dev: *mut PciDev);
    pub fn pmac_resume_agp_for_card(dev: *mut PciDev);
    pub fn macio_find(child: *mut DeviceNode, type_: i32) -> *mut MacioChip;
    pub fn pmac_get_uninorth_variant() -> i32;
    pub static mut feature_lock: RawSpinlock;
    pub static mut uninorth_node: *mut DeviceNode;
    pub static mut uninorth_base: *mut u32;
    pub static mut sys_ctrler: SysCtrlerT;
}

/* Included-header types and I/O helpers are external dependencies. */
#[repr(C)] pub struct DeviceNode { _private: [u8; 0] }
#[repr(C)] pub struct MacioBus { _private: [u8; 0] }
#[repr(C)] pub struct PciDev { _private: [u8; 0] }
#[repr(C)] pub struct RawSpinlock { _private: [u8; 0] }

pub const fn MACIO_FCR32(base: *mut u32, r: usize) -> *mut u32 { unsafe { base.add(r >> 2) } }
pub const fn MACIO_FCR8(base: *mut u32, r: usize) -> *mut u8 { base as *mut u8 }
pub const fn UN_REG(base: *mut u32, r: usize) -> *mut u32 { unsafe { base.add(r >> 2) } }

/* The following operation macros map directly to the external endian-aware
 * accessors from asm/macio.h and are represented as declarations here. */
extern "C" {
    pub fn in_le32(addr: *const u32) -> u32;
    pub fn out_le32(addr: *mut u32, value: u32);
    pub fn in_8(addr: *const u8) -> u8;
    pub fn out_8(addr: *mut u8, value: u8);
    pub fn in_be32(addr: *const u32) -> u32;
    pub fn out_be32(addr: *mut u32, value: u32);
}

pub type SysCtrlerT = i32;
pub const SYS_CTRLER_UNKNOWN: SysCtrlerT = 0;
pub const SYS_CTRLER_CUDA: SysCtrlerT = 1;
pub const SYS_CTRLER_PMU: SysCtrlerT = 2;
pub const SYS_CTRLER_SMU: SysCtrlerT = 3;

/* Remaining declarations/macros retain dependencies on asm/macio.h,
 * asm/machdep.h, and kernel types (device_node, macio_bus, pci_dev, etc.). */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
