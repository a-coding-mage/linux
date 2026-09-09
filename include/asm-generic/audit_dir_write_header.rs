/* SPDX-License-Identifier: GPL-2.0 */

// The following entries are conditionally present when the corresponding
// syscall-number macro is defined by the target architecture.

#[cfg(__NR_rename)]
__NR_rename,

#[cfg(__NR_mkdir)]
__NR_mkdir,

#[cfg(__NR_rmdir)]
__NR_rmdir,

#[cfg(__NR_creat)]
__NR_creat,

#[cfg(__NR_link)]
__NR_link,

#[cfg(__NR_unlink)]
__NR_unlink,

#[cfg(__NR_symlink)]
__NR_symlink,

#[cfg(__NR_mknod)]
__NR_mknod,

#[cfg(__NR_mkdirat)]
__NR_mkdirat,
__NR_mknodat,
__NR_unlinkat,
// Conditional on __NR_renameat in the original header.
#[cfg(__NR_renameat)]
__NR_renameat,
__NR_linkat,
__NR_symlinkat,

#[cfg(__NR_renameat2)]
__NR_renameat2,

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
