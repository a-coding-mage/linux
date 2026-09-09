/* Rust translation of linux/mmc/mmc.h. */

pub const MMC_GO_IDLE_STATE: u32 = 0;
pub const MMC_SEND_OP_COND: u32 = 1;
pub const MMC_ALL_SEND_CID: u32 = 2;
pub const MMC_SET_RELATIVE_ADDR: u32 = 3;
pub const MMC_SET_DSR: u32 = 4;
pub const MMC_SLEEP_AWAKE: u32 = 5;
pub const MMC_SWITCH: u32 = 6;
pub const MMC_SELECT_CARD: u32 = 7;
pub const MMC_SEND_EXT_CSD: u32 = 8;
pub const MMC_SEND_CSD: u32 = 9;
pub const MMC_SEND_CID: u32 = 10;
pub const MMC_READ_DAT_UNTIL_STOP: u32 = 11;
pub const MMC_STOP_TRANSMISSION: u32 = 12;
pub const MMC_SEND_STATUS: u32 = 13;
pub const MMC_BUS_TEST_R: u32 = 14;
pub const MMC_GO_INACTIVE_STATE: u32 = 15;
pub const MMC_SET_BLOCKLEN: u32 = 16;
pub const MMC_READ_SINGLE_BLOCK: u32 = 17;
pub const MMC_READ_MULTIPLE_BLOCK: u32 = 18;
pub const MMC_BUS_TEST_W: u32 = 19;
pub const MMC_SEND_TUNING_BLOCK: u32 = 19;
pub const MMC_WRITE_DAT_UNTIL_STOP: u32 = 20;
pub const MMC_SEND_TUNING_BLOCK_HS200: u32 = 21;
pub const MMC_SET_BLOCK_COUNT: u32 = 23;
pub const MMC_WRITE_BLOCK: u32 = 24;
pub const MMC_WRITE_MULTIPLE_BLOCK: u32 = 25;
pub const MMC_PROGRAM_CID: u32 = 26;
pub const MMC_PROGRAM_CSD: u32 = 27;
pub const MMC_SET_WRITE_PROT: u32 = 28;
pub const MMC_CLR_WRITE_PROT: u32 = 29;
pub const MMC_SEND_WRITE_PROT: u32 = 30;
pub const MMC_ERASE_GROUP_START: u32 = 35;
pub const MMC_ERASE_GROUP_END: u32 = 36;
pub const MMC_ERASE: u32 = 38;
pub const MMC_FAST_IO: u32 = 39;
pub const MMC_GO_IRQ_STATE: u32 = 40;
pub const MMC_LOCK_UNLOCK: u32 = 42;
pub const MMC_QUE_TASK_PARAMS: u32 = 44;
pub const MMC_QUE_TASK_ADDR: u32 = 45;
pub const MMC_EXECUTE_READ_TASK: u32 = 46;
pub const MMC_EXECUTE_WRITE_TASK: u32 = 47;
pub const MMC_CMDQ_TASK_MGMT: u32 = 48;
pub const MMC_APP_CMD: u32 = 55;
pub const MMC_GEN_CMD: u32 = 56;
pub const MMC_SPI_READ_OCR: u32 = 58;
pub const MMC_SPI_CRC_ON_OFF: u32 = 59;

#[inline]
pub fn mmc_op_multi(opcode: u32) -> bool { opcode == MMC_WRITE_MULTIPLE_BLOCK || opcode == MMC_READ_MULTIPLE_BLOCK }
#[inline]
pub fn mmc_op_tuning(opcode: u32) -> bool { opcode == MMC_SEND_TUNING_BLOCK || opcode == MMC_SEND_TUNING_BLOCK_HS200 }

pub const R1_OUT_OF_RANGE: u32 = 1 << 31;
pub const R1_ADDRESS_ERROR: u32 = 1 << 30;
pub const R1_BLOCK_LEN_ERROR: u32 = 1 << 29;
pub const R1_ERASE_SEQ_ERROR: u32 = 1 << 28;
pub const R1_ERASE_PARAM: u32 = 1 << 27;
pub const R1_WP_VIOLATION: u32 = 1 << 26;
pub const R1_CARD_IS_LOCKED: u32 = 1 << 25;
pub const R1_LOCK_UNLOCK_FAILED: u32 = 1 << 24;
pub const R1_COM_CRC_ERROR: u32 = 1 << 23;
pub const R1_ILLEGAL_COMMAND: u32 = 1 << 22;
pub const R1_CARD_ECC_FAILED: u32 = 1 << 21;
pub const R1_CC_ERROR: u32 = 1 << 20;
pub const R1_ERROR: u32 = 1 << 19;
pub const R1_UNDERRUN: u32 = 1 << 18;
pub const R1_OVERRUN: u32 = 1 << 17;
pub const R1_CID_CSD_OVERWRITE: u32 = 1 << 16;
pub const R1_WP_ERASE_SKIP: u32 = 1 << 15;
pub const R1_CARD_ECC_DISABLED: u32 = 1 << 14;
pub const R1_ERASE_RESET: u32 = 1 << 13;
#[inline] pub const fn R1_STATUS(x: u32) -> u32 { x & 0xFFF9A000 }
#[inline] pub const fn R1_CURRENT_STATE(x: u32) -> u32 { (x & 0x00001E00) >> 9 }
pub const R1_READY_FOR_DATA: u32 = 1 << 8;
pub const R1_SWITCH_ERROR: u32 = 1 << 7;
pub const R1_EXCEPTION_EVENT: u32 = 1 << 6;
pub const R1_APP_CMD: u32 = 1 << 5;
pub const R1_STATE_IDLE: u32 = 0; pub const R1_STATE_READY: u32 = 1; pub const R1_STATE_IDENT: u32 = 2;
pub const R1_STATE_STBY: u32 = 3; pub const R1_STATE_TRAN: u32 = 4; pub const R1_STATE_DATA: u32 = 5;
pub const R1_STATE_RCV: u32 = 6; pub const R1_STATE_PRG: u32 = 7; pub const R1_STATE_DIS: u32 = 8;
#[inline]
pub fn mmc_ready_for_data(status: u32) -> bool { status & R1_READY_FOR_DATA != 0 && R1_CURRENT_STATE(status) == R1_STATE_TRAN }

pub const R1_SPI_IDLE: u32 = 1 << 0; pub const R1_SPI_ERASE_RESET: u32 = 1 << 1;
pub const R1_SPI_ILLEGAL_COMMAND: u32 = 1 << 2; pub const R1_SPI_COM_CRC: u32 = 1 << 3;
pub const R1_SPI_ERASE_SEQ: u32 = 1 << 4; pub const R1_SPI_ADDRESS: u32 = 1 << 5; pub const R1_SPI_PARAMETER: u32 = 1 << 6;
pub const R2_SPI_CARD_LOCKED: u32 = 1 << 8; pub const R2_SPI_WP_ERASE_SKIP: u32 = 1 << 9;
pub const R2_SPI_LOCK_UNLOCK_FAIL: u32 = R2_SPI_WP_ERASE_SKIP; pub const R2_SPI_ERROR: u32 = 1 << 10;
pub const R2_SPI_CC_ERROR: u32 = 1 << 11; pub const R2_SPI_CARD_ECC_ERROR: u32 = 1 << 12;
pub const R2_SPI_WP_VIOLATION: u32 = 1 << 13; pub const R2_SPI_ERASE_PARAM: u32 = 1 << 14;
pub const R2_SPI_OUT_OF_RANGE: u32 = 1 << 15; pub const R2_SPI_CSD_OVERWRITE: u32 = R2_SPI_OUT_OF_RANGE;
pub const MMC_CARD_BUSY: u32 = 0x80000000;

pub const CCC_BASIC: u32 = 1<<0; pub const CCC_STREAM_READ: u32 = 1<<1; pub const CCC_BLOCK_READ: u32 = 1<<2;
pub const CCC_STREAM_WRITE: u32 = 1<<3; pub const CCC_BLOCK_WRITE: u32 = 1<<4; pub const CCC_ERASE: u32 = 1<<5;
pub const CCC_WRITE_PROT: u32 = 1<<6; pub const CCC_LOCK_CARD: u32 = 1<<7; pub const CCC_APP_SPEC: u32 = 1<<8;
pub const CCC_IO_MODE: u32 = 1<<9; pub const CCC_SWITCH: u32 = 1<<10;

pub const CSD_STRUCT_VER_1_0: u32=0; pub const CSD_STRUCT_VER_1_1: u32=1; pub const CSD_STRUCT_VER_1_2: u32=2; pub const CSD_STRUCT_EXT_CSD: u32=3;
pub const CSD_SPEC_VER_0: u32=0; pub const CSD_SPEC_VER_1: u32=1; pub const CSD_SPEC_VER_2: u32=2; pub const CSD_SPEC_VER_3: u32=3; pub const CSD_SPEC_VER_4: u32=4;

pub const EXT_CSD_CMDQ_MODE_EN:u32=15; pub const EXT_CSD_FLUSH_CACHE:u32=32; pub const EXT_CSD_CACHE_CTRL:u32=33; pub const EXT_CSD_POWER_OFF_NOTIFICATION:u32=34;
pub const EXT_CSD_EXP_EVENTS_STATUS:u32=54; pub const EXT_CSD_EXP_EVENTS_CTRL:u32=56; pub const EXT_CSD_DATA_SECTOR_SIZE:u32=61; pub const EXT_CSD_GP_SIZE_MULT:u32=143;
pub const EXT_CSD_PARTITION_SETTING_COMPLETED:u32=155; pub const EXT_CSD_PARTITION_ATTRIBUTE:u32=156; pub const EXT_CSD_PARTITION_SUPPORT:u32=160; pub const EXT_CSD_HPI_MGMT:u32=161; pub const EXT_CSD_RST_N_FUNCTION:u32=162; pub const EXT_CSD_BKOPS_EN:u32=163; pub const EXT_CSD_BKOPS_START:u32=164; pub const EXT_CSD_SANITIZE_START:u32=165; pub const EXT_CSD_WR_REL_PARAM:u32=166; pub const EXT_CSD_RPMB_MULT:u32=168; pub const EXT_CSD_FW_CONFIG:u32=169; pub const EXT_CSD_BOOT_WP:u32=173; pub const EXT_CSD_ERASE_GROUP_DEF:u32=175; pub const EXT_CSD_PART_CONFIG:u32=179; pub const EXT_CSD_ERASED_MEM_CONT:u32=181; pub const EXT_CSD_BUS_WIDTH:u32=183; pub const EXT_CSD_STROBE_SUPPORT:u32=184; pub const EXT_CSD_HS_TIMING:u32=185; pub const EXT_CSD_POWER_CLASS:u32=187; pub const EXT_CSD_REV:u32=192; pub const EXT_CSD_STRUCTURE:u32=194; pub const EXT_CSD_CARD_TYPE:u32=196; pub const EXT_CSD_DRIVER_STRENGTH:u32=197; pub const EXT_CSD_OUT_OF_INTERRUPT_TIME:u32=198; pub const EXT_CSD_PART_SWITCH_TIME:u32=199; pub const EXT_CSD_PWR_CL_52_195:u32=200; pub const EXT_CSD_PWR_CL_26_195:u32=201; pub const EXT_CSD_PWR_CL_52_360:u32=202; pub const EXT_CSD_PWR_CL_26_360:u32=203; pub const EXT_CSD_SEC_CNT:u32=212; pub const EXT_CSD_S_A_TIMEOUT:u32=217; pub const EXT_CSD_HC_WP_GRP_SIZE:u32=221; pub const EXT_CSD_REL_WR_SEC_C:u32=222; pub const EXT_CSD_ERASE_TIMEOUT_MULT:u32=223; pub const EXT_CSD_HC_ERASE_GRP_SIZE:u32=224; pub const EXT_CSD_BOOT_MULT:u32=226; pub const EXT_CSD_SEC_TRIM_MULT:u32=229; pub const EXT_CSD_SEC_ERASE_MULT:u32=230; pub const EXT_CSD_SEC_FEATURE_SUPPORT:u32=231; pub const EXT_CSD_TRIM_MULT:u32=232; pub const EXT_CSD_PWR_CL_200_195:u32=236; pub const EXT_CSD_PWR_CL_200_360:u32=237; pub const EXT_CSD_PWR_CL_DDR_52_195:u32=238; pub const EXT_CSD_PWR_CL_DDR_52_360:u32=239; pub const EXT_CSD_BKOPS_STATUS:u32=246; pub const EXT_CSD_POWER_OFF_LONG_TIME:u32=247; pub const EXT_CSD_GENERIC_CMD6_TIME:u32=248; pub const EXT_CSD_CACHE_SIZE:u32=249; pub const EXT_CSD_PWR_CL_DDR_200_360:u32=253; pub const EXT_CSD_FIRMWARE_VERSION:u32=254; pub const EXT_CSD_PRE_EOL_INFO:u32=267; pub const EXT_CSD_DEVICE_LIFE_TIME_EST_TYP_A:u32=268; pub const EXT_CSD_DEVICE_LIFE_TIME_EST_TYP_B:u32=269; pub const EXT_CSD_CMDQ_DEPTH:u32=307; pub const EXT_CSD_CMDQ_SUPPORT:u32=308; pub const EXT_CSD_SUPPORTED_MODE:u32=493; pub const EXT_CSD_TAG_UNIT_SIZE:u32=498; pub const EXT_CSD_DATA_TAG_SUPPORT:u32=499; pub const EXT_CSD_BKOPS_SUPPORT:u32=502; pub const EXT_CSD_HPI_FEATURES:u32=503;

pub const EXT_CSD_WR_REL_PARAM_EN:u32=1<<2; pub const EXT_CSD_WR_REL_PARAM_EN_RPMB_REL_WR:u32=1<<4;
pub const EXT_CSD_BOOT_WP_B_PWR_WP_DIS:u32=0x40; pub const EXT_CSD_BOOT_WP_B_PERM_WP_DIS:u32=0x10; pub const EXT_CSD_BOOT_WP_B_PERM_WP_EN:u32=0x04; pub const EXT_CSD_BOOT_WP_B_PWR_WP_EN:u32=0x01;
pub const EXT_CSD_PART_CONFIG_ACC_MASK:u32=0x7; pub const EXT_CSD_PART_CONFIG_ACC_BOOT0:u32=0x1; pub const EXT_CSD_PART_CONFIG_ACC_RPMB:u32=0x3; pub const EXT_CSD_PART_CONFIG_ACC_GP0:u32=0x4;
pub const EXT_CSD_PART_SETTING_COMPLETED:u32=1; pub const EXT_CSD_PART_SUPPORT_PART_EN:u32=1; pub const EXT_CSD_CMD_SET_NORMAL:u32=1; pub const EXT_CSD_CMD_SET_SECURE:u32=2; pub const EXT_CSD_CMD_SET_CPSECURE:u32=4;
pub const EXT_CSD_CARD_TYPE_HS_26:u32=1; pub const EXT_CSD_CARD_TYPE_HS_52:u32=2; pub const EXT_CSD_CARD_TYPE_HS:u32=3; pub const EXT_CSD_CARD_TYPE_DDR_1_8V:u32=4; pub const EXT_CSD_CARD_TYPE_DDR_1_2V:u32=8; pub const EXT_CSD_CARD_TYPE_DDR_52:u32=12; pub const EXT_CSD_CARD_TYPE_HS200_1_8V:u32=16; pub const EXT_CSD_CARD_TYPE_HS200_1_2V:u32=32; pub const EXT_CSD_CARD_TYPE_HS200:u32=48; pub const EXT_CSD_CARD_TYPE_HS400_1_8V:u32=64; pub const EXT_CSD_CARD_TYPE_HS400_1_2V:u32=128; pub const EXT_CSD_CARD_TYPE_HS400:u32=192; pub const EXT_CSD_CARD_TYPE_HS400ES:u32=1<<8;
pub const EXT_CSD_BUS_WIDTH_1:u32=0; pub const EXT_CSD_BUS_WIDTH_4:u32=1; pub const EXT_CSD_BUS_WIDTH_8:u32=2; pub const EXT_CSD_DDR_BUS_WIDTH_4:u32=5; pub const EXT_CSD_DDR_BUS_WIDTH_8:u32=6;
pub const EXT_CSD_BUS_WIDTH_STROBE:u32=1<<7;
pub const EXT_CSD_TIMING_BC:u32=0; pub const EXT_CSD_TIMING_HS:u32=1; pub const EXT_CSD_TIMING_HS200:u32=2; pub const EXT_CSD_TIMING_HS400:u32=3; pub const EXT_CSD_DRV_STR_SHIFT:u32=4;
pub const EXT_CSD_RST_N_EN_MASK:u32=0x3; pub const EXT_CSD_RST_N_ENABLED:u32=1; pub const EXT_CSD_NO_POWER_NOTIFICATION:u32=0; pub const EXT_CSD_POWER_ON:u32=1; pub const EXT_CSD_POWER_OFF_SHORT:u32=2; pub const EXT_CSD_POWER_OFF_LONG:u32=3;
pub const EXT_CSD_PWR_CL_8BIT_MASK:u32=0xF0; pub const EXT_CSD_PWR_CL_4BIT_MASK:u32=0x0F; pub const EXT_CSD_PWR_CL_8BIT_SHIFT:u32=4; pub const EXT_CSD_PWR_CL_4BIT_SHIFT:u32=0;
pub const EXT_CSD_SEC_ER_EN:u32=1<<0; pub const EXT_CSD_SEC_BD_BLK_EN:u32=1<<2; pub const EXT_CSD_SEC_GB_CL_EN:u32=1<<4; pub const EXT_CSD_SEC_SANITIZE:u32=1<<6;
pub const EXT_CSD_URGENT_BKOPS:u32=1<<0; pub const EXT_CSD_DYNCAP_NEEDED:u32=1<<1; pub const EXT_CSD_SYSPOOL_EXHAUSTED:u32=1<<2;
pub const EXT_CSD_BKOPS_LEVEL_2:u32=2; pub const EXT_CSD_MANUAL_BKOPS_MASK:u32=1; pub const EXT_CSD_AUTO_BKOPS_MASK:u32=2;
pub const EXT_CSD_CMDQ_MODE_ENABLED:u32=1<<0; pub const EXT_CSD_CMDQ_DEPTH_MASK:u32=0x1F; pub const EXT_CSD_CMDQ_SUPPORTED:u32=1<<0;
pub const MMC_SWITCH_MODE_CMD_SET:u32=0; pub const MMC_SWITCH_MODE_SET_BITS:u32=1; pub const MMC_SWITCH_MODE_CLEAR_BITS:u32=2; pub const MMC_SWITCH_MODE_WRITE_BYTE:u32=3;
pub const MMC_ERASE_ARG:u32=0; pub const MMC_SECURE_ERASE_ARG:u32=0x80000000; pub const MMC_TRIM_ARG:u32=1; pub const MMC_DISCARD_ARG:u32=3; pub const MMC_SECURE_TRIM1_ARG:u32=0x80000001; pub const MMC_SECURE_TRIM2_ARG:u32=0x80008000; pub const MMC_SECURE_ARGS:u32=0x80000000; pub const MMC_TRIM_OR_DISCARD_ARGS:u32=0x80008003;
#[inline] pub const fn mmc_driver_type_mask(n: u32) -> u32 { 1 << n }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
