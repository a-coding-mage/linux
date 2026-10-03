#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Bindgen omits a named anonymous struct member in ns_common's C union.
# Recover that union arm using the separately generated canonical ns_tree.
# No C field offset, member list, size or alignment is re-created here.
use strict;
use warnings;
local $/;
my $source=<>;
die "missing canonical ns_tree" unless $source =~ /pub struct ns_tree\s*\{/;
die "missing configured ns_tree size" unless $source =~ /pub const LUPOS_PID_SIZE_ns_tree\s*:/;
my $count = ($source =~ s~(pub union ns_common__bindgen_ty_2\s*\{)\s*(pub ns_rcu\s*:\s*callback_head\s*,)\s*\}~$1 pub tree: ns_tree, $2 }~g);
die "anonymous namespace union representation changed; inspect native declarations" unless $count == 1;
print $source;
