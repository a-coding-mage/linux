



















MODULE_FIRMWARE("amdgpu/sienna_cichlid_sdma.bin");
MODULE_FIRMWARE("amdgpu/navy_flounder_sdma.bin");
MODULE_FIRMWARE("amdgpu/dimgrey_cavefish_sdma.bin");
MODULE_FIRMWARE("amdgpu/beige_goby_sdma.bin");

MODULE_FIRMWARE("amdgpu/vangogh_sdma.bin");
MODULE_FIRMWARE("amdgpu/yellow_carp_sdma.bin");
MODULE_FIRMWARE("amdgpu/sdma_5_2_6.bin");
MODULE_FIRMWARE("amdgpu/sdma_5_2_7.bin");






const amdgpu_hwip_reg_entry sdma_reg_list_5_2[] = {
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_STATUS_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_STATUS1_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_STATUS2_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_STATUS3_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UCODE_CHECKSUM),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RB_RPTR_FETCH_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RB_RPTR_FETCH),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UTCL1_RD_STATUS),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UTCL1_WR_STATUS),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UTCL1_RD_XNACK0),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UTCL1_RD_XNACK1),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UTCL1_WR_XNACK0),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_UTCL1_WR_XNACK1),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_RB_CNTL),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_RB_RPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_RB_RPTR_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_RB_WPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_RB_WPTR_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_IB_OFFSET),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_IB_BASE_LO),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_IB_BASE_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_IB_CNTL),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_IB_RPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_IB_SUB_REMAIN),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_GFX_DUMMY_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_RB_CNTL),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_RB_RPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_RB_RPTR_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_RB_WPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_RB_WPTR_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_IB_OFFSET),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_IB_BASE_LO),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_IB_BASE_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_PAGE_DUMMY_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_RB_CNTL),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_RB_RPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_RB_RPTR_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_RB_WPTR),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_RB_WPTR_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_IB_OFFSET),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_IB_BASE_LO),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_IB_BASE_HI),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_RLC0_DUMMY_REG),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_INT_STATUS),
	SOC15_REG_ENTRY_STR(GC, 0, mmSDMA0_VM_CNTL),
	SOC15_REG_ENTRY_STR(GC, 0, mmGRBM_STATUS2)
};

() sdma_v5_2_set_ring_funcs(struct amdgpu_device *adev);
() sdma_v5_2_set_buffer_funcs(struct amdgpu_device *adev);
() sdma_v5_2_set_irq_funcs(struct amdgpu_device *adev);
i32 sdma_v5_2_stop_queue(struct amdgpu_ring *ring);
i32 sdma_v5_2_restore_queue(struct amdgpu_ring *ring);

u32 sdma_v5_2_get_reg_offset(struct amdgpu_device *adev, u32 instance, u32 internal_offset)
{
	u32 base;

	if (internal_offset >= SDMA0_HYP_DEC_REG_START &&
	    internal_offset <= SDMA0_HYP_DEC_REG_END) {
		base = adev->reg_offset[GC_HWIP][0][1];
		if (instance != 0)
			internal_offset += SDMA1_HYP_DEC_REG_OFFSET * instance;
	} else {
		if (instance < 2) {
			base = adev->reg_offset[GC_HWIP][0][0];
			if (instance == 1)
				internal_offset += SDMA1_REG_OFFSET;
		} else {
			base = adev->reg_offset[GC_HWIP][0][2];
			if (instance == 3)
				internal_offset += SDMA3_REG_OFFSET;
		}
	}

	return base + internal_offset;
}

u32 sdma_v5_2_ring_init_cond_exec(struct amdgpu_ring *ring,
					      u64 addr)
{
	u32 ret;

	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_COND_EXE));
	amdgpu_ring_write(ring, lower_32_bits(addr));
	amdgpu_ring_write(ring, upper_32_bits(addr));
	amdgpu_ring_write(ring, 1);
	
	ret = ring->wptr & ring->buf_mask;
	
	amdgpu_ring_write(ring, 0);

	return ret;
}


u64 sdma_v5_2_ring_get_rptr(struct amdgpu_ring *ring)
{
	u64 *rptr;

	
	rptr = (u64 *)ring->rptr_cpu_addr;

	DRM_DEBUG("rptr before shift == 0x%016llx\n", *rptr);
	return ((*rptr) >> 2);
}


u64 sdma_v5_2_ring_get_wptr(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;
	u64 wptr;

	if (ring->use_doorbell) {
		
		wptr = READ_ONCE(*((u64 *)ring->wptr_cpu_addr));
		DRM_DEBUG("wptr/doorbell before shift == 0x%016llx\n", wptr);
	} else {
		wptr = RREG32(sdma_v5_2_get_reg_offset(adev, ring->me, mmSDMA0_GFX_RB_WPTR_HI));
		wptr = wptr << 32;
		wptr |= RREG32(sdma_v5_2_get_reg_offset(adev, ring->me, mmSDMA0_GFX_RB_WPTR));
		DRM_DEBUG("wptr before shift [%i] wptr == 0x%016llx\n", ring->me, wptr);
	}

	return wptr >> 2;
}


() sdma_v5_2_ring_set_wptr(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;

	DRM_DEBUG("Setting write pointer\n");
	if (ring->use_doorbell) {
		DRM_DEBUG("Using doorbell -- "
				"wptr_offs == 0x%08x "
				"lower_32_bits(ring->wptr << 2) == 0x%08x "
				"upper_32_bits(ring->wptr << 2) == 0x%08x\n",
				ring->wptr_offs,
				lower_32_bits(ring->wptr << 2),
				upper_32_bits(ring->wptr << 2));
		
		atomic64_set((atomic64_t *)ring->wptr_cpu_addr,
			     ring->wptr << 2);
		DRM_DEBUG("calling WDOORBELL64(0x%08x, 0x%016llx)\n",
				ring->doorbell_index, ring->wptr << 2);
		WDOORBELL64(ring->doorbell_index, ring->wptr << 2);
		if (amdgpu_ip_version(adev, SDMA0_HWIP, 0) == IP_VERSION(5, 2, 1)) {
			
			WREG32(sdma_v5_2_get_reg_offset(adev, ring->me, mmSDMA0_GFX_RB_WPTR),
			       lower_32_bits(ring->wptr << 2));
			WREG32(sdma_v5_2_get_reg_offset(adev, ring->me, mmSDMA0_GFX_RB_WPTR_HI),
			       upper_32_bits(ring->wptr << 2));
		}
	} else {
		DRM_DEBUG("Not using doorbell -- "
				"mmSDMA%i_GFX_RB_WPTR == 0x%08x "
				"mmSDMA%i_GFX_RB_WPTR_HI == 0x%08x\n",
				ring->me,
				lower_32_bits(ring->wptr << 2),
				ring->me,
				upper_32_bits(ring->wptr << 2));
		WREG32(sdma_v5_2_get_reg_offset(adev, ring->me, mmSDMA0_GFX_RB_WPTR),
			lower_32_bits(ring->wptr << 2));
		WREG32(sdma_v5_2_get_reg_offset(adev, ring->me, mmSDMA0_GFX_RB_WPTR_HI),
			upper_32_bits(ring->wptr << 2));
	}
}

() sdma_v5_2_ring_insert_nop(struct amdgpu_ring *ring, u32 count)
{
	struct amdgpu_sdma_instance *sdma = amdgpu_sdma_get_instance_from_ring(ring);
	i32 i;

	for (i = 0; i < count; i++)
		if (sdma && sdma->burst_nop && (i == 0))
			amdgpu_ring_write(ring, ring->funcs->nop |
				SDMA_PKT_NOP_HEADER_COUNT(count - 1));
		else
			amdgpu_ring_write(ring, ring->funcs->nop);
}


() sdma_v5_2_ring_emit_ib(struct amdgpu_ring *ring,
				   struct amdgpu_job *job,
				   struct amdgpu_ib *ib,
				   u32 flags)
{
	u32 vmid = AMDGPU_JOB_GET_VMID(job);
	u64 csa_mc_addr = amdgpu_sdma_get_csa_mc_addr(ring, vmid);

	
	sdma_v5_2_ring_insert_nop(ring, (2 - lower_32_bits(ring->wptr)) & 7);

	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_INDIRECT) |
			  SDMA_PKT_INDIRECT_HEADER_VMID(vmid & 0xf));
	
	amdgpu_ring_write(ring, lower_32_bits(ib->gpu_addr) & 0xffffffe0);
	amdgpu_ring_write(ring, upper_32_bits(ib->gpu_addr));
	amdgpu_ring_write(ring, ib->length_dw);
	amdgpu_ring_write(ring, lower_32_bits(csa_mc_addr));
	amdgpu_ring_write(ring, upper_32_bits(csa_mc_addr));
}


() sdma_v5_2_ring_emit_mem_sync(struct amdgpu_ring *ring)
{
	u32 gcr_cntl = SDMA_GCR_GL2_INV | SDMA_GCR_GL2_WB |
			    SDMA_GCR_GLM_INV | SDMA_GCR_GL1_INV |
			    SDMA_GCR_GLV_INV | SDMA_GCR_GLK_INV |
			    SDMA_GCR_GLI_INV(1);

	
	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_GCR_REQ));
	amdgpu_ring_write(ring, SDMA_PKT_GCR_REQ_PAYLOAD1_BASE_VA_31_7(0));
	amdgpu_ring_write(ring, SDMA_PKT_GCR_REQ_PAYLOAD2_GCR_CONTROL_15_0(gcr_cntl) |
			SDMA_PKT_GCR_REQ_PAYLOAD2_BASE_VA_47_32(0));
	amdgpu_ring_write(ring, SDMA_PKT_GCR_REQ_PAYLOAD3_LIMIT_VA_31_7(0) |
			SDMA_PKT_GCR_REQ_PAYLOAD3_GCR_CONTROL_18_16(gcr_cntl >> 16));
	amdgpu_ring_write(ring, SDMA_PKT_GCR_REQ_PAYLOAD4_LIMIT_VA_47_32(0) |
			SDMA_PKT_GCR_REQ_PAYLOAD4_VMID(0));
}


() sdma_v5_2_ring_emit_hdp_flush(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;
	u32 ref_and_mask = 0;
	const struct nbio_hdp_flush_reg *nbio_hf_reg = adev->nbio.hdp_flush_reg;

	if (ring->me > 1) {
		amdgpu_hdp_flush(adev, ring);
	} else {
		ref_and_mask = nbio_hf_reg->ref_and_mask_sdma0 << ring->me;

		amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_POLL_REGMEM) |
				  SDMA_PKT_POLL_REGMEM_HEADER_HDP_FLUSH(1) |
				  SDMA_PKT_POLL_REGMEM_HEADER_FUNC(3)); 
		amdgpu_ring_write(ring, (adev->nbio.funcs->get_hdp_flush_done_offset(adev)) << 2);
		amdgpu_ring_write(ring, (adev->nbio.funcs->get_hdp_flush_req_offset(adev)) << 2);
		amdgpu_ring_write(ring, ref_and_mask); 
		amdgpu_ring_write(ring, ref_and_mask); 
		amdgpu_ring_write(ring, SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT(0xfff) |
				  SDMA_PKT_POLL_REGMEM_DW5_INTERVAL(10)); 
	}
}


() sdma_v5_2_ring_emit_fence(struct amdgpu_ring *ring, u64 addr, u64 seq,
				      u32 flags)
{
	bool write64bit = flags & AMDGPU_FENCE_FLAG_64BIT;
	
	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_FENCE) |
			  SDMA_PKT_FENCE_HEADER_MTYPE(0x3)); 
	
	WARN_ON(addr & 0x3);
	amdgpu_ring_write(ring, lower_32_bits(addr));
	amdgpu_ring_write(ring, upper_32_bits(addr));
	amdgpu_ring_write(ring, lower_32_bits(seq));

	
	if (write64bit) {
		addr += 4;
		amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_FENCE) |
				  SDMA_PKT_FENCE_HEADER_MTYPE(0x3));
		
		WARN_ON(addr & 0x3);
		amdgpu_ring_write(ring, lower_32_bits(addr));
		amdgpu_ring_write(ring, upper_32_bits(addr));
		amdgpu_ring_write(ring, upper_32_bits(seq));
	}

	if ((flags & AMDGPU_FENCE_FLAG_INT)) {
		
		amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_TRAP));
		amdgpu_ring_write(ring, SDMA_PKT_TRAP_INT_CONTEXT_INT_CONTEXT(0));
	}
}



() sdma_v5_2_gfx_stop(struct amdgpu_device *adev,  u32 inst_mask)
{
	u32 rb_cntl, ib_cntl;
	i32 i;

	for_each_inst(i, inst_mask) {
		rb_cntl = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_CNTL));
		rb_cntl = REG_SET_FIELD(rb_cntl, SDMA0_GFX_RB_CNTL, RB_ENABLE, 0);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_CNTL), rb_cntl);
		ib_cntl = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_IB_CNTL));
		ib_cntl = REG_SET_FIELD(ib_cntl, SDMA0_GFX_IB_CNTL, IB_ENABLE, 0);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_IB_CNTL), ib_cntl);
	}
}


() sdma_v5_2_rlc_stop(struct amdgpu_device *adev)
{
	
}


() sdma_v5_2_ctx_switch_enable(struct amdgpu_device *adev, bool enable)
{
	u32 f32_cntl, phase_quantum = 0;
	i32 i;

	if (amdgpu_sdma_phase_quantum) {
		u32 value = amdgpu_sdma_phase_quantum;
		u32 unit = 0;

		while (value > (SDMA0_PHASE0_QUANTUM__VALUE_MASK >>
				SDMA0_PHASE0_QUANTUM__VALUE__SHIFT)) {
			value = (value + 1) >> 1;
			unit++;
		}
		if (unit > (SDMA0_PHASE0_QUANTUM__UNIT_MASK >>
			    SDMA0_PHASE0_QUANTUM__UNIT__SHIFT)) {
			value = (SDMA0_PHASE0_QUANTUM__VALUE_MASK >>
				 SDMA0_PHASE0_QUANTUM__VALUE__SHIFT);
			unit = (SDMA0_PHASE0_QUANTUM__UNIT_MASK >>
				SDMA0_PHASE0_QUANTUM__UNIT__SHIFT);
			WARN_ONCE(1,
			"clamping sdma_phase_quantum to %uK clock cycles\n",
				  value << unit);
		}
		phase_quantum =
			value << SDMA0_PHASE0_QUANTUM__VALUE__SHIFT |
			unit  << SDMA0_PHASE0_QUANTUM__UNIT__SHIFT;
	}

	for (i = 0; i < adev->sdma.num_instances; i++) {
		if (enable && amdgpu_sdma_phase_quantum) {
			WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_PHASE0_QUANTUM),
			       phase_quantum);
			WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_PHASE1_QUANTUM),
			       phase_quantum);
			WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_PHASE2_QUANTUM),
			       phase_quantum);
		}

		if (!amdgpu_sriov_vf(adev)) {
			f32_cntl = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CNTL));
			f32_cntl = REG_SET_FIELD(f32_cntl, SDMA0_CNTL,
					AUTO_CTXSW_ENABLE, enable ? 1 : 0);
			WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CNTL), f32_cntl);
		}
	}

}


() sdma_v5_2_enable(struct amdgpu_device *adev, bool enable)
{
	u32 f32_cntl;
	i32 i;
	u32 inst_mask;

	inst_mask = GENMASK(adev->sdma.num_instances - 1, 0);
	if (!enable) {
		sdma_v5_2_gfx_stop(adev, inst_mask);
		sdma_v5_2_rlc_stop(adev);
	}

	if (!amdgpu_sriov_vf(adev)) {
		for (i = 0; i < adev->sdma.num_instances; i++) {
			f32_cntl = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_F32_CNTL));
			f32_cntl = REG_SET_FIELD(f32_cntl, SDMA0_F32_CNTL, HALT, enable ? 0 : 1);
			WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_F32_CNTL), f32_cntl);
		}
	}
}



i32 sdma_v5_2_gfx_resume_instance(struct amdgpu_device *adev, i32 i, bool restore)
{
	struct amdgpu_ring *ring;
	u32 rb_cntl, ib_cntl;
	u32 rb_bufsz;
	u32 doorbell;
	u32 doorbell_offset;
	u32 temp;
	u32 wptr_poll_cntl;
	u64 wptr_gpu_addr;

	ring = &adev->sdma.instance[i].ring;

	if (!amdgpu_sriov_vf(adev))
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_SEM_WAIT_FAIL_TIMER_CNTL), 0);

	
	rb_bufsz = order_base_2(ring->ring_size / 4);
	rb_cntl = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_CNTL));
	rb_cntl = REG_SET_FIELD(rb_cntl, SDMA0_GFX_RB_CNTL, RB_SIZE, rb_bufsz);

	rb_cntl = REG_SET_FIELD(rb_cntl, SDMA0_GFX_RB_CNTL, RB_SWAP_ENABLE, 1);
	rb_cntl = REG_SET_FIELD(rb_cntl, SDMA0_GFX_RB_CNTL,
				RPTR_WRITEBACK_SWAP_ENABLE, 1);

	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_CNTL), rb_cntl);

	
	if (restore) {
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_RPTR), lower_32_bits(ring->wptr << 2));
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_RPTR_HI), upper_32_bits(ring->wptr << 2));
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR), lower_32_bits(ring->wptr << 2));
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR_HI), upper_32_bits(ring->wptr << 2));
	} else {
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_RPTR), 0);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_RPTR_HI), 0);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR), 0);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR_HI), 0);
	}

	
	wptr_gpu_addr = ring->wptr_gpu_addr;
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR_POLL_ADDR_LO),
	       lower_32_bits(wptr_gpu_addr));
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR_POLL_ADDR_HI),
	       upper_32_bits(wptr_gpu_addr));
	wptr_poll_cntl = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i,
						 mmSDMA0_GFX_RB_WPTR_POLL_CNTL));
	wptr_poll_cntl = REG_SET_FIELD(wptr_poll_cntl,
				       SDMA0_GFX_RB_WPTR_POLL_CNTL,
				       F32_POLL_ENABLE, 1);
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR_POLL_CNTL),
	       wptr_poll_cntl);

	
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_RPTR_ADDR_HI),
	       upper_32_bits(ring->rptr_gpu_addr) & 0xFFFFFFFF);
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_RPTR_ADDR_LO),
	       lower_32_bits(ring->rptr_gpu_addr) & 0xFFFFFFFC);

	rb_cntl = REG_SET_FIELD(rb_cntl, SDMA0_GFX_RB_CNTL, RPTR_WRITEBACK_ENABLE, 1);

	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_BASE), ring->gpu_addr >> 8);
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_BASE_HI), ring->gpu_addr >> 40);

	if (!restore)
		ring->wptr = 0;

	
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_MINOR_PTR_UPDATE), 1);

	if (!amdgpu_sriov_vf(adev)) { 
		WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR), lower_32_bits(ring->wptr << 2));
		WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_WPTR_HI), upper_32_bits(ring->wptr << 2));
	}

	doorbell = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_DOORBELL));
	doorbell_offset = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_DOORBELL_OFFSET));

	if (ring->use_doorbell) {
		doorbell = REG_SET_FIELD(doorbell, SDMA0_GFX_DOORBELL, ENABLE, 1);
		doorbell_offset = REG_SET_FIELD(doorbell_offset, SDMA0_GFX_DOORBELL_OFFSET,
				OFFSET, ring->doorbell_index);
	} else {
		doorbell = REG_SET_FIELD(doorbell, SDMA0_GFX_DOORBELL, ENABLE, 0);
	}
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_DOORBELL), doorbell);
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_DOORBELL_OFFSET), doorbell_offset);

	adev->nbio.funcs->sdma_doorbell_range(adev, i, ring->use_doorbell,
					      ring->doorbell_index,
					      adev->doorbell_index.sdma_doorbell_range);

	if (amdgpu_sriov_vf(adev))
		sdma_v5_2_ring_set_wptr(ring);

	

	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_MINOR_PTR_UPDATE), 0);

	
	if (!amdgpu_sriov_vf(adev)) {
		
		temp = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CNTL));
		temp = REG_SET_FIELD(temp, SDMA0_CNTL, UTC_L1_ENABLE, 1);

		
		temp = REG_SET_FIELD(temp, SDMA0_CNTL, MIDCMD_PREEMPT_ENABLE, 1);
		WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CNTL), temp);

		
		temp = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UTCL1_CNTL));
		temp = REG_SET_FIELD(temp, SDMA0_UTCL1_CNTL, RESP_MODE, 3);
		temp = REG_SET_FIELD(temp, SDMA0_UTCL1_CNTL, REDO_DELAY, 9);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UTCL1_CNTL), temp);

		
		temp = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UTCL1_PAGE));
		
		temp &= 0xFF0FFF;
		temp |= ((CACHE_READ_POLICY_L2__DEFAULT << 12) |
			 (CACHE_WRITE_POLICY_L2__DEFAULT << 14) |
			 SDMA0_UTCL1_PAGE__LLC_NOALLOC_MASK);
		WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UTCL1_PAGE), temp);

		
		temp = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_F32_CNTL));
		temp = REG_SET_FIELD(temp, SDMA0_F32_CNTL, HALT, 0);
		WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_F32_CNTL), temp);
	}

	
	rb_cntl = REG_SET_FIELD(rb_cntl, SDMA0_GFX_RB_CNTL, RB_ENABLE, 1);
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_RB_CNTL), rb_cntl);

	ib_cntl = RREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_IB_CNTL));
	ib_cntl = REG_SET_FIELD(ib_cntl, SDMA0_GFX_IB_CNTL, IB_ENABLE, 1);

	ib_cntl = REG_SET_FIELD(ib_cntl, SDMA0_GFX_IB_CNTL, IB_SWAP_ENABLE, 1);

	
	WREG32_SOC15_IP(GC, sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_GFX_IB_CNTL), ib_cntl);

	if (amdgpu_sriov_vf(adev)) { 
		sdma_v5_2_ctx_switch_enable(adev, true);
		sdma_v5_2_enable(adev, true);
	}

	return amdgpu_ring_test_helper(ring);
}


i32 sdma_v5_2_gfx_resume(struct amdgpu_device *adev)
{
	i32 i, r;

	for (i = 0; i < adev->sdma.num_instances; i++) {
		r = sdma_v5_2_gfx_resume_instance(adev, i, false);
		if (r)
			return r;
	}

	return 0;
}


i32 sdma_v5_2_rlc_resume(struct amdgpu_device *adev)
{
	return 0;
}


i32 sdma_v5_2_load_microcode(struct amdgpu_device *adev)
{
	const struct sdma_firmware_header_v1_0 *hdr;
	const __le32 *fw_data;
	u32 fw_size;
	i32 i, j;

	
	sdma_v5_2_enable(adev, false);

	for (i = 0; i < adev->sdma.num_instances; i++) {
		if (!adev->sdma.instance[i].fw)
			return -EINVAL;

		hdr = (const struct sdma_firmware_header_v1_0 *)adev->sdma.instance[i].fw->data;
		amdgpu_ucode_print_sdma_hdr(&hdr->header);
		fw_size = le32_to_cpu(hdr->header.ucode_size_bytes) / 4;

		fw_data = (const __le32 *)
			(adev->sdma.instance[i].fw->data +
				le32_to_cpu(hdr->header.ucode_array_offset_bytes));

		WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UCODE_ADDR), 0);

		for (j = 0; j < fw_size; j++) {
			if (amdgpu_emu_mode == 1 && j % 500 == 0)
				msleep(1);
			WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UCODE_DATA), le32_to_cpup(fw_data++));
		}

		WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_UCODE_ADDR), adev->sdma.instance[i].fw_version);
	}

	return 0;
}

i32 sdma_v5_2_soft_reset_engine(struct amdgpu_device *adev, u32 instance_id)
{
	u32 grbm_soft_reset;
	u32 tmp;

	grbm_soft_reset = REG_SET_FIELD(0,
					GRBM_SOFT_RESET, SOFT_RESET_SDMA0,
					1);
	grbm_soft_reset <<= instance_id;

	tmp = RREG32_SOC15(GC, 0, mmGRBM_SOFT_RESET);
	tmp |= grbm_soft_reset;
	DRM_DEBUG("GRBM_SOFT_RESET=0x%08X\n", tmp);
	WREG32_SOC15(GC, 0, mmGRBM_SOFT_RESET, tmp);
	tmp = RREG32_SOC15(GC, 0, mmGRBM_SOFT_RESET);

	udelay(50);

	tmp &= ~grbm_soft_reset;
	WREG32_SOC15(GC, 0, mmGRBM_SOFT_RESET, tmp);
	tmp = RREG32_SOC15(GC, 0, mmGRBM_SOFT_RESET);
	return 0;
}

i32 sdma_v5_2_soft_reset(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 i;

	for (i = 0; i < adev->sdma.num_instances; i++) {
		sdma_v5_2_soft_reset_engine(adev, i);
		udelay(50);
	}

	return 0;
}

const amdgpu_sdma_funcs sdma_v5_2_sdma_funcs = {
	.stop_kernel_queue = &sdma_v5_2_stop_queue,
	.start_kernel_queue = &sdma_v5_2_restore_queue,
	.soft_reset_kernel_queue = &sdma_v5_2_soft_reset_engine,
};


i32 sdma_v5_2_start(struct amdgpu_device *adev)
{
	i32 r = 0;
	struct amdgpu_ip_block *ip_block;

	if (amdgpu_sriov_vf(adev)) {
		sdma_v5_2_ctx_switch_enable(adev, false);
		sdma_v5_2_enable(adev, false);

		
		r = sdma_v5_2_gfx_resume(adev);
		return r;
	}

	if (adev->firmware.load_type == AMDGPU_FW_LOAD_DIRECT) {
		r = sdma_v5_2_load_microcode(adev);
		if (r)
			return r;

		
		if (amdgpu_emu_mode == 1)
			msleep(1000);
	}

	ip_block = amdgpu_device_ip_get_ip_block(adev, AMD_IP_BLOCK_TYPE_SDMA);
	if (!ip_block)
		return -EINVAL;

	sdma_v5_2_soft_reset(ip_block);
	
	sdma_v5_2_enable(adev, true);
	
	sdma_v5_2_ctx_switch_enable(adev, true);

	
	r = sdma_v5_2_gfx_resume(adev);
	if (r)
		return r;
	r = sdma_v5_2_rlc_resume(adev);

	return r;
}

i32 sdma_v5_2_mqd_init(struct amdgpu_device *adev, () *mqd,
			      struct amdgpu_mqd_prop *prop)
{
	struct v10_sdma_mqd *m = mqd;
	u64 wb_gpu_addr;

	m->sdmax_rlcx_rb_cntl =
		order_base_2(prop->queue_size / 4) << SDMA0_RLC0_RB_CNTL__RB_SIZE__SHIFT |
		1 << SDMA0_RLC0_RB_CNTL__RPTR_WRITEBACK_ENABLE__SHIFT |
		6 << SDMA0_RLC0_RB_CNTL__RPTR_WRITEBACK_TIMER__SHIFT |
		1 << SDMA0_RLC0_RB_CNTL__RB_PRIV__SHIFT;

	m->sdmax_rlcx_rb_base = lower_32_bits(prop->hqd_base_gpu_addr >> 8);
	m->sdmax_rlcx_rb_base_hi = upper_32_bits(prop->hqd_base_gpu_addr >> 8);

	m->sdmax_rlcx_rb_wptr_poll_cntl = RREG32(sdma_v5_2_get_reg_offset(adev, 0,
						  mmSDMA0_GFX_RB_WPTR_POLL_CNTL));

	wb_gpu_addr = prop->wptr_gpu_addr;
	m->sdmax_rlcx_rb_wptr_poll_addr_lo = lower_32_bits(wb_gpu_addr);
	m->sdmax_rlcx_rb_wptr_poll_addr_hi = upper_32_bits(wb_gpu_addr);

	wb_gpu_addr = prop->rptr_gpu_addr;
	m->sdmax_rlcx_rb_rptr_addr_lo = lower_32_bits(wb_gpu_addr);
	m->sdmax_rlcx_rb_rptr_addr_hi = upper_32_bits(wb_gpu_addr);

	m->sdmax_rlcx_ib_cntl = RREG32(sdma_v5_2_get_reg_offset(adev, 0,
							mmSDMA0_GFX_IB_CNTL));

	m->sdmax_rlcx_doorbell_offset =
		prop->doorbell_index << SDMA0_RLC0_DOORBELL_OFFSET__OFFSET__SHIFT;

	m->sdmax_rlcx_doorbell = REG_SET_FIELD(0, SDMA0_RLC0_DOORBELL, ENABLE, 1);

	return 0;
}

() sdma_v5_2_set_mqd_funcs(struct amdgpu_device *adev)
{
	adev->mqds[AMDGPU_HW_IP_DMA].mqd_size = sizeof(struct v10_sdma_mqd);
	adev->mqds[AMDGPU_HW_IP_DMA].init_mqd = sdma_v5_2_mqd_init;
}


i32 sdma_v5_2_ring_test_ring(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;
	u32 i;
	u32 index;
	i32 r;
	u32 tmp;
	u64 gpu_addr;

	tmp = 0xCAFEDEAD;

	r = amdgpu_wb_get(adev, &index);
	if (r) {
		dev_err(adev->dev, "(%d) failed to allocate wb slot\n", r);
		return r;
	}

	gpu_addr = adev->wb.gpu_addr + (index * 4);
	adev->wb.wb[index] = cpu_to_le32(tmp);

	r = amdgpu_ring_alloc(ring, 20);
	if (r) {
		drm_err(adev_to_drm(adev), "dma failed to lock ring %d (%d).\n", ring->idx, r);
		amdgpu_wb_free(adev, index);
		return r;
	}

	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_WRITE) |
			  SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR));
	amdgpu_ring_write(ring, lower_32_bits(gpu_addr));
	amdgpu_ring_write(ring, upper_32_bits(gpu_addr));
	amdgpu_ring_write(ring, SDMA_PKT_WRITE_UNTILED_DW_3_COUNT(0));
	amdgpu_ring_write(ring, 0xDEADBEEF);
	amdgpu_ring_commit(ring);

	for (i = 0; i < adev->usec_timeout; i++) {
		tmp = le32_to_cpu(adev->wb.wb[index]);
		if (tmp == 0xDEADBEEF)
			break;
		if (amdgpu_emu_mode == 1)
			msleep(1);
		else
			udelay(1);
	}

	if (i >= adev->usec_timeout)
		r = -ETIMEDOUT;

	amdgpu_wb_free(adev, index);

	return r;
}


i32 sdma_v5_2_ring_test_ib(struct amdgpu_ring *ring, i64 timeout)
{
	struct amdgpu_device *adev = ring->adev;
	struct amdgpu_ib ib;
	struct dma_fence *f = core::ptr::null_mut();
	u32 index;
	i64 r;
	u32 tmp = 0;
	u64 gpu_addr;

	tmp = 0xCAFEDEAD;
	memset(&ib, 0, sizeof(ib));

	r = amdgpu_wb_get(adev, &index);
	if (r) {
		dev_err(adev->dev, "(%ld) failed to allocate wb slot\n", r);
		return r;
	}

	gpu_addr = adev->wb.gpu_addr + (index * 4);
	adev->wb.wb[index] = cpu_to_le32(tmp);

	r = amdgpu_ib_get(adev, core::ptr::null_mut(), 256, AMDGPU_IB_POOL_DIRECT, &ib);
	if (r) {
		drm_err(adev_to_drm(adev), "failed to get ib (%ld).\n", r);
		goto err0;
	}

	ib.ptr[0] = SDMA_PKT_HEADER_OP(SDMA_OP_WRITE) |
		SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR);
	ib.ptr[1] = lower_32_bits(gpu_addr);
	ib.ptr[2] = upper_32_bits(gpu_addr);
	ib.ptr[3] = SDMA_PKT_WRITE_UNTILED_DW_3_COUNT(0);
	ib.ptr[4] = 0xDEADBEEF;
	ib.ptr[5] = SDMA_PKT_NOP_HEADER_OP(SDMA_OP_NOP);
	ib.ptr[6] = SDMA_PKT_NOP_HEADER_OP(SDMA_OP_NOP);
	ib.ptr[7] = SDMA_PKT_NOP_HEADER_OP(SDMA_OP_NOP);
	ib.length_dw = 8;

	r = amdgpu_ib_schedule(ring, 1, &ib, core::ptr::null_mut(), &f);
	if (r)
		goto err1;

	r = dma_fence_wait_timeout(f, false, timeout);
	if (r == 0) {
		drm_err(adev_to_drm(adev), "IB test timed out\n");
		r = -ETIMEDOUT;
		goto err1;
	} else if (r < 0) {
		drm_err(adev_to_drm(adev), "fence wait failed (%ld).\n", r);
		goto err1;
	}

	tmp = le32_to_cpu(adev->wb.wb[index]);

	if (tmp == 0xDEADBEEF)
		r = 0;
	else
		r = -EINVAL;

err1:
	amdgpu_ib_free(&ib, core::ptr::null_mut());
	dma_fence_put(f);
err0:
	amdgpu_wb_free(adev, index);
	return r;
}



() sdma_v5_2_vm_copy_pte(struct amdgpu_ib *ib,
				  u64 pe, u64 src,
				  u32 count)
{
	u32 bytes = count * 8;

	ib->ptr[ib->length_dw++] = SDMA_PKT_HEADER_OP(SDMA_OP_COPY) |
		SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR);
	ib->ptr[ib->length_dw++] = bytes - 1;
	ib->ptr[ib->length_dw++] = 0; 
	ib->ptr[ib->length_dw++] = lower_32_bits(src);
	ib->ptr[ib->length_dw++] = upper_32_bits(src);
	ib->ptr[ib->length_dw++] = lower_32_bits(pe);
	ib->ptr[ib->length_dw++] = upper_32_bits(pe);

}


() sdma_v5_2_vm_write_pte(struct amdgpu_ib *ib, u64 pe,
				   u64 value, u32 count,
				   u32 incr)
{
	u32 ndw = count * 2;

	ib->ptr[ib->length_dw++] = SDMA_PKT_HEADER_OP(SDMA_OP_WRITE) |
		SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_WRITE_LINEAR);
	ib->ptr[ib->length_dw++] = lower_32_bits(pe);
	ib->ptr[ib->length_dw++] = upper_32_bits(pe);
	ib->ptr[ib->length_dw++] = ndw - 1;
	for (; ndw > 0; ndw -= 2) {
		ib->ptr[ib->length_dw++] = lower_32_bits(value);
		ib->ptr[ib->length_dw++] = upper_32_bits(value);
		value += incr;
	}
}


() sdma_v5_2_vm_set_pte_pde(struct amdgpu_ib *ib,
				     u64 pe,
				     u64 addr, u32 count,
				     u32 incr, u64 flags)
{
	
	ib->ptr[ib->length_dw++] = SDMA_PKT_HEADER_OP(SDMA_OP_PTEPDE);
	ib->ptr[ib->length_dw++] = lower_32_bits(pe); 
	ib->ptr[ib->length_dw++] = upper_32_bits(pe);
	ib->ptr[ib->length_dw++] = lower_32_bits(flags); 
	ib->ptr[ib->length_dw++] = upper_32_bits(flags);
	ib->ptr[ib->length_dw++] = lower_32_bits(addr); 
	ib->ptr[ib->length_dw++] = upper_32_bits(addr);
	ib->ptr[ib->length_dw++] = incr; 
	ib->ptr[ib->length_dw++] = 0;
	ib->ptr[ib->length_dw++] = count - 1; 
}


() sdma_v5_2_ring_pad_ib(struct amdgpu_ring *ring, struct amdgpu_ib *ib)
{
	struct amdgpu_sdma_instance *sdma = amdgpu_sdma_get_instance_from_ring(ring);
	u32 pad_count;
	i32 i;

	pad_count = (-ib->length_dw) & 0x7;
	for (i = 0; i < pad_count; i++)
		if (sdma && sdma->burst_nop && (i == 0))
			ib->ptr[ib->length_dw++] =
				SDMA_PKT_HEADER_OP(SDMA_OP_NOP) |
				SDMA_PKT_NOP_HEADER_COUNT(pad_count - 1);
		else
			ib->ptr[ib->length_dw++] =
				SDMA_PKT_HEADER_OP(SDMA_OP_NOP);
}



() sdma_v5_2_ring_emit_pipeline_sync(struct amdgpu_ring *ring)
{
	u32 seq = ring->fence_drv.sync_seq;
	u64 addr = ring->fence_drv.gpu_addr;

	
	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_POLL_REGMEM) |
			  SDMA_PKT_POLL_REGMEM_HEADER_HDP_FLUSH(0) |
			  SDMA_PKT_POLL_REGMEM_HEADER_FUNC(3) | 
			  SDMA_PKT_POLL_REGMEM_HEADER_MEM_POLL(1));
	amdgpu_ring_write(ring, addr & 0xfffffffc);
	amdgpu_ring_write(ring, upper_32_bits(addr) & 0xffffffff);
	amdgpu_ring_write(ring, seq); 
	amdgpu_ring_write(ring, 0xffffffff); 
	amdgpu_ring_write(ring, SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT(0xfff) |
			  SDMA_PKT_POLL_REGMEM_DW5_INTERVAL(4)); 
}



() sdma_v5_2_ring_emit_vm_flush(struct amdgpu_ring *ring,
					 u32 vmid, u64 pd_addr)
{
	struct amdgpu_vmhub *hub = &ring->adev->vmhub[ring->vm_hub];
	u32 req = hub->vmhub_funcs->get_invalidate_req(vmid, 0);

	
	amdgpu_ring_emit_wreg(ring, hub->ctx0_ptb_addr_lo32 +
			      (hub->ctx_addr_distance * vmid),
			      lower_32_bits(pd_addr));
	amdgpu_ring_emit_wreg(ring, hub->ctx0_ptb_addr_hi32 +
			      (hub->ctx_addr_distance * vmid),
			      upper_32_bits(pd_addr));

	
	amdgpu_ring_write(ring,
			  SDMA_PKT_VM_INVALIDATION_HEADER_OP(SDMA_OP_POLL_REGMEM) |
			  SDMA_PKT_VM_INVALIDATION_HEADER_SUB_OP(SDMA_SUBOP_VM_INVALIDATION) |
			  SDMA_PKT_VM_INVALIDATION_HEADER_GFX_ENG_ID(ring->vm_inv_eng) |
			  SDMA_PKT_VM_INVALIDATION_HEADER_MM_ENG_ID(0x1f));
	amdgpu_ring_write(ring, req);
	amdgpu_ring_write(ring, 0xFFFFFFFF);
	amdgpu_ring_write(ring,
			  SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_INVALIDATEACK(1 << vmid) |
			  SDMA_PKT_VM_INVALIDATION_ADDRESSRANGEHI_ADDRESSRANGEHI(0x1F));
}

() sdma_v5_2_ring_emit_wreg(struct amdgpu_ring *ring,
				     u32 reg, u32 val)
{
	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_SRBM_WRITE) |
			  SDMA_PKT_SRBM_WRITE_HEADER_BYTE_EN(0xf));
	amdgpu_ring_write(ring, reg);
	amdgpu_ring_write(ring, val);
}

() sdma_v5_2_ring_emit_reg_wait(struct amdgpu_ring *ring, u32 reg,
					 u32 val, u32 mask)
{
	amdgpu_ring_write(ring, SDMA_PKT_HEADER_OP(SDMA_OP_POLL_REGMEM) |
			  SDMA_PKT_POLL_REGMEM_HEADER_HDP_FLUSH(0) |
			  SDMA_PKT_POLL_REGMEM_HEADER_FUNC(3)); 
	amdgpu_ring_write(ring, reg << 2);
	amdgpu_ring_write(ring, 0);
	amdgpu_ring_write(ring, val); 
	amdgpu_ring_write(ring, mask); 
	amdgpu_ring_write(ring, SDMA_PKT_POLL_REGMEM_DW5_RETRY_COUNT(0xfff) |
			  SDMA_PKT_POLL_REGMEM_DW5_INTERVAL(10));
}

() sdma_v5_2_ring_emit_reg_write_reg_wait(struct amdgpu_ring *ring,
						   u32 reg0, u32 reg1,
						   u32 ref, u32 mask)
{
	amdgpu_ring_emit_wreg(ring, reg0, ref);
	
	amdgpu_ring_emit_reg_wait(ring, reg0, 0, 0);
	amdgpu_ring_emit_reg_wait(ring, reg1, mask, mask);
}

const amdgpu_vm_pte_funcs sdma_v5_2_vm_pte_funcs = {
	.copy_pte_num_dw = 7,
	.copy_pte = sdma_v5_2_vm_copy_pte,
	.write_pte = sdma_v5_2_vm_write_pte,
	.set_pte_pde = sdma_v5_2_vm_set_pte_pde,
};

i32 sdma_v5_2_early_init(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 r;

	r = amdgpu_sdma_init_microcode(adev, 0, true);
	if (r)
		return r;

	sdma_v5_2_set_ring_funcs(adev);
	amdgpu_sdma_set_vm_pte_scheds(adev, &sdma_v5_2_vm_pte_funcs);
	sdma_v5_2_set_irq_funcs(adev);
	sdma_v5_2_set_mqd_funcs(adev);

	return 0;
}

u32 sdma_v5_2_seq_to_irq_id(i32 seq_num)
{
	switch (seq_num) {
	case 0:
		return SOC15_IH_CLIENTID_SDMA0;
	case 1:
		return SOC15_IH_CLIENTID_SDMA1;
	case 2:
		return SOC15_IH_CLIENTID_SDMA2;
	case 3:
		return SOC15_IH_CLIENTID_SDMA3_Sienna_Cichlid;
	default:
		break;
	}
	return -EINVAL;
}

u32 sdma_v5_2_seq_to_trap_id(i32 seq_num)
{
	switch (seq_num) {
	case 0:
		return SDMA0_5_0__SRCID__SDMA_TRAP;
	case 1:
		return SDMA1_5_0__SRCID__SDMA_TRAP;
	case 2:
		return SDMA2_5_0__SRCID__SDMA_TRAP;
	case 3:
		return SDMA3_5_0__SRCID__SDMA_TRAP;
	default:
		break;
	}
	return -EINVAL;
}

i32 sdma_v5_2_sw_init(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_ring *ring;
	i32 r, i;
	struct amdgpu_device *adev = ip_block->adev;
	u32 reg_count = ARRAY_SIZE(sdma_reg_list_5_2);
	u32 *ptr;

	
	for (i = 0; i < adev->sdma.num_instances; i++) {
		r = amdgpu_irq_add_id(adev, sdma_v5_2_seq_to_irq_id(i),
				      sdma_v5_2_seq_to_trap_id(i),
				      &adev->sdma.trap_irq);
		if (r)
			return r;
	}

	for (i = 0; i < adev->sdma.num_instances; i++) {
		mutex_init(&adev->sdma.instance[i].engine_reset_mutex);
		adev->sdma.instance[i].funcs = &sdma_v5_2_sdma_funcs;
		ring = &adev->sdma.instance[i].ring;
		ring->ring_obj = core::ptr::null_mut();
		ring->use_doorbell = true;
		ring->me = i;

		drm_info(adev_to_drm(adev), "use_doorbell being set to: [%s]\n",
			 ring->use_doorbell?"true":"false");

		ring->doorbell_index =
			(adev->doorbell_index.sdma_engine[i] << 1); //get DWORD offset

		ring->vm_hub = AMDGPU_GFXHUB(0);
		sprintf(ring->name, "sdma%d", i);
		r = amdgpu_ring_init(adev, ring, 1024, &adev->sdma.trap_irq,
				     AMDGPU_SDMA_IRQ_INSTANCE0 + i,
				     AMDGPU_RING_PRIO_DEFAULT, core::ptr::null_mut());
		if (r)
			return r;
	}

	adev->sdma.supported_reset =
		amdgpu_get_soft_full_reset_mask(&adev->sdma.instance[0].ring);
	if (!amdgpu_sriov_vf(adev) &&
	    !adev->debug_disable_gpu_ring_reset)
		adev->sdma.supported_reset |= AMDGPU_RESET_TYPE_PER_QUEUE;

	
	ptr = kcalloc(adev->sdma.num_instances * reg_count, sizeof(u32), GFP_KERNEL);
	if (ptr)
		adev->sdma.ip_dump = ptr;
	else
		DRM_ERROR("Failed to allocated memory for SDMA IP Dump\n");

	r = amdgpu_sdma_sysfs_reset_mask_init(adev);
	if (r)
		return r;

	return r;
}

i32 sdma_v5_2_sw_fini(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 i;

	for (i = 0; i < adev->sdma.num_instances; i++)
		amdgpu_ring_fini(&adev->sdma.instance[i].ring);

	amdgpu_sdma_sysfs_reset_mask_fini(adev);
	amdgpu_sdma_destroy_inst_ctx(adev, true);

	kfree(adev->sdma.ip_dump);

	return 0;
}

i32 sdma_v5_2_hw_init(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 r;

	r = sdma_v5_2_start(adev);
	if (r)
		return r;
	sdma_v5_2_set_buffer_funcs(adev);

	return 0;
}

i32 sdma_v5_2_hw_fini(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;

	if (amdgpu_sriov_vf(adev))
		return 0;

	sdma_v5_2_ctx_switch_enable(adev, false);
	sdma_v5_2_enable(adev, false);

	return 0;
}

i32 sdma_v5_2_suspend(struct amdgpu_ip_block *ip_block)
{
	return sdma_v5_2_hw_fini(ip_block);
}

i32 sdma_v5_2_resume(struct amdgpu_ip_block *ip_block)
{
	return sdma_v5_2_hw_init(ip_block);
}

bool sdma_v5_2_is_idle(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;
	u32 i;

	for (i = 0; i < adev->sdma.num_instances; i++) {
		u32 tmp = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_STATUS_REG));

		if (!(tmp & SDMA0_STATUS_REG__IDLE_MASK))
			return false;
	}

	return true;
}

i32 sdma_v5_2_wait_for_idle(struct amdgpu_ip_block *ip_block)
{
	u32 i;
	u32 sdma0, sdma1, sdma2, sdma3;
	struct amdgpu_device *adev = ip_block->adev;

	for (i = 0; i < adev->usec_timeout; i++) {
		sdma0 = RREG32(sdma_v5_2_get_reg_offset(adev, 0, mmSDMA0_STATUS_REG));
		sdma1 = RREG32(sdma_v5_2_get_reg_offset(adev, 1, mmSDMA0_STATUS_REG));
		sdma2 = RREG32(sdma_v5_2_get_reg_offset(adev, 2, mmSDMA0_STATUS_REG));
		sdma3 = RREG32(sdma_v5_2_get_reg_offset(adev, 3, mmSDMA0_STATUS_REG));

		if (sdma0 & sdma1 & sdma2 & sdma3 & SDMA0_STATUS_REG__IDLE_MASK)
			return 0;
		udelay(1);
	}
	return -ETIMEDOUT;
}

i32 sdma_v5_2_reset_queue(struct amdgpu_ring *ring,
				 u32 i32 vmid,
				 struct amdgpu_fence *timedout_fence)
{
	struct amdgpu_device *adev = ring->adev;
	i32 r;

	if (ring->me >= adev->sdma.num_instances) {
		dev_err(adev->dev, "sdma instance not found\n");
		return -EINVAL;
	}

	amdgpu_ring_reset_helper_begin(ring, timedout_fence);

	amdgpu_amdkfd_suspend(adev, true);
	r = amdgpu_sdma_reset_engine(adev, ring->me, true);
	amdgpu_amdkfd_resume(adev, true);
	if (r)
		return r;

	return amdgpu_ring_reset_helper_end(ring, timedout_fence);
}

i32 sdma_v5_2_stop_queue(struct amdgpu_ring *ring)
{
	u32 f32_cntl, freeze, cntl, stat1_reg;
	struct amdgpu_device *adev = ring->adev;
	i32 i, j, r = 0;

	if (amdgpu_sriov_vf(adev))
		return -EINVAL;

	i = ring->me;
	amdgpu_gfx_rlc_enter_safe_mode(adev, 0);

	
	sdma_v5_2_gfx_stop(adev, 1 << i);

	
	freeze = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_FREEZE));
	freeze = REG_SET_FIELD(freeze, SDMA0_FREEZE, FREEZE, 1);
	WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_FREEZE), freeze);

	for (j = 0; j < adev->usec_timeout; j++) {
		freeze = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_FREEZE));

		if (REG_GET_FIELD(freeze, SDMA0_FREEZE, FROZEN) & 1)
			break;
		udelay(1);
	}


	if (j == adev->usec_timeout) {
		stat1_reg = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_STATUS1_REG));
		if ((stat1_reg & 0x3FF) != 0x3FF) {
			DRM_ERROR("cannot soft reset as sdma not idle\n");
			r = -ETIMEDOUT;
			goto err0;
		}
	}

	f32_cntl = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_F32_CNTL));
	f32_cntl = REG_SET_FIELD(f32_cntl, SDMA0_F32_CNTL, HALT, 1);
	WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_F32_CNTL), f32_cntl);

	cntl = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CNTL));
	cntl = REG_SET_FIELD(cntl, SDMA0_CNTL, UTC_L1_ENABLE, 0);
	WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CNTL), cntl);

err0:
	amdgpu_gfx_rlc_exit_safe_mode(adev, 0);
	return r;
}

i32 sdma_v5_2_restore_queue(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;
	u32 inst_id = ring->me;
	u32 freeze;
	i32 r;

	amdgpu_gfx_rlc_enter_safe_mode(adev, 0);
	
	freeze = RREG32(sdma_v5_2_get_reg_offset(adev, inst_id, mmSDMA0_FREEZE));
	freeze = REG_SET_FIELD(freeze, SDMA0_FREEZE, FREEZE, 0);
	WREG32(sdma_v5_2_get_reg_offset(adev, inst_id, mmSDMA0_FREEZE), freeze);

	r = sdma_v5_2_gfx_resume_instance(adev, inst_id, true);

	amdgpu_gfx_rlc_exit_safe_mode(adev, 0);

	return r;
}

i32 sdma_v5_2_ring_preempt_ib(struct amdgpu_ring *ring)
{
	i32 i, r = 0;
	struct amdgpu_device *adev = ring->adev;
	u32 index = 0;
	u64 sdma_gfx_preempt;

	amdgpu_sdma_get_index_from_ring(ring, &index);
	sdma_gfx_preempt =
		sdma_v5_2_get_reg_offset(adev, index, mmSDMA0_GFX_PREEMPT);

	
	amdgpu_ring_set_preempt_cond_exec(ring, false);

	
	ring->trail_seq += 1;
	amdgpu_ring_alloc(ring, 10);
	sdma_v5_2_ring_emit_fence(ring, ring->trail_fence_gpu_addr,
				  ring->trail_seq, 0);
	amdgpu_ring_commit(ring);

	
	WREG32(sdma_gfx_preempt, 1);

	
	for (i = 0; i < adev->usec_timeout; i++) {
		if (ring->trail_seq ==
		    le32_to_cpu(*(ring->trail_fence_cpu_addr)))
			break;
		udelay(1);
	}

	if (i >= adev->usec_timeout) {
		r = -EINVAL;
		DRM_ERROR("ring %d failed to be preempted\n", ring->idx);
	}

	
	WREG32(sdma_gfx_preempt, 0);

	
	amdgpu_ring_set_preempt_cond_exec(ring, true);
	return r;
}

i32 sdma_v5_2_set_trap_irq_state(struct amdgpu_device *adev,
					struct amdgpu_irq_src *source,
					u32 type,
					enum amdgpu_interrupt_state state)
{
	u32 sdma_cntl;
	u32 reg_offset = sdma_v5_2_get_reg_offset(adev, type, mmSDMA0_CNTL);

	if (!amdgpu_sriov_vf(adev)) {
		sdma_cntl = RREG32(reg_offset);
		sdma_cntl = REG_SET_FIELD(sdma_cntl, SDMA0_CNTL, TRAP_ENABLE,
			       state == AMDGPU_IRQ_STATE_ENABLE ? 1 : 0);
		WREG32(reg_offset, sdma_cntl);
	}

	return 0;
}

i32 sdma_v5_2_process_trap_irq(struct amdgpu_device *adev,
				      struct amdgpu_irq_src *source,
				      struct amdgpu_iv_entry *entry)
{
	DRM_DEBUG("IH: SDMA trap\n");

	if (drm_WARN_ON_ONCE(&adev->ddev,
			     adev->enable_mes &&
			     (entry->src_data[0] & AMDGPU_FENCE_MES_QUEUE_FLAG)))
		return 0;

	switch (entry->client_id) {
	case SOC15_IH_CLIENTID_SDMA0:
		switch (entry->ring_id) {
		case 0:
			amdgpu_fence_process(&adev->sdma.instance[0].ring);
			break;
		case 1:
			
			break;
		case 2:
			
			break;
		case 3:
			
			break;
		}
		break;
	case SOC15_IH_CLIENTID_SDMA1:
		switch (entry->ring_id) {
		case 0:
			amdgpu_fence_process(&adev->sdma.instance[1].ring);
			break;
		case 1:
			
			break;
		case 2:
			
			break;
		case 3:
			
			break;
		}
		break;
	case SOC15_IH_CLIENTID_SDMA2:
		switch (entry->ring_id) {
		case 0:
			amdgpu_fence_process(&adev->sdma.instance[2].ring);
			break;
		case 1:
			
			break;
		case 2:
			
			break;
		case 3:
			
			break;
		}
		break;
	case SOC15_IH_CLIENTID_SDMA3_Sienna_Cichlid:
		switch (entry->ring_id) {
		case 0:
			amdgpu_fence_process(&adev->sdma.instance[3].ring);
			break;
		case 1:
			
			break;
		case 2:
			
			break;
		case 3:
			
			break;
		}
		break;
	}
	return 0;
}

i32 sdma_v5_2_process_illegal_inst_irq(struct amdgpu_device *adev,
					      struct amdgpu_irq_src *source,
					      struct amdgpu_iv_entry *entry)
{
	return 0;
}

bool sdma_v5_2_firmware_mgcg_support(struct amdgpu_device *adev,
						     i32 i)
{
	switch (amdgpu_ip_version(adev, SDMA0_HWIP, 0)) {
	case IP_VERSION(5, 2, 1):
		if (adev->sdma.instance[i].fw_version < 70)
			return false;
		break;
	case IP_VERSION(5, 2, 3):
		if (adev->sdma.instance[i].fw_version < 47)
			return false;
		break;
	case IP_VERSION(5, 2, 7):
		if (adev->sdma.instance[i].fw_version < 9)
			return false;
		break;
	default:
		return true;
	}

	return true;

}

() sdma_v5_2_update_medium_grain_clock_gating(struct amdgpu_device *adev,
						       bool enable)
{
	u32 data, def;
	i32 i;

	for (i = 0; i < adev->sdma.num_instances; i++) {

		if (!sdma_v5_2_firmware_mgcg_support(adev, i))
			adev->cg_flags &= ~AMD_CG_SUPPORT_SDMA_MGCG;

		if (enable && (adev->cg_flags & AMD_CG_SUPPORT_SDMA_MGCG)) {
			
			def = data = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CLK_CTRL));
			data &= ~(SDMA0_CLK_CTRL__SOFT_OVERRIDE4_MASK |
				  SDMA0_CLK_CTRL__SOFT_OVERRIDE3_MASK |
				  SDMA0_CLK_CTRL__SOFT_OVERRIDE2_MASK |
				  SDMA0_CLK_CTRL__SOFT_OVERRIDE1_MASK |
				  SDMA0_CLK_CTRL__SOFT_OVERRIDE0_MASK |
				  SDMA0_CLK_CTRL__SOFT_OVERRIDER_REG_MASK);
			if (def != data)
				WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CLK_CTRL), data);
		} else {
			
			def = data = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CLK_CTRL));
			data |= (SDMA0_CLK_CTRL__SOFT_OVERRIDE4_MASK |
				 SDMA0_CLK_CTRL__SOFT_OVERRIDE3_MASK |
				 SDMA0_CLK_CTRL__SOFT_OVERRIDE2_MASK |
				 SDMA0_CLK_CTRL__SOFT_OVERRIDE1_MASK |
				 SDMA0_CLK_CTRL__SOFT_OVERRIDE0_MASK |
				 SDMA0_CLK_CTRL__SOFT_OVERRIDER_REG_MASK);
			if (def != data)
				WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_CLK_CTRL), data);
		}
	}
}

() sdma_v5_2_update_medium_grain_light_sleep(struct amdgpu_device *adev,
						      bool enable)
{
	u32 data, def;
	i32 i;

	for (i = 0; i < adev->sdma.num_instances; i++) {
		if (adev->sdma.instance[i].fw_version < 70 &&
		    amdgpu_ip_version(adev, SDMA0_HWIP, 0) ==
			    IP_VERSION(5, 2, 1))
			adev->cg_flags &= ~AMD_CG_SUPPORT_SDMA_LS;

		if (enable && (adev->cg_flags & AMD_CG_SUPPORT_SDMA_LS)) {
			
			def = data = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_POWER_CNTL));
			data |= SDMA0_POWER_CNTL__MEM_POWER_OVERRIDE_MASK;
			if (def != data)
				WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_POWER_CNTL), data);

		} else {
			
			def = data = RREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_POWER_CNTL));
			data &= ~SDMA0_POWER_CNTL__MEM_POWER_OVERRIDE_MASK;
			if (def != data)
				WREG32(sdma_v5_2_get_reg_offset(adev, i, mmSDMA0_POWER_CNTL), data);

		}
	}
}

i32 sdma_v5_2_set_clockgating_state(struct amdgpu_ip_block *ip_block,
					   enum amd_clockgating_state state)
{
	struct amdgpu_device *adev = ip_block->adev;

	if (amdgpu_sriov_vf(adev))
		return 0;

	switch (amdgpu_ip_version(adev, SDMA0_HWIP, 0)) {
	case IP_VERSION(5, 2, 0):
	case IP_VERSION(5, 2, 2):
	case IP_VERSION(5, 2, 1):
	case IP_VERSION(5, 2, 4):
	case IP_VERSION(5, 2, 5):
	case IP_VERSION(5, 2, 6):
	case IP_VERSION(5, 2, 3):
	case IP_VERSION(5, 2, 7):
		sdma_v5_2_update_medium_grain_clock_gating(adev,
				state == AMD_CG_STATE_GATE);
		sdma_v5_2_update_medium_grain_light_sleep(adev,
				state == AMD_CG_STATE_GATE);
		break;
	default:
		break;
	}

	return 0;
}

i32 sdma_v5_2_set_powergating_state(struct amdgpu_ip_block *ip_block,
					  enum amd_powergating_state state)
{
	return 0;
}

() sdma_v5_2_get_clockgating_state(struct amdgpu_ip_block *ip_block, u64 *flags)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 data;

	if (amdgpu_sriov_vf(adev))
		*flags = 0;

	
	data = RREG32(sdma_v5_2_get_reg_offset(adev, 0, mmSDMA0_CLK_CTRL));
	if (!(data & SDMA0_CLK_CTRL__CGCG_EN_OVERRIDE_MASK))
		*flags |= AMD_CG_SUPPORT_SDMA_MGCG;

	
	data = RREG32_KIQ(sdma_v5_2_get_reg_offset(adev, 0, mmSDMA0_POWER_CNTL));
	if (data & SDMA0_POWER_CNTL__MEM_POWER_OVERRIDE_MASK)
		*flags |= AMD_CG_SUPPORT_SDMA_LS;
}

() sdma_v5_2_ring_begin_use(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;

	
	amdgpu_gfx_off_ctrl(adev, false);
}

() sdma_v5_2_ring_end_use(struct amdgpu_ring *ring)
{
	struct amdgpu_device *adev = ring->adev;

	
	amdgpu_gfx_off_ctrl(adev, true);
}

() sdma_v5_2_print_ip_state(struct amdgpu_ip_block *ip_block, struct drm_printer *p)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 i, j;
	u32 reg_count = ARRAY_SIZE(sdma_reg_list_5_2);
	u32 instance_offset;

	if (!adev->sdma.ip_dump)
		return;

	drm_printf(p, "num_instances:%d\n", adev->sdma.num_instances);
	for (i = 0; i < adev->sdma.num_instances; i++) {
		instance_offset = i * reg_count;
		drm_printf(p, "\nInstance:%d\n", i);

		for (j = 0; j < reg_count; j++)
			drm_printf(p, "%-50s \t 0x%08x\n", sdma_reg_list_5_2[j].reg_name,
				   adev->sdma.ip_dump[instance_offset + j]);
	}
}

() sdma_v5_2_dump_ip_state(struct amdgpu_ip_block *ip_block)
{
	struct amdgpu_device *adev = ip_block->adev;
	i32 i, j;
	u32 instance_offset;
	u32 reg_count = ARRAY_SIZE(sdma_reg_list_5_2);

	if (!adev->sdma.ip_dump)
		return;

	amdgpu_gfx_off_ctrl(adev, false);
	for (i = 0; i < adev->sdma.num_instances; i++) {
		instance_offset = i * reg_count;
		for (j = 0; j < reg_count; j++)
			adev->sdma.ip_dump[instance_offset + j] =
				RREG32(sdma_v5_2_get_reg_offset(adev, i,
				       sdma_reg_list_5_2[j].reg_offset));
	}
	amdgpu_gfx_off_ctrl(adev, true);
}

const amd_ip_funcs sdma_v5_2_ip_funcs = {
	.name = "sdma_v5_2",
	.early_init = sdma_v5_2_early_init,
	.sw_init = sdma_v5_2_sw_init,
	.sw_fini = sdma_v5_2_sw_fini,
	.hw_init = sdma_v5_2_hw_init,
	.hw_fini = sdma_v5_2_hw_fini,
	.suspend = sdma_v5_2_suspend,
	.resume = sdma_v5_2_resume,
	.is_idle = sdma_v5_2_is_idle,
	.wait_for_idle = sdma_v5_2_wait_for_idle,
	.soft_reset = sdma_v5_2_soft_reset,
	.set_clockgating_state = sdma_v5_2_set_clockgating_state,
	.set_powergating_state = sdma_v5_2_set_powergating_state,
	.get_clockgating_state = sdma_v5_2_get_clockgating_state,
	.dump_ip_state = sdma_v5_2_dump_ip_state,
	.print_ip_state = sdma_v5_2_print_ip_state,
};

const amdgpu_ring_funcs sdma_v5_2_ring_funcs = {
	.type = AMDGPU_RING_TYPE_SDMA,
	.align_mask = 0xf,
	.nop = SDMA_PKT_NOP_HEADER_OP(SDMA_OP_NOP),
	.support_64bit_ptrs = true,
	.secure_submission_supported = true,
	.get_rptr = sdma_v5_2_ring_get_rptr,
	.get_wptr = sdma_v5_2_ring_get_wptr,
	.set_wptr = sdma_v5_2_ring_set_wptr,
	.emit_frame_size =
		5 + 
		6 + 
		3 + 
		6 + 
		
		SOC15_FLUSH_GPU_TLB_NUM_WREG * 3 +
		SOC15_FLUSH_GPU_TLB_NUM_REG_WAIT * 6 +
		10 + 10 + 10, 
	.emit_ib_size = 7 + 6, 
	.emit_ib = sdma_v5_2_ring_emit_ib,
	.emit_mem_sync = sdma_v5_2_ring_emit_mem_sync,
	.emit_fence = sdma_v5_2_ring_emit_fence,
	.emit_pipeline_sync = sdma_v5_2_ring_emit_pipeline_sync,
	.emit_vm_flush = sdma_v5_2_ring_emit_vm_flush,
	.emit_hdp_flush = sdma_v5_2_ring_emit_hdp_flush,
	.test_ring = sdma_v5_2_ring_test_ring,
	.test_ib = sdma_v5_2_ring_test_ib,
	.insert_nop = sdma_v5_2_ring_insert_nop,
	.pad_ib = sdma_v5_2_ring_pad_ib,
	.begin_use = sdma_v5_2_ring_begin_use,
	.end_use = sdma_v5_2_ring_end_use,
	.emit_wreg = sdma_v5_2_ring_emit_wreg,
	.emit_reg_wait = sdma_v5_2_ring_emit_reg_wait,
	.emit_reg_write_reg_wait = sdma_v5_2_ring_emit_reg_write_reg_wait,
	.init_cond_exec = sdma_v5_2_ring_init_cond_exec,
	.preempt_ib = sdma_v5_2_ring_preempt_ib,
	.reset = sdma_v5_2_reset_queue,
};

() sdma_v5_2_set_ring_funcs(struct amdgpu_device *adev)
{
	i32 i;

	for (i = 0; i < adev->sdma.num_instances; i++) {
		adev->sdma.instance[i].ring.funcs = &sdma_v5_2_ring_funcs;
		adev->sdma.instance[i].ring.me = i;
	}
}

const amdgpu_irq_src_funcs sdma_v5_2_trap_irq_funcs = {
	.set = sdma_v5_2_set_trap_irq_state,
	.process = sdma_v5_2_process_trap_irq,
};

const amdgpu_irq_src_funcs sdma_v5_2_illegal_inst_irq_funcs = {
	.process = sdma_v5_2_process_illegal_inst_irq,
};

() sdma_v5_2_set_irq_funcs(struct amdgpu_device *adev)
{
	adev->sdma.trap_irq.num_types = AMDGPU_SDMA_IRQ_INSTANCE0 +
					adev->sdma.num_instances;
	adev->sdma.trap_irq.funcs = &sdma_v5_2_trap_irq_funcs;
	adev->sdma.illegal_inst_irq.funcs = &sdma_v5_2_illegal_inst_irq_funcs;
}


() sdma_v5_2_emit_copy_buffer(struct amdgpu_ib *ib,
				       u64 src_offset,
				       u64 dst_offset,
				       u32 byte_count,
				       u32 copy_flags)
{
	ib->ptr[ib->length_dw++] = SDMA_PKT_HEADER_OP(SDMA_OP_COPY) |
		SDMA_PKT_HEADER_SUB_OP(SDMA_SUBOP_COPY_LINEAR) |
		SDMA_PKT_COPY_LINEAR_HEADER_TMZ((copy_flags & AMDGPU_COPY_FLAGS_TMZ) ? 1 : 0);
	ib->ptr[ib->length_dw++] = byte_count - 1;
	ib->ptr[ib->length_dw++] = 0; 
	ib->ptr[ib->length_dw++] = lower_32_bits(src_offset);
	ib->ptr[ib->length_dw++] = upper_32_bits(src_offset);
	ib->ptr[ib->length_dw++] = lower_32_bits(dst_offset);
	ib->ptr[ib->length_dw++] = upper_32_bits(dst_offset);
}


() sdma_v5_2_emit_fill_buffer(struct amdgpu_ib *ib,
				       u32 src_data,
				       u64 dst_offset,
				       u32 byte_count)
{
	ib->ptr[ib->length_dw++] = SDMA_PKT_HEADER_OP(SDMA_OP_CONST_FILL);
	ib->ptr[ib->length_dw++] = lower_32_bits(dst_offset);
	ib->ptr[ib->length_dw++] = upper_32_bits(dst_offset);
	ib->ptr[ib->length_dw++] = src_data;
	ib->ptr[ib->length_dw++] = byte_count - 1;
}

const amdgpu_buffer_funcs sdma_v5_2_buffer_funcs = {
	.copy_max_bytes = 1 << 30,
	.copy_num_dw = 7,
	.emit_copy_buffer = sdma_v5_2_emit_copy_buffer,

	.fill_max_bytes = 1 << 30, 
	.fill_num_dw = 5,
	.emit_fill_buffer = sdma_v5_2_emit_fill_buffer,
};

() sdma_v5_2_set_buffer_funcs(struct amdgpu_device *adev)
{
	amdgpu_sdma_set_buffer_funcs_scheds(adev, &sdma_v5_2_buffer_funcs);
}

const struct amdgpu_ip_block_version sdma_v5_2_ip_block = {
	.type = AMD_IP_BLOCK_TYPE_SDMA,
	.major = 5,
	.minor = 2,
	.rev = 0,
	.funcs = &sdma_v5_2_ip_funcs,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
