/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 2004 Chris Dearman
 * Copyright (C) 2005 Ralf Baechle (ralf@linux-mips.org)
 */

/*
 * CPU feature overrides for MIPS boards
 */
// The following conditional configurations correspond to the C preprocessor
// symbols CONFIG_CPU_MIPS32 and CONFIG_CPU_MIPS64.
#[cfg(feature = "CONFIG_CPU_MIPS32")]
mod config_cpu_mips32 {
    pub const cpu_has_tlb: i32 = 1;
    pub const cpu_has_4kex: i32 = 1;
    pub const cpu_has_4k_cache: i32 = 1;
    // #define cpu_has_fpu       ?
    // #define cpu_has_32fpr     ?
    pub const cpu_has_counter: i32 = 1;
    // #define cpu_has_watch     ?
    pub const cpu_has_divec: i32 = 1;
    pub const cpu_has_vce: i32 = 0;
    // #define cpu_has_cache_cdex_p ?
    // #define cpu_has_cache_cdex_s ?
    // #define cpu_has_prefetch   ?
    pub const cpu_has_mcheck: i32 = 1;
    // #define cpu_has_ejtag     ?
    pub const cpu_has_llsc: i32 = 1;
    // #define cpu_has_vtag_icache ?
    // #define cpu_has_dc_aliases ?
    // #define cpu_has_ic_fills_f_dc ?
    pub const cpu_has_clo_clz: i32 = 1;
    pub const cpu_has_nofpuex: i32 = 0;
    // #define cpu_has_64bits    ?
    // #define cpu_has_64bit_zero_reg ?
    // #define cpu_has_inclusive_pcaches ?
    pub const cpu_icache_snoops_remote_store: i32 = 1;
}

#[cfg(feature = "CONFIG_CPU_MIPS64")]
mod config_cpu_mips64 {
    pub const cpu_has_tlb: i32 = 1;
    pub const cpu_has_4kex: i32 = 1;
    pub const cpu_has_4k_cache: i32 = 1;
    // #define cpu_has_fpu       ?
    // #define cpu_has_32fpr     ?
    pub const cpu_has_counter: i32 = 1;
    // #define cpu_has_watch     ?
    pub const cpu_has_divec: i32 = 1;
    pub const cpu_has_vce: i32 = 0;
    // #define cpu_has_cache_cdex_p ?
    // #define cpu_has_cache_cdex_s ?
    // #define cpu_has_prefetch   ?
    pub const cpu_has_mcheck: i32 = 1;
    // #define cpu_has_ejtag     ?
    pub const cpu_has_llsc: i32 = 1;
    // #define cpu_has_vtag_icache ?
    // #define cpu_has_dc_aliases ?
    // #define cpu_has_ic_fills_f_dc ?
    pub const cpu_has_clo_clz: i32 = 1;
    pub const cpu_has_nofpuex: i32 = 0;
    // #define cpu_has_64bits    ?
    // #define cpu_has_64bit_zero_reg ?
    // #define cpu_has_inclusive_pcaches ?
    pub const cpu_icache_snoops_remote_store: i32 = 1;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
