// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/meta_access.c */

// C dependencies translated as external expectations:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;

unsafe extern "C" {
    static bpf_xdp_adjust_meta: usize;
}

macro_rules! bpf_asm {
    ($body:literal) => {
        unsafe {
            asm!($body, options(noreturn));
        }
    };
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test1\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn meta_access_test1() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r0 = r2;
         r0 += 8;
         if r0 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test2\")"]
#[doc = "__failure __msg(\"R0 min value is negative\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test2() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r0 = r2;
         r0 -= 8;
         r4 = r2;
         r4 += 8;
         if r4 > r3 goto l0_%=;
         r0 = *(u8*)(r0 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test3\")"]
#[doc = "__failure __msg(\"invalid access to packet\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test3() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data_end});
         r0 = r2;
         r0 += 8;
         if r0 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data_end = const core::mem::offset_of!(xdp_md, data_end),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test4\")"]
#[doc = "__failure __msg(\"invalid access to packet\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test4() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data_end});
         r4 = *(u32*)(r1 + {xdp_md_data});
         r0 = r4;
         r0 += 8;
         if r0 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_end = const core::mem::offset_of!(xdp_md, data_end),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test5\")"]
#[doc = "__failure __msg(\"R3 !read_ok\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test5() {
    bpf_asm!(
        "r3 = *(u32*)(r1 + {xdp_md_data_meta});
         r4 = *(u32*)(r1 + {xdp_md_data});
         r0 = r3;
         r0 += 8;
         if r0 > r4 goto l0_%=;
         r2 = -8;
         call {bpf_xdp_adjust_meta};
         r0 = *(u8*)(r3 + 0);
         l0_%=: r0 = 0;
         exit;",
        bpf_xdp_adjust_meta = sym bpf_xdp_adjust_meta,
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test6\")"]
#[doc = "__failure __msg(\"invalid access to packet\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test6() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r0 = r3;
         r0 += 8;
         r4 = r2;
         r4 += 8;
         if r4 > r0 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test7\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn meta_access_test7() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r0 = r3;
         r0 += 8;
         r4 = r2;
         r4 += 8;
         if r4 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test8\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn meta_access_test8() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r4 = r2;
         r4 += 0xFFFF;
         if r4 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test9\")"]
#[doc = "__failure __msg(\"invalid access to packet\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test9() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r4 = r2;
         r4 += 0xFFFF;
         r4 += 1;
         if r4 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test10\")"]
#[doc = "__failure __msg(\"invalid access to packet\")"]
#[naked]
pub unsafe extern "C" fn meta_access_test10() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r4 = *(u32*)(r1 + {xdp_md_data_end});
         r5 = 42;
         r6 = 24;
         *(u64*)(r10 - 8) = r5;
         lock *(u64 *)(r10 - 8) += r6;
         r5 = *(u64*)(r10 - 8);
         if r5 > 100 goto l0_%=;
         r3 += r5;
         r5 = r3;
         r6 = r2;
         r6 += 8;
         if r6 > r5 goto l0_%=;
         r2 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_end = const core::mem::offset_of!(xdp_md, data_end),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test11\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn meta_access_test11() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r5 = 42;
         r6 = 24;
         *(u64*)(r10 - 8) = r5;
         lock *(u64 *)(r10 - 8) += r6;
         r5 = *(u64*)(r10 - 8);
         if r5 > 100 goto l0_%=;
         r2 += r5;
         r5 = r2;
         r6 = r2;
         r6 += 8;
         if r6 > r3 goto l0_%=;
         r5 = *(u8*)(r5 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "xdp"]
#[doc = "__description(\"meta access, test12\")"]
#[doc = "__success __retval(0)"]
#[naked]
pub unsafe extern "C" fn meta_access_test12() {
    bpf_asm!(
        "r2 = *(u32*)(r1 + {xdp_md_data_meta});
         r3 = *(u32*)(r1 + {xdp_md_data});
         r4 = *(u32*)(r1 + {xdp_md_data_end});
         r5 = r3;
         r5 += 16;
         if r5 > r4 goto l0_%=;
         r0 = *(u8*)(r3 + 0);
         r5 = r2;
         r5 += 16;
         if r5 > r3 goto l0_%=;
         r0 = *(u8*)(r2 + 0);
         l0_%=: r0 = 0;
         exit;",
        xdp_md_data = const core::mem::offset_of!(xdp_md, data),
        xdp_md_data_end = const core::mem::offset_of!(xdp_md, data_end),
        xdp_md_data_meta = const core::mem::offset_of!(xdp_md, data_meta),
    );
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
