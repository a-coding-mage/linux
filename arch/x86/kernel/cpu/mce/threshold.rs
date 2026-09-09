// SPDX-License-Identifier: GPL-2.0
/*
 * Common corrected MCE threshold handler code:
 */

// C includes provide the external kernel declarations used below.

static mut MCE_APEI_THR_LIMIT: u32 = 0;

pub unsafe fn mce_save_apei_thr_limit(thr_limit: u32) {
    MCE_APEI_THR_LIMIT = thr_limit;
    pr_info!("HEST corrected error threshold limit: %u\n", thr_limit);
}

pub unsafe fn mce_get_apei_thr_limit() -> u32 {
    MCE_APEI_THR_LIMIT
}

unsafe fn default_threshold_interrupt() {
    pr_err!("Unexpected threshold interrupt at vector %x\n", THRESHOLD_APIC_VECTOR);
}

pub static mut MCE_THRESHOLD_VECTOR: unsafe fn() = default_threshold_interrupt;

pub unsafe fn sysvec_threshold() {
    trace_threshold_apic_entry(THRESHOLD_APIC_VECTOR);
    inc_irq_stat(THRESHOLD_APIC);
    (MCE_THRESHOLD_VECTOR)();
    trace_threshold_apic_exit(THRESHOLD_APIC_VECTOR);
    apic_eoi();
}

// DEFINE_PER_CPU(struct mca_storm_desc, storm_desc);
// The per-CPU storage is supplied by the surrounding kernel translation.

pub unsafe fn mce_inherit_storm(bank: u32) {
    let storm = this_cpu_ptr(&storm_desc);

    /*
     * Previous CPU owning this bank had put it into storm mode,
     * but the precise history of that storm is unknown. Assume
     * the worst (all recent polls of the bank found a valid error
     * logged). This will avoid the new owner prematurely declaring
     * the storm has ended.
     */
    (*storm).banks[bank as usize].history = !0u64;
    (*storm).banks[bank as usize].timestamp = jiffies;
}

pub unsafe fn mce_get_storm_mode() -> bool {
    __this_cpu_read(storm_desc.poll_mode)
}

pub unsafe fn mce_set_storm_mode(storm: bool) {
    __this_cpu_write(storm_desc.poll_mode, storm);
}

unsafe fn mce_handle_storm(bank: u32, on: bool) {
    match boot_cpu_data.x86_vendor {
        X86_VENDOR_INTEL => mce_intel_handle_storm(bank, on),
        X86_VENDOR_AMD => mce_amd_handle_storm(bank, on),
        _ => {}
    }
}

pub unsafe fn cmci_storm_begin(bank: u32) {
    let storm = this_cpu_ptr(&storm_desc);

    __set_bit(bank, this_cpu_ptr(mce_poll_banks));
    (*storm).banks[bank as usize].in_storm_mode = true;

    /*
     * If this is the first bank on this CPU to enter storm mode
     * start polling.
     */
    (*storm).stormy_bank_count += 1;
    if (*storm).stormy_bank_count == 1 {
        mce_timer_kick(true);
    }
}

pub unsafe fn cmci_storm_end(bank: u32) {
    let storm = this_cpu_ptr(&storm_desc);

    if !mce_flags.amd_threshold {
        __clear_bit(bank, this_cpu_ptr(mce_poll_banks));
    }
    (*storm).banks[bank as usize].history = 0;
    (*storm).banks[bank as usize].in_storm_mode = false;

    /* If no banks left in storm mode, stop polling. */
    (*storm).stormy_bank_count -= 1;
    if (*storm).stormy_bank_count == 0 {
        mce_timer_kick(false);
    }
}

pub unsafe fn mce_track_storm(mce: *mut mce) {
    let storm = this_cpu_ptr(&storm_desc);
    let now = jiffies;
    let mut delta: unsigned_long;
    let mut shift: u32 = 1;
    let mut history: u64 = 0;
    let bank = (*mce).bank as usize;

    /* No tracking needed for banks that do not support CMCI */
    if (*storm).banks[bank].poll_only {
        return;
    }

    /*
     * When a bank is in storm mode it is polled once per second and
     * the history mask will record about the last minute of poll results.
     * If it is not in storm mode, then the bank is only checked when
     * there is a CMCI interrupt. Check how long it has been since
     * this bank was last checked, and adjust the amount of "shift"
     * to apply to history.
     */
    if !(*storm).banks[bank].in_storm_mode {
        delta = now - (*storm).banks[bank].timestamp;
        shift = (delta + HZ) / HZ;
    }

    /* If it has been a long time since the last poll, clear history. */
    if shift < NUM_HISTORY_BITS {
        history = (*storm).banks[bank].history << shift;
    }

    (*storm).banks[bank].timestamp = now;

    /* History keeps track of corrected errors. VAL=1 && UC=0 */
    if ((*mce).status & MCI_STATUS_VAL) != 0 && mce_is_correctable(mce) {
        history |= 1;
    }

    (*storm).banks[bank].history = history;

    if (*storm).banks[bank].in_storm_mode {
        if (history & GENMASK_ULL(STORM_END_POLL_THRESHOLD, 0)) != 0 {
            return;
        }
        printk_deferred!(KERN_NOTICE, "CPU%d BANK%d CMCI storm subsided\n", smp_processor_id(), (*mce).bank);
        mce_handle_storm((*mce).bank, false);
        cmci_storm_end((*mce).bank);
    } else {
        if hweight64(history) < STORM_BEGIN_THRESHOLD {
            return;
        }
        printk_deferred!(KERN_NOTICE, "CPU%d BANK%d CMCI storm detected\n", smp_processor_id(), (*mce).bank);
        mce_handle_storm((*mce).bank, true);
        cmci_storm_begin((*mce).bank);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
