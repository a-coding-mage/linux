// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding kernel/Btrfs translation.

#[repr(C)]
struct BtrfsCsums {
    size: u16,
    name: [u8; 10],
}

static BTRFS_CSUMS: &[BtrfsCsums] = &[
    BtrfsCsums { size: 4, name: *b"crc32c\0\0\0\0" },
    BtrfsCsums { size: 8, name: *b"xxhash64\0\0" },
    BtrfsCsums { size: 32, name: *b"sha256\0\0\0\0" },
    BtrfsCsums { size: 32, name: *b"blake2b\0\0\0" },
];

pub unsafe fn btrfs_csum_type_size(type_: u16) -> u16 { BTRFS_CSUMS[type_ as usize].size }

pub unsafe fn btrfs_super_csum_size(s: *const btrfs_super_block) -> i32 {
    let t = btrfs_super_csum_type(s);
    btrfs_csum_type_size(t) as i32
}

pub unsafe fn btrfs_super_csum_name(csum_type: u16) -> *const i8 {
    BTRFS_CSUMS[csum_type as usize].name.as_ptr() as *const i8
}

pub const fn btrfs_get_num_csums() -> usize { BTRFS_CSUMS.len() }

pub unsafe fn btrfs_csum(csum_type: u16, data: *const u8, len: usize, out: *mut u8) {
    match csum_type {
        BTRFS_CSUM_TYPE_CRC32 => put_unaligned_le32(!crc32c(!0, data, len), out),
        BTRFS_CSUM_TYPE_XXHASH => put_unaligned_le64(xxh64(data, len, 0), out),
        BTRFS_CSUM_TYPE_SHA256 => sha256(data, len, out),
        BTRFS_CSUM_TYPE_BLAKE2 => blake2b(core::ptr::null(), 0, data, len, out, 32),
        _ => BUG(),
    }
}

pub unsafe fn btrfs_csum_init(ctx: *mut btrfs_csum_ctx, csum_type: u16) {
    (*ctx).csum_type = csum_type;
    match (*ctx).csum_type {
        BTRFS_CSUM_TYPE_CRC32 => (*ctx).crc32 = !0,
        BTRFS_CSUM_TYPE_XXHASH => xxh64_reset(&mut (*ctx).xxh64, 0),
        BTRFS_CSUM_TYPE_SHA256 => sha256_init(&mut (*ctx).sha256),
        BTRFS_CSUM_TYPE_BLAKE2 => blake2b_init(&mut (*ctx).blake2b, 32),
        _ => BUG(),
    }
}

pub unsafe fn btrfs_csum_update(ctx: *mut btrfs_csum_ctx, data: *const u8, len: usize) {
    match (*ctx).csum_type {
        BTRFS_CSUM_TYPE_CRC32 => (*ctx).crc32 = crc32c((*ctx).crc32, data, len),
        BTRFS_CSUM_TYPE_XXHASH => xxh64_update(&mut (*ctx).xxh64, data, len),
        BTRFS_CSUM_TYPE_SHA256 => sha256_update(&mut (*ctx).sha256, data, len),
        BTRFS_CSUM_TYPE_BLAKE2 => blake2b_update(&mut (*ctx).blake2b, data, len),
        _ => BUG(),
    }
}

pub unsafe fn btrfs_csum_final(ctx: *mut btrfs_csum_ctx, out: *mut u8) {
    match (*ctx).csum_type {
        BTRFS_CSUM_TYPE_CRC32 => put_unaligned_le32(!(*ctx).crc32, out),
        BTRFS_CSUM_TYPE_XXHASH => put_unaligned_le64(xxh64_digest(&(*ctx).xxh64), out),
        BTRFS_CSUM_TYPE_SHA256 => sha256_final(&mut (*ctx).sha256, out),
        BTRFS_CSUM_TYPE_BLAKE2 => blake2b_final(&mut (*ctx).blake2b, out),
        _ => BUG(),
    }
}

pub const fn btrfs_supported_blocksize(blocksize: u32) -> bool {
    // @blocksize should be validated first.
    if blocksize <= PAGE_SIZE { return true; }
    // CONFIG_BTRFS_EXPERIMENTAL: larger folio support is build-time gated.
    #[cfg(feature = "CONFIG_BTRFS_EXPERIMENTAL")]
    {
        if blocksize > PAGE_SIZE {
            #[cfg(feature = "CONFIG_HIGHMEM")]
            { return false; }
            #[cfg(not(feature = "CONFIG_TRANSPARENT_HUGEPAGE"))]
            { return false; }
        }
        return true;
    }
    false
}

pub unsafe fn btrfs_exclop_start(fs_info: *mut btrfs_fs_info, type_: btrfs_exclusive_operation) -> bool {
    let mut ret = false;
    spin_lock(&mut (*fs_info).super_lock);
    if (*fs_info).exclusive_operation == BTRFS_EXCLOP_NONE { (*fs_info).exclusive_operation = type_; ret = true; }
    spin_unlock(&mut (*fs_info).super_lock);
    ret
}

pub unsafe fn btrfs_exclop_start_try_lock(fs_info: *mut btrfs_fs_info, type_: btrfs_exclusive_operation) -> bool {
    spin_lock(&mut (*fs_info).super_lock);
    if (*fs_info).exclusive_operation == type_ || ((*fs_info).exclusive_operation == BTRFS_EXCLOP_BALANCE_PAUSED && type_ == BTRFS_EXCLOP_DEV_ADD) { return true; }
    spin_unlock(&mut (*fs_info).super_lock); false
}

pub unsafe fn btrfs_exclop_start_unlock(fs_info: *mut btrfs_fs_info) { spin_unlock(&mut (*fs_info).super_lock); }

pub unsafe fn btrfs_exclop_finish(fs_info: *mut btrfs_fs_info) {
    spin_lock(&mut (*fs_info).super_lock); WRITE_ONCE(&mut (*fs_info).exclusive_operation, BTRFS_EXCLOP_NONE); spin_unlock(&mut (*fs_info).super_lock);
    sysfs_notify(&mut (*(*fs_info).fs_devices).fsid_kobj, core::ptr::null(), b"exclusive_operation\0".as_ptr() as *const i8);
}

pub unsafe fn btrfs_exclop_balance(fs_info: *mut btrfs_fs_info, op: btrfs_exclusive_operation) {
    match op {
        BTRFS_EXCLOP_BALANCE_PAUSED => { spin_lock(&mut (*fs_info).super_lock); (*fs_info).exclusive_operation = op; spin_unlock(&mut (*fs_info).super_lock); }
        BTRFS_EXCLOP_BALANCE => { spin_lock(&mut (*fs_info).super_lock); (*fs_info).exclusive_operation = op; spin_unlock(&mut (*fs_info).super_lock); }
        _ => btrfs_warn(fs_info, b"invalid exclop balance operation %d requested\0".as_ptr() as *const i8, op),
    }
}

pub unsafe fn __btrfs_set_fs_incompat(fs_info: *mut btrfs_fs_info, flag: u64, name: *const i8) {
    let disk_super = (*fs_info).super_copy; let mut features = btrfs_super_incompat_flags(disk_super);
    if features & flag == 0 { spin_lock(&mut (*fs_info).super_lock); features = btrfs_super_incompat_flags(disk_super); if features & flag == 0 { features |= flag; btrfs_set_super_incompat_flags(disk_super, features); btrfs_info(fs_info, b"setting incompat feature flag for %s (0x%llx)\0".as_ptr() as *const i8, name, flag); } spin_unlock(&mut (*fs_info).super_lock); set_bit(BTRFS_FS_FEATURE_CHANGED, &mut (*fs_info).flags); }
}

pub unsafe fn __btrfs_clear_fs_incompat(fs_info: *mut btrfs_fs_info, flag: u64, name: *const i8) {
    let disk_super = (*fs_info).super_copy; let mut features = btrfs_super_incompat_flags(disk_super);
    if features & flag != 0 { spin_lock(&mut (*fs_info).super_lock); features = btrfs_super_incompat_flags(disk_super); if features & flag != 0 { features &= !flag; btrfs_set_super_incompat_flags(disk_super, features); btrfs_info(fs_info, b"clearing incompat feature flag for %s (0x%llx)\0".as_ptr() as *const i8, name, flag); } spin_unlock(&mut (*fs_info).super_lock); set_bit(BTRFS_FS_FEATURE_CHANGED, &mut (*fs_info).flags); }
}

pub unsafe fn __btrfs_set_fs_compat_ro(fs_info: *mut btrfs_fs_info, flag: u64, name: *const i8) {
    let disk_super = (*fs_info).super_copy; let mut features = btrfs_super_compat_ro_flags(disk_super);
    if features & flag == 0 { spin_lock(&mut (*fs_info).super_lock); features = btrfs_super_compat_ro_flags(disk_super); if features & flag == 0 { features |= flag; btrfs_set_super_compat_ro_flags(disk_super, features); btrfs_info(fs_info, b"setting compat-ro feature flag for %s (0x%llx)\0".as_ptr() as *const i8, name, flag); } spin_unlock(&mut (*fs_info).super_lock); set_bit(BTRFS_FS_FEATURE_CHANGED, &mut (*fs_info).flags); }
}

pub unsafe fn __btrfs_clear_fs_compat_ro(fs_info: *mut btrfs_fs_info, flag: u64, name: *const i8) {
    let disk_super = (*fs_info).super_copy; let mut features = btrfs_super_compat_ro_flags(disk_super);
    if features & flag != 0 { spin_lock(&mut (*fs_info).super_lock); features = btrfs_super_compat_ro_flags(disk_super); if features & flag != 0 { features &= !flag; btrfs_set_super_compat_ro_flags(disk_super, features); btrfs_info(fs_info, b"clearing compat-ro feature flag for %s (0x%llx)\0".as_ptr() as *const i8, name, flag); } spin_unlock(&mut (*fs_info).super_lock); set_bit(BTRFS_FS_FEATURE_CHANGED, &mut (*fs_info).flags); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
