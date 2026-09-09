/* SPDX-License-Identifier: GPL-2.0 */
/*
 * scsicam.h - SCSI CAM support functions, use for HDIO_GETGEO, etc.
 *
 * Copyright 1993, 1994 Drew Eckhardt
 *      Visionary Computing
 *      (Unix and Linux consulting and custom programming)
 *      drew@Colorado.EDU
 *\t+1 (303) 786-7975
 *
 * For more information, please consult the SCSI-CAM draft.
 */

// C header guard: SCSICAM_H

#[repr(C)]
pub struct gendisk {
    _private: [u8; 0],
}

extern "C" {
    pub fn scsicam_bios_param(
        disk: *mut gendisk,
        capacity: sector_t,
        ip: *mut i32,
    ) -> i32;

    pub fn scsi_partsize(
        disk: *mut gendisk,
        capacity: sector_t,
        geom: *mut i32,
    ) -> bool;

    pub fn scsi_bios_ptable(disk: *mut gendisk) -> *mut u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
