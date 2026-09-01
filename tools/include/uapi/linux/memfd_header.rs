/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on <asm-generic/hugetlb_encode.h> for HUGETLB_FLAG_ENCODE_* values. */

/* flags for memfd_create(2) (unsigned int) */
pub const MFD_CLOEXEC: u32 = 0x0001u32;
pub const MFD_ALLOW_SEALING: u32 = 0x0002u32;
pub const MFD_HUGETLB: u32 = 0x0004u32;
/* not executable and sealed to prevent changing to executable. */
pub const MFD_NOEXEC_SEAL: u32 = 0x0008u32;
/* executable */
pub const MFD_EXEC: u32 = 0x0010u32;

/*
 * Huge page size encoding when MFD_HUGETLB is specified, and a huge page
 * size other than the default is desired.  See hugetlb_encode.h.
 * All known huge page size encodings are provided here.  It is the
 * responsibility of the application to know which sizes are supported on
 * the running system.  See mmap(2) man page for details.
 */
pub const MFD_HUGE_SHIFT: u32 = HUGETLB_FLAG_ENCODE_SHIFT;
pub const MFD_HUGE_MASK: u32 = HUGETLB_FLAG_ENCODE_MASK;

pub const MFD_HUGE_64KB: u32 = HUGETLB_FLAG_ENCODE_64KB;
pub const MFD_HUGE_512KB: u32 = HUGETLB_FLAG_ENCODE_512KB;
pub const MFD_HUGE_1MB: u32 = HUGETLB_FLAG_ENCODE_1MB;
pub const MFD_HUGE_2MB: u32 = HUGETLB_FLAG_ENCODE_2MB;
pub const MFD_HUGE_8MB: u32 = HUGETLB_FLAG_ENCODE_8MB;
pub const MFD_HUGE_16MB: u32 = HUGETLB_FLAG_ENCODE_16MB;
pub const MFD_HUGE_32MB: u32 = HUGETLB_FLAG_ENCODE_32MB;
pub const MFD_HUGE_256MB: u32 = HUGETLB_FLAG_ENCODE_256MB;
pub const MFD_HUGE_512MB: u32 = HUGETLB_FLAG_ENCODE_512MB;
pub const MFD_HUGE_1GB: u32 = HUGETLB_FLAG_ENCODE_1GB;
pub const MFD_HUGE_2GB: u32 = HUGETLB_FLAG_ENCODE_2GB;
pub const MFD_HUGE_16GB: u32 = HUGETLB_FLAG_ENCODE_16GB;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
