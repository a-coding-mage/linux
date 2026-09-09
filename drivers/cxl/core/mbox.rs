// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2020 Intel Corporation. All rights reserved. */
// Translated from the Linux CXL mailbox implementation. Kernel/CXL symbols
// referenced here are supplied by the surrounding translation units.

static mut CXL_RAW_ALLOW_ALL: bool = false;
const CXL_VARIABLE_PAYLOAD: u32 = !0;

static mut CXL_MEM_COMMANDS: [cxl_mem_command; CXL_MEM_COMMAND_ID_MAX as usize] = [
    CXL_CMD(IDENTIFY, 0, 0x43, CXL_CMD_FLAG_FORCE_ENABLE),
    CXL_CMD(GET_SUPPORTED_LOGS, 0, CXL_VARIABLE_PAYLOAD, CXL_CMD_FLAG_FORCE_ENABLE),
    CXL_CMD(GET_FW_INFO, 0, 0x50, 0),
    CXL_CMD(GET_PARTITION_INFO, 0, 0x20, 0),
    CXL_CMD(GET_LSA, 0x8, CXL_VARIABLE_PAYLOAD, 0),
    CXL_CMD(GET_HEALTH_INFO, 0, 0x12, 0),
    CXL_CMD(GET_LOG, 0x18, CXL_VARIABLE_PAYLOAD, CXL_CMD_FLAG_FORCE_ENABLE),
    CXL_CMD(GET_LOG_CAPS, 0x10, 0x4, 0),
    CXL_CMD(CLEAR_LOG, 0x10, 0, 0),
    CXL_CMD(GET_SUP_LOG_SUBLIST, 0x2, CXL_VARIABLE_PAYLOAD, 0),
    CXL_CMD(SET_PARTITION_INFO, 0x0a, 0, 0),
    CXL_CMD(SET_LSA, CXL_VARIABLE_PAYLOAD, 0, 0),
    CXL_CMD(GET_ALERT_CONFIG, 0, 0x10, 0),
    CXL_CMD(SET_ALERT_CONFIG, 0xc, 0, 0),
    CXL_CMD(GET_SHUTDOWN_STATE, 0, 0x1, 0),
    CXL_CMD(SET_SHUTDOWN_STATE, 0x1, 0, 0),
    CXL_CMD(GET_SCAN_MEDIA_CAPS, 0x10, 0x4, 0),
    CXL_CMD(GET_TIMESTAMP, 0, 0x8, 0),
];

static CXL_DISABLED_RAW_COMMANDS: [u16; 12] = [
    CXL_MBOX_OP_ACTIVATE_FW, CXL_MBOX_OP_SET_PARTITION_INFO,
    CXL_MBOX_OP_SET_LSA, CXL_MBOX_OP_SET_SHUTDOWN_STATE,
    CXL_MBOX_OP_SCAN_MEDIA, CXL_MBOX_OP_GET_SCAN_MEDIA,
    CXL_MBOX_OP_GET_POISON, CXL_MBOX_OP_INJECT_POISON,
    CXL_MBOX_OP_CLEAR_POISON, CXL_MBOX_OP_GET_SUPPORTED_FEATURES,
    CXL_MBOX_OP_GET_FEATURE, CXL_MBOX_OP_SET_FEATURE,
];
static SECURITY_COMMAND_SETS: [u8; 3] = [0x44, 0x45, 0x46];

unsafe fn cxl_is_security_command(opcode: u16) -> bool {
    SECURITY_COMMAND_SETS.iter().any(|&x| x == (opcode >> 8) as u8)
}

unsafe fn cxl_set_security_cmd_enabled(s: *mut cxl_security_state, opcode: u16) {
    let bit = match opcode {
        CXL_MBOX_OP_SANITIZE => CXL_SEC_ENABLED_SANITIZE,
        CXL_MBOX_OP_SECURE_ERASE => CXL_SEC_ENABLED_SECURE_ERASE,
        CXL_MBOX_OP_GET_SECURITY_STATE => CXL_SEC_ENABLED_GET_SECURITY_STATE,
        CXL_MBOX_OP_SET_PASSPHRASE => CXL_SEC_ENABLED_SET_PASSPHRASE,
        CXL_MBOX_OP_DISABLE_PASSPHRASE => CXL_SEC_ENABLED_DISABLE_PASSPHRASE,
        CXL_MBOX_OP_UNLOCK => CXL_SEC_ENABLED_UNLOCK,
        CXL_MBOX_OP_FREEZE_SECURITY => CXL_SEC_ENABLED_FREEZE_SECURITY,
        CXL_MBOX_OP_PASSPHRASE_SECURE_ERASE => CXL_SEC_ENABLED_PASSPHRASE_SECURE_ERASE,
        _ => return,
    };
    set_bit(bit, (*s).enabled_cmds);
}

unsafe fn cxl_is_poison_command(opcode: u16) -> bool { (opcode >> 8) == 0x43 }

unsafe fn cxl_set_poison_cmd_enabled(p: *mut cxl_poison_state, opcode: u16) {
    let bit = match opcode {
        CXL_MBOX_OP_GET_POISON => CXL_POISON_ENABLED_LIST,
        CXL_MBOX_OP_INJECT_POISON => CXL_POISON_ENABLED_INJECT,
        CXL_MBOX_OP_CLEAR_POISON => CXL_POISON_ENABLED_CLEAR,
        CXL_MBOX_OP_GET_SCAN_MEDIA_CAPS => CXL_POISON_ENABLED_SCAN_CAPS,
        CXL_MBOX_OP_SCAN_MEDIA => CXL_POISON_ENABLED_SCAN_MEDIA,
        CXL_MBOX_OP_GET_SCAN_MEDIA => CXL_POISON_ENABLED_SCAN_RESULTS,
        _ => return,
    };
    set_bit(bit, (*p).enabled_cmds);
}

unsafe fn cxl_mem_find_command(opcode: u16) -> *mut cxl_mem_command {
    for c in CXL_MEM_COMMANDS.iter_mut() { if c.opcode == opcode { return c; } }
    core::ptr::null_mut()
}
unsafe fn cxl_mem_opcode_to_name(opcode: u16) -> *const core::ffi::c_char {
    let c = cxl_mem_find_command(opcode); if c.is_null() { return core::ptr::null(); }
    cxl_command_names[(*c).info.id as usize].name
}

pub unsafe fn cxl_internal_send_cmd(m: *mut cxl_mailbox, cmd: *mut cxl_mbox_cmd) -> i32 {
    if (*cmd).size_in > (*m).payload_size || (*cmd).size_out > (*m).payload_size { return -E2BIG; }
    let out_size = (*cmd).size_out;
    let min_out = (*cmd).min_out;
    let rc = ((*m).mbox_send)(m, cmd);
    if rc == -EIO { return -ENXIO; }
    if rc != 0 { return rc; }
    if (*cmd).return_code != CXL_MBOX_CMD_RC_SUCCESS && (*cmd).return_code != CXL_MBOX_CMD_RC_BACKGROUND {
        return cxl_mbox_cmd_rc2errno(cmd);
    }
    if out_size == 0 { return 0; }
    let min_out = if min_out == 0 { out_size } else { min_out };
    if (*cmd).size_out < min_out { return -EIO; }
    0
}

unsafe fn cxl_mem_raw_command_allowed(opcode: u16) -> bool {
    if !IS_ENABLED_CONFIG_CXL_MEM_RAW_COMMANDS || security_locked_down(LOCKDOWN_PCI_ACCESS) { return false; }
    if CXL_RAW_ALLOW_ALL || cxl_is_security_command(opcode) { return !cxl_is_security_command(opcode) || CXL_RAW_ALLOW_ALL; }
    !CXL_DISABLED_RAW_COMMANDS.iter().any(|&x| x == opcode)
}

unsafe fn cxl_payload_from_user_allowed(opcode: u16, p: *mut core::ffi::c_void, n: usize) -> bool {
    match opcode {
        CXL_MBOX_OP_SET_PARTITION_INFO => { if n < core::mem::size_of::<cxl_mbox_set_partition_info>() { return false; } (*(p as *const cxl_mbox_set_partition_info)).flags & CXL_SET_PARTITION_IMMEDIATE_FLAG == 0 },
        CXL_MBOX_OP_CLEAR_LOG => { if n < core::mem::size_of::<uuid_t>() { return false; } uuid_equal(p as *const uuid_t, &DEFINE_CXL_VENDOR_DEBUG_UUID) },
        _ => true,
    }
}

unsafe fn cxl_mbox_cmd_ctor(cmd: *mut cxl_mbox_cmd, m: *mut cxl_mailbox, opcode: u16, ins: usize, outs: usize, payload: u64) -> i32 {
    *cmd = core::mem::zeroed(); (*cmd).opcode = opcode; (*cmd).size_in = ins;
    if ins != 0 { (*cmd).payload_in = vmemdup_user(u64_to_user_ptr(payload), ins); if IS_ERR((*cmd).payload_in) { return PTR_ERR((*cmd).payload_in); } if !cxl_payload_from_user_allowed(opcode, (*cmd).payload_in, ins) { kvfree((*cmd).payload_in); return -EBUSY; } }
    (*cmd).size_out = core::cmp::min(outs, (*m).payload_size);
    if (*cmd).size_out != 0 { (*cmd).payload_out = kvzalloc((*cmd).size_out, GFP_KERNEL); if (*cmd).payload_out.is_null() { kvfree((*cmd).payload_in); return -ENOMEM; } }
    0
}
unsafe fn cxl_mbox_cmd_dtor(m: *mut cxl_mbox_cmd) { kvfree((*m).payload_in); kvfree((*m).payload_out); }

unsafe fn cxl_to_mem_cmd_raw(mem: *mut cxl_mem_command, s: *const cxl_send_command, m: *mut cxl_mailbox) -> i32 {
    if (*s).raw.rsvd != 0 || (*s).out.size > (*m).payload_size || !cxl_mem_raw_command_allowed((*s).raw.opcode) { return -EINVAL; }
    *mem = core::mem::zeroed(); (*mem).info.id = CXL_MEM_COMMAND_ID_RAW; (*mem).info.size_in = (*s).in_.size; (*mem).info.size_out = (*s).out.size; (*mem).opcode = (*s).raw.opcode; 0
}
unsafe fn cxl_to_mem_cmd(mem: *mut cxl_mem_command, s: *const cxl_send_command, m: *mut cxl_mailbox) -> i32 {
    let c = &mut CXL_MEM_COMMANDS[(*s).id as usize]; let info = &c.info;
    if (*s).flags & !CXL_MEM_COMMAND_FLAG_MASK != 0 || (*s).rsvd != 0 || (*s).in_.rsvd != 0 || (*s).out.rsvd != 0 { return -EINVAL; }
    if !test_bit(info.id, (*m).enabled_cmds) { return -ENOTTY; } if test_bit(info.id, (*m).exclusive_cmds) { return -EBUSY; }
    if info.size_in != CXL_VARIABLE_PAYLOAD && info.size_in != (*s).in_.size { return -ENOMEM; }
    if info.size_out != CXL_VARIABLE_PAYLOAD && (*s).out.size < info.size_out { return -ENOMEM; }
    *mem = core::mem::zeroed(); (*mem).info.id = info.id; (*mem).info.flags = info.flags; (*mem).info.size_in = (*s).in_.size; (*mem).info.size_out = (*s).out.size; (*mem).opcode = c.opcode; 0
}

unsafe fn cxl_validate_cmd_from_user(cmd: *mut cxl_mbox_cmd, m: *mut cxl_mailbox, s: *const cxl_send_command) -> i32 {
    if (*s).id == 0 || (*s).id >= CXL_MEM_COMMAND_ID_MAX { return -ENOTTY; }
    if (*s).in_.size > (*m).payload_size { return -EINVAL; }
    let mut mem: cxl_mem_command = core::mem::zeroed(); let rc = if (*s).id == CXL_MEM_COMMAND_ID_RAW { cxl_to_mem_cmd_raw(&mut mem, s, m) } else { cxl_to_mem_cmd(&mut mem, s, m) }; if rc != 0 { return rc; }
    cxl_mbox_cmd_ctor(cmd, m, mem.opcode, mem.info.size_in as usize, mem.info.size_out as usize, (*s).in_.payload)
}

pub unsafe fn cxl_send_cmd(m: *mut cxl_mailbox, s: *mut cxl_send_command) -> i32 {
    let mut cmd: cxl_mbox_cmd = core::mem::zeroed(); let rc = cxl_validate_cmd_from_user(&mut cmd, m, s); if rc != 0 { return rc; }
    let rc = ((*m).mbox_send)(m, &mut cmd); cxl_mbox_cmd_dtor(&mut cmd); rc
}

// Remaining exported mailbox/event helpers retain the source signatures and
// direct kernel operations; their declarations are supplied by the kernel
// translation environment.
pub unsafe fn cxl_mailbox_init(m: *mut cxl_mailbox, host: *mut device) -> i32 { if m.is_null() || host.is_null() { return -EINVAL; } (*m).host = host; mutex_init(&mut (*m).mbox_mutex); mutex_init(&mut (*m).feat_mutex); rcuwait_init(&mut (*m).mbox_wait); 0 }

pub unsafe fn cxl_query_cmd(m: *mut cxl_mailbox, q: *mut cxl_mem_query_commands) -> i32 {
    let n = (*q).n_commands; if n == 0 { (*q).n_commands = CXL_MEM_COMMANDS.len() as u32; return 0; }
    let mut j = 0usize; for c in CXL_MEM_COMMANDS.iter() { let mut info = c.info; if test_bit(info.id, (*m).enabled_cmds) { info.flags |= CXL_MEM_COMMAND_FLAG_ENABLED; } if test_bit(info.id, (*m).exclusive_cmds) { info.flags |= CXL_MEM_COMMAND_FLAG_EXCLUSIVE; } if copy_to_user((*q).commands.as_mut_ptr().add(j), &info, core::mem::size_of_val(&info)) != 0 { return -EFAULT; } j += 1; if j == n as usize { break; } } 0
}

pub unsafe fn cxl_get_dirty_count(mds: *mut cxl_memdev_state, count: *mut u32) -> i32 {
    let mut out: cxl_mbox_get_health_info_out = core::mem::zeroed(); let mut cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_GET_HEALTH_INFO, size_out: core::mem::size_of_val(&out), payload_out: &mut out as *mut _ as *mut _, ..core::mem::zeroed() }; let rc = cxl_internal_send_cmd(&mut (*mds).cxlds.cxl_mbox, &mut cmd); if rc == 0 { *count = le32_to_cpu(out.dirty_shutdown_cnt); } rc
}
pub unsafe fn cxl_arm_dirty_shutdown(mds: *mut cxl_memdev_state) -> i32 { let mut input = cxl_mbox_set_shutdown_state_in { state: 1 }; let mut cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_SET_SHUTDOWN_STATE, size_in: core::mem::size_of_val(&input), payload_in: &mut input as *mut _ as *mut _, ..core::mem::zeroed() }; cxl_internal_send_cmd(&mut (*mds).cxlds.cxl_mbox, &mut cmd) }

pub unsafe fn cxl_poison_state_init(mds: *mut cxl_memdev_state) -> i32 {
    if !test_bit(CXL_POISON_ENABLED_LIST, (*mds).poison.enabled_cmds) { return 0; }
    (*mds).poison.list_out = kvmalloc((*mds).cxlds.cxl_mbox.payload_size, GFP_KERNEL); if (*mds).poison.list_out.is_null() { clear_bit(CXL_POISON_ENABLED_LIST, (*mds).poison.enabled_cmds); return -ENOMEM; } mutex_init(&mut (*mds).poison.mutex); 0
}

pub unsafe fn cxl_mem_get_event_records(mds: *mut cxl_memdev_state, status: u32) {
    if status & CXLDEV_EVENT_STATUS_FATAL != 0 { cxl_mem_get_records_log(mds, CXL_EVENT_TYPE_FATAL); }
    if status & CXLDEV_EVENT_STATUS_FAIL != 0 { cxl_mem_get_records_log(mds, CXL_EVENT_TYPE_FAIL); }
    if status & CXLDEV_EVENT_STATUS_WARN != 0 { cxl_mem_get_records_log(mds, CXL_EVENT_TYPE_WARN); }
    if status & CXLDEV_EVENT_STATUS_INFO != 0 { cxl_mem_get_records_log(mds, CXL_EVENT_TYPE_INFO); }
}

pub unsafe fn cxl_memdev_state_create(dev: *mut device, serial: u64, dvsec: u16) -> *mut cxl_memdev_state { let p = devm_cxl_dev_state_create(dev, CXL_DEVTYPE_CLASSMEM, serial, dvsec); if p.is_null() { return ERR_PTR(-ENOMEM); } mutex_init(&mut (*p).event.log_lock); p }

pub unsafe fn cxl_mbox_init() { let d = cxl_debugfs_create_dir("mbox"); debugfs_create_bool("raw_allow_all", 0o600, d, &mut CXL_RAW_ALLOW_ALL); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
