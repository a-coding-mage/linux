// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the surrounding Ceph and kernel bindings.

#[cfg(CONFIG_DEBUG_FS)]
static mut CEPH_DEBUGFS_DIR: *mut dentry = core::ptr::null_mut();

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn monmap_show(s: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 {
    let client = (*s).private as *mut ceph_client;
    mutex_lock(&mut (*client).monc.mutex);
    if (*client).monc.monmap.is_null() {
        mutex_unlock(&mut (*client).monc.mutex);
        return 0;
    }
    seq_printf(s, "epoch %d\n", (*(*client).monc.monmap).epoch);
    let mut i = 0;
    while i < (*(*client).monc.monmap).num_mon {
        let inst = &(*(*client).monc.monmap).mon_inst[i] as *const ceph_entity_inst;
        seq_printf(s, "\t%s%lld\t%s\n", ENTITY_NAME((*inst).name), ceph_pr_addr(&(*inst).addr));
        i += 1;
    }
    mutex_unlock(&mut (*client).monc.mutex);
    0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn osdmap_show(s: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 {
    let client = (*s).private as *mut ceph_client;
    let osdc = &mut (*client).osdc;
    down_read(&mut osdc.lock);
    let map = osdc.osdmap;
    if map.is_null() {
        up_read(&mut osdc.lock);
        return 0;
    }
    seq_printf(s, "epoch %u barrier %u flags 0x%x\n", (*map).epoch, osdc.epoch_barrier, (*map).flags);
    let mut n = rb_first(&mut (*map).pg_pools);
    while !n.is_null() {
        let pi = rb_entry(n, ceph_pg_pool_info, node);
        seq_printf(s, "pool %lld '%s' type %d size %d min_size %d pg_num %u pg_num_mask %d flags 0x%llx lfor %u read_tier %lld write_tier %lld\n", pi.id, pi.name, pi.type_, pi.size, pi.min_size, pi.pg_num, pi.pg_num_mask, pi.flags, pi.last_force_request_resend, pi.read_tier, pi.write_tier);
        n = rb_next(n);
    }
    let mut i = 0;
    while i < (*map).max_osd {
        let addr = &(*map).osd_addr[i];
        let state = (*map).osd_state[i];
        let mut sb = [0i8; 64];
        seq_printf(s, "osd%d\t%s\t%3d%%\t(%s)\t%3d%%\t%2d\n", i, ceph_pr_addr(addr), ((*map).osd_weight[i] * 100) >> 16, ceph_osdmap_state_str(sb.as_mut_ptr(), sb.len()), ((ceph_get_primary_affinity(map, i) * 100) >> 16), ceph_get_crush_locality(map, i, &(*client).options.crush_locs));
        i += 1;
    }
    n = rb_first(&mut (*map).pg_temp);
    while !n.is_null() {
        let pg = rb_entry(n, ceph_pg_mapping, node);
        seq_printf(s, "pg_temp %llu.%x [", pg.pgid.pool, pg.pgid.seed);
        i = 0; while i < pg.pg_temp.len { seq_printf(s, "%s%d", if i == 0 { "" } else { "," }, pg.pg_temp.osds[i]); i += 1; }
        seq_puts(s, "]\n"); n = rb_next(n);
    }
    n = rb_first(&mut (*map).primary_temp);
    while !n.is_null() { let pg = rb_entry(n, ceph_pg_mapping, node); seq_printf(s, "primary_temp %llu.%x %d\n", pg.pgid.pool, pg.pgid.seed, pg.primary_temp.osd); n = rb_next(n); }
    n = rb_first(&mut (*map).pg_upmap);
    while !n.is_null() { let pg = rb_entry(n, ceph_pg_mapping, node); seq_printf(s, "pg_upmap %llu.%x [", pg.pgid.pool, pg.pgid.seed); i = 0; while i < pg.pg_upmap.len { seq_printf(s, "%s%d", if i == 0 { "" } else { "," }, pg.pg_upmap.osds[i]); i += 1; } seq_puts(s, "]\n"); n = rb_next(n); }
    n = rb_first(&mut (*map).pg_upmap_items);
    while !n.is_null() { let pg = rb_entry(n, ceph_pg_mapping, node); seq_printf(s, "pg_upmap_items %llu.%x [", pg.pgid.pool, pg.pgid.seed); i = 0; while i < pg.pg_upmap_items.len { seq_printf(s, "%s%d->%d", if i == 0 { "" } else { "," }, pg.pg_upmap_items.from_to[i][0], pg.pg_upmap_items.from_to[i][1]); i += 1; } seq_puts(s, "]\n"); n = rb_next(n); }
    up_read(&mut osdc.lock); 0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn monc_show(s: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 {
    let client = (*s).private as *mut ceph_client; let monc = &mut (*client).monc;
    mutex_lock(&mut monc.mutex);
    let mut i = 0; while i < ARRAY_SIZE(monc.subs) { seq_printf(s, "have %s %u", ceph_sub_str[i], monc.subs[i].have); if monc.subs[i].want { seq_printf(s, " want %llu%s", le64_to_cpu(monc.subs[i].item.start), if monc.subs[i].item.flags & CEPH_SUBSCRIBE_ONETIME != 0 { "" } else { "+" }); } seq_putc(s, b'\n' as i32); i += 1; }
    seq_printf(s, "fs_cluster_id %d\n", monc.fs_cluster_id);
    let mut rp = rb_first(&mut monc.generic_request_tree); while !rp.is_null() { let req = rb_entry(rp, ceph_mon_generic_request, node); let op = le16_to_cpu((*req.request).hdr.type_); if op == CEPH_MSG_STATFS { seq_printf(s, "%llu statfs\n", req.tid); } else if op == CEPH_MSG_MON_GET_VERSION { seq_printf(s, "%llu mon_get_version", req.tid); } else { seq_printf(s, "%llu unknown\n", req.tid); } rp = rb_next(rp); }
    mutex_unlock(&mut monc.mutex); 0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_spgid(s: *mut seq_file, spgid: *const ceph_spg) { seq_printf(s, "%llu.%x", (*spgid).pgid.pool, (*spgid).pgid.seed); if (*spgid).shard != CEPH_SPG_NOSHARD { seq_printf(s, "s%d", (*spgid).shard); } }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_target(s: *mut seq_file, t: *mut ceph_osd_request_target) {
    seq_printf(s, "osd%d\t%llu.%x\t", (*t).osd, (*t).pgid.pool, (*t).pgid.seed); dump_spgid(s, &(*t).spgid); seq_puts(s, "\t["); let mut i = 0; while i < (*t).up.size { seq_printf(s, "%s%d", if i == 0 { "" } else { "," }, (*t).up.osds[i]); i += 1; } seq_printf(s, "]/%d\t[", (*t).up.primary); i = 0; while i < (*t).acting.size { seq_printf(s, "%s%d", if i == 0 { "" } else { "," }, (*t).acting.osds[i]); i += 1; } seq_printf(s, "]/%d\te%u\t", (*t).acting.primary, (*t).epoch); if !(*t).target_oloc.pool_ns.is_null() { seq_printf(s, "%*pE/%*pE\t0x%x", (*t).target_oloc.pool_ns.len, (*t).target_oloc.pool_ns.str, (*t).target_oid.name_len, (*t).target_oid.name, (*t).flags); } else { seq_printf(s, "%*pE\t0x%x", (*t).target_oid.name_len, (*t).target_oid.name, (*t).flags); } if (*t).paused { seq_puts(s, "\tP"); }
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_request(s: *mut seq_file, req: *mut ceph_osd_request) { seq_printf(s, "%llu\t", (*req).r_tid); dump_target(s, &mut (*req).r_t); seq_printf(s, "\t%d", (*req).r_attempts); let mut i = 0; while i < (*req).r_num_ops { let op = &(*req).r_ops[i]; seq_printf(s, "%s%s", if i == 0 { "\t" } else { "," }, ceph_osd_op_name(op.op)); if op.op == CEPH_OSD_OP_WATCH { seq_printf(s, "-%s", ceph_osd_watch_op_name(op.watch.op)); } else if op.op == CEPH_OSD_OP_CALL { seq_printf(s, "-%s/%s", op.cls.class_name, op.cls.method_name); } i += 1; } seq_putc(s, b'\n' as i32); }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_requests(s: *mut seq_file, osd: *mut ceph_osd) { mutex_lock(&mut (*osd).lock); let mut n = rb_first(&mut (*osd).o_requests); while !n.is_null() { dump_request(s, rb_entry(n, ceph_osd_request, r_node)); n = rb_next(n); } mutex_unlock(&mut (*osd).lock); }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_linger_request(s: *mut seq_file, lreq: *mut ceph_osd_linger_request) { seq_printf(s, "%llu\t", (*lreq).linger_id); dump_target(s, &mut (*lreq).t); seq_printf(s, "\t%u\t%s%s/%d\n", (*lreq).register_gen, if (*lreq).is_watch { "W" } else { "N" }, if (*lreq).committed { "C" } else { "" }, (*lreq).last_error); }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_linger_requests(s: *mut seq_file, osd: *mut ceph_osd) { mutex_lock(&mut (*osd).lock); let mut n = rb_first(&mut (*osd).o_linger_requests); while !n.is_null() { dump_linger_request(s, rb_entry(n, ceph_osd_linger_request, node)); n = rb_next(n); } mutex_unlock(&mut (*osd).lock); }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_snapid(s: *mut seq_file, snapid: u64) { if snapid == CEPH_NOSNAP { seq_puts(s, "head"); } else if snapid == CEPH_SNAPDIR { seq_puts(s, "snapdir"); } else { seq_printf(s, "%llx", snapid); } }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_name_escaped(s: *mut seq_file, name: *mut u8, len: usize) { let mut i = 0; while i < len { if *name.add(i) == b'%' || *name.add(i) == b':' || *name.add(i) == b'/' || *name.add(i) < 32 || *name.add(i) >= 127 { seq_printf(s, "%%%02x", *name.add(i)); } else { seq_putc(s, *name.add(i) as i32); } i += 1; } }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_hoid(s: *mut seq_file, hoid: *const ceph_hobject_id) { if (*hoid).snapid == 0 && (*hoid).hash == 0 && !(*hoid).is_max && (*hoid).pool == S64_MIN { seq_puts(s, "MIN"); return; } if (*hoid).is_max { seq_puts(s, "MAX"); return; } seq_printf(s, "%lld:%08x:", (*hoid).pool, (*hoid).hash_reverse_bits); dump_name_escaped(s, (*hoid).nspace, (*hoid).nspace_len); seq_putc(s, b':' as i32); dump_name_escaped(s, (*hoid).key, (*hoid).key_len); seq_putc(s, b':' as i32); dump_name_escaped(s, (*hoid).oid, (*hoid).oid_len); seq_putc(s, b':' as i32); dump_snapid(s, (*hoid).snapid); }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn dump_backoffs(s: *mut seq_file, osd: *mut ceph_osd) { mutex_lock(&mut (*osd).lock); let mut n = rb_first(&mut (*osd).o_backoffs_by_id); while !n.is_null() { let backoff = rb_entry(n, ceph_osd_backoff, id_node); seq_printf(s, "osd%d\t", (*osd).o_osd); dump_spgid(s, &backoff.spgid); seq_printf(s, "\t%llu\t", backoff.id); dump_hoid(s, backoff.begin); seq_putc(s, b'\t' as i32); dump_hoid(s, backoff.end); seq_putc(s, b'\n' as i32); n = rb_next(n); } mutex_unlock(&mut (*osd).lock); }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn osdc_show(s: *mut seq_file, _pp: *mut core::ffi::c_void) -> i32 { let client = (*s).private as *mut ceph_client; let osdc = &mut (*client).osdc; down_read(&mut osdc.lock); seq_printf(s, "REQUESTS %d homeless %d\n", atomic_read(&osdc.num_requests), atomic_read(&osdc.num_homeless)); let mut n = rb_first(&mut osdc.osds); while !n.is_null() { dump_requests(s, rb_entry(n, ceph_osd, o_node)); n = rb_next(n); } dump_requests(s, &mut osdc.homeless_osd); seq_puts(s, "LINGER REQUESTS\n"); n = rb_first(&mut osdc.osds); while !n.is_null() { dump_linger_requests(s, rb_entry(n, ceph_osd, o_node)); n = rb_next(n); } dump_linger_requests(s, &mut osdc.homeless_osd); seq_puts(s, "BACKOFFS\n"); n = rb_first(&mut osdc.osds); while !n.is_null() { dump_backoffs(s, rb_entry(n, ceph_osd, o_node)); n = rb_next(n); } up_read(&mut osdc.lock); 0 }

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn client_options_show(s: *mut seq_file, _p: *mut core::ffi::c_void) -> i32 { let client = (*s).private as *mut ceph_client; let ret = ceph_print_client_options(s, client, true); if ret != 0 { return ret; } seq_putc(s, b'\n' as i32); 0 }

// DEFINE_SHOW_ATTRIBUTE(monmap), osdmap, monc, osdc, and client_options.

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn ceph_debugfs_init() { CEPH_DEBUGFS_DIR = debugfs_create_dir("ceph", core::ptr::null_mut()); }
#[cfg(CONFIG_DEBUG_FS)]
unsafe fn ceph_debugfs_cleanup() { debugfs_remove(CEPH_DEBUGFS_DIR); }
#[cfg(CONFIG_DEBUG_FS)]
unsafe fn ceph_debugfs_client_init(client: *mut ceph_client) { let mut name = [0i8; 80]; snprintf(name.as_mut_ptr(), name.len(), "%pU.client%lld", &(*client).fsid, (*(*client).monc.auth).global_id); dout!("ceph_debugfs_client_init %p %s\n", client, name.as_ptr()); (*client).debugfs_dir = debugfs_create_dir(name.as_ptr(), CEPH_DEBUGFS_DIR); (*client).monc.debugfs_file = debugfs_create_file("monc", 0o400, (*client).debugfs_dir, client as *mut _, &monc_fops); (*client).osdc.debugfs_file = debugfs_create_file("osdc", 0o400, (*client).debugfs_dir, client as *mut _, &osdc_fops); (*client).debugfs_monmap = debugfs_create_file("monmap", 0o400, (*client).debugfs_dir, client as *mut _, &monmap_fops); (*client).debugfs_osdmap = debugfs_create_file("osdmap", 0o400, (*client).debugfs_dir, client as *mut _, &osdmap_fops); (*client).debugfs_options = debugfs_create_file("client_options", 0o400, (*client).debugfs_dir, client as *mut _, &client_options_fops); }
#[cfg(CONFIG_DEBUG_FS)]
unsafe fn ceph_debugfs_client_cleanup(client: *mut ceph_client) { dout!("ceph_debugfs_client_cleanup %p\n", client); debugfs_remove((*client).debugfs_options); debugfs_remove((*client).debugfs_osdmap); debugfs_remove((*client).debugfs_monmap); debugfs_remove((*client).osdc.debugfs_file); debugfs_remove((*client).monc.debugfs_file); debugfs_remove((*client).debugfs_dir); }

#[cfg(not(CONFIG_DEBUG_FS))]
unsafe fn ceph_debugfs_init() {}
#[cfg(not(CONFIG_DEBUG_FS))]
unsafe fn ceph_debugfs_cleanup() {}
#[cfg(not(CONFIG_DEBUG_FS))]
unsafe fn ceph_debugfs_client_init(_client: *mut ceph_client) {}
#[cfg(not(CONFIG_DEBUG_FS))]
unsafe fn ceph_debugfs_client_cleanup(_client: *mut ceph_client) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
