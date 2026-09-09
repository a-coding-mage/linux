/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

/* ELF header e_flags defines */
pub const EF_SH_PIC: u32 = 0x100;
pub const EF_SH_FDPIC: u32 = 0x8000;

/* SH (particularly SHcompact) relocation types */
pub const R_SH_NONE: u32 = 0;
pub const R_SH_DIR32: u32 = 1;
pub const R_SH_REL32: u32 = 2;
pub const R_SH_DIR8WPN: u32 = 3;
pub const R_SH_IND12W: u32 = 4;
pub const R_SH_DIR8WPL: u32 = 5;
pub const R_SH_DIR8WPZ: u32 = 6;
pub const R_SH_DIR8BP: u32 = 7;
pub const R_SH_DIR8W: u32 = 8;
pub const R_SH_DIR8L: u32 = 9;
pub const R_SH_SWITCH16: u32 = 25;
pub const R_SH_SWITCH32: u32 = 26;
pub const R_SH_USES: u32 = 27;
pub const R_SH_COUNT: u32 = 28;
pub const R_SH_ALIGN: u32 = 29;
pub const R_SH_CODE: u32 = 30;
pub const R_SH_DATA: u32 = 31;
pub const R_SH_LABEL: u32 = 32;
pub const R_SH_SWITCH8: u32 = 33;
pub const R_SH_GNU_VTINHERIT: u32 = 34;
pub const R_SH_GNU_VTENTRY: u32 = 35;
pub const R_SH_TLS_GD_32: u32 = 144;
pub const R_SH_TLS_LD_32: u32 = 145;
pub const R_SH_TLS_LDO_32: u32 = 146;
pub const R_SH_TLS_IE_32: u32 = 147;
pub const R_SH_TLS_LE_32: u32 = 148;
pub const R_SH_TLS_DTPMOD32: u32 = 149;
pub const R_SH_TLS_DTPOFF32: u32 = 150;
pub const R_SH_TLS_TPOFF32: u32 = 151;
pub const R_SH_GOT32: u32 = 160;
pub const R_SH_PLT32: u32 = 161;
pub const R_SH_COPY: u32 = 162;
pub const R_SH_GLOB_DAT: u32 = 163;
pub const R_SH_JMP_SLOT: u32 = 164;
pub const R_SH_RELATIVE: u32 = 165;
pub const R_SH_GOTOFF: u32 = 166;
pub const R_SH_GOTPC: u32 = 167;

/* FDPIC relocs */
pub const R_SH_GOT20: u32 = 201;
pub const R_SH_GOTOFF20: u32 = 202;
pub const R_SH_GOTFUNCDESC: u32 = 203;
pub const R_SH_GOTFUNCDESC20: u32 = 204;
pub const R_SH_GOTOFFFUNCDESC: u32 = 205;
pub const R_SH_GOTOFFFUNCDESC20: u32 = 206;
pub const R_SH_FUNCDESC: u32 = 207;
pub const R_SH_FUNCDESC_VALUE: u32 = 208;

/* SHmedia relocs */
pub const R_SH_IMM_LOW16: u32 = 246;
pub const R_SH_IMM_LOW16_PCREL: u32 = 247;
pub const R_SH_IMM_MEDLOW16: u32 = 248;
pub const R_SH_IMM_MEDLOW16_PCREL: u32 = 249;
/* Keep this the last entry. */
pub const R_SH_NUM: u32 = 256;

/* ELF register definitions. */
pub type elf_greg_t = libc::c_ulong;
pub const ELF_NGREG: usize = core::mem::size_of::<crate::pt_regs>() / core::mem::size_of::<elf_greg_t>();
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpregset_t = crate::user_fpu_struct;

pub const ELF_CLASS: u32 = crate::ELFCLASS32;
#[cfg(target_endian = "little")]
pub const ELF_DATA: u32 = crate::ELFDATA2LSB;
#[cfg(not(target_endian = "little"))]
pub const ELF_DATA: u32 = crate::ELFDATA2MSB;
pub const ELF_ARCH: u32 = crate::EM_SH;

/* Build-time configuration and kernel macros are preserved as Rust macros. */
#[macro_export]
macro_rules! elf_check_arch { ($x:expr) => { (($x).e_machine == $crate::EM_SH) }; }
#[macro_export]
macro_rules! elf_check_fdpic { ($x:expr) => { (($x).e_flags & $crate::EF_SH_FDPIC) }; }
#[macro_export]
macro_rules! elf_check_const_displacement { ($x:expr) => { (($x).e_flags & $crate::EF_SH_PIC) }; }

pub const ELF_FDPIC_CORE_EFLAGS: u32 = EF_SH_FDPIC;
pub const ELF_EXEC_PAGESIZE: usize = crate::PAGE_SIZE;
pub const ELF_ET_DYN_BASE: usize = 2 * crate::TASK_SIZE / 3;

pub const CORE_DUMP_USE_REGSET: bool = true;

pub unsafe fn elf_core_copy_regs<T>(dest: *mut T, regs: *const crate::pt_regs) {
    core::ptr::copy_nonoverlapping(regs as *const u8, dest as *mut u8, core::mem::size_of::<crate::pt_regs>());
}

#[macro_export]
macro_rules! elf_plat_init {
    ($r:expr, $load_addr:expr) => {{
        $(let _ = &$load_addr;)?
        for i in 0..15 { $r.regs[i] = 0; }
        $r.sr = $crate::SR_FD;
    }};
}

#[macro_export]
macro_rules! elf_fdpic_plat_init {
    ($r:expr, $exec:expr, $interp:expr, $dynamic:expr) => {{
        for i in 0..8 { $r.regs[i] = 0; }
        $r.regs[8] = $exec; $r.regs[9] = $interp; $r.regs[10] = $dynamic;
        for i in 11..15 { $r.regs[i] = 0; }
        $r.sr = $crate::SR_FD;
    }};
}

#[macro_export]
macro_rules! set_personality {
    ($ex:expr) => { $crate::set_personality($crate::PER_LINUX_32BIT | ($crate::current_personality() & !$crate::PER_MASK)) };
}

#[cfg(feature = "CONFIG_VSYSCALL")]
pub const ARCH_HAS_SETUP_ADDITIONAL_PAGES: bool = true;

#[cfg(feature = "CONFIG_VSYSCALL")]
#[macro_export]
macro_rules! vsyscall_aux_ent {
    () => { if $crate::vdso_enabled != 0 { $crate::new_aux_ent($crate::AT_SYSINFO_EHDR, $crate::VDSO_BASE); } else { $crate::new_aux_ent($crate::AT_IGNORE, 0); } };
}
#[cfg(not(feature = "CONFIG_VSYSCALL"))]
#[macro_export]
macro_rules! vsyscall_aux_ent { () => { $crate::new_aux_ent($crate::AT_IGNORE, 0); }; }

#[cfg(feature = "CONFIG_SH_FPU")]
#[macro_export]
macro_rules! fpu_aux_ent { () => { $crate::new_aux_ent($crate::AT_FPUCW, $crate::FPSCR_INIT); }; }
#[cfg(not(feature = "CONFIG_SH_FPU"))]
#[macro_export]
macro_rules! fpu_aux_ent { () => { $crate::new_aux_ent($crate::AT_IGNORE, 0); }; }

/* ARCH_DLINFO emits the optional FPU/vsyscall entries and cache descriptors. */
#[macro_export]
macro_rules! arch_dlinfo {
    () => {{
        $crate::fpu_aux_ent!();
        $crate::vsyscall_aux_ent!();
        $crate::new_aux_ent($crate::AT_L1I_CACHESHAPE, $crate::l1i_cache_shape);
        $crate::new_aux_ent($crate::AT_L1D_CACHESHAPE, $crate::l1d_cache_shape);
        $crate::new_aux_ent($crate::AT_L2_CACHESHAPE, $crate::l2_cache_shape);
    }};
}

extern "C" {
    pub fn arch_setup_additional_pages(bprm: *mut crate::linux_binprm, uses_interp: libc::c_int) -> libc::c_int;
    pub static mut vdso_enabled: libc::c_uint;
    pub static mut __kernel_vsyscall: (); 
    pub static mut l1i_cache_shape: libc::c_int;
    pub static mut l1d_cache_shape: libc::c_int;
    pub static mut l2_cache_shape: libc::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
