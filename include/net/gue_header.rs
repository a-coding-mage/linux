/* SPDX-License-Identifier: GPL-2.0 */

/* Definitions for the GUE header, standard and private flags, lengths
 * of optional fields are below.
 *
 * The bit-field ordering of the first byte follows the target's
 * __LITTLE_ENDIAN_BITFIELD or __BIG_ENDIAN_BITFIELD configuration.
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guehdr_fields {
    /* hlen:5, control:1, version:2 on little endian;
     * version:2, control:1, hlen:5 on big endian. */
    pub first: u8,
    pub proto_ctype: u8,
    pub flags: u16,
}

#[repr(C)]
pub union guehdr {
    pub fields: guehdr_fields,
    pub word: u32,
}

/* Standard flags in GUE header */

pub const GUE_FLAG_PRIV: u16 = (1u16).to_be(); /* Private flags are in options */
pub const GUE_LEN_PRIV: usize = 4;

pub const GUE_FLAGS_ALL: u16 = GUE_FLAG_PRIV;

/* Private flags in the private option extension */

pub const GUE_PFLAG_REMCSUM: u32 = (1u32 << 31).to_be();
pub const GUE_PLEN_REMCSUM: usize = 4;

pub const GUE_PFLAGS_ALL: u32 = GUE_PFLAG_REMCSUM;

/* Functions to compute options length corresponding to flags.
 * If we ever have a lot of flags this can be potentially be converted to a
 * more optimized algorithm (table lookup for instance).
 */
#[inline]
pub fn guehdr_flags_len(flags: u16) -> usize {
    if flags & GUE_FLAG_PRIV != 0 {
        GUE_LEN_PRIV
    } else {
        0
    }
}

#[inline]
pub fn guehdr_priv_flags_len(flags: u32) -> usize {
    if flags & GUE_PFLAG_REMCSUM != 0 {
        GUE_PLEN_REMCSUM
    } else {
        0
    }
}

/* Validate standard and private flags. Returns non-zero (meaning invalid)
 * if there is an unknown standard or private flags, or the options length for
 * the flags exceeds the options length specific in hlen of the GUE header.
 */
#[inline]
pub unsafe fn validate_gue_flags(guehdr: *mut guehdr, optlen: usize) -> i32 {
    let flags = (*guehdr).fields.flags;
    let mut len: usize;

    if flags & !GUE_FLAGS_ALL != 0 {
        return 1;
    }

    len = guehdr_flags_len(flags);
    if len > optlen {
        return 1;
    }

    if flags & GUE_FLAG_PRIV != 0 {
        /* Private flags are last four bytes accounted in
         * guehdr_flags_len
         */
        let pflags = *((guehdr as *const u8).add(core::mem::size_of::<guehdr>()
            + len - GUE_LEN_PRIV) as *const u32);

        if pflags & !GUE_PFLAGS_ALL != 0 {
            return 1;
        }

        len += guehdr_priv_flags_len(pflags);
        if len > optlen {
            return 1;
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
