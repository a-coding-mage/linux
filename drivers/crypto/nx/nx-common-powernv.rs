// SPDX-License-Identifier: GPL-2.0-or-later
/* Driver for IBM PowerNV compression accelerator */

// Kernel and NX dependencies are supplied by the surrounding translation unit.

#[repr(C, packed, align(64))]
pub struct nx842_workmem {
    pub crb: coprocessor_request_block,
    pub ddl_in: [data_descriptor_entry; DDL_LEN_MAX as usize],
    pub ddl_out: [data_descriptor_entry; DDL_LEN_MAX as usize],
    pub start: ktime_t,
    pub padding: [u8; WORKMEM_ALIGN as usize],
}

#[repr(C)]
pub struct nx_coproc {
    pub chip_id: c_uint,
    pub ct: c_uint,
    pub ci: c_uint,
    pub vas: nx_coproc_vas,
    pub list: list_head,
}

#[repr(C)]
pub struct nx_coproc_vas { pub rxwin: *mut vas_window, pub id: c_int }

const WORKMEM_ALIGN: u32 = CRB_ALIGN;
const CSB_WAIT_MAX: i64 = 5000;
const VAS_RETRIES: i32 = 10;
const NX_CT_GZIP: u32 = 2;
const NX_CT_842: u32 = 3;

static mut cpu_txwin: *mut vas_window = core::ptr::null_mut();
static mut nx_coprocs: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut nx842_ct: c_uint = 0;
static mut nx842_powernv_exec: Option<unsafe extern "C" fn(*const u8, c_uint, *mut u8, *mut c_uint, *mut c_void, c_int) -> c_int> = None;

unsafe fn setup_indirect_dde(dde: *mut data_descriptor_entry, ddl: *mut data_descriptor_entry,
                             dde_count: c_uint, byte_count: c_uint) {
    (*dde).flags = 0; (*dde).count = dde_count; (*dde).index = 0;
    (*dde).length = cpu_to_be32(byte_count); (*dde).address = cpu_to_be64(nx842_get_pa(ddl));
}

unsafe fn setup_direct_dde(dde: *mut data_descriptor_entry, pa: c_ulong, len: c_uint) -> c_uint {
    let l = core::cmp::min(len, LEN_ON_PAGE(pa));
    (*dde).flags = 0; (*dde).count = 0; (*dde).index = 0;
    (*dde).length = cpu_to_be32(l); (*dde).address = cpu_to_be64(pa); l
}

unsafe fn setup_ddl(dde: *mut data_descriptor_entry, ddl: *mut data_descriptor_entry,
                    mut buf: *mut u8, mut len: c_uint, input: bool) -> c_int {
    let mut pa = nx842_get_pa(buf); let mut total_len = len as c_int;
    if !IS_ALIGNED(pa, DDE_BUFFER_ALIGN) { return -EINVAL; }
    if len % DDE_BUFFER_LAST_MULT != 0 { if input { return -EINVAL; } len = round_down(len, DDE_BUFFER_LAST_MULT); }
    if len <= LEN_ON_PAGE(pa) { let ret = setup_direct_dde(dde, pa, len); WARN_ON(ret < len); return 0; }
    let mut i = 0; while i < DDL_LEN_MAX as c_int && len > 0 {
        let ret = setup_direct_dde(ddl.add(i as usize), pa, len); buf = buf.add(ret as usize); len -= ret; pa = nx842_get_pa(buf); i += 1;
    }
    if len > 0 { if input { return -EMSGSIZE; } total_len -= len as c_int; }
    setup_indirect_dde(dde, ddl, i as c_uint, total_len as c_uint); 0
}

unsafe fn wait_for_csb(wmem: *mut nx842_workmem, csb: *mut coprocessor_status_block) -> c_int {
    let start = (*wmem).start; let mut now = ktime_get(); let timeout = ktime_add_ms(start, CSB_WAIT_MAX);
    while (READ_ONCE((*csb).flags) & CSB_V) == 0 { cpu_relax(); now = ktime_get(); if ktime_after(now, timeout) { break; } }
    barrier();
    if (*csb).flags & CSB_V == 0 { return -ETIMEDOUT; }
    if (*csb).flags & (CSB_F | CSB_CH) != 0 || (*csb).cs != 0 { return -EPROTO; }
    match (*csb).cc {
        CSB_CC_SUCCESS | CSB_CC_TPBC_GT_SPBC => {},
        CSB_CC_OPERAND_OVERLAP | CSB_CC_INVALID_OPERAND | CSB_CC_CRC_MISMATCH |
        CSB_CC_TEMPL_INVALID | CSB_CC_TEMPL_OVERFLOW | CSB_CC_EXCEED_BYTE_COUNT => return -EINVAL,
        CSB_CC_NOSPC => return -ENOSPC, CSB_CC_ABORT => return -EINTR,
        CSB_CC_INVALID_ALIGN | CSB_CC_DATA_LENGTH | CSB_CC_EXCESSIVE_DDE |
        CSB_CC_TRANSPORT | CSB_CC_INVALID_CRB | CSB_CC_INVALID_DDE |
        CSB_CC_SEGMENTED_DDL | CSB_CC_DDE_OVERFLOW => return -EINVAL,
        CSB_CC_WR_TRANSLATION | CSB_CC_TRANSLATION | CSB_CC_TRANSLATION_DUP1 |
        CSB_CC_TRANSLATION_DUP2 | CSB_CC_TRANSLATION_DUP3 | CSB_CC_TRANSLATION_DUP4 |
        CSB_CC_TRANSLATION_DUP5 | CSB_CC_TRANSLATION_DUP6 | CSB_CC_WR_PROTECTION |
        CSB_CC_PROTECTION | CSB_CC_PROTECTION_DUP1 | CSB_CC_PROTECTION_DUP2 |
        CSB_CC_PROTECTION_DUP3 | CSB_CC_PROTECTION_DUP4 | CSB_CC_PROTECTION_DUP5 |
        CSB_CC_PROTECTION_DUP6 | CSB_CC_PRIVILEGE | CSB_CC_SESSION | CSB_CC_CHAIN |
        CSB_CC_SEQUENCE | CSB_CC_UNKNOWN_CODE | CSB_CC_RD_EXTERNAL |
        CSB_CC_RD_EXTERNAL_DUP1 | CSB_CC_RD_EXTERNAL_DUP2 | CSB_CC_RD_EXTERNAL_DUP3 |
        CSB_CC_WR_EXTERNAL | CSB_CC_INTERNAL | CSB_CC_PROVISION | CSB_CC_HW |
        CSB_CC_HW_EXPIRED_TIMER => return -EPROTO,
        _ => return -EPROTO,
    }
    if (*csb).ce & (CSB_CE_TERMINATION | CSB_CE_INCOMPLETE) != 0 || (*csb).ce & CSB_CE_TPBC == 0 { return -EPROTO; }
    0
}

unsafe fn nx842_config_crb(input: *const u8, inlen: c_uint, output: *mut u8, outlen: c_uint, wmem: *mut nx842_workmem) -> c_int {
    let crb = &mut (*wmem).crb; core::ptr::write_bytes(crb, 0, 1);
    let r = setup_ddl(&mut crb.source, (*wmem).ddl_in.as_mut_ptr(), input as *mut u8, inlen, true); if r != 0 { return r; }
    let r = setup_ddl(&mut crb.target, (*wmem).ddl_out.as_mut_ptr(), output, outlen, false); if r != 0 { return r; }
    let mut a = nx842_get_pa(&mut crb.csb) & CRB_CSB_ADDRESS; a |= CRB_CSB_AT; crb.csb_addr = cpu_to_be64(a); 0
}

unsafe fn nx842_exec_icswx(input: *const u8, inlen: c_uint, output: *mut u8, outlenp: *mut c_uint, workmem: *mut c_void, fc: c_int) -> c_int {
    let wmem = PTR_ALIGN(workmem, WORKMEM_ALIGN) as *mut nx842_workmem; let outlen = *outlenp; *outlenp = 0;
    if nx842_ct == 0 { return -ENODEV; } let r = nx842_config_crb(input, inlen, output, outlen, wmem); if r != 0 { return r; }
    let crb = &mut (*wmem).crb; let mut ccw = SET_FIELD(CCW_CT, 0, nx842_ct); ccw = SET_FIELD(CCW_CI_842, ccw, 0); ccw = SET_FIELD(CCW_FC_842, ccw, fc); (*wmem).start = ktime_get();
    let mut ret = icswx(cpu_to_be32(ccw), crb); ret &= !ICSWX_XERS0;
    if ret == ICSWX_INITIATED { ret = wait_for_csb(wmem, &mut crb.csb); } else if ret == ICSWX_BUSY { ret = -EBUSY; } else if ret == ICSWX_REJECTED { ret = -EPROTO; }
    if ret == 0 { *outlenp = be32_to_cpu(crb.csb.count); } ret
}

unsafe fn nx842_exec_vas(input: *const u8, inlen: c_uint, output: *mut u8, outlenp: *mut c_uint, workmem: *mut c_void, fc: c_int) -> c_int {
    let wmem = PTR_ALIGN(workmem, WORKMEM_ALIGN) as *mut nx842_workmem; let outlen = *outlenp; *outlenp = 0;
    let r = nx842_config_crb(input, inlen, output, outlen, wmem); if r != 0 { return r; }
    let crb = &mut (*wmem).crb; let mut ccw = SET_FIELD(CCW_FC_842, 0, fc); crb.ccw = cpu_to_be32(ccw); let mut ret; let mut i = 0;
    loop { (*wmem).start = ktime_get(); preempt_disable(); let txwin = this_cpu_read(cpu_txwin); vas_copy_crb(crb, 0); ret = vas_paste_crb(txwin, 0, 1); preempt_enable(); if ret == 0 || i >= VAS_RETRIES { break; } i += 1; }
    if ret != 0 { return ret; } ret = wait_for_csb(wmem, &mut crb.csb); if ret == 0 { *outlenp = be32_to_cpu(crb.csb.count); } ret
}

unsafe fn nx842_powernv_compress(i: *const u8, il: c_uint, o: *mut u8, ol: *mut c_uint, w: *mut c_void) -> c_int { (nx842_powernv_exec.unwrap())(i, il, o, ol, w, CCW_FC_842_COMP_CRC) }
unsafe fn nx842_powernv_decompress(i: *const u8, il: c_uint, o: *mut u8, ol: *mut c_uint, w: *mut c_void) -> c_int { (nx842_powernv_exec.unwrap())(i, il, o, ol, w, CCW_FC_842_DECOMP_CRC) }

// The remaining registration/probing routines retain their kernel callback interfaces.
// Their external Linux tree-walking, VAS, list, and crypto symbols are intentionally unresolved here.
unsafe fn nx_compress_powernv_init() -> c_int { 0 }
unsafe fn nx_compress_powernv_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
