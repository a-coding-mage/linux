// SPDX-License-Identifier: GPL-2.0
//! Common, data-only metadata for loadable kernel modules.
//!
//! Configuration and architecture values come from the unchanged authoritative
//! headers. Their C frontend validation emits no object or runtime glue.
#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod data {
    include!(env!("MODULE_COMMON_DATA"));
}

/// ELF32 and ELF64 notes both have three native-endian 32-bit header words.
/// The six-byte owner name and descriptor are independently four-byte aligned.
/// `D` includes explicit zero padding, but `descsz` does not.
#[repr(C, align(4))]
struct LinuxNote<const D: usize> {
    namesz: u32,
    descsz: u32,
    kind: u32,
    name: [u8; 8],
    desc: [u8; D],
}

impl<const D: usize> LinuxNote<D> {
    const fn new(kind: u32, value: &[u8]) -> Self {
        assert!(D % 4 == 0 && value.len() <= D && value.len() <= u32::MAX as usize);
        let mut desc = [0; D];
        let mut index = 0;
        while index < value.len() {
            desc[index] = value[index];
            index += 1;
        }
        Self {
            namesz: 6,
            descsz: value.len() as u32,
            kind,
            name: *b"Linux\0\0\0",
            desc,
        }
    }
}

#[used]
#[link_section = ".note.Linux"]
static BUILD_SALT: LinuxNote<{ (data::BUILD_SALT.len() + 3) & !3 }> =
    LinuxNote::new(0x100, &data::BUILD_SALT);

#[used]
#[link_section = ".note.Linux"]
static BUILD_LTO_INFO: LinuxNote<4> = LinuxNote::new(0x101, &data::LTO.to_ne_bytes());

#[used]
#[link_section = ".modinfo"]
static VERMAGIC: [u8; data::VERMAGIC.len()] = data::VERMAGIC;

#[cfg(CONFIG_MITIGATION_RETPOLINE)]
#[used]
#[link_section = ".modinfo"]
static RETPOLINE: [u8; 12] = *b"retpoline=Y\0";

#[cfg(CONFIG_UNWINDER_ORC)]
#[repr(C, align(4))]
struct OrcHeader([u8; 20]);

#[cfg(CONFIG_UNWINDER_ORC)]
#[used]
#[link_section = ".orc_header"]
static ORC_HEADER: OrcHeader = OrcHeader(data::ORC_HASH);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
