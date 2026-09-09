// SPDX-License-Identifier: GPL-2.0-only
/*
 * KUnit tests for negotiated TTLM (TID-To-Link Mapping) parsing
 *
 * Copyright (C) 2026 Michael Bommarito <michael.bommarito@gmail.com>
 */

// External kernel/KUnit declarations and constants are supplied by the surrounding build.
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");

/*
 * Build a negotiated TTLM element in caller-supplied buffer.
 *
 * @buf:       destination buffer (must be at least elem_size bytes)
 * @elem_size: sizeof(ttlm_elem) + 1 (presence byte) + npresent * bm_size
 * @presence:  link_map_presence bitmask; each set bit => one map follows
 * @bm_size:   bytes per map (1 or 2); 2 => LINK_MAP_SIZE bit clear
 * @maps:      array of npresent u16 maps, one per set bit in presence
 *
 * Control field encodes direction=BOTH; no switch-time, no expected-dur,
 * no DEF_LINK_MAP.  LINK_MAP_SIZE bit is set iff bm_size==1.
 *
 * Returns pointer to the ieee80211_ttlm_elem at buf.
 */
unsafe fn build_neg_ttlm_elem(
    buf: *mut u8,
    elem_size: usize,
    presence: u8,
    bm_size: u8,
    maps: *const u16,
) -> *const ieee80211_ttlm_elem {
    let t = buf as *mut ieee80211_ttlm_elem;
    let mut control: u8;
    let mut pos: *mut u8;

    core::ptr::write_bytes(buf, 0, elem_size);

    control = IEEE80211_TTLM_DIRECTION_BOTH as u8; // bits [1:0] = 2
    if bm_size == 1 {
        control |= IEEE80211_TTLM_CONTROL_LINK_MAP_SIZE as u8;
    }

    (*t).control = control;

    pos = (*t).optional.as_mut_ptr();
    *pos = presence;
    pos = pos.add(1);

    let mut i = 0usize;
    for tid in 0..IEEE80211_TTLM_NUM_TIDS {
        if (presence & (1u8 << tid)) == 0 {
            continue;
        }
        if bm_size == 1 {
            *pos = *maps.add(i) as u8;
        } else {
            let value = (*maps.add(i)).to_le_bytes();
            *pos = value[0];
            *pos.add(1) = value[1];
        }
        pos = pos.add(bm_size as usize);
        i += 1;
    }

    t
}

/*
 * sparse_presence_no_oob_read - BIT(0)|BIT(7) presence, bm_size=2
 *
 * Only TID 0 and TID 7 have maps; TIDs 1-6 are absent.  Element length
 * is exactly 6 bytes (1 control + 1 presence + 2 * 2-byte maps).
 *
 * Pre-fix the parser advanced pos by bm_size AFTER the switch() block
 * (i.e. unconditionally for every TID), so when processing TID 7 it
 * had already advanced 6 * bm_size = 12 bytes past the presence byte
 * for the absent TIDs before reading the TID-7 map - 14 bytes past the
 * end of the 2-byte TID-7 map.  Under KASAN that is a slab-out-of-bounds.
 *
 * After the fix pos is advanced only inside the presence-bit branch so
 * the cursor lands exactly at end-of-element after processing TID 7.
 */
unsafe fn sparse_presence_no_oob_read(test: *mut kunit) {
    /*
     * presence = BIT(0)|BIT(7): 2 maps present.
     * elem_size = sizeof(ttlm_elem) + 1 (presence) + 2*2 (maps) = 6.
     */
    let presence: u8 = (1 << 0) | (1 << 7);
    let bm_size: u8 = 2;
    let npresent: usize = 2;
    let elem_size = core::mem::size_of::<ieee80211_ttlm_elem>() + 1 + npresent * bm_size as usize;
    /* Allocate exact-size buffer so a pre-fix OOB read walks into the
     * KASAN red zone immediately after the allocation. */
    let buf = kunit_kzalloc(test, elem_size, GFP_KERNEL) as *mut u8;
    let ttlm: *const ieee80211_ttlm_elem;
    let mut neg_ttlm: ieee80211_neg_ttlm = core::mem::zeroed();
    /* Non-zero maps so the parser does not reject with -EINVAL. */
    let maps: [u16; 2] = [0x0001, 0x0001];
    let mut direction: u8 = 0;
    let ret: i32;

    KUNIT_ASSERT_NOT_NULL(test, buf);

    ttlm = build_neg_ttlm_elem(buf, elem_size, presence, bm_size, maps.as_ptr());

    /*
     * Pass NULL for sdata: the only sdata dereference in this code path
     * is inside mlme_dbg() on error returns, which are guarded by
     * MAC80211_MLME_DEBUG == 0 in non-debug builds and by the dead-code
     * eliminator in KUnit builds.  The success path does not touch sdata.
     */
    ret = ieee80211_parse_neg_ttlm(core::ptr::null_mut(), ttlm, &mut neg_ttlm, &mut direction);

    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, direction as i32, IEEE80211_TTLM_DIRECTION_BOTH);
    /* TID 0: map present */
    KUNIT_EXPECT_EQ(test, neg_ttlm.downlink[0] as i32, 0x0001);
    KUNIT_EXPECT_EQ(test, neg_ttlm.uplink[0] as i32, 0x0001);
    /* TID 3: absent => map should be 0 */
    KUNIT_EXPECT_EQ(test, neg_ttlm.downlink[3] as i32, 0);
    KUNIT_EXPECT_EQ(test, neg_ttlm.uplink[3] as i32, 0);
    /* TID 7: map present */
    KUNIT_EXPECT_EQ(test, neg_ttlm.downlink[7] as i32, 0x0001);
    KUNIT_EXPECT_EQ(test, neg_ttlm.uplink[7] as i32, 0x0001);
}

/*
 * dense_presence_baseline - presence=0xff (all 8 TIDs), bm_size=2
 *
 * Every TID has a map; this is the dense layout the parser handled
 * correctly even before the fix.  Confirms the cursor-advance fix
 * does not regress the already-correct path.
 */
unsafe fn dense_presence_baseline(test: *mut kunit) {
    let presence: u8 = 0xff;
    let bm_size: u8 = 2;
    let npresent: usize = 8;
    let elem_size = core::mem::size_of::<ieee80211_ttlm_elem>() + 1 + npresent * bm_size as usize;
    let buf = kunit_kzalloc(test, elem_size, GFP_KERNEL) as *mut u8;
    let ttlm: *const ieee80211_ttlm_elem;
    let mut neg_ttlm: ieee80211_neg_ttlm = core::mem::zeroed();
    let maps: [u16; 8] = [0x0003, 0x0003, 0x0003, 0x0003,
                           0x0003, 0x0003, 0x0003, 0x0003];
    let mut direction: u8 = 0;
    let ret: i32;

    KUNIT_ASSERT_NOT_NULL(test, buf);

    ttlm = build_neg_ttlm_elem(buf, elem_size, presence, bm_size, maps.as_ptr());

    ret = ieee80211_parse_neg_ttlm(core::ptr::null_mut(), ttlm, &mut neg_ttlm, &mut direction);

    KUNIT_EXPECT_EQ(test, ret, 0);
    KUNIT_EXPECT_EQ(test, direction as i32, IEEE80211_TTLM_DIRECTION_BOTH);
    /* All TIDs present: every downlink/uplink entry must be 0x0003. */
    for tid in 0..IEEE80211_TTLM_NUM_TIDS {
        KUNIT_EXPECT_EQ(test, neg_ttlm.downlink[tid] as i32, 0x0003);
        KUNIT_EXPECT_EQ(test, neg_ttlm.uplink[tid] as i32, 0x0003);
    }
}

static mut mac80211_ttlm_test_cases: [kunit_case; 3] = [
    KUNIT_CASE(sparse_presence_no_oob_read),
    KUNIT_CASE(dense_presence_baseline),
    KUNIT_CASE_NONE,
];

static mut mac80211_ttlm: kunit_suite = kunit_suite {
    name: "mac80211-ttlm",
    test_cases: mac80211_ttlm_test_cases.as_mut_ptr(),
};

// kunit_test_suite(mac80211_ttlm);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
