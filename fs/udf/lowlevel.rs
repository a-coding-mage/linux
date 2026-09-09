// SPDX-License-Identifier: GPL-2.0-only
/*
 * lowlevel.c
 *
 * PURPOSE
 *  Low Level Device Routines for the UDF filesystem
 *
 * COPYRIGHT
 *  (C) 1999-2001 Ben Fennema
 *
 * HISTORY
 *
 *  03/26/99 blf  Created.
 */

// Dependencies supplied by the surrounding UDF and Linux compatibility code:
// udfdecl.h, linux/blkdev.h, linux/cdrom.h, linux/uaccess.h, and udf_sb.h.

pub unsafe fn udf_get_last_session(sb: *mut super_block) -> u32 {
    let cdi = disk_to_cdi((*(*sb).s_bdev).bd_disk);
    let mut ms_info: cdrom_multisession = core::mem::zeroed();

    if cdi.is_null() {
        udf_debug!("CDROMMULTISESSION not supported.\n");
        return 0;
    }

    ms_info.addr_format = CDROM_LBA;
    if cdrom_multisession(cdi, &mut ms_info) == 0 {
        udf_debug!(
            "XA disk: %s, vol_desc_start=%d\n",
            if ms_info.xa_flag != 0 { "yes" } else { "no" },
            ms_info.addr.lba
        );
        if ms_info.xa_flag != 0 {
            // necessary for a valid ms_info.addr
            return ms_info.addr.lba;
        }
    }
    0
}

pub unsafe fn udf_get_last_block(sb: *mut super_block) -> udf_pblk_t {
    let cdi = disk_to_cdi((*(*sb).s_bdev).bd_disk);
    let mut lblock: libc::c_ulong = 0;

    /*
     * The cdrom layer call failed or returned obviously bogus value?
     * Try using the device size...
     */
    if cdi.is_null() || cdrom_get_last_written(cdi, &mut lblock) != 0 || lblock == 0 {
        if sb_bdev_nr_blocks(sb) > !(0 as udf_pblk_t) {
            return 0;
        }
        lblock = sb_bdev_nr_blocks(sb);
    }

    if lblock != 0 {
        return lblock - 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
