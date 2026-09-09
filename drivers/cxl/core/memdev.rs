// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2020 Intel Corporation. */
// Linux kernel dependencies and build-time configuration are supplied externally.

static mut CXL_MEMDEV_RWSEM: rw_semaphore = DECLARE_RWSEM!();
const CXL_MEM_MAX_DEVS: i32 = 65536;
static mut CXL_MEM_MAJOR: i32 = 0;
static mut CXL_MEMDEV_IDA: ida = DEFINE_IDA!();

unsafe fn cxl_memdev_release(dev: *mut device) {
    let cxlmd = to_cxl_memdev(dev);
    let parent = (*dev).parent;
    ida_free(&mut CXL_MEMDEV_IDA, (*cxlmd).id);
    kfree(cxlmd as *mut core::ffi::c_void);
    put_device(parent);
}

unsafe fn cxl_memdev_devnode(dev: *const device, _mode: *mut umode_t,
                             _uid: *mut kuid_t, _gid: *mut kgid_t) -> *mut i8 {
    kasprintf(GFP_KERNEL, c"cxl/%s", dev_name(dev))
}

unsafe fn firmware_version_show(dev: *mut device, _attr: *mut device_attribute,
                                buf: *mut i8) -> ssize_t {
    let cxlmd = to_cxl_memdev(dev);
    let cxlds = (*cxlmd).cxlds;
    let mds = to_cxl_memdev_state(cxlds);
    if mds.is_null() { return sysfs_emit(buf, c"\n"); }
    sysfs_emit(buf, c"%.16s\n", (*mds).firmware_version)
}
static DEVICE_ATTR_RO!(firmware_version);

unsafe fn payload_max_show(dev: *mut device, _attr: *mut device_attribute,
                           buf: *mut i8) -> ssize_t {
    let cxlmd = to_cxl_memdev(dev); let cxlds = (*cxlmd).cxlds;
    let mds = to_cxl_memdev_state(cxlds);
    if mds.is_null() { return sysfs_emit(buf, c"\n"); }
    sysfs_emit(buf, c"%zu\n", (*cxlds).cxl_mbox.payload_size)
}
static DEVICE_ATTR_RO!(payload_max);

unsafe fn label_storage_size_show(dev: *mut device, _attr: *mut device_attribute,
                                  buf: *mut i8) -> ssize_t {
    let cxlmd = to_cxl_memdev(dev); let cxlds = (*cxlmd).cxlds;
    let mds = to_cxl_memdev_state(cxlds);
    if mds.is_null() { return sysfs_emit(buf, c"\n"); }
    sysfs_emit(buf, c"%zu\n", (*mds).lsa_size)
}
static DEVICE_ATTR_RO!(label_storage_size);

unsafe fn cxl_ram_size(cxlds: *mut cxl_dev_state) -> resource_size_t {
    // Static RAM is only expected at partition 0.
    if (*cxlds).part[0].mode != CXL_PARTMODE_RAM { return 0; }
    resource_size(&(*cxlds).part[0].res)
}
unsafe fn ram_size_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let cxlds = (*to_cxl_memdev(dev)).cxlds;
    sysfs_emit(buf, c"%#llx\n", cxl_ram_size(cxlds))
}
static mut DEV_ATTR_RAM_SIZE: device_attribute = __ATTR!(size, 0o444, ram_size_show, None);

unsafe fn pmem_size_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let cxlds = (*to_cxl_memdev(dev)).cxlds;
    sysfs_emit(buf, c"%#llx\n", cxl_pmem_size(cxlds))
}
static mut DEV_ATTR_PMEM_SIZE: device_attribute = __ATTR!(size, 0o444, pmem_size_show, None);

unsafe fn serial_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    sysfs_emit(buf, c"%#llx\n", (*(*to_cxl_memdev(dev)).cxlds).serial)
}
static DEVICE_ATTR_RO!(serial);
unsafe fn numa_node_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    sysfs_emit(buf, c"%d\n", dev_to_node(dev))
}
static DEVICE_ATTR_RO!(numa_node);

unsafe fn security_state_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let cxlds = (*to_cxl_memdev(dev)).cxlds;
    let mds = to_cxl_memdev_state(cxlds); let mbox = &mut (*cxlds).cxl_mbox;
    let state = (*mds).security.state; let mut rc = 0;
    mutex_lock(&mut mbox.mbox_mutex);
    if (*mds).security.sanitize_active { rc = sysfs_emit(buf, c"sanitize\n"); }
    mutex_unlock(&mut mbox.mbox_mutex);
    if rc != 0 { return rc; }
    if state & CXL_PMEM_SEC_STATE_USER_PASS_SET == 0 { return sysfs_emit(buf, c"disabled\n"); }
    if state & (CXL_PMEM_SEC_STATE_FROZEN | CXL_PMEM_SEC_STATE_MASTER_PLIMIT | CXL_PMEM_SEC_STATE_USER_PLIMIT) != 0 { return sysfs_emit(buf, c"frozen\n"); }
    if state & CXL_PMEM_SEC_STATE_LOCKED != 0 { return sysfs_emit(buf, c"locked\n"); }
    sysfs_emit(buf, c"unlocked\n")
}
static mut DEV_ATTR_SECURITY_STATE: device_attribute = __ATTR!(state, 0o444, security_state_show, None);

unsafe fn security_sanitize_store(dev: *mut device, _attr: *mut device_attribute,
                                  buf: *const i8, len: usize) -> ssize_t {
    let cxlmd = to_cxl_memdev(dev); let mut sanitize = false;
    if kstrtobool(buf, &mut sanitize) != 0 || !sanitize { return -EINVAL as ssize_t; }
    let rc = cxl_mem_sanitize(cxlmd, CXL_MBOX_OP_SANITIZE); if rc != 0 { return rc; } len as ssize_t
}
static mut DEV_ATTR_SECURITY_SANITIZE: device_attribute = __ATTR!(sanitize, 0o200, None, security_sanitize_store);
unsafe fn security_erase_store(dev: *mut device, _attr: *mut device_attribute,
                               buf: *const i8, len: usize) -> ssize_t {
    let cxlmd = to_cxl_memdev(dev); let mut erase = false;
    if kstrtobool(buf, &mut erase) != 0 || !erase { return -EINVAL as ssize_t; }
    let rc = cxl_mem_sanitize(cxlmd, CXL_MBOX_OP_SECURE_ERASE); if rc != 0 { return rc; } len as ssize_t
}
static mut DEV_ATTR_SECURITY_ERASE: device_attribute = __ATTR!(erase, 0o200, None, security_erase_store);

pub unsafe fn cxl_memdev_has_poison_cmd(cxlmd: *mut cxl_memdev, cmd: poison_cmd_enabled_bits) -> bool {
    let mds = to_cxl_memdev_state((*cxlmd).cxlds); if mds.is_null() { return false; }
    test_bit(cmd, (*mds).poison.enabled_cmds)
}
unsafe fn cxl_get_poison_by_memdev(cxlmd: *mut cxl_memdev) -> i32 {
    let cxlds = (*cxlmd).cxlds; let mut rc = 0;
    for i in 0..(*cxlds).nr_partitions { let res = &(*cxlds).part[i].res;
        rc = cxl_mem_get_poison(cxlmd, res.start, resource_size(res), core::ptr::null_mut());
        if rc == -EFAULT && (*cxlds).part[i].mode == CXL_PARTMODE_RAM { rc = 0; }
    } rc
}
pub unsafe fn cxl_trigger_poison_list(cxlmd: *mut cxl_memdev) -> i32 {
    let port = (*cxlmd).endpoint; if port.is_null() || !is_cxl_endpoint(port) { return -EINVAL; }
    let mut rc = acquire_rwsem_read_intr(&mut CXL_RWSEM_REGION); if rc != 0 { return rc; }
    rc = acquire_rwsem_read_intr(&mut CXL_RWSEM_DPA); if rc != 0 { return rc; }
    if cxl_num_decoders_committed(port) == 0 { cxl_get_poison_by_memdev(cxlmd) } else { cxl_get_poison_by_endpoint(port) }
}
EXPORT_SYMBOL_NS_GPL!(cxl_trigger_poison_list, "CXL");

unsafe fn cxl_validate_poison_dpa(cxlmd: *mut cxl_memdev, dpa: u64) -> i32 {
    if !IS_ENABLED!(CONFIG_DEBUG_FS) { return 0; }
    let r = &(*(*cxlmd).cxlds).dpa_res;
    if resource_size(r) == 0 || !cxl_resource_contains_addr(r, dpa) || !IS_ALIGNED!(dpa, 64) { return -EINVAL; } 0
}
pub unsafe fn cxl_inject_poison_locked(cxlmd: *mut cxl_memdev, dpa: u64) -> i32 {
    if !IS_ENABLED!(CONFIG_DEBUG_FS) { return 0; }
    let mbox = &mut (*(*cxlmd).cxlds).cxl_mbox; let mut inject = cxl_mbox_inject_poison { address: cpu_to_le64(dpa) };
    let cmd = cxl_mbox_cmd { opcode: CXL_MBOX_OP_INJECT_POISON, size_in: size_of::<cxl_mbox_inject_poison>(), payload_in: &mut inject as *mut _, ..Default::default() };
    let rc = cxl_internal_send_cmd(mbox, &cmd); if rc != 0 { return rc; } 0
}
pub unsafe fn cxl_inject_poison(cxlmd: *mut cxl_memdev, dpa: u64) -> i32 { acquire_rwsem_read_intr(&mut CXL_RWSEM_REGION); acquire_rwsem_read_intr(&mut CXL_RWSEM_DPA); cxl_inject_poison_locked(cxlmd, dpa) }
EXPORT_SYMBOL_NS_GPL!(cxl_inject_poison, "CXL");
pub unsafe fn cxl_clear_poison_locked(cxlmd: *mut cxl_memdev, dpa: u64) -> i32 {
    if !IS_ENABLED!(CONFIG_DEBUG_FS) { return 0; } let mbox=&mut (*(*cxlmd).cxlds).cxl_mbox;
    let mut clear=cxl_mbox_clear_poison{address:cpu_to_le64(dpa)}; let cmd=cxl_mbox_cmd{opcode:CXL_MBOX_OP_CLEAR_POISON,size_in:size_of::<cxl_mbox_clear_poison>(),payload_in:&mut clear as *mut _,..Default::default()}; cxl_internal_send_cmd(mbox,&cmd)
}
pub unsafe fn cxl_clear_poison(cxlmd:*mut cxl_memdev,dpa:u64)->i32 { acquire_rwsem_read_intr(&mut CXL_RWSEM_REGION); acquire_rwsem_read_intr(&mut CXL_RWSEM_DPA); cxl_clear_poison_locked(cxlmd,dpa) }
EXPORT_SYMBOL_NS_GPL!(cxl_clear_poison,"CXL");

unsafe fn add_part(info:*mut cxl_dpa_info,start:u64,size:u64,mode:cxl_partition_mode){if size==0{return;} let i=(*info).nr_partitions; (*info).part[i].range=range{start,end:start+size-1};(*info).part[i].mode=mode;(*info).nr_partitions+=1;}
pub unsafe fn cxl_mem_dpa_fetch(mds:*mut cxl_memdev_state,info:*mut cxl_dpa_info)->i32{let cxlds=&mut (*mds).cxlds;if !cxlds.media_ready{(*info).size=0;return 0;}(*info).size=(*mds).total_bytes;if (*mds).partition_align_bytes==0{add_part(info,0,(*mds).volatile_only_bytes,CXL_PARTMODE_RAM);add_part(info,(*mds).volatile_only_bytes,(*mds).persistent_only_bytes,CXL_PARTMODE_PMEM);return 0;}let rc=cxl_mem_get_partition_info(mds);if rc!=0{return rc;}add_part(info,0,(*mds).active_volatile_bytes,CXL_PARTMODE_RAM);add_part(info,(*mds).active_volatile_bytes,(*mds).active_persistent_bytes,CXL_PARTMODE_PMEM);0}
EXPORT_SYMBOL_NS_GPL!(cxl_mem_dpa_fetch,"CXL");
pub unsafe fn cxl_set_capacity(cxlds:*mut cxl_dev_state,capacity:u64)->i32{let mut info=cxl_dpa_info{size:capacity,..Default::default()};add_part(&mut info,0,capacity,CXL_PARTMODE_RAM);cxl_dpa_setup(cxlds,&mut info)}
EXPORT_SYMBOL_NS_GPL!(cxl_set_capacity,"CXL");

pub unsafe fn cxl_memdev_update_perf(cxlmd:*mut cxl_memdev){sysfs_update_group(&mut (*cxlmd).dev.kobj,&CXL_MEMDEV_RAM_ATTRIBUTE_GROUP);sysfs_update_group(&mut (*cxlmd).dev.kobj,&CXL_MEMDEV_PMEM_ATTRIBUTE_GROUP)}
EXPORT_SYMBOL_NS_GPL!(cxl_memdev_update_perf,"CXL");
pub unsafe fn is_cxl_memdev(dev:*const device)->bool{(*dev).type_==&CXL_CLASS_MEMDEV_TYPE||(*dev).type_==&CXL_MEMDEV_TYPE}
EXPORT_SYMBOL_NS_GPL!(is_cxl_memdev,"CXL");

unsafe fn cxl_memdev_shutdown(dev:*mut device){let cxlmd=to_cxl_memdev(dev);guard_rwsem_write!(&mut CXL_MEMDEV_RWSEM);(*cxlmd).cxlds=core::ptr::null_mut();}
unsafe fn cxl_memdev_unregister(data:*mut core::ffi::c_void){let cxlmd=data as *mut cxl_memdev;cdev_device_del(&mut (*cxlmd).cdev,&mut (*cxlmd).dev);cxl_memdev_shutdown(&mut (*cxlmd).dev);put_device(&mut (*cxlmd).dev);}
unsafe fn detach_memdev(work:*mut work_struct){let cxlmd=container_of!(work,cxl_memdev,detach_work);if (*cxlmd).attach.is_some(){device_release_driver((*cxlmd).dev.parent)}else{device_release_driver(&mut (*cxlmd).dev); }put_device(&mut (*cxlmd).dev);}

pub unsafe fn _devm_cxl_dev_state_create(dev:*mut device,ty:cxl_devtype,serial:u64,dvsec:u16,size:usize,has_mbox:bool)->*mut cxl_dev_state{let p=devm_kzalloc(dev,size,GFP_KERNEL) as *mut cxl_dev_state;if p.is_null(){return p;}(*p).dev=dev;(*p).type_=ty;(*p).serial=serial;(*p).cxl_dvsec=dvsec;(*p).reg_map.host=dev;(*p).reg_map.resource=CXL_RESOURCE_NONE;if has_mbox{(*p).cxl_mbox.host=dev;}p}
EXPORT_SYMBOL_NS_GPL!(_devm_cxl_dev_state_create,"CXL");

unsafe fn __cxl_memdev_ioctl(cxlmd:*mut cxl_memdev,cmd:u32,arg:usize)->isize{let mds=to_cxl_memdev_state((*cxlmd).cxlds);let mbox=&mut (*mds).cxlds.cxl_mbox;match cmd{CXL_MEM_QUERY_COMMANDS=>cxl_query_cmd(mbox,arg as *mut core::ffi::c_void) as isize,CXL_MEM_SEND_COMMAND=>cxl_send_cmd(mbox,arg as *mut core::ffi::c_void) as isize,_=>-ENOTTY as isize}}
unsafe fn cxl_memdev_ioctl(file:*mut file,cmd:u32,arg:usize)->isize{let cxlmd=(*file).private_data as *mut cxl_memdev;guard_rwsem_read!(&mut CXL_MEMDEV_RWSEM);let cxlds=(*cxlmd).cxlds;if !cxlds.is_null()&&(*cxlds).type_==CXL_DEVTYPE_CLASSMEM{__cxl_memdev_ioctl(cxlmd,cmd,arg)}else{-ENXIO as isize}}
unsafe fn cxl_memdev_open(inode:*mut inode,file:*mut file)->i32{let cxlmd=container_of!((*inode).i_cdev,cxl_memdev,cdev);get_device(&mut (*cxlmd).dev);(*file).private_data=cxlmd as *mut _;0}
unsafe fn cxl_memdev_release_file(inode:*mut inode,_file:*mut file)->i32{let cxlmd=container_of!((*inode).i_cdev,cxl_memdev,cdev);put_device(&mut (*cxlmd).dev);0}

pub unsafe fn devm_cxl_setup_fw_upload(host:*mut device,mds:*mut cxl_memdev_state)->i32{if !test_bit(CXL_MEM_COMMAND_ID_GET_FW_INFO,(*mds).cxlds.cxl_mbox.enabled_cmds){return 0;}let dev=&mut (*(*mds).cxlds.cxlmd).dev;let fwl=firmware_upload_register(THIS_MODULE,dev,dev_name(dev),&CXL_MEMDEV_FW_OPS,mds as *mut _);if IS_ERR(fwl){return PTR_ERR(fwl)}devm_add_action_or_reset(host,cxl_remove_fw_upload,fwl as *mut _)}
EXPORT_SYMBOL_NS_GPL!(devm_cxl_setup_fw_upload,"CXL");
pub unsafe fn __devm_cxl_add_memdev(cxlds:*mut cxl_dev_state,attach:*const cxl_memdev_attach)->*mut cxl_memdev{let cxlmd=cxl_memdev_alloc(cxlds,&CXL_MEMDEV_FOPS,attach);if IS_ERR(cxlmd){return cxlmd;}let rc=dev_set_name(&mut (*cxlmd).dev,c"mem%d",(*cxlmd).id);if rc!=0{return ERR_PTR(rc) as *mut _;}let rc=cxlmd_add(cxlmd,cxlds);if rc!=0{return ERR_PTR(rc) as *mut _;}cxl_memdev_autoremove(cxlmd)}
EXPORT_SYMBOL_FOR_MODULES!(__devm_cxl_add_memdev,"cxl_mem");
pub unsafe fn devm_cxl_sanitize_setup_notifier(host:*mut device,cxlmd:*mut cxl_memdev)->i32{let mds=to_cxl_memdev_state((*cxlmd).cxlds);if !test_bit(CXL_SEC_ENABLED_SANITIZE,(*mds).security.enabled_cmds){return 0;}devm_add_action_or_reset(host,sanitize_teardown_notifier,mds as *mut _)}
EXPORT_SYMBOL_NS_GPL!(devm_cxl_sanitize_setup_notifier,"CXL");
pub unsafe fn cxl_memdev_init()->i32{let mut devt=0;let rc=alloc_chrdev_region(&mut devt,0,CXL_MEM_MAX_DEVS,c"cxl");if rc!=0{return rc;}CXL_MEM_MAJOR=MAJOR(devt);0}
pub unsafe fn cxl_memdev_exit(){unregister_chrdev_region(MKDEV!(CXL_MEM_MAJOR,0),CXL_MEM_MAX_DEVS);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
