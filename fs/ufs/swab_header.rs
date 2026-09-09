/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/fs/ufs/swab.h
 *
 * Notes:
 *    HERE WE ASSUME EITHER BIG OR LITTLE ENDIAN UFSes
 *    in case there are ufs implementations that have strange bytesexes,
 *    you'll need to modify code here as well as in ufs_super.c and ufs_fs.h
 *    to support them.
 */

pub const BYTESEX_LE: i32 = 0;
pub const BYTESEX_BE: i32 = 1;

pub type __fs16 = u16;
pub type __fs32 = u32;
pub type __fs64 = u64;

#[repr(C)]
pub struct super_block {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ufs_sb_info {
    pub s_bytesex: i32,
}

unsafe extern "C" {
    fn UFS_SB(sbp: *mut super_block) -> *mut ufs_sb_info;
}

#[inline]
pub unsafe fn fs64_to_cpu(sbp: *mut super_block, n: __fs64) -> u64 {
    if (*UFS_SB(sbp)).s_bytesex == BYTESEX_LE {
        u64::from_le(n)
    } else {
        u64::from_be(n)
    }
}

#[inline]
pub unsafe fn cpu_to_fs64(sbp: *mut super_block, n: u64) -> __fs64 {
    if (*UFS_SB(sbp)).s_bytesex == BYTESEX_LE {
        n.to_le()
    } else {
        n.to_be()
    }
}

#[inline]
pub unsafe fn fs32_to_cpu(sbp: *mut super_block, n: __fs32) -> u32 {
    if (*UFS_SB(sbp)).s_bytesex == BYTESEX_LE {
        u32::from_le(n)
    } else {
        u32::from_be(n)
    }
}

#[inline]
pub unsafe fn cpu_to_fs32(sbp: *mut super_block, n: u32) -> __fs32 {
    if (*UFS_SB(sbp)).s_bytesex == BYTESEX_LE {
        n.to_le()
    } else {
        n.to_be()
    }
}

#[inline]
pub unsafe fn fs32_add(sbp: *mut super_block, n: *mut __fs32, d: i32) {
    let value = fs32_to_cpu(sbp, *n);
    *n = cpu_to_fs32(sbp, value.wrapping_add(d as u32));
}

#[inline]
pub unsafe fn fs32_sub(sbp: *mut super_block, n: *mut __fs32, d: i32) {
    let value = fs32_to_cpu(sbp, *n);
    *n = cpu_to_fs32(sbp, value.wrapping_sub(d as u32));
}

#[inline]
pub unsafe fn fs16_to_cpu(sbp: *mut super_block, n: __fs16) -> u16 {
    if (*UFS_SB(sbp)).s_bytesex == BYTESEX_LE {
        u16::from_le(n)
    } else {
        u16::from_be(n)
    }
}

#[inline]
pub unsafe fn cpu_to_fs16(sbp: *mut super_block, n: u16) -> __fs16 {
    if (*UFS_SB(sbp)).s_bytesex == BYTESEX_LE {
        n.to_le()
    } else {
        n.to_be()
    }
}

#[inline]
pub unsafe fn fs16_add(sbp: *mut super_block, n: *mut __fs16, d: i32) {
    let value = fs16_to_cpu(sbp, *n);
    *n = cpu_to_fs16(sbp, value.wrapping_add(d as u16));
}

#[inline]
pub unsafe fn fs16_sub(sbp: *mut super_block, n: *mut __fs16, d: i32) {
    let value = fs16_to_cpu(sbp, *n);
    *n = cpu_to_fs16(sbp, value.wrapping_sub(d as u16));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
