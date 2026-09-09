// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Intel CPU Microcode Update Driver for Linux
 *
 * Copyright (C) 2000-2006 Tigran Aivazian <aivazian.tigran@gmail.com>
 *                       2006 Shaohua Li <shaohua.li@intel.com>
 *
 * Intel CPU microcode early update for Linux
 *
 * Copyright (C) 2012 Fenghua Yu <fenghua.yu@intel.com>
 *                     H Peter Anvin" <hpa@zytor.com>
 */

// Kernel includes and C preprocessor configuration are supplied by dependencies.

static UCODE_PATH: &[u8] = b"kernel/x86/microcode/GenuineIntel.bin\0";

const MBOX_REG_NUM: usize = 4;
const MBOX_REG_SIZE: usize = core::mem::size_of::<u32>();
const MBOX_CONTROL_OFFSET: usize = 0x0;
const MBOX_STATUS_OFFSET: usize = 0x4;
const MBOX_WRDATA_OFFSET: usize = 0x8;
const MBOX_RDDATA_OFFSET: usize = 0xc;
const MASK_MBOX_CTRL_ABORT: u32 = 1 << 0;
const MASK_MBOX_CTRL_GO: u32 = 1 << 31;
const MASK_MBOX_STATUS_ERROR: u32 = 1 << 2;
const MASK_MBOX_STATUS_READY: u32 = 1 << 31;
const MASK_MBOX_RESP_SUCCESS: u32 = 1 << 0;
const MASK_MBOX_RESP_PROGRESS: u32 = 1 << 1;
const MASK_MBOX_RESP_ERROR: u32 = 1 << 2;
const MBOX_CMD_LOAD: u64 = 0x3;
const MBOX_OBJ_STAGING: u64 = 0xb;
const MBOX_HEADER_SIZE: usize = core::mem::size_of::<u64>();
const MBOX_RESPONSE_SIZE: usize = core::mem::size_of::<u64>();
const MBOX_XACTION_TIMEOUT_MS: u32 = 10 * MSEC_PER_SEC;

static mut ucode_patch_va: *mut microcode_intel = core::ptr::null_mut();
static mut ucode_patch_late: *mut microcode_intel = core::ptr::null_mut();
static mut llc_size_per_core: u32 = 0;

#[repr(C)]
pub struct extended_signature { pub sig: u32, pub pf: u32, pub cksum: u32 }
#[repr(C)]
pub struct extended_sigtable { pub count: u32, pub cksum: u32, pub reserved: [u32; 3], pub sigs: [extended_signature; 0] }

#[repr(C)]
pub struct staging_state {
    pub mmio_base: *mut core::ffi::c_void,
    pub ucode_len: u32,
    pub chunk_size: u32,
    pub bytes_sent: u32,
    pub offset: u32,
}

const fn get_totalsize(hdr: *const microcode_header_intel) -> u32 {
    unsafe { if (*hdr).datasize != 0 { (*hdr).totalsize } else { DEFAULT_UCODE_TOTALSIZE } }
}
const fn exttable_size(et: *const extended_sigtable) -> usize {
    unsafe { (*et).count as usize * EXT_SIGNATURE_SIZE + EXT_HEADER_SIZE }
}

pub unsafe fn intel_collect_cpu_info(sig: *mut cpu_signature) {
    (*sig).sig = cpuid_eax(1);
    (*sig).rev = intel_get_microcode_revision();
    (*sig).pf = 1 << intel_get_platform_id();
}

unsafe fn cpu_signatures_match(s1: *const cpu_signature, sig2: u32, pf2: u32) -> bool {
    if (*s1).sig != sig2 { return false; }
    if pf2 == 0 { return true; }
    ((*s1).pf & pf2) != 0
}

pub unsafe fn intel_find_matching_signature(mc: *mut core::ffi::c_void, sig: *const cpu_signature) -> bool {
    let hdr = mc as *mut microcode_header_intel;
    if cpu_signatures_match(sig, (*hdr).sig, (*hdr).pf) { return true; }
    if get_totalsize(hdr) as usize <= intel_microcode_get_datasize(hdr) as usize + MC_HEADER_SIZE { return false; }
    let ext = (mc as *mut u8).add(intel_microcode_get_datasize(hdr) as usize + MC_HEADER_SIZE) as *mut extended_sigtable;
    let mut s = (ext as *mut u8).add(EXT_HEADER_SIZE) as *mut extended_signature;
    for _ in 0..(*ext).count { if cpu_signatures_match(sig, (*s).sig, (*s).pf) { return true; } s = s.add(1); }
    false
}

pub unsafe fn intel_microcode_sanity_check(mc: *mut core::ffi::c_void, print_err: bool, hdr_type: i32) -> i32 {
    let hdr = mc as *mut microcode_header_intel;
    let total = get_totalsize(hdr) as usize;
    let data = intel_microcode_get_datasize(hdr) as usize;
    if data + MC_HEADER_SIZE > total { if print_err { pr_err!("Error: bad microcode data file size.\n"); } return -EINVAL; }
    if (*hdr).ldrver != 1 || (*hdr).hdrver != hdr_type as u32 { if print_err { pr_err!("Error: invalid/unknown microcode update format. Header type %d\n", (*hdr).hdrver); } return -EINVAL; }
    let ext_size = total - (MC_HEADER_SIZE + data);
    let mut ext = core::ptr::null_mut();
    let mut count = 0u32;
    if ext_size != 0 {
        if ext_size < EXT_HEADER_SIZE || (ext_size - EXT_HEADER_SIZE) % EXT_SIGNATURE_SIZE != 0 { if print_err { pr_err!("Error: truncated extended signature table.\n"); } return -EINVAL; }
        ext = (mc as *mut u8).add(MC_HEADER_SIZE + data) as *mut extended_sigtable;
        if ext_size != exttable_size(ext) { if print_err { pr_err!("Error: extended signature table size mismatch.\n"); } return -EFAULT; }
        count = (*ext).count;
        let p = ext as *const u32;
        let mut sum = 0u32; for i in 0..ext_size / 4 { sum = sum.wrapping_add(*p.add(i)); }
        if sum != 0 { if print_err { pr_warn!("Bad extended signature table checksum, aborting.\n"); } return -EINVAL; }
    }
    let p = mc as *const u32; let mut sum = 0u32;
    for i in 0..(MC_HEADER_SIZE + data) / 4 { sum = sum.wrapping_add(*p.add(i)); }
    if sum != 0 { if print_err { pr_err!("Bad microcode data checksum, aborting.\n"); } return -EINVAL; }
    if ext_size == 0 { return 0; }
    for i in 0..count as usize {
        let s = (ext as *mut u8).add(EXT_HEADER_SIZE + EXT_SIGNATURE_SIZE * i) as *mut extended_signature;
        let sum = ((*hdr).sig.wrapping_add((*hdr).pf).wrapping_add((*hdr).cksum)).wrapping_sub((*s).sig.wrapping_add((*s).pf).wrapping_add((*s).cksum));
        if sum != 0 { if print_err { pr_err!("Bad extended signature checksum, aborting.\n"); } return -EINVAL; }
    }
    0
}

unsafe fn update_ucode_pointer(mc: *mut microcode_intel) { kvfree(ucode_patch_va as *mut _); ucode_patch_va = mc; }
unsafe fn save_microcode_patch(patch: *mut microcode_intel) { let size = get_totalsize(&(*patch).hdr); let mc = kvmemdup(patch, size as usize, GFP_KERNEL); if !mc.is_null() { update_ucode_pointer(mc); } else { pr_err!("Unable to allocate microcode memory size: %u\n", size); } }

unsafe fn scan_microcode(mut data: *mut u8, mut size: usize, uci: *mut ucode_cpu_info, save: bool) -> *mut microcode_intel {
    let mut patch = core::ptr::null_mut(); let mut cur_rev = (*uci).cpu_sig.rev;
    while size >= core::mem::size_of::<microcode_header_intel>() {
        let h = data as *mut microcode_header_intel; let mc_size = get_totalsize(h) as usize;
        if mc_size == 0 || mc_size > size || intel_microcode_sanity_check(data as *mut _, false, MC_HEADER_TYPE_MICROCODE) < 0 { break; }
        if !intel_find_matching_signature(data as *mut _, &(*uci).cpu_sig) { data = data.add(mc_size); size -= mc_size; continue; }
        if (save && cur_rev != (*h).rev) || (!save && cur_rev >= (*h).rev) { data = data.add(mc_size); size -= mc_size; continue; }
        patch = data as *mut microcode_intel; cur_rev = (*h).rev; data = data.add(mc_size); size -= mc_size;
    }
    if size != 0 { core::ptr::null_mut() } else { patch }
}

unsafe fn read_mbox_dword(base: *mut core::ffi::c_void) -> u32 { let v = readl((base as *mut u8).add(MBOX_RDDATA_OFFSET)); writel(0, (base as *mut u8).add(MBOX_RDDATA_OFFSET)); v }
unsafe fn write_mbox_dword(base: *mut core::ffi::c_void, dword: u32) { writel(dword, (base as *mut u8).add(MBOX_WRDATA_OFFSET)); }
unsafe fn read_mbox_header(base: *mut core::ffi::c_void) -> u64 { let low = read_mbox_dword(base); let high = read_mbox_dword(base); ((high as u64) << 32) | low as u64 }
unsafe fn write_mbox_header(base: *mut core::ffi::c_void, value: u64) { write_mbox_dword(base, value as u32); write_mbox_dword(base, (value >> 32) as u32); }
unsafe fn write_mbox_data(base: *mut core::ffi::c_void, chunk: *mut u32, bytes: u32) { for i in 0..bytes as usize / 4 { write_mbox_dword(base, *chunk.add(i)); } }

unsafe fn init_stage(ss: *mut staging_state) { (*ss).ucode_len = get_totalsize(&(*ucode_patch_late).hdr); writel(MASK_MBOX_CTRL_ABORT, ((*ss).mmio_base as *mut u8).add(MBOX_CONTROL_OFFSET)); }
unsafe fn can_send_next_chunk(ss: *mut staging_state, err: *mut i32) -> bool { (*ss).chunk_size = core::cmp::min(PAGE_SIZE as u32, (*ss).ucode_len - (*ss).offset); if (*ss).bytes_sent + (*ss).chunk_size > (*ss).ucode_len * 2 { *err = -EMSGSIZE; false } else { *err = 0; true } }
unsafe fn is_end_offset(offset: u32) -> bool { offset == u32::MAX }
unsafe fn staging_is_complete(ss: *mut staging_state, err: *mut i32) -> bool { is_end_offset((*ss).offset) || !can_send_next_chunk(ss, err) }
unsafe fn wait_for_transaction(ss: *mut staging_state) -> i32 { let mut status = 0; for _ in 0..MBOX_XACTION_TIMEOUT_MS { msleep(1); status = readl(((*ss).mmio_base as *mut u8).add(MBOX_STATUS_OFFSET)); if status & MASK_MBOX_STATUS_READY != 0 { break; } } if status & MASK_MBOX_STATUS_ERROR != 0 { -EIO } else if status & MASK_MBOX_STATUS_READY == 0 { -ETIMEDOUT } else { 0 } }
unsafe fn send_data_chunk(ss: *mut staging_state, ptr: *mut core::ffi::c_void) -> i32 { let src = (ptr as *mut u8).add((*ss).offset as usize) as *mut u32; let size = (MBOX_HEADER_SIZE * 2) as u32 + (*ss).chunk_size; write_mbox_header((*ss).mmio_base, MBOX_HEADER(size)); write_mbox_header((*ss).mmio_base, MBOX_CMD_LOAD); write_mbox_data((*ss).mmio_base, src, (*ss).chunk_size); (*ss).bytes_sent += (*ss).chunk_size; writel(MASK_MBOX_CTRL_GO, ((*ss).mmio_base as *mut u8).add(MBOX_CONTROL_OFFSET)); wait_for_transaction(ss) }
unsafe fn fetch_next_offset(ss: *mut staging_state) -> i32 { let header = read_mbox_header((*ss).mmio_base); let offset = read_mbox_dword((*ss).mmio_base); let status = read_mbox_dword((*ss).mmio_base); if header != MBOX_HEADER((MBOX_HEADER_SIZE + MBOX_RESPONSE_SIZE) as u32) { pr_err_once!("staging: invalid response header (0x%llx)\n", header); return -EBADR; } if !is_end_offset(offset) && offset > (*ss).ucode_len { pr_err_once!("staging: invalid offset (%u) past the image end (%u)\n", offset, (*ss).ucode_len); return -EINVAL; } if status & MASK_MBOX_RESP_ERROR != 0 { return -EPROTO; } (*ss).offset = offset; 0 }

unsafe fn do_stage(mmio_pa: u64) -> i32 { let mut ss: staging_state = core::mem::zeroed(); let mut err; ss.mmio_base = ioremap(mmio_pa, MBOX_REG_NUM * MBOX_REG_SIZE); if ss.mmio_base.is_null() { return -EADDRNOTAVAIL; } init_stage(&mut ss); while !staging_is_complete(&mut ss, &mut err) { err = send_data_chunk(&mut ss, ucode_patch_late as *mut _); if err != 0 { break; } err = fetch_next_offset(&mut ss); if err != 0 { break; } } iounmap(ss.mmio_base); err }

unsafe fn stage_microcode() { let mut pkg_id = u32::MAX; if !IS_ALIGNED(get_totalsize(&(*ucode_patch_late).hdr), core::mem::size_of::<u32>()) { pr_err!("Microcode image 32-bit misaligned (0x%x), staging failed.\n", get_totalsize(&(*ucode_patch_late).hdr)); return; } lockdep_assert_cpus_held(); for_each_cpu!(cpu, cpu_primary_thread_mask) { if topology_logical_package_id(cpu) == pkg_id { continue; } pkg_id = topology_logical_package_id(cpu); let mut mmio_pa = 0u64; let err = rdmsrq_on_cpu(cpu, MSR_IA32_MCU_STAGING_MBOX_ADDR, &mut mmio_pa); if err != 0 { return; } let err = do_stage(mmio_pa); if err != 0 { pr_err!("Error: staging failed (%d) for CPU%d at package %u.\n", err, cpu, pkg_id); return; } } pr_info!("Staging of patch revision 0x%x succeeded.\n", (*ucode_patch_late).hdr.rev); }

unsafe fn __apply_microcode(uci: *mut ucode_cpu_info, mc: *mut microcode_intel, cur_rev: *mut u32) -> u32 { if mc.is_null() { return UCODE_NFOUND; } *cur_rev = intel_get_microcode_revision(); if *cur_rev >= (*mc).hdr.rev { (*uci).cpu_sig.rev = *cur_rev; return UCODE_OK; } native_wrmsrq(MSR_IA32_UCODE_WRITE, (*mc).bits as usize); let rev = intel_get_microcode_revision(); if rev != (*mc).hdr.rev { return UCODE_ERROR; } (*uci).cpu_sig.rev = rev; UCODE_UPDATED }
unsafe fn apply_microcode_early(uci: *mut ucode_cpu_info) -> u32 { let mut rev = 0; __apply_microcode(uci, (*uci).mc, &mut rev) }

// The remaining exported driver callbacks retain the C driver's interfaces; external kernel facilities provide their bodies and types.
extern "C" {
    fn load_builtin_intel_microcode(cp: *mut cpio_data) -> bool;
    fn get_microcode_blob(uci: *mut ucode_cpu_info, save: bool) -> *mut microcode_intel;
    fn save_builtin_microcode() -> i32;
    fn collect_cpu_info(cpu_num: i32, csig: *mut cpu_signature) -> i32;
    fn apply_microcode_late(cpu: i32) -> u32;
    fn parse_microcode_blobs(cpu: i32, iter: *mut iov_iter) -> u32;
    fn request_microcode_fw(cpu: i32, device: *mut device) -> u32;
    fn finalize_late_load(result: i32);
    fn calc_llc_size_per_core(c: *mut cpuinfo_x86);
    fn staging_available() -> bool;
    pub fn init_intel_microcode() -> *mut microcode_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
