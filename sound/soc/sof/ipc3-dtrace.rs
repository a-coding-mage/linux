// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>

// Dependencies from: <linux/debugfs.h>, <linux/sched/signal.h>,
// "sof-priv.h", "sof-audio.h", "ops.h", "sof-utils.h", "ipc3-priv.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, MaybeUninit};
use core::ptr;

const TRACE_FILTER_ELEMENTS_PER_ENTRY: c_int = 4;
const TRACE_FILTER_MAX_CONFIG_STRING_LENGTH: usize = 1024;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum sof_dtrace_state {
    SOF_DTRACE_DISABLED,
    SOF_DTRACE_STOPPED,
    SOF_DTRACE_INITIALIZING,
    SOF_DTRACE_ENABLED,
}

#[repr(C)]
struct sof_dtrace_priv {
    dmatb: snd_dma_buffer,
    dmatp: snd_dma_buffer,
    dma_trace_pages: c_int,
    trace_sleep: wait_queue_head_t,
    host_offset: u32,
    dtrace_error: bool,
    dtrace_draining: bool,
    dtrace_state: sof_dtrace_state,
}

unsafe fn trace_pos_update_expected(priv_: *mut sof_dtrace_priv) -> bool {
    if (*priv_).dtrace_state == sof_dtrace_state::SOF_DTRACE_ENABLED
        || (*priv_).dtrace_state == sof_dtrace_state::SOF_DTRACE_INITIALIZING
    {
        return true;
    }

    false
}

unsafe fn trace_filter_append_elem(
    _sdev: *mut snd_sof_dev,
    key: u32,
    value: u32,
    elem_list: *mut sof_ipc_trace_filter_elem,
    capacity: c_int,
    counter: *mut c_int,
) -> c_int {
    if *counter >= capacity {
        return -ENOMEM;
    }

    (*elem_list.add(*counter as usize)).key = key;
    (*elem_list.add(*counter as usize)).value = value;
    *counter += 1;

    0
}

unsafe fn trace_filter_parse_entry(
    sdev: *mut snd_sof_dev,
    line: *const c_char,
    elem: *mut sof_ipc_trace_filter_elem,
    capacity: c_int,
    counter: *mut c_int,
) -> c_int {
    let mut log_level: c_int = 0;
    let mut pipe_id: c_int = 0;
    let mut comp_id: c_int = 0;
    let mut read: c_int = 0;
    let mut ret: c_int;
    let len = strlen(line) as c_int;
    let mut cnt = *counter;
    let mut uuid_id: u32 = 0;

    /* ignore empty content */
    ret = sscanf(line, c_str!(" %n"), &mut read);
    if ret == 0 && read == len {
        return len;
    }

    ret = sscanf(
        line,
        c_str!(" %d %x %d %d %n"),
        &mut log_level,
        &mut uuid_id,
        &mut pipe_id,
        &mut comp_id,
        &mut read,
    );
    if ret != TRACE_FILTER_ELEMENTS_PER_ENTRY || read != len {
        dev_err((*sdev).dev, c_str!("Invalid trace filter entry '%s'\n"), line);
        return -EINVAL;
    }

    if uuid_id > 0 {
        ret = trace_filter_append_elem(
            sdev,
            SOF_IPC_TRACE_FILTER_ELEM_BY_UUID,
            uuid_id,
            elem,
            capacity,
            &mut cnt,
        );
        if ret != 0 {
            return ret;
        }
    }
    if pipe_id >= 0 {
        ret = trace_filter_append_elem(
            sdev,
            SOF_IPC_TRACE_FILTER_ELEM_BY_PIPE,
            pipe_id as u32,
            elem,
            capacity,
            &mut cnt,
        );
        if ret != 0 {
            return ret;
        }
    }
    if comp_id >= 0 {
        ret = trace_filter_append_elem(
            sdev,
            SOF_IPC_TRACE_FILTER_ELEM_BY_COMP,
            comp_id as u32,
            elem,
            capacity,
            &mut cnt,
        );
        if ret != 0 {
            return ret;
        }
    }

    ret = trace_filter_append_elem(
        sdev,
        SOF_IPC_TRACE_FILTER_ELEM_SET_LEVEL | SOF_IPC_TRACE_FILTER_ELEM_FIN,
        log_level as u32,
        elem,
        capacity,
        &mut cnt,
    );
    if ret != 0 {
        return ret;
    }

    /* update counter only when parsing whole entry passed */
    *counter = cnt;

    len
}

unsafe fn trace_filter_parse(
    sdev: *mut snd_sof_dev,
    mut string: *mut c_char,
    out_elem_cnt: *mut c_int,
    out: *mut *mut sof_ipc_trace_filter_elem,
) -> c_int {
    static ENTRY_DELIMITER: [c_char; 2] = [b';' as c_char, 0];
    let mut entry = string;
    let mut capacity: c_int = 0;
    let mut entry_len: c_int;
    let mut cnt: c_int = 0;

    /*
     * Each entry contains at least 1, up to TRACE_FILTER_ELEMENTS_PER_ENTRY
     * IPC elements, depending on content. Calculate IPC elements capacity
     * for the input string where each element is set.
     */
    while !entry.is_null() {
        capacity += TRACE_FILTER_ELEMENTS_PER_ENTRY;
        entry = strchr(entry.add(1), ENTRY_DELIMITER[0]);
    }
    *out = kmalloc_objs::<sof_ipc_trace_filter_elem>(capacity as usize);
    if (*out).is_null() {
        return -ENOMEM;
    }

    /* split input string by ';', and parse each entry separately in trace_filter_parse_entry */
    loop {
        entry = strsep(&mut string, ENTRY_DELIMITER.as_ptr());
        if entry.is_null() {
            break;
        }
        entry_len = trace_filter_parse_entry(sdev, entry, *out, capacity, &mut cnt);
        if entry_len < 0 {
            dev_err(
                (*sdev).dev,
                c_str!("Parsing filter entry '%s' failed with %d\n"),
                entry,
                entry_len,
            );
            return -EINVAL;
        }
    }

    *out_elem_cnt = cnt;

    0
}

unsafe fn ipc3_trace_update_filter(
    sdev: *mut snd_sof_dev,
    num_elems: c_int,
    elems: *mut sof_ipc_trace_filter_elem,
) -> c_int {
    let mut msg: *mut sof_ipc_trace_filter;
    let size: usize;
    let mut ret: c_int;

    size = struct_size_trace_filter(num_elems as usize);
    if size > SOF_IPC_MSG_MAX_SIZE {
        return -EINVAL;
    }

    msg = kmalloc(size, GFP_KERNEL) as *mut sof_ipc_trace_filter;
    if msg.is_null() {
        return -ENOMEM;
    }

    (*msg).hdr.size = size as u32;
    (*msg).hdr.cmd = SOF_IPC_GLB_TRACE_MSG | SOF_IPC_TRACE_FILTER_UPDATE;
    (*msg).elem_cnt = num_elems as u32;
    memcpy(
        (*msg).elems.as_mut_ptr() as *mut c_void,
        elems as *const c_void,
        num_elems as usize * size_of::<sof_ipc_trace_filter_elem>(),
    );

    ret = pm_runtime_resume_and_get((*sdev).dev);
    if ret < 0 && ret != -EACCES {
        dev_err((*sdev).dev, c_str!("enabling device failed: %d\n"), ret);
        kfree(msg as *mut c_void);
        return ret;
    }

    /* Make sure the DSP/firmware is booted up */
    ret = snd_sof_boot_dsp_firmware(sdev);
    if ret == 0 {
        ret = sof_ipc_tx_message_no_reply((*sdev).ipc, msg as *mut c_void, (*msg).hdr.size);
    }

    pm_runtime_put_autosuspend((*sdev).dev);

    kfree(msg as *mut c_void);
    ret
}

unsafe extern "C" fn dfsentry_trace_filter_write(
    file: *mut file,
    from: *const c_char,
    count: usize,
    _ppos: *mut loff_t,
) -> ssize_t {
    let dfse = (*file).private_data as *mut snd_sof_dfsentry;
    let mut elems: *mut sof_ipc_trace_filter_elem = ptr::null_mut();
    let sdev = (*dfse).sdev;
    let mut num_elems: c_int = 0;
    let string: *mut c_char;
    let mut ret: c_int;

    if count > TRACE_FILTER_MAX_CONFIG_STRING_LENGTH {
        dev_err(
            (*sdev).dev,
            c_str!("%s too long input, %zu > %d\n"),
            c_str!("dfsentry_trace_filter_write"),
            count,
            TRACE_FILTER_MAX_CONFIG_STRING_LENGTH as c_int,
        );
        return -EINVAL as ssize_t;
    }

    string = memdup_user_nul(from as *const c_void, count) as *mut c_char;
    if IS_ERR(string as *const c_void) {
        return PTR_ERR(string as *const c_void) as ssize_t;
    }

    ret = trace_filter_parse(sdev, string, &mut num_elems, &mut elems);
    if ret < 0 {
        kfree(string as *mut c_void);
        kfree(elems as *mut c_void);
        return ret as ssize_t;
    }

    if num_elems != 0 {
        ret = ipc3_trace_update_filter(sdev, num_elems, elems);
        if ret < 0 {
            dev_err((*sdev).dev, c_str!("Filter update failed: %d\n"), ret);
            kfree(string as *mut c_void);
            kfree(elems as *mut c_void);
            return ret as ssize_t;
        }
    }
    ret = count as c_int;

    kfree(string as *mut c_void);
    kfree(elems as *mut c_void);
    ret as ssize_t
}

static sof_dfs_trace_filter_fops: file_operations = file_operations {
    open: Some(simple_open),
    write: Some(dfsentry_trace_filter_write),
    llseek: Some(default_llseek),
    ..file_operations::zeroed()
};

unsafe fn debugfs_create_trace_filter(sdev: *mut snd_sof_dev) -> c_int {
    let dfse: *mut snd_sof_dfsentry;

    dfse = devm_kzalloc((*sdev).dev, size_of::<snd_sof_dfsentry>(), GFP_KERNEL) as *mut snd_sof_dfsentry;
    if dfse.is_null() {
        return -ENOMEM;
    }

    (*dfse).sdev = sdev;
    (*dfse).type_ = SOF_DFSENTRY_TYPE_BUF;

    debugfs_create_file(
        c_str!("filter"),
        0o200,
        (*sdev).debugfs_root,
        dfse as *mut c_void,
        &sof_dfs_trace_filter_fops,
    );
    /* add to dfsentry list */
    list_add(&mut (*dfse).list, &mut (*sdev).dfsentry_list);

    0
}

unsafe fn sof_dtrace_set_host_offset(priv_: *mut sof_dtrace_priv, new_offset: u32) -> bool {
    let host_offset: u32 = READ_ONCE(&(*priv_).host_offset);

    if host_offset != new_offset {
        /* This is a bit paranoid and unlikely that it is needed */
        let ret: u32 = cmpxchg(&mut (*priv_).host_offset, host_offset, new_offset);

        if ret == host_offset {
            return true;
        }
    }

    false
}

unsafe fn sof_dtrace_avail(sdev: *mut snd_sof_dev, pos: loff_t, buffer_size: usize) -> usize {
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;
    let host_offset: loff_t = READ_ONCE(&(*priv_).host_offset) as loff_t;

    /*
     * If host offset is less than local pos, it means write pointer of
     * host DMA buffer has been wrapped. We should output the trace data
     * at the end of host DMA buffer at first.
     */
    if host_offset < pos {
        return buffer_size - pos as usize;
    }

    /* If there is available trace data now, it is unnecessary to wait. */
    if host_offset > pos {
        return (host_offset - pos) as usize;
    }

    0
}

unsafe fn sof_wait_dtrace_avail(sdev: *mut snd_sof_dev, pos: loff_t, buffer_size: usize) -> usize {
    let mut ret = sof_dtrace_avail(sdev, pos, buffer_size);
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;
    let mut wait = MaybeUninit::<wait_queue_entry_t>::uninit();

    /* data immediately available */
    if ret != 0 {
        return ret;
    }

    if (*priv_).dtrace_draining && !trace_pos_update_expected(priv_) {
        /*
         * tracing has ended and all traces have been
         * read by client, return EOF
         */
        (*priv_).dtrace_draining = false;
        return 0;
    }

    /* wait for available trace data from FW */
    init_waitqueue_entry(wait.as_mut_ptr(), current);
    set_current_state(TASK_INTERRUPTIBLE);
    add_wait_queue(&mut (*priv_).trace_sleep, wait.as_mut_ptr());

    if signal_pending(current) == 0 {
        /* set timeout to max value, no error code */
        schedule_timeout(MAX_SCHEDULE_TIMEOUT);
    }
    remove_wait_queue(&mut (*priv_).trace_sleep, wait.as_mut_ptr());

    ret = sof_dtrace_avail(sdev, pos, buffer_size);
    ret
}

unsafe extern "C" fn dfsentry_dtrace_read(
    file: *mut file,
    buffer: *mut c_char,
    mut count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let dfse = (*file).private_data as *mut snd_sof_dfsentry;
    let sdev = (*dfse).sdev;
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;
    let rem: c_ulong;
    let mut lpos: loff_t = *ppos;
    let avail: usize;
    let buffer_size: usize = (*dfse).size;
    let mut lpos_64: u64;

    /* make sure we know about any failures on the DSP side */
    (*priv_).dtrace_error = false;

    /* check pos and count */
    if lpos < 0 {
        return -EINVAL as ssize_t;
    }
    if count == 0 {
        return 0;
    }

    /* check for buffer wrap and count overflow */
    lpos_64 = lpos as u64;
    lpos = do_div(&mut lpos_64, buffer_size as u32) as loff_t;

    /* get available count based on current host offset */
    avail = sof_wait_dtrace_avail(sdev, lpos, buffer_size);
    if (*priv_).dtrace_error {
        dev_err((*sdev).dev, c_str!("trace IO error\n"));
        return -EIO as ssize_t;
    }

    /* no new trace data */
    if avail == 0 {
        return 0;
    }

    /* make sure count is <= avail */
    if count > avail {
        count = avail;
    }

    /*
     * make sure that all trace data is available for the CPU as the trace
     * data buffer might be allocated from non consistent memory.
     * Note: snd_dma_buffer_sync() is called for normal audio playback and
     *	 capture streams also.
     */
    snd_dma_buffer_sync(&mut (*priv_).dmatb, SNDRV_DMA_SYNC_CPU);
    /* copy available trace data to debugfs */
    rem = copy_to_user(buffer as *mut c_void, ((*dfse).buf as *mut u8).add(lpos as usize) as *const c_void, count);
    if rem != 0 {
        return -EFAULT as ssize_t;
    }

    *ppos += count as loff_t;

    /* move debugfs reading position */
    count as ssize_t
}

unsafe extern "C" fn dfsentry_dtrace_release(_inode: *mut inode, file: *mut file) -> c_int {
    let dfse = (*_inode).i_private as *mut snd_sof_dfsentry;
    let sdev = (*dfse).sdev;
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;

    /* avoid duplicate traces at next open */
    if (*priv_).dtrace_state != sof_dtrace_state::SOF_DTRACE_ENABLED {
        sof_dtrace_set_host_offset(priv_, 0);
    }

    0
}

static sof_dfs_dtrace_fops: file_operations = file_operations {
    open: Some(simple_open),
    read: Some(dfsentry_dtrace_read),
    llseek: Some(default_llseek),
    release: Some(dfsentry_dtrace_release),
    ..file_operations::zeroed()
};

unsafe fn debugfs_create_dtrace(sdev: *mut snd_sof_dev) -> c_int {
    let priv_: *mut sof_dtrace_priv;
    let dfse: *mut snd_sof_dfsentry;
    let ret: c_int;

    if sdev.is_null() {
        return -EINVAL;
    }

    priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;

    ret = debugfs_create_trace_filter(sdev);
    if ret < 0 {
        dev_warn((*sdev).dev, c_str!("failed to create filter debugfs file: %d"), ret);
    }

    dfse = devm_kzalloc((*sdev).dev, size_of::<snd_sof_dfsentry>(), GFP_KERNEL) as *mut snd_sof_dfsentry;
    if dfse.is_null() {
        return -ENOMEM;
    }

    (*dfse).type_ = SOF_DFSENTRY_TYPE_BUF;
    (*dfse).buf = (*priv_).dmatb.area;
    (*dfse).size = (*priv_).dmatb.bytes;
    (*dfse).sdev = sdev;

    debugfs_create_file(
        c_str!("trace"),
        0o444,
        (*sdev).debugfs_root,
        dfse as *mut c_void,
        &sof_dfs_dtrace_fops,
    );

    0
}

unsafe fn ipc3_dtrace_enable(sdev: *mut snd_sof_dev) -> c_int {
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;
    let ready = &mut (*sdev).fw_ready as *mut sof_ipc_fw_ready;
    let v = &mut (*ready).version as *mut sof_ipc_fw_version;
    let mut params = MaybeUninit::<sof_ipc_dma_trace_params_ext>::zeroed().assume_init();
    let mut ret: c_int;

    if !(*sdev).fw_trace_is_supported {
        return 0;
    }

    if (*priv_).dtrace_state == sof_dtrace_state::SOF_DTRACE_ENABLED || (*priv_).dma_trace_pages == 0 {
        return -EINVAL;
    }

    if (*priv_).dtrace_state != sof_dtrace_state::SOF_DTRACE_STOPPED {
        /* set IPC parameters */
        params.hdr.cmd = SOF_IPC_GLB_TRACE_MSG;
        /* PARAMS_EXT is only supported from ABI 3.7.0 onwards */
        if (*v).abi_version >= SOF_ABI_VER(3, 7, 0) {
            params.hdr.size = size_of::<sof_ipc_dma_trace_params_ext>() as u32;
            params.hdr.cmd |= SOF_IPC_TRACE_DMA_PARAMS_EXT;
            params.timestamp_ns = ktime_get(); /* in nanosecond */
        } else {
            params.hdr.size = size_of::<sof_ipc_dma_trace_params>() as u32;
            params.hdr.cmd |= SOF_IPC_TRACE_DMA_PARAMS;
        }
        params.buffer.phy_addr = (*priv_).dmatp.addr;
        params.buffer.size = (*priv_).dmatb.bytes;
        params.buffer.pages = (*priv_).dma_trace_pages as u32;
        params.stream_tag = 0;

        sof_dtrace_set_host_offset(priv_, 0);
        (*priv_).dtrace_draining = false;

        ret = sof_dtrace_host_init(sdev, &mut (*priv_).dmatb, &mut params);
        if ret < 0 {
            dev_err((*sdev).dev, c_str!("Host dtrace init failed: %d\n"), ret);
            return ret;
        }
        dev_dbg((*sdev).dev, c_str!("stream_tag: %d\n"), params.stream_tag);

        /* send IPC to the DSP */
        (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_INITIALIZING;
        ret = sof_ipc_tx_message_no_reply(
            (*sdev).ipc,
            &mut params as *mut _ as *mut c_void,
            size_of::<sof_ipc_dma_trace_params_ext>() as u32,
        );
        if ret < 0 {
            dev_err((*sdev).dev, c_str!("can't set params for DMA for trace %d\n"), ret);
            (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_DISABLED;
            sof_dtrace_host_release(sdev);
            return ret;
        }
    }

    (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_ENABLED;

    ret = sof_dtrace_host_trigger(sdev, SNDRV_PCM_TRIGGER_START);
    if ret < 0 {
        dev_err((*sdev).dev, c_str!("Host dtrace trigger start failed: %d\n"), ret);
        (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_DISABLED;
        sof_dtrace_host_release(sdev);
        return ret;
    }

    0
}

unsafe fn ipc3_dtrace_init(sdev: *mut snd_sof_dev) -> c_int {
    let priv_: *mut sof_dtrace_priv;
    let mut ret: c_int;

    /* dtrace is only supported with SOF_IPC */
    if (*(*sdev).pdata).ipc_type != SOF_IPC_TYPE_3 {
        return -EOPNOTSUPP;
    }

    if !(*sdev).fw_trace_data.is_null() {
        dev_err((*sdev).dev, c_str!("fw_trace_data has been already allocated\n"));
        return -EBUSY;
    }

    priv_ = devm_kzalloc((*sdev).dev, size_of::<sof_dtrace_priv>(), GFP_KERNEL) as *mut sof_dtrace_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    (*sdev).fw_trace_data = priv_ as *mut c_void;

    /* set false before start initialization */
    (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_DISABLED;

    /* allocate trace page table buffer */
    ret = snd_dma_alloc_pages(SNDRV_DMA_TYPE_DEV, (*sdev).dev, PAGE_SIZE, &mut (*priv_).dmatp);
    if ret < 0 {
        dev_err((*sdev).dev, c_str!("can't alloc page table for trace %d\n"), ret);
        return ret;
    }

    /* allocate trace data buffer */
    ret = snd_dma_alloc_dir_pages(
        SNDRV_DMA_TYPE_DEV_SG,
        (*sdev).dev,
        DMA_FROM_DEVICE,
        DMA_BUF_SIZE_FOR_TRACE,
        &mut (*priv_).dmatb,
    );
    if ret < 0 {
        dev_err((*sdev).dev, c_str!("can't alloc buffer for trace %d\n"), ret);
        snd_dma_free_pages(&mut (*priv_).dmatp);
        return ret;
    }

    /* create compressed page table for audio firmware */
    ret = snd_sof_create_page_table((*sdev).dev, &mut (*priv_).dmatb, (*priv_).dmatp.area, (*priv_).dmatb.bytes);
    if ret < 0 {
        (*priv_).dma_trace_pages = 0;
        snd_dma_free_pages(&mut (*priv_).dmatb);
        snd_dma_free_pages(&mut (*priv_).dmatp);
        return ret;
    }

    (*priv_).dma_trace_pages = ret;
    dev_dbg((*sdev).dev, c_str!("dma_trace_pages: %d\n"), (*priv_).dma_trace_pages);

    if (*sdev).first_boot {
        ret = debugfs_create_dtrace(sdev);
        if ret < 0 {
            (*priv_).dma_trace_pages = 0;
            snd_dma_free_pages(&mut (*priv_).dmatb);
            snd_dma_free_pages(&mut (*priv_).dmatp);
            return ret;
        }
    }

    init_waitqueue_head(&mut (*priv_).trace_sleep);

    ret = ipc3_dtrace_enable(sdev);
    if ret < 0 {
        (*priv_).dma_trace_pages = 0;
        snd_dma_free_pages(&mut (*priv_).dmatb);
        snd_dma_free_pages(&mut (*priv_).dmatp);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn ipc3_dtrace_posn_update(
    sdev: *mut snd_sof_dev,
    posn: *mut sof_ipc_dma_trace_posn,
) -> c_int {
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;

    if !(*sdev).fw_trace_is_supported {
        return 0;
    }

    if trace_pos_update_expected(priv_) && sof_dtrace_set_host_offset(priv_, (*posn).host_offset) {
        wake_up(&mut (*priv_).trace_sleep);
    }

    if (*posn).overflow != 0 {
        dev_err(
            (*sdev).dev,
            c_str!("DSP trace buffer overflow %u bytes. Total messages %d\n"),
            (*posn).overflow,
            (*posn).messages,
        );
    }

    0
}

/* an error has occurred within the DSP that prevents further trace */
unsafe extern "C" fn ipc3_dtrace_fw_crashed(sdev: *mut snd_sof_dev) {
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;

    if (*priv_).dtrace_state == sof_dtrace_state::SOF_DTRACE_ENABLED {
        (*priv_).dtrace_error = true;
        wake_up(&mut (*priv_).trace_sleep);
    }
}

unsafe extern "C" fn ipc3_dtrace_release(sdev: *mut snd_sof_dev, only_stop: bool) {
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;
    let ready = &mut (*sdev).fw_ready as *mut sof_ipc_fw_ready;
    let v = &mut (*ready).version as *mut sof_ipc_fw_version;
    let mut hdr = MaybeUninit::<sof_ipc_cmd_hdr>::zeroed().assume_init();
    let mut ret: c_int;

    if !(*sdev).fw_trace_is_supported || (*priv_).dtrace_state == sof_dtrace_state::SOF_DTRACE_DISABLED {
        return;
    }

    ret = sof_dtrace_host_trigger(sdev, SNDRV_PCM_TRIGGER_STOP);
    if ret < 0 {
        dev_err((*sdev).dev, c_str!("Host dtrace trigger stop failed: %d\n"), ret);
    }
    (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_STOPPED;

    /*
     * stop and free trace DMA in the DSP. TRACE_DMA_FREE is only supported from
     * ABI 3.20.0 onwards
     */
    if (*v).abi_version >= SOF_ABI_VER(3, 20, 0) {
        hdr.size = size_of::<sof_ipc_cmd_hdr>() as u32;
        hdr.cmd = SOF_IPC_GLB_TRACE_MSG | SOF_IPC_TRACE_DMA_FREE;

        ret = sof_ipc_tx_message_no_reply((*sdev).ipc, &mut hdr as *mut _ as *mut c_void, hdr.size);
        if ret < 0 {
            dev_err((*sdev).dev, c_str!("DMA_TRACE_FREE failed with error: %d\n"), ret);
        }
    }

    if !only_stop {
        ret = sof_dtrace_host_release(sdev);
        if ret < 0 {
            dev_err((*sdev).dev, c_str!("Host dtrace release failed %d\n"), ret);
        }

        (*priv_).dtrace_state = sof_dtrace_state::SOF_DTRACE_DISABLED;
    }

    (*priv_).dtrace_draining = true;
    wake_up(&mut (*priv_).trace_sleep);
}

unsafe extern "C" fn ipc3_dtrace_suspend(sdev: *mut snd_sof_dev, pm_state: pm_message_t) {
    ipc3_dtrace_release(sdev, pm_state.event == SOF_DSP_PM_D0);
}

unsafe extern "C" fn ipc3_dtrace_resume(sdev: *mut snd_sof_dev) -> c_int {
    ipc3_dtrace_enable(sdev)
}

unsafe extern "C" fn ipc3_dtrace_free(sdev: *mut snd_sof_dev) {
    let priv_ = (*sdev).fw_trace_data as *mut sof_dtrace_priv;

    /* release trace */
    ipc3_dtrace_release(sdev, false);

    if (*priv_).dma_trace_pages != 0 {
        snd_dma_free_pages(&mut (*priv_).dmatb);
        snd_dma_free_pages(&mut (*priv_).dmatp);
        (*priv_).dma_trace_pages = 0;
    }
}

#[no_mangle]
pub static ipc3_dtrace_ops: sof_ipc_fw_tracing_ops = sof_ipc_fw_tracing_ops {
    init: Some(ipc3_dtrace_init),
    free: Some(ipc3_dtrace_free),
    fw_crashed: Some(ipc3_dtrace_fw_crashed),
    suspend: Some(ipc3_dtrace_suspend),
    resume: Some(ipc3_dtrace_resume),
};


// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
