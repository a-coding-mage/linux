// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022 Linaro Ltd.
 * Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 */

pub fn mhi_ep_check_mhi_state(
    _mhi_cntrl: *mut mhi_ep_cntrl,
    cur_mhi_state: mhi_state,
    mhi_state: mhi_state,
) -> bool {
    if mhi_state == MHI_STATE_SYS_ERR {
        return true; // Allowed in any state
    }

    if mhi_state == MHI_STATE_READY {
        return cur_mhi_state == MHI_STATE_RESET;
    }

    if mhi_state == MHI_STATE_M0 {
        return cur_mhi_state == MHI_STATE_M3 || cur_mhi_state == MHI_STATE_READY;
    }

    if mhi_state == MHI_STATE_M3 {
        return cur_mhi_state == MHI_STATE_M0;
    }

    false
}

pub fn mhi_ep_set_mhi_state(mhi_cntrl: *mut mhi_ep_cntrl, mhi_state: mhi_state) -> i32 {
    unsafe {
        let dev = &(*(*mhi_cntrl).mhi_dev).dev;

        if !mhi_ep_check_mhi_state(mhi_cntrl, (*mhi_cntrl).mhi_state, mhi_state) {
            dev_err(
                dev,
                "MHI state change to %s from %s is not allowed!\n",
                mhi_state_str(mhi_state),
                mhi_state_str((*mhi_cntrl).mhi_state),
            );
            return -EACCES;
        }

        // TODO: Add support for M1 and M2 states
        if mhi_state == MHI_STATE_M1 || mhi_state == MHI_STATE_M2 {
            dev_err(
                dev,
                "MHI state (%s) not supported\n",
                mhi_state_str(mhi_state),
            );
            return -EOPNOTSUPP;
        }

        mhi_ep_mmio_masked_write(
            mhi_cntrl,
            EP_MHISTATUS,
            MHISTATUS_MHISTATE_MASK,
            mhi_state,
        );
        (*mhi_cntrl).mhi_state = mhi_state;

        if mhi_state == MHI_STATE_READY {
            mhi_ep_mmio_masked_write(mhi_cntrl, EP_MHISTATUS, MHISTATUS_READY_MASK, 1);
        }

        if mhi_state == MHI_STATE_SYS_ERR {
            mhi_ep_mmio_masked_write(mhi_cntrl, EP_MHISTATUS, MHISTATUS_SYSERR_MASK, 1);
        }

        0
    }
}

pub fn mhi_ep_set_m0_state(mhi_cntrl: *mut mhi_ep_cntrl) -> i32 {
    unsafe {
        let dev = &(*(*mhi_cntrl).mhi_dev).dev;
        let old_state: mhi_state;
        let ret: i32;

        // If MHI is in M3, resume suspended channels
        mutex_lock(&mut (*mhi_cntrl).state_lock);

        old_state = (*mhi_cntrl).mhi_state;
        if old_state == MHI_STATE_M3 {
            mhi_ep_resume_channels(mhi_cntrl);
        }

        ret = mhi_ep_set_mhi_state(mhi_cntrl, MHI_STATE_M0);
        if ret != 0 {
            mhi_ep_handle_syserr(mhi_cntrl);
            mutex_unlock(&mut (*mhi_cntrl).state_lock);
            return ret;
        }

        // Signal host that the device moved to M0
        ret = mhi_ep_send_state_change_event(mhi_cntrl, MHI_STATE_M0);
        if ret != 0 {
            dev_err(dev, "Failed sending M0 state change event\n");
            mutex_unlock(&mut (*mhi_cntrl).state_lock);
            return ret;
        }

        if old_state == MHI_STATE_READY {
            // Send AMSS EE event to host
            ret = mhi_ep_send_ee_event(mhi_cntrl, MHI_EE_AMSS);
            if ret != 0 {
                dev_err(dev, "Failed sending AMSS EE event\n");
                mutex_unlock(&mut (*mhi_cntrl).state_lock);
                return ret;
            }
        }

        mutex_unlock(&mut (*mhi_cntrl).state_lock);
        ret
    }
}

pub fn mhi_ep_set_m3_state(mhi_cntrl: *mut mhi_ep_cntrl) -> i32 {
    unsafe {
        let dev = &(*(*mhi_cntrl).mhi_dev).dev;
        let ret: i32;

        mutex_lock(&mut (*mhi_cntrl).state_lock);

        ret = mhi_ep_set_mhi_state(mhi_cntrl, MHI_STATE_M3);
        if ret != 0 {
            mhi_ep_handle_syserr(mhi_cntrl);
            mutex_unlock(&mut (*mhi_cntrl).state_lock);
            return ret;
        }

        mhi_ep_suspend_channels(mhi_cntrl);

        // Signal host that the device moved to M3
        ret = mhi_ep_send_state_change_event(mhi_cntrl, MHI_STATE_M3);
        if ret != 0 {
            dev_err(dev, "Failed sending M3 state change event\n");
            mutex_unlock(&mut (*mhi_cntrl).state_lock);
            return ret;
        }

        mutex_unlock(&mut (*mhi_cntrl).state_lock);
        ret
    }
}

pub fn mhi_ep_set_ready_state(mhi_cntrl: *mut mhi_ep_cntrl) -> i32 {
    unsafe {
        let dev = &(*(*mhi_cntrl).mhi_dev).dev;
        let mhi_state: mhi_state;
        let ret: i32;
        let is_ready: i32;

        mutex_lock(&mut (*mhi_cntrl).state_lock);

        // Ensure that the MHISTATUS is set to RESET by host
        mhi_state = mhi_ep_mmio_masked_read(
            mhi_cntrl,
            EP_MHISTATUS,
            MHISTATUS_MHISTATE_MASK,
        );
        is_ready = mhi_ep_mmio_masked_read(mhi_cntrl, EP_MHISTATUS, MHISTATUS_READY_MASK);

        if mhi_state != MHI_STATE_RESET || is_ready != 0 {
            dev_err(
                dev,
                "READY state transition failed. MHI host not in RESET state\n",
            );
            ret = -EIO;
            mutex_unlock(&mut (*mhi_cntrl).state_lock);
            return ret;
        }

        ret = mhi_ep_set_mhi_state(mhi_cntrl, MHI_STATE_READY);
        if ret != 0 {
            mhi_ep_handle_syserr(mhi_cntrl);
        }

        mutex_unlock(&mut (*mhi_cntrl).state_lock);
        ret
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
