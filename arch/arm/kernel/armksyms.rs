// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/armksyms.c
 *
 *  Copyright (C) 2000 Russell King
 */

// C headers and build-time configuration conditions are supplied by the
// surrounding kernel translation unit.

extern "C" {
    pub fn __ashldi3();
    pub fn __ashrdi3();
    pub fn __divsi3();
    pub fn __lshrdi3();
    pub fn __modsi3();
    pub fn __muldi3();
    pub fn __ucmpdi2();
    pub fn __udivsi3();
    pub fn __umodsi3();
    pub fn __do_div64();
    pub fn __bswapsi2();
    pub fn __bswapdi2();

    pub fn __aeabi_idiv();
    pub fn __aeabi_idivmod();
    pub fn __aeabi_lasr();
    pub fn __aeabi_llsl();
    pub fn __aeabi_llsr();
    pub fn __aeabi_lmul();
    pub fn __aeabi_uidiv();
    pub fn __aeabi_uidivmod();
    pub fn __aeabi_ulcmp();

    pub fn fpundefinstr();

    pub fn mmioset(_: *mut core::ffi::c_void, _: core::ffi::c_uint, _: usize);
    pub fn mmiocpy(
        _: *mut core::ffi::c_void,
        _: *const core::ffi::c_void,
        _: usize,
    );

    pub static mut arm_delay_ops: core::ffi::c_void;

    pub fn csum_partial();
    pub fn csum_partial_copy_from_user();
    pub fn csum_partial_copy_nocheck();
    pub fn __csum_ipv6_magic();

    pub fn __raw_readsb();
    pub fn __raw_readsw();
    pub fn __raw_readsl();
    pub fn __raw_writesb();
    pub fn __raw_writesw();
    pub fn __raw_writesl();

    pub fn strchr();
    pub fn strrchr();
    pub fn memset();
    pub fn __memset32();
    pub fn __memset64();
    pub fn memcpy();
    pub fn memmove();
    pub fn memchr();

    #[cfg(CONFIG_MMU)]
    pub fn copy_page();
    #[cfg(CONFIG_MMU)]
    pub fn arm_copy_from_user();
    #[cfg(CONFIG_MMU)]
    pub fn arm_copy_to_user();
    #[cfg(CONFIG_MMU)]
    pub fn arm_clear_user();
    #[cfg(CONFIG_MMU)]
    pub fn __get_user_1();
    #[cfg(CONFIG_MMU)]
    pub fn __get_user_2();
    #[cfg(CONFIG_MMU)]
    pub fn __get_user_4();
    #[cfg(CONFIG_MMU)]
    pub fn __get_user_8();
    // #ifdef __ARMEB__
    pub fn __get_user_64t_1();
    pub fn __get_user_64t_2();
    pub fn __get_user_64t_4();
    pub fn __get_user_32t_8();
    // #endif
    #[cfg(CONFIG_MMU)]
    pub fn __put_user_1();
    #[cfg(CONFIG_MMU)]
    pub fn __put_user_2();
    #[cfg(CONFIG_MMU)]
    pub fn __put_user_4();
    #[cfg(CONFIG_MMU)]
    pub fn __put_user_8();

    pub fn _set_bit();
    pub fn _test_and_set_bit();
    pub fn _clear_bit();
    pub fn _test_and_clear_bit();
    pub fn _change_bit();
    pub fn _test_and_change_bit();
    pub fn _find_first_zero_bit_le();
    pub fn _find_next_zero_bit_le();
    pub fn _find_first_bit_le();
    pub fn _find_next_bit_le();

    // #ifdef __ARMEB__
    pub fn _find_first_zero_bit_be();
    pub fn _find_next_zero_bit_be();
    pub fn _find_first_bit_be();
    pub fn _find_next_bit_be();
    // #endif

    pub fn __gnu_mcount_nc();
    pub fn __pv_phys_pfn_offset();
    pub fn __pv_offset();
    pub fn __arm_smccc_smc();
    pub fn __arm_smccc_hvc();
}

// EXPORT_SYMBOL declarations from the C source are preserved by the public
// declarations above; the kernel export mechanism is supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
