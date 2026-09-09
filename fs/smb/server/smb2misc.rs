// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) 2016 Namjae Jeon <linkinjeon@kernel.org>
 *   Copyright (C) 2018 Samsung Electronics Co., Ltd.
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn check_smb2_hdr(hdr: *mut smb2_hdr) -> i32 {
    /* Make sure that this really is an SMB, that it is a response. */
    if (*hdr).Flags & SMB2_FLAGS_SERVER_TO_REDIR != 0 { 1 } else { 0 }
}

static SMB2_REQ_STRUCT_SIZES: [__le16; NUMBER_OF_SMB2_COMMANDS] = [
    cpu_to_le16(36), cpu_to_le16(25), cpu_to_le16(4), cpu_to_le16(9),
    cpu_to_le16(4), cpu_to_le16(57), cpu_to_le16(24), cpu_to_le16(24),
    cpu_to_le16(49), cpu_to_le16(49), cpu_to_le16(48), cpu_to_le16(57),
    cpu_to_le16(4), cpu_to_le16(4), cpu_to_le16(33), cpu_to_le16(32),
    cpu_to_le16(41), cpu_to_le16(33), cpu_to_le16(36),
];

static HAS_SMB2_DATA_AREA: [bool; NUMBER_OF_SMB2_COMMANDS] = [
    true, true, false, true, false, true, false, false, true, true,
    true, true, false, false, true, false, true, true, false,
];

unsafe fn smb2_get_data_area_len(off: *mut u32, len: *mut u32,
                                 hdr: *mut smb2_hdr) -> i32 {
    let mut ret: i32 = 0;
    *off = 0; *len = 0;
    match (*hdr).Command {
        SMB2_SESSION_SETUP => { *off = le16_to_cpu((*((hdr as *mut smb2_sess_setup_req))).SecurityBufferOffset) as u32; *len = le16_to_cpu((*((hdr as *mut smb2_sess_setup_req))).SecurityBufferLength) as u32; }
        SMB2_TREE_CONNECT => { *off = max_t::<u16>(le16_to_cpu((*((hdr as *mut smb2_tree_connect_req))).PathOffset), offsetof::<smb2_tree_connect_req>("Buffer")) as u32; *len = le16_to_cpu((*((hdr as *mut smb2_tree_connect_req))).PathLength) as u32; }
        SMB2_CREATE => {
            let name_off = max_t::<u16>(le16_to_cpu((*((hdr as *mut smb2_create_req))).NameOffset), offsetof::<smb2_create_req>("Buffer"));
            let name_len = le16_to_cpu((*((hdr as *mut smb2_create_req))).NameLength);
            if (*((hdr as *mut smb2_create_req))).CreateContextsLength != 0 {
                *off = le32_to_cpu((*((hdr as *mut smb2_create_req))).CreateContextsOffset);
                *len = le32_to_cpu((*((hdr as *mut smb2_create_req))).CreateContextsLength);
                if name_len == 0 { return ret; }
                if (name_off as u64 + name_len as u64) < (*off as u64 + *len as u64) { return ret; }
            }
            *off = name_off as u32; *len = name_len as u32;
        }
        SMB2_QUERY_INFO => { *off = max_t::<u32>(le16_to_cpu((*((hdr as *mut smb2_query_info_req))).InputBufferOffset) as u32, offsetof::<smb2_query_info_req>("Buffer")); *len = le32_to_cpu((*((hdr as *mut smb2_query_info_req))).InputBufferLength); }
        SMB2_SET_INFO => { *off = max_t::<u32>(le16_to_cpu((*((hdr as *mut smb2_set_info_req))).BufferOffset) as u32, offsetof::<smb2_set_info_req>("Buffer")); *len = le32_to_cpu((*((hdr as *mut smb2_set_info_req))).BufferLength); }
        SMB2_READ => { *off = le16_to_cpu((*((hdr as *mut smb2_read_req))).ReadChannelInfoOffset) as u32; *len = le16_to_cpu((*((hdr as *mut smb2_read_req))).ReadChannelInfoLength) as u32; }
        SMB2_WRITE => {
            let h = hdr as *mut smb2_write_req;
            if (*h).DataOffset != 0 || (*h).Length != 0 { *off = max_t::<u16>(le16_to_cpu((*h).DataOffset), offsetof::<smb2_write_req>("Buffer")) as u32; *len = le32_to_cpu((*h).Length); }
            else { *off = le16_to_cpu((*h).WriteChannelInfoOffset) as u32; *len = le16_to_cpu((*h).WriteChannelInfoLength) as u32; }
        }
        SMB2_QUERY_DIRECTORY => { *off = max_t::<u16>(le16_to_cpu((*((hdr as *mut smb2_query_directory_req))).FileNameOffset), offsetof::<smb2_query_directory_req>("Buffer")) as u32; *len = le16_to_cpu((*((hdr as *mut smb2_query_directory_req))).FileNameLength) as u32; }
        SMB2_LOCK => { let n = le16_to_cpu((*((hdr as *mut smb2_lock_req))).LockCount); if n > 0 { *off = offsetof::<smb2_lock_req>("locks"); *len = size_of::<smb2_lock_element>() as u32 * n as u32; } }
        SMB2_IOCTL => { *off = max_t::<u32>(le32_to_cpu((*((hdr as *mut smb2_ioctl_req))).InputOffset), offsetof::<smb2_ioctl_req>("Buffer")); *len = le32_to_cpu((*((hdr as *mut smb2_ioctl_req))).InputCount); }
        _ => { ksmbd_debug!(SMB, "no length check for command\n"); }
    }
    if *off > 4096 { ksmbd_debug!(SMB, "offset %d too large\n", *off); ret = -EINVAL; }
    else if (*off as u64 + *len as u64) > MAX_STREAM_PROT_LEN as u64 { ksmbd_debug!(SMB, "Request is larger than maximum stream protocol length(%u): %llu\n", MAX_STREAM_PROT_LEN, *off as u64 + *len as u64); ret = -EINVAL; }
    ret
}

unsafe fn smb2_calc_size(buf: *mut core::ffi::c_void, len: *mut u32) -> i32 {
    let pdu = buf as *mut smb2_pdu; let hdr = &mut (*pdu).hdr as *mut smb2_hdr;
    let mut offset = 0u32; let mut data_length = 0u32;
    *len = le16_to_cpu((*hdr).StructureSize) as u32 + le16_to_cpu((*pdu).StructureSize2) as u32;
    if (*hdr).Command == SMB2_LOCK { *len -= size_of::<smb2_lock_element>() as u32; }
    if !HAS_SMB2_DATA_AREA[le16_to_cpu((*hdr).Command) as usize] { ksmbd_debug!(SMB, "SMB2 len %u\n", *len); return 0; }
    let ret = smb2_get_data_area_len(&mut offset, &mut data_length, hdr); if ret != 0 { return ret; }
    if data_length > 0 { if offset + 1 < *len { ksmbd_debug!(SMB, "data area offset %d overlaps SMB2 header %u\n", offset + 1, *len); return -EINVAL; } *len = offset + data_length; }
    ksmbd_debug!(SMB, "SMB2 len %u\n", *len); 0
}

unsafe fn smb2_query_info_req_len(h: *mut smb2_query_info_req) -> i32 { le32_to_cpu((*h).InputBufferLength) as i32 }
unsafe fn smb2_query_info_resp_len(h: *mut smb2_query_info_req) -> i32 { le32_to_cpu((*h).OutputBufferLength) as i32 }
unsafe fn smb2_set_info_req_len(h: *mut smb2_set_info_req) -> i32 { le32_to_cpu((*h).BufferLength) as i32 }
unsafe fn smb2_read_req_len(h: *mut smb2_read_req) -> i32 { le32_to_cpu((*h).Length) as i32 }
unsafe fn smb2_write_req_len(h: *mut smb2_write_req) -> i32 { le32_to_cpu((*h).Length) as i32 }
unsafe fn smb2_query_dir_req_len(h: *mut smb2_query_directory_req) -> i32 { le32_to_cpu((*h).OutputBufferLength) as i32 }
unsafe fn smb2_ioctl_req_len(h: *mut smb2_ioctl_req) -> i32 { (le32_to_cpu((*h).InputCount) + le32_to_cpu((*h).OutputCount)) as i32 }
unsafe fn smb2_ioctl_resp_len(h: *mut smb2_ioctl_req) -> i32 { (le32_to_cpu((*h).MaxInputResponse) + le32_to_cpu((*h).MaxOutputResponse)) as i32 }

unsafe fn smb2_validate_credit_charge(work: *mut ksmbd_work, hdr: *mut smb2_hdr) -> i32 {
    let conn = (*work).conn; let mut req_len = 0u32; let mut expect = 0u32;
    let mut charge = le16_to_cpu((*hdr).CreditCharge) as u32;
    match (*hdr).Command {
        SMB2_QUERY_INFO => { req_len = smb2_query_info_req_len(hdr as *mut _ ) as u32; expect = smb2_query_info_resp_len(hdr as *mut _) as u32; }
        SMB2_SET_INFO => req_len = smb2_set_info_req_len(hdr as *mut _) as u32,
        SMB2_READ => req_len = smb2_read_req_len(hdr as *mut _) as u32,
        SMB2_WRITE => req_len = smb2_write_req_len(hdr as *mut _) as u32,
        SMB2_QUERY_DIRECTORY => req_len = smb2_query_dir_req_len(hdr as *mut _) as u32,
        SMB2_IOCTL => { req_len = smb2_ioctl_req_len(hdr as *mut _) as u32; expect = smb2_ioctl_resp_len(hdr as *mut _) as u32; }
        SMB2_CANCEL => return 0,
        _ => req_len = 1,
    }
    if charge < 1 { charge = 1; }
    let max_len = if req_len > expect { req_len } else { expect };
    let needed = (max_len + SMB2_MAX_BUFFER_SIZE - 1) / SMB2_MAX_BUFFER_SIZE;
    if charge < needed || charge > (*(*conn).vals).max_credits as u32 { return 1; }
    spin_lock(&mut (*conn).credits_lock);
    let mut ret = 0;
    if charge > (*conn).total_credits { ret = 1; }
    if (*conn).outstanding_credits as u64 + charge as u64 > (*conn).total_credits as u64 { ret = 1; }
    else { (*conn).outstanding_credits += charge as _; (*work).credit_charge = charge as _; }
    spin_unlock(&mut (*conn).credits_lock); ret
}

unsafe fn smb2_check_sequence_number(work: *mut ksmbd_work, hdr: *mut smb2_hdr) -> i32 {
    let conn = (*work).conn; if (*hdr).Command == SMB2_CANCEL { return 0; }
    let mid = le64_to_cpu((*hdr).MessageId); let mut charge = le16_to_cpu((*hdr).CreditCharge) as u64;
    if (*(*conn).vals).req_capabilities & SMB2_GLOBAL_CAP_LARGE_MTU == 0 || charge == 0 { charge = 1; }
    if mid.wrapping_add(charge) < mid { return 1; }
    spin_lock(&mut (*conn).credits_lock); let mut ret = 0;
    if mid < (*conn).seq_low || mid + charge > (*conn).seq_high { ret = 1; }
    else { let mut i = mid; while i < mid + charge { if !test_bit(i & (KSMBD_CMD_SEQ_WINDOW - 1), (*conn).seq_bitmap) { ret = 1; break; } i += 1; } if ret == 0 { let mut j = mid; while j < mid + charge { __clear_bit(j & (KSMBD_CMD_SEQ_WINDOW - 1), (*conn).seq_bitmap); j += 1; } while (*conn).seq_low < (*conn).seq_high && !test_bit((*conn).seq_low & (KSMBD_CMD_SEQ_WINDOW - 1), (*conn).seq_bitmap) { (*conn).seq_low += 1; } } }
    spin_unlock(&mut (*conn).credits_lock); ret
}

#[no_mangle]
pub unsafe extern "C" fn ksmbd_smb2_check_message(work: *mut ksmbd_work) -> i32 {
    let pdu = ksmbd_req_buf_next(work); let hdr = &mut (*pdu).hdr as *mut smb2_hdr;
    let mut len = get_rfc1002_len((*work).request_buf); let next_cmd = le32_to_cpu((*hdr).NextCommand);
    if (*work).next_smb2_rcv_hdr_off as u64 + next_cmd as u64 > len as u64 { pr_err!("next command({}) offset exceeds smb msg size\n", next_cmd); return 1; }
    if next_cmd > 0 { len = next_cmd; } else if (*work).next_smb2_rcv_hdr_off != 0 { len -= (*work).next_smb2_rcv_hdr_off; }
    if check_smb2_hdr(hdr) != 0 || (*hdr).StructureSize != SMB2_HEADER_STRUCTURE_SIZE { return 1; }
    let command = le16_to_cpu((*hdr).Command) as usize; if command >= NUMBER_OF_SMB2_COMMANDS { return 1; }
    if len < (__SMB2_HEADER_STRUCTURE_SIZE + size_of::<__le16>()) as u32 { return 1; }
    if SMB2_REQ_STRUCT_SIZES[command] != (*pdu).StructureSize2 { return 1; }
    let mut calc = 0; if smb2_calc_size(hdr as *mut _, &mut calc) != 0 { return 1; }
    if len != calc && calc != len + 1 && ALIGN!(calc, 8) != len && !(calc < len && len - calc <= 8) && command != SMB2_NEGOTIATE_HE as usize { return 1; }
    if ((*(*work).conn).vals).req_capabilities & SMB2_GLOBAL_CAP_LARGE_MTU != 0 && smb2_validate_credit_charge(work, hdr) != 0 { return 1; }
    if smb2_check_sequence_number(work, hdr) != 0 { ksmbd_conn_set_exiting((*work).conn); return 1; } 0
}

#[no_mangle]
pub unsafe extern "C" fn smb2_negotiate_request(work: *mut ksmbd_work) -> i32 { ksmbd_smb_negotiate_common(work, SMB2_NEGOTIATE_HE) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
