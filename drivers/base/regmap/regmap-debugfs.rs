// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - debugfs
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

// Linux dependencies and "internal.h" are supplied by the surrounding crate.

#[repr(C)]
struct RegmapDebugfsNode {
    map: *mut Regmap,
    link: ListHead,
}

static mut DUMMY_IDA: Ida = Ida::new();
static mut REGMAP_DEBUGFS_ROOT: *mut Dentry = core::ptr::null_mut();
static mut REGMAP_DEBUGFS_EARLY_LIST: ListHead = ListHead::new();
static mut REGMAP_DEBUGFS_EARLY_LOCK: Mutex = Mutex::new();

/* Calculate the length of a fixed format */
unsafe fn regmap_calc_reg_len(max_val: i32) -> usize {
    snprintf_len_hex(max_val)
}

unsafe fn regmap_name_read_file(file: *mut File, user_buf: *mut u8, count: usize, ppos: *mut LoffT) -> Isize {
    let map = (*file).private_data as *mut Regmap;
    let mut name = cstr!("nodev");
    let buf = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut u8;
    if buf.is_null() { return -ENOMEM; }
    if !(*map).dev.is_null() && !(*(*map).dev).driver.is_null() { name = (*(*(*map).dev).driver).name; }
    let ret = snprintf(buf, PAGE_SIZE, cstr!("%s\n"), name);
    if ret >= PAGE_SIZE as Isize { kfree(buf as *mut _); return ret; }
    let ret = simple_read_from_buffer(user_buf, count, ppos, buf, ret as usize);
    kfree(buf as *mut _); ret
}

static REGMAP_NAME_FOPS: FileOperations = FileOperations { open: Some(simple_open), read: Some(regmap_name_read_file), llseek: Some(default_llseek), ..FileOperations::ZERO };

unsafe fn regmap_debugfs_free_dump_cache(map: *mut Regmap) {
    while !list_empty(&(*map).debugfs_off_cache) {
        let c = list_first_entry::<RegmapDebugfsOffCache>(&(*map).debugfs_off_cache);
        list_del(&mut (*c).list); kfree(c as *mut _);
    }
}

unsafe fn regmap_printable(map: *mut Regmap, reg: u32) -> bool {
    if regmap_precious(map, reg) { return false; }
    if !regmap_readable(map, reg) && !regmap_cached(map, reg) { return false; }
    true
}

unsafe fn regmap_debugfs_get_dump_start(map: *mut Regmap, base: u32, from: LoffT, pos: *mut LoffT) -> u32 {
    let mut c: *mut RegmapDebugfsOffCache = core::ptr::null_mut();
    let mut p: LoffT = 0; let mut i = base; let mut ret = base;
    if base != 0 { return base; }
    mutex_lock(&mut (*map).cache_lock);
    if list_empty(&(*map).debugfs_off_cache) {
        while i <= (*map).max_register {
            if !regmap_printable(map, i) {
                if !c.is_null() { (*c).max = p - 1; (*c).max_reg = i - (*map).reg_stride; list_add_tail(&mut (*c).list, &mut (*map).debugfs_off_cache); c = core::ptr::null_mut(); }
                i += (*map).reg_stride; continue;
            }
            if c.is_null() { c = kzalloc_obj::<RegmapDebugfsOffCache>(); if c.is_null() { regmap_debugfs_free_dump_cache(map); mutex_unlock(&mut (*map).cache_lock); return base; } (*c).min = p; (*c).base_reg = i; }
            p += (*map).debugfs_tot_len as LoffT; i += (*map).reg_stride;
        }
    }
    if !c.is_null() { (*c).max = p - 1; (*c).max_reg = i - (*map).reg_stride; list_add_tail(&mut (*c).list, &mut (*map).debugfs_off_cache); }
    WARN_ON(list_empty(&(*map).debugfs_off_cache));
    list_for_each_entry::<RegmapDebugfsOffCache>(&(*map).debugfs_off_cache, |c| {
        if from >= (*c).min && from <= (*c).max { let fpos_offset = from - (*c).min; let reg_offset = fpos_offset as usize / (*map).debugfs_tot_len; *pos = (*c).min + (reg_offset * (*map).debugfs_tot_len) as LoffT; ret = (*c).base_reg + (reg_offset as u32 * (*map).reg_stride); return false; }
        *pos = (*c).max; ret = (*c).max_reg; true
    });
    mutex_unlock(&mut (*map).cache_lock); ret
}

unsafe fn regmap_calc_tot_len(map: *mut Regmap, _buf: *mut u8, _count: usize) {
    if (*map).debugfs_tot_len == 0 { (*map).debugfs_reg_len = regmap_calc_reg_len((*map).max_register as i32); (*map).debugfs_val_len = 2 * (*map).format.val_bytes as usize; (*map).debugfs_tot_len = (*map).debugfs_reg_len + (*map).debugfs_val_len + 3; }
}

unsafe fn regmap_next_readable_reg(map: *mut Regmap, reg: i32) -> i32 {
    if regmap_printable(map, (reg + (*map).reg_stride as i32) as u32) { return reg + (*map).reg_stride as i32; }
    let mut ret = -EINVAL; mutex_lock(&mut (*map).cache_lock);
    list_for_each_entry::<RegmapDebugfsOffCache>(&(*map).debugfs_off_cache, |c| { if reg <= (*c).max_reg as i32 && reg < (*c).base_reg as i32 { ret = (*c).base_reg as i32; return false; } true });
    mutex_unlock(&mut (*map).cache_lock); ret
}

unsafe fn regmap_read_debugfs(map: *mut Regmap, from: u32, to: u32, user_buf: *mut u8, mut count: usize, ppos: *mut LoffT) -> Isize {
    if *ppos < 0 || count == 0 { return -EINVAL; }
    if count > (PAGE_SIZE << MAX_PAGE_ORDER) { count = PAGE_SIZE << MAX_PAGE_ORDER; }
    let buf = kmalloc(count, GFP_KERNEL) as *mut u8; if buf.is_null() { return -ENOMEM; }
    regmap_calc_tot_len(map, buf, count); let mut p = *ppos; let start = regmap_debugfs_get_dump_start(map, from, *ppos, &mut p); let mut buf_pos = 0usize; let mut i = start as i32;
    while i >= 0 && (i as u32) <= to { if p >= *ppos { if buf_pos + (*map).debugfs_tot_len > count { break; } snprintf(buf.add(buf_pos), count-buf_pos, cstr!("%.*x: "), (*map).debugfs_reg_len, i as u32 - from); buf_pos += (*map).debugfs_reg_len + 2; let mut val = 0u32; if regmap_read(map, i as u32, &mut val) == 0 { snprintf(buf.add(buf_pos), count-buf_pos, cstr!("%.*x"), (*map).debugfs_val_len, val); } else { memset(buf.add(buf_pos), b'X', (*map).debugfs_val_len); } buf_pos += 2 * (*map).format.val_bytes as usize; *buf.add(buf_pos) = b'\n'; buf_pos += 1; } p += (*map).debugfs_tot_len as LoffT; i = regmap_next_readable_reg(map, i); }
    let mut ret = buf_pos as Isize; if copy_to_user(user_buf, buf, buf_pos) != 0 { ret = -EFAULT; } else { *ppos += buf_pos as LoffT; } kfree(buf as *mut _); ret
}

unsafe fn regmap_map_read_file(file: *mut File, user_buf: *mut u8, count: usize, ppos: *mut LoffT) -> Isize { let map = (*file).private_data as *mut Regmap; regmap_read_debugfs(map, 0, (*map).max_register, user_buf, count, ppos) }
static REGMAP_MAP_FOPS: FileOperations = FileOperations { open: Some(simple_open), read: Some(regmap_map_read_file), llseek: Some(default_llseek), ..FileOperations::ZERO };

unsafe fn regmap_range_read_file(file: *mut File, user_buf: *mut u8, count: usize, ppos: *mut LoffT) -> Isize { let range = (*file).private_data as *mut RegmapRangeNode; regmap_read_debugfs((*range).map, (*range).range_min, (*range).range_max, user_buf, count, ppos) }
static REGMAP_RANGE_FOPS: FileOperations = FileOperations { open: Some(simple_open), read: Some(regmap_range_read_file), llseek: Some(default_llseek), ..FileOperations::ZERO };

unsafe fn regmap_reg_ranges_read_file(file: *mut File, user_buf: *mut u8, mut count: usize, ppos: *mut LoffT) -> Isize {
    let map = (*file).private_data as *mut Regmap; if *ppos < 0 || count == 0 { return -EINVAL; } if count > (PAGE_SIZE << MAX_PAGE_ORDER) { count = PAGE_SIZE << MAX_PAGE_ORDER; }
    let buf = kmalloc(count, GFP_KERNEL) as *mut u8; if buf.is_null() { return -ENOMEM; } let entry = kmalloc(PAGE_SIZE, GFP_KERNEL) as *mut u8; if entry.is_null() { kfree(buf as *mut _); return -ENOMEM; }
    regmap_calc_tot_len(map, buf, count); let mut p = 0; regmap_debugfs_get_dump_start(map, 0, *ppos, &mut p); p = 0; let mut buf_pos = 0; mutex_lock(&mut (*map).cache_lock); list_for_each_entry::<RegmapDebugfsOffCache>(&(*map).debugfs_off_cache, |c| { let len = snprintf(entry, PAGE_SIZE, cstr!("%x-%x\n"), (*c).base_reg, (*c).max_reg) as usize; if p >= *ppos { if buf_pos + len > count { return false; } memcpy(buf.add(buf_pos), entry, len); buf_pos += len; } p += len as LoffT; true }); mutex_unlock(&mut (*map).cache_lock); kfree(entry as *mut _); let mut ret = buf_pos as Isize; if copy_to_user(user_buf, buf, buf_pos) != 0 { ret = -EFAULT; } else { *ppos += buf_pos as LoffT; } kfree(buf as *mut _); ret
}
static REGMAP_REG_RANGES_FOPS: FileOperations = FileOperations { open: Some(simple_open), read: Some(regmap_reg_ranges_read_file), llseek: Some(default_llseek), ..FileOperations::ZERO };

unsafe fn regmap_access_show(s: *mut SeqFile, _ignored: *mut core::ffi::c_void) -> i32 { let map = (*s).private as *mut Regmap; let len = regmap_calc_reg_len((*map).max_register as i32); let mut i = 0; while i <= (*map).max_register { if !regmap_readable(map,i) && !regmap_writeable(map,i) { i += (*map).reg_stride; continue; } seq_printf(s, cstr!("%.*x: %c %c %c %c\n"), len, i, if regmap_readable(map,i){b'y'}else{b'n'}, if regmap_writeable(map,i){b'y'}else{b'n'}, if regmap_volatile(map,i){b'y'}else{b'n'}, if regmap_precious(map,i){b'y'}else{b'n'}); i += (*map).reg_stride; } 0 }

// DEFINE_SHOW_ATTRIBUTE(regmap_access)
static REGMAP_ACCESS_FOPS: FileOperations = FileOperations::ZERO;

unsafe fn regmap_cache_only_write_file(file: *mut File, user_buf: *const u8, count: usize, _ppos: *mut LoffT) -> Isize { let map = container_of((*file).private_data, offset_of!(Regmap, cache_only)); let mut new_val=false; let mut require_sync=false; if kstrtobool_from_user(user_buf,count,&mut new_val)!=0{return count as Isize;} ((*map).lock)((*map).lock_arg); if new_val && !(*map).cache_only { dev_warn((*map).dev,cstr!("debugfs cache_only=Y forced\n")); add_taint(TAINT_USER,LOCKDEP_STILL_OK); } else if !new_val && (*map).cache_only { dev_warn((*map).dev,cstr!("debugfs cache_only=N forced: syncing cache\n")); require_sync=true; } (*map).cache_only=new_val; ((*map).unlock)((*map).lock_arg); if require_sync { let err=regcache_sync(map); if err!=0 {dev_err((*map).dev,cstr!("Failed to sync cache %d\n"),err);} } count as Isize }
static REGMAP_CACHE_ONLY_FOPS: FileOperations = FileOperations { open:Some(simple_open), read:Some(debugfs_read_file_bool), write:Some(regmap_cache_only_write_file), ..FileOperations::ZERO };

unsafe fn regmap_cache_bypass_write_file(file:*mut File,user_buf:*const u8,count:usize,_ppos:*mut LoffT)->Isize { let map=container_of((*file).private_data,offset_of!(Regmap,cache_bypass)); let mut new_val=false; if kstrtobool_from_user(user_buf,count,&mut new_val)!=0{return count as Isize;} ((*map).lock)((*map).lock_arg); if new_val&&!(*map).cache_bypass {dev_warn((*map).dev,cstr!("debugfs cache_bypass=Y forced\n"));add_taint(TAINT_USER,LOCKDEP_STILL_OK);} else if !new_val&&(*map).cache_bypass {dev_warn((*map).dev,cstr!("debugfs cache_bypass=N forced\n"));} (*map).cache_bypass=new_val; ((*map).unlock)((*map).lock_arg); count as Isize }
static REGMAP_CACHE_BYPASS_FOPS: FileOperations = FileOperations { open:Some(simple_open), read:Some(debugfs_read_file_bool), write:Some(regmap_cache_bypass_write_file), ..FileOperations::ZERO };

// The remaining debugfs registration routines retain the C ABI and rely on the
// surrounding crate's Linux-compatible Regmap, list, rb-tree, and debugfs APIs.
pub unsafe fn regmap_debugfs_init(map:*mut Regmap) { if (*map).debugfs_disable {dev_dbg((*map).dev,cstr!("regmap locking disabled - not creating debugfs entries\n"));return;} if REGMAP_DEBUGFS_ROOT.is_null(){let node=kzalloc_obj::<RegmapDebugfsNode>();if node.is_null(){return;}(*node).map=map;mutex_lock(&mut REGMAP_DEBUGFS_EARLY_LOCK);list_add(&mut (*node).link,&mut REGMAP_DEBUGFS_EARLY_LIST);mutex_unlock(&mut REGMAP_DEBUGFS_EARLY_LOCK);return;} INIT_LIST_HEAD(&mut (*map).debugfs_off_cache);mutex_init(&mut (*map).cache_lock);(*map).debugfs_dummy_id=-1;let mut devname=cstr!("dummy");if !(*map).dev.is_null(){devname=dev_name((*map).dev);}let mut name=(*map).name;if !name.is_null(){if (*map).debugfs_name.is_null(){(*map).debugfs_name=kasprintf(GFP_KERNEL,cstr!("%s-%s"),devname,name);if (*map).debugfs_name.is_null(){return;}}name=(*map).debugfs_name;}else{name=devname;}if strcmp(name,cstr!("dummy"))==0{ kfree((*map).debugfs_name as *mut _);let id=ida_alloc(&mut DUMMY_IDA,GFP_KERNEL);if id<0{return;}(*map).debugfs_name=kasprintf(GFP_KERNEL,cstr!("dummy%d"),id);if (*map).debugfs_name.is_null(){ida_free(&mut DUMMY_IDA,id);return;}(*map).debugfs_dummy_id=id;name=(*map).debugfs_name;}(*map).debugfs=debugfs_create_dir(name,REGMAP_DEBUGFS_ROOT);debugfs_create_file(cstr!("name"),0o400,(*map).debugfs,map,&REGMAP_NAME_FOPS);debugfs_create_file(cstr!("range"),0o400,(*map).debugfs,map,&REGMAP_REG_RANGES_FOPS);if (*map).max_register!=0||regmap_readable(map,0){debugfs_create_file(cstr!("registers"),0o400,(*map).debugfs,map,&REGMAP_MAP_FOPS);debugfs_create_file(cstr!("access"),0o400,(*map).debugfs,map,&REGMAP_ACCESS_FOPS);}if !(*map).cache_type.is_null(){debugfs_create_file(cstr!("cache_only"),0o600,(*map).debugfs,&mut (*map).cache_only as *mut _ as *mut _,&REGMAP_CACHE_ONLY_FOPS);debugfs_create_bool(cstr!("cache_dirty"),0o400,(*map).debugfs,&mut (*map).cache_dirty);debugfs_create_file(cstr!("cache_bypass"),0o600,(*map).debugfs,&mut (*map).cache_bypass as *mut _ as *mut _,&REGMAP_CACHE_BYPASS_FOPS);}if !(*map).cache_ops.is_null(){if let Some(f)=(*(*map).cache_ops).debugfs_init{f(map);}} }

pub unsafe fn regmap_debugfs_exit(map:*mut Regmap){if !(*map).debugfs.is_null(){debugfs_remove_recursive((*map).debugfs);mutex_lock(&mut (*map).cache_lock);regmap_debugfs_free_dump_cache(map);mutex_unlock(&mut (*map).cache_lock);if (*map).debugfs_dummy_id>=0{ida_free(&mut DUMMY_IDA,(*map).debugfs_dummy_id);(*map).debugfs_dummy_id=-1;}kfree((*map).debugfs_name as *mut _);(*map).debugfs_name=core::ptr::null_mut();}else{mutex_lock(&mut REGMAP_DEBUGFS_EARLY_LOCK);list_for_each_entry_safe::<RegmapDebugfsNode>(&mut REGMAP_DEBUGFS_EARLY_LIST,|node|{if (*node).map==map{list_del(&mut (*node).link);kfree(node as *mut _);}});mutex_unlock(&mut REGMAP_DEBUGFS_EARLY_LOCK);}}
pub unsafe fn regmap_debugfs_initcall(){REGMAP_DEBUGFS_ROOT=debugfs_create_dir(cstr!("regmap"),core::ptr::null_mut());mutex_lock(&mut REGMAP_DEBUGFS_EARLY_LOCK);list_for_each_entry_safe::<RegmapDebugfsNode>(&mut REGMAP_DEBUGFS_EARLY_LIST,|node|{regmap_debugfs_init((*node).map);list_del(&mut (*node).link);kfree(node as *mut _);});mutex_unlock(&mut REGMAP_DEBUGFS_EARLY_LOCK);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
