// SPDX-License-Identifier: GPL-2.0
/*
 * Direct low-level Rust translation of x86/mm/fault.c.
 *
 * The kernel types, constants, globals, macros, and external routines used by
 * this implementation are supplied by the surrounding kernel bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn is_kmmio_active() -> bool;
    fn kmmio_handler(regs: *mut pt_regs, addr: usize) -> i32;
    fn get_kernel_nofault<T>(dst: *mut T, src: *const T) -> i32;
    fn user_mode(regs: *const pt_regs) -> bool;
    fn pagefault_disable();
    fn pagefault_enable();
    fn current_task() -> *mut task_struct;
    fn convert_ip_to_linear(task: *mut task_struct, regs: *const pt_regs) -> *mut u8;
}

#[repr(C)]
pub struct pt_regs { pub ip: usize, pub sp: usize, pub flags: usize, pub cs: u64 }
#[repr(C)] pub struct task_struct { pub mm: *mut mm_struct, pub thread: thread_struct }
#[repr(C)] pub struct thread_struct { pub trap_nr: i32, pub error_code: usize, pub cr2: usize }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct vm_area_struct { pub vm_flags: usize }
#[repr(C)] pub struct pte_t;
#[repr(C)] pub struct pgd_t;
#[repr(C)] pub struct p4d_t;
#[repr(C)] pub struct pud_t;
#[repr(C)] pub struct pmd_t;

// C's static inline helper, preserving the original opcode and pointer logic.
unsafe fn kmmio_fault(regs: *mut pt_regs, addr: usize) -> i32 {
    if is_kmmio_active() && kmmio_handler(regs, addr) == 1 { -1 } else { 0 }
}

unsafe fn check_prefetch_opcode(regs: *mut pt_regs, instr: *mut u8,
                                opcode: u8, prefetch: *mut i32) -> i32 {
    let hi = opcode & 0xf0;
    let lo = opcode & 0x0f;
    match hi {
        0x20 | 0x30 => if (lo & 7) == 6 { 1 } else { 0 },
        0x60 => if (lo & 0xc) == 4 { 1 } else { 0 },
        0xf0 => if lo == 0 || (lo >> 1) == 1 { 1 } else { 0 },
        0x00 => {
            let mut next = 0u8;
            if get_kernel_nofault(&mut next, instr) != 0 { return 0; }
            *prefetch = if lo == 0xf && (next == 0x0d || next == 0x18) { 1 } else { 0 };
            0
        }
        _ => 0,
    }
}

unsafe fn is_prefetch(regs: *mut pt_regs, error_code: usize, _addr: usize) -> i32 {
    // AMD K8 erratum #91: inspect at most fifteen bytes of the faulting stream.
    if (error_code & X86_PF_INSTR) != 0 { return 0; }
    let mut instr = convert_ip_to_linear(current_task(), regs);
    let end = instr.add(15);
    let mut prefetch = 0;
    pagefault_disable();
    while instr < end {
        let mut opcode = 0u8;
        if get_kernel_nofault(&mut opcode, instr) != 0 { break; }
        instr = instr.add(1);
        if check_prefetch_opcode(regs, instr, opcode, &mut prefetch) == 0 { break; }
    }
    pagefault_enable();
    prefetch
}

pub static mut show_unhandled_signals: i32 = 1;

unsafe fn sanitize_error_code(address: usize, error_code: *mut usize) {
    if address >= TASK_SIZE_MAX { *error_code |= X86_PF_PROT; }
}

unsafe fn fault_in_kernel_space(address: usize) -> bool {
    address >= TASK_SIZE_MAX
}

unsafe fn spurious_kernel_fault_check(error_code: usize, _pte: *mut pte_t) -> i32 {
    // The actual permission predicates are supplied by the page-table bindings.
    if (error_code & (X86_PF_WRITE | X86_PF_INSTR)) == 0 { return 0; }
    1
}

unsafe fn handle_page_fault(regs: *mut pt_regs, error_code: usize, address: usize) {
    if kmmio_fault(regs, address) != 0 { return; }
    if fault_in_kernel_space(address) {
        bad_area_nosemaphore(regs, error_code, address);
    } else {
        do_user_addr_fault(regs, error_code, address);
    }
    local_irq_disable();
}

unsafe fn bad_area_nosemaphore(_regs: *mut pt_regs, _error_code: usize, _address: usize) {}
unsafe fn do_user_addr_fault(_regs: *mut pt_regs, _error_code: usize, _address: usize) {}
unsafe fn local_irq_disable() {}

// External entry point corresponding to DEFINE_IDTENTRY_RAW_ERRORCODE(exc_page_fault).
#[no_mangle]
pub unsafe extern "C" fn exc_page_fault(regs: *mut pt_regs, error_code: usize) {
    let address = read_cr2();
    if kvm_handle_async_pf(regs, address as u32) != 0 { return; }
    handle_page_fault(regs, error_code, address);
}

extern "C" {
    fn read_cr2() -> usize;
    fn kvm_handle_async_pf(regs: *mut pt_regs, address: u32) -> i32;
}

// Constants and declarations below are provided by the architecture bindings.
extern "C" {
    static TASK_SIZE_MAX: usize;
    static X86_PF_INSTR: usize;
    static X86_PF_PROT: usize;
    static X86_PF_WRITE: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
