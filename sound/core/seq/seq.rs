// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  ALSA sequencer main module
 *  Copyright (c) 1998-1999 by Frank van de Pol <fvdpol@coil.demon.nl>
 */

/*
 * C dependencies:
 * linux/init.h, linux/module.h, linux/device.h
 * sound/core.h, sound/initval.h
 * sound/seq_kernel.h, seq_clientmgr.h, seq_memory.h, seq_queue.h
 * seq_lock.h, seq_timer.h, seq_system.h, seq_info.h
 * sound/minors.h, sound/seq_device.h
 */

use core::ffi::c_int;

extern "C" {
    static SNDRV_SEQ_CLIENT_DUMMY: c_int;
    static SNDRV_TIMER_CLASS_GLOBAL: c_int;
    static SNDRV_TIMER_SCLASS_NONE: c_int;
    static SNDRV_TIMER_GLOBAL_HRTIMER: c_int;
    static SNDRV_TIMER_GLOBAL_SYSTEM: c_int;

    fn client_init_data() -> c_int;
    fn snd_sequencer_device_init() -> c_int;
    fn snd_sequencer_device_done();
    fn snd_seq_info_init() -> c_int;
    fn snd_seq_info_done();
    fn snd_seq_system_client_init() -> c_int;
    fn snd_seq_system_client_done();
    fn snd_seq_autoload_init();
    fn snd_seq_autoload_exit();
    fn snd_seq_queues_delete();
}

#[cfg(CONFIG_SND_SEQ_DUMMY_MODULE)]
#[no_mangle]
pub static mut seq_client_load: [c_int; 15] = [
    unsafe { SNDRV_SEQ_CLIENT_DUMMY },
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
    -1,
];

#[cfg(not(CONFIG_SND_SEQ_DUMMY_MODULE))]
#[no_mangle]
pub static mut seq_client_load: [c_int; 15] = [-1; 15];

#[no_mangle]
pub static mut seq_default_timer_class: c_int = unsafe { SNDRV_TIMER_CLASS_GLOBAL };
#[no_mangle]
pub static mut seq_default_timer_sclass: c_int = unsafe { SNDRV_TIMER_SCLASS_NONE };
#[no_mangle]
pub static mut seq_default_timer_card: c_int = -1;

#[cfg(CONFIG_SND_SEQ_HRTIMER_DEFAULT)]
#[no_mangle]
pub static mut seq_default_timer_device: c_int = unsafe { SNDRV_TIMER_GLOBAL_HRTIMER };

#[cfg(not(CONFIG_SND_SEQ_HRTIMER_DEFAULT))]
#[no_mangle]
pub static mut seq_default_timer_device: c_int = unsafe { SNDRV_TIMER_GLOBAL_SYSTEM };

#[no_mangle]
pub static mut seq_default_timer_subdevice: c_int = 0;
#[no_mangle]
pub static mut seq_default_timer_resolution: c_int = 0; /* Hz */

/*
 * MODULE_AUTHOR("Frank van de Pol <fvdpol@coil.demon.nl>, Jaroslav Kysela <perex@perex.cz>");
 * MODULE_DESCRIPTION("Advanced Linux Sound Architecture sequencer.");
 * MODULE_LICENSE("GPL");
 *
 * module_param_array(seq_client_load, int, NULL, 0444);
 * MODULE_PARM_DESC(seq_client_load, "The numbers of global (system) clients to load through kmod.");
 * module_param(seq_default_timer_class, int, 0644);
 * MODULE_PARM_DESC(seq_default_timer_class, "The default timer class.");
 * module_param(seq_default_timer_sclass, int, 0644);
 * MODULE_PARM_DESC(seq_default_timer_sclass, "The default timer slave class.");
 * module_param(seq_default_timer_card, int, 0644);
 * MODULE_PARM_DESC(seq_default_timer_card, "The default timer card number.");
 * module_param(seq_default_timer_device, int, 0644);
 * MODULE_PARM_DESC(seq_default_timer_device, "The default timer device number.");
 * module_param(seq_default_timer_subdevice, int, 0644);
 * MODULE_PARM_DESC(seq_default_timer_subdevice, "The default timer subdevice number.");
 * module_param(seq_default_timer_resolution, int, 0644);
 * MODULE_PARM_DESC(seq_default_timer_resolution, "The default timer resolution in Hz.");
 *
 * MODULE_ALIAS_CHARDEV(CONFIG_SND_MAJOR, SNDRV_MINOR_SEQUENCER);
 * MODULE_ALIAS("devname:snd/seq");
 */

/*
 *  INIT PART
 */

unsafe extern "C" fn alsa_seq_init() -> c_int {
    let mut err: c_int;

    err = client_init_data();
    if err < 0 {
        return err;
    }

    /* register sequencer device */
    err = snd_sequencer_device_init();
    if err < 0 {
        return err;
    }

    /* register proc interface */
    err = snd_seq_info_init();
    if err < 0 {
        snd_sequencer_device_done();
        return err;
    }

    /* register our internal client */
    err = snd_seq_system_client_init();
    if err < 0 {
        snd_seq_info_done();
        snd_sequencer_device_done();
        return err;
    }

    snd_seq_autoload_init();
    0
}

unsafe extern "C" fn alsa_seq_exit() {
    /* unregister our internal client */
    snd_seq_system_client_done();

    /* unregister proc interface */
    snd_seq_info_done();

    /* delete timing queues */
    snd_seq_queues_delete();

    /* unregister sequencer device */
    snd_sequencer_device_done();

    snd_seq_autoload_exit();
}

/*
 * module_init(alsa_seq_init)
 * module_exit(alsa_seq_exit)
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
