/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/include/linux/mtd/onenand_regs.h
 *
 *  OneNAND Register header file
 *
 *  Copyright (C) 2005-2007 Samsung Electronics
 *  Kyungmin Park <kyungmin.park@samsung.com>
 */

/* Memory Address Map Translation (Word order) */
pub const fn ONENAND_MEMORY_MAP(x: u32) -> u32 { x << 1 }

/*
 * External BufferRAM area
 */
pub const ONENAND_BOOTRAM: u32 = ONENAND_MEMORY_MAP(0x0000);
pub const ONENAND_DATARAM: u32 = ONENAND_MEMORY_MAP(0x0200);
pub const ONENAND_SPARERAM: u32 = ONENAND_MEMORY_MAP(0x8010);

/*
 * OneNAND Registers
 */
pub const ONENAND_REG_MANUFACTURER_ID: u32 = ONENAND_MEMORY_MAP(0xF000);
pub const ONENAND_REG_DEVICE_ID: u32 = ONENAND_MEMORY_MAP(0xF001);
pub const ONENAND_REG_VERSION_ID: u32 = ONENAND_MEMORY_MAP(0xF002);
pub const ONENAND_REG_DATA_BUFFER_SIZE: u32 = ONENAND_MEMORY_MAP(0xF003);
pub const ONENAND_REG_BOOT_BUFFER_SIZE: u32 = ONENAND_MEMORY_MAP(0xF004);
pub const ONENAND_REG_NUM_BUFFERS: u32 = ONENAND_MEMORY_MAP(0xF005);
pub const ONENAND_REG_TECHNOLOGY: u32 = ONENAND_MEMORY_MAP(0xF006);

pub const ONENAND_REG_START_ADDRESS1: u32 = ONENAND_MEMORY_MAP(0xF100);
pub const ONENAND_REG_START_ADDRESS2: u32 = ONENAND_MEMORY_MAP(0xF101);
pub const ONENAND_REG_START_ADDRESS3: u32 = ONENAND_MEMORY_MAP(0xF102);
pub const ONENAND_REG_START_ADDRESS4: u32 = ONENAND_MEMORY_MAP(0xF103);
pub const ONENAND_REG_START_ADDRESS5: u32 = ONENAND_MEMORY_MAP(0xF104);
pub const ONENAND_REG_START_ADDRESS6: u32 = ONENAND_MEMORY_MAP(0xF105);
pub const ONENAND_REG_START_ADDRESS7: u32 = ONENAND_MEMORY_MAP(0xF106);
pub const ONENAND_REG_START_ADDRESS8: u32 = ONENAND_MEMORY_MAP(0xF107);

pub const ONENAND_REG_START_BUFFER: u32 = ONENAND_MEMORY_MAP(0xF200);
pub const ONENAND_REG_COMMAND: u32 = ONENAND_MEMORY_MAP(0xF220);
pub const ONENAND_REG_SYS_CFG1: u32 = ONENAND_MEMORY_MAP(0xF221);
pub const ONENAND_REG_SYS_CFG2: u32 = ONENAND_MEMORY_MAP(0xF222);
pub const ONENAND_REG_CTRL_STATUS: u32 = ONENAND_MEMORY_MAP(0xF240);
pub const ONENAND_REG_INTERRUPT: u32 = ONENAND_MEMORY_MAP(0xF241);
pub const ONENAND_REG_START_BLOCK_ADDRESS: u32 = ONENAND_MEMORY_MAP(0xF24C);
pub const ONENAND_REG_END_BLOCK_ADDRESS: u32 = ONENAND_MEMORY_MAP(0xF24D);
pub const ONENAND_REG_WP_STATUS: u32 = ONENAND_MEMORY_MAP(0xF24E);

pub const ONENAND_REG_ECC_STATUS: u32 = ONENAND_MEMORY_MAP(0xFF00);
pub const ONENAND_REG_ECC_M0: u32 = ONENAND_MEMORY_MAP(0xFF01);
pub const ONENAND_REG_ECC_S0: u32 = ONENAND_MEMORY_MAP(0xFF02);
pub const ONENAND_REG_ECC_M1: u32 = ONENAND_MEMORY_MAP(0xFF03);
pub const ONENAND_REG_ECC_S1: u32 = ONENAND_MEMORY_MAP(0xFF04);
pub const ONENAND_REG_ECC_M2: u32 = ONENAND_MEMORY_MAP(0xFF05);
pub const ONENAND_REG_ECC_S2: u32 = ONENAND_MEMORY_MAP(0xFF06);
pub const ONENAND_REG_ECC_M3: u32 = ONENAND_MEMORY_MAP(0xFF07);
pub const ONENAND_REG_ECC_S3: u32 = ONENAND_MEMORY_MAP(0xFF08);

/* Device ID Register F001h (R) */
pub const DEVICE_IS_FLEXONENAND: u32 = 1 << 9;
pub const FLEXONENAND_PI_MASK: u32 = 0x3ff;
pub const FLEXONENAND_PI_UNLOCK_SHIFT: u32 = 14;
pub const ONENAND_DEVICE_DENSITY_MASK: u32 = 0xf;
pub const ONENAND_DEVICE_DENSITY_SHIFT: u32 = 4;
pub const ONENAND_DEVICE_IS_DDP: u32 = 1 << 3;
pub const ONENAND_DEVICE_IS_DEMUX: u32 = 1 << 2;
pub const ONENAND_DEVICE_VCC_MASK: u32 = 0x3;

pub const ONENAND_DEVICE_DENSITY_512Mb: u32 = 0x002;
pub const ONENAND_DEVICE_DENSITY_1Gb: u32 = 0x003;
pub const ONENAND_DEVICE_DENSITY_2Gb: u32 = 0x004;
pub const ONENAND_DEVICE_DENSITY_4Gb: u32 = 0x005;
pub const ONENAND_DEVICE_DENSITY_8Gb: u32 = 0x006;

/* Version ID Register F002h (R) */
pub const ONENAND_VERSION_PROCESS_SHIFT: u32 = 8;

/* Technology Register F006h (R) */
pub const ONENAND_TECHNOLOGY_IS_MLC: u32 = 1 << 0;

/* Start Address 1 F100h (R/W) & Start Address 2 F101h (R/W) */
pub const ONENAND_DDP_SHIFT: u32 = 15;
pub const ONENAND_DDP_CHIP0: u32 = 0;
pub const ONENAND_DDP_CHIP1: u32 = 1 << ONENAND_DDP_SHIFT;

/* Start Address 8 F107h (R/W) */
/* Note: It's actually 0x3f in case of SLC */
pub const ONENAND_FPA_MASK: u32 = 0x7f;
pub const ONENAND_FPA_SHIFT: u32 = 2;
pub const ONENAND_FSA_MASK: u32 = 0x03;

/* Start Buffer Register F200h (R/W) */
pub const ONENAND_BSA_MASK: u32 = 0x03;
pub const ONENAND_BSA_SHIFT: u32 = 8;
pub const ONENAND_BSA_BOOTRAM: u32 = 0 << 2;
pub const ONENAND_BSA_DATARAM0: u32 = 2 << 2;
pub const ONENAND_BSA_DATARAM1: u32 = 3 << 2;
/* Note: It's actually 0x03 in case of SLC */
pub const ONENAND_BSC_MASK: u32 = 0x07;

/* Command Register F220h (R/W) */
pub const ONENAND_CMD_READ: u32 = 0x00;
pub const ONENAND_CMD_READOOB: u32 = 0x13;
pub const ONENAND_CMD_PROG: u32 = 0x80;
pub const ONENAND_CMD_PROGOOB: u32 = 0x1A;
pub const ONENAND_CMD_2X_PROG: u32 = 0x7D;
pub const ONENAND_CMD_2X_CACHE_PROG: u32 = 0x7F;
pub const ONENAND_CMD_UNLOCK: u32 = 0x23;
pub const ONENAND_CMD_LOCK: u32 = 0x2A;
pub const ONENAND_CMD_LOCK_TIGHT: u32 = 0x2C;
pub const ONENAND_CMD_UNLOCK_ALL: u32 = 0x27;
pub const ONENAND_CMD_ERASE: u32 = 0x94;
pub const ONENAND_CMD_MULTIBLOCK_ERASE: u32 = 0x95;
pub const ONENAND_CMD_ERASE_VERIFY: u32 = 0x71;
pub const ONENAND_CMD_RESET: u32 = 0xF0;
pub const ONENAND_CMD_OTP_ACCESS: u32 = 0x65;
pub const ONENAND_CMD_READID: u32 = 0x90;
pub const FLEXONENAND_CMD_PI_UPDATE: u32 = 0x05;
pub const FLEXONENAND_CMD_PI_ACCESS: u32 = 0x66;
pub const FLEXONENAND_CMD_RECOVER_LSB: u32 = 0x05;

/* NOTE: Those are not *REAL* commands */
pub const ONENAND_CMD_BUFFERRAM: u32 = 0x1978;
pub const FLEXONENAND_CMD_READ_PI: u32 = 0x1985;

/* System Configuration 1 Register F221h (R, R/W) */
pub const ONENAND_SYS_CFG1_SYNC_READ: u32 = 1 << 15;
pub const ONENAND_SYS_CFG1_BRL_7: u32 = 7 << 12;
pub const ONENAND_SYS_CFG1_BRL_6: u32 = 6 << 12;
pub const ONENAND_SYS_CFG1_BRL_5: u32 = 5 << 12;
pub const ONENAND_SYS_CFG1_BRL_4: u32 = 4 << 12;
pub const ONENAND_SYS_CFG1_BRL_3: u32 = 3 << 12;
pub const ONENAND_SYS_CFG1_BRL_10: u32 = 2 << 12;
pub const ONENAND_SYS_CFG1_BRL_9: u32 = 1 << 12;
pub const ONENAND_SYS_CFG1_BRL_8: u32 = 0 << 12;
pub const ONENAND_SYS_CFG1_BRL_SHIFT: u32 = 12;
pub const ONENAND_SYS_CFG1_BL_32: u32 = 4 << 9;
pub const ONENAND_SYS_CFG1_BL_16: u32 = 3 << 9;
pub const ONENAND_SYS_CFG1_BL_8: u32 = 2 << 9;
pub const ONENAND_SYS_CFG1_BL_4: u32 = 1 << 9;
pub const ONENAND_SYS_CFG1_BL_CONT: u32 = 0 << 9;
pub const ONENAND_SYS_CFG1_BL_SHIFT: u32 = 9;
pub const ONENAND_SYS_CFG1_NO_ECC: u32 = 1 << 8;
pub const ONENAND_SYS_CFG1_RDY: u32 = 1 << 7;
pub const ONENAND_SYS_CFG1_INT: u32 = 1 << 6;
pub const ONENAND_SYS_CFG1_IOBE: u32 = 1 << 5;
pub const ONENAND_SYS_CFG1_RDY_CONF: u32 = 1 << 4;
pub const ONENAND_SYS_CFG1_VHF: u32 = 1 << 3;
pub const ONENAND_SYS_CFG1_HF: u32 = 1 << 2;
pub const ONENAND_SYS_CFG1_SYNC_WRITE: u32 = 1 << 1;

/* Controller Status Register F240h (R) */
pub const ONENAND_CTRL_ONGO: u32 = 1 << 15;
pub const ONENAND_CTRL_LOCK: u32 = 1 << 14;
pub const ONENAND_CTRL_LOAD: u32 = 1 << 13;
pub const ONENAND_CTRL_PROGRAM: u32 = 1 << 12;
pub const ONENAND_CTRL_ERASE: u32 = 1 << 11;
pub const ONENAND_CTRL_ERROR: u32 = 1 << 10;
pub const ONENAND_CTRL_RSTB: u32 = 1 << 7;
pub const ONENAND_CTRL_OTP_L: u32 = 1 << 6;
pub const ONENAND_CTRL_OTP_BL: u32 = 1 << 5;

/* Interrupt Status Register F241h (R) */
pub const ONENAND_INT_MASTER: u32 = 1 << 15;
pub const ONENAND_INT_READ: u32 = 1 << 7;
pub const ONENAND_INT_WRITE: u32 = 1 << 6;
pub const ONENAND_INT_ERASE: u32 = 1 << 5;
pub const ONENAND_INT_RESET: u32 = 1 << 4;
pub const ONENAND_INT_CLEAR: u32 = 0 << 0;

/* NAND Flash Write Protection Status Register F24Eh (R) */
pub const ONENAND_WP_US: u32 = 1 << 2;
pub const ONENAND_WP_LS: u32 = 1 << 1;
pub const ONENAND_WP_LTS: u32 = 1 << 0;

/* ECC Status Reigser FF00h (R) */
pub const ONENAND_ECC_1BIT: u32 = 1 << 0;
pub const ONENAND_ECC_1BIT_ALL: u32 = 0x5555;
pub const ONENAND_ECC_2BIT: u32 = 1 << 1;
pub const ONENAND_ECC_2BIT_ALL: u32 = 0xAAAA;
pub const FLEXONENAND_UNCORRECTABLE_ERROR: u32 = 0x1010;
pub const ONENAND_ECC_3BIT: u32 = 1 << 2;
pub const ONENAND_ECC_4BIT: u32 = 1 << 3;
pub const ONENAND_ECC_4BIT_UNCORRECTABLE: u32 = 0x1010;

/*
 * One-Time Programmable (OTP)
 */
pub const FLEXONENAND_OTP_LOCK_OFFSET: u32 = 2048;
pub const ONENAND_OTP_LOCK_OFFSET: u32 = 14;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
