/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2023 WANG Xuerui <git@xen0n.name>
 */

/* Dependency provided by asm/cpu-features.h. */

/*
 * For grins, also test the generic routines.
 *
 * More importantly: it cannot be ruled out at this point of time, that some
 * future (maybe reduced) models could run the vector algorithms slower than
 * the scalar ones, maybe for errata or micro-op reasons. It may be
 * appropriate to revisit this after one or two more uarch generations.
 */

extern "C" {
    pub static mut xor_block_lsx: xor_block_template;
    pub static mut xor_block_lasx: xor_block_template;

    pub static mut xor_block_8regs: xor_block_template;
    pub static mut xor_block_8regs_p: xor_block_template;
    pub static mut xor_block_32regs: xor_block_template;
    pub static mut xor_block_32regs_p: xor_block_template;

    pub static cpu_has_lsx: bool;
    pub static cpu_has_lasx: bool;

    pub fn xor_register(template: *const xor_block_template);
}

#[inline(always)]
pub unsafe fn arch_xor_init() {
    xor_register(&raw const xor_block_8regs);
    xor_register(&raw const xor_block_8regs_p);
    xor_register(&raw const xor_block_32regs);
    xor_register(&raw const xor_block_32regs_p);

    /* Preserves CONFIG_CPU_HAS_LSX conditional compilation intent. */
    #[cfg(feature = "CONFIG_CPU_HAS_LSX")]
    if cpu_has_lsx {
        xor_register(&raw const xor_block_lsx);
    }

    /* Preserves CONFIG_CPU_HAS_LASX conditional compilation intent. */
    #[cfg(feature = "CONFIG_CPU_HAS_LASX")]
    if cpu_has_lasx {
        xor_register(&raw const xor_block_lasx);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
