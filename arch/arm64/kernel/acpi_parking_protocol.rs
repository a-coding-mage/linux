// SPDX-License-Identifier: GPL-2.0-only
/*
 * ARM64 ACPI Parking Protocol implementation
 *
 * Authors: Lorenzo Pieralisi <lorenzo.pieralisi@arm.com>
 *          Mark Salter <msalter@redhat.com>
 */

// Dependencies supplied by the surrounding kernel translation.
type __le32 = u32;
type __le64 = u64;
type phys_addr_t = u64;

#[repr(C)]
pub struct acpi_madt_generic_interrupt {
    pub parked_address: phys_addr_t,
    pub parking_version: u8,
    pub cpu_interface_number: u8,
}

#[repr(C)]
pub struct parking_protocol_mailbox {
    pub cpu_id: __le32,
    pub reserved: __le32,
    pub entry_point: __le64,
}

#[repr(C)]
pub struct cpu_mailbox_entry {
    pub mailbox: *mut parking_protocol_mailbox,
    pub mailbox_addr: phys_addr_t,
    pub version: u8,
    pub gic_cpu_id: u8,
}

extern "C" {
    static mut cpu_mailbox_entries: [cpu_mailbox_entry; NR_CPUS];

    fn ioremap(addr: phys_addr_t, size: usize) -> *mut parking_protocol_mailbox;
    fn iounmap(addr: *mut parking_protocol_mailbox);
    fn readl_relaxed(addr: *const __le32) -> u32;
    fn readq_relaxed(addr: *const __le64) -> u64;
    fn writel_relaxed(value: u32, addr: *mut __le32);
    fn writeq_relaxed(value: u64, addr: *mut __le64);
    fn __pa_symbol(symbol: unsafe extern "C" fn()) -> u64;
    fn arch_send_wakeup_ipi(cpu: u32);
    fn smp_processor_id() -> i32;
}

extern "C" {
    static secondary_entry: unsafe extern "C" fn();
}

#[repr(C)]
pub struct cpu_operations {
    pub name: *const u8,
    pub cpu_init: Option<unsafe fn(u32) -> i32>,
    pub cpu_prepare: Option<unsafe fn(u32) -> i32>,
    pub cpu_boot: Option<unsafe fn(u32) -> i32>,
    pub cpu_postboot: Option<unsafe fn()>,
}

pub unsafe fn acpi_set_mailbox_entry(
    cpu: i32,
    p: *const acpi_madt_generic_interrupt,
) {
    let cpu_entry = &mut cpu_mailbox_entries[cpu as usize];

    cpu_entry.mailbox_addr = (*p).parked_address;
    cpu_entry.version = (*p).parking_version;
    cpu_entry.gic_cpu_id = (*p).cpu_interface_number;
}

pub unsafe fn acpi_parking_protocol_valid(cpu: i32) -> bool {
    let cpu_entry = &cpu_mailbox_entries[cpu as usize];

    cpu_entry.mailbox_addr != 0 && cpu_entry.version != 0
}

unsafe fn acpi_parking_protocol_cpu_init(cpu: u32) -> i32 {
    // pr_debug("%s: ACPI parked addr=%llx\n", __func__,
    //          cpu_mailbox_entries[cpu].mailbox_addr);
    let _ = cpu;
    0
}

unsafe fn acpi_parking_protocol_cpu_prepare(_cpu: u32) -> i32 {
    0
}

unsafe fn acpi_parking_protocol_cpu_boot(cpu: u32) -> i32 {
    let cpu_entry = &mut cpu_mailbox_entries[cpu as usize];
    let mailbox: *mut parking_protocol_mailbox;
    let cpu_id: u32;

    /*
     * Map mailbox memory with attribute device nGnRE (ie ioremap -
     * this deviates from the parking protocol specifications since
     * the mailboxes are required to be mapped nGnRnE; the attribute
     * discrepancy is harmless insofar as the protocol specification
     * is concerned).
     * If the mailbox is mistakenly allocated in the linear mapping
     * by FW ioremap will fail since the mapping will be prevented
     * by the kernel (it clashes with the linear mapping attributes
     * specifications).
     */
    mailbox = ioremap(cpu_entry.mailbox_addr, core::mem::size_of::<parking_protocol_mailbox>());
    if mailbox.is_null() {
        return -5; // -EIO
    }

    cpu_id = readl_relaxed(&(*mailbox).cpu_id);
    /*
     * Check if firmware has set-up the mailbox entry properly
     * before kickstarting the respective cpu.
     */
    if cpu_id != !0u32 {
        iounmap(mailbox);
        return -6; // -ENXIO
    }

    /* stash the mailbox address mapping to use it for further FW checks in the postboot method */
    cpu_entry.mailbox = mailbox;

    /*
     * We write the entry point and cpu id as LE regardless of the
     * native endianness of the kernel. Therefore, any boot-loaders
     * that read this address need to convert this address to the
     * Boot-Loader's endianness before jumping.
     */
    writeq_relaxed(__pa_symbol(secondary_entry), &mut (*mailbox).entry_point);
    writel_relaxed(cpu_entry.gic_cpu_id as u32, &mut (*mailbox).cpu_id);

    arch_send_wakeup_ipi(cpu);

    0
}

unsafe fn acpi_parking_protocol_cpu_postboot() {
    let cpu = smp_processor_id();
    let cpu_entry = &mut cpu_mailbox_entries[cpu as usize];
    let mailbox = cpu_entry.mailbox;
    let entry_point: u64;

    entry_point = readq_relaxed(&(*mailbox).entry_point);
    /*
     * Check if firmware has cleared the entry_point as expected
     * by the protocol specification.
     */
    // WARN_ON(entry_point);
    let _ = entry_point;
}

pub static acpi_parking_protocol_ops: cpu_operations = cpu_operations {
    name: b"parking-protocol\0".as_ptr(),
    cpu_init: Some(acpi_parking_protocol_cpu_init),
    cpu_prepare: Some(acpi_parking_protocol_cpu_prepare),
    cpu_boot: Some(acpi_parking_protocol_cpu_boot),
    cpu_postboot: Some(acpi_parking_protocol_cpu_postboot),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
