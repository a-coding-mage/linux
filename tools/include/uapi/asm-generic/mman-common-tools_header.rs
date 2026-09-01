/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on asm-generic/mman-common.h. */

/* We need this because we need to have tools/include/uapi/ included in the tools
 * header search path to get access to stuff that is not yet in the system's
 * copy of the files in that directory, but since this cset:
 *
 *     746c9398f5ac ("arch: move common mmap flags to linux/mman.h")
 *
 * We end up making sys/mman.h, that is in the system headers, to not find the
 * MAP_SHARED and MAP_PRIVATE defines because they are not anymore in our copy
 * of asm-generic/mman-common.h. So we define them here and include this header
 * from each of the per arch mman.h headers.
 */
/* C preprocessor condition preserved: define these only if MAP_SHARED is not
 * already defined by the including environment.
 */
pub const MAP_SHARED: u32 = 0x01; /* Share changes */
pub const MAP_PRIVATE: u32 = 0x02; /* Changes are private */
pub const MAP_SHARED_VALIDATE: u32 = 0x03; /* share + validate extension flags */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
