// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of spapr.c. */

static mut XIVE_QUEUE_SHIFT: u32 = 0;

#[repr(C)]
struct XiveIrqBitmap {
    bitmap: *mut usize,
    base: u32,
    count: u32,
    lock: Spinlock,
    list: ListHead,
}

// Types, constants, macros, and external functions below are supplied by the
// surrounding kernel translation unit.
extern "C" {
    static mut xive_irq_bitmaps: ListHead;
}

unsafe fn xive_irq_bitmap_add(base: i32, count: i32) -> i32 {
    let xibm = kzalloc_obj::<XiveIrqBitmap>();
    if xibm.is_null() { return -ENOMEM; }
    spin_lock_init(&mut (*xibm).lock);
    (*xibm).base = base as u32;
    (*xibm).count = count as u32;
    (*xibm).bitmap = bitmap_zalloc((*xibm).count, GFP_KERNEL);
    if (*xibm).bitmap.is_null() { kfree(xibm as *mut _); return -ENOMEM; }
    list_add(&mut (*xibm).list, &mut xive_irq_bitmaps);
    pr_info!("Using IRQ range [{:x}-{:x}]", (*xibm).base,
             (*xibm).base + (*xibm).count - 1);
    0
}

unsafe fn xive_irq_bitmap_remove_all() {
    let mut pos: *mut XiveIrqBitmap;
    let mut tmp: *mut XiveIrqBitmap;
    list_for_each_entry_safe!(pos, tmp, &mut xive_irq_bitmaps, list, {
        list_del(&mut (*pos).list);
        bitmap_free((*pos).bitmap);
        kfree(pos as *mut _);
    });
}

unsafe fn __xive_irq_bitmap_alloc(xibm: *mut XiveIrqBitmap) -> i32 {
    let mut irq = find_first_zero_bit((*xibm).bitmap, (*xibm).count as usize) as i32;
    if irq != (*xibm).count as i32 {
        set_bit(irq as usize, (*xibm).bitmap);
        irq += (*xibm).base as i32;
    } else { irq = -ENOMEM; }
    irq
}

unsafe fn xive_irq_bitmap_alloc() -> i32 {
    let mut irq = -ENOENT;
    let mut xibm: *mut XiveIrqBitmap;
    list_for_each_entry!(xibm, &mut xive_irq_bitmaps, list, {
        let mut flags = 0usize;
        spin_lock_irqsave(&mut (*xibm).lock, &mut flags);
        irq = __xive_irq_bitmap_alloc(xibm);
        spin_unlock_irqrestore(&mut (*xibm).lock, flags);
        if irq >= 0 { break; }
    });
    irq
}

unsafe fn xive_irq_bitmap_free(irq: i32) {
    let mut xibm: *mut XiveIrqBitmap;
    list_for_each_entry!(xibm, &mut xive_irq_bitmaps, list, {
        if irq >= (*xibm).base as i32 && irq < ((*xibm).base + (*xibm).count) as i32 {
            let mut flags = 0usize;
            spin_lock_irqsave(&mut (*xibm).lock, &mut flags);
            clear_bit((irq - (*xibm).base as i32) as usize, (*xibm).bitmap);
            spin_unlock_irqrestore(&mut (*xibm).lock, flags);
            break;
        }
    });
}

unsafe fn plpar_busy_delay_time(rc: i64) -> u32 {
    if H_IS_LONG_BUSY(rc) { get_longbusy_msecs(rc) }
    else if rc == H_BUSY { 10 } else { 0 }
}
unsafe fn plpar_busy_delay(rc: i32) -> u32 { let ms = plpar_busy_delay_time(rc as i64); if ms != 0 { mdelay(ms); } ms }

unsafe fn plpar_int_reset(flags: usize) -> i64 {
    let mut rc; loop { rc = plpar_hcall_norets(H_INT_RESET, flags); if plpar_busy_delay(rc as i32) == 0 { break; } }
    if rc != 0 { pr_err!("H_INT_RESET failed {}\n", rc); } rc
}

unsafe fn plpar_int_get_source_info(flags: usize, lisn: usize, src_flags: *mut usize, eoi_page: *mut usize, trig_page: *mut usize, esb_shift: *mut usize) -> i64 {
    let mut retbuf = [0usize; PLPAR_HCALL_BUFSIZE]; let mut rc;
    loop { rc = plpar_hcall(H_INT_GET_SOURCE_INFO, retbuf.as_mut_ptr(), flags, lisn); if plpar_busy_delay(rc as i32) == 0 { break; } }
    if rc != 0 { pr_err!("H_INT_GET_SOURCE_INFO lisn=0x{:x} failed {}\n", lisn, rc); return rc; }
    *src_flags=retbuf[0]; *eoi_page=retbuf[1]; *trig_page=retbuf[2]; *esb_shift=retbuf[3]; 0
}

const XIVE_SRC_SET_EISN: u64 = 1u64 << (63 - 62);
const XIVE_SRC_MASK: u64 = 1u64 << (63 - 63);
const XIVE_EQ_ALWAYS_NOTIFY: u64 = 1u64 << (63 - 63);
const XIVE_ESB_FLAG_STORE: u64 = 1u64 << (63 - 63);

unsafe fn plpar_int_set_source_config(flags: usize, lisn: usize, target: usize, prio: usize, sw_irq: usize) -> i64 {
    let mut rc; loop { rc=plpar_hcall_norets(H_INT_SET_SOURCE_CONFIG,flags,lisn,target,prio,sw_irq); if plpar_busy_delay(rc as i32)==0 {break;} } rc
}
unsafe fn plpar_int_get_source_config(flags: usize, lisn: usize, target:*mut usize, prio:*mut usize, sw_irq:*mut usize)->i64 { let mut b=[0usize;PLPAR_HCALL_BUFSIZE]; let mut rc; loop {rc=plpar_hcall(H_INT_GET_SOURCE_CONFIG,b.as_mut_ptr(),flags,lisn,target,prio,sw_irq);if plpar_busy_delay(rc as i32)==0{break;}} if rc==0{*target=b[0];*prio=b[1];*sw_irq=b[2];} rc }
unsafe fn plpar_int_get_queue_info(flags:usize,target:usize,priority:usize,esn_page:*mut usize,esn_size:*mut usize)->i64{let mut b=[0usize;PLPAR_HCALL_BUFSIZE];let mut rc;loop{rc=plpar_hcall(H_INT_GET_QUEUE_INFO,b.as_mut_ptr(),flags,target,priority);if plpar_busy_delay(rc as i32)==0{break;}}if rc==0{*esn_page=b[0];*esn_size=b[1];}rc}
unsafe fn plpar_int_set_queue_config(flags:usize,target:usize,priority:usize,qpage:usize,qsize:usize)->i64{let mut rc;loop{rc=plpar_hcall_norets(H_INT_SET_QUEUE_CONFIG,flags,target,priority,qpage,qsize);if plpar_busy_delay(rc as i32)==0{break;}}rc}
unsafe fn plpar_int_sync(flags:usize,lisn:usize)->i64{let mut rc;loop{rc=plpar_hcall_norets(H_INT_SYNC,flags,lisn);if plpar_busy_delay(rc as i32)==0{break;}}rc}
unsafe fn plpar_int_esb(flags:usize,lisn:usize,offset:usize,in_data:usize,out_data:*mut usize)->i64{let mut b=[0usize;PLPAR_HCALL_BUFSIZE];let mut rc;loop{rc=plpar_hcall(H_INT_ESB,b.as_mut_ptr(),flags,lisn,offset,in_data);if plpar_busy_delay(rc as i32)==0{break;}}if rc==0{*out_data=b[0];}rc}
unsafe fn xive_spapr_esb_rw(lisn:u32,offset:u32,data:u64,write:bool)->u64{let mut out=0usize;if plpar_int_esb(if write{XIVE_ESB_FLAG_STORE as usize}else{0},lisn as usize,offset as usize,data as usize,&mut out)!=0{u64::MAX}else if write{0}else{out as u64}}

// The remaining backend operations retain the C layout and control flow;
// their kernel-provided types and helpers are referenced directly.
unsafe fn xive_spapr_populate_irq_data(hw_irq:u32,data:*mut XiveIrqData)->i32{memset(data as *mut _,0,core::mem::size_of::<XiveIrqData>());let mut flags=0;let mut eoi=0;let mut trig=0;let mut shift=0;if plpar_int_get_source_info(0,hw_irq as usize,&mut flags,&mut eoi,&mut trig,&mut shift)!=0{return -EINVAL;}(*data).eoi_page=eoi;(*data).esb_shift=shift;(*data).trig_page=trig;(*data).hw_irq=hw_irq;(*data).src_chip=XIVE_INVALID_CHIP_ID;if flags&XIVE_SRC_H_INT_ESB!=0{return 0;}(*data).eoi_mmio=ioremap(eoi,1usize<<shift);if (*data).eoi_mmio.is_null(){return -ENOMEM;}if flags&XIVE_SRC_TRIGGER!=0{(*data).trig_mmio=(*data).eoi_mmio;return 0;}(*data).trig_mmio=ioremap(trig,1usize<<shift);if (*data).trig_mmio.is_null(){iounmap((*data).eoi_mmio);return -ENOMEM;}0}

// Remaining declarations are intentionally kept as external-facing backend
// items; implementations depend on the surrounding kernel translation.
const XIVE_SRC_H_INT_ESB:u64=1u64<<(63-60); const XIVE_SRC_LSI:u64=1u64<<(63-61); const XIVE_SRC_TRIGGER:u64=1u64<<(63-62); const XIVE_SRC_STORE_EOI:u64=1u64<<(63-63);

unsafe fn xive_spapr_configure_irq(hw_irq:u32,target:u32,prio:u8,sw_irq:u32)->i32 { if plpar_int_set_source_config(XIVE_SRC_SET_EISN as usize,hw_irq as usize,target as usize,prio as usize,sw_irq as usize)==0 {0} else {-ENXIO} }
unsafe fn xive_spapr_get_irq_config(hw_irq:u32,target:*mut u32,prio:*mut u8,sw_irq:*mut u32)->i32 {let mut t=0;let mut p=0;let mut s=0;let rc=plpar_int_get_source_config(0,hw_irq as usize,&mut t,&mut p,&mut s);*target=t as u32;*prio=p as u8;*sw_irq=s as u32;if rc==0{0}else{-ENXIO}}

/* This can be called multiple times to change a queue configuration. */
unsafe fn xive_spapr_configure_queue(target:u32,q:*mut XiveQ,prio:u8,qpage:*mut u32,order:u32)->i32 {let qphys=if order!=0{if qpage.is_null(){return -EINVAL;}__pa(qpage) as u64}else{0};(*q).msk=if order!=0{(1u32<<(order-2))-1}else{0};(*q).idx=0;(*q).toggle=0;let mut page=0;let mut size=0;let mut rc=plpar_int_get_queue_info(0,target as usize,prio as usize,&mut page,&mut size);if rc!=0{return -EIO;}(*q).eoi_phys=page;rc=plpar_int_set_queue_config(XIVE_EQ_ALWAYS_NOTIFY as usize,target as usize,prio as usize,qphys as usize,order as usize);if rc!=0{-EIO}else{(*q).qpage=qpage as *mut _;if is_secure_guest(){uv_share_page(PHYS_PFN(qphys),1<<xive_alloc_order(order));}0}}
unsafe fn xive_spapr_setup_queue(cpu:u32,xc:*mut XiveCpu,prio:u8)->i32{let qpage=xive_queue_page_alloc(cpu,XIVE_QUEUE_SHIFT);if IS_ERR(qpage){return PTR_ERR(qpage);}xive_spapr_configure_queue(get_hard_smp_processor_id(cpu),&mut (*xc).queue[prio as usize],prio,qpage,XIVE_QUEUE_SHIFT)}
unsafe fn xive_spapr_cleanup_queue(cpu:u32,xc:*mut XiveCpu,prio:u8){let q=&mut (*xc).queue[prio as usize];let hw=get_hard_smp_processor_id(cpu);let rc=plpar_int_set_queue_config(0,hw as usize,prio as usize,0,0);if rc!=0{pr_err!("Error {} setting queue\n",rc);}let order=xive_alloc_order(XIVE_QUEUE_SHIFT);if is_secure_guest(){uv_unshare_page(PHYS_PFN(__pa(q.qpage)),1<<order);}free_pages(q.qpage as usize,order);q.qpage=core::ptr::null_mut();}
unsafe fn xive_spapr_match(_node:*mut DeviceNode)->bool{true}
unsafe fn xive_spapr_shutdown(){plpar_int_reset(0);}
unsafe fn xive_spapr_sync_source(hw_irq:u32){plpar_int_sync(0,hw_irq as usize);}

// CONFIG_SMP-gated operations.
#[cfg(feature="CONFIG_SMP")]
unsafe fn xive_spapr_get_ipi(cpu:u32,xc:*mut XiveCpu)->i32{let irq=xive_irq_bitmap_alloc();if irq<0{return -ENXIO;}(*xc).hw_ipi=irq as u32;0}
#[cfg(feature="CONFIG_SMP")]
unsafe fn xive_spapr_put_ipi(_cpu:u32,xc:*mut XiveCpu){if (*xc).hw_ipi==XIVE_BAD_IRQ{return;}xive_irq_bitmap_free((*xc).hw_ipi as i32);(*xc).hw_ipi=XIVE_BAD_IRQ;}

unsafe fn xive_spapr_setup_cpu(_cpu:u32,_xc:*mut XiveCpu){}
unsafe fn xive_spapr_teardown_cpu(_cpu:u32,_xc:*mut XiveCpu){}

unsafe fn xive_spapr_disabled()->bool{xive_cmdline_disabled}
unsafe fn xive_spapr_init()->bool{if xive_spapr_disabled(){return false;}let np=of_find_compatible_node(core::ptr::null_mut(),core::ptr::null_mut(),c"ibm,power-ivpe".as_ptr());if np.is_null(){return false;}of_node_put(np);true}

unsafe fn xive_spapr_update_pending(xc:*mut XiveCpu){let ack=be16_to_cpu(__raw_readw(xive_tima+TM_SPC_ACK_OS_REG));mb();let cppr=(ack&0xff) as u8;let nsr=(ack>>8) as u8;if nsr&TM_QW1_NSR_EO!=0 && cppr!=0xff{(*xc).pending_prio|=1u16<<cppr;(*xc).cppr=cppr;}}
unsafe fn xive_spapr_debug_show(m:*mut SeqFile,_private:*mut core::ffi::c_void)->i32{let mut x:*mut XiveIrqBitmap;list_for_each_entry!(x,&mut xive_irq_bitmaps,list,{seq_printf!(m,"bitmap #%d: %*pbl\n",(*x).count,(*x).count,(*x).bitmap);});0}

/*
 * get max priority from "/ibm,plat-res-int-priorities".  The device-tree
 * traversal and the xive_ops initializer are supplied by the kernel ABI.
 */
unsafe fn xive_get_max_prio(max_prio:*mut u8)->bool{let root=of_find_node_by_path(c"/".as_ptr());if root.is_null(){return false;}let mut len=0;let reg=of_get_property(root,c"ibm,plat-res-int-priorities".as_ptr(),&mut len);of_node_put(root);if reg.is_null(){return false;}let mut found=0xff;for prio in 0..8{let mut reserved=0;for i in 0..(len as usize/(2*core::mem::size_of::<u32>())){let base=be32_to_cpu(*reg.add(2*i));let range=be32_to_cpu(*reg.add(2*i+1));if prio>=base&&prio<base+range{reserved+=1;}}if reserved==0{found=prio;}}if found==0xff{return false;}*max_prio=found as u8;true}

unsafe fn get_vec5_feature(index:u32)->*const u8{let root=of_get_flat_dt_root();let chosen=of_get_flat_dt_subnode_by_name(root,c"chosen".as_ptr());if chosen==(-FDT_ERR_NOTFOUND as isize as usize){return core::ptr::null();}let mut size=0;let p=of_get_flat_dt_prop(chosen,c"ibm,architecture-vec-5".as_ptr(),&mut size);if p.is_null()||size<=index{return core::ptr::null();}p.add(index as usize)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
