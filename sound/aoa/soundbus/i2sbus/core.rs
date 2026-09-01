// SPDX-License-Identifier: GPL-2.0-only
/*
 * i2sbus driver
 *
 * Copyright 2006-2008 Johannes Berg <johannes@sipsolutions.net>
 */

// Translated from C implementation source. Kernel headers and module metadata
// from the original include list are external dependencies for the final tree.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type U32 = u32;
type DmaAddrT = usize;
type IrqReturnT = c_uint;
type IrqHandlerT = Option<unsafe extern "C" fn(c_int, *mut c_void) -> IrqReturnT>;
type PmMessageT = c_uint;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const ENXIO: c_int = 6;
const ENODEV: c_int = 19;
const IRQ_HANDLED: IrqReturnT = 1;
const KERN_ERR: *const c_char = b"\0".as_ptr() as *const c_char;
const KERN_DEBUG: *const c_char = b"\0".as_ptr() as *const c_char;
const MAX_DBDMA_COMMANDS: c_int = 0;

const AOA_RESOURCE_I2SMMIO: c_int = 0;
const AOA_RESOURCE_TXDBDMA: c_int = 1;
const AOA_RESOURCE_RXDBDMA: c_int = 2;

#[repr(C)]
pub struct Device {
    pub of_node: *mut DeviceNode,
    pub dma_mask: *mut u64,
    pub parent: *mut Device,
    pub release: Option<unsafe extern "C" fn(*mut Device)>,
    pub kobj: Kobject,
}

#[repr(C)]
pub struct Kobject {
    pub state_initialized: bool,
}

#[repr(C)]
pub struct OfDevice {
    pub dev: Device,
    pub archdata: ArchData,
}

#[repr(C)]
pub struct ArchData {
    pub dma_mask: u64,
}

#[repr(C)]
pub struct MacioDev {
    pub ofdev: OfDevice,
}

#[repr(C)]
pub struct PciDev {
    pub dev: Device,
}

#[repr(C)]
pub struct DeviceNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct OfDeviceId {
    pub name: *const c_char,
}

#[repr(C)]
pub struct Resource {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct DbdmaCmd {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DbdmaRegs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DbdmaCommandMem {
    pub size: usize,
    pub space: *mut c_void,
    pub bus_addr: DmaAddrT,
    pub cmds: *mut DbdmaCmd,
    pub bus_cmd_start: DmaAddrT,
}

#[repr(C)]
pub struct I2sInterfaceRegs {
    pub intr_ctl: U32,
}

#[repr(C)]
pub struct Mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Spinlock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ListHead {
    _private: [u8; 0],
}

#[repr(C)]
pub struct SoundbusDev {
    pub ofdev: OfDevice,
    pub modalias: [c_char; 32],
    pub attach_codec: Option<unsafe extern "C" fn()>,
    pub detach_codec: Option<unsafe extern "C" fn()>,
    pub pcmid: c_int,
    pub codec_list: ListHead,
}

#[repr(C)]
pub struct I2sStream {
    pub dbdma: *mut DbdmaRegs,
    pub dbdma_ring: DbdmaCommandMem,
}

#[repr(C)]
pub struct I2sbusDev {
    pub sound: SoundbusDev,
    pub intfregs: *mut I2sInterfaceRegs,
    pub out: I2sStream,
    pub in_: I2sStream,
    pub allocated_resource: [*mut Resource; 3],
    pub interrupts: [c_int; 3],
    pub lock: Mutex,
    pub low_lock: Spinlock,
    pub macio: *mut MacioDev,
    pub control: *mut I2sbusControl,
    pub bus_number: c_int,
    pub rnames: [[c_char; 64]; 3],
    pub resources: [Resource; 3],
    pub item: ListHead,
}

#[repr(C)]
pub struct I2sbusControl {
    pub list: ListHead,
}

#[repr(C)]
pub struct Codec {
    pub suspend: Option<unsafe extern "C" fn(*mut CodecInfoItem, PmMessageT) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut CodecInfoItem) -> c_int>,
}

#[repr(C)]
pub struct CodecInfoItem {
    pub list: ListHead,
    pub codec: *mut Codec,
}

#[repr(C)]
pub struct MacioDriverInner {
    pub name: *const c_char,
    pub owner: *mut c_void,
    pub of_match_table: *const OfDeviceId,
}

#[repr(C)]
pub struct MacioDriver {
    pub driver: MacioDriverInner,
    pub probe: Option<unsafe extern "C" fn(*mut MacioDev, *const OfDeviceId) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut MacioDev)>,
    // CONFIG_PM: suspend and resume callbacks are present when power management is enabled.
    pub suspend: Option<unsafe extern "C" fn(*mut MacioDev, PmMessageT) -> c_int>,
    pub resume: Option<unsafe extern "C" fn(*mut MacioDev) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut MacioDev) -> c_int>,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut force: c_int;

    fn macio_get_pci_dev(dev: *mut MacioDev) -> *mut PciDev;
    fn dma_alloc_coherent(dev: *mut Device, size: usize, bus_addr: *mut DmaAddrT, flags: c_uint) -> *mut c_void;
    fn dma_free_coherent(dev: *mut Device, size: usize, cpu_addr: *mut c_void, bus_addr: DmaAddrT);
    fn DBDMA_ALIGN(ptr: *mut c_void) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn release_and_free_resource(res: *mut Resource);
    fn free_irq(irq: c_int, dev_id: *mut c_void);
    fn i2sbus_control_remove_dev(control: *mut I2sbusControl, dev: *mut I2sbusDev);
    fn of_node_put(node: *mut DeviceNode);
    fn mutex_destroy(lock: *mut Mutex);
    fn kfree(ptr: *mut c_void);
    fn in_le32(addr: *const U32) -> U32;
    fn out_le32(addr: *mut U32, val: U32);
    fn of_address_to_resource(np: *mut DeviceNode, index: c_int, res: *mut Resource) -> c_int;
    fn of_get_parent(np: *mut DeviceNode) -> *mut DeviceNode;
    fn of_get_property(np: *mut DeviceNode, name: *const c_char, lenp: *mut c_void) -> *const U32;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn of_node_name_eq(np: *mut DeviceNode, name: *const c_char) -> bool;
    fn of_node_get(np: *mut DeviceNode) -> *mut DeviceNode;
    fn mutex_init(lock: *mut Mutex);
    fn spin_lock_init(lock: *mut Spinlock);
    fn INIT_LIST_HEAD(list: *mut ListHead);
    fn irq_of_parse_and_map(np: *mut DeviceNode, index: c_int) -> c_int;
    fn request_irq(irq: c_int, handler: IrqHandlerT, flags: c_uint, name: *const c_char, dev: *mut c_void) -> c_int;
    fn request_mem_region(start: usize, len: usize, name: *const c_char) -> *mut Resource;
    fn resource_size(res: *const Resource) -> usize;
    fn printk(fmt: *const c_char, ...);
    fn ioremap(offset: usize, size: usize) -> *mut c_void;
    fn i2sbus_control_add_dev(control: *mut I2sbusControl, dev: *mut I2sbusDev) -> c_int;
    fn soundbus_add_one(sound: *mut SoundbusDev) -> c_int;
    fn soundbus_dev_put(sound: *mut SoundbusDev);
    fn i2sbus_control_cell(control: *mut I2sbusControl, dev: *mut I2sbusDev, enable: c_int);
    fn i2sbus_control_enable(control: *mut I2sbusControl, dev: *mut I2sbusDev);
    fn i2sbus_control_clock(control: *mut I2sbusControl, dev: *mut I2sbusDev, enable: c_int);
    fn i2sbus_control_init(dev: *mut MacioDev, control: *mut *mut I2sbusControl) -> c_int;
    fn of_device_is_compatible(np: *mut DeviceNode, compat: *const c_char) -> bool;
    fn i2sbus_control_destroy(control: *mut I2sbusControl);
    fn dev_set_drvdata(dev: *mut Device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut Device) -> *mut c_void;
    fn soundbus_remove_one(sound: *mut SoundbusDev);
    fn list_empty(head: *const ListHead) -> bool;
    fn i2sbus_wait_for_stop_both(dev: *mut I2sbusDev);
    fn i2sbus_pcm_prepare_both(dev: *mut I2sbusDev);
    fn macio_register_driver(driver: *mut MacioDriver) -> c_int;
    fn macio_unregister_driver(driver: *mut MacioDriver);
    fn i2sbus_tx_intr(irq: c_int, devid: *mut c_void) -> IrqReturnT;
    fn i2sbus_rx_intr(irq: c_int, devid: *mut c_void) -> IrqReturnT;
    fn i2sbus_attach_codec();
    fn i2sbus_detach_codec();
    fn for_each_child_of_node_next(parent: *mut DeviceNode, child: *mut *mut DeviceNode) -> bool;
    fn list_for_each_i2sbus_dev_safe(
        pos: *mut *mut I2sbusDev,
        tmp: *mut *mut I2sbusDev,
        head: *mut ListHead,
    ) -> bool;
    fn list_for_each_i2sbus_dev(pos: *mut *mut I2sbusDev, head: *mut ListHead) -> bool;
    fn list_for_each_codec_info_item(pos: *mut *mut CodecInfoItem, head: *mut ListHead) -> bool;
}

static I2SBUS_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { name: b"i2s\0".as_ptr() as *const c_char },
    OfDeviceId { name: ptr::null() },
];

unsafe fn alloc_dbdma_descriptor_ring(
    i2sdev: *mut I2sbusDev,
    r: *mut DbdmaCommandMem,
    numcmds: c_int,
) -> c_int {
    /* one more for rounding, one for branch back, one for stop command */
    (*r).size = ((numcmds + 3) as usize).wrapping_mul(size_of::<DbdmaCmd>());
    /* We use the PCI APIs for now until the generic one gets fixed
     * enough or until we get some macio-specific versions
     */
    (*r).space = dma_alloc_coherent(
        &mut (*macio_get_pci_dev((*i2sdev).macio)).dev,
        (*r).size,
        &mut (*r).bus_addr,
        GFP_KERNEL,
    );
    if (*r).space.is_null() {
        return -ENOMEM;
    }

    (*r).cmds = DBDMA_ALIGN((*r).space) as *mut DbdmaCmd;
    (*r).bus_cmd_start =
        (*r).bus_addr.wrapping_add(((*r).cmds as *mut c_char).offset_from((*r).space as *mut c_char) as DmaAddrT);

    0
}

unsafe fn free_dbdma_descriptor_ring(i2sdev: *mut I2sbusDev, r: *mut DbdmaCommandMem) {
    if (*r).space.is_null() {
        return;
    }

    dma_free_coherent(
        &mut (*macio_get_pci_dev((*i2sdev).macio)).dev,
        (*r).size,
        (*r).space,
        (*r).bus_addr,
    );
}

unsafe extern "C" fn i2sbus_release_dev(dev: *mut Device) {
    let i2sdev = dev as *mut I2sbusDev;
    let mut i: c_int;

    iounmap((*i2sdev).intfregs as *mut c_void);
    iounmap((*i2sdev).out.dbdma as *mut c_void);
    iounmap((*i2sdev).in_.dbdma as *mut c_void);
    i = AOA_RESOURCE_I2SMMIO;
    while i <= AOA_RESOURCE_RXDBDMA {
        release_and_free_resource((*i2sdev).allocated_resource[i as usize]);
        i += 1;
    }
    free_dbdma_descriptor_ring(i2sdev, &mut (*i2sdev).out.dbdma_ring);
    free_dbdma_descriptor_ring(i2sdev, &mut (*i2sdev).in_.dbdma_ring);
    i = AOA_RESOURCE_I2SMMIO;
    while i <= AOA_RESOURCE_RXDBDMA {
        free_irq((*i2sdev).interrupts[i as usize], i2sdev as *mut c_void);
        i += 1;
    }
    i2sbus_control_remove_dev((*i2sdev).control, i2sdev);
    of_node_put((*i2sdev).sound.ofdev.dev.of_node);
    mutex_destroy(&mut (*i2sdev).lock);
    kfree(i2sdev as *mut c_void);
}

unsafe extern "C" fn i2sbus_bus_intr(_irq: c_int, devid: *mut c_void) -> IrqReturnT {
    let dev = devid as *mut I2sbusDev;
    let intreg: U32;

    // guard(spinlock)(&dev->low_lock);
    intreg = in_le32(&(*(*dev).intfregs).intr_ctl);

    /* acknowledge interrupt reasons */
    out_le32(&mut (*(*dev).intfregs).intr_ctl, intreg);

    IRQ_HANDLED
}

/*
 * XXX FIXME: We test the layout_id's here to get the proper way of
 * mapping in various registers, thanks to bugs in Apple device-trees.
 * We could instead key off the machine model and the name of the i2s
 * node (i2s-a). This we'll do when we move it all to macio_asic.c
 * and have that export items for each sub-node too.
 */
unsafe fn i2sbus_get_and_fixup_rsrc(
    np: *mut DeviceNode,
    index: c_int,
    layout: c_int,
    res: *mut Resource,
) -> c_int {
    let parent: *mut DeviceNode;
    let pindex: c_int;
    let mut rc: c_int = -ENXIO;
    let reg: *const U32;

    /* Machines with layout 76 and 36 (K2 based) have a weird device
     * tree what we need to special case.
     * Normal machines just fetch the resource from the i2s-X node.
     * Darwin further divides normal machines into old and new layouts
     * with a subtely different code path but that doesn't seem necessary
     * in practice, they just bloated it. In addition, even on our K2
     * case the i2s-modem node, if we ever want to handle it, uses the
     * normal layout
     */
    if layout != 76 && layout != 36 {
        return of_address_to_resource(np, index, res);
    }

    parent = of_get_parent(np);
    pindex = if index == AOA_RESOURCE_I2SMMIO { 0 } else { 1 };
    rc = of_address_to_resource(parent, pindex, res);
    if rc != 0 {
        goto_bail(parent, rc)
    } else {
        reg = of_get_property(np, b"reg\0".as_ptr() as *const c_char, ptr::null_mut());
        if reg.is_null() {
            rc = -ENXIO;
            goto_bail(parent, rc)
        } else {
            (*res).start = (*res).start.wrapping_add(*reg.add((index * 2) as usize) as usize);
            (*res).end = (*res)
                .start
                .wrapping_add(*reg.add((index * 2 + 1) as usize) as usize)
                .wrapping_sub(1);
            goto_bail(parent, rc)
        }
    }
}

unsafe fn goto_bail(parent: *mut DeviceNode, rc: c_int) -> c_int {
    of_node_put(parent);
    rc
}

/* Returns 1 if added, 0 for otherwise; don't return a negative value! */
unsafe fn i2sbus_add_dev(
    macio: *mut MacioDev,
    control: *mut I2sbusControl,
    np: *mut DeviceNode,
) -> c_int {
    let dev: *mut I2sbusDev;
    let mut child: *mut DeviceNode = ptr::null_mut();
    let mut sound: *mut DeviceNode = ptr::null_mut();
    let mut r: *mut Resource;
    let mut i: c_int;
    let mut layout: c_int = 0;
    let mut rlen: usize;
    let mut ok: c_int = force;
    let mut node_name = [0 as c_char; 8];
    static RNAMES: [*const c_char; 3] = [
        b"i2sbus: %pOFn (control)\0".as_ptr() as *const c_char,
        b"i2sbus: %pOFn (tx)\0".as_ptr() as *const c_char,
        b"i2sbus: %pOFn (rx)\0".as_ptr() as *const c_char,
    ];
    static INTS: [IrqHandlerT; 3] = [
        Some(i2sbus_bus_intr),
        Some(i2sbus_tx_intr),
        Some(i2sbus_rx_intr),
    ];

    if snprintf(
        node_name.as_mut_ptr(),
        node_name.len(),
        b"%pOFn\0".as_ptr() as *const c_char,
        np,
    ) != 5
    {
        return 0;
    }
    if strncmp(
        node_name.as_ptr(),
        b"i2s-\0".as_ptr() as *const c_char,
        4,
    ) != 0
    {
        return 0;
    }

    dev = kzalloc(size_of::<I2sbusDev>(), GFP_KERNEL) as *mut I2sbusDev;
    if dev.is_null() {
        return 0;
    }

    i = 0;
    // for_each_child_of_node(np, child)
    while for_each_child_of_node_next(np, &mut child) {
        if of_node_name_eq(child, b"sound\0".as_ptr() as *const c_char) {
            of_node_put(sound);
            i += 1;
            sound = of_node_get(child);
        }
    }
    if i == 1 {
        let mut id = of_get_property(sound, b"layout-id\0".as_ptr() as *const c_char, ptr::null_mut());

        if !id.is_null() {
            layout = *id as c_int;
            snprintf(
                (*dev).sound.modalias.as_mut_ptr(),
                32,
                b"sound-layout-%d\0".as_ptr() as *const c_char,
                layout,
            );
            ok = 1;
        } else {
            id = of_get_property(sound, b"device-id\0".as_ptr() as *const c_char, ptr::null_mut());
            /*
             * We probably cannot handle all device-id machines,
             * so restrict to those we do handle for now.
             */
            if !id.is_null()
                && (*id == 22 || *id == 14 || *id == 35 || *id == 31 || *id == 44)
            {
                snprintf(
                    (*dev).sound.modalias.as_mut_ptr(),
                    32,
                    b"aoa-device-id-%d\0".as_ptr() as *const c_char,
                    *id,
                );
                ok = 1;
                layout = -1;
            }
        }
    }
    of_node_put(sound);
    /* for the time being, until we can handle non-layout-id
     * things in some fabric, refuse to attach if there is no
     * layout-id property or we haven't been forced to attach.
     * When there are two i2s busses and only one has a layout-id,
     * then this depends on the order, but that isn't important
     * either as the second one in that case is just a modem. */
    if ok == 0 {
        kfree(dev as *mut c_void);
        return 0;
    }

    mutex_init(&mut (*dev).lock);
    spin_lock_init(&mut (*dev).low_lock);
    (*dev).sound.ofdev.archdata.dma_mask = (*macio).ofdev.archdata.dma_mask;
    (*dev).sound.ofdev.dev.of_node = of_node_get(np);
    (*dev).sound.ofdev.dev.dma_mask = &mut (*dev).sound.ofdev.archdata.dma_mask;
    (*dev).sound.ofdev.dev.parent = &mut (*macio).ofdev.dev;
    (*dev).sound.ofdev.dev.release = Some(i2sbus_release_dev);
    (*dev).sound.attach_codec = Some(i2sbus_attach_codec);
    (*dev).sound.detach_codec = Some(i2sbus_detach_codec);
    (*dev).sound.pcmid = -1;
    (*dev).macio = macio;
    (*dev).control = control;
    (*dev).bus_number = node_name[4] as c_int - b'a' as c_int;
    INIT_LIST_HEAD(&mut (*dev).sound.codec_list);

    i = AOA_RESOURCE_I2SMMIO;
    while i <= AOA_RESOURCE_RXDBDMA {
        (*dev).interrupts[i as usize] = -1;
        snprintf(
            (*dev).rnames[i as usize].as_mut_ptr(),
            (*dev).rnames[i as usize].len(),
            RNAMES[i as usize],
            np,
        );
        i += 1;
    }

    i = AOA_RESOURCE_I2SMMIO;
    while i <= AOA_RESOURCE_RXDBDMA {
        let irq = irq_of_parse_and_map(np, i);
        if irq == 0 {
            return i2sbus_add_dev_err(dev);
        }
        if request_irq(
            irq,
            INTS[i as usize],
            0,
            (*dev).rnames[i as usize].as_ptr(),
            dev as *mut c_void,
        ) != 0
        {
            return i2sbus_add_dev_err(dev);
        }
        (*dev).interrupts[i as usize] = irq;
        i += 1;
    }

    /* Resource handling is problematic as some device-trees contain
     * useless crap (ugh ugh ugh). We work around that here by calling
     * specific functions for calculating the appropriate resources.
     *
     * This will all be moved to macio_asic.c at one point
     */
    i = AOA_RESOURCE_I2SMMIO;
    while i <= AOA_RESOURCE_RXDBDMA {
        if i2sbus_get_and_fixup_rsrc(np, i, layout, &mut (*dev).resources[i as usize]) != 0 {
            return i2sbus_add_dev_err(dev);
        }
        /* If only we could use our resource dev->resources[i]...
         * but request_resource doesn't know about parents and
         * contained resources...
         */
        (*dev).allocated_resource[i as usize] = request_mem_region(
            (*dev).resources[i as usize].start,
            resource_size(&(*dev).resources[i as usize]),
            (*dev).rnames[i as usize].as_ptr(),
        );
        if (*dev).allocated_resource[i as usize].is_null() {
            printk(
                b"i2sbus: failed to claim resource %d!\n\0".as_ptr() as *const c_char,
                i,
            );
            return i2sbus_add_dev_err(dev);
        }
        i += 1;
    }

    r = &mut (*dev).resources[AOA_RESOURCE_I2SMMIO as usize];
    rlen = resource_size(r);
    if rlen < size_of::<I2sInterfaceRegs>() {
        return i2sbus_add_dev_err(dev);
    }
    (*dev).intfregs = ioremap((*r).start, rlen) as *mut I2sInterfaceRegs;

    r = &mut (*dev).resources[AOA_RESOURCE_TXDBDMA as usize];
    rlen = resource_size(r);
    if rlen < size_of::<DbdmaRegs>() {
        return i2sbus_add_dev_err(dev);
    }
    (*dev).out.dbdma = ioremap((*r).start, rlen) as *mut DbdmaRegs;

    r = &mut (*dev).resources[AOA_RESOURCE_RXDBDMA as usize];
    rlen = resource_size(r);
    if rlen < size_of::<DbdmaRegs>() {
        return i2sbus_add_dev_err(dev);
    }
    (*dev).in_.dbdma = ioremap((*r).start, rlen) as *mut DbdmaRegs;

    if (*dev).intfregs.is_null() || (*dev).out.dbdma.is_null() || (*dev).in_.dbdma.is_null() {
        return i2sbus_add_dev_err(dev);
    }

    if alloc_dbdma_descriptor_ring(dev, &mut (*dev).out.dbdma_ring, MAX_DBDMA_COMMANDS) != 0 {
        return i2sbus_add_dev_err(dev);
    }
    if alloc_dbdma_descriptor_ring(dev, &mut (*dev).in_.dbdma_ring, MAX_DBDMA_COMMANDS) != 0 {
        return i2sbus_add_dev_err(dev);
    }

    if i2sbus_control_add_dev((*dev).control, dev) != 0 {
        printk(b"i2sbus: control layer didn't like bus\n\0".as_ptr() as *const c_char);
        return i2sbus_add_dev_err(dev);
    }

    if soundbus_add_one(&mut (*dev).sound) != 0 {
        printk(b"i2sbus: device registration error!\n\0".as_ptr() as *const c_char);
        if (*dev).sound.ofdev.dev.kobj.state_initialized {
            soundbus_dev_put(&mut (*dev).sound);
            return 0;
        }
        return i2sbus_add_dev_err(dev);
    }

    /* enable this cell */
    i2sbus_control_cell((*dev).control, dev, 1);
    i2sbus_control_enable((*dev).control, dev);
    i2sbus_control_clock((*dev).control, dev, 1);

    1
}

unsafe fn i2sbus_add_dev_err(dev: *mut I2sbusDev) -> c_int {
    let mut i: c_int = 0;
    while i < 3 {
        if (*dev).interrupts[i as usize] != -1 {
            free_irq((*dev).interrupts[i as usize], dev as *mut c_void);
        }
        i += 1;
    }
    free_dbdma_descriptor_ring(dev, &mut (*dev).out.dbdma_ring);
    free_dbdma_descriptor_ring(dev, &mut (*dev).in_.dbdma_ring);
    iounmap((*dev).intfregs as *mut c_void);
    iounmap((*dev).out.dbdma as *mut c_void);
    iounmap((*dev).in_.dbdma as *mut c_void);
    i = 0;
    while i < 3 {
        release_and_free_resource((*dev).allocated_resource[i as usize]);
        i += 1;
    }
    mutex_destroy(&mut (*dev).lock);
    of_node_put((*dev).sound.ofdev.dev.of_node);
    kfree(dev as *mut c_void);
    0
}

unsafe extern "C" fn i2sbus_probe(dev: *mut MacioDev, _match: *const OfDeviceId) -> c_int {
    let mut np: *mut DeviceNode = ptr::null_mut();
    let mut got: c_int = 0;
    let err: c_int;
    let mut control: *mut I2sbusControl = ptr::null_mut();

    err = i2sbus_control_init(dev, &mut control);
    if err != 0 {
        return err;
    }
    if control.is_null() {
        printk(b"i2sbus_control_init API breakage\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    // for_each_child_of_node(dev->ofdev.dev.of_node, np)
    while for_each_child_of_node_next((*dev).ofdev.dev.of_node, &mut np) {
        if of_device_is_compatible(np, b"i2sbus\0".as_ptr() as *const c_char)
            || of_device_is_compatible(np, b"i2s-modem\0".as_ptr() as *const c_char)
        {
            got += i2sbus_add_dev(dev, control, np);
        }
    }

    if got == 0 {
        /* found none, clean up */
        i2sbus_control_destroy(control);
        return -ENODEV;
    }

    dev_set_drvdata(&mut (*dev).ofdev.dev, control as *mut c_void);

    0
}

unsafe extern "C" fn i2sbus_remove(dev: *mut MacioDev) {
    let control = dev_get_drvdata(&mut (*dev).ofdev.dev) as *mut I2sbusControl;
    let mut i2sdev: *mut I2sbusDev = ptr::null_mut();
    let mut tmp: *mut I2sbusDev = ptr::null_mut();

    // list_for_each_entry_safe(i2sdev, tmp, &control->list, item)
    while list_for_each_i2sbus_dev_safe(&mut i2sdev, &mut tmp, &mut (*control).list) {
        soundbus_remove_one(&mut (*i2sdev).sound);
    }
}

// CONFIG_PM
unsafe extern "C" fn i2sbus_suspend(dev: *mut MacioDev, state: PmMessageT) -> c_int {
    let control = dev_get_drvdata(&mut (*dev).ofdev.dev) as *mut I2sbusControl;
    let mut cii: *mut CodecInfoItem = ptr::null_mut();
    let mut i2sdev: *mut I2sbusDev = ptr::null_mut();
    let mut err: c_int;
    let mut ret: c_int = 0;

    // list_for_each_entry(i2sdev, &control->list, item)
    while list_for_each_i2sbus_dev(&mut i2sdev, &mut (*control).list) {
        /* Notify codecs */
        while list_for_each_codec_info_item(&mut cii, &mut (*i2sdev).sound.codec_list) {
            err = 0;
            if (*(*cii).codec).suspend.is_some() {
                err = ((*(*cii).codec).suspend.unwrap())(cii, state);
            }
            if err != 0 {
                ret = err;
            }
        }

        /* wait until streams are stopped */
        i2sbus_wait_for_stop_both(i2sdev);
    }

    ret
}

unsafe extern "C" fn i2sbus_resume(dev: *mut MacioDev) -> c_int {
    let control = dev_get_drvdata(&mut (*dev).ofdev.dev) as *mut I2sbusControl;
    let mut cii: *mut CodecInfoItem = ptr::null_mut();
    let mut i2sdev: *mut I2sbusDev = ptr::null_mut();
    let mut err: c_int;
    let mut ret: c_int = 0;

    // list_for_each_entry(i2sdev, &control->list, item)
    while list_for_each_i2sbus_dev(&mut i2sdev, &mut (*control).list) {
        if list_empty(&(*i2sdev).sound.codec_list) {
            continue;
        }

        /* reset i2s bus format etc. */
        i2sbus_pcm_prepare_both(i2sdev);

        /* Notify codecs so they can re-initialize */
        while list_for_each_codec_info_item(&mut cii, &mut (*i2sdev).sound.codec_list) {
            err = 0;
            if (*(*cii).codec).resume.is_some() {
                err = ((*(*cii).codec).resume.unwrap())(cii);
            }
            if err != 0 {
                ret = err;
            }
        }
    }

    ret
}
// #endif /* CONFIG_PM */

unsafe extern "C" fn i2sbus_shutdown(_dev: *mut MacioDev) -> c_int {
    0
}

static mut I2SBUS_DRV: MacioDriver = MacioDriver {
    driver: MacioDriverInner {
        name: b"soundbus-i2s\0".as_ptr() as *const c_char,
        owner: ptr::null_mut(),
        of_match_table: I2SBUS_MATCH.as_ptr(),
    },
    probe: Some(i2sbus_probe),
    remove: Some(i2sbus_remove),
    // CONFIG_PM
    suspend: Some(i2sbus_suspend),
    resume: Some(i2sbus_resume),
    shutdown: Some(i2sbus_shutdown),
};

unsafe fn soundbus_i2sbus_init() -> c_int {
    I2SBUS_DRV.driver.owner = THIS_MODULE;
    macio_register_driver(&mut I2SBUS_DRV)
}

unsafe fn soundbus_i2sbus_exit() {
    macio_unregister_driver(&mut I2SBUS_DRV);
}

// module_init(soundbus_i2sbus_init);
// module_exit(soundbus_i2sbus_exit);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
