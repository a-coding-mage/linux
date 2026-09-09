/* SPDX-License-Identifier: GPL-2.0 */
// Translated from vio.h. Linux/architecture dependencies are supplied externally.

#[repr(C)]
pub struct vio_msg_tag { pub type_: u8, pub stype: u8, pub stype_env: u16, pub sid: u32 }
pub const VIO_TYPE_CTRL: u8 = 0x01; pub const VIO_TYPE_DATA: u8 = 0x02; pub const VIO_TYPE_ERR: u8 = 0x04;
pub const VIO_SUBTYPE_INFO: u8 = 0x01; pub const VIO_SUBTYPE_ACK: u8 = 0x02; pub const VIO_SUBTYPE_NACK: u8 = 0x04;
pub const VIO_VER_INFO: u16 = 1; pub const VIO_ATTR_INFO: u16 = 2; pub const VIO_DRING_REG: u16 = 3; pub const VIO_DRING_UNREG: u16 = 4;
pub const VIO_RDX: u16 = 5; pub const VIO_PKT_DATA: u16 = 0x40; pub const VIO_DESC_DATA: u16 = 0x41; pub const VIO_DRING_DATA: u16 = 0x42; pub const VNET_MCAST_INFO: u16 = 0x101;

#[repr(C)] pub struct vio_rdx { pub tag: vio_msg_tag, pub resv: [u64; 6] }
#[repr(C)] pub struct vio_ver_info { pub tag: vio_msg_tag, pub major: u16, pub minor: u16, pub dev_class: u8, pub resv1: [u8; 3], pub resv2: [u64; 5] }
pub const VDEV_NETWORK: u8=1; pub const VDEV_NETWORK_SWITCH: u8=2; pub const VDEV_DISK: u8=3; pub const VDEV_DISK_SERVER: u8=4; pub const VDEV_CONSOLE_CON: u8=5;
#[repr(C)] pub struct vio_dring_register { pub tag: vio_msg_tag, pub dring_ident: u64, pub num_descr: u32, pub descr_size: u32, pub options: u16, pub resv: u16, pub num_cookies: u32, pub cookies: [ldc_trans_cookie; 0] }
pub const VIO_TX_DRING:u16=1; pub const VIO_RX_DRING:u16=2; pub const VIO_RX_DRING_DATA:u16=4;
#[repr(C)] pub struct vio_dring_unregister { pub tag: vio_msg_tag, pub dring_ident:u64, pub resv:[u64;5] }
pub const VIO_PKT_MODE:u8=1; pub const VIO_DESC_MODE:u8=2; pub const VIO_DRING_MODE:u8=3; pub const VIO_NEW_DRING_MODE:u8=4;
#[repr(C)] pub struct vio_dring_data { pub tag:vio_msg_tag, pub seq:u64, pub dring_ident:u64, pub start_idx:u32, pub end_idx:u32, pub state:u8, pub _pad1:u8, pub _pad2:u16, pub _pad3:u32, pub _par4:[u64;2] }
pub const VIO_DRING_ACTIVE:u8=1; pub const VIO_DRING_STOPPED:u8=2;
#[repr(C)] pub struct vio_dring_hdr { pub state:u8, pub ack:u8, pub _pad1:u16, pub _pad2:u32 }
pub const VIO_DESC_FREE:u8=1; pub const VIO_DESC_READY:u8=2; pub const VIO_DESC_ACCEPTED:u8=3; pub const VIO_DESC_DONE:u8=4; pub const VIO_ACK_ENABLE:u8=1; pub const VIO_ACK_DISABLE:u8=0;

#[repr(C)] pub struct vio_disk_attr_info { pub tag:vio_msg_tag, pub xfer_mode:u8, pub vdisk_type:u8, pub vdisk_mtype:u8, pub resv1:u8, pub vdisk_block_size:u32, pub operations:u64, pub vdisk_size:u64, pub max_xfer_size:u64, pub phys_block_size:u32, pub resv2:u32, pub resv3:[u64;1] }
pub const VD_DISK_TYPE_SLICE:u8=1; pub const VD_DISK_TYPE_DISK:u8=2; pub const VD_MEDIA_TYPE_FIXED:u8=1; pub const VD_MEDIA_TYPE_CD:u8=2; pub const VD_MEDIA_TYPE_DVD:u8=3;
#[repr(C)] pub struct vio_disk_desc { pub hdr:vio_dring_hdr, pub req_id:u64, pub operation:u8, pub slice:u8, pub resv1:u16, pub status:u32, pub offset:u64, pub size:u64, pub ncookies:u32, pub resv2:u32, pub cookies:[ldc_trans_cookie;0] }
pub const VD_OP_BREAD:u8=1; pub const VD_OP_BWRITE:u8=2; pub const VD_OP_FLUSH:u8=3; pub const VD_OP_GET_WCE:u8=4; pub const VD_OP_SET_WCE:u8=5; pub const VD_OP_GET_VTOC:u8=6; pub const VD_OP_SET_VTOC:u8=7; pub const VD_OP_GET_DISKGEOM:u8=8; pub const VD_OP_SET_DISKGEOM:u8=9; pub const VD_OP_SCSICMD:u8=0xa; pub const VD_OP_GET_DEVID:u8=0xb; pub const VD_OP_GET_EFI:u8=0xc; pub const VD_OP_SET_EFI:u8=0xd;
pub const VIO_DISK_VNAME_LEN:usize=8; pub const VIO_DISK_ALABEL_LEN:usize=128; pub const VIO_DISK_NUM_PART:usize=8;
#[repr(C)] pub struct vio_disk_partition { pub id:u16,pub perm_flags:u16,pub resv:u32,pub start_block:u64,pub num_blocks:u64 }
#[repr(C)] pub struct vio_disk_vtoc { pub volume_name:[u8;8], pub sector_size:u16, pub num_partitions:u16, pub ascii_label:[u8;128], pub partitions:[vio_disk_partition;8] }
#[repr(C)] pub struct vio_disk_geom { pub num_cyl:u16,pub alt_cyl:u16,pub beg_cyl:u16,pub num_hd:u16,pub num_sec:u16,pub ifact:u16,pub apc:u16,pub rpm:u16,pub phy_cyl:u16,pub wr_skip:u16,pub rd_skip:u16 }
#[repr(C)] pub struct vio_disk_devid { pub resv:u16,pub type_:u16,pub len:u32,pub id:[i8;0] }
#[repr(C)] pub struct vio_disk_efi { pub lba:u64,pub len:u64,pub data:[i8;0] }

#[repr(C)] pub struct vio_net_attr_info { pub tag:vio_msg_tag,pub xfer_mode:u8,pub addr_type:u8,pub ack_freq:u16,pub plnk_updt:u8,pub options:u8,pub resv1:u16,pub addr:u64,pub mtu:u16,pub cflags:u16,pub ipv4_lso_maxlen:u16,pub resv2:u32,pub resv3:[u64;2] }
pub const VNET_ADDR_ETHERMAC:u8=1; pub const PHYSLINK_UPDATE_NONE:u8=0; pub const PHYSLINK_UPDATE_STATE:u8=1; pub const PHYSLINK_UPDATE_STATE_ACK:u8=2; pub const PHYSLINK_UPDATE_STATE_NACK:u8=3; pub const VNET_LSO_IPV4_CAPAB:u16=1;
pub const VNET_NUM_MCAST:usize=7;
#[repr(C)] pub struct vio_net_mcast_info { pub tag:vio_msg_tag,pub set:u8,pub count:u8,pub mcast_addr:[u8;42],pub resv:u32 }
#[repr(C)] pub struct vio_net_desc { pub hdr:vio_dring_hdr,pub size:u32,pub ncookies:u32,pub cookies:[ldc_trans_cookie;0] }
#[repr(C)] pub struct vio_net_dext { pub flags:u8,pub vnet_hashval:u8,pub ipv4_lso_mss:u16,pub resv3:u32 }
pub const VNET_PKT_HASH:u8=1; pub const VNET_PKT_HCK_IPV4_HDRCKSUM:u8=2; pub const VNET_PKT_HCK_FULLCKSUM:u8=4; pub const VNET_PKT_IPV4_LSO:u8=8; pub const VNET_PKT_HCK_IPV4_HDRCKSUM_OK:u8=0x10; pub const VNET_PKT_HCK_FULLCKSUM_OK:u8=0x20;
pub unsafe fn vio_net_ext(desc:*mut vio_net_desc)->*mut vio_net_dext { (*desc).cookies.as_mut_ptr().add(2) as *mut vio_net_dext }

pub const VIO_MAX_RING_COOKIES:usize=24;
#[repr(C)] pub struct vio_dring_state { pub ident:u64,pub base:*mut core::ffi::c_void,pub snd_nxt:u64,pub rcv_nxt:u64,pub entry_size:u32,pub num_entries:u32,pub prod:u32,pub cons:u32,pub pending:u32,pub ncookies:i32,pub cookies:[ldc_trans_cookie;24] }
pub const VIO_TAG_SIZE:usize=core::mem::size_of::<vio_msg_tag>();
pub const VIO_VCC_MTU_SIZE:usize=LDC_PACKET_SIZE - VIO_TAG_SIZE;
#[repr(C)] pub struct vio_vcc { pub tag:vio_msg_tag,pub data:[i8;VIO_VCC_MTU_SIZE] }
pub unsafe fn vio_dring_cur(dr:*mut vio_dring_state)->*mut core::ffi::c_void { (dr as *mut u8).add((*dr).entry_size as usize * (*dr).prod as usize) as *mut core::ffi::c_void }
pub unsafe fn vio_dring_entry(dr:*mut vio_dring_state,index:u32)->*mut core::ffi::c_void { (dr as *mut u8).add((*dr).entry_size as usize * index as usize) as *mut core::ffi::c_void }
pub unsafe fn vio_dring_avail(dr:*mut vio_dring_state,ring_size:u32)->u32 { (*dr).pending - (((*dr).prod - (*dr).cons) & (ring_size-1)) - 1 }
pub unsafe fn vio_dring_next(dr:*mut vio_dring_state,mut index:u32)->u32 { index=index.wrapping_add(1); if index==(*dr).num_entries {index=0}; index }
pub unsafe fn vio_dring_prev(dr:*mut vio_dring_state,index:u32)->u32 { if index==0 {(*dr).num_entries-1} else {index-1} }

pub const VIO_MAX_TYPE_LEN:usize=32; pub const VIO_MAX_NAME_LEN:usize=32; pub const VIO_MAX_COMPAT_LEN:usize=64;
#[repr(C)] pub struct vio_dev { pub mp:u64,pub dp:*mut device_node,pub node_name:[i8;32],pub type_:[i8;32],pub compat:[i8;64],pub compat_len:i32,pub dev_no:u64,pub port_id:usize,pub channel_id:usize,pub tx_irq:u32,pub rx_irq:u32,pub rx_ino:u64,pub tx_ino:u64,pub cdev_handle:u64,pub md_node_info:md_node_info,pub dev:device }
#[repr(C)] pub struct vio_driver { pub name:*const i8,pub node:list_head,pub id_table:*const vio_device_id,pub probe:Option<unsafe extern "C" fn(*mut vio_dev,*const vio_device_id)->i32>,pub remove:Option<unsafe extern "C" fn(*mut vio_dev)>,pub shutdown:Option<unsafe extern "C" fn(*mut vio_dev)>,pub driver_data:usize,pub driver:device_driver,pub no_irq:bool }
#[repr(C)] pub struct vio_version { pub major:u16,pub minor:u16 }
#[repr(C)] pub struct vio_driver_ops { pub send_attr:Option<unsafe extern "C" fn(*mut vio_driver_state)->i32>,pub handle_attr:Option<unsafe extern "C" fn(*mut vio_driver_state,*mut core::ffi::c_void)->i32>,pub handshake_complete:Option<unsafe extern "C" fn(*mut vio_driver_state)> }
#[repr(C)] pub struct vio_completion { pub com:completion,pub err:i32,pub waiting_for:i32 }
#[repr(C)] pub struct vio_driver_state { pub lock:spinlock_t,pub lp:*mut ldc_channel,pub _peer_sid:u32,pub _local_sid:u32,pub drings:[vio_dring_state;2],pub hs_state:u8,pub dev_class:u8,pub dr_state:u8,pub debug:u8,pub desc_buf:*mut core::ffi::c_void,pub desc_buf_len:u32,pub cmp:*mut vio_completion,pub vdev:*mut vio_dev,pub timer:timer_list,pub ver:vio_version,pub ver_table:*mut vio_version,pub ver_table_entries:i32,pub name:*mut i8,pub ops:*mut vio_driver_ops }
pub const VIO_DRIVER_TX_RING:usize=0; pub const VIO_DRIVER_RX_RING:usize=1; pub const VIO_HS_INVALID:u8=0; pub const VIO_HS_GOTVERS:u8=1; pub const VIO_HS_GOT_ATTR:u8=4; pub const VIO_HS_SENT_DREG:u8=8; pub const VIO_HS_SENT_RDX:u8=0x10; pub const VIO_HS_GOT_RDX_ACK:u8=0x20; pub const VIO_HS_GOT_RDX:u8=0x40; pub const VIO_HS_SENT_RDX_ACK:u8=0x80; pub const VIO_HS_COMPLETE:u8=0xa0; pub const VIO_DR_STATE_TXREG:u8=1; pub const VIO_DR_STATE_RXREG:u8=2; pub const VIO_DR_STATE_TXREQ:u8=0x10; pub const VIO_DR_STATE_RXREQ:u8=0x20; pub const VIO_DEBUG_HS:u8=1; pub const VIO_DEBUG_DATA:u8=2;
pub unsafe fn vio_version_before(vio:*mut vio_driver_state,major:u16,minor:u16)->bool { ((((*vio).ver.major as u32)<<16)|(*vio).ver.minor as u32) < (((major as u32)<<16)|minor as u32) }
pub unsafe fn vio_version_after(vio:*mut vio_driver_state,major:u16,minor:u16)->bool { ((((*vio).ver.major as u32)<<16)|(*vio).ver.minor as u32) > (((major as u32)<<16)|minor as u32) }
pub unsafe fn vio_version_after_eq(vio:*mut vio_driver_state,major:u16,minor:u16)->bool { ((((*vio).ver.major as u32)<<16)|(*vio).ver.minor as u32) >= (((major as u32)<<16)|minor as u32) }

extern "C" { pub fn __vio_register_driver(drv:*mut vio_driver,owner:*mut module,mod_name:*const i8)->i32; pub fn vio_unregister_driver(drv:*mut vio_driver); pub fn vio_ldc_send(vio:*mut vio_driver_state,data:*mut core::ffi::c_void,len:i32)->i32; pub fn vio_link_state_change(vio:*mut vio_driver_state,event:i32); pub fn vio_conn_reset(vio:*mut vio_driver_state); pub fn vio_control_pkt_engine(vio:*mut vio_driver_state,pkt:*mut core::ffi::c_void)->i32; pub fn vio_validate_sid(vio:*mut vio_driver_state,tp:*mut vio_msg_tag)->i32; pub fn vio_send_sid(vio:*mut vio_driver_state)->u32; pub fn vio_ldc_alloc(vio:*mut vio_driver_state,base_cfg:*mut ldc_channel_config,event_arg:*mut core::ffi::c_void)->i32; pub fn vio_ldc_free(vio:*mut vio_driver_state); pub fn vio_driver_init(vio:*mut vio_driver_state,vdev:*mut vio_dev,dev_class:u8,ver_table:*mut vio_version,ver_table_size:i32,ops:*mut vio_driver_ops,name:*mut i8)->i32; pub fn vio_port_up(vio:*mut vio_driver_state); pub fn vio_set_intr(dev_ino:usize,state:i32)->i32; pub fn vio_vdev_node(hp:*mut mdesc_handle,vdev:*mut vio_dev)->u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
