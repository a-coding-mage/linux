/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of linux/elfnote.h.
 *
 * The original header provides separate assembler and C preprocessor
 * interfaces.  The assembler interface is retained as Rust declarative
 * macros which emit the corresponding assembler directives; the non-
 * assembler interface retains the ELF note layout and section attributes.
 */

/* __ASSEMBLER__ */

/// Generate the beginning of an ELF note assembler section.
#[macro_export]
macro_rules! ELFNOTE_START {
    ($name:ident, $type:expr, $flags:expr) => {
        core::arch::global_asm!(concat!(
            ".pushsection .note.", stringify!($name), ",", $flags, ",@note\\n",
            ".balign 4\\n",
            ".long 2f - 1f\\n",
            ".long 4484f - 3f\\n",
            ".long ", stringify!($type), "\\n",
            "1:.asciz \\\"", stringify!($name), "\\\"\\n",
            "2:.balign 4\\n",
            "3:\\n",
        ));
    };
}

/// Finish an ELF note assembler section.
#[macro_export]
macro_rules! ELFNOTE_END {
    () => {
        core::arch::global_asm!(
            "4484:.balign 4\n.popsection\n"
        );
    };
}

/// Generate an ELF note from assembler data.
#[macro_export]
macro_rules! ELFNOTE_ASM {
    ($name:ident, $type:expr, $desc:expr) => {
        $crate::ELFNOTE_START!($name, $type, "a");
        core::arch::global_asm!(stringify!($desc));
        $crate::ELFNOTE_END!();
    };
}

/* !__ASSEMBLER__
 * The following macros correspond to the C interface.  `elf32_note` and
 * `elf64_note`, `Elf32_Word`, and `Elf64_Word` are supplied by the ELF
 * bindings imported by the containing translation unit.
 */

/// Emit a statically allocated ELF note with the C header's layout.
#[macro_export]
macro_rules! ELFNOTE {
    ($size:literal, $name:expr, $type:expr, $desc:expr) => {
        #[allow(non_upper_case_globals)]
        #[link_section = concat!(".note.", $name)]
        #[used]
        static __ELFNOTE: $crate::ElfNote<$size, { $name.len() }, _> =
            $crate::ElfNote::new($name, $type, $desc);
    };
}

/// Rust representation of the anonymous C ELF note object.
#[repr(C)]
pub struct ElfNote<const SIZE: usize, const NAME_SIZE: usize, Desc> {
    pub nhdr: [u32; 3],
    pub name: [u8; NAME_SIZE],
    pub desc: Desc,
}

impl<const SIZE: usize, const NAME_SIZE: usize, Desc> ElfNote<SIZE, NAME_SIZE, Desc> {
    pub const fn new(name: &[u8; NAME_SIZE], note_type: u32, desc: Desc) -> Self {
        Self {
            nhdr: [NAME_SIZE as u32, core::mem::size_of::<Desc>() as u32, note_type],
            name: *name,
            desc,
        }
    }
}

#[macro_export]
macro_rules! ELFNOTE32 {
    ($name:expr, $type:expr, $desc:expr) => {
        $crate::ELFNOTE!(32, $name, $type, $desc);
    };
}

#[macro_export]
macro_rules! ELFNOTE64 {
    ($name:expr, $type:expr, $desc:expr) => {
        $crate::ELFNOTE!(64, $name, $type, $desc);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
