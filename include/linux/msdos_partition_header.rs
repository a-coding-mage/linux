/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _LINUX_MSDOS_PARTITION_H

pub const MSDOS_LABEL_MAGIC: u16 = 0xAA55;

#[repr(C, packed)]
pub struct msdos_partition {
    pub boot_ind: u8,   /* 0x80 - active */
    pub head: u8,       /* starting head */
    pub sector: u8,     /* starting sector */
    pub cyl: u8,        /* starting cylinder */
    pub sys_ind: u8,    /* What partition type */
    pub end_head: u8,   /* end head */
    pub end_sector: u8, /* end sector */
    pub end_cyl: u8,   /* end cylinder */
    pub start_sect: __le32, /* starting sector counting from 0 */
    pub nr_sects: __le32,   /* nr of sectors in partition */
}

#[repr(C)]
pub enum msdos_sys_ind {
    /*
     * These three have identical behaviour; use the second one if DOS FDISK
     * gets confused about extended/logical partitions starting past
     * cylinder 1023.
     */
    DOS_EXTENDED_PARTITION = 5,
    LINUX_EXTENDED_PARTITION = 0x85,
    WIN98_EXTENDED_PARTITION = 0x0f,

    LINUX_DATA_PARTITION = 0x83,
    LINUX_LVM_PARTITION = 0x8e,
    LINUX_RAID_PARTITION = 0xfd, /* autodetect RAID partition */

    SOLARIS_X86_PARTITION = 0x82, /* also Linux swap partitions */
    NEW_SOLARIS_X86_PARTITION = 0xbf,

    DM6_AUX1PARTITION = 0x51, /* no DDO:  use xlated geom */
    DM6_AUX3PARTITION = 0x53, /* no DDO:  use xlated geom */
    DM6_PARTITION = 0x54,     /* has DDO: use xlated geom & offset */
    EZD_PARTITION = 0x55,     /* EZ-DRIVE */

    FREEBSD_PARTITION = 0xa5, /* FreeBSD Partition ID */
    OPENBSD_PARTITION = 0xa6, /* OpenBSD Partition ID */
    NETBSD_PARTITION = 0xa9,  /* NetBSD Partition ID */
    BSDI_PARTITION = 0xb7,    /* BSDI Partition ID */
    MINIX_PARTITION = 0x81,   /* Minix Partition ID */
    UNIXWARE_PARTITION = 0x63, /* Same as GNU_HURD and SCO Unix */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
