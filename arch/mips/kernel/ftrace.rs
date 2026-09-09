// SPDX-License-Identifier: GPL-2.0
/*
 * Code for replacing ftrace calls with jumps.
 *
 * Copyright (C) 2007-2008 Steven Rostedt <srostedt@redhat.com>
 * Copyright (C) 2009, 2010 DSLab, Lanzhou University, China
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 *
 * Thanks goes to Steven Rostedt for writing the original x86 version.
 */

// Kernel and MIPS architecture dependencies are supplied by other translation units.

#[cfg(all(feature = "kbuild_mcount_ra_address", feature = "config_32bit"))]
const MCOUNT_OFFSET_INSNS: u32 = 5;
#[cfg(not(all(feature = "kbuild_mcount_ra_address", feature = "config_32bit")))]
const MCOUNT_OFFSET_INSNS: u32 = 4;

#[cfg(feature = "config_dynamic_ftrace")]
mod dynamic_ftrace {
    use super::*;

    extern "C" {
        static mut MCOUNT_ADDR: usize;
        static mut FTRACE_ADDR: usize;
        static mut ftrace_call: usize;
        fn ftrace_modify_all_code(command: i32);
        fn uasm_in_compat_space_p(address: usize) -> bool;
        fn UASM_i_LA(buf: *mut *mut u32, reg: u32, address: usize);
        fn uasm_i_jal(buf: *mut *mut u32, address: usize);
        fn uasm_i_j(buf: *mut *mut u32, address: usize);
        fn ftrace_graph_caller();
        fn flush_icache_range(start: usize, end: usize);
        fn core_kernel_text(address: usize) -> bool;
        fn pr_warn(fmt: *const core::ffi::c_char, ...);
        fn safe_store_code(code: u32, address: usize, faulted: *mut i32);
    }

    #[repr(C)]
    pub struct module {
        _private: [u8; 0],
    }
    #[repr(C)]
    pub struct dyn_ftrace {
        pub ip: usize,
    }
    pub type ftrace_func_t = unsafe extern "C" fn();

    pub unsafe extern "C" fn arch_ftrace_update_code(command: i32) {
        ftrace_modify_all_code(command);
    }

    const JAL: u32 = 0x0c000000;
    const ADDR_MASK: u32 = 0x03ffffff;
    const JUMP_RANGE_MASK: usize = (1usize << 28) - 1;
    const INSN_NOP: u32 = 0x00000000;

    #[inline]
    fn insn_jal(addr: usize) -> u32 {
        JAL | (((addr >> 2) as u32) & ADDR_MASK)
    }

    static mut insn_jal_ftrace_caller: u32 = 0;
    static mut insn_la_mcount: [u32; 2] = [0; 2];
    #[cfg(feature = "config_function_graph_tracer")]
    static mut insn_j_ftrace_graph_caller: u32 = 0;

    unsafe fn ftrace_dyn_arch_init_insns() {
        let mut buf: *mut u32;
        let mut v1: u32;

        /* If we are not in compat space, the number of generated
         * instructions will exceed the maximum expected limit of 2.
         * To prevent buffer overflow, we avoid generating them.
         * insn_la_mcount will not be used later in ftrace_make_call.
         */
        if uasm_in_compat_space_p(MCOUNT_ADDR) {
            /* la v1, _mcount */
            v1 = 3;
            buf = core::ptr::addr_of_mut!(insn_la_mcount[0]);
            UASM_i_LA(&mut buf, v1, MCOUNT_ADDR);
        } else {
            pr_warn(b"ftrace: mcount address beyond 32 bits is not supported (%lX)\n\0".as_ptr() as *const _, MCOUNT_ADDR);
        }

        /* jal (ftrace_caller + 8), jump over the first two instruction */
        buf = core::ptr::addr_of_mut!(insn_jal_ftrace_caller);
        uasm_i_jal(&mut buf, (FTRACE_ADDR + 8) & JUMP_RANGE_MASK);

        #[cfg(feature = "config_function_graph_tracer")]
        {
            /* j ftrace_graph_caller */
            buf = core::ptr::addr_of_mut!(insn_j_ftrace_graph_caller);
            uasm_i_j(&mut buf, ftrace_graph_caller as usize & JUMP_RANGE_MASK);
        }
    }

    unsafe fn ftrace_modify_code(ip: usize, new_code: u32) -> i32 {
        let mut faulted = 0;
        safe_store_code(new_code, ip, &mut faulted);
        if faulted != 0 {
            return -14;
        }
        flush_icache_range(ip, ip + 8);
        0
    }

    #[cfg(not(feature = "config_64bit"))]
    unsafe fn ftrace_modify_code_2(ip: usize, new_code1: u32, new_code2: u32) -> i32 {
        let mut faulted = 0;
        safe_store_code(new_code1, ip, &mut faulted);
        if faulted != 0 { return -14; }
        safe_store_code(new_code2, ip + 4, &mut faulted);
        if faulted != 0 { return -14; }
        flush_icache_range(ip, ip + 8);
        0
    }

    #[cfg(not(feature = "config_64bit"))]
    unsafe fn ftrace_modify_code_2r(mut ip: usize, new_code1: u32, new_code2: u32) -> i32 {
        let mut faulted = 0;
        ip += 4;
        safe_store_code(new_code2, ip, &mut faulted);
        if faulted != 0 { return -14; }
        ip -= 4;
        safe_store_code(new_code1, ip, &mut faulted);
        if faulted != 0 { return -14; }
        flush_icache_range(ip, ip + 8);
        0
    }

    const INSN_B_1F: u32 = 0x10000000 | MCOUNT_OFFSET_INSNS;

    pub unsafe extern "C" fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, _addr: usize) -> i32 {
        let ip = (*rec).ip;
        let new = if core_kernel_text(ip) { INSN_NOP } else { INSN_B_1F };
        #[cfg(feature = "config_64bit")]
        { ftrace_modify_code(ip, new) }
        #[cfg(not(feature = "config_64bit"))]
        { ftrace_modify_code_2(ip, new, INSN_NOP) }
    }

    pub unsafe extern "C" fn ftrace_make_call(rec: *mut dyn_ftrace, _addr: usize) -> i32 {
        let ip = (*rec).ip;
        if !core_kernel_text(ip) && !uasm_in_compat_space_p(MCOUNT_ADDR) { return -14; }
        let new = if core_kernel_text(ip) { insn_jal_ftrace_caller } else { insn_la_mcount[0] };
        #[cfg(feature = "config_64bit")]
        { ftrace_modify_code(ip, new) }
        #[cfg(not(feature = "config_64bit"))]
        { ftrace_modify_code_2r(ip, new, if core_kernel_text(ip) { INSN_NOP } else { insn_la_mcount[1] }) }
    }

    pub unsafe extern "C" fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
        ftrace_modify_code((&raw mut ftrace_call) as *mut usize as usize, insn_jal(func as usize))
    }

    pub unsafe extern "C" fn ftrace_dyn_arch_init() -> i32 {
        ftrace_dyn_arch_init_insns();
        ftrace_modify_code(MCOUNT_ADDR, INSN_NOP);
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
