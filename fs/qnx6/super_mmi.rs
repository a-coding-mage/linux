// SPDX-License-Identifier: GPL-2.0
/*
 * QNX6 file system, Linux implementation.
 *
 * Version : 1.0.0
 *
 * History :
 *
 * 01-02-2012 by Kai Bankett (chaosman@ontika.net) : first release.
 *
 */

// Linux kernel headers and qnx6.h provide the types, constants, macros, and
// functions referenced below.

unsafe fn qnx6_mmi_copy_sb(
    qsb: *mut qnx6_super_block,
    sb: *mut qnx6_mmi_super_block,
) {
    (*qsb).sb_magic = (*sb).sb_magic;
    (*qsb).sb_checksum = (*sb).sb_checksum;
    (*qsb).sb_serial = (*sb).sb_serial;
    (*qsb).sb_blocksize = (*sb).sb_blocksize;
    (*qsb).sb_num_inodes = (*sb).sb_num_inodes;
    (*qsb).sb_free_inodes = (*sb).sb_free_inodes;
    (*qsb).sb_num_blocks = (*sb).sb_num_blocks;
    (*qsb).sb_free_blocks = (*sb).sb_free_blocks;

    /* the rest of the superblock is the same */
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*sb).Inode) as *const u8,
        core::ptr::addr_of_mut!((*qsb).Inode) as *mut u8,
        core::mem::size_of_val(&(*sb).Inode),
    );
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*sb).Bitmap) as *const u8,
        core::ptr::addr_of_mut!((*qsb).Bitmap) as *mut u8,
        core::mem::size_of_val(&(*sb).Bitmap),
    );
    core::ptr::copy_nonoverlapping(
        core::ptr::addr_of!((*sb).Longfile) as *const u8,
        core::ptr::addr_of_mut!((*qsb).Longfile) as *mut u8,
        core::mem::size_of_val(&(*sb).Longfile),
    );
}

unsafe fn qnx6_mmi_fill_super(s: *mut super_block, silent: i32) -> *mut qnx6_super_block {
    let mut bh1: *mut buffer_head;
    let mut bh2: *mut buffer_head = core::ptr::null_mut();
    let mut sb1: *mut qnx6_mmi_super_block;
    let mut sb2: *mut qnx6_mmi_super_block;
    let mut qsb: *mut qnx6_super_block = core::ptr::null_mut();
    let sbi: *mut qnx6_sb_info;
    let mut offset: u64;

    macro_rules! goto_out {
        () => {{
            if !bh1.is_null() { brelse(bh1); }
            if !bh2.is_null() { brelse(bh2); }
            return core::ptr::null_mut();
        }};
    }

    /* Check the superblock signatures
       start with the first superblock */
    bh1 = sb_bread(s, 0);
    if bh1.is_null() {
        pr_err!("Unable to read first mmi superblock\n");
        return core::ptr::null_mut();
    }
    sb1 = (*bh1).b_data as *mut qnx6_mmi_super_block;
    sbi = QNX6_SB!(s);
    if fs32_to_cpu!(sbi, (*sb1).sb_magic) != QNX6_SUPER_MAGIC {
        if silent == 0 {
            pr_err!("wrong signature (magic) in superblock #1.\n");
            goto_out!();
        }
    }
    

    /* checksum check - start at byte 8 and end at byte 512 */
    if fs32_to_cpu!(sbi, (*sb1).sb_checksum)
        != crc32_be(0, (*bh1).b_data.add(8) as *const i8, 504)
    {
        pr_err!("superblock #1 checksum error\n");
        goto_out!();
    }

    /* calculate second superblock blocknumber */
    offset = fs32_to_cpu!(sbi, (*sb1).sb_num_blocks) as u64
        + QNX6_SUPERBLOCK_AREA as u64 / fs32_to_cpu!(sbi, (*sb1).sb_blocksize) as u64;

    /* set new blocksize */
    if sb_set_blocksize(s, fs32_to_cpu!(sbi, (*sb1).sb_blocksize)) == 0 {
        pr_err!("unable to set blocksize\n");
        goto_out!();
    }
    /* blocksize invalidates bh - pull it back in */
    brelse(bh1);
    bh1 = sb_bread(s, 0);
    if bh1.is_null() {
        goto_out!();
    }
    sb1 = (*bh1).b_data as *mut qnx6_mmi_super_block;

    /* read second superblock */
    bh2 = sb_bread(s, offset);
    if bh2.is_null() {
        pr_err!("unable to read the second superblock\n");
        goto_out!();
    }
    sb2 = (*bh2).b_data as *mut qnx6_mmi_super_block;
    if fs32_to_cpu!(sbi, (*sb2).sb_magic) != QNX6_SUPER_MAGIC {
        if silent == 0 {
            pr_err!("wrong signature (magic) in superblock #2.\n");
        }
        goto_out!();
    }
    if fs32_to_cpu!(sbi, (*sb2).sb_checksum)
        != crc32_be(0, (*bh2).b_data.add(8) as *const i8, 504)
    {
        pr_err!("superblock #1 checksum error\n");
        goto_out!();
    }
    qsb = kmalloc_obj!(*qsb);
    if qsb.is_null() {
        pr_err!("unable to allocate memory.\n");
        goto_out!();
    }
    if fs64_to_cpu!(sbi, (*sb1).sb_serial) > fs64_to_cpu!(sbi, (*sb2).sb_serial) {
        /* superblock #1 active */
        qnx6_mmi_copy_sb(qsb, sb1);
        #[cfg(feature = "CONFIG_QNX6FS_DEBUG")]
        qnx6_superblock_debug(qsb, s);
        core::ptr::copy_nonoverlapping(
            qsb as *const u8,
            (*bh1).b_data,
            core::mem::size_of::<qnx6_super_block>(),
        );
        (*sbi).sb_buf = bh1;
        (*sbi).sb = (*bh1).b_data as *mut qnx6_super_block;
        brelse(bh2);
        pr_info!("superblock #1 active\n");
    } else {
        /* superblock #2 active */
        qnx6_mmi_copy_sb(qsb, sb2);
        #[cfg(feature = "CONFIG_QNX6FS_DEBUG")]
        qnx6_superblock_debug(qsb, s);
        core::ptr::copy_nonoverlapping(
            qsb as *const u8,
            (*bh2).b_data,
            core::mem::size_of::<qnx6_super_block>(),
        );
        (*sbi).sb_buf = bh2;
        (*sbi).sb = (*bh2).b_data as *mut qnx6_super_block;
        brelse(bh1);
        pr_info!("superblock #2 active\n");
    }
    kfree(qsb as *mut core::ffi::c_void);

    /* offset for mmi_fs is just SUPERBLOCK_AREA bytes */
    (*sbi).s_blks_off = QNX6_SUPERBLOCK_AREA / (*s).s_blocksize;

    /* success */
    (*sbi).sb
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
