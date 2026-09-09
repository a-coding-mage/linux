/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the surrounding kernel translation: asm/cpu-info.h
// Dependency supplied by the surrounding kernel translation: loongson_regs.h

#[cfg(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION)]
pub const LOONGSON_FPREV_MASK: u32 = 0x7;

#[cfg(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION)]
extern "C" {
    pub fn loongson3_cpucfg_synthesize_data(c: *mut cpuinfo_mips);
}

#[cfg(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION)]
#[inline]
pub unsafe fn loongson3_cpucfg_emulation_enabled(c: *mut cpuinfo_mips) -> bool {
    /* All supported cores have non-zero LOONGSON_CFG1 data. */
    (*c).loongson3_cpucfg_data[0] != 0
}

#[cfg(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION)]
#[inline]
pub unsafe fn loongson3_cpucfg_read_synthesized(c: *mut cpuinfo_mips, sel: u64) -> u32 {
    match sel {
        LOONGSON_CFG0 => (*c).processor_id,
        LOONGSON_CFG1 | LOONGSON_CFG2 | LOONGSON_CFG3 => {
            (*c).loongson3_cpucfg_data[(sel - 1) as usize]
        }
        LOONGSON_CFG4 | LOONGSON_CFG5 => {
            /* CPUCFG selects 4 and 5 are related to the input clock
             * signal.
             *
             * Unimplemented for now.
             */
            0
        }
        LOONGSON_CFG6 => {
            /* CPUCFG select 6 is for the undocumented Safe Extension. */
            0
        }
        LOONGSON_CFG7 => {
            /* CPUCFG select 7 is for the virtualization extension.
             * We don't know if the two currently known features are
             * supported on older cores according to the public
             * documentation, so leave this at zero.
             */
            0
        }
        _ => {
            /*
             * Return 0 for unrecognized CPUCFG selects, which is real hardware
             * behavior observed on Loongson 3A R4.
             */
            0
        }
    }
}

#[cfg(not(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION))]
#[inline]
pub unsafe fn loongson3_cpucfg_synthesize_data(_c: *mut cpuinfo_mips) {}

#[cfg(not(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION))]
#[inline]
pub unsafe fn loongson3_cpucfg_emulation_enabled(_c: *mut cpuinfo_mips) -> bool {
    false
}

#[cfg(not(CONFIG_CPU_LOONGSON3_CPUCFG_EMULATION))]
#[inline]
pub unsafe fn loongson3_cpucfg_read_synthesized(_c: *mut cpuinfo_mips, _sel: u64) -> u32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
