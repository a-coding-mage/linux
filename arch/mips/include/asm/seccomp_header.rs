/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/unistd.h>.

#[cfg(CONFIG_COMPAT)]
pub unsafe fn get_compat_mode1_syscalls() -> *const core::ffi::c_int {
    static SYSCALLS_O32: [core::ffi::c_int; 5] = [
        __NR_O32_Linux + 3,
        __NR_O32_Linux + 4,
        __NR_O32_Linux + 1,
        __NR_O32_Linux + 193,
        -1, // negative terminated
    ];
    static SYSCALLS_N32: [core::ffi::c_int; 5] = [
        __NR_N32_Linux + 0,
        __NR_N32_Linux + 1,
        __NR_N32_Linux + 58,
        __NR_N32_Linux + 211,
        -1, // negative terminated
    ];

    // CONFIG_MIPS32_O32 is a build-time condition; preserve its intent here.
    if IS_ENABLED(CONFIG_MIPS32_O32) && test_thread_flag(TIF_32BIT_REGS) {
        return SYSCALLS_O32.as_ptr();
    }

    // CONFIG_MIPS32_N32 is a build-time condition; preserve its intent here.
    if IS_ENABLED(CONFIG_MIPS32_N32) {
        return SYSCALLS_N32.as_ptr();
    }

    BUG();
}

// #define get_compat_mode1_syscalls get_compat_mode1_syscalls

// Declaration supplied by <asm-generic/seccomp.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
