// SPDX-License-Identifier: GPL-2.0
//! Early CPU discovery shared by setup and the compressed kernel.

use crate::cpu_bindings as b;
use core::arch::asm;
use core::ffi::{c_int, c_ulong};
use core::ptr::{addr_of, addr_of_mut};

// The layout and array length come from the native boot cpuflags.h bindings.
#[export_name = "cpu"]
pub(crate) static mut CPU: b::cpu_features = b::cpu_features {
    level: 0,
    family: 0,
    model: 0,
    flags: [0; b::NCAPINTS as usize],
};
#[export_name = "cpu_vendor"]
pub(crate) static mut CPU_VENDOR: [u32; 3] = [0; 3];
static mut LOADED_FLAGS: bool = false;

unsafe fn has_fpu() -> c_int {
    let mut fcw = u16::MAX;
    let mut fsw = u16::MAX;
    let mut cr0: c_ulong;

    // SAFETY: this function is called only in privileged early boot. Match the
    // C owner: clear EM/TS if needed and deliberately leave them cleared.
    unsafe {
        asm!("mov {}, cr0", out(reg) cr0, options(nomem, nostack, preserves_flags));
        let em_ts = (b::LUPOS_BOOT_X86_CR0_EM | b::LUPOS_BOOT_X86_CR0_TS) as c_ulong;
        if cr0 & em_ts != 0 {
            cr0 &= !em_ts;
            asm!("mov cr0, {}", in(reg) cr0, options(nomem, nostack, preserves_flags));
        }

        // These are the original C probe's three intentional x87 operations,
        // not compiler-generated floating-point code: initialize the FPU, then
        // store its 16-bit status and control words. No wait instruction is added.
        asm!(
            "fninit",
            "fnstsw word ptr [{fsw}]",
            "fnstcw word ptr [{fcw}]",
            fsw = in(reg) addr_of_mut!(fsw),
            fcw = in(reg) addr_of_mut!(fcw),
            out("st(0)") _,
            out("st(1)") _,
            out("st(2)") _,
            out("st(3)") _,
            out("st(4)") _,
            out("st(5)") _,
            out("st(6)") _,
            out("st(7)") _,
            options(nostack),
        );
    }

    (fsw == 0 && (fcw & 0x103f) == 0x003f) as c_int
}

#[cfg(CONFIG_X86_32)]
#[no_mangle]
pub(crate) unsafe extern "C" fn has_eflag(mask: c_ulong) -> bool {
    let f0: c_ulong;
    let f1: c_ulong;

    // SAFETY: preserve the original two saved EFLAGS words and restore EFLAGS.
    // Explicit 32-bit operations also match the C source's .code16gcc intent.
    // Early outputs cannot overlap the still-live input mask.
    unsafe {
        asm!(
            "pushfd",
            "pushfd",
            "pop {f0:e}",
            "mov {f1:e}, {f0:e}",
            "xor {f1:e}, {mask:e}",
            "push {f1:e}",
            "popfd",
            "pushfd",
            "pop {f1:e}",
            "popfd",
            f0 = out(reg) f0,
            f1 = out(reg) f1,
            mask = in(reg) mask,
            options(preserves_flags),
        );
    }

    ((f0 ^ f1) & mask) != 0
}

#[cfg(not(CONFIG_X86_32))]
#[inline]
unsafe fn has_eflag(_mask: c_ulong) -> bool {
    // cpuflags.h defines this inline result on x86-64.
    true
}

#[no_mangle]
pub(crate) unsafe extern "C" fn cpuid_count(
    id: u32,
    count: u32,
    a: *mut u32,
    b: *mut u32,
    c: *mut u32,
    d: *mut u32,
) {
    let eax: u32;
    let ebx: usize;
    let ecx: u32;
    let edx: u32;

    // SAFETY: callers established CPUID availability. LLVM can reserve BX as a
    // base register, so save/restore it explicitly using an early output that
    // cannot overlap AX/CX inputs or DX output. No stack or output pointers are
    // used inside the assembly. Each architectural output is captured once.
    unsafe {
        #[cfg(CONFIG_X86_64)]
        asm!(
            "mov {saved_b}, rbx",
            "cpuid",
            "xchg {saved_b}, rbx",
            saved_b = out(reg) ebx,
            inout("eax") id => eax,
            inout("ecx") count => ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );
        #[cfg(CONFIG_X86_32)]
        asm!(
            "mov {saved_b:e}, ebx",
            "cpuid",
            "xchg {saved_b:e}, ebx",
            saved_b = out(reg) ebx,
            inout("eax") id => eax,
            inout("ecx") count => ecx,
            out("edx") edx,
            options(nostack, preserves_flags),
        );

        // The original callers deliberately alias multiple ignored outputs.
        // Raw stores impose no Rust-reference exclusivity on those pointers.
        a.write(eax);
        b.write(ebx as u32);
        c.write(ecx);
        d.write(edx);
    }
}

#[inline]
unsafe fn cpuid(id: u32, a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32) {
    // SAFETY: forward the native pointer and CPUID-availability contract.
    unsafe { cpuid_count(id, 0, a, b, c, d) }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn get_cpuflags() {
    let mut max_intel_level = 0;
    let mut max_amd_level = 0;
    let mut tfms = 0;
    let mut ignored = 0;

    // SAFETY: boot discovery is serialized, as in the C owner. Use raw pointers
    // throughout so neither the global state nor aliased ignored CPUID outputs
    // create Rust references with stronger lifetime/aliasing requirements.
    unsafe {
        if addr_of!(LOADED_FLAGS).read() {
            return;
        }
        addr_of_mut!(LOADED_FLAGS).write(true);

        let flags = addr_of_mut!(CPU.flags).cast::<u32>();
        if has_fpu() != 0 {
            // bitops.h set_bit uses a non-locked 32-bit BTS memory operation.
            asm!(
                "bts dword ptr [{flags}], {bit:e}",
                flags = in(reg) flags,
                bit = in(reg) b::X86_FEATURE_FPU,
                options(nostack),
            );
        }

        if has_eflag(b::LUPOS_BOOT_X86_EFLAGS_ID as c_ulong) {
            let vendor = addr_of_mut!(CPU_VENDOR).cast::<u32>();
            cpuid(
                0,
                addr_of_mut!(max_intel_level),
                vendor,
                vendor.add(2),
                vendor.add(1),
            );

            if max_intel_level >= 0x00000001 && max_intel_level <= 0x0000ffff {
                cpuid(
                    1,
                    addr_of_mut!(tfms),
                    addr_of_mut!(ignored),
                    flags.add(4),
                    flags,
                );
                let level = ((tfms >> 8) & 15) as c_int;
                let mut model = ((tfms >> 4) & 15) as c_int;
                if level >= 6 {
                    model += (((tfms >> 16) & 0xf) << 4) as c_int;
                }
                addr_of_mut!(CPU.level).write(level);
                addr_of_mut!(CPU.family).write(level);
                addr_of_mut!(CPU.model).write(model);
            }

            if max_intel_level >= 0x00000007 {
                cpuid_count(
                    7,
                    0,
                    addr_of_mut!(ignored),
                    addr_of_mut!(ignored),
                    flags.add(16),
                    addr_of_mut!(ignored),
                );
            }

            cpuid(
                0x80000000,
                addr_of_mut!(max_amd_level),
                addr_of_mut!(ignored),
                addr_of_mut!(ignored),
                addr_of_mut!(ignored),
            );
            if max_amd_level >= 0x80000001 && max_amd_level <= 0x8000ffff {
                cpuid(
                    0x80000001,
                    addr_of_mut!(ignored),
                    addr_of_mut!(ignored),
                    flags.add(6),
                    flags.add(1),
                );
            }
        }
    }
}
