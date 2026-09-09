// SPDX-License-Identifier: GPL-2.0-only

// linux/jump_label.h, linux/kernel.h, linux/memory.h, linux/mutex.h,
// linux/uaccess.h, and asm/cacheflush.h provide the types and functions used
// below in the surrounding kernel translation.

#[repr(C)]
pub struct jump_entry {
    _private: [u8; 0],
}

pub type jump_label_type = i32;
pub const JUMP_LABEL_JMP: jump_label_type = 0;

const NOP32_HI: u16 = 0xc400;
const NOP32_LO: u16 = 0x4820;
const BSR_LINK: u16 = 0xe000;

extern "C" {
    fn jump_entry_code(entry: *mut jump_entry) -> usize;
    fn jump_entry_target(entry: *mut jump_entry) -> usize;
    fn copy_to_kernel_nofault(dst: *mut core::ffi::c_void, src: *const u16, len: usize) -> i32;
    fn WARN_ON(condition: bool) -> bool;
    fn flush_icache_range(start: usize, end: usize);
}

pub unsafe fn arch_jump_label_transform(
    entry: *mut jump_entry,
    type_: jump_label_type,
) {
    let addr: usize = jump_entry_code(entry);
    let mut insn: [u16; 2] = [0; 2];
    let mut ret: i32 = 0;

    if type_ == JUMP_LABEL_JMP {
        let mut offset: isize = jump_entry_target(entry) as isize - jump_entry_code(entry) as isize;

        if WARN_ON(offset & 1 != 0 || offset < -67108864 || offset >= 67108864) {
            return;
        }

        offset >>= 1;

        insn[0] = BSR_LINK
            | ((((offset as usize) >> 16) as u16) & 0x3ff);
        insn[1] = ((offset as usize) & 0xffff) as u16;
    } else {
        insn[0] = NOP32_HI;
        insn[1] = NOP32_LO;
    }

    ret = copy_to_kernel_nofault(addr as *mut core::ffi::c_void, insn.as_ptr(), 4);
    WARN_ON(ret != 0);

    flush_icache_range(addr, addr + 4);
}

pub unsafe fn arch_jump_label_transform_static(
    entry: *mut jump_entry,
    type_: jump_label_type,
) {
    /*
     * We use the same instructions in the arch_static_branch and
     * arch_static_branch_jump inline functions, so there's no
     * need to patch them up here.
     * The core will call arch_jump_label_transform  when those
     * instructions need to be replaced.
     */
    arch_jump_label_transform(entry, type_);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
