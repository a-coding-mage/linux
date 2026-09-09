// SPDX-License-Identifier: GPL-2.0

// C dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    static mut stack_protector_debug: core::ffi::c_int;
    static mut __stack_chk_guard: usize;
    static mut vmlinux: Vmlinux;
    fn machine_has_relocated_lowcore() -> bool;
    fn s390_kernel_write(dst: *mut InsnRil, src: *const InsnRil, size: usize);
    fn hex_byte_pack(dst: *mut core::ffi::c_char, byte: u8);
    fn __kernel_pa(address: usize) -> usize;
    fn __kernel_va(address: *const InsnRil) -> usize;
    fn boot_debug(fmt: *const core::ffi::c_char, ...);
    fn boot_emerg(fmt: *const core::ffi::c_char, ...);
    fn boot_panic(fmt: *const core::ffi::c_char, ...) -> !;
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
    fn pr_emerg(fmt: *const core::ffi::c_char, ...);
    fn panic(fmt: *const core::ffi::c_char, ...) -> !;
}

#[repr(C)]
pub struct Vmlinux {
    pub stack_prot_start: usize,
    pub stack_prot_end: usize,
}

#[repr(C, packed)]
pub struct InsnRil {
    pub opc1: u8,
    // C bitfields r1:4 and opc2:4 share this byte; their bit ordering is ABI-defined.
    pub r1_opc2: u8,
    pub imm: u32,
}

const INSN_RIL_STRING_SIZE: usize = core::mem::size_of::<InsnRil>() * 2 + 1;
const EINVAL: i32 = 22;
const TEXT_OFFSET: usize = 0;
const LOWCORE_ALT_ADDRESS: usize = 0;
const __LC_STACK_CANARY: usize = 0;

#[inline]
unsafe fn insn_opc2(insn: *const InsnRil) -> u8 {
    (*insn).r1_opc2 & 0x0f
}

unsafe fn vaddress_to_insn(vaddress: usize) -> *mut InsnRil {
    #[cfg(__DECOMPRESSOR)]
    { __kernel_pa(vaddress) as *mut InsnRil }
    #[cfg(not(__DECOMPRESSOR))]
    { vaddress as *mut InsnRil }
}

unsafe fn insn_to_vaddress(insn: *mut InsnRil) -> usize {
    #[cfg(__DECOMPRESSOR)]
    { __kernel_va(insn) }
    #[cfg(not(__DECOMPRESSOR))]
    { insn as usize }
}

unsafe fn insn_ril_to_string(str_: *mut core::ffi::c_char, insn: *mut InsnRil) {
    let ptr = insn as *mut u8;
    let mut i = 0usize;
    while i < core::mem::size_of::<InsnRil>() {
        hex_byte_pack(str_.add(2 * i), *ptr.add(i));
        i += 1;
    }
    *str_.add(2 * i) = 0;
}

unsafe fn stack_protector_dump(old: *mut InsnRil, new: *mut InsnRil) {
    let mut ostr = [0i8; INSN_RIL_STRING_SIZE];
    let mut nstr = [0i8; INSN_RIL_STRING_SIZE];
    insn_ril_to_string(ostr.as_mut_ptr(), old);
    insn_ril_to_string(nstr.as_mut_ptr(), new);
    pr_debug(b"%016lx: %s -> %s\n\0".as_ptr() as _, insn_to_vaddress(old), ostr.as_ptr(), nstr.as_ptr());
}

unsafe fn stack_protector_verify(insn: *mut InsnRil, kernel_start: usize) -> i32 {
    if (*insn).opc1 == 0xc0 && insn_opc2(insn) == 0 { return 0; }
    if (*insn).opc1 == 0xc4 && insn_opc2(insn) == 8 { return 0; }
    let mut istr = [0i8; INSN_RIL_STRING_SIZE];
    insn_ril_to_string(istr.as_mut_ptr(), insn);
    let vaddress = insn_to_vaddress(insn);
    #[cfg(__DECOMPRESSOR)]
    {
        let offset = insn as usize - kernel_start + TEXT_OFFSET;
        boot_emerg(b"Unexpected instruction at %016lx/%016lx: %s\n\0".as_ptr() as _, vaddress, offset, istr.as_ptr());
        boot_panic(b"Stackprotector error\n\0".as_ptr() as _);
    }
    #[cfg(not(__DECOMPRESSOR))]
    { pr_emerg(b"Unexpected instruction at %016lx: %s\n\0".as_ptr() as _, vaddress, istr.as_ptr()); }
    -EINVAL
}

pub unsafe fn __stack_protector_apply(start: *mut usize, end: *mut usize, kernel_start: usize) -> i32 {
    let mut canary = __LC_STACK_CANARY;
    if machine_has_relocated_lowcore() { canary = canary.wrapping_add(LOWCORE_ALT_ADDRESS); }
    let mut loc = start;
    while loc < end {
        let insn = vaddress_to_insn(*loc);
        let rc = stack_protector_verify(insn, kernel_start);
        if rc != 0 { return rc; }
        let mut new = core::ptr::read_unaligned(insn);
        new.opc1 = 0xc0;
        new.r1_opc2 = (new.r1_opc2 & 0xf0) | 0x0f;
        new.imm = canary as u32;
        if stack_protector_debug != 0 { stack_protector_dump(insn, &mut new); }
        s390_kernel_write(insn, &new, core::mem::size_of::<InsnRil>());
        loc = loc.add(1);
    }
    0
}

#[cfg(__DECOMPRESSOR)]
pub unsafe fn __stack_protector_apply_early(kernel_start: usize) {
    let start = vmlinux.stack_prot_start as *mut usize;
    let end = vmlinux.stack_prot_end as *mut usize;
    __stack_protector_apply(start, end, kernel_start);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
