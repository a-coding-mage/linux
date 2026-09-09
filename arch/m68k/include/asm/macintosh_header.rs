/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// not implemented here.

/// Opaque external IRQ data type supplied by the IRQ subsystem.
pub enum irq_data {}

/*
 *	Apple Macintoshisms
 */

extern "C" {
    pub fn mac_reset();
    pub fn mac_poweroff();
    pub fn mac_init_IRQ();

    pub fn mac_irq_enable(data: *mut irq_data);
    pub fn mac_irq_disable(data: *mut irq_data);

    pub fn mac_pram_read_byte(_: i32) -> u8;
    pub fn mac_pram_write_byte(_: u8, _: i32);
    pub fn mac_pram_get_size() -> isize;
}

/*
 *	Macintosh Table
 */

#[repr(C)]
pub struct mac_model {
    pub ident: i16,
    pub name: *mut core::ffi::c_char,
    pub adb_type: i8,
    pub via_type: i8,
    pub scsi_type: i8,
    pub ide_type: i8,
    pub scc_type: i8,
    pub ether_type: i8,
    pub expansion_type: i8,
    pub floppy_type: i8,
}

pub const MAC_ADB_NONE: i32 = 0;
pub const MAC_ADB_II: i32 = 1;
pub const MAC_ADB_EGRET: i32 = 2;
pub const MAC_ADB_CUDA: i32 = 3;
pub const MAC_ADB_PB1: i32 = 4;
pub const MAC_ADB_PB2: i32 = 5;
pub const MAC_ADB_IOP: i32 = 6;

pub const MAC_VIA_II: i32 = 1;
pub const MAC_VIA_IICI: i32 = 2;
pub const MAC_VIA_QUADRA: i32 = 3;

pub const MAC_SCSI_NONE: i32 = 0;
pub const MAC_SCSI_OLD: i32 = 1;
pub const MAC_SCSI_QUADRA: i32 = 2;
pub const MAC_SCSI_QUADRA2: i32 = 3;
pub const MAC_SCSI_QUADRA3: i32 = 4;
pub const MAC_SCSI_IIFX: i32 = 5;
pub const MAC_SCSI_DUO: i32 = 6;
pub const MAC_SCSI_LC: i32 = 7;

pub const MAC_IDE_NONE: i32 = 0;
pub const MAC_IDE_QUADRA: i32 = 1;
pub const MAC_IDE_PB: i32 = 2;
pub const MAC_IDE_BABOON: i32 = 3;

pub const MAC_SCC_II: i32 = 1;
pub const MAC_SCC_IOP: i32 = 2;
pub const MAC_SCC_QUADRA: i32 = 3;
pub const MAC_SCC_PSC: i32 = 4;

pub const MAC_ETHER_NONE: i32 = 0;
pub const MAC_ETHER_SONIC: i32 = 1;
pub const MAC_ETHER_MACE: i32 = 2;

pub const MAC_EXP_NONE: i32 = 0;
pub const MAC_EXP_PDS: i32 = 1; // Accepts only a PDS card
pub const MAC_EXP_NUBUS: i32 = 2; // Accepts only NuBus card(s)
pub const MAC_EXP_PDS_NUBUS: i32 = 3; // Accepts PDS card and/or NuBus card(s)
pub const MAC_EXP_PDS_COMM: i32 = 4; // Accepts PDS card or Comm Slot card

pub const MAC_FLOPPY_UNSUPPORTED: i32 = 0;
pub const MAC_FLOPPY_SWIM_IOP: i32 = 1;
pub const MAC_FLOPPY_OLD: i32 = 2;
pub const MAC_FLOPPY_QUADRA: i32 = 3;
pub const MAC_FLOPPY_LC: i32 = 4;

extern "C" {
    pub static mut macintosh_config: *mut mac_model;
}

/*
 * Internal representation of the Mac hardware, filled in from bootinfo
 */

#[repr(C)]
pub struct mac_booter_data {
    pub videoaddr: usize,
    pub videorow: usize,
    pub videodepth: usize,
    pub dimensions: usize,
    pub boottime: usize,
    pub gmtbias: usize,
    pub videological: usize,
    pub sccbase: usize,
    pub id: usize,
    pub memsize: usize,
    pub cpuid: usize,
    pub rombase: usize,
}

extern "C" {
    pub static mut mac_bi_data: mac_booter_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
