// SPDX-License-Identifier: GPL-2.0
// C dependencies and build-time annotations are supplied by the surrounding kernel.

/*
 * PVH variables.
 *
 * pvh_bootparams and pvh_start_info need to live in a data segment since
 * they are used after startup_{32|64}, which clear .bss, are invoked.
 */
#[no_mangle]
pub static mut pvh_bootparams: boot_params = unsafe { core::mem::zeroed() };
#[no_mangle]
pub static mut pvh_start_info: hvm_start_info = unsafe { core::mem::zeroed() };

#[no_mangle]
pub static pvh_start_info_sz: usize = core::mem::size_of::<hvm_start_info>();

/*
 * Xen guests are able to obtain the memory map from the hypervisor via the
 * HYPERVISOR_memory_op hypercall.
 * If we are trying to boot a Xen PVH guest, it is expected that the kernel
 * will have been configured to provide an override for this routine to do
 * just that.
 */
#[no_mangle]
pub unsafe extern "C" fn mem_map_via_hcall(_ptr: *mut boot_params) {
    xen_raw_printk(c"Error: Could not find memory map\n".as_ptr());
    BUG();
}

unsafe fn init_pvh_bootparams(xen_guest: bool) {
    if pvh_start_info.version > 0 && pvh_start_info.memmap_entries != 0 {
        let mut ep: *mut hvm_memmap_table_entry =
            __va(pvh_start_info.memmap_paddr as usize) as *mut hvm_memmap_table_entry;
        let mut i: i32 = 0;

        pvh_bootparams.e820_entries = pvh_start_info.memmap_entries;

        while i < pvh_bootparams.e820_entries as i32 {
            (*(&mut pvh_bootparams.e820_table[i as usize])).addr = (*ep).addr;
            (*(&mut pvh_bootparams.e820_table[i as usize])).size = (*ep).size;
            (*(&mut pvh_bootparams.e820_table[i as usize])).type_ = (*ep).type_;
            i += 1;
            ep = ep.add(1);
        }
    } else if xen_guest {
        mem_map_via_hcall(&mut pvh_bootparams);
    } else {
        // Non-xen guests are not supported by version 0
        BUG();
    }

    if pvh_bootparams.e820_entries < E820_MAX_ENTRIES_ZEROPAGE - 1 {
        let entry = &mut pvh_bootparams.e820_table[pvh_bootparams.e820_entries as usize];
        entry.addr = ISA_START_ADDRESS;
        entry.size = ISA_END_ADDRESS - ISA_START_ADDRESS;
        entry.type_ = E820_TYPE_RESERVED;
        pvh_bootparams.e820_entries += 1;
    } else {
        xen_raw_printk(c"Warning: Can fit ISA range into e820\n".as_ptr());
    }

    pvh_bootparams.hdr.cmd_line_ptr = pvh_start_info.cmdline_paddr;

    /* The first module is always ramdisk. */
    if pvh_start_info.nr_modules != 0 {
        let modaddr = __va(pvh_start_info.modlist_paddr as usize) as *mut hvm_modlist_entry;
        pvh_bootparams.hdr.ramdisk_image = (*modaddr).paddr;
        pvh_bootparams.hdr.ramdisk_size = (*modaddr).size;
    }

    /*
     * See Documentation/arch/x86/boot.rst.
     *
     * Version 2.12 supports Xen entry point but we will use default x86/PC
     * environment (i.e. hardware_subarch 0).
     */
    pvh_bootparams.hdr.version = (2u16 << 8) | 12;
    pvh_bootparams.hdr.type_of_loader = (((if xen_guest { 0x9 } else { 0xb }) << 4) | 0) as _;

    pvh_bootparams.acpi_rsdp_addr = pvh_start_info.rsdp_paddr;
}

/* If booting a Xen PVH guest, the kernel supplies the required override. */
#[no_mangle]
pub unsafe extern "C" fn xen_pvh_init(boot_params: *mut boot_params) {
    xen_raw_printk(c"Error: Missing xen PVH initialization\n".as_ptr());
    BUG();
}

unsafe fn hypervisor_specific_init(xen_guest: bool) {
    if xen_guest {
        xen_pvh_init(&mut pvh_bootparams);
    }
}

/* This routine and its callees must not use .bss before it is cleared. */
#[no_mangle]
pub unsafe extern "C" fn xen_prepare_pvh() {
    let msr: u32 = xen_cpuid_base();
    let xen_guest: bool = msr != 0;

    if pvh_start_info.magic != XEN_HVM_START_MAGIC_VALUE {
        xen_raw_printk(c"Error: Unexpected magic value (0x%08x)\n".as_ptr(), pvh_start_info.magic);
        BUG();
    }

    /* Must not compile to a call to instrumented memset(). */
    core::ptr::write_bytes(
        &mut pvh_bootparams as *mut boot_params as *mut u8,
        0,
        core::mem::size_of::<boot_params>(),
    );

    hypervisor_specific_init(xen_guest);
    init_pvh_bootparams(xen_guest);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
