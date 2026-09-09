/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    Copyright IBM Corp. 2000, 2009
 *    Author(s): Hartmut Penner <hp@de.ibm.com>,
 *               Martin Schwidefsky <schwidefsky@de.ibm.com>,
 *               Christian Ehrhardt <ehrhardt@de.ibm.com>,
 */

// C source dependencies: linux/types.h and linux/jump_label.h.
// The original declaration is excluded for assembler builds.

/// Corresponds to the packed, 8-byte-aligned C bit-field structure.
///
/// The fields occupy, in declaration order, 8, 24, 16, and 16 bits of the
/// underlying 64-bit word.
#[repr(C, align(8))]
#[derive(Copy, Clone)]
pub struct Cpuid {
    pub bits: u64,
}

impl Cpuid {
    #[inline]
    pub const fn new(version: u8, ident: u32, machine: u16, unused: u16) -> Self {
        Self {
            bits: (version as u64)
                | (((ident as u64) & 0x00ff_ffff) << 8)
                | ((machine as u64) << 32)
                | ((unused as u64) << 48),
        }
    }

    #[inline]
    pub const fn version(self) -> u8 {
        (self.bits & 0xff) as u8
    }

    #[inline]
    pub const fn ident(self) -> u32 {
        ((self.bits >> 8) & 0x00ff_ffff) as u32
    }

    #[inline]
    pub const fn machine(self) -> u16 {
        (self.bits >> 32) as u16
    }

    #[inline]
    pub const fn unused(self) -> u16 {
        (self.bits >> 48) as u16
    }
}

// Corresponds to DECLARE_STATIC_KEY_FALSE(cpu_has_bear), whose type is
// supplied by linux/jump_label.h.
extern "C" {
    pub static cpu_has_bear: StaticKeyFalse;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
