/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for NTFS kernel journal (LogFile) handling.
 *
 * Copyright (c) 2000-2005 Anton Altaparmakov
 */

// Dependency supplied by the translated layout definitions: __le16, __le32,
// __le64, and struct inode.

/*
 * Journal (LogFile) organization:
 *
 * Two restart areas present in the first two pages (restart pages, one restart
 * area in each page).  When the volume is dismounted they should be identical,
 * except for the update sequence array which usually has a different update
 * sequence number.
 *
 * These are followed by log records organized in pages headed by a log record
 * header going up to log file size.  Not all pages contain log records when a
 * volume is first formatted, but as the volume ages, all records will be used.
 * When the log file fills up, the records at the beginning are purged (by
 * modifying the oldest_lsn to a higher value presumably) and writing begins
 * at the beginning of the file.  Effectively, the log file is viewed as a
 * circular entity.
 *
 * NOTE: Windows NT, 2000, and XP all use log file version 1.1 but they accept
 * versions <= 1.x, including 0.-1.  (Yes, that is a minus one in there!)  We
 * probably only want to support 1.1 as this seems to be the current version
 * and we don't know how that differs from the older versions.  The only
 * exception is if the journal is clean as marked by the two restart pages
 * then it doesn't matter whether we are on an earlier version.  We can just
 * reinitialize the logfile and start again with version 1.1.
 */

/* Some LogFile related constants. */
pub const MAX_LOG_FILE_SIZE: u64 = 0x100000000;
pub const DEFAULT_LOG_PAGE_SIZE: u32 = 4096;
pub const MIN_LOG_RECORD_PAGES: u32 = 48;

/* Log file restart page header (begins the restart area). */
#[repr(C, packed)]
pub struct restart_page_header {
    pub magic: __le32,
    pub usa_ofs: __le16,
    pub usa_count: __le16,
    pub chkdsk_lsn: __le64,
    pub system_page_size: __le32,
    pub log_page_size: __le32,
    pub restart_area_offset: __le16,
    pub minor_ver: __le16,
    pub major_ver: __le16,
}

/* Constant for log client indices meaning there are no client records. */
pub const LOGFILE_NO_CLIENT: __le16 = 0xffff;
pub const LOGFILE_NO_CLIENT_CPU: u16 = 0xffff;

/* Known RESTART_AREA flags (16-bit). */
pub const RESTART_VOLUME_IS_CLEAN: __le16 = 0x0002;
pub const RESTART_SPACE_FILLER: __le16 = 0xffff;

/* Log file restart area record. */
#[repr(C, packed)]
pub struct restart_area {
    pub current_lsn: __le64,
    pub log_clients: __le16,
    pub client_free_list: __le16,
    pub client_in_use_list: __le16,
    pub flags: __le16,
    pub seq_number_bits: __le32,
    pub restart_area_length: __le16,
    pub client_array_offset: __le16,
    pub file_size: __le64,
    pub last_lsn_data_length: __le32,
    pub log_record_header_length: __le16,
    pub log_page_data_offset: __le16,
    pub restart_log_open_count: __le32,
    pub reserved: __le32,
}

/* Log client record. */
#[repr(C, packed)]
pub struct log_client_record {
    pub oldest_lsn: __le64,
    pub client_restart_lsn: __le64,
    pub prev_client: __le16,
    pub next_client: __le16,
    pub seq_number: __le16,
    pub reserved: [u8; 6],
    pub client_name_length: __le32,
    pub client_name: [__le16; 64],
}

extern "C" {
    pub fn ntfs_check_logfile(
        log_vi: *mut inode,
        rp: *mut *mut restart_page_header,
    ) -> bool;
    pub fn ntfs_empty_logfile(log_vi: *mut inode) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
