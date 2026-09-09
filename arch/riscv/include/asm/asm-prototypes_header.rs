/* SPDX-License-Identifier: GPL-2.0 */

// Translated from asm-prototypes.h.
// The C preprocessor conditions CONFIG_RISCV_ISA_V, CONFIG_MMU, and
// CONFIG_RISCV_ISA_V_PREEMPTIVE are build-time conditions supplied externally.

extern "C" {
    pub fn __lshrdi3(a: i64, b: i32) -> i64;
    pub fn __ashrdi3(a: i64, b: i32) -> i64;
    pub fn __ashldi3(a: i64, b: i32) -> i64;

    pub fn __lshrti3(a: i64, b: i32) -> i64;
    pub fn __ashrti3(a: i64, b: i32) -> i64;
    pub fn __ashlti3(a: i64, b: i32) -> i64;

    // CONFIG_RISCV_ISA_V
    // CONFIG_MMU
    pub fn enter_vector_usercopy(
        dst: *mut core::ffi::c_void,
        src: *mut core::ffi::c_void,
        n: usize,
        enable_sum: bool,
    ) -> i32;

    pub fn xor_regs_2_(bytes: usize, p1: *mut usize, p2: *const usize);
    pub fn xor_regs_3_(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize);
    pub fn xor_regs_4_(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
        p4: *const usize,
    );
    pub fn xor_regs_5_(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
        p4: *const usize,
        p5: *const usize,
    );

    // CONFIG_RISCV_ISA_V_PREEMPTIVE
    pub fn riscv_v_context_nesting_start(regs: *mut crate::pt_regs);
    pub fn riscv_v_context_nesting_end(regs: *mut crate::pt_regs);

    pub fn do_trap_unknown(regs: *mut crate::pt_regs);
    pub fn do_trap_hardware_error(regs: *mut crate::pt_regs);
    pub fn do_trap_insn_misaligned(regs: *mut crate::pt_regs);
    pub fn do_trap_insn_fault(regs: *mut crate::pt_regs);
    pub fn do_trap_insn_illegal(regs: *mut crate::pt_regs);
    pub fn do_trap_load_fault(regs: *mut crate::pt_regs);
    pub fn do_trap_load_misaligned(regs: *mut crate::pt_regs);
    pub fn do_trap_store_misaligned(regs: *mut crate::pt_regs);
    pub fn do_trap_store_fault(regs: *mut crate::pt_regs);
    pub fn do_trap_ecall_u(regs: *mut crate::pt_regs);
    pub fn do_trap_ecall_s(regs: *mut crate::pt_regs);
    pub fn do_trap_ecall_m(regs: *mut crate::pt_regs);
    pub fn do_trap_break(regs: *mut crate::pt_regs);
    pub fn do_trap_software_check(regs: *mut crate::pt_regs);

    pub fn ret_from_fork_kernel(
        fn_arg: *mut core::ffi::c_void,
        f: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> i32>,
        regs: *mut crate::pt_regs,
    );
    pub fn ret_from_fork_user(regs: *mut crate::pt_regs);
    pub fn handle_bad_stack(regs: *mut crate::pt_regs);
    pub fn do_page_fault(regs: *mut crate::pt_regs);
    pub fn do_irq(regs: *mut crate::pt_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
