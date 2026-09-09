/* SPDX-License-Identifier: GPL-2.0-or-later */
/* ELF register definitions. */

// C dependencies supplied by other translation units:
// linux/sched.h, asm/page.h, asm/string.h, and uapi/asm/elf.h.

/* This is used to ensure we don't load something for the wrong architecture. */
#[macro_export]
macro_rules! elf_check_arch { ($x:expr) => { ($x).e_machine == ELF_ARCH }; }
#[macro_export]
macro_rules! compat_elf_check_arch { ($x:expr) => { ($x).e_machine == EM_PPC }; }

pub const CORE_DUMP_USE_REGSET: bool = true;
// ELF_EXEC_PAGESIZE is PAGE_SIZE, supplied externally.
pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;

/* Base location for PIE (ET_DYN with INTERP) loads. */
#[inline]
pub const fn elf_et_dyn_base() -> usize {
    if is_32bit_task() { 0x0004_0000_0usize } else { 0x1_0000_0000usize }
}

#[inline]
pub const fn elf_core_eflags() -> i32 { if is_elf2_task() { 2 } else { 0 } }

/* Copy the register set, preserving the C truncation and zero-fill behavior. */
#[macro_export]
macro_rules! PPC_ELF_CORE_COPY_REGS {
    ($elf_regs:expr, $regs:expr) => {{
        let mut i: usize = 0;
        let nregs = core::cmp::min(
            core::mem::size_of_val(&$regs) / core::mem::size_of::<c_ulong>(),
            ELF_NGREG as usize,
        );
        while i < nregs {
            $elf_regs[i] = *((($regs as *const _ as *const c_ulong).add(i)));
            i += 1;
        }
        while i < ELF_NGREG as usize {
            $elf_regs[i] = 0;
            i += 1;
        }
    }};
}

/* Common routine for both 32-bit and 64-bit native processes. */
#[inline]
pub unsafe fn ppc_elf_core_copy_regs(
    elf_regs: *mut elf_gregset_t,
    regs: *mut pt_regs,
) {
    PPC_ELF_CORE_COPY_REGS!(*elf_regs, regs);
}

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS { ($gregs:expr, $regs:expr) => {
    ppc_elf_core_copy_regs($gregs, $regs)
}; }

/* ELF_HWCAP yields a mask describing the instruction set supported by this CPU. */
#[macro_export]
macro_rules! ELF_HWCAP { () => { cur_cpu_spec.cpu_user_features }; }
#[macro_export]
macro_rules! ELF_HWCAP2 { () => { cur_cpu_spec.cpu_user_features2 }; }

/* String used by ld.so to load implementation-specific optimized libraries. */
#[macro_export]
macro_rules! ELF_PLATFORM { () => { cur_cpu_spec.platform }; }
#[macro_export]
macro_rules! ELF_BASE_PLATFORM { () => { powerpc_base_platform }; }

// On 64-bit PowerPC, ELF_PLAT_INIT sets the TOC pointer in GPR 2.
#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! ELF_PLAT_INIT { ($r:expr, $load_addr:expr) => {{ (*$r).gpr[2] = $load_addr; }}; }

// SET_PERSONALITY and elf_read_implies_exec retain the architecture-dependent
// 64-bit/32-bit build condition from the original header.
#[cfg(target_pointer_width = "64")]
#[macro_export]
macro_rules! SET_PERSONALITY { ($ex:expr) => {{
    if (($ex).e_flags & 0x3) == 2 { set_thread_flag(TIF_ELF2ABI); }
    else { clear_thread_flag(TIF_ELF2ABI); }
    if ($ex).e_ident[EI_CLASS] == ELFCLASS32 { set_thread_flag(TIF_32BIT); }
    else { clear_thread_flag(TIF_32BIT); }
    if personality(current->personality) != PER_LINUX32 {
        set_personality(PER_LINUX | (current->personality & !PER_MASK));
    }
}}; }

#[inline]
pub fn elf_read_implies_exec(_ex: *const elf32_hdr, exec_stk: i32) -> i32 {
    if is_32bit_task() { (exec_stk == EXSTACK_DEFAULT) as i32 } else { 0 }
}

extern "C" {
    pub static mut dcache_bsize: i32;
    pub static mut icache_bsize: i32;
    pub static mut ucache_bsize: i32;
    pub fn arch_setup_additional_pages(bprm: *mut linux_binprm, uses_interp: i32) -> i32;
    pub fn relocate(final_address: c_ulong);
}

pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: bool = true;

#[macro_export]
macro_rules! VDSO_AUX_ENT { ($a:expr, $b:expr) => { NEW_AUX_ENT!($a, $b) }; }

#[inline]
pub fn stack_rnd_mask() -> usize {
    if is_32bit_task() { 0x7ffusize >> (PAGE_SHIFT - 12) }
    else { 0x3ffffusize >> (PAGE_SHIFT - 12) }
}

#[cfg(feature = "CONFIG_SPU_BASE")]
pub const NT_SPU: i32 = 1;

#[cfg(feature = "CONFIG_PPC64")]
#[inline]
pub unsafe fn get_cache_geometry(level: CacheLevel) -> u64 {
    (ppc64_caches.level.assoc << 16) | ppc64_caches.level.line_size
}

// COMMON_ARCH_DLINFO, ARCH_DLINFO, and COMPAT_ARCH_DLINFO are statement-like
// C macros; their NEW_AUX_ENT sequences are preserved as Rust macros.
#[macro_export]
macro_rules! COMMON_ARCH_DLINFO { () => {{
    NEW_AUX_ENT!(AT_IGNOREPPC, AT_IGNOREPPC);
    NEW_AUX_ENT!(AT_IGNOREPPC, AT_IGNOREPPC);
    NEW_AUX_ENT!(AT_DCACHEBSIZE, dcache_bsize);
    NEW_AUX_ENT!(AT_ICACHEBSIZE, icache_bsize);
    NEW_AUX_ENT!(AT_UCACHEBSIZE, 0);
    VDSO_AUX_ENT!(AT_SYSINFO_EHDR, current.mm.context.vdso as c_ulong);
    ARCH_DLINFO_CACHE_GEOMETRY!();
}}; }

#[macro_export]
macro_rules! ARCH_DLINFO { () => {{ COMMON_ARCH_DLINFO!(); NEW_AUX_ENT!(AT_MINSIGSTKSZ, get_min_sigframe_size()); }}; }
#[macro_export]
macro_rules! COMPAT_ARCH_DLINFO { () => {{ COMMON_ARCH_DLINFO!(); NEW_AUX_ENT!(AT_MINSIGSTKSZ, get_min_sigframe_size_compat()); }}; }

#[repr(C)]
pub struct func_desc {
    pub addr: c_ulong,
    pub toc: c_ulong,
    pub env: c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
