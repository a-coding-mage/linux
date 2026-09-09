// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2023 Google LLC
// Authors: Ard Biesheuvel <ardb@google.com>
//          Peter Collingbourne <peter@pcc.me.uk>

// Dependencies corresponding to the Linux ELF, init, types, and pi.h includes
// are supplied by the surrounding translation unit.

extern "C" {
    static rela_start: [Elf64_Rela; 0];
    static rela_end: [Elf64_Rela; 0];
    static relr_start: [u64; 0];
    static relr_end: [u64; 0];
}

pub unsafe fn relocate_kernel(offset: u64) {
    let mut place: *mut u64 = core::ptr::null_mut();

    let mut rela = rela_start.as_ptr();
    while rela < rela_end.as_ptr() {
        if ELF64_R_TYPE((*rela).r_info) != R_AARCH64_RELATIVE {
            rela = rela.add(1);
            continue;
        }
        *(((*rela).r_offset.wrapping_add(offset)) as *mut u64) =
            (*rela).r_addend.wrapping_add(offset);
        rela = rela.add(1);
    }

    // Equivalent to: if (!IS_ENABLED(CONFIG_RELR) || !offset)
    #[cfg(not(feature = "CONFIG_RELR"))]
    {
        return;
    }
    if offset == 0 {
        return;
    }

    /*
     * Apply RELR relocations.
     *
     * RELR is a compressed format for storing relative relocations. The
     * encoded sequence of entries looks like:
     * [ AAAAAAAA BBBBBBB1 BBBBBBB1 ... AAAAAAAA BBBBBB1 ... ]
     *
     * i.e. start with an address, followed by any number of bitmaps. The
     * address entry encodes 1 relocation. The subsequent bitmap entries
     * encode up to 63 relocations each, at subsequent offsets following the
     * last address entry.
     *
     * The bitmap entries must have 1 in the least significant bit. The
     * assumption here is that an address cannot have 1 in lsb. Odd
     * addresses are not supported. Any odd addresses are stored in the RELA
     * section, which is handled above.
     *
     * With the exception of the least significant bit, each bit in the
     * bitmap corresponds with a machine word that follows the base address
     * word, and the bit value indicates whether or not a relocation needs to
     * be applied to it. The second least significant bit represents the
     * machine word immediately following the initial address, and each bit
     * that follows represents the next word, in linear order. As such, a
     * single bitmap can encode up to 63 relocations in a 64-bit object.
     */
    let mut relr = relr_start.as_ptr();
    while relr < relr_end.as_ptr() {
        if (*relr & 1) == 0 {
            place = (*relr).wrapping_add(offset) as *mut u64;
            *place = (*place).wrapping_add(offset);
            place = place.add(1);
        } else {
            let mut p = place;
            let mut r = *relr >> 1;
            while r != 0 {
                if (r & 1) != 0 {
                    *p = (*p).wrapping_add(offset);
                }
                p = p.add(1);
                r >>= 1;
            }
            place = place.add(63);
        }
        relr = relr.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
