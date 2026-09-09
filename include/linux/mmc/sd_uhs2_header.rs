/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Header file for UHS-II packets, Host Controller registers and I/O
 * accessors.
 *
 * Copyright (C) 2014 Intel Corp, All Rights Reserved.
 */

/* LINK Layer definition */
/*
 * UHS2 Header:
 * Refer to UHS-II Addendum Version 1.02 Figure 5-2, the format of CCMD Header is described below:
 *      bit [3:0]  : DID(Destination ID = Node ID of UHS2 card)
 *      bit [6:4]  : TYP(Packet Type)
 *                   000b: CCMD(Control command packet)
 *                   001b: DCMD(Data command packet)
 *                   010b: RES(Response packet)
 *                   011b: DATA(Data payload packet)
 *                   111b: MSG(Message packet)
 *                   Others: Reserved
 *      bit [7]    : NP(Native Packet)
 *      bit [10:8] : TID(Transaction ID)
 *      bit [11]   : Reserved
 *      bit [15:12]: SID(Source ID 0: Node ID of Host)
 *
 * Broadcast CCMD issued by Host is represented as DID=SID=0.
 */
/*
 * UHS2 Argument:
 * Refer to UHS-II Addendum Version 1.02 Figure 6-5, the format of CCMD Argument is described below:
 *      bit [3:0]  : MSB of IOADR
 *      bit [5:4]  : PLEN(Payload Length)
 *                   00b: 0 byte
 *                   01b: 4 bytes
 *                   10b: 8 bytes
 *                   11b: 16 bytes
 *      bit [6]    : Reserved
 *      bit [7]    : R/W(Read/Write)
 *                   0: Control read command
 *                   1: Control write command
 *      bit [15:8] : LSB of IOADR
 *
 * I/O Address specifies the address of register in UHS-II I/O space accessed by CCMD.
 * The unit of I/O Address is 4 Bytes. It is transmitted in MSB first, LSB last.
 */
pub const UHS2_NATIVE_PACKET_POS: u32 = 7;
pub const UHS2_NATIVE_PACKET: u32 = 1u32 << UHS2_NATIVE_PACKET_POS;

pub const UHS2_PACKET_TYPE_POS: u32 = 4;
pub const UHS2_PACKET_TYPE_CCMD: u32 = 0u32 << UHS2_PACKET_TYPE_POS;
pub const UHS2_PACKET_TYPE_DCMD: u32 = 1u32 << UHS2_PACKET_TYPE_POS;
pub const UHS2_PACKET_TYPE_RES: u32 = 2u32 << UHS2_PACKET_TYPE_POS;
pub const UHS2_PACKET_TYPE_DATA: u32 = 3u32 << UHS2_PACKET_TYPE_POS;
pub const UHS2_PACKET_TYPE_MSG: u32 = 7u32 << UHS2_PACKET_TYPE_POS;

pub const UHS2_DEST_ID_MASK: u32 = 0x0F;
pub const UHS2_DEST_ID: u32 = 0x1;
pub const UHS2_SRC_ID_POS: u32 = 12;
pub const UHS2_SRC_ID_MASK: u32 = 0xF000;
pub const UHS2_TRANS_ID_POS: u32 = 8;
pub const UHS2_TRANS_ID_MASK: u32 = 0x0700;

/* UHS2 MSG */
pub const UHS2_MSG_CTG_POS: u32 = 5;
pub const UHS2_MSG_CTG_LMSG: u32 = 0x00;
pub const UHS2_MSG_CTG_INT: u32 = 0x60;
pub const UHS2_MSG_CTG_AMSG: u32 = 0x80;
pub const UHS2_MSG_CTG_FCREQ: u32 = 0x00;
pub const UHS2_MSG_CTG_FCRDY: u32 = 0x01;
pub const UHS2_MSG_CTG_STAT: u32 = 0x02;
pub const UHS2_MSG_CODE_POS: u32 = 8;
pub const UHS2_MSG_CODE_FC_UNRECOVER_ERR: u32 = 0x8;
pub const UHS2_MSG_CODE_STAT_UNRECOVER_ERR: u32 = 0x8;
pub const UHS2_MSG_CODE_STAT_RECOVER_ERR: u32 = 0x1;

/* TRANS Layer definition */
/* Native packets*/
pub const UHS2_NATIVE_CMD_RW_POS: u32 = 7;
pub const UHS2_NATIVE_CMD_WRITE: u32 = 1u32 << UHS2_NATIVE_CMD_RW_POS;
pub const UHS2_NATIVE_CMD_READ: u32 = 0u32 << UHS2_NATIVE_CMD_RW_POS;
pub const UHS2_NATIVE_CMD_PLEN_POS: u32 = 4;
pub const UHS2_NATIVE_CMD_PLEN_4B: u32 = 1u32 << UHS2_NATIVE_CMD_PLEN_POS;
pub const UHS2_NATIVE_CMD_PLEN_8B: u32 = 2u32 << UHS2_NATIVE_CMD_PLEN_POS;
pub const UHS2_NATIVE_CMD_PLEN_16B: u32 = 3u32 << UHS2_NATIVE_CMD_PLEN_POS;
pub const UHS2_NATIVE_CCMD_GET_MIOADR_MASK: u32 = 0xF00;
pub const UHS2_NATIVE_CCMD_MIOADR_MASK: u32 = 0x0F;
pub const UHS2_NATIVE_CCMD_LIOADR_POS: u32 = 8;
pub const UHS2_NATIVE_CCMD_GET_LIOADR_MASK: u32 = 0x0FF;
pub const UHS2_CCMD_DEV_INIT_COMPLETE_FLAG: u32 = 1u32 << 11;
pub const UHS2_DEV_INIT_PAYLOAD_LEN: u32 = 1;
pub const UHS2_DEV_INIT_RESP_LEN: u32 = 6;
pub const UHS2_DEV_ENUM_PAYLOAD_LEN: u32 = 1;
pub const UHS2_DEV_ENUM_RESP_LEN: u32 = 8;
pub const UHS2_CFG_WRITE_PAYLOAD_LEN: u32 = 2;
pub const UHS2_CFG_WRITE_PHY_SET_RESP_LEN: u32 = 4;
pub const UHS2_CFG_WRITE_GENERIC_SET_RESP_LEN: u32 = 5;
pub const UHS2_GO_DORMANT_PAYLOAD_LEN: u32 = 1;

/* UHS2 Argument: DCMD Argument format, per UHS-II Addendum Version 1.02 Figure 6-8. */
pub const UHS2_DCMD_DM_POS: u32 = 6;
pub const UHS2_DCMD_2L_HD_MODE: u32 = 1u32 << UHS2_DCMD_DM_POS;
pub const UHS2_DCMD_LM_POS: u32 = 5;
pub const UHS2_DCMD_LM_TLEN_EXIST: u32 = 1u32 << UHS2_DCMD_LM_POS;
pub const UHS2_DCMD_TLUM_POS: u32 = 4;
pub const UHS2_DCMD_TLUM_BYTE_MODE: u32 = 1u32 << UHS2_DCMD_TLUM_POS;
pub const UHS2_NATIVE_DCMD_DAM_POS: u32 = 3;
pub const UHS2_NATIVE_DCMD_DAM_IO: u32 = 1u32 << UHS2_NATIVE_DCMD_DAM_POS;
pub const UHS2_RES_NACK_POS: u32 = 7;
pub const UHS2_RES_NACK_MASK: u32 = 0x1u32 << UHS2_RES_NACK_POS;
pub const UHS2_RES_ECODE_POS: u32 = 4;
pub const UHS2_RES_ECODE_MASK: u32 = 0x7;
pub const UHS2_RES_ECODE_COND: u32 = 1;
pub const UHS2_RES_ECODE_ARG: u32 = 2;
pub const UHS2_RES_ECODE_GEN: u32 = 3;

/* IOADR of device registers */
pub const UHS2_IOADR_GENERIC_CAPS: u32 = 0x00;
pub const UHS2_IOADR_PHY_CAPS: u32 = 0x02;
pub const UHS2_IOADR_LINK_CAPS: u32 = 0x04;
pub const UHS2_IOADR_RSV_CAPS: u32 = 0x06;
pub const UHS2_IOADR_GENERIC_SETTINGS: u32 = 0x08;
pub const UHS2_IOADR_PHY_SETTINGS: u32 = 0x0A;
pub const UHS2_IOADR_LINK_SETTINGS: u32 = 0x0C;
pub const UHS2_IOADR_PRESET: u32 = 0x40;

/* SD application packets */
pub const UHS2_SD_CMD_INDEX_POS: u32 = 8;
pub const UHS2_SD_CMD_APP_POS: u32 = 14;
pub const UHS2_SD_CMD_APP: u32 = 1u32 << UHS2_SD_CMD_APP_POS;

/* UHS-II Device Registers */
pub const UHS2_DEV_CONFIG_REG: u32 = 0x000;
pub const UHS2_DEV_CONFIG_GEN_CAPS: u32 = UHS2_DEV_CONFIG_REG + 0x000;
pub const UHS2_DEV_CONFIG_N_LANES_POS: u32 = 8;
pub const UHS2_DEV_CONFIG_N_LANES_MASK: u32 = 0x3F;
pub const UHS2_DEV_CONFIG_2L_HD_FD: u32 = 0x1;
pub const UHS2_DEV_CONFIG_2D1U_FD: u32 = 0x2;
pub const UHS2_DEV_CONFIG_1D2U_FD: u32 = 0x4;
pub const UHS2_DEV_CONFIG_2D2U_FD: u32 = 0x8;
pub const UHS2_DEV_CONFIG_DADR_POS: u32 = 14;
pub const UHS2_DEV_CONFIG_DADR_MASK: u32 = 0x1;
pub const UHS2_DEV_CONFIG_APP_POS: u32 = 16;
pub const UHS2_DEV_CONFIG_APP_MASK: u32 = 0xFF;
pub const UHS2_DEV_CONFIG_APP_SD_MEM: u32 = 0x1;
pub const UHS2_DEV_CONFIG_GEN_SET: u32 = UHS2_DEV_CONFIG_REG + 0x008;
pub const UHS2_DEV_CONFIG_GEN_SET_N_LANES_POS: u32 = 8;
pub const UHS2_DEV_CONFIG_GEN_SET_2L_FD_HD: u32 = 0x0;
pub const UHS2_DEV_CONFIG_GEN_SET_2D1U_FD: u32 = 0x2;
pub const UHS2_DEV_CONFIG_GEN_SET_1D2U_FD: u32 = 0x3;
pub const UHS2_DEV_CONFIG_GEN_SET_2D2U_FD: u32 = 0x4;
pub const UHS2_DEV_CONFIG_GEN_SET_CFG_COMPLETE: u32 = 1u32 << 31;

/* PHY Caps and Settings registers */
pub const UHS2_DEV_CONFIG_PHY_CAPS: u32 = UHS2_DEV_CONFIG_REG + 0x002;
pub const UHS2_DEV_CONFIG_PHY_MINOR_MASK: u32 = 0xF;
pub const UHS2_DEV_CONFIG_PHY_MAJOR_POS: u32 = 4;
pub const UHS2_DEV_CONFIG_PHY_MAJOR_MASK: u32 = 0x3;
pub const UHS2_DEV_CONFIG_CAN_HIBER_POS: u32 = 15;
pub const UHS2_DEV_CONFIG_CAN_HIBER_MASK: u32 = 0x1;
pub const UHS2_DEV_CONFIG_PHY_CAPS1: u32 = UHS2_DEV_CONFIG_REG + 0x003;
pub const UHS2_DEV_CONFIG_N_LSS_SYN_MASK: u32 = 0xF;
pub const UHS2_DEV_CONFIG_N_LSS_DIR_POS: u32 = 4;
pub const UHS2_DEV_CONFIG_N_LSS_DIR_MASK: u32 = 0xF;
pub const UHS2_DEV_CONFIG_PHY_SET: u32 = UHS2_DEV_CONFIG_REG + 0x00A;
pub const UHS2_DEV_CONFIG_PHY_SET_SPEED_POS: u32 = 6;
pub const UHS2_DEV_CONFIG_PHY_SET_SPEED_A: u32 = 0x0;
pub const UHS2_DEV_CONFIG_PHY_SET_SPEED_B: u32 = 0x1;

/* LINK-TRAN Caps and Settings registers */
pub const UHS2_DEV_CONFIG_LINK_TRAN_CAPS: u32 = UHS2_DEV_CONFIG_REG + 0x004;
pub const UHS2_DEV_CONFIG_LT_MINOR_MASK: u32 = 0xF;
pub const UHS2_DEV_CONFIG_LT_MAJOR_POS: u32 = 4;
pub const UHS2_DEV_CONFIG_LT_MAJOR_MASK: u32 = 0x3;
pub const UHS2_DEV_CONFIG_N_FCU_POS: u32 = 8;
pub const UHS2_DEV_CONFIG_N_FCU_MASK: u32 = 0xFF;
pub const UHS2_DEV_CONFIG_DEV_TYPE_POS: u32 = 16;
pub const UHS2_DEV_CONFIG_DEV_TYPE_MASK: u32 = 0x7;
pub const UHS2_DEV_CONFIG_MAX_BLK_LEN_POS: u32 = 20;
pub const UHS2_DEV_CONFIG_MAX_BLK_LEN_MASK: u32 = 0xFFF;
pub const UHS2_DEV_CONFIG_LINK_TRAN_CAPS1: u32 = UHS2_DEV_CONFIG_REG + 0x005;
pub const UHS2_DEV_CONFIG_N_DATA_GAP_MASK: u32 = 0xFF;
pub const UHS2_DEV_CONFIG_LINK_TRAN_SET: u32 = UHS2_DEV_CONFIG_REG + 0x00C;
pub const UHS2_DEV_CONFIG_LT_SET_MAX_BLK_LEN: u32 = 0x200;
pub const UHS2_DEV_CONFIG_LT_SET_MAX_RETRY_POS: u32 = 16;

/* Preset register */
pub const UHS2_DEV_CONFIG_PRESET: u32 = UHS2_DEV_CONFIG_REG + 0x040;
pub const UHS2_DEV_INT_REG: u32 = 0x100;
pub const UHS2_DEV_STATUS_REG: u32 = 0x180;
pub const UHS2_DEV_CMD_REG: u32 = 0x200;
pub const UHS2_DEV_CMD_FULL_RESET: u32 = UHS2_DEV_CMD_REG + 0x000;
pub const UHS2_DEV_CMD_GO_DORMANT_STATE: u32 = UHS2_DEV_CMD_REG + 0x001;
pub const UHS2_DEV_CMD_DORMANT_HIBER: u32 = 1u32 << 7;
pub const UHS2_DEV_CMD_DEVICE_INIT: u32 = UHS2_DEV_CMD_REG + 0x002;
pub const UHS2_DEV_INIT_COMPLETE_FLAG: u32 = 1u32 << 11;
pub const UHS2_DEV_CMD_ENUMERATE: u32 = UHS2_DEV_CMD_REG + 0x003;
pub const UHS2_DEV_CMD_TRANS_ABORT: u32 = UHS2_DEV_CMD_REG + 0x004;
pub const UHS2_RCLK_MAX: u32 = 52000000;
pub const UHS2_RCLK_MIN: u32 = 26000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
