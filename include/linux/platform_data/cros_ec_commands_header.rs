// SPDX-License-Identifier: GPL-2.0-only
#![allow(non_upper_case_globals, non_camel_case_types, dead_code)]

const fn bit(n: u64) -> u64 { 1u64 << n }
const fn bit_ull(n: u64) -> u64 { 1u64 << n }

pub const EC_PROTO_VERSION: u64 = 0x00000002;
pub const EC_LPC_ADDR_ACPI_DATA: u64 = 0x62;
pub const EC_LPC_ADDR_ACPI_CMD: u64 = 0x66;
pub const EC_LPC_ADDR_HOST_DATA: u64 = 0x200;
pub const EC_LPC_ADDR_HOST_CMD: u64 = 0x204;
pub const EC_LPC_ADDR_HOST_ARGS: u64 = 0x800;
pub const EC_LPC_ADDR_HOST_PARAM: u64 = 0x804;
pub const EC_LPC_ADDR_HOST_PACKET: u64 = 0x800;
pub const EC_LPC_HOST_PACKET_SIZE: u64 = 0x100;
pub const EC_HOST_CMD_REGION0: u64 = 0x800;
pub const EC_HOST_CMD_REGION1: u64 = 0x880;
pub const EC_HOST_CMD_REGION_SIZE: u64 = 0x80;
pub const EC_HOST_CMD_MEC_REGION_SIZE: u64 = 0x8;
pub const EC_LPC_CMDR_DATA: u64 = bit(0);
pub const EC_LPC_CMDR_PENDING: u64 = bit(1);
pub const EC_LPC_CMDR_BUSY: u64 = bit(2);
pub const EC_LPC_CMDR_CMD: u64 = bit(3);
pub const EC_LPC_CMDR_ACPI_BRST: u64 = bit(4);
pub const EC_LPC_CMDR_SCI: u64 = bit(5);
pub const EC_LPC_CMDR_SMI: u64 = bit(6);
pub const EC_LPC_ADDR_MEMMAP: u64 = 0x900;
pub const EC_MEMMAP_SIZE: u64 = 255;
pub const EC_MEMMAP_TEXT_MAX: u64 = 8;
pub const EC_MEMMAP_TEMP_SENSOR: u64 = 0x00;
pub const EC_MEMMAP_FAN: u64 = 0x10;
pub const EC_MEMMAP_TEMP_SENSOR_B: u64 = 0x18;
pub const EC_MEMMAP_ID: u64 = 0x20;
pub const EC_MEMMAP_ID_VERSION: u64 = 0x22;
pub const EC_MEMMAP_THERMAL_VERSION: u64 = 0x23;
pub const EC_MEMMAP_BATTERY_VERSION: u64 = 0x24;
pub const EC_MEMMAP_SWITCHES_VERSION: u64 = 0x25;
pub const EC_MEMMAP_EVENTS_VERSION: u64 = 0x26;
pub const EC_MEMMAP_HOST_CMD_FLAGS: u64 = 0x27;
pub const EC_MEMMAP_SWITCHES: u64 = 0x30;
pub const EC_MEMMAP_HOST_EVENTS: u64 = 0x34;
pub const EC_MEMMAP_BATT_VOLT: u64 = 0x40;
pub const EC_MEMMAP_BATT_RATE: u64 = 0x44;
pub const EC_MEMMAP_BATT_CAP: u64 = 0x48;
pub const EC_MEMMAP_BATT_FLAG: u64 = 0x4c;
pub const EC_MEMMAP_BATT_COUNT: u64 = 0x4d;
pub const EC_MEMMAP_BATT_INDEX: u64 = 0x4e;
pub const EC_MEMMAP_BATT_DCAP: u64 = 0x50;
pub const EC_MEMMAP_BATT_DVLT: u64 = 0x54;
pub const EC_MEMMAP_BATT_LFCC: u64 = 0x58;
pub const EC_MEMMAP_BATT_CCNT: u64 = 0x5c;
pub const EC_MEMMAP_BATT_MFGR: u64 = 0x60;
pub const EC_MEMMAP_BATT_MODEL: u64 = 0x68;
pub const EC_MEMMAP_BATT_SERIAL: u64 = 0x70;
pub const EC_MEMMAP_BATT_TYPE: u64 = 0x78;
pub const EC_MEMMAP_ALS: u64 = 0x80;
pub const EC_MEMMAP_ACC_STATUS: u64 = 0x90;
pub const EC_MEMMAP_ACC_DATA: u64 = 0x92;
pub const EC_MEMMAP_GYRO_DATA: u64 = 0xa0;
pub const EC_MEMMAP_NO_ACPI: u64 = 0xe0;
pub const EC_MEMMAP_ACC_STATUS_SAMPLE_ID_MASK: u64 = 0x0f;
pub const EC_MEMMAP_ACC_STATUS_BUSY_BIT: u64 = bit(4);
pub const EC_MEMMAP_ACC_STATUS_PRESENCE_BIT: u64 = bit(7);
pub const EC_TEMP_SENSOR_ENTRIES: u64 = 16;
pub const EC_TEMP_SENSOR_B_ENTRIES: u64 = 8;
pub const EC_TEMP_SENSOR_NOT_PRESENT: u64 = 0xff;
pub const EC_TEMP_SENSOR_ERROR: u64 = 0xfe;
pub const EC_TEMP_SENSOR_NOT_POWERED: u64 = 0xfd;
pub const EC_TEMP_SENSOR_NOT_CALIBRATED: u64 = 0xfc;
pub const EC_TEMP_SENSOR_OFFSET: u64 = 200;
pub const EC_ALS_ENTRIES: u64 = 2;
pub const EC_TEMP_SENSOR_DEFAULT: u64 = (296 - EC_TEMP_SENSOR_OFFSET);
pub const EC_FAN_SPEED_ENTRIES: u64 = 4;
pub const EC_FAN_SPEED_NOT_PRESENT: u64 = 0xffff;
pub const EC_FAN_SPEED_STALLED: u64 = 0xfffe;
pub const EC_BATT_FLAG_AC_PRESENT: u64 = 0x01;
pub const EC_BATT_FLAG_BATT_PRESENT: u64 = 0x02;
pub const EC_BATT_FLAG_DISCHARGING: u64 = 0x04;
pub const EC_BATT_FLAG_CHARGING: u64 = 0x08;
pub const EC_BATT_FLAG_LEVEL_CRITICAL: u64 = 0x10;
pub const EC_BATT_FLAG_INVALID_DATA: u64 = 0x20;
pub const EC_SWITCH_LID_OPEN: u64 = 0x01;
pub const EC_SWITCH_POWER_BUTTON_PRESSED: u64 = 0x02;
pub const EC_SWITCH_WRITE_PROTECT_DISABLED: u64 = 0x04;
pub const EC_SWITCH_IGNORE1: u64 = 0x08;
pub const EC_SWITCH_DEDICATED_RECOVERY: u64 = 0x10;
pub const EC_SWITCH_IGNORE0: u64 = 0x20;
pub const EC_HOST_CMD_FLAG_LPC_ARGS_SUPPORTED: u64 = 0x01;
pub const EC_HOST_CMD_FLAG_VERSION_3: u64 = 0x02;
pub const EC_WIRELESS_SWITCH_ALL: u64 = ~0x00;
pub const EC_WIRELESS_SWITCH_WLAN: u64 = 0x01;
pub const EC_WIRELESS_SWITCH_BLUETOOTH: u64 = 0x02;
pub const EC_WIRELESS_SWITCH_WWAN: u64 = 0x04;
pub const EC_WIRELESS_SWITCH_WLAN_POWER: u64 = 0x08;
pub const EC_CMD_ACPI_READ: u64 = 0x0080;
pub const EC_CMD_ACPI_WRITE: u64 = 0x0081;
pub const EC_CMD_ACPI_BURST_ENABLE: u64 = 0x0082;
pub const EC_CMD_ACPI_BURST_DISABLE: u64 = 0x0083;
pub const EC_CMD_ACPI_QUERY_EVENT: u64 = 0x0084;
pub const EC_ACPI_MEM_VERSION: u64 = 0x00;
pub const EC_ACPI_MEM_TEST: u64 = 0x01;
pub const EC_ACPI_MEM_TEST_COMPLIMENT: u64 = 0x02;
pub const EC_ACPI_MEM_KEYBOARD_BACKLIGHT: u64 = 0x03;
pub const EC_ACPI_MEM_FAN_DUTY: u64 = 0x04;
pub const EC_ACPI_MEM_TEMP_ID: u64 = 0x05;
pub const EC_ACPI_MEM_TEMP_THRESHOLD: u64 = 0x06;
pub const EC_ACPI_MEM_TEMP_COMMIT: u64 = 0x07;
pub const EC_ACPI_MEM_TEMP_COMMIT_SELECT_MASK: u64 = bit(0);
pub const EC_ACPI_MEM_TEMP_COMMIT_ENABLE_MASK: u64 = bit(1);
pub const EC_ACPI_MEM_CHARGING_LIMIT: u64 = 0x08;
pub const EC_ACPI_MEM_CHARGING_LIMIT_STEP_MA: u64 = 64;
pub const EC_ACPI_MEM_CHARGING_LIMIT_DISABLED: u64 = 0xff;
pub const EC_ACPI_MEM_DEVICE_ORIENTATION: u64 = 0x09;
pub const EC_ACPI_MEM_TBMD_SHIFT: u64 = 0;
pub const EC_ACPI_MEM_TBMD_MASK: u64 = 0x1;
pub const EC_ACPI_MEM_DDPN_SHIFT: u64 = 1;
pub const EC_ACPI_MEM_DDPN_MASK: u64 = 0x7;
pub const EC_ACPI_MEM_DEVICE_FEATURES0: u64 = 0x0a;
pub const EC_ACPI_MEM_DEVICE_FEATURES1: u64 = 0x0b;
pub const EC_ACPI_MEM_DEVICE_FEATURES2: u64 = 0x0c;
pub const EC_ACPI_MEM_DEVICE_FEATURES3: u64 = 0x0d;
pub const EC_ACPI_MEM_DEVICE_FEATURES4: u64 = 0x0e;
pub const EC_ACPI_MEM_DEVICE_FEATURES5: u64 = 0x0f;
pub const EC_ACPI_MEM_DEVICE_FEATURES6: u64 = 0x10;
pub const EC_ACPI_MEM_DEVICE_FEATURES7: u64 = 0x11;
pub const EC_ACPI_MEM_BATTERY_INDEX: u64 = 0x12;
pub const EC_ACPI_MEM_USB_PORT_POWER: u64 = 0x13;
pub const EC_ACPI_MEM_MAPPED_BEGIN: u64 = 0x20;
pub const EC_ACPI_MEM_MAPPED_SIZE: u64 = 0xe0;
pub const EC_ACPI_MEM_VERSION_CURRENT: u64 = 2;
pub const __ec_align1: u64 = __packed;
pub const __ec_align2: u64 = __packed;
pub const __ec_align4: u64 = __packed;
pub const __ec_align_size1: u64 = __packed;
pub const __ec_align_offset1: u64 = __packed;
pub const __ec_align_offset2: u64 = __packed;
pub const __ec_todo_packed: u64 = __packed;
pub const EC_LPC_STATUS_TO_HOST: u64 = 0x01;
pub const EC_LPC_STATUS_FROM_HOST: u64 = 0x02;
pub const EC_LPC_STATUS_PROCESSING: u64 = 0x04;
pub const EC_LPC_STATUS_LAST_CMD: u64 = 0x08;
pub const EC_LPC_STATUS_BURST_MODE: u64 = 0x10;
pub const EC_LPC_STATUS_SCI_PENDING: u64 = 0x20;
pub const EC_LPC_STATUS_SMI_PENDING: u64 = 0x40;
pub const EC_LPC_STATUS_RESERVED: u64 = 0x80;
pub const EC_LPC_STATUS_BUSY_MASK: u64 = \;
pub const EC_HOST_ARGS_FLAG_FROM_HOST: u64 = 0x01;
pub const EC_HOST_ARGS_FLAG_TO_HOST: u64 = 0x02;
pub const EC_SPI_FRAME_START: u64 = 0xec;
pub const EC_SPI_PAST_END: u64 = 0xed;
pub const EC_SPI_RX_READY: u64 = 0xf8;
pub const EC_SPI_RECEIVING: u64 = 0xf9;
pub const EC_SPI_PROCESSING: u64 = 0xfa;
pub const EC_SPI_RX_BAD_DATA: u64 = 0xfb;
pub const EC_SPI_NOT_READY: u64 = 0xfc;
pub const EC_SPI_OLD_READY: u64 = 0xfd;
pub const EC_PROTO2_REQUEST_HEADER_BYTES: u64 = 3;
pub const EC_PROTO2_REQUEST_TRAILER_BYTES: u64 = 1;
pub const EC_PROTO2_REQUEST_OVERHEAD: u64 = (EC_PROTO2_REQUEST_HEADER_BYTES +	\;
pub const EC_PROTO2_RESPONSE_HEADER_BYTES: u64 = 2;
pub const EC_PROTO2_RESPONSE_TRAILER_BYTES: u64 = 1;
pub const EC_PROTO2_RESPONSE_OVERHEAD: u64 = (EC_PROTO2_RESPONSE_HEADER_BYTES +	\;
pub const EC_PROTO2_MAX_PARAM_SIZE: u64 = 0xfc;
pub const EC_PROTO2_MAX_REQUEST_SIZE: u64 = (EC_PROTO2_REQUEST_OVERHEAD +	\;
pub const EC_PROTO2_MAX_RESPONSE_SIZE: u64 = (EC_PROTO2_RESPONSE_OVERHEAD +	\;
pub const EC_COMMAND_PROTOCOL_3: u64 = 0xda;
pub const EC_HOST_REQUEST_VERSION: u64 = 3;
pub const EC_HOST_RESPONSE_VERSION: u64 = 3;
pub const EC_PACKET4_0_STRUCT_VERSION_MASK: u64 = 0x0f;
pub const EC_PACKET4_0_IS_RESPONSE_MASK: u64 = 0x10;
pub const EC_PACKET4_0_SEQ_NUM_SHIFT: u64 = 5;
pub const EC_PACKET4_0_SEQ_NUM_MASK: u64 = 0x60;
pub const EC_PACKET4_0_SEQ_DUP_MASK: u64 = 0x80;
pub const EC_PACKET4_1_COMMAND_VERSION_MASK: u64 = 0x1f;
pub const EC_PACKET4_1_DATA_CRC_PRESENT_MASK: u64 = 0x80;
pub const EC_CMD_PROTO_VERSION: u64 = 0x0000;
pub const EC_CMD_HELLO: u64 = 0x0001;
pub const EC_CMD_GET_VERSION: u64 = 0x0002;
pub const EC_CMD_READ_TEST: u64 = 0x0003;
pub const EC_CMD_GET_BUILD_INFO: u64 = 0x0004;
pub const EC_CMD_GET_CHIP_INFO: u64 = 0x0005;
pub const EC_CMD_GET_BOARD_VERSION: u64 = 0x0006;
pub const EC_CMD_READ_MEMMAP: u64 = 0x0007;
pub const EC_CMD_GET_CMD_VERSIONS: u64 = 0x0008;
pub const EC_CMD_GET_COMMS_STATUS: u64 = 0x0009;
pub const EC_CMD_TEST_PROTOCOL: u64 = 0x000A;
pub const EC_CMD_GET_PROTOCOL_INFO: u64 = 0x000B;
pub const EC_PROTOCOL_INFO_IN_PROGRESS_SUPPORTED: u64 = bit(0);
pub const EC_GSV_SET: u64 = 0x80000000;
pub const EC_GSV_PARAM_MASK: u64 = 0x00ffffff;
pub const EC_CMD_GSV_PAUSE_IN_S5: u64 = 0x000C;
pub const EC_CMD_GET_FEATURES: u64 = 0x000D;
pub const EC_CMD_GET_SKU_ID: u64 = 0x000E;
pub const EC_CMD_SET_SKU_ID: u64 = 0x000F;
pub const EC_CMD_FLASH_INFO: u64 = 0x0010;
pub const EC_VER_FLASH_INFO: u64 = 2;
pub const EC_FLASH_INFO_ERASE_TO_0: u64 = bit(0);
pub const EC_FLASH_INFO_SELECT_REQUIRED: u64 = bit(1);
pub const EC_CMD_FLASH_READ: u64 = 0x0011;
pub const EC_CMD_FLASH_WRITE: u64 = 0x0012;
pub const EC_VER_FLASH_WRITE: u64 = 1;
pub const EC_FLASH_WRITE_VER0_SIZE: u64 = 64;
pub const EC_CMD_FLASH_ERASE: u64 = 0x0013;
pub const EC_CMD_FLASH_PROTECT: u64 = 0x0015;
pub const EC_VER_FLASH_PROTECT: u64 = 1;
pub const EC_FLASH_PROTECT_RO_AT_BOOT: u64 = bit(0);
pub const EC_FLASH_PROTECT_RO_NOW: u64 = bit(1);
pub const EC_FLASH_PROTECT_ALL_NOW: u64 = bit(2);
pub const EC_FLASH_PROTECT_GPIO_ASSERTED: u64 = bit(3);
pub const EC_FLASH_PROTECT_ERROR_STUCK: u64 = bit(4);
pub const EC_FLASH_PROTECT_ERROR_INCONSISTENT: u64 = bit(5);
pub const EC_FLASH_PROTECT_ALL_AT_BOOT: u64 = bit(6);
pub const EC_FLASH_PROTECT_RW_AT_BOOT: u64 = bit(7);
pub const EC_FLASH_PROTECT_RW_NOW: u64 = bit(8);
pub const EC_FLASH_PROTECT_ROLLBACK_AT_BOOT: u64 = bit(9);
pub const EC_FLASH_PROTECT_ROLLBACK_NOW: u64 = bit(10);
pub const EC_CMD_FLASH_REGION_INFO: u64 = 0x0016;
pub const EC_VER_FLASH_REGION_INFO: u64 = 1;
pub const EC_FLASH_REGION_RW: u64 = EC_FLASH_REGION_ACTIVE;
pub const EC_CMD_VBNV_CONTEXT: u64 = 0x0017;
pub const EC_VER_VBNV_CONTEXT: u64 = 1;
pub const EC_VBNV_BLOCK_SIZE: u64 = 16;
pub const EC_CMD_FLASH_SPI_INFO: u64 = 0x0018;
pub const EC_CMD_FLASH_SELECT: u64 = 0x0019;
pub const EC_CMD_PWM_GET_FAN_TARGET_RPM: u64 = 0x0020;
pub const EC_CMD_PWM_SET_FAN_TARGET_RPM: u64 = 0x0021;
pub const EC_CMD_PWM_GET_KEYBOARD_BACKLIGHT: u64 = 0x0022;
pub const EC_CMD_PWM_SET_KEYBOARD_BACKLIGHT: u64 = 0x0023;
pub const EC_CMD_PWM_SET_FAN_DUTY: u64 = 0x0024;
pub const EC_CMD_PWM_SET_DUTY: u64 = 0x0025;
pub const EC_PWM_MAX_DUTY: u64 = 0xffff;
pub const EC_CMD_PWM_GET_DUTY: u64 = 0x0026;
pub const EC_CMD_PWM_GET_FAN_DUTY: u64 = 0x0027;
pub const EC_CMD_LIGHTBAR_CMD: u64 = 0x0028;
pub const LB_BATTERY_LEVELS: u64 = 4;
pub const EC_LB_PROG_LEN: u64 = 192;
pub const EC_CMD_LED_CONTROL: u64 = 0x0029;
pub const EC_LED_FLAGS_QUERY: u64 = bit(0);
pub const EC_LED_FLAGS_AUTO: u64 = bit(1);
pub const EC_CMD_VBOOT_HASH: u64 = 0x002A;
pub const EC_VBOOT_HASH_OFFSET_RO: u64 = 0xfffffffe;
pub const EC_VBOOT_HASH_OFFSET_ACTIVE: u64 = 0xfffffffd;
pub const EC_VBOOT_HASH_OFFSET_UPDATE: u64 = 0xfffffffc;
pub const EC_VBOOT_HASH_OFFSET_RW: u64 = EC_VBOOT_HASH_OFFSET_ACTIVE;
pub const EC_CMD_MOTION_SENSE_CMD: u64 = 0x002B;
pub const MOTIONSENSE_MODULE_FLAG_ACTIVE: u64 = bit(0);
pub const MOTIONSENSE_SENSOR_FLAG_PRESENT: u64 = bit(0);
pub const MOTIONSENSE_SENSOR_FLAG_FLUSH: u64 = bit(0);
pub const MOTIONSENSE_SENSOR_FLAG_TIMESTAMP: u64 = bit(1);
pub const MOTIONSENSE_SENSOR_FLAG_WAKEUP: u64 = bit(2);
pub const MOTIONSENSE_SENSOR_FLAG_TABLET_MODE: u64 = bit(3);
pub const MOTIONSENSE_SENSOR_FLAG_ODR: u64 = bit(4);
pub const EC_MOTION_SENSE_NO_VALUE: u64 = -1;
pub const EC_MOTION_SENSE_INVALID_CALIB_TEMP: u64 = 0x8000;
pub const MOTION_SENSE_SET_OFFSET: u64 = bit(0);
pub const MOTION_SENSE_DEFAULT_SCALE: u64 = bit(15);
pub const LID_ANGLE_UNRELIABLE: u64 = 500;
pub const EC_CMD_FORCE_LID_OPEN: u64 = 0x002C;
pub const EC_CMD_CONFIG_POWER_BUTTON: u64 = 0x002D;
pub const EC_CMD_USB_CHARGE_SET_MODE: u64 = 0x0030;
pub const EC_PSTORE_SIZE_MAX: u64 = 64;
pub const EC_CMD_PSTORE_INFO: u64 = 0x0040;
pub const EC_CMD_PSTORE_READ: u64 = 0x0041;
pub const EC_CMD_PSTORE_WRITE: u64 = 0x0042;
pub const EC_CMD_RTC_GET_VALUE: u64 = 0x0044;
pub const EC_CMD_RTC_GET_ALARM: u64 = 0x0045;
pub const EC_CMD_RTC_SET_VALUE: u64 = 0x0046;
pub const EC_CMD_RTC_SET_ALARM: u64 = 0x0047;
pub const EC_RTC_ALARM_CLEAR: u64 = 0;
pub const EC_PORT80_SIZE_MAX: u64 = 32;
pub const EC_CMD_PORT80_LAST_BOOT: u64 = 0x0048;
pub const EC_CMD_PORT80_READ: u64 = 0x0048;
pub const EC_VSTORE_SLOT_SIZE: u64 = 64;
pub const EC_VSTORE_SLOT_MAX: u64 = 32;
pub const EC_CMD_VSTORE_INFO: u64 = 0x0049;
pub const EC_CMD_VSTORE_READ: u64 = 0x004A;
pub const EC_CMD_VSTORE_WRITE: u64 = 0x004B;
pub const EC_CMD_THERMAL_SET_THRESHOLD: u64 = 0x0050;
pub const EC_CMD_THERMAL_GET_THRESHOLD: u64 = 0x0051;
pub const EC_CMD_THERMAL_AUTO_FAN_CTRL: u64 = 0x0052;
pub const EC_CMD_TMP006_GET_CALIBRATION: u64 = 0x0053;
pub const EC_CMD_TMP006_SET_CALIBRATION: u64 = 0x0054;
pub const EC_CMD_TMP006_GET_RAW: u64 = 0x0055;
pub const EC_CMD_MKBP_STATE: u64 = 0x0060;
pub const EC_CMD_MKBP_INFO: u64 = 0x0061;
pub const EC_CMD_MKBP_SIMULATE_KEY: u64 = 0x0062;
pub const EC_CMD_GET_KEYBOARD_ID: u64 = 0x0063;
pub const EC_CMD_MKBP_SET_CONFIG: u64 = 0x0064;
pub const EC_CMD_MKBP_GET_CONFIG: u64 = 0x0065;
pub const EC_CMD_KEYSCAN_SEQ_CTRL: u64 = 0x0066;
pub const EC_CMD_GET_NEXT_EVENT: u64 = 0x0067;
pub const EC_MKBP_HAS_MORE_EVENTS_SHIFT: u64 = 7;
pub const EC_MKBP_HAS_MORE_EVENTS: u64 = bit(EC_MKBP_HAS_MORE_EVENTS_SHIFT);
pub const EC_MKBP_EVENT_TYPE_MASK: u64 = (bit(EC_MKBP_HAS_MORE_EVENTS_SHIFT) - 1);
pub const EC_MKBP_POWER_BUTTON: u64 = 0;
pub const EC_MKBP_VOL_UP: u64 = 1;
pub const EC_MKBP_VOL_DOWN: u64 = 2;
pub const EC_MKBP_RECOVERY: u64 = 3;
pub const EC_MKBP_BRI_UP: u64 = 4;
pub const EC_MKBP_BRI_DOWN: u64 = 5;
pub const EC_MKBP_SCREEN_LOCK: u64 = 6;
pub const EC_MKBP_LID_OPEN: u64 = 0;
pub const EC_MKBP_TABLET_MODE: u64 = 1;
pub const EC_MKBP_BASE_ATTACHED: u64 = 2;
pub const EC_MKBP_FRONT_PROXIMITY: u64 = 3;
pub const EC_CMD_KEYBOARD_FACTORY_TEST: u64 = 0x0068;
pub const EC_MKBP_FP_ENROLL_PROGRESS_OFFSET: u64 = 4;
pub const EC_MKBP_FP_MATCH_IDX_OFFSET: u64 = 12;
pub const EC_MKBP_FP_MATCH_IDX_MASK: u64 = 0x0000F000;
pub const EC_MKBP_FP_ENROLL: u64 = bit(27);
pub const EC_MKBP_FP_MATCH: u64 = bit(28);
pub const EC_MKBP_FP_FINGER_DOWN: u64 = bit(29);
pub const EC_MKBP_FP_FINGER_UP: u64 = bit(30);
pub const EC_MKBP_FP_IMAGE_READY: u64 = bit(31);
pub const EC_MKBP_FP_ERR_ENROLL_OK: u64 = 0;
pub const EC_MKBP_FP_ERR_ENROLL_LOW_QUALITY: u64 = 1;
pub const EC_MKBP_FP_ERR_ENROLL_IMMOBILE: u64 = 2;
pub const EC_MKBP_FP_ERR_ENROLL_LOW_COVERAGE: u64 = 3;
pub const EC_MKBP_FP_ERR_ENROLL_INTERNAL: u64 = 5;
pub const EC_MKBP_FP_ERR_ENROLL_PROBLEM_MASK: u64 = 1;
pub const EC_MKBP_FP_ERR_MATCH_NO: u64 = 0;
pub const EC_MKBP_FP_ERR_MATCH_NO_INTERNAL: u64 = 6;
pub const EC_MKBP_FP_ERR_MATCH_NO_TEMPLATES: u64 = 7;
pub const EC_MKBP_FP_ERR_MATCH_NO_LOW_QUALITY: u64 = 2;
pub const EC_MKBP_FP_ERR_MATCH_NO_LOW_COVERAGE: u64 = 4;
pub const EC_MKBP_FP_ERR_MATCH_YES: u64 = 1;
pub const EC_MKBP_FP_ERR_MATCH_YES_UPDATED: u64 = 3;
pub const EC_MKBP_FP_ERR_MATCH_YES_UPDATE_FAILED: u64 = 5;
pub const EC_CMD_TEMP_SENSOR_GET_INFO: u64 = 0x0070;
pub const EC_CMD_HOST_EVENT_GET_B: u64 = 0x0087;
pub const EC_CMD_HOST_EVENT_GET_SMI_MASK: u64 = 0x0088;
pub const EC_CMD_HOST_EVENT_GET_SCI_MASK: u64 = 0x0089;
pub const EC_CMD_HOST_EVENT_GET_WAKE_MASK: u64 = 0x008D;
pub const EC_CMD_HOST_EVENT_SET_SMI_MASK: u64 = 0x008A;
pub const EC_CMD_HOST_EVENT_SET_SCI_MASK: u64 = 0x008B;
pub const EC_CMD_HOST_EVENT_CLEAR: u64 = 0x008C;
pub const EC_CMD_HOST_EVENT_SET_WAKE_MASK: u64 = 0x008E;
pub const EC_CMD_HOST_EVENT_CLEAR_B: u64 = 0x008F;
pub const EC_CMD_HOST_EVENT: u64 = 0x00A4;
pub const EC_CMD_SWITCH_ENABLE_BKLIGHT: u64 = 0x0090;
pub const EC_CMD_SWITCH_ENABLE_WIRELESS: u64 = 0x0091;
pub const EC_VER_SWITCH_ENABLE_WIRELESS: u64 = 1;
pub const EC_CMD_GPIO_SET: u64 = 0x0092;
pub const EC_CMD_GPIO_GET: u64 = 0x0093;
pub const EC_CMD_I2C_READ: u64 = 0x0094;
pub const EC_CMD_I2C_WRITE: u64 = 0x0095;
pub const EC_CMD_CHARGE_CONTROL: u64 = 0x0096;
pub const EC_VER_CHARGE_CONTROL: u64 = 3;
pub const EC_CHARGE_MODE_TEXT: u64 = \;
pub const EC_CMD_CONSOLE_SNAPSHOT: u64 = 0x0097;
pub const EC_CMD_CONSOLE_READ: u64 = 0x0098;
pub const EC_CMD_BATTERY_CUT_OFF: u64 = 0x0099;
pub const EC_BATTERY_CUTOFF_FLAG_AT_SHUTDOWN: u64 = bit(0);
pub const EC_CMD_USB_MUX: u64 = 0x009A;
pub const EC_CMD_LDO_SET: u64 = 0x009B;
pub const EC_CMD_LDO_GET: u64 = 0x009C;
pub const EC_CMD_POWER_INFO: u64 = 0x009D;
pub const EC_CMD_I2C_PASSTHRU: u64 = 0x009E;
pub const EC_I2C_FLAG_READ: u64 = bit(15);
pub const EC_I2C_ADDR_MASK: u64 = 0x3ff;
pub const EC_I2C_STATUS_NAK: u64 = bit(0);
pub const EC_I2C_STATUS_TIMEOUT: u64 = bit(1);
pub const EC_I2C_STATUS_ERROR: u64 = (EC_I2C_STATUS_NAK | EC_I2C_STATUS_TIMEOUT);
pub const EC_CMD_HANG_DETECT: u64 = 0x009F;
pub const EC_HANG_DETECT_MIN_TIMEOUT: u64 = 5;
pub const EC_HANG_DETECT_MAX_TIMEOUT: u64 = 65535;
pub const EC_CMD_CHARGE_STATE: u64 = 0x00A0;
pub const EC_CMD_CHARGE_CURRENT_LIMIT: u64 = 0x00A1;
pub const EC_CMD_EXTERNAL_POWER_LIMIT: u64 = 0x00A2;
pub const EC_POWER_LIMIT_NONE: u64 = 0xffff;
pub const EC_CMD_OVERRIDE_DEDICATED_CHARGER_LIMIT: u64 = 0x00A3;
pub const EC_CMD_HIBERNATION_DELAY: u64 = 0x00A8;
pub const EC_CMD_HOST_SLEEP_EVENT: u64 = 0x00A9;
pub const EC_HOST_SLEEP_TIMEOUT_DEFAULT: u64 = 0;
pub const EC_HOST_SLEEP_TIMEOUT_INFINITE: u64 = 0xFFFF;
pub const EC_HOST_RESUME_SLEEP_TIMEOUT: u64 = 0x80000000;
pub const EC_HOST_RESUME_SLEEP_TRANSITIONS_MASK: u64 = 0x7FFFFFFF;
pub const EC_CMD_DEVICE_EVENT: u64 = 0x00AA;
pub const EC_CMD_SB_READ_WORD: u64 = 0x00B0;
pub const EC_CMD_SB_WRITE_WORD: u64 = 0x00B1;
pub const EC_CMD_SB_READ_BLOCK: u64 = 0x00B2;
pub const EC_CMD_SB_WRITE_BLOCK: u64 = 0x00B3;
pub const EC_CMD_BATTERY_VENDOR_PARAM: u64 = 0x00B4;
pub const EC_CMD_SB_FW_UPDATE: u64 = 0x00B5;
pub const SB_FW_UPDATE_CMD_WRITE_BLOCK_SIZE: u64 = 32;
pub const SB_FW_UPDATE_CMD_STATUS_SIZE: u64 = 2;
pub const SB_FW_UPDATE_CMD_INFO_SIZE: u64 = 8;
pub const EC_CMD_ENTERING_MODE: u64 = 0x00B6;
pub const VBOOT_MODE_NORMAL: u64 = 0;
pub const VBOOT_MODE_DEVELOPER: u64 = 1;
pub const VBOOT_MODE_RECOVERY: u64 = 2;
pub const EC_CMD_I2C_PASSTHRU_PROTECT: u64 = 0x00B7;
pub const EC_CEC_MAX_PORTS: u64 = 16;
pub const MAX_CEC_MSG_LEN: u64 = 16;
pub const EC_CMD_CEC_WRITE_MSG: u64 = 0x00B8;
pub const EC_CMD_CEC_READ_MSG: u64 = 0x00B9;
pub const EC_CMD_CEC_SET: u64 = 0x00BA;
pub const EC_CMD_CEC_GET: u64 = 0x00BB;
pub const EC_CMD_CEC_PORT_COUNT: u64 = 0x00C1;
pub const EC_CMD_EC_CODEC: u64 = 0x00BC;
pub const EC_CMD_EC_CODEC_DMIC: u64 = 0x00BD;
pub const EC_CMD_EC_CODEC_I2S_RX: u64 = 0x00BE;
pub const EC_CMD_EC_CODEC_WOV: u64 = 0x00BF;
pub const EC_CMD_REBOOT_EC: u64 = 0x00D2;
pub const EC_REBOOT_FLAG_RESERVED0: u64 = bit(0);
pub const EC_REBOOT_FLAG_ON_AP_SHUTDOWN: u64 = bit(1);
pub const EC_REBOOT_FLAG_SWITCH_RW_SLOT: u64 = bit(2);
pub const EC_CMD_GET_PANIC_INFO: u64 = 0x00D3;
pub const EC_CMD_REBOOT: u64 = 0x00D1;
pub const EC_CMD_RESEND_RESPONSE: u64 = 0x00DB;
pub const EC_CMD_VERSION0: u64 = 0x00DC;
pub const EC_CMD_PD_EXCHANGE_STATUS: u64 = 0x0100;
pub const EC_VER_PD_EXCHANGE_STATUS: u64 = 2;
pub const EC_STATUS_HIBERNATING: u64 = bit(0);
pub const PD_STATUS_HOST_EVENT: u64 = bit(0);
pub const PD_STATUS_IN_RW: u64 = bit(1);
pub const PD_STATUS_JUMPED_TO_IMAGE: u64 = bit(2);
pub const PD_STATUS_TCPC_ALERT_0: u64 = bit(3);
pub const PD_STATUS_TCPC_ALERT_1: u64 = bit(4);
pub const PD_STATUS_TCPC_ALERT_2: u64 = bit(5);
pub const PD_STATUS_TCPC_ALERT_3: u64 = bit(6);
pub const PD_STATUS_EC_INT_ACTIVE: u64 = (PD_STATUS_TCPC_ALERT_0 | \;
pub const EC_CMD_PD_HOST_EVENT_STATUS: u64 = 0x0104;
pub const PD_EVENT_UPDATE_DEVICE: u64 = bit(0);
pub const PD_EVENT_POWER_CHANGE: u64 = bit(1);
pub const PD_EVENT_IDENTITY_RECEIVED: u64 = bit(2);
pub const PD_EVENT_DATA_SWAP: u64 = bit(3);
pub const PD_EVENT_TYPEC: u64 = bit(4);
pub const PD_EVENT_PPM: u64 = bit(5);
pub const PD_EVENT_INIT: u64 = bit(6);
pub const EC_CMD_USB_PD_CONTROL: u64 = 0x0101;
pub const PD_CTRL_RESP_ENABLED_COMMS: u64 = bit(0);
pub const PD_CTRL_RESP_ENABLED_CONNECTED: u64 = bit(1);
pub const PD_CTRL_RESP_ENABLED_PD_CAPABLE: u64 = bit(2);
pub const PD_CTRL_RESP_ROLE_POWER: u64 = bit(0);
pub const PD_CTRL_RESP_ROLE_DATA: u64 = bit(1);
pub const PD_CTRL_RESP_ROLE_VCONN: u64 = bit(2);
pub const PD_CTRL_RESP_ROLE_DR_POWER: u64 = bit(3);
pub const PD_CTRL_RESP_ROLE_DR_DATA: u64 = bit(4);
pub const PD_CTRL_RESP_ROLE_USB_COMM: u64 = bit(5);
pub const PD_CTRL_RESP_ROLE_EXT_POWERED: u64 = bit(6);
pub const USBC_PD_CC_NONE: u64 = 0;
pub const USBC_PD_CC_NO_UFP: u64 = 1;
pub const USBC_PD_CC_AUDIO_ACC: u64 = 2;
pub const USBC_PD_CC_DEBUG_ACC: u64 = 3;
pub const USBC_PD_CC_UFP_ATTACHED: u64 = 4;
pub const USBC_PD_CC_DFP_ATTACHED: u64 = 5;
pub const USB_PD_CTRL_ACTIVE_CABLE: u64 = bit(0);
pub const USB_PD_CTRL_OPTICAL_CABLE: u64 = bit(1);
pub const USB_PD_CTRL_TBT_LEGACY_ADAPTER: u64 = bit(2);
pub const USB_PD_CTRL_ACTIVE_LINK_UNIDIR: u64 = bit(3);
pub const EC_CMD_USB_PD_PORTS: u64 = 0x0102;
pub const EC_USB_PD_MAX_PORTS: u64 = 8;
pub const EC_CMD_USB_PD_POWER_INFO: u64 = 0x0103;
pub const PD_POWER_CHARGING_PORT: u64 = 0xff;
pub const EC_CMD_CHARGE_PORT_COUNT: u64 = 0x0105;
pub const EC_CMD_USB_PD_FW_UPDATE: u64 = 0x0110;
pub const EC_CMD_USB_PD_RW_HASH_ENTRY: u64 = 0x0111;
pub const PD_RW_HASH_SIZE: u64 = 20;
pub const EC_CMD_USB_PD_DEV_INFO: u64 = 0x0112;
pub const EC_CMD_USB_PD_DISCOVERY: u64 = 0x0113;
pub const EC_CMD_PD_CHARGE_PORT_OVERRIDE: u64 = 0x0114;
pub const EC_CMD_PD_GET_LOG_ENTRY: u64 = 0x0115;
pub const PD_LOG_TIMESTAMP_SHIFT: u64 = 10;
pub const PD_LOG_SIZE_MASK: u64 = 0x1f;
pub const PD_LOG_PORT_MASK: u64 = 0xe0;
pub const PD_LOG_PORT_SHIFT: u64 = 5;
pub const PD_EVENT_MCU_BASE: u64 = 0x00;
pub const PD_EVENT_MCU_CHARGE: u64 = (PD_EVENT_MCU_BASE+0);
pub const PD_EVENT_MCU_CONNECT: u64 = (PD_EVENT_MCU_BASE+1);
pub const PD_EVENT_MCU_BOARD_CUSTOM: u64 = (PD_EVENT_MCU_BASE+2);
pub const PD_EVENT_ACC_BASE: u64 = 0x20;
pub const PD_EVENT_ACC_RW_FAIL: u64 = (PD_EVENT_ACC_BASE+0);
pub const PD_EVENT_ACC_RW_ERASE: u64 = (PD_EVENT_ACC_BASE+1);
pub const PD_EVENT_PS_BASE: u64 = 0x40;
pub const PD_EVENT_PS_FAULT: u64 = (PD_EVENT_PS_BASE+0);
pub const PD_EVENT_VIDEO_BASE: u64 = 0x60;
pub const PD_EVENT_VIDEO_DP_MODE: u64 = (PD_EVENT_VIDEO_BASE+0);
pub const PD_EVENT_VIDEO_CODEC: u64 = (PD_EVENT_VIDEO_BASE+1);
pub const PD_EVENT_NO_ENTRY: u64 = 0xff;
pub const CHARGE_FLAGS_DUAL_ROLE: u64 = bit(15);
pub const CHARGE_FLAGS_DELAYED_OVERRIDE: u64 = bit(14);
pub const CHARGE_FLAGS_OVERRIDE: u64 = bit(13);
pub const CHARGE_FLAGS_TYPE_SHIFT: u64 = 3;
pub const CHARGE_FLAGS_TYPE_MASK: u64 = (0xf << CHARGE_FLAGS_TYPE_SHIFT);
pub const CHARGE_FLAGS_ROLE_MASK: u64 = (7 <<  0);
pub const PS_FAULT_OCP: u64 = 1;
pub const PS_FAULT_FAST_OCP: u64 = 2;
pub const PS_FAULT_OVP: u64 = 3;
pub const PS_FAULT_DISCH: u64 = 4;
pub const EC_CMD_USB_PD_GET_AMODE: u64 = 0x0116;
pub const EC_CMD_USB_PD_SET_AMODE: u64 = 0x0117;
pub const EC_CMD_PD_WRITE_LOG_ENTRY: u64 = 0x0118;
pub const EC_CMD_PD_CONTROL: u64 = 0x0119;
pub const EC_CMD_USB_PD_MUX_INFO: u64 = 0x011A;
pub const USB_PD_MUX_NONE: u64 = 0;
pub const USB_PD_MUX_USB_ENABLED: u64 = bit(0);
pub const USB_PD_MUX_DP_ENABLED: u64 = bit(1);
pub const USB_PD_MUX_POLARITY_INVERTED: u64 = bit(2);
pub const USB_PD_MUX_HPD_IRQ: u64 = bit(3);
pub const USB_PD_MUX_HPD_LVL: u64 = bit(4);
pub const USB_PD_MUX_SAFE_MODE: u64 = bit(5);
pub const USB_PD_MUX_TBT_COMPAT_ENABLED: u64 = bit(6);
pub const USB_PD_MUX_USB4_ENABLED: u64 = bit(7);
pub const EC_CMD_PD_CHIP_INFO: u64 = 0x011B;
pub const EC_CMD_RWSIG_CHECK_STATUS: u64 = 0x011C;
pub const EC_CMD_RWSIG_ACTION: u64 = 0x011D;
pub const EC_CMD_EFS_VERIFY: u64 = 0x011E;
pub const EC_CMD_GET_CROS_BOARD_INFO: u64 = 0x011F;
pub const EC_CMD_SET_CROS_BOARD_INFO: u64 = 0x0120;
pub const CBI_GET_RELOAD: u64 = bit(0);
pub const CBI_SET_NO_SYNC: u64 = bit(0);
pub const CBI_SET_INIT: u64 = bit(1);
pub const EC_CMD_GET_UPTIME_INFO: u64 = 0x0121;
pub const EC_CMD_ADD_ENTROPY: u64 = 0x0122;
pub const EC_CMD_ADC_READ: u64 = 0x0123;
pub const EC_CMD_ROLLBACK_INFO: u64 = 0x0124;
pub const EC_CMD_AP_RESET: u64 = 0x0125;
pub const EC_CMD_PCHG_COUNT: u64 = 0x0134;
pub const EC_PCHG_MAX_PORTS: u64 = 8;
pub const EC_CMD_PCHG: u64 = 0x0135;
pub const EC_CMD_PCHG_UPDATE: u64 = 0x0136;
pub const EC_MKBP_PCHG_PORT_SHIFT: u64 = 28;
pub const EC_MKBP_PCHG_UPDATE_OPENED: u64 = bit(0);
pub const EC_MKBP_PCHG_WRITE_COMPLETE: u64 = bit(1);
pub const EC_MKBP_PCHG_UPDATE_CLOSED: u64 = bit(2);
pub const EC_MKBP_PCHG_UPDATE_ERROR: u64 = bit(3);
pub const EC_MKBP_PCHG_DEVICE_EVENT: u64 = bit(4);
pub const EC_CMD_REGULATOR_GET_INFO: u64 = 0x012C;
pub const EC_REGULATOR_NAME_MAX_LEN: u64 = 16;
pub const EC_REGULATOR_VOLTAGE_MAX_COUNT: u64 = 16;
pub const EC_CMD_REGULATOR_ENABLE: u64 = 0x012D;
pub const EC_CMD_REGULATOR_IS_ENABLED: u64 = 0x012E;
pub const EC_CMD_REGULATOR_SET_VOLTAGE: u64 = 0x012F;
pub const EC_CMD_REGULATOR_GET_VOLTAGE: u64 = 0x0130;
pub const EC_CMD_TYPEC_DISCOVERY: u64 = 0x0131;
pub const EC_CMD_TYPEC_CONTROL: u64 = 0x0132;
pub const VDO_MAX_SIZE: u64 = 7;
pub const EC_CMD_TYPEC_STATUS: u64 = 0x0133;
pub const PD_STATUS_EVENT_SOP_DISC_DONE: u64 = bit(0);
pub const PD_STATUS_EVENT_SOP_PRIME_DISC_DONE: u64 = bit(1);
pub const PD_STATUS_EVENT_HARD_RESET: u64 = bit(2);
pub const PD_STATUS_EVENT_DISCONNECTED: u64 = bit(3);
pub const PD_STATUS_EVENT_MUX_0_SET_DONE: u64 = bit(4);
pub const PD_STATUS_EVENT_MUX_1_SET_DONE: u64 = bit(5);
pub const PD_STATUS_EVENT_VDM_REQ_REPLY: u64 = bit(6);
pub const PD_STATUS_EVENT_VDM_REQ_FAILED: u64 = bit(7);
pub const PD_STATUS_EVENT_VDM_ATTENTION: u64 = bit(8);
pub const EC_CMD_TYPEC_VDM_RESPONSE: u64 = 0x013C;
pub const EC_CMD_UCSI_PPM_SET: u64 = 0x0140;
pub const EC_CMD_UCSI_PPM_GET: u64 = 0x0141;
pub const EC_CMD_CR51_BASE: u64 = 0x0300;
pub const EC_CMD_CR51_LAST: u64 = 0x03FF;
pub const EC_CMD_FP_PASSTHRU: u64 = 0x0400;
pub const EC_FP_FLAG_NOT_COMPLETE: u64 = 0x1;
pub const EC_CMD_FP_MODE: u64 = 0x0402;
pub const FP_MODE_DEEPSLEEP: u64 = bit(0);
pub const FP_MODE_FINGER_DOWN: u64 = bit(1);
pub const FP_MODE_FINGER_UP: u64 = bit(2);
pub const FP_MODE_CAPTURE: u64 = bit(3);
pub const FP_MODE_ENROLL_SESSION: u64 = bit(4);
pub const FP_MODE_ENROLL_IMAGE: u64 = bit(5);
pub const FP_MODE_MATCH: u64 = bit(6);
pub const FP_MODE_RESET_SENSOR: u64 = bit(7);
pub const FP_MODE_DONT_CHANGE: u64 = bit(31);
pub const FP_VALID_MODES: u64 = (FP_MODE_DEEPSLEEP      | \;
pub const FP_MODE_CAPTURE_TYPE_SHIFT: u64 = 28;
pub const FP_MODE_CAPTURE_TYPE_MASK: u64 = (0x7 << FP_MODE_CAPTURE_TYPE_SHIFT);
pub const EC_CMD_FP_INFO: u64 = 0x0403;
pub const FP_ERROR_DEAD_PIXELS_UNKNOWN: u64 = (0x3FF);
pub const FP_ERROR_NO_IRQ: u64 = bit(12);
pub const FP_ERROR_SPI_COMM: u64 = bit(13);
pub const FP_ERROR_BAD_HWID: u64 = bit(14);
pub const FP_ERROR_INIT_FAIL: u64 = bit(15);
pub const EC_CMD_FP_FRAME: u64 = 0x0404;
pub const FP_FRAME_INDEX_SHIFT: u64 = 28;
pub const FP_FRAME_INDEX_RAW_IMAGE: u64 = 0;
pub const FP_FRAME_INDEX_TEMPLATE: u64 = 1;
pub const FP_FRAME_OFFSET_MASK: u64 = 0x0FFFFFFF;
pub const FP_TEMPLATE_FORMAT_VERSION: u64 = 3;
pub const FP_CONTEXT_NONCE_BYTES: u64 = 12;
pub const FP_CONTEXT_USERID_WORDS: u64 = (32 / sizeof(uint32_t));
pub const FP_CONTEXT_TAG_BYTES: u64 = 16;
pub const FP_CONTEXT_SALT_BYTES: u64 = 16;
pub const FP_CONTEXT_TPM_BYTES: u64 = 32;
pub const EC_CMD_FP_TEMPLATE: u64 = 0x0405;
pub const FP_TEMPLATE_COMMIT: u64 = 0x80000000;
pub const EC_CMD_FP_CONTEXT: u64 = 0x0406;
pub const EC_CMD_FP_STATS: u64 = 0x0407;
pub const FPSTATS_CAPTURE_INV: u64 = bit(0);
pub const FPSTATS_MATCHING_INV: u64 = bit(1);
pub const EC_CMD_FP_SEED: u64 = 0x0408;
pub const EC_CMD_FP_ENC_STATUS: u64 = 0x0409;
pub const FP_ENC_STATUS_SEED_SET: u64 = bit(0);
pub const EC_CMD_TP_SELF_TEST: u64 = 0x0500;
pub const EC_CMD_TP_FRAME_INFO: u64 = 0x0501;
pub const EC_CMD_TP_FRAME_SNAPSHOT: u64 = 0x0502;
pub const EC_CMD_TP_FRAME_GET: u64 = 0x0503;
pub const EC_COMM_TEXT_MAX: u64 = 8;
pub const EC_CMD_BATTERY_GET_STATIC: u64 = 0x0600;
pub const EC_CMD_BATTERY_GET_DYNAMIC: u64 = 0x0601;
pub const EC_CMD_CHARGER_CONTROL: u64 = 0x0602;
pub const EC_CMD_USB_PD_MUX_ACK: u64 = 0x0603;
pub const EC_CMD_BOARD_SPECIFIC_BASE: u64 = 0x3E00;
pub const EC_CMD_BOARD_SPECIFIC_LAST: u64 = 0x3FFF;
pub const EC_HOST_PARAM_SIZE: u64 = EC_PROTO2_MAX_PARAM_SIZE;
pub const EC_LPC_ADDR_OLD_PARAM: u64 = EC_HOST_CMD_REGION1;
pub const EC_OLD_PARAM_SIZE: u64 = EC_HOST_CMD_REGION_SIZE;
// enum ec_status {
// enum host_event_code {
// struct ec_lpc_host_args {
// struct ec_host_request {
// struct ec_host_response {
// struct ec_host_request4 {
// struct ec_host_response4 {
// struct ec_response_proto_version {
// struct ec_params_hello {
// struct ec_response_hello {
// enum ec_current_image {
// struct ec_response_get_version {
// struct ec_params_read_test {
// struct ec_response_read_test {
// struct ec_response_get_chip_info {
// struct ec_response_board_version {
// struct ec_params_read_memmap {
// struct ec_params_get_cmd_versions {
// struct ec_params_get_cmd_versions_v1 {
// struct ec_response_get_cmd_versions {
// enum ec_comms_status {
// struct ec_response_get_comms_status {
// struct ec_params_test_protocol {
// struct ec_response_test_protocol {
// struct ec_response_get_protocol_info {
// struct ec_params_get_set_value {
// struct ec_response_get_set_value {
// enum ec_feature_code {
// struct ec_response_get_features {
// struct ec_sku_id_info {
// struct ec_response_flash_info {
// struct ec_response_flash_info_1 {
// struct ec_params_flash_info_2 {
// struct ec_flash_bank {
// struct ec_response_flash_info_2 {
// struct ec_params_flash_read {
// struct ec_params_flash_write {
// struct ec_params_flash_erase {
// enum ec_flash_erase_cmd {
// struct ec_params_flash_erase_v1 {
// struct ec_params_flash_protect {
// struct ec_response_flash_protect {
// enum ec_flash_region {
// struct ec_params_flash_region_info {
// struct ec_response_flash_region_info {
// enum ec_vbnvcontext_op {
// struct ec_params_vbnvcontext {
// struct ec_response_vbnvcontext {
// struct ec_response_flash_spi_info {
// struct ec_params_flash_select {
// struct ec_response_pwm_get_fan_rpm {
// struct ec_params_pwm_set_fan_target_rpm_v0 {
// struct ec_params_pwm_set_fan_target_rpm_v1 {
// struct ec_response_pwm_get_keyboard_backlight {
// struct ec_params_pwm_set_keyboard_backlight {
// struct ec_params_pwm_set_fan_duty_v0 {
// struct ec_params_pwm_set_fan_duty_v1 {
// enum ec_pwm_type {
// struct ec_params_pwm_set_duty {
// struct ec_params_pwm_get_duty {
// struct ec_response_pwm_get_duty {
// struct ec_params_pwm_get_fan_duty {
// struct ec_response_pwm_get_fan_duty {
// struct rgb_s {
// struct lightbar_params_v0 {
// struct lightbar_params_v1 {
// struct lightbar_params_v2_timing {
// struct lightbar_params_v2_tap {
// struct lightbar_params_v2_oscillation {
// struct lightbar_params_v2_brightness {
// struct lightbar_params_v2_thresholds {
// struct lightbar_params_v2_colors {
// struct lightbar_params_v3 {
// struct lightbar_program {
// struct lightbar_program_ex {
// struct ec_params_lightbar {
// struct ec_response_lightbar {
// enum lightbar_command {
// enum ec_led_id {
// enum ec_led_colors {
// struct ec_params_led_control {
// struct ec_response_led_control {
// struct ec_params_vboot_hash {
// struct ec_response_vboot_hash {
// enum ec_vboot_hash_cmd {
// enum ec_vboot_hash_type {
// enum ec_vboot_hash_status {
// enum motionsense_command {
// enum motionsensor_type {
// enum motionsensor_location {
// enum motionsensor_chip {
// enum motionsensor_orientation {
// struct ec_response_activity_data {
// struct ec_response_motion_sensor_data {
// struct ec_response_motion_sense_fifo_info {
// struct ec_response_motion_sense_fifo_data {
// enum motionsensor_activity {
// struct ec_motion_sense_activity {
// enum motionsense_spoof_mode {
// struct ec_params_motion_sense {
// struct ec_response_motion_sense {
// struct ec_params_force_lid_open {
// enum ec_config_power_button_flags {
// struct ec_params_config_power_button {
// struct ec_params_usb_charge_set_mode {
// struct ec_response_pstore_info {
// struct ec_params_pstore_read {
// struct ec_params_pstore_write {
// struct ec_params_rtc {
// struct ec_response_rtc {
// enum ec_port80_subcmd {
// struct ec_params_port80_read {
// struct ec_response_port80_read {
// struct ec_response_port80_last_boot {
// struct ec_response_vstore_info {
// struct ec_params_vstore_read {
// struct ec_response_vstore_read {
// struct ec_params_vstore_write {
// struct ec_params_thermal_set_threshold {
// struct ec_params_thermal_get_threshold {
// struct ec_response_thermal_get_threshold {
// enum ec_temp_thresholds {
// struct ec_thermal_config {
// struct ec_params_thermal_get_threshold_v1 {
// struct ec_params_thermal_set_threshold_v1 {
// enum ec_auto_fan_ctrl_cmd {
// struct ec_params_auto_fan_ctrl_v1 {
// struct ec_params_auto_fan_ctrl_v2 {
// struct ec_response_auto_fan_control {
// struct ec_params_tmp006_get_calibration {
// struct ec_response_tmp006_get_calibration_v0 {
// struct ec_params_tmp006_set_calibration_v0 {
// struct ec_response_tmp006_get_calibration_v1 {
// struct ec_params_tmp006_set_calibration_v1 {
// struct ec_params_tmp006_get_raw {
// struct ec_response_tmp006_get_raw {
// struct ec_response_mkbp_info {
// struct ec_params_mkbp_info {
// enum ec_mkbp_info_type {
// struct ec_params_mkbp_simulate_key {
// struct ec_response_keyboard_id {
// enum keyboard_id {
// enum mkbp_config_flags {
// enum mkbp_config_valid {
// struct ec_mkbp_config {
// struct ec_params_mkbp_set_config {
// struct ec_response_mkbp_get_config {
// enum ec_keyscan_seq_cmd {
// enum ec_collect_flags {
// struct ec_collect_item {
// struct ec_params_keyscan_seq_ctrl {
// struct ec_result_keyscan_seq_ctrl {
// enum ec_mkbp_event {
// struct ec_response_get_next_event {
// struct ec_response_get_next_event_v1 {
// struct ec_response_get_next_event_v3 {
// struct ec_response_keyboard_factory_test {
// struct ec_params_temp_sensor_get_info {
// struct ec_response_temp_sensor_get_info {
// struct ec_params_host_event_mask {
// struct ec_response_host_event_mask {
// struct ec_params_host_event {
// struct ec_response_host_event {
// enum ec_host_event_action {
// enum ec_host_event_mask_type {
// struct ec_params_switch_enable_backlight {
// struct ec_params_switch_enable_wireless_v0 {
// struct ec_params_switch_enable_wireless_v1 {
// struct ec_response_switch_enable_wireless_v1 {
// struct ec_params_gpio_set {
// struct ec_params_gpio_get {
// struct ec_response_gpio_get {
// struct ec_params_gpio_get_v1 {
// struct ec_response_gpio_get_v1 {
// enum gpio_get_subcmd {
// struct ec_params_i2c_read {
// struct ec_response_i2c_read {
// struct ec_params_i2c_write {
// enum ec_charge_control_mode {
// enum ec_charge_control_cmd {
// enum ec_charge_control_flag {
// struct ec_params_charge_control {
// struct ec_response_charge_control {
// enum ec_console_read_subcmd {
// struct ec_params_console_read_v1 {
// struct ec_params_battery_cutoff {
// struct ec_params_usb_mux {
// enum ec_ldo_state {
// struct ec_params_ldo_set {
// struct ec_params_ldo_get {
// struct ec_response_ldo_get {
// struct ec_response_power_info {
// struct ec_params_i2c_passthru_msg {
// struct ec_params_i2c_passthru {
// struct ec_response_i2c_passthru {
// enum ec_hang_detect_cmds {
// struct ec_params_hang_detect {
// enum ec_hang_detect_status {
// struct ec_response_hang_detect {
// enum charge_state_command {
// enum charge_state_params {
// struct ec_params_charge_state {
// struct ec_response_charge_state {
// struct ec_params_current_limit {
// struct ec_params_external_power_limit_v1 {
// struct ec_params_dedicated_charger_limit {
// struct ec_params_hibernation_delay {
// struct ec_response_hibernation_delay {
// enum host_sleep_event {
// struct ec_params_host_sleep_event {
// struct ec_params_host_sleep_event_v1 {
// struct ec_response_host_sleep_event_v1 {
// enum ec_device_event {
// enum ec_device_event_param {
// struct ec_params_device_event {
// struct ec_response_device_event {
// struct ec_params_sb_rd {
// struct ec_response_sb_rd_word {
// struct ec_params_sb_wr_word {
// struct ec_response_sb_rd_block {
// struct ec_params_sb_wr_block {
// enum ec_battery_vendor_param_mode {
// struct ec_params_battery_vendor_param {
// struct ec_response_battery_vendor_param {
// enum ec_sb_fw_update_subcmd {
// struct ec_sb_fw_update_header {
// struct ec_params_sb_fw_update {
// struct ec_response_sb_fw_update {
// struct ec_params_entering_mode {
// enum ec_i2c_passthru_protect_subcmd {
// struct ec_params_i2c_passthru_protect {
// struct ec_response_i2c_passthru_protect {
// struct ec_params_cec_write {
// struct ec_params_cec_write_v1 {
// struct ec_params_cec_read {
// struct ec_response_cec_read {
// struct ec_params_cec_set {
// struct ec_params_cec_get {
// struct ec_response_cec_get {
// struct ec_response_cec_port_count {
// enum cec_command {
// enum mkbp_cec_event {
// enum ec_codec_subcmd {
// enum ec_codec_cap {
// enum ec_codec_shm_id {
// enum ec_codec_shm_type {
// struct __ec_align1 ec_param_ec_codec_get_shm_addr {
// struct __ec_align4 ec_param_ec_codec_set_shm_addr {
// struct __ec_align4 ec_param_ec_codec {
// struct __ec_align4 ec_response_ec_codec_get_capabilities {
// struct __ec_align4 ec_response_ec_codec_get_shm_addr {
// enum ec_codec_dmic_subcmd {
// enum ec_codec_dmic_channel {
// struct __ec_align1 ec_param_ec_codec_dmic_set_gain_idx {
// struct __ec_align1 ec_param_ec_codec_dmic_get_gain_idx {
// struct __ec_align4 ec_param_ec_codec_dmic {
// struct __ec_align1 ec_response_ec_codec_dmic_get_max_gain {
// struct __ec_align1 ec_response_ec_codec_dmic_get_gain_idx {
// enum ec_codec_i2s_rx_subcmd {
// enum ec_codec_i2s_rx_sample_depth {
// enum ec_codec_i2s_rx_daifmt {
// struct __ec_align1 ec_param_ec_codec_i2s_rx_set_sample_depth {
// struct __ec_align1 ec_param_ec_codec_i2s_rx_set_gain {
// struct __ec_align1 ec_param_ec_codec_i2s_rx_set_daifmt {
// struct __ec_align4 ec_param_ec_codec_i2s_rx_set_bclk {
// struct __ec_align4 ec_param_ec_codec_i2s_rx {
// enum ec_codec_wov_subcmd {
// struct __ec_align4 ec_param_ec_codec_wov_set_lang {
// struct __ec_align4 ec_param_ec_codec_wov_set_lang_shm {
// struct __ec_align4 ec_param_ec_codec_wov {
// struct __ec_align4 ec_response_ec_codec_wov_get_lang {
// struct __ec_align4 ec_response_ec_codec_wov_read_audio {
// struct __ec_align4 ec_response_ec_codec_wov_read_audio_shm {
// enum ec_reboot_cmd {
// struct ec_params_reboot_ec {
// enum pd_charge_state {
// struct ec_params_pd_status {
// struct ec_response_pd_status {
// struct ec_response_host_event_status {
// enum usb_pd_control_role {
// enum usb_pd_control_mux {
// enum usb_pd_control_swap {
// struct ec_params_usb_pd_control {
// struct ec_response_usb_pd_control {
// struct ec_response_usb_pd_control_v1 {
// struct ec_response_usb_pd_control_v2 {
// struct ec_response_usb_pd_ports {
// struct ec_params_usb_pd_power_info {
// enum usb_chg_type {
// enum usb_power_roles {
// struct usb_chg_measures {
// struct ec_response_usb_pd_power_info {
// struct ec_response_charge_port_count {
// enum usb_pd_fw_update_cmds {
// struct ec_params_usb_pd_fw_update {
// struct ec_params_usb_pd_rw_hash_entry {
// struct ec_params_usb_pd_info_request {
// struct ec_params_usb_pd_discovery_entry {
// enum usb_pd_override_ports {
// struct ec_params_charge_port_override {
// struct ec_response_pd_log {
// struct mcdp_version {
// struct mcdp_info {
// struct ec_params_usb_pd_get_mode_request {
// struct ec_params_usb_pd_get_mode_response {
// enum pd_mode_cmd {
// struct ec_params_usb_pd_set_mode_request {
// struct ec_params_pd_write_log_entry {
// enum ec_pd_control_cmd {
// struct ec_params_pd_control {
// struct ec_params_usb_pd_mux_info {
// struct ec_response_usb_pd_mux_info {
// struct ec_params_pd_chip_info {
// struct ec_response_pd_chip_info {
// struct ec_response_pd_chip_info_v1 {
// struct ec_response_rwsig_check_status {
// enum rwsig_action {
// struct ec_params_rwsig_action {
// struct ec_params_efs_verify {
// enum cbi_data_tag {
// struct ec_params_get_cbi {
// struct ec_params_set_cbi {
// struct ec_response_uptime_info {
// enum add_entropy_action {
// struct ec_params_rollback_add_entropy {
// struct ec_params_adc_read {
// struct ec_response_adc_read {
// struct ec_response_rollback_info {
// struct ec_response_pchg_count {
// struct ec_params_pchg {
// struct ec_response_pchg {
// enum pchg_state {
// enum ec_pchg_update_cmd {
// struct ec_params_pchg_update {
// struct ec_response_pchg_update {
// struct ec_params_regulator_get_info {
// struct ec_response_regulator_get_info {
// struct ec_params_regulator_enable {
// struct ec_params_regulator_is_enabled {
// struct ec_response_regulator_is_enabled {
// struct ec_params_regulator_set_voltage {
// struct ec_params_regulator_get_voltage {
// struct ec_response_regulator_get_voltage {
// enum typec_partner_type {
// struct ec_params_typec_discovery {
// struct svid_mode_info {
// struct ec_response_typec_discovery {
// enum typec_control_command {
// enum typec_tbt_ufp_reply {
// struct typec_usb_mux_set {
// struct typec_vdm_req {
// struct ec_params_typec_control {
// enum pd_power_role {
// enum pd_data_role {
// enum pd_vconn_role {
// enum tcpc_cc_polarity {
// struct ec_params_typec_status {
// struct ec_response_typec_status {
// struct ec_params_typec_vdm_response {
// struct ec_response_typec_vdm_response {
// struct ec_params_ucsi_ppm_set {
// struct ec_params_ucsi_ppm_get {
// struct ec_params_fp_passthru {
// enum fp_capture_type {
// struct ec_params_fp_mode {
// struct ec_response_fp_mode {
// struct ec_response_fp_info_v0 {
// struct ec_response_fp_info {
// struct ec_fp_template_encryption_metadata {
// struct ec_params_fp_frame {
// struct ec_params_fp_template {
// struct ec_params_fp_context {
// struct ec_response_fp_stats {
// struct ec_params_fp_seed {
// struct ec_response_fp_encryption_status {
// struct ec_response_tp_frame_info {
// struct ec_params_tp_frame_get {
// struct ec_params_battery_static_info {
// struct ec_response_battery_static_info {
// struct ec_params_battery_dynamic_info {
// struct ec_response_battery_dynamic_info {
// struct ec_params_charger_control {
// struct ec_params_usb_pd_mux_ack {

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
