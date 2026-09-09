/* SPDX-License-Identifier: GPL-2.0 */
/* Z8536 CIO Internal registers */

// C header guard omitted; this file is intended to be included once.
const fn BIT(n: u32) -> u32 { 1u32 << n }

pub const Z8536_INT_CTRL_REG: u32 = 0x00;
pub const Z8536_INT_CTRL_MIE: u32 = BIT(7); /* Master Interrupt Enable */
pub const Z8536_INT_CTRL_DLC: u32 = BIT(6); /* Disable Lower Chain */
pub const Z8536_INT_CTRL_NV: u32 = BIT(5); /* No Vector */
pub const Z8536_INT_CTRL_PA_VIS: u32 = BIT(4); /* Port A Vect Inc Status */
pub const Z8536_INT_CTRL_PB_VIS: u32 = BIT(3); /* Port B Vect Inc Status */
pub const Z8536_INT_CTRL_VT_VIS: u32 = BIT(2); /* C/T Vect Inc Status */
pub const Z8536_INT_CTRL_RJA: u32 = BIT(1); /* Right Justified Addresses */
pub const Z8536_INT_CTRL_RESET: u32 = BIT(0); /* Reset */

pub const Z8536_CFG_CTRL_REG: u32 = 0x01;
pub const Z8536_CFG_CTRL_PBE: u32 = BIT(7); /* Port B Enable */
pub const Z8536_CFG_CTRL_CT1E: u32 = BIT(6); /* C/T 1 Enable */
pub const Z8536_CFG_CTRL_CT2E: u32 = BIT(5); /* C/T 2 Enable */
pub const Z8536_CFG_CTRL_PCE_CT3E: u32 = BIT(4); /* Port C & C/T 3 Enable */
pub const Z8536_CFG_CTRL_PLC: u32 = BIT(3); /* Port A/B Link Control */
pub const Z8536_CFG_CTRL_PAE: u32 = BIT(2); /* Port A Enable */
pub const fn Z8536_CFG_CTRL_LC(x: u32) -> u32 { (x & 0x3) << 0 } /* Link Control */
pub const Z8536_CFG_CTRL_LC_INDEP: u32 = Z8536_CFG_CTRL_LC(0); /* Independent */
pub const Z8536_CFG_CTRL_LC_GATE: u32 = Z8536_CFG_CTRL_LC(1); /* 1 Gates 2 */
pub const Z8536_CFG_CTRL_LC_TRIG: u32 = Z8536_CFG_CTRL_LC(2); /* 1 Triggers 2 */
pub const Z8536_CFG_CTRL_LC_CLK: u32 = Z8536_CFG_CTRL_LC(3); /* 1 Clocks 2 */
pub const Z8536_CFG_CTRL_LC_MASK: u32 = Z8536_CFG_CTRL_LC(3);

pub const Z8536_PA_INT_VECT_REG: u32 = 0x02;
pub const Z8536_PB_INT_VECT_REG: u32 = 0x03;
pub const Z8536_CT_INT_VECT_REG: u32 = 0x04;
pub const Z8536_CURR_INT_VECT_REG: u32 = 0x1f;

pub const Z8536_PA_CMDSTAT_REG: u32 = 0x08;
pub const Z8536_PB_CMDSTAT_REG: u32 = 0x09;
pub const Z8536_CT1_CMDSTAT_REG: u32 = 0x0a;
pub const Z8536_CT2_CMDSTAT_REG: u32 = 0x0b;
pub const Z8536_CT3_CMDSTAT_REG: u32 = 0x0c;
pub const fn Z8536_CT_CMDSTAT_REG(x: u32) -> u32 { 0x0a + x }
pub const fn Z8536_CMD(x: u32) -> u32 { (x & 0x7) << 5 }
pub const Z8536_CMD_NULL: u32 = Z8536_CMD(0); /* Null Code */
pub const Z8536_CMD_CLR_IP_IUS: u32 = Z8536_CMD(1); /* Clear IP & IUS */
pub const Z8536_CMD_SET_IUS: u32 = Z8536_CMD(2); /* Set IUS */
pub const Z8536_CMD_CLR_IUS: u32 = Z8536_CMD(3); /* Clear IUS */
pub const Z8536_CMD_SET_IP: u32 = Z8536_CMD(4); /* Set IP */
pub const Z8536_CMD_CLR_IP: u32 = Z8536_CMD(5); /* Clear IP */
pub const Z8536_CMD_SET_IE: u32 = Z8536_CMD(6); /* Set IE */
pub const Z8536_CMD_CLR_IE: u32 = Z8536_CMD(7); /* Clear IE */
pub const Z8536_CMD_MASK: u32 = Z8536_CMD(7);
pub const Z8536_STAT_IUS: u32 = BIT(7); /* Interrupt Under Service */
pub const Z8536_STAT_IE: u32 = BIT(6); /* Interrupt Enable */
pub const Z8536_STAT_IP: u32 = BIT(5); /* Interrupt Pending */
pub const Z8536_STAT_ERR: u32 = BIT(4); /* Interrupt Error */
pub const Z8536_STAT_IE_IP: u32 = Z8536_STAT_IE | Z8536_STAT_IP;
pub const Z8536_PAB_STAT_ORE: u32 = BIT(3); /* Output Register Empty */
pub const Z8536_PAB_STAT_IRF: u32 = BIT(2); /* Input Register Full */
pub const Z8536_PAB_STAT_PMF: u32 = BIT(1); /* Pattern Match Flag */
pub const Z8536_PAB_CMDSTAT_IOE: u32 = BIT(0); /* Interrupt On Error */
pub const Z8536_CT_CMD_RCC: u32 = BIT(3); /* Read Counter Control */
pub const Z8536_CT_CMDSTAT_GCB: u32 = BIT(2); /* Gate Command Bit */
pub const Z8536_CT_CMD_TCB: u32 = BIT(1); /* Trigger Command Bit */
pub const Z8536_CT_STAT_CIP: u32 = BIT(0); /* Count In Progress */

pub const Z8536_PA_DATA_REG: u32 = 0x0d;
pub const Z8536_PB_DATA_REG: u32 = 0x0e;
pub const Z8536_PC_DATA_REG: u32 = 0x0f;

pub const Z8536_CT1_VAL_MSB_REG: u32 = 0x10;
pub const Z8536_CT1_VAL_LSB_REG: u32 = 0x11;
pub const Z8536_CT2_VAL_MSB_REG: u32 = 0x12;
pub const Z8536_CT2_VAL_LSB_REG: u32 = 0x13;
pub const Z8536_CT3_VAL_MSB_REG: u32 = 0x14;
pub const Z8536_CT3_VAL_LSB_REG: u32 = 0x15;
pub const fn Z8536_CT_VAL_MSB_REG(x: u32) -> u32 { 0x10 + x * 2 }
pub const fn Z8536_CT_VAL_LSB_REG(x: u32) -> u32 { 0x11 + x * 2 }
pub const Z8536_CT1_RELOAD_MSB_REG: u32 = 0x16;
pub const Z8536_CT1_RELOAD_LSB_REG: u32 = 0x17;
pub const Z8536_CT2_RELOAD_MSB_REG: u32 = 0x18;
pub const Z8536_CT2_RELOAD_LSB_REG: u32 = 0x19;
pub const Z8536_CT3_RELOAD_MSB_REG: u32 = 0x1a;
pub const Z8536_CT3_RELOAD_LSB_REG: u32 = 0x1b;
pub const fn Z8536_CT_RELOAD_MSB_REG(x: u32) -> u32 { 0x16 + x * 2 }
pub const fn Z8536_CT_RELOAD_LSB_REG(x: u32) -> u32 { 0x17 + x * 2 }
pub const Z8536_CT1_MODE_REG: u32 = 0x1c;
pub const Z8536_CT2_MODE_REG: u32 = 0x1d;
pub const Z8536_CT3_MODE_REG: u32 = 0x1e;
pub const fn Z8536_CT_MODE_REG(x: u32) -> u32 { 0x1c + x }
pub const Z8536_CT_MODE_CSC: u32 = BIT(7); /* Continuous/Single Cycle */
pub const Z8536_CT_MODE_EOE: u32 = BIT(6); /* External Output Enable */
pub const Z8536_CT_MODE_ECE: u32 = BIT(5); /* External Count Enable */
pub const Z8536_CT_MODE_ETE: u32 = BIT(4); /* External Trigger Enable */
pub const Z8536_CT_MODE_EGE: u32 = BIT(3); /* External Gate Enable */
pub const Z8536_CT_MODE_REB: u32 = BIT(2); /* Retrigger Enable Bit */
pub const fn Z8536_CT_MODE_DCS(x: u32) -> u32 { (x & 0x3) << 0 }
pub const Z8536_CT_MODE_DCS_PULSE: u32 = Z8536_CT_MODE_DCS(0); /* Pulse */
pub const Z8536_CT_MODE_DCS_ONESHOT: u32 = Z8536_CT_MODE_DCS(1); /* One-Shot */
pub const Z8536_CT_MODE_DCS_SQRWAVE: u32 = Z8536_CT_MODE_DCS(2); /* Square Wave */
pub const Z8536_CT_MODE_DCS_DO_NOT_USE: u32 = Z8536_CT_MODE_DCS(3); /* Do Not Use */
pub const Z8536_CT_MODE_DCS_MASK: u32 = Z8536_CT_MODE_DCS(3);

pub const Z8536_PA_MODE_REG: u32 = 0x20;
pub const Z8536_PB_MODE_REG: u32 = 0x28;
pub const fn Z8536_PAB_MODE_PTS(x: u32) -> u32 { (x & 0x3) << 6 }
pub const Z8536_PAB_MODE_PTS_BIT: u32 = Z8536_PAB_MODE_PTS(0 << 6); /* Bit */
pub const Z8536_PAB_MODE_PTS_INPUT: u32 = Z8536_PAB_MODE_PTS(1 << 6); /* Input */
pub const Z8536_PAB_MODE_PTS_OUTPUT: u32 = Z8536_PAB_MODE_PTS(2 << 6); /* Output */
pub const Z8536_PAB_MODE_PTS_BIDIR: u32 = Z8536_PAB_MODE_PTS(3 << 6); /* Bidir */
pub const Z8536_PAB_MODE_PTS_MASK: u32 = Z8536_PAB_MODE_PTS(3 << 6);
pub const Z8536_PAB_MODE_ITB: u32 = BIT(5); /* Interrupt on Two Bytes */
pub const Z8536_PAB_MODE_SB: u32 = BIT(4); /* Single Buffered mode */
pub const Z8536_PAB_MODE_IMO: u32 = BIT(3); /* Interrupt on Match Only */
pub const fn Z8536_PAB_MODE_PMS(x: u32) -> u32 { (x & 0x3) << 1 }
pub const Z8536_PAB_MODE_PMS_DISABLE: u32 = Z8536_PAB_MODE_PMS(0); /* Disabled */
pub const Z8536_PAB_MODE_PMS_AND: u32 = Z8536_PAB_MODE_PMS(1); /* "AND" */
pub const Z8536_PAB_MODE_PMS_OR: u32 = Z8536_PAB_MODE_PMS(2); /* "OR" */
pub const Z8536_PAB_MODE_PMS_OR_PEV: u32 = Z8536_PAB_MODE_PMS(3); /* "OR-Priority" */
pub const Z8536_PAB_MODE_PMS_MASK: u32 = Z8536_PAB_MODE_PMS(3);
pub const Z8536_PAB_MODE_LPM: u32 = BIT(0); /* Latch on Pattern Match */
pub const Z8536_PAB_MODE_DTE: u32 = BIT(0); /* Deskew Timer Enabled */

pub const Z8536_PA_HANDSHAKE_REG: u32 = 0x21;
pub const Z8536_PB_HANDSHAKE_REG: u32 = 0x29;
pub const fn Z8536_PAB_HANDSHAKE_HST(x: u32) -> u32 { (x & 0x3) << 6 }
pub const Z8536_PAB_HANDSHAKE_HST_INTER: u32 = Z8536_PAB_HANDSHAKE_HST(0); /* Interlock */
pub const Z8536_PAB_HANDSHAKE_HST_STROBED: u32 = Z8536_PAB_HANDSHAKE_HST(1); /* Strobed */
pub const Z8536_PAB_HANDSHAKE_HST_PULSED: u32 = Z8536_PAB_HANDSHAKE_HST(2); /* Pulsed */
pub const Z8536_PAB_HANDSHAKE_HST_3WIRE: u32 = Z8536_PAB_HANDSHAKE_HST(3); /* 3-Wire */
pub const Z8536_PAB_HANDSHAKE_HST_MASK: u32 = Z8536_PAB_HANDSHAKE_HST(3);
pub const fn Z8536_PAB_HANDSHAKE_RWS(x: u32) -> u32 { (x & 0x7) << 3 }
pub const Z8536_PAB_HANDSHAKE_RWS_DISABLE: u32 = Z8536_PAB_HANDSHAKE_RWS(0); /* Disabled */
pub const Z8536_PAB_HANDSHAKE_RWS_OUTWAIT: u32 = Z8536_PAB_HANDSHAKE_RWS(1); /* Out Wait */
pub const Z8536_PAB_HANDSHAKE_RWS_INWAIT: u32 = Z8536_PAB_HANDSHAKE_RWS(3); /* In Wait */
pub const Z8536_PAB_HANDSHAKE_RWS_SPREQ: u32 = Z8536_PAB_HANDSHAKE_RWS(4); /* Special */
pub const Z8536_PAB_HANDSHAKE_RWS_OUTREQ: u32 = Z8536_PAB_HANDSHAKE_RWS(5); /* Out Req */
pub const Z8536_PAB_HANDSHAKE_RWS_INREQ: u32 = Z8536_PAB_HANDSHAKE_RWS(7); /* In Req */
pub const Z8536_PAB_HANDSHAKE_RWS_MASK: u32 = Z8536_PAB_HANDSHAKE_RWS(7);
pub const fn Z8536_PAB_HANDSHAKE_DESKEW(x: u32) -> u32 { x << 0 }
pub const Z8536_PAB_HANDSHAKE_DESKEW_MASK: u32 = 3 << 0;

/* Port A/B/C Data Path Polarity registers */
pub const Z8536_PA_DPP_REG: u32 = 0x22;
pub const Z8536_PB_DPP_REG: u32 = 0x2a;
pub const Z8536_PC_DPP_REG: u32 = 0x05;
/* Port A/B/C Data Direction registers */
pub const Z8536_PA_DD_REG: u32 = 0x23;
pub const Z8536_PB_DD_REG: u32 = 0x2b;
pub const Z8536_PC_DD_REG: u32 = 0x06;
/* Port A/B/C Special I/O Control registers */
pub const Z8536_PA_SIO_REG: u32 = 0x24;
pub const Z8536_PB_SIO_REG: u32 = 0x2c;
pub const Z8536_PC_SIO_REG: u32 = 0x07;
/* Port A/B Pattern Polarity/Transition/Mask registers */
pub const Z8536_PA_PP_REG: u32 = 0x25;
pub const Z8536_PB_PP_REG: u32 = 0x2d;
pub const Z8536_PA_PT_REG: u32 = 0x26;
pub const Z8536_PB_PT_REG: u32 = 0x2e;
pub const Z8536_PA_PM_REG: u32 = 0x27;
pub const Z8536_PB_PM_REG: u32 = 0x2f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
