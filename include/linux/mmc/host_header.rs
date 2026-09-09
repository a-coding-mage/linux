/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/include/linux/mmc/host.h. */

#[repr(C)]
pub struct mmc_ios {
    pub clock: u32, pub vdd: u16, pub power_delay_ms: u32,
    pub bus_mode: u8, pub chip_select: u8, pub power_mode: u8, pub bus_width: u8,
    pub timing: u8, pub signal_voltage: u8, pub vqmmc2_voltage: u8,
    pub drv_type: u8, pub enhanced_strobe: bool,
}
pub const MMC_BUSMODE_OPENDRAIN: u8 = 1; pub const MMC_BUSMODE_PUSHPULL: u8 = 2;
pub const MMC_CS_DONTCARE: u8 = 0; pub const MMC_CS_HIGH: u8 = 1; pub const MMC_CS_LOW: u8 = 2;
pub const MMC_POWER_OFF: u8 = 0; pub const MMC_POWER_UP: u8 = 1; pub const MMC_POWER_ON: u8 = 2; pub const MMC_POWER_UNDEFINED: u8 = 3;
pub const MMC_BUS_WIDTH_1: u8 = 0; pub const MMC_BUS_WIDTH_4: u8 = 2; pub const MMC_BUS_WIDTH_8: u8 = 3;
pub const MMC_TIMING_LEGACY: u8 = 0; pub const MMC_TIMING_MMC_HS: u8 = 1; pub const MMC_TIMING_SD_HS: u8 = 2;
pub const MMC_TIMING_UHS_SDR12: u8 = 3; pub const MMC_TIMING_UHS_SDR25: u8 = 4; pub const MMC_TIMING_UHS_SDR50: u8 = 5;
pub const MMC_TIMING_UHS_SDR104: u8 = 6; pub const MMC_TIMING_UHS_DDR50: u8 = 7; pub const MMC_TIMING_MMC_DDR52: u8 = 8;
pub const MMC_TIMING_MMC_HS200: u8 = 9; pub const MMC_TIMING_MMC_HS400: u8 = 10; pub const MMC_TIMING_SD_EXP: u8 = 11;
pub const MMC_TIMING_SD_EXP_1_2V: u8 = 12; pub const MMC_TIMING_UHS2_SPEED_A: u8 = 13;
pub const MMC_TIMING_UHS2_SPEED_A_HD: u8 = 14; pub const MMC_TIMING_UHS2_SPEED_B: u8 = 15; pub const MMC_TIMING_UHS2_SPEED_B_HD: u8 = 16;
pub const MMC_SIGNAL_VOLTAGE_330: u8 = 0; pub const MMC_SIGNAL_VOLTAGE_180: u8 = 1; pub const MMC_SIGNAL_VOLTAGE_120: u8 = 2;
pub const MMC_VQMMC2_VOLTAGE_180: u8 = 0; pub const MMC_SET_DRIVER_TYPE_B: u8 = 0; pub const MMC_SET_DRIVER_TYPE_A: u8 = 1;
pub const MMC_SET_DRIVER_TYPE_C: u8 = 2; pub const MMC_SET_DRIVER_TYPE_D: u8 = 3;

#[repr(C)] pub struct mmc_clk_phase { pub valid: bool, pub in_deg: u16, pub out_deg: u16 }
pub const MMC_NUM_CLK_PHASES: usize = (MMC_TIMING_MMC_HS400 as usize) + 1;
#[repr(C)] pub struct mmc_clk_phase_map { pub phase: [mmc_clk_phase; MMC_NUM_CLK_PHASES] }
#[repr(C)] pub struct sd_uhs2_caps { pub dap:u32,pub gap:u32,pub group_desc:u32,pub maxblk_len:u32,pub n_fcu:u32,pub n_lanes:u8,pub addr64:u8,pub card_type:u8,pub phy_rev:u8,pub speed_range:u8,pub n_lss_sync:u8,pub n_lss_dir:u8,pub link_rev:u8,pub host_type:u8,pub n_data_gap:u8,pub maxblk_len_set:u32,pub n_fcu_set:u32,pub n_lanes_set:u8,pub n_lss_sync_set:u8,pub n_lss_dir_set:u8,pub n_data_gap_set:u8,pub max_retry_set:u8 }

#[repr(C)] pub enum sd_uhs2_operation { UHS2_PHY_INIT=0, UHS2_SET_CONFIG, UHS2_ENABLE_INT, UHS2_DISABLE_INT, UHS2_ENABLE_CLK, UHS2_DISABLE_CLK, UHS2_CHECK_DORMANT, UHS2_SET_IOS }
pub enum mmc_host {}
#[repr(C)] pub enum mmc_err_stat { MMC_ERR_CMD_TIMEOUT, MMC_ERR_CMD_CRC, MMC_ERR_DAT_TIMEOUT, MMC_ERR_DAT_CRC, MMC_ERR_AUTO_CMD, MMC_ERR_ADMA, MMC_ERR_TUNING, MMC_ERR_CMDQ_RED, MMC_ERR_CMDQ_GCE, MMC_ERR_CMDQ_ICCE, MMC_ERR_REQ_TIMEOUT, MMC_ERR_CMDQ_REQ_TIMEOUT, MMC_ERR_ICE_CFG, MMC_ERR_CTRL_TIMEOUT, MMC_ERR_UNEXPECTED_IRQ, MMC_ERR_MAX }

/* Function-pointer members retain the C ABI and nullability. */
#[repr(C)] pub struct mmc_host_ops { pub post_req: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request,i32)>, pub pre_req: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request)>, pub request: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request)>, pub request_atomic: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request)->i32>, pub set_ios: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_ios)>, pub get_ro: Option<unsafe extern "C" fn(*mut mmc_host)->i32>, pub get_cd: Option<unsafe extern "C" fn(*mut mmc_host)->i32>, pub enable_sdio_irq: Option<unsafe extern "C" fn(*mut mmc_host,i32)>, pub ack_sdio_irq: Option<unsafe extern "C" fn(*mut mmc_host)>, pub init_card: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_card)>, pub start_signal_voltage_switch: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_ios)->i32>, pub card_busy: Option<unsafe extern "C" fn(*mut mmc_host)->i32>, pub execute_tuning: Option<unsafe extern "C" fn(*mut mmc_host,u32)->i32>, pub prepare_hs400_tuning: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_ios)->i32>, pub execute_hs400_tuning: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_card)->i32>, pub prepare_sd_hs_tuning: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_card)->i32>, pub execute_sd_hs_tuning: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_card)->i32>, pub hs400_prepare_ddr: Option<unsafe extern "C" fn(*mut mmc_host)->i32>, pub hs400_downgrade: Option<unsafe extern "C" fn(*mut mmc_host)>, pub hs400_complete: Option<unsafe extern "C" fn(*mut mmc_host)>, pub hs400_enhanced_strobe: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_ios)>, pub select_drive_strength: Option<unsafe extern "C" fn(*mut mmc_card,u32,i32,i32,*mut i32)->i32>, pub card_hw_reset: Option<unsafe extern "C" fn(*mut mmc_host)>, pub card_event: Option<unsafe extern "C" fn(*mut mmc_host)>, pub multi_io_quirk: Option<unsafe extern "C" fn(*mut mmc_card,u32,i32)->i32>, pub init_sd_express: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_ios)->i32>, pub uhs2_control: Option<unsafe extern "C" fn(*mut mmc_host,sd_uhs2_operation)->i32> }

#[repr(C)] pub struct mmc_cqe_ops { pub cqe_enable: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_card)->i32>, pub cqe_disable: Option<unsafe extern "C" fn(*mut mmc_host)>, pub cqe_request: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request)->i32>, pub cqe_post_req: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request)>, pub cqe_off: Option<unsafe extern "C" fn(*mut mmc_host)>, pub cqe_wait_for_idle: Option<unsafe extern "C" fn(*mut mmc_host)->i32>, pub cqe_timeout: Option<unsafe extern "C" fn(*mut mmc_host,*mut mmc_request,*mut bool)->bool>, pub cqe_recovery_start: Option<unsafe extern "C" fn(*mut mmc_host)>, pub cqe_recovery_finish: Option<unsafe extern "C" fn(*mut mmc_host)> }

/* External types supplied by the included kernel headers. */
extern "C" { fn mmc_alloc_host(extra:i32, dev:*mut device)->*mut mmc_host; fn devm_mmc_alloc_host(dev:*mut device, extra:i32)->*mut mmc_host; fn mmc_add_host(host:*mut mmc_host)->i32; fn mmc_remove_host(host:*mut mmc_host); fn mmc_free_host(host:*mut mmc_host); fn mmc_of_parse_clk_phase(dev:*mut device,map:*mut mmc_clk_phase_map); fn mmc_of_parse(host:*mut mmc_host)->i32; fn mmc_of_parse_voltage(host:*mut mmc_host,mask:*mut u32)->i32; fn mmc_detect_change(host:*mut mmc_host, delay: usize); fn mmc_request_done(host:*mut mmc_host, req:*mut mmc_request); fn mmc_command_done(host:*mut mmc_host, req:*mut mmc_request); fn mmc_cqe_request_done(host:*mut mmc_host, req:*mut mmc_request); fn sdio_signal_irq(host:*mut mmc_host); fn mmc_regulator_get_supply(host:*mut mmc_host)->i32; fn mmc_regulator_enable_vqmmc(host:*mut mmc_host)->i32; fn mmc_regulator_disable_vqmmc(host:*mut mmc_host); fn mmc_retune_timer_stop(host:*mut mmc_host); fn mmc_sd_switch(card:*mut mmc_card,mode:bool,group:i32,value:u8,resp:*mut u8)->i32; fn mmc_send_status(card:*mut mmc_card,status:*mut u32)->i32; fn mmc_send_tuning(host:*mut mmc_host,opcode:u32,error:*mut i32)->i32; fn mmc_send_abort_tuning(host:*mut mmc_host,opcode:u32)->i32; fn mmc_get_ext_csd(card:*mut mmc_card,ext:*mut *mut u8)->i32; fn mmc_read_tuning(host:*mut mmc_host,blksz:u32,blocks:u32)->i32; }

#[inline] pub unsafe fn mmc_host_is_spi(host:*const mmc_host)->bool { (*(host as *const mmc_host_private)).caps & (1<<4) != 0 }
#[inline] pub unsafe fn sdio_irq_claimed(host:*const mmc_host_private)->bool { (*host).sdio_irqs > 0 }

/* The complete host object is layout-owned by the including kernel bindings. */
#[repr(C)] pub struct mmc_host_private { pub caps:u32, pub ios:mmc_ios, pub sdio_irqs:u32, pub need_retune:i32, pub can_retune:bool, pub doing_retune:bool, pub doing_init_tune:bool, pub timing:u8, pub err_stats:[u32;15] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
