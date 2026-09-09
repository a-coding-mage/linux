// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Common functions for DMA access on PA Semi PWRficient
 */

// Linux kernel dependencies and build-time configuration are supplied externally.

const MAX_TXCH: usize = 64;
const MAX_RXCH: usize = 64;
const MAX_FLAGS: usize = 64;
const MAX_FUN: usize = 8;
const MAX_RETRIES: i32 = 5000;

static mut dma_status: *mut pasdma_status = core::ptr::null_mut();
static mut iob_regs: *mut core::ffi::c_void = core::ptr::null_mut();
static mut mac_regs: [*mut core::ffi::c_void; 6] = [core::ptr::null_mut(); 6];
static mut dma_regs: *mut core::ffi::c_void = core::ptr::null_mut();
static mut base_hw_irq: i32 = 0;
static mut num_txch: i32 = 0;
static mut num_rxch: i32 = 0;
static mut dma_pdev: *mut pci_dev = core::ptr::null_mut();

static mut txch_free: [u64; 1] = [0; 1];
static mut rxch_free: [u64; 1] = [0; 1];
static mut flags_free: [u64; 1] = [0; 1];
static mut fun_free: [u64; 1] = [0; 1];

pub unsafe fn pasemi_read_iob_reg(reg: u32) -> u32 { in_le32(iob_regs.add(reg as usize)) }
pub unsafe fn pasemi_write_iob_reg(reg: u32, val: u32) { out_le32(iob_regs.add(reg as usize), val); }
pub unsafe fn pasemi_read_mac_reg(intf: i32, reg: u32) -> u32 { in_le32(mac_regs[intf as usize].add(reg as usize)) }
pub unsafe fn pasemi_write_mac_reg(intf: i32, reg: u32, val: u32) { out_le32(mac_regs[intf as usize].add(reg as usize), val); }
pub unsafe fn pasemi_read_dma_reg(reg: u32) -> u32 { in_le32(dma_regs.add(reg as usize)) }
pub unsafe fn pasemi_write_dma_reg(reg: u32, val: u32) { out_le32(dma_regs.add(reg as usize), val); }

unsafe fn pasemi_alloc_tx_chan(ty: pasemi_dmachan_type) -> i32 {
    let (start, limit) = match ty & (TXCHAN_EVT0 | TXCHAN_EVT1) {
        TXCHAN_EVT0 => (0, 10), TXCHAN_EVT1 => (10, MAX_TXCH as i32), _ => (0, MAX_TXCH as i32)
    };
    loop { let bit = find_next_bit(&txch_free, MAX_TXCH, start as usize) as i32; if bit >= limit { return -ENOSPC; } if test_and_clear_bit(bit as usize, &mut txch_free) { return bit; } }
}
unsafe fn pasemi_free_tx_chan(chan: i32) { BUG_ON(test_bit(chan as usize, &txch_free)); set_bit(chan as usize, &mut txch_free); }
unsafe fn pasemi_alloc_rx_chan() -> i32 {
    loop { let bit = find_first_bit(&rxch_free, MAX_RXCH) as i32; if bit >= MAX_TXCH as i32 { return -ENOSPC; } if test_and_clear_bit(bit as usize, &mut rxch_free) { return bit; } }
}
unsafe fn pasemi_free_rx_chan(chan: i32) { BUG_ON(test_bit(chan as usize, &rxch_free)); set_bit(chan as usize, &mut rxch_free); }

pub unsafe fn pasemi_dma_alloc_chan(ty: pasemi_dmachan_type, total_size: i32, offset: i32) -> *mut core::ffi::c_void {
    BUG_ON(total_size < core::mem::size_of::<pasemi_dmachan>() as i32);
    let buf = kzalloc(total_size as usize, GFP_KERNEL); if buf.is_null() { return core::ptr::null_mut(); }
    let chan = (buf as *mut u8).add(offset as usize) as *mut pasemi_dmachan; (*chan).priv_ = buf;
    match ty & (TXCHAN | RXCHAN) { RXCHAN => { let chno = pasemi_alloc_rx_chan(); (*chan).chno=chno; (*chan).irq=irq_create_mapping(core::ptr::null_mut(), base_hw_irq+num_txch+chno); (*chan).status=&mut (*dma_status).rx_sta[chno as usize]; }, TXCHAN => { let chno=pasemi_alloc_tx_chan(ty); (*chan).chno=chno; (*chan).irq=irq_create_mapping(core::ptr::null_mut(), base_hw_irq+chno); (*chan).status=&mut (*dma_status).tx_sta[chno as usize]; }, _ => {} }
    (*chan).chan_type=ty; chan as *mut core::ffi::c_void
}
pub unsafe fn pasemi_dma_free_chan(chan: *mut pasemi_dmachan) { if !(*chan).ring_virt.is_null() { pasemi_dma_free_ring(chan); } match (*chan).chan_type & (RXCHAN|TXCHAN) { RXCHAN=>pasemi_free_rx_chan((*chan).chno), TXCHAN=>pasemi_free_tx_chan((*chan).chno), _=>{} } kfree((*chan).priv_); }
pub unsafe fn pasemi_dma_alloc_ring(chan:*mut pasemi_dmachan, ring_size:i32)->i32 { BUG_ON(!(*chan).ring_virt.is_null()); (*chan).ring_size=ring_size; (*chan).ring_virt=dma_alloc_coherent(&mut (*dma_pdev).dev, (ring_size as usize)*core::mem::size_of::<u64>(), &mut (*chan).ring_dma, GFP_KERNEL); if (*chan).ring_virt.is_null(){-ENOMEM}else{0} }
pub unsafe fn pasemi_dma_free_ring(chan:*mut pasemi_dmachan){ BUG_ON((*chan).ring_virt.is_null()); dma_free_coherent(&mut (*dma_pdev).dev, (*chan).ring_size as usize*core::mem::size_of::<u64>(), (*chan).ring_virt, (*chan).ring_dma); (*chan).ring_virt=core::ptr::null_mut(); (*chan).ring_size=0; (*chan).ring_dma=0; }
pub unsafe fn pasemi_dma_start_chan(chan:*const pasemi_dmachan, cmdsta:u32){ if (*chan).chan_type==RXCHAN {pasemi_write_dma_reg(PAS_DMA_RXCHAN_CCMDSTA((*chan).chno),cmdsta|PAS_DMA_RXCHAN_CCMDSTA_EN)}else{pasemi_write_dma_reg(PAS_DMA_TXCHAN_TCMDSTA((*chan).chno),cmdsta|PAS_DMA_TXCHAN_TCMDSTA_EN)} }
pub unsafe fn pasemi_dma_stop_chan(chan:*const pasemi_dmachan)->i32 { let reg=if (*chan).chan_type==RXCHAN{PAS_DMA_RXCHAN_CCMDSTA((*chan).chno)}else{PAS_DMA_TXCHAN_TCMDSTA((*chan).chno)}; let st=if (*chan).chan_type==RXCHAN{PAS_DMA_RXCHAN_CCMDSTA_ST}else{PAS_DMA_TXCHAN_TCMDSTA_ST}; let act=if (*chan).chan_type==RXCHAN{PAS_DMA_RXCHAN_CCMDSTA_ACT}else{PAS_DMA_TXCHAN_TCMDSTA_ACT}; pasemi_write_dma_reg(reg,st); for _ in 0..MAX_RETRIES { if pasemi_read_dma_reg(reg)&act==0 {pasemi_write_dma_reg(reg,0);return 1} cond_resched(); } 0 }
pub unsafe fn pasemi_dma_alloc_buf(_chan:*mut pasemi_dmachan,size:i32,handle:*mut dma_addr_t)->*mut core::ffi::c_void{dma_alloc_coherent(&mut (*dma_pdev).dev,size as usize,handle,GFP_KERNEL)}
pub unsafe fn pasemi_dma_free_buf(_chan:*mut pasemi_dmachan,size:i32,handle:*mut dma_addr_t){dma_free_coherent(&mut (*dma_pdev).dev,size as usize,handle,GFP_KERNEL)}
pub unsafe fn pasemi_dma_alloc_flag()->i32{loop{let bit=find_first_bit(&flags_free,MAX_FLAGS) as i32;if bit>=MAX_FLAGS as i32{return -ENOSPC}if test_and_clear_bit(bit as usize,&mut flags_free){return bit}}}
pub unsafe fn pasemi_dma_free_flag(flag:i32){BUG_ON(test_bit(flag as usize,&flags_free));BUG_ON(flag>=MAX_FLAGS as i32);set_bit(flag as usize,&mut flags_free)}
pub unsafe fn pasemi_dma_set_flag(flag:i32){BUG_ON(flag>=MAX_FLAGS as i32);if flag<32{pasemi_write_dma_reg(PAS_DMA_TXF_SFLG0,1u32<<flag)}else{pasemi_write_dma_reg(PAS_DMA_TXF_SFLG1,1u32<<flag)}}
pub unsafe fn pasemi_dma_clear_flag(flag:i32){BUG_ON(flag>=MAX_FLAGS as i32);if flag<32{pasemi_write_dma_reg(PAS_DMA_TXF_CFLG0,1u32<<flag)}else{pasemi_write_dma_reg(PAS_DMA_TXF_CFLG1,1u32<<flag)}}
pub unsafe fn pasemi_dma_alloc_fun()->i32{loop{let bit=find_first_bit(&fun_free,MAX_FLAGS) as i32;if bit>=MAX_FLAGS as i32{return -ENOSPC}if test_and_clear_bit(bit as usize,&mut fun_free){return bit}}}
pub unsafe fn pasemi_dma_free_fun(fun:i32){BUG_ON(test_bit(fun as usize,&fun_free));BUG_ON(fun>=MAX_FLAGS as i32);set_bit(fun as usize,&mut fun_free)}

unsafe fn map_onedev(p:*mut pci_dev,index:i32)->*mut core::ffi::c_void { let dn=pci_device_to_OF_node(p); if !dn.is_null(){let ret=of_iomap(dn,index);if !ret.is_null(){return ret}} ioremap(0xe0000000u64+(((*p).devfn as u64)<<12),0x2000) }

pub unsafe fn pasemi_dma_init()->i32 {
    static mut init_lock: spinlock_t = spinlock_t::new(); let mut iob_pdev; let mut pdev; let mut res=resource::default(); let mut dn; let mut err=0; let mut timeout; let mut tmp;
    if !machine_is(pasemi){return -ENODEV} spin_lock(&mut init_lock); if !dma_pdev.is_null(){spin_unlock(&mut init_lock);return 0}
    iob_pdev=pci_get_device(PCI_VENDOR_ID_PASEMI,0xa001,core::ptr::null_mut()); if iob_pdev.is_null(){BUG();pr_warn!("Can't find I/O Bridge\n");err=-ENODEV;spin_unlock(&mut init_lock);return err} iob_regs=map_onedev(iob_pdev,0);
    dma_pdev=pci_get_device(PCI_VENDOR_ID_PASEMI,0xa007,core::ptr::null_mut()); if dma_pdev.is_null(){BUG();pr_warn!("Can't find DMA controller\n");err=-ENODEV;spin_unlock(&mut init_lock);return err} dma_regs=map_onedev(dma_pdev,0);base_hw_irq=virq_to_hw((*dma_pdev).irq);pci_read_config_dword(dma_pdev,PAS_DMA_CAP_TXCH,&mut tmp);num_txch=((tmp&PAS_DMA_CAP_TXCH_TCHN_M)>>PAS_DMA_CAP_TXCH_TCHN_S) as i32;pci_read_config_dword(dma_pdev,PAS_DMA_CAP_RXCH,&mut tmp);num_rxch=((tmp&PAS_DMA_CAP_RXCH_RCHN_M)>>PAS_DMA_CAP_RXCH_RCHN_S) as i32;
    let mut intf=0; pdev=pci_get_device(PCI_VENDOR_ID_PASEMI,0xa006,core::ptr::null_mut());while !pdev.is_null(){mac_regs[intf]=map_onedev(pdev,0);intf+=1;pdev=pci_get_device(PCI_VENDOR_ID_PASEMI,0xa006,pdev)}pci_dev_put(pdev);pdev=pci_get_device(PCI_VENDOR_ID_PASEMI,0xa005,core::ptr::null_mut());while !pdev.is_null(){mac_regs[intf]=map_onedev(pdev,0);intf+=1;pdev=pci_get_device(PCI_VENDOR_ID_PASEMI,0xa005,pdev)}pci_dev_put(pdev);
    dn=pci_device_to_OF_node(iob_pdev);if !dn.is_null(){err=of_address_to_resource(dn,1,&mut res)}if dn.is_null()||err!=0{res.start=0xfd800000;res.end=res.start+0x1000}dma_status=ioremap_cache(res.start,resource_size(&res));pci_dev_put(iob_pdev);
    for i in 0..MAX_TXCH{__set_bit(i,&mut txch_free)}for i in 0..MAX_RXCH{__set_bit(i,&mut rxch_free)} timeout=jiffies()+HZ;pasemi_write_dma_reg(PAS_DMA_COM_RXCMD,0);while pasemi_read_dma_reg(PAS_DMA_COM_RXSTA)&1!=0{if time_after(jiffies(),timeout){pr_warn!("Warning: Could not disable RX section\n");break}} timeout=jiffies()+HZ;pasemi_write_dma_reg(PAS_DMA_COM_TXCMD,0);while pasemi_read_dma_reg(PAS_DMA_COM_TXSTA)&1!=0{if time_after(jiffies(),timeout){pr_warn!("Warning: Could not disable TX section\n");break}}
    tmp=pasemi_read_dma_reg(PAS_DMA_COM_CFG);pasemi_write_dma_reg(PAS_DMA_COM_CFG,tmp|0x18000000);pasemi_write_dma_reg(PAS_DMA_COM_TXCMD,PAS_DMA_COM_TXCMD_EN);pasemi_write_dma_reg(PAS_DMA_COM_RXCMD,PAS_DMA_COM_RXCMD_EN);for i in 0..MAX_FLAGS{__set_bit(i,&mut flags_free)}for i in 0..MAX_FUN{__set_bit(i,&mut fun_free)}pasemi_write_dma_reg(PAS_DMA_TXF_CFLG0,0xffffffff);pasemi_write_dma_reg(PAS_DMA_TXF_CFLG1,0xffffffff);pr_info!("PA Semi PWRficient DMA library initialized (%d tx, %d rx channels)\n",num_txch,num_rxch);spin_unlock(&mut init_lock);err
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
