/* Translated from mips/include/asm/processor.h. */

/* C header dependencies are supplied by the surrounding translation unit. */

extern "C" {
    pub static mut vced_count: u32;
    pub static mut vcei_count: u32;
    pub fn arch_dup_task_struct(dst: *mut task_struct, src: *mut task_struct) -> i32;
    pub fn mips_stack_top() -> c_ulong;
    pub fn start_thread(regs: *mut pt_regs, pc: c_ulong, sp: c_ulong);
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
    pub fn mips_get_process_fp_mode(task: *mut task_struct) -> i32;
    pub fn mips_set_process_fp_mode(task: *mut task_struct, value: u32) -> i32;
    pub fn show_registers(regs: *mut pt_regs);
}

#[cfg(CONFIG_32BIT)]
pub const TASK_SIZE: c_ulong = 0x80000000;
#[cfg(CONFIG_32BIT)]
pub const STACK_TOP_MAX: c_ulong = TASK_SIZE;
#[cfg(CONFIG_32BIT)]
pub const TASK_IS_32BIT_ADDR: i32 = 1;

#[cfg(CONFIG_64BIT)]
pub const TASK_SIZE32: c_ulong = 0x7fff8000;
#[cfg(all(CONFIG_64BIT, not(CONFIG_MIPS_VA_BITS_48)))]
pub const TASK_SIZE64: c_ulong = 0x10000000000;
/* CONFIG_MIPS_VA_BITS_48 preserves the source's cpu_data/test_thread_flag dependency. */
#[cfg(all(CONFIG_64BIT, CONFIG_MIPS_VA_BITS_48))]
pub const TASK_SIZE64: c_ulong = 1u64 << 48;
#[cfg(CONFIG_64BIT)]
pub const STACK_TOP_MAX: c_ulong = TASK_SIZE64;

pub const VDSO_RANDOMIZE_SIZE: c_ulong = if unsafe { test_thread_flag(TIF_32BIT_ADDR) } != 0 {
    SZ_1M
} else { SZ_64M };
pub const STACK_TOP: c_ulong = unsafe { mips_stack_top() };
pub const NUM_FPU_REGS: usize = 32;
#[cfg(CONFIG_CPU_HAS_MSA)]
pub const FPU_REG_WIDTH: usize = 128;
#[cfg(not(CONFIG_CPU_HAS_MSA))]
pub const FPU_REG_WIDTH: usize = 64;

#[repr(C)]
pub union fpureg {
    pub val32: [u32; FPU_REG_WIDTH / 32],
    pub val64: [u64; FPU_REG_WIDTH / 64],
}

#[inline]
pub unsafe fn get_fpr32(fpr: *mut fpureg, idx: usize) -> u32 {
    (*fpr).val32[idx]
}
#[inline]
pub unsafe fn set_fpr32(fpr: *mut fpureg, idx: usize, val: u32) {
    (*fpr).val32[idx] = val;
}
#[inline]
pub unsafe fn get_fpr64(fpr: *mut fpureg, idx: usize) -> u64 {
    (*fpr).val64[idx]
}
#[inline]
pub unsafe fn set_fpr64(fpr: *mut fpureg, idx: usize, val: u64) {
    (*fpr).val64[idx] = val;
}

#[repr(C)]
pub struct mips_fpu_struct {
    pub fpr: [fpureg; NUM_FPU_REGS],
    pub fcr31: u32,
    pub msacsr: u32,
}

pub const NUM_DSP_REGS: usize = 6;
pub type dspreg_t = c_ulong;
#[repr(C)]
pub struct mips_dsp_state { pub dspr: [dspreg_t; NUM_DSP_REGS], pub dspcontrol: u32 }

#[repr(C)]
pub struct mips3264_watch_reg_state {
    pub watchlo: [c_ulong; NUM_WATCH_REGS],
    pub watchhi: [u16; NUM_WATCH_REGS],
}
#[repr(C)]
pub union mips_watch_reg_state { pub mips3264: mips3264_watch_reg_state }

#[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
#[repr(C)]
pub struct octeon_cop2_state {
    pub cop2_crc_iv: c_ulong, pub cop2_crc_length: c_ulong, pub cop2_crc_poly: c_ulong,
    pub cop2_llm_dat: [c_ulong; 2], pub cop2_3des_iv: c_ulong, pub cop2_3des_key: [c_ulong; 3],
    pub cop2_3des_result: c_ulong, pub cop2_aes_inp0: c_ulong, pub cop2_aes_iv: [c_ulong; 2],
    pub cop2_aes_key: [c_ulong; 4], pub cop2_aes_keylen: c_ulong, pub cop2_aes_result: [c_ulong; 2],
    pub cop2_hsh_datw: [c_ulong; 15], pub cop2_hsh_ivw: [c_ulong; 8],
    pub cop2_gfm_mult: [c_ulong; 2], pub cop2_gfm_poly: c_ulong, pub cop2_gfm_result: [c_ulong; 2],
    pub cop2_sha3: [c_ulong; 2],
}

#[repr(C)]
pub struct thread_struct {
    pub reg16: c_ulong, pub reg17: c_ulong, pub reg18: c_ulong, pub reg19: c_ulong,
    pub reg20: c_ulong, pub reg21: c_ulong, pub reg22: c_ulong, pub reg23: c_ulong,
    pub reg29: c_ulong, pub reg30: c_ulong, pub reg31: c_ulong, pub cp0_status: c_ulong,
    #[cfg(CONFIG_MIPS_FP_SUPPORT)] pub fpu: mips_fpu_struct,
    #[cfg(CONFIG_MIPS_FP_SUPPORT)] pub bd_emu_frame: atomic_t,
    #[cfg(CONFIG_MIPS_FP_SUPPORT)] pub bd_emu_branch_pc: c_ulong,
    #[cfg(CONFIG_MIPS_FP_SUPPORT)] pub bd_emu_cont_pc: c_ulong,
    #[cfg(CONFIG_MIPS_MT_FPAFF)] pub emulated_fp: c_ulong,
    #[cfg(CONFIG_MIPS_MT_FPAFF)] pub user_cpus_allowed: cpumask_t,
    pub dsp: mips_dsp_state,
    pub watch: mips_watch_reg_state,
    pub cp0_badvaddr: c_ulong, pub cp0_baduaddr: c_ulong, pub error_code: c_ulong,
    pub trap_nr: c_ulong,
    #[cfg(CONFIG_CPU_CAVIUM_OCTEON)] pub cp2: octeon_cop2_state,
    pub abi: *mut mips_abi,
}

pub fn flush_thread() {}

/* Stack/register access macros retain their C expressions for dependent types and helpers. */
/* __KSTK_TOS(tsk) = (task_stack_page(tsk) as c_ulong) + THREAD_SIZE - 32 - sizeof(pt_regs) */
/* task_pt_regs(tsk), KSTK_EIP(tsk), KSTK_ESP(tsk), KSTK_STATUS(tsk) */

#[cfg(CONFIG_CPU_HAS_PREFETCH)]
pub const ARCH_HAS_PREFETCH: bool = true;
#[cfg(CONFIG_CPU_HAS_PREFETCH)]
pub const ARCH_HAS_PREFETCHW: bool = true;

pub const GET_FP_MODE: unsafe fn(*mut task_struct) -> i32 = mips_get_process_fp_mode;
pub const SET_FP_MODE: unsafe fn(*mut task_struct, u32) -> i32 = mips_set_process_fp_mode;

#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct mips_abi;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
