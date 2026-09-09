/* SPDX-License-Identifier: GPL-2.0 */

// Linux/Ceph dependencies are supplied by the surrounding translation unit.

static mut metrics_disable_warned: bool = false;

#[inline]
unsafe fn ceph_subvolume_entry_payload_len() -> u32 {
    core::mem::size_of::<ceph_subvolume_metric_entry_wire>() as u32
}

#[inline]
unsafe fn ceph_subvolume_entry_encoded_len() -> u32 {
    CEPH_ENCODING_START_BLK_LEN + ceph_subvolume_entry_payload_len()
}

#[inline]
unsafe fn ceph_subvolume_outer_payload_len(nr_subvols: u32) -> u32 {
    // count is encoded as le64 (size_t on wire) to match FUSE client
    core::mem::size_of::<__le64>() as u32 +
        nr_subvols.wrapping_mul(ceph_subvolume_entry_encoded_len())
}

#[inline]
unsafe fn ceph_subvolume_metric_data_len(nr_subvols: u32) -> u32 {
    CEPH_ENCODING_START_BLK_LEN + ceph_subvolume_outer_payload_len(nr_subvols)
}

#[inline]
unsafe fn ceph_subvolume_clamp_u32(val: u64) -> u32 {
    if val > U32_MAX { U32_MAX } else { val as u32 }
}

unsafe fn ceph_init_subvolume_wire_entry(
    dst: *mut ceph_subvolume_metric_entry_wire,
    src: *const ceph_subvol_metric_snapshot,
) {
    (*dst).subvolume_id = cpu_to_le64((*src).subvolume_id);
    (*dst).read_ops = cpu_to_le32(ceph_subvolume_clamp_u32((*src).read_ops));
    (*dst).write_ops = cpu_to_le32(ceph_subvolume_clamp_u32((*src).write_ops));
    (*dst).read_bytes = cpu_to_le64((*src).read_bytes);
    (*dst).write_bytes = cpu_to_le64((*src).write_bytes);
    (*dst).read_latency_us = cpu_to_le64((*src).read_latency_us);
    (*dst).write_latency_us = cpu_to_le64((*src).write_latency_us);
    (*dst).time_stamp = 0;
}

unsafe fn ceph_encode_subvolume_metrics(
    p: *mut *mut core::ffi::c_void,
    end: *mut core::ffi::c_void,
    subvols: *mut ceph_subvol_metric_snapshot,
    nr_subvols: u32,
) -> i32 {
    ceph_start_encoding(p, 1, 1, ceph_subvolume_outer_payload_len(nr_subvols));
    // count is encoded as le64 (size_t on wire) to match FUSE client
    ceph_encode_64_safe(p, end, nr_subvols as u64, enc_err);

    for i in 0..nr_subvols {
        let mut wire_entry: ceph_subvolume_metric_entry_wire = core::mem::zeroed();
        ceph_init_subvolume_wire_entry(&mut wire_entry, subvols.add(i as usize));
        ceph_start_encoding(p, 1, 1, ceph_subvolume_entry_payload_len());
        ceph_encode_copy_safe(p, end, &wire_entry, core::mem::size_of_val(&wire_entry), enc_err);
    }
    return 0;

    enc_err: {
        return -ERANGE;
    }
}

unsafe fn ktime_to_ceph_timespec(ts: *mut ceph_timespec, val: ktime_t) {
    let t: timespec64 = ktime_to_timespec64(val);
    ceph_encode_timespec64(ts, &t);
}

unsafe fn ceph_mdsc_send_metrics(mdsc: *mut ceph_mds_client, s: *mut ceph_mds_session) -> bool {
    let mut subvols: *mut ceph_subvol_metric_snapshot = core::ptr::null_mut();
    let m = &mut (*mdsc).metric;
    let mut nr_caps = atomic64_read(&m.total_caps);
    let header_len = core::mem::size_of::<ceph_metric_header>() as u32;
    let cl = (*(*mdsc).fsc).client;
    let mut nr_subvols: u32 = 0;
    let mut subvol_len: usize = 0;
    let mut items: i32 = 0;

    mutex_lock(&mut (*mdsc).mutex);
    if ceph_mdsmap_get_state((*mdsc).mdsmap, (*s).s_mds) != CEPH_MDS_STATE_ACTIVE {
        mutex_unlock(&mut (*mdsc).mutex);
        return false;
    }
    mutex_unlock(&mut (*mdsc).mutex);

    if ceph_subvolume_metrics_enabled(&mut (*mdsc).subvol_metrics) &&
        test_bit(CEPHFS_FEATURE_SUBVOLUME_METRICS, &(*s).s_features) {
        let ret = ceph_subvolume_metrics_snapshot(&mut (*mdsc).subvol_metrics,
            &mut subvols, &mut nr_subvols, true);
        if ret != 0 {
            pr_warn_client(cl, "failed to snapshot subvolume metrics: %d\n", ret);
            nr_subvols = 0;
            subvols = core::ptr::null_mut();
        }
    }

    if nr_subvols != 0 {
        subvol_len = core::mem::size_of::<__le32>() + ceph_subvolume_metric_data_len(nr_subvols) as usize;
    }

    let len = core::mem::size_of::<ceph_metric_head>() + core::mem::size_of::<ceph_metric_cap>() +
        core::mem::size_of::<ceph_metric_read_latency>() + core::mem::size_of::<ceph_metric_write_latency>() +
        core::mem::size_of::<ceph_metric_metadata_latency>() + core::mem::size_of::<ceph_metric_dlease>() +
        core::mem::size_of::<ceph_opened_files>() + core::mem::size_of::<ceph_pinned_icaps>() +
        core::mem::size_of::<ceph_opened_inodes>() + core::mem::size_of::<ceph_read_io_size>() +
        core::mem::size_of::<ceph_write_io_size>() + subvol_len;
    let msg = ceph_msg_new(CEPH_MSG_CLIENT_METRICS, len as u32, GFP_NOFS, true);
    if msg.is_null() {
        pr_err_client(cl, "to mds%d, failed to allocate message\n", (*s).s_mds);
        kfree(subvols);
        return false;
    }

    let head = (*msg).front.iov_base as *mut ceph_metric_head;
    let cap = head.add(1) as *mut ceph_metric_cap;
    (*cap).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_CAP_INFO); (*cap).header.ver = 1; (*cap).header.compat = 1;
    (*cap).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_metric_cap>() as u32 - header_len);
    (*cap).hit = cpu_to_le64(percpu_counter_sum(&m.i_caps_hit)); (*cap).mis = cpu_to_le64(percpu_counter_sum(&m.i_caps_mis));
    (*cap).total = cpu_to_le64(nr_caps); items += 1;

    let read = cap.add(1) as *mut ceph_metric_read_latency;
    (*read).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_READ_LATENCY); (*read).header.ver = 2; (*read).header.compat = 1;
    (*read).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_metric_read_latency>() as u32 - header_len);
    ktime_to_ceph_timespec(&mut (*read).lat, m.metric[METRIC_READ].latency_sum);
    ktime_to_ceph_timespec(&mut (*read).avg, m.metric[METRIC_READ].latency_avg);
    (*read).sq_sum = cpu_to_le64(m.metric[METRIC_READ].latency_sq_sum); (*read).count = cpu_to_le64(m.metric[METRIC_READ].total); items += 1;

    let write = read.add(1) as *mut ceph_metric_write_latency;
    (*write).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_WRITE_LATENCY); (*write).header.ver = 2; (*write).header.compat = 1;
    (*write).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_metric_write_latency>() as u32 - header_len);
    ktime_to_ceph_timespec(&mut (*write).lat, m.metric[METRIC_WRITE].latency_sum);
    ktime_to_ceph_timespec(&mut (*write).avg, m.metric[METRIC_WRITE].latency_avg);
    (*write).sq_sum = cpu_to_le64(m.metric[METRIC_WRITE].latency_sq_sum); (*write).count = cpu_to_le64(m.metric[METRIC_WRITE].total); items += 1;

    let meta = write.add(1) as *mut ceph_metric_metadata_latency;
    (*meta).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_METADATA_LATENCY); (*meta).header.ver = 2; (*meta).header.compat = 1;
    (*meta).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_metric_metadata_latency>() as u32 - header_len);
    ktime_to_ceph_timespec(&mut (*meta).lat, m.metric[METRIC_METADATA].latency_sum);
    ktime_to_ceph_timespec(&mut (*meta).avg, m.metric[METRIC_METADATA].latency_avg);
    (*meta).sq_sum = cpu_to_le64(m.metric[METRIC_METADATA].latency_sq_sum); (*meta).count = cpu_to_le64(m.metric[METRIC_METADATA].total); items += 1;

    let dlease = meta.add(1) as *mut ceph_metric_dlease;
    (*dlease).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_DENTRY_LEASE); (*dlease).header.ver = 1; (*dlease).header.compat = 1;
    (*dlease).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_metric_dlease>() as u32 - header_len);
    (*dlease).hit = cpu_to_le64(percpu_counter_sum(&m.d_lease_hit)); (*dlease).mis = cpu_to_le64(percpu_counter_sum(&m.d_lease_mis));
    (*dlease).total = cpu_to_le64(atomic64_read(&m.total_dentries)); items += 1;
    let sum = percpu_counter_sum(&m.total_inodes);

    let files = dlease.add(1) as *mut ceph_opened_files;
    (*files).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_OPENED_FILES); (*files).header.ver = 1; (*files).header.compat = 1;
    (*files).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_opened_files>() as u32 - header_len);
    (*files).opened_files = cpu_to_le64(atomic64_read(&m.opened_files)); (*files).total = cpu_to_le64(sum); items += 1;
    let icaps = files.add(1) as *mut ceph_pinned_icaps;
    (*icaps).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_PINNED_ICAPS); (*icaps).header.ver = 1; (*icaps).header.compat = 1;
    (*icaps).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_pinned_icaps>() as u32 - header_len);
    (*icaps).pinned_icaps = cpu_to_le64(nr_caps); (*icaps).total = cpu_to_le64(sum); items += 1;
    let inodes = icaps.add(1) as *mut ceph_opened_inodes;
    (*inodes).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_OPENED_INODES); (*inodes).header.ver = 1; (*inodes).header.compat = 1;
    (*inodes).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_opened_inodes>() as u32 - header_len);
    (*inodes).opened_inodes = cpu_to_le64(percpu_counter_sum(&m.opened_inodes)); (*inodes).total = cpu_to_le64(sum); items += 1;
    let rsize = inodes.add(1) as *mut ceph_read_io_size;
    (*rsize).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_READ_IO_SIZES); (*rsize).header.ver = 1; (*rsize).header.compat = 1;
    (*rsize).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_read_io_size>() as u32 - header_len);
    (*rsize).total_ops = cpu_to_le64(m.metric[METRIC_READ].total); (*rsize).total_size = cpu_to_le64(m.metric[METRIC_READ].size_sum); items += 1;
    let wsize = rsize.add(1) as *mut ceph_write_io_size;
    (*wsize).header.type_ = cpu_to_le32(CLIENT_METRIC_TYPE_WRITE_IO_SIZES); (*wsize).header.ver = 1; (*wsize).header.compat = 1;
    (*wsize).header.data_len = cpu_to_le32(core::mem::size_of::<ceph_write_io_size>() as u32 - header_len);
    (*wsize).total_ops = cpu_to_le64(m.metric[METRIC_WRITE].total); (*wsize).total_size = cpu_to_le64(m.metric[METRIC_WRITE].size_sum); items += 1;

    let mut cursor = wsize.add(1) as *mut u8;
    if nr_subvols != 0 {
        ceph_encode_32(&mut (cursor as *mut core::ffi::c_void), CLIENT_METRIC_TYPE_SUBVOLUME_METRICS);
        items += 1;
        let mut payload = cursor.add(4) as *mut core::ffi::c_void;
        let payload_end = (payload as *mut u8).add(ceph_subvolume_metric_data_len(nr_subvols) as usize) as *mut core::ffi::c_void;
        if ceph_encode_subvolume_metrics(&mut payload, payload_end, subvols, nr_subvols) != 0 {
            pr_warn_client(cl, "failed to encode subvolume metrics\n"); kfree(subvols); ceph_msg_put(msg); return false;
        }
        cursor = payload as *mut u8;
    }
    put_unaligned_le32(items as u32, &mut (*head).num);
    (*msg).front.iov_len = cursor.offset_from(head as *mut u8) as usize;
    (*msg).hdr.version = cpu_to_le16(1); (*msg).hdr.compat_version = cpu_to_le16(1);
    (*msg).hdr.front_len = cpu_to_le32((*msg).front.iov_len as u32);
    ceph_con_send(&mut (*s).s_con, msg);
    if nr_subvols != 0 {
        mutex_lock(&mut (*mdsc).subvol_metrics_last_mutex); kfree((*mdsc).subvol_metrics_last);
        (*mdsc).subvol_metrics_last = subvols; (*mdsc).subvol_metrics_last_nr = nr_subvols;
        (*mdsc).subvol_metrics_sent += nr_subvols; (*mdsc).subvol_metrics_nonzero_sends += 1;
        mutex_unlock(&mut (*mdsc).subvol_metrics_last_mutex); subvols = core::ptr::null_mut();
    }
    kfree(subvols); true
}

unsafe fn metric_get_session(mdsc: *mut ceph_mds_client) {
    mutex_lock(&mut (*mdsc).mutex);
    for i in 0..(*mdsc).max_sessions {
        let s = __ceph_lookup_mds_session(mdsc, i);
        if s.is_null() { continue; }
        if check_session_state(s) && test_bit(CEPHFS_FEATURE_METRIC_COLLECT, &(*s).s_features) {
            if ceph_subvolume_metrics_enabled(&mut (*mdsc).subvol_metrics) && !test_bit(CEPHFS_FEATURE_SUBVOLUME_METRICS, &(*s).s_features) { ceph_put_mds_session(s); continue; }
            (*mdsc).metric.session = s; break;
        }
        ceph_put_mds_session(s);
    }
    mutex_unlock(&mut (*mdsc).mutex);
}

unsafe fn metric_delayed_work(work: *mut work_struct) {
    let m = container_of!(work, ceph_client_metric, delayed_work.work);
    let mdsc = container_of!(m, ceph_mds_client, metric);
    if (*mdsc).stopping { return; }
    if disable_send_metrics {
        if !metrics_disable_warned { pr_info!("ceph: metrics sending disabled via module parameter\n"); metrics_disable_warned = true; }
        return;
    }
    metrics_disable_warned = false;
    if (*m).session.is_null() || !check_session_state((*m).session) {
        if !(*m).session.is_null() { ceph_put_mds_session((*m).session); (*m).session = core::ptr::null_mut(); }
        metric_get_session(mdsc);
    }
    if !(*m).session.is_null() { ceph_mdsc_send_metrics(mdsc, (*m).session); } else { pr_warn_ratelimited!("ceph: metrics worker has no MDS session\n"); }
    metric_schedule_delayed(m);
}

pub unsafe fn ceph_metric_init(m: *mut ceph_client_metric) -> i32 {
    if m.is_null() { return -EINVAL; }
    atomic64_set(&mut (*m).total_dentries, 0);
    let mut ret = percpu_counter_init(&mut (*m).d_lease_hit, 0, GFP_KERNEL); if ret != 0 { return ret; }
    ret = percpu_counter_init(&mut (*m).d_lease_mis, 0, GFP_KERNEL); if ret != 0 { percpu_counter_destroy(&mut (*m).d_lease_hit); return ret; }
    atomic64_set(&mut (*m).total_caps, 0);
    ret = percpu_counter_init(&mut (*m).i_caps_hit, 0, GFP_KERNEL); if ret != 0 { percpu_counter_destroy(&mut (*m).d_lease_mis); percpu_counter_destroy(&mut (*m).d_lease_hit); return ret; }
    ret = percpu_counter_init(&mut (*m).i_caps_mis, 0, GFP_KERNEL); if ret != 0 { percpu_counter_destroy(&mut (*m).i_caps_hit); percpu_counter_destroy(&mut (*m).d_lease_mis); percpu_counter_destroy(&mut (*m).d_lease_hit); return ret; }
    for i in 0..METRIC_MAX { let metric = &mut (*m).metric[i]; spin_lock_init(&mut metric.lock); metric.size_sum = 0; metric.size_min = U64_MAX; metric.size_max = 0; metric.total = 0; metric.latency_sum = 0; metric.latency_avg = 0; metric.latency_sq_sum = 0; metric.latency_min = KTIME_MAX; metric.latency_max = 0; }
    atomic64_set(&mut (*m).opened_files, 0);
    ret = percpu_counter_init(&mut (*m).opened_inodes, 0, GFP_KERNEL); if ret != 0 { percpu_counter_destroy(&mut (*m).i_caps_mis); percpu_counter_destroy(&mut (*m).i_caps_hit); percpu_counter_destroy(&mut (*m).d_lease_mis); percpu_counter_destroy(&mut (*m).d_lease_hit); return ret; }
    ret = percpu_counter_init(&mut (*m).total_inodes, 0, GFP_KERNEL); if ret != 0 { percpu_counter_destroy(&mut (*m).opened_inodes); percpu_counter_destroy(&mut (*m).i_caps_mis); percpu_counter_destroy(&mut (*m).i_caps_hit); percpu_counter_destroy(&mut (*m).d_lease_mis); percpu_counter_destroy(&mut (*m).d_lease_hit); return ret; }
    (*m).session = core::ptr::null_mut(); INIT_DELAYED_WORK(&mut (*m).delayed_work, metric_delayed_work); 0
}

pub unsafe fn ceph_metric_destroy(m: *mut ceph_client_metric) {
    if m.is_null() { return; }
    cancel_delayed_work_sync(&mut (*m).delayed_work); percpu_counter_destroy(&mut (*m).total_inodes); percpu_counter_destroy(&mut (*m).opened_inodes); percpu_counter_destroy(&mut (*m).i_caps_mis); percpu_counter_destroy(&mut (*m).i_caps_hit); percpu_counter_destroy(&mut (*m).d_lease_mis); percpu_counter_destroy(&mut (*m).d_lease_hit); ceph_put_mds_session((*m).session);
}

#[inline]
unsafe fn metric_update_min_max(min: &mut u64, max: &mut u64, new: u64) { if new < *min { *min = new; } if new > *max { *max = new; } }

#[inline]
unsafe fn __update_mean_and_stdev(total: ktime_t, lavg: *mut ktime_t, sq_sump: *mut ktime_t, lat: ktime_t) {
    if total == 1 { *lavg = lat; } else { let avg = (*lavg).wrapping_add(div64_s64(lat.wrapping_sub(*lavg), total)); *sq_sump = (*sq_sump).wrapping_add(lat.wrapping_sub(*lavg).wrapping_mul(lat.wrapping_sub(avg))); *lavg = avg; }
}

pub unsafe fn ceph_update_metrics(m: *mut ceph_metric, r_start: ktime_t, r_end: ktime_t, size: u32, rc: i32) {
    let lat = ktime_sub(r_end, r_start); if rc < 0 && rc != -ENOENT && rc != -ETIMEDOUT { return; }
    spin_lock(&mut (*m).lock); (*m).total = (*m).total.wrapping_add(1); (*m).size_sum = (*m).size_sum.wrapping_add(size as u64); metric_update_min_max(&mut (*m).size_min, &mut (*m).size_max, size as u64); (*m).latency_sum = (*m).latency_sum.wrapping_add(lat); metric_update_min_max(&mut (*m).latency_min, &mut (*m).latency_max, lat as u64); __update_mean_and_stdev((*m).total, &mut (*m).latency_avg, &mut (*m).latency_sq_sum, lat); spin_unlock(&mut (*m).lock);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
