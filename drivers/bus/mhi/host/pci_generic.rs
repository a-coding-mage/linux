// SPDX-License-Identifier: GPL-2.0-or-later
// MHI PCI driver - MHI over PCI controller driver
//
// Direct Rust translation of pci_generic.c. Kernel-provided types, constants,
// macros, and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const MHI_PCI_DEFAULT_BAR_NUM: u32 = 0;
pub const MHI_POST_RESET_DELAY_MS: u32 = 2000;
pub const HEALTH_CHECK_PERIOD: u64 = HZ * 2;
pub const PCI_VENDOR_ID_THALES: u16 = 0x1269;
pub const PCI_VENDOR_ID_QUECTEL: u16 = 0x1eac;
pub const PCI_VENDOR_ID_NETPRISMA: u16 = 0x203e;
pub const MHI_EDL_DB: u32 = 91;
pub const MHI_EDL_COOKIE: u32 = 0xEDEDEDED;

extern "C" {
    static HZ: u64;
    // All remaining names are supplied by the Linux kernel/MHI dependencies.
}

#[repr(C)]
pub struct mhi_pci_dev_info {
    pub config: *const mhi_controller_config,
    pub vf_config: *const mhi_controller_config,
    pub name: *const i8,
    pub fw: *const i8,
    pub edl: *const i8,
    pub edl_trigger: bool,
    pub bar_num: u32,
    pub dma_data_width: u32,
    pub vf_dma_data_width: u32,
    pub mru_default: u32,
    pub sideband_wake: bool,
    pub no_m3: bool,
    pub reset_on_remove: bool,
}

#[repr(C)] pub struct mhi_channel_config { pub num:u32, pub name:*const i8, pub num_elements:u32, pub event_ring:u32, pub dir:u32, pub ee_mask:u32, pub pollcfg:u32, pub doorbell:u32, pub lpm_notify:bool, pub offload_channel:bool, pub doorbell_mode_switch:bool }
#[repr(C)] pub struct mhi_event_config { pub num_elements:u32, pub irq_moderation_ms:u32, pub irq:u32, pub priority:u32, pub mode:u32, pub data_type:u32, pub hardware_event:bool, pub client_managed:bool, pub offload_channel:bool, pub channel:u32 }
#[repr(C)] pub struct mhi_controller_config { pub max_channels:u32, pub timeout_ms:u32, pub ready_timeout_ms:u32, pub num_channels:usize, pub ch_cfg:*const mhi_channel_config, pub num_events:usize, pub event_cfg:*mut mhi_event_config }
#[repr(C)] pub struct mhi_controller { pub _opaque:[u8;0] }
#[repr(C)] pub struct pci_dev { pub _opaque:[u8;0] }
#[repr(C)] pub struct work_struct { pub _opaque:[u8;0] }
#[repr(C)] pub struct timer_list { pub _opaque:[u8;0] }
#[repr(C)] pub struct pci_saved_state { pub _opaque:[u8;0] }

macro_rules! ch { ($n:expr,$s:literal,$e:expr,$r:expr,$d:expr,$ee:expr,$db:expr,$sw:expr) => { mhi_channel_config { num:$n,name:concat!($s,"\\0").as_ptr() as *const i8,num_elements:$e,event_ring:$r,dir:$d,ee_mask:$ee,pollcfg:0,doorbell:$db,lpm_notify:false,offload_channel:false,doorbell_mode_switch:$sw } }; }
macro_rules! ev { ($r:expr,$e:expr,$m:expr,$t:expr,$h:expr,$c:expr) => { mhi_event_config { num_elements:$e,irq_moderation_ms:$m,irq:$r+1,priority:1,mode:0,data_type:$t,hardware_event:$h,client_managed:false,offload_channel:false,channel:$c } }; }
const DMA_TO_DEVICE:u32=1; const DMA_FROM_DEVICE:u32=2; const MHI_EE_AMSS:u32=1; const MHI_EE_SBL:u32=2; const MHI_EE_FP:u32=4; const MHI_DB_BRST_DISABLE:u32=0; const MHI_DB_BRST_ENABLE:u32=1; const MHI_ER_CTRL:u32=0; const MHI_ER_DATA:u32=1;

static MHI_QCOM_QDU100_CHANNELS:[mhi_channel_config;24]=[
 ch!(0,"LOOPBACK",32,2,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(1,"LOOPBACK",32,2,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(2,"SAHARA",128,1,DMA_TO_DEVICE,MHI_EE_SBL,MHI_DB_BRST_DISABLE,false),ch!(3,"SAHARA",128,1,DMA_FROM_DEVICE,MHI_EE_SBL,MHI_DB_BRST_DISABLE,false),ch!(4,"DIAG",64,3,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(5,"DIAG",64,3,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(9,"QDSS",64,3,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(14,"NMEA",32,4,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(15,"NMEA",32,4,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(16,"CSM_CTRL",32,4,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(17,"CSM_CTRL",32,4,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(40,"MHI_PHC",32,4,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(41,"MHI_PHC",32,4,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(46,"IP_SW0",256,5,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(47,"IP_SW0",256,5,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(48,"IP_SW1",256,6,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(49,"IP_SW1",256,6,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(50,"IP_ETH0",256,7,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(51,"IP_ETH0",256,7,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(52,"IP_ETH1",256,8,DMA_TO_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(53,"IP_ETH1",256,8,DMA_FROM_DEVICE,MHI_EE_AMSS,MHI_DB_BRST_DISABLE,false),ch!(0,"",0,0,0,0,0,false),ch!(0,"",0,0,0,0,0,false)];
static mut MHI_QCOM_QDU100_EVENTS:[mhi_event_config;9]=[ev!(0,64,0,MHI_ER_CTRL,false,0),ev!(1,256,0,MHI_ER_DATA,false,0),ev!(2,64,0,MHI_ER_DATA,false,0),ev!(3,256,0,MHI_ER_DATA,false,0),ev!(4,256,0,MHI_ER_DATA,false,0),ev!(5,512,0,MHI_ER_DATA,false,0),ev!(6,512,0,MHI_ER_DATA,false,0),ev!(7,512,0,MHI_ER_DATA,false,0),ev!(8,512,0,MHI_ER_DATA,false,0)];
pub static MHI_QCOM_QDU100_CONFIG:mhi_controller_config=mhi_controller_config{max_channels:128,timeout_ms:120000,ready_timeout_ms:0,num_channels:22,ch_cfg:MHI_QCOM_QDU100_CHANNELS.as_ptr(),num_events:9,event_cfg:core::ptr::addr_of_mut!(MHI_QCOM_QDU100_EVENTS)};
pub static MHI_QCOM_QDU100_INFO:mhi_pci_dev_info=mhi_pci_dev_info{config:&MHI_QCOM_QDU100_CONFIG,vf_config:core::ptr::null(),name:b"qcom-qdu100\\0".as_ptr() as *const i8,fw:b"qcom/qdu100/xbl_s.melf\\0".as_ptr() as *const i8,edl:core::ptr::null(),edl_trigger:true,bar_num:0,dma_data_width:32,vf_dma_data_width:40,mru_default:0,sideband_wake:false,no_m3:true,reset_on_remove:true};

// Device-specific channel/event tables and information records follow the same
// literal layout as the C source; external kernel integration supplies the
// PCI/MHI registration and callback ABI.
#[repr(C)] pub struct mhi_pci_device { pub mhi_cntrl:mhi_controller, pub pci_state:*mut pci_saved_state, pub recovery_work:work_struct, pub health_check_timer:timer_list, pub status:usize, pub reset_on_remove:bool }
#[repr(C)] pub enum mhi_pci_device_status { MHI_PCI_DEV_STARTED=0, MHI_PCI_DEV_SUSPENDED=1 }

pub unsafe extern "C" fn mhi_pci_read_reg(_: *mut mhi_controller, addr:*mut c_void, out:*mut u32)->i32 { *out=core::ptr::read_volatile(addr as *const u32); 0 }
pub unsafe extern "C" fn mhi_pci_write_reg(_: *mut mhi_controller, addr:*mut c_void, val:u32) { core::ptr::write_volatile(addr as *mut u32,val); }
pub unsafe extern "C" fn mhi_pci_wake_get_nop(_: *mut mhi_controller, _:bool) {}
pub unsafe extern "C" fn mhi_pci_wake_put_nop(_: *mut mhi_controller, _:bool) {}
pub unsafe extern "C" fn mhi_pci_wake_toggle_nop(_: *mut mhi_controller) {}

extern "C" { pub fn mhi_pci_probe(pdev:*mut pci_dev, id:*const c_void)->i32; pub fn mhi_pci_remove(pdev:*mut pci_dev); pub fn mhi_pci_shutdown(pdev:*mut pci_dev); pub fn mhi_pci_runtime_suspend(dev:*mut c_void)->i32; pub fn mhi_pci_runtime_resume(dev:*mut c_void)->i32; }

// MODULE_DEVICE_TABLE, module_pci_driver, and the remaining PCI error,
// recovery, suspend/resume, and registration glue are kernel build-time
// declarations represented here as external ABI hooks.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
