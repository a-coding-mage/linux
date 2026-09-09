// SPDX-License-Identifier: GPL-2.0
//
// Declarations and constants supplied by ubifs.h and related dependencies are
// intentionally referenced here rather than reimplemented.

unsafe fn ubifs_crypt_get_context(
    inode: *mut inode,
    ctx: *mut core::ffi::c_void,
    len: usize,
) -> i32 {
    ubifs_xattr_get(inode, UBIFS_XATTR_NAME_ENCRYPTION_CONTEXT, ctx, len)
}

unsafe fn ubifs_crypt_set_context(
    inode: *mut inode,
    ctx: *const core::ffi::c_void,
    len: usize,
    _fs_data: *mut core::ffi::c_void,
) -> i32 {
    /*
     * Creating an encryption context is done unlocked since we
     * operate on a new inode which is not visible to other users
     * at this point. So, no need to check whether inode is locked.
     */
    ubifs_xattr_set(
        inode,
        UBIFS_XATTR_NAME_ENCRYPTION_CONTEXT,
        ctx,
        len,
        0,
        false,
    )
}

unsafe fn ubifs_crypt_empty_dir(inode: *mut inode) -> bool {
    ubifs_check_dir_empty(inode) == 0
}

/**
 * ubifs_encrypt - Encrypt data.
 * @inode: inode which refers to the data node
 * @dn: data node to encrypt
 * @in_len: length of data to be compressed
 * @out_len: allocated memory size for the data area of @dn
 * @block: logical block number of the block
 *
 * This function encrypt a possibly-compressed data in the data node.
 * The encrypted data length will store in @out_len.
 */
pub unsafe fn ubifs_encrypt(
    inode: *const inode,
    dn: *mut ubifs_data_node,
    in_len: u32,
    out_len: *mut u32,
    block: i32,
) -> i32 {
    let c = (*(*inode).i_sb).s_fs_info;
    let p = core::ptr::addr_of_mut!((*dn).data) as *mut u8;
    let pad_len = round_up(in_len, UBIFS_CIPHER_BLOCK_SIZE);
    let mut err: i32;

    ubifs_assert(c, pad_len <= *out_len);
    (*dn).compr_size = cpu_to_le16(in_len as _);

    /* pad to full block cipher length */
    if pad_len != in_len {
        memset(
            p.add(in_len as usize) as *mut core::ffi::c_void,
            0,
            (pad_len - in_len) as usize,
        );
    }

    err = fscrypt_encrypt_block_inplace(
        inode,
        virt_to_page(p as *mut core::ffi::c_void),
        pad_len,
        offset_in_page(p as *mut core::ffi::c_void),
        block,
    );
    if err != 0 {
        ubifs_err(c, "fscrypt_encrypt_block_inplace() failed: %d", err);
        return err;
    }
    *out_len = pad_len;

    0
}

pub unsafe fn ubifs_decrypt(
    inode: *const inode,
    dn: *mut ubifs_data_node,
    out_len: *mut u32,
    block: i32,
) -> i32 {
    let c = (*(*inode).i_sb).s_fs_info;
    let mut err: i32;
    let clen = le16_to_cpu((*dn).compr_size);
    let dlen = *out_len;

    if clen <= 0 || clen > UBIFS_BLOCK_SIZE || clen > dlen {
        ubifs_err(c, "bad compr_size: %i", clen);
        return -EINVAL;
    }

    ubifs_assert(c, dlen <= UBIFS_BLOCK_SIZE);
    err = fscrypt_decrypt_block_inplace(
        inode,
        virt_to_page(core::ptr::addr_of_mut!((*dn).data) as *mut core::ffi::c_void),
        dlen,
        offset_in_page(core::ptr::addr_of_mut!((*dn).data) as *mut core::ffi::c_void),
        block,
    );
    if err != 0 {
        ubifs_err(c, "fscrypt_decrypt_block_inplace() failed: %d", err);
        return err;
    }
    *out_len = clen;

    0
}

pub static ubifs_crypt_operations: fscrypt_operations = fscrypt_operations {
    inode_info_offs: core::mem::offset_of!(ubifs_inode, i_crypt_info) as i32
        - core::mem::offset_of!(ubifs_inode, vfs_inode) as i32,
    legacy_key_prefix: "ubifs:\0",
    get_context: Some(ubifs_crypt_get_context),
    set_context: Some(ubifs_crypt_set_context),
    empty_dir: Some(ubifs_crypt_empty_dir),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
