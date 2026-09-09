/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/include/linux/ufs_fs.h. */

pub type __fs64 = u64;
pub type __fs32 = u32;
pub type __fs16 = u16;

pub const UFS_BBLOCK: u32 = 0;
pub const UFS_BBSIZE: u32 = 8192;
pub const UFS_SBLOCK: u32 = 8192;
pub const UFS_SBSIZE: u32 = 8192;
pub const UFS_SECTOR_SIZE: u32 = 512;
pub const UFS_SECTOR_BITS: u32 = 9;
pub const UFS_MAGIC: u32 = 0x00011954;
pub const UFS_MAGIC_BW: u32 = 0x0f242697;
pub const UFS2_MAGIC: u32 = 0x19540119;
pub const UFS_CIGAM: u32 = 0x54190100;

pub const SBLOCK_FLOPPY: i32 = 0;
pub const SBLOCK_UFS1: i32 = 8192;
pub const SBLOCK_UFS2: i32 = 65536;
pub const SBLOCK_PIGGY: i32 = 262144;
pub const SBLOCKSIZE: i32 = 8192;
pub const SBLOCKSEARCH: [i32; 5] = [SBLOCK_UFS2, SBLOCK_UFS1, SBLOCK_FLOPPY, SBLOCK_PIGGY, -1];

pub const UFS_MAGIC_LFN: u32 = 0x00095014;
pub const UFS_CIGAM_LFN: u32 = 0x14500900;
pub const UFS_MAGIC_SEC: u32 = 0x00612195;
pub const UFS_CIGAM_SEC: u32 = 0x95216100;
pub const UFS_MAGIC_FEA: u32 = 0x00195612;
pub const UFS_CIGAM_FEA: u32 = 0x12561900;
pub const UFS_MAGIC_4GB: u32 = 0x05231994;
pub const UFS_CIGAM_4GB: u32 = 0x94192305;
pub const UFS_FSF_LFN: u32 = 1;
pub const UFS_FSF_B1: u32 = 2;
pub const UFS_FSF_LFS: u32 = 2;
pub const UFS_FSF_LUID: u32 = 4;

pub const UFS_BSIZE: u32 = 8192;
pub const UFS_MINBSIZE: u32 = 4096;
pub const UFS_FSIZE: u32 = 1024;
pub const UFS_MAXFRAG: usize = (UFS_BSIZE / UFS_FSIZE) as usize;
pub const UFS_NDADDR: usize = 12;
pub const UFS_NINDIR: usize = 3;
pub const UFS_IND_BLOCK: usize = UFS_NDADDR;
pub const UFS_DIND_BLOCK: usize = UFS_NDADDR + 1;
pub const UFS_TIND_BLOCK: usize = UFS_NDADDR + 2;
pub const UFS_ROOTINO: u32 = 2;
pub const UFS_FIRST_INO: u32 = UFS_ROOTINO + 1;
pub const UFS_USEEFT: u16 = 65535;
pub const UFS_FSOK: u32 = 0x7c269d38;
pub const UFS_FSACTIVE: i8 = 0;
pub const UFS_FSCLEAN: i8 = 1;
pub const UFS_FSSTABLE: i8 = 2;
pub const UFS_FSOSF1: i8 = 3;
pub const UFS_FSBAD: i8 = -1;
pub const UFS_FSSUSPEND: i8 = -2;
pub const UFS_FSLOG: i8 = -3;
pub const UFS_FSFIX: i8 = -4;

pub const UFS_DE_MASK: u32 = 0x10; pub const UFS_DE_OLD: u32 = 0; pub const UFS_DE_44BSD: u32 = 0x10;
pub const UFS_UID_MASK: u32 = 0x60; pub const UFS_UID_OLD: u32 = 0; pub const UFS_UID_44BSD: u32 = 0x20; pub const UFS_UID_EFT: u32 = 0x40;
pub const UFS_ST_MASK: u32 = 0x700; pub const UFS_ST_OLD: u32 = 0; pub const UFS_ST_44BSD: u32 = 0x100; pub const UFS_ST_SUN: u32 = 0x200; pub const UFS_ST_SUNOS: u32 = 0x300; pub const UFS_ST_SUNx86: u32 = 0x400;
pub const UFS_CG_MASK: u32 = 0x3000; pub const UFS_CG_OLD: u32 = 0; pub const UFS_CG_44BSD: u32 = 0x2000; pub const UFS_CG_SUN: u32 = 0x1000;
pub const UFS_TYPE_MASK: u32 = 0x10000; pub const UFS_TYPE_UFS1: u32 = 0; pub const UFS_TYPE_UFS2: u32 = 0x10000;
pub const UFS_42INODEFMT: i32 = -1; pub const UFS_44INODEFMT: i32 = 2;
pub const UFS_MINFREE: u32 = 5;
pub const UFS_OPTTIME: u32 = 0; pub const UFS_OPTSPACE: u32 = 1; pub const UFS_DEFAULTOPT: u32 = UFS_OPTTIME;
pub const UFS_MAXNAMLEN: usize = 255; pub const UFS_MAXMNTLEN: usize = 512; pub const UFS2_MAXMNTLEN: usize = 468; pub const UFS2_MAXVOLLEN: usize = 32; pub const UFS_MAXCSBUFS: usize = 31; pub const UFS_LINK_MAX: u32 = 32000; pub const UFS2_NOCSPTRS: usize = 28;
pub const UFS_DIR_PAD: usize = 4; pub const UFS_DIR_ROUND: usize = UFS_DIR_PAD - 1;
pub const UFS_UNCLEAN: u32 = 1; pub const UFS_DOSOFTDEP: u32 = 2; pub const UFS_NEEDSFSCK: u32 = 4; pub const UFS_INDEXDIRS: u32 = 8; pub const UFS_ACLS: u32 = 0x10; pub const UFS_MULTILABEL: u32 = 0x20; pub const UFS_FLAGS_UPDATED: u32 = 0x80;

#[repr(C)] pub struct ufs_timeval { pub tv_sec: __fs32, pub tv_usec: __fs32 }
#[repr(C)] pub union ufs_dir_entry_u { pub d_namlen: __fs16, pub d_44: ufs_dir_entry_44 }
#[repr(C)] pub struct ufs_dir_entry_44 { pub d_type: u8, pub d_namlen: u8 }
#[repr(C)] pub struct ufs_dir_entry { pub d_ino: __fs32, pub d_reclen: __fs16, pub d_u: ufs_dir_entry_u, pub d_name: [u8; UFS_MAXNAMLEN + 1] }
#[repr(C)] pub struct ufs_csum { pub cs_ndir: __fs32, pub cs_nbfree: __fs32, pub cs_nifree: __fs32, pub cs_nffree: __fs32 }
#[repr(C)] pub struct ufs2_csum_total { pub cs_ndir: __fs64, pub cs_nbfree: __fs64, pub cs_nifree: __fs64, pub cs_nffree: __fs64, pub cs_numclusters: __fs64, pub cs_spare: [__fs64; 3] }
#[repr(C)] pub struct ufs_csum_core { pub cs_ndir: u64, pub cs_nbfree: u64, pub cs_nifree: u64, pub cs_nffree: u64, pub cs_numclusters: u64 }

#[repr(C)] pub union ufs_inode_u1 { pub oldids: ufs_inode_oldids, pub ui_inumber: __fs32, pub ui_author: __fs32 }
#[repr(C)] pub struct ufs_inode_oldids { pub ui_suid: __fs16, pub ui_sgid: __fs16 }
#[repr(C)] pub union ufs_inode_u2 { pub ui_addr: ufs_inode_addr, pub ui_symlink: [u8; 60] }
#[repr(C)] pub struct ufs_inode_addr { pub ui_db: [__fs32; UFS_NDADDR], pub ui_ib: [__fs32; UFS_NINDIR] }
#[repr(C)] pub union ufs_inode_u3 { pub ui_sun: ufs_inode_sun, pub ui_44: ufs_inode_44, pub ui_hurd: ufs_inode_hurd }
#[repr(C)] pub struct ufs_inode_sun { pub ui_shadow: __fs32, pub ui_uid: __fs32, pub ui_gid: __fs32, pub ui_oeftflag: __fs32 }
#[repr(C)] pub struct ufs_inode_44 { pub ui_uid: __fs32, pub ui_gid: __fs32, pub ui_spare: [__fs32; 2] }
#[repr(C)] pub struct ufs_inode_hurd { pub ui_uid: __fs32, pub ui_gid: __fs32, pub ui_modeh: __fs16, pub ui_spare: __fs16, pub ui_trans: __fs32 }
#[repr(C)] pub struct ufs_inode { pub ui_mode: __fs16, pub ui_nlink: __fs16, pub ui_u1: ufs_inode_u1, pub ui_size: __fs64, pub ui_atime: ufs_timeval, pub ui_mtime: ufs_timeval, pub ui_ctime: ufs_timeval, pub ui_u2: ufs_inode_u2, pub ui_flags: __fs32, pub ui_blocks: __fs32, pub ui_gen: __fs32, pub ui_u3: ufs_inode_u3 }

pub const UFS_NXADDR: usize = 2;
#[repr(C)] pub union ufs2_inode_u2 { pub ui_addr: ufs2_inode_addr, pub ui_symlink: [u8; 120] }
#[repr(C)] pub struct ufs2_inode_addr { pub ui_db: [__fs64; UFS_NDADDR], pub ui_ib: [__fs64; UFS_NINDIR] }
#[repr(C)] pub struct ufs2_inode { pub ui_mode: __fs16, pub ui_nlink: __fs16, pub ui_uid: __fs32, pub ui_gid: __fs32, pub ui_blksize: __fs32, pub ui_size: __fs64, pub ui_blocks: __fs64, pub ui_atime: __fs64, pub ui_mtime: __fs64, pub ui_ctime: __fs64, pub ui_birthtime: __fs64, pub ui_mtimensec: __fs32, pub ui_atimensec: __fs32, pub ui_ctimensec: __fs32, pub ui_birthnsec: __fs32, pub ui_gen: __fs32, pub ui_kernflags: __fs32, pub ui_flags: __fs32, pub ui_extsize: __fs32, pub ui_extb: [__fs64; UFS_NXADDR], pub ui_u2: ufs2_inode_u2, pub ui_spare: [__fs64; 3] }

pub const UFS_UF_SETTABLE: u32 = 0xffff; pub const UFS_UF_NODUMP: u32 = 1; pub const UFS_UF_IMMUTABLE: u32 = 2; pub const UFS_UF_APPEND: u32 = 4; pub const UFS_UF_OPAQUE: u32 = 8; pub const UFS_UF_NOUNLINK: u32 = 0x10;
pub const UFS_SF_SETTABLE: u32 = 0xffff0000; pub const UFS_SF_ARCHIVED: u32 = 0x10000; pub const UFS_SF_IMMUTABLE: u32 = 0x20000; pub const UFS_SF_APPEND: u32 = 0x40000; pub const UFS_SF_NOUNLINK: u32 = 0x100000;

/* External dependency: struct buffer_head is supplied by the surrounding kernel translation. */
#[repr(C)] pub struct ufs_buffer_head { pub fragment: u64, pub count: u64, pub bh: [*mut buffer_head; UFS_MAXFRAG] }
#[repr(C)] pub struct buffer_head;
#[repr(C)] pub struct ufs_cg_private_info { pub c_ubh: ufs_buffer_head, pub c_cgx: u32, pub c_ncyl: u16, pub c_niblk: u16, pub c_ndblk: u32, pub c_rotor: u32, pub c_frotor: u32, pub c_irotor: u32, pub c_btotoff: u32, pub c_boff: u32, pub c_iusedoff: u32, pub c_freeoff: u32, pub c_nextfreeoff: u32, pub c_clustersumoff: u32, pub c_clusteroff: u32, pub c_nclusterblks: u32 }

/* The remaining on-disk superblock and cylinder-group declarations retain C layout. */
#[repr(C)] pub struct ufs_cylinder_group { pub cg_link: __fs32, pub cg_magic: __fs32, pub cg_time: __fs32, pub cg_cgx: __fs32, pub cg_ncyl: __fs16, pub cg_niblk: __fs16, pub cg_ndblk: __fs32, pub cg_cs: ufs_csum, pub cg_rotor: __fs32, pub cg_frotor: __fs32, pub cg_irotor: __fs32, pub cg_frsum: [__fs32; UFS_MAXFRAG], pub cg_btotoff: __fs32, pub cg_boff: __fs32, pub cg_iusedoff: __fs32, pub cg_freeoff: __fs32, pub cg_nextfreeoff: __fs32, pub cg_space: [u8; 1] }
#[repr(C)] pub struct ufs_old_cylinder_group { pub cg_link: __fs32, pub cg_rlink: __fs32, pub cg_time: __fs32, pub cg_cgx: __fs32, pub cg_ncyl: __fs16, pub cg_niblk: __fs16, pub cg_ndblk: __fs32, pub cg_cs: ufs_csum, pub cg_rotor: __fs32, pub cg_frotor: __fs32, pub cg_irotor: __fs32, pub cg_frsum: [__fs32; 8], pub cg_btot: [__fs32; 32], pub cg_b: [[__fs16; 8]; 32], pub cg_iused: [u8; 256], pub cg_magic: __fs32, pub cg_free: [u8; 1] }

pub const CG_MAGIC: u32 = 0x090255;
pub const UFS_42POSTBLFMT: i32 = -1;
pub const UFS_DYNAMICPOSTBLFMT: i32 = 1;

#[repr(C)] pub struct ufs_sb_private_info {
    pub s_ubh: ufs_buffer_head, pub cs_total: ufs_csum_core,
    pub s_sblkno: u32, pub s_cblkno: u32, pub s_iblkno: u32, pub s_dblkno: u32,
    pub s_cgoffset: u32, pub s_cgmask: u32, pub s_size: u64, pub s_dsize: u64,
    pub s_ncg: u32, pub s_bsize: u32, pub s_fsize: u32, pub s_fpb: u32, pub s_minfree: u32,
    pub s_bmask: u32, pub s_fmask: u32, pub s_bshift: u32, pub s_fshift: u32,
    pub s_fpbshift: u32, pub s_fsbtodb: u32, pub s_sbsize: u32, pub s_csmask: u32,
    pub s_csshift: u32, pub s_nindir: u32, pub s_inopb: u32, pub s_nspf: u32,
    pub s_npsect: u32, pub s_interleave: u32, pub s_trackskew: u32, pub s_csaddr: u64,
    pub s_cssize: u32, pub s_cgsize: u32, pub s_ntrak: u32, pub s_nsect: u32, pub s_spc: u32,
    pub s_ipg: u32, pub s_fpg: u32, pub s_cpc: u32, pub s_contigsumsize: i32,
    pub s_qbmask: i64, pub s_qfmask: i64, pub s_postblformat: i32, pub s_nrpos: i32,
    pub s_postbloff: i32, pub s_rotbloff: i32, pub s_fpbmask: u32, pub s_apb: u32,
    pub s_apbmask: u32, pub s_apbshift: u32, pub s_nspfshift: u32, pub s_nspb: u32,
    pub s_inopf: u32, pub s_sbbase: u32, pub s_bpf: u32, pub s_bpfshift: u32,
    pub s_bpfmask: u32, pub s_maxsymlinklen: u32, pub fs_magic: i32,
    pub s_dirblksize: u32, pub s_root_blocks: u64, pub s_time_to_space: u64, pub s_space_to_time: u64,
}

#[repr(C)] pub struct ufs_super_block_first {
    pub fs_u0: [__fs32; 1], pub fs_rlink: __fs32, pub fs_sblkno: __fs32, pub fs_cblkno: __fs32,
    pub fs_iblkno: __fs32, pub fs_dblkno: __fs32, pub fs_cgoffset: __fs32, pub fs_cgmask: __fs32,
    pub fs_time: __fs32, pub fs_size: __fs32, pub fs_dsize: __fs32, pub fs_ncg: __fs32,
    pub fs_bsize: __fs32, pub fs_fsize: __fs32, pub fs_frag: __fs32, pub fs_minfree: __fs32,
    pub fs_rotdelay: __fs32, pub fs_rps: __fs32, pub fs_bmask: __fs32, pub fs_fmask: __fs32,
    pub fs_bshift: __fs32, pub fs_fshift: __fs32, pub fs_maxcontig: __fs32, pub fs_maxbpg: __fs32,
    pub fs_fragshift: __fs32, pub fs_fsbtodb: __fs32, pub fs_sbsize: __fs32, pub fs_csmask: __fs32,
    pub fs_csshift: __fs32, pub fs_nindir: __fs32, pub fs_inopb: __fs32, pub fs_nspf: __fs32,
    pub fs_optim: __fs32, pub fs_u1: [__fs32; 1], pub fs_interleave: __fs32, pub fs_trackskew: __fs32,
    pub fs_id: [__fs32; 2], pub fs_csaddr: __fs32, pub fs_cssize: __fs32, pub fs_cgsize: __fs32,
    pub fs_ntrak: __fs32, pub fs_nsect: __fs32, pub fs_spc: __fs32, pub fs_ncyl: __fs32,
    pub fs_cpg: __fs32, pub fs_ipg: __fs32, pub fs_fpg: __fs32, pub fs_cstotal: ufs_csum,
    pub fs_fmod: i8, pub fs_clean: i8, pub fs_ronly: i8, pub fs_flags: i8,
    pub fs_fsmnt: [i8; UFS_MAXMNTLEN - 212],
}

#[repr(C)] pub struct ufs_super_block_second { pub fs_un: [u8; 512] }
#[repr(C, packed)] pub struct ufs_super_block_third { pub fs_un1: [u8; 128], pub fs_un2: [u8; 232], pub fs_postblformat: __fs32, pub fs_nrpos: __fs32, pub fs_postbloff: __fs32, pub fs_rotbloff: __fs32, pub fs_magic: __fs32, pub fs_space: [u8; 1] }

/* Macro translations (the original names and expressions are preserved as documentation). */
// ufs_fsbtodb, ufs_dbtofsb, cylinder-group location, inode-location, and block rounding macros
// remain dependent on the caller's ufs_sb_info/super_block objects and are intentionally expressed
// at their call sites in the surrounding translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
