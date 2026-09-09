/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of scsi_proto.h. */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

pub const TEST_UNIT_READY: u8 = 0x00;
pub const REZERO_UNIT: u8 = 0x01;
pub const REQUEST_SENSE: u8 = 0x03;
pub const FORMAT_UNIT: u8 = 0x04;
pub const READ_BLOCK_LIMITS: u8 = 0x05;
pub const REASSIGN_BLOCKS: u8 = 0x07;
pub const INITIALIZE_ELEMENT_STATUS: u8 = 0x07;
pub const READ_6: u8 = 0x08;
pub const WRITE_6: u8 = 0x0a;
pub const SEEK_6: u8 = 0x0b;
pub const READ_REVERSE: u8 = 0x0f;
pub const WRITE_FILEMARKS: u8 = 0x10;
pub const SPACE: u8 = 0x11;
pub const INQUIRY: u8 = 0x12;
pub const RECOVER_BUFFERED_DATA: u8 = 0x14;
pub const MODE_SELECT: u8 = 0x15;
pub const RESERVE_6: u8 = 0x16;
pub const RELEASE_6: u8 = 0x17;
pub const COPY: u8 = 0x18;
pub const ERASE: u8 = 0x19;
pub const MODE_SENSE: u8 = 0x1a;
pub const START_STOP: u8 = 0x1b;
pub const RECEIVE_DIAGNOSTIC: u8 = 0x1c;
pub const SEND_DIAGNOSTIC: u8 = 0x1d;
pub const ALLOW_MEDIUM_REMOVAL: u8 = 0x1e;
pub const READ_FORMAT_CAPACITIES: u8 = 0x23;
pub const SET_WINDOW: u8 = 0x24;
pub const READ_CAPACITY: u8 = 0x25;
pub const READ_10: u8 = 0x28;
pub const WRITE_10: u8 = 0x2a;
pub const SEEK_10: u8 = 0x2b;
pub const POSITION_TO_ELEMENT: u8 = 0x2b;
pub const WRITE_VERIFY: u8 = 0x2e;
pub const VERIFY: u8 = 0x2f;
pub const SEARCH_HIGH: u8 = 0x30;
pub const SEARCH_EQUAL: u8 = 0x31;
pub const SEARCH_LOW: u8 = 0x32;
pub const SET_LIMITS: u8 = 0x33;
pub const PRE_FETCH: u8 = 0x34;
pub const READ_POSITION: u8 = 0x34;
pub const SYNCHRONIZE_CACHE: u8 = 0x35;
pub const LOCK_UNLOCK_CACHE: u8 = 0x36;
pub const READ_DEFECT_DATA: u8 = 0x37;
pub const MEDIUM_SCAN: u8 = 0x38;
pub const COMPARE: u8 = 0x39;
pub const COPY_VERIFY: u8 = 0x3a;
pub const WRITE_BUFFER: u8 = 0x3b;
pub const READ_BUFFER: u8 = 0x3c;
pub const UPDATE_BLOCK: u8 = 0x3d;
pub const READ_LONG: u8 = 0x3e;
pub const WRITE_LONG: u8 = 0x3f;
pub const CHANGE_DEFINITION: u8 = 0x40;
pub const WRITE_SAME: u8 = 0x41;
pub const UNMAP: u8 = 0x42;
pub const READ_TOC: u8 = 0x43;
pub const READ_HEADER: u8 = 0x44;
pub const GET_EVENT_STATUS_NOTIFICATION: u8 = 0x4a;
pub const LOG_SELECT: u8 = 0x4c;
pub const LOG_SENSE: u8 = 0x4d;
pub const XDWRITEREAD_10: u8 = 0x53;
pub const MODE_SELECT_10: u8 = 0x55;
pub const RESERVE_10: u8 = 0x56;
pub const RELEASE_10: u8 = 0x57;
pub const MODE_SENSE_10: u8 = 0x5a;
pub const PERSISTENT_RESERVE_IN: u8 = 0x5e;
pub const PERSISTENT_RESERVE_OUT: u8 = 0x5f;
pub const VARIABLE_LENGTH_CMD: u8 = 0x7f;
pub const REPORT_LUNS: u8 = 0xa0;
pub const SECURITY_PROTOCOL_IN: u8 = 0xa2;
pub const MAINTENANCE_IN: u8 = 0xa3;
pub const MAINTENANCE_OUT: u8 = 0xa4;
pub const MOVE_MEDIUM: u8 = 0xa5;
pub const EXCHANGE_MEDIUM: u8 = 0xa6;
pub const READ_12: u8 = 0xa8;
pub const SERVICE_ACTION_OUT_12: u8 = 0xa9;
pub const WRITE_12: u8 = 0xaa;
pub const READ_MEDIA_SERIAL_NUMBER: u8 = 0xab;
pub const SERVICE_ACTION_IN_12: u8 = 0xab;
pub const WRITE_VERIFY_12: u8 = 0xae;
pub const VERIFY_12: u8 = 0xaf;
pub const SEARCH_HIGH_12: u8 = 0xb0;
pub const SEARCH_EQUAL_12: u8 = 0xb1;
pub const SEARCH_LOW_12: u8 = 0xb2;
pub const SECURITY_PROTOCOL_OUT: u8 = 0xb5;
pub const SEND_VOLUME_TAG: u8 = 0xb6;
pub const READ_ELEMENT_STATUS: u8 = 0xb8;
pub const EXTENDED_COPY: u8 = 0x83;
pub const RECEIVE_COPY_RESULTS: u8 = 0x84;
pub const ACCESS_CONTROL_IN: u8 = 0x86;
pub const ACCESS_CONTROL_OUT: u8 = 0x87;
pub const READ_16: u8 = 0x88;
pub const COMPARE_AND_WRITE: u8 = 0x89;
pub const WRITE_16: u8 = 0x8a;
pub const READ_ATTRIBUTE: u8 = 0x8c;
pub const WRITE_ATTRIBUTE: u8 = 0x8d;
pub const WRITE_VERIFY_16: u8 = 0x8e;
pub const VERIFY_16: u8 = 0x8f;
pub const SYNCHRONIZE_CACHE_16: u8 = 0x91;
pub const WRITE_SAME_16: u8 = 0x93;
pub const ZBC_OUT: u8 = 0x94;
pub const ZBC_IN: u8 = 0x95;
pub const WRITE_ATOMIC_16: u8 = 0x9c;
pub const SERVICE_ACTION_BIDIRECTIONAL: u8 = 0x9d;
pub const SERVICE_ACTION_IN_16: u8 = 0x9e;
pub const SERVICE_ACTION_OUT_16: u8 = 0x9f;
pub const SAI_READ_CAPACITY_16: u8 = 0x10;
pub const SAI_GET_LBA_STATUS: u8 = 0x12;
pub const SAI_REPORT_REFERRALS: u8 = 0x13;
pub const SAI_GET_STREAM_STATUS: u8 = 0x16;
pub const SAI_GET_PHYSICAL_ELEMENT_STATUS: u8 = 0x17;
pub const SAI_REMOVE_ELEMENT_AND_TRUNCATE: u8 = 0x18;
pub const SAI_RESTORE_ELEMENTS_AND_REBUILD: u8 = 0x19;
pub const SAI_REMOVE_ELEMENT_AND_MODIFY_ZONES: u8 = 0x1a;
pub const MI_REPORT_IDENTIFYING_INFORMATION: u8 = 0x05;
pub const MI_REPORT_TARGET_PGS: u8 = 0x0a;
pub const MI_REPORT_ALIASES: u8 = 0x0b;
pub const MI_REPORT_SUPPORTED_OPERATION_CODES: u8 = 0x0c;
pub const MI_REPORT_SUPPORTED_TASK_MANAGEMENT_FUNCTIONS: u8 = 0x0d;
pub const MI_REPORT_PRIORITY: u8 = 0x0e;
pub const MI_REPORT_TIMESTAMP: u8 = 0x0f;
pub const MI_MANAGEMENT_PROTOCOL_IN: u8 = 0x10;
pub const MI_EXT_HDR_PARAM_FMT: u8 = 0x20;
pub const MO_SET_IDENTIFYING_INFORMATION: u8 = 0x06;
pub const MO_SET_TARGET_PGS: u8 = 0x0a;
pub const MO_CHANGE_ALIASES: u8 = 0x0b;
pub const MO_SET_PRIORITY: u8 = 0x0e;
pub const MO_SET_TIMESTAMP: u8 = 0x0f;
pub const MO_MANAGEMENT_PROTOCOL_OUT: u8 = 0x10;
pub const ZI_REPORT_ZONES: u8 = 0x00;
pub const ZO_CLOSE_ZONE: u8 = 0x01;
pub const ZO_FINISH_ZONE: u8 = 0x02;
pub const ZO_OPEN_ZONE: u8 = 0x03;
pub const ZO_RESET_WRITE_POINTER: u8 = 0x04;
pub const READ_KEYS: u8 = 0x00;
pub const READ_RESERVATION: u8 = 0x01;
pub const REPORT_CAPABILITES: u8 = 0x02;
pub const READ_FULL_STATUS: u8 = 0x03;
pub const XDREAD_32: u16 = 0x03;
pub const XDWRITE_32: u16 = 0x04;
pub const XPWRITE_32: u16 = 0x06;
pub const XDWRITEREAD_32: u16 = 0x07;
pub const READ_32: u16 = 0x09;
pub const VERIFY_32: u16 = 0x0a;
pub const WRITE_32: u16 = 0x0b;
pub const WRITE_VERIFY_32: u16 = 0x0c;
pub const WRITE_SAME_32: u16 = 0x0d;
pub const ATA_32: u16 = 0x1ff0;
pub const ATA_16: u8 = 0x85;
pub const ATA_12: u8 = 0xa1;
pub const VENDOR_SPECIFIC_CDB: u8 = 0xc0;
pub const SCSI_MAX_VARLEN_CDB_SIZE: usize = 260;

#[repr(C)]
pub struct scsi_varlen_cdb_hdr { pub opcode: u8, pub control: u8, pub misc: [u8; 5], pub additional_cdb_length: u8, pub service_action: u16 }

#[repr(C)]
pub struct scsi_lun { pub scsi_lun: [u8; 8] }

#[repr(C)]
pub struct scsi_io_group_descriptor { pub first: u8, pub reserved2: [u8; 3], pub second: u8, pub params: [u8; 2], pub reserved4: u8, pub reserved5: [u8; 8] }

#[repr(C)]
pub struct scsi_stream_status { pub first: u8, pub reserved2: u8, pub stream_identifier: u16, pub fourth: u8, pub reserved4: [u8; 3] }

#[repr(C)]
pub struct scsi_stream_status_header { pub len: u32, pub reserved: u16, pub number_of_open_streams: u16 }

pub const SAM_STAT_GOOD: u8 = 0x00;
pub const SAM_STAT_CHECK_CONDITION: u8 = 0x02;
pub const SAM_STAT_CONDITION_MET: u8 = 0x04;
pub const SAM_STAT_BUSY: u8 = 0x08;
pub const SAM_STAT_INTERMEDIATE: u8 = 0x10;
pub const SAM_STAT_INTERMEDIATE_CONDITION_MET: u8 = 0x14;
pub const SAM_STAT_RESERVATION_CONFLICT: u8 = 0x18;
pub const SAM_STAT_COMMAND_TERMINATED: u8 = 0x22;
pub const SAM_STAT_TASK_SET_FULL: u8 = 0x28;
pub const SAM_STAT_ACA_ACTIVE: u8 = 0x30;
pub const SAM_STAT_TASK_ABORTED: u8 = 0x40;
pub const STATUS_MASK: u8 = 0xfe;

pub const NO_SENSE: u8 = 0x00;
pub const RECOVERED_ERROR: u8 = 0x01;
pub const NOT_READY: u8 = 0x02;
pub const MEDIUM_ERROR: u8 = 0x03;
pub const HARDWARE_ERROR: u8 = 0x04;
pub const ILLEGAL_REQUEST: u8 = 0x05;
pub const UNIT_ATTENTION: u8 = 0x06;
pub const DATA_PROTECT: u8 = 0x07;
pub const BLANK_CHECK: u8 = 0x08;
pub const VENDOR_SPECIFIC: u8 = 0x09;
pub const COPY_ABORTED: u8 = 0x0a;
pub const ABORTED_COMMAND: u8 = 0x0b;
pub const VOLUME_OVERFLOW: u8 = 0x0d;
pub const MISCOMPARE: u8 = 0x0e;
pub const COMPLETED: u8 = 0x0f;
pub const NO_ADDITIONAL_SENSE: u8 = 0x00;
pub const OVERLAP_ATOMIC_COMMAND_ASC: u8 = 0x00;
pub const LOGICAL_UNIT_NOT_READY: u8 = 0x04;
pub const LOGICAL_UNIT_COMMUNICATION_FAILURE: u8 = 0x08;
pub const WRITE_ERROR_ASC: u8 = 0x0c;
pub const UNRECOVERED_READ_ERR: u8 = 0x11;
pub const PARAMETER_LIST_LENGTH_ERR: u8 = 0x1a;
pub const MISCOMPARE_VERIFY_ASC: u8 = 0x1d;
pub const INVALID_OPCODE: u8 = 0x20;
pub const LBA_OUT_OF_RANGE: u8 = 0x21;
pub const INVALID_FIELD_IN_CDB: u8 = 0x24;
pub const INVALID_FIELD_IN_PARAM_LIST: u8 = 0x26;
pub const WRITE_PROTECTED: u8 = 0x27;
pub const UA_READY_ASC: u8 = 0x28;
pub const UA_RESET_ASC: u8 = 0x29;
pub const UA_CHANGED_ASC: u8 = 0x2a;
pub const TOO_MANY_IN_PARTITION_ASC: u8 = 0x3b;
pub const TARGET_CHANGED_ASC: u8 = 0x3f;
pub const SAVING_PARAMS_UNSUP: u8 = 0x39;
pub const TRANSPORT_PROBLEM: u8 = 0x4b;
pub const INSUFF_RES_ASC: u8 = 0x55;
pub const LOW_POWER_COND_ON: u8 = 0x5e;
pub const THRESHOLD_EXCEEDED: u8 = 0x5d;
pub const POWER_ON_RESET_ASCQ: u8 = 0x00;
pub const MODE_CHANGED_ASCQ: u8 = 0x01;
pub const FILEMARK_DETECTED_ASCQ: u8 = 0x01;
pub const POWER_ON_OCCURRED_ASCQ: u8 = 0x01;
pub const MICROCODE_CHANGED_ASCQ: u8 = 0x01;
pub const BUS_RESET_ASCQ: u8 = 0x02;
pub const EOP_EOM_DETECTED_ASCQ: u8 = 0x02;
pub const INSUFF_RES_ASCQ: u8 = 0x03;
pub const BEGINNING_OF_P_M_DETECTED_ASCQ: u8 = 0x04;
pub const UNALIGNED_WRITE_ASCQ: u8 = 0x04;
pub const EOD_DETECTED_ASCQ: u8 = 0x05;
pub const WRITE_BOUNDARY_ASCQ: u8 = 0x05;
pub const READ_INVDATA_ASCQ: u8 = 0x06;
pub const READ_BOUNDARY_ASCQ: u8 = 0x07;
pub const CAPACITY_CHANGED_ASCQ: u8 = 0x09;
pub const ATTEMPT_ACCESS_GAP: u8 = 0x09;
pub const LUNS_CHANGED_ASCQ: u8 = 0x0e;
pub const INSUFF_ZONE_ASCQ: u8 = 0x0e;
pub const MICROCODE_CHANGED_WO_RESET_ASCQ: u8 = 0x16;
pub const OVERLAP_ATOMIC_COMMAND_ASCQ: u8 = 0x23;

pub const TYPE_DISK: u8 = 0x00; pub const TYPE_TAPE: u8 = 0x01; pub const TYPE_PRINTER: u8 = 0x02;
pub const TYPE_PROCESSOR: u8 = 0x03; pub const TYPE_WORM: u8 = 0x04; pub const TYPE_ROM: u8 = 0x05;
pub const TYPE_SCANNER: u8 = 0x06; pub const TYPE_MOD: u8 = 0x07; pub const TYPE_MEDIUM_CHANGER: u8 = 0x08;
pub const TYPE_COMM: u8 = 0x09; pub const TYPE_RAID: u8 = 0x0c; pub const TYPE_ENCLOSURE: u8 = 0x0d;
pub const TYPE_RBC: u8 = 0x0e; pub const TYPE_OSD: u8 = 0x11; pub const TYPE_ZBC: u8 = 0x14;
pub const TYPE_WLUN: u8 = 0x1e; pub const TYPE_NO_LUN: u8 = 0x7f;

#[repr(u8)] pub enum scsi_protocol { SCSI_PROTOCOL_FCP=0, SCSI_PROTOCOL_SPI=1, SCSI_PROTOCOL_SSA=2, SCSI_PROTOCOL_SBP=3, SCSI_PROTOCOL_SRP=4, SCSI_PROTOCOL_ISCSI=5, SCSI_PROTOCOL_SAS=6, SCSI_PROTOCOL_ADT=7, SCSI_PROTOCOL_ATA=8, SCSI_PROTOCOL_UNSPEC=0xf }
pub const SCSI_ACCESS_STATE_OPTIMAL: u8=0; pub const SCSI_ACCESS_STATE_ACTIVE:u8=1; pub const SCSI_ACCESS_STATE_STANDBY:u8=2; pub const SCSI_ACCESS_STATE_UNAVAILABLE:u8=3; pub const SCSI_ACCESS_STATE_LBA:u8=4; pub const SCSI_ACCESS_STATE_OFFLINE:u8=0x0e; pub const SCSI_ACCESS_STATE_TRANSITIONING:u8=0x0f;
pub const SCSI_ACCESS_STATE_MASK:u8=0x0f; pub const SCSI_ACCESS_STATE_PREFERRED:u8=0x80;
#[repr(u8)] pub enum zbc_zone_reporting_options { ZBC_ZONE_REPORTING_OPTION_ALL=0, ZBC_ZONE_REPORTING_OPTION_EMPTY=1, ZBC_ZONE_REPORTING_OPTION_IMPLICIT_OPEN=2, ZBC_ZONE_REPORTING_OPTION_EXPLICIT_OPEN=3, ZBC_ZONE_REPORTING_OPTION_CLOSED=4, ZBC_ZONE_REPORTING_OPTION_FULL=5, ZBC_ZONE_REPORTING_OPTION_READONLY=6, ZBC_ZONE_REPORTING_OPTION_OFFLINE=7, ZBC_ZONE_REPORTING_OPTION_NEED_RESET_WP=0x10, ZBC_ZONE_REPORTING_OPTION_NON_SEQWRITE=0x11, ZBC_ZONE_REPORTING_OPTION_NON_WP=0x3f }
pub const ZBC_REPORT_ZONE_PARTIAL:u8=0x80;
#[repr(u8)] pub enum zbc_zone_type { ZBC_ZONE_TYPE_CONV=1, ZBC_ZONE_TYPE_SEQWRITE_REQ=2, ZBC_ZONE_TYPE_SEQWRITE_PREF=3, ZBC_ZONE_TYPE_SEQ_OR_BEFORE_REQ=4, ZBC_ZONE_TYPE_GAP=5 }
#[repr(u8)] pub enum zbc_zone_cond { ZBC_ZONE_COND_NO_WP=0, ZBC_ZONE_COND_EMPTY=1, ZBC_ZONE_COND_IMP_OPEN=2, ZBC_ZONE_COND_EXP_OPEN=3, ZBC_ZONE_COND_CLOSED=4, ZBC_ZONE_COND_READONLY=0xd, ZBC_ZONE_COND_FULL=0xe, ZBC_ZONE_COND_OFFLINE=0xf }
#[repr(u8)] pub enum zbc_zone_alignment_method { ZBC_CONSTANT_ZONE_LENGTH=1, ZBC_CONSTANT_ZONE_START_OFFSET=8 }
#[repr(u8)] pub enum scsi_phys_element_type { SCSI_PHYS_ELEM_TYPE_ALL_ACCESS_STORAGE=1, SCSI_PHYS_ELEM_TYPE_FRAC_ACCESS_STORAGE=2 }
#[repr(u8)] pub enum scsi_phys_element_health { SCSI_PHYS_ELEM_HEALTH_NOT_REPORTED=0, SCSI_PHYS_ELEM_HEALTH_WITHIN_SPEC_LIMITS=1, SCSI_PHYS_ELEM_HEALTH_AT_SPEC_LIMITS=0x64, SCSI_PHYS_ELEM_HEALTH_OUTSIDE_SPEC_LIMITS=0x65, SCSI_PHYS_ELEM_HEALTH_DEPOP_REVOKE_ERR=0xfb, SCSI_PHYS_ELEM_HEALTH_DEPOP_REVOKE_IN_PROGRESS=0xfc, SCSI_PHYS_ELEM_HEALTH_DEPOP_ERR=0xfd, SCSI_PHYS_ELEM_HEALTH_DEPOP_IN_PROGRESS=0xfe, SCSI_PHYS_ELEM_HEALTH_DEPOP_OK=0xff }
#[repr(u16)] pub enum scsi_version_descriptor { SCSI_VERSION_DESCRIPTOR_FCP4=0x0a40, SCSI_VERSION_DESCRIPTOR_ISCSI=0x0960, SCSI_VERSION_DESCRIPTOR_SAM5=0x00a0, SCSI_VERSION_DESCRIPTOR_SAS3=0x0c60, SCSI_VERSION_DESCRIPTOR_SBC3=0x04c0, SCSI_VERSION_DESCRIPTOR_SBP3=0x0980, SCSI_VERSION_DESCRIPTOR_SPC4=0x0460, SCSI_VERSION_DESCRIPTOR_SRP=0x0940 }
#[repr(u8)] pub enum scsi_support_opcode { SCSI_SUPPORT_NO_INFO=0, SCSI_SUPPORT_NOT_SUPPORTED=1, SCSI_SUPPORT_FULL=3, SCSI_SUPPORT_VENDOR=5 }
pub const SCSI_CONTROL_MASK:u8=0; pub const SCSI_GROUP_NUMBER_MASK:u8=0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
