/*
 *  Copyright (C) 2007-2009 Geert Uytterhoeven
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Linux headers and Amiga architecture headers are supplied by other files.

#[repr(C)]
pub struct Resource {
    pub name: *const ::core::ffi::c_char,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct PlatformDevice {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GayleIdePlatformData {
    pub base: u32,
    pub irqport: u32,
    pub explicit_ack: u32,
}

#[repr(C)]
pub struct ExpansionRom {
    pub er_Manufacturer: u16,
    pub er_Product: u8,
    _private: [u8; 0],
}

pub type ZorroId = u32;

unsafe extern "C" {
    static zorro_num_autocon: usize;
    static zorro_autocon_init: *const ZorroAutocon;
    fn platform_device_register_simple(
        name: *const ::core::ffi::c_char,
        id: i32,
        resource: *const Resource,
        num_resources: u32,
    ) -> *mut PlatformDevice;
    fn platform_device_add_data(
        pdev: *mut PlatformDevice,
        data: *const ::core::ffi::c_void,
        size: usize,
    ) -> i32;
    fn ptr_err_or_zero(ptr: *mut PlatformDevice) -> i32;
    fn ptr_err(ptr: *mut PlatformDevice) -> i32;
    fn is_err(ptr: *mut PlatformDevice) -> bool;
    fn be16_to_cpu(value: u16) -> u16;
    fn mach_is_amiga() -> bool;
    fn amigahw_present(hw: u32) -> bool;
    fn zorro_manuf(id: ZorroId) -> u16;
    fn zorro_prod(id: ZorroId) -> u8;
}

#[repr(C)]
pub struct ZorroAutocon {
    pub rom: ExpansionRom,
}

const IORESOURCE_MEM: u64 = 0x0000_0200;

// Hardware identifiers and Zorro product identifiers are supplied by asm headers.
const ZORRO: u32 = 0;
const ZORRO3: u32 = 1;
const AMI_VIDEO: u32 = 2;
const AMI_AUDIO: u32 = 3;
const AMI_FLOPPY: u32 = 4;
const A3000_SCSI: u32 = 5;
const A4000_SCSI: u32 = 6;
const A1200_IDE: u32 = 7;
const A4000_IDE: u32 = 8;
const AMI_KEYBOARD: u32 = 9;
const AMI_MOUSE: u32 = 10;
const AMI_SERIAL: u32 = 11;
const AMI_PARALLEL: u32 = 12;
const A2000_CLK: u32 = 13;
const A3000_CLK: u32 = 14;
const ZORRO_PROD_MTEC_VIPER_MK_V_E_MATRIX_530_SCSI_IDE: ZorroId = 0;

#[cfg(feature = "config_zorro")]
static ZORRO_RESOURCES: [Resource; 4] = [
    Resource { name: b"Zorro II exp\0".as_ptr() as *const _, start: 0x00e8_0000, end: 0x00ef_ffff, flags: IORESOURCE_MEM },
    Resource { name: b"Zorro II mem\0".as_ptr() as *const _, start: 0x0020_0000, end: 0x009f_ffff, flags: IORESOURCE_MEM },
    Resource { name: b"Zorro III exp\0".as_ptr() as *const _, start: 0xff00_0000, end: 0xffff_ffff, flags: IORESOURCE_MEM },
    Resource { name: b"Zorro III cfg\0".as_ptr() as *const _, start: 0x4000_0000, end: 0x7fff_ffff, flags: IORESOURCE_MEM },
];

#[cfg(feature = "config_zorro")]
unsafe fn amiga_init_bus() -> i32 {
    if !mach_is_amiga() || !amigahw_present(ZORRO) { return -19; }
    let n = if amigahw_present(ZORRO3) { 4 } else { 2 };
    let pdev = platform_device_register_simple(b"amiga-zorro\0".as_ptr() as *const _, -1, ZORRO_RESOURCES.as_ptr(), n);
    ptr_err_or_zero(pdev)
}

#[cfg(feature = "config_zorro")]
unsafe fn z_dev_present(id: ZorroId) -> i32 {
    for i in 0..zorro_num_autocon {
        let rom = &(*zorro_autocon_init.add(i)).rom;
        if be16_to_cpu(rom.er_Manufacturer) == zorro_manuf(id) && rom.er_Product == zorro_prod(id) { return 1; }
    }
    0
}

#[cfg(not(feature = "config_zorro"))]
unsafe fn z_dev_present(_id: ZorroId) -> i32 { 0 }

static A3000_SCSI_RESOURCE: Resource = Resource { name: core::ptr::null(), start: 0xdd0000, end: 0xdd00ff, flags: IORESOURCE_MEM };
static A4000T_SCSI_RESOURCE: Resource = Resource { name: core::ptr::null(), start: 0xdd0000, end: 0xdd0fff, flags: IORESOURCE_MEM };
static A1200_IDE_RESOURCE: Resource = Resource { name: core::ptr::null(), start: 0xda0000, end: 0xda1fff, flags: IORESOURCE_MEM };
static A1200_IDE_PDATA: GayleIdePlatformData = GayleIdePlatformData { base: 0xda0000, irqport: 0xda9000, explicit_ack: 1 };
static A4000_IDE_RESOURCE: Resource = Resource { name: core::ptr::null(), start: 0xdd2000, end: 0xdd3fff, flags: IORESOURCE_MEM };
static A4000_IDE_PDATA: GayleIdePlatformData = GayleIdePlatformData { base: 0xdd2020, irqport: 0xdd3020, explicit_ack: 0 };
static AMIGA_RTC_RESOURCE: Resource = Resource { name: core::ptr::null(), start: 0x00dc0000, end: 0x00dcffff, flags: IORESOURCE_MEM };

unsafe fn amiga_init_devices() -> i32 {
    if !mach_is_amiga() { return -19; }
    if amigahw_present(AMI_VIDEO) { let p = platform_device_register_simple(b"amiga-video\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(AMI_AUDIO) { let p = platform_device_register_simple(b"amiga-audio\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(AMI_FLOPPY) { let p = platform_device_register_simple(b"amiga-floppy\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(A3000_SCSI) { let p = platform_device_register_simple(b"amiga-a3000-scsi\0".as_ptr() as *const _, -1, &A3000_SCSI_RESOURCE, 1); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(A4000_SCSI) { let p = platform_device_register_simple(b"amiga-a4000t-scsi\0".as_ptr() as *const _, -1, &A4000T_SCSI_RESOURCE, 1); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(A1200_IDE) || z_dev_present(ZORRO_PROD_MTEC_VIPER_MK_V_E_MATRIX_530_SCSI_IDE) != 0 {
        let p = platform_device_register_simple(b"amiga-gayle-ide\0".as_ptr() as *const _, -1, &A1200_IDE_RESOURCE, 1); if is_err(p) { return ptr_err(p); }
        let e = platform_device_add_data(p, &A1200_IDE_PDATA as *const _ as *const _, core::mem::size_of::<GayleIdePlatformData>()); if e != 0 { return e; }
    }
    if amigahw_present(A4000_IDE) {
        let p = platform_device_register_simple(b"amiga-gayle-ide\0".as_ptr() as *const _, -1, &A4000_IDE_RESOURCE, 1); if is_err(p) { return ptr_err(p); }
        let e = platform_device_add_data(p, &A4000_IDE_PDATA as *const _ as *const _, core::mem::size_of::<GayleIdePlatformData>()); if e != 0 { return e; }
    }
    if amigahw_present(AMI_KEYBOARD) { let p = platform_device_register_simple(b"amiga-keyboard\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(AMI_MOUSE) { let p = platform_device_register_simple(b"amiga-mouse\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(AMI_SERIAL) { let p = platform_device_register_simple(b"amiga-serial\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(AMI_PARALLEL) { let p = platform_device_register_simple(b"amiga-parallel\0".as_ptr() as *const _, -1, core::ptr::null(), 0); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(A2000_CLK) { let p = platform_device_register_simple(b"rtc-msm6242\0".as_ptr() as *const _, -1, &AMIGA_RTC_RESOURCE, 1); if is_err(p) { return ptr_err(p); } }
    if amigahw_present(A3000_CLK) { let p = platform_device_register_simple(b"rtc-rp5c01\0".as_ptr() as *const _, -1, &AMIGA_RTC_RESOURCE, 1); if is_err(p) { return ptr_err(p); } }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
