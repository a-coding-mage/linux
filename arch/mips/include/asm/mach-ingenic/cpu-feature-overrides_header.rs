/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 */

// CPU feature overrides for the Ingenic JZ4740 architecture.
pub const CPU_HAS_TLB: i32 = 1;
pub const CPU_HAS_4KEX: i32 = 1;
pub const CPU_HAS_3K_CACHE: i32 = 0;
pub const CPU_HAS_4K_CACHE: i32 = 1;
pub const CPU_HAS_COUNTER: i32 = 0;
pub const CPU_HAS_WATCH: i32 = 1;
pub const CPU_HAS_DIVEC: i32 = 1;
pub const CPU_HAS_VCE: i32 = 0;
pub const CPU_HAS_CACHE_CDEX_P: i32 = 0;
pub const CPU_HAS_CACHE_CDEX_S: i32 = 0;
pub const CPU_HAS_PREFETCH: i32 = 1;
pub const CPU_HAS_MCHECK: i32 = 1;
pub const CPU_HAS_EJTAG: i32 = 1;
pub const CPU_HAS_LLSC: i32 = 1;
pub const CPU_HAS_MIPS16: i32 = 0;
pub const CPU_HAS_MIPS16E2: i32 = 0;
pub const CPU_HAS_MDMX: i32 = 0;
pub const CPU_HAS_MIPS3D: i32 = 0;
pub const CPU_HAS_SMARTMIPS: i32 = 0;
pub const KERNEL_USES_LLSC: i32 = 1;
pub const CPU_HAS_VTAG_ICACHE: i32 = 1;
pub const CPU_HAS_DC_ALIASES: i32 = 0;
pub const CPU_HAS_IC_FILLS_F_DC: i32 = 0;
pub const CPU_HAS_PINDEXED_DCACHE: i32 = 0;
pub const CPU_HAS_MIPS32R1: i32 = 1;
pub const CPU_HAS_MIPS64R1: i32 = 0;
pub const CPU_HAS_MIPS64R2: i32 = 0;
pub const CPU_HAS_DSP: i32 = 0;
pub const CPU_HAS_DSP2: i32 = 0;
pub const CPU_HAS_MIPSMT: i32 = 0;
pub const CPU_HAS_USERLOCAL: i32 = 0;
pub const CPU_HAS_NOFPUEX: i32 = 0;
pub const CPU_HAS_64BITS: i32 = 0;
pub const CPU_HAS_64BIT_ZERO_REG: i32 = 0;
pub const CPU_HAS_INCLUSIVE_PCACHES: i32 = 0;

#[inline]
pub const fn cpu_dcache_line_size() -> i32 {
    32
}

#[inline]
pub const fn cpu_icache_line_size() -> i32 {
    32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
