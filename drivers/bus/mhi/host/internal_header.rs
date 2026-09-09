/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from internal.h. Definitions supplied by common.h remain external. */

use core::ffi::c_void;

pub const MHI_SOC_RESET_REQ_OFFSET: u32 = 0xb0;
pub const MHI_SOC_RESET_REQ: u32 = 1 << 0;

pub unsafe extern "C" {
    pub static mhi_bus_type: bus_type;
}

#[repr(C)]
pub struct mhi_ctxt { pub er_ctxt: *mut mhi_event_ctxt, pub chan_ctxt: *mut mhi_chan_ctxt, pub cmd_ctxt: *mut mhi_cmd_ctxt, pub er_ctxt_addr: dma_addr_t, pub chan_ctxt_addr: dma_addr_t, pub cmd_ctxt_addr: dma_addr_t }
#[repr(C)] pub struct bhi_vec_entry { pub dma_addr: __le64, pub size: __le64 }

#[repr(C)] #[derive(Copy, Clone)] pub enum mhi_fw_load_type { MHI_FW_LOAD_BHI, MHI_FW_LOAD_BHIE, MHI_FW_LOAD_FBC, MHI_FW_LOAD_MAX }
#[repr(C)] #[derive(Copy, Clone)] pub enum mhi_ch_state_type { MHI_CH_STATE_TYPE_RESET, MHI_CH_STATE_TYPE_STOP, MHI_CH_STATE_TYPE_START, MHI_CH_STATE_TYPE_MAX }
pub unsafe extern "C" { pub static mhi_ch_state_type_str: [*const core::ffi::c_char; 3]; }
pub const MHI_CH_STATE_TYPE_LIST: &str = "RESET STOP START";
pub unsafe fn to_ch_state_type_str(state: usize) -> *const core::ffi::c_char { if state >= mhi_ch_state_type::MHI_CH_STATE_TYPE_MAX as usize { c"INVALID_STATE".as_ptr() } else { mhi_ch_state_type_str[state] } }

pub const MHI_INVALID_BRSTMODE: fn(mhi_db_brst_mode) -> bool = |mode| mode != MHI_DB_BRST_DISABLE && mode != MHI_DB_BRST_ENABLE;
pub const MHI_EE_LIST: &str = "PBL SBL AMSS RDDM WFW PTHRU EDL FP DISABLE_TRANSITION NOT_SUPPORTED";
pub unsafe extern "C" { pub static mhi_ee_str: [*const core::ffi::c_char; MHI_EE_MAX as usize]; }
pub unsafe fn to_mhi_exec_str(ee: usize) -> *const core::ffi::c_char { if ee >= MHI_EE_MAX as usize { c"INVALID_EE".as_ptr() } else { mhi_ee_str[ee] } }
pub const fn mhi_in_pbl(ee: mhi_ee_type) -> bool { matches!(ee, MHI_EE_PBL | MHI_EE_PTHRU | MHI_EE_EDL) }
pub const fn mhi_power_up_capable(ee: mhi_ee_type) -> bool { mhi_in_pbl(ee) || matches!(ee, MHI_EE_AMSS) }
pub const fn mhi_fw_load_capable(ee: mhi_ee_type) -> bool { matches!(ee, MHI_EE_PBL | MHI_EE_EDL) }
pub const fn mhi_in_mission_mode(ee: mhi_ee_type) -> bool { matches!(ee, MHI_EE_AMSS | MHI_EE_WFW | MHI_EE_FP) }

#[repr(C)] #[derive(Copy, Clone)] pub enum dev_st_transition { DEV_ST_TRANSITION_PBL, DEV_ST_TRANSITION_READY, DEV_ST_TRANSITION_SBL, DEV_ST_TRANSITION_MISSION_MODE, DEV_ST_TRANSITION_FP, DEV_ST_TRANSITION_SYS_ERR, DEV_ST_TRANSITION_DISABLE, DEV_ST_TRANSITION_DISABLE_DESTROY_DEVICE, DEV_ST_TRANSITION_MAX }
pub const DEV_ST_TRANSITION_LIST: &str = "PBL READY SBL MISSION_MODE FP SYS_ERR DISABLE DISABLE_DESTROY_DEVICE";
pub unsafe extern "C" { pub static dev_state_tran_str: [*const core::ffi::c_char; DEV_ST_TRANSITION_MAX as usize]; }
pub unsafe fn to_dev_state_trans_str(state: usize) -> *const core::ffi::c_char { if state >= DEV_ST_TRANSITION_MAX as usize { c"INVALID_STATE".as_ptr() } else { dev_state_tran_str[state] } }

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)] pub enum mhi_pm_state { MHI_PM_STATE_DISABLE, MHI_PM_STATE_POR, MHI_PM_STATE_M0, MHI_PM_STATE_M2, MHI_PM_STATE_M3_ENTER, MHI_PM_STATE_M3, MHI_PM_STATE_M3_EXIT, MHI_PM_STATE_FW_DL_ERR, MHI_PM_STATE_SYS_ERR_DETECT, MHI_PM_STATE_SYS_ERR_PROCESS, MHI_PM_STATE_SYS_ERR_FAIL, MHI_PM_STATE_SHUTDOWN_PROCESS, MHI_PM_STATE_LD_ERR_FATAL_DETECT, MHI_PM_STATE_MAX }
pub const MHI_PM_DISABLE: u32 = 1 << 0; pub const MHI_PM_POR: u32 = 1 << 1; pub const MHI_PM_M0: u32 = 1 << 2; pub const MHI_PM_M2: u32 = 1 << 3; pub const MHI_PM_M3_ENTER: u32 = 1 << 4; pub const MHI_PM_M3: u32 = 1 << 5; pub const MHI_PM_M3_EXIT: u32 = 1 << 6; pub const MHI_PM_FW_DL_ERR: u32 = 1 << 7; pub const MHI_PM_SYS_ERR_DETECT: u32 = 1 << 8; pub const MHI_PM_SYS_ERR_PROCESS: u32 = 1 << 9; pub const MHI_PM_SYS_ERR_FAIL: u32 = 1 << 10; pub const MHI_PM_SHUTDOWN_PROCESS: u32 = 1 << 11; pub const MHI_PM_LD_ERR_FATAL_DETECT: u32 = 1 << 12;
pub const NR_OF_CMD_RINGS: usize = 1; pub const CMD_EL_PER_RING: usize = 128; pub const PRIMARY_CMD_RING: usize = 0; pub const MHI_DEV_WAKE_DB: usize = 127; pub const MHI_MAX_MTU: usize = 0xffff;
pub const MHI_TIMEOUT_MS: u32 = 1000;

#[repr(C)] #[derive(Copy, Clone)] pub enum mhi_er_type { MHI_ER_TYPE_INVALID = 0, MHI_ER_TYPE_VALID = 1 }
#[repr(C)] pub struct db_cfg { pub reset_req: bool, pub db_mode: bool, pub pollcfg: u32, pub brstmode: mhi_db_brst_mode, pub db_val: dma_addr_t, pub process_db: Option<unsafe extern "C" fn(*mut mhi_controller, *mut db_cfg, *mut c_void, dma_addr_t)> }
#[repr(C)] pub struct mhi_pm_transitions { pub from_state: mhi_pm_state, pub to_states: u32 }
#[repr(C)] pub struct state_transition { pub node: list_head, pub state: dev_st_transition }
#[repr(C)] pub struct mhi_ring { pub dma_handle: dma_addr_t, pub iommu_base: dma_addr_t, pub ctxt_wp: *mut __le64, pub pre_aligned: *mut c_void, pub base: *mut c_void, pub rp: *mut c_void, pub wp: *mut c_void, pub el_size: usize, pub len: usize, pub elements: usize, pub alloc_size: usize, pub db_addr: *mut c_void }
#[repr(C)] pub struct mhi_cmd { pub ring: mhi_ring, pub lock: spinlock_t }
#[repr(C)] pub struct mhi_buf_info { pub v_addr: *mut c_void, pub bb_addr: *mut c_void, pub wp: *mut c_void, pub cb_buf: *mut c_void, pub p_addr: dma_addr_t, pub len: usize, pub dir: dma_data_direction, pub used: bool, pub pre_mapped: bool }
#[repr(C)] pub struct mhi_event { pub mhi_cntrl: *mut mhi_controller, pub mhi_chan: *mut mhi_chan, pub er_index: u32, pub intmod: u32, pub irq: u32, pub chan: i32, pub priority: u32, pub data_type: mhi_er_data_type, pub ring: mhi_ring, pub db_cfg: db_cfg, pub task: tasklet_struct, pub lock: spinlock_t, pub process_event: Option<unsafe extern "C" fn(*mut mhi_controller, *mut mhi_event, u32) -> i32>, pub hw_ring: bool, pub cl_manage: bool, pub offload_ev: bool }
#[repr(C)] pub struct mhi_chan { pub name: *const core::ffi::c_char, pub buf_ring: mhi_ring, pub tre_ring: mhi_ring, pub chan: u32, pub er_index: u32, pub intmod: u32, pub type_: mhi_ch_type, pub dir: dma_data_direction, pub db_cfg: db_cfg, pub ee_mask: mhi_ch_ee_mask, pub ch_state: mhi_ch_state, pub ccs: mhi_ev_ccs, pub mhi_dev: *mut mhi_device, pub xfer_cb: Option<unsafe extern "C" fn(*mut mhi_device, *mut mhi_result)>, pub mutex: mutex, pub completion: completion, pub lock: rwlock_t, pub node: list_head, pub lpm_notify: bool, pub configured: bool, pub offload_ch: bool, pub wake_capable: bool }

extern "C" { pub fn mhi_alloc_device(c: *mut mhi_controller) -> *mut mhi_device; pub fn mhi_destroy_device(dev: *mut device, data: *mut c_void) -> i32; pub fn mhi_create_devices(c: *mut mhi_controller); }
extern "C" { pub fn mhi_alloc_bhie_table(c: *mut mhi_controller, i: *mut *mut image_info, s: usize) -> i32; pub fn mhi_free_bhie_table(c: *mut mhi_controller, i: *mut image_info); }
extern "C" { pub fn mhi_tryset_pm_state(c: *mut mhi_controller, s: mhi_pm_state) -> mhi_pm_state; pub fn to_mhi_pm_state_str(s: u32) -> *const core::ffi::c_char; pub fn mhi_queue_state_transition(c: *mut mhi_controller, s: dev_st_transition) -> i32; pub fn mhi_pm_st_worker(w: *mut work_struct); pub fn mhi_pm_sys_err_handler(c: *mut mhi_controller); pub fn mhi_ready_state_transition(c: *mut mhi_controller) -> i32; pub fn mhi_pm_m0_transition(c: *mut mhi_controller) -> i32; pub fn mhi_pm_m1_transition(c: *mut mhi_controller); pub fn mhi_pm_m3_transition(c: *mut mhi_controller) -> i32; pub fn __mhi_device_get_sync(c: *mut mhi_controller) -> i32; pub fn mhi_send_cmd(c: *mut mhi_controller, ch: *mut mhi_chan, cmd: mhi_cmd_type) -> i32; pub fn mhi_download_amss_image(c: *mut mhi_controller) -> i32; }
pub unsafe fn mhi_is_active(c: *mut mhi_controller) -> bool { (*c).dev_state >= MHI_STATE_M0 && (*c).dev_state <= MHI_STATE_M3_FAST }
extern "C" { pub fn mhi_db_brstmode(c:*mut mhi_controller,d:*mut db_cfg,a:*mut c_void,v:dma_addr_t); pub fn mhi_db_brstmode_disable(c:*mut mhi_controller,d:*mut db_cfg,a:*mut c_void,v:dma_addr_t); pub fn mhi_read_reg(c:*mut mhi_controller,b:*mut c_void,o:u32,out:*mut u32)->i32; pub fn mhi_read_reg_field(c:*mut mhi_controller,b:*mut c_void,o:u32,m:u32,out:*mut u32)->i32; pub fn mhi_poll_reg_field(c:*mut mhi_controller,b:*mut c_void,o:u32,m:u32,v:u32,d:u32,t:u32)->i32; pub fn mhi_write_reg(c:*mut mhi_controller,b:*mut c_void,o:u32,v:u32); pub fn mhi_write_reg_field(c:*mut mhi_controller,b:*mut c_void,o:u32,m:u32,v:u32)->i32; pub fn mhi_ring_er_db(e:*mut mhi_event); pub fn mhi_write_db(c:*mut mhi_controller,a:*mut c_void,v:dma_addr_t); pub fn mhi_ring_cmd_db(c:*mut mhi_controller,m:*mut mhi_cmd); pub fn mhi_ring_chan_db(c:*mut mhi_controller,ch:*mut mhi_chan); }
extern "C" { pub fn mhi_init_mmio(c:*mut mhi_controller)->i32; pub fn mhi_rddm_prepare(c:*mut mhi_controller,i:*mut image_info)->i32; pub fn mhi_fw_load_handler(c:*mut mhi_controller); pub fn mhi_init_chan_ctxt(c:*mut mhi_controller,ch:*mut mhi_chan)->i32; pub fn mhi_deinit_chan_ctxt(c:*mut mhi_controller,ch:*mut mhi_chan); pub fn mhi_reset_chan(c:*mut mhi_controller,ch:*mut mhi_chan); pub fn mhi_ctrl_ev_task(d:usize); pub fn mhi_ev_task(d:usize); pub fn mhi_process_data_event_ring(c:*mut mhi_controller,e:*mut mhi_event,q:u32)->i32; pub fn mhi_process_ctrl_ev_ring(c:*mut mhi_controller,e:*mut mhi_event,q:u32)->i32; pub fn mhi_uevent_notify(c:*mut mhi_controller,e:mhi_ee_type); pub fn mhi_irq_handler(n:i32,d:*mut c_void)->irqreturn_t; pub fn mhi_intvec_threaded_handler(n:i32,d:*mut c_void)->irqreturn_t; pub fn mhi_intvec_handler(n:i32,d:*mut c_void)->irqreturn_t; pub fn mhi_gen_tre(c:*mut mhi_controller,ch:*mut mhi_chan,i:*mut mhi_buf_info,f:mhi_flags)->i32; pub fn mhi_map_single_no_bb(c:*mut mhi_controller,i:*mut mhi_buf_info)->i32; pub fn mhi_map_single_use_bb(c:*mut mhi_controller,i:*mut mhi_buf_info)->i32; pub fn mhi_unmap_single_no_bb(c:*mut mhi_controller,i:*mut mhi_buf_info); pub fn mhi_unmap_single_use_bb(c:*mut mhi_controller,i:*mut mhi_buf_info); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
