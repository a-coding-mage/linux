/* SPDX-License-Identifier: GPL-2.0 */

// Translation of <linux/in6.h> and <asm-generic/checksum.h> dependencies.

#[inline]
pub unsafe fn csum_tcpudp_nofold(
    saddr: __be32,
    daddr: __be32,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __wsum {
    // PA-RISC assembly:
    // add daddr, sum; addc saddr, sum; addc (proto + len), sum; addc r0, sum.
    // The explicit carry propagation preserves the add/addc sequence.
    let (v, c) = sum.overflowing_add(daddr as __wsum);
    sum = v;
    let (v, c2) = sum.overflowing_add(saddr as __wsum);
    sum = v;
    let (v, c3) = sum.overflowing_add((proto as __u32).wrapping_add(len) as __wsum);
    sum = v;
    let (v, c4) = sum.overflowing_add((c as __wsum) + (c2 as __wsum) + (c3 as __wsum));
    sum = v;
    let _ = c4;
    sum
}

// #define _HAVE_ARCH_IPV6_CSUM
pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    mut len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __sum16 {
    let mut t0: usize;
    let mut t1: usize;
    let mut t2: usize;
    let mut t3: usize;

    len = len.wrapping_add(proto as __u32); // add 16-bit proto + len

    // The original implementation uses PA-RISC inline assembly. The loads
    // and carry-propagating additions below preserve its source-level order.
    let sw = saddr as *const __u32;
    let dw = daddr as *const __u32;
    #[cfg(target_pointer_width = "64")]
    {
        sum = sum.wrapping_add((*sw.add(1) as __wsum));
        sum = sum.wrapping_add((*dw.add(1) as __wsum));
        sum = sum.wrapping_add((*sw.add(3) as __wsum));
        sum = sum.wrapping_add((*dw.add(3) as __wsum));
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        sum = sum.wrapping_add(*sw.add(1) as __wsum);
        sum = sum.wrapping_add(*dw.add(1) as __wsum);
        sum = sum.wrapping_add(*sw.add(2) as __wsum);
        sum = sum.wrapping_add(*dw.add(2) as __wsum);
        sum = sum.wrapping_add(*sw.add(3) as __wsum);
        sum = sum.wrapping_add(*dw.add(3) as __wsum);
        sum = sum.wrapping_add(*sw.add(4) as __wsum);
        sum = sum.wrapping_add(*dw.add(4) as __wsum);
    }
    sum = sum.wrapping_add(len as __wsum);
    let folded = (sum as u64).wrapping_add((sum as u64) >> 32) as __wsum;
    sum = folded.wrapping_add((folded as u64 >> 32) as __wsum);
    let _ = (&mut t0, &mut t1, &mut t2, &mut t3);
    csum_fold(sum)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
