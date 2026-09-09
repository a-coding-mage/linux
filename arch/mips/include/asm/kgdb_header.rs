/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This header is intended for kernel builds.  The original C preprocessor
 * conditions on _MIPS_ISA and CONFIG_32BIT are preserved here as comments;
 * select the corresponding constants through the build configuration.
 */

#[cfg(any(
    feature = "mips1",
    feature = "mips2",
    feature = "mips32",
    feature = "mips3-32",
    feature = "mips4-32",
    feature = "mips64-32",
))]
pub const KGDB_GDB_REG_SIZE: usize = 32;

#[cfg(any(
    feature = "mips1",
    feature = "mips2",
    feature = "mips32",
    feature = "mips3-32",
    feature = "mips4-32",
    feature = "mips64-32",
))]
pub const GDB_SIZEOF_REG: usize = core::mem::size_of::<u32>();

#[cfg(any(
    feature = "mips3",
    feature = "mips4",
    feature = "mips64",
))]
pub const KGDB_GDB_REG_SIZE: usize = 64;

#[cfg(any(
    feature = "mips3",
    feature = "mips4",
    feature = "mips64",
))]
pub const GDB_SIZEOF_REG: usize = core::mem::size_of::<u64>();

pub const BUFMAX: usize = 2048;
pub const DBG_MAX_REG_NUM: usize = 72;
/* The C expression is sizeof(GDB_SIZEOF_REG), i.e. sizeof(size_t). */
pub const NUMREGBYTES: usize =
    DBG_MAX_REG_NUM * core::mem::size_of::<usize>();
pub const NUMCRITREGBYTES: usize = 12 * core::mem::size_of::<usize>();
pub const BREAK_INSTR_SIZE: usize = 4;
pub const CACHE_FLUSH_IS_SAFE: usize = 0;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn arch_kgdb_breakpoint();
    pub static mut saved_vectors: [*mut core::ffi::c_void; 32];
    pub fn handle_exception(regs: *mut pt_regs);
    pub fn breakinst();
    pub fn kgdb_ll_trap(
        cmd: core::ffi::c_int,
        str_: *const core::ffi::c_char,
        regs: *mut pt_regs,
        err: core::ffi::c_long,
        trap: core::ffi::c_int,
        sig: core::ffi::c_int,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
