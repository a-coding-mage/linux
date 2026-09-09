// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2024-2025 Intel Corporation. All rights reserved. */
// Translated from features.c; external kernel types, constants, and helpers are
// supplied by the surrounding repository.

static CXL_EXCLUSIVE_FEATS: [uuid_t; 8] = [
    CXL_FEAT_PATROL_SCRUB_UUID, CXL_FEAT_ECS_UUID, CXL_FEAT_SPPR_UUID,
    CXL_FEAT_HPPR_UUID, CXL_FEAT_CACHELINE_SPARING_UUID,
    CXL_FEAT_ROW_SPARING_UUID, CXL_FEAT_BANK_SPARING_UUID,
    CXL_FEAT_RANK_SPARING_UUID,
];

unsafe fn is_cxl_feature_exclusive_by_uuid(uuid: *const uuid_t) -> bool {
    for i in 0..CXL_EXCLUSIVE_FEATS.len() {
        if uuid_equal(uuid, &CXL_EXCLUSIVE_FEATS[i]) { return true; }
    }
    false
}

unsafe fn is_cxl_feature_exclusive(entry: *mut cxl_feat_entry) -> bool {
    is_cxl_feature_exclusive_by_uuid(&(*entry).uuid)
}

pub unsafe fn to_cxlfs(cxlds: *mut cxl_dev_state) -> *mut cxl_features_state { (*cxlds).cxlfs }

unsafe fn cxl_get_supported_features_count(cxl_mbox: *mut cxl_mailbox) -> i32 {
    let mut mbox_out: cxl_mbox_get_sup_feats_out = core::mem::zeroed();
    let mut mbox_in: cxl_mbox_get_sup_feats_in = core::mem::zeroed();
    mbox_in.count = cpu_to_le32(core::mem::size_of::<cxl_mbox_get_sup_feats_out>() as _);
    let mut mbox_cmd = cxl_mbox_cmd {
        opcode: CXL_MBOX_OP_GET_SUPPORTED_FEATURES,
        size_in: core::mem::size_of_val(&mbox_in), payload_in: &mut mbox_in as *mut _ as _,
        size_out: core::mem::size_of_val(&mbox_out), payload_out: &mut mbox_out as *mut _ as _,
        min_out: core::mem::size_of_val(&mbox_out), ..core::mem::zeroed()
    };
    let rc = cxl_internal_send_cmd(cxl_mbox, &mut mbox_cmd);
    if rc < 0 { return rc; }
    le16_to_cpu(mbox_out.supported_feats) as i32
}

unsafe fn get_supported_features(cxlfs: *mut cxl_features_state) -> *mut cxl_feat_entries {
    let cxl_mbox = &mut (*(*cxlfs).cxlds).cxl_mbox;
    let count = cxl_get_supported_features_count(cxl_mbox);
    if count <= 0 { return core::ptr::null_mut(); }
    let entries = kvmalloc_flex::<cxl_feat_entries>(count as usize);
    if entries.is_null() { return core::ptr::null_mut(); }
    let mbox_out = kvmalloc(cxl_mbox.payload_size, GFP_KERNEL) as *mut cxl_mbox_get_sup_feats_out;
    if mbox_out.is_null() { return core::ptr::null_mut(); }
    let hdr_size = struct_size::<cxl_mbox_get_sup_feats_out>(0);
    let feat_size = core::mem::size_of::<cxl_feat_entry>();
    let max_feats = (cxl_mbox.payload_size - hdr_size) / feat_size;
    let mut entry = (*entries).ent.as_mut_ptr();
    let mut start = 0; let mut remain = count as usize; let mut user_feats = 0;
    while remain != 0 {
        let copy_feats = core::cmp::min(remain, max_feats);
        let next_remain = if remain > max_feats { remain - max_feats } else { 0 };
        let alloc_size = hdr_size + copy_feats * feat_size;
        let mut input: cxl_mbox_get_sup_feats_in = core::mem::zeroed();
        input.count = cpu_to_le32(alloc_size as _); input.start_idx = cpu_to_le16(start as _);
        core::ptr::write_bytes(mbox_out as *mut u8, 0, alloc_size);
        let mut cmd: cxl_mbox_cmd = core::mem::zeroed();
        cmd.opcode = CXL_MBOX_OP_GET_SUPPORTED_FEATURES; cmd.size_in = core::mem::size_of_val(&input);
        cmd.payload_in = &mut input as *mut _ as _; cmd.size_out = alloc_size; cmd.payload_out = mbox_out as _; cmd.min_out = hdr_size;
        if cxl_internal_send_cmd(cxl_mbox, &mut cmd) < 0 || cmd.size_out <= hdr_size { return core::ptr::null_mut(); }
        let retrieved = cmd.size_out - hdr_size;
        if retrieved % feat_size != 0 { return core::ptr::null_mut(); }
        let num = le16_to_cpu((*mbox_out).num_entries) as usize;
        if num * feat_size != retrieved { return core::ptr::null_mut(); }
        core::ptr::copy_nonoverlapping((*mbox_out).ents.as_ptr() as *const u8, entry as *mut u8, retrieved);
        for i in 0..num { if !is_cxl_feature_exclusive(entry.add(i)) { user_feats += 1; } }
        entry = entry.add(num); remain = next_remain + copy_feats - num; start += num;
    }
    (*entries).num_features = count as _; (*entries).num_user_features = user_feats as _; entries
}

pub unsafe fn devm_cxl_setup_features(cxlds: *mut cxl_dev_state) -> i32 {
    if (*cxlds).cxl_mbox.feat_cap < CXL_FEATURES_RO { return -ENODEV; }
    let cxlfs = kzalloc_obj::<cxl_features_state>(); if cxlfs.is_null() { return -ENOMEM; }
    (*cxlfs).cxlds = cxlds; (*cxlfs).entries = get_supported_features(cxlfs);
    if (*cxlfs).entries.is_null() { return -ENOMEM; } (*cxlds).cxlfs = cxlfs; 0
}

pub unsafe fn cxl_get_feature(cxl_mbox: *mut cxl_mailbox, feat_uuid: *const uuid_t, selection: enum_cxl_get_feat_selection, feat_out: *mut u8, feat_out_size: usize, offset: u16, return_code: *mut u16) -> usize {
    if !return_code.is_null() { *return_code = CXL_MBOX_CMD_RC_INPUT; }
    if feat_out.is_null() || feat_out_size == 0 { return 0; }
    let mut pi: cxl_mbox_get_feat_in = core::mem::zeroed(); uuid_copy(&mut pi.uuid, feat_uuid); pi.selection = selection;
    let mut received = 0;
    while received < feat_out_size {
        let amount = core::cmp::min(feat_out_size - received, (*cxl_mbox).payload_size);
        pi.offset = cpu_to_le16(offset.wrapping_add(received as u16)); pi.count = cpu_to_le16(amount as _);
        let mut cmd: cxl_mbox_cmd = core::mem::zeroed(); cmd.opcode = CXL_MBOX_OP_GET_FEATURE; cmd.size_in = core::mem::size_of_val(&pi); cmd.payload_in = &mut pi as *mut _ as _; cmd.size_out = amount; cmd.payload_out = feat_out.add(received) as _; cmd.min_out = amount;
        if cxl_internal_send_cmd(cxl_mbox, &mut cmd) < 0 || cmd.size_out == 0 { if !return_code.is_null() { *return_code = cmd.return_code; } return 0; } received += cmd.size_out;
    }
    if !return_code.is_null() { *return_code = CXL_MBOX_CMD_RC_SUCCESS; } received
}

pub unsafe fn cxl_set_feature(cxl_mbox: *mut cxl_mailbox, feat_uuid: *const uuid_t, version: u8, data: *const u8, data_size: usize, mut flags: u32, offset: u16, return_code: *mut u16) -> i32 {
    if !return_code.is_null() { *return_code = CXL_MBOX_CMD_RC_INPUT; }
    let pi = kcalloc((*cxl_mbox).payload_size, 1, GFP_KERNEL) as *mut cxl_mbox_set_feat_in; if pi.is_null() { return -ENOMEM; }
    uuid_copy(&mut (*pi).uuid, feat_uuid); (*pi).version = version; flags = (flags & !CXL_SET_FEAT_FLAG_DATA_TRANSFER_MASK) | CXL_SET_FEAT_FLAG_DATA_SAVED_ACROSS_RESET;
    let hdr = core::mem::size_of_val(&(*pi).hdr); if hdr + FEAT_DATA_MIN_PAYLOAD_SIZE > (*cxl_mbox).payload_size { return -ENOMEM; }
    let mut amount = if hdr + data_size <= (*cxl_mbox).payload_size { (*pi).flags = cpu_to_le32(flags | CXL_SET_FEAT_FLAG_FULL_DATA_TRANSFER); data_size } else { (*pi).flags = cpu_to_le32(flags | CXL_SET_FEAT_FLAG_INITIATE_DATA_TRANSFER); (*cxl_mbox).payload_size - hdr };
    let mut sent = 0; loop { (*pi).offset = cpu_to_le16(offset.wrapping_add(sent as u16)); core::ptr::copy_nonoverlapping(data.add(sent), (*pi).feat_data.as_mut_ptr(), amount); let mut cmd: cxl_mbox_cmd = core::mem::zeroed(); cmd.opcode = CXL_MBOX_OP_SET_FEATURE; cmd.size_in = hdr + amount; cmd.payload_in = pi as _; let rc = cxl_internal_send_cmd(cxl_mbox, &mut cmd); if rc < 0 { if !return_code.is_null() { *return_code = cmd.return_code; } return rc; } sent += amount; if sent >= data_size { if !return_code.is_null() { *return_code = CXL_MBOX_CMD_RC_SUCCESS; } return 0; } amount = core::cmp::min(data_size - sent, (*cxl_mbox).payload_size - hdr); (*pi).flags = cpu_to_le32(flags | if sent + amount >= data_size { CXL_SET_FEAT_FLAG_FINISH_DATA_TRANSFER } else { CXL_SET_FEAT_FLAG_CONTINUE_DATA_TRANSFER }); }
}

const FEAT_DATA_MIN_PAYLOAD_SIZE: usize = 10;

unsafe fn free_cxlfs(p: *mut u8) { let c = p as *mut cxl_features_state; (*(*c).cxlds).cxlfs = core::ptr::null_mut(); kvfree((*c).entries as _); kfree(c as _); }
unsafe fn fwctl_to_memdev(d: *mut fwctl_device) -> *mut cxl_memdev { to_cxl_memdev((*d).dev.parent) }
unsafe fn cxlctl_get_supported_features(_: *mut cxl_features_state, _: *const fwctl_rpc_cxl, _: *mut usize) -> *mut u8 { ERR_PTR(-EOPNOTSUPP) }
unsafe fn cxlctl_get_feature(_: *mut cxl_features_state, _: *const fwctl_rpc_cxl, _: *mut usize) -> *mut u8 { ERR_PTR(-EOPNOTSUPP) }
unsafe fn cxlctl_set_feature(_: *mut cxl_features_state, _: *const fwctl_rpc_cxl, _: *mut usize) -> *mut u8 { ERR_PTR(-EOPNOTSUPP) }
unsafe fn cxlctl_handle_commands(c: *mut cxl_features_state, r: *const fwctl_rpc_cxl, o: *mut usize, op: u16) -> *mut u8 { match op { CXL_MBOX_OP_GET_SUPPORTED_FEATURES => cxlctl_get_supported_features(c,r,o), CXL_MBOX_OP_GET_FEATURE => cxlctl_get_feature(c,r,o), CXL_MBOX_OP_SET_FEATURE => cxlctl_set_feature(c,r,o), _ => ERR_PTR(-EOPNOTSUPP) } }

pub unsafe fn cxl_feature_info(cxlfs: *mut cxl_features_state, uuid: *const uuid_t) -> *mut cxl_feat_entry {
    if cxlfs.is_null() || (*cxlfs).entries.is_null() { return ERR_PTR(-EOPNOTSUPP); }
    for i in 0..(*(*cxlfs).entries).num_features as usize { let p = (*(*cxlfs).entries).ent.as_mut_ptr().add(i); if uuid_equal(uuid, &(*p).uuid) { return p; } }
    ERR_PTR(-EINVAL)
}

unsafe fn cxlctl_validate_set_features(cxlfs: *mut cxl_features_state, rpc: *const fwctl_rpc_cxl, scope: enum_fwctl_rpc_scope) -> bool {
    if (*rpc).op_size < core::mem::size_of::<uuid_t>() { return false; }
    let feat = cxl_feature_info(cxlfs, &(*rpc).set_feat_in.uuid); if IS_ERR(feat) { return false; }
    let flags = le32_to_cpu((*feat).flags); if flags & CXL_FEATURE_F_CHANGEABLE == 0 { return false; }
    let effects = le16_to_cpu((*feat).effects); if effects & CXL_CMD_EFFECTS_RESERVED != 0 || effects & CXL_CMD_BACKGROUND != 0 { return false; }
    let imm = CXL_CMD_CONFIG_CHANGE_IMMEDIATE | CXL_CMD_DATA_CHANGE_IMMEDIATE | CXL_CMD_POLICY_CHANGE_IMMEDIATE | CXL_CMD_LOG_CHANGE_IMMEDIATE;
    let reset = CXL_CMD_CONFIG_CHANGE_COLD_RESET | CXL_CMD_CONFIG_CHANGE_CONV_RESET | CXL_CMD_CONFIG_CHANGE_CXL_RESET;
    if effects & imm == 0 && effects & reset == 0 { return false; }
    if effects & imm != 0 { scope >= FWCTL_RPC_DEBUG_WRITE_FULL } else { scope >= FWCTL_RPC_DEBUG_WRITE }
}

unsafe fn cxlctl_validate_hw_command(cxlfs: *mut cxl_features_state, rpc: *const fwctl_rpc_cxl, scope: enum_fwctl_rpc_scope, opcode: u16) -> bool {
    let cap = (*(*cxlfs).cxlds).cxl_mbox.feat_cap;
    match opcode { CXL_MBOX_OP_GET_SUPPORTED_FEATURES | CXL_MBOX_OP_GET_FEATURE => cap >= CXL_FEATURES_RO, CXL_MBOX_OP_SET_FEATURE => cap >= CXL_FEATURES_RW && cxlctl_validate_set_features(cxlfs, rpc, scope), _ => false }
}

unsafe fn cxlctl_open_uctx(_: *mut fwctl_uctx) -> i32 { 0 }
unsafe fn cxlctl_close_uctx(_: *mut fwctl_uctx) {}

// FWCTL RPC dispatch and device-registration glue. The referenced FWCTL and
// CXL structures/functions are external dependencies supplied by the kernel.
unsafe fn cxlctl_fw_rpc(uctx: *mut fwctl_uctx, scope: enum_fwctl_rpc_scope, input: *mut u8, input_len: usize, out_len: *mut usize) -> *mut u8 {
    let dev = (*uctx).fwctl; let cxlmd = to_cxl_memdev((*dev).dev.parent); let cxlfs = to_cxlfs((*cxlmd).cxlds); let rpc = input as *const fwctl_rpc_cxl;
    if input_len < core::mem::size_of::<fwctl_rpc_hdr>() || (*rpc).op_size > input_len - core::mem::size_of::<fwctl_rpc_hdr>() { return ERR_PTR(-EINVAL); }
    if !cxlctl_validate_hw_command(cxlfs, rpc, scope, (*rpc).opcode) { return ERR_PTR(-EINVAL); }
    cxlctl_handle_commands(cxlfs, rpc, out_len, (*rpc).opcode)
}

unsafe fn free_memdev_fwctl(p: *mut u8) { let d = p as *mut fwctl_device; fwctl_unregister(d); fwctl_put(d); }

pub unsafe fn devm_cxl_setup_fwctl(host: *mut device, cxlmd: *mut cxl_memdev) -> i32 {
    let cxlfs = to_cxlfs((*cxlmd).cxlds); if cxlfs.is_null() { return -ENODEV; }
    if (*(*cxlfs).entries).num_user_features == 0 { return -ENODEV; }
    let fwctl = _fwctl_alloc_device(&mut (*cxlmd).dev, core::ptr::null(), core::mem::size_of::<fwctl_device>()); if fwctl.is_null() { return -ENOMEM; }
    let rc = fwctl_register(fwctl); if rc != 0 { return rc; }
    devm_add_action_or_reset(host, free_memdev_fwctl, fwctl)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
