/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2022 Loongson Technology Corporation Limited */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stack_type {
    STACK_TYPE_UNKNOWN,
    STACK_TYPE_IRQ,
    STACK_TYPE_TASK,
}

#[repr(C)]
pub struct stack_info {
    pub type_: stack_type,
    pub begin: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
    pub next_sp: ::core::ffi::c_ulong,
}

#[repr(C)]
pub struct stack_frame {
    pub fp: ::core::ffi::c_ulong,
    pub ra: ::core::ffi::c_ulong,
}

pub enum task_struct {}

extern "C" {
    pub fn in_irq_stack(stack: ::core::ffi::c_ulong, info: *mut stack_info) -> bool;
    pub fn in_task_stack(
        stack: ::core::ffi::c_ulong,
        task: *mut task_struct,
        info: *mut stack_info,
    ) -> bool;
    pub fn get_stack_info(
        stack: ::core::ffi::c_ulong,
        task: *mut task_struct,
        info: *mut stack_info,
    ) -> ::core::ffi::c_int;
}

// The C header's build-time architecture constants and stringify macros are
// represented as literals here; their values are supplied by the target ABI.
pub const STR_LONG_L: &str = stringify!(LONG_L);
pub const STR_LONG_S: &str = stringify!(LONG_S);
pub const STR_LONGSIZE: &str = stringify!(LONGSIZE);

#[inline(always)]
pub unsafe fn on_thread_stack() -> bool {
    !((((*current).stack as ::core::ffi::c_ulong) ^ current_stack_pointer)
        & !(THREAD_SIZE - 1))
        != 0
}

#[repr(C)]
pub struct pt_regs {
    pub regs: *mut ::core::ffi::c_ulong,
    pub csr_era: ::core::ffi::c_ulong,
    pub csr_badvaddr: ::core::ffi::c_ulong,
    pub csr_crmd: ::core::ffi::c_ulong,
    pub csr_prmd: ::core::ffi::c_ulong,
    pub csr_euen: ::core::ffi::c_ulong,
    pub csr_ecfg: ::core::ffi::c_ulong,
    pub csr_estat: ::core::ffi::c_ulong,
}

extern "C" {
    static mut current: *mut task_struct;
    static mut current_stack_pointer: ::core::ffi::c_ulong;
    static THREAD_SIZE: ::core::ffi::c_ulong;
}

#[inline(always)]
pub unsafe fn prepare_frametrace(regs: *mut pt_regs) {
    // Save and restore the register frame and read the exception CSRs exactly
    // as the original LoongArch inline assembly does.
    ::core::arch::asm!(
        "/* UNWIND_HINT_SAVE */",
        "st.d $r1, {regs}, 8*1",
        "pcaddi $ra, 0",
        "st.d $ra, [{era}]",
        "ld.d $ra, {regs}, 8*1",
        "st.d $r2, {regs}, 8*2",
        "st.d $r3, {regs}, 8*3",
        "st.d $r4, {regs}, 8*4",
        "st.d $r5, {regs}, 8*5",
        "st.d $r6, {regs}, 8*6",
        "st.d $r7, {regs}, 8*7",
        "st.d $r8, {regs}, 8*8",
        "st.d $r9, {regs}, 8*9",
        "st.d $r10, {regs}, 8*10",
        "st.d $r11, {regs}, 8*11",
        "st.d $r12, {regs}, 8*12",
        "st.d $r13, {regs}, 8*13",
        "st.d $r14, {regs}, 8*14",
        "st.d $r15, {regs}, 8*15",
        "st.d $r16, {regs}, 8*16",
        "st.d $r17, {regs}, 8*17",
        "st.d $r18, {regs}, 8*18",
        "st.d $r19, {regs}, 8*19",
        "st.d $r20, {regs}, 8*20",
        "st.d $r21, {regs}, 8*21",
        "st.d $r22, {regs}, 8*22",
        "st.d $r23, {regs}, 8*23",
        "st.d $r24, {regs}, 8*24",
        "st.d $r25, {regs}, 8*25",
        "st.d $r26, {regs}, 8*26",
        "st.d $r27, {regs}, 8*27",
        "st.d $r28, {regs}, 8*28",
        "st.d $r29, {regs}, 8*29",
        "st.d $r30, {regs}, 8*30",
        "st.d $r31, {regs}, 8*31",
        "/* UNWIND_HINT_RESTORE */",
        regs = in(regs), era = in(regs), options(nostack)
    );
    ::core::arch::asm!("csrrd {0}, LOONGARCH_CSR_BADV", out(reg) (*regs).csr_badvaddr);
    ::core::arch::asm!("csrrd {0}, LOONGARCH_CSR_CRMD", out(reg) (*regs).csr_crmd);
    ::core::arch::asm!("csrrd {0}, LOONGARCH_CSR_PRMD", out(reg) (*regs).csr_prmd);
    ::core::arch::asm!("csrrd {0}, LOONGARCH_CSR_EUEN", out(reg) (*regs).csr_euen);
    ::core::arch::asm!("csrrd {0}, LOONGARCH_CSR_ECFG", out(reg) (*regs).csr_ecfg);
    ::core::arch::asm!("csrrd {0}, LOONGARCH_CSR_ESTAT", out(reg) (*regs).csr_estat);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
