#!/usr/bin/env perl
# SPDX-License-Identifier: GPL-2.0-only
# Entirely synthetic protocol fixtures, built in memory. No kernel/VM execution.
use strict;use warnings;use FindBin;use lib "$FindBin::Bin/..";use Evidence;use CompareEvidence;use Digest::SHA qw(sha256_hex);use Encode qw(encode);
my $dir=$ENV{PILOT_FIXTURE_PROFILE_DIR}//"$FindBin::Bin/..";my $run='123';my $trigger='a'x40;
my $job_start=1790899200;my $job_deadline=$job_start+6900;
my $container_start=$job_start+300;my $container_deadline=$job_start+6300;
my $image='docker.io/library/debian:trixie-slim@sha256:7792b1f7702a86946cd518db72b6a407302c3e9bc1635634368b878189e8221c';
sub cmd {return join(' ',map{my$x=$_;$x=~s/([^A-Za-z0-9_\@%+=:,\.\/-])/\\$1/g;$x}@_)." \n"}
sub fixtures {
 my($p)=@_;my%f=map{$_->[0]=>"synthetic fixture\n"}Evidence::roles();
 my $resources="phase\tutc\tfree_bytes\tavailable_memory\tfree_inodes\tO_allocated_bytes\tevidence_allocated_bytes\nadmission\t2026-10-02T00:00:00Z\t8000000000\t8000000000\t1000000\t500000000\t5000000\nterminal\t2026-10-02T00:00:05Z\t8000000000\t8000000000\t1000000\t500000000\t5000000\n";
 my%limits=(pull=>600,container=>6000,install=>1200,configure=>600,build=>1800,noop=>600,ownership=>300,guest=>1020,parse=>300);
 for my$s(qw(pull container)){ $f{"host.$s.status"}="stage=$s native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=0 timeout_seconds=$limits{$s}\n";$f{"host.$s.resources.tsv"}=$resources }
 for my$s(@Evidence::STAGES){$f{"runtime.$s.status"}="stage=$s native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=0 timeout_seconds=$limits{$s}\n";$f{"runtime.$s.resources.tsv"}=$resources}
 $f{'host.preflight.txt'}="RESOURCE_STAGE=initial UTC=2026-10-02T00:00:00Z\nCAPS O_allocated_bytes=1 evidence_allocated_bytes=1\n";
 $f{'host.terminal.txt'}="RESOURCE_STAGE=terminal UTC=2026-10-02T00:00:05Z\nCAPS O_allocated_bytes=1 evidence_allocated_bytes=1\n";
 $f{'host.image.txt'}="$image amd64\n";$f{'host.cleanup.status'}="cleanup_exit=0\n";$f{'host.result.status'}="native_exit=0 cleanup_exit=0\n";
 $f{'host.container.cid'}='b'x64;$f{'host.cleanup.inspect'}='cleanup_verified_absent='.('b'x64)."\n";$f{'host.cleanup.stderr'}='';
 $f{'host.pull.command'}=cmd('docker','pull','--platform','linux/amd64',$image);
 my$root='/runner/work/linux/linux';
 $f{'host.container.command'}=cmd('docker','run','--rm','--platform','linux/amd64','--name',"unicode-pilot-$run-$p",'--cidfile',"$root/evidence/container.cid",'--label',"unicode_pilot_run=$run",'--mount',"type=bind,src=$root/linux,dst=/src,readonly",'--mount',"type=bind,src=$root/pilot/.github/pilots/unicode-v1,dst=/pilot,readonly",'--mount',"type=bind,src=$root/work,dst=/work",'--env',"PILOT_PROVIDER=$p",'--env','PILOT_RUN_ATTEMPT=1','--env',"PILOT_RUN_ID=$run",'--env',"PILOT_TRIGGER_SHA=$trigger",'--env',"PILOT_JOB_STARTED_EPOCH=$job_start",'--env',"PILOT_JOB_DEADLINE_EPOCH=$job_deadline",'--env',"PILOT_CONTAINER_STARTED_EPOCH=$container_start",'--env',"PILOT_CONTAINER_DEADLINE_EPOCH=$container_deadline",$image,'bash','/pilot/run-container.sh');
 $f{'runtime.context.json'}=$Evidence::JSON->encode({provider=>$p,run_id=>$run,trigger_sha=>$trigger,run_attempt=>1,source_sha=>$Evidence::SOURCE,job_started_epoch=>$job_start,job_deadline_epoch=>$job_deadline,container_started_epoch=>$container_start,container_deadline_epoch=>$container_deadline})."\n";
 $f{'runtime.source.lock'}=Evidence::slurp("$dir/public-source.sha256");
 $f{'runtime.source.before.txt'}=join('',map{/^\w+  (.+)$/;"$1: OK\n"}split/\n/,$f{'runtime.source.lock'})."SOURCE_VERIFIED=$Evidence::SOURCE\n";
 $f{'runtime.source.after.txt'}=$f{'runtime.source.before.txt'};
 $f{'runtime.apt-suites.tsv'}="trixie\t0584fba32e13e0ab8285fb16c27adea1ec03a73669c18702821094fd6ca86675\ntrixie-updates\tbde606f5b1303e2c864d2118fca1a07c8aca59e097d10180f4e1ea1e7f53c99c\ntrixie-security\tf44452462d1d78526274ee79a391cd8e8df9dc327717dabc39a9d868eb7e4114\n";
 my@pins=split/\n/,Evidence::slurp("$dir/public-packages-candidate.tsv");shift@pins;
 $f{'runtime.installed-packages.tsv'}=join('',map{my@v=split/\t/;join("\t",@v[0..2])."\n"}@pins);
 $f{'runtime.base-packages.tsv'}="base-files\t13.7\tamd64\n";
 $f{'runtime.downloaded-packages.tsv'}=join('',map{my@v=split/\t/;join("\t",@v[0,1,2,5])."\n"}@pins);
 $f{'runtime.tool-versions.txt'}="RUSTC_PATH_EQUIVALENCE bare=rustc resolved=/usr/bin/rustc requested=/usr/bin/rustc\nDebian clang version 19.1.7\nrustc 1.85.1\nbindgen 0.71.1\nv1.30\nQEMU emulator version 10.0.13\n";
 $f{'runtime.tool-binaries.sha256'}=join('',map{sha256_hex($_)."  $_\n"}qw(/usr/bin/rustc /usr/bin/rustfmt /usr/bin/bindgen /usr/bin/pahole /usr/bin/qemu-system-x86_64 /usr/share/qemu/qboot.rom));
 $f{'runtime.tool-binaries.sha256'}=~s{^[0-9a-f]{64}  /usr/bin/rustc$}{b4e139165f4f075f9a3fb4b20e7e4898472f08304961706072752d4aa11522b1  /usr/bin/rustc}m;
 $f{'runtime.config.full'}=Evidence::slurp("$dir/config.symbols");$f{'runtime.config.full'}=~s/^CONFIG_RUST_UNICODE_NORM=y$/# CONFIG_RUST_UNICODE_NORM is not set/m if$p eq'c';
 $f{'runtime.config-check.txt'}="CONFIG_ALL_VALUES_PRESERVED provider=$p symbols=".scalar(keys%{Evidence::config($f{'runtime.config.full'})})."\n";
 my@make=('make','-C','/src','O=/work/O','ARCH=x86_64','LLVM=1','HOST_TOOLS_LANG=rust','RUSTC=/usr/bin/rustc','-j2');
 $f{'runtime.install.command'}=cmd('bash','/pilot/install-public-tools.sh');
 $f{'runtime.install.log'}=join('',map{"/work/apt-lists/snapshot_dists_${_}_InRelease: OK\n"}qw(trixie trixie-updates trixie-security));
 $f{'runtime.configure.command'}=cmd(@make,'olddefconfig');$f{'runtime.build.command'}=cmd(@make,'bzImage');$f{'runtime.noop.command'}=cmd(@make,'bzImage');
 $f{'runtime.configure.log'}="make: Entering directory '/src'\n";$f{'runtime.noop.log'}=$f{'runtime.configure.log'};$f{'runtime.build.log'}="Kernel: arch/x86/boot/bzImage is ready (#1)\n";
 $f{'runtime.ownership.command'}=cmd('bash','/pilot/audit-commands.sh',$p,'/work/O');
 $f{'runtime.guest.command'}=cmd('timeout','--signal=TERM','--kill-after=10','900','qemu-system-x86_64','-L','/usr/share/qemu','-nodefaults','-m','2048','-kernel','/work/O/arch/x86/boot/bzImage','-append','console=ttyS0 kunit.enable=1 kunit.autorun=1 kunit.filter_glob=unicode_normalization kunit_shutdown=reboot panic=-1 oops=panic nokaslr','-no-reboot','-nographic','-accel','tcg,thread=single','-cpu','max','-smp','1','-serial','stdio','-nic','none','-bios','qboot.rom');
 $f{'runtime.guest-admission.json'}=$Evidence::JSON->encode({now=>$job_start+2000,job_remaining=>4900,container_remaining=>4300,required_seconds=>1560,guest_timeout=>900,guest_kill_grace=>10,guest_stage_allowance=>1020,parser_seconds=>300,cleanup_seconds=>60,evidence_seconds=>180});
 $f{'runtime.parse.command'}=cmd('bash','/pilot/parse-guest.sh');
 my@objects=qw(fs/unicode/tests/utf8_kunit.o fs/unicode/utf8-core.o fs/unicode/utf8data.o lib/crc/crc16.o);my@labels=qw(test core data crc);my@cmdpaths=qw(fs/unicode/tests/.utf8_kunit.o.cmd fs/unicode/.utf8-core.o.cmd fs/unicode/.utf8data.o.cmd lib/crc/.crc16.o.cmd);my@sources=('/src/fs/unicode/tests/utf8_kunit.c','/src/fs/unicode/utf8-core.c','fs/unicode/utf8data.c','/src/lib/crc/crc16.c');my%whole;
 for my$i(0..3){$f{"runtime.$labels[$i].cmd"}="savedcmd_$objects[$i] := clang -O2 -fno-strict-overflow -D__KERNEL__ -c -o $objects[$i] $sources[$i]\nsource_$objects[$i] := $sources[$i]\n";$whole{"./$cmdpaths[$i]"}=sha256_hex($f{"runtime.$labels[$i].cmd"});$whole{"./$objects[$i]"}=sha256_hex($objects[$i])}
 $f{'runtime.norm.cmd'}=$p eq 'rust'?"savedcmd_fs/unicode/utf8-norm.o := OBJTREE=/work/O RUST_MODFILE=fs/unicode/unicode /usr/bin/rustc --edition=2021 -Coverflow-checks=y /src/fs/unicode/utf8-norm.rs \nsource_fs/unicode/utf8-norm.o := /src/fs/unicode/utf8-norm.rs\n":"savedcmd_fs/unicode/utf8-norm.o := clang -O2 -fno-strict-overflow -D__KERNEL__ -c -o fs/unicode/utf8-norm.o /src/fs/unicode/utf8-norm.c\nsource_fs/unicode/utf8-norm.o := /src/fs/unicode/utf8-norm.c\n";
 $whole{'./fs/unicode/.utf8-norm.o.cmd'}=sha256_hex($f{'runtime.norm.cmd'});
 $whole{'./fs/unicode/utf8-norm.o'}=sha256_hex("synthetic normalizer $p");$whole{'./vmlinux.a'}=sha256_hex("synthetic archive $p");
 $f{'runtime.object-identities.sha256'}=join('',map{$whole{"./$_"}."  /work/O/$_\n"}@objects);
 my@images=qw(arch/x86/boot/bzImage vmlinux vmlinux.unstripped vmlinux.o .config);for(@images){$whole{"./$_"}= $_ eq '.config'?sha256_hex($f{'runtime.config.full'}):sha256_hex("$p:$_")}
 $f{'runtime.image-identities.sha256'}=join('',map{$whole{"./$_"}."  /work/O/$_\n"}@images);
 for my$s(qw(before-noop after-noop before-guest after-guest)){$f{"runtime.O-$s.sha256"}=join('',map{"$whole{$_}  $_\n"}sort keys%whole);$f{"runtime.O-links-$s.tsv"}="path\ttarget\n./source\t/src\n"}
 my$owner=join('',map{"LINKED_OBJECT_SECTION_BYTES $_ .text 100\n"}qw(selected_normalizer original_C_tests original_C_core original_C_crc16));
 $owner.=join('',map{"OWNER_FUNCTION $_ input_bytes=100 final_bytes=100\nFINAL_CALL original_core synthetic_caller -> $_\n"}@Evidence::APIS);
 $owner.="ORIGINAL_TABLE_BYTES utf8agetab 92\nORIGINAL_TABLE_BYTES utf8nfdicfdata 184\nORIGINAL_TABLE_BYTES utf8nfdidata 184\nORIGINAL_TABLE_BYTES utf8data 64256\n";
 $owner.='FINAL_DATA unicode_normalization_test_cases '.join(',',map{($_,"string:$_",'string:utf8_kunit')}@Evidence::CASES)."\n";
 $owner.="FINAL_DATA unicode_normalization_test_suite init_test_ucd,exit_test_ucd,unicode_normalization_test_cases\nFINAL_DATA utf8_data_table utf8agetab,utf8nfdicfdata,utf8nfdidata,utf8data\n";
 $owner.="FINAL_CALL original_test check_supported_versions -> utf8version_is_supported\nFINAL_CALL original_core utf8_validate -> utf8nlen\nFINAL_CALL original_core utf8_strncmp -> utf8ncursor\nFINAL_CALL original_core utf8_strncmp -> utf8byte\n";
 $owner.="FINAL_ELF_OWNERSHIP_PASS original_C_calls=resolved cases=4 suite=1 original_tables=equal\nSELECTED_COMMAND_ARCHIVE_AND_FINAL_OWNER_PASS\n";$f{'runtime.ownership.log'}=$owner;
 $f{'runtime.guest.log'}="KTAP version 1\n1..1\n    # Subtest: unicode_normalization\n    1..4\n".join('',map{'    ok '.($_+1)." $Evidence::CASES[$_]\n"}0..3)."ok 1 unicode_normalization\n";
 $f{'runtime.strict-observer.txt'}='ORIGINAL_C_CASES_PASS '.join(',',@Evidence::CASES)."\n";$f{'runtime.observer-parser.status'}="observer_exit=0 parser_exit=0\n";
 $f{'runtime.strict-observer.stderr'}='';
 my$misc={tests=>4,passed=>4,failed=>0,crashed=>0,skipped=>0,errors=>0};
 $f{'runtime.upstream-result.json'}=$Evidence::JSON->encode({name=>'KUnit Test Group',sub_groups=>[{name=>'unicode_normalization',sub_groups=>[],test_cases=>[map{{name=>$_,status=>'PASS'}}@Evidence::CASES],misc=>$misc}],test_cases=>[],misc=>$misc});
 $f{'runtime.upstream-parser.txt'}="Testing complete. Ran 4 tests: passed: 4\n";
 $f{'runtime.member.status'}="provider=$p source=$Evidence::SOURCE config=pass noop=pass ownership=pass guest=pass parser=pass invariance=pass\n";return \%f;
}
sub envelope {
 my($f,$p)=@_;my$s='UEV3 BEGIN '.$Evidence::JSON->encode({schema=>3,provider=>$p,run_id=>$run,trigger_sha=>$trigger,run_attempt=>1,source_sha=>$Evidence::SOURCE})."\n";my@manifest;my$count=0;
 for my$r(Evidence::roles()){my($role,$path)=@$r;my$text=$f->{$role};my$raw=encode('UTF-8',$text);my$hash=sha256_hex($raw);my$len=length($raw);$s.='UEV3 FILE '.$Evidence::JSON->encode({role=>$role,path=>$path,bytes=>$len,sha256=>$hash})."\n";my$n=0;
 while(length$text){my$part=substr($text,0,4096,'');$s.='UEV3 DATA '.$Evidence::JSON->encode({index=>$n,text=>$part})."\n";$n++}
 $s.="UEV3 END_FILE $role $n\n";push@manifest,join("\t",$role,$path,$len,$hash,$n);$count++}
 $s.='UEV3 END '.$Evidence::JSON->encode({files=>$count,manifest_sha256=>sha256_hex(join("\n",@manifest)."\n"),complete=>JSON::PP::true})."\n";
  return $s."UNICODE_PILOT_V3_TERMINAL provider=$p exit=0 cleanup=0 emission=0 post_resource=0\n";
}
sub receipt {
 my($c,$r)=@_;return{schema=>1,repository=>'a-coding-mage/linux',branch=>'feat/rust-translation-lupos',event=>'push',run_id=>$run,run_attempt=>1,trigger_sha=>$trigger,decoding=>'connector-decoded-text-utf8',jobs=>{c=>{job_id=>11,name=>'unicode-c',status=>'completed',conclusion=>'success',text_sha256=>sha256_hex($c)},rust=>{job_id=>12,name=>'unicode-rust',status=>'completed',conclusion=>'success',text_sha256=>sha256_hex($r)}}};
}
my$c=fixtures('c');my$r=fixtures('rust');my$clog=envelope($c,'c');my$rlog=envelope($r,'rust');
CompareEvidence::compare(receipt($clog,$rlog),$clog,$rlog,$dir);print "PASS synthetic complete evidence pair\n";
my$stamped=join('',map{"2026-10-02T00:00:00.1234567Z $_\n"}split/\n/,$rlog);CompareEvidence::compare(receipt($clog,$stamped),$clog,$stamped,$dir);print "PASS GitHub timestamp normalization with original decoded-text digest\n";
sub reject {
 my($name,$bad,$rec)=@_;$rec//=receipt($clog,$bad);my$pass=eval{CompareEvidence::compare($rec,$clog,$bad,$dir);1};die "ACCEPTED $name\n" if$pass;print "PASS rejects $name\n";
}
reject('old fabricated success markers',"END_COMPLETE_MEMBER_LOG provider=rust\nBEGIN_COMPLETE_MEMBER_LOG provider=rust\nCONTAINER_TERMINAL provider=rust exit=99\n");
reject('truncated envelope',substr($rlog,0,-100));
reject('duplicate envelope',$rlog.$rlog);
my$bad=$rlog;$bad=~s/exit=0 cleanup=0 emission=0/exit=99 cleanup=0 emission=0/;reject('explicit terminal failure',$bad);
$bad=$rlog;$bad=~s/UEV3 END_FILE host.pull.command 1/UEV3 END_FILE host.pull.command 2/;reject('wrong chunk count',$bad);
$bad=$rlog;$bad=~s/host\/pull.command/\/work\/not-evidence/;reject('arbitrary named path',$bad);
$bad=$rlog;$bad=~s/UEV3 DATA /UEV3 UNKNOWN /;reject('unknown data framing',$bad);
$bad=$rlog;$bad=~s/UEV3 BEGIN /UEV3 END /;reject('reversed framing',$bad);
$bad=$rlog;$bad=~s/UEV3 DATA \{"index":0,/UEV3 DATA {"index":1,/;reject('reordered chunk index',$bad);
$bad=$rlog;$bad=~s/"sha256":"[0-9a-f]{64}"/"sha256":"0000000000000000000000000000000000000000000000000000000000000000"/;reject('payload digest corruption',$bad);
$bad=$rlog;$bad=~s/UEV3 FILE [^\n]+\n.*?UEV3 END_FILE [^\n]+\n//s;reject('missing required role',$bad);
for my$case(
 ['incorrect earlier newline CID fixture','host.container.cid',('b'x64)."\n"],
 ['short CID','host.container.cid','b'x63],
 ['cleanup without observed absence','host.cleanup.inspect',"member_entry_exit=0\n"],
 ['cleanup of wrong container','host.cleanup.inspect','cleanup_verified_absent='.('c'x64)."\n"],
 ['nonhex CID','host.container.cid','g'x64],
 ['missing full normalizer command','runtime.norm.cmd',"source_fs/unicode/utf8-norm.o := /src/fs/unicode/utf8-norm.rs\n"],
 ['missing guest budget admission','runtime.guest-admission.json','{}'],
 ['failed telemetry status','runtime.build.status',"stage=build native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=79 timeout_seconds=1800\n"],
 ['changed guest stage allowance','runtime.guest.status',"stage=guest native_exit=0 supervisor_exit=0 cleanup_exit=0 interrupted=0 telemetry_exit=0 timeout_seconds=900\n"],
 ['failed build status','runtime.build.status',"stage=build native_exit=99 supervisor_exit=0 cleanup_exit=0 interrupted=0\n"],
 ['missing build completion','runtime.build.log',"success\n"],
 ['different original cases','runtime.guest.log',"ok 1 fake\n"],
 ['missing upstream JSON','runtime.upstream-result.json','{}'],
 ['ignored write failure','runtime.observer-parser.status',"observer_exit=1 parser_exit=0\n"],
 ['unproved ownership','runtime.ownership.log',"FINAL_ELF_OWNERSHIP_PASS\n"],
 ['missing source verification','runtime.source.before.txt',"SOURCE_VERIFIED=$Evidence::SOURCE\n"],
 ['arbitrary repeated object hashes','runtime.object-identities.sha256',('a'x64)."  /work/not-evidence\n"],
 ['missing full O inventory','runtime.O-before-noop.sha256',('a'x64)."  ./fake\n"],
 ['extra compiler flags','runtime.build.command',cmd('make','KCFLAGS=-O0','bzImage')],
 ['skipped security snapshot','runtime.apt-suites.tsv',"trixie\t0584fba32e13e0ab8285fb16c27adea1ec03a73669c18702821094fd6ca86675\n"],
 ['missing terminal capacity','host.container.resources.tsv',"phase\tutc\tfree_bytes\tavailable_memory\tfree_inodes\tO_bytes\tevidence_bytes\n"],
 ['failed cleanup','host.cleanup.status',"cleanup_exit=79\n"]
){my%copy=%$r;$copy{$case->[1]}=$case->[2];reject($case->[0],envelope(\%copy,'rust'))}
my$rec=receipt($clog,$rlog);$rec->{run_attempt}=2;reject('GitHub rerun receipt',$rlog,$rec);
$rec=receipt($clog,$rlog);$rec->{jobs}{rust}{conclusion}='failure';reject('failed GitHub job metadata',$rlog,$rec);
$rec=receipt($clog,$rlog);$rec->{jobs}{rust}{text_sha256}='0'x64;reject('decoded download digest mismatch',$rlog,$rec);
print "SYNTHETIC_PROTOCOL_TESTS_COMPLETE no_kernel_execution=true\n";
for my $case (
 ['bare compiler',sub { $_[0]=~s! /usr/bin/rustc ! rustc ! },'Rust compiler/overflow flags absent'],
 ['wrong compiler prefix',sub { $_[0]=~s! /usr/bin/rustc ! /wrong/usr/bin/rustc ! },'Rust compiler/overflow flags absent'],
 ['compiler text in environment value',sub { $_[0]=~s! /usr/bin/rustc ! COMPILER=/usr/bin/rustc /wrong/rustc ! },'Rust compiler/overflow flags absent'],
 ['C source selected as Rust',sub { $_[0]=~s!utf8-norm\.rs!utf8-norm.c!g },'wrong selected normalizer source'],
 ['disabled overflow checking',sub { $_[0]=~s/Coverflow-checks=y/Coverflow-checks=n/ },'overflow policy: conflicting/disabled setting']
) {
 my %copy=%$r; $case->[1]->($copy{'runtime.norm.cmd'});
 my $hash=sha256_hex(encode('UTF-8',$copy{'runtime.norm.cmd'}));
 for my $stage(qw(before-noop after-noop before-guest after-guest)) {
  $copy{"runtime.O-$stage.sha256"}=~s{^[0-9a-f]{64}  \./fs/unicode/\.utf8-norm\.o\.cmd$}{$hash  ./fs/unicode/.utf8-norm.o.cmd}m or die 'fixture inventory';
 }
 my $bad=envelope(\%copy,'rust');
 my $pass=eval {CompareEvidence::compare(receipt($clog,$bad),$clog,$bad,$dir);1};
 die "wrong rejection for $case->[0]: $@" if $pass || $@ !~ /\Q$case->[2]\E/;
 print "PASS rejects $case->[0] with rebound command digests\n";
}
for my $case (
 ['missing explicit compiler selection','runtime.build.command',sub {$_[0]=~s! RUSTC=/usr/bin/rustc!!}],
 ['wrong pinned compiler hash','runtime.tool-binaries.sha256',sub {$_[0]=~s!^[0-9a-f]{64}  /usr/bin/rustc$!('0'x64).'  /usr/bin/rustc'!me}],
 ['missing path equivalence','runtime.tool-versions.txt',sub {$_[0]=~s/^RUSTC_PATH_EQUIVALENCE[^\n]*\n//m}]
) {
 my %copy=%$r; $case->[2]->($copy{$case->[1]});reject($case->[0],envelope(\%copy,'rust'));
}
print "OWNERSHIP_PROPOSAL_CONTROLS_COMPLETE no_kernel_execution=true\n";
