/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* Multimedia device API. Rust translation of media.h. */

#[repr(C)]
pub struct media_device_info {
    pub driver: [u8; 16], pub model: [u8; 32], pub serial: [u8; 40],
    pub bus_info: [u8; 32], pub media_version: u32, pub hw_revision: u32,
    pub driver_version: u32, pub reserved: [u32; 31],
}

pub const MEDIA_ENT_F_BASE: u32 = 0x00000000;
pub const MEDIA_ENT_F_OLD_BASE: u32 = 0x00010000;
pub const MEDIA_ENT_F_OLD_SUBDEV_BASE: u32 = 0x00020000;
pub const MEDIA_ENT_F_UNKNOWN: u32 = MEDIA_ENT_F_BASE;
pub const MEDIA_ENT_F_V4L2_SUBDEV_UNKNOWN: u32 = MEDIA_ENT_F_OLD_SUBDEV_BASE;
pub const MEDIA_ENT_F_DTV_DEMOD: u32 = MEDIA_ENT_F_BASE + 0x00001;
pub const MEDIA_ENT_F_TS_DEMUX: u32 = MEDIA_ENT_F_BASE + 0x00002;
pub const MEDIA_ENT_F_DTV_CA: u32 = MEDIA_ENT_F_BASE + 0x00003;
pub const MEDIA_ENT_F_DTV_NET_DECAP: u32 = MEDIA_ENT_F_BASE + 0x00004;
pub const MEDIA_ENT_F_IO_V4L: u32 = MEDIA_ENT_F_OLD_BASE + 1;
pub const MEDIA_ENT_F_IO_DTV: u32 = MEDIA_ENT_F_BASE + 0x01001;
pub const MEDIA_ENT_F_IO_VBI: u32 = MEDIA_ENT_F_BASE + 0x01002;
pub const MEDIA_ENT_F_IO_SWRADIO: u32 = MEDIA_ENT_F_BASE + 0x01003;
pub const MEDIA_ENT_F_CAM_SENSOR: u32 = MEDIA_ENT_F_OLD_SUBDEV_BASE + 1;
pub const MEDIA_ENT_F_FLASH: u32 = MEDIA_ENT_F_OLD_SUBDEV_BASE + 2;
pub const MEDIA_ENT_F_LENS: u32 = MEDIA_ENT_F_OLD_SUBDEV_BASE + 3;
pub const MEDIA_ENT_F_TUNER: u32 = MEDIA_ENT_F_OLD_SUBDEV_BASE + 5;
pub const MEDIA_ENT_F_IF_VID_DECODER: u32 = MEDIA_ENT_F_BASE + 0x02001;
pub const MEDIA_ENT_F_IF_AUD_DECODER: u32 = MEDIA_ENT_F_BASE + 0x02002;
pub const MEDIA_ENT_F_AUDIO_CAPTURE: u32 = MEDIA_ENT_F_BASE + 0x03001;
pub const MEDIA_ENT_F_AUDIO_PLAYBACK: u32 = MEDIA_ENT_F_BASE + 0x03002;
pub const MEDIA_ENT_F_AUDIO_MIXER: u32 = MEDIA_ENT_F_BASE + 0x03003;
pub const MEDIA_ENT_F_PROC_VIDEO_COMPOSER: u32 = MEDIA_ENT_F_BASE + 0x4001;
pub const MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER: u32 = MEDIA_ENT_F_BASE + 0x4002;
pub const MEDIA_ENT_F_PROC_VIDEO_PIXEL_ENC_CONV: u32 = MEDIA_ENT_F_BASE + 0x4003;
pub const MEDIA_ENT_F_PROC_VIDEO_LUT: u32 = MEDIA_ENT_F_BASE + 0x4004;
pub const MEDIA_ENT_F_PROC_VIDEO_SCALER: u32 = MEDIA_ENT_F_BASE + 0x4005;
pub const MEDIA_ENT_F_PROC_VIDEO_STATISTICS: u32 = MEDIA_ENT_F_BASE + 0x4006;
pub const MEDIA_ENT_F_PROC_VIDEO_ENCODER: u32 = MEDIA_ENT_F_BASE + 0x4007;
pub const MEDIA_ENT_F_PROC_VIDEO_DECODER: u32 = MEDIA_ENT_F_BASE + 0x4008;
pub const MEDIA_ENT_F_PROC_VIDEO_ISP: u32 = MEDIA_ENT_F_BASE + 0x4009;
pub const MEDIA_ENT_F_VID_MUX: u32 = MEDIA_ENT_F_BASE + 0x5001;
pub const MEDIA_ENT_F_VID_IF_BRIDGE: u32 = MEDIA_ENT_F_BASE + 0x5002;
pub const MEDIA_ENT_F_ATV_DECODER: u32 = MEDIA_ENT_F_OLD_SUBDEV_BASE + 4;
pub const MEDIA_ENT_F_DV_DECODER: u32 = MEDIA_ENT_F_BASE + 0x6001;
pub const MEDIA_ENT_F_DV_ENCODER: u32 = MEDIA_ENT_F_BASE + 0x6002;
pub const MEDIA_ENT_FL_DEFAULT: u32 = 1u32 << 0;
pub const MEDIA_ENT_FL_CONNECTOR: u32 = 1u32 << 1;
pub const MEDIA_ENT_ID_FLAG_NEXT: u32 = 1u32 << 31;

#[repr(C)]
pub union media_entity_desc_union {
    pub dev: media_entity_desc_dev,
    pub alsa: media_entity_desc_alsa,
    pub v4l: media_entity_desc_dev,
    pub fb: media_entity_desc_dev,
    pub dvb: i32,
    pub raw: [u8; 184],
}
#[repr(C)] pub struct media_entity_desc_dev { pub major: u32, pub minor: u32 }
#[repr(C)] pub struct media_entity_desc_alsa { pub card: u32, pub device: u32, pub subdevice: u32 }
#[repr(C)]
pub struct media_entity_desc {
    pub id: u32, pub name: [u8; 32], pub type_: u32, pub revision: u32,
    pub flags: u32, pub group_id: u32, pub pads: u16, pub links: u16,
    pub reserved: [u32; 4], pub u: media_entity_desc_union,
}

pub const MEDIA_PAD_FL_SINK: u32 = 1u32 << 0;
pub const MEDIA_PAD_FL_SOURCE: u32 = 1u32 << 1;
pub const MEDIA_PAD_FL_MUST_CONNECT: u32 = 1u32 << 2;
#[repr(C)] pub struct media_pad_desc { pub entity: u32, pub index: u16, pub flags: u32, pub reserved: [u32; 2] }
pub const MEDIA_LNK_FL_ENABLED: u32 = 1u32 << 0;
pub const MEDIA_LNK_FL_IMMUTABLE: u32 = 1u32 << 1;
pub const MEDIA_LNK_FL_DYNAMIC: u32 = 1u32 << 2;
pub const MEDIA_LNK_FL_LINK_TYPE: u32 = 0xf << 28;
pub const MEDIA_LNK_FL_DATA_LINK: u32 = 0u32 << 28;
pub const MEDIA_LNK_FL_INTERFACE_LINK: u32 = 1u32 << 28;
pub const MEDIA_LNK_FL_ANCILLARY_LINK: u32 = 2u32 << 28;
#[repr(C)] pub struct media_link_desc { pub source: media_pad_desc, pub sink: media_pad_desc, pub flags: u32, pub reserved: [u32; 2] }
#[repr(C)] pub struct media_links_enum { pub entity: u32, pub pads: *mut media_pad_desc, pub links: *mut media_link_desc, pub reserved: [u32; 4] }

pub const MEDIA_INTF_T_DVB_BASE: u32 = 0x100;
pub const MEDIA_INTF_T_V4L_BASE: u32 = 0x200;
pub const MEDIA_INTF_T_DVB_FE: u32 = MEDIA_INTF_T_DVB_BASE;
pub const MEDIA_INTF_T_DVB_DEMUX: u32 = MEDIA_INTF_T_DVB_BASE + 1;
pub const MEDIA_INTF_T_DVB_DVR: u32 = MEDIA_INTF_T_DVB_BASE + 2;
pub const MEDIA_INTF_T_DVB_CA: u32 = MEDIA_INTF_T_DVB_BASE + 3;
pub const MEDIA_INTF_T_DVB_NET: u32 = MEDIA_INTF_T_DVB_BASE + 4;
pub const MEDIA_INTF_T_V4L_VIDEO: u32 = MEDIA_INTF_T_V4L_BASE;
pub const MEDIA_INTF_T_V4L_VBI: u32 = MEDIA_INTF_T_V4L_BASE + 1;
pub const MEDIA_INTF_T_V4L_RADIO: u32 = MEDIA_INTF_T_V4L_BASE + 2;
pub const MEDIA_INTF_T_V4L_SUBDEV: u32 = MEDIA_INTF_T_V4L_BASE + 3;
pub const MEDIA_INTF_T_V4L_SWRADIO: u32 = MEDIA_INTF_T_V4L_BASE + 4;
pub const MEDIA_INTF_T_V4L_TOUCH: u32 = MEDIA_INTF_T_V4L_BASE + 5;
pub const MEDIA_INTF_T_ALSA_BASE: u32 = 0x300;
pub const MEDIA_INTF_T_ALSA_PCM_CAPTURE: u32 = MEDIA_INTF_T_ALSA_BASE;
pub const MEDIA_INTF_T_ALSA_PCM_PLAYBACK: u32 = MEDIA_INTF_T_ALSA_BASE + 1;
pub const MEDIA_INTF_T_ALSA_CONTROL: u32 = MEDIA_INTF_T_ALSA_BASE + 2;

pub const MEDIA_ENT_F_CONN_RF: u32 = MEDIA_ENT_F_BASE + 0x30001;
pub const MEDIA_ENT_F_CONN_SVIDEO: u32 = MEDIA_ENT_F_BASE + 0x30002;
pub const MEDIA_ENT_F_CONN_COMPOSITE: u32 = MEDIA_ENT_F_BASE + 0x30003;

#[inline] pub const fn MEDIA_V2_ENTITY_HAS_FLAGS(media_version: u32) -> bool { media_version >= ((4u32 << 16) | (19u32 << 8)) }
#[repr(C, packed)] pub struct media_v2_entity { pub id: u32, pub name: [u8; 64], pub function: u32, pub flags: u32, pub reserved: [u32; 5] }
#[repr(C, packed)] pub struct media_v2_intf_devnode { pub major: u32, pub minor: u32 }
#[repr(C, packed)] pub union media_v2_interface_union { pub devnode: media_v2_intf_devnode, pub raw: [u32; 16] }
#[repr(C, packed)] pub struct media_v2_interface { pub id: u32, pub intf_type: u32, pub flags: u32, pub reserved: [u32; 9], pub u: media_v2_interface_union }
#[inline] pub const fn MEDIA_V2_PAD_HAS_INDEX(media_version: u32) -> bool { media_version >= ((4u32 << 16) | (19u32 << 8)) }
#[repr(C, packed)] pub struct media_v2_pad { pub id: u32, pub entity_id: u32, pub flags: u32, pub index: u32, pub reserved: [u32; 4] }
#[repr(C, packed)] pub struct media_v2_link { pub id: u32, pub source_id: u32, pub sink_id: u32, pub flags: u32, pub reserved: [u32; 6] }
#[repr(C, packed)] pub struct media_v2_topology { pub topology_version: u64, pub num_entities: u32, pub reserved1: u32, pub ptr_entities: u64, pub num_interfaces: u32, pub reserved2: u32, pub ptr_interfaces: u64, pub num_pads: u32, pub reserved3: u32, pub ptr_pads: u64, pub num_links: u32, pub reserved4: u32, pub ptr_links: u64 }

/* ioctl values depend on the platform-specific _IO, _IOR and _IOWR definitions. */
pub const MEDIA_IOC_DEVICE_INFO: u64 = crate::_IOWR(b'|' as u32, 0x00, core::mem::size_of::<media_device_info>() as u32);
pub const MEDIA_IOC_ENUM_ENTITIES: u64 = crate::_IOWR(b'|' as u32, 0x01, core::mem::size_of::<media_entity_desc>() as u32);
pub const MEDIA_IOC_ENUM_LINKS: u64 = crate::_IOWR(b'|' as u32, 0x02, core::mem::size_of::<media_links_enum>() as u32);
pub const MEDIA_IOC_SETUP_LINK: u64 = crate::_IOWR(b'|' as u32, 0x03, core::mem::size_of::<media_link_desc>() as u32);
pub const MEDIA_IOC_G_TOPOLOGY: u64 = crate::_IOWR(b'|' as u32, 0x04, core::mem::size_of::<media_v2_topology>() as u32);
pub const MEDIA_IOC_REQUEST_ALLOC: u64 = crate::_IOR(b'|' as u32, 0x05, core::mem::size_of::<i32>() as u32);
pub const MEDIA_REQUEST_IOC_QUEUE: u64 = crate::_IO(b'|' as u32, 0x80);
pub const MEDIA_REQUEST_IOC_REINIT: u64 = crate::_IO(b'|' as u32, 0x81);

pub const MEDIA_ENT_TYPE_SHIFT: u32 = 16;
pub const MEDIA_ENT_TYPE_MASK: u32 = 0x00ff0000;
pub const MEDIA_ENT_SUBTYPE_MASK: u32 = 0x0000ffff;
pub const MEDIA_ENT_T_DEVNODE_UNKNOWN: u32 = MEDIA_ENT_F_OLD_BASE | MEDIA_ENT_SUBTYPE_MASK;
pub const MEDIA_ENT_T_DEVNODE: u32 = MEDIA_ENT_F_OLD_BASE;
pub const MEDIA_ENT_T_DEVNODE_V4L: u32 = MEDIA_ENT_F_IO_V4L;
pub const MEDIA_ENT_T_DEVNODE_FB: u32 = MEDIA_ENT_F_OLD_BASE + 2;
pub const MEDIA_ENT_T_DEVNODE_ALSA: u32 = MEDIA_ENT_F_OLD_BASE + 3;
pub const MEDIA_ENT_T_DEVNODE_DVB: u32 = MEDIA_ENT_F_OLD_BASE + 4;
pub const MEDIA_ENT_T_UNKNOWN: u32 = MEDIA_ENT_F_UNKNOWN;
pub const MEDIA_ENT_T_V4L2_VIDEO: u32 = MEDIA_ENT_F_IO_V4L;
pub const MEDIA_ENT_T_V4L2_SUBDEV: u32 = MEDIA_ENT_F_V4L2_SUBDEV_UNKNOWN;
pub const MEDIA_ENT_T_V4L2_SUBDEV_SENSOR: u32 = MEDIA_ENT_F_CAM_SENSOR;
pub const MEDIA_ENT_T_V4L2_SUBDEV_FLASH: u32 = MEDIA_ENT_F_FLASH;
pub const MEDIA_ENT_T_V4L2_SUBDEV_LENS: u32 = MEDIA_ENT_F_LENS;
pub const MEDIA_ENT_T_V4L2_SUBDEV_DECODER: u32 = MEDIA_ENT_F_ATV_DECODER;
pub const MEDIA_ENT_T_V4L2_SUBDEV_TUNER: u32 = MEDIA_ENT_F_TUNER;
pub const MEDIA_ENT_F_DTV_DECODER: u32 = MEDIA_ENT_F_DV_DECODER;
pub const MEDIA_INTF_T_ALSA_COMPRESS: u32 = MEDIA_INTF_T_ALSA_BASE + 3;
pub const MEDIA_INTF_T_ALSA_RAWMIDI: u32 = MEDIA_INTF_T_ALSA_BASE + 4;
pub const MEDIA_INTF_T_ALSA_HWDEP: u32 = MEDIA_INTF_T_ALSA_BASE + 5;
pub const MEDIA_INTF_T_ALSA_SEQUENCER: u32 = MEDIA_INTF_T_ALSA_BASE + 6;
pub const MEDIA_INTF_T_ALSA_TIMER: u32 = MEDIA_INTF_T_ALSA_BASE + 7;
pub const MEDIA_API_VERSION: u32 = (0u32 << 16) | (1u32 << 8);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
