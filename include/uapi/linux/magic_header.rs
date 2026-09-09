/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

pub const ADFS_SUPER_MAGIC: u32 = 0xadf5;
pub const AFFS_SUPER_MAGIC: u32 = 0xadff;
pub const AFS_SUPER_MAGIC: u32 = 0x5346414F;
pub const AUTOFS_SUPER_MAGIC: u32 = 0x0187;
pub const CEPH_SUPER_MAGIC: u32 = 0x00c36400;
pub const CODA_SUPER_MAGIC: u32 = 0x73757245;
pub const CRAMFS_MAGIC: u32 = 0x28cd3d45; /* some random number */
pub const CRAMFS_MAGIC_WEND: u32 = 0x453dcd28; /* magic number with the wrong endianess */
pub const DEBUGFS_MAGIC: u32 = 0x64626720;
pub const SECURITYFS_MAGIC: u32 = 0x73636673;
pub const SELINUX_MAGIC: u32 = 0xf97cff8c;
pub const SMACK_MAGIC: u32 = 0x43415d53; /* "SMAC" */
pub const RAMFS_MAGIC: u32 = 0x858458f6; /* some random number */
pub const TMPFS_MAGIC: u32 = 0x01021994;
pub const HUGETLBFS_MAGIC: u32 = 0x958458f6; /* some random number */
pub const SQUASHFS_MAGIC: u32 = 0x73717368;
pub const ECRYPTFS_SUPER_MAGIC: u32 = 0xf15f;
pub const EFS_SUPER_MAGIC: u32 = 0x414A53;
pub const EROFS_SUPER_MAGIC_V1: u32 = 0xE0F5E1E2;
pub const EXT2_SUPER_MAGIC: u32 = 0xEF53;
pub const EXT3_SUPER_MAGIC: u32 = 0xEF53;
pub const XENFS_SUPER_MAGIC: u32 = 0xabba1974;
pub const EXT4_SUPER_MAGIC: u32 = 0xEF53;
pub const BTRFS_SUPER_MAGIC: u32 = 0x9123683E;
pub const NILFS_SUPER_MAGIC: u32 = 0x3434;
pub const F2FS_SUPER_MAGIC: u32 = 0xF2F52010;
pub const HPFS_SUPER_MAGIC: u32 = 0xf995e849;
pub const ISOFS_SUPER_MAGIC: u32 = 0x9660;
pub const JFFS2_SUPER_MAGIC: u32 = 0x72b6;
pub const XFS_SUPER_MAGIC: u32 = 0x58465342; /* "XFSB" */
pub const PSTOREFS_MAGIC: u32 = 0x6165676C;
pub const EFIVARFS_MAGIC: u32 = 0xde5e81e4;
pub const HOSTFS_SUPER_MAGIC: u32 = 0x00c0ffee;
pub const OVERLAYFS_SUPER_MAGIC: u32 = 0x794c7630;
pub const FUSE_SUPER_MAGIC: u32 = 0x65735546;
pub const BCACHEFS_SUPER_MAGIC: u32 = 0xca451a4e;

pub const MINIX_SUPER_MAGIC: u32 = 0x137F; /* minix v1 fs, 14 char names */
pub const MINIX_SUPER_MAGIC2: u32 = 0x138F; /* minix v1 fs, 30 char names */
pub const MINIX2_SUPER_MAGIC: u32 = 0x2468; /* minix v2 fs, 14 char names */
pub const MINIX2_SUPER_MAGIC2: u32 = 0x2478; /* minix v2 fs, 30 char names */
pub const MINIX3_SUPER_MAGIC: u32 = 0x4d5a; /* minix v3 fs, 60 char names */

pub const MSDOS_SUPER_MAGIC: u32 = 0x4d44; /* MD */
pub const EXFAT_SUPER_MAGIC: u32 = 0x2011BAB0;
pub const NCP_SUPER_MAGIC: u32 = 0x564c; /* Guess, what 0x564c is :-) */
pub const NFS_SUPER_MAGIC: u32 = 0x6969;
pub const OCFS2_SUPER_MAGIC: u32 = 0x7461636f;
pub const OPENPROM_SUPER_MAGIC: u32 = 0x9fa1;
pub const QNX4_SUPER_MAGIC: u32 = 0x002f; /* qnx4 fs detection */
pub const QNX6_SUPER_MAGIC: u32 = 0x68191122; /* qnx6 fs detection */
pub const AFS_FS_MAGIC: u32 = 0x6B414653;

pub const REISERFS_SUPER_MAGIC: u32 = 0x52654973; /* used by gcc */
/* used by file system utilities that look at the superblock, etc. */
pub const REISERFS_SUPER_MAGIC_STRING: &[u8] = b"ReIsErFs\0";
pub const REISER2FS_SUPER_MAGIC_STRING: &[u8] = b"ReIsEr2Fs\0";
pub const REISER2FS_JR_SUPER_MAGIC_STRING: &[u8] = b"ReIsEr3Fs\0";

pub const SMB_SUPER_MAGIC: u32 = 0x517B;
pub const CIFS_SUPER_MAGIC: u32 = 0xFF534D42; /* the first four bytes of SMB PDUs */
pub const SMB2_SUPER_MAGIC: u32 = 0xFE534D42;

pub const CGROUP_SUPER_MAGIC: u32 = 0x27e0eb;
pub const CGROUP2_SUPER_MAGIC: u32 = 0x63677270;
pub const RDTGROUP_SUPER_MAGIC: u32 = 0x7655821;
pub const STACK_END_MAGIC: u32 = 0x57AC6E9D;
pub const TRACEFS_MAGIC: u32 = 0x74726163;
pub const V9FS_MAGIC: u32 = 0x01021997;

pub const BDEVFS_MAGIC: u32 = 0x62646576;
pub const DAXFS_MAGIC: u32 = 0x64646178;
pub const BINFMTFS_MAGIC: u32 = 0x42494e4d;
pub const DEVPTS_SUPER_MAGIC: u32 = 0x1cd1;
pub const BINDERFS_SUPER_MAGIC: u32 = 0x6c6f6f70;
pub const FUTEXFS_SUPER_MAGIC: u32 = 0xBAD1DEA;
pub const PIPEFS_MAGIC: u32 = 0x50495045;
pub const PROC_SUPER_MAGIC: u32 = 0x9fa0;
pub const SOCKFS_MAGIC: u32 = 0x534F434B;
pub const SYSFS_MAGIC: u32 = 0x62656572;
pub const USBDEVICE_SUPER_MAGIC: u32 = 0x9fa2;
pub const MTD_INODE_FS_MAGIC: u32 = 0x11307854;
pub const ANON_INODE_FS_MAGIC: u32 = 0x09041934;
pub const BTRFS_TEST_MAGIC: u32 = 0x73727279;
pub const NSFS_MAGIC: u32 = 0x6e736673;
pub const BPF_FS_MAGIC: u32 = 0xcafe4a11;
pub const AAFS_MAGIC: u32 = 0x5a3c69f0;
pub const ZONEFS_MAGIC: u32 = 0x5a4f4653;

/* Since UDF 2.01 is ISO 13346 based... */
pub const UDF_SUPER_MAGIC: u32 = 0x15013346;
pub const DMA_BUF_MAGIC: u32 = 0x444d4142; /* "DMAB" */
pub const DEVMEM_MAGIC: u32 = 0x454d444d; /* "DMEM" */
pub const SECRETMEM_MAGIC: u32 = 0x5345434d; /* "SECM" */
pub const PID_FS_MAGIC: u32 = 0x50494446; /* "PIDF" */
pub const GUEST_MEMFD_MAGIC: u32 = 0x474d454d; /* "GMEM" */
pub const NULL_FS_MAGIC: u32 = 0x4E554C4C; /* "NULL" */
pub const FAIL_FS_MAGIC: u32 = 0x4641494C; /* "FAIL" */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
