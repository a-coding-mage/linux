// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

use core::arch::asm;

unsafe extern "C" {
    fn bpf_get_prandom_u32() -> u32;
    fn bpf_probe_read_user(dst: *mut core::ffi::c_void, size: u32, unsafe_ptr: *const core::ffi::c_void) -> i64;
}

/* Read an uninitialized value from stack at a fixed offset */
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_uninit_stack_fixed_off(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        asm!(
            "
            r0 = 0
            /* force stack depth to be 128 */
            *(u64*)(r10 - 128) = r1
            r1 = *(u8 *)(r10 - 8 )
            r0 += r1
            r1 = *(u8 *)(r10 - 11)
            r1 = *(u8 *)(r10 - 13)
            r1 = *(u8 *)(r10 - 15)
            r1 = *(u16*)(r10 - 16)
            r1 = *(u32*)(r10 - 32)
            r1 = *(u64*)(r10 - 64)
            /* read from a spill of a wrong size, it is a separate
             * branch in check_stack_read_fixed_off()
             */
            *(u32*)(r10 - 72) = r1
            r1 = *(u64*)(r10 - 72)
            r0 = 0
            exit
            ",
            in("r1") ctx,
            options(noreturn)
        );
    }
}

/* Read an uninitialized value from stack at a variable offset */
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn read_uninit_stack_var_off(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        asm!(
            "
            call {bpf_get_prandom_u32}
            /* force stack depth to be 64 */
            *(u64*)(r10 - 64) = r0
            r0 = -r0
            /* give r0 a range [-31, -1] */
            if r0 s<= -32 goto 1f
            if r0 s>= 0 goto 1f
            /* access stack using r0 */
            r1 = r10
            r1 += r0
            r2 = *(u8*)(r1 + 0)
        1:
            r0 = 0
            exit
            ",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            in("r1") ctx,
            options(noreturn)
        );
    }
}

#[inline(never)]
unsafe extern "C" fn dummy() {}

/* Pass a pointer to uninitialized stack memory to a helper.
 * Passed memory block should be marked as STACK_MISC after helper call.
 */
// Original C annotations: SEC("socket"), __log_level(7), __msg("fp-104=mmmmmmmm")
#[unsafe(link_section = "socket")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn helper_uninit_to_misc(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe {
        asm!(
            "
            /* force stack depth to be 128 */
            *(u64*)(r10 - 128) = r1
            r1 = r10
            r1 += -128
            r2 = 32
            r3 = 0
            call {bpf_probe_read_user}
            /* Call to dummy() forces print_verifier_state(..., true),
             * thus showing the stack state, matched by __msg().
             */
            call {dummy}
            r1 = *(u64*)(r10 - 104)
            r0 = 0
            exit
            ",
            bpf_probe_read_user = sym bpf_probe_read_user,
            dummy = sym dummy,
            in("r1") ctx,
            options(noreturn)
        );
    }
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
