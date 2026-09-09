/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/include/asm/ecard.h
 *
 * definitions for expansion cards
 *
 * This is a new system as from Linux 1.2.3
 *
 * Changelog:
 *  11-12-1996 RMK Further minor improvements
 *  12-09-1997 RMK Added interrupt enable/disable for card level
 *
 * Reference: Acorns Risc OS 3 Programmers Reference Manuals.
 */

pub const MANU_ACORN: u16 = 0x0000;
pub const PROD_ACORN_SCSI: u16 = 0x0002;
pub const PROD_ACORN_ETHER1: u16 = 0x0003;
pub const PROD_ACORN_MFM: u16 = 0x000b;
pub const MANU_ANT2: u16 = 0x0011;
pub const PROD_ANT_ETHER3: u16 = 0x00a4;
pub const MANU_ATOMWIDE: u16 = 0x0017;
pub const PROD_ATOMWIDE_3PSERIAL: u16 = 0x0090;
pub const MANU_IRLAM_INSTRUMENTS: u16 = 0x001f;
pub const MANU_IRLAM_INSTRUMENTS_ETHERN: u16 = 0x5678;
pub const MANU_OAK: u16 = 0x0021;
pub const PROD_OAK_SCSI: u16 = 0x0058;
pub const MANU_MORLEY: u16 = 0x002b;
pub const PROD_MORLEY_SCSI_UNCACHED: u16 = 0x0067;
pub const MANU_CUMANA: u16 = 0x003a;
pub const PROD_CUMANA_SCSI_2: u16 = 0x003a;
pub const PROD_CUMANA_SCSI_1: u16 = 0x00a0;
pub const MANU_ICS: u16 = 0x003c;
pub const PROD_ICS_IDE: u16 = 0x00ae;
pub const MANU_ICS2: u16 = 0x003d;
pub const PROD_ICS2_IDE: u16 = 0x00ae;
pub const MANU_SERPORT: u16 = 0x003f;
pub const PROD_SERPORT_DSPORT: u16 = 0x00b9;
pub const MANU_ARXE: u16 = 0x0041;
pub const PROD_ARXE_SCSI: u16 = 0x00be;
pub const MANU_I3: u16 = 0x0046;
pub const PROD_I3_ETHERLAN500: u16 = 0x00d4;
pub const PROD_I3_ETHERLAN600: u16 = 0x00ec;
pub const PROD_I3_ETHERLAN600A: u16 = 0x011e;
pub const MANU_ANT: u16 = 0x0053;
pub const PROD_ANT_ETHERM: u16 = 0x00d8;
pub const PROD_ANT_ETHERB: u16 = 0x00e4;
pub const MANU_ALSYSTEMS: u16 = 0x005b;
pub const PROD_ALSYS_SCSIATAPI: u16 = 0x0107;
pub const MANU_MCS: u16 = 0x0063;
pub const PROD_MCS_CONNECT32: u16 = 0x0125;
pub const MANU_EESOX: u16 = 0x0064;
pub const PROD_EESOX_SCSI2: u16 = 0x008c;
pub const MANU_YELLOWSTONE: u16 = 0x0096;
pub const PROD_YELLOWSTONE_RAPIDE32: u16 = 0x0120;

pub const MAX_ECARDS: usize = 9;

#[repr(C)]
pub struct ecard_id {
    pub manufacturer: u16,
    pub product: u16,
    pub data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct in_ecid {
    pub product: u16,
    pub manufacturer: u16,
    /* C bitfields id:4, cd:1, is:1, w:2, packed into one byte. */
    pub flags: u8,
    pub country: u8,
    pub irqmask: u8,
    pub fiqmask: u8,
    pub irqoff: core::ffi::c_ulong,
    pub fiqoff: core::ffi::c_ulong,
}

pub type ecard_t = expansion_card;
pub type loader_t = *mut core::ffi::c_ulong;

#[repr(C)]
pub struct expansion_card_ops {
    pub irqenable: Option<unsafe extern "C" fn(*mut ecard_t, i32)>,
    pub irqdisable: Option<unsafe extern "C" fn(*mut ecard_t, i32)>,
    pub irqpending: Option<unsafe extern "C" fn(*mut ecard_t) -> i32>,
    pub fiqenable: Option<unsafe extern "C" fn(*mut ecard_t, i32)>,
    pub fiqdisable: Option<unsafe extern "C" fn(*mut ecard_t, i32)>,
    pub fiqpending: Option<unsafe extern "C" fn(*mut ecard_t) -> i32>,
}
pub type expansioncard_ops_t = expansion_card_ops;

pub const ECARD_NUM_RESOURCES: usize = 6;
pub const ECARD_RES_IOCSLOW: usize = 0;
pub const ECARD_RES_IOCMEDIUM: usize = 1;
pub const ECARD_RES_IOCFAST: usize = 2;
pub const ECARD_RES_IOCSYNC: usize = 3;
pub const ECARD_RES_MEMC: usize = 4;
pub const ECARD_RES_EASI: usize = 5;

#[repr(C)]
pub struct expansion_card {
    pub next: *mut expansion_card,
    pub dev: device,
    pub resource: [resource; ECARD_NUM_RESOURCES],
    pub irqaddr: *mut core::ffi::c_void,
    pub fiqaddr: *mut core::ffi::c_void,
    pub irqmask: u8,
    pub fiqmask: u8,
    pub claimed: u8,
    pub easi: u8,
    pub irq_data: *mut core::ffi::c_void,
    pub fiq_data: *mut core::ffi::c_void,
    pub ops: *const expansioncard_ops_t,
    /* CONST fields are const in the C build when ECARD_C is not defined. */
    pub slot_no: u32,
    pub dma: u32,
    pub irq: u32,
    pub fiq: u32,
    pub cid: in_ecid,
    pub card_desc: *const core::ffi::c_char,
    pub loader: loader_t,
    pub dma_mask: u64,
}

extern "C" {
    pub fn ecard_setirq(ec: *mut expansion_card, ops: *const expansion_card_ops, irq_data: *mut core::ffi::c_void);
}

#[repr(C)]
pub union in_chunk_dir_d {
    pub string: [u8; 256],
    pub data: [u8; 1],
}
#[repr(C)]
pub struct in_chunk_dir {
    pub start_offset: u32,
    pub d: in_chunk_dir_d,
}

extern "C" {
    pub fn ecard_readchunk(cd: *mut in_chunk_dir, ec: *mut expansion_card, id: i32, num: i32) -> i32;
    pub fn ecard_request_resources(ec: *mut expansion_card) -> i32;
    pub fn ecard_release_resources(ec: *mut expansion_card);
    pub fn ecardm_iomap(ec: *mut expansion_card, res: u32, offset: core::ffi::c_ulong, maxsize: core::ffi::c_ulong) -> *mut core::ffi::c_void;
    pub static ecard_bus_type: bus_type;
    pub fn ecard_register_driver(driver: *mut ecard_driver) -> i32;
    pub fn ecard_remove_driver(driver: *mut ecard_driver);
}

#[repr(C)]
pub struct ecard_driver {
    pub probe: Option<unsafe extern "C" fn(*mut expansion_card, *const ecard_id) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut expansion_card)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut expansion_card)>,
    pub id_table: *const ecard_id,
    pub id: u32,
    pub drv: device_driver,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
