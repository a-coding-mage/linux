// SPDX-License-Identifier: GPL-2.0-only
/* GHES/EDAC Linux driver */

const OTHER_DETAIL_LEN: usize = 400;

#[repr(C)]
struct GhesPvt {
    mci: *mut MemCtlInfo,
    other_detail: [i8; OTHER_DETAIL_LEN],
    msg: [i8; 80],
}

static mut GHES_REFCOUNT: Refcount = Refcount::new(0);
static mut GHES_PVT: *mut GhesPvt = core::ptr::null_mut();

#[repr(C)]
struct GhesHwDesc {
    num_dimms: i32,
    dimms: *mut DimmInfo,
}

static mut GHES_HW: GhesHwDesc = GhesHwDesc { num_dimms: 0, dimms: core::ptr::null_mut() };
static mut SYSTEM_SCANNED: bool = false;
static mut GHES_DEVS: *mut ListHead = core::ptr::null_mut();

#[repr(C, packed)]
struct MemdevDmiEntry {
    type_: u8, length: u8, handle: u16, phys_mem_array_handle: u16,
    mem_err_info_handle: u16, total_width: u16, data_width: u16, size: u16,
    form_factor: u8, device_set: u8, device_locator: u8, bank_locator: u8,
    memory_type: u8, type_detail: u16, speed: u16, manufacturer: u8,
    serial_number: u8, asset_tag: u8, part_number: u8, attributes: u8,
    extended_size: u32, conf_mem_clk_speed: u16,
}

unsafe fn find_dimm_by_handle(mci: *mut MemCtlInfo, handle: u16) -> *mut DimmInfo {
    let mut dimm: *mut DimmInfo = core::ptr::null_mut();
    mci_for_each_dimm!(mci, dimm) {
        if (*dimm).smbios_handle == handle { return dimm; }
    }
    core::ptr::null_mut()
}

unsafe fn dimm_setup_label(dimm: *mut DimmInfo, handle: u16) {
    let mut bank: *const i8 = core::ptr::null();
    let mut device: *const i8 = core::ptr::null();
    dmi_memdev_name(handle, &mut bank, &mut device);
    snprintf_label((*dimm).label.as_mut_ptr(), (*dimm).label.len(), bank, device);
}

unsafe fn assign_dmi_dimm_info(dimm: *mut DimmInfo, entry: *mut MemdevDmiEntry) {
    let rdr_mask: u16 = (1 << 7) | (1 << 13);
    if (*entry).size == 0xffff {
        pr_info!("Can't get DIMM%i size\n", (*dimm).idx);
        (*dimm).nr_pages = MiB_TO_PAGES!(32);
    } else if (*entry).size == 0x7fff {
        (*dimm).nr_pages = MiB_TO_PAGES!((*entry).extended_size);
    } else if (*entry).size & (1 << 15) != 0 {
        (*dimm).nr_pages = MiB_TO_PAGES!(((*entry).size & 0x7fff) << 10);
    } else { (*dimm).nr_pages = MiB_TO_PAGES!((*entry).size); }

    (*dimm).mtype = match (*entry).memory_type {
        0x12 => if (*entry).type_detail & (1 << 13) != 0 { MEM_RDDR } else { MEM_DDR },
        0x13 => if (*entry).type_detail & (1 << 13) != 0 { MEM_RDDR2 } else { MEM_DDR2 },
        0x14 => MEM_FB_DDR2,
        0x18 => if (*entry).type_detail & (1 << 12) != 0 { MEM_NVDIMM } else if (*entry).type_detail & (1 << 13) != 0 { MEM_RDDR3 } else { MEM_DDR3 },
        0x1a => if (*entry).type_detail & (1 << 12) != 0 { MEM_NVDIMM } else if (*entry).type_detail & (1 << 13) != 0 { MEM_RDDR4 } else { MEM_DDR4 },
        _ => if (*entry).type_detail & (1 << 6) != 0 { MEM_RMBS } else if (*entry).type_detail & rdr_mask == rdr_mask { MEM_RDR } else if (*entry).type_detail & (1 << 7) != 0 { MEM_SDR } else if (*entry).type_detail & (1 << 9) != 0 { MEM_EDO } else { MEM_UNKNOWN },
    };
    (*dimm).edac_mode = if (*entry).total_width == (*entry).data_width { EDAC_NONE } else { EDAC_SECDED };
    (*dimm).dtype = DEV_UNKNOWN; (*dimm).grain = 128;
    dimm_setup_label(dimm, (*entry).handle);
    (*dimm).smbios_handle = (*entry).handle;
}

unsafe extern "C" fn enumerate_dimms(dh: *const DmiHeader, arg: *mut core::ffi::c_void) {
    let entry = dh as *mut MemdevDmiEntry;
    let hw = arg as *mut GhesHwDesc;
    if (*dh).type_ != DMI_ENTRY_MEM_DEVICE { return; }
    if (*hw).num_dimms == 0 || (*hw).num_dimms % 16 == 0 {
        let new = krealloc_array((*hw).dimms, ((*hw).num_dimms + 16) as usize, core::mem::size_of::<DimmInfo>(), GFP_KERNEL);
        if new.is_null() { WARN_ON_ONCE!(1); return; }
        (*hw).dimms = new as *mut DimmInfo;
    }
    let d = (*hw).dimms.add((*hw).num_dimms as usize);
    (*d).idx = (*hw).num_dimms;
    assign_dmi_dimm_info(d, entry);
    (*hw).num_dimms += 1;
}

unsafe fn ghes_scan_system() {
    if SYSTEM_SCANNED { return; }
    dmi_walk(enumerate_dimms, &mut GHES_HW as *mut _ as *mut core::ffi::c_void);
    SYSTEM_SCANNED = true;
}

unsafe fn print_mem_error_other_detail(mem: *const CperSecMemErr, msg: *mut i8, location: *const i8, mut len: usize) -> i32 {
    if msg.is_null() { return 0; }
    let mut n: usize = 0; len -= 1;
    n += scnprintf!(msg.add(n), len - n, "APEI location: %s ", location);
    if (*mem).validation_bits & CPER_MEM_VALID_ERROR_STATUS != 0 {
        n += scnprintf!(msg.add(n), len - n, "status(0x%016llx): ", (*mem).error_status);
        n += scnprintf!(msg.add(n), len - n, "%s ", cper_mem_err_status_str((*mem).error_status));
    }
    *msg.add(n) = 0; n as i32
}

unsafe extern "C" fn ghes_edac_report_mem_error(_nb: *mut NotifierBlock, val: usize, data: *mut core::ffi::c_void) -> i32 {
    let mem_err = data as *mut CperSecMemErr;
    let mut flags = 0usize;
    if WARN_ON_ONCE!(in_nmi()) { return NOTIFY_OK; }
    spin_lock_irqsave!(&GHES_LOCK, flags);
    let pvt = GHES_PVT;
    if pvt.is_null() { spin_unlock_irqrestore!(&GHES_LOCK, flags); return NOTIFY_OK; }
    let mci = (*pvt).mci; let e = &mut (*mci).error_desc;
    core::ptr::write_bytes(e as *mut _, 0, 1);
    e.error_count = 1; e.grain = 1; e.msg = (*pvt).msg.as_mut_ptr(); e.other_detail = (*pvt).other_detail.as_mut_ptr();
    e.top_layer = -1; e.mid_layer = -1; e.low_layer = -1; (*pvt).other_detail[0] = 0; (*pvt).msg[0] = 0;
    e.type_ = match val { GHES_SEV_CORRECTED => HW_EVENT_ERR_CORRECTED, GHES_SEV_RECOVERABLE => HW_EVENT_ERR_UNCORRECTED, GHES_SEV_PANIC => HW_EVENT_ERR_FATAL, _ => HW_EVENT_ERR_INFO };
    if (*mem_err).validation_bits & CPER_MEM_VALID_ERROR_TYPE != 0 { snprintf!(e.msg, 80, "%s", cper_mem_err_type_str((*mem_err).error_type)); } else { strscpy!(e.msg, "unknown error"); }
    if (*mem_err).validation_bits & CPER_MEM_VALID_PA != 0 { e.page_frame_number = PHYS_PFN!((*mem_err).physical_addr); e.offset_in_page = offset_in_page!((*mem_err).physical_addr); }
    if (*mem_err).validation_bits & CPER_MEM_VALID_PA_MASK != 0 { e.grain = (!(*mem_err).physical_addr_mask).wrapping_add(1); }
    let mut p = e.location.as_mut_ptr(); let mut cmem = CperMemErrCompact::default(); cper_mem_err_pack(mem_err, &mut cmem); p = p.add(cper_mem_err_location(&cmem, p));
    if (*mem_err).validation_bits & CPER_MEM_VALID_MODULE_HANDLE != 0 { p = p.add(cper_dimm_err_location(&cmem, p)); let dimm = find_dimm_by_handle(mci, (*mem_err).mem_dev_handle); if !dimm.is_null() { e.top_layer = (*dimm).idx; strscpy!(e.label, (*dimm).label); } }
    if p > e.location.as_mut_ptr() { *p.sub(1) = 0; } if e.label[0] == 0 { strscpy!(e.label, "unknown memory"); }
    let mut q = (*pvt).other_detail.as_mut_ptr(); q = q.add(print_mem_error_other_detail(mem_err, q, e.location.as_ptr(), OTHER_DETAIL_LEN) as usize); if q > (*pvt).other_detail.as_mut_ptr() { *q.sub(1) = 0; }
    edac_raw_mc_handle_error(e); spin_unlock_irqrestore!(&GHES_LOCK, flags); NOTIFY_OK
}

static mut GHES_EDAC_MEM_ERR_NB: NotifierBlock = NotifierBlock { notifier_call: Some(ghes_edac_report_mem_error), priority: 0 };

unsafe extern "C" fn ghes_edac_register(dev: *mut Device) -> i32 {
    mutex_lock!(&GHES_REG_MUTEX); if refcount_inc_not_zero!(&mut GHES_REFCOUNT) { mutex_unlock!(&GHES_REG_MUTEX); return 0; }
    ghes_scan_system(); let fake = GHES_HW.num_dimms == 0; if fake { GHES_HW.num_dimms = 1; }
    let mut layer = EdacMcLayer { type_: EDAC_MC_LAYER_ALL_MEM, size: GHES_HW.num_dimms, is_virt_csrow: true };
    let mci = edac_mc_alloc(0, 1, &mut layer, core::mem::size_of::<GhesPvt>()); if mci.is_null() { mutex_unlock!(&GHES_REG_MUTEX); return -ENOMEM; }
    let pvt = (*mci).pvt_info as *mut GhesPvt; (*pvt).mci = mci; (*mci).pdev = dev;
    (*mci).mtype_cap = MEM_FLAG_EMPTY; (*mci).edac_ctl_cap = EDAC_FLAG_NONE; (*mci).edac_cap = EDAC_FLAG_NONE;
    (*mci).mod_name = c"ghes_edac.c".as_ptr(); (*mci).ctl_name = c"ghes_edac".as_ptr(); (*mci).dev_name = c"ghes".as_ptr();
    if !fake {
        let mut i = 0;
        mci_for_each_dimm!(mci, dst) { let src = GHES_HW.dimms.add(i); (*dst).idx = (*src).idx; (*dst).smbios_handle = (*src).smbios_handle; (*dst).nr_pages = (*src).nr_pages; (*dst).mtype = (*src).mtype; (*dst).edac_mode = (*src).edac_mode; (*dst).dtype = (*src).dtype; (*dst).grain = (*src).grain; if strlen!((*src).label.as_ptr()) != 0 { memcpy!((*dst).label.as_mut_ptr(), (*src).label.as_ptr(), (*src).label.len()); } i += 1; }
    } else {
        let dimm = edac_get_dimm(mci, 0, 0, 0); (*dimm).nr_pages = 1; (*dimm).grain = 128; (*dimm).mtype = MEM_UNKNOWN; (*dimm).dtype = DEV_UNKNOWN; (*dimm).edac_mode = EDAC_SECDED;
    }
    let rc = edac_mc_add_mc(mci); if rc < 0 { edac_mc_free(mci); mutex_unlock!(&GHES_REG_MUTEX); return -ENODEV; }
    spin_lock_irqsave!(&GHES_LOCK, 0); GHES_PVT = pvt; spin_unlock_irqrestore!(&GHES_LOCK, 0); ghes_register_report_chain(&mut GHES_EDAC_MEM_ERR_NB); refcount_set!(&mut GHES_REFCOUNT, 1); kfree!(GHES_HW.dimms); GHES_HW.dimms = core::ptr::null_mut(); mutex_unlock!(&GHES_REG_MUTEX); 0
}

unsafe extern "C" fn ghes_edac_unregister(_ghes: *mut Ghes) { mutex_lock!(&GHES_REG_MUTEX); SYSTEM_SCANNED = false; GHES_HW = GhesHwDesc { num_dimms: 0, dimms: core::ptr::null_mut() }; if refcount_dec_and_test!(&mut GHES_REFCOUNT) { let mci = if GHES_PVT.is_null() { core::ptr::null_mut() } else { (*GHES_PVT).mci }; GHES_PVT = core::ptr::null_mut(); if !mci.is_null() { let m = edac_mc_del_mc((*mci).pdev); if !m.is_null() { edac_mc_free(m); } } ghes_unregister_report_chain(&mut GHES_EDAC_MEM_ERR_NB); } mutex_unlock!(&GHES_REG_MUTEX); }

unsafe extern "C" fn ghes_edac_init() -> i32 { GHES_DEVS = ghes_get_devices(); if GHES_DEVS.is_null() || list_empty!(GHES_DEVS) { return -ENODEV; } list_for_each_entry_safe!(g, g_tmp, GHES_DEVS, elist, { ghes_edac_register((*g).dev); }); 0 }
unsafe extern "C" fn ghes_edac_exit() { list_for_each_entry_safe!(g, g_tmp, GHES_DEVS, elist, { ghes_edac_unregister(g); }); }

module_init!(ghes_edac_init);
module_exit!(ghes_edac_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("Output ACPI APEI/GHES BIOS detected errors via EDAC");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
