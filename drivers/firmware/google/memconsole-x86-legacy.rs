// SPDX-License-Identifier: GPL-2.0-only
/*
 * memconsole-x86-legacy.c
 *
 * EBDA specific parts of the memory based BIOS console.
 *
 * Copyright 2017 Google Inc.
 */

// Linux kernel headers and the local memconsole header provide the external
// types, constants, and functions referenced below.

use core::ffi::{c_char, c_int, c_void};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type SizeT = usize;
type SSizeT = isize;
type LoffT = i64;

const BIOS_MEMCONSOLE_V1_MAGIC: U32 = 0xDEADBABE;
const BIOS_MEMCONSOLE_V2_MAGIC: U32 = (b'M' as U32)
    | ((b'C' as U32) << 8)
    | ((b'O' as U32) << 16)
    | ((b'N' as U32) << 24);

#[repr(C, packed)]
pub struct BiosmemconEbdaV1 {
    pub enabled: U8,
    pub buffer_addr: U32,
    pub start: U16,
    pub end: U16,
    pub num_chars: U16,
    pub wrapped: U8,
}

#[repr(C, packed)]
pub struct BiosmemconEbdaV2 {
    pub buffer_addr: U32,
    // Misdocumented as number of pages!
    pub num_bytes: U16,
    pub start: U16,
    pub end: U16,
}

#[repr(C)]
pub union BiosmemconEbdaPayload {
    pub v1: BiosmemconEbdaV1,
    pub v2: BiosmemconEbdaV2,
}

#[repr(C, packed)]
pub struct BiosmemconEbda {
    pub signature: U32,
    pub payload: BiosmemconEbdaPayload,
}

static mut MEMCONSOLE_BASEADDR: *mut c_char = core::ptr::null_mut();
static mut MEMCONSOLE_LENGTH: SizeT = 0;

extern "C" {
    fn memory_read_from_buffer(
        buf: *mut c_char,
        count: SizeT,
        pos: *mut LoffT,
        addr: *const c_char,
        size: SizeT,
    ) -> SSizeT;
    fn phys_to_virt(address: usize) -> *mut c_void;
    fn get_bios_ebda() -> u32;
    fn memconsole_setup(read: unsafe extern "C" fn(*mut c_char, LoffT, SizeT) -> SSizeT);
    fn memconsole_sysfs_init() -> c_int;
    fn memconsole_exit();
    fn dmi_check_system(table: *const c_void) -> bool;
}

unsafe extern "C" fn memconsole_read(buf: *mut c_char, pos: LoffT, count: SizeT) -> SSizeT {
    let mut offset = pos;
    memory_read_from_buffer(
        buf,
        count,
        &mut offset,
        MEMCONSOLE_BASEADDR,
        MEMCONSOLE_LENGTH,
    )
}

unsafe fn found_v1_header(hdr: *mut BiosmemconEbda) {
    // pr_info("memconsole: BIOS console v1 EBDA structure found at %p\n", hdr);
    let v1 = &(*hdr).payload.v1;
    // pr_info("memconsole: BIOS console buffer at 0x%.8x, start = %d, end = %d, num = %d\n", ...);
    MEMCONSOLE_BASEADDR = phys_to_virt(v1.buffer_addr as usize) as *mut c_char;
    MEMCONSOLE_LENGTH = v1.num_chars as SizeT;
    memconsole_setup(memconsole_read);
}

unsafe fn found_v2_header(hdr: *mut BiosmemconEbda) {
    // pr_info("memconsole: BIOS console v2 EBDA structure found at %p\n", hdr);
    let v2 = &(*hdr).payload.v2;
    // pr_info("memconsole: BIOS console buffer at 0x%.8x, start = %d, end = %d, num_bytes = %d\n", ...);
    MEMCONSOLE_BASEADDR = phys_to_virt((v2.buffer_addr + v2.start as U32) as usize)
        as *mut c_char;
    MEMCONSOLE_LENGTH = (v2.end - v2.start) as SizeT;
    memconsole_setup(memconsole_read);
}

/* Search through the EBDA for the BIOS Memory Console and set the global
 * variables to point to it. Return true if found. */
unsafe fn memconsole_ebda_init() -> bool {
    let address = get_bios_ebda();
    if address == 0 {
        // pr_info("memconsole: BIOS EBDA non-existent.\n");
        return false;
    }

    // EBDA length is byte 0 of EBDA (in KB).
    let mut length = *(phys_to_virt(address as usize) as *const U8) as SizeT;
    length <<= 10; // convert to bytes

    // Search through EBDA; the signature is not necessarily dword-aligned.
    for cur in 0..length {
        let hdr = phys_to_virt((address as SizeT) + cur) as *mut BiosmemconEbda;
        if (*hdr).signature == BIOS_MEMCONSOLE_V1_MAGIC {
            found_v1_header(hdr);
            return true;
        }
        if (*hdr).signature == BIOS_MEMCONSOLE_V2_MAGIC {
            found_v2_header(hdr);
            return true;
        }
    }

    // pr_info("memconsole: BIOS console EBDA structure not found!\n");
    false
}

// DMI table: one entry matching DMI_BOARD_VENDOR == "Google, Inc.", followed
// by an empty terminator entry. __initconst and MODULE_DEVICE_TABLE are build
// time kernel annotations and are intentionally represented as comments.
static MEMCONSOLE_DMI_TABLE: [u8; 0] = [];

unsafe fn memconsole_find() -> bool {
    if !dmi_check_system(MEMCONSOLE_DMI_TABLE.as_ptr() as *const c_void) {
        return false;
    }
    memconsole_ebda_init()
}

unsafe fn memconsole_x86_init() -> c_int {
    if !memconsole_find() {
        return -19; // -ENODEV
    }
    memconsole_sysfs_init()
}

unsafe fn memconsole_x86_exit() {
    memconsole_exit();
}

// module_init(memconsole_x86_init);
// module_exit(memconsole_x86_exit);
// MODULE_AUTHOR("Google, Inc.");
// MODULE_DESCRIPTION("EBDA specific parts of the memory based BIOS console.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
