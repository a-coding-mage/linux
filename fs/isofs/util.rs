// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/isofs/util.c
 */

// Dependency intent: the C source includes <linux/time.h> and "isofs.h".

/*
 * We have to convert from a MM/DD/YY format to the Unix ctime format.
 * We have to take into account leap years and all of that good stuff.
 * Unfortunately, the kernel does not have the information on hand to
 * take into account daylight savings time, but it shouldn't matter.
 * The time stored should be localtime (with or without DST in effect),
 * and the timezone offset should hold the offset required to get back
 * to GMT.  Thus  we should always be correct.
 */

extern "C" {
    fn mktime64(year: i32, mon: i32, day: i32, hour: i32, min: i32, sec: i32) -> i64;
}

pub unsafe fn iso_date(p: *mut u8, flags: i32) -> timespec64 {
    let mut year: i32;
    let mut month: i32;
    let mut day: i32;
    let mut hour: i32;
    let mut minute: i32;
    let mut second: i32;
    let mut tz: i32;
    let mut ts: timespec64;

    if flags & ISO_DATE_LONG_FORM != 0 {
        year = (*p.add(0) as i32 - b'0' as i32) * 1000
            + (*p.add(1) as i32 - b'0' as i32) * 100
            + (*p.add(2) as i32 - b'0' as i32) * 10
            + (*p.add(3) as i32 - b'0' as i32) - 1900;
        month = (*p.add(4) as i32 - b'0' as i32) * 10 + (*p.add(5) as i32 - b'0' as i32);
        day = (*p.add(6) as i32 - b'0' as i32) * 10 + (*p.add(7) as i32 - b'0' as i32);
        hour = (*p.add(8) as i32 - b'0' as i32) * 10 + (*p.add(9) as i32 - b'0' as i32);
        minute = (*p.add(10) as i32 - b'0' as i32) * 10 + (*p.add(11) as i32 - b'0' as i32);
        second = (*p.add(12) as i32 - b'0' as i32) * 10 + (*p.add(13) as i32 - b'0' as i32);
        ts.tv_nsec = ((*p.add(14) as i32 - b'0' as i32) * 10
            + (*p.add(15) as i32 - b'0' as i32)) * 10000000;
        tz = *p.add(16) as i32;
    } else {
        year = *p.add(0) as i32;
        month = *p.add(1) as i32;
        day = *p.add(2) as i32;
        hour = *p.add(3) as i32;
        minute = *p.add(4) as i32;
        second = *p.add(5) as i32;
        ts.tv_nsec = 0;
        /* High sierra has no time zone */
        tz = if flags & ISO_DATE_HIGH_SIERRA != 0 { 0 } else { *p.add(6) as i32 };
    }

    if year < 0 {
        ts.tv_sec = 0;
    } else {
        ts.tv_sec = mktime64(year + 1900, month, day, hour, minute, second);

        /* sign extend */
        if tz & 0x80 != 0 {
            tz |= -1 << 8;
        }

        /*
         * The timezone offset is unreliable on some disks,
         * so we make a sanity check.  In no case is it ever
         * more than 13 hours from GMT, which is 52*15min.
         * The time is always stored in localtime with the
         * timezone offset being what get added to GMT to
         * get to localtime.  Thus we need to subtract the offset
         * to get to true GMT, which is what we store the time
         * as internally.  On the local system, the user may set
         * their timezone any way they wish, of course, so GMT
         * gets converted back to localtime on the receiving
         * system.
         *
         * NOTE: mkisofs in versions prior to mkisofs-1.10 had
         * the sign wrong on the timezone offset.  This has now
         * been corrected there too, but if you are getting screwy
         * results this may be the explanation.  If enough people
         * complain, a user configuration option could be added
         * to add the timezone offset in with the wrong sign
         * for 'compatibility' with older discs, but I cannot see how
         * it will matter that much.
         *
         * Thanks to kuhlmav@elec.canterbury.ac.nz (Volker Kuhlmann)
         * for pointing out the sign error.
         */
        if -52 <= tz && tz <= 52 {
            ts.tv_sec -= tz as i64 * 15 * 60;
        }
    }
    ts
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
