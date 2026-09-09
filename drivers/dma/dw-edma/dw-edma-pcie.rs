// SPDX-License-Identifier: GPL-2.0
/* Synopsys DesignWare eDMA PCIe driver; literal low-level translation. */

const DW_PCIE_SYNOPSYS_VSEC_DMA_ID: u16 = 0x6;
const DW_PCIE_SYNOPSYS_VSEC_DMA_BAR: u32 = 0x700;
const DW_PCIE_SYNOPSYS_VSEC_DMA_MAP: u32 = 0x7;
const DW_PCIE_SYNOPSYS_VSEC_DMA_WR_CH: u32 = 0x3ff;
const DW_PCIE_SYNOPSYS_VSEC_DMA_RD_CH: u32 = 0x03ff0000;
const PCI_DEVICE_ID_XILINX_B054: u16 = 0xb054;
const PCI_DEVICE_ID_XILINX_B00F: u16 = 0xb00f;
const DW_PCIE_XILINX_MDB_VSEC_DMA_ID: u16 = 0x6;
const DW_PCIE_XILINX_MDB_VSEC_ID: u16 = 0x20;
const DW_PCIE_XILINX_MDB_VSEC_DMA_BAR: u32 = 0x700;
const DW_PCIE_XILINX_MDB_VSEC_DMA_MAP: u32 = 0x7;
const DW_PCIE_XILINX_MDB_VSEC_DMA_WR_CH: u32 = 0x3ff;
const DW_PCIE_XILINX_MDB_VSEC_DMA_RD_CH: u32 = 0x03ff0000;
const DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_HIGH: u16 = 0xc;
const DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_LOW: u16 = 0x8;
const DW_PCIE_XILINX_MDB_INVALID_ADDR: u64 = !0;
const DW_PCIE_XILINX_MDB_LL_OFF_GAP: isize = 0x200000;
const DW_PCIE_XILINX_MDB_LL_SIZE: usize = 0x800;
const DW_PCIE_XILINX_MDB_DT_OFF_GAP: isize = 0x100000;
const DW_PCIE_XILINX_MDB_DT_SIZE: usize = 0x800;
const DW_EDMA_PCIE_F_DEVMEM_PHYS_OFF: u64 = 1 << 0;
const DW_EDMA_PCIE_F_REG_OFFSET: u64 = 1 << 1;

#[repr(C)]
pub struct dw_edma_block { pub bar: pci_barno, pub off: isize, pub paddr: u64, pub paddr_valid: bool, pub sz: usize }
#[repr(C)]
pub struct dw_edma_pcie_data {
    pub rg: dw_edma_block,
    pub ll_wr: [dw_edma_block; HDMA_MAX_WR_CH], pub ll_rd: [dw_edma_block; HDMA_MAX_RD_CH],
    pub dt_wr: [dw_edma_block; HDMA_MAX_WR_CH], pub dt_rd: [dw_edma_block; HDMA_MAX_RD_CH],
    pub mf: dw_edma_map_format, pub irqs: u8, pub wr_ch_cnt: u16, pub rd_ch_cnt: u16,
    pub devmem_phys_off: u64, pub cfg_non_ll: bool,
}
#[repr(C)]
pub struct dw_edma_pcie_match_data {
    pub data: *const dw_edma_pcie_data, pub plat_ops: *const dw_edma_plat_ops,
    pub parse_caps: Option<unsafe extern "C" fn(*mut pci_dev, *mut dw_edma_pcie_data) -> i32>,
    pub flags: u64, pub chip_flags: u32,
}

unsafe fn dw_edma_set_chan_region_offset(p: *mut dw_edma_pcie_data, bar: pci_barno, start: isize, ll_gap: isize, ll_sz: usize, dt_gap: isize, dt_sz: usize) {
    let mut off = start;
    for i in 0..(*p).wr_ch_cnt as usize { (*p).ll_wr[i] = dw_edma_block { bar, off, paddr: 0, paddr_valid: false, sz: ll_sz }; off += ll_gap; }
    for i in 0..(*p).rd_ch_cnt as usize { (*p).ll_rd[i] = dw_edma_block { bar, off, paddr: 0, paddr_valid: false, sz: ll_sz }; off += ll_gap; }
    for i in 0..(*p).wr_ch_cnt as usize { (*p).dt_wr[i] = dw_edma_block { bar, off, paddr: 0, paddr_valid: false, sz: dt_sz }; off += dt_gap; }
    for i in 0..(*p).rd_ch_cnt as usize { (*p).dt_rd[i] = dw_edma_block { bar, off, paddr: 0, paddr_valid: false, sz: dt_sz }; off += dt_gap; }
}

unsafe extern "C" fn dw_edma_pcie_irq_vector(dev: *mut device, nr: u32) -> i32 { pci_irq_vector(to_pci_dev(dev), nr) }
unsafe extern "C" fn dw_edma_pcie_address(dev: *mut device, cpu_addr: u64) -> u64 {
    let pdev = to_pci_dev(dev); let mut region = pci_bus_region { start: 0, end: 0 };
    let mut res = resource { flags: IORESOURCE_MEM, start: cpu_addr, end: cpu_addr };
    pcibios_resource_to_bus((*pdev).bus, &mut region, &mut res); region.start
}

unsafe fn dw_edma_pcie_get_synopsys_dma_data(pdev: *mut pci_dev, p: *mut dw_edma_pcie_data) {
    let mut val = 0u32; let vsec = pci_find_vsec_capability(pdev, PCI_VENDOR_ID_SYNOPSYS, DW_PCIE_SYNOPSYS_VSEC_DMA_ID); if vsec == 0 { return; }
    pci_read_config_dword(pdev, vsec + PCI_VNDR_HEADER, &mut val); if PCI_VNDR_HEADER_REV(val) != 0 || PCI_VNDR_HEADER_LEN(val) != 0x18 { return; }
    pci_read_config_dword(pdev, vsec + 8, &mut val); let map = field_get(DW_PCIE_SYNOPSYS_VSEC_DMA_MAP, val); if map < EDMA_MF_EDMA_LEGACY || map > EDMA_MF_HDMA_NATIVE { return; }
    (*p).mf = map; (*p).rg.bar = field_get(DW_PCIE_SYNOPSYS_VSEC_DMA_BAR, val);
    pci_read_config_dword(pdev, vsec + 0xc, &mut val); (*p).wr_ch_cnt = core::cmp::min((*p).wr_ch_cnt, field_get(DW_PCIE_SYNOPSYS_VSEC_DMA_WR_CH, val)); (*p).rd_ch_cnt = core::cmp::min((*p).rd_ch_cnt, field_get(DW_PCIE_SYNOPSYS_VSEC_DMA_RD_CH, val));
    pci_read_config_dword(pdev, vsec + 0x14, &mut val); let mut off = val as u64; pci_read_config_dword(pdev, vsec + 0x10, &mut val); off = (off << 32) | val as u64; (*p).rg.off = off as isize;
}

unsafe fn dw_edma_pcie_get_xilinx_dma_data(pdev: *mut pci_dev, p: *mut dw_edma_pcie_data) {
    let mut val=0u32; (*p).devmem_phys_off=DW_PCIE_XILINX_MDB_INVALID_ADDR; let vsec=pci_find_vsec_capability(pdev,PCI_VENDOR_ID_XILINX,DW_PCIE_XILINX_MDB_VSEC_DMA_ID); if vsec==0{return;}
    pci_read_config_dword(pdev,vsec+PCI_VNDR_HEADER,&mut val); if PCI_VNDR_HEADER_REV(val)!=0 || PCI_VNDR_HEADER_LEN(val)!=0x18{return;}
    pci_read_config_dword(pdev,vsec+8,&mut val); if field_get(DW_PCIE_XILINX_MDB_VSEC_DMA_MAP,val)!=EDMA_MF_HDMA_NATIVE{return;} (*p).mf=EDMA_MF_HDMA_NATIVE; (*p).rg.bar=field_get(DW_PCIE_XILINX_MDB_VSEC_DMA_BAR,val);
    pci_read_config_dword(pdev,vsec+0xc,&mut val); (*p).wr_ch_cnt=core::cmp::min((*p).wr_ch_cnt,field_get(DW_PCIE_XILINX_MDB_VSEC_DMA_WR_CH,val)); (*p).rd_ch_cnt=core::cmp::min((*p).rd_ch_cnt,field_get(DW_PCIE_XILINX_MDB_VSEC_DMA_RD_CH,val)); pci_read_config_dword(pdev,vsec+0x14,&mut val); let mut off=val as u64; pci_read_config_dword(pdev,vsec+0x10,&mut val); off=(off<<32)|val as u64; (*p).rg.off=off as isize;
    let vsec2=pci_find_vsec_capability(pdev,PCI_VENDOR_ID_XILINX,DW_PCIE_XILINX_MDB_VSEC_ID); if vsec2==0{return;} pci_read_config_dword(pdev,vsec2+DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_HIGH,&mut val); off=val as u64; pci_read_config_dword(pdev,vsec2+DW_PCIE_XILINX_MDB_DEVMEM_OFF_REG_LOW,&mut val); (*p).devmem_phys_off=(off<<32)|val as u64;
}
unsafe extern "C" fn dw_edma_pcie_parse_synopsys_caps(d:*mut pci_dev,p:*mut dw_edma_pcie_data)->i32 { dw_edma_pcie_get_synopsys_dma_data(d,p); 0 }
unsafe extern "C" fn dw_edma_pcie_parse_xilinx_caps(d:*mut pci_dev,p:*mut dw_edma_pcie_data)->i32 { dw_edma_pcie_get_xilinx_dma_data(d,p); if (*p).devmem_phys_off==DW_PCIE_XILINX_MDB_INVALID_ADDR { (*p).cfg_non_ll=true; return 0; } dw_edma_set_chan_region_offset(p,BAR_2,0,DW_PCIE_XILINX_MDB_LL_OFF_GAP,DW_PCIE_XILINX_MDB_LL_SIZE,DW_PCIE_XILINX_MDB_DT_OFF_GAP,DW_PCIE_XILINX_MDB_DT_SIZE); 0 }

unsafe fn dw_edma_get_phys_addr(pdev:*mut pci_dev,m:*const dw_edma_pcie_match_data,p:*mut dw_edma_pcie_data,bar:pci_barno)->u64 { if (*m).flags&DW_EDMA_PCIE_F_DEVMEM_PHYS_OFF!=0 {(*p).devmem_phys_off} else {pci_bus_address(pdev,bar)} }
unsafe fn dw_edma_get_block_addr(d:*mut pci_dev,m:*const dw_edma_pcie_match_data,p:*mut dw_edma_pcie_data,b:*const dw_edma_block)->u64 { if (*b).paddr_valid {(*b).paddr} else {dw_edma_get_phys_addr(d,m,p,(*b).bar)+(*b).off as u64} }

// The probe/remove bodies retain the original kernel call sequence and field assignments.
unsafe extern "C" fn dw_edma_pcie_probe(pdev:*mut pci_dev,pid:*const pci_device_id)->i32 {
    let m=(*pid).driver_data as *const dw_edma_pcie_match_data; if m.is_null()||(*m).data.is_null(){return -19;} let p=(*m).data; let mut dma_data=kmemdup(p,core::mem::size_of::<dw_edma_pcie_data>(),GFP_KERNEL) as *mut dw_edma_pcie_data; if dma_data.is_null(){return -12;}
    let mut err=pcim_enable_device(pdev); if err!=0{return err;} if (*m).parse_caps.is_none()||(*m).plat_ops.is_null(){return -22;} err=((*m).parse_caps.unwrap())(pdev,dma_data); if err!=0{return err;}
    let mut mask=1u32<<(*dma_data).rg.bar; for i in 0..(*dma_data).wr_ch_cnt as usize {mask|=1<<(*dma_data).ll_wr[i].bar; if (*dma_data).dt_wr[i].sz!=0{mask|=1<<(*dma_data).dt_wr[i].bar;}} for i in 0..(*dma_data).rd_ch_cnt as usize {mask|=1<<(*dma_data).ll_rd[i].bar; if (*dma_data).dt_rd[i].sz!=0{mask|=1<<(*dma_data).dt_rd[i].bar;}}
    err=pcim_iomap_regions(pdev,mask,pci_name(pdev)); if err!=0{return err;} pci_set_master(pdev); err=dma_set_mask_and_coherent(&mut (*pdev).dev,DMA_BIT_MASK(64)); if err!=0{return err;}
    let chip=devm_kzalloc(&mut (*pdev).dev,core::mem::size_of::<dw_edma_chip>(),GFP_KERNEL) as *mut dw_edma_chip; if chip.is_null(){return -12;} let nr=pci_alloc_irq_vectors(pdev,1,(*dma_data).irqs as u32,PCI_IRQ_MSI|PCI_IRQ_MSIX); if nr<1{return -1;}
    (*chip).dev=&mut (*pdev).dev; (*chip).mf=(*dma_data).mf; (*chip).flags=(*m).chip_flags; (*chip).func_no=PCI_FUNC((*pdev).devfn); (*chip).nr_irqs=nr; (*chip).ops=(*m).plat_ops; (*chip).cfg_non_ll=(*dma_data).cfg_non_ll; (*chip).ll_wr_cnt=(*dma_data).wr_ch_cnt; (*chip).ll_rd_cnt=(*dma_data).rd_ch_cnt;
    (*chip).reg_base=pcim_iomap_table(pdev)[(*dma_data).rg.bar as usize]; if (*chip).reg_base.is_null(){return -12;} if (*m).flags&DW_EDMA_PCIE_F_REG_OFFSET!=0{(*chip).reg_base=(*chip).reg_base.offset((*dma_data).rg.off);}
    if !(*dma_data).cfg_non_ll { for i in 0..(*chip).ll_wr_cnt as usize { let l=&mut (*chip).ll_region_wr[i]; let b=&(*dma_data).ll_wr[i]; l.vaddr.io=pcim_iomap_table(pdev)[b.bar as usize]; if l.vaddr.io.is_null(){return -12;} l.vaddr.io=l.vaddr.io.offset(b.off); l.paddr=dw_edma_get_block_addr(pdev,m,dma_data,b); l.sz=b.sz; let db=&(*dma_data).dt_wr[i]; if db.sz!=0 {let t=&mut (*chip).dt_region_wr[i];t.vaddr.io=pcim_iomap_table(pdev)[db.bar as usize];if t.vaddr.io.is_null(){return -12;}t.vaddr.io=t.vaddr.io.offset(db.off);t.paddr=dw_edma_get_block_addr(pdev,m,dma_data,db);t.sz=db.sz;} } }
    if !(*dma_data).cfg_non_ll { for i in 0..(*chip).ll_rd_cnt as usize { let l=&mut (*chip).ll_region_rd[i]; let b=&(*dma_data).ll_rd[i]; l.vaddr.io=pcim_iomap_table(pdev)[b.bar as usize]; if l.vaddr.io.is_null(){return -12;} l.vaddr.io=l.vaddr.io.offset(b.off); l.paddr=dw_edma_get_block_addr(pdev,m,dma_data,b); l.sz=b.sz; let db=&(*dma_data).dt_rd[i]; if db.sz!=0 {let t=&mut (*chip).dt_region_rd[i];t.vaddr.io=pcim_iomap_table(pdev)[db.bar as usize];if t.vaddr.io.is_null(){return -12;}t.vaddr.io=t.vaddr.io.offset(db.off);t.paddr=dw_edma_get_block_addr(pdev,m,dma_data,db);t.sz=db.sz;} } }
    if !pci_dev_msi_enabled(pdev){return -1;} err=dw_edma_probe(chip); if err!=0{return err;} pci_set_drvdata(pdev,chip); 0
}
unsafe extern "C" fn dw_edma_pcie_remove(pdev:*mut pci_dev){let _=dw_edma_remove(pci_get_drvdata(pdev));}

/* Static templates and PCI registration data (external kernel types/macros are intentionally unresolved). */
extern "C" {
    static snps_edda_data: dw_edma_pcie_data;
    static xilinx_mdb_data: dw_edma_pcie_data;
    static xilinx_cpm6_dma_data: dw_edma_pcie_data;
    static dw_edma_pcie_plat_ops: dw_edma_plat_ops;
    static snps_edda_match_data: dw_edma_pcie_match_data;
    static xilinx_mdb_match_data: dw_edma_pcie_match_data;
    static xilinx_cpm6_dma_match_data: dw_edma_pcie_match_data;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
