/******************************************************************************
 * tpmif.h
 *
 * TPM I/O interface for Xen guest OSes, v2
 *
 * This file is in the public domain.
 *
 */

/*
 * Xenbus state machine
 *
 * Device open:
 *   1. Both ends start in XenbusStateInitialising
 *   2. Backend transitions to InitWait (frontend does not wait on this step)
 *   3. Frontend populates ring-ref, event-channel, feature-protocol-v2
 *   4. Frontend transitions to Initialised
 *   5. Backend maps grant and event channel, verifies feature-protocol-v2
 *   6. Backend transitions to Connected
 *   7. Frontend verifies feature-protocol-v2, transitions to Connected
 *
 * Device close:
 *   1. State is changed to XenbusStateClosing
 *   2. Frontend transitions to Closed
 *   3. Backend unmaps grant and event, changes state to InitWait
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum vtpm_shared_page_state {
    VTPM_STATE_IDLE,   /* no contents / vTPM idle / cancel complete */
    VTPM_STATE_SUBMIT, /* request ready / vTPM working */
    VTPM_STATE_FINISH, /* response ready / vTPM idle */
    VTPM_STATE_CANCEL, /* cancel requested / vTPM working */
}
/* The backend should only change state to IDLE or FINISH, while the
 * frontend should only change to SUBMIT or CANCEL. */

#[repr(C)]
pub struct vtpm_shared_page {
    pub length: u32, /* request/response length in bytes */

    pub state: u8,    /* enum vtpm_shared_page_state */
    pub locality: u8, /* for the current request */
    pub pad: u8,

    pub nr_extra_pages: u8, /* extra pages for long packets; may be zero */
    pub extra_pages: [u32; 0], /* grant IDs; length in nr_extra_pages */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
