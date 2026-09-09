/* SPDX-License-Identifier: GPL-2.0 */

// Define a weak system-call symbol that aliases the unavailable-system-call
// implementation, matching the C asm(".weak ... = sys_ni_syscall") macro.
#[macro_export]
macro_rules! cond_syscall {
    ($x:ident) => {
        core::arch::global_asm!(concat!(
            ".weak\\t",
            stringify!($x),
            "\\n",
            stringify!($x),
            " = sys_ni_syscall"
        ));
    };
}

// Define a symbol alias and export the alias globally, matching the C asm
// macro's "alias = name\\n\\t.globl alias" sequence.
#[macro_export]
macro_rules! SYSCALL_ALIAS {
    ($alias:ident, $name:ident) => {
        core::arch::global_asm!(concat!(
            stringify!($alias),
            " = ",
            stringify!($name),
            "\\n\\t.globl ",
            stringify!($alias)
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
