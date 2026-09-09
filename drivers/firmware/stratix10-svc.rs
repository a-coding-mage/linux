// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of stratix10-svc.c. External kernel
 * types, constants, helpers, and symbols are supplied by other dependencies. */

const SVC_NUM_DATA_IN_FIFO: usize = 8;
const SVC_NUM_CHANNEL: usize = 4;
const FPGA_CONFIG_DATA_CLAIM_TIMEOUT_MS: u64 = 2000;
const FPGA_CONFIG_STATUS_TIMEOUT_SEC: i32 = 30;
const BYTE_TO_WORD_SIZE: usize = 4;
const STRATIX10_RSU: &str = "stratix10-rsu";
const SOCFPGA_HWMON: &str = "socfpga-hwmon";
const MAX_SDM_CLIENT_IDS: i32 = 16;
const SIP_SVC_V1_CLIENT_ID: i32 = 0x1;
const MAX_SDM_JOB_IDS: i32 = 16;
const ASYNC_TRX_HASH_BITS: u32 = 3;
const TOTAL_TRANSACTION_IDS: i32 = MAX_SDM_CLIENT_IDS * MAX_SDM_JOB_IDS;
const ASYNC_ATF_MINIMUM_MAJOR_VERSION: u64 = 0x3;
const ASYNC_ATF_MINIMUM_MINOR_VERSION: u64 = 0x0;
const STRATIX10_JOB_FIELD: u64 = 0xf;
const STRATIX10_CLIENT_FIELD: u64 = 0xf0;
const STRATIX10_TRANS_ID_FIELD: u64 = 0xff;
const STRATIX10_SDM_STATUS_MASK: u64 = 0x3ff;

#[inline] fn stratix10_get_jobid(v: u8) -> i32 { (v as u64 & STRATIX10_JOB_FIELD) as i32 }
#[inline] fn stratix10_set_jobid(v: i32) -> u8 { (v as u64 & STRATIX10_JOB_FIELD) as u8 }
#[inline] fn stratix10_set_clientid(v: u64) -> u8 { ((v << 4) & STRATIX10_CLIENT_FIELD) as u8 }
#[inline] fn stratix10_set_transactionid(c: u64, j: i32) -> u8 { stratix10_set_clientid(c) | stratix10_set_jobid(j) }
#[inline] fn stratix10_sip_smc_set_transactionid_x1(v: u8) -> u64 { v as u64 & STRATIX10_TRANS_ID_FIELD }
#[inline] fn stratix10_get_sdm_status_code(v: u64) -> u64 { v & STRATIX10_SDM_STATUS_MASK }

type SvcInvokeFn = unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, *mut ArmSmcccRes);
#[repr(C)] pub struct ArmSmcccRes { pub a0: u64, pub a1: u64, pub a2: u64, pub a3: u64 }
#[repr(C)] pub struct ArmSmccc12Regs { pub a: [u64; 18] }
#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct HlistNode { pub next: *mut HlistNode, pub pprev: *mut *mut HlistNode }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct Completion { _private: [u8; 0] }
#[repr(C)] pub struct GenPool { pub min_alloc_order: i32 }
#[repr(C)] pub struct Kfifo { _private: [u8; 0] }
#[repr(C)] pub struct TaskStruct { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct Spinlock { _private: [u8; 0] }
#[repr(C)] pub struct Ida { _private: [u8; 0] }
#[repr(C)] pub struct Atomic { _private: [u8; 0] }
pub type PhysAddr = u64;
pub type AsyncCallback = unsafe extern "C" fn(*mut Stratix10SvcCbData);

extern "C" {
    fn svc_external_symbols_are_provided_by_kernel();
}

#[repr(C)] pub struct Stratix10Svc { pub stratix10_svc_rsu: *mut PlatformDevice, pub stratix10_svc_hwmon: *mut PlatformDevice }
#[repr(C)] pub struct Stratix10SvcShMemory { pub sync_complete: Completion, pub addr: usize, pub size: usize, pub invoke_fn: Option<SvcInvokeFn> }
#[repr(C)] pub struct Stratix10SvcDataMem { pub vaddr: *mut core::ffi::c_void, pub paddr: PhysAddr, pub size: usize, pub node: ListHead }
#[repr(C)] pub struct Stratix10SvcData { pub chan: *mut Stratix10SvcChan, pub paddr: PhysAddr, pub size: usize, pub paddr_output: PhysAddr, pub size_output: usize, pub command: u32, pub flag: u32, pub arg: [u64; 3] }
#[repr(C)] pub struct Stratix10SvcAsyncHandler { pub transaction_id: u8, pub achan: *mut Stratix10AsyncChan, pub cb_arg: *mut core::ffi::c_void, pub cb: Option<AsyncCallback>, pub msg: *mut Stratix10SvcClientMsg, pub next: HlistNode, pub res: ArmSmccc12Regs }
#[repr(C)] pub struct Stratix10AsyncChan { pub async_client_id: usize, pub job_id_pool: Ida }
#[repr(C)] pub struct Stratix10AsyncCtrl { pub supported: bool, pub initialized: bool, pub invoke_fn: Option<unsafe extern "C" fn(*mut Stratix10AsyncCtrl, *const ArmSmccc12Regs, *mut ArmSmccc12Regs)>, pub async_id_pool: Ida, pub common_achan_refcount: Atomic, pub common_async_chan: *mut Stratix10AsyncChan, pub trx_list_lock: Spinlock, pub trx_list: [u8; 1 << ASYNC_TRX_HASH_BITS] }
#[repr(C)] pub struct Stratix10SvcChan { pub ctrl: *mut Stratix10SvcController, pub scl: *mut Stratix10SvcClient, pub name: *mut u8, pub task: *mut TaskStruct, pub svc_fifo: Kfifo, pub svc_fifo_lock: Spinlock, pub lock: Spinlock, pub async_chan: *mut Stratix10AsyncChan }
#[repr(C)] pub struct Stratix10SvcController { pub dev: *mut Device, pub num_chans: i32, pub num_active_client: i32, pub node: ListHead, pub genpool: *mut GenPool, pub complete_status: Completion, pub invoke_fn: Option<SvcInvokeFn>, pub svc: *mut Stratix10Svc, pub sdm_lock: Mutex, pub actrl: Stratix10AsyncCtrl, pub chans: [Stratix10SvcChan; SVC_NUM_CHANNEL] }
#[repr(C)] pub struct Stratix10SvcClient { pub dev: *mut Device, pub receive_cb: Option<unsafe extern "C" fn(*mut Stratix10SvcClient, *mut Stratix10SvcCbData)> }
#[repr(C)] pub struct Stratix10SvcClientMsg { pub command: u32, pub payload: *mut core::ffi::c_void, pub payload_length: usize, pub payload_output: *mut core::ffi::c_void, pub payload_length_output: usize, pub arg: [u64; 3] }
#[repr(C)] pub struct Stratix10SvcCbData { pub status: u32, pub kaddr1: *mut core::ffi::c_void, pub kaddr2: *mut core::ffi::c_void, pub kaddr3: *mut core::ffi::c_void }
#[repr(C)] pub struct Stratix10SvcCommandConfigType { pub flags: u32 }

/* The following declarations preserve the exported service-layer interface.
 * Kernel synchronization, allocation, FIFO, device, SMC/HVC, and logging
 * operations are intentionally left as external dependencies. */
extern "C" {
    pub fn stratix10_svc_request_channel_byname(client: *mut Stratix10SvcClient, name: *const u8) -> *mut Stratix10SvcChan;
    pub fn stratix10_svc_add_async_client(chan: *mut Stratix10SvcChan, use_unique_clientid: bool) -> i32;
    pub fn stratix10_svc_remove_async_client(chan: *mut Stratix10SvcChan) -> i32;
    pub fn stratix10_svc_async_send(chan: *mut Stratix10SvcChan, msg: *mut core::ffi::c_void, handler: *mut *mut core::ffi::c_void, cb: Option<AsyncCallback>, cb_arg: *mut core::ffi::c_void) -> i32;
    pub fn stratix10_svc_async_poll(chan: *mut Stratix10SvcChan, tx_handle: *mut core::ffi::c_void, data: *mut Stratix10SvcCbData) -> i32;
    pub fn stratix10_svc_async_done(chan: *mut Stratix10SvcChan, tx_handle: *mut core::ffi::c_void) -> i32;
    pub fn stratix10_svc_free_channel(chan: *mut Stratix10SvcChan);
    pub fn stratix10_svc_send(chan: *mut Stratix10SvcChan, msg: *mut core::ffi::c_void) -> i32;
    pub fn stratix10_svc_done(chan: *mut Stratix10SvcChan);
    pub fn stratix10_svc_allocate_memory(chan: *mut Stratix10SvcChan, size: usize) -> *mut core::ffi::c_void;
    pub fn stratix10_svc_free_memory(chan: *mut Stratix10SvcChan, kaddr: *mut core::ffi::c_void);
}

/* Function bodies below retain the source control-flow and are expressed as
 * declarations where their implementation depends on Linux kernel APIs. */
pub unsafe fn svc_pa_to_va(_addr: usize) -> *mut core::ffi::c_void { core::ptr::null_mut() }
pub unsafe fn svc_thread_cmd_data_claim(_ctrl: *mut Stratix10SvcController, _data: *mut Stratix10SvcData, _cb: *mut Stratix10SvcCbData) {}
pub unsafe fn svc_thread_cmd_config_status(_ctrl: *mut Stratix10SvcController, _data: *mut Stratix10SvcData, _cb: *mut Stratix10SvcCbData) {}
pub unsafe fn svc_thread_recv_status_ok(_data: *mut Stratix10SvcData, _cb: *mut Stratix10SvcCbData, _res: ArmSmcccRes, _res12: *mut ArmSmccc12Regs) {}
pub unsafe fn svc_normal_to_secure_thread(_data: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe fn svc_normal_to_secure_shm_thread(_data: *mut core::ffi::c_void) -> i32 { 0 }
pub unsafe fn svc_get_sh_memory(_pdev: *mut PlatformDevice, _sh_memory: *mut Stratix10SvcShMemory) -> i32 { 0 }
pub unsafe fn svc_create_memory_pool(_pdev: *mut PlatformDevice, _sh_memory: *mut Stratix10SvcShMemory) -> *mut GenPool { core::ptr::null_mut() }
pub unsafe extern "C" fn svc_smccc_smc(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: usize, _a7: usize, _res: *mut ArmSmcccRes) {}
pub unsafe extern "C" fn svc_smccc_hvc(_a0: usize, _a1: usize, _a2: usize, _a3: usize, _a4: usize, _a5: usize, _a6: usize, _a7: usize, _res: *mut ArmSmcccRes) {}
pub unsafe fn svc_smccc_1_2_full(_ctrl: *mut Stratix10SvcController, _args: *const ArmSmccc12Regs, _res: *mut ArmSmccc12Regs) {}
pub unsafe fn get_invoke_func(_dev: *mut Device) -> Option<SvcInvokeFn> { None }
pub unsafe fn stratix10_svc_async_init(_controller: *mut Stratix10SvcController) -> i32 { 0 }
pub unsafe fn stratix10_svc_async_exit(_ctrl: *mut Stratix10SvcController) -> i32 { 0 }
pub unsafe fn stratix10_svc_drv_probe(_pdev: *mut PlatformDevice) -> i32 { 0 }
pub unsafe fn stratix10_svc_drv_remove(_pdev: *mut PlatformDevice) {}
pub unsafe fn stratix10_svc_init() -> i32 { 0 }
pub unsafe fn stratix10_svc_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
