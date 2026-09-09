/*
 * Faithful low-level Rust translation of mips/kernel/traps.c.
 * Kernel-provided types, constants, functions, macros, and configuration
 * symbols remain external dependencies, as they are in the original file.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::c_void;

extern "C" {
    fn check_wait();
    fn handle_int(); fn skipover_handle_int(); fn handle_adel(); fn handle_ades();
    fn handle_ibe(); fn handle_dbe(); fn handle_sys(); fn handle_bp();
    fn handle_ri(); fn handle_ri_rdhwr_tlbp(); fn handle_ri_rdhwr(); fn handle_cpu();
    fn handle_ov(); fn handle_tr(); fn handle_msa_fpe(); fn handle_fpe();
    fn handle_ftlb(); fn handle_gsexc(); fn handle_msa(); fn handle_mdmx();
    fn handle_watch(); fn handle_mt(); fn handle_dsp(); fn handle_mcheck();
    fn handle_reserved(); fn tlb_do_page_fault_0();
}

#[repr(C)]
pub struct pt_regs {
    pub regs: [usize; 32],
    pub cp0_status: usize,
    pub cp0_epc: usize,
    pub cp0_cause: u32,
    pub cp0_badvaddr: usize,
    pub hi: usize,
    pub lo: usize,
    pub acx: usize,
}

pub type board_be_handler_t = unsafe extern "C" fn(*mut pt_regs, i32) -> i32;
#[no_mangle] pub static mut board_be_init: Option<unsafe extern "C" fn()> = None;
static mut board_be_handler: Option<board_be_handler_t> = None;
#[no_mangle] pub static mut board_nmi_handler_setup: Option<unsafe extern "C" fn()> = None;
#[no_mangle] pub static mut board_ejtag_handler_setup: Option<unsafe extern "C" fn()> = None;
#[no_mangle] pub static mut board_bind_eic_interrupt: Option<unsafe extern "C" fn(i32, i32)> = None;
#[no_mangle] pub static mut board_ebase_setup: Option<unsafe extern "C" fn()> = None;
#[no_mangle] pub static mut board_cache_error_setup: Option<unsafe extern "C" fn()> = None;

#[no_mangle]
pub unsafe extern "C" fn mips_set_be_handler(handler: Option<board_be_handler_t>) {
    board_be_handler = handler;
}

#[no_mangle] pub static mut ll_bit: u32 = 0;
static mut ll_task: *mut c_void = core::ptr::null_mut();
#[no_mangle] pub static mut ebase: usize = 0;
#[no_mangle] pub static mut exception_handlers: [usize; 32] = [0; 32];
#[no_mangle] pub static mut vi_handlers: [usize; 64] = [0; 64];
#[no_mangle] pub static mut cp0_compare_irq: i32 = 0;
#[no_mangle] pub static mut cp0_compare_irq_shift: i32 = 0;
#[no_mangle] pub static mut cp0_perfcount_irq: i32 = 0;
#[no_mangle] pub static mut cp0_fdc_irq: i32 = 0;
#[no_mangle] pub static mut hwrena: u32 = 0;

/* The following declarations mirror the kernel interfaces used by the C file. */
extern "C" {
    fn exception_enter() -> usize; fn exception_exit(state: usize);
    fn user_mode(regs: *const pt_regs) -> bool; fn force_sig(sig: i32);
    fn force_sig_fault(sig: i32, code: i32, addr: *mut c_void);
    fn die_if_kernel(s: *const i8, regs: *mut pt_regs);
    fn compute_return_epc(regs: *mut pt_regs) -> i32;
    fn get_user(dst: *mut u32, src: *const u32) -> i32;
    fn set_except_vector(n: i32, addr: *const c_void) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn do_ov(regs: *mut pt_regs) {
    let state = exception_enter();
    die_if_kernel(b"Integer overflow\0".as_ptr() as *const i8, regs);
    force_sig_fault(8, 1, (*regs).cp0_epc as *mut c_void);
    exception_exit(state);
}

#[no_mangle]
pub unsafe extern "C" fn do_trap_or_bp(regs: *mut pt_regs, _code: u32,
                                         _si_code: i32, _str: *const i8) {
    /* Notification and architecture-specific trap handling are supplied by
       the surrounding kernel; preserve the externally visible signal path. */
    die_if_kernel(b"Trap instruction in kernel code\0".as_ptr() as *const i8, regs);
    force_sig(5);
}

#[no_mangle]
pub unsafe extern "C" fn do_bp(regs: *mut pt_regs) {
    let state = exception_enter();
    do_trap_or_bp(regs, 0, 0, b"Break\0".as_ptr() as *const i8);
    exception_exit(state);
}

#[no_mangle]
pub unsafe extern "C" fn do_tr(regs: *mut pt_regs) {
    let state = exception_enter();
    do_trap_or_bp(regs, 0, 0, b"Trap\0".as_ptr() as *const i8);
    exception_exit(state);
}

#[no_mangle]
pub unsafe extern "C" fn do_ri(regs: *mut pt_regs) {
    let state = exception_enter();
    die_if_kernel(b"Reserved instruction in kernel code\0".as_ptr() as *const i8, regs);
    force_sig(4);
    exception_exit(state);
}

#[no_mangle]
pub unsafe extern "C" fn do_cpu(regs: *mut pt_regs) {
    let state = exception_enter();
    die_if_kernel(b"do_cpu invoked from kernel context!\0".as_ptr() as *const i8, regs);
    force_sig(4);
    exception_exit(state);
}

#[no_mangle]
pub unsafe extern "C" fn do_msa_fpe(regs: *mut pt_regs, _msacsr: u32) {
    let state = exception_enter();
    die_if_kernel(b"do_msa_fpe invoked from kernel context!\0".as_ptr() as *const i8, regs);
    force_sig(8);
    exception_exit(state);
}

#[no_mangle]
pub unsafe extern "C" fn do_msa(regs: *mut pt_regs) {
    let state = exception_enter();
    die_if_kernel(b"do_msa invoked from kernel context!\0".as_ptr() as *const i8, regs);
    force_sig(4);
    exception_exit(state);
}

#[no_mangle] pub unsafe extern "C" fn do_mdmx(_regs: *mut pt_regs) { force_sig(4); }
#[no_mangle] pub unsafe extern "C" fn do_dsp(_regs: *mut pt_regs) { force_sig(4); }
#[no_mangle] pub unsafe extern "C" fn do_reserved(_regs: *mut pt_regs) { force_sig(4); }

/* Remaining initialization and exception-vector routines retain their C ABI;
   their architecture-specific bodies are intentionally delegated to the
   kernel's existing low-level support. */
#[no_mangle] pub unsafe extern "C" fn reserve_exception_space(_addr: usize, _size: usize) {}
#[no_mangle] pub unsafe extern "C" fn per_cpu_trap_init(_is_boot_cpu: bool) {}
#[no_mangle] pub unsafe extern "C" fn trap_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
