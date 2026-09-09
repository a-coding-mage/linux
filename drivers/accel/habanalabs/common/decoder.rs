// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependency declarations and symbols are supplied by the surrounding kernel
// translation; the original source included "habanalabs.h" here.

const VCMD_CONTROL_OFFSET: u32 = 0x40; // SWREG16
const VCMD_IRQ_STATUS_OFFSET: u32 = 0x44; // SWREG17

const VCMD_IRQ_STATUS_ENDCMD_MASK: u32 = 0x1;
const VCMD_IRQ_STATUS_BUSERR_MASK: u32 = 0x2;
const VCMD_IRQ_STATUS_TIMEOUT_MASK: u32 = 0x4;
const VCMD_IRQ_STATUS_CMDERR_MASK: u32 = 0x8;
const VCMD_IRQ_STATUS_ABORT_MASK: u32 = 0x10;
const VCMD_IRQ_STATUS_RESET_MASK: u32 = 0x20;

unsafe fn dec_print_abnrm_intr_source(hdev: *mut hl_device, irq_status: u32) {
    let format = "abnormal interrupt source:%s%s%s%s%s%s\n";
    let mut intr_source: [&str; 6] = ["Unknown", "", "", "", "", ""];
    let mut i = 0usize;

    if irq_status == 0 {
        return;
    }

    if irq_status & VCMD_IRQ_STATUS_ENDCMD_MASK != 0 {
        intr_source[i] = " ENDCMD";
        i += 1;
    }
    if irq_status & VCMD_IRQ_STATUS_BUSERR_MASK != 0 {
        intr_source[i] = " BUSERR";
        i += 1;
    }
    if irq_status & VCMD_IRQ_STATUS_TIMEOUT_MASK != 0 {
        intr_source[i] = " TIMEOUT";
        i += 1;
    }
    if irq_status & VCMD_IRQ_STATUS_CMDERR_MASK != 0 {
        intr_source[i] = " CMDERR";
        i += 1;
    }
    if irq_status & VCMD_IRQ_STATUS_ABORT_MASK != 0 {
        intr_source[i] = " ABORT";
        i += 1;
    }
    if irq_status & VCMD_IRQ_STATUS_RESET_MASK != 0 {
        intr_source[i] = " RESET";
        i += 1;
    }

    dev_err(
        (*hdev).dev,
        format,
        intr_source[0],
        intr_source[1],
        intr_source[2],
        intr_source[3],
        intr_source[4],
        intr_source[5],
    );
}

unsafe fn dec_abnrm_intr_work(work: *mut work_struct) {
    let dec = container_of!(work, hl_dec, abnrm_intr_work);
    let hdev = (*dec).hdev;
    let mut irq_status: u32;
    let mut event_mask = 0;
    let mut reset_required = false;

    irq_status = RREG32((*dec).base_addr + VCMD_IRQ_STATUS_OFFSET);

    dev_err(
        (*hdev).dev,
        "Decoder abnormal interrupt %#x, core %d\n",
        irq_status,
        (*dec).core_id,
    );

    dec_print_abnrm_intr_source(hdev, irq_status);

    // Clear the interrupt
    WREG32((*dec).base_addr + VCMD_IRQ_STATUS_OFFSET, irq_status);

    // Flush the interrupt clear
    RREG32((*dec).base_addr + VCMD_IRQ_STATUS_OFFSET);

    if irq_status & VCMD_IRQ_STATUS_TIMEOUT_MASK != 0 {
        reset_required = true;
        event_mask |= HL_NOTIFIER_EVENT_GENERAL_HW_ERR;
    }

    if irq_status & VCMD_IRQ_STATUS_CMDERR_MASK != 0 {
        event_mask |= HL_NOTIFIER_EVENT_UNDEFINED_OPCODE;
    }

    if irq_status
        & (VCMD_IRQ_STATUS_ENDCMD_MASK
            | VCMD_IRQ_STATUS_BUSERR_MASK
            | VCMD_IRQ_STATUS_ABORT_MASK)
        != 0
    {
        event_mask |= HL_NOTIFIER_EVENT_USER_ENGINE_ERR;
    }

    if reset_required {
        event_mask |= HL_NOTIFIER_EVENT_DEVICE_RESET;
        hl_device_cond_reset(hdev, 0, event_mask);
    } else if event_mask != 0 {
        hl_notifier_event_send_all(hdev, event_mask);
    }
}

pub unsafe fn hl_dec_fini(hdev: *mut hl_device) {
    kfree((*hdev).dec);
}

pub unsafe fn hl_dec_init(hdev: *mut hl_device) -> i32 {
    let prop = &mut (*hdev).asic_prop;
    let mut dec: *mut hl_dec;
    let mut rc: i32;
    let mut j: i32;

    // if max core is 0, nothing to do
    if prop.max_dec == 0 {
        return 0;
    }

    (*hdev).dec = kzalloc_objs::<hl_dec>(prop.max_dec);
    if (*hdev).dec.is_null() {
        return -ENOMEM;
    }

    j = 0;
    while j < prop.max_dec {
        dec = (*hdev).dec.add(j as usize);

        (*dec).hdev = hdev;
        INIT_WORK(&mut (*dec).abnrm_intr_work, dec_abnrm_intr_work);
        (*dec).core_id = j;
        (*dec).base_addr = (*(*hdev).asic_funcs).get_dec_base_addr(hdev, j);
        if (*dec).base_addr == 0 {
            dev_err((*hdev).dev, "Invalid base address of decoder %d\n", j);
            rc = -EINVAL;
            hl_dec_fini(hdev);
            return rc;
        }

        j += 1;
    }

    0
}

pub unsafe fn hl_dec_ctx_fini(ctx: *mut hl_ctx) {
    let hdev = (*ctx).hdev;
    let prop = &mut (*hdev).asic_prop;
    let mut dec: *mut hl_dec;
    let mut j: i32;

    j = 0;
    while j < prop.max_dec {
        if prop.decoder_enabled_mask & BIT(j as u32) != 0 {
            dec = (*hdev).dec.add(j as usize);
            // Stop the decoder
            WREG32((*dec).base_addr + VCMD_CONTROL_OFFSET, 0);
        }
        j += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
