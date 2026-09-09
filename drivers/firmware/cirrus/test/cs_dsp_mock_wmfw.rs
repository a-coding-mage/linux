// SPDX-License-Identifier: GPL-2.0-only
//
// wmfw file builder for cs_dsp KUnit tests.
//
// Copyright (C) 2024 Cirrus Logic, Inc. and
//                    Cirrus Logic International Semiconductor Ltd.

/* Kernel headers and symbols are supplied by the surrounding kernel tree. */

const CS_DSP_MOCK_WMFW_BUF_SIZE: usize = 131072;

#[repr(C)]
pub struct cs_dsp_mock_wmfw_builder {
    pub test_priv: *mut cs_dsp_test,
    pub format_version: i32,
    pub buf: *mut core::ffi::c_void,
    pub buf_size_bytes: usize,
    pub write_p: *mut core::ffi::c_void,
    pub bytes_used: usize,
    pub alg_data_header: *mut core::ffi::c_void,
    pub num_coeffs: u32,
}

#[repr(C, packed)]
struct wmfw_adsp2_halo_header {
    header: wmfw_header,
    sizes: wmfw_adsp2_sizes,
    footer: wmfw_footer,
}

#[repr(C, packed)]
struct wmfw_long_string {
    len: u16,
    data: [u8; 0],
}

#[repr(C, packed)]
struct wmfw_short_string {
    len: u8,
    data: [u8; 0],
}

extern "C" {
    fn cs_dsp_mock_size_of_region(dsp: *const cs_dsp, region: i32) -> u32;
}

unsafe fn round_up_4(v: usize) -> usize { (v + 3) & !3 }
unsafe fn c_strlen(p: *const core::ffi::c_char) -> usize {
    let mut n = 0;
    while *p.add(n) != 0 { n += 1; }
    n
}
unsafe fn put_u16(p: *mut u8, v: u16) { core::ptr::write_unaligned(p as *mut u16, v.to_le()); }
unsafe fn put_u32(p: *mut u8, v: u32) { core::ptr::write_unaligned(p as *mut u32, v.to_le()); }

pub unsafe extern "C" fn cs_dsp_mock_wmfw_format_version(
    builder: *mut cs_dsp_mock_wmfw_builder,
) -> i32 { (*builder).format_version }

pub unsafe extern "C" fn cs_dsp_mock_wmfw_get_firmware(
    builder: *mut cs_dsp_mock_wmfw_builder,
) -> *mut firmware {
    if builder.is_null() { return core::ptr::null_mut(); }
    let fw = kunit_kzalloc((*(*builder).test_priv).test, core::mem::size_of::<firmware>(), GFP_KERNEL) as *mut firmware;
    kunit_assert_not_err_or_null((*(*builder).test_priv).test, fw);
    (*fw).data = (*builder).buf as *const u8;
    (*fw).size = (*builder).bytes_used;
    fw
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_add_raw_block(
    builder: *mut cs_dsp_mock_wmfw_builder, block_type: i32, offset: u32,
    payload_data: *const core::ffi::c_void, payload_len_bytes: usize,
) {
    let header = (*builder).write_p as *mut wmfw_region;
    let bytes_needed = 8 + payload_len_bytes;
    kunit_assert_true((*(*builder).test_priv).test,
        (*builder).write_p.add(bytes_needed) < (*builder).buf.add(CS_DSP_MOCK_WMFW_BUF_SIZE));
    put_u32(header as *mut u8, offset | ((block_type as u32) << 24));
    put_u32((header as *mut u8).add(4), payload_len_bytes as u32);
    if payload_len_bytes != 0 { core::ptr::copy_nonoverlapping(payload_data as *const u8, (header as *mut u8).add(8), payload_len_bytes); }
    (*builder).write_p = (*builder).write_p.add(bytes_needed);
    (*builder).bytes_used += bytes_needed;
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_add_info(builder: *mut cs_dsp_mock_wmfw_builder, info: *const core::ffi::c_char) {
    let mut len = c_strlen(info); let mut tmp = core::ptr::null_mut();
    if len % 4 != 0 {
        let copy_len = len; len = round_up_4(len);
        tmp = kunit_kzalloc((*(*builder).test_priv).test, len, GFP_KERNEL) as *mut core::ffi::c_char;
        kunit_assert_not_err_or_null((*(*builder).test_priv).test, tmp);
        core::ptr::copy_nonoverlapping(info as *const u8, tmp as *mut u8, copy_len); info = tmp;
    }
    cs_dsp_mock_wmfw_add_raw_block(builder, WMFW_INFO_TEXT, 0, info as *const _, len);
    kunit_kfree((*(*builder).test_priv).test, tmp as *mut _);
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_add_data_block(builder: *mut cs_dsp_mock_wmfw_builder, mem_region: i32, mem_offset_dsp_words: u32, payload_data: *const core::ffi::c_void, payload_len_bytes: usize) {
    kunit_assert_eq((*(*builder).test_priv).test, payload_len_bytes % 4, 0);
    cs_dsp_mock_wmfw_add_raw_block(builder, mem_region, mem_offset_dsp_words, payload_data, payload_len_bytes);
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_start_alg_info_block(builder: *mut cs_dsp_mock_wmfw_builder, alg_id: u32, name: *const core::ffi::c_char, description: *const core::ffi::c_char) {
    let rgn = (*builder).write_p as *mut wmfw_region;
    let mut bytes_needed = 8usize;
    (*builder).alg_data_header = (*builder).write_p; (*builder).num_coeffs = 0;
    kunit_assert_le((*(*builder).test_priv).test, alg_id, 0xffffff);
    match (*builder).format_version {
        0 => { kunit_fail((*(*builder).test_priv).test, b"wmfwV0 does not have alg blocks\n\0".as_ptr() as _); return; },
        1 => {
            bytes_needed += core::mem::size_of::<wmfw_adsp_alg_data>();
            kunit_assert_true((*(*builder).test_priv).test, (*builder).write_p.add(bytes_needed) < (*builder).buf.add(CS_DSP_MOCK_WMFW_BUF_SIZE));
            core::ptr::write_bytes((*builder).write_p as *mut u8, 0, bytes_needed);
            put_u32(rgn as *mut u8, WMFW_ALGORITHM_DATA << 24); put_u32((rgn as *mut u8).add(8), alg_id);
            if !name.is_null() { strscpy((rgn as *mut u8).add(12) as _, name, 64); }
            if !description.is_null() { strscpy((rgn as *mut u8).add(76) as _, description, 256); }
        },
        _ => {
            let nl = if name.is_null() { 0 } else { c_strlen(name) }; let dl = if description.is_null() { 0 } else { c_strlen(description) };
            bytes_needed += 4 + round_up_4(nl + 1) + round_up_4(dl + 2) + 4;
            kunit_assert_true((*(*builder).test_priv).test, (*builder).write_p.add(bytes_needed) < (*builder).buf.add(CS_DSP_MOCK_WMFW_BUF_SIZE));
            core::ptr::write_bytes((*builder).write_p as *mut u8, 0, bytes_needed); put_u32(rgn as *mut u8, WMFW_ALGORITHM_DATA << 24); put_u32((rgn as *mut u8).add(8), alg_id);
            let p = (rgn as *mut u8).add(12); *p = nl as u8; if nl != 0 { core::ptr::copy_nonoverlapping(name as *const u8, p.add(1), nl); }
            let q = p.add(round_up_4(nl + 1)); put_u16(q, dl as u16); if dl != 0 { core::ptr::copy_nonoverlapping(description as *const u8, q.add(2), dl); }
        }
    }
    (*builder).write_p = (*builder).write_p.add(bytes_needed); (*builder).bytes_used += bytes_needed;
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_add_coeff_desc(builder: *mut cs_dsp_mock_wmfw_builder, def: *const cs_dsp_mock_coeff_def) {
    kunit_assert_not_null((*(*builder).test_priv).test, (*builder).alg_data_header);
    if (*builder).format_version == 0 { return; }
    let (sn, fn_, dn) = (c_strlen((*def).shortname), if (*def).fullname.is_null() { 0 } else { c_strlen((*def).fullname) }, if (*def).description.is_null() { 0 } else { c_strlen((*def).description) });
    let bytes_needed = if (*builder).format_version == 1 { core::mem::size_of::<wmfw_adsp_coeff_data>() } else { 8 + round_up_4(sn+1) + round_up_4(fn_+1) + round_up_4(dn+2) + 8 };
    kunit_assert_true((*(*builder).test_priv).test, (*builder).write_p.add(bytes_needed) < (*builder).buf.add(CS_DSP_MOCK_WMFW_BUF_SIZE));
    core::ptr::write_bytes((*builder).write_p as *mut u8, 0, bytes_needed);
    let p = (*builder).write_p as *mut u8;
    if (*builder).format_version == 1 { put_u16(p, (*def).offset_dsp_words as u16); put_u16(p.add(2), (*def).mem_type as u16); put_u32(p.add(4), (bytes_needed-4) as u32); put_u16(p.add(8), (*def).type_ as u16); put_u16(p.add(10), (*def).flags as u16); put_u32(p.add(12), (*def).length_bytes); }
    else { put_u32(p, (*def).offset_dsp_words | ((*def).mem_type as u32 << 16)); put_u32(p.add(4), (bytes_needed-8) as u32); let mut q=p.add(8); *q=sn as u8; core::ptr::copy_nonoverlapping((*def).shortname as *const u8,q.add(1),sn); q=q.add(round_up_4(sn+1)); *q=fn_ as u8; if fn_!=0 {core::ptr::copy_nonoverlapping((*def).fullname as *const u8,q.add(1),fn_)} q=q.add(round_up_4(fn_+1)); put_u16(q,dn as u16); if dn!=0 {core::ptr::copy_nonoverlapping((*def).description as *const u8,q.add(2),dn)} q=q.add(round_up_4(dn+2)); put_u32(q,(*def).type_ | ((*def).flags as u32<<16)); put_u32(q.add(4),(*def).length_bytes); }
    (*builder).write_p=(*builder).write_p.add(bytes_needed); (*builder).bytes_used+=bytes_needed; (*builder).num_coeffs+=1;
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_end_alg_info_block(builder: *mut cs_dsp_mock_wmfw_builder) {
    let rgn=(*builder).alg_data_header as *mut u8; kunit_assert_not_null((*(*builder).test_priv).test,rgn);
    put_u32(rgn.add(4), ((*builder).write_p as usize - rgn.add(8) as usize) as u32);
    if (*builder).format_version==1 { put_u32(rgn.add(8+core::mem::size_of::<wmfw_adsp_alg_data>()-4),(*builder).num_coeffs); }
    else if (*builder).format_version!=0 { let mut o=12usize; let nl=*rgn.add(o) as usize; o+=round_up_4(nl+1); let dl=u16::from_le(core::ptr::read_unaligned(rgn.add(o) as *const u16)) as usize; o+=round_up_4(dl+2); put_u32(rgn.add(o),(*builder).num_coeffs); }
    (*builder).alg_data_header=core::ptr::null_mut();
}

pub unsafe extern "C" fn cs_dsp_mock_wmfw_init(priv_: *mut cs_dsp_test, mut format_version: i32) -> *mut cs_dsp_mock_wmfw_builder {
    kunit_assert_le((*priv_).test,format_version,0xff); if format_version<0 { format_version=if (*(*priv_).dsp).type_==WMFW_ADSP2 {2}else{3}; }
    let b=kunit_kzalloc((*priv_).test,core::mem::size_of::<cs_dsp_mock_wmfw_builder>(),GFP_KERNEL) as *mut cs_dsp_mock_wmfw_builder; kunit_assert_not_err_or_null((*priv_).test,b);
    (*b).test_priv=priv_; (*b).format_version=format_version; (*b).buf=vmalloc(CS_DSP_MOCK_WMFW_BUF_SIZE); kunit_assert_not_null((*priv_).test,(*b).buf); (*b).buf_size_bytes=CS_DSP_MOCK_WMFW_BUF_SIZE; (*b).write_p=(*b).buf; b
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
