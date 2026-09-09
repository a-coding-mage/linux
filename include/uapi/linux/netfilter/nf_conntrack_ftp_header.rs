/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* FTP tracking. */

/* This enum is exposed to userspace */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum nf_ct_ftp_type {
    /* PORT command from client */
    NF_CT_FTP_PORT = 0,
    /* PASV response from server */
    NF_CT_FTP_PASV = 1,
    /* EPRT command from client */
    NF_CT_FTP_EPRT = 2,
    /* EPSV response from server */
    NF_CT_FTP_EPSV = 3,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
