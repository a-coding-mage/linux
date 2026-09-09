// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2017 Arm Ltd.
// pr_fmt(fmt) expands to "sdei: " fmt

use core::ffi::c_void;

pub const ENOMEM: i32 = 12;

pub const SDEI_STACK_SIZE: usize = 0; // supplied by the kernel headers
pub const SMCCC_CONDUIT_HVC: i32 = 0; // supplied by the kernel headers
pub const SDEI_EXIT_HVC: usize = 0; // supplied by the kernel headers
pub const SDEI_EXIT_SMC: usize = 0; // supplied by the kernel headers
pub const SDEI_EV_HANDLED: usize = 0; // supplied by the kernel headers
pub const SDEI_EV_FAILED: usize = 0; // supplied by the kernel headers
pub const PSR_MODE32_BIT: u64 = 0; // supplied by the architecture headers
pub const PSR_MODE_MASK: u64 = 0; // supplied by the architecture headers
pub const TRAMP_VALIAS: usize = 0; // supplied by the architecture headers

#[repr(C)]
pub struct pt_regs {
    pub regs: [u64; 31],
    pub pstate: u64,
}

#[repr(C)]
pub struct sdei_registered_event {
    _private: [u8; 0],
}

extern "C" {
    fn per_cpu_possible_count() -> i32;
    fn arch_alloc_vmap_stack(size: usize, node: i32) -> *mut usize;
    fn vfree(p: *mut usize);
    fn cpu_to_node(cpu: i32) -> i32;
    fn scs_alloc(node: i32) -> *mut c_void;
    fn scs_free(s: *mut c_void);
    fn scs_is_enabled() -> bool;
    fn is_hyp_nvhe() -> bool;
    fn arm64_kernel_unmapped_at_el0() -> bool;
    fn sdei_api_event_context(index: i32, value: *mut u64) -> i32;
    fn sdei_event_handler(regs: *mut pt_regs, arg: *mut sdei_registered_event) -> i32;
    fn regs_irqs_disabled(regs: *const pt_regs) -> bool;
    fn read_elr_el1() -> u64;
    fn read_current_el() -> u64;
    fn read_vbar_el1() -> usize;
    static __sdei_asm_entry_trampoline: u8;
    static __entry_tramp_text_start: u8;
    static __sdei_asm_handler: u8;
}

pub static mut sdei_exit_mode: usize = 0;

// VMAP'd stacks use sp as a scratch register while checking stack overflow on
// exception, so SDEI has to switch to its own stack. These are per-CPU slots.
pub static mut sdei_stack_normal_ptr: *mut usize = core::ptr::null_mut();
pub static mut sdei_stack_critical_ptr: *mut usize = core::ptr::null_mut();
pub static mut sdei_shadow_call_stack_normal_ptr: *mut usize = core::ptr::null_mut();
pub static mut sdei_shadow_call_stack_critical_ptr: *mut usize = core::ptr::null_mut();
pub static mut sdei_active_normal_event: *mut sdei_registered_event = core::ptr::null_mut();
pub static mut sdei_active_critical_event: *mut sdei_registered_event = core::ptr::null_mut();

unsafe fn _free_sdei_stack(ptr: *mut *mut usize, _cpu: i32) {
    let p = *ptr;
    if !p.is_null() {
        *ptr = core::ptr::null_mut();
        vfree(p);
    }
}

unsafe fn free_sdei_stacks() {
    for cpu in 0..per_cpu_possible_count() {
        _free_sdei_stack(&raw mut sdei_stack_normal_ptr, cpu);
        _free_sdei_stack(&raw mut sdei_stack_critical_ptr, cpu);
    }
}

unsafe fn _init_sdei_stack(ptr: *mut *mut usize, cpu: i32) -> i32 {
    let p = arch_alloc_vmap_stack(SDEI_STACK_SIZE, cpu_to_node(cpu));
    if p.is_null() {
        return -ENOMEM;
    }
    *ptr = p;
    0
}

unsafe fn init_sdei_stacks() -> i32 {
    let mut err = 0;
    for cpu in 0..per_cpu_possible_count() {
        err = _init_sdei_stack(&raw mut sdei_stack_normal_ptr, cpu);
        if err != 0 { break; }
        err = _init_sdei_stack(&raw mut sdei_stack_critical_ptr, cpu);
        if err != 0 { break; }
    }
    if err != 0 { free_sdei_stacks(); }
    err
}

unsafe fn _free_sdei_scs(ptr: *mut *mut usize, _cpu: i32) {
    let s = *ptr as *mut c_void;
    if !s.is_null() {
        *ptr = core::ptr::null_mut();
        scs_free(s);
    }
}

unsafe fn free_sdei_scs() {
    for cpu in 0..per_cpu_possible_count() {
        _free_sdei_scs(&raw mut sdei_shadow_call_stack_normal_ptr, cpu);
        _free_sdei_scs(&raw mut sdei_shadow_call_stack_critical_ptr, cpu);
    }
}

unsafe fn _init_sdei_scs(ptr: *mut *mut usize, cpu: i32) -> i32 {
    let s = scs_alloc(cpu_to_node(cpu));
    if s.is_null() { return -ENOMEM; }
    *ptr = s as *mut usize;
    0
}

unsafe fn init_sdei_scs() -> i32 {
    if !scs_is_enabled() { return 0; }
    let mut err = 0;
    for cpu in 0..per_cpu_possible_count() {
        err = _init_sdei_scs(&raw mut sdei_shadow_call_stack_normal_ptr, cpu);
        if err != 0 { break; }
        err = _init_sdei_scs(&raw mut sdei_shadow_call_stack_critical_ptr, cpu);
        if err != 0 { break; }
    }
    if err != 0 { free_sdei_scs(); }
    err
}

pub unsafe fn sdei_arch_get_entry_point(conduit: i32) -> usize {
    if is_hyp_nvhe() { return 0; }
    if init_sdei_stacks() != 0 { return 0; }
    if init_sdei_scs() != 0 { free_sdei_stacks(); return 0; }
    sdei_exit_mode = if conduit == SMCCC_CONDUIT_HVC { SDEI_EXIT_HVC } else { SDEI_EXIT_SMC };
    // CONFIG_UNMAP_KERNEL_AT_EL0 is a build-time condition.
    if arm64_kernel_unmapped_at_el0() {
        let offset = (&__sdei_asm_entry_trampoline as *const u8 as usize)
            .wrapping_sub(&__entry_tramp_text_start as *const u8 as usize);
        TRAMP_VALIAS.wrapping_add(offset)
    } else {
        &__sdei_asm_handler as *const u8 as usize
    }
}

pub unsafe fn do_sdei_event(regs: *mut pt_regs, arg: *mut sdei_registered_event) -> usize {
    let elr = read_elr_el1();
    let kernel_mode = read_current_el() as u32 | 1;
    let vbar = read_vbar_el1();
    let clobbered_registers = if arm64_kernel_unmapped_at_el0() { 5 } else { 4 };
    for i in 0..clobbered_registers {
        sdei_api_event_context(i, &mut (*regs).regs[i as usize]);
    }
    if sdei_event_handler(regs, arg) != 0 { return SDEI_EV_FAILED; }
    if elr != read_elr_el1() { /* unsafe: exception during handler */ }
    let mode = (*regs).pstate & (PSR_MODE32_BIT | PSR_MODE_MASK);
    if mode == kernel_mode as u64 && regs_irqs_disabled(regs) { return SDEI_EV_HANDLED; }
    if mode == kernel_mode as u64 { return vbar.wrapping_add(0x280); }
    if mode & PSR_MODE32_BIT != 0 { return vbar.wrapping_add(0x680); }
    vbar.wrapping_add(0x480)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
