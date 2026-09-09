/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014 Imagination Technologies Ltd.
 */

/* Translated from asm-eva.h. C preprocessor configuration is preserved below. */

/* Kernel variants, for C code. */
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_cache { ($op:expr, $base:expr) => { concat!("cache ", $op, ", ", $base, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_pref { ($hint:expr, $base:expr) => { concat!("pref ", $hint, ", ", $base, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_ll { ($reg:expr, $addr:expr) => { concat!("ll ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_sc { ($reg:expr, $addr:expr) => { concat!("sc ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_lw { ($reg:expr, $addr:expr) => { concat!("lw ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_lwl { ($reg:expr, $addr:expr) => { concat!("lwl ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_lwr { ($reg:expr, $addr:expr) => { concat!("lwr ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_lh { ($reg:expr, $addr:expr) => { concat!("lh ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_lb { ($reg:expr, $addr:expr) => { concat!("lb ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_lbu { ($reg:expr, $addr:expr) => { concat!("lbu ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_sw { ($reg:expr, $addr:expr) => { concat!("sw ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_swl { ($reg:expr, $addr:expr) => { concat!("swl ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_swr { ($reg:expr, $addr:expr) => { concat!("swr ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_sh { ($reg:expr, $addr:expr) => { concat!("sh ", $reg, ", ", $addr, "\n") }; }
#[cfg(not(__ASSEMBLER__))]
macro_rules! kernel_sb { ($reg:expr, $addr:expr) => { concat!("sb ", $reg, ", ", $addr, "\n") }; }

/* CONFIG_32BIT selects the source's user_sw/user_lw fallback for sd/ld. */
#[cfg(all(not(__ASSEMBLER__), CONFIG_32BIT))]
macro_rules! kernel_sd { ($reg:expr, $addr:expr) => { user_sw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_32BIT))]
macro_rules! kernel_ld { ($reg:expr, $addr:expr) => { user_lw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_32BIT)))]
macro_rules! kernel_sd { ($reg:expr, $addr:expr) => { concat!("sd ", $reg, ", ", $addr, "\n") }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_32BIT)))]
macro_rules! kernel_ld { ($reg:expr, $addr:expr) => { concat!("ld ", $reg, ", ", $addr, "\n") }; }

/* CONFIG_EVA controls the EVA instruction spellings. */
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! __BUILD_EVA_INSN { ($insn:expr, $reg:expr, $addr:expr) => { concat!("\t.set\tpush\n\t.set\tmips0\n\t.set\teva\n\t", $insn, $reg, ", ", $addr, "\n\t.set\tpop\n") }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_cache { ($op:expr, $base:expr) => { __BUILD_EVA_INSN!("cachee", $op, $base) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_pref { ($hint:expr, $base:expr) => { __BUILD_EVA_INSN!("prefe", $hint, $base) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_ll { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lle", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_sc { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("sce", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_lw { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lwe", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_lwl { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lwle", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_lwr { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lwre", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_lh { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lhe", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_lb { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lbe", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_lbu { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("lbue", $reg, $addr) }; }
/* No 64-bit EVA instruction for loading double words. */
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_ld { ($reg:expr, $addr:expr) => { user_lw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_sw { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("swe", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_swl { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("swle", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_swr { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("swre", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_sh { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("she", $reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_sb { ($reg:expr, $addr:expr) => { __BUILD_EVA_INSN!("sbe", $reg, $addr) }; }
/* No 64-bit EVA instruction for storing double words. */
#[cfg(all(not(__ASSEMBLER__), CONFIG_EVA))]
macro_rules! user_sd { ($reg:expr, $addr:expr) => { user_sw!($reg, $addr) }; }

#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_cache { ($op:expr, $base:expr) => { kernel_cache!($op, $base) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_pref { ($hint:expr, $base:expr) => { kernel_pref!($hint, $base) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_ll { ($reg:expr, $addr:expr) => { kernel_ll!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_sc { ($reg:expr, $addr:expr) => { kernel_sc!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_lw { ($reg:expr, $addr:expr) => { kernel_lw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_lwl { ($reg:expr, $addr:expr) => { kernel_lwl!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_lwr { ($reg:expr, $addr:expr) => { kernel_lwr!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_lh { ($reg:expr, $addr:expr) => { kernel_lh!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_lb { ($reg:expr, $addr:expr) => { kernel_lb!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_lbu { ($reg:expr, $addr:expr) => { kernel_lbu!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_sw { ($reg:expr, $addr:expr) => { kernel_sw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_swl { ($reg:expr, $addr:expr) => { kernel_swl!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_swr { ($reg:expr, $addr:expr) => { kernel_swr!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_sh { ($reg:expr, $addr:expr) => { kernel_sh!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA)))]
macro_rules! user_sb { ($reg:expr, $addr:expr) => { kernel_sb!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA), CONFIG_32BIT))]
macro_rules! user_sd { ($reg:expr, $addr:expr) => { kernel_sw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA), CONFIG_32BIT))]
macro_rules! user_ld { ($reg:expr, $addr:expr) => { kernel_lw!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA), not(CONFIG_32BIT)))]
macro_rules! user_sd { ($reg:expr, $addr:expr) => { kernel_sd!($reg, $addr) }; }
#[cfg(all(not(__ASSEMBLER__), not(CONFIG_EVA), not(CONFIG_32BIT)))]
macro_rules! user_ld { ($reg:expr, $addr:expr) => { kernel_ld!($reg, $addr) }; }

/* The __ASSEMBLER__ branch consists of assembler-token macros; retained as
 * declarative equivalents so the original conditional interface is visible. */
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_cache { ($op:tt, $base:tt) => { cache $op, $base }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_pref { ($hint:tt, $base:tt) => { pref $hint, $base }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_ll { ($reg:tt, $addr:tt) => { ll $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_sc { ($reg:tt, $addr:tt) => { sc $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_lw { ($reg:tt, $addr:tt) => { lw $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_lwl { ($reg:tt, $addr:tt) => { lwl $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_lwr { ($reg:tt, $addr:tt) => { lwr $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_lh { ($reg:tt, $addr:tt) => { lh $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_lb { ($reg:tt, $addr:tt) => { lb $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_lbu { ($reg:tt, $addr:tt) => { lbu $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_sw { ($reg:tt, $addr:tt) => { sw $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_swl { ($reg:tt, $addr:tt) => { swl $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_swr { ($reg:tt, $addr:tt) => { swr $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_sh { ($reg:tt, $addr:tt) => { sh $reg, $addr }; }
#[cfg(__ASSEMBLER__)]
macro_rules! kernel_sb { ($reg:tt, $addr:tt) => { sb $reg, $addr }; }

/* CONFIG_32BIT and CONFIG_EVA assembler variants intentionally remain
 * conditional in the source; their token-level forms are represented above
 * and the corresponding aliases follow the same mappings as C. */
#[cfg(all(__ASSEMBLER__, CONFIG_32BIT))]
macro_rules! kernel_sd { ($reg:tt, $addr:tt) => { user_sw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_32BIT))]
macro_rules! kernel_ld { ($reg:tt, $addr:tt) => { user_lw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_32BIT)))]
macro_rules! kernel_sd { ($reg:tt, $addr:tt) => { sd $reg, $addr }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_32BIT)))]
macro_rules! kernel_ld { ($reg:tt, $addr:tt) => { ld $reg, $addr }; }

#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! __BUILD_EVA_INSN { ($insn:tt, $reg:tt, $addr:tt) => { .set push; .set mips0; .set eva; $insn $reg, $addr; .set pop; }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_cache { ($op:tt, $base:tt) => { __BUILD_EVA_INSN!(cachee, $op, $base) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_pref { ($hint:tt, $base:tt) => { __BUILD_EVA_INSN!(prefe, $hint, $base) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_ll { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lle, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_sc { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(sce, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_lw { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lwe, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_lwl { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lwle, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_lwr { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lwre, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_lh { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lhe, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_lb { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lbe, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_lbu { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(lbue, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_ld { ($reg:tt, $addr:tt) => { user_lw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_sw { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(swe, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_swl { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(swle, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_swr { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(swre, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_sh { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(she, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_sb { ($reg:tt, $addr:tt) => { __BUILD_EVA_INSN!(sbe, $reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, CONFIG_EVA))]
macro_rules! user_sd { ($reg:tt, $addr:tt) => { user_sw!($reg, $addr) }; }

#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_cache { ($op:tt, $base:tt) => { kernel_cache!($op, $base) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_pref { ($hint:tt, $base:tt) => { kernel_pref!($hint, $base) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_ll { ($reg:tt, $addr:tt) => { kernel_ll!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_sc { ($reg:tt, $addr:tt) => { kernel_sc!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_lw { ($reg:tt, $addr:tt) => { kernel_lw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_lwl { ($reg:tt, $addr:tt) => { kernel_lwl!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_lwr { ($reg:tt, $addr:tt) => { kernel_lwr!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_lh { ($reg:tt, $addr:tt) => { kernel_lh!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_lb { ($reg:tt, $addr:tt) => { kernel_lb!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_lbu { ($reg:tt, $addr:tt) => { kernel_lbu!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_sw { ($reg:tt, $addr:tt) => { kernel_sw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_swl { ($reg:tt, $addr:tt) => { kernel_swl!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_swr { ($reg:tt, $addr:tt) => { kernel_swr!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_sh { ($reg:tt, $addr:tt) => { kernel_sh!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA)))]
macro_rules! user_sb { ($reg:tt, $addr:tt) => { kernel_sb!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA), CONFIG_32BIT))]
macro_rules! user_sd { ($reg:tt, $addr:tt) => { kernel_sw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA), CONFIG_32BIT))]
macro_rules! user_ld { ($reg:tt, $addr:tt) => { kernel_lw!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA), not(CONFIG_32BIT)))]
macro_rules! user_sd { ($reg:tt, $addr:tt) => { kernel_sd!($reg, $addr) }; }
#[cfg(all(__ASSEMBLER__, not(CONFIG_EVA), not(CONFIG_32BIT)))]
macro_rules! user_ld { ($reg:tt, $addr:tt) => { kernel_ld!($reg, $addr) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
