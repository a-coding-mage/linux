// SPDX-License-Identifier: GPL-2.0

use core::arch::asm;

// Definitions supplied by the surrounding architecture code.
pub const X86_CR0_EM: usize = 1 << 2;
pub const X86_CR0_TS: usize = 1 << 3;
pub const X86_EFLAGS_ID: u32 = 1 << 21;
pub const X86_FEATURE_FPU: usize = 0;

#[repr(C)]
pub struct cpu_features {
    pub flags: [u32; 20],
    pub level: u32,
    pub family: u32,
    pub model: u32,
}

pub static mut cpu: cpu_features = cpu_features {
    flags: [0; 20],
    level: 0,
    family: 0,
    model: 0,
};
pub static mut cpu_vendor: [u32; 3] = [0; 3];

static mut loaded_flags: bool = false;

unsafe fn has_fpu() -> i32 {
    let mut fcw: u16 = u16::MAX;
    let mut fsw: u16 = u16::MAX;
    let mut cr0: usize;

    asm!("mov {}, cr0", out(reg) cr0);
    if cr0 & (X86_CR0_EM | X86_CR0_TS) != 0 {
        cr0 &= !(X86_CR0_EM | X86_CR0_TS);
        asm!("mov cr0, {}", in(reg) cr0);
    }

    asm!(
        "fninit",
        "fnstsw [{}]",
        "fnstcw [{}]",
        in(reg) &mut fsw,
        in(reg) &mut fcw,
    );

    (fsw == 0 && (fcw & 0x103f) == 0x003f) as i32
}

#[cfg(CONFIG_X86_32)]
pub unsafe fn has_eflag(mask: usize) -> bool {
    let f0: usize;
    let f1: usize;

    asm!(
        "pushfd",
        "pushfd",
        "pop {0}",
        "mov {0}, {1}",
        "xor {1}, {2}",
        "push {1}",
        "popfd",
        "pushfd",
        "pop {1}",
        "popfd",
        out(reg) f0,
        out(reg) f1,
        in(reg) mask,
    );

    ((f0 ^ f1) & mask) != 0
}

pub unsafe fn cpuid_count(id: u32, count: u32, a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32) {
    asm!(
        "cpuid",
        inout("eax") id => _,
        inout("ecx") count => _,
        lateout("ebx") *b,
        lateout("edx") *d,
        lateout("eax") *a,
        lateout("ecx") *c,
    );
}

#[inline(always)]
unsafe fn cpuid(id: u32, a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32) {
    cpuid_count(id, 0, a, b, c, d);
}

extern "C" {
    fn set_bit(nr: usize, addr: *mut u32);
}

pub unsafe fn get_cpuflags() {
    let mut max_intel_level: u32;
    let mut max_amd_level: u32;
    let mut tfms: u32;
    let mut ignored: u32;

    if loaded_flags {
        return;
    }
    loaded_flags = true;

    if has_fpu() != 0 {
        set_bit(X86_FEATURE_FPU, cpu.flags.as_mut_ptr());
    }

    #[cfg(CONFIG_X86_32)]
    if has_eflag(X86_EFLAGS_ID as usize) {
        cpuid(0x0, &mut max_intel_level, &mut cpu_vendor[0], &mut cpu_vendor[2], &mut cpu_vendor[1]);

        if max_intel_level >= 0x00000001 && max_intel_level <= 0x0000ffff {
            cpuid(0x1, &mut tfms, &mut ignored, &mut cpu.flags[4], &mut cpu.flags[0]);
            cpu.level = (tfms >> 8) & 15;
            cpu.family = cpu.level;
            cpu.model = (tfms >> 4) & 15;
            if cpu.level >= 6 {
                cpu.model += ((tfms >> 16) & 0xf) << 4;
            }
        }

        if max_intel_level >= 0x00000007 {
            cpuid_count(0x00000007, 0, &mut ignored, &mut ignored, &mut cpu.flags[16], &mut ignored);
        }

        cpuid(0x80000000, &mut max_amd_level, &mut ignored, &mut ignored, &mut ignored);
        if max_amd_level >= 0x80000001 && max_amd_level <= 0x8000ffff {
            cpuid(0x80000001, &mut ignored, &mut ignored, &mut cpu.flags[6], &mut cpu.flags[1]);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
