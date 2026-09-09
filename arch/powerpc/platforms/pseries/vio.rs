// SPDX-License-Identifier: GPL-2.0-or-later
/* IBM PowerPC Virtual I/O Infrastructure Support. */

// Kernel declarations supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

#[repr(C)]
pub struct VioCmoPool { pub size: usize, pub free: usize }
#[repr(C)]
pub struct VioCmoDevEntry { pub viodev: *mut vio_dev, pub list: list_head }
#[repr(C)]
pub struct VioCmo {
    pub lock: spinlock_t, pub balance_q: delayed_work, pub device_list: list_head,
    pub entitled: usize, pub reserve: VioCmoPool, pub excess: VioCmoPool,
    pub spare: usize, pub min: usize, pub desired: usize, pub curr: usize, pub high: usize,
}

static mut vio_bus_device: device = device { name: b"vio\0".as_ptr() as *mut c_char, ..device::ZERO };
#[cfg(feature = "CONFIG_PPC_SMLPAR")]
static mut vio_cmo: VioCmo = VioCmo::zero();

#[cfg(feature = "CONFIG_PPC_SMLPAR")]
unsafe fn vio_cmo_num_OF_devs() -> c_int {
    let root = of_find_node_by_name(core::ptr::null_mut(), b"vdevice\0".as_ptr() as *const c_char);
    let mut count = 0;
    if !root.is_null() {
        let mut node = of_get_next_child(root, core::ptr::null_mut());
        while !node.is_null() {
            if !of_find_property(node, b"ibm,my-dma-window\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null() { count += 1; }
            node = of_get_next_child(root, node);
        }
    }
    of_node_put(root); count
}

#[cfg(feature = "CONFIG_PPC_SMLPAR")]
unsafe fn vio_cmo_alloc(v: *mut vio_dev, mut size: usize) -> c_int {
    let mut flags = 0; let mut reserve_free = 0; let mut ret = -ENOMEM;
    spin_lock_irqsave(&mut vio_cmo.lock, &mut flags);
    if (*v).cmo.entitled > (*v).cmo.allocated { reserve_free = (*v).cmo.entitled - (*v).cmo.allocated; }
    let excess_free = if vio_cmo.spare >= VIO_CMO_MIN_ENT { vio_cmo.excess.free } else { 0 };
    if reserve_free + excess_free >= size {
        vio_cmo.curr += size; if vio_cmo.curr > vio_cmo.high { vio_cmo.high = vio_cmo.curr; }
        (*v).cmo.allocated += size; size -= core::cmp::min(reserve_free, size); vio_cmo.excess.free -= size; ret = 0;
    }
    spin_unlock_irqrestore(&mut vio_cmo.lock, flags); ret
}

#[cfg(feature = "CONFIG_PPC_SMLPAR")]
unsafe fn vio_cmo_dealloc(v: *mut vio_dev, size: usize) {
    let mut flags=0; let mut spare_needed; let mut excess_freed=0; let mut reserve_freed=size; let mut balance=0;
    spin_lock_irqsave(&mut vio_cmo.lock, &mut flags); vio_cmo.curr -= size;
    if (*v).cmo.allocated > (*v).cmo.entitled { excess_freed=core::cmp::min(reserve_freed,(*v).cmo.allocated-(*v).cmo.entitled); reserve_freed-=excess_freed; }
    (*v).cmo.allocated -= reserve_freed + excess_freed; spare_needed=VIO_CMO_MIN_ENT-vio_cmo.spare;
    if spare_needed != 0 && excess_freed != 0 { let t=core::cmp::min(excess_freed,spare_needed); vio_cmo.excess.size-=t; vio_cmo.reserve.size+=t; vio_cmo.spare+=t; excess_freed-=t; spare_needed-=t; balance=1; }
    if spare_needed != 0 && reserve_freed != 0 { let t=core::cmp::min(spare_needed,core::cmp::min(reserve_freed,(*v).cmo.entitled-VIO_CMO_MIN_ENT)); vio_cmo.spare+=t; (*v).cmo.entitled-=t; reserve_freed-=t; spare_needed-=t; balance=1; }
    if excess_freed != 0 && vio_cmo.desired > vio_cmo.reserve.size { let t=core::cmp::min(excess_freed,vio_cmo.desired-vio_cmo.reserve.size); vio_cmo.excess.size-=t; vio_cmo.reserve.size+=t; excess_freed-=t; balance=1; }
    vio_cmo.excess.free+=excess_freed; if balance != 0 { schedule_delayed_work(&mut vio_cmo.balance_q,VIO_CMO_BALANCE_DELAY); }
    spin_unlock_irqrestore(&mut vio_cmo.lock,flags);
}

#[cfg(not(feature = "CONFIG_PPC_SMLPAR"))]
pub unsafe fn vio_cmo_entitlement_update(_: usize) -> c_int { 0 }
#[cfg(not(feature = "CONFIG_PPC_SMLPAR"))]
pub unsafe fn vio_cmo_set_dev_desired(_: *mut vio_dev, _: usize) {}

pub unsafe fn vio_h_cop_sync(vdev: *mut vio_dev, op: *mut vio_pfo_op) -> c_int {
    let mut deadline=0; let mut hret=0; if (*op).timeout != 0 { deadline=jiffies()+msecs_to_jiffies((*op).timeout); }
    loop { hret=plpar_hcall_norets(H_COP,(*op).flags,(*vdev).resource_id,(*op).in_,(*op).inlen,(*op).out,(*op).outlen,(*op).csbcpb); if hret==H_SUCCESS || (hret!=H_NOT_ENOUGH_RESOURCES && hret!=H_BUSY && hret!=H_RESOURCE) || ((*op).timeout!=0 && time_after(deadline,jiffies())) { break; } }
    (*op).hcall_err=hret; match hret { H_SUCCESS=>0, H_OP_MODE|H_TOO_BIG=>-E2BIG, H_RESCINDED=>-EACCES, H_HARDWARE=>-EPERM, H_NOT_ENOUGH_RESOURCES|H_RESOURCE|H_BUSY=>-EBUSY, _=>-EINVAL }
}

pub unsafe fn vio_unregister_device(v: *mut vio_dev) { device_unregister(&mut (*v).dev); if (*v).family==VDEVICE { irq_dispose_mapping((*v).irq); } }
pub unsafe fn vio_get_attribute(v: *mut vio_dev, which: *mut c_char, length: *mut c_int) -> *const c_void { of_get_property((*v).dev.of_node,which,length) }
pub unsafe fn vio_enable_interrupts(v: *mut vio_dev) -> c_int { h_vio_signal((*v).unit_address,VIO_IRQ_ENABLE) }
pub unsafe fn vio_disable_interrupts(v: *mut vio_dev) -> c_int { h_vio_signal((*v).unit_address,VIO_IRQ_DISABLE) }

// The remaining bus registration, CMO sysfs, DMA mapping, device-tree registration,
// matching, and initialization routines retain their kernel ABI and are declared here
// for linkage with the translated kernel support files.
extern "C" {
    fn vio_bus_init() -> c_int;
    fn vio_device_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
