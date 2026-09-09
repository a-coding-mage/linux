/* SPDX-License-Identifier: GPL-2.0 */
/*
 * linux/fs/befs/endian.h
 *
 * Copyright (C) 2001 Will Dyson <will_dyson@pobox.com>
 *
 * Partially based on similar funtions in the sysv driver.
 */

// #include <asm/byteorder.h>

pub unsafe fn fs64_to_cpu(sb: *const super_block, n: fs64) -> u64 {
    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        le64_to_cpu(n as __le64)
    } else {
        be64_to_cpu(n as __be64)
    }
}

pub unsafe fn cpu_to_fs64(sb: *const super_block, n: u64) -> fs64 {
    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        cpu_to_le64(n) as fs64
    } else {
        cpu_to_be64(n) as fs64
    }
}

pub unsafe fn fs32_to_cpu(sb: *const super_block, n: fs32) -> u32 {
    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        le32_to_cpu(n as __le32)
    } else {
        be32_to_cpu(n as __be32)
    }
}

pub unsafe fn cpu_to_fs32(sb: *const super_block, n: u32) -> fs32 {
    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        cpu_to_le32(n) as fs32
    } else {
        cpu_to_be32(n) as fs32
    }
}

pub unsafe fn fs16_to_cpu(sb: *const super_block, n: fs16) -> u16 {
    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        le16_to_cpu(n as __le16)
    } else {
        be16_to_cpu(n as __be16)
    }
}

pub unsafe fn cpu_to_fs16(sb: *const super_block, n: u16) -> fs16 {
    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        cpu_to_le16(n) as fs16
    } else {
        cpu_to_be16(n) as fs16
    }
}

/* Composite types below here */

pub unsafe fn fsrun_to_cpu(
    sb: *const super_block,
    n: befs_disk_block_run,
) -> befs_block_run {
    let mut run: befs_block_run;

    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        run.allocation_group = le32_to_cpu(n.allocation_group as __le32);
        run.start = le16_to_cpu(n.start as __le16);
        run.len = le16_to_cpu(n.len as __le16);
    } else {
        run.allocation_group = be32_to_cpu(n.allocation_group as __be32);
        run.start = be16_to_cpu(n.start as __be16);
        run.len = be16_to_cpu(n.len as __be16);
    }
    run
}

pub unsafe fn cpu_to_fsrun(
    sb: *const super_block,
    n: befs_block_run,
) -> befs_disk_block_run {
    let mut run: befs_disk_block_run;

    if (*BEFS_SB(sb)).byte_order == BEFS_BYTESEX_LE {
        run.allocation_group = cpu_to_le32(n.allocation_group);
        run.start = cpu_to_le16(n.start);
        run.len = cpu_to_le16(n.len);
    } else {
        run.allocation_group = cpu_to_be32(n.allocation_group);
        run.start = cpu_to_be16(n.start);
        run.len = cpu_to_be16(n.len);
    }
    run
}

pub unsafe fn fsds_to_cpu(
    sb: *const super_block,
    n: *const befs_disk_data_stream,
) -> befs_data_stream {
    let mut data: befs_data_stream;
    let mut i: i32;

    i = 0;
    while i < BEFS_NUM_DIRECT_BLOCKS {
        data.direct[i as usize] = fsrun_to_cpu(sb, (*n).direct[i as usize]);
        i += 1;
    }

    data.max_direct_range = fs64_to_cpu(sb, (*n).max_direct_range);
    data.indirect = fsrun_to_cpu(sb, (*n).indirect);
    data.max_indirect_range = fs64_to_cpu(sb, (*n).max_indirect_range);
    data.double_indirect = fsrun_to_cpu(sb, (*n).double_indirect);
    data.max_double_indirect_range = fs64_to_cpu(sb, (*n).max_double_indirect_range);
    data.size = fs64_to_cpu(sb, (*n).size);

    data
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
