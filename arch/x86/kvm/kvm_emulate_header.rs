/* SPDX-License-Identifier: GPL-2.0 */
/* Translation of kvm_emulate.h. Included C dependencies remain external. */

use core::ffi::c_void;

// Forward declarations from the C header are resolved by the definitions below.

#[repr(C)]
pub union x86_exception_address {
    pub address: u64,
    pub dr6: usize,
    pub payload: u64,
}

#[repr(C)]
pub struct x86_exception {
    pub vector: u8,
    pub error_code_valid: bool,
    pub error_code: u64,
    pub nested_page_fault: bool,
    pub address_or_dr6_or_payload: x86_exception_address,
    pub async_page_fault: u8,
    pub exit_qualification: usize,
}

#[repr(C)]
pub struct x86_instruction_info {
    pub intercept: u8,
    pub rep_prefix: u8,
    pub modrm_mod: u8,
    pub modrm_reg: u8,
    pub modrm_rm: u8,
    pub src_val: u64,
    pub dst_val: u64,
    pub src_bytes: u8,
    pub dst_bytes: u8,
    pub src_type: u8,
    pub dst_type: u8,
    pub ad_bytes: u8,
    pub rip: u64,
    pub next_rip: u64,
}

#[repr(C)]
pub struct x86_emulate_ops {
    pub vm_bugged: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt)>,
    pub read_gpr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32) -> usize>,
    pub write_gpr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, usize)>,
    pub read_std: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, *mut c_void, u32, *mut x86_exception, bool) -> i32>,
    pub write_std: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, *mut c_void, u32, *mut x86_exception, bool) -> i32>,
    pub fetch: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, *mut c_void, u32, *mut x86_exception) -> i32>,
    pub read_emulated: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, *mut c_void, u32, *mut x86_exception) -> i32>,
    pub write_emulated: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, *const c_void, u32, *mut x86_exception) -> i32>,
    pub cmpxchg_emulated: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, *const c_void, *const c_void, u32, *mut x86_exception) -> i32>,
    pub invlpg: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize)>,
    pub pio_in_emulated: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32, u16, *mut c_void, u32) -> i32>,
    pub pio_out_emulated: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32, u16, *const c_void, u32) -> i32>,
    pub get_segment: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut u16, *mut desc_struct, *mut u32, i32) -> bool>,
    pub set_segment: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u16, *mut desc_struct, u32, i32)>,
    pub get_cached_segment_base: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32) -> usize>,
    pub get_gdt: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut desc_ptr)>,
    pub get_idt: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut desc_ptr)>,
    pub set_gdt: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut desc_ptr)>,
    pub set_idt: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut desc_ptr)>,
    pub get_cr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32) -> usize>,
    pub set_cr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32, usize) -> i32>,
    pub cpl: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> i32>,
    pub get_effective_dr7: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> usize>,
    pub get_dr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32) -> usize>,
    pub set_dr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, i32, usize) -> i32>,
    pub set_msr_with_filter: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, u64) -> i32>,
    pub get_msr_with_filter: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, *mut u64) -> i32>,
    pub get_msr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, *mut u64) -> i32>,
    pub check_rdpmc_early: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32) -> i32>,
    pub read_pmc: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, *mut u64) -> i32>,
    pub halt: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt)>,
    pub wbinvd: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt)>,
    pub fix_hypercall: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> i32>,
    pub intercept: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut x86_instruction_info, x86_intercept_stage) -> i32>,
    pub is_cpuid_allowed: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> bool>,
    pub get_cpuid: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, *mut u32, *mut u32, *mut u32, *mut u32, bool) -> bool>,
    pub guest_has_movbe: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> bool>,
    pub guest_has_fxsr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> bool>,
    pub guest_has_rdpid: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> bool>,
    pub guest_cpuid_is_intel_compatible: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> bool>,
    pub set_nmi_mask: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, bool)>,
    pub is_smm: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> bool>,
    pub leave_smm: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> i32>,
    pub triple_fault: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt)>,
    pub get_xcr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, *mut u64) -> i32>,
    pub set_xcr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, u32, u64) -> i32>,
    pub get_untagged_addr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, u32) -> usize>,
    pub is_canonical_addr: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize, u32) -> bool>,
    pub page_address_valid: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt, usize) -> bool>,
}

pub enum desc_struct {}
pub enum desc_ptr {}
pub type gpa_t = usize;
pub type gva_t = usize;
pub type sse128_t = [u8; 16];
pub type avx256_t = [u8; 32];

#[repr(C)] pub struct segmented_address { pub ea: usize, pub seg: u32 }
#[repr(C)] pub union operand_orig { pub orig_val: usize, pub orig_val64: u64 }
#[repr(C)] pub union operand_addr { pub reg: *mut usize, pub mem: segmented_address, pub xmm: u32, pub mm: u32 }
#[repr(C)] pub union operand_value { pub val: usize, pub val64: u64, pub valptr: [i8; 32], pub vec_val: sse128_t, pub vec_val2: avx256_t, pub mm_val: u64, pub data: *mut c_void }
#[repr(C)] pub struct operand { pub r#type: u32, pub bytes: u32, pub count: u32, pub orig: operand_orig, pub addr: operand_addr, pub value: operand_value }

pub const X86_MAX_INSTRUCTION_LENGTH: usize = 15;
#[repr(C)] pub struct fetch_cache { pub data: [u8; 15], pub ptr: *mut u8, pub end: *mut u8 }
#[repr(C)] pub struct read_cache { pub data: [u8; 1024], pub pos: usize, pub end: usize }

#[repr(C)] pub enum x86emul_mode { X86EMUL_MODE_REAL, X86EMUL_MODE_VM86, X86EMUL_MODE_PROT16, X86EMUL_MODE_PROT32, X86EMUL_MODE_PROT64 }
pub struct fastop;
pub type fastop_t = Option<unsafe extern "C" fn(*mut fastop)>;
pub const NR_EMULATOR_GPRS: usize = 16;
#[repr(C)] pub enum rex_type { REX_NONE, REX_PREFIX }

#[repr(C)] pub union ctxt_execute { pub execute: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> i32>, pub fop: fastop_t }
#[repr(C)] pub struct x86_emulate_ctxt {
    pub vcpu: *mut c_void, pub ops: *const x86_emulate_ops, pub eflags: usize, pub eip: usize, pub mode: x86emul_mode,
    pub interruptibility: i32, pub perm_ok: bool, pub tf: bool, pub have_exception: bool, pub exception: x86_exception,
    pub gpa_available: bool, pub gpa_val: gpa_t, pub opcode_len: u8, pub b: u8, pub intercept: u8, pub op_prefix: bool,
    pub op_bytes: u8, pub ad_bytes: u8, pub execute: ctxt_execute, pub check_perm: Option<unsafe extern "C" fn(*mut x86_emulate_ctxt) -> i32>,
    pub rip_relative: bool, pub rex_prefix: rex_type, pub rex_bits: u8, pub lock_prefix: u8, pub rep_prefix: u8,
    pub regs_valid: u16, pub regs_dirty: u16, pub modrm: u8, pub modrm_mod: u8, pub modrm_reg: u8, pub modrm_rm: u8,
    pub modrm_seg: u8, pub seg_override: u8, pub d: u64, pub _eip: usize, pub src: operand, pub src2: operand,
    pub dst: operand, pub memop: operand, pub _regs: [usize; NR_EMULATOR_GPRS], pub memopp: *mut operand,
    pub fetch: fetch_cache, pub io_read: read_cache, pub mem_read: read_cache, pub is_branch: bool,
}

pub const X86EMUL_CONTINUE: i32 = 0; pub const X86EMUL_UNHANDLEABLE: i32 = 1; pub const X86EMUL_PROPAGATE_FAULT: i32 = 2;
pub const X86EMUL_RETRY_INSTR: i32 = 3; pub const X86EMUL_CMPXCHG_FAILED: i32 = 4; pub const X86EMUL_IO_NEEDED: i32 = 5;
pub const X86EMUL_INTERCEPTED: i32 = 6; pub const X86EMUL_UNHANDLEABLE_VECTORING: i32 = 7;
pub const X86EMUL_F_WRITE: u32 = 1 << 0; pub const X86EMUL_F_FETCH: u32 = 1 << 1; pub const X86EMUL_F_IMPLICIT: u32 = 1 << 2;
pub const X86EMUL_F_INVLPG: u32 = 1 << 3; pub const X86EMUL_F_MSR: u32 = 1 << 4; pub const X86EMUL_F_DT_LOAD: u32 = 1 << 5;
pub const REPE_PREFIX: u8 = 0xf3; pub const REPNE_PREFIX: u8 = 0xf2;

pub const X86EMUL_CPUID_VENDOR_AuthenticAMD_ebx: u32 = 0x68747541; pub const X86EMUL_CPUID_VENDOR_AuthenticAMD_ecx: u32 = 0x444d4163; pub const X86EMUL_CPUID_VENDOR_AuthenticAMD_edx: u32 = 0x69746e65;
pub const X86EMUL_CPUID_VENDOR_AMDisbetterI_ebx: u32 = 0x69444d41; pub const X86EMUL_CPUID_VENDOR_AMDisbetterI_ecx: u32 = 0x21726574; pub const X86EMUL_CPUID_VENDOR_AMDisbetterI_edx: u32 = 0x74656273;
pub const X86EMUL_CPUID_VENDOR_HygonGenuine_ebx: u32 = 0x6f677948; pub const X86EMUL_CPUID_VENDOR_HygonGenuine_ecx: u32 = 0x656e6975; pub const X86EMUL_CPUID_VENDOR_HygonGenuine_edx: u32 = 0x6e65476e;
pub const X86EMUL_CPUID_VENDOR_GenuineIntel_ebx: u32 = 0x756e6547; pub const X86EMUL_CPUID_VENDOR_GenuineIntel_ecx: u32 = 0x6c65746e; pub const X86EMUL_CPUID_VENDOR_GenuineIntel_edx: u32 = 0x49656e69;
pub const X86EMUL_CPUID_VENDOR_CentaurHauls_ebx: u32 = 0x746e6543; pub const X86EMUL_CPUID_VENDOR_CentaurHauls_ecx: u32 = 0x736c7561; pub const X86EMUL_CPUID_VENDOR_CentaurHauls_edx: u32 = 0x48727561;

pub unsafe extern "C" fn is_guest_vendor_intel(ebx:u32,ecx:u32,edx:u32)->bool { ebx==X86EMUL_CPUID_VENDOR_GenuineIntel_ebx && ecx==X86EMUL_CPUID_VENDOR_GenuineIntel_ecx && edx==X86EMUL_CPUID_VENDOR_GenuineIntel_edx }
pub unsafe extern "C" fn is_guest_vendor_amd(ebx:u32,ecx:u32,edx:u32)->bool { (ebx==X86EMUL_CPUID_VENDOR_AuthenticAMD_ebx&&ecx==X86EMUL_CPUID_VENDOR_AuthenticAMD_ecx&&edx==X86EMUL_CPUID_VENDOR_AuthenticAMD_edx)||(ebx==X86EMUL_CPUID_VENDOR_AMDisbetterI_ebx&&ecx==X86EMUL_CPUID_VENDOR_AMDisbetterI_ecx&&edx==X86EMUL_CPUID_VENDOR_AMDisbetterI_edx) }
pub unsafe extern "C" fn is_guest_vendor_hygon(ebx:u32,ecx:u32,edx:u32)->bool { ebx==X86EMUL_CPUID_VENDOR_HygonGenuine_ebx&&ecx==X86EMUL_CPUID_VENDOR_HygonGenuine_ecx&&edx==X86EMUL_CPUID_VENDOR_HygonGenuine_edx }

#[repr(C)] pub enum x86_intercept_stage { X86_ICTP_NONE=0, X86_ICPT_PRE_EXCEPT, X86_ICPT_POST_EXCEPT, X86_ICPT_POST_MEMACCESS }
#[repr(C)] pub enum x86_intercept { x86_intercept_none, x86_intercept_cr_read, x86_intercept_cr_write, x86_intercept_clts, x86_intercept_lmsw, x86_intercept_smsw, x86_intercept_dr_read, x86_intercept_dr_write, x86_intercept_lidt, x86_intercept_sidt, x86_intercept_lgdt, x86_intercept_sgdt, x86_intercept_lldt, x86_intercept_sldt, x86_intercept_ltr, x86_intercept_str, x86_intercept_rdtsc, x86_intercept_rdpmc, x86_intercept_pushf, x86_intercept_popf, x86_intercept_cpuid, x86_intercept_rsm, x86_intercept_iret, x86_intercept_intn, x86_intercept_invd, x86_intercept_pause, x86_intercept_hlt, x86_intercept_invlpg, x86_intercept_invlpga, x86_intercept_vmrun, x86_intercept_vmload, x86_intercept_vmsave, x86_intercept_vmmcall, x86_intercept_stgi, x86_intercept_clgi, x86_intercept_skinit, x86_intercept_rdtscp, x86_intercept_rdpid, x86_intercept_icebp, x86_intercept_wbinvd, x86_intercept_monitor, x86_intercept_mwait, x86_intercept_rdmsr, x86_intercept_wrmsr, x86_intercept_in, x86_intercept_ins, x86_intercept_out, x86_intercept_outs, x86_intercept_xsetbv, nr_x86_intercepts }

pub const EMULATION_FAILED:i32=-1; pub const EMULATION_OK:i32=0; pub const EMULATION_RESTART:i32=1; pub const EMULATION_INTERCEPTED:i32=2;
extern "C" { pub fn x86_decode_insn(ctxt:*mut x86_emulate_ctxt, insn:*mut c_void, insn_len:i32, emulation_type:i32)->i32; pub fn x86_page_table_writing_insn(ctxt:*mut x86_emulate_ctxt)->bool; pub fn init_decode_cache(ctxt:*mut x86_emulate_ctxt); pub fn x86_emulate_insn(ctxt:*mut x86_emulate_ctxt, check_intercepts:bool)->i32; pub fn emulator_task_switch(ctxt:*mut x86_emulate_ctxt,tss_selector:u16,idt_index:i32,reason:i32,has_error_code:bool,error_code:u32)->i32; pub fn emulate_int_real(ctxt:*mut x86_emulate_ctxt,irq:i32)->i32; pub fn emulator_invalidate_register_cache(ctxt:*mut x86_emulate_ctxt); pub fn emulator_writeback_register_cache(ctxt:*mut x86_emulate_ctxt); pub fn emulator_can_use_gpa(ctxt:*mut x86_emulate_ctxt)->bool; }

// KVM_EMULATOR_BUG_ON retains the source macro's side-effect contract; WARN_ON_ONCE
// and unlikely are supplied by the surrounding kernel translation.
#[macro_export]
macro_rules! KVM_EMULATOR_BUG_ON { ($cond:expr, $ctxt:expr) => {{ let __ret = $cond; if __ret { unsafe { if let Some(f) = (*$ctxt).ops.as_ref().and_then(|o| o.vm_bugged) { f($ctxt); } } } __ret }} }

pub unsafe fn reg_read(ctxt: *mut x86_emulate_ctxt, mut nr: u32) -> usize {
    if nr >= NR_EMULATOR_GPRS as u32 { nr &= (NR_EMULATOR_GPRS - 1) as u32; }
    if (*ctxt).regs_valid & (1u16 << nr) == 0 {
        (*ctxt).regs_valid |= 1u16 << nr;
        (*ctxt)._regs[nr as usize] = ((*ctxt).ops).as_ref().unwrap().read_gpr.unwrap()(ctxt, nr);
    }
    (*ctxt)._regs[nr as usize]
}

pub unsafe fn reg_write(ctxt: *mut x86_emulate_ctxt, mut nr: u32) -> *mut usize {
    if nr >= NR_EMULATOR_GPRS as u32 { nr &= (NR_EMULATOR_GPRS - 1) as u32; }
    (*ctxt).regs_valid |= 1u16 << nr;
    (*ctxt).regs_dirty |= 1u16 << nr;
    &mut (*ctxt)._regs[nr as usize]
}

pub unsafe fn reg_rmw(ctxt: *mut x86_emulate_ctxt, nr: u32) -> *mut usize { reg_read(ctxt, nr); reg_write(ctxt, nr) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
