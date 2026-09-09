/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003, 2004 Ralf Baechle
 * Copyright (C) 2004  Maciej W. Rozycki
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/smp.h, linux/compiler.h

#[inline]
pub fn __get_cpu_type(cpu_type: i32) -> i32 {
    match cpu_type {
        #[cfg(any(CONFIG_SYS_HAS_CPU_LOONGSON2E, CONFIG_SYS_HAS_CPU_LOONGSON2F))]
        CPU_LOONGSON2EF => {}
        #[cfg(CONFIG_SYS_HAS_CPU_LOONGSON64)]
        CPU_LOONGSON64 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_LOONGSON32)]
        CPU_LOONGSON32 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_MIPS32_R1)]
        CPU_4KC | CPU_ALCHEMY | CPU_PR4450 => {}
        #[cfg(any(CONFIG_SYS_HAS_CPU_MIPS32_R1, CONFIG_SYS_HAS_CPU_MIPS32_R2))]
        CPU_4KEC | CPU_XBURST => {}
        #[cfg(CONFIG_SYS_HAS_CPU_MIPS32_R2)]
        CPU_4KSC | CPU_24K | CPU_34K | CPU_1004K | CPU_74K | CPU_1074K |
        CPU_M14KC | CPU_M14KEC | CPU_INTERAPTIV | CPU_PROAPTIV => {}
        #[cfg(CONFIG_SYS_HAS_CPU_MIPS32_R5)]
        CPU_M5150 | CPU_P5600 => {}
        #[cfg(any(
            CONFIG_SYS_HAS_CPU_MIPS32_R2,
            CONFIG_SYS_HAS_CPU_MIPS32_R5,
            CONFIG_SYS_HAS_CPU_MIPS32_R6,
            CONFIG_SYS_HAS_CPU_MIPS64_R2,
            CONFIG_SYS_HAS_CPU_MIPS64_R5,
            CONFIG_SYS_HAS_CPU_MIPS64_R6
        ))]
        CPU_QEMU_GENERIC => {}
        #[cfg(CONFIG_SYS_HAS_CPU_MIPS64_R1)]
        CPU_5KC | CPU_5KE | CPU_20KC | CPU_25KF | CPU_SB1 | CPU_SB1A => {}
        // All MIPS64 R2 processors have their own special symbols.  That is,
        // there currently is no pure R2 core.
        #[cfg(CONFIG_SYS_HAS_CPU_MIPS32_R6)]
        CPU_M6250 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_MIPS64_R6)]
        CPU_I6400 | CPU_I6500 | CPU_P6600 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_R3000)]
        CPU_R2000 | CPU_R3000 | CPU_R3000A | CPU_R3041 | CPU_R3051 |
        CPU_R3052 | CPU_R3081 | CPU_R3081E => {}
        #[cfg(CONFIG_SYS_HAS_CPU_R4300)]
        CPU_R4300 | CPU_R4310 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_R4X00)]
        CPU_R4000PC | CPU_R4000SC | CPU_R4000MC | CPU_R4200 | CPU_R4400PC |
        CPU_R4400SC | CPU_R4400MC | CPU_R4600 | CPU_R4700 | CPU_R4640 |
        CPU_R4650 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_TX49XX)]
        CPU_TX49XX => {}
        #[cfg(CONFIG_SYS_HAS_CPU_R5000)]
        CPU_R5000 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_R5500)]
        CPU_R5500 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_NEVADA)]
        CPU_NEVADA => {}
        #[cfg(CONFIG_SYS_HAS_CPU_R10000)]
        CPU_R10000 | CPU_R12000 | CPU_R14000 | CPU_R16000 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_RM7000)]
        CPU_RM7000 | CPU_SR71000 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_SB1)]
        CPU_SB1 | CPU_SB1A => {}
        #[cfg(CONFIG_SYS_HAS_CPU_CAVIUM_OCTEON)]
        CPU_CAVIUM_OCTEON | CPU_CAVIUM_OCTEON_PLUS | CPU_CAVIUM_OCTEON2 |
        CPU_CAVIUM_OCTEON3 => {}
        #[cfg(any(CONFIG_SYS_HAS_CPU_BMIPS32_3300, CONFIG_SYS_HAS_CPU_MIPS32_R1))]
        CPU_BMIPS32 | CPU_BMIPS3300 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_BMIPS4350)]
        CPU_BMIPS4350 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_BMIPS4380)]
        CPU_BMIPS4380 => {}
        #[cfg(CONFIG_SYS_HAS_CPU_BMIPS5000)]
        CPU_BMIPS5000 => {}
        _ => unreachable!(),
    }

    cpu_type
}

#[inline]
pub fn current_cpu_type() -> i32 {
    let cpu_type: i32 = current_cpu_data.cputype;
    __get_cpu_type(cpu_type)
}

#[inline]
pub fn boot_cpu_type() -> i32 {
    let cpu_type: i32 = cpu_data[0].cputype;
    __get_cpu_type(cpu_type)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
