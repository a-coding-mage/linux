/* Copyright (c) 2012 Coraid, Inc.  See COPYING for GPL terms. */
/*
 * aoemain.c
 * Module initialization routines, discover timer
 */

// C dependencies supplied by the surrounding kernel/module environment:
// linux/hdreg.h, linux/blkdev.h, linux/module.h, linux/skbuff.h, and aoe.h.

// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Sam Hopkins <sah@coraid.com>");
// MODULE_DESCRIPTION("AoE block/char driver for 2.6.2 and newer 2.6 kernels");
// MODULE_VERSION(VERSION);

static mut timer: timer_list = unsafe { core::mem::zeroed() };
static mut aoe_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn discover_timer(t: *mut timer_list) {
    mod_timer(t, jiffies.wrapping_add(HZ.wrapping_mul(60))); /* one minute */

    aoecmd_cfg(0xffff, 0xff);
}

unsafe extern "C" fn aoe_exit() {
    timer_delete_sync(&raw mut timer);

    aoenet_exit();
    unregister_blkdev(AOE_MAJOR, DEVICE_NAME);
    aoecmd_exit();
    aoechr_exit();
    aoedev_exit();
    aoeblk_exit(); /* free cache after de-allocating bufs */
    destroy_workqueue(aoe_wq);
}

unsafe extern "C" fn aoe_init() -> i32 {
    let mut ret: i32;

    aoe_wq = alloc_workqueue(c"aoe_wq".as_ptr() as *const i8, WQ_PERCPU, 0);
    if aoe_wq.is_null() {
        return -ENOMEM;
    }

    ret = aoedev_init();
    if ret != 0 {
        destroy_workqueue(aoe_wq);
        printk(KERN_INFO, c"aoe: initialisation failure.\n".as_ptr() as *const i8);
        return ret;
    }
    ret = aoechr_init();
    if ret != 0 {
        aoedev_exit();
        destroy_workqueue(aoe_wq);
        printk(KERN_INFO, c"aoe: initialisation failure.\n".as_ptr() as *const i8);
        return ret;
    }
    ret = aoeblk_init();
    if ret != 0 {
        aoechr_exit();
        aoedev_exit();
        destroy_workqueue(aoe_wq);
        printk(KERN_INFO, c"aoe: initialisation failure.\n".as_ptr() as *const i8);
        return ret;
    }
    ret = aoenet_init();
    if ret != 0 {
        aoeblk_exit();
        aoechr_exit();
        aoedev_exit();
        destroy_workqueue(aoe_wq);
        printk(KERN_INFO, c"aoe: initialisation failure.\n".as_ptr() as *const i8);
        return ret;
    }
    ret = aoecmd_init();
    if ret != 0 {
        aoenet_exit();
        aoeblk_exit();
        aoechr_exit();
        aoedev_exit();
        destroy_workqueue(aoe_wq);
        printk(KERN_INFO, c"aoe: initialisation failure.\n".as_ptr() as *const i8);
        return ret;
    }
    ret = register_blkdev(AOE_MAJOR, DEVICE_NAME);
    if ret < 0 {
        aoecmd_exit();
        aoenet_exit();
        aoeblk_exit();
        aoechr_exit();
        aoedev_exit();
        destroy_workqueue(aoe_wq);
        printk(KERN_ERR, c"aoe: can't register major\n".as_ptr() as *const i8);
        printk(KERN_INFO, c"aoe: initialisation failure.\n".as_ptr() as *const i8);
        return ret;
    }
    printk(KERN_INFO, c"aoe: AoE v%s initialised.\n".as_ptr() as *const i8, VERSION);

    timer_setup(&raw mut timer, Some(discover_timer), 0);
    discover_timer(&raw mut timer);
    return 0;

}

// C module_init(aoe_init);
// C module_exit(aoe_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
