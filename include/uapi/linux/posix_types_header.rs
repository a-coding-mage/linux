/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header guard: _LINUX_POSIX_TYPES_H

// Dependency: <linux/stddef.h>

/*
 * This allows for 1024 file descriptors: if NR_OPEN is ever grown
 * beyond that you'll have to change this too. But 1024 fd's seem to be
 * enough even for such "real" unices like OSF/1, so hopefully this is
 * one limit that doesn't have to be changed [again].
 *
 * Note that POSIX wants the FD_CLEAR(fd,fdsetp) defines to be in
 * <sys/time.h> (and thus <linux/time.h>) - but this is a more logical
 * place for them. Solved by having dummy defines in <sys/time.h>.
 */

/*
 * This macro may have been defined in <gnu/types.h>. But we always
 * use the one here.
 */
pub const __FD_SETSIZE: usize = 1024;

#[repr(C)]
pub struct __kernel_fd_set {
    pub fds_bits: [core::ffi::c_ulong; __FD_SETSIZE / (8 * core::mem::size_of::<core::ffi::c_ulong>())],
}

/* Type of a signal handler.  */
pub type __kernel_sighandler_t = Option<unsafe extern "C" fn(core::ffi::c_int)>;

/* Type of a SYSV IPC key.  */
pub type __kernel_key_t = core::ffi::c_int;
pub type __kernel_mqd_t = core::ffi::c_int;

// Dependency: <asm/posix_types.h>

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
