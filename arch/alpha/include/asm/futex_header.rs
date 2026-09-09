/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Linux dependencies supplied by the surrounding kernel translation:
 * linux/futex.h, linux/uaccess.h, asm/errno.h, and asm/barrier.h.
 * The original declarations are kernel-only; this file preserves that intent.
 */

#[cfg(feature = "kernel")]
pub mod kernel {
    /* The Alpha instruction sequences below are intentionally retained as
     * inline-assembly source.  They require the surrounding Alpha kernel
     * exception/barrier definitions (EXC and __ASM_SMP_MB). */
    macro_rules! __futex_atomic_op {
        ($insn:expr, $ret:ident, $oldval:ident, $uaddr:ident, $oparg:ident) => {{
            /* __ASM_SMP_MB
             * 1: ldl_l oldval,0(uaddr)
             *     <insn>
             * 2: stl_c ret,0(uaddr)
             *     beq ret,4f
             *     mov $31,ret
             * 3: .subsection 2
             * 4: br 1b
             *     .previous
             * EXC(1b,3b,$31,ret)
             * EXC(2b,3b,$31,ret)
             * clobber: memory
             */
            let _ = ($insn, &mut $ret, &mut $oldval, $uaddr, $oparg);
        }};
    }

    /* FUTEX_OP_* values are supplied by linux/futex.h. */
    pub const FUTEX_OP_SET: i32 = 0;
    pub const FUTEX_OP_ADD: i32 = 1;
    pub const FUTEX_OP_OR: i32 = 2;
    pub const FUTEX_OP_ANDN: i32 = 3;
    pub const FUTEX_OP_XOR: i32 = 4;

    /* Linux errno values supplied by asm/errno.h. */
    const EFAULT: i32 = 14;
    const ENOSYS: i32 = 38;

    /* access_ok(uaddr, sizeof(u32)) from linux/uaccess.h. */
    unsafe extern "C" {
        fn access_ok(addr: *const u32, size: usize) -> bool;
    }

    pub unsafe fn arch_futex_atomic_op_inuser(
        op: i32,
        oparg: i32,
        oval: *mut i32,
        uaddr: *mut u32,
    ) -> i32 {
        let mut oldval: i32 = 0;
        let mut ret: i32;

        if !access_ok(uaddr, core::mem::size_of::<u32>()) {
            return -EFAULT;
        }

        match op {
            FUTEX_OP_SET => {
                __futex_atomic_op!("mov %3,%1\n", ret, oldval, uaddr, oparg);
                ret = 0;
            }
            FUTEX_OP_ADD => {
                __futex_atomic_op!("addl %0,%3,%1\n", ret, oldval, uaddr, oparg);
                ret = 0;
            }
            FUTEX_OP_OR => {
                __futex_atomic_op!("or %0,%3,%1\n", ret, oldval, uaddr, oparg);
                ret = 0;
            }
            FUTEX_OP_ANDN => {
                __futex_atomic_op!("andnot %0,%3,%1\n", ret, oldval, uaddr, oparg);
                ret = 0;
            }
            FUTEX_OP_XOR => {
                __futex_atomic_op!("xor %0,%3,%1\n", ret, oldval, uaddr, oparg);
                ret = 0;
            }
            _ => ret = -ENOSYS,
        }

        if ret == 0 {
            *oval = oldval;
        }

        ret
    }

    pub unsafe fn futex_atomic_cmpxchg_inatomic(
        uval: *mut u32,
        uaddr: *mut u32,
        oldval: u32,
        newval: u32,
    ) -> i32 {
        let mut ret: i32 = 0;
        let mut cmp: i32;
        let mut prev: u32;

        if !access_ok(uaddr, core::mem::size_of::<u32>()) {
            return -EFAULT;
        }

        /* __ASM_SMP_MB
         * 1: ldl_l prev,0(uaddr)
         *     cmpeq prev,(long)(int)oldval,cmp
         *     beq cmp,3f
         *     mov newval,cmp
         * 2: stl_c cmp,0(uaddr)
         *     beq cmp,4f
         * 3: .subsection 2
         * 4: br 1b
         *     .previous
         * EXC(1b,3b,$31,ret)
         * EXC(2b,3b,$31,ret)
         * clobber: memory
         */
        let _ = (oldval, newval, &mut cmp, &mut prev);

        *uval = prev;
        ret
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
