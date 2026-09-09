// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2009-2011 Samsung Electronics Co., Ltd.
//		http://www.samsung.com
//
// Samsung CPU Support

// Linux dependencies supplied by the surrounding translation.

extern "C" {
    fn readl_relaxed(addr: usize) -> u32;
    fn writel_relaxed(value: u32, addr: usize);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

extern "C" {
    static mut S3C_VA_SYS: usize;
}

#[no_mangle]
pub static mut samsung_cpu_id: core::ffi::c_ulong = 0;

// __init
#[no_mangle]
pub unsafe extern "C" fn s3c64xx_init_cpu() {
    samsung_cpu_id = readl_relaxed(S3C_VA_SYS.wrapping_add(0x118)) as core::ffi::c_ulong;
    if samsung_cpu_id == 0 {
        /*
         * S3C6400 has the ID register in a different place,
         * and needs a write before it can be read.
         */
        writel_relaxed(0x0, S3C_VA_SYS.wrapping_add(0xA1C));
        samsung_cpu_id =
            readl_relaxed(S3C_VA_SYS.wrapping_add(0xA1C)) as core::ffi::c_ulong;
    }

    pr_info(
        b"Samsung CPU ID: 0x%08lx\n\0".as_ptr() as *const core::ffi::c_char,
        samsung_cpu_id,
    );
    pr_err(
        b"The platform is deprecated and scheduled for removal. Please reach to the maintainers of the platform and linux-samsung-soc@vger.kernel.org if you still use it.  Without such feedback, the platform will be removed after 2022.\n\0"
            .as_ptr() as *const core::ffi::c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
