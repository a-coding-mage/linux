/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008 Michal Simek <monstr@monstr.eu>
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/*
 * Computes the checksum of the TCP/UDP pseudo-header.
 * Returns a 16-bit checksum, already complemented.
 *
 * The C header aliases the macro name to the function name:
 *     #define csum_tcpudp_nofold csum_tcpudp_nofold
 */
#[inline]
pub unsafe fn csum_tcpudp_nofold(
    mut saddr: __be32,
    mut daddr: __be32,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __wsum {
    /*
     * Original MicroBlaze inline assembly:
     *
     * add  sum, sum, saddr
     * addc sum, sum, daddr
     * addc sum, sum, (len + proto) [shifted left by 8 on MicroBlaze LE]
     * addc sum, sum, r0
     *
     * Keep the operation and carry behavior in target assembly.  The
     * conditional operand preserves the original __MICROBLAZEEL__ intent.
     */
    #[cfg(target_endian = "little")]
    {
        let len_proto = (len.wrapping_add(proto as __u32)) << 8;
        core::arch::asm!(
            "add {sum}, {sum}, {saddr}",
            "addc {sum}, {sum}, {daddr}",
            "addc {sum}, {sum}, {len_proto}",
            "addc {sum}, {sum}, r0",
            sum = inout(reg) sum,
            saddr = in(reg) saddr,
            daddr = in(reg) daddr,
            len_proto = in(reg) len_proto,
            options(nostack),
        );
    }
    #[cfg(not(target_endian = "little"))]
    {
        let len_proto = len.wrapping_add(proto as __u32);
        core::arch::asm!(
            "add {sum}, {sum}, {saddr}",
            "addc {sum}, {sum}, {daddr}",
            "addc {sum}, {sum}, {len_proto}",
            "addc {sum}, {sum}, r0",
            sum = inout(reg) sum,
            saddr = in(reg) saddr,
            daddr = in(reg) daddr,
            len_proto = in(reg) len_proto,
            options(nostack),
        );
    }
    sum
}

/* Declarations supplied by the included asm-generic checksum definitions. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
