/* SPDX-License-Identifier: GPL-2.0 */
// Translated from trace/events/target.h.  The included Linux tracepoint,
// SCSI, and target-core definitions are supplied by the surrounding crate.

pub const SCSI_OPCODE_NAMES: &[(u32, &str)] = &[
    (TEST_UNIT_READY, "TEST_UNIT_READY"), (REZERO_UNIT, "REZERO_UNIT"),
    (REQUEST_SENSE, "REQUEST_SENSE"), (FORMAT_UNIT, "FORMAT_UNIT"),
    (READ_BLOCK_LIMITS, "READ_BLOCK_LIMITS"), (REASSIGN_BLOCKS, "REASSIGN_BLOCKS"),
    (INITIALIZE_ELEMENT_STATUS, "INITIALIZE_ELEMENT_STATUS"), (READ_6, "READ_6"),
    (WRITE_6, "WRITE_6"), (SEEK_6, "SEEK_6"), (READ_REVERSE, "READ_REVERSE"),
    (WRITE_FILEMARKS, "WRITE_FILEMARKS"), (SPACE, "SPACE"), (INQUIRY, "INQUIRY"),
    (RECOVER_BUFFERED_DATA, "RECOVER_BUFFERED_DATA"), (MODE_SELECT, "MODE_SELECT"),
    (RESERVE_6, "RESERVE_6"), (RELEASE_6, "RELEASE_6"), (COPY, "COPY"),
    (ERASE, "ERASE"), (MODE_SENSE, "MODE_SENSE"), (START_STOP, "START_STOP"),
    (RECEIVE_DIAGNOSTIC, "RECEIVE_DIAGNOSTIC"), (SEND_DIAGNOSTIC, "SEND_DIAGNOSTIC"),
    (ALLOW_MEDIUM_REMOVAL, "ALLOW_MEDIUM_REMOVAL"), (SET_WINDOW, "SET_WINDOW"),
    (READ_CAPACITY, "READ_CAPACITY"), (READ_10, "READ_10"), (WRITE_10, "WRITE_10"),
    (SEEK_10, "SEEK_10"), (POSITION_TO_ELEMENT, "POSITION_TO_ELEMENT"),
    (WRITE_VERIFY, "WRITE_VERIFY"), (VERIFY, "VERIFY"), (SEARCH_HIGH, "SEARCH_HIGH"),
    (SEARCH_EQUAL, "SEARCH_EQUAL"), (SEARCH_LOW, "SEARCH_LOW"), (SET_LIMITS, "SET_LIMITS"),
    (PRE_FETCH, "PRE_FETCH"), (READ_POSITION, "READ_POSITION"),
    (SYNCHRONIZE_CACHE, "SYNCHRONIZE_CACHE"), (LOCK_UNLOCK_CACHE, "LOCK_UNLOCK_CACHE"),
    (READ_DEFECT_DATA, "READ_DEFECT_DATA"), (MEDIUM_SCAN, "MEDIUM_SCAN"),
    (COMPARE, "COMPARE"), (COPY_VERIFY, "COPY_VERIFY"), (WRITE_BUFFER, "WRITE_BUFFER"),
    (READ_BUFFER, "READ_BUFFER"), (UPDATE_BLOCK, "UPDATE_BLOCK"), (READ_LONG, "READ_LONG"),
    (WRITE_LONG, "WRITE_LONG"), (CHANGE_DEFINITION, "CHANGE_DEFINITION"),
    (WRITE_SAME, "WRITE_SAME"), (UNMAP, "UNMAP"), (READ_TOC, "READ_TOC"),
    (LOG_SELECT, "LOG_SELECT"), (LOG_SENSE, "LOG_SENSE"), (XDWRITEREAD_10, "XDWRITEREAD_10"),
    (MODE_SELECT_10, "MODE_SELECT_10"), (RESERVE_10, "RESERVE_10"),
    (RELEASE_10, "RELEASE_10"), (MODE_SENSE_10, "MODE_SENSE_10"),
    (PERSISTENT_RESERVE_IN, "PERSISTENT_RESERVE_IN"), (PERSISTENT_RESERVE_OUT, "PERSISTENT_RESERVE_OUT"),
    (VARIABLE_LENGTH_CMD, "VARIABLE_LENGTH_CMD"), (REPORT_LUNS, "REPORT_LUNS"),
    (MAINTENANCE_IN, "MAINTENANCE_IN"), (MAINTENANCE_OUT, "MAINTENANCE_OUT"),
    (MOVE_MEDIUM, "MOVE_MEDIUM"), (EXCHANGE_MEDIUM, "EXCHANGE_MEDIUM"),
    (READ_12, "READ_12"), (WRITE_12, "WRITE_12"), (WRITE_VERIFY_12, "WRITE_VERIFY_12"),
    (SEARCH_HIGH_12, "SEARCH_HIGH_12"), (SEARCH_EQUAL_12, "SEARCH_EQUAL_12"),
    (SEARCH_LOW_12, "SEARCH_LOW_12"), (READ_ELEMENT_STATUS, "READ_ELEMENT_STATUS"),
    (SEND_VOLUME_TAG, "SEND_VOLUME_TAG"), (WRITE_LONG_2, "WRITE_LONG_2"),
    (READ_16, "READ_16"), (WRITE_16, "WRITE_16"), (VERIFY_16, "VERIFY_16"),
    (WRITE_SAME_16, "WRITE_SAME_16"), (SERVICE_ACTION_IN_16, "SERVICE_ACTION_IN_16"),
    (SAI_READ_CAPACITY_16, "SAI_READ_CAPACITY_16"), (SAI_GET_LBA_STATUS, "SAI_GET_LBA_STATUS"),
    (MI_REPORT_TARGET_PGS, "MI_REPORT_TARGET_PGS"), (MO_SET_TARGET_PGS, "MO_SET_TARGET_PGS"),
    (READ_32, "READ_32"), (WRITE_32, "WRITE_32"), (WRITE_SAME_32, "WRITE_SAME_32"),
    (ATA_16, "ATA_16"), (ATA_12, "ATA_12"),
];

pub const TASK_ATTRIBUTE_NAMES: &[(u32, &str)] = &[
    (TCM_SIMPLE_TAG, "SIMPLE"), (TCM_HEAD_TAG, "HEAD"),
    (TCM_ORDERED_TAG, "ORDERED"), (TCM_ACA_TAG, "ACA"),
];

pub const SCSI_STATUS_NAMES: &[(u8, &str)] = &[
    (SAM_STAT_GOOD, "GOOD"), (SAM_STAT_CHECK_CONDITION, "CHECK CONDITION"),
    (SAM_STAT_CONDITION_MET, "CONDITION MET"), (SAM_STAT_BUSY, "BUSY"),
    (SAM_STAT_INTERMEDIATE, "INTERMEDIATE"),
    (SAM_STAT_INTERMEDIATE_CONDITION_MET, "INTERMEDIATE CONDITION MET"),
    (SAM_STAT_RESERVATION_CONFLICT, "RESERVATION CONFLICT"),
    (SAM_STAT_COMMAND_TERMINATED, "COMMAND TERMINATED"),
    (SAM_STAT_TASK_SET_FULL, "TASK SET FULL"), (SAM_STAT_ACA_ACTIVE, "ACA ACTIVE"),
    (SAM_STAT_TASK_ABORTED, "TASK ABORTED"),
];

// The following invocations preserve the Linux TRACE_EVENT declarations and
// their assignment/printing behavior; `trace_event!` is provided externally.
trace_event!(target_sequencer_start {
    proto: (*mut se_cmd),
    fields: { unpacked_lun: u32, tag: u64, opcode: u32, data_length: u32,
              task_attribute: u32, control: u8, cdb: [u8; TCM_MAX_COMMAND_SIZE],
              initiator: trace_string },
    assign: { unpacked_lun = cmd.orig_fe_lun; tag = cmd.tag; opcode = cmd.t_task_cdb[0];
              data_length = cmd.data_length; task_attribute = cmd.sam_task_attr;
              control = scsi_command_control(cmd.t_task_cdb); memcpy(cdb, cmd.t_task_cdb);
              assign_string(initiator); }
});

trace_event!(target_cmd_complete {
    proto: (*mut se_cmd),
    fields: { unpacked_lun: u32, tag: u64, opcode: u32, data_length: u32,
              task_attribute: u32, control: u8, scsi_status: u8, sense_length: u8,
              cdb: [u8; TCM_MAX_COMMAND_SIZE], sense_data: [u8; 18], initiator: trace_string },
    assign: { unpacked_lun = cmd.orig_fe_lun; tag = cmd.tag; opcode = cmd.t_task_cdb[0];
              data_length = cmd.data_length; task_attribute = cmd.sam_task_attr;
              control = scsi_command_control(cmd.t_task_cdb); scsi_status = cmd.scsi_status;
              sense_length = if cmd.scsi_status == SAM_STAT_CHECK_CONDITION {
                  core::cmp::min(18, cmd.sense_buffer[SPC_ADD_SENSE_LEN_OFFSET] as u32 + 8) as u8
              } else { 0 }; memcpy(cdb, cmd.t_task_cdb); memcpy(sense_data, cmd.sense_buffer);
              assign_string(initiator); }
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
