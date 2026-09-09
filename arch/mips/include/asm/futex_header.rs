/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 2006  Ralf Baechle (ralf@linux-mips.org)
 */

// C dependencies supplied by the surrounding kernel translation unit:
// linux::futex, linux::uaccess, asm::asm_eva, asm::barrier, asm::compiler,
// asm::errno, and asm::sync.

pub const ARCH_FUTEX_ATOMIC_OP_INUSER: &str = "arch_futex_atomic_op_inuser";
pub const FUTEX_ATOMIC_CMPXCHG_INATOMIC: &str = "futex_atomic_cmpxchg_inatomic";

/* The MIPS inline-assembly implementation is retained here as a source-level
 * note.  Its exact instruction templates and exception-table fixups require
 * the kernel's assembler macros and target-specific Rust asm support. */
macro_rules! __futex_atomic_op {
    ($op:expr, $insn:expr, $ret:expr, $oldval:expr, $uaddr:expr, $oparg:expr) => {{
        if cpu_has_llsc && is_enabled_config_war_r10000_llsc() {
            // ll/sc loop, .fixup and __ex_table entries from the C definition.
            // __WEAK_LLSC_MB, __UA_ADDR, user_ll/user_sc and __SYNC are external
            // kernel assembler facilities and cannot be expressed file-locally.
            unsafe { mips_futex_atomic_asm($ret, $oldval, $uaddr, $oparg, $insn) }
        } else if cpu_has_llsc {
            unsafe { mips_futex_atomic_asm($ret, $oldval, $uaddr, $oparg, $insn) }
        } else {
            $ret = futex_atomic_op_inuser_local($op, $oparg, $oldval, $uaddr);
        }
    }};
}

extern "C" {
    static cpu_has_llsc: bool;
    fn is_enabled_config_war_r10000_llsc() -> bool;
    fn mips_futex_atomic_asm(
        ret: *mut i32, oldval: *mut i32, uaddr: *mut u32, oparg: i32,
        insn: *const core::ffi::c_char,
    );
    fn futex_atomic_op_inuser_local(op: i32, oparg: i32, oval: *mut i32, uaddr: *mut u32) -> i32;
    fn futex_atomic_cmpxchg_inatomic_local(
        uval: *mut u32, uaddr: *mut u32, oldval: u32, newval: u32,
    ) -> i32;
    fn access_ok(addr: *const u32, size: usize) -> bool;
}

pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32, oparg: i32, oval: *mut i32, uaddr: *mut u32,
) -> i32 {
    let mut oldval: i32 = 0;
    let mut ret: i32;

    if !access_ok(uaddr, core::mem::size_of::<u32>()) {
        return -14; // -EFAULT
    }

    match op {
        FUTEX_OP_SET => __futex_atomic_op!(op, c"move $1, %z5".as_ptr(), &mut ret, &mut oldval, uaddr, oparg),
        FUTEX_OP_ADD => __futex_atomic_op!(op, c"addu $1, %1, %z5".as_ptr(), &mut ret, &mut oldval, uaddr, oparg),
        FUTEX_OP_OR => __futex_atomic_op!(op, c"or $1, %1, %z5".as_ptr(), &mut ret, &mut oldval, uaddr, oparg),
        FUTEX_OP_ANDN => __futex_atomic_op!(op, c"and $1, %1, %z5".as_ptr(), &mut ret, &mut oldval, uaddr, !oparg),
        FUTEX_OP_XOR => __futex_atomic_op!(op, c"xor $1, %1, %z5".as_ptr(), &mut ret, &mut oldval, uaddr, oparg),
        _ => ret = -38, // -ENOSYS
    }

    if ret == 0 { *oval = oldval; }
    ret
}

pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32, uaddr: *mut u32, oldval: u32, newval: u32,
) -> i32 {
    let mut ret: i32 = 0;
    let mut val: u32 = 0;

    if !access_ok(uaddr, core::mem::size_of::<u32>()) { return -14; }
    if cpu_has_llsc {
        // MIPS ll/sc compare-and-exchange sequence, including exception fixups.
        ret = mips_futex_cmpxchg_asm(&mut val, uaddr, oldval, newval);
    } else {
        return futex_atomic_cmpxchg_inatomic_local(uval, uaddr, oldval, newval);
    }
    *uval = val;
    ret
}

extern "C" {
    fn mips_futex_cmpxchg_asm(val: *mut u32, uaddr: *mut u32, oldval: u32, newval: u32) -> i32;
}

extern "C" {
    static FUTEX_OP_SET: i32;
    static FUTEX_OP_ADD: i32;
    static FUTEX_OP_OR: i32;
    static FUTEX_OP_ANDN: i32;
    static FUTEX_OP_XOR: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
