// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Translated from the C implementation. Kernel headers and configuration
// supplied symbols are intentionally left as external dependencies.

#[cfg(CONFIG_DYNAMIC_FTRACE)]
const NOP: u16 = 0x4000;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const NOP32_HI: u16 = 0xc400;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const NOP32_LO: u16 = 0x4820;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const PUSH_LR: u16 = 0x14d0;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const MOVIH_LINK: u16 = 0xea3a;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const ORI_LINK: u16 = 0xef5a;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const JSR_LINK: u16 = 0xe8fa;
#[cfg(CONFIG_DYNAMIC_FTRACE)]
const BSR_LINK: u16 = 0xe000;

#[repr(C)]
pub struct dyn_ftrace { pub ip: usize }
#[repr(C)]
pub struct module { _private: [u8; 0] }
pub type ftrace_func_t = unsafe extern "C" fn();

#[cfg(CONFIG_DYNAMIC_FTRACE)]
static mut nops: [u16; 7] = [NOP, NOP32_HI, NOP32_LO, NOP32_HI, NOP32_LO, NOP32_HI, NOP32_LO];

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn make_jbsr(callee: usize, pc: usize, call: *mut u16, nolr: bool) {
    let mut offset: isize;
    *call.add(0) = if nolr { NOP } else { PUSH_LR };
    offset = callee as isize - pc as isize;
    if offset < -67108864 || offset > 67108864 {
        *call.add(1) = MOVIH_LINK;
        *call.add(2) = (callee >> 16) as u16;
        *call.add(3) = ORI_LINK;
        *call.add(4) = (callee & 0xffff) as u16;
        *call.add(5) = JSR_LINK;
        *call.add(6) = 0;
    } else {
        offset >>= 1;
        *call.add(1) = BSR_LINK | ((((offset as usize) >> 16) as u16) & 0x3ff);
        *call.add(2) = (offset as usize & 0xffff) as u16;
        *call.add(3) = NOP32_HI;
        *call.add(5) = NOP32_HI;
        *call.add(4) = NOP32_LO;
        *call.add(6) = NOP32_LO;
    }
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_check_current_nop(hook: usize) -> i32 {
    let mut olds = [0u16; 7];
    let hook_pos = hook - 2;
    if copy_from_kernel_nofault(olds.as_mut_ptr() as *mut _, hook_pos as *const _, core::mem::size_of_val(&nops)) != 0 { return -14; }
    if core::slice::from_raw_parts(nops.as_ptr(), 7) != olds {
        return -22;
    }
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
unsafe fn ftrace_modify_code(hook: usize, target: usize, enable: bool, nolr: bool) -> i32 {
    let mut call = [0u16; 7];
    let hook_pos = hook - 2;
    make_jbsr(target, hook, call.as_mut_ptr(), nolr);
    let src = if enable { call.as_ptr() } else { nops.as_ptr() };
    if copy_to_kernel_nofault(hook_pos as *mut _, src as *const _, core::mem::size_of_val(&nops)) != 0 { return -1; }
    flush_icache_range(hook_pos, hook_pos + MCOUNT_INSN_SIZE as usize);
    0
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[no_mangle]
pub unsafe extern "C" fn ftrace_make_call(rec: *mut dyn_ftrace, addr: usize) -> i32 {
    let ret = ftrace_check_current_nop((*rec).ip);
    if ret != 0 { return ret; }
    ftrace_modify_code((*rec).ip, addr, true, false)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[no_mangle]
pub unsafe extern "C" fn ftrace_make_nop(_mod: *mut module, rec: *mut dyn_ftrace, addr: usize) -> i32 {
    ftrace_modify_code((*rec).ip, addr, false, false)
}

#[cfg(CONFIG_DYNAMIC_FTRACE)]
#[no_mangle]
pub unsafe extern "C" fn ftrace_update_ftrace_func(func: ftrace_func_t) -> i32 {
    let mut ret = ftrace_modify_code(&ftrace_call as *const _ as usize, func as usize, true, true);
    if ret == 0 { ret = ftrace_modify_code(&ftrace_regs_call as *const _ as usize, func as usize, true, true); }
    ret
}

#[cfg(CONFIG_DYNAMIC_FTRACE_WITH_REGS)]
pub unsafe extern "C" fn ftrace_modify_call(rec: *mut dyn_ftrace, _old_addr: usize, addr: usize) -> i32 {
    ftrace_modify_code((*rec).ip, addr, true, true)
}

#[cfg(CONFIG_FUNCTION_GRAPH_TRACER)]
pub unsafe extern "C" fn prepare_ftrace_return(parent: *mut usize, self_addr: usize, mut frame_pointer: usize) {
    let return_hooker = &return_to_handler as *const _ as usize;
    if atomic_read(&current_tracing_graph_pause) != 0 { return; }
    let old = *parent;
    if function_graph_enter(old, self_addr, *(frame_pointer as *const usize), parent) == 0 {
        *parent = return_hooker;
        frame_pointer += 4;
        if *(frame_pointer as *const usize) == old { *(frame_pointer as *mut usize) = return_hooker; }
    }
}

#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe extern "C" fn ftrace_enable_ftrace_graph_caller() -> i32 { ftrace_modify_code(&ftrace_graph_call as *const _ as usize, &ftrace_graph_caller as *const _ as usize, true, true) }
#[cfg(all(CONFIG_FUNCTION_GRAPH_TRACER, CONFIG_DYNAMIC_FTRACE))]
pub unsafe extern "C" fn ftrace_disable_ftrace_graph_caller() -> i32 { ftrace_modify_code(&ftrace_graph_call as *const _ as usize, &ftrace_graph_caller as *const _ as usize, false, true) }

extern "C" {
    fn copy_from_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> i32;
    fn copy_to_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> i32;
    fn flush_icache_range(start: usize, end: usize);
    fn ftrace_call(); fn ftrace_regs_call(); fn return_to_handler();
    fn ftrace_graph_call(); fn ftrace_graph_caller();
    fn atomic_read(v: *const i32) -> i32;
    fn function_graph_enter(old: usize, self_addr: usize, frame: usize, parent: *mut usize) -> i32;
}

#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_CPU_HAS_ICACHE_INS)))]
#[repr(C)]
struct ftrace_modify_param { command: i32, cpu_count: i32 }

#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_CPU_HAS_ICACHE_INS)))]
unsafe extern "C" fn __ftrace_modify_code(data: *mut core::ffi::c_void) -> i32 {
    let param = data as *mut ftrace_modify_param;
    if atomic_inc_return(&mut (*param).cpu_count) == 1 {
        ftrace_modify_all_code((*param).command);
        atomic_inc(&mut (*param).cpu_count);
    } else {
        while atomic_read(&(*param).cpu_count) <= num_online_cpus() { cpu_relax(); }
        local_icache_inv_all(core::ptr::null_mut());
    }
    0
}

#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_CPU_HAS_ICACHE_INS)))]
pub unsafe extern "C" fn arch_ftrace_update_code(command: i32) {
    let mut param = ftrace_modify_param { command, cpu_count: 0 };
    stop_machine(Some(__ftrace_modify_code), &mut param as *mut _ as *mut _, cpu_online_mask);
}

#[cfg(all(CONFIG_DYNAMIC_FTRACE, not(CONFIG_CPU_HAS_ICACHE_INS)))]
extern "C" {
    fn atomic_inc_return(v: *mut i32) -> i32;
    fn atomic_inc(v: *mut i32);
    fn ftrace_modify_all_code(command: i32);
    fn num_online_cpus() -> i32;
    fn cpu_relax();
    fn local_icache_inv_all(arg: *mut core::ffi::c_void);
    fn stop_machine(fnptr: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>, data: *mut core::ffi::c_void, mask: *const core::ffi::c_void) -> i32;
    static cpu_online_mask: *const core::ffi::c_void;
}

// _mcount is defined in abi's mcount.S.
extern "C" { fn _mcount(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
