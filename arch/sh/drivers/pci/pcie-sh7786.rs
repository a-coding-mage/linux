// SPDX-License-Identifier: GPL-2.0
/* Low-Level PCI Express Support for the SH7786 */

// C includes and build-time kernel definitions are supplied by the surrounding kernel translation.

#[repr(C)]
pub struct sh7786_pcie_port {
    pub hose: *mut pci_channel,
    pub fclk: *mut clk,
    pub phy_clk: clk,
    pub index: c_uint,
    pub endpoint: c_int,
    pub link: c_int,
}

#[repr(C)]
pub struct sh7786_pcie_hwops {
    pub core_init: Option<unsafe extern "C" fn() -> c_int>,
    pub port_init_hw: Option<unsafe extern "C" fn(*mut c_void, async_cookie_t)>,
}

static mut sh7786_pcie_ports: *mut sh7786_pcie_port = core::ptr::null_mut();
static mut nr_ports: c_uint = 0;
pub static mut memsize: usize = 0;
pub static mut memstart: u64 = 0;
static mut sh7786_pcie_hwops: *mut sh7786_pcie_hwops = core::ptr::null_mut();

extern "C" {
    static mut sh7786_pci_ops: pci_ops;
    static mut memory_start: usize;
    static mut memory_end: usize;
    static mut fixed_pciexclkp: clk;
}

#[repr(C)]
struct resource { name: *const c_char, start: u64, end: u64, flags: u64 }
#[repr(C)]
struct pci_channel {
    pci_ops: *mut pci_ops, resources: *mut resource, nr_resources: c_uint,
    reg_base: u64, mem_offset: u64, io_offset: u64, io_map_base: u64,
}
#[repr(C)] struct pci_ops;
#[repr(C)] struct pci_dev { bus: *mut c_void, devfn: c_uint, dev: c_void }
#[repr(C)] struct clk { rate: u64, parent: *mut clk, enable_reg: *mut c_void, enable_bit: c_uint }

static mut sh7786_pci0_resources: [resource; 4] = [
    resource { name: b"PCIe0 MEM 0\0".as_ptr() as _, start: 0xfd000000, end: 0xfd000000 + SZ_8M - 1, flags: IORESOURCE_MEM },
    resource { name: b"PCIe0 MEM 1\0".as_ptr() as _, start: 0xc0000000, end: 0xc0000000 + SZ_512M - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT },
    resource { name: b"PCIe0 MEM 2\0".as_ptr() as _, start: 0x10000000, end: 0x10000000 + SZ_64M - 1, flags: IORESOURCE_MEM },
    resource { name: b"PCIe0 IO\0".as_ptr() as _, start: 0xfe100000, end: 0xfe100000 + SZ_1M - 1, flags: IORESOURCE_IO },
];
static mut sh7786_pci1_resources: [resource; 4] = [
    resource { name: b"PCIe1 MEM 0\0".as_ptr() as _, start: 0xfd800000, end: 0xfd800000 + SZ_8M - 1, flags: IORESOURCE_MEM },
    resource { name: b"PCIe1 MEM 1\0".as_ptr() as _, start: 0xa0000000, end: 0xa0000000 + SZ_512M - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT },
    resource { name: b"PCIe1 MEM 2\0".as_ptr() as _, start: 0x30000000, end: 0x30000000 + SZ_256M - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT },
    resource { name: b"PCIe1 IO\0".as_ptr() as _, start: 0xfe300000, end: 0xfe300000 + SZ_1M - 1, flags: IORESOURCE_IO },
];
static mut sh7786_pci2_resources: [resource; 4] = [
    resource { name: b"PCIe2 MEM 0\0".as_ptr() as _, start: 0xfc800000, end: 0xfc800000 + SZ_4M - 1, flags: IORESOURCE_MEM },
    resource { name: b"PCIe2 MEM 1\0".as_ptr() as _, start: 0x80000000, end: 0x80000000 + SZ_512M - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT },
    resource { name: b"PCIe2 MEM 2\0".as_ptr() as _, start: 0x20000000, end: 0x20000000 + SZ_256M - 1, flags: IORESOURCE_MEM | IORESOURCE_MEM_32BIT },
    resource { name: b"PCIe2 IO\0".as_ptr() as _, start: 0xfcd00000, end: 0xfcd00000 + SZ_1M - 1, flags: IORESOURCE_IO },
];

static mut sh7786_pci_channels: [pci_channel; 3] = [
    pci_channel { pci_ops: unsafe { &mut sh7786_pci_ops }, resources: unsafe { sh7786_pci0_resources.as_mut_ptr() }, nr_resources: 4, reg_base: 0xfe000000, mem_offset: 0, io_offset: 0, io_map_base: 0 },
    pci_channel { pci_ops: unsafe { &mut sh7786_pci_ops }, resources: unsafe { sh7786_pci1_resources.as_mut_ptr() }, nr_resources: 4, reg_base: 0xfe200000, mem_offset: 0, io_offset: 0, io_map_base: 0 },
    pci_channel { pci_ops: unsafe { &mut sh7786_pci_ops }, resources: unsafe { sh7786_pci2_resources.as_mut_ptr() }, nr_resources: 4, reg_base: 0xfcc00000, mem_offset: 0, io_offset: 0, io_map_base: 0 },
];

static mut fixed_pciexclkp_local: clk = clk { rate: 100000000, parent: core::ptr::null_mut(), enable_reg: core::ptr::null_mut(), enable_bit: 0 };

unsafe fn phy_wait_for_ack(chan: *mut pci_channel) -> c_int {
    let mut timeout = 100;
    while timeout != 0 { timeout -= 1; if pci_read_reg(chan, SH4A_PCIEPHYADRR) & (1 << BITS_ACK) != 0 { return 0; } udelay(100); }
    -ETIMEDOUT
}
unsafe fn pci_wait_for_irq(chan: *mut pci_channel, mask: c_uint) -> c_int {
    let mut timeout = 100;
    while timeout != 0 { timeout -= 1; if pci_read_reg(chan, SH4A_PCIEINTR) & mask == mask { return 0; } udelay(100); }
    -ETIMEDOUT
}
unsafe fn phy_write_reg(chan: *mut pci_channel, addr: c_uint, lane: c_uint, data: c_uint) {
    let phyaddr = (1 << BITS_CMD) + ((lane & 0xf) << BITS_LANE) + ((addr & 0xff) << BITS_ADR);
    pci_write_reg(chan, data, SH4A_PCIEPHYDOUTR); pci_write_reg(chan, phyaddr, SH4A_PCIEPHYADRR); phy_wait_for_ack(chan);
    pci_write_reg(chan, 0, SH4A_PCIEPHYDOUTR); pci_write_reg(chan, 0, SH4A_PCIEPHYADRR); phy_wait_for_ack(chan);
}

unsafe fn pcie_clk_init(port: *mut sh7786_pcie_port) -> c_int {
    let chan = (*port).hose; let mut ret = clk_register(&mut fixed_pciexclkp_local); if ret != 0 { return ret; }
    let mut name = [0i8; 16]; snprintf(name.as_mut_ptr(), name.len(), b"pcie%d_fck\0".as_ptr() as _, (*port).index);
    (*port).fclk = clk_get(core::ptr::null_mut(), name.as_ptr()); if is_err((*port).fclk) { ret = ptr_err((*port).fclk); goto_err_fclk(ret); }
    clk_enable((*port).fclk); let clk = &mut (*port).phy_clk; *clk = core::mem::zeroed(); clk.parent = &mut fixed_pciexclkp_local; clk.enable_reg = ((*chan).reg_base + SH4A_PCIEPHYCTLR as u64) as *mut c_void; clk.enable_bit = BITS_CKE;
    ret = sh_clk_mstp_register(clk, 1); if ret < 0 { clk_disable((*port).fclk); clk_put((*port).fclk); clk_unregister(&mut fixed_pciexclkp_local); } ret
}

unsafe fn phy_init(port: *mut sh7786_pcie_port) -> c_int { let chan=(*port).hose; let mut timeout=100; clk_enable(&mut (*port).phy_clk);
    phy_write_reg(chan,0x60,0xf,0x004b008b); phy_write_reg(chan,0x61,0xf,0x00007b41); phy_write_reg(chan,0x64,0xf,0x00ff4f00); phy_write_reg(chan,0x65,0xf,0x09070907); phy_write_reg(chan,0x66,0xf,0x00000010); phy_write_reg(chan,0x74,0xf,0x0007001c); phy_write_reg(chan,0x79,0xf,0x01fc000d); phy_write_reg(chan,0xb0,0xf,0x00000610); phy_write_reg(chan,0x67,1,0x00000400); clk_disable(&mut (*port).phy_clk);
    while timeout != 0 { timeout-=1; if pci_read_reg(chan,SH4A_PCIEPHYSR)!=0{return 0;} udelay(100); } -ETIMEDOUT }

unsafe fn pcie_reset(port: *mut sh7786_pcie_port) { let c=(*port).hose; pci_write_reg(c,1,SH4A_PCIESRSTR); pci_write_reg(c,0,SH4A_PCIETCTLR); pci_write_reg(c,0,SH4A_PCIESRSTR); pci_write_reg(c,0,SH4A_PCIETXVC0SR); }

// The remaining initialization routines retain the source ordering and call external kernel APIs.
// Their register constants, resource helpers, and platform declarations are supplied externally.
unsafe fn pcie_init(port: *mut sh7786_pcie_port) -> c_int {
    let chan=(*port).hose; let mut data; let (mut mem_start, mem_end)=(__pa(memory_start),__pa(memory_end));
    pcie_reset(port); pci_write_reg(chan,PCI_CLASS_BRIDGE_PCI_NORMAL<<8,SH4A_PCIEIDSETR1); data=pci_read_reg(chan,SH4A_PCIEEXPCAP0); data &= !(PCI_EXP_FLAGS_TYPE<<16); data |= if (*port).endpoint { PCI_EXP_TYPE_ENDPOINT<<20 } else { PCI_EXP_TYPE_ROOT_PORT<<20 }; data|=PCI_CAP_ID_EXP; pci_write_reg(chan,data,SH4A_PCIEEXPCAP0);
    pci_write_reg(chan,PCI_EXP_LNKCAP_DLLLARC,SH4A_PCIEEXPCAP3); data=pci_read_reg(chan,SH4A_PCIEEXPCAP4); data &= !PCI_EXP_LNKCTL_ASPMC; data|=PCI_EXP_LNKCTL_ES|1; pci_write_reg(chan,data,SH4A_PCIEEXPCAP4); data=pci_read_reg(chan,SH4A_PCIEEXPCAP5); data &= !PCI_EXP_SLTCAP_PSN; data|=((*port).index+1)<<19; pci_write_reg(chan,data,SH4A_PCIEEXPCAP5);
    data=pci_read_reg(chan,SH4A_PCIETLCTLR); data&=!0x3f00; data|=0x32<<8; pci_write_reg(chan,data,SH4A_PCIETLCTLR); data=pci_read_reg(chan,SH4A_PCIEMACCTLR); data&=!PCIEMACCTLR_SCR_DIS; data|=0xff<<16; pci_write_reg(chan,data,SH4A_PCIEMACCTLR);
    memsize=roundup_pow_of_two(mem_end-mem_start); mem_start=align_down(mem_start,memsize); memsize=roundup_pow_of_two(mem_end-mem_start); if memsize>SZ_512M {pci_write_reg(chan,mem_start+SZ_512M,SH4A_PCIELAR1); pci_write_reg(chan,(memsize-SZ_512M)-SZ_256|1,SH4A_PCIELAMR1); memsize=SZ_512M;} else {pci_write_reg(chan,0,SH4A_PCIELAR1);pci_write_reg(chan,0,SH4A_PCIELAMR1);} pci_write_reg(chan,mem_start,SH4A_PCIELAR0); pci_write_reg(chan,(memsize-SZ_256)|1,SH4A_PCIELAMR0);
    data=pci_read_reg(chan,SH4A_PCIETCTLR)|1; pci_write_reg(chan,data,SH4A_PCIETCTLR); mdelay(100); data=pci_read_reg(chan,SH4A_PCIEDLINTENR)|PCIEDLINTENR_DLL_ACT_ENABLE; pci_write_reg(chan,data,SH4A_PCIEDLINTENR); data=pci_read_reg(chan,SH4A_PCIEMACCTLR)|PCIEMACCTLR_SCR_DIS|(0xff<<16); pci_write_reg(chan,data,SH4A_PCIEMACCTLR); let ret=pci_wait_for_irq(chan,MASK_INT_TX_CTRL);
    data=pci_read_reg(chan,SH4A_PCIEPCICONF1); data &= !(PCI_STATUS_DEVSEL_MASK<<16); data|=PCI_COMMAND_IO|PCI_COMMAND_MEMORY|PCI_COMMAND_MASTER|(PCI_STATUS_CAP_LIST|PCI_STATUS_DEVSEL_FAST)<<16; pci_write_reg(chan,data,SH4A_PCIEPCICONF1); pci_write_reg(chan,0x80888000,SH4A_PCIETXVC0DCTLR); pci_write_reg(chan,0x00222000,SH4A_PCIERXVC0DCTLR); wmb(); ret
}

// External declarations and platform entry points referenced by this translation.
extern "C" { fn pci_read_reg(*mut pci_channel,u32)->u32; fn pci_write_reg(*mut pci_channel,u32,u32); fn udelay(u32); fn mdelay(u32); fn clk_register(*mut clk)->c_int; fn clk_get(*mut c_void,*const c_char)->*mut clk; fn clk_enable(*mut clk); fn clk_disable(*mut clk); fn clk_put(*mut clk); fn clk_unregister(*mut clk); fn sh_clk_mstp_register(*mut clk,u32)->c_int; fn ptr_err(*mut clk)->c_int; fn is_err(*mut clk)->bool; fn snprintf(*mut c_char,usize,*const c_char,...)->c_int; fn __pa(usize)->u64; fn roundup_pow_of_two(usize)->usize; fn align_down(u64,usize)->u64; fn wmb(); }
type c_int=i32; type c_uint=u32; type c_char=i8; type c_void=core::ffi::c_void; type async_cookie_t=u64;
const ETIMEDOUT:c_int=110;

pub unsafe extern "C" fn pcibios_map_platform_irq(_pdev: *const pci_dev, _slot: u8, _pin: u8) -> c_int { evt2irq(0xae0) }
pub unsafe extern "C" fn pcibios_bus_add_device(pdev: *mut pci_dev) {
    dma_direct_set_offset(&mut (*pdev).dev, __pa(memory_start), __pa(memory_start) - memstart, memsize);
}
unsafe fn sh7786_pcie_core_init() -> c_int { if test_mode_pin(MODE_PIN12) { 3 } else { 2 } }
unsafe extern "C" fn sh7786_pcie_init_hw(data: *mut c_void, cookie: async_cookie_t) {
    let port=data as *mut sh7786_pcie_port; (*port).endpoint=test_mode_pin(MODE_PIN11) as c_int;
    let mut ret=pcie_clk_init(port); if ret<0 { return; } ret=phy_init(port); if ret<0 { return; } ret=pcie_init(port); if ret<0 { return; }
    async_synchronize_cookie(cookie); register_pci_controller((*port).hose);
}
static mut sh7786_65nm_pcie_hwops: sh7786_pcie_hwops = sh7786_pcie_hwops { core_init: Some(sh7786_pcie_core_init), port_init_hw: Some(sh7786_pcie_init_hw) };

pub unsafe extern "C" fn sh7786_pcie_init() -> c_int {
    sh7786_pcie_hwops=&mut sh7786_65nm_pcie_hwops; nr_ports=((*sh7786_pcie_hwops).core_init.unwrap())() as c_uint; if nr_ports==0{return -ENODEV;}
    sh7786_pcie_ports=kzalloc_objs::<sh7786_pcie_port>(nr_ports as usize); if sh7786_pcie_ports.is_null(){return -ENOMEM;}
    let platclk=clk_get(core::ptr::null_mut(),b"pcie_plat_clk\0".as_ptr() as _); if !is_err(platclk){clk_enable(platclk);}
    let mm_sel=sh7786_mm_sel(); if mm_sel!=1&&mm_sel!=2&&mm_sel!=5&&mm_sel!=6 { sh7786_pci0_resources[2].flags|=IORESOURCE_DISABLED; }
    for i in 0..nr_ports as usize { let p=sh7786_pcie_ports.add(i); (*p).index=i as c_uint; (*p).hose=sh7786_pci_channels.as_mut_ptr().add(i); (*(*p).hose).io_map_base=(*(*p).hose).resources.as_ref().unwrap().start; async_schedule((*sh7786_pcie_hwops).port_init_hw.unwrap(),p as *mut c_void); }
    async_synchronize_full(); 0
}

extern "C" { fn evt2irq(u32)->c_int; fn dma_direct_set_offset(*mut c_void,u64,u64,usize); fn test_mode_pin(u32)->bool; fn async_synchronize_cookie(async_cookie_t); fn register_pci_controller(*mut pci_channel); fn async_schedule(unsafe extern "C" fn(*mut c_void,async_cookie_t),*mut c_void)->async_cookie_t; fn async_synchronize_full(); fn sh7786_mm_sel()->u32; }
extern "C" { fn kzalloc_objs<T>(usize)->*mut T; }
const ENODEV:c_int=19; const ENOMEM:c_int=12; const IORESOURCE_MEM:u64=1; const IORESOURCE_IO:u64=2; const IORESOURCE_MEM_32BIT:u64=4; const IORESOURCE_DISABLED:u64=0x8000;
const SZ_1M:u64=0x100000; const SZ_4M:u64=0x400000; const SZ_8M:u64=0x800000; const SZ_64M:u64=0x4000000; const SZ_256M:u64=0x10000000; const SZ_512M:u64=0x20000000; const SZ_256:u64=0x100; const MODE_PIN11:u32=11; const MODE_PIN12:u32=12;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
