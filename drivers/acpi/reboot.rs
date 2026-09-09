// SPDX-License-Identifier: GPL-2.0

// pr_fmt(fmt) = "ACPI: " fmt

use crate::{
    acpi_disabled, acpi_gbl_FADT, acpi_reset, mdelay, pci_bus_write_config_byte,
    pci_find_bus, pr_debug, pr_warn_once, AcpiGenericAddress, AcpiTableFadt, PciBus,
    ACPI_ADR_SPACE_PCI_CONFIG, ACPI_ADR_SPACE_SYSTEM_IO, ACPI_ADR_SPACE_SYSTEM_MEMORY,
    ACPI_FADT_RESET_REGISTER,
};

#[cfg(feature = "CONFIG_PCI")]
unsafe fn acpi_pci_reboot(rr: *mut AcpiGenericAddress, reset_value: u8) {
    let mut devfn: u32;
    let bus0: *mut PciBus;

    /* The reset register can only live on bus 0. */
    bus0 = pci_find_bus(0, 0);
    if bus0.is_null() {
        return;
    }
    /* Form PCI device/function pair. */
    devfn = ((((*rr).address >> 32) & 0xffff) as u32) << 3
        | (((*rr).address >> 16) & 0xffff) as u32;
    pr_debug("Resetting with ACPI PCI RESET_REG.\n");
    /* Write the value that resets us. */
    pci_bus_write_config_byte(bus0, devfn, ((*rr).address & 0xffff) as u32, reset_value);
}

#[cfg(not(feature = "CONFIG_PCI"))]
unsafe fn acpi_pci_reboot(_rr: *mut AcpiGenericAddress, _reset_value: u8) {
    pr_warn_once("PCI configuration space access is not supported\n");
}

pub unsafe fn acpi_reboot() {
    let rr: *mut AcpiGenericAddress;
    let reset_value: u8;

    if acpi_disabled {
        return;
    }

    rr = &mut acpi_gbl_FADT.reset_register;

    /* ACPI reset register was only introduced with v2 of the FADT */

    if acpi_gbl_FADT.header.revision < 2 {
        return;
    }

    /* Is the reset register supported? The spec says we should be
     * checking the bit width and bit offset, but Windows ignores
     * these fields */
    if (acpi_gbl_FADT.flags & ACPI_FADT_RESET_REGISTER) == 0 {
        return;
    }

    reset_value = acpi_gbl_FADT.reset_value;

    /* The reset register can only exist in I/O, Memory or PCI config space
     * on a device on bus 0. */
    match (*rr).space_id {
        ACPI_ADR_SPACE_PCI_CONFIG => {
            acpi_pci_reboot(rr, reset_value);
        }
        ACPI_ADR_SPACE_SYSTEM_MEMORY | ACPI_ADR_SPACE_SYSTEM_IO => {
            pr_debug("ACPI MEMORY or I/O RESET_REG.\n");
            acpi_reset();
        }
        _ => {}
    }

    /*
     * Some platforms do not shut down immediately after writing to the
     * ACPI reset register, and this results in racing with the
     * subsequent reboot mechanism.
     *
     * The 15ms delay has been found to be long enough for the system
     * to reboot on the affected platforms.
     */
    mdelay(15);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
