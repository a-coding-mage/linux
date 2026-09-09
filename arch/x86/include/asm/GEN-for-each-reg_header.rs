/* SPDX-License-Identifier: GPL-2.0 */
/*
 * These are in machine order; things rely on that.
 *
 * `GEN!` is supplied by the including translation context.
 */

#[cfg(target_pointer_width = "64")]
mod config_64bit {
    GEN!(rax);
    GEN!(rcx);
    GEN!(rdx);
    GEN!(rbx);
    GEN!(rsp);
    GEN!(rbp);
    GEN!(rsi);
    GEN!(rdi);
    GEN!(r8);
    GEN!(r9);
    GEN!(r10);
    GEN!(r11);
    GEN!(r12);
    GEN!(r13);
    GEN!(r14);
    GEN!(r15);
}

#[cfg(not(target_pointer_width = "64"))]
mod config_32bit {
    GEN!(eax);
    GEN!(ecx);
    GEN!(edx);
    GEN!(ebx);
    GEN!(esp);
    GEN!(ebp);
    GEN!(esi);
    GEN!(edi);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
