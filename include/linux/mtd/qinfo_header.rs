/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependencies supplied by the corresponding Linux MTD headers:
 * map_word, map_info, and flchip.
 */

/*
 * qinfo_chip structure contains necessary qinfo records data
 * @DevSizeShift - Device size 2^n bytes
 * @BufSizeShift - Program buffer size 2^n bytes
 * @TotalBlocksNum - Total number of blocks
 * @UniformBlockSizeShift - Uniform block size 2^UniformBlockSizeShift bytes
 * @HWPartsNum - Number of hardware partitions
 * @SuspEraseSupp - Suspend erase supported
 * @SingleWordProgTime - Single word program 2^SingleWordProgTime u-sec
 * @ProgBufferTime - Program buffer write 2^ProgBufferTime u-sec
 * @BlockEraseTime - Block erase 2^BlockEraseTime m-sec
 */
#[repr(C)]
pub struct qinfo_chip {
    /* General device info */
    pub DevSizeShift: u16,
    pub BufSizeShift: u16,
    /* Erase block information */
    pub TotalBlocksNum: u16,
    pub UniformBlockSizeShift: u16,
    /* Partition information */
    pub HWPartsNum: u16,
    /* Optional features */
    pub SuspEraseSupp: u16,
    /* Operation typical time */
    pub SingleWordProgTime: u16,
    pub ProgBufferTime: u16,
    pub BlockEraseTime: u16,
}

/* lpddr_private describes lpddr flash chip in memory map
 * @ManufactId - Chip Manufacture ID
 * @DevId - Chip Device ID
 * @qinfo - pointer to qinfo records describing the chip
 * @numchips - number of chips including virual RWW partitions
 * @chipshift - Chip/partition size 2^chipshift
 * @chips - per-chip data structure
 */
#[repr(C)]
pub struct lpddr_private {
    pub ManufactId: u16,
    pub DevId: u16,
    pub qinfo: *mut qinfo_chip,
    pub numchips: core::ffi::c_int,
    pub chipshift: core::ffi::c_ulong,
    pub chips: [flchip; 0],
}

/* qinfo_query_info structure contains request information for
 * each qinfo record
 * @major - major number of qinfo record
 * @major - minor number of qinfo record
 * @id_str - descriptive string to access the record
 * @desc - detailed description for the qinfo record
 */
#[repr(C)]
pub struct qinfo_query_info {
    pub major: u8,
    pub minor: u8,
    pub id_str: *mut core::ffi::c_char,
    pub desc: *mut core::ffi::c_char,
}

/* defines for fixup usage */
pub const LPDDR_MFR_ANY: u16 = 0xffff;
pub const LPDDR_ID_ANY: u16 = 0xffff;
pub const NUMONYX_MFGR_ID: u16 = 0x0089;
pub const R18_DEVICE_ID_1G: u16 = 0x893c;

pub unsafe fn lpddr_build_cmd(cmd: core::ffi::c_ulong, _map: *mut map_info) -> map_word {
    let mut val: map_word = core::mem::zeroed();
    val.x[0] = cmd;
    val
}

#[macro_export]
macro_rules! CMD {
    ($x:expr) => {{
        $crate::lpddr_build_cmd($x, map)
    }};
}

#[macro_export]
macro_rules! CMDVAL {
    ($cmd:expr) => {
        $cmd.x[0]
    };
}

unsafe extern "C" {
    pub fn lpddr_cmdset(map: *mut map_info) -> *mut mtd_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
