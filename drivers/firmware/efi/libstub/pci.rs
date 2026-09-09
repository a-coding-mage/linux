// SPDX-License-Identifier: GPL-2.0
/*
 * PCI-related functions used by the EFI stub on multiple
 * architectures.
 *
 * Copyright 2019 Google, LLC
 */

// Dependencies supplied by the surrounding EFI stub and Linux PCI bindings.

pub unsafe fn efi_pci_disable_bridge_busmaster() {
    let mut pci_proto = EFI_PCI_IO_PROTOCOL_GUID;
    let mut pci_handle: *mut efi_handle_t = core::ptr::null_mut();
    let mut pci_handle_num: c_ulong = 0;
    let mut handle: efi_handle_t;
    let mut status: efi_status_t;
    let mut class: u16 = 0;
    let mut command: u16 = 0;

    status = efi_bs_call!(
        locate_handle_buffer,
        EFI_LOCATE_BY_PROTOCOL,
        &mut pci_proto,
        core::ptr::null_mut(),
        &mut pci_handle_num,
        &mut pci_handle
    );
    if status != EFI_SUCCESS {
        efi_err!("Failed to locate PCI I/O handles\n");
        return;
    }

    for_each_efi_handle!(handle, pci_handle, pci_handle_num, {
        let mut pci: *mut efi_pci_io_protocol_t = core::ptr::null_mut();
        let mut segment_nr: c_ulong = 0;
        let mut bus_nr: c_ulong = 0;
        let mut device_nr: c_ulong = 0;
        let mut func_nr: c_ulong = 0;

        status = efi_bs_call!(
            handle_protocol,
            handle,
            &mut pci_proto,
            &mut pci as *mut _ as *mut *mut core::ffi::c_void
        );
        if status != EFI_SUCCESS {
            continue;
        }

        /*
         * Disregard devices living on bus 0 - these are not behind a
         * bridge so no point in disconnecting them from their drivers.
         */
        status = efi_call_proto!(
            pci,
            get_location,
            &mut segment_nr,
            &mut bus_nr,
            &mut device_nr,
            &mut func_nr
        );
        if status != EFI_SUCCESS || bus_nr == 0 {
            continue;
        }

        /*
         * Don't disconnect VGA controllers so we don't risk losing
         * access to the framebuffer. Drivers for true PCIe graphics
         * controllers that are behind a PCIe root port do not use
         * DMA to implement the GOP framebuffer anyway [although they
         * may use it in their implementation of Gop->Blt()], and so
         * disabling DMA in the PCI bridge should not interfere with
         * normal operation of the device.
         */
        status = efi_call_proto!(
            pci,
            pci.read,
            EfiPciIoWidthUint16,
            PCI_CLASS_DEVICE,
            1,
            &mut class
        );
        if status != EFI_SUCCESS || class == PCI_CLASS_DISPLAY_VGA {
            continue;
        }

        /* Disconnect this handle from all its drivers */
        efi_bs_call!(disconnect_controller, handle, core::ptr::null_mut(), core::ptr::null_mut());
    });

    for_each_efi_handle!(handle, pci_handle, pci_handle_num, {
        let mut pci: *mut efi_pci_io_protocol_t = core::ptr::null_mut();

        status = efi_bs_call!(
            handle_protocol,
            handle,
            &mut pci_proto,
            &mut pci as *mut _ as *mut *mut core::ffi::c_void
        );
        if status != EFI_SUCCESS || pci.is_null() {
            continue;
        }

        status = efi_call_proto!(
            pci,
            pci.read,
            EfiPciIoWidthUint16,
            PCI_CLASS_DEVICE,
            1,
            &mut class
        );

        if status != EFI_SUCCESS || class != PCI_CLASS_BRIDGE_PCI {
            continue;
        }

        /* Disable busmastering */
        status = efi_call_proto!(
            pci,
            pci.read,
            EfiPciIoWidthUint16,
            PCI_COMMAND,
            1,
            &mut command
        );
        if status != EFI_SUCCESS || (command & PCI_COMMAND_MASTER) == 0 {
            continue;
        }

        command &= !PCI_COMMAND_MASTER;
        status = efi_call_proto!(
            pci,
            pci.write,
            EfiPciIoWidthUint16,
            PCI_COMMAND,
            1,
            &mut command
        );
        if status != EFI_SUCCESS {
            efi_err!("Failed to disable PCI busmastering\n");
        }
    });
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
