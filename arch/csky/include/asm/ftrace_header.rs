/* SPDX-License-Identifier: GPL-2.0 */

pub const MCOUNT_INSN_SIZE: usize = 14;

// HAVE_FUNCTION_GRAPH_FP_TEST

pub const ARCH_SUPPORTS_FTRACE_OPS: usize = 1;

// Equivalent of: ((unsigned long)_mcount)
pub const MCOUNT_ADDR: usize = _mcount as usize;

unsafe extern "C" {
    pub fn _mcount(arg: usize);

    pub fn ftrace_graph_call();

    pub fn prepare_ftrace_return(
        parent: *mut usize,
        self_addr: usize,
        frame_pointer: usize,
    );
}

#[inline]
pub fn ftrace_call_adjust(addr: usize) -> usize {
    addr
}

#[repr(C)]
pub struct dyn_arch_ftrace {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
