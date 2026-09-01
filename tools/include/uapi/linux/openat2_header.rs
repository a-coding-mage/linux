/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C source included <linux/types.h> for __u64.
pub type __u64 = u64;

/*
 * Arguments for how openat2(2) should open the target path. If only @flags and
 * @mode are non-zero, then openat2(2) operates very similarly to openat(2).
 *
 * However, unlike openat(2), unknown or invalid bits in @flags result in
 * -EINVAL rather than being silently ignored. @mode must be zero unless one of
 * {O_CREAT, O_TMPFILE} are set.
 *
 * @flags: O_* flags.
 * @mode: O_CREAT/O_TMPFILE file mode.
 * @resolve: RESOLVE_* flags.
 */
#[repr(C)]
pub struct open_how {
    pub flags: __u64,
    pub mode: __u64,
    pub resolve: __u64,
}

/* how->resolve flags for openat2(2). */
pub const RESOLVE_NO_XDEV: u32 = 0x01; /* Block mount-point crossings
                                        * (includes bind-mounts). */
pub const RESOLVE_NO_MAGICLINKS: u32 = 0x02; /* Block traversal through procfs-style
                                              * "magic-links". */
pub const RESOLVE_NO_SYMLINKS: u32 = 0x04; /* Block traversal through all symlinks
                                            * (implies OEXT_NO_MAGICLINKS) */
pub const RESOLVE_BENEATH: u32 = 0x08; /* Block "lexical" trickery like
                                        * "..", symlinks, and absolute
                                        * paths which escape the dirfd. */
pub const RESOLVE_IN_ROOT: u32 = 0x10; /* Make all jumps to "/" and ".."
                                        * be scoped inside the dirfd
                                        * (similar to chroot(2)). */
pub const RESOLVE_CACHED: u32 = 0x20; /* Only complete if resolution can be
                                       * completed through cached lookup. May
                                       * return -EAGAIN if that's not
                                       * possible. */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
