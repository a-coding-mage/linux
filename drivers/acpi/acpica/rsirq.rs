// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
/*******************************************************************************
 *
 * Module Name: rsirq - IRQ resource descriptors
 *
 ******************************************************************************/

// Dependencies are supplied by the surrounding ACPICA translation unit.

/* #define _COMPONENT ACPI_RESOURCES */
// ACPI_MODULE_NAME("rsirq")

/*******************************************************************************
 *
 * acpi_rs_get_irq
 *
 ******************************************************************************/
pub static mut acpi_rs_get_irq: [acpi_rsconvert_info; 9] = [
    acpi_rsconvert_info { info_type: ACPI_RSC_INITGET, value: ACPI_RESOURCE_TYPE_IRQ,
        flags: ACPI_RS_SIZE!(acpi_resource_irq), value2: ACPI_RSC_TABLE_SIZE!(acpi_rs_get_irq) },

    /* Get the IRQ mask (bytes 1:2) */
    acpi_rsconvert_info { info_type: ACPI_RSC_BITMASK16, value: ACPI_RS_OFFSET!(data.irq.interrupts[0]),
        flags: AML_OFFSET!(irq.irq_mask), value2: ACPI_RS_OFFSET!(data.irq.interrupt_count) },

    /* Set default flags (others are zero) */
    acpi_rsconvert_info { info_type: ACPI_RSC_SET8, value: ACPI_RS_OFFSET!(data.irq.triggering),
        flags: ACPI_EDGE_SENSITIVE, value2: 1 },

    /* Get the descriptor length (2 or 3 for IRQ descriptor) */
    acpi_rsconvert_info { info_type: ACPI_RSC_2BITFLAG, value: ACPI_RS_OFFSET!(data.irq.descriptor_length),
        flags: AML_OFFSET!(irq.descriptor_type), value2: 0 },

    /* All done if no flag byte present in descriptor */
    acpi_rsconvert_info { info_type: ACPI_RSC_EXIT_NE, value: ACPI_RSC_COMPARE_AML_LENGTH,
        flags: 0, value2: 3 },

    /* Get flags: Triggering[0], Polarity[3], Sharing[4], Wake[5] */
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.triggering),
        flags: AML_OFFSET!(irq.flags), value2: 0 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.polarity),
        flags: AML_OFFSET!(irq.flags), value2: 3 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.shareable),
        flags: AML_OFFSET!(irq.flags), value2: 4 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.wake_capable),
        flags: AML_OFFSET!(irq.flags), value2: 5 },
];

/*******************************************************************************
 *
 * acpi_rs_set_irq
 *
 ******************************************************************************/
pub static mut acpi_rs_set_irq: [acpi_rsconvert_info; 14] = [
    /* Start with a default descriptor of length 3 */
    acpi_rsconvert_info { info_type: ACPI_RSC_INITSET, value: ACPI_RESOURCE_NAME_IRQ,
        flags: core::mem::size_of::<aml_resource_irq>(), value2: ACPI_RSC_TABLE_SIZE!(acpi_rs_set_irq) },
    /* Convert interrupt list to 16-bit IRQ bitmask */
    acpi_rsconvert_info { info_type: ACPI_RSC_BITMASK16, value: ACPI_RS_OFFSET!(data.irq.interrupts[0]),
        flags: AML_OFFSET!(irq.irq_mask), value2: ACPI_RS_OFFSET!(data.irq.interrupt_count) },
    /* Set flags: Triggering[0], Polarity[3], Sharing[4], Wake[5] */
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.triggering), flags: AML_OFFSET!(irq.flags), value2: 0 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.polarity), flags: AML_OFFSET!(irq.flags), value2: 3 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.shareable), flags: AML_OFFSET!(irq.flags), value2: 4 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.irq.wake_capable), flags: AML_OFFSET!(irq.flags), value2: 5 },
    /* All done if the output descriptor length is required to be 3 */
    acpi_rsconvert_info { info_type: ACPI_RSC_EXIT_EQ, value: ACPI_RSC_COMPARE_VALUE, flags: ACPI_RS_OFFSET!(data.irq.descriptor_length), value2: 3 },
    /* Set length to 2 bytes (no flags byte) */
    acpi_rsconvert_info { info_type: ACPI_RSC_LENGTH, value: 0, flags: 0, value2: core::mem::size_of::<aml_resource_irq_noflags>() },
    /* All done if the output descriptor length is required to be 2. */
    acpi_rsconvert_info { info_type: ACPI_RSC_EXIT_EQ, value: ACPI_RSC_COMPARE_VALUE, flags: ACPI_RS_OFFSET!(data.irq.descriptor_length), value2: 2 },
    /* Reset length to 3 bytes (descriptor with flags byte) */
    acpi_rsconvert_info { info_type: ACPI_RSC_LENGTH, value: 0, flags: 0, value2: core::mem::size_of::<aml_resource_irq>() },
    /* Check if the flags byte is necessary. */
    acpi_rsconvert_info { info_type: ACPI_RSC_EXIT_NE, value: ACPI_RSC_COMPARE_VALUE, flags: ACPI_RS_OFFSET!(data.irq.triggering), value2: ACPI_EDGE_SENSITIVE },
    acpi_rsconvert_info { info_type: ACPI_RSC_EXIT_NE, value: ACPI_RSC_COMPARE_VALUE, flags: ACPI_RS_OFFSET!(data.irq.polarity), value2: ACPI_ACTIVE_HIGH },
    acpi_rsconvert_info { info_type: ACPI_RSC_EXIT_NE, value: ACPI_RSC_COMPARE_VALUE, flags: ACPI_RS_OFFSET!(data.irq.shareable), value2: ACPI_EXCLUSIVE },
    /* We can optimize to a 2-byte irq_no_flags() descriptor */
    acpi_rsconvert_info { info_type: ACPI_RSC_LENGTH, value: 0, flags: 0, value2: core::mem::size_of::<aml_resource_irq_noflags>() },
];

pub static mut acpi_rs_convert_ext_irq: [acpi_rsconvert_info; 10] = [
    acpi_rsconvert_info { info_type: ACPI_RSC_INITGET, value: ACPI_RESOURCE_TYPE_EXTENDED_IRQ, flags: ACPI_RS_SIZE!(acpi_resource_extended_irq), value2: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_ext_irq) },
    acpi_rsconvert_info { info_type: ACPI_RSC_INITSET, value: ACPI_RESOURCE_NAME_EXTENDED_IRQ, flags: core::mem::size_of::<aml_resource_extended_irq>(), value2: 0 },
    /* Flags: Producer/Consumer[0], Triggering[1], Polarity[2], Sharing[3], Wake[4] */
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.extended_irq.producer_consumer), flags: AML_OFFSET!(extended_irq.flags), value2: 0 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.extended_irq.triggering), flags: AML_OFFSET!(extended_irq.flags), value2: 1 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.extended_irq.polarity), flags: AML_OFFSET!(extended_irq.flags), value2: 2 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.extended_irq.shareable), flags: AML_OFFSET!(extended_irq.flags), value2: 3 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.extended_irq.wake_capable), flags: AML_OFFSET!(extended_irq.flags), value2: 4 },
    /* IRQ Table length (Byte4) */
    acpi_rsconvert_info { info_type: ACPI_RSC_COUNT, value: ACPI_RS_OFFSET!(data.extended_irq.interrupt_count), flags: AML_OFFSET!(extended_irq.interrupt_count), value2: core::mem::size_of::<u32>() },
    /* Copy every IRQ in the table, each is 32 bits */
    acpi_rsconvert_info { info_type: ACPI_RSC_MOVE32, value: ACPI_RS_OFFSET!(data.extended_irq.interrupts[0]), flags: AML_OFFSET!(extended_irq.interrupts[0]), value2: 0 },
    /* Optional resource_source (Index and String) */
    acpi_rsconvert_info { info_type: ACPI_RSC_SOURCEX, value: ACPI_RS_OFFSET!(data.extended_irq.resource_source), flags: ACPI_RS_OFFSET!(data.extended_irq.interrupts[0]), value2: core::mem::size_of::<aml_resource_extended_irq>() },
];

pub static mut acpi_rs_convert_dma: [acpi_rsconvert_info; 6] = [
    acpi_rsconvert_info { info_type: ACPI_RSC_INITGET, value: ACPI_RESOURCE_TYPE_DMA, flags: ACPI_RS_SIZE!(acpi_resource_dma), value2: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_dma) },
    acpi_rsconvert_info { info_type: ACPI_RSC_INITSET, value: ACPI_RESOURCE_NAME_DMA, flags: core::mem::size_of::<aml_resource_dma>(), value2: 0 },
    /* Flags: transfer preference, bus mastering, channel speed */
    acpi_rsconvert_info { info_type: ACPI_RSC_2BITFLAG, value: ACPI_RS_OFFSET!(data.dma.transfer), flags: AML_OFFSET!(dma.flags), value2: 0 },
    acpi_rsconvert_info { info_type: ACPI_RSC_1BITFLAG, value: ACPI_RS_OFFSET!(data.dma.bus_master), flags: AML_OFFSET!(dma.flags), value2: 2 },
    acpi_rsconvert_info { info_type: ACPI_RSC_2BITFLAG, value: ACPI_RS_OFFSET!(data.dma.type), flags: AML_OFFSET!(dma.flags), value2: 5 },
    /* DMA channel mask bits */
    acpi_rsconvert_info { info_type: ACPI_RSC_BITMASK, value: ACPI_RS_OFFSET!(data.dma.channels[0]), flags: AML_OFFSET!(dma.dma_channel_mask), value2: ACPI_RS_OFFSET!(data.dma.channel_count) },
];

pub static mut acpi_rs_convert_fixed_dma: [acpi_rsconvert_info; 4] = [
    acpi_rsconvert_info { info_type: ACPI_RSC_INITGET, value: ACPI_RESOURCE_TYPE_FIXED_DMA, flags: ACPI_RS_SIZE!(acpi_resource_fixed_dma), value2: ACPI_RSC_TABLE_SIZE!(acpi_rs_convert_fixed_dma) },
    acpi_rsconvert_info { info_type: ACPI_RSC_INITSET, value: ACPI_RESOURCE_NAME_FIXED_DMA, flags: core::mem::size_of::<aml_resource_fixed_dma>(), value2: 0 },
    /* These fields are contiguous in both the source and destination: request_lines, Channels */
    acpi_rsconvert_info { info_type: ACPI_RSC_MOVE16, value: ACPI_RS_OFFSET!(data.fixed_dma.request_lines), flags: AML_OFFSET!(fixed_dma.request_lines), value2: 2 },
    acpi_rsconvert_info { info_type: ACPI_RSC_MOVE8, value: ACPI_RS_OFFSET!(data.fixed_dma.width), flags: AML_OFFSET!(fixed_dma.width), value2: 1 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
