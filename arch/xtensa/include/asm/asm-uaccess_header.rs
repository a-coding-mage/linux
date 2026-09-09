/*
 * include/asm-xtensa/uaccess.h
 *
 * User space memory access functions
 *
 * These routines provide basic accessing functions to the user memory
 * space for the kernel. This header file provides functions such as:
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/errno.h, asm/types.h, asm/current.h, asm/asm-offsets.h,
// and asm/processor.h.

/*
 * user_ok determines whether the access to user-space memory is allowed.
 * See the equivalent C-macro version below for clarity.
 *
 * On error, user_ok branches to a label indicated by parameter <error>.
 * This implies that the macro falls through to the next instruction on
 * success.
 *
 * Note that while this macro can be used independently, we designed it for
 * optimal use in the access_ok macro below (i.e., we fall through on success).
 *
 * On Entry:
 *     <aa> register containing memory address
 *     <as> register containing memory size
 *     <at> temp register
 *     <error> label to branch to on error; implies fall-through macro on
 *             success
 * On Exit:
 *     <aa> preserved
 *     <as> preserved
 *     <at> destroyed (actually, (TASK_SIZE + 1 - size))
 */
#[macro_export]
macro_rules! user_ok {
    ($aa:expr, $as:expr, $at:expr, $error:expr) => {{
        // movi $at, __XTENSA_UL_CONST(TASK_SIZE)
        $at = __XTENSA_UL_CONST(TASK_SIZE);
        // bgeu $as, $at, $error
        if ($as as usize) >= ($at as usize) {
            $error;
        }
        // sub $at, $at, $as
        $at = $at.wrapping_sub($as);
        // bgeu $aa, $at, $error
        if ($aa as usize) >= ($at as usize) {
            $error;
        }
    }};
}

/*
 * access_ok determines whether a memory access is allowed. See the
 * equivalent C-macro version below for clarity.
 *
 * On error, access_ok branches to a label indicated by parameter <error>.
 * This implies that the macro falls through to the next instruction on
 * success.
 *
 * Note that we assume success is the common case, and we optimize the
 * branch fall-through case on success.
 *
 * On Entry:
 *     <aa> register containing memory address
 *     <as> register containing memory size
 *     <at> temp register
 *     <sp>
 *     <error> label to branch to on error; implies fall-through macro on
 *             success
 * On Exit:
 *     <aa> preserved
 *     <as> preserved
 *     <at> destroyed
 */
#[macro_export]
macro_rules! access_ok {
    ($aa:expr, $as:expr, $at:expr, $sp:expr, $error:expr) => {{
        user_ok!($aa, $as, $at, $error);
        // .Laccess_ok_<unique>:
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
