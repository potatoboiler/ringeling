#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type LGL;
    fn lglgetenv(lgl: *mut LGL, opt: *mut Opt, lname: *const libc::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Opt {
    pub lng: *const libc::c_char,
    pub descrp: *const libc::c_char,
    pub val: libc::c_int,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub dflt: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Opts {
    pub beforefirst: Opt,
    pub abstime: Opt,
    pub activity: Opt,
    pub agility: Opt,
    pub agilitylim: Opt,
    pub bate: Opt,
    pub batewait: Opt,
    pub bca: Opt,
    pub bcaddlimldscale: Opt,
    pub bcamaxeff: Opt,
    pub bcaminuse: Opt,
    pub bcawait: Opt,
    pub bkwdocclim: Opt,
    pub bkwdresched: Opt,
    pub bkwdroundlim: Opt,
    pub bkwdscale: Opt,
    pub blkboost: Opt,
    pub blkboostvlim: Opt,
    pub blkclslim: Opt,
    pub blklarge: Opt,
    pub blkmaxeff: Opt,
    pub blkmineff: Opt,
    pub blkocclim1: Opt,
    pub blkocclim: Opt,
    pub blkocclim2: Opt,
    pub blkreleff: Opt,
    pub blkresched: Opt,
    pub blkrtc: Opt,
    pub blksmall: Opt,
    pub blksuccessmaxwortc: Opt,
    pub blksuccessrat: Opt,
    pub block: Opt,
    pub blockwait: Opt,
    pub boost: Opt,
    pub bumpreasonlits: Opt,
    pub bumpsimp: Opt,
    pub card: Opt,
    pub cardcut: Opt,
    pub cardexpam1: Opt,
    pub cardglue: Opt,
    pub cardignused: Opt,
    pub cardmaxeff: Opt,
    pub cardmaxlen: Opt,
    pub cardmineff: Opt,
    pub cardminlen: Opt,
    pub cardocclim1: Opt,
    pub cardocclim2: Opt,
    pub cardreleff: Opt,
    pub cardreschedint: Opt,
    pub carduse: Opt,
    pub cardwait: Opt,
    pub classify: Opt,
    pub cce2wait: Opt,
    pub cce: Opt,
    pub cce3wait: Opt,
    pub cceateint: Opt,
    pub cceboost: Opt,
    pub cceboostdel: Opt,
    pub cceboostint: Opt,
    pub cceboostvlim: Opt,
    pub ccemaxeff: Opt,
    pub ccemaxround: Opt,
    pub ccemineff: Opt,
    pub ccereleff: Opt,
    pub ccertc: Opt,
    pub ccertcint: Opt,
    pub ccertcintvlim: Opt,
    pub ccesuccessrat: Opt,
    pub ccewait: Opt,
    pub check: Opt,
    pub clim: Opt,
    pub compact: Opt,
    pub deco: Opt,
    pub deco1opt: Opt,
    pub decolim: Opt,
    pub decompose: Opt,
    pub defragfree: Opt,
    pub defragint: Opt,
    pub delmax: Opt,
    pub dlim: Opt,
    pub druplig: Opt,
    pub drupligcheck: Opt,
    pub drupligtrace: Opt,
    pub drupligtraceorig: Opt,
    pub elim: Opt,
    pub elmaxeff: Opt,
    pub elmblk: Opt,
    pub elmblkwait: Opt,
    pub elmboost: Opt,
    pub elmboostdel: Opt,
    pub elmboostint: Opt,
    pub elmboostvlim: Opt,
    pub elmclslim: Opt,
    pub elmfull: Opt,
    pub elmineff: Opt,
    pub elmlitslim: Opt,
    pub elmocclim1: Opt,
    pub elmocclim: Opt,
    pub elmocclim2: Opt,
    pub elmoccsumforced: Opt,
    pub elmreleff: Opt,
    pub elmresched: Opt,
    pub elmroundlim: Opt,
    pub elmrtc: Opt,
    pub elmrtcint: Opt,
    pub elmrtcintvlim: Opt,
    pub elmotfstr: Opt,
    pub elmotfsub: Opt,
    pub elmsuccessmaxwortc: Opt,
    pub elmsuccessrat: Opt,
    pub exitonabort: Opt,
    pub factmax: Opt,
    pub factor: Opt,
    pub features: Opt,
    pub gauss: Opt,
    pub gausscardweak: Opt,
    pub gaussexptrn: Opt,
    pub gaussextrall: Opt,
    pub gaussmaxeff: Opt,
    pub gaussmaxor: Opt,
    pub gaussmineff: Opt,
    pub gaussreleff: Opt,
    pub gausswait: Opt,
    pub gluekeep: Opt,
    pub gluekeepsize: Opt,
    pub gluemacdfast: Opt,
    pub gluemacdslow: Opt,
    pub gluemacdsmooth: Opt,
    pub gluescale: Opt,
    pub hbrdom: Opt,
    pub import: Opt,
    pub incredcint: Opt,
    pub incredconfslim: Opt,
    pub incsavevisits: Opt,
    pub inprocessing: Opt,
    pub irrlim: Opt,
    pub itsmacdfast: Opt,
    pub itsmacdslow: Opt,
    pub itsmacdsmooth: Opt,
    pub jlevelmacdfast: Opt,
    pub jlevelmacdslow: Opt,
    pub jlevelmacdsmooth: Opt,
    pub jwhred: Opt,
    pub keepmaxglue: Opt,
    pub keepmaxglueint: Opt,
    pub lhbr: Opt,
    pub lkhd: Opt,
    pub locs: Opt,
    pub locsbanner: Opt,
    pub locsboost: Opt,
    pub locscint: Opt,
    pub locsclim: Opt,
    pub locset: Opt,
    pub locsexport: Opt,
    pub locsmaxeff: Opt,
    pub locsmineff: Opt,
    pub locsred: Opt,
    pub locsreleff: Opt,
    pub locsrtc: Opt,
    pub locsvared: Opt,
    pub locswait: Opt,
    pub log: Opt,
    pub maxscaledglue: Opt,
    pub memlim: Opt,
    pub minimize: Opt,
    pub minlocalgluelim: Opt,
    pub minlocalsizelim: Opt,
    pub minrecgluelim: Opt,
    pub minrecsizelim: Opt,
    pub move_0: Opt,
    pub otfs: Opt,
    pub penmax: Opt,
    pub phase: Opt,
    pub phaseluckfactor: Opt,
    pub phaselucklim: Opt,
    pub phaseluckmaxround: Opt,
    pub phasesave: Opt,
    pub plain: Opt,
    pub plim: Opt,
    pub poison: Opt,
    pub prbasic: Opt,
    pub prbasicmaxeff: Opt,
    pub prbasicmineff: Opt,
    pub prbasicreleff: Opt,
    pub prbasicroundlim: Opt,
    pub prbasicrtc: Opt,
    pub prbrtc: Opt,
    pub prbsimple: Opt,
    pub prbsimpleboost: Opt,
    pub prbsimpleliftdepth: Opt,
    pub prbsimplemaxeff: Opt,
    pub prbsimplemineff: Opt,
    pub prbsimplereleff: Opt,
    pub prbsimplertc: Opt,
    pub probe: Opt,
    pub profile: Opt,
    pub profilelong: Opt,
    pub promote: Opt,
    pub promotegluelim: Opt,
    pub prune: Opt,
    pub pruneclim: Opt,
    pub pruneinit: Opt,
    pub prunelevel: Opt,
    pub prunesize: Opt,
    pub prunemin: Opt,
    pub prunemax: Opt,
    pub prunepure: Opt,
    pub prunered: Opt,
    pub prunevsids: Opt,
    pub pure_0: Opt,
    pub quatres: Opt,
    pub quatreswait: Opt,
    pub queuesort: Opt,
    pub randec: Opt,
    pub randecint: Opt,
    pub randphase: Opt,
    pub randphaseint: Opt,
    pub redcls: Opt,
    pub redclsglue: Opt,
    pub redclsize: Opt,
    pub redclsmaxdec: Opt,
    pub redclsmaxdepth: Opt,
    pub redclsmaxlrg: Opt,
    pub redclsmaxprops: Opt,
    pub redclstype: Opt,
    pub reduce: Opt,
    pub reducefixed: Opt,
    pub reduceinc: Opt,
    pub reduceinit: Opt,
    pub reducereset: Opt,
    pub restart: Opt,
    pub restartfixed: Opt,
    pub restartblock: Opt,
    pub restartblocklim: Opt,
    pub restartblockbound: Opt,
    pub restartcheckforced: Opt,
    pub restartdelay: Opt,
    pub restartdelaylim: Opt,
    pub restartint: Opt,
    pub restartforcelim: Opt,
    pub restartforcemode: Opt,
    pub restartpen1: Opt,
    pub restartpen2: Opt,
    pub restartpen3: Opt,
    pub restartpenstab: Opt,
    pub retireint: Opt,
    pub retiremin: Opt,
    pub retirenb: Opt,
    pub reusetrail: Opt,
    pub rmincpen: Opt,
    pub scincinc: Opt,
    pub scincincdelta: Opt,
    pub scincincincint: Opt,
    pub scincincmin: Opt,
    pub scincincmode: Opt,
    pub scoreshift: Opt,
    pub seed: Opt,
    pub simpbintinc: Opt,
    pub simpbintinclim: Opt,
    pub simpcintdelay: Opt,
    pub simpcintinc: Opt,
    pub simpcintincdiv: Opt,
    pub simpcintmaxhard: Opt,
    pub simpcintmaxsoft: Opt,
    pub simpidiv: Opt,
    pub simpincdelmaxfact: Opt,
    pub simpincdelmaxmin: Opt,
    pub simpinitdelay: Opt,
    pub simpintsizepen: Opt,
    pub simpiscale: Opt,
    pub simpitdelay: Opt,
    pub simpjleveldecdelay: Opt,
    pub simpitintdecdelay: Opt,
    pub simpitintinc: Opt,
    pub simpitintinclim: Opt,
    pub simplify: Opt,
    pub simprtc: Opt,
    pub simptintinc: Opt,
    pub simptintinclim: Opt,
    pub simpvarchg: Opt,
    pub simpvarlim: Opt,
    pub sizemaxpen: Opt,
    pub sizepen: Opt,
    pub sleeponabort: Opt,
    pub smallirr: Opt,
    pub smallve: Opt,
    pub smallvevars: Opt,
    pub smallvewait: Opt,
    pub sortlits: Opt,
    pub stabema: Opt,
    pub subl: Opt,
    pub sweep: Opt,
    pub sweepboost: Opt,
    pub sweepboostdel: Opt,
    pub sweepboostint: Opt,
    pub sweepboostvlim: Opt,
    pub sweepfacdec: Opt,
    pub sweepforgive: Opt,
    pub sweepirr: Opt,
    pub sweepmaxdec: Opt,
    pub sweepmaxeff: Opt,
    pub sweepmaxround: Opt,
    pub sweepmindec: Opt,
    pub sweepmineff: Opt,
    pub sweepred: Opt,
    pub sweepreleff: Opt,
    pub sweeprtc: Opt,
    pub sweeprtcint: Opt,
    pub sweeprtcintvlim: Opt,
    pub sweepsuccessmaxwortc: Opt,
    pub sweepsuccessrat: Opt,
    pub sweepwait: Opt,
    pub synclsall: Opt,
    pub synclsglue: Opt,
    pub synclsint: Opt,
    pub synclslen: Opt,
    pub syncunint: Opt,
    pub termint: Opt,
    pub ternres: Opt,
    pub ternresboost: Opt,
    pub ternresrtc: Opt,
    pub ternreswait: Opt,
    pub tlevelema: Opt,
    pub transred: Opt,
    pub transredwait: Opt,
    pub trapiflush: Opt,
    pub trdmaxeff: Opt,
    pub trdmineff: Opt,
    pub trdreleff: Opt,
    pub treelook: Opt,
    pub treelookboost: Opt,
    pub treelookfull: Opt,
    pub treelookmaxeff: Opt,
    pub treelookmineff: Opt,
    pub treelookreleff: Opt,
    pub treelookrtc: Opt,
    pub trep: Opt,
    pub trepint: Opt,
    pub trnreleff: Opt,
    pub trnrmaxeff: Opt,
    pub trnrmineff: Opt,
    pub unhdatrn: Opt,
    pub unhdextstamp: Opt,
    pub unhdhbr: Opt,
    pub unhdlnpr: Opt,
    pub unhdmaxeff: Opt,
    pub unhdmineff: Opt,
    pub unhdreleff: Opt,
    pub unhdroundlim: Opt,
    pub unhide: Opt,
    pub unhidewait: Opt,
    pub usedtwice: Opt,
    pub verbose: Opt,
    pub wait: Opt,
    pub waitmax: Opt,
    pub witness: Opt,
    pub afterlast: Opt,
}
#[no_mangle]
pub unsafe extern "C" fn lglinitopts(mut lgl: *mut LGL, mut opts: *mut Opts) {
    let K: libc::c_int = 1000 as libc::c_int;
    let M: libc::c_int = K * K;
    let I: libc::c_int = 2147483647 as libc::c_int;
    let MG: libc::c_int = ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int;
    let mut opt: *mut Opt = &mut (*opts).abstime;
    (*opt).lng = b"abstime\0" as *const u8 as *const libc::c_char;
    (*opt).val = 0 as libc::c_int;
    (*opt).dflt = (*opt).val;
    (*opt).min = 0 as libc::c_int;
    (*opt).max = 1 as libc::c_int;
    (*opt)
        .descrp = b"print absolute time when reporting\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt, b"abstime\0" as *const u8 as *const libc::c_char);
    let mut opt_0: *mut Opt = &mut (*opts).activity;
    (*opt_0).lng = b"activity\0" as *const u8 as *const libc::c_char;
    (*opt_0).val = 0 as libc::c_int;
    (*opt_0).dflt = (*opt_0).val;
    (*opt_0).min = 0 as libc::c_int;
    (*opt_0).max = 2 as libc::c_int;
    (*opt_0)
        .descrp = b"activity based clause reduction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_0, b"activity\0" as *const u8 as *const libc::c_char);
    let mut opt_1: *mut Opt = &mut (*opts).agility;
    (*opt_1).lng = b"agility\0" as *const u8 as *const libc::c_char;
    (*opt_1).val = 1 as libc::c_int;
    (*opt_1).dflt = (*opt_1).val;
    (*opt_1).min = 0 as libc::c_int;
    (*opt_1).max = 1 as libc::c_int;
    (*opt_1)
        .descrp = b"enable agility based restart skipping\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_1, b"agility\0" as *const u8 as *const libc::c_char);
    let mut opt_2: *mut Opt = &mut (*opts).agilitylim;
    (*opt_2).lng = b"agilitylim\0" as *const u8 as *const libc::c_char;
    (*opt_2).val = 70 as libc::c_int;
    (*opt_2).dflt = (*opt_2).val;
    (*opt_2).min = 0 as libc::c_int;
    (*opt_2).max = 100 as libc::c_int;
    (*opt_2)
        .descrp = b"agility limit for restarts in percent\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_2, b"agilitylim\0" as *const u8 as *const libc::c_char);
    let mut opt_3: *mut Opt = &mut (*opts).bate;
    (*opt_3).lng = b"bate\0" as *const u8 as *const libc::c_char;
    (*opt_3).val = 1 as libc::c_int;
    (*opt_3).dflt = (*opt_3).val;
    (*opt_3).min = 0 as libc::c_int;
    (*opt_3).max = 1 as libc::c_int;
    (*opt_3)
        .descrp = b"basic ATE removal during probing\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_3, b"bate\0" as *const u8 as *const libc::c_char);
    let mut opt_4: *mut Opt = &mut (*opts).batewait;
    (*opt_4).lng = b"batewait\0" as *const u8 as *const libc::c_char;
    (*opt_4).val = 2 as libc::c_int;
    (*opt_4).dflt = (*opt_4).val;
    (*opt_4).min = 0 as libc::c_int;
    (*opt_4).max = 2 as libc::c_int;
    (*opt_4)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_4, b"batewait\0" as *const u8 as *const libc::c_char);
    let mut opt_5: *mut Opt = &mut (*opts).bca;
    (*opt_5).lng = b"bca\0" as *const u8 as *const libc::c_char;
    (*opt_5).val = 0 as libc::c_int;
    (*opt_5).dflt = (*opt_5).val;
    (*opt_5).min = 0 as libc::c_int;
    (*opt_5).max = 2 as libc::c_int;
    (*opt_5)
        .descrp = b"enable blocked clause addition (1=weak,2=strong)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_5, b"bca\0" as *const u8 as *const libc::c_char);
    let mut opt_6: *mut Opt = &mut (*opts).bcaddlimldscale;
    (*opt_6).lng = b"bcaddlimldscale\0" as *const u8 as *const libc::c_char;
    (*opt_6).val = 2 as libc::c_int;
    (*opt_6).dflt = (*opt_6).val;
    (*opt_6).min = -(7 as libc::c_int);
    (*opt_6).max = 7 as libc::c_int;
    (*opt_6)
        .descrp = b"bca added clause limit ld scale\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_6, b"bcaddlimldscale\0" as *const u8 as *const libc::c_char);
    let mut opt_7: *mut Opt = &mut (*opts).bcamaxeff;
    (*opt_7).lng = b"bcamaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_7).val = 10 as libc::c_int * M;
    (*opt_7).dflt = (*opt_7).val;
    (*opt_7).min = 0 as libc::c_int;
    (*opt_7).max = I;
    (*opt_7)
        .descrp = b"BCA maximum number of steps\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_7, b"bcamaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_8: *mut Opt = &mut (*opts).bcaminuse;
    (*opt_8).lng = b"bcaminuse\0" as *const u8 as *const libc::c_char;
    (*opt_8).val = 100 as libc::c_int;
    (*opt_8).dflt = (*opt_8).val;
    (*opt_8).min = 0 as libc::c_int;
    (*opt_8).max = I;
    (*opt_8)
        .descrp = b"min number of literals required to be usable\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_8, b"bcaminuse\0" as *const u8 as *const libc::c_char);
    let mut opt_9: *mut Opt = &mut (*opts).bcawait;
    (*opt_9).lng = b"bcawait\0" as *const u8 as *const libc::c_char;
    (*opt_9).val = 2 as libc::c_int;
    (*opt_9).dflt = (*opt_9).val;
    (*opt_9).min = 0 as libc::c_int;
    (*opt_9).max = 2 as libc::c_int;
    (*opt_9)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_9, b"bcawait\0" as *const u8 as *const libc::c_char);
    let mut opt_10: *mut Opt = &mut (*opts).bkwdocclim;
    (*opt_10).lng = b"bkwdocclim\0" as *const u8 as *const libc::c_char;
    (*opt_10).val = 100 as libc::c_int;
    (*opt_10).dflt = (*opt_10).val;
    (*opt_10).min = 0 as libc::c_int;
    (*opt_10).max = I;
    (*opt_10)
        .descrp = b"backward occurrence limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_10, b"bkwdocclim\0" as *const u8 as *const libc::c_char);
    let mut opt_11: *mut Opt = &mut (*opts).bkwdresched;
    (*opt_11).lng = b"bkwdresched\0" as *const u8 as *const libc::c_char;
    (*opt_11).val = 1 as libc::c_int;
    (*opt_11).dflt = (*opt_11).val;
    (*opt_11).min = 0 as libc::c_int;
    (*opt_11).max = 1 as libc::c_int;
    (*opt_11)
        .descrp = b"reschedule variables during backward\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_11, b"bkwdresched\0" as *const u8 as *const libc::c_char);
    let mut opt_12: *mut Opt = &mut (*opts).bkwdroundlim;
    (*opt_12).lng = b"bkwdroundlim\0" as *const u8 as *const libc::c_char;
    (*opt_12).val = 7 as libc::c_int;
    (*opt_12).dflt = (*opt_12).val;
    (*opt_12).min = 1 as libc::c_int;
    (*opt_12).max = I;
    (*opt_12).descrp = b"bkwd round limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_12, b"bkwdroundlim\0" as *const u8 as *const libc::c_char);
    let mut opt_13: *mut Opt = &mut (*opts).bkwdscale;
    (*opt_13).lng = b"bkwdscale\0" as *const u8 as *const libc::c_char;
    (*opt_13).val = 100 as libc::c_int;
    (*opt_13).dflt = (*opt_13).val;
    (*opt_13).min = 1 as libc::c_int;
    (*opt_13).max = 10000 as libc::c_int;
    (*opt_13)
        .descrp = b"bkwd steps vs elm steps in percent\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_13, b"bkwdscale\0" as *const u8 as *const libc::c_char);
    let mut opt_14: *mut Opt = &mut (*opts).blkboost;
    (*opt_14).lng = b"blkboost\0" as *const u8 as *const libc::c_char;
    (*opt_14).val = 10 as libc::c_int;
    (*opt_14).dflt = (*opt_14).val;
    (*opt_14).min = 1 as libc::c_int;
    (*opt_14).max = 10000 as libc::c_int;
    (*opt_14).descrp = b"initial BCE boost\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_14, b"blkboost\0" as *const u8 as *const libc::c_char);
    let mut opt_15: *mut Opt = &mut (*opts).blkboostvlim;
    (*opt_15).lng = b"blkboostvlim\0" as *const u8 as *const libc::c_char;
    (*opt_15).val = 10 as libc::c_int * M;
    (*opt_15).dflt = (*opt_15).val;
    (*opt_15).min = 0 as libc::c_int;
    (*opt_15).max = I;
    (*opt_15).descrp = b"BCE boost variable limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_15, b"blkboostvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_16: *mut Opt = &mut (*opts).blkclslim;
    (*opt_16).lng = b"blkclslim\0" as *const u8 as *const libc::c_char;
    (*opt_16).val = 1 as libc::c_int * M;
    (*opt_16).dflt = (*opt_16).val;
    (*opt_16).min = 3 as libc::c_int;
    (*opt_16).max = I;
    (*opt_16).descrp = b"max blocked clause size\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_16, b"blkclslim\0" as *const u8 as *const libc::c_char);
    let mut opt_17: *mut Opt = &mut (*opts).blklarge;
    (*opt_17).lng = b"blklarge\0" as *const u8 as *const libc::c_char;
    (*opt_17).val = 1 as libc::c_int;
    (*opt_17).dflt = (*opt_17).val;
    (*opt_17).min = 0 as libc::c_int;
    (*opt_17).max = 1 as libc::c_int;
    (*opt_17).descrp = b"BCE of large clauses\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_17, b"blklarge\0" as *const u8 as *const libc::c_char);
    let mut opt_18: *mut Opt = &mut (*opts).blkmaxeff;
    (*opt_18).lng = b"blkmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_18).val = 800 as libc::c_int * M;
    (*opt_18).dflt = (*opt_18).val;
    (*opt_18).min = -(1 as libc::c_int);
    (*opt_18).max = I;
    (*opt_18)
        .descrp = b"max effort in BCE (-1=unlimited)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_18, b"blkmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_19: *mut Opt = &mut (*opts).blkmineff;
    (*opt_19).lng = b"blkmineff\0" as *const u8 as *const libc::c_char;
    (*opt_19).val = 50 as libc::c_int * M;
    (*opt_19).dflt = (*opt_19).val;
    (*opt_19).min = 0 as libc::c_int;
    (*opt_19).max = I;
    (*opt_19).descrp = b"min effort in BCE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_19, b"blkmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_20: *mut Opt = &mut (*opts).blkocclim1;
    (*opt_20).lng = b"blkocclim1\0" as *const u8 as *const libc::c_char;
    (*opt_20).val = 100 as libc::c_int * K;
    (*opt_20).dflt = (*opt_20).val;
    (*opt_20).min = 1 as libc::c_int;
    (*opt_20).max = I;
    (*opt_20).descrp = b"one-sided max occ of BCE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_20, b"blkocclim1\0" as *const u8 as *const libc::c_char);
    let mut opt_21: *mut Opt = &mut (*opts).blkocclim;
    (*opt_21).lng = b"blkocclim\0" as *const u8 as *const libc::c_char;
    (*opt_21).val = 1 as libc::c_int * M;
    (*opt_21).dflt = (*opt_21).val;
    (*opt_21).min = 3 as libc::c_int;
    (*opt_21).max = I;
    (*opt_21).descrp = b"max occ in BCE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_21, b"blkocclim\0" as *const u8 as *const libc::c_char);
    let mut opt_22: *mut Opt = &mut (*opts).blkocclim2;
    (*opt_22).lng = b"blkocclim2\0" as *const u8 as *const libc::c_char;
    (*opt_22).val = 10 as libc::c_int * K;
    (*opt_22).dflt = (*opt_22).val;
    (*opt_22).min = 2 as libc::c_int;
    (*opt_22).max = I;
    (*opt_22).descrp = b"two-sided max occ of BCE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_22, b"blkocclim2\0" as *const u8 as *const libc::c_char);
    let mut opt_23: *mut Opt = &mut (*opts).blkreleff;
    (*opt_23).lng = b"blkreleff\0" as *const u8 as *const libc::c_char;
    (*opt_23).val = 100 as libc::c_int;
    (*opt_23).dflt = (*opt_23).val;
    (*opt_23).min = 0 as libc::c_int;
    (*opt_23).max = K;
    (*opt_23).descrp = b"rel effort in BCE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_23, b"blkreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_24: *mut Opt = &mut (*opts).blkresched;
    (*opt_24).lng = b"blkresched\0" as *const u8 as *const libc::c_char;
    (*opt_24).val = 1 as libc::c_int;
    (*opt_24).dflt = (*opt_24).val;
    (*opt_24).min = 0 as libc::c_int;
    (*opt_24).max = 1 as libc::c_int;
    (*opt_24)
        .descrp = b"reschedule tried but failed literals\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_24, b"blkresched\0" as *const u8 as *const libc::c_char);
    let mut opt_25: *mut Opt = &mut (*opts).blkrtc;
    (*opt_25).lng = b"blkrtc\0" as *const u8 as *const libc::c_char;
    (*opt_25).val = 0 as libc::c_int;
    (*opt_25).dflt = (*opt_25).val;
    (*opt_25).min = 0 as libc::c_int;
    (*opt_25).max = 1 as libc::c_int;
    (*opt_25).descrp = b"run BCE until completion\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_25, b"blkrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_26: *mut Opt = &mut (*opts).blksmall;
    (*opt_26).lng = b"blksmall\0" as *const u8 as *const libc::c_char;
    (*opt_26).val = 1 as libc::c_int;
    (*opt_26).dflt = (*opt_26).val;
    (*opt_26).min = 0 as libc::c_int;
    (*opt_26).max = 1 as libc::c_int;
    (*opt_26).descrp = b"BCE of small clauses\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_26, b"blksmall\0" as *const u8 as *const libc::c_char);
    let mut opt_27: *mut Opt = &mut (*opts).blksuccessmaxwortc;
    (*opt_27).lng = b"blksuccessmaxwortc\0" as *const u8 as *const libc::c_char;
    (*opt_27).val = 6 as libc::c_int;
    (*opt_27).dflt = (*opt_27).val;
    (*opt_27).min = 1 as libc::c_int;
    (*opt_27).max = I;
    (*opt_27)
        .descrp = b"BCE success max without run-to-completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_27, b"blksuccessmaxwortc\0" as *const u8 as *const libc::c_char);
    let mut opt_28: *mut Opt = &mut (*opts).blksuccessrat;
    (*opt_28).lng = b"blksuccessrat\0" as *const u8 as *const libc::c_char;
    (*opt_28).val = 100 as libc::c_int;
    (*opt_28).dflt = (*opt_28).val;
    (*opt_28).min = 1 as libc::c_int;
    (*opt_28).max = I;
    (*opt_28).descrp = b"BCE success ratio\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_28, b"blksuccessrat\0" as *const u8 as *const libc::c_char);
    let mut opt_29: *mut Opt = &mut (*opts).block;
    (*opt_29).lng = b"block\0" as *const u8 as *const libc::c_char;
    (*opt_29).val = 0 as libc::c_int;
    (*opt_29).dflt = (*opt_29).val;
    (*opt_29).min = 0 as libc::c_int;
    (*opt_29).max = 1 as libc::c_int;
    (*opt_29)
        .descrp = b"blocked clause elimination (BCE)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_29, b"block\0" as *const u8 as *const libc::c_char);
    let mut opt_30: *mut Opt = &mut (*opts).blockwait;
    (*opt_30).lng = b"blockwait\0" as *const u8 as *const libc::c_char;
    (*opt_30).val = 1 as libc::c_int;
    (*opt_30).dflt = (*opt_30).val;
    (*opt_30).min = 0 as libc::c_int;
    (*opt_30).max = 1 as libc::c_int;
    (*opt_30).descrp = b"wait for BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_30, b"blockwait\0" as *const u8 as *const libc::c_char);
    let mut opt_31: *mut Opt = &mut (*opts).boost;
    (*opt_31).lng = b"boost\0" as *const u8 as *const libc::c_char;
    (*opt_31).val = 1 as libc::c_int;
    (*opt_31).dflt = (*opt_31).val;
    (*opt_31).min = 0 as libc::c_int;
    (*opt_31).max = 1 as libc::c_int;
    (*opt_31)
        .descrp = b"enable boosting of preprocessors\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_31, b"boost\0" as *const u8 as *const libc::c_char);
    let mut opt_32: *mut Opt = &mut (*opts).bumpreasonlits;
    (*opt_32).lng = b"bumpreasonlits\0" as *const u8 as *const libc::c_char;
    (*opt_32).val = 1 as libc::c_int;
    (*opt_32).dflt = (*opt_32).val;
    (*opt_32).min = 0 as libc::c_int;
    (*opt_32).max = 1 as libc::c_int;
    (*opt_32).descrp = b"bump reason literals too\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_32, b"bumpreasonlits\0" as *const u8 as *const libc::c_char);
    let mut opt_33: *mut Opt = &mut (*opts).bumpsimp;
    (*opt_33).lng = b"bumpsimp\0" as *const u8 as *const libc::c_char;
    (*opt_33).val = 0 as libc::c_int;
    (*opt_33).dflt = (*opt_33).val;
    (*opt_33).min = 0 as libc::c_int;
    (*opt_33).max = 1 as libc::c_int;
    (*opt_33)
        .descrp = b"bump during simplification too\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_33, b"bumpsimp\0" as *const u8 as *const libc::c_char);
    let mut opt_34: *mut Opt = &mut (*opts).card;
    (*opt_34).lng = b"card\0" as *const u8 as *const libc::c_char;
    (*opt_34).val = 1 as libc::c_int;
    (*opt_34).dflt = (*opt_34).val;
    (*opt_34).min = 0 as libc::c_int;
    (*opt_34).max = 1 as libc::c_int;
    (*opt_34)
        .descrp = b"cardinality constraint reasoning\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_34, b"card\0" as *const u8 as *const libc::c_char);
    let mut opt_35: *mut Opt = &mut (*opts).cardcut;
    (*opt_35).lng = b"cardcut\0" as *const u8 as *const libc::c_char;
    (*opt_35).val = 2 as libc::c_int;
    (*opt_35).dflt = (*opt_35).val;
    (*opt_35).min = 0 as libc::c_int;
    (*opt_35).max = 2 as libc::c_int;
    (*opt_35)
        .descrp = b"1=gomoroy-cuts,2=strengthen\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_35, b"cardcut\0" as *const u8 as *const libc::c_char);
    let mut opt_36: *mut Opt = &mut (*opts).cardexpam1;
    (*opt_36).lng = b"cardexpam1\0" as *const u8 as *const libc::c_char;
    (*opt_36).val = 3 as libc::c_int;
    (*opt_36).dflt = (*opt_36).val;
    (*opt_36).min = 2 as libc::c_int;
    (*opt_36).max = I;
    (*opt_36)
        .descrp = b"min length of exported at-most-one constraint\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_36, b"cardexpam1\0" as *const u8 as *const libc::c_char);
    let mut opt_37: *mut Opt = &mut (*opts).cardglue;
    (*opt_37).lng = b"cardglue\0" as *const u8 as *const libc::c_char;
    (*opt_37).val = 0 as libc::c_int;
    (*opt_37).dflt = (*opt_37).val;
    (*opt_37).min = -(1 as libc::c_int);
    (*opt_37).max = ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int;
    (*opt_37)
        .descrp = b"use lrg red cls too (-1=irr,0=moved,...)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_37, b"cardglue\0" as *const u8 as *const libc::c_char);
    let mut opt_38: *mut Opt = &mut (*opts).cardignused;
    (*opt_38).lng = b"cardignused\0" as *const u8 as *const libc::c_char;
    (*opt_38).val = 0 as libc::c_int;
    (*opt_38).dflt = (*opt_38).val;
    (*opt_38).min = 0 as libc::c_int;
    (*opt_38).max = 1 as libc::c_int;
    (*opt_38)
        .descrp = b"ignore already used literals in extraction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_38, b"cardignused\0" as *const u8 as *const libc::c_char);
    let mut opt_39: *mut Opt = &mut (*opts).cardmaxeff;
    (*opt_39).lng = b"cardmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_39).val = 300 as libc::c_int * M;
    (*opt_39).dflt = (*opt_39).val;
    (*opt_39).min = -(1 as libc::c_int);
    (*opt_39).max = I;
    (*opt_39)
        .descrp = b"max effort for cardmineff reasoning\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_39, b"cardmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_40: *mut Opt = &mut (*opts).cardmaxlen;
    (*opt_40).lng = b"cardmaxlen\0" as *const u8 as *const libc::c_char;
    (*opt_40).val = 1000 as libc::c_int;
    (*opt_40).dflt = (*opt_40).val;
    (*opt_40).min = 0 as libc::c_int;
    (*opt_40).max = I;
    (*opt_40)
        .descrp = b"maximal length of cardinality constraints\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_40, b"cardmaxlen\0" as *const u8 as *const libc::c_char);
    let mut opt_41: *mut Opt = &mut (*opts).cardmineff;
    (*opt_41).lng = b"cardmineff\0" as *const u8 as *const libc::c_char;
    (*opt_41).val = 2 as libc::c_int * M;
    (*opt_41).dflt = (*opt_41).val;
    (*opt_41).min = 0 as libc::c_int;
    (*opt_41).max = I;
    (*opt_41)
        .descrp = b"min effort for cardmineff reasoning\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_41, b"cardmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_42: *mut Opt = &mut (*opts).cardminlen;
    (*opt_42).lng = b"cardminlen\0" as *const u8 as *const libc::c_char;
    (*opt_42).val = 3 as libc::c_int;
    (*opt_42).dflt = (*opt_42).val;
    (*opt_42).min = 0 as libc::c_int;
    (*opt_42).max = I;
    (*opt_42)
        .descrp = b"minimal length of (initial) card constraints\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_42, b"cardminlen\0" as *const u8 as *const libc::c_char);
    let mut opt_43: *mut Opt = &mut (*opts).cardocclim1;
    (*opt_43).lng = b"cardocclim1\0" as *const u8 as *const libc::c_char;
    (*opt_43).val = 300 as libc::c_int;
    (*opt_43).dflt = (*opt_43).val;
    (*opt_43).min = 0 as libc::c_int;
    (*opt_43).max = I;
    (*opt_43)
        .descrp = b"one-sided cardinality constraints occ limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_43, b"cardocclim1\0" as *const u8 as *const libc::c_char);
    let mut opt_44: *mut Opt = &mut (*opts).cardocclim2;
    (*opt_44).lng = b"cardocclim2\0" as *const u8 as *const libc::c_char;
    (*opt_44).val = 15 as libc::c_int;
    (*opt_44).dflt = (*opt_44).val;
    (*opt_44).min = 0 as libc::c_int;
    (*opt_44).max = I;
    (*opt_44)
        .descrp = b"two-sided cardinality constraints occ limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_44, b"cardocclim2\0" as *const u8 as *const libc::c_char);
    let mut opt_45: *mut Opt = &mut (*opts).cardreleff;
    (*opt_45).lng = b"cardreleff\0" as *const u8 as *const libc::c_char;
    (*opt_45).val = 5 as libc::c_int;
    (*opt_45).dflt = (*opt_45).val;
    (*opt_45).min = 0 as libc::c_int;
    (*opt_45).max = 10 as libc::c_int * K;
    (*opt_45)
        .descrp = b"rel effort for cardinality reasoning\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_45, b"cardreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_46: *mut Opt = &mut (*opts).cardreschedint;
    (*opt_46).lng = b"cardreschedint\0" as *const u8 as *const libc::c_char;
    (*opt_46).val = 10 as libc::c_int;
    (*opt_46).dflt = (*opt_46).val;
    (*opt_46).min = 1 as libc::c_int;
    (*opt_46).max = I;
    (*opt_46)
        .descrp = b"reschedule variable for card reasoning\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_46, b"cardreschedint\0" as *const u8 as *const libc::c_char);
    let mut opt_47: *mut Opt = &mut (*opts).carduse;
    (*opt_47).lng = b"carduse\0" as *const u8 as *const libc::c_char;
    (*opt_47).val = 2 as libc::c_int;
    (*opt_47).dflt = (*opt_47).val;
    (*opt_47).min = 0 as libc::c_int;
    (*opt_47).max = 3 as libc::c_int;
    (*opt_47)
        .descrp = b"use clauses (1=oneside,2=bothsidetoo,3=anyside)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_47, b"carduse\0" as *const u8 as *const libc::c_char);
    let mut opt_48: *mut Opt = &mut (*opts).cardwait;
    (*opt_48).lng = b"cardwait\0" as *const u8 as *const libc::c_char;
    (*opt_48).val = 0 as libc::c_int;
    (*opt_48).dflt = (*opt_48).val;
    (*opt_48).min = 0 as libc::c_int;
    (*opt_48).max = 2 as libc::c_int;
    (*opt_48)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_48, b"cardwait\0" as *const u8 as *const libc::c_char);
    let mut opt_49: *mut Opt = &mut (*opts).classify;
    (*opt_49).lng = b"classify\0" as *const u8 as *const libc::c_char;
    (*opt_49).val = 2 as libc::c_int;
    (*opt_49).dflt = (*opt_49).val;
    (*opt_49).min = 0 as libc::c_int;
    (*opt_49).max = 3 as libc::c_int;
    (*opt_49)
        .descrp = b"classifier for parameter setting\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_49, b"classify\0" as *const u8 as *const libc::c_char);
    let mut opt_50: *mut Opt = &mut (*opts).cce2wait;
    (*opt_50).lng = b"cce2wait\0" as *const u8 as *const libc::c_char;
    (*opt_50).val = 1 as libc::c_int;
    (*opt_50).dflt = (*opt_50).val;
    (*opt_50).min = 0 as libc::c_int;
    (*opt_50).max = I;
    (*opt_50)
        .descrp = b"wait for ATE to finish before doing ABCE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_50, b"cce2wait\0" as *const u8 as *const libc::c_char);
    let mut opt_51: *mut Opt = &mut (*opts).cce;
    (*opt_51).lng = b"cce\0" as *const u8 as *const libc::c_char;
    (*opt_51).val = 3 as libc::c_int;
    (*opt_51).dflt = (*opt_51).val;
    (*opt_51).min = 0 as libc::c_int;
    (*opt_51).max = 3 as libc::c_int;
    (*opt_51)
        .descrp = b"covered clause elimination (1=ate,2=abce,3=acce)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_51, b"cce\0" as *const u8 as *const libc::c_char);
    let mut opt_52: *mut Opt = &mut (*opts).cce3wait;
    (*opt_52).lng = b"cce3wait\0" as *const u8 as *const libc::c_char;
    (*opt_52).val = 2 as libc::c_int;
    (*opt_52).dflt = (*opt_52).val;
    (*opt_52).min = 0 as libc::c_int;
    (*opt_52).max = I;
    (*opt_52)
        .descrp = b"wait for ABCE to finish before doing ACCE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_52, b"cce3wait\0" as *const u8 as *const libc::c_char);
    let mut opt_53: *mut Opt = &mut (*opts).cceateint;
    (*opt_53).lng = b"cceateint\0" as *const u8 as *const libc::c_char;
    (*opt_53).val = 2 as libc::c_int;
    (*opt_53).dflt = (*opt_53).val;
    (*opt_53).min = 1 as libc::c_int;
    (*opt_53).max = I;
    (*opt_53).descrp = b"frequency of only ATE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_53, b"cceateint\0" as *const u8 as *const libc::c_char);
    let mut opt_54: *mut Opt = &mut (*opts).cceboost;
    (*opt_54).lng = b"cceboost\0" as *const u8 as *const libc::c_char;
    (*opt_54).val = 10 as libc::c_int;
    (*opt_54).dflt = (*opt_54).val;
    (*opt_54).min = 1 as libc::c_int;
    (*opt_54).max = 1000 as libc::c_int;
    (*opt_54).descrp = b"CCE boost\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_54, b"cceboost\0" as *const u8 as *const libc::c_char);
    let mut opt_55: *mut Opt = &mut (*opts).cceboostdel;
    (*opt_55).lng = b"cceboostdel\0" as *const u8 as *const libc::c_char;
    (*opt_55).val = 3 as libc::c_int;
    (*opt_55).dflt = (*opt_55).val;
    (*opt_55).min = 0 as libc::c_int;
    (*opt_55).max = 100 as libc::c_int;
    (*opt_55).descrp = b"initial CCE boost delay\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_55, b"cceboostdel\0" as *const u8 as *const libc::c_char);
    let mut opt_56: *mut Opt = &mut (*opts).cceboostint;
    (*opt_56).lng = b"cceboostint\0" as *const u8 as *const libc::c_char;
    (*opt_56).val = 5 as libc::c_int;
    (*opt_56).dflt = (*opt_56).val;
    (*opt_56).min = 1 as libc::c_int;
    (*opt_56).max = I;
    (*opt_56).descrp = b"CCE boost interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_56, b"cceboostint\0" as *const u8 as *const libc::c_char);
    let mut opt_57: *mut Opt = &mut (*opts).cceboostvlim;
    (*opt_57).lng = b"cceboostvlim\0" as *const u8 as *const libc::c_char;
    (*opt_57).val = 10 as libc::c_int * M;
    (*opt_57).dflt = (*opt_57).val;
    (*opt_57).min = 0 as libc::c_int;
    (*opt_57).max = I;
    (*opt_57).descrp = b"CCE boost variable limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_57, b"cceboostvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_58: *mut Opt = &mut (*opts).ccemaxeff;
    (*opt_58).lng = b"ccemaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_58).val = I;
    (*opt_58).dflt = (*opt_58).val;
    (*opt_58).min = -(1 as libc::c_int);
    (*opt_58).max = I;
    (*opt_58)
        .descrp = b"max effort in covered clause elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_58, b"ccemaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_59: *mut Opt = &mut (*opts).ccemaxround;
    (*opt_59).lng = b"ccemaxround\0" as *const u8 as *const libc::c_char;
    (*opt_59).val = 3 as libc::c_int;
    (*opt_59).dflt = (*opt_59).val;
    (*opt_59).min = 0 as libc::c_int;
    (*opt_59).max = I;
    (*opt_59).descrp = b"cce maximum rounds\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_59, b"ccemaxround\0" as *const u8 as *const libc::c_char);
    let mut opt_60: *mut Opt = &mut (*opts).ccemineff;
    (*opt_60).lng = b"ccemineff\0" as *const u8 as *const libc::c_char;
    (*opt_60).val = 30 as libc::c_int * M;
    (*opt_60).dflt = (*opt_60).val;
    (*opt_60).min = 0 as libc::c_int;
    (*opt_60).max = I;
    (*opt_60)
        .descrp = b"min effort in covered clause elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_60, b"ccemineff\0" as *const u8 as *const libc::c_char);
    let mut opt_61: *mut Opt = &mut (*opts).ccereleff;
    (*opt_61).lng = b"ccereleff\0" as *const u8 as *const libc::c_char;
    (*opt_61).val = 50 as libc::c_int;
    (*opt_61).dflt = (*opt_61).val;
    (*opt_61).min = 0 as libc::c_int;
    (*opt_61).max = K;
    (*opt_61)
        .descrp = b"rel effort in covered clause elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_61, b"ccereleff\0" as *const u8 as *const libc::c_char);
    let mut opt_62: *mut Opt = &mut (*opts).ccertc;
    (*opt_62).lng = b"ccertc\0" as *const u8 as *const libc::c_char;
    (*opt_62).val = 0 as libc::c_int;
    (*opt_62).dflt = (*opt_62).val;
    (*opt_62).min = 0 as libc::c_int;
    (*opt_62).max = 2 as libc::c_int;
    (*opt_62)
        .descrp = b"run CCE until completition (1=almost-no-limit)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_62, b"ccertc\0" as *const u8 as *const libc::c_char);
    let mut opt_63: *mut Opt = &mut (*opts).ccertcint;
    (*opt_63).lng = b"ccertcint\0" as *const u8 as *const libc::c_char;
    (*opt_63).val = 15 as libc::c_int;
    (*opt_63).dflt = (*opt_63).val;
    (*opt_63).min = 1 as libc::c_int;
    (*opt_63).max = I;
    (*opt_63)
        .descrp = b"run CCE until completion interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_63, b"ccertcint\0" as *const u8 as *const libc::c_char);
    let mut opt_64: *mut Opt = &mut (*opts).ccertcintvlim;
    (*opt_64).lng = b"ccertcintvlim\0" as *const u8 as *const libc::c_char;
    (*opt_64).val = 2 as libc::c_int * M;
    (*opt_64).dflt = (*opt_64).val;
    (*opt_64).min = 1 as libc::c_int;
    (*opt_64).max = I;
    (*opt_64)
        .descrp = b"run CCE until completion int var limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_64, b"ccertcintvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_65: *mut Opt = &mut (*opts).ccesuccessrat;
    (*opt_65).lng = b"ccesuccessrat\0" as *const u8 as *const libc::c_char;
    (*opt_65).val = 100 as libc::c_int;
    (*opt_65).dflt = (*opt_65).val;
    (*opt_65).min = 1 as libc::c_int;
    (*opt_65).max = I;
    (*opt_65).descrp = b"CCE success ratio\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_65, b"ccesuccessrat\0" as *const u8 as *const libc::c_char);
    let mut opt_66: *mut Opt = &mut (*opts).ccewait;
    (*opt_66).lng = b"ccewait\0" as *const u8 as *const libc::c_char;
    (*opt_66).val = 2 as libc::c_int;
    (*opt_66).dflt = (*opt_66).val;
    (*opt_66).min = 0 as libc::c_int;
    (*opt_66).max = 2 as libc::c_int;
    (*opt_66)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_66, b"ccewait\0" as *const u8 as *const libc::c_char);
    let mut opt_67: *mut Opt = &mut (*opts).check;
    (*opt_67).lng = b"check\0" as *const u8 as *const libc::c_char;
    (*opt_67).val = 0 as libc::c_int;
    (*opt_67).dflt = (*opt_67).val;
    (*opt_67).min = 0 as libc::c_int;
    (*opt_67).max = 3 as libc::c_int;
    (*opt_67).descrp = b"check level\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_67, b"check\0" as *const u8 as *const libc::c_char);
    let mut opt_68: *mut Opt = &mut (*opts).clim;
    (*opt_68).lng = b"clim\0" as *const u8 as *const libc::c_char;
    (*opt_68).val = -(1 as libc::c_int);
    (*opt_68).dflt = (*opt_68).val;
    (*opt_68).min = -(1 as libc::c_int);
    (*opt_68).max = I;
    (*opt_68).descrp = b"conflict limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_68, b"clim\0" as *const u8 as *const libc::c_char);
    let mut opt_69: *mut Opt = &mut (*opts).compact;
    (*opt_69).lng = b"compact\0" as *const u8 as *const libc::c_char;
    (*opt_69).val = 0 as libc::c_int;
    (*opt_69).dflt = (*opt_69).val;
    (*opt_69).min = 0 as libc::c_int;
    (*opt_69).max = 2 as libc::c_int;
    (*opt_69)
        .descrp = b"compactify after 'lglsat/lglsimp' (1=UNS,2=SAT)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_69, b"compact\0" as *const u8 as *const libc::c_char);
    let mut opt_70: *mut Opt = &mut (*opts).deco;
    (*opt_70).lng = b"deco\0" as *const u8 as *const libc::c_char;
    (*opt_70).val = 1 as libc::c_int;
    (*opt_70).dflt = (*opt_70).val;
    (*opt_70).min = 0 as libc::c_int;
    (*opt_70).max = 1 as libc::c_int;
    (*opt_70)
        .descrp = b"learn decision-only clauses too\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_70, b"deco\0" as *const u8 as *const libc::c_char);
    let mut opt_71: *mut Opt = &mut (*opts).deco1opt;
    (*opt_71).lng = b"deco1opt\0" as *const u8 as *const libc::c_char;
    (*opt_71).val = 1 as libc::c_int;
    (*opt_71).dflt = (*opt_71).val;
    (*opt_71).min = 0 as libc::c_int;
    (*opt_71).max = 1 as libc::c_int;
    (*opt_71).descrp = b"optimized deco 1\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_71, b"deco1opt\0" as *const u8 as *const libc::c_char);
    let mut opt_72: *mut Opt = &mut (*opts).decolim;
    (*opt_72).lng = b"decolim\0" as *const u8 as *const libc::c_char;
    (*opt_72).val = 100 as libc::c_int;
    (*opt_72).dflt = (*opt_72).val;
    (*opt_72).min = 0 as libc::c_int;
    (*opt_72).max = I;
    (*opt_72)
        .descrp = b"decision-only clauses glue limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_72, b"decolim\0" as *const u8 as *const libc::c_char);
    let mut opt_73: *mut Opt = &mut (*opts).decompose;
    (*opt_73).lng = b"decompose\0" as *const u8 as *const libc::c_char;
    (*opt_73).val = 1 as libc::c_int;
    (*opt_73).dflt = (*opt_73).val;
    (*opt_73).min = 0 as libc::c_int;
    (*opt_73).max = 1 as libc::c_int;
    (*opt_73).descrp = b"enable decompose\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_73, b"decompose\0" as *const u8 as *const libc::c_char);
    let mut opt_74: *mut Opt = &mut (*opts).defragfree;
    (*opt_74).lng = b"defragfree\0" as *const u8 as *const libc::c_char;
    (*opt_74).val = 50 as libc::c_int;
    (*opt_74).dflt = (*opt_74).val;
    (*opt_74).min = 10 as libc::c_int;
    (*opt_74).max = K;
    (*opt_74)
        .descrp = b"defragmentation free watches limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_74, b"defragfree\0" as *const u8 as *const libc::c_char);
    let mut opt_75: *mut Opt = &mut (*opts).defragint;
    (*opt_75).lng = b"defragint\0" as *const u8 as *const libc::c_char;
    (*opt_75).val = 10 as libc::c_int * M;
    (*opt_75).dflt = (*opt_75).val;
    (*opt_75).min = 100 as libc::c_int;
    (*opt_75).max = I;
    (*opt_75)
        .descrp = b"defragmentation pushed watches interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_75, b"defragint\0" as *const u8 as *const libc::c_char);
    let mut opt_76: *mut Opt = &mut (*opts).delmax;
    (*opt_76).lng = b"delmax\0" as *const u8 as *const libc::c_char;
    (*opt_76).val = 10 as libc::c_int;
    (*opt_76).dflt = (*opt_76).val;
    (*opt_76).min = 0 as libc::c_int;
    (*opt_76).max = 10 as libc::c_int;
    (*opt_76).descrp = b"maximum delay\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_76, b"delmax\0" as *const u8 as *const libc::c_char);
    let mut opt_77: *mut Opt = &mut (*opts).dlim;
    (*opt_77).lng = b"dlim\0" as *const u8 as *const libc::c_char;
    (*opt_77).val = -(1 as libc::c_int);
    (*opt_77).dflt = (*opt_77).val;
    (*opt_77).min = -(1 as libc::c_int);
    (*opt_77).max = I;
    (*opt_77).descrp = b"decision limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_77, b"dlim\0" as *const u8 as *const libc::c_char);
    let mut opt_78: *mut Opt = &mut (*opts).druplig;
    (*opt_78).lng = b"druplig\0" as *const u8 as *const libc::c_char;
    (*opt_78).val = 0 as libc::c_int;
    (*opt_78).dflt = (*opt_78).val;
    (*opt_78).min = 0 as libc::c_int;
    (*opt_78).max = 1 as libc::c_int;
    (*opt_78)
        .descrp = b"connect to Druplig library\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_78, b"druplig\0" as *const u8 as *const libc::c_char);
    let mut opt_79: *mut Opt = &mut (*opts).drupligcheck;
    (*opt_79).lng = b"drupligcheck\0" as *const u8 as *const libc::c_char;
    (*opt_79).val = 0 as libc::c_int;
    (*opt_79).dflt = (*opt_79).val;
    (*opt_79).min = 0 as libc::c_int;
    (*opt_79).max = 1 as libc::c_int;
    (*opt_79)
        .descrp = b"enable checking of proof by Druplig\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_79, b"drupligcheck\0" as *const u8 as *const libc::c_char);
    let mut opt_80: *mut Opt = &mut (*opts).drupligtrace;
    (*opt_80).lng = b"drupligtrace\0" as *const u8 as *const libc::c_char;
    (*opt_80).val = 0 as libc::c_int;
    (*opt_80).dflt = (*opt_80).val;
    (*opt_80).min = 0 as libc::c_int;
    (*opt_80).max = 1 as libc::c_int;
    (*opt_80)
        .descrp = b"enable tracing of proof by Druplig\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_80, b"drupligtrace\0" as *const u8 as *const libc::c_char);
    let mut opt_81: *mut Opt = &mut (*opts).drupligtraceorig;
    (*opt_81).lng = b"drupligtraceorig\0" as *const u8 as *const libc::c_char;
    (*opt_81).val = 0 as libc::c_int;
    (*opt_81).dflt = (*opt_81).val;
    (*opt_81).min = 0 as libc::c_int;
    (*opt_81).max = 1 as libc::c_int;
    (*opt_81)
        .descrp = b"trace original clauses too\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_81, b"drupligtraceorig\0" as *const u8 as *const libc::c_char);
    let mut opt_82: *mut Opt = &mut (*opts).elim;
    (*opt_82).lng = b"elim\0" as *const u8 as *const libc::c_char;
    (*opt_82).val = 1 as libc::c_int;
    (*opt_82).dflt = (*opt_82).val;
    (*opt_82).min = 0 as libc::c_int;
    (*opt_82).max = 1 as libc::c_int;
    (*opt_82)
        .descrp = b"bounded variable eliminiation (BVE)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_82, b"elim\0" as *const u8 as *const libc::c_char);
    let mut opt_83: *mut Opt = &mut (*opts).elmaxeff;
    (*opt_83).lng = b"elmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_83).val = 800 as libc::c_int * M;
    (*opt_83).dflt = (*opt_83).val;
    (*opt_83).min = -(1 as libc::c_int);
    (*opt_83).max = I;
    (*opt_83)
        .descrp = b"max effort in BVE (-1=unlimited)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_83, b"elmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_84: *mut Opt = &mut (*opts).elmblk;
    (*opt_84).lng = b"elmblk\0" as *const u8 as *const libc::c_char;
    (*opt_84).val = 1 as libc::c_int;
    (*opt_84).dflt = (*opt_84).val;
    (*opt_84).min = 0 as libc::c_int;
    (*opt_84).max = 1 as libc::c_int;
    (*opt_84).descrp = b"enable BCE during BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_84, b"elmblk\0" as *const u8 as *const libc::c_char);
    let mut opt_85: *mut Opt = &mut (*opts).elmblkwait;
    (*opt_85).lng = b"elmblkwait\0" as *const u8 as *const libc::c_char;
    (*opt_85).val = 1 as libc::c_int;
    (*opt_85).dflt = (*opt_85).val;
    (*opt_85).min = 0 as libc::c_int;
    (*opt_85).max = 1 as libc::c_int;
    (*opt_85)
        .descrp = b"wait for BVE to be completed once\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_85, b"elmblkwait\0" as *const u8 as *const libc::c_char);
    let mut opt_86: *mut Opt = &mut (*opts).elmboost;
    (*opt_86).lng = b"elmboost\0" as *const u8 as *const libc::c_char;
    (*opt_86).val = 20 as libc::c_int;
    (*opt_86).dflt = (*opt_86).val;
    (*opt_86).min = 1 as libc::c_int;
    (*opt_86).max = 1000 as libc::c_int;
    (*opt_86).descrp = b"elimination boost\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_86, b"elmboost\0" as *const u8 as *const libc::c_char);
    let mut opt_87: *mut Opt = &mut (*opts).elmboostdel;
    (*opt_87).lng = b"elmboostdel\0" as *const u8 as *const libc::c_char;
    (*opt_87).val = 3 as libc::c_int;
    (*opt_87).dflt = (*opt_87).val;
    (*opt_87).min = 0 as libc::c_int;
    (*opt_87).max = 100 as libc::c_int;
    (*opt_87)
        .descrp = b"initial elimination boost delay\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_87, b"elmboostdel\0" as *const u8 as *const libc::c_char);
    let mut opt_88: *mut Opt = &mut (*opts).elmboostint;
    (*opt_88).lng = b"elmboostint\0" as *const u8 as *const libc::c_char;
    (*opt_88).val = 5 as libc::c_int;
    (*opt_88).dflt = (*opt_88).val;
    (*opt_88).min = 1 as libc::c_int;
    (*opt_88).max = I;
    (*opt_88)
        .descrp = b"elimination boost interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_88, b"elmboostint\0" as *const u8 as *const libc::c_char);
    let mut opt_89: *mut Opt = &mut (*opts).elmboostvlim;
    (*opt_89).lng = b"elmboostvlim\0" as *const u8 as *const libc::c_char;
    (*opt_89).val = 4 as libc::c_int * M;
    (*opt_89).dflt = (*opt_89).val;
    (*opt_89).min = 1 as libc::c_int;
    (*opt_89).max = I;
    (*opt_89)
        .descrp = b"elimination boost var lim\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_89, b"elmboostvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_90: *mut Opt = &mut (*opts).elmclslim;
    (*opt_90).lng = b"elmclslim\0" as *const u8 as *const libc::c_char;
    (*opt_90).val = 1 as libc::c_int * M;
    (*opt_90).dflt = (*opt_90).val;
    (*opt_90).min = 3 as libc::c_int;
    (*opt_90).max = I;
    (*opt_90)
        .descrp = b"max antecendent size in elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_90, b"elmclslim\0" as *const u8 as *const libc::c_char);
    let mut opt_91: *mut Opt = &mut (*opts).elmfull;
    (*opt_91).lng = b"elmfull\0" as *const u8 as *const libc::c_char;
    (*opt_91).val = 0 as libc::c_int;
    (*opt_91).dflt = (*opt_91).val;
    (*opt_91).min = 0 as libc::c_int;
    (*opt_91).max = 1 as libc::c_int;
    (*opt_91).descrp = b"no elimination limits\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_91, b"elmfull\0" as *const u8 as *const libc::c_char);
    let mut opt_92: *mut Opt = &mut (*opts).elmineff;
    (*opt_92).lng = b"elmineff\0" as *const u8 as *const libc::c_char;
    (*opt_92).val = 20 as libc::c_int * M;
    (*opt_92).dflt = (*opt_92).val;
    (*opt_92).min = 0 as libc::c_int;
    (*opt_92).max = I;
    (*opt_92).descrp = b"min effort in BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_92, b"elmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_93: *mut Opt = &mut (*opts).elmlitslim;
    (*opt_93).lng = b"elmlitslim\0" as *const u8 as *const libc::c_char;
    (*opt_93).val = 200 as libc::c_int;
    (*opt_93).dflt = (*opt_93).val;
    (*opt_93).min = 0 as libc::c_int;
    (*opt_93).max = I;
    (*opt_93)
        .descrp = b"one side literals limit for elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_93, b"elmlitslim\0" as *const u8 as *const libc::c_char);
    let mut opt_94: *mut Opt = &mut (*opts).elmocclim1;
    (*opt_94).lng = b"elmocclim1\0" as *const u8 as *const libc::c_char;
    (*opt_94).val = 1000 as libc::c_int;
    (*opt_94).dflt = (*opt_94).val;
    (*opt_94).min = 1 as libc::c_int;
    (*opt_94).max = I;
    (*opt_94).descrp = b"one-sided max occ of BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_94, b"elmocclim1\0" as *const u8 as *const libc::c_char);
    let mut opt_95: *mut Opt = &mut (*opts).elmocclim;
    (*opt_95).lng = b"elmocclim\0" as *const u8 as *const libc::c_char;
    (*opt_95).val = 1 as libc::c_int * M;
    (*opt_95).dflt = (*opt_95).val;
    (*opt_95).min = 3 as libc::c_int;
    (*opt_95).max = I;
    (*opt_95).descrp = b"max occurrences in BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_95, b"elmocclim\0" as *const u8 as *const libc::c_char);
    let mut opt_96: *mut Opt = &mut (*opts).elmocclim2;
    (*opt_96).lng = b"elmocclim2\0" as *const u8 as *const libc::c_char;
    (*opt_96).val = 100 as libc::c_int;
    (*opt_96).dflt = (*opt_96).val;
    (*opt_96).min = 2 as libc::c_int;
    (*opt_96).max = I;
    (*opt_96).descrp = b"two-sided max occ of BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_96, b"elmocclim2\0" as *const u8 as *const libc::c_char);
    let mut opt_97: *mut Opt = &mut (*opts).elmoccsumforced;
    (*opt_97).lng = b"elmoccsumforced\0" as *const u8 as *const libc::c_char;
    (*opt_97).val = 0 as libc::c_int;
    (*opt_97).dflt = (*opt_97).val;
    (*opt_97).min = 0 as libc::c_int;
    (*opt_97).max = 10 as libc::c_int;
    (*opt_97).descrp = b"forced occurrence sum\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_97, b"elmoccsumforced\0" as *const u8 as *const libc::c_char);
    let mut opt_98: *mut Opt = &mut (*opts).elmreleff;
    (*opt_98).lng = b"elmreleff\0" as *const u8 as *const libc::c_char;
    (*opt_98).val = 200 as libc::c_int;
    (*opt_98).dflt = (*opt_98).val;
    (*opt_98).min = 0 as libc::c_int;
    (*opt_98).max = 10 as libc::c_int * K;
    (*opt_98).descrp = b"rel effort in BVE\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_98, b"elmreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_99: *mut Opt = &mut (*opts).elmresched;
    (*opt_99).lng = b"elmresched\0" as *const u8 as *const libc::c_char;
    (*opt_99).val = 0 as libc::c_int;
    (*opt_99).dflt = (*opt_99).val;
    (*opt_99).min = 0 as libc::c_int;
    (*opt_99).max = 7 as libc::c_int;
    (*opt_99)
        .descrp = b"reschedule variables (1=else,2=boost,4=full)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_99, b"elmresched\0" as *const u8 as *const libc::c_char);
    let mut opt_100: *mut Opt = &mut (*opts).elmroundlim;
    (*opt_100).lng = b"elmroundlim\0" as *const u8 as *const libc::c_char;
    (*opt_100).val = 3 as libc::c_int;
    (*opt_100).dflt = (*opt_100).val;
    (*opt_100).min = 1 as libc::c_int;
    (*opt_100).max = I;
    (*opt_100)
        .descrp = b"variable elimination rounds limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_100, b"elmroundlim\0" as *const u8 as *const libc::c_char);
    let mut opt_101: *mut Opt = &mut (*opts).elmrtc;
    (*opt_101).lng = b"elmrtc\0" as *const u8 as *const libc::c_char;
    (*opt_101).val = 0 as libc::c_int;
    (*opt_101).dflt = (*opt_101).val;
    (*opt_101).min = 0 as libc::c_int;
    (*opt_101).max = 2 as libc::c_int;
    (*opt_101)
        .descrp = b"run BVE until completion (1=almost-no-limit)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_101, b"elmrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_102: *mut Opt = &mut (*opts).elmrtcint;
    (*opt_102).lng = b"elmrtcint\0" as *const u8 as *const libc::c_char;
    (*opt_102).val = 10 as libc::c_int;
    (*opt_102).dflt = (*opt_102).val;
    (*opt_102).min = 1 as libc::c_int;
    (*opt_102).max = I;
    (*opt_102)
        .descrp = b"run BVE until completion interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_102, b"elmrtcint\0" as *const u8 as *const libc::c_char);
    let mut opt_103: *mut Opt = &mut (*opts).elmrtcintvlim;
    (*opt_103).lng = b"elmrtcintvlim\0" as *const u8 as *const libc::c_char;
    (*opt_103).val = 500 as libc::c_int * K;
    (*opt_103).dflt = (*opt_103).val;
    (*opt_103).min = 1 as libc::c_int;
    (*opt_103).max = I;
    (*opt_103)
        .descrp = b"run BVE until completion int var limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_103, b"elmrtcintvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_104: *mut Opt = &mut (*opts).elmotfstr;
    (*opt_104).lng = b"elmotfstr\0" as *const u8 as *const libc::c_char;
    (*opt_104).val = 1 as libc::c_int;
    (*opt_104).dflt = (*opt_104).val;
    (*opt_104).min = 0 as libc::c_int;
    (*opt_104).max = 1 as libc::c_int;
    (*opt_104)
        .descrp = b"on-the-fly strengthening during BVE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_104, b"elmotfstr\0" as *const u8 as *const libc::c_char);
    let mut opt_105: *mut Opt = &mut (*opts).elmotfsub;
    (*opt_105).lng = b"elmotfsub\0" as *const u8 as *const libc::c_char;
    (*opt_105).val = 1 as libc::c_int;
    (*opt_105).dflt = (*opt_105).val;
    (*opt_105).min = 0 as libc::c_int;
    (*opt_105).max = 1 as libc::c_int;
    (*opt_105)
        .descrp = b"on-the-fly subsumption during BVE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_105, b"elmotfsub\0" as *const u8 as *const libc::c_char);
    let mut opt_106: *mut Opt = &mut (*opts).elmsuccessmaxwortc;
    (*opt_106).lng = b"elmsuccessmaxwortc\0" as *const u8 as *const libc::c_char;
    (*opt_106).val = 4 as libc::c_int;
    (*opt_106).dflt = (*opt_106).val;
    (*opt_106).min = 1 as libc::c_int;
    (*opt_106).max = I;
    (*opt_106)
        .descrp = b"BVE success max without run-to-completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_106, b"elmsuccessmaxwortc\0" as *const u8 as *const libc::c_char);
    let mut opt_107: *mut Opt = &mut (*opts).elmsuccessrat;
    (*opt_107).lng = b"elmsuccessrat\0" as *const u8 as *const libc::c_char;
    (*opt_107).val = 100 as libc::c_int;
    (*opt_107).dflt = (*opt_107).val;
    (*opt_107).min = 1 as libc::c_int;
    (*opt_107).max = I;
    (*opt_107).descrp = b"BVE success ratio\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_107, b"elmsuccessrat\0" as *const u8 as *const libc::c_char);
    let mut opt_108: *mut Opt = &mut (*opts).exitonabort;
    (*opt_108).lng = b"exitonabort\0" as *const u8 as *const libc::c_char;
    (*opt_108).val = 0 as libc::c_int;
    (*opt_108).dflt = (*opt_108).val;
    (*opt_108).min = 0 as libc::c_int;
    (*opt_108).max = 1 as libc::c_int;
    (*opt_108)
        .descrp = b"exit instead abort after internal error\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_108, b"exitonabort\0" as *const u8 as *const libc::c_char);
    let mut opt_109: *mut Opt = &mut (*opts).factmax;
    (*opt_109).lng = b"factmax\0" as *const u8 as *const libc::c_char;
    (*opt_109).val = 100000 as libc::c_int;
    (*opt_109).dflt = (*opt_109).val;
    (*opt_109).min = 1 as libc::c_int;
    (*opt_109).max = I;
    (*opt_109).descrp = b"maximum factor\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_109, b"factmax\0" as *const u8 as *const libc::c_char);
    let mut opt_110: *mut Opt = &mut (*opts).factor;
    (*opt_110).lng = b"factor\0" as *const u8 as *const libc::c_char;
    (*opt_110).val = 3 as libc::c_int;
    (*opt_110).dflt = (*opt_110).val;
    (*opt_110).min = 0 as libc::c_int;
    (*opt_110).max = 3 as libc::c_int;
    (*opt_110)
        .descrp = b"{cls,occ}lim factors (0=const1,1=ld,2=lin,3=sqr)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_110, b"factor\0" as *const u8 as *const libc::c_char);
    let mut opt_111: *mut Opt = &mut (*opts).features;
    (*opt_111).lng = b"features\0" as *const u8 as *const libc::c_char;
    (*opt_111).val = 0 as libc::c_int;
    (*opt_111).dflt = (*opt_111).val;
    (*opt_111).min = 0 as libc::c_int;
    (*opt_111).max = I;
    (*opt_111)
        .descrp = b"print features after that many simplifications\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_111, b"features\0" as *const u8 as *const libc::c_char);
    let mut opt_112: *mut Opt = &mut (*opts).gauss;
    (*opt_112).lng = b"gauss\0" as *const u8 as *const libc::c_char;
    (*opt_112).val = 1 as libc::c_int;
    (*opt_112).dflt = (*opt_112).val;
    (*opt_112).min = 0 as libc::c_int;
    (*opt_112).max = 1 as libc::c_int;
    (*opt_112)
        .descrp = b"enable gaussian elimination\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_112, b"gauss\0" as *const u8 as *const libc::c_char);
    let mut opt_113: *mut Opt = &mut (*opts).gausscardweak;
    (*opt_113).lng = b"gausscardweak\0" as *const u8 as *const libc::c_char;
    (*opt_113).val = 1 as libc::c_int;
    (*opt_113).dflt = (*opt_113).val;
    (*opt_113).min = 0 as libc::c_int;
    (*opt_113).max = 1 as libc::c_int;
    (*opt_113)
        .descrp = b"extract XOR from cardinality constraints\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_113, b"gausscardweak\0" as *const u8 as *const libc::c_char);
    let mut opt_114: *mut Opt = &mut (*opts).gaussexptrn;
    (*opt_114).lng = b"gaussexptrn\0" as *const u8 as *const libc::c_char;
    (*opt_114).val = 1 as libc::c_int;
    (*opt_114).dflt = (*opt_114).val;
    (*opt_114).min = 0 as libc::c_int;
    (*opt_114).max = 1 as libc::c_int;
    (*opt_114)
        .descrp = b"export trn cls from gaussian elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_114, b"gaussexptrn\0" as *const u8 as *const libc::c_char);
    let mut opt_115: *mut Opt = &mut (*opts).gaussextrall;
    (*opt_115).lng = b"gaussextrall\0" as *const u8 as *const libc::c_char;
    (*opt_115).val = 1 as libc::c_int;
    (*opt_115).dflt = (*opt_115).val;
    (*opt_115).min = 0 as libc::c_int;
    (*opt_115).max = 1 as libc::c_int;
    (*opt_115)
        .descrp = b"extract all xors (with duplicates)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_115, b"gaussextrall\0" as *const u8 as *const libc::c_char);
    let mut opt_116: *mut Opt = &mut (*opts).gaussmaxeff;
    (*opt_116).lng = b"gaussmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_116).val = 50 as libc::c_int * M;
    (*opt_116).dflt = (*opt_116).val;
    (*opt_116).min = -(1 as libc::c_int);
    (*opt_116).max = I;
    (*opt_116)
        .descrp = b"max effort in gaussian elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_116, b"gaussmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_117: *mut Opt = &mut (*opts).gaussmaxor;
    (*opt_117).lng = b"gaussmaxor\0" as *const u8 as *const libc::c_char;
    (*opt_117).val = 20 as libc::c_int;
    (*opt_117).dflt = (*opt_117).val;
    (*opt_117).min = 2 as libc::c_int;
    (*opt_117).max = 64 as libc::c_int;
    (*opt_117)
        .descrp = b"maximum xor size in gaussian elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_117, b"gaussmaxor\0" as *const u8 as *const libc::c_char);
    let mut opt_118: *mut Opt = &mut (*opts).gaussmineff;
    (*opt_118).lng = b"gaussmineff\0" as *const u8 as *const libc::c_char;
    (*opt_118).val = 2 as libc::c_int * M;
    (*opt_118).dflt = (*opt_118).val;
    (*opt_118).min = 0 as libc::c_int;
    (*opt_118).max = I;
    (*opt_118)
        .descrp = b"min effort in gaussian elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_118, b"gaussmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_119: *mut Opt = &mut (*opts).gaussreleff;
    (*opt_119).lng = b"gaussreleff\0" as *const u8 as *const libc::c_char;
    (*opt_119).val = 2 as libc::c_int;
    (*opt_119).dflt = (*opt_119).val;
    (*opt_119).min = 0 as libc::c_int;
    (*opt_119).max = 10 as libc::c_int * K;
    (*opt_119)
        .descrp = b"rel effort in gaussian elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_119, b"gaussreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_120: *mut Opt = &mut (*opts).gausswait;
    (*opt_120).lng = b"gausswait\0" as *const u8 as *const libc::c_char;
    (*opt_120).val = 2 as libc::c_int;
    (*opt_120).dflt = (*opt_120).val;
    (*opt_120).min = 0 as libc::c_int;
    (*opt_120).max = 2 as libc::c_int;
    (*opt_120)
        .descrp = b"wait for BCE (1) and/or BVE (2) for XOR extraction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_120, b"gausswait\0" as *const u8 as *const libc::c_char);
    let mut opt_121: *mut Opt = &mut (*opts).gluekeep;
    (*opt_121).lng = b"gluekeep\0" as *const u8 as *const libc::c_char;
    (*opt_121).val = 4 as libc::c_int;
    (*opt_121).dflt = (*opt_121).val;
    (*opt_121).min = 1 as libc::c_int;
    (*opt_121).max = I;
    (*opt_121)
        .descrp = b"keep clauses with this original glue\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_121, b"gluekeep\0" as *const u8 as *const libc::c_char);
    let mut opt_122: *mut Opt = &mut (*opts).gluekeepsize;
    (*opt_122).lng = b"gluekeepsize\0" as *const u8 as *const libc::c_char;
    (*opt_122).val = 15 as libc::c_int;
    (*opt_122).dflt = (*opt_122).val;
    (*opt_122).min = 1 as libc::c_int;
    (*opt_122).max = I;
    (*opt_122)
        .descrp = b"but limit them to this size\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_122, b"gluekeepsize\0" as *const u8 as *const libc::c_char);
    let mut opt_123: *mut Opt = &mut (*opts).gluemacdfast;
    (*opt_123).lng = b"gluemacdfast\0" as *const u8 as *const libc::c_char;
    (*opt_123).val = 5 as libc::c_int;
    (*opt_123).dflt = (*opt_123).val;
    (*opt_123).min = 0 as libc::c_int;
    (*opt_123).max = 32 as libc::c_int;
    (*opt_123)
        .descrp = b"e for fast (D)EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_123, b"gluemacdfast\0" as *const u8 as *const libc::c_char);
    let mut opt_124: *mut Opt = &mut (*opts).gluemacdslow;
    (*opt_124).lng = b"gluemacdslow\0" as *const u8 as *const libc::c_char;
    (*opt_124).val = 16 as libc::c_int;
    (*opt_124).dflt = (*opt_124).val;
    (*opt_124).min = 0 as libc::c_int;
    (*opt_124).max = 32 as libc::c_int;
    (*opt_124)
        .descrp = b"e for slow (D)EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_124, b"gluemacdslow\0" as *const u8 as *const libc::c_char);
    let mut opt_125: *mut Opt = &mut (*opts).gluemacdsmooth;
    (*opt_125).lng = b"gluemacdsmooth\0" as *const u8 as *const libc::c_char;
    (*opt_125).val = 3 as libc::c_int;
    (*opt_125).dflt = (*opt_125).val;
    (*opt_125).min = 0 as libc::c_int;
    (*opt_125).max = 32 as libc::c_int;
    (*opt_125)
        .descrp = b"e for avg EMA with alpha=2^-e\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_125, b"gluemacdsmooth\0" as *const u8 as *const libc::c_char);
    let mut opt_126: *mut Opt = &mut (*opts).gluescale;
    (*opt_126).lng = b"gluescale\0" as *const u8 as *const libc::c_char;
    (*opt_126).val = 4 as libc::c_int;
    (*opt_126).dflt = (*opt_126).val;
    (*opt_126).min = 1 as libc::c_int;
    (*opt_126).max = 5 as libc::c_int;
    (*opt_126)
        .descrp = b"glue scaling: 1=ar1,2=ar2,3=sqrt,4=sqrtld,5=ld\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_126, b"gluescale\0" as *const u8 as *const libc::c_char);
    let mut opt_127: *mut Opt = &mut (*opts).hbrdom;
    (*opt_127).lng = b"hbrdom\0" as *const u8 as *const libc::c_char;
    (*opt_127).val = 2 as libc::c_int;
    (*opt_127).dflt = (*opt_127).val;
    (*opt_127).min = 0 as libc::c_int;
    (*opt_127).max = 2 as libc::c_int;
    (*opt_127)
        .descrp = b"0=root-impl-tree,1=lca-impl-tree,2=lca-big-dag\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_127, b"hbrdom\0" as *const u8 as *const libc::c_char);
    let mut opt_128: *mut Opt = &mut (*opts).import;
    (*opt_128).lng = b"import\0" as *const u8 as *const libc::c_char;
    (*opt_128).val = 1 as libc::c_int;
    (*opt_128).dflt = (*opt_128).val;
    (*opt_128).min = 0 as libc::c_int;
    (*opt_128).max = 1 as libc::c_int;
    (*opt_128)
        .descrp = b"import external indices and map them\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_128, b"import\0" as *const u8 as *const libc::c_char);
    let mut opt_129: *mut Opt = &mut (*opts).incredcint;
    (*opt_129).lng = b"incredcint\0" as *const u8 as *const libc::c_char;
    (*opt_129).val = 1 as libc::c_int;
    (*opt_129).dflt = (*opt_129).val;
    (*opt_129).min = 1 as libc::c_int;
    (*opt_129).max = I;
    (*opt_129)
        .descrp = b"incremental reduce conflict interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_129, b"incredcint\0" as *const u8 as *const libc::c_char);
    let mut opt_130: *mut Opt = &mut (*opts).incredconfslim;
    (*opt_130).lng = b"incredconfslim\0" as *const u8 as *const libc::c_char;
    (*opt_130).val = 0 as libc::c_int;
    (*opt_130).dflt = (*opt_130).val;
    (*opt_130).min = 0 as libc::c_int;
    (*opt_130).max = 100 as libc::c_int;
    (*opt_130)
        .descrp = b"incremental reduce conflict limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_130, b"incredconfslim\0" as *const u8 as *const libc::c_char);
    let mut opt_131: *mut Opt = &mut (*opts).incsavevisits;
    (*opt_131).lng = b"incsavevisits\0" as *const u8 as *const libc::c_char;
    (*opt_131).val = 0 as libc::c_int;
    (*opt_131).dflt = (*opt_131).val;
    (*opt_131).min = 0 as libc::c_int;
    (*opt_131).max = 1 as libc::c_int;
    (*opt_131)
        .descrp = b"incremental start new visits counter\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_131, b"incsavevisits\0" as *const u8 as *const libc::c_char);
    let mut opt_132: *mut Opt = &mut (*opts).inprocessing;
    (*opt_132).lng = b"inprocessing\0" as *const u8 as *const libc::c_char;
    (*opt_132).val = 1 as libc::c_int;
    (*opt_132).dflt = (*opt_132).val;
    (*opt_132).min = 0 as libc::c_int;
    (*opt_132).max = 1 as libc::c_int;
    (*opt_132).descrp = b"enable inprocessing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_132, b"inprocessing\0" as *const u8 as *const libc::c_char);
    let mut opt_133: *mut Opt = &mut (*opts).irrlim;
    (*opt_133).lng = b"irrlim\0" as *const u8 as *const libc::c_char;
    (*opt_133).val = 1 as libc::c_int;
    (*opt_133).dflt = (*opt_133).val;
    (*opt_133).min = 0 as libc::c_int;
    (*opt_133).max = 1 as libc::c_int;
    (*opt_133)
        .descrp = b"use irredundant clauses as limit for simps\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_133, b"irrlim\0" as *const u8 as *const libc::c_char);
    let mut opt_134: *mut Opt = &mut (*opts).itsmacdfast;
    (*opt_134).lng = b"itsmacdfast\0" as *const u8 as *const libc::c_char;
    (*opt_134).val = 12 as libc::c_int;
    (*opt_134).dflt = (*opt_134).val;
    (*opt_134).min = 0 as libc::c_int;
    (*opt_134).max = 32 as libc::c_int;
    (*opt_134)
        .descrp = b"e for fast (D)EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_134, b"itsmacdfast\0" as *const u8 as *const libc::c_char);
    let mut opt_135: *mut Opt = &mut (*opts).itsmacdslow;
    (*opt_135).lng = b"itsmacdslow\0" as *const u8 as *const libc::c_char;
    (*opt_135).val = 18 as libc::c_int;
    (*opt_135).dflt = (*opt_135).val;
    (*opt_135).min = 0 as libc::c_int;
    (*opt_135).max = 32 as libc::c_int;
    (*opt_135)
        .descrp = b"e for slow (D)EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_135, b"itsmacdslow\0" as *const u8 as *const libc::c_char);
    let mut opt_136: *mut Opt = &mut (*opts).itsmacdsmooth;
    (*opt_136).lng = b"itsmacdsmooth\0" as *const u8 as *const libc::c_char;
    (*opt_136).val = 10 as libc::c_int;
    (*opt_136).dflt = (*opt_136).val;
    (*opt_136).min = 0 as libc::c_int;
    (*opt_136).max = 32 as libc::c_int;
    (*opt_136)
        .descrp = b"e for avg EMA with alpha=2^-e\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_136, b"itsmacdsmooth\0" as *const u8 as *const libc::c_char);
    let mut opt_137: *mut Opt = &mut (*opts).jlevelmacdfast;
    (*opt_137).lng = b"jlevelmacdfast\0" as *const u8 as *const libc::c_char;
    (*opt_137).val = 12 as libc::c_int;
    (*opt_137).dflt = (*opt_137).val;
    (*opt_137).min = 0 as libc::c_int;
    (*opt_137).max = 32 as libc::c_int;
    (*opt_137)
        .descrp = b"e for fast (D)EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_137, b"jlevelmacdfast\0" as *const u8 as *const libc::c_char);
    let mut opt_138: *mut Opt = &mut (*opts).jlevelmacdslow;
    (*opt_138).lng = b"jlevelmacdslow\0" as *const u8 as *const libc::c_char;
    (*opt_138).val = 14 as libc::c_int;
    (*opt_138).dflt = (*opt_138).val;
    (*opt_138).min = 0 as libc::c_int;
    (*opt_138).max = 32 as libc::c_int;
    (*opt_138)
        .descrp = b"e for slow (D)EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_138, b"jlevelmacdslow\0" as *const u8 as *const libc::c_char);
    let mut opt_139: *mut Opt = &mut (*opts).jlevelmacdsmooth;
    (*opt_139).lng = b"jlevelmacdsmooth\0" as *const u8 as *const libc::c_char;
    (*opt_139).val = 10 as libc::c_int;
    (*opt_139).dflt = (*opt_139).val;
    (*opt_139).min = 0 as libc::c_int;
    (*opt_139).max = 32 as libc::c_int;
    (*opt_139)
        .descrp = b"e for avg EMA with alpha=2^-e\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_139, b"jlevelmacdsmooth\0" as *const u8 as *const libc::c_char);
    let mut opt_140: *mut Opt = &mut (*opts).jwhred;
    (*opt_140).lng = b"jwhred\0" as *const u8 as *const libc::c_char;
    (*opt_140).val = 1 as libc::c_int;
    (*opt_140).dflt = (*opt_140).val;
    (*opt_140).min = 0 as libc::c_int;
    (*opt_140).max = 2 as libc::c_int;
    (*opt_140)
        .descrp = b"JWH score based on redundant clauses too (2=only)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_140, b"jwhred\0" as *const u8 as *const libc::c_char);
    let mut opt_141: *mut Opt = &mut (*opts).keepmaxglue;
    (*opt_141).lng = b"keepmaxglue\0" as *const u8 as *const libc::c_char;
    (*opt_141).val = 1 as libc::c_int;
    (*opt_141).dflt = (*opt_141).val;
    (*opt_141).min = 0 as libc::c_int;
    (*opt_141).max = 1 as libc::c_int;
    (*opt_141)
        .descrp = b"keep maximum glue clauses\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_141, b"keepmaxglue\0" as *const u8 as *const libc::c_char);
    let mut opt_142: *mut Opt = &mut (*opts).keepmaxglueint;
    (*opt_142).lng = b"keepmaxglueint\0" as *const u8 as *const libc::c_char;
    (*opt_142).val = 1 as libc::c_int;
    (*opt_142).dflt = (*opt_142).val;
    (*opt_142).min = 1 as libc::c_int;
    (*opt_142).max = I;
    (*opt_142)
        .descrp = b"keep maximum glue clause interval (1 always)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_142, b"keepmaxglueint\0" as *const u8 as *const libc::c_char);
    let mut opt_143: *mut Opt = &mut (*opts).lhbr;
    (*opt_143).lng = b"lhbr\0" as *const u8 as *const libc::c_char;
    (*opt_143).val = 1 as libc::c_int;
    (*opt_143).dflt = (*opt_143).val;
    (*opt_143).min = 0 as libc::c_int;
    (*opt_143).max = 1 as libc::c_int;
    (*opt_143)
        .descrp = b"enable lazy hyber binary reasoning\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_143, b"lhbr\0" as *const u8 as *const libc::c_char);
    let mut opt_144: *mut Opt = &mut (*opts).lkhd;
    (*opt_144).lng = b"lkhd\0" as *const u8 as *const libc::c_char;
    (*opt_144).val = 2 as libc::c_int;
    (*opt_144).dflt = (*opt_144).val;
    (*opt_144).min = -(1 as libc::c_int);
    (*opt_144).max = 4 as libc::c_int;
    (*opt_144)
        .descrp = b"-1=LOCS,0=LIS,1=JWH,2=TREELOOK,3=LENSUM,4=RELEVANCE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_144, b"lkhd\0" as *const u8 as *const libc::c_char);
    let mut opt_145: *mut Opt = &mut (*opts).locs;
    (*opt_145).lng = b"locs\0" as *const u8 as *const libc::c_char;
    (*opt_145).val = 0 as libc::c_int;
    (*opt_145).dflt = (*opt_145).val;
    (*opt_145).min = -(1 as libc::c_int);
    (*opt_145).max = I;
    (*opt_145)
        .descrp = b"use local search (-1=always otherwise how often)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_145, b"locs\0" as *const u8 as *const libc::c_char);
    let mut opt_146: *mut Opt = &mut (*opts).locsbanner;
    (*opt_146).lng = b"locsbanner\0" as *const u8 as *const libc::c_char;
    (*opt_146).val = 0 as libc::c_int;
    (*opt_146).dflt = (*opt_146).val;
    (*opt_146).min = 0 as libc::c_int;
    (*opt_146).max = 1 as libc::c_int;
    (*opt_146)
        .descrp = b"print version number of LOCS component\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_146, b"locsbanner\0" as *const u8 as *const libc::c_char);
    let mut opt_147: *mut Opt = &mut (*opts).locsboost;
    (*opt_147).lng = b"locsboost\0" as *const u8 as *const libc::c_char;
    (*opt_147).val = 2 as libc::c_int;
    (*opt_147).dflt = (*opt_147).val;
    (*opt_147).min = 0 as libc::c_int;
    (*opt_147).max = 100 as libc::c_int;
    (*opt_147)
        .descrp = b"initial local search boost\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_147, b"locsboost\0" as *const u8 as *const libc::c_char);
    let mut opt_148: *mut Opt = &mut (*opts).locscint;
    (*opt_148).lng = b"locscint\0" as *const u8 as *const libc::c_char;
    (*opt_148).val = 10 as libc::c_int * K;
    (*opt_148).dflt = (*opt_148).val;
    (*opt_148).min = 1 as libc::c_int;
    (*opt_148).max = I;
    (*opt_148)
        .descrp = b"conflict interval for LOCS\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_148, b"locscint\0" as *const u8 as *const libc::c_char);
    let mut opt_149: *mut Opt = &mut (*opts).locsclim;
    (*opt_149).lng = b"locsclim\0" as *const u8 as *const libc::c_char;
    (*opt_149).val = 1 as libc::c_int * M;
    (*opt_149).dflt = (*opt_149).val;
    (*opt_149).min = 0 as libc::c_int;
    (*opt_149).max = I >> 8 as libc::c_int;
    (*opt_149)
        .descrp = b"clause limit for local search\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_149, b"locsclim\0" as *const u8 as *const libc::c_char);
    let mut opt_150: *mut Opt = &mut (*opts).locset;
    (*opt_150).lng = b"locset\0" as *const u8 as *const libc::c_char;
    (*opt_150).val = 2 as libc::c_int;
    (*opt_150).dflt = (*opt_150).val;
    (*opt_150).min = 0 as libc::c_int;
    (*opt_150).max = 2 as libc::c_int;
    (*opt_150)
        .descrp = b"initialize local search phases (1=prev,2=cur)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_150, b"locset\0" as *const u8 as *const libc::c_char);
    let mut opt_151: *mut Opt = &mut (*opts).locsexport;
    (*opt_151).lng = b"locsexport\0" as *const u8 as *const libc::c_char;
    (*opt_151).val = 1 as libc::c_int;
    (*opt_151).dflt = (*opt_151).val;
    (*opt_151).min = 0 as libc::c_int;
    (*opt_151).max = 1 as libc::c_int;
    (*opt_151)
        .descrp = b"export phases from local search\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_151, b"locsexport\0" as *const u8 as *const libc::c_char);
    let mut opt_152: *mut Opt = &mut (*opts).locsmaxeff;
    (*opt_152).lng = b"locsmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_152).val = 100000 as libc::c_int;
    (*opt_152).dflt = (*opt_152).val;
    (*opt_152).min = 0 as libc::c_int;
    (*opt_152).max = I;
    (*opt_152)
        .descrp = b"max effort in local search\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_152, b"locsmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_153: *mut Opt = &mut (*opts).locsmineff;
    (*opt_153).lng = b"locsmineff\0" as *const u8 as *const libc::c_char;
    (*opt_153).val = 1000 as libc::c_int;
    (*opt_153).dflt = (*opt_153).val;
    (*opt_153).min = 0 as libc::c_int;
    (*opt_153).max = I;
    (*opt_153)
        .descrp = b"min effort in local search\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_153, b"locsmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_154: *mut Opt = &mut (*opts).locsred;
    (*opt_154).lng = b"locsred\0" as *const u8 as *const libc::c_char;
    (*opt_154).val = 0 as libc::c_int;
    (*opt_154).dflt = (*opt_154).val;
    (*opt_154).min = 0 as libc::c_int;
    (*opt_154).max = 4 as libc::c_int;
    (*opt_154)
        .descrp = b"apply local search on redundant clauses too\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_154, b"locsred\0" as *const u8 as *const libc::c_char);
    let mut opt_155: *mut Opt = &mut (*opts).locsreleff;
    (*opt_155).lng = b"locsreleff\0" as *const u8 as *const libc::c_char;
    (*opt_155).val = 5 as libc::c_int;
    (*opt_155).dflt = (*opt_155).val;
    (*opt_155).min = 0 as libc::c_int;
    (*opt_155).max = 100 as libc::c_int;
    (*opt_155)
        .descrp = b"rel effort in local search\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_155, b"locsreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_156: *mut Opt = &mut (*opts).locsrtc;
    (*opt_156).lng = b"locsrtc\0" as *const u8 as *const libc::c_char;
    (*opt_156).val = 0 as libc::c_int;
    (*opt_156).dflt = (*opt_156).val;
    (*opt_156).min = 0 as libc::c_int;
    (*opt_156).max = 1 as libc::c_int;
    (*opt_156)
        .descrp = b"run local search until completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_156, b"locsrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_157: *mut Opt = &mut (*opts).locsvared;
    (*opt_157).lng = b"locsvared\0" as *const u8 as *const libc::c_char;
    (*opt_157).val = 100 as libc::c_int;
    (*opt_157).dflt = (*opt_157).val;
    (*opt_157).min = 0 as libc::c_int;
    (*opt_157).max = 1000 as libc::c_int;
    (*opt_157)
        .descrp = b"max variable reduction for LOCS\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_157, b"locsvared\0" as *const u8 as *const libc::c_char);
    let mut opt_158: *mut Opt = &mut (*opts).locswait;
    (*opt_158).lng = b"locswait\0" as *const u8 as *const libc::c_char;
    (*opt_158).val = 2 as libc::c_int;
    (*opt_158).dflt = (*opt_158).val;
    (*opt_158).min = 0 as libc::c_int;
    (*opt_158).max = 2 as libc::c_int;
    (*opt_158)
        .descrp = b"wait for BCE(1) and/or BVE(2)\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_158, b"locswait\0" as *const u8 as *const libc::c_char);
    let mut opt_159: *mut Opt = &mut (*opts).log;
    (*opt_159).lng = b"log\0" as *const u8 as *const libc::c_char;
    (*opt_159).val = -(1 as libc::c_int);
    (*opt_159).dflt = (*opt_159).val;
    (*opt_159).min = -(1 as libc::c_int);
    (*opt_159).max = 5 as libc::c_int;
    (*opt_159).descrp = b"log level\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_159, b"log\0" as *const u8 as *const libc::c_char);
    let mut opt_160: *mut Opt = &mut (*opts).maxscaledglue;
    (*opt_160).lng = b"maxscaledglue\0" as *const u8 as *const libc::c_char;
    (*opt_160).val = ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int;
    (*opt_160).dflt = (*opt_160).val;
    (*opt_160).min = 0 as libc::c_int;
    (*opt_160).max = ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int;
    (*opt_160)
        .descrp = b"maximum scaled glue bound\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_160, b"maxscaledglue\0" as *const u8 as *const libc::c_char);
    let mut opt_161: *mut Opt = &mut (*opts).memlim;
    (*opt_161).lng = b"memlim\0" as *const u8 as *const libc::c_char;
    (*opt_161).val = -(1 as libc::c_int);
    (*opt_161).dflt = (*opt_161).val;
    (*opt_161).min = -(1 as libc::c_int);
    (*opt_161).max = I;
    (*opt_161)
        .descrp = b"memory limit in MB (-1=no limit)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_161, b"memlim\0" as *const u8 as *const libc::c_char);
    let mut opt_162: *mut Opt = &mut (*opts).minimize;
    (*opt_162).lng = b"minimize\0" as *const u8 as *const libc::c_char;
    (*opt_162).val = 2 as libc::c_int;
    (*opt_162).dflt = (*opt_162).val;
    (*opt_162).min = 0 as libc::c_int;
    (*opt_162).max = 2 as libc::c_int;
    (*opt_162)
        .descrp = b"minimize learned clauses (1=local,2=recursive)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_162, b"minimize\0" as *const u8 as *const libc::c_char);
    let mut opt_163: *mut Opt = &mut (*opts).minlocalgluelim;
    (*opt_163).lng = b"minlocalgluelim\0" as *const u8 as *const libc::c_char;
    (*opt_163).val = 200 as libc::c_int;
    (*opt_163).dflt = (*opt_163).val;
    (*opt_163).min = 0 as libc::c_int;
    (*opt_163).max = I;
    (*opt_163)
        .descrp = b"glue limit for using local minimization\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_163, b"minlocalgluelim\0" as *const u8 as *const libc::c_char);
    let mut opt_164: *mut Opt = &mut (*opts).minlocalsizelim;
    (*opt_164).lng = b"minlocalsizelim\0" as *const u8 as *const libc::c_char;
    (*opt_164).val = 3000 as libc::c_int;
    (*opt_164).dflt = (*opt_164).val;
    (*opt_164).min = 0 as libc::c_int;
    (*opt_164).max = I;
    (*opt_164)
        .descrp = b"size limit for using local minimization\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_164, b"minlocalsizelim\0" as *const u8 as *const libc::c_char);
    let mut opt_165: *mut Opt = &mut (*opts).minrecgluelim;
    (*opt_165).lng = b"minrecgluelim\0" as *const u8 as *const libc::c_char;
    (*opt_165).val = 100 as libc::c_int;
    (*opt_165).dflt = (*opt_165).val;
    (*opt_165).min = 0 as libc::c_int;
    (*opt_165).max = I;
    (*opt_165)
        .descrp = b"glue limit for using recursive minimization\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_165, b"minrecgluelim\0" as *const u8 as *const libc::c_char);
    let mut opt_166: *mut Opt = &mut (*opts).minrecsizelim;
    (*opt_166).lng = b"minrecsizelim\0" as *const u8 as *const libc::c_char;
    (*opt_166).val = 1000 as libc::c_int;
    (*opt_166).dflt = (*opt_166).val;
    (*opt_166).min = 0 as libc::c_int;
    (*opt_166).max = I;
    (*opt_166)
        .descrp = b"size limit for using recursive minimization\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_166, b"minrecsizelim\0" as *const u8 as *const libc::c_char);
    let mut opt_167: *mut Opt = &mut (*opts).move_0;
    (*opt_167).lng = b"move\0" as *const u8 as *const libc::c_char;
    (*opt_167).val = 2 as libc::c_int;
    (*opt_167).dflt = (*opt_167).val;
    (*opt_167).min = 0 as libc::c_int;
    (*opt_167).max = 2 as libc::c_int;
    (*opt_167)
        .descrp = b"move redundant cls (1=only-binary,2=ternary-too)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_167, b"move\0" as *const u8 as *const libc::c_char);
    let mut opt_168: *mut Opt = &mut (*opts).otfs;
    (*opt_168).lng = b"otfs\0" as *const u8 as *const libc::c_char;
    (*opt_168).val = 0 as libc::c_int;
    (*opt_168).dflt = (*opt_168).val;
    (*opt_168).min = 0 as libc::c_int;
    (*opt_168).max = 1 as libc::c_int;
    (*opt_168)
        .descrp = b"enable on-the-fly subsumption\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_168, b"otfs\0" as *const u8 as *const libc::c_char);
    let mut opt_169: *mut Opt = &mut (*opts).penmax;
    (*opt_169).lng = b"penmax\0" as *const u8 as *const libc::c_char;
    (*opt_169).val = 4 as libc::c_int;
    (*opt_169).dflt = (*opt_169).val;
    (*opt_169).min = 0 as libc::c_int;
    (*opt_169).max = 16 as libc::c_int;
    (*opt_169).descrp = b"maximum penalty\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_169, b"penmax\0" as *const u8 as *const libc::c_char);
    let mut opt_170: *mut Opt = &mut (*opts).phase;
    (*opt_170).lng = b"phase\0" as *const u8 as *const libc::c_char;
    (*opt_170).val = 0 as libc::c_int;
    (*opt_170).dflt = (*opt_170).val;
    (*opt_170).min = -(1 as libc::c_int);
    (*opt_170).max = 1 as libc::c_int;
    (*opt_170)
        .descrp = b"default initial phase (-1=neg,0=JeroslowWang,1=pos)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_170, b"phase\0" as *const u8 as *const libc::c_char);
    let mut opt_171: *mut Opt = &mut (*opts).phaseluckfactor;
    (*opt_171).lng = b"phaseluckfactor\0" as *const u8 as *const libc::c_char;
    (*opt_171).val = 200 as libc::c_int;
    (*opt_171).dflt = (*opt_171).val;
    (*opt_171).min = 100 as libc::c_int;
    (*opt_171).max = 10 as libc::c_int * K;
    (*opt_171)
        .descrp = b"min phase luck factor (pos/neg)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_171, b"phaseluckfactor\0" as *const u8 as *const libc::c_char);
    let mut opt_172: *mut Opt = &mut (*opts).phaselucklim;
    (*opt_172).lng = b"phaselucklim\0" as *const u8 as *const libc::c_char;
    (*opt_172).val = 100 as libc::c_int;
    (*opt_172).dflt = (*opt_172).val;
    (*opt_172).min = 0 as libc::c_int;
    (*opt_172).max = 1000 as libc::c_int;
    (*opt_172)
        .descrp = b"phase luck limit in promille\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_172, b"phaselucklim\0" as *const u8 as *const libc::c_char);
    let mut opt_173: *mut Opt = &mut (*opts).phaseluckmaxround;
    (*opt_173).lng = b"phaseluckmaxround\0" as *const u8 as *const libc::c_char;
    (*opt_173).val = 10 as libc::c_int;
    (*opt_173).dflt = (*opt_173).val;
    (*opt_173).min = 0 as libc::c_int;
    (*opt_173).max = I;
    (*opt_173)
        .descrp = b"maximum number of phase luck checks\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_173, b"phaseluckmaxround\0" as *const u8 as *const libc::c_char);
    let mut opt_174: *mut Opt = &mut (*opts).phasesave;
    (*opt_174).lng = b"phasesave\0" as *const u8 as *const libc::c_char;
    (*opt_174).val = 1 as libc::c_int;
    (*opt_174).dflt = (*opt_174).val;
    (*opt_174).min = -(1 as libc::c_int);
    (*opt_174).max = 1 as libc::c_int;
    (*opt_174)
        .descrp = b"save and use previous phase (-1=reverse)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_174, b"phasesave\0" as *const u8 as *const libc::c_char);
    let mut opt_175: *mut Opt = &mut (*opts).plain;
    (*opt_175).lng = b"plain\0" as *const u8 as *const libc::c_char;
    (*opt_175).val = 0 as libc::c_int;
    (*opt_175).dflt = (*opt_175).val;
    (*opt_175).min = 0 as libc::c_int;
    (*opt_175).max = 1 as libc::c_int;
    (*opt_175)
        .descrp = b"plain mode disables all preprocessing\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_175, b"plain\0" as *const u8 as *const libc::c_char);
    let mut opt_176: *mut Opt = &mut (*opts).plim;
    (*opt_176).lng = b"plim\0" as *const u8 as *const libc::c_char;
    (*opt_176).val = -(1 as libc::c_int);
    (*opt_176).dflt = (*opt_176).val;
    (*opt_176).min = -(1 as libc::c_int);
    (*opt_176).max = I;
    (*opt_176)
        .descrp = b"propagation limit (thousands)\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_176, b"plim\0" as *const u8 as *const libc::c_char);
    let mut opt_177: *mut Opt = &mut (*opts).poison;
    (*opt_177).lng = b"poison\0" as *const u8 as *const libc::c_char;
    (*opt_177).val = 1 as libc::c_int;
    (*opt_177).dflt = (*opt_177).val;
    (*opt_177).min = 0 as libc::c_int;
    (*opt_177).max = 1 as libc::c_int;
    (*opt_177)
        .descrp = b"poison optimization for clause minimization\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_177, b"poison\0" as *const u8 as *const libc::c_char);
    let mut opt_178: *mut Opt = &mut (*opts).prbasic;
    (*opt_178).lng = b"prbasic\0" as *const u8 as *const libc::c_char;
    (*opt_178).val = 1 as libc::c_int;
    (*opt_178).dflt = (*opt_178).val;
    (*opt_178).min = 0 as libc::c_int;
    (*opt_178).max = 2 as libc::c_int;
    (*opt_178)
        .descrp = b"enable basic probing procedure (1=roots-only)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_178, b"prbasic\0" as *const u8 as *const libc::c_char);
    let mut opt_179: *mut Opt = &mut (*opts).prbasicmaxeff;
    (*opt_179).lng = b"prbasicmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_179).val = 400 as libc::c_int * M;
    (*opt_179).dflt = (*opt_179).val;
    (*opt_179).min = -(1 as libc::c_int);
    (*opt_179).max = I;
    (*opt_179)
        .descrp = b"max effort in basic probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_179, b"prbasicmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_180: *mut Opt = &mut (*opts).prbasicmineff;
    (*opt_180).lng = b"prbasicmineff\0" as *const u8 as *const libc::c_char;
    (*opt_180).val = M;
    (*opt_180).dflt = (*opt_180).val;
    (*opt_180).min = 0 as libc::c_int;
    (*opt_180).max = I;
    (*opt_180)
        .descrp = b"min effort in basic probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_180, b"prbasicmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_181: *mut Opt = &mut (*opts).prbasicreleff;
    (*opt_181).lng = b"prbasicreleff\0" as *const u8 as *const libc::c_char;
    (*opt_181).val = 50 as libc::c_int;
    (*opt_181).dflt = (*opt_181).val;
    (*opt_181).min = 0 as libc::c_int;
    (*opt_181).max = 10 as libc::c_int * K;
    (*opt_181)
        .descrp = b"rel effort in basic probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_181, b"prbasicreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_182: *mut Opt = &mut (*opts).prbasicroundlim;
    (*opt_182).lng = b"prbasicroundlim\0" as *const u8 as *const libc::c_char;
    (*opt_182).val = 8 as libc::c_int;
    (*opt_182).dflt = (*opt_182).val;
    (*opt_182).min = 1 as libc::c_int;
    (*opt_182).max = I;
    (*opt_182)
        .descrp = b"basic probing round limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_182, b"prbasicroundlim\0" as *const u8 as *const libc::c_char);
    let mut opt_183: *mut Opt = &mut (*opts).prbasicrtc;
    (*opt_183).lng = b"prbasicrtc\0" as *const u8 as *const libc::c_char;
    (*opt_183).val = 0 as libc::c_int;
    (*opt_183).dflt = (*opt_183).val;
    (*opt_183).min = 0 as libc::c_int;
    (*opt_183).max = 1 as libc::c_int;
    (*opt_183)
        .descrp = b"run basic probing until completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_183, b"prbasicrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_184: *mut Opt = &mut (*opts).prbrtc;
    (*opt_184).lng = b"prbrtc\0" as *const u8 as *const libc::c_char;
    (*opt_184).val = 0 as libc::c_int;
    (*opt_184).dflt = (*opt_184).val;
    (*opt_184).min = 0 as libc::c_int;
    (*opt_184).max = 1 as libc::c_int;
    (*opt_184)
        .descrp = b"run all probing until completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_184, b"prbrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_185: *mut Opt = &mut (*opts).prbsimple;
    (*opt_185).lng = b"prbsimple\0" as *const u8 as *const libc::c_char;
    (*opt_185).val = 2 as libc::c_int;
    (*opt_185).dflt = (*opt_185).val;
    (*opt_185).min = 0 as libc::c_int;
    (*opt_185).max = 3 as libc::c_int;
    (*opt_185)
        .descrp = b"simple probing (1=shallow,2=deep,3=touchall)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_185, b"prbsimple\0" as *const u8 as *const libc::c_char);
    let mut opt_186: *mut Opt = &mut (*opts).prbsimpleboost;
    (*opt_186).lng = b"prbsimpleboost\0" as *const u8 as *const libc::c_char;
    (*opt_186).val = 10 as libc::c_int;
    (*opt_186).dflt = (*opt_186).val;
    (*opt_186).min = 1 as libc::c_int;
    (*opt_186).max = 1000 as libc::c_int;
    (*opt_186)
        .descrp = b"initial simple probing boost\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_186, b"prbsimpleboost\0" as *const u8 as *const libc::c_char);
    let mut opt_187: *mut Opt = &mut (*opts).prbsimpleliftdepth;
    (*opt_187).lng = b"prbsimpleliftdepth\0" as *const u8 as *const libc::c_char;
    (*opt_187).val = 2 as libc::c_int;
    (*opt_187).dflt = (*opt_187).val;
    (*opt_187).min = 1 as libc::c_int;
    (*opt_187).max = 4 as libc::c_int;
    (*opt_187)
        .descrp = b"simple probing lifting depth\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_187, b"prbsimpleliftdepth\0" as *const u8 as *const libc::c_char);
    let mut opt_188: *mut Opt = &mut (*opts).prbsimplemaxeff;
    (*opt_188).lng = b"prbsimplemaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_188).val = 200 as libc::c_int * M;
    (*opt_188).dflt = (*opt_188).val;
    (*opt_188).min = -(1 as libc::c_int);
    (*opt_188).max = I;
    (*opt_188)
        .descrp = b"max effort in simple probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_188, b"prbsimplemaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_189: *mut Opt = &mut (*opts).prbsimplemineff;
    (*opt_189).lng = b"prbsimplemineff\0" as *const u8 as *const libc::c_char;
    (*opt_189).val = 2 as libc::c_int * M;
    (*opt_189).dflt = (*opt_189).val;
    (*opt_189).min = 0 as libc::c_int;
    (*opt_189).max = I;
    (*opt_189)
        .descrp = b"min effort in simple probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_189, b"prbsimplemineff\0" as *const u8 as *const libc::c_char);
    let mut opt_190: *mut Opt = &mut (*opts).prbsimplereleff;
    (*opt_190).lng = b"prbsimplereleff\0" as *const u8 as *const libc::c_char;
    (*opt_190).val = 40 as libc::c_int;
    (*opt_190).dflt = (*opt_190).val;
    (*opt_190).min = 0 as libc::c_int;
    (*opt_190).max = 10 as libc::c_int * K;
    (*opt_190)
        .descrp = b"rel effort in simple probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_190, b"prbsimplereleff\0" as *const u8 as *const libc::c_char);
    let mut opt_191: *mut Opt = &mut (*opts).prbsimplertc;
    (*opt_191).lng = b"prbsimplertc\0" as *const u8 as *const libc::c_char;
    (*opt_191).val = 0 as libc::c_int;
    (*opt_191).dflt = (*opt_191).val;
    (*opt_191).min = 0 as libc::c_int;
    (*opt_191).max = 1 as libc::c_int;
    (*opt_191)
        .descrp = b"run simple probing until completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_191, b"prbsimplertc\0" as *const u8 as *const libc::c_char);
    let mut opt_192: *mut Opt = &mut (*opts).probe;
    (*opt_192).lng = b"probe\0" as *const u8 as *const libc::c_char;
    (*opt_192).val = 1 as libc::c_int;
    (*opt_192).dflt = (*opt_192).val;
    (*opt_192).min = 0 as libc::c_int;
    (*opt_192).max = 1 as libc::c_int;
    (*opt_192).descrp = b"enable probing\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_192, b"probe\0" as *const u8 as *const libc::c_char);
    let mut opt_193: *mut Opt = &mut (*opts).profile;
    (*opt_193).lng = b"profile\0" as *const u8 as *const libc::c_char;
    (*opt_193).val = 1 as libc::c_int;
    (*opt_193).dflt = (*opt_193).val;
    (*opt_193).min = 0 as libc::c_int;
    (*opt_193).max = 4 as libc::c_int;
    (*opt_193).descrp = b"profile level\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_193, b"profile\0" as *const u8 as *const libc::c_char);
    let mut opt_194: *mut Opt = &mut (*opts).profilelong;
    (*opt_194).lng = b"profilelong\0" as *const u8 as *const libc::c_char;
    (*opt_194).val = 0 as libc::c_int;
    (*opt_194).dflt = (*opt_194).val;
    (*opt_194).min = 0 as libc::c_int;
    (*opt_194).max = 1 as libc::c_int;
    (*opt_194)
        .descrp = b"print long profile information\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_194, b"profilelong\0" as *const u8 as *const libc::c_char);
    let mut opt_195: *mut Opt = &mut (*opts).promote;
    (*opt_195).lng = b"promote\0" as *const u8 as *const libc::c_char;
    (*opt_195).val = 1 as libc::c_int;
    (*opt_195).dflt = (*opt_195).val;
    (*opt_195).min = 0 as libc::c_int;
    (*opt_195).max = 1 as libc::c_int;
    (*opt_195)
        .descrp = b"keep clauses with reduced glue longer\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_195, b"promote\0" as *const u8 as *const libc::c_char);
    let mut opt_196: *mut Opt = &mut (*opts).promotegluelim;
    (*opt_196).lng = b"promotegluelim\0" as *const u8 as *const libc::c_char;
    (*opt_196).val = 8 as libc::c_int;
    (*opt_196).dflt = (*opt_196).val;
    (*opt_196).min = 0 as libc::c_int;
    (*opt_196).max = MG;
    (*opt_196)
        .descrp = b"promoted clauses reduced glue limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_196, b"promotegluelim\0" as *const u8 as *const libc::c_char);
    let mut opt_197: *mut Opt = &mut (*opts).prune;
    (*opt_197).lng = b"prune\0" as *const u8 as *const libc::c_char;
    (*opt_197).val = 0 as libc::c_int;
    (*opt_197).dflt = (*opt_197).val;
    (*opt_197).min = 0 as libc::c_int;
    (*opt_197).max = 1 as libc::c_int;
    (*opt_197)
        .descrp = b"pruning through satisfication\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_197, b"prune\0" as *const u8 as *const libc::c_char);
    let mut opt_198: *mut Opt = &mut (*opts).pruneclim;
    (*opt_198).lng = b"pruneclim\0" as *const u8 as *const libc::c_char;
    (*opt_198).val = 1000 as libc::c_int;
    (*opt_198).dflt = (*opt_198).val;
    (*opt_198).min = 0 as libc::c_int;
    (*opt_198).max = I;
    (*opt_198).descrp = b"maximum conflict limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_198, b"pruneclim\0" as *const u8 as *const libc::c_char);
    let mut opt_199: *mut Opt = &mut (*opts).pruneinit;
    (*opt_199).lng = b"pruneinit\0" as *const u8 as *const libc::c_char;
    (*opt_199).val = 2 as libc::c_int;
    (*opt_199).dflt = (*opt_199).val;
    (*opt_199).min = 1 as libc::c_int;
    (*opt_199).max = I;
    (*opt_199)
        .descrp = b"initial decision interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_199, b"pruneinit\0" as *const u8 as *const libc::c_char);
    let mut opt_200: *mut Opt = &mut (*opts).prunelevel;
    (*opt_200).lng = b"prunelevel\0" as *const u8 as *const libc::c_char;
    (*opt_200).val = 2 as libc::c_int;
    (*opt_200).dflt = (*opt_200).val;
    (*opt_200).min = 1 as libc::c_int;
    (*opt_200).max = I;
    (*opt_200).descrp = b"maximum decision level\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_200, b"prunelevel\0" as *const u8 as *const libc::c_char);
    let mut opt_201: *mut Opt = &mut (*opts).prunesize;
    (*opt_201).lng = b"prunesize\0" as *const u8 as *const libc::c_char;
    (*opt_201).val = 10 as libc::c_int;
    (*opt_201).dflt = (*opt_201).val;
    (*opt_201).min = 2 as libc::c_int;
    (*opt_201).max = I;
    (*opt_201).descrp = b"maximum clause size\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_201, b"prunesize\0" as *const u8 as *const libc::c_char);
    let mut opt_202: *mut Opt = &mut (*opts).prunemin;
    (*opt_202).lng = b"prunemin\0" as *const u8 as *const libc::c_char;
    (*opt_202).val = 0 as libc::c_int;
    (*opt_202).dflt = (*opt_202).val;
    (*opt_202).min = 0 as libc::c_int;
    (*opt_202).max = I;
    (*opt_202)
        .descrp = b"minimum decision interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_202, b"prunemin\0" as *const u8 as *const libc::c_char);
    let mut opt_203: *mut Opt = &mut (*opts).prunemax;
    (*opt_203).lng = b"prunemax\0" as *const u8 as *const libc::c_char;
    (*opt_203).val = 1000000 as libc::c_int;
    (*opt_203).dflt = (*opt_203).val;
    (*opt_203).min = 1 as libc::c_int;
    (*opt_203).max = I;
    (*opt_203)
        .descrp = b"maximum decision interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_203, b"prunemax\0" as *const u8 as *const libc::c_char);
    let mut opt_204: *mut Opt = &mut (*opts).prunepure;
    (*opt_204).lng = b"prunepure\0" as *const u8 as *const libc::c_char;
    (*opt_204).val = 1 as libc::c_int;
    (*opt_204).dflt = (*opt_204).val;
    (*opt_204).min = 0 as libc::c_int;
    (*opt_204).max = 1 as libc::c_int;
    (*opt_204)
        .descrp = b"find and treat pure literals explicitly\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_204, b"prunepure\0" as *const u8 as *const libc::c_char);
    let mut opt_205: *mut Opt = &mut (*opts).prunered;
    (*opt_205).lng = b"prunered\0" as *const u8 as *const libc::c_char;
    (*opt_205).val = 1 as libc::c_int;
    (*opt_205).dflt = (*opt_205).val;
    (*opt_205).min = 0 as libc::c_int;
    (*opt_205).max = 1 as libc::c_int;
    (*opt_205)
        .descrp = b"learned pruning clauses as redundant clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_205, b"prunered\0" as *const u8 as *const libc::c_char);
    let mut opt_206: *mut Opt = &mut (*opts).prunevsids;
    (*opt_206).lng = b"prunevsids\0" as *const u8 as *const libc::c_char;
    (*opt_206).val = 0 as libc::c_int;
    (*opt_206).dflt = (*opt_206).val;
    (*opt_206).min = 0 as libc::c_int;
    (*opt_206).max = 1 as libc::c_int;
    (*opt_206)
        .descrp = b"pruning decisions using default heuristic\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_206, b"prunevsids\0" as *const u8 as *const libc::c_char);
    let mut opt_207: *mut Opt = &mut (*opts).pure_0;
    (*opt_207).lng = b"pure\0" as *const u8 as *const libc::c_char;
    (*opt_207).val = 1 as libc::c_int;
    (*opt_207).dflt = (*opt_207).val;
    (*opt_207).min = 0 as libc::c_int;
    (*opt_207).max = 1 as libc::c_int;
    (*opt_207)
        .descrp = b"enable pure literal elimination during BCE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_207, b"pure\0" as *const u8 as *const libc::c_char);
    let mut opt_208: *mut Opt = &mut (*opts).quatres;
    (*opt_208).lng = b"quatres\0" as *const u8 as *const libc::c_char;
    (*opt_208).val = 1 as libc::c_int;
    (*opt_208).dflt = (*opt_208).val;
    (*opt_208).min = 0 as libc::c_int;
    (*opt_208).max = 1 as libc::c_int;
    (*opt_208)
        .descrp = b"enable quaternay resolution\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_208, b"quatres\0" as *const u8 as *const libc::c_char);
    let mut opt_209: *mut Opt = &mut (*opts).quatreswait;
    (*opt_209).lng = b"quatreswait\0" as *const u8 as *const libc::c_char;
    (*opt_209).val = 2 as libc::c_int;
    (*opt_209).dflt = (*opt_209).val;
    (*opt_209).min = 0 as libc::c_int;
    (*opt_209).max = 2 as libc::c_int;
    (*opt_209)
        .descrp = b"wait with quaternay resolution\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_209, b"quatreswait\0" as *const u8 as *const libc::c_char);
    let mut opt_210: *mut Opt = &mut (*opts).queuesort;
    (*opt_210).lng = b"queuesort\0" as *const u8 as *const libc::c_char;
    (*opt_210).val = 1 as libc::c_int;
    (*opt_210).dflt = (*opt_210).val;
    (*opt_210).min = 0 as libc::c_int;
    (*opt_210).max = 1 as libc::c_int;
    (*opt_210)
        .descrp = b"sort decision queue by JWH score\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_210, b"queuesort\0" as *const u8 as *const libc::c_char);
    let mut opt_211: *mut Opt = &mut (*opts).randec;
    (*opt_211).lng = b"randec\0" as *const u8 as *const libc::c_char;
    (*opt_211).val = 0 as libc::c_int;
    (*opt_211).dflt = (*opt_211).val;
    (*opt_211).min = 0 as libc::c_int;
    (*opt_211).max = 1 as libc::c_int;
    (*opt_211)
        .descrp = b"enable random decisions order\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_211, b"randec\0" as *const u8 as *const libc::c_char);
    let mut opt_212: *mut Opt = &mut (*opts).randecint;
    (*opt_212).lng = b"randecint\0" as *const u8 as *const libc::c_char;
    (*opt_212).val = 809 as libc::c_int;
    (*opt_212).dflt = (*opt_212).val;
    (*opt_212).min = 2 as libc::c_int;
    (*opt_212).max = I / 2 as libc::c_int;
    (*opt_212)
        .descrp = b"random decision order interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_212, b"randecint\0" as *const u8 as *const libc::c_char);
    let mut opt_213: *mut Opt = &mut (*opts).randphase;
    (*opt_213).lng = b"randphase\0" as *const u8 as *const libc::c_char;
    (*opt_213).val = 0 as libc::c_int;
    (*opt_213).dflt = (*opt_213).val;
    (*opt_213).min = 0 as libc::c_int;
    (*opt_213).max = 1 as libc::c_int;
    (*opt_213)
        .descrp = b"enable random decision phases\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_213, b"randphase\0" as *const u8 as *const libc::c_char);
    let mut opt_214: *mut Opt = &mut (*opts).randphaseint;
    (*opt_214).lng = b"randphaseint\0" as *const u8 as *const libc::c_char;
    (*opt_214).val = 503 as libc::c_int;
    (*opt_214).dflt = (*opt_214).val;
    (*opt_214).min = 2 as libc::c_int;
    (*opt_214).max = I / 2 as libc::c_int;
    (*opt_214)
        .descrp = b"random decision phases interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_214, b"randphaseint\0" as *const u8 as *const libc::c_char);
    let mut opt_215: *mut Opt = &mut (*opts).redcls;
    (*opt_215).lng = b"redcls\0" as *const u8 as *const libc::c_char;
    (*opt_215).val = 1 as libc::c_int;
    (*opt_215).dflt = (*opt_215).val;
    (*opt_215).min = 0 as libc::c_int;
    (*opt_215).max = 1 as libc::c_int;
    (*opt_215)
        .descrp = b"reduce literals in learned clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_215, b"redcls\0" as *const u8 as *const libc::c_char);
    let mut opt_216: *mut Opt = &mut (*opts).redclsglue;
    (*opt_216).lng = b"redclsglue\0" as *const u8 as *const libc::c_char;
    (*opt_216).val = 6 as libc::c_int;
    (*opt_216).dflt = (*opt_216).val;
    (*opt_216).min = 0 as libc::c_int;
    (*opt_216).max = I;
    (*opt_216)
        .descrp = b"upper glue limit for clause reduction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_216, b"redclsglue\0" as *const u8 as *const libc::c_char);
    let mut opt_217: *mut Opt = &mut (*opts).redclsize;
    (*opt_217).lng = b"redclsize\0" as *const u8 as *const libc::c_char;
    (*opt_217).val = 30 as libc::c_int;
    (*opt_217).dflt = (*opt_217).val;
    (*opt_217).min = 0 as libc::c_int;
    (*opt_217).max = I;
    (*opt_217)
        .descrp = b"size limit reducing literals in learned clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_217, b"redclsize\0" as *const u8 as *const libc::c_char);
    let mut opt_218: *mut Opt = &mut (*opts).redclsmaxdec;
    (*opt_218).lng = b"redclsmaxdec\0" as *const u8 as *const libc::c_char;
    (*opt_218).val = 5 as libc::c_int;
    (*opt_218).dflt = (*opt_218).val;
    (*opt_218).min = 1 as libc::c_int;
    (*opt_218).max = I;
    (*opt_218)
        .descrp = b"max decisions checked per lit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_218, b"redclsmaxdec\0" as *const u8 as *const libc::c_char);
    let mut opt_219: *mut Opt = &mut (*opts).redclsmaxdepth;
    (*opt_219).lng = b"redclsmaxdepth\0" as *const u8 as *const libc::c_char;
    (*opt_219).val = 10 as libc::c_int;
    (*opt_219).dflt = (*opt_219).val;
    (*opt_219).min = 1 as libc::c_int;
    (*opt_219).max = I;
    (*opt_219)
        .descrp = b"max depth for propagation per lit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_219, b"redclsmaxdepth\0" as *const u8 as *const libc::c_char);
    let mut opt_220: *mut Opt = &mut (*opts).redclsmaxlrg;
    (*opt_220).lng = b"redclsmaxlrg\0" as *const u8 as *const libc::c_char;
    (*opt_220).val = 10 as libc::c_int;
    (*opt_220).dflt = (*opt_220).val;
    (*opt_220).min = 0 as libc::c_int;
    (*opt_220).max = I;
    (*opt_220)
        .descrp = b"max large checked per lit for redcls\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_220, b"redclsmaxlrg\0" as *const u8 as *const libc::c_char);
    let mut opt_221: *mut Opt = &mut (*opts).redclsmaxprops;
    (*opt_221).lng = b"redclsmaxprops\0" as *const u8 as *const libc::c_char;
    (*opt_221).val = 100 as libc::c_int;
    (*opt_221).dflt = (*opt_221).val;
    (*opt_221).min = 0 as libc::c_int;
    (*opt_221).max = I;
    (*opt_221)
        .descrp = b"max props per lit for redcls\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_221, b"redclsmaxprops\0" as *const u8 as *const libc::c_char);
    let mut opt_222: *mut Opt = &mut (*opts).redclstype;
    (*opt_222).lng = b"redclstype\0" as *const u8 as *const libc::c_char;
    (*opt_222).val = 4 as libc::c_int;
    (*opt_222).dflt = (*opt_222).val;
    (*opt_222).min = 2 as libc::c_int;
    (*opt_222).max = 4 as libc::c_int;
    (*opt_222)
        .descrp = b"type of clauses used for reduction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_222, b"redclstype\0" as *const u8 as *const libc::c_char);
    let mut opt_223: *mut Opt = &mut (*opts).reduce;
    (*opt_223).lng = b"reduce\0" as *const u8 as *const libc::c_char;
    (*opt_223).val = 1 as libc::c_int;
    (*opt_223).dflt = (*opt_223).val;
    (*opt_223).min = 0 as libc::c_int;
    (*opt_223).max = 1 as libc::c_int;
    (*opt_223).descrp = b"enable clause reduction\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_223, b"reduce\0" as *const u8 as *const libc::c_char);
    let mut opt_224: *mut Opt = &mut (*opts).reducefixed;
    (*opt_224).lng = b"reducefixed\0" as *const u8 as *const libc::c_char;
    (*opt_224).val = 0 as libc::c_int;
    (*opt_224).dflt = (*opt_224).val;
    (*opt_224).min = 0 as libc::c_int;
    (*opt_224).max = 1 as libc::c_int;
    (*opt_224)
        .descrp = b"enabled fixed bound on learned clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_224, b"reducefixed\0" as *const u8 as *const libc::c_char);
    let mut opt_225: *mut Opt = &mut (*opts).reduceinc;
    (*opt_225).lng = b"reduceinc\0" as *const u8 as *const libc::c_char;
    (*opt_225).val = 300 as libc::c_int;
    (*opt_225).dflt = (*opt_225).val;
    (*opt_225).min = 1 as libc::c_int;
    (*opt_225).max = 10 as libc::c_int * M;
    (*opt_225).descrp = b"reduce limit increment\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_225, b"reduceinc\0" as *const u8 as *const libc::c_char);
    let mut opt_226: *mut Opt = &mut (*opts).reduceinit;
    (*opt_226).lng = b"reduceinit\0" as *const u8 as *const libc::c_char;
    (*opt_226).val = 2 as libc::c_int * K;
    (*opt_226).dflt = (*opt_226).val;
    (*opt_226).min = 1 as libc::c_int;
    (*opt_226).max = 100 as libc::c_int * M;
    (*opt_226).descrp = b"initial reduce limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_226, b"reduceinit\0" as *const u8 as *const libc::c_char);
    let mut opt_227: *mut Opt = &mut (*opts).reducereset;
    (*opt_227).lng = b"reducereset\0" as *const u8 as *const libc::c_char;
    (*opt_227).val = 0 as libc::c_int;
    (*opt_227).dflt = (*opt_227).val;
    (*opt_227).min = 0 as libc::c_int;
    (*opt_227).max = 2 as libc::c_int;
    (*opt_227)
        .descrp = b"enable reduce increment reset\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_227, b"reducereset\0" as *const u8 as *const libc::c_char);
    let mut opt_228: *mut Opt = &mut (*opts).restart;
    (*opt_228).lng = b"restart\0" as *const u8 as *const libc::c_char;
    (*opt_228).val = 1 as libc::c_int;
    (*opt_228).dflt = (*opt_228).val;
    (*opt_228).min = 0 as libc::c_int;
    (*opt_228).max = 1 as libc::c_int;
    (*opt_228).descrp = b"enable restarting\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_228, b"restart\0" as *const u8 as *const libc::c_char);
    let mut opt_229: *mut Opt = &mut (*opts).restartfixed;
    (*opt_229).lng = b"restartfixed\0" as *const u8 as *const libc::c_char;
    (*opt_229).val = 0 as libc::c_int;
    (*opt_229).dflt = (*opt_229).val;
    (*opt_229).min = 0 as libc::c_int;
    (*opt_229).max = 1 as libc::c_int;
    (*opt_229).descrp = b"fixed restart\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_229, b"restartfixed\0" as *const u8 as *const libc::c_char);
    let mut opt_230: *mut Opt = &mut (*opts).restartblock;
    (*opt_230).lng = b"restartblock\0" as *const u8 as *const libc::c_char;
    (*opt_230).val = 0 as libc::c_int;
    (*opt_230).dflt = (*opt_230).val;
    (*opt_230).min = 0 as libc::c_int;
    (*opt_230).max = 2 as libc::c_int;
    (*opt_230)
        .descrp = b"enable restart blocking (1=conflict,2=restart)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_230, b"restartblock\0" as *const u8 as *const libc::c_char);
    let mut opt_231: *mut Opt = &mut (*opts).restartblocklim;
    (*opt_231).lng = b"restartblocklim\0" as *const u8 as *const libc::c_char;
    (*opt_231).val = 200 as libc::c_int;
    (*opt_231).dflt = (*opt_231).val;
    (*opt_231).min = 1 as libc::c_int;
    (*opt_231).max = K;
    (*opt_231)
        .descrp = b"restart blocking limit percent (glucose R)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_231, b"restartblocklim\0" as *const u8 as *const libc::c_char);
    let mut opt_232: *mut Opt = &mut (*opts).restartblockbound;
    (*opt_232).lng = b"restartblockbound\0" as *const u8 as *const libc::c_char;
    (*opt_232).val = 10 as libc::c_int * K;
    (*opt_232).dflt = (*opt_232).val;
    (*opt_232).min = 0 as libc::c_int;
    (*opt_232).max = I;
    (*opt_232)
        .descrp = b"restart blocking conflict lower bound\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_232, b"restartblockbound\0" as *const u8 as *const libc::c_char);
    let mut opt_233: *mut Opt = &mut (*opts).restartcheckforced;
    (*opt_233).lng = b"restartcheckforced\0" as *const u8 as *const libc::c_char;
    (*opt_233).val = 1 as libc::c_int;
    (*opt_233).dflt = (*opt_233).val;
    (*opt_233).min = 0 as libc::c_int;
    (*opt_233).max = 1 as libc::c_int;
    (*opt_233)
        .descrp = b"enable skipping restarts if not forced\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_233, b"restartcheckforced\0" as *const u8 as *const libc::c_char);
    let mut opt_234: *mut Opt = &mut (*opts).restartdelay;
    (*opt_234).lng = b"restartdelay\0" as *const u8 as *const libc::c_char;
    (*opt_234).val = 1 as libc::c_int;
    (*opt_234).dflt = (*opt_234).val;
    (*opt_234).min = 0 as libc::c_int;
    (*opt_234).max = 1 as libc::c_int;
    (*opt_234)
        .descrp = b"enable restart delaying based on jump level\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_234, b"restartdelay\0" as *const u8 as *const libc::c_char);
    let mut opt_235: *mut Opt = &mut (*opts).restartdelaylim;
    (*opt_235).lng = b"restartdelaylim\0" as *const u8 as *const libc::c_char;
    (*opt_235).val = 50 as libc::c_int;
    (*opt_235).dflt = (*opt_235).val;
    (*opt_235).min = 1 as libc::c_int;
    (*opt_235).max = K;
    (*opt_235)
        .descrp = b"restart delaying limit in percent\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_235, b"restartdelaylim\0" as *const u8 as *const libc::c_char);
    let mut opt_236: *mut Opt = &mut (*opts).restartint;
    (*opt_236).lng = b"restartint\0" as *const u8 as *const libc::c_char;
    (*opt_236).val = 10 as libc::c_int;
    (*opt_236).dflt = (*opt_236).val;
    (*opt_236).min = 1 as libc::c_int;
    (*opt_236).max = I;
    (*opt_236).descrp = b"base restart interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_236, b"restartint\0" as *const u8 as *const libc::c_char);
    let mut opt_237: *mut Opt = &mut (*opts).restartforcelim;
    (*opt_237).lng = b"restartforcelim\0" as *const u8 as *const libc::c_char;
    (*opt_237).val = 115 as libc::c_int;
    (*opt_237).dflt = (*opt_237).val;
    (*opt_237).min = 1 as libc::c_int;
    (*opt_237).max = K;
    (*opt_237)
        .descrp = b"restart forcing limit percent (glucose K)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_237, b"restartforcelim\0" as *const u8 as *const libc::c_char);
    let mut opt_238: *mut Opt = &mut (*opts).restartforcemode;
    (*opt_238).lng = b"restartforcemode\0" as *const u8 as *const libc::c_char;
    (*opt_238).val = 1 as libc::c_int;
    (*opt_238).dflt = (*opt_238).val;
    (*opt_238).min = 0 as libc::c_int;
    (*opt_238).max = 2 as libc::c_int;
    (*opt_238)
        .descrp = b"forced restart mode (0=avg,1=ema,2=macd)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_238, b"restartforcemode\0" as *const u8 as *const libc::c_char);
    let mut opt_239: *mut Opt = &mut (*opts).restartpen1;
    (*opt_239).lng = b"restartpen1\0" as *const u8 as *const libc::c_char;
    (*opt_239).val = 1 as libc::c_int;
    (*opt_239).dflt = (*opt_239).val;
    (*opt_239).min = 0 as libc::c_int;
    (*opt_239).max = 1 as libc::c_int;
    (*opt_239)
        .descrp = b"increase restart interval if few units\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_239, b"restartpen1\0" as *const u8 as *const libc::c_char);
    let mut opt_240: *mut Opt = &mut (*opts).restartpen2;
    (*opt_240).lng = b"restartpen2\0" as *const u8 as *const libc::c_char;
    (*opt_240).val = 1 as libc::c_int;
    (*opt_240).dflt = (*opt_240).val;
    (*opt_240).min = 0 as libc::c_int;
    (*opt_240).max = 1 as libc::c_int;
    (*opt_240)
        .descrp = b"increase restart interval if few binary clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_240, b"restartpen2\0" as *const u8 as *const libc::c_char);
    let mut opt_241: *mut Opt = &mut (*opts).restartpen3;
    (*opt_241).lng = b"restartpen3\0" as *const u8 as *const libc::c_char;
    (*opt_241).val = 1 as libc::c_int;
    (*opt_241).dflt = (*opt_241).val;
    (*opt_241).min = 0 as libc::c_int;
    (*opt_241).max = 1 as libc::c_int;
    (*opt_241)
        .descrp = b"increase restart interval if few ternary clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_241, b"restartpen3\0" as *const u8 as *const libc::c_char);
    let mut opt_242: *mut Opt = &mut (*opts).restartpenstab;
    (*opt_242).lng = b"restartpenstab\0" as *const u8 as *const libc::c_char;
    (*opt_242).val = 1 as libc::c_int;
    (*opt_242).dflt = (*opt_242).val;
    (*opt_242).min = 0 as libc::c_int;
    (*opt_242).max = 1 as libc::c_int;
    (*opt_242)
        .descrp = b"increase restart interval if stabilizing\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_242, b"restartpenstab\0" as *const u8 as *const libc::c_char);
    let mut opt_243: *mut Opt = &mut (*opts).retireint;
    (*opt_243).lng = b"retireint\0" as *const u8 as *const libc::c_char;
    (*opt_243).val = 4 as libc::c_int;
    (*opt_243).dflt = (*opt_243).val;
    (*opt_243).min = 0 as libc::c_int;
    (*opt_243).max = 1000 as libc::c_int;
    (*opt_243)
        .descrp = b"retire inactive clauses inprocessing phases count\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_243, b"retireint\0" as *const u8 as *const libc::c_char);
    let mut opt_244: *mut Opt = &mut (*opts).retiremin;
    (*opt_244).lng = b"retiremin\0" as *const u8 as *const libc::c_char;
    (*opt_244).val = 1 as libc::c_int;
    (*opt_244).dflt = (*opt_244).val;
    (*opt_244).min = 0 as libc::c_int;
    (*opt_244).max = I;
    (*opt_244)
        .descrp = b"minimum glue for retirement\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_244, b"retiremin\0" as *const u8 as *const libc::c_char);
    let mut opt_245: *mut Opt = &mut (*opts).retirenb;
    (*opt_245).lng = b"retirenb\0" as *const u8 as *const libc::c_char;
    (*opt_245).val = 1 as libc::c_int;
    (*opt_245).dflt = (*opt_245).val;
    (*opt_245).min = 0 as libc::c_int;
    (*opt_245).max = 1 as libc::c_int;
    (*opt_245)
        .descrp = b"enabled inactive clause retirement\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_245, b"retirenb\0" as *const u8 as *const libc::c_char);
    let mut opt_246: *mut Opt = &mut (*opts).reusetrail;
    (*opt_246).lng = b"reusetrail\0" as *const u8 as *const libc::c_char;
    (*opt_246).val = 1 as libc::c_int;
    (*opt_246).dflt = (*opt_246).val;
    (*opt_246).min = 0 as libc::c_int;
    (*opt_246).max = 1 as libc::c_int;
    (*opt_246).descrp = b"reuse trail\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_246, b"reusetrail\0" as *const u8 as *const libc::c_char);
    let mut opt_247: *mut Opt = &mut (*opts).rmincpen;
    (*opt_247).lng = b"rmincpen\0" as *const u8 as *const libc::c_char;
    (*opt_247).val = 4 as libc::c_int;
    (*opt_247).dflt = (*opt_247).val;
    (*opt_247).min = 0 as libc::c_int;
    (*opt_247).max = 32 as libc::c_int;
    (*opt_247)
        .descrp = b"logarithm of watcher removal penalty\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_247, b"rmincpen\0" as *const u8 as *const libc::c_char);
    let mut opt_248: *mut Opt = &mut (*opts).scincinc;
    (*opt_248).lng = b"scincinc\0" as *const u8 as *const libc::c_char;
    (*opt_248).val = 250 as libc::c_int;
    (*opt_248).dflt = (*opt_248).val;
    (*opt_248).min = 1 as libc::c_int;
    (*opt_248).max = 10 as libc::c_int * K;
    (*opt_248)
        .descrp = b"score increment increment in per mille\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_248, b"scincinc\0" as *const u8 as *const libc::c_char);
    let mut opt_249: *mut Opt = &mut (*opts).scincincdelta;
    (*opt_249).lng = b"scincincdelta\0" as *const u8 as *const libc::c_char;
    (*opt_249).val = 10 as libc::c_int;
    (*opt_249).dflt = (*opt_249).val;
    (*opt_249).min = 0 as libc::c_int;
    (*opt_249).max = 10 as libc::c_int * K;
    (*opt_249)
        .descrp = b"delta score inc inc in per mille\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_249, b"scincincdelta\0" as *const u8 as *const libc::c_char);
    let mut opt_250: *mut Opt = &mut (*opts).scincincincint;
    (*opt_250).lng = b"scincincincint\0" as *const u8 as *const libc::c_char;
    (*opt_250).val = 100 as libc::c_int * K;
    (*opt_250).dflt = (*opt_250).val;
    (*opt_250).min = 1 as libc::c_int;
    (*opt_250).max = I;
    (*opt_250)
        .descrp = b"score inc inc inc interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_250, b"scincincincint\0" as *const u8 as *const libc::c_char);
    let mut opt_251: *mut Opt = &mut (*opts).scincincmin;
    (*opt_251).lng = b"scincincmin\0" as *const u8 as *const libc::c_char;
    (*opt_251).val = 50 as libc::c_int;
    (*opt_251).dflt = (*opt_251).val;
    (*opt_251).min = 1 as libc::c_int;
    (*opt_251).max = 10 as libc::c_int * K;
    (*opt_251)
        .descrp = b"min score inc inc in per mille\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_251, b"scincincmin\0" as *const u8 as *const libc::c_char);
    let mut opt_252: *mut Opt = &mut (*opts).scincincmode;
    (*opt_252).lng = b"scincincmode\0" as *const u8 as *const libc::c_char;
    (*opt_252).val = 1 as libc::c_int;
    (*opt_252).dflt = (*opt_252).val;
    (*opt_252).min = 0 as libc::c_int;
    (*opt_252).max = 2 as libc::c_int;
    (*opt_252)
        .descrp = b"score inc inc mode (0=keep,1=delta,2=avg)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_252, b"scincincmode\0" as *const u8 as *const libc::c_char);
    let mut opt_253: *mut Opt = &mut (*opts).scoreshift;
    (*opt_253).lng = b"scoreshift\0" as *const u8 as *const libc::c_char;
    (*opt_253).val = 24 as libc::c_int;
    (*opt_253).dflt = (*opt_253).val;
    (*opt_253).min = 0 as libc::c_int;
    (*opt_253).max = 64 as libc::c_int;
    (*opt_253).descrp = b"score shift\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_253, b"scoreshift\0" as *const u8 as *const libc::c_char);
    let mut opt_254: *mut Opt = &mut (*opts).seed;
    (*opt_254).lng = b"seed\0" as *const u8 as *const libc::c_char;
    (*opt_254).val = 0 as libc::c_int;
    (*opt_254).dflt = (*opt_254).val;
    (*opt_254).min = 0 as libc::c_int;
    (*opt_254).max = I;
    (*opt_254)
        .descrp = b"random number generator seed\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_254, b"seed\0" as *const u8 as *const libc::c_char);
    let mut opt_255: *mut Opt = &mut (*opts).simpbintinc;
    (*opt_255).lng = b"simpbintinc\0" as *const u8 as *const libc::c_char;
    (*opt_255).val = 100 as libc::c_int;
    (*opt_255).dflt = (*opt_255).val;
    (*opt_255).min = 1 as libc::c_int;
    (*opt_255).max = I;
    (*opt_255)
        .descrp = b"inprocessing binary interval increment\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_255, b"simpbintinc\0" as *const u8 as *const libc::c_char);
    let mut opt_256: *mut Opt = &mut (*opts).simpbintinclim;
    (*opt_256).lng = b"simpbintinclim\0" as *const u8 as *const libc::c_char;
    (*opt_256).val = 10 as libc::c_int * K;
    (*opt_256).dflt = (*opt_256).val;
    (*opt_256).min = 1 as libc::c_int;
    (*opt_256).max = I;
    (*opt_256)
        .descrp = b"inprocessing bin int inc limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_256, b"simpbintinclim\0" as *const u8 as *const libc::c_char);
    let mut opt_257: *mut Opt = &mut (*opts).simpcintdelay;
    (*opt_257).lng = b"simpcintdelay\0" as *const u8 as *const libc::c_char;
    (*opt_257).val = 2000 as libc::c_int;
    (*opt_257).dflt = (*opt_257).val;
    (*opt_257).min = 0 as libc::c_int;
    (*opt_257).max = I;
    (*opt_257)
        .descrp = b"inprocessing conflict delay\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_257, b"simpcintdelay\0" as *const u8 as *const libc::c_char);
    let mut opt_258: *mut Opt = &mut (*opts).simpcintinc;
    (*opt_258).lng = b"simpcintinc\0" as *const u8 as *const libc::c_char;
    (*opt_258).val = 20 as libc::c_int * K;
    (*opt_258).dflt = (*opt_258).val;
    (*opt_258).min = 10 as libc::c_int;
    (*opt_258).max = M;
    (*opt_258)
        .descrp = b"inprocessing conflict interval increment\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_258, b"simpcintinc\0" as *const u8 as *const libc::c_char);
    let mut opt_259: *mut Opt = &mut (*opts).simpcintincdiv;
    (*opt_259).lng = b"simpcintincdiv\0" as *const u8 as *const libc::c_char;
    (*opt_259).val = 1 as libc::c_int;
    (*opt_259).dflt = (*opt_259).val;
    (*opt_259).min = 0 as libc::c_int;
    (*opt_259).max = 3 as libc::c_int;
    (*opt_259)
        .descrp = b"cintinc reduction: 0=no,1=div1,2=div2,3=heur\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_259, b"simpcintincdiv\0" as *const u8 as *const libc::c_char);
    let mut opt_260: *mut Opt = &mut (*opts).simpcintmaxhard;
    (*opt_260).lng = b"simpcintmaxhard\0" as *const u8 as *const libc::c_char;
    (*opt_260).val = 10 as libc::c_int * M;
    (*opt_260).dflt = (*opt_260).val;
    (*opt_260).min = -(1 as libc::c_int);
    (*opt_260).max = I;
    (*opt_260)
        .descrp = b"hard max conflict interval limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_260, b"simpcintmaxhard\0" as *const u8 as *const libc::c_char);
    let mut opt_261: *mut Opt = &mut (*opts).simpcintmaxsoft;
    (*opt_261).lng = b"simpcintmaxsoft\0" as *const u8 as *const libc::c_char;
    (*opt_261).val = 1 as libc::c_int * M;
    (*opt_261).dflt = (*opt_261).val;
    (*opt_261).min = -(1 as libc::c_int);
    (*opt_261).max = I;
    (*opt_261)
        .descrp = b"soft max conflict interval limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_261, b"simpcintmaxsoft\0" as *const u8 as *const libc::c_char);
    let mut opt_262: *mut Opt = &mut (*opts).simpidiv;
    (*opt_262).lng = b"simpidiv\0" as *const u8 as *const libc::c_char;
    (*opt_262).val = 3 as libc::c_int;
    (*opt_262).dflt = (*opt_262).val;
    (*opt_262).min = 1 as libc::c_int;
    (*opt_262).max = 100 as libc::c_int;
    (*opt_262)
        .descrp = b"simplification inter delay divisor\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_262, b"simpidiv\0" as *const u8 as *const libc::c_char);
    let mut opt_263: *mut Opt = &mut (*opts).simpincdelmaxfact;
    (*opt_263).lng = b"simpincdelmaxfact\0" as *const u8 as *const libc::c_char;
    (*opt_263).val = 50 as libc::c_int;
    (*opt_263).dflt = (*opt_263).val;
    (*opt_263).min = 0 as libc::c_int;
    (*opt_263).max = 1000 as libc::c_int;
    (*opt_263)
        .descrp = b"inproc incremental delay max fact\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_263, b"simpincdelmaxfact\0" as *const u8 as *const libc::c_char);
    let mut opt_264: *mut Opt = &mut (*opts).simpincdelmaxmin;
    (*opt_264).lng = b"simpincdelmaxmin\0" as *const u8 as *const libc::c_char;
    (*opt_264).val = 10 as libc::c_int * K;
    (*opt_264).dflt = (*opt_264).val;
    (*opt_264).min = 0 as libc::c_int;
    (*opt_264).max = I;
    (*opt_264)
        .descrp = b"inproc incremental delay max min confs\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_264, b"simpincdelmaxmin\0" as *const u8 as *const libc::c_char);
    let mut opt_265: *mut Opt = &mut (*opts).simpinitdelay;
    (*opt_265).lng = b"simpinitdelay\0" as *const u8 as *const libc::c_char;
    (*opt_265).val = 0 as libc::c_int;
    (*opt_265).dflt = (*opt_265).val;
    (*opt_265).min = 0 as libc::c_int;
    (*opt_265).max = I;
    (*opt_265)
        .descrp = b"initial simplification delay\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_265, b"simpinitdelay\0" as *const u8 as *const libc::c_char);
    let mut opt_266: *mut Opt = &mut (*opts).simpintsizepen;
    (*opt_266).lng = b"simpintsizepen\0" as *const u8 as *const libc::c_char;
    (*opt_266).val = 0 as libc::c_int;
    (*opt_266).dflt = (*opt_266).val;
    (*opt_266).min = 0 as libc::c_int;
    (*opt_266).max = 1 as libc::c_int;
    (*opt_266)
        .descrp = b"penalize interval (positively) by size\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_266, b"simpintsizepen\0" as *const u8 as *const libc::c_char);
    let mut opt_267: *mut Opt = &mut (*opts).simpiscale;
    (*opt_267).lng = b"simpiscale\0" as *const u8 as *const libc::c_char;
    (*opt_267).val = 100 as libc::c_int;
    (*opt_267).dflt = (*opt_267).val;
    (*opt_267).min = 1 as libc::c_int;
    (*opt_267).max = 10000 as libc::c_int;
    (*opt_267)
        .descrp = b"relative simplification inter delay scale\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_267, b"simpiscale\0" as *const u8 as *const libc::c_char);
    let mut opt_268: *mut Opt = &mut (*opts).simpitdelay;
    (*opt_268).lng = b"simpitdelay\0" as *const u8 as *const libc::c_char;
    (*opt_268).val = 10 as libc::c_int;
    (*opt_268).dflt = (*opt_268).val;
    (*opt_268).min = 0 as libc::c_int;
    (*opt_268).max = 1000 as libc::c_int;
    (*opt_268)
        .descrp = b"delay inpr by simpitdelay/delta-conf-per-it\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_268, b"simpitdelay\0" as *const u8 as *const libc::c_char);
    let mut opt_269: *mut Opt = &mut (*opts).simpjleveldecdelay;
    (*opt_269).lng = b"simpjleveldecdelay\0" as *const u8 as *const libc::c_char;
    (*opt_269).val = 1 as libc::c_int;
    (*opt_269).dflt = (*opt_269).val;
    (*opt_269).min = 0 as libc::c_int;
    (*opt_269).max = 1 as libc::c_int;
    (*opt_269)
        .descrp = b"delay simp if jlevel decreases\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_269, b"simpjleveldecdelay\0" as *const u8 as *const libc::c_char);
    let mut opt_270: *mut Opt = &mut (*opts).simpitintdecdelay;
    (*opt_270).lng = b"simpitintdecdelay\0" as *const u8 as *const libc::c_char;
    (*opt_270).val = 1 as libc::c_int;
    (*opt_270).dflt = (*opt_270).val;
    (*opt_270).min = 0 as libc::c_int;
    (*opt_270).max = 1 as libc::c_int;
    (*opt_270)
        .descrp = b"delay simp if iteration interval decreases\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_270, b"simpitintdecdelay\0" as *const u8 as *const libc::c_char);
    let mut opt_271: *mut Opt = &mut (*opts).simpitintinc;
    (*opt_271).lng = b"simpitintinc\0" as *const u8 as *const libc::c_char;
    (*opt_271).val = 10 as libc::c_int;
    (*opt_271).dflt = (*opt_271).val;
    (*opt_271).min = 1 as libc::c_int;
    (*opt_271).max = I;
    (*opt_271)
        .descrp = b"inprocessing iteration interval increment\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_271, b"simpitintinc\0" as *const u8 as *const libc::c_char);
    let mut opt_272: *mut Opt = &mut (*opts).simpitintinclim;
    (*opt_272).lng = b"simpitintinclim\0" as *const u8 as *const libc::c_char;
    (*opt_272).val = 1 as libc::c_int * K;
    (*opt_272).dflt = (*opt_272).val;
    (*opt_272).min = 1 as libc::c_int;
    (*opt_272).max = I;
    (*opt_272)
        .descrp = b"inprocessing its int inc limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_272, b"simpitintinclim\0" as *const u8 as *const libc::c_char);
    let mut opt_273: *mut Opt = &mut (*opts).simplify;
    (*opt_273).lng = b"simplify\0" as *const u8 as *const libc::c_char;
    (*opt_273).val = 2 as libc::c_int;
    (*opt_273).dflt = (*opt_273).val;
    (*opt_273).min = 0 as libc::c_int;
    (*opt_273).max = 2 as libc::c_int;
    (*opt_273).descrp = b"enable simplification\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_273, b"simplify\0" as *const u8 as *const libc::c_char);
    let mut opt_274: *mut Opt = &mut (*opts).simprtc;
    (*opt_274).lng = b"simprtc\0" as *const u8 as *const libc::c_char;
    (*opt_274).val = 5 as libc::c_int;
    (*opt_274).dflt = (*opt_274).val;
    (*opt_274).min = 1 as libc::c_int;
    (*opt_274).max = 100 as libc::c_int;
    (*opt_274)
        .descrp = b"min var reduction for simplification RTC\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_274, b"simprtc\0" as *const u8 as *const libc::c_char);
    let mut opt_275: *mut Opt = &mut (*opts).simptintinc;
    (*opt_275).lng = b"simptintinc\0" as *const u8 as *const libc::c_char;
    (*opt_275).val = 1000 as libc::c_int;
    (*opt_275).dflt = (*opt_275).val;
    (*opt_275).min = 1 as libc::c_int;
    (*opt_275).max = I;
    (*opt_275)
        .descrp = b"inprocessing ternary interval increment\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_275, b"simptintinc\0" as *const u8 as *const libc::c_char);
    let mut opt_276: *mut Opt = &mut (*opts).simptintinclim;
    (*opt_276).lng = b"simptintinclim\0" as *const u8 as *const libc::c_char;
    (*opt_276).val = 10 as libc::c_int * K;
    (*opt_276).dflt = (*opt_276).val;
    (*opt_276).min = 1 as libc::c_int;
    (*opt_276).max = I;
    (*opt_276)
        .descrp = b"inprocessing trn int inc limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_276, b"simptintinclim\0" as *const u8 as *const libc::c_char);
    let mut opt_277: *mut Opt = &mut (*opts).simpvarchg;
    (*opt_277).lng = b"simpvarchg\0" as *const u8 as *const libc::c_char;
    (*opt_277).val = 100 as libc::c_int;
    (*opt_277).dflt = (*opt_277).val;
    (*opt_277).min = 1 as libc::c_int;
    (*opt_277).max = 1000 as libc::c_int;
    (*opt_277)
        .descrp = b"simp remaining vars percentage change lim\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_277, b"simpvarchg\0" as *const u8 as *const libc::c_char);
    let mut opt_278: *mut Opt = &mut (*opts).simpvarlim;
    (*opt_278).lng = b"simpvarlim\0" as *const u8 as *const libc::c_char;
    (*opt_278).val = 100 as libc::c_int;
    (*opt_278).dflt = (*opt_278).val;
    (*opt_278).min = 0 as libc::c_int;
    (*opt_278).max = I;
    (*opt_278)
        .descrp = b"simp remaining vars min limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_278, b"simpvarlim\0" as *const u8 as *const libc::c_char);
    let mut opt_279: *mut Opt = &mut (*opts).sizemaxpen;
    (*opt_279).lng = b"sizemaxpen\0" as *const u8 as *const libc::c_char;
    (*opt_279).val = 5 as libc::c_int;
    (*opt_279).dflt = (*opt_279).val;
    (*opt_279).min = 0 as libc::c_int;
    (*opt_279).max = 20 as libc::c_int;
    (*opt_279)
        .descrp = b"maximum logarithmic size penalty\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_279, b"sizemaxpen\0" as *const u8 as *const libc::c_char);
    let mut opt_280: *mut Opt = &mut (*opts).sizepen;
    (*opt_280).lng = b"sizepen\0" as *const u8 as *const libc::c_char;
    (*opt_280).val = 1 as libc::c_int * M;
    (*opt_280).dflt = (*opt_280).val;
    (*opt_280).min = 1 as libc::c_int;
    (*opt_280).max = I;
    (*opt_280)
        .descrp = b"number of clauses size penalty starting point\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_280, b"sizepen\0" as *const u8 as *const libc::c_char);
    let mut opt_281: *mut Opt = &mut (*opts).sleeponabort;
    (*opt_281).lng = b"sleeponabort\0" as *const u8 as *const libc::c_char;
    (*opt_281).val = 0 as libc::c_int;
    (*opt_281).dflt = (*opt_281).val;
    (*opt_281).min = 0 as libc::c_int;
    (*opt_281).max = I;
    (*opt_281)
        .descrp = b"sleep this seconds before abort/exit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_281, b"sleeponabort\0" as *const u8 as *const libc::c_char);
    let mut opt_282: *mut Opt = &mut (*opts).smallirr;
    (*opt_282).lng = b"smallirr\0" as *const u8 as *const libc::c_char;
    (*opt_282).val = 90 as libc::c_int;
    (*opt_282).dflt = (*opt_282).val;
    (*opt_282).min = 0 as libc::c_int;
    (*opt_282).max = 100 as libc::c_int;
    (*opt_282)
        .descrp = b"max percentage irr lits for BCE and VE\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_282, b"smallirr\0" as *const u8 as *const libc::c_char);
    let mut opt_283: *mut Opt = &mut (*opts).smallve;
    (*opt_283).lng = b"smallve\0" as *const u8 as *const libc::c_char;
    (*opt_283).val = 1 as libc::c_int;
    (*opt_283).dflt = (*opt_283).val;
    (*opt_283).min = 0 as libc::c_int;
    (*opt_283).max = 1 as libc::c_int;
    (*opt_283)
        .descrp = b"enable small number variables elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_283, b"smallve\0" as *const u8 as *const libc::c_char);
    let mut opt_284: *mut Opt = &mut (*opts).smallvevars;
    (*opt_284).lng = b"smallvevars\0" as *const u8 as *const libc::c_char;
    (*opt_284).val = 12 as libc::c_int;
    (*opt_284).dflt = (*opt_284).val;
    (*opt_284).min = 4 as libc::c_int;
    (*opt_284).max = 12 as libc::c_int;
    (*opt_284)
        .descrp = b"variables small variable elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_284, b"smallvevars\0" as *const u8 as *const libc::c_char);
    let mut opt_285: *mut Opt = &mut (*opts).smallvewait;
    (*opt_285).lng = b"smallvewait\0" as *const u8 as *const libc::c_char;
    (*opt_285).val = 0 as libc::c_int;
    (*opt_285).dflt = (*opt_285).val;
    (*opt_285).min = 0 as libc::c_int;
    (*opt_285).max = 1 as libc::c_int;
    (*opt_285)
        .descrp = b"wait with small variable elimination\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_285, b"smallvewait\0" as *const u8 as *const libc::c_char);
    let mut opt_286: *mut Opt = &mut (*opts).sortlits;
    (*opt_286).lng = b"sortlits\0" as *const u8 as *const libc::c_char;
    (*opt_286).val = 0 as libc::c_int;
    (*opt_286).dflt = (*opt_286).val;
    (*opt_286).min = 0 as libc::c_int;
    (*opt_286).max = 1 as libc::c_int;
    (*opt_286)
        .descrp = b"sort lits of cls during garbage collection\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_286, b"sortlits\0" as *const u8 as *const libc::c_char);
    let mut opt_287: *mut Opt = &mut (*opts).stabema;
    (*opt_287).lng = b"stabema\0" as *const u8 as *const libc::c_char;
    (*opt_287).val = 7 as libc::c_int;
    (*opt_287).dflt = (*opt_287).val;
    (*opt_287).min = 0 as libc::c_int;
    (*opt_287).max = 32 as libc::c_int;
    (*opt_287)
        .descrp = b"e for stability EMA with alpha=2^-e\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_287, b"stabema\0" as *const u8 as *const libc::c_char);
    let mut opt_288: *mut Opt = &mut (*opts).subl;
    (*opt_288).lng = b"subl\0" as *const u8 as *const libc::c_char;
    (*opt_288).val = 0 as libc::c_int;
    (*opt_288).dflt = (*opt_288).val;
    (*opt_288).min = 0 as libc::c_int;
    (*opt_288).max = 10 as libc::c_int * K;
    (*opt_288)
        .descrp = b"try to subsume this many recent learned clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_288, b"subl\0" as *const u8 as *const libc::c_char);
    let mut opt_289: *mut Opt = &mut (*opts).sweep;
    (*opt_289).lng = b"sweep\0" as *const u8 as *const libc::c_char;
    (*opt_289).val = 1 as libc::c_int;
    (*opt_289).dflt = (*opt_289).val;
    (*opt_289).min = 0 as libc::c_int;
    (*opt_289).max = 1 as libc::c_int;
    (*opt_289).descrp = b"enabled SAT sweeping\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_289, b"sweep\0" as *const u8 as *const libc::c_char);
    let mut opt_290: *mut Opt = &mut (*opts).sweepboost;
    (*opt_290).lng = b"sweepboost\0" as *const u8 as *const libc::c_char;
    (*opt_290).val = 10 as libc::c_int;
    (*opt_290).dflt = (*opt_290).val;
    (*opt_290).min = 1 as libc::c_int;
    (*opt_290).max = 1000 as libc::c_int;
    (*opt_290).descrp = b"sweeping boost\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_290, b"sweepboost\0" as *const u8 as *const libc::c_char);
    let mut opt_291: *mut Opt = &mut (*opts).sweepboostdel;
    (*opt_291).lng = b"sweepboostdel\0" as *const u8 as *const libc::c_char;
    (*opt_291).val = 4 as libc::c_int;
    (*opt_291).dflt = (*opt_291).val;
    (*opt_291).min = 0 as libc::c_int;
    (*opt_291).max = 100 as libc::c_int;
    (*opt_291)
        .descrp = b"initial sweeping boost delay\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_291, b"sweepboostdel\0" as *const u8 as *const libc::c_char);
    let mut opt_292: *mut Opt = &mut (*opts).sweepboostint;
    (*opt_292).lng = b"sweepboostint\0" as *const u8 as *const libc::c_char;
    (*opt_292).val = 7 as libc::c_int;
    (*opt_292).dflt = (*opt_292).val;
    (*opt_292).min = 1 as libc::c_int;
    (*opt_292).max = I;
    (*opt_292).descrp = b"sweeping boost interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_292, b"sweepboostint\0" as *const u8 as *const libc::c_char);
    let mut opt_293: *mut Opt = &mut (*opts).sweepboostvlim;
    (*opt_293).lng = b"sweepboostvlim\0" as *const u8 as *const libc::c_char;
    (*opt_293).val = 1 as libc::c_int * M;
    (*opt_293).dflt = (*opt_293).val;
    (*opt_293).min = 1 as libc::c_int;
    (*opt_293).max = I;
    (*opt_293).descrp = b"sweeping boost var lim\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_293, b"sweepboostvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_294: *mut Opt = &mut (*opts).sweepfacdec;
    (*opt_294).lng = b"sweepfacdec\0" as *const u8 as *const libc::c_char;
    (*opt_294).val = 10 as libc::c_int;
    (*opt_294).dflt = (*opt_294).val;
    (*opt_294).min = 1 as libc::c_int;
    (*opt_294).max = 100 as libc::c_int;
    (*opt_294).descrp = b"decisions limit factor\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_294, b"sweepfacdec\0" as *const u8 as *const libc::c_char);
    let mut opt_295: *mut Opt = &mut (*opts).sweepforgive;
    (*opt_295).lng = b"sweepforgive\0" as *const u8 as *const libc::c_char;
    (*opt_295).val = 2 as libc::c_int;
    (*opt_295).dflt = (*opt_295).val;
    (*opt_295).min = 0 as libc::c_int;
    (*opt_295).max = I;
    (*opt_295)
        .descrp = b"forgive that many unsucessful rounds\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_295, b"sweepforgive\0" as *const u8 as *const libc::c_char);
    let mut opt_296: *mut Opt = &mut (*opts).sweepirr;
    (*opt_296).lng = b"sweepirr\0" as *const u8 as *const libc::c_char;
    (*opt_296).val = 3 as libc::c_int;
    (*opt_296).dflt = (*opt_296).val;
    (*opt_296).min = 0 as libc::c_int;
    (*opt_296).max = 3 as libc::c_int;
    (*opt_296)
        .descrp = b"irredundant clauses (1=bin,2=trn,3=lrg)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_296, b"sweepirr\0" as *const u8 as *const libc::c_char);
    let mut opt_297: *mut Opt = &mut (*opts).sweepmaxdec;
    (*opt_297).lng = b"sweepmaxdec\0" as *const u8 as *const libc::c_char;
    (*opt_297).val = 10 as libc::c_int * K;
    (*opt_297).dflt = (*opt_297).val;
    (*opt_297).min = 0 as libc::c_int;
    (*opt_297).max = I;
    (*opt_297)
        .descrp = b"maximum decisions in one sweep implies check\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_297, b"sweepmaxdec\0" as *const u8 as *const libc::c_char);
    let mut opt_298: *mut Opt = &mut (*opts).sweepmaxeff;
    (*opt_298).lng = b"sweepmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_298).val = 200 as libc::c_int * M;
    (*opt_298).dflt = (*opt_298).val;
    (*opt_298).min = -(1 as libc::c_int);
    (*opt_298).max = I;
    (*opt_298).descrp = b"max effort in sweeping\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_298, b"sweepmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_299: *mut Opt = &mut (*opts).sweepmaxround;
    (*opt_299).lng = b"sweepmaxround\0" as *const u8 as *const libc::c_char;
    (*opt_299).val = 3 as libc::c_int;
    (*opt_299).dflt = (*opt_299).val;
    (*opt_299).min = -(1 as libc::c_int);
    (*opt_299).max = I;
    (*opt_299)
        .descrp = b"maximum rounds in sweeping\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_299, b"sweepmaxround\0" as *const u8 as *const libc::c_char);
    let mut opt_300: *mut Opt = &mut (*opts).sweepmindec;
    (*opt_300).lng = b"sweepmindec\0" as *const u8 as *const libc::c_char;
    (*opt_300).val = 100 as libc::c_int;
    (*opt_300).dflt = (*opt_300).val;
    (*opt_300).min = 0 as libc::c_int;
    (*opt_300).max = I;
    (*opt_300)
        .descrp = b"mininum decisions in one sweep implies check\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_300, b"sweepmindec\0" as *const u8 as *const libc::c_char);
    let mut opt_301: *mut Opt = &mut (*opts).sweepmineff;
    (*opt_301).lng = b"sweepmineff\0" as *const u8 as *const libc::c_char;
    (*opt_301).val = M;
    (*opt_301).dflt = (*opt_301).val;
    (*opt_301).min = 0 as libc::c_int;
    (*opt_301).max = I;
    (*opt_301).descrp = b"min effort in sweeping\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_301, b"sweepmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_302: *mut Opt = &mut (*opts).sweepred;
    (*opt_302).lng = b"sweepred\0" as *const u8 as *const libc::c_char;
    (*opt_302).val = 3 as libc::c_int;
    (*opt_302).dflt = (*opt_302).val;
    (*opt_302).min = 0 as libc::c_int;
    (*opt_302).max = 3 as libc::c_int;
    (*opt_302)
        .descrp = b"include redundant clauses (1=bin,2=trn,3=lrg)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_302, b"sweepred\0" as *const u8 as *const libc::c_char);
    let mut opt_303: *mut Opt = &mut (*opts).sweepreleff;
    (*opt_303).lng = b"sweepreleff\0" as *const u8 as *const libc::c_char;
    (*opt_303).val = 3 as libc::c_int;
    (*opt_303).dflt = (*opt_303).val;
    (*opt_303).min = 0 as libc::c_int;
    (*opt_303).max = 10 as libc::c_int * K;
    (*opt_303).descrp = b"rel effort in sweeping\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_303, b"sweepreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_304: *mut Opt = &mut (*opts).sweeprtc;
    (*opt_304).lng = b"sweeprtc\0" as *const u8 as *const libc::c_char;
    (*opt_304).val = 0 as libc::c_int;
    (*opt_304).dflt = (*opt_304).val;
    (*opt_304).min = 0 as libc::c_int;
    (*opt_304).max = 1 as libc::c_int;
    (*opt_304)
        .descrp = b"run sweeping until completion\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_304, b"sweeprtc\0" as *const u8 as *const libc::c_char);
    let mut opt_305: *mut Opt = &mut (*opts).sweeprtcint;
    (*opt_305).lng = b"sweeprtcint\0" as *const u8 as *const libc::c_char;
    (*opt_305).val = 14 as libc::c_int;
    (*opt_305).dflt = (*opt_305).val;
    (*opt_305).min = 1 as libc::c_int;
    (*opt_305).max = I;
    (*opt_305)
        .descrp = b"run sweeping until completion interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_305, b"sweeprtcint\0" as *const u8 as *const libc::c_char);
    let mut opt_306: *mut Opt = &mut (*opts).sweeprtcintvlim;
    (*opt_306).lng = b"sweeprtcintvlim\0" as *const u8 as *const libc::c_char;
    (*opt_306).val = 100 as libc::c_int * K;
    (*opt_306).dflt = (*opt_306).val;
    (*opt_306).min = 1 as libc::c_int;
    (*opt_306).max = I;
    (*opt_306)
        .descrp = b"run sweeping until completion int var lim\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_306, b"sweeprtcintvlim\0" as *const u8 as *const libc::c_char);
    let mut opt_307: *mut Opt = &mut (*opts).sweepsuccessmaxwortc;
    (*opt_307).lng = b"sweepsuccessmaxwortc\0" as *const u8 as *const libc::c_char;
    (*opt_307).val = 6 as libc::c_int;
    (*opt_307).dflt = (*opt_307).val;
    (*opt_307).min = 1 as libc::c_int;
    (*opt_307).max = I;
    (*opt_307)
        .descrp = b"sweeping success max wo run-to-completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(
        lgl,
        opt_307,
        b"sweepsuccessmaxwortc\0" as *const u8 as *const libc::c_char,
    );
    let mut opt_308: *mut Opt = &mut (*opts).sweepsuccessrat;
    (*opt_308).lng = b"sweepsuccessrat\0" as *const u8 as *const libc::c_char;
    (*opt_308).val = 1000 as libc::c_int;
    (*opt_308).dflt = (*opt_308).val;
    (*opt_308).min = 1 as libc::c_int;
    (*opt_308).max = I;
    (*opt_308).descrp = b"sweeping success ratio\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_308, b"sweepsuccessrat\0" as *const u8 as *const libc::c_char);
    let mut opt_309: *mut Opt = &mut (*opts).sweepwait;
    (*opt_309).lng = b"sweepwait\0" as *const u8 as *const libc::c_char;
    (*opt_309).val = 2 as libc::c_int;
    (*opt_309).dflt = (*opt_309).val;
    (*opt_309).min = 0 as libc::c_int;
    (*opt_309).max = 2 as libc::c_int;
    (*opt_309)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_309, b"sweepwait\0" as *const u8 as *const libc::c_char);
    let mut opt_310: *mut Opt = &mut (*opts).synclsall;
    (*opt_310).lng = b"synclsall\0" as *const u8 as *const libc::c_char;
    (*opt_310).val = 1 as libc::c_int;
    (*opt_310).dflt = (*opt_310).val;
    (*opt_310).min = 0 as libc::c_int;
    (*opt_310).max = 1 as libc::c_int;
    (*opt_310)
        .descrp = b"always synchronize all unconsumed clauses\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_310, b"synclsall\0" as *const u8 as *const libc::c_char);
    let mut opt_311: *mut Opt = &mut (*opts).synclsglue;
    (*opt_311).lng = b"synclsglue\0" as *const u8 as *const libc::c_char;
    (*opt_311).val = 8 as libc::c_int;
    (*opt_311).dflt = (*opt_311).val;
    (*opt_311).min = 0 as libc::c_int;
    (*opt_311).max = I;
    (*opt_311)
        .descrp = b"clause synchronization glue limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_311, b"synclsglue\0" as *const u8 as *const libc::c_char);
    let mut opt_312: *mut Opt = &mut (*opts).synclsint;
    (*opt_312).lng = b"synclsint\0" as *const u8 as *const libc::c_char;
    (*opt_312).val = 100 as libc::c_int;
    (*opt_312).dflt = (*opt_312).val;
    (*opt_312).min = 0 as libc::c_int;
    (*opt_312).max = 1000 as libc::c_int;
    (*opt_312)
        .descrp = b"clause synchronization confs interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_312, b"synclsint\0" as *const u8 as *const libc::c_char);
    let mut opt_313: *mut Opt = &mut (*opts).synclslen;
    (*opt_313).lng = b"synclslen\0" as *const u8 as *const libc::c_char;
    (*opt_313).val = 40 as libc::c_int;
    (*opt_313).dflt = (*opt_313).val;
    (*opt_313).min = 0 as libc::c_int;
    (*opt_313).max = I;
    (*opt_313)
        .descrp = b"clause synchronization length limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_313, b"synclslen\0" as *const u8 as *const libc::c_char);
    let mut opt_314: *mut Opt = &mut (*opts).syncunint;
    (*opt_314).lng = b"syncunint\0" as *const u8 as *const libc::c_char;
    (*opt_314).val = 111111 as libc::c_int;
    (*opt_314).dflt = (*opt_314).val;
    (*opt_314).min = 0 as libc::c_int;
    (*opt_314).max = M;
    (*opt_314)
        .descrp = b"unit synchronization steps interval\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_314, b"syncunint\0" as *const u8 as *const libc::c_char);
    let mut opt_315: *mut Opt = &mut (*opts).termint;
    (*opt_315).lng = b"termint\0" as *const u8 as *const libc::c_char;
    (*opt_315).val = 122222 as libc::c_int;
    (*opt_315).dflt = (*opt_315).val;
    (*opt_315).min = 0 as libc::c_int;
    (*opt_315).max = M;
    (*opt_315)
        .descrp = b"termination check interval\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_315, b"termint\0" as *const u8 as *const libc::c_char);
    let mut opt_316: *mut Opt = &mut (*opts).ternres;
    (*opt_316).lng = b"ternres\0" as *const u8 as *const libc::c_char;
    (*opt_316).val = 1 as libc::c_int;
    (*opt_316).dflt = (*opt_316).val;
    (*opt_316).min = 0 as libc::c_int;
    (*opt_316).max = 1 as libc::c_int;
    (*opt_316)
        .descrp = b"generate ternary resolvents\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_316, b"ternres\0" as *const u8 as *const libc::c_char);
    let mut opt_317: *mut Opt = &mut (*opts).ternresboost;
    (*opt_317).lng = b"ternresboost\0" as *const u8 as *const libc::c_char;
    (*opt_317).val = 5 as libc::c_int;
    (*opt_317).dflt = (*opt_317).val;
    (*opt_317).min = 1 as libc::c_int;
    (*opt_317).max = 100 as libc::c_int;
    (*opt_317)
        .descrp = b"initial ternary resolution boost\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_317, b"ternresboost\0" as *const u8 as *const libc::c_char);
    let mut opt_318: *mut Opt = &mut (*opts).ternresrtc;
    (*opt_318).lng = b"ternresrtc\0" as *const u8 as *const libc::c_char;
    (*opt_318).val = 0 as libc::c_int;
    (*opt_318).dflt = (*opt_318).val;
    (*opt_318).min = 0 as libc::c_int;
    (*opt_318).max = 1 as libc::c_int;
    (*opt_318)
        .descrp = b"run ternary resolvents until completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_318, b"ternresrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_319: *mut Opt = &mut (*opts).ternreswait;
    (*opt_319).lng = b"ternreswait\0" as *const u8 as *const libc::c_char;
    (*opt_319).val = 2 as libc::c_int;
    (*opt_319).dflt = (*opt_319).val;
    (*opt_319).min = 0 as libc::c_int;
    (*opt_319).max = 2 as libc::c_int;
    (*opt_319)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_319, b"ternreswait\0" as *const u8 as *const libc::c_char);
    let mut opt_320: *mut Opt = &mut (*opts).tlevelema;
    (*opt_320).lng = b"tlevelema\0" as *const u8 as *const libc::c_char;
    (*opt_320).val = 12 as libc::c_int;
    (*opt_320).dflt = (*opt_320).val;
    (*opt_320).min = 0 as libc::c_int;
    (*opt_320).max = 32 as libc::c_int;
    (*opt_320)
        .descrp = b"e for EMA with alpha=2^-e\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_320, b"tlevelema\0" as *const u8 as *const libc::c_char);
    let mut opt_321: *mut Opt = &mut (*opts).transred;
    (*opt_321).lng = b"transred\0" as *const u8 as *const libc::c_char;
    (*opt_321).val = 1 as libc::c_int;
    (*opt_321).dflt = (*opt_321).val;
    (*opt_321).min = 0 as libc::c_int;
    (*opt_321).max = 1 as libc::c_int;
    (*opt_321)
        .descrp = b"enable transitive reduction\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_321, b"transred\0" as *const u8 as *const libc::c_char);
    let mut opt_322: *mut Opt = &mut (*opts).transredwait;
    (*opt_322).lng = b"transredwait\0" as *const u8 as *const libc::c_char;
    (*opt_322).val = 2 as libc::c_int;
    (*opt_322).dflt = (*opt_322).val;
    (*opt_322).min = 0 as libc::c_int;
    (*opt_322).max = 2 as libc::c_int;
    (*opt_322)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_322, b"transredwait\0" as *const u8 as *const libc::c_char);
    let mut opt_323: *mut Opt = &mut (*opts).trapiflush;
    (*opt_323).lng = b"trapiflush\0" as *const u8 as *const libc::c_char;
    (*opt_323).val = 0 as libc::c_int;
    (*opt_323).dflt = (*opt_323).val;
    (*opt_323).min = 0 as libc::c_int;
    (*opt_323).max = 1 as libc::c_int;
    (*opt_323)
        .descrp = b"flush API trace after each call\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_323, b"trapiflush\0" as *const u8 as *const libc::c_char);
    let mut opt_324: *mut Opt = &mut (*opts).trdmaxeff;
    (*opt_324).lng = b"trdmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_324).val = 2 as libc::c_int * M;
    (*opt_324).dflt = (*opt_324).val;
    (*opt_324).min = -(1 as libc::c_int);
    (*opt_324).max = I;
    (*opt_324)
        .descrp = b"max effort in transitive reduction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_324, b"trdmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_325: *mut Opt = &mut (*opts).trdmineff;
    (*opt_325).lng = b"trdmineff\0" as *const u8 as *const libc::c_char;
    (*opt_325).val = 100 as libc::c_int * K;
    (*opt_325).dflt = (*opt_325).val;
    (*opt_325).min = 0 as libc::c_int;
    (*opt_325).max = I;
    (*opt_325)
        .descrp = b"min effort in transitive reduction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_325, b"trdmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_326: *mut Opt = &mut (*opts).trdreleff;
    (*opt_326).lng = b"trdreleff\0" as *const u8 as *const libc::c_char;
    (*opt_326).val = 10 as libc::c_int;
    (*opt_326).dflt = (*opt_326).val;
    (*opt_326).min = 0 as libc::c_int;
    (*opt_326).max = 10 as libc::c_int * K;
    (*opt_326)
        .descrp = b"rel effort in transitive reduction\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_326, b"trdreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_327: *mut Opt = &mut (*opts).treelook;
    (*opt_327).lng = b"treelook\0" as *const u8 as *const libc::c_char;
    (*opt_327).val = 1 as libc::c_int;
    (*opt_327).dflt = (*opt_327).val;
    (*opt_327).min = 0 as libc::c_int;
    (*opt_327).max = 2 as libc::c_int;
    (*opt_327)
        .descrp = b"enable tree-based look-ahead (2=scheduleprobing)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_327, b"treelook\0" as *const u8 as *const libc::c_char);
    let mut opt_328: *mut Opt = &mut (*opts).treelookboost;
    (*opt_328).lng = b"treelookboost\0" as *const u8 as *const libc::c_char;
    (*opt_328).val = 10 as libc::c_int;
    (*opt_328).dflt = (*opt_328).val;
    (*opt_328).min = 1 as libc::c_int;
    (*opt_328).max = 100000 as libc::c_int;
    (*opt_328)
        .descrp = b"tree-based look-head boost factor\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_328, b"treelookboost\0" as *const u8 as *const libc::c_char);
    let mut opt_329: *mut Opt = &mut (*opts).treelookfull;
    (*opt_329).lng = b"treelookfull\0" as *const u8 as *const libc::c_char;
    (*opt_329).val = 0 as libc::c_int;
    (*opt_329).dflt = (*opt_329).val;
    (*opt_329).min = 0 as libc::c_int;
    (*opt_329).max = 1 as libc::c_int;
    (*opt_329)
        .descrp = b"do not limit tree-based look-head\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_329, b"treelookfull\0" as *const u8 as *const libc::c_char);
    let mut opt_330: *mut Opt = &mut (*opts).treelookmaxeff;
    (*opt_330).lng = b"treelookmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_330).val = 50 as libc::c_int * M;
    (*opt_330).dflt = (*opt_330).val;
    (*opt_330).min = -(1 as libc::c_int);
    (*opt_330).max = I;
    (*opt_330)
        .descrp = b"max effort in tree-look based probing\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_330, b"treelookmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_331: *mut Opt = &mut (*opts).treelookmineff;
    (*opt_331).lng = b"treelookmineff\0" as *const u8 as *const libc::c_char;
    (*opt_331).val = 300 as libc::c_int * K;
    (*opt_331).dflt = (*opt_331).val;
    (*opt_331).min = 0 as libc::c_int;
    (*opt_331).max = I;
    (*opt_331)
        .descrp = b"min effort in tree-look based probing\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_331, b"treelookmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_332: *mut Opt = &mut (*opts).treelookreleff;
    (*opt_332).lng = b"treelookreleff\0" as *const u8 as *const libc::c_char;
    (*opt_332).val = 2 as libc::c_int;
    (*opt_332).dflt = (*opt_332).val;
    (*opt_332).min = 0 as libc::c_int;
    (*opt_332).max = 10 as libc::c_int * K;
    (*opt_332)
        .descrp = b"rel effort in tree-look based probing\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_332, b"treelookreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_333: *mut Opt = &mut (*opts).treelookrtc;
    (*opt_333).lng = b"treelookrtc\0" as *const u8 as *const libc::c_char;
    (*opt_333).val = 0 as libc::c_int;
    (*opt_333).dflt = (*opt_333).val;
    (*opt_333).min = 0 as libc::c_int;
    (*opt_333).max = 1 as libc::c_int;
    (*opt_333)
        .descrp = b"run tree-based look-ahead until completion\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_333, b"treelookrtc\0" as *const u8 as *const libc::c_char);
    let mut opt_334: *mut Opt = &mut (*opts).trep;
    (*opt_334).lng = b"trep\0" as *const u8 as *const libc::c_char;
    (*opt_334).val = 0 as libc::c_int;
    (*opt_334).dflt = (*opt_334).val;
    (*opt_334).min = 0 as libc::c_int;
    (*opt_334).max = 1 as libc::c_int;
    (*opt_334)
        .descrp = b"enable time based interval reporting\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_334, b"trep\0" as *const u8 as *const libc::c_char);
    let mut opt_335: *mut Opt = &mut (*opts).trepint;
    (*opt_335).lng = b"trepint\0" as *const u8 as *const libc::c_char;
    (*opt_335).val = 55555 as libc::c_int;
    (*opt_335).dflt = (*opt_335).val;
    (*opt_335).min = 1 as libc::c_int;
    (*opt_335).max = I;
    (*opt_335)
        .descrp = b"interval for time based reporting\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_335, b"trepint\0" as *const u8 as *const libc::c_char);
    let mut opt_336: *mut Opt = &mut (*opts).trnreleff;
    (*opt_336).lng = b"trnreleff\0" as *const u8 as *const libc::c_char;
    (*opt_336).val = 10 as libc::c_int;
    (*opt_336).dflt = (*opt_336).val;
    (*opt_336).min = 0 as libc::c_int;
    (*opt_336).max = K;
    (*opt_336)
        .descrp = b"rel effort in ternary resolutions\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_336, b"trnreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_337: *mut Opt = &mut (*opts).trnrmaxeff;
    (*opt_337).lng = b"trnrmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_337).val = 200 as libc::c_int * M;
    (*opt_337).dflt = (*opt_337).val;
    (*opt_337).min = -(1 as libc::c_int);
    (*opt_337).max = I;
    (*opt_337)
        .descrp = b"max effort in ternary resolutions\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_337, b"trnrmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_338: *mut Opt = &mut (*opts).trnrmineff;
    (*opt_338).lng = b"trnrmineff\0" as *const u8 as *const libc::c_char;
    (*opt_338).val = 4 as libc::c_int * M;
    (*opt_338).dflt = (*opt_338).val;
    (*opt_338).min = 0 as libc::c_int;
    (*opt_338).max = I;
    (*opt_338)
        .descrp = b"min effort in ternary resolutions\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_338, b"trnrmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_339: *mut Opt = &mut (*opts).unhdatrn;
    (*opt_339).lng = b"unhdatrn\0" as *const u8 as *const libc::c_char;
    (*opt_339).val = 2 as libc::c_int;
    (*opt_339).dflt = (*opt_339).val;
    (*opt_339).min = 0 as libc::c_int;
    (*opt_339).max = 2 as libc::c_int;
    (*opt_339)
        .descrp = b"unhide redundant ternary clauses (1=move,2=force)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_339, b"unhdatrn\0" as *const u8 as *const libc::c_char);
    let mut opt_340: *mut Opt = &mut (*opts).unhdextstamp;
    (*opt_340).lng = b"unhdextstamp\0" as *const u8 as *const libc::c_char;
    (*opt_340).val = 1 as libc::c_int;
    (*opt_340).dflt = (*opt_340).val;
    (*opt_340).min = 0 as libc::c_int;
    (*opt_340).max = 1 as libc::c_int;
    (*opt_340)
        .descrp = b"used extended stamping features\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_340, b"unhdextstamp\0" as *const u8 as *const libc::c_char);
    let mut opt_341: *mut Opt = &mut (*opts).unhdhbr;
    (*opt_341).lng = b"unhdhbr\0" as *const u8 as *const libc::c_char;
    (*opt_341).val = 0 as libc::c_int;
    (*opt_341).dflt = (*opt_341).val;
    (*opt_341).min = 0 as libc::c_int;
    (*opt_341).max = 1 as libc::c_int;
    (*opt_341)
        .descrp = b"enable unhiding hyper binary resolution\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_341, b"unhdhbr\0" as *const u8 as *const libc::c_char);
    let mut opt_342: *mut Opt = &mut (*opts).unhdlnpr;
    (*opt_342).lng = b"unhdlnpr\0" as *const u8 as *const libc::c_char;
    (*opt_342).val = 3 as libc::c_int;
    (*opt_342).dflt = (*opt_342).val;
    (*opt_342).min = 0 as libc::c_int;
    (*opt_342).max = I;
    (*opt_342)
        .descrp = b"unhide no progress round limit\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_342, b"unhdlnpr\0" as *const u8 as *const libc::c_char);
    let mut opt_343: *mut Opt = &mut (*opts).unhdmaxeff;
    (*opt_343).lng = b"unhdmaxeff\0" as *const u8 as *const libc::c_char;
    (*opt_343).val = 20 as libc::c_int * M;
    (*opt_343).dflt = (*opt_343).val;
    (*opt_343).min = -(1 as libc::c_int);
    (*opt_343).max = I;
    (*opt_343).descrp = b"max effort in unhiding\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_343, b"unhdmaxeff\0" as *const u8 as *const libc::c_char);
    let mut opt_344: *mut Opt = &mut (*opts).unhdmineff;
    (*opt_344).lng = b"unhdmineff\0" as *const u8 as *const libc::c_char;
    (*opt_344).val = 100 as libc::c_int * K;
    (*opt_344).dflt = (*opt_344).val;
    (*opt_344).min = 0 as libc::c_int;
    (*opt_344).max = I;
    (*opt_344).descrp = b"min effort in unhiding\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_344, b"unhdmineff\0" as *const u8 as *const libc::c_char);
    let mut opt_345: *mut Opt = &mut (*opts).unhdreleff;
    (*opt_345).lng = b"unhdreleff\0" as *const u8 as *const libc::c_char;
    (*opt_345).val = 2 as libc::c_int;
    (*opt_345).dflt = (*opt_345).val;
    (*opt_345).min = 0 as libc::c_int;
    (*opt_345).max = 10 as libc::c_int * K;
    (*opt_345).descrp = b"rel effort in unhiding\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_345, b"unhdreleff\0" as *const u8 as *const libc::c_char);
    let mut opt_346: *mut Opt = &mut (*opts).unhdroundlim;
    (*opt_346).lng = b"unhdroundlim\0" as *const u8 as *const libc::c_char;
    (*opt_346).val = 20 as libc::c_int;
    (*opt_346).dflt = (*opt_346).val;
    (*opt_346).min = 0 as libc::c_int;
    (*opt_346).max = 100 as libc::c_int;
    (*opt_346).descrp = b"unhide round limit\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_346, b"unhdroundlim\0" as *const u8 as *const libc::c_char);
    let mut opt_347: *mut Opt = &mut (*opts).unhide;
    (*opt_347).lng = b"unhide\0" as *const u8 as *const libc::c_char;
    (*opt_347).val = 1 as libc::c_int;
    (*opt_347).dflt = (*opt_347).val;
    (*opt_347).min = 0 as libc::c_int;
    (*opt_347).max = 1 as libc::c_int;
    (*opt_347).descrp = b"enable unhiding\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_347, b"unhide\0" as *const u8 as *const libc::c_char);
    let mut opt_348: *mut Opt = &mut (*opts).unhidewait;
    (*opt_348).lng = b"unhidewait\0" as *const u8 as *const libc::c_char;
    (*opt_348).val = 0 as libc::c_int;
    (*opt_348).dflt = (*opt_348).val;
    (*opt_348).min = 0 as libc::c_int;
    (*opt_348).max = 2 as libc::c_int;
    (*opt_348)
        .descrp = b"wait for BCE (1) and/or BVE (2)\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_348, b"unhidewait\0" as *const u8 as *const libc::c_char);
    let mut opt_349: *mut Opt = &mut (*opts).usedtwice;
    (*opt_349).lng = b"usedtwice\0" as *const u8 as *const libc::c_char;
    (*opt_349).val = 1 as libc::c_int;
    (*opt_349).dflt = (*opt_349).val;
    (*opt_349).min = 0 as libc::c_int;
    (*opt_349).max = 1 as libc::c_int;
    (*opt_349)
        .descrp = b"used twice optimization for clause minimization\0" as *const u8
        as *const libc::c_char;
    lglgetenv(lgl, opt_349, b"usedtwice\0" as *const u8 as *const libc::c_char);
    let mut opt_350: *mut Opt = &mut (*opts).verbose;
    (*opt_350).lng = b"verbose\0" as *const u8 as *const libc::c_char;
    (*opt_350).val = 0 as libc::c_int;
    (*opt_350).dflt = (*opt_350).val;
    (*opt_350).min = -(1 as libc::c_int);
    (*opt_350).max = 5 as libc::c_int;
    (*opt_350).descrp = b"verbosity level\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_350, b"verbose\0" as *const u8 as *const libc::c_char);
    let mut opt_351: *mut Opt = &mut (*opts).wait;
    (*opt_351).lng = b"wait\0" as *const u8 as *const libc::c_char;
    (*opt_351).val = 1 as libc::c_int;
    (*opt_351).dflt = (*opt_351).val;
    (*opt_351).min = 0 as libc::c_int;
    (*opt_351).max = 1 as libc::c_int;
    (*opt_351)
        .descrp = b"enable or disable all waiting\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_351, b"wait\0" as *const u8 as *const libc::c_char);
    let mut opt_352: *mut Opt = &mut (*opts).waitmax;
    (*opt_352).lng = b"waitmax\0" as *const u8 as *const libc::c_char;
    (*opt_352).val = 4 as libc::c_int;
    (*opt_352).dflt = (*opt_352).val;
    (*opt_352).min = -(1 as libc::c_int);
    (*opt_352).max = I;
    (*opt_352)
        .descrp = b"max simps to wait (-1=nomax)\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_352, b"waitmax\0" as *const u8 as *const libc::c_char);
    let mut opt_353: *mut Opt = &mut (*opts).witness;
    (*opt_353).lng = b"witness\0" as *const u8 as *const libc::c_char;
    (*opt_353).val = 1 as libc::c_int;
    (*opt_353).dflt = (*opt_353).val;
    (*opt_353).min = 0 as libc::c_int;
    (*opt_353).max = 1 as libc::c_int;
    (*opt_353).descrp = b"print witness\0" as *const u8 as *const libc::c_char;
    lglgetenv(lgl, opt_353, b"witness\0" as *const u8 as *const libc::c_char);
}
