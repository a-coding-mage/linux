/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

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
    pub flags: u64,
    pub mode: u64,
    pub resolve: u64,
}

/*
 * how->flags bits exclusive to openat2(2). These live in the upper 32 bits
 * of @flags so that they cannot be expressed by open(2) / openat(2), whose
 * @flags argument is a C int.
 */
pub const OPENAT2_REGULAR: u64 = (1u64) << 32; /* Only open regular files. */

/* how->resolve flags for openat2(2). */
pub const RESOLVE_NO_XDEV: u64 = 0x01; /* Block mount-point crossings
                                          (includes bind-mounts). */
pub const RESOLVE_NO_MAGICLINKS: u64 = 0x02; /* Block traversal through procfs-style
                                                "magic-links". */
pub const RESOLVE_NO_SYMLINKS: u64 = 0x04; /* Block traversal through all symlinks
                                              (implies OEXT_NO_MAGICLINKS) */
pub const RESOLVE_BENEATH: u64 = 0x08; /* Block "lexical" trickery like
                                          "..", symlinks, and absolute
                                          paths which escape the dirfd. */
pub const RESOLVE_IN_ROOT: u64 = 0x10; /* Make all jumps to "/" and ".."
                                         be scoped inside the dirfd
                                         (similar to chroot(2)). */
pub const RESOLVE_CACHED: u64 = 0x20; /* Only complete if resolution can be
                                        completed through cached lookup. May
                                        return -EAGAIN if that's not
                                        possible. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
