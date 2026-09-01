// SPDX-License-Identifier: GPL-2.0-or-later
/* ALSA sequencer binding for UMP device */

/* Original C dependencies:
 * <linux/init.h>, <linux/slab.h>, <linux/errno.h>, <linux/mutex.h>,
 * <linux/string.h>, <linux/module.h>, <asm/byteorder.h>, <sound/core.h>,
 * <sound/ump.h>, <sound/seq_kernel.h>, <sound/seq_device.h>,
 * "seq_clientmgr.h", "seq_system.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

#[repr(C)]
pub struct seq_ump_client {
    pub ump: *mut snd_ump_endpoint, /* assigned endpoint */
    pub seq_client: c_int,          /* sequencer client id */
    pub opened: [c_int; 2],         /* current opens for each direction */
    pub out_rfile: snd_rawmidi_file, /* rawmidi for output */
    /* RCU-protected shadow of out_rfile.output for the delivery hot path;
     * out_rfile itself is only touched by open/close under open_mutex
     */
    pub out_substream: *mut snd_rawmidi_substream,
    pub input: seq_ump_input_buffer, /* input parser context */
    pub ump_info: [*mut c_void; SNDRV_UMP_MAX_BLOCKS + 1], /* shadow of seq client ump_info */
    pub group_notify_work: work_struct, /* FB change notification */
}

#[repr(C)]
pub struct seq_ump_group {
    _unused: [u8; 0],
}

pub const STR_IN: usize = SNDRV_RAWMIDI_STREAM_INPUT as usize;
pub const STR_OUT: usize = SNDRV_RAWMIDI_STREAM_OUTPUT as usize;

/* context for UMP input parsing, per EP */
#[repr(C)]
pub struct seq_ump_input_buffer {
    pub len: u8,     /* total length in words */
    pub pending: u8, /* pending words */
    pub type_: u8,   /* parsed UMP packet type */
    pub group: u8,   /* parsed UMP packet group */
    pub buf: [u32; 4], /* incoming UMP packet */
}

/* number of 32bit words for each UMP message type */
static mut ump_packet_words: [u8; 0x10] = [1, 1, 1, 2, 2, 4, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4];

/* conversion between UMP group and seq port;
 * assume the port number is equal with UMP group number (1-based)
 */
unsafe fn ump_group_to_seq_port(group: u8) -> u8 {
    group.wrapping_add(1)
}

/* process the incoming rawmidi stream */
unsafe extern "C" fn seq_ump_input_receive(ump: *mut snd_ump_endpoint, val: *const u32, words: c_int) {
    let client: *mut seq_ump_client = (*ump).seq_client as *mut seq_ump_client;
    let mut ev: snd_seq_ump_event = mem::zeroed();

    if (*client).opened[STR_IN] == 0 {
        return;
    }

    if ump_is_groupless_msg(ump_message_type(*val)) {
        ev.source.port = 0; /* UMP EP port */
    } else {
        ev.source.port = ump_group_to_seq_port(ump_message_group(*val)) as c_int;
    }
    ev.dest.client = SNDRV_SEQ_ADDRESS_SUBSCRIBERS;
    ev.flags = SNDRV_SEQ_EVENT_UMP;
    memcpy(ev.ump.as_mut_ptr() as *mut c_void, val as *const c_void, (words << 2) as usize);
    snd_seq_kernel_client_dispatch(
        (*client).seq_client,
        &mut ev as *mut snd_seq_ump_event as *mut snd_seq_event,
        true,
        0,
    );
}

/* process an input sequencer event; only deal with UMP types */
unsafe extern "C" fn seq_ump_process_event(
    ev: *mut snd_seq_event,
    _direct: c_int,
    private_data: *mut c_void,
    _atomic: c_int,
    _hop: c_int,
) -> c_int {
    let client: *mut seq_ump_client = private_data as *mut seq_ump_client;
    let substream: *mut snd_rawmidi_substream;
    let ump_ev: *mut snd_seq_ump_event;
    let type_: u8;
    let len: c_int;

    rcu_read_lock();
    substream = rcu_dereference((*client).out_substream);
    if substream.is_null() {
        rcu_read_unlock();
        return -ENODEV;
    }
    if !snd_seq_ev_is_ump(ev) {
        rcu_read_unlock();
        return 0; /* invalid event, skip */
    }
    ump_ev = ev as *mut snd_seq_ump_event;
    type_ = ump_message_type((*ump_ev).ump[0]);
    len = ump_packet_words[type_ as usize] as c_int;
    if len > 4 {
        rcu_read_unlock();
        return 0; /* invalid - skip */
    }
    snd_rawmidi_kernel_write(substream, (*ev).data.raw8.d.as_ptr(), len << 2);
    rcu_read_unlock();
    0
}

/* open the rawmidi */
unsafe fn seq_ump_client_open(client: *mut seq_ump_client, dir: c_int) -> c_int {
    let ump: *mut snd_ump_endpoint = (*client).ump;
    let mut err: c_int;

    mutex_lock(&mut (*ump).open_mutex);
    if dir == STR_OUT as c_int && (*client).opened[dir as usize] == 0 {
        /* out_rfile is only accessed under open_mutex; the delivery
         * path reads out_substream via RCU, so open into out_rfile
         * directly and publish the substream afterwards
         */
        err = snd_rawmidi_kernel_open(
            &mut (*ump).core,
            0,
            SNDRV_RAWMIDI_LFLG_OUTPUT | SNDRV_RAWMIDI_LFLG_APPEND,
            &mut (*client).out_rfile,
        );
        if err < 0 {
            mutex_unlock(&mut (*ump).open_mutex);
            return err;
        }
        rcu_assign_pointer(&mut (*client).out_substream, (*client).out_rfile.output);
    }
    (*client).opened[dir as usize] += 1;
    mutex_unlock(&mut (*ump).open_mutex);
    0
}

/* close the rawmidi */
unsafe fn seq_ump_client_close(client: *mut seq_ump_client, dir: c_int) -> c_int {
    let ump: *mut snd_ump_endpoint = (*client).ump;

    mutex_lock(&mut (*ump).open_mutex);
    (*client).opened[dir as usize] -= 1;
    if (*client).opened[dir as usize] == 0 {
        if dir == STR_OUT as c_int && !(*client).out_rfile.rmidi.is_null() {
            rcu_assign_pointer(&mut (*client).out_substream, ptr::null_mut());
            /* wait for a grace period so that no reader in the
             * delivery path is still writing to the substream
             * before it is released
             */
            synchronize_rcu();
            snd_rawmidi_kernel_release(&mut (*client).out_rfile);
            (*client).out_rfile = mem::zeroed();
        }
    }
    mutex_unlock(&mut (*ump).open_mutex);
    0
}

/* sequencer subscription ops for each client */
unsafe extern "C" fn seq_ump_subscribe(pdata: *mut c_void, _info: *mut snd_seq_port_subscribe) -> c_int {
    let client: *mut seq_ump_client = pdata as *mut seq_ump_client;

    seq_ump_client_open(client, STR_IN as c_int)
}

unsafe extern "C" fn seq_ump_unsubscribe(pdata: *mut c_void, _info: *mut snd_seq_port_subscribe) -> c_int {
    let client: *mut seq_ump_client = pdata as *mut seq_ump_client;

    seq_ump_client_close(client, STR_IN as c_int)
}

unsafe extern "C" fn seq_ump_use(pdata: *mut c_void, _info: *mut snd_seq_port_subscribe) -> c_int {
    let client: *mut seq_ump_client = pdata as *mut seq_ump_client;

    seq_ump_client_open(client, STR_OUT as c_int)
}

unsafe extern "C" fn seq_ump_unuse(pdata: *mut c_void, _info: *mut snd_seq_port_subscribe) -> c_int {
    let client: *mut seq_ump_client = pdata as *mut seq_ump_client;

    seq_ump_client_close(client, STR_OUT as c_int)
}

/* fill port_info from the given UMP EP and group info */
unsafe fn fill_port_info(
    port: *mut snd_seq_port_info,
    client: *mut seq_ump_client,
    group: *mut snd_ump_group,
) {
    let rawmidi_info: c_uint = (*(*client).ump).core.info_flags;

    (*port).addr.client = (*client).seq_client;
    (*port).addr.port = ump_group_to_seq_port((*group).group) as c_int;
    (*port).capability = 0;
    if rawmidi_info & SNDRV_RAWMIDI_INFO_OUTPUT != 0 {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_WRITE
            | SNDRV_SEQ_PORT_CAP_SYNC_WRITE
            | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
    }
    if rawmidi_info & SNDRV_RAWMIDI_INFO_INPUT != 0 {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_READ
            | SNDRV_SEQ_PORT_CAP_SYNC_READ
            | SNDRV_SEQ_PORT_CAP_SUBS_READ;
    }
    if rawmidi_info & SNDRV_RAWMIDI_INFO_DUPLEX != 0 {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_DUPLEX;
    }
    if (*group).dir_bits & (1 << STR_IN) != 0 {
        (*port).direction |= SNDRV_SEQ_PORT_DIR_INPUT;
    }
    if (*group).dir_bits & (1 << STR_OUT) != 0 {
        (*port).direction |= SNDRV_SEQ_PORT_DIR_OUTPUT;
    }
    (*port).ump_group = (*group).group.wrapping_add(1);
    if !(*group).active {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_INACTIVE;
    }
    if (*group).is_midi1 {
        (*port).flags |= SNDRV_SEQ_PORT_FLG_IS_MIDI1;
    }
    (*port).type_ = SNDRV_SEQ_PORT_TYPE_MIDI_GENERIC
        | SNDRV_SEQ_PORT_TYPE_MIDI_UMP
        | SNDRV_SEQ_PORT_TYPE_HARDWARE
        | SNDRV_SEQ_PORT_TYPE_PORT;
    (*port).midi_channels = 16;
    if (*group).name[0] != 0 {
        snprintf(
            (*port).name.as_mut_ptr(),
            (*port).name.len(),
            c"Group %d (%.53s)".as_ptr(),
            (*group).group as c_int + 1,
            (*group).name.as_ptr(),
        );
    } else {
        sprintf(
            (*port).name.as_mut_ptr(),
            c"Group %d".as_ptr(),
            (*group).group as c_int + 1,
        );
    }
}

/* skip non-existing group for static blocks */
unsafe fn skip_group(client: *mut seq_ump_client, group: *mut snd_ump_group) -> bool {
    !(*group).valid && ((*(*client).ump).info.flags & SNDRV_UMP_EP_INFO_STATIC_BLOCKS) != 0
}

/* create a new sequencer port per UMP group */
unsafe fn seq_ump_group_init(client: *mut seq_ump_client, group_index: c_int) -> c_int {
    let group: *mut snd_ump_group = &mut (*(*client).ump).groups[group_index as usize];
    let mut pcallbacks: snd_seq_port_callback;
    let port: *mut snd_seq_port_info = kzalloc_obj::<snd_seq_port_info>();

    if skip_group(client, group) {
        return 0;
    }

    if port.is_null() {
        return -ENOMEM;
    }

    fill_port_info(port, client, group);
    (*port).flags |= SNDRV_SEQ_PORT_FLG_GIVEN_PORT;
    pcallbacks = mem::zeroed();
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.private_data = client as *mut c_void;
    pcallbacks.subscribe = Some(seq_ump_subscribe);
    pcallbacks.unsubscribe = Some(seq_ump_unsubscribe);
    pcallbacks.use_ = Some(seq_ump_use);
    pcallbacks.unuse = Some(seq_ump_unuse);
    pcallbacks.event_input = Some(seq_ump_process_event);
    (*port).kernel = &mut pcallbacks;
    let ret = snd_seq_kernel_client_ctl((*client).seq_client, SNDRV_SEQ_IOCTL_CREATE_PORT, port as *mut c_void);
    kfree(port as *mut c_void);
    ret
}

/* update the sequencer ports; called from notify_fb_change callback */
unsafe fn update_port_infos(client: *mut seq_ump_client) {
    let mut i: c_int;
    let mut err: c_int;

    let old: *mut snd_seq_port_info = kzalloc_obj::<snd_seq_port_info>();
    let new: *mut snd_seq_port_info = kzalloc_obj::<snd_seq_port_info>();
    if old.is_null() || new.is_null() {
        if !old.is_null() {
            kfree(old as *mut c_void);
        }
        if !new.is_null() {
            kfree(new as *mut c_void);
        }
        return;
    }

    i = 0;
    while i < SNDRV_UMP_MAX_GROUPS as c_int {
        if skip_group(client, &mut (*(*client).ump).groups[i as usize]) {
            i += 1;
            continue;
        }

        (*old).addr.client = (*client).seq_client;
        (*old).addr.port = ump_group_to_seq_port(i as u8) as c_int;
        err = snd_seq_kernel_client_ctl(
            (*client).seq_client,
            SNDRV_SEQ_IOCTL_GET_PORT_INFO,
            old as *mut c_void,
        );
        if err < 0 {
            i += 1;
            continue;
        }
        fill_port_info(new, client, &mut (*(*client).ump).groups[i as usize]);
        if (*old).capability == (*new).capability && strcmp((*old).name.as_ptr(), (*new).name.as_ptr()) == 0 {
            i += 1;
            continue;
        }
        err = snd_seq_kernel_client_ctl(
            (*client).seq_client,
            SNDRV_SEQ_IOCTL_SET_PORT_INFO,
            new as *mut c_void,
        );
        if err < 0 {
            i += 1;
            continue;
        }
        i += 1;
    }

    kfree(old as *mut c_void);
    kfree(new as *mut c_void);
}

/* create a UMP Endpoint port */
unsafe fn create_ump_endpoint_port(client: *mut seq_ump_client) -> c_int {
    let mut pcallbacks: snd_seq_port_callback;
    let rawmidi_info: c_uint = (*(*client).ump).core.info_flags;
    let err: c_int;

    let port: *mut snd_seq_port_info = kzalloc_obj::<snd_seq_port_info>();
    if port.is_null() {
        return -ENOMEM;
    }

    (*port).addr.client = (*client).seq_client;
    (*port).addr.port = 0; /* fixed */
    (*port).flags = SNDRV_SEQ_PORT_FLG_GIVEN_PORT;
    (*port).capability = SNDRV_SEQ_PORT_CAP_UMP_ENDPOINT;
    if rawmidi_info & SNDRV_RAWMIDI_INFO_INPUT != 0 {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_READ
            | SNDRV_SEQ_PORT_CAP_SYNC_READ
            | SNDRV_SEQ_PORT_CAP_SUBS_READ;
        (*port).direction |= SNDRV_SEQ_PORT_DIR_INPUT;
    }
    if rawmidi_info & SNDRV_RAWMIDI_INFO_OUTPUT != 0 {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_WRITE
            | SNDRV_SEQ_PORT_CAP_SYNC_WRITE
            | SNDRV_SEQ_PORT_CAP_SUBS_WRITE;
        (*port).direction |= SNDRV_SEQ_PORT_DIR_OUTPUT;
    }
    if rawmidi_info & SNDRV_RAWMIDI_INFO_DUPLEX != 0 {
        (*port).capability |= SNDRV_SEQ_PORT_CAP_DUPLEX;
    }
    (*port).ump_group = 0; /* no associated group, no conversion */
    (*port).type_ = SNDRV_SEQ_PORT_TYPE_MIDI_UMP
        | SNDRV_SEQ_PORT_TYPE_HARDWARE
        | SNDRV_SEQ_PORT_TYPE_PORT;
    (*port).midi_channels = 16;
    strscpy((*port).name.as_mut_ptr(), c"MIDI 2.0".as_ptr(), (*port).name.len());
    pcallbacks = mem::zeroed();
    pcallbacks.owner = THIS_MODULE;
    pcallbacks.private_data = client as *mut c_void;
    if rawmidi_info & SNDRV_RAWMIDI_INFO_INPUT != 0 {
        pcallbacks.subscribe = Some(seq_ump_subscribe);
        pcallbacks.unsubscribe = Some(seq_ump_unsubscribe);
    }
    if rawmidi_info & SNDRV_RAWMIDI_INFO_OUTPUT != 0 {
        pcallbacks.use_ = Some(seq_ump_use);
        pcallbacks.unuse = Some(seq_ump_unuse);
        pcallbacks.event_input = Some(seq_ump_process_event);
    }
    (*port).kernel = &mut pcallbacks;
    err = snd_seq_kernel_client_ctl(
        (*client).seq_client,
        SNDRV_SEQ_IOCTL_CREATE_PORT,
        port as *mut c_void,
    );
    kfree(port as *mut c_void);
    err
}

/* release the client resources */
unsafe fn seq_ump_client_free(client: *mut seq_ump_client) {
    cancel_work_sync(&mut (*client).group_notify_work);

    if (*client).seq_client >= 0 {
        snd_seq_delete_kernel_client((*client).seq_client);
    }

    (*(*client).ump).seq_ops = ptr::null();
    (*(*client).ump).seq_client = ptr::null_mut();

    kfree(client as *mut c_void);
}

/* update the MIDI version for the given client */
unsafe fn setup_client_midi_version(client: *mut seq_ump_client) {
    let cptr: *mut snd_seq_client;

    cptr = snd_seq_kernel_client_get((*client).seq_client);
    if cptr.is_null() {
        return;
    }
    if (*(*client).ump).info.protocol & SNDRV_UMP_EP_INFO_PROTO_MIDI2 != 0 {
        (*cptr).midi_version = SNDRV_SEQ_CLIENT_UMP_MIDI_2_0;
    } else {
        (*cptr).midi_version = SNDRV_SEQ_CLIENT_UMP_MIDI_1_0;
    }
    snd_seq_kernel_client_put(cptr);
}

/* set up client's group_filter bitmap */
unsafe fn setup_client_group_filter(client: *mut seq_ump_client) {
    let cptr: *mut snd_seq_client;
    let mut filter: c_uint;
    let mut p: c_int;

    cptr = snd_seq_kernel_client_get((*client).seq_client);
    if cptr.is_null() {
        return;
    }
    filter = SND_SEQ_GROUP_FILTER_GROUPS; /* always allow groupless messages */
    p = 0;
    while p < SNDRV_UMP_MAX_GROUPS as c_int {
        if (*(*client).ump).groups[p as usize].active {
            filter &= !(1u32 << (p + 1));
        }
        p += 1;
    }
    (*cptr).group_filter = filter;
    snd_seq_kernel_client_put(cptr);
}

/* UMP group change notification */
unsafe extern "C" fn handle_group_notify(work: *mut work_struct) {
    let client: *mut seq_ump_client = container_of!(
        work,
        seq_ump_client,
        group_notify_work
    );

    update_port_infos(client);
    setup_client_group_filter(client);
}

/* UMP EP change notification */
unsafe extern "C" fn seq_ump_notify_ep_change(ump: *mut snd_ump_endpoint) -> c_int {
    let client: *mut seq_ump_client = (*ump).seq_client as *mut seq_ump_client;
    let cptr: *mut snd_seq_client;
    let client_id: c_int;

    if client.is_null() {
        return -ENODEV;
    }
    client_id = (*client).seq_client;
    cptr = snd_seq_kernel_client_get(client_id);
    if cptr.is_null() {
        return -ENODEV;
    }

    snd_seq_system_ump_notify(client_id, 0, SNDRV_SEQ_EVENT_UMP_EP_CHANGE, true);

    /* update sequencer client name if needed */
    if (*ump).core.name[0] != 0 && strcmp((*ump).core.name.as_ptr(), (*cptr).name.as_ptr()) != 0 {
        strscpy((*cptr).name.as_mut_ptr(), (*ump).core.name.as_ptr(), (*cptr).name.len());
        snd_seq_system_client_ev_client_change(client_id);
    }

    snd_seq_kernel_client_put(cptr);
    0
}

/* UMP FB change notification */
unsafe extern "C" fn seq_ump_notify_fb_change(
    ump: *mut snd_ump_endpoint,
    fb: *mut snd_ump_block,
) -> c_int {
    let client: *mut seq_ump_client = (*ump).seq_client as *mut seq_ump_client;

    if client.is_null() {
        return -ENODEV;
    }
    schedule_work(&mut (*client).group_notify_work);
    snd_seq_system_ump_notify(
        (*client).seq_client,
        (*fb).info.block_id,
        SNDRV_SEQ_EVENT_UMP_BLOCK_CHANGE,
        true,
    );
    0
}

/* UMP protocol change notification; just update the midi_version field */
unsafe extern "C" fn seq_ump_switch_protocol(ump: *mut snd_ump_endpoint) -> c_int {
    let client: *mut seq_ump_client = (*ump).seq_client as *mut seq_ump_client;

    if client.is_null() {
        return -ENODEV;
    }
    setup_client_midi_version(client);
    snd_seq_system_ump_notify(
        (*client).seq_client,
        0,
        SNDRV_SEQ_EVENT_UMP_EP_CHANGE,
        true,
    );
    0
}

static seq_ump_ops: snd_seq_ump_ops = snd_seq_ump_ops {
    input_receive: Some(seq_ump_input_receive),
    notify_ep_change: Some(seq_ump_notify_ep_change),
    notify_fb_change: Some(seq_ump_notify_fb_change),
    switch_protocol: Some(seq_ump_switch_protocol),
};

/* create a sequencer client and ports for the given UMP endpoint */
unsafe extern "C" fn snd_seq_ump_probe(dev: *mut snd_seq_device) -> c_int {
    let ump: *mut snd_ump_endpoint = (*dev).private_data as *mut snd_ump_endpoint;
    let card: *mut snd_card = (*dev).card;
    let client: *mut seq_ump_client;
    let mut fb: *mut snd_ump_block;
    let cptr: *mut snd_seq_client;
    let mut p: c_int;
    let mut err: c_int;

    client = kzalloc_obj::<seq_ump_client>();
    if client.is_null() {
        return -ENOMEM;
    }

    INIT_WORK(&mut (*client).group_notify_work, Some(handle_group_notify));
    (*client).ump = ump;

    (*client).seq_client = snd_seq_create_kernel_client(card, (*ump).core.device, (*ump).core.name.as_ptr());
    if (*client).seq_client < 0 {
        err = (*client).seq_client;
        seq_ump_client_free(client);
        return err;
    }

    (*client).ump_info[0] = &mut (*ump).info as *mut _ as *mut c_void;
    fb = list_first_entry_or_null(&mut (*ump).block_list);
    while !fb.is_null() {
        (*client).ump_info[(*fb).info.block_id as usize + 1] = &mut (*fb).info as *mut _ as *mut c_void;
        fb = list_next_entry_or_null(fb, &mut (*ump).block_list);
    }

    setup_client_midi_version(client);

    p = 0;
    while p < SNDRV_UMP_MAX_GROUPS as c_int {
        err = seq_ump_group_init(client, p);
        if err < 0 {
            seq_ump_client_free(client);
            return err;
        }
        p += 1;
    }

    setup_client_group_filter(client);

    err = create_ump_endpoint_port(client);
    if err < 0 {
        seq_ump_client_free(client);
        return err;
    }

    cptr = snd_seq_kernel_client_get((*client).seq_client);
    if cptr.is_null() {
        err = -EINVAL;
        seq_ump_client_free(client);
        return err;
    }
    (*cptr).ump_info = (*client).ump_info.as_mut_ptr();
    snd_seq_kernel_client_put(cptr);

    (*ump).seq_client = client as *mut c_void;
    (*ump).seq_ops = &seq_ump_ops;
    0
}

/* remove a sequencer client */
unsafe extern "C" fn snd_seq_ump_remove(dev: *mut snd_seq_device) {
    let ump: *mut snd_ump_endpoint = (*dev).private_data as *mut snd_ump_endpoint;

    if !(*ump).seq_client.is_null() {
        seq_ump_client_free((*ump).seq_client as *mut seq_ump_client);
    }
}

static mut seq_ump_driver: snd_seq_driver = snd_seq_driver {
    probe: Some(snd_seq_ump_probe),
    remove: Some(snd_seq_ump_remove),
    driver: device_driver {
        name: KBUILD_MODNAME,
    },
    id: SNDRV_SEQ_DEV_ID_UMP,
    argsize: 0,
};

/* module_snd_seq_driver(seq_ump_driver); */

/* MODULE_DESCRIPTION("ALSA sequencer client for UMP rawmidi"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
