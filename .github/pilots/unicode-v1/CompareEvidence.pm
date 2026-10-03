package CompareEvidence;
# SPDX-License-Identifier: GPL-2.0-only
use OverflowPolicy (); use strict; use warnings; use Evidence; use Digest::SHA qw(sha256_hex); use Encode qw(encode decode FB_CROAK); use Text::ParseWords qw(shellwords);
use Time::Local qw(timegm);
my $IMAGE='docker.io/library/debian:trixie-slim@sha256:7792b1f7702a86946cd518db72b6a407302c3e9bc1635634368b878189e8221c';
sub canonical_json {
 my($text)=@_; my $v=$Evidence::JSON->decode($text);
 die "noncanonical/duplicate protocol JSON\n" unless $Evidence::JSON->encode($v) eq $text; return $v;
}
sub runner_line {
 my($line)=@_;$line=~s/\r$//;
 die "NUL within runner text\n" if index($line,"\0")>=0;
 # A downloaded runner segment may start with one UTF-8 BOM before its
 # timestamp. This never normalizes protocol text or decoded payload bytes.
 $line=~s/^\xEF\xBB\xBF(?=\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(?:\.\d+)?Z )//;
 die "misplaced transport BOM\n" if $line=~/\xEF\xBB\xBF|\x{FEFF}/;
 if($line=~/^(\d{4})-(\d\d)-(\d\d)T(\d\d):(\d\d):(\d\d)(?:\.\d+)?Z /){
  my($year,$month,$day,$hour,$minute,$second)=($1,$2,$3,$4,$5,$6);
  die "malformed runner timestamp\n" unless $month>=1 && $month<=12 && $day>=1 && $hour<=23 && $minute<=59 && $second<=59;
  eval {timegm($second,$minute,$hour,$day,$month-1,$year)};
  die "malformed runner timestamp\n" if $@;
  $line=~s/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(?:\.\d+)?Z //;
 }elsif($line=~/^\d{4}-/){die "malformed runner timestamp\n"}
 return $line;
}
sub parse {
 my($raw,$provider,$receipt)=@_;
 my @roles=Evidence::roles(); my(%files,@manifest); my($state,$index,$terminal)=('outside',0,0); my($head,$chunk,$bytes);
 for my $line(split /\n/,$raw,-1){
  $line=runner_line($line);
  if($state eq 'outside'){
   if($line=~/^UEV3 BEGIN (.+)$/){
    my $h=canonical_json($1); Evidence::exact_keys($h,qw(schema provider run_id trigger_sha run_attempt source_sha));
    die "wrong protocol context\n" unless $h->{schema}==3 && $h->{provider} eq $provider && "$h->{run_id}" eq "$receipt->{run_id}" && $h->{trigger_sha} eq $receipt->{trigger_sha} && $h->{run_attempt}==1 && $h->{source_sha} eq $Evidence::SOURCE;
    $state='between'; next;
   }
  } elsif($state eq 'between'){
   if($line=~/^UEV3 FILE (.+)$/){
    die "extra evidence role\n" unless $index<@roles;
    $head=canonical_json($1);Evidence::exact_keys($head,qw(role path bytes sha256));
    die "missing/reordered/duplicate/arbitrary evidence role\n" unless $head->{role} eq $roles[$index][0] && $head->{path} eq $roles[$index][1];
    die "invalid payload metadata\n" unless "$head->{bytes}"=~/^(?:0|[1-9][0-9]*)$/ && ($head->{bytes}>0 || Evidence::may_be_empty($head->{role})) && $head->{bytes}<=134217728 && $head->{sha256}=~/^[0-9a-f]{64}$/;
    ($chunk,$bytes)=(0,'');$state='file';next;
   }
   if($line=~/^UEV3 END (.+)$/){
    my $end=canonical_json($1);Evidence::exact_keys($end,qw(files manifest_sha256 complete));
    die "incomplete evidence role set\n" unless $index==@roles && $end->{files}==@roles && $end->{complete};
    die "manifest digest mismatch\n" unless $end->{manifest_sha256} eq sha256_hex(join("\n",@manifest)."\n");
    $state='ended';next;
   }
   die "unexpected evidence framing\n";
  } elsif($state eq 'file'){
   if($line=~/^UEV3 DATA (.+)$/){
    my $part=canonical_json($1);Evidence::exact_keys($part,qw(index text));
    die "BOM within evidence payload\n" if !ref($part->{text}) && $part->{text}=~/\x{FEFF}/;
    die "NUL within evidence payload\n" if !ref($part->{text}) && index($part->{text},"\0")>=0;
    die "reordered/invalid chunk\n" unless $part->{index}==$chunk && !ref($part->{text}) && length($part->{text})>0 && length($part->{text})<=4096;
    $bytes.=encode('UTF-8',$part->{text});$chunk++;die "payload exceeds declaration\n" if length($bytes)>$head->{bytes};next;
   }
   if($line=~/^UEV3 END_FILE ([a-zA-Z0-9_.-]+) ([0-9]+)$/){
    die "file footer mismatch\n" unless $1 eq $head->{role} && $2==$chunk && ($chunk>0 || Evidence::may_be_empty($head->{role}));
    die "truncated/corrupt payload\n" unless length($bytes)==$head->{bytes} && sha256_hex($bytes) eq $head->{sha256};
    $files{$head->{role}}=decode('UTF-8',$bytes,FB_CROAK);
    push @manifest,join("\t",$head->{role},$head->{path},$head->{bytes},$head->{sha256},$chunk);
    $index++;$state='between';next;
   }
   die "unexpected text within evidence payload\n";
  } elsif($state eq 'ended'){
   if($line=~/^UNICODE_PILOT_V3_TERMINAL /){
    die "failed/duplicate terminal result\n" unless $line eq "UNICODE_PILOT_V3_TERMINAL provider=$provider exit=0 cleanup=0 emission=0 post_resource=0" && !$terminal++;
    next;
   }
  }
  die "unexpected/reordered protocol or legacy result\n" if $line=~/^(UEV3|UNICODE_PILOT|CONTAINER_TERMINAL|BEGIN_PAIR_RECEIPT|END_PAIR_RECEIPT|BEGIN_COMPLETE_MEMBER_LOG|END_COMPLETE_MEMBER_LOG)/;
  die "runner reported failure\n" if $line=~/Process completed with exit code ([1-9][0-9]*)/;
 }
 die "missing/truncated/endless evidence envelope\n" unless $state eq 'ended' && $terminal==1;
 return \%files;
}
sub assert_keys {my($hash,@wanted)=@_;die "wrong named hash roles\n" unless join('\n',sort keys%$hash) eq join('\n',sort@wanted)}
sub command {
 my($text,@want)=@_;my @actual=shellwords($text);
 die "unexpected command array\n" unless join("\0",@actual) eq join("\0",@want);
}
sub stage {
 my($files,$scope,$name,@cmd)=@_;
 die "failed/incomplete stage $scope.$name\n" unless $files->{"$scope.$name.status"}=~/\Astage=\Q$name\E native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=0 timeout_seconds=([1-9][0-9]*)\n\z/;
 my $allowance=$1;my %limits=(pull=>600,install=>1200,configure=>600,build=>1800,noop=>600,ownership=>300,guest=>1020,parse=>300);
 die "wrong stage allowance\n" unless $name eq 'container' ? ($allowance>1560 && $allowance<=6000) : (exists($limits{$name}) && $allowance==$limits{$name});
 die "empty stage log\n" unless $files->{"$scope.$name.log"}=~/\S/;
 Evidence::resources($files->{"$scope.$name.resources.tsv"});
 command($files->{"$scope.$name.command"},@cmd) if @cmd;
}
sub validate {
 my($f,$provider,$receipt,$dir)=@_;
 my $context=$Evidence::JSON->decode($f->{'runtime.context.json'});
 Evidence::exact_keys($context,qw(provider run_id trigger_sha run_attempt source_sha job_started_epoch job_deadline_epoch container_started_epoch container_deadline_epoch));
 die "runtime context mismatch\n" unless $context->{provider} eq $provider && "$context->{run_id}" eq "$receipt->{run_id}" && $context->{trigger_sha} eq $receipt->{trigger_sha} && $context->{run_attempt}==1 && $context->{source_sha} eq $Evidence::SOURCE;
 for(qw(job_started_epoch job_deadline_epoch container_started_epoch container_deadline_epoch)){die "invalid deadline value\n" unless "$context->{$_}"=~/^[1-9][0-9]{9,11}$/}
 die "unreviewed aggregate/job budget\n" unless $context->{job_deadline_epoch}==$context->{job_started_epoch}+6900 && $context->{container_started_epoch}>=$context->{job_started_epoch} && $context->{container_deadline_epoch}<=$context->{container_started_epoch}+6000 && $context->{container_deadline_epoch}<=$context->{job_deadline_epoch}-180;
 my $container_allowance=$context->{container_deadline_epoch}-$context->{container_started_epoch};
 die "container allowance differs from deadline\n" unless $f->{'host.container.status'}=~/timeout_seconds=\Q$container_allowance\E\n\z/;
 die "host result/cleanup failed\n" unless $f->{'host.cleanup.status'} eq "cleanup_exit=0\n" && $f->{'host.result.status'} eq "native_exit=0 cleanup_exit=0\n";
 die "missing owned container identity\n" unless $f->{'host.container.cid'}=~/\A[0-9a-f]{64}\z/;
 my $owned_cid=$f->{'host.container.cid'};
 my $verified_absence=()=$f->{'host.cleanup.inspect'}=~/^cleanup_verified_absent=\Q$owned_cid\E$/mg;
 die "container absence not verified\n" unless $verified_absence==1;
 for my $p(qw(preflight terminal)){
  my $phase=$p eq 'preflight'?'initial':'terminal';
  die "host admission/terminal missing\n" unless $f->{"host.$p.txt"}=~/RESOURCE_STAGE=\Q$phase\E UTC=/ && $f->{"host.$p.txt"}=~/CAPS O_allocated_bytes=(\d+) evidence_allocated_bytes=(\d+)/ && $1<=1342177280 && $2<=134209536;
 }
 die "wrong image\n" unless $f->{'host.image.txt'}=~/sha256:7792b1f7702a86946cd518db72b6a407302c3e9bc1635634368b878189e8221c/ && $f->{'host.image.txt'}=~/amd64/;
 stage($f,'host','pull','docker','pull','--platform','linux/amd64',$IMAGE);
 stage($f,'host','container');
 my @docker=shellwords($f->{'host.container.command'});
 die "wrong Docker execution boundaries\n" unless @docker==36 && join(' ',@docker[0..7]) eq "docker run --rm --platform linux/amd64 --name unicode-pilot-$receipt->{run_id}-$provider --cidfile";
 die "wrong Docker label\n" unless $docker[9] eq '--label' && $docker[10] eq "unicode_pilot_run=$receipt->{run_id}";
 my @roots;
 for my $j(0..2){my $i=11+$j*2;die "unexpected Docker mount\n" unless $docker[$i] eq '--mount';my($src,$dst,$ro)=$docker[$i+1]=~/^type=bind,src=(\/[a-zA-Z0-9_.\/-]+),dst=(\/[a-z]+)(,readonly)?$/;die "invalid Docker bind\n" unless defined$src;
  my @expected=('/src','/pilot','/work');die "wrong Docker destination\n" unless $dst eq $expected[$j] && ($j==2?!defined($ro):defined($ro));
  my $suffix=$j==0?'/linux':$j==1?'/pilot/.github/pilots/unicode-v1':'/work';die "unexpected source mount\n" unless $src=~s/\Q$suffix\E$//;push @roots,$src;
 }
 die "mount roots differ\n" unless $roots[0] eq $roots[1] && $roots[0] eq $roots[2] && $docker[8] eq "$roots[0]/evidence/container.cid";
 my @tail=('--env',"PILOT_PROVIDER=$provider",'--env','PILOT_RUN_ATTEMPT=1','--env',"PILOT_RUN_ID=$receipt->{run_id}",'--env',"PILOT_TRIGGER_SHA=$receipt->{trigger_sha}",'--env',"PILOT_JOB_STARTED_EPOCH=$context->{job_started_epoch}",'--env',"PILOT_JOB_DEADLINE_EPOCH=$context->{job_deadline_epoch}",'--env',"PILOT_CONTAINER_STARTED_EPOCH=$context->{container_started_epoch}",'--env',"PILOT_CONTAINER_DEADLINE_EPOCH=$context->{container_deadline_epoch}",$IMAGE,'bash','/pilot/run-container.sh');
 die "unexpected Docker environment/command\n" unless join("\0",@docker[17..$#docker]) eq join("\0",@tail);
 stage($f,'runtime','install','bash','/pilot/install-public-tools.sh');
 my @make=('make','-C','/src','O=/work/O','ARCH=x86_64','LLVM=1','HOST_TOOLS_LANG=rust','RUSTC=/usr/bin/rustc','-j2');
 stage($f,'runtime','configure',@make,'olddefconfig');stage($f,'runtime','build',@make,'bzImage');stage($f,'runtime','noop',@make,'bzImage');
 stage($f,'runtime','ownership','bash','/pilot/audit-commands.sh',$provider,'/work/O');
 stage($f,'runtime','guest','timeout','--signal=TERM','--kill-after=10','900','qemu-system-x86_64','-L','/usr/share/qemu','-nodefaults','-m','2048','-kernel','/work/O/arch/x86/boot/bzImage','-append','console=ttyS0 kunit.enable=1 kunit.autorun=1 kunit.filter_glob=unicode_normalization kunit_shutdown=reboot panic=-1 oops=panic nokaslr','-no-reboot','-nographic','-accel','tcg,thread=single','-cpu','max','-smp','1','-serial','stdio','-nic','none','-bios','qboot.rom');
 my $admission=$Evidence::JSON->decode($f->{'runtime.guest-admission.json'});
 Evidence::exact_keys($admission,qw(now job_remaining container_remaining required_seconds guest_timeout guest_kill_grace guest_stage_allowance parser_seconds cleanup_seconds evidence_seconds));
 for(keys%$admission){die "invalid guest admission number\n" unless "$admission->{$_}"=~/^[0-9]+$/}
 die "changed original guest timeout/grace or reserve\n" unless $admission->{guest_timeout}==900 && $admission->{guest_kill_grace}==10 && $admission->{guest_stage_allowance}==1020 && $admission->{parser_seconds}==300 && $admission->{cleanup_seconds}==60 && $admission->{evidence_seconds}==180 && $admission->{required_seconds}==1560;
 die "insufficient or inconsistent guest budget\n" unless $admission->{now}>=$context->{container_started_epoch} && $admission->{container_remaining}==$context->{container_deadline_epoch}-$admission->{now} && $admission->{job_remaining}==$context->{job_deadline_epoch}-$admission->{now} && $admission->{container_remaining}>=1560 && $admission->{job_remaining}>=1560;
 stage($f,'runtime','parse','bash','/pilot/parse-guest.sh');
 die "build output missing completed kernel\n" unless $f->{'runtime.build.log'}=~/Kernel: arch\/x86\/boot\/bzImage is ready/;
 for(qw(configure noop)){die "missing make output\n" unless $f->{"runtime.$_.log"}=~/make.*(?:Entering|Leaving) directory/}
 my $lock=Evidence::slurp("$dir/public-source.sha256");die "wrong public source lock\n" unless $f->{'runtime.source.lock'} eq $lock;
 my $source_hashes=Evidence::hashes($lock);
 my $source_ok=join('',map{"$_: OK\n"}map{ /^\w+  (.+)$/ ? $1 : die 'lock' }split /\n/,$lock)."SOURCE_VERIFIED=$Evidence::SOURCE\n";
 die "missing original source verification\n" unless $f->{'runtime.source.before.txt'} eq $source_ok && $f->{'runtime.source.after.txt'} eq $source_ok;
 my $want=Evidence::config(Evidence::slurp("$dir/config.symbols"));my $got=Evidence::config($f->{'runtime.config.full'});$want->{CONFIG_RUST_UNICODE_NORM}=$provider eq 'rust'?'y':'n';
 for(keys%$want){die "changed/missing configuration $_\n" unless exists$got->{$_} && $got->{$_} eq $want->{$_}}
 my %allowed=map{$_=>1}qw(CONFIG_RUST_AFS_ADDR_PREFS CONFIG_RUST_BLK_CRYPTO_FALLBACK CONFIG_RUST_BPF_TOKEN);
 for(keys%$got){die "unreviewed config addition\n" unless exists$want->{$_} || ($allowed{$_} && $got->{$_} eq 'n')}
 die "config observer receipt absent\n" unless $f->{'runtime.config-check.txt'} eq "CONFIG_ALL_VALUES_PRESERVED provider=$provider symbols=".scalar(keys%$got)."\n";
 my $suites="trixie\t0584fba32e13e0ab8285fb16c27adea1ec03a73669c18702821094fd6ca86675\ntrixie-updates\tbde606f5b1303e2c864d2118fca1a07c8aca59e097d10180f4e1ea1e7f53c99c\ntrixie-security\tf44452462d1d78526274ee79a391cd8e8df9dc327717dabc39a9d868eb7e4114\n";
 die "missing/wrong signed suite identities\n" unless $f->{'runtime.apt-suites.tsv'} eq $suites;
 for my $suite(qw(trixie trixie-updates trixie-security)){die "missing actual apt suite hash check\n" unless $f->{'runtime.install.log'}=~/_dists_\Q$suite\E_InRelease: OK/m}
 my %installed;
 for(split /\n/,$f->{'runtime.installed-packages.tsv'}){my($n,$v,$a)=split /\t/;die "installed package record\n" unless defined$a && $n=~/^[a-z0-9+.:_-]+$/ && $v=~/^\S+$/ && $a=~/^(amd64|all)$/;$n=~s/:amd64$//;die "duplicate package\n" if exists$installed{$n};$installed{$n}=$v}
 my @pins=split /\n/,Evidence::slurp("$dir/public-packages-candidate.tsv");shift@pins;for(@pins){my($n,$v)=split /\t/;die "missing/wrong pinned package $n\n" unless ($installed{$n}//'') eq $v}
 die "missing dependency closure\n" unless $f->{'runtime.base-packages.tsv'}=~/^base-files\t/m && $f->{'runtime.downloaded-packages.tsv'}=~/^[a-z0-9+.-]+\t\S+\t(?:amd64|all)\t[0-9a-f]{64}$/m;
 my $versions=$f->{'runtime.tool-versions.txt'};for(qr/clang version 19\.1\.7/,qr/rustc 1\.85\.1/,qr/bindgen 0\.71\.1/,qr/v1\.30/,qr/QEMU emulator version 10\.0\.13/){die "wrong/missing tool version\n" unless $versions=~$_}
 die "missing compiler path equivalence\n" unless $versions=~m{^RUSTC_PATH_EQUIVALENCE\ bare\=rustc\ resolved\=\/usr\/bin\/rustc\ requested\=\/usr\/bin\/rustc$}m;
 my $tools=Evidence::hashes($f->{'runtime.tool-binaries.sha256'});assert_keys($tools,qw(/usr/bin/rustc /usr/bin/rustfmt /usr/bin/bindgen /usr/bin/pahole /usr/bin/qemu-system-x86_64 /usr/share/qemu/qboot.rom));
 my $images=Evidence::hashes($f->{'runtime.image-identities.sha256'});assert_keys($images,qw(/work/O/arch/x86/boot/bzImage /work/O/vmlinux /work/O/vmlinux.unstripped /work/O/vmlinux.o /work/O/.config));
 my @objects=qw(fs/unicode/tests/utf8_kunit.o fs/unicode/utf8-core.o fs/unicode/utf8data.o lib/crc/crc16.o);
 my $objects=Evidence::hashes($f->{'runtime.object-identities.sha256'});assert_keys($objects,map{"/work/O/$_"}@objects);
 my $whole=Evidence::hashes($f->{'runtime.O-before-noop.sha256'});
 for(qw(./fs/unicode/utf8-norm.o ./vmlinux.a)){die "missing selected normalizer/archive identity\n" unless exists $whole->{$_}}
 for(keys%$whole){die "unsafe O manifest path\n" unless m!^\./[a-zA-Z0-9_+.,/=-]+$! && !m!(?:^|/)\.\.(?:/|$)!}
 for my $kind(qw(after-noop before-guest after-guest)){die "O changed/incomplete invariance evidence\n" unless $f->{"runtime.O-$kind.sha256"} eq $f->{'runtime.O-before-noop.sha256'}}
 for my $path(keys%$images,keys%$objects){my $key=$path;$key=~s!^/work/O/!./!;my $hash=$images->{$path}//$objects->{$path};die "O manifest disagrees with named objects/images\n" unless ($whole->{$key}//'') eq $hash}
 die "config digest mismatch\n" unless $images->{'/work/O/.config'} eq sha256_hex(encode('UTF-8',$f->{'runtime.config.full'}));
 for my $kind(qw(after-noop before-guest after-guest)){die "O symlinks changed\n" unless $f->{"runtime.O-links-$kind.tsv"} eq $f->{'runtime.O-links-before-noop.tsv'}}
 die "missing O symlink inventory\n" unless $f->{'runtime.O-links-before-noop.tsv'}=~/^path\ttarget\n/;
 my @cmdroles=qw(test core data crc);my @cmdpaths=qw(fs/unicode/tests/.utf8_kunit.o.cmd fs/unicode/.utf8-core.o.cmd fs/unicode/.utf8data.o.cmd lib/crc/.crc16.o.cmd);
 my @sources=('/src/fs/unicode/tests/utf8_kunit.c','/src/fs/unicode/utf8-core.c','fs/unicode/utf8data.c','/src/lib/crc/crc16.c');
 for my $i(0..3){my $cmd=$f->{"runtime.$cmdroles[$i].cmd"};die "original C command/source/flags absent\n" unless $cmd=~/^savedcmd_\Q$objects[$i]\E := clang .* -fno-strict-overflow .* -c -o /m && $cmd=~/^source_\Q$objects[$i]\E := \Q$sources[$i]\E$/m;die "command digest missing from O inventory\n" unless ($whole->{"./$cmdpaths[$i]"}//'') eq sha256_hex(encode('UTF-8',$cmd))}
 die "unapproved Rust compiler hash\n" unless $tools->{'/usr/bin/rustc'} eq 'b4e139165f4f075f9a3fb4b20e7e4898472f08304961706072752d4aa11522b1';
 my $norm=$f->{'runtime.norm.cmd'};my $extension=$provider eq 'rust'?'rs':'c';
 OverflowPolicy::check($norm) if $provider eq 'rust';
 die "wrong selected normalizer source\n" unless $norm=~/^source_fs\/unicode\/utf8-norm\.o := \/src\/fs\/unicode\/utf8-norm\.\Q$extension\E$/m;
 if($provider eq 'rust'){die "Rust compiler/overflow flags absent\n" unless $norm=~/^savedcmd_fs\/unicode\/utf8-norm\.o := OBJTREE=\/work\/O RUST_MODFILE=fs\/unicode\/unicode \/usr\/bin\/rustc .* -Coverflow-checks=y /m}
 else{die "original C normalizer flags absent\n" unless $norm=~/^savedcmd_fs\/unicode\/utf8-norm\.o := clang .* -fno-strict-overflow .* -c -o /m}
 die "selected normalizer command not bound to O inventory\n" unless ($whole->{'./fs/unicode/.utf8-norm.o.cmd'}//'') eq sha256_hex(encode('UTF-8',$norm));
 my $owner=$f->{'runtime.ownership.log'};
 for my $label(qw(selected_normalizer original_C_tests original_C_core original_C_crc16)){die "missing linked body proof\n" unless $owner=~/^LINKED_OBJECT_SECTION_BYTES \Q$label\E \.text [1-9][0-9]*$/m}
 for my $api(@Evidence::APIS){die "missing owner/caller proof\n" unless $owner=~/^OWNER_FUNCTION \Q$api\E input_bytes=([1-9][0-9]*) final_bytes=([1-9][0-9]*)$/m && $1==$2 && $owner=~/^FINAL_CALL original_(?:test|core) \S+ -> \Q$api\E$/m}
 for my $pair(['utf8agetab',92],['utf8nfdicfdata',184],['utf8nfdidata',184],['utf8data',64256]){die "missing unchanged table proof\n" unless $owner=~/^ORIGINAL_TABLE_BYTES \Q$pair->[0]\E $pair->[1]$/m}
 my $registrations=join(',',map{($_,"string:$_",'string:utf8_kunit')}@Evidence::CASES);
 die "missing exact original test registration pointers\n" unless $owner=~/^FINAL_DATA unicode_normalization_test_cases \Q$registrations\E$/m;
 die "missing setup/teardown pointers\n" unless $owner=~/^FINAL_DATA unicode_normalization_test_suite init_test_ucd,exit_test_ucd,unicode_normalization_test_cases$/m;
 die "missing original table pointers\n" unless $owner=~/^FINAL_DATA utf8_data_table utf8agetab,utf8nfdicfdata,utf8nfdidata,utf8data$/m;
 for my $call('original_test check_supported_versions -> utf8version_is_supported','original_core utf8_validate -> utf8nlen','original_core utf8_strncmp -> utf8ncursor','original_core utf8_strncmp -> utf8byte'){die "missing exact original C caller\n" unless $owner=~/^FINAL_CALL \Q$call\E$/m}
 my $owner_verdicts=()=$owner=~/^FINAL_ELF_OWNERSHIP_PASS original_C_calls=resolved cases=4 suite=1 original_tables=equal$/mg;
 my $command_verdicts=()=$owner=~/^SELECTED_COMMAND_ARCHIVE_AND_FINAL_OWNER_PASS$/mg;
 die "missing/duplicate complete final owner verdict\n" unless $owner_verdicts==1;
 die "missing/duplicate command/archive verdict\n" unless $command_verdicts==1;
 my $serial=$f->{'runtime.guest.log'};my(@seen,$suite,$plan,$top,$summary);
 for(split /\n/,$serial){s/\r$//;die "failed guest diagnostic\n" if /\bnot ok\b|#\s*SKIP\b|Bail out!|Kernel panic|Oops:|BUG:|KASAN:|UBSAN:/i;
  if(/^1\.\.1\s*$/){$top++;next}if(/^\s*# Subtest: unicode_normalization\s*$/){$suite++;next}if(/^\s+1\.\.4\s*$/){$plan++;next}
  if(/^\s+ok ([1-4]) (check_\w+)\s*$/){push@seen,"$1:$2";next}if(/^ok 1 unicode_normalization\s*$/){$summary++;next}die "extra guest result\n" if /^\s*(?:(?:not\s+)?ok\b|\d+\.\.\d+\b|# Subtest:)/;
 }
 my @wantcases=map{($_+1).':'.$Evidence::CASES[$_]}0..3;die "not exact original guest cases\n" unless ($suite//0)==1 && ($plan//0)==1 && ($top//0)==1 && ($summary//0)==1 && join(',',@seen) eq join(',',@wantcases);
 die "observer/parser status failed\n" unless $f->{'runtime.observer-parser.status'} eq "observer_exit=0 parser_exit=0\n" && $f->{'runtime.strict-observer.stderr'} eq '' && $f->{'runtime.strict-observer.txt'} eq 'ORIGINAL_C_CASES_PASS '.join(',',@Evidence::CASES)."\n";
 my $json=$Evidence::JSON->decode($f->{'runtime.upstream-result.json'});die "upstream result topology\n" unless defined($json->{name}) && $json->{name} eq 'KUnit Test Group' && ref($json->{sub_groups}) eq 'ARRAY' && @{$json->{sub_groups}}==1 && ref($json->{test_cases}) eq 'ARRAY' && !@{$json->{test_cases}};
 my $group=$json->{sub_groups}[0];die "upstream wrong suite\n" unless $group->{name} eq 'unicode_normalization' && ref($group->{sub_groups}) eq 'ARRAY' && !@{$group->{sub_groups}} && ref($group->{test_cases}) eq 'ARRAY' && @{$group->{test_cases}}==4;
 for my $i(0..3){die "upstream wrong/failed original case\n" unless $group->{test_cases}[$i]{name} eq $Evidence::CASES[$i] && $group->{test_cases}[$i]{status} eq 'PASS'}
 for my $node($json,$group){for(qw(tests passed)){die "upstream count mismatch\n" unless $node->{misc}{$_}==4}for(qw(failed crashed skipped errors)){die "upstream failure\n" unless defined($node->{misc}{$_}) && $node->{misc}{$_}==0}}
 die "upstream parser output missing\n" unless $f->{'runtime.upstream-parser.txt'}=~/Testing complete\. Ran 4 tests: passed: 4/;
 die "incomplete member verdict\n" unless $f->{'runtime.member.status'} eq "provider=$provider source=$Evidence::SOURCE config=pass noop=pass ownership=pass guest=pass parser=pass invariance=pass\n";
 return $got;
}
sub compare {
 my($receipt,$craw,$rraw,$dir)=@_;
 Evidence::exact_keys($receipt,qw(schema repository branch event run_id run_attempt trigger_sha decoding jobs));
 die "wrong trusted download receipt context\n" unless $receipt->{schema}==1 && $receipt->{repository} eq 'a-coding-mage/linux' && $receipt->{branch} eq 'feat/rust-translation-lupos' && $receipt->{event} eq 'push' && "$receipt->{run_id}"=~/^[1-9][0-9]*$/ && $receipt->{run_attempt}==1 && $receipt->{trigger_sha}=~/^[0-9a-f]{40}$/ && $receipt->{decoding} eq 'connector-decoded-text-utf8' && ref($receipt->{jobs}) eq 'HASH';
 Evidence::exact_keys($receipt->{jobs},qw(c rust));my%files;my%config;
 for my $provider(qw(c rust)){
  my $raw=$provider eq 'c'?$craw:$rraw;my $job=$receipt->{jobs}{$provider};Evidence::exact_keys($job,qw(job_id name status conclusion text_sha256));
  die "wrong/incomplete GitHub job receipt\n" unless "$job->{job_id}"=~/^[1-9][0-9]*$/ && $job->{name} eq "unicode-$provider" && $job->{status} eq 'completed' && $job->{conclusion} eq 'success' && $job->{text_sha256} eq sha256_hex($raw);
  $files{$provider}=parse($raw,$provider,$receipt);$config{$provider}=validate($files{$provider},$provider,$receipt,$dir);
 }
 die "same job supplied twice\n" if $receipt->{jobs}{c}{job_id}==$receipt->{jobs}{rust}{job_id};
 for my $role(qw(source.lock source.before.txt source.after.txt apt-suites.tsv base-packages.tsv downloaded-packages.tsv installed-packages.tsv tool-versions.txt tool-binaries.sha256 object-identities.sha256 test.cmd core.cmd data.cmd crc.cmd)){
  die "C/R comparison mismatch $role\n" unless $files{c}{"runtime.$role"} eq $files{rust}{"runtime.$role"};
 }
 delete $config{c}{CONFIG_RUST_UNICODE_NORM};delete $config{rust}{CONFIG_RUST_UNICODE_NORM};
 die "C/R config drift beyond selector\n" unless $Evidence::JSON->encode($config{c}) eq $Evidence::JSON->encode($config{rust});
 return 1;
}
1;
