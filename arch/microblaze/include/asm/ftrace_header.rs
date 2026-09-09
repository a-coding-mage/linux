/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _ASM_MICROBLAZE_FTRACE

#[cfg(CONFIG_FUNCTION_TRACER)]
pub const MCOUNT_INSN_SIZE: usize = 8; /* sizeof mcount call */

#[cfg(CONFIG_FUNCTION_TRACER)]
extern "C" {
    pub fn _mcount();
    pub fn ftrace_call_graph();
    pub fn prepare_ftrace_return(parent: *mut usize, self_addr: usize);
}

#[cfg(CONFIG_FUNCTION_TRACER)]
#[macro_export]
macro_rules! MCOUNT_ADDR {
    () => {
        $crate::_mcount as usize
    };
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
/* relocation of mcount call site is the same as the address */
#[inline]
pub const unsafe fn ftrace_call_adjust(addr: usize) -> usize {
    addr
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[repr(C)]
pub struct dyn_arch_ftrace {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
