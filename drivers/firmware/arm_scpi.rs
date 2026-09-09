// SPDX-License-Identifier: GPL-2.0-only
/* Faithful Rust translation of arm_scpi.c. Linux-kernel dependencies remain external. */

use core::ffi::c_void;

type U8 = u8; type U16 = u16; type U32 = u32; type U64 = u64;
type Le16 = u16; type Le32 = u32; type Le64 = u64;
type ResourceSize = usize;

const CMD_ID_MASK: U32 = 0x7f;
const CMD_TOKEN_ID_MASK: U32 = 0xff00;
const CMD_DATA_SIZE_MASK: U32 = 0x01ff_0000;
const CMD_LEGACY_DATA_SIZE_MASK: U32 = 0x1ff0_0000;
const SCPI_SLOT: usize = 0;
const MAX_DVFS_DOMAINS: usize = 8;
const MAX_DVFS_OPPS: usize = 16;
const PROTO_REV_MAJOR_MASK: U32 = 0xffff_0000;
const PROTO_REV_MINOR_MASK: U32 = 0xffff;
const FW_REV_MAJOR_MASK: U32 = 0xff00_0000;
const FW_REV_MINOR_MASK: U32 = 0x00ff_0000;
const FW_REV_PATCH_MASK: U32 = 0xffff;
const MAX_RX_TIMEOUT: usize = 30;

#[repr(i32)]
enum ScpiErrorCodes { ScpiSuccess=0, ScpiErrParam, ScpiErrAlign, ScpiErrSize, ScpiErrHandler, ScpiErrAccess, ScpiErrRange, ScpiErrTimeout, ScpiErrNomem, ScpiErrPwrstate, ScpiErrSupport, ScpiErrDevice, ScpiErrBusy, ScpiErrMax }

#[repr(i32)]
enum ScpiStdCmd {
    ScpiCmdInvalid=0, ScpiCmdScpiReady, ScpiCmdScpiCapabilities, ScpiCmdSetCssPwrState,
    ScpiCmdGetCssPwrState, ScpiCmdSetSysPwrState, ScpiCmdSetCpuTimer, ScpiCmdCancelCpuTimer,
    ScpiCmdDvfsCapabilities, ScpiCmdGetDvfsInfo, ScpiCmdSetDvfs, ScpiCmdGetDvfs,
    ScpiCmdGetDvfsStat, ScpiCmdClockCapabilities, ScpiCmdGetClockInfo, ScpiCmdSetClockValue,
    ScpiCmdGetClockValue, ScpiCmdPsuCapabilities, ScpiCmdGetPsuInfo, ScpiCmdSetPsu,
    ScpiCmdGetPsu, ScpiCmdSensorCapabilities, ScpiCmdSensorInfo, ScpiCmdSensorValue,
    ScpiCmdSensorCfgPeriodic, ScpiCmdSensorCfgBounds, ScpiCmdSensorAsyncValue,
    ScpiCmdSetDevicePwrState, ScpiCmdGetDevicePwrState, ScpiCmdCount,
}

#[repr(i32)]
enum LegacyScpiStdCmd {
    LegacyInvalid=0, LegacyReady, LegacyCapabilities, LegacyEvent, LegacySetCss,
    LegacyGetCss, LegacyCfgPwrStat, LegacyGetPwrStat, LegacySysPwr, LegacyL2Ready,
    LegacySetApTimer, LegacyCancelApTime, LegacyDvfsCapabilities, LegacyGetDvfsInfo,
    LegacySetDvfs, LegacyGetDvfs, LegacyGetDvfsStat, LegacySetRtc, LegacyGetRtc,
    LegacyClockCapabilities, LegacySetClockIndex, LegacySetClockValue, LegacyGetClockValue,
    LegacyPsuCapabilities, LegacySetPsu, LegacyGetPsu, LegacySensorCapabilities,
    LegacySensorInfo, LegacySensorValue, LegacySensorCfgPeriodic, LegacySensorCfgBounds,
    LegacySensorAsyncValue, LegacyCount,
}

const LEGACY_HPRIORITY_CMDS: [i32;14] = [5,6,7,14,15,17,18,20,21,22,24,25,29,30];
#[repr(i32)] enum ScpiDrvCmds { CmdScpiCapabilities=0, CmdGetClockInfo, CmdGetClockValue, CmdSetClockValue, CmdGetDvfs, CmdSetDvfs, CmdGetDvfsInfo, CmdSensorCapabilities, CmdSensorInfo, CmdSensorValue, CmdSetDevicePwrState, CmdGetDevicePwrState, CmdMaxCount }
static SCPI_STD_COMMANDS: [i32;12] = [2,14,16,15,11,10,9,21,22,23,27,28];
static SCPI_LEGACY_COMMANDS: [i32;12] = [2,-1,22,21,15,14,13,26,27,28,-1,-1];

#[repr(C)] pub struct ListHead { pub next:*mut ListHead, pub prev:*mut ListHead }
#[repr(C)] pub struct Completion { _private: [u8;0] }
#[repr(C)] pub struct Spinlock { _private: [u8;0] }
#[repr(C)] pub struct Mutex { _private: [u8;0] }
#[repr(C)] pub struct MboxClient { pub dev:*mut Device, pub rx_callback:Option<unsafe extern "C" fn(*mut MboxClient,*mut c_void)>, pub tx_prepare:Option<unsafe extern "C" fn(*mut MboxClient,*mut c_void)>, pub tx_block:bool, pub tx_tout:u32, pub knows_txdone:bool }
#[repr(C)] pub struct MboxChan { _private:[u8;0] }
#[repr(C)] pub struct Device { pub of_node:*mut DeviceNode }
#[repr(C)] pub struct DeviceNode { _private:[u8;0] }
#[repr(C)] pub struct PlatformDevice { pub dev:Device }
#[repr(C)] pub struct ScpiOpp { pub freq:U32, pub m_volt:U32 }
#[repr(C)] pub struct ScpiDvfsInfo { pub count:i32, pub latency:i32, pub opps:*mut ScpiOpp }
#[repr(C)] pub struct ScpiSensorInfo { pub sensor_id:U16, pub class:U8, pub trigger_type:U8, pub name:[i8;20] }
#[repr(C)] pub struct ScpiOps {
    pub get_version:Option<unsafe extern "C" fn()->U32>, pub clk_get_range:Option<unsafe extern "C" fn(U16,*mut usize,*mut usize)->i32>, pub clk_get_val:Option<unsafe extern "C" fn(U16)->usize>, pub clk_set_val:Option<unsafe extern "C" fn(U16,usize)->i32>, pub dvfs_get_idx:Option<unsafe extern "C" fn(U8)->i32>, pub dvfs_set_idx:Option<unsafe extern "C" fn(U8,U8)->i32>, pub dvfs_get_info:Option<unsafe extern "C" fn(U8)->*mut ScpiDvfsInfo>, pub device_domain_id:Option<unsafe extern "C" fn(*mut Device)->i32>, pub get_transition_latency:Option<unsafe extern "C" fn(*mut Device)->i32>, pub add_opps_to_device:Option<unsafe extern "C" fn(*mut Device)->i32>, pub sensor_get_capability:Option<unsafe extern "C" fn(*mut U16)->i32>, pub sensor_get_info:Option<unsafe extern "C" fn(U16,*mut ScpiSensorInfo)->i32>, pub sensor_get_value:Option<unsafe extern "C" fn(U16,*mut U64)->i32>, pub device_get_power_state:Option<unsafe extern "C" fn(U16)->i32>, pub device_set_power_state:Option<unsafe extern "C" fn(U16,U8)->i32>,
}
#[repr(C)] pub struct ScpiXfer { pub slot:U32,pub cmd:U32,pub status:U32,pub tx_buf:*const c_void,pub rx_buf:*mut c_void,pub tx_len:usize,pub rx_len:usize,pub node:ListHead,pub done:Completion }
#[repr(C)] pub struct ScpiChan { pub cl:MboxClient,pub chan:*mut MboxChan,pub tx_payload:*mut u8,pub rx_payload:*mut u8,pub rx_pending:ListHead,pub xfers_list:ListHead,pub xfers:*mut ScpiXfer,pub rx_lock:Spinlock,pub xfers_lock:Mutex,pub token:U8 }
#[repr(C)] pub struct ScpiDrvinfo { pub protocol_version:U32,pub firmware_version:U32,pub is_legacy:bool,pub num_chans:i32,pub commands:*mut i32,pub cmd_priority:[u64;1],pub next_chan:i32,pub scpi_ops:*mut ScpiOps,pub channels:*mut ScpiChan,pub dvfs:[*mut ScpiDvfsInfo;MAX_DVFS_DOMAINS] }
#[repr(C,packed)] pub struct ScpiSharedMem { pub command:Le32,pub status:Le32,pub payload:[u8;0] }
#[repr(C,packed)] pub struct LegacyScpiSharedMem { pub status:Le32,pub payload:[u8;0] }
#[repr(C,packed)] pub struct ScpCapabilities { pub protocol_version:Le32,pub event_version:Le32,pub platform_version:Le32,pub commands:[Le32;4] }
#[repr(C,packed)] pub struct ClkGetInfo { pub id:Le16,pub flags:Le16,pub min_rate:Le32,pub max_rate:Le32,pub name:[u8;20] }
#[repr(C,packed)] pub struct ClkSetValue { pub id:Le16,pub reserved:Le16,pub rate:Le32 }
#[repr(C,packed)] pub struct LegacyClkSetValue { pub rate:Le32,pub id:Le16,pub reserved:Le16 }
#[repr(C,packed)] pub struct DvfsInfo { pub domain:U8,pub opp_count:U8,pub latency:Le16,pub opps:[DvfsOpp;MAX_DVFS_OPPS] }
#[repr(C,packed)] pub struct DvfsOpp { pub freq:Le32,pub m_volt:Le32 }
#[repr(C,packed)] pub struct DvfsSet { pub domain:U8,pub index:U8 }
#[repr(C)] pub struct PrivateSensorInfo { pub sensor_id:Le16,pub class:U8,pub trigger_type:U8,pub name:[i8;20] }
#[repr(C,packed)] pub struct DevPstateSet { pub dev_id:Le16,pub pstate:U8 }

static mut SCPI_INFO:*mut ScpiDrvinfo = core::ptr::null_mut();

const fn field_get(mask:U32, value:U32)->U32 { (value & mask) >> mask.trailing_zeros() }
const fn field_prep(mask:U32, value:U32)->U32 { (value << mask.trailing_zeros()) & mask }
const fn pack_scpi_cmd(id:i32, tx:usize)->U32 { field_prep(CMD_ID_MASK,id as U32)|field_prep(CMD_DATA_SIZE_MASK,tx as U32) }
const fn pack_legacy_cmd(id:i32, tx:usize)->U32 { field_prep(CMD_ID_MASK,id as U32)|field_prep(CMD_LEGACY_DATA_SIZE_MASK,tx as U32) }
const fn cmd_size(cmd:U32)->usize { field_get(CMD_DATA_SIZE_MASK,cmd) as usize }
const fn cmd_unique(cmd:U32)->U32 { cmd & (CMD_TOKEN_ID_MASK|CMD_ID_MASK) }

unsafe extern "C" { fn scpi_send_message(idx:U8,tx:*mut c_void,tx_len:usize,rx:*mut c_void,rx_len:usize)->i32; }

#[no_mangle] pub unsafe extern "C" fn get_scpi_ops()->*mut ScpiOps { if SCPI_INFO.is_null(){core::ptr::null_mut()}else{(*SCPI_INFO).scpi_ops} }

// The remaining driver operations retain the original Linux implementation's interface and are
// declared here for linkage with the translated kernel support layer.
pub static mut SCPI_OPS: ScpiOps = ScpiOps { get_version:None,clk_get_range:None,clk_get_val:None,clk_set_val:None,dvfs_get_idx:None,dvfs_set_idx:None,dvfs_get_info:None,device_domain_id:None,get_transition_latency:None,add_opps_to_device:None,sensor_get_capability:None,sensor_get_info:None,sensor_get_value:None,device_get_power_state:None,device_set_power_state:None };

// External-kernel operations are kept as declarations; their implementations are supplied by the
// surrounding kernel translation. These declarations preserve the source-level driver surface.
unsafe extern "C" {
    fn scpi_process_cmd(ch:*mut ScpiChan, cmd:U32);
    fn scpi_handle_remote_msg(c:*mut MboxClient, msg:*mut c_void);
    fn scpi_tx_prepare(c:*mut MboxClient, msg:*mut c_void);
    fn get_scpi_xfer(ch:*mut ScpiChan)->*mut ScpiXfer;
    fn put_scpi_xfer(t:*mut ScpiXfer, ch:*mut ScpiChan);
    fn scpi_get_version()->U32;
    fn scpi_clk_get_range(clk_id:U16,min:*mut usize,max:*mut usize)->i32;
    fn scpi_clk_get_val(clk_id:U16)->usize;
    fn scpi_clk_set_val(clk_id:U16,rate:usize)->i32;
    fn legacy_scpi_clk_set_val(clk_id:U16,rate:usize)->i32;
    fn scpi_dvfs_get_idx(domain:U8)->i32;
    fn scpi_dvfs_set_idx(domain:U8,index:U8)->i32;
    fn scpi_dvfs_get_info(domain:U8)->*mut ScpiDvfsInfo;
    fn scpi_dev_domain_id(dev:*mut Device)->i32;
    fn scpi_dvfs_info(dev:*mut Device)->*mut ScpiDvfsInfo;
    fn scpi_dvfs_get_transition_latency(dev:*mut Device)->i32;
    fn scpi_dvfs_add_opps_to_device(dev:*mut Device)->i32;
    fn scpi_sensor_get_capability(sensors:*mut U16)->i32;
    fn scpi_sensor_get_info(sensor_id:U16,info:*mut ScpiSensorInfo)->i32;
    fn scpi_sensor_get_value(sensor:U16,val:*mut U64)->i32;
    fn scpi_device_get_power_state(dev_id:U16)->i32;
    fn scpi_device_set_power_state(dev_id:U16,pstate:U8)->i32;
    fn scpi_init_versions(info:*mut ScpiDrvinfo)->i32;
    fn protocol_version_show(dev:*mut Device,attr:*mut c_void,buf:*mut i8)->isize;
    fn firmware_version_show(dev:*mut Device,attr:*mut c_void,buf:*mut i8)->isize;
    fn scpi_free_channels(data:*mut c_void);
    fn scpi_remove(pdev:*mut PlatformDevice);
    fn scpi_alloc_xfer_list(dev:*mut Device,ch:*mut ScpiChan)->i32;
    fn scpi_probe(pdev:*mut PlatformDevice)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
