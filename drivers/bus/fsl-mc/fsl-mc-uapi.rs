// SPDX-License-Identifier: GPL-2.0
/*
 * Management Complex (MC) userspace support
 *
 * Copyright 2021 NXP
 */

// The following types, constants, macros, and functions are supplied by the
// corresponding kernel dependencies.

#[repr(C)]
struct UapiPrivData {
    uapi: *mut FslMcUapi,
    mc_io: *mut FslMcIo,
}

#[repr(C)]
struct FslMcCmdDesc {
    cmdid_value: u16,
    cmdid_mask: u16,
    size: i32,
    token: bool,
    flags: i32,
}

const FSL_MC_CHECK_MODULE_ID: i32 = 1 << 0;
const FSL_MC_CAP_NET_ADMIN_NEEDED: i32 = 1 << 1;

#[repr(usize)]
enum FslMcCmdIndex {
    DpdbgDump = 0,
    DpdbgSet,
    DprcGetContainerId,
    DprcCreateCont,
    DprcDestroyCont,
    DprcAssign,
    DprcUnassign,
    DprcGetObjCount,
    DprcGetObj,
    DprcGetResCount,
    DprcGetResIds,
    DprcSetObjLabel,
    DprcSetLocked,
    DprcConnect,
    DprcDisconnect,
    DprcGetPool,
    DprcGetPoolCount,
    DprcGetConnection,
    DprcGetMem,
    DpciGetLinkState,
    DpciGetPeerAttr,
    DpaiopGetSlVersion,
    DpaiopGetState,
    DpmngGetVersion,
    DpseciGetTxQueue,
    DpmacGetCounter,
    DpmacGetMacAddr,
    DpniSetPrimMac,
    DpniGetPrimMac,
    DpniGetStatistics,
    DpniGetLinkState,
    DpniGetMaxFrameLength,
    DpswGetTaildrop,
    DpswSetTaildrop,
    DpswIfGetCounter,
    DpswIfGetMaxFrameLength,
    DpdmuxGetCounter,
    DpdmuxIfGetMaxFrameLength,
    GetAttr,
    GetIrqMask,
    GetIrqStatus,
    Close,
    Open,
    GetApiVersion,
    Destroy,
    Create,
}

const fn desc(cmdid_value: u16, cmdid_mask: u16, size: i32, token: bool, flags: i32) -> FslMcCmdDesc {
    FslMcCmdDesc { cmdid_value, cmdid_mask, size, token, flags }
}

static mut FSL_MC_ACCEPTED_CMDS: [FslMcCmdDesc; 47] = [
    desc(0x1300, 0xFFF0, 28, true, 0), desc(0x1400, 0xFFF0, 28, true, 0),
    desc(0x8300, 0xFFF0, 8, false, 0),
    desc(0x1510, 0xFFF0, 40, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1520, 0xFFF0, 12, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1570, 0xFFF0, 40, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1580, 0xFFF0, 40, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1590, 0xFFF0, 16, true, 0), desc(0x15A0, 0xFFF0, 12, true, 0),
    desc(0x15B0, 0xFFF0, 32, true, 0), desc(0x15C0, 0xFFF0, 40, true, 0),
    desc(0x1610, 0xFFF0, 48, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x16B0, 0xFFF0, 16, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1670, 0xFFF0, 56, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1680, 0xFFF0, 32, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x1690, 0xFFF0, 12, true, 0), desc(0x16A0, 0xFFF0, 8, true, 0),
    desc(0x16C0, 0xFFF0, 32, true, 0), desc(0x16D0, 0xFFF0, 12, true, 0),
    desc(0x0E10, 0xFFF0, 8, true, 0), desc(0x0E20, 0xFFF0, 8, true, 0),
    desc(0x2820, 0xFFF0, 8, true, 0), desc(0x2830, 0xFFF0, 8, true, 0),
    desc(0x8310, 0xFFF0, 8, false, 0), desc(0x1970, 0xFFF0, 14, true, 0),
    desc(0x0C40, 0xFFF0, 9, true, 0), desc(0x0C50, 0xFFF0, 8, true, 0),
    desc(0x2240, 0xFFF0, 16, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x2250, 0xFFF0, 8, true, 0), desc(0x25D0, 0xFFF0, 10, true, 0),
    desc(0x2150, 0xFFF0, 8, true, 0), desc(0x2170, 0xFFF0, 8, true, 0),
    desc(0x0A90, 0xFFF0, 14, true, 0),
    desc(0x0A80, 0xFFF0, 24, true, FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x0340, 0xFFF0, 11, true, 0), desc(0x0450, 0xFFF0, 10, true, 0),
    desc(0x0B20, 0xFFF0, 11, true, 0), desc(0x0A20, 0xFFF0, 10, true, 0),
    desc(0x0040, 0xFFF0, 8, true, 0), desc(0x0150, 0xFFF0, 13, true, 0),
    desc(0x0160, 0xFFF0, 13, true, 0), desc(0x8000, 0xFFF0, 8, true, 0),
    desc(0x8000, 0xFC00, 12, false, FSL_MC_CHECK_MODULE_ID),
    desc(0xA000, 0xFC00, 8, false, FSL_MC_CHECK_MODULE_ID),
    desc(0x9800, 0xFC00, 12, true, FSL_MC_CHECK_MODULE_ID | FSL_MC_CAP_NET_ADMIN_NEEDED),
    desc(0x9000, 0xFC00, 64, true, FSL_MC_CHECK_MODULE_ID | FSL_MC_CAP_NET_ADMIN_NEEDED),
];

const FSL_MC_MAX_MODULE_ID: u16 = 0x10;

unsafe fn fsl_mc_command_check(mc_dev: *mut FslMcDevice, mc_cmd: *mut FslMcCommand) -> i32 {
    let cmdid = mc_cmd_hdr_read_cmdid(mc_cmd);
    let mut desc_ptr: *mut FslMcCmdDesc = core::ptr::null_mut();
    for i in 0..FSL_MC_ACCEPTED_CMDS.len() {
        let candidate = &mut FSL_MC_ACCEPTED_CMDS[i];
        if (cmdid & candidate.cmdid_mask) == candidate.cmdid_value {
            desc_ptr = candidate;
            break;
        }
    }
    if desc_ptr.is_null() { dev_err(mc_dev, "MC command 0x%04x: cmdid not accepted\n", cmdid); return -13; }
    let size = (*desc_ptr).size as usize;
    let bytes = core::slice::from_raw_parts(mc_cmd as *const u8, core::mem::size_of::<FslMcCommand>());
    let mut sum = 0u8;
    for byte in &bytes[size..] { sum |= *byte; }
    if sum != 0 { dev_err(mc_dev, "MC command 0x%04x: garbage beyond max size of %d bytes!\n", cmdid, (*desc_ptr).size); return -13; }
    let token = mc_cmd_hdr_read_token(mc_cmd) != 0;
    if token != (*desc_ptr).token { dev_err(mc_dev, "MC command 0x%04x: token 0x%04x is invalid!\n", cmdid, mc_cmd_hdr_read_token(mc_cmd)); return -13; }
    if ((*desc_ptr).flags & FSL_MC_CHECK_MODULE_ID) != 0 {
        let module_id = (cmdid & 0x03f0) >> 4;
        if module_id == 0 || module_id > FSL_MC_MAX_MODULE_ID { dev_err(mc_dev, "MC command 0x%04x: unknown module ID 0x%x\n", cmdid, module_id); return -13; }
    }
    if ((*desc_ptr).flags & FSL_MC_CAP_NET_ADMIN_NEEDED) != 0 && !capable(CAP_NET_ADMIN) { dev_err(mc_dev, "MC command 0x%04x: needs CAP_NET_ADMIN!\n", cmdid); return -1; }
    0
}

unsafe fn fsl_mc_uapi_send_command(mc_dev: *mut FslMcDevice, arg: usize, mc_io: *mut FslMcIo) -> i32 {
    let mut mc_cmd: FslMcCommand = core::mem::zeroed();
    if copy_from_user(&mut mc_cmd as *mut _, arg as *const _, core::mem::size_of::<FslMcCommand>()) != 0 { return -14; }
    let error = fsl_mc_command_check(mc_dev, &mut mc_cmd);
    if error != 0 { return error; }
    let error = mc_send_command(mc_io, &mut mc_cmd);
    if error != 0 { return error; }
    if copy_to_user(arg as *mut _, &mc_cmd as *const _, core::mem::size_of::<FslMcCommand>()) != 0 { return -14; }
    0
}

unsafe fn fsl_mc_uapi_dev_open(_inode: *mut Inode, filep: *mut File) -> i32 {
    let priv_data = kzalloc_obj::<UapiPrivData>();
    if priv_data.is_null() { return -12; }
    let mc_uapi = container_of((*filep).private_data, FslMcUapi::misc);
    let mc_bus = container_of(mc_uapi, FslMcBus::uapi_misc);
    let root_mc_device = &mut (*mc_bus).mc_dev;
    mutex_lock(&mut (*mc_uapi).mutex);
    if !(*mc_uapi).local_instance_in_use {
        (*priv_data).mc_io = (*mc_uapi).static_mc_io;
        (*mc_uapi).local_instance_in_use = true;
    } else {
        let mut dynamic_mc_io = core::ptr::null_mut();
        let error = fsl_mc_portal_allocate(root_mc_device, 0, &mut dynamic_mc_io);
        if error != 0 { dev_dbg(root_mc_device, "Could not allocate MC portal\n"); mutex_unlock(&mut (*mc_uapi).mutex); kfree(priv_data); return error; }
        (*priv_data).mc_io = dynamic_mc_io;
    }
    (*priv_data).uapi = mc_uapi;
    (*filep).private_data = priv_data as *mut _;
    mutex_unlock(&mut (*mc_uapi).mutex);
    0
}

unsafe fn fsl_mc_uapi_dev_release(_inode: *mut Inode, filep: *mut File) -> i32 {
    let priv_data = (*filep).private_data as *mut UapiPrivData;
    let mc_uapi = (*priv_data).uapi;
    let mc_io = (*priv_data).mc_io;
    mutex_lock(&mut (*mc_uapi).mutex);
    if mc_io == (*mc_uapi).static_mc_io { (*mc_uapi).local_instance_in_use = false; } else { fsl_mc_portal_free(mc_io); }
    kfree(priv_data as *mut _); (*filep).private_data = core::ptr::null_mut();
    mutex_unlock(&mut (*mc_uapi).mutex);
    0
}

unsafe fn fsl_mc_uapi_dev_ioctl(file: *mut File, cmd: u32, arg: usize) -> isize {
    let priv_data = (*file).private_data as *mut UapiPrivData;
    let mc_bus = container_of((*priv_data).uapi, FslMcBus::uapi_misc);
    let root_mc_device = &mut (*mc_bus).mc_dev;
    if cmd == FSL_MC_SEND_MC_COMMAND { fsl_mc_uapi_send_command(root_mc_device, arg, (*priv_data).mc_io) as isize } else { dev_dbg(root_mc_device, "unexpected ioctl call number\n"); -22 }
}

static FSL_MC_UAPI_DEV_FOPS: FileOperations = FileOperations { owner: THIS_MODULE, open: Some(fsl_mc_uapi_dev_open), release: Some(fsl_mc_uapi_dev_release), unlocked_ioctl: Some(fsl_mc_uapi_dev_ioctl) };

unsafe fn fsl_mc_uapi_create_device_file(mc_bus: *mut FslMcBus) -> i32 {
    let mc_dev = &mut (*mc_bus).mc_dev;
    let mc_uapi = &mut (*mc_bus).uapi_misc;
    (*mc_uapi).misc.minor = MISC_DYNAMIC_MINOR;
    (*mc_uapi).misc.name = dev_name(&mut mc_dev.dev);
    (*mc_uapi).misc.fops = &FSL_MC_UAPI_DEV_FOPS;
    let error = misc_register(&mut (*mc_uapi).misc);
    if error != 0 { return error; }
    (*mc_uapi).static_mc_io = mc_dev.mc_io;
    mutex_init(&mut (*mc_uapi).mutex);
    0
}

unsafe fn fsl_mc_uapi_remove_device_file(mc_bus: *mut FslMcBus) {
    misc_deregister(&mut (*mc_bus).uapi_misc.misc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
