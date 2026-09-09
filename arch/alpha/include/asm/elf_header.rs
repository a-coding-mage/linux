/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Alpha kernel headers:
// asm/auxvec.h and asm/special_insns.h

/* Special values for the st_other field in the symbol table.  */
pub const STO_ALPHA_NOPV: u32 = 0x80;
pub const STO_ALPHA_STD_GPLOAD: u32 = 0x88;

/* Alpha ELF relocation types */
pub const R_ALPHA_NONE: u32 = 0; /* No reloc */
pub const R_ALPHA_REFLONG: u32 = 1; /* Direct 32 bit */
pub const R_ALPHA_REFQUAD: u32 = 2; /* Direct 64 bit */
pub const R_ALPHA_GPREL32: u32 = 3; /* GP relative 32 bit */
pub const R_ALPHA_LITERAL: u32 = 4; /* GP relative 16 bit w/optimization */
pub const R_ALPHA_LITUSE: u32 = 5; /* Optimization hint for LITERAL */
pub const R_ALPHA_GPDISP: u32 = 6; /* Add displacement to GP */
pub const R_ALPHA_BRADDR: u32 = 7; /* PC+4 relative 23 bit shifted */
pub const R_ALPHA_HINT: u32 = 8; /* PC+4 relative 16 bit shifted */
pub const R_ALPHA_SREL16: u32 = 9; /* PC relative 16 bit */
pub const R_ALPHA_SREL32: u32 = 10; /* PC relative 32 bit */
pub const R_ALPHA_SREL64: u32 = 11; /* PC relative 64 bit */
pub const R_ALPHA_GPRELHIGH: u32 = 17; /* GP relative 32 bit, high 16 bits */
pub const R_ALPHA_GPRELLOW: u32 = 18; /* GP relative 32 bit, low 16 bits */
pub const R_ALPHA_GPREL16: u32 = 19; /* GP relative 16 bit */
pub const R_ALPHA_COPY: u32 = 24; /* Copy symbol at runtime */
pub const R_ALPHA_GLOB_DAT: u32 = 25; /* Create GOT entry */
pub const R_ALPHA_JMP_SLOT: u32 = 26; /* Create PLT entry */
pub const R_ALPHA_RELATIVE: u32 = 27; /* Adjust by program base */
pub const R_ALPHA_BRSGP: u32 = 28;
pub const R_ALPHA_TLSGD: u32 = 29;
pub const R_ALPHA_TLS_LDM: u32 = 30;
pub const R_ALPHA_DTPMOD64: u32 = 31;
pub const R_ALPHA_GOTDTPREL: u32 = 32;
pub const R_ALPHA_DTPREL64: u32 = 33;
pub const R_ALPHA_DTPRELHI: u32 = 34;
pub const R_ALPHA_DTPRELLO: u32 = 35;
pub const R_ALPHA_DTPREL16: u32 = 36;
pub const R_ALPHA_GOTTPREL: u32 = 37;
pub const R_ALPHA_TPREL64: u32 = 38;
pub const R_ALPHA_TPRELHI: u32 = 39;
pub const R_ALPHA_TPRELLO: u32 = 40;
pub const R_ALPHA_TPREL16: u32 = 41;

pub const SHF_ALPHA_GPREL: u32 = 0x10000000;

/* Legal values for e_flags field of Elf64_Ehdr.  */
pub const EF_ALPHA_32BIT: u32 = 1; /* All addresses are below 2GB */

pub const CORE_DUMP_USE_REGSET: u32 = 1;

/* ELF register definitions.. */

/*
 * The OSF/1 version of <sys/procfs.h> makes gregset_t 46 entries long.
 * I have no idea why that is so.  For now, we just leave it at 33
 * (32 general regs + processor status word).
 */
pub const ELF_NGREG: usize = 33;
pub const ELF_NFPREG: usize = 32;

pub type elf_greg_t = ::std::os::raw::c_ulong;
pub type elf_gregset_t = [elf_greg_t; ELF_NGREG];
pub type elf_fpreg_t = f64;
pub type elf_fpregset_t = [elf_fpreg_t; ELF_NFPREG];

/* This is used to ensure we don't load something for the wrong architecture. */
#[macro_export]
macro_rules! elf_check_arch {
    ($x:expr) => {
        (($x).e_machine == EM_ALPHA) && !(($x).e_flags & EF_ALPHA_32BIT != 0)
    };
}

/* These are used to set parameters in the core dumps. */
pub const ELF_CLASS: u32 = ELFCLASS64;
pub const ELF_DATA: u32 = ELFDATA2LSB;
pub const ELF_ARCH: u32 = EM_ALPHA;
pub const ELF_EXEC_PAGESIZE: usize = 8192;

/* This is the location that an ET_DYN program is loaded if exec'ed. */
pub const ELF_ET_DYN_BASE: usize = TASK_UNMAPPED_BASE + 0x1000000;

/* $0 is set by ld.so to a pointer to a function which might be registered using atexit. */
#[macro_export]
macro_rules! ELF_PLAT_INIT {
    ($r:expr, $load_addr:expr) => {
        $r.r0 = 0
    };
}

#[repr(C)]
pub struct pt_regs;
#[repr(C)]
pub struct thread_info;
#[repr(C)]
pub struct task_struct;

extern "C" {
    pub fn dump_elf_thread(dest: *mut elf_greg_t, pt: *mut pt_regs, ti: *mut thread_info);
    pub fn dump_elf_task(dest: elf_greg_t, task: *mut task_struct) -> ::std::os::raw::c_int;
    pub static mut alpha_l1i_cacheshape: ::std::os::raw::c_int;
    pub static mut alpha_l1d_cacheshape: ::std::os::raw::c_int;
    pub static mut alpha_l2_cacheshape: ::std::os::raw::c_int;
    pub static mut alpha_l3_cacheshape: ::std::os::raw::c_int;
}

#[macro_export]
macro_rules! ELF_CORE_COPY_REGS {
    ($dest:expr, $regs:expr) => {
        dump_elf_thread($dest, $regs, current_thread_info())
    };
}

#[macro_export]
macro_rules! ELF_CORE_COPY_TASK_REGS {
    ($task:expr, $dest:expr) => {
        dump_elf_task(*($dest), $task)
    };
}

/* This yields a mask that user programs can use to figure out what instruction set this CPU supports. */
#[macro_export]
macro_rules! ELF_HWCAP {
    () => { (!amask(-1)) };
}

/* This yields a string that ld.so will use to load implementation specific libraries for optimization. */
#[macro_export]
macro_rules! ELF_PLATFORM {
    () => {{
        let i_ = implver();
        if i_ == IMPLVER_EV5 { "ev56" } else if amask(AMASK_CIX) != 0 { "ev6" } else { "ev67" }
    }};
}

/* update AT_VECTOR_SIZE_ARCH if the number of NEW_AUX_ENT entries changes */
#[macro_export]
macro_rules! ARCH_DLINFO {
    () => {{
        NEW_AUX_ENT(AT_L1I_CACHESHAPE, alpha_l1i_cacheshape);
        NEW_AUX_ENT(AT_L1D_CACHESHAPE, alpha_l1d_cacheshape);
        NEW_AUX_ENT(AT_L2_CACHESHAPE, alpha_l2_cacheshape);
        NEW_AUX_ENT(AT_L3_CACHESHAPE, alpha_l3_cacheshape);
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
