// SPDX-License-Identifier: GPL-2.0-only
/*******************************************************************************
    AudioScience HPI driver
    Common Linux HPI ioctl and module probe/remove functions

    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


*******************************************************************************/

pub const SOURCEFILE_NAME: &[u8] = b"hpioctl.c\0";

// Dependencies originally supplied by:
// hpi_internal.h, hpi_version.h, hpimsginit.h, hpidebug.h, hpimsgx.h,
// hpioctl.h, hpicmn.h, and Linux kernel headers.
// MODULE_FIRMWARE("asihpi/dsp5000.bin");
// MODULE_FIRMWARE("asihpi/dsp6200.bin");
// MODULE_FIRMWARE("asihpi/dsp6205.bin");
// MODULE_FIRMWARE("asihpi/dsp6400.bin");
// MODULE_FIRMWARE("asihpi/dsp6600.bin");
// MODULE_FIRMWARE("asihpi/dsp8700.bin");
// MODULE_FIRMWARE("asihpi/dsp8900.bin");

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

type u16_t = u16;
type u32_t = u32;
type size_t = usize;
type irqreturn_t = c_int;

extern "C" {
    static mut hpi_debug_level: c_int;
}

static mut prealloc_stream_buf: c_int = 0;
// module_param(prealloc_stream_buf, int, 0444);
// MODULE_PARM_DESC(prealloc_stream_buf,
//      "Preallocate size for per-adapter stream buffer");

// Allow the debug level to be changed after module load.
// E.g.   echo 2 > /sys/module/asihpi/parameters/hpiDebugLevel
// module_param(hpi_debug_level, int, 0644);
// MODULE_PARM_DESC(hpi_debug_level, "debug verbosity 0..5");

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pci_dev {
    pub dev: device,
    pub vendor: c_uint,
    pub device: c_uint,
    pub subsystem_vendor: c_uint,
    pub subsystem_device: c_uint,
    pub devfn: c_uint,
    pub irq: c_uint,
    pub resource: [resource; HPI_MAX_ADAPTER_MEM_SPACES],
}

#[repr(C)]
pub struct pci_device_id {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_adapter_obj {
    pub index: c_int,
    pub type_: c_int,
    pub irq_query_and_clear:
        Option<unsafe extern "C" fn(*mut hpi_adapter_obj, c_int) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_adapter {
    pub adapter: *mut hpi_adapter_obj,
    pub mutex: mutex,
    pub p_buffer: *mut c_void,
    pub buffer_size: size_t,
    pub interrupt_callback: Option<unsafe extern "C" fn(*mut hpi_adapter)>,
    pub interrupt_mode: c_int,
    pub irq: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_pci_resource {
    pub pci: *mut hpi_pci,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_resource_union {
    pub pci: *mut hpi_pci,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_resource {
    pub bus_type: c_int,
    pub r: hpi_resource_union,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_subsys_msg {
    pub resource: hpi_resource,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_data_msg {
    pub pb_data: *mut c_void,
    pub data_size: u32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_d_u {
    pub data: hpi_data_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_d_msg {
    pub u: hpi_d_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_property_set {
    pub property: c_int,
    pub parameter1: c_int,
    pub parameter2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_ax_msg {
    pub property_set: hpi_property_set,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_message_u {
    pub s: hpi_subsys_msg,
    pub d: hpi_d_msg,
    pub ax: hpi_ax_msg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_message_header {
    pub size: u16_t,
    pub object: u16_t,
    pub function: u16_t,
    pub adapter_index: u16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_message {
    pub size: u16_t,
    pub object: u16_t,
    pub function: u16_t,
    pub adapter_index: u16_t,
    pub u: hpi_message_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_response_header {
    pub size: u16_t,
    pub error: u16_t,
    pub specific_error: u16_t,
    pub function: u16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_subsys_response {
    pub adapter_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_mode_response {
    pub adapter_mode: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_property_get {
    pub parameter1: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_ax_response {
    pub mode: hpi_mode_response,
    pub property_get: hpi_property_get,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_response_u {
    pub s: hpi_subsys_response,
    pub ax: hpi_ax_response,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_response {
    pub size: u16_t,
    pub error: u16_t,
    pub specific_error: u16_t,
    pub function: u16_t,
    pub u: hpi_response_u,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_message_buffer_v1 {
    pub h: hpi_message_header,
    pub m0: hpi_message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hpi_response_buffer_v1 {
    pub h: hpi_response_header,
    pub r0: hpi_response,
}

#[repr(C)]
pub struct hpi_ioctl_linux {
    pub phm: *mut c_void,
    pub phr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_pci {
    pub ap_mem_base: [*mut c_void; HPI_MAX_ADAPTER_MEM_SPACES],
    pub pci_dev: *mut pci_dev,
}

pub const HPI_MAX_ADAPTERS: usize = 32;
pub const HPI_MAX_ADAPTER_MEM_SPACES: usize = 8;
pub const HPI_IOCTL_LINUX: c_uint = 0;
pub const HPI_OBJ_SUBSYSTEM: u16_t = 1;
pub const HPI_OBJ_ADAPTER: u16_t = 2;
pub const HPI_SUBSYS_CLOSE: u16_t = 0;
pub const HPI_SUBSYS_CREATE_ADAPTER: u16_t = 0;
pub const HPI_SUBSYS_DRIVER_LOAD: u16_t = 0;
pub const HPI_SUBSYS_DRIVER_UNLOAD: u16_t = 0;
pub const HPI_ADAPTER_DELETE: u16_t = 0;
pub const HPI_ADAPTER_OPEN: u16_t = 0;
pub const HPI_ADAPTER_GET_MODE: u16_t = 0;
pub const HPI_ADAPTER_GET_PROPERTY: u16_t = 0;
pub const HPI_ADAPTER_SET_PROPERTY: u16_t = 0;
pub const HPI_OSTREAM_WRITE: u16_t = 0;
pub const HPI_ISTREAM_READ: u16_t = 0;
pub const HPI_ERROR_INVALID_OBJ_INDEX: u16_t = 0;
pub const HPI_ERROR_INVALID_OPERATION: u16_t = 0;
pub const HPI_ERROR_BAD_ADAPTER_NUMBER: u16_t = 0;
pub const HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL: u16_t = 0;
pub const HPI_ERROR_PROCESSING_MESSAGE: u16_t = 0;
pub const HPI_ADAPTER_INDEX_INVALID: u16_t = 0xffff;
pub const HPI_BUS_PCI: c_int = 0;
pub const HPI_ADAPTER_MODE_LOW_LATENCY: c_int = 0;
pub const HPI_ADAPTER_PROPERTY_SUPPORTS_IRQ: c_int = 0;
pub const HPI_ADAPTER_PROPERTY_IRQ_RATE: c_int = 0;
pub const IORESOURCE_MEM: c_ulong = 0;
pub const IRQF_SHARED: c_ulong = 0;
pub const IRQ_NONE: irqreturn_t = 0;
pub const IRQ_WAKE_THREAD: irqreturn_t = 2;
pub const IRQ_HANDLED: irqreturn_t = 1;
pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const EFAULT: c_int = 14;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENODEV: c_int = 19;

extern "C" {
    static HOWNER_KERNEL: *mut file;

    fn hpi_send_recv_ex(phm: *mut hpi_message, phr: *mut hpi_response, file: *mut file);
    fn hpi_init_message_response(
        phm: *mut hpi_message,
        phr: *mut hpi_response,
        object: u16_t,
        function: u16_t,
    );
    fn hpi_init_response(
        phr: *mut hpi_response,
        object: u16_t,
        function: u16_t,
        error: u16_t,
    );
    fn hpi_find_adapter(adapter_index: c_int) -> *mut hpi_adapter_obj;

    fn kmalloc(size: size_t) -> *mut c_void;
    fn kzalloc(size: size_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn vmalloc(size: size_t) -> *mut c_void;
    fn vfree(ptr: *mut c_void);
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_ulong;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: size_t) -> c_ulong;
    fn mutex_lock_interruptible(lock: *mut mutex) -> c_int;
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_init(lock: *mut mutex);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    fn pr_err(fmt: *const c_char, ...);
    fn printk(fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn HPI_DEBUG_LOG(level: c_int, fmt: *const c_char, ...);

    fn pcim_enable_device(pci_dev: *mut pci_dev) -> c_int;
    fn pci_set_master(pci_dev: *mut pci_dev);
    fn pci_resource_flags(pci_dev: *mut pci_dev, bar: c_int) -> c_ulong;
    fn pci_resource_len(pci_dev: *mut pci_dev, bar: c_int) -> c_uint;
    fn pcim_iomap(pci_dev: *mut pci_dev, bar: c_int, len: c_uint) -> *mut c_void;
    fn pci_set_drvdata(pci_dev: *mut pci_dev, data: *mut c_void);
    fn pci_get_drvdata(pci_dev: *mut pci_dev) -> *mut c_void;
    fn request_threaded_irq(
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        flags: c_ulong,
        name: *const c_char,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
}

pub const ERROR: c_int = 0;
pub const WARNING: c_int = 0;
pub const INFO: c_int = 0;
pub const DEBUG: c_int = 0;

static mut adapters: [hpi_adapter; HPI_MAX_ADAPTERS] = [hpi_adapter {
    adapter: null_mut(),
    mutex: mutex { _private: [] },
    p_buffer: null_mut(),
    buffer_size: 0,
    interrupt_callback: None,
    interrupt_mode: 0,
    irq: 0,
}; HPI_MAX_ADAPTERS];

unsafe fn array_index_nospec(index: u16_t, size: usize) -> usize {
    let i = index as usize;
    if i < size { i } else { 0 }
}

unsafe fn hpi_send_recv_f(phm: *mut hpi_message, phr: *mut hpi_response, file: *mut file) {
    if ((*phm).adapter_index as usize >= HPI_MAX_ADAPTERS) && ((*phm).object != HPI_OBJ_SUBSYSTEM) {
        (*phr).error = HPI_ERROR_INVALID_OBJ_INDEX;
    } else {
        hpi_send_recv_ex(phm, phr, file);
    }
}

// This is called from hpifunc.c functions, called by ALSA
// (or other kernel process) In this case there is no file descriptor
// available for the message cache code
#[no_mangle]
pub unsafe extern "C" fn hpi_send_recv(phm: *mut hpi_message, phr: *mut hpi_response) {
    hpi_send_recv_f(phm, phr, HOWNER_KERNEL);
}

// EXPORT_SYMBOL(hpi_send_recv);
// for radio-asihpi

#[no_mangle]
pub unsafe extern "C" fn asihpi_hpi_release(file: *mut file) -> c_int {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    // HPI_DEBUG_LOG(INFO,"hpi_release file %p, pid %d\n", file, current->pid);
    // close the subsystem just in case the application forgot to.
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_CLOSE);
    hpi_send_recv_ex(&mut hm, &mut hr, file);
    0
}

#[no_mangle]
pub unsafe extern "C" fn asihpi_hpi_ioctl(
    file: *mut file,
    cmd: c_uint,
    arg: c_ulong,
) -> c_long {
    let mut phpi_ioctl_data: *mut hpi_ioctl_linux;
    let mut puhm: *mut c_void = null_mut();
    let mut puhr: *mut c_void = null_mut();
    let hm: *mut hpi_message_buffer_v1;
    let hr: *mut hpi_response_buffer_v1;
    let mut msg_size: u16_t = 0;
    let mut res_max_size: u16_t = 0;
    let mut uncopied_bytes: u32_t;
    let mut err: c_int = 0;

    if cmd != HPI_IOCTL_LINUX {
        return -(EINVAL as c_long);
    }

    hm = kmalloc(size_of::<hpi_message_buffer_v1>()) as *mut hpi_message_buffer_v1;
    hr = kzalloc(size_of::<hpi_response_buffer_v1>()) as *mut hpi_response_buffer_v1;
    if hm.is_null() || hr.is_null() {
        err = -ENOMEM;
        goto_out(hm, hr);
        return err as c_long;
    }

    phpi_ioctl_data = arg as *mut hpi_ioctl_linux;

    // Read the message and response pointers from user space.
    if copy_from_user(
        &mut puhm as *mut _ as *mut c_void,
        addr_of_mut!((*phpi_ioctl_data).phm) as *const c_void,
        size_of::<*mut c_void>(),
    ) != 0
        || copy_from_user(
            &mut puhr as *mut _ as *mut c_void,
            addr_of_mut!((*phpi_ioctl_data).phr) as *const c_void,
            size_of::<*mut c_void>(),
        ) != 0
    {
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }

    // Now read the message size and data from user space.
    if copy_from_user(
        &mut msg_size as *mut _ as *mut c_void,
        puhm as *const c_void,
        size_of::<u16_t>(),
    ) != 0
    {
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }
    if msg_size as usize > size_of::<hpi_message_buffer_v1>() {
        msg_size = size_of::<hpi_message_buffer_v1>() as u16_t;
    }

    // printk(KERN_INFO "message size %d\n", hm->h.wSize);

    uncopied_bytes = copy_from_user(hm as *mut c_void, puhm as *const c_void, msg_size as size_t) as u32_t;
    if uncopied_bytes != 0 {
        HPI_DEBUG_LOG(ERROR, b"uncopied bytes %d\n\0".as_ptr() as *const c_char, uncopied_bytes);
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }

    // Override h.size in case it is changed between two userspace fetches
    (*hm).h.size = msg_size;

    if copy_from_user(
        &mut res_max_size as *mut _ as *mut c_void,
        puhr as *const c_void,
        size_of::<u16_t>(),
    ) != 0
    {
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }
    // printk(KERN_INFO "user response size %d\n", res_max_size);
    if (res_max_size as usize) < size_of::<hpi_response_header>() {
        HPI_DEBUG_LOG(WARNING, b"small res size %d\n\0".as_ptr() as *const c_char, res_max_size as c_int);
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }

    res_max_size = core::cmp::min(res_max_size as usize, size_of::<hpi_response_buffer_v1>()) as u16_t;

    match (*hm).h.function {
        HPI_SUBSYS_CREATE_ADAPTER | HPI_ADAPTER_DELETE => {
            // Application must not use these functions!
            (*hr).h.size = size_of::<hpi_response_header>() as u16_t;
            (*hr).h.error = HPI_ERROR_INVALID_OPERATION;
            (*hr).h.function = (*hm).h.function;
            uncopied_bytes =
                copy_to_user(puhr, hr as *const c_void, (*hr).h.size as size_t) as u32_t;
            if uncopied_bytes != 0 {
                err = -EFAULT;
            } else {
                err = 0;
            }
            goto_out(hm, hr);
            return err as c_long;
        }
        _ => {}
    }

    (*hr).h.size = res_max_size;
    if (*hm).h.object == HPI_OBJ_SUBSYSTEM {
        hpi_send_recv_f(&mut (*hm).m0, &mut (*hr).r0, file);
    } else {
        let mut ptr: *mut u16_t = null_mut();
        let mut size: u32_t = 0;
        // -1=no data 0=read from user mem, 1=write to user mem
        let mut wrflag: c_int = -1;
        let mut pa: *mut hpi_adapter = null_mut();

        if ((*hm).h.adapter_index as usize) < adapters.len() {
            pa = &mut adapters[array_index_nospec((*hm).h.adapter_index, adapters.len())];
        }

        if pa.is_null() || (*pa).adapter.is_null() || (*(*pa).adapter).type_ == 0 {
            hpi_init_response(
                &mut (*hr).r0,
                (*hm).h.object,
                (*hm).h.function,
                HPI_ERROR_BAD_ADAPTER_NUMBER,
            );

            uncopied_bytes =
                copy_to_user(puhr, hr as *const c_void, size_of::<hpi_response_header>()) as u32_t;
            if uncopied_bytes != 0 {
                err = -EFAULT;
            } else {
                err = 0;
            }
            goto_out(hm, hr);
            return err as c_long;
        }

        if mutex_lock_interruptible(&mut (*pa).mutex) != 0 {
            err = -EINTR;
            goto_out(hm, hr);
            return err as c_long;
        }

        // Dig out any pointers embedded in the message.
        match (*hm).h.function {
            HPI_OSTREAM_WRITE | HPI_ISTREAM_READ => {
                // Yes, sparse, this is correct.
                ptr = (*hm).m0.u.d.u.data.pb_data as *mut u16_t;
                size = (*hm).m0.u.d.u.data.data_size;

                // Allocate buffer according to application request.
                // ?Is it better to alloc/free for the duration
                // of the transaction?
                if (*pa).buffer_size < size as usize {
                    HPI_DEBUG_LOG(
                        DEBUG,
                        b"Realloc adapter %d stream buffer from %zd to %d\n\0".as_ptr()
                            as *const c_char,
                        (*hm).h.adapter_index as c_int,
                        (*pa).buffer_size,
                        size,
                    );
                    if !(*pa).p_buffer.is_null() {
                        (*pa).buffer_size = 0;
                        vfree((*pa).p_buffer);
                    }
                    (*pa).p_buffer = vmalloc(size as size_t);
                    if !(*pa).p_buffer.is_null() {
                        (*pa).buffer_size = size as size_t;
                    } else {
                        HPI_DEBUG_LOG(
                            ERROR,
                            b"HPI could not allocate stream buffer size %d\n\0".as_ptr()
                                as *const c_char,
                            size,
                        );

                        mutex_unlock(&mut (*pa).mutex);
                        err = -EINVAL;
                        goto_out(hm, hr);
                        return err as c_long;
                    }
                }

                (*hm).m0.u.d.u.data.pb_data = (*pa).p_buffer;
                if (*hm).h.function == HPI_ISTREAM_READ {
                    // from card, WRITE to user mem
                    wrflag = 1;
                } else {
                    wrflag = 0;
                }
            }

            _ => {
                size = 0;
            }
        }

        if size != 0 && wrflag == 0 {
            uncopied_bytes = copy_from_user((*pa).p_buffer, ptr as *const c_void, size as size_t) as u32_t;
            if uncopied_bytes != 0 {
                HPI_DEBUG_LOG(
                    WARNING,
                    b"Missed %d of %d bytes from user\n\0".as_ptr() as *const c_char,
                    uncopied_bytes,
                    size,
                );
            }
        }

        hpi_send_recv_f(&mut (*hm).m0, &mut (*hr).r0, file);

        if size != 0 && wrflag == 1 {
            uncopied_bytes = copy_to_user(ptr as *mut c_void, (*pa).p_buffer, size as size_t) as u32_t;
            if uncopied_bytes != 0 {
                HPI_DEBUG_LOG(
                    WARNING,
                    b"Missed %d of %d bytes to user\n\0".as_ptr() as *const c_char,
                    uncopied_bytes,
                    size,
                );
            }
        }

        mutex_unlock(&mut (*pa).mutex);
    }

    // on return response size must be set
    // printk(KERN_INFO "response size %d\n", hr->h.wSize);

    if (*hr).h.size == 0 {
        HPI_DEBUG_LOG(ERROR, b"response zero size\n\0".as_ptr() as *const c_char);
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }

    if (*hr).h.size > res_max_size {
        HPI_DEBUG_LOG(
            ERROR,
            b"response too big %d %d\n\0".as_ptr() as *const c_char,
            (*hr).h.size as c_int,
            res_max_size as c_int,
        );
        (*hr).h.error = HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL;
        (*hr).h.specific_error = (*hr).h.size;
        (*hr).h.size = size_of::<hpi_response_header>() as u16_t;
    }

    uncopied_bytes = copy_to_user(puhr, hr as *const c_void, (*hr).h.size as size_t) as u32_t;
    if uncopied_bytes != 0 {
        HPI_DEBUG_LOG(ERROR, b"uncopied bytes %d\n\0".as_ptr() as *const c_char, uncopied_bytes);
        err = -EFAULT;
        goto_out(hm, hr);
        return err as c_long;
    }

    goto_out(hm, hr);
    err as c_long
}

unsafe fn goto_out(hm: *mut hpi_message_buffer_v1, hr: *mut hpi_response_buffer_v1) {
    kfree(hm as *mut c_void);
    kfree(hr as *mut c_void);
}

static mut asihpi_irq_count: c_int = 0;

unsafe extern "C" fn asihpi_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let a = dev_id as *mut hpi_adapter;
    let handled: c_int;

    if (*(*a).adapter).irq_query_and_clear.is_none() {
        pr_err(
            b"asihpi_isr ASI%04X:%d no handler\n\0".as_ptr() as *const c_char,
            (*(*a).adapter).type_,
            (*(*a).adapter).index,
        );
        return IRQ_NONE;
    }

    handled = ((*(*a).adapter).irq_query_and_clear.unwrap())((*a).adapter, 0);

    if handled == 0 {
        return IRQ_NONE;
    }

    asihpi_irq_count += 1;
    // printk(KERN_INFO "asihpi_isr %d ASI%04X:%d irq handled\n",
    //    asihpi_irq_count, a->adapter->type, a->adapter->index);

    if (*a).interrupt_callback.is_some() {
        return IRQ_WAKE_THREAD;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn asihpi_isr_thread(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let a = dev_id as *mut hpi_adapter;

    if let Some(callback) = (*a).interrupt_callback {
        callback(a);
    }
    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn asihpi_adapter_probe(
    pci_dev: *mut pci_dev,
    _pci_id: *const pci_device_id,
) -> c_int {
    let mut idx: c_int;
    let nm: c_int;
    let mut low_latency_mode: c_int = 0;
    let mut irq_supported: c_int = 0;
    let adapter_index: c_int;
    let mut memlen: c_uint;
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();
    let mut adapter: hpi_adapter = zeroed();
    let mut pci: hpi_pci = zeroed();

    memset(&mut adapter as *mut _ as *mut c_void, 0, size_of::<hpi_adapter>());

    dev_dbg(
        &mut (*pci_dev).dev,
        b"probe %04x:%04x,%04x:%04x,%04x\n\0".as_ptr() as *const c_char,
        (*pci_dev).vendor,
        (*pci_dev).device,
        (*pci_dev).subsystem_vendor,
        (*pci_dev).subsystem_device,
        (*pci_dev).devfn,
    );

    if pcim_enable_device(pci_dev) < 0 {
        dev_err(
            &mut (*pci_dev).dev,
            b"pci_enable_device failed, disabling device\n\0".as_ptr() as *const c_char,
        );
        return -EIO;
    }

    pci_set_master(pci_dev); // also sets latency timer if < 16

    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_CREATE_ADAPTER);
    hpi_init_response(
        &mut hr,
        HPI_OBJ_SUBSYSTEM,
        HPI_SUBSYS_CREATE_ADAPTER,
        HPI_ERROR_PROCESSING_MESSAGE,
    );

    hm.adapter_index = HPI_ADAPTER_INDEX_INVALID;

    nm = HPI_MAX_ADAPTER_MEM_SPACES as c_int;

    idx = 0;
    while idx < nm {
        HPI_DEBUG_LOG(
            INFO,
            b"resource %d %pR\n\0".as_ptr() as *const c_char,
            idx,
            &mut (*pci_dev).resource[idx as usize] as *mut resource,
        );

        if (pci_resource_flags(pci_dev, idx) & IORESOURCE_MEM) != 0 {
            memlen = pci_resource_len(pci_dev, idx);
            pci.ap_mem_base[idx as usize] = pcim_iomap(pci_dev, idx, memlen);
            if pci.ap_mem_base[idx as usize].is_null() {
                HPI_DEBUG_LOG(ERROR, b"ioremap failed, aborting\n\0".as_ptr() as *const c_char);
                // unmap previously mapped pci mem space
                return adapter_probe_err(&mut adapter);
            }
        }
        idx += 1;
    }

    pci.pci_dev = pci_dev;
    hm.u.s.resource.bus_type = HPI_BUS_PCI;
    hm.u.s.resource.r.pci = &mut pci;

    // call CreateAdapterObject on the relevant hpi module
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);
    if hr.error != 0 {
        return adapter_probe_err(&mut adapter);
    }

    adapter_index = hr.u.s.adapter_index;
    adapter.adapter = hpi_find_adapter(adapter_index);

    if prealloc_stream_buf != 0 {
        adapter.p_buffer = vmalloc(prealloc_stream_buf as size_t);
        if adapter.p_buffer.is_null() {
            HPI_DEBUG_LOG(
                ERROR,
                b"HPI could not allocate kernel buffer size %d\n\0".as_ptr() as *const c_char,
                prealloc_stream_buf,
            );
            return adapter_probe_err(&mut adapter);
        }
    }

    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_OPEN);
    hm.adapter_index = (*adapter.adapter).index as u16_t;
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);

    if hr.error != 0 {
        HPI_DEBUG_LOG(ERROR, b"HPI_ADAPTER_OPEN failed, aborting\n\0".as_ptr() as *const c_char);
        return adapter_probe_err(&mut adapter);
    }

    // Check if current mode == Low Latency mode
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_GET_MODE);
    hm.adapter_index = (*adapter.adapter).index as u16_t;
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);

    if hr.error == 0 && hr.u.ax.mode.adapter_mode == HPI_ADAPTER_MODE_LOW_LATENCY {
        low_latency_mode = 1;
    } else {
        dev_info(
            &mut (*pci_dev).dev,
            b"Adapter at index %d is not in low latency mode\n\0".as_ptr() as *const c_char,
            (*adapter.adapter).index,
        );
    }

    // Check if IRQs are supported
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_GET_PROPERTY);
    hm.adapter_index = (*adapter.adapter).index as u16_t;
    hm.u.ax.property_set.property = HPI_ADAPTER_PROPERTY_SUPPORTS_IRQ;
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);
    if hr.error != 0 || hr.u.ax.property_get.parameter1 == 0 {
        dev_info(
            &mut (*pci_dev).dev,
            b"IRQs not supported by adapter at index %d\n\0".as_ptr() as *const c_char,
            (*adapter.adapter).index,
        );
    } else {
        irq_supported = 1;
    }

    // WARNING can't init mutex in 'adapter'
    // and then copy it to adapters[] ?!?!
    adapters[adapter_index as usize] = adapter;
    mutex_init(&mut adapters[adapter_index as usize].mutex);
    pci_set_drvdata(pci_dev, &mut adapters[adapter_index as usize] as *mut _ as *mut c_void);

    if low_latency_mode != 0 && irq_supported != 0 {
        if (*adapter.adapter).irq_query_and_clear.is_none() {
            dev_err(
                &mut (*pci_dev).dev,
                b"no IRQ handler for adapter %d, aborting\n\0".as_ptr() as *const c_char,
                (*adapter.adapter).index,
            );
            return adapter_probe_err(&mut adapter);
        }

        // Disable IRQ generation on DSP side by setting the rate to 0
        hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_SET_PROPERTY);
        hm.adapter_index = (*adapter.adapter).index as u16_t;
        hm.u.ax.property_set.property = HPI_ADAPTER_PROPERTY_IRQ_RATE;
        hm.u.ax.property_set.parameter1 = 0;
        hm.u.ax.property_set.parameter2 = 0;
        hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);
        if hr.error != 0 {
            HPI_DEBUG_LOG(ERROR, b"HPI_ADAPTER_GET_MODE failed, aborting\n\0".as_ptr() as *const c_char);
            return adapter_probe_err(&mut adapter);
        }

        // Note: request_irq calls asihpi_isr here
        if request_threaded_irq(
            (*pci_dev).irq,
            Some(asihpi_isr),
            Some(asihpi_isr_thread),
            IRQF_SHARED,
            b"asihpi\0".as_ptr() as *const c_char,
            &mut adapters[adapter_index as usize] as *mut _ as *mut c_void,
        ) != 0
        {
            dev_err(
                &mut (*pci_dev).dev,
                b"request_irq(%d) failed\n\0".as_ptr() as *const c_char,
                (*pci_dev).irq,
            );
            return adapter_probe_err(&mut adapter);
        }

        adapters[adapter_index as usize].interrupt_mode = 1;

        dev_info(
            &mut (*pci_dev).dev,
            b"using irq %d\n\0".as_ptr() as *const c_char,
            (*pci_dev).irq,
        );
        adapters[adapter_index as usize].irq = (*pci_dev).irq;
    } else {
        dev_info(&mut (*pci_dev).dev, b"using polled mode\n\0".as_ptr() as *const c_char);
    }

    dev_info(
        &mut (*pci_dev).dev,
        b"probe succeeded for ASI%04X HPI index %d\n\0".as_ptr() as *const c_char,
        (*adapter.adapter).type_,
        adapter_index,
    );

    0
}

unsafe fn adapter_probe_err(adapter: *mut hpi_adapter) -> c_int {
    if !(*adapter).p_buffer.is_null() {
        (*adapter).buffer_size = 0;
        vfree((*adapter).p_buffer);
    }

    HPI_DEBUG_LOG(ERROR, b"adapter_probe failed\n\0".as_ptr() as *const c_char);
    -ENODEV
}

#[no_mangle]
pub unsafe extern "C" fn asihpi_adapter_remove(pci_dev: *mut pci_dev) {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();
    let pa: *mut hpi_adapter;

    pa = pci_get_drvdata(pci_dev) as *mut hpi_adapter;

    // Disable IRQ generation on DSP side
    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_SET_PROPERTY);
    hm.adapter_index = (*(*pa).adapter).index as u16_t;
    hm.u.ax.property_set.property = HPI_ADAPTER_PROPERTY_IRQ_RATE;
    hm.u.ax.property_set.parameter1 = 0;
    hm.u.ax.property_set.parameter2 = 0;
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);

    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_ADAPTER, HPI_ADAPTER_DELETE);
    hm.adapter_index = (*(*pa).adapter).index as u16_t;
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);

    if (*pa).irq != 0 {
        free_irq((*pa).irq, pa as *mut c_void);
    }

    vfree((*pa).p_buffer);

    if 1 != 0 {
        dev_info(
            &mut (*pci_dev).dev,
            b"remove %04x:%04x,%04x:%04x,%04x, HPI index %d\n\0".as_ptr() as *const c_char,
            (*pci_dev).vendor,
            (*pci_dev).device,
            (*pci_dev).subsystem_vendor,
            (*pci_dev).subsystem_device,
            (*pci_dev).devfn,
            (*(*pa).adapter).index,
        );
    }

    memset(pa as *mut c_void, 0, size_of::<hpi_adapter>());
}

#[no_mangle]
pub unsafe extern "C" fn asihpi_init() {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    memset(
        adapters.as_mut_ptr() as *mut c_void,
        0,
        size_of::<[hpi_adapter; HPI_MAX_ADAPTERS]>(),
    );

    printk(b"ASIHPI driver HPI_VER_STRING\n\0".as_ptr() as *const c_char);

    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_DRIVER_LOAD);
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);
}

#[no_mangle]
pub unsafe extern "C" fn asihpi_exit() {
    let mut hm: hpi_message = zeroed();
    let mut hr: hpi_response = zeroed();

    hpi_init_message_response(&mut hm, &mut hr, HPI_OBJ_SUBSYSTEM, HPI_SUBSYS_DRIVER_UNLOAD);
    hpi_send_recv_ex(&mut hm, &mut hr, HOWNER_KERNEL);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
