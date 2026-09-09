/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: equivalent declarations from <asm-generic/module.h> are
// supplied by the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum M68kFixupType {
    M68kFixupMemoffset,
    M68kFixupVnodeShift,
}

#[repr(C)]
pub struct M68kFixupInfo {
    pub type_: M68kFixupType,
    pub addr: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct ModArchSpecific {
    pub fixup_start: *mut M68kFixupInfo,
    pub fixup_end: *mut M68kFixupInfo,
}

// CONFIG_MMU conditional declarations from the source header.
#[cfg(CONFIG_MMU)]
#[macro_export]
macro_rules! MODULE_ARCH_INIT {
    () => {
        $crate::ModArchSpecific {
            fixup_start: unsafe { $crate::__start_fixup.as_mut_ptr() },
            fixup_end: unsafe { $crate::__stop_fixup.as_mut_ptr() },
        }
    };
}

#[cfg(CONFIG_MMU)]
#[macro_export]
macro_rules! m68k_fixup {
    ($type:ident, $addr:ident) => {
        concat!(
            "\t.section \".m68k_fixup\",\"aw\"\n",
            "\t.long ", stringify!($type), ",", stringify!($addr), "\n",
            "\t.previous\n"
        )
    };
}

extern "C" {
    pub static mut __start_fixup: [M68kFixupInfo; 0];
    pub static mut __stop_fixup: [M68kFixupInfo; 0];
}

// Declaration equivalent to `struct module` from the surrounding headers.
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

extern "C" {
    pub fn module_fixup(
        mod_: *mut module,
        start: *mut M68kFixupInfo,
        end: *mut M68kFixupInfo,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
