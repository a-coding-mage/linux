// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NTFS multi sector transfer protection handling code.
 *
 * Copyright (c) 2001-2004 Anton Altaparmakov
 */

/* Linux ratelimit declarations and ntfs declarations are supplied externally. */

/*
 * post_read_mst_fixup - deprotect multi sector transfer protected data
 * @b:      pointer to the data to deprotect
 * @size:   size in bytes of @b
 */
pub unsafe extern "C" fn post_read_mst_fixup(b: *mut ntfs_record, size: u32) -> i32 {
    let usa_ofs: u16 = le16_to_cpu((*b).usa_ofs);
    let mut usa_count: u16 = le16_to_cpu((*b).usa_count).wrapping_sub(1);

    if (size & (NTFS_BLOCK_SIZE - 1)) != 0
        || (usa_ofs & 1) != 0
        || (usa_ofs as u32).wrapping_add((usa_count as u32) * 2) > size
        || (size >> NTFS_BLOCK_SIZE_BITS) != usa_count as u32
    {
        return 0;
    }

    let mut usa_pos = (b as *mut u16).add((usa_ofs as usize) / core::mem::size_of::<u16>());
    let usn: u16 = *usa_pos;
    let mut data_pos = (b as *mut u16).add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<u16>() - 1);

    while usa_count != 0 {
        usa_count = usa_count.wrapping_sub(1);
        if *data_pos != usn {
            let m = b as *mut mft_record;
            pr_err_ratelimited!("ntfs: Incomplete multi sector transfer detected! (Record magic : 0x%x, mft number : 0x%x, base mft number : 0x%lx, mft in use : %d, data : 0x%x, usn 0x%x)\n",
                le32_to_cpu((*m).magic), le32_to_cpu((*m).mft_record_number),
                MREF_LE((*m).base_mft_record), (*m).flags & MFT_RECORD_IN_USE,
                *data_pos, usn);
            (*b).magic = magic_BAAD;
            return -EINVAL;
        }
        data_pos = data_pos.add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<u16>());
    }

    usa_count = le16_to_cpu((*b).usa_count).wrapping_sub(1);
    data_pos = (b as *mut u16).add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<u16>() - 1);
    while usa_count != 0 {
        usa_count = usa_count.wrapping_sub(1);
        usa_pos = usa_pos.add(1);
        *data_pos = *usa_pos;
        data_pos = data_pos.add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<u16>());
    }
    0
}

pub unsafe extern "C" fn pre_write_mst_fixup(b: *mut ntfs_record, size: u32) -> i32 {
    if b.is_null() || ntfs_is_baad_record((*b).magic) || ntfs_is_hole_record((*b).magic) {
        return -EINVAL;
    }
    let usa_ofs: u16 = le16_to_cpu((*b).usa_ofs);
    let mut usa_count: u16 = le16_to_cpu((*b).usa_count).wrapping_sub(1);
    if (size & (NTFS_BLOCK_SIZE - 1)) != 0
        || (usa_ofs & 1) != 0
        || (usa_ofs as u32).wrapping_add((usa_count as u32) * 2) > size
        || (size >> NTFS_BLOCK_SIZE_BITS) != usa_count as u32
    {
        return -EINVAL;
    }
    let mut usa_pos = (b as *mut u8).add(usa_ofs as usize) as *mut __le16;
    let mut usn: u16 = le16_to_cpup(usa_pos).wrapping_add(1);
    if usn == 0xffff || usn == 0 { usn = 1; }
    let le_usn: __le16 = cpu_to_le16(usn);
    *usa_pos = le_usn;
    let mut data_pos = (b as *mut __le16).add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<__le16>() - 1);
    while usa_count != 0 {
        usa_count = usa_count.wrapping_sub(1);
        usa_pos = usa_pos.add(1);
        *usa_pos = *data_pos;
        *data_pos = le_usn;
        data_pos = data_pos.add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<__le16>());
    }
    0
}

pub unsafe extern "C" fn post_write_mst_fixup(b: *mut ntfs_record) {
    let usa_ofs: u16 = le16_to_cpu((*b).usa_ofs);
    let mut usa_count: u16 = le16_to_cpu((*b).usa_count).wrapping_sub(1);
    let mut usa_pos = (b as *mut __le16).add((usa_ofs as usize) / core::mem::size_of::<__le16>());
    let mut data_pos = (b as *mut __le16).add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<__le16>() - 1);
    while usa_count != 0 {
        usa_count = usa_count.wrapping_sub(1);
        usa_pos = usa_pos.add(1);
        *data_pos = *usa_pos;
        data_pos = data_pos.add(NTFS_BLOCK_SIZE as usize / core::mem::size_of::<__le16>());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
