/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * ldt.h
 *
 * Definitions of structures used with the modify_ldt system call.
 */

/* Maximum number of LDT entries supported. */
pub const LDT_ENTRIES: u32 = 8192;
/* The size of each LDT entry. */
pub const LDT_ENTRY_SIZE: u32 = 8;

/*
 * Note on 64bit base and limit is ignored and you cannot set DS/ES/CS
 * not to the default values if you still want to do syscalls. This
 * call is more for 32bit mode therefore.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_desc {
    pub entry_number: u32,
    pub base_addr: u32,
    pub limit: u32,
    /* C bit-fields packed into one unsigned int. */
    pub flags: u32,
}

impl user_desc {
    pub const SEG_32BIT_SHIFT: u32 = 0;
    pub const CONTENTS_SHIFT: u32 = 1;
    pub const READ_EXEC_ONLY_SHIFT: u32 = 3;
    pub const LIMIT_IN_PAGES_SHIFT: u32 = 4;
    pub const SEG_NOT_PRESENT_SHIFT: u32 = 5;
    pub const USEABLE_SHIFT: u32 = 6;
    #[cfg(target_arch = "x86_64")]
    pub const LM_SHIFT: u32 = 7;

    pub const fn seg_32bit(&self) -> u32 {
        (self.flags >> Self::SEG_32BIT_SHIFT) & 1
    }

    pub const fn contents(&self) -> u32 {
        (self.flags >> Self::CONTENTS_SHIFT) & 3
    }

    pub const fn read_exec_only(&self) -> u32 {
        (self.flags >> Self::READ_EXEC_ONLY_SHIFT) & 1
    }

    pub const fn limit_in_pages(&self) -> u32 {
        (self.flags >> Self::LIMIT_IN_PAGES_SHIFT) & 1
    }

    pub const fn seg_not_present(&self) -> u32 {
        (self.flags >> Self::SEG_NOT_PRESENT_SHIFT) & 1
    }

    pub const fn useable(&self) -> u32 {
        (self.flags >> Self::USEABLE_SHIFT) & 1
    }

    #[cfg(target_arch = "x86_64")]
    pub const fn lm(&self) -> u32 {
        (self.flags >> Self::LM_SHIFT) & 1
    }
}

pub const MODIFY_LDT_CONTENTS_DATA: u32 = 0;
pub const MODIFY_LDT_CONTENTS_STACK: u32 = 1;
pub const MODIFY_LDT_CONTENTS_CODE: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
