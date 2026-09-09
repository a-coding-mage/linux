// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Device Feature List (DFL) PCIe device
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Zhang Yi <Yi.Z.Zhang@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Henry Mitchel <henry.mitchel@intel.com>
 */

// Kernel and local dependency declarations are supplied by the surrounding build.

const DRV_VERSION: *const u8 = b"0.8\0".as_ptr();
const DRV_NAME: *const u8 = b"dfl-pci\0".as_ptr();
const PCI_VSEC_ID_INTEL_DFLS: u32 = 0x43;
const PCI_VNDR_DFLS_CNT: u32 = 0x8;
const PCI_VNDR_DFLS_RES: u32 = 0xc;
const PCI_VNDR_DFLS_RES_BAR_MASK: u32 = 0x7;
const PCI_VNDR_DFLS_RES_OFF_MASK: u32 = 0xfffffff8;

#[repr(C)]
struct cci_drvdata {
    cdev: *mut dfl_fpga_cdev,
}

unsafe fn cci_pci_alloc_irq(pcidev: *mut pci_dev) -> i32 {
    let nvec = pci_msix_vec_count(pcidev);
    if nvec <= 0 {
        dev_dbg(&mut (*pcidev).dev, "fpga interrupt not supported\n");
        return 0;
    }
    let ret = pci_alloc_irq_vectors(pcidev, nvec, nvec, PCI_IRQ_MSIX);
    if ret < 0 { return ret; }
    nvec
}

unsafe fn cci_pci_free_irq(pcidev: *mut pci_dev) { pci_free_irq_vectors(pcidev); }

const PCIE_DEVICE_ID_PF_INT_5_X: u16 = 0xBCBD;
const PCIE_DEVICE_ID_PF_INT_6_X: u16 = 0xBCC0;
const PCIE_DEVICE_ID_PF_DSC_1_X: u16 = 0x09C4;
const PCIE_DEVICE_ID_INTEL_PAC_N3000: u16 = 0x0B30;
const PCIE_DEVICE_ID_INTEL_PAC_D5005: u16 = 0x0B2B;
const PCIE_DEVICE_ID_SILICOM_PAC_N5010: u16 = 0x1000;
const PCIE_DEVICE_ID_SILICOM_PAC_N5011: u16 = 0x1001;
const PCIE_DEVICE_ID_INTEL_DFL: u16 = 0xbcce;
const PCIE_SUBDEVICE_ID_INTEL_D5005: u16 = 0x138d;
const PCIE_SUBDEVICE_ID_INTEL_N6000: u16 = 0x1770;
const PCIE_SUBDEVICE_ID_INTEL_N6001: u16 = 0x1771;
const PCIE_SUBDEVICE_ID_INTEL_C6100: u16 = 0x17d4;
const PCIE_DEVICE_ID_VF_INT_5_X: u16 = 0xBCBF;
const PCIE_DEVICE_ID_VF_INT_6_X: u16 = 0xBCC1;
const PCIE_DEVICE_ID_VF_DSC_1_X: u16 = 0x09C5;
const PCIE_DEVICE_ID_INTEL_PAC_D5005_VF: u16 = 0x0B2C;
const PCIE_DEVICE_ID_INTEL_DFL_VF: u16 = 0xbccf;

// PCI device table; PCI_DEVICE and PCI_DEVICE_SUB expand to the platform's repr(C) entries.
static mut cci_pcie_id_tbl: [pci_device_id; 19] = [
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_PF_INT_5_X),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_VF_INT_5_X),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_PF_INT_6_X),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_VF_INT_6_X),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_PF_DSC_1_X),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_VF_DSC_1_X),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_PAC_N3000),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_PAC_D5005),
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_PAC_D5005_VF),
    PCI_DEVICE(PCI_VENDOR_ID_SILICOM_DENMARK, PCIE_DEVICE_ID_SILICOM_PAC_N5010),
    PCI_DEVICE(PCI_VENDOR_ID_SILICOM_DENMARK, PCIE_DEVICE_ID_SILICOM_PAC_N5011),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_D5005),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6000),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL_VF, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6000),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6001),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL_VF, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_N6001),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_C6100),
    PCI_DEVICE_SUB(PCI_VENDOR_ID_INTEL, PCIE_DEVICE_ID_INTEL_DFL_VF, PCI_VENDOR_ID_INTEL, PCIE_SUBDEVICE_ID_INTEL_C6100),
    pci_device_id { ..zeroed() },
];

// The remaining routines retain the kernel driver's original implementation and call graph.
// External kernel symbols are intentionally left as declarations.

unsafe fn cci_init_drvdata(pcidev: *mut pci_dev) -> i32 {
    let drvdata = devm_kzalloc(&mut (*pcidev).dev, core::mem::size_of::<cci_drvdata>(), GFP_KERNEL) as *mut cci_drvdata;
    if drvdata.is_null() { return -ENOMEM; }
    pci_set_drvdata(pcidev, drvdata as *mut _); 0
}

unsafe fn cci_remove_feature_devs(pcidev: *mut pci_dev) {
    let drvdata = pci_get_drvdata(pcidev) as *mut cci_drvdata;
    dfl_fpga_feature_devs_remove((*drvdata).cdev); cci_pci_free_irq(pcidev);
}

unsafe fn cci_pci_create_irq_table(pcidev: *mut pci_dev, nvec: u32) -> *mut i32 {
    let table = kzalloc_objs::<i32>(nvec);
    if table.is_null() { return table; }
    for i in 0..nvec { *table.add(i as usize) = pci_irq_vector(pcidev, i); }
    table
}

// Full DFL discovery logic is expressed below with the same branches and operations as C.
unsafe fn find_dfls_by_vsec(pcidev: *mut pci_dev, info: *mut dfl_fpga_enum_info) -> i32 {
    let voff = pci_find_vsec_capability(pcidev, PCI_VENDOR_ID_INTEL, PCI_VSEC_ID_INTEL_DFLS);
    if voff == 0 { dev_dbg(&mut (*pcidev).dev, "%s no DFL VSEC found\n", "find_dfls_by_vsec"); return -ENODEV; }
    let mut dfl_cnt = 0u32; pci_read_config_dword(pcidev, voff + PCI_VNDR_DFLS_CNT, &mut dfl_cnt);
    if dfl_cnt > PCI_STD_NUM_BARS { return -EINVAL; }
    let mut off = voff + PCI_VNDR_DFLS_RES; if off + dfl_cnt * 4 > PCI_CFG_SPACE_EXP_SIZE { return -EINVAL; }
    let mut bars = 0u32;
    for _ in 0..dfl_cnt { let mut res = u32::MAX; pci_read_config_dword(pcidev, off, &mut res); off += 4;
        let bir = res & PCI_VNDR_DFLS_RES_BAR_MASK; if bir >= PCI_STD_NUM_BARS || (bars & (1 << bir)) != 0 { return -EINVAL; } bars |= 1 << bir;
        let mut len = pci_resource_len(pcidev, bir); let offset = res & PCI_VNDR_DFLS_RES_OFF_MASK; if offset as u64 >= len { return -EINVAL; } len -= offset as u64;
        dfl_fpga_enum_info_add_dfl(info, pci_resource_start(pcidev, bir) + offset as u64, len);
    } 0
}

unsafe fn find_dfls_by_default(pcidev: *mut pci_dev, info: *mut dfl_fpga_enum_info) -> i32 {
    let base = pcim_iomap_region(pcidev, 0, DRV_NAME); if IS_ERR(base) { return PTR_ERR(base); }
    let mut ret = 0; let mut start; let mut len;
    if dfl_feature_is_fme(base) {
        start = pci_resource_start(pcidev, 0); len = pci_resource_len(pcidev, 0); dfl_fpga_enum_info_add_dfl(info, start, len);
        let v = readq(base.add(FME_HDR_CAP as usize)); let port_num = FIELD_GET(FME_CAP_NUM_PORTS, v); WARN_ON(port_num > MAX_DFL_FPGA_PORT_NUM);
        for i in 0..port_num { let v = readq(base.add(FME_HDR_PORT_OFST(i) as usize)); if (v & FME_PORT_OFST_IMP) == 0 { continue; }
            let bar = FIELD_GET(FME_PORT_OFST_BAR_ID, v); let offset = FIELD_GET(FME_PORT_OFST_DFH_OFST, v);
            if bar == FME_PORT_OFST_BAR_SKIP { continue; } else if bar >= PCI_STD_NUM_BARS { ret = -EINVAL; break; }
            start = pci_resource_start(pcidev, bar) + offset; len = pci_resource_len(pcidev, bar) - offset; dfl_fpga_enum_info_add_dfl(info, start, len);
        }
    } else if dfl_feature_is_port(base) { start = pci_resource_start(pcidev, 0); len = pci_resource_len(pcidev, 0); dfl_fpga_enum_info_add_dfl(info, start, len); } else { ret = -ENODEV; }
    pcim_iounmap_region(pcidev, 0); ret
}

unsafe fn cci_enumerate_feature_devs(pcidev: *mut pci_dev) -> i32 {
    let drvdata = pci_get_drvdata(pcidev) as *mut cci_drvdata; let info = dfl_fpga_enum_info_alloc(&mut (*pcidev).dev); if info.is_null() { return -ENOMEM; }
    let nvec = cci_pci_alloc_irq(pcidev); if nvec < 0 { dfl_fpga_enum_info_free(info); return nvec; }
    if nvec != 0 { let table = cci_pci_create_irq_table(pcidev, nvec as u32); if table.is_null() { cci_pci_free_irq(pcidev); dfl_fpga_enum_info_free(info); return -ENOMEM; } let ret = dfl_fpga_enum_info_add_irq(info, nvec as u32, table); kfree(table as *mut _); if ret != 0 { cci_pci_free_irq(pcidev); dfl_fpga_enum_info_free(info); return ret; } }
    let mut ret = find_dfls_by_vsec(pcidev, info); if ret == -ENODEV { ret = find_dfls_by_default(pcidev, info); }
    if ret == 0 { let cdev = dfl_fpga_feature_devs_enumerate(info); if IS_ERR(cdev) { ret = PTR_ERR(cdev); } else { (*drvdata).cdev = cdev; } }
    if ret != 0 { cci_pci_free_irq(pcidev); } dfl_fpga_enum_info_free(info); ret
}

unsafe fn cci_pci_probe(pcidev: *mut pci_dev, _pcidevid: *const pci_device_id) -> i32 {
    let mut ret = pcim_enable_device(pcidev); if ret < 0 { return ret; } pci_set_master(pcidev);
    ret = dma_set_mask_and_coherent(&mut (*pcidev).dev, DMA_BIT_MASK(64)); if ret != 0 { ret = dma_set_mask_and_coherent(&mut (*pcidev).dev, DMA_BIT_MASK(32)); } if ret != 0 { return ret; }
    ret = cci_init_drvdata(pcidev); if ret != 0 { return ret; } cci_enumerate_feature_devs(pcidev)
}

unsafe fn cci_pci_sriov_configure(pcidev: *mut pci_dev, num_vfs: i32) -> i32 {
    let cdev = (*(pci_get_drvdata(pcidev) as *mut cci_drvdata)).cdev;
    if num_vfs == 0 { pci_disable_sriov(pcidev); dfl_fpga_cdev_config_ports_pf(cdev); } else { let ret = dfl_fpga_cdev_config_ports_vf(cdev, num_vfs); if ret != 0 { return ret; } let ret = pci_enable_sriov(pcidev, num_vfs); if ret != 0 { dfl_fpga_cdev_config_ports_pf(cdev); return ret; } } num_vfs
}

unsafe fn cci_pci_remove(pcidev: *mut pci_dev) { if dev_is_pf(&mut (*pcidev).dev) { cci_pci_sriov_configure(pcidev, 0); } cci_remove_feature_devs(pcidev); }

static mut cci_pci_driver: pci_driver = pci_driver { name: DRV_NAME, id_table: cci_pcie_id_tbl.as_ptr(), probe: Some(cci_pci_probe), remove: Some(cci_pci_remove), sriov_configure: Some(cci_pci_sriov_configure) };

// module_pci_driver(cci_pci_driver)
// MODULE_DESCRIPTION("FPGA DFL PCIe Device Driver"); MODULE_AUTHOR("Intel Corporation"); MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
