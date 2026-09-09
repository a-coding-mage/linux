/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* The target member is reused for adding new actions, the
 * value of the real target is -1 to -NUM_STANDARD_TARGETS.
 * For backward compatibility, the 4 lsb (2 would be enough,
 * but let's play it safe) are kept to designate this target.
 * The remaining bits designate the action. By making the set
 * action 0xfffffff0, the result will look ok for older
 * versions. [September 2006] */
pub const MARK_SET_VALUE: u32 = 0xfffffff0;
pub const MARK_OR_VALUE: u32 = 0xffffffe0;
pub const MARK_AND_VALUE: u32 = 0xffffffd0;
pub const MARK_XOR_VALUE: u32 = 0xffffffc0;

#[repr(C)]
pub struct ebt_mark_t_info {
    pub mark: usize,
    /* EBT_ACCEPT, EBT_DROP, EBT_CONTINUE or EBT_RETURN */
    pub target: i32,
}

pub const EBT_MARK_TARGET: &str = "mark";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
