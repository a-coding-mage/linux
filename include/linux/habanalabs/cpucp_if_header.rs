/* SPDX-License-Identifier: GPL-2.0 */
/* Faithful Rust translation of cpucp_if.h. */

pub type __le16 = u16;
pub type __le32 = u32;
pub type __le64 = u64;
pub type __u8 = u8;
pub type u32 = u32;
pub const ETH_ALEN: usize = 6;
pub const VERSION_MAX_LEN: usize = 64;

pub const NUM_HBM_PSEUDO_CH: u32 = 2;
pub const NUM_HBM_CH_PER_DEV: u32 = 8;
pub const PLL_MAP_MAX_BITS: u32 = 128;
pub const PLL_MAP_LEN: usize = (PLL_MAP_MAX_BITS / 8) as usize;
pub const HBM_CA_ERR_CMD_LIFO_LEN: usize = 8;
pub const HBM_RD_ERR_DATA_LIFO_LEN: usize = 8;
pub const HBM_WR_PAR_CMD_LIFO_LEN: usize = 11;
pub const ADDR_DEC_ADDRESS_COUNT_MAX: usize = 4;
pub const CPUCP_PACKET_FENCE_VAL: u32 = 0xFE8CE7A5;
pub const CARD_NAME_MAX_LEN: usize = 16;
pub const CPUCP_MAX_SENSORS: usize = 128;
pub const CPUCP_MAX_NICS: usize = 128;
pub const CPUCP_LANES_PER_NIC: usize = 4;
pub const CPUCP_NIC_QSFP_EEPROM_MAX_LEN: usize = 1024;
pub const CPUCP_MAX_NIC_LANES: usize = CPUCP_MAX_NICS * CPUCP_LANES_PER_NIC;
pub const CPUCP_NIC_MASK_ARR_LEN: usize = (CPUCP_MAX_NICS + 63) / 64;
pub const CPUCP_NIC_POLARITY_ARR_LEN: usize = (CPUCP_MAX_NIC_LANES + 63) / 64;
pub const CPUCP_HBM_ROW_REPLACE_MAX: usize = 32;
pub const PAGE_DISCARD_MAX: usize = 64;
pub const SEC_PCR_DATA_BUF_SZ: usize = 256;
pub const SEC_PCR_QUOTE_BUF_SZ: usize = 510;
pub const SEC_SIGNATURE_BUF_SZ: usize = 255;
pub const SEC_PUB_DATA_BUF_SZ: usize = 510;
pub const SEC_CERTIFICATE_BUF_SZ: usize = 2046;
pub const DCORE_MON_REGS_SZ: usize = 512;
pub const RAZWI_HAPPENED_HBW: u32 = 0x1;
pub const RAZWI_HAPPENED_LBW: u32 = 0x2;
pub const RAZWI_HAPPENED_AW: u32 = 0x4;
pub const RAZWI_HAPPENED_AR: u32 = 0x8;
pub const EQ_CTL_READY_SHIFT: u32 = 31;
pub const EQ_CTL_READY_MASK: u32 = 0x80000000;
pub const EQ_CTL_EVENT_MODE_SHIFT: u32 = 28;
pub const EQ_CTL_EVENT_MODE_MASK: u32 = 0x70000000;
pub const EQ_CTL_EVENT_TYPE_SHIFT: u32 = 16;
pub const EQ_CTL_EVENT_TYPE_MASK: u32 = 0x0FFF0000;
pub const EQ_CTL_INDEX_SHIFT: u32 = 0;
pub const EQ_CTL_INDEX_MASK: u32 = 0x0000FFFF;
pub const CPUCP_PKT_CTL_RC_SHIFT: u32 = 12;
pub const CPUCP_PKT_CTL_RC_MASK: u32 = 0x0000F000;
pub const CPUCP_PKT_CTL_OPCODE_SHIFT: u32 = 16;
pub const CPUCP_PKT_CTL_OPCODE_MASK: u32 = 0x1FFF0000;
pub const CPUCP_PKT_HB_STATUS_EQ_FAULT_SHIFT: u32 = 0;
pub const CPUCP_PKT_HB_STATUS_EQ_FAULT_MASK: u32 = 1;

macro_rules! c_enum { ($name:ident { $( $v:ident $(= $n:expr)? ),* $(,)? }) => {
    #[repr(C)] #[derive(Copy, Clone, Debug, PartialEq, Eq)] pub enum $name { $( $v $(= $n)? ),* }
};}
c_enum!(eq_event_id { EQ_EVENT_NIC_STS_REQUEST=0, EQ_EVENT_PWR_MODE_0, EQ_EVENT_PWR_MODE_1, EQ_EVENT_PWR_MODE_2, EQ_EVENT_PWR_MODE_3, EQ_EVENT_PWR_BRK_ENTRY, EQ_EVENT_PWR_BRK_EXIT, EQ_EVENT_HEARTBEAT, EQ_EVENT_CPLD_RESET_REASON, EQ_EVENT_CPLD_SHUTDOWN, EQ_EVENT_POWER_EVT_START, EQ_EVENT_POWER_EVT_END, EQ_EVENT_THERMAL_EVT_START, EQ_EVENT_THERMAL_EVT_END });
c_enum!(hl_sm_sei_cause { SM_SEI_SO_OVERFLOW, SM_SEI_LBW_4B_UNALIGNED, SM_SEI_AXI_RESPONSE_ERR });
c_enum!(hl_fw_alive_severity { FW_ALIVE_SEVERITY_MINOR, FW_ALIVE_SEVERITY_CRITICAL });
c_enum!(hl_hbm_sei_cause { HBM_SEI_CMD_PARITY_EVEN, HBM_SEI_CMD_PARITY_ODD, HBM_SEI_READ_ERR, HBM_SEI_WRITE_DATA_PARITY_ERR, HBM_SEI_CATTRIP, HBM_SEI_MEM_BIST_FAIL, HBM_SEI_DFI, HBM_SEI_INV_TEMP_READ_OUT, HBM_SEI_BIST_FAIL });
c_enum!(hl_engine_arc_interrupt_type { ENGINE_ARC_DCCM_QUEUE_FULL_IRQ=1 });
c_enum!(pq_init_status { PQ_INIT_STATUS_NA=0, PQ_INIT_STATUS_READY_FOR_CP, PQ_INIT_STATUS_READY_FOR_HOST, PQ_INIT_STATUS_READY_FOR_CP_SINGLE_MSI, PQ_INIT_STATUS_LEN_NOT_POWER_OF_TWO_ERR, PQ_INIT_STATUS_ILLEGAL_Q_ADDR_ERR });

#[repr(C)] pub struct cpucp_pkt_sync_err { pub pi: __le32, pub ci: __le32 }
#[repr(C)] pub struct hl_eq_hbm_ecc_data { pub sec_cnt: __le32, pub dec_cnt: __le32, pub hbm_ecc_info: __le32, pub first_addr: __le32, pub sec_cont_cnt: __le32, pub pad: __le32 }
#[repr(C)] pub struct hl_eq_header { pub reserved: __le32, pub ctl: __le32 }
#[repr(C)] pub struct hl_eq_ecc_data { pub ecc_address: __le64, pub ecc_syndrom: __le64, pub memory_wrapper_idx: __u8, pub is_critical: __u8, pub block_id: __le16, pub pad: [__u8;4] }
#[repr(C)] pub struct hl_eq_sm_sei_data { pub sei_log: __le32, pub sei_cause: __u8, pub pad: [__u8;3] }
#[repr(C)] pub struct hl_eq_fw_alive { pub uptime_seconds: __le64, pub process_id: __le32, pub thread_id: __le32, pub severity: __u8, pub pad: [__u8;7] }
#[repr(C)] pub struct hl_eq_intr_cause { pub intr_cause_data: __le64 }
#[repr(C)] pub struct hl_eq_pcie_drain_ind_data { pub intr_cause: hl_eq_intr_cause, pub drain_wr_addr_lbw: __le64, pub drain_rd_addr_lbw: __le64, pub drain_wr_addr_hbw: __le64, pub drain_rd_addr_hbw: __le64 }
#[repr(C)] pub struct hl_eq_razwi_lbw_info_regs { pub rr_aw_razwi_reg: __le32, pub rr_aw_razwi_id_reg: __le32, pub rr_ar_razwi_reg: __le32, pub rr_ar_razwi_id_reg: __le32 }
#[repr(C)] pub struct hl_eq_razwi_hbw_info_regs { pub rr_aw_razwi_hi_reg: __le32, pub rr_aw_razwi_lo_reg: __le32, pub rr_aw_razwi_id_reg: __le32, pub rr_ar_razwi_hi_reg: __le32, pub rr_ar_razwi_lo_reg: __le32, pub rr_ar_razwi_id_reg: __le32 }
#[repr(C)] pub union hl_eq_razwi_info_union { pub lbw: hl_eq_razwi_lbw_info_regs, pub hbw: hl_eq_razwi_hbw_info_regs }
#[repr(C)] pub struct hl_eq_razwi_info { pub razwi_happened_mask: __le32, pub regs: hl_eq_razwi_info_union, pub pad: __le32 }
#[repr(C)] pub struct hl_eq_razwi_with_intr_cause { pub razwi_info: hl_eq_razwi_info, pub intr_cause: hl_eq_intr_cause }

#[repr(C)] pub struct hbm_rd_addr { pub rd_addr_val: __le32 }
#[repr(C)] pub struct hl_eq_hbm_sei_read_err_intr_info { pub dbg_rd_err_addr: hbm_rd_addr, pub dbg_rd_err_misc: __le32, pub dbg_rd_err_dm: __le32, pub dbg_rd_err_syndrome: __le32, pub dbg_rd_err_data: [__le32; HBM_RD_ERR_DATA_LIFO_LEN] }
#[repr(C)] pub struct hl_eq_hbm_sei_ca_par_intr_info { pub dbg_row: [__le16; HBM_CA_ERR_CMD_LIFO_LEN], pub dbg_col: [__le32; HBM_CA_ERR_CMD_LIFO_LEN] }
#[repr(C)] pub struct hbm_sei_wr_cmd_address { pub dbg_wr_cmd_addr: __le32 }
#[repr(C)] pub struct hl_eq_hbm_sei_wr_par_intr_info { pub dbg_last_wr_cmds: [hbm_sei_wr_cmd_address; HBM_WR_PAR_CMD_LIFO_LEN], pub dbg_derr: __u8, pub pad: [__u8;3] }
#[repr(C)] pub union hl_hbm_sei_data_union { pub ca_parity_even_info: hl_eq_hbm_sei_ca_par_intr_info, pub ca_parity_odd_info: hl_eq_hbm_sei_ca_par_intr_info, pub read_err_info: hl_eq_hbm_sei_read_err_intr_info, pub wr_parity_info: hl_eq_hbm_sei_wr_par_intr_info }
#[repr(C)] pub struct hl_hbm_sei_header { pub cnt: __le32, pub sei_cause: __u8, pub mc_channel: __u8, pub mc_pseudo_channel: __u8, pub is_critical: __u8 }
#[repr(C)] pub struct hl_eq_hbm_sei_data { pub hdr: hl_hbm_sei_header, pub data: hl_hbm_sei_data_union }
#[repr(C)] pub struct hl_engine_arc_dccm_queue_full_irq { pub queue_index: __le32, pub pad: __le32 }
#[repr(C)] pub struct hl_eq_engine_arc_intr_data { pub engine_id: __le32, pub intr_type: __le32, pub payload: __le64, pub pad: [__le64;5] }
#[repr(C)] pub struct hl_eq_addr_dec_intr_data { pub intr_cause: hl_eq_intr_cause, pub addr: [__le64; ADDR_DEC_ADDRESS_COUNT_MAX], pub addr_cnt: __u8, pub pad: [__u8;7] }
#[repr(C)] pub union hl_eq_entry_data { pub data_placeholder: __le64, pub ecc_data: hl_eq_ecc_data, pub hbm_ecc_data: hl_eq_hbm_ecc_data, pub sm_sei_data: hl_eq_sm_sei_data, pub pkt_sync_err: cpucp_pkt_sync_err, pub fw_alive: hl_eq_fw_alive, pub intr_cause: hl_eq_intr_cause, pub pcie_drain_ind_data: hl_eq_pcie_drain_ind_data, pub razwi_info: hl_eq_razwi_info, pub razwi_with_intr_cause: hl_eq_razwi_with_intr_cause, pub sei_data: hl_eq_hbm_sei_data, pub arc_data: hl_eq_engine_arc_intr_data, pub addr_dec: hl_eq_addr_dec_intr_data, pub data: [__le64;7] }
#[repr(C)] pub struct hl_eq_entry { pub hdr: hl_eq_header, pub data: hl_eq_entry_data }
pub const HL_EQ_ENTRY_SIZE: usize = core::mem::size_of::<hl_eq_entry>();

// Packet identifiers retain C enum ordering.
#[repr(C)] #[derive(Copy,Clone,Debug,PartialEq,Eq)] pub enum cpucp_packet_id { CPUCP_PACKET_DISABLE_PCI_ACCESS=1, CPUCP_PACKET_ENABLE_PCI_ACCESS, CPUCP_PACKET_TEMPERATURE_GET, CPUCP_PACKET_VOLTAGE_GET, CPUCP_PACKET_CURRENT_GET, CPUCP_PACKET_FAN_SPEED_GET, CPUCP_PACKET_PWM_GET, CPUCP_PACKET_PWM_SET, CPUCP_PACKET_FREQUENCY_SET, CPUCP_PACKET_FREQUENCY_GET, CPUCP_PACKET_LED_SET, CPUCP_PACKET_I2C_WR, CPUCP_PACKET_I2C_RD, CPUCP_PACKET_INFO_GET, CPUCP_PACKET_FLASH_PROGRAM_REMOVED, CPUCP_PACKET_UNMASK_RAZWI_IRQ, CPUCP_PACKET_UNMASK_RAZWI_IRQ_ARRAY, CPUCP_PACKET_TEST, CPUCP_PACKET_FREQUENCY_CURR_GET, CPUCP_PACKET_MAX_POWER_GET, CPUCP_PACKET_MAX_POWER_SET, CPUCP_PACKET_EEPROM_DATA_GET, CPUCP_PACKET_NIC_INFO_GET, CPUCP_PACKET_TEMPERATURE_SET, CPUCP_PACKET_VOLTAGE_SET, CPUCP_PACKET_CURRENT_SET, CPUCP_PACKET_PCIE_THROUGHPUT_GET, CPUCP_PACKET_PCIE_REPLAY_CNT_GET, CPUCP_PACKET_TOTAL_ENERGY_GET, CPUCP_PACKET_PLL_INFO_GET, CPUCP_PACKET_NIC_STATUS, CPUCP_PACKET_POWER_GET, CPUCP_PACKET_NIC_PFC_SET, CPUCP_PACKET_NIC_FAULT_GET, CPUCP_PACKET_NIC_LPBK_SET, CPUCP_PACKET_NIC_MAC_CFG, CPUCP_PACKET_MSI_INFO_SET, CPUCP_PACKET_NIC_XPCS91_REGS_GET, CPUCP_PACKET_NIC_STAT_REGS_GET, CPUCP_PACKET_NIC_STAT_REGS_CLR, CPUCP_PACKET_NIC_STAT_REGS_ALL_GET, CPUCP_PACKET_IS_IDLE_CHECK, CPUCP_PACKET_HBM_REPLACED_ROWS_INFO_GET, CPUCP_PACKET_HBM_PENDING_ROWS_STATUS, CPUCP_PACKET_POWER_SET, CPUCP_PACKET_RESERVED, CPUCP_PACKET_ENGINE_CORE_ASID_SET, CPUCP_PACKET_RESERVED2, CPUCP_PACKET_SEC_ATTEST_GET, CPUCP_PACKET_INFO_SIGNED_GET, CPUCP_PACKET_RESERVED4, CPUCP_PACKET_MONITOR_DUMP_GET, CPUCP_PACKET_RESERVED5, CPUCP_PACKET_RESERVED6, CPUCP_PACKET_RESERVED7, CPUCP_PACKET_GENERIC_PASSTHROUGH, CPUCP_PACKET_RESERVED8, CPUCP_PACKET_ACTIVE_STATUS_SET, CPUCP_PACKET_RESERVED9, CPUCP_PACKET_RESERVED10, CPUCP_PACKET_RESERVED11, CPUCP_PACKET_RESERVED12, CPUCP_PACKET_RESERVED13, CPUCP_PACKET_SOFT_RESET, CPUCP_PACKET_INTS_REGISTER, CPUCP_PACKET_ID_MAX }

#[repr(C)] pub union cpucp_packet_first { pub value: __le64, pub result: __le64, pub addr: __le64 }
#[repr(C)] pub union cpucp_packet_args { pub sensor_index: __le16, pub type_: __le16, pub index: __le32, pub pll_index: __le32, pub led_index: __le32, pub data_max_size: __le32, pub status_mask: __le32 }
#[repr(C)] pub union cpucp_packet_last { pub port_index: __le32, pub pkt_subidx: __le32, pub nonce: __le32 }
#[repr(C)] pub struct cpucp_packet { pub first: cpucp_packet_first, pub ctl: __le32, pub fence: __le32, pub args: cpucp_packet_args, pub last: cpucp_packet_last }
#[repr(C)] pub struct cpucp_unmask_irq_arr_packet { pub cpucp_pkt: cpucp_packet, pub length: __le32, pub irqs: [__le32;0] }
#[repr(C)] pub struct cpucp_nic_status_packet { pub cpucp_pkt: cpucp_packet, pub length: __le32, pub data: [__le32;0] }
#[repr(C)] pub struct cpucp_array_data_packet { pub cpucp_pkt: cpucp_packet, pub length: __le32, pub data: [__le32;0] }

#[repr(C)] pub struct eq_generic_event { pub data: [__le64;7] }
#[repr(C)] pub struct cpucp_sensor { pub type_: __le32, pub flags: __le32 }
#[repr(C)] pub struct cpucp_security_info { pub config: __u8, pub keys_num: __u8, pub revoked_keys: __u8, pub min_svn: __u8 }
#[repr(C)] pub struct cpucp_mac_addr { pub mac_addr: [__u8;ETH_ALEN] }
#[repr(C)] pub struct cpucp_info { pub sensors: [cpucp_sensor;CPUCP_MAX_SENSORS], pub kernel_version: [__u8;VERSION_MAX_LEN], pub reserved1: __le32, pub card_type: __le32, pub card_location: __le32, pub cpld_version: __le32, pub infineon_version: __le32, pub fuse_version: [__u8;VERSION_MAX_LEN], pub thermal_version: [__u8;VERSION_MAX_LEN], pub cpucp_version: [__u8;VERSION_MAX_LEN], pub infineon_second_stage_version: __le32, pub dram_size: __le64, pub card_name: [u8;CARD_NAME_MAX_LEN], pub tpc_binning_mask: __le64, pub decoder_binning_mask: __le64, pub sram_binning: __u8, pub dram_binning_mask: __u8, pub memory_repair_flag: __u8, pub edma_binning_mask: __u8, pub xbar_binning_mask: __u8, pub interposer_version: __u8, pub substrate_version: __u8, pub eq_health_check_supported: __u8, pub sec_info: cpucp_security_info, pub cpld_timestamp: __le32, pub pll_map: [__u8;PLL_MAP_LEN], pub mme_binning_mask: __le64, pub fw_os_version: [__u8;VERSION_MAX_LEN] }
#[repr(C)] pub struct cpucp_nic_info { pub mac_addrs: [cpucp_mac_addr;CPUCP_MAX_NICS], pub link_mask: [__le64;CPUCP_NIC_MASK_ARR_LEN], pub pol_tx_mask: [__le64;CPUCP_NIC_POLARITY_ARR_LEN], pub pol_rx_mask: [__le64;CPUCP_NIC_POLARITY_ARR_LEN], pub link_ext_mask: [__le64;CPUCP_NIC_MASK_ARR_LEN], pub qsfp_eeprom: [__u8;CPUCP_NIC_QSFP_EEPROM_MAX_LEN], pub auto_neg_mask: [__le64;CPUCP_NIC_MASK_ARR_LEN], pub serdes_type: __le16, pub tx_swap_map: [__le16;CPUCP_MAX_NICS], pub reserved: [__u8;6] }
#[repr(C)] pub struct page_discard_info { pub num_entries: __u8, pub reserved: [__u8;7], pub mmu_page_idx: [__le32;PAGE_DISCARD_MAX] }
#[repr(C)] pub union frac_val_union { pub parts: [__le16;2], pub val: __le32 }
#[repr(C)] pub struct frac_val { pub data: frac_val_union }
#[repr(C)] pub struct ser_val { pub integer: __le16, pub exp: __le16 }
#[repr(C)] pub struct cpucp_nic_status { pub port: __le32, pub bad_format_cnt: __le32, pub responder_out_of_sequence_psn_cnt: __le32, pub high_ber_reinit: __le32, pub correctable_err_cnt: __le32, pub uncorrectable_err_cnt: __le32, pub retraining_cnt: __le32, pub up: __u8, pub pcs_link: __u8, pub phy_ready: __u8, pub auto_neg: __u8, pub timeout_retransmission_cnt: __le32, pub high_ber_cnt: __le32, pub pre_fec_ser: ser_val, pub post_fec_ser: ser_val, pub bandwidth: frac_val, pub lat: frac_val }
#[repr(C)] pub struct cpucp_hbm_row_info { pub hbm_idx: __u8, pub pc: __u8, pub sid: __u8, pub bank_idx: __u8, pub row_addr: __le16, pub replaced_row_cause: __u8, pub pad: __u8 }
#[repr(C)] pub struct cpucp_hbm_row_replaced_rows_info { pub num_replaced_rows: __le16, pub pad: [__u8;6], pub replaced_rows: [cpucp_hbm_row_info;CPUCP_HBM_ROW_REPLACE_MAX] }
#[repr(C)] pub struct cpucp_sec_attest_info { pub pcr_data: [__u8;SEC_PCR_DATA_BUF_SZ], pub pcr_num_reg: __u8, pub pcr_reg_len: __u8, pub pad0: __le16, pub nonce: __le32, pub pcr_quote_len: __le16, pub pcr_quote: [__u8;SEC_PCR_QUOTE_BUF_SZ], pub quote_sig_len: __u8, pub quote_sig: [__u8;SEC_SIGNATURE_BUF_SZ], pub pub_data_len: __le16, pub public_data: [__u8;SEC_PUB_DATA_BUF_SZ], pub certificate_len: __le16, pub certificate: [__u8;SEC_CERTIFICATE_BUF_SZ] }
#[repr(C)] pub struct cpucp_dev_info_signed { pub info: cpucp_info, pub nonce: __le32, pub pad0: __le32, pub info_sig_len: __u8, pub info_sig: [__u8;SEC_SIGNATURE_BUF_SZ], pub pub_data_len: __le16, pub public_data: [__u8;SEC_PUB_DATA_BUF_SZ], pub certificate_len: __le16, pub certificate: [__u8;SEC_CERTIFICATE_BUF_SZ] }
#[repr(C)] pub struct dcore_monitor_regs_data { pub mon_pay_addrl: [__le32;DCORE_MON_REGS_SZ], pub mon_pay_addrh: [__le32;DCORE_MON_REGS_SZ], pub mon_pay_data: [__le32;DCORE_MON_REGS_SZ], pub mon_arm: [__le32;DCORE_MON_REGS_SZ], pub mon_status: [__le32;DCORE_MON_REGS_SZ] }
#[repr(C)] pub struct cpucp_monitor_dump { pub sync_mngr_w_s: dcore_monitor_regs_data, pub sync_mngr_e_s: dcore_monitor_regs_data, pub sync_mngr_w_n: dcore_monitor_regs_data, pub sync_mngr_e_n: dcore_monitor_regs_data }

// Remaining C enums preserve declaration ordering and explicit values.
c_enum!(cpucp_packet_rc { cpucp_packet_success, cpucp_packet_invalid, cpucp_packet_fault, cpucp_packet_invalid_pkt, cpucp_packet_invalid_params, cpucp_packet_rc_max });
c_enum!(cpucp_led_index { CPUCP_LED0_INDEX=0, CPUCP_LED1_INDEX, CPUCP_LED2_INDEX, CPUCP_LED_MAX_INDEX });
c_enum!(cpucp_temp_type { cpucp_temp_input, cpucp_temp_min=4, cpucp_temp_min_hyst, cpucp_temp_max=6, cpucp_temp_max_hyst, cpucp_temp_crit, cpucp_temp_crit_hyst, cpucp_temp_offset=19, cpucp_temp_lowest=21, cpucp_temp_highest, cpucp_temp_reset_history, cpucp_temp_warn, cpucp_temp_max_crit, cpucp_temp_max_warn });
c_enum!(cpucp_in_attributes { cpucp_in_input, cpucp_in_min, cpucp_in_max, cpucp_in_lowest=6, cpucp_in_highest, cpucp_in_reset_history, cpucp_in_intr_alarm_a, cpucp_in_intr_alarm_b });
c_enum!(cpucp_curr_attributes { cpucp_curr_input, cpucp_curr_min, cpucp_curr_max, cpucp_curr_lowest=6, cpucp_curr_highest, cpucp_curr_reset_history });
c_enum!(cpucp_fan_attributes { cpucp_fan_input, cpucp_fan_min=2, cpucp_fan_max });
c_enum!(cpucp_pwm_attributes { cpucp_pwm_input, cpucp_pwm_enable });
c_enum!(cpucp_pcie_throughput_attributes { cpucp_pcie_throughput_tx, cpucp_pcie_throughput_rx });
c_enum!(cpucp_power_type { CPUCP_POWER_INPUT=8, CPUCP_POWER_INPUT_HIGHEST, CPUCP_POWER_RESET_INPUT_HISTORY=11 });
c_enum!(cpucp_pll_reg_attributes { cpucp_pll_nr_reg, cpucp_pll_nf_reg, cpucp_pll_od_reg, cpucp_pll_div_factor_reg, cpucp_pll_div_sel_reg });
c_enum!(cpucp_pll_type_attributes { cpucp_pll_cpu, cpucp_pll_pci });
c_enum!(cpucp_msi_type { CPUCP_EVENT_QUEUE_MSI_TYPE, CPUCP_NIC_PORT1_MSI_TYPE, CPUCP_NIC_PORT3_MSI_TYPE, CPUCP_NIC_PORT5_MSI_TYPE, CPUCP_NIC_PORT7_MSI_TYPE, CPUCP_NIC_PORT9_MSI_TYPE, CPUCP_EVENT_QUEUE_ERR_MSI_TYPE, CPUCP_NUM_OF_MSI_TYPES });
c_enum!(pll_index { CPU_PLL=0, PCI_PLL=1, NIC_PLL, DMA_PLL, MESH_PLL, MME_PLL, TPC_PLL, IF_PLL, SRAM_PLL, NS_PLL, HBM_PLL, MSS_PLL, DDR_PLL, VID_PLL, BANK_PLL, MMU_PLL, IC_PLL, MC_PLL, EMMC_PLL, D2D_PLL, CS_PLL, C2C_PLL, NCH_PLL, C2M_PLL, PLL_MAX });
c_enum!(rl_index { TPC_RL=0, MME_RL, EDMA_RL });
c_enum!(pvt_index { PVT_SW, PVT_SE, PVT_NW, PVT_NE });
c_enum!(cpucp_card_types { cpucp_card_type_pci, cpucp_card_type_pmc });
c_enum!(cpucp_serdes_type { TYPE_1_SERDES_TYPE, TYPE_2_SERDES_TYPE, HLS1_SERDES_TYPE, HLS1H_SERDES_TYPE, HLS2_SERDES_TYPE, HLS2_TYPE_1_SERDES_TYPE, MAX_NUM_SERDES_TYPE, UNKNOWN_SERDES_TYPE=0xFFFF });
c_enum!(cpucp_hbm_row_replace_cause { REPLACE_CAUSE_DOUBLE_ECC_ERR, REPLACE_CAUSE_MULTI_SINGLE_ECC_ERR });
c_enum!(cpu_reset_status { CPU_RST_STATUS_NA=0, CPU_RST_STATUS_SOFT_RST_DONE=1 });
c_enum!(hl_passthrough_type { HL_PASSTHROUGH_VERSIONS, HL_GET_ERR_COUNTERS_CMD, HL_GET_P_STATE });

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
