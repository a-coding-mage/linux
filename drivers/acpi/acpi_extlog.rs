// SPDX-License-Identifier: GPL-2.0-only
/*
 * Extended Error Log driver
 *
 * Copyright (C) 2013 Intel Corp.
 * Author: Chen, Gong <gong.chen@intel.com>
 */

// Kernel headers and symbols used by this translation are supplied by other
// Rust bindings/modules.

const EXT_ELOG_ENTRY_MASK: u64 = (1u64 << 52) - 1; /* elog entry address mask */
const EXTLOG_DSM_REV: u64 = 0x0;
const EXTLOG_FN_ADDR: u64 = 0x1;
const FLAG_OS_OPTIN: u32 = 1 << 0;
const ELOG_ENTRY_VALID: u64 = 1u64 << 63;
const ELOG_ENTRY_LEN: usize = 0x1000;

const EMCA_BUG: &str =
    "Can not request iomem region <0x%016llx-0x%016llx> - eMCA disabled\n";

#[repr(C)]
struct extlog_l1_head {
    ver: u32,
    hdr_len: u32,
    total_len: u64,
    elog_base: u64,
    elog_len: u64,
    flags: u32,
    rev0: [u8; 12],
    entries: u32,
    rev1: [u8; 12],
}

#[link_section = ".init.data"]
static mut extlog_dsm_uuid: [u8; 37] = *b"663E35AF-CC10-41A4-88EA-5470AF055295\0";

static mut elog_base: u64 = 0;
static mut elog_size: usize = 0;
static mut l1_dirbase: u64 = 0;
static mut l1_size: usize = 0;
static mut extlog_l1_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut elog_addr: *mut core::ffi::c_void = core::ptr::null_mut();
static mut elog_buf: *mut core::ffi::c_void = core::ptr::null_mut();
static mut l1_entry_base: *mut u64 = core::ptr::null_mut();
static mut l1_percpu_entry: u32 = 0;

#[inline]
unsafe fn elog_idx(cpu: i32, bank: i32) -> usize {
    (cpu_physical_id(cpu) as usize) * l1_percpu_entry as usize + bank as usize
}

#[inline]
unsafe fn elog_entry_data(idx: usize) -> u64 {
    core::ptr::read_volatile(l1_entry_base.add(idx))
}

#[inline]
unsafe fn elog_entry_addr(phyaddr: u64) -> *mut acpi_hest_generic_status {
    (elog_addr as *mut u8).add((phyaddr - elog_base) as usize)
        as *mut acpi_hest_generic_status
}

unsafe fn extlog_elog_entry_check(cpu: i32, bank: i32) -> *mut acpi_hest_generic_status {
    let idx: usize;
    let mut data: u64;
    let estatus: *mut acpi_hest_generic_status;

    WARN_ON(cpu < 0);
    idx = elog_idx(cpu, bank);
    data = elog_entry_data(idx);
    if data & ELOG_ENTRY_VALID == 0 {
        return core::ptr::null_mut();
    }
    data &= EXT_ELOG_ENTRY_MASK;
    estatus = elog_entry_addr(data);
    if (*estatus).block_status == 0 {
        return core::ptr::null_mut();
    }
    estatus
}

unsafe fn __print_extlog_rcd(
    mut pfx: *const core::ffi::c_char,
    estatus: *mut acpi_hest_generic_status,
    cpu: i32,
) {
    static mut seqno: atomic_t = atomic_t { counter: 0 };
    let curr_seqno: u32;
    let mut pfx_seq = [0i8; 64];

    if pfx.is_null() {
        if (*estatus).error_severity <= CPER_SEV_CORRECTED {
            pfx = KERN_INFO;
        } else {
            pfx = KERN_ERR;
        }
    }
    curr_seqno = atomic_inc_return(&mut seqno) as u32;
    snprintf(pfx_seq.as_mut_ptr(), pfx_seq.len(), c"%s{%u}".as_ptr(), pfx, curr_seqno);
    printk(c"%sHardware error detected on CPU%d\n".as_ptr(), pfx_seq.as_ptr(), cpu);
    cper_estatus_print(pfx_seq.as_ptr(), estatus);
}

unsafe fn print_extlog_rcd(
    pfx: *const core::ffi::c_char,
    estatus: *mut acpi_hest_generic_status,
    cpu: i32,
) -> i32 {
    static mut ratelimit_corrected: ratelimit_state = DEFINE_RATELIMIT_STATE!(5 * HZ, 2);
    static mut ratelimit_uncorrected: ratelimit_state = DEFINE_RATELIMIT_STATE!(5 * HZ, 2);
    let ratelimit: *mut ratelimit_state;

    if (*estatus).error_severity == CPER_SEV_CORRECTED
        || (*estatus).error_severity == CPER_SEV_INFORMATIONAL
    {
        ratelimit = &mut ratelimit_corrected;
    } else {
        ratelimit = &mut ratelimit_uncorrected;
    }
    if __ratelimit(ratelimit) {
        __print_extlog_rcd(pfx, estatus, cpu);
        return 0;
    }
    1
}

unsafe fn extlog_print_pcie(pcie_err: *mut cper_sec_pcie, severity: i32) {
    // #ifdef ACPI_APEI_PCIEAER
    if (*pcie_err).validation_bits & CPER_PCIE_VALID_DEVICE_ID == 0
        || (*pcie_err).validation_bits & CPER_PCIE_VALID_AER_INFO == 0
    {
        return;
    }
    let aer_severity = cper_severity_to_aer(severity);
    let aer = (*pcie_err).aer_info.as_ptr() as *mut aer_capability_regs;
    let domain = (*pcie_err).device_id.segment;
    let bus = (*pcie_err).device_id.bus;
    let devfn = PCI_DEVFN((*pcie_err).device_id.device, (*pcie_err).device_id.function);
    let pdev = pci_get_domain_bus_and_slot(domain, bus, devfn);
    if pdev.is_null() {
        return;
    }
    pci_print_aer(pdev, aer_severity, aer);
    pci_dev_put(pdev);
    // #endif
}

unsafe fn extlog_cxl_cper_handle_prot_err(
    prot_err: *mut cxl_cper_sec_prot_err,
    severity: i32,
) {
    // #ifdef ACPI_APEI_PCIEAER
    let mut wd: cxl_cper_prot_err_work_data = core::mem::zeroed();
    if cxl_cper_sec_prot_err_valid(prot_err) {
        return;
    }
    if cxl_cper_setup_prot_err_work_data(&mut wd, prot_err, severity) != 0 {
        return;
    }
    cxl_cper_handle_prot_err(&mut wd);
    // #endif
}

unsafe fn extlog_print(nb: *mut notifier_block, val: u64, data: *mut core::ffi::c_void) -> i32 {
    let mce = data as *mut mce;
    let bank = (*mce).bank;
    let cpu = (*mce).extcpu;
    let estatus = extlog_elog_entry_check(cpu, bank);
    if estatus.is_null() {
        return NOTIFY_DONE;
    }
    if (*mce).kflags & MCE_HANDLED_CEC != 0 {
        (*estatus).block_status = 0;
        return NOTIFY_DONE;
    }
    core::ptr::copy_nonoverlapping(estatus as *const u8, elog_buf as *mut u8, ELOG_ENTRY_LEN);
    (*estatus).block_status = 0;
    let tmp = elog_buf as *mut acpi_hest_generic_status;
    if !ras_userspace_consumers() {
        print_extlog_rcd(core::ptr::null(), tmp, cpu);
    } else {
        static mut err_seq: u32 = 0;
        err_seq = err_seq.wrapping_add(1);
        let mut gdata: *mut acpi_hest_generic_data = core::ptr::null_mut();
        apei_estatus_for_each_section!(tmp, gdata, {
            let fru_id = if (*gdata).validation_bits & CPER_SEC_VALID_FRU_ID != 0 {
                (*gdata).fru_id.as_ptr() as *const guid_t
            } else { &guid_null };
            let fru_text = if (*gdata).validation_bits & CPER_SEC_VALID_FRU_TEXT != 0 {
                (*gdata).fru_text.as_ptr()
            } else { c"".as_ptr() };
            let sec_type = (*gdata).section_type.as_ptr() as *mut guid_t;
            if guid_equal(sec_type, &CPER_SEC_PLATFORM_MEM) {
                let mem = acpi_hest_get_payload(gdata) as *mut cper_sec_mem_err;
                if (*gdata).error_data_length >= core::mem::size_of::<cper_sec_mem_err>() {
                    trace_extlog_mem_event(mem, err_seq, fru_id, fru_text, (*gdata).error_severity as u8);
                }
            } else if guid_equal(sec_type, &CPER_SEC_CXL_PROT_ERR) {
                extlog_cxl_cper_handle_prot_err(acpi_hest_get_payload(gdata), (*gdata).error_severity);
            } else if guid_equal(sec_type, &CPER_SEC_PCIE) {
                extlog_print_pcie(acpi_hest_get_payload(gdata), (*gdata).error_severity);
            } else {
                log_non_standard_event(sec_type, fru_id, fru_text, (*gdata).error_severity,
                    acpi_hest_get_payload(gdata), (*gdata).error_data_length);
            }
        });
    }
    (*mce).kflags |= MCE_HANDLED_EXTLOG;
    NOTIFY_OK
}

unsafe fn extlog_get_l1addr() -> bool {
    let mut guid: guid_t = core::mem::zeroed();
    let mut handle: acpi_handle = core::ptr::null_mut();
    let obj: *mut acpi_object;
    if guid_parse(extlog_dsm_uuid.as_ptr(), &mut guid) != 0 { return false; }
    if ACPI_FAILURE(acpi_get_handle(core::ptr::null_mut(), c"\\_SB".as_ptr(), &mut handle)) { return false; }
    if !acpi_check_dsm(handle, &guid, EXTLOG_DSM_REV, 1 << EXTLOG_FN_ADDR) { return false; }
    obj = acpi_evaluate_dsm_typed(handle, &guid, EXTLOG_DSM_REV, EXTLOG_FN_ADDR, core::ptr::null_mut(), ACPI_TYPE_INTEGER);
    if obj.is_null() { return false; }
    l1_dirbase = (*obj).integer.value;
    ACPI_FREE(obj);
    if l1_dirbase & ((1u64 << 12) - 1) != 0 {
        pr_warn(c"L1 Directory is invalid at physical %llx\n".as_ptr(), l1_dirbase);
        return false;
    }
    true
}

static mut extlog_mce_dec: notifier_block = notifier_block {
    notifier_call: Some(extlog_print),
    priority: MCE_PRIO_EXTLOG,
};

unsafe fn extlog_init() -> i32 {
    let mut l1_head: *mut extlog_l1_head;
    let mut extlog_l1_hdr: *mut core::ffi::c_void;
    let l1_hdr_size = core::mem::size_of::<extlog_l1_head>();
    let mut r: *mut resource;
    let mut cap: u64 = 0;
    let mut rc: i32;
    if rdmsrq_safe(MSR_IA32_MCG_CAP, &mut cap) != 0 || cap & MCG_ELOG_P == 0 || !extlog_get_l1addr() { return -ENODEV; }
    rc = -EINVAL;
    r = request_mem_region(l1_dirbase, l1_hdr_size, c"L1 DIR HDR".as_ptr());
    if r.is_null() { pr_warn(EMCA_BUG.as_ptr(), l1_dirbase, l1_dirbase + l1_hdr_size as u64); return rc; }
    extlog_l1_hdr = acpi_os_map_iomem(l1_dirbase, l1_hdr_size);
    if extlog_l1_hdr.is_null() { rc = -ENOMEM; goto err_release_l1_hdr; }
    l1_head = extlog_l1_hdr as *mut extlog_l1_head;
    l1_size = (*l1_head).total_len as usize;
    l1_percpu_entry = (*l1_head).entries;
    elog_base = (*l1_head).elog_base;
    elog_size = (*l1_head).elog_len as usize;
    acpi_os_unmap_iomem(extlog_l1_hdr, l1_hdr_size);
    release_mem_region(l1_dirbase, l1_hdr_size);
    r = request_mem_region(l1_dirbase, l1_size, c"L1 Table".as_ptr());
    if r.is_null() { pr_warn(EMCA_BUG.as_ptr(), l1_dirbase, l1_dirbase + l1_size as u64); goto err; }
    extlog_l1_addr = acpi_os_map_iomem(l1_dirbase, l1_size);
    if extlog_l1_addr.is_null() { rc = -ENOMEM; goto err_release_l1_dir; }
    l1_entry_base = (extlog_l1_addr as *mut u8).add(l1_hdr_size) as *mut u64;
    r = request_mem_region(elog_base, elog_size, c"Elog Table".as_ptr());
    if r.is_null() { pr_warn(EMCA_BUG.as_ptr(), elog_base, elog_base + elog_size as u64); goto err_release_l1_dir; }
    elog_addr = acpi_os_map_iomem(elog_base, elog_size);
    if elog_addr.is_null() { rc = -ENOMEM; goto err_release_elog; }
    rc = -ENOMEM;
    elog_buf = kmalloc(ELOG_ENTRY_LEN, GFP_KERNEL);
    if elog_buf.is_null() { goto err_release_elog; }
    mce_register_decode_chain(&mut extlog_mce_dec);
    (*(extlog_l1_addr as *mut extlog_l1_head)).flags |= FLAG_OS_OPTIN;
    return 0;
err_release_elog:
    if !elog_addr.is_null() { acpi_os_unmap_iomem(elog_addr, elog_size); }
    release_mem_region(elog_base, elog_size);
err_release_l1_dir:
    if !extlog_l1_addr.is_null() { acpi_os_unmap_iomem(extlog_l1_addr, l1_size); }
    release_mem_region(l1_dirbase, l1_size);
err_release_l1_hdr:
    release_mem_region(l1_dirbase, l1_hdr_size);
err:
    pr_warn(c"Extended error log disabled because of problems parsing f/w tables\n".as_ptr());
    rc
}

unsafe fn extlog_exit() {
    mce_unregister_decode_chain(&mut extlog_mce_dec);
    if !extlog_l1_addr.is_null() {
        (*(extlog_l1_addr as *mut extlog_l1_head)).flags &= !FLAG_OS_OPTIN;
        acpi_os_unmap_iomem(extlog_l1_addr, l1_size);
    }
    if !elog_addr.is_null() { acpi_os_unmap_iomem(elog_addr, elog_size); }
    release_mem_region(elog_base, elog_size);
    release_mem_region(l1_dirbase, l1_size);
    kfree(elog_buf);
}

module_init!(extlog_init);
module_exit!(extlog_exit);
module_author!("Chen, Gong <gong.chen@intel.com>");
module_description!("Extended MCA Error Log Driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
