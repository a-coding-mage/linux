/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2020-2022 Loongson Technology Corporation Limited */

// Dependencies supplied by the surrounding kernel translation:
// linux/threads.h, asm/{addrspace,asm,asmmacro,asm-offsets,loongarch,
// thread_info,unwind_hints}.h

/* Assembly macros translated as Rust macro_rules! wrappers around inline asm. */

macro_rules! cfi_rel_offset { ($reg:tt, $offset:expr, $docfi:expr) => {{
    if $docfi != 0 { unsafe { core::arch::asm!(concat!(".cfi_rel_offset ", stringify!($reg), ", ", stringify!($offset))); } }
}}; }
macro_rules! cfi_st { ($reg:tt, $offset:expr, $docfi:expr) => {{
    cfi_rel_offset!($reg, $offset, $docfi);
    unsafe { core::arch::asm!(concat!("LONG_S ", stringify!($reg), ", sp, ", stringify!($offset))); }
}}; }
macro_rules! cfi_restore { ($reg:tt, $offset:expr, $docfi:expr) => {{
    if $docfi != 0 { unsafe { core::arch::asm!(concat!(".cfi_restore ", stringify!($reg))); } }
}}; }
macro_rules! cfi_ld { ($reg:tt, $offset:expr, $docfi:expr) => {{
    unsafe { core::arch::asm!(concat!("LONG_L ", stringify!($reg), ", sp, ", stringify!($offset))); }
    cfi_restore!($reg, $offset, $docfi);
}}; }

macro_rules! SETUP_TWINS { ($temp:tt) => { unsafe { core::arch::asm!(r#"
pcaddi t0, 0
PTR_LI t1, ~TO_PHYS_MASK
and t0, t0, t1
ori t0, t0, (1 << 4 | 1)
csrwr t0, LOONGARCH_CSR_DMWIN0
PTR_LI t0, CSR_DMW1_INIT
csrwr t0, LOONGARCH_CSR_DMWIN1"#); } }; }
macro_rules! SETUP_MODES { ($temp:tt) => { unsafe { core::arch::asm!(concat!("li.w ", stringify!($temp), ", 0xb0\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_CRMD\nli.w ", stringify!($temp), ", 0x04\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_PRMD\nli.w ", stringify!($temp), ", 0x00\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_EUEN")); } }; }
macro_rules! SETUP_DMWINS { ($temp:tt) => { unsafe { core::arch::asm!(concat!("PTR_LI ", stringify!($temp), ", CSR_DMW0_INIT\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_DMWIN0\nPTR_LI ", stringify!($temp), ", CSR_DMW1_INIT\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_DMWIN1\nPTR_LI ", stringify!($temp), ", CSR_DMW2_INIT\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_DMWIN2\nPTR_LI ", stringify!($temp), ", CSR_DMW3_INIT\ncsrwr ", stringify!($temp), ", LOONGARCH_CSR_DMWIN3")); } }; }
macro_rules! JUMP_VIRT_ADDR { ($temp1:tt, $temp2:tt) => { unsafe { core::arch::asm!(concat!("PTR_LI ", stringify!($temp1), ", CACHE_BASE\npcaddi ", stringify!($temp2), ", 0\nPTR_BSTRINS ", stringify!($temp1), ", ", stringify!($temp2), ", (DMW_PABITS - 1), 0\njirl zero, ", stringify!($temp1), ", 0xc")); } }; }
macro_rules! STACKLEAK_ERASE { () => { /* CONFIG_KSTACK_ERASE: bl stackleak_erase_on_task_stack */ }; }
macro_rules! BACKUP_T0T1 { () => { unsafe { core::arch::asm!("csrwr t0, EXCEPTION_KS0\ncsrwr t1, EXCEPTION_KS1"); } }; }
macro_rules! RELOAD_T0T1 { () => { unsafe { core::arch::asm!("csrrd t0, EXCEPTION_KS0\ncsrrd t1, EXCEPTION_KS1"); } }; }

macro_rules! SAVE_TEMP { ($docfi:expr) => {{ RELOAD_T0T1!(); cfi_st!(t0, PT_R12, $docfi); cfi_st!(t1, PT_R13, $docfi); cfi_st!(t2, PT_R14, $docfi); cfi_st!(t3, PT_R15, $docfi); cfi_st!(t4, PT_R16, $docfi); cfi_st!(t5, PT_R17, $docfi); cfi_st!(t6, PT_R18, $docfi); cfi_st!(t7, PT_R19, $docfi); cfi_st!(t8, PT_R20, $docfi); }}; }
macro_rules! SAVE_STATIC { ($docfi:expr) => {{ cfi_st!(s0, PT_R23, $docfi); cfi_st!(s1, PT_R24, $docfi); cfi_st!(s2, PT_R25, $docfi); cfi_st!(s3, PT_R26, $docfi); cfi_st!(s4, PT_R27, $docfi); cfi_st!(s5, PT_R28, $docfi); cfi_st!(s6, PT_R29, $docfi); cfi_st!(s7, PT_R30, $docfi); cfi_st!(s8, PT_R31, $docfi); }}; }
macro_rules! RESTORE_TEMP { ($docfi:expr) => {{ cfi_ld!(t0, PT_R12, $docfi); cfi_ld!(t1, PT_R13, $docfi); cfi_ld!(t2, PT_R14, $docfi); cfi_ld!(t3, PT_R15, $docfi); cfi_ld!(t4, PT_R16, $docfi); cfi_ld!(t5, PT_R17, $docfi); cfi_ld!(t6, PT_R18, $docfi); cfi_ld!(t7, PT_R19, $docfi); cfi_ld!(t8, PT_R20, $docfi); }}; }
macro_rules! RESTORE_STATIC { ($docfi:expr) => {{ cfi_ld!(s0, PT_R23, $docfi); cfi_ld!(s1, PT_R24, $docfi); cfi_ld!(s2, PT_R25, $docfi); cfi_ld!(s3, PT_R26, $docfi); cfi_ld!(s4, PT_R27, $docfi); cfi_ld!(s5, PT_R28, $docfi); cfi_ld!(s6, PT_R29, $docfi); cfi_ld!(s7, PT_R30, $docfi); cfi_ld!(s8, PT_R31, $docfi); }}; }

macro_rules! SAVE_SOME { ($docfi:expr) => { unsafe { core::arch::asm!(r#"csrrd t1, LOONGARCH_CSR_PRMD
andi t1, t1, 0x3
move t0, sp
beqz t1, 8f
/* Called from user mode, new stack. */
/* get_saved_sp */
8:
PTR_ADDI sp, sp, -PT_SIZE
LONG_S zero, sp, PT_R0
csrrd t0, LOONGARCH_CSR_PRMD
LONG_S t0, sp, PT_PRMD
csrrd t0, LOONGARCH_CSR_CRMD
LONG_S t0, sp, PT_CRMD
csrrd t0, LOONGARCH_CSR_EUEN
LONG_S t0, sp, PT_EUEN
csrrd t0, LOONGARCH_CSR_ECFG
LONG_S t0, sp, PT_ECFG
csrrd t0, LOONGARCH_CSR_ESTAT
PTR_S t0, sp, PT_ESTAT
LONG_S ra, sp, PT_R1
LONG_S a0, sp, PT_R4
LONG_S a1, sp, PT_R5
LONG_S a2, sp, PT_R6
LONG_S a3, sp, PT_R7
LONG_S a4, sp, PT_R8
LONG_S a5, sp, PT_R9
LONG_S a6, sp, PT_R10
LONG_S a7, sp, PT_R11
csrrd ra, LOONGARCH_CSR_ERA
LONG_S ra, sp, PT_ERA
LONG_S tp, sp, PT_R2
LONG_S u0, sp, PT_R21
LONG_S fp, sp, PT_R22"#); } }; }
macro_rules! SAVE_ALL { ($docfi:expr) => {{ SAVE_SOME!($docfi); SAVE_TEMP!($docfi); SAVE_STATIC!($docfi); }}; }
macro_rules! RESTORE_SOME { ($docfi:expr) => { unsafe { core::arch::asm!("LONG_L a0, sp, PT_PRMD\nandi a0, a0, 0x3\nbeqz a0, 8f\nLONG_L u0, sp, PT_R21\n8:\nLONG_L a0, sp, PT_ERA\ncsrwr a0, LOONGARCH_CSR_ERA\nLONG_L a0, sp, PT_PRMD\ncsrwr a0, LOONGARCH_CSR_PRMD\nLONG_L ra, sp, PT_R1\nLONG_L a0, sp, PT_R4\nLONG_L a1, sp, PT_R5\nLONG_L a2, sp, PT_R6\nLONG_L a3, sp, PT_R7\nLONG_L a4, sp, PT_R8\nLONG_L a5, sp, PT_R9\nLONG_L a6, sp, PT_R10\nLONG_L a7, sp, PT_R11\nLONG_L tp, sp, PT_R2\nLONG_L fp, sp, PT_R22"); } }; }
macro_rules! RESTORE_SP_AND_RET { ($docfi:expr) => { unsafe { core::arch::asm!("LONG_L sp, sp, PT_R3\nUNWIND_HINT_FUNC\nertn"); } }; }
macro_rules! RESTORE_ALL_AND_RET { ($docfi:expr) => {{ RESTORE_STATIC!($docfi); RESTORE_TEMP!($docfi); RESTORE_SOME!($docfi); RESTORE_SP_AND_RET!($docfi); }}; }

macro_rules! get_saved_sp { ($docfi:expr) => { unsafe { core::arch::asm!("la_abs t1, kernelsp\ncsrrd t0, PERCPU_BASE_KS\nmove t0, sp\nLONG_L sp, t1, 0"); } }; }
macro_rules! set_saved_sp { ($stackp:tt, $temp:tt, $temp2:tt) => { unsafe { core::arch::asm!(concat!("la.pcrel ", stringify!($temp), ", kernelsp\nLONG_S ", stringify!($stackp), ", ", stringify!($temp), ", 0")); } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
