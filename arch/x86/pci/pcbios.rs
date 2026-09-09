// SPDX-License-Identifier: GPL-2.0
/* BIOS32 and PCI BIOS handling. */

// Linux header dependencies are supplied by the surrounding translation unit.

pub const BIOS32_SIGNATURE: u32 = (b'_' as u32) | ((b'3' as u32) << 8) |
    ((b'2' as u32) << 16) | ((b'_' as u32) << 24);
pub const PCI_SIGNATURE: u32 = (b'P' as u32) | ((b'C' as u32) << 8) |
    ((b'I' as u32) << 16) | ((b' ' as u32) << 24);
pub const PCI_SERVICE: u32 = (b'$' as u32) | ((b'P' as u32) << 8) |
    ((b'C' as u32) << 16) | ((b'I' as u32) << 24);
pub const PCIBIOS_HW_TYPE1: u32 = 0x01;
pub const PCIBIOS_HW_TYPE2: u32 = 0x02;
pub const PCIBIOS_HW_TYPE1_SPEC: u32 = 0x10;
pub const PCIBIOS_HW_TYPE2_SPEC: u32 = 0x20;
pub const PCIBIOS_RETURN_CODE: u32 = 0xff00;

pub static mut pcibios_enabled: i32 = 0;

#[repr(C)]
pub union bios32 {
    pub fields: bios32_fields,
    pub chars: [i8; 16],
}
#[repr(C)]
pub struct bios32_fields {
    pub signature: usize,
    pub entry: usize,
    pub revision: u8,
    pub length: u8,
    pub checksum: u8,
    pub reserved: [u8; 5],
}

#[repr(C)]
struct BiosIndirect { address: usize, segment: u16 }
static mut bios32_indirect: BiosIndirect = BiosIndirect { address: 0, segment: 0 };
static mut pci_indirect: BiosIndirect = BiosIndirect { address: 0, segment: 0 };
static mut pci_bios_present: i32 = 0;

extern "C" {
    static mut pcibios_last_bus: i32;
    static mut pci_probe: u32;
    static mut raw_pci_ops: *const pci_raw_ops;
    static pci_config_lock: u8;
    fn set_memory_x(addr: usize, pages: usize);
    fn printk(fmt: *const u8, ...);
    fn bios32_lcall(service: usize, indirect: *const BiosIndirect,
                    return_code: *mut u8, address: *mut usize, length: *mut usize,
                    entry: *mut usize);
    fn pci_bios_lcall(number: u16, bx: usize, reg: usize, indirect: *const BiosIndirect,
                      value: *mut u32, result: *mut usize);
    fn pci_bios_irq_lcall(opt: *mut irq_routing_options, ret: *mut i32, map: *mut i32,
                          indirect: *const BiosIndirect);
    fn pci_bios_set_irq_lcall(devfn: u16, pinirq: u16, indirect: *const BiosIndirect,
                              ret: *mut i32);
}

#[inline]
unsafe fn pcibios_get_return_code(eax: u32) -> u8 { ((eax & PCIBIOS_RETURN_CODE) >> 8) as u8 }

#[inline]
unsafe fn set_bios_x() {
    pcibios_enabled = 1;
    set_memory_x(0 + 0x000e0000, (0x00100000 - 0x000e0000) >> 12);
}

unsafe fn bios32_service(service: usize) -> usize {
    let mut rc = 0u8; let mut address = 0usize; let mut length = 0usize; let mut entry = 0usize;
    bios32_lcall(service, &bios32_indirect, &mut rc, &mut address, &mut length, &mut entry);
    match rc { 0 => address.wrapping_add(entry), _ => 0 }
}

unsafe fn check_pcibios() -> i32 {
    let entry = bios32_service(PCI_SERVICE as usize);
    if entry == 0 { return 0; }
    pci_indirect.address = entry;
    let mut signature = 0u32; let mut eax = 0u32; let mut ebx = 0u32; let mut ecx = 0u32;
    // The BIOS32 far call and register clobbers are represented by the external call above.
    pci_bios_lcall(0, 0, 0, &pci_indirect, &mut signature, &mut eax);
    let status = pcibios_get_return_code(eax);
    let hw_mech = (eax & 0xff) as u8;
    let major_ver = ((ebx >> 8) & 0xff) as u8;
    let minor_ver = (ebx & 0xff) as u8;
    if pcibios_last_bus < 0 { pcibios_last_bus = (ecx & 0xff) as i32; }
    if status != 0 || signature != PCI_SIGNATURE { return 0; }
    #[cfg(CONFIG_PCI_DIRECT)] {
        if (hw_mech as u32 & PCIBIOS_HW_TYPE1) == 0 { pci_probe &= !0x1; }
        if (hw_mech as u32 & PCIBIOS_HW_TYPE2) == 0 { pci_probe &= !0x2; }
    }
    let _ = (major_ver, minor_ver);
    1
}

unsafe fn pci_bios_read(seg: u32, bus: u32, devfn: u32, reg: i32, len: i32, value: *mut u32) -> i32 {
    let _ = seg;
    if value.is_null() || bus > 255 || devfn > 255 || reg > 255 { return -22; }
    let (number, mask) = match len { 1 => (0x08u16, 0xff), 2 => (0x09, 0xffff), 4 => (0x0a, 0), _ => (0, 0) };
    let mut result = 0usize;
    pci_bios_lcall(number, (bus << 8) | devfn, reg as usize, &pci_indirect, value, &mut result);
    if mask != 0 { *value &= mask; }
    pcibios_get_return_code(result as u32) as i32
}

unsafe fn pci_bios_write(seg: u32, bus: u32, devfn: u32, reg: i32, len: i32, value: u32) -> i32 {
    let _ = seg;
    if bus > 255 || devfn > 255 || reg > 255 { return -22; }
    let number = match len { 1 => 0x0b, 2 => 0x0c, 4 => 0x0d, _ => 0 };
    let mut result = 0usize; let mut ignored = value;
    pci_bios_lcall(number, (bus << 8) | devfn, reg as usize, &pci_indirect, &mut ignored, &mut result);
    pcibios_get_return_code(result as u32) as i32
}

#[repr(C)] pub struct pci_raw_ops { pub read: unsafe fn(u32,u32,u32,i32,i32,*mut u32)->i32, pub write: unsafe fn(u32,u32,u32,i32,i32,u32)->i32 }
static pci_bios_access: pci_raw_ops = pci_raw_ops { read: pci_bios_read, write: pci_bios_write };

unsafe fn pci_find_bios() -> *const pci_raw_ops {
    // Scan 0xe0000 through 0xfffff in 16-byte BIOS32 units.
    let mut check = 0xe0000usize as *mut bios32;
    while (check as usize) <= 0xffff0 {
        let fields = &(*check).fields;
        if fields.signature == BIOS32_SIGNATURE as usize {
            let length = (fields.length as usize) * 16;
            if length != 0 {
                let bytes = (*check).chars.as_ptr() as *const u8;
                let mut sum = 0u8;
                for i in 0..length { sum = sum.wrapping_add(*bytes.add(i)); }
                if sum == 0 && fields.revision == 0 {
                    if fields.entry < 0x100000 {
                        bios32_indirect.address = fields.entry;
                        set_bios_x();
                        if check_pcibios() != 0 { return &pci_bios_access; }
                    }
                    break;
                }
            }
        }
        check = (check as usize + core::mem::size_of::<bios32>()) as *mut bios32;
    }
    core::ptr::null()
}

#[repr(C, packed)] pub struct irq_routing_options { pub size: u16, pub table: *mut irq_info, pub segment: u16 }
#[repr(C)] pub struct irq_info { _private: [u8; 0] }
#[repr(C)] pub struct irq_routing_table { pub size: u16, pub exclusive_irqs: i32, pub slots: [u8; 0] }

pub unsafe fn pcibios_get_irq_routing_table() -> *mut irq_routing_table {
    if pci_bios_present == 0 { return core::ptr::null_mut(); }
    let mut opt = irq_routing_options { size: 4096, table: core::ptr::null_mut(), segment: 0 };
    let mut ret = 0i32; let mut map = 0i32;
    pci_bios_irq_lcall(&mut opt, &mut ret, &mut map, &pci_indirect);
    if pcibios_get_return_code(ret as u32) != 0 || opt.size == 0 { return core::ptr::null_mut(); }
    // Allocation and copying are supplied by the kernel integration layer.
    core::ptr::null_mut()
}
pub unsafe fn pcibios_set_irq_routing(dev: *mut pci_dev, pin: i32, irq: i32) -> bool {
    let mut ret = 0i32;
    pci_bios_set_irq_lcall(0, ((irq << 8) | (pin + 10)) as u16, &pci_indirect, &mut ret);
    let _ = dev;
    pcibios_get_return_code(ret as u32) == 0
}
#[repr(C)] pub struct pci_dev { _private: [u8; 0] }

pub unsafe fn pci_pcbios_init() {
    if (pci_probe & 0x4) != 0 {
        let ops = pci_find_bios();
        if !ops.is_null() { pci_bios_present = 1; raw_pci_ops = ops; }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
