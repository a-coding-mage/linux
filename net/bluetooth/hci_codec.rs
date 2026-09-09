// SPDX-License-Identifier: GPL-2.0

/* Copyright (C) 2021 Intel Corporation */

// Dependencies are supplied by the surrounding Bluetooth implementation.

unsafe fn hci_codec_list_add(
    list: *mut list_head,
    sent: *mut hci_op_read_local_codec_caps,
    rp: *mut hci_rp_read_local_codec_caps,
    caps: *const core::ffi::c_void,
    len: __u32,
) -> i32 {
    let entry = kzalloc(core::mem::size_of::<codec_list>() + len as usize, GFP_KERNEL);
    if entry.is_null() {
        return -ENOMEM;
    }

    (*entry).id = (*sent).id;
    if (*sent).id == 0xFF {
        (*entry).cid = __le16_to_cpu((*sent).cid);
        (*entry).vid = __le16_to_cpu((*sent).vid);
    }
    (*entry).transport = (*sent).transport;
    (*entry).len = len;
    (*entry).num_caps = 0;
    if !rp.is_null() {
        (*entry).num_caps = (*rp).num_caps;
        memcpy((*entry).caps.as_mut_ptr() as *mut core::ffi::c_void, caps, len as usize);
    }
    list_add(&mut (*entry).list, list);

    0
}

pub unsafe fn hci_codec_list_clear(codec_list: *mut list_head) {
    // Equivalent to list_for_each_entry_safe(c, n, codec_list, list).
    let mut c: *mut codec_list = core::ptr::null_mut();
    let mut n: *mut codec_list = core::ptr::null_mut();
    list_for_each_entry_safe!(c, n, codec_list, list, {
        list_del(&mut (*c).list);
        kfree(c);
    });
}

unsafe fn hci_read_codec_capabilities(
    hdev: *mut hci_dev,
    transport: __u8,
    cmd: *mut hci_op_read_local_codec_caps,
) {
    let mut i: __u8 = 0;

    while i < TRANSPORT_TYPE_MAX {
        if transport & BIT(i) != 0 {
            let mut rp: *mut hci_rp_read_local_codec_caps;
            let mut caps: *mut hci_codec_caps;
            let mut skb: *mut sk_buff;
            let mut j: __u8;
            let mut len: __u32;

            (*cmd).transport = i;

            /* If Read_Codec_Capabilities command is not supported
             * then just add codec to the list without caps
             */
            if (*hdev).commands[45] & 0x08 == 0 {
                hci_dev_lock(hdev);
                hci_codec_list_add(&mut (*hdev).local_codecs, cmd, core::ptr::null_mut(), core::ptr::null(), 0);
                hci_dev_unlock(hdev);
                i += 1;
                continue;
            }

            skb = __hci_cmd_sync_sk(hdev, HCI_OP_READ_LOCAL_CODEC_CAPS,
                core::mem::size_of::<hci_op_read_local_codec_caps>() as u16, cmd, 0,
                HCI_CMD_TIMEOUT, core::ptr::null_mut());
            if IS_ERR(skb) {
                bt_dev_err!(hdev, "Failed to read codec capabilities ({})", PTR_ERR(skb));
                i += 1;
                continue;
            }

            if (*skb).len < core::mem::size_of::<hci_rp_read_local_codec_caps>() { kfree_skb(skb); i += 1; continue; }
            rp = (*skb).data as *mut hci_rp_read_local_codec_caps;
            if (*rp).status != 0 { kfree_skb(skb); i += 1; continue; }
            if (*rp).num_caps == 0 { len = 0; goto skip_caps_parse; }

            skb_pull(skb, core::mem::size_of::<hci_rp_read_local_codec_caps>());
            j = 0; len = 0;
            while j < (*rp).num_caps {
                caps = (*skb).data as *mut hci_codec_caps;
                if (*skb).len < core::mem::size_of::<hci_codec_caps>() { kfree_skb(skb); i += 1; continue; }
                if (*skb).len < core::mem::size_of_val(&(*caps).len) + (*caps).len as usize { kfree_skb(skb); i += 1; continue; }
                len += (core::mem::size_of_val(&(*caps).len) + (*caps).len as usize) as __u32;
                skb_pull(skb, core::mem::size_of_val(&(*caps).len) + (*caps).len as usize);
                j += 1;
            }

        skip_caps_parse:
            hci_dev_lock(hdev);
            hci_codec_list_add(&mut (*hdev).local_codecs, cmd, rp,
                (rp as *mut __u8).add(core::mem::size_of::<hci_rp_read_local_codec_caps>() ) as *const _, len);
            hci_dev_unlock(hdev);
            kfree_skb(skb);
        }
        i += 1;
    }
}

pub unsafe fn hci_read_supported_codecs(hdev: *mut hci_dev) {
    let skb = __hci_cmd_sync_sk(hdev, HCI_OP_READ_LOCAL_CODECS, 0, core::ptr::null_mut(), 0, HCI_CMD_TIMEOUT, core::ptr::null_mut());
    if IS_ERR(skb) { bt_dev_err!(hdev, "Failed to read local supported codecs ({})", PTR_ERR(skb)); return; }
    if (*skb).len < core::mem::size_of::<hci_rp_read_local_supported_codecs>() { kfree_skb(skb); return; }
    let rp = (*skb).data as *mut hci_rp_read_local_supported_codecs;
    if (*rp).status != 0 { kfree_skb(skb); return; }
    skb_pull(skb, core::mem::size_of_val(&(*rp).status));
    let std_codecs = (*skb).data as *mut hci_std_codecs;
    if (*skb).len < flex_array_size(std_codecs, codec, (*std_codecs).num) + core::mem::size_of_val(&(*std_codecs).num) { kfree_skb(skb); return; }
    let mut caps: hci_op_read_local_codec_caps = core::mem::zeroed();
    let mut i: __u8 = 0;
    while i < (*std_codecs).num { caps.id = (*std_codecs).codec[i as usize]; caps.direction = 0; hci_read_codec_capabilities(hdev, LOCAL_CODEC_ACL_MASK | LOCAL_CODEC_SCO_MASK, &mut caps); i += 1; }
    skb_pull(skb, flex_array_size(std_codecs, codec, (*std_codecs).num) + core::mem::size_of_val(&(*std_codecs).num));
    let vnd_codecs = (*skb).data as *mut hci_vnd_codecs;
    if (*skb).len < flex_array_size(vnd_codecs, codec, (*vnd_codecs).num) + core::mem::size_of_val(&(*vnd_codecs).num) { kfree_skb(skb); return; }
    i = 0; while i < (*vnd_codecs).num { caps.id = 0xFF; caps.cid = (*vnd_codecs).codec[i as usize].cid; caps.vid = (*vnd_codecs).codec[i as usize].vid; caps.direction = 0; hci_read_codec_capabilities(hdev, LOCAL_CODEC_ACL_MASK | LOCAL_CODEC_SCO_MASK, &mut caps); i += 1; }
    kfree_skb(skb);
}

pub unsafe fn hci_read_supported_codecs_v2(hdev: *mut hci_dev) {
    let skb = __hci_cmd_sync_sk(hdev, HCI_OP_READ_LOCAL_CODECS_V2, 0, core::ptr::null_mut(), 0, HCI_CMD_TIMEOUT, core::ptr::null_mut());
    if IS_ERR(skb) { bt_dev_err!(hdev, "Failed to read local supported codecs ({})", PTR_ERR(skb)); return; }
    if (*skb).len < core::mem::size_of::<hci_rp_read_local_supported_codecs_v2>() { kfree_skb(skb); return; }
    let rp = (*skb).data as *mut hci_rp_read_local_supported_codecs_v2;
    if (*rp).status != 0 { kfree_skb(skb); return; }
    skb_pull(skb, core::mem::size_of_val(&(*rp).status));
    let std_codecs = (*skb).data as *mut hci_std_codecs_v2;
    if (*skb).len < flex_array_size(std_codecs, codec, (*std_codecs).num) + core::mem::size_of_val(&(*std_codecs).num) { kfree_skb(skb); return; }
    let mut caps: hci_op_read_local_codec_caps = core::mem::zeroed(); let mut i: __u8 = 0;
    while i < (*std_codecs).num { caps.id = (*std_codecs).codec[i as usize].id; hci_read_codec_capabilities(hdev, (*std_codecs).codec[i as usize].transport, &mut caps); i += 1; }
    skb_pull(skb, flex_array_size(std_codecs, codec, (*std_codecs).num) + core::mem::size_of_val(&(*std_codecs).num));
    let vnd_codecs = (*skb).data as *mut hci_vnd_codecs_v2;
    if (*skb).len < flex_array_size(vnd_codecs, codec, (*vnd_codecs).num) + core::mem::size_of_val(&(*vnd_codecs).num) { kfree_skb(skb); return; }
    i = 0; while i < (*vnd_codecs).num { caps.id = 0xFF; caps.cid = (*vnd_codecs).codec[i as usize].cid; caps.vid = (*vnd_codecs).codec[i as usize].vid; hci_read_codec_capabilities(hdev, (*vnd_codecs).codec[i as usize].transport, &mut caps); i += 1; }
    kfree_skb(skb);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
