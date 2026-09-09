// SPDX-License-Identifier: GPL-2.0
// C dependencies: linux/module.h, linux/string.h, linux/uaccess.h,
// linux/delay.h, linux/mm.h, asm/checksum.h, asm/sections.h

// EXPORT_SYMBOL declarations from the original source.
extern "C" {
    pub fn memchr();
    pub fn memcpy();
    pub fn memset();
    pub fn memmove();
    pub fn __copy_user();
    pub fn __udelay();
    pub fn __ndelay();
    pub fn __const_udelay();
    pub fn strlen();
    pub fn csum_partial();
    pub fn csum_partial_copy_generic();
    pub fn copy_page();
    pub fn __clear_user();

    // CONFIG_FLATMEM: needed by the pfn_valid macro.
    #[cfg(CONFIG_FLATMEM)]
    pub static mut min_low_pfn: usize;
    #[cfg(CONFIG_FLATMEM)]
    pub static mut max_low_pfn: usize;

    pub fn __udivsi3();
    pub fn __sdivsi3();
    pub fn __lshrsi3();
    pub fn __ashrsi3();
    pub fn __ashlsi3();
    pub fn __lshrsi3_r0();
    pub fn __ashrsi3_r0();
    pub fn __ashlsi3_r0();

    pub fn __ashiftrt_r4_0();
    pub fn __ashiftrt_r4_1();
    pub fn __ashiftrt_r4_2();
    pub fn __ashiftrt_r4_3();
    pub fn __ashiftrt_r4_4();
    pub fn __ashiftrt_r4_5();
    pub fn __ashiftrt_r4_6();
    pub fn __ashiftrt_r4_7();
    pub fn __ashiftrt_r4_8();
    pub fn __ashiftrt_r4_9();
    pub fn __ashiftrt_r4_10();
    pub fn __ashiftrt_r4_11();
    pub fn __ashiftrt_r4_12();
    pub fn __ashiftrt_r4_13();
    pub fn __ashiftrt_r4_14();
    pub fn __ashiftrt_r4_15();
    pub fn __ashiftrt_r4_16();
    pub fn __ashiftrt_r4_17();
    pub fn __ashiftrt_r4_18();
    pub fn __ashiftrt_r4_19();
    pub fn __ashiftrt_r4_20();
    pub fn __ashiftrt_r4_21();
    pub fn __ashiftrt_r4_22();
    pub fn __ashiftrt_r4_23();
    pub fn __ashiftrt_r4_24();
    pub fn __ashiftrt_r4_25();
    pub fn __ashiftrt_r4_26();
    pub fn __ashiftrt_r4_27();
    pub fn __ashiftrt_r4_28();
    pub fn __ashiftrt_r4_29();
    pub fn __ashiftrt_r4_30();
    pub fn __ashiftrt_r4_31();
    pub fn __ashiftrt_r4_32();
    pub fn __movstr();
    pub fn __movstrSI8();
    pub fn __movstrSI12();
    pub fn __movstrSI16();
    pub fn __movstrSI20();
    pub fn __movstrSI24();
    pub fn __movstrSI28();
    pub fn __movstrSI32();
    pub fn __movstrSI36();
    pub fn __movstrSI40();
    pub fn __movstrSI44();
    pub fn __movstrSI48();
    pub fn __movstrSI52();
    pub fn __movstrSI56();
    pub fn __movstrSI60();
    pub fn __movstr_i4_even();
    pub fn __movstr_i4_odd();
    pub fn __movstrSI12_i4();
    pub fn __movmem();
    pub fn __movmemSI8();
    pub fn __movmemSI12();
    pub fn __movmemSI16();
    pub fn __movmemSI20();
    pub fn __movmemSI24();
    pub fn __movmemSI28();
    pub fn __movmemSI32();
    pub fn __movmemSI36();
    pub fn __movmemSI40();
    pub fn __movmemSI44();
    pub fn __movmemSI48();
    pub fn __movmemSI52();
    pub fn __movmemSI56();
    pub fn __movmemSI60();
    pub fn __movmem_i4_even();
    pub fn __movmem_i4_odd();
    pub fn __movmemSI12_i4();
    pub fn __udiv_qrnnd_16();
    pub fn __sdivsi3_i4();
    pub fn __udivsi3_i4();
    pub fn __sdivsi3_i4i();
    pub fn __udivsi3_i4i();

    #[cfg(CONFIG_MCOUNT)]
    pub fn mcount();
}

// Each declaration above corresponds to DECLARE_EXPORT(name), which declared
// an external void symbol and exported it with EXPORT_SYMBOL(name).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
