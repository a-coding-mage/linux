/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 1997, 1999 Jakub Jelinek (jj@ultra.linux.cz)
 * Copyright (C) 2006 David S. Miller <davem@davemloft.net>
 */

// C condition: defined(__sparc__) && defined(__arch64__)
#[cfg(all(target_arch = "sparc", target_pointer_width = "64"))]
mod sparc64 {
    extern "C" {
        pub static mut xor_block_VIS: xor_block_template;
        pub static mut xor_block_niagara: xor_block_template;

        pub static mut tlb_type: i32;
        pub static mut sun4v_chip_type: i32;

        fn xor_force(template: *mut xor_block_template);
    }

    // Supplied by the surrounding kernel translation.
    pub enum xor_block_template {}
    // `hypervisor` and the `SUN4V_CHIP_NIAGARA*` constants are supplied by
    // the surrounding kernel translation.

    #[inline(always)]
    pub unsafe fn arch_xor_init() {
        /* Force VIS for everything except Niagara. */
        if tlb_type == hypervisor
            && (sun4v_chip_type == SUN4V_CHIP_NIAGARA1
                || sun4v_chip_type == SUN4V_CHIP_NIAGARA2
                || sun4v_chip_type == SUN4V_CHIP_NIAGARA3
                || sun4v_chip_type == SUN4V_CHIP_NIAGARA4
                || sun4v_chip_type == SUN4V_CHIP_NIAGARA5)
        {
            xor_force(&raw mut xor_block_niagara);
        } else {
            xor_force(&raw mut xor_block_VIS);
        }
    }
}

// C else branch: !sparc64
#[cfg(not(all(target_arch = "sparc", target_pointer_width = "64")))]
mod sparc32 {
    extern "C" {
        pub static mut xor_block_SPARC: xor_block_template;
        pub static mut xor_block_8regs: xor_block_template;
        pub static mut xor_block_32regs: xor_block_template;

        fn xor_register(template: *mut xor_block_template);
    }

    // Supplied by the surrounding kernel translation.
    pub enum xor_block_template {}

    #[inline(always)]
    pub unsafe fn arch_xor_init() {
        xor_register(&raw mut xor_block_8regs);
        xor_register(&raw mut xor_block_32regs);
        xor_register(&raw mut xor_block_SPARC);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
