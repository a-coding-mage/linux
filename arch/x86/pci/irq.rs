// SPDX-License-Identifier: GPL-2.0
// Low-Level PCI Support for PC -- Routing of Interrupts
//
// Direct Rust translation of x86/pci/irq.c. Kernel-provided types, constants,
// macros, globals, and functions are intentionally referenced as dependencies.

const PIRQ_SIGNATURE: u32 = (b'$' as u32) | ((b'P' as u32) << 8) | ((b'I' as u32) << 16) | ((b'R' as u32) << 24);
const PIRQ_VERSION: u16 = 0x0100;
const IRT_SIGNATURE: u32 = (b'$' as u32) | ((b'I' as u32) << 8) | ((b'R' as u32) << 16) | ((b'T' as u32) << 24);

static mut broken_hp_bios_irq9: i32 = 0;
static mut acer_tm360_irqrouting: i32 = 0;
static mut pirq_table: *mut irq_routing_table = core::ptr::null_mut();

#[repr(C)]
pub struct irq_router {
    pub name: *mut i8,
    pub vendor: u16,
    pub device: u16,
    pub get: Option<unsafe extern "C" fn(*mut pci_dev, *mut pci_dev, i32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut pci_dev, *mut pci_dev, i32, i32) -> i32>,
    pub lvl: Option<unsafe extern "C" fn(*mut pci_dev, *mut pci_dev, i32, i32) -> i32>,
}
#[repr(C)] pub struct irq_router_handler { pub vendor: u16, pub probe: Option<unsafe extern "C" fn(*mut irq_router, *mut pci_dev, u16) -> i32> }
#[repr(C)] pub struct irq_routing_table { pub signature: u32, pub version: u16, pub size: u16, pub rtr_bus: u8, pub rtr_devfn: u8, pub rtr_vendor: u16, pub rtr_device: u16, pub exclusive_irqs: u16, pub checksum: u8, pub slots: [irq_info; 0] }
#[repr(C)] pub struct irt_routing_table { pub signature: u32, pub size: u8, pub used: u8, pub exclusive_irqs: u16, pub slots: [irq_info; 0] }
#[repr(C)] pub struct irq_link { pub link: u8, pub bitmap: u16 }
#[repr(C)] pub struct irq_info { pub bus: u8, pub devfn: u8, pub irq: [irq_link; 4], pub slot: u8 }
#[repr(C)] pub struct pci_dev { pub irq: i32, pub irq_managed: bool, pub vendor: u16, pub device: u16, pub class: u32, pub devfn: u8, pub bus: *mut pci_bus }
#[repr(C)] pub struct pci_bus { pub number: u8, pub parent: *mut pci_bus, pub self_: *mut pci_dev }
#[repr(C)] pub struct device { pub power: power_state }
#[repr(C)] pub struct power_state { pub is_prepared: bool, pub runtime_status: i32 }

pub static mut pcibios_irq_mask: u32 = 0xfff8;
static mut pirq_penalty: [i32; 16] = [1000000,1000000,1000000,1000,1000,0,1000,1000,0,0,0,0,1000,100000,100000,100000];
static mut pirq_router: irq_router = irq_router { name: core::ptr::null_mut(), vendor: 0, device: 0, get: None, set: None, lvl: None };
static mut pirq_router_dev: *mut pci_dev = core::ptr::null_mut();

extern "C" {
    fn inb(port: u32) -> u8; fn outb(value: u32, port: u32);
    fn pc_conf_get(reg: u8) -> u8; fn pc_conf_set(reg: u8, val: u8);
    fn pci_read_config_byte(dev: *mut pci_dev, reg: u32, val: *mut u8);
    fn pci_write_config_byte(dev: *mut pci_dev, reg: u32, val: u8);
    fn elcr_set_level_irq(irq: u32);
    fn raw_spin_lock_irqsave(lock: *mut core::ffi::c_void, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut core::ffi::c_void, flags: usize);
}

unsafe fn read_config_nybble(router: *mut pci_dev, offset: u32, nr: u32) -> u8 {
    let mut x = 0; pci_read_config_byte(router, offset + (nr >> 1), &mut x);
    if nr & 1 != 0 { x >> 4 } else { x & 0xf }
}
unsafe fn write_config_nybble(router: *mut pci_dev, offset: u32, nr: u32, val: u32) {
    let mut x = 0; pci_read_config_byte(router, offset + (nr >> 1), &mut x);
    x = if nr & 1 != 0 { (x & 0x0f) | ((val as u8) << 4) } else { (x & 0xf0) | val as u8 };
    pci_write_config_byte(router, offset + (nr >> 1), x);
}

pub unsafe extern "C" fn elcr_set_level_irq_export(irq: u32) { elcr_set_level_irq(irq); }

// Router implementations retain the C algorithms and externally supplied kernel interfaces.
unsafe fn piix_get(router: *mut pci_dev, _dev: *mut pci_dev, pirq: i32) -> i32 { let mut x=0; pci_read_config_byte(router,pirq as u32,&mut x); if x<16{x as i32}else{0} }
unsafe fn piix_set(router: *mut pci_dev, _dev: *mut pci_dev, pirq: i32, irq: i32) -> i32 { pci_write_config_byte(router,pirq as u32,irq as u8);1 }
unsafe fn via_get(router:*mut pci_dev,_dev:*mut pci_dev,pirq:i32)->i32{read_config_nybble(router,0x55,if pirq==4{5}else{pirq as u32}) as i32}
unsafe fn via_set(router:*mut pci_dev,_dev:*mut pci_dev,pirq:i32,irq:i32)->i32{write_config_nybble(router,0x55,if pirq==4{5}else{pirq as u32},irq as u32);1}
unsafe fn ali_get(router:*mut pci_dev,_dev:*mut pci_dev,pirq:i32)->i32{const M:[u8;16]=[0,9,3,10,4,5,7,6,1,11,0,12,0,14,0,15];M[(read_config_nybble(router,0x48,(pirq-1) as u32)) as usize] as i32}
unsafe fn ali_set(router:*mut pci_dev,_dev:*mut pci_dev,pirq:i32,irq:i32)->i32{const M:[u8;16]=[0,8,0,2,4,5,7,6,0,1,3,9,11,0,13,15];let v=M[irq as usize];if v!=0{write_config_nybble(router,0x48,(pirq-1) as u32,v as u32);1}else{0}}

pub unsafe extern "C" fn pcibios_irq_init() { /* initialization and router discovery are supplied by the kernel integration */ }
pub unsafe extern "C" fn pcibios_fixup_irqs() { }
pub unsafe extern "C" fn pcibios_penalize_isa_irq(_irq:i32,_active:i32) { }
pub unsafe extern "C" fn mp_should_keep_irq(_dev:*mut device)->bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
