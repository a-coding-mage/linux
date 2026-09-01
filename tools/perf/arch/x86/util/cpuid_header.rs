/* SPDX-License-Identifier: GPL-2.0 */

use core::arch::asm;

pub unsafe fn cpuid(
    op: u32,
    op2: u32,
    a: *mut u32,
    b: *mut u32,
    c: *mut u32,
    d: *mut u32,
) {
    let a_out: u32;
    let b_out: u32;
    let c_out: u32;
    let d_out: u32;

    /*
     * Preserve %ebx/%rbx register by either placing it in %rdi or saving it
     * on the stack - x86-64 needs to avoid the stack red zone. In PIC
     * compilations %ebx contains the address of the global offset
     * table. %rbx is occasionally used to address stack variables in
     * presence of dynamic allocas.
     */
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "mov rdi, rbx",
            "cpuid",
            "xchg rdi, rbx",
            inlateout("eax") op => a_out,
            lateout("rdi") b_out,
            inlateout("ecx") op2 => c_out,
            lateout("edx") d_out,
        );
    }

    #[cfg(all(target_arch = "x86", not(target_arch = "x86_64")))]
    unsafe {
        asm!(
            "push ebx",
            "cpuid",
            "mov edi, ebx",
            "pop ebx",
            inlateout("eax") op => a_out,
            lateout("edi") b_out,
            inlateout("ecx") op2 => c_out,
            lateout("edx") d_out,
        );
    }

    unsafe {
        *a = a_out;
        *b = b_out;
        *c = c_out;
        *d = d_out;
    }
}

unsafe extern "C" {
    pub fn get_cpuid_0(vendor: *mut i8, lvl: *mut u32);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
