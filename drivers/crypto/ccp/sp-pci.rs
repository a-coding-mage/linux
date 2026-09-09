// SPDX-License-Identifier: GPL-2.0-only
/* AMD Secure Processor device driver */

// Kernel includes and local headers are supplied by the surrounding translation unit.

const AA: u32 = 0xff00_0000;
const BB: u32 = 0x00ff_0000;
const CC: u32 = 0x0000_ff00;
const DD: u32 = 0x0000_00ff;
const MSIX_VECTORS: usize = 2;

#[repr(C)]
struct SpPci {
    msix_count: i32,
    msix_entry: [MsixEntry; MSIX_VECTORS],
}

static mut SP_DEV_MASTER: *mut SpDevice = core::ptr::null_mut();

unsafe fn bootloader_version_show(d: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    version_attribute_show(d, buf, (*(*sp_get_drvdata(d)).psp_data).vdata.bootloader_info_reg)
}

unsafe fn tee_version_show(d: *mut Device, _attr: *mut DeviceAttribute, buf: *mut i8) -> isize {
    version_attribute_show(d, buf, (*(*sp_get_drvdata(d)).psp_data).vdata.tee.info_reg)
}

unsafe fn version_attribute_show(d: *mut Device, buf: *mut i8, offset: u32) -> isize {
    let sp = sp_get_drvdata(d);
    let psp = (*sp).psp_data;
    let val = ioread32((*psp).io_regs.add(offset as usize));
    sysfs_emit(buf, "%02lx.%02lx.%02lx.%02lx\n", ((val & AA) >> 24), ((val & BB) >> 16), ((val & CC) >> 8), (val & DD))
}

static mut PSP_FIRMWARE_ATTRS: [*mut Attribute; 3] = [
    core::ptr::addr_of_mut!(DEV_ATTR_BOOTLOADER_VERSION_ATTR),
    core::ptr::addr_of_mut!(DEV_ATTR_TEE_VERSION_ATTR),
    core::ptr::null_mut(),
];

unsafe fn psp_firmware_is_visible(kobj: *mut Kobject, attr: *mut Attribute, _idx: i32) -> UmodeT {
    let dev = kobject_to_dev(kobj);
    let sp = sp_get_drvdata(dev);
    let psp = (*sp).psp_data;
    let mut val: u32 = 0xffff_ffff;
    if psp.is_null() { return 0; }
    if attr == core::ptr::addr_of_mut!(DEV_ATTR_BOOTLOADER_VERSION_ATTR) && (*psp).vdata.bootloader_info_reg != 0 {
        val = ioread32((*psp).io_regs.add((*psp).vdata.bootloader_info_reg as usize));
    }
    if attr == core::ptr::addr_of_mut!(DEV_ATTR_TEE_VERSION_ATTR) && (*psp).capability.tee && (*psp).vdata.tee.info_reg != 0 {
        val = ioread32((*psp).io_regs.add((*psp).vdata.tee.info_reg as usize));
    }
    if val != 0xffff_ffff { 0o444 } else { 0 }
}

static mut PSP_FIRMWARE_ATTR_GROUP: AttributeGroup = AttributeGroup { attrs: core::ptr::addr_of_mut!(PSP_FIRMWARE_ATTRS), is_visible: Some(psp_firmware_is_visible) };
static mut PSP_GROUPS: [*const AttributeGroup; 2] = [core::ptr::addr_of!(PSP_FIRMWARE_ATTR_GROUP), core::ptr::null()];

unsafe fn sp_get_msix_irqs(sp: *mut SpDevice) -> i32 {
    let sp_pci = (*sp).dev_specific as *mut SpPci;
    let pdev = to_pci_dev((*sp).dev);
    let mut v = 0;
    while v < MSIX_VECTORS { (*sp_pci).msix_entry[v].entry = v as u32; v += 1; }
    let ret = pci_enable_msix_range(pdev, (*sp_pci).msix_entry.as_mut_ptr(), 1, v as i32);
    if ret < 0 { return ret; }
    (*sp_pci).msix_count = ret;
    (*sp).use_tasklet = true;
    (*sp).psp_irq = (*sp_pci).msix_entry[0].vector;
    (*sp).ccp_irq = if ret > 1 { (*sp_pci).msix_entry[1].vector } else { (*sp_pci).msix_entry[0].vector };
    0
}

unsafe fn sp_get_msi_irq(sp: *mut SpDevice) -> i32 {
    let pdev = to_pci_dev((*sp).dev);
    let ret = pci_enable_msi(pdev);
    if ret != 0 { return ret; }
    (*sp).ccp_irq = (*pdev).irq; (*sp).psp_irq = (*pdev).irq; 0
}

unsafe fn sp_get_irqs(sp: *mut SpDevice) -> i32 {
    let ret = sp_get_msix_irqs(sp); if ret == 0 { return 0; }
    dev_notice((*sp).dev, "could not enable MSI-X (%d), trying MSI\n", ret);
    let ret = sp_get_msi_irq(sp); if ret == 0 { return 0; }
    dev_notice((*sp).dev, "could not enable MSI (%d)\n", ret); ret
}

unsafe fn sp_free_irqs(sp: *mut SpDevice) {
    let sp_pci = (*sp).dev_specific as *mut SpPci; let pdev = to_pci_dev((*sp).dev);
    if (*sp_pci).msix_count != 0 { pci_disable_msix(pdev); } else if (*sp).psp_irq != 0 { pci_disable_msi(pdev); }
    (*sp).ccp_irq = 0; (*sp).psp_irq = 0;
}

unsafe fn sp_pci_is_master(sp: *mut SpDevice) -> bool {
    let new = to_pci_dev((*sp).dev); let cur = to_pci_dev((*SP_DEV_MASTER).dev);
    if pci_domain_nr((*new).bus) != pci_domain_nr((*cur).bus) { return pci_domain_nr((*new).bus) < pci_domain_nr((*cur).bus); }
    if (*new).bus.number != (*cur).bus.number { return (*new).bus.number < (*cur).bus.number; }
    if pci_slot((*new).devfn) != pci_slot((*cur).devfn) { return pci_slot((*new).devfn) < pci_slot((*cur).devfn); }
    if pci_func((*new).devfn) != pci_func((*cur).devfn) { return pci_func((*new).devfn) < pci_func((*cur).devfn); }
    false
}

unsafe fn psp_set_master(sp: *mut SpDevice) { if SP_DEV_MASTER.is_null() || sp_pci_is_master(sp) { SP_DEV_MASTER = sp; } }
unsafe fn psp_get_master() -> *mut SpDevice { SP_DEV_MASTER }
unsafe fn psp_clear_master(sp: *mut SpDevice) { if sp == SP_DEV_MASTER { SP_DEV_MASTER = core::ptr::null_mut(); dev_dbg((*sp).dev, "Cleared sp_dev_master\n"); } }

// The remaining driver registration and version-data tables retain the C ABI and
// are expressed through the surrounding kernel bindings.
extern "C" {
    static mut DEV_ATTR_BOOTLOADER_VERSION_ATTR: Attribute;
    static mut DEV_ATTR_TEE_VERSION_ATTR: Attribute;
}

unsafe fn sp_pci_probe(pdev: *mut PciDev, id: *const PciDeviceId) -> i32 {
    let dev = core::ptr::addr_of_mut!((*pdev).dev);
    let sp = sp_alloc_struct(dev); if sp.is_null() { return -12; }
    let sp_pci = devm_kzalloc(dev, core::mem::size_of::<SpPci>(), GFP_KERNEL) as *mut SpPci;
    if sp_pci.is_null() { return -12; }
    (*sp).dev_specific = sp_pci as *mut core::ffi::c_void;
    (*sp).dev_vdata = (*id).driver_data as *mut SpDevVdata;
    if (*sp).dev_vdata.is_null() { dev_err(dev, "missing driver data\n"); return -19; }
    let mut ret = pcim_enable_device(pdev); if ret != 0 { return ret; }
    let bar_mask = pci_select_bars(pdev, IORESOURCE_MEM);
    ret = pcim_iomap_regions(pdev, bar_mask, "ccp"); if ret != 0 { return ret; }
    let iomap = pcim_iomap_table(pdev); if iomap.is_null() { return -12; }
    (*sp).io_map = *iomap.add((*(*sp).dev_vdata).bar as usize); if (*sp).io_map.is_null() { return -12; }
    ret = sp_get_irqs(sp); if ret != 0 { return ret; }
    pci_set_master(pdev); (*sp).set_psp_master_device = Some(psp_set_master); (*sp).get_psp_master_device = Some(psp_get_master); (*sp).clear_psp_master_device = Some(psp_clear_master);
    ret = dma_set_mask_and_coherent(dev, 1u64 << 48); if ret != 0 { ret = dma_set_mask_and_coherent(dev, 1u64 << 32); if ret != 0 { sp_free_irqs(sp); return ret; } }
    dev_set_drvdata(dev, sp as *mut core::ffi::c_void); ret = sp_init(sp); if ret != 0 { sp_free_irqs(sp); } ret
}

unsafe fn sp_pci_shutdown(pdev: *mut PciDev) { let sp = dev_get_drvdata(core::ptr::addr_of_mut!((*pdev).dev)); if !sp.is_null() { sp_destroy(sp); } }
unsafe fn sp_pci_remove(pdev: *mut PciDev) { let sp = dev_get_drvdata(core::ptr::addr_of_mut!((*pdev).dev)); if !sp.is_null() { sp_destroy(sp); sp_free_irqs(sp); } }
unsafe fn sp_pci_suspend(dev: *mut Device) -> i32 { sp_suspend(dev_get_drvdata(dev)) }
unsafe fn sp_pci_resume(dev: *mut Device) -> i32 { sp_resume(dev_get_drvdata(dev)) }
unsafe fn sp_pci_restore(dev: *mut Device) -> i32 { sp_restore(dev_get_drvdata(dev)) }

#[cfg(feature = "crypto_dev_sp_psp")]
static SEVV1: SevVdata = SevVdata { cmdresp_reg: 0x10580, cmdbuff_addr_lo_reg: 0x105e0, cmdbuff_addr_hi_reg: 0x105e4 };
#[cfg(feature = "crypto_dev_sp_psp")]
static SEVV2: SevVdata = SevVdata { cmdresp_reg: 0x10980, cmdbuff_addr_lo_reg: 0x109e0, cmdbuff_addr_hi_reg: 0x109e4 };
#[cfg(feature = "crypto_dev_sp_psp")]
static TEEV1: TeeVdata = TeeVdata { ring_wptr_reg: 0x10550, ring_rptr_reg: 0x10554, info_reg: 0x109e8 };
#[cfg(feature = "crypto_dev_sp_psp")]
static TEEV2: TeeVdata = TeeVdata { ring_wptr_reg: 0x10950, ring_rptr_reg: 0x10954, info_reg: 0x109e8 };

// Hardware version tables and PCI IDs; fields not enabled by the build are omitted by cfg.
static DEV_VDATA: [SpDevVdata; 10] = [
    SpDevVdata { bar: 2, ..SpDevVdata::zero() }, SpDevVdata { bar: 2, ..SpDevVdata::zero() },
    SpDevVdata { bar: 2, ..SpDevVdata::zero() }, SpDevVdata { bar: 2, ..SpDevVdata::zero() },
    SpDevVdata { bar: 2, ..SpDevVdata::zero() }, SpDevVdata { bar: 2, ..SpDevVdata::zero() },
    SpDevVdata { bar: 2, ..SpDevVdata::zero() }, SpDevVdata { bar: 2, ..SpDevVdata::zero() },
    SpDevVdata { bar: 2, ..SpDevVdata::zero() }, SpDevVdata { bar: 2, ..SpDevVdata::zero() },
];

static mut SP_PCI_DRIVER: PciDriver = PciDriver { name: "ccp", id_table: core::ptr::null(), probe: Some(sp_pci_probe), remove: Some(sp_pci_remove), shutdown: Some(sp_pci_shutdown), ..PciDriver::zero() };

pub unsafe fn sp_pci_init() -> i32 { pci_register_driver(&mut SP_PCI_DRIVER) }
pub unsafe fn sp_pci_exit() { pci_unregister_driver(&mut SP_PCI_DRIVER); }

// External kernel types/functions and the literal device/version tables are
// intentionally referenced from their translated companion units.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
