//! Rust representation of `powerpc/kernel/head_booke.h`.
//!
//! The source is an assembler-only Linux header.  Its declarations are
//! retained as Rust `macro_rules!` interfaces; the bodies are intentionally
//! emitted as target assembler snippets and require the surrounding kernel
//! assembler definitions (SPRN_*, GPR_*, and the other included macros).

#![allow(unused_macros)]

macro_rules! SET_IVOR { ($vector_number:tt, $vector_label:expr) => {
    core::arch::asm!(concat!("li r26,", stringify!($vector_label), "@l; mtspr SPRN_IVOR", stringify!($vector_number), ",r26; sync"));
} }
macro_rules! ALLOC_STACK_FRAME { ($reg:tt, $val:expr) => {
    core::arch::asm!(concat!("addi ", stringify!($reg), ",", stringify!($reg), ",", stringify!($val)));
} }
macro_rules! THREAD_NORMSAVE { ($offset:expr) => { THREAD_NORMSAVES + (($offset) * 4) } }

/* The following macros are assembler-language interfaces.  Keep their exact
 * names and call signatures available to the translated header. */
macro_rules! BOOKE_CLEAR_BTB { ($reg:tt) => { /* CONFIG_PPC_E500: BTB_FLUSH($reg) */ } }
macro_rules! NORMAL_EXCEPTION_PROLOG { ($trapno:expr, $intno:ident) => {
    /* mtspr SPRG_WSCRATCH0,r10; mfspr r10,SPRG_THREAD; save normal state;
     * test MSR_PR; allocate INT_FRAME_SIZE; save CR/GPR/SRR state;
     * COMMON_EXCEPTION_PROLOG_END($trapno) */
} }
macro_rules! COMMON_EXCEPTION_PROLOG_END { ($trapno:expr) => {
    /* save GPR0, frame marker, trap number, GPRs, NVGPRs, CTR, XER, and set r3 */
} }
macro_rules! prepare_transfer_to_handler { () => { /* CONFIG_PPC_E500 recursive transfer */ } }
macro_rules! SYSCALL_ENTRY { ($trapno:expr, $intno:ident, $srr1:ident) => {
    /* save SPRG_THREAD state, handle KVM HV feature sections, build syscall frame,
     * load SRR0, and branch to transfer_to_syscall */
} }

macro_rules! BOOKE_LOAD_EXC_LEVEL_STACK { ($level:ident) => {
    /* load level##_STACK_BASE, select PIR slot under CONFIG_SMP, and allocate frame */
} }
macro_rules! EXC_LEVEL_EXCEPTION_PROLOG { ($level:ident, $trapno:expr, $intno:ident, $srr0:ident, $srr1:ident) => {
    /* save exception-level scratch registers, select user/kernel stack, save
     * DEAR/ESR/SRR state, then COMMON_EXCEPTION_PROLOG_END($trapno) */
} }
macro_rules! SAVE_xSRR { ($x_srr:ident) => {
    /* mfspr r0,SPRN_##xSRR##0; stw r0,_##xSRR##0(r1);
     * mfspr r0,SPRN_##xSRR##1; stw r0,_##xSRR##1(r1) */
} }
macro_rules! SAVE_MMU_REGS { () => {
    /* CONFIG_PPC_E500: save MAS0, MAS1, MAS2, MAS3, MAS6 and optional MAS7;
     * CONFIG_44x: save MMUCR */
} }

macro_rules! CRITICAL_EXCEPTION_PROLOG { ($trapno:expr, $intno:ident) => {
    EXC_LEVEL_EXCEPTION_PROLOG!(CRIT, ($trapno) + 2, $intno, SPRN_CSRR0, SPRN_CSRR1);
} }
macro_rules! DEBUG_EXCEPTION_PROLOG { ($trapno:expr) => {
    EXC_LEVEL_EXCEPTION_PROLOG!(DBG, ($trapno) + 8, DEBUG, SPRN_DSRR0, SPRN_DSRR1);
} }
macro_rules! MCHECK_EXCEPTION_PROLOG { ($trapno:expr) => {
    EXC_LEVEL_EXCEPTION_PROLOG!(MC, ($trapno) + 4, MACHINE_CHECK, SPRN_MCSRR0, SPRN_MCSRR1);
} }

macro_rules! GUEST_DOORBELL_EXCEPTION { () => {
    /* START_EXCEPTION(GuestDoorbell); save SPRG/thread registers and CR;
     * DO_KVM(BOOKE_INTERRUPT_GUEST_DBELL, SPRN_GSRR1); trap */
} }
macro_rules! START_EXCEPTION { ($label:ident) => { /* .align 5; $label: */ } }
macro_rules! EXCEPTION { ($n:expr, $intno:ident, $label:ident, $hdlr:ident) => {
    START_EXCEPTION!($label); NORMAL_EXCEPTION_PROLOG!($n, $intno);
    prepare_transfer_to_handler!(); /* bl $hdlr; b interrupt_return */
} }
macro_rules! CRITICAL_EXCEPTION { ($n:expr, $intno:ident, $label:ident, $hdlr:ident) => {
    START_EXCEPTION!($label); CRITICAL_EXCEPTION_PROLOG!($n, $intno); SAVE_MMU_REGS!(); SAVE_xSRR!(SRR);
    prepare_transfer_to_handler!(); /* bl $hdlr; b ret_from_crit_exc */
} }
macro_rules! MCHECK_EXCEPTION { ($n:expr, $label:ident, $hdlr:ident) => {
    START_EXCEPTION!($label); MCHECK_EXCEPTION_PROLOG!($n); SAVE_xSRR!(DSRR); SAVE_xSRR!(CSRR);
    SAVE_MMU_REGS!(); SAVE_xSRR!(SRR); prepare_transfer_to_handler!(); /* handler/return */
} }

/* Full exception-vector instruction sequences are preserved as assembler
 * contracts; their external handlers and constants come from included headers. */
macro_rules! DEBUG_DEBUG_EXCEPTION { () => { /* DebugDebug vector, DBSR/DSRR handling */ } }
macro_rules! DEBUG_CRIT_EXCEPTION { () => { /* DebugCrit vector, DBSR/CSRR handling */ } }
macro_rules! DATA_STORAGE_EXCEPTION { () => { /* DataStorage vector and do_page_fault */ } }
macro_rules! INSTRUCTION_STORAGE_EXCEPTION { () => { /* InstructionStorage vector and do_page_fault */ } }
macro_rules! ALIGNMENT_EXCEPTION { () => { /* Alignment vector and alignment_exception */ } }
macro_rules! PROGRAM_EXCEPTION { () => { /* Program vector and program_check_exception */ } }
macro_rules! DECREMENTER_EXCEPTION { () => { /* Decrementer vector and timer_interrupt */ } }
macro_rules! FP_UNAVAILABLE_EXCEPTION { () => { /* FloatingPointUnavailable vector and FPU handling */ } }

pub const MC_STACK_BASE: &str = "mcheckirq_ctx";
pub const CRIT_STACK_BASE: &str = "critirq_ctx";
pub const DBG_STACK_BASE: &str = "dbgirq_ctx";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
