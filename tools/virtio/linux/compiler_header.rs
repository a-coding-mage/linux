/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard LINUX_COMPILER_H omitted in Rust. */

/* Avoid redefinition warnings */
/* C dependency: ../../../include/linux/compiler_types.h */
/* C __user marker is undefined around the include, then defined empty here. */
macro_rules! __user {
    () => {};
}

macro_rules! WRITE_ONCE {
    ($var:expr, $val:expr) => {{
        unsafe {
            core::ptr::write_volatile(
                core::ptr::addr_of_mut!($var) as *mut _,
                $val,
            )
        }
    }};
}

macro_rules! READ_ONCE {
    ($var:expr) => {{
        unsafe { core::ptr::read_volatile(core::ptr::addr_of!($var)) }
    }};
}

macro_rules! __aligned {
    ($x:expr) => {
        /* C attribute intent: __attribute((__aligned__($x))) */
    };
}

/**
 * data_race - mark an expression as containing intentional data races
 *
 * This data_race() macro is useful for situations in which data races
 * should be forgiven.  One example is diagnostic code that accesses
 * shared variables but is not a part of the core synchronization design.
 * For example, if accesses to a given variable are protected by a lock,
 * except for diagnostic code, then the accesses under the lock should
 * be plain C-language accesses and those in the diagnostic code should
 * use data_race().  This way, KCSAN will complain if buggy lockless
 * accesses to that variable are introduced, even if the buggy accesses
 * are protected by READ_ONCE() or WRITE_ONCE().
 *
 * This macro *does not* affect normal code generation, but is a hint
 * to tooling that data races here are to be ignored.  If the access must
 * be atomic *and* KCSAN should ignore the access, use both data_race()
 * and READ_ONCE(), for example, data_race(READ_ONCE(x)).
 */
macro_rules! data_race {
    ($expr:expr) => {{
        let __v = $expr;
        __v
    }};
}

macro_rules! __must_check {
    () => {};
}
