// SPDX-License-Identifier: GPL-2.0
// Register cache access API

// External kernel/regmap declarations are supplied by other translation units.

static mut CACHE_TYPES: [*const RegcacheOps; 4] = [
    &regcache_flat_sparse_ops, &regcache_rbtree_ops, &regcache_maple_ops,
    &regcache_flat_ops,
];

unsafe fn regcache_defaults_cmp(a: *const core::ffi::c_void, b: *const core::ffi::c_void) -> i32 {
    let x = &*(a as *const RegDefault); let y = &*(b as *const RegDefault);
    if x.reg > y.reg { 1 } else if x.reg < y.reg { -1 } else { 0 }
}

pub unsafe fn regcache_sort_defaults(defaults: *mut RegDefault, ndefaults: u32) {
    sort(defaults as *mut core::ffi::c_void, ndefaults as usize, core::mem::size_of::<RegDefault>(), regcache_defaults_cmp, core::ptr::null_mut());
}

unsafe fn regcache_count_cacheable_registers(map: *mut Regmap) -> i32 {
    let mut count = 0;
    for i in 0..(*map).num_reg_defaults_raw {
        let reg = i * (*map).reg_stride;
        if regmap_readable(map, reg) && !regmap_volatile(map, reg) { count += 1; }
    }
    count
}

unsafe fn regcache_hw_init(map: *mut Regmap) -> i32 {
    let mut ret; let mut reg; let mut val = 0; let mut tmp_buf: *mut core::ffi::c_void;
    if (*map).reg_defaults_raw.is_null() {
        let cache_bypass = (*map).cache_bypass; dev_dbg((*map).dev, "No cache defaults, reading back from HW\n");
        (*map).cache_bypass = true; tmp_buf = kmalloc((*map).cache_size_raw, GFP_KERNEL);
        if tmp_buf.is_null() { return -ENOMEM; }
        ret = regmap_raw_read(map, 0, tmp_buf, (*map).cache_size_raw); (*map).cache_bypass = cache_bypass;
        if ret == 0 { (*map).reg_defaults_raw = tmp_buf; (*map).cache_free = true; } else { kfree(tmp_buf); }
    }
    let mut j = 0;
    for i in 0..(*map).num_reg_defaults_raw {
        reg = i * (*map).reg_stride;
        if !regmap_readable(map, reg) || regmap_volatile(map, reg) { continue; }
        if !(*map).reg_defaults_raw.is_null() { val = regcache_get_val(map, (*map).reg_defaults_raw, i); }
        else { let cache_bypass = (*map).cache_bypass; (*map).cache_bypass = true; ret = regmap_read(map, reg, &mut val); (*map).cache_bypass = cache_bypass; if ret != 0 { dev_err((*map).dev, "Failed to read %x: %d\n", reg, ret); return ret; } }
        (*map).reg_defaults.add(j as usize).write(RegDefault { reg, def: val }); j += 1;
    } 0
}

unsafe fn regcache_hw_exit(map: *mut Regmap) { if (*map).cache_free { kfree((*map).reg_defaults_raw); } }

pub unsafe fn regcache_init(map: *mut Regmap, config: *const RegmapConfig) -> i32 {
    let mut sort_defaults = false; let mut reg_prev = 0; let mut count = 0; let mut ret; let mut tmp_buf;
    if (*map).cache_type == REGCACHE_NONE { if !(*config).reg_defaults.is_null() || (*config).num_reg_defaults_raw != 0 { dev_warn((*map).dev, "No cache used with register defaults set!\n"); } (*map).cache_bypass = true; return 0; }
    if !(*config).reg_defaults.is_null() && (*config).num_reg_defaults == 0 { dev_err((*map).dev, "Register defaults are set without the number!\n"); return -EINVAL; }
    if (*config).num_reg_defaults != 0 && (*config).reg_defaults.is_null() { dev_err((*map).dev, "Register defaults number are set without the reg!\n"); return -EINVAL; }
    for i in 0..(*config).num_reg_defaults { let d = &*(*config).reg_defaults.add(i as usize); if d.reg % (*map).reg_stride != 0 { return -EINVAL; } if reg_prev > d.reg { sort_defaults = true; } reg_prev = d.reg; }
    let mut i = 0; while i < 4 && (*CACHE_TYPES[i]).type_ != (*map).cache_type { i += 1; } if i == 4 { dev_err((*map).dev, "Could not match cache type: %d\n", (*map).cache_type); return -EINVAL; }
    (*map).num_reg_defaults = (*config).num_reg_defaults; (*map).num_reg_defaults_raw = (*config).num_reg_defaults_raw; (*map).reg_defaults_raw = (*config).reg_defaults_raw; (*map).cache_word_size = BITS_TO_BYTES((*config).val_bits); (*map).cache_size_raw = (*map).cache_word_size * (*config).num_reg_defaults_raw; (*map).cache = core::ptr::null_mut(); (*map).cache_ops = CACHE_TYPES[i];
    if (*map).cache_ops.read().read.is_none() || (*map).cache_ops.read().write.is_none() || (*map).cache_ops.read().name.is_null() { return -EINVAL; }
    if !(*config).reg_defaults.is_null() { tmp_buf = kmemdup_array((*config).reg_defaults as *const core::ffi::c_void, (*map).num_reg_defaults, core::mem::size_of::<RegDefault>(), GFP_KERNEL); if tmp_buf.is_null() { return -ENOMEM; } if sort_defaults { dev_warn((*map).dev, "Driver needs fixing: Unsorted reg_defaults, sorting the copy\n"); regcache_sort_defaults(tmp_buf as *mut RegDefault, (*map).num_reg_defaults); } (*map).reg_defaults = tmp_buf as *mut RegDefault; }
    else if (*map).num_reg_defaults_raw != 0 { count = regcache_count_cacheable_registers(map); if count == 0 { (*map).cache_bypass = true; } if (*map).cache_bypass { return 0; } (*map).num_reg_defaults = count as u32; (*map).reg_defaults = kmalloc_objs::<RegDefault>(count as usize); if (*map).reg_defaults.is_null() { return -ENOMEM; } }
    if !(*map).max_register_is_set && (*map).num_reg_defaults_raw != 0 { (*map).max_register = ((*map).num_reg_defaults_raw - 1) * (*map).reg_stride; (*map).max_register_is_set = true; }
    if let Some(f) = (*map).cache_ops.read().init { dev_dbg((*map).dev, "Initializing %s cache\n", (*map).cache_ops.read().name); ((*map).lock.unwrap())((*map).lock_arg); ret = f(map); ((*map).unlock.unwrap())((*map).lock_arg); if ret != 0 { kfree((*map).reg_defaults as *mut _); return ret; } }
    if count != 0 { ret = regcache_hw_init(map); if ret != 0 { regcache_hw_exit(map); if let Some(f) = (*map).cache_ops.read().exit { f(map); } kfree((*map).reg_defaults as *mut _); return ret; } }
    if let Some(f) = (*map).cache_ops.read().populate { if (*map).num_reg_defaults != 0 || (*map).reg_default_cb.is_some() { dev_dbg((*map).dev, "Populating %s cache\n", (*map).cache_ops.read().name); ((*map).lock.unwrap())((*map).lock_arg); ret = f(map); ((*map).unlock.unwrap())((*map).lock_arg); if ret != 0 { regcache_hw_exit(map); if let Some(e) = (*map).cache_ops.read().exit { e(map); } kfree((*map).reg_defaults as *mut _); return ret; } } } 0
}

pub unsafe fn regcache_exit(map: *mut Regmap) { if (*map).cache_type == REGCACHE_NONE { return; } regcache_hw_exit(map); if let Some(f) = (*map).cache_ops.read().exit { f(map); } kfree((*map).reg_defaults as *mut _); }

pub unsafe fn regcache_read(map: *mut Regmap, reg: u32, value: *mut u32) -> i32 { if (*map).cache_type == REGCACHE_NONE { return -EINVAL; } if !regmap_volatile(map, reg) { let r = ((*map).cache_ops.read().read.unwrap())(map, reg, value); if r == 0 { trace_regmap_reg_read_cache(map, reg, *value); } return r; } -EINVAL }
pub unsafe fn regcache_write(map: *mut Regmap, reg: u32, value: u32) -> i32 { if (*map).cache_type == REGCACHE_NONE { return 0; } if !regmap_volatile(map, reg) { return ((*map).cache_ops.read().write.unwrap())(map, reg, value); } 0 }
pub unsafe fn regcache_reg_needs_sync(map: *mut Regmap, reg: u32, val: u32) -> bool { if !regmap_writeable(map, reg) { return false; } if !(*map).no_sync_defaults { return true; } let ret = regcache_lookup_reg(map, reg); !(ret >= 0 && val == (*map).reg_defaults.add(ret as usize).read().def) }

unsafe fn regcache_default_sync(map: *mut Regmap, min: u32, max: u32) -> i32 { let mut reg = min; while reg <= max { if !regmap_volatile(map, reg) && regmap_writeable(map, reg) { let mut val=0; let ret=regcache_read(map,reg,&mut val); if ret == -ENOENT { reg += (*map).reg_stride; continue; } if ret != 0 { return ret; } if regcache_reg_needs_sync(map,reg,val) { (*map).cache_bypass=true; let r=_regmap_write(map,reg,val); (*map).cache_bypass=false; if r != 0 { dev_err((*map).dev,"Unable to sync register %#x. %d\n",reg,r); return r; } dev_dbg((*map).dev,"Synced register %#x, value %#x\n",reg,val); } } reg += (*map).reg_stride; } 0 }

unsafe fn rbtree_all(_key: *const core::ffi::c_void, _node: *const RbNode) -> i32 { 0 }

pub unsafe fn regcache_sync(map: *mut Regmap) -> i32 { if (*map).cache_type == REGCACHE_NONE { return -EINVAL; } ((*map).lock.unwrap())((*map).lock_arg); if (*map).cache_only { ((*map).unlock.unwrap())((*map).lock_arg); return -EINVAL; } let bypass=(*map).cache_bypass; let name=(*map).cache_ops.read().name; trace_regcache_sync(map,name,"start"); let mut sync_ret=0; if (*map).cache_dirty { (*map).cache_bypass=true; for i in 0..(*map).patch_regs { sync_ret=_regmap_write(map,(*map).patch.add(i as usize).read().reg,(*map).patch.add(i as usize).read().def); if sync_ret!=0 { break; } } (*map).cache_bypass=false; if sync_ret==0 { sync_ret=if let Some(f)=(*map).cache_ops.read().sync { f(map,0,(*map).max_register) } else { regcache_default_sync(map,0,(*map).max_register) }; if sync_ret==0 { (*map).cache_dirty=false; } } (*map).cache_bypass=bypass; (*map).no_sync_defaults=false; ((*map).unlock.unwrap())((*map).lock_arg); regmap_async_complete(map); trace_regcache_sync(map,name,"stop"); sync_ret }

pub unsafe fn regcache_sync_region(map:*mut Regmap,min:u32,max:u32)->i32 { if (*map).cache_type==REGCACHE_NONE{return -EINVAL;} ((*map).lock.unwrap())((*map).lock_arg); if (*map).cache_only { ((*map).unlock.unwrap())((*map).lock_arg); return -EINVAL;} let bypass=(*map).cache_bypass; let mut ret=0; if (*map).cache_dirty { (*map).async_=true; ret=if let Some(f)=(*map).cache_ops.read().sync {f(map,min,max)} else {regcache_default_sync(map,min,max)}; } (*map).cache_bypass=bypass; (*map).async_=false; (*map).no_sync_defaults=false; ((*map).unlock.unwrap())((*map).lock_arg); regmap_async_complete(map); ret }
pub unsafe fn regcache_drop_region(map:*mut Regmap,min:u32,max:u32)->i32 { if (*map).cache_ops.is_null() || (*map).cache_ops.read().drop.is_none(){return -EINVAL;} ((*map).lock.unwrap())((*map).lock_arg); let r=((*map).cache_ops.read().drop.unwrap())(map,min,max); ((*map).unlock.unwrap())((*map).lock_arg); r }
pub unsafe fn regcache_cache_only(map:*mut Regmap,enable:bool){((*map).lock.unwrap())((*map).lock_arg); (*map).cache_only=enable; trace_regmap_cache_only(map,enable);((*map).unlock.unwrap())((*map).lock_arg);}
pub unsafe fn regcache_mark_dirty(map:*mut Regmap){((*map).lock.unwrap())((*map).lock_arg);(*map).cache_dirty=true;(*map).no_sync_defaults=true;((*map).unlock.unwrap())((*map).lock_arg);}
pub unsafe fn regcache_cache_bypass(map:*mut Regmap,enable:bool){((*map).lock.unwrap())((*map).lock_arg);(*map).cache_bypass=enable;trace_regmap_cache_bypass(map,enable);((*map).unlock.unwrap())((*map).lock_arg);}
pub unsafe fn regcache_reg_cached(map:*mut Regmap,reg:u32)->bool{let mut v=0;((*map).lock.unwrap())((*map).lock_arg);let r=regcache_read(map,reg,&mut v);((*map).unlock.unwrap())((*map).lock_arg);r==0}
pub unsafe fn regcache_set_val(map:*mut Regmap,base:*mut core::ffi::c_void,idx:u32,val:u32){if let Some(f)=(*map).format.format_val{f((base as *mut u8).add((*map).cache_word_size as usize*idx as usize) as *mut _,val,0);return;}match (*map).cache_word_size{1=>(base as *mut u8).add(idx as usize).write(val as u8),2=>(base as *mut u16).add(idx as usize).write(val as u16),4=>(base as *mut u32).add(idx as usize).write(val),_=>unimplemented!()}}
pub unsafe fn regcache_get_val(map:*mut Regmap,base:*const core::ffi::c_void,idx:u32)->u32{if base.is_null(){return -EINVAL as u32;}if let Some(f)=(*map).format.parse_val{return f(regcache_get_val_addr(map,base,idx));}match (*map).cache_word_size{1=>(base as *const u8).add(idx as usize).read() as u32,2=>(base as *const u16).add(idx as usize).read() as u32,4=>(base as *const u32).add(idx as usize).read(),_=>unimplemented!()}}
pub unsafe fn regcache_lookup_reg(map:*mut Regmap,reg:u32)->i32{for i in 0..(*map).num_reg_defaults{if (*map).reg_defaults.add(i as usize).read().reg==reg{return i as i32;}}-ENOENT}
unsafe fn regcache_reg_present(cache_present:*mut usize,idx:u32)->bool{cache_present.is_null()||test_bit(idx,cache_present)}
pub unsafe fn regcache_sync_val(map:*mut Regmap,reg:u32,val:u32)->i32{if !regcache_reg_needs_sync(map,reg,val){return 0;}(*map).cache_bypass=true;let r=_regmap_write(map,reg,val);(*map).cache_bypass=false;r}
pub unsafe fn regcache_sync_block(map:*mut Regmap,block:*mut core::ffi::c_void,cache_present:*mut usize,block_base:u32,start:u32,end:u32)->i32{let mut i=start;while i<end{let reg=block_base+i*(*map).reg_stride;if regcache_reg_present(cache_present,i)&&regmap_writeable(map,reg){let v=regcache_get_val(map,block,i);let r=regcache_sync_val(map,reg,v);if r!=0{return r;}}i+=1;}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
