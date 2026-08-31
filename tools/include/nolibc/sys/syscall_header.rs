/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * syscall() definition for NOLIBC
 * Copyright (C) 2024 Thomas Weißschuh <linux@weissschuh.net>
 */

/* make sure to include all global symbols */
/* C dependency: #include "../nolibc.h" */

/* Header guard in C: _NOLIBC_SYS_SYSCALL_H */

macro_rules! ___nolibc_syscall_narg {
    ($_0:expr, $_1:expr, $_2:expr, $_3:expr, $_4:expr, $_5:expr, $_6:expr, $N:expr $(, $rest:expr)*) => {
        $N
    };
}

macro_rules! __nolibc_syscall_narg {
    ($($args:expr),*) => {
        ___nolibc_syscall_narg!($($args,)* 6, 5, 4, 3, 2, 1, 0)
    };
}

macro_rules! __nolibc_syscall {
    (0 $(,)?) => {
        __nolibc_syscall0!()
    };
    (1, $a0:expr $(,)?) => {
        __nolibc_syscall1!($a0)
    };
    (2, $a0:expr, $a1:expr $(,)?) => {
        __nolibc_syscall2!($a0, $a1)
    };
    (3, $a0:expr, $a1:expr, $a2:expr $(,)?) => {
        __nolibc_syscall3!($a0, $a1, $a2)
    };
    (4, $a0:expr, $a1:expr, $a2:expr, $a3:expr $(,)?) => {
        __nolibc_syscall4!($a0, $a1, $a2, $a3)
    };
    (5, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr $(,)?) => {
        __nolibc_syscall5!($a0, $a1, $a2, $a3, $a4)
    };
    (6, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr $(,)?) => {
        __nolibc_syscall6!($a0, $a1, $a2, $a3, $a4, $a5)
    };
}

macro_rules! __nolibc_syscall_n {
    ($N:tt $(, $args:expr)*) => {
        __nolibc_syscall!($N $(, $args)*)
    };
}

macro_rules! _syscall {
    () => {
        __nolibc_syscall_n!(0)
    };
    ($($args:expr),+ $(,)?) => {
        __nolibc_syscall_n!(__nolibc_syscall_narg!($($args),+), $($args),+)
    };
}

macro_rules! syscall {
    ($($args:expr),* $(,)?) => {
        __sysret(_syscall!($($args),*))
    };
}
