/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Dependencies supplied by the surrounding kernel translation.

pub const EM_ARC_INUSE: u32 = if cfg!(feature = "CONFIG_ISA_ARCOMPACT") {
    EM_ARCOMPACT
} else {
    EM_ARCV2
};

/* ARC Relocations (kernel Modules only) */
pub const R_ARC_32: u32 = 0x4;
pub const R_ARC_32_ME: u32 = 0x1B;
pub const R_ARC_32_PCREL: u32 = 0x31;

/* to set parameters in the core dumps */
pub const ELF_ARCH: u32 = EM_ARC_INUSE;
pub const ELF_CLASS: u32 = ELFCLASS32;

#[cfg(feature = "CONFIG_CPU_BIG_ENDIAN")]
pub const ELF_DATA: u32 = ELFDATA2MSB;
#[cfg(not(feature = "CONFIG_CPU_BIG_ENDIAN"))]
pub const ELF_DATA: u32 = ELFDATA2LSB;

/*
 * To ensure that
 *  -we don't load something for the wrong architecture.
 *  -The userspace is using the correct syscall ABI
 */
#[repr(C)]
pub struct elf32_hdr {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn elf_check_arch(hdr: *const elf32_hdr) -> i32;
}

pub const CORE_DUMP_USE_REGSET: bool = true;

pub const ELF_EXEC_PAGESIZE: usize = PAGE_SIZE;

/*
 * This is the location that an ET_DYN program is loaded if exec'ed.  Typical
 * use of this is to invoke "./ld.so someprog" to test out a new version of
 * the loader.  We need to make sure that it is out of the way of the program
 * that it will "exec", and that there is sufficient room for the brk.
 */
pub const ELF_ET_DYN_BASE: usize = (2usize * TASK_SIZE) / 3;

/*
 * When the program starts, a1 contains a pointer to a function to be
 * registered with atexit, as per the SVR4 ABI.  A value of 0 means we
 * have no such handler.
 */
#[macro_export]
macro_rules! ELF_PLAT_INIT {
    ($r:expr, $load_addr:expr) => {{
        ($r).r0 = 0;
    }};
}

/*
 * This yields a mask that user programs can use to figure out what
 * instruction set this cpu supports.
 */
pub const ELF_HWCAP: u32 = 0;

/*
 * This yields a string that ld.so will use to load implementation
 * specific libraries for optimization.  This is more specific in
 * intent than poking at uname or /proc/cpuinfo.
 */
pub const ELF_PLATFORM: *const core::ffi::c_char = core::ptr::null();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
