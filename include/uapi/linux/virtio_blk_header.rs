/* This header is BSD licensed so anyone can use the definitions to implement
 * compatible drivers/servers. */

// Dependency types are supplied by the corresponding Linux virtio headers.

/* Feature bits */
pub const VIRTIO_BLK_F_SIZE_MAX: u32 = 1;
pub const VIRTIO_BLK_F_SEG_MAX: u32 = 2;
pub const VIRTIO_BLK_F_GEOMETRY: u32 = 4;
pub const VIRTIO_BLK_F_RO: u32 = 5;
pub const VIRTIO_BLK_F_BLK_SIZE: u32 = 6;
pub const VIRTIO_BLK_F_TOPOLOGY: u32 = 10;
pub const VIRTIO_BLK_F_MQ: u32 = 12;
pub const VIRTIO_BLK_F_DISCARD: u32 = 13;
pub const VIRTIO_BLK_F_WRITE_ZEROES: u32 = 14;
pub const VIRTIO_BLK_F_SECURE_ERASE: u32 = 16;
pub const VIRTIO_BLK_F_ZONED: u32 = 17;

/* Legacy feature bits (when VIRTIO_BLK_NO_LEGACY is not defined). */
pub const VIRTIO_BLK_F_BARRIER: u32 = 0;
pub const VIRTIO_BLK_F_SCSI: u32 = 7;
pub const VIRTIO_BLK_F_FLUSH: u32 = 9;
pub const VIRTIO_BLK_F_CONFIG_WCE: u32 = 11;
pub const VIRTIO_BLK_F_WCE: u32 = VIRTIO_BLK_F_FLUSH;

pub const VIRTIO_BLK_ID_BYTES: usize = 20;

#[repr(C, packed)]
pub struct virtio_blk_geometry {
    pub cylinders: __virtio16,
    pub heads: __u8,
    pub sectors: __u8,
}

#[repr(C, packed)]
pub struct virtio_blk_zoned_characteristics {
    pub zone_sectors: __virtio32,
    pub max_open_zones: __virtio32,
    pub max_active_zones: __virtio32,
    pub max_append_sectors: __virtio32,
    pub write_granularity: __virtio32,
    pub model: __u8,
    pub unused2: [__u8; 3],
}

#[repr(C, packed)]
pub struct virtio_blk_config {
    pub capacity: __virtio64,
    pub size_max: __virtio32,
    pub seg_max: __virtio32,
    pub geometry: virtio_blk_geometry,
    pub blk_size: __virtio32,
    pub physical_block_exp: __u8,
    pub alignment_offset: __u8,
    pub min_io_size: __virtio16,
    pub opt_io_size: __virtio32,
    pub wce: __u8,
    pub unused: __u8,
    pub num_queues: __virtio16,
    pub max_discard_sectors: __virtio32,
    pub max_discard_seg: __virtio32,
    pub discard_sector_alignment: __virtio32,
    pub max_write_zeroes_sectors: __virtio32,
    pub max_write_zeroes_seg: __virtio32,
    pub write_zeroes_may_unmap: __u8,
    pub unused1: [__u8; 3],
    pub max_secure_erase_sectors: __virtio32,
    pub max_secure_erase_seg: __virtio32,
    pub secure_erase_sector_alignment: __virtio32,
    pub zoned: virtio_blk_zoned_characteristics,
}

pub const VIRTIO_BLK_T_IN: u32 = 0;
pub const VIRTIO_BLK_T_OUT: u32 = 1;
pub const VIRTIO_BLK_T_SCSI_CMD: u32 = 2;
pub const VIRTIO_BLK_T_FLUSH: u32 = 4;
pub const VIRTIO_BLK_T_GET_ID: u32 = 8;
pub const VIRTIO_BLK_T_DISCARD: u32 = 11;
pub const VIRTIO_BLK_T_WRITE_ZEROES: u32 = 13;
pub const VIRTIO_BLK_T_SECURE_ERASE: u32 = 14;
pub const VIRTIO_BLK_T_ZONE_APPEND: u32 = 15;
pub const VIRTIO_BLK_T_ZONE_REPORT: u32 = 16;
pub const VIRTIO_BLK_T_ZONE_OPEN: u32 = 18;
pub const VIRTIO_BLK_T_ZONE_CLOSE: u32 = 20;
pub const VIRTIO_BLK_T_ZONE_FINISH: u32 = 22;
pub const VIRTIO_BLK_T_ZONE_RESET: u32 = 24;
pub const VIRTIO_BLK_T_ZONE_RESET_ALL: u32 = 26;
pub const VIRTIO_BLK_T_BARRIER: u32 = 0x80000000;

#[repr(C, packed)]
pub struct virtio_blk_outhdr {
    pub r#type: __virtio32,
    pub ioprio: __virtio32,
    pub sector: __virtio64,
}

pub const VIRTIO_BLK_Z_NONE: u32 = 0;
pub const VIRTIO_BLK_Z_HM: u32 = 1;
pub const VIRTIO_BLK_Z_HA: u32 = 2;

#[repr(C, packed)]
pub struct virtio_blk_zone_descriptor {
    pub z_cap: __virtio64,
    pub z_start: __virtio64,
    pub z_wp: __virtio64,
    pub z_type: __u8,
    pub z_state: __u8,
    pub reserved: [__u8; 38],
}

#[repr(C, packed)]
pub struct virtio_blk_zone_report {
    pub nr_zones: __virtio64,
    pub reserved: [__u8; 56],
    pub zones: [virtio_blk_zone_descriptor; 0],
}

pub const VIRTIO_BLK_ZT_CONV: u32 = 1;
pub const VIRTIO_BLK_ZT_SWR: u32 = 2;
pub const VIRTIO_BLK_ZT_SWP: u32 = 3;
pub const VIRTIO_BLK_ZS_NOT_WP: u32 = 0;
pub const VIRTIO_BLK_ZS_EMPTY: u32 = 1;
pub const VIRTIO_BLK_ZS_IOPEN: u32 = 2;
pub const VIRTIO_BLK_ZS_EOPEN: u32 = 3;
pub const VIRTIO_BLK_ZS_CLOSED: u32 = 4;
pub const VIRTIO_BLK_ZS_RDONLY: u32 = 13;
pub const VIRTIO_BLK_ZS_FULL: u32 = 14;
pub const VIRTIO_BLK_ZS_OFFLINE: u32 = 15;

pub const VIRTIO_BLK_WRITE_ZEROES_FLAG_UNMAP: u32 = 0x00000001;

#[repr(C, packed)]
pub struct virtio_blk_discard_write_zeroes {
    pub sector: __le64,
    pub num_sectors: __le32,
    pub flags: __le32,
}

#[repr(C, packed)]
pub struct virtio_scsi_inhdr {
    pub errors: __virtio32,
    pub data_len: __virtio32,
    pub sense_len: __virtio32,
    pub residual: __virtio32,
}

pub const VIRTIO_BLK_S_OK: u32 = 0;
pub const VIRTIO_BLK_S_IOERR: u32 = 1;
pub const VIRTIO_BLK_S_UNSUPP: u32 = 2;
pub const VIRTIO_BLK_S_ZONE_INVALID_CMD: u32 = 3;
pub const VIRTIO_BLK_S_ZONE_UNALIGNED_WP: u32 = 4;
pub const VIRTIO_BLK_S_ZONE_OPEN_RESOURCE: u32 = 5;
pub const VIRTIO_BLK_S_ZONE_ACTIVE_RESOURCE: u32 = 6;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
