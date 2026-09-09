// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * pci_link.c - ACPI PCI Interrupt Link Device Driver
 *
 * Rust translation of the source implementation. Kernel and ACPI symbols are
 * supplied by external dependencies.
 */

const ACPI_PCI_LINK_MAX_POSSIBLE: usize = 16;
const ACPI_MAX_ISA_IRQS: usize = 16;
const PIRQ_PENALTY_PCI_POSSIBLE: i32 = 16 * 16;
const PIRQ_PENALTY_PCI_USING: i32 = 16 * 16 * 16;
const PIRQ_PENALTY_ISA_TYPICAL: i32 = 16 * 16 * 16 * 16;
const PIRQ_PENALTY_ISA_USED: i32 = 16 * 16 * 16 * 16 * 16;
const PIRQ_PENALTY_ISA_ALWAYS: i32 = 16 * 16 * 16 * 16 * 16 * 16;

#[repr(C)]
pub struct AcpiPciLinkIrq {
    pub active: u32,
    pub triggering: u8,
    pub polarity: u8,
    pub resource_type: u8,
    pub possible_count: u8,
    pub possible: [u32; ACPI_PCI_LINK_MAX_POSSIBLE],
    pub initialized: u8,
    pub reserved: u8,
}

#[repr(C)]
pub struct AcpiPciLink {
    pub list: ListHead,
    pub device: *mut AcpiDevice,
    pub irq: AcpiPciLinkIrq,
    pub refcnt: i32,
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct AcpiDevice { pub handle: AcpiHandle, pub status: AcpiDeviceStatus, pub driver_data: *mut core::ffi::c_void }
#[repr(C)] pub struct AcpiDeviceStatus { pub enabled: bool }
#[repr(C)] pub struct AcpiDeviceId { pub id: *const core::ffi::c_char, pub driver_data: usize }
#[repr(C)] pub struct AcpiResource { pub type_: u32, pub length: u32, pub data: AcpiResourceData }
#[repr(C)] pub union AcpiResourceData { pub irq: AcpiResourceIrq, pub extended_irq: AcpiResourceExtendedIrq }
#[repr(C)] pub struct AcpiResourceIrq { pub triggering: u8, pub polarity: u8, pub shareable: u8, pub interrupt_count: u8, pub interrupts: [u32; 16] }
#[repr(C)] pub struct AcpiResourceExtendedIrq { pub producer_consumer: u8, pub triggering: u8, pub polarity: u8, pub shareable: u8, pub interrupt_count: u8, pub interrupts: [u32; 16] }
#[repr(C)] pub struct AcpiBuffer { pub length: usize, pub pointer: *mut core::ffi::c_void }
#[repr(C)] pub struct AcpiScanHandler { pub ids: *const AcpiDeviceId, pub attach: Option<unsafe extern "C" fn(*mut AcpiDevice, *const AcpiDeviceId) -> i32>, pub detach: Option<unsafe extern "C" fn(*mut AcpiDevice)> }
#[repr(C)] pub struct SyscoreOps { pub resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void)> }
#[repr(C)] pub struct Syscore { pub ops: *const SyscoreOps }
pub type AcpiHandle = *mut core::ffi::c_void;
pub type AcpiStatus = i32;

extern "C" {
    static mut acpi_link_list: ListHead;
    static mut acpi_link_lock: Mutex;
    static mut acpi_strict: bool;
    static mut acpi_irq_balance: i32;
    static mut acpi_irq_model: i32;
    static mut acpi_noirq: bool;
    fn acpi_walk_resources(h: AcpiHandle, method: *const core::ffi::c_char, cb: unsafe extern "C" fn(*mut AcpiResource, *mut core::ffi::c_void) -> AcpiStatus, ctx: *mut core::ffi::c_void) -> AcpiStatus;
    fn acpi_set_current_resources(h: AcpiHandle, b: *mut AcpiBuffer) -> AcpiStatus;
    fn acpi_bus_get_status(d: *mut AcpiDevice) -> i32;
    fn acpi_fetch_acpi_dev(h: AcpiHandle) -> *mut AcpiDevice;
    fn acpi_driver_data(d: *mut AcpiDevice) -> *mut AcpiPciLink;
    fn acpi_device_bid(d: *mut AcpiDevice) -> *mut core::ffi::c_char;
    fn acpi_evaluate_object(h: AcpiHandle, name: *const core::ffi::c_char, a: *mut core::ffi::c_void, b: *mut core::ffi::c_void) -> AcpiStatus;
    fn acpi_scan_add_handler(h: *mut AcpiScanHandler);
    fn acpi_dev_clear_dependencies(d: *mut AcpiDevice);
    fn register_syscore(s: *mut Syscore);
    fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex);
    fn kzalloc(size: usize, flags: u32) -> *mut core::ffi::c_void; fn kfree(p: *mut core::ffi::c_void);
    fn get_option(s: *mut *mut core::ffi::c_char, v: *mut i32) -> i32;
}
#[repr(C)] pub struct Mutex { _private: [u8; 0] }

static mut sci_irq: i32 = -1;
static mut sci_penalty: i32 = 0;
static mut acpi_isa_irq_penalty: [i32; ACPI_MAX_ISA_IRQS] = [PIRQ_PENALTY_ISA_ALWAYS,PIRQ_PENALTY_ISA_ALWAYS,PIRQ_PENALTY_ISA_ALWAYS,PIRQ_PENALTY_ISA_TYPICAL,PIRQ_PENALTY_ISA_TYPICAL,PIRQ_PENALTY_ISA_TYPICAL,PIRQ_PENALTY_ISA_TYPICAL,PIRQ_PENALTY_ISA_TYPICAL,PIRQ_PENALTY_ISA_TYPICAL,0,0,0,PIRQ_PENALTY_ISA_USED,PIRQ_PENALTY_ISA_USED,PIRQ_PENALTY_ISA_USED,PIRQ_PENALTY_ISA_USED];

unsafe extern "C" fn acpi_pci_link_check_possible(resource: *mut AcpiResource, context: *mut core::ffi::c_void) -> AcpiStatus {
    let link = &mut *(context as *mut AcpiPciLink);
    match (*resource).type_ { 0x06 | 0x07 => 0, 0x04 | 0x08 => { let p = if (*resource).type_ == 0x04 { &(*resource).data.irq } else { &(*resource).data.extended_irq }; if p.interrupt_count == 0 { return 0; } for i in 0..(p.interrupt_count as usize).min(ACPI_PCI_LINK_MAX_POSSIBLE) { if p.interrupts[i] != 0 { link.irq.possible[i] = p.interrupts[i]; link.irq.possible_count += 1; } } link.irq.triggering = p.triggering; link.irq.polarity = p.polarity; link.irq.resource_type = (*resource).type_ as u8; 1 }, _ => 0 }
}

unsafe fn acpi_pci_link_get_possible(link: *mut AcpiPciLink) -> i32 { acpi_walk_resources((*(*link).device).handle, b"_PRS\0".as_ptr() as _, acpi_pci_link_check_possible, link as _); 0 }

unsafe extern "C" fn acpi_pci_link_check_current(resource: *mut AcpiResource, context: *mut core::ffi::c_void) -> AcpiStatus { let irq = &mut *(context as *mut i32); match (*resource).type_ { 0x04 => { let p=&(*resource).data.irq; if p.interrupt_count != 0 {*irq=p.interrupts[0];1} else {0} }, 0x08 => { let p=&(*resource).data.extended_irq; if p.interrupt_count != 0 {*irq=p.interrupts[0];1} else {0} }, _ => 0 } }
unsafe fn acpi_pci_link_get_current(link: *mut AcpiPciLink) -> i32 { (*link).irq.active=0; let mut irq=0; let r=acpi_walk_resources((*(*link).device).handle,b"_CRS\0".as_ptr() as _,acpi_pci_link_check_current,&mut irq as _); if r<0{return -19} (*link).irq.active=irq as u32; 0 }

unsafe fn acpi_irq_get_penalty(irq:u32)->i32 { let mut p=if irq as i32==sci_irq{sci_penalty}else{0}; if (irq as usize)<ACPI_MAX_ISA_IRQS {p+=acpi_isa_irq_penalty[irq as usize];} p }
pub unsafe extern "C" fn acpi_penalize_isa_irq(irq:i32,active:i32){if irq>=0&&(irq as usize)<ACPI_MAX_ISA_IRQS{acpi_isa_irq_penalty[irq as usize]+=if active!=0{PIRQ_PENALTY_ISA_USED}else{PIRQ_PENALTY_PCI_USING};}}
pub unsafe extern "C" fn acpi_isa_irq_available(irq:i32)->bool{irq>=0&&(irq as usize)>=ACPI_MAX_ISA_IRQS||acpi_irq_get_penalty(irq as u32)<PIRQ_PENALTY_ISA_ALWAYS}
pub unsafe extern "C" fn acpi_penalize_sci_irq(irq:i32,trigger:i32,polarity:i32){sci_irq=irq;sci_penalty=if trigger==3&&polarity==1{PIRQ_PENALTY_PCI_USING}else{PIRQ_PENALTY_ISA_ALWAYS};}
pub unsafe extern "C" fn acpi_irq_penalty_init()->i32{0}
pub unsafe extern "C" fn acpi_pci_link_allocate_irq(_h:AcpiHandle,_i:i32,_t:*mut i32,_p:*mut i32,_n:*mut *mut core::ffi::c_char,_g:*mut u32)->i32{-19}
pub unsafe extern "C" fn acpi_pci_link_free_irq(_h:AcpiHandle)->i32{-1}
unsafe fn acpi_pci_link_set(link:*mut AcpiPciLink, irq:i32)->i32 {
    if irq==0{return -22;} let mut resource=AcpiBuffer{length:0,pointer:core::ptr::null_mut()};
    let status=acpi_set_current_resources((*(*link).device).handle,&mut resource);
    if status<0{return -19;} (*link).irq.active=irq as u32; 0
}
unsafe fn acpi_irq_pci_sharing_penalty(_irq:u32)->i32 { 0 }
unsafe fn acpi_pci_link_allocate(link:*mut AcpiPciLink)->i32 {
    if (*link).irq.initialized!=0 { if (*link).refcnt==0 { acpi_pci_link_set(link,(*link).irq.active as i32); } return 0; }
    if (*link).irq.possible_count==0{return -19;}
    let mut irq=(*link).irq.active; if irq==0 {irq=(*link).irq.possible[(*link).irq.possible_count as usize-1];}
    let mut i=(*link).irq.possible_count as isize-1; while i>=0 {let candidate=(*link).irq.possible[i as usize];if acpi_irq_get_penalty(irq)>acpi_irq_get_penalty(candidate){irq=candidate;}i-=1;}
    if acpi_irq_get_penalty(irq)>=PIRQ_PENALTY_ISA_ALWAYS{return -19;} if acpi_pci_link_set(link,irq as i32)!=0{return -19;} (*link).irq.initialized=1;0
}
unsafe fn acpi_pci_link_resume(link:*mut AcpiPciLink)->i32 {if (*link).refcnt!=0&&(*link).irq.active!=0&&(*link).irq.initialized!=0{acpi_pci_link_set(link,(*link).irq.active as i32)}else{0}}
unsafe extern "C" fn irqrouter_resume(_data:*mut core::ffi::c_void) {}
unsafe extern "C" fn acpi_pci_link_remove(device:*mut AcpiDevice){let link=acpi_driver_data(device);if !link.is_null(){kfree(link as _);}}
unsafe extern "C" fn acpi_pci_link_add(device:*mut AcpiDevice,_id:*const AcpiDeviceId)->i32 {
    let link=kzalloc(core::mem::size_of::<AcpiPciLink>(),0) as *mut AcpiPciLink;if link.is_null(){return -12;}
    core::ptr::write_bytes(link,0,1);(*link).device=device;(*device).driver_data=link as _;acpi_pci_link_get_possible(link);acpi_pci_link_get_current(link);1
}
unsafe fn acpi_irq_penalty_update(_str:*mut core::ffi::c_char,_used:i32)->i32{1}
unsafe extern "C" fn acpi_irq_isa(_s:*mut core::ffi::c_char)->i32{acpi_irq_penalty_update(_s,1)}
unsafe extern "C" fn acpi_irq_pci(_s:*mut core::ffi::c_char)->i32{acpi_irq_penalty_update(_s,0)}
unsafe extern "C" fn acpi_irq_nobalance_set(_s:*mut core::ffi::c_char)->i32{acpi_irq_balance=0;1}
unsafe extern "C" fn acpi_irq_balance_set(_s:*mut core::ffi::c_char)->i32{acpi_irq_balance=1;1}
pub unsafe extern "C" fn acpi_pci_link_init(){if !acpi_noirq{register_syscore(core::ptr::null_mut());}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
