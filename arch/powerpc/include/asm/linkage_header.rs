/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from <asm/types.h>; declarations supplied by other files
// remain external to this translation unit.

// CONFIG_PPC64_ELF_ABI_V1
#[cfg(feature = "CONFIG_PPC64_ELF_ABI_V1")]
macro_rules! cond_syscall {
    ($x:ident) => {
        core::arch::global_asm!(concat!(
            "\t.weak ", stringify!($x), "\n\t.set ", stringify!($x),
            ", sys_ni_syscall\n\t.weak .", stringify!($x), "\n\t.set .",
            stringify!($x), ", .sys_ni_syscall\n"
        ));
    };
}

#[cfg(feature = "CONFIG_PPC64_ELF_ABI_V1")]
macro_rules! SYSCALL_ALIAS {
    ($alias:ident, $name:ident) => {
        core::arch::global_asm!(concat!(
            "\t.globl ", stringify!($alias), "\n\t.set ", stringify!($alias),
            ", ", stringify!($name), "\n\t.globl .", stringify!($alias),
            "\n\t.set .", stringify!($alias), ", .", stringify!($name)
        ));
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
