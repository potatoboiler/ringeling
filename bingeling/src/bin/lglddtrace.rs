#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn setrlimit(__resource: __rlimit_resource_t, __rlimits: *const rlimit) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglfirstopt(_: *mut LGL) -> *mut libc::c_void;
    fn lglnextopt(
        _: *mut LGL,
        iterator: *mut libc::c_void,
        nameptr: *mut *const libc::c_char,
        valptr: *mut libc::c_int,
        minptr: *mut libc::c_int,
        maxptr: *mut libc::c_int,
    ) -> *mut libc::c_void;
    fn lglsetphase(_: *mut LGL, lit_0: libc::c_int);
    fn lglresetphase(_: *mut LGL, lit_0: libc::c_int);
    fn lglsetimportant(_: *mut LGL, lit_0: libc::c_int);
    fn lglsetphases(_: *mut LGL);
    fn lglonabort(
        _: *mut LGL,
        state: *mut libc::c_void,
        callback: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn lglincvar(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit_0: libc::c_int);
    fn lglassume(_: *mut LGL, lit_0: libc::c_int);
    fn lglfixate(_: *mut LGL);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfixed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfailed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglchanged(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglflushcache(_: *mut LGL);
    fn lglrepr(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfreeze(_: *mut LGL, lit_0: libc::c_int);
    fn lglfrozen(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglmelt(_: *mut LGL, lit_0: libc::c_int);
    fn lglusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglreusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglreuse(_: *mut LGL, lit_0: libc::c_int);
    fn lglookahead(_: *mut LGL) -> libc::c_int;
    fn lglchkclone(_: *mut LGL);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag<'h0,'h1> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h0 (libc::c_void),
    // 109: overflow_arg_area: typeof(overflow_arg_area) = *mut {g138} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 109: overflow_arg_area:   g138 = UNIQUE | NON_NULL, (empty)
    pub reg_save_area: &'h1 (libc::c_void),
    // 110: reg_save_area: typeof(reg_save_area) = *mut {g139} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 110: reg_save_area:   g139 = UNIQUE | NON_NULL, (empty)
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __rlim_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type va_list = __builtin_va_list;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 137: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g140} i8
    // 137: _IO_read_ptr:   g140 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 138: _IO_read_end: typeof(_IO_read_end) = *mut {g141} i8
    // 138: _IO_read_end:   g141 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 139: _IO_read_base: typeof(_IO_read_base) = *mut {g142} i8
    // 139: _IO_read_base:   g142 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 140: _IO_write_base: typeof(_IO_write_base) = *mut {g143} i8
    // 140: _IO_write_base:   g143 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 141: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g144} i8
    // 141: _IO_write_ptr:   g144 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 142: _IO_write_end: typeof(_IO_write_end) = *mut {g145} i8
    // 142: _IO_write_end:   g145 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 143: _IO_buf_base: typeof(_IO_buf_base) = *mut {g146} i8
    // 143: _IO_buf_base:   g146 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 144: _IO_buf_end: typeof(_IO_buf_end) = *mut {g147} i8
    // 144: _IO_buf_end:   g147 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 145: _IO_save_base: typeof(_IO_save_base) = *mut {g148} i8
    // 145: _IO_save_base:   g148 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 146: _IO_backup_base: typeof(_IO_backup_base) = *mut {g149} i8
    // 146: _IO_backup_base:   g149 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 147: _IO_save_end: typeof(_IO_save_end) = *mut {g150} i8
    // 147: _IO_save_end:   g150 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 148: _markers: typeof(_markers) = *mut {g151} _IO_marker
    // 148: _markers:   g151 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 149: _chain: typeof(_chain) = *mut {g152} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 149: _chain:   g152 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 156: _lock: typeof(_lock) = *mut {g153} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 156: _lock:   g153 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 158: _codecvt: typeof(_codecvt) = *mut {g154} _IO_codecvt
    // 158: _codecvt:   g154 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 159: _wide_data: typeof(_wide_data) = *mut {g155} _IO_wide_data
    // 159: _wide_data:   g155 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 160: _freeres_list: typeof(_freeres_list) = *mut {g156} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 160: _freeres_list:   g156 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 161: _freeres_buf: typeof(_freeres_buf) = *mut {g157} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 161: _freeres_buf:   g157 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
pub type rlim_t = __rlim_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = libc::c_int;
pub type Type = libc::c_uint;
pub const LKHD: Type = 31;
pub const INCONSISTENT: Type = 30;
pub const CHANGED: Type = 29;
pub const CHKCLONE: Type = 28;
pub const FIXATE: Type = 27;
pub const FIXED: Type = 26;
pub const INCVAR: Type = 25;
pub const MAXVAR: Type = 24;
pub const REUSABLE: Type = 23;
pub const USABLE: Type = 22;
pub const FROZEN: Type = 21;
pub const REDUCE: Type = 20;
pub const FLUSH: Type = 19;
pub const SETPHASES: Type = 18;
pub const RESETPHASE: Type = 17;
pub const SETPHASE: Type = 16;
pub const SETIMPORTANT: Type = 15;
pub const REPR: Type = 14;
pub const SIMP: Type = 13;
pub const SAT: Type = 12;
pub const RETURN: Type = 11;
pub const RELEASE: Type = 10;
pub const PHASE: Type = 9;
pub const OPTION: Type = 8;
pub const REUSE: Type = 7;
pub const MELT: Type = 6;
pub const INIT: Type = 5;
pub const FREEZE: Type = 4;
pub const FAILED: Type = 3;
pub const DEREF: Type = 2;
pub const ASSUME: Type = 1;
pub const ADD: Type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event<'h2> {
    pub type_0: Type,
    pub removed: libc::c_int,
    pub arg: libc::c_int,
    pub opt: *mut libc::c_char,
    // 241: opt: typeof(opt) = *mut {g158} i8
    // 241: opt:   g158 = (empty), FIXED
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Opt<'h3> {
    pub name: *mut libc::c_char,
    // 246: name: typeof(name) = *mut {g159} i8
    // 246: name:   g159 = UNIQUE | NON_NULL, FIXED
    pub val: libc::c_int,
    pub min: libc::c_int,
    pub max: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub from: libc::c_int,
    pub to: libc::c_int,
    pub removed: libc::c_int,
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    // 260: mut __fmt: typeof(_1) = *const {g0} i8
    // 260: mut __fmt:   g0 = UNIQUE | NON_NULL, FIXED
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
    // 263: stdout: typeof(_4) = *mut {l4} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 263: stdout:   l4 = UNIQUE | NON_NULL, (empty)
    // 263: stdout: typeof(_5) = *mut {l6} *mut {l7} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 263: stdout:   l6 = UNIQUE | NON_NULL, (empty)
    // 263: stdout:   l7 = UNIQUE | NON_NULL, (empty)
    // 263: __fmt: typeof(_6) = *const {l9} i8
    // 263: __fmt:   l9 = UNIQUE | NON_NULL, (empty)
    // 263: __arg.as_va_list(): typeof(_8) = &mut {l12} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 263: __arg.as_va_list():   l12 = UNIQUE | NON_NULL, (empty)
    // 263: __arg.as_va_list(): typeof(_9) = &mut {l14} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 263: __arg.as_va_list():   l14 = UNIQUE | NON_NULL, FIXED
    // 263: __arg.as_va_list(): typeof(_10) = &mut {l16} DefId(2:46558 ~ core[480a]::ffi::VaList)
    // 263: __arg.as_va_list():   l16 = UNIQUE | NON_NULL, (empty)
    // 263: __arg.as_va_list(): typeof(_10 = &mut _2) = &mut {l18} DefId(2:46558 ~ core[480a]::ffi::VaList)
    // 263: __arg.as_va_list():   l18 = UNIQUE | NON_NULL, (empty)
    // 263: __arg.as_va_list(): typeof(_8 = &mut (*_9)) = &mut {l19} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 263: __arg.as_va_list():   l19 = UNIQUE | NON_NULL, (empty)
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
// 266: mut __nptr: typeof(_1) = *const {g1} i8
// 266: mut __nptr:   g1 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    return strtol(
        __nptr,
        // 268: __nptr: typeof(_4) = *const {l4} i8
        // 268: __nptr:   l4 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 269: 0 as *mut libc: ... _char: typeof(_5) = *mut {l6} *mut {g45} i8
        // 269: 0 as *mut libc: ... _char:   l6 = WRITE | UNIQUE, (empty)
        // 269: 0 as *mut libc: ... _char:   g45 = WRITE | OFFSET_ADD, CELL | FIXED
        // 269: 0 as *mut libc: ... _void: typeof(_6) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 269: 0 as *mut libc: ... _void:   l8 = WRITE | UNIQUE, (empty)
        // 269: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 269: 0 as *mut libc: ... _void:   l11 = WRITE | UNIQUE, (empty)
        // 269: 0 as *mut libc: ... _char: typeof(_5 = move _6 as *mut *mut i8 (Misc)) = *mut {l12} *mut {g45} i8
        // 269: 0 as *mut libc: ... _char:   l12 = WRITE | UNIQUE, (empty)
        // 269: 0 as *mut libc: ... _char:   g45 = WRITE | OFFSET_ADD, CELL | FIXED
        10 as libc::c_int,
    ) as libc::c_int;
}
static runs: libc::c_int = 0;
static golden: libc::c_int = 0;
static timelimit: libc::c_int = 0;
static lineno: libc::c_int = 0;
static verbose: libc::c_int = 0;
static checkreturn: libc::c_int = 0;
static ddopts: libc::c_int = 0;
static mut nmap: libc::c_int = -(1 as libc::c_int);
static mut szmap: libc::c_int = 0;
static mut map: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static prevents: libc::c_int = 0;
static iname: *const libc::c_char = 0 as *const libc::c_char;
static oname: *const libc::c_char = 0 as *const libc::c_char;
static mut events: *mut Event = 0 as *const Event as *mut Event;
static mut nevents: libc::c_int = 0;
static mut szevents: libc::c_int = 0;
static opts: *mut Opt = 0 as *const Opt as *mut Opt;
static nopts: libc::c_int = 0;
static szopts: libc::c_int = 0;
static sumoptvals: libc::c_longlong = 0;
unsafe extern "C" fn event(mut type_0: Type, mut arg: libc::c_int, mut opt: *const libc::c_char) {
// 293: mut opt: typeof(_3) = *const {g2} i8
// 293: mut opt:   g2 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    let mut e: *mut Event = 0 as *mut Event;
    // 294: mut e: typeof(_4) = *mut {l4} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 294: mut e:   l4 = READ | WRITE | UNIQUE, (empty)
    // 294: 0 as *mut Event: typeof(_4 = const 0_usize as *mut Event (PointerFromExposedAddress)) = *mut {l138} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 294: 0 as *mut Event:   l138 = READ | WRITE | UNIQUE, (empty)
    let mut idx: libc::c_int = 0;
    if nevents == szevents {
    // 296: nevents: typeof(_9) = *mut {l10} i32
    // 296: nevents:   l10 = READ | UNIQUE | NON_NULL, (empty)
    // 296: szevents: typeof(_11) = *mut {l13} i32
    // 296: szevents:   l13 = READ | UNIQUE | NON_NULL, (empty)
        szevents = if szevents != 0 {
        // 297: szevents: typeof(_15) = *mut {l18} i32
        // 297: szevents:   l18 = READ | UNIQUE | NON_NULL, (empty)
        // 297: szevents: typeof(_20) = *mut {l25} i32
        // 297: szevents:   l25 = READ | WRITE | UNIQUE | NON_NULL, (empty)
            2 as libc::c_int * szevents
            // 298: szevents: typeof(_18) = *mut {l22} i32
            // 298: szevents:   l22 = READ | UNIQUE | NON_NULL, (empty)
        } else {
            1 as libc::c_int
        };
        events = realloc(
        // 302: realloc( events ... g), ): typeof(_21) = *mut {l27} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 302: realloc( events ... g), ):   l27 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
        // 302: events: typeof(_31) = *mut {l42} *mut {g125} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 302: events:   l42 = READ | WRITE | UNIQUE | NON_NULL, (empty)
        // 302: events:   g125 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
        // 302: events = reallo ... Event: typeof((*_31) = move _21 as *mut Event (Misc)) = *mut {l140} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 302: events = reallo ... Event:   l140 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
            events as *mut libc::c_void,
            // 303: events as *mut  ... _void: typeof(_22) = *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 303: events as *mut  ... _void:   l29 = UNIQUE | FREE | NON_NULL, (empty)
            // 303: events: typeof(_23) = *mut {l31} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 303: events:   l31 = UNIQUE | FREE | NON_NULL, (empty)
            // 303: events: typeof(_24) = *mut {l33} *mut {g125} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 303: events:   l33 = READ | UNIQUE | NON_NULL, (empty)
            // 303: events:   g125 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
            // 303: events as *mut  ... _void: typeof(_22 = move _23 as *mut libc::c_void (Misc)) = *mut {l139} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 303: events as *mut  ... _void:   l139 = UNIQUE | FREE | NON_NULL, (empty)
            (szevents as libc::c_ulong)
            // 304: szevents: typeof(_28) = *mut {l38} i32
            // 304: szevents:   l38 = READ | UNIQUE | NON_NULL, (empty)
                .wrapping_mul(::core::mem::size_of::<Event>() as libc::c_ulong),
        ) as *mut Event;
    }
    let fresh0 = nevents;
    // 308: nevents: typeof(_33) = *mut {l45} i32
    // 308: nevents:   l45 = READ | UNIQUE | NON_NULL, (empty)
    nevents = nevents + 1;
    // 309: nevents: typeof(_35) = *mut {l48} i32
    // 309: nevents:   l48 = READ | UNIQUE | NON_NULL, (empty)
    // 309: nevents: typeof(_37) = *mut {l51} i32
    // 309: nevents:   l51 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    e = events.offset(fresh0 as isize);
    // 310: events.offset(f ... size): typeof(_38) = *mut {l53} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 310: events.offset(f ... size):   l53 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    // 310: events: typeof(_39) = *mut {l55} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 310: events:   l55 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
    // 310: events: typeof(_40) = *mut {l57} *mut {g125} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 310: events:   l57 = READ | UNIQUE | NON_NULL, (empty)
    // 310: events:   g125 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
    (*e).type_0 = type_0;
    (*e).arg = arg;
    (*e).opt = if !opt.is_null() {
    // 313: if !opt.is_null ... har }: typeof(_45) = *mut {l63} i8
    // 313: if !opt.is_null ... har }:   l63 = (empty), (empty)
    // 313: opt: typeof(_48) = *const {l67} i8
    // 313: opt:   l67 = UNIQUE | NON_NULL, (empty)
        strdup(opt)
        // 314: opt: typeof(_49) = *const {l69} i8
        // 314: opt:   l69 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    } else {
        0 as *mut libc::c_char
        // 316: 0 as *mut libc: ... _char: typeof(_45 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l141} i8
        // 316: 0 as *mut libc: ... _char:   l141 = UNIQUE, (empty)
    };
    (*e).removed = 2147483647 as libc::c_int;
    match type_0 as libc::c_uint {
        0 | 1 | 2 | 3 | 4 | 15 | 16 | 17 | 6 | 7 | 9 | 26 | 21 | 23 | 22 | 14 => {
            idx = abs((*e).arg);
            if idx > nmap {
            // 322: nmap: typeof(_59) = *mut {l80} i32
            // 322: nmap:   l80 = READ | UNIQUE | NON_NULL, (empty)
                if idx >= szmap {
                // 323: szmap: typeof(_64) = *mut {l86} i32
                // 323: szmap:   l86 = READ | UNIQUE | NON_NULL, (empty)
                    loop {
                        szmap = if szmap != 0 {
                        // 325: szmap: typeof(_70) = *mut {l93} i32
                        // 325: szmap:   l93 = READ | UNIQUE | NON_NULL, (empty)
                        // 325: szmap: typeof(_75) = *mut {l100} i32
                        // 325: szmap:   l100 = READ | WRITE | UNIQUE | NON_NULL, (empty)
                            2 as libc::c_int * szmap
                            // 326: szmap: typeof(_73) = *mut {l97} i32
                            // 326: szmap:   l97 = READ | UNIQUE | NON_NULL, (empty)
                        } else {
                            2 as libc::c_int
                        };
                        if !(szmap <= idx) {
                        // 330: szmap: typeof(_79) = *mut {l105} i32
                        // 330: szmap:   l105 = READ | UNIQUE | NON_NULL, (empty)
                            break;
                        }
                    }
                    map = realloc(
                    // 334: realloc( map as ... g), ): typeof(_82) = *mut {l109} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 334: realloc( map as ... g), ):   l109 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
                    // 334: map: typeof(_92) = *mut {l124} *mut {g118} i32
                    // 334: map:   l124 = READ | WRITE | UNIQUE | NON_NULL, (empty)
                    // 334: map:   g118 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
                    // 334: map = realloc(  ... c_int: typeof((*_92) = move _82 as *mut i32 (Misc)) = *mut {l143} i32
                    // 334: map = realloc(  ... c_int:   l143 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
                        map as *mut libc::c_void,
                        // 335: map as *mut lib ... _void: typeof(_83) = *mut {l111} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 335: map as *mut lib ... _void:   l111 = UNIQUE | FREE | NON_NULL, (empty)
                        // 335: map: typeof(_84) = *mut {l113} i32
                        // 335: map:   l113 = UNIQUE | FREE | NON_NULL, (empty)
                        // 335: map: typeof(_85) = *mut {l115} *mut {g118} i32
                        // 335: map:   l115 = READ | UNIQUE | NON_NULL, (empty)
                        // 335: map:   g118 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
                        // 335: map as *mut lib ... _void: typeof(_83 = move _84 as *mut libc::c_void (Misc)) = *mut {l142} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 335: map as *mut lib ... _void:   l142 = UNIQUE | FREE | NON_NULL, (empty)
                        (szmap as libc::c_ulong)
                        // 336: szmap: typeof(_89) = *mut {l120} i32
                        // 336: szmap:   l120 = READ | UNIQUE | NON_NULL, (empty)
                            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                    ) as *mut libc::c_int;
                }
                nmap = idx;
                // 340: nmap: typeof(_94) = *mut {l127} i32
                // 340: nmap:   l127 = READ | WRITE | UNIQUE | NON_NULL, (empty)
            }
            *map.offset(idx as isize) = idx;
            // 342: map.offset(idx  ... size): typeof(_96) = *mut {l130} i32
            // 342: map.offset(idx  ... size):   l130 = READ | WRITE | UNIQUE | NON_NULL, (empty)
            // 342: map: typeof(_97) = *mut {l132} i32
            // 342: map:   l132 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
            // 342: map: typeof(_98) = *mut {l134} *mut {g118} i32
            // 342: map:   l134 = READ | UNIQUE | NON_NULL, (empty)
            // 342: map:   g118 = READ | WRITE | UNIQUE | OFFSET_ADD | OFFSET_SUB | FREE | NON_NULL, (empty)
        }
        _ => {}
    };
}
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
// 347: mut fmt: typeof(_1) = *const {g3} i8
// 347: mut fmt:   g3 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** lglddtrace: \0" as *const u8 as *const libc::c_char,
        // 350: b"*** lglddtrac ... _char: typeof(_6) = *const {l6} i8
        // 350: b"*** lglddtrac ... _char:   l6 = UNIQUE | NON_NULL, (empty)
        // 350: b"*** lglddtrac ... st u8: typeof(_7) = *const {l8} u8
        // 350: b"*** lglddtrac ... st u8:   l8 = UNIQUE | NON_NULL, (empty)
        // 350: b"*** lglddtrac ... : \0": typeof(_8) = *const {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 350: b"*** lglddtrac ... : \0":   l10 = UNIQUE | NON_NULL, (empty)
        // 350: b"*** lglddtrac ... : \0": typeof(_9) = & {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 350: b"*** lglddtrac ... : \0":   l12 = UNIQUE | NON_NULL, FIXED
        // 350: b"*** lglddtrac ... : \0": typeof(_8 = &raw const (*_9)) = *const {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 350: b"*** lglddtrac ... : \0":   l49 = UNIQUE | NON_NULL, (empty)
        // 350: b"*** lglddtrac ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l51} i8
        // 350: b"*** lglddtrac ... _char:   l51 = UNIQUE | NON_NULL, (empty)
        // 350: b"*** lglddtrac ... : \0": typeof(_9 = const b"*** lglddtrace: \x00") = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 350: b"*** lglddtrac ... : \0":   l48 = UNIQUE | NON_NULL, (empty)
        // 350: b"*** lglddtrac ... st u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l50} u8
        // 350: b"*** lglddtrac ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 351: stderr: typeof(_10) = *mut {l14} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 351: stderr:   l14 = UNIQUE | NON_NULL, (empty)
        // 351: stderr: typeof(_11) = *mut {l16} *mut {l17} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 351: stderr:   l16 = UNIQUE | NON_NULL, (empty)
        // 351: stderr:   l17 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 353: args.clone(): typeof(_13) = & {l20} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 353: args.clone():   l20 = UNIQUE | NON_NULL, (empty)
    // 353: args.clone(): typeof(_13 = &_2) = & {l52} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 353: args.clone():   l52 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 354: stderr: typeof(_15) = *mut {l23} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 354: stderr:   l23 = UNIQUE | NON_NULL, (empty)
    // 354: stderr: typeof(_16) = *mut {l25} *mut {l26} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 354: stderr:   l25 = UNIQUE | NON_NULL, (empty)
    // 354: stderr:   l26 = UNIQUE | NON_NULL, (empty)
    // 354: fmt: typeof(_17) = *const {l28} i8
    // 354: fmt:   l28 = UNIQUE | NON_NULL, (empty)
    // 354: ap.as_va_list(): typeof(_19) = &mut {l31} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 354: ap.as_va_list():   l31 = UNIQUE | NON_NULL, (empty)
    // 354: ap.as_va_list(): typeof(_19 = &mut _4) = &mut {l53} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 354: ap.as_va_list():   l53 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 355: stderr: typeof(_22) = *mut {l35} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 355: stderr:   l35 = UNIQUE | NON_NULL, (empty)
    // 355: stderr: typeof(_23) = *mut {l37} *mut {l38} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 355: stderr:   l37 = UNIQUE | NON_NULL, (empty)
    // 355: stderr:   l38 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 356: stderr: typeof(_25) = *mut {l41} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 356: stderr:   l41 = UNIQUE | NON_NULL, (empty)
    // 356: stderr: typeof(_26) = *mut {l43} *mut {l44} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 356: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 356: stderr:   l44 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn perr(mut fmt: *const libc::c_char, mut args: ...) {
// 359: mut fmt: typeof(_1) = *const {g4} i8
// 359: mut fmt:   g4 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(
        stderr,
        // 362: stderr: typeof(_6) = *mut {l6} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 362: stderr:   l6 = UNIQUE | NON_NULL, (empty)
        // 362: stderr: typeof(_7) = *mut {l8} *mut {l9} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 362: stderr:   l8 = UNIQUE | NON_NULL, (empty)
        // 362: stderr:   l9 = UNIQUE | NON_NULL, (empty)
        b"*** lglddtrace: parse error in '%s' line %d: \0" as *const u8 as *const libc::c_char,
        // 363: b"*** lglddtrac ... _char: typeof(_8) = *const {l11} i8
        // 363: b"*** lglddtrac ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 363: b"*** lglddtrac ... st u8: typeof(_9) = *const {l13} u8
        // 363: b"*** lglddtrac ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 363: b"*** lglddtrac ... : \0": typeof(_10) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 363: b"*** lglddtrac ... : \0":   l15 = UNIQUE | NON_NULL, (empty)
        // 363: b"*** lglddtrac ... : \0": typeof(_11) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 363: b"*** lglddtrac ... : \0":   l17 = UNIQUE | NON_NULL, FIXED
        // 363: b"*** lglddtrac ... : \0": typeof(_10 = &raw const (*_11)) = *const {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 363: b"*** lglddtrac ... : \0":   l57 = UNIQUE | NON_NULL, (empty)
        // 363: b"*** lglddtrac ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l58} u8
        // 363: b"*** lglddtrac ... st u8:   l58 = UNIQUE | NON_NULL, (empty)
        // 363: b"*** lglddtrac ... : \0": typeof(_11 = const b"*** lglddtrace: parse error in \'%s\' line %d: \x00") = & {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
        // 363: b"*** lglddtrac ... : \0":   l56 = UNIQUE | NON_NULL, (empty)
        // 363: b"*** lglddtrac ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l59} i8
        // 363: b"*** lglddtrac ... _char:   l59 = UNIQUE | NON_NULL, (empty)
        iname,
        // 364: iname: typeof(_12) = *const {l19} i8
        // 364: iname:   l19 = UNIQUE | NON_NULL, (empty)
        // 364: iname: typeof(_13) = *mut {l21} *const {l22} i8
        // 364: iname:   l21 = UNIQUE | NON_NULL, (empty)
        // 364: iname:   l22 = UNIQUE | NON_NULL, (empty)
        lineno,
        // 365: lineno: typeof(_15) = *mut {l25} i32
        // 365: lineno:   l25 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 367: args.clone(): typeof(_17) = & {l28} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 367: args.clone():   l28 = UNIQUE | NON_NULL, (empty)
    // 367: args.clone(): typeof(_17 = &_2) = & {l60} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 367: args.clone():   l60 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 368: stderr: typeof(_19) = *mut {l31} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 368: stderr:   l31 = UNIQUE | NON_NULL, (empty)
    // 368: stderr: typeof(_20) = *mut {l33} *mut {l34} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 368: stderr:   l33 = UNIQUE | NON_NULL, (empty)
    // 368: stderr:   l34 = UNIQUE | NON_NULL, (empty)
    // 368: fmt: typeof(_21) = *const {l36} i8
    // 368: fmt:   l36 = UNIQUE | NON_NULL, (empty)
    // 368: ap.as_va_list(): typeof(_23) = &mut {l39} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 368: ap.as_va_list():   l39 = UNIQUE | NON_NULL, (empty)
    // 368: ap.as_va_list(): typeof(_23 = &mut _4) = &mut {l61} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 368: ap.as_va_list():   l61 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 369: stderr: typeof(_26) = *mut {l43} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 369: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 369: stderr: typeof(_27) = *mut {l45} *mut {l46} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 369: stderr:   l45 = UNIQUE | NON_NULL, (empty)
    // 369: stderr:   l46 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 370: stderr: typeof(_29) = *mut {l49} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 370: stderr:   l49 = UNIQUE | NON_NULL, (empty)
    // 370: stderr: typeof(_30) = *mut {l51} *mut {l52} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 370: stderr:   l51 = UNIQUE | NON_NULL, (empty)
    // 370: stderr:   l52 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn rep(mut fmt: *const libc::c_char, mut args: ...) {
// 373: mut fmt: typeof(_1) = *const {g5} i8
// 373: mut fmt:   g5 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
    // 375: verbose: typeof(_7) = *mut {l7} i32
    // 375: verbose:   l7 = UNIQUE | NON_NULL, (empty)
        return;
    }
    if isatty(1 as libc::c_int) == 0 {
        return;
    }
    fputs(
        b"c [lglddtrace] \0" as *const u8 as *const libc::c_char,
        // 382: b"c [lglddtrace ... _char: typeof(_15) = *const {l16} i8
        // 382: b"c [lglddtrace ... _char:   l16 = UNIQUE | NON_NULL, (empty)
        // 382: b"c [lglddtrace ... st u8: typeof(_16) = *const {l18} u8
        // 382: b"c [lglddtrace ... st u8:   l18 = UNIQUE | NON_NULL, (empty)
        // 382: b"c [lglddtrace] \0": typeof(_17) = *const {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 382: b"c [lglddtrace] \0":   l20 = UNIQUE | NON_NULL, (empty)
        // 382: b"c [lglddtrace] \0": typeof(_18) = & {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 382: b"c [lglddtrace] \0":   l22 = UNIQUE | NON_NULL, FIXED
        // 382: b"c [lglddtrace ... st u8: typeof(_16 = move _17 as *const u8 (Pointer(ArrayToPointer))) = *const {l60} u8
        // 382: b"c [lglddtrace ... st u8:   l60 = UNIQUE | NON_NULL, (empty)
        // 382: b"c [lglddtrace] \0": typeof(_18 = const b"c [lglddtrace] \x00") = & {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 382: b"c [lglddtrace] \0":   l58 = UNIQUE | NON_NULL, (empty)
        // 382: b"c [lglddtrace] \0": typeof(_17 = &raw const (*_18)) = *const {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 382: b"c [lglddtrace] \0":   l59 = UNIQUE | NON_NULL, (empty)
        // 382: b"c [lglddtrace ... _char: typeof(_15 = move _16 as *const i8 (Misc)) = *const {l61} i8
        // 382: b"c [lglddtrace ... _char:   l61 = UNIQUE | NON_NULL, (empty)
        stdout,
        // 383: stdout: typeof(_19) = *mut {l24} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 383: stdout:   l24 = UNIQUE | NON_NULL, (empty)
        // 383: stdout: typeof(_20) = *mut {l26} *mut {l27} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 383: stdout:   l26 = UNIQUE | NON_NULL, (empty)
        // 383: stdout:   l27 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 385: args.clone(): typeof(_22) = & {l30} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 385: args.clone():   l30 = UNIQUE | NON_NULL, (empty)
    // 385: args.clone(): typeof(_22 = &_2) = & {l62} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 385: args.clone():   l62 = UNIQUE | NON_NULL, (empty)
    vprintf(fmt, ap.as_va_list());
    // 386: fmt: typeof(_24) = *const {l33} i8
    // 386: fmt:   l33 = UNIQUE | NON_NULL, (empty)
    // 386: ap.as_va_list(): typeof(_26) = &mut {l36} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 386: ap.as_va_list():   l36 = UNIQUE | NON_NULL, (empty)
    // 386: ap.as_va_list(): typeof(_26 = &mut _3) = &mut {l63} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 386: ap.as_va_list():   l63 = UNIQUE | NON_NULL, (empty)
    fputs(b"       \r\0" as *const u8 as *const libc::c_char, stdout);
    // 387: b" \r\0" as *co ... _char: typeof(_28) = *const {l39} i8
    // 387: b" \r\0" as *co ... _char:   l39 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0" as *co ... st u8: typeof(_29) = *const {l41} u8
    // 387: b" \r\0" as *co ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0": typeof(_30) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 387: b" \r\0":   l43 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0": typeof(_31) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 387: b" \r\0":   l45 = UNIQUE | NON_NULL, FIXED
    // 387: stdout: typeof(_32) = *mut {l47} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 387: stdout:   l47 = UNIQUE | NON_NULL, (empty)
    // 387: stdout: typeof(_33) = *mut {l49} *mut {l50} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 387: stdout:   l49 = UNIQUE | NON_NULL, (empty)
    // 387: stdout:   l50 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0" as *co ... _char: typeof(_28 = move _29 as *const i8 (Misc)) = *const {l67} i8
    // 387: b" \r\0" as *co ... _char:   l67 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0" as *co ... st u8: typeof(_29 = move _30 as *const u8 (Pointer(ArrayToPointer))) = *const {l66} u8
    // 387: b" \r\0" as *co ... st u8:   l66 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0": typeof(_30 = &raw const (*_31)) = *const {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 387: b" \r\0":   l65 = UNIQUE | NON_NULL, (empty)
    // 387: b" \r\0": typeof(_31 = const b"       \r\x00") = & {l64} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
    // 387: b" \r\0":   l64 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 388: stdout: typeof(_35) = *mut {l53} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 388: stdout:   l53 = UNIQUE | NON_NULL, (empty)
    // 388: stdout: typeof(_36) = *mut {l55} *mut {l56} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 388: stdout:   l55 = UNIQUE | NON_NULL, (empty)
    // 388: stdout:   l56 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn msg(mut fmt: *const libc::c_char, mut args: ...) {
// 390: mut fmt: typeof(_1) = *const {g6} i8
// 390: mut fmt:   g6 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    if verbose == 0 {
    // 392: verbose: typeof(_7) = *mut {l7} i32
    // 392: verbose:   l7 = UNIQUE | NON_NULL, (empty)
        return;
    }
    fputs(
        b"c [lglddtrace] \0" as *const u8 as *const libc::c_char,
        // 396: b"c [lglddtrace ... _char: typeof(_10) = *const {l11} i8
        // 396: b"c [lglddtrace ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 396: b"c [lglddtrace ... st u8: typeof(_11) = *const {l13} u8
        // 396: b"c [lglddtrace ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 396: b"c [lglddtrace] \0": typeof(_12) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 396: b"c [lglddtrace] \0":   l15 = UNIQUE | NON_NULL, (empty)
        // 396: b"c [lglddtrace] \0": typeof(_13) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 396: b"c [lglddtrace] \0":   l17 = UNIQUE | NON_NULL, FIXED
        // 396: b"c [lglddtrace ... st u8: typeof(_11 = move _12 as *const u8 (Pointer(ArrayToPointer))) = *const {l48} u8
        // 396: b"c [lglddtrace ... st u8:   l48 = UNIQUE | NON_NULL, (empty)
        // 396: b"c [lglddtrace ... _char: typeof(_10 = move _11 as *const i8 (Misc)) = *const {l49} i8
        // 396: b"c [lglddtrace ... _char:   l49 = UNIQUE | NON_NULL, (empty)
        // 396: b"c [lglddtrace] \0": typeof(_12 = &raw const (*_13)) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 396: b"c [lglddtrace] \0":   l47 = UNIQUE | NON_NULL, (empty)
        // 396: b"c [lglddtrace] \0": typeof(_13 = const b"c [lglddtrace] \x00") = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 396: b"c [lglddtrace] \0":   l46 = UNIQUE | NON_NULL, (empty)
        stdout,
        // 397: stdout: typeof(_14) = *mut {l19} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 397: stdout:   l19 = UNIQUE | NON_NULL, (empty)
        // 397: stdout: typeof(_15) = *mut {l21} *mut {l22} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 397: stdout:   l21 = UNIQUE | NON_NULL, (empty)
        // 397: stdout:   l22 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 399: args.clone(): typeof(_17) = & {l25} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 399: args.clone():   l25 = UNIQUE | NON_NULL, (empty)
    // 399: args.clone(): typeof(_17 = &_2) = & {l50} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 399: args.clone():   l50 = UNIQUE | NON_NULL, (empty)
    vprintf(fmt, ap.as_va_list());
    // 400: fmt: typeof(_19) = *const {l28} i8
    // 400: fmt:   l28 = UNIQUE | NON_NULL, (empty)
    // 400: ap.as_va_list(): typeof(_21) = &mut {l31} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 400: ap.as_va_list():   l31 = UNIQUE | NON_NULL, (empty)
    // 400: ap.as_va_list(): typeof(_21 = &mut _3) = &mut {l51} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 400: ap.as_va_list():   l51 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stdout);
    // 401: stdout: typeof(_24) = *mut {l35} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 401: stdout:   l35 = UNIQUE | NON_NULL, (empty)
    // 401: stdout: typeof(_25) = *mut {l37} *mut {l38} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 401: stdout:   l37 = UNIQUE | NON_NULL, (empty)
    // 401: stdout:   l38 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 402: stdout: typeof(_27) = *mut {l41} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 402: stdout:   l41 = UNIQUE | NON_NULL, (empty)
    // 402: stdout: typeof(_28) = *mut {l43} *mut {l44} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 402: stdout:   l43 = UNIQUE | NON_NULL, (empty)
    // 402: stdout:   l44 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn isnumstr(mut str: *const libc::c_char) -> libc::c_int {
// 404: mut str: typeof(_1) = *const {g7} i8
// 404: mut str:   g7 = UNIQUE | NON_NULL, FIXED
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    // 405: mut p: typeof(_3) = *const {l3} i8
    // 405: mut p:   l3 = UNIQUE | NON_NULL, (empty)
    // 405: 0 as *const lib ... _char: typeof(_3 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l74} i8
    // 405: 0 as *const lib ... _char:   l74 = UNIQUE | NON_NULL, (empty)
    let mut ch: libc::c_int = 0;
    p = str;
    // 407: str: typeof(_5) = *const {l6} i8
    // 407: str:   l6 = UNIQUE | NON_NULL, (empty)
    if *p as libc::c_int == '-' as i32 {
        p = p.offset(1);
        // 409: p.offset(1): typeof(_11) = *const {l13} i8
        // 409: p.offset(1):   l13 = UNIQUE | NON_NULL, (empty)
        // 409: p: typeof(_12) = *const {l15} i8
        // 409: p:   l15 = UNIQUE | NON_NULL, (empty)
        p;
        // 410: p: typeof(_13) = *const {l17} i8
        // 410: p:   l17 = UNIQUE | NON_NULL, (empty)
    }
    let fresh1 = p;
    // 412: fresh1: typeof(_14) = *const {l19} i8
    // 412: fresh1:   l19 = UNIQUE | NON_NULL, (empty)
    p = p.offset(1);
    // 413: p.offset(1): typeof(_15) = *const {l21} i8
    // 413: p.offset(1):   l21 = UNIQUE | NON_NULL, (empty)
    // 413: p: typeof(_16) = *const {l23} i8
    // 413: p:   l23 = UNIQUE | NON_NULL, (empty)
    if *(*__ctype_b_loc()).offset(*fresh1 as libc::c_int as isize) as libc::c_int
    // 414: (*__ctype_b_loc ... size): typeof(_22) = *const {l30} u16
    // 414: (*__ctype_b_loc ... size):   l30 = UNIQUE | NON_NULL, (empty)
    // 414: (*__ctype_b_loc()): typeof(_23) = *const {l32} u16
    // 414: (*__ctype_b_loc()):   l32 = UNIQUE | NON_NULL, (empty)
    // 414: __ctype_b_loc(): typeof(_24) = *mut {l34} *const {l35} u16
    // 414: __ctype_b_loc():   l34 = UNIQUE | NON_NULL, (empty)
    // 414: __ctype_b_loc():   l35 = UNIQUE | NON_NULL, (empty)
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return 0 as libc::c_int;
    }
    loop {
        ch = *p as libc::c_int;
        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        // 422: (*__ctype_b_loc ... size): typeof(_41) = *const {l53} u16
        // 422: (*__ctype_b_loc ... size):   l53 = UNIQUE | NON_NULL, (empty)
        // 422: (*__ctype_b_loc()): typeof(_42) = *const {l55} u16
        // 422: (*__ctype_b_loc()):   l55 = UNIQUE | NON_NULL, (empty)
        // 422: __ctype_b_loc(): typeof(_43) = *mut {l57} *const {l58} u16
        // 422: __ctype_b_loc():   l57 = UNIQUE | NON_NULL, (empty)
        // 422: __ctype_b_loc():   l58 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0)
        {
            break;
        }
        p = p.offset(1);
        // 428: p.offset(1): typeof(_50) = *const {l66} i8
        // 428: p.offset(1):   l66 = UNIQUE | NON_NULL, (empty)
        // 428: p: typeof(_51) = *const {l68} i8
        // 428: p:   l68 = UNIQUE | NON_NULL, (empty)
        p;
        // 429: p: typeof(_52) = *const {l70} i8
        // 429: p:   l70 = UNIQUE | NON_NULL, (empty)
    }
    return (ch == 0) as libc::c_int;
}
unsafe extern "C" fn intarg(mut op: *mut libc::c_char) -> libc::c_int {
// 433: mut op: typeof(_1) = *mut {g8} i8
// 433: mut op:   g8 = UNIQUE | NON_NULL, FIXED
    let mut tok: *const libc::c_char = 0 as *const libc::c_char;
    // 434: mut tok: typeof(_3) = *const {l3} i8
    // 434: mut tok:   l3 = UNIQUE | NON_NULL, (empty)
    // 434: 0 as *const lib ... _char: typeof(_3 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l128} i8
    // 434: 0 as *const lib ... _char:   l128 = UNIQUE | NON_NULL, (empty)
    tok = strtok(
    // 435: strtok( 0 as *m ... ar, ): typeof(_4) = *mut {l5} i8
    // 435: strtok( 0 as *m ... ar, ):   l5 = UNIQUE | NON_NULL, (empty)
    // 435: tok = strtok( 0 ... ar, ): typeof(_3 = move _4 as *const i8 (Pointer(MutToConstPointer))) = *const {l134} i8
    // 435: tok = strtok( 0 ... ar, ):   l134 = UNIQUE | NON_NULL, (empty)
        0 as *mut libc::c_char,
        // 436: 0 as *mut libc: ... _char: typeof(_5) = *mut {l7} i8
        // 436: 0 as *mut libc: ... _char:   l7 = UNIQUE | NON_NULL, (empty)
        // 436: 0 as *mut libc: ... _char: typeof(_5 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l129} i8
        // 436: 0 as *mut libc: ... _char:   l129 = UNIQUE | NON_NULL, (empty)
        b" \0" as *const u8 as *const libc::c_char,
        // 437: b" \0" as *cons ... _char: typeof(_6) = *const {l9} i8
        // 437: b" \0" as *cons ... _char:   l9 = UNIQUE | NON_NULL, (empty)
        // 437: b" \0" as *const u8: typeof(_7) = *const {l11} u8
        // 437: b" \0" as *const u8:   l11 = UNIQUE | NON_NULL, (empty)
        // 437: b" \0": typeof(_8) = *const {l13} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 437: b" \0":   l13 = UNIQUE | NON_NULL, (empty)
        // 437: b" \0": typeof(_9) = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 437: b" \0":   l15 = UNIQUE | NON_NULL, FIXED
        // 437: b" \0" as *const u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l132} u8
        // 437: b" \0" as *const u8:   l132 = UNIQUE | NON_NULL, (empty)
        // 437: b" \0" as *cons ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l133} i8
        // 437: b" \0" as *cons ... _char:   l133 = UNIQUE | NON_NULL, (empty)
        // 437: b" \0": typeof(_9 = const b" \x00") = & {l130} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 437: b" \0":   l130 = UNIQUE | NON_NULL, (empty)
        // 437: b" \0": typeof(_8 = &raw const (*_9)) = *const {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 437: b" \0":   l131 = UNIQUE | NON_NULL, (empty)
    );
    if tok.is_null()
    // 439: tok: typeof(_14) = *const {l21} i8
    // 439: tok:   l21 = UNIQUE | NON_NULL, (empty)
        || isnumstr(tok) == 0
        // 440: tok: typeof(_17) = *const {l25} i8
        // 440: tok:   l25 = UNIQUE | NON_NULL, (empty)
        || !(strtok(
        // 441: (strtok( 0 as * ... r, )): typeof(_20) = *mut {l29} i8
        // 441: (strtok( 0 as * ... r, )):   l29 = UNIQUE | NON_NULL, (empty)
            0 as *mut libc::c_char,
            // 442: 0 as *mut libc: ... _char: typeof(_21) = *mut {l31} i8
            // 442: 0 as *mut libc: ... _char:   l31 = UNIQUE | NON_NULL, (empty)
            // 442: 0 as *mut libc: ... _char: typeof(_21 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l135} i8
            // 442: 0 as *mut libc: ... _char:   l135 = UNIQUE | NON_NULL, (empty)
            b" \0" as *const u8 as *const libc::c_char,
            // 443: b" \0" as *cons ... _char: typeof(_22) = *const {l33} i8
            // 443: b" \0" as *cons ... _char:   l33 = UNIQUE | NON_NULL, (empty)
            // 443: b" \0" as *const u8: typeof(_23) = *const {l35} u8
            // 443: b" \0" as *const u8:   l35 = UNIQUE | NON_NULL, (empty)
            // 443: b" \0": typeof(_24) = *const {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 443: b" \0":   l37 = UNIQUE | NON_NULL, (empty)
            // 443: b" \0": typeof(_25) = & {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 443: b" \0":   l39 = UNIQUE | NON_NULL, FIXED
            // 443: b" \0" as *cons ... _char: typeof(_22 = move _23 as *const i8 (Misc)) = *const {l139} i8
            // 443: b" \0" as *cons ... _char:   l139 = UNIQUE | NON_NULL, (empty)
            // 443: b" \0": typeof(_25 = const b" \x00") = & {l136} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 443: b" \0":   l136 = UNIQUE | NON_NULL, (empty)
            // 443: b" \0" as *const u8: typeof(_23 = move _24 as *const u8 (Pointer(ArrayToPointer))) = *const {l138} u8
            // 443: b" \0" as *const u8:   l138 = UNIQUE | NON_NULL, (empty)
            // 443: b" \0": typeof(_24 = &raw const (*_25)) = *const {l137} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
            // 443: b" \0":   l137 = UNIQUE | NON_NULL, (empty)
        ))
        .is_null()
    {
        perr(
            b"expected integer argument for '%s'\0" as *const u8 as *const libc::c_char,
            // 448: b"expected inte ... _char: typeof(_27) = *const {l42} i8
            // 448: b"expected inte ... _char:   l42 = UNIQUE | NON_NULL, (empty)
            // 448: b"expected inte ... st u8: typeof(_28) = *const {l44} u8
            // 448: b"expected inte ... st u8:   l44 = UNIQUE | NON_NULL, (empty)
            // 448: b"expected inte ... s'\0": typeof(_29) = *const {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 448: b"expected inte ... s'\0":   l46 = UNIQUE | NON_NULL, (empty)
            // 448: b"expected inte ... s'\0": typeof(_30) = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 448: b"expected inte ... s'\0":   l48 = UNIQUE | NON_NULL, FIXED
            // 448: b"expected inte ... s'\0": typeof(_30 = const b"expected integer argument for \'%s\'\x00") = & {l140} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 448: b"expected inte ... s'\0":   l140 = UNIQUE | NON_NULL, (empty)
            // 448: b"expected inte ... _char: typeof(_27 = move _28 as *const i8 (Misc)) = *const {l143} i8
            // 448: b"expected inte ... _char:   l143 = UNIQUE | NON_NULL, (empty)
            // 448: b"expected inte ... s'\0": typeof(_29 = &raw const (*_30)) = *const {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 448: b"expected inte ... s'\0":   l141 = UNIQUE | NON_NULL, (empty)
            // 448: b"expected inte ... st u8: typeof(_28 = move _29 as *const u8 (Pointer(ArrayToPointer))) = *const {l142} u8
            // 448: b"expected inte ... st u8:   l142 = UNIQUE | NON_NULL, (empty)
            op,
            // 449: op: typeof(_31) = *mut {l50} i8
            // 449: op:   l50 = UNIQUE | NON_NULL, (empty)
        );
    }
    if !tok.is_null() {
    // 452: tok: typeof(_35) = *const {l55} i8
    // 452: tok:   l55 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"tok\0" as *const u8 as *const libc::c_char,
            // 455: b"tok\0" as *co ... _char: typeof(_38) = *const {l59} i8
            // 455: b"tok\0" as *co ... _char:   l59 = UNIQUE | NON_NULL, (empty)
            // 455: b"tok\0" as *co ... st u8: typeof(_39) = *const {l61} u8
            // 455: b"tok\0" as *co ... st u8:   l61 = UNIQUE | NON_NULL, (empty)
            // 455: b"tok\0": typeof(_40) = *const {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 455: b"tok\0":   l63 = UNIQUE | NON_NULL, (empty)
            // 455: b"tok\0": typeof(_41) = & {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 455: b"tok\0":   l65 = UNIQUE | NON_NULL, FIXED
            // 455: b"tok\0" as *co ... st u8: typeof(_39 = move _40 as *const u8 (Pointer(ArrayToPointer))) = *const {l146} u8
            // 455: b"tok\0" as *co ... st u8:   l146 = UNIQUE | NON_NULL, (empty)
            // 455: b"tok\0" as *co ... _char: typeof(_38 = move _39 as *const i8 (Misc)) = *const {l147} i8
            // 455: b"tok\0" as *co ... _char:   l147 = UNIQUE | NON_NULL, (empty)
            // 455: b"tok\0": typeof(_41 = const b"tok\x00") = & {l144} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 455: b"tok\0":   l144 = UNIQUE | NON_NULL, (empty)
            // 455: b"tok\0": typeof(_40 = &raw const (*_41)) = *const {l145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 455: b"tok\0":   l145 = UNIQUE | NON_NULL, (empty)
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            // 456: b"lglddtrace.c\ ... _char: typeof(_42) = *const {l67} i8
            // 456: b"lglddtrace.c\ ... _char:   l67 = UNIQUE | NON_NULL, (empty)
            // 456: b"lglddtrace.c\ ... st u8: typeof(_43) = *const {l69} u8
            // 456: b"lglddtrace.c\ ... st u8:   l69 = UNIQUE | NON_NULL, (empty)
            // 456: b"lglddtrace.c\0": typeof(_44) = *const {l71} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 456: b"lglddtrace.c\0":   l71 = UNIQUE | NON_NULL, (empty)
            // 456: b"lglddtrace.c\0": typeof(_45) = & {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 456: b"lglddtrace.c\0":   l73 = UNIQUE | NON_NULL, FIXED
            // 456: b"lglddtrace.c\0": typeof(_45 = const b"lglddtrace.c\x00") = & {l148} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 456: b"lglddtrace.c\0":   l148 = UNIQUE | NON_NULL, (empty)
            // 456: b"lglddtrace.c\ ... st u8: typeof(_43 = move _44 as *const u8 (Pointer(ArrayToPointer))) = *const {l150} u8
            // 456: b"lglddtrace.c\ ... st u8:   l150 = UNIQUE | NON_NULL, (empty)
            // 456: b"lglddtrace.c\0": typeof(_44 = &raw const (*_45)) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 456: b"lglddtrace.c\0":   l149 = UNIQUE | NON_NULL, (empty)
            // 456: b"lglddtrace.c\ ... _char: typeof(_42 = move _43 as *const i8 (Misc)) = *const {l151} i8
            // 456: b"lglddtrace.c\ ... _char:   l151 = UNIQUE | NON_NULL, (empty)
            166 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"int intarg(char *)\0"))
            // 458: (*::core::mem:: ... ptr(): typeof(_48) = *const {l77} i8
            // 458: (*::core::mem:: ... ptr():   l77 = UNIQUE | NON_NULL, (empty)
            // 458: (*::core::mem:: ... ptr(): typeof(_49) = & {l79} [i8]
            // 458: (*::core::mem:: ... ptr():   l79 = UNIQUE | NON_NULL, FIXED
            // 458: (*::core::mem:: ... ptr(): typeof(_50) = & {l81} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: (*::core::mem:: ... ptr():   l81 = UNIQUE | NON_NULL, (empty)
            // 458: ::core::mem::tr ... )\0"): typeof(_51) = & {l83} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: ::core::mem::tr ... )\0"):   l83 = UNIQUE | NON_NULL, FIXED
            // 458: b"int intarg(ch ... *)\0": typeof(_52) = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: b"int intarg(ch ... *)\0":   l85 = UNIQUE | NON_NULL, (empty)
            // 458: b"int intarg(ch ... *)\0": typeof(_53) = & {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: b"int intarg(ch ... *)\0":   l87 = UNIQUE | NON_NULL, FIXED
            // 458: b"int intarg(ch ... *)\0": typeof(_53 = const b"int intarg(char *)\x00") = & {l152} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: b"int intarg(ch ... *)\0":   l152 = UNIQUE | NON_NULL, (empty)
            // 458: b"int intarg(ch ... *)\0": typeof(_52 = &(*_53)) = & {l153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: b"int intarg(ch ... *)\0":   l153 = UNIQUE | NON_NULL, (empty)
            // 458: (*::core::mem:: ... ptr(): typeof(_50 = &(*_51)) = & {l154} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 458: (*::core::mem:: ... ptr():   l154 = UNIQUE | NON_NULL, (empty)
            // 458: (*::core::mem:: ... ptr(): typeof(_49 = move _50 as &[i8] (Pointer(Unsize))) = & {l155} [i8]
            // 458: (*::core::mem:: ... ptr():   l155 = UNIQUE | NON_NULL, FIXED
                .as_ptr(),
        );
    }
    'c_5090: {
        if !tok.is_null() {
        // 463: tok: typeof(_57) = *const {l92} i8
        // 463: tok:   l92 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"tok\0" as *const u8 as *const libc::c_char,
                // 466: b"tok\0" as *co ... _char: typeof(_60) = *const {l96} i8
                // 466: b"tok\0" as *co ... _char:   l96 = UNIQUE | NON_NULL, (empty)
                // 466: b"tok\0" as *co ... st u8: typeof(_61) = *const {l98} u8
                // 466: b"tok\0" as *co ... st u8:   l98 = UNIQUE | NON_NULL, (empty)
                // 466: b"tok\0": typeof(_62) = *const {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 466: b"tok\0":   l100 = UNIQUE | NON_NULL, (empty)
                // 466: b"tok\0": typeof(_63) = & {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 466: b"tok\0":   l102 = UNIQUE | NON_NULL, FIXED
                // 466: b"tok\0": typeof(_62 = &raw const (*_63)) = *const {l157} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 466: b"tok\0":   l157 = UNIQUE | NON_NULL, (empty)
                // 466: b"tok\0" as *co ... st u8: typeof(_61 = move _62 as *const u8 (Pointer(ArrayToPointer))) = *const {l158} u8
                // 466: b"tok\0" as *co ... st u8:   l158 = UNIQUE | NON_NULL, (empty)
                // 466: b"tok\0" as *co ... _char: typeof(_60 = move _61 as *const i8 (Misc)) = *const {l159} i8
                // 466: b"tok\0" as *co ... _char:   l159 = UNIQUE | NON_NULL, (empty)
                // 466: b"tok\0": typeof(_63 = const b"tok\x00") = & {l156} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                // 466: b"tok\0":   l156 = UNIQUE | NON_NULL, (empty)
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                // 467: b"lglddtrace.c\ ... _char: typeof(_64) = *const {l104} i8
                // 467: b"lglddtrace.c\ ... _char:   l104 = UNIQUE | NON_NULL, (empty)
                // 467: b"lglddtrace.c\ ... st u8: typeof(_65) = *const {l106} u8
                // 467: b"lglddtrace.c\ ... st u8:   l106 = UNIQUE | NON_NULL, (empty)
                // 467: b"lglddtrace.c\0": typeof(_66) = *const {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 467: b"lglddtrace.c\0":   l108 = UNIQUE | NON_NULL, (empty)
                // 467: b"lglddtrace.c\0": typeof(_67) = & {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 467: b"lglddtrace.c\0":   l110 = UNIQUE | NON_NULL, FIXED
                // 467: b"lglddtrace.c\0": typeof(_67 = const b"lglddtrace.c\x00") = & {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 467: b"lglddtrace.c\0":   l160 = UNIQUE | NON_NULL, (empty)
                // 467: b"lglddtrace.c\0": typeof(_66 = &raw const (*_67)) = *const {l161} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 467: b"lglddtrace.c\0":   l161 = UNIQUE | NON_NULL, (empty)
                // 467: b"lglddtrace.c\ ... st u8: typeof(_65 = move _66 as *const u8 (Pointer(ArrayToPointer))) = *const {l162} u8
                // 467: b"lglddtrace.c\ ... st u8:   l162 = UNIQUE | NON_NULL, (empty)
                // 467: b"lglddtrace.c\ ... _char: typeof(_64 = move _65 as *const i8 (Misc)) = *const {l163} i8
                // 467: b"lglddtrace.c\ ... _char:   l163 = UNIQUE | NON_NULL, (empty)
                166 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                // 469: (*::core::mem:: ... ptr(): typeof(_70) = *const {l114} i8
                // 469: (*::core::mem:: ... ptr():   l114 = UNIQUE | NON_NULL, (empty)
                // 469: (*::core::mem:: ... ptr(): typeof(_71) = & {l116} [i8]
                // 469: (*::core::mem:: ... ptr():   l116 = UNIQUE | NON_NULL, FIXED
                // 469: (*::core::mem:: ... ptr(): typeof(_72) = & {l118} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 469: (*::core::mem:: ... ptr():   l118 = UNIQUE | NON_NULL, (empty)
                // 469: ::core::mem::tr ... 0", ): typeof(_73) = & {l120} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 469: ::core::mem::tr ... 0", ):   l120 = UNIQUE | NON_NULL, FIXED
                // 469: (*::core::mem:: ... ptr(): typeof(_72 = &(*_73)) = & {l166} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 469: (*::core::mem:: ... ptr():   l166 = UNIQUE | NON_NULL, (empty)
                // 469: (*::core::mem:: ... ptr(): typeof(_71 = move _72 as &[i8] (Pointer(Unsize))) = & {l167} [i8]
                // 469: (*::core::mem:: ... ptr():   l167 = UNIQUE | NON_NULL, FIXED
                    b"int intarg(char *)\0",
                    // 470: b"int intarg(ch ... *)\0": typeof(_74) = & {l122} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 470: b"int intarg(ch ... *)\0":   l122 = UNIQUE | NON_NULL, (empty)
                    // 470: b"int intarg(ch ... *)\0": typeof(_75) = & {l124} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 470: b"int intarg(ch ... *)\0":   l124 = UNIQUE | NON_NULL, FIXED
                    // 470: b"int intarg(ch ... *)\0": typeof(_74 = &(*_75)) = & {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 470: b"int intarg(ch ... *)\0":   l165 = UNIQUE | NON_NULL, (empty)
                    // 470: b"int intarg(ch ... *)\0": typeof(_75 = const b"int intarg(char *)\x00") = & {l164} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 470: b"int intarg(ch ... *)\0":   l164 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    return atoi(tok);
    // 476: tok: typeof(_76) = *const {l126} i8
    // 476: tok:   l126 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn remr<'h0>(mut r: &'h0 (Range)) -> libc::c_int {
// 478: mut r: typeof(_1) = *mut {g9} DefId(0:370 ~ lglddtrace[7e63]::Range)
// 478: mut r:   g9 = READ | UNIQUE | NON_NULL, (empty)
    return ((*r).removed <= runs) as libc::c_int;
    // 479: runs: typeof(_6) = *mut {l6} i32
    // 479: runs:   l6 = READ | UNIQUE | NON_NULL, (empty)
}
unsafe fn remr_shim(arg0: *mut Range) -> libc::c_int {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_result = remr(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn reme<'h0,'h1>(mut e: &'h0 (Event<'h1>)) -> libc::c_int {
// 481: mut e: typeof(_1) = *mut {g10} DefId(0:354 ~ lglddtrace[7e63]::Event)
// 481: mut e:   g10 = READ | UNIQUE | NON_NULL, (empty)
    return ((*e).removed <= runs) as libc::c_int;
    // 482: runs: typeof(_6) = *mut {l6} i32
    // 482: runs:   l6 = READ | UNIQUE | NON_NULL, (empty)
}
unsafe fn reme_shim(arg0: *mut Event) -> libc::c_int {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_result = reme(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn onabort<'h0>(mut d: &'h0 (libc::c_void)) {
// 484: mut d: typeof(_1) = *mut {g11} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 484: mut d:   g11 = UNIQUE | NON_NULL, (empty)
    exit(0 as libc::c_int);
}
unsafe fn onabort_shim(arg0: *mut libc::c_void) {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_result = onabort(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn process() {
    let mut saved1: libc::c_int = 0;
    let mut saved2: libc::c_int = 0;
    let mut null: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut rlim: rlimit = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };
    let mut lgl: *mut LGL = 0 as *mut LGL;
    // 497: mut lgl: typeof(_7) = *mut {l7} LGL
    // 497: mut lgl:   l7 = UNIQUE | NON_NULL, (empty)
    // 497: 0 as *mut LGL: typeof(_7 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l776} LGL
    // 497: 0 as *mut LGL:   l776 = UNIQUE | NON_NULL, (empty)
    let mut e: *mut Event = 0 as *mut Event;
    // 498: mut e: typeof(_8) = *mut {l9} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 498: mut e:   l9 = UNIQUE | NON_NULL, (empty)
    // 498: 0 as *mut Event: typeof(_8 = const 0_usize as *mut Event (PointerFromExposedAddress)) = *mut {l777} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 498: 0 as *mut Event:   l777 = UNIQUE | NON_NULL, (empty)
    let mut o: *mut Opt = 0 as *mut Opt;
    // 499: mut o: typeof(_9) = *mut {l11} DefId(0:362 ~ lglddtrace[7e63]::Opt)
    // 499: mut o:   l11 = UNIQUE | NON_NULL, (empty)
    // 499: 0 as *mut Opt: typeof(_9 = const 0_usize as *mut Opt (PointerFromExposedAddress)) = *mut {l778} DefId(0:362 ~ lglddtrace[7e63]::Opt)
    // 499: 0 as *mut Opt:   l778 = UNIQUE | NON_NULL, (empty)
    rlim.rlim_cur = timelimit as rlim_t;
    // 500: timelimit: typeof(_11) = *mut {l14} i32
    // 500: timelimit:   l14 = UNIQUE | NON_NULL, (empty)
    rlim.rlim_max = (rlim.rlim_cur).wrapping_add(10 as libc::c_int as rlim_t);
    setrlimit(RLIMIT_CPU as libc::c_int, &mut rlim);
    // 502: &mut rlim: typeof(_18) = *const {l22} DefId(0:348 ~ lglddtrace[7e63]::rlimit)
    // 502: &mut rlim:   l22 = UNIQUE | NON_NULL, (empty)
    // 502: &mut rlim: typeof(_19) = &mut {l24} DefId(0:348 ~ lglddtrace[7e63]::rlimit)
    // 502: &mut rlim:   l24 = UNIQUE | NON_NULL, (empty)
    // 502: &mut rlim: typeof(_19 = &mut _6) = &mut {l779} DefId(0:348 ~ lglddtrace[7e63]::rlimit)
    // 502: &mut rlim:   l779 = UNIQUE | NON_NULL, (empty)
    // 502: &mut rlim: typeof(_18 = &raw const (*_19)) = *const {l780} DefId(0:348 ~ lglddtrace[7e63]::rlimit)
    // 502: &mut rlim:   l780 = UNIQUE | NON_NULL, (empty)
    saved1 = dup(1 as libc::c_int);
    saved2 = dup(2 as libc::c_int);
    null = open(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        // 506: b"/dev/null\0"  ... _char: typeof(_25) = *const {l31} i8
        // 506: b"/dev/null\0"  ... _char:   l31 = UNIQUE | NON_NULL, (empty)
        // 506: b"/dev/null\0"  ... st u8: typeof(_26) = *const {l33} u8
        // 506: b"/dev/null\0"  ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
        // 506: b"/dev/null\0": typeof(_27) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 506: b"/dev/null\0":   l35 = UNIQUE | NON_NULL, (empty)
        // 506: b"/dev/null\0": typeof(_28) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 506: b"/dev/null\0":   l37 = UNIQUE | NON_NULL, FIXED
        // 506: b"/dev/null\0": typeof(_28 = const b"/dev/null\x00") = & {l781} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 506: b"/dev/null\0":   l781 = UNIQUE | NON_NULL, (empty)
        // 506: b"/dev/null\0"  ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l783} u8
        // 506: b"/dev/null\0"  ... st u8:   l783 = UNIQUE | NON_NULL, (empty)
        // 506: b"/dev/null\0": typeof(_27 = &raw const (*_28)) = *const {l782} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 506: b"/dev/null\0":   l782 = UNIQUE | NON_NULL, (empty)
        // 506: b"/dev/null\0"  ... _char: typeof(_25 = move _26 as *const i8 (Misc)) = *const {l784} i8
        // 506: b"/dev/null\0"  ... _char:   l784 = UNIQUE | NON_NULL, (empty)
        0o1 as libc::c_int,
    );
    close(1 as libc::c_int);
    close(2 as libc::c_int);
    tmp = dup(null);
    if tmp == 1 as libc::c_int {
    } else {
        __assert_fail(
            b"tmp == 1\0" as *const u8 as *const libc::c_char,
            // 515: b"tmp == 1\0" a ... _char: typeof(_42) = *const {l52} i8
            // 515: b"tmp == 1\0" a ... _char:   l52 = UNIQUE | NON_NULL, (empty)
            // 515: b"tmp == 1\0" a ... st u8: typeof(_43) = *const {l54} u8
            // 515: b"tmp == 1\0" a ... st u8:   l54 = UNIQUE | NON_NULL, (empty)
            // 515: b"tmp == 1\0": typeof(_44) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 515: b"tmp == 1\0":   l56 = UNIQUE | NON_NULL, (empty)
            // 515: b"tmp == 1\0": typeof(_45) = & {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 515: b"tmp == 1\0":   l58 = UNIQUE | NON_NULL, FIXED
            // 515: b"tmp == 1\0": typeof(_45 = const b"tmp == 1\x00") = & {l785} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 515: b"tmp == 1\0":   l785 = UNIQUE | NON_NULL, (empty)
            // 515: b"tmp == 1\0" a ... _char: typeof(_42 = move _43 as *const i8 (Misc)) = *const {l788} i8
            // 515: b"tmp == 1\0" a ... _char:   l788 = UNIQUE | NON_NULL, (empty)
            // 515: b"tmp == 1\0" a ... st u8: typeof(_43 = move _44 as *const u8 (Pointer(ArrayToPointer))) = *const {l787} u8
            // 515: b"tmp == 1\0" a ... st u8:   l787 = UNIQUE | NON_NULL, (empty)
            // 515: b"tmp == 1\0": typeof(_44 = &raw const (*_45)) = *const {l786} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 515: b"tmp == 1\0":   l786 = UNIQUE | NON_NULL, (empty)
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            // 516: b"lglddtrace.c\ ... _char: typeof(_46) = *const {l60} i8
            // 516: b"lglddtrace.c\ ... _char:   l60 = UNIQUE | NON_NULL, (empty)
            // 516: b"lglddtrace.c\ ... st u8: typeof(_47) = *const {l62} u8
            // 516: b"lglddtrace.c\ ... st u8:   l62 = UNIQUE | NON_NULL, (empty)
            // 516: b"lglddtrace.c\0": typeof(_48) = *const {l64} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 516: b"lglddtrace.c\0":   l64 = UNIQUE | NON_NULL, (empty)
            // 516: b"lglddtrace.c\0": typeof(_49) = & {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 516: b"lglddtrace.c\0":   l66 = UNIQUE | NON_NULL, FIXED
            // 516: b"lglddtrace.c\0": typeof(_49 = const b"lglddtrace.c\x00") = & {l789} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 516: b"lglddtrace.c\0":   l789 = UNIQUE | NON_NULL, (empty)
            // 516: b"lglddtrace.c\ ... st u8: typeof(_47 = move _48 as *const u8 (Pointer(ArrayToPointer))) = *const {l791} u8
            // 516: b"lglddtrace.c\ ... st u8:   l791 = UNIQUE | NON_NULL, (empty)
            // 516: b"lglddtrace.c\0": typeof(_48 = &raw const (*_49)) = *const {l790} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 516: b"lglddtrace.c\0":   l790 = UNIQUE | NON_NULL, (empty)
            // 516: b"lglddtrace.c\ ... _char: typeof(_46 = move _47 as *const i8 (Misc)) = *const {l792} i8
            // 516: b"lglddtrace.c\ ... _char:   l792 = UNIQUE | NON_NULL, (empty)
            189 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"void process(void)\0"))
            // 518: (*::core::mem:: ... ptr(): typeof(_52) = *const {l70} i8
            // 518: (*::core::mem:: ... ptr():   l70 = UNIQUE | NON_NULL, (empty)
            // 518: (*::core::mem:: ... ptr(): typeof(_53) = & {l72} [i8]
            // 518: (*::core::mem:: ... ptr():   l72 = UNIQUE | NON_NULL, FIXED
            // 518: (*::core::mem:: ... ptr(): typeof(_54) = & {l74} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: (*::core::mem:: ... ptr():   l74 = UNIQUE | NON_NULL, (empty)
            // 518: ::core::mem::tr ... )\0"): typeof(_55) = & {l76} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: ::core::mem::tr ... )\0"):   l76 = UNIQUE | NON_NULL, FIXED
            // 518: b"void process( ... d)\0": typeof(_56) = & {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: b"void process( ... d)\0":   l78 = UNIQUE | NON_NULL, (empty)
            // 518: b"void process( ... d)\0": typeof(_57) = & {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: b"void process( ... d)\0":   l80 = UNIQUE | NON_NULL, FIXED
            // 518: (*::core::mem:: ... ptr(): typeof(_54 = &(*_55)) = & {l795} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: (*::core::mem:: ... ptr():   l795 = UNIQUE | NON_NULL, (empty)
            // 518: (*::core::mem:: ... ptr(): typeof(_53 = move _54 as &[i8] (Pointer(Unsize))) = & {l796} [i8]
            // 518: (*::core::mem:: ... ptr():   l796 = UNIQUE | NON_NULL, FIXED
            // 518: b"void process( ... d)\0": typeof(_57 = const b"void process(void)\x00") = & {l793} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: b"void process( ... d)\0":   l793 = UNIQUE | NON_NULL, (empty)
            // 518: b"void process( ... d)\0": typeof(_56 = &(*_57)) = & {l794} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 518: b"void process( ... d)\0":   l794 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_6042: {
        if tmp == 1 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                // 526: b"tmp == 1\0" a ... _char: typeof(_64) = *const {l88} i8
                // 526: b"tmp == 1\0" a ... _char:   l88 = UNIQUE | NON_NULL, (empty)
                // 526: b"tmp == 1\0" a ... st u8: typeof(_65) = *const {l90} u8
                // 526: b"tmp == 1\0" a ... st u8:   l90 = UNIQUE | NON_NULL, (empty)
                // 526: b"tmp == 1\0": typeof(_66) = *const {l92} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 526: b"tmp == 1\0":   l92 = UNIQUE | NON_NULL, (empty)
                // 526: b"tmp == 1\0": typeof(_67) = & {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 526: b"tmp == 1\0":   l94 = UNIQUE | NON_NULL, FIXED
                // 526: b"tmp == 1\0": typeof(_67 = const b"tmp == 1\x00") = & {l797} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 526: b"tmp == 1\0":   l797 = UNIQUE | NON_NULL, (empty)
                // 526: b"tmp == 1\0" a ... _char: typeof(_64 = move _65 as *const i8 (Misc)) = *const {l800} i8
                // 526: b"tmp == 1\0" a ... _char:   l800 = UNIQUE | NON_NULL, (empty)
                // 526: b"tmp == 1\0" a ... st u8: typeof(_65 = move _66 as *const u8 (Pointer(ArrayToPointer))) = *const {l799} u8
                // 526: b"tmp == 1\0" a ... st u8:   l799 = UNIQUE | NON_NULL, (empty)
                // 526: b"tmp == 1\0": typeof(_66 = &raw const (*_67)) = *const {l798} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 526: b"tmp == 1\0":   l798 = UNIQUE | NON_NULL, (empty)
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                // 527: b"lglddtrace.c\ ... _char: typeof(_68) = *const {l96} i8
                // 527: b"lglddtrace.c\ ... _char:   l96 = UNIQUE | NON_NULL, (empty)
                // 527: b"lglddtrace.c\ ... st u8: typeof(_69) = *const {l98} u8
                // 527: b"lglddtrace.c\ ... st u8:   l98 = UNIQUE | NON_NULL, (empty)
                // 527: b"lglddtrace.c\0": typeof(_70) = *const {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 527: b"lglddtrace.c\0":   l100 = UNIQUE | NON_NULL, (empty)
                // 527: b"lglddtrace.c\0": typeof(_71) = & {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 527: b"lglddtrace.c\0":   l102 = UNIQUE | NON_NULL, FIXED
                // 527: b"lglddtrace.c\ ... st u8: typeof(_69 = move _70 as *const u8 (Pointer(ArrayToPointer))) = *const {l803} u8
                // 527: b"lglddtrace.c\ ... st u8:   l803 = UNIQUE | NON_NULL, (empty)
                // 527: b"lglddtrace.c\ ... _char: typeof(_68 = move _69 as *const i8 (Misc)) = *const {l804} i8
                // 527: b"lglddtrace.c\ ... _char:   l804 = UNIQUE | NON_NULL, (empty)
                // 527: b"lglddtrace.c\0": typeof(_70 = &raw const (*_71)) = *const {l802} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 527: b"lglddtrace.c\0":   l802 = UNIQUE | NON_NULL, (empty)
                // 527: b"lglddtrace.c\0": typeof(_71 = const b"lglddtrace.c\x00") = & {l801} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 527: b"lglddtrace.c\0":   l801 = UNIQUE | NON_NULL, (empty)
                189 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                // 529: (*::core::mem:: ... ptr(): typeof(_74) = *const {l106} i8
                // 529: (*::core::mem:: ... ptr():   l106 = UNIQUE | NON_NULL, (empty)
                // 529: (*::core::mem:: ... ptr(): typeof(_75) = & {l108} [i8]
                // 529: (*::core::mem:: ... ptr():   l108 = UNIQUE | NON_NULL, FIXED
                // 529: (*::core::mem:: ... ptr(): typeof(_76) = & {l110} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 529: (*::core::mem:: ... ptr():   l110 = UNIQUE | NON_NULL, (empty)
                // 529: ::core::mem::tr ... 0", ): typeof(_77) = & {l112} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 529: ::core::mem::tr ... 0", ):   l112 = UNIQUE | NON_NULL, FIXED
                // 529: (*::core::mem:: ... ptr(): typeof(_76 = &(*_77)) = & {l807} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 529: (*::core::mem:: ... ptr():   l807 = UNIQUE | NON_NULL, (empty)
                // 529: (*::core::mem:: ... ptr(): typeof(_75 = move _76 as &[i8] (Pointer(Unsize))) = & {l808} [i8]
                // 529: (*::core::mem:: ... ptr():   l808 = UNIQUE | NON_NULL, FIXED
                    b"void process(void)\0",
                    // 530: b"void process( ... d)\0": typeof(_78) = & {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 530: b"void process( ... d)\0":   l114 = UNIQUE | NON_NULL, (empty)
                    // 530: b"void process( ... d)\0": typeof(_79) = & {l116} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 530: b"void process( ... d)\0":   l116 = UNIQUE | NON_NULL, FIXED
                    // 530: b"void process( ... d)\0": typeof(_78 = &(*_79)) = & {l806} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 530: b"void process( ... d)\0":   l806 = UNIQUE | NON_NULL, (empty)
                    // 530: b"void process( ... d)\0": typeof(_79 = const b"void process(void)\x00") = & {l805} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 530: b"void process( ... d)\0":   l805 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    tmp = dup(null);
    if tmp == 2 as libc::c_int {
    } else {
        __assert_fail(
            b"tmp == 2\0" as *const u8 as *const libc::c_char,
            // 540: b"tmp == 2\0" a ... _char: typeof(_88) = *const {l126} i8
            // 540: b"tmp == 2\0" a ... _char:   l126 = UNIQUE | NON_NULL, (empty)
            // 540: b"tmp == 2\0" a ... st u8: typeof(_89) = *const {l128} u8
            // 540: b"tmp == 2\0" a ... st u8:   l128 = UNIQUE | NON_NULL, (empty)
            // 540: b"tmp == 2\0": typeof(_90) = *const {l130} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 540: b"tmp == 2\0":   l130 = UNIQUE | NON_NULL, (empty)
            // 540: b"tmp == 2\0": typeof(_91) = & {l132} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 540: b"tmp == 2\0":   l132 = UNIQUE | NON_NULL, FIXED
            // 540: b"tmp == 2\0" a ... _char: typeof(_88 = move _89 as *const i8 (Misc)) = *const {l812} i8
            // 540: b"tmp == 2\0" a ... _char:   l812 = UNIQUE | NON_NULL, (empty)
            // 540: b"tmp == 2\0" a ... st u8: typeof(_89 = move _90 as *const u8 (Pointer(ArrayToPointer))) = *const {l811} u8
            // 540: b"tmp == 2\0" a ... st u8:   l811 = UNIQUE | NON_NULL, (empty)
            // 540: b"tmp == 2\0": typeof(_90 = &raw const (*_91)) = *const {l810} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 540: b"tmp == 2\0":   l810 = UNIQUE | NON_NULL, (empty)
            // 540: b"tmp == 2\0": typeof(_91 = const b"tmp == 2\x00") = & {l809} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 540: b"tmp == 2\0":   l809 = UNIQUE | NON_NULL, (empty)
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            // 541: b"lglddtrace.c\ ... _char: typeof(_92) = *const {l134} i8
            // 541: b"lglddtrace.c\ ... _char:   l134 = UNIQUE | NON_NULL, (empty)
            // 541: b"lglddtrace.c\ ... st u8: typeof(_93) = *const {l136} u8
            // 541: b"lglddtrace.c\ ... st u8:   l136 = UNIQUE | NON_NULL, (empty)
            // 541: b"lglddtrace.c\0": typeof(_94) = *const {l138} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 541: b"lglddtrace.c\0":   l138 = UNIQUE | NON_NULL, (empty)
            // 541: b"lglddtrace.c\0": typeof(_95) = & {l140} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 541: b"lglddtrace.c\0":   l140 = UNIQUE | NON_NULL, FIXED
            // 541: b"lglddtrace.c\ ... _char: typeof(_92 = move _93 as *const i8 (Misc)) = *const {l816} i8
            // 541: b"lglddtrace.c\ ... _char:   l816 = UNIQUE | NON_NULL, (empty)
            // 541: b"lglddtrace.c\0": typeof(_95 = const b"lglddtrace.c\x00") = & {l813} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 541: b"lglddtrace.c\0":   l813 = UNIQUE | NON_NULL, (empty)
            // 541: b"lglddtrace.c\ ... st u8: typeof(_93 = move _94 as *const u8 (Pointer(ArrayToPointer))) = *const {l815} u8
            // 541: b"lglddtrace.c\ ... st u8:   l815 = UNIQUE | NON_NULL, (empty)
            // 541: b"lglddtrace.c\0": typeof(_94 = &raw const (*_95)) = *const {l814} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 541: b"lglddtrace.c\0":   l814 = UNIQUE | NON_NULL, (empty)
            191 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"void process(void)\0"))
            // 543: (*::core::mem:: ... ptr(): typeof(_98) = *const {l144} i8
            // 543: (*::core::mem:: ... ptr():   l144 = UNIQUE | NON_NULL, (empty)
            // 543: (*::core::mem:: ... ptr(): typeof(_99) = & {l146} [i8]
            // 543: (*::core::mem:: ... ptr():   l146 = UNIQUE | NON_NULL, FIXED
            // 543: (*::core::mem:: ... ptr(): typeof(_100) = & {l148} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: (*::core::mem:: ... ptr():   l148 = UNIQUE | NON_NULL, (empty)
            // 543: ::core::mem::tr ... )\0"): typeof(_101) = & {l150} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: ::core::mem::tr ... )\0"):   l150 = UNIQUE | NON_NULL, FIXED
            // 543: b"void process( ... d)\0": typeof(_102) = & {l152} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: b"void process( ... d)\0":   l152 = UNIQUE | NON_NULL, (empty)
            // 543: b"void process( ... d)\0": typeof(_103) = & {l154} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: b"void process( ... d)\0":   l154 = UNIQUE | NON_NULL, FIXED
            // 543: (*::core::mem:: ... ptr(): typeof(_99 = move _100 as &[i8] (Pointer(Unsize))) = & {l820} [i8]
            // 543: (*::core::mem:: ... ptr():   l820 = UNIQUE | NON_NULL, FIXED
            // 543: (*::core::mem:: ... ptr(): typeof(_100 = &(*_101)) = & {l819} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: (*::core::mem:: ... ptr():   l819 = UNIQUE | NON_NULL, (empty)
            // 543: b"void process( ... d)\0": typeof(_103 = const b"void process(void)\x00") = & {l817} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: b"void process( ... d)\0":   l817 = UNIQUE | NON_NULL, (empty)
            // 543: b"void process( ... d)\0": typeof(_102 = &(*_103)) = & {l818} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 543: b"void process( ... d)\0":   l818 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_5999: {
        if tmp == 2 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                // 551: b"tmp == 2\0" a ... _char: typeof(_110) = *const {l162} i8
                // 551: b"tmp == 2\0" a ... _char:   l162 = UNIQUE | NON_NULL, (empty)
                // 551: b"tmp == 2\0" a ... st u8: typeof(_111) = *const {l164} u8
                // 551: b"tmp == 2\0" a ... st u8:   l164 = UNIQUE | NON_NULL, (empty)
                // 551: b"tmp == 2\0": typeof(_112) = *const {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 551: b"tmp == 2\0":   l166 = UNIQUE | NON_NULL, (empty)
                // 551: b"tmp == 2\0": typeof(_113) = & {l168} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 551: b"tmp == 2\0":   l168 = UNIQUE | NON_NULL, FIXED
                // 551: b"tmp == 2\0" a ... _char: typeof(_110 = move _111 as *const i8 (Misc)) = *const {l824} i8
                // 551: b"tmp == 2\0" a ... _char:   l824 = UNIQUE | NON_NULL, (empty)
                // 551: b"tmp == 2\0": typeof(_112 = &raw const (*_113)) = *const {l822} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 551: b"tmp == 2\0":   l822 = UNIQUE | NON_NULL, (empty)
                // 551: b"tmp == 2\0": typeof(_113 = const b"tmp == 2\x00") = & {l821} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 551: b"tmp == 2\0":   l821 = UNIQUE | NON_NULL, (empty)
                // 551: b"tmp == 2\0" a ... st u8: typeof(_111 = move _112 as *const u8 (Pointer(ArrayToPointer))) = *const {l823} u8
                // 551: b"tmp == 2\0" a ... st u8:   l823 = UNIQUE | NON_NULL, (empty)
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                // 552: b"lglddtrace.c\ ... _char: typeof(_114) = *const {l170} i8
                // 552: b"lglddtrace.c\ ... _char:   l170 = UNIQUE | NON_NULL, (empty)
                // 552: b"lglddtrace.c\ ... st u8: typeof(_115) = *const {l172} u8
                // 552: b"lglddtrace.c\ ... st u8:   l172 = UNIQUE | NON_NULL, (empty)
                // 552: b"lglddtrace.c\0": typeof(_116) = *const {l174} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 552: b"lglddtrace.c\0":   l174 = UNIQUE | NON_NULL, (empty)
                // 552: b"lglddtrace.c\0": typeof(_117) = & {l176} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 552: b"lglddtrace.c\0":   l176 = UNIQUE | NON_NULL, FIXED
                // 552: b"lglddtrace.c\ ... st u8: typeof(_115 = move _116 as *const u8 (Pointer(ArrayToPointer))) = *const {l827} u8
                // 552: b"lglddtrace.c\ ... st u8:   l827 = UNIQUE | NON_NULL, (empty)
                // 552: b"lglddtrace.c\0": typeof(_117 = const b"lglddtrace.c\x00") = & {l825} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 552: b"lglddtrace.c\0":   l825 = UNIQUE | NON_NULL, (empty)
                // 552: b"lglddtrace.c\ ... _char: typeof(_114 = move _115 as *const i8 (Misc)) = *const {l828} i8
                // 552: b"lglddtrace.c\ ... _char:   l828 = UNIQUE | NON_NULL, (empty)
                // 552: b"lglddtrace.c\0": typeof(_116 = &raw const (*_117)) = *const {l826} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 552: b"lglddtrace.c\0":   l826 = UNIQUE | NON_NULL, (empty)
                191 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                // 554: (*::core::mem:: ... ptr(): typeof(_120) = *const {l180} i8
                // 554: (*::core::mem:: ... ptr():   l180 = UNIQUE | NON_NULL, (empty)
                // 554: (*::core::mem:: ... ptr(): typeof(_121) = & {l182} [i8]
                // 554: (*::core::mem:: ... ptr():   l182 = UNIQUE | NON_NULL, FIXED
                // 554: (*::core::mem:: ... ptr(): typeof(_122) = & {l184} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 554: (*::core::mem:: ... ptr():   l184 = UNIQUE | NON_NULL, (empty)
                // 554: ::core::mem::tr ... 0", ): typeof(_123) = & {l186} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 554: ::core::mem::tr ... 0", ):   l186 = UNIQUE | NON_NULL, FIXED
                // 554: (*::core::mem:: ... ptr(): typeof(_121 = move _122 as &[i8] (Pointer(Unsize))) = & {l832} [i8]
                // 554: (*::core::mem:: ... ptr():   l832 = UNIQUE | NON_NULL, FIXED
                // 554: (*::core::mem:: ... ptr(): typeof(_122 = &(*_123)) = & {l831} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 554: (*::core::mem:: ... ptr():   l831 = UNIQUE | NON_NULL, (empty)
                    b"void process(void)\0",
                    // 555: b"void process( ... d)\0": typeof(_124) = & {l188} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 555: b"void process( ... d)\0":   l188 = UNIQUE | NON_NULL, (empty)
                    // 555: b"void process( ... d)\0": typeof(_125) = & {l190} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 555: b"void process( ... d)\0":   l190 = UNIQUE | NON_NULL, FIXED
                    // 555: b"void process( ... d)\0": typeof(_124 = &(*_125)) = & {l830} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 555: b"void process( ... d)\0":   l830 = UNIQUE | NON_NULL, (empty)
                    // 555: b"void process( ... d)\0": typeof(_125 = const b"void process(void)\x00") = & {l829} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 555: b"void process( ... d)\0":   l829 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    lgl = 0 as *mut LGL;
    // 561: lgl = 0 as *mut LGL: typeof(_7 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l833} LGL
    // 561: lgl = 0 as *mut LGL:   l833 = UNIQUE | NON_NULL, (empty)
    res = 0 as libc::c_int;
    e = events;
    // 563: events: typeof(_127) = *mut {l193} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 563: events:   l193 = UNIQUE | NON_NULL, (empty)
    // 563: events: typeof(_128) = *mut {l195} *mut {l196} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 563: events:   l195 = UNIQUE | NON_NULL, (empty)
    // 563: events:   l196 = UNIQUE | NON_NULL, (empty)
    while e < events.offset(nevents as isize) {
    // 564: e: typeof(_132) = *mut {l201} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 564: e:   l201 = UNIQUE | NON_NULL, (empty)
    // 564: events.offset(n ... size): typeof(_133) = *mut {l203} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 564: events.offset(n ... size):   l203 = UNIQUE | NON_NULL, (empty)
    // 564: events: typeof(_134) = *mut {l205} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 564: events:   l205 = UNIQUE | NON_NULL, (empty)
    // 564: events: typeof(_135) = *mut {l207} *mut {l208} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 564: events:   l207 = UNIQUE | NON_NULL, (empty)
    // 564: events:   l208 = UNIQUE | NON_NULL, (empty)
    // 564: nevents: typeof(_138) = *mut {l212} i32
    // 564: nevents:   l212 = UNIQUE | NON_NULL, (empty)
        if !(reme_shim(e) != 0) {
        // 565: e: typeof(_143) = *mut {l218} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 565: e:   l218 = UNIQUE | NON_NULL, (empty)
            match (*e).type_0 as libc::c_uint {
                0 => {
                    lgladd(lgl, (*e).arg);
                    // 568: lgl: typeof(_146) = *mut {l222} LGL
                    // 568: lgl:   l222 = UNIQUE | NON_NULL, (empty)
                }
                1 => {
                    lglassume(lgl, (*e).arg);
                    // 571: lgl: typeof(_149) = *mut {l226} LGL
                    // 571: lgl:   l226 = UNIQUE | NON_NULL, (empty)
                }
                28 => {
                    lglchkclone(lgl);
                    // 574: lgl: typeof(_152) = *mut {l230} LGL
                    // 574: lgl:   l230 = UNIQUE | NON_NULL, (empty)
                }
                2 => {
                    res = lglderef(lgl, (*e).arg);
                    // 577: lgl: typeof(_154) = *mut {l233} LGL
                    // 577: lgl:   l233 = UNIQUE | NON_NULL, (empty)
                }
                26 => {
                    res = lglfixed(lgl, (*e).arg);
                    // 580: lgl: typeof(_157) = *mut {l237} LGL
                    // 580: lgl:   l237 = UNIQUE | NON_NULL, (empty)
                }
                21 => {
                    res = lglfrozen(lgl, (*e).arg);
                    // 583: lgl: typeof(_160) = *mut {l241} LGL
                    // 583: lgl:   l241 = UNIQUE | NON_NULL, (empty)
                }
                23 => {
                    res = lglreusable(lgl, (*e).arg);
                    // 586: lgl: typeof(_163) = *mut {l245} LGL
                    // 586: lgl:   l245 = UNIQUE | NON_NULL, (empty)
                }
                22 => {
                    res = lglusable(lgl, (*e).arg);
                    // 589: lgl: typeof(_166) = *mut {l249} LGL
                    // 589: lgl:   l249 = UNIQUE | NON_NULL, (empty)
                }
                14 => {
                    res = lglrepr(lgl, (*e).arg);
                    // 592: lgl: typeof(_169) = *mut {l253} LGL
                    // 592: lgl:   l253 = UNIQUE | NON_NULL, (empty)
                }
                3 => {
                    res = lglfailed(lgl, (*e).arg);
                    // 595: lgl: typeof(_172) = *mut {l257} LGL
                    // 595: lgl:   l257 = UNIQUE | NON_NULL, (empty)
                }
                27 => {
                    lglfixate(lgl);
                    // 598: lgl: typeof(_175) = *mut {l261} LGL
                    // 598: lgl:   l261 = UNIQUE | NON_NULL, (empty)
                }
                20 => {
                    lglreducecache(lgl);
                    // 601: lgl: typeof(_177) = *mut {l264} LGL
                    // 601: lgl:   l264 = UNIQUE | NON_NULL, (empty)
                }
                19 => {
                    lglflushcache(lgl);
                    // 604: lgl: typeof(_179) = *mut {l267} LGL
                    // 604: lgl:   l267 = UNIQUE | NON_NULL, (empty)
                }
                15 => {
                    lglsetimportant(lgl, (*e).arg);
                    // 607: lgl: typeof(_181) = *mut {l270} LGL
                    // 607: lgl:   l270 = UNIQUE | NON_NULL, (empty)
                }
                18 => {
                    lglsetphases(lgl);
                    // 610: lgl: typeof(_184) = *mut {l274} LGL
                    // 610: lgl:   l274 = UNIQUE | NON_NULL, (empty)
                }
                16 => {
                    lglsetphase(lgl, (*e).arg);
                    // 613: lgl: typeof(_186) = *mut {l277} LGL
                    // 613: lgl:   l277 = UNIQUE | NON_NULL, (empty)
                }
                17 => {
                    lglresetphase(lgl, (*e).arg);
                    // 616: lgl: typeof(_189) = *mut {l281} LGL
                    // 616: lgl:   l281 = UNIQUE | NON_NULL, (empty)
                }
                4 => {
                    lglfreeze(lgl, (*e).arg);
                    // 619: lgl: typeof(_192) = *mut {l285} LGL
                    // 619: lgl:   l285 = UNIQUE | NON_NULL, (empty)
                }
                30 => {
                    res = lglinconsistent(lgl);
                    // 622: lgl: typeof(_195) = *mut {l289} LGL
                    // 622: lgl:   l289 = UNIQUE | NON_NULL, (empty)
                }
                31 => {
                    res = lglookahead(lgl);
                    // 625: lgl: typeof(_197) = *mut {l292} LGL
                    // 625: lgl:   l292 = UNIQUE | NON_NULL, (empty)
                }
                25 => {
                    res = lglincvar(lgl);
                    // 628: lgl: typeof(_199) = *mut {l295} LGL
                    // 628: lgl:   l295 = UNIQUE | NON_NULL, (empty)
                }
                24 => {
                    res = lglincvar(lgl);
                    // 631: lgl: typeof(_201) = *mut {l298} LGL
                    // 631: lgl:   l298 = UNIQUE | NON_NULL, (empty)
                }
                29 => {
                    res = lglchanged(lgl);
                    // 634: lgl: typeof(_203) = *mut {l301} LGL
                    // 634: lgl:   l301 = UNIQUE | NON_NULL, (empty)
                }
                5 => {
                    lgl = lglinit();
                    // 637: lglinit(): typeof(_204) = *mut {l303} LGL
                    // 637: lglinit():   l303 = UNIQUE | NON_NULL, (empty)
                    lglonabort(
                        lgl,
                        // 639: lgl: typeof(_206) = *mut {l306} LGL
                        // 639: lgl:   l306 = UNIQUE | NON_NULL, (empty)
                        0 as *mut libc::c_void,
                        // 640: 0 as *mut libc: ... _void: typeof(_207) = *mut {l308} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 640: 0 as *mut libc: ... _void:   l308 = UNIQUE | NON_NULL, (empty)
                        // 640: 0 as *mut libc: ... _void: typeof(_207 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l834} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 640: 0 as *mut libc: ... _void:   l834 = UNIQUE | NON_NULL, (empty)
                        Some(onabort_shim as unsafe extern "C" fn(*mut libc::c_void) -> ()),
                        // 641: Some(onabort as ... > ()): typeof(_208) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l310} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
                        // 641: Some(onabort as ... > ()):   l310 = UNIQUE | NON_NULL, (empty)
                        // 641: onabort as unsa ... -> (): typeof(_209) = fn(*mut {l312} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
                        // 641: onabort as unsa ... -> ():   l312 = UNIQUE | NON_NULL, (empty)
                        // 641: onabort: typeof(_209 = onabort as unsafe extern "C" fn(*mut libc::c_void) (Pointer(ReifyFnPointer))) = fn(*mut {l835} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
                        // 641: onabort:   l835 = UNIQUE | NON_NULL, (empty)
                        // 641: Some(onabort as ... > ()): typeof(_208 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void)>::Some(move _209)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l836} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
                        // 641: Some(onabort as ... > ()):   l836 = UNIQUE | NON_NULL, (empty)
                    );
                    if !opts.is_null() {
                    // 643: opts: typeof(_212) = *mut {l316} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 643: opts:   l316 = UNIQUE | NON_NULL, (empty)
                    // 643: opts: typeof(_213) = *mut {l318} *mut {l319} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 643: opts:   l318 = UNIQUE | NON_NULL, (empty)
                    // 643: opts:   l319 = UNIQUE | NON_NULL, (empty)
                        if ddopts != 0 {
                        // 644: ddopts: typeof(_217) = *mut {l324} i32
                        // 644: ddopts:   l324 = UNIQUE | NON_NULL, (empty)
                        } else {
                            __assert_fail(
                                b"ddopts\0" as *const u8 as *const libc::c_char,
                                // 647: b"ddopts\0" as  ... _char: typeof(_220) = *const {l328} i8
                                // 647: b"ddopts\0" as  ... _char:   l328 = UNIQUE | NON_NULL, (empty)
                                // 647: b"ddopts\0" as  ... st u8: typeof(_221) = *const {l330} u8
                                // 647: b"ddopts\0" as  ... st u8:   l330 = UNIQUE | NON_NULL, (empty)
                                // 647: b"ddopts\0": typeof(_222) = *const {l332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                // 647: b"ddopts\0":   l332 = UNIQUE | NON_NULL, (empty)
                                // 647: b"ddopts\0": typeof(_223) = & {l334} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                // 647: b"ddopts\0":   l334 = UNIQUE | NON_NULL, FIXED
                                // 647: b"ddopts\0" as  ... st u8: typeof(_221 = move _222 as *const u8 (Pointer(ArrayToPointer))) = *const {l839} u8
                                // 647: b"ddopts\0" as  ... st u8:   l839 = UNIQUE | NON_NULL, (empty)
                                // 647: b"ddopts\0": typeof(_222 = &raw const (*_223)) = *const {l838} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                // 647: b"ddopts\0":   l838 = UNIQUE | NON_NULL, (empty)
                                // 647: b"ddopts\0" as  ... _char: typeof(_220 = move _221 as *const i8 (Misc)) = *const {l840} i8
                                // 647: b"ddopts\0" as  ... _char:   l840 = UNIQUE | NON_NULL, (empty)
                                // 647: b"ddopts\0": typeof(_223 = const b"ddopts\x00") = & {l837} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                // 647: b"ddopts\0":   l837 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 648: b"lglddtrace.c\ ... _char: typeof(_224) = *const {l336} i8
                                // 648: b"lglddtrace.c\ ... _char:   l336 = UNIQUE | NON_NULL, (empty)
                                // 648: b"lglddtrace.c\ ... st u8: typeof(_225) = *const {l338} u8
                                // 648: b"lglddtrace.c\ ... st u8:   l338 = UNIQUE | NON_NULL, (empty)
                                // 648: b"lglddtrace.c\0": typeof(_226) = *const {l340} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 648: b"lglddtrace.c\0":   l340 = UNIQUE | NON_NULL, (empty)
                                // 648: b"lglddtrace.c\0": typeof(_227) = & {l342} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 648: b"lglddtrace.c\0":   l342 = UNIQUE | NON_NULL, FIXED
                                // 648: b"lglddtrace.c\0": typeof(_227 = const b"lglddtrace.c\x00") = & {l841} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 648: b"lglddtrace.c\0":   l841 = UNIQUE | NON_NULL, (empty)
                                // 648: b"lglddtrace.c\ ... _char: typeof(_224 = move _225 as *const i8 (Misc)) = *const {l844} i8
                                // 648: b"lglddtrace.c\ ... _char:   l844 = UNIQUE | NON_NULL, (empty)
                                // 648: b"lglddtrace.c\0": typeof(_226 = &raw const (*_227)) = *const {l842} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 648: b"lglddtrace.c\0":   l842 = UNIQUE | NON_NULL, (empty)
                                // 648: b"lglddtrace.c\ ... st u8: typeof(_225 = move _226 as *const u8 (Pointer(ArrayToPointer))) = *const {l843} u8
                                // 648: b"lglddtrace.c\ ... st u8:   l843 = UNIQUE | NON_NULL, (empty)
                                224 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                                // 650: (*::core::mem:: ... ptr(): typeof(_230) = *const {l346} i8
                                // 650: (*::core::mem:: ... ptr():   l346 = UNIQUE | NON_NULL, (empty)
                                // 650: (*::core::mem:: ... ptr(): typeof(_231) = & {l348} [i8]
                                // 650: (*::core::mem:: ... ptr():   l348 = UNIQUE | NON_NULL, FIXED
                                // 650: (*::core::mem:: ... ptr(): typeof(_232) = & {l350} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 650: (*::core::mem:: ... ptr():   l350 = UNIQUE | NON_NULL, (empty)
                                // 650: ::core::mem::tr ... 0", ): typeof(_233) = & {l352} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 650: ::core::mem::tr ... 0", ):   l352 = UNIQUE | NON_NULL, FIXED
                                // 650: (*::core::mem:: ... ptr(): typeof(_231 = move _232 as &[i8] (Pointer(Unsize))) = & {l848} [i8]
                                // 650: (*::core::mem:: ... ptr():   l848 = UNIQUE | NON_NULL, FIXED
                                // 650: (*::core::mem:: ... ptr(): typeof(_232 = &(*_233)) = & {l847} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 650: (*::core::mem:: ... ptr():   l847 = UNIQUE | NON_NULL, (empty)
                                    b"void process(void)\0",
                                    // 651: b"void process( ... d)\0": typeof(_234) = & {l354} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 651: b"void process( ... d)\0":   l354 = UNIQUE | NON_NULL, (empty)
                                    // 651: b"void process( ... d)\0": typeof(_235) = & {l356} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 651: b"void process( ... d)\0":   l356 = UNIQUE | NON_NULL, FIXED
                                    // 651: b"void process( ... d)\0": typeof(_235 = const b"void process(void)\x00") = & {l845} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 651: b"void process( ... d)\0":   l845 = UNIQUE | NON_NULL, (empty)
                                    // 651: b"void process( ... d)\0": typeof(_234 = &(*_235)) = & {l846} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 651: b"void process( ... d)\0":   l846 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                        'c_5631: {
                            if ddopts != 0 {
                            // 657: ddopts: typeof(_239) = *mut {l361} i32
                            // 657: ddopts:   l361 = UNIQUE | NON_NULL, (empty)
                            } else {
                                __assert_fail(
                                    b"ddopts\0" as *const u8 as *const libc::c_char,
                                    // 660: b"ddopts\0" as  ... _char: typeof(_242) = *const {l365} i8
                                    // 660: b"ddopts\0" as  ... _char:   l365 = UNIQUE | NON_NULL, (empty)
                                    // 660: b"ddopts\0" as  ... st u8: typeof(_243) = *const {l367} u8
                                    // 660: b"ddopts\0" as  ... st u8:   l367 = UNIQUE | NON_NULL, (empty)
                                    // 660: b"ddopts\0": typeof(_244) = *const {l369} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                    // 660: b"ddopts\0":   l369 = UNIQUE | NON_NULL, (empty)
                                    // 660: b"ddopts\0": typeof(_245) = & {l371} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                    // 660: b"ddopts\0":   l371 = UNIQUE | NON_NULL, FIXED
                                    // 660: b"ddopts\0": typeof(_244 = &raw const (*_245)) = *const {l850} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                    // 660: b"ddopts\0":   l850 = UNIQUE | NON_NULL, (empty)
                                    // 660: b"ddopts\0" as  ... _char: typeof(_242 = move _243 as *const i8 (Misc)) = *const {l852} i8
                                    // 660: b"ddopts\0" as  ... _char:   l852 = UNIQUE | NON_NULL, (empty)
                                    // 660: b"ddopts\0": typeof(_245 = const b"ddopts\x00") = & {l849} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                                    // 660: b"ddopts\0":   l849 = UNIQUE | NON_NULL, (empty)
                                    // 660: b"ddopts\0" as  ... st u8: typeof(_243 = move _244 as *const u8 (Pointer(ArrayToPointer))) = *const {l851} u8
                                    // 660: b"ddopts\0" as  ... st u8:   l851 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 661: b"lglddtrace.c\ ... _char: typeof(_246) = *const {l373} i8
                                    // 661: b"lglddtrace.c\ ... _char:   l373 = UNIQUE | NON_NULL, (empty)
                                    // 661: b"lglddtrace.c\ ... st u8: typeof(_247) = *const {l375} u8
                                    // 661: b"lglddtrace.c\ ... st u8:   l375 = UNIQUE | NON_NULL, (empty)
                                    // 661: b"lglddtrace.c\0": typeof(_248) = *const {l377} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 661: b"lglddtrace.c\0":   l377 = UNIQUE | NON_NULL, (empty)
                                    // 661: b"lglddtrace.c\0": typeof(_249) = & {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 661: b"lglddtrace.c\0":   l379 = UNIQUE | NON_NULL, FIXED
                                    // 661: b"lglddtrace.c\0": typeof(_249 = const b"lglddtrace.c\x00") = & {l853} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 661: b"lglddtrace.c\0":   l853 = UNIQUE | NON_NULL, (empty)
                                    // 661: b"lglddtrace.c\ ... st u8: typeof(_247 = move _248 as *const u8 (Pointer(ArrayToPointer))) = *const {l855} u8
                                    // 661: b"lglddtrace.c\ ... st u8:   l855 = UNIQUE | NON_NULL, (empty)
                                    // 661: b"lglddtrace.c\0": typeof(_248 = &raw const (*_249)) = *const {l854} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 661: b"lglddtrace.c\0":   l854 = UNIQUE | NON_NULL, (empty)
                                    // 661: b"lglddtrace.c\ ... _char: typeof(_246 = move _247 as *const i8 (Misc)) = *const {l856} i8
                                    // 661: b"lglddtrace.c\ ... _char:   l856 = UNIQUE | NON_NULL, (empty)
                                    224 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                                    // 663: (*::core::mem:: ... ptr(): typeof(_252) = *const {l383} i8
                                    // 663: (*::core::mem:: ... ptr():   l383 = UNIQUE | NON_NULL, (empty)
                                    // 663: (*::core::mem:: ... ptr(): typeof(_253) = & {l385} [i8]
                                    // 663: (*::core::mem:: ... ptr():   l385 = UNIQUE | NON_NULL, FIXED
                                    // 663: (*::core::mem:: ... ptr(): typeof(_254) = & {l387} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 663: (*::core::mem:: ... ptr():   l387 = UNIQUE | NON_NULL, (empty)
                                    // 663: ::core::mem::tr ... 0", ): typeof(_255) = & {l389} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 663: ::core::mem::tr ... 0", ):   l389 = UNIQUE | NON_NULL, FIXED
                                    // 663: (*::core::mem:: ... ptr(): typeof(_253 = move _254 as &[i8] (Pointer(Unsize))) = & {l860} [i8]
                                    // 663: (*::core::mem:: ... ptr():   l860 = UNIQUE | NON_NULL, FIXED
                                    // 663: (*::core::mem:: ... ptr(): typeof(_254 = &(*_255)) = & {l859} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 663: (*::core::mem:: ... ptr():   l859 = UNIQUE | NON_NULL, (empty)
                                        b"void process(void)\0",
                                        // 664: b"void process( ... d)\0": typeof(_256) = & {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 664: b"void process( ... d)\0":   l391 = UNIQUE | NON_NULL, (empty)
                                        // 664: b"void process( ... d)\0": typeof(_257) = & {l393} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 664: b"void process( ... d)\0":   l393 = UNIQUE | NON_NULL, FIXED
                                        // 664: b"void process( ... d)\0": typeof(_257 = const b"void process(void)\x00") = & {l857} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 664: b"void process( ... d)\0":   l857 = UNIQUE | NON_NULL, (empty)
                                        // 664: b"void process( ... d)\0": typeof(_256 = &(*_257)) = & {l858} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 664: b"void process( ... d)\0":   l858 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                        };
                        o = opts;
                        // 670: opts: typeof(_258) = *mut {l395} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 670: opts:   l395 = UNIQUE | NON_NULL, (empty)
                        // 670: opts: typeof(_259) = *mut {l397} *mut {l398} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 670: opts:   l397 = UNIQUE | NON_NULL, (empty)
                        // 670: opts:   l398 = UNIQUE | NON_NULL, (empty)
                        while o < opts.offset(nopts as isize) {
                        // 671: o: typeof(_261) = *mut {l401} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 671: o:   l401 = UNIQUE | NON_NULL, (empty)
                        // 671: opts.offset(nop ... size): typeof(_262) = *mut {l403} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 671: opts.offset(nop ... size):   l403 = UNIQUE | NON_NULL, (empty)
                        // 671: opts: typeof(_263) = *mut {l405} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 671: opts:   l405 = UNIQUE | NON_NULL, (empty)
                        // 671: opts: typeof(_264) = *mut {l407} *mut {l408} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 671: opts:   l407 = UNIQUE | NON_NULL, (empty)
                        // 671: opts:   l408 = UNIQUE | NON_NULL, (empty)
                        // 671: nopts: typeof(_267) = *mut {l412} i32
                        // 671: nopts:   l412 = UNIQUE | NON_NULL, (empty)
                            lglsetopt(lgl, (*o).name, (*o).val);
                            // 672: lgl: typeof(_269) = *mut {l415} LGL
                            // 672: lgl:   l415 = UNIQUE | NON_NULL, (empty)
                            // 672: (*o).name: typeof(_270) = *const {l417} i8
                            // 672: (*o).name:   l417 = UNIQUE | NON_NULL, (empty)
                            // 672: (*o).name: typeof(_271) = *mut {l419} i8
                            // 672: (*o).name:   l419 = UNIQUE | NON_NULL, (empty)
                            // 672: (*o).name: typeof(_270 = move _271 as *const i8 (Pointer(MutToConstPointer))) = *const {l861} i8
                            // 672: (*o).name:   l861 = UNIQUE | NON_NULL, (empty)
                            o = o.offset(1);
                            // 673: o.offset(1): typeof(_273) = *mut {l422} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 673: o.offset(1):   l422 = UNIQUE | NON_NULL, (empty)
                            // 673: o: typeof(_274) = *mut {l424} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 673: o:   l424 = UNIQUE | NON_NULL, (empty)
                            o;
                            // 674: o: typeof(_275) = *mut {l426} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 674: o:   l426 = UNIQUE | NON_NULL, (empty)
                        }
                    }
                }
                6 => {
                    lglmelt(lgl, (*e).arg);
                    // 679: lgl: typeof(_280) = *mut {l432} LGL
                    // 679: lgl:   l432 = UNIQUE | NON_NULL, (empty)
                }
                7 => {
                    lglreuse(lgl, (*e).arg);
                    // 682: lgl: typeof(_283) = *mut {l436} LGL
                    // 682: lgl:   l436 = UNIQUE | NON_NULL, (empty)
                }
                8 => {
                    if opts.is_null() {
                    // 685: opts: typeof(_286) = *mut {l440} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 685: opts:   l440 = UNIQUE | NON_NULL, (empty)
                    // 685: opts: typeof(_287) = *mut {l442} *mut {l443} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 685: opts:   l442 = UNIQUE | NON_NULL, (empty)
                    // 685: opts:   l443 = UNIQUE | NON_NULL, (empty)
                        lglsetopt(lgl, (*e).opt, (*e).arg);
                        // 686: lgl: typeof(_289) = *mut {l446} LGL
                        // 686: lgl:   l446 = UNIQUE | NON_NULL, (empty)
                        // 686: (*e).opt: typeof(_290) = *const {l448} i8
                        // 686: (*e).opt:   l448 = UNIQUE | NON_NULL, (empty)
                        // 686: (*e).opt: typeof(_291) = *mut {l450} i8
                        // 686: (*e).opt:   l450 = UNIQUE | NON_NULL, (empty)
                        // 686: (*e).opt: typeof(_290 = move _291 as *const i8 (Pointer(MutToConstPointer))) = *const {l862} i8
                        // 686: (*e).opt:   l862 = UNIQUE | NON_NULL, (empty)
                    }
                }
                13 => {
                    res = lglsimp(lgl, (*e).arg);
                    // 690: lgl: typeof(_294) = *mut {l454} LGL
                    // 690: lgl:   l454 = UNIQUE | NON_NULL, (empty)
                }
                10 => {
                    lglrelease(lgl);
                    // 693: lgl: typeof(_297) = *mut {l458} LGL
                    // 693: lgl:   l458 = UNIQUE | NON_NULL, (empty)
                }
                11 => {
                    if checkreturn != 0 {
                    // 696: checkreturn: typeof(_300) = *mut {l462} i32
                    // 696: checkreturn:   l462 = UNIQUE | NON_NULL, (empty)
                        if (*e).arg == res {
                        } else {
                            __assert_fail(
                                b"e->arg == res\0" as *const u8 as *const libc::c_char,
                                // 700: b"e->arg == res ... _char: typeof(_307) = *const {l470} i8
                                // 700: b"e->arg == res ... _char:   l470 = UNIQUE | NON_NULL, (empty)
                                // 700: b"e->arg == res ... st u8: typeof(_308) = *const {l472} u8
                                // 700: b"e->arg == res ... st u8:   l472 = UNIQUE | NON_NULL, (empty)
                                // 700: b"e->arg == res\0": typeof(_309) = *const {l474} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 700: b"e->arg == res\0":   l474 = UNIQUE | NON_NULL, (empty)
                                // 700: b"e->arg == res\0": typeof(_310) = & {l476} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 700: b"e->arg == res\0":   l476 = UNIQUE | NON_NULL, FIXED
                                // 700: b"e->arg == res\0": typeof(_310 = const b"e->arg == res\x00") = & {l863} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 700: b"e->arg == res\0":   l863 = UNIQUE | NON_NULL, (empty)
                                // 700: b"e->arg == res ... st u8: typeof(_308 = move _309 as *const u8 (Pointer(ArrayToPointer))) = *const {l865} u8
                                // 700: b"e->arg == res ... st u8:   l865 = UNIQUE | NON_NULL, (empty)
                                // 700: b"e->arg == res\0": typeof(_309 = &raw const (*_310)) = *const {l864} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 700: b"e->arg == res\0":   l864 = UNIQUE | NON_NULL, (empty)
                                // 700: b"e->arg == res ... _char: typeof(_307 = move _308 as *const i8 (Misc)) = *const {l866} i8
                                // 700: b"e->arg == res ... _char:   l866 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 701: b"lglddtrace.c\ ... _char: typeof(_311) = *const {l478} i8
                                // 701: b"lglddtrace.c\ ... _char:   l478 = UNIQUE | NON_NULL, (empty)
                                // 701: b"lglddtrace.c\ ... st u8: typeof(_312) = *const {l480} u8
                                // 701: b"lglddtrace.c\ ... st u8:   l480 = UNIQUE | NON_NULL, (empty)
                                // 701: b"lglddtrace.c\0": typeof(_313) = *const {l482} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 701: b"lglddtrace.c\0":   l482 = UNIQUE | NON_NULL, (empty)
                                // 701: b"lglddtrace.c\0": typeof(_314) = & {l484} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 701: b"lglddtrace.c\0":   l484 = UNIQUE | NON_NULL, FIXED
                                // 701: b"lglddtrace.c\ ... st u8: typeof(_312 = move _313 as *const u8 (Pointer(ArrayToPointer))) = *const {l869} u8
                                // 701: b"lglddtrace.c\ ... st u8:   l869 = UNIQUE | NON_NULL, (empty)
                                // 701: b"lglddtrace.c\ ... _char: typeof(_311 = move _312 as *const i8 (Misc)) = *const {l870} i8
                                // 701: b"lglddtrace.c\ ... _char:   l870 = UNIQUE | NON_NULL, (empty)
                                // 701: b"lglddtrace.c\0": typeof(_314 = const b"lglddtrace.c\x00") = & {l867} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 701: b"lglddtrace.c\0":   l867 = UNIQUE | NON_NULL, (empty)
                                // 701: b"lglddtrace.c\0": typeof(_313 = &raw const (*_314)) = *const {l868} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 701: b"lglddtrace.c\0":   l868 = UNIQUE | NON_NULL, (empty)
                                235 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                                // 703: (*::core::mem:: ... ptr(): typeof(_317) = *const {l488} i8
                                // 703: (*::core::mem:: ... ptr():   l488 = UNIQUE | NON_NULL, (empty)
                                // 703: (*::core::mem:: ... ptr(): typeof(_318) = & {l490} [i8]
                                // 703: (*::core::mem:: ... ptr():   l490 = UNIQUE | NON_NULL, FIXED
                                // 703: (*::core::mem:: ... ptr(): typeof(_319) = & {l492} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 703: (*::core::mem:: ... ptr():   l492 = UNIQUE | NON_NULL, (empty)
                                // 703: ::core::mem::tr ... 0", ): typeof(_320) = & {l494} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 703: ::core::mem::tr ... 0", ):   l494 = UNIQUE | NON_NULL, FIXED
                                // 703: (*::core::mem:: ... ptr(): typeof(_319 = &(*_320)) = & {l873} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 703: (*::core::mem:: ... ptr():   l873 = UNIQUE | NON_NULL, (empty)
                                // 703: (*::core::mem:: ... ptr(): typeof(_318 = move _319 as &[i8] (Pointer(Unsize))) = & {l874} [i8]
                                // 703: (*::core::mem:: ... ptr():   l874 = UNIQUE | NON_NULL, FIXED
                                    b"void process(void)\0",
                                    // 704: b"void process( ... d)\0": typeof(_321) = & {l496} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 704: b"void process( ... d)\0":   l496 = UNIQUE | NON_NULL, (empty)
                                    // 704: b"void process( ... d)\0": typeof(_322) = & {l498} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 704: b"void process( ... d)\0":   l498 = UNIQUE | NON_NULL, FIXED
                                    // 704: b"void process( ... d)\0": typeof(_322 = const b"void process(void)\x00") = & {l871} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 704: b"void process( ... d)\0":   l871 = UNIQUE | NON_NULL, (empty)
                                    // 704: b"void process( ... d)\0": typeof(_321 = &(*_322)) = & {l872} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 704: b"void process( ... d)\0":   l872 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                        'c_5474: {
                            if (*e).arg == res {
                            } else {
                                __assert_fail(
                                    b"e->arg == res\0" as *const u8 as *const libc::c_char,
                                    // 713: b"e->arg == res ... _char: typeof(_329) = *const {l506} i8
                                    // 713: b"e->arg == res ... _char:   l506 = UNIQUE | NON_NULL, (empty)
                                    // 713: b"e->arg == res ... st u8: typeof(_330) = *const {l508} u8
                                    // 713: b"e->arg == res ... st u8:   l508 = UNIQUE | NON_NULL, (empty)
                                    // 713: b"e->arg == res\0": typeof(_331) = *const {l510} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 713: b"e->arg == res\0":   l510 = UNIQUE | NON_NULL, (empty)
                                    // 713: b"e->arg == res\0": typeof(_332) = & {l512} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 713: b"e->arg == res\0":   l512 = UNIQUE | NON_NULL, FIXED
                                    // 713: b"e->arg == res ... st u8: typeof(_330 = move _331 as *const u8 (Pointer(ArrayToPointer))) = *const {l877} u8
                                    // 713: b"e->arg == res ... st u8:   l877 = UNIQUE | NON_NULL, (empty)
                                    // 713: b"e->arg == res\0": typeof(_331 = &raw const (*_332)) = *const {l876} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 713: b"e->arg == res\0":   l876 = UNIQUE | NON_NULL, (empty)
                                    // 713: b"e->arg == res ... _char: typeof(_329 = move _330 as *const i8 (Misc)) = *const {l878} i8
                                    // 713: b"e->arg == res ... _char:   l878 = UNIQUE | NON_NULL, (empty)
                                    // 713: b"e->arg == res\0": typeof(_332 = const b"e->arg == res\x00") = & {l875} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 713: b"e->arg == res\0":   l875 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 714: b"lglddtrace.c\ ... _char: typeof(_333) = *const {l514} i8
                                    // 714: b"lglddtrace.c\ ... _char:   l514 = UNIQUE | NON_NULL, (empty)
                                    // 714: b"lglddtrace.c\ ... st u8: typeof(_334) = *const {l516} u8
                                    // 714: b"lglddtrace.c\ ... st u8:   l516 = UNIQUE | NON_NULL, (empty)
                                    // 714: b"lglddtrace.c\0": typeof(_335) = *const {l518} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 714: b"lglddtrace.c\0":   l518 = UNIQUE | NON_NULL, (empty)
                                    // 714: b"lglddtrace.c\0": typeof(_336) = & {l520} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 714: b"lglddtrace.c\0":   l520 = UNIQUE | NON_NULL, FIXED
                                    // 714: b"lglddtrace.c\ ... _char: typeof(_333 = move _334 as *const i8 (Misc)) = *const {l882} i8
                                    // 714: b"lglddtrace.c\ ... _char:   l882 = UNIQUE | NON_NULL, (empty)
                                    // 714: b"lglddtrace.c\0": typeof(_335 = &raw const (*_336)) = *const {l880} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 714: b"lglddtrace.c\0":   l880 = UNIQUE | NON_NULL, (empty)
                                    // 714: b"lglddtrace.c\0": typeof(_336 = const b"lglddtrace.c\x00") = & {l879} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 714: b"lglddtrace.c\0":   l879 = UNIQUE | NON_NULL, (empty)
                                    // 714: b"lglddtrace.c\ ... st u8: typeof(_334 = move _335 as *const u8 (Pointer(ArrayToPointer))) = *const {l881} u8
                                    // 714: b"lglddtrace.c\ ... st u8:   l881 = UNIQUE | NON_NULL, (empty)
                                    235 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                                    // 716: (*::core::mem:: ... ptr(): typeof(_339) = *const {l524} i8
                                    // 716: (*::core::mem:: ... ptr():   l524 = UNIQUE | NON_NULL, (empty)
                                    // 716: (*::core::mem:: ... ptr(): typeof(_340) = & {l526} [i8]
                                    // 716: (*::core::mem:: ... ptr():   l526 = UNIQUE | NON_NULL, FIXED
                                    // 716: (*::core::mem:: ... ptr(): typeof(_341) = & {l528} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 716: (*::core::mem:: ... ptr():   l528 = UNIQUE | NON_NULL, (empty)
                                    // 716: ::core::mem::tr ... 0", ): typeof(_342) = & {l530} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 716: ::core::mem::tr ... 0", ):   l530 = UNIQUE | NON_NULL, FIXED
                                    // 716: (*::core::mem:: ... ptr(): typeof(_340 = move _341 as &[i8] (Pointer(Unsize))) = & {l886} [i8]
                                    // 716: (*::core::mem:: ... ptr():   l886 = UNIQUE | NON_NULL, FIXED
                                    // 716: (*::core::mem:: ... ptr(): typeof(_341 = &(*_342)) = & {l885} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 716: (*::core::mem:: ... ptr():   l885 = UNIQUE | NON_NULL, (empty)
                                        b"void process(void)\0",
                                        // 717: b"void process( ... d)\0": typeof(_343) = & {l532} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 717: b"void process( ... d)\0":   l532 = UNIQUE | NON_NULL, (empty)
                                        // 717: b"void process( ... d)\0": typeof(_344) = & {l534} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 717: b"void process( ... d)\0":   l534 = UNIQUE | NON_NULL, FIXED
                                        // 717: b"void process( ... d)\0": typeof(_344 = const b"void process(void)\x00") = & {l883} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 717: b"void process( ... d)\0":   l883 = UNIQUE | NON_NULL, (empty)
                                        // 717: b"void process( ... d)\0": typeof(_343 = &(*_344)) = & {l884} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                        // 717: b"void process( ... d)\0":   l884 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                        };
                    }
                }
                12 | _ => {
                    if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {
                    } else {
                        __assert_fail(
                            b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                            // 729: b"e->type == SA ... _char: typeof(_352) = *const {l543} i8
                            // 729: b"e->type == SA ... _char:   l543 = UNIQUE | NON_NULL, (empty)
                            // 729: b"e->type == SA ... st u8: typeof(_353) = *const {l545} u8
                            // 729: b"e->type == SA ... st u8:   l545 = UNIQUE | NON_NULL, (empty)
                            // 729: b"e->type == SAT\0": typeof(_354) = *const {l547} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 729: b"e->type == SAT\0":   l547 = UNIQUE | NON_NULL, (empty)
                            // 729: b"e->type == SAT\0": typeof(_355) = & {l549} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 729: b"e->type == SAT\0":   l549 = UNIQUE | NON_NULL, FIXED
                            // 729: b"e->type == SA ... _char: typeof(_352 = move _353 as *const i8 (Misc)) = *const {l890} i8
                            // 729: b"e->type == SA ... _char:   l890 = UNIQUE | NON_NULL, (empty)
                            // 729: b"e->type == SA ... st u8: typeof(_353 = move _354 as *const u8 (Pointer(ArrayToPointer))) = *const {l889} u8
                            // 729: b"e->type == SA ... st u8:   l889 = UNIQUE | NON_NULL, (empty)
                            // 729: b"e->type == SAT\0": typeof(_354 = &raw const (*_355)) = *const {l888} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 729: b"e->type == SAT\0":   l888 = UNIQUE | NON_NULL, (empty)
                            // 729: b"e->type == SAT\0": typeof(_355 = const b"e->type == SAT\x00") = & {l887} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 729: b"e->type == SAT\0":   l887 = UNIQUE | NON_NULL, (empty)
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            // 730: b"lglddtrace.c\ ... _char: typeof(_356) = *const {l551} i8
                            // 730: b"lglddtrace.c\ ... _char:   l551 = UNIQUE | NON_NULL, (empty)
                            // 730: b"lglddtrace.c\ ... st u8: typeof(_357) = *const {l553} u8
                            // 730: b"lglddtrace.c\ ... st u8:   l553 = UNIQUE | NON_NULL, (empty)
                            // 730: b"lglddtrace.c\0": typeof(_358) = *const {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 730: b"lglddtrace.c\0":   l555 = UNIQUE | NON_NULL, (empty)
                            // 730: b"lglddtrace.c\0": typeof(_359) = & {l557} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 730: b"lglddtrace.c\0":   l557 = UNIQUE | NON_NULL, FIXED
                            // 730: b"lglddtrace.c\ ... st u8: typeof(_357 = move _358 as *const u8 (Pointer(ArrayToPointer))) = *const {l893} u8
                            // 730: b"lglddtrace.c\ ... st u8:   l893 = UNIQUE | NON_NULL, (empty)
                            // 730: b"lglddtrace.c\ ... _char: typeof(_356 = move _357 as *const i8 (Misc)) = *const {l894} i8
                            // 730: b"lglddtrace.c\ ... _char:   l894 = UNIQUE | NON_NULL, (empty)
                            // 730: b"lglddtrace.c\0": typeof(_358 = &raw const (*_359)) = *const {l892} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 730: b"lglddtrace.c\0":   l892 = UNIQUE | NON_NULL, (empty)
                            // 730: b"lglddtrace.c\0": typeof(_359 = const b"lglddtrace.c\x00") = & {l891} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 730: b"lglddtrace.c\0":   l891 = UNIQUE | NON_NULL, (empty)
                            238 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                            // 732: (*::core::mem:: ... ptr(): typeof(_362) = *const {l561} i8
                            // 732: (*::core::mem:: ... ptr():   l561 = UNIQUE | NON_NULL, (empty)
                            // 732: (*::core::mem:: ... ptr(): typeof(_363) = & {l563} [i8]
                            // 732: (*::core::mem:: ... ptr():   l563 = UNIQUE | NON_NULL, FIXED
                            // 732: (*::core::mem:: ... ptr(): typeof(_364) = & {l565} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 732: (*::core::mem:: ... ptr():   l565 = UNIQUE | NON_NULL, (empty)
                            // 732: ::core::mem::tr ... 0", ): typeof(_365) = & {l567} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 732: ::core::mem::tr ... 0", ):   l567 = UNIQUE | NON_NULL, FIXED
                            // 732: (*::core::mem:: ... ptr(): typeof(_364 = &(*_365)) = & {l897} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 732: (*::core::mem:: ... ptr():   l897 = UNIQUE | NON_NULL, (empty)
                            // 732: (*::core::mem:: ... ptr(): typeof(_363 = move _364 as &[i8] (Pointer(Unsize))) = & {l898} [i8]
                            // 732: (*::core::mem:: ... ptr():   l898 = UNIQUE | NON_NULL, FIXED
                                b"void process(void)\0",
                                // 733: b"void process( ... d)\0": typeof(_366) = & {l569} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 733: b"void process( ... d)\0":   l569 = UNIQUE | NON_NULL, (empty)
                                // 733: b"void process( ... d)\0": typeof(_367) = & {l571} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 733: b"void process( ... d)\0":   l571 = UNIQUE | NON_NULL, FIXED
                                // 733: b"void process( ... d)\0": typeof(_366 = &(*_367)) = & {l896} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 733: b"void process( ... d)\0":   l896 = UNIQUE | NON_NULL, (empty)
                                // 733: b"void process( ... d)\0": typeof(_367 = const b"void process(void)\x00") = & {l895} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 733: b"void process( ... d)\0":   l895 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                    'c_5422: {
                        if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {
                        } else {
                            __assert_fail(
                                b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                                // 742: b"e->type == SA ... _char: typeof(_375) = *const {l580} i8
                                // 742: b"e->type == SA ... _char:   l580 = UNIQUE | NON_NULL, (empty)
                                // 742: b"e->type == SA ... st u8: typeof(_376) = *const {l582} u8
                                // 742: b"e->type == SA ... st u8:   l582 = UNIQUE | NON_NULL, (empty)
                                // 742: b"e->type == SAT\0": typeof(_377) = *const {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 742: b"e->type == SAT\0":   l584 = UNIQUE | NON_NULL, (empty)
                                // 742: b"e->type == SAT\0": typeof(_378) = & {l586} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 742: b"e->type == SAT\0":   l586 = UNIQUE | NON_NULL, FIXED
                                // 742: b"e->type == SAT\0": typeof(_377 = &raw const (*_378)) = *const {l900} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 742: b"e->type == SAT\0":   l900 = UNIQUE | NON_NULL, (empty)
                                // 742: b"e->type == SA ... st u8: typeof(_376 = move _377 as *const u8 (Pointer(ArrayToPointer))) = *const {l901} u8
                                // 742: b"e->type == SA ... st u8:   l901 = UNIQUE | NON_NULL, (empty)
                                // 742: b"e->type == SA ... _char: typeof(_375 = move _376 as *const i8 (Misc)) = *const {l902} i8
                                // 742: b"e->type == SA ... _char:   l902 = UNIQUE | NON_NULL, (empty)
                                // 742: b"e->type == SAT\0": typeof(_378 = const b"e->type == SAT\x00") = & {l899} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 742: b"e->type == SAT\0":   l899 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 743: b"lglddtrace.c\ ... _char: typeof(_379) = *const {l588} i8
                                // 743: b"lglddtrace.c\ ... _char:   l588 = UNIQUE | NON_NULL, (empty)
                                // 743: b"lglddtrace.c\ ... st u8: typeof(_380) = *const {l590} u8
                                // 743: b"lglddtrace.c\ ... st u8:   l590 = UNIQUE | NON_NULL, (empty)
                                // 743: b"lglddtrace.c\0": typeof(_381) = *const {l592} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 743: b"lglddtrace.c\0":   l592 = UNIQUE | NON_NULL, (empty)
                                // 743: b"lglddtrace.c\0": typeof(_382) = & {l594} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 743: b"lglddtrace.c\0":   l594 = UNIQUE | NON_NULL, FIXED
                                // 743: b"lglddtrace.c\ ... _char: typeof(_379 = move _380 as *const i8 (Misc)) = *const {l906} i8
                                // 743: b"lglddtrace.c\ ... _char:   l906 = UNIQUE | NON_NULL, (empty)
                                // 743: b"lglddtrace.c\0": typeof(_381 = &raw const (*_382)) = *const {l904} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 743: b"lglddtrace.c\0":   l904 = UNIQUE | NON_NULL, (empty)
                                // 743: b"lglddtrace.c\0": typeof(_382 = const b"lglddtrace.c\x00") = & {l903} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 743: b"lglddtrace.c\0":   l903 = UNIQUE | NON_NULL, (empty)
                                // 743: b"lglddtrace.c\ ... st u8: typeof(_380 = move _381 as *const u8 (Pointer(ArrayToPointer))) = *const {l905} u8
                                // 743: b"lglddtrace.c\ ... st u8:   l905 = UNIQUE | NON_NULL, (empty)
                                238 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                                // 745: (*::core::mem:: ... ptr(): typeof(_385) = *const {l598} i8
                                // 745: (*::core::mem:: ... ptr():   l598 = UNIQUE | NON_NULL, (empty)
                                // 745: (*::core::mem:: ... ptr(): typeof(_386) = & {l600} [i8]
                                // 745: (*::core::mem:: ... ptr():   l600 = UNIQUE | NON_NULL, FIXED
                                // 745: (*::core::mem:: ... ptr(): typeof(_387) = & {l602} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 745: (*::core::mem:: ... ptr():   l602 = UNIQUE | NON_NULL, (empty)
                                // 745: ::core::mem::tr ... 0", ): typeof(_388) = & {l604} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 745: ::core::mem::tr ... 0", ):   l604 = UNIQUE | NON_NULL, FIXED
                                // 745: (*::core::mem:: ... ptr(): typeof(_386 = move _387 as &[i8] (Pointer(Unsize))) = & {l910} [i8]
                                // 745: (*::core::mem:: ... ptr():   l910 = UNIQUE | NON_NULL, FIXED
                                // 745: (*::core::mem:: ... ptr(): typeof(_387 = &(*_388)) = & {l909} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                // 745: (*::core::mem:: ... ptr():   l909 = UNIQUE | NON_NULL, (empty)
                                    b"void process(void)\0",
                                    // 746: b"void process( ... d)\0": typeof(_389) = & {l606} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 746: b"void process( ... d)\0":   l606 = UNIQUE | NON_NULL, (empty)
                                    // 746: b"void process( ... d)\0": typeof(_390) = & {l608} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 746: b"void process( ... d)\0":   l608 = UNIQUE | NON_NULL, FIXED
                                    // 746: b"void process( ... d)\0": typeof(_390 = const b"void process(void)\x00") = & {l907} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 746: b"void process( ... d)\0":   l907 = UNIQUE | NON_NULL, (empty)
                                    // 746: b"void process( ... d)\0": typeof(_389 = &(*_390)) = & {l908} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                                    // 746: b"void process( ... d)\0":   l908 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                    };
                    res = lglsat(lgl);
                    // 752: lgl: typeof(_392) = *mut {l611} LGL
                    // 752: lgl:   l611 = UNIQUE | NON_NULL, (empty)
                }
            }
        }
        e = e.offset(1);
        // 756: e.offset(1): typeof(_393) = *mut {l613} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 756: e.offset(1):   l613 = UNIQUE | NON_NULL, (empty)
        // 756: e: typeof(_394) = *mut {l615} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 756: e:   l615 = UNIQUE | NON_NULL, (empty)
        e;
        // 757: e: typeof(_395) = *mut {l617} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 757: e:   l617 = UNIQUE | NON_NULL, (empty)
    }
    close(null);
    close(2 as libc::c_int);
    close(1 as libc::c_int);
    tmp = dup(saved1);
    if tmp == 1 as libc::c_int {
    } else {
        __assert_fail(
            b"tmp == 1\0" as *const u8 as *const libc::c_char,
            // 766: b"tmp == 1\0" a ... _char: typeof(_413) = *const {l636} i8
            // 766: b"tmp == 1\0" a ... _char:   l636 = UNIQUE | NON_NULL, (empty)
            // 766: b"tmp == 1\0" a ... st u8: typeof(_414) = *const {l638} u8
            // 766: b"tmp == 1\0" a ... st u8:   l638 = UNIQUE | NON_NULL, (empty)
            // 766: b"tmp == 1\0": typeof(_415) = *const {l640} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 766: b"tmp == 1\0":   l640 = UNIQUE | NON_NULL, (empty)
            // 766: b"tmp == 1\0": typeof(_416) = & {l642} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 766: b"tmp == 1\0":   l642 = UNIQUE | NON_NULL, FIXED
            // 766: b"tmp == 1\0" a ... _char: typeof(_413 = move _414 as *const i8 (Misc)) = *const {l914} i8
            // 766: b"tmp == 1\0" a ... _char:   l914 = UNIQUE | NON_NULL, (empty)
            // 766: b"tmp == 1\0": typeof(_416 = const b"tmp == 1\x00") = & {l911} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 766: b"tmp == 1\0":   l911 = UNIQUE | NON_NULL, (empty)
            // 766: b"tmp == 1\0": typeof(_415 = &raw const (*_416)) = *const {l912} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 766: b"tmp == 1\0":   l912 = UNIQUE | NON_NULL, (empty)
            // 766: b"tmp == 1\0" a ... st u8: typeof(_414 = move _415 as *const u8 (Pointer(ArrayToPointer))) = *const {l913} u8
            // 766: b"tmp == 1\0" a ... st u8:   l913 = UNIQUE | NON_NULL, (empty)
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            // 767: b"lglddtrace.c\ ... _char: typeof(_417) = *const {l644} i8
            // 767: b"lglddtrace.c\ ... _char:   l644 = UNIQUE | NON_NULL, (empty)
            // 767: b"lglddtrace.c\ ... st u8: typeof(_418) = *const {l646} u8
            // 767: b"lglddtrace.c\ ... st u8:   l646 = UNIQUE | NON_NULL, (empty)
            // 767: b"lglddtrace.c\0": typeof(_419) = *const {l648} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 767: b"lglddtrace.c\0":   l648 = UNIQUE | NON_NULL, (empty)
            // 767: b"lglddtrace.c\0": typeof(_420) = & {l650} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 767: b"lglddtrace.c\0":   l650 = UNIQUE | NON_NULL, FIXED
            // 767: b"lglddtrace.c\0": typeof(_420 = const b"lglddtrace.c\x00") = & {l915} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 767: b"lglddtrace.c\0":   l915 = UNIQUE | NON_NULL, (empty)
            // 767: b"lglddtrace.c\0": typeof(_419 = &raw const (*_420)) = *const {l916} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 767: b"lglddtrace.c\0":   l916 = UNIQUE | NON_NULL, (empty)
            // 767: b"lglddtrace.c\ ... _char: typeof(_417 = move _418 as *const i8 (Misc)) = *const {l918} i8
            // 767: b"lglddtrace.c\ ... _char:   l918 = UNIQUE | NON_NULL, (empty)
            // 767: b"lglddtrace.c\ ... st u8: typeof(_418 = move _419 as *const u8 (Pointer(ArrayToPointer))) = *const {l917} u8
            // 767: b"lglddtrace.c\ ... st u8:   l917 = UNIQUE | NON_NULL, (empty)
            247 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"void process(void)\0"))
            // 769: (*::core::mem:: ... ptr(): typeof(_423) = *const {l654} i8
            // 769: (*::core::mem:: ... ptr():   l654 = UNIQUE | NON_NULL, (empty)
            // 769: (*::core::mem:: ... ptr(): typeof(_424) = & {l656} [i8]
            // 769: (*::core::mem:: ... ptr():   l656 = UNIQUE | NON_NULL, FIXED
            // 769: (*::core::mem:: ... ptr(): typeof(_425) = & {l658} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: (*::core::mem:: ... ptr():   l658 = UNIQUE | NON_NULL, (empty)
            // 769: ::core::mem::tr ... )\0"): typeof(_426) = & {l660} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: ::core::mem::tr ... )\0"):   l660 = UNIQUE | NON_NULL, FIXED
            // 769: b"void process( ... d)\0": typeof(_427) = & {l662} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: b"void process( ... d)\0":   l662 = UNIQUE | NON_NULL, (empty)
            // 769: b"void process( ... d)\0": typeof(_428) = & {l664} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: b"void process( ... d)\0":   l664 = UNIQUE | NON_NULL, FIXED
            // 769: b"void process( ... d)\0": typeof(_427 = &(*_428)) = & {l920} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: b"void process( ... d)\0":   l920 = UNIQUE | NON_NULL, (empty)
            // 769: (*::core::mem:: ... ptr(): typeof(_424 = move _425 as &[i8] (Pointer(Unsize))) = & {l922} [i8]
            // 769: (*::core::mem:: ... ptr():   l922 = UNIQUE | NON_NULL, FIXED
            // 769: b"void process( ... d)\0": typeof(_428 = const b"void process(void)\x00") = & {l919} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: b"void process( ... d)\0":   l919 = UNIQUE | NON_NULL, (empty)
            // 769: (*::core::mem:: ... ptr(): typeof(_425 = &(*_426)) = & {l921} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 769: (*::core::mem:: ... ptr():   l921 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_5278: {
        if tmp == 1 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                // 777: b"tmp == 1\0" a ... _char: typeof(_435) = *const {l672} i8
                // 777: b"tmp == 1\0" a ... _char:   l672 = UNIQUE | NON_NULL, (empty)
                // 777: b"tmp == 1\0" a ... st u8: typeof(_436) = *const {l674} u8
                // 777: b"tmp == 1\0" a ... st u8:   l674 = UNIQUE | NON_NULL, (empty)
                // 777: b"tmp == 1\0": typeof(_437) = *const {l676} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 777: b"tmp == 1\0":   l676 = UNIQUE | NON_NULL, (empty)
                // 777: b"tmp == 1\0": typeof(_438) = & {l678} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 777: b"tmp == 1\0":   l678 = UNIQUE | NON_NULL, FIXED
                // 777: b"tmp == 1\0": typeof(_438 = const b"tmp == 1\x00") = & {l923} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 777: b"tmp == 1\0":   l923 = UNIQUE | NON_NULL, (empty)
                // 777: b"tmp == 1\0": typeof(_437 = &raw const (*_438)) = *const {l924} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 777: b"tmp == 1\0":   l924 = UNIQUE | NON_NULL, (empty)
                // 777: b"tmp == 1\0" a ... _char: typeof(_435 = move _436 as *const i8 (Misc)) = *const {l926} i8
                // 777: b"tmp == 1\0" a ... _char:   l926 = UNIQUE | NON_NULL, (empty)
                // 777: b"tmp == 1\0" a ... st u8: typeof(_436 = move _437 as *const u8 (Pointer(ArrayToPointer))) = *const {l925} u8
                // 777: b"tmp == 1\0" a ... st u8:   l925 = UNIQUE | NON_NULL, (empty)
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                // 778: b"lglddtrace.c\ ... _char: typeof(_439) = *const {l680} i8
                // 778: b"lglddtrace.c\ ... _char:   l680 = UNIQUE | NON_NULL, (empty)
                // 778: b"lglddtrace.c\ ... st u8: typeof(_440) = *const {l682} u8
                // 778: b"lglddtrace.c\ ... st u8:   l682 = UNIQUE | NON_NULL, (empty)
                // 778: b"lglddtrace.c\0": typeof(_441) = *const {l684} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 778: b"lglddtrace.c\0":   l684 = UNIQUE | NON_NULL, (empty)
                // 778: b"lglddtrace.c\0": typeof(_442) = & {l686} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 778: b"lglddtrace.c\0":   l686 = UNIQUE | NON_NULL, FIXED
                // 778: b"lglddtrace.c\ ... _char: typeof(_439 = move _440 as *const i8 (Misc)) = *const {l930} i8
                // 778: b"lglddtrace.c\ ... _char:   l930 = UNIQUE | NON_NULL, (empty)
                // 778: b"lglddtrace.c\0": typeof(_442 = const b"lglddtrace.c\x00") = & {l927} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 778: b"lglddtrace.c\0":   l927 = UNIQUE | NON_NULL, (empty)
                // 778: b"lglddtrace.c\ ... st u8: typeof(_440 = move _441 as *const u8 (Pointer(ArrayToPointer))) = *const {l929} u8
                // 778: b"lglddtrace.c\ ... st u8:   l929 = UNIQUE | NON_NULL, (empty)
                // 778: b"lglddtrace.c\0": typeof(_441 = &raw const (*_442)) = *const {l928} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 778: b"lglddtrace.c\0":   l928 = UNIQUE | NON_NULL, (empty)
                247 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                // 780: (*::core::mem:: ... ptr(): typeof(_445) = *const {l690} i8
                // 780: (*::core::mem:: ... ptr():   l690 = UNIQUE | NON_NULL, (empty)
                // 780: (*::core::mem:: ... ptr(): typeof(_446) = & {l692} [i8]
                // 780: (*::core::mem:: ... ptr():   l692 = UNIQUE | NON_NULL, FIXED
                // 780: (*::core::mem:: ... ptr(): typeof(_447) = & {l694} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 780: (*::core::mem:: ... ptr():   l694 = UNIQUE | NON_NULL, (empty)
                // 780: ::core::mem::tr ... 0", ): typeof(_448) = & {l696} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 780: ::core::mem::tr ... 0", ):   l696 = UNIQUE | NON_NULL, FIXED
                // 780: (*::core::mem:: ... ptr(): typeof(_447 = &(*_448)) = & {l933} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 780: (*::core::mem:: ... ptr():   l933 = UNIQUE | NON_NULL, (empty)
                // 780: (*::core::mem:: ... ptr(): typeof(_446 = move _447 as &[i8] (Pointer(Unsize))) = & {l934} [i8]
                // 780: (*::core::mem:: ... ptr():   l934 = UNIQUE | NON_NULL, FIXED
                    b"void process(void)\0",
                    // 781: b"void process( ... d)\0": typeof(_449) = & {l698} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 781: b"void process( ... d)\0":   l698 = UNIQUE | NON_NULL, (empty)
                    // 781: b"void process( ... d)\0": typeof(_450) = & {l700} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 781: b"void process( ... d)\0":   l700 = UNIQUE | NON_NULL, FIXED
                    // 781: b"void process( ... d)\0": typeof(_449 = &(*_450)) = & {l932} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 781: b"void process( ... d)\0":   l932 = UNIQUE | NON_NULL, (empty)
                    // 781: b"void process( ... d)\0": typeof(_450 = const b"void process(void)\x00") = & {l931} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 781: b"void process( ... d)\0":   l931 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    tmp = dup(saved2);
    if tmp == 2 as libc::c_int {
    } else {
        __assert_fail(
            b"tmp == 2\0" as *const u8 as *const libc::c_char,
            // 791: b"tmp == 2\0" a ... _char: typeof(_459) = *const {l710} i8
            // 791: b"tmp == 2\0" a ... _char:   l710 = UNIQUE | NON_NULL, (empty)
            // 791: b"tmp == 2\0" a ... st u8: typeof(_460) = *const {l712} u8
            // 791: b"tmp == 2\0" a ... st u8:   l712 = UNIQUE | NON_NULL, (empty)
            // 791: b"tmp == 2\0": typeof(_461) = *const {l714} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 791: b"tmp == 2\0":   l714 = UNIQUE | NON_NULL, (empty)
            // 791: b"tmp == 2\0": typeof(_462) = & {l716} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 791: b"tmp == 2\0":   l716 = UNIQUE | NON_NULL, FIXED
            // 791: b"tmp == 2\0" a ... _char: typeof(_459 = move _460 as *const i8 (Misc)) = *const {l938} i8
            // 791: b"tmp == 2\0" a ... _char:   l938 = UNIQUE | NON_NULL, (empty)
            // 791: b"tmp == 2\0": typeof(_461 = &raw const (*_462)) = *const {l936} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 791: b"tmp == 2\0":   l936 = UNIQUE | NON_NULL, (empty)
            // 791: b"tmp == 2\0" a ... st u8: typeof(_460 = move _461 as *const u8 (Pointer(ArrayToPointer))) = *const {l937} u8
            // 791: b"tmp == 2\0" a ... st u8:   l937 = UNIQUE | NON_NULL, (empty)
            // 791: b"tmp == 2\0": typeof(_462 = const b"tmp == 2\x00") = & {l935} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 791: b"tmp == 2\0":   l935 = UNIQUE | NON_NULL, (empty)
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            // 792: b"lglddtrace.c\ ... _char: typeof(_463) = *const {l718} i8
            // 792: b"lglddtrace.c\ ... _char:   l718 = UNIQUE | NON_NULL, (empty)
            // 792: b"lglddtrace.c\ ... st u8: typeof(_464) = *const {l720} u8
            // 792: b"lglddtrace.c\ ... st u8:   l720 = UNIQUE | NON_NULL, (empty)
            // 792: b"lglddtrace.c\0": typeof(_465) = *const {l722} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 792: b"lglddtrace.c\0":   l722 = UNIQUE | NON_NULL, (empty)
            // 792: b"lglddtrace.c\0": typeof(_466) = & {l724} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 792: b"lglddtrace.c\0":   l724 = UNIQUE | NON_NULL, FIXED
            // 792: b"lglddtrace.c\ ... _char: typeof(_463 = move _464 as *const i8 (Misc)) = *const {l942} i8
            // 792: b"lglddtrace.c\ ... _char:   l942 = UNIQUE | NON_NULL, (empty)
            // 792: b"lglddtrace.c\0": typeof(_466 = const b"lglddtrace.c\x00") = & {l939} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 792: b"lglddtrace.c\0":   l939 = UNIQUE | NON_NULL, (empty)
            // 792: b"lglddtrace.c\ ... st u8: typeof(_464 = move _465 as *const u8 (Pointer(ArrayToPointer))) = *const {l941} u8
            // 792: b"lglddtrace.c\ ... st u8:   l941 = UNIQUE | NON_NULL, (empty)
            // 792: b"lglddtrace.c\0": typeof(_465 = &raw const (*_466)) = *const {l940} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 792: b"lglddtrace.c\0":   l940 = UNIQUE | NON_NULL, (empty)
            249 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(b"void process(void)\0"))
            // 794: (*::core::mem:: ... ptr(): typeof(_469) = *const {l728} i8
            // 794: (*::core::mem:: ... ptr():   l728 = UNIQUE | NON_NULL, (empty)
            // 794: (*::core::mem:: ... ptr(): typeof(_470) = & {l730} [i8]
            // 794: (*::core::mem:: ... ptr():   l730 = UNIQUE | NON_NULL, FIXED
            // 794: (*::core::mem:: ... ptr(): typeof(_471) = & {l732} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: (*::core::mem:: ... ptr():   l732 = UNIQUE | NON_NULL, (empty)
            // 794: ::core::mem::tr ... )\0"): typeof(_472) = & {l734} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: ::core::mem::tr ... )\0"):   l734 = UNIQUE | NON_NULL, FIXED
            // 794: b"void process( ... d)\0": typeof(_473) = & {l736} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: b"void process( ... d)\0":   l736 = UNIQUE | NON_NULL, (empty)
            // 794: b"void process( ... d)\0": typeof(_474) = & {l738} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: b"void process( ... d)\0":   l738 = UNIQUE | NON_NULL, FIXED
            // 794: (*::core::mem:: ... ptr(): typeof(_471 = &(*_472)) = & {l945} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: (*::core::mem:: ... ptr():   l945 = UNIQUE | NON_NULL, (empty)
            // 794: b"void process( ... d)\0": typeof(_473 = &(*_474)) = & {l944} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: b"void process( ... d)\0":   l944 = UNIQUE | NON_NULL, (empty)
            // 794: b"void process( ... d)\0": typeof(_474 = const b"void process(void)\x00") = & {l943} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
            // 794: b"void process( ... d)\0":   l943 = UNIQUE | NON_NULL, (empty)
            // 794: (*::core::mem:: ... ptr(): typeof(_470 = move _471 as &[i8] (Pointer(Unsize))) = & {l946} [i8]
            // 794: (*::core::mem:: ... ptr():   l946 = UNIQUE | NON_NULL, FIXED
                .as_ptr(),
        );
    }
    'c_5233: {
        if tmp == 2 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                // 802: b"tmp == 2\0" a ... _char: typeof(_481) = *const {l746} i8
                // 802: b"tmp == 2\0" a ... _char:   l746 = UNIQUE | NON_NULL, (empty)
                // 802: b"tmp == 2\0" a ... st u8: typeof(_482) = *const {l748} u8
                // 802: b"tmp == 2\0" a ... st u8:   l748 = UNIQUE | NON_NULL, (empty)
                // 802: b"tmp == 2\0": typeof(_483) = *const {l750} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 802: b"tmp == 2\0":   l750 = UNIQUE | NON_NULL, (empty)
                // 802: b"tmp == 2\0": typeof(_484) = & {l752} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 802: b"tmp == 2\0":   l752 = UNIQUE | NON_NULL, FIXED
                // 802: b"tmp == 2\0" a ... st u8: typeof(_482 = move _483 as *const u8 (Pointer(ArrayToPointer))) = *const {l949} u8
                // 802: b"tmp == 2\0" a ... st u8:   l949 = UNIQUE | NON_NULL, (empty)
                // 802: b"tmp == 2\0" a ... _char: typeof(_481 = move _482 as *const i8 (Misc)) = *const {l950} i8
                // 802: b"tmp == 2\0" a ... _char:   l950 = UNIQUE | NON_NULL, (empty)
                // 802: b"tmp == 2\0": typeof(_484 = const b"tmp == 2\x00") = & {l947} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 802: b"tmp == 2\0":   l947 = UNIQUE | NON_NULL, (empty)
                // 802: b"tmp == 2\0": typeof(_483 = &raw const (*_484)) = *const {l948} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 802: b"tmp == 2\0":   l948 = UNIQUE | NON_NULL, (empty)
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                // 803: b"lglddtrace.c\ ... _char: typeof(_485) = *const {l754} i8
                // 803: b"lglddtrace.c\ ... _char:   l754 = UNIQUE | NON_NULL, (empty)
                // 803: b"lglddtrace.c\ ... st u8: typeof(_486) = *const {l756} u8
                // 803: b"lglddtrace.c\ ... st u8:   l756 = UNIQUE | NON_NULL, (empty)
                // 803: b"lglddtrace.c\0": typeof(_487) = *const {l758} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 803: b"lglddtrace.c\0":   l758 = UNIQUE | NON_NULL, (empty)
                // 803: b"lglddtrace.c\0": typeof(_488) = & {l760} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 803: b"lglddtrace.c\0":   l760 = UNIQUE | NON_NULL, FIXED
                // 803: b"lglddtrace.c\0": typeof(_488 = const b"lglddtrace.c\x00") = & {l951} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 803: b"lglddtrace.c\0":   l951 = UNIQUE | NON_NULL, (empty)
                // 803: b"lglddtrace.c\ ... _char: typeof(_485 = move _486 as *const i8 (Misc)) = *const {l954} i8
                // 803: b"lglddtrace.c\ ... _char:   l954 = UNIQUE | NON_NULL, (empty)
                // 803: b"lglddtrace.c\0": typeof(_487 = &raw const (*_488)) = *const {l952} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 803: b"lglddtrace.c\0":   l952 = UNIQUE | NON_NULL, (empty)
                // 803: b"lglddtrace.c\ ... st u8: typeof(_486 = move _487 as *const u8 (Pointer(ArrayToPointer))) = *const {l953} u8
                // 803: b"lglddtrace.c\ ... st u8:   l953 = UNIQUE | NON_NULL, (empty)
                249 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 19], &[libc::c_char; 19]>(
                // 805: (*::core::mem:: ... ptr(): typeof(_491) = *const {l764} i8
                // 805: (*::core::mem:: ... ptr():   l764 = UNIQUE | NON_NULL, (empty)
                // 805: (*::core::mem:: ... ptr(): typeof(_492) = & {l766} [i8]
                // 805: (*::core::mem:: ... ptr():   l766 = UNIQUE | NON_NULL, FIXED
                // 805: (*::core::mem:: ... ptr(): typeof(_493) = & {l768} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 805: (*::core::mem:: ... ptr():   l768 = UNIQUE | NON_NULL, (empty)
                // 805: ::core::mem::tr ... 0", ): typeof(_494) = & {l770} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 805: ::core::mem::tr ... 0", ):   l770 = UNIQUE | NON_NULL, FIXED
                // 805: (*::core::mem:: ... ptr(): typeof(_492 = move _493 as &[i8] (Pointer(Unsize))) = & {l958} [i8]
                // 805: (*::core::mem:: ... ptr():   l958 = UNIQUE | NON_NULL, FIXED
                // 805: (*::core::mem:: ... ptr(): typeof(_493 = &(*_494)) = & {l957} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                // 805: (*::core::mem:: ... ptr():   l957 = UNIQUE | NON_NULL, (empty)
                    b"void process(void)\0",
                    // 806: b"void process( ... d)\0": typeof(_495) = & {l772} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 806: b"void process( ... d)\0":   l772 = UNIQUE | NON_NULL, (empty)
                    // 806: b"void process( ... d)\0": typeof(_496) = & {l774} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 806: b"void process( ... d)\0":   l774 = UNIQUE | NON_NULL, FIXED
                    // 806: b"void process( ... d)\0": typeof(_495 = &(*_496)) = & {l956} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 806: b"void process( ... d)\0":   l956 = UNIQUE | NON_NULL, (empty)
                    // 806: b"void process( ... d)\0": typeof(_496 = const b"void process(void)\x00") = & {l955} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                    // 806: b"void process( ... d)\0":   l955 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
}
unsafe extern "C" fn run() -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut id: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    id = fork();
    if id != 0 {
        if id < 0 as libc::c_int {
            die(b"can not generate child process\0" as *const u8 as *const libc::c_char);
            // 820: b"can not gener ... _char: typeof(_14) = *const {l14} i8
            // 820: b"can not gener ... _char:   l14 = UNIQUE | NON_NULL, (empty)
            // 820: b"can not gener ... st u8: typeof(_15) = *const {l16} u8
            // 820: b"can not gener ... st u8:   l16 = UNIQUE | NON_NULL, (empty)
            // 820: b"can not gener ... ss\0": typeof(_16) = *const {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 820: b"can not gener ... ss\0":   l18 = UNIQUE | NON_NULL, (empty)
            // 820: b"can not gener ... ss\0": typeof(_17) = & {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 820: b"can not gener ... ss\0":   l20 = UNIQUE | NON_NULL, FIXED
            // 820: b"can not gener ... _char: typeof(_14 = move _15 as *const i8 (Misc)) = *const {l52} i8
            // 820: b"can not gener ... _char:   l52 = UNIQUE | NON_NULL, (empty)
            // 820: b"can not gener ... ss\0": typeof(_16 = &raw const (*_17)) = *const {l50} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 820: b"can not gener ... ss\0":   l50 = UNIQUE | NON_NULL, (empty)
            // 820: b"can not gener ... st u8: typeof(_15 = move _16 as *const u8 (Pointer(ArrayToPointer))) = *const {l51} u8
            // 820: b"can not gener ... st u8:   l51 = UNIQUE | NON_NULL, (empty)
            // 820: b"can not gener ... ss\0": typeof(_17 = const b"can not generate child process\x00") = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 820: b"can not gener ... ss\0":   l49 = UNIQUE | NON_NULL, (empty)
        }
        tmp = wait(&mut status);
        // 822: &mut status: typeof(_19) = *mut {l23} i32
        // 822: &mut status:   l23 = UNIQUE | NON_NULL, (empty)
        // 822: &mut status: typeof(_20) = &mut {l25} i32
        // 822: &mut status:   l25 = UNIQUE | NON_NULL, (empty)
        // 822: &mut status: typeof(_20 = &mut _2) = &mut {l53} i32
        // 822: &mut status:   l53 = UNIQUE | NON_NULL, (empty)
        // 822: &mut status: typeof(_19 = &raw mut (*_20)) = *mut {l54} i32
        // 822: &mut status:   l54 = UNIQUE | NON_NULL, (empty)
        if tmp != id {
            die(b"'wait' did not return child process\0" as *const u8 as *const libc::c_char);
            // 824: b"'wait' did no ... _char: typeof(_25) = *const {l31} i8
            // 824: b"'wait' did no ... _char:   l31 = UNIQUE | NON_NULL, (empty)
            // 824: b"'wait' did no ... st u8: typeof(_26) = *const {l33} u8
            // 824: b"'wait' did no ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
            // 824: b"'wait' did no ... ss\0": typeof(_27) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 824: b"'wait' did no ... ss\0":   l35 = UNIQUE | NON_NULL, (empty)
            // 824: b"'wait' did no ... ss\0": typeof(_28) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 824: b"'wait' did no ... ss\0":   l37 = UNIQUE | NON_NULL, FIXED
            // 824: b"'wait' did no ... ss\0": typeof(_27 = &raw const (*_28)) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 824: b"'wait' did no ... ss\0":   l56 = UNIQUE | NON_NULL, (empty)
            // 824: b"'wait' did no ... _char: typeof(_25 = move _26 as *const i8 (Misc)) = *const {l58} i8
            // 824: b"'wait' did no ... _char:   l58 = UNIQUE | NON_NULL, (empty)
            // 824: b"'wait' did no ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l57} u8
            // 824: b"'wait' did no ... st u8:   l57 = UNIQUE | NON_NULL, (empty)
            // 824: b"'wait' did no ... ss\0": typeof(_28 = const b"\'wait\' did not return child process\x00") = & {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
            // 824: b"'wait' did no ... ss\0":   l55 = UNIQUE | NON_NULL, (empty)
        }
    } else {
        process();
        exit(0 as libc::c_int);
    }
    runs += 1;
    // 830: runs: typeof(_33) = *mut {l43} i32
    // 830: runs:   l43 = UNIQUE | NON_NULL, (empty)
    runs;
    // 831: runs: typeof(_36) = *mut {l47} i32
    // 831: runs:   l47 = UNIQUE | NON_NULL, (empty)
    return status;
}
unsafe extern "C" fn lit(mut lit_0: libc::c_int) -> libc::c_int {
    let mut idx: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    if lit_0 == 0 {
        return 0 as libc::c_int;
    }
    idx = abs(lit_0);
    if (0 as libc::c_int) < idx && idx <= nmap {
    // 841: nmap: typeof(_19) = *mut {l19} i32
    // 841: nmap:   l19 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"0 < idx && idx <= nmap\0" as *const u8 as *const libc::c_char,
            // 844: b"0 < idx && id ... _char: typeof(_22) = *const {l23} i8
            // 844: b"0 < idx && id ... _char:   l23 = UNIQUE | NON_NULL, (empty)
            // 844: b"0 < idx && id ... st u8: typeof(_23) = *const {l25} u8
            // 844: b"0 < idx && id ... st u8:   l25 = UNIQUE | NON_NULL, (empty)
            // 844: b"0 < idx && id ... ap\0": typeof(_24) = *const {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 844: b"0 < idx && id ... ap\0":   l27 = UNIQUE | NON_NULL, (empty)
            // 844: b"0 < idx && id ... ap\0": typeof(_25) = & {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 844: b"0 < idx && id ... ap\0":   l29 = UNIQUE | NON_NULL, FIXED
            // 844: b"0 < idx && id ... st u8: typeof(_23 = move _24 as *const u8 (Pointer(ArrayToPointer))) = *const {l113} u8
            // 844: b"0 < idx && id ... st u8:   l113 = UNIQUE | NON_NULL, (empty)
            // 844: b"0 < idx && id ... _char: typeof(_22 = move _23 as *const i8 (Misc)) = *const {l114} i8
            // 844: b"0 < idx && id ... _char:   l114 = UNIQUE | NON_NULL, (empty)
            // 844: b"0 < idx && id ... ap\0": typeof(_25 = const b"0 < idx && idx <= nmap\x00") = & {l111} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 844: b"0 < idx && id ... ap\0":   l111 = UNIQUE | NON_NULL, (empty)
            // 844: b"0 < idx && id ... ap\0": typeof(_24 = &raw const (*_25)) = *const {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 844: b"0 < idx && id ... ap\0":   l112 = UNIQUE | NON_NULL, (empty)
            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
            // 845: b"lglddtrace.c\ ... _char: typeof(_26) = *const {l31} i8
            // 845: b"lglddtrace.c\ ... _char:   l31 = UNIQUE | NON_NULL, (empty)
            // 845: b"lglddtrace.c\ ... st u8: typeof(_27) = *const {l33} u8
            // 845: b"lglddtrace.c\ ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
            // 845: b"lglddtrace.c\0": typeof(_28) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 845: b"lglddtrace.c\0":   l35 = UNIQUE | NON_NULL, (empty)
            // 845: b"lglddtrace.c\0": typeof(_29) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 845: b"lglddtrace.c\0":   l37 = UNIQUE | NON_NULL, FIXED
            // 845: b"lglddtrace.c\ ... st u8: typeof(_27 = move _28 as *const u8 (Pointer(ArrayToPointer))) = *const {l117} u8
            // 845: b"lglddtrace.c\ ... st u8:   l117 = UNIQUE | NON_NULL, (empty)
            // 845: b"lglddtrace.c\0": typeof(_29 = const b"lglddtrace.c\x00") = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 845: b"lglddtrace.c\0":   l115 = UNIQUE | NON_NULL, (empty)
            // 845: b"lglddtrace.c\ ... _char: typeof(_26 = move _27 as *const i8 (Misc)) = *const {l118} i8
            // 845: b"lglddtrace.c\ ... _char:   l118 = UNIQUE | NON_NULL, (empty)
            // 845: b"lglddtrace.c\0": typeof(_28 = &raw const (*_29)) = *const {l116} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 845: b"lglddtrace.c\0":   l116 = UNIQUE | NON_NULL, (empty)
            268 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"int lit(int)\0")).as_ptr(),
            // 847: (*::core::mem:: ... ptr(): typeof(_32) = *const {l41} i8
            // 847: (*::core::mem:: ... ptr():   l41 = UNIQUE | NON_NULL, (empty)
            // 847: (*::core::mem:: ... ptr(): typeof(_33) = & {l43} [i8]
            // 847: (*::core::mem:: ... ptr():   l43 = UNIQUE | NON_NULL, FIXED
            // 847: (*::core::mem:: ... ptr(): typeof(_34) = & {l45} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: (*::core::mem:: ... ptr():   l45 = UNIQUE | NON_NULL, (empty)
            // 847: ::core::mem::tr ... )\0"): typeof(_35) = & {l47} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: ::core::mem::tr ... )\0"):   l47 = UNIQUE | NON_NULL, FIXED
            // 847: b"int lit(int)\0": typeof(_36) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: b"int lit(int)\0":   l49 = UNIQUE | NON_NULL, (empty)
            // 847: b"int lit(int)\0": typeof(_37) = & {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: b"int lit(int)\0":   l51 = UNIQUE | NON_NULL, FIXED
            // 847: (*::core::mem:: ... ptr(): typeof(_34 = &(*_35)) = & {l121} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: (*::core::mem:: ... ptr():   l121 = UNIQUE | NON_NULL, (empty)
            // 847: (*::core::mem:: ... ptr(): typeof(_33 = move _34 as &[i8] (Pointer(Unsize))) = & {l122} [i8]
            // 847: (*::core::mem:: ... ptr():   l122 = UNIQUE | NON_NULL, FIXED
            // 847: b"int lit(int)\0": typeof(_37 = const b"int lit(int)\x00") = & {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: b"int lit(int)\0":   l119 = UNIQUE | NON_NULL, (empty)
            // 847: b"int lit(int)\0": typeof(_36 = &(*_37)) = & {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 847: b"int lit(int)\0":   l120 = UNIQUE | NON_NULL, (empty)
        );
    }
    'c_6238: {
        if (0 as libc::c_int) < idx && idx <= nmap {
        // 851: nmap: typeof(_46) = *mut {l61} i32
        // 851: nmap:   l61 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"0 < idx && idx <= nmap\0" as *const u8 as *const libc::c_char,
                // 854: b"0 < idx && id ... _char: typeof(_49) = *const {l65} i8
                // 854: b"0 < idx && id ... _char:   l65 = UNIQUE | NON_NULL, (empty)
                // 854: b"0 < idx && id ... st u8: typeof(_50) = *const {l67} u8
                // 854: b"0 < idx && id ... st u8:   l67 = UNIQUE | NON_NULL, (empty)
                // 854: b"0 < idx && id ... ap\0": typeof(_51) = *const {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 854: b"0 < idx && id ... ap\0":   l69 = UNIQUE | NON_NULL, (empty)
                // 854: b"0 < idx && id ... ap\0": typeof(_52) = & {l71} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 854: b"0 < idx && id ... ap\0":   l71 = UNIQUE | NON_NULL, FIXED
                // 854: b"0 < idx && id ... ap\0": typeof(_51 = &raw const (*_52)) = *const {l124} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 854: b"0 < idx && id ... ap\0":   l124 = UNIQUE | NON_NULL, (empty)
                // 854: b"0 < idx && id ... _char: typeof(_49 = move _50 as *const i8 (Misc)) = *const {l126} i8
                // 854: b"0 < idx && id ... _char:   l126 = UNIQUE | NON_NULL, (empty)
                // 854: b"0 < idx && id ... ap\0": typeof(_52 = const b"0 < idx && idx <= nmap\x00") = & {l123} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 854: b"0 < idx && id ... ap\0":   l123 = UNIQUE | NON_NULL, (empty)
                // 854: b"0 < idx && id ... st u8: typeof(_50 = move _51 as *const u8 (Pointer(ArrayToPointer))) = *const {l125} u8
                // 854: b"0 < idx && id ... st u8:   l125 = UNIQUE | NON_NULL, (empty)
                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                // 855: b"lglddtrace.c\ ... _char: typeof(_53) = *const {l73} i8
                // 855: b"lglddtrace.c\ ... _char:   l73 = UNIQUE | NON_NULL, (empty)
                // 855: b"lglddtrace.c\ ... st u8: typeof(_54) = *const {l75} u8
                // 855: b"lglddtrace.c\ ... st u8:   l75 = UNIQUE | NON_NULL, (empty)
                // 855: b"lglddtrace.c\0": typeof(_55) = *const {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 855: b"lglddtrace.c\0":   l77 = UNIQUE | NON_NULL, (empty)
                // 855: b"lglddtrace.c\0": typeof(_56) = & {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 855: b"lglddtrace.c\0":   l79 = UNIQUE | NON_NULL, FIXED
                // 855: b"lglddtrace.c\0": typeof(_56 = const b"lglddtrace.c\x00") = & {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 855: b"lglddtrace.c\0":   l127 = UNIQUE | NON_NULL, (empty)
                // 855: b"lglddtrace.c\ ... _char: typeof(_53 = move _54 as *const i8 (Misc)) = *const {l130} i8
                // 855: b"lglddtrace.c\ ... _char:   l130 = UNIQUE | NON_NULL, (empty)
                // 855: b"lglddtrace.c\0": typeof(_55 = &raw const (*_56)) = *const {l128} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 855: b"lglddtrace.c\0":   l128 = UNIQUE | NON_NULL, (empty)
                // 855: b"lglddtrace.c\ ... st u8: typeof(_54 = move _55 as *const u8 (Pointer(ArrayToPointer))) = *const {l129} u8
                // 855: b"lglddtrace.c\ ... st u8:   l129 = UNIQUE | NON_NULL, (empty)
                268 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 13], &[libc::c_char; 13]>(b"int lit(int)\0"))
                // 857: (*::core::mem:: ... ptr(): typeof(_59) = *const {l83} i8
                // 857: (*::core::mem:: ... ptr():   l83 = UNIQUE | NON_NULL, (empty)
                // 857: (*::core::mem:: ... ptr(): typeof(_60) = & {l85} [i8]
                // 857: (*::core::mem:: ... ptr():   l85 = UNIQUE | NON_NULL, FIXED
                // 857: (*::core::mem:: ... ptr(): typeof(_61) = & {l87} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: (*::core::mem:: ... ptr():   l87 = UNIQUE | NON_NULL, (empty)
                // 857: ::core::mem::tr ... )\0"): typeof(_62) = & {l89} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: ::core::mem::tr ... )\0"):   l89 = UNIQUE | NON_NULL, FIXED
                // 857: b"int lit(int)\0": typeof(_63) = & {l91} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: b"int lit(int)\0":   l91 = UNIQUE | NON_NULL, (empty)
                // 857: b"int lit(int)\0": typeof(_64) = & {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: b"int lit(int)\0":   l93 = UNIQUE | NON_NULL, FIXED
                // 857: (*::core::mem:: ... ptr(): typeof(_61 = &(*_62)) = & {l133} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: (*::core::mem:: ... ptr():   l133 = UNIQUE | NON_NULL, (empty)
                // 857: (*::core::mem:: ... ptr(): typeof(_60 = move _61 as &[i8] (Pointer(Unsize))) = & {l134} [i8]
                // 857: (*::core::mem:: ... ptr():   l134 = UNIQUE | NON_NULL, FIXED
                // 857: b"int lit(int)\0": typeof(_64 = const b"int lit(int)\x00") = & {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: b"int lit(int)\0":   l131 = UNIQUE | NON_NULL, (empty)
                // 857: b"int lit(int)\0": typeof(_63 = &(*_64)) = & {l132} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 857: b"int lit(int)\0":   l132 = UNIQUE | NON_NULL, (empty)
                    .as_ptr(),
            );
        }
    };
    res = *map.offset(idx as isize);
    // 862: map.offset(idx  ... size): typeof(_66) = *mut {l96} i32
    // 862: map.offset(idx  ... size):   l96 = UNIQUE | NON_NULL, (empty)
    // 862: map: typeof(_67) = *mut {l98} i32
    // 862: map:   l98 = UNIQUE | NON_NULL, (empty)
    // 862: map: typeof(_68) = *mut {l100} *mut {l101} i32
    // 862: map:   l100 = UNIQUE | NON_NULL, (empty)
    // 862: map:   l101 = UNIQUE | NON_NULL, (empty)
    if lit_0 < 0 as libc::c_int {
        res = -res;
    }
    return res;
}
unsafe extern "C" fn print(mut e: *mut Event, mut file: *mut FILE) {
// 868: mut e: typeof(_1) = *mut {g12} DefId(0:354 ~ lglddtrace[7e63]::Event)
// 868: mut e:   g12 = UNIQUE | NON_NULL, FIXED
// 868: mut file: typeof(_2) = *mut {g13} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
// 868: mut file:   g13 = UNIQUE | NON_NULL, FIXED
    let mut o: *mut Opt = 0 as *mut Opt;
    // 869: mut o: typeof(_3) = *mut {l3} DefId(0:362 ~ lglddtrace[7e63]::Opt)
    // 869: mut o:   l3 = UNIQUE | NON_NULL, (empty)
    // 869: 0 as *mut Opt: typeof(_3 = const 0_usize as *mut Opt (PointerFromExposedAddress)) = *mut {l594} DefId(0:362 ~ lglddtrace[7e63]::Opt)
    // 869: 0 as *mut Opt:   l594 = UNIQUE | NON_NULL, (empty)
    match (*e).type_0 as libc::c_uint {
        0 => {
            fprintf(
                file,
                // 873: file: typeof(_7) = *mut {l8} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 873: file:   l8 = UNIQUE | NON_NULL, (empty)
                b"add %d\n\0" as *const u8 as *const libc::c_char,
                // 874: b"add %d\n\0" a ... _char: typeof(_8) = *const {l10} i8
                // 874: b"add %d\n\0" a ... _char:   l10 = UNIQUE | NON_NULL, (empty)
                // 874: b"add %d\n\0" a ... st u8: typeof(_9) = *const {l12} u8
                // 874: b"add %d\n\0" a ... st u8:   l12 = UNIQUE | NON_NULL, (empty)
                // 874: b"add %d\n\0": typeof(_10) = *const {l14} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 874: b"add %d\n\0":   l14 = UNIQUE | NON_NULL, (empty)
                // 874: b"add %d\n\0": typeof(_11) = & {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 874: b"add %d\n\0":   l16 = UNIQUE | NON_NULL, FIXED
                // 874: b"add %d\n\0" a ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l598} i8
                // 874: b"add %d\n\0" a ... _char:   l598 = UNIQUE | NON_NULL, (empty)
                // 874: b"add %d\n\0" a ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l597} u8
                // 874: b"add %d\n\0" a ... st u8:   l597 = UNIQUE | NON_NULL, (empty)
                // 874: b"add %d\n\0": typeof(_11 = const b"add %d\n\x00") = & {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 874: b"add %d\n\0":   l595 = UNIQUE | NON_NULL, (empty)
                // 874: b"add %d\n\0": typeof(_10 = &raw const (*_11)) = *const {l596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 874: b"add %d\n\0":   l596 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        1 => {
            fprintf(
                file,
                // 880: file: typeof(_15) = *mut {l21} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 880: file:   l21 = UNIQUE | NON_NULL, (empty)
                b"assume %d\n\0" as *const u8 as *const libc::c_char,
                // 881: b"assume %d\n\0 ... _char: typeof(_16) = *const {l23} i8
                // 881: b"assume %d\n\0 ... _char:   l23 = UNIQUE | NON_NULL, (empty)
                // 881: b"assume %d\n\0 ... st u8: typeof(_17) = *const {l25} u8
                // 881: b"assume %d\n\0 ... st u8:   l25 = UNIQUE | NON_NULL, (empty)
                // 881: b"assume %d\n\0": typeof(_18) = *const {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 881: b"assume %d\n\0":   l27 = UNIQUE | NON_NULL, (empty)
                // 881: b"assume %d\n\0": typeof(_19) = & {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 881: b"assume %d\n\0":   l29 = UNIQUE | NON_NULL, FIXED
                // 881: b"assume %d\n\0 ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l602} i8
                // 881: b"assume %d\n\0 ... _char:   l602 = UNIQUE | NON_NULL, (empty)
                // 881: b"assume %d\n\0 ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l601} u8
                // 881: b"assume %d\n\0 ... st u8:   l601 = UNIQUE | NON_NULL, (empty)
                // 881: b"assume %d\n\0": typeof(_18 = &raw const (*_19)) = *const {l600} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 881: b"assume %d\n\0":   l600 = UNIQUE | NON_NULL, (empty)
                // 881: b"assume %d\n\0": typeof(_19 = const b"assume %d\n\x00") = & {l599} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 881: b"assume %d\n\0":   l599 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        29 => {
            fprintf(file, b"changed\n\0" as *const u8 as *const libc::c_char);
            // 886: file: typeof(_23) = *mut {l34} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 886: file:   l34 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0"  ... _char: typeof(_24) = *const {l36} i8
            // 886: b"changed\n\0"  ... _char:   l36 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0"  ... st u8: typeof(_25) = *const {l38} u8
            // 886: b"changed\n\0"  ... st u8:   l38 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0": typeof(_26) = *const {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 886: b"changed\n\0":   l40 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0": typeof(_27) = & {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 886: b"changed\n\0":   l42 = UNIQUE | NON_NULL, FIXED
            // 886: b"changed\n\0": typeof(_27 = const b"changed\n\x00") = & {l603} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 886: b"changed\n\0":   l603 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0"  ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l605} u8
            // 886: b"changed\n\0"  ... st u8:   l605 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0"  ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l606} i8
            // 886: b"changed\n\0"  ... _char:   l606 = UNIQUE | NON_NULL, (empty)
            // 886: b"changed\n\0": typeof(_26 = &raw const (*_27)) = *const {l604} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 886: b"changed\n\0":   l604 = UNIQUE | NON_NULL, (empty)
        }
        28 => {
            fprintf(file, b"chkclone\n\0" as *const u8 as *const libc::c_char);
            // 889: file: typeof(_29) = *mut {l45} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 889: file:   l45 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0" ... _char: typeof(_30) = *const {l47} i8
            // 889: b"chkclone\n\0" ... _char:   l47 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0" ... st u8: typeof(_31) = *const {l49} u8
            // 889: b"chkclone\n\0" ... st u8:   l49 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0": typeof(_32) = *const {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 889: b"chkclone\n\0":   l51 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0": typeof(_33) = & {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 889: b"chkclone\n\0":   l53 = UNIQUE | NON_NULL, FIXED
            // 889: b"chkclone\n\0": typeof(_32 = &raw const (*_33)) = *const {l608} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 889: b"chkclone\n\0":   l608 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0" ... _char: typeof(_30 = move _31 as *const i8 (Misc)) = *const {l610} i8
            // 889: b"chkclone\n\0" ... _char:   l610 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0" ... st u8: typeof(_31 = move _32 as *const u8 (Pointer(ArrayToPointer))) = *const {l609} u8
            // 889: b"chkclone\n\0" ... st u8:   l609 = UNIQUE | NON_NULL, (empty)
            // 889: b"chkclone\n\0": typeof(_33 = const b"chkclone\n\x00") = & {l607} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 889: b"chkclone\n\0":   l607 = UNIQUE | NON_NULL, (empty)
        }
        2 => {
            fprintf(
                file,
                // 893: file: typeof(_35) = *mut {l56} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 893: file:   l56 = UNIQUE | NON_NULL, (empty)
                b"deref %d\n\0" as *const u8 as *const libc::c_char,
                // 894: b"deref %d\n\0" ... _char: typeof(_36) = *const {l58} i8
                // 894: b"deref %d\n\0" ... _char:   l58 = UNIQUE | NON_NULL, (empty)
                // 894: b"deref %d\n\0" ... st u8: typeof(_37) = *const {l60} u8
                // 894: b"deref %d\n\0" ... st u8:   l60 = UNIQUE | NON_NULL, (empty)
                // 894: b"deref %d\n\0": typeof(_38) = *const {l62} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 894: b"deref %d\n\0":   l62 = UNIQUE | NON_NULL, (empty)
                // 894: b"deref %d\n\0": typeof(_39) = & {l64} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 894: b"deref %d\n\0":   l64 = UNIQUE | NON_NULL, FIXED
                // 894: b"deref %d\n\0": typeof(_39 = const b"deref %d\n\x00") = & {l611} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 894: b"deref %d\n\0":   l611 = UNIQUE | NON_NULL, (empty)
                // 894: b"deref %d\n\0": typeof(_38 = &raw const (*_39)) = *const {l612} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 894: b"deref %d\n\0":   l612 = UNIQUE | NON_NULL, (empty)
                // 894: b"deref %d\n\0" ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l613} u8
                // 894: b"deref %d\n\0" ... st u8:   l613 = UNIQUE | NON_NULL, (empty)
                // 894: b"deref %d\n\0" ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l614} i8
                // 894: b"deref %d\n\0" ... _char:   l614 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        3 => {
            fprintf(
                file,
                // 900: file: typeof(_43) = *mut {l69} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 900: file:   l69 = UNIQUE | NON_NULL, (empty)
                b"failed %d\n\0" as *const u8 as *const libc::c_char,
                // 901: b"failed %d\n\0 ... _char: typeof(_44) = *const {l71} i8
                // 901: b"failed %d\n\0 ... _char:   l71 = UNIQUE | NON_NULL, (empty)
                // 901: b"failed %d\n\0 ... st u8: typeof(_45) = *const {l73} u8
                // 901: b"failed %d\n\0 ... st u8:   l73 = UNIQUE | NON_NULL, (empty)
                // 901: b"failed %d\n\0": typeof(_46) = *const {l75} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 901: b"failed %d\n\0":   l75 = UNIQUE | NON_NULL, (empty)
                // 901: b"failed %d\n\0": typeof(_47) = & {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 901: b"failed %d\n\0":   l77 = UNIQUE | NON_NULL, FIXED
                // 901: b"failed %d\n\0 ... st u8: typeof(_45 = move _46 as *const u8 (Pointer(ArrayToPointer))) = *const {l617} u8
                // 901: b"failed %d\n\0 ... st u8:   l617 = UNIQUE | NON_NULL, (empty)
                // 901: b"failed %d\n\0": typeof(_46 = &raw const (*_47)) = *const {l616} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 901: b"failed %d\n\0":   l616 = UNIQUE | NON_NULL, (empty)
                // 901: b"failed %d\n\0": typeof(_47 = const b"failed %d\n\x00") = & {l615} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 901: b"failed %d\n\0":   l615 = UNIQUE | NON_NULL, (empty)
                // 901: b"failed %d\n\0 ... _char: typeof(_44 = move _45 as *const i8 (Misc)) = *const {l618} i8
                // 901: b"failed %d\n\0 ... _char:   l618 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        26 => {
            fprintf(
                file,
                // 907: file: typeof(_51) = *mut {l82} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 907: file:   l82 = UNIQUE | NON_NULL, (empty)
                b"fixed %d\n\0" as *const u8 as *const libc::c_char,
                // 908: b"fixed %d\n\0" ... _char: typeof(_52) = *const {l84} i8
                // 908: b"fixed %d\n\0" ... _char:   l84 = UNIQUE | NON_NULL, (empty)
                // 908: b"fixed %d\n\0" ... st u8: typeof(_53) = *const {l86} u8
                // 908: b"fixed %d\n\0" ... st u8:   l86 = UNIQUE | NON_NULL, (empty)
                // 908: b"fixed %d\n\0": typeof(_54) = *const {l88} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 908: b"fixed %d\n\0":   l88 = UNIQUE | NON_NULL, (empty)
                // 908: b"fixed %d\n\0": typeof(_55) = & {l90} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 908: b"fixed %d\n\0":   l90 = UNIQUE | NON_NULL, FIXED
                // 908: b"fixed %d\n\0": typeof(_54 = &raw const (*_55)) = *const {l620} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 908: b"fixed %d\n\0":   l620 = UNIQUE | NON_NULL, (empty)
                // 908: b"fixed %d\n\0": typeof(_55 = const b"fixed %d\n\x00") = & {l619} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 908: b"fixed %d\n\0":   l619 = UNIQUE | NON_NULL, (empty)
                // 908: b"fixed %d\n\0" ... st u8: typeof(_53 = move _54 as *const u8 (Pointer(ArrayToPointer))) = *const {l621} u8
                // 908: b"fixed %d\n\0" ... st u8:   l621 = UNIQUE | NON_NULL, (empty)
                // 908: b"fixed %d\n\0" ... _char: typeof(_52 = move _53 as *const i8 (Misc)) = *const {l622} i8
                // 908: b"fixed %d\n\0" ... _char:   l622 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        21 => {
            fprintf(
                file,
                // 914: file: typeof(_59) = *mut {l95} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 914: file:   l95 = UNIQUE | NON_NULL, (empty)
                b"frozen %d\n\0" as *const u8 as *const libc::c_char,
                // 915: b"frozen %d\n\0 ... _char: typeof(_60) = *const {l97} i8
                // 915: b"frozen %d\n\0 ... _char:   l97 = UNIQUE | NON_NULL, (empty)
                // 915: b"frozen %d\n\0 ... st u8: typeof(_61) = *const {l99} u8
                // 915: b"frozen %d\n\0 ... st u8:   l99 = UNIQUE | NON_NULL, (empty)
                // 915: b"frozen %d\n\0": typeof(_62) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 915: b"frozen %d\n\0":   l101 = UNIQUE | NON_NULL, (empty)
                // 915: b"frozen %d\n\0": typeof(_63) = & {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 915: b"frozen %d\n\0":   l103 = UNIQUE | NON_NULL, FIXED
                // 915: b"frozen %d\n\0": typeof(_62 = &raw const (*_63)) = *const {l624} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 915: b"frozen %d\n\0":   l624 = UNIQUE | NON_NULL, (empty)
                // 915: b"frozen %d\n\0": typeof(_63 = const b"frozen %d\n\x00") = & {l623} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 915: b"frozen %d\n\0":   l623 = UNIQUE | NON_NULL, (empty)
                // 915: b"frozen %d\n\0 ... _char: typeof(_60 = move _61 as *const i8 (Misc)) = *const {l626} i8
                // 915: b"frozen %d\n\0 ... _char:   l626 = UNIQUE | NON_NULL, (empty)
                // 915: b"frozen %d\n\0 ... st u8: typeof(_61 = move _62 as *const u8 (Pointer(ArrayToPointer))) = *const {l625} u8
                // 915: b"frozen %d\n\0 ... st u8:   l625 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        23 => {
            fprintf(
                file,
                // 921: file: typeof(_67) = *mut {l108} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 921: file:   l108 = UNIQUE | NON_NULL, (empty)
                b"reusable %d\n\0" as *const u8 as *const libc::c_char,
                // 922: b"reusable %d\n ... _char: typeof(_68) = *const {l110} i8
                // 922: b"reusable %d\n ... _char:   l110 = UNIQUE | NON_NULL, (empty)
                // 922: b"reusable %d\n ... st u8: typeof(_69) = *const {l112} u8
                // 922: b"reusable %d\n ... st u8:   l112 = UNIQUE | NON_NULL, (empty)
                // 922: b"reusable %d\n\0": typeof(_70) = *const {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 922: b"reusable %d\n\0":   l114 = UNIQUE | NON_NULL, (empty)
                // 922: b"reusable %d\n\0": typeof(_71) = & {l116} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 922: b"reusable %d\n\0":   l116 = UNIQUE | NON_NULL, FIXED
                // 922: b"reusable %d\n ... _char: typeof(_68 = move _69 as *const i8 (Misc)) = *const {l630} i8
                // 922: b"reusable %d\n ... _char:   l630 = UNIQUE | NON_NULL, (empty)
                // 922: b"reusable %d\n\0": typeof(_70 = &raw const (*_71)) = *const {l628} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 922: b"reusable %d\n\0":   l628 = UNIQUE | NON_NULL, (empty)
                // 922: b"reusable %d\n\0": typeof(_71 = const b"reusable %d\n\x00") = & {l627} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 922: b"reusable %d\n\0":   l627 = UNIQUE | NON_NULL, (empty)
                // 922: b"reusable %d\n ... st u8: typeof(_69 = move _70 as *const u8 (Pointer(ArrayToPointer))) = *const {l629} u8
                // 922: b"reusable %d\n ... st u8:   l629 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        22 => {
            fprintf(
                file,
                // 928: file: typeof(_75) = *mut {l121} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 928: file:   l121 = UNIQUE | NON_NULL, (empty)
                b"usable %d\n\0" as *const u8 as *const libc::c_char,
                // 929: b"usable %d\n\0 ... _char: typeof(_76) = *const {l123} i8
                // 929: b"usable %d\n\0 ... _char:   l123 = UNIQUE | NON_NULL, (empty)
                // 929: b"usable %d\n\0 ... st u8: typeof(_77) = *const {l125} u8
                // 929: b"usable %d\n\0 ... st u8:   l125 = UNIQUE | NON_NULL, (empty)
                // 929: b"usable %d\n\0": typeof(_78) = *const {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 929: b"usable %d\n\0":   l127 = UNIQUE | NON_NULL, (empty)
                // 929: b"usable %d\n\0": typeof(_79) = & {l129} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 929: b"usable %d\n\0":   l129 = UNIQUE | NON_NULL, FIXED
                // 929: b"usable %d\n\0 ... _char: typeof(_76 = move _77 as *const i8 (Misc)) = *const {l634} i8
                // 929: b"usable %d\n\0 ... _char:   l634 = UNIQUE | NON_NULL, (empty)
                // 929: b"usable %d\n\0 ... st u8: typeof(_77 = move _78 as *const u8 (Pointer(ArrayToPointer))) = *const {l633} u8
                // 929: b"usable %d\n\0 ... st u8:   l633 = UNIQUE | NON_NULL, (empty)
                // 929: b"usable %d\n\0": typeof(_79 = const b"usable %d\n\x00") = & {l631} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 929: b"usable %d\n\0":   l631 = UNIQUE | NON_NULL, (empty)
                // 929: b"usable %d\n\0": typeof(_78 = &raw const (*_79)) = *const {l632} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 929: b"usable %d\n\0":   l632 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        14 => {
            fprintf(
                file,
                // 935: file: typeof(_83) = *mut {l134} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 935: file:   l134 = UNIQUE | NON_NULL, (empty)
                b"repr %d\n\0" as *const u8 as *const libc::c_char,
                // 936: b"repr %d\n\0"  ... _char: typeof(_84) = *const {l136} i8
                // 936: b"repr %d\n\0"  ... _char:   l136 = UNIQUE | NON_NULL, (empty)
                // 936: b"repr %d\n\0"  ... st u8: typeof(_85) = *const {l138} u8
                // 936: b"repr %d\n\0"  ... st u8:   l138 = UNIQUE | NON_NULL, (empty)
                // 936: b"repr %d\n\0": typeof(_86) = *const {l140} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 936: b"repr %d\n\0":   l140 = UNIQUE | NON_NULL, (empty)
                // 936: b"repr %d\n\0": typeof(_87) = & {l142} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 936: b"repr %d\n\0":   l142 = UNIQUE | NON_NULL, FIXED
                // 936: b"repr %d\n\0"  ... st u8: typeof(_85 = move _86 as *const u8 (Pointer(ArrayToPointer))) = *const {l637} u8
                // 936: b"repr %d\n\0"  ... st u8:   l637 = UNIQUE | NON_NULL, (empty)
                // 936: b"repr %d\n\0": typeof(_87 = const b"repr %d\n\x00") = & {l635} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 936: b"repr %d\n\0":   l635 = UNIQUE | NON_NULL, (empty)
                // 936: b"repr %d\n\0"  ... _char: typeof(_84 = move _85 as *const i8 (Misc)) = *const {l638} i8
                // 936: b"repr %d\n\0"  ... _char:   l638 = UNIQUE | NON_NULL, (empty)
                // 936: b"repr %d\n\0": typeof(_86 = &raw const (*_87)) = *const {l636} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 936: b"repr %d\n\0":   l636 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        27 => {
            fprintf(file, b"fixate\n\0" as *const u8 as *const libc::c_char);
            // 941: file: typeof(_91) = *mut {l147} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 941: file:   l147 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0" a ... _char: typeof(_92) = *const {l149} i8
            // 941: b"fixate\n\0" a ... _char:   l149 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0" a ... st u8: typeof(_93) = *const {l151} u8
            // 941: b"fixate\n\0" a ... st u8:   l151 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0": typeof(_94) = *const {l153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 941: b"fixate\n\0":   l153 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0": typeof(_95) = & {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 941: b"fixate\n\0":   l155 = UNIQUE | NON_NULL, FIXED
            // 941: b"fixate\n\0" a ... st u8: typeof(_93 = move _94 as *const u8 (Pointer(ArrayToPointer))) = *const {l641} u8
            // 941: b"fixate\n\0" a ... st u8:   l641 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0": typeof(_95 = const b"fixate\n\x00") = & {l639} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 941: b"fixate\n\0":   l639 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0" a ... _char: typeof(_92 = move _93 as *const i8 (Misc)) = *const {l642} i8
            // 941: b"fixate\n\0" a ... _char:   l642 = UNIQUE | NON_NULL, (empty)
            // 941: b"fixate\n\0": typeof(_94 = &raw const (*_95)) = *const {l640} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 941: b"fixate\n\0":   l640 = UNIQUE | NON_NULL, (empty)
        }
        20 => {
            fprintf(file, b"reduce\n\0" as *const u8 as *const libc::c_char);
            // 944: file: typeof(_97) = *mut {l158} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 944: file:   l158 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0" a ... _char: typeof(_98) = *const {l160} i8
            // 944: b"reduce\n\0" a ... _char:   l160 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0" a ... st u8: typeof(_99) = *const {l162} u8
            // 944: b"reduce\n\0" a ... st u8:   l162 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0": typeof(_100) = *const {l164} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 944: b"reduce\n\0":   l164 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0": typeof(_101) = & {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 944: b"reduce\n\0":   l166 = UNIQUE | NON_NULL, FIXED
            // 944: b"reduce\n\0": typeof(_101 = const b"reduce\n\x00") = & {l643} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 944: b"reduce\n\0":   l643 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0" a ... _char: typeof(_98 = move _99 as *const i8 (Misc)) = *const {l646} i8
            // 944: b"reduce\n\0" a ... _char:   l646 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0": typeof(_100 = &raw const (*_101)) = *const {l644} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 944: b"reduce\n\0":   l644 = UNIQUE | NON_NULL, (empty)
            // 944: b"reduce\n\0" a ... st u8: typeof(_99 = move _100 as *const u8 (Pointer(ArrayToPointer))) = *const {l645} u8
            // 944: b"reduce\n\0" a ... st u8:   l645 = UNIQUE | NON_NULL, (empty)
        }
        19 => {
            fprintf(file, b"flush\n\0" as *const u8 as *const libc::c_char);
            // 947: file: typeof(_103) = *mut {l169} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 947: file:   l169 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0" as ... _char: typeof(_104) = *const {l171} i8
            // 947: b"flush\n\0" as ... _char:   l171 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0" as ... st u8: typeof(_105) = *const {l173} u8
            // 947: b"flush\n\0" as ... st u8:   l173 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0": typeof(_106) = *const {l175} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 947: b"flush\n\0":   l175 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0": typeof(_107) = & {l177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 947: b"flush\n\0":   l177 = UNIQUE | NON_NULL, FIXED
            // 947: b"flush\n\0" as ... st u8: typeof(_105 = move _106 as *const u8 (Pointer(ArrayToPointer))) = *const {l649} u8
            // 947: b"flush\n\0" as ... st u8:   l649 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0": typeof(_107 = const b"flush\n\x00") = & {l647} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 947: b"flush\n\0":   l647 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0": typeof(_106 = &raw const (*_107)) = *const {l648} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 947: b"flush\n\0":   l648 = UNIQUE | NON_NULL, (empty)
            // 947: b"flush\n\0" as ... _char: typeof(_104 = move _105 as *const i8 (Misc)) = *const {l650} i8
            // 947: b"flush\n\0" as ... _char:   l650 = UNIQUE | NON_NULL, (empty)
        }
        15 => {
            fprintf(
                file,
                // 951: file: typeof(_109) = *mut {l180} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 951: file:   l180 = UNIQUE | NON_NULL, (empty)
                b"setimportant %d\n\0" as *const u8 as *const libc::c_char,
                // 952: b"setimportant  ... _char: typeof(_110) = *const {l182} i8
                // 952: b"setimportant  ... _char:   l182 = UNIQUE | NON_NULL, (empty)
                // 952: b"setimportant  ... st u8: typeof(_111) = *const {l184} u8
                // 952: b"setimportant  ... st u8:   l184 = UNIQUE | NON_NULL, (empty)
                // 952: b"setimportant  ... \n\0": typeof(_112) = *const {l186} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 952: b"setimportant  ... \n\0":   l186 = UNIQUE | NON_NULL, (empty)
                // 952: b"setimportant  ... \n\0": typeof(_113) = & {l188} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 952: b"setimportant  ... \n\0":   l188 = UNIQUE | NON_NULL, FIXED
                // 952: b"setimportant  ... \n\0": typeof(_113 = const b"setimportant %d\n\x00") = & {l651} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 952: b"setimportant  ... \n\0":   l651 = UNIQUE | NON_NULL, (empty)
                // 952: b"setimportant  ... \n\0": typeof(_112 = &raw const (*_113)) = *const {l652} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 952: b"setimportant  ... \n\0":   l652 = UNIQUE | NON_NULL, (empty)
                // 952: b"setimportant  ... _char: typeof(_110 = move _111 as *const i8 (Misc)) = *const {l654} i8
                // 952: b"setimportant  ... _char:   l654 = UNIQUE | NON_NULL, (empty)
                // 952: b"setimportant  ... st u8: typeof(_111 = move _112 as *const u8 (Pointer(ArrayToPointer))) = *const {l653} u8
                // 952: b"setimportant  ... st u8:   l653 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        18 => {
            fprintf(file, b"setphases\n\0" as *const u8 as *const libc::c_char);
            // 957: file: typeof(_117) = *mut {l193} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 957: file:   l193 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0 ... _char: typeof(_118) = *const {l195} i8
            // 957: b"setphases\n\0 ... _char:   l195 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0 ... st u8: typeof(_119) = *const {l197} u8
            // 957: b"setphases\n\0 ... st u8:   l197 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0": typeof(_120) = *const {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 957: b"setphases\n\0":   l199 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0": typeof(_121) = & {l201} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 957: b"setphases\n\0":   l201 = UNIQUE | NON_NULL, FIXED
            // 957: b"setphases\n\0": typeof(_121 = const b"setphases\n\x00") = & {l655} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 957: b"setphases\n\0":   l655 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0 ... _char: typeof(_118 = move _119 as *const i8 (Misc)) = *const {l658} i8
            // 957: b"setphases\n\0 ... _char:   l658 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0 ... st u8: typeof(_119 = move _120 as *const u8 (Pointer(ArrayToPointer))) = *const {l657} u8
            // 957: b"setphases\n\0 ... st u8:   l657 = UNIQUE | NON_NULL, (empty)
            // 957: b"setphases\n\0": typeof(_120 = &raw const (*_121)) = *const {l656} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 957: b"setphases\n\0":   l656 = UNIQUE | NON_NULL, (empty)
        }
        16 => {
            fprintf(
                file,
                // 961: file: typeof(_123) = *mut {l204} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 961: file:   l204 = UNIQUE | NON_NULL, (empty)
                b"setphase %d\n\0" as *const u8 as *const libc::c_char,
                // 962: b"setphase %d\n ... _char: typeof(_124) = *const {l206} i8
                // 962: b"setphase %d\n ... _char:   l206 = UNIQUE | NON_NULL, (empty)
                // 962: b"setphase %d\n ... st u8: typeof(_125) = *const {l208} u8
                // 962: b"setphase %d\n ... st u8:   l208 = UNIQUE | NON_NULL, (empty)
                // 962: b"setphase %d\n\0": typeof(_126) = *const {l210} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 962: b"setphase %d\n\0":   l210 = UNIQUE | NON_NULL, (empty)
                // 962: b"setphase %d\n\0": typeof(_127) = & {l212} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 962: b"setphase %d\n\0":   l212 = UNIQUE | NON_NULL, FIXED
                // 962: b"setphase %d\n ... st u8: typeof(_125 = move _126 as *const u8 (Pointer(ArrayToPointer))) = *const {l661} u8
                // 962: b"setphase %d\n ... st u8:   l661 = UNIQUE | NON_NULL, (empty)
                // 962: b"setphase %d\n ... _char: typeof(_124 = move _125 as *const i8 (Misc)) = *const {l662} i8
                // 962: b"setphase %d\n ... _char:   l662 = UNIQUE | NON_NULL, (empty)
                // 962: b"setphase %d\n\0": typeof(_127 = const b"setphase %d\n\x00") = & {l659} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 962: b"setphase %d\n\0":   l659 = UNIQUE | NON_NULL, (empty)
                // 962: b"setphase %d\n\0": typeof(_126 = &raw const (*_127)) = *const {l660} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 962: b"setphase %d\n\0":   l660 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        17 => {
            fprintf(
                file,
                // 968: file: typeof(_131) = *mut {l217} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 968: file:   l217 = UNIQUE | NON_NULL, (empty)
                b"resetphase %d\n\0" as *const u8 as *const libc::c_char,
                // 969: b"resetphase %d ... _char: typeof(_132) = *const {l219} i8
                // 969: b"resetphase %d ... _char:   l219 = UNIQUE | NON_NULL, (empty)
                // 969: b"resetphase %d ... st u8: typeof(_133) = *const {l221} u8
                // 969: b"resetphase %d ... st u8:   l221 = UNIQUE | NON_NULL, (empty)
                // 969: b"resetphase %d\n\0": typeof(_134) = *const {l223} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 969: b"resetphase %d\n\0":   l223 = UNIQUE | NON_NULL, (empty)
                // 969: b"resetphase %d\n\0": typeof(_135) = & {l225} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 969: b"resetphase %d\n\0":   l225 = UNIQUE | NON_NULL, FIXED
                // 969: b"resetphase %d\n\0": typeof(_134 = &raw const (*_135)) = *const {l664} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 969: b"resetphase %d\n\0":   l664 = UNIQUE | NON_NULL, (empty)
                // 969: b"resetphase %d ... _char: typeof(_132 = move _133 as *const i8 (Misc)) = *const {l666} i8
                // 969: b"resetphase %d ... _char:   l666 = UNIQUE | NON_NULL, (empty)
                // 969: b"resetphase %d\n\0": typeof(_135 = const b"resetphase %d\n\x00") = & {l663} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 969: b"resetphase %d\n\0":   l663 = UNIQUE | NON_NULL, (empty)
                // 969: b"resetphase %d ... st u8: typeof(_133 = move _134 as *const u8 (Pointer(ArrayToPointer))) = *const {l665} u8
                // 969: b"resetphase %d ... st u8:   l665 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        4 => {
            fprintf(
                file,
                // 975: file: typeof(_139) = *mut {l230} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 975: file:   l230 = UNIQUE | NON_NULL, (empty)
                b"freeze %d\n\0" as *const u8 as *const libc::c_char,
                // 976: b"freeze %d\n\0 ... _char: typeof(_140) = *const {l232} i8
                // 976: b"freeze %d\n\0 ... _char:   l232 = UNIQUE | NON_NULL, (empty)
                // 976: b"freeze %d\n\0 ... st u8: typeof(_141) = *const {l234} u8
                // 976: b"freeze %d\n\0 ... st u8:   l234 = UNIQUE | NON_NULL, (empty)
                // 976: b"freeze %d\n\0": typeof(_142) = *const {l236} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 976: b"freeze %d\n\0":   l236 = UNIQUE | NON_NULL, (empty)
                // 976: b"freeze %d\n\0": typeof(_143) = & {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 976: b"freeze %d\n\0":   l238 = UNIQUE | NON_NULL, FIXED
                // 976: b"freeze %d\n\0 ... st u8: typeof(_141 = move _142 as *const u8 (Pointer(ArrayToPointer))) = *const {l669} u8
                // 976: b"freeze %d\n\0 ... st u8:   l669 = UNIQUE | NON_NULL, (empty)
                // 976: b"freeze %d\n\0 ... _char: typeof(_140 = move _141 as *const i8 (Misc)) = *const {l670} i8
                // 976: b"freeze %d\n\0 ... _char:   l670 = UNIQUE | NON_NULL, (empty)
                // 976: b"freeze %d\n\0": typeof(_143 = const b"freeze %d\n\x00") = & {l667} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 976: b"freeze %d\n\0":   l667 = UNIQUE | NON_NULL, (empty)
                // 976: b"freeze %d\n\0": typeof(_142 = &raw const (*_143)) = *const {l668} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 976: b"freeze %d\n\0":   l668 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        30 => {
            fprintf(
                file,
                // 982: file: typeof(_147) = *mut {l243} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 982: file:   l243 = UNIQUE | NON_NULL, (empty)
                b"inconsistent\n\0" as *const u8 as *const libc::c_char,
                // 983: b"inconsistent\ ... _char: typeof(_148) = *const {l245} i8
                // 983: b"inconsistent\ ... _char:   l245 = UNIQUE | NON_NULL, (empty)
                // 983: b"inconsistent\ ... st u8: typeof(_149) = *const {l247} u8
                // 983: b"inconsistent\ ... st u8:   l247 = UNIQUE | NON_NULL, (empty)
                // 983: b"inconsistent\n\0": typeof(_150) = *const {l249} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 983: b"inconsistent\n\0":   l249 = UNIQUE | NON_NULL, (empty)
                // 983: b"inconsistent\n\0": typeof(_151) = & {l251} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 983: b"inconsistent\n\0":   l251 = UNIQUE | NON_NULL, FIXED
                // 983: b"inconsistent\n\0": typeof(_150 = &raw const (*_151)) = *const {l672} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 983: b"inconsistent\n\0":   l672 = UNIQUE | NON_NULL, (empty)
                // 983: b"inconsistent\ ... st u8: typeof(_149 = move _150 as *const u8 (Pointer(ArrayToPointer))) = *const {l673} u8
                // 983: b"inconsistent\ ... st u8:   l673 = UNIQUE | NON_NULL, (empty)
                // 983: b"inconsistent\n\0": typeof(_151 = const b"inconsistent\n\x00") = & {l671} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 983: b"inconsistent\n\0":   l671 = UNIQUE | NON_NULL, (empty)
                // 983: b"inconsistent\ ... _char: typeof(_148 = move _149 as *const i8 (Misc)) = *const {l674} i8
                // 983: b"inconsistent\ ... _char:   l674 = UNIQUE | NON_NULL, (empty)
            );
        }
        31 => {
            fprintf(file, b"lkhd\n\0" as *const u8 as *const libc::c_char);
            // 987: file: typeof(_153) = *mut {l254} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 987: file:   l254 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0" as  ... _char: typeof(_154) = *const {l256} i8
            // 987: b"lkhd\n\0" as  ... _char:   l256 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0" as  ... st u8: typeof(_155) = *const {l258} u8
            // 987: b"lkhd\n\0" as  ... st u8:   l258 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0": typeof(_156) = *const {l260} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 987: b"lkhd\n\0":   l260 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0": typeof(_157) = & {l262} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 987: b"lkhd\n\0":   l262 = UNIQUE | NON_NULL, FIXED
            // 987: b"lkhd\n\0": typeof(_156 = &raw const (*_157)) = *const {l676} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 987: b"lkhd\n\0":   l676 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0": typeof(_157 = const b"lkhd\n\x00") = & {l675} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 987: b"lkhd\n\0":   l675 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0" as  ... st u8: typeof(_155 = move _156 as *const u8 (Pointer(ArrayToPointer))) = *const {l677} u8
            // 987: b"lkhd\n\0" as  ... st u8:   l677 = UNIQUE | NON_NULL, (empty)
            // 987: b"lkhd\n\0" as  ... _char: typeof(_154 = move _155 as *const i8 (Misc)) = *const {l678} i8
            // 987: b"lkhd\n\0" as  ... _char:   l678 = UNIQUE | NON_NULL, (empty)
        }
        25 => {
            fprintf(file, b"incvar\n\0" as *const u8 as *const libc::c_char);
            // 990: file: typeof(_159) = *mut {l265} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 990: file:   l265 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0" a ... _char: typeof(_160) = *const {l267} i8
            // 990: b"incvar\n\0" a ... _char:   l267 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0" a ... st u8: typeof(_161) = *const {l269} u8
            // 990: b"incvar\n\0" a ... st u8:   l269 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0": typeof(_162) = *const {l271} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 990: b"incvar\n\0":   l271 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0": typeof(_163) = & {l273} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 990: b"incvar\n\0":   l273 = UNIQUE | NON_NULL, FIXED
            // 990: b"incvar\n\0": typeof(_163 = const b"incvar\n\x00") = & {l679} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 990: b"incvar\n\0":   l679 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0": typeof(_162 = &raw const (*_163)) = *const {l680} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 990: b"incvar\n\0":   l680 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0" a ... _char: typeof(_160 = move _161 as *const i8 (Misc)) = *const {l682} i8
            // 990: b"incvar\n\0" a ... _char:   l682 = UNIQUE | NON_NULL, (empty)
            // 990: b"incvar\n\0" a ... st u8: typeof(_161 = move _162 as *const u8 (Pointer(ArrayToPointer))) = *const {l681} u8
            // 990: b"incvar\n\0" a ... st u8:   l681 = UNIQUE | NON_NULL, (empty)
        }
        5 => {
            fprintf(file, b"init\n\0" as *const u8 as *const libc::c_char);
            // 993: file: typeof(_165) = *mut {l276} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 993: file:   l276 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0" as  ... _char: typeof(_166) = *const {l278} i8
            // 993: b"init\n\0" as  ... _char:   l278 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0" as  ... st u8: typeof(_167) = *const {l280} u8
            // 993: b"init\n\0" as  ... st u8:   l280 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0": typeof(_168) = *const {l282} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 993: b"init\n\0":   l282 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0": typeof(_169) = & {l284} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 993: b"init\n\0":   l284 = UNIQUE | NON_NULL, FIXED
            // 993: b"init\n\0": typeof(_168 = &raw const (*_169)) = *const {l684} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 993: b"init\n\0":   l684 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0" as  ... st u8: typeof(_167 = move _168 as *const u8 (Pointer(ArrayToPointer))) = *const {l685} u8
            // 993: b"init\n\0" as  ... st u8:   l685 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0": typeof(_169 = const b"init\n\x00") = & {l683} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 993: b"init\n\0":   l683 = UNIQUE | NON_NULL, (empty)
            // 993: b"init\n\0" as  ... _char: typeof(_166 = move _167 as *const i8 (Misc)) = *const {l686} i8
            // 993: b"init\n\0" as  ... _char:   l686 = UNIQUE | NON_NULL, (empty)
            if !opts.is_null() {
            // 994: opts: typeof(_172) = *mut {l288} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 994: opts:   l288 = UNIQUE | NON_NULL, (empty)
            // 994: opts: typeof(_173) = *mut {l290} *mut {l291} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 994: opts:   l290 = UNIQUE | NON_NULL, (empty)
            // 994: opts:   l291 = UNIQUE | NON_NULL, (empty)
                if ddopts != 0 {
                // 995: ddopts: typeof(_177) = *mut {l296} i32
                // 995: ddopts:   l296 = UNIQUE | NON_NULL, (empty)
                } else {
                    __assert_fail(
                        b"ddopts\0" as *const u8 as *const libc::c_char,
                        // 998: b"ddopts\0" as  ... _char: typeof(_180) = *const {l300} i8
                        // 998: b"ddopts\0" as  ... _char:   l300 = UNIQUE | NON_NULL, (empty)
                        // 998: b"ddopts\0" as  ... st u8: typeof(_181) = *const {l302} u8
                        // 998: b"ddopts\0" as  ... st u8:   l302 = UNIQUE | NON_NULL, (empty)
                        // 998: b"ddopts\0": typeof(_182) = *const {l304} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                        // 998: b"ddopts\0":   l304 = UNIQUE | NON_NULL, (empty)
                        // 998: b"ddopts\0": typeof(_183) = & {l306} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                        // 998: b"ddopts\0":   l306 = UNIQUE | NON_NULL, FIXED
                        // 998: b"ddopts\0" as  ... _char: typeof(_180 = move _181 as *const i8 (Misc)) = *const {l690} i8
                        // 998: b"ddopts\0" as  ... _char:   l690 = UNIQUE | NON_NULL, (empty)
                        // 998: b"ddopts\0" as  ... st u8: typeof(_181 = move _182 as *const u8 (Pointer(ArrayToPointer))) = *const {l689} u8
                        // 998: b"ddopts\0" as  ... st u8:   l689 = UNIQUE | NON_NULL, (empty)
                        // 998: b"ddopts\0": typeof(_183 = const b"ddopts\x00") = & {l687} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                        // 998: b"ddopts\0":   l687 = UNIQUE | NON_NULL, (empty)
                        // 998: b"ddopts\0": typeof(_182 = &raw const (*_183)) = *const {l688} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                        // 998: b"ddopts\0":   l688 = UNIQUE | NON_NULL, (empty)
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        // 999: b"lglddtrace.c\ ... _char: typeof(_184) = *const {l308} i8
                        // 999: b"lglddtrace.c\ ... _char:   l308 = UNIQUE | NON_NULL, (empty)
                        // 999: b"lglddtrace.c\ ... st u8: typeof(_185) = *const {l310} u8
                        // 999: b"lglddtrace.c\ ... st u8:   l310 = UNIQUE | NON_NULL, (empty)
                        // 999: b"lglddtrace.c\0": typeof(_186) = *const {l312} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 999: b"lglddtrace.c\0":   l312 = UNIQUE | NON_NULL, (empty)
                        // 999: b"lglddtrace.c\0": typeof(_187) = & {l314} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 999: b"lglddtrace.c\0":   l314 = UNIQUE | NON_NULL, FIXED
                        // 999: b"lglddtrace.c\0": typeof(_186 = &raw const (*_187)) = *const {l692} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 999: b"lglddtrace.c\0":   l692 = UNIQUE | NON_NULL, (empty)
                        // 999: b"lglddtrace.c\0": typeof(_187 = const b"lglddtrace.c\x00") = & {l691} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 999: b"lglddtrace.c\0":   l691 = UNIQUE | NON_NULL, (empty)
                        // 999: b"lglddtrace.c\ ... st u8: typeof(_185 = move _186 as *const u8 (Pointer(ArrayToPointer))) = *const {l693} u8
                        // 999: b"lglddtrace.c\ ... st u8:   l693 = UNIQUE | NON_NULL, (empty)
                        // 999: b"lglddtrace.c\ ... _char: typeof(_184 = move _185 as *const i8 (Misc)) = *const {l694} i8
                        // 999: b"lglddtrace.c\ ... _char:   l694 = UNIQUE | NON_NULL, (empty)
                        302 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        // 1001: (*::core::mem:: ... ptr(): typeof(_190) = *const {l318} i8
                        // 1001: (*::core::mem:: ... ptr():   l318 = UNIQUE | NON_NULL, (empty)
                        // 1001: (*::core::mem:: ... ptr(): typeof(_191) = & {l320} [i8]
                        // 1001: (*::core::mem:: ... ptr():   l320 = UNIQUE | NON_NULL, FIXED
                        // 1001: (*::core::mem:: ... ptr(): typeof(_192) = & {l322} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1001: (*::core::mem:: ... ptr():   l322 = UNIQUE | NON_NULL, (empty)
                        // 1001: ::core::mem::tr ... 0", ): typeof(_193) = & {l324} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1001: ::core::mem::tr ... 0", ):   l324 = UNIQUE | NON_NULL, FIXED
                        // 1001: (*::core::mem:: ... ptr(): typeof(_191 = move _192 as &[i8] (Pointer(Unsize))) = & {l698} [i8]
                        // 1001: (*::core::mem:: ... ptr():   l698 = UNIQUE | NON_NULL, FIXED
                        // 1001: (*::core::mem:: ... ptr(): typeof(_192 = &(*_193)) = & {l697} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1001: (*::core::mem:: ... ptr():   l697 = UNIQUE | NON_NULL, (empty)
                            b"void print(Event *, FILE *)\0",
                            // 1002: b"void print(Ev ... *)\0": typeof(_194) = & {l326} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1002: b"void print(Ev ... *)\0":   l326 = UNIQUE | NON_NULL, (empty)
                            // 1002: b"void print(Ev ... *)\0": typeof(_195) = & {l328} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1002: b"void print(Ev ... *)\0":   l328 = UNIQUE | NON_NULL, FIXED
                            // 1002: b"void print(Ev ... *)\0": typeof(_195 = const b"void print(Event *, FILE *)\x00") = & {l695} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1002: b"void print(Ev ... *)\0":   l695 = UNIQUE | NON_NULL, (empty)
                            // 1002: b"void print(Ev ... *)\0": typeof(_194 = &(*_195)) = & {l696} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1002: b"void print(Ev ... *)\0":   l696 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
                'c_6595: {
                    if ddopts != 0 {
                    // 1008: ddopts: typeof(_199) = *mut {l333} i32
                    // 1008: ddopts:   l333 = UNIQUE | NON_NULL, (empty)
                    } else {
                        __assert_fail(
                            b"ddopts\0" as *const u8 as *const libc::c_char,
                            // 1011: b"ddopts\0" as  ... _char: typeof(_202) = *const {l337} i8
                            // 1011: b"ddopts\0" as  ... _char:   l337 = UNIQUE | NON_NULL, (empty)
                            // 1011: b"ddopts\0" as  ... st u8: typeof(_203) = *const {l339} u8
                            // 1011: b"ddopts\0" as  ... st u8:   l339 = UNIQUE | NON_NULL, (empty)
                            // 1011: b"ddopts\0": typeof(_204) = *const {l341} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                            // 1011: b"ddopts\0":   l341 = UNIQUE | NON_NULL, (empty)
                            // 1011: b"ddopts\0": typeof(_205) = & {l343} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                            // 1011: b"ddopts\0":   l343 = UNIQUE | NON_NULL, FIXED
                            // 1011: b"ddopts\0": typeof(_204 = &raw const (*_205)) = *const {l700} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                            // 1011: b"ddopts\0":   l700 = UNIQUE | NON_NULL, (empty)
                            // 1011: b"ddopts\0" as  ... _char: typeof(_202 = move _203 as *const i8 (Misc)) = *const {l702} i8
                            // 1011: b"ddopts\0" as  ... _char:   l702 = UNIQUE | NON_NULL, (empty)
                            // 1011: b"ddopts\0" as  ... st u8: typeof(_203 = move _204 as *const u8 (Pointer(ArrayToPointer))) = *const {l701} u8
                            // 1011: b"ddopts\0" as  ... st u8:   l701 = UNIQUE | NON_NULL, (empty)
                            // 1011: b"ddopts\0": typeof(_205 = const b"ddopts\x00") = & {l699} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                            // 1011: b"ddopts\0":   l699 = UNIQUE | NON_NULL, (empty)
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            // 1012: b"lglddtrace.c\ ... _char: typeof(_206) = *const {l345} i8
                            // 1012: b"lglddtrace.c\ ... _char:   l345 = UNIQUE | NON_NULL, (empty)
                            // 1012: b"lglddtrace.c\ ... st u8: typeof(_207) = *const {l347} u8
                            // 1012: b"lglddtrace.c\ ... st u8:   l347 = UNIQUE | NON_NULL, (empty)
                            // 1012: b"lglddtrace.c\0": typeof(_208) = *const {l349} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1012: b"lglddtrace.c\0":   l349 = UNIQUE | NON_NULL, (empty)
                            // 1012: b"lglddtrace.c\0": typeof(_209) = & {l351} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1012: b"lglddtrace.c\0":   l351 = UNIQUE | NON_NULL, FIXED
                            // 1012: b"lglddtrace.c\ ... _char: typeof(_206 = move _207 as *const i8 (Misc)) = *const {l706} i8
                            // 1012: b"lglddtrace.c\ ... _char:   l706 = UNIQUE | NON_NULL, (empty)
                            // 1012: b"lglddtrace.c\0": typeof(_209 = const b"lglddtrace.c\x00") = & {l703} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1012: b"lglddtrace.c\0":   l703 = UNIQUE | NON_NULL, (empty)
                            // 1012: b"lglddtrace.c\ ... st u8: typeof(_207 = move _208 as *const u8 (Pointer(ArrayToPointer))) = *const {l705} u8
                            // 1012: b"lglddtrace.c\ ... st u8:   l705 = UNIQUE | NON_NULL, (empty)
                            // 1012: b"lglddtrace.c\0": typeof(_208 = &raw const (*_209)) = *const {l704} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1012: b"lglddtrace.c\0":   l704 = UNIQUE | NON_NULL, (empty)
                            302 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                            // 1014: (*::core::mem:: ... ptr(): typeof(_212) = *const {l355} i8
                            // 1014: (*::core::mem:: ... ptr():   l355 = UNIQUE | NON_NULL, (empty)
                            // 1014: (*::core::mem:: ... ptr(): typeof(_213) = & {l357} [i8]
                            // 1014: (*::core::mem:: ... ptr():   l357 = UNIQUE | NON_NULL, FIXED
                            // 1014: (*::core::mem:: ... ptr(): typeof(_214) = & {l359} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1014: (*::core::mem:: ... ptr():   l359 = UNIQUE | NON_NULL, (empty)
                            // 1014: ::core::mem::tr ... 0", ): typeof(_215) = & {l361} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1014: ::core::mem::tr ... 0", ):   l361 = UNIQUE | NON_NULL, FIXED
                            // 1014: (*::core::mem:: ... ptr(): typeof(_214 = &(*_215)) = & {l709} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1014: (*::core::mem:: ... ptr():   l709 = UNIQUE | NON_NULL, (empty)
                            // 1014: (*::core::mem:: ... ptr(): typeof(_213 = move _214 as &[i8] (Pointer(Unsize))) = & {l710} [i8]
                            // 1014: (*::core::mem:: ... ptr():   l710 = UNIQUE | NON_NULL, FIXED
                                b"void print(Event *, FILE *)\0",
                                // 1015: b"void print(Ev ... *)\0": typeof(_216) = & {l363} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                                // 1015: b"void print(Ev ... *)\0":   l363 = UNIQUE | NON_NULL, (empty)
                                // 1015: b"void print(Ev ... *)\0": typeof(_217) = & {l365} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                                // 1015: b"void print(Ev ... *)\0":   l365 = UNIQUE | NON_NULL, FIXED
                                // 1015: b"void print(Ev ... *)\0": typeof(_216 = &(*_217)) = & {l708} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                                // 1015: b"void print(Ev ... *)\0":   l708 = UNIQUE | NON_NULL, (empty)
                                // 1015: b"void print(Ev ... *)\0": typeof(_217 = const b"void print(Event *, FILE *)\x00") = & {l707} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                                // 1015: b"void print(Ev ... *)\0":   l707 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                };
                o = opts;
                // 1021: opts: typeof(_218) = *mut {l367} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 1021: opts:   l367 = UNIQUE | NON_NULL, (empty)
                // 1021: opts: typeof(_219) = *mut {l369} *mut {l370} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 1021: opts:   l369 = UNIQUE | NON_NULL, (empty)
                // 1021: opts:   l370 = UNIQUE | NON_NULL, (empty)
                while o < opts.offset(nopts as isize) {
                // 1022: o: typeof(_222) = *mut {l374} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 1022: o:   l374 = UNIQUE | NON_NULL, (empty)
                // 1022: opts.offset(nop ... size): typeof(_223) = *mut {l376} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 1022: opts.offset(nop ... size):   l376 = UNIQUE | NON_NULL, (empty)
                // 1022: opts: typeof(_224) = *mut {l378} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 1022: opts:   l378 = UNIQUE | NON_NULL, (empty)
                // 1022: opts: typeof(_225) = *mut {l380} *mut {l381} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 1022: opts:   l380 = UNIQUE | NON_NULL, (empty)
                // 1022: opts:   l381 = UNIQUE | NON_NULL, (empty)
                // 1022: nopts: typeof(_228) = *mut {l385} i32
                // 1022: nopts:   l385 = UNIQUE | NON_NULL, (empty)
                    fprintf(
                        file,
                        // 1024: file: typeof(_230) = *mut {l388} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                        // 1024: file:   l388 = UNIQUE | NON_NULL, (empty)
                        b"option %s %d\n\0" as *const u8 as *const libc::c_char,
                        // 1025: b"option %s %d\ ... _char: typeof(_231) = *const {l390} i8
                        // 1025: b"option %s %d\ ... _char:   l390 = UNIQUE | NON_NULL, (empty)
                        // 1025: b"option %s %d\ ... st u8: typeof(_232) = *const {l392} u8
                        // 1025: b"option %s %d\ ... st u8:   l392 = UNIQUE | NON_NULL, (empty)
                        // 1025: b"option %s %d\n\0": typeof(_233) = *const {l394} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1025: b"option %s %d\n\0":   l394 = UNIQUE | NON_NULL, (empty)
                        // 1025: b"option %s %d\n\0": typeof(_234) = & {l396} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1025: b"option %s %d\n\0":   l396 = UNIQUE | NON_NULL, FIXED
                        // 1025: b"option %s %d\ ... st u8: typeof(_232 = move _233 as *const u8 (Pointer(ArrayToPointer))) = *const {l713} u8
                        // 1025: b"option %s %d\ ... st u8:   l713 = UNIQUE | NON_NULL, (empty)
                        // 1025: b"option %s %d\ ... _char: typeof(_231 = move _232 as *const i8 (Misc)) = *const {l714} i8
                        // 1025: b"option %s %d\ ... _char:   l714 = UNIQUE | NON_NULL, (empty)
                        // 1025: b"option %s %d\n\0": typeof(_233 = &raw const (*_234)) = *const {l712} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1025: b"option %s %d\n\0":   l712 = UNIQUE | NON_NULL, (empty)
                        // 1025: b"option %s %d\n\0": typeof(_234 = const b"option %s %d\n\x00") = & {l711} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1025: b"option %s %d\n\0":   l711 = UNIQUE | NON_NULL, (empty)
                        (*o).name,
                        // 1026: (*o).name: typeof(_235) = *mut {l398} i8
                        // 1026: (*o).name:   l398 = UNIQUE | NON_NULL, (empty)
                        (*o).val,
                    );
                    o = o.offset(1);
                    // 1029: o.offset(1): typeof(_237) = *mut {l401} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1029: o.offset(1):   l401 = UNIQUE | NON_NULL, (empty)
                    // 1029: o: typeof(_238) = *mut {l403} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1029: o:   l403 = UNIQUE | NON_NULL, (empty)
                    o;
                    // 1030: o: typeof(_239) = *mut {l405} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1030: o:   l405 = UNIQUE | NON_NULL, (empty)
                }
            }
        }
        24 => {
            fprintf(file, b"maxvar\n\0" as *const u8 as *const libc::c_char);
            // 1035: file: typeof(_244) = *mut {l411} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 1035: file:   l411 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0" a ... _char: typeof(_245) = *const {l413} i8
            // 1035: b"maxvar\n\0" a ... _char:   l413 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0" a ... st u8: typeof(_246) = *const {l415} u8
            // 1035: b"maxvar\n\0" a ... st u8:   l415 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0": typeof(_247) = *const {l417} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1035: b"maxvar\n\0":   l417 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0": typeof(_248) = & {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1035: b"maxvar\n\0":   l419 = UNIQUE | NON_NULL, FIXED
            // 1035: b"maxvar\n\0" a ... _char: typeof(_245 = move _246 as *const i8 (Misc)) = *const {l718} i8
            // 1035: b"maxvar\n\0" a ... _char:   l718 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0": typeof(_248 = const b"maxvar\n\x00") = & {l715} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1035: b"maxvar\n\0":   l715 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0" a ... st u8: typeof(_246 = move _247 as *const u8 (Pointer(ArrayToPointer))) = *const {l717} u8
            // 1035: b"maxvar\n\0" a ... st u8:   l717 = UNIQUE | NON_NULL, (empty)
            // 1035: b"maxvar\n\0": typeof(_247 = &raw const (*_248)) = *const {l716} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1035: b"maxvar\n\0":   l716 = UNIQUE | NON_NULL, (empty)
        }
        6 => {
            fprintf(
                file,
                // 1039: file: typeof(_250) = *mut {l422} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 1039: file:   l422 = UNIQUE | NON_NULL, (empty)
                b"melt %d\n\0" as *const u8 as *const libc::c_char,
                // 1040: b"melt %d\n\0"  ... _char: typeof(_251) = *const {l424} i8
                // 1040: b"melt %d\n\0"  ... _char:   l424 = UNIQUE | NON_NULL, (empty)
                // 1040: b"melt %d\n\0"  ... st u8: typeof(_252) = *const {l426} u8
                // 1040: b"melt %d\n\0"  ... st u8:   l426 = UNIQUE | NON_NULL, (empty)
                // 1040: b"melt %d\n\0": typeof(_253) = *const {l428} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1040: b"melt %d\n\0":   l428 = UNIQUE | NON_NULL, (empty)
                // 1040: b"melt %d\n\0": typeof(_254) = & {l430} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1040: b"melt %d\n\0":   l430 = UNIQUE | NON_NULL, FIXED
                // 1040: b"melt %d\n\0"  ... _char: typeof(_251 = move _252 as *const i8 (Misc)) = *const {l722} i8
                // 1040: b"melt %d\n\0"  ... _char:   l722 = UNIQUE | NON_NULL, (empty)
                // 1040: b"melt %d\n\0": typeof(_254 = const b"melt %d\n\x00") = & {l719} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1040: b"melt %d\n\0":   l719 = UNIQUE | NON_NULL, (empty)
                // 1040: b"melt %d\n\0": typeof(_253 = &raw const (*_254)) = *const {l720} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1040: b"melt %d\n\0":   l720 = UNIQUE | NON_NULL, (empty)
                // 1040: b"melt %d\n\0"  ... st u8: typeof(_252 = move _253 as *const u8 (Pointer(ArrayToPointer))) = *const {l721} u8
                // 1040: b"melt %d\n\0"  ... st u8:   l721 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        7 => {
            fprintf(
                file,
                // 1046: file: typeof(_258) = *mut {l435} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 1046: file:   l435 = UNIQUE | NON_NULL, (empty)
                b"reuse %d\n\0" as *const u8 as *const libc::c_char,
                // 1047: b"reuse %d\n\0" ... _char: typeof(_259) = *const {l437} i8
                // 1047: b"reuse %d\n\0" ... _char:   l437 = UNIQUE | NON_NULL, (empty)
                // 1047: b"reuse %d\n\0" ... st u8: typeof(_260) = *const {l439} u8
                // 1047: b"reuse %d\n\0" ... st u8:   l439 = UNIQUE | NON_NULL, (empty)
                // 1047: b"reuse %d\n\0": typeof(_261) = *const {l441} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1047: b"reuse %d\n\0":   l441 = UNIQUE | NON_NULL, (empty)
                // 1047: b"reuse %d\n\0": typeof(_262) = & {l443} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1047: b"reuse %d\n\0":   l443 = UNIQUE | NON_NULL, FIXED
                // 1047: b"reuse %d\n\0": typeof(_261 = &raw const (*_262)) = *const {l724} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1047: b"reuse %d\n\0":   l724 = UNIQUE | NON_NULL, (empty)
                // 1047: b"reuse %d\n\0" ... _char: typeof(_259 = move _260 as *const i8 (Misc)) = *const {l726} i8
                // 1047: b"reuse %d\n\0" ... _char:   l726 = UNIQUE | NON_NULL, (empty)
                // 1047: b"reuse %d\n\0" ... st u8: typeof(_260 = move _261 as *const u8 (Pointer(ArrayToPointer))) = *const {l725} u8
                // 1047: b"reuse %d\n\0" ... st u8:   l725 = UNIQUE | NON_NULL, (empty)
                // 1047: b"reuse %d\n\0": typeof(_262 = const b"reuse %d\n\x00") = & {l723} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1047: b"reuse %d\n\0":   l723 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        8 => {
            fprintf(
                file,
                // 1053: file: typeof(_266) = *mut {l448} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 1053: file:   l448 = UNIQUE | NON_NULL, (empty)
                b"option %s %d\n\0" as *const u8 as *const libc::c_char,
                // 1054: b"option %s %d\ ... _char: typeof(_267) = *const {l450} i8
                // 1054: b"option %s %d\ ... _char:   l450 = UNIQUE | NON_NULL, (empty)
                // 1054: b"option %s %d\ ... st u8: typeof(_268) = *const {l452} u8
                // 1054: b"option %s %d\ ... st u8:   l452 = UNIQUE | NON_NULL, (empty)
                // 1054: b"option %s %d\n\0": typeof(_269) = *const {l454} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 1054: b"option %s %d\n\0":   l454 = UNIQUE | NON_NULL, (empty)
                // 1054: b"option %s %d\n\0": typeof(_270) = & {l456} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 1054: b"option %s %d\n\0":   l456 = UNIQUE | NON_NULL, FIXED
                // 1054: b"option %s %d\n\0": typeof(_269 = &raw const (*_270)) = *const {l728} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 1054: b"option %s %d\n\0":   l728 = UNIQUE | NON_NULL, (empty)
                // 1054: b"option %s %d\ ... st u8: typeof(_268 = move _269 as *const u8 (Pointer(ArrayToPointer))) = *const {l729} u8
                // 1054: b"option %s %d\ ... st u8:   l729 = UNIQUE | NON_NULL, (empty)
                // 1054: b"option %s %d\ ... _char: typeof(_267 = move _268 as *const i8 (Misc)) = *const {l730} i8
                // 1054: b"option %s %d\ ... _char:   l730 = UNIQUE | NON_NULL, (empty)
                // 1054: b"option %s %d\n\0": typeof(_270 = const b"option %s %d\n\x00") = & {l727} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 1054: b"option %s %d\n\0":   l727 = UNIQUE | NON_NULL, (empty)
                (*e).opt,
                // 1055: (*e).opt: typeof(_271) = *mut {l458} i8
                // 1055: (*e).opt:   l458 = UNIQUE | NON_NULL, (empty)
                (*e).arg,
            );
        }
        9 => {
            fprintf(
                file,
                // 1061: file: typeof(_274) = *mut {l462} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 1061: file:   l462 = UNIQUE | NON_NULL, (empty)
                b"phase %d\n\0" as *const u8 as *const libc::c_char,
                // 1062: b"phase %d\n\0" ... _char: typeof(_275) = *const {l464} i8
                // 1062: b"phase %d\n\0" ... _char:   l464 = UNIQUE | NON_NULL, (empty)
                // 1062: b"phase %d\n\0" ... st u8: typeof(_276) = *const {l466} u8
                // 1062: b"phase %d\n\0" ... st u8:   l466 = UNIQUE | NON_NULL, (empty)
                // 1062: b"phase %d\n\0": typeof(_277) = *const {l468} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1062: b"phase %d\n\0":   l468 = UNIQUE | NON_NULL, (empty)
                // 1062: b"phase %d\n\0": typeof(_278) = & {l470} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1062: b"phase %d\n\0":   l470 = UNIQUE | NON_NULL, FIXED
                // 1062: b"phase %d\n\0": typeof(_278 = const b"phase %d\n\x00") = & {l731} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1062: b"phase %d\n\0":   l731 = UNIQUE | NON_NULL, (empty)
                // 1062: b"phase %d\n\0" ... st u8: typeof(_276 = move _277 as *const u8 (Pointer(ArrayToPointer))) = *const {l733} u8
                // 1062: b"phase %d\n\0" ... st u8:   l733 = UNIQUE | NON_NULL, (empty)
                // 1062: b"phase %d\n\0" ... _char: typeof(_275 = move _276 as *const i8 (Misc)) = *const {l734} i8
                // 1062: b"phase %d\n\0" ... _char:   l734 = UNIQUE | NON_NULL, (empty)
                // 1062: b"phase %d\n\0": typeof(_277 = &raw const (*_278)) = *const {l732} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1062: b"phase %d\n\0":   l732 = UNIQUE | NON_NULL, (empty)
                lit((*e).arg),
            );
        }
        10 => {
            fprintf(file, b"release\n\0" as *const u8 as *const libc::c_char);
            // 1067: file: typeof(_282) = *mut {l475} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 1067: file:   l475 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0"  ... _char: typeof(_283) = *const {l477} i8
            // 1067: b"release\n\0"  ... _char:   l477 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0"  ... st u8: typeof(_284) = *const {l479} u8
            // 1067: b"release\n\0"  ... st u8:   l479 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0": typeof(_285) = *const {l481} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1067: b"release\n\0":   l481 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0": typeof(_286) = & {l483} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1067: b"release\n\0":   l483 = UNIQUE | NON_NULL, FIXED
            // 1067: b"release\n\0"  ... _char: typeof(_283 = move _284 as *const i8 (Misc)) = *const {l738} i8
            // 1067: b"release\n\0"  ... _char:   l738 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0"  ... st u8: typeof(_284 = move _285 as *const u8 (Pointer(ArrayToPointer))) = *const {l737} u8
            // 1067: b"release\n\0"  ... st u8:   l737 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0": typeof(_285 = &raw const (*_286)) = *const {l736} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1067: b"release\n\0":   l736 = UNIQUE | NON_NULL, (empty)
            // 1067: b"release\n\0": typeof(_286 = const b"release\n\x00") = & {l735} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1067: b"release\n\0":   l735 = UNIQUE | NON_NULL, (empty)
        }
        11 => {
            fprintf(
                file,
                // 1071: file: typeof(_288) = *mut {l486} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 1071: file:   l486 = UNIQUE | NON_NULL, (empty)
                b"return %d\n\0" as *const u8 as *const libc::c_char,
                // 1072: b"return %d\n\0 ... _char: typeof(_289) = *const {l488} i8
                // 1072: b"return %d\n\0 ... _char:   l488 = UNIQUE | NON_NULL, (empty)
                // 1072: b"return %d\n\0 ... st u8: typeof(_290) = *const {l490} u8
                // 1072: b"return %d\n\0 ... st u8:   l490 = UNIQUE | NON_NULL, (empty)
                // 1072: b"return %d\n\0": typeof(_291) = *const {l492} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1072: b"return %d\n\0":   l492 = UNIQUE | NON_NULL, (empty)
                // 1072: b"return %d\n\0": typeof(_292) = & {l494} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1072: b"return %d\n\0":   l494 = UNIQUE | NON_NULL, FIXED
                // 1072: b"return %d\n\0": typeof(_292 = const b"return %d\n\x00") = & {l739} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1072: b"return %d\n\0":   l739 = UNIQUE | NON_NULL, (empty)
                // 1072: b"return %d\n\0": typeof(_291 = &raw const (*_292)) = *const {l740} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1072: b"return %d\n\0":   l740 = UNIQUE | NON_NULL, (empty)
                // 1072: b"return %d\n\0 ... st u8: typeof(_290 = move _291 as *const u8 (Pointer(ArrayToPointer))) = *const {l741} u8
                // 1072: b"return %d\n\0 ... st u8:   l741 = UNIQUE | NON_NULL, (empty)
                // 1072: b"return %d\n\0 ... _char: typeof(_289 = move _290 as *const i8 (Misc)) = *const {l742} i8
                // 1072: b"return %d\n\0 ... _char:   l742 = UNIQUE | NON_NULL, (empty)
                (*e).arg,
            );
        }
        13 => {
            fprintf(
                file,
                // 1078: file: typeof(_295) = *mut {l498} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
                // 1078: file:   l498 = UNIQUE | NON_NULL, (empty)
                b"simp %d\n\0" as *const u8 as *const libc::c_char,
                // 1079: b"simp %d\n\0"  ... _char: typeof(_296) = *const {l500} i8
                // 1079: b"simp %d\n\0"  ... _char:   l500 = UNIQUE | NON_NULL, (empty)
                // 1079: b"simp %d\n\0"  ... st u8: typeof(_297) = *const {l502} u8
                // 1079: b"simp %d\n\0"  ... st u8:   l502 = UNIQUE | NON_NULL, (empty)
                // 1079: b"simp %d\n\0": typeof(_298) = *const {l504} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1079: b"simp %d\n\0":   l504 = UNIQUE | NON_NULL, (empty)
                // 1079: b"simp %d\n\0": typeof(_299) = & {l506} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1079: b"simp %d\n\0":   l506 = UNIQUE | NON_NULL, FIXED
                // 1079: b"simp %d\n\0"  ... _char: typeof(_296 = move _297 as *const i8 (Misc)) = *const {l746} i8
                // 1079: b"simp %d\n\0"  ... _char:   l746 = UNIQUE | NON_NULL, (empty)
                // 1079: b"simp %d\n\0": typeof(_298 = &raw const (*_299)) = *const {l744} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1079: b"simp %d\n\0":   l744 = UNIQUE | NON_NULL, (empty)
                // 1079: b"simp %d\n\0"  ... st u8: typeof(_297 = move _298 as *const u8 (Pointer(ArrayToPointer))) = *const {l745} u8
                // 1079: b"simp %d\n\0"  ... st u8:   l745 = UNIQUE | NON_NULL, (empty)
                // 1079: b"simp %d\n\0": typeof(_299 = const b"simp %d\n\x00") = & {l743} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1079: b"simp %d\n\0":   l743 = UNIQUE | NON_NULL, (empty)
                (*e).arg,
            );
        }
        12 | _ => {
            if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(
                    b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                    // 1087: b"e->type == SA ... _char: typeof(_308) = *const {l516} i8
                    // 1087: b"e->type == SA ... _char:   l516 = UNIQUE | NON_NULL, (empty)
                    // 1087: b"e->type == SA ... st u8: typeof(_309) = *const {l518} u8
                    // 1087: b"e->type == SA ... st u8:   l518 = UNIQUE | NON_NULL, (empty)
                    // 1087: b"e->type == SAT\0": typeof(_310) = *const {l520} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1087: b"e->type == SAT\0":   l520 = UNIQUE | NON_NULL, (empty)
                    // 1087: b"e->type == SAT\0": typeof(_311) = & {l522} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1087: b"e->type == SAT\0":   l522 = UNIQUE | NON_NULL, FIXED
                    // 1087: b"e->type == SA ... st u8: typeof(_309 = move _310 as *const u8 (Pointer(ArrayToPointer))) = *const {l749} u8
                    // 1087: b"e->type == SA ... st u8:   l749 = UNIQUE | NON_NULL, (empty)
                    // 1087: b"e->type == SAT\0": typeof(_310 = &raw const (*_311)) = *const {l748} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1087: b"e->type == SAT\0":   l748 = UNIQUE | NON_NULL, (empty)
                    // 1087: b"e->type == SA ... _char: typeof(_308 = move _309 as *const i8 (Misc)) = *const {l750} i8
                    // 1087: b"e->type == SA ... _char:   l750 = UNIQUE | NON_NULL, (empty)
                    // 1087: b"e->type == SAT\0": typeof(_311 = const b"e->type == SAT\x00") = & {l747} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                    // 1087: b"e->type == SAT\0":   l747 = UNIQUE | NON_NULL, (empty)
                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                    // 1088: b"lglddtrace.c\ ... _char: typeof(_312) = *const {l524} i8
                    // 1088: b"lglddtrace.c\ ... _char:   l524 = UNIQUE | NON_NULL, (empty)
                    // 1088: b"lglddtrace.c\ ... st u8: typeof(_313) = *const {l526} u8
                    // 1088: b"lglddtrace.c\ ... st u8:   l526 = UNIQUE | NON_NULL, (empty)
                    // 1088: b"lglddtrace.c\0": typeof(_314) = *const {l528} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1088: b"lglddtrace.c\0":   l528 = UNIQUE | NON_NULL, (empty)
                    // 1088: b"lglddtrace.c\0": typeof(_315) = & {l530} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1088: b"lglddtrace.c\0":   l530 = UNIQUE | NON_NULL, FIXED
                    // 1088: b"lglddtrace.c\0": typeof(_315 = const b"lglddtrace.c\x00") = & {l751} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1088: b"lglddtrace.c\0":   l751 = UNIQUE | NON_NULL, (empty)
                    // 1088: b"lglddtrace.c\0": typeof(_314 = &raw const (*_315)) = *const {l752} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1088: b"lglddtrace.c\0":   l752 = UNIQUE | NON_NULL, (empty)
                    // 1088: b"lglddtrace.c\ ... _char: typeof(_312 = move _313 as *const i8 (Misc)) = *const {l754} i8
                    // 1088: b"lglddtrace.c\ ... _char:   l754 = UNIQUE | NON_NULL, (empty)
                    // 1088: b"lglddtrace.c\ ... st u8: typeof(_313 = move _314 as *const u8 (Pointer(ArrayToPointer))) = *const {l753} u8
                    // 1088: b"lglddtrace.c\ ... st u8:   l753 = UNIQUE | NON_NULL, (empty)
                    316 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                    // 1090: (*::core::mem:: ... ptr(): typeof(_318) = *const {l534} i8
                    // 1090: (*::core::mem:: ... ptr():   l534 = UNIQUE | NON_NULL, (empty)
                    // 1090: (*::core::mem:: ... ptr(): typeof(_319) = & {l536} [i8]
                    // 1090: (*::core::mem:: ... ptr():   l536 = UNIQUE | NON_NULL, FIXED
                    // 1090: (*::core::mem:: ... ptr(): typeof(_320) = & {l538} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                    // 1090: (*::core::mem:: ... ptr():   l538 = UNIQUE | NON_NULL, (empty)
                    // 1090: ::core::mem::tr ... 0", ): typeof(_321) = & {l540} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                    // 1090: ::core::mem::tr ... 0", ):   l540 = UNIQUE | NON_NULL, FIXED
                    // 1090: (*::core::mem:: ... ptr(): typeof(_319 = move _320 as &[i8] (Pointer(Unsize))) = & {l758} [i8]
                    // 1090: (*::core::mem:: ... ptr():   l758 = UNIQUE | NON_NULL, FIXED
                    // 1090: (*::core::mem:: ... ptr(): typeof(_320 = &(*_321)) = & {l757} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                    // 1090: (*::core::mem:: ... ptr():   l757 = UNIQUE | NON_NULL, (empty)
                        b"void print(Event *, FILE *)\0",
                        // 1091: b"void print(Ev ... *)\0": typeof(_322) = & {l542} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1091: b"void print(Ev ... *)\0":   l542 = UNIQUE | NON_NULL, (empty)
                        // 1091: b"void print(Ev ... *)\0": typeof(_323) = & {l544} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1091: b"void print(Ev ... *)\0":   l544 = UNIQUE | NON_NULL, FIXED
                        // 1091: b"void print(Ev ... *)\0": typeof(_322 = &(*_323)) = & {l756} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1091: b"void print(Ev ... *)\0":   l756 = UNIQUE | NON_NULL, (empty)
                        // 1091: b"void print(Ev ... *)\0": typeof(_323 = const b"void print(Event *, FILE *)\x00") = & {l755} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1091: b"void print(Ev ... *)\0":   l755 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
            'c_6385: {
                if (*e).type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(
                        b"e->type == SAT\0" as *const u8 as *const libc::c_char,
                        // 1100: b"e->type == SA ... _char: typeof(_331) = *const {l553} i8
                        // 1100: b"e->type == SA ... _char:   l553 = UNIQUE | NON_NULL, (empty)
                        // 1100: b"e->type == SA ... st u8: typeof(_332) = *const {l555} u8
                        // 1100: b"e->type == SA ... st u8:   l555 = UNIQUE | NON_NULL, (empty)
                        // 1100: b"e->type == SAT\0": typeof(_333) = *const {l557} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1100: b"e->type == SAT\0":   l557 = UNIQUE | NON_NULL, (empty)
                        // 1100: b"e->type == SAT\0": typeof(_334) = & {l559} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1100: b"e->type == SAT\0":   l559 = UNIQUE | NON_NULL, FIXED
                        // 1100: b"e->type == SA ... _char: typeof(_331 = move _332 as *const i8 (Misc)) = *const {l762} i8
                        // 1100: b"e->type == SA ... _char:   l762 = UNIQUE | NON_NULL, (empty)
                        // 1100: b"e->type == SA ... st u8: typeof(_332 = move _333 as *const u8 (Pointer(ArrayToPointer))) = *const {l761} u8
                        // 1100: b"e->type == SA ... st u8:   l761 = UNIQUE | NON_NULL, (empty)
                        // 1100: b"e->type == SAT\0": typeof(_334 = const b"e->type == SAT\x00") = & {l759} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1100: b"e->type == SAT\0":   l759 = UNIQUE | NON_NULL, (empty)
                        // 1100: b"e->type == SAT\0": typeof(_333 = &raw const (*_334)) = *const {l760} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1100: b"e->type == SAT\0":   l760 = UNIQUE | NON_NULL, (empty)
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        // 1101: b"lglddtrace.c\ ... _char: typeof(_335) = *const {l561} i8
                        // 1101: b"lglddtrace.c\ ... _char:   l561 = UNIQUE | NON_NULL, (empty)
                        // 1101: b"lglddtrace.c\ ... st u8: typeof(_336) = *const {l563} u8
                        // 1101: b"lglddtrace.c\ ... st u8:   l563 = UNIQUE | NON_NULL, (empty)
                        // 1101: b"lglddtrace.c\0": typeof(_337) = *const {l565} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1101: b"lglddtrace.c\0":   l565 = UNIQUE | NON_NULL, (empty)
                        // 1101: b"lglddtrace.c\0": typeof(_338) = & {l567} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1101: b"lglddtrace.c\0":   l567 = UNIQUE | NON_NULL, FIXED
                        // 1101: b"lglddtrace.c\0": typeof(_337 = &raw const (*_338)) = *const {l764} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1101: b"lglddtrace.c\0":   l764 = UNIQUE | NON_NULL, (empty)
                        // 1101: b"lglddtrace.c\ ... st u8: typeof(_336 = move _337 as *const u8 (Pointer(ArrayToPointer))) = *const {l765} u8
                        // 1101: b"lglddtrace.c\ ... st u8:   l765 = UNIQUE | NON_NULL, (empty)
                        // 1101: b"lglddtrace.c\0": typeof(_338 = const b"lglddtrace.c\x00") = & {l763} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1101: b"lglddtrace.c\0":   l763 = UNIQUE | NON_NULL, (empty)
                        // 1101: b"lglddtrace.c\ ... _char: typeof(_335 = move _336 as *const i8 (Misc)) = *const {l766} i8
                        // 1101: b"lglddtrace.c\ ... _char:   l766 = UNIQUE | NON_NULL, (empty)
                        316 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 28], &[libc::c_char; 28]>(
                        // 1103: (*::core::mem:: ... ptr(): typeof(_341) = *const {l571} i8
                        // 1103: (*::core::mem:: ... ptr():   l571 = UNIQUE | NON_NULL, (empty)
                        // 1103: (*::core::mem:: ... ptr(): typeof(_342) = & {l573} [i8]
                        // 1103: (*::core::mem:: ... ptr():   l573 = UNIQUE | NON_NULL, FIXED
                        // 1103: (*::core::mem:: ... ptr(): typeof(_343) = & {l575} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1103: (*::core::mem:: ... ptr():   l575 = UNIQUE | NON_NULL, (empty)
                        // 1103: ::core::mem::tr ... 0", ): typeof(_344) = & {l577} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1103: ::core::mem::tr ... 0", ):   l577 = UNIQUE | NON_NULL, FIXED
                        // 1103: (*::core::mem:: ... ptr(): typeof(_343 = &(*_344)) = & {l769} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                        // 1103: (*::core::mem:: ... ptr():   l769 = UNIQUE | NON_NULL, (empty)
                        // 1103: (*::core::mem:: ... ptr(): typeof(_342 = move _343 as &[i8] (Pointer(Unsize))) = & {l770} [i8]
                        // 1103: (*::core::mem:: ... ptr():   l770 = UNIQUE | NON_NULL, FIXED
                            b"void print(Event *, FILE *)\0",
                            // 1104: b"void print(Ev ... *)\0": typeof(_345) = & {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1104: b"void print(Ev ... *)\0":   l579 = UNIQUE | NON_NULL, (empty)
                            // 1104: b"void print(Ev ... *)\0": typeof(_346) = & {l581} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1104: b"void print(Ev ... *)\0":   l581 = UNIQUE | NON_NULL, FIXED
                            // 1104: b"void print(Ev ... *)\0": typeof(_345 = &(*_346)) = & {l768} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1104: b"void print(Ev ... *)\0":   l768 = UNIQUE | NON_NULL, (empty)
                            // 1104: b"void print(Ev ... *)\0": typeof(_346 = const b"void print(Event *, FILE *)\x00") = & {l767} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                            // 1104: b"void print(Ev ... *)\0":   l767 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
            };
            fprintf(file, b"sat\n\0" as *const u8 as *const libc::c_char);
            // 1110: file: typeof(_348) = *mut {l584} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 1110: file:   l584 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0" as * ... _char: typeof(_349) = *const {l586} i8
            // 1110: b"sat\n\0" as * ... _char:   l586 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0" as * ... st u8: typeof(_350) = *const {l588} u8
            // 1110: b"sat\n\0" as * ... st u8:   l588 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0": typeof(_351) = *const {l590} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1110: b"sat\n\0":   l590 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0": typeof(_352) = & {l592} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1110: b"sat\n\0":   l592 = UNIQUE | NON_NULL, FIXED
            // 1110: b"sat\n\0" as * ... st u8: typeof(_350 = move _351 as *const u8 (Pointer(ArrayToPointer))) = *const {l773} u8
            // 1110: b"sat\n\0" as * ... st u8:   l773 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0" as * ... _char: typeof(_349 = move _350 as *const i8 (Misc)) = *const {l774} i8
            // 1110: b"sat\n\0" as * ... _char:   l774 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0": typeof(_352 = const b"sat\n\x00") = & {l771} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1110: b"sat\n\0":   l771 = UNIQUE | NON_NULL, (empty)
            // 1110: b"sat\n\0": typeof(_351 = &raw const (*_352)) = *const {l772} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1110: b"sat\n\0":   l772 = UNIQUE | NON_NULL, (empty)
        }
    };
}
unsafe extern "C" fn type2str(mut type_0: Type) -> *const libc::c_char {
// 1114: *const libc::c_char: typeof(_0) = *const {g14} i8
// 1114: *const libc::c_char:   g14 = UNIQUE | NON_NULL, FIXED
    match type_0 as libc::c_uint {
        0 => return b"add\0" as *const u8 as *const libc::c_char,
        // 1116: b"add\0" as *co ... st u8: typeof(_6) = *const {l6} u8
        // 1116: b"add\0" as *co ... st u8:   l6 = UNIQUE | NON_NULL, (empty)
        // 1116: b"add\0": typeof(_7) = *const {l8} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 1116: b"add\0":   l8 = UNIQUE | NON_NULL, (empty)
        // 1116: b"add\0": typeof(_8) = & {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 1116: b"add\0":   l10 = UNIQUE | NON_NULL, FIXED
        // 1116: b"add\0": typeof(_7 = &raw const (*_8)) = *const {l304} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 1116: b"add\0":   l304 = UNIQUE | NON_NULL, (empty)
        // 1116: b"add\0" as *co ... _char: typeof(_0 = move _6 as *const i8 (Misc)) = *const {l306} i8
        // 1116: b"add\0" as *co ... _char:   l306 = UNIQUE | NON_NULL, (empty)
        // 1116: b"add\0" as *co ... st u8: typeof(_6 = move _7 as *const u8 (Pointer(ArrayToPointer))) = *const {l305} u8
        // 1116: b"add\0" as *co ... st u8:   l305 = UNIQUE | NON_NULL, (empty)
        // 1116: b"add\0": typeof(_8 = const b"add\x00") = & {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
        // 1116: b"add\0":   l303 = UNIQUE | NON_NULL, (empty)
        1 => return b"assume\0" as *const u8 as *const libc::c_char,
        // 1117: b"assume\0" as  ... st u8: typeof(_10) = *const {l13} u8
        // 1117: b"assume\0" as  ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 1117: b"assume\0": typeof(_11) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1117: b"assume\0":   l15 = UNIQUE | NON_NULL, (empty)
        // 1117: b"assume\0": typeof(_12) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1117: b"assume\0":   l17 = UNIQUE | NON_NULL, FIXED
        // 1117: b"assume\0" as  ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l309} u8
        // 1117: b"assume\0" as  ... st u8:   l309 = UNIQUE | NON_NULL, (empty)
        // 1117: b"assume\0" as  ... _char: typeof(_0 = move _10 as *const i8 (Misc)) = *const {l310} i8
        // 1117: b"assume\0" as  ... _char:   l310 = UNIQUE | NON_NULL, (empty)
        // 1117: b"assume\0": typeof(_12 = const b"assume\x00") = & {l307} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1117: b"assume\0":   l307 = UNIQUE | NON_NULL, (empty)
        // 1117: b"assume\0": typeof(_11 = &raw const (*_12)) = *const {l308} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1117: b"assume\0":   l308 = UNIQUE | NON_NULL, (empty)
        29 => return b"changed\0" as *const u8 as *const libc::c_char,
        // 1118: b"changed\0" as ... st u8: typeof(_14) = *const {l20} u8
        // 1118: b"changed\0" as ... st u8:   l20 = UNIQUE | NON_NULL, (empty)
        // 1118: b"changed\0": typeof(_15) = *const {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1118: b"changed\0":   l22 = UNIQUE | NON_NULL, (empty)
        // 1118: b"changed\0": typeof(_16) = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1118: b"changed\0":   l24 = UNIQUE | NON_NULL, FIXED
        // 1118: b"changed\0": typeof(_16 = const b"changed\x00") = & {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1118: b"changed\0":   l311 = UNIQUE | NON_NULL, (empty)
        // 1118: b"changed\0" as ... _char: typeof(_0 = move _14 as *const i8 (Misc)) = *const {l314} i8
        // 1118: b"changed\0" as ... _char:   l314 = UNIQUE | NON_NULL, (empty)
        // 1118: b"changed\0" as ... st u8: typeof(_14 = move _15 as *const u8 (Pointer(ArrayToPointer))) = *const {l313} u8
        // 1118: b"changed\0" as ... st u8:   l313 = UNIQUE | NON_NULL, (empty)
        // 1118: b"changed\0": typeof(_15 = &raw const (*_16)) = *const {l312} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1118: b"changed\0":   l312 = UNIQUE | NON_NULL, (empty)
        28 => return b"chkclone\0" as *const u8 as *const libc::c_char,
        // 1119: b"chkclone\0" a ... st u8: typeof(_18) = *const {l27} u8
        // 1119: b"chkclone\0" a ... st u8:   l27 = UNIQUE | NON_NULL, (empty)
        // 1119: b"chkclone\0": typeof(_19) = *const {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1119: b"chkclone\0":   l29 = UNIQUE | NON_NULL, (empty)
        // 1119: b"chkclone\0": typeof(_20) = & {l31} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1119: b"chkclone\0":   l31 = UNIQUE | NON_NULL, FIXED
        // 1119: b"chkclone\0" a ... _char: typeof(_0 = move _18 as *const i8 (Misc)) = *const {l318} i8
        // 1119: b"chkclone\0" a ... _char:   l318 = UNIQUE | NON_NULL, (empty)
        // 1119: b"chkclone\0": typeof(_19 = &raw const (*_20)) = *const {l316} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1119: b"chkclone\0":   l316 = UNIQUE | NON_NULL, (empty)
        // 1119: b"chkclone\0": typeof(_20 = const b"chkclone\x00") = & {l315} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1119: b"chkclone\0":   l315 = UNIQUE | NON_NULL, (empty)
        // 1119: b"chkclone\0" a ... st u8: typeof(_18 = move _19 as *const u8 (Pointer(ArrayToPointer))) = *const {l317} u8
        // 1119: b"chkclone\0" a ... st u8:   l317 = UNIQUE | NON_NULL, (empty)
        2 => return b"deref\0" as *const u8 as *const libc::c_char,
        // 1120: b"deref\0" as * ... st u8: typeof(_22) = *const {l34} u8
        // 1120: b"deref\0" as * ... st u8:   l34 = UNIQUE | NON_NULL, (empty)
        // 1120: b"deref\0": typeof(_23) = *const {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1120: b"deref\0":   l36 = UNIQUE | NON_NULL, (empty)
        // 1120: b"deref\0": typeof(_24) = & {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1120: b"deref\0":   l38 = UNIQUE | NON_NULL, FIXED
        // 1120: b"deref\0": typeof(_23 = &raw const (*_24)) = *const {l320} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1120: b"deref\0":   l320 = UNIQUE | NON_NULL, (empty)
        // 1120: b"deref\0" as * ... _char: typeof(_0 = move _22 as *const i8 (Misc)) = *const {l322} i8
        // 1120: b"deref\0" as * ... _char:   l322 = UNIQUE | NON_NULL, (empty)
        // 1120: b"deref\0": typeof(_24 = const b"deref\x00") = & {l319} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1120: b"deref\0":   l319 = UNIQUE | NON_NULL, (empty)
        // 1120: b"deref\0" as * ... st u8: typeof(_22 = move _23 as *const u8 (Pointer(ArrayToPointer))) = *const {l321} u8
        // 1120: b"deref\0" as * ... st u8:   l321 = UNIQUE | NON_NULL, (empty)
        3 => return b"failed\0" as *const u8 as *const libc::c_char,
        // 1121: b"failed\0" as  ... st u8: typeof(_26) = *const {l41} u8
        // 1121: b"failed\0" as  ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
        // 1121: b"failed\0": typeof(_27) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1121: b"failed\0":   l43 = UNIQUE | NON_NULL, (empty)
        // 1121: b"failed\0": typeof(_28) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1121: b"failed\0":   l45 = UNIQUE | NON_NULL, FIXED
        // 1121: b"failed\0": typeof(_27 = &raw const (*_28)) = *const {l324} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1121: b"failed\0":   l324 = UNIQUE | NON_NULL, (empty)
        // 1121: b"failed\0": typeof(_28 = const b"failed\x00") = & {l323} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1121: b"failed\0":   l323 = UNIQUE | NON_NULL, (empty)
        // 1121: b"failed\0" as  ... _char: typeof(_0 = move _26 as *const i8 (Misc)) = *const {l326} i8
        // 1121: b"failed\0" as  ... _char:   l326 = UNIQUE | NON_NULL, (empty)
        // 1121: b"failed\0" as  ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l325} u8
        // 1121: b"failed\0" as  ... st u8:   l325 = UNIQUE | NON_NULL, (empty)
        26 => return b"fixed\0" as *const u8 as *const libc::c_char,
        // 1122: b"fixed\0" as * ... st u8: typeof(_30) = *const {l48} u8
        // 1122: b"fixed\0" as * ... st u8:   l48 = UNIQUE | NON_NULL, (empty)
        // 1122: b"fixed\0": typeof(_31) = *const {l50} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1122: b"fixed\0":   l50 = UNIQUE | NON_NULL, (empty)
        // 1122: b"fixed\0": typeof(_32) = & {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1122: b"fixed\0":   l52 = UNIQUE | NON_NULL, FIXED
        // 1122: b"fixed\0" as * ... st u8: typeof(_30 = move _31 as *const u8 (Pointer(ArrayToPointer))) = *const {l329} u8
        // 1122: b"fixed\0" as * ... st u8:   l329 = UNIQUE | NON_NULL, (empty)
        // 1122: b"fixed\0": typeof(_32 = const b"fixed\x00") = & {l327} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1122: b"fixed\0":   l327 = UNIQUE | NON_NULL, (empty)
        // 1122: b"fixed\0" as * ... _char: typeof(_0 = move _30 as *const i8 (Misc)) = *const {l330} i8
        // 1122: b"fixed\0" as * ... _char:   l330 = UNIQUE | NON_NULL, (empty)
        // 1122: b"fixed\0": typeof(_31 = &raw const (*_32)) = *const {l328} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1122: b"fixed\0":   l328 = UNIQUE | NON_NULL, (empty)
        21 => return b"frozen\0" as *const u8 as *const libc::c_char,
        // 1123: b"frozen\0" as  ... st u8: typeof(_34) = *const {l55} u8
        // 1123: b"frozen\0" as  ... st u8:   l55 = UNIQUE | NON_NULL, (empty)
        // 1123: b"frozen\0": typeof(_35) = *const {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1123: b"frozen\0":   l57 = UNIQUE | NON_NULL, (empty)
        // 1123: b"frozen\0": typeof(_36) = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1123: b"frozen\0":   l59 = UNIQUE | NON_NULL, FIXED
        // 1123: b"frozen\0" as  ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l333} u8
        // 1123: b"frozen\0" as  ... st u8:   l333 = UNIQUE | NON_NULL, (empty)
        // 1123: b"frozen\0": typeof(_35 = &raw const (*_36)) = *const {l332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1123: b"frozen\0":   l332 = UNIQUE | NON_NULL, (empty)
        // 1123: b"frozen\0": typeof(_36 = const b"frozen\x00") = & {l331} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1123: b"frozen\0":   l331 = UNIQUE | NON_NULL, (empty)
        // 1123: b"frozen\0" as  ... _char: typeof(_0 = move _34 as *const i8 (Misc)) = *const {l334} i8
        // 1123: b"frozen\0" as  ... _char:   l334 = UNIQUE | NON_NULL, (empty)
        23 => return b"reusable\0" as *const u8 as *const libc::c_char,
        // 1124: b"reusable\0" a ... st u8: typeof(_38) = *const {l62} u8
        // 1124: b"reusable\0" a ... st u8:   l62 = UNIQUE | NON_NULL, (empty)
        // 1124: b"reusable\0": typeof(_39) = *const {l64} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1124: b"reusable\0":   l64 = UNIQUE | NON_NULL, (empty)
        // 1124: b"reusable\0": typeof(_40) = & {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1124: b"reusable\0":   l66 = UNIQUE | NON_NULL, FIXED
        // 1124: b"reusable\0": typeof(_39 = &raw const (*_40)) = *const {l336} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1124: b"reusable\0":   l336 = UNIQUE | NON_NULL, (empty)
        // 1124: b"reusable\0" a ... st u8: typeof(_38 = move _39 as *const u8 (Pointer(ArrayToPointer))) = *const {l337} u8
        // 1124: b"reusable\0" a ... st u8:   l337 = UNIQUE | NON_NULL, (empty)
        // 1124: b"reusable\0" a ... _char: typeof(_0 = move _38 as *const i8 (Misc)) = *const {l338} i8
        // 1124: b"reusable\0" a ... _char:   l338 = UNIQUE | NON_NULL, (empty)
        // 1124: b"reusable\0": typeof(_40 = const b"reusable\x00") = & {l335} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1124: b"reusable\0":   l335 = UNIQUE | NON_NULL, (empty)
        22 => return b"usable\0" as *const u8 as *const libc::c_char,
        // 1125: b"usable\0" as  ... st u8: typeof(_42) = *const {l69} u8
        // 1125: b"usable\0" as  ... st u8:   l69 = UNIQUE | NON_NULL, (empty)
        // 1125: b"usable\0": typeof(_43) = *const {l71} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1125: b"usable\0":   l71 = UNIQUE | NON_NULL, (empty)
        // 1125: b"usable\0": typeof(_44) = & {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1125: b"usable\0":   l73 = UNIQUE | NON_NULL, FIXED
        // 1125: b"usable\0": typeof(_43 = &raw const (*_44)) = *const {l340} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1125: b"usable\0":   l340 = UNIQUE | NON_NULL, (empty)
        // 1125: b"usable\0" as  ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l341} u8
        // 1125: b"usable\0" as  ... st u8:   l341 = UNIQUE | NON_NULL, (empty)
        // 1125: b"usable\0" as  ... _char: typeof(_0 = move _42 as *const i8 (Misc)) = *const {l342} i8
        // 1125: b"usable\0" as  ... _char:   l342 = UNIQUE | NON_NULL, (empty)
        // 1125: b"usable\0": typeof(_44 = const b"usable\x00") = & {l339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1125: b"usable\0":   l339 = UNIQUE | NON_NULL, (empty)
        14 => return b"repr\0" as *const u8 as *const libc::c_char,
        // 1126: b"repr\0" as *c ... st u8: typeof(_46) = *const {l76} u8
        // 1126: b"repr\0" as *c ... st u8:   l76 = UNIQUE | NON_NULL, (empty)
        // 1126: b"repr\0": typeof(_47) = *const {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1126: b"repr\0":   l78 = UNIQUE | NON_NULL, (empty)
        // 1126: b"repr\0": typeof(_48) = & {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1126: b"repr\0":   l80 = UNIQUE | NON_NULL, FIXED
        // 1126: b"repr\0" as *c ... st u8: typeof(_46 = move _47 as *const u8 (Pointer(ArrayToPointer))) = *const {l345} u8
        // 1126: b"repr\0" as *c ... st u8:   l345 = UNIQUE | NON_NULL, (empty)
        // 1126: b"repr\0": typeof(_47 = &raw const (*_48)) = *const {l344} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1126: b"repr\0":   l344 = UNIQUE | NON_NULL, (empty)
        // 1126: b"repr\0": typeof(_48 = const b"repr\x00") = & {l343} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1126: b"repr\0":   l343 = UNIQUE | NON_NULL, (empty)
        // 1126: b"repr\0" as *c ... _char: typeof(_0 = move _46 as *const i8 (Misc)) = *const {l346} i8
        // 1126: b"repr\0" as *c ... _char:   l346 = UNIQUE | NON_NULL, (empty)
        27 => return b"fixate\0" as *const u8 as *const libc::c_char,
        // 1127: b"fixate\0" as  ... st u8: typeof(_50) = *const {l83} u8
        // 1127: b"fixate\0" as  ... st u8:   l83 = UNIQUE | NON_NULL, (empty)
        // 1127: b"fixate\0": typeof(_51) = *const {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1127: b"fixate\0":   l85 = UNIQUE | NON_NULL, (empty)
        // 1127: b"fixate\0": typeof(_52) = & {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1127: b"fixate\0":   l87 = UNIQUE | NON_NULL, FIXED
        // 1127: b"fixate\0": typeof(_51 = &raw const (*_52)) = *const {l348} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1127: b"fixate\0":   l348 = UNIQUE | NON_NULL, (empty)
        // 1127: b"fixate\0": typeof(_52 = const b"fixate\x00") = & {l347} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1127: b"fixate\0":   l347 = UNIQUE | NON_NULL, (empty)
        // 1127: b"fixate\0" as  ... _char: typeof(_0 = move _50 as *const i8 (Misc)) = *const {l350} i8
        // 1127: b"fixate\0" as  ... _char:   l350 = UNIQUE | NON_NULL, (empty)
        // 1127: b"fixate\0" as  ... st u8: typeof(_50 = move _51 as *const u8 (Pointer(ArrayToPointer))) = *const {l349} u8
        // 1127: b"fixate\0" as  ... st u8:   l349 = UNIQUE | NON_NULL, (empty)
        20 => return b"reduce\0" as *const u8 as *const libc::c_char,
        // 1128: b"reduce\0" as  ... st u8: typeof(_54) = *const {l90} u8
        // 1128: b"reduce\0" as  ... st u8:   l90 = UNIQUE | NON_NULL, (empty)
        // 1128: b"reduce\0": typeof(_55) = *const {l92} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1128: b"reduce\0":   l92 = UNIQUE | NON_NULL, (empty)
        // 1128: b"reduce\0": typeof(_56) = & {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1128: b"reduce\0":   l94 = UNIQUE | NON_NULL, FIXED
        // 1128: b"reduce\0" as  ... _char: typeof(_0 = move _54 as *const i8 (Misc)) = *const {l354} i8
        // 1128: b"reduce\0" as  ... _char:   l354 = UNIQUE | NON_NULL, (empty)
        // 1128: b"reduce\0": typeof(_56 = const b"reduce\x00") = & {l351} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1128: b"reduce\0":   l351 = UNIQUE | NON_NULL, (empty)
        // 1128: b"reduce\0": typeof(_55 = &raw const (*_56)) = *const {l352} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1128: b"reduce\0":   l352 = UNIQUE | NON_NULL, (empty)
        // 1128: b"reduce\0" as  ... st u8: typeof(_54 = move _55 as *const u8 (Pointer(ArrayToPointer))) = *const {l353} u8
        // 1128: b"reduce\0" as  ... st u8:   l353 = UNIQUE | NON_NULL, (empty)
        19 => return b"flush\0" as *const u8 as *const libc::c_char,
        // 1129: b"flush\0" as * ... st u8: typeof(_58) = *const {l97} u8
        // 1129: b"flush\0" as * ... st u8:   l97 = UNIQUE | NON_NULL, (empty)
        // 1129: b"flush\0": typeof(_59) = *const {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1129: b"flush\0":   l99 = UNIQUE | NON_NULL, (empty)
        // 1129: b"flush\0": typeof(_60) = & {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1129: b"flush\0":   l101 = UNIQUE | NON_NULL, FIXED
        // 1129: b"flush\0" as * ... _char: typeof(_0 = move _58 as *const i8 (Misc)) = *const {l358} i8
        // 1129: b"flush\0" as * ... _char:   l358 = UNIQUE | NON_NULL, (empty)
        // 1129: b"flush\0": typeof(_59 = &raw const (*_60)) = *const {l356} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1129: b"flush\0":   l356 = UNIQUE | NON_NULL, (empty)
        // 1129: b"flush\0": typeof(_60 = const b"flush\x00") = & {l355} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1129: b"flush\0":   l355 = UNIQUE | NON_NULL, (empty)
        // 1129: b"flush\0" as * ... st u8: typeof(_58 = move _59 as *const u8 (Pointer(ArrayToPointer))) = *const {l357} u8
        // 1129: b"flush\0" as * ... st u8:   l357 = UNIQUE | NON_NULL, (empty)
        4 => return b"freeze\0" as *const u8 as *const libc::c_char,
        // 1130: b"freeze\0" as  ... st u8: typeof(_62) = *const {l104} u8
        // 1130: b"freeze\0" as  ... st u8:   l104 = UNIQUE | NON_NULL, (empty)
        // 1130: b"freeze\0": typeof(_63) = *const {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1130: b"freeze\0":   l106 = UNIQUE | NON_NULL, (empty)
        // 1130: b"freeze\0": typeof(_64) = & {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1130: b"freeze\0":   l108 = UNIQUE | NON_NULL, FIXED
        // 1130: b"freeze\0" as  ... _char: typeof(_0 = move _62 as *const i8 (Misc)) = *const {l362} i8
        // 1130: b"freeze\0" as  ... _char:   l362 = UNIQUE | NON_NULL, (empty)
        // 1130: b"freeze\0" as  ... st u8: typeof(_62 = move _63 as *const u8 (Pointer(ArrayToPointer))) = *const {l361} u8
        // 1130: b"freeze\0" as  ... st u8:   l361 = UNIQUE | NON_NULL, (empty)
        // 1130: b"freeze\0": typeof(_63 = &raw const (*_64)) = *const {l360} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1130: b"freeze\0":   l360 = UNIQUE | NON_NULL, (empty)
        // 1130: b"freeze\0": typeof(_64 = const b"freeze\x00") = & {l359} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1130: b"freeze\0":   l359 = UNIQUE | NON_NULL, (empty)
        15 => return b"setimportant\0" as *const u8 as *const libc::c_char,
        // 1131: b"setimportant\ ... st u8: typeof(_66) = *const {l111} u8
        // 1131: b"setimportant\ ... st u8:   l111 = UNIQUE | NON_NULL, (empty)
        // 1131: b"setimportant\0": typeof(_67) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1131: b"setimportant\0":   l113 = UNIQUE | NON_NULL, (empty)
        // 1131: b"setimportant\0": typeof(_68) = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1131: b"setimportant\0":   l115 = UNIQUE | NON_NULL, FIXED
        // 1131: b"setimportant\0": typeof(_68 = const b"setimportant\x00") = & {l363} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1131: b"setimportant\0":   l363 = UNIQUE | NON_NULL, (empty)
        // 1131: b"setimportant\0": typeof(_67 = &raw const (*_68)) = *const {l364} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1131: b"setimportant\0":   l364 = UNIQUE | NON_NULL, (empty)
        // 1131: b"setimportant\ ... _char: typeof(_0 = move _66 as *const i8 (Misc)) = *const {l366} i8
        // 1131: b"setimportant\ ... _char:   l366 = UNIQUE | NON_NULL, (empty)
        // 1131: b"setimportant\ ... st u8: typeof(_66 = move _67 as *const u8 (Pointer(ArrayToPointer))) = *const {l365} u8
        // 1131: b"setimportant\ ... st u8:   l365 = UNIQUE | NON_NULL, (empty)
        18 => return b"setphases\0" as *const u8 as *const libc::c_char,
        // 1132: b"setphases\0"  ... st u8: typeof(_70) = *const {l118} u8
        // 1132: b"setphases\0"  ... st u8:   l118 = UNIQUE | NON_NULL, (empty)
        // 1132: b"setphases\0": typeof(_71) = *const {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1132: b"setphases\0":   l120 = UNIQUE | NON_NULL, (empty)
        // 1132: b"setphases\0": typeof(_72) = & {l122} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1132: b"setphases\0":   l122 = UNIQUE | NON_NULL, FIXED
        // 1132: b"setphases\0": typeof(_72 = const b"setphases\x00") = & {l367} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1132: b"setphases\0":   l367 = UNIQUE | NON_NULL, (empty)
        // 1132: b"setphases\0": typeof(_71 = &raw const (*_72)) = *const {l368} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1132: b"setphases\0":   l368 = UNIQUE | NON_NULL, (empty)
        // 1132: b"setphases\0"  ... _char: typeof(_0 = move _70 as *const i8 (Misc)) = *const {l370} i8
        // 1132: b"setphases\0"  ... _char:   l370 = UNIQUE | NON_NULL, (empty)
        // 1132: b"setphases\0"  ... st u8: typeof(_70 = move _71 as *const u8 (Pointer(ArrayToPointer))) = *const {l369} u8
        // 1132: b"setphases\0"  ... st u8:   l369 = UNIQUE | NON_NULL, (empty)
        16 => return b"setphase\0" as *const u8 as *const libc::c_char,
        // 1133: b"setphase\0" a ... st u8: typeof(_74) = *const {l125} u8
        // 1133: b"setphase\0" a ... st u8:   l125 = UNIQUE | NON_NULL, (empty)
        // 1133: b"setphase\0": typeof(_75) = *const {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1133: b"setphase\0":   l127 = UNIQUE | NON_NULL, (empty)
        // 1133: b"setphase\0": typeof(_76) = & {l129} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1133: b"setphase\0":   l129 = UNIQUE | NON_NULL, FIXED
        // 1133: b"setphase\0": typeof(_76 = const b"setphase\x00") = & {l371} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1133: b"setphase\0":   l371 = UNIQUE | NON_NULL, (empty)
        // 1133: b"setphase\0" a ... _char: typeof(_0 = move _74 as *const i8 (Misc)) = *const {l374} i8
        // 1133: b"setphase\0" a ... _char:   l374 = UNIQUE | NON_NULL, (empty)
        // 1133: b"setphase\0": typeof(_75 = &raw const (*_76)) = *const {l372} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1133: b"setphase\0":   l372 = UNIQUE | NON_NULL, (empty)
        // 1133: b"setphase\0" a ... st u8: typeof(_74 = move _75 as *const u8 (Pointer(ArrayToPointer))) = *const {l373} u8
        // 1133: b"setphase\0" a ... st u8:   l373 = UNIQUE | NON_NULL, (empty)
        17 => return b"resetphase\0" as *const u8 as *const libc::c_char,
        // 1134: b"resetphase\0" ... st u8: typeof(_78) = *const {l132} u8
        // 1134: b"resetphase\0" ... st u8:   l132 = UNIQUE | NON_NULL, (empty)
        // 1134: b"resetphase\0": typeof(_79) = *const {l134} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1134: b"resetphase\0":   l134 = UNIQUE | NON_NULL, (empty)
        // 1134: b"resetphase\0": typeof(_80) = & {l136} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1134: b"resetphase\0":   l136 = UNIQUE | NON_NULL, FIXED
        // 1134: b"resetphase\0" ... st u8: typeof(_78 = move _79 as *const u8 (Pointer(ArrayToPointer))) = *const {l377} u8
        // 1134: b"resetphase\0" ... st u8:   l377 = UNIQUE | NON_NULL, (empty)
        // 1134: b"resetphase\0": typeof(_80 = const b"resetphase\x00") = & {l375} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1134: b"resetphase\0":   l375 = UNIQUE | NON_NULL, (empty)
        // 1134: b"resetphase\0": typeof(_79 = &raw const (*_80)) = *const {l376} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 1134: b"resetphase\0":   l376 = UNIQUE | NON_NULL, (empty)
        // 1134: b"resetphase\0" ... _char: typeof(_0 = move _78 as *const i8 (Misc)) = *const {l378} i8
        // 1134: b"resetphase\0" ... _char:   l378 = UNIQUE | NON_NULL, (empty)
        25 => return b"incvar\0" as *const u8 as *const libc::c_char,
        // 1135: b"incvar\0" as  ... st u8: typeof(_82) = *const {l139} u8
        // 1135: b"incvar\0" as  ... st u8:   l139 = UNIQUE | NON_NULL, (empty)
        // 1135: b"incvar\0": typeof(_83) = *const {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1135: b"incvar\0":   l141 = UNIQUE | NON_NULL, (empty)
        // 1135: b"incvar\0": typeof(_84) = & {l143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1135: b"incvar\0":   l143 = UNIQUE | NON_NULL, FIXED
        // 1135: b"incvar\0" as  ... _char: typeof(_0 = move _82 as *const i8 (Misc)) = *const {l382} i8
        // 1135: b"incvar\0" as  ... _char:   l382 = UNIQUE | NON_NULL, (empty)
        // 1135: b"incvar\0": typeof(_84 = const b"incvar\x00") = & {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1135: b"incvar\0":   l379 = UNIQUE | NON_NULL, (empty)
        // 1135: b"incvar\0": typeof(_83 = &raw const (*_84)) = *const {l380} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1135: b"incvar\0":   l380 = UNIQUE | NON_NULL, (empty)
        // 1135: b"incvar\0" as  ... st u8: typeof(_82 = move _83 as *const u8 (Pointer(ArrayToPointer))) = *const {l381} u8
        // 1135: b"incvar\0" as  ... st u8:   l381 = UNIQUE | NON_NULL, (empty)
        30 => return b"inconsistent\0" as *const u8 as *const libc::c_char,
        // 1136: b"inconsistent\ ... st u8: typeof(_86) = *const {l146} u8
        // 1136: b"inconsistent\ ... st u8:   l146 = UNIQUE | NON_NULL, (empty)
        // 1136: b"inconsistent\0": typeof(_87) = *const {l148} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1136: b"inconsistent\0":   l148 = UNIQUE | NON_NULL, (empty)
        // 1136: b"inconsistent\0": typeof(_88) = & {l150} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1136: b"inconsistent\0":   l150 = UNIQUE | NON_NULL, FIXED
        // 1136: b"inconsistent\ ... st u8: typeof(_86 = move _87 as *const u8 (Pointer(ArrayToPointer))) = *const {l385} u8
        // 1136: b"inconsistent\ ... st u8:   l385 = UNIQUE | NON_NULL, (empty)
        // 1136: b"inconsistent\0": typeof(_88 = const b"inconsistent\x00") = & {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1136: b"inconsistent\0":   l383 = UNIQUE | NON_NULL, (empty)
        // 1136: b"inconsistent\0": typeof(_87 = &raw const (*_88)) = *const {l384} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1136: b"inconsistent\0":   l384 = UNIQUE | NON_NULL, (empty)
        // 1136: b"inconsistent\ ... _char: typeof(_0 = move _86 as *const i8 (Misc)) = *const {l386} i8
        // 1136: b"inconsistent\ ... _char:   l386 = UNIQUE | NON_NULL, (empty)
        31 => return b"lkhd\0" as *const u8 as *const libc::c_char,
        // 1137: b"lkhd\0" as *c ... st u8: typeof(_90) = *const {l153} u8
        // 1137: b"lkhd\0" as *c ... st u8:   l153 = UNIQUE | NON_NULL, (empty)
        // 1137: b"lkhd\0": typeof(_91) = *const {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1137: b"lkhd\0":   l155 = UNIQUE | NON_NULL, (empty)
        // 1137: b"lkhd\0": typeof(_92) = & {l157} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1137: b"lkhd\0":   l157 = UNIQUE | NON_NULL, FIXED
        // 1137: b"lkhd\0" as *c ... st u8: typeof(_90 = move _91 as *const u8 (Pointer(ArrayToPointer))) = *const {l389} u8
        // 1137: b"lkhd\0" as *c ... st u8:   l389 = UNIQUE | NON_NULL, (empty)
        // 1137: b"lkhd\0": typeof(_92 = const b"lkhd\x00") = & {l387} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1137: b"lkhd\0":   l387 = UNIQUE | NON_NULL, (empty)
        // 1137: b"lkhd\0": typeof(_91 = &raw const (*_92)) = *const {l388} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1137: b"lkhd\0":   l388 = UNIQUE | NON_NULL, (empty)
        // 1137: b"lkhd\0" as *c ... _char: typeof(_0 = move _90 as *const i8 (Misc)) = *const {l390} i8
        // 1137: b"lkhd\0" as *c ... _char:   l390 = UNIQUE | NON_NULL, (empty)
        5 => return b"init\0" as *const u8 as *const libc::c_char,
        // 1138: b"init\0" as *c ... st u8: typeof(_94) = *const {l160} u8
        // 1138: b"init\0" as *c ... st u8:   l160 = UNIQUE | NON_NULL, (empty)
        // 1138: b"init\0": typeof(_95) = *const {l162} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1138: b"init\0":   l162 = UNIQUE | NON_NULL, (empty)
        // 1138: b"init\0": typeof(_96) = & {l164} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1138: b"init\0":   l164 = UNIQUE | NON_NULL, FIXED
        // 1138: b"init\0": typeof(_95 = &raw const (*_96)) = *const {l392} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1138: b"init\0":   l392 = UNIQUE | NON_NULL, (empty)
        // 1138: b"init\0" as *c ... st u8: typeof(_94 = move _95 as *const u8 (Pointer(ArrayToPointer))) = *const {l393} u8
        // 1138: b"init\0" as *c ... st u8:   l393 = UNIQUE | NON_NULL, (empty)
        // 1138: b"init\0": typeof(_96 = const b"init\x00") = & {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1138: b"init\0":   l391 = UNIQUE | NON_NULL, (empty)
        // 1138: b"init\0" as *c ... _char: typeof(_0 = move _94 as *const i8 (Misc)) = *const {l394} i8
        // 1138: b"init\0" as *c ... _char:   l394 = UNIQUE | NON_NULL, (empty)
        24 => return b"maxvar\0" as *const u8 as *const libc::c_char,
        // 1139: b"maxvar\0" as  ... st u8: typeof(_98) = *const {l167} u8
        // 1139: b"maxvar\0" as  ... st u8:   l167 = UNIQUE | NON_NULL, (empty)
        // 1139: b"maxvar\0": typeof(_99) = *const {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1139: b"maxvar\0":   l169 = UNIQUE | NON_NULL, (empty)
        // 1139: b"maxvar\0": typeof(_100) = & {l171} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1139: b"maxvar\0":   l171 = UNIQUE | NON_NULL, FIXED
        // 1139: b"maxvar\0" as  ... st u8: typeof(_98 = move _99 as *const u8 (Pointer(ArrayToPointer))) = *const {l397} u8
        // 1139: b"maxvar\0" as  ... st u8:   l397 = UNIQUE | NON_NULL, (empty)
        // 1139: b"maxvar\0" as  ... _char: typeof(_0 = move _98 as *const i8 (Misc)) = *const {l398} i8
        // 1139: b"maxvar\0" as  ... _char:   l398 = UNIQUE | NON_NULL, (empty)
        // 1139: b"maxvar\0": typeof(_100 = const b"maxvar\x00") = & {l395} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1139: b"maxvar\0":   l395 = UNIQUE | NON_NULL, (empty)
        // 1139: b"maxvar\0": typeof(_99 = &raw const (*_100)) = *const {l396} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1139: b"maxvar\0":   l396 = UNIQUE | NON_NULL, (empty)
        6 => return b"melt\0" as *const u8 as *const libc::c_char,
        // 1140: b"melt\0" as *c ... st u8: typeof(_102) = *const {l174} u8
        // 1140: b"melt\0" as *c ... st u8:   l174 = UNIQUE | NON_NULL, (empty)
        // 1140: b"melt\0": typeof(_103) = *const {l176} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1140: b"melt\0":   l176 = UNIQUE | NON_NULL, (empty)
        // 1140: b"melt\0": typeof(_104) = & {l178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1140: b"melt\0":   l178 = UNIQUE | NON_NULL, FIXED
        // 1140: b"melt\0": typeof(_104 = const b"melt\x00") = & {l399} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1140: b"melt\0":   l399 = UNIQUE | NON_NULL, (empty)
        // 1140: b"melt\0" as *c ... _char: typeof(_0 = move _102 as *const i8 (Misc)) = *const {l402} i8
        // 1140: b"melt\0" as *c ... _char:   l402 = UNIQUE | NON_NULL, (empty)
        // 1140: b"melt\0" as *c ... st u8: typeof(_102 = move _103 as *const u8 (Pointer(ArrayToPointer))) = *const {l401} u8
        // 1140: b"melt\0" as *c ... st u8:   l401 = UNIQUE | NON_NULL, (empty)
        // 1140: b"melt\0": typeof(_103 = &raw const (*_104)) = *const {l400} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1140: b"melt\0":   l400 = UNIQUE | NON_NULL, (empty)
        7 => return b"REUSE\0" as *const u8 as *const libc::c_char,
        // 1141: b"REUSE\0" as * ... st u8: typeof(_106) = *const {l181} u8
        // 1141: b"REUSE\0" as * ... st u8:   l181 = UNIQUE | NON_NULL, (empty)
        // 1141: b"REUSE\0": typeof(_107) = *const {l183} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1141: b"REUSE\0":   l183 = UNIQUE | NON_NULL, (empty)
        // 1141: b"REUSE\0": typeof(_108) = & {l185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1141: b"REUSE\0":   l185 = UNIQUE | NON_NULL, FIXED
        // 1141: b"REUSE\0": typeof(_108 = const b"REUSE\x00") = & {l403} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1141: b"REUSE\0":   l403 = UNIQUE | NON_NULL, (empty)
        // 1141: b"REUSE\0" as * ... st u8: typeof(_106 = move _107 as *const u8 (Pointer(ArrayToPointer))) = *const {l405} u8
        // 1141: b"REUSE\0" as * ... st u8:   l405 = UNIQUE | NON_NULL, (empty)
        // 1141: b"REUSE\0": typeof(_107 = &raw const (*_108)) = *const {l404} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1141: b"REUSE\0":   l404 = UNIQUE | NON_NULL, (empty)
        // 1141: b"REUSE\0" as * ... _char: typeof(_0 = move _106 as *const i8 (Misc)) = *const {l406} i8
        // 1141: b"REUSE\0" as * ... _char:   l406 = UNIQUE | NON_NULL, (empty)
        8 => return b"option\0" as *const u8 as *const libc::c_char,
        // 1142: b"option\0" as  ... st u8: typeof(_110) = *const {l188} u8
        // 1142: b"option\0" as  ... st u8:   l188 = UNIQUE | NON_NULL, (empty)
        // 1142: b"option\0": typeof(_111) = *const {l190} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1142: b"option\0":   l190 = UNIQUE | NON_NULL, (empty)
        // 1142: b"option\0": typeof(_112) = & {l192} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1142: b"option\0":   l192 = UNIQUE | NON_NULL, FIXED
        // 1142: b"option\0" as  ... _char: typeof(_0 = move _110 as *const i8 (Misc)) = *const {l410} i8
        // 1142: b"option\0" as  ... _char:   l410 = UNIQUE | NON_NULL, (empty)
        // 1142: b"option\0": typeof(_112 = const b"option\x00") = & {l407} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1142: b"option\0":   l407 = UNIQUE | NON_NULL, (empty)
        // 1142: b"option\0": typeof(_111 = &raw const (*_112)) = *const {l408} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1142: b"option\0":   l408 = UNIQUE | NON_NULL, (empty)
        // 1142: b"option\0" as  ... st u8: typeof(_110 = move _111 as *const u8 (Pointer(ArrayToPointer))) = *const {l409} u8
        // 1142: b"option\0" as  ... st u8:   l409 = UNIQUE | NON_NULL, (empty)
        9 => return b"phase\0" as *const u8 as *const libc::c_char,
        // 1143: b"phase\0" as * ... st u8: typeof(_114) = *const {l195} u8
        // 1143: b"phase\0" as * ... st u8:   l195 = UNIQUE | NON_NULL, (empty)
        // 1143: b"phase\0": typeof(_115) = *const {l197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1143: b"phase\0":   l197 = UNIQUE | NON_NULL, (empty)
        // 1143: b"phase\0": typeof(_116) = & {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1143: b"phase\0":   l199 = UNIQUE | NON_NULL, FIXED
        // 1143: b"phase\0" as * ... _char: typeof(_0 = move _114 as *const i8 (Misc)) = *const {l414} i8
        // 1143: b"phase\0" as * ... _char:   l414 = UNIQUE | NON_NULL, (empty)
        // 1143: b"phase\0": typeof(_115 = &raw const (*_116)) = *const {l412} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1143: b"phase\0":   l412 = UNIQUE | NON_NULL, (empty)
        // 1143: b"phase\0" as * ... st u8: typeof(_114 = move _115 as *const u8 (Pointer(ArrayToPointer))) = *const {l413} u8
        // 1143: b"phase\0" as * ... st u8:   l413 = UNIQUE | NON_NULL, (empty)
        // 1143: b"phase\0": typeof(_116 = const b"phase\x00") = & {l411} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1143: b"phase\0":   l411 = UNIQUE | NON_NULL, (empty)
        10 => return b"release\0" as *const u8 as *const libc::c_char,
        // 1144: b"release\0" as ... st u8: typeof(_118) = *const {l202} u8
        // 1144: b"release\0" as ... st u8:   l202 = UNIQUE | NON_NULL, (empty)
        // 1144: b"release\0": typeof(_119) = *const {l204} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1144: b"release\0":   l204 = UNIQUE | NON_NULL, (empty)
        // 1144: b"release\0": typeof(_120) = & {l206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1144: b"release\0":   l206 = UNIQUE | NON_NULL, FIXED
        // 1144: b"release\0" as ... st u8: typeof(_118 = move _119 as *const u8 (Pointer(ArrayToPointer))) = *const {l417} u8
        // 1144: b"release\0" as ... st u8:   l417 = UNIQUE | NON_NULL, (empty)
        // 1144: b"release\0" as ... _char: typeof(_0 = move _118 as *const i8 (Misc)) = *const {l418} i8
        // 1144: b"release\0" as ... _char:   l418 = UNIQUE | NON_NULL, (empty)
        // 1144: b"release\0": typeof(_120 = const b"release\x00") = & {l415} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1144: b"release\0":   l415 = UNIQUE | NON_NULL, (empty)
        // 1144: b"release\0": typeof(_119 = &raw const (*_120)) = *const {l416} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1144: b"release\0":   l416 = UNIQUE | NON_NULL, (empty)
        11 => return b"return\0" as *const u8 as *const libc::c_char,
        // 1145: b"return\0" as  ... st u8: typeof(_122) = *const {l209} u8
        // 1145: b"return\0" as  ... st u8:   l209 = UNIQUE | NON_NULL, (empty)
        // 1145: b"return\0": typeof(_123) = *const {l211} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1145: b"return\0":   l211 = UNIQUE | NON_NULL, (empty)
        // 1145: b"return\0": typeof(_124) = & {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1145: b"return\0":   l213 = UNIQUE | NON_NULL, FIXED
        // 1145: b"return\0" as  ... st u8: typeof(_122 = move _123 as *const u8 (Pointer(ArrayToPointer))) = *const {l421} u8
        // 1145: b"return\0" as  ... st u8:   l421 = UNIQUE | NON_NULL, (empty)
        // 1145: b"return\0" as  ... _char: typeof(_0 = move _122 as *const i8 (Misc)) = *const {l422} i8
        // 1145: b"return\0" as  ... _char:   l422 = UNIQUE | NON_NULL, (empty)
        // 1145: b"return\0": typeof(_123 = &raw const (*_124)) = *const {l420} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1145: b"return\0":   l420 = UNIQUE | NON_NULL, (empty)
        // 1145: b"return\0": typeof(_124 = const b"return\x00") = & {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1145: b"return\0":   l419 = UNIQUE | NON_NULL, (empty)
        13 => return b"simp\0" as *const u8 as *const libc::c_char,
        // 1146: b"simp\0" as *c ... st u8: typeof(_126) = *const {l216} u8
        // 1146: b"simp\0" as *c ... st u8:   l216 = UNIQUE | NON_NULL, (empty)
        // 1146: b"simp\0": typeof(_127) = *const {l218} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1146: b"simp\0":   l218 = UNIQUE | NON_NULL, (empty)
        // 1146: b"simp\0": typeof(_128) = & {l220} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1146: b"simp\0":   l220 = UNIQUE | NON_NULL, FIXED
        // 1146: b"simp\0" as *c ... st u8: typeof(_126 = move _127 as *const u8 (Pointer(ArrayToPointer))) = *const {l425} u8
        // 1146: b"simp\0" as *c ... st u8:   l425 = UNIQUE | NON_NULL, (empty)
        // 1146: b"simp\0": typeof(_127 = &raw const (*_128)) = *const {l424} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1146: b"simp\0":   l424 = UNIQUE | NON_NULL, (empty)
        // 1146: b"simp\0" as *c ... _char: typeof(_0 = move _126 as *const i8 (Misc)) = *const {l426} i8
        // 1146: b"simp\0" as *c ... _char:   l426 = UNIQUE | NON_NULL, (empty)
        // 1146: b"simp\0": typeof(_128 = const b"simp\x00") = & {l423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1146: b"simp\0":   l423 = UNIQUE | NON_NULL, (empty)
        12 | _ => {
            if type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {
            } else {
                __assert_fail(
                    b"type == SAT\0" as *const u8 as *const libc::c_char,
                    // 1151: b"type == SAT\0 ... _char: typeof(_137) = *const {l230} i8
                    // 1151: b"type == SAT\0 ... _char:   l230 = UNIQUE | NON_NULL, (empty)
                    // 1151: b"type == SAT\0 ... st u8: typeof(_138) = *const {l232} u8
                    // 1151: b"type == SAT\0 ... st u8:   l232 = UNIQUE | NON_NULL, (empty)
                    // 1151: b"type == SAT\0": typeof(_139) = *const {l234} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1151: b"type == SAT\0":   l234 = UNIQUE | NON_NULL, (empty)
                    // 1151: b"type == SAT\0": typeof(_140) = & {l236} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1151: b"type == SAT\0":   l236 = UNIQUE | NON_NULL, FIXED
                    // 1151: b"type == SAT\0 ... _char: typeof(_137 = move _138 as *const i8 (Misc)) = *const {l430} i8
                    // 1151: b"type == SAT\0 ... _char:   l430 = UNIQUE | NON_NULL, (empty)
                    // 1151: b"type == SAT\0": typeof(_139 = &raw const (*_140)) = *const {l428} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1151: b"type == SAT\0":   l428 = UNIQUE | NON_NULL, (empty)
                    // 1151: b"type == SAT\0 ... st u8: typeof(_138 = move _139 as *const u8 (Pointer(ArrayToPointer))) = *const {l429} u8
                    // 1151: b"type == SAT\0 ... st u8:   l429 = UNIQUE | NON_NULL, (empty)
                    // 1151: b"type == SAT\0": typeof(_140 = const b"type == SAT\x00") = & {l427} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1151: b"type == SAT\0":   l427 = UNIQUE | NON_NULL, (empty)
                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                    // 1152: b"lglddtrace.c\ ... _char: typeof(_141) = *const {l238} i8
                    // 1152: b"lglddtrace.c\ ... _char:   l238 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"lglddtrace.c\ ... st u8: typeof(_142) = *const {l240} u8
                    // 1152: b"lglddtrace.c\ ... st u8:   l240 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"lglddtrace.c\0": typeof(_143) = *const {l242} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1152: b"lglddtrace.c\0":   l242 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"lglddtrace.c\0": typeof(_144) = & {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1152: b"lglddtrace.c\0":   l244 = UNIQUE | NON_NULL, FIXED
                    // 1152: b"lglddtrace.c\ ... st u8: typeof(_142 = move _143 as *const u8 (Pointer(ArrayToPointer))) = *const {l433} u8
                    // 1152: b"lglddtrace.c\ ... st u8:   l433 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"lglddtrace.c\0": typeof(_144 = const b"lglddtrace.c\x00") = & {l431} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1152: b"lglddtrace.c\0":   l431 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"lglddtrace.c\ ... _char: typeof(_141 = move _142 as *const i8 (Misc)) = *const {l434} i8
                    // 1152: b"lglddtrace.c\ ... _char:   l434 = UNIQUE | NON_NULL, (empty)
                    // 1152: b"lglddtrace.c\0": typeof(_143 = &raw const (*_144)) = *const {l432} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1152: b"lglddtrace.c\0":   l432 = UNIQUE | NON_NULL, (empty)
                    356 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                    // 1154: (*::core::mem:: ... ptr(): typeof(_147) = *const {l248} i8
                    // 1154: (*::core::mem:: ... ptr():   l248 = UNIQUE | NON_NULL, (empty)
                    // 1154: (*::core::mem:: ... ptr(): typeof(_148) = & {l250} [i8]
                    // 1154: (*::core::mem:: ... ptr():   l250 = UNIQUE | NON_NULL, FIXED
                    // 1154: (*::core::mem:: ... ptr(): typeof(_149) = & {l252} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                    // 1154: (*::core::mem:: ... ptr():   l252 = UNIQUE | NON_NULL, (empty)
                    // 1154: ::core::mem::tr ... 0", ): typeof(_150) = & {l254} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                    // 1154: ::core::mem::tr ... 0", ):   l254 = UNIQUE | NON_NULL, FIXED
                    // 1154: (*::core::mem:: ... ptr(): typeof(_148 = move _149 as &[i8] (Pointer(Unsize))) = & {l438} [i8]
                    // 1154: (*::core::mem:: ... ptr():   l438 = UNIQUE | NON_NULL, FIXED
                    // 1154: (*::core::mem:: ... ptr(): typeof(_149 = &(*_150)) = & {l437} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                    // 1154: (*::core::mem:: ... ptr():   l437 = UNIQUE | NON_NULL, (empty)
                        b"const char *type2str(Type)\0",
                        // 1155: b"const char *t ... e)\0": typeof(_151) = & {l256} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1155: b"const char *t ... e)\0":   l256 = UNIQUE | NON_NULL, (empty)
                        // 1155: b"const char *t ... e)\0": typeof(_152) = & {l258} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1155: b"const char *t ... e)\0":   l258 = UNIQUE | NON_NULL, FIXED
                        // 1155: b"const char *t ... e)\0": typeof(_152 = const b"const char *type2str(Type)\x00") = & {l435} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1155: b"const char *t ... e)\0":   l435 = UNIQUE | NON_NULL, (empty)
                        // 1155: b"const char *t ... e)\0": typeof(_151 = &(*_152)) = & {l436} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1155: b"const char *t ... e)\0":   l436 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
            'c_7023: {
                if type_0 as libc::c_uint == SAT as libc::c_int as libc::c_uint {
                } else {
                    __assert_fail(
                        b"type == SAT\0" as *const u8 as *const libc::c_char,
                        // 1164: b"type == SAT\0 ... _char: typeof(_160) = *const {l267} i8
                        // 1164: b"type == SAT\0 ... _char:   l267 = UNIQUE | NON_NULL, (empty)
                        // 1164: b"type == SAT\0 ... st u8: typeof(_161) = *const {l269} u8
                        // 1164: b"type == SAT\0 ... st u8:   l269 = UNIQUE | NON_NULL, (empty)
                        // 1164: b"type == SAT\0": typeof(_162) = *const {l271} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                        // 1164: b"type == SAT\0":   l271 = UNIQUE | NON_NULL, (empty)
                        // 1164: b"type == SAT\0": typeof(_163) = & {l273} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                        // 1164: b"type == SAT\0":   l273 = UNIQUE | NON_NULL, FIXED
                        // 1164: b"type == SAT\0 ... _char: typeof(_160 = move _161 as *const i8 (Misc)) = *const {l442} i8
                        // 1164: b"type == SAT\0 ... _char:   l442 = UNIQUE | NON_NULL, (empty)
                        // 1164: b"type == SAT\0": typeof(_162 = &raw const (*_163)) = *const {l440} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                        // 1164: b"type == SAT\0":   l440 = UNIQUE | NON_NULL, (empty)
                        // 1164: b"type == SAT\0": typeof(_163 = const b"type == SAT\x00") = & {l439} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                        // 1164: b"type == SAT\0":   l439 = UNIQUE | NON_NULL, (empty)
                        // 1164: b"type == SAT\0 ... st u8: typeof(_161 = move _162 as *const u8 (Pointer(ArrayToPointer))) = *const {l441} u8
                        // 1164: b"type == SAT\0 ... st u8:   l441 = UNIQUE | NON_NULL, (empty)
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        // 1165: b"lglddtrace.c\ ... _char: typeof(_164) = *const {l275} i8
                        // 1165: b"lglddtrace.c\ ... _char:   l275 = UNIQUE | NON_NULL, (empty)
                        // 1165: b"lglddtrace.c\ ... st u8: typeof(_165) = *const {l277} u8
                        // 1165: b"lglddtrace.c\ ... st u8:   l277 = UNIQUE | NON_NULL, (empty)
                        // 1165: b"lglddtrace.c\0": typeof(_166) = *const {l279} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1165: b"lglddtrace.c\0":   l279 = UNIQUE | NON_NULL, (empty)
                        // 1165: b"lglddtrace.c\0": typeof(_167) = & {l281} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1165: b"lglddtrace.c\0":   l281 = UNIQUE | NON_NULL, FIXED
                        // 1165: b"lglddtrace.c\ ... st u8: typeof(_165 = move _166 as *const u8 (Pointer(ArrayToPointer))) = *const {l445} u8
                        // 1165: b"lglddtrace.c\ ... st u8:   l445 = UNIQUE | NON_NULL, (empty)
                        // 1165: b"lglddtrace.c\ ... _char: typeof(_164 = move _165 as *const i8 (Misc)) = *const {l446} i8
                        // 1165: b"lglddtrace.c\ ... _char:   l446 = UNIQUE | NON_NULL, (empty)
                        // 1165: b"lglddtrace.c\0": typeof(_166 = &raw const (*_167)) = *const {l444} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1165: b"lglddtrace.c\0":   l444 = UNIQUE | NON_NULL, (empty)
                        // 1165: b"lglddtrace.c\0": typeof(_167 = const b"lglddtrace.c\x00") = & {l443} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1165: b"lglddtrace.c\0":   l443 = UNIQUE | NON_NULL, (empty)
                        356 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 27], &[libc::c_char; 27]>(
                        // 1167: (*::core::mem:: ... ptr(): typeof(_170) = *const {l285} i8
                        // 1167: (*::core::mem:: ... ptr():   l285 = UNIQUE | NON_NULL, (empty)
                        // 1167: (*::core::mem:: ... ptr(): typeof(_171) = & {l287} [i8]
                        // 1167: (*::core::mem:: ... ptr():   l287 = UNIQUE | NON_NULL, FIXED
                        // 1167: (*::core::mem:: ... ptr(): typeof(_172) = & {l289} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1167: (*::core::mem:: ... ptr():   l289 = UNIQUE | NON_NULL, (empty)
                        // 1167: ::core::mem::tr ... 0", ): typeof(_173) = & {l291} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1167: ::core::mem::tr ... 0", ):   l291 = UNIQUE | NON_NULL, FIXED
                        // 1167: (*::core::mem:: ... ptr(): typeof(_171 = move _172 as &[i8] (Pointer(Unsize))) = & {l450} [i8]
                        // 1167: (*::core::mem:: ... ptr():   l450 = UNIQUE | NON_NULL, FIXED
                        // 1167: (*::core::mem:: ... ptr(): typeof(_172 = &(*_173)) = & {l449} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                        // 1167: (*::core::mem:: ... ptr():   l449 = UNIQUE | NON_NULL, (empty)
                            b"const char *type2str(Type)\0",
                            // 1168: b"const char *t ... e)\0": typeof(_174) = & {l293} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1168: b"const char *t ... e)\0":   l293 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"const char *t ... e)\0": typeof(_175) = & {l295} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1168: b"const char *t ... e)\0":   l295 = UNIQUE | NON_NULL, FIXED
                            // 1168: b"const char *t ... e)\0": typeof(_175 = const b"const char *type2str(Type)\x00") = & {l447} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1168: b"const char *t ... e)\0":   l447 = UNIQUE | NON_NULL, (empty)
                            // 1168: b"const char *t ... e)\0": typeof(_174 = &(*_175)) = & {l448} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                            // 1168: b"const char *t ... e)\0":   l448 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
            };
            return b"sat\0" as *const u8 as *const libc::c_char;
            // 1174: b"sat\0" as *co ... st u8: typeof(_176) = *const {l297} u8
            // 1174: b"sat\0" as *co ... st u8:   l297 = UNIQUE | NON_NULL, (empty)
            // 1174: b"sat\0": typeof(_177) = *const {l299} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1174: b"sat\0":   l299 = UNIQUE | NON_NULL, (empty)
            // 1174: b"sat\0": typeof(_178) = & {l301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1174: b"sat\0":   l301 = UNIQUE | NON_NULL, FIXED
            // 1174: b"sat\0" as *co ... st u8: typeof(_176 = move _177 as *const u8 (Pointer(ArrayToPointer))) = *const {l453} u8
            // 1174: b"sat\0" as *co ... st u8:   l453 = UNIQUE | NON_NULL, (empty)
            // 1174: b"sat\0": typeof(_178 = const b"sat\x00") = & {l451} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1174: b"sat\0":   l451 = UNIQUE | NON_NULL, (empty)
            // 1174: b"sat\0": typeof(_177 = &raw const (*_178)) = *const {l452} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1174: b"sat\0":   l452 = UNIQUE | NON_NULL, (empty)
            // 1174: b"sat\0" as *co ... _char: typeof(_0 = move _176 as *const i8 (Misc)) = *const {l454} i8
            // 1174: b"sat\0" as *co ... _char:   l454 = UNIQUE | NON_NULL, (empty)
        }
    };
}
unsafe extern "C" fn noarg(mut op: Type) {
    if !(strtok(
    // 1179: (strtok( 0 as * ... r, )): typeof(_5) = *mut {l5} i8
    // 1179: (strtok( 0 as * ... r, )):   l5 = UNIQUE | NON_NULL, (empty)
        0 as *mut libc::c_char,
        // 1180: 0 as *mut libc: ... _char: typeof(_6) = *mut {l7} i8
        // 1180: 0 as *mut libc: ... _char:   l7 = UNIQUE | NON_NULL, (empty)
        // 1180: 0 as *mut libc: ... _char: typeof(_6 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l34} i8
        // 1180: 0 as *mut libc: ... _char:   l34 = UNIQUE | NON_NULL, (empty)
        b" \0" as *const u8 as *const libc::c_char,
        // 1181: b" \0" as *cons ... _char: typeof(_7) = *const {l9} i8
        // 1181: b" \0" as *cons ... _char:   l9 = UNIQUE | NON_NULL, (empty)
        // 1181: b" \0" as *const u8: typeof(_8) = *const {l11} u8
        // 1181: b" \0" as *const u8:   l11 = UNIQUE | NON_NULL, (empty)
        // 1181: b" \0": typeof(_9) = *const {l13} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1181: b" \0":   l13 = UNIQUE | NON_NULL, (empty)
        // 1181: b" \0": typeof(_10) = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1181: b" \0":   l15 = UNIQUE | NON_NULL, FIXED
        // 1181: b" \0" as *cons ... _char: typeof(_7 = move _8 as *const i8 (Misc)) = *const {l38} i8
        // 1181: b" \0" as *cons ... _char:   l38 = UNIQUE | NON_NULL, (empty)
        // 1181: b" \0": typeof(_10 = const b" \x00") = & {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1181: b" \0":   l35 = UNIQUE | NON_NULL, (empty)
        // 1181: b" \0": typeof(_9 = &raw const (*_10)) = *const {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1181: b" \0":   l36 = UNIQUE | NON_NULL, (empty)
        // 1181: b" \0" as *const u8: typeof(_8 = move _9 as *const u8 (Pointer(ArrayToPointer))) = *const {l37} u8
        // 1181: b" \0" as *const u8:   l37 = UNIQUE | NON_NULL, (empty)
    ))
    .is_null()
    {
        perr(
            b"argument after '%s'\0" as *const u8 as *const libc::c_char,
            // 1186: b"argument afte ... _char: typeof(_12) = *const {l18} i8
            // 1186: b"argument afte ... _char:   l18 = UNIQUE | NON_NULL, (empty)
            // 1186: b"argument afte ... st u8: typeof(_13) = *const {l20} u8
            // 1186: b"argument afte ... st u8:   l20 = UNIQUE | NON_NULL, (empty)
            // 1186: b"argument afte ... s'\0": typeof(_14) = *const {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 1186: b"argument afte ... s'\0":   l22 = UNIQUE | NON_NULL, (empty)
            // 1186: b"argument afte ... s'\0": typeof(_15) = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 1186: b"argument afte ... s'\0":   l24 = UNIQUE | NON_NULL, FIXED
            // 1186: b"argument afte ... st u8: typeof(_13 = move _14 as *const u8 (Pointer(ArrayToPointer))) = *const {l41} u8
            // 1186: b"argument afte ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
            // 1186: b"argument afte ... s'\0": typeof(_14 = &raw const (*_15)) = *const {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 1186: b"argument afte ... s'\0":   l40 = UNIQUE | NON_NULL, (empty)
            // 1186: b"argument afte ... _char: typeof(_12 = move _13 as *const i8 (Misc)) = *const {l42} i8
            // 1186: b"argument afte ... _char:   l42 = UNIQUE | NON_NULL, (empty)
            // 1186: b"argument afte ... s'\0": typeof(_15 = const b"argument after \'%s\'\x00") = & {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 1186: b"argument afte ... s'\0":   l39 = UNIQUE | NON_NULL, (empty)
            type2str(op),
            // 1187: type2str(op): typeof(_16) = *const {l26} i8
            // 1187: type2str(op):   l26 = UNIQUE | NON_NULL, (empty)
        );
    }
    event(op, 0 as libc::c_int, 0 as *const libc::c_char);
    // 1190: 0 as *const lib ... _char: typeof(_21) = *const {l32} i8
    // 1190: 0 as *const lib ... _char:   l32 = UNIQUE | NON_NULL, (empty)
    // 1190: 0 as *const lib ... _char: typeof(_21 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l43} i8
    // 1190: 0 as *const lib ... _char:   l43 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn newline() {
    let mut i: libc::c_int = 0;
    if verbose == 0 {
    // 1194: verbose: typeof(_5) = *mut {l5} i32
    // 1194: verbose:   l5 = UNIQUE | NON_NULL, (empty)
        return;
    }
    if isatty(1 as libc::c_int) == 0 {
        return;
    }
    printf(b"\r\0" as *const u8 as *const libc::c_char);
    // 1200: b"\r\0" as *con ... _char: typeof(_13) = *const {l14} i8
    // 1200: b"\r\0" as *con ... _char:   l14 = UNIQUE | NON_NULL, (empty)
    // 1200: b"\r\0" as *const u8: typeof(_14) = *const {l16} u8
    // 1200: b"\r\0" as *const u8:   l16 = UNIQUE | NON_NULL, (empty)
    // 1200: b"\r\0": typeof(_15) = *const {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1200: b"\r\0":   l18 = UNIQUE | NON_NULL, (empty)
    // 1200: b"\r\0": typeof(_16) = & {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1200: b"\r\0":   l20 = UNIQUE | NON_NULL, FIXED
    // 1200: b"\r\0": typeof(_15 = &raw const (*_16)) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1200: b"\r\0":   l56 = UNIQUE | NON_NULL, (empty)
    // 1200: b"\r\0" as *con ... _char: typeof(_13 = move _14 as *const i8 (Misc)) = *const {l58} i8
    // 1200: b"\r\0" as *con ... _char:   l58 = UNIQUE | NON_NULL, (empty)
    // 1200: b"\r\0": typeof(_16 = const b"\r\x00") = & {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1200: b"\r\0":   l55 = UNIQUE | NON_NULL, (empty)
    // 1200: b"\r\0" as *const u8: typeof(_14 = move _15 as *const u8 (Pointer(ArrayToPointer))) = *const {l57} u8
    // 1200: b"\r\0" as *const u8:   l57 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < 78 as libc::c_int {
        fputc(' ' as i32, stdout);
        // 1203: stdout: typeof(_25) = *mut {l30} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1203: stdout:   l30 = UNIQUE | NON_NULL, (empty)
        // 1203: stdout: typeof(_26) = *mut {l32} *mut {l33} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1203: stdout:   l32 = UNIQUE | NON_NULL, (empty)
        // 1203: stdout:   l33 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    printf(b"\r\0" as *const u8 as *const libc::c_char);
    // 1207: b"\r\0" as *con ... _char: typeof(_33) = *const {l41} i8
    // 1207: b"\r\0" as *con ... _char:   l41 = UNIQUE | NON_NULL, (empty)
    // 1207: b"\r\0" as *const u8: typeof(_34) = *const {l43} u8
    // 1207: b"\r\0" as *const u8:   l43 = UNIQUE | NON_NULL, (empty)
    // 1207: b"\r\0": typeof(_35) = *const {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1207: b"\r\0":   l45 = UNIQUE | NON_NULL, (empty)
    // 1207: b"\r\0": typeof(_36) = & {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1207: b"\r\0":   l47 = UNIQUE | NON_NULL, FIXED
    // 1207: b"\r\0": typeof(_35 = &raw const (*_36)) = *const {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1207: b"\r\0":   l60 = UNIQUE | NON_NULL, (empty)
    // 1207: b"\r\0" as *const u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l61} u8
    // 1207: b"\r\0" as *const u8:   l61 = UNIQUE | NON_NULL, (empty)
    // 1207: b"\r\0" as *con ... _char: typeof(_33 = move _34 as *const i8 (Misc)) = *const {l62} i8
    // 1207: b"\r\0" as *con ... _char:   l62 = UNIQUE | NON_NULL, (empty)
    // 1207: b"\r\0": typeof(_36 = const b"\r\x00") = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1207: b"\r\0":   l59 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 1208: stdout: typeof(_38) = *mut {l50} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 1208: stdout:   l50 = UNIQUE | NON_NULL, (empty)
    // 1208: stdout: typeof(_39) = *mut {l52} *mut {l53} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 1208: stdout:   l52 = UNIQUE | NON_NULL, (empty)
    // 1208: stdout:   l53 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn prt(mut final_0: libc::c_int) {
    let mut close_0: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    // 1214: mut file: typeof(_5) = *mut {l5} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 1214: mut file:   l5 = UNIQUE | NON_NULL, (empty)
    // 1214: 0 as *mut FILE: typeof(_5 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l255} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 1214: 0 as *mut FILE:   l255 = UNIQUE | NON_NULL, (empty)
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    // 1215: mut cmd: typeof(_6) = *mut {l7} i8
    // 1215: mut cmd:   l7 = UNIQUE | NON_NULL, (empty)
    // 1215: 0 as *mut libc: ... _char: typeof(_6 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l256} i8
    // 1215: 0 as *mut libc: ... _char:   l256 = UNIQUE | NON_NULL, (empty)
    unlink(oname);
    // 1216: oname: typeof(_8) = *const {l10} i8
    // 1216: oname:   l10 = UNIQUE | NON_NULL, (empty)
    // 1216: oname: typeof(_9) = *mut {l12} *const {l13} i8
    // 1216: oname:   l12 = UNIQUE | NON_NULL, (empty)
    // 1216: oname:   l13 = UNIQUE | NON_NULL, (empty)
    len = strlen(oname) as libc::c_int;
    // 1217: oname: typeof(_11) = *const {l16} i8
    // 1217: oname:   l16 = UNIQUE | NON_NULL, (empty)
    // 1217: oname: typeof(_12) = *mut {l18} *const {l19} i8
    // 1217: oname:   l18 = UNIQUE | NON_NULL, (empty)
    // 1217: oname:   l19 = UNIQUE | NON_NULL, (empty)
    if len >= 3 as libc::c_int
        && strcmp(
            oname
            // 1220: oname .offset(l ... ize)): typeof(_20) = *const {l28} i8
            // 1220: oname .offset(l ... ize)):   l28 = UNIQUE | NON_NULL, (empty)
            // 1220: oname .offset(l ... size): typeof(_21) = *const {l30} i8
            // 1220: oname .offset(l ... size):   l30 = UNIQUE | NON_NULL, (empty)
            // 1220: oname: typeof(_22) = *const {l32} i8
            // 1220: oname:   l32 = UNIQUE | NON_NULL, (empty)
            // 1220: oname: typeof(_23) = *mut {l34} *const {l35} i8
            // 1220: oname:   l34 = UNIQUE | NON_NULL, (empty)
            // 1220: oname:   l35 = UNIQUE | NON_NULL, (empty)
                .offset(len as isize)
                .offset(-(3 as libc::c_int as isize)),
            b".gz\0" as *const u8 as *const libc::c_char,
            // 1223: b".gz\0" as *co ... _char: typeof(_30) = *const {l43} i8
            // 1223: b".gz\0" as *co ... _char:   l43 = UNIQUE | NON_NULL, (empty)
            // 1223: b".gz\0" as *co ... st u8: typeof(_31) = *const {l45} u8
            // 1223: b".gz\0" as *co ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
            // 1223: b".gz\0": typeof(_32) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1223: b".gz\0":   l47 = UNIQUE | NON_NULL, (empty)
            // 1223: b".gz\0": typeof(_33) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1223: b".gz\0":   l49 = UNIQUE | NON_NULL, FIXED
            // 1223: b".gz\0" as *co ... st u8: typeof(_31 = move _32 as *const u8 (Pointer(ArrayToPointer))) = *const {l259} u8
            // 1223: b".gz\0" as *co ... st u8:   l259 = UNIQUE | NON_NULL, (empty)
            // 1223: b".gz\0" as *co ... _char: typeof(_30 = move _31 as *const i8 (Misc)) = *const {l260} i8
            // 1223: b".gz\0" as *co ... _char:   l260 = UNIQUE | NON_NULL, (empty)
            // 1223: b".gz\0": typeof(_33 = const b".gz\x00") = & {l257} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1223: b".gz\0":   l257 = UNIQUE | NON_NULL, (empty)
            // 1223: b".gz\0": typeof(_32 = &raw const (*_33)) = *const {l258} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 1223: b".gz\0":   l258 = UNIQUE | NON_NULL, (empty)
        ) == 0
    {
        cmd = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        // 1226: malloc((len + 2 ... long): typeof(_34) = *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1226: malloc((len + 2 ... long):   l51 = UNIQUE | NON_NULL, (empty)
        // 1226: cmd = malloc((l ... _char: typeof(_6 = move _34 as *mut i8 (Misc)) = *mut {l261} i8
        // 1226: cmd = malloc((l ... _char:   l261 = UNIQUE | NON_NULL, (empty)
        sprintf(
            cmd,
            // 1228: cmd: typeof(_41) = *mut {l59} i8
            // 1228: cmd:   l59 = UNIQUE | NON_NULL, (empty)
            b"gzip -c > %s\0" as *const u8 as *const libc::c_char,
            // 1229: b"gzip -c > %s\ ... _char: typeof(_42) = *const {l61} i8
            // 1229: b"gzip -c > %s\ ... _char:   l61 = UNIQUE | NON_NULL, (empty)
            // 1229: b"gzip -c > %s\ ... st u8: typeof(_43) = *const {l63} u8
            // 1229: b"gzip -c > %s\ ... st u8:   l63 = UNIQUE | NON_NULL, (empty)
            // 1229: b"gzip -c > %s\0": typeof(_44) = *const {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1229: b"gzip -c > %s\0":   l65 = UNIQUE | NON_NULL, (empty)
            // 1229: b"gzip -c > %s\0": typeof(_45) = & {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1229: b"gzip -c > %s\0":   l67 = UNIQUE | NON_NULL, FIXED
            // 1229: b"gzip -c > %s\ ... st u8: typeof(_43 = move _44 as *const u8 (Pointer(ArrayToPointer))) = *const {l264} u8
            // 1229: b"gzip -c > %s\ ... st u8:   l264 = UNIQUE | NON_NULL, (empty)
            // 1229: b"gzip -c > %s\ ... _char: typeof(_42 = move _43 as *const i8 (Misc)) = *const {l265} i8
            // 1229: b"gzip -c > %s\ ... _char:   l265 = UNIQUE | NON_NULL, (empty)
            // 1229: b"gzip -c > %s\0": typeof(_45 = const b"gzip -c > %s\x00") = & {l262} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1229: b"gzip -c > %s\0":   l262 = UNIQUE | NON_NULL, (empty)
            // 1229: b"gzip -c > %s\0": typeof(_44 = &raw const (*_45)) = *const {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 1229: b"gzip -c > %s\0":   l263 = UNIQUE | NON_NULL, (empty)
            oname,
            // 1230: oname: typeof(_46) = *const {l69} i8
            // 1230: oname:   l69 = UNIQUE | NON_NULL, (empty)
            // 1230: oname: typeof(_47) = *mut {l71} *const {l72} i8
            // 1230: oname:   l71 = UNIQUE | NON_NULL, (empty)
            // 1230: oname:   l72 = UNIQUE | NON_NULL, (empty)
        );
        file = popen(cmd, b"w\0" as *const u8 as *const libc::c_char);
        // 1232: popen(cmd, b"w\ ... char): typeof(_48) = *mut {l74} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1232: popen(cmd, b"w\ ... char):   l74 = UNIQUE | NON_NULL, (empty)
        // 1232: cmd: typeof(_49) = *const {l76} i8
        // 1232: cmd:   l76 = UNIQUE | NON_NULL, (empty)
        // 1232: cmd: typeof(_50) = *mut {l78} i8
        // 1232: cmd:   l78 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0" as *cons ... _char: typeof(_51) = *const {l80} i8
        // 1232: b"w\0" as *cons ... _char:   l80 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0" as *const u8: typeof(_52) = *const {l82} u8
        // 1232: b"w\0" as *const u8:   l82 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0": typeof(_53) = *const {l84} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1232: b"w\0":   l84 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0": typeof(_54) = & {l86} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1232: b"w\0":   l86 = UNIQUE | NON_NULL, FIXED
        // 1232: cmd: typeof(_49 = move _50 as *const i8 (Pointer(MutToConstPointer))) = *const {l266} i8
        // 1232: cmd:   l266 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0": typeof(_54 = const b"w\x00") = & {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1232: b"w\0":   l267 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0": typeof(_53 = &raw const (*_54)) = *const {l268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1232: b"w\0":   l268 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0" as *const u8: typeof(_52 = move _53 as *const u8 (Pointer(ArrayToPointer))) = *const {l269} u8
        // 1232: b"w\0" as *const u8:   l269 = UNIQUE | NON_NULL, (empty)
        // 1232: b"w\0" as *cons ... _char: typeof(_51 = move _52 as *const i8 (Misc)) = *const {l270} i8
        // 1232: b"w\0" as *cons ... _char:   l270 = UNIQUE | NON_NULL, (empty)
        if !file.is_null() {
        // 1233: file: typeof(_58) = *mut {l91} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1233: file:   l91 = UNIQUE | NON_NULL, (empty)
            close_0 = 2 as libc::c_int;
        }
        free(cmd as *mut libc::c_void);
        // 1236: cmd as *mut lib ... _void: typeof(_61) = *mut {l95} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1236: cmd as *mut lib ... _void:   l95 = UNIQUE | NON_NULL, (empty)
        // 1236: cmd: typeof(_62) = *mut {l97} i8
        // 1236: cmd:   l97 = UNIQUE | NON_NULL, (empty)
        // 1236: cmd as *mut lib ... _void: typeof(_61 = move _62 as *mut libc::c_void (Misc)) = *mut {l271} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1236: cmd as *mut lib ... _void:   l271 = UNIQUE | NON_NULL, (empty)
    } else {
        file = fopen(oname, b"w\0" as *const u8 as *const libc::c_char);
        // 1238: fopen(oname, b" ... char): typeof(_63) = *mut {l99} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1238: fopen(oname, b" ... char):   l99 = UNIQUE | NON_NULL, (empty)
        // 1238: oname: typeof(_64) = *const {l101} i8
        // 1238: oname:   l101 = UNIQUE | NON_NULL, (empty)
        // 1238: oname: typeof(_65) = *mut {l103} *const {l104} i8
        // 1238: oname:   l103 = UNIQUE | NON_NULL, (empty)
        // 1238: oname:   l104 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0" as *cons ... _char: typeof(_66) = *const {l106} i8
        // 1238: b"w\0" as *cons ... _char:   l106 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0" as *const u8: typeof(_67) = *const {l108} u8
        // 1238: b"w\0" as *const u8:   l108 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0": typeof(_68) = *const {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1238: b"w\0":   l110 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0": typeof(_69) = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1238: b"w\0":   l112 = UNIQUE | NON_NULL, FIXED
        // 1238: b"w\0": typeof(_69 = const b"w\x00") = & {l272} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1238: b"w\0":   l272 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0" as *cons ... _char: typeof(_66 = move _67 as *const i8 (Misc)) = *const {l275} i8
        // 1238: b"w\0" as *cons ... _char:   l275 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0" as *const u8: typeof(_67 = move _68 as *const u8 (Pointer(ArrayToPointer))) = *const {l274} u8
        // 1238: b"w\0" as *const u8:   l274 = UNIQUE | NON_NULL, (empty)
        // 1238: b"w\0": typeof(_68 = &raw const (*_69)) = *const {l273} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1238: b"w\0":   l273 = UNIQUE | NON_NULL, (empty)
        if !file.is_null() {
        // 1239: file: typeof(_72) = *mut {l116} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1239: file:   l116 = UNIQUE | NON_NULL, (empty)
            close_0 = 1 as libc::c_int;
        }
    }
    if file.is_null() {
    // 1243: file: typeof(_76) = *mut {l121} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 1243: file:   l121 = UNIQUE | NON_NULL, (empty)
        die(
            b"can not write to '%s'\0" as *const u8 as *const libc::c_char,
            // 1245: b"can not write ... _char: typeof(_78) = *const {l124} i8
            // 1245: b"can not write ... _char:   l124 = UNIQUE | NON_NULL, (empty)
            // 1245: b"can not write ... st u8: typeof(_79) = *const {l126} u8
            // 1245: b"can not write ... st u8:   l126 = UNIQUE | NON_NULL, (empty)
            // 1245: b"can not write ... s'\0": typeof(_80) = *const {l128} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1245: b"can not write ... s'\0":   l128 = UNIQUE | NON_NULL, (empty)
            // 1245: b"can not write ... s'\0": typeof(_81) = & {l130} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1245: b"can not write ... s'\0":   l130 = UNIQUE | NON_NULL, FIXED
            // 1245: b"can not write ... _char: typeof(_78 = move _79 as *const i8 (Misc)) = *const {l279} i8
            // 1245: b"can not write ... _char:   l279 = UNIQUE | NON_NULL, (empty)
            // 1245: b"can not write ... s'\0": typeof(_81 = const b"can not write to \'%s\'\x00") = & {l276} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1245: b"can not write ... s'\0":   l276 = UNIQUE | NON_NULL, (empty)
            // 1245: b"can not write ... s'\0": typeof(_80 = &raw const (*_81)) = *const {l277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1245: b"can not write ... s'\0":   l277 = UNIQUE | NON_NULL, (empty)
            // 1245: b"can not write ... st u8: typeof(_79 = move _80 as *const u8 (Pointer(ArrayToPointer))) = *const {l278} u8
            // 1245: b"can not write ... st u8:   l278 = UNIQUE | NON_NULL, (empty)
            oname,
            // 1246: oname: typeof(_82) = *const {l132} i8
            // 1246: oname:   l132 = UNIQUE | NON_NULL, (empty)
            // 1246: oname: typeof(_83) = *mut {l134} *const {l135} i8
            // 1246: oname:   l134 = UNIQUE | NON_NULL, (empty)
            // 1246: oname:   l135 = UNIQUE | NON_NULL, (empty)
        );
    }
    prevents = 0 as libc::c_int;
    // 1249: prevents: typeof(_85) = *mut {l138} i32
    // 1249: prevents:   l138 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < nevents {
    // 1251: nevents: typeof(_92) = *mut {l146} i32
    // 1251: nevents:   l146 = UNIQUE | NON_NULL, (empty)
        if !(reme_shim(events.offset(i as isize)) != 0) {
        // 1252: events.offset(i ... size): typeof(_97) = *mut {l152} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1252: events.offset(i ... size):   l152 = UNIQUE | NON_NULL, (empty)
        // 1252: events: typeof(_98) = *mut {l154} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1252: events:   l154 = UNIQUE | NON_NULL, (empty)
        // 1252: events: typeof(_99) = *mut {l156} *mut {l157} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1252: events:   l156 = UNIQUE | NON_NULL, (empty)
        // 1252: events:   l157 = UNIQUE | NON_NULL, (empty)
            print(events.offset(i as isize), file);
            // 1253: events.offset(i ... size): typeof(_103) = *mut {l162} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1253: events.offset(i ... size):   l162 = UNIQUE | NON_NULL, (empty)
            // 1253: events: typeof(_104) = *mut {l164} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1253: events:   l164 = UNIQUE | NON_NULL, (empty)
            // 1253: events: typeof(_105) = *mut {l166} *mut {l167} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1253: events:   l166 = UNIQUE | NON_NULL, (empty)
            // 1253: events:   l167 = UNIQUE | NON_NULL, (empty)
            // 1253: file: typeof(_108) = *mut {l171} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 1253: file:   l171 = UNIQUE | NON_NULL, (empty)
            prevents += 1;
            // 1254: prevents: typeof(_109) = *mut {l173} i32
            // 1254: prevents:   l173 = UNIQUE | NON_NULL, (empty)
            prevents;
            // 1255: prevents: typeof(_112) = *mut {l177} i32
            // 1255: prevents:   l177 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    if close_0 == 2 as libc::c_int {
        pclose(file);
        // 1261: file: typeof(_123) = *mut {l189} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1261: file:   l189 = UNIQUE | NON_NULL, (empty)
    }
    if close_0 == 1 as libc::c_int {
        fclose(file);
        // 1264: file: typeof(_129) = *mut {l196} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1264: file:   l196 = UNIQUE | NON_NULL, (empty)
    }
    if verbose != 0 && isatty(1 as libc::c_int) != 0 {
    // 1266: verbose: typeof(_134) = *mut {l202} i32
    // 1266: verbose:   l202 = UNIQUE | NON_NULL, (empty)
        fputc('\r' as i32, stdout);
        // 1267: stdout: typeof(_140) = *mut {l209} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1267: stdout:   l209 = UNIQUE | NON_NULL, (empty)
        // 1267: stdout: typeof(_141) = *mut {l211} *mut {l212} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1267: stdout:   l211 = UNIQUE | NON_NULL, (empty)
        // 1267: stdout:   l212 = UNIQUE | NON_NULL, (empty)
        i = 0 as libc::c_int;
        while i < 78 as libc::c_int {
            fputc(' ' as i32, stdout);
            // 1270: stdout: typeof(_149) = *mut {l221} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 1270: stdout:   l221 = UNIQUE | NON_NULL, (empty)
            // 1270: stdout: typeof(_150) = *mut {l223} *mut {l224} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
            // 1270: stdout:   l223 = UNIQUE | NON_NULL, (empty)
            // 1270: stdout:   l224 = UNIQUE | NON_NULL, (empty)
            i += 1;
            i;
        }
        fputc('\r' as i32, stdout);
        // 1274: stdout: typeof(_158) = *mut {l233} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1274: stdout:   l233 = UNIQUE | NON_NULL, (empty)
        // 1274: stdout: typeof(_159) = *mut {l235} *mut {l236} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 1274: stdout:   l235 = UNIQUE | NON_NULL, (empty)
        // 1274: stdout:   l236 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        b"written %s with %d events\0" as *const u8 as *const libc::c_char,
        // 1277: b"written %s wi ... _char: typeof(_161) = *const {l239} i8
        // 1277: b"written %s wi ... _char:   l239 = UNIQUE | NON_NULL, (empty)
        // 1277: b"written %s wi ... st u8: typeof(_162) = *const {l241} u8
        // 1277: b"written %s wi ... st u8:   l241 = UNIQUE | NON_NULL, (empty)
        // 1277: b"written %s wi ... ts\0": typeof(_163) = *const {l243} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
        // 1277: b"written %s wi ... ts\0":   l243 = UNIQUE | NON_NULL, (empty)
        // 1277: b"written %s wi ... ts\0": typeof(_164) = & {l245} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
        // 1277: b"written %s wi ... ts\0":   l245 = UNIQUE | NON_NULL, FIXED
        // 1277: b"written %s wi ... _char: typeof(_161 = move _162 as *const i8 (Misc)) = *const {l283} i8
        // 1277: b"written %s wi ... _char:   l283 = UNIQUE | NON_NULL, (empty)
        // 1277: b"written %s wi ... ts\0": typeof(_163 = &raw const (*_164)) = *const {l281} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
        // 1277: b"written %s wi ... ts\0":   l281 = UNIQUE | NON_NULL, (empty)
        // 1277: b"written %s wi ... ts\0": typeof(_164 = const b"written %s with %d events\x00") = & {l280} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
        // 1277: b"written %s wi ... ts\0":   l280 = UNIQUE | NON_NULL, (empty)
        // 1277: b"written %s wi ... st u8: typeof(_162 = move _163 as *const u8 (Pointer(ArrayToPointer))) = *const {l282} u8
        // 1277: b"written %s wi ... st u8:   l282 = UNIQUE | NON_NULL, (empty)
        oname,
        // 1278: oname: typeof(_165) = *const {l247} i8
        // 1278: oname:   l247 = UNIQUE | NON_NULL, (empty)
        // 1278: oname: typeof(_166) = *mut {l249} *const {l250} i8
        // 1278: oname:   l249 = UNIQUE | NON_NULL, (empty)
        // 1278: oname:   l250 = UNIQUE | NON_NULL, (empty)
        prevents,
        // 1279: prevents: typeof(_168) = *mut {l253} i32
        // 1279: prevents:   l253 = UNIQUE | NON_NULL, (empty)
    );
}
unsafe extern "C" fn dd() {
    let mut from: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut rgran: libc::c_int = 0;
    let mut nranges: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut changed: libc::c_int = 0;
    let mut ranges: *mut Range = calloc(
    // 1294: mut ranges: typeof(_12) = *mut {l12} DefId(0:370 ~ lglddtrace[7e63]::Range)
    // 1294: mut ranges:   l12 = UNIQUE | NON_NULL, (empty)
    // 1294: calloc( nevents ... ng, ): typeof(_13) = *mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1294: calloc( nevents ... ng, ):   l14 = UNIQUE | NON_NULL, (empty)
    // 1294: calloc( nevents ... Range: typeof(_12 = move _13 as *mut Range (Misc)) = *mut {l2311} DefId(0:370 ~ lglddtrace[7e63]::Range)
    // 1294: calloc( nevents ... Range:   l2311 = UNIQUE | NON_NULL, (empty)
        nevents as libc::c_ulong,
        // 1295: nevents: typeof(_16) = *mut {l18} i32
        // 1295: nevents:   l18 = UNIQUE | NON_NULL, (empty)
        ::core::mem::size_of::<Range>() as libc::c_ulong,
    ) as *mut Range;
    let mut r: *mut Range = 0 as *mut Range;
    // 1298: mut r: typeof(_19) = *mut {l22} DefId(0:370 ~ lglddtrace[7e63]::Range)
    // 1298: mut r:   l22 = UNIQUE | NON_NULL, (empty)
    // 1298: 0 as *mut Range: typeof(_19 = const 0_usize as *mut Range (PointerFromExposedAddress)) = *mut {l2312} DefId(0:370 ~ lglddtrace[7e63]::Range)
    // 1298: 0 as *mut Range:   l2312 = UNIQUE | NON_NULL, (empty)
    let mut smap: *mut libc::c_int = 0 as *mut libc::c_int;
    // 1299: mut smap: typeof(_20) = *mut {l24} i32
    // 1299: mut smap:   l24 = UNIQUE | NON_NULL, (empty)
    // 1299: 0 as *mut libc: ... c_int: typeof(_20 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l2313} i32
    // 1299: 0 as *mut libc: ... c_int:   l2313 = UNIQUE | NON_NULL, (empty)
    let mut idx: libc::c_int = 0;
    let mut moved: libc::c_int = 0;
    let mut mapto: libc::c_int = 0;
    let mut nused: libc::c_int = 0;
    let mut cluster: Type = ADD;
    let mut used: *mut libc::c_char = 0 as *mut libc::c_char;
    // 1305: mut used: typeof(_26) = *mut {l31} i8
    // 1305: mut used:   l31 = UNIQUE | NON_NULL, (empty)
    // 1305: 0 as *mut libc: ... _char: typeof(_26 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l2314} i8
    // 1305: 0 as *mut libc: ... _char:   l2314 = UNIQUE | NON_NULL, (empty)
    let mut e: *mut Event = 0 as *mut Event;
    // 1306: mut e: typeof(_27) = *mut {l33} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 1306: mut e:   l33 = UNIQUE | NON_NULL, (empty)
    // 1306: 0 as *mut Event: typeof(_27 = const 0_usize as *mut Event (PointerFromExposedAddress)) = *mut {l2315} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 1306: 0 as *mut Event:   l2315 = UNIQUE | NON_NULL, (empty)
    loop {
        prt(0 as libc::c_int);
        changed = 0 as libc::c_int;
        nused = 0 as libc::c_int;
        used = malloc((nmap + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        // 1311: malloc((nmap +  ... long): typeof(_34) = *mut {l41} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1311: malloc((nmap +  ... long):   l41 = UNIQUE | NON_NULL, (empty)
        // 1311: nmap: typeof(_38) = *mut {l46} i32
        // 1311: nmap:   l46 = UNIQUE | NON_NULL, (empty)
        // 1311: used = malloc(( ... _char: typeof(_26 = move _34 as *mut i8 (Misc)) = *mut {l2316} i8
        // 1311: used = malloc(( ... _char:   l2316 = UNIQUE | NON_NULL, (empty)
        memset(
        // 1312: memset( used as ... ng, ): typeof(_41) = *mut {l50} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1312: memset( used as ... ng, ):   l50 = UNIQUE | NON_NULL, (empty)
            used as *mut libc::c_void,
            // 1313: used as *mut li ... _void: typeof(_42) = *mut {l52} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1313: used as *mut li ... _void:   l52 = UNIQUE | NON_NULL, (empty)
            // 1313: used: typeof(_43) = *mut {l54} i8
            // 1313: used:   l54 = UNIQUE | NON_NULL, (empty)
            // 1313: used as *mut li ... _void: typeof(_42 = move _43 as *mut libc::c_void (Misc)) = *mut {l2317} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1313: used as *mut li ... _void:   l2317 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int,
            (nmap + 1 as libc::c_int) as libc::c_ulong,
            // 1315: nmap: typeof(_48) = *mut {l60} i32
            // 1315: nmap:   l60 = UNIQUE | NON_NULL, (empty)
        );
        e = events;
        // 1317: events: typeof(_51) = *mut {l64} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1317: events:   l64 = UNIQUE | NON_NULL, (empty)
        // 1317: events: typeof(_52) = *mut {l66} *mut {l67} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1317: events:   l66 = UNIQUE | NON_NULL, (empty)
        // 1317: events:   l67 = UNIQUE | NON_NULL, (empty)
        while e < events.offset(nevents as isize) {
        // 1318: e: typeof(_55) = *mut {l71} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1318: e:   l71 = UNIQUE | NON_NULL, (empty)
        // 1318: events.offset(n ... size): typeof(_56) = *mut {l73} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1318: events.offset(n ... size):   l73 = UNIQUE | NON_NULL, (empty)
        // 1318: events: typeof(_57) = *mut {l75} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1318: events:   l75 = UNIQUE | NON_NULL, (empty)
        // 1318: events: typeof(_58) = *mut {l77} *mut {l78} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 1318: events:   l77 = UNIQUE | NON_NULL, (empty)
        // 1318: events:   l78 = UNIQUE | NON_NULL, (empty)
        // 1318: nevents: typeof(_61) = *mut {l82} i32
        // 1318: nevents:   l82 = UNIQUE | NON_NULL, (empty)
            if !(reme_shim(e) != 0) {
            // 1319: e: typeof(_66) = *mut {l88} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1319: e:   l88 = UNIQUE | NON_NULL, (empty)
                idx = 0 as libc::c_int;
                match (*e).type_0 as libc::c_uint {
                    0 | 1 | 2 | 3 | 26 | 21 | 23 | 22 | 14 | 15 | 16 | 17 | 4 | 6 | 7 | 9 => {
                        idx = (*e).arg;
                    }
                    5 | 8 | 10 | 11 | 12 | 13 | 29 | 24 | 25 | 30 | 31 | 27 | 19 | 20 | 28 | 18
                    | _ => {}
                }
                if !(idx == 0) {
                    idx = abs(idx);
                    if !(*used.offset(idx as isize) != 0) {
                    // 1330: used.offset(idx ... size): typeof(_79) = *mut {l102} i8
                    // 1330: used.offset(idx ... size):   l102 = UNIQUE | NON_NULL, (empty)
                    // 1330: used: typeof(_80) = *mut {l104} i8
                    // 1330: used:   l104 = UNIQUE | NON_NULL, (empty)
                        *used.offset(idx as isize) = 1 as libc::c_int as libc::c_char;
                        // 1331: used.offset(idx ... size): typeof(_84) = *mut {l109} i8
                        // 1331: used.offset(idx ... size):   l109 = UNIQUE | NON_NULL, (empty)
                        // 1331: used: typeof(_85) = *mut {l111} i8
                        // 1331: used:   l111 = UNIQUE | NON_NULL, (empty)
                        nused += 1;
                        nused;
                    }
                }
            }
            e = e.offset(1);
            // 1337: e.offset(1): typeof(_90) = *mut {l117} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1337: e.offset(1):   l117 = UNIQUE | NON_NULL, (empty)
            // 1337: e: typeof(_91) = *mut {l119} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1337: e:   l119 = UNIQUE | NON_NULL, (empty)
            e;
            // 1338: e: typeof(_92) = *mut {l121} DefId(0:354 ~ lglddtrace[7e63]::Event)
            // 1338: e:   l121 = UNIQUE | NON_NULL, (empty)
        }
        if nused < nmap {
        // 1340: nmap: typeof(_100) = *mut {l130} i32
        // 1340: nmap:   l130 = UNIQUE | NON_NULL, (empty)
            let mut current_block_37: u64;
            smap = map;
            // 1342: map: typeof(_102) = *mut {l133} i32
            // 1342: map:   l133 = UNIQUE | NON_NULL, (empty)
            // 1342: map: typeof(_103) = *mut {l135} *mut {l136} i32
            // 1342: map:   l135 = UNIQUE | NON_NULL, (empty)
            // 1342: map:   l136 = UNIQUE | NON_NULL, (empty)
            moved = 0 as libc::c_int;
            mapto = 1 as libc::c_int;
            map = malloc(
            // 1345: malloc( (szmap  ... g), ): typeof(_106) = *mut {l140} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1345: malloc( (szmap  ... g), ):   l140 = UNIQUE | NON_NULL, (empty)
            // 1345: map: typeof(_113) = *mut {l149} *mut {l150} i32
            // 1345: map:   l149 = UNIQUE | NON_NULL, (empty)
            // 1345: map:   l150 = UNIQUE | NON_NULL, (empty)
            // 1345: map = malloc( ( ... c_int: typeof((*_113) = move _106 as *mut i32 (Misc)) = *mut {l2318} i32
            // 1345: map = malloc( ( ... c_int:   l2318 = UNIQUE | NON_NULL, (empty)
                (szmap as libc::c_ulong)
                // 1346: szmap: typeof(_110) = *mut {l145} i32
                // 1346: szmap:   l145 = UNIQUE | NON_NULL, (empty)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            idx = 1 as libc::c_int;
            while idx <= nmap {
            // 1350: nmap: typeof(_119) = *mut {l157} i32
            // 1350: nmap:   l157 = UNIQUE | NON_NULL, (empty)
                if *used.offset(idx as isize) != 0 {
                // 1351: used.offset(idx ... size): typeof(_123) = *mut {l162} i8
                // 1351: used.offset(idx ... size):   l162 = UNIQUE | NON_NULL, (empty)
                // 1351: used: typeof(_124) = *mut {l164} i8
                // 1351: used:   l164 = UNIQUE | NON_NULL, (empty)
                    if *smap.offset(idx as isize) != mapto {
                    // 1352: smap.offset(idx ... size): typeof(_130) = *mut {l171} i32
                    // 1352: smap.offset(idx ... size):   l171 = UNIQUE | NON_NULL, (empty)
                    // 1352: smap: typeof(_131) = *mut {l173} i32
                    // 1352: smap:   l173 = UNIQUE | NON_NULL, (empty)
                        if mapto < *smap.offset(idx as isize) {
                        // 1353: smap.offset(idx ... size): typeof(_139) = *mut {l182} i32
                        // 1353: smap.offset(idx ... size):   l182 = UNIQUE | NON_NULL, (empty)
                        // 1353: smap: typeof(_140) = *mut {l184} i32
                        // 1353: smap:   l184 = UNIQUE | NON_NULL, (empty)
                        } else {
                            __assert_fail(
                                b"mapto < smap[idx]\0" as *const u8 as *const libc::c_char,
                                // 1356: b"mapto < smap[ ... _char: typeof(_145) = *const {l190} i8
                                // 1356: b"mapto < smap[ ... _char:   l190 = UNIQUE | NON_NULL, (empty)
                                // 1356: b"mapto < smap[ ... st u8: typeof(_146) = *const {l192} u8
                                // 1356: b"mapto < smap[ ... st u8:   l192 = UNIQUE | NON_NULL, (empty)
                                // 1356: b"mapto < smap[ ... x]\0": typeof(_147) = *const {l194} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1356: b"mapto < smap[ ... x]\0":   l194 = UNIQUE | NON_NULL, (empty)
                                // 1356: b"mapto < smap[ ... x]\0": typeof(_148) = & {l196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1356: b"mapto < smap[ ... x]\0":   l196 = UNIQUE | NON_NULL, FIXED
                                // 1356: b"mapto < smap[ ... _char: typeof(_145 = move _146 as *const i8 (Misc)) = *const {l2322} i8
                                // 1356: b"mapto < smap[ ... _char:   l2322 = UNIQUE | NON_NULL, (empty)
                                // 1356: b"mapto < smap[ ... st u8: typeof(_146 = move _147 as *const u8 (Pointer(ArrayToPointer))) = *const {l2321} u8
                                // 1356: b"mapto < smap[ ... st u8:   l2321 = UNIQUE | NON_NULL, (empty)
                                // 1356: b"mapto < smap[ ... x]\0": typeof(_147 = &raw const (*_148)) = *const {l2320} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1356: b"mapto < smap[ ... x]\0":   l2320 = UNIQUE | NON_NULL, (empty)
                                // 1356: b"mapto < smap[ ... x]\0": typeof(_148 = const b"mapto < smap[idx]\x00") = & {l2319} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1356: b"mapto < smap[ ... x]\0":   l2319 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 1357: b"lglddtrace.c\ ... _char: typeof(_149) = *const {l198} i8
                                // 1357: b"lglddtrace.c\ ... _char:   l198 = UNIQUE | NON_NULL, (empty)
                                // 1357: b"lglddtrace.c\ ... st u8: typeof(_150) = *const {l200} u8
                                // 1357: b"lglddtrace.c\ ... st u8:   l200 = UNIQUE | NON_NULL, (empty)
                                // 1357: b"lglddtrace.c\0": typeof(_151) = *const {l202} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1357: b"lglddtrace.c\0":   l202 = UNIQUE | NON_NULL, (empty)
                                // 1357: b"lglddtrace.c\0": typeof(_152) = & {l204} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1357: b"lglddtrace.c\0":   l204 = UNIQUE | NON_NULL, FIXED
                                // 1357: b"lglddtrace.c\0": typeof(_152 = const b"lglddtrace.c\x00") = & {l2323} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1357: b"lglddtrace.c\0":   l2323 = UNIQUE | NON_NULL, (empty)
                                // 1357: b"lglddtrace.c\ ... _char: typeof(_149 = move _150 as *const i8 (Misc)) = *const {l2326} i8
                                // 1357: b"lglddtrace.c\ ... _char:   l2326 = UNIQUE | NON_NULL, (empty)
                                // 1357: b"lglddtrace.c\0": typeof(_151 = &raw const (*_152)) = *const {l2324} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1357: b"lglddtrace.c\0":   l2324 = UNIQUE | NON_NULL, (empty)
                                // 1357: b"lglddtrace.c\ ... st u8: typeof(_150 = move _151 as *const u8 (Pointer(ArrayToPointer))) = *const {l2325} u8
                                // 1357: b"lglddtrace.c\ ... st u8:   l2325 = UNIQUE | NON_NULL, (empty)
                                479 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                // 1359: (*::core::mem:: ... ptr(): typeof(_155) = *const {l208} i8
                                // 1359: (*::core::mem:: ... ptr():   l208 = UNIQUE | NON_NULL, (empty)
                                // 1359: (*::core::mem:: ... ptr(): typeof(_156) = & {l210} [i8]
                                // 1359: (*::core::mem:: ... ptr():   l210 = UNIQUE | NON_NULL, FIXED
                                // 1359: (*::core::mem:: ... ptr(): typeof(_157) = & {l212} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1359: (*::core::mem:: ... ptr():   l212 = UNIQUE | NON_NULL, (empty)
                                // 1359: ::core::mem::tr ... 0", ): typeof(_158) = & {l214} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1359: ::core::mem::tr ... 0", ):   l214 = UNIQUE | NON_NULL, FIXED
                                // 1359: (*::core::mem:: ... ptr(): typeof(_157 = &(*_158)) = & {l2329} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1359: (*::core::mem:: ... ptr():   l2329 = UNIQUE | NON_NULL, (empty)
                                // 1359: (*::core::mem:: ... ptr(): typeof(_156 = move _157 as &[i8] (Pointer(Unsize))) = & {l2330} [i8]
                                // 1359: (*::core::mem:: ... ptr():   l2330 = UNIQUE | NON_NULL, FIXED
                                    b"void dd(void)\0",
                                    // 1360: b"void dd(void)\0": typeof(_159) = & {l216} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1360: b"void dd(void)\0":   l216 = UNIQUE | NON_NULL, (empty)
                                    // 1360: b"void dd(void)\0": typeof(_160) = & {l218} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1360: b"void dd(void)\0":   l218 = UNIQUE | NON_NULL, FIXED
                                    // 1360: b"void dd(void)\0": typeof(_160 = const b"void dd(void)\x00") = & {l2327} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1360: b"void dd(void)\0":   l2327 = UNIQUE | NON_NULL, (empty)
                                    // 1360: b"void dd(void)\0": typeof(_159 = &(*_160)) = & {l2328} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1360: b"void dd(void)\0":   l2328 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                        'c_9636: {
                            if mapto < *smap.offset(idx as isize) {
                            // 1366: smap.offset(idx ... size): typeof(_165) = *mut {l224} i32
                            // 1366: smap.offset(idx ... size):   l224 = UNIQUE | NON_NULL, (empty)
                            // 1366: smap: typeof(_166) = *mut {l226} i32
                            // 1366: smap:   l226 = UNIQUE | NON_NULL, (empty)
                            } else {
                                __assert_fail(
                                    b"mapto < smap[idx]\0" as *const u8 as *const libc::c_char,
                                    // 1369: b"mapto < smap[ ... _char: typeof(_171) = *const {l232} i8
                                    // 1369: b"mapto < smap[ ... _char:   l232 = UNIQUE | NON_NULL, (empty)
                                    // 1369: b"mapto < smap[ ... st u8: typeof(_172) = *const {l234} u8
                                    // 1369: b"mapto < smap[ ... st u8:   l234 = UNIQUE | NON_NULL, (empty)
                                    // 1369: b"mapto < smap[ ... x]\0": typeof(_173) = *const {l236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                    // 1369: b"mapto < smap[ ... x]\0":   l236 = UNIQUE | NON_NULL, (empty)
                                    // 1369: b"mapto < smap[ ... x]\0": typeof(_174) = & {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                    // 1369: b"mapto < smap[ ... x]\0":   l238 = UNIQUE | NON_NULL, FIXED
                                    // 1369: b"mapto < smap[ ... x]\0": typeof(_174 = const b"mapto < smap[idx]\x00") = & {l2331} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                    // 1369: b"mapto < smap[ ... x]\0":   l2331 = UNIQUE | NON_NULL, (empty)
                                    // 1369: b"mapto < smap[ ... x]\0": typeof(_173 = &raw const (*_174)) = *const {l2332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                    // 1369: b"mapto < smap[ ... x]\0":   l2332 = UNIQUE | NON_NULL, (empty)
                                    // 1369: b"mapto < smap[ ... _char: typeof(_171 = move _172 as *const i8 (Misc)) = *const {l2334} i8
                                    // 1369: b"mapto < smap[ ... _char:   l2334 = UNIQUE | NON_NULL, (empty)
                                    // 1369: b"mapto < smap[ ... st u8: typeof(_172 = move _173 as *const u8 (Pointer(ArrayToPointer))) = *const {l2333} u8
                                    // 1369: b"mapto < smap[ ... st u8:   l2333 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 1370: b"lglddtrace.c\ ... _char: typeof(_175) = *const {l240} i8
                                    // 1370: b"lglddtrace.c\ ... _char:   l240 = UNIQUE | NON_NULL, (empty)
                                    // 1370: b"lglddtrace.c\ ... st u8: typeof(_176) = *const {l242} u8
                                    // 1370: b"lglddtrace.c\ ... st u8:   l242 = UNIQUE | NON_NULL, (empty)
                                    // 1370: b"lglddtrace.c\0": typeof(_177) = *const {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1370: b"lglddtrace.c\0":   l244 = UNIQUE | NON_NULL, (empty)
                                    // 1370: b"lglddtrace.c\0": typeof(_178) = & {l246} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1370: b"lglddtrace.c\0":   l246 = UNIQUE | NON_NULL, FIXED
                                    // 1370: b"lglddtrace.c\0": typeof(_178 = const b"lglddtrace.c\x00") = & {l2335} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1370: b"lglddtrace.c\0":   l2335 = UNIQUE | NON_NULL, (empty)
                                    // 1370: b"lglddtrace.c\ ... _char: typeof(_175 = move _176 as *const i8 (Misc)) = *const {l2338} i8
                                    // 1370: b"lglddtrace.c\ ... _char:   l2338 = UNIQUE | NON_NULL, (empty)
                                    // 1370: b"lglddtrace.c\0": typeof(_177 = &raw const (*_178)) = *const {l2336} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1370: b"lglddtrace.c\0":   l2336 = UNIQUE | NON_NULL, (empty)
                                    // 1370: b"lglddtrace.c\ ... st u8: typeof(_176 = move _177 as *const u8 (Pointer(ArrayToPointer))) = *const {l2337} u8
                                    // 1370: b"lglddtrace.c\ ... st u8:   l2337 = UNIQUE | NON_NULL, (empty)
                                    479 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    // 1372: (*::core::mem:: ... ptr(): typeof(_181) = *const {l250} i8
                                    // 1372: (*::core::mem:: ... ptr():   l250 = UNIQUE | NON_NULL, (empty)
                                    // 1372: (*::core::mem:: ... ptr(): typeof(_182) = & {l252} [i8]
                                    // 1372: (*::core::mem:: ... ptr():   l252 = UNIQUE | NON_NULL, FIXED
                                    // 1372: (*::core::mem:: ... ptr(): typeof(_183) = & {l254} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1372: (*::core::mem:: ... ptr():   l254 = UNIQUE | NON_NULL, (empty)
                                    // 1372: ::core::mem::tr ... 0", ): typeof(_184) = & {l256} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1372: ::core::mem::tr ... 0", ):   l256 = UNIQUE | NON_NULL, FIXED
                                    // 1372: (*::core::mem:: ... ptr(): typeof(_183 = &(*_184)) = & {l2341} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1372: (*::core::mem:: ... ptr():   l2341 = UNIQUE | NON_NULL, (empty)
                                    // 1372: (*::core::mem:: ... ptr(): typeof(_182 = move _183 as &[i8] (Pointer(Unsize))) = & {l2342} [i8]
                                    // 1372: (*::core::mem:: ... ptr():   l2342 = UNIQUE | NON_NULL, FIXED
                                        b"void dd(void)\0",
                                        // 1373: b"void dd(void)\0": typeof(_185) = & {l258} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1373: b"void dd(void)\0":   l258 = UNIQUE | NON_NULL, (empty)
                                        // 1373: b"void dd(void)\0": typeof(_186) = & {l260} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1373: b"void dd(void)\0":   l260 = UNIQUE | NON_NULL, FIXED
                                        // 1373: b"void dd(void)\0": typeof(_186 = const b"void dd(void)\x00") = & {l2339} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1373: b"void dd(void)\0":   l2339 = UNIQUE | NON_NULL, (empty)
                                        // 1373: b"void dd(void)\0": typeof(_185 = &(*_186)) = & {l2340} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1373: b"void dd(void)\0":   l2340 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                        };
                        moved += 1;
                        moved;
                    }
                    let fresh2 = mapto;
                    mapto = mapto + 1;
                    *map.offset(idx as isize) = fresh2;
                    // 1384: map.offset(idx  ... size): typeof(_193) = *mut {l268} i32
                    // 1384: map.offset(idx  ... size):   l268 = UNIQUE | NON_NULL, (empty)
                    // 1384: map: typeof(_194) = *mut {l270} i32
                    // 1384: map:   l270 = UNIQUE | NON_NULL, (empty)
                    // 1384: map: typeof(_195) = *mut {l272} *mut {l273} i32
                    // 1384: map:   l272 = UNIQUE | NON_NULL, (empty)
                    // 1384: map:   l273 = UNIQUE | NON_NULL, (empty)
                } else {
                    *map.offset(idx as isize) = 0 as libc::c_int;
                    // 1386: map.offset(idx  ... size): typeof(_199) = *mut {l278} i32
                    // 1386: map.offset(idx  ... size):   l278 = UNIQUE | NON_NULL, (empty)
                    // 1386: map: typeof(_200) = *mut {l280} i32
                    // 1386: map:   l280 = UNIQUE | NON_NULL, (empty)
                    // 1386: map: typeof(_201) = *mut {l282} *mut {l283} i32
                    // 1386: map:   l282 = UNIQUE | NON_NULL, (empty)
                    // 1386: map:   l283 = UNIQUE | NON_NULL, (empty)
                }
                idx += 1;
                idx;
            }
            if moved == 0 {
                current_block_37 = 3333766589073626915;
            } else {
                res = run();
                if res == golden {
                // 1395: golden: typeof(_216) = *mut {l299} i32
                // 1395: golden:   l299 = UNIQUE | NON_NULL, (empty)
                    if verbose > 1 as libc::c_int {
                    // 1396: verbose: typeof(_220) = *mut {l304} i32
                    // 1396: verbose:   l304 = UNIQUE | NON_NULL, (empty)
                        newline();
                        msg(
                            b"moved %d variables\0" as *const u8 as *const libc::c_char,
                            // 1399: b"moved %d vari ... _char: typeof(_224) = *const {l309} i8
                            // 1399: b"moved %d vari ... _char:   l309 = UNIQUE | NON_NULL, (empty)
                            // 1399: b"moved %d vari ... st u8: typeof(_225) = *const {l311} u8
                            // 1399: b"moved %d vari ... st u8:   l311 = UNIQUE | NON_NULL, (empty)
                            // 1399: b"moved %d vari ... es\0": typeof(_226) = *const {l313} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1399: b"moved %d vari ... es\0":   l313 = UNIQUE | NON_NULL, (empty)
                            // 1399: b"moved %d vari ... es\0": typeof(_227) = & {l315} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1399: b"moved %d vari ... es\0":   l315 = UNIQUE | NON_NULL, FIXED
                            // 1399: b"moved %d vari ... _char: typeof(_224 = move _225 as *const i8 (Misc)) = *const {l2346} i8
                            // 1399: b"moved %d vari ... _char:   l2346 = UNIQUE | NON_NULL, (empty)
                            // 1399: b"moved %d vari ... st u8: typeof(_225 = move _226 as *const u8 (Pointer(ArrayToPointer))) = *const {l2345} u8
                            // 1399: b"moved %d vari ... st u8:   l2345 = UNIQUE | NON_NULL, (empty)
                            // 1399: b"moved %d vari ... es\0": typeof(_227 = const b"moved %d variables\x00") = & {l2343} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1399: b"moved %d vari ... es\0":   l2343 = UNIQUE | NON_NULL, (empty)
                            // 1399: b"moved %d vari ... es\0": typeof(_226 = &raw const (*_227)) = *const {l2344} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1399: b"moved %d vari ... es\0":   l2344 = UNIQUE | NON_NULL, (empty)
                            moved,
                        );
                    }
                    free(smap as *mut libc::c_void);
                    // 1403: smap as *mut li ... _void: typeof(_230) = *mut {l319} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1403: smap as *mut li ... _void:   l319 = UNIQUE | NON_NULL, (empty)
                    // 1403: smap: typeof(_231) = *mut {l321} i32
                    // 1403: smap:   l321 = UNIQUE | NON_NULL, (empty)
                    // 1403: smap as *mut li ... _void: typeof(_230 = move _231 as *mut libc::c_void (Misc)) = *mut {l2347} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1403: smap as *mut li ... _void:   l2347 = UNIQUE | NON_NULL, (empty)
                    changed = 1 as libc::c_int;
                    current_block_37 = 11763295167351361500;
                } else {
                    current_block_37 = 3333766589073626915;
                }
            }
            match current_block_37 {
                3333766589073626915 => {
                    free(map as *mut libc::c_void);
                    // 1412: map as *mut lib ... _void: typeof(_234) = *mut {l325} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1412: map as *mut lib ... _void:   l325 = UNIQUE | NON_NULL, (empty)
                    // 1412: map: typeof(_235) = *mut {l327} i32
                    // 1412: map:   l327 = UNIQUE | NON_NULL, (empty)
                    // 1412: map: typeof(_236) = *mut {l329} *mut {l330} i32
                    // 1412: map:   l329 = UNIQUE | NON_NULL, (empty)
                    // 1412: map:   l330 = UNIQUE | NON_NULL, (empty)
                    // 1412: map as *mut lib ... _void: typeof(_234 = move _235 as *mut libc::c_void (Misc)) = *mut {l2348} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1412: map as *mut lib ... _void:   l2348 = UNIQUE | NON_NULL, (empty)
                    map = smap;
                    // 1413: smap: typeof(_237) = *mut {l332} i32
                    // 1413: smap:   l332 = UNIQUE | NON_NULL, (empty)
                    // 1413: map: typeof(_238) = *mut {l334} *mut {l335} i32
                    // 1413: map:   l334 = UNIQUE | NON_NULL, (empty)
                    // 1413: map:   l335 = UNIQUE | NON_NULL, (empty)
                }
                _ => {}
            }
        }
        free(used as *mut libc::c_void);
        // 1418: used as *mut li ... _void: typeof(_240) = *mut {l338} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1418: used as *mut li ... _void:   l338 = UNIQUE | NON_NULL, (empty)
        // 1418: used: typeof(_241) = *mut {l340} i8
        // 1418: used:   l340 = UNIQUE | NON_NULL, (empty)
        // 1418: used as *mut li ... _void: typeof(_240 = move _241 as *mut libc::c_void (Misc)) = *mut {l2349} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1418: used as *mut li ... _void:   l2349 = UNIQUE | NON_NULL, (empty)
        rgran = 2 as libc::c_int;
        while rgran >= 0 as libc::c_int {
            to = 0 as libc::c_int;
            from = to;
            r = ranges;
            // 1423: ranges: typeof(_249) = *mut {l349} DefId(0:370 ~ lglddtrace[7e63]::Range)
            // 1423: ranges:   l349 = UNIQUE | NON_NULL, (empty)
            loop {
                while from < nevents && reme_shim(events.offset(from as isize)) != 0 {
                // 1425: nevents: typeof(_256) = *mut {l357} i32
                // 1425: nevents:   l357 = UNIQUE | NON_NULL, (empty)
                // 1425: events.offset(f ... size): typeof(_259) = *mut {l361} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1425: events.offset(f ... size):   l361 = UNIQUE | NON_NULL, (empty)
                // 1425: events: typeof(_260) = *mut {l363} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1425: events:   l363 = UNIQUE | NON_NULL, (empty)
                // 1425: events: typeof(_261) = *mut {l365} *mut {l366} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1425: events:   l365 = UNIQUE | NON_NULL, (empty)
                // 1425: events:   l366 = UNIQUE | NON_NULL, (empty)
                    from += 1;
                    from;
                }
                if from == nevents {
                // 1429: nevents: typeof(_273) = *mut {l379} i32
                // 1429: nevents:   l379 = UNIQUE | NON_NULL, (empty)
                    break;
                }
                to = from;
                if rgran == 2 as libc::c_int {
                    cluster = (*events.offset(from as isize)).type_0;
                    // 1434: events.offset(f ... size): typeof(_281) = *mut {l388} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1434: events.offset(f ... size):   l388 = UNIQUE | NON_NULL, (empty)
                    // 1434: events: typeof(_282) = *mut {l390} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1434: events:   l390 = UNIQUE | NON_NULL, (empty)
                    // 1434: events: typeof(_283) = *mut {l392} *mut {l393} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1434: events:   l392 = UNIQUE | NON_NULL, (empty)
                    // 1434: events:   l393 = UNIQUE | NON_NULL, (empty)
                    while (to + 1 as libc::c_int) < nevents {
                    // 1435: nevents: typeof(_292) = *mut {l403} i32
                    // 1435: nevents:   l403 = UNIQUE | NON_NULL, (empty)
                        e = events.offset(to as isize).offset(1 as libc::c_int as isize);
                        // 1436: events.offset(t ... size): typeof(_293) = *mut {l405} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1436: events.offset(t ... size):   l405 = UNIQUE | NON_NULL, (empty)
                        // 1436: events.offset(t ... size): typeof(_294) = *mut {l407} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1436: events.offset(t ... size):   l407 = UNIQUE | NON_NULL, (empty)
                        // 1436: events: typeof(_295) = *mut {l409} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1436: events:   l409 = UNIQUE | NON_NULL, (empty)
                        // 1436: events: typeof(_296) = *mut {l411} *mut {l412} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1436: events:   l411 = UNIQUE | NON_NULL, (empty)
                        // 1436: events:   l412 = UNIQUE | NON_NULL, (empty)
                        if (*e).type_0 as libc::c_uint != cluster as libc::c_uint {
                            if !(cluster as libc::c_uint == DEREF as libc::c_int as libc::c_uint
                                && (*e).type_0 as libc::c_uint
                                    == RETURN as libc::c_int as libc::c_uint)
                            {
                                if !(cluster as libc::c_uint == SAT as libc::c_int as libc::c_uint
                                    && (*e).type_0 as libc::c_uint
                                        == RETURN as libc::c_int as libc::c_uint)
                                {
                                    break;
                                }
                                to += 1;
                                to;
                                break;
                            }
                        } else if cluster as libc::c_uint == INIT as libc::c_int as libc::c_uint
                            || cluster as libc::c_uint == SAT as libc::c_int as libc::c_uint
                            || cluster as libc::c_uint == RELEASE as libc::c_int as libc::c_uint
                        {
                            break;
                        }
                        to += 1;
                        to;
                    }
                } else if rgran == 1 as libc::c_int {
                    if (to + 1 as libc::c_int) < nevents
                    // 1462: nevents: typeof(_360) = *mut {l477} i32
                    // 1462: nevents:   l477 = UNIQUE | NON_NULL, (empty)
                        && (*events.offset(to as isize)).type_0 as libc::c_uint
                        // 1463: events.offset(t ... size): typeof(_363) = *mut {l481} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1463: events.offset(t ... size):   l481 = UNIQUE | NON_NULL, (empty)
                        // 1463: events: typeof(_364) = *mut {l483} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1463: events:   l483 = UNIQUE | NON_NULL, (empty)
                        // 1463: events: typeof(_365) = *mut {l485} *mut {l486} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1463: events:   l485 = UNIQUE | NON_NULL, (empty)
                        // 1463: events:   l486 = UNIQUE | NON_NULL, (empty)
                            == ADD as libc::c_int as libc::c_uint
                    {
                        while (to + 1 as libc::c_int) < nevents && {
                        // 1466: nevents: typeof(_377) = *mut {l499} i32
                        // 1466: nevents:   l499 = UNIQUE | NON_NULL, (empty)
                            e = events.offset(to as isize);
                            // 1467: events.offset(t ... size): typeof(_379) = *mut {l502} DefId(0:354 ~ lglddtrace[7e63]::Event)
                            // 1467: events.offset(t ... size):   l502 = UNIQUE | NON_NULL, (empty)
                            // 1467: events: typeof(_380) = *mut {l504} DefId(0:354 ~ lglddtrace[7e63]::Event)
                            // 1467: events:   l504 = UNIQUE | NON_NULL, (empty)
                            // 1467: events: typeof(_381) = *mut {l506} *mut {l507} DefId(0:354 ~ lglddtrace[7e63]::Event)
                            // 1467: events:   l506 = UNIQUE | NON_NULL, (empty)
                            // 1467: events:   l507 = UNIQUE | NON_NULL, (empty)
                            reme_shim(e) != 0
                            // 1468: e: typeof(_386) = *mut {l513} DefId(0:354 ~ lglddtrace[7e63]::Event)
                            // 1468: e:   l513 = UNIQUE | NON_NULL, (empty)
                                || (*e).type_0 as libc::c_uint == ADD as libc::c_int as libc::c_uint
                                    && (*e).arg != 0
                        } {
                            to += 1;
                            to;
                        }
                    }
                } else if (to + 1 as libc::c_int) < nevents {
                // 1476: nevents: typeof(_405) = *mut {l533} i32
                // 1476: nevents:   l533 = UNIQUE | NON_NULL, (empty)
                    while (to + 1 as libc::c_int) < nevents && reme_shim(events.offset(to as isize)) != 0
                    // 1477: nevents: typeof(_413) = *mut {l542} i32
                    // 1477: nevents:   l542 = UNIQUE | NON_NULL, (empty)
                    // 1477: events.offset(t ... size): typeof(_416) = *mut {l546} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1477: events.offset(t ... size):   l546 = UNIQUE | NON_NULL, (empty)
                    // 1477: events: typeof(_417) = *mut {l548} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1477: events:   l548 = UNIQUE | NON_NULL, (empty)
                    // 1477: events: typeof(_418) = *mut {l550} *mut {l551} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1477: events:   l550 = UNIQUE | NON_NULL, (empty)
                    // 1477: events:   l551 = UNIQUE | NON_NULL, (empty)
                    {
                        to += 1;
                        to;
                    }
                }
                if r < ranges.offset(nevents as isize) {
                // 1483: r: typeof(_428) = *mut {l562} DefId(0:370 ~ lglddtrace[7e63]::Range)
                // 1483: r:   l562 = UNIQUE | NON_NULL, (empty)
                // 1483: ranges.offset(n ... size): typeof(_429) = *mut {l564} DefId(0:370 ~ lglddtrace[7e63]::Range)
                // 1483: ranges.offset(n ... size):   l564 = UNIQUE | NON_NULL, (empty)
                // 1483: ranges: typeof(_430) = *mut {l566} DefId(0:370 ~ lglddtrace[7e63]::Range)
                // 1483: ranges:   l566 = UNIQUE | NON_NULL, (empty)
                // 1483: nevents: typeof(_433) = *mut {l570} i32
                // 1483: nevents:   l570 = UNIQUE | NON_NULL, (empty)
                } else {
                    __assert_fail(
                        b"r < ranges + nevents\0" as *const u8 as *const libc::c_char,
                        // 1486: b"r < ranges +  ... _char: typeof(_436) = *const {l574} i8
                        // 1486: b"r < ranges +  ... _char:   l574 = UNIQUE | NON_NULL, (empty)
                        // 1486: b"r < ranges +  ... st u8: typeof(_437) = *const {l576} u8
                        // 1486: b"r < ranges +  ... st u8:   l576 = UNIQUE | NON_NULL, (empty)
                        // 1486: b"r < ranges +  ... ts\0": typeof(_438) = *const {l578} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 1486: b"r < ranges +  ... ts\0":   l578 = UNIQUE | NON_NULL, (empty)
                        // 1486: b"r < ranges +  ... ts\0": typeof(_439) = & {l580} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 1486: b"r < ranges +  ... ts\0":   l580 = UNIQUE | NON_NULL, FIXED
                        // 1486: b"r < ranges +  ... ts\0": typeof(_438 = &raw const (*_439)) = *const {l2351} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 1486: b"r < ranges +  ... ts\0":   l2351 = UNIQUE | NON_NULL, (empty)
                        // 1486: b"r < ranges +  ... _char: typeof(_436 = move _437 as *const i8 (Misc)) = *const {l2353} i8
                        // 1486: b"r < ranges +  ... _char:   l2353 = UNIQUE | NON_NULL, (empty)
                        // 1486: b"r < ranges +  ... st u8: typeof(_437 = move _438 as *const u8 (Pointer(ArrayToPointer))) = *const {l2352} u8
                        // 1486: b"r < ranges +  ... st u8:   l2352 = UNIQUE | NON_NULL, (empty)
                        // 1486: b"r < ranges +  ... ts\0": typeof(_439 = const b"r < ranges + nevents\x00") = & {l2350} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 1486: b"r < ranges +  ... ts\0":   l2350 = UNIQUE | NON_NULL, (empty)
                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                        // 1487: b"lglddtrace.c\ ... _char: typeof(_440) = *const {l582} i8
                        // 1487: b"lglddtrace.c\ ... _char:   l582 = UNIQUE | NON_NULL, (empty)
                        // 1487: b"lglddtrace.c\ ... st u8: typeof(_441) = *const {l584} u8
                        // 1487: b"lglddtrace.c\ ... st u8:   l584 = UNIQUE | NON_NULL, (empty)
                        // 1487: b"lglddtrace.c\0": typeof(_442) = *const {l586} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1487: b"lglddtrace.c\0":   l586 = UNIQUE | NON_NULL, (empty)
                        // 1487: b"lglddtrace.c\0": typeof(_443) = & {l588} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1487: b"lglddtrace.c\0":   l588 = UNIQUE | NON_NULL, FIXED
                        // 1487: b"lglddtrace.c\0": typeof(_442 = &raw const (*_443)) = *const {l2355} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1487: b"lglddtrace.c\0":   l2355 = UNIQUE | NON_NULL, (empty)
                        // 1487: b"lglddtrace.c\0": typeof(_443 = const b"lglddtrace.c\x00") = & {l2354} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 1487: b"lglddtrace.c\0":   l2354 = UNIQUE | NON_NULL, (empty)
                        // 1487: b"lglddtrace.c\ ... _char: typeof(_440 = move _441 as *const i8 (Misc)) = *const {l2357} i8
                        // 1487: b"lglddtrace.c\ ... _char:   l2357 = UNIQUE | NON_NULL, (empty)
                        // 1487: b"lglddtrace.c\ ... st u8: typeof(_441 = move _442 as *const u8 (Pointer(ArrayToPointer))) = *const {l2356} u8
                        // 1487: b"lglddtrace.c\ ... st u8:   l2356 = UNIQUE | NON_NULL, (empty)
                        534 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                        // 1489: (*::core::mem:: ... ptr(): typeof(_446) = *const {l592} i8
                        // 1489: (*::core::mem:: ... ptr():   l592 = UNIQUE | NON_NULL, (empty)
                        // 1489: (*::core::mem:: ... ptr(): typeof(_447) = & {l594} [i8]
                        // 1489: (*::core::mem:: ... ptr():   l594 = UNIQUE | NON_NULL, FIXED
                        // 1489: (*::core::mem:: ... ptr(): typeof(_448) = & {l596} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1489: (*::core::mem:: ... ptr():   l596 = UNIQUE | NON_NULL, (empty)
                        // 1489: ::core::mem::tr ... 0", ): typeof(_449) = & {l598} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1489: ::core::mem::tr ... 0", ):   l598 = UNIQUE | NON_NULL, FIXED
                        // 1489: (*::core::mem:: ... ptr(): typeof(_447 = move _448 as &[i8] (Pointer(Unsize))) = & {l2361} [i8]
                        // 1489: (*::core::mem:: ... ptr():   l2361 = UNIQUE | NON_NULL, FIXED
                        // 1489: (*::core::mem:: ... ptr(): typeof(_448 = &(*_449)) = & {l2360} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1489: (*::core::mem:: ... ptr():   l2360 = UNIQUE | NON_NULL, (empty)
                            b"void dd(void)\0",
                            // 1490: b"void dd(void)\0": typeof(_450) = & {l600} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1490: b"void dd(void)\0":   l600 = UNIQUE | NON_NULL, (empty)
                            // 1490: b"void dd(void)\0": typeof(_451) = & {l602} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1490: b"void dd(void)\0":   l602 = UNIQUE | NON_NULL, FIXED
                            // 1490: b"void dd(void)\0": typeof(_450 = &(*_451)) = & {l2359} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1490: b"void dd(void)\0":   l2359 = UNIQUE | NON_NULL, (empty)
                            // 1490: b"void dd(void)\0": typeof(_451 = const b"void dd(void)\x00") = & {l2358} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1490: b"void dd(void)\0":   l2358 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
                'c_9234: {
                    if r < ranges.offset(nevents as isize) {
                    // 1496: r: typeof(_454) = *mut {l606} DefId(0:370 ~ lglddtrace[7e63]::Range)
                    // 1496: r:   l606 = UNIQUE | NON_NULL, (empty)
                    // 1496: ranges.offset(n ... size): typeof(_455) = *mut {l608} DefId(0:370 ~ lglddtrace[7e63]::Range)
                    // 1496: ranges.offset(n ... size):   l608 = UNIQUE | NON_NULL, (empty)
                    // 1496: ranges: typeof(_456) = *mut {l610} DefId(0:370 ~ lglddtrace[7e63]::Range)
                    // 1496: ranges:   l610 = UNIQUE | NON_NULL, (empty)
                    // 1496: nevents: typeof(_459) = *mut {l614} i32
                    // 1496: nevents:   l614 = UNIQUE | NON_NULL, (empty)
                    } else {
                        __assert_fail(
                            b"r < ranges + nevents\0" as *const u8 as *const libc::c_char,
                            // 1499: b"r < ranges +  ... _char: typeof(_462) = *const {l618} i8
                            // 1499: b"r < ranges +  ... _char:   l618 = UNIQUE | NON_NULL, (empty)
                            // 1499: b"r < ranges +  ... st u8: typeof(_463) = *const {l620} u8
                            // 1499: b"r < ranges +  ... st u8:   l620 = UNIQUE | NON_NULL, (empty)
                            // 1499: b"r < ranges +  ... ts\0": typeof(_464) = *const {l622} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                            // 1499: b"r < ranges +  ... ts\0":   l622 = UNIQUE | NON_NULL, (empty)
                            // 1499: b"r < ranges +  ... ts\0": typeof(_465) = & {l624} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                            // 1499: b"r < ranges +  ... ts\0":   l624 = UNIQUE | NON_NULL, FIXED
                            // 1499: b"r < ranges +  ... st u8: typeof(_463 = move _464 as *const u8 (Pointer(ArrayToPointer))) = *const {l2364} u8
                            // 1499: b"r < ranges +  ... st u8:   l2364 = UNIQUE | NON_NULL, (empty)
                            // 1499: b"r < ranges +  ... ts\0": typeof(_465 = const b"r < ranges + nevents\x00") = & {l2362} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                            // 1499: b"r < ranges +  ... ts\0":   l2362 = UNIQUE | NON_NULL, (empty)
                            // 1499: b"r < ranges +  ... _char: typeof(_462 = move _463 as *const i8 (Misc)) = *const {l2365} i8
                            // 1499: b"r < ranges +  ... _char:   l2365 = UNIQUE | NON_NULL, (empty)
                            // 1499: b"r < ranges +  ... ts\0": typeof(_464 = &raw const (*_465)) = *const {l2363} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                            // 1499: b"r < ranges +  ... ts\0":   l2363 = UNIQUE | NON_NULL, (empty)
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            // 1500: b"lglddtrace.c\ ... _char: typeof(_466) = *const {l626} i8
                            // 1500: b"lglddtrace.c\ ... _char:   l626 = UNIQUE | NON_NULL, (empty)
                            // 1500: b"lglddtrace.c\ ... st u8: typeof(_467) = *const {l628} u8
                            // 1500: b"lglddtrace.c\ ... st u8:   l628 = UNIQUE | NON_NULL, (empty)
                            // 1500: b"lglddtrace.c\0": typeof(_468) = *const {l630} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1500: b"lglddtrace.c\0":   l630 = UNIQUE | NON_NULL, (empty)
                            // 1500: b"lglddtrace.c\0": typeof(_469) = & {l632} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1500: b"lglddtrace.c\0":   l632 = UNIQUE | NON_NULL, FIXED
                            // 1500: b"lglddtrace.c\0": typeof(_469 = const b"lglddtrace.c\x00") = & {l2366} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1500: b"lglddtrace.c\0":   l2366 = UNIQUE | NON_NULL, (empty)
                            // 1500: b"lglddtrace.c\ ... st u8: typeof(_467 = move _468 as *const u8 (Pointer(ArrayToPointer))) = *const {l2368} u8
                            // 1500: b"lglddtrace.c\ ... st u8:   l2368 = UNIQUE | NON_NULL, (empty)
                            // 1500: b"lglddtrace.c\0": typeof(_468 = &raw const (*_469)) = *const {l2367} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1500: b"lglddtrace.c\0":   l2367 = UNIQUE | NON_NULL, (empty)
                            // 1500: b"lglddtrace.c\ ... _char: typeof(_466 = move _467 as *const i8 (Misc)) = *const {l2369} i8
                            // 1500: b"lglddtrace.c\ ... _char:   l2369 = UNIQUE | NON_NULL, (empty)
                            534 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            // 1502: (*::core::mem:: ... ptr(): typeof(_472) = *const {l636} i8
                            // 1502: (*::core::mem:: ... ptr():   l636 = UNIQUE | NON_NULL, (empty)
                            // 1502: (*::core::mem:: ... ptr(): typeof(_473) = & {l638} [i8]
                            // 1502: (*::core::mem:: ... ptr():   l638 = UNIQUE | NON_NULL, FIXED
                            // 1502: (*::core::mem:: ... ptr(): typeof(_474) = & {l640} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1502: (*::core::mem:: ... ptr():   l640 = UNIQUE | NON_NULL, (empty)
                            // 1502: ::core::mem::tr ... 0", ): typeof(_475) = & {l642} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1502: ::core::mem::tr ... 0", ):   l642 = UNIQUE | NON_NULL, FIXED
                            // 1502: (*::core::mem:: ... ptr(): typeof(_474 = &(*_475)) = & {l2372} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1502: (*::core::mem:: ... ptr():   l2372 = UNIQUE | NON_NULL, (empty)
                            // 1502: (*::core::mem:: ... ptr(): typeof(_473 = move _474 as &[i8] (Pointer(Unsize))) = & {l2373} [i8]
                            // 1502: (*::core::mem:: ... ptr():   l2373 = UNIQUE | NON_NULL, FIXED
                                b"void dd(void)\0",
                                // 1503: b"void dd(void)\0": typeof(_476) = & {l644} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1503: b"void dd(void)\0":   l644 = UNIQUE | NON_NULL, (empty)
                                // 1503: b"void dd(void)\0": typeof(_477) = & {l646} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1503: b"void dd(void)\0":   l646 = UNIQUE | NON_NULL, FIXED
                                // 1503: b"void dd(void)\0": typeof(_477 = const b"void dd(void)\x00") = & {l2370} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1503: b"void dd(void)\0":   l2370 = UNIQUE | NON_NULL, (empty)
                                // 1503: b"void dd(void)\0": typeof(_476 = &(*_477)) = & {l2371} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1503: b"void dd(void)\0":   l2371 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                };
                (*r).from = from;
                (*r).to = to;
                (*r).removed = 2147483647 as libc::c_int;
                if verbose > 1 as libc::c_int {
                // 1512: verbose: typeof(_484) = *mut {l654} i32
                // 1512: verbose:   l654 = UNIQUE | NON_NULL, (empty)
                    msg(
                        b"range %d [%d,%d]\0" as *const u8 as *const libc::c_char,
                        // 1514: b"range %d [%d, ... _char: typeof(_487) = *const {l658} i8
                        // 1514: b"range %d [%d, ... _char:   l658 = UNIQUE | NON_NULL, (empty)
                        // 1514: b"range %d [%d, ... st u8: typeof(_488) = *const {l660} u8
                        // 1514: b"range %d [%d, ... st u8:   l660 = UNIQUE | NON_NULL, (empty)
                        // 1514: b"range %d [%d, ... d]\0": typeof(_489) = *const {l662} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1514: b"range %d [%d, ... d]\0":   l662 = UNIQUE | NON_NULL, (empty)
                        // 1514: b"range %d [%d, ... d]\0": typeof(_490) = & {l664} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1514: b"range %d [%d, ... d]\0":   l664 = UNIQUE | NON_NULL, FIXED
                        // 1514: b"range %d [%d, ... d]\0": typeof(_489 = &raw const (*_490)) = *const {l2375} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1514: b"range %d [%d, ... d]\0":   l2375 = UNIQUE | NON_NULL, (empty)
                        // 1514: b"range %d [%d, ... d]\0": typeof(_490 = const b"range %d [%d,%d]\x00") = & {l2374} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1514: b"range %d [%d, ... d]\0":   l2374 = UNIQUE | NON_NULL, (empty)
                        // 1514: b"range %d [%d, ... st u8: typeof(_488 = move _489 as *const u8 (Pointer(ArrayToPointer))) = *const {l2376} u8
                        // 1514: b"range %d [%d, ... st u8:   l2376 = UNIQUE | NON_NULL, (empty)
                        // 1514: b"range %d [%d, ... _char: typeof(_487 = move _488 as *const i8 (Misc)) = *const {l2377} i8
                        // 1514: b"range %d [%d, ... _char:   l2377 = UNIQUE | NON_NULL, (empty)
                        r.offset_from(ranges) as libc::c_long,
                        // 1515: r: typeof(_493) = *mut {l668} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1515: r:   l668 = UNIQUE | NON_NULL, (empty)
                        // 1515: ranges: typeof(_494) = *const {l670} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1515: ranges:   l670 = UNIQUE | NON_NULL, (empty)
                        // 1515: ranges: typeof(_495) = *mut {l672} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1515: ranges:   l672 = UNIQUE | NON_NULL, (empty)
                        // 1515: ranges: typeof(_494 = move _495 as *const Range (Pointer(MutToConstPointer))) = *const {l2378} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1515: ranges:   l2378 = UNIQUE | NON_NULL, (empty)
                        from,
                        to,
                    );
                }
                if verbose > 2 as libc::c_int {
                // 1520: verbose: typeof(_501) = *mut {l679} i32
                // 1520: verbose:   l679 = UNIQUE | NON_NULL, (empty)
                    i = from;
                    while i <= to {
                        e = events.offset(i as isize);
                        // 1523: events.offset(i ... size): typeof(_507) = *mut {l686} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1523: events.offset(i ... size):   l686 = UNIQUE | NON_NULL, (empty)
                        // 1523: events: typeof(_508) = *mut {l688} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1523: events:   l688 = UNIQUE | NON_NULL, (empty)
                        // 1523: events: typeof(_509) = *mut {l690} *mut {l691} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1523: events:   l690 = UNIQUE | NON_NULL, (empty)
                        // 1523: events:   l691 = UNIQUE | NON_NULL, (empty)
                        if !(reme_shim(e) != 0 && verbose < 4 as libc::c_int) {
                        // 1524: e: typeof(_517) = *mut {l700} DefId(0:354 ~ lglddtrace[7e63]::Event)
                        // 1524: e:   l700 = UNIQUE | NON_NULL, (empty)
                        // 1524: verbose: typeof(_520) = *mut {l704} i32
                        // 1524: verbose:   l704 = UNIQUE | NON_NULL, (empty)
                            if (*e).type_0 as libc::c_uint == OPTION as libc::c_int as libc::c_uint
                            {
                                msg(
                                    b"range %d [%d] %s %s %d%s\0" as *const u8
                                    // 1528: b"range %d [%d] ... _char: typeof(_527) = *const {l712} i8
                                    // 1528: b"range %d [%d] ... _char:   l712 = UNIQUE | NON_NULL, (empty)
                                    // 1528: b"range %d [%d] ... st u8: typeof(_528) = *const {l714} u8
                                    // 1528: b"range %d [%d] ... st u8:   l714 = UNIQUE | NON_NULL, (empty)
                                    // 1528: b"range %d [%d] ... %s\0": typeof(_529) = *const {l716} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1528: b"range %d [%d] ... %s\0":   l716 = UNIQUE | NON_NULL, (empty)
                                    // 1528: b"range %d [%d] ... %s\0": typeof(_530) = & {l718} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1528: b"range %d [%d] ... %s\0":   l718 = UNIQUE | NON_NULL, FIXED
                                    // 1528: b"range %d [%d] ... %s\0": typeof(_530 = const b"range %d [%d] %s %s %d%s\x00") = & {l2379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1528: b"range %d [%d] ... %s\0":   l2379 = UNIQUE | NON_NULL, (empty)
                                    // 1528: b"range %d [%d] ... _char: typeof(_527 = move _528 as *const i8 (Misc)) = *const {l2382} i8
                                    // 1528: b"range %d [%d] ... _char:   l2382 = UNIQUE | NON_NULL, (empty)
                                    // 1528: b"range %d [%d] ... st u8: typeof(_528 = move _529 as *const u8 (Pointer(ArrayToPointer))) = *const {l2381} u8
                                    // 1528: b"range %d [%d] ... st u8:   l2381 = UNIQUE | NON_NULL, (empty)
                                    // 1528: b"range %d [%d] ... %s\0": typeof(_529 = &raw const (*_530)) = *const {l2380} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1528: b"range %d [%d] ... %s\0":   l2380 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                    r.offset_from(ranges) as libc::c_long,
                                    // 1530: r: typeof(_533) = *mut {l722} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1530: r:   l722 = UNIQUE | NON_NULL, (empty)
                                    // 1530: ranges: typeof(_534) = *const {l724} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1530: ranges:   l724 = UNIQUE | NON_NULL, (empty)
                                    // 1530: ranges: typeof(_535) = *mut {l726} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1530: ranges:   l726 = UNIQUE | NON_NULL, (empty)
                                    // 1530: ranges: typeof(_534 = move _535 as *const Range (Pointer(MutToConstPointer))) = *const {l2383} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1530: ranges:   l2383 = UNIQUE | NON_NULL, (empty)
                                    i,
                                    type2str((*e).type_0),
                                    // 1532: type2str((*e).t ... pe_0): typeof(_537) = *const {l729} i8
                                    // 1532: type2str((*e).t ... pe_0):   l729 = UNIQUE | NON_NULL, (empty)
                                    (*e).opt,
                                    // 1533: (*e).opt: typeof(_539) = *mut {l732} i8
                                    // 1533: (*e).opt:   l732 = UNIQUE | NON_NULL, (empty)
                                    (*e).arg,
                                    if reme_shim(e) != 0 {
                                    // 1535: if reme(e) != 0 ... har }: typeof(_541) = *const {l735} i8
                                    // 1535: if reme(e) != 0 ... har }:   l735 = UNIQUE | NON_NULL, (empty)
                                    // 1535: e: typeof(_544) = *mut {l739} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                    // 1535: e:   l739 = UNIQUE | NON_NULL, (empty)
                                        b" (removed)\0" as *const u8 as *const libc::c_char
                                        // 1536: b" (removed)\0" ... st u8: typeof(_545) = *const {l741} u8
                                        // 1536: b" (removed)\0" ... st u8:   l741 = UNIQUE | NON_NULL, (empty)
                                        // 1536: b" (removed)\0": typeof(_546) = *const {l743} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1536: b" (removed)\0":   l743 = UNIQUE | NON_NULL, (empty)
                                        // 1536: b" (removed)\0": typeof(_547) = & {l745} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1536: b" (removed)\0":   l745 = UNIQUE | NON_NULL, FIXED
                                        // 1536: b" (removed)\0": typeof(_547 = const b" (removed)\x00") = & {l2384} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1536: b" (removed)\0":   l2384 = UNIQUE | NON_NULL, (empty)
                                        // 1536: b" (removed)\0": typeof(_546 = &raw const (*_547)) = *const {l2385} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1536: b" (removed)\0":   l2385 = UNIQUE | NON_NULL, (empty)
                                        // 1536: b" (removed)\0" ... st u8: typeof(_545 = move _546 as *const u8 (Pointer(ArrayToPointer))) = *const {l2386} u8
                                        // 1536: b" (removed)\0" ... st u8:   l2386 = UNIQUE | NON_NULL, (empty)
                                        // 1536: b" (removed)\0" ... _char: typeof(_541 = move _545 as *const i8 (Misc)) = *const {l2387} i8
                                        // 1536: b" (removed)\0" ... _char:   l2387 = UNIQUE | NON_NULL, (empty)
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                        // 1538: b"\0" as *const u8: typeof(_548) = *const {l747} u8
                                        // 1538: b"\0" as *const u8:   l747 = UNIQUE | NON_NULL, (empty)
                                        // 1538: b"\0": typeof(_549) = *const {l749} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1538: b"\0":   l749 = UNIQUE | NON_NULL, (empty)
                                        // 1538: b"\0": typeof(_550) = & {l751} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1538: b"\0":   l751 = UNIQUE | NON_NULL, FIXED
                                        // 1538: b"\0": typeof(_550 = const b"\x00") = & {l2388} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1538: b"\0":   l2388 = UNIQUE | NON_NULL, (empty)
                                        // 1538: b"\0" as *const u8: typeof(_548 = move _549 as *const u8 (Pointer(ArrayToPointer))) = *const {l2390} u8
                                        // 1538: b"\0" as *const u8:   l2390 = UNIQUE | NON_NULL, (empty)
                                        // 1538: b"\0": typeof(_549 = &raw const (*_550)) = *const {l2389} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1538: b"\0":   l2389 = UNIQUE | NON_NULL, (empty)
                                        // 1538: b"\0" as *const ... _char: typeof(_541 = move _548 as *const i8 (Misc)) = *const {l2391} i8
                                        // 1538: b"\0" as *const ... _char:   l2391 = UNIQUE | NON_NULL, (empty)
                                    },
                                );
                            } else {
                                msg(
                                    b"range %d [%d] %s %d%s\0" as *const u8 as *const libc::c_char,
                                    // 1543: b"range %d [%d] ... _char: typeof(_552) = *const {l754} i8
                                    // 1543: b"range %d [%d] ... _char:   l754 = UNIQUE | NON_NULL, (empty)
                                    // 1543: b"range %d [%d] ... st u8: typeof(_553) = *const {l756} u8
                                    // 1543: b"range %d [%d] ... st u8:   l756 = UNIQUE | NON_NULL, (empty)
                                    // 1543: b"range %d [%d] ... %s\0": typeof(_554) = *const {l758} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 1543: b"range %d [%d] ... %s\0":   l758 = UNIQUE | NON_NULL, (empty)
                                    // 1543: b"range %d [%d] ... %s\0": typeof(_555) = & {l760} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 1543: b"range %d [%d] ... %s\0":   l760 = UNIQUE | NON_NULL, FIXED
                                    // 1543: b"range %d [%d] ... %s\0": typeof(_554 = &raw const (*_555)) = *const {l2393} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 1543: b"range %d [%d] ... %s\0":   l2393 = UNIQUE | NON_NULL, (empty)
                                    // 1543: b"range %d [%d] ... st u8: typeof(_553 = move _554 as *const u8 (Pointer(ArrayToPointer))) = *const {l2394} u8
                                    // 1543: b"range %d [%d] ... st u8:   l2394 = UNIQUE | NON_NULL, (empty)
                                    // 1543: b"range %d [%d] ... _char: typeof(_552 = move _553 as *const i8 (Misc)) = *const {l2395} i8
                                    // 1543: b"range %d [%d] ... _char:   l2395 = UNIQUE | NON_NULL, (empty)
                                    // 1543: b"range %d [%d] ... %s\0": typeof(_555 = const b"range %d [%d] %s %d%s\x00") = & {l2392} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 1543: b"range %d [%d] ... %s\0":   l2392 = UNIQUE | NON_NULL, (empty)
                                    r.offset_from(ranges) as libc::c_long,
                                    // 1544: r: typeof(_558) = *mut {l764} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1544: r:   l764 = UNIQUE | NON_NULL, (empty)
                                    // 1544: ranges: typeof(_559) = *const {l766} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1544: ranges:   l766 = UNIQUE | NON_NULL, (empty)
                                    // 1544: ranges: typeof(_560) = *mut {l768} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1544: ranges:   l768 = UNIQUE | NON_NULL, (empty)
                                    // 1544: ranges: typeof(_559 = move _560 as *const Range (Pointer(MutToConstPointer))) = *const {l2396} DefId(0:370 ~ lglddtrace[7e63]::Range)
                                    // 1544: ranges:   l2396 = UNIQUE | NON_NULL, (empty)
                                    i,
                                    type2str((*e).type_0),
                                    // 1546: type2str((*e).t ... pe_0): typeof(_562) = *const {l771} i8
                                    // 1546: type2str((*e).t ... pe_0):   l771 = UNIQUE | NON_NULL, (empty)
                                    (*e).arg,
                                    if reme_shim(e) != 0 {
                                    // 1548: if reme(e) != 0 ... har }: typeof(_565) = *const {l775} i8
                                    // 1548: if reme(e) != 0 ... har }:   l775 = UNIQUE | NON_NULL, (empty)
                                    // 1548: e: typeof(_568) = *mut {l779} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                    // 1548: e:   l779 = UNIQUE | NON_NULL, (empty)
                                        b" (removed)\0" as *const u8 as *const libc::c_char
                                        // 1549: b" (removed)\0" ... st u8: typeof(_569) = *const {l781} u8
                                        // 1549: b" (removed)\0" ... st u8:   l781 = UNIQUE | NON_NULL, (empty)
                                        // 1549: b" (removed)\0": typeof(_570) = *const {l783} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1549: b" (removed)\0":   l783 = UNIQUE | NON_NULL, (empty)
                                        // 1549: b" (removed)\0": typeof(_571) = & {l785} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1549: b" (removed)\0":   l785 = UNIQUE | NON_NULL, FIXED
                                        // 1549: b" (removed)\0" ... st u8: typeof(_569 = move _570 as *const u8 (Pointer(ArrayToPointer))) = *const {l2399} u8
                                        // 1549: b" (removed)\0" ... st u8:   l2399 = UNIQUE | NON_NULL, (empty)
                                        // 1549: b" (removed)\0" ... _char: typeof(_565 = move _569 as *const i8 (Misc)) = *const {l2400} i8
                                        // 1549: b" (removed)\0" ... _char:   l2400 = UNIQUE | NON_NULL, (empty)
                                        // 1549: b" (removed)\0": typeof(_570 = &raw const (*_571)) = *const {l2398} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1549: b" (removed)\0":   l2398 = UNIQUE | NON_NULL, (empty)
                                        // 1549: b" (removed)\0": typeof(_571 = const b" (removed)\x00") = & {l2397} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                                        // 1549: b" (removed)\0":   l2397 = UNIQUE | NON_NULL, (empty)
                                    } else {
                                        b"\0" as *const u8 as *const libc::c_char
                                        // 1551: b"\0" as *const u8: typeof(_572) = *const {l787} u8
                                        // 1551: b"\0" as *const u8:   l787 = UNIQUE | NON_NULL, (empty)
                                        // 1551: b"\0": typeof(_573) = *const {l789} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1551: b"\0":   l789 = UNIQUE | NON_NULL, (empty)
                                        // 1551: b"\0": typeof(_574) = & {l791} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1551: b"\0":   l791 = UNIQUE | NON_NULL, FIXED
                                        // 1551: b"\0" as *const u8: typeof(_572 = move _573 as *const u8 (Pointer(ArrayToPointer))) = *const {l2403} u8
                                        // 1551: b"\0" as *const u8:   l2403 = UNIQUE | NON_NULL, (empty)
                                        // 1551: b"\0": typeof(_573 = &raw const (*_574)) = *const {l2402} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1551: b"\0":   l2402 = UNIQUE | NON_NULL, (empty)
                                        // 1551: b"\0" as *const ... _char: typeof(_565 = move _572 as *const i8 (Misc)) = *const {l2404} i8
                                        // 1551: b"\0" as *const ... _char:   l2404 = UNIQUE | NON_NULL, (empty)
                                        // 1551: b"\0": typeof(_574 = const b"\x00") = & {l2401} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                                        // 1551: b"\0":   l2401 = UNIQUE | NON_NULL, (empty)
                                    },
                                );
                            }
                        }
                        i += 1;
                        i;
                    }
                }
                r = r.offset(1);
                // 1560: r.offset(1): typeof(_580) = *mut {l798} DefId(0:370 ~ lglddtrace[7e63]::Range)
                // 1560: r.offset(1):   l798 = UNIQUE | NON_NULL, (empty)
                // 1560: r: typeof(_581) = *mut {l800} DefId(0:370 ~ lglddtrace[7e63]::Range)
                // 1560: r:   l800 = UNIQUE | NON_NULL, (empty)
                r;
                // 1561: r: typeof(_582) = *mut {l802} DefId(0:370 ~ lglddtrace[7e63]::Range)
                // 1561: r:   l802 = UNIQUE | NON_NULL, (empty)
                from = to + 1 as libc::c_int;
                if from == nevents {
                // 1563: nevents: typeof(_589) = *mut {l810} i32
                // 1563: nevents:   l810 = UNIQUE | NON_NULL, (empty)
                    break;
                }
            }
            nranges = r.offset_from(ranges) as libc::c_long as libc::c_int;
            // 1567: r: typeof(_593) = *mut {l815} DefId(0:370 ~ lglddtrace[7e63]::Range)
            // 1567: r:   l815 = UNIQUE | NON_NULL, (empty)
            // 1567: ranges: typeof(_594) = *const {l817} DefId(0:370 ~ lglddtrace[7e63]::Range)
            // 1567: ranges:   l817 = UNIQUE | NON_NULL, (empty)
            // 1567: ranges: typeof(_595) = *mut {l819} DefId(0:370 ~ lglddtrace[7e63]::Range)
            // 1567: ranges:   l819 = UNIQUE | NON_NULL, (empty)
            // 1567: ranges: typeof(_594 = move _595 as *const Range (Pointer(MutToConstPointer))) = *const {l2405} DefId(0:370 ~ lglddtrace[7e63]::Range)
            // 1567: ranges:   l2405 = UNIQUE | NON_NULL, (empty)
            if verbose > 1 as libc::c_int {
            // 1568: verbose: typeof(_599) = *mut {l824} i32
            // 1568: verbose:   l824 = UNIQUE | NON_NULL, (empty)
                msg(
                    b"found %d ranges of range granularity %d\0" as *const u8
                    // 1570: b"found %d rang ... _char: typeof(_602) = *const {l828} i8
                    // 1570: b"found %d rang ... _char:   l828 = UNIQUE | NON_NULL, (empty)
                    // 1570: b"found %d rang ... st u8: typeof(_603) = *const {l830} u8
                    // 1570: b"found %d rang ... st u8:   l830 = UNIQUE | NON_NULL, (empty)
                    // 1570: b"found %d rang ... %d\0": typeof(_604) = *const {l832} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 1570: b"found %d rang ... %d\0":   l832 = UNIQUE | NON_NULL, (empty)
                    // 1570: b"found %d rang ... %d\0": typeof(_605) = & {l834} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 1570: b"found %d rang ... %d\0":   l834 = UNIQUE | NON_NULL, FIXED
                    // 1570: b"found %d rang ... %d\0": typeof(_605 = const b"found %d ranges of range granularity %d\x00") = & {l2406} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 1570: b"found %d rang ... %d\0":   l2406 = UNIQUE | NON_NULL, (empty)
                    // 1570: b"found %d rang ... st u8: typeof(_603 = move _604 as *const u8 (Pointer(ArrayToPointer))) = *const {l2408} u8
                    // 1570: b"found %d rang ... st u8:   l2408 = UNIQUE | NON_NULL, (empty)
                    // 1570: b"found %d rang ... _char: typeof(_602 = move _603 as *const i8 (Misc)) = *const {l2409} i8
                    // 1570: b"found %d rang ... _char:   l2409 = UNIQUE | NON_NULL, (empty)
                    // 1570: b"found %d rang ... %d\0": typeof(_604 = &raw const (*_605)) = *const {l2407} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                    // 1570: b"found %d rang ... %d\0":   l2407 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    nranges,
                    rgran,
                );
            }
            width = nranges / 2 as libc::c_int;
            while width > 0 as libc::c_int {
                pos = 0 as libc::c_int;
                loop {
                    rep(
                        b"g%d w%d : %6d .. %-6d / %d %lld\0" as *const u8 as *const libc::c_char,
                        // 1581: b"g%d w%d : %6d ... _char: typeof(_621) = *const {l851} i8
                        // 1581: b"g%d w%d : %6d ... _char:   l851 = UNIQUE | NON_NULL, (empty)
                        // 1581: b"g%d w%d : %6d ... st u8: typeof(_622) = *const {l853} u8
                        // 1581: b"g%d w%d : %6d ... st u8:   l853 = UNIQUE | NON_NULL, (empty)
                        // 1581: b"g%d w%d : %6d ... ld\0": typeof(_623) = *const {l855} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1581: b"g%d w%d : %6d ... ld\0":   l855 = UNIQUE | NON_NULL, (empty)
                        // 1581: b"g%d w%d : %6d ... ld\0": typeof(_624) = & {l857} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1581: b"g%d w%d : %6d ... ld\0":   l857 = UNIQUE | NON_NULL, FIXED
                        // 1581: b"g%d w%d : %6d ... st u8: typeof(_622 = move _623 as *const u8 (Pointer(ArrayToPointer))) = *const {l2412} u8
                        // 1581: b"g%d w%d : %6d ... st u8:   l2412 = UNIQUE | NON_NULL, (empty)
                        // 1581: b"g%d w%d : %6d ... ld\0": typeof(_623 = &raw const (*_624)) = *const {l2411} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1581: b"g%d w%d : %6d ... ld\0":   l2411 = UNIQUE | NON_NULL, (empty)
                        // 1581: b"g%d w%d : %6d ... _char: typeof(_621 = move _622 as *const i8 (Misc)) = *const {l2413} i8
                        // 1581: b"g%d w%d : %6d ... _char:   l2413 = UNIQUE | NON_NULL, (empty)
                        // 1581: b"g%d w%d : %6d ... ld\0": typeof(_624 = const b"g%d w%d : %6d .. %-6d / %d %lld\x00") = & {l2410} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1581: b"g%d w%d : %6d ... ld\0":   l2410 = UNIQUE | NON_NULL, (empty)
                        rgran,
                        width,
                        pos,
                        if pos + width <= nranges {
                            pos + width - 1 as libc::c_int
                        } else {
                            nranges - 1 as libc::c_int
                        },
                        nranges,
                        sumoptvals,
                        // 1591: sumoptvals: typeof(_646) = *mut {l880} i64
                        // 1591: sumoptvals:   l880 = UNIQUE | NON_NULL, (empty)
                    );
                    found = 0 as libc::c_int;
                    i = pos;
                    while i < nranges && i < pos + width {
                        r = ranges.offset(i as isize);
                        // 1596: ranges.offset(i ... size): typeof(_660) = *mut {l895} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1596: ranges.offset(i ... size):   l895 = UNIQUE | NON_NULL, (empty)
                        // 1596: ranges: typeof(_661) = *mut {l897} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1596: ranges:   l897 = UNIQUE | NON_NULL, (empty)
                        if !(remr_shim(r) != 0) {
                        // 1597: r: typeof(_668) = *mut {l905} DefId(0:370 ~ lglddtrace[7e63]::Range)
                        // 1597: r:   l905 = UNIQUE | NON_NULL, (empty)
                            (*r).removed = runs;
                            // 1598: runs: typeof(_670) = *mut {l908} i32
                            // 1598: runs:   l908 = UNIQUE | NON_NULL, (empty)
                            found = 0 as libc::c_int;
                            j = (*r).from;
                            while j <= (*r).to {
                                e = events.offset(j as isize);
                                // 1602: events.offset(j ... size): typeof(_677) = *mut {l916} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                // 1602: events.offset(j ... size):   l916 = UNIQUE | NON_NULL, (empty)
                                // 1602: events: typeof(_678) = *mut {l918} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                // 1602: events:   l918 = UNIQUE | NON_NULL, (empty)
                                // 1602: events: typeof(_679) = *mut {l920} *mut {l921} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                // 1602: events:   l920 = UNIQUE | NON_NULL, (empty)
                                // 1602: events:   l921 = UNIQUE | NON_NULL, (empty)
                                if !(reme_shim(e) != 0) {
                                // 1603: e: typeof(_686) = *mut {l929} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                // 1603: e:   l929 = UNIQUE | NON_NULL, (empty)
                                    found += 1;
                                    found;
                                    (*e).removed = runs;
                                    // 1606: runs: typeof(_690) = *mut {l934} i32
                                    // 1606: runs:   l934 = UNIQUE | NON_NULL, (empty)
                                }
                                j += 1;
                                j;
                            }
                            if found != 0 {
                            } else {
                                __assert_fail(
                                    b"found\0" as *const u8 as *const libc::c_char,
                                    // 1614: b"found\0" as * ... _char: typeof(_701) = *const {l946} i8
                                    // 1614: b"found\0" as * ... _char:   l946 = UNIQUE | NON_NULL, (empty)
                                    // 1614: b"found\0" as * ... st u8: typeof(_702) = *const {l948} u8
                                    // 1614: b"found\0" as * ... st u8:   l948 = UNIQUE | NON_NULL, (empty)
                                    // 1614: b"found\0": typeof(_703) = *const {l950} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                    // 1614: b"found\0":   l950 = UNIQUE | NON_NULL, (empty)
                                    // 1614: b"found\0": typeof(_704) = & {l952} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                    // 1614: b"found\0":   l952 = UNIQUE | NON_NULL, FIXED
                                    // 1614: b"found\0" as * ... _char: typeof(_701 = move _702 as *const i8 (Misc)) = *const {l2417} i8
                                    // 1614: b"found\0" as * ... _char:   l2417 = UNIQUE | NON_NULL, (empty)
                                    // 1614: b"found\0": typeof(_704 = const b"found\x00") = & {l2414} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                    // 1614: b"found\0":   l2414 = UNIQUE | NON_NULL, (empty)
                                    // 1614: b"found\0" as * ... st u8: typeof(_702 = move _703 as *const u8 (Pointer(ArrayToPointer))) = *const {l2416} u8
                                    // 1614: b"found\0" as * ... st u8:   l2416 = UNIQUE | NON_NULL, (empty)
                                    // 1614: b"found\0": typeof(_703 = &raw const (*_704)) = *const {l2415} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                    // 1614: b"found\0":   l2415 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 1615: b"lglddtrace.c\ ... _char: typeof(_705) = *const {l954} i8
                                    // 1615: b"lglddtrace.c\ ... _char:   l954 = UNIQUE | NON_NULL, (empty)
                                    // 1615: b"lglddtrace.c\ ... st u8: typeof(_706) = *const {l956} u8
                                    // 1615: b"lglddtrace.c\ ... st u8:   l956 = UNIQUE | NON_NULL, (empty)
                                    // 1615: b"lglddtrace.c\0": typeof(_707) = *const {l958} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1615: b"lglddtrace.c\0":   l958 = UNIQUE | NON_NULL, (empty)
                                    // 1615: b"lglddtrace.c\0": typeof(_708) = & {l960} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1615: b"lglddtrace.c\0":   l960 = UNIQUE | NON_NULL, FIXED
                                    // 1615: b"lglddtrace.c\0": typeof(_707 = &raw const (*_708)) = *const {l2419} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1615: b"lglddtrace.c\0":   l2419 = UNIQUE | NON_NULL, (empty)
                                    // 1615: b"lglddtrace.c\0": typeof(_708 = const b"lglddtrace.c\x00") = & {l2418} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1615: b"lglddtrace.c\0":   l2418 = UNIQUE | NON_NULL, (empty)
                                    // 1615: b"lglddtrace.c\ ... _char: typeof(_705 = move _706 as *const i8 (Misc)) = *const {l2421} i8
                                    // 1615: b"lglddtrace.c\ ... _char:   l2421 = UNIQUE | NON_NULL, (empty)
                                    // 1615: b"lglddtrace.c\ ... st u8: typeof(_706 = move _707 as *const u8 (Pointer(ArrayToPointer))) = *const {l2420} u8
                                    // 1615: b"lglddtrace.c\ ... st u8:   l2420 = UNIQUE | NON_NULL, (empty)
                                    581 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    // 1617: (*::core::mem:: ... ptr(): typeof(_711) = *const {l964} i8
                                    // 1617: (*::core::mem:: ... ptr():   l964 = UNIQUE | NON_NULL, (empty)
                                    // 1617: (*::core::mem:: ... ptr(): typeof(_712) = & {l966} [i8]
                                    // 1617: (*::core::mem:: ... ptr():   l966 = UNIQUE | NON_NULL, FIXED
                                    // 1617: (*::core::mem:: ... ptr(): typeof(_713) = & {l968} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1617: (*::core::mem:: ... ptr():   l968 = UNIQUE | NON_NULL, (empty)
                                    // 1617: ::core::mem::tr ... 0", ): typeof(_714) = & {l970} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1617: ::core::mem::tr ... 0", ):   l970 = UNIQUE | NON_NULL, FIXED
                                    // 1617: (*::core::mem:: ... ptr(): typeof(_713 = &(*_714)) = & {l2424} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1617: (*::core::mem:: ... ptr():   l2424 = UNIQUE | NON_NULL, (empty)
                                    // 1617: (*::core::mem:: ... ptr(): typeof(_712 = move _713 as &[i8] (Pointer(Unsize))) = & {l2425} [i8]
                                    // 1617: (*::core::mem:: ... ptr():   l2425 = UNIQUE | NON_NULL, FIXED
                                        b"void dd(void)\0",
                                        // 1618: b"void dd(void)\0": typeof(_715) = & {l972} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1618: b"void dd(void)\0":   l972 = UNIQUE | NON_NULL, (empty)
                                        // 1618: b"void dd(void)\0": typeof(_716) = & {l974} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1618: b"void dd(void)\0":   l974 = UNIQUE | NON_NULL, FIXED
                                        // 1618: b"void dd(void)\0": typeof(_715 = &(*_716)) = & {l2423} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1618: b"void dd(void)\0":   l2423 = UNIQUE | NON_NULL, (empty)
                                        // 1618: b"void dd(void)\0": typeof(_716 = const b"void dd(void)\x00") = & {l2422} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1618: b"void dd(void)\0":   l2422 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                            'c_8856: {
                                if found != 0 {
                                } else {
                                    __assert_fail(
                                        b"found\0" as *const u8 as *const libc::c_char,
                                        // 1627: b"found\0" as * ... _char: typeof(_722) = *const {l981} i8
                                        // 1627: b"found\0" as * ... _char:   l981 = UNIQUE | NON_NULL, (empty)
                                        // 1627: b"found\0" as * ... st u8: typeof(_723) = *const {l983} u8
                                        // 1627: b"found\0" as * ... st u8:   l983 = UNIQUE | NON_NULL, (empty)
                                        // 1627: b"found\0": typeof(_724) = *const {l985} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                        // 1627: b"found\0":   l985 = UNIQUE | NON_NULL, (empty)
                                        // 1627: b"found\0": typeof(_725) = & {l987} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                        // 1627: b"found\0":   l987 = UNIQUE | NON_NULL, FIXED
                                        // 1627: b"found\0": typeof(_724 = &raw const (*_725)) = *const {l2427} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                        // 1627: b"found\0":   l2427 = UNIQUE | NON_NULL, (empty)
                                        // 1627: b"found\0": typeof(_725 = const b"found\x00") = & {l2426} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                                        // 1627: b"found\0":   l2426 = UNIQUE | NON_NULL, (empty)
                                        // 1627: b"found\0" as * ... st u8: typeof(_723 = move _724 as *const u8 (Pointer(ArrayToPointer))) = *const {l2428} u8
                                        // 1627: b"found\0" as * ... st u8:   l2428 = UNIQUE | NON_NULL, (empty)
                                        // 1627: b"found\0" as * ... _char: typeof(_722 = move _723 as *const i8 (Misc)) = *const {l2429} i8
                                        // 1627: b"found\0" as * ... _char:   l2429 = UNIQUE | NON_NULL, (empty)
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        // 1628: b"lglddtrace.c\ ... _char: typeof(_726) = *const {l989} i8
                                        // 1628: b"lglddtrace.c\ ... _char:   l989 = UNIQUE | NON_NULL, (empty)
                                        // 1628: b"lglddtrace.c\ ... st u8: typeof(_727) = *const {l991} u8
                                        // 1628: b"lglddtrace.c\ ... st u8:   l991 = UNIQUE | NON_NULL, (empty)
                                        // 1628: b"lglddtrace.c\0": typeof(_728) = *const {l993} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1628: b"lglddtrace.c\0":   l993 = UNIQUE | NON_NULL, (empty)
                                        // 1628: b"lglddtrace.c\0": typeof(_729) = & {l995} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1628: b"lglddtrace.c\0":   l995 = UNIQUE | NON_NULL, FIXED
                                        // 1628: b"lglddtrace.c\ ... _char: typeof(_726 = move _727 as *const i8 (Misc)) = *const {l2433} i8
                                        // 1628: b"lglddtrace.c\ ... _char:   l2433 = UNIQUE | NON_NULL, (empty)
                                        // 1628: b"lglddtrace.c\0": typeof(_728 = &raw const (*_729)) = *const {l2431} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1628: b"lglddtrace.c\0":   l2431 = UNIQUE | NON_NULL, (empty)
                                        // 1628: b"lglddtrace.c\ ... st u8: typeof(_727 = move _728 as *const u8 (Pointer(ArrayToPointer))) = *const {l2432} u8
                                        // 1628: b"lglddtrace.c\ ... st u8:   l2432 = UNIQUE | NON_NULL, (empty)
                                        // 1628: b"lglddtrace.c\0": typeof(_729 = const b"lglddtrace.c\x00") = & {l2430} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1628: b"lglddtrace.c\0":   l2430 = UNIQUE | NON_NULL, (empty)
                                        581 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                        // 1630: (*::core::mem:: ... ptr(): typeof(_732) = *const {l999} i8
                                        // 1630: (*::core::mem:: ... ptr():   l999 = UNIQUE | NON_NULL, (empty)
                                        // 1630: (*::core::mem:: ... ptr(): typeof(_733) = & {l1001} [i8]
                                        // 1630: (*::core::mem:: ... ptr():   l1001 = UNIQUE | NON_NULL, FIXED
                                        // 1630: (*::core::mem:: ... ptr(): typeof(_734) = & {l1003} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1630: (*::core::mem:: ... ptr():   l1003 = UNIQUE | NON_NULL, (empty)
                                        // 1630: ::core::mem::tr ... 0", ): typeof(_735) = & {l1005} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1630: ::core::mem::tr ... 0", ):   l1005 = UNIQUE | NON_NULL, FIXED
                                        // 1630: (*::core::mem:: ... ptr(): typeof(_734 = &(*_735)) = & {l2436} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1630: (*::core::mem:: ... ptr():   l2436 = UNIQUE | NON_NULL, (empty)
                                        // 1630: (*::core::mem:: ... ptr(): typeof(_733 = move _734 as &[i8] (Pointer(Unsize))) = & {l2437} [i8]
                                        // 1630: (*::core::mem:: ... ptr():   l2437 = UNIQUE | NON_NULL, FIXED
                                            b"void dd(void)\0",
                                            // 1631: b"void dd(void)\0": typeof(_736) = & {l1007} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1631: b"void dd(void)\0":   l1007 = UNIQUE | NON_NULL, (empty)
                                            // 1631: b"void dd(void)\0": typeof(_737) = & {l1009} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1631: b"void dd(void)\0":   l1009 = UNIQUE | NON_NULL, FIXED
                                            // 1631: b"void dd(void)\0": typeof(_737 = const b"void dd(void)\x00") = & {l2434} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1631: b"void dd(void)\0":   l2434 = UNIQUE | NON_NULL, (empty)
                                            // 1631: b"void dd(void)\0": typeof(_736 = &(*_737)) = & {l2435} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1631: b"void dd(void)\0":   l2435 = UNIQUE | NON_NULL, (empty)
                                        ))
                                        .as_ptr(),
                                    );
                                }
                            };
                        }
                        i += 1;
                        i;
                    }
                    res = run();
                    if res == golden {
                    // 1642: golden: typeof(_748) = *mut {l1021} i32
                    // 1642: golden:   l1021 = UNIQUE | NON_NULL, (empty)
                        if verbose > 1 as libc::c_int {
                        // 1643: verbose: typeof(_752) = *mut {l1026} i32
                        // 1643: verbose:   l1026 = UNIQUE | NON_NULL, (empty)
                            newline();
                            msg(
                                b"removed %d events\0" as *const u8 as *const libc::c_char,
                                // 1646: b"removed %d ev ... _char: typeof(_756) = *const {l1031} i8
                                // 1646: b"removed %d ev ... _char:   l1031 = UNIQUE | NON_NULL, (empty)
                                // 1646: b"removed %d ev ... st u8: typeof(_757) = *const {l1033} u8
                                // 1646: b"removed %d ev ... st u8:   l1033 = UNIQUE | NON_NULL, (empty)
                                // 1646: b"removed %d ev ... ts\0": typeof(_758) = *const {l1035} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1646: b"removed %d ev ... ts\0":   l1035 = UNIQUE | NON_NULL, (empty)
                                // 1646: b"removed %d ev ... ts\0": typeof(_759) = & {l1037} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1646: b"removed %d ev ... ts\0":   l1037 = UNIQUE | NON_NULL, FIXED
                                // 1646: b"removed %d ev ... st u8: typeof(_757 = move _758 as *const u8 (Pointer(ArrayToPointer))) = *const {l2440} u8
                                // 1646: b"removed %d ev ... st u8:   l2440 = UNIQUE | NON_NULL, (empty)
                                // 1646: b"removed %d ev ... ts\0": typeof(_758 = &raw const (*_759)) = *const {l2439} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1646: b"removed %d ev ... ts\0":   l2439 = UNIQUE | NON_NULL, (empty)
                                // 1646: b"removed %d ev ... _char: typeof(_756 = move _757 as *const i8 (Misc)) = *const {l2441} i8
                                // 1646: b"removed %d ev ... _char:   l2441 = UNIQUE | NON_NULL, (empty)
                                // 1646: b"removed %d ev ... ts\0": typeof(_759 = const b"removed %d events\x00") = & {l2438} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                // 1646: b"removed %d ev ... ts\0":   l2438 = UNIQUE | NON_NULL, (empty)
                                found,
                            );
                        }
                        changed = 1 as libc::c_int;
                    } else {
                        i = pos;
                        while i < nranges && i < pos + width {
                            r = ranges.offset(i as isize);
                            // 1654: ranges.offset(i ... size): typeof(_773) = *mut {l1052} DefId(0:370 ~ lglddtrace[7e63]::Range)
                            // 1654: ranges.offset(i ... size):   l1052 = UNIQUE | NON_NULL, (empty)
                            // 1654: ranges: typeof(_774) = *mut {l1054} DefId(0:370 ~ lglddtrace[7e63]::Range)
                            // 1654: ranges:   l1054 = UNIQUE | NON_NULL, (empty)
                            if !((*r).removed < runs - 1 as libc::c_int) {
                            // 1655: runs: typeof(_783) = *mut {l1064} i32
                            // 1655: runs:   l1064 = UNIQUE | NON_NULL, (empty)
                                if (*r).removed == runs - 1 as libc::c_int {
                                // 1656: runs: typeof(_791) = *mut {l1073} i32
                                // 1656: runs:   l1073 = UNIQUE | NON_NULL, (empty)
                                } else {
                                    __assert_fail(
                                        b"r->removed == runs - 1\0" as *const u8
                                        // 1659: b"r->removed == ... _char: typeof(_796) = *const {l1079} i8
                                        // 1659: b"r->removed == ... _char:   l1079 = UNIQUE | NON_NULL, (empty)
                                        // 1659: b"r->removed == ... st u8: typeof(_797) = *const {l1081} u8
                                        // 1659: b"r->removed == ... st u8:   l1081 = UNIQUE | NON_NULL, (empty)
                                        // 1659: b"r->removed == ...  1\0": typeof(_798) = *const {l1083} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                        // 1659: b"r->removed == ...  1\0":   l1083 = UNIQUE | NON_NULL, (empty)
                                        // 1659: b"r->removed == ...  1\0": typeof(_799) = & {l1085} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                        // 1659: b"r->removed == ...  1\0":   l1085 = UNIQUE | NON_NULL, FIXED
                                        // 1659: b"r->removed == ... _char: typeof(_796 = move _797 as *const i8 (Misc)) = *const {l2445} i8
                                        // 1659: b"r->removed == ... _char:   l2445 = UNIQUE | NON_NULL, (empty)
                                        // 1659: b"r->removed == ... st u8: typeof(_797 = move _798 as *const u8 (Pointer(ArrayToPointer))) = *const {l2444} u8
                                        // 1659: b"r->removed == ... st u8:   l2444 = UNIQUE | NON_NULL, (empty)
                                        // 1659: b"r->removed == ...  1\0": typeof(_798 = &raw const (*_799)) = *const {l2443} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                        // 1659: b"r->removed == ...  1\0":   l2443 = UNIQUE | NON_NULL, (empty)
                                        // 1659: b"r->removed == ...  1\0": typeof(_799 = const b"r->removed == runs - 1\x00") = & {l2442} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                        // 1659: b"r->removed == ...  1\0":   l2442 = UNIQUE | NON_NULL, (empty)
                                            as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        // 1661: b"lglddtrace.c\ ... _char: typeof(_800) = *const {l1087} i8
                                        // 1661: b"lglddtrace.c\ ... _char:   l1087 = UNIQUE | NON_NULL, (empty)
                                        // 1661: b"lglddtrace.c\ ... st u8: typeof(_801) = *const {l1089} u8
                                        // 1661: b"lglddtrace.c\ ... st u8:   l1089 = UNIQUE | NON_NULL, (empty)
                                        // 1661: b"lglddtrace.c\0": typeof(_802) = *const {l1091} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1661: b"lglddtrace.c\0":   l1091 = UNIQUE | NON_NULL, (empty)
                                        // 1661: b"lglddtrace.c\0": typeof(_803) = & {l1093} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1661: b"lglddtrace.c\0":   l1093 = UNIQUE | NON_NULL, FIXED
                                        // 1661: b"lglddtrace.c\0": typeof(_803 = const b"lglddtrace.c\x00") = & {l2446} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1661: b"lglddtrace.c\0":   l2446 = UNIQUE | NON_NULL, (empty)
                                        // 1661: b"lglddtrace.c\ ... _char: typeof(_800 = move _801 as *const i8 (Misc)) = *const {l2449} i8
                                        // 1661: b"lglddtrace.c\ ... _char:   l2449 = UNIQUE | NON_NULL, (empty)
                                        // 1661: b"lglddtrace.c\0": typeof(_802 = &raw const (*_803)) = *const {l2447} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1661: b"lglddtrace.c\0":   l2447 = UNIQUE | NON_NULL, (empty)
                                        // 1661: b"lglddtrace.c\ ... st u8: typeof(_801 = move _802 as *const u8 (Pointer(ArrayToPointer))) = *const {l2448} u8
                                        // 1661: b"lglddtrace.c\ ... st u8:   l2448 = UNIQUE | NON_NULL, (empty)
                                        594 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                        // 1663: (*::core::mem:: ... ptr(): typeof(_806) = *const {l1097} i8
                                        // 1663: (*::core::mem:: ... ptr():   l1097 = UNIQUE | NON_NULL, (empty)
                                        // 1663: (*::core::mem:: ... ptr(): typeof(_807) = & {l1099} [i8]
                                        // 1663: (*::core::mem:: ... ptr():   l1099 = UNIQUE | NON_NULL, FIXED
                                        // 1663: (*::core::mem:: ... ptr(): typeof(_808) = & {l1101} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1663: (*::core::mem:: ... ptr():   l1101 = UNIQUE | NON_NULL, (empty)
                                        // 1663: ::core::mem::tr ... 0", ): typeof(_809) = & {l1103} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1663: ::core::mem::tr ... 0", ):   l1103 = UNIQUE | NON_NULL, FIXED
                                        // 1663: (*::core::mem:: ... ptr(): typeof(_807 = move _808 as &[i8] (Pointer(Unsize))) = & {l2453} [i8]
                                        // 1663: (*::core::mem:: ... ptr():   l2453 = UNIQUE | NON_NULL, FIXED
                                        // 1663: (*::core::mem:: ... ptr(): typeof(_808 = &(*_809)) = & {l2452} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1663: (*::core::mem:: ... ptr():   l2452 = UNIQUE | NON_NULL, (empty)
                                            b"void dd(void)\0",
                                            // 1664: b"void dd(void)\0": typeof(_810) = & {l1105} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1664: b"void dd(void)\0":   l1105 = UNIQUE | NON_NULL, (empty)
                                            // 1664: b"void dd(void)\0": typeof(_811) = & {l1107} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1664: b"void dd(void)\0":   l1107 = UNIQUE | NON_NULL, FIXED
                                            // 1664: b"void dd(void)\0": typeof(_810 = &(*_811)) = & {l2451} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1664: b"void dd(void)\0":   l2451 = UNIQUE | NON_NULL, (empty)
                                            // 1664: b"void dd(void)\0": typeof(_811 = const b"void dd(void)\x00") = & {l2450} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1664: b"void dd(void)\0":   l2450 = UNIQUE | NON_NULL, (empty)
                                        ))
                                        .as_ptr(),
                                    );
                                }
                                'c_8732: {
                                    if (*r).removed == runs - 1 as libc::c_int {
                                    // 1670: runs: typeof(_817) = *mut {l1114} i32
                                    // 1670: runs:   l1114 = UNIQUE | NON_NULL, (empty)
                                    } else {
                                        __assert_fail(
                                            b"r->removed == runs - 1\0" as *const u8
                                            // 1673: b"r->removed == ... _char: typeof(_822) = *const {l1120} i8
                                            // 1673: b"r->removed == ... _char:   l1120 = UNIQUE | NON_NULL, (empty)
                                            // 1673: b"r->removed == ... st u8: typeof(_823) = *const {l1122} u8
                                            // 1673: b"r->removed == ... st u8:   l1122 = UNIQUE | NON_NULL, (empty)
                                            // 1673: b"r->removed == ...  1\0": typeof(_824) = *const {l1124} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                            // 1673: b"r->removed == ...  1\0":   l1124 = UNIQUE | NON_NULL, (empty)
                                            // 1673: b"r->removed == ...  1\0": typeof(_825) = & {l1126} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                            // 1673: b"r->removed == ...  1\0":   l1126 = UNIQUE | NON_NULL, FIXED
                                            // 1673: b"r->removed == ...  1\0": typeof(_824 = &raw const (*_825)) = *const {l2455} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                            // 1673: b"r->removed == ...  1\0":   l2455 = UNIQUE | NON_NULL, (empty)
                                            // 1673: b"r->removed == ...  1\0": typeof(_825 = const b"r->removed == runs - 1\x00") = & {l2454} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                            // 1673: b"r->removed == ...  1\0":   l2454 = UNIQUE | NON_NULL, (empty)
                                            // 1673: b"r->removed == ... _char: typeof(_822 = move _823 as *const i8 (Misc)) = *const {l2457} i8
                                            // 1673: b"r->removed == ... _char:   l2457 = UNIQUE | NON_NULL, (empty)
                                            // 1673: b"r->removed == ... st u8: typeof(_823 = move _824 as *const u8 (Pointer(ArrayToPointer))) = *const {l2456} u8
                                            // 1673: b"r->removed == ... st u8:   l2456 = UNIQUE | NON_NULL, (empty)
                                                as *const libc::c_char,
                                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                            // 1675: b"lglddtrace.c\ ... _char: typeof(_826) = *const {l1128} i8
                                            // 1675: b"lglddtrace.c\ ... _char:   l1128 = UNIQUE | NON_NULL, (empty)
                                            // 1675: b"lglddtrace.c\ ... st u8: typeof(_827) = *const {l1130} u8
                                            // 1675: b"lglddtrace.c\ ... st u8:   l1130 = UNIQUE | NON_NULL, (empty)
                                            // 1675: b"lglddtrace.c\0": typeof(_828) = *const {l1132} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1675: b"lglddtrace.c\0":   l1132 = UNIQUE | NON_NULL, (empty)
                                            // 1675: b"lglddtrace.c\0": typeof(_829) = & {l1134} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1675: b"lglddtrace.c\0":   l1134 = UNIQUE | NON_NULL, FIXED
                                            // 1675: b"lglddtrace.c\ ... st u8: typeof(_827 = move _828 as *const u8 (Pointer(ArrayToPointer))) = *const {l2460} u8
                                            // 1675: b"lglddtrace.c\ ... st u8:   l2460 = UNIQUE | NON_NULL, (empty)
                                            // 1675: b"lglddtrace.c\ ... _char: typeof(_826 = move _827 as *const i8 (Misc)) = *const {l2461} i8
                                            // 1675: b"lglddtrace.c\ ... _char:   l2461 = UNIQUE | NON_NULL, (empty)
                                            // 1675: b"lglddtrace.c\0": typeof(_829 = const b"lglddtrace.c\x00") = & {l2458} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1675: b"lglddtrace.c\0":   l2458 = UNIQUE | NON_NULL, (empty)
                                            // 1675: b"lglddtrace.c\0": typeof(_828 = &raw const (*_829)) = *const {l2459} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1675: b"lglddtrace.c\0":   l2459 = UNIQUE | NON_NULL, (empty)
                                            594 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                            // 1677: (*::core::mem:: ... ptr(): typeof(_832) = *const {l1138} i8
                                            // 1677: (*::core::mem:: ... ptr():   l1138 = UNIQUE | NON_NULL, (empty)
                                            // 1677: (*::core::mem:: ... ptr(): typeof(_833) = & {l1140} [i8]
                                            // 1677: (*::core::mem:: ... ptr():   l1140 = UNIQUE | NON_NULL, FIXED
                                            // 1677: (*::core::mem:: ... ptr(): typeof(_834) = & {l1142} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1677: (*::core::mem:: ... ptr():   l1142 = UNIQUE | NON_NULL, (empty)
                                            // 1677: ::core::mem::tr ... \0" ): typeof(_835) = & {l1144} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1677: ::core::mem::tr ... \0" ):   l1144 = UNIQUE | NON_NULL, FIXED
                                            // 1677: (*::core::mem:: ... ptr(): typeof(_833 = move _834 as &[i8] (Pointer(Unsize))) = & {l2465} [i8]
                                            // 1677: (*::core::mem:: ... ptr():   l2465 = UNIQUE | NON_NULL, FIXED
                                            // 1677: (*::core::mem:: ... ptr(): typeof(_834 = &(*_835)) = & {l2464} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1677: (*::core::mem:: ... ptr():   l2464 = UNIQUE | NON_NULL, (empty)
                                                &[u8; 14],
                                                &[libc::c_char; 14],
                                            >(
                                                b"void dd(void)\0"
                                                // 1681: b"void dd(void)\0": typeof(_836) = & {l1146} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1681: b"void dd(void)\0":   l1146 = UNIQUE | NON_NULL, (empty)
                                                // 1681: b"void dd(void)\0": typeof(_837) = & {l1148} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1681: b"void dd(void)\0":   l1148 = UNIQUE | NON_NULL, FIXED
                                                // 1681: b"void dd(void)\0": typeof(_837 = const b"void dd(void)\x00") = & {l2462} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1681: b"void dd(void)\0":   l2462 = UNIQUE | NON_NULL, (empty)
                                                // 1681: b"void dd(void)\0": typeof(_836 = &(*_837)) = & {l2463} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1681: b"void dd(void)\0":   l2463 = UNIQUE | NON_NULL, (empty)
                                            ))
                                            .as_ptr(),
                                        );
                                    }
                                };
                                (*r).removed = 2147483647 as libc::c_int;
                                j = (*r).from;
                                while j <= (*r).to {
                                    e = events.offset(j as isize);
                                    // 1690: events.offset(j ... size): typeof(_843) = *mut {l1155} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                    // 1690: events.offset(j ... size):   l1155 = UNIQUE | NON_NULL, (empty)
                                    // 1690: events: typeof(_844) = *mut {l1157} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                    // 1690: events:   l1157 = UNIQUE | NON_NULL, (empty)
                                    // 1690: events: typeof(_845) = *mut {l1159} *mut {l1160} DefId(0:354 ~ lglddtrace[7e63]::Event)
                                    // 1690: events:   l1159 = UNIQUE | NON_NULL, (empty)
                                    // 1690: events:   l1160 = UNIQUE | NON_NULL, (empty)
                                    if (*e).removed < runs {
                                    // 1691: runs: typeof(_852) = *mut {l1168} i32
                                    // 1691: runs:   l1168 = UNIQUE | NON_NULL, (empty)
                                    } else {
                                        __assert_fail(
                                            b"e->removed < runs\0" as *const u8
                                            // 1694: b"e->removed <  ... _char: typeof(_855) = *const {l1172} i8
                                            // 1694: b"e->removed <  ... _char:   l1172 = UNIQUE | NON_NULL, (empty)
                                            // 1694: b"e->removed <  ... st u8: typeof(_856) = *const {l1174} u8
                                            // 1694: b"e->removed <  ... st u8:   l1174 = UNIQUE | NON_NULL, (empty)
                                            // 1694: b"e->removed <  ... ns\0": typeof(_857) = *const {l1176} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                            // 1694: b"e->removed <  ... ns\0":   l1176 = UNIQUE | NON_NULL, (empty)
                                            // 1694: b"e->removed <  ... ns\0": typeof(_858) = & {l1178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                            // 1694: b"e->removed <  ... ns\0":   l1178 = UNIQUE | NON_NULL, FIXED
                                            // 1694: b"e->removed <  ... st u8: typeof(_856 = move _857 as *const u8 (Pointer(ArrayToPointer))) = *const {l2468} u8
                                            // 1694: b"e->removed <  ... st u8:   l2468 = UNIQUE | NON_NULL, (empty)
                                            // 1694: b"e->removed <  ... ns\0": typeof(_857 = &raw const (*_858)) = *const {l2467} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                            // 1694: b"e->removed <  ... ns\0":   l2467 = UNIQUE | NON_NULL, (empty)
                                            // 1694: b"e->removed <  ... _char: typeof(_855 = move _856 as *const i8 (Misc)) = *const {l2469} i8
                                            // 1694: b"e->removed <  ... _char:   l2469 = UNIQUE | NON_NULL, (empty)
                                            // 1694: b"e->removed <  ... ns\0": typeof(_858 = const b"e->removed < runs\x00") = & {l2466} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                            // 1694: b"e->removed <  ... ns\0":   l2466 = UNIQUE | NON_NULL, (empty)
                                                as *const libc::c_char,
                                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                            // 1696: b"lglddtrace.c\ ... _char: typeof(_859) = *const {l1180} i8
                                            // 1696: b"lglddtrace.c\ ... _char:   l1180 = UNIQUE | NON_NULL, (empty)
                                            // 1696: b"lglddtrace.c\ ... st u8: typeof(_860) = *const {l1182} u8
                                            // 1696: b"lglddtrace.c\ ... st u8:   l1182 = UNIQUE | NON_NULL, (empty)
                                            // 1696: b"lglddtrace.c\0": typeof(_861) = *const {l1184} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1696: b"lglddtrace.c\0":   l1184 = UNIQUE | NON_NULL, (empty)
                                            // 1696: b"lglddtrace.c\0": typeof(_862) = & {l1186} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1696: b"lglddtrace.c\0":   l1186 = UNIQUE | NON_NULL, FIXED
                                            // 1696: b"lglddtrace.c\ ... _char: typeof(_859 = move _860 as *const i8 (Misc)) = *const {l2473} i8
                                            // 1696: b"lglddtrace.c\ ... _char:   l2473 = UNIQUE | NON_NULL, (empty)
                                            // 1696: b"lglddtrace.c\0": typeof(_861 = &raw const (*_862)) = *const {l2471} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1696: b"lglddtrace.c\0":   l2471 = UNIQUE | NON_NULL, (empty)
                                            // 1696: b"lglddtrace.c\ ... st u8: typeof(_860 = move _861 as *const u8 (Pointer(ArrayToPointer))) = *const {l2472} u8
                                            // 1696: b"lglddtrace.c\ ... st u8:   l2472 = UNIQUE | NON_NULL, (empty)
                                            // 1696: b"lglddtrace.c\0": typeof(_862 = const b"lglddtrace.c\x00") = & {l2470} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                            // 1696: b"lglddtrace.c\0":   l2470 = UNIQUE | NON_NULL, (empty)
                                            598 as libc::c_int as libc::c_uint,
                                            (*::core::mem::transmute::<
                                            // 1698: (*::core::mem:: ... ptr(): typeof(_865) = *const {l1190} i8
                                            // 1698: (*::core::mem:: ... ptr():   l1190 = UNIQUE | NON_NULL, (empty)
                                            // 1698: (*::core::mem:: ... ptr(): typeof(_866) = & {l1192} [i8]
                                            // 1698: (*::core::mem:: ... ptr():   l1192 = UNIQUE | NON_NULL, FIXED
                                            // 1698: (*::core::mem:: ... ptr(): typeof(_867) = & {l1194} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1698: (*::core::mem:: ... ptr():   l1194 = UNIQUE | NON_NULL, (empty)
                                            // 1698: ::core::mem::tr ... \0" ): typeof(_868) = & {l1196} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1698: ::core::mem::tr ... \0" ):   l1196 = UNIQUE | NON_NULL, FIXED
                                            // 1698: (*::core::mem:: ... ptr(): typeof(_867 = &(*_868)) = & {l2476} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1698: (*::core::mem:: ... ptr():   l2476 = UNIQUE | NON_NULL, (empty)
                                            // 1698: (*::core::mem:: ... ptr(): typeof(_866 = move _867 as &[i8] (Pointer(Unsize))) = & {l2477} [i8]
                                            // 1698: (*::core::mem:: ... ptr():   l2477 = UNIQUE | NON_NULL, FIXED
                                                &[u8; 14],
                                                &[libc::c_char; 14],
                                            >(
                                                b"void dd(void)\0"
                                                // 1702: b"void dd(void)\0": typeof(_869) = & {l1198} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1702: b"void dd(void)\0":   l1198 = UNIQUE | NON_NULL, (empty)
                                                // 1702: b"void dd(void)\0": typeof(_870) = & {l1200} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1702: b"void dd(void)\0":   l1200 = UNIQUE | NON_NULL, FIXED
                                                // 1702: b"void dd(void)\0": typeof(_870 = const b"void dd(void)\x00") = & {l2474} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1702: b"void dd(void)\0":   l2474 = UNIQUE | NON_NULL, (empty)
                                                // 1702: b"void dd(void)\0": typeof(_869 = &(*_870)) = & {l2475} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1702: b"void dd(void)\0":   l2475 = UNIQUE | NON_NULL, (empty)
                                            ))
                                            .as_ptr(),
                                        );
                                    }
                                    'c_8662: {
                                        if (*e).removed < runs {
                                        // 1708: runs: typeof(_875) = *mut {l1206} i32
                                        // 1708: runs:   l1206 = UNIQUE | NON_NULL, (empty)
                                        } else {
                                            __assert_fail(
                                                b"e->removed < runs\0" as *const u8
                                                // 1711: b"e->removed <  ... _char: typeof(_878) = *const {l1210} i8
                                                // 1711: b"e->removed <  ... _char:   l1210 = UNIQUE | NON_NULL, (empty)
                                                // 1711: b"e->removed <  ... st u8: typeof(_879) = *const {l1212} u8
                                                // 1711: b"e->removed <  ... st u8:   l1212 = UNIQUE | NON_NULL, (empty)
                                                // 1711: b"e->removed <  ... ns\0": typeof(_880) = *const {l1214} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                                // 1711: b"e->removed <  ... ns\0":   l1214 = UNIQUE | NON_NULL, (empty)
                                                // 1711: b"e->removed <  ... ns\0": typeof(_881) = & {l1216} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                                // 1711: b"e->removed <  ... ns\0":   l1216 = UNIQUE | NON_NULL, FIXED
                                                // 1711: b"e->removed <  ... ns\0": typeof(_881 = const b"e->removed < runs\x00") = & {l2478} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                                // 1711: b"e->removed <  ... ns\0":   l2478 = UNIQUE | NON_NULL, (empty)
                                                // 1711: b"e->removed <  ... _char: typeof(_878 = move _879 as *const i8 (Misc)) = *const {l2481} i8
                                                // 1711: b"e->removed <  ... _char:   l2481 = UNIQUE | NON_NULL, (empty)
                                                // 1711: b"e->removed <  ... st u8: typeof(_879 = move _880 as *const u8 (Pointer(ArrayToPointer))) = *const {l2480} u8
                                                // 1711: b"e->removed <  ... st u8:   l2480 = UNIQUE | NON_NULL, (empty)
                                                // 1711: b"e->removed <  ... ns\0": typeof(_880 = &raw const (*_881)) = *const {l2479} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                                                // 1711: b"e->removed <  ... ns\0":   l2479 = UNIQUE | NON_NULL, (empty)
                                                    as *const libc::c_char,
                                                b"lglddtrace.c\0" as *const u8
                                                // 1713: b"lglddtrace.c\ ... _char: typeof(_882) = *const {l1218} i8
                                                // 1713: b"lglddtrace.c\ ... _char:   l1218 = UNIQUE | NON_NULL, (empty)
                                                // 1713: b"lglddtrace.c\ ... st u8: typeof(_883) = *const {l1220} u8
                                                // 1713: b"lglddtrace.c\ ... st u8:   l1220 = UNIQUE | NON_NULL, (empty)
                                                // 1713: b"lglddtrace.c\0": typeof(_884) = *const {l1222} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                                // 1713: b"lglddtrace.c\0":   l1222 = UNIQUE | NON_NULL, (empty)
                                                // 1713: b"lglddtrace.c\0": typeof(_885) = & {l1224} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                                // 1713: b"lglddtrace.c\0":   l1224 = UNIQUE | NON_NULL, FIXED
                                                // 1713: b"lglddtrace.c\ ... _char: typeof(_882 = move _883 as *const i8 (Misc)) = *const {l2485} i8
                                                // 1713: b"lglddtrace.c\ ... _char:   l2485 = UNIQUE | NON_NULL, (empty)
                                                // 1713: b"lglddtrace.c\0": typeof(_885 = const b"lglddtrace.c\x00") = & {l2482} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                                // 1713: b"lglddtrace.c\0":   l2482 = UNIQUE | NON_NULL, (empty)
                                                // 1713: b"lglddtrace.c\0": typeof(_884 = &raw const (*_885)) = *const {l2483} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                                // 1713: b"lglddtrace.c\0":   l2483 = UNIQUE | NON_NULL, (empty)
                                                // 1713: b"lglddtrace.c\ ... st u8: typeof(_883 = move _884 as *const u8 (Pointer(ArrayToPointer))) = *const {l2484} u8
                                                // 1713: b"lglddtrace.c\ ... st u8:   l2484 = UNIQUE | NON_NULL, (empty)
                                                    as *const libc::c_char,
                                                598 as libc::c_int as libc::c_uint,
                                                (*::core::mem::transmute::<
                                                // 1716: (*::core::mem:: ... ptr(): typeof(_888) = *const {l1228} i8
                                                // 1716: (*::core::mem:: ... ptr():   l1228 = UNIQUE | NON_NULL, (empty)
                                                // 1716: (*::core::mem:: ... ptr(): typeof(_889) = & {l1230} [i8]
                                                // 1716: (*::core::mem:: ... ptr():   l1230 = UNIQUE | NON_NULL, FIXED
                                                // 1716: (*::core::mem:: ... ptr(): typeof(_890) = & {l1232} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1716: (*::core::mem:: ... ptr():   l1232 = UNIQUE | NON_NULL, (empty)
                                                // 1716: ::core::mem::tr ... \0" ): typeof(_891) = & {l1234} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1716: ::core::mem::tr ... \0" ):   l1234 = UNIQUE | NON_NULL, FIXED
                                                // 1716: (*::core::mem:: ... ptr(): typeof(_889 = move _890 as &[i8] (Pointer(Unsize))) = & {l2489} [i8]
                                                // 1716: (*::core::mem:: ... ptr():   l2489 = UNIQUE | NON_NULL, FIXED
                                                // 1716: (*::core::mem:: ... ptr(): typeof(_890 = &(*_891)) = & {l2488} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                // 1716: (*::core::mem:: ... ptr():   l2488 = UNIQUE | NON_NULL, (empty)
                                                    &[u8; 14],
                                                    &[libc::c_char; 14],
                                                >(
                                                    b"void dd(void)\0"
                                                    // 1720: b"void dd(void)\0": typeof(_892) = & {l1236} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                    // 1720: b"void dd(void)\0":   l1236 = UNIQUE | NON_NULL, (empty)
                                                    // 1720: b"void dd(void)\0": typeof(_893) = & {l1238} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                    // 1720: b"void dd(void)\0":   l1238 = UNIQUE | NON_NULL, FIXED
                                                    // 1720: b"void dd(void)\0": typeof(_893 = const b"void dd(void)\x00") = & {l2486} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                    // 1720: b"void dd(void)\0":   l2486 = UNIQUE | NON_NULL, (empty)
                                                    // 1720: b"void dd(void)\0": typeof(_892 = &(*_893)) = & {l2487} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                                    // 1720: b"void dd(void)\0":   l2487 = UNIQUE | NON_NULL, (empty)
                                                ))
                                                .as_ptr(),
                                            );
                                        }
                                    };
                                    if (*e).removed == runs - 1 as libc::c_int {
                                    // 1726: runs: typeof(_899) = *mut {l1245} i32
                                    // 1726: runs:   l1245 = UNIQUE | NON_NULL, (empty)
                                        (*e).removed = 2147483647 as libc::c_int;
                                    }
                                    j += 1;
                                    j;
                                }
                            }
                            i += 1;
                            i;
                        }
                    }
                    pos += width;
                    if !(pos < nranges) {
                        break;
                    }
                }
                width = if width > 4 as libc::c_int {
                    width / 2 as libc::c_int
                } else {
                    width - 1 as libc::c_int
                };
            }
            if verbose > 1 as libc::c_int {
            // 1748: verbose: typeof(_939) = *mut {l1286} i32
            // 1748: verbose:   l1286 = UNIQUE | NON_NULL, (empty)
                newline();
            }
            rgran -= 1;
            rgran;
        }
        if ddopts != 0 {
        // 1754: ddopts: typeof(_950) = *mut {l1298} i32
        // 1754: ddopts:   l1298 = UNIQUE | NON_NULL, (empty)
            let mut reported: libc::c_int = 0 as libc::c_int;
            let mut o: *mut Opt = 0 as *mut Opt;
            // 1756: mut o: typeof(_952) = *mut {l1301} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1756: mut o:   l1301 = UNIQUE | NON_NULL, (empty)
            // 1756: 0 as *mut Opt: typeof(_952 = const 0_usize as *mut Opt (PointerFromExposedAddress)) = *mut {l2490} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1756: 0 as *mut Opt:   l2490 = UNIQUE | NON_NULL, (empty)
            if opts.is_null() {
            // 1757: opts: typeof(_955) = *mut {l1305} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1757: opts:   l1305 = UNIQUE | NON_NULL, (empty)
            // 1757: opts: typeof(_956) = *mut {l1307} *mut {l1308} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1757: opts:   l1307 = UNIQUE | NON_NULL, (empty)
            // 1757: opts:   l1308 = UNIQUE | NON_NULL, (empty)
                let mut it: *mut libc::c_void = 0 as *mut libc::c_void;
                // 1758: mut it: typeof(_957) = *mut {l1310} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1758: mut it:   l1310 = UNIQUE | NON_NULL, (empty)
                // 1758: 0 as *mut libc: ... _void: typeof(_957 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l2491} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1758: 0 as *mut libc: ... _void:   l2491 = UNIQUE | NON_NULL, (empty)
                let mut name: *const libc::c_char = 0 as *const libc::c_char;
                // 1759: mut name: typeof(_958) = *const {l1312} i8
                // 1759: mut name:   l1312 = UNIQUE | NON_NULL, (empty)
                // 1759: 0 as *const lib ... _char: typeof(_958 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l2492} i8
                // 1759: 0 as *const lib ... _char:   l2492 = UNIQUE | NON_NULL, (empty)
                let mut opt: Opt = Opt {
                    name: 0 as *mut libc::c_char,
                    // 1761: 0 as *mut libc: ... _char: typeof(_960) = *mut {l1315} i8
                    // 1761: 0 as *mut libc: ... _char:   l1315 = UNIQUE | NON_NULL, (empty)
                    // 1761: 0 as *mut libc: ... _char: typeof(_960 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l2493} i8
                    // 1761: 0 as *mut libc: ... _char:   l2493 = UNIQUE | NON_NULL, (empty)
                    val: 0,
                    min: 0,
                    max: 0,
                };
                let mut lgl: *mut LGL = lglinit();
                // 1766: mut lgl: typeof(_961) = *mut {l1317} LGL
                // 1766: mut lgl:   l1317 = UNIQUE | NON_NULL, (empty)
                it = lglfirstopt(lgl);
                // 1767: lglfirstopt(lgl): typeof(_962) = *mut {l1319} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1767: lglfirstopt(lgl):   l1319 = UNIQUE | NON_NULL, (empty)
                // 1767: lgl: typeof(_963) = *mut {l1321} LGL
                // 1767: lgl:   l1321 = UNIQUE | NON_NULL, (empty)
                loop {
                    it = lglnextopt(lgl, it, &mut name, &mut opt.val, &mut opt.min, &mut opt.max);
                    // 1769: lglnextopt(lgl, ... .max): typeof(_965) = *mut {l1324} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1769: lglnextopt(lgl, ... .max):   l1324 = UNIQUE | NON_NULL, (empty)
                    // 1769: lgl: typeof(_966) = *mut {l1326} LGL
                    // 1769: lgl:   l1326 = UNIQUE | NON_NULL, (empty)
                    // 1769: it: typeof(_967) = *mut {l1328} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1769: it:   l1328 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name: typeof(_968) = *mut {l1330} *const {l1331} i8
                    // 1769: &mut name:   l1330 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name:   l1331 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name: typeof(_969) = &mut {l1333} *const {l1334} i8
                    // 1769: &mut name:   l1333 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name:   l1334 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.val: typeof(_970) = *mut {l1336} i32
                    // 1769: &mut opt.val:   l1336 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.val: typeof(_971) = &mut {l1338} i32
                    // 1769: &mut opt.val:   l1338 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.min: typeof(_972) = *mut {l1340} i32
                    // 1769: &mut opt.min:   l1340 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.min: typeof(_973) = &mut {l1342} i32
                    // 1769: &mut opt.min:   l1342 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.max: typeof(_974) = *mut {l1344} i32
                    // 1769: &mut opt.max:   l1344 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.max: typeof(_975) = &mut {l1346} i32
                    // 1769: &mut opt.max:   l1346 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.max: typeof(_975 = &mut (_959.3: i32)) = &mut {l2502} i32
                    // 1769: &mut opt.max:   l2502 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.val: typeof(_971 = &mut (_959.1: i32)) = &mut {l2498} i32
                    // 1769: &mut opt.val:   l2498 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.val: typeof(_970 = &raw mut (*_971)) = *mut {l2499} i32
                    // 1769: &mut opt.val:   l2499 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.max: typeof(_974 = &raw mut (*_975)) = *mut {l2503} i32
                    // 1769: &mut opt.max:   l2503 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.min: typeof(_972 = &raw mut (*_973)) = *mut {l2501} i32
                    // 1769: &mut opt.min:   l2501 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name: typeof(_968 = &raw mut (*_969)) = *mut {l2496} *const {l2497} i8
                    // 1769: &mut name:   l2496 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name:   l2497 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name: typeof(_969 = &mut _958) = &mut {l2494} *const {l2495} i8
                    // 1769: &mut name:   l2494 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut name:   l2495 = UNIQUE | NON_NULL, (empty)
                    // 1769: &mut opt.min: typeof(_973 = &mut (_959.2: i32)) = &mut {l2500} i32
                    // 1769: &mut opt.min:   l2500 = UNIQUE | NON_NULL, (empty)
                    if it.is_null() {
                    // 1770: it: typeof(_978) = *mut {l1350} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1770: it:   l1350 = UNIQUE | NON_NULL, (empty)
                        break;
                    }
                    if strcmp(name, b"log\0" as *const u8 as *const libc::c_char) == 0 {
                    // 1773: name: typeof(_983) = *const {l1356} i8
                    // 1773: name:   l1356 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0" as *co ... _char: typeof(_984) = *const {l1358} i8
                    // 1773: b"log\0" as *co ... _char:   l1358 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0" as *co ... st u8: typeof(_985) = *const {l1360} u8
                    // 1773: b"log\0" as *co ... st u8:   l1360 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0": typeof(_986) = *const {l1362} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 1773: b"log\0":   l1362 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0": typeof(_987) = & {l1364} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 1773: b"log\0":   l1364 = UNIQUE | NON_NULL, FIXED
                    // 1773: b"log\0": typeof(_986 = &raw const (*_987)) = *const {l2505} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 1773: b"log\0":   l2505 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0": typeof(_987 = const b"log\x00") = & {l2504} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 1773: b"log\0":   l2504 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0" as *co ... st u8: typeof(_985 = move _986 as *const u8 (Pointer(ArrayToPointer))) = *const {l2506} u8
                    // 1773: b"log\0" as *co ... st u8:   l2506 = UNIQUE | NON_NULL, (empty)
                    // 1773: b"log\0" as *co ... _char: typeof(_984 = move _985 as *const i8 (Misc)) = *const {l2507} i8
                    // 1773: b"log\0" as *co ... _char:   l2507 = UNIQUE | NON_NULL, (empty)
                        continue;
                    }
                    if strcmp(name, b"check\0" as *const u8 as *const libc::c_char) == 0 {
                    // 1776: name: typeof(_992) = *const {l1370} i8
                    // 1776: name:   l1370 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0" as * ... _char: typeof(_993) = *const {l1372} i8
                    // 1776: b"check\0" as * ... _char:   l1372 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0" as * ... st u8: typeof(_994) = *const {l1374} u8
                    // 1776: b"check\0" as * ... st u8:   l1374 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0": typeof(_995) = *const {l1376} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 1776: b"check\0":   l1376 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0": typeof(_996) = & {l1378} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 1776: b"check\0":   l1378 = UNIQUE | NON_NULL, FIXED
                    // 1776: b"check\0": typeof(_995 = &raw const (*_996)) = *const {l2509} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 1776: b"check\0":   l2509 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0" as * ... st u8: typeof(_994 = move _995 as *const u8 (Pointer(ArrayToPointer))) = *const {l2510} u8
                    // 1776: b"check\0" as * ... st u8:   l2510 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0" as * ... _char: typeof(_993 = move _994 as *const i8 (Misc)) = *const {l2511} i8
                    // 1776: b"check\0" as * ... _char:   l2511 = UNIQUE | NON_NULL, (empty)
                    // 1776: b"check\0": typeof(_996 = const b"check\x00") = & {l2508} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 1776: b"check\0":   l2508 = UNIQUE | NON_NULL, (empty)
                        continue;
                    }
                    if strcmp(name, b"verbose\0" as *const u8 as *const libc::c_char) == 0 {
                    // 1779: name: typeof(_1001) = *const {l1384} i8
                    // 1779: name:   l1384 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0" as ... _char: typeof(_1002) = *const {l1386} i8
                    // 1779: b"verbose\0" as ... _char:   l1386 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0" as ... st u8: typeof(_1003) = *const {l1388} u8
                    // 1779: b"verbose\0" as ... st u8:   l1388 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0": typeof(_1004) = *const {l1390} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1779: b"verbose\0":   l1390 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0": typeof(_1005) = & {l1392} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1779: b"verbose\0":   l1392 = UNIQUE | NON_NULL, FIXED
                    // 1779: b"verbose\0": typeof(_1004 = &raw const (*_1005)) = *const {l2513} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1779: b"verbose\0":   l2513 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0": typeof(_1005 = const b"verbose\x00") = & {l2512} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1779: b"verbose\0":   l2512 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0" as ... st u8: typeof(_1003 = move _1004 as *const u8 (Pointer(ArrayToPointer))) = *const {l2514} u8
                    // 1779: b"verbose\0" as ... st u8:   l2514 = UNIQUE | NON_NULL, (empty)
                    // 1779: b"verbose\0" as ... _char: typeof(_1002 = move _1003 as *const i8 (Misc)) = *const {l2515} i8
                    // 1779: b"verbose\0" as ... _char:   l2515 = UNIQUE | NON_NULL, (empty)
                        continue;
                    }
                    if strcmp(name, b"witness\0" as *const u8 as *const libc::c_char) == 0 {
                    // 1782: name: typeof(_1010) = *const {l1398} i8
                    // 1782: name:   l1398 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0" as ... _char: typeof(_1011) = *const {l1400} i8
                    // 1782: b"witness\0" as ... _char:   l1400 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0" as ... st u8: typeof(_1012) = *const {l1402} u8
                    // 1782: b"witness\0" as ... st u8:   l1402 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0": typeof(_1013) = *const {l1404} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1782: b"witness\0":   l1404 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0": typeof(_1014) = & {l1406} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1782: b"witness\0":   l1406 = UNIQUE | NON_NULL, FIXED
                    // 1782: b"witness\0": typeof(_1013 = &raw const (*_1014)) = *const {l2517} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1782: b"witness\0":   l2517 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0": typeof(_1014 = const b"witness\x00") = & {l2516} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                    // 1782: b"witness\0":   l2516 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0" as ... _char: typeof(_1011 = move _1012 as *const i8 (Misc)) = *const {l2519} i8
                    // 1782: b"witness\0" as ... _char:   l2519 = UNIQUE | NON_NULL, (empty)
                    // 1782: b"witness\0" as ... st u8: typeof(_1012 = move _1013 as *const u8 (Pointer(ArrayToPointer))) = *const {l2518} u8
                    // 1782: b"witness\0" as ... st u8:   l2518 = UNIQUE | NON_NULL, (empty)
                        continue;
                    }
                    if strcmp(name, b"exitonabort\0" as *const u8 as *const libc::c_char) == 0 {
                    // 1785: name: typeof(_1019) = *const {l1412} i8
                    // 1785: name:   l1412 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0 ... _char: typeof(_1020) = *const {l1414} i8
                    // 1785: b"exitonabort\0 ... _char:   l1414 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0 ... st u8: typeof(_1021) = *const {l1416} u8
                    // 1785: b"exitonabort\0 ... st u8:   l1416 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0": typeof(_1022) = *const {l1418} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1785: b"exitonabort\0":   l1418 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0": typeof(_1023) = & {l1420} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1785: b"exitonabort\0":   l1420 = UNIQUE | NON_NULL, FIXED
                    // 1785: b"exitonabort\0": typeof(_1023 = const b"exitonabort\x00") = & {l2520} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1785: b"exitonabort\0":   l2520 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0 ... _char: typeof(_1020 = move _1021 as *const i8 (Misc)) = *const {l2523} i8
                    // 1785: b"exitonabort\0 ... _char:   l2523 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0 ... st u8: typeof(_1021 = move _1022 as *const u8 (Pointer(ArrayToPointer))) = *const {l2522} u8
                    // 1785: b"exitonabort\0 ... st u8:   l2522 = UNIQUE | NON_NULL, (empty)
                    // 1785: b"exitonabort\0": typeof(_1022 = &raw const (*_1023)) = *const {l2521} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                    // 1785: b"exitonabort\0":   l2521 = UNIQUE | NON_NULL, (empty)
                        continue;
                    }
                    if strcmp(name, b"sleeponabort\0" as *const u8 as *const libc::c_char) == 0 {
                    // 1788: name: typeof(_1028) = *const {l1426} i8
                    // 1788: name:   l1426 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\ ... _char: typeof(_1029) = *const {l1428} i8
                    // 1788: b"sleeponabort\ ... _char:   l1428 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\ ... st u8: typeof(_1030) = *const {l1430} u8
                    // 1788: b"sleeponabort\ ... st u8:   l1430 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\0": typeof(_1031) = *const {l1432} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1788: b"sleeponabort\0":   l1432 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\0": typeof(_1032) = & {l1434} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1788: b"sleeponabort\0":   l1434 = UNIQUE | NON_NULL, FIXED
                    // 1788: b"sleeponabort\ ... _char: typeof(_1029 = move _1030 as *const i8 (Misc)) = *const {l2527} i8
                    // 1788: b"sleeponabort\ ... _char:   l2527 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\0": typeof(_1032 = const b"sleeponabort\x00") = & {l2524} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1788: b"sleeponabort\0":   l2524 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\ ... st u8: typeof(_1030 = move _1031 as *const u8 (Pointer(ArrayToPointer))) = *const {l2526} u8
                    // 1788: b"sleeponabort\ ... st u8:   l2526 = UNIQUE | NON_NULL, (empty)
                    // 1788: b"sleeponabort\0": typeof(_1031 = &raw const (*_1032)) = *const {l2525} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 1788: b"sleeponabort\0":   l2525 = UNIQUE | NON_NULL, (empty)
                        continue;
                    }
                    if nopts == szopts {
                    // 1791: nopts: typeof(_1037) = *mut {l1440} i32
                    // 1791: nopts:   l1440 = UNIQUE | NON_NULL, (empty)
                    // 1791: szopts: typeof(_1039) = *mut {l1443} i32
                    // 1791: szopts:   l1443 = UNIQUE | NON_NULL, (empty)
                        szopts = if szopts != 0 {
                        // 1792: szopts: typeof(_1043) = *mut {l1448} i32
                        // 1792: szopts:   l1448 = UNIQUE | NON_NULL, (empty)
                        // 1792: szopts: typeof(_1048) = *mut {l1455} i32
                        // 1792: szopts:   l1455 = UNIQUE | NON_NULL, (empty)
                            2 as libc::c_int * szopts
                            // 1793: szopts: typeof(_1046) = *mut {l1452} i32
                            // 1793: szopts:   l1452 = UNIQUE | NON_NULL, (empty)
                        } else {
                            1 as libc::c_int
                        };
                        opts = realloc(
                        // 1797: realloc( opts a ... g), ): typeof(_1049) = *mut {l1457} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1797: realloc( opts a ... g), ):   l1457 = UNIQUE | NON_NULL, (empty)
                        // 1797: opts: typeof(_1059) = *mut {l1473} *mut {l1474} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1797: opts:   l1473 = UNIQUE | NON_NULL, (empty)
                        // 1797: opts:   l1474 = UNIQUE | NON_NULL, (empty)
                        // 1797: opts = realloc( ... t Opt: typeof((*_1059) = move _1049 as *mut Opt (Misc)) = *mut {l2529} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1797: opts = realloc( ... t Opt:   l2529 = UNIQUE | NON_NULL, (empty)
                            opts as *mut libc::c_void,
                            // 1798: opts as *mut li ... _void: typeof(_1050) = *mut {l1459} DefId(2:5583 ~ core[480a]::ffi::c_void)
                            // 1798: opts as *mut li ... _void:   l1459 = UNIQUE | NON_NULL, (empty)
                            // 1798: opts: typeof(_1051) = *mut {l1461} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 1798: opts:   l1461 = UNIQUE | NON_NULL, (empty)
                            // 1798: opts: typeof(_1052) = *mut {l1463} *mut {l1464} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 1798: opts:   l1463 = UNIQUE | NON_NULL, (empty)
                            // 1798: opts:   l1464 = UNIQUE | NON_NULL, (empty)
                            // 1798: opts as *mut li ... _void: typeof(_1050 = move _1051 as *mut libc::c_void (Misc)) = *mut {l2528} DefId(2:5583 ~ core[480a]::ffi::c_void)
                            // 1798: opts as *mut li ... _void:   l2528 = UNIQUE | NON_NULL, (empty)
                            (szopts as libc::c_ulong)
                            // 1799: szopts: typeof(_1056) = *mut {l1469} i32
                            // 1799: szopts:   l1469 = UNIQUE | NON_NULL, (empty)
                                .wrapping_mul(::core::mem::size_of::<Opt>() as libc::c_ulong),
                        ) as *mut Opt;
                    }
                    opt.name = strdup(name);
                    // 1803: strdup(name): typeof(_1060) = *mut {l1476} i8
                    // 1803: strdup(name):   l1476 = UNIQUE | NON_NULL, (empty)
                    // 1803: name: typeof(_1061) = *const {l1478} i8
                    // 1803: name:   l1478 = UNIQUE | NON_NULL, (empty)
                    let fresh3 = nopts;
                    // 1804: nopts: typeof(_1063) = *mut {l1481} i32
                    // 1804: nopts:   l1481 = UNIQUE | NON_NULL, (empty)
                    nopts = nopts + 1;
                    // 1805: nopts: typeof(_1065) = *mut {l1484} i32
                    // 1805: nopts:   l1484 = UNIQUE | NON_NULL, (empty)
                    // 1805: nopts: typeof(_1067) = *mut {l1487} i32
                    // 1805: nopts:   l1487 = UNIQUE | NON_NULL, (empty)
                    *opts.offset(fresh3 as isize) = opt;
                    // 1806: opts.offset(fre ... size): typeof(_1069) = *mut {l1490} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1806: opts.offset(fre ... size):   l1490 = UNIQUE | NON_NULL, (empty)
                    // 1806: opts: typeof(_1070) = *mut {l1492} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1806: opts:   l1492 = UNIQUE | NON_NULL, (empty)
                    // 1806: opts: typeof(_1071) = *mut {l1494} *mut {l1495} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1806: opts:   l1494 = UNIQUE | NON_NULL, (empty)
                    // 1806: opts:   l1495 = UNIQUE | NON_NULL, (empty)
                }
                e = events;
                // 1808: events: typeof(_1074) = *mut {l1499} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1808: events:   l1499 = UNIQUE | NON_NULL, (empty)
                // 1808: events: typeof(_1075) = *mut {l1501} *mut {l1502} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1808: events:   l1501 = UNIQUE | NON_NULL, (empty)
                // 1808: events:   l1502 = UNIQUE | NON_NULL, (empty)
                while e < events.offset(nevents as isize) {
                // 1809: e: typeof(_1078) = *mut {l1506} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1809: e:   l1506 = UNIQUE | NON_NULL, (empty)
                // 1809: events.offset(n ... size): typeof(_1079) = *mut {l1508} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1809: events.offset(n ... size):   l1508 = UNIQUE | NON_NULL, (empty)
                // 1809: events: typeof(_1080) = *mut {l1510} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1809: events:   l1510 = UNIQUE | NON_NULL, (empty)
                // 1809: events: typeof(_1081) = *mut {l1512} *mut {l1513} DefId(0:354 ~ lglddtrace[7e63]::Event)
                // 1809: events:   l1512 = UNIQUE | NON_NULL, (empty)
                // 1809: events:   l1513 = UNIQUE | NON_NULL, (empty)
                // 1809: nevents: typeof(_1084) = *mut {l1517} i32
                // 1809: nevents:   l1517 = UNIQUE | NON_NULL, (empty)
                    if !((*e).type_0 as libc::c_uint != OPTION as libc::c_int as libc::c_uint) {
                        o = opts;
                        // 1811: opts: typeof(_1091) = *mut {l1525} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1811: opts:   l1525 = UNIQUE | NON_NULL, (empty)
                        // 1811: opts: typeof(_1092) = *mut {l1527} *mut {l1528} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1811: opts:   l1527 = UNIQUE | NON_NULL, (empty)
                        // 1811: opts:   l1528 = UNIQUE | NON_NULL, (empty)
                        while o < opts.offset(nopts as isize) {
                        // 1812: o: typeof(_1094) = *mut {l1531} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1812: o:   l1531 = UNIQUE | NON_NULL, (empty)
                        // 1812: opts.offset(nop ... size): typeof(_1095) = *mut {l1533} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1812: opts.offset(nop ... size):   l1533 = UNIQUE | NON_NULL, (empty)
                        // 1812: opts: typeof(_1096) = *mut {l1535} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1812: opts:   l1535 = UNIQUE | NON_NULL, (empty)
                        // 1812: opts: typeof(_1097) = *mut {l1537} *mut {l1538} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                        // 1812: opts:   l1537 = UNIQUE | NON_NULL, (empty)
                        // 1812: opts:   l1538 = UNIQUE | NON_NULL, (empty)
                        // 1812: nopts: typeof(_1100) = *mut {l1542} i32
                        // 1812: nopts:   l1542 = UNIQUE | NON_NULL, (empty)
                            if strcmp((*e).opt, (*o).name) == 0 {
                            // 1813: (*e).opt: typeof(_1104) = *const {l1547} i8
                            // 1813: (*e).opt:   l1547 = UNIQUE | NON_NULL, (empty)
                            // 1813: (*e).opt: typeof(_1105) = *mut {l1549} i8
                            // 1813: (*e).opt:   l1549 = UNIQUE | NON_NULL, (empty)
                            // 1813: (*o).name: typeof(_1106) = *const {l1551} i8
                            // 1813: (*o).name:   l1551 = UNIQUE | NON_NULL, (empty)
                            // 1813: (*o).name: typeof(_1107) = *mut {l1553} i8
                            // 1813: (*o).name:   l1553 = UNIQUE | NON_NULL, (empty)
                            // 1813: (*e).opt: typeof(_1104 = move _1105 as *const i8 (Pointer(MutToConstPointer))) = *const {l2530} i8
                            // 1813: (*e).opt:   l2530 = UNIQUE | NON_NULL, (empty)
                            // 1813: (*o).name: typeof(_1106 = move _1107 as *const i8 (Pointer(MutToConstPointer))) = *const {l2531} i8
                            // 1813: (*o).name:   l2531 = UNIQUE | NON_NULL, (empty)
                                (*o).val = (*e).arg;
                                sumoptvals +=
                                // 1815: sumoptvals: typeof(_1115) = *mut {l1562} i64
                                // 1815: sumoptvals:   l1562 = UNIQUE | NON_NULL, (empty)
                                    (*o).val as libc::c_longlong - (*o).min as libc::c_longlong;
                            }
                            o = o.offset(1);
                            // 1818: o.offset(1): typeof(_1117) = *mut {l1565} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 1818: o.offset(1):   l1565 = UNIQUE | NON_NULL, (empty)
                            // 1818: o: typeof(_1118) = *mut {l1567} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 1818: o:   l1567 = UNIQUE | NON_NULL, (empty)
                            o;
                            // 1819: o: typeof(_1119) = *mut {l1569} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                            // 1819: o:   l1569 = UNIQUE | NON_NULL, (empty)
                        }
                    }
                    e = e.offset(1);
                    // 1822: e.offset(1): typeof(_1123) = *mut {l1574} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1822: e.offset(1):   l1574 = UNIQUE | NON_NULL, (empty)
                    // 1822: e: typeof(_1124) = *mut {l1576} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1822: e:   l1576 = UNIQUE | NON_NULL, (empty)
                    e;
                    // 1823: e: typeof(_1125) = *mut {l1578} DefId(0:354 ~ lglddtrace[7e63]::Event)
                    // 1823: e:   l1578 = UNIQUE | NON_NULL, (empty)
                }
                lglrelease(lgl);
                // 1825: lgl: typeof(_1130) = *mut {l1584} LGL
                // 1825: lgl:   l1584 = UNIQUE | NON_NULL, (empty)
            }
            o = opts;
            // 1827: opts: typeof(_1131) = *mut {l1586} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1827: opts:   l1586 = UNIQUE | NON_NULL, (empty)
            // 1827: opts: typeof(_1132) = *mut {l1588} *mut {l1589} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1827: opts:   l1588 = UNIQUE | NON_NULL, (empty)
            // 1827: opts:   l1589 = UNIQUE | NON_NULL, (empty)
            while o < opts.offset(nopts as isize) {
            // 1828: o: typeof(_1135) = *mut {l1593} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1828: o:   l1593 = UNIQUE | NON_NULL, (empty)
            // 1828: opts.offset(nop ... size): typeof(_1136) = *mut {l1595} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1828: opts.offset(nop ... size):   l1595 = UNIQUE | NON_NULL, (empty)
            // 1828: opts: typeof(_1137) = *mut {l1597} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1828: opts:   l1597 = UNIQUE | NON_NULL, (empty)
            // 1828: opts: typeof(_1138) = *mut {l1599} *mut {l1600} DefId(0:362 ~ lglddtrace[7e63]::Opt)
            // 1828: opts:   l1599 = UNIQUE | NON_NULL, (empty)
            // 1828: opts:   l1600 = UNIQUE | NON_NULL, (empty)
            // 1828: nopts: typeof(_1141) = *mut {l1604} i32
            // 1828: nopts:   l1604 = UNIQUE | NON_NULL, (empty)
                let mut delta: libc::c_longlong =
                    (*o).val as libc::c_longlong - (*o).min as libc::c_longlong;
                rep(
                    b"o %d / %d %lld                            \0" as *const u8
                    // 1832: b"o %d / %d %ll ... _char: typeof(_1149) = *const {l1613} i8
                    // 1832: b"o %d / %d %ll ... _char:   l1613 = UNIQUE | NON_NULL, (empty)
                    // 1832: b"o %d / %d %ll ... st u8: typeof(_1150) = *const {l1615} u8
                    // 1832: b"o %d / %d %ll ... st u8:   l1615 = UNIQUE | NON_NULL, (empty)
                    // 1832: b"o %d / %d %lld \0": typeof(_1151) = *const {l1617} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1832: b"o %d / %d %lld \0":   l1617 = UNIQUE | NON_NULL, (empty)
                    // 1832: b"o %d / %d %lld \0": typeof(_1152) = & {l1619} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1832: b"o %d / %d %lld \0":   l1619 = UNIQUE | NON_NULL, FIXED
                    // 1832: b"o %d / %d %ll ... _char: typeof(_1149 = move _1150 as *const i8 (Misc)) = *const {l2535} i8
                    // 1832: b"o %d / %d %ll ... _char:   l2535 = UNIQUE | NON_NULL, (empty)
                    // 1832: b"o %d / %d %lld \0": typeof(_1152 = const b"o %d / %d %lld                            \x00") = & {l2532} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1832: b"o %d / %d %lld \0":   l2532 = UNIQUE | NON_NULL, (empty)
                    // 1832: b"o %d / %d %lld \0": typeof(_1151 = &raw const (*_1152)) = *const {l2533} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                    // 1832: b"o %d / %d %lld \0":   l2533 = UNIQUE | NON_NULL, (empty)
                    // 1832: b"o %d / %d %ll ... st u8: typeof(_1150 = move _1151 as *const u8 (Pointer(ArrayToPointer))) = *const {l2534} u8
                    // 1832: b"o %d / %d %ll ... st u8:   l2534 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    o.offset_from(opts) as libc::c_long,
                    // 1834: o: typeof(_1155) = *mut {l1623} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1834: o:   l1623 = UNIQUE | NON_NULL, (empty)
                    // 1834: opts: typeof(_1156) = *const {l1625} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1834: opts:   l1625 = UNIQUE | NON_NULL, (empty)
                    // 1834: opts: typeof(_1157) = *mut {l1627} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1834: opts:   l1627 = UNIQUE | NON_NULL, (empty)
                    // 1834: opts: typeof(_1158) = *mut {l1629} *mut {l1630} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1834: opts:   l1629 = UNIQUE | NON_NULL, (empty)
                    // 1834: opts:   l1630 = UNIQUE | NON_NULL, (empty)
                    // 1834: opts: typeof(_1156 = move _1157 as *const Opt (Pointer(MutToConstPointer))) = *const {l2536} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                    // 1834: opts:   l2536 = UNIQUE | NON_NULL, (empty)
                    nopts,
                    // 1835: nopts: typeof(_1160) = *mut {l1633} i32
                    // 1835: nopts:   l1633 = UNIQUE | NON_NULL, (empty)
                    sumoptvals,
                    // 1836: sumoptvals: typeof(_1162) = *mut {l1636} i64
                    // 1836: sumoptvals:   l1636 = UNIQUE | NON_NULL, (empty)
                );
                reported += 1;
                reported;
                if (*o).val > (*o).min {
                    let mut oldval: libc::c_int = (*o).val;
                    (*o).val = (*o).min;
                    res = run();
                    if res == golden {
                    // 1844: golden: typeof(_1175) = *mut {l1650} i32
                    // 1844: golden:   l1650 = UNIQUE | NON_NULL, (empty)
                        if verbose > 1 as libc::c_int {
                        // 1845: verbose: typeof(_1179) = *mut {l1655} i32
                        // 1845: verbose:   l1655 = UNIQUE | NON_NULL, (empty)
                            if reported != 0 {
                                newline();
                            }
                            msg(
                                b"reduced option %s from %d to %d by one\0" as *const u8
                                // 1850: b"reduced optio ... _char: typeof(_1186) = *const {l1663} i8
                                // 1850: b"reduced optio ... _char:   l1663 = UNIQUE | NON_NULL, (empty)
                                // 1850: b"reduced optio ... st u8: typeof(_1187) = *const {l1665} u8
                                // 1850: b"reduced optio ... st u8:   l1665 = UNIQUE | NON_NULL, (empty)
                                // 1850: b"reduced optio ... ne\0": typeof(_1188) = *const {l1667} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                // 1850: b"reduced optio ... ne\0":   l1667 = UNIQUE | NON_NULL, (empty)
                                // 1850: b"reduced optio ... ne\0": typeof(_1189) = & {l1669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                // 1850: b"reduced optio ... ne\0":   l1669 = UNIQUE | NON_NULL, FIXED
                                // 1850: b"reduced optio ... _char: typeof(_1186 = move _1187 as *const i8 (Misc)) = *const {l2540} i8
                                // 1850: b"reduced optio ... _char:   l2540 = UNIQUE | NON_NULL, (empty)
                                // 1850: b"reduced optio ... st u8: typeof(_1187 = move _1188 as *const u8 (Pointer(ArrayToPointer))) = *const {l2539} u8
                                // 1850: b"reduced optio ... st u8:   l2539 = UNIQUE | NON_NULL, (empty)
                                // 1850: b"reduced optio ... ne\0": typeof(_1189 = const b"reduced option %s from %d to %d by one\x00") = & {l2537} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                // 1850: b"reduced optio ... ne\0":   l2537 = UNIQUE | NON_NULL, (empty)
                                // 1850: b"reduced optio ... ne\0": typeof(_1188 = &raw const (*_1189)) = *const {l2538} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                // 1850: b"reduced optio ... ne\0":   l2538 = UNIQUE | NON_NULL, (empty)
                                    as *const libc::c_char,
                                (*o).name,
                                // 1852: (*o).name: typeof(_1190) = *mut {l1671} i8
                                // 1852: (*o).name:   l1671 = UNIQUE | NON_NULL, (empty)
                                oldval,
                                (*o).val,
                            );
                            reported = 0 as libc::c_int;
                        }
                        changed = 1 as libc::c_int;
                        sumoptvals -= 1;
                        // 1859: sumoptvals: typeof(_1195) = *mut {l1677} i64
                        // 1859: sumoptvals:   l1677 = UNIQUE | NON_NULL, (empty)
                        sumoptvals;
                        // 1860: sumoptvals: typeof(_1198) = *mut {l1681} i64
                        // 1860: sumoptvals:   l1681 = UNIQUE | NON_NULL, (empty)
                    } else {
                        (*o).val = oldval;
                    }
                }
                if delta < 10 as libc::c_int as libc::c_longlong {
                    while (*o).val > (*o).min {
                        let mut oldval_0: libc::c_int = (*o).val;
                        let mut newval: libc::c_int = oldval_0 - 1 as libc::c_int;
                        if newval >= (*o).min {
                        } else {
                            __assert_fail(
                                b"newval >= o->min\0" as *const u8 as *const libc::c_char,
                                // 1872: b"newval >= o-> ... _char: typeof(_1219) = *const {l1703} i8
                                // 1872: b"newval >= o-> ... _char:   l1703 = UNIQUE | NON_NULL, (empty)
                                // 1872: b"newval >= o-> ... st u8: typeof(_1220) = *const {l1705} u8
                                // 1872: b"newval >= o-> ... st u8:   l1705 = UNIQUE | NON_NULL, (empty)
                                // 1872: b"newval >= o-> ... in\0": typeof(_1221) = *const {l1707} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                // 1872: b"newval >= o-> ... in\0":   l1707 = UNIQUE | NON_NULL, (empty)
                                // 1872: b"newval >= o-> ... in\0": typeof(_1222) = & {l1709} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                // 1872: b"newval >= o-> ... in\0":   l1709 = UNIQUE | NON_NULL, FIXED
                                // 1872: b"newval >= o-> ... _char: typeof(_1219 = move _1220 as *const i8 (Misc)) = *const {l2544} i8
                                // 1872: b"newval >= o-> ... _char:   l2544 = UNIQUE | NON_NULL, (empty)
                                // 1872: b"newval >= o-> ... st u8: typeof(_1220 = move _1221 as *const u8 (Pointer(ArrayToPointer))) = *const {l2543} u8
                                // 1872: b"newval >= o-> ... st u8:   l2543 = UNIQUE | NON_NULL, (empty)
                                // 1872: b"newval >= o-> ... in\0": typeof(_1222 = const b"newval >= o->min\x00") = & {l2541} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                // 1872: b"newval >= o-> ... in\0":   l2541 = UNIQUE | NON_NULL, (empty)
                                // 1872: b"newval >= o-> ... in\0": typeof(_1221 = &raw const (*_1222)) = *const {l2542} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                // 1872: b"newval >= o-> ... in\0":   l2542 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 1873: b"lglddtrace.c\ ... _char: typeof(_1223) = *const {l1711} i8
                                // 1873: b"lglddtrace.c\ ... _char:   l1711 = UNIQUE | NON_NULL, (empty)
                                // 1873: b"lglddtrace.c\ ... st u8: typeof(_1224) = *const {l1713} u8
                                // 1873: b"lglddtrace.c\ ... st u8:   l1713 = UNIQUE | NON_NULL, (empty)
                                // 1873: b"lglddtrace.c\0": typeof(_1225) = *const {l1715} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1873: b"lglddtrace.c\0":   l1715 = UNIQUE | NON_NULL, (empty)
                                // 1873: b"lglddtrace.c\0": typeof(_1226) = & {l1717} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1873: b"lglddtrace.c\0":   l1717 = UNIQUE | NON_NULL, FIXED
                                // 1873: b"lglddtrace.c\ ... _char: typeof(_1223 = move _1224 as *const i8 (Misc)) = *const {l2548} i8
                                // 1873: b"lglddtrace.c\ ... _char:   l2548 = UNIQUE | NON_NULL, (empty)
                                // 1873: b"lglddtrace.c\0": typeof(_1226 = const b"lglddtrace.c\x00") = & {l2545} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1873: b"lglddtrace.c\0":   l2545 = UNIQUE | NON_NULL, (empty)
                                // 1873: b"lglddtrace.c\ ... st u8: typeof(_1224 = move _1225 as *const u8 (Pointer(ArrayToPointer))) = *const {l2547} u8
                                // 1873: b"lglddtrace.c\ ... st u8:   l2547 = UNIQUE | NON_NULL, (empty)
                                // 1873: b"lglddtrace.c\0": typeof(_1225 = &raw const (*_1226)) = *const {l2546} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1873: b"lglddtrace.c\0":   l2546 = UNIQUE | NON_NULL, (empty)
                                666 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                // 1875: (*::core::mem:: ... ptr(): typeof(_1229) = *const {l1721} i8
                                // 1875: (*::core::mem:: ... ptr():   l1721 = UNIQUE | NON_NULL, (empty)
                                // 1875: (*::core::mem:: ... ptr(): typeof(_1230) = & {l1723} [i8]
                                // 1875: (*::core::mem:: ... ptr():   l1723 = UNIQUE | NON_NULL, FIXED
                                // 1875: (*::core::mem:: ... ptr(): typeof(_1231) = & {l1725} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1875: (*::core::mem:: ... ptr():   l1725 = UNIQUE | NON_NULL, (empty)
                                // 1875: ::core::mem::tr ... 0", ): typeof(_1232) = & {l1727} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1875: ::core::mem::tr ... 0", ):   l1727 = UNIQUE | NON_NULL, FIXED
                                // 1875: (*::core::mem:: ... ptr(): typeof(_1231 = &(*_1232)) = & {l2551} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1875: (*::core::mem:: ... ptr():   l2551 = UNIQUE | NON_NULL, (empty)
                                // 1875: (*::core::mem:: ... ptr(): typeof(_1230 = move _1231 as &[i8] (Pointer(Unsize))) = & {l2552} [i8]
                                // 1875: (*::core::mem:: ... ptr():   l2552 = UNIQUE | NON_NULL, FIXED
                                    b"void dd(void)\0",
                                    // 1876: b"void dd(void)\0": typeof(_1233) = & {l1729} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1876: b"void dd(void)\0":   l1729 = UNIQUE | NON_NULL, (empty)
                                    // 1876: b"void dd(void)\0": typeof(_1234) = & {l1731} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1876: b"void dd(void)\0":   l1731 = UNIQUE | NON_NULL, FIXED
                                    // 1876: b"void dd(void)\0": typeof(_1233 = &(*_1234)) = & {l2550} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1876: b"void dd(void)\0":   l2550 = UNIQUE | NON_NULL, (empty)
                                    // 1876: b"void dd(void)\0": typeof(_1234 = const b"void dd(void)\x00") = & {l2549} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1876: b"void dd(void)\0":   l2549 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                        'c_8135: {
                            if newval >= (*o).min {
                            } else {
                                __assert_fail(
                                    b"newval >= o->min\0" as *const u8 as *const libc::c_char,
                                    // 1885: b"newval >= o-> ... _char: typeof(_1241) = *const {l1739} i8
                                    // 1885: b"newval >= o-> ... _char:   l1739 = UNIQUE | NON_NULL, (empty)
                                    // 1885: b"newval >= o-> ... st u8: typeof(_1242) = *const {l1741} u8
                                    // 1885: b"newval >= o-> ... st u8:   l1741 = UNIQUE | NON_NULL, (empty)
                                    // 1885: b"newval >= o-> ... in\0": typeof(_1243) = *const {l1743} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 1885: b"newval >= o-> ... in\0":   l1743 = UNIQUE | NON_NULL, (empty)
                                    // 1885: b"newval >= o-> ... in\0": typeof(_1244) = & {l1745} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 1885: b"newval >= o-> ... in\0":   l1745 = UNIQUE | NON_NULL, FIXED
                                    // 1885: b"newval >= o-> ... in\0": typeof(_1244 = const b"newval >= o->min\x00") = & {l2553} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 1885: b"newval >= o-> ... in\0":   l2553 = UNIQUE | NON_NULL, (empty)
                                    // 1885: b"newval >= o-> ... _char: typeof(_1241 = move _1242 as *const i8 (Misc)) = *const {l2556} i8
                                    // 1885: b"newval >= o-> ... _char:   l2556 = UNIQUE | NON_NULL, (empty)
                                    // 1885: b"newval >= o-> ... in\0": typeof(_1243 = &raw const (*_1244)) = *const {l2554} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 1885: b"newval >= o-> ... in\0":   l2554 = UNIQUE | NON_NULL, (empty)
                                    // 1885: b"newval >= o-> ... st u8: typeof(_1242 = move _1243 as *const u8 (Pointer(ArrayToPointer))) = *const {l2555} u8
                                    // 1885: b"newval >= o-> ... st u8:   l2555 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 1886: b"lglddtrace.c\ ... _char: typeof(_1245) = *const {l1747} i8
                                    // 1886: b"lglddtrace.c\ ... _char:   l1747 = UNIQUE | NON_NULL, (empty)
                                    // 1886: b"lglddtrace.c\ ... st u8: typeof(_1246) = *const {l1749} u8
                                    // 1886: b"lglddtrace.c\ ... st u8:   l1749 = UNIQUE | NON_NULL, (empty)
                                    // 1886: b"lglddtrace.c\0": typeof(_1247) = *const {l1751} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1886: b"lglddtrace.c\0":   l1751 = UNIQUE | NON_NULL, (empty)
                                    // 1886: b"lglddtrace.c\0": typeof(_1248) = & {l1753} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1886: b"lglddtrace.c\0":   l1753 = UNIQUE | NON_NULL, FIXED
                                    // 1886: b"lglddtrace.c\0": typeof(_1247 = &raw const (*_1248)) = *const {l2558} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1886: b"lglddtrace.c\0":   l2558 = UNIQUE | NON_NULL, (empty)
                                    // 1886: b"lglddtrace.c\ ... st u8: typeof(_1246 = move _1247 as *const u8 (Pointer(ArrayToPointer))) = *const {l2559} u8
                                    // 1886: b"lglddtrace.c\ ... st u8:   l2559 = UNIQUE | NON_NULL, (empty)
                                    // 1886: b"lglddtrace.c\0": typeof(_1248 = const b"lglddtrace.c\x00") = & {l2557} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1886: b"lglddtrace.c\0":   l2557 = UNIQUE | NON_NULL, (empty)
                                    // 1886: b"lglddtrace.c\ ... _char: typeof(_1245 = move _1246 as *const i8 (Misc)) = *const {l2560} i8
                                    // 1886: b"lglddtrace.c\ ... _char:   l2560 = UNIQUE | NON_NULL, (empty)
                                    666 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    // 1888: (*::core::mem:: ... ptr(): typeof(_1251) = *const {l1757} i8
                                    // 1888: (*::core::mem:: ... ptr():   l1757 = UNIQUE | NON_NULL, (empty)
                                    // 1888: (*::core::mem:: ... ptr(): typeof(_1252) = & {l1759} [i8]
                                    // 1888: (*::core::mem:: ... ptr():   l1759 = UNIQUE | NON_NULL, FIXED
                                    // 1888: (*::core::mem:: ... ptr(): typeof(_1253) = & {l1761} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1888: (*::core::mem:: ... ptr():   l1761 = UNIQUE | NON_NULL, (empty)
                                    // 1888: ::core::mem::tr ... 0", ): typeof(_1254) = & {l1763} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1888: ::core::mem::tr ... 0", ):   l1763 = UNIQUE | NON_NULL, FIXED
                                    // 1888: (*::core::mem:: ... ptr(): typeof(_1252 = move _1253 as &[i8] (Pointer(Unsize))) = & {l2564} [i8]
                                    // 1888: (*::core::mem:: ... ptr():   l2564 = UNIQUE | NON_NULL, FIXED
                                    // 1888: (*::core::mem:: ... ptr(): typeof(_1253 = &(*_1254)) = & {l2563} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1888: (*::core::mem:: ... ptr():   l2563 = UNIQUE | NON_NULL, (empty)
                                        b"void dd(void)\0",
                                        // 1889: b"void dd(void)\0": typeof(_1255) = & {l1765} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1889: b"void dd(void)\0":   l1765 = UNIQUE | NON_NULL, (empty)
                                        // 1889: b"void dd(void)\0": typeof(_1256) = & {l1767} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1889: b"void dd(void)\0":   l1767 = UNIQUE | NON_NULL, FIXED
                                        // 1889: b"void dd(void)\0": typeof(_1255 = &(*_1256)) = & {l2562} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1889: b"void dd(void)\0":   l2562 = UNIQUE | NON_NULL, (empty)
                                        // 1889: b"void dd(void)\0": typeof(_1256 = const b"void dd(void)\x00") = & {l2561} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1889: b"void dd(void)\0":   l2561 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                        };
                        (*o).val = newval;
                        res = run();
                        if res != golden {
                        // 1897: golden: typeof(_1262) = *mut {l1774} i32
                        // 1897: golden:   l1774 = UNIQUE | NON_NULL, (empty)
                            (*o).val = oldval_0;
                            break;
                        } else {
                            if verbose > 1 as libc::c_int {
                            // 1901: verbose: typeof(_1268) = *mut {l1781} i32
                            // 1901: verbose:   l1781 = UNIQUE | NON_NULL, (empty)
                                if reported != 0 {
                                    newline();
                                }
                                msg(
                                    b"reduced option %s from %d to %d by one\0" as *const u8
                                    // 1906: b"reduced optio ... _char: typeof(_1275) = *const {l1789} i8
                                    // 1906: b"reduced optio ... _char:   l1789 = UNIQUE | NON_NULL, (empty)
                                    // 1906: b"reduced optio ... st u8: typeof(_1276) = *const {l1791} u8
                                    // 1906: b"reduced optio ... st u8:   l1791 = UNIQUE | NON_NULL, (empty)
                                    // 1906: b"reduced optio ... ne\0": typeof(_1277) = *const {l1793} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                    // 1906: b"reduced optio ... ne\0":   l1793 = UNIQUE | NON_NULL, (empty)
                                    // 1906: b"reduced optio ... ne\0": typeof(_1278) = & {l1795} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                    // 1906: b"reduced optio ... ne\0":   l1795 = UNIQUE | NON_NULL, FIXED
                                    // 1906: b"reduced optio ... ne\0": typeof(_1277 = &raw const (*_1278)) = *const {l2566} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                    // 1906: b"reduced optio ... ne\0":   l2566 = UNIQUE | NON_NULL, (empty)
                                    // 1906: b"reduced optio ... _char: typeof(_1275 = move _1276 as *const i8 (Misc)) = *const {l2568} i8
                                    // 1906: b"reduced optio ... _char:   l2568 = UNIQUE | NON_NULL, (empty)
                                    // 1906: b"reduced optio ... ne\0": typeof(_1278 = const b"reduced option %s from %d to %d by one\x00") = & {l2565} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                                    // 1906: b"reduced optio ... ne\0":   l2565 = UNIQUE | NON_NULL, (empty)
                                    // 1906: b"reduced optio ... st u8: typeof(_1276 = move _1277 as *const u8 (Pointer(ArrayToPointer))) = *const {l2567} u8
                                    // 1906: b"reduced optio ... st u8:   l2567 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                    (*o).name,
                                    // 1908: (*o).name: typeof(_1279) = *mut {l1797} i8
                                    // 1908: (*o).name:   l1797 = UNIQUE | NON_NULL, (empty)
                                    oldval_0,
                                    newval,
                                );
                                reported = 0 as libc::c_int;
                            }
                            changed = 1 as libc::c_int;
                            if oldval_0 - newval == 1 as libc::c_int {
                            } else {
                                __assert_fail(
                                    b"oldval - newval == 1\0" as *const u8 as *const libc::c_char,
                                    // 1918: b"oldval - newv ... _char: typeof(_1293) = *const {l1812} i8
                                    // 1918: b"oldval - newv ... _char:   l1812 = UNIQUE | NON_NULL, (empty)
                                    // 1918: b"oldval - newv ... st u8: typeof(_1294) = *const {l1814} u8
                                    // 1918: b"oldval - newv ... st u8:   l1814 = UNIQUE | NON_NULL, (empty)
                                    // 1918: b"oldval - newv ...  1\0": typeof(_1295) = *const {l1816} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                    // 1918: b"oldval - newv ...  1\0":   l1816 = UNIQUE | NON_NULL, (empty)
                                    // 1918: b"oldval - newv ...  1\0": typeof(_1296) = & {l1818} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                    // 1918: b"oldval - newv ...  1\0":   l1818 = UNIQUE | NON_NULL, FIXED
                                    // 1918: b"oldval - newv ... _char: typeof(_1293 = move _1294 as *const i8 (Misc)) = *const {l2572} i8
                                    // 1918: b"oldval - newv ... _char:   l2572 = UNIQUE | NON_NULL, (empty)
                                    // 1918: b"oldval - newv ...  1\0": typeof(_1296 = const b"oldval - newval == 1\x00") = & {l2569} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                    // 1918: b"oldval - newv ...  1\0":   l2569 = UNIQUE | NON_NULL, (empty)
                                    // 1918: b"oldval - newv ... st u8: typeof(_1294 = move _1295 as *const u8 (Pointer(ArrayToPointer))) = *const {l2571} u8
                                    // 1918: b"oldval - newv ... st u8:   l2571 = UNIQUE | NON_NULL, (empty)
                                    // 1918: b"oldval - newv ...  1\0": typeof(_1295 = &raw const (*_1296)) = *const {l2570} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                    // 1918: b"oldval - newv ...  1\0":   l2570 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 1919: b"lglddtrace.c\ ... _char: typeof(_1297) = *const {l1820} i8
                                    // 1919: b"lglddtrace.c\ ... _char:   l1820 = UNIQUE | NON_NULL, (empty)
                                    // 1919: b"lglddtrace.c\ ... st u8: typeof(_1298) = *const {l1822} u8
                                    // 1919: b"lglddtrace.c\ ... st u8:   l1822 = UNIQUE | NON_NULL, (empty)
                                    // 1919: b"lglddtrace.c\0": typeof(_1299) = *const {l1824} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1919: b"lglddtrace.c\0":   l1824 = UNIQUE | NON_NULL, (empty)
                                    // 1919: b"lglddtrace.c\0": typeof(_1300) = & {l1826} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1919: b"lglddtrace.c\0":   l1826 = UNIQUE | NON_NULL, FIXED
                                    // 1919: b"lglddtrace.c\0": typeof(_1300 = const b"lglddtrace.c\x00") = & {l2573} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1919: b"lglddtrace.c\0":   l2573 = UNIQUE | NON_NULL, (empty)
                                    // 1919: b"lglddtrace.c\ ... _char: typeof(_1297 = move _1298 as *const i8 (Misc)) = *const {l2576} i8
                                    // 1919: b"lglddtrace.c\ ... _char:   l2576 = UNIQUE | NON_NULL, (empty)
                                    // 1919: b"lglddtrace.c\0": typeof(_1299 = &raw const (*_1300)) = *const {l2574} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 1919: b"lglddtrace.c\0":   l2574 = UNIQUE | NON_NULL, (empty)
                                    // 1919: b"lglddtrace.c\ ... st u8: typeof(_1298 = move _1299 as *const u8 (Pointer(ArrayToPointer))) = *const {l2575} u8
                                    // 1919: b"lglddtrace.c\ ... st u8:   l2575 = UNIQUE | NON_NULL, (empty)
                                    677 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    // 1921: (*::core::mem:: ... ptr(): typeof(_1303) = *const {l1830} i8
                                    // 1921: (*::core::mem:: ... ptr():   l1830 = UNIQUE | NON_NULL, (empty)
                                    // 1921: (*::core::mem:: ... ptr(): typeof(_1304) = & {l1832} [i8]
                                    // 1921: (*::core::mem:: ... ptr():   l1832 = UNIQUE | NON_NULL, FIXED
                                    // 1921: (*::core::mem:: ... ptr(): typeof(_1305) = & {l1834} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1921: (*::core::mem:: ... ptr():   l1834 = UNIQUE | NON_NULL, (empty)
                                    // 1921: ::core::mem::tr ... 0", ): typeof(_1306) = & {l1836} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1921: ::core::mem::tr ... 0", ):   l1836 = UNIQUE | NON_NULL, FIXED
                                    // 1921: (*::core::mem:: ... ptr(): typeof(_1305 = &(*_1306)) = & {l2579} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1921: (*::core::mem:: ... ptr():   l2579 = UNIQUE | NON_NULL, (empty)
                                    // 1921: (*::core::mem:: ... ptr(): typeof(_1304 = move _1305 as &[i8] (Pointer(Unsize))) = & {l2580} [i8]
                                    // 1921: (*::core::mem:: ... ptr():   l2580 = UNIQUE | NON_NULL, FIXED
                                        b"void dd(void)\0",
                                        // 1922: b"void dd(void)\0": typeof(_1307) = & {l1838} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1922: b"void dd(void)\0":   l1838 = UNIQUE | NON_NULL, (empty)
                                        // 1922: b"void dd(void)\0": typeof(_1308) = & {l1840} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1922: b"void dd(void)\0":   l1840 = UNIQUE | NON_NULL, FIXED
                                        // 1922: b"void dd(void)\0": typeof(_1307 = &(*_1308)) = & {l2578} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1922: b"void dd(void)\0":   l2578 = UNIQUE | NON_NULL, (empty)
                                        // 1922: b"void dd(void)\0": typeof(_1308 = const b"void dd(void)\x00") = & {l2577} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1922: b"void dd(void)\0":   l2577 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                            'c_8023: {
                                if oldval_0 - newval == 1 as libc::c_int {
                                } else {
                                    __assert_fail(
                                        b"oldval - newval == 1\0" as *const u8
                                        // 1931: b"oldval - newv ... _char: typeof(_1318) = *const {l1851} i8
                                        // 1931: b"oldval - newv ... _char:   l1851 = UNIQUE | NON_NULL, (empty)
                                        // 1931: b"oldval - newv ... st u8: typeof(_1319) = *const {l1853} u8
                                        // 1931: b"oldval - newv ... st u8:   l1853 = UNIQUE | NON_NULL, (empty)
                                        // 1931: b"oldval - newv ...  1\0": typeof(_1320) = *const {l1855} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                        // 1931: b"oldval - newv ...  1\0":   l1855 = UNIQUE | NON_NULL, (empty)
                                        // 1931: b"oldval - newv ...  1\0": typeof(_1321) = & {l1857} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                        // 1931: b"oldval - newv ...  1\0":   l1857 = UNIQUE | NON_NULL, FIXED
                                        // 1931: b"oldval - newv ... _char: typeof(_1318 = move _1319 as *const i8 (Misc)) = *const {l2584} i8
                                        // 1931: b"oldval - newv ... _char:   l2584 = UNIQUE | NON_NULL, (empty)
                                        // 1931: b"oldval - newv ... st u8: typeof(_1319 = move _1320 as *const u8 (Pointer(ArrayToPointer))) = *const {l2583} u8
                                        // 1931: b"oldval - newv ... st u8:   l2583 = UNIQUE | NON_NULL, (empty)
                                        // 1931: b"oldval - newv ...  1\0": typeof(_1321 = const b"oldval - newval == 1\x00") = & {l2581} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                        // 1931: b"oldval - newv ...  1\0":   l2581 = UNIQUE | NON_NULL, (empty)
                                        // 1931: b"oldval - newv ...  1\0": typeof(_1320 = &raw const (*_1321)) = *const {l2582} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                                        // 1931: b"oldval - newv ...  1\0":   l2582 = UNIQUE | NON_NULL, (empty)
                                            as *const libc::c_char,
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        // 1933: b"lglddtrace.c\ ... _char: typeof(_1322) = *const {l1859} i8
                                        // 1933: b"lglddtrace.c\ ... _char:   l1859 = UNIQUE | NON_NULL, (empty)
                                        // 1933: b"lglddtrace.c\ ... st u8: typeof(_1323) = *const {l1861} u8
                                        // 1933: b"lglddtrace.c\ ... st u8:   l1861 = UNIQUE | NON_NULL, (empty)
                                        // 1933: b"lglddtrace.c\0": typeof(_1324) = *const {l1863} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1933: b"lglddtrace.c\0":   l1863 = UNIQUE | NON_NULL, (empty)
                                        // 1933: b"lglddtrace.c\0": typeof(_1325) = & {l1865} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1933: b"lglddtrace.c\0":   l1865 = UNIQUE | NON_NULL, FIXED
                                        // 1933: b"lglddtrace.c\0": typeof(_1324 = &raw const (*_1325)) = *const {l2586} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1933: b"lglddtrace.c\0":   l2586 = UNIQUE | NON_NULL, (empty)
                                        // 1933: b"lglddtrace.c\ ... _char: typeof(_1322 = move _1323 as *const i8 (Misc)) = *const {l2588} i8
                                        // 1933: b"lglddtrace.c\ ... _char:   l2588 = UNIQUE | NON_NULL, (empty)
                                        // 1933: b"lglddtrace.c\ ... st u8: typeof(_1323 = move _1324 as *const u8 (Pointer(ArrayToPointer))) = *const {l2587} u8
                                        // 1933: b"lglddtrace.c\ ... st u8:   l2587 = UNIQUE | NON_NULL, (empty)
                                        // 1933: b"lglddtrace.c\0": typeof(_1325 = const b"lglddtrace.c\x00") = & {l2585} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 1933: b"lglddtrace.c\0":   l2585 = UNIQUE | NON_NULL, (empty)
                                        677 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                        // 1935: (*::core::mem:: ... ptr(): typeof(_1328) = *const {l1869} i8
                                        // 1935: (*::core::mem:: ... ptr():   l1869 = UNIQUE | NON_NULL, (empty)
                                        // 1935: (*::core::mem:: ... ptr(): typeof(_1329) = & {l1871} [i8]
                                        // 1935: (*::core::mem:: ... ptr():   l1871 = UNIQUE | NON_NULL, FIXED
                                        // 1935: (*::core::mem:: ... ptr(): typeof(_1330) = & {l1873} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1935: (*::core::mem:: ... ptr():   l1873 = UNIQUE | NON_NULL, (empty)
                                        // 1935: ::core::mem::tr ... 0", ): typeof(_1331) = & {l1875} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1935: ::core::mem::tr ... 0", ):   l1875 = UNIQUE | NON_NULL, FIXED
                                        // 1935: (*::core::mem:: ... ptr(): typeof(_1329 = move _1330 as &[i8] (Pointer(Unsize))) = & {l2592} [i8]
                                        // 1935: (*::core::mem:: ... ptr():   l2592 = UNIQUE | NON_NULL, FIXED
                                        // 1935: (*::core::mem:: ... ptr(): typeof(_1330 = &(*_1331)) = & {l2591} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 1935: (*::core::mem:: ... ptr():   l2591 = UNIQUE | NON_NULL, (empty)
                                            b"void dd(void)\0",
                                            // 1936: b"void dd(void)\0": typeof(_1332) = & {l1877} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1936: b"void dd(void)\0":   l1877 = UNIQUE | NON_NULL, (empty)
                                            // 1936: b"void dd(void)\0": typeof(_1333) = & {l1879} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1936: b"void dd(void)\0":   l1879 = UNIQUE | NON_NULL, FIXED
                                            // 1936: b"void dd(void)\0": typeof(_1333 = const b"void dd(void)\x00") = & {l2589} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1936: b"void dd(void)\0":   l2589 = UNIQUE | NON_NULL, (empty)
                                            // 1936: b"void dd(void)\0": typeof(_1332 = &(*_1333)) = & {l2590} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 1936: b"void dd(void)\0":   l2590 = UNIQUE | NON_NULL, (empty)
                                        ))
                                        .as_ptr(),
                                    );
                                }
                            };
                            sumoptvals -= 1;
                            // 1942: sumoptvals: typeof(_1334) = *mut {l1881} i64
                            // 1942: sumoptvals:   l1881 = UNIQUE | NON_NULL, (empty)
                            sumoptvals;
                            // 1943: sumoptvals: typeof(_1337) = *mut {l1885} i64
                            // 1943: sumoptvals:   l1885 = UNIQUE | NON_NULL, (empty)
                        }
                    }
                } else {
                    let mut upper: libc::c_int = (*o).val;
                    let mut lower: libc::c_int = (*o).min;
                    if lower <= upper {
                    } else {
                        __assert_fail(
                            b"lower <= upper\0" as *const u8 as *const libc::c_char,
                            // 1952: b"lower <= uppe ... _char: typeof(_1349) = *const {l1898} i8
                            // 1952: b"lower <= uppe ... _char:   l1898 = UNIQUE | NON_NULL, (empty)
                            // 1952: b"lower <= uppe ... st u8: typeof(_1350) = *const {l1900} u8
                            // 1952: b"lower <= uppe ... st u8:   l1900 = UNIQUE | NON_NULL, (empty)
                            // 1952: b"lower <= upper\0": typeof(_1351) = *const {l1902} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 1952: b"lower <= upper\0":   l1902 = UNIQUE | NON_NULL, (empty)
                            // 1952: b"lower <= upper\0": typeof(_1352) = & {l1904} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 1952: b"lower <= upper\0":   l1904 = UNIQUE | NON_NULL, FIXED
                            // 1952: b"lower <= upper\0": typeof(_1351 = &raw const (*_1352)) = *const {l2594} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 1952: b"lower <= upper\0":   l2594 = UNIQUE | NON_NULL, (empty)
                            // 1952: b"lower <= upper\0": typeof(_1352 = const b"lower <= upper\x00") = & {l2593} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 1952: b"lower <= upper\0":   l2593 = UNIQUE | NON_NULL, (empty)
                            // 1952: b"lower <= uppe ... st u8: typeof(_1350 = move _1351 as *const u8 (Pointer(ArrayToPointer))) = *const {l2595} u8
                            // 1952: b"lower <= uppe ... st u8:   l2595 = UNIQUE | NON_NULL, (empty)
                            // 1952: b"lower <= uppe ... _char: typeof(_1349 = move _1350 as *const i8 (Misc)) = *const {l2596} i8
                            // 1952: b"lower <= uppe ... _char:   l2596 = UNIQUE | NON_NULL, (empty)
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            // 1953: b"lglddtrace.c\ ... _char: typeof(_1353) = *const {l1906} i8
                            // 1953: b"lglddtrace.c\ ... _char:   l1906 = UNIQUE | NON_NULL, (empty)
                            // 1953: b"lglddtrace.c\ ... st u8: typeof(_1354) = *const {l1908} u8
                            // 1953: b"lglddtrace.c\ ... st u8:   l1908 = UNIQUE | NON_NULL, (empty)
                            // 1953: b"lglddtrace.c\0": typeof(_1355) = *const {l1910} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1953: b"lglddtrace.c\0":   l1910 = UNIQUE | NON_NULL, (empty)
                            // 1953: b"lglddtrace.c\0": typeof(_1356) = & {l1912} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1953: b"lglddtrace.c\0":   l1912 = UNIQUE | NON_NULL, FIXED
                            // 1953: b"lglddtrace.c\ ... _char: typeof(_1353 = move _1354 as *const i8 (Misc)) = *const {l2600} i8
                            // 1953: b"lglddtrace.c\ ... _char:   l2600 = UNIQUE | NON_NULL, (empty)
                            // 1953: b"lglddtrace.c\0": typeof(_1356 = const b"lglddtrace.c\x00") = & {l2597} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1953: b"lglddtrace.c\0":   l2597 = UNIQUE | NON_NULL, (empty)
                            // 1953: b"lglddtrace.c\ ... st u8: typeof(_1354 = move _1355 as *const u8 (Pointer(ArrayToPointer))) = *const {l2599} u8
                            // 1953: b"lglddtrace.c\ ... st u8:   l2599 = UNIQUE | NON_NULL, (empty)
                            // 1953: b"lglddtrace.c\0": typeof(_1355 = &raw const (*_1356)) = *const {l2598} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 1953: b"lglddtrace.c\0":   l2598 = UNIQUE | NON_NULL, (empty)
                            684 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            // 1955: (*::core::mem:: ... ptr(): typeof(_1359) = *const {l1916} i8
                            // 1955: (*::core::mem:: ... ptr():   l1916 = UNIQUE | NON_NULL, (empty)
                            // 1955: (*::core::mem:: ... ptr(): typeof(_1360) = & {l1918} [i8]
                            // 1955: (*::core::mem:: ... ptr():   l1918 = UNIQUE | NON_NULL, FIXED
                            // 1955: (*::core::mem:: ... ptr(): typeof(_1361) = & {l1920} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1955: (*::core::mem:: ... ptr():   l1920 = UNIQUE | NON_NULL, (empty)
                            // 1955: ::core::mem::tr ... 0", ): typeof(_1362) = & {l1922} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1955: ::core::mem::tr ... 0", ):   l1922 = UNIQUE | NON_NULL, FIXED
                            // 1955: (*::core::mem:: ... ptr(): typeof(_1360 = move _1361 as &[i8] (Pointer(Unsize))) = & {l2604} [i8]
                            // 1955: (*::core::mem:: ... ptr():   l2604 = UNIQUE | NON_NULL, FIXED
                            // 1955: (*::core::mem:: ... ptr(): typeof(_1361 = &(*_1362)) = & {l2603} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1955: (*::core::mem:: ... ptr():   l2603 = UNIQUE | NON_NULL, (empty)
                                b"void dd(void)\0",
                                // 1956: b"void dd(void)\0": typeof(_1363) = & {l1924} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1956: b"void dd(void)\0":   l1924 = UNIQUE | NON_NULL, (empty)
                                // 1956: b"void dd(void)\0": typeof(_1364) = & {l1926} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1956: b"void dd(void)\0":   l1926 = UNIQUE | NON_NULL, FIXED
                                // 1956: b"void dd(void)\0": typeof(_1364 = const b"void dd(void)\x00") = & {l2601} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1956: b"void dd(void)\0":   l2601 = UNIQUE | NON_NULL, (empty)
                                // 1956: b"void dd(void)\0": typeof(_1363 = &(*_1364)) = & {l2602} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1956: b"void dd(void)\0":   l2602 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                    'c_7971: {
                        if lower <= upper {
                        } else {
                            __assert_fail(
                                b"lower <= upper\0" as *const u8 as *const libc::c_char,
                                // 1965: b"lower <= uppe ... _char: typeof(_1371) = *const {l1934} i8
                                // 1965: b"lower <= uppe ... _char:   l1934 = UNIQUE | NON_NULL, (empty)
                                // 1965: b"lower <= uppe ... st u8: typeof(_1372) = *const {l1936} u8
                                // 1965: b"lower <= uppe ... st u8:   l1936 = UNIQUE | NON_NULL, (empty)
                                // 1965: b"lower <= upper\0": typeof(_1373) = *const {l1938} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 1965: b"lower <= upper\0":   l1938 = UNIQUE | NON_NULL, (empty)
                                // 1965: b"lower <= upper\0": typeof(_1374) = & {l1940} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 1965: b"lower <= upper\0":   l1940 = UNIQUE | NON_NULL, FIXED
                                // 1965: b"lower <= uppe ... _char: typeof(_1371 = move _1372 as *const i8 (Misc)) = *const {l2608} i8
                                // 1965: b"lower <= uppe ... _char:   l2608 = UNIQUE | NON_NULL, (empty)
                                // 1965: b"lower <= uppe ... st u8: typeof(_1372 = move _1373 as *const u8 (Pointer(ArrayToPointer))) = *const {l2607} u8
                                // 1965: b"lower <= uppe ... st u8:   l2607 = UNIQUE | NON_NULL, (empty)
                                // 1965: b"lower <= upper\0": typeof(_1374 = const b"lower <= upper\x00") = & {l2605} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 1965: b"lower <= upper\0":   l2605 = UNIQUE | NON_NULL, (empty)
                                // 1965: b"lower <= upper\0": typeof(_1373 = &raw const (*_1374)) = *const {l2606} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 1965: b"lower <= upper\0":   l2606 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 1966: b"lglddtrace.c\ ... _char: typeof(_1375) = *const {l1942} i8
                                // 1966: b"lglddtrace.c\ ... _char:   l1942 = UNIQUE | NON_NULL, (empty)
                                // 1966: b"lglddtrace.c\ ... st u8: typeof(_1376) = *const {l1944} u8
                                // 1966: b"lglddtrace.c\ ... st u8:   l1944 = UNIQUE | NON_NULL, (empty)
                                // 1966: b"lglddtrace.c\0": typeof(_1377) = *const {l1946} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1966: b"lglddtrace.c\0":   l1946 = UNIQUE | NON_NULL, (empty)
                                // 1966: b"lglddtrace.c\0": typeof(_1378) = & {l1948} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1966: b"lglddtrace.c\0":   l1948 = UNIQUE | NON_NULL, FIXED
                                // 1966: b"lglddtrace.c\ ... st u8: typeof(_1376 = move _1377 as *const u8 (Pointer(ArrayToPointer))) = *const {l2611} u8
                                // 1966: b"lglddtrace.c\ ... st u8:   l2611 = UNIQUE | NON_NULL, (empty)
                                // 1966: b"lglddtrace.c\0": typeof(_1377 = &raw const (*_1378)) = *const {l2610} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1966: b"lglddtrace.c\0":   l2610 = UNIQUE | NON_NULL, (empty)
                                // 1966: b"lglddtrace.c\0": typeof(_1378 = const b"lglddtrace.c\x00") = & {l2609} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 1966: b"lglddtrace.c\0":   l2609 = UNIQUE | NON_NULL, (empty)
                                // 1966: b"lglddtrace.c\ ... _char: typeof(_1375 = move _1376 as *const i8 (Misc)) = *const {l2612} i8
                                // 1966: b"lglddtrace.c\ ... _char:   l2612 = UNIQUE | NON_NULL, (empty)
                                684 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                // 1968: (*::core::mem:: ... ptr(): typeof(_1381) = *const {l1952} i8
                                // 1968: (*::core::mem:: ... ptr():   l1952 = UNIQUE | NON_NULL, (empty)
                                // 1968: (*::core::mem:: ... ptr(): typeof(_1382) = & {l1954} [i8]
                                // 1968: (*::core::mem:: ... ptr():   l1954 = UNIQUE | NON_NULL, FIXED
                                // 1968: (*::core::mem:: ... ptr(): typeof(_1383) = & {l1956} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1968: (*::core::mem:: ... ptr():   l1956 = UNIQUE | NON_NULL, (empty)
                                // 1968: ::core::mem::tr ... 0", ): typeof(_1384) = & {l1958} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1968: ::core::mem::tr ... 0", ):   l1958 = UNIQUE | NON_NULL, FIXED
                                // 1968: (*::core::mem:: ... ptr(): typeof(_1383 = &(*_1384)) = & {l2615} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1968: (*::core::mem:: ... ptr():   l2615 = UNIQUE | NON_NULL, (empty)
                                // 1968: (*::core::mem:: ... ptr(): typeof(_1382 = move _1383 as &[i8] (Pointer(Unsize))) = & {l2616} [i8]
                                // 1968: (*::core::mem:: ... ptr():   l2616 = UNIQUE | NON_NULL, FIXED
                                    b"void dd(void)\0",
                                    // 1969: b"void dd(void)\0": typeof(_1385) = & {l1960} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1969: b"void dd(void)\0":   l1960 = UNIQUE | NON_NULL, (empty)
                                    // 1969: b"void dd(void)\0": typeof(_1386) = & {l1962} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1969: b"void dd(void)\0":   l1962 = UNIQUE | NON_NULL, FIXED
                                    // 1969: b"void dd(void)\0": typeof(_1386 = const b"void dd(void)\x00") = & {l2613} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1969: b"void dd(void)\0":   l2613 = UNIQUE | NON_NULL, (empty)
                                    // 1969: b"void dd(void)\0": typeof(_1385 = &(*_1386)) = & {l2614} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 1969: b"void dd(void)\0":   l2614 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                    };
                    while upper > lower {
                        let mut oldval_1: libc::c_int = (*o).val;
                        let mut longnewval: libc::c_long = lower as libc::c_long
                            + (upper as libc::c_long - lower as libc::c_long)
                                / 2 as libc::c_int as libc::c_long;
                        let mut newval_0: libc::c_int = longnewval as libc::c_int;
                        (*o).val = newval_0;
                        res = run();
                        if res == golden {
                        // 1983: golden: typeof(_1416) = *mut {l1993} i32
                        // 1983: golden:   l1993 = UNIQUE | NON_NULL, (empty)
                            if verbose > 1 as libc::c_int {
                            // 1984: verbose: typeof(_1420) = *mut {l1998} i32
                            // 1984: verbose:   l1998 = UNIQUE | NON_NULL, (empty)
                                if reported != 0 {
                                    newline();
                                }
                                msg(
                                    b"reduced %s from %d to %d\0" as *const u8
                                    // 1989: b"reduced %s fr ... _char: typeof(_1427) = *const {l2006} i8
                                    // 1989: b"reduced %s fr ... _char:   l2006 = UNIQUE | NON_NULL, (empty)
                                    // 1989: b"reduced %s fr ... st u8: typeof(_1428) = *const {l2008} u8
                                    // 1989: b"reduced %s fr ... st u8:   l2008 = UNIQUE | NON_NULL, (empty)
                                    // 1989: b"reduced %s fr ... %d\0": typeof(_1429) = *const {l2010} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1989: b"reduced %s fr ... %d\0":   l2010 = UNIQUE | NON_NULL, (empty)
                                    // 1989: b"reduced %s fr ... %d\0": typeof(_1430) = & {l2012} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1989: b"reduced %s fr ... %d\0":   l2012 = UNIQUE | NON_NULL, FIXED
                                    // 1989: b"reduced %s fr ... _char: typeof(_1427 = move _1428 as *const i8 (Misc)) = *const {l2620} i8
                                    // 1989: b"reduced %s fr ... _char:   l2620 = UNIQUE | NON_NULL, (empty)
                                    // 1989: b"reduced %s fr ... %d\0": typeof(_1430 = const b"reduced %s from %d to %d\x00") = & {l2617} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1989: b"reduced %s fr ... %d\0":   l2617 = UNIQUE | NON_NULL, (empty)
                                    // 1989: b"reduced %s fr ... %d\0": typeof(_1429 = &raw const (*_1430)) = *const {l2618} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                    // 1989: b"reduced %s fr ... %d\0":   l2618 = UNIQUE | NON_NULL, (empty)
                                    // 1989: b"reduced %s fr ... st u8: typeof(_1428 = move _1429 as *const u8 (Pointer(ArrayToPointer))) = *const {l2619} u8
                                    // 1989: b"reduced %s fr ... st u8:   l2619 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                    (*o).name,
                                    // 1991: (*o).name: typeof(_1431) = *mut {l2014} i8
                                    // 1991: (*o).name:   l2014 = UNIQUE | NON_NULL, (empty)
                                    oldval_1,
                                    newval_0,
                                );
                                reported = 0 as libc::c_int;
                            }
                            changed = 1 as libc::c_int;
                            if newval_0 < upper {
                            } else {
                                __assert_fail(
                                    b"newval < upper\0" as *const u8 as *const libc::c_char,
                                    // 2001: b"newval < uppe ... _char: typeof(_1442) = *const {l2026} i8
                                    // 2001: b"newval < uppe ... _char:   l2026 = UNIQUE | NON_NULL, (empty)
                                    // 2001: b"newval < uppe ... st u8: typeof(_1443) = *const {l2028} u8
                                    // 2001: b"newval < uppe ... st u8:   l2028 = UNIQUE | NON_NULL, (empty)
                                    // 2001: b"newval < upper\0": typeof(_1444) = *const {l2030} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2001: b"newval < upper\0":   l2030 = UNIQUE | NON_NULL, (empty)
                                    // 2001: b"newval < upper\0": typeof(_1445) = & {l2032} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2001: b"newval < upper\0":   l2032 = UNIQUE | NON_NULL, FIXED
                                    // 2001: b"newval < uppe ... _char: typeof(_1442 = move _1443 as *const i8 (Misc)) = *const {l2624} i8
                                    // 2001: b"newval < uppe ... _char:   l2624 = UNIQUE | NON_NULL, (empty)
                                    // 2001: b"newval < upper\0": typeof(_1445 = const b"newval < upper\x00") = & {l2621} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2001: b"newval < upper\0":   l2621 = UNIQUE | NON_NULL, (empty)
                                    // 2001: b"newval < uppe ... st u8: typeof(_1443 = move _1444 as *const u8 (Pointer(ArrayToPointer))) = *const {l2623} u8
                                    // 2001: b"newval < uppe ... st u8:   l2623 = UNIQUE | NON_NULL, (empty)
                                    // 2001: b"newval < upper\0": typeof(_1444 = &raw const (*_1445)) = *const {l2622} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2001: b"newval < upper\0":   l2622 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 2002: b"lglddtrace.c\ ... _char: typeof(_1446) = *const {l2034} i8
                                    // 2002: b"lglddtrace.c\ ... _char:   l2034 = UNIQUE | NON_NULL, (empty)
                                    // 2002: b"lglddtrace.c\ ... st u8: typeof(_1447) = *const {l2036} u8
                                    // 2002: b"lglddtrace.c\ ... st u8:   l2036 = UNIQUE | NON_NULL, (empty)
                                    // 2002: b"lglddtrace.c\0": typeof(_1448) = *const {l2038} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2002: b"lglddtrace.c\0":   l2038 = UNIQUE | NON_NULL, (empty)
                                    // 2002: b"lglddtrace.c\0": typeof(_1449) = & {l2040} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2002: b"lglddtrace.c\0":   l2040 = UNIQUE | NON_NULL, FIXED
                                    // 2002: b"lglddtrace.c\0": typeof(_1449 = const b"lglddtrace.c\x00") = & {l2625} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2002: b"lglddtrace.c\0":   l2625 = UNIQUE | NON_NULL, (empty)
                                    // 2002: b"lglddtrace.c\ ... st u8: typeof(_1447 = move _1448 as *const u8 (Pointer(ArrayToPointer))) = *const {l2627} u8
                                    // 2002: b"lglddtrace.c\ ... st u8:   l2627 = UNIQUE | NON_NULL, (empty)
                                    // 2002: b"lglddtrace.c\0": typeof(_1448 = &raw const (*_1449)) = *const {l2626} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2002: b"lglddtrace.c\0":   l2626 = UNIQUE | NON_NULL, (empty)
                                    // 2002: b"lglddtrace.c\ ... _char: typeof(_1446 = move _1447 as *const i8 (Misc)) = *const {l2628} i8
                                    // 2002: b"lglddtrace.c\ ... _char:   l2628 = UNIQUE | NON_NULL, (empty)
                                    698 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    // 2004: (*::core::mem:: ... ptr(): typeof(_1452) = *const {l2044} i8
                                    // 2004: (*::core::mem:: ... ptr():   l2044 = UNIQUE | NON_NULL, (empty)
                                    // 2004: (*::core::mem:: ... ptr(): typeof(_1453) = & {l2046} [i8]
                                    // 2004: (*::core::mem:: ... ptr():   l2046 = UNIQUE | NON_NULL, FIXED
                                    // 2004: (*::core::mem:: ... ptr(): typeof(_1454) = & {l2048} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2004: (*::core::mem:: ... ptr():   l2048 = UNIQUE | NON_NULL, (empty)
                                    // 2004: ::core::mem::tr ... 0", ): typeof(_1455) = & {l2050} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2004: ::core::mem::tr ... 0", ):   l2050 = UNIQUE | NON_NULL, FIXED
                                    // 2004: (*::core::mem:: ... ptr(): typeof(_1454 = &(*_1455)) = & {l2631} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2004: (*::core::mem:: ... ptr():   l2631 = UNIQUE | NON_NULL, (empty)
                                    // 2004: (*::core::mem:: ... ptr(): typeof(_1453 = move _1454 as &[i8] (Pointer(Unsize))) = & {l2632} [i8]
                                    // 2004: (*::core::mem:: ... ptr():   l2632 = UNIQUE | NON_NULL, FIXED
                                        b"void dd(void)\0",
                                        // 2005: b"void dd(void)\0": typeof(_1456) = & {l2052} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2005: b"void dd(void)\0":   l2052 = UNIQUE | NON_NULL, (empty)
                                        // 2005: b"void dd(void)\0": typeof(_1457) = & {l2054} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2005: b"void dd(void)\0":   l2054 = UNIQUE | NON_NULL, FIXED
                                        // 2005: b"void dd(void)\0": typeof(_1457 = const b"void dd(void)\x00") = & {l2629} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2005: b"void dd(void)\0":   l2629 = UNIQUE | NON_NULL, (empty)
                                        // 2005: b"void dd(void)\0": typeof(_1456 = &(*_1457)) = & {l2630} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2005: b"void dd(void)\0":   l2630 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                            'c_7878: {
                                if newval_0 < upper {
                                } else {
                                    __assert_fail(
                                        b"newval < upper\0" as *const u8 as *const libc::c_char,
                                        // 2014: b"newval < uppe ... _char: typeof(_1464) = *const {l2062} i8
                                        // 2014: b"newval < uppe ... _char:   l2062 = UNIQUE | NON_NULL, (empty)
                                        // 2014: b"newval < uppe ... st u8: typeof(_1465) = *const {l2064} u8
                                        // 2014: b"newval < uppe ... st u8:   l2064 = UNIQUE | NON_NULL, (empty)
                                        // 2014: b"newval < upper\0": typeof(_1466) = *const {l2066} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2014: b"newval < upper\0":   l2066 = UNIQUE | NON_NULL, (empty)
                                        // 2014: b"newval < upper\0": typeof(_1467) = & {l2068} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2014: b"newval < upper\0":   l2068 = UNIQUE | NON_NULL, FIXED
                                        // 2014: b"newval < upper\0": typeof(_1467 = const b"newval < upper\x00") = & {l2633} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2014: b"newval < upper\0":   l2633 = UNIQUE | NON_NULL, (empty)
                                        // 2014: b"newval < uppe ... _char: typeof(_1464 = move _1465 as *const i8 (Misc)) = *const {l2636} i8
                                        // 2014: b"newval < uppe ... _char:   l2636 = UNIQUE | NON_NULL, (empty)
                                        // 2014: b"newval < uppe ... st u8: typeof(_1465 = move _1466 as *const u8 (Pointer(ArrayToPointer))) = *const {l2635} u8
                                        // 2014: b"newval < uppe ... st u8:   l2635 = UNIQUE | NON_NULL, (empty)
                                        // 2014: b"newval < upper\0": typeof(_1466 = &raw const (*_1467)) = *const {l2634} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2014: b"newval < upper\0":   l2634 = UNIQUE | NON_NULL, (empty)
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        // 2015: b"lglddtrace.c\ ... _char: typeof(_1468) = *const {l2070} i8
                                        // 2015: b"lglddtrace.c\ ... _char:   l2070 = UNIQUE | NON_NULL, (empty)
                                        // 2015: b"lglddtrace.c\ ... st u8: typeof(_1469) = *const {l2072} u8
                                        // 2015: b"lglddtrace.c\ ... st u8:   l2072 = UNIQUE | NON_NULL, (empty)
                                        // 2015: b"lglddtrace.c\0": typeof(_1470) = *const {l2074} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2015: b"lglddtrace.c\0":   l2074 = UNIQUE | NON_NULL, (empty)
                                        // 2015: b"lglddtrace.c\0": typeof(_1471) = & {l2076} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2015: b"lglddtrace.c\0":   l2076 = UNIQUE | NON_NULL, FIXED
                                        // 2015: b"lglddtrace.c\ ... _char: typeof(_1468 = move _1469 as *const i8 (Misc)) = *const {l2640} i8
                                        // 2015: b"lglddtrace.c\ ... _char:   l2640 = UNIQUE | NON_NULL, (empty)
                                        // 2015: b"lglddtrace.c\0": typeof(_1470 = &raw const (*_1471)) = *const {l2638} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2015: b"lglddtrace.c\0":   l2638 = UNIQUE | NON_NULL, (empty)
                                        // 2015: b"lglddtrace.c\ ... st u8: typeof(_1469 = move _1470 as *const u8 (Pointer(ArrayToPointer))) = *const {l2639} u8
                                        // 2015: b"lglddtrace.c\ ... st u8:   l2639 = UNIQUE | NON_NULL, (empty)
                                        // 2015: b"lglddtrace.c\0": typeof(_1471 = const b"lglddtrace.c\x00") = & {l2637} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2015: b"lglddtrace.c\0":   l2637 = UNIQUE | NON_NULL, (empty)
                                        698 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                        // 2017: (*::core::mem:: ... ptr(): typeof(_1474) = *const {l2080} i8
                                        // 2017: (*::core::mem:: ... ptr():   l2080 = UNIQUE | NON_NULL, (empty)
                                        // 2017: (*::core::mem:: ... ptr(): typeof(_1475) = & {l2082} [i8]
                                        // 2017: (*::core::mem:: ... ptr():   l2082 = UNIQUE | NON_NULL, FIXED
                                        // 2017: (*::core::mem:: ... ptr(): typeof(_1476) = & {l2084} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2017: (*::core::mem:: ... ptr():   l2084 = UNIQUE | NON_NULL, (empty)
                                        // 2017: ::core::mem::tr ... 0", ): typeof(_1477) = & {l2086} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2017: ::core::mem::tr ... 0", ):   l2086 = UNIQUE | NON_NULL, FIXED
                                        // 2017: (*::core::mem:: ... ptr(): typeof(_1476 = &(*_1477)) = & {l2643} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2017: (*::core::mem:: ... ptr():   l2643 = UNIQUE | NON_NULL, (empty)
                                        // 2017: (*::core::mem:: ... ptr(): typeof(_1475 = move _1476 as &[i8] (Pointer(Unsize))) = & {l2644} [i8]
                                        // 2017: (*::core::mem:: ... ptr():   l2644 = UNIQUE | NON_NULL, FIXED
                                            b"void dd(void)\0",
                                            // 2018: b"void dd(void)\0": typeof(_1478) = & {l2088} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2018: b"void dd(void)\0":   l2088 = UNIQUE | NON_NULL, (empty)
                                            // 2018: b"void dd(void)\0": typeof(_1479) = & {l2090} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2018: b"void dd(void)\0":   l2090 = UNIQUE | NON_NULL, FIXED
                                            // 2018: b"void dd(void)\0": typeof(_1478 = &(*_1479)) = & {l2642} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2018: b"void dd(void)\0":   l2642 = UNIQUE | NON_NULL, (empty)
                                            // 2018: b"void dd(void)\0": typeof(_1479 = const b"void dd(void)\x00") = & {l2641} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2018: b"void dd(void)\0":   l2641 = UNIQUE | NON_NULL, (empty)
                                        ))
                                        .as_ptr(),
                                    );
                                }
                            };
                            upper = newval_0;
                            sumoptvals -= (oldval_1 - newval_0) as libc::c_longlong;
                            // 2025: sumoptvals: typeof(_1486) = *mut {l2098} i64
                            // 2025: sumoptvals:   l2098 = UNIQUE | NON_NULL, (empty)
                        } else {
                            (*o).val = oldval_1;
                            if lower == newval_0 {
                                break;
                            }
                            if lower < newval_0 {
                            } else {
                                __assert_fail(
                                    b"lower < newval\0" as *const u8 as *const libc::c_char,
                                    // 2034: b"lower < newva ... _char: typeof(_1500) = *const {l2113} i8
                                    // 2034: b"lower < newva ... _char:   l2113 = UNIQUE | NON_NULL, (empty)
                                    // 2034: b"lower < newva ... st u8: typeof(_1501) = *const {l2115} u8
                                    // 2034: b"lower < newva ... st u8:   l2115 = UNIQUE | NON_NULL, (empty)
                                    // 2034: b"lower < newval\0": typeof(_1502) = *const {l2117} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2034: b"lower < newval\0":   l2117 = UNIQUE | NON_NULL, (empty)
                                    // 2034: b"lower < newval\0": typeof(_1503) = & {l2119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2034: b"lower < newval\0":   l2119 = UNIQUE | NON_NULL, FIXED
                                    // 2034: b"lower < newval\0": typeof(_1503 = const b"lower < newval\x00") = & {l2645} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2034: b"lower < newval\0":   l2645 = UNIQUE | NON_NULL, (empty)
                                    // 2034: b"lower < newva ... _char: typeof(_1500 = move _1501 as *const i8 (Misc)) = *const {l2648} i8
                                    // 2034: b"lower < newva ... _char:   l2648 = UNIQUE | NON_NULL, (empty)
                                    // 2034: b"lower < newval\0": typeof(_1502 = &raw const (*_1503)) = *const {l2646} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                    // 2034: b"lower < newval\0":   l2646 = UNIQUE | NON_NULL, (empty)
                                    // 2034: b"lower < newva ... st u8: typeof(_1501 = move _1502 as *const u8 (Pointer(ArrayToPointer))) = *const {l2647} u8
                                    // 2034: b"lower < newva ... st u8:   l2647 = UNIQUE | NON_NULL, (empty)
                                    b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                    // 2035: b"lglddtrace.c\ ... _char: typeof(_1504) = *const {l2121} i8
                                    // 2035: b"lglddtrace.c\ ... _char:   l2121 = UNIQUE | NON_NULL, (empty)
                                    // 2035: b"lglddtrace.c\ ... st u8: typeof(_1505) = *const {l2123} u8
                                    // 2035: b"lglddtrace.c\ ... st u8:   l2123 = UNIQUE | NON_NULL, (empty)
                                    // 2035: b"lglddtrace.c\0": typeof(_1506) = *const {l2125} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2035: b"lglddtrace.c\0":   l2125 = UNIQUE | NON_NULL, (empty)
                                    // 2035: b"lglddtrace.c\0": typeof(_1507) = & {l2127} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2035: b"lglddtrace.c\0":   l2127 = UNIQUE | NON_NULL, FIXED
                                    // 2035: b"lglddtrace.c\0": typeof(_1507 = const b"lglddtrace.c\x00") = & {l2649} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2035: b"lglddtrace.c\0":   l2649 = UNIQUE | NON_NULL, (empty)
                                    // 2035: b"lglddtrace.c\ ... _char: typeof(_1504 = move _1505 as *const i8 (Misc)) = *const {l2652} i8
                                    // 2035: b"lglddtrace.c\ ... _char:   l2652 = UNIQUE | NON_NULL, (empty)
                                    // 2035: b"lglddtrace.c\ ... st u8: typeof(_1505 = move _1506 as *const u8 (Pointer(ArrayToPointer))) = *const {l2651} u8
                                    // 2035: b"lglddtrace.c\ ... st u8:   l2651 = UNIQUE | NON_NULL, (empty)
                                    // 2035: b"lglddtrace.c\0": typeof(_1506 = &raw const (*_1507)) = *const {l2650} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                    // 2035: b"lglddtrace.c\0":   l2650 = UNIQUE | NON_NULL, (empty)
                                    705 as libc::c_int as libc::c_uint,
                                    (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                    // 2037: (*::core::mem:: ... ptr(): typeof(_1510) = *const {l2131} i8
                                    // 2037: (*::core::mem:: ... ptr():   l2131 = UNIQUE | NON_NULL, (empty)
                                    // 2037: (*::core::mem:: ... ptr(): typeof(_1511) = & {l2133} [i8]
                                    // 2037: (*::core::mem:: ... ptr():   l2133 = UNIQUE | NON_NULL, FIXED
                                    // 2037: (*::core::mem:: ... ptr(): typeof(_1512) = & {l2135} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2037: (*::core::mem:: ... ptr():   l2135 = UNIQUE | NON_NULL, (empty)
                                    // 2037: ::core::mem::tr ... 0", ): typeof(_1513) = & {l2137} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2037: ::core::mem::tr ... 0", ):   l2137 = UNIQUE | NON_NULL, FIXED
                                    // 2037: (*::core::mem:: ... ptr(): typeof(_1512 = &(*_1513)) = & {l2655} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2037: (*::core::mem:: ... ptr():   l2655 = UNIQUE | NON_NULL, (empty)
                                    // 2037: (*::core::mem:: ... ptr(): typeof(_1511 = move _1512 as &[i8] (Pointer(Unsize))) = & {l2656} [i8]
                                    // 2037: (*::core::mem:: ... ptr():   l2656 = UNIQUE | NON_NULL, FIXED
                                        b"void dd(void)\0",
                                        // 2038: b"void dd(void)\0": typeof(_1514) = & {l2139} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2038: b"void dd(void)\0":   l2139 = UNIQUE | NON_NULL, (empty)
                                        // 2038: b"void dd(void)\0": typeof(_1515) = & {l2141} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2038: b"void dd(void)\0":   l2141 = UNIQUE | NON_NULL, FIXED
                                        // 2038: b"void dd(void)\0": typeof(_1515 = const b"void dd(void)\x00") = & {l2653} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2038: b"void dd(void)\0":   l2653 = UNIQUE | NON_NULL, (empty)
                                        // 2038: b"void dd(void)\0": typeof(_1514 = &(*_1515)) = & {l2654} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2038: b"void dd(void)\0":   l2654 = UNIQUE | NON_NULL, (empty)
                                    ))
                                    .as_ptr(),
                                );
                            }
                            'c_7806: {
                                if lower < newval_0 {
                                } else {
                                    __assert_fail(
                                        b"lower < newval\0" as *const u8 as *const libc::c_char,
                                        // 2047: b"lower < newva ... _char: typeof(_1522) = *const {l2149} i8
                                        // 2047: b"lower < newva ... _char:   l2149 = UNIQUE | NON_NULL, (empty)
                                        // 2047: b"lower < newva ... st u8: typeof(_1523) = *const {l2151} u8
                                        // 2047: b"lower < newva ... st u8:   l2151 = UNIQUE | NON_NULL, (empty)
                                        // 2047: b"lower < newval\0": typeof(_1524) = *const {l2153} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2047: b"lower < newval\0":   l2153 = UNIQUE | NON_NULL, (empty)
                                        // 2047: b"lower < newval\0": typeof(_1525) = & {l2155} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2047: b"lower < newval\0":   l2155 = UNIQUE | NON_NULL, FIXED
                                        // 2047: b"lower < newva ... _char: typeof(_1522 = move _1523 as *const i8 (Misc)) = *const {l2660} i8
                                        // 2047: b"lower < newva ... _char:   l2660 = UNIQUE | NON_NULL, (empty)
                                        // 2047: b"lower < newval\0": typeof(_1524 = &raw const (*_1525)) = *const {l2658} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2047: b"lower < newval\0":   l2658 = UNIQUE | NON_NULL, (empty)
                                        // 2047: b"lower < newval\0": typeof(_1525 = const b"lower < newval\x00") = & {l2657} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                        // 2047: b"lower < newval\0":   l2657 = UNIQUE | NON_NULL, (empty)
                                        // 2047: b"lower < newva ... st u8: typeof(_1523 = move _1524 as *const u8 (Pointer(ArrayToPointer))) = *const {l2659} u8
                                        // 2047: b"lower < newva ... st u8:   l2659 = UNIQUE | NON_NULL, (empty)
                                        b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                        // 2048: b"lglddtrace.c\ ... _char: typeof(_1526) = *const {l2157} i8
                                        // 2048: b"lglddtrace.c\ ... _char:   l2157 = UNIQUE | NON_NULL, (empty)
                                        // 2048: b"lglddtrace.c\ ... st u8: typeof(_1527) = *const {l2159} u8
                                        // 2048: b"lglddtrace.c\ ... st u8:   l2159 = UNIQUE | NON_NULL, (empty)
                                        // 2048: b"lglddtrace.c\0": typeof(_1528) = *const {l2161} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2048: b"lglddtrace.c\0":   l2161 = UNIQUE | NON_NULL, (empty)
                                        // 2048: b"lglddtrace.c\0": typeof(_1529) = & {l2163} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2048: b"lglddtrace.c\0":   l2163 = UNIQUE | NON_NULL, FIXED
                                        // 2048: b"lglddtrace.c\ ... st u8: typeof(_1527 = move _1528 as *const u8 (Pointer(ArrayToPointer))) = *const {l2663} u8
                                        // 2048: b"lglddtrace.c\ ... st u8:   l2663 = UNIQUE | NON_NULL, (empty)
                                        // 2048: b"lglddtrace.c\ ... _char: typeof(_1526 = move _1527 as *const i8 (Misc)) = *const {l2664} i8
                                        // 2048: b"lglddtrace.c\ ... _char:   l2664 = UNIQUE | NON_NULL, (empty)
                                        // 2048: b"lglddtrace.c\0": typeof(_1529 = const b"lglddtrace.c\x00") = & {l2661} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2048: b"lglddtrace.c\0":   l2661 = UNIQUE | NON_NULL, (empty)
                                        // 2048: b"lglddtrace.c\0": typeof(_1528 = &raw const (*_1529)) = *const {l2662} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                        // 2048: b"lglddtrace.c\0":   l2662 = UNIQUE | NON_NULL, (empty)
                                        705 as libc::c_int as libc::c_uint,
                                        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                        // 2050: (*::core::mem:: ... ptr(): typeof(_1532) = *const {l2167} i8
                                        // 2050: (*::core::mem:: ... ptr():   l2167 = UNIQUE | NON_NULL, (empty)
                                        // 2050: (*::core::mem:: ... ptr(): typeof(_1533) = & {l2169} [i8]
                                        // 2050: (*::core::mem:: ... ptr():   l2169 = UNIQUE | NON_NULL, FIXED
                                        // 2050: (*::core::mem:: ... ptr(): typeof(_1534) = & {l2171} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2050: (*::core::mem:: ... ptr():   l2171 = UNIQUE | NON_NULL, (empty)
                                        // 2050: ::core::mem::tr ... 0", ): typeof(_1535) = & {l2173} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2050: ::core::mem::tr ... 0", ):   l2173 = UNIQUE | NON_NULL, FIXED
                                        // 2050: (*::core::mem:: ... ptr(): typeof(_1534 = &(*_1535)) = & {l2667} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                        // 2050: (*::core::mem:: ... ptr():   l2667 = UNIQUE | NON_NULL, (empty)
                                        // 2050: (*::core::mem:: ... ptr(): typeof(_1533 = move _1534 as &[i8] (Pointer(Unsize))) = & {l2668} [i8]
                                        // 2050: (*::core::mem:: ... ptr():   l2668 = UNIQUE | NON_NULL, FIXED
                                            b"void dd(void)\0",
                                            // 2051: b"void dd(void)\0": typeof(_1536) = & {l2175} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2051: b"void dd(void)\0":   l2175 = UNIQUE | NON_NULL, (empty)
                                            // 2051: b"void dd(void)\0": typeof(_1537) = & {l2177} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2051: b"void dd(void)\0":   l2177 = UNIQUE | NON_NULL, FIXED
                                            // 2051: b"void dd(void)\0": typeof(_1537 = const b"void dd(void)\x00") = & {l2665} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2051: b"void dd(void)\0":   l2665 = UNIQUE | NON_NULL, (empty)
                                            // 2051: b"void dd(void)\0": typeof(_1536 = &(*_1537)) = & {l2666} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                            // 2051: b"void dd(void)\0":   l2666 = UNIQUE | NON_NULL, (empty)
                                        ))
                                        .as_ptr(),
                                    );
                                }
                            };
                            lower = newval_0;
                        }
                    }
                    if lower <= upper {
                    } else {
                        __assert_fail(
                            b"lower <= upper\0" as *const u8 as *const libc::c_char,
                            // 2063: b"lower <= uppe ... _char: typeof(_1548) = *const {l2189} i8
                            // 2063: b"lower <= uppe ... _char:   l2189 = UNIQUE | NON_NULL, (empty)
                            // 2063: b"lower <= uppe ... st u8: typeof(_1549) = *const {l2191} u8
                            // 2063: b"lower <= uppe ... st u8:   l2191 = UNIQUE | NON_NULL, (empty)
                            // 2063: b"lower <= upper\0": typeof(_1550) = *const {l2193} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 2063: b"lower <= upper\0":   l2193 = UNIQUE | NON_NULL, (empty)
                            // 2063: b"lower <= upper\0": typeof(_1551) = & {l2195} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 2063: b"lower <= upper\0":   l2195 = UNIQUE | NON_NULL, FIXED
                            // 2063: b"lower <= upper\0": typeof(_1550 = &raw const (*_1551)) = *const {l2670} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 2063: b"lower <= upper\0":   l2670 = UNIQUE | NON_NULL, (empty)
                            // 2063: b"lower <= uppe ... st u8: typeof(_1549 = move _1550 as *const u8 (Pointer(ArrayToPointer))) = *const {l2671} u8
                            // 2063: b"lower <= uppe ... st u8:   l2671 = UNIQUE | NON_NULL, (empty)
                            // 2063: b"lower <= upper\0": typeof(_1551 = const b"lower <= upper\x00") = & {l2669} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                            // 2063: b"lower <= upper\0":   l2669 = UNIQUE | NON_NULL, (empty)
                            // 2063: b"lower <= uppe ... _char: typeof(_1548 = move _1549 as *const i8 (Misc)) = *const {l2672} i8
                            // 2063: b"lower <= uppe ... _char:   l2672 = UNIQUE | NON_NULL, (empty)
                            b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                            // 2064: b"lglddtrace.c\ ... _char: typeof(_1552) = *const {l2197} i8
                            // 2064: b"lglddtrace.c\ ... _char:   l2197 = UNIQUE | NON_NULL, (empty)
                            // 2064: b"lglddtrace.c\ ... st u8: typeof(_1553) = *const {l2199} u8
                            // 2064: b"lglddtrace.c\ ... st u8:   l2199 = UNIQUE | NON_NULL, (empty)
                            // 2064: b"lglddtrace.c\0": typeof(_1554) = *const {l2201} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 2064: b"lglddtrace.c\0":   l2201 = UNIQUE | NON_NULL, (empty)
                            // 2064: b"lglddtrace.c\0": typeof(_1555) = & {l2203} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 2064: b"lglddtrace.c\0":   l2203 = UNIQUE | NON_NULL, FIXED
                            // 2064: b"lglddtrace.c\0": typeof(_1555 = const b"lglddtrace.c\x00") = & {l2673} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 2064: b"lglddtrace.c\0":   l2673 = UNIQUE | NON_NULL, (empty)
                            // 2064: b"lglddtrace.c\0": typeof(_1554 = &raw const (*_1555)) = *const {l2674} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                            // 2064: b"lglddtrace.c\0":   l2674 = UNIQUE | NON_NULL, (empty)
                            // 2064: b"lglddtrace.c\ ... st u8: typeof(_1553 = move _1554 as *const u8 (Pointer(ArrayToPointer))) = *const {l2675} u8
                            // 2064: b"lglddtrace.c\ ... st u8:   l2675 = UNIQUE | NON_NULL, (empty)
                            // 2064: b"lglddtrace.c\ ... _char: typeof(_1552 = move _1553 as *const i8 (Misc)) = *const {l2676} i8
                            // 2064: b"lglddtrace.c\ ... _char:   l2676 = UNIQUE | NON_NULL, (empty)
                            709 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                            // 2066: (*::core::mem:: ... ptr(): typeof(_1558) = *const {l2207} i8
                            // 2066: (*::core::mem:: ... ptr():   l2207 = UNIQUE | NON_NULL, (empty)
                            // 2066: (*::core::mem:: ... ptr(): typeof(_1559) = & {l2209} [i8]
                            // 2066: (*::core::mem:: ... ptr():   l2209 = UNIQUE | NON_NULL, FIXED
                            // 2066: (*::core::mem:: ... ptr(): typeof(_1560) = & {l2211} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 2066: (*::core::mem:: ... ptr():   l2211 = UNIQUE | NON_NULL, (empty)
                            // 2066: ::core::mem::tr ... 0", ): typeof(_1561) = & {l2213} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 2066: ::core::mem::tr ... 0", ):   l2213 = UNIQUE | NON_NULL, FIXED
                            // 2066: (*::core::mem:: ... ptr(): typeof(_1560 = &(*_1561)) = & {l2679} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 2066: (*::core::mem:: ... ptr():   l2679 = UNIQUE | NON_NULL, (empty)
                            // 2066: (*::core::mem:: ... ptr(): typeof(_1559 = move _1560 as &[i8] (Pointer(Unsize))) = & {l2680} [i8]
                            // 2066: (*::core::mem:: ... ptr():   l2680 = UNIQUE | NON_NULL, FIXED
                                b"void dd(void)\0",
                                // 2067: b"void dd(void)\0": typeof(_1562) = & {l2215} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2067: b"void dd(void)\0":   l2215 = UNIQUE | NON_NULL, (empty)
                                // 2067: b"void dd(void)\0": typeof(_1563) = & {l2217} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2067: b"void dd(void)\0":   l2217 = UNIQUE | NON_NULL, FIXED
                                // 2067: b"void dd(void)\0": typeof(_1562 = &(*_1563)) = & {l2678} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2067: b"void dd(void)\0":   l2678 = UNIQUE | NON_NULL, (empty)
                                // 2067: b"void dd(void)\0": typeof(_1563 = const b"void dd(void)\x00") = & {l2677} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2067: b"void dd(void)\0":   l2677 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                    'c_7719: {
                        if lower <= upper {
                        } else {
                            __assert_fail(
                                b"lower <= upper\0" as *const u8 as *const libc::c_char,
                                // 2076: b"lower <= uppe ... _char: typeof(_1570) = *const {l2225} i8
                                // 2076: b"lower <= uppe ... _char:   l2225 = UNIQUE | NON_NULL, (empty)
                                // 2076: b"lower <= uppe ... st u8: typeof(_1571) = *const {l2227} u8
                                // 2076: b"lower <= uppe ... st u8:   l2227 = UNIQUE | NON_NULL, (empty)
                                // 2076: b"lower <= upper\0": typeof(_1572) = *const {l2229} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 2076: b"lower <= upper\0":   l2229 = UNIQUE | NON_NULL, (empty)
                                // 2076: b"lower <= upper\0": typeof(_1573) = & {l2231} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 2076: b"lower <= upper\0":   l2231 = UNIQUE | NON_NULL, FIXED
                                // 2076: b"lower <= uppe ... st u8: typeof(_1571 = move _1572 as *const u8 (Pointer(ArrayToPointer))) = *const {l2683} u8
                                // 2076: b"lower <= uppe ... st u8:   l2683 = UNIQUE | NON_NULL, (empty)
                                // 2076: b"lower <= uppe ... _char: typeof(_1570 = move _1571 as *const i8 (Misc)) = *const {l2684} i8
                                // 2076: b"lower <= uppe ... _char:   l2684 = UNIQUE | NON_NULL, (empty)
                                // 2076: b"lower <= upper\0": typeof(_1572 = &raw const (*_1573)) = *const {l2682} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 2076: b"lower <= upper\0":   l2682 = UNIQUE | NON_NULL, (empty)
                                // 2076: b"lower <= upper\0": typeof(_1573 = const b"lower <= upper\x00") = & {l2681} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                                // 2076: b"lower <= upper\0":   l2681 = UNIQUE | NON_NULL, (empty)
                                b"lglddtrace.c\0" as *const u8 as *const libc::c_char,
                                // 2077: b"lglddtrace.c\ ... _char: typeof(_1574) = *const {l2233} i8
                                // 2077: b"lglddtrace.c\ ... _char:   l2233 = UNIQUE | NON_NULL, (empty)
                                // 2077: b"lglddtrace.c\ ... st u8: typeof(_1575) = *const {l2235} u8
                                // 2077: b"lglddtrace.c\ ... st u8:   l2235 = UNIQUE | NON_NULL, (empty)
                                // 2077: b"lglddtrace.c\0": typeof(_1576) = *const {l2237} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 2077: b"lglddtrace.c\0":   l2237 = UNIQUE | NON_NULL, (empty)
                                // 2077: b"lglddtrace.c\0": typeof(_1577) = & {l2239} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 2077: b"lglddtrace.c\0":   l2239 = UNIQUE | NON_NULL, FIXED
                                // 2077: b"lglddtrace.c\0": typeof(_1577 = const b"lglddtrace.c\x00") = & {l2685} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 2077: b"lglddtrace.c\0":   l2685 = UNIQUE | NON_NULL, (empty)
                                // 2077: b"lglddtrace.c\ ... _char: typeof(_1574 = move _1575 as *const i8 (Misc)) = *const {l2688} i8
                                // 2077: b"lglddtrace.c\ ... _char:   l2688 = UNIQUE | NON_NULL, (empty)
                                // 2077: b"lglddtrace.c\0": typeof(_1576 = &raw const (*_1577)) = *const {l2686} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                                // 2077: b"lglddtrace.c\0":   l2686 = UNIQUE | NON_NULL, (empty)
                                // 2077: b"lglddtrace.c\ ... st u8: typeof(_1575 = move _1576 as *const u8 (Pointer(ArrayToPointer))) = *const {l2687} u8
                                // 2077: b"lglddtrace.c\ ... st u8:   l2687 = UNIQUE | NON_NULL, (empty)
                                709 as libc::c_int as libc::c_uint,
                                (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(
                                // 2079: (*::core::mem:: ... ptr(): typeof(_1580) = *const {l2243} i8
                                // 2079: (*::core::mem:: ... ptr():   l2243 = UNIQUE | NON_NULL, (empty)
                                // 2079: (*::core::mem:: ... ptr(): typeof(_1581) = & {l2245} [i8]
                                // 2079: (*::core::mem:: ... ptr():   l2245 = UNIQUE | NON_NULL, FIXED
                                // 2079: (*::core::mem:: ... ptr(): typeof(_1582) = & {l2247} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2079: (*::core::mem:: ... ptr():   l2247 = UNIQUE | NON_NULL, (empty)
                                // 2079: ::core::mem::tr ... 0", ): typeof(_1583) = & {l2249} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2079: ::core::mem::tr ... 0", ):   l2249 = UNIQUE | NON_NULL, FIXED
                                // 2079: (*::core::mem:: ... ptr(): typeof(_1582 = &(*_1583)) = & {l2691} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 2079: (*::core::mem:: ... ptr():   l2691 = UNIQUE | NON_NULL, (empty)
                                // 2079: (*::core::mem:: ... ptr(): typeof(_1581 = move _1582 as &[i8] (Pointer(Unsize))) = & {l2692} [i8]
                                // 2079: (*::core::mem:: ... ptr():   l2692 = UNIQUE | NON_NULL, FIXED
                                    b"void dd(void)\0",
                                    // 2080: b"void dd(void)\0": typeof(_1584) = & {l2251} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2080: b"void dd(void)\0":   l2251 = UNIQUE | NON_NULL, (empty)
                                    // 2080: b"void dd(void)\0": typeof(_1585) = & {l2253} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2080: b"void dd(void)\0":   l2253 = UNIQUE | NON_NULL, FIXED
                                    // 2080: b"void dd(void)\0": typeof(_1584 = &(*_1585)) = & {l2690} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2080: b"void dd(void)\0":   l2690 = UNIQUE | NON_NULL, (empty)
                                    // 2080: b"void dd(void)\0": typeof(_1585 = const b"void dd(void)\x00") = & {l2689} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                    // 2080: b"void dd(void)\0":   l2689 = UNIQUE | NON_NULL, (empty)
                                ))
                                .as_ptr(),
                            );
                        }
                    };
                }
                if verbose > 1 as libc::c_int {
                // 2087: verbose: typeof(_1589) = *mut {l2258} i32
                // 2087: verbose:   l2258 = UNIQUE | NON_NULL, (empty)
                    if reported != 0 {
                        newline();
                    }
                    msg(
                        b"final option %s set to %d\0" as *const u8 as *const libc::c_char,
                        // 2092: b"final option  ... _char: typeof(_1596) = *const {l2266} i8
                        // 2092: b"final option  ... _char:   l2266 = UNIQUE | NON_NULL, (empty)
                        // 2092: b"final option  ... st u8: typeof(_1597) = *const {l2268} u8
                        // 2092: b"final option  ... st u8:   l2268 = UNIQUE | NON_NULL, (empty)
                        // 2092: b"final option  ... %d\0": typeof(_1598) = *const {l2270} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                        // 2092: b"final option  ... %d\0":   l2270 = UNIQUE | NON_NULL, (empty)
                        // 2092: b"final option  ... %d\0": typeof(_1599) = & {l2272} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                        // 2092: b"final option  ... %d\0":   l2272 = UNIQUE | NON_NULL, FIXED
                        // 2092: b"final option  ... _char: typeof(_1596 = move _1597 as *const i8 (Misc)) = *const {l2696} i8
                        // 2092: b"final option  ... _char:   l2696 = UNIQUE | NON_NULL, (empty)
                        // 2092: b"final option  ... %d\0": typeof(_1599 = const b"final option %s set to %d\x00") = & {l2693} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                        // 2092: b"final option  ... %d\0":   l2693 = UNIQUE | NON_NULL, (empty)
                        // 2092: b"final option  ... st u8: typeof(_1597 = move _1598 as *const u8 (Pointer(ArrayToPointer))) = *const {l2695} u8
                        // 2092: b"final option  ... st u8:   l2695 = UNIQUE | NON_NULL, (empty)
                        // 2092: b"final option  ... %d\0": typeof(_1598 = &raw const (*_1599)) = *const {l2694} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                        // 2092: b"final option  ... %d\0":   l2694 = UNIQUE | NON_NULL, (empty)
                        (*o).name,
                        // 2093: (*o).name: typeof(_1600) = *mut {l2274} i8
                        // 2093: (*o).name:   l2274 = UNIQUE | NON_NULL, (empty)
                        (*o).val,
                    );
                    reported = 0 as libc::c_int;
                }
                o = o.offset(1);
                // 2098: o.offset(1): typeof(_1603) = *mut {l2278} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 2098: o.offset(1):   l2278 = UNIQUE | NON_NULL, (empty)
                // 2098: o: typeof(_1604) = *mut {l2280} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 2098: o:   l2280 = UNIQUE | NON_NULL, (empty)
                o;
                // 2099: o: typeof(_1605) = *mut {l2282} DefId(0:362 ~ lglddtrace[7e63]::Opt)
                // 2099: o:   l2282 = UNIQUE | NON_NULL, (empty)
            }
            if reported != 0 && verbose > 1 as libc::c_int {
            // 2101: verbose: typeof(_1614) = *mut {l2292} i32
            // 2101: verbose:   l2292 = UNIQUE | NON_NULL, (empty)
                newline();
            }
        }
        if !(changed != 0) {
            break;
        }
    }
    if verbose != 0 {
    // 2109: verbose: typeof(_1624) = *mut {l2303} i32
    // 2109: verbose:   l2303 = UNIQUE | NON_NULL, (empty)
        newline();
    }
    free(ranges as *mut libc::c_void);
    // 2112: ranges as *mut  ... _void: typeof(_1627) = *mut {l2307} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2112: ranges as *mut  ... _void:   l2307 = UNIQUE | NON_NULL, (empty)
    // 2112: ranges: typeof(_1628) = *mut {l2309} DefId(0:370 ~ lglddtrace[7e63]::Range)
    // 2112: ranges:   l2309 = UNIQUE | NON_NULL, (empty)
    // 2112: ranges as *mut  ... _void: typeof(_1627 = move _1628 as *mut libc::c_void (Misc)) = *mut {l2697} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2112: ranges as *mut  ... _void:   l2697 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn getime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
    // 2120: &mut tv: typeof(_8) = *mut {l8} DefId(0:342 ~ lglddtrace[7e63]::timeval)
    // 2120: &mut tv:   l8 = UNIQUE | NON_NULL, (empty)
    // 2120: &mut tv: typeof(_9) = &mut {l10} DefId(0:342 ~ lglddtrace[7e63]::timeval)
    // 2120: &mut tv:   l10 = UNIQUE | NON_NULL, (empty)
    // 2120: 0 as *mut libc: ... _void: typeof(_10) = *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2120: 0 as *mut libc: ... _void:   l12 = UNIQUE | NON_NULL, (empty)
    // 2120: &mut tv: typeof(_9 = &mut _4) = &mut {l18} DefId(0:342 ~ lglddtrace[7e63]::timeval)
    // 2120: &mut tv:   l18 = UNIQUE | NON_NULL, (empty)
    // 2120: 0 as *mut libc: ... _void: typeof(_10 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2120: 0 as *mut libc: ... _void:   l20 = UNIQUE | NON_NULL, (empty)
    // 2120: &mut tv: typeof(_8 = &raw mut (*_9)) = *mut {l19} DefId(0:342 ~ lglddtrace[7e63]::timeval)
    // 2120: &mut tv:   l19 = UNIQUE | NON_NULL, (empty)
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
// 2126: mut argv: typeof(_2) = *mut {g15} *mut {g16} i8
// 2126: mut argv:   g15 = UNIQUE | NON_NULL, FIXED
// 2126: mut argv:   g16 = UNIQUE | NON_NULL, FIXED
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    let mut close_0: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_int = 0;
    let mut buffer: [libc::c_char; 80] = [0; 80];
    let mut tok: *mut libc::c_char = 0 as *mut libc::c_char;
    // 2133: mut tok: typeof(_10) = *mut {l10} i8
    // 2133: mut tok:   l10 = UNIQUE | NON_NULL, (empty)
    // 2133: 0 as *mut libc: ... _char: typeof(_10 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1461} i8
    // 2133: 0 as *mut libc: ... _char:   l1461 = UNIQUE | NON_NULL, (empty)
    let mut opt: *mut libc::c_char = 0 as *mut libc::c_char;
    // 2134: mut opt: typeof(_11) = *mut {l12} i8
    // 2134: mut opt:   l12 = UNIQUE | NON_NULL, (empty)
    // 2134: 0 as *mut libc: ... _char: typeof(_11 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1462} i8
    // 2134: 0 as *mut libc: ... _char:   l1462 = UNIQUE | NON_NULL, (empty)
    let mut start: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut file: *mut FILE = 0 as *mut FILE;
    // 2137: mut file: typeof(_14) = *mut {l16} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 2137: mut file:   l16 = UNIQUE | NON_NULL, (empty)
    // 2137: 0 as *mut FILE: typeof(_14 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l1463} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 2137: 0 as *mut FILE:   l1463 = UNIQUE | NON_NULL, (empty)
    start = getime();
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    // 2139: mut cmd: typeof(_16) = *mut {l19} i8
    // 2139: mut cmd:   l19 = UNIQUE | NON_NULL, (empty)
    // 2139: 0 as *mut libc: ... _char: typeof(_16 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1464} i8
    // 2139: 0 as *mut libc: ... _char:   l1464 = UNIQUE | NON_NULL, (empty)
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            // 2143: *argv.offset(i  ... size): typeof(_26) = *const {l30} i8
            // 2143: *argv.offset(i  ... size):   l30 = UNIQUE | NON_NULL, (empty)
            // 2143: *argv.offset(i  ... size): typeof(_27) = *mut {l32} i8
            // 2143: *argv.offset(i  ... size):   l32 = UNIQUE | NON_NULL, (empty)
            // 2143: argv.offset(i a ... size): typeof(_28) = *mut {l34} *mut {l35} i8
            // 2143: argv.offset(i a ... size):   l34 = UNIQUE | NON_NULL, (empty)
            // 2143: argv.offset(i a ... size):   l35 = UNIQUE | NON_NULL, (empty)
            // 2143: argv: typeof(_29) = *mut {l37} *mut {l38} i8
            // 2143: argv:   l37 = UNIQUE | NON_NULL, (empty)
            // 2143: argv:   l38 = UNIQUE | NON_NULL, (empty)
            // 2143: *argv.offset(i  ... size): typeof(_26 = move _27 as *const i8 (Pointer(MutToConstPointer))) = *const {l1465} i8
            // 2143: *argv.offset(i  ... size):   l1465 = UNIQUE | NON_NULL, (empty)
            b"-h\0" as *const u8 as *const libc::c_char,
            // 2144: b"-h\0" as *con ... _char: typeof(_32) = *const {l42} i8
            // 2144: b"-h\0" as *con ... _char:   l42 = UNIQUE | NON_NULL, (empty)
            // 2144: b"-h\0" as *const u8: typeof(_33) = *const {l44} u8
            // 2144: b"-h\0" as *const u8:   l44 = UNIQUE | NON_NULL, (empty)
            // 2144: b"-h\0": typeof(_34) = *const {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2144: b"-h\0":   l46 = UNIQUE | NON_NULL, (empty)
            // 2144: b"-h\0": typeof(_35) = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2144: b"-h\0":   l48 = UNIQUE | NON_NULL, FIXED
            // 2144: b"-h\0" as *const u8: typeof(_33 = move _34 as *const u8 (Pointer(ArrayToPointer))) = *const {l1468} u8
            // 2144: b"-h\0" as *const u8:   l1468 = UNIQUE | NON_NULL, (empty)
            // 2144: b"-h\0" as *con ... _char: typeof(_32 = move _33 as *const i8 (Misc)) = *const {l1469} i8
            // 2144: b"-h\0" as *con ... _char:   l1469 = UNIQUE | NON_NULL, (empty)
            // 2144: b"-h\0": typeof(_35 = const b"-h\x00") = & {l1466} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2144: b"-h\0":   l1466 = UNIQUE | NON_NULL, (empty)
            // 2144: b"-h\0": typeof(_34 = &raw const (*_35)) = *const {l1467} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2144: b"-h\0":   l1467 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(
                b"usage: lglddtrace [-h][-v][-s][-d][-f][-O] <in>[.gz] <out>[.gz]\n\0" as *const u8
                // 2148: b"usage: lglddt ... _char: typeof(_38) = *const {l52} i8
                // 2148: b"usage: lglddt ... _char:   l52 = UNIQUE | NON_NULL, (empty)
                // 2148: b"usage: lglddt ... st u8: typeof(_39) = *const {l54} u8
                // 2148: b"usage: lglddt ... st u8:   l54 = UNIQUE | NON_NULL, (empty)
                // 2148: b"usage: lglddt ... \n\0": typeof(_40) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2148: b"usage: lglddt ... \n\0":   l56 = UNIQUE | NON_NULL, (empty)
                // 2148: b"usage: lglddt ... \n\0": typeof(_41) = & {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2148: b"usage: lglddt ... \n\0":   l58 = UNIQUE | NON_NULL, FIXED
                // 2148: b"usage: lglddt ... \n\0": typeof(_41 = const b"usage: lglddtrace [-h][-v][-s][-d][-f][-O] <in>[.gz] <out>[.gz]\n\x00") = & {l1470} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2148: b"usage: lglddt ... \n\0":   l1470 = UNIQUE | NON_NULL, (empty)
                // 2148: b"usage: lglddt ... \n\0": typeof(_40 = &raw const (*_41)) = *const {l1471} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000041)) }]
                // 2148: b"usage: lglddt ... \n\0":   l1471 = UNIQUE | NON_NULL, (empty)
                // 2148: b"usage: lglddt ... st u8: typeof(_39 = move _40 as *const u8 (Pointer(ArrayToPointer))) = *const {l1472} u8
                // 2148: b"usage: lglddt ... st u8:   l1472 = UNIQUE | NON_NULL, (empty)
                // 2148: b"usage: lglddt ... _char: typeof(_38 = move _39 as *const i8 (Misc)) = *const {l1473} i8
                // 2148: b"usage: lglddt ... _char:   l1473 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            // 2153: *argv.offset(i  ... size): typeof(_46) = *const {l64} i8
            // 2153: *argv.offset(i  ... size):   l64 = UNIQUE | NON_NULL, (empty)
            // 2153: *argv.offset(i  ... size): typeof(_47) = *mut {l66} i8
            // 2153: *argv.offset(i  ... size):   l66 = UNIQUE | NON_NULL, (empty)
            // 2153: argv.offset(i a ... size): typeof(_48) = *mut {l68} *mut {l69} i8
            // 2153: argv.offset(i a ... size):   l68 = UNIQUE | NON_NULL, (empty)
            // 2153: argv.offset(i a ... size):   l69 = UNIQUE | NON_NULL, (empty)
            // 2153: argv: typeof(_49) = *mut {l71} *mut {l72} i8
            // 2153: argv:   l71 = UNIQUE | NON_NULL, (empty)
            // 2153: argv:   l72 = UNIQUE | NON_NULL, (empty)
            // 2153: *argv.offset(i  ... size): typeof(_46 = move _47 as *const i8 (Pointer(MutToConstPointer))) = *const {l1474} i8
            // 2153: *argv.offset(i  ... size):   l1474 = UNIQUE | NON_NULL, (empty)
            b"-v\0" as *const u8 as *const libc::c_char,
            // 2154: b"-v\0" as *con ... _char: typeof(_52) = *const {l76} i8
            // 2154: b"-v\0" as *con ... _char:   l76 = UNIQUE | NON_NULL, (empty)
            // 2154: b"-v\0" as *const u8: typeof(_53) = *const {l78} u8
            // 2154: b"-v\0" as *const u8:   l78 = UNIQUE | NON_NULL, (empty)
            // 2154: b"-v\0": typeof(_54) = *const {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2154: b"-v\0":   l80 = UNIQUE | NON_NULL, (empty)
            // 2154: b"-v\0": typeof(_55) = & {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2154: b"-v\0":   l82 = UNIQUE | NON_NULL, FIXED
            // 2154: b"-v\0" as *const u8: typeof(_53 = move _54 as *const u8 (Pointer(ArrayToPointer))) = *const {l1477} u8
            // 2154: b"-v\0" as *const u8:   l1477 = UNIQUE | NON_NULL, (empty)
            // 2154: b"-v\0": typeof(_55 = const b"-v\x00") = & {l1475} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2154: b"-v\0":   l1475 = UNIQUE | NON_NULL, (empty)
            // 2154: b"-v\0": typeof(_54 = &raw const (*_55)) = *const {l1476} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2154: b"-v\0":   l1476 = UNIQUE | NON_NULL, (empty)
            // 2154: b"-v\0" as *con ... _char: typeof(_52 = move _53 as *const i8 (Misc)) = *const {l1478} i8
            // 2154: b"-v\0" as *con ... _char:   l1478 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            verbose += 1;
            // 2157: verbose: typeof(_56) = *mut {l84} i32
            // 2157: verbose:   l84 = UNIQUE | NON_NULL, (empty)
            verbose;
            // 2158: verbose: typeof(_59) = *mut {l88} i32
            // 2158: verbose:   l88 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2160: *argv.offset(i  ... size): typeof(_62) = *const {l92} i8
            // 2160: *argv.offset(i  ... size):   l92 = UNIQUE | NON_NULL, (empty)
            // 2160: *argv.offset(i  ... size): typeof(_63) = *mut {l94} i8
            // 2160: *argv.offset(i  ... size):   l94 = UNIQUE | NON_NULL, (empty)
            // 2160: argv.offset(i a ... size): typeof(_64) = *mut {l96} *mut {l97} i8
            // 2160: argv.offset(i a ... size):   l96 = UNIQUE | NON_NULL, (empty)
            // 2160: argv.offset(i a ... size):   l97 = UNIQUE | NON_NULL, (empty)
            // 2160: argv: typeof(_65) = *mut {l99} *mut {l100} i8
            // 2160: argv:   l99 = UNIQUE | NON_NULL, (empty)
            // 2160: argv:   l100 = UNIQUE | NON_NULL, (empty)
            // 2160: *argv.offset(i  ... size): typeof(_62 = move _63 as *const i8 (Pointer(MutToConstPointer))) = *const {l1479} i8
            // 2160: *argv.offset(i  ... size):   l1479 = UNIQUE | NON_NULL, (empty)
            b"-c\0" as *const u8 as *const libc::c_char,
            // 2161: b"-c\0" as *con ... _char: typeof(_68) = *const {l104} i8
            // 2161: b"-c\0" as *con ... _char:   l104 = UNIQUE | NON_NULL, (empty)
            // 2161: b"-c\0" as *const u8: typeof(_69) = *const {l106} u8
            // 2161: b"-c\0" as *const u8:   l106 = UNIQUE | NON_NULL, (empty)
            // 2161: b"-c\0": typeof(_70) = *const {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2161: b"-c\0":   l108 = UNIQUE | NON_NULL, (empty)
            // 2161: b"-c\0": typeof(_71) = & {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2161: b"-c\0":   l110 = UNIQUE | NON_NULL, FIXED
            // 2161: b"-c\0" as *const u8: typeof(_69 = move _70 as *const u8 (Pointer(ArrayToPointer))) = *const {l1482} u8
            // 2161: b"-c\0" as *const u8:   l1482 = UNIQUE | NON_NULL, (empty)
            // 2161: b"-c\0" as *con ... _char: typeof(_68 = move _69 as *const i8 (Misc)) = *const {l1483} i8
            // 2161: b"-c\0" as *con ... _char:   l1483 = UNIQUE | NON_NULL, (empty)
            // 2161: b"-c\0": typeof(_70 = &raw const (*_71)) = *const {l1481} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2161: b"-c\0":   l1481 = UNIQUE | NON_NULL, (empty)
            // 2161: b"-c\0": typeof(_71 = const b"-c\x00") = & {l1480} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2161: b"-c\0":   l1480 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            checkreturn += 1;
            // 2164: checkreturn: typeof(_72) = *mut {l112} i32
            // 2164: checkreturn:   l112 = UNIQUE | NON_NULL, (empty)
            checkreturn;
            // 2165: checkreturn: typeof(_75) = *mut {l116} i32
            // 2165: checkreturn:   l116 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2167: *argv.offset(i  ... size): typeof(_78) = *const {l120} i8
            // 2167: *argv.offset(i  ... size):   l120 = UNIQUE | NON_NULL, (empty)
            // 2167: *argv.offset(i  ... size): typeof(_79) = *mut {l122} i8
            // 2167: *argv.offset(i  ... size):   l122 = UNIQUE | NON_NULL, (empty)
            // 2167: argv.offset(i a ... size): typeof(_80) = *mut {l124} *mut {l125} i8
            // 2167: argv.offset(i a ... size):   l124 = UNIQUE | NON_NULL, (empty)
            // 2167: argv.offset(i a ... size):   l125 = UNIQUE | NON_NULL, (empty)
            // 2167: argv: typeof(_81) = *mut {l127} *mut {l128} i8
            // 2167: argv:   l127 = UNIQUE | NON_NULL, (empty)
            // 2167: argv:   l128 = UNIQUE | NON_NULL, (empty)
            // 2167: *argv.offset(i  ... size): typeof(_78 = move _79 as *const i8 (Pointer(MutToConstPointer))) = *const {l1484} i8
            // 2167: *argv.offset(i  ... size):   l1484 = UNIQUE | NON_NULL, (empty)
            b"-O\0" as *const u8 as *const libc::c_char,
            // 2168: b"-O\0" as *con ... _char: typeof(_84) = *const {l132} i8
            // 2168: b"-O\0" as *con ... _char:   l132 = UNIQUE | NON_NULL, (empty)
            // 2168: b"-O\0" as *const u8: typeof(_85) = *const {l134} u8
            // 2168: b"-O\0" as *const u8:   l134 = UNIQUE | NON_NULL, (empty)
            // 2168: b"-O\0": typeof(_86) = *const {l136} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2168: b"-O\0":   l136 = UNIQUE | NON_NULL, (empty)
            // 2168: b"-O\0": typeof(_87) = & {l138} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2168: b"-O\0":   l138 = UNIQUE | NON_NULL, FIXED
            // 2168: b"-O\0": typeof(_86 = &raw const (*_87)) = *const {l1486} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2168: b"-O\0":   l1486 = UNIQUE | NON_NULL, (empty)
            // 2168: b"-O\0" as *con ... _char: typeof(_84 = move _85 as *const i8 (Misc)) = *const {l1488} i8
            // 2168: b"-O\0" as *con ... _char:   l1488 = UNIQUE | NON_NULL, (empty)
            // 2168: b"-O\0" as *const u8: typeof(_85 = move _86 as *const u8 (Pointer(ArrayToPointer))) = *const {l1487} u8
            // 2168: b"-O\0" as *const u8:   l1487 = UNIQUE | NON_NULL, (empty)
            // 2168: b"-O\0": typeof(_87 = const b"-O\x00") = & {l1485} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2168: b"-O\0":   l1485 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            ddopts += 1;
            // 2171: ddopts: typeof(_88) = *mut {l140} i32
            // 2171: ddopts:   l140 = UNIQUE | NON_NULL, (empty)
            ddopts;
            // 2172: ddopts: typeof(_91) = *mut {l144} i32
            // 2172: ddopts:   l144 = UNIQUE | NON_NULL, (empty)
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        // 2173: (*argv.offset(i ... size): typeof(_95) = *mut {l149} i8
        // 2173: (*argv.offset(i ... size):   l149 = UNIQUE | NON_NULL, (empty)
        // 2173: (*argv.offset(i ... ize)): typeof(_96) = *mut {l151} i8
        // 2173: (*argv.offset(i ... ize)):   l151 = UNIQUE | NON_NULL, (empty)
        // 2173: argv.offset(i a ... size): typeof(_97) = *mut {l153} *mut {l154} i8
        // 2173: argv.offset(i a ... size):   l153 = UNIQUE | NON_NULL, (empty)
        // 2173: argv.offset(i a ... size):   l154 = UNIQUE | NON_NULL, (empty)
        // 2173: argv: typeof(_98) = *mut {l156} *mut {l157} i8
        // 2173: argv:   l156 = UNIQUE | NON_NULL, (empty)
        // 2173: argv:   l157 = UNIQUE | NON_NULL, (empty)
            == '-' as i32
        {
            die(
                b"invalid command line option '%s' (try '-h')\0" as *const u8
                // 2177: b"invalid comma ... _char: typeof(_105) = *const {l165} i8
                // 2177: b"invalid comma ... _char:   l165 = UNIQUE | NON_NULL, (empty)
                // 2177: b"invalid comma ... st u8: typeof(_106) = *const {l167} u8
                // 2177: b"invalid comma ... st u8:   l167 = UNIQUE | NON_NULL, (empty)
                // 2177: b"invalid comma ... ')\0": typeof(_107) = *const {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2177: b"invalid comma ... ')\0":   l169 = UNIQUE | NON_NULL, (empty)
                // 2177: b"invalid comma ... ')\0": typeof(_108) = & {l171} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2177: b"invalid comma ... ')\0":   l171 = UNIQUE | NON_NULL, FIXED
                // 2177: b"invalid comma ... ')\0": typeof(_107 = &raw const (*_108)) = *const {l1490} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2177: b"invalid comma ... ')\0":   l1490 = UNIQUE | NON_NULL, (empty)
                // 2177: b"invalid comma ... st u8: typeof(_106 = move _107 as *const u8 (Pointer(ArrayToPointer))) = *const {l1491} u8
                // 2177: b"invalid comma ... st u8:   l1491 = UNIQUE | NON_NULL, (empty)
                // 2177: b"invalid comma ... ')\0": typeof(_108 = const b"invalid command line option \'%s\' (try \'-h\')\x00") = & {l1489} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2177: b"invalid comma ... ')\0":   l1489 = UNIQUE | NON_NULL, (empty)
                // 2177: b"invalid comma ... _char: typeof(_105 = move _106 as *const i8 (Misc)) = *const {l1492} i8
                // 2177: b"invalid comma ... _char:   l1492 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                *argv.offset(i as isize),
                // 2179: *argv.offset(i  ... size): typeof(_109) = *mut {l173} i8
                // 2179: *argv.offset(i  ... size):   l173 = UNIQUE | NON_NULL, (empty)
                // 2179: argv.offset(i a ... size): typeof(_110) = *mut {l175} *mut {l176} i8
                // 2179: argv.offset(i a ... size):   l175 = UNIQUE | NON_NULL, (empty)
                // 2179: argv.offset(i a ... size):   l176 = UNIQUE | NON_NULL, (empty)
                // 2179: argv: typeof(_111) = *mut {l178} *mut {l179} i8
                // 2179: argv:   l178 = UNIQUE | NON_NULL, (empty)
                // 2179: argv:   l179 = UNIQUE | NON_NULL, (empty)
            );
        } else if !oname.is_null() {
        // 2181: oname: typeof(_116) = *const {l185} i8
        // 2181: oname:   l185 = UNIQUE | NON_NULL, (empty)
        // 2181: oname: typeof(_117) = *mut {l187} *const {l188} i8
        // 2181: oname:   l187 = UNIQUE | NON_NULL, (empty)
        // 2181: oname:   l188 = UNIQUE | NON_NULL, (empty)
            die(b"two many options pecified (try '-h')\0" as *const u8 as *const libc::c_char);
            // 2182: b"two many opti ... _char: typeof(_119) = *const {l191} i8
            // 2182: b"two many opti ... _char:   l191 = UNIQUE | NON_NULL, (empty)
            // 2182: b"two many opti ... st u8: typeof(_120) = *const {l193} u8
            // 2182: b"two many opti ... st u8:   l193 = UNIQUE | NON_NULL, (empty)
            // 2182: b"two many opti ... ')\0": typeof(_121) = *const {l195} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 2182: b"two many opti ... ')\0":   l195 = UNIQUE | NON_NULL, (empty)
            // 2182: b"two many opti ... ')\0": typeof(_122) = & {l197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 2182: b"two many opti ... ')\0":   l197 = UNIQUE | NON_NULL, FIXED
            // 2182: b"two many opti ... _char: typeof(_119 = move _120 as *const i8 (Misc)) = *const {l1496} i8
            // 2182: b"two many opti ... _char:   l1496 = UNIQUE | NON_NULL, (empty)
            // 2182: b"two many opti ... st u8: typeof(_120 = move _121 as *const u8 (Pointer(ArrayToPointer))) = *const {l1495} u8
            // 2182: b"two many opti ... st u8:   l1495 = UNIQUE | NON_NULL, (empty)
            // 2182: b"two many opti ... ')\0": typeof(_121 = &raw const (*_122)) = *const {l1494} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 2182: b"two many opti ... ')\0":   l1494 = UNIQUE | NON_NULL, (empty)
            // 2182: b"two many opti ... ')\0": typeof(_122 = const b"two many options pecified (try \'-h\')\x00") = & {l1493} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 2182: b"two many opti ... ')\0":   l1493 = UNIQUE | NON_NULL, (empty)
        } else if !iname.is_null() {
        // 2183: iname: typeof(_125) = *const {l201} i8
        // 2183: iname:   l201 = UNIQUE | NON_NULL, (empty)
        // 2183: iname: typeof(_126) = *mut {l203} *const {l204} i8
        // 2183: iname:   l203 = UNIQUE | NON_NULL, (empty)
        // 2183: iname:   l204 = UNIQUE | NON_NULL, (empty)
            oname = *argv.offset(i as isize);
            // 2184: *argv.offset(i  ... size): typeof(_127) = *mut {l206} i8
            // 2184: *argv.offset(i  ... size):   l206 = UNIQUE | NON_NULL, (empty)
            // 2184: argv.offset(i a ... size): typeof(_128) = *mut {l208} *mut {l209} i8
            // 2184: argv.offset(i a ... size):   l208 = UNIQUE | NON_NULL, (empty)
            // 2184: argv.offset(i a ... size):   l209 = UNIQUE | NON_NULL, (empty)
            // 2184: argv: typeof(_129) = *mut {l211} *mut {l212} i8
            // 2184: argv:   l211 = UNIQUE | NON_NULL, (empty)
            // 2184: argv:   l212 = UNIQUE | NON_NULL, (empty)
            // 2184: oname: typeof(_132) = *mut {l216} *const {l217} i8
            // 2184: oname:   l216 = UNIQUE | NON_NULL, (empty)
            // 2184: oname:   l217 = UNIQUE | NON_NULL, (empty)
            // 2184: oname = *argv.o ... size): typeof((*_132) = move _127 as *const i8 (Pointer(MutToConstPointer))) = *const {l1497} i8
            // 2184: oname = *argv.o ... size):   l1497 = UNIQUE | NON_NULL, (empty)
        } else {
            iname = *argv.offset(i as isize);
            // 2186: *argv.offset(i  ... size): typeof(_133) = *mut {l219} i8
            // 2186: *argv.offset(i  ... size):   l219 = UNIQUE | NON_NULL, (empty)
            // 2186: argv.offset(i a ... size): typeof(_134) = *mut {l221} *mut {l222} i8
            // 2186: argv.offset(i a ... size):   l221 = UNIQUE | NON_NULL, (empty)
            // 2186: argv.offset(i a ... size):   l222 = UNIQUE | NON_NULL, (empty)
            // 2186: argv: typeof(_135) = *mut {l224} *mut {l225} i8
            // 2186: argv:   l224 = UNIQUE | NON_NULL, (empty)
            // 2186: argv:   l225 = UNIQUE | NON_NULL, (empty)
            // 2186: iname: typeof(_138) = *mut {l229} *const {l230} i8
            // 2186: iname:   l229 = UNIQUE | NON_NULL, (empty)
            // 2186: iname:   l230 = UNIQUE | NON_NULL, (empty)
            // 2186: iname = *argv.o ... size): typeof((*_138) = move _133 as *const i8 (Pointer(MutToConstPointer))) = *const {l1498} i8
            // 2186: iname = *argv.o ... size):   l1498 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    if iname.is_null() {
    // 2191: iname: typeof(_146) = *const {l239} i8
    // 2191: iname:   l239 = UNIQUE | NON_NULL, (empty)
    // 2191: iname: typeof(_147) = *mut {l241} *const {l242} i8
    // 2191: iname:   l241 = UNIQUE | NON_NULL, (empty)
    // 2191: iname:   l242 = UNIQUE | NON_NULL, (empty)
        die(b"input file missing\0" as *const u8 as *const libc::c_char);
        // 2192: b"input file mi ... _char: typeof(_149) = *const {l245} i8
        // 2192: b"input file mi ... _char:   l245 = UNIQUE | NON_NULL, (empty)
        // 2192: b"input file mi ... st u8: typeof(_150) = *const {l247} u8
        // 2192: b"input file mi ... st u8:   l247 = UNIQUE | NON_NULL, (empty)
        // 2192: b"input file mi ... ng\0": typeof(_151) = *const {l249} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 2192: b"input file mi ... ng\0":   l249 = UNIQUE | NON_NULL, (empty)
        // 2192: b"input file mi ... ng\0": typeof(_152) = & {l251} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 2192: b"input file mi ... ng\0":   l251 = UNIQUE | NON_NULL, FIXED
        // 2192: b"input file mi ... st u8: typeof(_150 = move _151 as *const u8 (Pointer(ArrayToPointer))) = *const {l1501} u8
        // 2192: b"input file mi ... st u8:   l1501 = UNIQUE | NON_NULL, (empty)
        // 2192: b"input file mi ... ng\0": typeof(_152 = const b"input file missing\x00") = & {l1499} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 2192: b"input file mi ... ng\0":   l1499 = UNIQUE | NON_NULL, (empty)
        // 2192: b"input file mi ... _char: typeof(_149 = move _150 as *const i8 (Misc)) = *const {l1502} i8
        // 2192: b"input file mi ... _char:   l1502 = UNIQUE | NON_NULL, (empty)
        // 2192: b"input file mi ... ng\0": typeof(_151 = &raw const (*_152)) = *const {l1500} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 2192: b"input file mi ... ng\0":   l1500 = UNIQUE | NON_NULL, (empty)
    }
    if oname.is_null() {
    // 2194: oname: typeof(_155) = *const {l255} i8
    // 2194: oname:   l255 = UNIQUE | NON_NULL, (empty)
    // 2194: oname: typeof(_156) = *mut {l257} *const {l258} i8
    // 2194: oname:   l257 = UNIQUE | NON_NULL, (empty)
    // 2194: oname:   l258 = UNIQUE | NON_NULL, (empty)
        die(b"output file missing\0" as *const u8 as *const libc::c_char);
        // 2195: b"output file m ... _char: typeof(_158) = *const {l261} i8
        // 2195: b"output file m ... _char:   l261 = UNIQUE | NON_NULL, (empty)
        // 2195: b"output file m ... st u8: typeof(_159) = *const {l263} u8
        // 2195: b"output file m ... st u8:   l263 = UNIQUE | NON_NULL, (empty)
        // 2195: b"output file m ... ng\0": typeof(_160) = *const {l265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 2195: b"output file m ... ng\0":   l265 = UNIQUE | NON_NULL, (empty)
        // 2195: b"output file m ... ng\0": typeof(_161) = & {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 2195: b"output file m ... ng\0":   l267 = UNIQUE | NON_NULL, FIXED
        // 2195: b"output file m ... st u8: typeof(_159 = move _160 as *const u8 (Pointer(ArrayToPointer))) = *const {l1505} u8
        // 2195: b"output file m ... st u8:   l1505 = UNIQUE | NON_NULL, (empty)
        // 2195: b"output file m ... ng\0": typeof(_161 = const b"output file missing\x00") = & {l1503} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 2195: b"output file m ... ng\0":   l1503 = UNIQUE | NON_NULL, (empty)
        // 2195: b"output file m ... ng\0": typeof(_160 = &raw const (*_161)) = *const {l1504} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
        // 2195: b"output file m ... ng\0":   l1504 = UNIQUE | NON_NULL, (empty)
        // 2195: b"output file m ... _char: typeof(_158 = move _159 as *const i8 (Misc)) = *const {l1506} i8
        // 2195: b"output file m ... _char:   l1506 = UNIQUE | NON_NULL, (empty)
    }
    len = strlen(iname) as libc::c_int;
    // 2197: iname: typeof(_163) = *const {l270} i8
    // 2197: iname:   l270 = UNIQUE | NON_NULL, (empty)
    // 2197: iname: typeof(_164) = *mut {l272} *const {l273} i8
    // 2197: iname:   l272 = UNIQUE | NON_NULL, (empty)
    // 2197: iname:   l273 = UNIQUE | NON_NULL, (empty)
    if len >= 3 as libc::c_int
        && strcmp(
            iname
            // 2200: iname .offset(l ... ize)): typeof(_172) = *const {l282} i8
            // 2200: iname .offset(l ... ize)):   l282 = UNIQUE | NON_NULL, (empty)
            // 2200: iname .offset(l ... size): typeof(_173) = *const {l284} i8
            // 2200: iname .offset(l ... size):   l284 = UNIQUE | NON_NULL, (empty)
            // 2200: iname: typeof(_174) = *const {l286} i8
            // 2200: iname:   l286 = UNIQUE | NON_NULL, (empty)
            // 2200: iname: typeof(_175) = *mut {l288} *const {l289} i8
            // 2200: iname:   l288 = UNIQUE | NON_NULL, (empty)
            // 2200: iname:   l289 = UNIQUE | NON_NULL, (empty)
                .offset(len as isize)
                .offset(-(3 as libc::c_int as isize)),
            b".gz\0" as *const u8 as *const libc::c_char,
            // 2203: b".gz\0" as *co ... _char: typeof(_182) = *const {l297} i8
            // 2203: b".gz\0" as *co ... _char:   l297 = UNIQUE | NON_NULL, (empty)
            // 2203: b".gz\0" as *co ... st u8: typeof(_183) = *const {l299} u8
            // 2203: b".gz\0" as *co ... st u8:   l299 = UNIQUE | NON_NULL, (empty)
            // 2203: b".gz\0": typeof(_184) = *const {l301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2203: b".gz\0":   l301 = UNIQUE | NON_NULL, (empty)
            // 2203: b".gz\0": typeof(_185) = & {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2203: b".gz\0":   l303 = UNIQUE | NON_NULL, FIXED
            // 2203: b".gz\0" as *co ... _char: typeof(_182 = move _183 as *const i8 (Misc)) = *const {l1510} i8
            // 2203: b".gz\0" as *co ... _char:   l1510 = UNIQUE | NON_NULL, (empty)
            // 2203: b".gz\0": typeof(_185 = const b".gz\x00") = & {l1507} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2203: b".gz\0":   l1507 = UNIQUE | NON_NULL, (empty)
            // 2203: b".gz\0": typeof(_184 = &raw const (*_185)) = *const {l1508} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2203: b".gz\0":   l1508 = UNIQUE | NON_NULL, (empty)
            // 2203: b".gz\0" as *co ... st u8: typeof(_183 = move _184 as *const u8 (Pointer(ArrayToPointer))) = *const {l1509} u8
            // 2203: b".gz\0" as *co ... st u8:   l1509 = UNIQUE | NON_NULL, (empty)
        ) == 0
    {
        cmd = malloc((len + 20 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
        // 2206: malloc((len + 2 ... long): typeof(_186) = *mut {l305} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2206: malloc((len + 2 ... long):   l305 = UNIQUE | NON_NULL, (empty)
        // 2206: cmd = malloc((l ... _char: typeof(_16 = move _186 as *mut i8 (Misc)) = *mut {l1511} i8
        // 2206: cmd = malloc((l ... _char:   l1511 = UNIQUE | NON_NULL, (empty)
        sprintf(
            cmd,
            // 2208: cmd: typeof(_193) = *mut {l313} i8
            // 2208: cmd:   l313 = UNIQUE | NON_NULL, (empty)
            b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
            // 2209: b"gunzip -c %s\ ... _char: typeof(_194) = *const {l315} i8
            // 2209: b"gunzip -c %s\ ... _char:   l315 = UNIQUE | NON_NULL, (empty)
            // 2209: b"gunzip -c %s\ ... st u8: typeof(_195) = *const {l317} u8
            // 2209: b"gunzip -c %s\ ... st u8:   l317 = UNIQUE | NON_NULL, (empty)
            // 2209: b"gunzip -c %s\0": typeof(_196) = *const {l319} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2209: b"gunzip -c %s\0":   l319 = UNIQUE | NON_NULL, (empty)
            // 2209: b"gunzip -c %s\0": typeof(_197) = & {l321} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2209: b"gunzip -c %s\0":   l321 = UNIQUE | NON_NULL, FIXED
            // 2209: b"gunzip -c %s\0": typeof(_197 = const b"gunzip -c %s\x00") = & {l1512} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2209: b"gunzip -c %s\0":   l1512 = UNIQUE | NON_NULL, (empty)
            // 2209: b"gunzip -c %s\ ... _char: typeof(_194 = move _195 as *const i8 (Misc)) = *const {l1515} i8
            // 2209: b"gunzip -c %s\ ... _char:   l1515 = UNIQUE | NON_NULL, (empty)
            // 2209: b"gunzip -c %s\0": typeof(_196 = &raw const (*_197)) = *const {l1513} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2209: b"gunzip -c %s\0":   l1513 = UNIQUE | NON_NULL, (empty)
            // 2209: b"gunzip -c %s\ ... st u8: typeof(_195 = move _196 as *const u8 (Pointer(ArrayToPointer))) = *const {l1514} u8
            // 2209: b"gunzip -c %s\ ... st u8:   l1514 = UNIQUE | NON_NULL, (empty)
            iname,
            // 2210: iname: typeof(_198) = *const {l323} i8
            // 2210: iname:   l323 = UNIQUE | NON_NULL, (empty)
            // 2210: iname: typeof(_199) = *mut {l325} *const {l326} i8
            // 2210: iname:   l325 = UNIQUE | NON_NULL, (empty)
            // 2210: iname:   l326 = UNIQUE | NON_NULL, (empty)
        );
        file = popen(cmd, b"r\0" as *const u8 as *const libc::c_char);
        // 2212: popen(cmd, b"r\ ... char): typeof(_200) = *mut {l328} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2212: popen(cmd, b"r\ ... char):   l328 = UNIQUE | NON_NULL, (empty)
        // 2212: cmd: typeof(_201) = *const {l330} i8
        // 2212: cmd:   l330 = UNIQUE | NON_NULL, (empty)
        // 2212: cmd: typeof(_202) = *mut {l332} i8
        // 2212: cmd:   l332 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0" as *cons ... _char: typeof(_203) = *const {l334} i8
        // 2212: b"r\0" as *cons ... _char:   l334 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0" as *const u8: typeof(_204) = *const {l336} u8
        // 2212: b"r\0" as *const u8:   l336 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0": typeof(_205) = *const {l338} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2212: b"r\0":   l338 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0": typeof(_206) = & {l340} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2212: b"r\0":   l340 = UNIQUE | NON_NULL, FIXED
        // 2212: b"r\0": typeof(_206 = const b"r\x00") = & {l1517} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2212: b"r\0":   l1517 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0": typeof(_205 = &raw const (*_206)) = *const {l1518} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2212: b"r\0":   l1518 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0" as *const u8: typeof(_204 = move _205 as *const u8 (Pointer(ArrayToPointer))) = *const {l1519} u8
        // 2212: b"r\0" as *const u8:   l1519 = UNIQUE | NON_NULL, (empty)
        // 2212: cmd: typeof(_201 = move _202 as *const i8 (Pointer(MutToConstPointer))) = *const {l1516} i8
        // 2212: cmd:   l1516 = UNIQUE | NON_NULL, (empty)
        // 2212: b"r\0" as *cons ... _char: typeof(_203 = move _204 as *const i8 (Misc)) = *const {l1520} i8
        // 2212: b"r\0" as *cons ... _char:   l1520 = UNIQUE | NON_NULL, (empty)
        free(cmd as *mut libc::c_void);
        // 2213: cmd as *mut lib ... _void: typeof(_208) = *mut {l343} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2213: cmd as *mut lib ... _void:   l343 = UNIQUE | NON_NULL, (empty)
        // 2213: cmd: typeof(_209) = *mut {l345} i8
        // 2213: cmd:   l345 = UNIQUE | NON_NULL, (empty)
        // 2213: cmd as *mut lib ... _void: typeof(_208 = move _209 as *mut libc::c_void (Misc)) = *mut {l1521} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2213: cmd as *mut lib ... _void:   l1521 = UNIQUE | NON_NULL, (empty)
        if !file.is_null() {
        // 2214: file: typeof(_212) = *mut {l349} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2214: file:   l349 = UNIQUE | NON_NULL, (empty)
            close_0 = 2 as libc::c_int;
        }
    } else {
        file = fopen(iname, b"r\0" as *const u8 as *const libc::c_char);
        // 2218: fopen(iname, b" ... char): typeof(_214) = *mut {l352} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2218: fopen(iname, b" ... char):   l352 = UNIQUE | NON_NULL, (empty)
        // 2218: iname: typeof(_215) = *const {l354} i8
        // 2218: iname:   l354 = UNIQUE | NON_NULL, (empty)
        // 2218: iname: typeof(_216) = *mut {l356} *const {l357} i8
        // 2218: iname:   l356 = UNIQUE | NON_NULL, (empty)
        // 2218: iname:   l357 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0" as *cons ... _char: typeof(_217) = *const {l359} i8
        // 2218: b"r\0" as *cons ... _char:   l359 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0" as *const u8: typeof(_218) = *const {l361} u8
        // 2218: b"r\0" as *const u8:   l361 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0": typeof(_219) = *const {l363} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2218: b"r\0":   l363 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0": typeof(_220) = & {l365} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2218: b"r\0":   l365 = UNIQUE | NON_NULL, FIXED
        // 2218: b"r\0": typeof(_220 = const b"r\x00") = & {l1522} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2218: b"r\0":   l1522 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0": typeof(_219 = &raw const (*_220)) = *const {l1523} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2218: b"r\0":   l1523 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0" as *const u8: typeof(_218 = move _219 as *const u8 (Pointer(ArrayToPointer))) = *const {l1524} u8
        // 2218: b"r\0" as *const u8:   l1524 = UNIQUE | NON_NULL, (empty)
        // 2218: b"r\0" as *cons ... _char: typeof(_217 = move _218 as *const i8 (Misc)) = *const {l1525} i8
        // 2218: b"r\0" as *cons ... _char:   l1525 = UNIQUE | NON_NULL, (empty)
        if !file.is_null() {
        // 2219: file: typeof(_223) = *mut {l369} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2219: file:   l369 = UNIQUE | NON_NULL, (empty)
            close_0 = 1 as libc::c_int;
        }
    }
    if file.is_null() {
    // 2223: file: typeof(_227) = *mut {l374} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
    // 2223: file:   l374 = UNIQUE | NON_NULL, (empty)
        die(
            b"can not read '%s'\0" as *const u8 as *const libc::c_char,
            // 2225: b"can not read  ... _char: typeof(_229) = *const {l377} i8
            // 2225: b"can not read  ... _char:   l377 = UNIQUE | NON_NULL, (empty)
            // 2225: b"can not read  ... st u8: typeof(_230) = *const {l379} u8
            // 2225: b"can not read  ... st u8:   l379 = UNIQUE | NON_NULL, (empty)
            // 2225: b"can not read  ... s'\0": typeof(_231) = *const {l381} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 2225: b"can not read  ... s'\0":   l381 = UNIQUE | NON_NULL, (empty)
            // 2225: b"can not read  ... s'\0": typeof(_232) = & {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 2225: b"can not read  ... s'\0":   l383 = UNIQUE | NON_NULL, FIXED
            // 2225: b"can not read  ... s'\0": typeof(_231 = &raw const (*_232)) = *const {l1527} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 2225: b"can not read  ... s'\0":   l1527 = UNIQUE | NON_NULL, (empty)
            // 2225: b"can not read  ... st u8: typeof(_230 = move _231 as *const u8 (Pointer(ArrayToPointer))) = *const {l1528} u8
            // 2225: b"can not read  ... st u8:   l1528 = UNIQUE | NON_NULL, (empty)
            // 2225: b"can not read  ... s'\0": typeof(_232 = const b"can not read \'%s\'\x00") = & {l1526} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 2225: b"can not read  ... s'\0":   l1526 = UNIQUE | NON_NULL, (empty)
            // 2225: b"can not read  ... _char: typeof(_229 = move _230 as *const i8 (Misc)) = *const {l1529} i8
            // 2225: b"can not read  ... _char:   l1529 = UNIQUE | NON_NULL, (empty)
            iname,
            // 2226: iname: typeof(_233) = *const {l385} i8
            // 2226: iname:   l385 = UNIQUE | NON_NULL, (empty)
            // 2226: iname: typeof(_234) = *mut {l387} *const {l388} i8
            // 2226: iname:   l387 = UNIQUE | NON_NULL, (empty)
            // 2226: iname:   l388 = UNIQUE | NON_NULL, (empty)
        );
    }
    msg(b"reading %s\0" as *const u8 as *const libc::c_char, iname);
    // 2229: b"reading %s\0" ... _char: typeof(_236) = *const {l391} i8
    // 2229: b"reading %s\0" ... _char:   l391 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0" ... st u8: typeof(_237) = *const {l393} u8
    // 2229: b"reading %s\0" ... st u8:   l393 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0": typeof(_238) = *const {l395} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 2229: b"reading %s\0":   l395 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0": typeof(_239) = & {l397} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 2229: b"reading %s\0":   l397 = UNIQUE | NON_NULL, FIXED
    // 2229: iname: typeof(_240) = *const {l399} i8
    // 2229: iname:   l399 = UNIQUE | NON_NULL, (empty)
    // 2229: iname: typeof(_241) = *mut {l401} *const {l402} i8
    // 2229: iname:   l401 = UNIQUE | NON_NULL, (empty)
    // 2229: iname:   l402 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0": typeof(_239 = const b"reading %s\x00") = & {l1530} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 2229: b"reading %s\0":   l1530 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0" ... _char: typeof(_236 = move _237 as *const i8 (Misc)) = *const {l1533} i8
    // 2229: b"reading %s\0" ... _char:   l1533 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0": typeof(_238 = &raw const (*_239)) = *const {l1531} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
    // 2229: b"reading %s\0":   l1531 = UNIQUE | NON_NULL, (empty)
    // 2229: b"reading %s\0" ... st u8: typeof(_237 = move _238 as *const u8 (Pointer(ArrayToPointer))) = *const {l1532} u8
    // 2229: b"reading %s\0" ... st u8:   l1532 = UNIQUE | NON_NULL, (empty)
    len = 0 as libc::c_int;
    buffer[len as usize] = 0 as libc::c_int as libc::c_char;
    lineno = 1 as libc::c_int;
    // 2232: lineno: typeof(_249) = *mut {l411} i32
    // 2232: lineno:   l411 = UNIQUE | NON_NULL, (empty)
    count = 0 as libc::c_int;
    loop {
        ch = getc(file);
        // 2235: file: typeof(_253) = *mut {l416} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2235: file:   l416 = UNIQUE | NON_NULL, (empty)
        if ch == -(1 as libc::c_int) {
            break;
        }
        if ch == '\r' as i32 {
            continue;
        }
        if ch != '\n' as i32 {
            if len + 1 as libc::c_int
                >= ::core::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int
            {
                perr(b"line buffer exceeded\0" as *const u8 as *const libc::c_char);
                // 2246: b"line buffer e ... _char: typeof(_279) = *const {l443} i8
                // 2246: b"line buffer e ... _char:   l443 = UNIQUE | NON_NULL, (empty)
                // 2246: b"line buffer e ... st u8: typeof(_280) = *const {l445} u8
                // 2246: b"line buffer e ... st u8:   l445 = UNIQUE | NON_NULL, (empty)
                // 2246: b"line buffer e ... ed\0": typeof(_281) = *const {l447} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2246: b"line buffer e ... ed\0":   l447 = UNIQUE | NON_NULL, (empty)
                // 2246: b"line buffer e ... ed\0": typeof(_282) = & {l449} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2246: b"line buffer e ... ed\0":   l449 = UNIQUE | NON_NULL, FIXED
                // 2246: b"line buffer e ... _char: typeof(_279 = move _280 as *const i8 (Misc)) = *const {l1537} i8
                // 2246: b"line buffer e ... _char:   l1537 = UNIQUE | NON_NULL, (empty)
                // 2246: b"line buffer e ... st u8: typeof(_280 = move _281 as *const u8 (Pointer(ArrayToPointer))) = *const {l1536} u8
                // 2246: b"line buffer e ... st u8:   l1536 = UNIQUE | NON_NULL, (empty)
                // 2246: b"line buffer e ... ed\0": typeof(_282 = const b"line buffer exceeded\x00") = & {l1534} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2246: b"line buffer e ... ed\0":   l1534 = UNIQUE | NON_NULL, (empty)
                // 2246: b"line buffer e ... ed\0": typeof(_281 = &raw const (*_282)) = *const {l1535} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2246: b"line buffer e ... ed\0":   l1535 = UNIQUE | NON_NULL, (empty)
            }
            let fresh4 = len;
            len = len + 1;
            buffer[fresh4 as usize] = ch as libc::c_char;
            buffer[len as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            if verbose > 2 as libc::c_int {
            // 2253: verbose: typeof(_299) = *mut {l467} i32
            // 2253: verbose:   l467 = UNIQUE | NON_NULL, (empty)
                msg(
                    b"line %d : %s\0" as *const u8 as *const libc::c_char,
                    // 2255: b"line %d : %s\ ... _char: typeof(_302) = *const {l471} i8
                    // 2255: b"line %d : %s\ ... _char:   l471 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"line %d : %s\ ... st u8: typeof(_303) = *const {l473} u8
                    // 2255: b"line %d : %s\ ... st u8:   l473 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"line %d : %s\0": typeof(_304) = *const {l475} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 2255: b"line %d : %s\0":   l475 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"line %d : %s\0": typeof(_305) = & {l477} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 2255: b"line %d : %s\0":   l477 = UNIQUE | NON_NULL, FIXED
                    // 2255: b"line %d : %s\0": typeof(_304 = &raw const (*_305)) = *const {l1539} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 2255: b"line %d : %s\0":   l1539 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"line %d : %s\ ... st u8: typeof(_303 = move _304 as *const u8 (Pointer(ArrayToPointer))) = *const {l1540} u8
                    // 2255: b"line %d : %s\ ... st u8:   l1540 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"line %d : %s\0": typeof(_305 = const b"line %d : %s\x00") = & {l1538} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                    // 2255: b"line %d : %s\0":   l1538 = UNIQUE | NON_NULL, (empty)
                    // 2255: b"line %d : %s\ ... _char: typeof(_302 = move _303 as *const i8 (Misc)) = *const {l1541} i8
                    // 2255: b"line %d : %s\ ... _char:   l1541 = UNIQUE | NON_NULL, (empty)
                    lineno,
                    // 2256: lineno: typeof(_307) = *mut {l480} i32
                    // 2256: lineno:   l480 = UNIQUE | NON_NULL, (empty)
                    buffer.as_mut_ptr(),
                    // 2257: buffer.as_mut_ptr(): typeof(_308) = *mut {l482} i8
                    // 2257: buffer.as_mut_ptr():   l482 = UNIQUE | NON_NULL, (empty)
                    // 2257: buffer.as_mut_ptr(): typeof(_309) = &mut {l484} [i8]
                    // 2257: buffer.as_mut_ptr():   l484 = UNIQUE | NON_NULL, FIXED
                    // 2257: buffer.as_mut_ptr(): typeof(_310) = &mut {l486} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                    // 2257: buffer.as_mut_ptr():   l486 = UNIQUE | NON_NULL, (empty)
                    // 2257: buffer.as_mut_ptr(): typeof(_310 = &mut _9) = &mut {l1542} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                    // 2257: buffer.as_mut_ptr():   l1542 = UNIQUE | NON_NULL, (empty)
                    // 2257: buffer.as_mut_ptr(): typeof(_309 = move _310 as &mut [i8] (Pointer(Unsize))) = &mut {l1543} [i8]
                    // 2257: buffer.as_mut_ptr():   l1543 = UNIQUE | NON_NULL, FIXED
                );
            }
            tok = strtok(
            // 2260: strtok( buffer. ... ar, ): typeof(_311) = *mut {l488} i8
            // 2260: strtok( buffer. ... ar, ):   l488 = UNIQUE | NON_NULL, (empty)
                buffer.as_mut_ptr(),
                // 2261: buffer.as_mut_ptr(): typeof(_312) = *mut {l490} i8
                // 2261: buffer.as_mut_ptr():   l490 = UNIQUE | NON_NULL, (empty)
                // 2261: buffer.as_mut_ptr(): typeof(_313) = &mut {l492} [i8]
                // 2261: buffer.as_mut_ptr():   l492 = UNIQUE | NON_NULL, FIXED
                // 2261: buffer.as_mut_ptr(): typeof(_314) = &mut {l494} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                // 2261: buffer.as_mut_ptr():   l494 = UNIQUE | NON_NULL, (empty)
                // 2261: buffer.as_mut_ptr(): typeof(_313 = move _314 as &mut [i8] (Pointer(Unsize))) = &mut {l1545} [i8]
                // 2261: buffer.as_mut_ptr():   l1545 = UNIQUE | NON_NULL, FIXED
                // 2261: buffer.as_mut_ptr(): typeof(_314 = &mut _9) = &mut {l1544} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000050)) }]
                // 2261: buffer.as_mut_ptr():   l1544 = UNIQUE | NON_NULL, (empty)
                b" \0" as *const u8 as *const libc::c_char,
                // 2262: b" \0" as *cons ... _char: typeof(_315) = *const {l496} i8
                // 2262: b" \0" as *cons ... _char:   l496 = UNIQUE | NON_NULL, (empty)
                // 2262: b" \0" as *const u8: typeof(_316) = *const {l498} u8
                // 2262: b" \0" as *const u8:   l498 = UNIQUE | NON_NULL, (empty)
                // 2262: b" \0": typeof(_317) = *const {l500} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 2262: b" \0":   l500 = UNIQUE | NON_NULL, (empty)
                // 2262: b" \0": typeof(_318) = & {l502} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 2262: b" \0":   l502 = UNIQUE | NON_NULL, FIXED
                // 2262: b" \0" as *cons ... _char: typeof(_315 = move _316 as *const i8 (Misc)) = *const {l1549} i8
                // 2262: b" \0" as *cons ... _char:   l1549 = UNIQUE | NON_NULL, (empty)
                // 2262: b" \0": typeof(_318 = const b" \x00") = & {l1546} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 2262: b" \0":   l1546 = UNIQUE | NON_NULL, (empty)
                // 2262: b" \0": typeof(_317 = &raw const (*_318)) = *const {l1547} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                // 2262: b" \0":   l1547 = UNIQUE | NON_NULL, (empty)
                // 2262: b" \0" as *const u8: typeof(_316 = move _317 as *const u8 (Pointer(ArrayToPointer))) = *const {l1548} u8
                // 2262: b" \0" as *const u8:   l1548 = UNIQUE | NON_NULL, (empty)
            );
            if tok.is_null() {
            // 2264: tok: typeof(_321) = *mut {l506} i8
            // 2264: tok:   l506 = UNIQUE | NON_NULL, (empty)
                perr(b"empty line\0" as *const u8 as *const libc::c_char);
                // 2265: b"empty line\0" ... _char: typeof(_323) = *const {l509} i8
                // 2265: b"empty line\0" ... _char:   l509 = UNIQUE | NON_NULL, (empty)
                // 2265: b"empty line\0" ... st u8: typeof(_324) = *const {l511} u8
                // 2265: b"empty line\0" ... st u8:   l511 = UNIQUE | NON_NULL, (empty)
                // 2265: b"empty line\0": typeof(_325) = *const {l513} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 2265: b"empty line\0":   l513 = UNIQUE | NON_NULL, (empty)
                // 2265: b"empty line\0": typeof(_326) = & {l515} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 2265: b"empty line\0":   l515 = UNIQUE | NON_NULL, FIXED
                // 2265: b"empty line\0": typeof(_326 = const b"empty line\x00") = & {l1550} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 2265: b"empty line\0":   l1550 = UNIQUE | NON_NULL, (empty)
                // 2265: b"empty line\0" ... st u8: typeof(_324 = move _325 as *const u8 (Pointer(ArrayToPointer))) = *const {l1552} u8
                // 2265: b"empty line\0" ... st u8:   l1552 = UNIQUE | NON_NULL, (empty)
                // 2265: b"empty line\0": typeof(_325 = &raw const (*_326)) = *const {l1551} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 2265: b"empty line\0":   l1551 = UNIQUE | NON_NULL, (empty)
                // 2265: b"empty line\0" ... _char: typeof(_323 = move _324 as *const i8 (Misc)) = *const {l1553} i8
                // 2265: b"empty line\0" ... _char:   l1553 = UNIQUE | NON_NULL, (empty)
            } else if strcmp(tok, b"add\0" as *const u8 as *const libc::c_char) == 0 {
            // 2266: tok: typeof(_329) = *const {l519} i8
            // 2266: tok:   l519 = UNIQUE | NON_NULL, (empty)
            // 2266: tok: typeof(_330) = *mut {l521} i8
            // 2266: tok:   l521 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0" as *co ... _char: typeof(_331) = *const {l523} i8
            // 2266: b"add\0" as *co ... _char:   l523 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0" as *co ... st u8: typeof(_332) = *const {l525} u8
            // 2266: b"add\0" as *co ... st u8:   l525 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0": typeof(_333) = *const {l527} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2266: b"add\0":   l527 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0": typeof(_334) = & {l529} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2266: b"add\0":   l529 = UNIQUE | NON_NULL, FIXED
            // 2266: b"add\0": typeof(_334 = const b"add\x00") = & {l1555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2266: b"add\0":   l1555 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0" as *co ... st u8: typeof(_332 = move _333 as *const u8 (Pointer(ArrayToPointer))) = *const {l1557} u8
            // 2266: b"add\0" as *co ... st u8:   l1557 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0": typeof(_333 = &raw const (*_334)) = *const {l1556} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2266: b"add\0":   l1556 = UNIQUE | NON_NULL, (empty)
            // 2266: tok: typeof(_329 = move _330 as *const i8 (Pointer(MutToConstPointer))) = *const {l1554} i8
            // 2266: tok:   l1554 = UNIQUE | NON_NULL, (empty)
            // 2266: b"add\0" as *co ... _char: typeof(_331 = move _332 as *const i8 (Misc)) = *const {l1558} i8
            // 2266: b"add\0" as *co ... _char:   l1558 = UNIQUE | NON_NULL, (empty)
                event(
                    ADD,
                    intarg(b"add\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2269: b"add\0" as *co ... _char: typeof(_337) = *mut {l533} i8
                    // 2269: b"add\0" as *co ... _char:   l533 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0" as *co ... _char: typeof(_338) = *const {l535} i8
                    // 2269: b"add\0" as *co ... _char:   l535 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0" as *co ... st u8: typeof(_339) = *const {l537} u8
                    // 2269: b"add\0" as *co ... st u8:   l537 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0": typeof(_340) = *const {l539} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2269: b"add\0":   l539 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0": typeof(_341) = & {l541} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2269: b"add\0":   l541 = UNIQUE | NON_NULL, FIXED
                    // 2269: b"add\0" as *co ... _char: typeof(_337 = move _338 as *mut i8 (Misc)) = *mut {l1563} i8
                    // 2269: b"add\0" as *co ... _char:   l1563 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0" as *co ... st u8: typeof(_339 = move _340 as *const u8 (Pointer(ArrayToPointer))) = *const {l1561} u8
                    // 2269: b"add\0" as *co ... st u8:   l1561 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0": typeof(_340 = &raw const (*_341)) = *const {l1560} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2269: b"add\0":   l1560 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0": typeof(_341 = const b"add\x00") = & {l1559} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                    // 2269: b"add\0":   l1559 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"add\0" as *co ... _char: typeof(_338 = move _339 as *const i8 (Misc)) = *const {l1562} i8
                    // 2269: b"add\0" as *co ... _char:   l1562 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2270: 0 as *const lib ... _char: typeof(_342) = *const {l543} i8
                    // 2270: 0 as *const lib ... _char:   l543 = UNIQUE | NON_NULL, (empty)
                    // 2270: 0 as *const lib ... _char: typeof(_342 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1564} i8
                    // 2270: 0 as *const lib ... _char:   l1564 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"return\0" as *const u8 as *const libc::c_char) == 0 {
            // 2272: tok: typeof(_345) = *const {l547} i8
            // 2272: tok:   l547 = UNIQUE | NON_NULL, (empty)
            // 2272: tok: typeof(_346) = *mut {l549} i8
            // 2272: tok:   l549 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0" as  ... _char: typeof(_347) = *const {l551} i8
            // 2272: b"return\0" as  ... _char:   l551 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0" as  ... st u8: typeof(_348) = *const {l553} u8
            // 2272: b"return\0" as  ... st u8:   l553 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0": typeof(_349) = *const {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2272: b"return\0":   l555 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0": typeof(_350) = & {l557} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2272: b"return\0":   l557 = UNIQUE | NON_NULL, FIXED
            // 2272: b"return\0": typeof(_350 = const b"return\x00") = & {l1566} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2272: b"return\0":   l1566 = UNIQUE | NON_NULL, (empty)
            // 2272: tok: typeof(_345 = move _346 as *const i8 (Pointer(MutToConstPointer))) = *const {l1565} i8
            // 2272: tok:   l1565 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0" as  ... _char: typeof(_347 = move _348 as *const i8 (Misc)) = *const {l1569} i8
            // 2272: b"return\0" as  ... _char:   l1569 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0": typeof(_349 = &raw const (*_350)) = *const {l1567} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2272: b"return\0":   l1567 = UNIQUE | NON_NULL, (empty)
            // 2272: b"return\0" as  ... st u8: typeof(_348 = move _349 as *const u8 (Pointer(ArrayToPointer))) = *const {l1568} u8
            // 2272: b"return\0" as  ... st u8:   l1568 = UNIQUE | NON_NULL, (empty)
                event(
                    RETURN,
                    intarg(b"return\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2275: b"return\0" as  ... _char: typeof(_353) = *mut {l561} i8
                    // 2275: b"return\0" as  ... _char:   l561 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0" as  ... _char: typeof(_354) = *const {l563} i8
                    // 2275: b"return\0" as  ... _char:   l563 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0" as  ... st u8: typeof(_355) = *const {l565} u8
                    // 2275: b"return\0" as  ... st u8:   l565 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0": typeof(_356) = *const {l567} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2275: b"return\0":   l567 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0": typeof(_357) = & {l569} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2275: b"return\0":   l569 = UNIQUE | NON_NULL, FIXED
                    // 2275: b"return\0" as  ... st u8: typeof(_355 = move _356 as *const u8 (Pointer(ArrayToPointer))) = *const {l1572} u8
                    // 2275: b"return\0" as  ... st u8:   l1572 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0": typeof(_357 = const b"return\x00") = & {l1570} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2275: b"return\0":   l1570 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0" as  ... _char: typeof(_354 = move _355 as *const i8 (Misc)) = *const {l1573} i8
                    // 2275: b"return\0" as  ... _char:   l1573 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0" as  ... _char: typeof(_353 = move _354 as *mut i8 (Misc)) = *mut {l1574} i8
                    // 2275: b"return\0" as  ... _char:   l1574 = UNIQUE | NON_NULL, (empty)
                    // 2275: b"return\0": typeof(_356 = &raw const (*_357)) = *const {l1571} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2275: b"return\0":   l1571 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2276: 0 as *const lib ... _char: typeof(_358) = *const {l571} i8
                    // 2276: 0 as *const lib ... _char:   l571 = UNIQUE | NON_NULL, (empty)
                    // 2276: 0 as *const lib ... _char: typeof(_358 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1575} i8
                    // 2276: 0 as *const lib ... _char:   l1575 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"deref\0" as *const u8 as *const libc::c_char) == 0 {
            // 2278: tok: typeof(_361) = *const {l575} i8
            // 2278: tok:   l575 = UNIQUE | NON_NULL, (empty)
            // 2278: tok: typeof(_362) = *mut {l577} i8
            // 2278: tok:   l577 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0" as * ... _char: typeof(_363) = *const {l579} i8
            // 2278: b"deref\0" as * ... _char:   l579 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0" as * ... st u8: typeof(_364) = *const {l581} u8
            // 2278: b"deref\0" as * ... st u8:   l581 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0": typeof(_365) = *const {l583} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2278: b"deref\0":   l583 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0": typeof(_366) = & {l585} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2278: b"deref\0":   l585 = UNIQUE | NON_NULL, FIXED
            // 2278: tok: typeof(_361 = move _362 as *const i8 (Pointer(MutToConstPointer))) = *const {l1576} i8
            // 2278: tok:   l1576 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0": typeof(_365 = &raw const (*_366)) = *const {l1578} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2278: b"deref\0":   l1578 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0" as * ... _char: typeof(_363 = move _364 as *const i8 (Misc)) = *const {l1580} i8
            // 2278: b"deref\0" as * ... _char:   l1580 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0" as * ... st u8: typeof(_364 = move _365 as *const u8 (Pointer(ArrayToPointer))) = *const {l1579} u8
            // 2278: b"deref\0" as * ... st u8:   l1579 = UNIQUE | NON_NULL, (empty)
            // 2278: b"deref\0": typeof(_366 = const b"deref\x00") = & {l1577} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2278: b"deref\0":   l1577 = UNIQUE | NON_NULL, (empty)
                event(
                    DEREF,
                    intarg(b"deref\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2281: b"deref\0" as * ... _char: typeof(_369) = *mut {l589} i8
                    // 2281: b"deref\0" as * ... _char:   l589 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0" as * ... _char: typeof(_370) = *const {l591} i8
                    // 2281: b"deref\0" as * ... _char:   l591 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0" as * ... st u8: typeof(_371) = *const {l593} u8
                    // 2281: b"deref\0" as * ... st u8:   l593 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0": typeof(_372) = *const {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2281: b"deref\0":   l595 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0": typeof(_373) = & {l597} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2281: b"deref\0":   l597 = UNIQUE | NON_NULL, FIXED
                    // 2281: b"deref\0" as * ... st u8: typeof(_371 = move _372 as *const u8 (Pointer(ArrayToPointer))) = *const {l1583} u8
                    // 2281: b"deref\0" as * ... st u8:   l1583 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0": typeof(_373 = const b"deref\x00") = & {l1581} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2281: b"deref\0":   l1581 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0" as * ... _char: typeof(_369 = move _370 as *mut i8 (Misc)) = *mut {l1585} i8
                    // 2281: b"deref\0" as * ... _char:   l1585 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0" as * ... _char: typeof(_370 = move _371 as *const i8 (Misc)) = *const {l1584} i8
                    // 2281: b"deref\0" as * ... _char:   l1584 = UNIQUE | NON_NULL, (empty)
                    // 2281: b"deref\0": typeof(_372 = &raw const (*_373)) = *const {l1582} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2281: b"deref\0":   l1582 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2282: 0 as *const lib ... _char: typeof(_374) = *const {l599} i8
                    // 2282: 0 as *const lib ... _char:   l599 = UNIQUE | NON_NULL, (empty)
                    // 2282: 0 as *const lib ... _char: typeof(_374 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1586} i8
                    // 2282: 0 as *const lib ... _char:   l1586 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"fixed\0" as *const u8 as *const libc::c_char) == 0 {
            // 2284: tok: typeof(_377) = *const {l603} i8
            // 2284: tok:   l603 = UNIQUE | NON_NULL, (empty)
            // 2284: tok: typeof(_378) = *mut {l605} i8
            // 2284: tok:   l605 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0" as * ... _char: typeof(_379) = *const {l607} i8
            // 2284: b"fixed\0" as * ... _char:   l607 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0" as * ... st u8: typeof(_380) = *const {l609} u8
            // 2284: b"fixed\0" as * ... st u8:   l609 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0": typeof(_381) = *const {l611} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2284: b"fixed\0":   l611 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0": typeof(_382) = & {l613} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2284: b"fixed\0":   l613 = UNIQUE | NON_NULL, FIXED
            // 2284: tok: typeof(_377 = move _378 as *const i8 (Pointer(MutToConstPointer))) = *const {l1587} i8
            // 2284: tok:   l1587 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0" as * ... _char: typeof(_379 = move _380 as *const i8 (Misc)) = *const {l1591} i8
            // 2284: b"fixed\0" as * ... _char:   l1591 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0": typeof(_382 = const b"fixed\x00") = & {l1588} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2284: b"fixed\0":   l1588 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0" as * ... st u8: typeof(_380 = move _381 as *const u8 (Pointer(ArrayToPointer))) = *const {l1590} u8
            // 2284: b"fixed\0" as * ... st u8:   l1590 = UNIQUE | NON_NULL, (empty)
            // 2284: b"fixed\0": typeof(_381 = &raw const (*_382)) = *const {l1589} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2284: b"fixed\0":   l1589 = UNIQUE | NON_NULL, (empty)
                event(
                    FIXED,
                    intarg(b"fixed\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2287: b"fixed\0" as * ... _char: typeof(_385) = *mut {l617} i8
                    // 2287: b"fixed\0" as * ... _char:   l617 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0" as * ... _char: typeof(_386) = *const {l619} i8
                    // 2287: b"fixed\0" as * ... _char:   l619 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0" as * ... st u8: typeof(_387) = *const {l621} u8
                    // 2287: b"fixed\0" as * ... st u8:   l621 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0": typeof(_388) = *const {l623} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2287: b"fixed\0":   l623 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0": typeof(_389) = & {l625} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2287: b"fixed\0":   l625 = UNIQUE | NON_NULL, FIXED
                    // 2287: b"fixed\0": typeof(_388 = &raw const (*_389)) = *const {l1593} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2287: b"fixed\0":   l1593 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0" as * ... _char: typeof(_386 = move _387 as *const i8 (Misc)) = *const {l1595} i8
                    // 2287: b"fixed\0" as * ... _char:   l1595 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0": typeof(_389 = const b"fixed\x00") = & {l1592} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2287: b"fixed\0":   l1592 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0" as * ... _char: typeof(_385 = move _386 as *mut i8 (Misc)) = *mut {l1596} i8
                    // 2287: b"fixed\0" as * ... _char:   l1596 = UNIQUE | NON_NULL, (empty)
                    // 2287: b"fixed\0" as * ... st u8: typeof(_387 = move _388 as *const u8 (Pointer(ArrayToPointer))) = *const {l1594} u8
                    // 2287: b"fixed\0" as * ... st u8:   l1594 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2288: 0 as *const lib ... _char: typeof(_390) = *const {l627} i8
                    // 2288: 0 as *const lib ... _char:   l627 = UNIQUE | NON_NULL, (empty)
                    // 2288: 0 as *const lib ... _char: typeof(_390 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1597} i8
                    // 2288: 0 as *const lib ... _char:   l1597 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"frozen\0" as *const u8 as *const libc::c_char) == 0 {
            // 2290: tok: typeof(_393) = *const {l631} i8
            // 2290: tok:   l631 = UNIQUE | NON_NULL, (empty)
            // 2290: tok: typeof(_394) = *mut {l633} i8
            // 2290: tok:   l633 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0" as  ... _char: typeof(_395) = *const {l635} i8
            // 2290: b"frozen\0" as  ... _char:   l635 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0" as  ... st u8: typeof(_396) = *const {l637} u8
            // 2290: b"frozen\0" as  ... st u8:   l637 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0": typeof(_397) = *const {l639} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2290: b"frozen\0":   l639 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0": typeof(_398) = & {l641} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2290: b"frozen\0":   l641 = UNIQUE | NON_NULL, FIXED
            // 2290: b"frozen\0" as  ... _char: typeof(_395 = move _396 as *const i8 (Misc)) = *const {l1602} i8
            // 2290: b"frozen\0" as  ... _char:   l1602 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0": typeof(_398 = const b"frozen\x00") = & {l1599} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2290: b"frozen\0":   l1599 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0" as  ... st u8: typeof(_396 = move _397 as *const u8 (Pointer(ArrayToPointer))) = *const {l1601} u8
            // 2290: b"frozen\0" as  ... st u8:   l1601 = UNIQUE | NON_NULL, (empty)
            // 2290: tok: typeof(_393 = move _394 as *const i8 (Pointer(MutToConstPointer))) = *const {l1598} i8
            // 2290: tok:   l1598 = UNIQUE | NON_NULL, (empty)
            // 2290: b"frozen\0": typeof(_397 = &raw const (*_398)) = *const {l1600} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2290: b"frozen\0":   l1600 = UNIQUE | NON_NULL, (empty)
                event(
                    FROZEN,
                    intarg(b"frozen\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2293: b"frozen\0" as  ... _char: typeof(_401) = *mut {l645} i8
                    // 2293: b"frozen\0" as  ... _char:   l645 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0" as  ... _char: typeof(_402) = *const {l647} i8
                    // 2293: b"frozen\0" as  ... _char:   l647 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0" as  ... st u8: typeof(_403) = *const {l649} u8
                    // 2293: b"frozen\0" as  ... st u8:   l649 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0": typeof(_404) = *const {l651} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2293: b"frozen\0":   l651 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0": typeof(_405) = & {l653} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2293: b"frozen\0":   l653 = UNIQUE | NON_NULL, FIXED
                    // 2293: b"frozen\0" as  ... _char: typeof(_402 = move _403 as *const i8 (Misc)) = *const {l1606} i8
                    // 2293: b"frozen\0" as  ... _char:   l1606 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0": typeof(_404 = &raw const (*_405)) = *const {l1604} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2293: b"frozen\0":   l1604 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0" as  ... st u8: typeof(_403 = move _404 as *const u8 (Pointer(ArrayToPointer))) = *const {l1605} u8
                    // 2293: b"frozen\0" as  ... st u8:   l1605 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0" as  ... _char: typeof(_401 = move _402 as *mut i8 (Misc)) = *mut {l1607} i8
                    // 2293: b"frozen\0" as  ... _char:   l1607 = UNIQUE | NON_NULL, (empty)
                    // 2293: b"frozen\0": typeof(_405 = const b"frozen\x00") = & {l1603} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2293: b"frozen\0":   l1603 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2294: 0 as *const lib ... _char: typeof(_406) = *const {l655} i8
                    // 2294: 0 as *const lib ... _char:   l655 = UNIQUE | NON_NULL, (empty)
                    // 2294: 0 as *const lib ... _char: typeof(_406 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1608} i8
                    // 2294: 0 as *const lib ... _char:   l1608 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"reusable\0" as *const u8 as *const libc::c_char) == 0 {
            // 2296: tok: typeof(_409) = *const {l659} i8
            // 2296: tok:   l659 = UNIQUE | NON_NULL, (empty)
            // 2296: tok: typeof(_410) = *mut {l661} i8
            // 2296: tok:   l661 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0" a ... _char: typeof(_411) = *const {l663} i8
            // 2296: b"reusable\0" a ... _char:   l663 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0" a ... st u8: typeof(_412) = *const {l665} u8
            // 2296: b"reusable\0" a ... st u8:   l665 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0": typeof(_413) = *const {l667} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2296: b"reusable\0":   l667 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0": typeof(_414) = & {l669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2296: b"reusable\0":   l669 = UNIQUE | NON_NULL, FIXED
            // 2296: b"reusable\0": typeof(_414 = const b"reusable\x00") = & {l1610} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2296: b"reusable\0":   l1610 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0" a ... _char: typeof(_411 = move _412 as *const i8 (Misc)) = *const {l1613} i8
            // 2296: b"reusable\0" a ... _char:   l1613 = UNIQUE | NON_NULL, (empty)
            // 2296: tok: typeof(_409 = move _410 as *const i8 (Pointer(MutToConstPointer))) = *const {l1609} i8
            // 2296: tok:   l1609 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0" a ... st u8: typeof(_412 = move _413 as *const u8 (Pointer(ArrayToPointer))) = *const {l1612} u8
            // 2296: b"reusable\0" a ... st u8:   l1612 = UNIQUE | NON_NULL, (empty)
            // 2296: b"reusable\0": typeof(_413 = &raw const (*_414)) = *const {l1611} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2296: b"reusable\0":   l1611 = UNIQUE | NON_NULL, (empty)
                event(
                    REUSABLE,
                    intarg(b"reusable\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2299: b"reusable\0" a ... _char: typeof(_417) = *mut {l673} i8
                    // 2299: b"reusable\0" a ... _char:   l673 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0" a ... _char: typeof(_418) = *const {l675} i8
                    // 2299: b"reusable\0" a ... _char:   l675 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0" a ... st u8: typeof(_419) = *const {l677} u8
                    // 2299: b"reusable\0" a ... st u8:   l677 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0": typeof(_420) = *const {l679} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2299: b"reusable\0":   l679 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0": typeof(_421) = & {l681} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2299: b"reusable\0":   l681 = UNIQUE | NON_NULL, FIXED
                    // 2299: b"reusable\0": typeof(_420 = &raw const (*_421)) = *const {l1615} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2299: b"reusable\0":   l1615 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0" a ... _char: typeof(_418 = move _419 as *const i8 (Misc)) = *const {l1617} i8
                    // 2299: b"reusable\0" a ... _char:   l1617 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0" a ... st u8: typeof(_419 = move _420 as *const u8 (Pointer(ArrayToPointer))) = *const {l1616} u8
                    // 2299: b"reusable\0" a ... st u8:   l1616 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0": typeof(_421 = const b"reusable\x00") = & {l1614} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2299: b"reusable\0":   l1614 = UNIQUE | NON_NULL, (empty)
                    // 2299: b"reusable\0" a ... _char: typeof(_417 = move _418 as *mut i8 (Misc)) = *mut {l1618} i8
                    // 2299: b"reusable\0" a ... _char:   l1618 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2300: 0 as *const lib ... _char: typeof(_422) = *const {l683} i8
                    // 2300: 0 as *const lib ... _char:   l683 = UNIQUE | NON_NULL, (empty)
                    // 2300: 0 as *const lib ... _char: typeof(_422 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1619} i8
                    // 2300: 0 as *const lib ... _char:   l1619 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"usable\0" as *const u8 as *const libc::c_char) == 0 {
            // 2302: tok: typeof(_425) = *const {l687} i8
            // 2302: tok:   l687 = UNIQUE | NON_NULL, (empty)
            // 2302: tok: typeof(_426) = *mut {l689} i8
            // 2302: tok:   l689 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0" as  ... _char: typeof(_427) = *const {l691} i8
            // 2302: b"usable\0" as  ... _char:   l691 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0" as  ... st u8: typeof(_428) = *const {l693} u8
            // 2302: b"usable\0" as  ... st u8:   l693 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0": typeof(_429) = *const {l695} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2302: b"usable\0":   l695 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0": typeof(_430) = & {l697} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2302: b"usable\0":   l697 = UNIQUE | NON_NULL, FIXED
            // 2302: b"usable\0" as  ... st u8: typeof(_428 = move _429 as *const u8 (Pointer(ArrayToPointer))) = *const {l1623} u8
            // 2302: b"usable\0" as  ... st u8:   l1623 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0" as  ... _char: typeof(_427 = move _428 as *const i8 (Misc)) = *const {l1624} i8
            // 2302: b"usable\0" as  ... _char:   l1624 = UNIQUE | NON_NULL, (empty)
            // 2302: tok: typeof(_425 = move _426 as *const i8 (Pointer(MutToConstPointer))) = *const {l1620} i8
            // 2302: tok:   l1620 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0": typeof(_430 = const b"usable\x00") = & {l1621} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2302: b"usable\0":   l1621 = UNIQUE | NON_NULL, (empty)
            // 2302: b"usable\0": typeof(_429 = &raw const (*_430)) = *const {l1622} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2302: b"usable\0":   l1622 = UNIQUE | NON_NULL, (empty)
                event(
                    USABLE,
                    intarg(b"usable\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2305: b"usable\0" as  ... _char: typeof(_433) = *mut {l701} i8
                    // 2305: b"usable\0" as  ... _char:   l701 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0" as  ... _char: typeof(_434) = *const {l703} i8
                    // 2305: b"usable\0" as  ... _char:   l703 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0" as  ... st u8: typeof(_435) = *const {l705} u8
                    // 2305: b"usable\0" as  ... st u8:   l705 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0": typeof(_436) = *const {l707} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2305: b"usable\0":   l707 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0": typeof(_437) = & {l709} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2305: b"usable\0":   l709 = UNIQUE | NON_NULL, FIXED
                    // 2305: b"usable\0": typeof(_437 = const b"usable\x00") = & {l1625} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2305: b"usable\0":   l1625 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0" as  ... st u8: typeof(_435 = move _436 as *const u8 (Pointer(ArrayToPointer))) = *const {l1627} u8
                    // 2305: b"usable\0" as  ... st u8:   l1627 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0" as  ... _char: typeof(_433 = move _434 as *mut i8 (Misc)) = *mut {l1629} i8
                    // 2305: b"usable\0" as  ... _char:   l1629 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0": typeof(_436 = &raw const (*_437)) = *const {l1626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2305: b"usable\0":   l1626 = UNIQUE | NON_NULL, (empty)
                    // 2305: b"usable\0" as  ... _char: typeof(_434 = move _435 as *const i8 (Misc)) = *const {l1628} i8
                    // 2305: b"usable\0" as  ... _char:   l1628 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2306: 0 as *const lib ... _char: typeof(_438) = *const {l711} i8
                    // 2306: 0 as *const lib ... _char:   l711 = UNIQUE | NON_NULL, (empty)
                    // 2306: 0 as *const lib ... _char: typeof(_438 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1630} i8
                    // 2306: 0 as *const lib ... _char:   l1630 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"repr\0" as *const u8 as *const libc::c_char) == 0 {
            // 2308: tok: typeof(_441) = *const {l715} i8
            // 2308: tok:   l715 = UNIQUE | NON_NULL, (empty)
            // 2308: tok: typeof(_442) = *mut {l717} i8
            // 2308: tok:   l717 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0" as *c ... _char: typeof(_443) = *const {l719} i8
            // 2308: b"repr\0" as *c ... _char:   l719 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0" as *c ... st u8: typeof(_444) = *const {l721} u8
            // 2308: b"repr\0" as *c ... st u8:   l721 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0": typeof(_445) = *const {l723} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2308: b"repr\0":   l723 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0": typeof(_446) = & {l725} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2308: b"repr\0":   l725 = UNIQUE | NON_NULL, FIXED
            // 2308: tok: typeof(_441 = move _442 as *const i8 (Pointer(MutToConstPointer))) = *const {l1631} i8
            // 2308: tok:   l1631 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0" as *c ... st u8: typeof(_444 = move _445 as *const u8 (Pointer(ArrayToPointer))) = *const {l1634} u8
            // 2308: b"repr\0" as *c ... st u8:   l1634 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0": typeof(_446 = const b"repr\x00") = & {l1632} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2308: b"repr\0":   l1632 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0": typeof(_445 = &raw const (*_446)) = *const {l1633} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2308: b"repr\0":   l1633 = UNIQUE | NON_NULL, (empty)
            // 2308: b"repr\0" as *c ... _char: typeof(_443 = move _444 as *const i8 (Misc)) = *const {l1635} i8
            // 2308: b"repr\0" as *c ... _char:   l1635 = UNIQUE | NON_NULL, (empty)
                event(
                    REPR,
                    intarg(b"repr\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2311: b"repr\0" as *c ... _char: typeof(_449) = *mut {l729} i8
                    // 2311: b"repr\0" as *c ... _char:   l729 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0" as *c ... _char: typeof(_450) = *const {l731} i8
                    // 2311: b"repr\0" as *c ... _char:   l731 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0" as *c ... st u8: typeof(_451) = *const {l733} u8
                    // 2311: b"repr\0" as *c ... st u8:   l733 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0": typeof(_452) = *const {l735} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2311: b"repr\0":   l735 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0": typeof(_453) = & {l737} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2311: b"repr\0":   l737 = UNIQUE | NON_NULL, FIXED
                    // 2311: b"repr\0" as *c ... _char: typeof(_450 = move _451 as *const i8 (Misc)) = *const {l1639} i8
                    // 2311: b"repr\0" as *c ... _char:   l1639 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0" as *c ... st u8: typeof(_451 = move _452 as *const u8 (Pointer(ArrayToPointer))) = *const {l1638} u8
                    // 2311: b"repr\0" as *c ... st u8:   l1638 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0": typeof(_452 = &raw const (*_453)) = *const {l1637} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2311: b"repr\0":   l1637 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0": typeof(_453 = const b"repr\x00") = & {l1636} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2311: b"repr\0":   l1636 = UNIQUE | NON_NULL, (empty)
                    // 2311: b"repr\0" as *c ... _char: typeof(_449 = move _450 as *mut i8 (Misc)) = *mut {l1640} i8
                    // 2311: b"repr\0" as *c ... _char:   l1640 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2312: 0 as *const lib ... _char: typeof(_454) = *const {l739} i8
                    // 2312: 0 as *const lib ... _char:   l739 = UNIQUE | NON_NULL, (empty)
                    // 2312: 0 as *const lib ... _char: typeof(_454 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1641} i8
                    // 2312: 0 as *const lib ... _char:   l1641 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"failed\0" as *const u8 as *const libc::c_char) == 0 {
            // 2314: tok: typeof(_457) = *const {l743} i8
            // 2314: tok:   l743 = UNIQUE | NON_NULL, (empty)
            // 2314: tok: typeof(_458) = *mut {l745} i8
            // 2314: tok:   l745 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0" as  ... _char: typeof(_459) = *const {l747} i8
            // 2314: b"failed\0" as  ... _char:   l747 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0" as  ... st u8: typeof(_460) = *const {l749} u8
            // 2314: b"failed\0" as  ... st u8:   l749 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0": typeof(_461) = *const {l751} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2314: b"failed\0":   l751 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0": typeof(_462) = & {l753} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2314: b"failed\0":   l753 = UNIQUE | NON_NULL, FIXED
            // 2314: b"failed\0" as  ... st u8: typeof(_460 = move _461 as *const u8 (Pointer(ArrayToPointer))) = *const {l1645} u8
            // 2314: b"failed\0" as  ... st u8:   l1645 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0": typeof(_462 = const b"failed\x00") = & {l1643} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2314: b"failed\0":   l1643 = UNIQUE | NON_NULL, (empty)
            // 2314: tok: typeof(_457 = move _458 as *const i8 (Pointer(MutToConstPointer))) = *const {l1642} i8
            // 2314: tok:   l1642 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0": typeof(_461 = &raw const (*_462)) = *const {l1644} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2314: b"failed\0":   l1644 = UNIQUE | NON_NULL, (empty)
            // 2314: b"failed\0" as  ... _char: typeof(_459 = move _460 as *const i8 (Misc)) = *const {l1646} i8
            // 2314: b"failed\0" as  ... _char:   l1646 = UNIQUE | NON_NULL, (empty)
                event(
                    FAILED,
                    intarg(b"failed\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2317: b"failed\0" as  ... _char: typeof(_465) = *mut {l757} i8
                    // 2317: b"failed\0" as  ... _char:   l757 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0" as  ... _char: typeof(_466) = *const {l759} i8
                    // 2317: b"failed\0" as  ... _char:   l759 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0" as  ... st u8: typeof(_467) = *const {l761} u8
                    // 2317: b"failed\0" as  ... st u8:   l761 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0": typeof(_468) = *const {l763} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2317: b"failed\0":   l763 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0": typeof(_469) = & {l765} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2317: b"failed\0":   l765 = UNIQUE | NON_NULL, FIXED
                    // 2317: b"failed\0": typeof(_469 = const b"failed\x00") = & {l1647} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2317: b"failed\0":   l1647 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0" as  ... _char: typeof(_466 = move _467 as *const i8 (Misc)) = *const {l1650} i8
                    // 2317: b"failed\0" as  ... _char:   l1650 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0": typeof(_468 = &raw const (*_469)) = *const {l1648} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2317: b"failed\0":   l1648 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0" as  ... st u8: typeof(_467 = move _468 as *const u8 (Pointer(ArrayToPointer))) = *const {l1649} u8
                    // 2317: b"failed\0" as  ... st u8:   l1649 = UNIQUE | NON_NULL, (empty)
                    // 2317: b"failed\0" as  ... _char: typeof(_465 = move _466 as *mut i8 (Misc)) = *mut {l1651} i8
                    // 2317: b"failed\0" as  ... _char:   l1651 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2318: 0 as *const lib ... _char: typeof(_470) = *const {l767} i8
                    // 2318: 0 as *const lib ... _char:   l767 = UNIQUE | NON_NULL, (empty)
                    // 2318: 0 as *const lib ... _char: typeof(_470 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1652} i8
                    // 2318: 0 as *const lib ... _char:   l1652 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"assume\0" as *const u8 as *const libc::c_char) == 0 {
            // 2320: tok: typeof(_473) = *const {l771} i8
            // 2320: tok:   l771 = UNIQUE | NON_NULL, (empty)
            // 2320: tok: typeof(_474) = *mut {l773} i8
            // 2320: tok:   l773 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0" as  ... _char: typeof(_475) = *const {l775} i8
            // 2320: b"assume\0" as  ... _char:   l775 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0" as  ... st u8: typeof(_476) = *const {l777} u8
            // 2320: b"assume\0" as  ... st u8:   l777 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0": typeof(_477) = *const {l779} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2320: b"assume\0":   l779 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0": typeof(_478) = & {l781} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2320: b"assume\0":   l781 = UNIQUE | NON_NULL, FIXED
            // 2320: tok: typeof(_473 = move _474 as *const i8 (Pointer(MutToConstPointer))) = *const {l1653} i8
            // 2320: tok:   l1653 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0": typeof(_477 = &raw const (*_478)) = *const {l1655} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2320: b"assume\0":   l1655 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0" as  ... _char: typeof(_475 = move _476 as *const i8 (Misc)) = *const {l1657} i8
            // 2320: b"assume\0" as  ... _char:   l1657 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0" as  ... st u8: typeof(_476 = move _477 as *const u8 (Pointer(ArrayToPointer))) = *const {l1656} u8
            // 2320: b"assume\0" as  ... st u8:   l1656 = UNIQUE | NON_NULL, (empty)
            // 2320: b"assume\0": typeof(_478 = const b"assume\x00") = & {l1654} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2320: b"assume\0":   l1654 = UNIQUE | NON_NULL, (empty)
                event(
                    ASSUME,
                    intarg(b"assume\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2323: b"assume\0" as  ... _char: typeof(_481) = *mut {l785} i8
                    // 2323: b"assume\0" as  ... _char:   l785 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0" as  ... _char: typeof(_482) = *const {l787} i8
                    // 2323: b"assume\0" as  ... _char:   l787 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0" as  ... st u8: typeof(_483) = *const {l789} u8
                    // 2323: b"assume\0" as  ... st u8:   l789 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0": typeof(_484) = *const {l791} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2323: b"assume\0":   l791 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0": typeof(_485) = & {l793} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2323: b"assume\0":   l793 = UNIQUE | NON_NULL, FIXED
                    // 2323: b"assume\0": typeof(_484 = &raw const (*_485)) = *const {l1659} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2323: b"assume\0":   l1659 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0" as  ... _char: typeof(_481 = move _482 as *mut i8 (Misc)) = *mut {l1662} i8
                    // 2323: b"assume\0" as  ... _char:   l1662 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0" as  ... _char: typeof(_482 = move _483 as *const i8 (Misc)) = *const {l1661} i8
                    // 2323: b"assume\0" as  ... _char:   l1661 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0" as  ... st u8: typeof(_483 = move _484 as *const u8 (Pointer(ArrayToPointer))) = *const {l1660} u8
                    // 2323: b"assume\0" as  ... st u8:   l1660 = UNIQUE | NON_NULL, (empty)
                    // 2323: b"assume\0": typeof(_485 = const b"assume\x00") = & {l1658} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2323: b"assume\0":   l1658 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2324: 0 as *const lib ... _char: typeof(_486) = *const {l795} i8
                    // 2324: 0 as *const lib ... _char:   l795 = UNIQUE | NON_NULL, (empty)
                    // 2324: 0 as *const lib ... _char: typeof(_486 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1663} i8
                    // 2324: 0 as *const lib ... _char:   l1663 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"phase\0" as *const u8 as *const libc::c_char) == 0 {
            // 2326: tok: typeof(_489) = *const {l799} i8
            // 2326: tok:   l799 = UNIQUE | NON_NULL, (empty)
            // 2326: tok: typeof(_490) = *mut {l801} i8
            // 2326: tok:   l801 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0" as * ... _char: typeof(_491) = *const {l803} i8
            // 2326: b"phase\0" as * ... _char:   l803 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0" as * ... st u8: typeof(_492) = *const {l805} u8
            // 2326: b"phase\0" as * ... st u8:   l805 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0": typeof(_493) = *const {l807} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2326: b"phase\0":   l807 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0": typeof(_494) = & {l809} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2326: b"phase\0":   l809 = UNIQUE | NON_NULL, FIXED
            // 2326: b"phase\0" as * ... st u8: typeof(_492 = move _493 as *const u8 (Pointer(ArrayToPointer))) = *const {l1667} u8
            // 2326: b"phase\0" as * ... st u8:   l1667 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0": typeof(_494 = const b"phase\x00") = & {l1665} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2326: b"phase\0":   l1665 = UNIQUE | NON_NULL, (empty)
            // 2326: tok: typeof(_489 = move _490 as *const i8 (Pointer(MutToConstPointer))) = *const {l1664} i8
            // 2326: tok:   l1664 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0": typeof(_493 = &raw const (*_494)) = *const {l1666} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2326: b"phase\0":   l1666 = UNIQUE | NON_NULL, (empty)
            // 2326: b"phase\0" as * ... _char: typeof(_491 = move _492 as *const i8 (Misc)) = *const {l1668} i8
            // 2326: b"phase\0" as * ... _char:   l1668 = UNIQUE | NON_NULL, (empty)
                event(
                    PHASE,
                    intarg(b"phase\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2329: b"phase\0" as * ... _char: typeof(_497) = *mut {l813} i8
                    // 2329: b"phase\0" as * ... _char:   l813 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0" as * ... _char: typeof(_498) = *const {l815} i8
                    // 2329: b"phase\0" as * ... _char:   l815 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0" as * ... st u8: typeof(_499) = *const {l817} u8
                    // 2329: b"phase\0" as * ... st u8:   l817 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0": typeof(_500) = *const {l819} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2329: b"phase\0":   l819 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0": typeof(_501) = & {l821} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2329: b"phase\0":   l821 = UNIQUE | NON_NULL, FIXED
                    // 2329: b"phase\0" as * ... st u8: typeof(_499 = move _500 as *const u8 (Pointer(ArrayToPointer))) = *const {l1671} u8
                    // 2329: b"phase\0" as * ... st u8:   l1671 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0" as * ... _char: typeof(_498 = move _499 as *const i8 (Misc)) = *const {l1672} i8
                    // 2329: b"phase\0" as * ... _char:   l1672 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0" as * ... _char: typeof(_497 = move _498 as *mut i8 (Misc)) = *mut {l1673} i8
                    // 2329: b"phase\0" as * ... _char:   l1673 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0": typeof(_500 = &raw const (*_501)) = *const {l1670} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2329: b"phase\0":   l1670 = UNIQUE | NON_NULL, (empty)
                    // 2329: b"phase\0": typeof(_501 = const b"phase\x00") = & {l1669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2329: b"phase\0":   l1669 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2330: 0 as *const lib ... _char: typeof(_502) = *const {l823} i8
                    // 2330: 0 as *const lib ... _char:   l823 = UNIQUE | NON_NULL, (empty)
                    // 2330: 0 as *const lib ... _char: typeof(_502 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1674} i8
                    // 2330: 0 as *const lib ... _char:   l1674 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"init\0" as *const u8 as *const libc::c_char) == 0 {
            // 2332: tok: typeof(_505) = *const {l827} i8
            // 2332: tok:   l827 = UNIQUE | NON_NULL, (empty)
            // 2332: tok: typeof(_506) = *mut {l829} i8
            // 2332: tok:   l829 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0" as *c ... _char: typeof(_507) = *const {l831} i8
            // 2332: b"init\0" as *c ... _char:   l831 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0" as *c ... st u8: typeof(_508) = *const {l833} u8
            // 2332: b"init\0" as *c ... st u8:   l833 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0": typeof(_509) = *const {l835} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2332: b"init\0":   l835 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0": typeof(_510) = & {l837} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2332: b"init\0":   l837 = UNIQUE | NON_NULL, FIXED
            // 2332: b"init\0" as *c ... st u8: typeof(_508 = move _509 as *const u8 (Pointer(ArrayToPointer))) = *const {l1678} u8
            // 2332: b"init\0" as *c ... st u8:   l1678 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0": typeof(_509 = &raw const (*_510)) = *const {l1677} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2332: b"init\0":   l1677 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0" as *c ... _char: typeof(_507 = move _508 as *const i8 (Misc)) = *const {l1679} i8
            // 2332: b"init\0" as *c ... _char:   l1679 = UNIQUE | NON_NULL, (empty)
            // 2332: b"init\0": typeof(_510 = const b"init\x00") = & {l1676} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2332: b"init\0":   l1676 = UNIQUE | NON_NULL, (empty)
            // 2332: tok: typeof(_505 = move _506 as *const i8 (Pointer(MutToConstPointer))) = *const {l1675} i8
            // 2332: tok:   l1675 = UNIQUE | NON_NULL, (empty)
                noarg(INIT);
            } else if strcmp(tok, b"sat\0" as *const u8 as *const libc::c_char) == 0 {
            // 2334: tok: typeof(_514) = *const {l842} i8
            // 2334: tok:   l842 = UNIQUE | NON_NULL, (empty)
            // 2334: tok: typeof(_515) = *mut {l844} i8
            // 2334: tok:   l844 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0" as *co ... _char: typeof(_516) = *const {l846} i8
            // 2334: b"sat\0" as *co ... _char:   l846 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0" as *co ... st u8: typeof(_517) = *const {l848} u8
            // 2334: b"sat\0" as *co ... st u8:   l848 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0": typeof(_518) = *const {l850} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2334: b"sat\0":   l850 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0": typeof(_519) = & {l852} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2334: b"sat\0":   l852 = UNIQUE | NON_NULL, FIXED
            // 2334: b"sat\0": typeof(_519 = const b"sat\x00") = & {l1681} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2334: b"sat\0":   l1681 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0" as *co ... _char: typeof(_516 = move _517 as *const i8 (Misc)) = *const {l1684} i8
            // 2334: b"sat\0" as *co ... _char:   l1684 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0" as *co ... st u8: typeof(_517 = move _518 as *const u8 (Pointer(ArrayToPointer))) = *const {l1683} u8
            // 2334: b"sat\0" as *co ... st u8:   l1683 = UNIQUE | NON_NULL, (empty)
            // 2334: b"sat\0": typeof(_518 = &raw const (*_519)) = *const {l1682} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2334: b"sat\0":   l1682 = UNIQUE | NON_NULL, (empty)
            // 2334: tok: typeof(_514 = move _515 as *const i8 (Pointer(MutToConstPointer))) = *const {l1680} i8
            // 2334: tok:   l1680 = UNIQUE | NON_NULL, (empty)
                noarg(SAT);
            } else if strcmp(tok, b"simp\0" as *const u8 as *const libc::c_char) == 0 {
            // 2336: tok: typeof(_523) = *const {l857} i8
            // 2336: tok:   l857 = UNIQUE | NON_NULL, (empty)
            // 2336: tok: typeof(_524) = *mut {l859} i8
            // 2336: tok:   l859 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0" as *c ... _char: typeof(_525) = *const {l861} i8
            // 2336: b"simp\0" as *c ... _char:   l861 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0" as *c ... st u8: typeof(_526) = *const {l863} u8
            // 2336: b"simp\0" as *c ... st u8:   l863 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0": typeof(_527) = *const {l865} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2336: b"simp\0":   l865 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0": typeof(_528) = & {l867} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2336: b"simp\0":   l867 = UNIQUE | NON_NULL, FIXED
            // 2336: b"simp\0": typeof(_528 = const b"simp\x00") = & {l1686} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2336: b"simp\0":   l1686 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0": typeof(_527 = &raw const (*_528)) = *const {l1687} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2336: b"simp\0":   l1687 = UNIQUE | NON_NULL, (empty)
            // 2336: tok: typeof(_523 = move _524 as *const i8 (Pointer(MutToConstPointer))) = *const {l1685} i8
            // 2336: tok:   l1685 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0" as *c ... _char: typeof(_525 = move _526 as *const i8 (Misc)) = *const {l1689} i8
            // 2336: b"simp\0" as *c ... _char:   l1689 = UNIQUE | NON_NULL, (empty)
            // 2336: b"simp\0" as *c ... st u8: typeof(_526 = move _527 as *const u8 (Pointer(ArrayToPointer))) = *const {l1688} u8
            // 2336: b"simp\0" as *c ... st u8:   l1688 = UNIQUE | NON_NULL, (empty)
                event(
                    SIMP,
                    intarg(b"simp\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2339: b"simp\0" as *c ... _char: typeof(_531) = *mut {l871} i8
                    // 2339: b"simp\0" as *c ... _char:   l871 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0" as *c ... _char: typeof(_532) = *const {l873} i8
                    // 2339: b"simp\0" as *c ... _char:   l873 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0" as *c ... st u8: typeof(_533) = *const {l875} u8
                    // 2339: b"simp\0" as *c ... st u8:   l875 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0": typeof(_534) = *const {l877} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2339: b"simp\0":   l877 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0": typeof(_535) = & {l879} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2339: b"simp\0":   l879 = UNIQUE | NON_NULL, FIXED
                    // 2339: b"simp\0": typeof(_534 = &raw const (*_535)) = *const {l1691} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2339: b"simp\0":   l1691 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0" as *c ... _char: typeof(_532 = move _533 as *const i8 (Misc)) = *const {l1693} i8
                    // 2339: b"simp\0" as *c ... _char:   l1693 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0": typeof(_535 = const b"simp\x00") = & {l1690} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2339: b"simp\0":   l1690 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0" as *c ... _char: typeof(_531 = move _532 as *mut i8 (Misc)) = *mut {l1694} i8
                    // 2339: b"simp\0" as *c ... _char:   l1694 = UNIQUE | NON_NULL, (empty)
                    // 2339: b"simp\0" as *c ... st u8: typeof(_533 = move _534 as *const u8 (Pointer(ArrayToPointer))) = *const {l1692} u8
                    // 2339: b"simp\0" as *c ... st u8:   l1692 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2340: 0 as *const lib ... _char: typeof(_536) = *const {l881} i8
                    // 2340: 0 as *const lib ... _char:   l881 = UNIQUE | NON_NULL, (empty)
                    // 2340: 0 as *const lib ... _char: typeof(_536 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1695} i8
                    // 2340: 0 as *const lib ... _char:   l1695 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"setphases\0" as *const u8 as *const libc::c_char) == 0 {
            // 2342: tok: typeof(_539) = *const {l885} i8
            // 2342: tok:   l885 = UNIQUE | NON_NULL, (empty)
            // 2342: tok: typeof(_540) = *mut {l887} i8
            // 2342: tok:   l887 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0"  ... _char: typeof(_541) = *const {l889} i8
            // 2342: b"setphases\0"  ... _char:   l889 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0"  ... st u8: typeof(_542) = *const {l891} u8
            // 2342: b"setphases\0"  ... st u8:   l891 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0": typeof(_543) = *const {l893} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2342: b"setphases\0":   l893 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0": typeof(_544) = & {l895} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2342: b"setphases\0":   l895 = UNIQUE | NON_NULL, FIXED
            // 2342: tok: typeof(_539 = move _540 as *const i8 (Pointer(MutToConstPointer))) = *const {l1696} i8
            // 2342: tok:   l1696 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0"  ... _char: typeof(_541 = move _542 as *const i8 (Misc)) = *const {l1700} i8
            // 2342: b"setphases\0"  ... _char:   l1700 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0": typeof(_543 = &raw const (*_544)) = *const {l1698} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2342: b"setphases\0":   l1698 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0": typeof(_544 = const b"setphases\x00") = & {l1697} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2342: b"setphases\0":   l1697 = UNIQUE | NON_NULL, (empty)
            // 2342: b"setphases\0"  ... st u8: typeof(_542 = move _543 as *const u8 (Pointer(ArrayToPointer))) = *const {l1699} u8
            // 2342: b"setphases\0"  ... st u8:   l1699 = UNIQUE | NON_NULL, (empty)
                noarg(SETPHASES);
            } else if strcmp(tok, b"freeze\0" as *const u8 as *const libc::c_char) == 0 {
            // 2344: tok: typeof(_548) = *const {l900} i8
            // 2344: tok:   l900 = UNIQUE | NON_NULL, (empty)
            // 2344: tok: typeof(_549) = *mut {l902} i8
            // 2344: tok:   l902 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0" as  ... _char: typeof(_550) = *const {l904} i8
            // 2344: b"freeze\0" as  ... _char:   l904 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0" as  ... st u8: typeof(_551) = *const {l906} u8
            // 2344: b"freeze\0" as  ... st u8:   l906 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0": typeof(_552) = *const {l908} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2344: b"freeze\0":   l908 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0": typeof(_553) = & {l910} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2344: b"freeze\0":   l910 = UNIQUE | NON_NULL, FIXED
            // 2344: b"freeze\0": typeof(_552 = &raw const (*_553)) = *const {l1703} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2344: b"freeze\0":   l1703 = UNIQUE | NON_NULL, (empty)
            // 2344: tok: typeof(_548 = move _549 as *const i8 (Pointer(MutToConstPointer))) = *const {l1701} i8
            // 2344: tok:   l1701 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0" as  ... st u8: typeof(_551 = move _552 as *const u8 (Pointer(ArrayToPointer))) = *const {l1704} u8
            // 2344: b"freeze\0" as  ... st u8:   l1704 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0": typeof(_553 = const b"freeze\x00") = & {l1702} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2344: b"freeze\0":   l1702 = UNIQUE | NON_NULL, (empty)
            // 2344: b"freeze\0" as  ... _char: typeof(_550 = move _551 as *const i8 (Misc)) = *const {l1705} i8
            // 2344: b"freeze\0" as  ... _char:   l1705 = UNIQUE | NON_NULL, (empty)
                event(
                    FREEZE,
                    intarg(b"freeze\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2347: b"freeze\0" as  ... _char: typeof(_556) = *mut {l914} i8
                    // 2347: b"freeze\0" as  ... _char:   l914 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0" as  ... _char: typeof(_557) = *const {l916} i8
                    // 2347: b"freeze\0" as  ... _char:   l916 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0" as  ... st u8: typeof(_558) = *const {l918} u8
                    // 2347: b"freeze\0" as  ... st u8:   l918 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0": typeof(_559) = *const {l920} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2347: b"freeze\0":   l920 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0": typeof(_560) = & {l922} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2347: b"freeze\0":   l922 = UNIQUE | NON_NULL, FIXED
                    // 2347: b"freeze\0" as  ... st u8: typeof(_558 = move _559 as *const u8 (Pointer(ArrayToPointer))) = *const {l1708} u8
                    // 2347: b"freeze\0" as  ... st u8:   l1708 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0": typeof(_559 = &raw const (*_560)) = *const {l1707} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2347: b"freeze\0":   l1707 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0" as  ... _char: typeof(_556 = move _557 as *mut i8 (Misc)) = *mut {l1710} i8
                    // 2347: b"freeze\0" as  ... _char:   l1710 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0": typeof(_560 = const b"freeze\x00") = & {l1706} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2347: b"freeze\0":   l1706 = UNIQUE | NON_NULL, (empty)
                    // 2347: b"freeze\0" as  ... _char: typeof(_557 = move _558 as *const i8 (Misc)) = *const {l1709} i8
                    // 2347: b"freeze\0" as  ... _char:   l1709 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2348: 0 as *const lib ... _char: typeof(_561) = *const {l924} i8
                    // 2348: 0 as *const lib ... _char:   l924 = UNIQUE | NON_NULL, (empty)
                    // 2348: 0 as *const lib ... _char: typeof(_561 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1711} i8
                    // 2348: 0 as *const lib ... _char:   l1711 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"setimportant\0" as *const u8 as *const libc::c_char) == 0 {
            // 2350: tok: typeof(_564) = *const {l928} i8
            // 2350: tok:   l928 = UNIQUE | NON_NULL, (empty)
            // 2350: tok: typeof(_565) = *mut {l930} i8
            // 2350: tok:   l930 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\ ... _char: typeof(_566) = *const {l932} i8
            // 2350: b"setimportant\ ... _char:   l932 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\ ... st u8: typeof(_567) = *const {l934} u8
            // 2350: b"setimportant\ ... st u8:   l934 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\0": typeof(_568) = *const {l936} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2350: b"setimportant\0":   l936 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\0": typeof(_569) = & {l938} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2350: b"setimportant\0":   l938 = UNIQUE | NON_NULL, FIXED
            // 2350: tok: typeof(_564 = move _565 as *const i8 (Pointer(MutToConstPointer))) = *const {l1712} i8
            // 2350: tok:   l1712 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\ ... _char: typeof(_566 = move _567 as *const i8 (Misc)) = *const {l1716} i8
            // 2350: b"setimportant\ ... _char:   l1716 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\0": typeof(_569 = const b"setimportant\x00") = & {l1713} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2350: b"setimportant\0":   l1713 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\0": typeof(_568 = &raw const (*_569)) = *const {l1714} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2350: b"setimportant\0":   l1714 = UNIQUE | NON_NULL, (empty)
            // 2350: b"setimportant\ ... st u8: typeof(_567 = move _568 as *const u8 (Pointer(ArrayToPointer))) = *const {l1715} u8
            // 2350: b"setimportant\ ... st u8:   l1715 = UNIQUE | NON_NULL, (empty)
                event(
                    SETIMPORTANT,
                    intarg(
                        b"setimportant\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        // 2354: b"setimportant\ ... _char: typeof(_572) = *mut {l942} i8
                        // 2354: b"setimportant\ ... _char:   l942 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\ ... _char: typeof(_573) = *const {l944} i8
                        // 2354: b"setimportant\ ... _char:   l944 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\ ... st u8: typeof(_574) = *const {l946} u8
                        // 2354: b"setimportant\ ... st u8:   l946 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\0": typeof(_575) = *const {l948} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 2354: b"setimportant\0":   l948 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\0": typeof(_576) = & {l950} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 2354: b"setimportant\0":   l950 = UNIQUE | NON_NULL, FIXED
                        // 2354: b"setimportant\ ... _char: typeof(_573 = move _574 as *const i8 (Misc)) = *const {l1720} i8
                        // 2354: b"setimportant\ ... _char:   l1720 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\ ... _char: typeof(_572 = move _573 as *mut i8 (Misc)) = *mut {l1721} i8
                        // 2354: b"setimportant\ ... _char:   l1721 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\0": typeof(_575 = &raw const (*_576)) = *const {l1718} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 2354: b"setimportant\0":   l1718 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\0": typeof(_576 = const b"setimportant\x00") = & {l1717} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                        // 2354: b"setimportant\0":   l1717 = UNIQUE | NON_NULL, (empty)
                        // 2354: b"setimportant\ ... st u8: typeof(_574 = move _575 as *const u8 (Pointer(ArrayToPointer))) = *const {l1719} u8
                        // 2354: b"setimportant\ ... st u8:   l1719 = UNIQUE | NON_NULL, (empty)
                    ),
                    0 as *const libc::c_char,
                    // 2356: 0 as *const lib ... _char: typeof(_577) = *const {l952} i8
                    // 2356: 0 as *const lib ... _char:   l952 = UNIQUE | NON_NULL, (empty)
                    // 2356: 0 as *const lib ... _char: typeof(_577 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1722} i8
                    // 2356: 0 as *const lib ... _char:   l1722 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"setphase\0" as *const u8 as *const libc::c_char) == 0 {
            // 2358: tok: typeof(_580) = *const {l956} i8
            // 2358: tok:   l956 = UNIQUE | NON_NULL, (empty)
            // 2358: tok: typeof(_581) = *mut {l958} i8
            // 2358: tok:   l958 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0" a ... _char: typeof(_582) = *const {l960} i8
            // 2358: b"setphase\0" a ... _char:   l960 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0" a ... st u8: typeof(_583) = *const {l962} u8
            // 2358: b"setphase\0" a ... st u8:   l962 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0": typeof(_584) = *const {l964} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2358: b"setphase\0":   l964 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0": typeof(_585) = & {l966} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2358: b"setphase\0":   l966 = UNIQUE | NON_NULL, FIXED
            // 2358: b"setphase\0" a ... st u8: typeof(_583 = move _584 as *const u8 (Pointer(ArrayToPointer))) = *const {l1726} u8
            // 2358: b"setphase\0" a ... st u8:   l1726 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0": typeof(_584 = &raw const (*_585)) = *const {l1725} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2358: b"setphase\0":   l1725 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0": typeof(_585 = const b"setphase\x00") = & {l1724} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2358: b"setphase\0":   l1724 = UNIQUE | NON_NULL, (empty)
            // 2358: b"setphase\0" a ... _char: typeof(_582 = move _583 as *const i8 (Misc)) = *const {l1727} i8
            // 2358: b"setphase\0" a ... _char:   l1727 = UNIQUE | NON_NULL, (empty)
            // 2358: tok: typeof(_580 = move _581 as *const i8 (Pointer(MutToConstPointer))) = *const {l1723} i8
            // 2358: tok:   l1723 = UNIQUE | NON_NULL, (empty)
                event(
                    SETPHASE,
                    intarg(b"setphase\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2361: b"setphase\0" a ... _char: typeof(_588) = *mut {l970} i8
                    // 2361: b"setphase\0" a ... _char:   l970 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0" a ... _char: typeof(_589) = *const {l972} i8
                    // 2361: b"setphase\0" a ... _char:   l972 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0" a ... st u8: typeof(_590) = *const {l974} u8
                    // 2361: b"setphase\0" a ... st u8:   l974 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0": typeof(_591) = *const {l976} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2361: b"setphase\0":   l976 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0": typeof(_592) = & {l978} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2361: b"setphase\0":   l978 = UNIQUE | NON_NULL, FIXED
                    // 2361: b"setphase\0": typeof(_592 = const b"setphase\x00") = & {l1728} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2361: b"setphase\0":   l1728 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0" a ... _char: typeof(_588 = move _589 as *mut i8 (Misc)) = *mut {l1732} i8
                    // 2361: b"setphase\0" a ... _char:   l1732 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0" a ... st u8: typeof(_590 = move _591 as *const u8 (Pointer(ArrayToPointer))) = *const {l1730} u8
                    // 2361: b"setphase\0" a ... st u8:   l1730 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0": typeof(_591 = &raw const (*_592)) = *const {l1729} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 2361: b"setphase\0":   l1729 = UNIQUE | NON_NULL, (empty)
                    // 2361: b"setphase\0" a ... _char: typeof(_589 = move _590 as *const i8 (Misc)) = *const {l1731} i8
                    // 2361: b"setphase\0" a ... _char:   l1731 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2362: 0 as *const lib ... _char: typeof(_593) = *const {l980} i8
                    // 2362: 0 as *const lib ... _char:   l980 = UNIQUE | NON_NULL, (empty)
                    // 2362: 0 as *const lib ... _char: typeof(_593 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1733} i8
                    // 2362: 0 as *const lib ... _char:   l1733 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"resetphase\0" as *const u8 as *const libc::c_char) == 0 {
            // 2364: tok: typeof(_596) = *const {l984} i8
            // 2364: tok:   l984 = UNIQUE | NON_NULL, (empty)
            // 2364: tok: typeof(_597) = *mut {l986} i8
            // 2364: tok:   l986 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0" ... _char: typeof(_598) = *const {l988} i8
            // 2364: b"resetphase\0" ... _char:   l988 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0" ... st u8: typeof(_599) = *const {l990} u8
            // 2364: b"resetphase\0" ... st u8:   l990 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0": typeof(_600) = *const {l992} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2364: b"resetphase\0":   l992 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0": typeof(_601) = & {l994} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2364: b"resetphase\0":   l994 = UNIQUE | NON_NULL, FIXED
            // 2364: b"resetphase\0": typeof(_600 = &raw const (*_601)) = *const {l1736} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2364: b"resetphase\0":   l1736 = UNIQUE | NON_NULL, (empty)
            // 2364: tok: typeof(_596 = move _597 as *const i8 (Pointer(MutToConstPointer))) = *const {l1734} i8
            // 2364: tok:   l1734 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0" ... st u8: typeof(_599 = move _600 as *const u8 (Pointer(ArrayToPointer))) = *const {l1737} u8
            // 2364: b"resetphase\0" ... st u8:   l1737 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0" ... _char: typeof(_598 = move _599 as *const i8 (Misc)) = *const {l1738} i8
            // 2364: b"resetphase\0" ... _char:   l1738 = UNIQUE | NON_NULL, (empty)
            // 2364: b"resetphase\0": typeof(_601 = const b"resetphase\x00") = & {l1735} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2364: b"resetphase\0":   l1735 = UNIQUE | NON_NULL, (empty)
                event(
                    SETPHASE,
                    intarg(
                        b"resetphase\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        // 2368: b"resetphase\0" ... _char: typeof(_604) = *mut {l998} i8
                        // 2368: b"resetphase\0" ... _char:   l998 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0" ... _char: typeof(_605) = *const {l1000} i8
                        // 2368: b"resetphase\0" ... _char:   l1000 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0" ... st u8: typeof(_606) = *const {l1002} u8
                        // 2368: b"resetphase\0" ... st u8:   l1002 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0": typeof(_607) = *const {l1004} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2368: b"resetphase\0":   l1004 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0": typeof(_608) = & {l1006} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2368: b"resetphase\0":   l1006 = UNIQUE | NON_NULL, FIXED
                        // 2368: b"resetphase\0" ... _char: typeof(_604 = move _605 as *mut i8 (Misc)) = *mut {l1743} i8
                        // 2368: b"resetphase\0" ... _char:   l1743 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0": typeof(_607 = &raw const (*_608)) = *const {l1740} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2368: b"resetphase\0":   l1740 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0" ... st u8: typeof(_606 = move _607 as *const u8 (Pointer(ArrayToPointer))) = *const {l1741} u8
                        // 2368: b"resetphase\0" ... st u8:   l1741 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0" ... _char: typeof(_605 = move _606 as *const i8 (Misc)) = *const {l1742} i8
                        // 2368: b"resetphase\0" ... _char:   l1742 = UNIQUE | NON_NULL, (empty)
                        // 2368: b"resetphase\0": typeof(_608 = const b"resetphase\x00") = & {l1739} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2368: b"resetphase\0":   l1739 = UNIQUE | NON_NULL, (empty)
                    ),
                    0 as *const libc::c_char,
                    // 2370: 0 as *const lib ... _char: typeof(_609) = *const {l1008} i8
                    // 2370: 0 as *const lib ... _char:   l1008 = UNIQUE | NON_NULL, (empty)
                    // 2370: 0 as *const lib ... _char: typeof(_609 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1744} i8
                    // 2370: 0 as *const lib ... _char:   l1744 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"melt\0" as *const u8 as *const libc::c_char) == 0 {
            // 2372: tok: typeof(_612) = *const {l1012} i8
            // 2372: tok:   l1012 = UNIQUE | NON_NULL, (empty)
            // 2372: tok: typeof(_613) = *mut {l1014} i8
            // 2372: tok:   l1014 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0" as *c ... _char: typeof(_614) = *const {l1016} i8
            // 2372: b"melt\0" as *c ... _char:   l1016 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0" as *c ... st u8: typeof(_615) = *const {l1018} u8
            // 2372: b"melt\0" as *c ... st u8:   l1018 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0": typeof(_616) = *const {l1020} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2372: b"melt\0":   l1020 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0": typeof(_617) = & {l1022} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2372: b"melt\0":   l1022 = UNIQUE | NON_NULL, FIXED
            // 2372: tok: typeof(_612 = move _613 as *const i8 (Pointer(MutToConstPointer))) = *const {l1745} i8
            // 2372: tok:   l1745 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0" as *c ... _char: typeof(_614 = move _615 as *const i8 (Misc)) = *const {l1749} i8
            // 2372: b"melt\0" as *c ... _char:   l1749 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0" as *c ... st u8: typeof(_615 = move _616 as *const u8 (Pointer(ArrayToPointer))) = *const {l1748} u8
            // 2372: b"melt\0" as *c ... st u8:   l1748 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0": typeof(_617 = const b"melt\x00") = & {l1746} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2372: b"melt\0":   l1746 = UNIQUE | NON_NULL, (empty)
            // 2372: b"melt\0": typeof(_616 = &raw const (*_617)) = *const {l1747} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2372: b"melt\0":   l1747 = UNIQUE | NON_NULL, (empty)
                event(
                    MELT,
                    intarg(b"melt\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2375: b"melt\0" as *c ... _char: typeof(_620) = *mut {l1026} i8
                    // 2375: b"melt\0" as *c ... _char:   l1026 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0" as *c ... _char: typeof(_621) = *const {l1028} i8
                    // 2375: b"melt\0" as *c ... _char:   l1028 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0" as *c ... st u8: typeof(_622) = *const {l1030} u8
                    // 2375: b"melt\0" as *c ... st u8:   l1030 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0": typeof(_623) = *const {l1032} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2375: b"melt\0":   l1032 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0": typeof(_624) = & {l1034} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2375: b"melt\0":   l1034 = UNIQUE | NON_NULL, FIXED
                    // 2375: b"melt\0" as *c ... st u8: typeof(_622 = move _623 as *const u8 (Pointer(ArrayToPointer))) = *const {l1752} u8
                    // 2375: b"melt\0" as *c ... st u8:   l1752 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0" as *c ... _char: typeof(_620 = move _621 as *mut i8 (Misc)) = *mut {l1754} i8
                    // 2375: b"melt\0" as *c ... _char:   l1754 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0": typeof(_624 = const b"melt\x00") = & {l1750} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2375: b"melt\0":   l1750 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0" as *c ... _char: typeof(_621 = move _622 as *const i8 (Misc)) = *const {l1753} i8
                    // 2375: b"melt\0" as *c ... _char:   l1753 = UNIQUE | NON_NULL, (empty)
                    // 2375: b"melt\0": typeof(_623 = &raw const (*_624)) = *const {l1751} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 2375: b"melt\0":   l1751 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2376: 0 as *const lib ... _char: typeof(_625) = *const {l1036} i8
                    // 2376: 0 as *const lib ... _char:   l1036 = UNIQUE | NON_NULL, (empty)
                    // 2376: 0 as *const lib ... _char: typeof(_625 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1755} i8
                    // 2376: 0 as *const lib ... _char:   l1755 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"reuse\0" as *const u8 as *const libc::c_char) == 0 {
            // 2378: tok: typeof(_628) = *const {l1040} i8
            // 2378: tok:   l1040 = UNIQUE | NON_NULL, (empty)
            // 2378: tok: typeof(_629) = *mut {l1042} i8
            // 2378: tok:   l1042 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0" as * ... _char: typeof(_630) = *const {l1044} i8
            // 2378: b"reuse\0" as * ... _char:   l1044 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0" as * ... st u8: typeof(_631) = *const {l1046} u8
            // 2378: b"reuse\0" as * ... st u8:   l1046 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0": typeof(_632) = *const {l1048} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2378: b"reuse\0":   l1048 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0": typeof(_633) = & {l1050} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2378: b"reuse\0":   l1050 = UNIQUE | NON_NULL, FIXED
            // 2378: b"reuse\0": typeof(_633 = const b"reuse\x00") = & {l1757} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2378: b"reuse\0":   l1757 = UNIQUE | NON_NULL, (empty)
            // 2378: tok: typeof(_628 = move _629 as *const i8 (Pointer(MutToConstPointer))) = *const {l1756} i8
            // 2378: tok:   l1756 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0" as * ... st u8: typeof(_631 = move _632 as *const u8 (Pointer(ArrayToPointer))) = *const {l1759} u8
            // 2378: b"reuse\0" as * ... st u8:   l1759 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0" as * ... _char: typeof(_630 = move _631 as *const i8 (Misc)) = *const {l1760} i8
            // 2378: b"reuse\0" as * ... _char:   l1760 = UNIQUE | NON_NULL, (empty)
            // 2378: b"reuse\0": typeof(_632 = &raw const (*_633)) = *const {l1758} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2378: b"reuse\0":   l1758 = UNIQUE | NON_NULL, (empty)
                event(
                    REUSE,
                    intarg(b"reuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2381: b"reuse\0" as * ... _char: typeof(_636) = *mut {l1054} i8
                    // 2381: b"reuse\0" as * ... _char:   l1054 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0" as * ... _char: typeof(_637) = *const {l1056} i8
                    // 2381: b"reuse\0" as * ... _char:   l1056 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0" as * ... st u8: typeof(_638) = *const {l1058} u8
                    // 2381: b"reuse\0" as * ... st u8:   l1058 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0": typeof(_639) = *const {l1060} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2381: b"reuse\0":   l1060 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0": typeof(_640) = & {l1062} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2381: b"reuse\0":   l1062 = UNIQUE | NON_NULL, FIXED
                    // 2381: b"reuse\0": typeof(_639 = &raw const (*_640)) = *const {l1762} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2381: b"reuse\0":   l1762 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0" as * ... _char: typeof(_637 = move _638 as *const i8 (Misc)) = *const {l1764} i8
                    // 2381: b"reuse\0" as * ... _char:   l1764 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0" as * ... _char: typeof(_636 = move _637 as *mut i8 (Misc)) = *mut {l1765} i8
                    // 2381: b"reuse\0" as * ... _char:   l1765 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0": typeof(_640 = const b"reuse\x00") = & {l1761} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 2381: b"reuse\0":   l1761 = UNIQUE | NON_NULL, (empty)
                    // 2381: b"reuse\0" as * ... st u8: typeof(_638 = move _639 as *const u8 (Pointer(ArrayToPointer))) = *const {l1763} u8
                    // 2381: b"reuse\0" as * ... st u8:   l1763 = UNIQUE | NON_NULL, (empty)
                    0 as *const libc::c_char,
                    // 2382: 0 as *const lib ... _char: typeof(_641) = *const {l1064} i8
                    // 2382: 0 as *const lib ... _char:   l1064 = UNIQUE | NON_NULL, (empty)
                    // 2382: 0 as *const lib ... _char: typeof(_641 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1766} i8
                    // 2382: 0 as *const lib ... _char:   l1766 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"option\0" as *const u8 as *const libc::c_char) == 0 {
            // 2384: tok: typeof(_644) = *const {l1068} i8
            // 2384: tok:   l1068 = UNIQUE | NON_NULL, (empty)
            // 2384: tok: typeof(_645) = *mut {l1070} i8
            // 2384: tok:   l1070 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0" as  ... _char: typeof(_646) = *const {l1072} i8
            // 2384: b"option\0" as  ... _char:   l1072 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0" as  ... st u8: typeof(_647) = *const {l1074} u8
            // 2384: b"option\0" as  ... st u8:   l1074 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0": typeof(_648) = *const {l1076} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2384: b"option\0":   l1076 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0": typeof(_649) = & {l1078} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2384: b"option\0":   l1078 = UNIQUE | NON_NULL, FIXED
            // 2384: b"option\0": typeof(_649 = const b"option\x00") = & {l1768} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2384: b"option\0":   l1768 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0": typeof(_648 = &raw const (*_649)) = *const {l1769} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2384: b"option\0":   l1769 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0" as  ... st u8: typeof(_647 = move _648 as *const u8 (Pointer(ArrayToPointer))) = *const {l1770} u8
            // 2384: b"option\0" as  ... st u8:   l1770 = UNIQUE | NON_NULL, (empty)
            // 2384: b"option\0" as  ... _char: typeof(_646 = move _647 as *const i8 (Misc)) = *const {l1771} i8
            // 2384: b"option\0" as  ... _char:   l1771 = UNIQUE | NON_NULL, (empty)
            // 2384: tok: typeof(_644 = move _645 as *const i8 (Pointer(MutToConstPointer))) = *const {l1767} i8
            // 2384: tok:   l1767 = UNIQUE | NON_NULL, (empty)
                opt = strtok(
                // 2385: strtok( 0 as *m ... ar, ): typeof(_650) = *mut {l1080} i8
                // 2385: strtok( 0 as *m ... ar, ):   l1080 = UNIQUE | NON_NULL, (empty)
                    0 as *mut libc::c_char,
                    // 2386: 0 as *mut libc: ... _char: typeof(_651) = *mut {l1082} i8
                    // 2386: 0 as *mut libc: ... _char:   l1082 = UNIQUE | NON_NULL, (empty)
                    // 2386: 0 as *mut libc: ... _char: typeof(_651 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l1772} i8
                    // 2386: 0 as *mut libc: ... _char:   l1772 = UNIQUE | NON_NULL, (empty)
                    b" \0" as *const u8 as *const libc::c_char,
                    // 2387: b" \0" as *cons ... _char: typeof(_652) = *const {l1084} i8
                    // 2387: b" \0" as *cons ... _char:   l1084 = UNIQUE | NON_NULL, (empty)
                    // 2387: b" \0" as *const u8: typeof(_653) = *const {l1086} u8
                    // 2387: b" \0" as *const u8:   l1086 = UNIQUE | NON_NULL, (empty)
                    // 2387: b" \0": typeof(_654) = *const {l1088} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2387: b" \0":   l1088 = UNIQUE | NON_NULL, (empty)
                    // 2387: b" \0": typeof(_655) = & {l1090} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2387: b" \0":   l1090 = UNIQUE | NON_NULL, FIXED
                    // 2387: b" \0" as *cons ... _char: typeof(_652 = move _653 as *const i8 (Misc)) = *const {l1776} i8
                    // 2387: b" \0" as *cons ... _char:   l1776 = UNIQUE | NON_NULL, (empty)
                    // 2387: b" \0" as *const u8: typeof(_653 = move _654 as *const u8 (Pointer(ArrayToPointer))) = *const {l1775} u8
                    // 2387: b" \0" as *const u8:   l1775 = UNIQUE | NON_NULL, (empty)
                    // 2387: b" \0": typeof(_655 = const b" \x00") = & {l1773} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2387: b" \0":   l1773 = UNIQUE | NON_NULL, (empty)
                    // 2387: b" \0": typeof(_654 = &raw const (*_655)) = *const {l1774} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2387: b" \0":   l1774 = UNIQUE | NON_NULL, (empty)
                );
                if opt.is_null() {
                // 2389: opt: typeof(_658) = *mut {l1094} i8
                // 2389: opt:   l1094 = UNIQUE | NON_NULL, (empty)
                    perr(b"option iname missing\0" as *const u8 as *const libc::c_char);
                    // 2390: b"option iname  ... _char: typeof(_660) = *const {l1097} i8
                    // 2390: b"option iname  ... _char:   l1097 = UNIQUE | NON_NULL, (empty)
                    // 2390: b"option iname  ... st u8: typeof(_661) = *const {l1099} u8
                    // 2390: b"option iname  ... st u8:   l1099 = UNIQUE | NON_NULL, (empty)
                    // 2390: b"option iname  ... ng\0": typeof(_662) = *const {l1101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2390: b"option iname  ... ng\0":   l1101 = UNIQUE | NON_NULL, (empty)
                    // 2390: b"option iname  ... ng\0": typeof(_663) = & {l1103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2390: b"option iname  ... ng\0":   l1103 = UNIQUE | NON_NULL, FIXED
                    // 2390: b"option iname  ... ng\0": typeof(_663 = const b"option iname missing\x00") = & {l1777} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2390: b"option iname  ... ng\0":   l1777 = UNIQUE | NON_NULL, (empty)
                    // 2390: b"option iname  ... _char: typeof(_660 = move _661 as *const i8 (Misc)) = *const {l1780} i8
                    // 2390: b"option iname  ... _char:   l1780 = UNIQUE | NON_NULL, (empty)
                    // 2390: b"option iname  ... ng\0": typeof(_662 = &raw const (*_663)) = *const {l1778} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2390: b"option iname  ... ng\0":   l1778 = UNIQUE | NON_NULL, (empty)
                    // 2390: b"option iname  ... st u8: typeof(_661 = move _662 as *const u8 (Pointer(ArrayToPointer))) = *const {l1779} u8
                    // 2390: b"option iname  ... st u8:   l1779 = UNIQUE | NON_NULL, (empty)
                }
                event(
                    OPTION,
                    intarg(b"option\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
                    // 2394: b"option\0" as  ... _char: typeof(_666) = *mut {l1107} i8
                    // 2394: b"option\0" as  ... _char:   l1107 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0" as  ... _char: typeof(_667) = *const {l1109} i8
                    // 2394: b"option\0" as  ... _char:   l1109 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0" as  ... st u8: typeof(_668) = *const {l1111} u8
                    // 2394: b"option\0" as  ... st u8:   l1111 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0": typeof(_669) = *const {l1113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2394: b"option\0":   l1113 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0": typeof(_670) = & {l1115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2394: b"option\0":   l1115 = UNIQUE | NON_NULL, FIXED
                    // 2394: b"option\0" as  ... _char: typeof(_666 = move _667 as *mut i8 (Misc)) = *mut {l1785} i8
                    // 2394: b"option\0" as  ... _char:   l1785 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0": typeof(_669 = &raw const (*_670)) = *const {l1782} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2394: b"option\0":   l1782 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0": typeof(_670 = const b"option\x00") = & {l1781} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2394: b"option\0":   l1781 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0" as  ... st u8: typeof(_668 = move _669 as *const u8 (Pointer(ArrayToPointer))) = *const {l1783} u8
                    // 2394: b"option\0" as  ... st u8:   l1783 = UNIQUE | NON_NULL, (empty)
                    // 2394: b"option\0" as  ... _char: typeof(_667 = move _668 as *const i8 (Misc)) = *const {l1784} i8
                    // 2394: b"option\0" as  ... _char:   l1784 = UNIQUE | NON_NULL, (empty)
                    opt,
                    // 2395: opt: typeof(_671) = *const {l1117} i8
                    // 2395: opt:   l1117 = UNIQUE | NON_NULL, (empty)
                    // 2395: opt: typeof(_672) = *mut {l1119} i8
                    // 2395: opt:   l1119 = UNIQUE | NON_NULL, (empty)
                    // 2395: opt: typeof(_671 = move _672 as *const i8 (Pointer(MutToConstPointer))) = *const {l1786} i8
                    // 2395: opt:   l1786 = UNIQUE | NON_NULL, (empty)
                );
            } else if strcmp(tok, b"release\0" as *const u8 as *const libc::c_char) == 0 {
            // 2397: tok: typeof(_675) = *const {l1123} i8
            // 2397: tok:   l1123 = UNIQUE | NON_NULL, (empty)
            // 2397: tok: typeof(_676) = *mut {l1125} i8
            // 2397: tok:   l1125 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0" as ... _char: typeof(_677) = *const {l1127} i8
            // 2397: b"release\0" as ... _char:   l1127 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0" as ... st u8: typeof(_678) = *const {l1129} u8
            // 2397: b"release\0" as ... st u8:   l1129 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0": typeof(_679) = *const {l1131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2397: b"release\0":   l1131 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0": typeof(_680) = & {l1133} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2397: b"release\0":   l1133 = UNIQUE | NON_NULL, FIXED
            // 2397: b"release\0": typeof(_679 = &raw const (*_680)) = *const {l1789} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2397: b"release\0":   l1789 = UNIQUE | NON_NULL, (empty)
            // 2397: tok: typeof(_675 = move _676 as *const i8 (Pointer(MutToConstPointer))) = *const {l1787} i8
            // 2397: tok:   l1787 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0": typeof(_680 = const b"release\x00") = & {l1788} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2397: b"release\0":   l1788 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0" as ... st u8: typeof(_678 = move _679 as *const u8 (Pointer(ArrayToPointer))) = *const {l1790} u8
            // 2397: b"release\0" as ... st u8:   l1790 = UNIQUE | NON_NULL, (empty)
            // 2397: b"release\0" as ... _char: typeof(_677 = move _678 as *const i8 (Misc)) = *const {l1791} i8
            // 2397: b"release\0" as ... _char:   l1791 = UNIQUE | NON_NULL, (empty)
                noarg(RELEASE);
            } else if strcmp(tok, b"incvar\0" as *const u8 as *const libc::c_char) == 0 {
            // 2399: tok: typeof(_684) = *const {l1138} i8
            // 2399: tok:   l1138 = UNIQUE | NON_NULL, (empty)
            // 2399: tok: typeof(_685) = *mut {l1140} i8
            // 2399: tok:   l1140 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0" as  ... _char: typeof(_686) = *const {l1142} i8
            // 2399: b"incvar\0" as  ... _char:   l1142 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0" as  ... st u8: typeof(_687) = *const {l1144} u8
            // 2399: b"incvar\0" as  ... st u8:   l1144 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0": typeof(_688) = *const {l1146} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2399: b"incvar\0":   l1146 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0": typeof(_689) = & {l1148} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2399: b"incvar\0":   l1148 = UNIQUE | NON_NULL, FIXED
            // 2399: b"incvar\0": typeof(_688 = &raw const (*_689)) = *const {l1794} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2399: b"incvar\0":   l1794 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0": typeof(_689 = const b"incvar\x00") = & {l1793} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2399: b"incvar\0":   l1793 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0" as  ... _char: typeof(_686 = move _687 as *const i8 (Misc)) = *const {l1796} i8
            // 2399: b"incvar\0" as  ... _char:   l1796 = UNIQUE | NON_NULL, (empty)
            // 2399: tok: typeof(_684 = move _685 as *const i8 (Pointer(MutToConstPointer))) = *const {l1792} i8
            // 2399: tok:   l1792 = UNIQUE | NON_NULL, (empty)
            // 2399: b"incvar\0" as  ... st u8: typeof(_687 = move _688 as *const u8 (Pointer(ArrayToPointer))) = *const {l1795} u8
            // 2399: b"incvar\0" as  ... st u8:   l1795 = UNIQUE | NON_NULL, (empty)
                noarg(INCVAR);
            } else if strcmp(tok, b"maxvar\0" as *const u8 as *const libc::c_char) == 0 {
            // 2401: tok: typeof(_693) = *const {l1153} i8
            // 2401: tok:   l1153 = UNIQUE | NON_NULL, (empty)
            // 2401: tok: typeof(_694) = *mut {l1155} i8
            // 2401: tok:   l1155 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0" as  ... _char: typeof(_695) = *const {l1157} i8
            // 2401: b"maxvar\0" as  ... _char:   l1157 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0" as  ... st u8: typeof(_696) = *const {l1159} u8
            // 2401: b"maxvar\0" as  ... st u8:   l1159 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0": typeof(_697) = *const {l1161} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2401: b"maxvar\0":   l1161 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0": typeof(_698) = & {l1163} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2401: b"maxvar\0":   l1163 = UNIQUE | NON_NULL, FIXED
            // 2401: b"maxvar\0" as  ... _char: typeof(_695 = move _696 as *const i8 (Misc)) = *const {l1801} i8
            // 2401: b"maxvar\0" as  ... _char:   l1801 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0": typeof(_698 = const b"maxvar\x00") = & {l1798} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2401: b"maxvar\0":   l1798 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0": typeof(_697 = &raw const (*_698)) = *const {l1799} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2401: b"maxvar\0":   l1799 = UNIQUE | NON_NULL, (empty)
            // 2401: b"maxvar\0" as  ... st u8: typeof(_696 = move _697 as *const u8 (Pointer(ArrayToPointer))) = *const {l1800} u8
            // 2401: b"maxvar\0" as  ... st u8:   l1800 = UNIQUE | NON_NULL, (empty)
            // 2401: tok: typeof(_693 = move _694 as *const i8 (Pointer(MutToConstPointer))) = *const {l1797} i8
            // 2401: tok:   l1797 = UNIQUE | NON_NULL, (empty)
                noarg(MAXVAR);
            } else if strcmp(tok, b"inconsistent\0" as *const u8 as *const libc::c_char) == 0 {
            // 2403: tok: typeof(_702) = *const {l1168} i8
            // 2403: tok:   l1168 = UNIQUE | NON_NULL, (empty)
            // 2403: tok: typeof(_703) = *mut {l1170} i8
            // 2403: tok:   l1170 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\ ... _char: typeof(_704) = *const {l1172} i8
            // 2403: b"inconsistent\ ... _char:   l1172 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\ ... st u8: typeof(_705) = *const {l1174} u8
            // 2403: b"inconsistent\ ... st u8:   l1174 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\0": typeof(_706) = *const {l1176} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2403: b"inconsistent\0":   l1176 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\0": typeof(_707) = & {l1178} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2403: b"inconsistent\0":   l1178 = UNIQUE | NON_NULL, FIXED
            // 2403: b"inconsistent\ ... st u8: typeof(_705 = move _706 as *const u8 (Pointer(ArrayToPointer))) = *const {l1805} u8
            // 2403: b"inconsistent\ ... st u8:   l1805 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\0": typeof(_706 = &raw const (*_707)) = *const {l1804} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2403: b"inconsistent\0":   l1804 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\0": typeof(_707 = const b"inconsistent\x00") = & {l1803} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2403: b"inconsistent\0":   l1803 = UNIQUE | NON_NULL, (empty)
            // 2403: tok: typeof(_702 = move _703 as *const i8 (Pointer(MutToConstPointer))) = *const {l1802} i8
            // 2403: tok:   l1802 = UNIQUE | NON_NULL, (empty)
            // 2403: b"inconsistent\ ... _char: typeof(_704 = move _705 as *const i8 (Misc)) = *const {l1806} i8
            // 2403: b"inconsistent\ ... _char:   l1806 = UNIQUE | NON_NULL, (empty)
                noarg(INCONSISTENT);
            } else if strcmp(tok, b"lkhd\0" as *const u8 as *const libc::c_char) == 0 {
            // 2405: tok: typeof(_711) = *const {l1183} i8
            // 2405: tok:   l1183 = UNIQUE | NON_NULL, (empty)
            // 2405: tok: typeof(_712) = *mut {l1185} i8
            // 2405: tok:   l1185 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0" as *c ... _char: typeof(_713) = *const {l1187} i8
            // 2405: b"lkhd\0" as *c ... _char:   l1187 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0" as *c ... st u8: typeof(_714) = *const {l1189} u8
            // 2405: b"lkhd\0" as *c ... st u8:   l1189 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0": typeof(_715) = *const {l1191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2405: b"lkhd\0":   l1191 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0": typeof(_716) = & {l1193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2405: b"lkhd\0":   l1193 = UNIQUE | NON_NULL, FIXED
            // 2405: tok: typeof(_711 = move _712 as *const i8 (Pointer(MutToConstPointer))) = *const {l1807} i8
            // 2405: tok:   l1807 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0": typeof(_715 = &raw const (*_716)) = *const {l1809} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2405: b"lkhd\0":   l1809 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0" as *c ... st u8: typeof(_714 = move _715 as *const u8 (Pointer(ArrayToPointer))) = *const {l1810} u8
            // 2405: b"lkhd\0" as *c ... st u8:   l1810 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0": typeof(_716 = const b"lkhd\x00") = & {l1808} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 2405: b"lkhd\0":   l1808 = UNIQUE | NON_NULL, (empty)
            // 2405: b"lkhd\0" as *c ... _char: typeof(_713 = move _714 as *const i8 (Misc)) = *const {l1811} i8
            // 2405: b"lkhd\0" as *c ... _char:   l1811 = UNIQUE | NON_NULL, (empty)
                noarg(LKHD);
            } else if strcmp(tok, b"fixate\0" as *const u8 as *const libc::c_char) == 0 {
            // 2407: tok: typeof(_720) = *const {l1198} i8
            // 2407: tok:   l1198 = UNIQUE | NON_NULL, (empty)
            // 2407: tok: typeof(_721) = *mut {l1200} i8
            // 2407: tok:   l1200 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0" as  ... _char: typeof(_722) = *const {l1202} i8
            // 2407: b"fixate\0" as  ... _char:   l1202 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0" as  ... st u8: typeof(_723) = *const {l1204} u8
            // 2407: b"fixate\0" as  ... st u8:   l1204 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0": typeof(_724) = *const {l1206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2407: b"fixate\0":   l1206 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0": typeof(_725) = & {l1208} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2407: b"fixate\0":   l1208 = UNIQUE | NON_NULL, FIXED
            // 2407: b"fixate\0": typeof(_725 = const b"fixate\x00") = & {l1813} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2407: b"fixate\0":   l1813 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0" as  ... _char: typeof(_722 = move _723 as *const i8 (Misc)) = *const {l1816} i8
            // 2407: b"fixate\0" as  ... _char:   l1816 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0" as  ... st u8: typeof(_723 = move _724 as *const u8 (Pointer(ArrayToPointer))) = *const {l1815} u8
            // 2407: b"fixate\0" as  ... st u8:   l1815 = UNIQUE | NON_NULL, (empty)
            // 2407: b"fixate\0": typeof(_724 = &raw const (*_725)) = *const {l1814} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2407: b"fixate\0":   l1814 = UNIQUE | NON_NULL, (empty)
            // 2407: tok: typeof(_720 = move _721 as *const i8 (Pointer(MutToConstPointer))) = *const {l1812} i8
            // 2407: tok:   l1812 = UNIQUE | NON_NULL, (empty)
                noarg(FIXATE);
            } else if strcmp(tok, b"reduce\0" as *const u8 as *const libc::c_char) == 0 {
            // 2409: tok: typeof(_729) = *const {l1213} i8
            // 2409: tok:   l1213 = UNIQUE | NON_NULL, (empty)
            // 2409: tok: typeof(_730) = *mut {l1215} i8
            // 2409: tok:   l1215 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0" as  ... _char: typeof(_731) = *const {l1217} i8
            // 2409: b"reduce\0" as  ... _char:   l1217 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0" as  ... st u8: typeof(_732) = *const {l1219} u8
            // 2409: b"reduce\0" as  ... st u8:   l1219 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0": typeof(_733) = *const {l1221} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2409: b"reduce\0":   l1221 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0": typeof(_734) = & {l1223} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2409: b"reduce\0":   l1223 = UNIQUE | NON_NULL, FIXED
            // 2409: b"reduce\0": typeof(_733 = &raw const (*_734)) = *const {l1819} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2409: b"reduce\0":   l1819 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0" as  ... _char: typeof(_731 = move _732 as *const i8 (Misc)) = *const {l1821} i8
            // 2409: b"reduce\0" as  ... _char:   l1821 = UNIQUE | NON_NULL, (empty)
            // 2409: tok: typeof(_729 = move _730 as *const i8 (Pointer(MutToConstPointer))) = *const {l1817} i8
            // 2409: tok:   l1817 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0": typeof(_734 = const b"reduce\x00") = & {l1818} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 2409: b"reduce\0":   l1818 = UNIQUE | NON_NULL, (empty)
            // 2409: b"reduce\0" as  ... st u8: typeof(_732 = move _733 as *const u8 (Pointer(ArrayToPointer))) = *const {l1820} u8
            // 2409: b"reduce\0" as  ... st u8:   l1820 = UNIQUE | NON_NULL, (empty)
                noarg(REDUCE);
            } else if strcmp(tok, b"flush\0" as *const u8 as *const libc::c_char) == 0 {
            // 2411: tok: typeof(_738) = *const {l1228} i8
            // 2411: tok:   l1228 = UNIQUE | NON_NULL, (empty)
            // 2411: tok: typeof(_739) = *mut {l1230} i8
            // 2411: tok:   l1230 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0" as * ... _char: typeof(_740) = *const {l1232} i8
            // 2411: b"flush\0" as * ... _char:   l1232 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0" as * ... st u8: typeof(_741) = *const {l1234} u8
            // 2411: b"flush\0" as * ... st u8:   l1234 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0": typeof(_742) = *const {l1236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2411: b"flush\0":   l1236 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0": typeof(_743) = & {l1238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2411: b"flush\0":   l1238 = UNIQUE | NON_NULL, FIXED
            // 2411: b"flush\0": typeof(_742 = &raw const (*_743)) = *const {l1824} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2411: b"flush\0":   l1824 = UNIQUE | NON_NULL, (empty)
            // 2411: tok: typeof(_738 = move _739 as *const i8 (Pointer(MutToConstPointer))) = *const {l1822} i8
            // 2411: tok:   l1822 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0": typeof(_743 = const b"flush\x00") = & {l1823} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2411: b"flush\0":   l1823 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0" as * ... st u8: typeof(_741 = move _742 as *const u8 (Pointer(ArrayToPointer))) = *const {l1825} u8
            // 2411: b"flush\0" as * ... st u8:   l1825 = UNIQUE | NON_NULL, (empty)
            // 2411: b"flush\0" as * ... _char: typeof(_740 = move _741 as *const i8 (Misc)) = *const {l1826} i8
            // 2411: b"flush\0" as * ... _char:   l1826 = UNIQUE | NON_NULL, (empty)
                noarg(FLUSH);
            } else if strcmp(tok, b"chkclone\0" as *const u8 as *const libc::c_char) == 0 {
            // 2413: tok: typeof(_747) = *const {l1243} i8
            // 2413: tok:   l1243 = UNIQUE | NON_NULL, (empty)
            // 2413: tok: typeof(_748) = *mut {l1245} i8
            // 2413: tok:   l1245 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0" a ... _char: typeof(_749) = *const {l1247} i8
            // 2413: b"chkclone\0" a ... _char:   l1247 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0" a ... st u8: typeof(_750) = *const {l1249} u8
            // 2413: b"chkclone\0" a ... st u8:   l1249 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0": typeof(_751) = *const {l1251} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2413: b"chkclone\0":   l1251 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0": typeof(_752) = & {l1253} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2413: b"chkclone\0":   l1253 = UNIQUE | NON_NULL, FIXED
            // 2413: b"chkclone\0" a ... st u8: typeof(_750 = move _751 as *const u8 (Pointer(ArrayToPointer))) = *const {l1830} u8
            // 2413: b"chkclone\0" a ... st u8:   l1830 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0": typeof(_752 = const b"chkclone\x00") = & {l1828} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2413: b"chkclone\0":   l1828 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0": typeof(_751 = &raw const (*_752)) = *const {l1829} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2413: b"chkclone\0":   l1829 = UNIQUE | NON_NULL, (empty)
            // 2413: tok: typeof(_747 = move _748 as *const i8 (Pointer(MutToConstPointer))) = *const {l1827} i8
            // 2413: tok:   l1827 = UNIQUE | NON_NULL, (empty)
            // 2413: b"chkclone\0" a ... _char: typeof(_749 = move _750 as *const i8 (Misc)) = *const {l1831} i8
            // 2413: b"chkclone\0" a ... _char:   l1831 = UNIQUE | NON_NULL, (empty)
                noarg(CHKCLONE);
            } else if strcmp(tok, b"changed\0" as *const u8 as *const libc::c_char) == 0 {
            // 2415: tok: typeof(_756) = *const {l1258} i8
            // 2415: tok:   l1258 = UNIQUE | NON_NULL, (empty)
            // 2415: tok: typeof(_757) = *mut {l1260} i8
            // 2415: tok:   l1260 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0" as ... _char: typeof(_758) = *const {l1262} i8
            // 2415: b"changed\0" as ... _char:   l1262 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0" as ... st u8: typeof(_759) = *const {l1264} u8
            // 2415: b"changed\0" as ... st u8:   l1264 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0": typeof(_760) = *const {l1266} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2415: b"changed\0":   l1266 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0": typeof(_761) = & {l1268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2415: b"changed\0":   l1268 = UNIQUE | NON_NULL, FIXED
            // 2415: b"changed\0" as ... st u8: typeof(_759 = move _760 as *const u8 (Pointer(ArrayToPointer))) = *const {l1835} u8
            // 2415: b"changed\0" as ... st u8:   l1835 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0": typeof(_760 = &raw const (*_761)) = *const {l1834} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2415: b"changed\0":   l1834 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0" as ... _char: typeof(_758 = move _759 as *const i8 (Misc)) = *const {l1836} i8
            // 2415: b"changed\0" as ... _char:   l1836 = UNIQUE | NON_NULL, (empty)
            // 2415: b"changed\0": typeof(_761 = const b"changed\x00") = & {l1833} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2415: b"changed\0":   l1833 = UNIQUE | NON_NULL, (empty)
            // 2415: tok: typeof(_756 = move _757 as *const i8 (Pointer(MutToConstPointer))) = *const {l1832} i8
            // 2415: tok:   l1832 = UNIQUE | NON_NULL, (empty)
                noarg(CHANGED);
            } else {
                perr(
                    b"invalid command '%s'\0" as *const u8 as *const libc::c_char,
                    // 2419: b"invalid comma ... _char: typeof(_764) = *const {l1272} i8
                    // 2419: b"invalid comma ... _char:   l1272 = UNIQUE | NON_NULL, (empty)
                    // 2419: b"invalid comma ... st u8: typeof(_765) = *const {l1274} u8
                    // 2419: b"invalid comma ... st u8:   l1274 = UNIQUE | NON_NULL, (empty)
                    // 2419: b"invalid comma ... s'\0": typeof(_766) = *const {l1276} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2419: b"invalid comma ... s'\0":   l1276 = UNIQUE | NON_NULL, (empty)
                    // 2419: b"invalid comma ... s'\0": typeof(_767) = & {l1278} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2419: b"invalid comma ... s'\0":   l1278 = UNIQUE | NON_NULL, FIXED
                    // 2419: b"invalid comma ... s'\0": typeof(_767 = const b"invalid command \'%s\'\x00") = & {l1837} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2419: b"invalid comma ... s'\0":   l1837 = UNIQUE | NON_NULL, (empty)
                    // 2419: b"invalid comma ... s'\0": typeof(_766 = &raw const (*_767)) = *const {l1838} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 2419: b"invalid comma ... s'\0":   l1838 = UNIQUE | NON_NULL, (empty)
                    // 2419: b"invalid comma ... _char: typeof(_764 = move _765 as *const i8 (Misc)) = *const {l1840} i8
                    // 2419: b"invalid comma ... _char:   l1840 = UNIQUE | NON_NULL, (empty)
                    // 2419: b"invalid comma ... st u8: typeof(_765 = move _766 as *const u8 (Pointer(ArrayToPointer))) = *const {l1839} u8
                    // 2419: b"invalid comma ... st u8:   l1839 = UNIQUE | NON_NULL, (empty)
                    tok,
                    // 2420: tok: typeof(_768) = *mut {l1280} i8
                    // 2420: tok:   l1280 = UNIQUE | NON_NULL, (empty)
                );
            }
            lineno += 1;
            // 2423: lineno: typeof(_769) = *mut {l1282} i32
            // 2423: lineno:   l1282 = UNIQUE | NON_NULL, (empty)
            lineno;
            // 2424: lineno: typeof(_772) = *mut {l1286} i32
            // 2424: lineno:   l1286 = UNIQUE | NON_NULL, (empty)
            count += 1;
            count;
            len = 0 as libc::c_int;
        }
    }
    if close_0 == 1 as libc::c_int {
        fclose(file);
        // 2431: file: typeof(_781) = *mut {l1296} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2431: file:   l1296 = UNIQUE | NON_NULL, (empty)
    }
    if close_0 == 2 as libc::c_int {
        pclose(file);
        // 2434: file: typeof(_787) = *mut {l1303} DefId(0:305 ~ lglddtrace[7e63]::_IO_FILE)
        // 2434: file:   l1303 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        b"parsed %d events in %s\0" as *const u8 as *const libc::c_char,
        // 2437: b"parsed %d eve ... _char: typeof(_789) = *const {l1306} i8
        // 2437: b"parsed %d eve ... _char:   l1306 = UNIQUE | NON_NULL, (empty)
        // 2437: b"parsed %d eve ... st u8: typeof(_790) = *const {l1308} u8
        // 2437: b"parsed %d eve ... st u8:   l1308 = UNIQUE | NON_NULL, (empty)
        // 2437: b"parsed %d eve ... %s\0": typeof(_791) = *const {l1310} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
        // 2437: b"parsed %d eve ... %s\0":   l1310 = UNIQUE | NON_NULL, (empty)
        // 2437: b"parsed %d eve ... %s\0": typeof(_792) = & {l1312} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
        // 2437: b"parsed %d eve ... %s\0":   l1312 = UNIQUE | NON_NULL, FIXED
        // 2437: b"parsed %d eve ... st u8: typeof(_790 = move _791 as *const u8 (Pointer(ArrayToPointer))) = *const {l1843} u8
        // 2437: b"parsed %d eve ... st u8:   l1843 = UNIQUE | NON_NULL, (empty)
        // 2437: b"parsed %d eve ... %s\0": typeof(_792 = const b"parsed %d events in %s\x00") = & {l1841} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
        // 2437: b"parsed %d eve ... %s\0":   l1841 = UNIQUE | NON_NULL, (empty)
        // 2437: b"parsed %d eve ... %s\0": typeof(_791 = &raw const (*_792)) = *const {l1842} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
        // 2437: b"parsed %d eve ... %s\0":   l1842 = UNIQUE | NON_NULL, (empty)
        // 2437: b"parsed %d eve ... _char: typeof(_789 = move _790 as *const i8 (Misc)) = *const {l1844} i8
        // 2437: b"parsed %d eve ... _char:   l1844 = UNIQUE | NON_NULL, (empty)
        count,
        iname,
        // 2439: iname: typeof(_794) = *const {l1315} i8
        // 2439: iname:   l1315 = UNIQUE | NON_NULL, (empty)
        // 2439: iname: typeof(_795) = *mut {l1317} *const {l1318} i8
        // 2439: iname:   l1317 = UNIQUE | NON_NULL, (empty)
        // 2439: iname:   l1318 = UNIQUE | NON_NULL, (empty)
    );
    golden = run();
    // 2441: golden: typeof(_797) = *mut {l1321} i32
    // 2441: golden:   l1321 = UNIQUE | NON_NULL, (empty)
    delta = getime() - start;
    timelimit = delta as libc::c_int;
    // 2443: timelimit: typeof(_801) = *mut {l1326} i32
    // 2443: timelimit:   l1326 = UNIQUE | NON_NULL, (empty)
    if delta < 0 as libc::c_int as libc::c_double {
        delta = 0 as libc::c_int as libc::c_double;
    }
    msg(
        b"golden exit code %d in %.3f seconds\0" as *const u8 as *const libc::c_char,
        // 2448: b"golden exit c ... _char: typeof(_809) = *const {l1335} i8
        // 2448: b"golden exit c ... _char:   l1335 = UNIQUE | NON_NULL, (empty)
        // 2448: b"golden exit c ... st u8: typeof(_810) = *const {l1337} u8
        // 2448: b"golden exit c ... st u8:   l1337 = UNIQUE | NON_NULL, (empty)
        // 2448: b"golden exit c ... ds\0": typeof(_811) = *const {l1339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 2448: b"golden exit c ... ds\0":   l1339 = UNIQUE | NON_NULL, (empty)
        // 2448: b"golden exit c ... ds\0": typeof(_812) = & {l1341} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 2448: b"golden exit c ... ds\0":   l1341 = UNIQUE | NON_NULL, FIXED
        // 2448: b"golden exit c ... st u8: typeof(_810 = move _811 as *const u8 (Pointer(ArrayToPointer))) = *const {l1847} u8
        // 2448: b"golden exit c ... st u8:   l1847 = UNIQUE | NON_NULL, (empty)
        // 2448: b"golden exit c ... ds\0": typeof(_811 = &raw const (*_812)) = *const {l1846} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 2448: b"golden exit c ... ds\0":   l1846 = UNIQUE | NON_NULL, (empty)
        // 2448: b"golden exit c ... _char: typeof(_809 = move _810 as *const i8 (Misc)) = *const {l1848} i8
        // 2448: b"golden exit c ... _char:   l1848 = UNIQUE | NON_NULL, (empty)
        // 2448: b"golden exit c ... ds\0": typeof(_812 = const b"golden exit code %d in %.3f seconds\x00") = & {l1845} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
        // 2448: b"golden exit c ... ds\0":   l1845 = UNIQUE | NON_NULL, (empty)
        golden,
        // 2449: golden: typeof(_814) = *mut {l1344} i32
        // 2449: golden:   l1344 = UNIQUE | NON_NULL, (empty)
        delta,
    );
    timelimit = delta as libc::c_int;
    // 2452: timelimit: typeof(_817) = *mut {l1348} i32
    // 2452: timelimit:   l1348 = UNIQUE | NON_NULL, (empty)
    if timelimit <= 0 as libc::c_int {
    // 2453: timelimit: typeof(_821) = *mut {l1353} i32
    // 2453: timelimit:   l1353 = UNIQUE | NON_NULL, (empty)
        timelimit = 1 as libc::c_int;
        // 2454: timelimit: typeof(_824) = *mut {l1357} i32
        // 2454: timelimit:   l1357 = UNIQUE | NON_NULL, (empty)
    }
    timelimit *= 100 as libc::c_int;
    // 2456: timelimit: typeof(_826) = *mut {l1360} i32
    // 2456: timelimit:   l1360 = UNIQUE | NON_NULL, (empty)
    msg(
        b"time limit %d seconds\0" as *const u8 as *const libc::c_char,
        // 2458: b"time limit %d ... _char: typeof(_829) = *const {l1364} i8
        // 2458: b"time limit %d ... _char:   l1364 = UNIQUE | NON_NULL, (empty)
        // 2458: b"time limit %d ... st u8: typeof(_830) = *const {l1366} u8
        // 2458: b"time limit %d ... st u8:   l1366 = UNIQUE | NON_NULL, (empty)
        // 2458: b"time limit %d ... ds\0": typeof(_831) = *const {l1368} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 2458: b"time limit %d ... ds\0":   l1368 = UNIQUE | NON_NULL, (empty)
        // 2458: b"time limit %d ... ds\0": typeof(_832) = & {l1370} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 2458: b"time limit %d ... ds\0":   l1370 = UNIQUE | NON_NULL, FIXED
        // 2458: b"time limit %d ... st u8: typeof(_830 = move _831 as *const u8 (Pointer(ArrayToPointer))) = *const {l1851} u8
        // 2458: b"time limit %d ... st u8:   l1851 = UNIQUE | NON_NULL, (empty)
        // 2458: b"time limit %d ... _char: typeof(_829 = move _830 as *const i8 (Misc)) = *const {l1852} i8
        // 2458: b"time limit %d ... _char:   l1852 = UNIQUE | NON_NULL, (empty)
        // 2458: b"time limit %d ... ds\0": typeof(_831 = &raw const (*_832)) = *const {l1850} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 2458: b"time limit %d ... ds\0":   l1850 = UNIQUE | NON_NULL, (empty)
        // 2458: b"time limit %d ... ds\0": typeof(_832 = const b"time limit %d seconds\x00") = & {l1849} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 2458: b"time limit %d ... ds\0":   l1849 = UNIQUE | NON_NULL, (empty)
        timelimit,
        // 2459: timelimit: typeof(_834) = *mut {l1373} i32
        // 2459: timelimit:   l1373 = UNIQUE | NON_NULL, (empty)
    );
    dd();
    prt(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < nevents {
    // 2464: nevents: typeof(_843) = *mut {l1383} i32
    // 2464: nevents:   l1383 = UNIQUE | NON_NULL, (empty)
        free((*events.offset(i as isize)).opt as *mut libc::c_void);
        // 2465: (*events.offset ... _void: typeof(_845) = *mut {l1386} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2465: (*events.offset ... _void:   l1386 = UNIQUE | NON_NULL, (empty)
        // 2465: (*events.offset ... ).opt: typeof(_846) = *mut {l1388} i8
        // 2465: (*events.offset ... ).opt:   l1388 = UNIQUE | NON_NULL, (empty)
        // 2465: events.offset(i ... size): typeof(_847) = *mut {l1390} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 2465: events.offset(i ... size):   l1390 = UNIQUE | NON_NULL, (empty)
        // 2465: events: typeof(_848) = *mut {l1392} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 2465: events:   l1392 = UNIQUE | NON_NULL, (empty)
        // 2465: events: typeof(_849) = *mut {l1394} *mut {l1395} DefId(0:354 ~ lglddtrace[7e63]::Event)
        // 2465: events:   l1394 = UNIQUE | NON_NULL, (empty)
        // 2465: events:   l1395 = UNIQUE | NON_NULL, (empty)
        // 2465: (*events.offset ... _void: typeof(_845 = move _846 as *mut libc::c_void (Misc)) = *mut {l1853} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2465: (*events.offset ... _void:   l1853 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    free(events as *mut libc::c_void);
    // 2469: events as *mut  ... _void: typeof(_858) = *mut {l1405} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2469: events as *mut  ... _void:   l1405 = UNIQUE | NON_NULL, (empty)
    // 2469: events: typeof(_859) = *mut {l1407} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 2469: events:   l1407 = UNIQUE | NON_NULL, (empty)
    // 2469: events: typeof(_860) = *mut {l1409} *mut {l1410} DefId(0:354 ~ lglddtrace[7e63]::Event)
    // 2469: events:   l1409 = UNIQUE | NON_NULL, (empty)
    // 2469: events:   l1410 = UNIQUE | NON_NULL, (empty)
    // 2469: events as *mut  ... _void: typeof(_858 = move _859 as *mut libc::c_void (Misc)) = *mut {l1854} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2469: events as *mut  ... _void:   l1854 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < nopts {
    // 2471: nopts: typeof(_866) = *mut {l1417} i32
    // 2471: nopts:   l1417 = UNIQUE | NON_NULL, (empty)
        free((*opts.offset(i as isize)).name as *mut libc::c_void);
        // 2472: (*opts.offset(i ... _void: typeof(_868) = *mut {l1420} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2472: (*opts.offset(i ... _void:   l1420 = UNIQUE | NON_NULL, (empty)
        // 2472: (*opts.offset(i ... .name: typeof(_869) = *mut {l1422} i8
        // 2472: (*opts.offset(i ... .name:   l1422 = UNIQUE | NON_NULL, (empty)
        // 2472: opts.offset(i a ... size): typeof(_870) = *mut {l1424} DefId(0:362 ~ lglddtrace[7e63]::Opt)
        // 2472: opts.offset(i a ... size):   l1424 = UNIQUE | NON_NULL, (empty)
        // 2472: opts: typeof(_871) = *mut {l1426} DefId(0:362 ~ lglddtrace[7e63]::Opt)
        // 2472: opts:   l1426 = UNIQUE | NON_NULL, (empty)
        // 2472: opts: typeof(_872) = *mut {l1428} *mut {l1429} DefId(0:362 ~ lglddtrace[7e63]::Opt)
        // 2472: opts:   l1428 = UNIQUE | NON_NULL, (empty)
        // 2472: opts:   l1429 = UNIQUE | NON_NULL, (empty)
        // 2472: (*opts.offset(i ... _void: typeof(_868 = move _869 as *mut libc::c_void (Misc)) = *mut {l1855} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2472: (*opts.offset(i ... _void:   l1855 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    free(opts as *mut libc::c_void);
    // 2476: opts as *mut li ... _void: typeof(_881) = *mut {l1439} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2476: opts as *mut li ... _void:   l1439 = UNIQUE | NON_NULL, (empty)
    // 2476: opts: typeof(_882) = *mut {l1441} DefId(0:362 ~ lglddtrace[7e63]::Opt)
    // 2476: opts:   l1441 = UNIQUE | NON_NULL, (empty)
    // 2476: opts: typeof(_883) = *mut {l1443} *mut {l1444} DefId(0:362 ~ lglddtrace[7e63]::Opt)
    // 2476: opts:   l1443 = UNIQUE | NON_NULL, (empty)
    // 2476: opts:   l1444 = UNIQUE | NON_NULL, (empty)
    // 2476: opts as *mut li ... _void: typeof(_881 = move _882 as *mut libc::c_void (Misc)) = *mut {l1856} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2476: opts as *mut li ... _void:   l1856 = UNIQUE | NON_NULL, (empty)
    msg(
        b"executed %d runs in %.3f seconds\0" as *const u8 as *const libc::c_char,
        // 2478: b"executed %d r ... _char: typeof(_885) = *const {l1447} i8
        // 2478: b"executed %d r ... _char:   l1447 = UNIQUE | NON_NULL, (empty)
        // 2478: b"executed %d r ... st u8: typeof(_886) = *const {l1449} u8
        // 2478: b"executed %d r ... st u8:   l1449 = UNIQUE | NON_NULL, (empty)
        // 2478: b"executed %d r ... ds\0": typeof(_887) = *const {l1451} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2478: b"executed %d r ... ds\0":   l1451 = UNIQUE | NON_NULL, (empty)
        // 2478: b"executed %d r ... ds\0": typeof(_888) = & {l1453} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2478: b"executed %d r ... ds\0":   l1453 = UNIQUE | NON_NULL, FIXED
        // 2478: b"executed %d r ... ds\0": typeof(_887 = &raw const (*_888)) = *const {l1858} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2478: b"executed %d r ... ds\0":   l1858 = UNIQUE | NON_NULL, (empty)
        // 2478: b"executed %d r ... ds\0": typeof(_888 = const b"executed %d runs in %.3f seconds\x00") = & {l1857} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
        // 2478: b"executed %d r ... ds\0":   l1857 = UNIQUE | NON_NULL, (empty)
        // 2478: b"executed %d r ... _char: typeof(_885 = move _886 as *const i8 (Misc)) = *const {l1860} i8
        // 2478: b"executed %d r ... _char:   l1860 = UNIQUE | NON_NULL, (empty)
        // 2478: b"executed %d r ... st u8: typeof(_886 = move _887 as *const u8 (Pointer(ArrayToPointer))) = *const {l1859} u8
        // 2478: b"executed %d r ... st u8:   l1859 = UNIQUE | NON_NULL, (empty)
        runs,
        // 2479: runs: typeof(_890) = *mut {l1456} i32
        // 2479: runs:   l1456 = UNIQUE | NON_NULL, (empty)
        getime() - start,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    // 2485: mut args: typeof(_1) = DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l1} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2485: mut args:   l1 = UNIQUE | NON_NULL, (empty)
    for arg in ::std::env::args() {
    // 2486: ::std::env::args(): typeof(_9) = &mut {l10} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2486: ::std::env::args():   l10 = UNIQUE | NON_NULL, (empty)
    // 2486: ::std::env::args(): typeof(_10) = &mut {l12} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2486: ::std::env::args():   l12 = UNIQUE | NON_NULL, (empty)
    // 2486: ::std::env::args(): typeof(_10 = &mut _5) = &mut {l51} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2486: ::std::env::args():   l51 = UNIQUE | NON_NULL, (empty)
    // 2486: ::std::env::args(): typeof(_9 = &mut (*_10)) = &mut {l52} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2486: ::std::env::args():   l52 = UNIQUE | NON_NULL, (empty)
        args.push(
        // 2487: args.push( (::s ... (), ): typeof(_15) = &mut {l18} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l19} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 2487: args.push( (::s ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 2487: args.push( (::s ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 2487: args.push( (::s ... (), ): typeof(_15 = &mut _1) = &mut {l53} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l54} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 2487: args.push( (::s ... (), ):   l53 = UNIQUE | NON_NULL, (empty)
        // 2487: args.push( (::s ... (), ):   l54 = UNIQUE | NON_NULL, (empty)
            (::std::ffi::CString::new(arg))
            // 2488: (::std::ffi::CS ... raw(): typeof(_16) = *mut {l21} i8
            // 2488: (::std::ffi::CS ... raw():   l21 = UNIQUE | NON_NULL, (empty)
                .expect("Failed to convert argument into CString.")
                // 2489: "Failed to conv ... ing.": typeof(_20) = & {l26} str
                // 2489: "Failed to conv ... ing.":   l26 = UNIQUE | NON_NULL, (empty)
                // 2489: "Failed to conv ... ing.": typeof(_21) = & {l28} str
                // 2489: "Failed to conv ... ing.":   l28 = UNIQUE | NON_NULL, FIXED
                // 2489: "Failed to conv ... ing.": typeof(_21 = const "Failed to convert argument into CString.") = & {l55} str
                // 2489: "Failed to conv ... ing.":   l55 = UNIQUE | NON_NULL, (empty)
                // 2489: "Failed to conv ... ing.": typeof(_20 = &(*_21)) = & {l56} str
                // 2489: "Failed to conv ... ing.":   l56 = UNIQUE | NON_NULL, (empty)
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    // 2493: args.push(::cor ... ut()): typeof(_23) = &mut {l31} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l32} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2493: args.push(::cor ... ut()):   l31 = UNIQUE | NON_NULL, (empty)
    // 2493: args.push(::cor ... ut()):   l32 = UNIQUE | NON_NULL, (empty)
    // 2493: ::core::ptr::nu ... mut(): typeof(_24) = *mut {l34} i8
    // 2493: ::core::ptr::nu ... mut():   l34 = UNIQUE | NON_NULL, (empty)
    // 2493: args.push(::cor ... ut()): typeof(_23 = &mut _1) = &mut {l57} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l58} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2493: args.push(::cor ... ut()):   l57 = UNIQUE | NON_NULL, (empty)
    // 2493: args.push(::cor ... ut()):   l58 = UNIQUE | NON_NULL, (empty)
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            // 2496: args.len(): typeof(_30) = & {l41} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l42} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2496: args.len():   l41 = UNIQUE | NON_NULL, (empty)
            // 2496: args.len():   l42 = UNIQUE | NON_NULL, (empty)
            // 2496: args.len(): typeof(_30 = &_1) = & {l59} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l60} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2496: args.len():   l59 = UNIQUE | NON_NULL, (empty)
            // 2496: args.len():   l60 = UNIQUE | NON_NULL, (empty)
            args.as_mut_ptr() as *mut *mut libc::c_char,
            // 2497: args.as_mut_ptr ... _char: typeof(_32) = *mut {l45} *mut {l46} i8
            // 2497: args.as_mut_ptr ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 2497: args.as_mut_ptr ... _char:   l46 = UNIQUE | NON_NULL, (empty)
            // 2497: args.as_mut_ptr(): typeof(_33) = &mut {l48} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l49} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2497: args.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
            // 2497: args.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 2497: args.as_mut_ptr(): typeof(_33 = &mut _1) = &mut {l61} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l62} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2497: args.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
            // 2497: args.as_mut_ptr():   l62 = UNIQUE | NON_NULL, (empty)
        ) as i32)
    }
}
