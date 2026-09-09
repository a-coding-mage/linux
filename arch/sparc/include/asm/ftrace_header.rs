/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_MCOUNT */
#[cfg(CONFIG_MCOUNT)]
pub const MCOUNT_INSN_SIZE: usize = 4; /* sizeof mcount call */

#[cfg(CONFIG_MCOUNT)]
unsafe extern "C" {
    pub fn _mcount();
}

/* MCOUNT_ADDR is the address of _mcount when CONFIG_MCOUNT is enabled. */
#[cfg(CONFIG_MCOUNT)]
#[inline]
pub unsafe fn mcount_addr() -> usize {
    _mcount as usize
}

/*
 * CONFIG_SPARC64 && !CC_USE_FENTRY
 *
 * The C header defines HAVE_FUNCTION_GRAPH_FP_TEST under this build-time
 * condition. Preserve the condition and marker for downstream configuration.
 */
#[cfg(all(CONFIG_SPARC64, not(CC_USE_FENTRY)))]
pub const HAVE_FUNCTION_GRAPH_FP_TEST: bool = true;

/* CONFIG_DYNAMIC_FTRACE */
#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[inline]
pub const fn ftrace_call_adjust(addr: usize) -> usize {
    addr
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[repr(C)]
pub struct dyn_arch_ftrace {}

unsafe extern "C" {
    pub fn prepare_ftrace_return(
        parent: usize,
        self_addr: usize,
        frame_pointer: usize,
    ) -> usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
