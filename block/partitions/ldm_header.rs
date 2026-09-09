// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ldm - Part of the Linux-NTFS project.
 *
 * Copyright (C) 2001,2002 Richard Russon <ldm@flatcap.org>
 * Copyright (c) 2001-2007 Anton Altaparmakov
 * Copyright (C) 2001,2002 Jakob Kemi <jakob.kemi@telia.com>
 *
 * Documentation is available at http://www.linux-ntfs.org/doku.php?id=downloads
 */

// External C dependencies supplied by other translated files:
// struct list_head; type uuid_t;

// Magic numbers in CPU format.
pub const MAGIC_VMDB: u32 = 0x564D4442; // VMDB
pub const MAGIC_VBLK: u32 = 0x56424C4B; // VBLK
pub const MAGIC_PRIVHEAD: u64 = 0x5052495648454144; // PRIVHEAD
pub const MAGIC_TOCBLOCK: u64 = 0x544F43424C4F434B; // TOCBLOCK

// The defined vblk types.
pub const VBLK_VOL5: u8 = 0x51; // Volume,     version 5
pub const VBLK_CMP3: u8 = 0x32; // Component,  version 3
pub const VBLK_PRT3: u8 = 0x33; // Partition,  version 3
pub const VBLK_DSK3: u8 = 0x34; // Disk,       version 3
pub const VBLK_DSK4: u8 = 0x44; // Disk,       version 4
pub const VBLK_DGR3: u8 = 0x35; // Disk Group, version 3
pub const VBLK_DGR4: u8 = 0x45; // Disk Group, version 4

// vblk flags indicating extra information will be present
pub const VBLK_FLAG_COMP_STRIPE: u8 = 0x10;
pub const VBLK_FLAG_PART_INDEX: u8 = 0x08;
pub const VBLK_FLAG_DGR3_IDS: u8 = 0x08;
pub const VBLK_FLAG_DGR4_IDS: u8 = 0x08;
pub const VBLK_FLAG_VOLU_ID1: u8 = 0x08;
pub const VBLK_FLAG_VOLU_ID2: u8 = 0x20;
pub const VBLK_FLAG_VOLU_SIZE: u8 = 0x80;
pub const VBLK_FLAG_VOLU_DRIVE: u8 = 0x02;

// size of a vblk's static parts
pub const VBLK_SIZE_HEAD: u32 = 16;
pub const VBLK_SIZE_CMP3: u32 = 22; // Name and version
pub const VBLK_SIZE_DGR3: u32 = 12;
pub const VBLK_SIZE_DGR4: u32 = 44;
pub const VBLK_SIZE_DSK3: u32 = 12;
pub const VBLK_SIZE_DSK4: u32 = 45;
pub const VBLK_SIZE_PRT3: u32 = 28;
pub const VBLK_SIZE_VOL5: u32 = 58;

// component types
pub const COMP_STRIPE: u8 = 0x01; // Stripe-set
pub const COMP_BASIC: u8 = 0x02; // Basic disk
pub const COMP_RAID: u8 = 0x03; // Raid-set

// Other constants.
pub const LDM_DB_SIZE: u32 = 2048; // Size in sectors (= 1MiB).
pub const OFF_PRIV1: u32 = 6; // Offset of the first privhead relative to the start of the device in sectors
pub const OFF_PRIV2: u32 = 1856; // Backup private headers.
pub const OFF_PRIV3: u32 = 2047;
pub const OFF_TOCB1: u32 = 1; // Tables of contents.
pub const OFF_TOCB2: u32 = 2;
pub const OFF_TOCB3: u32 = 2045;
pub const OFF_TOCB4: u32 = 2046;
pub const OFF_VMDB: u32 = 17; // List of partitions.
pub const LDM_PARTITION: u8 = 0x42; // Formerly SFS (Landis).
pub const TOC_BITMAP1: &str = "config"; // Names of the two defined bitmaps in the TOCBLOCK.
pub const TOC_BITMAP2: &str = "log";

#[repr(C)]
pub struct frag {
    pub list: list_head,
    pub group: u32,
    pub num: u8,
    pub rec: u8,
    pub map: u8,
    pub data: [u8; 0],
}

// In memory LDM database structures.

#[repr(C)]
pub struct privhead {
    pub ver_major: u16,
    pub ver_minor: u16,
    pub logical_disk_start: u64,
    pub logical_disk_size: u64,
    pub config_start: u64,
    pub config_size: u64,
    pub disk_id: uuid_t,
}

#[repr(C)]
pub struct tocblock {
    pub bitmap1_name: [u8; 16],
    pub bitmap1_start: u64,
    pub bitmap1_size: u64,
    pub bitmap2_name: [u8; 16],
    pub bitmap2_start: u64,
    pub bitmap2_size: u64,
}

#[repr(C)]
pub struct vmdb {
    pub ver_major: u16,
    pub ver_minor: u16,
    pub vblk_size: u32,
    pub vblk_offset: u32,
    pub last_vblk_seq: u32,
}

#[repr(C)]
pub struct vblk_comp {
    pub state: [u8; 16],
    pub parent_id: u64,
    pub r#type: u8,
    pub children: u8,
    pub chunksize: u16,
}

#[repr(C)]
pub struct vblk_dgrp {
    pub disk_id: [u8; 64],
}

#[repr(C)]
pub struct vblk_disk {
    pub disk_id: uuid_t,
    pub alt_name: [u8; 128],
}

#[repr(C)]
pub struct vblk_part {
    pub start: u64,
    pub size: u64,
    pub volume_offset: u64,
    pub parent_id: u64,
    pub disk_id: u64,
    pub partnum: u8,
}

#[repr(C)]
pub struct vblk_volu {
    pub volume_type: [u8; 16],
    pub volume_state: [u8; 16],
    pub guid: [u8; 16],
    pub drive_hint: [u8; 4],
    pub size: u64,
    pub partition_type: u8,
}

#[repr(C)]
pub struct vblk_head {
    pub group: u32,
    pub rec: u16,
    pub nrec: u16,
}

#[repr(C)]
pub union vblk_vblk {
    pub comp: vblk_comp,
    pub dgrp: vblk_dgrp,
    pub disk: vblk_disk,
    pub part: vblk_part,
    pub volu: vblk_volu,
}

#[repr(C)]
pub struct vblk {
    pub name: [u8; 64],
    pub obj_id: u64,
    pub sequence: u32,
    pub flags: u8,
    pub r#type: u8,
    pub vblk: vblk_vblk,
    pub list: list_head,
}

#[repr(C)]
pub struct ldmdb {
    pub ph: privhead,
    pub toc: tocblock,
    pub vm: vmdb,
    pub v_dgrp: list_head,
    pub v_disk: list_head,
    pub v_volu: list_head,
    pub v_comp: list_head,
    pub v_part: list_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
