// SPDX-License-Identifier: MIT
/*
 * Copyright 2025 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Kernel and AMDGPU declarations are supplied by the surrounding translation unit.

static MCE: guid_t = CPER_NOTIFY_MCE;
static CMC: guid_t = CPER_NOTIFY_CMC;
static BOOT: guid_t = BOOT_TYPE;
static CRASHDUMP: guid_t = AMD_CRASHDUMP;
static RUNTIME: guid_t = AMD_GPU_NONSTANDARD_ERROR;

const CPER_SIGNATURE_SZ: usize = core::mem::size_of::<cper_hdr>(); // sizeof(((struct cper_hdr *)0)->signature)

unsafe fn __inc_entry_length(hdr: *mut cper_hdr, size: u32) {
    (*hdr).record_length = (*hdr).record_length.wrapping_add(size);
}

unsafe fn amdgpu_cper_get_timestamp(timestamp: *mut cper_timestamp) {
    let mut tm: tm = core::mem::zeroed();
    let now: time64_t = ktime_get_real_seconds();
    time64_to_tm(now, 0, &mut tm);
    (*timestamp).seconds = tm.tm_sec;
    (*timestamp).minutes = tm.tm_min;
    (*timestamp).hours = tm.tm_hour;
    (*timestamp).flag = 0;
    (*timestamp).day = tm.tm_mday;
    (*timestamp).month = 1 + tm.tm_mon;
    (*timestamp).year = (1900 + tm.tm_year) % 100;
    (*timestamp).century = (1900 + tm.tm_year) / 100;
}

pub unsafe fn amdgpu_cper_entry_fill_hdr(adev: *mut amdgpu_device, hdr: *mut cper_hdr, ty: amdgpu_cper_type, sev: cper_error_severity) {
    let mut record_id = [0i8; 16];
    (*hdr).signature[0] = b'C' as i8; (*hdr).signature[1] = b'P' as i8;
    (*hdr).signature[2] = b'E' as i8; (*hdr).signature[3] = b'R' as i8;
    (*hdr).revision = CPER_HDR_REV_1; (*hdr).signature_end = 0xffff_ffff;
    (*hdr).error_severity = sev; (*hdr).valid_bits.platform_id = 1;
    (*hdr).valid_bits.timestamp = 1;
    amdgpu_cper_get_timestamp(&mut (*hdr).timestamp);
    snprintf(record_id.as_mut_ptr(), 9, c"%d:%X".as_ptr(),
        if !(*adev).smuio.funcs.is_null() && !(*(*adev).smuio.funcs).get_socket_id.is_none() { ((*(*adev).smuio.funcs).get_socket_id.unwrap())(adev) } else { 0 },
        atomic_inc_return(&mut (*adev).cper.unique_id));
    memcpy((*hdr).record_id.as_mut_ptr() as *mut _, record_id.as_ptr() as *const _, 8);
    snprintf((*hdr).platform_id.as_mut_ptr(), 16, c"0x%04X:0x%04X".as_ptr(), (*(*adev).pdev).vendor, (*(*adev).pdev).device);
    snprintf((*hdr).creator_id.as_mut_ptr(), 16, c"%s".as_ptr(), CPER_CREATOR_ID_AMDGPU);
    match ty {
        AMDGPU_CPER_TYPE_BOOT => (*hdr).notify_type = BOOT,
        AMDGPU_CPER_TYPE_FATAL | AMDGPU_CPER_TYPE_BP_THRESHOLD => (*hdr).notify_type = MCE,
        AMDGPU_CPER_TYPE_RUNTIME => (*hdr).notify_type = if sev == CPER_SEV_NON_FATAL_CORRECTED { CMC } else { MCE },
        _ => dev_err((*adev).dev, c"Unknown CPER Type\n".as_ptr()),
    }
    __inc_entry_length(hdr, HDR_LEN);
}

unsafe fn amdgpu_cper_entry_fill_section_desc(adev: *mut amdgpu_device, d: *mut cper_sec_desc, bp: bool, poison: bool, sev: cper_error_severity, sec_type: guid_t, len: u32, off: u32) -> i32 {
    (*d).revision_minor = CPER_SEC_MINOR_REV_1; (*d).revision_major = CPER_SEC_MAJOR_REV_22;
    (*d).sec_offset = off; (*d).sec_length = len; (*d).valid_bits.fru_text = 1;
    (*d).flag_bits.primary = 1; (*d).severity = sev; (*d).sec_type = sec_type;
    snprintf((*d).fru_text.as_mut_ptr(), 20, c"OAM%d".as_ptr(), if !(*adev).smuio.funcs.is_null() && !(*(*adev).smuio.funcs).get_socket_id.is_none() { ((*(*adev).smuio.funcs).get_socket_id.unwrap())(adev) } else { 0 });
    if bp { (*d).flag_bits.exceed_err_threshold = 1; }
    if poison { (*d).flag_bits.latent_err = 1; }
    0
}

pub unsafe fn amdgpu_cper_entry_fill_fatal_section(adev: *mut amdgpu_device, hdr: *mut cper_hdr, idx: u32, reg_data: cper_sec_crashdump_reg_data) -> i32 {
    let d = (hdr as *mut u8).add(SEC_DESC_OFFSET(idx) as usize) as *mut cper_sec_desc;
    let s = (hdr as *mut u8).add(FATAL_SEC_OFFSET((*hdr).sec_cnt, idx) as usize) as *mut cper_sec_crashdump_fatal;
    amdgpu_cper_entry_fill_section_desc(adev, d, false, false, CPER_SEV_FATAL_UNCORRECTED, CRASHDUMP, FATAL_SEC_LEN, FATAL_SEC_OFFSET((*hdr).sec_cnt, idx));
    (*s).body.reg_ctx_type = CPER_CTX_TYPE_CRASH; (*s).body.reg_arr_size = core::mem::size_of_val(&reg_data) as _; (*s).body.data = reg_data;
    __inc_entry_length(hdr, SEC_DESC_LEN + FATAL_SEC_LEN); 0
}

pub unsafe fn amdgpu_cper_entry_fill_runtime_section(adev: *mut amdgpu_device, hdr: *mut cper_hdr, idx: u32, sev: cper_error_severity, reg_dump: *const u32, mut reg_count: u32) -> i32 {
    let poison = sev != CPER_SEV_NON_FATAL_CORRECTED;
    let d = (hdr as *mut u8).add(SEC_DESC_OFFSET(idx) as usize) as *mut cper_sec_desc;
    let s = (hdr as *mut u8).add(NONSTD_SEC_OFFSET((*hdr).sec_cnt, idx) as usize) as *mut cper_sec_nonstd_err;
    amdgpu_cper_entry_fill_section_desc(adev, d, false, poison, sev, RUNTIME, NONSTD_SEC_LEN, NONSTD_SEC_OFFSET((*hdr).sec_cnt, idx));
    reg_count = umin(reg_count, CPER_ACA_REG_COUNT); (*s).hdr.valid_bits.err_info_cnt = 1; (*s).hdr.valid_bits.err_context_cnt = 1;
    (*s).info.error_type = RUNTIME; (*s).info.ms_chk_bits.err_type_valid = 1; (*s).ctx.reg_ctx_type = CPER_CTX_TYPE_CRASH; (*s).ctx.reg_arr_size = core::mem::size_of_val(&(*s).ctx.reg_dump) as _;
    memcpy((*s).ctx.reg_dump.as_mut_ptr() as *mut _, reg_dump as *const _, (reg_count * core::mem::size_of::<u32>() as u32) as usize);
    __inc_entry_length(hdr, SEC_DESC_LEN + NONSTD_SEC_LEN); 0
}

pub unsafe fn amdgpu_cper_entry_fill_bad_page_threshold_section(adev: *mut amdgpu_device, hdr: *mut cper_hdr, idx: u32) -> i32 {
    let d = (hdr as *mut u8).add(SEC_DESC_OFFSET(idx) as usize) as *mut cper_sec_desc;
    let s = (hdr as *mut u8).add(NONSTD_SEC_OFFSET((*hdr).sec_cnt, idx) as usize) as *mut cper_sec_nonstd_err;
    amdgpu_cper_entry_fill_section_desc(adev, d, true, false, CPER_SEV_FATAL_UNCORRECTED, RUNTIME, NONSTD_SEC_LEN, NONSTD_SEC_OFFSET((*hdr).sec_cnt, idx));
    (*s).hdr.valid_bits.err_info_cnt = 1; (*s).hdr.valid_bits.err_context_cnt = 1; (*s).info.error_type = RUNTIME;
    (*s).info.valid_bits.ms_chk = 1; (*s).info.ms_chk_bits.err_type_valid = 1; (*s).info.ms_chk_bits.err_type = 1; (*s).info.ms_chk_bits.pcc = 1;
    (*s).ctx.reg_ctx_type = CPER_CTX_TYPE_CRASH; (*s).ctx.reg_arr_size = core::mem::size_of_val(&(*s).ctx.reg_dump) as _;
    let socket_id = if !(*adev).smuio.funcs.is_null() && !(*(*adev).smuio.funcs).get_socket_id.is_none() { ((*(*adev).smuio.funcs).get_socket_id.unwrap())(adev) } else { 0 };
    (*s).ctx.reg_dump[CPER_ACA_REG_CTL_LO] = 0x1; (*s).ctx.reg_dump[CPER_ACA_REG_CTL_HI] = 0;
    (*s).ctx.reg_dump[CPER_ACA_REG_STATUS_LO] = 0x137; (*s).ctx.reg_dump[CPER_ACA_REG_STATUS_HI] = 0xB0000000;
    (*s).ctx.reg_dump[CPER_ACA_REG_ADDR_LO] = 0; (*s).ctx.reg_dump[CPER_ACA_REG_ADDR_HI] = 0;
    (*s).ctx.reg_dump[CPER_ACA_REG_MISC0_LO] = 0; (*s).ctx.reg_dump[CPER_ACA_REG_MISC0_HI] = 0;
    (*s).ctx.reg_dump[CPER_ACA_REG_CONFIG_LO] = 2; (*s).ctx.reg_dump[CPER_ACA_REG_CONFIG_HI] = 0x1ff;
    (*s).ctx.reg_dump[CPER_ACA_REG_IPID_LO] = (socket_id / 4) & 1; (*s).ctx.reg_dump[CPER_ACA_REG_IPID_HI] = 0x096 | (((socket_id % 4) & 3) << 12);
    (*s).ctx.reg_dump[CPER_ACA_REG_SYND_LO] = 0; (*s).ctx.reg_dump[CPER_ACA_REG_SYND_HI] = 0;
    __inc_entry_length(hdr, SEC_DESC_LEN + NONSTD_SEC_LEN); 0
}

pub unsafe fn amdgpu_cper_alloc_entry(adev: *mut amdgpu_device, ty: amdgpu_cper_type, section_count: u16) -> *mut cper_hdr {
    let mut size = HDR_LEN + SEC_DESC_LEN * section_count as u32;
    size += match ty { AMDGPU_CPER_TYPE_RUNTIME | AMDGPU_CPER_TYPE_BP_THRESHOLD => NONSTD_SEC_LEN * section_count as u32, AMDGPU_CPER_TYPE_FATAL => FATAL_SEC_LEN * section_count as u32, AMDGPU_CPER_TYPE_BOOT => BOOT_SEC_LEN * section_count as u32, _ => { dev_err((*adev).dev, c"Unknown CPER Type!\n".as_ptr()); return core::ptr::null_mut(); } };
    let hdr = kzalloc(size, GFP_KERNEL); if hdr.is_null() { return hdr; } (*hdr).sec_cnt = section_count; hdr
}

pub unsafe fn amdgpu_cper_generate_bp_threshold_record(adev: *mut amdgpu_device) -> i32 {
    let ring = &mut (*adev).cper.ring_buf; let hdr = amdgpu_cper_alloc_entry(adev, AMDGPU_CPER_TYPE_BP_THRESHOLD, 1);
    if hdr.is_null() { dev_err((*adev).dev, c"fail to alloc cper entry for bad page threshold record\n".as_ptr()); return -ENOMEM; }
    amdgpu_cper_entry_fill_hdr(adev, hdr, AMDGPU_CPER_TYPE_BP_THRESHOLD, CPER_SEV_FATAL_UNCORRECTED);
    let ret = amdgpu_cper_entry_fill_bad_page_threshold_section(adev, hdr, 0); if ret != 0 { return ret; }
    amdgpu_cper_ring_write(ring, hdr as *mut _, (*hdr).record_length as i32); kfree(hdr); 0
}

unsafe fn amdgpu_cper_is_hdr(ring: *mut amdgpu_ring, pos: u64) -> bool {
    let mut signature = [0i8; CPER_SIGNATURE_SZ]; let byte_pos = pos << 2;
    if byte_pos >= (*ring).ring_size as u64 { return false; }
    if byte_pos + CPER_SIGNATURE_SZ as u64 <= (*ring).ring_size as u64 { memcpy(signature.as_mut_ptr() as *mut _, (*ring).ring.add(pos as usize) as *const _, CPER_SIGNATURE_SZ); }
    else { let chunk = (*ring).ring_size as u64 - byte_pos; memcpy(signature.as_mut_ptr() as *mut _, (*ring).ring.add(pos as usize) as *const _, chunk as usize); memcpy(signature.as_mut_ptr().add(chunk as usize) as *mut _, (*ring).ring as *const _, CPER_SIGNATURE_SZ - chunk as usize); }
    !memcmp(signature.as_ptr() as *const _, c"CPER".as_ptr() as *const _, CPER_SIGNATURE_SZ)
}

unsafe fn amdgpu_cper_ring_get_ent_sz(ring: *mut amdgpu_ring, pos: u64) -> u32 {
    let mut chdr: cper_hdr = core::mem::zeroed(); let mut rec_len = 0; let chunk = (*ring).ring_size - ((pos << 2) as u32);
    if amdgpu_cper_is_hdr(ring, pos) { if chunk as usize >= core::mem::size_of::<cper_hdr>() { memcpy(&mut chdr as *mut _ as *mut _, (*ring).ring.add(pos as usize) as *const _, core::mem::size_of::<cper_hdr>()); } else { memcpy(&mut chdr as *mut _ as *mut _, (*ring).ring.add(pos as usize) as *const _, chunk as usize); memcpy((&mut chdr as *mut _ as *mut u8).add(chunk as usize) as *mut _, (*ring).ring as *const _, core::mem::size_of::<cper_hdr>() - chunk as usize); } rec_len = chdr.record_length; }
    else if (*ring).count_dw == 0 { let mut p = pos + 1; while p <= (*ring).buf_mask { if amdgpu_cper_is_hdr(ring, p) { rec_len = ((p - pos) << 2) as u32; break; } p += 1; } }
    if rec_len == 0 { chunk } else { umin(rec_len, chunk) }
}

pub unsafe fn amdgpu_cper_ring_write(ring: *mut amdgpu_ring, src: *mut core::ffi::c_void, mut count: i32) {
    let rec_cnt_dw = count >> 2; let mut s = src as *mut u8; if count >= (*ring).ring_size as i32 - 4 { dev_err((*(*ring).adev).dev, c"CPER data size(%d) is larger than ring size(%d)\n".as_ptr(), count, (*ring).ring_size - 4); return; }
    mutex_lock(&mut (*(*ring).adev).cper.ring_lock); let wptr_old = (*ring).wptr; let mut rptr = *(*ring).rptr_cpu_addr & (*ring).ptr_mask;
    while count != 0 { let ent_sz = amdgpu_cper_ring_get_ent_sz(ring, (*ring).wptr); let chunk = umin(ent_sz, count as u32); memcpy((*ring).ring.add((*ring).wptr as usize) as *mut _, s as *const _, chunk as usize); (*ring).wptr = ((*ring).wptr + (chunk >> 2)) & (*ring).ptr_mask; count -= chunk as i32; s = s.add(chunk as usize); }
    if (*ring).count_dw < rec_cnt_dw as u32 { (*ring).count_dw = 0; }
    if ((wptr_old < rptr && rptr <= (*ring).wptr) || ((*ring).wptr < wptr_old && wptr_old < rptr) || (rptr <= (*ring).wptr && (*ring).wptr < wptr_old)) {
        let mut pos = ((*ring).wptr + 1) & (*ring).ptr_mask;
        loop { let ent_sz = amdgpu_cper_ring_get_ent_sz(ring, pos); let next_rptr = if ent_sz >= core::mem::size_of::<u32>() as u32 { (rptr + (ent_sz >> 2)) & (*ring).ptr_mask } else { rptr }; if next_rptr == rptr { rptr = (*ring).wptr; *(*ring).rptr_cpu_addr = rptr; (*ring).count_dw = ((*ring).ring_size - 4) >> 2; break; } rptr = next_rptr; *(*ring).rptr_cpu_addr = rptr; pos = rptr; if amdgpu_cper_is_hdr(ring, rptr) { break; } }
    }
    if (*ring).count_dw >= rec_cnt_dw as u32 { (*ring).count_dw -= rec_cnt_dw as u32; } mutex_unlock(&mut (*(*ring).adev).cper.ring_lock);
}

unsafe fn amdgpu_cper_ring_get_rptr(ring: *mut amdgpu_ring) -> u64 { *(*ring).rptr_cpu_addr }
unsafe fn amdgpu_cper_ring_get_wptr(ring: *mut amdgpu_ring) -> u64 { (*ring).wptr }

static cper_ring_funcs: amdgpu_ring_funcs = amdgpu_ring_funcs { ty: AMDGPU_RING_TYPE_CPER, align_mask: 0xff, support_64bit_ptrs: false, get_rptr: Some(amdgpu_cper_ring_get_rptr), get_wptr: Some(amdgpu_cper_ring_get_wptr) };

unsafe fn amdgpu_cper_ring_init(adev: *mut amdgpu_device) -> i32 { let ring = &mut (*adev).cper.ring_buf; mutex_init(&mut (*adev).cper.ring_lock); ring.adev = core::ptr::null_mut(); ring.ring_obj = core::ptr::null_mut(); ring.use_doorbell = false; ring.no_scheduler = true; ring.funcs = &cper_ring_funcs; sprintf(ring.name.as_mut_ptr(), c"cper".as_ptr()); amdgpu_ring_init(adev, ring, CPER_MAX_RING_SIZE, core::ptr::null_mut(), 0, AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut()) }

pub unsafe fn amdgpu_cper_init(adev: *mut amdgpu_device) -> i32 { if amdgpu_sriov_vf(adev) && !amdgpu_sriov_ras_cper_en(adev) || !amdgpu_sriov_vf(adev) && !amdgpu_uniras_enabled(adev) { return 0; } let r = amdgpu_cper_ring_init(adev); if r != 0 { dev_err((*adev).dev, c"failed to initialize cper ring, r = %d\n".as_ptr(), r); return r; } mutex_init(&mut (*adev).cper.cper_lock); (*adev).cper.enabled = true; (*adev).cper.max_count = CPER_MAX_ALLOWED_COUNT; 0 }

pub unsafe fn amdgpu_cper_deferred_init(adev: *mut amdgpu_device) -> i32 { if (*adev).cper.enabled { return 0; } let r = amdgpu_cper_init(adev); if r != 0 || !(*adev).cper.enabled { return r; } // CONFIG_DEBUG_FS: debugfs initialization is build-time conditional.
    #[cfg(CONFIG_DEBUG_FS)] if !(*adev_to_drm(adev)).primary.debugfs_root.is_null() { amdgpu_debugfs_ring_init(adev, &mut (*adev).cper.ring_buf); } 0 }

pub unsafe fn amdgpu_cper_fini(adev: *mut amdgpu_device) -> i32 { if amdgpu_sriov_vf(adev) && !amdgpu_sriov_ras_cper_en(adev) { return 0; } (*adev).cper.enabled = false; amdgpu_ring_fini(&mut (*adev).cper.ring_buf); (*adev).cper.count = 0; (*adev).cper.wptr = 0; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
