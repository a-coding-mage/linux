/* Rust translation of amdgpu.h. Included kernel dependencies are external. */

// C headers and conditional compilation dependencies are supplied by the surrounding translation.

pub const MAX_GPU_INSTANCE: usize = 64;
pub const GFX_SLICE_PERIOD_MS: u32 = 250;
pub const AMDGPU_MAX_TIMEOUT_PARAM_LENGTH: usize = 256;
pub const AMDGPU_MAX_PPLL: usize = 3;
pub const AMDGPU_BIOS_NUM_SCRATCH: usize = 16;
pub const AMDGPU_RESET_MAGIC_NUM: usize = 64;
pub const AMDGPU_MAX_DF_PERFMONS: usize = 4;

#[repr(C)] pub struct amdgpu_gpu_instance { pub adev: *mut amdgpu_device, pub mgpu_fan_enabled: c_int }
#[repr(C)] pub struct amdgpu_mgpu_info { pub gpu_ins: [amdgpu_gpu_instance; MAX_GPU_INSTANCE], pub mutex: mutex, pub num_gpu: u32, pub num_dgpu: u32, pub num_apu: u32 }
#[repr(C)] pub struct amdgpu_hwip_reg_entry { pub hwip: u32, pub inst: u32, pub seg: u32, pub reg_offset: u32, pub reg_name: *const c_char }
#[repr(C)] pub struct amdgpu_watchdog_timer { pub timeout_fatal_disable: bool, pub period: u32 }

extern "C" {
    pub static mut amdgpu_modeset: c_int; pub static mut amdgpu_vram_limit: c_uint; pub static mut amdgpu_vis_vram_limit: c_int;
    pub static mut amdgpu_gart_size: c_int; pub static mut amdgpu_gtt_size: c_int; pub static mut amdgpu_moverate: c_int;
    pub static mut amdgpu_audio: c_int; pub static mut amdgpu_disp_priority: c_int; pub static mut amdgpu_hw_i2c: c_int;
    pub static mut amdgpu_pcie_gen2: c_int; pub static mut amdgpu_msi: c_int;
    pub static mut amdgpu_lockup_timeout: [c_char; AMDGPU_MAX_TIMEOUT_PARAM_LENGTH]; pub static mut amdgpu_dpm: c_int;
    pub static mut amdgpu_fw_load_type: c_int; pub static mut amdgpu_aspm: c_int; pub static mut amdgpu_runtime_pm: c_int;
    pub static mut amdgpu_ip_block_mask: c_uint; pub static mut amdgpu_bapm: c_int; pub static mut amdgpu_deep_color: c_int;
    pub static mut amdgpu_vm_size: c_int; pub static mut amdgpu_vm_block_size: c_int; pub static mut amdgpu_vm_fragment_size: c_int;
    pub static mut amdgpu_vm_fault_stop: c_int; pub static mut amdgpu_vm_debug: c_int; pub static mut amdgpu_vm_update_mode: c_int;
    pub static mut amdgpu_exp_hw_support: c_int; pub static mut amdgpu_dc: c_int; pub static mut amdgpu_sched_jobs: c_int;
    pub static mut amdgpu_sched_hw_submission: c_int; pub static mut amdgpu_pcie_gen_cap: c_uint; pub static mut amdgpu_pcie_lane_cap: c_uint;
    pub static mut amdgpu_cg_mask: u64; pub static mut amdgpu_pg_mask: c_uint; pub static mut amdgpu_sdma_phase_quantum: c_uint;
    pub static mut amdgpu_disable_cu: *mut c_char; pub static mut amdgpu_virtual_display: *mut c_char; pub static mut amdgpu_pp_feature_mask: c_uint;
    pub static mut amdgpu_force_long_training: c_uint; pub static mut amdgpu_lbpw: c_int; pub static mut amdgpu_compute_multipipe: c_int;
    pub static mut amdgpu_gpu_recovery: c_int; pub static mut amdgpu_emu_mode: c_int; pub static mut amdgpu_smu_memory_pool_size: c_uint;
    pub static mut amdgpu_smu_pptable_id: c_int; pub static mut amdgpu_dc_feature_mask: c_uint; pub static mut amdgpu_freesync_vid_mode: c_uint;
    pub static mut amdgpu_dc_debug_mask: c_uint; pub static mut amdgpu_dc_visual_confirm: c_uint; pub static mut amdgpu_dm_abm_level: c_int;
    pub static mut amdgpu_backlight: c_int; pub static mut amdgpu_damage_clips: c_int; pub static mut mgpu_info: amdgpu_mgpu_info;
    pub static mut amdgpu_ras_enable: c_int; pub static mut amdgpu_ras_mask: c_uint; pub static mut amdgpu_bad_page_threshold: c_int;
    pub static mut amdgpu_ignore_bad_page_threshold: bool; pub static mut amdgpu_watchdog_timer: amdgpu_watchdog_timer;
    pub static mut amdgpu_async_gfx_ring: c_int; pub static mut amdgpu_mcbp: c_int; pub static mut amdgpu_discovery: c_int;
    pub static mut amdgpu_mes_log_enable: c_int; pub static mut amdgpu_uni_mes: c_int; pub static mut amdgpu_noretry: c_int;
    pub static mut amdgpu_force_asic_type: c_int; pub static mut amdgpu_smartshift_bias: c_int; pub static mut amdgpu_use_xgmi_p2p: c_int;
    pub static mut amdgpu_mtype_local: c_int; pub static mut amdgpu_enforce_isolation: c_int; pub static mut amdgpu_debug_mask: c_uint;
    pub static mut amdgpu_tmz: c_int; pub static mut amdgpu_reset_method: c_int; pub static mut amdgpu_num_kcq: c_int;
    pub static mut amdgpu_vcnfw_log: c_int; pub static mut amdgpu_sg_display: c_int; pub static mut amdgpu_umsch_mm: c_int;
    pub static mut amdgpu_seamless: c_int; pub static mut amdgpu_umsch_mm_fwlog: c_int; pub static mut amdgpu_user_partt_mode: c_int;
    pub static mut amdgpu_agp: c_int; pub static mut amdgpu_rebar: c_int; pub static mut amdgpu_wbrf: c_int;
    pub static mut amdgpu_user_queue: c_int; pub static mut amdgpu_ptl: c_int; pub static mut amdgpu_hdmi_hpd_debounce_delay_ms: c_uint;
}

pub const AMDGPU_VCNFW_LOG_SIZE: usize = 32 * 1024;
pub const AMDGPU_UMSCHFW_LOG_SIZE: usize = 32 * 1024;
pub const AMDGPU_SG_THRESHOLD: u64 = 256 * 1024 * 1024;
pub const AMDGPU_WAIT_IDLE_TIMEOUT_IN_MS: u32 = 3000;
pub const AMDGPU_MAX_USEC_TIMEOUT: u32 = 100000;
pub const AMDGPU_DEBUGFS_MAX_COMPONENTS: usize = 32;
pub const AMDGPUFB_CONN_LIMIT: usize = 4;
pub const AMDGPU_VBIOS_VGA_ALLOCATION: u64 = 9 * 1024 * 1024;
pub const AMDGPU_ASIC_RESET_DATA: u32 = 0x39d5e86b;
pub const AMDGPU_RESET_GFX: u32 = 1<<0; pub const AMDGPU_RESET_COMPUTE: u32 = 1<<1; pub const AMDGPU_RESET_DMA: u32 = 1<<2;
pub const AMDGPU_RESET_CP: u32 = 1<<3; pub const AMDGPU_RESET_GRBM: u32 = 1<<4; pub const AMDGPU_RESET_DMA1: u32 = 1<<5;
pub const AMDGPU_RESET_RLC: u32 = 1<<6; pub const AMDGPU_RESET_SEM: u32 = 1<<7; pub const AMDGPU_RESET_IH: u32 = 1<<8;
pub const AMDGPU_RESET_VMC: u32 = 1<<9; pub const AMDGPU_RESET_MC: u32 = 1<<10; pub const AMDGPU_RESET_DISPLAY: u32 = 1<<11;
pub const AMDGPU_RESET_UVD: u32 = 1<<12; pub const AMDGPU_RESET_VCE: u32 = 1<<13; pub const AMDGPU_RESET_VCE1: u32 = 1<<14;
pub const AMDGPU_RESET_TYPE_FULL: u32 = 1; pub const AMDGPU_RESET_TYPE_SOFT_RECOVERY: u32 = 2; pub const AMDGPU_RESET_TYPE_PER_QUEUE: u32 = 4;
pub const AMDGPU_RESET_TYPE_PER_PIPE: u32 = 8; pub const AMDGPU_RESET_TYPE_IP_BLOCK_SOFT_RESET: u32 = 16;
pub const CIK_CURSOR_WIDTH: u32 = 128; pub const CIK_CURSOR_HEIGHT: u32 = 128; pub const AMDGPU_SMARTSHIFT_MAX_BIAS: i32 = 100; pub const AMDGPU_SMARTSHIFT_MIN_BIAS: i32 = -100; pub const AMDGPU_SWCTF_EXTRA_DELAY: u32 = 50;

#[repr(C)] pub struct amdgpu_clock { pub ppll: [amdgpu_pll; AMDGPU_MAX_PPLL], pub spll: amdgpu_pll, pub mpll: amdgpu_pll, pub default_mclk:u32, pub default_sclk:u32, pub default_dispclk:u32, pub dp_extclk:u32, pub max_pixel_clock:u32 }
#[repr(C)] pub struct amdgpu_mem_scratch { pub robj:*mut amdgpu_bo, pub ptr:*mut u32, pub gpu_addr:u64 }
#[repr(C)] pub struct amdgpu_mmio_remap { pub reg_offset:u32, pub bus_addr:resource_size_t, pub bo:*mut amdgpu_bo }
#[repr(C)] pub struct amd_powerplay { pub pp_handle:*mut c_void, pub pp_funcs:*const amd_pm_funcs }
#[repr(C)] pub struct amdgpu_uid { pub uid:[[u64;8];5], pub adev:*mut amdgpu_device }
#[repr(C)] pub struct amdgpu_pcie_reset_ctx { pub in_link_reset:bool, pub occurs_dpc:bool, pub audio_suspended:bool, pub swus:*mut pci_dev, pub swus_pcistate:*mut pci_saved_state, pub swds_pcistate:*mut pci_saved_state }
#[repr(C)] pub struct amdgpu_init_level { pub level:amdgpu_init_lvl_id, pub hwini_ip_block_mask:u32 }
#[repr(C)] pub struct amdgpu_asic_funcs { pub read_disabled_bios:Option<unsafe extern "C" fn(*mut amdgpu_device)->bool>, pub read_bios_from_rom:Option<unsafe extern "C" fn(*mut amdgpu_device,*mut u8,u32)->bool>, pub read_register:Option<unsafe extern "C" fn(*mut amdgpu_device,u32,u32,u32,*mut u32)->c_int>, pub set_vga_state:Option<unsafe extern "C" fn(*mut amdgpu_device,bool)>, pub reset:Option<unsafe extern "C" fn(*mut amdgpu_device)->c_int>, pub reset_method:Option<unsafe extern "C" fn(*mut amdgpu_device)->amd_reset_method>, pub get_xclk:Option<unsafe extern "C" fn(*mut amdgpu_device)->u32>, pub get_config_memsize:Option<unsafe extern "C" fn(*mut amdgpu_device)->u32> }

pub type c_int = i32; pub type c_uint = u32; pub type c_char = i8; pub type resource_size_t = u64;
#[repr(C)] pub struct amdgpu_device { pub dev:*mut device, pub pdev:*mut pci_dev, pub ddev:drm_device, pub clock:amdgpu_clock, pub mem_scratch:amdgpu_mem_scratch, pub aid_mask:u32, pub ip_versions:[[u32;HWIP_MAX_INSTANCE];MAX_HWIP], pub gfx_timeout:i64, pub compute_timeout:i64, pub sdma_timeout:i64, pub video_timeout:i64, pub gmc:amdgpu_gmc, pub vram_lost_counter:atomic_t }

#[repr(C)] pub struct amdgpu_afmt_acr { pub clock:u32, pub n_32khz:c_int, pub cts_32khz:c_int, pub n_44_1khz:c_int, pub cts_44_1khz:c_int, pub n_48khz:c_int, pub cts_48khz:c_int }
#[repr(C)] pub struct amdgpu_xcp_mgr { _private:[u8;0] } #[repr(C)] pub struct amdgpu_pll{_private:[u8;0]} #[repr(C)] pub struct amd_pm_funcs{_private:[u8;0]}
#[repr(C)] pub struct device{_private:[u8;0]} #[repr(C)] pub struct pci_dev{_private:[u8;0]} #[repr(C)] pub struct pci_saved_state{_private:[u8;0]} #[repr(C)] pub struct drm_device{_private:[u8;0]} #[repr(C)] pub struct amdgpu_bo{_private:[u8;0]} #[repr(C)] pub struct amdgpu_gmc{pub tmz_enabled:bool} #[repr(C)] pub struct atomic_t{_private:[u8;0]}
pub type amd_reset_method = c_int; pub const MAX_HWIP:usize=64; pub const HWIP_MAX_INSTANCE:usize=16;

// Remaining declarations retain their C ABI and are supplied by dependent translated headers.
extern "C" { pub fn amdgpu_get_bios(adev:*mut amdgpu_device)->bool; pub fn amdgpu_read_bios(adev:*mut amdgpu_device)->bool; pub fn amdgpu_bios_release(adev:*mut amdgpu_device); pub fn amdgpu_device_init(adev:*mut amdgpu_device, flags:u32)->c_int; pub fn amdgpu_device_fini_hw(adev:*mut amdgpu_device); pub fn amdgpu_device_fini_sw(adev:*mut amdgpu_device); pub fn amdgpu_in_reset(adev:*mut amdgpu_device)->c_int; pub fn amdgpu_afmt_acr(clock:u32)->amdgpu_afmt_acr; }

#[inline] pub unsafe fn amdgpu_ip_version(adev:*const amdgpu_device, ip:u8, inst:u8)->u32 { (*adev).ip_versions[ip as usize][inst as usize] & !0xff }
#[inline] pub unsafe fn amdgpu_ip_version_full(adev:*const amdgpu_device, ip:u8, inst:u8)->u32 { (*adev).ip_versions[ip as usize][inst as usize] }
#[inline] pub unsafe fn amdgpu_is_multi_aid(adev:*mut amdgpu_device)->bool { (*adev).aid_mask != 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
