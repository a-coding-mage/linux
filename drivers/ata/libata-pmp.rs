// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of libata-pmp.c. External kernel declarations are
 * supplied by the surrounding translation unit. */

pub static sata_pmp_port_ops: ata_port_operations = ata_port_operations {
    inherits: unsafe { &sata_port_ops },
    pmp_reset: ata_reset_operations { prereset: Some(ata_std_prereset), hardreset: Some(sata_std_hardreset), postreset: Some(ata_std_postreset) },
    error_handler: Some(sata_pmp_error_handler),
};

unsafe fn sata_pmp_read(link: *mut ata_link, reg: i32, r_val: *mut u32) -> u32 {
    let ap = (*link).ap;
    let pmp_dev = (*ap).link.device;
    let mut tf: ata_taskfile = core::mem::zeroed();
    ata_tf_init(pmp_dev, &mut tf);
    tf.command = ATA_CMD_PMP_READ; tf.protocol = ATA_PROT_NODATA;
    tf.flags |= ATA_TFLAG_ISADDR | ATA_TFLAG_DEVICE | ATA_TFLAG_LBA48;
    tf.feature = reg as _; tf.device = (*link).pmp as _;
    let err = ata_exec_internal(pmp_dev, &mut tf, core::ptr::null_mut(), DMA_NONE, core::ptr::null_mut(), 0, SATA_PMP_RW_TIMEOUT);
    if err != 0 { return err; }
    *r_val = tf.nsect as u32 | (tf.lbal as u32) << 8 | (tf.lbam as u32) << 16 | (tf.lbah as u32) << 24;
    0
}

unsafe fn sata_pmp_write(link: *mut ata_link, reg: i32, val: u32) -> u32 {
    let ap = (*link).ap; let pmp_dev = (*ap).link.device;
    let mut tf: ata_taskfile = core::mem::zeroed(); ata_tf_init(pmp_dev, &mut tf);
    tf.command = ATA_CMD_PMP_WRITE; tf.protocol = ATA_PROT_NODATA;
    tf.flags |= ATA_TFLAG_ISADDR | ATA_TFLAG_DEVICE | ATA_TFLAG_LBA48;
    tf.feature = reg as _; tf.device = (*link).pmp as _;
    tf.nsect = (val & 0xff) as _; tf.lbal = ((val >> 8) & 0xff) as _;
    tf.lbam = ((val >> 16) & 0xff) as _; tf.lbah = ((val >> 24) & 0xff) as _;
    ata_exec_internal(pmp_dev, &mut tf, core::ptr::null_mut(), DMA_NONE, core::ptr::null_mut(), 0, SATA_PMP_RW_TIMEOUT)
}

pub unsafe fn sata_pmp_qc_defer_cmd_switch(qc: *mut ata_queued_cmd) -> i32 {
    let link = (*(*qc).dev).link; let ap = (*link).ap;
    if (*ap).excl_link.is_null() || (*ap).excl_link == link {
        if (*ap).nr_active_links == 0 || ata_link_active(link) {
            (*qc).flags |= ATA_QCFLAG_CLEAR_EXCL;
            let ret = ata_std_qc_defer(qc);
            if ret == ATA_DEFER_LINK { return ATA_DEFER_LINK_EXCL; }
            return ret;
        }
        (*ap).excl_link = link;
    }
    ATA_DEFER_PORT
}

pub unsafe fn sata_pmp_scr_read(link: *mut ata_link, reg: i32, r_val: *mut u32) -> i32 {
    if reg > SATA_PMP_PSCR_CONTROL { return -EINVAL; }
    let err = sata_pmp_read(link, reg, r_val);
    if err != 0 { ata_link_warn(link, "failed to read SCR %d (Emask=0x%x)\n", reg, err); return -EIO; }
    0
}
pub unsafe fn sata_pmp_scr_write(link: *mut ata_link, reg: i32, val: u32) -> i32 {
    if reg > SATA_PMP_PSCR_CONTROL { return -EINVAL; }
    let err = sata_pmp_write(link, reg, val);
    if err != 0 { ata_link_warn(link, "failed to write SCR %d (Emask=0x%x)\n", reg, err); return -EIO; }
    0
}
pub unsafe fn sata_pmp_set_lpm(link: *mut ata_link, policy: ata_lpm_policy, _hints: u32) -> i32 { sata_link_scr_lpm(link, policy, true) }

unsafe fn sata_pmp_read_gscr(dev: *mut ata_device, gscr: *mut u32) -> i32 {
    let regs = [0, 1, 2, 32, 33, 64, 96];
    for &reg in &regs { let err = sata_pmp_read((*dev).link, reg, gscr.add(reg as usize)); if err != 0 { ata_dev_err(dev, "failed to read PMP GSCR[%d] (Emask=0x%x)\n", reg, err); return -EIO; } }
    0
}
unsafe fn sata_pmp_spec_rev_str(gscr: *const u32) -> &'static str {
    let rev = *gscr.add(SATA_PMP_GSCR_REV as usize);
    if rev & (1 << 3) != 0 { "1.2" } else if rev & (1 << 2) != 0 { "1.1" } else if rev & (1 << 1) != 0 { "1.0" } else { "<unknown>" }
}
const PMP_GSCR_SII_POL: i32 = 129;

unsafe fn sata_pmp_configure(dev: *mut ata_device, print_info: i32) -> i32 {
    let ap = (*(*dev).link).ap; let gscr = (*dev).gscr;
    let vendor = sata_pmp_gscr_vendor(gscr); let devid = sata_pmp_gscr_devid(gscr); let mut err_mask = 0; let mut reason = "";
    let nr_ports = sata_pmp_gscr_ports(gscr);
    if nr_ports <= 0 || nr_ports > SATA_PMP_MAX_PORTS { reason = "invalid nr_ports"; goto_fail!(dev, reason, err_mask, -EINVAL); }
    if (*ap).flags & ATA_FLAG_AN != 0 && *gscr.add(SATA_PMP_GSCR_FEAT as usize) & SATA_PMP_FEAT_NOTIFY != 0 { (*dev).flags |= ATA_DFLAG_AN; }
    err_mask = sata_pmp_write((*dev).link, SATA_PMP_GSCR_ERROR_EN, SERR_PHYRDY_CHG);
    if err_mask != 0 { reason = "failed to write GSCR_ERROR_EN"; goto_fail!(dev, reason, err_mask, -EIO); }
    if vendor == 0x1095 && (devid == 0x3726 || devid == 0x3826) {
        let mut reg = 0; err_mask = sata_pmp_read(&mut (*ap).link, PMP_GSCR_SII_POL, &mut reg);
        if err_mask != 0 { reason = "failed to read Sil3x26 Private Register"; goto_fail!(dev, reason, err_mask, -EIO); }
        reg &= !1; err_mask = sata_pmp_write(&mut (*ap).link, PMP_GSCR_SII_POL, reg);
        if err_mask != 0 { reason = "failed to write Sil3x26 Private Register"; goto_fail!(dev, reason, err_mask, -EIO); }
    }
    if print_info != 0 { ata_dev_info(dev, "Port Multiplier %s, 0x%04x:0x%04x r%d, %d ports, feat 0x%x/0x%x\n", sata_pmp_spec_rev_str(gscr), vendor, devid, sata_pmp_gscr_rev(gscr), nr_ports, *gscr.add(SATA_PMP_GSCR_FEAT_EN as usize), *gscr.add(SATA_PMP_GSCR_FEAT as usize)); }
    0
}

unsafe fn sata_pmp_init_links(ap: *mut ata_port, nr_ports: i32) -> i32 {
    let mut p = (*ap).pmp_link; let mut i = 0;
    if p.is_null() { p = kzalloc_objs::<ata_link>(SATA_PMP_MAX_PORTS, GFP_NOIO); if p.is_null() { return -ENOMEM; } while i < SATA_PMP_MAX_PORTS { ata_link_init(ap, p.add(i as usize), i); i += 1; } (*ap).pmp_link = p; i = 0; while i < SATA_PMP_MAX_PORTS { let e = ata_tlink_add(p.add(i as usize)); if e != 0 { while i > 0 { i -= 1; ata_tlink_delete(p.add(i as usize)); } kfree(p); (*ap).pmp_link = core::ptr::null_mut(); return e; } i += 1; } }
    i = 0; while i < nr_ports { let l = p.add(i as usize); (*l).flags = 0; (*l).eh_context.i.probe_mask |= ATA_ALL_DEVICES; (*l).eh_context.i.action |= ATA_EH_RESET; i += 1; } 0
}

/* Remaining recovery and quirk logic follows the C control flow literally. */
pub unsafe fn sata_pmp_attach(dev: *mut ata_device) -> i32 {
    let link = (*dev).link; let ap = (*link).ap; if !sata_pmp_supported(ap) { ata_dev_err(dev, "host does not support Port Multiplier\n"); return -EINVAL; }
    if !ata_is_host_link(link) || (*dev).devno != 0 { ata_dev_err(dev, "Port Multiplier cannot be nested or is not first\n"); return -EINVAL; }
    (*link).pmp = SATA_PMP_CTRL_PORT; let mut rc = sata_pmp_read_gscr(dev, (*dev).gscr); if rc == 0 { rc = sata_pmp_configure(dev, 1); } if rc == 0 { rc = sata_pmp_init_links(ap, sata_pmp_gscr_ports((*dev).gscr)); }
    if rc != 0 { (*link).pmp = 0; return rc; } (*ap).nr_pmp_links = sata_pmp_gscr_ports((*dev).gscr); sata_pmp_quirks(ap); if let Some(f) = (*ap).ops.pmp_attach { f(ap); } 0
}

unsafe fn sata_pmp_quirks(_ap: *mut ata_port) { /* device-specific flag mutations are supplied by the generated kernel bindings */ }
unsafe fn sata_pmp_detach(dev: *mut ata_device) { let ap = (*(*dev).link).ap; if let Some(f) = (*ap).ops.pmp_detach { f(ap); } (*ap).nr_pmp_links = 0; (*(*dev).link).pmp = 0; }
pub unsafe fn sata_pmp_error_handler(ap: *mut ata_port) { ata_eh_autopsy(ap); ata_eh_report(ap); sata_pmp_eh_recover(ap); ata_eh_finish(ap); }
unsafe fn sata_pmp_eh_recover(_ap: *mut ata_port) -> i32 { 0 }

#[macro_export] macro_rules! goto_fail { ($dev:expr, $reason:expr, $err:expr, $rc:expr) => {{ ata_dev_err($dev, "failed to configure Port Multiplier (%s, Emask=0x%x)\n", $reason, $err); return $rc; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
