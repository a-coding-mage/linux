/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2005 MIPS Technologies, Inc.  All rights reserved.
 * Copyright (C) 2013 Imagination Technologies Ltd.
 */

// Linux and MIPS declarations used by this implementation are supplied by
// the corresponding kernel dependencies.

static mut major: i32 = 0;

unsafe fn rtlx_dispatch() {
    if (read_c0_cause() & read_c0_status() & C_SW0) != 0 {
        do_IRQ(MIPS_CPU_IRQ_BASE + MIPS_CPU_RTLX_IRQ);
    }
}

/*
 * Interrupt handler may be called before rtlx_init has otherwise had
 * a chance to run.
 */
unsafe fn rtlx_interrupt(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let mut vpeflags: u32;
    let mut flags: usize = 0;

    local_irq_save(&mut flags);
    vpeflags = dvpe();
    set_c0_status(0x100 << MIPS_CPU_RTLX_IRQ);
    irq_enable_hazard();
    evpe(vpeflags);
    local_irq_restore(flags);

    for i in 0..RTLX_CHANNELS {
        wake_up(&mut channel_wqs[i].lx_queue);
        wake_up(&mut channel_wqs[i].rt_queue);
    }

    IRQ_HANDLED
}

static mut rtlx_irq_num: i32 = MIPS_CPU_IRQ_BASE + MIPS_CPU_RTLX_IRQ;

unsafe extern "C" fn _interrupt_sp() {
    let mut flags: usize = 0;

    local_irq_save(&mut flags);
    dvpe();
    settc(1);
    write_vpe_c0_cause(read_vpe_c0_cause() | C_SW0);
    evpe(EVPE_ENABLE);
    local_irq_restore(flags);
}

unsafe extern "C" fn rtlx_module_init() -> i32 {
    let mut dev: *mut device;
    let mut i: usize;
    let mut err: i32;

    if !cpu_has_mipsmt {
        pr_warn!("VPE loader: not a MIPS MT capable processor\n");
        return -ENODEV;
    }

    if aprp_cpu_index() == 0 {
        pr_warn!("No TCs reserved for AP/SP, not initializing RTLX.\n"
                "Pass maxtcs=<n> argument as kernel argument\n");
        return -ENODEV;
    }

    major = register_chrdev(0, RTLX_MODULE_NAME, &rtlx_fops);
    if major < 0 {
        pr_err!("rtlx_module_init: unable to register device\n");
        return major;
    }

    /* initialise the wait queues */
    for i in 0..RTLX_CHANNELS {
        init_waitqueue_head(&mut channel_wqs[i].rt_queue);
        init_waitqueue_head(&mut channel_wqs[i].lx_queue);
        atomic_set(&mut channel_wqs[i].in_open, 0);
        mutex_init(&mut channel_wqs[i].mutex);

        dev = device_create(&mut mt_class, core::ptr::null_mut(),
                            MKDEV(major, i as i32), core::ptr::null_mut(),
                            RTLX_MODULE_NAME, i);
        if IS_ERR(dev) {
            while i != 0 {
                i -= 1;
                device_destroy(&mut mt_class, MKDEV(major, i as i32));
            }

            err = PTR_ERR(dev);
            goto_out_chrdev:
            unregister_chrdev(major, RTLX_MODULE_NAME);
            return err;
        }
    }

    /* set up notifiers */
    rtlx_notify.start = Some(rtlx_starting);
    rtlx_notify.stop = Some(rtlx_stopping);
    vpe_notify(aprp_cpu_index(), &mut rtlx_notify);

    if cpu_has_vint {
        aprp_hook = Some(rtlx_dispatch);
    } else {
        pr_err!("APRP RTLX init on non-vectored-interrupt processor\n");
        err = -ENODEV;
        goto_out_class:
        for j in 0..RTLX_CHANNELS {
            device_destroy(&mut mt_class, MKDEV(major, j as i32));
        }
        goto_out_chrdev2:
        unregister_chrdev(major, RTLX_MODULE_NAME);
        return err;
    }

    err = request_irq(unsafe { rtlx_irq_num }, rtlx_interrupt, 0, "RTLX", rtlx);
    if err != 0 {
        for j in 0..RTLX_CHANNELS {
            device_destroy(&mut mt_class, MKDEV(major, j as i32));
        }
        unregister_chrdev(major, RTLX_MODULE_NAME);
        return err;
    }

    0
}

unsafe extern "C" fn rtlx_module_exit() {
    for i in 0..RTLX_CHANNELS {
        device_destroy(&mut mt_class, MKDEV(major, i as i32));
    }

    unregister_chrdev(major, RTLX_MODULE_NAME);

    aprp_hook = None;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
