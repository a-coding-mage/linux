// SPDX-License-Identifier: GPL-2.0
/* psycho_common.c: Code common to PSYCHO and derivative PCI controllers. */
// Translated from the C implementation; Linux and architecture dependencies
// are supplied by the surrounding kernel translation.

const PSYCHO_STRBUF_CTRL_DENAB: u64 = 0x0000000000000002;
const PSYCHO_STCERR_WRITE: u64 = 0x0000000000000002;
const PSYCHO_STCERR_READ: u64 = 0x0000000000000001;
const PSYCHO_STCTAG_PPN: u64 = 0x0fffffff00000000;
const PSYCHO_STCTAG_VPN: u64 = 0x00000000ffffe000;
const PSYCHO_STCTAG_VALID: u64 = 0x0000000000000002;
const PSYCHO_STCTAG_WRITE: u64 = 0x0000000000000001;
const PSYCHO_STCLINE_LINDX: u64 = 0x0000000001e00000;
const PSYCHO_STCLINE_SPTR: u64 = 0x00000000001f8000;
const PSYCHO_STCLINE_LADDR: u64 = 0x0000000000007f00;
const PSYCHO_STCLINE_EPTR: u64 = 0x00000000000000fc;
const PSYCHO_STCLINE_VALID: u64 = 0x0000000000000002;
const PSYCHO_STCLINE_FOFN: u64 = 0x0000000000000001;

static mut STC_BUF_LOCK: SpinLock = SpinLock::new();
static mut STC_ERROR_BUF: [usize; 128] = [0; 128];
static mut STC_TAG_BUF: [usize; 16] = [0; 16];
static mut STC_LINE_BUF: [usize; 16] = [0; 16];

unsafe fn psycho_check_stc_error(pbm: *mut pci_pbm_info) {
    let strbuf = &mut (*pbm).stc;
    if strbuf.strbuf_control == 0 { return; }
    let err_base = strbuf.strbuf_err_stat;
    let tag_base = strbuf.strbuf_tag_diag;
    let line_base = strbuf.strbuf_line_diag;
    spin_lock(&mut STC_BUF_LOCK);
    let control = upa_readq(strbuf.strbuf_control);
    upa_writeq(control | PSYCHO_STRBUF_CTRL_DENAB, strbuf.strbuf_control);
    for i in 0..128 { let val = upa_readq(err_base + i * 8); upa_writeq(0, err_base + i * 8); STC_ERROR_BUF[i] = val as usize; }
    for i in 0..16 {
        STC_TAG_BUF[i] = upa_readq(tag_base + i * 8) as usize;
        STC_LINE_BUF[i] = upa_readq(line_base + i * 8) as usize;
        upa_writeq(0, tag_base + i * 8); upa_writeq(0, line_base + i * 8);
    }
    upa_writeq(control, strbuf.strbuf_control);
    for i in 0..16 {
        let first = i * 8; let mut saw_error = 0;
        for j in first..first + 8 {
            let errval = STC_ERROR_BUF[j] as u64;
            if errval != 0 { saw_error += 1; printk(KERN_ERR, "%s: STC_ERR(%d)[wr(%d)rd(%d)]\n", (*pbm).name, j, if errval & PSYCHO_STCERR_WRITE != 0 {1} else {0}, if errval & PSYCHO_STCERR_READ != 0 {1} else {0}); }
        }
        if saw_error != 0 {
            let tagval = STC_TAG_BUF[i] as u64; let lineval = STC_LINE_BUF[i] as u64;
            printk(KERN_ERR, "%s: STC_TAG(%d)[PA(%016llx)VA(%08llx)V(%d)W(%d)]\n", (*pbm).name, i, (tagval & PSYCHO_STCTAG_PPN) >> 19, tagval & PSYCHO_STCTAG_VPN, if tagval & PSYCHO_STCTAG_VALID != 0 {1} else {0}, if tagval & PSYCHO_STCTAG_WRITE != 0 {1} else {0});
            printk(KERN_ERR, "%s: STC_LINE(%d)[LIDX(%llx)SP(%llx)LADDR(%llx)EP(%llx)V(%d)FOFN(%d)]\n", (*pbm).name, i, (lineval & PSYCHO_STCLINE_LINDX) >> 21, (lineval & PSYCHO_STCLINE_SPTR) >> 15, (lineval & PSYCHO_STCLINE_LADDR) >> 8, (lineval & PSYCHO_STCLINE_EPTR) >> 2, if lineval & PSYCHO_STCLINE_VALID != 0 {1} else {0}, if lineval & PSYCHO_STCLINE_FOFN != 0 {1} else {0});
        }
    }
    spin_unlock(&mut STC_BUF_LOCK);
}

const PSYCHO_IOMMU_TAG: usize = 0xa580;
const PSYCHO_IOMMU_DATA: usize = 0xa600;

unsafe fn psycho_record_iommu_tags_and_data(pbm: *mut pci_pbm_info, tag: *mut u64, data: *mut u64) {
    for i in 0..16 { let base = (*pbm).controller_regs; let off = i * 8; *tag.add(i) = upa_readq(base + PSYCHO_IOMMU_TAG + off); *data.add(i) = upa_readq(base + PSYCHO_IOMMU_DATA + off); upa_writeq(0, base + PSYCHO_IOMMU_TAG + off); upa_writeq(0, base + PSYCHO_IOMMU_DATA + off); }
}

const PSYCHO_IOMMU_TAG_ERRSTS: u64 = 0x3 << 23;
const PSYCHO_IOMMU_TAG_ERR: u64 = 0x1 << 22;
const PSYCHO_IOMMU_TAG_WRITE: u64 = 0x1 << 21;
const PSYCHO_IOMMU_TAG_STREAM: u64 = 0x1 << 20;
const PSYCHO_IOMMU_TAG_SIZE: u64 = 0x1 << 19;
const PSYCHO_IOMMU_TAG_VPAGE: u64 = 0x7ffff;
const PSYCHO_IOMMU_DATA_VALID: u64 = 1 << 30;
const PSYCHO_IOMMU_DATA_CACHE: u64 = 1 << 28;
const PSYCHO_IOMMU_DATA_PPAGE: u64 = 0xfffffff;

unsafe fn psycho_dump_iommu_tags_and_data(pbm: *mut pci_pbm_info, tag: *const u64, data: *const u64) {
    for i in 0..16 { let tag_val = *tag.add(i); if tag_val & PSYCHO_IOMMU_TAG_ERR == 0 { continue; } let data_val = *data.add(i); let type_str = match (tag_val & PSYCHO_IOMMU_TAG_ERRSTS) >> 23 { 0 => "Protection Error", 1 => "Invalid Error", 2 => "TimeOut Error", _ => "ECC Error" }; printk(KERN_ERR, "%s: IOMMU TAG(%d)[error(%s) wr(%d) str(%d) sz(%dK) vpg(%08llx)]\n", (*pbm).name, i, type_str, if tag_val & PSYCHO_IOMMU_TAG_WRITE != 0 {1} else {0}, if tag_val & PSYCHO_IOMMU_TAG_STREAM != 0 {1} else {0}, if tag_val & PSYCHO_IOMMU_TAG_SIZE != 0 {64} else {8}, (tag_val & PSYCHO_IOMMU_TAG_VPAGE) << IOMMU_PAGE_SHIFT); printk(KERN_ERR, "%s: IOMMU DATA(%d)[valid(%d) cache(%d) ppg(%016llx)]\n", (*pbm).name, i, if data_val & PSYCHO_IOMMU_DATA_VALID != 0 {1} else {0}, if data_val & PSYCHO_IOMMU_DATA_CACHE != 0 {1} else {0}, (data_val & PSYCHO_IOMMU_DATA_PPAGE) << IOMMU_PAGE_SHIFT); }
}

const PSYCHO_IOMMU_CTRL_XLTESTAT: u64 = 0x0000000006000000;
const PSYCHO_IOMMU_CTRL_XLTEERR: u64 = 0x0000000001000000;

pub unsafe fn psycho_check_iommu_error(pbm: *mut pci_pbm_info, _afsr: usize, _afar: usize, _type: psycho_error_type) {
    let iommu = (*pbm).iommu; let mut iommu_tag = [0u64; 16]; let mut iommu_data = [0u64; 16]; let flags = spin_lock_irqsave(&mut (*iommu).lock);
    let mut control = upa_readq((*iommu).iommu_control);
    if control & PSYCHO_IOMMU_CTRL_XLTEERR != 0 { control &= !PSYCHO_IOMMU_CTRL_XLTEERR; upa_writeq(control, (*iommu).iommu_control); let type_str = match (control & PSYCHO_IOMMU_CTRL_XLTESTAT) >> 25 { 0 => "Protection Error", 1 => "Invalid Error", 2 => "TimeOut Error", _ => "ECC Error" }; printk(KERN_ERR, "%s: IOMMU Error, type[%s]\n", (*pbm).name, type_str); psycho_record_iommu_tags_and_data(pbm, iommu_tag.as_mut_ptr(), iommu_data.as_mut_ptr()); psycho_dump_iommu_tags_and_data(pbm, iommu_tag.as_ptr(), iommu_data.as_ptr()); }
    psycho_check_stc_error(pbm); spin_unlock_irqrestore(&mut (*iommu).lock, flags);
}

const PSYCHO_PCIAFSR_PMA: u64 = 0x8000000000000000;
const PSYCHO_PCIAFSR_PTA: u64 = 0x4000000000000000;
const PSYCHO_PCIAFSR_PRTRY: u64 = 0x2000000000000000;
const PSYCHO_PCIAFSR_PPERR: u64 = 0x1000000000000000;
const PSYCHO_PCIAFSR_SMA: u64 = 0x0800000000000000;
const PSYCHO_PCIAFSR_STA: u64 = 0x0400000000000000;
const PSYCHO_PCIAFSR_SRTRY: u64 = 0x0200000000000000;
const PSYCHO_PCIAFSR_SPERR: u64 = 0x0100000000000000;
const PSYCHO_PCIAFSR_BMSK: u64 = 0x0000ffff00000000;
const PSYCHO_PCIAFSR_BLK: u64 = 0x0000000080000000;
const PSYCHO_PCIAFSR_MID: u64 = 0x000000003e000000;

pub unsafe fn psycho_pcierr_intr(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pbm = dev_id as *mut pci_pbm_info; let afsr = upa_readq((*pbm).pci_afsr); let afar = upa_readq((*pbm).pci_afar); let error_bits = afsr & (PSYCHO_PCIAFSR_PMA | PSYCHO_PCIAFSR_PTA | PSYCHO_PCIAFSR_PRTRY | PSYCHO_PCIAFSR_PPERR | PSYCHO_PCIAFSR_SMA | PSYCHO_PCIAFSR_STA | PSYCHO_PCIAFSR_SRTRY | PSYCHO_PCIAFSR_SPERR); if error_bits == 0 { return psycho_pcierr_intr_other(pbm); }
    upa_writeq(error_bits, (*pbm).pci_afsr); let primary = if error_bits & PSYCHO_PCIAFSR_PMA != 0 {"Master Abort"} else if error_bits & PSYCHO_PCIAFSR_PTA != 0 {"Target Abort"} else if error_bits & PSYCHO_PCIAFSR_PRTRY != 0 {"Excessive Retries"} else if error_bits & PSYCHO_PCIAFSR_PPERR != 0 {"Parity Error"} else {"???"}; printk(KERN_ERR, "%s: PCI Error, primary error type[%s]\n", (*pbm).name, primary); printk(KERN_ERR, "%s: bytemask[%04llx] UPA_MID[%02llx] was_block(%d)\n", (*pbm).name, (afsr & PSYCHO_PCIAFSR_BMSK) >> 32, (afsr & PSYCHO_PCIAFSR_MID) >> 25, if afsr & PSYCHO_PCIAFSR_BLK != 0 {1} else {0}); printk(KERN_ERR, "%s: PCI AFAR [%016llx]\n", (*pbm).name, afar); printk(KERN_ERR, "%s: PCI Secondary errors [", (*pbm).name); let mut reported = 0; for (bit, msg) in [(PSYCHO_PCIAFSR_SMA, "(Master Abort)"), (PSYCHO_PCIAFSR_STA, "(Target Abort)"), (PSYCHO_PCIAFSR_SRTRY, "(Excessive Retries)"), (PSYCHO_PCIAFSR_SPERR, "(Parity Error)")] { if afsr & bit != 0 { reported += 1; printk(msg); } } if reported == 0 { printk("(none)"); } printk("]\n");
    if error_bits & (PSYCHO_PCIAFSR_PTA | PSYCHO_PCIAFSR_STA) != 0 { psycho_check_iommu_error(pbm, afsr as usize, afar as usize, PCI_ERR); pci_scan_for_target_abort(pbm, (*pbm).pci_bus); } if error_bits & (PSYCHO_PCIAFSR_PMA | PSYCHO_PCIAFSR_SMA) != 0 { pci_scan_for_master_abort(pbm, (*pbm).pci_bus); } if error_bits & (PSYCHO_PCIAFSR_PPERR | PSYCHO_PCIAFSR_SPERR) != 0 { pci_scan_for_parity_error(pbm, (*pbm).pci_bus); } IRQ_HANDLED
}

unsafe fn psycho_pcierr_intr_other(pbm: *mut pci_pbm_info) -> irqreturn_t { let csr = upa_readq((*pbm).pci_csr); let bits = csr & (0x0000000800000000 | 0x0000000400000000); let mut ret = IRQ_NONE; if bits != 0 { upa_writeq(csr, (*pbm).pci_csr); ret = IRQ_HANDLED; } let addr = psycho_pci_config_mkaddr(pbm, (*pbm).pci_first_busno, 0, PCI_STATUS); let mut stat = 0u16; pci_config_read16(addr, &mut stat); if stat & (PCI_STATUS_PARITY | PCI_STATUS_SIG_TARGET_ABORT | PCI_STATUS_REC_TARGET_ABORT | PCI_STATUS_REC_MASTER_ABORT | PCI_STATUS_SIG_SYSTEM_ERROR) != 0 { pci_config_write16(addr, 0xffff); ret = IRQ_HANDLED; } ret }

unsafe fn psycho_iommu_flush(pbm: *mut pci_pbm_info) { for i in 0..16 { let off = i * 8; upa_writeq(0, (*pbm).controller_regs + PSYCHO_IOMMU_TAG + off); upa_writeq(0, (*pbm).controller_regs + PSYCHO_IOMMU_DATA + off); } }

pub unsafe fn psycho_iommu_init(pbm: *mut pci_pbm_info, tsbsize: i32, dvma_offset: u32, dma_mask: u32, write_complete_offset: usize) -> i32 { let iommu = (*pbm).iommu; (*iommu).iommu_control = (*pbm).controller_regs + 0x0200; (*iommu).iommu_tsbbase = (*pbm).controller_regs + 0x0208; (*iommu).iommu_flush = (*pbm).controller_regs + 0x0210; (*iommu).iommu_tags = (*pbm).controller_regs + PSYCHO_IOMMU_TAG; (*iommu).write_complete_reg = (*pbm).controller_regs + write_complete_offset; (*iommu).iommu_ctxflush = 0; let mut control = upa_readq((*iommu).iommu_control) | 2; upa_writeq(control, (*iommu).iommu_control); psycho_iommu_flush(pbm); let err = iommu_table_init(iommu, tsbsize * 1024 * 8, dvma_offset, dma_mask, (*pbm).numa_node); if err != 0 { return err; } upa_writeq(__pa((*iommu).page_table), (*iommu).iommu_tsbbase); control = upa_readq((*iommu).iommu_control) & !0x70004 | 1; match tsbsize { 64 => control |= 0x60000, 128 => control |= 0x70000, _ => return -22 }; upa_writeq(control, (*iommu).iommu_control); 0 }

pub unsafe fn psycho_pbm_init_common(pbm: *mut pci_pbm_info, op: *mut platform_device, chip_name: *const core::ffi::c_char, chip_type: i32) { let dp = (*(*op).dev).of_node; (*pbm).name = (*dp).full_name; (*pbm).numa_node = NUMA_NO_NODE; (*pbm).chip_type = chip_type; (*pbm).chip_version = of_getintprop_default(dp, "version#", 0); (*pbm).chip_revision = of_getintprop_default(dp, "module-revision#", 0); (*pbm).op = op; (*pbm).pci_ops = &sun4u_pci_ops; (*pbm).config_space_reg_bits = 8; (*pbm).index = pci_num_pbms; pci_num_pbms += 1; pci_get_pbm_props(pbm); pci_determine_mem_io_space(pbm); printk(KERN_INFO, "%s: %s PCI Bus Module ver[%x:%x]\n", (*pbm).name, chip_name, (*pbm).chip_version, (*pbm).chip_revision); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
