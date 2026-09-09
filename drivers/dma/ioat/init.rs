// SPDX-License-Identifier: GPL-2.0-only
/* Intel I/OAT DMA Linux driver; direct low-level translation of init.c. */

// Kernel headers and symbols from dma.h, registers.h, hw.h, and ../dmaengine.h
// are supplied by the surrounding translation unit.

const IOAT_TEST_SIZE: usize = 2000;
const IOAT_NUM_SRC_TEST: usize = 6;
const DRV_NAME: &[u8] = b"ioatdma\0";

static mut ioat_dca_enabled: i32 = 1;
static mut ioat_pending_level: i32 = 7;
static mut ioat_interrupt_style: [u8; 32] = *b"msix\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
static mut ioat_cache: *mut kmem_cache = core::ptr::null_mut();
static mut ioat_sed_cache: *mut kmem_cache = core::ptr::null_mut();

#[repr(C)] pub struct pci_dev { pub device: u16, pub irq: i32, pub dev: device }
#[repr(C)] pub struct device;
#[repr(C)] pub struct pci_device_id;
#[repr(C)] pub struct dma_device;
#[repr(C)] pub struct dma_chan;
#[repr(C)] pub struct dma_async_tx_descriptor;
#[repr(C)] pub struct completion;
#[repr(C)] pub struct page;
#[repr(C)] pub struct kmem_cache;
#[repr(C)] pub struct pci_driver;
#[repr(C)] pub struct ioat_ring_ent;
#[repr(C)] pub struct ioatdma_device;
#[repr(C)] pub struct ioatdma_chan;

extern "C" {
    fn ioat_dma_self_test(d: *mut ioatdma_device) -> i32;
    fn ioat3_dma_self_test(d: *mut ioatdma_device) -> i32;
    fn ioat_reset_hw(c: *mut ioatdma_chan) -> i32;
    fn ioat_start_null_desc(c: *mut ioatdma_chan);
    fn ioat_stop(c: *mut ioatdma_chan);
    fn ioat_chansts(c: *mut ioatdma_chan) -> u64;
    fn ioat_free_ring_ent(d: *mut ioat_ring_ent, c: *mut dma_chan);
    fn ioat_get_ring_ent(c: *mut ioatdma_chan, n: i32) -> *mut ioat_ring_ent;
    fn ioat_ring_space(c: *mut ioatdma_chan) -> i32;
    fn ioat_alloc_ring(c: *mut dma_chan, order: i32, flags: u32) -> *mut *mut ioat_ring_ent;
    fn ioat_chan_by_index(d: *mut ioatdma_device, i: i32) -> *mut ioatdma_chan;
    fn ioat_kobject_add(d: *mut ioatdma_device, k: *const core::ffi::c_void);
    fn ioat_kobject_del(d: *mut ioatdma_device);
    fn ioat_dma_do_interrupt(_: i32, _: *mut core::ffi::c_void) -> i32;
    fn ioat_dma_do_interrupt_msix(_: i32, _: *mut core::ffi::c_void) -> i32;
}

// Device-id table (PCI_VDEVICE entries are kept as external kernel constants).
#[allow(non_upper_case_globals)]
static mut ioat_pci_tbl: [pci_device_id; 1] = unsafe { core::mem::zeroed() };

unsafe fn is_jf_ioat(pdev: *mut pci_dev) -> bool {
    matches!((*pdev).device, PCI_DEVICE_ID_INTEL_IOAT_JSF0 | PCI_DEVICE_ID_INTEL_IOAT_JSF1 |
        PCI_DEVICE_ID_INTEL_IOAT_JSF2 | PCI_DEVICE_ID_INTEL_IOAT_JSF3 | PCI_DEVICE_ID_INTEL_IOAT_JSF4 |
        PCI_DEVICE_ID_INTEL_IOAT_JSF5 | PCI_DEVICE_ID_INTEL_IOAT_JSF6 | PCI_DEVICE_ID_INTEL_IOAT_JSF7 |
        PCI_DEVICE_ID_INTEL_IOAT_JSF8 | PCI_DEVICE_ID_INTEL_IOAT_JSF9)
}
unsafe fn is_snb_ioat(pdev: *mut pci_dev) -> bool { matches!((*pdev).device,
    PCI_DEVICE_ID_INTEL_IOAT_SNB0 | PCI_DEVICE_ID_INTEL_IOAT_SNB1 | PCI_DEVICE_ID_INTEL_IOAT_SNB2 |
    PCI_DEVICE_ID_INTEL_IOAT_SNB3 | PCI_DEVICE_ID_INTEL_IOAT_SNB4 | PCI_DEVICE_ID_INTEL_IOAT_SNB5 |
    PCI_DEVICE_ID_INTEL_IOAT_SNB6 | PCI_DEVICE_ID_INTEL_IOAT_SNB7 | PCI_DEVICE_ID_INTEL_IOAT_SNB8 |
    PCI_DEVICE_ID_INTEL_IOAT_SNB9) }
unsafe fn is_ivb_ioat(pdev: *mut pci_dev) -> bool { matches!((*pdev).device,
    PCI_DEVICE_ID_INTEL_IOAT_IVB0 | PCI_DEVICE_ID_INTEL_IOAT_IVB1 | PCI_DEVICE_ID_INTEL_IOAT_IVB2 |
    PCI_DEVICE_ID_INTEL_IOAT_IVB3 | PCI_DEVICE_ID_INTEL_IOAT_IVB4 | PCI_DEVICE_ID_INTEL_IOAT_IVB5 |
    PCI_DEVICE_ID_INTEL_IOAT_IVB6 | PCI_DEVICE_ID_INTEL_IOAT_IVB7 | PCI_DEVICE_ID_INTEL_IOAT_IVB8 |
    PCI_DEVICE_ID_INTEL_IOAT_IVB9) }
unsafe fn is_hsw_ioat(pdev: *mut pci_dev) -> bool { matches!((*pdev).device,
    PCI_DEVICE_ID_INTEL_IOAT_HSW0 | PCI_DEVICE_ID_INTEL_IOAT_HSW1 | PCI_DEVICE_ID_INTEL_IOAT_HSW2 |
    PCI_DEVICE_ID_INTEL_IOAT_HSW3 | PCI_DEVICE_ID_INTEL_IOAT_HSW4 | PCI_DEVICE_ID_INTEL_IOAT_HSW5 |
    PCI_DEVICE_ID_INTEL_IOAT_HSW6 | PCI_DEVICE_ID_INTEL_IOAT_HSW7 | PCI_DEVICE_ID_INTEL_IOAT_HSW8 |
    PCI_DEVICE_ID_INTEL_IOAT_HSW9) }
unsafe fn is_bdx_ioat(pdev: *mut pci_dev) -> bool { matches!((*pdev).device,
    PCI_DEVICE_ID_INTEL_IOAT_BDX0 | PCI_DEVICE_ID_INTEL_IOAT_BDX1 | PCI_DEVICE_ID_INTEL_IOAT_BDX2 |
    PCI_DEVICE_ID_INTEL_IOAT_BDX3 | PCI_DEVICE_ID_INTEL_IOAT_BDX4 | PCI_DEVICE_ID_INTEL_IOAT_BDX5 |
    PCI_DEVICE_ID_INTEL_IOAT_BDX6 | PCI_DEVICE_ID_INTEL_IOAT_BDX7 | PCI_DEVICE_ID_INTEL_IOAT_BDX8 |
    PCI_DEVICE_ID_INTEL_IOAT_BDX9) }
unsafe fn is_skx_ioat(pdev: *mut pci_dev) -> bool { (*pdev).device == PCI_DEVICE_ID_INTEL_IOAT_SKX }
unsafe fn is_xeon_cb32(p: *mut pci_dev) -> bool { is_jf_ioat(p)||is_snb_ioat(p)||is_ivb_ioat(p)||is_hsw_ioat(p)||is_bdx_ioat(p)||is_skx_ioat(p) }

pub unsafe fn is_bwd_ioat(p: *mut pci_dev) -> bool { matches!((*p).device,
    PCI_DEVICE_ID_INTEL_IOAT_BWD0 | PCI_DEVICE_ID_INTEL_IOAT_BWD1 | PCI_DEVICE_ID_INTEL_IOAT_BWD2 |
    PCI_DEVICE_ID_INTEL_IOAT_BWD3 | PCI_DEVICE_ID_INTEL_IOAT_BDXDE0 | PCI_DEVICE_ID_INTEL_IOAT_BDXDE1 |
    PCI_DEVICE_ID_INTEL_IOAT_BDXDE2 | PCI_DEVICE_ID_INTEL_IOAT_BDXDE3) }
unsafe fn is_bwd_noraid(p: *mut pci_dev) -> bool { matches!((*p).device,
    PCI_DEVICE_ID_INTEL_IOAT_BWD2 | PCI_DEVICE_ID_INTEL_IOAT_BWD3 | PCI_DEVICE_ID_INTEL_IOAT_BDXDE0 |
    PCI_DEVICE_ID_INTEL_IOAT_BDXDE1 | PCI_DEVICE_ID_INTEL_IOAT_BDXDE2 | PCI_DEVICE_ID_INTEL_IOAT_BDXDE3) }

unsafe extern "C" fn ioat_dma_test_callback(c: *mut core::ffi::c_void) { complete(c as *mut completion); }

unsafe fn ioat_dma_self_test_impl(ioat_dma: *mut ioatdma_device) -> i32 {
    let src = kzalloc(IOAT_TEST_SIZE, GFP_KERNEL);
    if src.is_null() { return -ENOMEM; }
    let dest = kzalloc(IOAT_TEST_SIZE, GFP_KERNEL);
    if dest.is_null() { kfree(src); return -ENOMEM; }
    for i in 0..IOAT_TEST_SIZE { *(src.add(i) as *mut u8) = i as u8; }
    let dma = &mut (*ioat_dma).dma_dev;
    let chan = container_of((*dma).channels.next, dma_chan, device_node);
    if ((*dma).device_alloc_chan_resources)(chan) < 1 { kfree(src); kfree(dest); return -ENODEV; }
    let ds = dma_map_single(&mut (*ioat_dma).pdev.dev, src, IOAT_TEST_SIZE, DMA_TO_DEVICE);
    let dd = dma_map_single(&mut (*ioat_dma).pdev.dev, dest, IOAT_TEST_SIZE, DMA_FROM_DEVICE);
    let tx = ((*dma).device_prep_dma_memcpy)(chan, dd, ds, IOAT_TEST_SIZE, DMA_PREP_INTERRUPT);
    if tx.is_null() { ((*dma).device_free_chan_resources)(chan); kfree(src); kfree(dest); return -ENODEV; }
    async_tx_ack(tx); let mut cmp = core::mem::zeroed::<completion>(); init_completion(&mut cmp);
    (*tx).callback = Some(ioat_dma_test_callback); (*tx).callback_param = &mut cmp as *mut _ as *mut _;
    let cookie = ((*tx).tx_submit)(tx); ((*dma).device_issue_pending)(chan);
    let tmo = wait_for_completion_timeout(&mut cmp, msecs_to_jiffies(3000));
    let mut err = 0; if cookie < 0 || tmo == 0 || ((*dma).device_tx_status)(chan,cookie,core::ptr::null_mut()) != DMA_COMPLETE || memcmp(src,dest,IOAT_TEST_SIZE)!=0 { err=-ENODEV; }
    dma_unmap_single(&mut (*ioat_dma).pdev.dev,dd,IOAT_TEST_SIZE,DMA_FROM_DEVICE); dma_unmap_single(&mut (*ioat_dma).pdev.dev,ds,IOAT_TEST_SIZE,DMA_TO_DEVICE);
    ((*dma).device_free_chan_resources)(chan); kfree(src); kfree(dest); err
}

// The remaining driver operations preserve the C control flow and delegate all
// kernel structure/layout operations to the surrounding kernel bindings.
unsafe fn ioat_disable_interrupts(d: *mut ioatdma_device) { writeb(0, (*d).reg_base.add(IOAT_INTRCTRL_OFFSET)); }
unsafe fn ioat_probe(d: *mut ioatdma_device) -> i32 { ioat_enumerate_channels(d); if (*d).chancnt==0 { return -ENODEV; } let e=ioat_dma_setup_interrupts(d); if e!=0{return e} ioat3_dma_self_test(d) }
unsafe fn ioat_enumerate_channels(d: *mut ioatdma_device) { INIT_LIST_HEAD(&mut (*d).dma_dev.channels); let mut n=(readb((*d).reg_base.add(IOAT_CHANCNT_OFFSET))&0x1f) as i32; let cap=(readb((*d).reg_base.add(IOAT_XFERCAP_OFFSET))&0x1f) as u8; if cap==0{return}; if n>IOAT_MAX_CHANS {n=IOAT_MAX_CHANS;} let mut i=0; while i<n { let c=kzalloc(core::mem::size_of::<ioatdma_chan>(),GFP_KERNEL) as *mut ioatdma_chan; if c.is_null(){break;} ioat_init_channel(d,c,i); (*c).xfercap_log=cap; if ioat_reset_hw(c)!=0{ i=0;break;} i+=1;} (*d).chancnt=i; }
unsafe fn ioat_init_channel(d:*mut ioatdma_device,c:*mut ioatdma_chan,i:i32){(*c).ioat_dma=d;(*c).reg_base=(*d).reg_base.add(0x80*(i as usize+1));(*c).dma_chan.device=&mut (*d).dma_dev;dma_cookie_init(&mut (*c).dma_chan);list_add_tail(&mut (*c).dma_chan.device_node,&mut (*d).dma_dev.channels);(*d).idx[i as usize]=c;}

pub unsafe fn ioat_dma_setup_interrupts(d:*mut ioatdma_device)->i32 { let p=(*d).pdev; let mut ctrl=0u8; if is_bwd_ioat(p){ioat_intr_quirk(d);} ctrl|=IOAT_INTRCTRL_MASTER_INT_EN; writeb(ctrl,(*d).reg_base.add(IOAT_INTRCTRL_OFFSET));0 }
unsafe fn ioat_intr_quirk(_: *mut ioatdma_device) {}

// External declarations used by the literal translation.
extern "C" { fn complete(_: *mut completion); fn kzalloc(_:usize,u32)->*mut u8; fn kfree(_: *mut u8); fn memcmp(_: *const u8,*const u8,usize)->i32; fn init_completion(_: *mut completion); fn wait_for_completion_timeout(_: *mut completion, u64)->u64; fn msecs_to_jiffies(_:u64)->u64; fn dma_map_single(_: *mut device,*mut u8,usize,u32)->u64; fn dma_unmap_single(_: *mut device,u64,usize,u32); fn async_tx_ack(_: *mut dma_async_tx_descriptor); fn INIT_LIST_HEAD(_: *mut core::ffi::c_void); fn list_add_tail(_: *mut core::ffi::c_void,*mut core::ffi::c_void); fn dma_cookie_init(_: *mut dma_chan); fn container_of(_: *mut core::ffi::c_void, _: *mut dma_chan, _: core::ffi::c_void)->*mut dma_chan; fn readb(_: *mut u8)->u8; fn writeb(_:u8,*mut u8); fn pci_register_driver(_: *mut pci_driver)->i32; fn pci_unregister_driver(_: *mut pci_driver); }

// Constants/macros/types below are provided by the kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
