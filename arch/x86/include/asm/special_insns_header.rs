/* SPDX-License-Identifier: GPL-2.0 */
// C header guard and includes omitted; dependent kernel symbols are supplied externally.

pub unsafe extern "C" fn native_write_cr0(val: usize);

#[inline]
pub unsafe fn native_read_cr0() -> usize {
    let val: usize;
    core::arch::asm!("mov {}, cr0", out(reg) val);
    val
}

#[inline(always)]
pub unsafe fn native_read_cr2() -> usize {
    let val: usize;
    core::arch::asm!("mov {}, cr2", out(reg) val);
    val
}

#[inline(always)]
pub unsafe fn native_write_cr2(val: usize) {
    core::arch::asm!("mov cr2, {}", in(reg) val, options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __native_read_cr3() -> usize {
    let val: usize;
    core::arch::asm!("mov {}, cr3", out(reg) val);
    val
}

#[inline(always)]
pub unsafe fn native_write_cr3(val: usize) {
    core::arch::asm!("mov cr3, {}", in(reg) val, options(nostack, preserves_flags));
}

#[inline]
pub unsafe fn native_read_cr4() -> usize {
    let val: usize;
    // CONFIG_X86_32 uses the C exception-table fallback for CPUs without CR4.
    core::arch::asm!("mov {}, cr4", out(reg) val);
    val
}

pub unsafe extern "C" fn native_write_cr4(val: usize);

// CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS selects the instruction-backed forms.
#[inline]
pub unsafe fn rdpkru() -> u32 {
    let pkru: u32;
    let mut edx: u32;
    core::arch::asm!("rdpkru", out("eax") pkru, out("edx") edx, in("ecx") 0u32);
    pkru
}

#[inline]
pub unsafe fn wrpkru(pkru: u32) {
    core::arch::asm!("wrpkru", in("eax") pkru, in("ecx") 0u32, in("edx") 0u32);
}

// When CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS is disabled, rdpkru returns 0 and
// wrpkru is a no-op. Build systems should select those alternate definitions.

#[inline(always)]
pub unsafe fn wbinvd() {
    core::arch::asm!("wbinvd", options(nostack));
}

// Instruction encoding provided for binutils backwards compatibility.
pub const ASM_WBNOINVD: [u8; 3] = [0xf3, 0x0f, 0x09];

#[inline(always)]
pub unsafe fn wbnoinvd() {
    // alternative("wbinvd", ASM_WBNOINVD, X86_FEATURE_WBNOINVD)
    core::arch::asm!("wbinvd", options(nostack));
}

#[inline]
pub unsafe fn __read_cr4() -> usize { native_read_cr4() }

#[inline]
pub unsafe fn read_cr0() -> usize { native_read_cr0() }

#[inline]
pub unsafe fn write_cr0(x: usize) { native_write_cr0(x); }

#[inline(always)]
pub unsafe fn read_cr2() -> usize { native_read_cr2() }

#[inline(always)]
pub unsafe fn write_cr2(x: usize) { native_write_cr2(x); }

/* Careful! CR3 contains more than just an address. */
#[inline]
pub unsafe fn __read_cr3() -> usize { __native_read_cr3() }

#[inline]
pub unsafe fn write_cr3(x: usize) { native_write_cr3(x); }

#[inline]
pub unsafe fn __write_cr4(x: usize) { native_write_cr4(x); }

#[inline(always)]
pub unsafe fn clflush(p: *mut core::ffi::c_void) {
    core::arch::asm!("clflush [{}]", in(reg) p, options(nostack));
}

#[inline]
pub unsafe fn clflushopt(p: *mut core::ffi::c_void) {
    // alternative_io("ds clflush", "clflushopt", X86_FEATURE_CLFLUSHOPT)
    core::arch::asm!("clflush [{}]", in(reg) p, options(nostack));
}

#[inline]
pub unsafe fn clwb(p: *mut core::ffi::c_void) {
    // ALTERNATIVE_2("ds clflush", "clflushopt", X86_FEATURE_CLFLUSHOPT,
    //               "clwb", X86_FEATURE_CLWB)
    core::arch::asm!("clflush [{}]", in(reg) p, options(nostack));
}

#[inline]
pub unsafe fn write_user_shstk_64(addr: *mut u64, val: u64) -> i32 {
    // CONFIG_X86_USER_SHADOW_STACK; the C exception table returns -EFAULT on fault.
    core::arch::asm!("wrussq {}, [{}]", in(reg) val, in(reg) addr, options(nostack));
    0
}

#[inline]
pub unsafe fn nop() { core::arch::asm!("nop"); }

#[inline(always)]
pub unsafe fn serialize() {
    core::arch::asm!(".byte 0x0f, 0x01, 0xe8", options(nostack));
}

#[repr(C)]
pub struct Movdir64bBlock { pub _: [u8; 64] }

#[inline]
pub unsafe fn movdir64b(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    core::arch::asm!(".byte 0x66, 0x0f, 0x38, 0xf8, 0x02",
        in("rax") dst, in("rdx") src,
        options(nostack));
}

#[inline]
pub unsafe fn movdir64b_io(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
    movdir64b(dst, src);
}

#[inline]
pub unsafe fn enqcmds(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) -> i32 {
    let zf: u8;
    core::arch::asm!(".byte 0xf3, 0x0f, 0x38, 0xf8, 0x02, 0x66, 0x90",
        in("rax") dst, in("rdx") src,
        "setz {zf}", zf = lateout(reg_byte) zf, options(nostack));
    if zf != 0 { -11 } else { 0 } // -EAGAIN
}

#[inline(always)]
pub unsafe fn tile_release() {
    core::arch::asm!(".byte 0xc4, 0xe2, 0x78, 0x49, 0xc0");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
