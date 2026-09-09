/*
 * Ftrace support for Microblaze.
 *
 * Copyright (C) 2009 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2009 PetaLogix
 *
 * Based on MIPS and PowerPC ftrace code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 */

/* External declarations supplied by the Microblaze/Linux support code. */
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dyn_ftrace {
    pub ip: usize,
}

pub type ftrace_func_t = unsafe extern "C" fn();

extern "C" {
    static mut ftrace_call: u8;
    static mut ftrace_caller: u8;
    static mut ftrace_call_graph: u8;
    static mut return_to_handler: u8;
    fn flush_dcache_range(start: u32, end: u32);
    fn flush_icache_range(start: u32, end: u32);
    fn ftrace_graph_is_dead() -> i32;
    fn ftrace_graph_stop();
    fn function_graph_enter(old: usize, self_addr: usize, depth: i32, data: *mut core::ffi::c_void) -> i32;
    fn pr_debug(fmt: *const u8, ...);
    fn warn_on(condition: i32) -> i32;
}

#[cfg(feature = "CONFIG_FUNCTION_GRAPH_TRACER")]
pub unsafe fn prepare_ftrace_return(parent: *mut usize, self_addr: usize) {
    let old: usize;
    let faulted: i32;
    let return_hooker = &return_to_handler as *const u8 as usize;

    if ftrace_graph_is_dead() != 0 {
        return;
    }

    /* `current` and tracing_graph_pause are supplied by the kernel. */
    extern "C" {
        static mut current_tracing_graph_pause: i32;
    }
    if current_tracing_graph_pause != 0 {
        return;
    }

    /* The original Microblaze asm also records page faults through __ex_table. */
    old = core::ptr::read_volatile(parent);
    core::ptr::write_volatile(parent, return_hooker);
    faulted = 0;

    flush_dcache_range(parent as u32, parent as u32 + 4);
    flush_icache_range(parent as u32, parent as u32 + 4);

    if faulted != 0 {
        ftrace_graph_stop();
        warn_on(1);
        return;
    }

    if function_graph_enter(old, self_addr, 0, core::ptr::null_mut()) != 0 {
        *parent = old;
    }
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
unsafe fn ftrace_modify_code(addr: usize, value: u32) -> i32 {
    /* The original inline asm performs the store with exception-table recovery. */
    core::ptr::write_volatile(addr as *mut u32, value);
    let faulted = 0;

    if faulted != 0 {
        return -14; // -EFAULT
    }

    flush_dcache_range(addr as u32, addr as u32 + 4);
    flush_icache_range(addr as u32, addr as u32 + 4);
    0
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
const MICROBLAZE_NOP: u32 = 0x80000000;
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
const MICROBLAZE_BRI: u32 = 0xb800000C;

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
static mut recorded: u32 = 0;
#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
static mut imm: u32 = 0;

/* USE_FTRACE_NOP is a build-time alternative retained in the source comments. */
#[cfg(all(feature = "CONFIG_DYNAMIC_FTRACE", feature = "USE_FTRACE_NOP"))]
static mut bralid: u32 = 0;

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
pub unsafe fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, _addr: usize) -> i32 {
    let mut ret = 0;
    if recorded == 0 {
        recorded = 1;
        imm = core::ptr::read_volatile((*rec).ip as *const u32);
        #[cfg(feature = "USE_FTRACE_NOP")]
        {
            bralid = core::ptr::read_volatile(((*rec).ip + 4) as *const u32);
        }
    }

    #[cfg(feature = "USE_FTRACE_NOP")]
    {
        ret = ftrace_modify_code((*rec).ip, MICROBLAZE_NOP);
        ret += ftrace_modify_code((*rec).ip + 4, MICROBLAZE_NOP);
    }
    #[cfg(not(feature = "USE_FTRACE_NOP"))]
    {
        ret = ftrace_modify_code((*rec).ip, MICROBLAZE_BRI);
    }
    ret
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
pub unsafe fn ftrace_make_call(rec: *mut dyn_ftrace, _addr: usize) -> i32 {
    let mut ret = ftrace_modify_code((*rec).ip, imm);
    #[cfg(feature = "USE_FTRACE_NOP")]
    {
        ret += ftrace_modify_code((*rec).ip + 4, bralid);
    }
    ret
}

#[cfg(feature = "CONFIG_DYNAMIC_FTRACE")]
pub unsafe fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let ip = &ftrace_call as *const u8 as usize;
    let mut upper = func as usize as u32;
    let lower = upper;
    upper = 0xb0000000 + (upper >> 16);
    let lower = 0x32800000 + (lower & 0xFFFF);
    let mut ret = ftrace_modify_code(ip, upper);
    ret += ftrace_modify_code(ip + 4, lower);
    ret += ftrace_modify_code(&ftrace_caller as *const u8 as usize, MICROBLAZE_NOP);
    ret
}

#[cfg(all(feature = "CONFIG_DYNAMIC_FTRACE", feature = "CONFIG_FUNCTION_GRAPH_TRACER"))]
static mut old_jump: u32 = 0;

#[cfg(all(feature = "CONFIG_DYNAMIC_FTRACE", feature = "CONFIG_FUNCTION_GRAPH_TRACER"))]
pub unsafe fn ftrace_enable_ftrace_graph_caller() -> i32 {
    let ip = &ftrace_call_graph as *const u8 as usize;
    old_jump = core::ptr::read_volatile(ip as *const u32);
    ftrace_modify_code(ip, MICROBLAZE_NOP)
}

#[cfg(all(feature = "CONFIG_DYNAMIC_FTRACE", feature = "CONFIG_FUNCTION_GRAPH_TRACER"))]
pub unsafe fn ftrace_disable_ftrace_graph_caller() -> i32 {
    let ip = &ftrace_call_graph as *const u8 as usize;
    ftrace_modify_code(ip, old_jump)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
