/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* sonet.h - SONET/SHD physical layer control */

/* Written 1995-2000 by Werner Almesberger, EPFL LRC/ICA */

/* The ioctl encoding macros and ATMIOC_PHYTYP are supplied by the
 * corresponding UAPI dependencies. */

#[repr(C, packed)]
pub struct sonet_stats {
    /* section parity errors (B1) */
    pub section_bip: ::kernel::ffi::c_int,
    /* line parity errors (B2) */
    pub line_bip: ::kernel::ffi::c_int,
    /* path parity errors (B3) */
    pub path_bip: ::kernel::ffi::c_int,
    /* line parity errors at remote */
    pub line_febe: ::kernel::ffi::c_int,
    /* path parity errors at remote */
    pub path_febe: ::kernel::ffi::c_int,
    /* correctable header errors */
    pub corr_hcs: ::kernel::ffi::c_int,
    /* uncorrectable header errors */
    pub uncorr_hcs: ::kernel::ffi::c_int,
    /* cells sent */
    pub tx_cells: ::kernel::ffi::c_int,
    /* cells received */
    pub rx_cells: ::kernel::ffi::c_int,
}

/* get statistics */
pub const SONET_GETSTAT: ::kernel::ffi::c_ulong =
    _IOR!('a', ATMIOC_PHYTYP, sonet_stats);
/* ... and zero counters */
pub const SONET_GETSTATZ: ::kernel::ffi::c_ulong =
    _IOR!('a', ATMIOC_PHYTYP + 1, sonet_stats);
/* set error insertion */
pub const SONET_SETDIAG: ::kernel::ffi::c_ulong =
    _IOWR!('a', ATMIOC_PHYTYP + 2, ::kernel::ffi::c_int);
/* clear error insertion */
pub const SONET_CLRDIAG: ::kernel::ffi::c_ulong =
    _IOWR!('a', ATMIOC_PHYTYP + 3, ::kernel::ffi::c_int);
/* query error insertion */
pub const SONET_GETDIAG: ::kernel::ffi::c_ulong =
    _IOR!('a', ATMIOC_PHYTYP + 4, ::kernel::ffi::c_int);
/* set framing mode (SONET/SDH) */
pub const SONET_SETFRAMING: ::kernel::ffi::c_ulong =
    _IOW!('a', ATMIOC_PHYTYP + 5, ::kernel::ffi::c_int);
/* get framing mode */
pub const SONET_GETFRAMING: ::kernel::ffi::c_ulong =
    _IOR!('a', ATMIOC_PHYTYP + 6, ::kernel::ffi::c_int);
/* get framing sense information */
pub const SONET_GETFRSENSE: ::kernel::ffi::c_ulong =
    _IOR!('a', ATMIOC_PHYTYP + 7, [u8; SONET_FRSENSE_SIZE]);

/* section BIP */
pub const SONET_INS_SBIP: ::kernel::ffi::c_int = 1;
/* line BIP */
pub const SONET_INS_LBIP: ::kernel::ffi::c_int = 2;
/* path BIP */
pub const SONET_INS_PBIP: ::kernel::ffi::c_int = 4;
/* out of frame */
pub const SONET_INS_FRAME: ::kernel::ffi::c_int = 8;
/* set line to zero */
pub const SONET_INS_LOS: ::kernel::ffi::c_int = 16;
/* line alarm indication signal */
pub const SONET_INS_LAIS: ::kernel::ffi::c_int = 32;
/* path alarm indication signal */
pub const SONET_INS_PAIS: ::kernel::ffi::c_int = 64;
/* insert HCS error */
pub const SONET_INS_HCS: ::kernel::ffi::c_int = 128;

/* SONET STS-3 framing */
pub const SONET_FRAME_SONET: ::kernel::ffi::c_int = 0;
/* SDH STM-1 framing */
pub const SONET_FRAME_SDH: ::kernel::ffi::c_int = 1;

/* C1[3],H1[3] (0xff for unknown) */
pub const SONET_FRSENSE_SIZE: usize = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
