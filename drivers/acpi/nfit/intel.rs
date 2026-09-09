// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2018 Intel Corporation. All rights reserved. */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

unsafe fn firmware_activate_noidle_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let nvdimm_bus = to_nvdimm_bus(dev);
    let nd_desc = to_nd_desc(nvdimm_bus);
    let acpi_desc = to_acpi_desc(nd_desc);
    sprintf(buf, c"%s\n".as_ptr(), if (*acpi_desc).fwa_noidle { c"Y\0".as_ptr() } else { c"N\0".as_ptr() })
}

unsafe fn firmware_activate_noidle_store(dev: *mut device, _attr: *mut device_attribute, buf: *const i8, size: usize) -> ssize_t {
    let nvdimm_bus = to_nvdimm_bus(dev);
    let nd_desc = to_nd_desc(nvdimm_bus);
    let acpi_desc = to_acpi_desc(nd_desc);
    let mut val = false;
    let rc = kstrtobool(buf, &mut val);
    if rc != 0 { return rc; }
    if val != (*acpi_desc).fwa_noidle { (*acpi_desc).fwa_cap = NVDIMM_FWA_CAP_INVALID; }
    (*acpi_desc).fwa_noidle = val;
    size as ssize_t
}

// DEVICE_ATTR_RW(firmware_activate_noidle)

pub unsafe fn intel_fwa_supported(nvdimm_bus: *mut nvdimm_bus) -> bool {
    let nd_desc = to_nd_desc(nvdimm_bus);
    let acpi_desc = to_acpi_desc(nd_desc);
    if !test_bit(NVDIMM_BUS_FAMILY_INTEL, &(*nd_desc).bus_family_mask) { return false; }
    let mask = &(*acpi_desc).family_dsm_mask[NVDIMM_BUS_FAMILY_INTEL as usize];
    *mask == NVDIMM_BUS_INTEL_FW_ACTIVATE_CMDMASK
}

unsafe fn intel_security_flags(nvdimm: *mut nvdimm, ptype: nvdimm_passphrase_type) -> c_ulong {
    let nfit_mem = nvdimm_provider_data(nvdimm);
    let mut security_flags: c_ulong = 0;
    let mut nd_cmd: nd_cmd_pkg_nd_intel_get_security_state = Default::default();
    nd_cmd.pkg.nd_command = NVDIMM_INTEL_GET_SECURITY_STATE;
    nd_cmd.pkg.nd_family = NVDIMM_FAMILY_INTEL;
    nd_cmd.pkg.nd_size_out = size_of::<nd_intel_get_security_state>();
    nd_cmd.pkg.nd_fw_size = size_of::<nd_intel_get_security_state>();
    if !test_bit(NVDIMM_INTEL_GET_SECURITY_STATE, &(*nfit_mem).dsm_mask) { return 0; }
    if nvdimm_in_overwrite(nvdimm) && ptype == NVDIMM_USER { return BIT(NVDIMM_SECURITY_OVERWRITE); }
    let rc = nvdimm_ctl(nvdimm, ND_CMD_CALL, &mut nd_cmd, size_of_val(&nd_cmd), null_mut());
    if rc < 0 || nd_cmd.cmd.status != 0 { pr_err(c"%s: security state retrieval failed (%d:%#x)\n".as_ptr(), nvdimm_name(nvdimm), rc, nd_cmd.cmd.status); return 0; }
    if ptype == NVDIMM_MASTER {
        if nd_cmd.cmd.extended_state & ND_INTEL_SEC_ESTATE_ENABLED != 0 { set_bit(NVDIMM_SECURITY_UNLOCKED, &mut security_flags); } else { set_bit(NVDIMM_SECURITY_DISABLED, &mut security_flags); }
        if nd_cmd.cmd.extended_state & ND_INTEL_SEC_ESTATE_PLIMIT != 0 { set_bit(NVDIMM_SECURITY_FROZEN, &mut security_flags); }
        return security_flags;
    }
    if nd_cmd.cmd.state & ND_INTEL_SEC_STATE_UNSUPPORTED != 0 { return 0; }
    if nd_cmd.cmd.state & ND_INTEL_SEC_STATE_ENABLED != 0 {
        if nd_cmd.cmd.state & (ND_INTEL_SEC_STATE_FROZEN | ND_INTEL_SEC_STATE_PLIMIT) != 0 { set_bit(NVDIMM_SECURITY_FROZEN, &mut security_flags); }
        if nd_cmd.cmd.state & ND_INTEL_SEC_STATE_LOCKED != 0 { set_bit(NVDIMM_SECURITY_LOCKED, &mut security_flags); } else { set_bit(NVDIMM_SECURITY_UNLOCKED, &mut security_flags); }
    } else { set_bit(NVDIMM_SECURITY_DISABLED, &mut security_flags); }
    security_flags
}

unsafe fn intel_security_freeze(nvdimm: *mut nvdimm) -> c_int {
    let nfit_mem = nvdimm_provider_data(nvdimm);
    let mut nd_cmd: nd_cmd_pkg_nd_intel_freeze_lock = Default::default();
    nd_cmd.pkg.nd_command = NVDIMM_INTEL_FREEZE_LOCK; nd_cmd.pkg.nd_family = NVDIMM_FAMILY_INTEL; nd_cmd.pkg.nd_size_out = ND_INTEL_STATUS_SIZE; nd_cmd.pkg.nd_fw_size = ND_INTEL_STATUS_SIZE;
    if !test_bit(NVDIMM_INTEL_FREEZE_LOCK, &(*nfit_mem).dsm_mask) { return -ENOTTY; }
    let rc = nvdimm_ctl(nvdimm, ND_CMD_CALL, &mut nd_cmd, size_of_val(&nd_cmd), null_mut());
    if rc < 0 { return rc; } if nd_cmd.cmd.status != 0 { return -EIO; } 0
}

unsafe fn intel_security_change_key(nvdimm: *mut nvdimm, old_data: *const nvdimm_key_data, new_data: *const nvdimm_key_data, ptype: nvdimm_passphrase_type) -> c_int {
    let nfit_mem = nvdimm_provider_data(nvdimm);
    let cmd = if ptype == NVDIMM_MASTER { NVDIMM_INTEL_SET_MASTER_PASSPHRASE } else { NVDIMM_INTEL_SET_PASSPHRASE };
    let mut nd_cmd: nd_cmd_pkg_nd_intel_set_passphrase = Default::default();
    nd_cmd.pkg.nd_family = NVDIMM_FAMILY_INTEL; nd_cmd.pkg.nd_size_in = ND_INTEL_PASSPHRASE_SIZE * 2; nd_cmd.pkg.nd_size_out = ND_INTEL_STATUS_SIZE; nd_cmd.pkg.nd_fw_size = ND_INTEL_STATUS_SIZE; nd_cmd.pkg.nd_command = cmd;
    if !test_bit(cmd, &(*nfit_mem).dsm_mask) { return -ENOTTY; }
    memcpy(nd_cmd.cmd.old_pass.as_mut_ptr(), (*old_data).data.as_ptr(), size_of_val(&nd_cmd.cmd.old_pass)); memcpy(nd_cmd.cmd.new_pass.as_mut_ptr(), (*new_data).data.as_ptr(), size_of_val(&nd_cmd.cmd.new_pass));
    let rc = nvdimm_ctl(nvdimm, ND_CMD_CALL, &mut nd_cmd, size_of_val(&nd_cmd), null_mut()); if rc < 0 { return rc; }
    match nd_cmd.cmd.status { 0 => 0, ND_INTEL_STATUS_INVALID_PASS => -EINVAL, ND_INTEL_STATUS_NOT_SUPPORTED => -EOPNOTSUPP, _ => -EIO }
}

unsafe fn intel_security_unlock(nvdimm: *mut nvdimm, key_data: *const nvdimm_key_data) -> c_int { security_simple(nvdimm, NVDIMM_INTEL_UNLOCK_UNIT, &(*key_data).data, -EINVAL, -EIO) }
unsafe fn intel_security_disable(nvdimm: *mut nvdimm, key_data: *const nvdimm_key_data) -> c_int { security_simple(nvdimm, NVDIMM_INTEL_DISABLE_PASSPHRASE, &(*key_data).data, -EINVAL, -ENXIO) }
unsafe fn intel_security_erase(nvdimm: *mut nvdimm, key: *const nvdimm_key_data, ptype: nvdimm_passphrase_type) -> c_int { security_simple(nvdimm, if ptype == NVDIMM_MASTER { NVDIMM_INTEL_MASTER_SECURE_ERASE } else { NVDIMM_INTEL_SECURE_ERASE }, &(*key).data, -EINVAL, -ENXIO) }

unsafe fn security_simple(nvdimm: *mut nvdimm, cmd: c_uint, data: &[u8], invalid: c_int, other: c_int) -> c_int {
    let nfit_mem = nvdimm_provider_data(nvdimm); if !test_bit(cmd, &(*nfit_mem).dsm_mask) { return -ENOTTY; }
    let mut packet = nd_cmd_pkg_status::new(cmd); memcpy(packet.passphrase.as_mut_ptr(), data.as_ptr(), packet.passphrase.len()); let rc = nvdimm_ctl(nvdimm, ND_CMD_CALL, &mut packet, size_of_val(&packet), null_mut()); if rc < 0 { return rc; }
    match packet.status { 0 => 0, ND_INTEL_STATUS_INVALID_PASS => invalid, _ => other }
}

unsafe fn intel_security_query_overwrite(nvdimm: *mut nvdimm) -> c_int { security_query(nvdimm, NVDIMM_INTEL_QUERY_OVERWRITE) }
unsafe fn intel_security_overwrite(nvdimm: *mut nvdimm, key: *const nvdimm_key_data) -> c_int { security_simple(nvdimm, NVDIMM_INTEL_OVERWRITE, &(*key).data, -EINVAL, -ENXIO) }

// CONFIG_X86 conditionally includes unlock, erase, overwrite, and query_overwrite.
pub static mut intel_security_ops: *const nvdimm_security_ops = &__intel_security_ops;
static __intel_security_ops: nvdimm_security_ops = nvdimm_security_ops { get_flags: intel_security_flags, freeze: intel_security_freeze, change_key: intel_security_change_key, disable: intel_security_disable, unlock: intel_security_unlock, erase: intel_security_erase, overwrite: intel_security_overwrite, query_overwrite: intel_security_query_overwrite };

unsafe fn intel_bus_fwa_businfo(nd_desc: *mut nvdimm_bus_descriptor, info: *mut nd_intel_bus_fw_activate_businfo) -> c_int { let mut cmd = nd_cmd_pkg_businfo::new(); let rc = ((*nd_desc).ndctl)(nd_desc, null_mut(), ND_CMD_CALL, &mut cmd, size_of_val(&cmd), null_mut()); *info = cmd.cmd; rc }

unsafe fn intel_bus_fwa_state(nd_desc: *mut nvdimm_bus_descriptor) -> nvdimm_fwa_state {
    let acpi_desc = to_acpi_desc(nd_desc); let mut info = Default::default(); let dev = (*acpi_desc).dev;
    match (*acpi_desc).fwa_state { NVDIMM_FWA_INVALID | NVDIMM_FWA_BUSY => {}, _ => { if (*acpi_desc).fwa_cap != NVDIMM_FWA_CAP_INVALID { return (*acpi_desc).fwa_state; } } }
    if intel_bus_fwa_businfo(nd_desc, &mut info) != 0 { return NVDIMM_FWA_INVALID; }
    let state = match info.state { ND_INTEL_FWA_IDLE => NVDIMM_FWA_IDLE, ND_INTEL_FWA_BUSY => NVDIMM_FWA_BUSY, ND_INTEL_FWA_ARMED => if info.activate_tmo > info.max_quiesce_tmo { NVDIMM_FWA_ARM_OVERFLOW } else { NVDIMM_FWA_ARMED }, _ => { dev_err_once(dev, c"invalid firmware activate state %d\n".as_ptr(), info.state); return NVDIMM_FWA_INVALID; } };
    if (*acpi_desc).fwa_cap == NVDIMM_FWA_CAP_INVALID { (*acpi_desc).fwa_cap = if info.capability & ND_INTEL_BUS_FWA_CAP_FWQUIESCE != 0 { NVDIMM_FWA_CAP_QUIESCE } else if info.capability & ND_INTEL_BUS_FWA_CAP_OSQUIESCE != 0 { NVDIMM_FWA_CAP_LIVE } else { NVDIMM_FWA_CAP_NONE }; }
    (*acpi_desc).fwa_state = state; state
}

unsafe fn intel_bus_fwa_capability(nd_desc: *mut nvdimm_bus_descriptor) -> nvdimm_fwa_capability { let a = to_acpi_desc(nd_desc); if (*a).fwa_cap > NVDIMM_FWA_CAP_INVALID { return (*a).fwa_cap; } if intel_bus_fwa_state(nd_desc) > NVDIMM_FWA_INVALID { return (*a).fwa_cap; } NVDIMM_FWA_CAP_INVALID }

unsafe fn intel_bus_fwa_activate(nd_desc: *mut nvdimm_bus_descriptor) -> c_int { let a = to_acpi_desc(nd_desc); let mut cmd = nd_cmd_pkg_bus_activate::new(); cmd.pkg.nd_command = NVDIMM_BUS_INTEL_FW_ACTIVATE; cmd.pkg.nd_family = NVDIMM_BUS_FAMILY_INTEL; cmd.cmd.iodev_state = if (*a).fwa_noidle { ND_INTEL_BUS_FWA_IODEV_OS_IDLE } else { ND_INTEL_BUS_FWA_IODEV_FORCE_IDLE }; match intel_bus_fwa_state(nd_desc) { NVDIMM_FWA_ARMED | NVDIMM_FWA_ARM_OVERFLOW => {}, _ => return -ENXIO }; let rc = ((*nd_desc).ndctl)(nd_desc, null_mut(), ND_CMD_CALL, &mut cmd, size_of_val(&cmd), null_mut()); (*a).fwa_state = NVDIMM_FWA_INVALID; (*a).fwa_count += 1; dev_dbg((*a).dev, c"result: %d\n".as_ptr(), rc); rc }

static __intel_bus_fw_ops: nvdimm_bus_fw_ops = nvdimm_bus_fw_ops { activate_state: intel_bus_fwa_state, capability: intel_bus_fwa_capability, activate: intel_bus_fwa_activate };
pub static mut intel_bus_fw_ops: *const nvdimm_bus_fw_ops = &__intel_bus_fw_ops;

unsafe fn intel_fwa_dimminfo(nvdimm: *mut nvdimm, info: *mut nd_intel_fw_activate_dimminfo) -> c_int { let mut cmd = nd_cmd_pkg_dimminfo::new(); let rc = nvdimm_ctl(nvdimm, ND_CMD_CALL, &mut cmd, size_of_val(&cmd), null_mut()); *info = cmd.cmd; rc }
unsafe fn intel_fwa_state(nvdimm: *mut nvdimm) -> nvdimm_fwa_state { let n = nvdimm_provider_data(nvdimm); let a = (*n).acpi_desc; let mut info = Default::default(); if (*n).fwa_state != NVDIMM_FWA_INVALID && (*n).fwa_state != NVDIMM_FWA_BUSY && (*n).fwa_count == (*a).fwa_count { return (*n).fwa_state; } if intel_fwa_dimminfo(nvdimm, &mut info) != 0 { return NVDIMM_FWA_INVALID; } (*n).fwa_state = match info.state { ND_INTEL_FWA_IDLE => NVDIMM_FWA_IDLE, ND_INTEL_FWA_BUSY => NVDIMM_FWA_BUSY, ND_INTEL_FWA_ARMED => NVDIMM_FWA_ARMED, _ => NVDIMM_FWA_INVALID }; (*n).fwa_result = match info.result { ND_INTEL_DIMM_FWA_NONE => NVDIMM_FWA_RESULT_NONE, ND_INTEL_DIMM_FWA_SUCCESS => NVDIMM_FWA_RESULT_SUCCESS, ND_INTEL_DIMM_FWA_NOTSTAGED => NVDIMM_FWA_RESULT_NOTSTAGED, ND_INTEL_DIMM_FWA_NEEDRESET => NVDIMM_FWA_RESULT_NEEDRESET, _ => NVDIMM_FWA_RESULT_FAIL }; (*n).fwa_count = (*a).fwa_count; (*n).fwa_state }
unsafe fn intel_fwa_result(nvdimm: *mut nvdimm) -> nvdimm_fwa_result { let n = nvdimm_provider_data(nvdimm); let a = (*n).acpi_desc; if (*n).fwa_count == (*a).fwa_count && (*n).fwa_result > NVDIMM_FWA_RESULT_INVALID { return (*n).fwa_result; } if intel_fwa_state(nvdimm) > NVDIMM_FWA_INVALID { (*n).fwa_result } else { NVDIMM_FWA_RESULT_INVALID } }
unsafe fn intel_fwa_arm(nvdimm: *mut nvdimm, arm: nvdimm_fwa_trigger) -> c_int { let n = nvdimm_provider_data(nvdimm); let a = (*n).acpi_desc; match intel_fwa_state(nvdimm) { NVDIMM_FWA_INVALID => return -ENXIO, NVDIMM_FWA_BUSY => return -EBUSY, NVDIMM_FWA_IDLE if arm == NVDIMM_FWA_DISARM => return 0, NVDIMM_FWA_ARMED if arm == NVDIMM_FWA_ARM => return 0, _ => {} }; (*a).fwa_state = NVDIMM_FWA_INVALID; (*n).fwa_state = NVDIMM_FWA_INVALID; let mut cmd = nd_cmd_pkg_arm::new(); cmd.cmd.activate_arm = if arm == NVDIMM_FWA_ARM { ND_INTEL_DIMM_FWA_ARM } else { ND_INTEL_DIMM_FWA_DISARM }; let rc = nvdimm_ctl(nvdimm, ND_CMD_CALL, &mut cmd, size_of_val(&cmd), null_mut()); dev_dbg((*a).dev, c"%s result: %d\n".as_ptr(), if arm == NVDIMM_FWA_ARM { c"arm\0".as_ptr() } else { c"disarm\0".as_ptr() }, rc); rc }
static __intel_fw_ops: nvdimm_fw_ops = nvdimm_fw_ops { activate_state: intel_fwa_state, activate_result: intel_fwa_result, arm: intel_fwa_arm };
pub static mut intel_fw_ops: *const nvdimm_fw_ops = &__intel_fw_ops;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
