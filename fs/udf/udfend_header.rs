/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent from the original header:
// #include <asm/byteorder.h>
// #include <linux/string.h>

#[inline]
fn lelb_to_cpu(in_: lb_addr) -> kernel_lb_addr {
    let mut out: kernel_lb_addr;

    out.logicalBlockNum = le32_to_cpu(in_.logicalBlockNum);
    out.partitionReferenceNum = le16_to_cpu(in_.partitionReferenceNum);

    out
}

#[inline]
fn cpu_to_lelb(in_: kernel_lb_addr) -> lb_addr {
    let mut out: lb_addr;

    out.logicalBlockNum = cpu_to_le32(in_.logicalBlockNum);
    out.partitionReferenceNum = cpu_to_le16(in_.partitionReferenceNum);

    out
}

#[inline]
fn lesa_to_cpu(in_: short_ad) -> short_ad {
    let mut out: short_ad;

    out.extLength = le32_to_cpu(in_.extLength);
    out.extPosition = le32_to_cpu(in_.extPosition);

    out
}

#[inline]
fn cpu_to_lesa(in_: short_ad) -> short_ad {
    let mut out: short_ad;

    out.extLength = cpu_to_le32(in_.extLength);
    out.extPosition = cpu_to_le32(in_.extPosition);

    out
}

#[inline]
fn lela_to_cpu(in_: long_ad) -> kernel_long_ad {
    let mut out: kernel_long_ad;

    out.extLength = le32_to_cpu(in_.extLength);
    out.extLocation = lelb_to_cpu(in_.extLocation);

    out
}

#[inline]
fn cpu_to_lela(in_: kernel_long_ad) -> long_ad {
    let mut out: long_ad;

    out.extLength = cpu_to_le32(in_.extLength);
    out.extLocation = cpu_to_lelb(in_.extLocation);

    out
}

#[inline]
fn leea_to_cpu(in_: extent_ad) -> kernel_extent_ad {
    let mut out: kernel_extent_ad;

    out.extLength = le32_to_cpu(in_.extLength);
    out.extLocation = le32_to_cpu(in_.extLocation);

    out
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
