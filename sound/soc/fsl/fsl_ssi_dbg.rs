// SPDX-License-Identifier: GPL-2.0
//
// Freescale SSI ALSA SoC Digital Audio Interface (DAI) debugging functions
//
// Copyright 2014 Markus Pargmann <mpa@pengutronix.de>, Pengutronix
//
// Split from fsl_ssi.c

use core::ffi::{c_char, c_int, c_uint, c_void};

// Dependencies from <linux/debugfs.h>, <linux/device.h>, <linux/kernel.h>,
// and "fsl_ssi.h" are declared here and expected to be supplied externally.

pub type u32 = c_uint;

#[repr(C)]
pub struct seq_file {
    pub private: *mut c_void,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file_operations {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fsl_ssi_stats {
    pub rfrc: c_uint,
    pub tfrc: c_uint,
    pub cmdau: c_uint,
    pub cmddu: c_uint,
    pub rxt: c_uint,
    pub rdr1: c_uint,
    pub rdr0: c_uint,
    pub tde1: c_uint,
    pub tde0: c_uint,
    pub roe1: c_uint,
    pub roe0: c_uint,
    pub tue1: c_uint,
    pub tue0: c_uint,
    pub tfs: c_uint,
    pub rfs: c_uint,
    pub tls: c_uint,
    pub rls: c_uint,
    pub rff1: c_uint,
    pub rff0: c_uint,
    pub tfe1: c_uint,
    pub tfe0: c_uint,
}

#[repr(C)]
pub struct fsl_ssi_dbg {
    pub stats: fsl_ssi_stats,
    pub dbg_dir: *mut dentry,
}

extern "C" {
    static SSI_SISR_RFRC: u32;
    static SSI_SISR_TFRC: u32;
    static SSI_SISR_CMDAU: u32;
    static SSI_SISR_CMDDU: u32;
    static SSI_SISR_RXT: u32;
    static SSI_SISR_RDR1: u32;
    static SSI_SISR_RDR0: u32;
    static SSI_SISR_TDE1: u32;
    static SSI_SISR_TDE0: u32;
    static SSI_SISR_ROE1: u32;
    static SSI_SISR_ROE0: u32;
    static SSI_SISR_TUE1: u32;
    static SSI_SISR_TUE0: u32;
    static SSI_SISR_TFS: u32;
    static SSI_SISR_RFS: u32;
    static SSI_SISR_TLS: u32;
    static SSI_SISR_RLS: u32;
    static SSI_SISR_RFF1: u32;
    static SSI_SISR_RFF0: u32;
    static SSI_SISR_TFE1: u32;
    static SSI_SISR_TFE0: u32;

    static SSI_SIER_RFRC_EN: u32;
    static SSI_SIER_TFRC_EN: u32;
    static SSI_SIER_CMDAU_EN: u32;
    static SSI_SIER_CMDDU_EN: u32;
    static SSI_SIER_RXT_EN: u32;
    static SSI_SIER_RDR1_EN: u32;
    static SSI_SIER_RDR0_EN: u32;
    static SSI_SIER_TDE1_EN: u32;
    static SSI_SIER_TDE0_EN: u32;
    static SSI_SIER_ROE1_EN: u32;
    static SSI_SIER_ROE0_EN: u32;
    static SSI_SIER_TUE1_EN: u32;
    static SSI_SIER_TUE0_EN: u32;
    static SSI_SIER_TFS_EN: u32;
    static SSI_SIER_RFS_EN: u32;
    static SSI_SIER_TLS_EN: u32;
    static SSI_SIER_RLS_EN: u32;
    static SSI_SIER_RFF1_EN: u32;
    static SSI_SIER_RFF0_EN: u32;
    static SSI_SIER_TFE1_EN: u32;
    static SSI_SIER_TFE0_EN: u32;

    static fsl_ssi_stats_fops: file_operations;

    fn seq_printf(s: *mut seq_file, fmt: *const c_char, ...) -> c_int;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn dev_name(dev: *const device) -> *const c_char;
}

#[no_mangle]
pub unsafe extern "C" fn fsl_ssi_dbg_isr(dbg: *mut fsl_ssi_dbg, sisr: u32) {
    if sisr & SSI_SISR_RFRC != 0 {
        (*dbg).stats.rfrc = (*dbg).stats.rfrc.wrapping_add(1);
    }

    if sisr & SSI_SISR_TFRC != 0 {
        (*dbg).stats.tfrc = (*dbg).stats.tfrc.wrapping_add(1);
    }

    if sisr & SSI_SISR_CMDAU != 0 {
        (*dbg).stats.cmdau = (*dbg).stats.cmdau.wrapping_add(1);
    }

    if sisr & SSI_SISR_CMDDU != 0 {
        (*dbg).stats.cmddu = (*dbg).stats.cmddu.wrapping_add(1);
    }

    if sisr & SSI_SISR_RXT != 0 {
        (*dbg).stats.rxt = (*dbg).stats.rxt.wrapping_add(1);
    }

    if sisr & SSI_SISR_RDR1 != 0 {
        (*dbg).stats.rdr1 = (*dbg).stats.rdr1.wrapping_add(1);
    }

    if sisr & SSI_SISR_RDR0 != 0 {
        (*dbg).stats.rdr0 = (*dbg).stats.rdr0.wrapping_add(1);
    }

    if sisr & SSI_SISR_TDE1 != 0 {
        (*dbg).stats.tde1 = (*dbg).stats.tde1.wrapping_add(1);
    }

    if sisr & SSI_SISR_TDE0 != 0 {
        (*dbg).stats.tde0 = (*dbg).stats.tde0.wrapping_add(1);
    }

    if sisr & SSI_SISR_ROE1 != 0 {
        (*dbg).stats.roe1 = (*dbg).stats.roe1.wrapping_add(1);
    }

    if sisr & SSI_SISR_ROE0 != 0 {
        (*dbg).stats.roe0 = (*dbg).stats.roe0.wrapping_add(1);
    }

    if sisr & SSI_SISR_TUE1 != 0 {
        (*dbg).stats.tue1 = (*dbg).stats.tue1.wrapping_add(1);
    }

    if sisr & SSI_SISR_TUE0 != 0 {
        (*dbg).stats.tue0 = (*dbg).stats.tue0.wrapping_add(1);
    }

    if sisr & SSI_SISR_TFS != 0 {
        (*dbg).stats.tfs = (*dbg).stats.tfs.wrapping_add(1);
    }

    if sisr & SSI_SISR_RFS != 0 {
        (*dbg).stats.rfs = (*dbg).stats.rfs.wrapping_add(1);
    }

    if sisr & SSI_SISR_TLS != 0 {
        (*dbg).stats.tls = (*dbg).stats.tls.wrapping_add(1);
    }

    if sisr & SSI_SISR_RLS != 0 {
        (*dbg).stats.rls = (*dbg).stats.rls.wrapping_add(1);
    }

    if sisr & SSI_SISR_RFF1 != 0 {
        (*dbg).stats.rff1 = (*dbg).stats.rff1.wrapping_add(1);
    }

    if sisr & SSI_SISR_RFF0 != 0 {
        (*dbg).stats.rff0 = (*dbg).stats.rff0.wrapping_add(1);
    }

    if sisr & SSI_SISR_TFE1 != 0 {
        (*dbg).stats.tfe1 = (*dbg).stats.tfe1.wrapping_add(1);
    }

    if sisr & SSI_SISR_TFE0 != 0 {
        (*dbg).stats.tfe0 = (*dbg).stats.tfe0.wrapping_add(1);
    }
}

/*
 * Show the statistics of a flag only if its interrupt is enabled
 *
 * Compilers will optimize it to a no-op if the interrupt is disabled
 */
macro_rules! SIER_SHOW {
    ($s:expr, $ssi_dbg:expr, $flag:ident, $name:ident, $fmt:expr) => {
        if $flag != 0 {
            seq_printf($s, $fmt.as_ptr() as *const c_char, (*$ssi_dbg).stats.$name);
        }
    };
}

/*
 * Display the statistics for the current SSI device
 *
 * To avoid confusion, only show those counts that are enabled
 */
unsafe extern "C" fn fsl_ssi_stats_show(s: *mut seq_file, _unused: *mut c_void) -> c_int {
    let ssi_dbg = (*s).private as *mut fsl_ssi_dbg;

    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RFRC_EN, rfrc, b"rfrc=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TFRC_EN, tfrc, b"tfrc=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_CMDAU_EN, cmdau, b"cmdau=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_CMDDU_EN, cmddu, b"cmddu=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RXT_EN, rxt, b"rxt=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RDR1_EN, rdr1, b"rdr1=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RDR0_EN, rdr0, b"rdr0=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TDE1_EN, tde1, b"tde1=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TDE0_EN, tde0, b"tde0=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_ROE1_EN, roe1, b"roe1=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_ROE0_EN, roe0, b"roe0=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TUE1_EN, tue1, b"tue1=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TUE0_EN, tue0, b"tue0=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TFS_EN, tfs, b"tfs=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RFS_EN, rfs, b"rfs=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TLS_EN, tls, b"tls=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RLS_EN, rls, b"rls=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RFF1_EN, rff1, b"rff1=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_RFF0_EN, rff0, b"rff0=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TFE1_EN, tfe1, b"tfe1=%u\n\0");
    SIER_SHOW!(s, ssi_dbg, SSI_SIER_TFE0_EN, tfe0, b"tfe0=%u\n\0");

    0
}

// DEFINE_SHOW_ATTRIBUTE(fsl_ssi_stats);

#[no_mangle]
pub unsafe extern "C" fn fsl_ssi_debugfs_create(ssi_dbg: *mut fsl_ssi_dbg, dev: *mut device) {
    (*ssi_dbg).dbg_dir = debugfs_create_dir(dev_name(dev), core::ptr::null_mut());

    debugfs_create_file(
        b"stats\0".as_ptr() as *const c_char,
        0o444,
        (*ssi_dbg).dbg_dir,
        ssi_dbg as *mut c_void,
        &fsl_ssi_stats_fops,
    );
}

#[no_mangle]
pub unsafe extern "C" fn fsl_ssi_debugfs_remove(ssi_dbg: *mut fsl_ssi_dbg) {
    debugfs_remove_recursive((*ssi_dbg).dbg_dir);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
