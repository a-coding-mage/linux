// SPDX-License-Identifier: GPL-2.0
/*
 * This file contains work-arounds for x86 and x86_64 platform bugs.
 *
 * C headers and PCI fixup-registration macros are supplied by the surrounding
 * kernel translation and are intentionally not reimplemented here.
 */

#[cfg(all(feature = "x86_io_apic", feature = "smp", feature = "pci"))]
unsafe fn quirk_intel_irqbalance(dev: *mut pci_dev) {
    let mut config: u8 = 0;
    let mut word: u16 = 0;

    if (*dev).revision > 0x9 { return; }
    pci_read_config_byte(dev, 0xf4, &mut config);
    pci_write_config_byte(dev, 0xf4, config | 0x2);
    pci_bus_read_config_word((*dev).bus, pci_devfn(8, 0), 0x4c, &mut word);

    if (word & (1 << 13)) == 0 {
        dev_info!(&(*dev).dev, "Intel E7520/7320/7525 detected; disabling irq balancing and affinity\n");
        noirqdebug_setup("");
        #[cfg(feature = "proc_fs")]
        { no_irq_affinity = 1; }
    }
    if (config & 0x2) == 0 { pci_write_config_byte(dev, 0xf4, config); }
}

#[cfg(feature = "hpet_timer")]
pub static mut force_hpet_address: usize = 0;

#[cfg(feature = "hpet_timer")]
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum ForceHpetResume { NoneForceHpetResume, OldIchForceHpetResume, IchForceHpetResume, Vt8237ForceHpetResume, NvidiaForceHpetResume, AtiForceHpetResume }

#[cfg(feature = "hpet_timer")]
static mut force_hpet_resume_type: ForceHpetResume = ForceHpetResume::NoneForceHpetResume;
#[cfg(feature = "hpet_timer")]
static mut rcba_base: *mut core::ffi::c_void = core::ptr::null_mut();
#[cfg(feature = "hpet_timer")]
static mut cached_dev: *mut pci_dev = core::ptr::null_mut();

#[cfg(feature = "hpet_timer")]
unsafe fn hpet_print_force_info() {
    printk!(KERN_INFO, "HPET not enabled in BIOS. You might try hpet=force boot option\n");
}

#[cfg(feature = "hpet_timer")]
unsafe fn ich_force_hpet_resume() {
    if force_hpet_address == 0 { return; }
    BUG_ON!(rcba_base.is_null());
    let mut val = readl(rcba_base.add(0x3404));
    if (val & 0x80) == 0 { writel(val | 0x80, rcba_base.add(0x3404)); }
    val = readl(rcba_base.add(0x3404));
    if (val & 0x80) == 0 { BUG!(); } else { printk!(KERN_DEBUG, "Force enabled HPET at resume\n"); }
}

#[cfg(feature = "hpet_timer")]
unsafe fn ich_force_enable_hpet(dev: *mut pci_dev) {
    if hpet_address != 0 || force_hpet_address != 0 { return; }
    let mut rcba = 0u32;
    pci_read_config_dword(dev, 0xF0, &mut rcba);
    rcba &= 0xFFFFC000;
    if rcba == 0 { dev_printk!(KERN_DEBUG, &(*dev).dev, "RCBA disabled; cannot force enable HPET\n"); return; }
    rcba_base = ioremap(rcba as usize, 0x4000);
    if rcba_base.is_null() { dev_printk!(KERN_DEBUG, &(*dev).dev, "ioremap failed; cannot force enable HPET\n"); return; }
    let mut val = readl(rcba_base.add(0x3404));
    if (val & 0x80) != 0 {
        val &= 0x3; force_hpet_address = 0xFED00000usize | ((val as usize) << 12);
        dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address);
        iounmap(rcba_base); return;
    }
    writel(val | 0x80, rcba_base.add(0x3404));
    val = readl(rcba_base.add(0x3404));
    if (val & 0x80) == 0 {
        force_hpet_address = 0; iounmap(rcba_base);
        dev_printk!(KERN_DEBUG, &(*dev).dev, "Failed to force enable HPET\n");
    } else {
        val &= 0x3; force_hpet_address = 0xFED00000usize | ((val as usize) << 12);
        force_hpet_resume_type = ForceHpetResume::IchForceHpetResume;
        dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address);
    }
}

// DECLARE_PCI_FIXUP_* registrations from the C source are emitted by the
// surrounding kernel integration for the corresponding functions and IDs.

#[cfg(feature = "hpet_timer")]
unsafe fn old_ich_force_hpet_resume() {
    if force_hpet_address == 0 || cached_dev.is_null() { return; }
    let mut gen_cntl = 0u32;
    let mut val;
    pci_read_config_dword(cached_dev, 0xD0, &mut gen_cntl);
    gen_cntl &= !(0x7 << 15); gen_cntl |= 0x4 << 15;
    pci_write_config_dword(cached_dev, 0xD0, gen_cntl);
    pci_read_config_dword(cached_dev, 0xD0, &mut gen_cntl);
    val = (gen_cntl >> 15) & 0x7;
    if val == 0x4 { printk!(KERN_DEBUG, "Force enabled HPET at resume\n"); } else { BUG!(); }
}

#[cfg(feature = "hpet_timer")]
unsafe fn old_ich_force_enable_hpet(dev: *mut pci_dev) {
    if hpet_address != 0 || force_hpet_address != 0 { return; }
    let mut gen_cntl = 0u32;
    pci_read_config_dword(dev, 0xD0, &mut gen_cntl);
    let mut val = (gen_cntl >> 15) & 0x7;
    if (val & 0x4) != 0 { val &= 0x3; force_hpet_address = 0xFED00000usize | ((val as usize) << 12); dev_printk!(KERN_DEBUG, &(*dev).dev, "HPET at 0x%lx\n", force_hpet_address); return; }
    gen_cntl &= !(0x7 << 15); gen_cntl |= 0x4 << 15;
    pci_write_config_dword(dev, 0xD0, gen_cntl); pci_read_config_dword(dev, 0xD0, &mut gen_cntl);
    val = (gen_cntl >> 15) & 0x7;
    if (val & 0x4) != 0 { val &= 0x3; force_hpet_address = 0xFED00000usize | ((val as usize) << 12); cached_dev = dev; force_hpet_resume_type = ForceHpetResume::OldIchForceHpetResume; dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address); }
    else { dev_printk!(KERN_DEBUG, &(*dev).dev, "Failed to force enable HPET\n"); }
}

#[cfg(feature = "hpet_timer")]
unsafe fn old_ich_force_enable_hpet_user(dev: *mut pci_dev) { if hpet_force_user { old_ich_force_enable_hpet(dev); } }

#[cfg(feature = "hpet_timer")]
unsafe fn vt8237_force_hpet_resume() {
    if force_hpet_address == 0 || cached_dev.is_null() { return; }
    let mut val = 0xfed00080u32; pci_write_config_dword(cached_dev, 0x68, val); pci_read_config_dword(cached_dev, 0x68, &mut val);
    if (val & 0x80) != 0 { printk!(KERN_DEBUG, "Force enabled HPET at resume\n"); } else { BUG!(); }
}

#[cfg(feature = "hpet_timer")]
unsafe fn vt8237_force_enable_hpet(dev: *mut pci_dev) {
    if hpet_address != 0 || force_hpet_address != 0 { return; }
    if !hpet_force_user { hpet_print_force_info(); return; }
    let mut val = 0u32; pci_read_config_dword(dev, 0x68, &mut val);
    if (val & 0x80) != 0 { force_hpet_address = (val & !0x3ff) as usize; dev_printk!(KERN_DEBUG, &(*dev).dev, "HPET at 0x%lx\n", force_hpet_address); return; }
    val = 0xfed00080; pci_write_config_dword(dev, 0x68, val); pci_read_config_dword(dev, 0x68, &mut val);
    if (val & 0x80) != 0 { force_hpet_address = (val & !0x3ff) as usize; cached_dev = dev; force_hpet_resume_type = ForceHpetResume::Vt8237ForceHpetResume; dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address); } else { dev_printk!(KERN_DEBUG, &(*dev).dev, "Failed to force enable HPET\n"); }
}

#[cfg(feature = "hpet_timer")]
unsafe fn ati_force_hpet_resume() { pci_write_config_dword(cached_dev, 0x14, 0xfed00000); printk!(KERN_DEBUG, "Force enabled HPET at resume\n"); }

#[cfg(feature = "hpet_timer")]
unsafe fn ati_ixp4x0_rev(dev: *mut pci_dev) -> u32 {
    let mut err = 0; let mut d = 0u32; let mut b = 0u8;
    err |= pci_read_config_byte(dev, 0xac, &mut b); b &= !(1 << 5); err |= pci_write_config_byte(dev, 0xac, b);
    err |= pci_read_config_dword(dev, 0x70, &mut d); d |= 1 << 8; err |= pci_write_config_dword(dev, 0x70, d); err |= pci_read_config_dword(dev, 0x8, &mut d); d &= 0xff;
    dev_printk!(KERN_DEBUG, &(*dev).dev, "SB4X0 revision 0x%x\n", d); WARN_ON_ONCE!(err != 0); d
}

#[cfg(feature = "hpet_timer")]
unsafe fn ati_force_enable_hpet(dev: *mut pci_dev) {
    if hpet_address != 0 || force_hpet_address != 0 { return; } if !hpet_force_user { hpet_print_force_info(); return; }
    let mut d = ati_ixp4x0_rev(dev); if d < 0x82 { return; }
    pci_write_config_dword(dev, 0x14, 0xfed00000); let mut val = 0; pci_read_config_dword(dev, 0x14, &mut val);
    outb(0x72, 0xcd6); let mut b = inb(0xcd7); b |= 1; outb(0x72, 0xcd6); outb(b, 0xcd7); outb(0x72, 0xcd6); b = inb(0xcd7); if (b & 1) == 0 { return; }
    pci_read_config_dword(dev, 0x64, &mut d); d |= 1 << 10; pci_write_config_dword(dev, 0x64, d); pci_read_config_dword(dev, 0x64, &mut d); if (d & (1 << 10)) == 0 { return; }
    force_hpet_address = val as usize; force_hpet_resume_type = ForceHpetResume::AtiForceHpetResume; cached_dev = dev; dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address);
}

#[cfg(feature = "hpet_timer")]
unsafe fn nvidia_force_hpet_resume() { pci_write_config_dword(cached_dev, 0x44, 0xfed00001); printk!(KERN_DEBUG, "Force enabled HPET at resume\n"); }
#[cfg(feature = "hpet_timer")]
unsafe fn nvidia_force_enable_hpet(dev: *mut pci_dev) { if hpet_address != 0 || force_hpet_address != 0 { return; } if !hpet_force_user { hpet_print_force_info(); return; } let mut val = 0; pci_write_config_dword(dev, 0x44, 0xfed00001); pci_read_config_dword(dev, 0x44, &mut val); force_hpet_address = (val & 0xfffffffe) as usize; force_hpet_resume_type = ForceHpetResume::NvidiaForceHpetResume; cached_dev = dev; dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address); }

#[cfg(feature = "hpet_timer")]
pub unsafe fn force_hpet_resume() { match force_hpet_resume_type { ForceHpetResume::IchForceHpetResume => ich_force_hpet_resume(), ForceHpetResume::OldIchForceHpetResume => old_ich_force_hpet_resume(), ForceHpetResume::Vt8237ForceHpetResume => vt8237_force_hpet_resume(), ForceHpetResume::NvidiaForceHpetResume => nvidia_force_hpet_resume(), ForceHpetResume::AtiForceHpetResume => ati_force_hpet_resume(), ForceHpetResume::NoneForceHpetResume => {} } }

#[cfg(feature = "hpet_timer")]
unsafe fn e6xx_force_enable_hpet(dev: *mut pci_dev) { if hpet_address != 0 || force_hpet_address != 0 { return; } force_hpet_address = 0xFED00000; force_hpet_resume_type = ForceHpetResume::NoneForceHpetResume; dev_printk!(KERN_DEBUG, &(*dev).dev, "Force enabled HPET at 0x%lx\n", force_hpet_address); }
#[cfg(feature = "hpet_timer")]
unsafe fn force_disable_hpet_msi(_unused: *mut pci_dev) { hpet_msi_disable = true; }

#[cfg(all(feature = "pci", feature = "numa"))]
unsafe fn quirk_amd_nb_node(dev: *mut pci_dev) {
    let devfn = pci_devfn(pci_slot((*dev).devfn), 0); let nb_ht = pci_get_slot((*dev).bus, devfn); if nb_ht.is_null() { return; }
    let mut val = 0u32; pci_read_config_dword(nb_ht, 0x60, &mut val); let node = pcibus_to_node((*dev).bus) | (val & 7); if node_online(node) { set_dev_node(&mut (*dev).dev, node); } pci_dev_put(nb_ht);
}

#[cfg(feature = "pci")]
unsafe fn amd_disable_seq_and_redirect_scrub(dev: *mut pci_dev) { let mut val = 0u32; pci_read_config_dword(dev, 0x58, &mut val); if (val & 0x1f) != 0 { val &= !0x1f; pci_write_config_dword(dev, 0x58, val); } pci_read_config_dword(dev, 0x5c, &mut val); if (val & BIT(0)) != 0 { val &= !BIT(0); pci_write_config_dword(dev, 0x5c, val); } }

#[cfg(feature = "pci")]
unsafe fn quirk_intel_brickland_xeon_ras_cap(pdev: *mut pci_dev) { let mut capid0 = 0; pci_read_config_dword(pdev, 0x84, &mut capid0); if (capid0 & 0x10) != 0 { enable_copy_mc_fragile(); } }
#[cfg(feature = "pci")]
unsafe fn quirk_intel_purley_xeon_ras_cap(pdev: *mut pci_dev) { let (mut capid0, mut capid5) = (0, 0); pci_read_config_dword(pdev, 0x84, &mut capid0); pci_read_config_dword(pdev, 0x98, &mut capid5); if (capid0 & 0xc0) == 0xc0 || (capid5 & 0x1e0) != 0 { enable_copy_mc_fragile(); } }

pub static mut x86_apple_machine: bool = false;
pub unsafe fn early_platform_quirks() { x86_apple_machine = dmi_match(DMI_SYS_VENDOR, "Apple Inc.") || dmi_match(DMI_SYS_VENDOR, "Apple Computer, Inc."); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
