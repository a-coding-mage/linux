/*
 * Licensed under the GPL
 */

#[inline]
pub unsafe fn ip_compute_csum(buff: *const core::ffi::c_void, len: i32) -> __sum16 {
    csum_fold(csum_partial(buff, len, 0))
}

// _HAVE_ARCH_IPV6_CSUM
pub const _HAVE_ARCH_IPV6_CSUM: bool = true;

#[inline]
pub unsafe fn csum_ipv6_magic(
    saddr: *const in6_addr,
    daddr: *const in6_addr,
    len: __u32,
    proto: __u8,
    mut sum: __wsum,
) -> __sum16 {
    core::arch::asm!(
        "addl 0({saddr}), {sum}",
        "adcl 4({saddr}), {sum}",
        "adcl 8({saddr}), {sum}",
        "adcl 12({saddr}), {sum}",
        "adcl 0({daddr}), {sum}",
        "adcl 4({daddr}), {sum}",
        "adcl 8({daddr}), {sum}",
        "adcl 12({daddr}), {sum}",
        "adcl {len}, {sum}",
        "adcl {proto}, {sum}",
        "adcl $0, {sum}",
        saddr = in(reg) saddr,
        daddr = in(reg) daddr,
        len = in(reg) htonl(len),
        proto = in(reg) htonl(proto),
        sum = inout(reg) sum,
        options(nostack),
    );

    csum_fold(sum)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
