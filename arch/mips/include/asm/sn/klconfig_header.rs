/* Translated from klconfig.h. External types and address helpers are supplied by dependent headers. */
use std::ffi::c_void;
use std::os::raw::{c_int, c_uint, c_ulong};

pub type nic_t = u64;
pub const KLCFGINFO_MAGIC: u32 = 0xbeedbabe;
pub type klconf_off_t = i32;

pub const MAX_ROUTER_PORTS: usize = 6;
pub const MAX_MODULE_ID: u32 = 255;
pub const SIZE_PAD: usize = 4096;
pub const MAX_SLOTS_PER_NODE: usize = 1 + 2 + 6 + 2;
pub const MAX_PCI_DEVS: usize = 8;
pub const ENABLE_BOARD: u8 = 0x01;
pub const FAILED_BOARD: u8 = 0x02;
pub const DUPLICATE_BOARD: u8 = 0x04;
pub const VISITED_BOARD: u8 = 0x08;
pub const LOCAL_MASTER_IO6: u8 = 0x10;
pub const GLOBAL_MASTER_IO6: u8 = 0x20;
pub const THIRD_NIC_PRESENT: u8 = 0x40;
pub const SECOND_NIC_PRESENT: u8 = 0x80;
pub const KLINFO_ENABLE: u8 = 0x01;
pub const KLINFO_FAILED: u8 = 0x02;
pub const KLINFO_DEVICE: u8 = 0x04;
pub const KLINFO_VISITED: u8 = 0x08;
pub const KLINFO_CONTROLLER: u8 = 0x10;
pub const KLINFO_INSTALL: u8 = 0x20;
pub const KLINFO_HEADLESS: u8 = 0x40;
pub const GB2: u32 = 0x80000000;
pub const MAX_RSV_PTRS: usize = 32;
pub const BOARD_STRUCT: usize = 0;
pub const COMPONENT_STRUCT: usize = 1;
pub const ERRINFO_STRUCT: usize = 2;
pub const KLMALLOC_TYPE_MAX: usize = ERRINFO_STRUCT + 1;
pub const DEVICE_STRUCT: usize = 3;

#[repr(C)]
pub struct console_t { pub uart_base: c_ulong, pub config_base: c_ulong, pub memory_base: c_ulong, pub baud: i16, pub flag: i16, pub r#type: c_int, pub nasid: nasid_t, pub wid: i8, pub npci: i8, pub baseio_nic: nic_t }
#[repr(C)] pub struct klc_malloc_hdr_t { pub km_base: klconf_off_t, pub km_limit: klconf_off_t, pub km_current: klconf_off_t }
#[repr(C)] pub struct kl_config_hdr_t { pub ch_magic: u64, pub ch_version: u32, pub ch_malloc_hdr_off: klconf_off_t, pub ch_cons_off: klconf_off_t, pub ch_board_info: klconf_off_t, pub ch_cons_info: console_t, pub ch_malloc_hdr: [klc_malloc_hdr_t; KLMALLOC_TYPE_MAX], pub ch_sw_belief: confidence_t, pub ch_sn0net_belief: confidence_t }

pub const KLCLASS_MASK: u8 = 0xf0; pub const KLCLASS_NONE: u8 = 0; pub const KLCLASS_NODE: u8 = 0x10; pub const KLCLASS_CPU: u8 = KLCLASS_NODE; pub const KLCLASS_IO: u8 = 0x20; pub const KLCLASS_ROUTER: u8 = 0x30; pub const KLCLASS_MIDPLANE: u8 = 0x40; pub const KLCLASS_GFX: u8 = 0x50; pub const KLCLASS_PSEUDO_GFX: u8 = 0x60; pub const KLCLASS_MAX: u8 = 7; pub const KLTYPE_MAX: u8 = 10; pub const KLCLASS_UNKNOWN: u8 = 0xf0;
pub const KLTYPE_MASK: u8 = 0x0f; pub const KLTYPE_NONE: u8 = 0; pub const KLTYPE_EMPTY: u8 = 0; pub const KLTYPE_WEIRDCPU: u8 = KLCLASS_CPU; pub const KLTYPE_IP27: u8 = KLCLASS_CPU|1; pub const KLTYPE_WEIRDIO: u8 = KLCLASS_IO; pub const KLTYPE_BASEIO: u8 = KLCLASS_IO|1; pub const KLTYPE_IO6: u8 = KLTYPE_BASEIO; pub const KLTYPE_4CHSCSI: u8 = KLCLASS_IO|2; pub const KLTYPE_MSCSI: u8 = KLTYPE_4CHSCSI; pub const KLTYPE_ETHERNET: u8 = KLCLASS_IO|3; pub const KLTYPE_MENET: u8 = KLTYPE_ETHERNET; pub const KLTYPE_FDDI: u8 = KLCLASS_IO|4; pub const KLTYPE_UNUSED: u8 = KLCLASS_IO|5; pub const KLTYPE_HAROLD: u8 = KLCLASS_IO|6; pub const KLTYPE_PCI: u8 = KLTYPE_HAROLD; pub const KLTYPE_VME: u8 = KLCLASS_IO|7; pub const KLTYPE_MIO: u8 = KLCLASS_IO|8; pub const KLTYPE_FC: u8 = KLCLASS_IO|9; pub const KLTYPE_LINC: u8 = KLCLASS_IO|0xA; pub const KLTYPE_TPU: u8 = KLCLASS_IO|0xB; pub const KLTYPE_GSN_A: u8 = KLCLASS_IO|0xC; pub const KLTYPE_GSN_B: u8 = KLCLASS_IO|0xD;
pub const KLTYPE_GFX: u8 = KLCLASS_GFX; pub const KLTYPE_GFX_KONA: u8 = KLCLASS_GFX|1; pub const KLTYPE_GFX_MGRA: u8 = KLCLASS_GFX|3; pub const KLTYPE_WEIRDROUTER: u8 = KLCLASS_ROUTER; pub const KLTYPE_ROUTER: u8 = KLCLASS_ROUTER|1; pub const KLTYPE_ROUTER2: u8 = KLTYPE_ROUTER; pub const KLTYPE_NULL_ROUTER: u8 = KLCLASS_ROUTER|2; pub const KLTYPE_META_ROUTER: u8 = KLCLASS_ROUTER|3; pub const KLTYPE_WEIRDMIDPLANE: u8 = KLCLASS_MIDPLANE; pub const KLTYPE_MIDPLANE8: u8 = KLCLASS_MIDPLANE|1; pub const KLTYPE_MIDPLANE: u8 = KLTYPE_MIDPLANE8; pub const KLTYPE_PBRICK_XBOW: u8 = KLCLASS_MIDPLANE|2; pub const KLTYPE_XTHD: u8 = KLCLASS_PSEUDO_GFX|9; pub const KLTYPE_UNKNOWN: u8 = KLCLASS_UNKNOWN|0xf;
pub const MAX_COMPTS_PER_BRD: usize = 24; pub const LOCAL_BOARD: u8 = 1; pub const REMOTE_BOARD: u8 = 2; pub const LBOARD_STRUCT_VERSION: u8 = 2;
pub const IP27_CPU0_INDEX: usize=0; pub const IP27_CPU1_INDEX: usize=1; pub const IP27_HUB_INDEX: usize=2; pub const IP27_MEM_INDEX: usize=3; pub const BASEIO_BRIDGE_INDEX: usize=0; pub const BASEIO_IOC3_INDEX: usize=1; pub const BASEIO_SCSI1_INDEX: usize=2; pub const BASEIO_SCSI2_INDEX: usize=3; pub const MIDPLANE_XBOW_INDEX: usize=0; pub const ROUTER_COMPONENT_INDEX: usize=0; pub const CH4SCSI_BRIDGE_INDEX: usize=0;
pub const CPU_STRUCT_VERSION:u8=2; pub const MEMORY_STRUCT_VERSION:u8=2; pub const MAX_XBOW_LINKS:usize=16; pub const MAX_PCI_SLOTS:usize=8; pub const BRIDGE_STRUCT_VERSION:u8=2; pub const MAX_IOC3_TTY:usize=2; pub const MAX_VME_SLOTS:usize=8; pub const ROUTER_VECTOR_VERS:u8=2; pub const KLGFX_COOKIE:u32=0x0c0de000; pub const MAX_SCSI_DEVS:usize=16; pub const MAX_FDDI_DEVS:usize=10;
pub const KL_CPU_R4000:u8=1; pub const KL_CPU_TFP:u8=2; pub const KL_CPU_R10000:u8=3; pub const KL_CPU_NONE:i8=-1;
pub const BRI_PER_XBOW:usize=6; pub const PCI_PER_BRI:usize=8; pub const DEV_PER_PCI:usize=16;

#[repr(C)] pub struct lboard_t { pub brd_next: klconf_off_t, pub struct_type: u8, pub brd_type: u8, pub brd_sversion: u8, pub brd_brevision: u8, pub brd_promver: u8, pub brd_flags: u8, pub brd_slot: u8, pub brd_debugsw: u16, pub brd_module: moduleid_t, pub brd_partition: partid_t, pub brd_diagval: u16, pub brd_diagparm: u16, pub brd_inventory: u8, pub brd_numcompts: u8, pub brd_nic: nic_t, pub brd_nasid: nasid_t, pub brd_compts: [klconf_off_t; MAX_COMPTS_PER_BRD], pub brd_errinfo: klconf_off_t, pub brd_parent: *mut lboard_t, pub brd_graph_link: vertex_hdl_t, pub brd_confidence: confidence_t, pub brd_owner: nasid_t, pub brd_nic_flags: u8, pub brd_name: [i8; 32] }
#[repr(C)] pub struct klinfo_t { pub struct_type: u8, pub struct_version: u8, pub flags: u8, pub revision: u8, pub diagval: u16, pub diagparm: u16, pub inventory: u8, pub nic: nic_t, pub physid: u8, pub virtid: c_uint, pub widid: u8, pub nasid: nasid_t, pub pad1: i8, pub pad2: i8, pub arcs_compt: *mut COMPONENT, pub errinfo: klconf_off_t, pub pad3: u16, pub pad4: u16 }
pub const KLSTRUCT_UNKNOWN: u8=0; pub const KLSTRUCT_CPU:u8=1; pub const KLSTRUCT_HUB:u8=2; pub const KLSTRUCT_MEMBNK:u8=3; pub const KLSTRUCT_XBOW:u8=4; pub const KLSTRUCT_BRI:u8=5; pub const KLSTRUCT_IOC3:u8=6; pub const KLSTRUCT_PCI:u8=7; pub const KLSTRUCT_VME:u8=8; pub const KLSTRUCT_ROU:u8=9; pub const KLSTRUCT_GFX:u8=10; pub const KLSTRUCT_SCSI:u8=11; pub const KLSTRUCT_FDDI:u8=12; pub const KLSTRUCT_MIO:u8=13; pub const KLSTRUCT_DISK:u8=14; pub const KLSTRUCT_TAPE:u8=15; pub const KLSTRUCT_CDROM:u8=16; pub const KLSTRUCT_HUB_UART:u8=17; pub const KLSTRUCT_IOC3ENET:u8=18; pub const KLSTRUCT_IOC3UART:u8=19; pub const KLSTRUCT_UNUSED:u8=20; pub const KLSTRUCT_IOC3PCKM:u8=21; pub const KLSTRUCT_RAD:u8=22; pub const KLSTRUCT_HUB_TTY:u8=23; pub const KLSTRUCT_IOC3_TTY:u8=24; pub const KLSTRUCT_FIBERCHANNEL:u8=25; pub const KLSTRUCT_MOD_SERIAL_NUM:u8=26; pub const KLSTRUCT_IOC3MS:u8=27; pub const KLSTRUCT_TPU:u8=28; pub const KLSTRUCT_GSN_A:u8=29; pub const KLSTRUCT_GSN_B:u8=30; pub const KLSTRUCT_XTHD:u8=31;

pub type pci_t = *mut u64; pub type vmeb_t=*mut u64; pub type vmed_t=*mut u64; pub type fddi_t=*mut u64; pub type scsi_t=*mut u64; pub type mio_t=*mut u64; pub type graphics_t=*mut u64; pub type router_t=*mut u64;
#[repr(C)] pub struct klport_t { pub port_nasid: nasid_t, pub port_flag: u8, pub port_offset: klconf_off_t }
#[repr(C)] pub struct klcpu_t { pub cpu_info: klinfo_t, pub cpu_prid:u16, pub cpu_fpirr:u16, pub cpu_speed:u16, pub cpu_scachesz:u16, pub cpu_scachespeed:u16 }
#[repr(C)] pub struct klhub_t { pub hub_info: klinfo_t, pub hub_flags:c_uint, pub hub_port:klport_t, pub hub_box_nic:nic_t, pub hub_mfg_nic:klconf_off_t, pub hub_speed:u64 }
#[repr(C)] pub struct klhub_uart_t { pub hubuart_info:klinfo_t, pub hubuart_flags:c_uint, pub hubuart_box_nic:nic_t }
#[repr(C)] pub struct klmembnk_t { pub membnk_info:klinfo_t, pub membnk_memsz:i16, pub membnk_dimm_select:i16, pub membnk_bnksz:[i16; MD_MEM_BANKS], pub membnk_attr:i16 }
#[repr(C)] pub union klmod_serial_num_t_snum { pub snum_str:[i8;10], pub snum_int:u64 }
#[repr(C)] pub struct klmod_serial_num_t { pub snum_info:klinfo_t, pub snum:klmod_serial_num_t_snum }
#[repr(C)] pub struct klxbow_t { pub xbow_info:klinfo_t, pub xbow_port_info:[klport_t;16], pub xbow_master_hub_link:c_int }
#[repr(C)] pub struct klpci_device_t { pub pci_device_id:i32, pub pci_device_pad:i32 }
#[repr(C)] pub struct klbri_t { pub bri_info:klinfo_t, pub bri_eprominfo:u8, pub bri_bustype:u8, pub pci_specific:pci_t, pub bri_devices:[klpci_device_t;8], pub bri_mfg_nic:klconf_off_t }
#[repr(C)] pub struct klioc3_t { pub ioc3_info:klinfo_t, pub ioc3_ssram:u8, pub ioc3_nvram:u8, pub ioc3_superio:klinfo_t, pub ioc3_tty_off:klconf_off_t, pub ioc3_enet:klinfo_t, pub ioc3_enet_off:klconf_off_t, pub ioc3_kbd_off:klconf_off_t }
#[repr(C)] pub struct klvmeb_t { pub vmeb_info:klinfo_t, pub vmeb_specific:vmeb_t, pub vmeb_brdinfo:[klconf_off_t;8] }
#[repr(C)] pub struct klvmed_t { pub vmed_info:klinfo_t, pub vmed_specific:vmed_t, pub vmed_brdinfo:[klconf_off_t;8] }
#[repr(C)] pub struct klrou_t { pub rou_info:klinfo_t, pub rou_flags:c_uint, pub rou_box_nic:nic_t, pub rou_port:[klport_t; MAX_ROUTER_PORTS+1], pub rou_mfg_nic:klconf_off_t, pub rou_vector:u64 }
#[repr(C)] pub struct klgfx_t { pub gfx_info:klinfo_t, pub old_gndevs:klconf_off_t, pub old_gdoff0:klconf_off_t, pub cookie:c_uint, pub moduleslot:c_uint, pub gfx_next_pipe:*mut klgfx_t, pub gfx_specific:graphics_t, pub pad0:klconf_off_t, pub gfx_mfg_nic:klconf_off_t }
#[repr(C)] pub struct klxthd_t { pub xthd_info:klinfo_t, pub xthd_mfg_nic:klconf_off_t }
#[repr(C)] pub struct kltpu_t { pub tpu_info:klinfo_t, pub tpu_mfg_nic:klconf_off_t }
#[repr(C)] pub struct klgsn_t { pub gsn_info:klinfo_t, pub gsn_mfg_nic:klconf_off_t }
#[repr(C)] pub struct klscsi_t { pub scsi_info:klinfo_t, pub scsi_specific:scsi_t, pub scsi_numdevs:u8, pub scsi_devinfo:[klconf_off_t;16] }
#[repr(C)] pub struct klscdev_t { pub scdev_info:klinfo_t, pub scdev_cfg:*mut scsidisk_data }
#[repr(C)] pub struct klttydev_t { pub ttydev_info:klinfo_t, pub ttydev_cfg:*mut terminal_data }
#[repr(C)] pub struct klenetdev_t { pub enetdev_info:klinfo_t, pub enetdev_cfg:*mut net_data }
#[repr(C)] pub struct klkbddev_t { pub kbddev_info:klinfo_t, pub kbddev_cfg:*mut keyboard_data }
#[repr(C)] pub struct klmsdev_t { pub msdev_info:klinfo_t, pub msdev_cfg:*mut c_void }
#[repr(C)] pub struct klfddi_t { pub fddi_info:klinfo_t, pub fddi_specific:fddi_t, pub fddi_devinfo:[klconf_off_t;10] }
#[repr(C)] pub struct klmio_t { pub mio_info:klinfo_t, pub mio_specific:mio_t }

#[repr(C)] pub union klcomp_t { pub kc_cpu:klcpu_t, pub kc_hub:klhub_t, pub kc_mem:klmembnk_t, pub kc_xbow:klxbow_t, pub kc_bri:klbri_t, pub kc_ioc3:klioc3_t, pub kc_vmeb:klvmeb_t, pub kc_vmed:klvmed_t, pub kc_rou:klrou_t, pub kc_gfx:klgfx_t, pub kc_scsi:klscsi_t, pub kc_scsi_dev:klscdev_t, pub kc_fddi:klfddi_t, pub kc_mio:klmio_t, pub kc_snum:klmod_serial_num_t }
#[repr(C)] pub union kldev_t { pub kc_scsi_dev:klscdev_t, pub kc_tty_dev:klttydev_t, pub kc_enet_dev:klenetdev_t, pub kc_kbd_dev:klkbddev_t }
#[repr(C)] pub union biptr_t { pub lbinfo:*mut lboard_t }

pub const VDS_NOGFX:u16=0x8000; pub const VDS_NOMP:u16=0x100; pub const VDS_MANUMODE:u16=0x80; pub const VDS_NOARB:u16=0x40; pub const VDS_PODMODE:u16=0x20; pub const VDS_NO_DIAGS:u16=0x10; pub const VDS_DEFAULTS:u16=8; pub const VDS_NOMEMCLEAR:u16=4; pub const VDS_2ND_IO4:u16=2; pub const VDS_DEBUG_PROM:u16=1;

extern "C" { pub fn find_lboard(start:*mut lboard_t, r#type:u8)->*mut lboard_t; pub fn find_component(brd:*mut lboard_t, kli:*mut klinfo_t, r#type:u8)->*mut klinfo_t; pub fn find_first_component(brd:*mut lboard_t, r#type:u8)->*mut klinfo_t; pub fn find_lboard_class(start:*mut lboard_t, brd_class:u8)->*mut lboard_t; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
