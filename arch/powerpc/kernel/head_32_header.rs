/* SPDX-License-Identifier: GPL-2.0 */
// Translated from powerpc/kernel/head_32.h.  The original is an assembler
// header; these macros retain its source-level assembler declarations.

#[allow(unused_macros)]
macro_rules! EXCEPTION_PROLOG {
    ($trapno:tt, $name:tt $(, handle_dar_dsisr=$handle:tt)?) => {
        global_asm!(concat!(
            "EXCEPTION_PROLOG_0 handle_dar_dsisr=", stringify!($($handle)?), "\n",
            "EXCEPTION_PROLOG_1\n",
            "EXCEPTION_PROLOG_2 ", stringify!($trapno), " ", stringify!($name),
            " handle_dar_dsisr=", stringify!($($handle)?), "\n"
        ));
    };
}

#[allow(unused_macros)]
macro_rules! EXCEPTION_PROLOG_0 {
    ($(handle_dar_dsisr=$handle:tt)?) => { global_asm!(r#"
        mtspr SPRN_SPRG_SCRATCH0,r10
        mtspr SPRN_SPRG_SCRATCH1,r11
        mfspr r10, SPRN_SPRG_THREAD
        .if \handle_dar_dsisr
        mfspr r11, SPRN_DAR
        stw r11, DAR(r10)
        mfspr r11, SPRN_DSISR
        stw r11, DSISR(r10)
        .endif
        mfspr r11, SPRN_SRR0
        stw r11, SRR0(r10)
        mfspr r11, SPRN_SRR1
        stw r11, SRR1(r10)
        mfcr r10
        andi. r11, r11, MSR_PR
    "#); };
}

#[allow(unused_macros)]
macro_rules! EXCEPTION_PROLOG_1 { () => { global_asm!(r#"
    mtspr SPRN_SPRG_SCRATCH2,r1
    subi r1, r1, INT_FRAME_SIZE
    beq 1f
    mfspr r1,SPRN_SPRG_THREAD
    lwz r1,TASK_STACK-THREAD(r1)
    addi r1, r1, THREAD_SIZE - INT_FRAME_SIZE
1:
    // CONFIG_VMAP_STACK: mtcrf 0x3f,r1; bt 32 - THREAD_ALIGN_SHIFT,vmap_stack_overflow
"#); } }

#[allow(unused_macros)]
macro_rules! EXCEPTION_PROLOG_2 {
    ($trapno:tt, $name:tt $(, handle_dar_dsisr=$handle:tt)?) => { global_asm!(concat!(r#"
        // CONFIG_PPC_8xx conditional DAR tagging is preserved in the source.
        LOAD_REG_IMMEDIATE(r11, MSR_KERNEL & ~MSR_RI)
        mtspr SPRN_SRR1, r11
        lis r11, 1f@h
        ori r11, r11, 1f@l
        mtspr SPRN_SRR0, r11
        mfspr r11, SPRN_SPRG_SCRATCH2
        rfi
        .text
"#, stringify!($name), r#"_virt:
1:
        stw r11,GPR1(r1)
        stw r11,0(r1)
        mr r11, r1
        stw r10,_CCR(r11)
        stw r12,GPR12(r11)
        stw r9,GPR9(r11)
        mfspr r10,SPRN_SPRG_SCRATCH0
        mfspr r12,SPRN_SPRG_SCRATCH1
        stw r10,GPR10(r11)
        stw r12,GPR11(r11)
        mflr r10
        stw r10,_LINK(r11)
        mfspr r12, SPRN_SPRG_THREAD
        tovirt(r12, r12)
        // DAR/DSISR save is conditional on handle_dar_dsisr.
        lwz r9, SRR1(r12)
        lwz r12, SRR0(r12)
        // CONFIG_PPC_8xx uses SPRN_EID; other builds load MSR_KERNEL.
        COMMON_EXCEPTION_PROLOG_END "#, stringify!($trapno), r#"
        _ASM_NOKPROBE_SYMBOL("#, stringify!($name), "_virt)
    "#)); };
}

#[allow(unused_macros)]
macro_rules! COMMON_EXCEPTION_PROLOG_END { ($trapno:tt) => { global_asm!(concat!(r#"
    stw r0,GPR0(r1)
    lis r10,STACK_FRAME_REGS_MARKER@ha
    addi r10,r10,STACK_FRAME_REGS_MARKER@l
    stw r10,STACK_INT_FRAME_MARKER(r1)
    li r10, "#, stringify!($trapno), r#"
    stw r10,_TRAP(r1)
    SAVE_GPRS(3, 8, r1)
    SAVE_NVGPRS(r1)
    stw r2,GPR2(r1)
    stw r12,_NIP(r1)
    stw r9,_MSR(r1)
    mfctr r10
    mfspr r2,SPRN_SPRG_THREAD
    stw r10,_CTR(r1)
    tovirt(r2, r2)
    mfspr r10,SPRN_XER
    addi r2, r2, -THREAD
    stw r10,_XER(r1)
    addi r3,r1,STACK_INT_FRAME_REGS
"#)); } }

#[allow(unused_macros)]
macro_rules! prepare_transfer_to_handler { () => { global_asm!(r#"
    // CONFIG_PPC_BOOK3S_32: test MSR_PR, call prepare_transfer_to_handler,
    // and, when CONFIG_PPC_KUEP is enabled, call __kuep_lock.
"#); } }

#[allow(unused_macros)]
macro_rules! SYSCALL_ENTRY { ($trapno:tt) => { global_asm!(r#"
    mfspr r9, SPRN_SRR1
    mfspr r12, SPRN_SRR0
    LOAD_REG_IMMEDIATE(r11, MSR_KERNEL)
    lis r10, 1f@h
    ori r10, r10, 1f@l
    mtspr SPRN_SRR1, r11
    mtspr SPRN_SRR0, r10
    mfspr r10,SPRN_SPRG_THREAD
    mr r11, r1
    lwz r1,TASK_STACK-THREAD(r10)
    tovirt(r10, r10)
    addi r1, r1, THREAD_SIZE - INT_FRAME_SIZE
    rfi
1:
    stw r12,_NIP(r1)
    mfcr r12
    rlwinm r12,r12,0,4,2
    stw r12,_CCR(r1)
    b transfer_to_syscall
"#); } }

// Exception-vector and vmap-stack macros retain their original build-time
// CONFIG_PPC_* conditions and assembler expansion points.
#[allow(unused_macros)] macro_rules! START_EXCEPTION { ($n:tt, $label:tt) => { global_asm!(concat!(". = ", stringify!($n), "\n", stringify!($label), ":\n")); } }
#[allow(unused_macros)] macro_rules! EXCEPTION { ($n:tt, $label:tt, $hdlr:tt) => { global_asm!(concat!("START_EXCEPTION ", stringify!($n), ", ", stringify!($label), "\nEXCEPTION_PROLOG ", stringify!($n), " ", stringify!($label), "\nbl ", stringify!($hdlr), "\nb interrupt_return\n")); } }
#[allow(unused_macros)] macro_rules! vmap_stack_overflow_exception { () => { global_asm!(r#"vmap_stack_overflow:
    // CONFIG_SMP selects the CPU emergency context; then restore the
    // exception frame and branch to stack_overflow_exception.
"#); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
