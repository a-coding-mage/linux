/* SPDX-License-Identifier: GPL-2.0 */

// Corresponds to the declarations supplied by <asm-generic/audit_dir_write.h>.
// The C preprocessor conditions below are preserved as comments; this file
// intentionally relies on the corresponding syscall constants being supplied
// by the surrounding translation unit.
pub const AUDIT_WRITE_SYSCALLS: &[i32] = &[
    __NR_acct,
    // #ifdef __NR_swapon
    __NR_swapon,
    // #endif
    __NR_quotactl,
    // #ifdef __NR_quotactl_fd
    __NR_quotactl_fd,
    // #endif
    // #ifdef __NR_truncate
    __NR_truncate,
    // #endif
    // #ifdef __NR_truncate64
    __NR_truncate64,
    // #endif
    // #ifdef __NR_ftruncate
    __NR_ftruncate,
    // #endif
    // #ifdef __NR_ftruncate64
    __NR_ftruncate64,
    // #endif
    // #ifdef __NR_bind
    __NR_bind, /* bind can affect fs object only in one way... */
    // #endif
    // #ifdef __NR_fallocate
    __NR_fallocate,
    // #endif
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
