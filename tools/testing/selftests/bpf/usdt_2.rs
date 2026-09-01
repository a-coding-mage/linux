// SPDX-License-Identifier: GPL-2.0

#[cfg(target_arch = "x86_64")]
mod usdt_2_impl {
    /*
     * Include usdt.h with default nop,nop10 instructions combo.
     */
    // Dependency from "usdt.h": USDT! macro.

    // C source used __attribute__((aligned(16))) on this function.
    #[no_mangle]
    pub extern "C" fn usdt_2() {
        USDT!(optimized_attach, usdt_2);
    }

    static mut usdt_red_zone_arg1: ::core::ffi::c_ulong = 0xDEADBEEF;
    static mut usdt_red_zone_arg2: ::core::ffi::c_ulong = 0xCAFEBABE;
    static mut usdt_red_zone_arg3: ::core::ffi::c_ulong = 0xFEEDFACE;

    #[inline(never)]
    #[no_mangle]
    pub extern "C" fn usdt_red_zone_trigger() {
        let a1: ::core::ffi::c_ulong =
            unsafe { ::core::ptr::read_volatile(::core::ptr::addr_of!(usdt_red_zone_arg1)) };
        let a2: ::core::ffi::c_ulong =
            unsafe { ::core::ptr::read_volatile(::core::ptr::addr_of!(usdt_red_zone_arg2)) };
        let a3: ::core::ffi::c_ulong =
            unsafe { ::core::ptr::read_volatile(::core::ptr::addr_of!(usdt_red_zone_arg3)) };

        USDT!(optimized_attach, usdt_red_zone, a1, a2, a3);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
