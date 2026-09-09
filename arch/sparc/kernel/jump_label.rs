// SPDX-License-Identifier: GPL-2.0
// Translated from the C implementation. Required kernel declarations are
// supplied by the surrounding Rust translation unit.

use core::ptr;

#[repr(C)]
pub struct jump_entry {
    pub code: u64,
    pub target: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jump_label_type {
    JUMP_LABEL_JMP = 0,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

extern "C" {
    pub static mut text_mutex: mutex;
    pub fn mutex_lock(lock: *mut mutex);
    pub fn mutex_unlock(lock: *mut mutex);
    pub fn flushi(addr: *mut u32);
    pub fn BUG_ON(condition: bool);
}

pub unsafe fn arch_jump_label_transform(
    entry: *mut jump_entry,
    type_: jump_label_type,
) {
    let insn = (*entry).code as *mut u32;
    let val: u32;

    if type_ == jump_label_type::JUMP_LABEL_JMP {
        let off = ((*entry).target as i32).wrapping_sub((*entry).code as i32);
        let mut use_v9_branch = false;

        BUG_ON((off & 3) != 0);

        // CONFIG_SPARC64 is a build-time condition from the original source.
        #[cfg(CONFIG_SPARC64)]
        {
            if off <= 0xfffff && off >= -0x100000 {
                use_v9_branch = true;
            }
        }
        if use_v9_branch {
            /* WDISP19 - target is . + immed << 2 */
            /* ba,pt %xcc, . + off */
            val = 0x10680000 | (((off as u32) >> 2) & 0x7ffff);
        } else {
            /* WDISP22 - target is . + immed << 2 */
            BUG_ON(off > 0x7fffff);
            BUG_ON(off < -0x800000);
            /* ba . + off */
            val = 0x10800000 | (((off as u32) >> 2) & 0x3fffff);
        }
    } else {
        val = 0x01000000;
    }

    mutex_lock(ptr::addr_of_mut!(text_mutex));
    ptr::write_volatile(insn, val);
    flushi(insn);
    mutex_unlock(ptr::addr_of_mut!(text_mutex));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
