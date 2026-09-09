/* SPDX-License-Identifier: GPL-2.0 */

//! Rust translation of x86/include/asm/nospec-branch.h.
//! Configuration and assembler-only branches from the C header are retained
//! as comments or string-producing macros where Rust has no preprocessor
//! equivalent.

pub const RET_DEPTH_SHIFT: u32 = 5;
pub const RSB_RET_STUFF_LOOPS: u32 = 16;
pub const RET_DEPTH_INIT: u64 = 0x8000_0000_0000_0000;
pub const RET_DEPTH_INIT_FROM_CALL: u64 = 0xfc00_0000_0000_0000;
pub const RET_DEPTH_CREDIT: u64 = 0xffff_ffff_ffff_ffff;

pub const RETPOLINE_THUNK_SIZE: usize = 32;
pub const RSB_CLEAR_LOOPS: usize = 32;
pub const ITS_THUNK_SIZE: usize = 64;

pub type RetpolineThunk = [u8; RETPOLINE_THUNK_SIZE];
pub type ItsThunk = [u8; ITS_THUNK_SIZE];

/* C configuration conditions: CONFIG_CALL_THUNKS_DEBUG,
 * CONFIG_MITIGATION_CALL_DEPTH_TRACKING, and COMPILE_OFFSETS control the
 * corresponding assembly macros in the original header. */
#[macro_export]
macro_rules! credit_call_depth {
    () => { "movq $-1, PER_CPU_VAR(__x86_call_depth)" };
}
#[macro_export]
macro_rules! reset_call_depth {
    () => { "xor %eax, %eax; bts $63, %rax; movq %rax, PER_CPU_VAR(__x86_call_depth)" };
}
#[macro_export]
macro_rules! reset_call_depth_from_call {
    () => { "movb $0xfc, %al; shl $56, %rax; movq %rax, PER_CPU_VAR(__x86_call_depth)" };
}
#[macro_export]
macro_rules! increment_call_depth {
    () => { "sarq $5, PER_CPU_VAR(__x86_call_depth)" };
}

#[macro_export]
macro_rules! fill_return_slot {
    () => { "ANNOTATE_INTRA_FUNCTION_CALL; call 772f; int3; 772:" };
}

#[macro_export]
macro_rules! fill_return_buffer {
    ($reg:expr, $nr:expr) => {
        concat!("mov $(", $nr, "/2), ", $reg,
                "; __FILL_RETURN_SLOT; __FILL_RETURN_SLOT; add $(BITS_PER_LONG/8)*2, %_ASM_SP; dec ",
                $reg, "; jnz 771b; lfence")
    };
}

#[macro_export]
macro_rules! fill_one_return {
    () => { "__FILL_RETURN_SLOT; add $(BITS_PER_LONG/8), %_ASM_SP; lfence" };
}

#[macro_export]
macro_rules! handle_intr_saferet {
    ($name:expr, $pt_regs:expr) => {
        concat!("cmpq $(", $name, "), RIP+", $pt_regs,
                "; jb 1f; cmpq $(", $name, ")+5, RIP+", $pt_regs,
                "; ja 1f; lfence; leaq ", $pt_regs,
                ", %rdi; call handle_interrupted_saferet; 1:")
    };
}

/* Assembler-only macros (JMP_NOSPEC, CALL_NOSPEC, FILL_RETURN_BUFFER,
 * CALL_UNTRAIN_RET, __UNTRAIN_RET, HANDLE_INTR_SAFERET, CALL_DEPTH_ACCOUNT,
 * CLEAR_BRANCH_HISTORY and related alternatives) retain their original
 * CONFIG-dependent intent here; their bodies are consumed by assembly users. */
#[macro_export]
macro_rules! clear_cpu_buffers {
    () => { "ALTERNATIVE \"\", __CLEAR_CPU_BUFFERS, X86_FEATURE_CLEAR_CPU_BUF" };
}

extern "C" {
    pub static mut __x86_indirect_thunk_array: [RetpolineThunk; 0];
    pub static mut __x86_indirect_call_thunk_array: [RetpolineThunk; 0];
    pub static mut __x86_indirect_jump_thunk_array: [RetpolineThunk; 0];
    pub static mut __x86_indirect_its_thunk_array: [ItsThunk; 0];

    pub fn __x86_return_thunk();
    pub fn retbleed_return_thunk();
    pub fn srso_alias_untrain_ret();
    pub fn srso_return_thunk();
    pub fn srso_alias_return_thunk();
    pub fn its_return_thunk();
    pub fn entry_untrain_ret();
    pub fn write_ibpb();
    pub fn bpf_arch_ibpb();
    pub fn clear_bhb_loop();
    pub static mut x86_return_thunk: Option<unsafe extern "C" fn()>;
    pub fn __warn_thunk();
    pub fn call_depth_return_thunk();

    pub static mut x86_spec_ctrl_base: u64;
    pub static mut x86_verw_sel: u16;
    pub fn update_spec_ctrl_cond(val: u64);
    pub fn spec_ctrl_current() -> u64;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SpectreV2Mitigation {
    SpectreV2None,
    SpectreV2Retpoline,
    SpectreV2Lfence,
    SpectreV2Eibrs,
    SpectreV2EibrsRetpoline,
    SpectreV2EibrsLfence,
    SpectreV2Ibrs,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SpectreV2UserMitigation {
    SpectreV2UserNone,
    SpectreV2UserStrict,
    SpectreV2UserStrictPreferred,
    SpectreV2UserPrctl,
    SpectreV2UserSeccomp,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SsbMitigation {
    SpecStoreBypassNone,
    SpecStoreBypassAuto,
    SpecStoreBypassDisable,
    SpecStoreBypassPrctl,
    SpecStoreBypassSeccomp,
}

pub unsafe fn alternative_msr_write(_msr: u32, _val: u64, _feature: u32) {
    /* ALTERNATIVE("", "wrmsr", feature); register constraints are supplied
     * by the platform's eventual low-level implementation. */
}

pub unsafe fn indirect_branch_prediction_barrier() {
    /* ALTERNATIVE("", "call write_ibpb", X86_FEATURE_IBPB). */
}

pub unsafe fn firmware_restrict_branch_speculation_start() {
    preempt_disable();
    alternative_msr_write(MSR_IA32_SPEC_CTRL, spec_ctrl_current() | SPEC_CTRL_IBRS, X86_FEATURE_USE_IBRS_FW);
    alternative_msr_write(MSR_IA32_PRED_CMD, PRED_CMD_IBPB, X86_FEATURE_USE_IBPB_FW);
}

pub unsafe fn firmware_restrict_branch_speculation_end() {
    alternative_msr_write(MSR_IA32_SPEC_CTRL, spec_ctrl_current(), X86_FEATURE_USE_IBRS_FW);
    preempt_enable();
}

pub unsafe fn x86_clear_cpu_buffers() {
    /* asm volatile("verw %[ds]" : : [ds] "m" (ds) : "cc"); */
}

pub unsafe fn x86_idle_clear_cpu_buffers() {
    if static_branch_likely(&cpu_buf_idle_clear) {
        x86_clear_cpu_buffers();
    }
}

extern "C" {
    pub fn srso_safe_ret();
    pub fn srso_alias_safe_ret();
    pub fn handle_interrupted_saferet(regs: *mut PtRegs);
}

/* External dependencies supplied by the translated kernel headers. */
extern "C" {
    fn preempt_disable();
    fn preempt_enable();
    fn static_branch_likely(key: *const StaticKeyFalse) -> bool;
    static cpu_buf_idle_clear: StaticKeyFalse;
}

#[repr(C)]
pub struct PtRegs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct StaticKeyFalse {
    _private: [u8; 0],
}

/* Symbolic constants supplied by asm/msr-index.h and asm/cpufeatures.h. */
const MSR_IA32_SPEC_CTRL: u32 = 0;
const MSR_IA32_PRED_CMD: u32 = 0;
const SPEC_CTRL_IBRS: u64 = 0;
const PRED_CMD_IBPB: u64 = 0;
const X86_FEATURE_USE_IBRS_FW: u32 = 0;
const X86_FEATURE_USE_IBPB_FW: u32 = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
