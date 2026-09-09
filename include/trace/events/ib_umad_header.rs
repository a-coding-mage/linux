/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2018 Intel Corporation. All rights reserved. */

//! Rust translation of the `ib_umad` tracepoint header.
//!
//! The types used by this header (`ib_umad_file`, `ib_user_mad_hdr`, and
//! `ib_mad_hdr`) are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct IbUmadTemplateEntry {
    pub port_num: u8,
    pub sl: u8,
    pub path_bits: u8,
    pub grh_present: u8,
    pub id: u32,
    pub status: u32,
    pub timeout_ms: u32,
    pub retires: u32,
    pub length: u32,
    pub qpn: u32,
    pub qkey: u32,
    pub gid_index: u8,
    pub hop_limit: u8,
    pub lid: u16,
    pub attr_id: u16,
    pub pkey_index: u16,
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub flow_label: u32,
    pub mad_status: u16,
    pub class_specific: u16,
    pub attr_mod: u32,
    pub tid: u64,
    pub gid: [u8; 16],
    pub dev_index: u32,
    pub traffic_class: u8,
}

/// Translation of the `TP_fast_assign` body for `ib_umad_template`.
pub unsafe fn ib_umad_template_fast_assign(
    entry: *mut IbUmadTemplateEntry,
    file: *const ib_umad_file,
    umad_hdr: *const ib_user_mad_hdr,
    mad_hdr: *const ib_mad_hdr,
) {
    (*entry).dev_index = (*(*file).port).ib_dev.index;
    (*entry).port_num = (*(*file).port).port_num;

    (*entry).id = (*umad_hdr).id;
    (*entry).status = (*umad_hdr).status;
    (*entry).timeout_ms = (*umad_hdr).timeout_ms;
    (*entry).retires = (*umad_hdr).retries;
    (*entry).length = (*umad_hdr).length;
    (*entry).qpn = (*umad_hdr).qpn;
    (*entry).qkey = (*umad_hdr).qkey;
    (*entry).lid = (*umad_hdr).lid;
    (*entry).sl = (*umad_hdr).sl;
    (*entry).path_bits = (*umad_hdr).path_bits;
    (*entry).grh_present = (*umad_hdr).grh_present;
    (*entry).gid_index = (*umad_hdr).gid_index;
    (*entry).hop_limit = (*umad_hdr).hop_limit;
    (*entry).traffic_class = (*umad_hdr).traffic_class;
    core::ptr::copy_nonoverlapping((*umad_hdr).gid.as_ptr(), (*entry).gid.as_mut_ptr(), 16);
    (*entry).flow_label = (*umad_hdr).flow_label;
    (*entry).pkey_index = (*umad_hdr).pkey_index;

    (*entry).base_version = (*mad_hdr).base_version;
    (*entry).mgmt_class = (*mad_hdr).mgmt_class;
    (*entry).class_version = (*mad_hdr).class_version;
    (*entry).method = (*mad_hdr).method;
    (*entry).mad_status = (*mad_hdr).status;
    (*entry).class_specific = (*mad_hdr).class_specific;
    (*entry).tid = (*mad_hdr).tid;
    (*entry).attr_id = (*mad_hdr).attr_id;
    (*entry).attr_mod = (*mad_hdr).attr_mod;
}

// TP_printk format and argument ordering are retained here as the trace
// formatter supplied by the surrounding tracing implementation.
pub const IB_UMAD_TEMPLATE_PRINTK: &str = "%d:%d umad_hdr: id 0x%08x status 0x%08x ms %u ret %u len %u QP%u qkey 0x%08x lid 0x%04x sl %u path_bits 0x%x grh 0x%x gidi %u hop_lim %u traf_cl %u gid %pI6c flow 0x%08x pkeyi %u MAD: base_ver 0x%x class 0x%x class_ver 0x%x method 0x%x status 0x%04x class_specific 0x%04x tid 0x%016llx attr_id 0x%04x attr_mod 0x%08x ";

pub type IbUmadWrite = IbUmadTemplateEntry;
pub type IbUmadReadRecv = IbUmadTemplateEntry;
pub type IbUmadReadSend = IbUmadTemplateEntry;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
