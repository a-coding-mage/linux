// SPDX-License-Identifier: GPL-2.0-or-later
/* Netfs support statistics
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

use std::ffi::{c_char, c_int, c_void};

// Types and functions supplied by the kernel headers.
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn atomic_read(v: *const atomic_t) -> c_int;
    fn seq_printf(m: *mut seq_file, fmt: *const c_char, ...);
    fn fscache_stats_show(m: *mut seq_file) -> c_int;
}

#[no_mangle]
pub static mut netfs_n_rh_dio_read: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_readahead: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_read_folio: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_read_single: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_rreq: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_sreq: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_download: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_download_done: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_download_failed: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_download_instead: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_read: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_read_done: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_read_failed: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_zero: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_short_read: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_write: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_write_begin: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_write_done: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_write_failed: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_write_zskip: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_retry_read_req: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_rh_retry_read_subreq: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_buffered_write: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_writethrough: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_dio_write: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_writepages: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_copy_to_cache: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_wstream_conflict: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_upload: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_upload_done: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_upload_failed: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_write: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_write_done: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_write_failed: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_retry_write_req: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wh_retry_write_subreq: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wb_lock_skip: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_wb_lock_wait: atomic_t = atomic_t { _private: [] };
#[no_mangle]
pub static mut netfs_n_folioq: atomic_t = atomic_t { _private: [] };

#[no_mangle]
pub unsafe extern "C" fn netfs_stats_show(m: *mut seq_file, _v: *mut c_void) -> c_int {
    seq_printf(m, c"Reads  : DR=%u RA=%u RF=%u RS=%u WB=%u WBZ=%u\n".as_ptr(),
        atomic_read(&raw const netfs_n_rh_dio_read),
        atomic_read(&raw const netfs_n_rh_readahead),
        atomic_read(&raw const netfs_n_rh_read_folio),
        atomic_read(&raw const netfs_n_rh_read_single),
        atomic_read(&raw const netfs_n_rh_write_begin),
        atomic_read(&raw const netfs_n_rh_write_zskip));
    seq_printf(m, c"Writes : BW=%u WT=%u DW=%u WP=%u 2C=%u\n".as_ptr(),
        atomic_read(&raw const netfs_n_wh_buffered_write), atomic_read(&raw const netfs_n_wh_writethrough),
        atomic_read(&raw const netfs_n_wh_dio_write), atomic_read(&raw const netfs_n_wh_writepages),
        atomic_read(&raw const netfs_n_wh_copy_to_cache));
    seq_printf(m, c"ZeroOps: ZR=%u sh=%u sk=%u\n".as_ptr(), atomic_read(&raw const netfs_n_rh_zero),
        atomic_read(&raw const netfs_n_rh_short_read), atomic_read(&raw const netfs_n_rh_write_zskip));
    seq_printf(m, c"DownOps: DL=%u ds=%u df=%u di=%u\n".as_ptr(), atomic_read(&raw const netfs_n_rh_download),
        atomic_read(&raw const netfs_n_rh_download_done), atomic_read(&raw const netfs_n_rh_download_failed), atomic_read(&raw const netfs_n_rh_download_instead));
    seq_printf(m, c"CaRdOps: RD=%u rs=%u rf=%u\n".as_ptr(), atomic_read(&raw const netfs_n_rh_read), atomic_read(&raw const netfs_n_rh_read_done), atomic_read(&raw const netfs_n_rh_read_failed));
    seq_printf(m, c"UpldOps: UL=%u us=%u uf=%u\n".as_ptr(), atomic_read(&raw const netfs_n_wh_upload), atomic_read(&raw const netfs_n_wh_upload_done), atomic_read(&raw const netfs_n_wh_upload_failed));
    seq_printf(m, c"CaWrOps: WR=%u ws=%u wf=%u\n".as_ptr(), atomic_read(&raw const netfs_n_wh_write), atomic_read(&raw const netfs_n_wh_write_done), atomic_read(&raw const netfs_n_wh_write_failed));
    seq_printf(m, c"Retries: rq=%u rs=%u wq=%u ws=%u\n".as_ptr(), atomic_read(&raw const netfs_n_rh_retry_read_req), atomic_read(&raw const netfs_n_rh_retry_read_subreq), atomic_read(&raw const netfs_n_wh_retry_write_req), atomic_read(&raw const netfs_n_wh_retry_write_subreq));
    seq_printf(m, c"Objs   : rr=%u sr=%u foq=%u wsc=%u\n".as_ptr(), atomic_read(&raw const netfs_n_rh_rreq), atomic_read(&raw const netfs_n_rh_sreq), atomic_read(&raw const netfs_n_folioq), atomic_read(&raw const netfs_n_wh_wstream_conflict));
    seq_printf(m, c"WbLock : skip=%u wait=%u\n".as_ptr(), atomic_read(&raw const netfs_n_wb_lock_skip), atomic_read(&raw const netfs_n_wb_lock_wait));
    fscache_stats_show(m)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
