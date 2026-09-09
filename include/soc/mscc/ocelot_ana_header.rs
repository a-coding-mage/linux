/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Microsemi Ocelot Switch driver
 *
 * Copyright (c) 2017 Microsemi Corporation
 */

pub const _MSCC_OCELOT_ANA_H_: u32 = ;

pub const ANA_ANAGEFIL_B_DOM_EN: u32 = BIT!(22);
pub const ANA_ANAGEFIL_B_DOM_VAL: u32 = BIT!(21);
pub const ANA_ANAGEFIL_AGE_LOCKED: u32 = BIT!(20);
pub const ANA_ANAGEFIL_PID_EN: u32 = BIT!(19);
macro_rules! ANA_ANAGEFIL_PID_VAL { ($x:expr) => { ((($x) << 14) & GENMASK!(18, 14)) }; }
pub const ANA_ANAGEFIL_PID_VAL_M: u32 = GENMASK!(18, 14);
macro_rules! ANA_ANAGEFIL_PID_VAL_X { ($x:expr) => { ((($x) & GENMASK!(18, 14)) >> 14) }; }
pub const ANA_ANAGEFIL_VID_EN: u32 = BIT!(13);
macro_rules! ANA_ANAGEFIL_VID_VAL { ($x:expr) => { (($x) & GENMASK!(12, 0)) }; }
pub const ANA_ANAGEFIL_VID_VAL_M: u32 = GENMASK!(12, 0);

pub const ANA_STORMLIMIT_CFG_RSZ: u32 = 0x4;

macro_rules! ANA_STORMLIMIT_CFG_STORM_RATE { ($x:expr) => { ((($x) << 3) & GENMASK!(6, 3)) }; }
pub const ANA_STORMLIMIT_CFG_STORM_RATE_M: u32 = GENMASK!(6, 3);
macro_rules! ANA_STORMLIMIT_CFG_STORM_RATE_X { ($x:expr) => { ((($x) & GENMASK!(6, 3)) >> 3) }; }
pub const ANA_STORMLIMIT_CFG_STORM_UNIT: u32 = BIT!(2);
macro_rules! ANA_STORMLIMIT_CFG_STORM_MODE { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_STORMLIMIT_CFG_STORM_MODE_M: u32 = GENMASK!(1, 0);

pub const ANA_AUTOAGE_AGE_FAST: u32 = BIT!(21);
macro_rules! ANA_AUTOAGE_AGE_PERIOD { ($x:expr) => { ((($x) << 1) & GENMASK!(20, 1)) }; }
pub const ANA_AUTOAGE_AGE_PERIOD_M: u32 = GENMASK!(20, 1);
macro_rules! ANA_AUTOAGE_AGE_PERIOD_X { ($x:expr) => { ((($x) & GENMASK!(20, 1)) >> 1) }; }
pub const ANA_AUTOAGE_AUTOAGE_LOCKED: u32 = BIT!(0);

pub const ANA_MACTOPTIONS_REDUCED_TABLE: u32 = BIT!(1);
pub const ANA_MACTOPTIONS_SHADOW: u32 = BIT!(0);

macro_rules! ANA_AGENCTRL_FID_MASK { ($x:expr) => { ((($x) << 12) & GENMASK!(23, 12)) }; }
pub const ANA_AGENCTRL_FID_MASK_M: u32 = GENMASK!(23, 12);
macro_rules! ANA_AGENCTRL_FID_MASK_X { ($x:expr) => { ((($x) & GENMASK!(23, 12)) >> 12) }; }
pub const ANA_AGENCTRL_IGNORE_DMAC_FLAGS: u32 = BIT!(11);
pub const ANA_AGENCTRL_IGNORE_SMAC_FLAGS: u32 = BIT!(10);
pub const ANA_AGENCTRL_FLOOD_SPECIAL: u32 = BIT!(9);
pub const ANA_AGENCTRL_FLOOD_IGNORE_VLAN: u32 = BIT!(8);
pub const ANA_AGENCTRL_MIRROR_CPU: u32 = BIT!(7);
pub const ANA_AGENCTRL_LEARN_CPU_COPY: u32 = BIT!(6);
pub const ANA_AGENCTRL_LEARN_FWD_KILL: u32 = BIT!(5);
pub const ANA_AGENCTRL_LEARN_IGNORE_VLAN: u32 = BIT!(4);
pub const ANA_AGENCTRL_CPU_CPU_KILL_ENA: u32 = BIT!(3);
pub const ANA_AGENCTRL_GREEN_COUNT_MODE: u32 = BIT!(2);
pub const ANA_AGENCTRL_YELLOW_COUNT_MODE: u32 = BIT!(1);
pub const ANA_AGENCTRL_RED_COUNT_MODE: u32 = BIT!(0);

pub const ANA_FLOODING_RSZ: u32 = 0x4;

macro_rules! ANA_FLOODING_FLD_UNICAST { ($x:expr) => { ((($x) << 12) & GENMASK!(17, 12)) }; }
pub const ANA_FLOODING_FLD_UNICAST_M: u32 = GENMASK!(17, 12);
macro_rules! ANA_FLOODING_FLD_UNICAST_X { ($x:expr) => { ((($x) & GENMASK!(17, 12)) >> 12) }; }
macro_rules! ANA_FLOODING_FLD_BROADCAST { ($x:expr) => { ((($x) << 6) & GENMASK!(11, 6)) }; }
pub const ANA_FLOODING_FLD_BROADCAST_M: u32 = GENMASK!(11, 6);
macro_rules! ANA_FLOODING_FLD_BROADCAST_X { ($x:expr) => { ((($x) & GENMASK!(11, 6)) >> 6) }; }
macro_rules! ANA_FLOODING_FLD_MULTICAST { ($x:expr) => { (($x) & GENMASK!(5, 0)) }; }
pub const ANA_FLOODING_FLD_MULTICAST_M: u32 = GENMASK!(5, 0);

macro_rules! ANA_FLOODING_IPMC_FLD_MC4_CTRL { ($x:expr) => { ((($x) << 18) & GENMASK!(23, 18)) }; }
pub const ANA_FLOODING_IPMC_FLD_MC4_CTRL_M: u32 = GENMASK!(23, 18);
macro_rules! ANA_FLOODING_IPMC_FLD_MC4_CTRL_X { ($x:expr) => { ((($x) & GENMASK!(23, 18)) >> 18) }; }
macro_rules! ANA_FLOODING_IPMC_FLD_MC4_DATA { ($x:expr) => { ((($x) << 12) & GENMASK!(17, 12)) }; }
pub const ANA_FLOODING_IPMC_FLD_MC4_DATA_M: u32 = GENMASK!(17, 12);
macro_rules! ANA_FLOODING_IPMC_FLD_MC4_DATA_X { ($x:expr) => { ((($x) & GENMASK!(17, 12)) >> 12) }; }
macro_rules! ANA_FLOODING_IPMC_FLD_MC6_CTRL { ($x:expr) => { ((($x) << 6) & GENMASK!(11, 6)) }; }
pub const ANA_FLOODING_IPMC_FLD_MC6_CTRL_M: u32 = GENMASK!(11, 6);
macro_rules! ANA_FLOODING_IPMC_FLD_MC6_CTRL_X { ($x:expr) => { ((($x) & GENMASK!(11, 6)) >> 6) }; }
macro_rules! ANA_FLOODING_IPMC_FLD_MC6_DATA { ($x:expr) => { (($x) & GENMASK!(5, 0)) }; }
pub const ANA_FLOODING_IPMC_FLD_MC6_DATA_M: u32 = GENMASK!(5, 0);

pub const ANA_SFLOW_CFG_RSZ: u32 = 0x4;

macro_rules! ANA_SFLOW_CFG_SF_RATE { ($x:expr) => { ((($x) << 2) & GENMASK!(13, 2)) }; }
pub const ANA_SFLOW_CFG_SF_RATE_M: u32 = GENMASK!(13, 2);
macro_rules! ANA_SFLOW_CFG_SF_RATE_X { ($x:expr) => { ((($x) & GENMASK!(13, 2)) >> 2) }; }
pub const ANA_SFLOW_CFG_SF_SAMPLE_RX: u32 = BIT!(1);
pub const ANA_SFLOW_CFG_SF_SAMPLE_TX: u32 = BIT!(0);

pub const ANA_PORT_MODE_RSZ: u32 = 0x4;

pub const ANA_PORT_MODE_REDTAG_PARSE_CFG: u32 = BIT!(3);
macro_rules! ANA_PORT_MODE_VLAN_PARSE_CFG { ($x:expr) => { ((($x) << 1) & GENMASK!(2, 1)) }; }
pub const ANA_PORT_MODE_VLAN_PARSE_CFG_M: u32 = GENMASK!(2, 1);
macro_rules! ANA_PORT_MODE_VLAN_PARSE_CFG_X { ($x:expr) => { ((($x) & GENMASK!(2, 1)) >> 1) }; }
pub const ANA_PORT_MODE_L3_PARSE_CFG: u32 = BIT!(0);

pub const ANA_CUT_THRU_CFG_RSZ: u32 = 0x4;

pub const ANA_PGID_PGID_RSZ: u32 = 0x4;

macro_rules! ANA_PGID_PGID_PGID { ($x:expr) => { (($x) & GENMASK!(11, 0)) }; }
pub const ANA_PGID_PGID_PGID_M: u32 = GENMASK!(11, 0);
macro_rules! ANA_PGID_PGID_CPUQ_DST_PGID { ($x:expr) => { ((($x) << 27) & GENMASK!(29, 27)) }; }
pub const ANA_PGID_PGID_CPUQ_DST_PGID_M: u32 = GENMASK!(29, 27);
macro_rules! ANA_PGID_PGID_CPUQ_DST_PGID_X { ($x:expr) => { ((($x) & GENMASK!(29, 27)) >> 27) }; }

macro_rules! ANA_TABLES_MACHDATA_VID { ($x:expr) => { ((($x) << 16) & GENMASK!(28, 16)) }; }
pub const ANA_TABLES_MACHDATA_VID_M: u32 = GENMASK!(28, 16);
macro_rules! ANA_TABLES_MACHDATA_VID_X { ($x:expr) => { ((($x) & GENMASK!(28, 16)) >> 16) }; }
macro_rules! ANA_TABLES_MACHDATA_MACHDATA { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_TABLES_MACHDATA_MACHDATA_M: u32 = GENMASK!(15, 0);

pub const ANA_TABLES_STREAMDATA_SSID_VALID: u32 = BIT!(16);
macro_rules! ANA_TABLES_STREAMDATA_SSID { ($x:expr) => { ((($x) << 9) & GENMASK!(15, 9)) }; }
pub const ANA_TABLES_STREAMDATA_SSID_M: u32 = GENMASK!(15, 9);
macro_rules! ANA_TABLES_STREAMDATA_SSID_X { ($x:expr) => { ((($x) & GENMASK!(15, 9)) >> 9) }; }
pub const ANA_TABLES_STREAMDATA_SFID_VALID: u32 = BIT!(8);
macro_rules! ANA_TABLES_STREAMDATA_SFID { ($x:expr) => { (($x) & GENMASK!(7, 0)) }; }
pub const ANA_TABLES_STREAMDATA_SFID_M: u32 = GENMASK!(7, 0);

pub const ANA_TABLES_MACACCESS_MAC_CPU_COPY: u32 = BIT!(15);
pub const ANA_TABLES_MACACCESS_SRC_KILL: u32 = BIT!(14);
pub const ANA_TABLES_MACACCESS_IGNORE_VLAN: u32 = BIT!(13);
pub const ANA_TABLES_MACACCESS_AGED_FLAG: u32 = BIT!(12);
pub const ANA_TABLES_MACACCESS_VALID: u32 = BIT!(11);
macro_rules! ANA_TABLES_MACACCESS_ENTRYTYPE { ($x:expr) => { ((($x) << 9) & GENMASK!(10, 9)) }; }
pub const ANA_TABLES_MACACCESS_ENTRYTYPE_M: u32 = GENMASK!(10, 9);
macro_rules! ANA_TABLES_MACACCESS_ENTRYTYPE_X { ($x:expr) => { ((($x) & GENMASK!(10, 9)) >> 9) }; }
macro_rules! ANA_TABLES_MACACCESS_DEST_IDX { ($x:expr) => { ((($x) << 3) & GENMASK!(8, 3)) }; }
pub const ANA_TABLES_MACACCESS_DEST_IDX_M: u32 = GENMASK!(8, 3);
macro_rules! ANA_TABLES_MACACCESS_DEST_IDX_X { ($x:expr) => { ((($x) & GENMASK!(8, 3)) >> 3) }; }
macro_rules! ANA_TABLES_MACACCESS_MAC_TABLE_CMD { ($x:expr) => { (($x) & GENMASK!(2, 0)) }; }
pub const ANA_TABLES_MACACCESS_MAC_TABLE_CMD_M: u32 = GENMASK!(2, 0);
pub const MACACCESS_CMD_IDLE: u32 = 0;
pub const MACACCESS_CMD_LEARN: u32 = 1;
pub const MACACCESS_CMD_FORGET: u32 = 2;
pub const MACACCESS_CMD_AGE: u32 = 3;
pub const MACACCESS_CMD_GET_NEXT: u32 = 4;
pub const MACACCESS_CMD_INIT: u32 = 5;
pub const MACACCESS_CMD_READ: u32 = 6;
pub const MACACCESS_CMD_WRITE: u32 = 7;

macro_rules! ANA_TABLES_VLANACCESS_VLAN_PORT_MASK { ($x:expr) => { ((($x) << 2) & GENMASK!(13, 2)) }; }
pub const ANA_TABLES_VLANACCESS_VLAN_PORT_MASK_M: u32 = GENMASK!(13, 2);
macro_rules! ANA_TABLES_VLANACCESS_VLAN_PORT_MASK_X { ($x:expr) => { ((($x) & GENMASK!(13, 2)) >> 2) }; }
macro_rules! ANA_TABLES_VLANACCESS_VLAN_TBL_CMD { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_TABLES_VLANACCESS_VLAN_TBL_CMD_M: u32 = GENMASK!(1, 0);
pub const ANA_TABLES_VLANACCESS_CMD_IDLE: u32 = 0x0;
pub const ANA_TABLES_VLANACCESS_CMD_WRITE: u32 = 0x2;
pub const ANA_TABLES_VLANACCESS_CMD_INIT: u32 = 0x3;

pub const ANA_TABLES_VLANTIDX_VLAN_SEC_FWD_ENA: u32 = BIT!(17);
pub const ANA_TABLES_VLANTIDX_VLAN_FLOOD_DIS: u32 = BIT!(16);
pub const ANA_TABLES_VLANTIDX_VLAN_PRIV_VLAN: u32 = BIT!(15);
pub const ANA_TABLES_VLANTIDX_VLAN_LEARN_DISABLED: u32 = BIT!(14);
pub const ANA_TABLES_VLANTIDX_VLAN_MIRROR: u32 = BIT!(13);
pub const ANA_TABLES_VLANTIDX_VLAN_SRC_CHK: u32 = BIT!(12);
macro_rules! ANA_TABLES_VLANTIDX_V_INDEX { ($x:expr) => { (($x) & GENMASK!(11, 0)) }; }
pub const ANA_TABLES_VLANTIDX_V_INDEX_M: u32 = GENMASK!(11, 0);

macro_rules! ANA_TABLES_ISDXACCESS_ISDX_PORT_MASK { ($x:expr) => { ((($x) << 2) & GENMASK!(8, 2)) }; }
pub const ANA_TABLES_ISDXACCESS_ISDX_PORT_MASK_M: u32 = GENMASK!(8, 2);
macro_rules! ANA_TABLES_ISDXACCESS_ISDX_PORT_MASK_X { ($x:expr) => { ((($x) & GENMASK!(8, 2)) >> 2) }; }
macro_rules! ANA_TABLES_ISDXACCESS_ISDX_TBL_CMD { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_TABLES_ISDXACCESS_ISDX_TBL_CMD_M: u32 = GENMASK!(1, 0);

macro_rules! ANA_TABLES_ISDXTIDX_ISDX_SDLBI { ($x:expr) => { ((($x) << 21) & GENMASK!(28, 21)) }; }
pub const ANA_TABLES_ISDXTIDX_ISDX_SDLBI_M: u32 = GENMASK!(28, 21);
macro_rules! ANA_TABLES_ISDXTIDX_ISDX_SDLBI_X { ($x:expr) => { ((($x) & GENMASK!(28, 21)) >> 21) }; }
macro_rules! ANA_TABLES_ISDXTIDX_ISDX_MSTI { ($x:expr) => { ((($x) << 15) & GENMASK!(20, 15)) }; }
pub const ANA_TABLES_ISDXTIDX_ISDX_MSTI_M: u32 = GENMASK!(20, 15);
macro_rules! ANA_TABLES_ISDXTIDX_ISDX_MSTI_X { ($x:expr) => { ((($x) & GENMASK!(20, 15)) >> 15) }; }
pub const ANA_TABLES_ISDXTIDX_ISDX_ES0_KEY_ENA: u32 = BIT!(14);
pub const ANA_TABLES_ISDXTIDX_ISDX_FORCE_ENA: u32 = BIT!(10);
macro_rules! ANA_TABLES_ISDXTIDX_ISDX_INDEX { ($x:expr) => { (($x) & GENMASK!(7, 0)) }; }
pub const ANA_TABLES_ISDXTIDX_ISDX_INDEX_M: u32 = GENMASK!(7, 0);

pub const ANA_TABLES_ENTRYLIM_RSZ: u32 = 0x4;

macro_rules! ANA_TABLES_ENTRYLIM_ENTRYLIM { ($x:expr) => { ((($x) << 14) & GENMASK!(17, 14)) }; }
pub const ANA_TABLES_ENTRYLIM_ENTRYLIM_M: u32 = GENMASK!(17, 14);
macro_rules! ANA_TABLES_ENTRYLIM_ENTRYLIM_X { ($x:expr) => { ((($x) & GENMASK!(17, 14)) >> 14) }; }
macro_rules! ANA_TABLES_ENTRYLIM_ENTRYSTAT { ($x:expr) => { (($x) & GENMASK!(13, 0)) }; }
pub const ANA_TABLES_ENTRYLIM_ENTRYSTAT_M: u32 = GENMASK!(13, 0);

macro_rules! ANA_TABLES_STREAMACCESS_GEN_REC_SEQ_NUM { ($x:expr) => { ((($x) << 4) & GENMASK!(31, 4)) }; }
pub const ANA_TABLES_STREAMACCESS_GEN_REC_SEQ_NUM_M: u32 = GENMASK!(31, 4);
macro_rules! ANA_TABLES_STREAMACCESS_GEN_REC_SEQ_NUM_X { ($x:expr) => { ((($x) & GENMASK!(31, 4)) >> 4) }; }
pub const ANA_TABLES_STREAMACCESS_SEQ_GEN_REC_ENA: u32 = BIT!(3);
pub const ANA_TABLES_STREAMACCESS_GEN_REC_TYPE: u32 = BIT!(2);
macro_rules! ANA_TABLES_STREAMACCESS_STREAM_TBL_CMD { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_TABLES_STREAMACCESS_STREAM_TBL_CMD_M: u32 = GENMASK!(1, 0);

macro_rules! ANA_TABLES_STREAMTIDX_SEQ_GEN_ERR_STATUS { ($x:expr) => { ((($x) << 30) & GENMASK!(31, 30)) }; }
pub const ANA_TABLES_STREAMTIDX_SEQ_GEN_ERR_STATUS_M: u32 = GENMASK!(31, 30);
macro_rules! ANA_TABLES_STREAMTIDX_SEQ_GEN_ERR_STATUS_X { ($x:expr) => { ((($x) & GENMASK!(31, 30)) >> 30) }; }
macro_rules! ANA_TABLES_STREAMTIDX_S_INDEX { ($x:expr) => { ((($x) << 16) & GENMASK!(22, 16)) }; }
pub const ANA_TABLES_STREAMTIDX_S_INDEX_M: u32 = GENMASK!(22, 16);
macro_rules! ANA_TABLES_STREAMTIDX_S_INDEX_X { ($x:expr) => { ((($x) & GENMASK!(22, 16)) >> 16) }; }
pub const ANA_TABLES_STREAMTIDX_FORCE_SF_BEHAVIOUR: u32 = BIT!(14);
macro_rules! ANA_TABLES_STREAMTIDX_SEQ_HISTORY_LEN { ($x:expr) => { ((($x) << 8) & GENMASK!(13, 8)) }; }
pub const ANA_TABLES_STREAMTIDX_SEQ_HISTORY_LEN_M: u32 = GENMASK!(13, 8);
macro_rules! ANA_TABLES_STREAMTIDX_SEQ_HISTORY_LEN_X { ($x:expr) => { ((($x) & GENMASK!(13, 8)) >> 8) }; }
pub const ANA_TABLES_STREAMTIDX_RESET_ON_ROGUE: u32 = BIT!(7);
pub const ANA_TABLES_STREAMTIDX_REDTAG_POP: u32 = BIT!(6);
pub const ANA_TABLES_STREAMTIDX_STREAM_SPLIT: u32 = BIT!(5);
macro_rules! ANA_TABLES_STREAMTIDX_SEQ_SPACE_LOG2 { ($x:expr) => { (($x) & GENMASK!(4, 0)) }; }
pub const ANA_TABLES_STREAMTIDX_SEQ_SPACE_LOG2_M: u32 = GENMASK!(4, 0);

macro_rules! ANA_TABLES_SEQ_MASK_SPLIT_MASK { ($x:expr) => { ((($x) << 16) & GENMASK!(22, 16)) }; }
pub const ANA_TABLES_SEQ_MASK_SPLIT_MASK_M: u32 = GENMASK!(22, 16);
macro_rules! ANA_TABLES_SEQ_MASK_SPLIT_MASK_X { ($x:expr) => { ((($x) & GENMASK!(22, 16)) >> 16) }; }
macro_rules! ANA_TABLES_SEQ_MASK_INPUT_PORT_MASK { ($x:expr) => { (($x) & GENMASK!(6, 0)) }; }
pub const ANA_TABLES_SEQ_MASK_INPUT_PORT_MASK_M: u32 = GENMASK!(6, 0);

macro_rules! ANA_TABLES_SFID_MASK_IGR_PORT_MASK { ($x:expr) => { ((($x) << 1) & GENMASK!(7, 1)) }; }
pub const ANA_TABLES_SFID_MASK_IGR_PORT_MASK_M: u32 = GENMASK!(7, 1);
macro_rules! ANA_TABLES_SFID_MASK_IGR_PORT_MASK_X { ($x:expr) => { ((($x) & GENMASK!(7, 1)) >> 1) }; }
pub const ANA_TABLES_SFID_MASK_IGR_SRCPORT_MATCH_ENA: u32 = BIT!(0);

pub const ANA_TABLES_SFIDACCESS_IGR_PRIO_MATCH_ENA: u32 = BIT!(22);
macro_rules! ANA_TABLES_SFIDACCESS_IGR_PRIO { ($x:expr) => { ((($x) << 19) & GENMASK!(21, 19)) }; }
pub const ANA_TABLES_SFIDACCESS_IGR_PRIO_M: u32 = GENMASK!(21, 19);
macro_rules! ANA_TABLES_SFIDACCESS_IGR_PRIO_X { ($x:expr) => { ((($x) & GENMASK!(21, 19)) >> 19) }; }
pub const ANA_TABLES_SFIDACCESS_FORCE_BLOCK: u32 = BIT!(18);
macro_rules! ANA_TABLES_SFIDACCESS_MAX_SDU_LEN { ($x:expr) => { ((($x) << 2) & GENMASK!(17, 2)) }; }
pub const ANA_TABLES_SFIDACCESS_MAX_SDU_LEN_M: u32 = GENMASK!(17, 2);
macro_rules! ANA_TABLES_SFIDACCESS_MAX_SDU_LEN_X { ($x:expr) => { ((($x) & GENMASK!(17, 2)) >> 2) }; }
macro_rules! ANA_TABLES_SFIDACCESS_SFID_TBL_CMD { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_TABLES_SFIDACCESS_SFID_TBL_CMD_M: u32 = GENMASK!(1, 0);

pub const SFIDACCESS_CMD_IDLE: u32 = 0;
pub const SFIDACCESS_CMD_READ: u32 = 1;
pub const SFIDACCESS_CMD_WRITE: u32 = 2;
pub const SFIDACCESS_CMD_INIT: u32 = 3;

pub const ANA_TABLES_SFIDTIDX_SGID_VALID: u32 = BIT!(26);
macro_rules! ANA_TABLES_SFIDTIDX_SGID { ($x:expr) => { ((($x) << 18) & GENMASK!(25, 18)) }; }
pub const ANA_TABLES_SFIDTIDX_SGID_M: u32 = GENMASK!(25, 18);
macro_rules! ANA_TABLES_SFIDTIDX_SGID_X { ($x:expr) => { ((($x) & GENMASK!(25, 18)) >> 18) }; }
pub const ANA_TABLES_SFIDTIDX_POL_ENA: u32 = BIT!(17);
macro_rules! ANA_TABLES_SFIDTIDX_POL_IDX { ($x:expr) => { ((($x) << 8) & GENMASK!(16, 8)) }; }
pub const ANA_TABLES_SFIDTIDX_POL_IDX_M: u32 = GENMASK!(16, 8);
macro_rules! ANA_TABLES_SFIDTIDX_POL_IDX_X { ($x:expr) => { ((($x) & GENMASK!(16, 8)) >> 8) }; }
macro_rules! ANA_TABLES_SFIDTIDX_SFID_INDEX { ($x:expr) => { (($x) & GENMASK!(7, 0)) }; }
pub const ANA_TABLES_SFIDTIDX_SFID_INDEX_M: u32 = GENMASK!(7, 0);

pub const ANA_MSTI_STATE_RSZ: u32 = 0x4;

pub const ANA_OAM_UPM_LM_CNT_RSZ: u32 = 0x4;

macro_rules! ANA_SG_ACCESS_CTRL_SGID { ($x:expr) => { (($x) & GENMASK!(7, 0)) }; }
pub const ANA_SG_ACCESS_CTRL_SGID_M: u32 = GENMASK!(7, 0);
pub const ANA_SG_ACCESS_CTRL_CONFIG_CHANGE: u32 = BIT!(28);

macro_rules! ANA_SG_CONFIG_REG_3_BASE_TIME_SEC_MSB { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_SG_CONFIG_REG_3_BASE_TIME_SEC_MSB_M: u32 = GENMASK!(15, 0);
macro_rules! ANA_SG_CONFIG_REG_3_LIST_LENGTH { ($x:expr) => { ((($x) << 16) & GENMASK!(18, 16)) }; }
pub const ANA_SG_CONFIG_REG_3_LIST_LENGTH_M: u32 = GENMASK!(18, 16);
macro_rules! ANA_SG_CONFIG_REG_3_LIST_LENGTH_X { ($x:expr) => { ((($x) & GENMASK!(18, 16)) >> 16) }; }
pub const ANA_SG_CONFIG_REG_3_GATE_ENABLE: u32 = BIT!(20);
macro_rules! ANA_SG_CONFIG_REG_3_INIT_IPS { ($x:expr) => { ((($x) << 21) & GENMASK!(24, 21)) }; }
pub const ANA_SG_CONFIG_REG_3_INIT_IPS_M: u32 = GENMASK!(24, 21);
macro_rules! ANA_SG_CONFIG_REG_3_INIT_IPS_X { ($x:expr) => { ((($x) & GENMASK!(24, 21)) >> 21) }; }
pub const ANA_SG_CONFIG_REG_3_IPV_VALID: u32 = BIT!(24);
macro_rules! ANA_SG_CONFIG_REG_3_IPV_INVALID { ($x:expr) => { ((($x) << 24) & GENMASK!(24, 24)) }; }
macro_rules! ANA_SG_CONFIG_REG_3_INIT_IPV { ($x:expr) => { ((($x) << 21) & GENMASK!(23, 21)) }; }
pub const ANA_SG_CONFIG_REG_3_INIT_IPV_M: u32 = GENMASK!(23, 21);
macro_rules! ANA_SG_CONFIG_REG_3_INIT_IPV_X { ($x:expr) => { ((($x) & GENMASK!(23, 21)) >> 21) }; }
pub const ANA_SG_CONFIG_REG_3_INIT_GATE_STATE: u32 = BIT!(25);

pub const ANA_SG_GCL_GS_CONFIG_RSZ: u32 = 0x4;

macro_rules! ANA_SG_GCL_GS_CONFIG_IPS { ($x:expr) => { (($x) & GENMASK!(3, 0)) }; }
pub const ANA_SG_GCL_GS_CONFIG_IPS_M: u32 = GENMASK!(3, 0);
pub const ANA_SG_GCL_GS_CONFIG_GATE_STATE: u32 = BIT!(4);

pub const ANA_SG_GCL_TI_CONFIG_RSZ: u32 = 0x4;

macro_rules! ANA_SG_STATUS_REG_3_CFG_CHG_TIME_SEC_MSB { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_SG_STATUS_REG_3_CFG_CHG_TIME_SEC_MSB_M: u32 = GENMASK!(15, 0);
pub const ANA_SG_STATUS_REG_3_GATE_STATE: u32 = BIT!(16);
macro_rules! ANA_SG_STATUS_REG_3_IPS { ($x:expr) => { ((($x) << 20) & GENMASK!(23, 20)) }; }
pub const ANA_SG_STATUS_REG_3_IPS_M: u32 = GENMASK!(23, 20);
macro_rules! ANA_SG_STATUS_REG_3_IPS_X { ($x:expr) => { ((($x) & GENMASK!(23, 20)) >> 20) }; }
pub const ANA_SG_STATUS_REG_3_CONFIG_PENDING: u32 = BIT!(24);

pub const ANA_PORT_VLAN_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_VLAN_CFG_VLAN_VID_AS_ISDX: u32 = BIT!(21);
pub const ANA_PORT_VLAN_CFG_VLAN_AWARE_ENA: u32 = BIT!(20);
macro_rules! ANA_PORT_VLAN_CFG_VLAN_POP_CNT { ($x:expr) => { ((($x) << 18) & GENMASK!(19, 18)) }; }
pub const ANA_PORT_VLAN_CFG_VLAN_POP_CNT_M: u32 = GENMASK!(19, 18);
macro_rules! ANA_PORT_VLAN_CFG_VLAN_POP_CNT_X { ($x:expr) => { ((($x) & GENMASK!(19, 18)) >> 18) }; }
pub const ANA_PORT_VLAN_CFG_VLAN_INNER_TAG_ENA: u32 = BIT!(17);
pub const ANA_PORT_VLAN_CFG_VLAN_TAG_TYPE: u32 = BIT!(16);
pub const ANA_PORT_VLAN_CFG_VLAN_DEI: u32 = BIT!(15);
macro_rules! ANA_PORT_VLAN_CFG_VLAN_PCP { ($x:expr) => { ((($x) << 12) & GENMASK!(14, 12)) }; }
pub const ANA_PORT_VLAN_CFG_VLAN_PCP_M: u32 = GENMASK!(14, 12);
macro_rules! ANA_PORT_VLAN_CFG_VLAN_PCP_X { ($x:expr) => { ((($x) & GENMASK!(14, 12)) >> 12) }; }
macro_rules! ANA_PORT_VLAN_CFG_VLAN_VID { ($x:expr) => { (($x) & GENMASK!(11, 0)) }; }
pub const ANA_PORT_VLAN_CFG_VLAN_VID_M: u32 = GENMASK!(11, 0);

pub const ANA_PORT_DROP_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_DROP_CFG_DROP_UNTAGGED_ENA: u32 = BIT!(6);
pub const ANA_PORT_DROP_CFG_DROP_S_TAGGED_ENA: u32 = BIT!(5);
pub const ANA_PORT_DROP_CFG_DROP_C_TAGGED_ENA: u32 = BIT!(4);
pub const ANA_PORT_DROP_CFG_DROP_PRIO_S_TAGGED_ENA: u32 = BIT!(3);
pub const ANA_PORT_DROP_CFG_DROP_PRIO_C_TAGGED_ENA: u32 = BIT!(2);
pub const ANA_PORT_DROP_CFG_DROP_NULL_MAC_ENA: u32 = BIT!(1);
pub const ANA_PORT_DROP_CFG_DROP_MC_SMAC_ENA: u32 = BIT!(0);

pub const ANA_PORT_QOS_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_QOS_CFG_DP_DEFAULT_VAL: u32 = BIT!(8);
macro_rules! ANA_PORT_QOS_CFG_QOS_DEFAULT_VAL { ($x:expr) => { ((($x) << 5) & GENMASK!(7, 5)) }; }
pub const ANA_PORT_QOS_CFG_QOS_DEFAULT_VAL_M: u32 = GENMASK!(7, 5);
macro_rules! ANA_PORT_QOS_CFG_QOS_DEFAULT_VAL_X { ($x:expr) => { ((($x) & GENMASK!(7, 5)) >> 5) }; }
pub const ANA_PORT_QOS_CFG_QOS_DSCP_ENA: u32 = BIT!(4);
pub const ANA_PORT_QOS_CFG_QOS_PCP_ENA: u32 = BIT!(3);
pub const ANA_PORT_QOS_CFG_DSCP_TRANSLATE_ENA: u32 = BIT!(2);
macro_rules! ANA_PORT_QOS_CFG_DSCP_REWR_CFG { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_PORT_QOS_CFG_DSCP_REWR_CFG_M: u32 = GENMASK!(1, 0);

pub const ANA_PORT_VCAP_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_VCAP_CFG_S1_ENA: u32 = BIT!(14);
macro_rules! ANA_PORT_VCAP_CFG_S1_DMAC_DIP_ENA { ($x:expr) => { ((($x) << 11) & GENMASK!(13, 11)) }; }
pub const ANA_PORT_VCAP_CFG_S1_DMAC_DIP_ENA_M: u32 = GENMASK!(13, 11);
macro_rules! ANA_PORT_VCAP_CFG_S1_DMAC_DIP_ENA_X { ($x:expr) => { ((($x) & GENMASK!(13, 11)) >> 11) }; }
macro_rules! ANA_PORT_VCAP_CFG_S1_VLAN_INNER_TAG_ENA { ($x:expr) => { ((($x) << 8) & GENMASK!(10, 8)) }; }
pub const ANA_PORT_VCAP_CFG_S1_VLAN_INNER_TAG_ENA_M: u32 = GENMASK!(10, 8);
macro_rules! ANA_PORT_VCAP_CFG_S1_VLAN_INNER_TAG_ENA_X { ($x:expr) => { ((($x) & GENMASK!(10, 8)) >> 8) }; }
macro_rules! ANA_PORT_VCAP_CFG_PAG_VAL { ($x:expr) => { (($x) & GENMASK!(7, 0)) }; }
pub const ANA_PORT_VCAP_CFG_PAG_VAL_M: u32 = GENMASK!(7, 0);

pub const ANA_PORT_VCAP_S1_KEY_CFG_GSZ: u32 = 0x100;
pub const ANA_PORT_VCAP_S1_KEY_CFG_RSZ: u32 = 0x4;

macro_rules! ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_IP6_CFG { ($x:expr) => { ((($x) << 4) & GENMASK!(6, 4)) }; }
pub const ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_IP6_CFG_M: u32 = GENMASK!(6, 4);
macro_rules! ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_IP6_CFG_X { ($x:expr) => { ((($x) & GENMASK!(6, 4)) >> 4) }; }
macro_rules! ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_IP4_CFG { ($x:expr) => { ((($x) << 2) & GENMASK!(3, 2)) }; }
pub const ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_IP4_CFG_M: u32 = GENMASK!(3, 2);
macro_rules! ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_IP4_CFG_X { ($x:expr) => { ((($x) & GENMASK!(3, 2)) >> 2) }; }
macro_rules! ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_OTHER_CFG { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_PORT_VCAP_S1_KEY_CFG_S1_KEY_OTHER_CFG_M: u32 = GENMASK!(1, 0);

pub const ANA_PORT_VCAP_S2_CFG_GSZ: u32 = 0x100;

macro_rules! ANA_PORT_VCAP_S2_CFG_S2_UDP_PAYLOAD_ENA { ($x:expr) => { ((($x) << 17) & GENMASK!(18, 17)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_UDP_PAYLOAD_ENA_M: u32 = GENMASK!(18, 17);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_UDP_PAYLOAD_ENA_X { ($x:expr) => { ((($x) & GENMASK!(18, 17)) >> 17) }; }
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_ETYPE_PAYLOAD_ENA { ($x:expr) => { ((($x) << 15) & GENMASK!(16, 15)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_ETYPE_PAYLOAD_ENA_M: u32 = GENMASK!(16, 15);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_ETYPE_PAYLOAD_ENA_X { ($x:expr) => { ((($x) & GENMASK!(16, 15)) >> 15) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_ENA: u32 = BIT!(14);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_SNAP_DIS { ($x:expr) => { ((($x) << 12) & GENMASK!(13, 12)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_SNAP_DIS_M: u32 = GENMASK!(13, 12);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_SNAP_DIS_X { ($x:expr) => { ((($x) & GENMASK!(13, 12)) >> 12) }; }
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_ARP_DIS { ($x:expr) => { ((($x) << 10) & GENMASK!(11, 10)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_ARP_DIS_M: u32 = GENMASK!(11, 10);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_ARP_DIS_X { ($x:expr) => { ((($x) & GENMASK!(11, 10)) >> 10) }; }
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_IP_TCPUDP_DIS { ($x:expr) => { ((($x) << 8) & GENMASK!(9, 8)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_IP_TCPUDP_DIS_M: u32 = GENMASK!(9, 8);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_IP_TCPUDP_DIS_X { ($x:expr) => { ((($x) & GENMASK!(9, 8)) >> 8) }; }
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_IP_OTHER_DIS { ($x:expr) => { ((($x) << 6) & GENMASK!(7, 6)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_IP_OTHER_DIS_M: u32 = GENMASK!(7, 6);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_IP_OTHER_DIS_X { ($x:expr) => { ((($x) & GENMASK!(7, 6)) >> 6) }; }
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_IP6_CFG { ($x:expr) => { ((($x) << 2) & GENMASK!(5, 2)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_IP6_CFG_M: u32 = GENMASK!(5, 2);
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_IP6_CFG_X { ($x:expr) => { ((($x) & GENMASK!(5, 2)) >> 2) }; }
macro_rules! ANA_PORT_VCAP_S2_CFG_S2_OAM_DIS { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_PORT_VCAP_S2_CFG_S2_OAM_DIS_M: u32 = GENMASK!(1, 0);

pub const ANA_PORT_PCP_DEI_MAP_GSZ: u32 = 0x100;
pub const ANA_PORT_PCP_DEI_MAP_RSZ: u32 = 0x4;

pub const ANA_PORT_PCP_DEI_MAP_DP_PCP_DEI_VAL: u32 = BIT!(3);
macro_rules! ANA_PORT_PCP_DEI_MAP_QOS_PCP_DEI_VAL { ($x:expr) => { (($x) & GENMASK!(2, 0)) }; }
pub const ANA_PORT_PCP_DEI_MAP_QOS_PCP_DEI_VAL_M: u32 = GENMASK!(2, 0);

pub const ANA_PORT_CPU_FWD_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_CPU_FWD_CFG_CPU_VRAP_REDIR_ENA: u32 = BIT!(7);
pub const ANA_PORT_CPU_FWD_CFG_CPU_MLD_REDIR_ENA: u32 = BIT!(6);
pub const ANA_PORT_CPU_FWD_CFG_CPU_IGMP_REDIR_ENA: u32 = BIT!(5);
pub const ANA_PORT_CPU_FWD_CFG_CPU_IPMC_CTRL_COPY_ENA: u32 = BIT!(4);
pub const ANA_PORT_CPU_FWD_CFG_CPU_SRC_COPY_ENA: u32 = BIT!(3);
pub const ANA_PORT_CPU_FWD_CFG_CPU_ALLBRIDGE_DROP_ENA: u32 = BIT!(2);
pub const ANA_PORT_CPU_FWD_CFG_CPU_ALLBRIDGE_REDIR_ENA: u32 = BIT!(1);
pub const ANA_PORT_CPU_FWD_CFG_CPU_OAM_ENA: u32 = BIT!(0);

pub const ANA_PORT_CPU_FWD_BPDU_CFG_GSZ: u32 = 0x100;

macro_rules! ANA_PORT_CPU_FWD_BPDU_CFG_BPDU_DROP_ENA { ($x:expr) => { ((($x) << 16) & GENMASK!(31, 16)) }; }
pub const ANA_PORT_CPU_FWD_BPDU_CFG_BPDU_DROP_ENA_M: u32 = GENMASK!(31, 16);
macro_rules! ANA_PORT_CPU_FWD_BPDU_CFG_BPDU_DROP_ENA_X { ($x:expr) => { ((($x) & GENMASK!(31, 16)) >> 16) }; }
macro_rules! ANA_PORT_CPU_FWD_BPDU_CFG_BPDU_REDIR_ENA { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_PORT_CPU_FWD_BPDU_CFG_BPDU_REDIR_ENA_M: u32 = GENMASK!(15, 0);

pub const ANA_PORT_CPU_FWD_GARP_CFG_GSZ: u32 = 0x100;

macro_rules! ANA_PORT_CPU_FWD_GARP_CFG_GARP_DROP_ENA { ($x:expr) => { ((($x) << 16) & GENMASK!(31, 16)) }; }
pub const ANA_PORT_CPU_FWD_GARP_CFG_GARP_DROP_ENA_M: u32 = GENMASK!(31, 16);
macro_rules! ANA_PORT_CPU_FWD_GARP_CFG_GARP_DROP_ENA_X { ($x:expr) => { ((($x) & GENMASK!(31, 16)) >> 16) }; }
macro_rules! ANA_PORT_CPU_FWD_GARP_CFG_GARP_REDIR_ENA { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_PORT_CPU_FWD_GARP_CFG_GARP_REDIR_ENA_M: u32 = GENMASK!(15, 0);

pub const ANA_PORT_CPU_FWD_CCM_CFG_GSZ: u32 = 0x100;

macro_rules! ANA_PORT_CPU_FWD_CCM_CFG_CCM_DROP_ENA { ($x:expr) => { ((($x) << 16) & GENMASK!(31, 16)) }; }
pub const ANA_PORT_CPU_FWD_CCM_CFG_CCM_DROP_ENA_M: u32 = GENMASK!(31, 16);
macro_rules! ANA_PORT_CPU_FWD_CCM_CFG_CCM_DROP_ENA_X { ($x:expr) => { ((($x) & GENMASK!(31, 16)) >> 16) }; }
macro_rules! ANA_PORT_CPU_FWD_CCM_CFG_CCM_REDIR_ENA { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_PORT_CPU_FWD_CCM_CFG_CCM_REDIR_ENA_M: u32 = GENMASK!(15, 0);

pub const ANA_PORT_PORT_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_PORT_CFG_SRC_MIRROR_ENA: u32 = BIT!(15);
pub const ANA_PORT_PORT_CFG_LIMIT_DROP: u32 = BIT!(14);
pub const ANA_PORT_PORT_CFG_LIMIT_CPU: u32 = BIT!(13);
pub const ANA_PORT_PORT_CFG_LOCKED_PORTMOVE_DROP: u32 = BIT!(12);
pub const ANA_PORT_PORT_CFG_LOCKED_PORTMOVE_CPU: u32 = BIT!(11);
pub const ANA_PORT_PORT_CFG_LEARNDROP: u32 = BIT!(10);
pub const ANA_PORT_PORT_CFG_LEARNCPU: u32 = BIT!(9);
pub const ANA_PORT_PORT_CFG_LEARNAUTO: u32 = BIT!(8);
pub const ANA_PORT_PORT_CFG_LEARN_ENA: u32 = BIT!(7);
pub const ANA_PORT_PORT_CFG_RECV_ENA: u32 = BIT!(6);
macro_rules! ANA_PORT_PORT_CFG_PORTID_VAL { ($x:expr) => { ((($x) << 2) & GENMASK!(5, 2)) }; }
pub const ANA_PORT_PORT_CFG_PORTID_VAL_M: u32 = GENMASK!(5, 2);
macro_rules! ANA_PORT_PORT_CFG_PORTID_VAL_X { ($x:expr) => { ((($x) & GENMASK!(5, 2)) >> 2) }; }
pub const ANA_PORT_PORT_CFG_USE_B_DOM_TBL: u32 = BIT!(1);
pub const ANA_PORT_PORT_CFG_LSR_MODE: u32 = BIT!(0);

pub const ANA_PORT_POL_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_POL_CFG_POL_CPU_REDIR_8021: u32 = BIT!(19);
pub const ANA_PORT_POL_CFG_POL_CPU_REDIR_IP: u32 = BIT!(18);
pub const ANA_PORT_POL_CFG_PORT_POL_ENA: u32 = BIT!(17);
macro_rules! ANA_PORT_POL_CFG_QUEUE_POL_ENA { ($x:expr) => { ((($x) << 9) & GENMASK!(16, 9)) }; }
pub const ANA_PORT_POL_CFG_QUEUE_POL_ENA_M: u32 = GENMASK!(16, 9);
macro_rules! ANA_PORT_POL_CFG_QUEUE_POL_ENA_X { ($x:expr) => { ((($x) & GENMASK!(16, 9)) >> 9) }; }
macro_rules! ANA_PORT_POL_CFG_POL_ORDER { ($x:expr) => { (($x) & GENMASK!(8, 0)) }; }
pub const ANA_PORT_POL_CFG_POL_ORDER_M: u32 = GENMASK!(8, 0);

pub const ANA_PORT_PTP_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_PTP_CFG_PTP_BACKPLANE_MODE: u32 = BIT!(0);

pub const ANA_PORT_PTP_DLY1_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_PTP_DLY2_CFG_GSZ: u32 = 0x100;

pub const ANA_PORT_SFID_CFG_GSZ: u32 = 0x100;
pub const ANA_PORT_SFID_CFG_RSZ: u32 = 0x4;

pub const ANA_PORT_SFID_CFG_SFID_VALID: u32 = BIT!(8);
macro_rules! ANA_PORT_SFID_CFG_SFID { ($x:expr) => { (($x) & GENMASK!(7, 0)) }; }
pub const ANA_PORT_SFID_CFG_SFID_M: u32 = GENMASK!(7, 0);

pub const ANA_PFC_PFC_CFG_GSZ: u32 = 0x40;

macro_rules! ANA_PFC_PFC_CFG_RX_PFC_ENA { ($x:expr) => { ((($x) << 2) & GENMASK!(9, 2)) }; }
pub const ANA_PFC_PFC_CFG_RX_PFC_ENA_M: u32 = GENMASK!(9, 2);
macro_rules! ANA_PFC_PFC_CFG_RX_PFC_ENA_X { ($x:expr) => { ((($x) & GENMASK!(9, 2)) >> 2) }; }
macro_rules! ANA_PFC_PFC_CFG_FC_LINK_SPEED { ($x:expr) => { (($x) & GENMASK!(1, 0)) }; }
pub const ANA_PFC_PFC_CFG_FC_LINK_SPEED_M: u32 = GENMASK!(1, 0);

pub const ANA_PFC_PFC_TIMER_GSZ: u32 = 0x40;
pub const ANA_PFC_PFC_TIMER_RSZ: u32 = 0x4;

pub const ANA_IPT_OAM_MEP_CFG_GSZ: u32 = 0x8;

macro_rules! ANA_IPT_OAM_MEP_CFG_MEP_IDX_P { ($x:expr) => { ((($x) << 6) & GENMASK!(10, 6)) }; }
pub const ANA_IPT_OAM_MEP_CFG_MEP_IDX_P_M: u32 = GENMASK!(10, 6);
macro_rules! ANA_IPT_OAM_MEP_CFG_MEP_IDX_P_X { ($x:expr) => { ((($x) & GENMASK!(10, 6)) >> 6) }; }
macro_rules! ANA_IPT_OAM_MEP_CFG_MEP_IDX { ($x:expr) => { ((($x) << 1) & GENMASK!(5, 1)) }; }
pub const ANA_IPT_OAM_MEP_CFG_MEP_IDX_M: u32 = GENMASK!(5, 1);
macro_rules! ANA_IPT_OAM_MEP_CFG_MEP_IDX_X { ($x:expr) => { ((($x) & GENMASK!(5, 1)) >> 1) }; }
pub const ANA_IPT_OAM_MEP_CFG_MEP_IDX_ENA: u32 = BIT!(0);

pub const ANA_IPT_IPT_GSZ: u32 = 0x8;

macro_rules! ANA_IPT_IPT_IPT_CFG { ($x:expr) => { ((($x) << 15) & GENMASK!(16, 15)) }; }
pub const ANA_IPT_IPT_IPT_CFG_M: u32 = GENMASK!(16, 15);
macro_rules! ANA_IPT_IPT_IPT_CFG_X { ($x:expr) => { ((($x) & GENMASK!(16, 15)) >> 15) }; }
macro_rules! ANA_IPT_IPT_ISDX_P { ($x:expr) => { ((($x) << 7) & GENMASK!(14, 7)) }; }
pub const ANA_IPT_IPT_ISDX_P_M: u32 = GENMASK!(14, 7);
macro_rules! ANA_IPT_IPT_ISDX_P_X { ($x:expr) => { ((($x) & GENMASK!(14, 7)) >> 7) }; }
macro_rules! ANA_IPT_IPT_PPT_IDX { ($x:expr) => { (($x) & GENMASK!(6, 0)) }; }
pub const ANA_IPT_IPT_PPT_IDX_M: u32 = GENMASK!(6, 0);

pub const ANA_PPT_PPT_RSZ: u32 = 0x4;

pub const ANA_FID_MAP_FID_MAP_RSZ: u32 = 0x4;

macro_rules! ANA_FID_MAP_FID_MAP_FID_C_VAL { ($x:expr) => { ((($x) << 6) & GENMASK!(11, 6)) }; }
pub const ANA_FID_MAP_FID_MAP_FID_C_VAL_M: u32 = GENMASK!(11, 6);
macro_rules! ANA_FID_MAP_FID_MAP_FID_C_VAL_X { ($x:expr) => { ((($x) & GENMASK!(11, 6)) >> 6) }; }
macro_rules! ANA_FID_MAP_FID_MAP_FID_B_VAL { ($x:expr) => { (($x) & GENMASK!(5, 0)) }; }
pub const ANA_FID_MAP_FID_MAP_FID_B_VAL_M: u32 = GENMASK!(5, 0);

pub const ANA_AGGR_CFG_AC_RND_ENA: u32 = BIT!(7);
pub const ANA_AGGR_CFG_AC_DMAC_ENA: u32 = BIT!(6);
pub const ANA_AGGR_CFG_AC_SMAC_ENA: u32 = BIT!(5);
pub const ANA_AGGR_CFG_AC_IP6_FLOW_LBL_ENA: u32 = BIT!(4);
pub const ANA_AGGR_CFG_AC_IP6_TCPUDP_ENA: u32 = BIT!(3);
pub const ANA_AGGR_CFG_AC_IP4_SIPDIP_ENA: u32 = BIT!(2);
pub const ANA_AGGR_CFG_AC_IP4_TCPUDP_ENA: u32 = BIT!(1);
pub const ANA_AGGR_CFG_AC_ISDX_ENA: u32 = BIT!(0);

macro_rules! ANA_CPUQ_CFG_CPUQ_MLD { ($x:expr) => { ((($x) << 27) & GENMASK!(29, 27)) }; }
pub const ANA_CPUQ_CFG_CPUQ_MLD_M: u32 = GENMASK!(29, 27);
macro_rules! ANA_CPUQ_CFG_CPUQ_MLD_X { ($x:expr) => { ((($x) & GENMASK!(29, 27)) >> 27) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_IGMP { ($x:expr) => { ((($x) << 24) & GENMASK!(26, 24)) }; }
pub const ANA_CPUQ_CFG_CPUQ_IGMP_M: u32 = GENMASK!(26, 24);
macro_rules! ANA_CPUQ_CFG_CPUQ_IGMP_X { ($x:expr) => { ((($x) & GENMASK!(26, 24)) >> 24) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_IPMC_CTRL { ($x:expr) => { ((($x) << 21) & GENMASK!(23, 21)) }; }
pub const ANA_CPUQ_CFG_CPUQ_IPMC_CTRL_M: u32 = GENMASK!(23, 21);
macro_rules! ANA_CPUQ_CFG_CPUQ_IPMC_CTRL_X { ($x:expr) => { ((($x) & GENMASK!(23, 21)) >> 21) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_ALLBRIDGE { ($x:expr) => { ((($x) << 18) & GENMASK!(20, 18)) }; }
pub const ANA_CPUQ_CFG_CPUQ_ALLBRIDGE_M: u32 = GENMASK!(20, 18);
macro_rules! ANA_CPUQ_CFG_CPUQ_ALLBRIDGE_X { ($x:expr) => { ((($x) & GENMASK!(20, 18)) >> 18) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_LOCKED_PORTMOVE { ($x:expr) => { ((($x) << 15) & GENMASK!(17, 15)) }; }
pub const ANA_CPUQ_CFG_CPUQ_LOCKED_PORTMOVE_M: u32 = GENMASK!(17, 15);
macro_rules! ANA_CPUQ_CFG_CPUQ_LOCKED_PORTMOVE_X { ($x:expr) => { ((($x) & GENMASK!(17, 15)) >> 15) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_SRC_COPY { ($x:expr) => { ((($x) << 12) & GENMASK!(14, 12)) }; }
pub const ANA_CPUQ_CFG_CPUQ_SRC_COPY_M: u32 = GENMASK!(14, 12);
macro_rules! ANA_CPUQ_CFG_CPUQ_SRC_COPY_X { ($x:expr) => { ((($x) & GENMASK!(14, 12)) >> 12) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_MAC_COPY { ($x:expr) => { ((($x) << 9) & GENMASK!(11, 9)) }; }
pub const ANA_CPUQ_CFG_CPUQ_MAC_COPY_M: u32 = GENMASK!(11, 9);
macro_rules! ANA_CPUQ_CFG_CPUQ_MAC_COPY_X { ($x:expr) => { ((($x) & GENMASK!(11, 9)) >> 9) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_LRN { ($x:expr) => { ((($x) << 6) & GENMASK!(8, 6)) }; }
pub const ANA_CPUQ_CFG_CPUQ_LRN_M: u32 = GENMASK!(8, 6);
macro_rules! ANA_CPUQ_CFG_CPUQ_LRN_X { ($x:expr) => { ((($x) & GENMASK!(8, 6)) >> 6) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_MIRROR { ($x:expr) => { ((($x) << 3) & GENMASK!(5, 3)) }; }
pub const ANA_CPUQ_CFG_CPUQ_MIRROR_M: u32 = GENMASK!(5, 3);
macro_rules! ANA_CPUQ_CFG_CPUQ_MIRROR_X { ($x:expr) => { ((($x) & GENMASK!(5, 3)) >> 3) }; }
macro_rules! ANA_CPUQ_CFG_CPUQ_SFLOW { ($x:expr) => { (($x) & GENMASK!(2, 0)) }; }
pub const ANA_CPUQ_CFG_CPUQ_SFLOW_M: u32 = GENMASK!(2, 0);

pub const ANA_CPUQ_8021_CFG_RSZ: u32 = 0x4;

macro_rules! ANA_CPUQ_8021_CFG_CPUQ_BPDU_VAL { ($x:expr) => { ((($x) << 6) & GENMASK!(8, 6)) }; }
pub const ANA_CPUQ_8021_CFG_CPUQ_BPDU_VAL_M: u32 = GENMASK!(8, 6);
macro_rules! ANA_CPUQ_8021_CFG_CPUQ_BPDU_VAL_X { ($x:expr) => { ((($x) & GENMASK!(8, 6)) >> 6) }; }
macro_rules! ANA_CPUQ_8021_CFG_CPUQ_GARP_VAL { ($x:expr) => { ((($x) << 3) & GENMASK!(5, 3)) }; }
pub const ANA_CPUQ_8021_CFG_CPUQ_GARP_VAL_M: u32 = GENMASK!(5, 3);
macro_rules! ANA_CPUQ_8021_CFG_CPUQ_GARP_VAL_X { ($x:expr) => { ((($x) & GENMASK!(5, 3)) >> 3) }; }
macro_rules! ANA_CPUQ_8021_CFG_CPUQ_CCM_VAL { ($x:expr) => { (($x) & GENMASK!(2, 0)) }; }
pub const ANA_CPUQ_8021_CFG_CPUQ_CCM_VAL_M: u32 = GENMASK!(2, 0);

pub const ANA_DSCP_CFG_RSZ: u32 = 0x4;

pub const ANA_DSCP_CFG_DP_DSCP_VAL: u32 = BIT!(11);
macro_rules! ANA_DSCP_CFG_QOS_DSCP_VAL { ($x:expr) => { ((($x) << 8) & GENMASK!(10, 8)) }; }
pub const ANA_DSCP_CFG_QOS_DSCP_VAL_M: u32 = GENMASK!(10, 8);
macro_rules! ANA_DSCP_CFG_QOS_DSCP_VAL_X { ($x:expr) => { ((($x) & GENMASK!(10, 8)) >> 8) }; }
macro_rules! ANA_DSCP_CFG_DSCP_TRANSLATE_VAL { ($x:expr) => { ((($x) << 2) & GENMASK!(7, 2)) }; }
pub const ANA_DSCP_CFG_DSCP_TRANSLATE_VAL_M: u32 = GENMASK!(7, 2);
macro_rules! ANA_DSCP_CFG_DSCP_TRANSLATE_VAL_X { ($x:expr) => { ((($x) & GENMASK!(7, 2)) >> 2) }; }
pub const ANA_DSCP_CFG_DSCP_TRUST_ENA: u32 = BIT!(1);
pub const ANA_DSCP_CFG_DSCP_REWR_ENA: u32 = BIT!(0);

pub const ANA_DSCP_REWR_CFG_RSZ: u32 = 0x4;

pub const ANA_VCAP_RNG_TYPE_CFG_RSZ: u32 = 0x4;

pub const ANA_VCAP_RNG_VAL_CFG_RSZ: u32 = 0x4;

macro_rules! ANA_VCAP_RNG_VAL_CFG_VCAP_RNG_MIN_VAL { ($x:expr) => { ((($x) << 16) & GENMASK!(31, 16)) }; }
pub const ANA_VCAP_RNG_VAL_CFG_VCAP_RNG_MIN_VAL_M: u32 = GENMASK!(31, 16);
macro_rules! ANA_VCAP_RNG_VAL_CFG_VCAP_RNG_MIN_VAL_X { ($x:expr) => { ((($x) & GENMASK!(31, 16)) >> 16) }; }
macro_rules! ANA_VCAP_RNG_VAL_CFG_VCAP_RNG_MAX_VAL { ($x:expr) => { (($x) & GENMASK!(15, 0)) }; }
pub const ANA_VCAP_RNG_VAL_CFG_VCAP_RNG_MAX_VAL_M: u32 = GENMASK!(15, 0);

pub const ANA_VRAP_CFG_VRAP_VLAN_AWARE_ENA: u32 = BIT!(12);
macro_rules! ANA_VRAP_CFG_VRAP_VID { ($x:expr) => { (($x) & GENMASK!(11, 0)) }; }
pub const ANA_VRAP_CFG_VRAP_VID_M: u32 = GENMASK!(11, 0);

pub const ANA_DISCARD_CFG_DROP_TAGGING_ISDX0: u32 = BIT!(3);
pub const ANA_DISCARD_CFG_DROP_CTRLPROT_ISDX0: u32 = BIT!(2);
pub const ANA_DISCARD_CFG_DROP_TAGGING_S2_ENA: u32 = BIT!(1);
pub const ANA_DISCARD_CFG_DROP_CTRLPROT_S2_ENA: u32 = BIT!(0);

pub const ANA_FID_CFG_VID_MC_ENA: u32 = BIT!(0);

pub const ANA_POL_PIR_CFG_GSZ: u32 = 0x20;

macro_rules! ANA_POL_PIR_CFG_PIR_RATE { ($x:expr) => { ((($x) << 6) & GENMASK!(20, 6)) }; }
pub const ANA_POL_PIR_CFG_PIR_RATE_M: u32 = GENMASK!(20, 6);
macro_rules! ANA_POL_PIR_CFG_PIR_RATE_X { ($x:expr) => { ((($x) & GENMASK!(20, 6)) >> 6) }; }
macro_rules! ANA_POL_PIR_CFG_PIR_BURST { ($x:expr) => { (($x) & GENMASK!(5, 0)) }; }
pub const ANA_POL_PIR_CFG_PIR_BURST_M: u32 = GENMASK!(5, 0);

pub const ANA_POL_CIR_CFG_GSZ: u32 = 0x20;

macro_rules! ANA_POL_CIR_CFG_CIR_RATE { ($x:expr) => { ((($x) << 6) & GENMASK!(20, 6)) }; }
pub const ANA_POL_CIR_CFG_CIR_RATE_M: u32 = GENMASK!(20, 6);
macro_rules! ANA_POL_CIR_CFG_CIR_RATE_X { ($x:expr) => { ((($x) & GENMASK!(20, 6)) >> 6) }; }
macro_rules! ANA_POL_CIR_CFG_CIR_BURST { ($x:expr) => { (($x) & GENMASK!(5, 0)) }; }
pub const ANA_POL_CIR_CFG_CIR_BURST_M: u32 = GENMASK!(5, 0);

pub const ANA_POL_MODE_CFG_GSZ: u32 = 0x20;

macro_rules! ANA_POL_MODE_CFG_IPG_SIZE { ($x:expr) => { ((($x) << 5) & GENMASK!(9, 5)) }; }
pub const ANA_POL_MODE_CFG_IPG_SIZE_M: u32 = GENMASK!(9, 5);
macro_rules! ANA_POL_MODE_CFG_IPG_SIZE_X { ($x:expr) => { ((($x) & GENMASK!(9, 5)) >> 5) }; }
macro_rules! ANA_POL_MODE_CFG_FRM_MODE { ($x:expr) => { ((($x) << 3) & GENMASK!(4, 3)) }; }
pub const ANA_POL_MODE_CFG_FRM_MODE_M: u32 = GENMASK!(4, 3);
macro_rules! ANA_POL_MODE_CFG_FRM_MODE_X { ($x:expr) => { ((($x) & GENMASK!(4, 3)) >> 3) }; }
pub const ANA_POL_MODE_CFG_DLB_COUPLED: u32 = BIT!(2);
pub const ANA_POL_MODE_CFG_CIR_ENA: u32 = BIT!(1);
pub const ANA_POL_MODE_CFG_OVERSHOOT_ENA: u32 = BIT!(0);

pub const ANA_POL_PIR_STATE_GSZ: u32 = 0x20;

pub const ANA_POL_CIR_STATE_GSZ: u32 = 0x20;

pub const ANA_POL_STATE_GSZ: u32 = 0x20;

pub const ANA_POL_FLOWC_RSZ: u32 = 0x4;

pub const ANA_POL_FLOWC_POL_FLOWC: u32 = BIT!(0);

macro_rules! ANA_POL_HYST_POL_FC_HYST { ($x:expr) => { ((($x) << 4) & GENMASK!(9, 4)) }; }
pub const ANA_POL_HYST_POL_FC_HYST_M: u32 = GENMASK!(9, 4);
macro_rules! ANA_POL_HYST_POL_FC_HYST_X { ($x:expr) => { ((($x) & GENMASK!(9, 4)) >> 4) }; }
macro_rules! ANA_POL_HYST_POL_STOP_HYST { ($x:expr) => { (($x) & GENMASK!(3, 0)) }; }
pub const ANA_POL_HYST_POL_STOP_HYST_M: u32 = GENMASK!(3, 0);

pub const ANA_POL_MISC_CFG_POL_CLOSE_ALL: u32 = BIT!(1);
pub const ANA_POL_MISC_CFG_POL_LEAK_DIS: u32 = BIT!(0);



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
