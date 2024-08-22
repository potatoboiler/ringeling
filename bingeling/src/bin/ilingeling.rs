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
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stdout: *mut FILE;
    static mut stdin: *mut FILE;
    fn lglinit() -> *mut LGL;
    fn lglrelease(_: *mut LGL);
    fn lglclone(_: *mut LGL) -> *mut LGL;
    fn lglunclone(dst: *mut LGL, src: *mut LGL) -> libc::c_int;
    fn lglversion() -> *const libc::c_char;
    fn lglbnr(name: *const libc::c_char, prefix: *const libc::c_char, file: *mut FILE);
    fn lglsetout(_: *mut LGL, _: *mut FILE);
    fn lglsetprefix(_: *mut LGL, _: *const libc::c_char);
    fn lglsetopt(_: *mut LGL, _: *const libc::c_char, _: libc::c_int);
    fn lglsetid(_: *mut LGL, tid: libc::c_int, tids: libc::c_int);
    fn lgladd(_: *mut LGL, lit: libc::c_int);
    fn lglassume(_: *mut LGL, lit: libc::c_int);
    fn lglfixate(_: *mut LGL);
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglreducecache(_: *mut LGL);
    fn lglflushcache(_: *mut LGL);
    fn lglfreeze(_: *mut LGL, lit: libc::c_int);
    fn lglmelt(_: *mut LGL, lit: libc::c_int);
    fn lglmeltall(_: *mut LGL);
    fn lglflushtimers(lgl: *mut LGL);
    fn lglstats(_: *mut LGL);
    fn lglgetconfs(_: *mut LGL) -> int64_t;
    fn lglgetdecs(_: *mut LGL) -> int64_t;
    fn lglgetprops(_: *mut LGL) -> int64_t;
    fn lglfailed(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit: libc::c_int) -> libc::c_int;
    fn lglmb(_: *mut LGL) -> libc::c_double;
    fn lglprocesstime() -> libc::c_double;
    fn lglseterm(
        _: *mut LGL,
        term_0: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
        _: *mut libc::c_void,
    );
    fn lglsetmsglock(
        _: *mut LGL,
        lock: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        unlock: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        _: *mut libc::c_void,
    );
    fn lglsetime(_: *mut LGL, time: Option<unsafe extern "C" fn() -> libc::c_double>);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag<'h0,'h1> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h0 (libc::c_void),
    // 108: overflow_arg_area: typeof(overflow_arg_area) = *mut {g182} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 108: overflow_arg_area:   g182 = UNIQUE | NON_NULL, (empty)
    pub reg_save_area: &'h1 (libc::c_void),
    // 109: reg_save_area: typeof(reg_save_area) = *mut {g183} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 109: reg_save_area:   g183 = UNIQUE | NON_NULL, (empty)
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __int64_t = libc::c_long;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 123: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g184} i8
    // 123: _IO_read_ptr:   g184 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 124: _IO_read_end: typeof(_IO_read_end) = *mut {g185} i8
    // 124: _IO_read_end:   g185 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 125: _IO_read_base: typeof(_IO_read_base) = *mut {g186} i8
    // 125: _IO_read_base:   g186 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 126: _IO_write_base: typeof(_IO_write_base) = *mut {g187} i8
    // 126: _IO_write_base:   g187 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 127: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g188} i8
    // 127: _IO_write_ptr:   g188 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 128: _IO_write_end: typeof(_IO_write_end) = *mut {g189} i8
    // 128: _IO_write_end:   g189 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 129: _IO_buf_base: typeof(_IO_buf_base) = *mut {g190} i8
    // 129: _IO_buf_base:   g190 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 130: _IO_buf_end: typeof(_IO_buf_end) = *mut {g191} i8
    // 130: _IO_buf_end:   g191 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 131: _IO_save_base: typeof(_IO_save_base) = *mut {g192} i8
    // 131: _IO_save_base:   g192 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 132: _IO_backup_base: typeof(_IO_backup_base) = *mut {g193} i8
    // 132: _IO_backup_base:   g193 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 133: _IO_save_end: typeof(_IO_save_end) = *mut {g194} i8
    // 133: _IO_save_end:   g194 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 134: _markers: typeof(_markers) = *mut {g195} _IO_marker
    // 134: _markers:   g195 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 135: _chain: typeof(_chain) = *mut {g196} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 135: _chain:   g196 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 142: _lock: typeof(_lock) = *mut {g197} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 142: _lock:   g197 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 144: _codecvt: typeof(_codecvt) = *mut {g198} _IO_codecvt
    // 144: _codecvt:   g198 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 145: _wide_data: typeof(_wide_data) = *mut {g199} _IO_wide_data
    // 145: _wide_data:   g199 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 146: _freeres_list: typeof(_freeres_list) = *mut {g200} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 146: _freeres_list:   g200 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 147: _freeres_buf: typeof(_freeres_buf) = *mut {g201} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 147: _freeres_buf:   g201 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    // 165: __prev: typeof(__prev) = *mut {g202} DefId(0:257 ~ ilingeling[c969]::__pthread_internal_list)
    // 165: __prev:   g202 = UNIQUE | NON_NULL, FIXED
    pub __next: *mut __pthread_internal_list,
    // 166: __next: typeof(__next) = *mut {g203} DefId(0:257 ~ ilingeling[c969]::__pthread_internal_list)
    // 166: __next:   g203 = UNIQUE | NON_NULL, FIXED
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_0 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_0 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_0 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_0 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_0 = 0;
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Worker<'h2,'h3,'h4,'h5,'h6> {
    pub lgl: *mut LGL,
    // 229: lgl: typeof(lgl) = *mut {g204} LGL
    // 229: lgl:   g204 = UNIQUE | NON_NULL, FIXED
    pub cloned: C2RustUnnamed_1<'h6>,
    pub last: libc::c_int,
    pub res: libc::c_int,
    pub thread: pthread_t,
    pub proof: *mut FILE,
    // 234: proof: typeof(proof) = *mut {g205} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 234: proof:   g205 = UNIQUE | NON_NULL, FIXED
    pub post: *mut FILE,
    // 235: post: typeof(post) = *mut {g206} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 235: post:   g206 = UNIQUE | NON_NULL, FIXED
    pub failed: *mut libc::c_int,
    // 236: failed: typeof(failed) = *mut {g207} i32
    // 236: failed:   g207 = UNIQUE | NON_NULL, FIXED
    pub nfailed: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1<'h6> {
    pub lgl: *mut LGL,
    // 242: lgl: typeof(lgl) = *mut {g208} LGL
    // 242: lgl:   g208 = UNIQUE | NON_NULL, FIXED
    pub count: libc::c_int,
    pub bcount: libc::c_int,
    pub decs: int64_t,
    pub confs: int64_t,
    pub props: int64_t,
    pub lock: pthread_mutex_t,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
// 251: mut __nptr: typeof(_1) = *const {g0} i8
// 251: mut __nptr:   g0 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    return strtol(
        __nptr,
        // 253: __nptr: typeof(_4) = *const {l4} i8
        // 253: __nptr:   l4 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 254: 0 as *mut libc: ... _char: typeof(_5) = *mut {l6} *mut {g25} i8
        // 254: 0 as *mut libc: ... _char:   l6 = WRITE | UNIQUE, (empty)
        // 254: 0 as *mut libc: ... _char:   g25 = WRITE | OFFSET_ADD, CELL | FIXED
        // 254: 0 as *mut libc: ... _void: typeof(_6) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 254: 0 as *mut libc: ... _void:   l8 = WRITE | UNIQUE, (empty)
        // 254: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 254: 0 as *mut libc: ... _void:   l11 = WRITE | UNIQUE, (empty)
        // 254: 0 as *mut libc: ... _char: typeof(_5 = move _6 as *mut *mut i8 (Misc)) = *mut {l12} *mut {g25} i8
        // 254: 0 as *mut libc: ... _char:   l12 = WRITE | UNIQUE, (empty)
        // 254: 0 as *mut libc: ... _char:   g25 = WRITE | OFFSET_ADD, CELL | FIXED
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const libc::c_char,
    // 260: mut __fmt: typeof(_1) = *const {g1} i8
    // 260: mut __fmt:   g1 = UNIQUE | NON_NULL, FIXED
    mut __arg: ::core::ffi::VaList,
) -> libc::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
    // 263: stdout: typeof(_4) = *mut {l4} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 263: stdout:   l4 = UNIQUE | NON_NULL, (empty)
    // 263: stdout: typeof(_5) = *mut {l6} *mut {l7} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
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
static startime: libc::c_double = 0.;
static allocated: size_t = 0;
static maxallocated: size_t = 0;
static statsfile: *mut FILE = 0 as *const FILE as *mut FILE;
static histfile: *mut FILE = 0 as *const FILE as *mut FILE;
static workers: *mut Worker = 0 as *const Worker as *mut Worker;
static nworkers: libc::c_int = 0;
static nassumptions: libc::c_int = 0;
static queue: libc::c_int = 0;
static szassumptions: libc::c_int = 0;
static maxassumptionsize: libc::c_int = 0;
static sumassumptions: libc::c_int = 0;
static redassumptions: libc::c_int = 0;
static times: *mut libc::c_double = 0 as *const libc::c_double as *mut libc::c_double;
static sumtimes: libc::c_double = 0.;
static assumptions: *mut *mut libc::c_int =
    0 as *const *mut libc::c_int as *mut *mut libc::c_int;
static nvars: libc::c_int = 0;
static szvars: libc::c_int = 0;
static nclauses: libc::c_int = 0;
static nused: libc::c_int = 0;
static used: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static vals: *mut libc::c_schar = 0 as *const libc::c_schar as *mut libc::c_schar;
static nlits: libc::c_int = 0;
static szlits: libc::c_int = 0;
static lits: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static verbose: libc::c_int = 0;
static bar: libc::c_int = 0;
static nowitness: libc::c_int = 0;
static plain: libc::c_int = 0;
static doclone: libc::c_int = 0;
static deterministic: libc::c_int = 0;
static noreverse: libc::c_int = 0;
static addassumptions: libc::c_int = 1 as libc::c_int;
static noflush: libc::c_int = 0;
static reduce: libc::c_int = 0;
static nomelt: libc::c_int = 0;
static druptraceprefix: *const libc::c_char = 0 as *const libc::c_char;
static lineno: libc::c_int = 1 as libc::c_int;
static inputname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static inputfile: *mut FILE = 0 as *const FILE as *mut FILE;
static done: libc::c_int = 0;
static msgmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static donemutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static queuemutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static finishedmutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int as libc::c_short,
            __elision: 0 as libc::c_int as libc::c_short,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static finished: libc::c_int = 0;
unsafe extern "C" fn msglock(mut voidptr: *mut libc::c_void) {
// 392: mut voidptr: typeof(_1) = *mut {g2} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 392: mut voidptr:   g2 = UNIQUE | NON_NULL, FIXED
    pthread_mutex_lock(&mut msgmutex);
    // 393: &mut msgmutex: typeof(_3) = *mut {l3} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 393: &mut msgmutex:   l3 = UNIQUE | NON_NULL, (empty)
    // 393: &mut msgmutex: typeof(_4) = &mut {l5} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 393: &mut msgmutex:   l5 = UNIQUE | NON_NULL, (empty)
    // 393: msgmutex: typeof(_5) = *mut {l7} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 393: msgmutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 393: &mut msgmutex: typeof(_4 = &mut (*_5)) = &mut {l9} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 393: &mut msgmutex:   l9 = UNIQUE | NON_NULL, (empty)
    // 393: &mut msgmutex: typeof(_3 = &raw mut (*_4)) = *mut {l10} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 393: &mut msgmutex:   l10 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn msgunlock(mut voidptr: *mut libc::c_void) {
// 395: mut voidptr: typeof(_1) = *mut {g3} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 395: mut voidptr:   g3 = UNIQUE | NON_NULL, FIXED
    pthread_mutex_unlock(&mut msgmutex);
    // 396: &mut msgmutex: typeof(_3) = *mut {l3} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 396: &mut msgmutex:   l3 = UNIQUE | NON_NULL, (empty)
    // 396: &mut msgmutex: typeof(_4) = &mut {l5} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 396: &mut msgmutex:   l5 = UNIQUE | NON_NULL, (empty)
    // 396: msgmutex: typeof(_5) = *mut {l7} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 396: msgmutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 396: &mut msgmutex: typeof(_3 = &raw mut (*_4)) = *mut {l10} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 396: &mut msgmutex:   l10 = UNIQUE | NON_NULL, (empty)
    // 396: &mut msgmutex: typeof(_4 = &mut (*_5)) = &mut {l9} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 396: &mut msgmutex:   l9 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn msg(
    mut w: *mut Worker,
    // 399: mut w: typeof(_1) = *mut {g4} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 399: mut w:   g4 = UNIQUE | NON_NULL, FIXED
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    // 401: mut fmt: typeof(_3) = *const {g5} i8
    // 401: mut fmt:   g5 = UNIQUE | NON_NULL, FIXED
    mut args: ...
) {
    let mut ap: ::core::ffi::VaListImpl;
    if verbose < level {
    // 405: verbose: typeof(_9) = *mut {l9} i32
    // 405: verbose:   l9 = UNIQUE | NON_NULL, (empty)
        return;
    }
    msglock(0 as *mut libc::c_void);
    // 408: 0 as *mut libc: ... _void: typeof(_13) = *mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 408: 0 as *mut libc: ... _void:   l14 = UNIQUE | NON_NULL, (empty)
    // 408: 0 as *mut libc: ... _void: typeof(_13 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l76} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 408: 0 as *mut libc: ... _void:   l76 = UNIQUE | NON_NULL, (empty)
    if !w.is_null() {
    // 409: w: typeof(_17) = *mut {l19} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 409: w:   l19 = UNIQUE | NON_NULL, (empty)
        printf(
            b"c %d \0" as *const u8 as *const libc::c_char,
            // 411: b"c %d \0" as * ... _char: typeof(_19) = *const {l22} i8
            // 411: b"c %d \0" as * ... _char:   l22 = UNIQUE | NON_NULL, (empty)
            // 411: b"c %d \0" as * ... st u8: typeof(_20) = *const {l24} u8
            // 411: b"c %d \0" as * ... st u8:   l24 = UNIQUE | NON_NULL, (empty)
            // 411: b"c %d \0": typeof(_21) = *const {l26} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 411: b"c %d \0":   l26 = UNIQUE | NON_NULL, (empty)
            // 411: b"c %d \0": typeof(_22) = & {l28} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 411: b"c %d \0":   l28 = UNIQUE | NON_NULL, FIXED
            // 411: b"c %d \0": typeof(_22 = const b"c %d \x00") = & {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 411: b"c %d \0":   l77 = UNIQUE | NON_NULL, (empty)
            // 411: b"c %d \0": typeof(_21 = &raw const (*_22)) = *const {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 411: b"c %d \0":   l78 = UNIQUE | NON_NULL, (empty)
            // 411: b"c %d \0" as * ... _char: typeof(_19 = move _20 as *const i8 (Misc)) = *const {l80} i8
            // 411: b"c %d \0" as * ... _char:   l80 = UNIQUE | NON_NULL, (empty)
            // 411: b"c %d \0" as * ... st u8: typeof(_20 = move _21 as *const u8 (Pointer(ArrayToPointer))) = *const {l79} u8
            // 411: b"c %d \0" as * ... st u8:   l79 = UNIQUE | NON_NULL, (empty)
            w.offset_from(workers) as libc::c_long as libc::c_int,
            // 412: w: typeof(_26) = *mut {l33} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 412: w:   l33 = UNIQUE | NON_NULL, (empty)
            // 412: workers: typeof(_27) = *const {l35} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 412: workers:   l35 = UNIQUE | NON_NULL, (empty)
            // 412: workers: typeof(_28) = *mut {l37} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 412: workers:   l37 = UNIQUE | NON_NULL, (empty)
            // 412: workers: typeof(_29) = *mut {l39} *mut {l40} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 412: workers:   l39 = UNIQUE | NON_NULL, (empty)
            // 412: workers:   l40 = UNIQUE | NON_NULL, (empty)
            // 412: workers: typeof(_27 = move _28 as *const Worker (Pointer(MutToConstPointer))) = *const {l81} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 412: workers:   l81 = UNIQUE | NON_NULL, (empty)
        );
    } else {
        printf(b"c - \0" as *const u8 as *const libc::c_char);
        // 415: b"c - \0" as *c ... _char: typeof(_31) = *const {l43} i8
        // 415: b"c - \0" as *c ... _char:   l43 = UNIQUE | NON_NULL, (empty)
        // 415: b"c - \0" as *c ... st u8: typeof(_32) = *const {l45} u8
        // 415: b"c - \0" as *c ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
        // 415: b"c - \0": typeof(_33) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 415: b"c - \0":   l47 = UNIQUE | NON_NULL, (empty)
        // 415: b"c - \0": typeof(_34) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 415: b"c - \0":   l49 = UNIQUE | NON_NULL, FIXED
        // 415: b"c - \0": typeof(_34 = const b"c - \x00") = & {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 415: b"c - \0":   l82 = UNIQUE | NON_NULL, (empty)
        // 415: b"c - \0": typeof(_33 = &raw const (*_34)) = *const {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 415: b"c - \0":   l83 = UNIQUE | NON_NULL, (empty)
        // 415: b"c - \0" as *c ... st u8: typeof(_32 = move _33 as *const u8 (Pointer(ArrayToPointer))) = *const {l84} u8
        // 415: b"c - \0" as *c ... st u8:   l84 = UNIQUE | NON_NULL, (empty)
        // 415: b"c - \0" as *c ... _char: typeof(_31 = move _32 as *const i8 (Misc)) = *const {l85} i8
        // 415: b"c - \0" as *c ... _char:   l85 = UNIQUE | NON_NULL, (empty)
    }
    ap = args.clone();
    // 417: args.clone(): typeof(_36) = & {l52} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 417: args.clone():   l52 = UNIQUE | NON_NULL, (empty)
    // 417: args.clone(): typeof(_36 = &_4) = & {l86} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 417: args.clone():   l86 = UNIQUE | NON_NULL, (empty)
    vprintf(fmt, ap.as_va_list());
    // 418: fmt: typeof(_38) = *const {l55} i8
    // 418: fmt:   l55 = UNIQUE | NON_NULL, (empty)
    // 418: ap.as_va_list(): typeof(_40) = &mut {l58} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 418: ap.as_va_list():   l58 = UNIQUE | NON_NULL, (empty)
    // 418: ap.as_va_list(): typeof(_40 = &mut _5) = &mut {l87} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 418: ap.as_va_list():   l87 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stdout);
    // 419: stdout: typeof(_43) = *mut {l62} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 419: stdout:   l62 = UNIQUE | NON_NULL, (empty)
    // 419: stdout: typeof(_44) = *mut {l64} *mut {l65} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 419: stdout:   l64 = UNIQUE | NON_NULL, (empty)
    // 419: stdout:   l65 = UNIQUE | NON_NULL, (empty)
    fflush(stdout);
    // 420: stdout: typeof(_46) = *mut {l68} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 420: stdout:   l68 = UNIQUE | NON_NULL, (empty)
    // 420: stdout: typeof(_47) = *mut {l70} *mut {l71} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 420: stdout:   l70 = UNIQUE | NON_NULL, (empty)
    // 420: stdout:   l71 = UNIQUE | NON_NULL, (empty)
    msgunlock(0 as *mut libc::c_void);
    // 421: 0 as *mut libc: ... _void: typeof(_49) = *mut {l74} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 421: 0 as *mut libc: ... _void:   l74 = UNIQUE | NON_NULL, (empty)
    // 421: 0 as *mut libc: ... _void: typeof(_49 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l88} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 421: 0 as *mut libc: ... _void:   l88 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn die(mut fmt: *const libc::c_char, mut args: ...) {
// 423: mut fmt: typeof(_1) = *const {g6} i8
// 423: mut fmt:   g6 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** [ilingeling] \0" as *const u8 as *const libc::c_char,
        // 426: b"*** [ilingeli ... _char: typeof(_6) = *const {l6} i8
        // 426: b"*** [ilingeli ... _char:   l6 = UNIQUE | NON_NULL, (empty)
        // 426: b"*** [ilingeli ... st u8: typeof(_7) = *const {l8} u8
        // 426: b"*** [ilingeli ... st u8:   l8 = UNIQUE | NON_NULL, (empty)
        // 426: b"*** [ilingeli ... ] \0": typeof(_8) = *const {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 426: b"*** [ilingeli ... ] \0":   l10 = UNIQUE | NON_NULL, (empty)
        // 426: b"*** [ilingeli ... ] \0": typeof(_9) = & {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 426: b"*** [ilingeli ... ] \0":   l12 = UNIQUE | NON_NULL, FIXED
        // 426: b"*** [ilingeli ... ] \0": typeof(_8 = &raw const (*_9)) = *const {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 426: b"*** [ilingeli ... ] \0":   l49 = UNIQUE | NON_NULL, (empty)
        // 426: b"*** [ilingeli ... ] \0": typeof(_9 = const b"*** [ilingeling] \x00") = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
        // 426: b"*** [ilingeli ... ] \0":   l48 = UNIQUE | NON_NULL, (empty)
        // 426: b"*** [ilingeli ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l51} i8
        // 426: b"*** [ilingeli ... _char:   l51 = UNIQUE | NON_NULL, (empty)
        // 426: b"*** [ilingeli ... st u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l50} u8
        // 426: b"*** [ilingeli ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 427: stderr: typeof(_10) = *mut {l14} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 427: stderr:   l14 = UNIQUE | NON_NULL, (empty)
        // 427: stderr: typeof(_11) = *mut {l16} *mut {l17} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 427: stderr:   l16 = UNIQUE | NON_NULL, (empty)
        // 427: stderr:   l17 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 429: args.clone(): typeof(_13) = & {l20} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 429: args.clone():   l20 = UNIQUE | NON_NULL, (empty)
    // 429: args.clone(): typeof(_13 = &_2) = & {l52} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 429: args.clone():   l52 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 430: stderr: typeof(_15) = *mut {l23} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 430: stderr:   l23 = UNIQUE | NON_NULL, (empty)
    // 430: stderr: typeof(_16) = *mut {l25} *mut {l26} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 430: stderr:   l25 = UNIQUE | NON_NULL, (empty)
    // 430: stderr:   l26 = UNIQUE | NON_NULL, (empty)
    // 430: fmt: typeof(_17) = *const {l28} i8
    // 430: fmt:   l28 = UNIQUE | NON_NULL, (empty)
    // 430: ap.as_va_list(): typeof(_19) = &mut {l31} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 430: ap.as_va_list():   l31 = UNIQUE | NON_NULL, (empty)
    // 430: ap.as_va_list(): typeof(_19 = &mut _4) = &mut {l53} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 430: ap.as_va_list():   l53 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 431: stderr: typeof(_22) = *mut {l35} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 431: stderr:   l35 = UNIQUE | NON_NULL, (empty)
    // 431: stderr: typeof(_23) = *mut {l37} *mut {l38} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 431: stderr:   l37 = UNIQUE | NON_NULL, (empty)
    // 431: stderr:   l38 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 432: stderr: typeof(_25) = *mut {l41} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 432: stderr:   l41 = UNIQUE | NON_NULL, (empty)
    // 432: stderr: typeof(_26) = *mut {l43} *mut {l44} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 432: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 432: stderr:   l44 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn warn(mut fmt: *const libc::c_char, mut args: ...) {
// 435: mut fmt: typeof(_1) = *const {g7} i8
// 435: mut fmt:   g7 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** [ilingeling] warning: \0" as *const u8 as *const libc::c_char,
        // 438: b"*** [ilingeli ... _char: typeof(_5) = *const {l5} i8
        // 438: b"*** [ilingeli ... _char:   l5 = UNIQUE | NON_NULL, (empty)
        // 438: b"*** [ilingeli ... st u8: typeof(_6) = *const {l7} u8
        // 438: b"*** [ilingeli ... st u8:   l7 = UNIQUE | NON_NULL, (empty)
        // 438: b"*** [ilingeli ... : \0": typeof(_7) = *const {l9} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 438: b"*** [ilingeli ... : \0":   l9 = UNIQUE | NON_NULL, (empty)
        // 438: b"*** [ilingeli ... : \0": typeof(_8) = & {l11} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 438: b"*** [ilingeli ... : \0":   l11 = UNIQUE | NON_NULL, FIXED
        // 438: b"*** [ilingeli ... : \0": typeof(_7 = &raw const (*_8)) = *const {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 438: b"*** [ilingeli ... : \0":   l46 = UNIQUE | NON_NULL, (empty)
        // 438: b"*** [ilingeli ... : \0": typeof(_8 = const b"*** [ilingeling] warning: \x00") = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 438: b"*** [ilingeli ... : \0":   l45 = UNIQUE | NON_NULL, (empty)
        // 438: b"*** [ilingeli ... _char: typeof(_5 = move _6 as *const i8 (Misc)) = *const {l48} i8
        // 438: b"*** [ilingeli ... _char:   l48 = UNIQUE | NON_NULL, (empty)
        // 438: b"*** [ilingeli ... st u8: typeof(_6 = move _7 as *const u8 (Pointer(ArrayToPointer))) = *const {l47} u8
        // 438: b"*** [ilingeli ... st u8:   l47 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 439: stderr: typeof(_9) = *mut {l13} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 439: stderr:   l13 = UNIQUE | NON_NULL, (empty)
        // 439: stderr: typeof(_10) = *mut {l15} *mut {l16} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 439: stderr:   l15 = UNIQUE | NON_NULL, (empty)
        // 439: stderr:   l16 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 441: args.clone(): typeof(_12) = & {l19} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 441: args.clone():   l19 = UNIQUE | NON_NULL, (empty)
    // 441: args.clone(): typeof(_12 = &_2) = & {l49} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 441: args.clone():   l49 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 442: stderr: typeof(_14) = *mut {l22} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 442: stderr:   l22 = UNIQUE | NON_NULL, (empty)
    // 442: stderr: typeof(_15) = *mut {l24} *mut {l25} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 442: stderr:   l24 = UNIQUE | NON_NULL, (empty)
    // 442: stderr:   l25 = UNIQUE | NON_NULL, (empty)
    // 442: fmt: typeof(_16) = *const {l27} i8
    // 442: fmt:   l27 = UNIQUE | NON_NULL, (empty)
    // 442: ap.as_va_list(): typeof(_18) = &mut {l30} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 442: ap.as_va_list():   l30 = UNIQUE | NON_NULL, (empty)
    // 442: ap.as_va_list(): typeof(_18 = &mut _3) = &mut {l50} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 442: ap.as_va_list():   l50 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 443: stderr: typeof(_21) = *mut {l34} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 443: stderr:   l34 = UNIQUE | NON_NULL, (empty)
    // 443: stderr: typeof(_22) = *mut {l36} *mut {l37} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 443: stderr:   l36 = UNIQUE | NON_NULL, (empty)
    // 443: stderr:   l37 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 444: stderr: typeof(_24) = *mut {l40} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 444: stderr:   l40 = UNIQUE | NON_NULL, (empty)
    // 444: stderr: typeof(_25) = *mut {l42} *mut {l43} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 444: stderr:   l42 = UNIQUE | NON_NULL, (empty)
    // 444: stderr:   l43 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn currentime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
    // 452: &mut tv: typeof(_8) = *mut {l8} DefId(0:251 ~ ilingeling[c969]::timeval)
    // 452: &mut tv:   l8 = UNIQUE | NON_NULL, (empty)
    // 452: &mut tv: typeof(_9) = &mut {l10} DefId(0:251 ~ ilingeling[c969]::timeval)
    // 452: &mut tv:   l10 = UNIQUE | NON_NULL, (empty)
    // 452: 0 as *mut libc: ... _void: typeof(_10) = *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 452: 0 as *mut libc: ... _void:   l12 = UNIQUE | NON_NULL, (empty)
    // 452: &mut tv: typeof(_9 = &mut _4) = &mut {l18} DefId(0:251 ~ ilingeling[c969]::timeval)
    // 452: &mut tv:   l18 = UNIQUE | NON_NULL, (empty)
    // 452: 0 as *mut libc: ... _void: typeof(_10 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 452: 0 as *mut libc: ... _void:   l20 = UNIQUE | NON_NULL, (empty)
    // 452: &mut tv: typeof(_8 = &raw mut (*_9)) = *mut {l19} DefId(0:251 ~ ilingeling[c969]::timeval)
    // 452: &mut tv:   l19 = UNIQUE | NON_NULL, (empty)
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe extern "C" fn getime() -> libc::c_double {
    return currentime() - startime;
    // 459: startime: typeof(_4) = *mut {l4} f64
    // 459: startime:   l4 = READ | UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn isnum(mut str: *const libc::c_char) -> libc::c_int {
// 461: mut str: typeof(_1) = *const {g8} i8
// 461: mut str:   g8 = UNIQUE | NON_NULL, FIXED
    let mut p: *const libc::c_char = str;
    // 462: mut p: typeof(_3) = *const {l3} i8
    // 462: mut p:   l3 = UNIQUE | NON_NULL, (empty)
    let fresh0 = p;
    // 463: fresh0: typeof(_4) = *const {l5} i8
    // 463: fresh0:   l5 = UNIQUE | NON_NULL, (empty)
    p = p.offset(1);
    // 464: p.offset(1): typeof(_5) = *const {l7} i8
    // 464: p.offset(1):   l7 = UNIQUE | NON_NULL, (empty)
    // 464: p: typeof(_6) = *const {l9} i8
    // 464: p:   l9 = UNIQUE | NON_NULL, (empty)
    if *(*__ctype_b_loc()).offset(*fresh0 as libc::c_int as isize) as libc::c_int
    // 465: (*__ctype_b_loc ... size): typeof(_12) = *const {l16} u16
    // 465: (*__ctype_b_loc ... size):   l16 = UNIQUE | NON_NULL, (empty)
    // 465: (*__ctype_b_loc()): typeof(_13) = *const {l18} u16
    // 465: (*__ctype_b_loc()):   l18 = UNIQUE | NON_NULL, (empty)
    // 465: __ctype_b_loc(): typeof(_14) = *mut {l20} *const {l21} u16
    // 465: __ctype_b_loc():   l20 = UNIQUE | NON_NULL, (empty)
    // 465: __ctype_b_loc():   l21 = UNIQUE | NON_NULL, (empty)
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return 0 as libc::c_int;
    }
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
        // 472: (*__ctype_b_loc ... size): typeof(_32) = *const {l40} u16
        // 472: (*__ctype_b_loc ... size):   l40 = UNIQUE | NON_NULL, (empty)
        // 472: (*__ctype_b_loc()): typeof(_33) = *const {l42} u16
        // 472: (*__ctype_b_loc()):   l42 = UNIQUE | NON_NULL, (empty)
        // 472: __ctype_b_loc(): typeof(_34) = *mut {l44} *const {l45} u16
        // 472: __ctype_b_loc():   l44 = UNIQUE | NON_NULL, (empty)
        // 472: __ctype_b_loc():   l45 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        // 476: p.offset(1): typeof(_41) = *const {l53} i8
        // 476: p.offset(1):   l53 = UNIQUE | NON_NULL, (empty)
        // 476: p: typeof(_42) = *const {l55} i8
        // 476: p:   l55 = UNIQUE | NON_NULL, (empty)
        p;
        // 477: p: typeof(_43) = *const {l57} i8
        // 477: p:   l57 = UNIQUE | NON_NULL, (empty)
    }
    return (*p == 0) as libc::c_int;
}
unsafe extern "C" fn term(mut voidptr: *mut libc::c_void) -> libc::c_int {
// 481: mut voidptr: typeof(_1) = *mut {g9} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 481: mut voidptr:   g9 = UNIQUE | NON_NULL, FIXED
    let mut w: *mut Worker = voidptr as *mut Worker;
    // 482: mut w: typeof(_3) = *mut {l3} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 482: mut w:   l3 = UNIQUE | NON_NULL, (empty)
    // 482: voidptr: typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 482: voidptr:   l5 = UNIQUE | NON_NULL, (empty)
    // 482: voidptr as *mut ... orker: typeof(_3 = move _4 as *mut Worker (Misc)) = *mut {l87} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 482: voidptr as *mut ... orker:   l87 = UNIQUE | NON_NULL, (empty)
    let mut res: libc::c_int = 0;
    msg(
        w,
        // 485: w: typeof(_7) = *mut {l9} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 485: w:   l9 = UNIQUE | NON_NULL, (empty)
        3 as libc::c_int,
        b"checking early termination\0" as *const u8 as *const libc::c_char,
        // 487: b"checking earl ... _char: typeof(_9) = *const {l12} i8
        // 487: b"checking earl ... _char:   l12 = UNIQUE | NON_NULL, (empty)
        // 487: b"checking earl ... st u8: typeof(_10) = *const {l14} u8
        // 487: b"checking earl ... st u8:   l14 = UNIQUE | NON_NULL, (empty)
        // 487: b"checking earl ... on\0": typeof(_11) = *const {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 487: b"checking earl ... on\0":   l16 = UNIQUE | NON_NULL, (empty)
        // 487: b"checking earl ... on\0": typeof(_12) = & {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 487: b"checking earl ... on\0":   l18 = UNIQUE | NON_NULL, FIXED
        // 487: b"checking earl ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l90} u8
        // 487: b"checking earl ... st u8:   l90 = UNIQUE | NON_NULL, (empty)
        // 487: b"checking earl ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l91} i8
        // 487: b"checking earl ... _char:   l91 = UNIQUE | NON_NULL, (empty)
        // 487: b"checking earl ... on\0": typeof(_12 = const b"checking early termination\x00") = & {l88} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 487: b"checking earl ... on\0":   l88 = UNIQUE | NON_NULL, (empty)
        // 487: b"checking earl ... on\0": typeof(_11 = &raw const (*_12)) = *const {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 487: b"checking earl ... on\0":   l89 = UNIQUE | NON_NULL, (empty)
    );
    if pthread_mutex_lock(&mut donemutex) != 0 {
    // 489: &mut donemutex: typeof(_16) = *mut {l23} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 489: &mut donemutex:   l23 = UNIQUE | NON_NULL, (empty)
    // 489: &mut donemutex: typeof(_17) = &mut {l25} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 489: &mut donemutex:   l25 = UNIQUE | NON_NULL, (empty)
    // 489: donemutex: typeof(_18) = *mut {l27} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 489: donemutex:   l27 = UNIQUE | NON_NULL, (empty)
    // 489: &mut donemutex: typeof(_17 = &mut (*_18)) = &mut {l92} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 489: &mut donemutex:   l92 = UNIQUE | NON_NULL, (empty)
    // 489: &mut donemutex: typeof(_16 = &raw mut (*_17)) = *mut {l93} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 489: &mut donemutex:   l93 = UNIQUE | NON_NULL, (empty)
        warn(
            b"failed to lock 'done' mutex in termination check\0" as *const u8
            // 491: b"failed to loc ... _char: typeof(_20) = *const {l30} i8
            // 491: b"failed to loc ... _char:   l30 = UNIQUE | NON_NULL, (empty)
            // 491: b"failed to loc ... st u8: typeof(_21) = *const {l32} u8
            // 491: b"failed to loc ... st u8:   l32 = UNIQUE | NON_NULL, (empty)
            // 491: b"failed to loc ... ck\0": typeof(_22) = *const {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 491: b"failed to loc ... ck\0":   l34 = UNIQUE | NON_NULL, (empty)
            // 491: b"failed to loc ... ck\0": typeof(_23) = & {l36} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 491: b"failed to loc ... ck\0":   l36 = UNIQUE | NON_NULL, FIXED
            // 491: b"failed to loc ... ck\0": typeof(_22 = &raw const (*_23)) = *const {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 491: b"failed to loc ... ck\0":   l95 = UNIQUE | NON_NULL, (empty)
            // 491: b"failed to loc ... ck\0": typeof(_23 = const b"failed to lock \'done\' mutex in termination check\x00") = & {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
            // 491: b"failed to loc ... ck\0":   l94 = UNIQUE | NON_NULL, (empty)
            // 491: b"failed to loc ... st u8: typeof(_21 = move _22 as *const u8 (Pointer(ArrayToPointer))) = *const {l96} u8
            // 491: b"failed to loc ... st u8:   l96 = UNIQUE | NON_NULL, (empty)
            // 491: b"failed to loc ... _char: typeof(_20 = move _21 as *const i8 (Misc)) = *const {l97} i8
            // 491: b"failed to loc ... _char:   l97 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    }
    res = done;
    // 495: done: typeof(_25) = *mut {l39} i32
    // 495: done:   l39 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut donemutex) != 0 {
    // 496: &mut donemutex: typeof(_29) = *mut {l44} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 496: &mut donemutex:   l44 = UNIQUE | NON_NULL, (empty)
    // 496: &mut donemutex: typeof(_30) = &mut {l46} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 496: &mut donemutex:   l46 = UNIQUE | NON_NULL, (empty)
    // 496: donemutex: typeof(_31) = *mut {l48} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 496: donemutex:   l48 = UNIQUE | NON_NULL, (empty)
    // 496: &mut donemutex: typeof(_29 = &raw mut (*_30)) = *mut {l99} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 496: &mut donemutex:   l99 = UNIQUE | NON_NULL, (empty)
    // 496: &mut donemutex: typeof(_30 = &mut (*_31)) = &mut {l98} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 496: &mut donemutex:   l98 = UNIQUE | NON_NULL, (empty)
        warn(
            b"failed to unlock 'done' mutex in termination check\0" as *const u8
            // 498: b"failed to unl ... _char: typeof(_33) = *const {l51} i8
            // 498: b"failed to unl ... _char:   l51 = UNIQUE | NON_NULL, (empty)
            // 498: b"failed to unl ... st u8: typeof(_34) = *const {l53} u8
            // 498: b"failed to unl ... st u8:   l53 = UNIQUE | NON_NULL, (empty)
            // 498: b"failed to unl ... ck\0": typeof(_35) = *const {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 498: b"failed to unl ... ck\0":   l55 = UNIQUE | NON_NULL, (empty)
            // 498: b"failed to unl ... ck\0": typeof(_36) = & {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 498: b"failed to unl ... ck\0":   l57 = UNIQUE | NON_NULL, FIXED
            // 498: b"failed to unl ... ck\0": typeof(_36 = const b"failed to unlock \'done\' mutex in termination check\x00") = & {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 498: b"failed to unl ... ck\0":   l100 = UNIQUE | NON_NULL, (empty)
            // 498: b"failed to unl ... ck\0": typeof(_35 = &raw const (*_36)) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000033)) }]
            // 498: b"failed to unl ... ck\0":   l101 = UNIQUE | NON_NULL, (empty)
            // 498: b"failed to unl ... _char: typeof(_33 = move _34 as *const i8 (Misc)) = *const {l103} i8
            // 498: b"failed to unl ... _char:   l103 = UNIQUE | NON_NULL, (empty)
            // 498: b"failed to unl ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l102} u8
            // 498: b"failed to unl ... st u8:   l102 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    }
    msg(
        w,
        // 503: w: typeof(_38) = *mut {l60} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 503: w:   l60 = UNIQUE | NON_NULL, (empty)
        3 as libc::c_int,
        b"early termination check %s\0" as *const u8 as *const libc::c_char,
        // 505: b"early termina ... _char: typeof(_40) = *const {l63} i8
        // 505: b"early termina ... _char:   l63 = UNIQUE | NON_NULL, (empty)
        // 505: b"early termina ... st u8: typeof(_41) = *const {l65} u8
        // 505: b"early termina ... st u8:   l65 = UNIQUE | NON_NULL, (empty)
        // 505: b"early termina ... %s\0": typeof(_42) = *const {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 505: b"early termina ... %s\0":   l67 = UNIQUE | NON_NULL, (empty)
        // 505: b"early termina ... %s\0": typeof(_43) = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 505: b"early termina ... %s\0":   l69 = UNIQUE | NON_NULL, FIXED
        // 505: b"early termina ... _char: typeof(_40 = move _41 as *const i8 (Misc)) = *const {l107} i8
        // 505: b"early termina ... _char:   l107 = UNIQUE | NON_NULL, (empty)
        // 505: b"early termina ... st u8: typeof(_41 = move _42 as *const u8 (Pointer(ArrayToPointer))) = *const {l106} u8
        // 505: b"early termina ... st u8:   l106 = UNIQUE | NON_NULL, (empty)
        // 505: b"early termina ... %s\0": typeof(_43 = const b"early termination check %s\x00") = & {l104} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 505: b"early termina ... %s\0":   l104 = UNIQUE | NON_NULL, (empty)
        // 505: b"early termina ... %s\0": typeof(_42 = &raw const (*_43)) = *const {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
        // 505: b"early termina ... %s\0":   l105 = UNIQUE | NON_NULL, (empty)
        if res != 0 {
        // 506: if res != 0 { b ... har }: typeof(_44) = *const {l71} i8
        // 506: if res != 0 { b ... har }:   l71 = UNIQUE | NON_NULL, (empty)
            b"succeeded\0" as *const u8 as *const libc::c_char
            // 507: b"succeeded\0"  ... st u8: typeof(_47) = *const {l75} u8
            // 507: b"succeeded\0"  ... st u8:   l75 = UNIQUE | NON_NULL, (empty)
            // 507: b"succeeded\0": typeof(_48) = *const {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 507: b"succeeded\0":   l77 = UNIQUE | NON_NULL, (empty)
            // 507: b"succeeded\0": typeof(_49) = & {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 507: b"succeeded\0":   l79 = UNIQUE | NON_NULL, FIXED
            // 507: b"succeeded\0"  ... st u8: typeof(_47 = move _48 as *const u8 (Pointer(ArrayToPointer))) = *const {l110} u8
            // 507: b"succeeded\0"  ... st u8:   l110 = UNIQUE | NON_NULL, (empty)
            // 507: b"succeeded\0": typeof(_48 = &raw const (*_49)) = *const {l109} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 507: b"succeeded\0":   l109 = UNIQUE | NON_NULL, (empty)
            // 507: b"succeeded\0": typeof(_49 = const b"succeeded\x00") = & {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 507: b"succeeded\0":   l108 = UNIQUE | NON_NULL, (empty)
            // 507: b"succeeded\0"  ... _char: typeof(_44 = move _47 as *const i8 (Misc)) = *const {l111} i8
            // 507: b"succeeded\0"  ... _char:   l111 = UNIQUE | NON_NULL, (empty)
        } else {
            b"failed\0" as *const u8 as *const libc::c_char
            // 509: b"failed\0" as  ... st u8: typeof(_50) = *const {l81} u8
            // 509: b"failed\0" as  ... st u8:   l81 = UNIQUE | NON_NULL, (empty)
            // 509: b"failed\0": typeof(_51) = *const {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"failed\0":   l83 = UNIQUE | NON_NULL, (empty)
            // 509: b"failed\0": typeof(_52) = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"failed\0":   l85 = UNIQUE | NON_NULL, FIXED
            // 509: b"failed\0" as  ... st u8: typeof(_50 = move _51 as *const u8 (Pointer(ArrayToPointer))) = *const {l114} u8
            // 509: b"failed\0" as  ... st u8:   l114 = UNIQUE | NON_NULL, (empty)
            // 509: b"failed\0" as  ... _char: typeof(_44 = move _50 as *const i8 (Misc)) = *const {l115} i8
            // 509: b"failed\0" as  ... _char:   l115 = UNIQUE | NON_NULL, (empty)
            // 509: b"failed\0": typeof(_52 = const b"failed\x00") = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"failed\0":   l112 = UNIQUE | NON_NULL, (empty)
            // 509: b"failed\0": typeof(_51 = &raw const (*_52)) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 509: b"failed\0":   l113 = UNIQUE | NON_NULL, (empty)
        },
    );
    return res;
}
unsafe extern "C" fn progress(
    mut pmille: libc::c_int,
    mut total: libc::c_int,
    mut max: libc::c_int,
    mut avg: libc::c_double,
    mut nl: libc::c_int,
) {
    let mut ch: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lim: libc::c_int = 0;
    let mut eta: libc::c_int = 0;
    let mut fmt: [libc::c_char; 16] = [0; 16];
    let mut rem: libc::c_double = 0.;
    msglock(0 as *mut libc::c_void);
    // 527: 0 as *mut libc: ... _void: typeof(_13) = *mut {l13} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 527: 0 as *mut libc: ... _void:   l13 = UNIQUE | NON_NULL, (empty)
    // 527: 0 as *mut libc: ... _void: typeof(_13 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l284} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 527: 0 as *mut libc: ... _void:   l284 = UNIQUE | NON_NULL, (empty)
    if isatty(1 as libc::c_int) != 0 {
        fputc('\r' as i32, stdout);
        // 529: stdout: typeof(_20) = *mut {l21} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 529: stdout:   l21 = UNIQUE | NON_NULL, (empty)
        // 529: stdout: typeof(_21) = *mut {l23} *mut {l24} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 529: stdout:   l23 = UNIQUE | NON_NULL, (empty)
        // 529: stdout:   l24 = UNIQUE | NON_NULL, (empty)
    }
    lim = 10 as libc::c_int;
    i = 1 as libc::c_int;
    while lim < max && i < 11 as libc::c_int {
        lim *= 10 as libc::c_int;
        i += 1;
        i;
    }
    sprintf(
        fmt.as_mut_ptr(),
        // 539: fmt.as_mut_ptr(): typeof(_41) = *mut {l45} i8
        // 539: fmt.as_mut_ptr():   l45 = UNIQUE | NON_NULL, (empty)
        // 539: fmt.as_mut_ptr(): typeof(_42) = &mut {l47} [i8]
        // 539: fmt.as_mut_ptr():   l47 = UNIQUE | NON_NULL, FIXED
        // 539: fmt.as_mut_ptr(): typeof(_43) = &mut {l49} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 539: fmt.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
        // 539: fmt.as_mut_ptr(): typeof(_42 = move _43 as &mut [i8] (Pointer(Unsize))) = &mut {l286} [i8]
        // 539: fmt.as_mut_ptr():   l286 = UNIQUE | NON_NULL, FIXED
        // 539: fmt.as_mut_ptr(): typeof(_43 = &mut _10) = &mut {l285} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
        // 539: fmt.as_mut_ptr():   l285 = UNIQUE | NON_NULL, (empty)
        b"c %%0%dd\0" as *const u8 as *const libc::c_char,
        // 540: b"c %%0%dd\0" a ... _char: typeof(_44) = *const {l51} i8
        // 540: b"c %%0%dd\0" a ... _char:   l51 = UNIQUE | NON_NULL, (empty)
        // 540: b"c %%0%dd\0" a ... st u8: typeof(_45) = *const {l53} u8
        // 540: b"c %%0%dd\0" a ... st u8:   l53 = UNIQUE | NON_NULL, (empty)
        // 540: b"c %%0%dd\0": typeof(_46) = *const {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 540: b"c %%0%dd\0":   l55 = UNIQUE | NON_NULL, (empty)
        // 540: b"c %%0%dd\0": typeof(_47) = & {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 540: b"c %%0%dd\0":   l57 = UNIQUE | NON_NULL, FIXED
        // 540: b"c %%0%dd\0": typeof(_46 = &raw const (*_47)) = *const {l288} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 540: b"c %%0%dd\0":   l288 = UNIQUE | NON_NULL, (empty)
        // 540: b"c %%0%dd\0" a ... st u8: typeof(_45 = move _46 as *const u8 (Pointer(ArrayToPointer))) = *const {l289} u8
        // 540: b"c %%0%dd\0" a ... st u8:   l289 = UNIQUE | NON_NULL, (empty)
        // 540: b"c %%0%dd\0" a ... _char: typeof(_44 = move _45 as *const i8 (Misc)) = *const {l290} i8
        // 540: b"c %%0%dd\0" a ... _char:   l290 = UNIQUE | NON_NULL, (empty)
        // 540: b"c %%0%dd\0": typeof(_47 = const b"c %%0%dd\x00") = & {l287} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 540: b"c %%0%dd\0":   l287 = UNIQUE | NON_NULL, (empty)
        i,
    );
    printf(fmt.as_mut_ptr(), total);
    // 543: fmt.as_mut_ptr(): typeof(_50) = *const {l61} i8
    // 543: fmt.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
    // 543: fmt.as_mut_ptr(): typeof(_51) = *mut {l63} i8
    // 543: fmt.as_mut_ptr():   l63 = UNIQUE | NON_NULL, (empty)
    // 543: fmt.as_mut_ptr(): typeof(_52) = &mut {l65} [i8]
    // 543: fmt.as_mut_ptr():   l65 = UNIQUE | NON_NULL, FIXED
    // 543: fmt.as_mut_ptr(): typeof(_53) = &mut {l67} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
    // 543: fmt.as_mut_ptr():   l67 = UNIQUE | NON_NULL, (empty)
    // 543: fmt.as_mut_ptr(): typeof(_53 = &mut _10) = &mut {l291} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
    // 543: fmt.as_mut_ptr():   l291 = UNIQUE | NON_NULL, (empty)
    // 543: fmt.as_mut_ptr(): typeof(_52 = move _53 as &mut [i8] (Pointer(Unsize))) = &mut {l292} [i8]
    // 543: fmt.as_mut_ptr():   l292 = UNIQUE | NON_NULL, FIXED
    // 543: fmt.as_mut_ptr(): typeof(_50 = move _51 as *const i8 (Pointer(MutToConstPointer))) = *const {l293} i8
    // 543: fmt.as_mut_ptr():   l293 = UNIQUE | NON_NULL, (empty)
    printf(b" / %d |\0" as *const u8 as *const libc::c_char, max);
    // 544: b" / %d |\0" as ... _char: typeof(_56) = *const {l71} i8
    // 544: b" / %d |\0" as ... _char:   l71 = UNIQUE | NON_NULL, (empty)
    // 544: b" / %d |\0" as ... st u8: typeof(_57) = *const {l73} u8
    // 544: b" / %d |\0" as ... st u8:   l73 = UNIQUE | NON_NULL, (empty)
    // 544: b" / %d |\0": typeof(_58) = *const {l75} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 544: b" / %d |\0":   l75 = UNIQUE | NON_NULL, (empty)
    // 544: b" / %d |\0": typeof(_59) = & {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 544: b" / %d |\0":   l77 = UNIQUE | NON_NULL, FIXED
    // 544: b" / %d |\0": typeof(_59 = const b" / %d |\x00") = & {l294} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 544: b" / %d |\0":   l294 = UNIQUE | NON_NULL, (empty)
    // 544: b" / %d |\0" as ... st u8: typeof(_57 = move _58 as *const u8 (Pointer(ArrayToPointer))) = *const {l296} u8
    // 544: b" / %d |\0" as ... st u8:   l296 = UNIQUE | NON_NULL, (empty)
    // 544: b" / %d |\0": typeof(_58 = &raw const (*_59)) = *const {l295} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
    // 544: b" / %d |\0":   l295 = UNIQUE | NON_NULL, (empty)
    // 544: b" / %d |\0" as ... _char: typeof(_56 = move _57 as *const i8 (Misc)) = *const {l297} i8
    // 544: b" / %d |\0" as ... _char:   l297 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < pmille / 50 as libc::c_int {
        fputc('=' as i32, stdout);
        // 547: stdout: typeof(_74) = *mut {l93} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 547: stdout:   l93 = UNIQUE | NON_NULL, (empty)
        // 547: stdout: typeof(_75) = *mut {l95} *mut {l96} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 547: stdout:   l95 = UNIQUE | NON_NULL, (empty)
        // 547: stdout:   l96 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    if total == max {
        ch = '=' as i32;
    } else {
        match pmille % 4 as libc::c_int {
            1 => {
                ch = '\\' as i32;
            }
            2 => {
                ch = '|' as i32;
            }
            3 => {
                ch = '/' as i32;
            }
            _ => {
                ch = '-' as i32;
            }
        }
    }
    let fresh1 = i;
    i = i + 1;
    if fresh1 < 20 as libc::c_int {
        fputc(ch, stdout);
        // 572: stdout: typeof(_101) = *mut {l123} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 572: stdout:   l123 = UNIQUE | NON_NULL, (empty)
        // 572: stdout: typeof(_102) = *mut {l125} *mut {l126} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 572: stdout:   l125 = UNIQUE | NON_NULL, (empty)
        // 572: stdout:   l126 = UNIQUE | NON_NULL, (empty)
    }
    loop {
        let fresh2 = i;
        i = i + 1;
        if !(fresh2 < 20 as libc::c_int) {
            break;
        }
        fputc('-' as i32, stdout);
        // 580: stdout: typeof(_115) = *mut {l140} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 580: stdout:   l140 = UNIQUE | NON_NULL, (empty)
        // 580: stdout: typeof(_116) = *mut {l142} *mut {l143} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 580: stdout:   l142 = UNIQUE | NON_NULL, (empty)
        // 580: stdout:   l143 = UNIQUE | NON_NULL, (empty)
    }
    printf(
        b"| %3d%%\0" as *const u8 as *const libc::c_char,
        // 583: b"| %3d%%\0" as ... _char: typeof(_118) = *const {l146} i8
        // 583: b"| %3d%%\0" as ... _char:   l146 = UNIQUE | NON_NULL, (empty)
        // 583: b"| %3d%%\0" as ... st u8: typeof(_119) = *const {l148} u8
        // 583: b"| %3d%%\0" as ... st u8:   l148 = UNIQUE | NON_NULL, (empty)
        // 583: b"| %3d%%\0": typeof(_120) = *const {l150} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 583: b"| %3d%%\0":   l150 = UNIQUE | NON_NULL, (empty)
        // 583: b"| %3d%%\0": typeof(_121) = & {l152} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 583: b"| %3d%%\0":   l152 = UNIQUE | NON_NULL, FIXED
        // 583: b"| %3d%%\0" as ... _char: typeof(_118 = move _119 as *const i8 (Misc)) = *const {l301} i8
        // 583: b"| %3d%%\0" as ... _char:   l301 = UNIQUE | NON_NULL, (empty)
        // 583: b"| %3d%%\0": typeof(_121 = const b"| %3d%%\x00") = & {l298} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 583: b"| %3d%%\0":   l298 = UNIQUE | NON_NULL, (empty)
        // 583: b"| %3d%%\0" as ... st u8: typeof(_119 = move _120 as *const u8 (Pointer(ArrayToPointer))) = *const {l300} u8
        // 583: b"| %3d%%\0" as ... st u8:   l300 = UNIQUE | NON_NULL, (empty)
        // 583: b"| %3d%%\0": typeof(_120 = &raw const (*_121)) = *const {l299} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 583: b"| %3d%%\0":   l299 = UNIQUE | NON_NULL, (empty)
        pmille / 10 as libc::c_int,
    );
    printf(b" %.4f sec/cube\0" as *const u8 as *const libc::c_char, avg);
    // 586: b" %.4f sec/cub ... _char: typeof(_130) = *const {l162} i8
    // 586: b" %.4f sec/cub ... _char:   l162 = UNIQUE | NON_NULL, (empty)
    // 586: b" %.4f sec/cub ... st u8: typeof(_131) = *const {l164} u8
    // 586: b" %.4f sec/cub ... st u8:   l164 = UNIQUE | NON_NULL, (empty)
    // 586: b" %.4f sec/cube\0": typeof(_132) = *const {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
    // 586: b" %.4f sec/cube\0":   l166 = UNIQUE | NON_NULL, (empty)
    // 586: b" %.4f sec/cube\0": typeof(_133) = & {l168} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
    // 586: b" %.4f sec/cube\0":   l168 = UNIQUE | NON_NULL, FIXED
    // 586: b" %.4f sec/cube\0": typeof(_132 = &raw const (*_133)) = *const {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
    // 586: b" %.4f sec/cube\0":   l303 = UNIQUE | NON_NULL, (empty)
    // 586: b" %.4f sec/cub ... _char: typeof(_130 = move _131 as *const i8 (Misc)) = *const {l305} i8
    // 586: b" %.4f sec/cub ... _char:   l305 = UNIQUE | NON_NULL, (empty)
    // 586: b" %.4f sec/cube\0": typeof(_133 = const b" %.4f sec/cube\x00") = & {l302} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
    // 586: b" %.4f sec/cube\0":   l302 = UNIQUE | NON_NULL, (empty)
    // 586: b" %.4f sec/cub ... st u8: typeof(_131 = move _132 as *const u8 (Pointer(ArrayToPointer))) = *const {l304} u8
    // 586: b" %.4f sec/cub ... st u8:   l304 = UNIQUE | NON_NULL, (empty)
    if max <= total {
        eta = 0 as libc::c_int;
    } else {
        rem = (max - total) as libc::c_double * avg;
        if rem >= (100 as libc::c_int * 3600 as libc::c_int) as libc::c_double {
            eta = 2147483647 as libc::c_int;
        } else {
            eta = rem as libc::c_int;
        }
    }
    if eta < 2147483647 as libc::c_int {
        if eta > 3600 as libc::c_int {
            printf(
                b" %02d:\0" as *const u8 as *const libc::c_char,
                // 600: b" %02d:\0" as  ... _char: typeof(_164) = *const {l200} i8
                // 600: b" %02d:\0" as  ... _char:   l200 = UNIQUE | NON_NULL, (empty)
                // 600: b" %02d:\0" as  ... st u8: typeof(_165) = *const {l202} u8
                // 600: b" %02d:\0" as  ... st u8:   l202 = UNIQUE | NON_NULL, (empty)
                // 600: b" %02d:\0": typeof(_166) = *const {l204} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 600: b" %02d:\0":   l204 = UNIQUE | NON_NULL, (empty)
                // 600: b" %02d:\0": typeof(_167) = & {l206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 600: b" %02d:\0":   l206 = UNIQUE | NON_NULL, FIXED
                // 600: b" %02d:\0" as  ... _char: typeof(_164 = move _165 as *const i8 (Misc)) = *const {l309} i8
                // 600: b" %02d:\0" as  ... _char:   l309 = UNIQUE | NON_NULL, (empty)
                // 600: b" %02d:\0" as  ... st u8: typeof(_165 = move _166 as *const u8 (Pointer(ArrayToPointer))) = *const {l308} u8
                // 600: b" %02d:\0" as  ... st u8:   l308 = UNIQUE | NON_NULL, (empty)
                // 600: b" %02d:\0": typeof(_167 = const b" %02d:\x00") = & {l306} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 600: b" %02d:\0":   l306 = UNIQUE | NON_NULL, (empty)
                // 600: b" %02d:\0": typeof(_166 = &raw const (*_167)) = *const {l307} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 600: b" %02d:\0":   l307 = UNIQUE | NON_NULL, (empty)
                eta / 3600 as libc::c_int,
            );
            eta %= 3600 as libc::c_int;
        } else {
            printf(b"    \0" as *const u8 as *const libc::c_char);
            // 605: b" \0" as *cons ... _char: typeof(_181) = *const {l221} i8
            // 605: b" \0" as *cons ... _char:   l221 = UNIQUE | NON_NULL, (empty)
            // 605: b" \0" as *const u8: typeof(_182) = *const {l223} u8
            // 605: b" \0" as *const u8:   l223 = UNIQUE | NON_NULL, (empty)
            // 605: b" \0": typeof(_183) = *const {l225} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 605: b" \0":   l225 = UNIQUE | NON_NULL, (empty)
            // 605: b" \0": typeof(_184) = & {l227} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 605: b" \0":   l227 = UNIQUE | NON_NULL, FIXED
            // 605: b" \0" as *const u8: typeof(_182 = move _183 as *const u8 (Pointer(ArrayToPointer))) = *const {l312} u8
            // 605: b" \0" as *const u8:   l312 = UNIQUE | NON_NULL, (empty)
            // 605: b" \0" as *cons ... _char: typeof(_181 = move _182 as *const i8 (Misc)) = *const {l313} i8
            // 605: b" \0" as *cons ... _char:   l313 = UNIQUE | NON_NULL, (empty)
            // 605: b" \0": typeof(_184 = const b"    \x00") = & {l310} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 605: b" \0":   l310 = UNIQUE | NON_NULL, (empty)
            // 605: b" \0": typeof(_183 = &raw const (*_184)) = *const {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 605: b" \0":   l311 = UNIQUE | NON_NULL, (empty)
        }
        printf(
            b"%02d:%02d ETS\0" as *const u8 as *const libc::c_char,
            // 608: b"%02d:%02d ETS ... _char: typeof(_186) = *const {l230} i8
            // 608: b"%02d:%02d ETS ... _char:   l230 = UNIQUE | NON_NULL, (empty)
            // 608: b"%02d:%02d ETS ... st u8: typeof(_187) = *const {l232} u8
            // 608: b"%02d:%02d ETS ... st u8:   l232 = UNIQUE | NON_NULL, (empty)
            // 608: b"%02d:%02d ETS\0": typeof(_188) = *const {l234} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 608: b"%02d:%02d ETS\0":   l234 = UNIQUE | NON_NULL, (empty)
            // 608: b"%02d:%02d ETS\0": typeof(_189) = & {l236} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 608: b"%02d:%02d ETS\0":   l236 = UNIQUE | NON_NULL, FIXED
            // 608: b"%02d:%02d ETS\0": typeof(_188 = &raw const (*_189)) = *const {l315} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 608: b"%02d:%02d ETS\0":   l315 = UNIQUE | NON_NULL, (empty)
            // 608: b"%02d:%02d ETS\0": typeof(_189 = const b"%02d:%02d ETS\x00") = & {l314} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 608: b"%02d:%02d ETS\0":   l314 = UNIQUE | NON_NULL, (empty)
            // 608: b"%02d:%02d ETS ... st u8: typeof(_187 = move _188 as *const u8 (Pointer(ArrayToPointer))) = *const {l316} u8
            // 608: b"%02d:%02d ETS ... st u8:   l316 = UNIQUE | NON_NULL, (empty)
            // 608: b"%02d:%02d ETS ... _char: typeof(_186 = move _187 as *const i8 (Misc)) = *const {l317} i8
            // 608: b"%02d:%02d ETS ... _char:   l317 = UNIQUE | NON_NULL, (empty)
            eta / 60 as libc::c_int,
            eta % 60 as libc::c_int,
        );
    } else {
        printf(b"   --:-- ETS\0" as *const u8 as *const libc::c_char);
        // 613: b" --:-- ETS\0" ... _char: typeof(_205) = *const {l253} i8
        // 613: b" --:-- ETS\0" ... _char:   l253 = UNIQUE | NON_NULL, (empty)
        // 613: b" --:-- ETS\0" ... st u8: typeof(_206) = *const {l255} u8
        // 613: b" --:-- ETS\0" ... st u8:   l255 = UNIQUE | NON_NULL, (empty)
        // 613: b" --:-- ETS\0": typeof(_207) = *const {l257} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 613: b" --:-- ETS\0":   l257 = UNIQUE | NON_NULL, (empty)
        // 613: b" --:-- ETS\0": typeof(_208) = & {l259} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 613: b" --:-- ETS\0":   l259 = UNIQUE | NON_NULL, FIXED
        // 613: b" --:-- ETS\0" ... _char: typeof(_205 = move _206 as *const i8 (Misc)) = *const {l321} i8
        // 613: b" --:-- ETS\0" ... _char:   l321 = UNIQUE | NON_NULL, (empty)
        // 613: b" --:-- ETS\0" ... st u8: typeof(_206 = move _207 as *const u8 (Pointer(ArrayToPointer))) = *const {l320} u8
        // 613: b" --:-- ETS\0" ... st u8:   l320 = UNIQUE | NON_NULL, (empty)
        // 613: b" --:-- ETS\0": typeof(_208 = const b"   --:-- ETS\x00") = & {l318} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 613: b" --:-- ETS\0":   l318 = UNIQUE | NON_NULL, (empty)
        // 613: b" --:-- ETS\0": typeof(_207 = &raw const (*_208)) = *const {l319} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 613: b" --:-- ETS\0":   l319 = UNIQUE | NON_NULL, (empty)
    }
    if nl != 0 || isatty(1 as libc::c_int) == 0 {
        fputc('\n' as i32, stdout);
        // 616: stdout: typeof(_218) = *mut {l270} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 616: stdout:   l270 = UNIQUE | NON_NULL, (empty)
        // 616: stdout: typeof(_219) = *mut {l272} *mut {l273} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 616: stdout:   l272 = UNIQUE | NON_NULL, (empty)
        // 616: stdout:   l273 = UNIQUE | NON_NULL, (empty)
    }
    fflush(stdout);
    // 618: stdout: typeof(_221) = *mut {l276} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 618: stdout:   l276 = UNIQUE | NON_NULL, (empty)
    // 618: stdout: typeof(_222) = *mut {l278} *mut {l279} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 618: stdout:   l278 = UNIQUE | NON_NULL, (empty)
    // 618: stdout:   l279 = UNIQUE | NON_NULL, (empty)
    msgunlock(0 as *mut libc::c_void);
    // 619: 0 as *mut libc: ... _void: typeof(_224) = *mut {l282} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 619: 0 as *mut libc: ... _void:   l282 = UNIQUE | NON_NULL, (empty)
    // 619: 0 as *mut libc: ... _void: typeof(_224 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l322} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 619: 0 as *mut libc: ... _void:   l322 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn initlgl(mut lgl: *mut LGL, mut w: *mut Worker, mut opts: libc::c_int) {
// 621: mut lgl: typeof(_1) = *mut {g10} LGL
// 621: mut lgl:   g10 = UNIQUE | NON_NULL, FIXED
// 621: mut w: typeof(_2) = *mut {g11} DefId(0:297 ~ ilingeling[c969]::Worker)
// 621: mut w:   g11 = UNIQUE | NON_NULL, FIXED
    lglsetid(
        lgl,
        // 623: lgl: typeof(_5) = *mut {l5} LGL
        // 623: lgl:   l5 = UNIQUE | NON_NULL, (empty)
        w.offset_from(workers) as libc::c_long as libc::c_int,
        // 624: w: typeof(_9) = *mut {l10} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 624: w:   l10 = UNIQUE | NON_NULL, (empty)
        // 624: workers: typeof(_10) = *const {l12} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 624: workers:   l12 = UNIQUE | NON_NULL, (empty)
        // 624: workers: typeof(_11) = *mut {l14} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 624: workers:   l14 = UNIQUE | NON_NULL, (empty)
        // 624: workers: typeof(_12) = *mut {l16} *mut {l17} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 624: workers:   l16 = UNIQUE | NON_NULL, (empty)
        // 624: workers:   l17 = UNIQUE | NON_NULL, (empty)
        // 624: workers: typeof(_10 = move _11 as *const Worker (Pointer(MutToConstPointer))) = *const {l365} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 624: workers:   l365 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 625: nworkers: typeof(_14) = *mut {l20} i32
        // 625: nworkers:   l20 = UNIQUE | NON_NULL, (empty)
    );
    lglsetime(
        lgl,
        // 628: lgl: typeof(_16) = *mut {l23} LGL
        // 628: lgl:   l23 = UNIQUE | NON_NULL, (empty)
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn() -> libc::c_double>,
            Option<unsafe extern "C" fn() -> libc::c_double>,
        >(Some(::core::mem::transmute::<
            unsafe extern "C" fn() -> libc::c_double,
            unsafe extern "C" fn() -> libc::c_double,
        >(getime))),
    );
    lglseterm(
        lgl,
        // 638: lgl: typeof(_22) = *mut {l30} LGL
        // 638: lgl:   l30 = UNIQUE | NON_NULL, (empty)
        Some(term as unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int),
        // 639: Some(term as un ... _int): typeof(_23) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 639: Some(term as un ... _int):   l32 = UNIQUE | NON_NULL, (empty)
        // 639: term as unsafe  ... c_int: typeof(_24) = fn(*mut {l34} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
        // 639: term as unsafe  ... c_int:   l34 = UNIQUE | NON_NULL, (empty)
        // 639: Some(term as un ... _int): typeof(_23 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> i32>::Some(move _24)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l367} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
        // 639: Some(term as un ... _int):   l367 = UNIQUE | NON_NULL, (empty)
        // 639: term: typeof(_24 = term as unsafe extern "C" fn(*mut libc::c_void) -> i32 (Pointer(ReifyFnPointer))) = fn(*mut {l366} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
        // 639: term:   l366 = UNIQUE | NON_NULL, (empty)
        w as *mut libc::c_void,
        // 640: w as *mut libc: ... _void: typeof(_25) = *mut {l36} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 640: w as *mut libc: ... _void:   l36 = UNIQUE | NON_NULL, (empty)
        // 640: w: typeof(_26) = *mut {l38} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 640: w:   l38 = UNIQUE | NON_NULL, (empty)
        // 640: w as *mut libc: ... _void: typeof(_25 = move _26 as *mut libc::c_void (Misc)) = *mut {l368} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 640: w as *mut libc: ... _void:   l368 = UNIQUE | NON_NULL, (empty)
    );
    lglsetmsglock(
        lgl,
        // 643: lgl: typeof(_28) = *mut {l41} LGL
        // 643: lgl:   l41 = UNIQUE | NON_NULL, (empty)
        Some(msglock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        // 644: Some(msglock as ... > ()): typeof(_29) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l43} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 644: Some(msglock as ... > ()):   l43 = UNIQUE | NON_NULL, (empty)
        // 644: msglock as unsa ... -> (): typeof(_30) = fn(*mut {l45} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 644: msglock as unsa ... -> ():   l45 = UNIQUE | NON_NULL, (empty)
        // 644: Some(msglock as ... > ()): typeof(_29 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void)>::Some(move _30)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l370} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 644: Some(msglock as ... > ()):   l370 = UNIQUE | NON_NULL, (empty)
        // 644: msglock: typeof(_30 = msglock as unsafe extern "C" fn(*mut libc::c_void) (Pointer(ReifyFnPointer))) = fn(*mut {l369} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 644: msglock:   l369 = UNIQUE | NON_NULL, (empty)
        Some(msgunlock as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        // 645: Some(msgunlock  ... > ()): typeof(_31) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l47} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 645: Some(msgunlock  ... > ()):   l47 = UNIQUE | NON_NULL, (empty)
        // 645: msgunlock as un ... -> (): typeof(_32) = fn(*mut {l49} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 645: msgunlock as un ... -> ():   l49 = UNIQUE | NON_NULL, (empty)
        // 645: Some(msgunlock  ... > ()): typeof(_31 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void)>::Some(move _32)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l372} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 645: Some(msgunlock  ... > ()):   l372 = UNIQUE | NON_NULL, (empty)
        // 645: msgunlock: typeof(_32 = msgunlock as unsafe extern "C" fn(*mut libc::c_void) (Pointer(ReifyFnPointer))) = fn(*mut {l371} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 645: msgunlock:   l371 = UNIQUE | NON_NULL, (empty)
        w as *mut libc::c_void,
        // 646: w as *mut libc: ... _void: typeof(_33) = *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 646: w as *mut libc: ... _void:   l51 = UNIQUE | NON_NULL, (empty)
        // 646: w: typeof(_34) = *mut {l53} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 646: w:   l53 = UNIQUE | NON_NULL, (empty)
        // 646: w as *mut libc: ... _void: typeof(_33 = move _34 as *mut libc::c_void (Misc)) = *mut {l373} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 646: w as *mut libc: ... _void:   l373 = UNIQUE | NON_NULL, (empty)
    );
    if opts == 0 {
        return;
    }
    if verbose != 0 {
    // 651: verbose: typeof(_42) = *mut {l62} i32
    // 651: verbose:   l62 = UNIQUE | NON_NULL, (empty)
        lglsetopt(
            lgl,
            // 653: lgl: typeof(_44) = *mut {l65} LGL
            // 653: lgl:   l65 = UNIQUE | NON_NULL, (empty)
            b"verbose\0" as *const u8 as *const libc::c_char,
            // 654: b"verbose\0" as ... _char: typeof(_45) = *const {l67} i8
            // 654: b"verbose\0" as ... _char:   l67 = UNIQUE | NON_NULL, (empty)
            // 654: b"verbose\0" as ... st u8: typeof(_46) = *const {l69} u8
            // 654: b"verbose\0" as ... st u8:   l69 = UNIQUE | NON_NULL, (empty)
            // 654: b"verbose\0": typeof(_47) = *const {l71} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 654: b"verbose\0":   l71 = UNIQUE | NON_NULL, (empty)
            // 654: b"verbose\0": typeof(_48) = & {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 654: b"verbose\0":   l73 = UNIQUE | NON_NULL, FIXED
            // 654: b"verbose\0" as ... _char: typeof(_45 = move _46 as *const i8 (Misc)) = *const {l377} i8
            // 654: b"verbose\0" as ... _char:   l377 = UNIQUE | NON_NULL, (empty)
            // 654: b"verbose\0": typeof(_48 = const b"verbose\x00") = & {l374} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 654: b"verbose\0":   l374 = UNIQUE | NON_NULL, (empty)
            // 654: b"verbose\0" as ... st u8: typeof(_46 = move _47 as *const u8 (Pointer(ArrayToPointer))) = *const {l376} u8
            // 654: b"verbose\0" as ... st u8:   l376 = UNIQUE | NON_NULL, (empty)
            // 654: b"verbose\0": typeof(_47 = &raw const (*_48)) = *const {l375} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 654: b"verbose\0":   l375 = UNIQUE | NON_NULL, (empty)
            verbose - 1 as libc::c_int,
            // 655: verbose: typeof(_51) = *mut {l77} i32
            // 655: verbose:   l77 = UNIQUE | NON_NULL, (empty)
        );
    }
    if plain != 0 {
    // 658: plain: typeof(_57) = *mut {l84} i32
    // 658: plain:   l84 = UNIQUE | NON_NULL, (empty)
        lglsetopt(
            lgl,
            // 660: lgl: typeof(_59) = *mut {l87} LGL
            // 660: lgl:   l87 = UNIQUE | NON_NULL, (empty)
            b"plain\0" as *const u8 as *const libc::c_char,
            // 661: b"plain\0" as * ... _char: typeof(_60) = *const {l89} i8
            // 661: b"plain\0" as * ... _char:   l89 = UNIQUE | NON_NULL, (empty)
            // 661: b"plain\0" as * ... st u8: typeof(_61) = *const {l91} u8
            // 661: b"plain\0" as * ... st u8:   l91 = UNIQUE | NON_NULL, (empty)
            // 661: b"plain\0": typeof(_62) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 661: b"plain\0":   l93 = UNIQUE | NON_NULL, (empty)
            // 661: b"plain\0": typeof(_63) = & {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 661: b"plain\0":   l95 = UNIQUE | NON_NULL, FIXED
            // 661: b"plain\0" as * ... _char: typeof(_60 = move _61 as *const i8 (Misc)) = *const {l381} i8
            // 661: b"plain\0" as * ... _char:   l381 = UNIQUE | NON_NULL, (empty)
            // 661: b"plain\0" as * ... st u8: typeof(_61 = move _62 as *const u8 (Pointer(ArrayToPointer))) = *const {l380} u8
            // 661: b"plain\0" as * ... st u8:   l380 = UNIQUE | NON_NULL, (empty)
            // 661: b"plain\0": typeof(_63 = const b"plain\x00") = & {l378} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 661: b"plain\0":   l378 = UNIQUE | NON_NULL, (empty)
            // 661: b"plain\0": typeof(_62 = &raw const (*_63)) = *const {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 661: b"plain\0":   l379 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
        );
    }
    lglsetopt(
        lgl,
        // 666: lgl: typeof(_66) = *mut {l99} LGL
        // 666: lgl:   l99 = UNIQUE | NON_NULL, (empty)
        b"reduceinc\0" as *const u8 as *const libc::c_char,
        // 667: b"reduceinc\0"  ... _char: typeof(_67) = *const {l101} i8
        // 667: b"reduceinc\0"  ... _char:   l101 = UNIQUE | NON_NULL, (empty)
        // 667: b"reduceinc\0"  ... st u8: typeof(_68) = *const {l103} u8
        // 667: b"reduceinc\0"  ... st u8:   l103 = UNIQUE | NON_NULL, (empty)
        // 667: b"reduceinc\0": typeof(_69) = *const {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 667: b"reduceinc\0":   l105 = UNIQUE | NON_NULL, (empty)
        // 667: b"reduceinc\0": typeof(_70) = & {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 667: b"reduceinc\0":   l107 = UNIQUE | NON_NULL, FIXED
        // 667: b"reduceinc\0": typeof(_69 = &raw const (*_70)) = *const {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 667: b"reduceinc\0":   l383 = UNIQUE | NON_NULL, (empty)
        // 667: b"reduceinc\0"  ... _char: typeof(_67 = move _68 as *const i8 (Misc)) = *const {l385} i8
        // 667: b"reduceinc\0"  ... _char:   l385 = UNIQUE | NON_NULL, (empty)
        // 667: b"reduceinc\0": typeof(_70 = const b"reduceinc\x00") = & {l382} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 667: b"reduceinc\0":   l382 = UNIQUE | NON_NULL, (empty)
        // 667: b"reduceinc\0"  ... st u8: typeof(_68 = move _69 as *const u8 (Pointer(ArrayToPointer))) = *const {l384} u8
        // 667: b"reduceinc\0"  ... st u8:   l384 = UNIQUE | NON_NULL, (empty)
        1000 as libc::c_int,
    );
    lglsetopt(
        lgl,
        // 671: lgl: typeof(_73) = *mut {l111} LGL
        // 671: lgl:   l111 = UNIQUE | NON_NULL, (empty)
        b"reduceinit\0" as *const u8 as *const libc::c_char,
        // 672: b"reduceinit\0" ... _char: typeof(_74) = *const {l113} i8
        // 672: b"reduceinit\0" ... _char:   l113 = UNIQUE | NON_NULL, (empty)
        // 672: b"reduceinit\0" ... st u8: typeof(_75) = *const {l115} u8
        // 672: b"reduceinit\0" ... st u8:   l115 = UNIQUE | NON_NULL, (empty)
        // 672: b"reduceinit\0": typeof(_76) = *const {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 672: b"reduceinit\0":   l117 = UNIQUE | NON_NULL, (empty)
        // 672: b"reduceinit\0": typeof(_77) = & {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 672: b"reduceinit\0":   l119 = UNIQUE | NON_NULL, FIXED
        // 672: b"reduceinit\0": typeof(_77 = const b"reduceinit\x00") = & {l386} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 672: b"reduceinit\0":   l386 = UNIQUE | NON_NULL, (empty)
        // 672: b"reduceinit\0": typeof(_76 = &raw const (*_77)) = *const {l387} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 672: b"reduceinit\0":   l387 = UNIQUE | NON_NULL, (empty)
        // 672: b"reduceinit\0" ... st u8: typeof(_75 = move _76 as *const u8 (Pointer(ArrayToPointer))) = *const {l388} u8
        // 672: b"reduceinit\0" ... st u8:   l388 = UNIQUE | NON_NULL, (empty)
        // 672: b"reduceinit\0" ... _char: typeof(_74 = move _75 as *const i8 (Misc)) = *const {l389} i8
        // 672: b"reduceinit\0" ... _char:   l389 = UNIQUE | NON_NULL, (empty)
        1000 as libc::c_int,
    );
    lglsetopt(
        lgl,
        // 676: lgl: typeof(_80) = *mut {l123} LGL
        // 676: lgl:   l123 = UNIQUE | NON_NULL, (empty)
        b"reusetrail\0" as *const u8 as *const libc::c_char,
        // 677: b"reusetrail\0" ... _char: typeof(_81) = *const {l125} i8
        // 677: b"reusetrail\0" ... _char:   l125 = UNIQUE | NON_NULL, (empty)
        // 677: b"reusetrail\0" ... st u8: typeof(_82) = *const {l127} u8
        // 677: b"reusetrail\0" ... st u8:   l127 = UNIQUE | NON_NULL, (empty)
        // 677: b"reusetrail\0": typeof(_83) = *const {l129} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 677: b"reusetrail\0":   l129 = UNIQUE | NON_NULL, (empty)
        // 677: b"reusetrail\0": typeof(_84) = & {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 677: b"reusetrail\0":   l131 = UNIQUE | NON_NULL, FIXED
        // 677: b"reusetrail\0" ... st u8: typeof(_82 = move _83 as *const u8 (Pointer(ArrayToPointer))) = *const {l392} u8
        // 677: b"reusetrail\0" ... st u8:   l392 = UNIQUE | NON_NULL, (empty)
        // 677: b"reusetrail\0": typeof(_84 = const b"reusetrail\x00") = & {l390} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 677: b"reusetrail\0":   l390 = UNIQUE | NON_NULL, (empty)
        // 677: b"reusetrail\0": typeof(_83 = &raw const (*_84)) = *const {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 677: b"reusetrail\0":   l391 = UNIQUE | NON_NULL, (empty)
        // 677: b"reusetrail\0" ... _char: typeof(_81 = move _82 as *const i8 (Misc)) = *const {l393} i8
        // 677: b"reusetrail\0" ... _char:   l393 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
    );
    lglsetopt(
        lgl,
        // 681: lgl: typeof(_87) = *mut {l135} LGL
        // 681: lgl:   l135 = UNIQUE | NON_NULL, (empty)
        b"gluekeep\0" as *const u8 as *const libc::c_char,
        // 682: b"gluekeep\0" a ... _char: typeof(_88) = *const {l137} i8
        // 682: b"gluekeep\0" a ... _char:   l137 = UNIQUE | NON_NULL, (empty)
        // 682: b"gluekeep\0" a ... st u8: typeof(_89) = *const {l139} u8
        // 682: b"gluekeep\0" a ... st u8:   l139 = UNIQUE | NON_NULL, (empty)
        // 682: b"gluekeep\0": typeof(_90) = *const {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 682: b"gluekeep\0":   l141 = UNIQUE | NON_NULL, (empty)
        // 682: b"gluekeep\0": typeof(_91) = & {l143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 682: b"gluekeep\0":   l143 = UNIQUE | NON_NULL, FIXED
        // 682: b"gluekeep\0": typeof(_91 = const b"gluekeep\x00") = & {l394} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 682: b"gluekeep\0":   l394 = UNIQUE | NON_NULL, (empty)
        // 682: b"gluekeep\0" a ... _char: typeof(_88 = move _89 as *const i8 (Misc)) = *const {l397} i8
        // 682: b"gluekeep\0" a ... _char:   l397 = UNIQUE | NON_NULL, (empty)
        // 682: b"gluekeep\0" a ... st u8: typeof(_89 = move _90 as *const u8 (Pointer(ArrayToPointer))) = *const {l396} u8
        // 682: b"gluekeep\0" a ... st u8:   l396 = UNIQUE | NON_NULL, (empty)
        // 682: b"gluekeep\0": typeof(_90 = &raw const (*_91)) = *const {l395} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 682: b"gluekeep\0":   l395 = UNIQUE | NON_NULL, (empty)
        3 as libc::c_int,
    );
    lglsetopt(
        lgl,
        // 686: lgl: typeof(_94) = *mut {l147} LGL
        // 686: lgl:   l147 = UNIQUE | NON_NULL, (empty)
        b"scincinc\0" as *const u8 as *const libc::c_char,
        // 687: b"scincinc\0" a ... _char: typeof(_95) = *const {l149} i8
        // 687: b"scincinc\0" a ... _char:   l149 = UNIQUE | NON_NULL, (empty)
        // 687: b"scincinc\0" a ... st u8: typeof(_96) = *const {l151} u8
        // 687: b"scincinc\0" a ... st u8:   l151 = UNIQUE | NON_NULL, (empty)
        // 687: b"scincinc\0": typeof(_97) = *const {l153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 687: b"scincinc\0":   l153 = UNIQUE | NON_NULL, (empty)
        // 687: b"scincinc\0": typeof(_98) = & {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 687: b"scincinc\0":   l155 = UNIQUE | NON_NULL, FIXED
        // 687: b"scincinc\0": typeof(_97 = &raw const (*_98)) = *const {l399} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 687: b"scincinc\0":   l399 = UNIQUE | NON_NULL, (empty)
        // 687: b"scincinc\0": typeof(_98 = const b"scincinc\x00") = & {l398} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 687: b"scincinc\0":   l398 = UNIQUE | NON_NULL, (empty)
        // 687: b"scincinc\0" a ... st u8: typeof(_96 = move _97 as *const u8 (Pointer(ArrayToPointer))) = *const {l400} u8
        // 687: b"scincinc\0" a ... st u8:   l400 = UNIQUE | NON_NULL, (empty)
        // 687: b"scincinc\0" a ... _char: typeof(_95 = move _96 as *const i8 (Misc)) = *const {l401} i8
        // 687: b"scincinc\0" a ... _char:   l401 = UNIQUE | NON_NULL, (empty)
        50 as libc::c_int,
    );
    lglsetopt(
        lgl,
        // 691: lgl: typeof(_101) = *mut {l159} LGL
        // 691: lgl:   l159 = UNIQUE | NON_NULL, (empty)
        b"scincincmode\0" as *const u8 as *const libc::c_char,
        // 692: b"scincincmode\ ... _char: typeof(_102) = *const {l161} i8
        // 692: b"scincincmode\ ... _char:   l161 = UNIQUE | NON_NULL, (empty)
        // 692: b"scincincmode\ ... st u8: typeof(_103) = *const {l163} u8
        // 692: b"scincincmode\ ... st u8:   l163 = UNIQUE | NON_NULL, (empty)
        // 692: b"scincincmode\0": typeof(_104) = *const {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 692: b"scincincmode\0":   l165 = UNIQUE | NON_NULL, (empty)
        // 692: b"scincincmode\0": typeof(_105) = & {l167} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 692: b"scincincmode\0":   l167 = UNIQUE | NON_NULL, FIXED
        // 692: b"scincincmode\ ... st u8: typeof(_103 = move _104 as *const u8 (Pointer(ArrayToPointer))) = *const {l404} u8
        // 692: b"scincincmode\ ... st u8:   l404 = UNIQUE | NON_NULL, (empty)
        // 692: b"scincincmode\0": typeof(_104 = &raw const (*_105)) = *const {l403} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 692: b"scincincmode\0":   l403 = UNIQUE | NON_NULL, (empty)
        // 692: b"scincincmode\0": typeof(_105 = const b"scincincmode\x00") = & {l402} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 692: b"scincincmode\0":   l402 = UNIQUE | NON_NULL, (empty)
        // 692: b"scincincmode\ ... _char: typeof(_102 = move _103 as *const i8 (Misc)) = *const {l405} i8
        // 692: b"scincincmode\ ... _char:   l405 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
    );
    if !druptraceprefix.is_null() {
    // 695: druptraceprefix: typeof(_110) = *const {l173} i8
    // 695: druptraceprefix:   l173 = UNIQUE | NON_NULL, (empty)
    // 695: druptraceprefix: typeof(_111) = *mut {l175} *const {l176} i8
    // 695: druptraceprefix:   l175 = UNIQUE | NON_NULL, (empty)
    // 695: druptraceprefix:   l176 = UNIQUE | NON_NULL, (empty)
        let mut name: *mut libc::c_char =
        // 696: mut name: typeof(_112) = *mut {l178} i8
        // 696: mut name:   l178 = UNIQUE | NON_NULL, (empty)
            malloc((strlen(druptraceprefix)).wrapping_add(30 as libc::c_int as libc::c_ulong))
            // 697: malloc((strlen( ... ong)): typeof(_113) = *mut {l180} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 697: malloc((strlen( ... ong)):   l180 = UNIQUE | NON_NULL, (empty)
            // 697: druptraceprefix: typeof(_116) = *const {l184} i8
            // 697: druptraceprefix:   l184 = UNIQUE | NON_NULL, (empty)
            // 697: druptraceprefix: typeof(_117) = *mut {l186} *const {l187} i8
            // 697: druptraceprefix:   l186 = UNIQUE | NON_NULL, (empty)
            // 697: druptraceprefix:   l187 = UNIQUE | NON_NULL, (empty)
            // 697: malloc((strlen( ... _char: typeof(_112 = move _113 as *mut i8 (Misc)) = *mut {l406} i8
            // 697: malloc((strlen( ... _char:   l406 = UNIQUE | NON_NULL, (empty)
                as *mut libc::c_char;
        sprintf(
            name,
            // 700: name: typeof(_121) = *mut {l192} i8
            // 700: name:   l192 = UNIQUE | NON_NULL, (empty)
            b"%s%d.proof\0" as *const u8 as *const libc::c_char,
            // 701: b"%s%d.proof\0" ... _char: typeof(_122) = *const {l194} i8
            // 701: b"%s%d.proof\0" ... _char:   l194 = UNIQUE | NON_NULL, (empty)
            // 701: b"%s%d.proof\0" ... st u8: typeof(_123) = *const {l196} u8
            // 701: b"%s%d.proof\0" ... st u8:   l196 = UNIQUE | NON_NULL, (empty)
            // 701: b"%s%d.proof\0": typeof(_124) = *const {l198} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 701: b"%s%d.proof\0":   l198 = UNIQUE | NON_NULL, (empty)
            // 701: b"%s%d.proof\0": typeof(_125) = & {l200} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 701: b"%s%d.proof\0":   l200 = UNIQUE | NON_NULL, FIXED
            // 701: b"%s%d.proof\0": typeof(_124 = &raw const (*_125)) = *const {l408} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 701: b"%s%d.proof\0":   l408 = UNIQUE | NON_NULL, (empty)
            // 701: b"%s%d.proof\0": typeof(_125 = const b"%s%d.proof\x00") = & {l407} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 701: b"%s%d.proof\0":   l407 = UNIQUE | NON_NULL, (empty)
            // 701: b"%s%d.proof\0" ... _char: typeof(_122 = move _123 as *const i8 (Misc)) = *const {l410} i8
            // 701: b"%s%d.proof\0" ... _char:   l410 = UNIQUE | NON_NULL, (empty)
            // 701: b"%s%d.proof\0" ... st u8: typeof(_123 = move _124 as *const u8 (Pointer(ArrayToPointer))) = *const {l409} u8
            // 701: b"%s%d.proof\0" ... st u8:   l409 = UNIQUE | NON_NULL, (empty)
            druptraceprefix,
            // 702: druptraceprefix: typeof(_126) = *const {l202} i8
            // 702: druptraceprefix:   l202 = UNIQUE | NON_NULL, (empty)
            // 702: druptraceprefix: typeof(_127) = *mut {l204} *const {l205} i8
            // 702: druptraceprefix:   l204 = UNIQUE | NON_NULL, (empty)
            // 702: druptraceprefix:   l205 = UNIQUE | NON_NULL, (empty)
            w.offset_from(workers) as libc::c_long as libc::c_int,
            // 703: w: typeof(_131) = *mut {l210} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 703: w:   l210 = UNIQUE | NON_NULL, (empty)
            // 703: workers: typeof(_132) = *const {l212} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 703: workers:   l212 = UNIQUE | NON_NULL, (empty)
            // 703: workers: typeof(_133) = *mut {l214} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 703: workers:   l214 = UNIQUE | NON_NULL, (empty)
            // 703: workers: typeof(_134) = *mut {l216} *mut {l217} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 703: workers:   l216 = UNIQUE | NON_NULL, (empty)
            // 703: workers:   l217 = UNIQUE | NON_NULL, (empty)
            // 703: workers: typeof(_132 = move _133 as *const Worker (Pointer(MutToConstPointer))) = *const {l411} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 703: workers:   l411 = UNIQUE | NON_NULL, (empty)
        );
        (*w).proof = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        // 705: fopen(name, b"w ... char): typeof(_135) = *mut {l219} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 705: fopen(name, b"w ... char):   l219 = UNIQUE | NON_NULL, (empty)
        // 705: name: typeof(_136) = *const {l221} i8
        // 705: name:   l221 = UNIQUE | NON_NULL, (empty)
        // 705: name: typeof(_137) = *mut {l223} i8
        // 705: name:   l223 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0" as *cons ... _char: typeof(_138) = *const {l225} i8
        // 705: b"w\0" as *cons ... _char:   l225 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0" as *const u8: typeof(_139) = *const {l227} u8
        // 705: b"w\0" as *const u8:   l227 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0": typeof(_140) = *const {l229} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 705: b"w\0":   l229 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0": typeof(_141) = & {l231} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 705: b"w\0":   l231 = UNIQUE | NON_NULL, FIXED
        // 705: b"w\0": typeof(_141 = const b"w\x00") = & {l413} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 705: b"w\0":   l413 = UNIQUE | NON_NULL, (empty)
        // 705: name: typeof(_136 = move _137 as *const i8 (Pointer(MutToConstPointer))) = *const {l412} i8
        // 705: name:   l412 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0": typeof(_140 = &raw const (*_141)) = *const {l414} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 705: b"w\0":   l414 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0" as *const u8: typeof(_139 = move _140 as *const u8 (Pointer(ArrayToPointer))) = *const {l415} u8
        // 705: b"w\0" as *const u8:   l415 = UNIQUE | NON_NULL, (empty)
        // 705: b"w\0" as *cons ... _char: typeof(_138 = move _139 as *const i8 (Misc)) = *const {l416} i8
        // 705: b"w\0" as *cons ... _char:   l416 = UNIQUE | NON_NULL, (empty)
        if ((*w).proof).is_null() {
        // 706: ((*w).proof): typeof(_144) = *mut {l235} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 706: ((*w).proof):   l235 = UNIQUE | NON_NULL, (empty)
            die(
                b"worker %d can not write DRUP proof to '%s'\0" as *const u8 as *const libc::c_char,
                // 708: b"worker %d can ... _char: typeof(_146) = *const {l238} i8
                // 708: b"worker %d can ... _char:   l238 = UNIQUE | NON_NULL, (empty)
                // 708: b"worker %d can ... st u8: typeof(_147) = *const {l240} u8
                // 708: b"worker %d can ... st u8:   l240 = UNIQUE | NON_NULL, (empty)
                // 708: b"worker %d can ... s'\0": typeof(_148) = *const {l242} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 708: b"worker %d can ... s'\0":   l242 = UNIQUE | NON_NULL, (empty)
                // 708: b"worker %d can ... s'\0": typeof(_149) = & {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 708: b"worker %d can ... s'\0":   l244 = UNIQUE | NON_NULL, FIXED
                // 708: b"worker %d can ... s'\0": typeof(_148 = &raw const (*_149)) = *const {l418} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 708: b"worker %d can ... s'\0":   l418 = UNIQUE | NON_NULL, (empty)
                // 708: b"worker %d can ... s'\0": typeof(_149 = const b"worker %d can not write DRUP proof to \'%s\'\x00") = & {l417} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                // 708: b"worker %d can ... s'\0":   l417 = UNIQUE | NON_NULL, (empty)
                // 708: b"worker %d can ... st u8: typeof(_147 = move _148 as *const u8 (Pointer(ArrayToPointer))) = *const {l419} u8
                // 708: b"worker %d can ... st u8:   l419 = UNIQUE | NON_NULL, (empty)
                // 708: b"worker %d can ... _char: typeof(_146 = move _147 as *const i8 (Misc)) = *const {l420} i8
                // 708: b"worker %d can ... _char:   l420 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 709: w: typeof(_153) = *mut {l249} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 709: w:   l249 = UNIQUE | NON_NULL, (empty)
                // 709: workers: typeof(_154) = *const {l251} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 709: workers:   l251 = UNIQUE | NON_NULL, (empty)
                // 709: workers: typeof(_155) = *mut {l253} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 709: workers:   l253 = UNIQUE | NON_NULL, (empty)
                // 709: workers: typeof(_156) = *mut {l255} *mut {l256} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 709: workers:   l255 = UNIQUE | NON_NULL, (empty)
                // 709: workers:   l256 = UNIQUE | NON_NULL, (empty)
                // 709: workers: typeof(_154 = move _155 as *const Worker (Pointer(MutToConstPointer))) = *const {l421} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 709: workers:   l421 = UNIQUE | NON_NULL, (empty)
                name,
                // 710: name: typeof(_157) = *mut {l258} i8
                // 710: name:   l258 = UNIQUE | NON_NULL, (empty)
            );
        }
        lglsetout(lgl, (*w).proof);
        // 713: lgl: typeof(_159) = *mut {l261} LGL
        // 713: lgl:   l261 = UNIQUE | NON_NULL, (empty)
        // 713: (*w).proof: typeof(_160) = *mut {l263} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 713: (*w).proof:   l263 = UNIQUE | NON_NULL, (empty)
        sprintf(
            name,
            // 715: name: typeof(_162) = *mut {l266} i8
            // 715: name:   l266 = UNIQUE | NON_NULL, (empty)
            b"%s%d.cnf\0" as *const u8 as *const libc::c_char,
            // 716: b"%s%d.cnf\0" a ... _char: typeof(_163) = *const {l268} i8
            // 716: b"%s%d.cnf\0" a ... _char:   l268 = UNIQUE | NON_NULL, (empty)
            // 716: b"%s%d.cnf\0" a ... st u8: typeof(_164) = *const {l270} u8
            // 716: b"%s%d.cnf\0" a ... st u8:   l270 = UNIQUE | NON_NULL, (empty)
            // 716: b"%s%d.cnf\0": typeof(_165) = *const {l272} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 716: b"%s%d.cnf\0":   l272 = UNIQUE | NON_NULL, (empty)
            // 716: b"%s%d.cnf\0": typeof(_166) = & {l274} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 716: b"%s%d.cnf\0":   l274 = UNIQUE | NON_NULL, FIXED
            // 716: b"%s%d.cnf\0" a ... st u8: typeof(_164 = move _165 as *const u8 (Pointer(ArrayToPointer))) = *const {l424} u8
            // 716: b"%s%d.cnf\0" a ... st u8:   l424 = UNIQUE | NON_NULL, (empty)
            // 716: b"%s%d.cnf\0" a ... _char: typeof(_163 = move _164 as *const i8 (Misc)) = *const {l425} i8
            // 716: b"%s%d.cnf\0" a ... _char:   l425 = UNIQUE | NON_NULL, (empty)
            // 716: b"%s%d.cnf\0": typeof(_165 = &raw const (*_166)) = *const {l423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 716: b"%s%d.cnf\0":   l423 = UNIQUE | NON_NULL, (empty)
            // 716: b"%s%d.cnf\0": typeof(_166 = const b"%s%d.cnf\x00") = & {l422} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 716: b"%s%d.cnf\0":   l422 = UNIQUE | NON_NULL, (empty)
            druptraceprefix,
            // 717: druptraceprefix: typeof(_167) = *const {l276} i8
            // 717: druptraceprefix:   l276 = UNIQUE | NON_NULL, (empty)
            // 717: druptraceprefix: typeof(_168) = *mut {l278} *const {l279} i8
            // 717: druptraceprefix:   l278 = UNIQUE | NON_NULL, (empty)
            // 717: druptraceprefix:   l279 = UNIQUE | NON_NULL, (empty)
            w.offset_from(workers) as libc::c_long as libc::c_int,
            // 718: w: typeof(_172) = *mut {l284} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 718: w:   l284 = UNIQUE | NON_NULL, (empty)
            // 718: workers: typeof(_173) = *const {l286} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 718: workers:   l286 = UNIQUE | NON_NULL, (empty)
            // 718: workers: typeof(_174) = *mut {l288} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 718: workers:   l288 = UNIQUE | NON_NULL, (empty)
            // 718: workers: typeof(_175) = *mut {l290} *mut {l291} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 718: workers:   l290 = UNIQUE | NON_NULL, (empty)
            // 718: workers:   l291 = UNIQUE | NON_NULL, (empty)
            // 718: workers: typeof(_173 = move _174 as *const Worker (Pointer(MutToConstPointer))) = *const {l426} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 718: workers:   l426 = UNIQUE | NON_NULL, (empty)
        );
        (*w).post = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
        // 720: fopen(name, b"w ... char): typeof(_176) = *mut {l293} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 720: fopen(name, b"w ... char):   l293 = UNIQUE | NON_NULL, (empty)
        // 720: name: typeof(_177) = *const {l295} i8
        // 720: name:   l295 = UNIQUE | NON_NULL, (empty)
        // 720: name: typeof(_178) = *mut {l297} i8
        // 720: name:   l297 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0" as *cons ... _char: typeof(_179) = *const {l299} i8
        // 720: b"w\0" as *cons ... _char:   l299 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0" as *const u8: typeof(_180) = *const {l301} u8
        // 720: b"w\0" as *const u8:   l301 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0": typeof(_181) = *const {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 720: b"w\0":   l303 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0": typeof(_182) = & {l305} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 720: b"w\0":   l305 = UNIQUE | NON_NULL, FIXED
        // 720: name: typeof(_177 = move _178 as *const i8 (Pointer(MutToConstPointer))) = *const {l427} i8
        // 720: name:   l427 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0" as *const u8: typeof(_180 = move _181 as *const u8 (Pointer(ArrayToPointer))) = *const {l430} u8
        // 720: b"w\0" as *const u8:   l430 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0": typeof(_181 = &raw const (*_182)) = *const {l429} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 720: b"w\0":   l429 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0": typeof(_182 = const b"w\x00") = & {l428} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 720: b"w\0":   l428 = UNIQUE | NON_NULL, (empty)
        // 720: b"w\0" as *cons ... _char: typeof(_179 = move _180 as *const i8 (Misc)) = *const {l431} i8
        // 720: b"w\0" as *cons ... _char:   l431 = UNIQUE | NON_NULL, (empty)
        if ((*w).post).is_null() {
        // 721: ((*w).post): typeof(_185) = *mut {l309} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 721: ((*w).post):   l309 = UNIQUE | NON_NULL, (empty)
            die(
                b"worker %d can not write post CNF cubes to '%s'\0" as *const u8
                // 723: b"worker %d can ... _char: typeof(_187) = *const {l312} i8
                // 723: b"worker %d can ... _char:   l312 = UNIQUE | NON_NULL, (empty)
                // 723: b"worker %d can ... st u8: typeof(_188) = *const {l314} u8
                // 723: b"worker %d can ... st u8:   l314 = UNIQUE | NON_NULL, (empty)
                // 723: b"worker %d can ... s'\0": typeof(_189) = *const {l316} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 723: b"worker %d can ... s'\0":   l316 = UNIQUE | NON_NULL, (empty)
                // 723: b"worker %d can ... s'\0": typeof(_190) = & {l318} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 723: b"worker %d can ... s'\0":   l318 = UNIQUE | NON_NULL, FIXED
                // 723: b"worker %d can ... _char: typeof(_187 = move _188 as *const i8 (Misc)) = *const {l435} i8
                // 723: b"worker %d can ... _char:   l435 = UNIQUE | NON_NULL, (empty)
                // 723: b"worker %d can ... st u8: typeof(_188 = move _189 as *const u8 (Pointer(ArrayToPointer))) = *const {l434} u8
                // 723: b"worker %d can ... st u8:   l434 = UNIQUE | NON_NULL, (empty)
                // 723: b"worker %d can ... s'\0": typeof(_190 = const b"worker %d can not write post CNF cubes to \'%s\'\x00") = & {l432} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 723: b"worker %d can ... s'\0":   l432 = UNIQUE | NON_NULL, (empty)
                // 723: b"worker %d can ... s'\0": typeof(_189 = &raw const (*_190)) = *const {l433} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
                // 723: b"worker %d can ... s'\0":   l433 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 725: w: typeof(_194) = *mut {l323} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 725: w:   l323 = UNIQUE | NON_NULL, (empty)
                // 725: workers: typeof(_195) = *const {l325} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 725: workers:   l325 = UNIQUE | NON_NULL, (empty)
                // 725: workers: typeof(_196) = *mut {l327} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 725: workers:   l327 = UNIQUE | NON_NULL, (empty)
                // 725: workers: typeof(_197) = *mut {l329} *mut {l330} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 725: workers:   l329 = UNIQUE | NON_NULL, (empty)
                // 725: workers:   l330 = UNIQUE | NON_NULL, (empty)
                // 725: workers: typeof(_195 = move _196 as *const Worker (Pointer(MutToConstPointer))) = *const {l436} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 725: workers:   l436 = UNIQUE | NON_NULL, (empty)
                name,
                // 726: name: typeof(_198) = *mut {l332} i8
                // 726: name:   l332 = UNIQUE | NON_NULL, (empty)
            );
        }
        free(name as *mut libc::c_void);
        // 729: name as *mut li ... _void: typeof(_200) = *mut {l335} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 729: name as *mut li ... _void:   l335 = UNIQUE | NON_NULL, (empty)
        // 729: name: typeof(_201) = *mut {l337} i8
        // 729: name:   l337 = UNIQUE | NON_NULL, (empty)
        // 729: name as *mut li ... _void: typeof(_200 = move _201 as *mut libc::c_void (Misc)) = *mut {l437} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 729: name as *mut li ... _void:   l437 = UNIQUE | NON_NULL, (empty)
        lglsetopt(
            lgl,
            // 731: lgl: typeof(_203) = *mut {l340} LGL
            // 731: lgl:   l340 = UNIQUE | NON_NULL, (empty)
            b"druplig\0" as *const u8 as *const libc::c_char,
            // 732: b"druplig\0" as ... _char: typeof(_204) = *const {l342} i8
            // 732: b"druplig\0" as ... _char:   l342 = UNIQUE | NON_NULL, (empty)
            // 732: b"druplig\0" as ... st u8: typeof(_205) = *const {l344} u8
            // 732: b"druplig\0" as ... st u8:   l344 = UNIQUE | NON_NULL, (empty)
            // 732: b"druplig\0": typeof(_206) = *const {l346} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 732: b"druplig\0":   l346 = UNIQUE | NON_NULL, (empty)
            // 732: b"druplig\0": typeof(_207) = & {l348} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 732: b"druplig\0":   l348 = UNIQUE | NON_NULL, FIXED
            // 732: b"druplig\0": typeof(_206 = &raw const (*_207)) = *const {l439} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 732: b"druplig\0":   l439 = UNIQUE | NON_NULL, (empty)
            // 732: b"druplig\0": typeof(_207 = const b"druplig\x00") = & {l438} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 732: b"druplig\0":   l438 = UNIQUE | NON_NULL, (empty)
            // 732: b"druplig\0" as ... _char: typeof(_204 = move _205 as *const i8 (Misc)) = *const {l441} i8
            // 732: b"druplig\0" as ... _char:   l441 = UNIQUE | NON_NULL, (empty)
            // 732: b"druplig\0" as ... st u8: typeof(_205 = move _206 as *const u8 (Pointer(ArrayToPointer))) = *const {l440} u8
            // 732: b"druplig\0" as ... st u8:   l440 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
        );
        lglsetopt(
            lgl,
            // 736: lgl: typeof(_210) = *mut {l352} LGL
            // 736: lgl:   l352 = UNIQUE | NON_NULL, (empty)
            b"drupligtrace\0" as *const u8 as *const libc::c_char,
            // 737: b"drupligtrace\ ... _char: typeof(_211) = *const {l354} i8
            // 737: b"drupligtrace\ ... _char:   l354 = UNIQUE | NON_NULL, (empty)
            // 737: b"drupligtrace\ ... st u8: typeof(_212) = *const {l356} u8
            // 737: b"drupligtrace\ ... st u8:   l356 = UNIQUE | NON_NULL, (empty)
            // 737: b"drupligtrace\0": typeof(_213) = *const {l358} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 737: b"drupligtrace\0":   l358 = UNIQUE | NON_NULL, (empty)
            // 737: b"drupligtrace\0": typeof(_214) = & {l360} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 737: b"drupligtrace\0":   l360 = UNIQUE | NON_NULL, FIXED
            // 737: b"drupligtrace\ ... st u8: typeof(_212 = move _213 as *const u8 (Pointer(ArrayToPointer))) = *const {l444} u8
            // 737: b"drupligtrace\ ... st u8:   l444 = UNIQUE | NON_NULL, (empty)
            // 737: b"drupligtrace\0": typeof(_214 = const b"drupligtrace\x00") = & {l442} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 737: b"drupligtrace\0":   l442 = UNIQUE | NON_NULL, (empty)
            // 737: b"drupligtrace\ ... _char: typeof(_211 = move _212 as *const i8 (Misc)) = *const {l445} i8
            // 737: b"drupligtrace\ ... _char:   l445 = UNIQUE | NON_NULL, (empty)
            // 737: b"drupligtrace\0": typeof(_213 = &raw const (*_214)) = *const {l443} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 737: b"drupligtrace\0":   l443 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
        );
    } else {
        (*w).proof = 0 as *mut FILE;
        // 741: (*w).proof = 0  ...  FILE: typeof(((*_2).5: *mut _IO_FILE) = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l446} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 741: (*w).proof = 0  ...  FILE:   l446 = UNIQUE | NON_NULL, (empty)
        (*w).post = (*w).proof;
        // 742: (*w).proof: typeof(_216) = *mut {l363} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 742: (*w).proof:   l363 = UNIQUE | NON_NULL, (empty)
    };
}
unsafe extern "C" fn justreturn(mut w: *mut Worker) -> libc::c_int {
// 745: mut w: typeof(_1) = *mut {g12} DefId(0:297 ~ ilingeling[c969]::Worker)
// 745: mut w:   g12 = UNIQUE | NON_NULL, FIXED
    let mut res: libc::c_int = 0;
    if pthread_mutex_lock(&mut donemutex) != 0 {
    // 747: &mut donemutex: typeof(_7) = *mut {l7} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 747: &mut donemutex:   l7 = UNIQUE | NON_NULL, (empty)
    // 747: &mut donemutex: typeof(_8) = &mut {l9} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 747: &mut donemutex:   l9 = UNIQUE | NON_NULL, (empty)
    // 747: donemutex: typeof(_9) = *mut {l11} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 747: donemutex:   l11 = UNIQUE | NON_NULL, (empty)
    // 747: &mut donemutex: typeof(_8 = &mut (*_9)) = &mut {l67} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 747: &mut donemutex:   l67 = UNIQUE | NON_NULL, (empty)
    // 747: &mut donemutex: typeof(_7 = &raw mut (*_8)) = *mut {l68} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 747: &mut donemutex:   l68 = UNIQUE | NON_NULL, (empty)
        warn(
            b"worker %d failed to lock 'done' mutex\0" as *const u8 as *const libc::c_char,
            // 749: b"worker %d fai ... _char: typeof(_11) = *const {l14} i8
            // 749: b"worker %d fai ... _char:   l14 = UNIQUE | NON_NULL, (empty)
            // 749: b"worker %d fai ... st u8: typeof(_12) = *const {l16} u8
            // 749: b"worker %d fai ... st u8:   l16 = UNIQUE | NON_NULL, (empty)
            // 749: b"worker %d fai ... ex\0": typeof(_13) = *const {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 749: b"worker %d fai ... ex\0":   l18 = UNIQUE | NON_NULL, (empty)
            // 749: b"worker %d fai ... ex\0": typeof(_14) = & {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 749: b"worker %d fai ... ex\0":   l20 = UNIQUE | NON_NULL, FIXED
            // 749: b"worker %d fai ... st u8: typeof(_12 = move _13 as *const u8 (Pointer(ArrayToPointer))) = *const {l71} u8
            // 749: b"worker %d fai ... st u8:   l71 = UNIQUE | NON_NULL, (empty)
            // 749: b"worker %d fai ... ex\0": typeof(_13 = &raw const (*_14)) = *const {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 749: b"worker %d fai ... ex\0":   l70 = UNIQUE | NON_NULL, (empty)
            // 749: b"worker %d fai ... _char: typeof(_11 = move _12 as *const i8 (Misc)) = *const {l72} i8
            // 749: b"worker %d fai ... _char:   l72 = UNIQUE | NON_NULL, (empty)
            // 749: b"worker %d fai ... ex\0": typeof(_14 = const b"worker %d failed to lock \'done\' mutex\x00") = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 749: b"worker %d fai ... ex\0":   l69 = UNIQUE | NON_NULL, (empty)
            w.offset_from(workers) as libc::c_long as libc::c_int,
            // 750: w: typeof(_18) = *mut {l25} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 750: w:   l25 = UNIQUE | NON_NULL, (empty)
            // 750: workers: typeof(_19) = *const {l27} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 750: workers:   l27 = UNIQUE | NON_NULL, (empty)
            // 750: workers: typeof(_20) = *mut {l29} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 750: workers:   l29 = UNIQUE | NON_NULL, (empty)
            // 750: workers: typeof(_21) = *mut {l31} *mut {l32} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 750: workers:   l31 = UNIQUE | NON_NULL, (empty)
            // 750: workers:   l32 = UNIQUE | NON_NULL, (empty)
            // 750: workers: typeof(_19 = move _20 as *const Worker (Pointer(MutToConstPointer))) = *const {l73} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 750: workers:   l73 = UNIQUE | NON_NULL, (empty)
        );
    }
    res = done;
    // 753: done: typeof(_23) = *mut {l35} i32
    // 753: done:   l35 = UNIQUE | NON_NULL, (empty)
    if pthread_mutex_unlock(&mut donemutex) != 0 {
    // 754: &mut donemutex: typeof(_27) = *mut {l40} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 754: &mut donemutex:   l40 = UNIQUE | NON_NULL, (empty)
    // 754: &mut donemutex: typeof(_28) = &mut {l42} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 754: &mut donemutex:   l42 = UNIQUE | NON_NULL, (empty)
    // 754: donemutex: typeof(_29) = *mut {l44} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 754: donemutex:   l44 = UNIQUE | NON_NULL, (empty)
    // 754: &mut donemutex: typeof(_27 = &raw mut (*_28)) = *mut {l75} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 754: &mut donemutex:   l75 = UNIQUE | NON_NULL, (empty)
    // 754: &mut donemutex: typeof(_28 = &mut (*_29)) = &mut {l74} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
    // 754: &mut donemutex:   l74 = UNIQUE | NON_NULL, (empty)
        warn(
            b"worker %d failed to unlock 'done' mutex\0" as *const u8 as *const libc::c_char,
            // 756: b"worker %d fai ... _char: typeof(_31) = *const {l47} i8
            // 756: b"worker %d fai ... _char:   l47 = UNIQUE | NON_NULL, (empty)
            // 756: b"worker %d fai ... st u8: typeof(_32) = *const {l49} u8
            // 756: b"worker %d fai ... st u8:   l49 = UNIQUE | NON_NULL, (empty)
            // 756: b"worker %d fai ... ex\0": typeof(_33) = *const {l51} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
            // 756: b"worker %d fai ... ex\0":   l51 = UNIQUE | NON_NULL, (empty)
            // 756: b"worker %d fai ... ex\0": typeof(_34) = & {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
            // 756: b"worker %d fai ... ex\0":   l53 = UNIQUE | NON_NULL, FIXED
            // 756: b"worker %d fai ... ex\0": typeof(_33 = &raw const (*_34)) = *const {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
            // 756: b"worker %d fai ... ex\0":   l77 = UNIQUE | NON_NULL, (empty)
            // 756: b"worker %d fai ... ex\0": typeof(_34 = const b"worker %d failed to unlock \'done\' mutex\x00") = & {l76} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
            // 756: b"worker %d fai ... ex\0":   l76 = UNIQUE | NON_NULL, (empty)
            // 756: b"worker %d fai ... _char: typeof(_31 = move _32 as *const i8 (Misc)) = *const {l79} i8
            // 756: b"worker %d fai ... _char:   l79 = UNIQUE | NON_NULL, (empty)
            // 756: b"worker %d fai ... st u8: typeof(_32 = move _33 as *const u8 (Pointer(ArrayToPointer))) = *const {l78} u8
            // 756: b"worker %d fai ... st u8:   l78 = UNIQUE | NON_NULL, (empty)
            w.offset_from(workers) as libc::c_long as libc::c_int,
            // 757: w: typeof(_38) = *mut {l58} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 757: w:   l58 = UNIQUE | NON_NULL, (empty)
            // 757: workers: typeof(_39) = *const {l60} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 757: workers:   l60 = UNIQUE | NON_NULL, (empty)
            // 757: workers: typeof(_40) = *mut {l62} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 757: workers:   l62 = UNIQUE | NON_NULL, (empty)
            // 757: workers: typeof(_41) = *mut {l64} *mut {l65} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 757: workers:   l64 = UNIQUE | NON_NULL, (empty)
            // 757: workers:   l65 = UNIQUE | NON_NULL, (empty)
            // 757: workers: typeof(_39 = move _40 as *const Worker (Pointer(MutToConstPointer))) = *const {l80} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 757: workers:   l80 = UNIQUE | NON_NULL, (empty)
        );
    }
    return res;
}
unsafe extern "C" fn sat(mut w: *mut Worker) -> libc::c_int {
// 762: mut w: typeof(_1) = *mut {g13} DefId(0:297 ~ ilingeling[c969]::Worker)
// 762: mut w:   g13 = UNIQUE | NON_NULL, FIXED
    let mut res: libc::c_int = 0;
    let mut name: [libc::c_char; 100] = [0; 100];
    let mut cloned: *mut LGL = 0 as *mut LGL;
    // 765: mut cloned: typeof(_5) = *mut {l5} LGL
    // 765: mut cloned:   l5 = UNIQUE | NON_NULL, (empty)
    // 765: 0 as *mut LGL: typeof(_5 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l356} LGL
    // 765: 0 as *mut LGL:   l356 = UNIQUE | NON_NULL, (empty)
    if druptraceprefix.is_null() && doclone != 0 {
    // 766: druptraceprefix: typeof(_9) = *const {l10} i8
    // 766: druptraceprefix:   l10 = UNIQUE | NON_NULL, (empty)
    // 766: druptraceprefix: typeof(_10) = *mut {l12} *const {l13} i8
    // 766: druptraceprefix:   l12 = UNIQUE | NON_NULL, (empty)
    // 766: druptraceprefix:   l13 = UNIQUE | NON_NULL, (empty)
    // 766: doclone: typeof(_13) = *mut {l17} i32
    // 766: doclone:   l17 = UNIQUE | NON_NULL, (empty)
        lglsetopt(
            (*w).lgl,
            // 768: (*w).lgl: typeof(_15) = *mut {l20} LGL
            // 768: (*w).lgl:   l20 = UNIQUE | NON_NULL, (empty)
            b"clim\0" as *const u8 as *const libc::c_char,
            // 769: b"clim\0" as *c ... _char: typeof(_16) = *const {l22} i8
            // 769: b"clim\0" as *c ... _char:   l22 = UNIQUE | NON_NULL, (empty)
            // 769: b"clim\0" as *c ... st u8: typeof(_17) = *const {l24} u8
            // 769: b"clim\0" as *c ... st u8:   l24 = UNIQUE | NON_NULL, (empty)
            // 769: b"clim\0": typeof(_18) = *const {l26} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 769: b"clim\0":   l26 = UNIQUE | NON_NULL, (empty)
            // 769: b"clim\0": typeof(_19) = & {l28} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 769: b"clim\0":   l28 = UNIQUE | NON_NULL, FIXED
            // 769: b"clim\0": typeof(_18 = &raw const (*_19)) = *const {l358} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 769: b"clim\0":   l358 = UNIQUE | NON_NULL, (empty)
            // 769: b"clim\0" as *c ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l360} i8
            // 769: b"clim\0" as *c ... _char:   l360 = UNIQUE | NON_NULL, (empty)
            // 769: b"clim\0" as *c ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l359} u8
            // 769: b"clim\0" as *c ... st u8:   l359 = UNIQUE | NON_NULL, (empty)
            // 769: b"clim\0": typeof(_19 = const b"clim\x00") = & {l357} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 769: b"clim\0":   l357 = UNIQUE | NON_NULL, (empty)
            20000 as libc::c_int,
        );
    } else {
        lglsetopt(
            (*w).lgl,
            // 774: (*w).lgl: typeof(_22) = *mut {l32} LGL
            // 774: (*w).lgl:   l32 = UNIQUE | NON_NULL, (empty)
            b"clim\0" as *const u8 as *const libc::c_char,
            // 775: b"clim\0" as *c ... _char: typeof(_23) = *const {l34} i8
            // 775: b"clim\0" as *c ... _char:   l34 = UNIQUE | NON_NULL, (empty)
            // 775: b"clim\0" as *c ... st u8: typeof(_24) = *const {l36} u8
            // 775: b"clim\0" as *c ... st u8:   l36 = UNIQUE | NON_NULL, (empty)
            // 775: b"clim\0": typeof(_25) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 775: b"clim\0":   l38 = UNIQUE | NON_NULL, (empty)
            // 775: b"clim\0": typeof(_26) = & {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 775: b"clim\0":   l40 = UNIQUE | NON_NULL, FIXED
            // 775: b"clim\0": typeof(_25 = &raw const (*_26)) = *const {l362} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 775: b"clim\0":   l362 = UNIQUE | NON_NULL, (empty)
            // 775: b"clim\0" as *c ... st u8: typeof(_24 = move _25 as *const u8 (Pointer(ArrayToPointer))) = *const {l363} u8
            // 775: b"clim\0" as *c ... st u8:   l363 = UNIQUE | NON_NULL, (empty)
            // 775: b"clim\0": typeof(_26 = const b"clim\x00") = & {l361} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 775: b"clim\0":   l361 = UNIQUE | NON_NULL, (empty)
            // 775: b"clim\0" as *c ... _char: typeof(_23 = move _24 as *const i8 (Misc)) = *const {l364} i8
            // 775: b"clim\0" as *c ... _char:   l364 = UNIQUE | NON_NULL, (empty)
            -(1 as libc::c_int),
        );
        lglsetopt(
            (*w).lgl,
            // 779: (*w).lgl: typeof(_31) = *mut {l46} LGL
            // 779: (*w).lgl:   l46 = UNIQUE | NON_NULL, (empty)
            b"plim\0" as *const u8 as *const libc::c_char,
            // 780: b"plim\0" as *c ... _char: typeof(_32) = *const {l48} i8
            // 780: b"plim\0" as *c ... _char:   l48 = UNIQUE | NON_NULL, (empty)
            // 780: b"plim\0" as *c ... st u8: typeof(_33) = *const {l50} u8
            // 780: b"plim\0" as *c ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
            // 780: b"plim\0": typeof(_34) = *const {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 780: b"plim\0":   l52 = UNIQUE | NON_NULL, (empty)
            // 780: b"plim\0": typeof(_35) = & {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 780: b"plim\0":   l54 = UNIQUE | NON_NULL, FIXED
            // 780: b"plim\0" as *c ... _char: typeof(_32 = move _33 as *const i8 (Misc)) = *const {l368} i8
            // 780: b"plim\0" as *c ... _char:   l368 = UNIQUE | NON_NULL, (empty)
            // 780: b"plim\0" as *c ... st u8: typeof(_33 = move _34 as *const u8 (Pointer(ArrayToPointer))) = *const {l367} u8
            // 780: b"plim\0" as *c ... st u8:   l367 = UNIQUE | NON_NULL, (empty)
            // 780: b"plim\0": typeof(_34 = &raw const (*_35)) = *const {l366} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 780: b"plim\0":   l366 = UNIQUE | NON_NULL, (empty)
            // 780: b"plim\0": typeof(_35 = const b"plim\x00") = & {l365} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 780: b"plim\0":   l365 = UNIQUE | NON_NULL, (empty)
            -(1 as libc::c_int),
        );
        lglsetopt(
            (*w).lgl,
            // 784: (*w).lgl: typeof(_40) = *mut {l60} LGL
            // 784: (*w).lgl:   l60 = UNIQUE | NON_NULL, (empty)
            b"dlim\0" as *const u8 as *const libc::c_char,
            // 785: b"dlim\0" as *c ... _char: typeof(_41) = *const {l62} i8
            // 785: b"dlim\0" as *c ... _char:   l62 = UNIQUE | NON_NULL, (empty)
            // 785: b"dlim\0" as *c ... st u8: typeof(_42) = *const {l64} u8
            // 785: b"dlim\0" as *c ... st u8:   l64 = UNIQUE | NON_NULL, (empty)
            // 785: b"dlim\0": typeof(_43) = *const {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 785: b"dlim\0":   l66 = UNIQUE | NON_NULL, (empty)
            // 785: b"dlim\0": typeof(_44) = & {l68} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 785: b"dlim\0":   l68 = UNIQUE | NON_NULL, FIXED
            // 785: b"dlim\0" as *c ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l371} u8
            // 785: b"dlim\0" as *c ... st u8:   l371 = UNIQUE | NON_NULL, (empty)
            // 785: b"dlim\0" as *c ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l372} i8
            // 785: b"dlim\0" as *c ... _char:   l372 = UNIQUE | NON_NULL, (empty)
            // 785: b"dlim\0": typeof(_43 = &raw const (*_44)) = *const {l370} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 785: b"dlim\0":   l370 = UNIQUE | NON_NULL, (empty)
            // 785: b"dlim\0": typeof(_44 = const b"dlim\x00") = & {l369} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 785: b"dlim\0":   l369 = UNIQUE | NON_NULL, (empty)
            -(1 as libc::c_int),
        );
    }
    if noflush == 0 {
    // 789: noflush: typeof(_51) = *mut {l76} i32
    // 789: noflush:   l76 = UNIQUE | NON_NULL, (empty)
        lglflushcache((*w).lgl);
        // 790: (*w).lgl: typeof(_53) = *mut {l79} LGL
        // 790: (*w).lgl:   l79 = UNIQUE | NON_NULL, (empty)
    }
    res = lglsat((*w).lgl);
    // 792: (*w).lgl: typeof(_55) = *mut {l82} LGL
    // 792: (*w).lgl:   l82 = UNIQUE | NON_NULL, (empty)
    if res == 0 && justreturn(w) == 0 {
    // 793: w: typeof(_62) = *mut {l90} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 793: w:   l90 = UNIQUE | NON_NULL, (empty)
        msg(
            w,
            // 795: w: typeof(_64) = *mut {l93} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 795: w:   l93 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
            b"cloning after %d conflicts\0" as *const u8 as *const libc::c_char,
            // 797: b"cloning after ... _char: typeof(_66) = *const {l96} i8
            // 797: b"cloning after ... _char:   l96 = UNIQUE | NON_NULL, (empty)
            // 797: b"cloning after ... st u8: typeof(_67) = *const {l98} u8
            // 797: b"cloning after ... st u8:   l98 = UNIQUE | NON_NULL, (empty)
            // 797: b"cloning after ... ts\0": typeof(_68) = *const {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 797: b"cloning after ... ts\0":   l100 = UNIQUE | NON_NULL, (empty)
            // 797: b"cloning after ... ts\0": typeof(_69) = & {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 797: b"cloning after ... ts\0":   l102 = UNIQUE | NON_NULL, FIXED
            // 797: b"cloning after ... ts\0": typeof(_68 = &raw const (*_69)) = *const {l374} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 797: b"cloning after ... ts\0":   l374 = UNIQUE | NON_NULL, (empty)
            // 797: b"cloning after ... _char: typeof(_66 = move _67 as *const i8 (Misc)) = *const {l376} i8
            // 797: b"cloning after ... _char:   l376 = UNIQUE | NON_NULL, (empty)
            // 797: b"cloning after ... st u8: typeof(_67 = move _68 as *const u8 (Pointer(ArrayToPointer))) = *const {l375} u8
            // 797: b"cloning after ... st u8:   l375 = UNIQUE | NON_NULL, (empty)
            // 797: b"cloning after ... ts\0": typeof(_69 = const b"cloning after %d conflicts\x00") = & {l373} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 797: b"cloning after ... ts\0":   l373 = UNIQUE | NON_NULL, (empty)
            20000 as libc::c_int,
        );
        cloned = lglclone((*w).lgl);
        // 800: lglclone((*w).lgl): typeof(_71) = *mut {l105} LGL
        // 800: lglclone((*w).lgl):   l105 = UNIQUE | NON_NULL, (empty)
        // 800: (*w).lgl: typeof(_72) = *mut {l107} LGL
        // 800: (*w).lgl:   l107 = UNIQUE | NON_NULL, (empty)
        lglfixate(cloned);
        // 801: cloned: typeof(_74) = *mut {l110} LGL
        // 801: cloned:   l110 = UNIQUE | NON_NULL, (empty)
        lglmeltall(cloned);
        // 802: cloned: typeof(_76) = *mut {l113} LGL
        // 802: cloned:   l113 = UNIQUE | NON_NULL, (empty)
        initlgl(cloned, w, 0 as libc::c_int);
        // 803: cloned: typeof(_78) = *mut {l116} LGL
        // 803: cloned:   l116 = UNIQUE | NON_NULL, (empty)
        // 803: w: typeof(_79) = *mut {l118} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 803: w:   l118 = UNIQUE | NON_NULL, (empty)
        lglsetopt(
            cloned,
            // 805: cloned: typeof(_82) = *mut {l122} LGL
            // 805: cloned:   l122 = UNIQUE | NON_NULL, (empty)
            b"clim\0" as *const u8 as *const libc::c_char,
            // 806: b"clim\0" as *c ... _char: typeof(_83) = *const {l124} i8
            // 806: b"clim\0" as *c ... _char:   l124 = UNIQUE | NON_NULL, (empty)
            // 806: b"clim\0" as *c ... st u8: typeof(_84) = *const {l126} u8
            // 806: b"clim\0" as *c ... st u8:   l126 = UNIQUE | NON_NULL, (empty)
            // 806: b"clim\0": typeof(_85) = *const {l128} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 806: b"clim\0":   l128 = UNIQUE | NON_NULL, (empty)
            // 806: b"clim\0": typeof(_86) = & {l130} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 806: b"clim\0":   l130 = UNIQUE | NON_NULL, FIXED
            // 806: b"clim\0" as *c ... _char: typeof(_83 = move _84 as *const i8 (Misc)) = *const {l380} i8
            // 806: b"clim\0" as *c ... _char:   l380 = UNIQUE | NON_NULL, (empty)
            // 806: b"clim\0": typeof(_86 = const b"clim\x00") = & {l377} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 806: b"clim\0":   l377 = UNIQUE | NON_NULL, (empty)
            // 806: b"clim\0" as *c ... st u8: typeof(_84 = move _85 as *const u8 (Pointer(ArrayToPointer))) = *const {l379} u8
            // 806: b"clim\0" as *c ... st u8:   l379 = UNIQUE | NON_NULL, (empty)
            // 806: b"clim\0": typeof(_85 = &raw const (*_86)) = *const {l378} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 806: b"clim\0":   l378 = UNIQUE | NON_NULL, (empty)
            -(1 as libc::c_int),
        );
        let fresh3 = (*w).cloned.count;
        (*w).cloned.count = (*w).cloned.count + 1;
        sprintf(
            name.as_mut_ptr(),
            // 812: name.as_mut_ptr(): typeof(_94) = *mut {l139} i8
            // 812: name.as_mut_ptr():   l139 = UNIQUE | NON_NULL, (empty)
            // 812: name.as_mut_ptr(): typeof(_95) = &mut {l141} [i8]
            // 812: name.as_mut_ptr():   l141 = UNIQUE | NON_NULL, FIXED
            // 812: name.as_mut_ptr(): typeof(_96) = &mut {l143} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
            // 812: name.as_mut_ptr():   l143 = UNIQUE | NON_NULL, (empty)
            // 812: name.as_mut_ptr(): typeof(_95 = move _96 as &mut [i8] (Pointer(Unsize))) = &mut {l382} [i8]
            // 812: name.as_mut_ptr():   l382 = UNIQUE | NON_NULL, FIXED
            // 812: name.as_mut_ptr(): typeof(_96 = &mut _4) = &mut {l381} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
            // 812: name.as_mut_ptr():   l381 = UNIQUE | NON_NULL, (empty)
            b"c F%d \0" as *const u8 as *const libc::c_char,
            // 813: b"c F%d \0" as  ... _char: typeof(_97) = *const {l145} i8
            // 813: b"c F%d \0" as  ... _char:   l145 = UNIQUE | NON_NULL, (empty)
            // 813: b"c F%d \0" as  ... st u8: typeof(_98) = *const {l147} u8
            // 813: b"c F%d \0" as  ... st u8:   l147 = UNIQUE | NON_NULL, (empty)
            // 813: b"c F%d \0": typeof(_99) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 813: b"c F%d \0":   l149 = UNIQUE | NON_NULL, (empty)
            // 813: b"c F%d \0": typeof(_100) = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 813: b"c F%d \0":   l151 = UNIQUE | NON_NULL, FIXED
            // 813: b"c F%d \0" as  ... st u8: typeof(_98 = move _99 as *const u8 (Pointer(ArrayToPointer))) = *const {l385} u8
            // 813: b"c F%d \0" as  ... st u8:   l385 = UNIQUE | NON_NULL, (empty)
            // 813: b"c F%d \0" as  ... _char: typeof(_97 = move _98 as *const i8 (Misc)) = *const {l386} i8
            // 813: b"c F%d \0" as  ... _char:   l386 = UNIQUE | NON_NULL, (empty)
            // 813: b"c F%d \0": typeof(_100 = const b"c F%d \x00") = & {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 813: b"c F%d \0":   l383 = UNIQUE | NON_NULL, (empty)
            // 813: b"c F%d \0": typeof(_99 = &raw const (*_100)) = *const {l384} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 813: b"c F%d \0":   l384 = UNIQUE | NON_NULL, (empty)
            fresh3,
        );
        lglsetprefix(cloned, name.as_mut_ptr());
        // 816: cloned: typeof(_103) = *mut {l155} LGL
        // 816: cloned:   l155 = UNIQUE | NON_NULL, (empty)
        // 816: name.as_mut_ptr(): typeof(_104) = *const {l157} i8
        // 816: name.as_mut_ptr():   l157 = UNIQUE | NON_NULL, (empty)
        // 816: name.as_mut_ptr(): typeof(_105) = *mut {l159} i8
        // 816: name.as_mut_ptr():   l159 = UNIQUE | NON_NULL, (empty)
        // 816: name.as_mut_ptr(): typeof(_106) = &mut {l161} [i8]
        // 816: name.as_mut_ptr():   l161 = UNIQUE | NON_NULL, FIXED
        // 816: name.as_mut_ptr(): typeof(_107) = &mut {l163} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
        // 816: name.as_mut_ptr():   l163 = UNIQUE | NON_NULL, (empty)
        // 816: name.as_mut_ptr(): typeof(_106 = move _107 as &mut [i8] (Pointer(Unsize))) = &mut {l388} [i8]
        // 816: name.as_mut_ptr():   l388 = UNIQUE | NON_NULL, FIXED
        // 816: name.as_mut_ptr(): typeof(_107 = &mut _4) = &mut {l387} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
        // 816: name.as_mut_ptr():   l387 = UNIQUE | NON_NULL, (empty)
        // 816: name.as_mut_ptr(): typeof(_104 = move _105 as *const i8 (Pointer(MutToConstPointer))) = *const {l389} i8
        // 816: name.as_mut_ptr():   l389 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_lock(&mut (*w).cloned.lock) != 0 {
        // 817: &mut (*w).clone ... .lock: typeof(_111) = *mut {l168} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 817: &mut (*w).clone ... .lock:   l168 = UNIQUE | NON_NULL, (empty)
        // 817: &mut (*w).clone ... .lock: typeof(_112) = &mut {l170} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 817: &mut (*w).clone ... .lock:   l170 = UNIQUE | NON_NULL, (empty)
        // 817: &mut (*w).clone ... .lock: typeof(_112 = &mut (((*_1).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l390} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 817: &mut (*w).clone ... .lock:   l390 = UNIQUE | NON_NULL, (empty)
        // 817: &mut (*w).clone ... .lock: typeof(_111 = &raw mut (*_112)) = *mut {l391} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 817: &mut (*w).clone ... .lock:   l391 = UNIQUE | NON_NULL, (empty)
            warn(
                b"worker %d failed to lock 'cloned' mutex\0" as *const u8 as *const libc::c_char,
                // 819: b"worker %d fai ... _char: typeof(_114) = *const {l173} i8
                // 819: b"worker %d fai ... _char:   l173 = UNIQUE | NON_NULL, (empty)
                // 819: b"worker %d fai ... st u8: typeof(_115) = *const {l175} u8
                // 819: b"worker %d fai ... st u8:   l175 = UNIQUE | NON_NULL, (empty)
                // 819: b"worker %d fai ... ex\0": typeof(_116) = *const {l177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 819: b"worker %d fai ... ex\0":   l177 = UNIQUE | NON_NULL, (empty)
                // 819: b"worker %d fai ... ex\0": typeof(_117) = & {l179} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 819: b"worker %d fai ... ex\0":   l179 = UNIQUE | NON_NULL, FIXED
                // 819: b"worker %d fai ... ex\0": typeof(_117 = const b"worker %d failed to lock \'cloned\' mutex\x00") = & {l392} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 819: b"worker %d fai ... ex\0":   l392 = UNIQUE | NON_NULL, (empty)
                // 819: b"worker %d fai ... st u8: typeof(_115 = move _116 as *const u8 (Pointer(ArrayToPointer))) = *const {l394} u8
                // 819: b"worker %d fai ... st u8:   l394 = UNIQUE | NON_NULL, (empty)
                // 819: b"worker %d fai ... _char: typeof(_114 = move _115 as *const i8 (Misc)) = *const {l395} i8
                // 819: b"worker %d fai ... _char:   l395 = UNIQUE | NON_NULL, (empty)
                // 819: b"worker %d fai ... ex\0": typeof(_116 = &raw const (*_117)) = *const {l393} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 819: b"worker %d fai ... ex\0":   l393 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 820: w: typeof(_121) = *mut {l184} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 820: w:   l184 = UNIQUE | NON_NULL, (empty)
                // 820: workers: typeof(_122) = *const {l186} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 820: workers:   l186 = UNIQUE | NON_NULL, (empty)
                // 820: workers: typeof(_123) = *mut {l188} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 820: workers:   l188 = UNIQUE | NON_NULL, (empty)
                // 820: workers: typeof(_124) = *mut {l190} *mut {l191} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 820: workers:   l190 = UNIQUE | NON_NULL, (empty)
                // 820: workers:   l191 = UNIQUE | NON_NULL, (empty)
                // 820: workers: typeof(_122 = move _123 as *const Worker (Pointer(MutToConstPointer))) = *const {l396} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 820: workers:   l396 = UNIQUE | NON_NULL, (empty)
            );
        }
        (*w).cloned.lgl = cloned;
        // 823: cloned: typeof(_125) = *mut {l193} LGL
        // 823: cloned:   l193 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_unlock(&mut (*w).cloned.lock) != 0 {
        // 824: &mut (*w).clone ... .lock: typeof(_129) = *mut {l198} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 824: &mut (*w).clone ... .lock:   l198 = UNIQUE | NON_NULL, (empty)
        // 824: &mut (*w).clone ... .lock: typeof(_130) = &mut {l200} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 824: &mut (*w).clone ... .lock:   l200 = UNIQUE | NON_NULL, (empty)
        // 824: &mut (*w).clone ... .lock: typeof(_129 = &raw mut (*_130)) = *mut {l398} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 824: &mut (*w).clone ... .lock:   l398 = UNIQUE | NON_NULL, (empty)
        // 824: &mut (*w).clone ... .lock: typeof(_130 = &mut (((*_1).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l397} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 824: &mut (*w).clone ... .lock:   l397 = UNIQUE | NON_NULL, (empty)
            warn(
                b"worker %d failed to unlock 'cloned' mutex\0" as *const u8 as *const libc::c_char,
                // 826: b"worker %d fai ... _char: typeof(_132) = *const {l203} i8
                // 826: b"worker %d fai ... _char:   l203 = UNIQUE | NON_NULL, (empty)
                // 826: b"worker %d fai ... st u8: typeof(_133) = *const {l205} u8
                // 826: b"worker %d fai ... st u8:   l205 = UNIQUE | NON_NULL, (empty)
                // 826: b"worker %d fai ... ex\0": typeof(_134) = *const {l207} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 826: b"worker %d fai ... ex\0":   l207 = UNIQUE | NON_NULL, (empty)
                // 826: b"worker %d fai ... ex\0": typeof(_135) = & {l209} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 826: b"worker %d fai ... ex\0":   l209 = UNIQUE | NON_NULL, FIXED
                // 826: b"worker %d fai ... _char: typeof(_132 = move _133 as *const i8 (Misc)) = *const {l402} i8
                // 826: b"worker %d fai ... _char:   l402 = UNIQUE | NON_NULL, (empty)
                // 826: b"worker %d fai ... ex\0": typeof(_135 = const b"worker %d failed to unlock \'cloned\' mutex\x00") = & {l399} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 826: b"worker %d fai ... ex\0":   l399 = UNIQUE | NON_NULL, (empty)
                // 826: b"worker %d fai ... st u8: typeof(_133 = move _134 as *const u8 (Pointer(ArrayToPointer))) = *const {l401} u8
                // 826: b"worker %d fai ... st u8:   l401 = UNIQUE | NON_NULL, (empty)
                // 826: b"worker %d fai ... ex\0": typeof(_134 = &raw const (*_135)) = *const {l400} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 826: b"worker %d fai ... ex\0":   l400 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 827: w: typeof(_139) = *mut {l214} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 827: w:   l214 = UNIQUE | NON_NULL, (empty)
                // 827: workers: typeof(_140) = *const {l216} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 827: workers:   l216 = UNIQUE | NON_NULL, (empty)
                // 827: workers: typeof(_141) = *mut {l218} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 827: workers:   l218 = UNIQUE | NON_NULL, (empty)
                // 827: workers: typeof(_142) = *mut {l220} *mut {l221} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 827: workers:   l220 = UNIQUE | NON_NULL, (empty)
                // 827: workers:   l221 = UNIQUE | NON_NULL, (empty)
                // 827: workers: typeof(_140 = move _141 as *const Worker (Pointer(MutToConstPointer))) = *const {l403} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 827: workers:   l403 = UNIQUE | NON_NULL, (empty)
            );
        }
        res = lglsat(cloned);
        // 830: cloned: typeof(_144) = *mut {l224} LGL
        // 830: cloned:   l224 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_lock(&mut (*w).cloned.lock) != 0 {
        // 831: &mut (*w).clone ... .lock: typeof(_148) = *mut {l229} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 831: &mut (*w).clone ... .lock:   l229 = UNIQUE | NON_NULL, (empty)
        // 831: &mut (*w).clone ... .lock: typeof(_149) = &mut {l231} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 831: &mut (*w).clone ... .lock:   l231 = UNIQUE | NON_NULL, (empty)
        // 831: &mut (*w).clone ... .lock: typeof(_149 = &mut (((*_1).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l404} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 831: &mut (*w).clone ... .lock:   l404 = UNIQUE | NON_NULL, (empty)
        // 831: &mut (*w).clone ... .lock: typeof(_148 = &raw mut (*_149)) = *mut {l405} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 831: &mut (*w).clone ... .lock:   l405 = UNIQUE | NON_NULL, (empty)
            warn(
                b"worker %d failed to lock 'cloned' mutex\0" as *const u8 as *const libc::c_char,
                // 833: b"worker %d fai ... _char: typeof(_151) = *const {l234} i8
                // 833: b"worker %d fai ... _char:   l234 = UNIQUE | NON_NULL, (empty)
                // 833: b"worker %d fai ... st u8: typeof(_152) = *const {l236} u8
                // 833: b"worker %d fai ... st u8:   l236 = UNIQUE | NON_NULL, (empty)
                // 833: b"worker %d fai ... ex\0": typeof(_153) = *const {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 833: b"worker %d fai ... ex\0":   l238 = UNIQUE | NON_NULL, (empty)
                // 833: b"worker %d fai ... ex\0": typeof(_154) = & {l240} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 833: b"worker %d fai ... ex\0":   l240 = UNIQUE | NON_NULL, FIXED
                // 833: b"worker %d fai ... ex\0": typeof(_153 = &raw const (*_154)) = *const {l407} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 833: b"worker %d fai ... ex\0":   l407 = UNIQUE | NON_NULL, (empty)
                // 833: b"worker %d fai ... st u8: typeof(_152 = move _153 as *const u8 (Pointer(ArrayToPointer))) = *const {l408} u8
                // 833: b"worker %d fai ... st u8:   l408 = UNIQUE | NON_NULL, (empty)
                // 833: b"worker %d fai ... _char: typeof(_151 = move _152 as *const i8 (Misc)) = *const {l409} i8
                // 833: b"worker %d fai ... _char:   l409 = UNIQUE | NON_NULL, (empty)
                // 833: b"worker %d fai ... ex\0": typeof(_154 = const b"worker %d failed to lock \'cloned\' mutex\x00") = & {l406} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                // 833: b"worker %d fai ... ex\0":   l406 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 834: w: typeof(_158) = *mut {l245} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 834: w:   l245 = UNIQUE | NON_NULL, (empty)
                // 834: workers: typeof(_159) = *const {l247} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 834: workers:   l247 = UNIQUE | NON_NULL, (empty)
                // 834: workers: typeof(_160) = *mut {l249} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 834: workers:   l249 = UNIQUE | NON_NULL, (empty)
                // 834: workers: typeof(_161) = *mut {l251} *mut {l252} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 834: workers:   l251 = UNIQUE | NON_NULL, (empty)
                // 834: workers:   l252 = UNIQUE | NON_NULL, (empty)
                // 834: workers: typeof(_159 = move _160 as *const Worker (Pointer(MutToConstPointer))) = *const {l410} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 834: workers:   l410 = UNIQUE | NON_NULL, (empty)
            );
        }
        (*w).cloned.lgl = 0 as *mut LGL;
        // 837: (*w).cloned.lgl ... t LGL: typeof((((*_1).1: C2RustUnnamed_1).0: *mut LGL) = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l411} LGL
        // 837: (*w).cloned.lgl ... t LGL:   l411 = UNIQUE | NON_NULL, (empty)
        (*w).cloned.decs += lglgetdecs(cloned);
        // 838: cloned: typeof(_163) = *mut {l255} LGL
        // 838: cloned:   l255 = UNIQUE | NON_NULL, (empty)
        (*w).cloned.confs += lglgetconfs(cloned);
        // 839: cloned: typeof(_166) = *mut {l259} LGL
        // 839: cloned:   l259 = UNIQUE | NON_NULL, (empty)
        (*w).cloned.props += lglgetprops(cloned);
        // 840: cloned: typeof(_169) = *mut {l263} LGL
        // 840: cloned:   l263 = UNIQUE | NON_NULL, (empty)
        (*w).cloned.decs -= lglgetdecs((*w).lgl);
        // 841: (*w).lgl: typeof(_172) = *mut {l267} LGL
        // 841: (*w).lgl:   l267 = UNIQUE | NON_NULL, (empty)
        (*w).cloned.confs -= lglgetconfs((*w).lgl);
        // 842: (*w).lgl: typeof(_175) = *mut {l271} LGL
        // 842: (*w).lgl:   l271 = UNIQUE | NON_NULL, (empty)
        (*w).cloned.props -= lglgetprops((*w).lgl);
        // 843: (*w).lgl: typeof(_178) = *mut {l275} LGL
        // 843: (*w).lgl:   l275 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_unlock(&mut (*w).cloned.lock) != 0 {
        // 844: &mut (*w).clone ... .lock: typeof(_183) = *mut {l281} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 844: &mut (*w).clone ... .lock:   l281 = UNIQUE | NON_NULL, (empty)
        // 844: &mut (*w).clone ... .lock: typeof(_184) = &mut {l283} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 844: &mut (*w).clone ... .lock:   l283 = UNIQUE | NON_NULL, (empty)
        // 844: &mut (*w).clone ... .lock: typeof(_183 = &raw mut (*_184)) = *mut {l413} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 844: &mut (*w).clone ... .lock:   l413 = UNIQUE | NON_NULL, (empty)
        // 844: &mut (*w).clone ... .lock: typeof(_184 = &mut (((*_1).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l412} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 844: &mut (*w).clone ... .lock:   l412 = UNIQUE | NON_NULL, (empty)
            warn(
                b"worker %d failed to unlock 'cloned' mutex\0" as *const u8 as *const libc::c_char,
                // 846: b"worker %d fai ... _char: typeof(_186) = *const {l286} i8
                // 846: b"worker %d fai ... _char:   l286 = UNIQUE | NON_NULL, (empty)
                // 846: b"worker %d fai ... st u8: typeof(_187) = *const {l288} u8
                // 846: b"worker %d fai ... st u8:   l288 = UNIQUE | NON_NULL, (empty)
                // 846: b"worker %d fai ... ex\0": typeof(_188) = *const {l290} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 846: b"worker %d fai ... ex\0":   l290 = UNIQUE | NON_NULL, (empty)
                // 846: b"worker %d fai ... ex\0": typeof(_189) = & {l292} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 846: b"worker %d fai ... ex\0":   l292 = UNIQUE | NON_NULL, FIXED
                // 846: b"worker %d fai ... ex\0": typeof(_188 = &raw const (*_189)) = *const {l415} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 846: b"worker %d fai ... ex\0":   l415 = UNIQUE | NON_NULL, (empty)
                // 846: b"worker %d fai ... st u8: typeof(_187 = move _188 as *const u8 (Pointer(ArrayToPointer))) = *const {l416} u8
                // 846: b"worker %d fai ... st u8:   l416 = UNIQUE | NON_NULL, (empty)
                // 846: b"worker %d fai ... _char: typeof(_186 = move _187 as *const i8 (Misc)) = *const {l417} i8
                // 846: b"worker %d fai ... _char:   l417 = UNIQUE | NON_NULL, (empty)
                // 846: b"worker %d fai ... ex\0": typeof(_189 = const b"worker %d failed to unlock \'cloned\' mutex\x00") = & {l414} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
                // 846: b"worker %d fai ... ex\0":   l414 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 847: w: typeof(_193) = *mut {l297} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 847: w:   l297 = UNIQUE | NON_NULL, (empty)
                // 847: workers: typeof(_194) = *const {l299} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 847: workers:   l299 = UNIQUE | NON_NULL, (empty)
                // 847: workers: typeof(_195) = *mut {l301} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 847: workers:   l301 = UNIQUE | NON_NULL, (empty)
                // 847: workers: typeof(_196) = *mut {l303} *mut {l304} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 847: workers:   l303 = UNIQUE | NON_NULL, (empty)
                // 847: workers:   l304 = UNIQUE | NON_NULL, (empty)
                // 847: workers: typeof(_194 = move _195 as *const Worker (Pointer(MutToConstPointer))) = *const {l418} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 847: workers:   l418 = UNIQUE | NON_NULL, (empty)
            );
        }
        if verbose >= 2 as libc::c_int && !statsfile.is_null() {
        // 850: verbose: typeof(_201) = *mut {l310} i32
        // 850: verbose:   l310 = UNIQUE | NON_NULL, (empty)
        // 850: statsfile: typeof(_205) = *mut {l315} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 850: statsfile:   l315 = UNIQUE | NON_NULL, (empty)
        // 850: statsfile: typeof(_206) = *mut {l317} *mut {l318} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 850: statsfile:   l317 = UNIQUE | NON_NULL, (empty)
        // 850: statsfile:   l318 = UNIQUE | NON_NULL, (empty)
            lglsetout(cloned, statsfile);
            // 851: cloned: typeof(_208) = *mut {l321} LGL
            // 851: cloned:   l321 = UNIQUE | NON_NULL, (empty)
            // 851: statsfile: typeof(_209) = *mut {l323} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 851: statsfile:   l323 = UNIQUE | NON_NULL, (empty)
            // 851: statsfile: typeof(_210) = *mut {l325} *mut {l326} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 851: statsfile:   l325 = UNIQUE | NON_NULL, (empty)
            // 851: statsfile:   l326 = UNIQUE | NON_NULL, (empty)
            lglstats(cloned);
            // 852: cloned: typeof(_212) = *mut {l329} LGL
            // 852: cloned:   l329 = UNIQUE | NON_NULL, (empty)
            lglsetout(cloned, stdout);
            // 853: cloned: typeof(_214) = *mut {l332} LGL
            // 853: cloned:   l332 = UNIQUE | NON_NULL, (empty)
            // 853: stdout: typeof(_215) = *mut {l334} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 853: stdout:   l334 = UNIQUE | NON_NULL, (empty)
            // 853: stdout: typeof(_216) = *mut {l336} *mut {l337} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 853: stdout:   l336 = UNIQUE | NON_NULL, (empty)
            // 853: stdout:   l337 = UNIQUE | NON_NULL, (empty)
        }
        msg(
            w,
            // 856: w: typeof(_218) = *mut {l340} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 856: w:   l340 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
            b"joining cloned solver\0" as *const u8 as *const libc::c_char,
            // 858: b"joining clone ... _char: typeof(_220) = *const {l343} i8
            // 858: b"joining clone ... _char:   l343 = UNIQUE | NON_NULL, (empty)
            // 858: b"joining clone ... st u8: typeof(_221) = *const {l345} u8
            // 858: b"joining clone ... st u8:   l345 = UNIQUE | NON_NULL, (empty)
            // 858: b"joining clone ... er\0": typeof(_222) = *const {l347} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 858: b"joining clone ... er\0":   l347 = UNIQUE | NON_NULL, (empty)
            // 858: b"joining clone ... er\0": typeof(_223) = & {l349} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 858: b"joining clone ... er\0":   l349 = UNIQUE | NON_NULL, FIXED
            // 858: b"joining clone ... er\0": typeof(_223 = const b"joining cloned solver\x00") = & {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 858: b"joining clone ... er\0":   l419 = UNIQUE | NON_NULL, (empty)
            // 858: b"joining clone ... _char: typeof(_220 = move _221 as *const i8 (Misc)) = *const {l422} i8
            // 858: b"joining clone ... _char:   l422 = UNIQUE | NON_NULL, (empty)
            // 858: b"joining clone ... er\0": typeof(_222 = &raw const (*_223)) = *const {l420} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 858: b"joining clone ... er\0":   l420 = UNIQUE | NON_NULL, (empty)
            // 858: b"joining clone ... st u8: typeof(_221 = move _222 as *const u8 (Pointer(ArrayToPointer))) = *const {l421} u8
            // 858: b"joining clone ... st u8:   l421 = UNIQUE | NON_NULL, (empty)
        );
        lglunclone((*w).lgl, cloned);
        // 860: (*w).lgl: typeof(_225) = *mut {l352} LGL
        // 860: (*w).lgl:   l352 = UNIQUE | NON_NULL, (empty)
        // 860: cloned: typeof(_226) = *mut {l354} LGL
        // 860: cloned:   l354 = UNIQUE | NON_NULL, (empty)
    }
    return res;
}
unsafe extern "C" fn work(mut voidptr: *mut libc::c_void) -> *mut libc::c_void {
// 864: *mut libc::c_void: typeof(_0) = *mut {g15} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 864: *mut libc::c_void:   g15 = UNIQUE | NON_NULL, FIXED
// 864: mut voidptr: typeof(_1) = *mut {g14} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 864: mut voidptr:   g14 = UNIQUE | NON_NULL, FIXED
    let mut i: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut pm: libc::c_int = 0;
    let mut lm: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut idx: libc::c_int = 0;
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    // 872: mut a: typeof(_9) = *mut {l9} i32
    // 872: mut a:   l9 = UNIQUE | NON_NULL, (empty)
    // 872: 0 as *mut libc: ... c_int: typeof(_9 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l999} i32
    // 872: 0 as *mut libc: ... c_int:   l999 = UNIQUE | NON_NULL, (empty)
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    // 873: mut p: typeof(_10) = *mut {l11} i32
    // 873: mut p:   l11 = UNIQUE | NON_NULL, (empty)
    // 873: 0 as *mut libc: ... c_int: typeof(_10 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l1000} i32
    // 873: 0 as *mut libc: ... c_int:   l1000 = UNIQUE | NON_NULL, (empty)
    let mut size: libc::c_int = 0;
    let mut red: libc::c_int = 0;
    let mut fin: libc::c_int = 0;
    let mut start_0: libc::c_double = 0.;
    let mut end: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut avg: libc::c_double = 0.;
    let mut w: *mut Worker = voidptr as *mut Worker;
    // 881: mut w: typeof(_18) = *mut {l20} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 881: mut w:   l20 = UNIQUE | NON_NULL, (empty)
    // 881: voidptr: typeof(_19) = *mut {l22} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 881: voidptr:   l22 = UNIQUE | NON_NULL, (empty)
    // 881: voidptr as *mut ... orker: typeof(_18 = move _19 as *mut Worker (Misc)) = *mut {l1001} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 881: voidptr as *mut ... orker:   l1001 = UNIQUE | NON_NULL, (empty)
    msg(
        w,
        // 883: w: typeof(_21) = *mut {l25} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 883: w:   l25 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"running\0" as *const u8 as *const libc::c_char,
        // 885: b"running\0" as ... _char: typeof(_23) = *const {l28} i8
        // 885: b"running\0" as ... _char:   l28 = UNIQUE | NON_NULL, (empty)
        // 885: b"running\0" as ... st u8: typeof(_24) = *const {l30} u8
        // 885: b"running\0" as ... st u8:   l30 = UNIQUE | NON_NULL, (empty)
        // 885: b"running\0": typeof(_25) = *const {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 885: b"running\0":   l32 = UNIQUE | NON_NULL, (empty)
        // 885: b"running\0": typeof(_26) = & {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 885: b"running\0":   l34 = UNIQUE | NON_NULL, FIXED
        // 885: b"running\0": typeof(_26 = const b"running\x00") = & {l1002} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 885: b"running\0":   l1002 = UNIQUE | NON_NULL, (empty)
        // 885: b"running\0" as ... st u8: typeof(_24 = move _25 as *const u8 (Pointer(ArrayToPointer))) = *const {l1004} u8
        // 885: b"running\0" as ... st u8:   l1004 = UNIQUE | NON_NULL, (empty)
        // 885: b"running\0": typeof(_25 = &raw const (*_26)) = *const {l1003} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 885: b"running\0":   l1003 = UNIQUE | NON_NULL, (empty)
        // 885: b"running\0" as ... _char: typeof(_23 = move _24 as *const i8 (Misc)) = *const {l1005} i8
        // 885: b"running\0" as ... _char:   l1005 = UNIQUE | NON_NULL, (empty)
    );
    loop {
        if pthread_mutex_lock(&mut queuemutex) != 0 {
        // 888: &mut queuemutex: typeof(_32) = *mut {l41} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 888: &mut queuemutex:   l41 = UNIQUE | NON_NULL, (empty)
        // 888: &mut queuemutex: typeof(_33) = &mut {l43} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 888: &mut queuemutex:   l43 = UNIQUE | NON_NULL, (empty)
        // 888: queuemutex: typeof(_34) = *mut {l45} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 888: queuemutex:   l45 = UNIQUE | NON_NULL, (empty)
        // 888: &mut queuemutex: typeof(_32 = &raw mut (*_33)) = *mut {l1007} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 888: &mut queuemutex:   l1007 = UNIQUE | NON_NULL, (empty)
        // 888: &mut queuemutex: typeof(_33 = &mut (*_34)) = &mut {l1006} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 888: &mut queuemutex:   l1006 = UNIQUE | NON_NULL, (empty)
            die(
                b"worker %d failed to lock 'queue' mutex\0" as *const u8 as *const libc::c_char,
                // 890: b"worker %d fai ... _char: typeof(_36) = *const {l48} i8
                // 890: b"worker %d fai ... _char:   l48 = UNIQUE | NON_NULL, (empty)
                // 890: b"worker %d fai ... st u8: typeof(_37) = *const {l50} u8
                // 890: b"worker %d fai ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
                // 890: b"worker %d fai ... ex\0": typeof(_38) = *const {l52} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 890: b"worker %d fai ... ex\0":   l52 = UNIQUE | NON_NULL, (empty)
                // 890: b"worker %d fai ... ex\0": typeof(_39) = & {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 890: b"worker %d fai ... ex\0":   l54 = UNIQUE | NON_NULL, FIXED
                // 890: b"worker %d fai ... _char: typeof(_36 = move _37 as *const i8 (Misc)) = *const {l1011} i8
                // 890: b"worker %d fai ... _char:   l1011 = UNIQUE | NON_NULL, (empty)
                // 890: b"worker %d fai ... st u8: typeof(_37 = move _38 as *const u8 (Pointer(ArrayToPointer))) = *const {l1010} u8
                // 890: b"worker %d fai ... st u8:   l1010 = UNIQUE | NON_NULL, (empty)
                // 890: b"worker %d fai ... ex\0": typeof(_39 = const b"worker %d failed to lock \'queue\' mutex\x00") = & {l1008} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 890: b"worker %d fai ... ex\0":   l1008 = UNIQUE | NON_NULL, (empty)
                // 890: b"worker %d fai ... ex\0": typeof(_38 = &raw const (*_39)) = *const {l1009} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 890: b"worker %d fai ... ex\0":   l1009 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 891: w: typeof(_43) = *mut {l59} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 891: w:   l59 = UNIQUE | NON_NULL, (empty)
                // 891: workers: typeof(_44) = *const {l61} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 891: workers:   l61 = UNIQUE | NON_NULL, (empty)
                // 891: workers: typeof(_45) = *mut {l63} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 891: workers:   l63 = UNIQUE | NON_NULL, (empty)
                // 891: workers: typeof(_46) = *mut {l65} *mut {l66} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 891: workers:   l65 = UNIQUE | NON_NULL, (empty)
                // 891: workers:   l66 = UNIQUE | NON_NULL, (empty)
                // 891: workers: typeof(_44 = move _45 as *const Worker (Pointer(MutToConstPointer))) = *const {l1012} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 891: workers:   l1012 = UNIQUE | NON_NULL, (empty)
            );
        }
        if deterministic != 0
        // 894: deterministic: typeof(_50) = *mut {l71} i32
        // 894: deterministic:   l71 = UNIQUE | NON_NULL, (empty)
            && queue % nworkers != w.offset_from(workers) as libc::c_long as libc::c_int
            // 895: queue: typeof(_54) = *mut {l76} i32
            // 895: queue:   l76 = UNIQUE | NON_NULL, (empty)
            // 895: nworkers: typeof(_56) = *mut {l79} i32
            // 895: nworkers:   l79 = UNIQUE | NON_NULL, (empty)
            // 895: w: typeof(_64) = *mut {l88} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 895: w:   l88 = UNIQUE | NON_NULL, (empty)
            // 895: workers: typeof(_65) = *const {l90} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 895: workers:   l90 = UNIQUE | NON_NULL, (empty)
            // 895: workers: typeof(_66) = *mut {l92} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 895: workers:   l92 = UNIQUE | NON_NULL, (empty)
            // 895: workers: typeof(_67) = *mut {l94} *mut {l95} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 895: workers:   l94 = UNIQUE | NON_NULL, (empty)
            // 895: workers:   l95 = UNIQUE | NON_NULL, (empty)
            // 895: workers: typeof(_65 = move _66 as *const Worker (Pointer(MutToConstPointer))) = *const {l1013} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 895: workers:   l1013 = UNIQUE | NON_NULL, (empty)
        {
            if pthread_mutex_unlock(&mut queuemutex) != 0 {
            // 897: &mut queuemutex: typeof(_71) = *mut {l100} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 897: &mut queuemutex:   l100 = UNIQUE | NON_NULL, (empty)
            // 897: &mut queuemutex: typeof(_72) = &mut {l102} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 897: &mut queuemutex:   l102 = UNIQUE | NON_NULL, (empty)
            // 897: queuemutex: typeof(_73) = *mut {l104} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 897: queuemutex:   l104 = UNIQUE | NON_NULL, (empty)
            // 897: &mut queuemutex: typeof(_71 = &raw mut (*_72)) = *mut {l1015} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 897: &mut queuemutex:   l1015 = UNIQUE | NON_NULL, (empty)
            // 897: &mut queuemutex: typeof(_72 = &mut (*_73)) = &mut {l1014} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 897: &mut queuemutex:   l1014 = UNIQUE | NON_NULL, (empty)
                die(
                    b"worker %d failed to unlock 'queue' mutex\0" as *const u8
                    // 899: b"worker %d fai ... _char: typeof(_75) = *const {l107} i8
                    // 899: b"worker %d fai ... _char:   l107 = UNIQUE | NON_NULL, (empty)
                    // 899: b"worker %d fai ... st u8: typeof(_76) = *const {l109} u8
                    // 899: b"worker %d fai ... st u8:   l109 = UNIQUE | NON_NULL, (empty)
                    // 899: b"worker %d fai ... ex\0": typeof(_77) = *const {l111} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 899: b"worker %d fai ... ex\0":   l111 = UNIQUE | NON_NULL, (empty)
                    // 899: b"worker %d fai ... ex\0": typeof(_78) = & {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 899: b"worker %d fai ... ex\0":   l113 = UNIQUE | NON_NULL, FIXED
                    // 899: b"worker %d fai ... _char: typeof(_75 = move _76 as *const i8 (Misc)) = *const {l1019} i8
                    // 899: b"worker %d fai ... _char:   l1019 = UNIQUE | NON_NULL, (empty)
                    // 899: b"worker %d fai ... st u8: typeof(_76 = move _77 as *const u8 (Pointer(ArrayToPointer))) = *const {l1018} u8
                    // 899: b"worker %d fai ... st u8:   l1018 = UNIQUE | NON_NULL, (empty)
                    // 899: b"worker %d fai ... ex\0": typeof(_78 = const b"worker %d failed to unlock \'queue\' mutex\x00") = & {l1016} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 899: b"worker %d fai ... ex\0":   l1016 = UNIQUE | NON_NULL, (empty)
                    // 899: b"worker %d fai ... ex\0": typeof(_77 = &raw const (*_78)) = *const {l1017} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 899: b"worker %d fai ... ex\0":   l1017 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    w.offset_from(workers) as libc::c_long as libc::c_int,
                    // 901: w: typeof(_82) = *mut {l118} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 901: w:   l118 = UNIQUE | NON_NULL, (empty)
                    // 901: workers: typeof(_83) = *const {l120} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 901: workers:   l120 = UNIQUE | NON_NULL, (empty)
                    // 901: workers: typeof(_84) = *mut {l122} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 901: workers:   l122 = UNIQUE | NON_NULL, (empty)
                    // 901: workers: typeof(_85) = *mut {l124} *mut {l125} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 901: workers:   l124 = UNIQUE | NON_NULL, (empty)
                    // 901: workers:   l125 = UNIQUE | NON_NULL, (empty)
                    // 901: workers: typeof(_83 = move _84 as *const Worker (Pointer(MutToConstPointer))) = *const {l1020} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 901: workers:   l1020 = UNIQUE | NON_NULL, (empty)
                );
            }
            usleep(1000 as libc::c_int as __useconds_t);
        } else {
            last = queue;
            // 906: queue: typeof(_91) = *mut {l132} i32
            // 906: queue:   l132 = UNIQUE | NON_NULL, (empty)
            if last < nassumptions {
            // 907: nassumptions: typeof(_96) = *mut {l138} i32
            // 907: nassumptions:   l138 = UNIQUE | NON_NULL, (empty)
                queue += 1;
                // 908: queue: typeof(_97) = *mut {l140} i32
                // 908: queue:   l140 = UNIQUE | NON_NULL, (empty)
                queue;
                // 909: queue: typeof(_100) = *mut {l144} i32
                // 909: queue:   l144 = UNIQUE | NON_NULL, (empty)
            }
            if pthread_mutex_unlock(&mut queuemutex) != 0 {
            // 911: &mut queuemutex: typeof(_104) = *mut {l149} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 911: &mut queuemutex:   l149 = UNIQUE | NON_NULL, (empty)
            // 911: &mut queuemutex: typeof(_105) = &mut {l151} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 911: &mut queuemutex:   l151 = UNIQUE | NON_NULL, (empty)
            // 911: queuemutex: typeof(_106) = *mut {l153} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 911: queuemutex:   l153 = UNIQUE | NON_NULL, (empty)
            // 911: &mut queuemutex: typeof(_105 = &mut (*_106)) = &mut {l1021} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 911: &mut queuemutex:   l1021 = UNIQUE | NON_NULL, (empty)
            // 911: &mut queuemutex: typeof(_104 = &raw mut (*_105)) = *mut {l1022} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
            // 911: &mut queuemutex:   l1022 = UNIQUE | NON_NULL, (empty)
                die(
                    b"worker %d failed to unlock 'queue' mutex\0" as *const u8
                    // 913: b"worker %d fai ... _char: typeof(_108) = *const {l156} i8
                    // 913: b"worker %d fai ... _char:   l156 = UNIQUE | NON_NULL, (empty)
                    // 913: b"worker %d fai ... st u8: typeof(_109) = *const {l158} u8
                    // 913: b"worker %d fai ... st u8:   l158 = UNIQUE | NON_NULL, (empty)
                    // 913: b"worker %d fai ... ex\0": typeof(_110) = *const {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 913: b"worker %d fai ... ex\0":   l160 = UNIQUE | NON_NULL, (empty)
                    // 913: b"worker %d fai ... ex\0": typeof(_111) = & {l162} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 913: b"worker %d fai ... ex\0":   l162 = UNIQUE | NON_NULL, FIXED
                    // 913: b"worker %d fai ... st u8: typeof(_109 = move _110 as *const u8 (Pointer(ArrayToPointer))) = *const {l1025} u8
                    // 913: b"worker %d fai ... st u8:   l1025 = UNIQUE | NON_NULL, (empty)
                    // 913: b"worker %d fai ... _char: typeof(_108 = move _109 as *const i8 (Misc)) = *const {l1026} i8
                    // 913: b"worker %d fai ... _char:   l1026 = UNIQUE | NON_NULL, (empty)
                    // 913: b"worker %d fai ... ex\0": typeof(_110 = &raw const (*_111)) = *const {l1024} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 913: b"worker %d fai ... ex\0":   l1024 = UNIQUE | NON_NULL, (empty)
                    // 913: b"worker %d fai ... ex\0": typeof(_111 = const b"worker %d failed to unlock \'queue\' mutex\x00") = & {l1023} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 913: b"worker %d fai ... ex\0":   l1023 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    w.offset_from(workers) as libc::c_long as libc::c_int,
                    // 915: w: typeof(_115) = *mut {l167} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 915: w:   l167 = UNIQUE | NON_NULL, (empty)
                    // 915: workers: typeof(_116) = *const {l169} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 915: workers:   l169 = UNIQUE | NON_NULL, (empty)
                    // 915: workers: typeof(_117) = *mut {l171} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 915: workers:   l171 = UNIQUE | NON_NULL, (empty)
                    // 915: workers: typeof(_118) = *mut {l173} *mut {l174} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 915: workers:   l173 = UNIQUE | NON_NULL, (empty)
                    // 915: workers:   l174 = UNIQUE | NON_NULL, (empty)
                    // 915: workers: typeof(_116 = move _117 as *const Worker (Pointer(MutToConstPointer))) = *const {l1027} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 915: workers:   l1027 = UNIQUE | NON_NULL, (empty)
                );
            }
            if !(last == nassumptions) {
            // 918: nassumptions: typeof(_124) = *mut {l181} i32
            // 918: nassumptions:   l181 = UNIQUE | NON_NULL, (empty)
                msg(
                    w,
                    // 920: w: typeof(_126) = *mut {l184} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 920: w:   l184 = UNIQUE | NON_NULL, (empty)
                    2 as libc::c_int,
                    b"got job %d\0" as *const u8 as *const libc::c_char,
                    // 922: b"got job %d\0" ... _char: typeof(_128) = *const {l187} i8
                    // 922: b"got job %d\0" ... _char:   l187 = UNIQUE | NON_NULL, (empty)
                    // 922: b"got job %d\0" ... st u8: typeof(_129) = *const {l189} u8
                    // 922: b"got job %d\0" ... st u8:   l189 = UNIQUE | NON_NULL, (empty)
                    // 922: b"got job %d\0": typeof(_130) = *const {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 922: b"got job %d\0":   l191 = UNIQUE | NON_NULL, (empty)
                    // 922: b"got job %d\0": typeof(_131) = & {l193} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 922: b"got job %d\0":   l193 = UNIQUE | NON_NULL, FIXED
                    // 922: b"got job %d\0" ... _char: typeof(_128 = move _129 as *const i8 (Misc)) = *const {l1031} i8
                    // 922: b"got job %d\0" ... _char:   l1031 = UNIQUE | NON_NULL, (empty)
                    // 922: b"got job %d\0": typeof(_130 = &raw const (*_131)) = *const {l1029} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 922: b"got job %d\0":   l1029 = UNIQUE | NON_NULL, (empty)
                    // 922: b"got job %d\0": typeof(_131 = const b"got job %d\x00") = & {l1028} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                    // 922: b"got job %d\0":   l1028 = UNIQUE | NON_NULL, (empty)
                    // 922: b"got job %d\0" ... st u8: typeof(_129 = move _130 as *const u8 (Pointer(ArrayToPointer))) = *const {l1030} u8
                    // 922: b"got job %d\0" ... st u8:   l1030 = UNIQUE | NON_NULL, (empty)
                    last,
                );
                count = 0 as libc::c_int;
                i = (*w).last + 1 as libc::c_int;
                while i <= last {
                    a = *assumptions.offset(i as isize);
                    // 928: *assumptions.of ... size): typeof(_141) = *mut {l204} i32
                    // 928: *assumptions.of ... size):   l204 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions.off ... size): typeof(_142) = *mut {l206} *mut {l207} i32
                    // 928: assumptions.off ... size):   l206 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions.off ... size):   l207 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions: typeof(_143) = *mut {l209} *mut {l210} i32
                    // 928: assumptions:   l209 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions:   l210 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions: typeof(_144) = *mut {l212} *mut {l213} *mut {l214} i32
                    // 928: assumptions:   l212 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions:   l213 = UNIQUE | NON_NULL, (empty)
                    // 928: assumptions:   l214 = UNIQUE | NON_NULL, (empty)
                    if addassumptions > 1 as libc::c_int && i < last {
                    // 929: addassumptions: typeof(_151) = *mut {l222} i32
                    // 929: addassumptions:   l222 = UNIQUE | NON_NULL, (empty)
                        p = a;
                        // 930: a: typeof(_156) = *mut {l228} i32
                        // 930: a:   l228 = UNIQUE | NON_NULL, (empty)
                        loop {
                            lit = *p;
                            if !(lit != 0) {
                                break;
                            }
                            lgladd((*w).lgl, -lit);
                            // 936: (*w).lgl: typeof(_165) = *mut {l238} LGL
                            // 936: (*w).lgl:   l238 = UNIQUE | NON_NULL, (empty)
                            p = p.offset(1);
                            // 937: p.offset(1): typeof(_169) = *mut {l243} i32
                            // 937: p.offset(1):   l243 = UNIQUE | NON_NULL, (empty)
                            // 937: p: typeof(_170) = *mut {l245} i32
                            // 937: p:   l245 = UNIQUE | NON_NULL, (empty)
                            p;
                            // 938: p: typeof(_171) = *mut {l247} i32
                            // 938: p:   l247 = UNIQUE | NON_NULL, (empty)
                        }
                        lgladd((*w).lgl, 0 as libc::c_int);
                        // 940: (*w).lgl: typeof(_173) = *mut {l250} LGL
                        // 940: (*w).lgl:   l250 = UNIQUE | NON_NULL, (empty)
                    }
                    if nomelt == 0 {
                    // 942: nomelt: typeof(_178) = *mut {l256} i32
                    // 942: nomelt:   l256 = UNIQUE | NON_NULL, (empty)
                        p = a;
                        // 943: a: typeof(_179) = *mut {l258} i32
                        // 943: a:   l258 = UNIQUE | NON_NULL, (empty)
                        loop {
                            lit = *p;
                            if !(lit != 0) {
                                break;
                            }
                            idx = abs(lit);
                            if *used.offset(idx as isize) == i {
                            // 950: used.offset(idx ... size): typeof(_191) = *mut {l271} i32
                            // 950: used.offset(idx ... size):   l271 = UNIQUE | NON_NULL, (empty)
                            // 950: used: typeof(_192) = *mut {l273} i32
                            // 950: used:   l273 = UNIQUE | NON_NULL, (empty)
                            // 950: used: typeof(_193) = *mut {l275} *mut {l276} i32
                            // 950: used:   l275 = UNIQUE | NON_NULL, (empty)
                            // 950: used:   l276 = UNIQUE | NON_NULL, (empty)
                                lglmelt((*w).lgl, idx);
                                // 951: (*w).lgl: typeof(_198) = *mut {l282} LGL
                                // 951: (*w).lgl:   l282 = UNIQUE | NON_NULL, (empty)
                                count += 1;
                                count;
                            }
                            p = p.offset(1);
                            // 955: p.offset(1): typeof(_202) = *mut {l287} i32
                            // 955: p.offset(1):   l287 = UNIQUE | NON_NULL, (empty)
                            // 955: p: typeof(_203) = *mut {l289} i32
                            // 955: p:   l289 = UNIQUE | NON_NULL, (empty)
                            p;
                            // 956: p: typeof(_204) = *mut {l291} i32
                            // 956: p:   l291 = UNIQUE | NON_NULL, (empty)
                        }
                    }
                    i += 1;
                    i;
                }
                msg(
                    w,
                    // 963: w: typeof(_211) = *mut {l299} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 963: w:   l299 = UNIQUE | NON_NULL, (empty)
                    2 as libc::c_int,
                    b"melted %d variables\0" as *const u8 as *const libc::c_char,
                    // 965: b"melted %d var ... _char: typeof(_213) = *const {l302} i8
                    // 965: b"melted %d var ... _char:   l302 = UNIQUE | NON_NULL, (empty)
                    // 965: b"melted %d var ... st u8: typeof(_214) = *const {l304} u8
                    // 965: b"melted %d var ... st u8:   l304 = UNIQUE | NON_NULL, (empty)
                    // 965: b"melted %d var ... es\0": typeof(_215) = *const {l306} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 965: b"melted %d var ... es\0":   l306 = UNIQUE | NON_NULL, (empty)
                    // 965: b"melted %d var ... es\0": typeof(_216) = & {l308} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 965: b"melted %d var ... es\0":   l308 = UNIQUE | NON_NULL, FIXED
                    // 965: b"melted %d var ... _char: typeof(_213 = move _214 as *const i8 (Misc)) = *const {l1035} i8
                    // 965: b"melted %d var ... _char:   l1035 = UNIQUE | NON_NULL, (empty)
                    // 965: b"melted %d var ... es\0": typeof(_216 = const b"melted %d variables\x00") = & {l1032} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 965: b"melted %d var ... es\0":   l1032 = UNIQUE | NON_NULL, (empty)
                    // 965: b"melted %d var ... es\0": typeof(_215 = &raw const (*_216)) = *const {l1033} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                    // 965: b"melted %d var ... es\0":   l1033 = UNIQUE | NON_NULL, (empty)
                    // 965: b"melted %d var ... st u8: typeof(_214 = move _215 as *const u8 (Pointer(ArrayToPointer))) = *const {l1034} u8
                    // 965: b"melted %d var ... st u8:   l1034 = UNIQUE | NON_NULL, (empty)
                    count,
                );
                (*w).last = last;
                a = *assumptions.offset((*w).last as isize);
                // 969: *assumptions.of ... size): typeof(_219) = *mut {l312} i32
                // 969: *assumptions.of ... size):   l312 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions.off ... size): typeof(_220) = *mut {l314} *mut {l315} i32
                // 969: assumptions.off ... size):   l314 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions.off ... size):   l315 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions: typeof(_221) = *mut {l317} *mut {l318} i32
                // 969: assumptions:   l317 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions:   l318 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions: typeof(_222) = *mut {l320} *mut {l321} *mut {l322} i32
                // 969: assumptions:   l320 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions:   l321 = UNIQUE | NON_NULL, (empty)
                // 969: assumptions:   l322 = UNIQUE | NON_NULL, (empty)
                if noreverse != 0 {
                // 970: noreverse: typeof(_228) = *mut {l329} i32
                // 970: noreverse:   l329 = UNIQUE | NON_NULL, (empty)
                    p = a;
                    // 971: a: typeof(_229) = *mut {l331} i32
                    // 971: a:   l331 = UNIQUE | NON_NULL, (empty)
                    loop {
                        lit = *p;
                        if !(lit != 0) {
                            break;
                        }
                        lglassume((*w).lgl, lit);
                        // 977: (*w).lgl: typeof(_237) = *mut {l340} LGL
                        // 977: (*w).lgl:   l340 = UNIQUE | NON_NULL, (empty)
                        p = p.offset(1);
                        // 978: p.offset(1): typeof(_239) = *mut {l343} i32
                        // 978: p.offset(1):   l343 = UNIQUE | NON_NULL, (empty)
                        // 978: p: typeof(_240) = *mut {l345} i32
                        // 978: p:   l345 = UNIQUE | NON_NULL, (empty)
                        p;
                        // 979: p: typeof(_241) = *mut {l347} i32
                        // 979: p:   l347 = UNIQUE | NON_NULL, (empty)
                    }
                } else {
                    i = 0 as libc::c_int;
                    while *a.offset(i as isize) != 0 {
                    // 983: a.offset(i as isize): typeof(_246) = *mut {l353} i32
                    // 983: a.offset(i as isize):   l353 = UNIQUE | NON_NULL, (empty)
                    // 983: a: typeof(_247) = *mut {l355} i32
                    // 983: a:   l355 = UNIQUE | NON_NULL, (empty)
                        i += 1;
                        i;
                    }
                    p = a.offset(i as isize).offset(-(1 as libc::c_int as isize));
                    // 987: a.offset(i as i ... ize)): typeof(_255) = *mut {l364} i32
                    // 987: a.offset(i as i ... ize)):   l364 = UNIQUE | NON_NULL, (empty)
                    // 987: a.offset(i as isize): typeof(_256) = *mut {l366} i32
                    // 987: a.offset(i as isize):   l366 = UNIQUE | NON_NULL, (empty)
                    // 987: a: typeof(_257) = *mut {l368} i32
                    // 987: a:   l368 = UNIQUE | NON_NULL, (empty)
                    while p >= a {
                    // 988: p: typeof(_265) = *mut {l377} i32
                    // 988: p:   l377 = UNIQUE | NON_NULL, (empty)
                    // 988: a: typeof(_266) = *mut {l379} i32
                    // 988: a:   l379 = UNIQUE | NON_NULL, (empty)
                        lglassume((*w).lgl, *p);
                        // 989: (*w).lgl: typeof(_268) = *mut {l382} LGL
                        // 989: (*w).lgl:   l382 = UNIQUE | NON_NULL, (empty)
                        p = p.offset(-1);
                        // 990: p.offset(-1): typeof(_270) = *mut {l385} i32
                        // 990: p.offset(-1):   l385 = UNIQUE | NON_NULL, (empty)
                        // 990: p: typeof(_271) = *mut {l387} i32
                        // 990: p:   l387 = UNIQUE | NON_NULL, (empty)
                        p;
                        // 991: p: typeof(_272) = *mut {l389} i32
                        // 991: p:   l389 = UNIQUE | NON_NULL, (empty)
                    }
                }
                start_0 = getime();
                (*w).res = sat(w);
                // 995: w: typeof(_278) = *mut {l396} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 995: w:   l396 = UNIQUE | NON_NULL, (empty)
                end = getime();
                delta = end - start_0;
                delta = if delta <= 0 as libc::c_int as libc::c_double {
                    0 as libc::c_int as libc::c_double
                } else {
                    delta
                };
                *times.offset(last as isize) = delta;
                // 1003: times.offset(la ... size): typeof(_289) = *mut {l408} f64
                // 1003: times.offset(la ... size):   l408 = UNIQUE | NON_NULL, (empty)
                // 1003: times: typeof(_290) = *mut {l410} f64
                // 1003: times:   l410 = UNIQUE | NON_NULL, (empty)
                // 1003: times: typeof(_291) = *mut {l412} *mut {l413} f64
                // 1003: times:   l412 = UNIQUE | NON_NULL, (empty)
                // 1003: times:   l413 = UNIQUE | NON_NULL, (empty)
                if bar != 0 {
                // 1004: bar: typeof(_297) = *mut {l420} i32
                // 1004: bar:   l420 = UNIQUE | NON_NULL, (empty)
                    pthread_mutex_lock(&mut finishedmutex);
                    // 1005: &mut finishedmutex: typeof(_299) = *mut {l423} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1005: &mut finishedmutex:   l423 = UNIQUE | NON_NULL, (empty)
                    // 1005: &mut finishedmutex: typeof(_300) = &mut {l425} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1005: &mut finishedmutex:   l425 = UNIQUE | NON_NULL, (empty)
                    // 1005: finishedmutex: typeof(_301) = *mut {l427} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1005: finishedmutex:   l427 = UNIQUE | NON_NULL, (empty)
                    // 1005: &mut finishedmutex: typeof(_300 = &mut (*_301)) = &mut {l1036} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1005: &mut finishedmutex:   l1036 = UNIQUE | NON_NULL, (empty)
                    // 1005: &mut finishedmutex: typeof(_299 = &raw mut (*_300)) = *mut {l1037} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1005: &mut finishedmutex:   l1037 = UNIQUE | NON_NULL, (empty)
                    finished += 1;
                    // 1006: finished: typeof(_302) = *mut {l429} i32
                    // 1006: finished:   l429 = UNIQUE | NON_NULL, (empty)
                    fin = finished;
                    // 1007: finished: typeof(_305) = *mut {l433} i32
                    // 1007: finished:   l433 = UNIQUE | NON_NULL, (empty)
                    sumtimes += delta;
                    // 1008: sumtimes: typeof(_307) = *mut {l436} f64
                    // 1008: sumtimes:   l436 = UNIQUE | NON_NULL, (empty)
                    avg = sumtimes / fin as libc::c_double;
                    // 1009: sumtimes: typeof(_309) = *mut {l439} f64
                    // 1009: sumtimes:   l439 = UNIQUE | NON_NULL, (empty)
                    pthread_mutex_unlock(&mut finishedmutex);
                    // 1010: &mut finishedmutex: typeof(_313) = *mut {l444} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1010: &mut finishedmutex:   l444 = UNIQUE | NON_NULL, (empty)
                    // 1010: &mut finishedmutex: typeof(_314) = &mut {l446} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1010: &mut finishedmutex:   l446 = UNIQUE | NON_NULL, (empty)
                    // 1010: finishedmutex: typeof(_315) = *mut {l448} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1010: finishedmutex:   l448 = UNIQUE | NON_NULL, (empty)
                    // 1010: &mut finishedmutex: typeof(_313 = &raw mut (*_314)) = *mut {l1039} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1010: &mut finishedmutex:   l1039 = UNIQUE | NON_NULL, (empty)
                    // 1010: &mut finishedmutex: typeof(_314 = &mut (*_315)) = &mut {l1038} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1010: &mut finishedmutex:   l1038 = UNIQUE | NON_NULL, (empty)
                    pm = 1000 as libc::c_int * (fin - 1 as libc::c_int) / nassumptions;
                    // 1011: nassumptions: typeof(_324) = *mut {l458} i32
                    // 1011: nassumptions:   l458 = UNIQUE | NON_NULL, (empty)
                    lm = 1000 as libc::c_int * fin / nassumptions;
                    // 1012: nassumptions: typeof(_334) = *mut {l469} i32
                    // 1012: nassumptions:   l469 = UNIQUE | NON_NULL, (empty)
                    if pm < lm {
                        progress(lm, fin, nassumptions, avg, 0 as libc::c_int);
                        // 1014: nassumptions: typeof(_346) = *mut {l482} i32
                        // 1014: nassumptions:   l482 = UNIQUE | NON_NULL, (empty)
                    }
                }
                if (*w).res == 10 as libc::c_int {
                    if bar == 0 {
                    // 1018: bar: typeof(_355) = *mut {l492} i32
                    // 1018: bar:   l492 = UNIQUE | NON_NULL, (empty)
                        msg(
                            w,
                            // 1020: w: typeof(_357) = *mut {l495} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1020: w:   l495 = UNIQUE | NON_NULL, (empty)
                            1 as libc::c_int,
                            b"job %d SATISFIABLE\0" as *const u8 as *const libc::c_char,
                            // 1022: b"job %d SATISF ... _char: typeof(_359) = *const {l498} i8
                            // 1022: b"job %d SATISF ... _char:   l498 = UNIQUE | NON_NULL, (empty)
                            // 1022: b"job %d SATISF ... st u8: typeof(_360) = *const {l500} u8
                            // 1022: b"job %d SATISF ... st u8:   l500 = UNIQUE | NON_NULL, (empty)
                            // 1022: b"job %d SATISF ... LE\0": typeof(_361) = *const {l502} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1022: b"job %d SATISF ... LE\0":   l502 = UNIQUE | NON_NULL, (empty)
                            // 1022: b"job %d SATISF ... LE\0": typeof(_362) = & {l504} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1022: b"job %d SATISF ... LE\0":   l504 = UNIQUE | NON_NULL, FIXED
                            // 1022: b"job %d SATISF ... _char: typeof(_359 = move _360 as *const i8 (Misc)) = *const {l1043} i8
                            // 1022: b"job %d SATISF ... _char:   l1043 = UNIQUE | NON_NULL, (empty)
                            // 1022: b"job %d SATISF ... LE\0": typeof(_362 = const b"job %d SATISFIABLE\x00") = & {l1040} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1022: b"job %d SATISF ... LE\0":   l1040 = UNIQUE | NON_NULL, (empty)
                            // 1022: b"job %d SATISF ... st u8: typeof(_360 = move _361 as *const u8 (Pointer(ArrayToPointer))) = *const {l1042} u8
                            // 1022: b"job %d SATISF ... st u8:   l1042 = UNIQUE | NON_NULL, (empty)
                            // 1022: b"job %d SATISF ... LE\0": typeof(_361 = &raw const (*_362)) = *const {l1041} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
                            // 1022: b"job %d SATISF ... LE\0":   l1041 = UNIQUE | NON_NULL, (empty)
                            last,
                        );
                    }
                    if pthread_mutex_lock(&mut donemutex) != 0 {
                    // 1026: &mut donemutex: typeof(_367) = *mut {l510} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1026: &mut donemutex:   l510 = UNIQUE | NON_NULL, (empty)
                    // 1026: &mut donemutex: typeof(_368) = &mut {l512} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1026: &mut donemutex:   l512 = UNIQUE | NON_NULL, (empty)
                    // 1026: donemutex: typeof(_369) = *mut {l514} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1026: donemutex:   l514 = UNIQUE | NON_NULL, (empty)
                    // 1026: &mut donemutex: typeof(_367 = &raw mut (*_368)) = *mut {l1045} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1026: &mut donemutex:   l1045 = UNIQUE | NON_NULL, (empty)
                    // 1026: &mut donemutex: typeof(_368 = &mut (*_369)) = &mut {l1044} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1026: &mut donemutex:   l1044 = UNIQUE | NON_NULL, (empty)
                        warn(
                            b"worker %d failed to lock 'done' mutex\0" as *const u8
                            // 1028: b"worker %d fai ... _char: typeof(_371) = *const {l517} i8
                            // 1028: b"worker %d fai ... _char:   l517 = UNIQUE | NON_NULL, (empty)
                            // 1028: b"worker %d fai ... st u8: typeof(_372) = *const {l519} u8
                            // 1028: b"worker %d fai ... st u8:   l519 = UNIQUE | NON_NULL, (empty)
                            // 1028: b"worker %d fai ... ex\0": typeof(_373) = *const {l521} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1028: b"worker %d fai ... ex\0":   l521 = UNIQUE | NON_NULL, (empty)
                            // 1028: b"worker %d fai ... ex\0": typeof(_374) = & {l523} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1028: b"worker %d fai ... ex\0":   l523 = UNIQUE | NON_NULL, FIXED
                            // 1028: b"worker %d fai ... ex\0": typeof(_373 = &raw const (*_374)) = *const {l1047} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1028: b"worker %d fai ... ex\0":   l1047 = UNIQUE | NON_NULL, (empty)
                            // 1028: b"worker %d fai ... ex\0": typeof(_374 = const b"worker %d failed to lock \'done\' mutex\x00") = & {l1046} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1028: b"worker %d fai ... ex\0":   l1046 = UNIQUE | NON_NULL, (empty)
                            // 1028: b"worker %d fai ... _char: typeof(_371 = move _372 as *const i8 (Misc)) = *const {l1049} i8
                            // 1028: b"worker %d fai ... _char:   l1049 = UNIQUE | NON_NULL, (empty)
                            // 1028: b"worker %d fai ... st u8: typeof(_372 = move _373 as *const u8 (Pointer(ArrayToPointer))) = *const {l1048} u8
                            // 1028: b"worker %d fai ... st u8:   l1048 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            w.offset_from(workers) as libc::c_long as libc::c_int,
                            // 1030: w: typeof(_378) = *mut {l528} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1030: w:   l528 = UNIQUE | NON_NULL, (empty)
                            // 1030: workers: typeof(_379) = *const {l530} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1030: workers:   l530 = UNIQUE | NON_NULL, (empty)
                            // 1030: workers: typeof(_380) = *mut {l532} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1030: workers:   l532 = UNIQUE | NON_NULL, (empty)
                            // 1030: workers: typeof(_381) = *mut {l534} *mut {l535} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1030: workers:   l534 = UNIQUE | NON_NULL, (empty)
                            // 1030: workers:   l535 = UNIQUE | NON_NULL, (empty)
                            // 1030: workers: typeof(_379 = move _380 as *const Worker (Pointer(MutToConstPointer))) = *const {l1050} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1030: workers:   l1050 = UNIQUE | NON_NULL, (empty)
                        );
                    }
                    done = 1 as libc::c_int;
                    // 1033: done: typeof(_383) = *mut {l538} i32
                    // 1033: done:   l538 = UNIQUE | NON_NULL, (empty)
                    if pthread_mutex_unlock(&mut donemutex) != 0 {
                    // 1034: &mut donemutex: typeof(_386) = *mut {l542} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1034: &mut donemutex:   l542 = UNIQUE | NON_NULL, (empty)
                    // 1034: &mut donemutex: typeof(_387) = &mut {l544} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1034: &mut donemutex:   l544 = UNIQUE | NON_NULL, (empty)
                    // 1034: donemutex: typeof(_388) = *mut {l546} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1034: donemutex:   l546 = UNIQUE | NON_NULL, (empty)
                    // 1034: &mut donemutex: typeof(_386 = &raw mut (*_387)) = *mut {l1052} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1034: &mut donemutex:   l1052 = UNIQUE | NON_NULL, (empty)
                    // 1034: &mut donemutex: typeof(_387 = &mut (*_388)) = &mut {l1051} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                    // 1034: &mut donemutex:   l1051 = UNIQUE | NON_NULL, (empty)
                        warn(
                            b"worker %d failed to unlock 'done' mutex\0" as *const u8
                            // 1036: b"worker %d fai ... _char: typeof(_390) = *const {l549} i8
                            // 1036: b"worker %d fai ... _char:   l549 = UNIQUE | NON_NULL, (empty)
                            // 1036: b"worker %d fai ... st u8: typeof(_391) = *const {l551} u8
                            // 1036: b"worker %d fai ... st u8:   l551 = UNIQUE | NON_NULL, (empty)
                            // 1036: b"worker %d fai ... ex\0": typeof(_392) = *const {l553} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                            // 1036: b"worker %d fai ... ex\0":   l553 = UNIQUE | NON_NULL, (empty)
                            // 1036: b"worker %d fai ... ex\0": typeof(_393) = & {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                            // 1036: b"worker %d fai ... ex\0":   l555 = UNIQUE | NON_NULL, FIXED
                            // 1036: b"worker %d fai ... _char: typeof(_390 = move _391 as *const i8 (Misc)) = *const {l1056} i8
                            // 1036: b"worker %d fai ... _char:   l1056 = UNIQUE | NON_NULL, (empty)
                            // 1036: b"worker %d fai ... ex\0": typeof(_392 = &raw const (*_393)) = *const {l1054} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                            // 1036: b"worker %d fai ... ex\0":   l1054 = UNIQUE | NON_NULL, (empty)
                            // 1036: b"worker %d fai ... ex\0": typeof(_393 = const b"worker %d failed to unlock \'done\' mutex\x00") = & {l1053} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                            // 1036: b"worker %d fai ... ex\0":   l1053 = UNIQUE | NON_NULL, (empty)
                            // 1036: b"worker %d fai ... st u8: typeof(_391 = move _392 as *const u8 (Pointer(ArrayToPointer))) = *const {l1055} u8
                            // 1036: b"worker %d fai ... st u8:   l1055 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            w.offset_from(workers) as libc::c_long as libc::c_int,
                            // 1038: w: typeof(_397) = *mut {l560} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1038: w:   l560 = UNIQUE | NON_NULL, (empty)
                            // 1038: workers: typeof(_398) = *const {l562} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1038: workers:   l562 = UNIQUE | NON_NULL, (empty)
                            // 1038: workers: typeof(_399) = *mut {l564} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1038: workers:   l564 = UNIQUE | NON_NULL, (empty)
                            // 1038: workers: typeof(_400) = *mut {l566} *mut {l567} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1038: workers:   l566 = UNIQUE | NON_NULL, (empty)
                            // 1038: workers:   l567 = UNIQUE | NON_NULL, (empty)
                            // 1038: workers: typeof(_398 = move _399 as *const Worker (Pointer(MutToConstPointer))) = *const {l1057} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1038: workers:   l1057 = UNIQUE | NON_NULL, (empty)
                        );
                    }
                } else if (*w).res == 20 as libc::c_int {
                    (*w).nfailed = 0 as libc::c_int;
                    if ((*w).failed).is_null() {
                    // 1043: ((*w).failed): typeof(_407) = *mut {l575} i32
                    // 1043: ((*w).failed):   l575 = UNIQUE | NON_NULL, (empty)
                        let mut BYTES: size_t = (maxassumptionsize as libc::c_ulong)
                        // 1044: maxassumptionsize: typeof(_411) = *mut {l580} i32
                        // 1044: maxassumptionsize:   l580 = UNIQUE | NON_NULL, (empty)
                            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
                        (*w).failed = malloc(BYTES) as *mut libc::c_int;
                        // 1046: malloc(BYTES): typeof(_414) = *mut {l584} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1046: malloc(BYTES):   l584 = UNIQUE | NON_NULL, (empty)
                        // 1046: (*w).failed = m ... c_int: typeof(((*_18).7: *mut i32) = move _414 as *mut i32 (Misc)) = *mut {l1058} i32
                        // 1046: (*w).failed = m ... c_int:   l1058 = UNIQUE | NON_NULL, (empty)
                        if ((*w).failed).is_null() {
                        // 1047: ((*w).failed): typeof(_418) = *mut {l589} i32
                        // 1047: ((*w).failed):   l589 = UNIQUE | NON_NULL, (empty)
                            die(b"out of memory\0" as *const u8 as *const libc::c_char);
                            // 1048: b"out of memory ... _char: typeof(_421) = *const {l593} i8
                            // 1048: b"out of memory ... _char:   l593 = UNIQUE | NON_NULL, (empty)
                            // 1048: b"out of memory ... st u8: typeof(_422) = *const {l595} u8
                            // 1048: b"out of memory ... st u8:   l595 = UNIQUE | NON_NULL, (empty)
                            // 1048: b"out of memory\0": typeof(_423) = *const {l597} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1048: b"out of memory\0":   l597 = UNIQUE | NON_NULL, (empty)
                            // 1048: b"out of memory\0": typeof(_424) = & {l599} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1048: b"out of memory\0":   l599 = UNIQUE | NON_NULL, FIXED
                            // 1048: b"out of memory ... _char: typeof(_421 = move _422 as *const i8 (Misc)) = *const {l1062} i8
                            // 1048: b"out of memory ... _char:   l1062 = UNIQUE | NON_NULL, (empty)
                            // 1048: b"out of memory\0": typeof(_423 = &raw const (*_424)) = *const {l1060} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1048: b"out of memory\0":   l1060 = UNIQUE | NON_NULL, (empty)
                            // 1048: b"out of memory ... st u8: typeof(_422 = move _423 as *const u8 (Pointer(ArrayToPointer))) = *const {l1061} u8
                            // 1048: b"out of memory ... st u8:   l1061 = UNIQUE | NON_NULL, (empty)
                            // 1048: b"out of memory\0": typeof(_424 = const b"out of memory\x00") = & {l1059} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1048: b"out of memory\0":   l1059 = UNIQUE | NON_NULL, (empty)
                            exit(1 as libc::c_int);
                        }
                        memset((*w).failed as *mut libc::c_void, 0 as libc::c_int, BYTES);
                        // 1051: memset((*w).fai ... YTES): typeof(_427) = *mut {l603} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1051: memset((*w).fai ... YTES):   l603 = UNIQUE | NON_NULL, (empty)
                        // 1051: (*w).failed as  ... _void: typeof(_428) = *mut {l605} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1051: (*w).failed as  ... _void:   l605 = UNIQUE | NON_NULL, (empty)
                        // 1051: (*w).failed: typeof(_429) = *mut {l607} i32
                        // 1051: (*w).failed:   l607 = UNIQUE | NON_NULL, (empty)
                        // 1051: (*w).failed as  ... _void: typeof(_428 = move _429 as *mut libc::c_void (Misc)) = *mut {l1063} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1051: (*w).failed as  ... _void:   l1063 = UNIQUE | NON_NULL, (empty)
                        allocated = allocated.wrapping_add(BYTES);
                        // 1052: allocated: typeof(_434) = *mut {l613} u64
                        // 1052: allocated:   l613 = UNIQUE | NON_NULL, (empty)
                        // 1052: allocated: typeof(_436) = *mut {l616} u64
                        // 1052: allocated:   l616 = UNIQUE | NON_NULL, (empty)
                        if allocated > maxallocated {
                        // 1053: allocated: typeof(_439) = *mut {l620} u64
                        // 1053: allocated:   l620 = UNIQUE | NON_NULL, (empty)
                        // 1053: maxallocated: typeof(_441) = *mut {l623} u64
                        // 1053: maxallocated:   l623 = UNIQUE | NON_NULL, (empty)
                            maxallocated = allocated;
                            // 1054: allocated: typeof(_443) = *mut {l626} u64
                            // 1054: allocated:   l626 = UNIQUE | NON_NULL, (empty)
                            // 1054: maxallocated: typeof(_444) = *mut {l628} u64
                            // 1054: maxallocated:   l628 = UNIQUE | NON_NULL, (empty)
                        }
                    }
                    p = a;
                    // 1057: a: typeof(_445) = *mut {l630} i32
                    // 1057: a:   l630 = UNIQUE | NON_NULL, (empty)
                    loop {
                        lit = *p;
                        if !(lit != 0) {
                            break;
                        }
                        if lglfailed((*w).lgl, lit) != 0 {
                        // 1063: (*w).lgl: typeof(_456) = *mut {l642} LGL
                        // 1063: (*w).lgl:   l642 = UNIQUE | NON_NULL, (empty)
                            let fresh4 = (*w).nfailed;
                            (*w).nfailed = (*w).nfailed + 1;
                            *((*w).failed).offset(fresh4 as isize) = lit;
                            // 1066: ((*w).failed).o ... size): typeof(_462) = *mut {l649} i32
                            // 1066: ((*w).failed).o ... size):   l649 = UNIQUE | NON_NULL, (empty)
                            // 1066: ((*w).failed): typeof(_463) = *mut {l651} i32
                            // 1066: ((*w).failed):   l651 = UNIQUE | NON_NULL, (empty)
                        }
                        p = p.offset(1);
                        // 1068: p.offset(1): typeof(_466) = *mut {l655} i32
                        // 1068: p.offset(1):   l655 = UNIQUE | NON_NULL, (empty)
                        // 1068: p: typeof(_467) = *mut {l657} i32
                        // 1068: p:   l657 = UNIQUE | NON_NULL, (empty)
                        p;
                        // 1069: p: typeof(_468) = *mut {l659} i32
                        // 1069: p:   l659 = UNIQUE | NON_NULL, (empty)
                    }
                    if !druptraceprefix.is_null() {
                    // 1071: druptraceprefix: typeof(_472) = *const {l664} i8
                    // 1071: druptraceprefix:   l664 = UNIQUE | NON_NULL, (empty)
                    // 1071: druptraceprefix: typeof(_473) = *mut {l666} *const {l667} i8
                    // 1071: druptraceprefix:   l666 = UNIQUE | NON_NULL, (empty)
                    // 1071: druptraceprefix:   l667 = UNIQUE | NON_NULL, (empty)
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            fprintf(
                                (*w).proof,
                                // 1075: (*w).proof: typeof(_480) = *mut {l675} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                                // 1075: (*w).proof:   l675 = UNIQUE | NON_NULL, (empty)
                                b"%d \0" as *const u8 as *const libc::c_char,
                                // 1076: b"%d \0" as *co ... _char: typeof(_481) = *const {l677} i8
                                // 1076: b"%d \0" as *co ... _char:   l677 = UNIQUE | NON_NULL, (empty)
                                // 1076: b"%d \0" as *co ... st u8: typeof(_482) = *const {l679} u8
                                // 1076: b"%d \0" as *co ... st u8:   l679 = UNIQUE | NON_NULL, (empty)
                                // 1076: b"%d \0": typeof(_483) = *const {l681} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1076: b"%d \0":   l681 = UNIQUE | NON_NULL, (empty)
                                // 1076: b"%d \0": typeof(_484) = & {l683} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1076: b"%d \0":   l683 = UNIQUE | NON_NULL, FIXED
                                // 1076: b"%d \0": typeof(_484 = const b"%d \x00") = & {l1064} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1076: b"%d \0":   l1064 = UNIQUE | NON_NULL, (empty)
                                // 1076: b"%d \0": typeof(_483 = &raw const (*_484)) = *const {l1065} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1076: b"%d \0":   l1065 = UNIQUE | NON_NULL, (empty)
                                // 1076: b"%d \0" as *co ... st u8: typeof(_482 = move _483 as *const u8 (Pointer(ArrayToPointer))) = *const {l1066} u8
                                // 1076: b"%d \0" as *co ... st u8:   l1066 = UNIQUE | NON_NULL, (empty)
                                // 1076: b"%d \0" as *co ... _char: typeof(_481 = move _482 as *const i8 (Misc)) = *const {l1067} i8
                                // 1076: b"%d \0" as *co ... _char:   l1067 = UNIQUE | NON_NULL, (empty)
                                -*((*w).failed).offset(i as isize),
                                // 1077: ((*w).failed).o ... size): typeof(_487) = *mut {l687} i32
                                // 1077: ((*w).failed).o ... size):   l687 = UNIQUE | NON_NULL, (empty)
                                // 1077: ((*w).failed): typeof(_488) = *mut {l689} i32
                                // 1077: ((*w).failed):   l689 = UNIQUE | NON_NULL, (empty)
                            );
                            i += 1;
                            i;
                        }
                        fputs(b"0\n\0" as *const u8 as *const libc::c_char, (*w).proof);
                        // 1082: b"0\n\0" as *co ... _char: typeof(_498) = *const {l700} i8
                        // 1082: b"0\n\0" as *co ... _char:   l700 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0" as *co ... st u8: typeof(_499) = *const {l702} u8
                        // 1082: b"0\n\0" as *co ... st u8:   l702 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0": typeof(_500) = *const {l704} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1082: b"0\n\0":   l704 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0": typeof(_501) = & {l706} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1082: b"0\n\0":   l706 = UNIQUE | NON_NULL, FIXED
                        // 1082: (*w).proof: typeof(_502) = *mut {l708} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                        // 1082: (*w).proof:   l708 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0": typeof(_500 = &raw const (*_501)) = *const {l1069} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1082: b"0\n\0":   l1069 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0": typeof(_501 = const b"0\n\x00") = & {l1068} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1082: b"0\n\0":   l1068 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0" as *co ... _char: typeof(_498 = move _499 as *const i8 (Misc)) = *const {l1071} i8
                        // 1082: b"0\n\0" as *co ... _char:   l1071 = UNIQUE | NON_NULL, (empty)
                        // 1082: b"0\n\0" as *co ... st u8: typeof(_499 = move _500 as *const u8 (Pointer(ArrayToPointer))) = *const {l1070} u8
                        // 1082: b"0\n\0" as *co ... st u8:   l1070 = UNIQUE | NON_NULL, (empty)
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            fprintf(
                                (*w).proof,
                                // 1086: (*w).proof: typeof(_509) = *mut {l716} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                                // 1086: (*w).proof:   l716 = UNIQUE | NON_NULL, (empty)
                                b"%d \0" as *const u8 as *const libc::c_char,
                                // 1087: b"%d \0" as *co ... _char: typeof(_510) = *const {l718} i8
                                // 1087: b"%d \0" as *co ... _char:   l718 = UNIQUE | NON_NULL, (empty)
                                // 1087: b"%d \0" as *co ... st u8: typeof(_511) = *const {l720} u8
                                // 1087: b"%d \0" as *co ... st u8:   l720 = UNIQUE | NON_NULL, (empty)
                                // 1087: b"%d \0": typeof(_512) = *const {l722} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1087: b"%d \0":   l722 = UNIQUE | NON_NULL, (empty)
                                // 1087: b"%d \0": typeof(_513) = & {l724} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1087: b"%d \0":   l724 = UNIQUE | NON_NULL, FIXED
                                // 1087: b"%d \0": typeof(_512 = &raw const (*_513)) = *const {l1073} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1087: b"%d \0":   l1073 = UNIQUE | NON_NULL, (empty)
                                // 1087: b"%d \0" as *co ... _char: typeof(_510 = move _511 as *const i8 (Misc)) = *const {l1075} i8
                                // 1087: b"%d \0" as *co ... _char:   l1075 = UNIQUE | NON_NULL, (empty)
                                // 1087: b"%d \0" as *co ... st u8: typeof(_511 = move _512 as *const u8 (Pointer(ArrayToPointer))) = *const {l1074} u8
                                // 1087: b"%d \0" as *co ... st u8:   l1074 = UNIQUE | NON_NULL, (empty)
                                // 1087: b"%d \0": typeof(_513 = const b"%d \x00") = & {l1072} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1087: b"%d \0":   l1072 = UNIQUE | NON_NULL, (empty)
                                -*((*w).failed).offset(i as isize),
                                // 1088: ((*w).failed).o ... size): typeof(_516) = *mut {l728} i32
                                // 1088: ((*w).failed).o ... size):   l728 = UNIQUE | NON_NULL, (empty)
                                // 1088: ((*w).failed): typeof(_517) = *mut {l730} i32
                                // 1088: ((*w).failed):   l730 = UNIQUE | NON_NULL, (empty)
                            );
                            i += 1;
                            i;
                        }
                        fputs(b"0\n\0" as *const u8 as *const libc::c_char, (*w).proof);
                        // 1093: b"0\n\0" as *co ... _char: typeof(_527) = *const {l741} i8
                        // 1093: b"0\n\0" as *co ... _char:   l741 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0" as *co ... st u8: typeof(_528) = *const {l743} u8
                        // 1093: b"0\n\0" as *co ... st u8:   l743 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0": typeof(_529) = *const {l745} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1093: b"0\n\0":   l745 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0": typeof(_530) = & {l747} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1093: b"0\n\0":   l747 = UNIQUE | NON_NULL, FIXED
                        // 1093: (*w).proof: typeof(_531) = *mut {l749} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                        // 1093: (*w).proof:   l749 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0": typeof(_529 = &raw const (*_530)) = *const {l1077} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1093: b"0\n\0":   l1077 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0": typeof(_530 = const b"0\n\x00") = & {l1076} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1093: b"0\n\0":   l1076 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0" as *co ... _char: typeof(_527 = move _528 as *const i8 (Misc)) = *const {l1079} i8
                        // 1093: b"0\n\0" as *co ... _char:   l1079 = UNIQUE | NON_NULL, (empty)
                        // 1093: b"0\n\0" as *co ... st u8: typeof(_528 = move _529 as *const u8 (Pointer(ArrayToPointer))) = *const {l1078} u8
                        // 1093: b"0\n\0" as *co ... st u8:   l1078 = UNIQUE | NON_NULL, (empty)
                    }
                    if addassumptions != 0 {
                    // 1095: addassumptions: typeof(_535) = *mut {l754} i32
                    // 1095: addassumptions:   l754 = UNIQUE | NON_NULL, (empty)
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            lgladd((*w).lgl, -*((*w).failed).offset(i as isize));
                            // 1098: (*w).lgl: typeof(_542) = *mut {l762} LGL
                            // 1098: (*w).lgl:   l762 = UNIQUE | NON_NULL, (empty)
                            // 1098: ((*w).failed).o ... size): typeof(_545) = *mut {l766} i32
                            // 1098: ((*w).failed).o ... size):   l766 = UNIQUE | NON_NULL, (empty)
                            // 1098: ((*w).failed): typeof(_546) = *mut {l768} i32
                            // 1098: ((*w).failed):   l768 = UNIQUE | NON_NULL, (empty)
                            i += 1;
                            i;
                        }
                        lgladd((*w).lgl, 0 as libc::c_int);
                        // 1102: (*w).lgl: typeof(_556) = *mut {l779} LGL
                        // 1102: (*w).lgl:   l779 = UNIQUE | NON_NULL, (empty)
                    }
                    if !druptraceprefix.is_null() {
                    // 1104: druptraceprefix: typeof(_561) = *const {l785} i8
                    // 1104: druptraceprefix:   l785 = UNIQUE | NON_NULL, (empty)
                    // 1104: druptraceprefix: typeof(_562) = *mut {l787} *const {l788} i8
                    // 1104: druptraceprefix:   l787 = UNIQUE | NON_NULL, (empty)
                    // 1104: druptraceprefix:   l788 = UNIQUE | NON_NULL, (empty)
                        i = 0 as libc::c_int;
                        while i < (*w).nfailed {
                            fprintf(
                                (*w).post,
                                // 1108: (*w).post: typeof(_569) = *mut {l796} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                                // 1108: (*w).post:   l796 = UNIQUE | NON_NULL, (empty)
                                b"%d \0" as *const u8 as *const libc::c_char,
                                // 1109: b"%d \0" as *co ... _char: typeof(_570) = *const {l798} i8
                                // 1109: b"%d \0" as *co ... _char:   l798 = UNIQUE | NON_NULL, (empty)
                                // 1109: b"%d \0" as *co ... st u8: typeof(_571) = *const {l800} u8
                                // 1109: b"%d \0" as *co ... st u8:   l800 = UNIQUE | NON_NULL, (empty)
                                // 1109: b"%d \0": typeof(_572) = *const {l802} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1109: b"%d \0":   l802 = UNIQUE | NON_NULL, (empty)
                                // 1109: b"%d \0": typeof(_573) = & {l804} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1109: b"%d \0":   l804 = UNIQUE | NON_NULL, FIXED
                                // 1109: b"%d \0" as *co ... _char: typeof(_570 = move _571 as *const i8 (Misc)) = *const {l1083} i8
                                // 1109: b"%d \0" as *co ... _char:   l1083 = UNIQUE | NON_NULL, (empty)
                                // 1109: b"%d \0": typeof(_573 = const b"%d \x00") = & {l1080} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1109: b"%d \0":   l1080 = UNIQUE | NON_NULL, (empty)
                                // 1109: b"%d \0": typeof(_572 = &raw const (*_573)) = *const {l1081} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
                                // 1109: b"%d \0":   l1081 = UNIQUE | NON_NULL, (empty)
                                // 1109: b"%d \0" as *co ... st u8: typeof(_571 = move _572 as *const u8 (Pointer(ArrayToPointer))) = *const {l1082} u8
                                // 1109: b"%d \0" as *co ... st u8:   l1082 = UNIQUE | NON_NULL, (empty)
                                -*((*w).failed).offset(i as isize),
                                // 1110: ((*w).failed).o ... size): typeof(_576) = *mut {l808} i32
                                // 1110: ((*w).failed).o ... size):   l808 = UNIQUE | NON_NULL, (empty)
                                // 1110: ((*w).failed): typeof(_577) = *mut {l810} i32
                                // 1110: ((*w).failed):   l810 = UNIQUE | NON_NULL, (empty)
                            );
                            i += 1;
                            i;
                        }
                        fputs(b"0\n\0" as *const u8 as *const libc::c_char, (*w).post);
                        // 1115: b"0\n\0" as *co ... _char: typeof(_587) = *const {l821} i8
                        // 1115: b"0\n\0" as *co ... _char:   l821 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0" as *co ... st u8: typeof(_588) = *const {l823} u8
                        // 1115: b"0\n\0" as *co ... st u8:   l823 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0": typeof(_589) = *const {l825} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1115: b"0\n\0":   l825 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0": typeof(_590) = & {l827} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1115: b"0\n\0":   l827 = UNIQUE | NON_NULL, FIXED
                        // 1115: (*w).post: typeof(_591) = *mut {l829} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                        // 1115: (*w).post:   l829 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0" as *co ... _char: typeof(_587 = move _588 as *const i8 (Misc)) = *const {l1087} i8
                        // 1115: b"0\n\0" as *co ... _char:   l1087 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0": typeof(_589 = &raw const (*_590)) = *const {l1085} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1115: b"0\n\0":   l1085 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0" as *co ... st u8: typeof(_588 = move _589 as *const u8 (Pointer(ArrayToPointer))) = *const {l1086} u8
                        // 1115: b"0\n\0" as *co ... st u8:   l1086 = UNIQUE | NON_NULL, (empty)
                        // 1115: b"0\n\0": typeof(_590 = const b"0\n\x00") = & {l1084} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                        // 1115: b"0\n\0":   l1084 = UNIQUE | NON_NULL, (empty)
                    }
                    red = (*w).nfailed;
                    size = p.offset_from(a) as libc::c_long as libc::c_int;
                    // 1118: p: typeof(_595) = *mut {l834} i32
                    // 1118: p:   l834 = UNIQUE | NON_NULL, (empty)
                    // 1118: a: typeof(_596) = *const {l836} i32
                    // 1118: a:   l836 = UNIQUE | NON_NULL, (empty)
                    // 1118: a: typeof(_597) = *mut {l838} i32
                    // 1118: a:   l838 = UNIQUE | NON_NULL, (empty)
                    // 1118: a: typeof(_596 = move _597 as *const i32 (Pointer(MutToConstPointer))) = *const {l1088} i32
                    // 1118: a:   l1088 = UNIQUE | NON_NULL, (empty)
                    sumassumptions += size;
                    // 1119: sumassumptions: typeof(_599) = *mut {l841} i32
                    // 1119: sumassumptions:   l841 = UNIQUE | NON_NULL, (empty)
                    redassumptions += red;
                    // 1120: redassumptions: typeof(_602) = *mut {l845} i32
                    // 1120: redassumptions:   l845 = UNIQUE | NON_NULL, (empty)
                    if bar == 0 {
                    // 1121: bar: typeof(_607) = *mut {l851} i32
                    // 1121: bar:   l851 = UNIQUE | NON_NULL, (empty)
                        msg(
                            w,
                            // 1123: w: typeof(_609) = *mut {l854} DefId(0:297 ~ ilingeling[c969]::Worker)
                            // 1123: w:   l854 = UNIQUE | NON_NULL, (empty)
                            1 as libc::c_int,
                            b"job %d UNSATISFIABLE (%d failed / %d) in %.3f seconds\0" as *const u8
                            // 1125: b"job %d UNSATI ... _char: typeof(_611) = *const {l857} i8
                            // 1125: b"job %d UNSATI ... _char:   l857 = UNIQUE | NON_NULL, (empty)
                            // 1125: b"job %d UNSATI ... st u8: typeof(_612) = *const {l859} u8
                            // 1125: b"job %d UNSATI ... st u8:   l859 = UNIQUE | NON_NULL, (empty)
                            // 1125: b"job %d UNSATI ... ds\0": typeof(_613) = *const {l861} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1125: b"job %d UNSATI ... ds\0":   l861 = UNIQUE | NON_NULL, (empty)
                            // 1125: b"job %d UNSATI ... ds\0": typeof(_614) = & {l863} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1125: b"job %d UNSATI ... ds\0":   l863 = UNIQUE | NON_NULL, FIXED
                            // 1125: b"job %d UNSATI ... ds\0": typeof(_614 = const b"job %d UNSATISFIABLE (%d failed / %d) in %.3f seconds\x00") = & {l1089} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1125: b"job %d UNSATI ... ds\0":   l1089 = UNIQUE | NON_NULL, (empty)
                            // 1125: b"job %d UNSATI ... st u8: typeof(_612 = move _613 as *const u8 (Pointer(ArrayToPointer))) = *const {l1091} u8
                            // 1125: b"job %d UNSATI ... st u8:   l1091 = UNIQUE | NON_NULL, (empty)
                            // 1125: b"job %d UNSATI ... ds\0": typeof(_613 = &raw const (*_614)) = *const {l1090} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000036)) }]
                            // 1125: b"job %d UNSATI ... ds\0":   l1090 = UNIQUE | NON_NULL, (empty)
                            // 1125: b"job %d UNSATI ... _char: typeof(_611 = move _612 as *const i8 (Misc)) = *const {l1092} i8
                            // 1125: b"job %d UNSATI ... _char:   l1092 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            last,
                            red,
                            size,
                            delta,
                        );
                    }
                    if red == 0 && deterministic == 0 {
                    // 1133: deterministic: typeof(_624) = *mut {l874} i32
                    // 1133: deterministic:   l874 = UNIQUE | NON_NULL, (empty)
                        if bar == 0 {
                        // 1134: bar: typeof(_628) = *mut {l879} i32
                        // 1134: bar:   l879 = UNIQUE | NON_NULL, (empty)
                            msg(
                                w,
                                // 1136: w: typeof(_630) = *mut {l882} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1136: w:   l882 = UNIQUE | NON_NULL, (empty)
                                1 as libc::c_int,
                                b"job %d ACTUALLY FOUND EMPTY CLAUSE\0" as *const u8
                                // 1138: b"job %d ACTUAL ... _char: typeof(_632) = *const {l885} i8
                                // 1138: b"job %d ACTUAL ... _char:   l885 = UNIQUE | NON_NULL, (empty)
                                // 1138: b"job %d ACTUAL ... st u8: typeof(_633) = *const {l887} u8
                                // 1138: b"job %d ACTUAL ... st u8:   l887 = UNIQUE | NON_NULL, (empty)
                                // 1138: b"job %d ACTUAL ... SE\0": typeof(_634) = *const {l889} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                // 1138: b"job %d ACTUAL ... SE\0":   l889 = UNIQUE | NON_NULL, (empty)
                                // 1138: b"job %d ACTUAL ... SE\0": typeof(_635) = & {l891} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                // 1138: b"job %d ACTUAL ... SE\0":   l891 = UNIQUE | NON_NULL, FIXED
                                // 1138: b"job %d ACTUAL ... st u8: typeof(_633 = move _634 as *const u8 (Pointer(ArrayToPointer))) = *const {l1095} u8
                                // 1138: b"job %d ACTUAL ... st u8:   l1095 = UNIQUE | NON_NULL, (empty)
                                // 1138: b"job %d ACTUAL ... SE\0": typeof(_634 = &raw const (*_635)) = *const {l1094} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                // 1138: b"job %d ACTUAL ... SE\0":   l1094 = UNIQUE | NON_NULL, (empty)
                                // 1138: b"job %d ACTUAL ... _char: typeof(_632 = move _633 as *const i8 (Misc)) = *const {l1096} i8
                                // 1138: b"job %d ACTUAL ... _char:   l1096 = UNIQUE | NON_NULL, (empty)
                                // 1138: b"job %d ACTUAL ... SE\0": typeof(_635 = const b"job %d ACTUALLY FOUND EMPTY CLAUSE\x00") = & {l1093} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                                // 1138: b"job %d ACTUAL ... SE\0":   l1093 = UNIQUE | NON_NULL, (empty)
                                    as *const libc::c_char,
                                last,
                            );
                        }
                        if pthread_mutex_lock(&mut donemutex) != 0 {
                        // 1143: &mut donemutex: typeof(_640) = *mut {l897} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1143: &mut donemutex:   l897 = UNIQUE | NON_NULL, (empty)
                        // 1143: &mut donemutex: typeof(_641) = &mut {l899} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1143: &mut donemutex:   l899 = UNIQUE | NON_NULL, (empty)
                        // 1143: donemutex: typeof(_642) = *mut {l901} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1143: donemutex:   l901 = UNIQUE | NON_NULL, (empty)
                        // 1143: &mut donemutex: typeof(_640 = &raw mut (*_641)) = *mut {l1098} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1143: &mut donemutex:   l1098 = UNIQUE | NON_NULL, (empty)
                        // 1143: &mut donemutex: typeof(_641 = &mut (*_642)) = &mut {l1097} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1143: &mut donemutex:   l1097 = UNIQUE | NON_NULL, (empty)
                            warn(
                                b"worker %d failed to lock 'done' mutex\0" as *const u8
                                // 1145: b"worker %d fai ... _char: typeof(_644) = *const {l904} i8
                                // 1145: b"worker %d fai ... _char:   l904 = UNIQUE | NON_NULL, (empty)
                                // 1145: b"worker %d fai ... st u8: typeof(_645) = *const {l906} u8
                                // 1145: b"worker %d fai ... st u8:   l906 = UNIQUE | NON_NULL, (empty)
                                // 1145: b"worker %d fai ... ex\0": typeof(_646) = *const {l908} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                // 1145: b"worker %d fai ... ex\0":   l908 = UNIQUE | NON_NULL, (empty)
                                // 1145: b"worker %d fai ... ex\0": typeof(_647) = & {l910} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                // 1145: b"worker %d fai ... ex\0":   l910 = UNIQUE | NON_NULL, FIXED
                                // 1145: b"worker %d fai ... st u8: typeof(_645 = move _646 as *const u8 (Pointer(ArrayToPointer))) = *const {l1101} u8
                                // 1145: b"worker %d fai ... st u8:   l1101 = UNIQUE | NON_NULL, (empty)
                                // 1145: b"worker %d fai ... ex\0": typeof(_647 = const b"worker %d failed to lock \'done\' mutex\x00") = & {l1099} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                // 1145: b"worker %d fai ... ex\0":   l1099 = UNIQUE | NON_NULL, (empty)
                                // 1145: b"worker %d fai ... _char: typeof(_644 = move _645 as *const i8 (Misc)) = *const {l1102} i8
                                // 1145: b"worker %d fai ... _char:   l1102 = UNIQUE | NON_NULL, (empty)
                                // 1145: b"worker %d fai ... ex\0": typeof(_646 = &raw const (*_647)) = *const {l1100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                                // 1145: b"worker %d fai ... ex\0":   l1100 = UNIQUE | NON_NULL, (empty)
                                    as *const libc::c_char,
                                w.offset_from(workers) as libc::c_long as libc::c_int,
                                // 1147: w: typeof(_651) = *mut {l915} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1147: w:   l915 = UNIQUE | NON_NULL, (empty)
                                // 1147: workers: typeof(_652) = *const {l917} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1147: workers:   l917 = UNIQUE | NON_NULL, (empty)
                                // 1147: workers: typeof(_653) = *mut {l919} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1147: workers:   l919 = UNIQUE | NON_NULL, (empty)
                                // 1147: workers: typeof(_654) = *mut {l921} *mut {l922} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1147: workers:   l921 = UNIQUE | NON_NULL, (empty)
                                // 1147: workers:   l922 = UNIQUE | NON_NULL, (empty)
                                // 1147: workers: typeof(_652 = move _653 as *const Worker (Pointer(MutToConstPointer))) = *const {l1103} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1147: workers:   l1103 = UNIQUE | NON_NULL, (empty)
                            );
                        }
                        done = 1 as libc::c_int;
                        // 1150: done: typeof(_656) = *mut {l925} i32
                        // 1150: done:   l925 = UNIQUE | NON_NULL, (empty)
                        if pthread_mutex_unlock(&mut donemutex) != 0 {
                        // 1151: &mut donemutex: typeof(_659) = *mut {l929} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1151: &mut donemutex:   l929 = UNIQUE | NON_NULL, (empty)
                        // 1151: &mut donemutex: typeof(_660) = &mut {l931} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1151: &mut donemutex:   l931 = UNIQUE | NON_NULL, (empty)
                        // 1151: donemutex: typeof(_661) = *mut {l933} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1151: donemutex:   l933 = UNIQUE | NON_NULL, (empty)
                        // 1151: &mut donemutex: typeof(_660 = &mut (*_661)) = &mut {l1104} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1151: &mut donemutex:   l1104 = UNIQUE | NON_NULL, (empty)
                        // 1151: &mut donemutex: typeof(_659 = &raw mut (*_660)) = *mut {l1105} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
                        // 1151: &mut donemutex:   l1105 = UNIQUE | NON_NULL, (empty)
                            warn(
                                b"worker %d failed to unlock 'done' mutex\0" as *const u8
                                // 1153: b"worker %d fai ... _char: typeof(_663) = *const {l936} i8
                                // 1153: b"worker %d fai ... _char:   l936 = UNIQUE | NON_NULL, (empty)
                                // 1153: b"worker %d fai ... st u8: typeof(_664) = *const {l938} u8
                                // 1153: b"worker %d fai ... st u8:   l938 = UNIQUE | NON_NULL, (empty)
                                // 1153: b"worker %d fai ... ex\0": typeof(_665) = *const {l940} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                                // 1153: b"worker %d fai ... ex\0":   l940 = UNIQUE | NON_NULL, (empty)
                                // 1153: b"worker %d fai ... ex\0": typeof(_666) = & {l942} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                                // 1153: b"worker %d fai ... ex\0":   l942 = UNIQUE | NON_NULL, FIXED
                                // 1153: b"worker %d fai ... _char: typeof(_663 = move _664 as *const i8 (Misc)) = *const {l1109} i8
                                // 1153: b"worker %d fai ... _char:   l1109 = UNIQUE | NON_NULL, (empty)
                                // 1153: b"worker %d fai ... ex\0": typeof(_666 = const b"worker %d failed to unlock \'done\' mutex\x00") = & {l1106} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                                // 1153: b"worker %d fai ... ex\0":   l1106 = UNIQUE | NON_NULL, (empty)
                                // 1153: b"worker %d fai ... st u8: typeof(_664 = move _665 as *const u8 (Pointer(ArrayToPointer))) = *const {l1108} u8
                                // 1153: b"worker %d fai ... st u8:   l1108 = UNIQUE | NON_NULL, (empty)
                                // 1153: b"worker %d fai ... ex\0": typeof(_665 = &raw const (*_666)) = *const {l1107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000028)) }]
                                // 1153: b"worker %d fai ... ex\0":   l1107 = UNIQUE | NON_NULL, (empty)
                                    as *const libc::c_char,
                                w.offset_from(workers) as libc::c_long as libc::c_int,
                                // 1155: w: typeof(_670) = *mut {l947} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1155: w:   l947 = UNIQUE | NON_NULL, (empty)
                                // 1155: workers: typeof(_671) = *const {l949} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1155: workers:   l949 = UNIQUE | NON_NULL, (empty)
                                // 1155: workers: typeof(_672) = *mut {l951} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1155: workers:   l951 = UNIQUE | NON_NULL, (empty)
                                // 1155: workers: typeof(_673) = *mut {l953} *mut {l954} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1155: workers:   l953 = UNIQUE | NON_NULL, (empty)
                                // 1155: workers:   l954 = UNIQUE | NON_NULL, (empty)
                                // 1155: workers: typeof(_671 = move _672 as *const Worker (Pointer(MutToConstPointer))) = *const {l1110} DefId(0:297 ~ ilingeling[c969]::Worker)
                                // 1155: workers:   l1110 = UNIQUE | NON_NULL, (empty)
                            );
                        }
                    } else {
                        if reduce != 0 {
                        // 1159: reduce: typeof(_678) = *mut {l960} i32
                        // 1159: reduce:   l960 = UNIQUE | NON_NULL, (empty)
                            lglreducecache((*w).lgl);
                            // 1160: (*w).lgl: typeof(_680) = *mut {l963} LGL
                            // 1160: (*w).lgl:   l963 = UNIQUE | NON_NULL, (empty)
                        }
                        continue;
                    }
                } else if bar == 0 {
                // 1164: bar: typeof(_683) = *mut {l967} i32
                // 1164: bar:   l967 = UNIQUE | NON_NULL, (empty)
                    msg(
                        w,
                        // 1166: w: typeof(_685) = *mut {l970} DefId(0:297 ~ ilingeling[c969]::Worker)
                        // 1166: w:   l970 = UNIQUE | NON_NULL, (empty)
                        1 as libc::c_int,
                        b"job %d UNKNOWN\0" as *const u8 as *const libc::c_char,
                        // 1168: b"job %d UNKNOW ... _char: typeof(_687) = *const {l973} i8
                        // 1168: b"job %d UNKNOW ... _char:   l973 = UNIQUE | NON_NULL, (empty)
                        // 1168: b"job %d UNKNOW ... st u8: typeof(_688) = *const {l975} u8
                        // 1168: b"job %d UNKNOW ... st u8:   l975 = UNIQUE | NON_NULL, (empty)
                        // 1168: b"job %d UNKNOWN\0": typeof(_689) = *const {l977} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1168: b"job %d UNKNOWN\0":   l977 = UNIQUE | NON_NULL, (empty)
                        // 1168: b"job %d UNKNOWN\0": typeof(_690) = & {l979} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1168: b"job %d UNKNOWN\0":   l979 = UNIQUE | NON_NULL, FIXED
                        // 1168: b"job %d UNKNOW ... _char: typeof(_687 = move _688 as *const i8 (Misc)) = *const {l1114} i8
                        // 1168: b"job %d UNKNOW ... _char:   l1114 = UNIQUE | NON_NULL, (empty)
                        // 1168: b"job %d UNKNOW ... st u8: typeof(_688 = move _689 as *const u8 (Pointer(ArrayToPointer))) = *const {l1113} u8
                        // 1168: b"job %d UNKNOW ... st u8:   l1113 = UNIQUE | NON_NULL, (empty)
                        // 1168: b"job %d UNKNOWN\0": typeof(_689 = &raw const (*_690)) = *const {l1112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1168: b"job %d UNKNOWN\0":   l1112 = UNIQUE | NON_NULL, (empty)
                        // 1168: b"job %d UNKNOWN\0": typeof(_690 = const b"job %d UNKNOWN\x00") = & {l1111} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                        // 1168: b"job %d UNKNOWN\0":   l1111 = UNIQUE | NON_NULL, (empty)
                        last,
                    );
                }
            }
            if bar == 0 {
            // 1173: bar: typeof(_695) = *mut {l985} i32
            // 1173: bar:   l985 = UNIQUE | NON_NULL, (empty)
                msg(
                    w,
                    // 1175: w: typeof(_697) = *mut {l988} DefId(0:297 ~ ilingeling[c969]::Worker)
                    // 1175: w:   l988 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int,
                    b"done\0" as *const u8 as *const libc::c_char,
                    // 1177: b"done\0" as *c ... _char: typeof(_699) = *const {l991} i8
                    // 1177: b"done\0" as *c ... _char:   l991 = UNIQUE | NON_NULL, (empty)
                    // 1177: b"done\0" as *c ... st u8: typeof(_700) = *const {l993} u8
                    // 1177: b"done\0" as *c ... st u8:   l993 = UNIQUE | NON_NULL, (empty)
                    // 1177: b"done\0": typeof(_701) = *const {l995} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1177: b"done\0":   l995 = UNIQUE | NON_NULL, (empty)
                    // 1177: b"done\0": typeof(_702) = & {l997} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1177: b"done\0":   l997 = UNIQUE | NON_NULL, FIXED
                    // 1177: b"done\0" as *c ... st u8: typeof(_700 = move _701 as *const u8 (Pointer(ArrayToPointer))) = *const {l1117} u8
                    // 1177: b"done\0" as *c ... st u8:   l1117 = UNIQUE | NON_NULL, (empty)
                    // 1177: b"done\0": typeof(_702 = const b"done\x00") = & {l1115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1177: b"done\0":   l1115 = UNIQUE | NON_NULL, (empty)
                    // 1177: b"done\0" as *c ... _char: typeof(_699 = move _700 as *const i8 (Misc)) = *const {l1118} i8
                    // 1177: b"done\0" as *c ... _char:   l1118 = UNIQUE | NON_NULL, (empty)
                    // 1177: b"done\0": typeof(_701 = &raw const (*_702)) = *const {l1116} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                    // 1177: b"done\0":   l1116 = UNIQUE | NON_NULL, (empty)
                );
            }
            return 0 as *mut libc::c_void;
            // 1180: 0 as *mut libc: ... _void: typeof(_0 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l1119} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1180: 0 as *mut libc: ... _void:   l1119 = UNIQUE | NON_NULL, (empty)
        }
    }
}
unsafe extern "C" fn init() {
    let mut w: *mut Worker = 0 as *mut Worker;
    // 1185: mut w: typeof(_1) = *mut {l1} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1185: mut w:   l1 = UNIQUE | NON_NULL, (empty)
    // 1185: 0 as *mut Worker: typeof(_1 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l128} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1185: 0 as *mut Worker:   l128 = UNIQUE | NON_NULL, (empty)
    let mut BYTES: size_t =
        (nworkers as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
        // 1187: nworkers: typeof(_5) = *mut {l6} i32
        // 1187: nworkers:   l6 = UNIQUE | NON_NULL, (empty)
    workers = malloc(BYTES) as *mut Worker;
    // 1188: malloc(BYTES): typeof(_8) = *mut {l10} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1188: malloc(BYTES):   l10 = UNIQUE | NON_NULL, (empty)
    // 1188: workers: typeof(_10) = *mut {l13} *mut {l14} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1188: workers:   l13 = UNIQUE | NON_NULL, (empty)
    // 1188: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1188: workers = mallo ... orker: typeof((*_10) = move _8 as *mut Worker (Misc)) = *mut {l129} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1188: workers = mallo ... orker:   l129 = UNIQUE | NON_NULL, (empty)
    if workers.is_null() {
    // 1189: workers: typeof(_13) = *mut {l18} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1189: workers:   l18 = UNIQUE | NON_NULL, (empty)
    // 1189: workers: typeof(_14) = *mut {l20} *mut {l21} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1189: workers:   l20 = UNIQUE | NON_NULL, (empty)
    // 1189: workers:   l21 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 1190: b"out of memory ... _char: typeof(_17) = *const {l25} i8
        // 1190: b"out of memory ... _char:   l25 = UNIQUE | NON_NULL, (empty)
        // 1190: b"out of memory ... st u8: typeof(_18) = *const {l27} u8
        // 1190: b"out of memory ... st u8:   l27 = UNIQUE | NON_NULL, (empty)
        // 1190: b"out of memory\0": typeof(_19) = *const {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1190: b"out of memory\0":   l29 = UNIQUE | NON_NULL, (empty)
        // 1190: b"out of memory\0": typeof(_20) = & {l31} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1190: b"out of memory\0":   l31 = UNIQUE | NON_NULL, FIXED
        // 1190: b"out of memory\0": typeof(_19 = &raw const (*_20)) = *const {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1190: b"out of memory\0":   l131 = UNIQUE | NON_NULL, (empty)
        // 1190: b"out of memory ... st u8: typeof(_18 = move _19 as *const u8 (Pointer(ArrayToPointer))) = *const {l132} u8
        // 1190: b"out of memory ... st u8:   l132 = UNIQUE | NON_NULL, (empty)
        // 1190: b"out of memory\0": typeof(_20 = const b"out of memory\x00") = & {l130} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1190: b"out of memory\0":   l130 = UNIQUE | NON_NULL, (empty)
        // 1190: b"out of memory ... _char: typeof(_17 = move _18 as *const i8 (Misc)) = *const {l133} i8
        // 1190: b"out of memory ... _char:   l133 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(workers as *mut libc::c_void, 0 as libc::c_int, BYTES);
    // 1193: memset(workers  ... YTES): typeof(_23) = *mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1193: memset(workers  ... YTES):   l35 = UNIQUE | NON_NULL, (empty)
    // 1193: workers as *mut ... _void: typeof(_24) = *mut {l37} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1193: workers as *mut ... _void:   l37 = UNIQUE | NON_NULL, (empty)
    // 1193: workers: typeof(_25) = *mut {l39} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1193: workers:   l39 = UNIQUE | NON_NULL, (empty)
    // 1193: workers: typeof(_26) = *mut {l41} *mut {l42} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1193: workers:   l41 = UNIQUE | NON_NULL, (empty)
    // 1193: workers:   l42 = UNIQUE | NON_NULL, (empty)
    // 1193: workers as *mut ... _void: typeof(_24 = move _25 as *mut libc::c_void (Misc)) = *mut {l134} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1193: workers as *mut ... _void:   l134 = UNIQUE | NON_NULL, (empty)
    allocated = allocated.wrapping_add(BYTES);
    // 1194: allocated: typeof(_31) = *mut {l48} u64
    // 1194: allocated:   l48 = UNIQUE | NON_NULL, (empty)
    // 1194: allocated: typeof(_33) = *mut {l51} u64
    // 1194: allocated:   l51 = UNIQUE | NON_NULL, (empty)
    if allocated > maxallocated {
    // 1195: allocated: typeof(_37) = *mut {l56} u64
    // 1195: allocated:   l56 = UNIQUE | NON_NULL, (empty)
    // 1195: maxallocated: typeof(_39) = *mut {l59} u64
    // 1195: maxallocated:   l59 = UNIQUE | NON_NULL, (empty)
        maxallocated = allocated;
        // 1196: allocated: typeof(_41) = *mut {l62} u64
        // 1196: allocated:   l62 = UNIQUE | NON_NULL, (empty)
        // 1196: maxallocated: typeof(_42) = *mut {l64} u64
        // 1196: maxallocated:   l64 = UNIQUE | NON_NULL, (empty)
    }
    w = workers;
    // 1198: workers: typeof(_43) = *mut {l66} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1198: workers:   l66 = UNIQUE | NON_NULL, (empty)
    // 1198: workers: typeof(_44) = *mut {l68} *mut {l69} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1198: workers:   l68 = UNIQUE | NON_NULL, (empty)
    // 1198: workers:   l69 = UNIQUE | NON_NULL, (empty)
    while w < workers.offset(nworkers as isize) {
    // 1199: w: typeof(_48) = *mut {l74} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1199: w:   l74 = UNIQUE | NON_NULL, (empty)
    // 1199: workers.offset( ... size): typeof(_49) = *mut {l76} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1199: workers.offset( ... size):   l76 = UNIQUE | NON_NULL, (empty)
    // 1199: workers: typeof(_50) = *mut {l78} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1199: workers:   l78 = UNIQUE | NON_NULL, (empty)
    // 1199: workers: typeof(_51) = *mut {l80} *mut {l81} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1199: workers:   l80 = UNIQUE | NON_NULL, (empty)
    // 1199: workers:   l81 = UNIQUE | NON_NULL, (empty)
    // 1199: nworkers: typeof(_54) = *mut {l85} i32
    // 1199: nworkers:   l85 = UNIQUE | NON_NULL, (empty)
        (*w).last = -(1 as libc::c_int);
        (*w).lgl = lglinit();
        // 1201: lglinit(): typeof(_57) = *mut {l89} LGL
        // 1201: lglinit():   l89 = UNIQUE | NON_NULL, (empty)
        pthread_mutex_init(&mut (*w).cloned.lock, 0 as *const pthread_mutexattr_t);
        // 1202: &mut (*w).clone ... .lock: typeof(_59) = *mut {l92} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1202: &mut (*w).clone ... .lock:   l92 = UNIQUE | NON_NULL, (empty)
        // 1202: &mut (*w).clone ... .lock: typeof(_60) = &mut {l94} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1202: &mut (*w).clone ... .lock:   l94 = UNIQUE | NON_NULL, (empty)
        // 1202: 0 as *const pth ... ttr_t: typeof(_61) = *const {l96} DefId(0:275 ~ ilingeling[c969]::pthread_mutexattr_t)
        // 1202: 0 as *const pth ... ttr_t:   l96 = UNIQUE | NON_NULL, (empty)
        // 1202: &mut (*w).clone ... .lock: typeof(_60 = &mut (((*_1).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l135} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1202: &mut (*w).clone ... .lock:   l135 = UNIQUE | NON_NULL, (empty)
        // 1202: 0 as *const pth ... ttr_t: typeof(_61 = const 0_usize as *const pthread_mutexattr_t (PointerFromExposedAddress)) = *const {l137} DefId(0:275 ~ ilingeling[c969]::pthread_mutexattr_t)
        // 1202: 0 as *const pth ... ttr_t:   l137 = UNIQUE | NON_NULL, (empty)
        // 1202: &mut (*w).clone ... .lock: typeof(_59 = &raw mut (*_60)) = *mut {l136} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1202: &mut (*w).clone ... .lock:   l136 = UNIQUE | NON_NULL, (empty)
        initlgl((*w).lgl, w, 1 as libc::c_int);
        // 1203: (*w).lgl: typeof(_63) = *mut {l99} LGL
        // 1203: (*w).lgl:   l99 = UNIQUE | NON_NULL, (empty)
        // 1203: w: typeof(_64) = *mut {l101} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1203: w:   l101 = UNIQUE | NON_NULL, (empty)
        w = w.offset(1);
        // 1204: w.offset(1): typeof(_66) = *mut {l104} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1204: w.offset(1):   l104 = UNIQUE | NON_NULL, (empty)
        // 1204: w: typeof(_67) = *mut {l106} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1204: w:   l106 = UNIQUE | NON_NULL, (empty)
        w;
        // 1205: w: typeof(_68) = *mut {l108} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1205: w:   l108 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        0 as *mut Worker,
        // 1208: 0 as *mut Worker: typeof(_73) = *mut {l114} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1208: 0 as *mut Worker:   l114 = UNIQUE | NON_NULL, (empty)
        // 1208: 0 as *mut Worker: typeof(_73 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l138} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1208: 0 as *mut Worker:   l138 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"allocated %d workers\0" as *const u8 as *const libc::c_char,
        // 1210: b"allocated %d  ... _char: typeof(_75) = *const {l117} i8
        // 1210: b"allocated %d  ... _char:   l117 = UNIQUE | NON_NULL, (empty)
        // 1210: b"allocated %d  ... st u8: typeof(_76) = *const {l119} u8
        // 1210: b"allocated %d  ... st u8:   l119 = UNIQUE | NON_NULL, (empty)
        // 1210: b"allocated %d  ... rs\0": typeof(_77) = *const {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1210: b"allocated %d  ... rs\0":   l121 = UNIQUE | NON_NULL, (empty)
        // 1210: b"allocated %d  ... rs\0": typeof(_78) = & {l123} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1210: b"allocated %d  ... rs\0":   l123 = UNIQUE | NON_NULL, FIXED
        // 1210: b"allocated %d  ... _char: typeof(_75 = move _76 as *const i8 (Misc)) = *const {l142} i8
        // 1210: b"allocated %d  ... _char:   l142 = UNIQUE | NON_NULL, (empty)
        // 1210: b"allocated %d  ... rs\0": typeof(_77 = &raw const (*_78)) = *const {l140} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1210: b"allocated %d  ... rs\0":   l140 = UNIQUE | NON_NULL, (empty)
        // 1210: b"allocated %d  ... rs\0": typeof(_78 = const b"allocated %d workers\x00") = & {l139} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
        // 1210: b"allocated %d  ... rs\0":   l139 = UNIQUE | NON_NULL, (empty)
        // 1210: b"allocated %d  ... st u8: typeof(_76 = move _77 as *const u8 (Pointer(ArrayToPointer))) = *const {l141} u8
        // 1210: b"allocated %d  ... st u8:   l141 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 1211: nworkers: typeof(_80) = *mut {l126} i32
        // 1211: nworkers:   l126 = UNIQUE | NON_NULL, (empty)
    );
}
unsafe extern "C" fn reset() {
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    // 1216: mut p: typeof(_2) = *mut {l2} i32
    // 1216: mut p:   l2 = UNIQUE | NON_NULL, (empty)
    // 1216: 0 as *mut libc: ... c_int: typeof(_2 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l309} i32
    // 1216: 0 as *mut libc: ... c_int:   l309 = UNIQUE | NON_NULL, (empty)
    let mut a: *mut libc::c_int = 0 as *mut libc::c_int;
    // 1217: mut a: typeof(_3) = *mut {l4} i32
    // 1217: mut a:   l4 = UNIQUE | NON_NULL, (empty)
    // 1217: 0 as *mut libc: ... c_int: typeof(_3 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l310} i32
    // 1217: 0 as *mut libc: ... c_int:   l310 = UNIQUE | NON_NULL, (empty)
    if !vals.is_null() {
    // 1218: vals: typeof(_7) = *mut {l9} i8
    // 1218: vals:   l9 = UNIQUE | NON_NULL, (empty)
    // 1218: vals: typeof(_8) = *mut {l11} *mut {l12} i8
    // 1218: vals:   l11 = UNIQUE | NON_NULL, (empty)
    // 1218: vals:   l12 = UNIQUE | NON_NULL, (empty)
        let mut BYTES: size_t = (nvars as libc::c_ulong)
        // 1219: nvars: typeof(_12) = *mut {l17} i32
        // 1219: nvars:   l17 = UNIQUE | NON_NULL, (empty)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong);
        allocated = allocated.wrapping_sub(BYTES);
        // 1221: allocated: typeof(_17) = *mut {l23} u64
        // 1221: allocated:   l23 = UNIQUE | NON_NULL, (empty)
        // 1221: allocated: typeof(_19) = *mut {l26} u64
        // 1221: allocated:   l26 = UNIQUE | NON_NULL, (empty)
        free(vals as *mut libc::c_void);
        // 1222: vals as *mut li ... _void: typeof(_21) = *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1222: vals as *mut li ... _void:   l29 = UNIQUE | NON_NULL, (empty)
        // 1222: vals: typeof(_22) = *mut {l31} i8
        // 1222: vals:   l31 = UNIQUE | NON_NULL, (empty)
        // 1222: vals: typeof(_23) = *mut {l33} *mut {l34} i8
        // 1222: vals:   l33 = UNIQUE | NON_NULL, (empty)
        // 1222: vals:   l34 = UNIQUE | NON_NULL, (empty)
        // 1222: vals as *mut li ... _void: typeof(_21 = move _22 as *mut libc::c_void (Misc)) = *mut {l311} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1222: vals as *mut li ... _void:   l311 = UNIQUE | NON_NULL, (empty)
        vals = 0 as *mut libc::c_schar;
        // 1223: vals: typeof(_24) = *mut {l36} *mut {l37} i8
        // 1223: vals:   l36 = UNIQUE | NON_NULL, (empty)
        // 1223: vals:   l37 = UNIQUE | NON_NULL, (empty)
        // 1223: vals = 0 as *mu ... schar: typeof((*_24) = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l312} i8
        // 1223: vals = 0 as *mu ... schar:   l312 = UNIQUE | NON_NULL, (empty)
    }
    i = 0 as libc::c_int;
    while i < nworkers {
    // 1226: nworkers: typeof(_31) = *mut {l45} i32
    // 1226: nworkers:   l45 = UNIQUE | NON_NULL, (empty)
        let mut w: *mut Worker = workers.offset(i as isize);
        // 1227: mut w: typeof(_32) = *mut {l47} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1227: mut w:   l47 = UNIQUE | NON_NULL, (empty)
        // 1227: workers: typeof(_33) = *mut {l49} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1227: workers:   l49 = UNIQUE | NON_NULL, (empty)
        // 1227: workers: typeof(_34) = *mut {l51} *mut {l52} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1227: workers:   l51 = UNIQUE | NON_NULL, (empty)
        // 1227: workers:   l52 = UNIQUE | NON_NULL, (empty)
        lglrelease((*w).lgl);
        // 1228: (*w).lgl: typeof(_38) = *mut {l57} LGL
        // 1228: (*w).lgl:   l57 = UNIQUE | NON_NULL, (empty)
        if !((*w).proof).is_null() {
        // 1229: ((*w).proof): typeof(_42) = *mut {l62} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1229: ((*w).proof):   l62 = UNIQUE | NON_NULL, (empty)
            (*w).proof = 0 as *mut FILE;
            // 1230: (*w).proof = 0  ...  FILE: typeof(((*_32).5: *mut _IO_FILE) = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l313} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1230: (*w).proof = 0  ...  FILE:   l313 = UNIQUE | NON_NULL, (empty)
            fclose((*w).post);
            // 1231: (*w).post: typeof(_44) = *mut {l65} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1231: (*w).post:   l65 = UNIQUE | NON_NULL, (empty)
            (*w).post = 0 as *mut FILE;
            // 1232: (*w).post = 0 a ...  FILE: typeof(((*_32).6: *mut _IO_FILE) = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l314} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1232: (*w).post = 0 a ...  FILE:   l314 = UNIQUE | NON_NULL, (empty)
        }
        if !((*w).failed).is_null() {
        // 1234: ((*w).failed): typeof(_48) = *mut {l70} i32
        // 1234: ((*w).failed):   l70 = UNIQUE | NON_NULL, (empty)
            let mut BYTES_0: size_t = (maxassumptionsize as libc::c_ulong)
            // 1235: maxassumptionsize: typeof(_52) = *mut {l75} i32
            // 1235: maxassumptionsize:   l75 = UNIQUE | NON_NULL, (empty)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
            allocated = allocated.wrapping_sub(BYTES_0);
            // 1237: allocated: typeof(_57) = *mut {l81} u64
            // 1237: allocated:   l81 = UNIQUE | NON_NULL, (empty)
            // 1237: allocated: typeof(_59) = *mut {l84} u64
            // 1237: allocated:   l84 = UNIQUE | NON_NULL, (empty)
            free((*w).failed as *mut libc::c_void);
            // 1238: (*w).failed as  ... _void: typeof(_61) = *mut {l87} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1238: (*w).failed as  ... _void:   l87 = UNIQUE | NON_NULL, (empty)
            // 1238: (*w).failed: typeof(_62) = *mut {l89} i32
            // 1238: (*w).failed:   l89 = UNIQUE | NON_NULL, (empty)
            // 1238: (*w).failed as  ... _void: typeof(_61 = move _62 as *mut libc::c_void (Misc)) = *mut {l315} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1238: (*w).failed as  ... _void:   l315 = UNIQUE | NON_NULL, (empty)
            (*w).failed = 0 as *mut libc::c_int;
            // 1239: (*w).failed = 0 ... c_int: typeof(((*_32).7: *mut i32) = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l316} i32
            // 1239: (*w).failed = 0 ... c_int:   l316 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    let mut BYTES_1: size_t =
        (nworkers as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Worker>() as libc::c_ulong);
        // 1245: nworkers: typeof(_71) = *mut {l99} i32
        // 1245: nworkers:   l99 = UNIQUE | NON_NULL, (empty)
    allocated = allocated.wrapping_sub(BYTES_1);
    // 1246: allocated: typeof(_76) = *mut {l105} u64
    // 1246: allocated:   l105 = UNIQUE | NON_NULL, (empty)
    // 1246: allocated: typeof(_78) = *mut {l108} u64
    // 1246: allocated:   l108 = UNIQUE | NON_NULL, (empty)
    free(workers as *mut libc::c_void);
    // 1247: workers as *mut ... _void: typeof(_80) = *mut {l111} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1247: workers as *mut ... _void:   l111 = UNIQUE | NON_NULL, (empty)
    // 1247: workers: typeof(_81) = *mut {l113} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1247: workers:   l113 = UNIQUE | NON_NULL, (empty)
    // 1247: workers: typeof(_82) = *mut {l115} *mut {l116} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1247: workers:   l115 = UNIQUE | NON_NULL, (empty)
    // 1247: workers:   l116 = UNIQUE | NON_NULL, (empty)
    // 1247: workers as *mut ... _void: typeof(_80 = move _81 as *mut libc::c_void (Misc)) = *mut {l317} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1247: workers as *mut ... _void:   l317 = UNIQUE | NON_NULL, (empty)
    workers = 0 as *mut Worker;
    // 1248: workers: typeof(_83) = *mut {l118} *mut {l119} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1248: workers:   l118 = UNIQUE | NON_NULL, (empty)
    // 1248: workers:   l119 = UNIQUE | NON_NULL, (empty)
    // 1248: workers = 0 as  ... orker: typeof((*_83) = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l318} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1248: workers = 0 as  ... orker:   l318 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < nassumptions {
    // 1250: nassumptions: typeof(_89) = *mut {l126} i32
    // 1250: nassumptions:   l126 = UNIQUE | NON_NULL, (empty)
        a = *assumptions.offset(i as isize);
        // 1251: *assumptions.of ... size): typeof(_90) = *mut {l128} i32
        // 1251: *assumptions.of ... size):   l128 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions.off ... size): typeof(_91) = *mut {l130} *mut {l131} i32
        // 1251: assumptions.off ... size):   l130 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions.off ... size):   l131 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions: typeof(_92) = *mut {l133} *mut {l134} i32
        // 1251: assumptions:   l133 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions:   l134 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions: typeof(_93) = *mut {l136} *mut {l137} *mut {l138} i32
        // 1251: assumptions:   l136 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions:   l137 = UNIQUE | NON_NULL, (empty)
        // 1251: assumptions:   l138 = UNIQUE | NON_NULL, (empty)
        p = a;
        // 1252: a: typeof(_96) = *mut {l142} i32
        // 1252: a:   l142 = UNIQUE | NON_NULL, (empty)
        while *p != 0 {
            p = p.offset(1);
            // 1254: p.offset(1): typeof(_100) = *mut {l147} i32
            // 1254: p.offset(1):   l147 = UNIQUE | NON_NULL, (empty)
            // 1254: p: typeof(_101) = *mut {l149} i32
            // 1254: p:   l149 = UNIQUE | NON_NULL, (empty)
            p;
            // 1255: p: typeof(_102) = *mut {l151} i32
            // 1255: p:   l151 = UNIQUE | NON_NULL, (empty)
        }
        let mut BYTES_2: size_t = ((p.offset_from(a) as libc::c_long
        // 1257: p: typeof(_111) = *mut {l161} i32
        // 1257: p:   l161 = UNIQUE | NON_NULL, (empty)
        // 1257: a: typeof(_112) = *const {l163} i32
        // 1257: a:   l163 = UNIQUE | NON_NULL, (empty)
        // 1257: a: typeof(_113) = *mut {l165} i32
        // 1257: a:   l165 = UNIQUE | NON_NULL, (empty)
        // 1257: a: typeof(_112 = move _113 as *const i32 (Pointer(MutToConstPointer))) = *const {l319} i32
        // 1257: a:   l319 = UNIQUE | NON_NULL, (empty)
            + 1 as libc::c_int as libc::c_long)
            as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
        allocated = allocated.wrapping_sub(BYTES_2);
        // 1261: allocated: typeof(_121) = *mut {l174} u64
        // 1261: allocated:   l174 = UNIQUE | NON_NULL, (empty)
        // 1261: allocated: typeof(_123) = *mut {l177} u64
        // 1261: allocated:   l177 = UNIQUE | NON_NULL, (empty)
        free(a as *mut libc::c_void);
        // 1262: a as *mut libc: ... _void: typeof(_125) = *mut {l180} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1262: a as *mut libc: ... _void:   l180 = UNIQUE | NON_NULL, (empty)
        // 1262: a: typeof(_126) = *mut {l182} i32
        // 1262: a:   l182 = UNIQUE | NON_NULL, (empty)
        // 1262: a as *mut libc: ... _void: typeof(_125 = move _126 as *mut libc::c_void (Misc)) = *mut {l320} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1262: a as *mut libc: ... _void:   l320 = UNIQUE | NON_NULL, (empty)
        a = 0 as *mut libc::c_int;
        // 1263: a = 0 as *mut l ... c_int: typeof(_3 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l321} i32
        // 1263: a = 0 as *mut l ... c_int:   l321 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    let mut BYTES_3: size_t = (szassumptions as libc::c_ulong)
    // 1267: szassumptions: typeof(_135) = *mut {l192} i32
    // 1267: szassumptions:   l192 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_3);
    // 1269: allocated: typeof(_140) = *mut {l198} u64
    // 1269: allocated:   l198 = UNIQUE | NON_NULL, (empty)
    // 1269: allocated: typeof(_142) = *mut {l201} u64
    // 1269: allocated:   l201 = UNIQUE | NON_NULL, (empty)
    free(assumptions as *mut libc::c_void);
    // 1270: assumptions as  ... _void: typeof(_144) = *mut {l204} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1270: assumptions as  ... _void:   l204 = UNIQUE | NON_NULL, (empty)
    // 1270: assumptions: typeof(_145) = *mut {l206} *mut {l207} i32
    // 1270: assumptions:   l206 = UNIQUE | NON_NULL, (empty)
    // 1270: assumptions:   l207 = UNIQUE | NON_NULL, (empty)
    // 1270: assumptions: typeof(_146) = *mut {l209} *mut {l210} *mut {l211} i32
    // 1270: assumptions:   l209 = UNIQUE | NON_NULL, (empty)
    // 1270: assumptions:   l210 = UNIQUE | NON_NULL, (empty)
    // 1270: assumptions:   l211 = UNIQUE | NON_NULL, (empty)
    // 1270: assumptions as  ... _void: typeof(_144 = move _145 as *mut libc::c_void (Misc)) = *mut {l322} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1270: assumptions as  ... _void:   l322 = UNIQUE | NON_NULL, (empty)
    assumptions = 0 as *mut *mut libc::c_int;
    // 1271: assumptions: typeof(_147) = *mut {l213} *mut {l214} *mut {l215} i32
    // 1271: assumptions:   l213 = UNIQUE | NON_NULL, (empty)
    // 1271: assumptions:   l214 = UNIQUE | NON_NULL, (empty)
    // 1271: assumptions:   l215 = UNIQUE | NON_NULL, (empty)
    // 1271: assumptions = 0 ... c_int: typeof((*_147) = const 0_usize as *mut *mut i32 (PointerFromExposedAddress)) = *mut {l323} *mut {l324} i32
    // 1271: assumptions = 0 ... c_int:   l323 = UNIQUE | NON_NULL, (empty)
    // 1271: assumptions = 0 ... c_int:   l324 = UNIQUE | NON_NULL, (empty)
    let mut BYTES_4: size_t = (nassumptions as libc::c_ulong)
    // 1272: nassumptions: typeof(_151) = *mut {l220} i32
    // 1272: nassumptions:   l220 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_4);
    // 1274: allocated: typeof(_156) = *mut {l226} u64
    // 1274: allocated:   l226 = UNIQUE | NON_NULL, (empty)
    // 1274: allocated: typeof(_158) = *mut {l229} u64
    // 1274: allocated:   l229 = UNIQUE | NON_NULL, (empty)
    free(times as *mut libc::c_void);
    // 1275: times as *mut l ... _void: typeof(_160) = *mut {l232} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1275: times as *mut l ... _void:   l232 = UNIQUE | NON_NULL, (empty)
    // 1275: times: typeof(_161) = *mut {l234} f64
    // 1275: times:   l234 = UNIQUE | NON_NULL, (empty)
    // 1275: times: typeof(_162) = *mut {l236} *mut {l237} f64
    // 1275: times:   l236 = UNIQUE | NON_NULL, (empty)
    // 1275: times:   l237 = UNIQUE | NON_NULL, (empty)
    // 1275: times as *mut l ... _void: typeof(_160 = move _161 as *mut libc::c_void (Misc)) = *mut {l325} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1275: times as *mut l ... _void:   l325 = UNIQUE | NON_NULL, (empty)
    times = 0 as *mut libc::c_double;
    // 1276: times: typeof(_163) = *mut {l239} *mut {l240} f64
    // 1276: times:   l239 = UNIQUE | NON_NULL, (empty)
    // 1276: times:   l240 = UNIQUE | NON_NULL, (empty)
    // 1276: times = 0 as *m ... ouble: typeof((*_163) = const 0_usize as *mut f64 (PointerFromExposedAddress)) = *mut {l326} f64
    // 1276: times = 0 as *m ... ouble:   l326 = UNIQUE | NON_NULL, (empty)
    let mut BYTES_5: size_t = (szlits as libc::c_ulong)
    // 1277: szlits: typeof(_167) = *mut {l245} i32
    // 1277: szlits:   l245 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_5);
    // 1279: allocated: typeof(_172) = *mut {l251} u64
    // 1279: allocated:   l251 = UNIQUE | NON_NULL, (empty)
    // 1279: allocated: typeof(_174) = *mut {l254} u64
    // 1279: allocated:   l254 = UNIQUE | NON_NULL, (empty)
    free(lits as *mut libc::c_void);
    // 1280: lits as *mut li ... _void: typeof(_176) = *mut {l257} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1280: lits as *mut li ... _void:   l257 = UNIQUE | NON_NULL, (empty)
    // 1280: lits: typeof(_177) = *mut {l259} i32
    // 1280: lits:   l259 = UNIQUE | NON_NULL, (empty)
    // 1280: lits: typeof(_178) = *mut {l261} *mut {l262} i32
    // 1280: lits:   l261 = UNIQUE | NON_NULL, (empty)
    // 1280: lits:   l262 = UNIQUE | NON_NULL, (empty)
    // 1280: lits as *mut li ... _void: typeof(_176 = move _177 as *mut libc::c_void (Misc)) = *mut {l327} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1280: lits as *mut li ... _void:   l327 = UNIQUE | NON_NULL, (empty)
    lits = 0 as *mut libc::c_int;
    // 1281: lits: typeof(_179) = *mut {l264} *mut {l265} i32
    // 1281: lits:   l264 = UNIQUE | NON_NULL, (empty)
    // 1281: lits:   l265 = UNIQUE | NON_NULL, (empty)
    // 1281: lits = 0 as *mu ... c_int: typeof((*_179) = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l328} i32
    // 1281: lits = 0 as *mu ... c_int:   l328 = UNIQUE | NON_NULL, (empty)
    let mut BYTES_6: size_t = (szvars as libc::c_ulong)
    // 1282: szvars: typeof(_183) = *mut {l270} i32
    // 1282: szvars:   l270 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
    allocated = allocated.wrapping_sub(BYTES_6);
    // 1284: allocated: typeof(_188) = *mut {l276} u64
    // 1284: allocated:   l276 = UNIQUE | NON_NULL, (empty)
    // 1284: allocated: typeof(_190) = *mut {l279} u64
    // 1284: allocated:   l279 = UNIQUE | NON_NULL, (empty)
    free(used as *mut libc::c_void);
    // 1285: used as *mut li ... _void: typeof(_192) = *mut {l282} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1285: used as *mut li ... _void:   l282 = UNIQUE | NON_NULL, (empty)
    // 1285: used: typeof(_193) = *mut {l284} i32
    // 1285: used:   l284 = UNIQUE | NON_NULL, (empty)
    // 1285: used: typeof(_194) = *mut {l286} *mut {l287} i32
    // 1285: used:   l286 = UNIQUE | NON_NULL, (empty)
    // 1285: used:   l287 = UNIQUE | NON_NULL, (empty)
    // 1285: used as *mut li ... _void: typeof(_192 = move _193 as *mut libc::c_void (Misc)) = *mut {l329} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1285: used as *mut li ... _void:   l329 = UNIQUE | NON_NULL, (empty)
    used = 0 as *mut libc::c_int;
    // 1286: used: typeof(_195) = *mut {l289} *mut {l290} i32
    // 1286: used:   l289 = UNIQUE | NON_NULL, (empty)
    // 1286: used:   l290 = UNIQUE | NON_NULL, (empty)
    // 1286: used = 0 as *mu ... c_int: typeof((*_195) = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l330} i32
    // 1286: used = 0 as *mu ... c_int:   l330 = UNIQUE | NON_NULL, (empty)
    if allocated != 0 {
    // 1287: allocated: typeof(_198) = *mut {l294} u64
    // 1287: allocated:   l294 = UNIQUE | NON_NULL, (empty)
        warn(
            b"internal memory leak of %lld bytes\0" as *const u8 as *const libc::c_char,
            // 1289: b"internal memo ... _char: typeof(_200) = *const {l297} i8
            // 1289: b"internal memo ... _char:   l297 = UNIQUE | NON_NULL, (empty)
            // 1289: b"internal memo ... st u8: typeof(_201) = *const {l299} u8
            // 1289: b"internal memo ... st u8:   l299 = UNIQUE | NON_NULL, (empty)
            // 1289: b"internal memo ... es\0": typeof(_202) = *const {l301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 1289: b"internal memo ... es\0":   l301 = UNIQUE | NON_NULL, (empty)
            // 1289: b"internal memo ... es\0": typeof(_203) = & {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 1289: b"internal memo ... es\0":   l303 = UNIQUE | NON_NULL, FIXED
            // 1289: b"internal memo ... st u8: typeof(_201 = move _202 as *const u8 (Pointer(ArrayToPointer))) = *const {l333} u8
            // 1289: b"internal memo ... st u8:   l333 = UNIQUE | NON_NULL, (empty)
            // 1289: b"internal memo ... es\0": typeof(_202 = &raw const (*_203)) = *const {l332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 1289: b"internal memo ... es\0":   l332 = UNIQUE | NON_NULL, (empty)
            // 1289: b"internal memo ... es\0": typeof(_203 = const b"internal memory leak of %lld bytes\x00") = & {l331} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 1289: b"internal memo ... es\0":   l331 = UNIQUE | NON_NULL, (empty)
            // 1289: b"internal memo ... _char: typeof(_200 = move _201 as *const i8 (Misc)) = *const {l334} i8
            // 1289: b"internal memo ... _char:   l334 = UNIQUE | NON_NULL, (empty)
            allocated as libc::c_longlong,
            // 1290: allocated: typeof(_206) = *mut {l307} u64
            // 1290: allocated:   l307 = UNIQUE | NON_NULL, (empty)
        );
    }
}
unsafe extern "C" fn perr(mut fmt: *const libc::c_char, mut args: ...) {
// 1294: mut fmt: typeof(_1) = *const {g16} i8
// 1294: mut fmt:   g16 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fprintf(
        stderr,
        // 1297: stderr: typeof(_6) = *mut {l6} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1297: stderr:   l6 = UNIQUE | NON_NULL, (empty)
        // 1297: stderr: typeof(_7) = *mut {l8} *mut {l9} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1297: stderr:   l8 = UNIQUE | NON_NULL, (empty)
        // 1297: stderr:   l9 = UNIQUE | NON_NULL, (empty)
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        // 1298: b"%s:%d: \0" as ... _char: typeof(_8) = *const {l11} i8
        // 1298: b"%s:%d: \0" as ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 1298: b"%s:%d: \0" as ... st u8: typeof(_9) = *const {l13} u8
        // 1298: b"%s:%d: \0" as ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 1298: b"%s:%d: \0": typeof(_10) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1298: b"%s:%d: \0":   l15 = UNIQUE | NON_NULL, (empty)
        // 1298: b"%s:%d: \0": typeof(_11) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1298: b"%s:%d: \0":   l17 = UNIQUE | NON_NULL, FIXED
        // 1298: b"%s:%d: \0" as ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l59} i8
        // 1298: b"%s:%d: \0" as ... _char:   l59 = UNIQUE | NON_NULL, (empty)
        // 1298: b"%s:%d: \0" as ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l58} u8
        // 1298: b"%s:%d: \0" as ... st u8:   l58 = UNIQUE | NON_NULL, (empty)
        // 1298: b"%s:%d: \0": typeof(_11 = const b"%s:%d: \x00") = & {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1298: b"%s:%d: \0":   l56 = UNIQUE | NON_NULL, (empty)
        // 1298: b"%s:%d: \0": typeof(_10 = &raw const (*_11)) = *const {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 1298: b"%s:%d: \0":   l57 = UNIQUE | NON_NULL, (empty)
        inputname,
        // 1299: inputname: typeof(_12) = *mut {l19} i8
        // 1299: inputname:   l19 = UNIQUE | NON_NULL, (empty)
        // 1299: inputname: typeof(_13) = *mut {l21} *mut {l22} i8
        // 1299: inputname:   l21 = UNIQUE | NON_NULL, (empty)
        // 1299: inputname:   l22 = UNIQUE | NON_NULL, (empty)
        lineno,
        // 1300: lineno: typeof(_15) = *mut {l25} i32
        // 1300: lineno:   l25 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 1302: args.clone(): typeof(_17) = & {l28} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1302: args.clone():   l28 = UNIQUE | NON_NULL, (empty)
    // 1302: args.clone(): typeof(_17 = &_2) = & {l60} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1302: args.clone():   l60 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, fmt, ap.as_va_list());
    // 1303: stderr: typeof(_19) = *mut {l31} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1303: stderr:   l31 = UNIQUE | NON_NULL, (empty)
    // 1303: stderr: typeof(_20) = *mut {l33} *mut {l34} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1303: stderr:   l33 = UNIQUE | NON_NULL, (empty)
    // 1303: stderr:   l34 = UNIQUE | NON_NULL, (empty)
    // 1303: fmt: typeof(_21) = *const {l36} i8
    // 1303: fmt:   l36 = UNIQUE | NON_NULL, (empty)
    // 1303: ap.as_va_list(): typeof(_23) = &mut {l39} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1303: ap.as_va_list():   l39 = UNIQUE | NON_NULL, (empty)
    // 1303: ap.as_va_list(): typeof(_23 = &mut _4) = &mut {l61} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1303: ap.as_va_list():   l61 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 1304: stderr: typeof(_26) = *mut {l43} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1304: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 1304: stderr: typeof(_27) = *mut {l45} *mut {l46} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1304: stderr:   l45 = UNIQUE | NON_NULL, (empty)
    // 1304: stderr:   l46 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 1305: stderr: typeof(_29) = *mut {l49} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1305: stderr:   l49 = UNIQUE | NON_NULL, (empty)
    // 1305: stderr: typeof(_30) = *mut {l51} *mut {l52} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1305: stderr:   l51 = UNIQUE | NON_NULL, (empty)
    // 1305: stderr:   l52 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn next() -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = getc(inputfile);
    // 1310: inputfile: typeof(_4) = *mut {l4} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1310: inputfile:   l4 = UNIQUE | NON_NULL, (empty)
    // 1310: inputfile: typeof(_5) = *mut {l6} *mut {l7} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1310: inputfile:   l6 = UNIQUE | NON_NULL, (empty)
    // 1310: inputfile:   l7 = UNIQUE | NON_NULL, (empty)
    if res == '\n' as i32 {
        lineno += 1;
        // 1312: lineno: typeof(_10) = *mut {l13} i32
        // 1312: lineno:   l13 = UNIQUE | NON_NULL, (empty)
        lineno;
        // 1313: lineno: typeof(_13) = *mut {l17} i32
        // 1313: lineno:   l17 = UNIQUE | NON_NULL, (empty)
    }
    return res;
}
unsafe extern "C" fn add(mut lit: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nworkers {
    // 1320: nworkers: typeof(_8) = *mut {l8} i32
    // 1320: nworkers:   l8 = UNIQUE | NON_NULL, (empty)
        lgladd((*workers.offset(i as isize)).lgl, lit);
        // 1321: (*workers.offse ... ).lgl: typeof(_10) = *mut {l11} LGL
        // 1321: (*workers.offse ... ).lgl:   l11 = UNIQUE | NON_NULL, (empty)
        // 1321: workers.offset( ... size): typeof(_11) = *mut {l13} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1321: workers.offset( ... size):   l13 = UNIQUE | NON_NULL, (empty)
        // 1321: workers: typeof(_12) = *mut {l15} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1321: workers:   l15 = UNIQUE | NON_NULL, (empty)
        // 1321: workers: typeof(_13) = *mut {l17} *mut {l18} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1321: workers:   l17 = UNIQUE | NON_NULL, (empty)
        // 1321: workers:   l18 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
}
unsafe extern "C" fn parse() {
    let mut ch: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut assumption: *mut libc::c_int = 0 as *mut libc::c_int;
    // 1330: mut assumption: typeof(_4) = *mut {l4} i32
    // 1330: mut assumption:   l4 = UNIQUE | NON_NULL, (empty)
    // 1330: 0 as *mut libc: ... c_int: typeof(_4 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l1183} i32
    // 1330: 0 as *mut libc: ... c_int:   l1183 = UNIQUE | NON_NULL, (empty)
    let mut i: libc::c_int = 0;
    loop {
        ch = next();
        if ch == -(1 as libc::c_int) {
            perr(b"unexpected end-of-file in header\0" as *const u8 as *const libc::c_char);
            // 1335: b"unexpected en ... _char: typeof(_16) = *const {l17} i8
            // 1335: b"unexpected en ... _char:   l17 = UNIQUE | NON_NULL, (empty)
            // 1335: b"unexpected en ... st u8: typeof(_17) = *const {l19} u8
            // 1335: b"unexpected en ... st u8:   l19 = UNIQUE | NON_NULL, (empty)
            // 1335: b"unexpected en ... er\0": typeof(_18) = *const {l21} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1335: b"unexpected en ... er\0":   l21 = UNIQUE | NON_NULL, (empty)
            // 1335: b"unexpected en ... er\0": typeof(_19) = & {l23} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1335: b"unexpected en ... er\0":   l23 = UNIQUE | NON_NULL, FIXED
            // 1335: b"unexpected en ... _char: typeof(_16 = move _17 as *const i8 (Misc)) = *const {l1187} i8
            // 1335: b"unexpected en ... _char:   l1187 = UNIQUE | NON_NULL, (empty)
            // 1335: b"unexpected en ... er\0": typeof(_18 = &raw const (*_19)) = *const {l1185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1335: b"unexpected en ... er\0":   l1185 = UNIQUE | NON_NULL, (empty)
            // 1335: b"unexpected en ... st u8: typeof(_17 = move _18 as *const u8 (Pointer(ArrayToPointer))) = *const {l1186} u8
            // 1335: b"unexpected en ... st u8:   l1186 = UNIQUE | NON_NULL, (empty)
            // 1335: b"unexpected en ... er\0": typeof(_19 = const b"unexpected end-of-file in header\x00") = & {l1184} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 1335: b"unexpected en ... er\0":   l1184 = UNIQUE | NON_NULL, (empty)
        }
        if !(ch == 'c' as i32) {
            break;
        }
        loop {
            ch = next();
            if !(ch != '\n' as i32) {
                break;
            }
            if ch == -(1 as libc::c_int) {
                perr(
                    b"unexpected end-of-file in header comment\0" as *const u8
                    // 1347: b"unexpected en ... _char: typeof(_39) = *const {l44} i8
                    // 1347: b"unexpected en ... _char:   l44 = UNIQUE | NON_NULL, (empty)
                    // 1347: b"unexpected en ... st u8: typeof(_40) = *const {l46} u8
                    // 1347: b"unexpected en ... st u8:   l46 = UNIQUE | NON_NULL, (empty)
                    // 1347: b"unexpected en ... nt\0": typeof(_41) = *const {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 1347: b"unexpected en ... nt\0":   l48 = UNIQUE | NON_NULL, (empty)
                    // 1347: b"unexpected en ... nt\0": typeof(_42) = & {l50} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 1347: b"unexpected en ... nt\0":   l50 = UNIQUE | NON_NULL, FIXED
                    // 1347: b"unexpected en ... st u8: typeof(_40 = move _41 as *const u8 (Pointer(ArrayToPointer))) = *const {l1190} u8
                    // 1347: b"unexpected en ... st u8:   l1190 = UNIQUE | NON_NULL, (empty)
                    // 1347: b"unexpected en ... _char: typeof(_39 = move _40 as *const i8 (Misc)) = *const {l1191} i8
                    // 1347: b"unexpected en ... _char:   l1191 = UNIQUE | NON_NULL, (empty)
                    // 1347: b"unexpected en ... nt\0": typeof(_42 = const b"unexpected end-of-file in header comment\x00") = & {l1188} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 1347: b"unexpected en ... nt\0":   l1188 = UNIQUE | NON_NULL, (empty)
                    // 1347: b"unexpected en ... nt\0": typeof(_41 = &raw const (*_42)) = *const {l1189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 1347: b"unexpected en ... nt\0":   l1189 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
            }
        }
    }
    if ch != 'p' as i32
        || next() != ' ' as i32
        || next() != 'i' as i32
        || next() != 'n' as i32
        || next() != 'c' as i32
        || next() != 'c' as i32
        || next() != 'n' as i32
        || next() != 'f' as i32
    {
        perr(b"invalid header (expected 'p inccnf')\0" as *const u8 as *const libc::c_char);
        // 1362: b"invalid heade ... _char: typeof(_76) = *const {l85} i8
        // 1362: b"invalid heade ... _char:   l85 = UNIQUE | NON_NULL, (empty)
        // 1362: b"invalid heade ... st u8: typeof(_77) = *const {l87} u8
        // 1362: b"invalid heade ... st u8:   l87 = UNIQUE | NON_NULL, (empty)
        // 1362: b"invalid heade ... ')\0": typeof(_78) = *const {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 1362: b"invalid heade ... ')\0":   l89 = UNIQUE | NON_NULL, (empty)
        // 1362: b"invalid heade ... ')\0": typeof(_79) = & {l91} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 1362: b"invalid heade ... ')\0":   l91 = UNIQUE | NON_NULL, FIXED
        // 1362: b"invalid heade ... st u8: typeof(_77 = move _78 as *const u8 (Pointer(ArrayToPointer))) = *const {l1194} u8
        // 1362: b"invalid heade ... st u8:   l1194 = UNIQUE | NON_NULL, (empty)
        // 1362: b"invalid heade ... _char: typeof(_76 = move _77 as *const i8 (Misc)) = *const {l1195} i8
        // 1362: b"invalid heade ... _char:   l1195 = UNIQUE | NON_NULL, (empty)
        // 1362: b"invalid heade ... ')\0": typeof(_78 = &raw const (*_79)) = *const {l1193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 1362: b"invalid heade ... ')\0":   l1193 = UNIQUE | NON_NULL, (empty)
        // 1362: b"invalid heade ... ')\0": typeof(_79 = const b"invalid header (expected \'p inccnf\')\x00") = & {l1192} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 1362: b"invalid heade ... ')\0":   l1192 = UNIQUE | NON_NULL, (empty)
    }
    ch = next();
    '_CLAUSES: loop {
        if ch == ' ' as i32 || ch == '\t' as i32 || ch == '\r' as i32 || ch == '\n' as i32 {
            ch = next();
        } else if ch == 'c' as i32 {
            loop {
                ch = next();
                if !(ch != '\n' as i32) {
                    break;
                }
                if ch == -(1 as libc::c_int) {
                    perr(
                        b"unexpected end-of-file in body comment\0" as *const u8
                        // 1376: b"unexpected en ... _char: typeof(_114) = *const {l127} i8
                        // 1376: b"unexpected en ... _char:   l127 = UNIQUE | NON_NULL, (empty)
                        // 1376: b"unexpected en ... st u8: typeof(_115) = *const {l129} u8
                        // 1376: b"unexpected en ... st u8:   l129 = UNIQUE | NON_NULL, (empty)
                        // 1376: b"unexpected en ... nt\0": typeof(_116) = *const {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1376: b"unexpected en ... nt\0":   l131 = UNIQUE | NON_NULL, (empty)
                        // 1376: b"unexpected en ... nt\0": typeof(_117) = & {l133} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1376: b"unexpected en ... nt\0":   l133 = UNIQUE | NON_NULL, FIXED
                        // 1376: b"unexpected en ... nt\0": typeof(_116 = &raw const (*_117)) = *const {l1197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1376: b"unexpected en ... nt\0":   l1197 = UNIQUE | NON_NULL, (empty)
                        // 1376: b"unexpected en ... st u8: typeof(_115 = move _116 as *const u8 (Pointer(ArrayToPointer))) = *const {l1198} u8
                        // 1376: b"unexpected en ... st u8:   l1198 = UNIQUE | NON_NULL, (empty)
                        // 1376: b"unexpected en ... _char: typeof(_114 = move _115 as *const i8 (Misc)) = *const {l1199} i8
                        // 1376: b"unexpected en ... _char:   l1199 = UNIQUE | NON_NULL, (empty)
                        // 1376: b"unexpected en ... nt\0": typeof(_117 = const b"unexpected end-of-file in body comment\x00") = & {l1196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1376: b"unexpected en ... nt\0":   l1196 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                }
            }
        } else {
            if ch == -(1 as libc::c_int) && nlits != 0 {
            // 1382: nlits: typeof(_127) = *mut {l144} i32
            // 1382: nlits:   l144 = UNIQUE | NON_NULL, (empty)
                perr(b"unexpected end-of-file in clause\0" as *const u8 as *const libc::c_char);
                // 1383: b"unexpected en ... _char: typeof(_129) = *const {l147} i8
                // 1383: b"unexpected en ... _char:   l147 = UNIQUE | NON_NULL, (empty)
                // 1383: b"unexpected en ... st u8: typeof(_130) = *const {l149} u8
                // 1383: b"unexpected en ... st u8:   l149 = UNIQUE | NON_NULL, (empty)
                // 1383: b"unexpected en ... se\0": typeof(_131) = *const {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 1383: b"unexpected en ... se\0":   l151 = UNIQUE | NON_NULL, (empty)
                // 1383: b"unexpected en ... se\0": typeof(_132) = & {l153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 1383: b"unexpected en ... se\0":   l153 = UNIQUE | NON_NULL, FIXED
                // 1383: b"unexpected en ... _char: typeof(_129 = move _130 as *const i8 (Misc)) = *const {l1203} i8
                // 1383: b"unexpected en ... _char:   l1203 = UNIQUE | NON_NULL, (empty)
                // 1383: b"unexpected en ... se\0": typeof(_131 = &raw const (*_132)) = *const {l1201} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 1383: b"unexpected en ... se\0":   l1201 = UNIQUE | NON_NULL, (empty)
                // 1383: b"unexpected en ... se\0": typeof(_132 = const b"unexpected end-of-file in clause\x00") = & {l1200} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 1383: b"unexpected en ... se\0":   l1200 = UNIQUE | NON_NULL, (empty)
                // 1383: b"unexpected en ... st u8: typeof(_130 = move _131 as *const u8 (Pointer(ArrayToPointer))) = *const {l1202} u8
                // 1383: b"unexpected en ... st u8:   l1202 = UNIQUE | NON_NULL, (empty)
            }
            if ch == 'a' as i32 && nlits != 0 {
            // 1385: nlits: typeof(_140) = *mut {l162} i32
            // 1385: nlits:   l162 = UNIQUE | NON_NULL, (empty)
                perr(b"unexpected 'a' in clause\0" as *const u8 as *const libc::c_char);
                // 1386: b"unexpected 'a ... _char: typeof(_142) = *const {l165} i8
                // 1386: b"unexpected 'a ... _char:   l165 = UNIQUE | NON_NULL, (empty)
                // 1386: b"unexpected 'a ... st u8: typeof(_143) = *const {l167} u8
                // 1386: b"unexpected 'a ... st u8:   l167 = UNIQUE | NON_NULL, (empty)
                // 1386: b"unexpected 'a ... se\0": typeof(_144) = *const {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1386: b"unexpected 'a ... se\0":   l169 = UNIQUE | NON_NULL, (empty)
                // 1386: b"unexpected 'a ... se\0": typeof(_145) = & {l171} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1386: b"unexpected 'a ... se\0":   l171 = UNIQUE | NON_NULL, FIXED
                // 1386: b"unexpected 'a ... se\0": typeof(_144 = &raw const (*_145)) = *const {l1205} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1386: b"unexpected 'a ... se\0":   l1205 = UNIQUE | NON_NULL, (empty)
                // 1386: b"unexpected 'a ... st u8: typeof(_143 = move _144 as *const u8 (Pointer(ArrayToPointer))) = *const {l1206} u8
                // 1386: b"unexpected 'a ... st u8:   l1206 = UNIQUE | NON_NULL, (empty)
                // 1386: b"unexpected 'a ... se\0": typeof(_145 = const b"unexpected \'a\' in clause\x00") = & {l1204} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1386: b"unexpected 'a ... se\0":   l1204 = UNIQUE | NON_NULL, (empty)
                // 1386: b"unexpected 'a ... _char: typeof(_142 = move _143 as *const i8 (Misc)) = *const {l1207} i8
                // 1386: b"unexpected 'a ... _char:   l1207 = UNIQUE | NON_NULL, (empty)
            }
            if !(ch == 'a' as i32) {
                if ch == -(1 as libc::c_int) {
                    break;
                }
                if !(ch == 'a' as i32) {
                    if ch == '-' as i32 {
                        sign = -(1 as libc::c_int);
                        ch = next();
                        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        // 1396: (*__ctype_b_loc ... size): typeof(_174) = *const {l201} u16
                        // 1396: (*__ctype_b_loc ... size):   l201 = UNIQUE | NON_NULL, (empty)
                        // 1396: (*__ctype_b_loc()): typeof(_175) = *const {l203} u16
                        // 1396: (*__ctype_b_loc()):   l203 = UNIQUE | NON_NULL, (empty)
                        // 1396: __ctype_b_loc(): typeof(_176) = *mut {l205} *const {l206} u16
                        // 1396: __ctype_b_loc():   l205 = UNIQUE | NON_NULL, (empty)
                        // 1396: __ctype_b_loc():   l206 = UNIQUE | NON_NULL, (empty)
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            perr(b"expected digit after '-'\0" as *const u8 as *const libc::c_char);
                            // 1400: b"expected digi ... _char: typeof(_183) = *const {l214} i8
                            // 1400: b"expected digi ... _char:   l214 = UNIQUE | NON_NULL, (empty)
                            // 1400: b"expected digi ... st u8: typeof(_184) = *const {l216} u8
                            // 1400: b"expected digi ... st u8:   l216 = UNIQUE | NON_NULL, (empty)
                            // 1400: b"expected digi ... -'\0": typeof(_185) = *const {l218} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1400: b"expected digi ... -'\0":   l218 = UNIQUE | NON_NULL, (empty)
                            // 1400: b"expected digi ... -'\0": typeof(_186) = & {l220} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1400: b"expected digi ... -'\0":   l220 = UNIQUE | NON_NULL, FIXED
                            // 1400: b"expected digi ... _char: typeof(_183 = move _184 as *const i8 (Misc)) = *const {l1211} i8
                            // 1400: b"expected digi ... _char:   l1211 = UNIQUE | NON_NULL, (empty)
                            // 1400: b"expected digi ... -'\0": typeof(_185 = &raw const (*_186)) = *const {l1209} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1400: b"expected digi ... -'\0":   l1209 = UNIQUE | NON_NULL, (empty)
                            // 1400: b"expected digi ... -'\0": typeof(_186 = const b"expected digit after \'-\'\x00") = & {l1208} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1400: b"expected digi ... -'\0":   l1208 = UNIQUE | NON_NULL, (empty)
                            // 1400: b"expected digi ... st u8: typeof(_184 = move _185 as *const u8 (Pointer(ArrayToPointer))) = *const {l1210} u8
                            // 1400: b"expected digi ... st u8:   l1210 = UNIQUE | NON_NULL, (empty)
                        }
                    } else {
                        sign = 1 as libc::c_int;
                    }
                    if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    // 1405: (*__ctype_b_loc ... size): typeof(_193) = *const {l228} u16
                    // 1405: (*__ctype_b_loc ... size):   l228 = UNIQUE | NON_NULL, (empty)
                    // 1405: (*__ctype_b_loc()): typeof(_194) = *const {l230} u16
                    // 1405: (*__ctype_b_loc()):   l230 = UNIQUE | NON_NULL, (empty)
                    // 1405: __ctype_b_loc(): typeof(_195) = *mut {l232} *const {l233} u16
                    // 1405: __ctype_b_loc():   l232 = UNIQUE | NON_NULL, (empty)
                    // 1405: __ctype_b_loc():   l233 = UNIQUE | NON_NULL, (empty)
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        == 0
                    {
                        perr(b"expected literal\0" as *const u8 as *const libc::c_char);
                        // 1409: b"expected lite ... _char: typeof(_202) = *const {l241} i8
                        // 1409: b"expected lite ... _char:   l241 = UNIQUE | NON_NULL, (empty)
                        // 1409: b"expected lite ... st u8: typeof(_203) = *const {l243} u8
                        // 1409: b"expected lite ... st u8:   l243 = UNIQUE | NON_NULL, (empty)
                        // 1409: b"expected lite ... al\0": typeof(_204) = *const {l245} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1409: b"expected lite ... al\0":   l245 = UNIQUE | NON_NULL, (empty)
                        // 1409: b"expected lite ... al\0": typeof(_205) = & {l247} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1409: b"expected lite ... al\0":   l247 = UNIQUE | NON_NULL, FIXED
                        // 1409: b"expected lite ... _char: typeof(_202 = move _203 as *const i8 (Misc)) = *const {l1215} i8
                        // 1409: b"expected lite ... _char:   l1215 = UNIQUE | NON_NULL, (empty)
                        // 1409: b"expected lite ... st u8: typeof(_203 = move _204 as *const u8 (Pointer(ArrayToPointer))) = *const {l1214} u8
                        // 1409: b"expected lite ... st u8:   l1214 = UNIQUE | NON_NULL, (empty)
                        // 1409: b"expected lite ... al\0": typeof(_204 = &raw const (*_205)) = *const {l1213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1409: b"expected lite ... al\0":   l1213 = UNIQUE | NON_NULL, (empty)
                        // 1409: b"expected lite ... al\0": typeof(_205 = const b"expected literal\x00") = & {l1212} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1409: b"expected lite ... al\0":   l1212 = UNIQUE | NON_NULL, (empty)
                    }
                    lit = ch - '0' as i32;
                    loop {
                        ch = next();
                        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        // 1414: (*__ctype_b_loc ... size): typeof(_217) = *const {l260} u16
                        // 1414: (*__ctype_b_loc ... size):   l260 = UNIQUE | NON_NULL, (empty)
                        // 1414: (*__ctype_b_loc()): typeof(_218) = *const {l262} u16
                        // 1414: (*__ctype_b_loc()):   l262 = UNIQUE | NON_NULL, (empty)
                        // 1414: __ctype_b_loc(): typeof(_219) = *mut {l264} *const {l265} u16
                        // 1414: __ctype_b_loc():   l264 = UNIQUE | NON_NULL, (empty)
                        // 1414: __ctype_b_loc():   l265 = UNIQUE | NON_NULL, (empty)
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                        lit = 10 as libc::c_int * lit + ch - '0' as i32;
                    }
                    if lit > nvars {
                    // 1422: nvars: typeof(_239) = *mut {l286} i32
                    // 1422: nvars:   l286 = UNIQUE | NON_NULL, (empty)
                        if lit >= szvars {
                        // 1423: szvars: typeof(_244) = *mut {l292} i32
                        // 1423: szvars:   l292 = UNIQUE | NON_NULL, (empty)
                            let mut oldszvars: libc::c_int = szvars;
                            // 1424: szvars: typeof(_246) = *mut {l295} i32
                            // 1424: szvars:   l295 = UNIQUE | NON_NULL, (empty)
                            szvars = if szvars != 0 {
                            // 1425: szvars: typeof(_250) = *mut {l300} i32
                            // 1425: szvars:   l300 = UNIQUE | NON_NULL, (empty)
                            // 1425: szvars: typeof(_255) = *mut {l307} i32
                            // 1425: szvars:   l307 = UNIQUE | NON_NULL, (empty)
                                2 as libc::c_int * szvars
                                // 1426: szvars: typeof(_253) = *mut {l304} i32
                                // 1426: szvars:   l304 = UNIQUE | NON_NULL, (empty)
                            } else {
                                1 as libc::c_int
                            };
                            while szvars <= lit {
                            // 1430: szvars: typeof(_259) = *mut {l312} i32
                            // 1430: szvars:   l312 = UNIQUE | NON_NULL, (empty)
                                szvars *= 2 as libc::c_int;
                                // 1431: szvars: typeof(_262) = *mut {l316} i32
                                // 1431: szvars:   l316 = UNIQUE | NON_NULL, (empty)
                            }
                            let mut OBYTES: size_t =
                                (oldszvars as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                    libc::c_int,
                                >(
                                )
                                    as libc::c_ulong);
                            let mut NBYTES: size_t =
                                (szvars as libc::c_ulong).wrapping_mul(::core::mem::size_of::<
                                // 1440: szvars: typeof(_275) = *mut {l330} i32
                                // 1440: szvars:   l330 = UNIQUE | NON_NULL, (empty)
                                    libc::c_int,
                                >(
                                )
                                    as libc::c_ulong);
                            allocated = allocated.wrapping_sub(OBYTES);
                            // 1445: allocated: typeof(_280) = *mut {l336} u64
                            // 1445: allocated:   l336 = UNIQUE | NON_NULL, (empty)
                            // 1445: allocated: typeof(_282) = *mut {l339} u64
                            // 1445: allocated:   l339 = UNIQUE | NON_NULL, (empty)
                            used = realloc(used as *mut libc::c_void, NBYTES) as *mut libc::c_int;
                            // 1446: realloc(used as ... YTES): typeof(_283) = *mut {l341} DefId(2:5583 ~ core[480a]::ffi::c_void)
                            // 1446: realloc(used as ... YTES):   l341 = UNIQUE | NON_NULL, (empty)
                            // 1446: used as *mut li ... _void: typeof(_284) = *mut {l343} DefId(2:5583 ~ core[480a]::ffi::c_void)
                            // 1446: used as *mut li ... _void:   l343 = UNIQUE | NON_NULL, (empty)
                            // 1446: used: typeof(_285) = *mut {l345} i32
                            // 1446: used:   l345 = UNIQUE | NON_NULL, (empty)
                            // 1446: used: typeof(_286) = *mut {l347} *mut {l348} i32
                            // 1446: used:   l347 = UNIQUE | NON_NULL, (empty)
                            // 1446: used:   l348 = UNIQUE | NON_NULL, (empty)
                            // 1446: used: typeof(_288) = *mut {l351} *mut {l352} i32
                            // 1446: used:   l351 = UNIQUE | NON_NULL, (empty)
                            // 1446: used:   l352 = UNIQUE | NON_NULL, (empty)
                            // 1446: used = realloc( ... c_int: typeof((*_288) = move _283 as *mut i32 (Misc)) = *mut {l1217} i32
                            // 1446: used = realloc( ... c_int:   l1217 = UNIQUE | NON_NULL, (empty)
                            // 1446: used as *mut li ... _void: typeof(_284 = move _285 as *mut libc::c_void (Misc)) = *mut {l1216} DefId(2:5583 ~ core[480a]::ffi::c_void)
                            // 1446: used as *mut li ... _void:   l1216 = UNIQUE | NON_NULL, (empty)
                            if used.is_null() {
                            // 1447: used: typeof(_291) = *mut {l356} i32
                            // 1447: used:   l356 = UNIQUE | NON_NULL, (empty)
                            // 1447: used: typeof(_292) = *mut {l358} *mut {l359} i32
                            // 1447: used:   l358 = UNIQUE | NON_NULL, (empty)
                            // 1447: used:   l359 = UNIQUE | NON_NULL, (empty)
                                die(b"out of memory\0" as *const u8 as *const libc::c_char);
                                // 1448: b"out of memory ... _char: typeof(_294) = *const {l362} i8
                                // 1448: b"out of memory ... _char:   l362 = UNIQUE | NON_NULL, (empty)
                                // 1448: b"out of memory ... st u8: typeof(_295) = *const {l364} u8
                                // 1448: b"out of memory ... st u8:   l364 = UNIQUE | NON_NULL, (empty)
                                // 1448: b"out of memory\0": typeof(_296) = *const {l366} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1448: b"out of memory\0":   l366 = UNIQUE | NON_NULL, (empty)
                                // 1448: b"out of memory\0": typeof(_297) = & {l368} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1448: b"out of memory\0":   l368 = UNIQUE | NON_NULL, FIXED
                                // 1448: b"out of memory\0": typeof(_296 = &raw const (*_297)) = *const {l1219} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1448: b"out of memory\0":   l1219 = UNIQUE | NON_NULL, (empty)
                                // 1448: b"out of memory\0": typeof(_297 = const b"out of memory\x00") = & {l1218} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                                // 1448: b"out of memory\0":   l1218 = UNIQUE | NON_NULL, (empty)
                                // 1448: b"out of memory ... _char: typeof(_294 = move _295 as *const i8 (Misc)) = *const {l1221} i8
                                // 1448: b"out of memory ... _char:   l1221 = UNIQUE | NON_NULL, (empty)
                                // 1448: b"out of memory ... st u8: typeof(_295 = move _296 as *const u8 (Pointer(ArrayToPointer))) = *const {l1220} u8
                                // 1448: b"out of memory ... st u8:   l1220 = UNIQUE | NON_NULL, (empty)
                            }
                            allocated = allocated.wrapping_add(NBYTES);
                            // 1450: allocated: typeof(_300) = *mut {l372} u64
                            // 1450: allocated:   l372 = UNIQUE | NON_NULL, (empty)
                            // 1450: allocated: typeof(_302) = *mut {l375} u64
                            // 1450: allocated:   l375 = UNIQUE | NON_NULL, (empty)
                            if allocated > maxallocated {
                            // 1451: allocated: typeof(_306) = *mut {l380} u64
                            // 1451: allocated:   l380 = UNIQUE | NON_NULL, (empty)
                            // 1451: maxallocated: typeof(_308) = *mut {l383} u64
                            // 1451: maxallocated:   l383 = UNIQUE | NON_NULL, (empty)
                                maxallocated = allocated;
                                // 1452: allocated: typeof(_310) = *mut {l386} u64
                                // 1452: allocated:   l386 = UNIQUE | NON_NULL, (empty)
                                // 1452: maxallocated: typeof(_311) = *mut {l388} u64
                                // 1452: maxallocated:   l388 = UNIQUE | NON_NULL, (empty)
                            }
                            i = oldszvars;
                            while i < szvars {
                            // 1455: szvars: typeof(_316) = *mut {l394} i32
                            // 1455: szvars:   l394 = UNIQUE | NON_NULL, (empty)
                                *used.offset(i as isize) = -(1 as libc::c_int);
                                // 1456: used.offset(i a ... size): typeof(_319) = *mut {l398} i32
                                // 1456: used.offset(i a ... size):   l398 = UNIQUE | NON_NULL, (empty)
                                // 1456: used: typeof(_320) = *mut {l400} i32
                                // 1456: used:   l400 = UNIQUE | NON_NULL, (empty)
                                // 1456: used: typeof(_321) = *mut {l402} *mut {l403} i32
                                // 1456: used:   l402 = UNIQUE | NON_NULL, (empty)
                                // 1456: used:   l403 = UNIQUE | NON_NULL, (empty)
                                i += 1;
                                i;
                            }
                        }
                        nvars = lit;
                        // 1461: nvars: typeof(_330) = *mut {l413} i32
                        // 1461: nvars:   l413 = UNIQUE | NON_NULL, (empty)
                    }
                    lit *= sign;
                    if lit != 0 {
                        nlits += 1;
                        // 1465: nlits: typeof(_336) = *mut {l420} i32
                        // 1465: nlits:   l420 = UNIQUE | NON_NULL, (empty)
                        nlits;
                        // 1466: nlits: typeof(_339) = *mut {l424} i32
                        // 1466: nlits:   l424 = UNIQUE | NON_NULL, (empty)
                    } else {
                        nlits = 0 as libc::c_int;
                        // 1468: nlits: typeof(_341) = *mut {l427} i32
                        // 1468: nlits:   l427 = UNIQUE | NON_NULL, (empty)
                        nclauses += 1;
                        // 1469: nclauses: typeof(_342) = *mut {l429} i32
                        // 1469: nclauses:   l429 = UNIQUE | NON_NULL, (empty)
                        nclauses;
                        // 1470: nclauses: typeof(_345) = *mut {l433} i32
                        // 1470: nclauses:   l433 = UNIQUE | NON_NULL, (empty)
                    }
                    add(lit);
                    continue;
                }
            }
            loop {
                ch = next();
                if ch != ' ' as i32 {
                    perr(b"expected space after 'a'\0" as *const u8 as *const libc::c_char);
                    // 1479: b"expected spac ... _char: typeof(_354) = *const {l443} i8
                    // 1479: b"expected spac ... _char:   l443 = UNIQUE | NON_NULL, (empty)
                    // 1479: b"expected spac ... st u8: typeof(_355) = *const {l445} u8
                    // 1479: b"expected spac ... st u8:   l445 = UNIQUE | NON_NULL, (empty)
                    // 1479: b"expected spac ... a'\0": typeof(_356) = *const {l447} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 1479: b"expected spac ... a'\0":   l447 = UNIQUE | NON_NULL, (empty)
                    // 1479: b"expected spac ... a'\0": typeof(_357) = & {l449} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 1479: b"expected spac ... a'\0":   l449 = UNIQUE | NON_NULL, FIXED
                    // 1479: b"expected spac ... st u8: typeof(_355 = move _356 as *const u8 (Pointer(ArrayToPointer))) = *const {l1224} u8
                    // 1479: b"expected spac ... st u8:   l1224 = UNIQUE | NON_NULL, (empty)
                    // 1479: b"expected spac ... a'\0": typeof(_357 = const b"expected space after \'a\'\x00") = & {l1222} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 1479: b"expected spac ... a'\0":   l1222 = UNIQUE | NON_NULL, (empty)
                    // 1479: b"expected spac ... _char: typeof(_354 = move _355 as *const i8 (Misc)) = *const {l1225} i8
                    // 1479: b"expected spac ... _char:   l1225 = UNIQUE | NON_NULL, (empty)
                    // 1479: b"expected spac ... a'\0": typeof(_356 = &raw const (*_357)) = *const {l1223} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 1479: b"expected spac ... a'\0":   l1223 = UNIQUE | NON_NULL, (empty)
                }
                loop {
                    ch = next();
                    if ch == ' ' as i32
                        || ch == '\t' as i32
                        || ch == '\r' as i32
                        || ch == '\n' as i32
                    {
                        continue;
                    }
                    if ch == -(1 as libc::c_int) && nlits != 0 {
                    // 1490: nlits: typeof(_386) = *mut {l479} i32
                    // 1490: nlits:   l479 = UNIQUE | NON_NULL, (empty)
                        perr(
                            b"unexpected end-of-file in assumptions\0" as *const u8
                            // 1492: b"unexpected en ... _char: typeof(_388) = *const {l482} i8
                            // 1492: b"unexpected en ... _char:   l482 = UNIQUE | NON_NULL, (empty)
                            // 1492: b"unexpected en ... st u8: typeof(_389) = *const {l484} u8
                            // 1492: b"unexpected en ... st u8:   l484 = UNIQUE | NON_NULL, (empty)
                            // 1492: b"unexpected en ... ns\0": typeof(_390) = *const {l486} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1492: b"unexpected en ... ns\0":   l486 = UNIQUE | NON_NULL, (empty)
                            // 1492: b"unexpected en ... ns\0": typeof(_391) = & {l488} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1492: b"unexpected en ... ns\0":   l488 = UNIQUE | NON_NULL, FIXED
                            // 1492: b"unexpected en ... ns\0": typeof(_390 = &raw const (*_391)) = *const {l1227} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1492: b"unexpected en ... ns\0":   l1227 = UNIQUE | NON_NULL, (empty)
                            // 1492: b"unexpected en ... st u8: typeof(_389 = move _390 as *const u8 (Pointer(ArrayToPointer))) = *const {l1228} u8
                            // 1492: b"unexpected en ... st u8:   l1228 = UNIQUE | NON_NULL, (empty)
                            // 1492: b"unexpected en ... ns\0": typeof(_391 = const b"unexpected end-of-file in assumptions\x00") = & {l1226} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                            // 1492: b"unexpected en ... ns\0":   l1226 = UNIQUE | NON_NULL, (empty)
                            // 1492: b"unexpected en ... _char: typeof(_388 = move _389 as *const i8 (Misc)) = *const {l1229} i8
                            // 1492: b"unexpected en ... _char:   l1229 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                        );
                    }
                    if ch == '-' as i32 {
                        sign = -(1 as libc::c_int);
                        ch = next();
                        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        // 1499: (*__ctype_b_loc ... size): typeof(_403) = *const {l501} u16
                        // 1499: (*__ctype_b_loc ... size):   l501 = UNIQUE | NON_NULL, (empty)
                        // 1499: (*__ctype_b_loc()): typeof(_404) = *const {l503} u16
                        // 1499: (*__ctype_b_loc()):   l503 = UNIQUE | NON_NULL, (empty)
                        // 1499: __ctype_b_loc(): typeof(_405) = *mut {l505} *const {l506} u16
                        // 1499: __ctype_b_loc():   l505 = UNIQUE | NON_NULL, (empty)
                        // 1499: __ctype_b_loc():   l506 = UNIQUE | NON_NULL, (empty)
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            == 0
                        {
                            perr(b"expected digit after '-'\0" as *const u8 as *const libc::c_char);
                            // 1503: b"expected digi ... _char: typeof(_412) = *const {l514} i8
                            // 1503: b"expected digi ... _char:   l514 = UNIQUE | NON_NULL, (empty)
                            // 1503: b"expected digi ... st u8: typeof(_413) = *const {l516} u8
                            // 1503: b"expected digi ... st u8:   l516 = UNIQUE | NON_NULL, (empty)
                            // 1503: b"expected digi ... -'\0": typeof(_414) = *const {l518} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1503: b"expected digi ... -'\0":   l518 = UNIQUE | NON_NULL, (empty)
                            // 1503: b"expected digi ... -'\0": typeof(_415) = & {l520} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1503: b"expected digi ... -'\0":   l520 = UNIQUE | NON_NULL, FIXED
                            // 1503: b"expected digi ... -'\0": typeof(_414 = &raw const (*_415)) = *const {l1231} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1503: b"expected digi ... -'\0":   l1231 = UNIQUE | NON_NULL, (empty)
                            // 1503: b"expected digi ... st u8: typeof(_413 = move _414 as *const u8 (Pointer(ArrayToPointer))) = *const {l1232} u8
                            // 1503: b"expected digi ... st u8:   l1232 = UNIQUE | NON_NULL, (empty)
                            // 1503: b"expected digi ... -'\0": typeof(_415 = const b"expected digit after \'-\'\x00") = & {l1230} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                            // 1503: b"expected digi ... -'\0":   l1230 = UNIQUE | NON_NULL, (empty)
                            // 1503: b"expected digi ... _char: typeof(_412 = move _413 as *const i8 (Misc)) = *const {l1233} i8
                            // 1503: b"expected digi ... _char:   l1233 = UNIQUE | NON_NULL, (empty)
                        }
                    } else {
                        sign = 1 as libc::c_int;
                    }
                    if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    // 1508: (*__ctype_b_loc ... size): typeof(_422) = *const {l528} u16
                    // 1508: (*__ctype_b_loc ... size):   l528 = UNIQUE | NON_NULL, (empty)
                    // 1508: (*__ctype_b_loc()): typeof(_423) = *const {l530} u16
                    // 1508: (*__ctype_b_loc()):   l530 = UNIQUE | NON_NULL, (empty)
                    // 1508: __ctype_b_loc(): typeof(_424) = *mut {l532} *const {l533} u16
                    // 1508: __ctype_b_loc():   l532 = UNIQUE | NON_NULL, (empty)
                    // 1508: __ctype_b_loc():   l533 = UNIQUE | NON_NULL, (empty)
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        == 0
                    {
                        perr(b"expected literal\0" as *const u8 as *const libc::c_char);
                        // 1512: b"expected lite ... _char: typeof(_431) = *const {l541} i8
                        // 1512: b"expected lite ... _char:   l541 = UNIQUE | NON_NULL, (empty)
                        // 1512: b"expected lite ... st u8: typeof(_432) = *const {l543} u8
                        // 1512: b"expected lite ... st u8:   l543 = UNIQUE | NON_NULL, (empty)
                        // 1512: b"expected lite ... al\0": typeof(_433) = *const {l545} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1512: b"expected lite ... al\0":   l545 = UNIQUE | NON_NULL, (empty)
                        // 1512: b"expected lite ... al\0": typeof(_434) = & {l547} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1512: b"expected lite ... al\0":   l547 = UNIQUE | NON_NULL, FIXED
                        // 1512: b"expected lite ... al\0": typeof(_434 = const b"expected literal\x00") = & {l1234} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1512: b"expected lite ... al\0":   l1234 = UNIQUE | NON_NULL, (empty)
                        // 1512: b"expected lite ... st u8: typeof(_432 = move _433 as *const u8 (Pointer(ArrayToPointer))) = *const {l1236} u8
                        // 1512: b"expected lite ... st u8:   l1236 = UNIQUE | NON_NULL, (empty)
                        // 1512: b"expected lite ... al\0": typeof(_433 = &raw const (*_434)) = *const {l1235} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                        // 1512: b"expected lite ... al\0":   l1235 = UNIQUE | NON_NULL, (empty)
                        // 1512: b"expected lite ... _char: typeof(_431 = move _432 as *const i8 (Misc)) = *const {l1237} i8
                        // 1512: b"expected lite ... _char:   l1237 = UNIQUE | NON_NULL, (empty)
                    }
                    lit = ch - '0' as i32;
                    loop {
                        ch = next();
                        if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                        // 1517: (*__ctype_b_loc ... size): typeof(_446) = *const {l560} u16
                        // 1517: (*__ctype_b_loc ... size):   l560 = UNIQUE | NON_NULL, (empty)
                        // 1517: (*__ctype_b_loc()): typeof(_447) = *const {l562} u16
                        // 1517: (*__ctype_b_loc()):   l562 = UNIQUE | NON_NULL, (empty)
                        // 1517: __ctype_b_loc(): typeof(_448) = *mut {l564} *const {l565} u16
                        // 1517: __ctype_b_loc():   l564 = UNIQUE | NON_NULL, (empty)
                        // 1517: __ctype_b_loc():   l565 = UNIQUE | NON_NULL, (empty)
                            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                            != 0)
                        {
                            break;
                        }
                        lit = 10 as libc::c_int * lit + ch - '0' as i32;
                    }
                    if lit > nvars {
                    // 1525: nvars: typeof(_468) = *mut {l586} i32
                    // 1525: nvars:   l586 = UNIQUE | NON_NULL, (empty)
                        perr(
                            b"assumption %d exceeds maximum variables %d\0" as *const u8
                            // 1527: b"assumption %d ... _char: typeof(_470) = *const {l589} i8
                            // 1527: b"assumption %d ... _char:   l589 = UNIQUE | NON_NULL, (empty)
                            // 1527: b"assumption %d ... st u8: typeof(_471) = *const {l591} u8
                            // 1527: b"assumption %d ... st u8:   l591 = UNIQUE | NON_NULL, (empty)
                            // 1527: b"assumption %d ... %d\0": typeof(_472) = *const {l593} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                            // 1527: b"assumption %d ... %d\0":   l593 = UNIQUE | NON_NULL, (empty)
                            // 1527: b"assumption %d ... %d\0": typeof(_473) = & {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                            // 1527: b"assumption %d ... %d\0":   l595 = UNIQUE | NON_NULL, FIXED
                            // 1527: b"assumption %d ... %d\0": typeof(_473 = const b"assumption %d exceeds maximum variables %d\x00") = & {l1238} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                            // 1527: b"assumption %d ... %d\0":   l1238 = UNIQUE | NON_NULL, (empty)
                            // 1527: b"assumption %d ... _char: typeof(_470 = move _471 as *const i8 (Misc)) = *const {l1241} i8
                            // 1527: b"assumption %d ... _char:   l1241 = UNIQUE | NON_NULL, (empty)
                            // 1527: b"assumption %d ... %d\0": typeof(_472 = &raw const (*_473)) = *const {l1239} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002b)) }]
                            // 1527: b"assumption %d ... %d\0":   l1239 = UNIQUE | NON_NULL, (empty)
                            // 1527: b"assumption %d ... st u8: typeof(_471 = move _472 as *const u8 (Pointer(ArrayToPointer))) = *const {l1240} u8
                            // 1527: b"assumption %d ... st u8:   l1240 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            lit,
                            nvars,
                            // 1530: nvars: typeof(_476) = *mut {l599} i32
                            // 1530: nvars:   l599 = UNIQUE | NON_NULL, (empty)
                        );
                    }
                    if *used.offset(lit as isize) < 0 as libc::c_int {
                    // 1533: used.offset(lit ... size): typeof(_480) = *mut {l604} i32
                    // 1533: used.offset(lit ... size):   l604 = UNIQUE | NON_NULL, (empty)
                    // 1533: used: typeof(_481) = *mut {l606} i32
                    // 1533: used:   l606 = UNIQUE | NON_NULL, (empty)
                    // 1533: used: typeof(_482) = *mut {l608} *mut {l609} i32
                    // 1533: used:   l608 = UNIQUE | NON_NULL, (empty)
                    // 1533: used:   l609 = UNIQUE | NON_NULL, (empty)
                        nused += 1;
                        // 1534: nused: typeof(_486) = *mut {l614} i32
                        // 1534: nused:   l614 = UNIQUE | NON_NULL, (empty)
                        nused;
                        // 1535: nused: typeof(_489) = *mut {l618} i32
                        // 1535: nused:   l618 = UNIQUE | NON_NULL, (empty)
                    }
                    *used.offset(lit as isize) = nassumptions;
                    // 1537: nassumptions: typeof(_491) = *mut {l621} i32
                    // 1537: nassumptions:   l621 = UNIQUE | NON_NULL, (empty)
                    // 1537: used.offset(lit ... size): typeof(_492) = *mut {l623} i32
                    // 1537: used.offset(lit ... size):   l623 = UNIQUE | NON_NULL, (empty)
                    // 1537: used: typeof(_493) = *mut {l625} i32
                    // 1537: used:   l625 = UNIQUE | NON_NULL, (empty)
                    // 1537: used: typeof(_494) = *mut {l627} *mut {l628} i32
                    // 1537: used:   l627 = UNIQUE | NON_NULL, (empty)
                    // 1537: used:   l628 = UNIQUE | NON_NULL, (empty)
                    lit *= sign;
                    if ch != ' ' as i32
                        && ch != '\t' as i32
                        && ch != '\r' as i32
                        && ch != '\n' as i32
                    {
                        perr(
                            b"expected white space after '%l'\0" as *const u8
                            // 1545: b"expected whit ... _char: typeof(_516) = *const {l651} i8
                            // 1545: b"expected whit ... _char:   l651 = UNIQUE | NON_NULL, (empty)
                            // 1545: b"expected whit ... st u8: typeof(_517) = *const {l653} u8
                            // 1545: b"expected whit ... st u8:   l653 = UNIQUE | NON_NULL, (empty)
                            // 1545: b"expected whit ... l'\0": typeof(_518) = *const {l655} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1545: b"expected whit ... l'\0":   l655 = UNIQUE | NON_NULL, (empty)
                            // 1545: b"expected whit ... l'\0": typeof(_519) = & {l657} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1545: b"expected whit ... l'\0":   l657 = UNIQUE | NON_NULL, FIXED
                            // 1545: b"expected whit ... l'\0": typeof(_519 = const b"expected white space after \'%l\'\x00") = & {l1242} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1545: b"expected whit ... l'\0":   l1242 = UNIQUE | NON_NULL, (empty)
                            // 1545: b"expected whit ... l'\0": typeof(_518 = &raw const (*_519)) = *const {l1243} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1545: b"expected whit ... l'\0":   l1243 = UNIQUE | NON_NULL, (empty)
                            // 1545: b"expected whit ... st u8: typeof(_517 = move _518 as *const u8 (Pointer(ArrayToPointer))) = *const {l1244} u8
                            // 1545: b"expected whit ... st u8:   l1244 = UNIQUE | NON_NULL, (empty)
                            // 1545: b"expected whit ... _char: typeof(_516 = move _517 as *const i8 (Misc)) = *const {l1245} i8
                            // 1545: b"expected whit ... _char:   l1245 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            lit,
                        );
                    }
                    if !(lit != 0) {
                        break;
                    }
                    if nlits >= szlits {
                    // 1553: nlits: typeof(_529) = *mut {l668} i32
                    // 1553: nlits:   l668 = UNIQUE | NON_NULL, (empty)
                    // 1553: szlits: typeof(_531) = *mut {l671} i32
                    // 1553: szlits:   l671 = UNIQUE | NON_NULL, (empty)
                        let mut oldbytes: size_t = 0;
                        let mut newbytes: size_t = 0;
                        oldbytes =
                            (szlits as libc::c_ulong).wrapping_mul(
                            // 1557: szlits: typeof(_537) = *mut {l678} i32
                            // 1557: szlits:   l678 = UNIQUE | NON_NULL, (empty)
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                        allocated = allocated.wrapping_sub(oldbytes);
                        // 1560: allocated: typeof(_542) = *mut {l684} u64
                        // 1560: allocated:   l684 = UNIQUE | NON_NULL, (empty)
                        // 1560: allocated: typeof(_544) = *mut {l687} u64
                        // 1560: allocated:   l687 = UNIQUE | NON_NULL, (empty)
                        szlits = if szlits != 0 {
                        // 1561: szlits: typeof(_548) = *mut {l692} i32
                        // 1561: szlits:   l692 = UNIQUE | NON_NULL, (empty)
                        // 1561: szlits: typeof(_553) = *mut {l699} i32
                        // 1561: szlits:   l699 = UNIQUE | NON_NULL, (empty)
                            2 as libc::c_int * szlits
                            // 1562: szlits: typeof(_551) = *mut {l696} i32
                            // 1562: szlits:   l696 = UNIQUE | NON_NULL, (empty)
                        } else {
                            1 as libc::c_int
                        };
                        newbytes =
                            (szlits as libc::c_ulong).wrapping_mul(
                            // 1567: szlits: typeof(_557) = *mut {l704} i32
                            // 1567: szlits:   l704 = UNIQUE | NON_NULL, (empty)
                                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            );
                        lits = realloc(lits as *mut libc::c_void, newbytes) as *mut libc::c_int;
                        // 1570: realloc(lits as ... ytes): typeof(_560) = *mut {l708} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1570: realloc(lits as ... ytes):   l708 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits as *mut li ... _void: typeof(_561) = *mut {l710} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1570: lits as *mut li ... _void:   l710 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits: typeof(_562) = *mut {l712} i32
                        // 1570: lits:   l712 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits: typeof(_563) = *mut {l714} *mut {l715} i32
                        // 1570: lits:   l714 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits:   l715 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits: typeof(_565) = *mut {l718} *mut {l719} i32
                        // 1570: lits:   l718 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits:   l719 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits = realloc( ... c_int: typeof((*_565) = move _560 as *mut i32 (Misc)) = *mut {l1247} i32
                        // 1570: lits = realloc( ... c_int:   l1247 = UNIQUE | NON_NULL, (empty)
                        // 1570: lits as *mut li ... _void: typeof(_561 = move _562 as *mut libc::c_void (Misc)) = *mut {l1246} DefId(2:5583 ~ core[480a]::ffi::c_void)
                        // 1570: lits as *mut li ... _void:   l1246 = UNIQUE | NON_NULL, (empty)
                        if lits.is_null() {
                        // 1571: lits: typeof(_568) = *mut {l723} i32
                        // 1571: lits:   l723 = UNIQUE | NON_NULL, (empty)
                        // 1571: lits: typeof(_569) = *mut {l725} *mut {l726} i32
                        // 1571: lits:   l725 = UNIQUE | NON_NULL, (empty)
                        // 1571: lits:   l726 = UNIQUE | NON_NULL, (empty)
                            die(b"out of memory\0" as *const u8 as *const libc::c_char);
                            // 1572: b"out of memory ... _char: typeof(_571) = *const {l729} i8
                            // 1572: b"out of memory ... _char:   l729 = UNIQUE | NON_NULL, (empty)
                            // 1572: b"out of memory ... st u8: typeof(_572) = *const {l731} u8
                            // 1572: b"out of memory ... st u8:   l731 = UNIQUE | NON_NULL, (empty)
                            // 1572: b"out of memory\0": typeof(_573) = *const {l733} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1572: b"out of memory\0":   l733 = UNIQUE | NON_NULL, (empty)
                            // 1572: b"out of memory\0": typeof(_574) = & {l735} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1572: b"out of memory\0":   l735 = UNIQUE | NON_NULL, FIXED
                            // 1572: b"out of memory ... st u8: typeof(_572 = move _573 as *const u8 (Pointer(ArrayToPointer))) = *const {l1250} u8
                            // 1572: b"out of memory ... st u8:   l1250 = UNIQUE | NON_NULL, (empty)
                            // 1572: b"out of memory\0": typeof(_573 = &raw const (*_574)) = *const {l1249} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1572: b"out of memory\0":   l1249 = UNIQUE | NON_NULL, (empty)
                            // 1572: b"out of memory ... _char: typeof(_571 = move _572 as *const i8 (Misc)) = *const {l1251} i8
                            // 1572: b"out of memory ... _char:   l1251 = UNIQUE | NON_NULL, (empty)
                            // 1572: b"out of memory\0": typeof(_574 = const b"out of memory\x00") = & {l1248} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                            // 1572: b"out of memory\0":   l1248 = UNIQUE | NON_NULL, (empty)
                        }
                        allocated = allocated.wrapping_add(newbytes);
                        // 1574: allocated: typeof(_577) = *mut {l739} u64
                        // 1574: allocated:   l739 = UNIQUE | NON_NULL, (empty)
                        // 1574: allocated: typeof(_579) = *mut {l742} u64
                        // 1574: allocated:   l742 = UNIQUE | NON_NULL, (empty)
                        if allocated > maxallocated {
                        // 1575: allocated: typeof(_582) = *mut {l746} u64
                        // 1575: allocated:   l746 = UNIQUE | NON_NULL, (empty)
                        // 1575: maxallocated: typeof(_584) = *mut {l749} u64
                        // 1575: maxallocated:   l749 = UNIQUE | NON_NULL, (empty)
                            maxallocated = allocated;
                            // 1576: allocated: typeof(_586) = *mut {l752} u64
                            // 1576: allocated:   l752 = UNIQUE | NON_NULL, (empty)
                            // 1576: maxallocated: typeof(_587) = *mut {l754} u64
                            // 1576: maxallocated:   l754 = UNIQUE | NON_NULL, (empty)
                        }
                    }
                    let fresh5 = nlits;
                    // 1579: nlits: typeof(_589) = *mut {l757} i32
                    // 1579: nlits:   l757 = UNIQUE | NON_NULL, (empty)
                    nlits = nlits + 1;
                    // 1580: nlits: typeof(_591) = *mut {l760} i32
                    // 1580: nlits:   l760 = UNIQUE | NON_NULL, (empty)
                    // 1580: nlits: typeof(_593) = *mut {l763} i32
                    // 1580: nlits:   l763 = UNIQUE | NON_NULL, (empty)
                    *lits.offset(fresh5 as isize) = lit;
                    // 1581: lits.offset(fre ... size): typeof(_595) = *mut {l766} i32
                    // 1581: lits.offset(fre ... size):   l766 = UNIQUE | NON_NULL, (empty)
                    // 1581: lits: typeof(_596) = *mut {l768} i32
                    // 1581: lits:   l768 = UNIQUE | NON_NULL, (empty)
                    // 1581: lits: typeof(_597) = *mut {l770} *mut {l771} i32
                    // 1581: lits:   l770 = UNIQUE | NON_NULL, (empty)
                    // 1581: lits:   l771 = UNIQUE | NON_NULL, (empty)
                }
                let mut BYTES: size_t = ((nlits + 1 as libc::c_int) as libc::c_ulong)
                // 1583: nlits: typeof(_604) = *mut {l779} i32
                // 1583: nlits:   l779 = UNIQUE | NON_NULL, (empty)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong);
                assumption = malloc(BYTES) as *mut libc::c_int;
                // 1585: malloc(BYTES): typeof(_609) = *mut {l785} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1585: malloc(BYTES):   l785 = UNIQUE | NON_NULL, (empty)
                // 1585: assumption = ma ... c_int: typeof(_4 = move _609 as *mut i32 (Misc)) = *mut {l1252} i32
                // 1585: assumption = ma ... c_int:   l1252 = UNIQUE | NON_NULL, (empty)
                if assumption.is_null() {
                // 1586: assumption: typeof(_613) = *mut {l790} i32
                // 1586: assumption:   l790 = UNIQUE | NON_NULL, (empty)
                    die(b"out of memory\0" as *const u8 as *const libc::c_char);
                    // 1587: b"out of memory ... _char: typeof(_616) = *const {l794} i8
                    // 1587: b"out of memory ... _char:   l794 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"out of memory ... st u8: typeof(_617) = *const {l796} u8
                    // 1587: b"out of memory ... st u8:   l796 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"out of memory\0": typeof(_618) = *const {l798} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 1587: b"out of memory\0":   l798 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"out of memory\0": typeof(_619) = & {l800} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 1587: b"out of memory\0":   l800 = UNIQUE | NON_NULL, FIXED
                    // 1587: b"out of memory\0": typeof(_619 = const b"out of memory\x00") = & {l1253} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 1587: b"out of memory\0":   l1253 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"out of memory ... st u8: typeof(_617 = move _618 as *const u8 (Pointer(ArrayToPointer))) = *const {l1255} u8
                    // 1587: b"out of memory ... st u8:   l1255 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"out of memory ... _char: typeof(_616 = move _617 as *const i8 (Misc)) = *const {l1256} i8
                    // 1587: b"out of memory ... _char:   l1256 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"out of memory\0": typeof(_618 = &raw const (*_619)) = *const {l1254} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                    // 1587: b"out of memory\0":   l1254 = UNIQUE | NON_NULL, (empty)
                    exit(1 as libc::c_int);
                }
                memset(assumption as *mut libc::c_void, 0 as libc::c_int, BYTES);
                // 1590: memset(assumpti ... YTES): typeof(_622) = *mut {l804} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1590: memset(assumpti ... YTES):   l804 = UNIQUE | NON_NULL, (empty)
                // 1590: assumption as * ... _void: typeof(_623) = *mut {l806} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1590: assumption as * ... _void:   l806 = UNIQUE | NON_NULL, (empty)
                // 1590: assumption: typeof(_624) = *mut {l808} i32
                // 1590: assumption:   l808 = UNIQUE | NON_NULL, (empty)
                // 1590: assumption as * ... _void: typeof(_623 = move _624 as *mut libc::c_void (Misc)) = *mut {l1257} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 1590: assumption as * ... _void:   l1257 = UNIQUE | NON_NULL, (empty)
                allocated = allocated.wrapping_add(BYTES);
                // 1591: allocated: typeof(_629) = *mut {l814} u64
                // 1591: allocated:   l814 = UNIQUE | NON_NULL, (empty)
                // 1591: allocated: typeof(_631) = *mut {l817} u64
                // 1591: allocated:   l817 = UNIQUE | NON_NULL, (empty)
                if allocated > maxallocated {
                // 1592: allocated: typeof(_635) = *mut {l822} u64
                // 1592: allocated:   l822 = UNIQUE | NON_NULL, (empty)
                // 1592: maxallocated: typeof(_637) = *mut {l825} u64
                // 1592: maxallocated:   l825 = UNIQUE | NON_NULL, (empty)
                    maxallocated = allocated;
                    // 1593: allocated: typeof(_639) = *mut {l828} u64
                    // 1593: allocated:   l828 = UNIQUE | NON_NULL, (empty)
                    // 1593: maxallocated: typeof(_640) = *mut {l830} u64
                    // 1593: maxallocated:   l830 = UNIQUE | NON_NULL, (empty)
                }
                i = 0 as libc::c_int;
                while i < nlits {
                // 1596: nlits: typeof(_646) = *mut {l837} i32
                // 1596: nlits:   l837 = UNIQUE | NON_NULL, (empty)
                    *assumption.offset(i as isize) = *lits.offset(i as isize);
                    // 1597: lits.offset(i a ... size): typeof(_648) = *mut {l840} i32
                    // 1597: lits.offset(i a ... size):   l840 = UNIQUE | NON_NULL, (empty)
                    // 1597: lits: typeof(_649) = *mut {l842} i32
                    // 1597: lits:   l842 = UNIQUE | NON_NULL, (empty)
                    // 1597: lits: typeof(_650) = *mut {l844} *mut {l845} i32
                    // 1597: lits:   l844 = UNIQUE | NON_NULL, (empty)
                    // 1597: lits:   l845 = UNIQUE | NON_NULL, (empty)
                    // 1597: assumption.offs ... size): typeof(_653) = *mut {l849} i32
                    // 1597: assumption.offs ... size):   l849 = UNIQUE | NON_NULL, (empty)
                    // 1597: assumption: typeof(_654) = *mut {l851} i32
                    // 1597: assumption:   l851 = UNIQUE | NON_NULL, (empty)
                    i += 1;
                    i;
                }
                if nassumptions >= szassumptions {
                // 1601: nassumptions: typeof(_665) = *mut {l863} i32
                // 1601: nassumptions:   l863 = UNIQUE | NON_NULL, (empty)
                // 1601: szassumptions: typeof(_667) = *mut {l866} i32
                // 1601: szassumptions:   l866 = UNIQUE | NON_NULL, (empty)
                    let mut oldbytes_0: size_t = 0;
                    let mut newbytes_0: size_t = 0;
                    oldbytes_0 = (szassumptions as libc::c_ulong)
                    // 1604: szassumptions: typeof(_673) = *mut {l873} i32
                    // 1604: szassumptions:   l873 = UNIQUE | NON_NULL, (empty)
                        .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong);
                    allocated = allocated.wrapping_sub(oldbytes_0);
                    // 1606: allocated: typeof(_678) = *mut {l879} u64
                    // 1606: allocated:   l879 = UNIQUE | NON_NULL, (empty)
                    // 1606: allocated: typeof(_680) = *mut {l882} u64
                    // 1606: allocated:   l882 = UNIQUE | NON_NULL, (empty)
                    szassumptions = if szassumptions != 0 {
                    // 1607: szassumptions: typeof(_684) = *mut {l887} i32
                    // 1607: szassumptions:   l887 = UNIQUE | NON_NULL, (empty)
                    // 1607: szassumptions: typeof(_689) = *mut {l894} i32
                    // 1607: szassumptions:   l894 = UNIQUE | NON_NULL, (empty)
                        2 as libc::c_int * szassumptions
                        // 1608: szassumptions: typeof(_687) = *mut {l891} i32
                        // 1608: szassumptions:   l891 = UNIQUE | NON_NULL, (empty)
                    } else {
                        1 as libc::c_int
                    };
                    newbytes_0 = (szassumptions as libc::c_ulong)
                    // 1612: szassumptions: typeof(_693) = *mut {l899} i32
                    // 1612: szassumptions:   l899 = UNIQUE | NON_NULL, (empty)
                        .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong);
                    assumptions = realloc(assumptions as *mut libc::c_void, newbytes_0)
                    // 1614: realloc(assumpt ... es_0): typeof(_696) = *mut {l903} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1614: realloc(assumpt ... es_0):   l903 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions as  ... _void: typeof(_697) = *mut {l905} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1614: assumptions as  ... _void:   l905 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions: typeof(_698) = *mut {l907} *mut {l908} i32
                    // 1614: assumptions:   l907 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions:   l908 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions: typeof(_699) = *mut {l910} *mut {l911} *mut {l912} i32
                    // 1614: assumptions:   l910 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions:   l911 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions:   l912 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions: typeof(_701) = *mut {l915} *mut {l916} *mut {l917} i32
                    // 1614: assumptions:   l915 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions:   l916 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions:   l917 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions as  ... _void: typeof(_697 = move _698 as *mut libc::c_void (Misc)) = *mut {l1258} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 1614: assumptions as  ... _void:   l1258 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions = r ... c_int: typeof((*_701) = move _696 as *mut *mut i32 (Misc)) = *mut {l1259} *mut {l1260} i32
                    // 1614: assumptions = r ... c_int:   l1259 = UNIQUE | NON_NULL, (empty)
                    // 1614: assumptions = r ... c_int:   l1260 = UNIQUE | NON_NULL, (empty)
                        as *mut *mut libc::c_int;
                    if assumptions.is_null() {
                    // 1616: assumptions: typeof(_704) = *mut {l921} *mut {l922} i32
                    // 1616: assumptions:   l921 = UNIQUE | NON_NULL, (empty)
                    // 1616: assumptions:   l922 = UNIQUE | NON_NULL, (empty)
                    // 1616: assumptions: typeof(_705) = *mut {l924} *mut {l925} *mut {l926} i32
                    // 1616: assumptions:   l924 = UNIQUE | NON_NULL, (empty)
                    // 1616: assumptions:   l925 = UNIQUE | NON_NULL, (empty)
                    // 1616: assumptions:   l926 = UNIQUE | NON_NULL, (empty)
                        die(b"out of memory\0" as *const u8 as *const libc::c_char);
                        // 1617: b"out of memory ... _char: typeof(_707) = *const {l929} i8
                        // 1617: b"out of memory ... _char:   l929 = UNIQUE | NON_NULL, (empty)
                        // 1617: b"out of memory ... st u8: typeof(_708) = *const {l931} u8
                        // 1617: b"out of memory ... st u8:   l931 = UNIQUE | NON_NULL, (empty)
                        // 1617: b"out of memory\0": typeof(_709) = *const {l933} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1617: b"out of memory\0":   l933 = UNIQUE | NON_NULL, (empty)
                        // 1617: b"out of memory\0": typeof(_710) = & {l935} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1617: b"out of memory\0":   l935 = UNIQUE | NON_NULL, FIXED
                        // 1617: b"out of memory ... _char: typeof(_707 = move _708 as *const i8 (Misc)) = *const {l1264} i8
                        // 1617: b"out of memory ... _char:   l1264 = UNIQUE | NON_NULL, (empty)
                        // 1617: b"out of memory\0": typeof(_709 = &raw const (*_710)) = *const {l1262} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1617: b"out of memory\0":   l1262 = UNIQUE | NON_NULL, (empty)
                        // 1617: b"out of memory\0": typeof(_710 = const b"out of memory\x00") = & {l1261} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                        // 1617: b"out of memory\0":   l1261 = UNIQUE | NON_NULL, (empty)
                        // 1617: b"out of memory ... st u8: typeof(_708 = move _709 as *const u8 (Pointer(ArrayToPointer))) = *const {l1263} u8
                        // 1617: b"out of memory ... st u8:   l1263 = UNIQUE | NON_NULL, (empty)
                    }
                    allocated = allocated.wrapping_add(newbytes_0);
                    // 1619: allocated: typeof(_713) = *mut {l939} u64
                    // 1619: allocated:   l939 = UNIQUE | NON_NULL, (empty)
                    // 1619: allocated: typeof(_715) = *mut {l942} u64
                    // 1619: allocated:   l942 = UNIQUE | NON_NULL, (empty)
                    if allocated > maxallocated {
                    // 1620: allocated: typeof(_718) = *mut {l946} u64
                    // 1620: allocated:   l946 = UNIQUE | NON_NULL, (empty)
                    // 1620: maxallocated: typeof(_720) = *mut {l949} u64
                    // 1620: maxallocated:   l949 = UNIQUE | NON_NULL, (empty)
                        maxallocated = allocated;
                        // 1621: allocated: typeof(_722) = *mut {l952} u64
                        // 1621: allocated:   l952 = UNIQUE | NON_NULL, (empty)
                        // 1621: maxallocated: typeof(_723) = *mut {l954} u64
                        // 1621: maxallocated:   l954 = UNIQUE | NON_NULL, (empty)
                    }
                }
                let fresh6 = nassumptions;
                // 1624: nassumptions: typeof(_725) = *mut {l957} i32
                // 1624: nassumptions:   l957 = UNIQUE | NON_NULL, (empty)
                nassumptions = nassumptions + 1;
                // 1625: nassumptions: typeof(_727) = *mut {l960} i32
                // 1625: nassumptions:   l960 = UNIQUE | NON_NULL, (empty)
                // 1625: nassumptions: typeof(_729) = *mut {l963} i32
                // 1625: nassumptions:   l963 = UNIQUE | NON_NULL, (empty)
                let ref mut fresh7 = *assumptions.offset(fresh6 as isize);
                // 1626: ref mut fresh7: typeof(_730) = &mut {l965} *mut {l966} i32
                // 1626: ref mut fresh7:   l965 = UNIQUE | NON_NULL, FIXED
                // 1626: ref mut fresh7:   l966 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions.off ... size): typeof(_731) = *mut {l968} *mut {l969} i32
                // 1626: assumptions.off ... size):   l968 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions.off ... size):   l969 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions: typeof(_732) = *mut {l971} *mut {l972} i32
                // 1626: assumptions:   l971 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions:   l972 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions: typeof(_733) = *mut {l974} *mut {l975} *mut {l976} i32
                // 1626: assumptions:   l974 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions:   l975 = UNIQUE | NON_NULL, (empty)
                // 1626: assumptions:   l976 = UNIQUE | NON_NULL, (empty)
                // 1626: ref mut fresh7: typeof(_730 = &mut (*_731)) = &mut {l1265} *mut {l1266} i32
                // 1626: ref mut fresh7:   l1265 = UNIQUE | NON_NULL, (empty)
                // 1626: ref mut fresh7:   l1266 = UNIQUE | NON_NULL, (empty)
                *fresh7 = assumption;
                // 1627: assumption: typeof(_736) = *mut {l980} i32
                // 1627: assumption:   l980 = UNIQUE | NON_NULL, (empty)
                if nlits > maxassumptionsize {
                // 1628: nlits: typeof(_740) = *mut {l985} i32
                // 1628: nlits:   l985 = UNIQUE | NON_NULL, (empty)
                // 1628: maxassumptionsize: typeof(_742) = *mut {l988} i32
                // 1628: maxassumptionsize:   l988 = UNIQUE | NON_NULL, (empty)
                    maxassumptionsize = nlits;
                    // 1629: nlits: typeof(_744) = *mut {l991} i32
                    // 1629: nlits:   l991 = UNIQUE | NON_NULL, (empty)
                    // 1629: maxassumptionsize: typeof(_745) = *mut {l993} i32
                    // 1629: maxassumptionsize:   l993 = UNIQUE | NON_NULL, (empty)
                }
                nlits = 0 as libc::c_int;
                // 1631: nlits: typeof(_747) = *mut {l996} i32
                // 1631: nlits:   l996 = UNIQUE | NON_NULL, (empty)
                loop {
                    ch = next();
                    if !(ch == ' ' as i32
                        || ch == '\t' as i32
                        || ch == '\r' as i32
                        || ch == '\n' as i32)
                    {
                        break;
                    }
                }
                if ch == -(1 as libc::c_int) {
                    break '_CLAUSES;
                }
                if ch == '-' as i32
                    || *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                    // 1646: (*__ctype_b_loc ... size): typeof(_783) = *const {l1033} u16
                    // 1646: (*__ctype_b_loc ... size):   l1033 = UNIQUE | NON_NULL, (empty)
                    // 1646: (*__ctype_b_loc()): typeof(_784) = *const {l1035} u16
                    // 1646: (*__ctype_b_loc()):   l1035 = UNIQUE | NON_NULL, (empty)
                    // 1646: __ctype_b_loc(): typeof(_785) = *mut {l1037} *const {l1038} u16
                    // 1646: __ctype_b_loc():   l1037 = UNIQUE | NON_NULL, (empty)
                    // 1646: __ctype_b_loc():   l1038 = UNIQUE | NON_NULL, (empty)
                        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                        != 0
                {
                    break;
                }
                if ch != 'a' as i32 {
                    perr(
                        b"expected literal, 'a' or end-of-file\0" as *const u8
                        // 1654: b"expected lite ... _char: typeof(_796) = *const {l1050} i8
                        // 1654: b"expected lite ... _char:   l1050 = UNIQUE | NON_NULL, (empty)
                        // 1654: b"expected lite ... st u8: typeof(_797) = *const {l1052} u8
                        // 1654: b"expected lite ... st u8:   l1052 = UNIQUE | NON_NULL, (empty)
                        // 1654: b"expected lite ... le\0": typeof(_798) = *const {l1054} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                        // 1654: b"expected lite ... le\0":   l1054 = UNIQUE | NON_NULL, (empty)
                        // 1654: b"expected lite ... le\0": typeof(_799) = & {l1056} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                        // 1654: b"expected lite ... le\0":   l1056 = UNIQUE | NON_NULL, FIXED
                        // 1654: b"expected lite ... _char: typeof(_796 = move _797 as *const i8 (Misc)) = *const {l1270} i8
                        // 1654: b"expected lite ... _char:   l1270 = UNIQUE | NON_NULL, (empty)
                        // 1654: b"expected lite ... st u8: typeof(_797 = move _798 as *const u8 (Pointer(ArrayToPointer))) = *const {l1269} u8
                        // 1654: b"expected lite ... st u8:   l1269 = UNIQUE | NON_NULL, (empty)
                        // 1654: b"expected lite ... le\0": typeof(_798 = &raw const (*_799)) = *const {l1268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                        // 1654: b"expected lite ... le\0":   l1268 = UNIQUE | NON_NULL, (empty)
                        // 1654: b"expected lite ... le\0": typeof(_799 = const b"expected literal, \'a\' or end-of-file\x00") = & {l1267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                        // 1654: b"expected lite ... le\0":   l1267 = UNIQUE | NON_NULL, (empty)
                            as *const libc::c_char,
                    );
                }
            }
        }
    }
    msg(
        0 as *mut Worker,
        // 1662: 0 as *mut Worker: typeof(_801) = *mut {l1059} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1662: 0 as *mut Worker:   l1059 = UNIQUE | NON_NULL, (empty)
        // 1662: 0 as *mut Worker: typeof(_801 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1271} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1662: 0 as *mut Worker:   l1271 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"maximum variable %d in %d clauses\0" as *const u8 as *const libc::c_char,
        // 1664: b"maximum varia ... _char: typeof(_803) = *const {l1062} i8
        // 1664: b"maximum varia ... _char:   l1062 = UNIQUE | NON_NULL, (empty)
        // 1664: b"maximum varia ... st u8: typeof(_804) = *const {l1064} u8
        // 1664: b"maximum varia ... st u8:   l1064 = UNIQUE | NON_NULL, (empty)
        // 1664: b"maximum varia ... es\0": typeof(_805) = *const {l1066} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
        // 1664: b"maximum varia ... es\0":   l1066 = UNIQUE | NON_NULL, (empty)
        // 1664: b"maximum varia ... es\0": typeof(_806) = & {l1068} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
        // 1664: b"maximum varia ... es\0":   l1068 = UNIQUE | NON_NULL, FIXED
        // 1664: b"maximum varia ... st u8: typeof(_804 = move _805 as *const u8 (Pointer(ArrayToPointer))) = *const {l1274} u8
        // 1664: b"maximum varia ... st u8:   l1274 = UNIQUE | NON_NULL, (empty)
        // 1664: b"maximum varia ... es\0": typeof(_806 = const b"maximum variable %d in %d clauses\x00") = & {l1272} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
        // 1664: b"maximum varia ... es\0":   l1272 = UNIQUE | NON_NULL, (empty)
        // 1664: b"maximum varia ... es\0": typeof(_805 = &raw const (*_806)) = *const {l1273} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
        // 1664: b"maximum varia ... es\0":   l1273 = UNIQUE | NON_NULL, (empty)
        // 1664: b"maximum varia ... _char: typeof(_803 = move _804 as *const i8 (Misc)) = *const {l1275} i8
        // 1664: b"maximum varia ... _char:   l1275 = UNIQUE | NON_NULL, (empty)
        nvars,
        // 1665: nvars: typeof(_808) = *mut {l1071} i32
        // 1665: nvars:   l1071 = UNIQUE | NON_NULL, (empty)
        nclauses,
        // 1666: nclauses: typeof(_810) = *mut {l1074} i32
        // 1666: nclauses:   l1074 = UNIQUE | NON_NULL, (empty)
    );
    msg(
        0 as *mut Worker,
        // 1669: 0 as *mut Worker: typeof(_812) = *mut {l1077} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1669: 0 as *mut Worker:   l1077 = UNIQUE | NON_NULL, (empty)
        // 1669: 0 as *mut Worker: typeof(_812 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1276} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1669: 0 as *mut Worker:   l1276 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"parsed %d assumptions\0" as *const u8 as *const libc::c_char,
        // 1671: b"parsed %d ass ... _char: typeof(_814) = *const {l1080} i8
        // 1671: b"parsed %d ass ... _char:   l1080 = UNIQUE | NON_NULL, (empty)
        // 1671: b"parsed %d ass ... st u8: typeof(_815) = *const {l1082} u8
        // 1671: b"parsed %d ass ... st u8:   l1082 = UNIQUE | NON_NULL, (empty)
        // 1671: b"parsed %d ass ... ns\0": typeof(_816) = *const {l1084} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1671: b"parsed %d ass ... ns\0":   l1084 = UNIQUE | NON_NULL, (empty)
        // 1671: b"parsed %d ass ... ns\0": typeof(_817) = & {l1086} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1671: b"parsed %d ass ... ns\0":   l1086 = UNIQUE | NON_NULL, FIXED
        // 1671: b"parsed %d ass ... ns\0": typeof(_817 = const b"parsed %d assumptions\x00") = & {l1277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1671: b"parsed %d ass ... ns\0":   l1277 = UNIQUE | NON_NULL, (empty)
        // 1671: b"parsed %d ass ... _char: typeof(_814 = move _815 as *const i8 (Misc)) = *const {l1280} i8
        // 1671: b"parsed %d ass ... _char:   l1280 = UNIQUE | NON_NULL, (empty)
        // 1671: b"parsed %d ass ... st u8: typeof(_815 = move _816 as *const u8 (Pointer(ArrayToPointer))) = *const {l1279} u8
        // 1671: b"parsed %d ass ... st u8:   l1279 = UNIQUE | NON_NULL, (empty)
        // 1671: b"parsed %d ass ... ns\0": typeof(_816 = &raw const (*_817)) = *const {l1278} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1671: b"parsed %d ass ... ns\0":   l1278 = UNIQUE | NON_NULL, (empty)
        nassumptions,
        // 1672: nassumptions: typeof(_819) = *mut {l1089} i32
        // 1672: nassumptions:   l1089 = UNIQUE | NON_NULL, (empty)
    );
    nvars += 1;
    // 1674: nvars: typeof(_820) = *mut {l1091} i32
    // 1674: nvars:   l1091 = UNIQUE | NON_NULL, (empty)
    nvars;
    // 1675: nvars: typeof(_823) = *mut {l1095} i32
    // 1675: nvars:   l1095 = UNIQUE | NON_NULL, (empty)
    let mut BYTES_0: size_t = (nassumptions as libc::c_ulong)
    // 1676: nassumptions: typeof(_827) = *mut {l1100} i32
    // 1676: nassumptions:   l1100 = UNIQUE | NON_NULL, (empty)
        .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong);
    times = malloc(BYTES_0) as *mut libc::c_double;
    // 1678: malloc(BYTES_0): typeof(_830) = *mut {l1104} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1678: malloc(BYTES_0):   l1104 = UNIQUE | NON_NULL, (empty)
    // 1678: times: typeof(_832) = *mut {l1107} *mut {l1108} f64
    // 1678: times:   l1107 = UNIQUE | NON_NULL, (empty)
    // 1678: times:   l1108 = UNIQUE | NON_NULL, (empty)
    // 1678: times = malloc( ... ouble: typeof((*_832) = move _830 as *mut f64 (Misc)) = *mut {l1281} f64
    // 1678: times = malloc( ... ouble:   l1281 = UNIQUE | NON_NULL, (empty)
    if times.is_null() {
    // 1679: times: typeof(_835) = *mut {l1112} f64
    // 1679: times:   l1112 = UNIQUE | NON_NULL, (empty)
    // 1679: times: typeof(_836) = *mut {l1114} *mut {l1115} f64
    // 1679: times:   l1114 = UNIQUE | NON_NULL, (empty)
    // 1679: times:   l1115 = UNIQUE | NON_NULL, (empty)
        die(b"out of memory\0" as *const u8 as *const libc::c_char);
        // 1680: b"out of memory ... _char: typeof(_839) = *const {l1119} i8
        // 1680: b"out of memory ... _char:   l1119 = UNIQUE | NON_NULL, (empty)
        // 1680: b"out of memory ... st u8: typeof(_840) = *const {l1121} u8
        // 1680: b"out of memory ... st u8:   l1121 = UNIQUE | NON_NULL, (empty)
        // 1680: b"out of memory\0": typeof(_841) = *const {l1123} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1680: b"out of memory\0":   l1123 = UNIQUE | NON_NULL, (empty)
        // 1680: b"out of memory\0": typeof(_842) = & {l1125} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1680: b"out of memory\0":   l1125 = UNIQUE | NON_NULL, FIXED
        // 1680: b"out of memory\0": typeof(_841 = &raw const (*_842)) = *const {l1283} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1680: b"out of memory\0":   l1283 = UNIQUE | NON_NULL, (empty)
        // 1680: b"out of memory\0": typeof(_842 = const b"out of memory\x00") = & {l1282} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
        // 1680: b"out of memory\0":   l1282 = UNIQUE | NON_NULL, (empty)
        // 1680: b"out of memory ... st u8: typeof(_840 = move _841 as *const u8 (Pointer(ArrayToPointer))) = *const {l1284} u8
        // 1680: b"out of memory ... st u8:   l1284 = UNIQUE | NON_NULL, (empty)
        // 1680: b"out of memory ... _char: typeof(_839 = move _840 as *const i8 (Misc)) = *const {l1285} i8
        // 1680: b"out of memory ... _char:   l1285 = UNIQUE | NON_NULL, (empty)
        exit(1 as libc::c_int);
    }
    memset(times as *mut libc::c_void, 0 as libc::c_int, BYTES_0);
    // 1683: memset(times as ... ES_0): typeof(_845) = *mut {l1129} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1683: memset(times as ... ES_0):   l1129 = UNIQUE | NON_NULL, (empty)
    // 1683: times as *mut l ... _void: typeof(_846) = *mut {l1131} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1683: times as *mut l ... _void:   l1131 = UNIQUE | NON_NULL, (empty)
    // 1683: times: typeof(_847) = *mut {l1133} f64
    // 1683: times:   l1133 = UNIQUE | NON_NULL, (empty)
    // 1683: times: typeof(_848) = *mut {l1135} *mut {l1136} f64
    // 1683: times:   l1135 = UNIQUE | NON_NULL, (empty)
    // 1683: times:   l1136 = UNIQUE | NON_NULL, (empty)
    // 1683: times as *mut l ... _void: typeof(_846 = move _847 as *mut libc::c_void (Misc)) = *mut {l1286} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1683: times as *mut l ... _void:   l1286 = UNIQUE | NON_NULL, (empty)
    allocated = allocated.wrapping_add(BYTES_0);
    // 1684: allocated: typeof(_853) = *mut {l1142} u64
    // 1684: allocated:   l1142 = UNIQUE | NON_NULL, (empty)
    // 1684: allocated: typeof(_855) = *mut {l1145} u64
    // 1684: allocated:   l1145 = UNIQUE | NON_NULL, (empty)
    if allocated > maxallocated {
    // 1685: allocated: typeof(_859) = *mut {l1150} u64
    // 1685: allocated:   l1150 = UNIQUE | NON_NULL, (empty)
    // 1685: maxallocated: typeof(_861) = *mut {l1153} u64
    // 1685: maxallocated:   l1153 = UNIQUE | NON_NULL, (empty)
        maxallocated = allocated;
        // 1686: allocated: typeof(_863) = *mut {l1156} u64
        // 1686: allocated:   l1156 = UNIQUE | NON_NULL, (empty)
        // 1686: maxallocated: typeof(_864) = *mut {l1158} u64
        // 1686: maxallocated:   l1158 = UNIQUE | NON_NULL, (empty)
    }
    i = 0 as libc::c_int;
    while i < nassumptions {
    // 1689: nassumptions: typeof(_869) = *mut {l1164} i32
    // 1689: nassumptions:   l1164 = UNIQUE | NON_NULL, (empty)
        *times.offset(i as isize) = -(1 as libc::c_int) as libc::c_double;
        // 1690: times.offset(i  ... size): typeof(_873) = *mut {l1169} f64
        // 1690: times.offset(i  ... size):   l1169 = UNIQUE | NON_NULL, (empty)
        // 1690: times: typeof(_874) = *mut {l1171} f64
        // 1690: times:   l1171 = UNIQUE | NON_NULL, (empty)
        // 1690: times: typeof(_875) = *mut {l1173} *mut {l1174} f64
        // 1690: times:   l1173 = UNIQUE | NON_NULL, (empty)
        // 1690: times:   l1174 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
}
unsafe extern "C" fn freeze() {
    let mut idx: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    idx = 1 as libc::c_int;
    while idx < nvars {
    // 1699: nvars: typeof(_8) = *mut {l8} i32
    // 1699: nvars:   l8 = UNIQUE | NON_NULL, (empty)
        if *used.offset(idx as isize) >= 0 as libc::c_int {
        // 1700: used.offset(idx ... size): typeof(_12) = *mut {l13} i32
        // 1700: used.offset(idx ... size):   l13 = UNIQUE | NON_NULL, (empty)
        // 1700: used: typeof(_13) = *mut {l15} i32
        // 1700: used:   l15 = UNIQUE | NON_NULL, (empty)
        // 1700: used: typeof(_14) = *mut {l17} *mut {l18} i32
        // 1700: used:   l17 = UNIQUE | NON_NULL, (empty)
        // 1700: used:   l18 = UNIQUE | NON_NULL, (empty)
            i = 0 as libc::c_int;
            while i < nworkers {
            // 1702: nworkers: typeof(_22) = *mut {l27} i32
            // 1702: nworkers:   l27 = UNIQUE | NON_NULL, (empty)
                lglfreeze((*workers.offset(i as isize)).lgl, idx);
                // 1703: (*workers.offse ... ).lgl: typeof(_24) = *mut {l30} LGL
                // 1703: (*workers.offse ... ).lgl:   l30 = UNIQUE | NON_NULL, (empty)
                // 1703: workers.offset( ... size): typeof(_25) = *mut {l32} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1703: workers.offset( ... size):   l32 = UNIQUE | NON_NULL, (empty)
                // 1703: workers: typeof(_26) = *mut {l34} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1703: workers:   l34 = UNIQUE | NON_NULL, (empty)
                // 1703: workers: typeof(_27) = *mut {l36} *mut {l37} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1703: workers:   l36 = UNIQUE | NON_NULL, (empty)
                // 1703: workers:   l37 = UNIQUE | NON_NULL, (empty)
                i += 1;
                i;
            }
        }
        idx += 1;
        idx;
    }
}
unsafe extern "C" fn start() {
    let mut w: *mut Worker = 0 as *mut Worker;
    // 1713: mut w: typeof(_1) = *mut {l1} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1713: mut w:   l1 = UNIQUE | NON_NULL, (empty)
    // 1713: 0 as *mut Worker: typeof(_1 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l72} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1713: 0 as *mut Worker:   l72 = UNIQUE | NON_NULL, (empty)
    w = workers;
    // 1714: workers: typeof(_2) = *mut {l3} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1714: workers:   l3 = UNIQUE | NON_NULL, (empty)
    // 1714: workers: typeof(_3) = *mut {l5} *mut {l6} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1714: workers:   l5 = UNIQUE | NON_NULL, (empty)
    // 1714: workers:   l6 = UNIQUE | NON_NULL, (empty)
    while w < workers.offset(nworkers as isize) {
    // 1715: w: typeof(_6) = *mut {l10} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1715: w:   l10 = UNIQUE | NON_NULL, (empty)
    // 1715: workers.offset( ... size): typeof(_7) = *mut {l12} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1715: workers.offset( ... size):   l12 = UNIQUE | NON_NULL, (empty)
    // 1715: workers: typeof(_8) = *mut {l14} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1715: workers:   l14 = UNIQUE | NON_NULL, (empty)
    // 1715: workers: typeof(_9) = *mut {l16} *mut {l17} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1715: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1715: workers:   l17 = UNIQUE | NON_NULL, (empty)
    // 1715: nworkers: typeof(_12) = *mut {l21} i32
    // 1715: nworkers:   l21 = UNIQUE | NON_NULL, (empty)
        if pthread_create(
            &mut (*w).thread,
            // 1717: &mut (*w).thread: typeof(_16) = *mut {l26} u64
            // 1717: &mut (*w).thread:   l26 = UNIQUE | NON_NULL, (empty)
            // 1717: &mut (*w).thread: typeof(_17) = &mut {l28} u64
            // 1717: &mut (*w).thread:   l28 = UNIQUE | NON_NULL, (empty)
            // 1717: &mut (*w).thread: typeof(_17 = &mut ((*_1).4: u64)) = &mut {l73} u64
            // 1717: &mut (*w).thread:   l73 = UNIQUE | NON_NULL, (empty)
            // 1717: &mut (*w).thread: typeof(_16 = &raw mut (*_17)) = *mut {l74} u64
            // 1717: &mut (*w).thread:   l74 = UNIQUE | NON_NULL, (empty)
            0 as *const pthread_attr_t,
            // 1718: 0 as *const pth ... ttr_t: typeof(_18) = *const {l30} DefId(0:282 ~ ilingeling[c969]::pthread_attr_t)
            // 1718: 0 as *const pth ... ttr_t:   l30 = UNIQUE | NON_NULL, (empty)
            // 1718: 0 as *const pth ... ttr_t: typeof(_18 = const 0_usize as *const pthread_attr_t (PointerFromExposedAddress)) = *const {l75} DefId(0:282 ~ ilingeling[c969]::pthread_attr_t)
            // 1718: 0 as *const pth ... ttr_t:   l75 = UNIQUE | NON_NULL, (empty)
            Some(work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            // 1719: Some(work as un ... void): typeof(_19) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l33} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 1719: Some(work as un ... void):   l32 = UNIQUE | NON_NULL, (empty)
            // 1719: Some(work as un ... void):   l33 = UNIQUE | NON_NULL, (empty)
            // 1719: work as unsafe  ... _void: typeof(_20) = fn(*mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l36} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1719: work as unsafe  ... _void:   l35 = UNIQUE | NON_NULL, (empty)
            // 1719: work as unsafe  ... _void:   l36 = UNIQUE | NON_NULL, (empty)
            // 1719: work: typeof(_20 = work as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l76} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l77} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1719: work:   l76 = UNIQUE | NON_NULL, (empty)
            // 1719: work:   l77 = UNIQUE | NON_NULL, (empty)
            // 1719: Some(work as un ... void): typeof(_19 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>::Some(move _20)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l78} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> *mut {l79} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 1719: Some(work as un ... void):   l78 = UNIQUE | NON_NULL, (empty)
            // 1719: Some(work as un ... void):   l79 = UNIQUE | NON_NULL, (empty)
            w as *mut libc::c_void,
            // 1720: w as *mut libc: ... _void: typeof(_21) = *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1720: w as *mut libc: ... _void:   l38 = UNIQUE | NON_NULL, (empty)
            // 1720: w: typeof(_22) = *mut {l40} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1720: w:   l40 = UNIQUE | NON_NULL, (empty)
            // 1720: w as *mut libc: ... _void: typeof(_21 = move _22 as *mut libc::c_void (Misc)) = *mut {l80} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1720: w as *mut libc: ... _void:   l80 = UNIQUE | NON_NULL, (empty)
        ) != 0
        {
            die(
                b"failed to create worker thread %d\0" as *const u8 as *const libc::c_char,
                // 1724: b"failed to cre ... _char: typeof(_24) = *const {l43} i8
                // 1724: b"failed to cre ... _char:   l43 = UNIQUE | NON_NULL, (empty)
                // 1724: b"failed to cre ... st u8: typeof(_25) = *const {l45} u8
                // 1724: b"failed to cre ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
                // 1724: b"failed to cre ... %d\0": typeof(_26) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 1724: b"failed to cre ... %d\0":   l47 = UNIQUE | NON_NULL, (empty)
                // 1724: b"failed to cre ... %d\0": typeof(_27) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 1724: b"failed to cre ... %d\0":   l49 = UNIQUE | NON_NULL, FIXED
                // 1724: b"failed to cre ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l83} u8
                // 1724: b"failed to cre ... st u8:   l83 = UNIQUE | NON_NULL, (empty)
                // 1724: b"failed to cre ... %d\0": typeof(_27 = const b"failed to create worker thread %d\x00") = & {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 1724: b"failed to cre ... %d\0":   l81 = UNIQUE | NON_NULL, (empty)
                // 1724: b"failed to cre ... %d\0": typeof(_26 = &raw const (*_27)) = *const {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                // 1724: b"failed to cre ... %d\0":   l82 = UNIQUE | NON_NULL, (empty)
                // 1724: b"failed to cre ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l84} i8
                // 1724: b"failed to cre ... _char:   l84 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 1725: w: typeof(_31) = *mut {l54} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1725: w:   l54 = UNIQUE | NON_NULL, (empty)
                // 1725: workers: typeof(_32) = *const {l56} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1725: workers:   l56 = UNIQUE | NON_NULL, (empty)
                // 1725: workers: typeof(_33) = *mut {l58} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1725: workers:   l58 = UNIQUE | NON_NULL, (empty)
                // 1725: workers: typeof(_34) = *mut {l60} *mut {l61} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1725: workers:   l60 = UNIQUE | NON_NULL, (empty)
                // 1725: workers:   l61 = UNIQUE | NON_NULL, (empty)
                // 1725: workers: typeof(_32 = move _33 as *const Worker (Pointer(MutToConstPointer))) = *const {l85} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1725: workers:   l85 = UNIQUE | NON_NULL, (empty)
            );
        }
        w = w.offset(1);
        // 1728: w.offset(1): typeof(_35) = *mut {l63} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1728: w.offset(1):   l63 = UNIQUE | NON_NULL, (empty)
        // 1728: w: typeof(_36) = *mut {l65} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1728: w:   l65 = UNIQUE | NON_NULL, (empty)
        w;
        // 1729: w: typeof(_37) = *mut {l67} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1729: w:   l67 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn stop() {
    let mut w: *mut Worker = 0 as *mut Worker;
    // 1733: mut w: typeof(_1) = *mut {l1} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1733: mut w:   l1 = UNIQUE | NON_NULL, (empty)
    // 1733: 0 as *mut Worker: typeof(_1 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l117} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1733: 0 as *mut Worker:   l117 = UNIQUE | NON_NULL, (empty)
    let mut avg: libc::c_double = 0.;
    w = workers;
    // 1735: workers: typeof(_3) = *mut {l4} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1735: workers:   l4 = UNIQUE | NON_NULL, (empty)
    // 1735: workers: typeof(_4) = *mut {l6} *mut {l7} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1735: workers:   l6 = UNIQUE | NON_NULL, (empty)
    // 1735: workers:   l7 = UNIQUE | NON_NULL, (empty)
    while w < workers.offset(nworkers as isize) {
    // 1736: w: typeof(_8) = *mut {l12} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1736: w:   l12 = UNIQUE | NON_NULL, (empty)
    // 1736: workers.offset( ... size): typeof(_9) = *mut {l14} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1736: workers.offset( ... size):   l14 = UNIQUE | NON_NULL, (empty)
    // 1736: workers: typeof(_10) = *mut {l16} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1736: workers:   l16 = UNIQUE | NON_NULL, (empty)
    // 1736: workers: typeof(_11) = *mut {l18} *mut {l19} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 1736: workers:   l18 = UNIQUE | NON_NULL, (empty)
    // 1736: workers:   l19 = UNIQUE | NON_NULL, (empty)
    // 1736: nworkers: typeof(_14) = *mut {l23} i32
    // 1736: nworkers:   l23 = UNIQUE | NON_NULL, (empty)
        if pthread_join((*w).thread, 0 as *mut *mut libc::c_void) != 0 {
        // 1737: 0 as *mut *mut  ... _void: typeof(_19) = *mut {l29} *mut {l30} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1737: 0 as *mut *mut  ... _void:   l29 = UNIQUE | NON_NULL, (empty)
        // 1737: 0 as *mut *mut  ... _void:   l30 = UNIQUE | NON_NULL, (empty)
        // 1737: 0 as *mut *mut  ... _void: typeof(_19 = const 0_usize as *mut *mut libc::c_void (PointerFromExposedAddress)) = *mut {l118} *mut {l119} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1737: 0 as *mut *mut  ... _void:   l118 = UNIQUE | NON_NULL, (empty)
        // 1737: 0 as *mut *mut  ... _void:   l119 = UNIQUE | NON_NULL, (empty)
            die(
                b"failed to join worker %d\0" as *const u8 as *const libc::c_char,
                // 1739: b"failed to joi ... _char: typeof(_21) = *const {l33} i8
                // 1739: b"failed to joi ... _char:   l33 = UNIQUE | NON_NULL, (empty)
                // 1739: b"failed to joi ... st u8: typeof(_22) = *const {l35} u8
                // 1739: b"failed to joi ... st u8:   l35 = UNIQUE | NON_NULL, (empty)
                // 1739: b"failed to joi ... %d\0": typeof(_23) = *const {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1739: b"failed to joi ... %d\0":   l37 = UNIQUE | NON_NULL, (empty)
                // 1739: b"failed to joi ... %d\0": typeof(_24) = & {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1739: b"failed to joi ... %d\0":   l39 = UNIQUE | NON_NULL, FIXED
                // 1739: b"failed to joi ... %d\0": typeof(_23 = &raw const (*_24)) = *const {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1739: b"failed to joi ... %d\0":   l121 = UNIQUE | NON_NULL, (empty)
                // 1739: b"failed to joi ... st u8: typeof(_22 = move _23 as *const u8 (Pointer(ArrayToPointer))) = *const {l122} u8
                // 1739: b"failed to joi ... st u8:   l122 = UNIQUE | NON_NULL, (empty)
                // 1739: b"failed to joi ... %d\0": typeof(_24 = const b"failed to join worker %d\x00") = & {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 1739: b"failed to joi ... %d\0":   l120 = UNIQUE | NON_NULL, (empty)
                // 1739: b"failed to joi ... _char: typeof(_21 = move _22 as *const i8 (Misc)) = *const {l123} i8
                // 1739: b"failed to joi ... _char:   l123 = UNIQUE | NON_NULL, (empty)
                w.offset_from(workers) as libc::c_long as libc::c_int,
                // 1740: w: typeof(_28) = *mut {l44} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1740: w:   l44 = UNIQUE | NON_NULL, (empty)
                // 1740: workers: typeof(_29) = *const {l46} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1740: workers:   l46 = UNIQUE | NON_NULL, (empty)
                // 1740: workers: typeof(_30) = *mut {l48} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1740: workers:   l48 = UNIQUE | NON_NULL, (empty)
                // 1740: workers: typeof(_31) = *mut {l50} *mut {l51} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1740: workers:   l50 = UNIQUE | NON_NULL, (empty)
                // 1740: workers:   l51 = UNIQUE | NON_NULL, (empty)
                // 1740: workers: typeof(_29 = move _30 as *const Worker (Pointer(MutToConstPointer))) = *const {l124} DefId(0:297 ~ ilingeling[c969]::Worker)
                // 1740: workers:   l124 = UNIQUE | NON_NULL, (empty)
            );
        }
        w = w.offset(1);
        // 1743: w.offset(1): typeof(_32) = *mut {l53} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1743: w.offset(1):   l53 = UNIQUE | NON_NULL, (empty)
        // 1743: w: typeof(_33) = *mut {l55} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1743: w:   l55 = UNIQUE | NON_NULL, (empty)
        w;
        // 1744: w: typeof(_34) = *mut {l57} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1744: w:   l57 = UNIQUE | NON_NULL, (empty)
    }
    if bar != 0 {
    // 1746: bar: typeof(_41) = *mut {l65} i32
    // 1746: bar:   l65 = UNIQUE | NON_NULL, (empty)
        avg = if finished != 0 {
        // 1747: finished: typeof(_45) = *mut {l70} i32
        // 1747: finished:   l70 = UNIQUE | NON_NULL, (empty)
            sumtimes / finished as libc::c_double
            // 1748: sumtimes: typeof(_47) = *mut {l73} f64
            // 1748: sumtimes:   l73 = UNIQUE | NON_NULL, (empty)
            // 1748: finished: typeof(_50) = *mut {l77} i32
            // 1748: finished:   l77 = UNIQUE | NON_NULL, (empty)
        } else {
            0.0f64
        };
        progress(
            1000 as libc::c_int * finished / nassumptions,
            // 1753: finished: typeof(_56) = *mut {l84} i32
            // 1753: finished:   l84 = UNIQUE | NON_NULL, (empty)
            // 1753: nassumptions: typeof(_59) = *mut {l88} i32
            // 1753: nassumptions:   l88 = UNIQUE | NON_NULL, (empty)
            finished,
            // 1754: finished: typeof(_65) = *mut {l95} i32
            // 1754: finished:   l95 = UNIQUE | NON_NULL, (empty)
            nassumptions,
            // 1755: nassumptions: typeof(_67) = *mut {l98} i32
            // 1755: nassumptions:   l98 = UNIQUE | NON_NULL, (empty)
            avg,
            1 as libc::c_int,
        );
    }
    msg(
        0 as *mut Worker,
        // 1761: 0 as *mut Worker: typeof(_71) = *mut {l103} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1761: 0 as *mut Worker:   l103 = UNIQUE | NON_NULL, (empty)
        // 1761: 0 as *mut Worker: typeof(_71 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l125} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1761: 0 as *mut Worker:   l125 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"joined all %d workers\0" as *const u8 as *const libc::c_char,
        // 1763: b"joined all %d ... _char: typeof(_73) = *const {l106} i8
        // 1763: b"joined all %d ... _char:   l106 = UNIQUE | NON_NULL, (empty)
        // 1763: b"joined all %d ... st u8: typeof(_74) = *const {l108} u8
        // 1763: b"joined all %d ... st u8:   l108 = UNIQUE | NON_NULL, (empty)
        // 1763: b"joined all %d ... rs\0": typeof(_75) = *const {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1763: b"joined all %d ... rs\0":   l110 = UNIQUE | NON_NULL, (empty)
        // 1763: b"joined all %d ... rs\0": typeof(_76) = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1763: b"joined all %d ... rs\0":   l112 = UNIQUE | NON_NULL, FIXED
        // 1763: b"joined all %d ... rs\0": typeof(_75 = &raw const (*_76)) = *const {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1763: b"joined all %d ... rs\0":   l127 = UNIQUE | NON_NULL, (empty)
        // 1763: b"joined all %d ... _char: typeof(_73 = move _74 as *const i8 (Misc)) = *const {l129} i8
        // 1763: b"joined all %d ... _char:   l129 = UNIQUE | NON_NULL, (empty)
        // 1763: b"joined all %d ... st u8: typeof(_74 = move _75 as *const u8 (Pointer(ArrayToPointer))) = *const {l128} u8
        // 1763: b"joined all %d ... st u8:   l128 = UNIQUE | NON_NULL, (empty)
        // 1763: b"joined all %d ... rs\0": typeof(_76 = const b"joined all %d workers\x00") = & {l126} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1763: b"joined all %d ... rs\0":   l126 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 1764: nworkers: typeof(_78) = *mut {l115} i32
        // 1764: nworkers:   l115 = UNIQUE | NON_NULL, (empty)
    );
}
unsafe extern "C" fn statsps(
    mut file: *mut FILE,
    // 1768: mut file: typeof(_1) = *mut {g17} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1768: mut file:   g17 = UNIQUE | NON_NULL, FIXED
    mut name: *const libc::c_char,
    // 1769: mut name: typeof(_2) = *const {g18} i8
    // 1769: mut name:   g18 = UNIQUE | NON_NULL, FIXED
    mut stats_0: libc::c_longlong,
    mut time: libc::c_double,
) {
    let mut scale: *const libc::c_char = 0 as *const libc::c_char;
    // 1773: mut scale: typeof(_5) = *const {l5} i8
    // 1773: mut scale:   l5 = UNIQUE | NON_NULL, (empty)
    // 1773: 0 as *const lib ... _char: typeof(_5 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l72} i8
    // 1773: 0 as *const lib ... _char:   l72 = UNIQUE | NON_NULL, (empty)
    if stats_0 > 10000000 as libc::c_int as libc::c_longlong {
        scale = b" million\0" as *const u8 as *const libc::c_char;
        // 1775: b" million\0" a ... st u8: typeof(_11) = *const {l12} u8
        // 1775: b" million\0" a ... st u8:   l12 = UNIQUE | NON_NULL, (empty)
        // 1775: b" million\0": typeof(_12) = *const {l14} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1775: b" million\0":   l14 = UNIQUE | NON_NULL, (empty)
        // 1775: b" million\0": typeof(_13) = & {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1775: b" million\0":   l16 = UNIQUE | NON_NULL, FIXED
        // 1775: b" million\0": typeof(_13 = const b" million\x00") = & {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1775: b" million\0":   l73 = UNIQUE | NON_NULL, (empty)
        // 1775: scale = b" mill ... _char: typeof(_5 = move _11 as *const i8 (Misc)) = *const {l76} i8
        // 1775: scale = b" mill ... _char:   l76 = UNIQUE | NON_NULL, (empty)
        // 1775: b" million\0": typeof(_12 = &raw const (*_13)) = *const {l74} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 1775: b" million\0":   l74 = UNIQUE | NON_NULL, (empty)
        // 1775: b" million\0" a ... st u8: typeof(_11 = move _12 as *const u8 (Pointer(ArrayToPointer))) = *const {l75} u8
        // 1775: b" million\0" a ... st u8:   l75 = UNIQUE | NON_NULL, (empty)
        stats_0 /= 1000000 as libc::c_int as libc::c_longlong;
    } else if stats_0 > 10000 as libc::c_int as libc::c_longlong {
        scale = b" thousand\0" as *const u8 as *const libc::c_char;
        // 1778: b" thousand\0"  ... st u8: typeof(_24) = *const {l28} u8
        // 1778: b" thousand\0"  ... st u8:   l28 = UNIQUE | NON_NULL, (empty)
        // 1778: b" thousand\0": typeof(_25) = *const {l30} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1778: b" thousand\0":   l30 = UNIQUE | NON_NULL, (empty)
        // 1778: b" thousand\0": typeof(_26) = & {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1778: b" thousand\0":   l32 = UNIQUE | NON_NULL, FIXED
        // 1778: b" thousand\0"  ... st u8: typeof(_24 = move _25 as *const u8 (Pointer(ArrayToPointer))) = *const {l79} u8
        // 1778: b" thousand\0"  ... st u8:   l79 = UNIQUE | NON_NULL, (empty)
        // 1778: scale = b" thou ... _char: typeof(_5 = move _24 as *const i8 (Misc)) = *const {l80} i8
        // 1778: scale = b" thou ... _char:   l80 = UNIQUE | NON_NULL, (empty)
        // 1778: b" thousand\0": typeof(_25 = &raw const (*_26)) = *const {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1778: b" thousand\0":   l78 = UNIQUE | NON_NULL, (empty)
        // 1778: b" thousand\0": typeof(_26 = const b" thousand\x00") = & {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1778: b" thousand\0":   l77 = UNIQUE | NON_NULL, (empty)
        stats_0 /= 1000 as libc::c_int as libc::c_longlong;
    } else {
        scale = b"\0" as *const u8 as *const libc::c_char;
        // 1781: b"\0" as *const u8: typeof(_33) = *const {l40} u8
        // 1781: b"\0" as *const u8:   l40 = UNIQUE | NON_NULL, (empty)
        // 1781: b"\0": typeof(_34) = *const {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
        // 1781: b"\0":   l42 = UNIQUE | NON_NULL, (empty)
        // 1781: b"\0": typeof(_35) = & {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
        // 1781: b"\0":   l44 = UNIQUE | NON_NULL, FIXED
        // 1781: b"\0": typeof(_35 = const b"\x00") = & {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
        // 1781: b"\0":   l81 = UNIQUE | NON_NULL, (empty)
        // 1781: b"\0": typeof(_34 = &raw const (*_35)) = *const {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
        // 1781: b"\0":   l82 = UNIQUE | NON_NULL, (empty)
        // 1781: scale = b"\0" a ... _char: typeof(_5 = move _33 as *const i8 (Misc)) = *const {l84} i8
        // 1781: scale = b"\0" a ... _char:   l84 = UNIQUE | NON_NULL, (empty)
        // 1781: b"\0" as *const u8: typeof(_33 = move _34 as *const u8 (Pointer(ArrayToPointer))) = *const {l83} u8
        // 1781: b"\0" as *const u8:   l83 = UNIQUE | NON_NULL, (empty)
    }
    fprintf(
        file,
        // 1784: file: typeof(_37) = *mut {l47} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1784: file:   l47 = UNIQUE | NON_NULL, (empty)
        b"c %lld%s %s, %.1f%s per second\n\0" as *const u8 as *const libc::c_char,
        // 1785: b"c %lld%s %s,  ... _char: typeof(_38) = *const {l49} i8
        // 1785: b"c %lld%s %s,  ... _char:   l49 = UNIQUE | NON_NULL, (empty)
        // 1785: b"c %lld%s %s,  ... st u8: typeof(_39) = *const {l51} u8
        // 1785: b"c %lld%s %s,  ... st u8:   l51 = UNIQUE | NON_NULL, (empty)
        // 1785: b"c %lld%s %s,  ... \n\0": typeof(_40) = *const {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1785: b"c %lld%s %s,  ... \n\0":   l53 = UNIQUE | NON_NULL, (empty)
        // 1785: b"c %lld%s %s,  ... \n\0": typeof(_41) = & {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1785: b"c %lld%s %s,  ... \n\0":   l55 = UNIQUE | NON_NULL, FIXED
        // 1785: b"c %lld%s %s,  ... \n\0": typeof(_41 = const b"c %lld%s %s, %.1f%s per second\n\x00") = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1785: b"c %lld%s %s,  ... \n\0":   l85 = UNIQUE | NON_NULL, (empty)
        // 1785: b"c %lld%s %s,  ... \n\0": typeof(_40 = &raw const (*_41)) = *const {l86} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1785: b"c %lld%s %s,  ... \n\0":   l86 = UNIQUE | NON_NULL, (empty)
        // 1785: b"c %lld%s %s,  ... st u8: typeof(_39 = move _40 as *const u8 (Pointer(ArrayToPointer))) = *const {l87} u8
        // 1785: b"c %lld%s %s,  ... st u8:   l87 = UNIQUE | NON_NULL, (empty)
        // 1785: b"c %lld%s %s,  ... _char: typeof(_38 = move _39 as *const i8 (Misc)) = *const {l88} i8
        // 1785: b"c %lld%s %s,  ... _char:   l88 = UNIQUE | NON_NULL, (empty)
        stats_0,
        scale,
        // 1787: scale: typeof(_43) = *const {l58} i8
        // 1787: scale:   l58 = UNIQUE | NON_NULL, (empty)
        name,
        // 1788: name: typeof(_44) = *const {l60} i8
        // 1788: name:   l60 = UNIQUE | NON_NULL, (empty)
        if time > 0 as libc::c_int as libc::c_double {
            stats_0 as libc::c_double / time
        } else {
            0.0f64
        },
        scale,
        // 1794: scale: typeof(_53) = *const {l70} i8
        // 1794: scale:   l70 = UNIQUE | NON_NULL, (empty)
    );
}
unsafe extern "C" fn cmpdblptr<'h0,'h1>(
    mut p: &'h0 f64,
    // 1798: mut p: typeof(_1) = *const {g19} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1798: mut p:   g19 = READ | UNIQUE | NON_NULL, (empty)
    mut q: &'h1 f64,
    // 1799: mut q: typeof(_2) = *const {g20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1799: mut q:   g20 = READ | UNIQUE | NON_NULL, (empty)
) -> libc::c_int {
    let mut a: libc::c_double = *(p);
    // 1801: (p as *mut libc ... uble): typeof(_5) = *mut {l5} f64
    // 1801: (p as *mut libc ... uble):   l5 = READ | UNIQUE | NON_NULL, (empty)
    // 1801: p: typeof(_6) = *const {l7} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1801: p:   l7 = READ | UNIQUE | NON_NULL, (empty)
    // 1801: (p as *mut libc ... uble): typeof(_5 = move _6 as *mut f64 (Misc)) = *mut {l26} f64
    // 1801: (p as *mut libc ... uble):   l26 = READ | UNIQUE | NON_NULL, (empty)
    let mut b: libc::c_double = *(q);
    // 1802: (q as *mut libc ... uble): typeof(_8) = *mut {l10} f64
    // 1802: (q as *mut libc ... uble):   l10 = READ | UNIQUE | NON_NULL, (empty)
    // 1802: q: typeof(_9) = *const {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1802: q:   l12 = READ | UNIQUE | NON_NULL, (empty)
    // 1802: (q as *mut libc ... uble): typeof(_8 = move _9 as *mut f64 (Misc)) = *mut {l27} f64
    // 1802: (q as *mut libc ... uble):   l27 = READ | UNIQUE | NON_NULL, (empty)
    if a < b {
        return -(1 as libc::c_int);
    }
    if a > b {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe fn cmpdblptr_shim(arg0: *const libc::c_void, arg1: *const libc::c_void) -> libc::c_int {
    {
    let safe_arg0 = &*arg0;
    let safe_arg1 = &*arg1;
    let safe_result = cmpdblptr(safe_arg0,safe_arg1);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn stats() {
    let mut decs: int64_t = 0 as libc::c_int as int64_t;
    let mut confs: int64_t = 0 as libc::c_int as int64_t;
    let mut props: int64_t = 0 as libc::c_int as int64_t;
    let mut mb: libc::c_double = maxallocated as libc::c_double
    // 1815: maxallocated: typeof(_10) = *mut {l10} u64
    // 1815: maxallocated:   l10 = UNIQUE | NON_NULL, (empty)
        / ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_double;
    let mut wct: libc::c_double = getime();
    let mut prt: libc::c_double = lglprocesstime();
    let mut file: *mut FILE = if !statsfile.is_null() {
    // 1819: mut file: typeof(_18) = *mut {l19} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1819: mut file:   l19 = UNIQUE | NON_NULL, (empty)
    // 1819: statsfile: typeof(_21) = *mut {l23} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1819: statsfile:   l23 = UNIQUE | NON_NULL, (empty)
    // 1819: statsfile: typeof(_22) = *mut {l25} *mut {l26} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1819: statsfile:   l25 = UNIQUE | NON_NULL, (empty)
    // 1819: statsfile:   l26 = UNIQUE | NON_NULL, (empty)
        statsfile
        // 1820: statsfile: typeof(_23) = *mut {l28} *mut {l29} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1820: statsfile:   l28 = UNIQUE | NON_NULL, (empty)
        // 1820: statsfile:   l29 = UNIQUE | NON_NULL, (empty)
    } else {
        stdout
        // 1822: stdout: typeof(_24) = *mut {l31} *mut {l32} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1822: stdout:   l31 = UNIQUE | NON_NULL, (empty)
        // 1822: stdout:   l32 = UNIQUE | NON_NULL, (empty)
    };
    let mut sum: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    let mut min: libc::c_double = 0.;
    let mut max: libc::c_double = 0.;
    let mut avg: libc::c_double = 0.;
    let mut std: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut cloned: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nworkers {
    // 1834: nworkers: typeof(_40) = *mut {l49} i32
    // 1834: nworkers:   l49 = UNIQUE | NON_NULL, (empty)
        lglflushtimers((*workers.offset(i as isize)).lgl);
        // 1835: (*workers.offse ... ).lgl: typeof(_42) = *mut {l52} LGL
        // 1835: (*workers.offse ... ).lgl:   l52 = UNIQUE | NON_NULL, (empty)
        // 1835: workers.offset( ... size): typeof(_43) = *mut {l54} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1835: workers.offset( ... size):   l54 = UNIQUE | NON_NULL, (empty)
        // 1835: workers: typeof(_44) = *mut {l56} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1835: workers:   l56 = UNIQUE | NON_NULL, (empty)
        // 1835: workers: typeof(_45) = *mut {l58} *mut {l59} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1835: workers:   l58 = UNIQUE | NON_NULL, (empty)
        // 1835: workers:   l59 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < nworkers {
    // 1840: nworkers: typeof(_58) = *mut {l73} i32
    // 1840: nworkers:   l73 = UNIQUE | NON_NULL, (empty)
        lglflushtimers((*workers.offset(i as isize)).lgl);
        // 1841: (*workers.offse ... ).lgl: typeof(_60) = *mut {l76} LGL
        // 1841: (*workers.offse ... ).lgl:   l76 = UNIQUE | NON_NULL, (empty)
        // 1841: workers.offset( ... size): typeof(_61) = *mut {l78} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1841: workers.offset( ... size):   l78 = UNIQUE | NON_NULL, (empty)
        // 1841: workers: typeof(_62) = *mut {l80} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1841: workers:   l80 = UNIQUE | NON_NULL, (empty)
        // 1841: workers: typeof(_63) = *mut {l82} *mut {l83} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1841: workers:   l82 = UNIQUE | NON_NULL, (empty)
        // 1841: workers:   l83 = UNIQUE | NON_NULL, (empty)
        fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
        // 1842: file: typeof(_67) = *mut {l88} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1842: file:   l88 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0" as *co ... _char: typeof(_68) = *const {l90} i8
        // 1842: b"c\n\0" as *co ... _char:   l90 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0" as *co ... st u8: typeof(_69) = *const {l92} u8
        // 1842: b"c\n\0" as *co ... st u8:   l92 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0": typeof(_70) = *const {l94} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1842: b"c\n\0":   l94 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0": typeof(_71) = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1842: b"c\n\0":   l96 = UNIQUE | NON_NULL, FIXED
        // 1842: b"c\n\0" as *co ... st u8: typeof(_69 = move _70 as *const u8 (Pointer(ArrayToPointer))) = *const {l890} u8
        // 1842: b"c\n\0" as *co ... st u8:   l890 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0" as *co ... _char: typeof(_68 = move _69 as *const i8 (Misc)) = *const {l891} i8
        // 1842: b"c\n\0" as *co ... _char:   l891 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0": typeof(_70 = &raw const (*_71)) = *const {l889} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1842: b"c\n\0":   l889 = UNIQUE | NON_NULL, (empty)
        // 1842: b"c\n\0": typeof(_71 = const b"c\n\x00") = & {l888} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1842: b"c\n\0":   l888 = UNIQUE | NON_NULL, (empty)
        fprintf(
            file,
            // 1844: file: typeof(_73) = *mut {l99} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1844: file:   l99 = UNIQUE | NON_NULL, (empty)
            b"c ---------[worker %d stats]------------------\n\0" as *const u8
            // 1845: b"c ---------[w ... _char: typeof(_74) = *const {l101} i8
            // 1845: b"c ---------[w ... _char:   l101 = UNIQUE | NON_NULL, (empty)
            // 1845: b"c ---------[w ... st u8: typeof(_75) = *const {l103} u8
            // 1845: b"c ---------[w ... st u8:   l103 = UNIQUE | NON_NULL, (empty)
            // 1845: b"c ---------[w ... \n\0": typeof(_76) = *const {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
            // 1845: b"c ---------[w ... \n\0":   l105 = UNIQUE | NON_NULL, (empty)
            // 1845: b"c ---------[w ... \n\0": typeof(_77) = & {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
            // 1845: b"c ---------[w ... \n\0":   l107 = UNIQUE | NON_NULL, FIXED
            // 1845: b"c ---------[w ... \n\0": typeof(_77 = const b"c ---------[worker %d stats]------------------\n\x00") = & {l892} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
            // 1845: b"c ---------[w ... \n\0":   l892 = UNIQUE | NON_NULL, (empty)
            // 1845: b"c ---------[w ... _char: typeof(_74 = move _75 as *const i8 (Misc)) = *const {l895} i8
            // 1845: b"c ---------[w ... _char:   l895 = UNIQUE | NON_NULL, (empty)
            // 1845: b"c ---------[w ... st u8: typeof(_75 = move _76 as *const u8 (Pointer(ArrayToPointer))) = *const {l894} u8
            // 1845: b"c ---------[w ... st u8:   l894 = UNIQUE | NON_NULL, (empty)
            // 1845: b"c ---------[w ... \n\0": typeof(_76 = &raw const (*_77)) = *const {l893} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
            // 1845: b"c ---------[w ... \n\0":   l893 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            i,
        );
        fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
        // 1849: file: typeof(_80) = *mut {l111} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1849: file:   l111 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0" as *co ... _char: typeof(_81) = *const {l113} i8
        // 1849: b"c\n\0" as *co ... _char:   l113 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0" as *co ... st u8: typeof(_82) = *const {l115} u8
        // 1849: b"c\n\0" as *co ... st u8:   l115 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0": typeof(_83) = *const {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1849: b"c\n\0":   l117 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0": typeof(_84) = & {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1849: b"c\n\0":   l119 = UNIQUE | NON_NULL, FIXED
        // 1849: b"c\n\0" as *co ... st u8: typeof(_82 = move _83 as *const u8 (Pointer(ArrayToPointer))) = *const {l898} u8
        // 1849: b"c\n\0" as *co ... st u8:   l898 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0" as *co ... _char: typeof(_81 = move _82 as *const i8 (Misc)) = *const {l899} i8
        // 1849: b"c\n\0" as *co ... _char:   l899 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0": typeof(_83 = &raw const (*_84)) = *const {l897} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1849: b"c\n\0":   l897 = UNIQUE | NON_NULL, (empty)
        // 1849: b"c\n\0": typeof(_84 = const b"c\n\x00") = & {l896} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1849: b"c\n\0":   l896 = UNIQUE | NON_NULL, (empty)
        lglsetout((*workers.offset(i as isize)).lgl, file);
        // 1850: (*workers.offse ... ).lgl: typeof(_86) = *mut {l122} LGL
        // 1850: (*workers.offse ... ).lgl:   l122 = UNIQUE | NON_NULL, (empty)
        // 1850: workers.offset( ... size): typeof(_87) = *mut {l124} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1850: workers.offset( ... size):   l124 = UNIQUE | NON_NULL, (empty)
        // 1850: workers: typeof(_88) = *mut {l126} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1850: workers:   l126 = UNIQUE | NON_NULL, (empty)
        // 1850: workers: typeof(_89) = *mut {l128} *mut {l129} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1850: workers:   l128 = UNIQUE | NON_NULL, (empty)
        // 1850: workers:   l129 = UNIQUE | NON_NULL, (empty)
        // 1850: file: typeof(_92) = *mut {l133} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1850: file:   l133 = UNIQUE | NON_NULL, (empty)
        lglstats((*workers.offset(i as isize)).lgl);
        // 1851: (*workers.offse ... ).lgl: typeof(_94) = *mut {l136} LGL
        // 1851: (*workers.offse ... ).lgl:   l136 = UNIQUE | NON_NULL, (empty)
        // 1851: workers.offset( ... size): typeof(_95) = *mut {l138} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1851: workers.offset( ... size):   l138 = UNIQUE | NON_NULL, (empty)
        // 1851: workers: typeof(_96) = *mut {l140} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1851: workers:   l140 = UNIQUE | NON_NULL, (empty)
        // 1851: workers: typeof(_97) = *mut {l142} *mut {l143} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1851: workers:   l142 = UNIQUE | NON_NULL, (empty)
        // 1851: workers:   l143 = UNIQUE | NON_NULL, (empty)
        lglsetout((*workers.offset(i as isize)).lgl, stdout);
        // 1852: (*workers.offse ... ).lgl: typeof(_101) = *mut {l148} LGL
        // 1852: (*workers.offse ... ).lgl:   l148 = UNIQUE | NON_NULL, (empty)
        // 1852: workers.offset( ... size): typeof(_102) = *mut {l150} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1852: workers.offset( ... size):   l150 = UNIQUE | NON_NULL, (empty)
        // 1852: workers: typeof(_103) = *mut {l152} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1852: workers:   l152 = UNIQUE | NON_NULL, (empty)
        // 1852: workers: typeof(_104) = *mut {l154} *mut {l155} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1852: workers:   l154 = UNIQUE | NON_NULL, (empty)
        // 1852: workers:   l155 = UNIQUE | NON_NULL, (empty)
        // 1852: stdout: typeof(_107) = *mut {l159} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1852: stdout:   l159 = UNIQUE | NON_NULL, (empty)
        // 1852: stdout: typeof(_108) = *mut {l161} *mut {l162} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1852: stdout:   l161 = UNIQUE | NON_NULL, (empty)
        // 1852: stdout:   l162 = UNIQUE | NON_NULL, (empty)
        mb += lglmb((*workers.offset(i as isize)).lgl);
        // 1853: (*workers.offse ... ).lgl: typeof(_110) = *mut {l165} LGL
        // 1853: (*workers.offse ... ).lgl:   l165 = UNIQUE | NON_NULL, (empty)
        // 1853: workers.offset( ... size): typeof(_111) = *mut {l167} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1853: workers.offset( ... size):   l167 = UNIQUE | NON_NULL, (empty)
        // 1853: workers: typeof(_112) = *mut {l169} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1853: workers:   l169 = UNIQUE | NON_NULL, (empty)
        // 1853: workers: typeof(_113) = *mut {l171} *mut {l172} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1853: workers:   l171 = UNIQUE | NON_NULL, (empty)
        // 1853: workers:   l172 = UNIQUE | NON_NULL, (empty)
        decs += lglgetdecs((*workers.offset(i as isize)).lgl);
        // 1854: (*workers.offse ... ).lgl: typeof(_117) = *mut {l177} LGL
        // 1854: (*workers.offse ... ).lgl:   l177 = UNIQUE | NON_NULL, (empty)
        // 1854: workers.offset( ... size): typeof(_118) = *mut {l179} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1854: workers.offset( ... size):   l179 = UNIQUE | NON_NULL, (empty)
        // 1854: workers: typeof(_119) = *mut {l181} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1854: workers:   l181 = UNIQUE | NON_NULL, (empty)
        // 1854: workers: typeof(_120) = *mut {l183} *mut {l184} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1854: workers:   l183 = UNIQUE | NON_NULL, (empty)
        // 1854: workers:   l184 = UNIQUE | NON_NULL, (empty)
        confs += lglgetconfs((*workers.offset(i as isize)).lgl);
        // 1855: (*workers.offse ... ).lgl: typeof(_125) = *mut {l190} LGL
        // 1855: (*workers.offse ... ).lgl:   l190 = UNIQUE | NON_NULL, (empty)
        // 1855: workers.offset( ... size): typeof(_126) = *mut {l192} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1855: workers.offset( ... size):   l192 = UNIQUE | NON_NULL, (empty)
        // 1855: workers: typeof(_127) = *mut {l194} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1855: workers:   l194 = UNIQUE | NON_NULL, (empty)
        // 1855: workers: typeof(_128) = *mut {l196} *mut {l197} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1855: workers:   l196 = UNIQUE | NON_NULL, (empty)
        // 1855: workers:   l197 = UNIQUE | NON_NULL, (empty)
        props += lglgetprops((*workers.offset(i as isize)).lgl);
        // 1856: (*workers.offse ... ).lgl: typeof(_133) = *mut {l203} LGL
        // 1856: (*workers.offse ... ).lgl:   l203 = UNIQUE | NON_NULL, (empty)
        // 1856: workers.offset( ... size): typeof(_134) = *mut {l205} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1856: workers.offset( ... size):   l205 = UNIQUE | NON_NULL, (empty)
        // 1856: workers: typeof(_135) = *mut {l207} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1856: workers:   l207 = UNIQUE | NON_NULL, (empty)
        // 1856: workers: typeof(_136) = *mut {l209} *mut {l210} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1856: workers:   l209 = UNIQUE | NON_NULL, (empty)
        // 1856: workers:   l210 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_lock(&mut (*workers.offset(i as isize)).cloned.lock) != 0 {
        // 1857: &mut (*workers. ... .lock: typeof(_143) = *mut {l218} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1857: &mut (*workers. ... .lock:   l218 = UNIQUE | NON_NULL, (empty)
        // 1857: &mut (*workers. ... .lock: typeof(_144) = &mut {l220} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1857: &mut (*workers. ... .lock:   l220 = UNIQUE | NON_NULL, (empty)
        // 1857: workers.offset( ... size): typeof(_145) = *mut {l222} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1857: workers.offset( ... size):   l222 = UNIQUE | NON_NULL, (empty)
        // 1857: workers: typeof(_146) = *mut {l224} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1857: workers:   l224 = UNIQUE | NON_NULL, (empty)
        // 1857: workers: typeof(_147) = *mut {l226} *mut {l227} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1857: workers:   l226 = UNIQUE | NON_NULL, (empty)
        // 1857: workers:   l227 = UNIQUE | NON_NULL, (empty)
        // 1857: &mut (*workers. ... .lock: typeof(_144 = &mut (((*_145).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l900} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1857: &mut (*workers. ... .lock:   l900 = UNIQUE | NON_NULL, (empty)
        // 1857: &mut (*workers. ... .lock: typeof(_143 = &raw mut (*_144)) = *mut {l901} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1857: &mut (*workers. ... .lock:   l901 = UNIQUE | NON_NULL, (empty)
            warn(b"worker failed to lock 'cloned' mutex\0" as *const u8 as *const libc::c_char);
            // 1858: b"worker failed ... _char: typeof(_151) = *const {l232} i8
            // 1858: b"worker failed ... _char:   l232 = UNIQUE | NON_NULL, (empty)
            // 1858: b"worker failed ... st u8: typeof(_152) = *const {l234} u8
            // 1858: b"worker failed ... st u8:   l234 = UNIQUE | NON_NULL, (empty)
            // 1858: b"worker failed ... ex\0": typeof(_153) = *const {l236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1858: b"worker failed ... ex\0":   l236 = UNIQUE | NON_NULL, (empty)
            // 1858: b"worker failed ... ex\0": typeof(_154) = & {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1858: b"worker failed ... ex\0":   l238 = UNIQUE | NON_NULL, FIXED
            // 1858: b"worker failed ... ex\0": typeof(_154 = const b"worker failed to lock \'cloned\' mutex\x00") = & {l902} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1858: b"worker failed ... ex\0":   l902 = UNIQUE | NON_NULL, (empty)
            // 1858: b"worker failed ... _char: typeof(_151 = move _152 as *const i8 (Misc)) = *const {l905} i8
            // 1858: b"worker failed ... _char:   l905 = UNIQUE | NON_NULL, (empty)
            // 1858: b"worker failed ... st u8: typeof(_152 = move _153 as *const u8 (Pointer(ArrayToPointer))) = *const {l904} u8
            // 1858: b"worker failed ... st u8:   l904 = UNIQUE | NON_NULL, (empty)
            // 1858: b"worker failed ... ex\0": typeof(_153 = &raw const (*_154)) = *const {l903} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1858: b"worker failed ... ex\0":   l903 = UNIQUE | NON_NULL, (empty)
        }
        if !((*workers.offset(i as isize)).cloned.lgl).is_null() {
        // 1860: ((*workers.offs ... .lgl): typeof(_158) = *mut {l243} LGL
        // 1860: ((*workers.offs ... .lgl):   l243 = UNIQUE | NON_NULL, (empty)
        // 1860: workers.offset( ... size): typeof(_159) = *mut {l245} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1860: workers.offset( ... size):   l245 = UNIQUE | NON_NULL, (empty)
        // 1860: workers: typeof(_160) = *mut {l247} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1860: workers:   l247 = UNIQUE | NON_NULL, (empty)
        // 1860: workers: typeof(_161) = *mut {l249} *mut {l250} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1860: workers:   l249 = UNIQUE | NON_NULL, (empty)
        // 1860: workers:   l250 = UNIQUE | NON_NULL, (empty)
            fprintf(
                file,
                // 1862: file: typeof(_165) = *mut {l255} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                // 1862: file:   l255 = UNIQUE | NON_NULL, (empty)
                b"c ---------[cloned worker %d dstats]----------\n\0" as *const u8
                // 1863: b"c ---------[c ... _char: typeof(_166) = *const {l257} i8
                // 1863: b"c ---------[c ... _char:   l257 = UNIQUE | NON_NULL, (empty)
                // 1863: b"c ---------[c ... st u8: typeof(_167) = *const {l259} u8
                // 1863: b"c ---------[c ... st u8:   l259 = UNIQUE | NON_NULL, (empty)
                // 1863: b"c ---------[c ... \n\0": typeof(_168) = *const {l261} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1863: b"c ---------[c ... \n\0":   l261 = UNIQUE | NON_NULL, (empty)
                // 1863: b"c ---------[c ... \n\0": typeof(_169) = & {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1863: b"c ---------[c ... \n\0":   l263 = UNIQUE | NON_NULL, FIXED
                // 1863: b"c ---------[c ... \n\0": typeof(_168 = &raw const (*_169)) = *const {l907} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1863: b"c ---------[c ... \n\0":   l907 = UNIQUE | NON_NULL, (empty)
                // 1863: b"c ---------[c ... st u8: typeof(_167 = move _168 as *const u8 (Pointer(ArrayToPointer))) = *const {l908} u8
                // 1863: b"c ---------[c ... st u8:   l908 = UNIQUE | NON_NULL, (empty)
                // 1863: b"c ---------[c ... \n\0": typeof(_169 = const b"c ---------[cloned worker %d dstats]----------\n\x00") = & {l906} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                // 1863: b"c ---------[c ... \n\0":   l906 = UNIQUE | NON_NULL, (empty)
                // 1863: b"c ---------[c ... _char: typeof(_166 = move _167 as *const i8 (Misc)) = *const {l909} i8
                // 1863: b"c ---------[c ... _char:   l909 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                i,
            );
            fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
            // 1867: file: typeof(_172) = *mut {l267} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1867: file:   l267 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0" as *co ... _char: typeof(_173) = *const {l269} i8
            // 1867: b"c\n\0" as *co ... _char:   l269 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0" as *co ... st u8: typeof(_174) = *const {l271} u8
            // 1867: b"c\n\0" as *co ... st u8:   l271 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0": typeof(_175) = *const {l273} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1867: b"c\n\0":   l273 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0": typeof(_176) = & {l275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1867: b"c\n\0":   l275 = UNIQUE | NON_NULL, FIXED
            // 1867: b"c\n\0" as *co ... st u8: typeof(_174 = move _175 as *const u8 (Pointer(ArrayToPointer))) = *const {l912} u8
            // 1867: b"c\n\0" as *co ... st u8:   l912 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0": typeof(_176 = const b"c\n\x00") = & {l910} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1867: b"c\n\0":   l910 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0" as *co ... _char: typeof(_173 = move _174 as *const i8 (Misc)) = *const {l913} i8
            // 1867: b"c\n\0" as *co ... _char:   l913 = UNIQUE | NON_NULL, (empty)
            // 1867: b"c\n\0": typeof(_175 = &raw const (*_176)) = *const {l911} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 1867: b"c\n\0":   l911 = UNIQUE | NON_NULL, (empty)
            lglsetout((*workers.offset(i as isize)).cloned.lgl, file);
            // 1868: (*workers.offse ... d.lgl: typeof(_178) = *mut {l278} LGL
            // 1868: (*workers.offse ... d.lgl:   l278 = UNIQUE | NON_NULL, (empty)
            // 1868: workers.offset( ... size): typeof(_179) = *mut {l280} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1868: workers.offset( ... size):   l280 = UNIQUE | NON_NULL, (empty)
            // 1868: workers: typeof(_180) = *mut {l282} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1868: workers:   l282 = UNIQUE | NON_NULL, (empty)
            // 1868: workers: typeof(_181) = *mut {l284} *mut {l285} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1868: workers:   l284 = UNIQUE | NON_NULL, (empty)
            // 1868: workers:   l285 = UNIQUE | NON_NULL, (empty)
            // 1868: file: typeof(_184) = *mut {l289} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1868: file:   l289 = UNIQUE | NON_NULL, (empty)
            lglstats((*workers.offset(i as isize)).cloned.lgl);
            // 1869: (*workers.offse ... d.lgl: typeof(_186) = *mut {l292} LGL
            // 1869: (*workers.offse ... d.lgl:   l292 = UNIQUE | NON_NULL, (empty)
            // 1869: workers.offset( ... size): typeof(_187) = *mut {l294} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1869: workers.offset( ... size):   l294 = UNIQUE | NON_NULL, (empty)
            // 1869: workers: typeof(_188) = *mut {l296} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1869: workers:   l296 = UNIQUE | NON_NULL, (empty)
            // 1869: workers: typeof(_189) = *mut {l298} *mut {l299} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1869: workers:   l298 = UNIQUE | NON_NULL, (empty)
            // 1869: workers:   l299 = UNIQUE | NON_NULL, (empty)
            lglsetout((*workers.offset(i as isize)).cloned.lgl, stdout);
            // 1870: (*workers.offse ... d.lgl: typeof(_193) = *mut {l304} LGL
            // 1870: (*workers.offse ... d.lgl:   l304 = UNIQUE | NON_NULL, (empty)
            // 1870: workers.offset( ... size): typeof(_194) = *mut {l306} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1870: workers.offset( ... size):   l306 = UNIQUE | NON_NULL, (empty)
            // 1870: workers: typeof(_195) = *mut {l308} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1870: workers:   l308 = UNIQUE | NON_NULL, (empty)
            // 1870: workers: typeof(_196) = *mut {l310} *mut {l311} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1870: workers:   l310 = UNIQUE | NON_NULL, (empty)
            // 1870: workers:   l311 = UNIQUE | NON_NULL, (empty)
            // 1870: stdout: typeof(_199) = *mut {l315} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1870: stdout:   l315 = UNIQUE | NON_NULL, (empty)
            // 1870: stdout: typeof(_200) = *mut {l317} *mut {l318} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1870: stdout:   l317 = UNIQUE | NON_NULL, (empty)
            // 1870: stdout:   l318 = UNIQUE | NON_NULL, (empty)
            mb += lglmb((*workers.offset(i as isize)).cloned.lgl);
            // 1871: (*workers.offse ... d.lgl: typeof(_202) = *mut {l321} LGL
            // 1871: (*workers.offse ... d.lgl:   l321 = UNIQUE | NON_NULL, (empty)
            // 1871: workers.offset( ... size): typeof(_203) = *mut {l323} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1871: workers.offset( ... size):   l323 = UNIQUE | NON_NULL, (empty)
            // 1871: workers: typeof(_204) = *mut {l325} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1871: workers:   l325 = UNIQUE | NON_NULL, (empty)
            // 1871: workers: typeof(_205) = *mut {l327} *mut {l328} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1871: workers:   l327 = UNIQUE | NON_NULL, (empty)
            // 1871: workers:   l328 = UNIQUE | NON_NULL, (empty)
            decs += lglgetdecs((*workers.offset(i as isize)).cloned.lgl);
            // 1872: (*workers.offse ... d.lgl: typeof(_209) = *mut {l333} LGL
            // 1872: (*workers.offse ... d.lgl:   l333 = UNIQUE | NON_NULL, (empty)
            // 1872: workers.offset( ... size): typeof(_210) = *mut {l335} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1872: workers.offset( ... size):   l335 = UNIQUE | NON_NULL, (empty)
            // 1872: workers: typeof(_211) = *mut {l337} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1872: workers:   l337 = UNIQUE | NON_NULL, (empty)
            // 1872: workers: typeof(_212) = *mut {l339} *mut {l340} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1872: workers:   l339 = UNIQUE | NON_NULL, (empty)
            // 1872: workers:   l340 = UNIQUE | NON_NULL, (empty)
            confs += lglgetconfs((*workers.offset(i as isize)).cloned.lgl);
            // 1873: (*workers.offse ... d.lgl: typeof(_217) = *mut {l346} LGL
            // 1873: (*workers.offse ... d.lgl:   l346 = UNIQUE | NON_NULL, (empty)
            // 1873: workers.offset( ... size): typeof(_218) = *mut {l348} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1873: workers.offset( ... size):   l348 = UNIQUE | NON_NULL, (empty)
            // 1873: workers: typeof(_219) = *mut {l350} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1873: workers:   l350 = UNIQUE | NON_NULL, (empty)
            // 1873: workers: typeof(_220) = *mut {l352} *mut {l353} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1873: workers:   l352 = UNIQUE | NON_NULL, (empty)
            // 1873: workers:   l353 = UNIQUE | NON_NULL, (empty)
            props += lglgetprops((*workers.offset(i as isize)).cloned.lgl);
            // 1874: (*workers.offse ... d.lgl: typeof(_225) = *mut {l359} LGL
            // 1874: (*workers.offse ... d.lgl:   l359 = UNIQUE | NON_NULL, (empty)
            // 1874: workers.offset( ... size): typeof(_226) = *mut {l361} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1874: workers.offset( ... size):   l361 = UNIQUE | NON_NULL, (empty)
            // 1874: workers: typeof(_227) = *mut {l363} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1874: workers:   l363 = UNIQUE | NON_NULL, (empty)
            // 1874: workers: typeof(_228) = *mut {l365} *mut {l366} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1874: workers:   l365 = UNIQUE | NON_NULL, (empty)
            // 1874: workers:   l366 = UNIQUE | NON_NULL, (empty)
            mb -= lglmb((*workers.offset(i as isize)).lgl);
            // 1875: (*workers.offse ... ).lgl: typeof(_233) = *mut {l372} LGL
            // 1875: (*workers.offse ... ).lgl:   l372 = UNIQUE | NON_NULL, (empty)
            // 1875: workers.offset( ... size): typeof(_234) = *mut {l374} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1875: workers.offset( ... size):   l374 = UNIQUE | NON_NULL, (empty)
            // 1875: workers: typeof(_235) = *mut {l376} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1875: workers:   l376 = UNIQUE | NON_NULL, (empty)
            // 1875: workers: typeof(_236) = *mut {l378} *mut {l379} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1875: workers:   l378 = UNIQUE | NON_NULL, (empty)
            // 1875: workers:   l379 = UNIQUE | NON_NULL, (empty)
            decs -= lglgetdecs((*workers.offset(i as isize)).lgl);
            // 1876: (*workers.offse ... ).lgl: typeof(_240) = *mut {l384} LGL
            // 1876: (*workers.offse ... ).lgl:   l384 = UNIQUE | NON_NULL, (empty)
            // 1876: workers.offset( ... size): typeof(_241) = *mut {l386} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1876: workers.offset( ... size):   l386 = UNIQUE | NON_NULL, (empty)
            // 1876: workers: typeof(_242) = *mut {l388} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1876: workers:   l388 = UNIQUE | NON_NULL, (empty)
            // 1876: workers: typeof(_243) = *mut {l390} *mut {l391} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1876: workers:   l390 = UNIQUE | NON_NULL, (empty)
            // 1876: workers:   l391 = UNIQUE | NON_NULL, (empty)
            confs -= lglgetconfs((*workers.offset(i as isize)).lgl);
            // 1877: (*workers.offse ... ).lgl: typeof(_248) = *mut {l397} LGL
            // 1877: (*workers.offse ... ).lgl:   l397 = UNIQUE | NON_NULL, (empty)
            // 1877: workers.offset( ... size): typeof(_249) = *mut {l399} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1877: workers.offset( ... size):   l399 = UNIQUE | NON_NULL, (empty)
            // 1877: workers: typeof(_250) = *mut {l401} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1877: workers:   l401 = UNIQUE | NON_NULL, (empty)
            // 1877: workers: typeof(_251) = *mut {l403} *mut {l404} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1877: workers:   l403 = UNIQUE | NON_NULL, (empty)
            // 1877: workers:   l404 = UNIQUE | NON_NULL, (empty)
            props -= lglgetprops((*workers.offset(i as isize)).lgl);
            // 1878: (*workers.offse ... ).lgl: typeof(_256) = *mut {l410} LGL
            // 1878: (*workers.offse ... ).lgl:   l410 = UNIQUE | NON_NULL, (empty)
            // 1878: workers.offset( ... size): typeof(_257) = *mut {l412} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1878: workers.offset( ... size):   l412 = UNIQUE | NON_NULL, (empty)
            // 1878: workers: typeof(_258) = *mut {l414} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1878: workers:   l414 = UNIQUE | NON_NULL, (empty)
            // 1878: workers: typeof(_259) = *mut {l416} *mut {l417} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 1878: workers:   l416 = UNIQUE | NON_NULL, (empty)
            // 1878: workers:   l417 = UNIQUE | NON_NULL, (empty)
        }
        decs += (*workers.offset(i as isize)).cloned.decs;
        // 1880: workers.offset( ... size): typeof(_264) = *mut {l423} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1880: workers.offset( ... size):   l423 = UNIQUE | NON_NULL, (empty)
        // 1880: workers: typeof(_265) = *mut {l425} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1880: workers:   l425 = UNIQUE | NON_NULL, (empty)
        // 1880: workers: typeof(_266) = *mut {l427} *mut {l428} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1880: workers:   l427 = UNIQUE | NON_NULL, (empty)
        // 1880: workers:   l428 = UNIQUE | NON_NULL, (empty)
        confs += (*workers.offset(i as isize)).cloned.confs;
        // 1881: workers.offset( ... size): typeof(_271) = *mut {l434} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1881: workers.offset( ... size):   l434 = UNIQUE | NON_NULL, (empty)
        // 1881: workers: typeof(_272) = *mut {l436} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1881: workers:   l436 = UNIQUE | NON_NULL, (empty)
        // 1881: workers: typeof(_273) = *mut {l438} *mut {l439} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1881: workers:   l438 = UNIQUE | NON_NULL, (empty)
        // 1881: workers:   l439 = UNIQUE | NON_NULL, (empty)
        props += (*workers.offset(i as isize)).cloned.props;
        // 1882: workers.offset( ... size): typeof(_278) = *mut {l445} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1882: workers.offset( ... size):   l445 = UNIQUE | NON_NULL, (empty)
        // 1882: workers: typeof(_279) = *mut {l447} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1882: workers:   l447 = UNIQUE | NON_NULL, (empty)
        // 1882: workers: typeof(_280) = *mut {l449} *mut {l450} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1882: workers:   l449 = UNIQUE | NON_NULL, (empty)
        // 1882: workers:   l450 = UNIQUE | NON_NULL, (empty)
        cloned += (*workers.offset(i as isize)).cloned.count;
        // 1883: workers.offset( ... size): typeof(_285) = *mut {l456} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1883: workers.offset( ... size):   l456 = UNIQUE | NON_NULL, (empty)
        // 1883: workers: typeof(_286) = *mut {l458} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1883: workers:   l458 = UNIQUE | NON_NULL, (empty)
        // 1883: workers: typeof(_287) = *mut {l460} *mut {l461} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1883: workers:   l460 = UNIQUE | NON_NULL, (empty)
        // 1883: workers:   l461 = UNIQUE | NON_NULL, (empty)
        if pthread_mutex_unlock(&mut (*workers.offset(i as isize)).cloned.lock) != 0 {
        // 1884: &mut (*workers. ... .lock: typeof(_294) = *mut {l469} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1884: &mut (*workers. ... .lock:   l469 = UNIQUE | NON_NULL, (empty)
        // 1884: &mut (*workers. ... .lock: typeof(_295) = &mut {l471} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1884: &mut (*workers. ... .lock:   l471 = UNIQUE | NON_NULL, (empty)
        // 1884: workers.offset( ... size): typeof(_296) = *mut {l473} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1884: workers.offset( ... size):   l473 = UNIQUE | NON_NULL, (empty)
        // 1884: workers: typeof(_297) = *mut {l475} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1884: workers:   l475 = UNIQUE | NON_NULL, (empty)
        // 1884: workers: typeof(_298) = *mut {l477} *mut {l478} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 1884: workers:   l477 = UNIQUE | NON_NULL, (empty)
        // 1884: workers:   l478 = UNIQUE | NON_NULL, (empty)
        // 1884: &mut (*workers. ... .lock: typeof(_294 = &raw mut (*_295)) = *mut {l915} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1884: &mut (*workers. ... .lock:   l915 = UNIQUE | NON_NULL, (empty)
        // 1884: &mut (*workers. ... .lock: typeof(_295 = &mut (((*_296).1: C2RustUnnamed_1).6: pthread_mutex_t)) = &mut {l914} DefId(0:289 ~ ilingeling[c969]::pthread_mutex_t)
        // 1884: &mut (*workers. ... .lock:   l914 = UNIQUE | NON_NULL, (empty)
            warn(b"worker failed to lock 'cloned' mutex\0" as *const u8 as *const libc::c_char);
            // 1885: b"worker failed ... _char: typeof(_302) = *const {l483} i8
            // 1885: b"worker failed ... _char:   l483 = UNIQUE | NON_NULL, (empty)
            // 1885: b"worker failed ... st u8: typeof(_303) = *const {l485} u8
            // 1885: b"worker failed ... st u8:   l485 = UNIQUE | NON_NULL, (empty)
            // 1885: b"worker failed ... ex\0": typeof(_304) = *const {l487} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1885: b"worker failed ... ex\0":   l487 = UNIQUE | NON_NULL, (empty)
            // 1885: b"worker failed ... ex\0": typeof(_305) = & {l489} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1885: b"worker failed ... ex\0":   l489 = UNIQUE | NON_NULL, FIXED
            // 1885: b"worker failed ... _char: typeof(_302 = move _303 as *const i8 (Misc)) = *const {l919} i8
            // 1885: b"worker failed ... _char:   l919 = UNIQUE | NON_NULL, (empty)
            // 1885: b"worker failed ... st u8: typeof(_303 = move _304 as *const u8 (Pointer(ArrayToPointer))) = *const {l918} u8
            // 1885: b"worker failed ... st u8:   l918 = UNIQUE | NON_NULL, (empty)
            // 1885: b"worker failed ... ex\0": typeof(_305 = const b"worker failed to lock \'cloned\' mutex\x00") = & {l916} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1885: b"worker failed ... ex\0":   l916 = UNIQUE | NON_NULL, (empty)
            // 1885: b"worker failed ... ex\0": typeof(_304 = &raw const (*_305)) = *const {l917} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 1885: b"worker failed ... ex\0":   l917 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    // 1890: file: typeof(_312) = *mut {l497} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1890: file:   l497 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0" as *co ... _char: typeof(_313) = *const {l499} i8
    // 1890: b"c\n\0" as *co ... _char:   l499 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0" as *co ... st u8: typeof(_314) = *const {l501} u8
    // 1890: b"c\n\0" as *co ... st u8:   l501 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0": typeof(_315) = *const {l503} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1890: b"c\n\0":   l503 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0": typeof(_316) = & {l505} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1890: b"c\n\0":   l505 = UNIQUE | NON_NULL, FIXED
    // 1890: b"c\n\0": typeof(_315 = &raw const (*_316)) = *const {l921} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1890: b"c\n\0":   l921 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0" as *co ... st u8: typeof(_314 = move _315 as *const u8 (Pointer(ArrayToPointer))) = *const {l922} u8
    // 1890: b"c\n\0" as *co ... st u8:   l922 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0": typeof(_316 = const b"c\n\x00") = & {l920} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1890: b"c\n\0":   l920 = UNIQUE | NON_NULL, (empty)
    // 1890: b"c\n\0" as *co ... _char: typeof(_313 = move _314 as *const i8 (Misc)) = *const {l923} i8
    // 1890: b"c\n\0" as *co ... _char:   l923 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 1892: file: typeof(_318) = *mut {l508} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1892: file:   l508 = UNIQUE | NON_NULL, (empty)
        b"c ---------[global-stats]-------------------------\n\0" as *const u8
        // 1893: b"c ---------[g ... _char: typeof(_319) = *const {l510} i8
        // 1893: b"c ---------[g ... _char:   l510 = UNIQUE | NON_NULL, (empty)
        // 1893: b"c ---------[g ... st u8: typeof(_320) = *const {l512} u8
        // 1893: b"c ---------[g ... st u8:   l512 = UNIQUE | NON_NULL, (empty)
        // 1893: b"c ---------[g ... \n\0": typeof(_321) = *const {l514} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
        // 1893: b"c ---------[g ... \n\0":   l514 = UNIQUE | NON_NULL, (empty)
        // 1893: b"c ---------[g ... \n\0": typeof(_322) = & {l516} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
        // 1893: b"c ---------[g ... \n\0":   l516 = UNIQUE | NON_NULL, FIXED
        // 1893: b"c ---------[g ... _char: typeof(_319 = move _320 as *const i8 (Misc)) = *const {l927} i8
        // 1893: b"c ---------[g ... _char:   l927 = UNIQUE | NON_NULL, (empty)
        // 1893: b"c ---------[g ... st u8: typeof(_320 = move _321 as *const u8 (Pointer(ArrayToPointer))) = *const {l926} u8
        // 1893: b"c ---------[g ... st u8:   l926 = UNIQUE | NON_NULL, (empty)
        // 1893: b"c ---------[g ... \n\0": typeof(_322 = const b"c ---------[global-stats]-------------------------\n\x00") = & {l924} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
        // 1893: b"c ---------[g ... \n\0":   l924 = UNIQUE | NON_NULL, (empty)
        // 1893: b"c ---------[g ... \n\0": typeof(_321 = &raw const (*_322)) = *const {l925} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000034)) }]
        // 1893: b"c ---------[g ... \n\0":   l925 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
    );
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    // 1896: file: typeof(_324) = *mut {l519} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1896: file:   l519 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0" as *co ... _char: typeof(_325) = *const {l521} i8
    // 1896: b"c\n\0" as *co ... _char:   l521 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0" as *co ... st u8: typeof(_326) = *const {l523} u8
    // 1896: b"c\n\0" as *co ... st u8:   l523 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0": typeof(_327) = *const {l525} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1896: b"c\n\0":   l525 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0": typeof(_328) = & {l527} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1896: b"c\n\0":   l527 = UNIQUE | NON_NULL, FIXED
    // 1896: b"c\n\0" as *co ... st u8: typeof(_326 = move _327 as *const u8 (Pointer(ArrayToPointer))) = *const {l930} u8
    // 1896: b"c\n\0" as *co ... st u8:   l930 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0": typeof(_328 = const b"c\n\x00") = & {l928} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1896: b"c\n\0":   l928 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0": typeof(_327 = &raw const (*_328)) = *const {l929} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1896: b"c\n\0":   l929 = UNIQUE | NON_NULL, (empty)
    // 1896: b"c\n\0" as *co ... _char: typeof(_325 = move _326 as *const i8 (Misc)) = *const {l931} i8
    // 1896: b"c\n\0" as *co ... _char:   l931 = UNIQUE | NON_NULL, (empty)
    statsps(
        file,
        // 1898: file: typeof(_330) = *mut {l530} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1898: file:   l530 = UNIQUE | NON_NULL, (empty)
        b"scheduled jobs\0" as *const u8 as *const libc::c_char,
        // 1899: b"scheduled job ... _char: typeof(_331) = *const {l532} i8
        // 1899: b"scheduled job ... _char:   l532 = UNIQUE | NON_NULL, (empty)
        // 1899: b"scheduled job ... st u8: typeof(_332) = *const {l534} u8
        // 1899: b"scheduled job ... st u8:   l534 = UNIQUE | NON_NULL, (empty)
        // 1899: b"scheduled jobs\0": typeof(_333) = *const {l536} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 1899: b"scheduled jobs\0":   l536 = UNIQUE | NON_NULL, (empty)
        // 1899: b"scheduled jobs\0": typeof(_334) = & {l538} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 1899: b"scheduled jobs\0":   l538 = UNIQUE | NON_NULL, FIXED
        // 1899: b"scheduled job ... st u8: typeof(_332 = move _333 as *const u8 (Pointer(ArrayToPointer))) = *const {l934} u8
        // 1899: b"scheduled job ... st u8:   l934 = UNIQUE | NON_NULL, (empty)
        // 1899: b"scheduled job ... _char: typeof(_331 = move _332 as *const i8 (Misc)) = *const {l935} i8
        // 1899: b"scheduled job ... _char:   l935 = UNIQUE | NON_NULL, (empty)
        // 1899: b"scheduled jobs\0": typeof(_334 = const b"scheduled jobs\x00") = & {l932} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 1899: b"scheduled jobs\0":   l932 = UNIQUE | NON_NULL, (empty)
        // 1899: b"scheduled jobs\0": typeof(_333 = &raw const (*_334)) = *const {l933} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 1899: b"scheduled jobs\0":   l933 = UNIQUE | NON_NULL, (empty)
        queue as libc::c_longlong,
        // 1900: queue: typeof(_337) = *mut {l542} i32
        // 1900: queue:   l542 = UNIQUE | NON_NULL, (empty)
        wct,
    );
    fprintf(
        file,
        // 1904: file: typeof(_340) = *mut {l546} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1904: file:   l546 = UNIQUE | NON_NULL, (empty)
        b"c %d failed assumptions %.0f%% out of %d\n\0" as *const u8 as *const libc::c_char,
        // 1905: b"c %d failed a ... _char: typeof(_341) = *const {l548} i8
        // 1905: b"c %d failed a ... _char:   l548 = UNIQUE | NON_NULL, (empty)
        // 1905: b"c %d failed a ... st u8: typeof(_342) = *const {l550} u8
        // 1905: b"c %d failed a ... st u8:   l550 = UNIQUE | NON_NULL, (empty)
        // 1905: b"c %d failed a ... \n\0": typeof(_343) = *const {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 1905: b"c %d failed a ... \n\0":   l552 = UNIQUE | NON_NULL, (empty)
        // 1905: b"c %d failed a ... \n\0": typeof(_344) = & {l554} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 1905: b"c %d failed a ... \n\0":   l554 = UNIQUE | NON_NULL, FIXED
        // 1905: b"c %d failed a ... _char: typeof(_341 = move _342 as *const i8 (Misc)) = *const {l939} i8
        // 1905: b"c %d failed a ... _char:   l939 = UNIQUE | NON_NULL, (empty)
        // 1905: b"c %d failed a ... st u8: typeof(_342 = move _343 as *const u8 (Pointer(ArrayToPointer))) = *const {l938} u8
        // 1905: b"c %d failed a ... st u8:   l938 = UNIQUE | NON_NULL, (empty)
        // 1905: b"c %d failed a ... \n\0": typeof(_344 = const b"c %d failed assumptions %.0f%% out of %d\n\x00") = & {l936} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 1905: b"c %d failed a ... \n\0":   l936 = UNIQUE | NON_NULL, (empty)
        // 1905: b"c %d failed a ... \n\0": typeof(_343 = &raw const (*_344)) = *const {l937} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 1905: b"c %d failed a ... \n\0":   l937 = UNIQUE | NON_NULL, (empty)
        redassumptions,
        // 1906: redassumptions: typeof(_346) = *mut {l557} i32
        // 1906: redassumptions:   l557 = UNIQUE | NON_NULL, (empty)
        if sumassumptions != 0 {
        // 1907: sumassumptions: typeof(_350) = *mut {l562} i32
        // 1907: sumassumptions:   l562 = UNIQUE | NON_NULL, (empty)
            100.0f64 * redassumptions as libc::c_double / sumassumptions as libc::c_double
            // 1908: redassumptions: typeof(_354) = *mut {l567} i32
            // 1908: redassumptions:   l567 = UNIQUE | NON_NULL, (empty)
            // 1908: sumassumptions: typeof(_357) = *mut {l571} i32
            // 1908: sumassumptions:   l571 = UNIQUE | NON_NULL, (empty)
        } else {
            0 as libc::c_int as libc::c_double
        },
        sumassumptions,
        // 1912: sumassumptions: typeof(_360) = *mut {l575} i32
        // 1912: sumassumptions:   l575 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    // 1914: file: typeof(_362) = *mut {l578} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1914: file:   l578 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0" as *co ... _char: typeof(_363) = *const {l580} i8
    // 1914: b"c\n\0" as *co ... _char:   l580 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0" as *co ... st u8: typeof(_364) = *const {l582} u8
    // 1914: b"c\n\0" as *co ... st u8:   l582 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0": typeof(_365) = *const {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1914: b"c\n\0":   l584 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0": typeof(_366) = & {l586} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1914: b"c\n\0":   l586 = UNIQUE | NON_NULL, FIXED
    // 1914: b"c\n\0": typeof(_366 = const b"c\n\x00") = & {l940} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1914: b"c\n\0":   l940 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0" as *co ... _char: typeof(_363 = move _364 as *const i8 (Misc)) = *const {l943} i8
    // 1914: b"c\n\0" as *co ... _char:   l943 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0" as *co ... st u8: typeof(_364 = move _365 as *const u8 (Pointer(ArrayToPointer))) = *const {l942} u8
    // 1914: b"c\n\0" as *co ... st u8:   l942 = UNIQUE | NON_NULL, (empty)
    // 1914: b"c\n\0": typeof(_365 = &raw const (*_366)) = *const {l941} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1914: b"c\n\0":   l941 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 1916: file: typeof(_368) = *mut {l589} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1916: file:   l589 = UNIQUE | NON_NULL, (empty)
        b"c %d cloned\n\0" as *const u8 as *const libc::c_char,
        // 1917: b"c %d cloned\n ... _char: typeof(_369) = *const {l591} i8
        // 1917: b"c %d cloned\n ... _char:   l591 = UNIQUE | NON_NULL, (empty)
        // 1917: b"c %d cloned\n ... st u8: typeof(_370) = *const {l593} u8
        // 1917: b"c %d cloned\n ... st u8:   l593 = UNIQUE | NON_NULL, (empty)
        // 1917: b"c %d cloned\n\0": typeof(_371) = *const {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1917: b"c %d cloned\n\0":   l595 = UNIQUE | NON_NULL, (empty)
        // 1917: b"c %d cloned\n\0": typeof(_372) = & {l597} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1917: b"c %d cloned\n\0":   l597 = UNIQUE | NON_NULL, FIXED
        // 1917: b"c %d cloned\n\0": typeof(_372 = const b"c %d cloned\n\x00") = & {l944} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1917: b"c %d cloned\n\0":   l944 = UNIQUE | NON_NULL, (empty)
        // 1917: b"c %d cloned\n ... _char: typeof(_369 = move _370 as *const i8 (Misc)) = *const {l947} i8
        // 1917: b"c %d cloned\n ... _char:   l947 = UNIQUE | NON_NULL, (empty)
        // 1917: b"c %d cloned\n ... st u8: typeof(_370 = move _371 as *const u8 (Pointer(ArrayToPointer))) = *const {l946} u8
        // 1917: b"c %d cloned\n ... st u8:   l946 = UNIQUE | NON_NULL, (empty)
        // 1917: b"c %d cloned\n\0": typeof(_371 = &raw const (*_372)) = *const {l945} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1917: b"c %d cloned\n\0":   l945 = UNIQUE | NON_NULL, (empty)
        cloned,
    );
    fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
    // 1920: file: typeof(_375) = *mut {l601} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1920: file:   l601 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0" as *co ... _char: typeof(_376) = *const {l603} i8
    // 1920: b"c\n\0" as *co ... _char:   l603 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0" as *co ... st u8: typeof(_377) = *const {l605} u8
    // 1920: b"c\n\0" as *co ... st u8:   l605 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0": typeof(_378) = *const {l607} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1920: b"c\n\0":   l607 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0": typeof(_379) = & {l609} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1920: b"c\n\0":   l609 = UNIQUE | NON_NULL, FIXED
    // 1920: b"c\n\0" as *co ... st u8: typeof(_377 = move _378 as *const u8 (Pointer(ArrayToPointer))) = *const {l950} u8
    // 1920: b"c\n\0" as *co ... st u8:   l950 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0": typeof(_379 = const b"c\n\x00") = & {l948} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1920: b"c\n\0":   l948 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0": typeof(_378 = &raw const (*_379)) = *const {l949} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
    // 1920: b"c\n\0":   l949 = UNIQUE | NON_NULL, (empty)
    // 1920: b"c\n\0" as *co ... _char: typeof(_376 = move _377 as *const i8 (Misc)) = *const {l951} i8
    // 1920: b"c\n\0" as *co ... _char:   l951 = UNIQUE | NON_NULL, (empty)
    statsps(
        file,
        // 1922: file: typeof(_381) = *mut {l612} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1922: file:   l612 = UNIQUE | NON_NULL, (empty)
        b"conflicts\0" as *const u8 as *const libc::c_char,
        // 1923: b"conflicts\0"  ... _char: typeof(_382) = *const {l614} i8
        // 1923: b"conflicts\0"  ... _char:   l614 = UNIQUE | NON_NULL, (empty)
        // 1923: b"conflicts\0"  ... st u8: typeof(_383) = *const {l616} u8
        // 1923: b"conflicts\0"  ... st u8:   l616 = UNIQUE | NON_NULL, (empty)
        // 1923: b"conflicts\0": typeof(_384) = *const {l618} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1923: b"conflicts\0":   l618 = UNIQUE | NON_NULL, (empty)
        // 1923: b"conflicts\0": typeof(_385) = & {l620} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1923: b"conflicts\0":   l620 = UNIQUE | NON_NULL, FIXED
        // 1923: b"conflicts\0"  ... st u8: typeof(_383 = move _384 as *const u8 (Pointer(ArrayToPointer))) = *const {l954} u8
        // 1923: b"conflicts\0"  ... st u8:   l954 = UNIQUE | NON_NULL, (empty)
        // 1923: b"conflicts\0"  ... _char: typeof(_382 = move _383 as *const i8 (Misc)) = *const {l955} i8
        // 1923: b"conflicts\0"  ... _char:   l955 = UNIQUE | NON_NULL, (empty)
        // 1923: b"conflicts\0": typeof(_384 = &raw const (*_385)) = *const {l953} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1923: b"conflicts\0":   l953 = UNIQUE | NON_NULL, (empty)
        // 1923: b"conflicts\0": typeof(_385 = const b"conflicts\x00") = & {l952} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1923: b"conflicts\0":   l952 = UNIQUE | NON_NULL, (empty)
        confs as libc::c_longlong,
        wct,
    );
    statsps(
        file,
        // 1928: file: typeof(_389) = *mut {l625} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1928: file:   l625 = UNIQUE | NON_NULL, (empty)
        b"decisions\0" as *const u8 as *const libc::c_char,
        // 1929: b"decisions\0"  ... _char: typeof(_390) = *const {l627} i8
        // 1929: b"decisions\0"  ... _char:   l627 = UNIQUE | NON_NULL, (empty)
        // 1929: b"decisions\0"  ... st u8: typeof(_391) = *const {l629} u8
        // 1929: b"decisions\0"  ... st u8:   l629 = UNIQUE | NON_NULL, (empty)
        // 1929: b"decisions\0": typeof(_392) = *const {l631} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1929: b"decisions\0":   l631 = UNIQUE | NON_NULL, (empty)
        // 1929: b"decisions\0": typeof(_393) = & {l633} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1929: b"decisions\0":   l633 = UNIQUE | NON_NULL, FIXED
        // 1929: b"decisions\0"  ... _char: typeof(_390 = move _391 as *const i8 (Misc)) = *const {l959} i8
        // 1929: b"decisions\0"  ... _char:   l959 = UNIQUE | NON_NULL, (empty)
        // 1929: b"decisions\0": typeof(_393 = const b"decisions\x00") = & {l956} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1929: b"decisions\0":   l956 = UNIQUE | NON_NULL, (empty)
        // 1929: b"decisions\0": typeof(_392 = &raw const (*_393)) = *const {l957} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 1929: b"decisions\0":   l957 = UNIQUE | NON_NULL, (empty)
        // 1929: b"decisions\0"  ... st u8: typeof(_391 = move _392 as *const u8 (Pointer(ArrayToPointer))) = *const {l958} u8
        // 1929: b"decisions\0"  ... st u8:   l958 = UNIQUE | NON_NULL, (empty)
        decs as libc::c_longlong,
        wct,
    );
    statsps(
        file,
        // 1934: file: typeof(_397) = *mut {l638} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1934: file:   l638 = UNIQUE | NON_NULL, (empty)
        b"propagations\0" as *const u8 as *const libc::c_char,
        // 1935: b"propagations\ ... _char: typeof(_398) = *const {l640} i8
        // 1935: b"propagations\ ... _char:   l640 = UNIQUE | NON_NULL, (empty)
        // 1935: b"propagations\ ... st u8: typeof(_399) = *const {l642} u8
        // 1935: b"propagations\ ... st u8:   l642 = UNIQUE | NON_NULL, (empty)
        // 1935: b"propagations\0": typeof(_400) = *const {l644} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1935: b"propagations\0":   l644 = UNIQUE | NON_NULL, (empty)
        // 1935: b"propagations\0": typeof(_401) = & {l646} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1935: b"propagations\0":   l646 = UNIQUE | NON_NULL, FIXED
        // 1935: b"propagations\ ... _char: typeof(_398 = move _399 as *const i8 (Misc)) = *const {l963} i8
        // 1935: b"propagations\ ... _char:   l963 = UNIQUE | NON_NULL, (empty)
        // 1935: b"propagations\0": typeof(_401 = const b"propagations\x00") = & {l960} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1935: b"propagations\0":   l960 = UNIQUE | NON_NULL, (empty)
        // 1935: b"propagations\ ... st u8: typeof(_399 = move _400 as *const u8 (Pointer(ArrayToPointer))) = *const {l962} u8
        // 1935: b"propagations\ ... st u8:   l962 = UNIQUE | NON_NULL, (empty)
        // 1935: b"propagations\0": typeof(_400 = &raw const (*_401)) = *const {l961} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1935: b"propagations\0":   l961 = UNIQUE | NON_NULL, (empty)
        props as libc::c_longlong,
        wct,
    );
    fprintf(
        file,
        // 1940: file: typeof(_405) = *mut {l651} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1940: file:   l651 = UNIQUE | NON_NULL, (empty)
        b"c wall clock time %.1f seconds\n\0" as *const u8 as *const libc::c_char,
        // 1941: b"c wall clock  ... _char: typeof(_406) = *const {l653} i8
        // 1941: b"c wall clock  ... _char:   l653 = UNIQUE | NON_NULL, (empty)
        // 1941: b"c wall clock  ... st u8: typeof(_407) = *const {l655} u8
        // 1941: b"c wall clock  ... st u8:   l655 = UNIQUE | NON_NULL, (empty)
        // 1941: b"c wall clock  ... \n\0": typeof(_408) = *const {l657} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1941: b"c wall clock  ... \n\0":   l657 = UNIQUE | NON_NULL, (empty)
        // 1941: b"c wall clock  ... \n\0": typeof(_409) = & {l659} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1941: b"c wall clock  ... \n\0":   l659 = UNIQUE | NON_NULL, FIXED
        // 1941: b"c wall clock  ... _char: typeof(_406 = move _407 as *const i8 (Misc)) = *const {l967} i8
        // 1941: b"c wall clock  ... _char:   l967 = UNIQUE | NON_NULL, (empty)
        // 1941: b"c wall clock  ... st u8: typeof(_407 = move _408 as *const u8 (Pointer(ArrayToPointer))) = *const {l966} u8
        // 1941: b"c wall clock  ... st u8:   l966 = UNIQUE | NON_NULL, (empty)
        // 1941: b"c wall clock  ... \n\0": typeof(_408 = &raw const (*_409)) = *const {l965} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1941: b"c wall clock  ... \n\0":   l965 = UNIQUE | NON_NULL, (empty)
        // 1941: b"c wall clock  ... \n\0": typeof(_409 = const b"c wall clock time %.1f seconds\n\x00") = & {l964} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
        // 1941: b"c wall clock  ... \n\0":   l964 = UNIQUE | NON_NULL, (empty)
        wct,
    );
    fprintf(
        file,
        // 1945: file: typeof(_412) = *mut {l663} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1945: file:   l663 = UNIQUE | NON_NULL, (empty)
        b"c process time %.1f seconds\n\0" as *const u8 as *const libc::c_char,
        // 1946: b"c process tim ... _char: typeof(_413) = *const {l665} i8
        // 1946: b"c process tim ... _char:   l665 = UNIQUE | NON_NULL, (empty)
        // 1946: b"c process tim ... st u8: typeof(_414) = *const {l667} u8
        // 1946: b"c process tim ... st u8:   l667 = UNIQUE | NON_NULL, (empty)
        // 1946: b"c process tim ... \n\0": typeof(_415) = *const {l669} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 1946: b"c process tim ... \n\0":   l669 = UNIQUE | NON_NULL, (empty)
        // 1946: b"c process tim ... \n\0": typeof(_416) = & {l671} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 1946: b"c process tim ... \n\0":   l671 = UNIQUE | NON_NULL, FIXED
        // 1946: b"c process tim ... \n\0": typeof(_415 = &raw const (*_416)) = *const {l969} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 1946: b"c process tim ... \n\0":   l969 = UNIQUE | NON_NULL, (empty)
        // 1946: b"c process tim ... st u8: typeof(_414 = move _415 as *const u8 (Pointer(ArrayToPointer))) = *const {l970} u8
        // 1946: b"c process tim ... st u8:   l970 = UNIQUE | NON_NULL, (empty)
        // 1946: b"c process tim ... _char: typeof(_413 = move _414 as *const i8 (Misc)) = *const {l971} i8
        // 1946: b"c process tim ... _char:   l971 = UNIQUE | NON_NULL, (empty)
        // 1946: b"c process tim ... \n\0": typeof(_416 = const b"c process time %.1f seconds\n\x00") = & {l968} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 1946: b"c process tim ... \n\0":   l968 = UNIQUE | NON_NULL, (empty)
        prt,
    );
    fprintf(
        file,
        // 1950: file: typeof(_419) = *mut {l675} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1950: file:   l675 = UNIQUE | NON_NULL, (empty)
        b"c utilization %.0f%%\n\0" as *const u8 as *const libc::c_char,
        // 1951: b"c utilization ... _char: typeof(_420) = *const {l677} i8
        // 1951: b"c utilization ... _char:   l677 = UNIQUE | NON_NULL, (empty)
        // 1951: b"c utilization ... st u8: typeof(_421) = *const {l679} u8
        // 1951: b"c utilization ... st u8:   l679 = UNIQUE | NON_NULL, (empty)
        // 1951: b"c utilization ... \n\0": typeof(_422) = *const {l681} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1951: b"c utilization ... \n\0":   l681 = UNIQUE | NON_NULL, (empty)
        // 1951: b"c utilization ... \n\0": typeof(_423) = & {l683} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1951: b"c utilization ... \n\0":   l683 = UNIQUE | NON_NULL, FIXED
        // 1951: b"c utilization ... st u8: typeof(_421 = move _422 as *const u8 (Pointer(ArrayToPointer))) = *const {l974} u8
        // 1951: b"c utilization ... st u8:   l974 = UNIQUE | NON_NULL, (empty)
        // 1951: b"c utilization ... \n\0": typeof(_422 = &raw const (*_423)) = *const {l973} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1951: b"c utilization ... \n\0":   l973 = UNIQUE | NON_NULL, (empty)
        // 1951: b"c utilization ... _char: typeof(_420 = move _421 as *const i8 (Misc)) = *const {l975} i8
        // 1951: b"c utilization ... _char:   l975 = UNIQUE | NON_NULL, (empty)
        // 1951: b"c utilization ... \n\0": typeof(_423 = const b"c utilization %.0f%%\n\x00") = & {l972} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
        // 1951: b"c utilization ... \n\0":   l972 = UNIQUE | NON_NULL, (empty)
        if wct > 0 as libc::c_int as libc::c_double {
            100.0f64 * (prt / wct / nworkers as libc::c_double)
            // 1953: nworkers: typeof(_435) = *mut {l696} i32
            // 1953: nworkers:   l696 = UNIQUE | NON_NULL, (empty)
        } else {
            0.0f64
        },
    );
    fflush(file);
    // 1958: file: typeof(_437) = *mut {l699} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 1958: file:   l699 = UNIQUE | NON_NULL, (empty)
    n = 0 as libc::c_int;
    sum = 0 as libc::c_int as libc::c_double;
    min = -(1 as libc::c_int) as libc::c_double;
    max = min;
    i = 0 as libc::c_int;
    while i < nassumptions {
    // 1964: nassumptions: typeof(_449) = *mut {l712} i32
    // 1964: nassumptions:   l712 = UNIQUE | NON_NULL, (empty)
        t = *times.offset(i as isize);
        // 1965: times.offset(i  ... size): typeof(_451) = *mut {l715} f64
        // 1965: times.offset(i  ... size):   l715 = UNIQUE | NON_NULL, (empty)
        // 1965: times: typeof(_452) = *mut {l717} f64
        // 1965: times:   l717 = UNIQUE | NON_NULL, (empty)
        // 1965: times: typeof(_453) = *mut {l719} *mut {l720} f64
        // 1965: times:   l719 = UNIQUE | NON_NULL, (empty)
        // 1965: times:   l720 = UNIQUE | NON_NULL, (empty)
        if !(t < 0 as libc::c_int as libc::c_double) {
            sum += t;
            let fresh8 = n;
            n = n + 1;
            *times.offset(fresh8 as isize) = t;
            // 1970: times.offset(fr ... size): typeof(_467) = *mut {l735} f64
            // 1970: times.offset(fr ... size):   l735 = UNIQUE | NON_NULL, (empty)
            // 1970: times: typeof(_468) = *mut {l737} f64
            // 1970: times:   l737 = UNIQUE | NON_NULL, (empty)
            // 1970: times: typeof(_469) = *mut {l739} *mut {l740} f64
            // 1970: times:   l739 = UNIQUE | NON_NULL, (empty)
            // 1970: times:   l740 = UNIQUE | NON_NULL, (empty)
            if min < 0 as libc::c_int as libc::c_double || t < min {
                min = t;
            }
            if max < 0 as libc::c_int as libc::c_double || t > max {
                max = t;
            }
        }
        i += 1;
        i;
    }
    if n != 0 {
        fprintf(file, b"c\n\0" as *const u8 as *const libc::c_char);
        // 1982: file: typeof(_500) = *mut {l772} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 1982: file:   l772 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0" as *co ... _char: typeof(_501) = *const {l774} i8
        // 1982: b"c\n\0" as *co ... _char:   l774 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0" as *co ... st u8: typeof(_502) = *const {l776} u8
        // 1982: b"c\n\0" as *co ... st u8:   l776 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0": typeof(_503) = *const {l778} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1982: b"c\n\0":   l778 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0": typeof(_504) = & {l780} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1982: b"c\n\0":   l780 = UNIQUE | NON_NULL, FIXED
        // 1982: b"c\n\0": typeof(_504 = const b"c\n\x00") = & {l976} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1982: b"c\n\0":   l976 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0" as *co ... _char: typeof(_501 = move _502 as *const i8 (Misc)) = *const {l979} i8
        // 1982: b"c\n\0" as *co ... _char:   l979 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0": typeof(_503 = &raw const (*_504)) = *const {l977} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 1982: b"c\n\0":   l977 = UNIQUE | NON_NULL, (empty)
        // 1982: b"c\n\0" as *co ... st u8: typeof(_502 = move _503 as *const u8 (Pointer(ArrayToPointer))) = *const {l978} u8
        // 1982: b"c\n\0" as *co ... st u8:   l978 = UNIQUE | NON_NULL, (empty)
        avg = sum / n as libc::c_double;
        fprintf(
            file,
            // 1985: file: typeof(_509) = *mut {l786} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1985: file:   l786 = UNIQUE | NON_NULL, (empty)
            b"c %d finished jobs in average time %.3f\n\0" as *const u8 as *const libc::c_char,
            // 1986: b"c %d finished ... _char: typeof(_510) = *const {l788} i8
            // 1986: b"c %d finished ... _char:   l788 = UNIQUE | NON_NULL, (empty)
            // 1986: b"c %d finished ... st u8: typeof(_511) = *const {l790} u8
            // 1986: b"c %d finished ... st u8:   l790 = UNIQUE | NON_NULL, (empty)
            // 1986: b"c %d finished ... \n\0": typeof(_512) = *const {l792} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
            // 1986: b"c %d finished ... \n\0":   l792 = UNIQUE | NON_NULL, (empty)
            // 1986: b"c %d finished ... \n\0": typeof(_513) = & {l794} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
            // 1986: b"c %d finished ... \n\0":   l794 = UNIQUE | NON_NULL, FIXED
            // 1986: b"c %d finished ... st u8: typeof(_511 = move _512 as *const u8 (Pointer(ArrayToPointer))) = *const {l982} u8
            // 1986: b"c %d finished ... st u8:   l982 = UNIQUE | NON_NULL, (empty)
            // 1986: b"c %d finished ... \n\0": typeof(_512 = &raw const (*_513)) = *const {l981} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
            // 1986: b"c %d finished ... \n\0":   l981 = UNIQUE | NON_NULL, (empty)
            // 1986: b"c %d finished ... \n\0": typeof(_513 = const b"c %d finished jobs in average time %.3f\n\x00") = & {l980} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
            // 1986: b"c %d finished ... \n\0":   l980 = UNIQUE | NON_NULL, (empty)
            // 1986: b"c %d finished ... _char: typeof(_510 = move _511 as *const i8 (Misc)) = *const {l983} i8
            // 1986: b"c %d finished ... _char:   l983 = UNIQUE | NON_NULL, (empty)
            n,
            avg,
        );
        fprintf(
            file,
            // 1991: file: typeof(_517) = *mut {l799} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 1991: file:   l799 = UNIQUE | NON_NULL, (empty)
            b"c time: sum %.3f, min %.3f, max %.3f\n\0" as *const u8 as *const libc::c_char,
            // 1992: b"c time: sum % ... _char: typeof(_518) = *const {l801} i8
            // 1992: b"c time: sum % ... _char:   l801 = UNIQUE | NON_NULL, (empty)
            // 1992: b"c time: sum % ... st u8: typeof(_519) = *const {l803} u8
            // 1992: b"c time: sum % ... st u8:   l803 = UNIQUE | NON_NULL, (empty)
            // 1992: b"c time: sum % ... \n\0": typeof(_520) = *const {l805} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 1992: b"c time: sum % ... \n\0":   l805 = UNIQUE | NON_NULL, (empty)
            // 1992: b"c time: sum % ... \n\0": typeof(_521) = & {l807} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 1992: b"c time: sum % ... \n\0":   l807 = UNIQUE | NON_NULL, FIXED
            // 1992: b"c time: sum % ... st u8: typeof(_519 = move _520 as *const u8 (Pointer(ArrayToPointer))) = *const {l986} u8
            // 1992: b"c time: sum % ... st u8:   l986 = UNIQUE | NON_NULL, (empty)
            // 1992: b"c time: sum % ... \n\0": typeof(_521 = const b"c time: sum %.3f, min %.3f, max %.3f\n\x00") = & {l984} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 1992: b"c time: sum % ... \n\0":   l984 = UNIQUE | NON_NULL, (empty)
            // 1992: b"c time: sum % ... \n\0": typeof(_520 = &raw const (*_521)) = *const {l985} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
            // 1992: b"c time: sum % ... \n\0":   l985 = UNIQUE | NON_NULL, (empty)
            // 1992: b"c time: sum % ... _char: typeof(_518 = move _519 as *const i8 (Misc)) = *const {l987} i8
            // 1992: b"c time: sum % ... _char:   l987 = UNIQUE | NON_NULL, (empty)
            sum,
            min,
            max,
        );
        std = 0 as libc::c_int as libc::c_double;
        i = 0 as libc::c_int;
        while i < n {
            t = *times.offset(i as isize) - avg;
            // 2000: times.offset(i  ... size): typeof(_532) = *mut {l819} f64
            // 2000: times.offset(i  ... size):   l819 = UNIQUE | NON_NULL, (empty)
            // 2000: times: typeof(_533) = *mut {l821} f64
            // 2000: times:   l821 = UNIQUE | NON_NULL, (empty)
            // 2000: times: typeof(_534) = *mut {l823} *mut {l824} f64
            // 2000: times:   l823 = UNIQUE | NON_NULL, (empty)
            // 2000: times:   l824 = UNIQUE | NON_NULL, (empty)
            std += t * t;
            i += 1;
            i;
        }
        std = sqrt(std);
        qsort(
            times as *mut libc::c_void,
            // 2007: times as *mut l ... _void: typeof(_549) = *mut {l840} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2007: times as *mut l ... _void:   l840 = UNIQUE | NON_NULL, (empty)
            // 2007: times: typeof(_550) = *mut {l842} f64
            // 2007: times:   l842 = UNIQUE | NON_NULL, (empty)
            // 2007: times: typeof(_551) = *mut {l844} *mut {l845} f64
            // 2007: times:   l844 = UNIQUE | NON_NULL, (empty)
            // 2007: times:   l845 = UNIQUE | NON_NULL, (empty)
            // 2007: times as *mut l ... _void: typeof(_549 = move _550 as *mut libc::c_void (Misc)) = *mut {l988} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 2007: times as *mut l ... _void:   l988 = UNIQUE | NON_NULL, (empty)
            n as size_t,
            ::core::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
            Some(
            // 2010: Some( cmpdblptr ... nt, ): typeof(_556) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*const {l851} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l852} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
            // 2010: Some( cmpdblptr ... nt, ):   l851 = UNIQUE | NON_NULL, (empty)
            // 2010: Some( cmpdblptr ... nt, ):   l852 = UNIQUE | NON_NULL, (empty)
            // 2010: Some( cmpdblptr ... nt, ): typeof(_556 = std::option::Option::<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32>::Some(move _557)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*const {l991} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l992} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32>
            // 2010: Some( cmpdblptr ... nt, ):   l991 = UNIQUE | NON_NULL, (empty)
            // 2010: Some( cmpdblptr ... nt, ):   l992 = UNIQUE | NON_NULL, (empty)
                cmpdblptr_shim
                // 2011: cmpdblptr as un ... c_int: typeof(_557) = fn(*const {l854} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l855} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
                // 2011: cmpdblptr as un ... c_int:   l854 = UNIQUE | NON_NULL, (empty)
                // 2011: cmpdblptr as un ... c_int:   l855 = UNIQUE | NON_NULL, (empty)
                // 2011: cmpdblptr: typeof(_557 = cmpdblptr as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32 (Pointer(ReifyFnPointer))) = fn(*const {l989} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {l990} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> i32
                // 2011: cmpdblptr:   l989 = UNIQUE | NON_NULL, (empty)
                // 2011: cmpdblptr:   l990 = UNIQUE | NON_NULL, (empty)
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        fprintf(
            file,
            // 2019: file: typeof(_559) = *mut {l858} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 2019: file:   l858 = UNIQUE | NON_NULL, (empty)
            b"c time: median %.3f, std dve %.3f\n\0" as *const u8 as *const libc::c_char,
            // 2020: b"c time: media ... _char: typeof(_560) = *const {l860} i8
            // 2020: b"c time: media ... _char:   l860 = UNIQUE | NON_NULL, (empty)
            // 2020: b"c time: media ... st u8: typeof(_561) = *const {l862} u8
            // 2020: b"c time: media ... st u8:   l862 = UNIQUE | NON_NULL, (empty)
            // 2020: b"c time: media ... \n\0": typeof(_562) = *const {l864} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 2020: b"c time: media ... \n\0":   l864 = UNIQUE | NON_NULL, (empty)
            // 2020: b"c time: media ... \n\0": typeof(_563) = & {l866} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 2020: b"c time: media ... \n\0":   l866 = UNIQUE | NON_NULL, FIXED
            // 2020: b"c time: media ... _char: typeof(_560 = move _561 as *const i8 (Misc)) = *const {l996} i8
            // 2020: b"c time: media ... _char:   l996 = UNIQUE | NON_NULL, (empty)
            // 2020: b"c time: media ... st u8: typeof(_561 = move _562 as *const u8 (Pointer(ArrayToPointer))) = *const {l995} u8
            // 2020: b"c time: media ... st u8:   l995 = UNIQUE | NON_NULL, (empty)
            // 2020: b"c time: media ... \n\0": typeof(_562 = &raw const (*_563)) = *const {l994} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 2020: b"c time: media ... \n\0":   l994 = UNIQUE | NON_NULL, (empty)
            // 2020: b"c time: media ... \n\0": typeof(_563 = const b"c time: median %.3f, std dve %.3f\n\x00") = & {l993} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
            // 2020: b"c time: media ... \n\0":   l993 = UNIQUE | NON_NULL, (empty)
            *times.offset((n / 2 as libc::c_int) as isize),
            // 2021: times.offset((n ... size): typeof(_565) = *mut {l869} f64
            // 2021: times.offset((n ... size):   l869 = UNIQUE | NON_NULL, (empty)
            // 2021: times: typeof(_566) = *mut {l871} f64
            // 2021: times:   l871 = UNIQUE | NON_NULL, (empty)
            // 2021: times: typeof(_567) = *mut {l873} *mut {l874} f64
            // 2021: times:   l873 = UNIQUE | NON_NULL, (empty)
            // 2021: times:   l874 = UNIQUE | NON_NULL, (empty)
            std,
        );
    }
    fflush(file);
    // 2025: file: typeof(_578) = *mut {l886} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2025: file:   l886 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn hist() {
    let mut file: *mut FILE = if !histfile.is_null() {
    // 2028: mut file: typeof(_1) = *mut {l1} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2028: mut file:   l1 = UNIQUE | NON_NULL, (empty)
    // 2028: histfile: typeof(_4) = *mut {l5} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2028: histfile:   l5 = UNIQUE | NON_NULL, (empty)
    // 2028: histfile: typeof(_5) = *mut {l7} *mut {l8} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2028: histfile:   l7 = UNIQUE | NON_NULL, (empty)
    // 2028: histfile:   l8 = UNIQUE | NON_NULL, (empty)
        histfile
        // 2029: histfile: typeof(_6) = *mut {l10} *mut {l11} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2029: histfile:   l10 = UNIQUE | NON_NULL, (empty)
        // 2029: histfile:   l11 = UNIQUE | NON_NULL, (empty)
    } else {
        stdout
        // 2031: stdout: typeof(_7) = *mut {l13} *mut {l14} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2031: stdout:   l13 = UNIQUE | NON_NULL, (empty)
        // 2031: stdout:   l14 = UNIQUE | NON_NULL, (empty)
    };
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nassumptions {
    // 2035: nassumptions: typeof(_15) = *mut {l23} i32
    // 2035: nassumptions:   l23 = UNIQUE | NON_NULL, (empty)
        fprintf(
            file,
            // 2037: file: typeof(_17) = *mut {l26} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 2037: file:   l26 = UNIQUE | NON_NULL, (empty)
            b"%.3f\n\0" as *const u8 as *const libc::c_char,
            // 2038: b"%.3f\n\0" as  ... _char: typeof(_18) = *const {l28} i8
            // 2038: b"%.3f\n\0" as  ... _char:   l28 = UNIQUE | NON_NULL, (empty)
            // 2038: b"%.3f\n\0" as  ... st u8: typeof(_19) = *const {l30} u8
            // 2038: b"%.3f\n\0" as  ... st u8:   l30 = UNIQUE | NON_NULL, (empty)
            // 2038: b"%.3f\n\0": typeof(_20) = *const {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2038: b"%.3f\n\0":   l32 = UNIQUE | NON_NULL, (empty)
            // 2038: b"%.3f\n\0": typeof(_21) = & {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2038: b"%.3f\n\0":   l34 = UNIQUE | NON_NULL, FIXED
            // 2038: b"%.3f\n\0": typeof(_21 = const b"%.3f\n\x00") = & {l54} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2038: b"%.3f\n\0":   l54 = UNIQUE | NON_NULL, (empty)
            // 2038: b"%.3f\n\0" as  ... _char: typeof(_18 = move _19 as *const i8 (Misc)) = *const {l57} i8
            // 2038: b"%.3f\n\0" as  ... _char:   l57 = UNIQUE | NON_NULL, (empty)
            // 2038: b"%.3f\n\0": typeof(_20 = &raw const (*_21)) = *const {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 2038: b"%.3f\n\0":   l55 = UNIQUE | NON_NULL, (empty)
            // 2038: b"%.3f\n\0" as  ... st u8: typeof(_19 = move _20 as *const u8 (Pointer(ArrayToPointer))) = *const {l56} u8
            // 2038: b"%.3f\n\0" as  ... st u8:   l56 = UNIQUE | NON_NULL, (empty)
            *times.offset(i as isize),
            // 2039: times.offset(i  ... size): typeof(_23) = *mut {l37} f64
            // 2039: times.offset(i  ... size):   l37 = UNIQUE | NON_NULL, (empty)
            // 2039: times: typeof(_24) = *mut {l39} f64
            // 2039: times:   l39 = UNIQUE | NON_NULL, (empty)
            // 2039: times: typeof(_25) = *mut {l41} *mut {l42} f64
            // 2039: times:   l41 = UNIQUE | NON_NULL, (empty)
            // 2039: times:   l42 = UNIQUE | NON_NULL, (empty)
        );
        i += 1;
        i;
    }
    fflush(file);
    // 2044: file: typeof(_34) = *mut {l52} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2044: file:   l52 = UNIQUE | NON_NULL, (empty)
}
static catchedsig: libc::c_int = 0;
static sig_int_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_segv_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_abrt_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static sig_term_handler: Option<unsafe extern "C" fn(libc::c_int) -> ()> = None;
unsafe extern "C" fn resetsighandlers() {
    signal(2 as libc::c_int, sig_int_handler);
    // 2052: sig_int_handler: typeof(_4) = *mut {l4} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2052: sig_int_handler:   l4 = UNIQUE | NON_NULL, (empty)
    signal(11 as libc::c_int, sig_segv_handler);
    // 2053: sig_segv_handler: typeof(_8) = *mut {l9} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2053: sig_segv_handler:   l9 = UNIQUE | NON_NULL, (empty)
    signal(6 as libc::c_int, sig_abrt_handler);
    // 2054: sig_abrt_handler: typeof(_12) = *mut {l14} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2054: sig_abrt_handler:   l14 = UNIQUE | NON_NULL, (empty)
    signal(15 as libc::c_int, sig_term_handler);
    // 2055: sig_term_handler: typeof(_16) = *mut {l19} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2055: sig_term_handler:   l19 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn caughtsigmsg(mut sig: libc::c_int) {
    if verbose == 0 {
    // 2058: verbose: typeof(_5) = *mut {l5} i32
    // 2058: verbose:   l5 = UNIQUE | NON_NULL, (empty)
        return;
    }
    printf(
        b"c\nc CAUGHT SIGNAL %d\nc\n\0" as *const u8 as *const libc::c_char,
        // 2062: b"c\nc CAUGHT S ... _char: typeof(_8) = *const {l9} i8
        // 2062: b"c\nc CAUGHT S ... _char:   l9 = UNIQUE | NON_NULL, (empty)
        // 2062: b"c\nc CAUGHT S ... st u8: typeof(_9) = *const {l11} u8
        // 2062: b"c\nc CAUGHT S ... st u8:   l11 = UNIQUE | NON_NULL, (empty)
        // 2062: b"c\nc CAUGHT S ... \n\0": typeof(_10) = *const {l13} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2062: b"c\nc CAUGHT S ... \n\0":   l13 = UNIQUE | NON_NULL, (empty)
        // 2062: b"c\nc CAUGHT S ... \n\0": typeof(_11) = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2062: b"c\nc CAUGHT S ... \n\0":   l15 = UNIQUE | NON_NULL, FIXED
        // 2062: b"c\nc CAUGHT S ... \n\0": typeof(_11 = const b"c\nc CAUGHT SIGNAL %d\nc\n\x00") = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2062: b"c\nc CAUGHT S ... \n\0":   l24 = UNIQUE | NON_NULL, (empty)
        // 2062: b"c\nc CAUGHT S ... st u8: typeof(_9 = move _10 as *const u8 (Pointer(ArrayToPointer))) = *const {l26} u8
        // 2062: b"c\nc CAUGHT S ... st u8:   l26 = UNIQUE | NON_NULL, (empty)
        // 2062: b"c\nc CAUGHT S ... \n\0": typeof(_10 = &raw const (*_11)) = *const {l25} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 2062: b"c\nc CAUGHT S ... \n\0":   l25 = UNIQUE | NON_NULL, (empty)
        // 2062: b"c\nc CAUGHT S ... _char: typeof(_8 = move _9 as *const i8 (Misc)) = *const {l27} i8
        // 2062: b"c\nc CAUGHT S ... _char:   l27 = UNIQUE | NON_NULL, (empty)
        sig,
    );
    fflush(stdout);
    // 2065: stdout: typeof(_14) = *mut {l19} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2065: stdout:   l19 = UNIQUE | NON_NULL, (empty)
    // 2065: stdout: typeof(_15) = *mut {l21} *mut {l22} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2065: stdout:   l21 = UNIQUE | NON_NULL, (empty)
    // 2065: stdout:   l22 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn catchsig(mut sig: libc::c_int) {
    if catchedsig == 0 {
    // 2068: catchedsig: typeof(_5) = *mut {l5} i32
    // 2068: catchedsig:   l5 = UNIQUE | NON_NULL, (empty)
        fputs(b"s UNKNOWN\n\0" as *const u8 as *const libc::c_char, stdout);
        // 2069: b"s UNKNOWN\n\0 ... _char: typeof(_7) = *const {l8} i8
        // 2069: b"s UNKNOWN\n\0 ... _char:   l8 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0 ... st u8: typeof(_8) = *const {l10} u8
        // 2069: b"s UNKNOWN\n\0 ... st u8:   l10 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0": typeof(_9) = *const {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2069: b"s UNKNOWN\n\0":   l12 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0": typeof(_10) = & {l14} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2069: b"s UNKNOWN\n\0":   l14 = UNIQUE | NON_NULL, FIXED
        // 2069: stdout: typeof(_11) = *mut {l16} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2069: stdout:   l16 = UNIQUE | NON_NULL, (empty)
        // 2069: stdout: typeof(_12) = *mut {l18} *mut {l19} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2069: stdout:   l18 = UNIQUE | NON_NULL, (empty)
        // 2069: stdout:   l19 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0 ... _char: typeof(_7 = move _8 as *const i8 (Misc)) = *const {l88} i8
        // 2069: b"s UNKNOWN\n\0 ... _char:   l88 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0": typeof(_9 = &raw const (*_10)) = *const {l86} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2069: b"s UNKNOWN\n\0":   l86 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0 ... st u8: typeof(_8 = move _9 as *const u8 (Pointer(ArrayToPointer))) = *const {l87} u8
        // 2069: b"s UNKNOWN\n\0 ... st u8:   l87 = UNIQUE | NON_NULL, (empty)
        // 2069: b"s UNKNOWN\n\0": typeof(_10 = const b"s UNKNOWN\n\x00") = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2069: b"s UNKNOWN\n\0":   l85 = UNIQUE | NON_NULL, (empty)
        fflush(stdout);
        // 2070: stdout: typeof(_14) = *mut {l22} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2070: stdout:   l22 = UNIQUE | NON_NULL, (empty)
        // 2070: stdout: typeof(_15) = *mut {l24} *mut {l25} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2070: stdout:   l24 = UNIQUE | NON_NULL, (empty)
        // 2070: stdout:   l25 = UNIQUE | NON_NULL, (empty)
        catchedsig = 1 as libc::c_int;
        // 2071: catchedsig: typeof(_17) = *mut {l28} i32
        // 2071: catchedsig:   l28 = UNIQUE | NON_NULL, (empty)
        caughtsigmsg(sig);
        if !statsfile.is_null() {
        // 2073: statsfile: typeof(_23) = *mut {l35} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2073: statsfile:   l35 = UNIQUE | NON_NULL, (empty)
        // 2073: statsfile: typeof(_24) = *mut {l37} *mut {l38} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2073: statsfile:   l37 = UNIQUE | NON_NULL, (empty)
        // 2073: statsfile:   l38 = UNIQUE | NON_NULL, (empty)
            stats();
        }
        if !histfile.is_null() {
        // 2076: histfile: typeof(_29) = *mut {l44} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2076: histfile:   l44 = UNIQUE | NON_NULL, (empty)
        // 2076: histfile: typeof(_30) = *mut {l46} *mut {l47} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2076: histfile:   l46 = UNIQUE | NON_NULL, (empty)
        // 2076: histfile:   l47 = UNIQUE | NON_NULL, (empty)
            hist();
        }
        if !statsfile.is_null() || !histfile.is_null() {
        // 2079: statsfile: typeof(_35) = *mut {l53} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2079: statsfile:   l53 = UNIQUE | NON_NULL, (empty)
        // 2079: statsfile: typeof(_36) = *mut {l55} *mut {l56} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2079: statsfile:   l55 = UNIQUE | NON_NULL, (empty)
        // 2079: statsfile:   l56 = UNIQUE | NON_NULL, (empty)
        // 2079: histfile: typeof(_39) = *mut {l60} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2079: histfile:   l60 = UNIQUE | NON_NULL, (empty)
        // 2079: histfile: typeof(_40) = *mut {l62} *mut {l63} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2079: histfile:   l62 = UNIQUE | NON_NULL, (empty)
        // 2079: histfile:   l63 = UNIQUE | NON_NULL, (empty)
            caughtsigmsg(sig);
        }
    }
    resetsighandlers();
    if (getenv(b"LGLNABORT\0" as *const u8 as *const libc::c_char)).is_null() {
    // 2084: (getenv(b"LGLNA ... har)): typeof(_46) = *mut {l70} i8
    // 2084: (getenv(b"LGLNA ... har)):   l70 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0"  ... _char: typeof(_47) = *const {l72} i8
    // 2084: b"LGLNABORT\0"  ... _char:   l72 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0"  ... st u8: typeof(_48) = *const {l74} u8
    // 2084: b"LGLNABORT\0"  ... st u8:   l74 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0": typeof(_49) = *const {l76} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 2084: b"LGLNABORT\0":   l76 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0": typeof(_50) = & {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 2084: b"LGLNABORT\0":   l78 = UNIQUE | NON_NULL, FIXED
    // 2084: b"LGLNABORT\0"  ... st u8: typeof(_48 = move _49 as *const u8 (Pointer(ArrayToPointer))) = *const {l91} u8
    // 2084: b"LGLNABORT\0"  ... st u8:   l91 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0"  ... _char: typeof(_47 = move _48 as *const i8 (Misc)) = *const {l92} i8
    // 2084: b"LGLNABORT\0"  ... _char:   l92 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0": typeof(_50 = const b"LGLNABORT\x00") = & {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 2084: b"LGLNABORT\0":   l89 = UNIQUE | NON_NULL, (empty)
    // 2084: b"LGLNABORT\0": typeof(_49 = &raw const (*_50)) = *const {l90} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
    // 2084: b"LGLNABORT\0":   l90 = UNIQUE | NON_NULL, (empty)
        raise(sig);
    } else {
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn setsighandlers() {
    sig_int_handler = signal(
    // 2091: sig_int_handler: typeof(_5) = *mut {l5} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2091: sig_int_handler:   l5 = UNIQUE | NON_NULL, (empty)
        2 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_segv_handler = signal(
    // 2095: sig_segv_handler: typeof(_10) = *mut {l11} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2095: sig_segv_handler:   l11 = UNIQUE | NON_NULL, (empty)
        11 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_abrt_handler = signal(
    // 2099: sig_abrt_handler: typeof(_15) = *mut {l17} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2099: sig_abrt_handler:   l17 = UNIQUE | NON_NULL, (empty)
        6 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    sig_term_handler = signal(
    // 2103: sig_term_handler: typeof(_20) = *mut {l23} DefId(2:47492 ~ core[480a]::option::Option)<fn(i32) -> ()>
    // 2103: sig_term_handler:   l23 = UNIQUE | NON_NULL, (empty)
        15 as libc::c_int,
        Some(catchsig as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
// 2108: mut argv: typeof(_2) = *mut {g21} *mut {g22} i8
// 2108: mut argv:   g21 = UNIQUE | NON_NULL, FIXED
// 2108: mut argv:   g22 = UNIQUE | NON_NULL, FIXED
    let mut statsfilename: *const libc::c_char = 0 as *const libc::c_char;
    // 2109: mut statsfilename: typeof(_4) = *const {l4} i8
    // 2109: mut statsfilename:   l4 = UNIQUE | NON_NULL, (empty)
    // 2109: 0 as *const lib ... _char: typeof(_4 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1452} i8
    // 2109: 0 as *const lib ... _char:   l1452 = UNIQUE | NON_NULL, (empty)
    let mut histfilename: *const libc::c_char = 0 as *const libc::c_char;
    // 2110: mut histfilename: typeof(_5) = *const {l6} i8
    // 2110: mut histfilename:   l6 = UNIQUE | NON_NULL, (empty)
    // 2110: 0 as *const lib ... _char: typeof(_5 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l1453} i8
    // 2110: 0 as *const lib ... _char:   l1453 = UNIQUE | NON_NULL, (empty)
    let mut i: libc::c_int = 0;
    let mut closeinputfile: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut winner: *mut Worker = 0 as *mut Worker;
    // 2114: mut winner: typeof(_9) = *mut {l11} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2114: mut winner:   l11 = UNIQUE | NON_NULL, (empty)
    // 2114: 0 as *mut Worker: typeof(_9 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1454} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2114: 0 as *mut Worker:   l1454 = UNIQUE | NON_NULL, (empty)
    let mut w: *mut Worker = 0 as *mut Worker;
    // 2115: mut w: typeof(_10) = *mut {l13} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2115: mut w:   l13 = UNIQUE | NON_NULL, (empty)
    // 2115: 0 as *mut Worker: typeof(_10 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1455} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2115: 0 as *mut Worker:   l1455 = UNIQUE | NON_NULL, (empty)
    startime = currentime();
    // 2116: startime: typeof(_12) = *mut {l16} f64
    // 2116: startime:   l16 = UNIQUE | NON_NULL, (empty)
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            // 2120: *argv.offset(i  ... size): typeof(_22) = *const {l27} i8
            // 2120: *argv.offset(i  ... size):   l27 = UNIQUE | NON_NULL, (empty)
            // 2120: *argv.offset(i  ... size): typeof(_23) = *mut {l29} i8
            // 2120: *argv.offset(i  ... size):   l29 = UNIQUE | NON_NULL, (empty)
            // 2120: argv.offset(i a ... size): typeof(_24) = *mut {l31} *mut {l32} i8
            // 2120: argv.offset(i a ... size):   l31 = UNIQUE | NON_NULL, (empty)
            // 2120: argv.offset(i a ... size):   l32 = UNIQUE | NON_NULL, (empty)
            // 2120: argv: typeof(_25) = *mut {l34} *mut {l35} i8
            // 2120: argv:   l34 = UNIQUE | NON_NULL, (empty)
            // 2120: argv:   l35 = UNIQUE | NON_NULL, (empty)
            // 2120: *argv.offset(i  ... size): typeof(_22 = move _23 as *const i8 (Pointer(MutToConstPointer))) = *const {l1456} i8
            // 2120: *argv.offset(i  ... size):   l1456 = UNIQUE | NON_NULL, (empty)
            b"-h\0" as *const u8 as *const libc::c_char,
            // 2121: b"-h\0" as *con ... _char: typeof(_28) = *const {l39} i8
            // 2121: b"-h\0" as *con ... _char:   l39 = UNIQUE | NON_NULL, (empty)
            // 2121: b"-h\0" as *const u8: typeof(_29) = *const {l41} u8
            // 2121: b"-h\0" as *const u8:   l41 = UNIQUE | NON_NULL, (empty)
            // 2121: b"-h\0": typeof(_30) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2121: b"-h\0":   l43 = UNIQUE | NON_NULL, (empty)
            // 2121: b"-h\0": typeof(_31) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2121: b"-h\0":   l45 = UNIQUE | NON_NULL, FIXED
            // 2121: b"-h\0" as *const u8: typeof(_29 = move _30 as *const u8 (Pointer(ArrayToPointer))) = *const {l1459} u8
            // 2121: b"-h\0" as *const u8:   l1459 = UNIQUE | NON_NULL, (empty)
            // 2121: b"-h\0" as *con ... _char: typeof(_28 = move _29 as *const i8 (Misc)) = *const {l1460} i8
            // 2121: b"-h\0" as *con ... _char:   l1460 = UNIQUE | NON_NULL, (empty)
            // 2121: b"-h\0": typeof(_31 = const b"-h\x00") = & {l1457} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2121: b"-h\0":   l1457 = UNIQUE | NON_NULL, (empty)
            // 2121: b"-h\0": typeof(_30 = &raw const (*_31)) = *const {l1458} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2121: b"-h\0":   l1458 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(
                b"usage: ilingeling [<option> ...][<inccnf>][<nworkers>]\n\nwhere <option> is one of the following:\n\n  -h  print this command line option summary\n\n  -v  increase verbose level\n  -q  do not print 'c job ...' lines (requires verbosity < 2)\n  -b  progress bar (implies '-q')\n\n  --version\n\n  -s  <stats> output statistics to separate file\n  -t  <hist> output job run time histogram to separate file\n\n  --clone       use cloning for hard cubes\n  --reduce      reduce learned clause cache after each job\n  --no-melt     do not melt unused assumption variables\n  --no-flush    do not flush learned clause cache before every job\n  --no-reverse  do not reverse assumptions\n\n  --deterministic | --det\n\n            jobs are mapped deterministically to workers\n\n  --no-add    do not add failed assumptions as don't care\n  -A  add all assumptions as don't care\n\n  <inccnf>    'p inccnf' + '<lit*> 0' clauses + 'a <lit>* 0' assumptions\n  <nworkers>  number of workers defaults to 1\n\n  -d|--drup   <path-prefix-for-traces>\n\0"
                // 2125: b"usage: ilinge ... _char: typeof(_34) = *const {l49} i8
                // 2125: b"usage: ilinge ... _char:   l49 = UNIQUE | NON_NULL, (empty)
                // 2125: b"usage: ilinge ... st u8: typeof(_35) = *const {l51} u8
                // 2125: b"usage: ilinge ... st u8:   l51 = UNIQUE | NON_NULL, (empty)
                // 2125: b"usage: ilinge ... \n\0": typeof(_36) = *const {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000003ed)) }]
                // 2125: b"usage: ilinge ... \n\0":   l53 = UNIQUE | NON_NULL, (empty)
                // 2125: b"usage: ilinge ... \n\0": typeof(_37) = & {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000003ed)) }]
                // 2125: b"usage: ilinge ... \n\0":   l55 = UNIQUE | NON_NULL, FIXED
                // 2125: b"usage: ilinge ... \n\0": typeof(_37 = const b"usage: ilingeling [<option> ...][<inccnf>][<nworkers>]\n\nwhere <option> is one of the following:\n\n  -h  print this command line option summary\n\n  -v  increase verbose level\n  -q  do not print \'c job ...\' lines (requires verbosity < 2)\n  -b  progress bar (implies \'-q\')\n\n  --version\n\n  -s  <stats> output statistics to separate file\n  -t  <hist> output job run time histogram to separate file\n\n  --clone       use cloning for hard cubes\n  --reduce      reduce learned clause cache after each job\n  --no-melt     do not melt unused assumption variables\n  --no-flush    do not flush learned clause cache before every job\n  --no-reverse  do not reverse assumptions\n\n  --deterministic | --det\n\n            jobs are mapped deterministically to workers\n\n  --no-add    do not add failed assumptions as don\'t care\n  -A  add all assumptions as don\'t care\n\n  <inccnf>    \'p inccnf\' + \'<lit*> 0\' clauses + \'a <lit>* 0\' assumptions\n  <nworkers>  number of workers defaults to 1\n\n  -d|--drup   <path-prefix-for-traces>\n\x00") = & {l1461} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000003ed)) }]
                // 2125: b"usage: ilinge ... \n\0":   l1461 = UNIQUE | NON_NULL, (empty)
                // 2125: b"usage: ilinge ... st u8: typeof(_35 = move _36 as *const u8 (Pointer(ArrayToPointer))) = *const {l1463} u8
                // 2125: b"usage: ilinge ... st u8:   l1463 = UNIQUE | NON_NULL, (empty)
                // 2125: b"usage: ilinge ... \n\0": typeof(_36 = &raw const (*_37)) = *const {l1462} [u8; Const { ty: usize, kind: Value(Leaf(0x00000000000003ed)) }]
                // 2125: b"usage: ilinge ... \n\0":   l1462 = UNIQUE | NON_NULL, (empty)
                // 2125: b"usage: ilinge ... _char: typeof(_34 = move _35 as *const i8 (Misc)) = *const {l1464} i8
                // 2125: b"usage: ilinge ... _char:   l1464 = UNIQUE | NON_NULL, (empty)
                    as *const u8 as *const libc::c_char,
            );
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            // 2130: *argv.offset(i  ... size): typeof(_42) = *const {l61} i8
            // 2130: *argv.offset(i  ... size):   l61 = UNIQUE | NON_NULL, (empty)
            // 2130: *argv.offset(i  ... size): typeof(_43) = *mut {l63} i8
            // 2130: *argv.offset(i  ... size):   l63 = UNIQUE | NON_NULL, (empty)
            // 2130: argv.offset(i a ... size): typeof(_44) = *mut {l65} *mut {l66} i8
            // 2130: argv.offset(i a ... size):   l65 = UNIQUE | NON_NULL, (empty)
            // 2130: argv.offset(i a ... size):   l66 = UNIQUE | NON_NULL, (empty)
            // 2130: argv: typeof(_45) = *mut {l68} *mut {l69} i8
            // 2130: argv:   l68 = UNIQUE | NON_NULL, (empty)
            // 2130: argv:   l69 = UNIQUE | NON_NULL, (empty)
            // 2130: *argv.offset(i  ... size): typeof(_42 = move _43 as *const i8 (Pointer(MutToConstPointer))) = *const {l1465} i8
            // 2130: *argv.offset(i  ... size):   l1465 = UNIQUE | NON_NULL, (empty)
            b"--version\0" as *const u8 as *const libc::c_char,
            // 2131: b"--version\0"  ... _char: typeof(_48) = *const {l73} i8
            // 2131: b"--version\0"  ... _char:   l73 = UNIQUE | NON_NULL, (empty)
            // 2131: b"--version\0"  ... st u8: typeof(_49) = *const {l75} u8
            // 2131: b"--version\0"  ... st u8:   l75 = UNIQUE | NON_NULL, (empty)
            // 2131: b"--version\0": typeof(_50) = *const {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2131: b"--version\0":   l77 = UNIQUE | NON_NULL, (empty)
            // 2131: b"--version\0": typeof(_51) = & {l79} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2131: b"--version\0":   l79 = UNIQUE | NON_NULL, FIXED
            // 2131: b"--version\0"  ... st u8: typeof(_49 = move _50 as *const u8 (Pointer(ArrayToPointer))) = *const {l1468} u8
            // 2131: b"--version\0"  ... st u8:   l1468 = UNIQUE | NON_NULL, (empty)
            // 2131: b"--version\0": typeof(_51 = const b"--version\x00") = & {l1466} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2131: b"--version\0":   l1466 = UNIQUE | NON_NULL, (empty)
            // 2131: b"--version\0": typeof(_50 = &raw const (*_51)) = *const {l1467} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2131: b"--version\0":   l1467 = UNIQUE | NON_NULL, (empty)
            // 2131: b"--version\0"  ... _char: typeof(_48 = move _49 as *const i8 (Misc)) = *const {l1469} i8
            // 2131: b"--version\0"  ... _char:   l1469 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, lglversion());
            // 2134: b"%s\n\0" as *c ... _char: typeof(_54) = *const {l83} i8
            // 2134: b"%s\n\0" as *c ... _char:   l83 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0" as *c ... st u8: typeof(_55) = *const {l85} u8
            // 2134: b"%s\n\0" as *c ... st u8:   l85 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0": typeof(_56) = *const {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2134: b"%s\n\0":   l87 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0": typeof(_57) = & {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2134: b"%s\n\0":   l89 = UNIQUE | NON_NULL, FIXED
            // 2134: lglversion(): typeof(_58) = *const {l91} i8
            // 2134: lglversion():   l91 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0" as *c ... st u8: typeof(_55 = move _56 as *const u8 (Pointer(ArrayToPointer))) = *const {l1472} u8
            // 2134: b"%s\n\0" as *c ... st u8:   l1472 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0" as *c ... _char: typeof(_54 = move _55 as *const i8 (Misc)) = *const {l1473} i8
            // 2134: b"%s\n\0" as *c ... _char:   l1473 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0": typeof(_56 = &raw const (*_57)) = *const {l1471} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2134: b"%s\n\0":   l1471 = UNIQUE | NON_NULL, (empty)
            // 2134: b"%s\n\0": typeof(_57 = const b"%s\n\x00") = & {l1470} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2134: b"%s\n\0":   l1470 = UNIQUE | NON_NULL, (empty)
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            // 2137: *argv.offset(i  ... size): typeof(_63) = *const {l97} i8
            // 2137: *argv.offset(i  ... size):   l97 = UNIQUE | NON_NULL, (empty)
            // 2137: *argv.offset(i  ... size): typeof(_64) = *mut {l99} i8
            // 2137: *argv.offset(i  ... size):   l99 = UNIQUE | NON_NULL, (empty)
            // 2137: argv.offset(i a ... size): typeof(_65) = *mut {l101} *mut {l102} i8
            // 2137: argv.offset(i a ... size):   l101 = UNIQUE | NON_NULL, (empty)
            // 2137: argv.offset(i a ... size):   l102 = UNIQUE | NON_NULL, (empty)
            // 2137: argv: typeof(_66) = *mut {l104} *mut {l105} i8
            // 2137: argv:   l104 = UNIQUE | NON_NULL, (empty)
            // 2137: argv:   l105 = UNIQUE | NON_NULL, (empty)
            // 2137: *argv.offset(i  ... size): typeof(_63 = move _64 as *const i8 (Pointer(MutToConstPointer))) = *const {l1474} i8
            // 2137: *argv.offset(i  ... size):   l1474 = UNIQUE | NON_NULL, (empty)
            b"-v\0" as *const u8 as *const libc::c_char,
            // 2138: b"-v\0" as *con ... _char: typeof(_69) = *const {l109} i8
            // 2138: b"-v\0" as *con ... _char:   l109 = UNIQUE | NON_NULL, (empty)
            // 2138: b"-v\0" as *const u8: typeof(_70) = *const {l111} u8
            // 2138: b"-v\0" as *const u8:   l111 = UNIQUE | NON_NULL, (empty)
            // 2138: b"-v\0": typeof(_71) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2138: b"-v\0":   l113 = UNIQUE | NON_NULL, (empty)
            // 2138: b"-v\0": typeof(_72) = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2138: b"-v\0":   l115 = UNIQUE | NON_NULL, FIXED
            // 2138: b"-v\0": typeof(_71 = &raw const (*_72)) = *const {l1476} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2138: b"-v\0":   l1476 = UNIQUE | NON_NULL, (empty)
            // 2138: b"-v\0": typeof(_72 = const b"-v\x00") = & {l1475} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2138: b"-v\0":   l1475 = UNIQUE | NON_NULL, (empty)
            // 2138: b"-v\0" as *con ... _char: typeof(_69 = move _70 as *const i8 (Misc)) = *const {l1478} i8
            // 2138: b"-v\0" as *con ... _char:   l1478 = UNIQUE | NON_NULL, (empty)
            // 2138: b"-v\0" as *const u8: typeof(_70 = move _71 as *const u8 (Pointer(ArrayToPointer))) = *const {l1477} u8
            // 2138: b"-v\0" as *const u8:   l1477 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            verbose += 1;
            // 2141: verbose: typeof(_73) = *mut {l117} i32
            // 2141: verbose:   l117 = UNIQUE | NON_NULL, (empty)
            verbose;
            // 2142: verbose: typeof(_76) = *mut {l121} i32
            // 2142: verbose:   l121 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2144: *argv.offset(i  ... size): typeof(_79) = *const {l125} i8
            // 2144: *argv.offset(i  ... size):   l125 = UNIQUE | NON_NULL, (empty)
            // 2144: *argv.offset(i  ... size): typeof(_80) = *mut {l127} i8
            // 2144: *argv.offset(i  ... size):   l127 = UNIQUE | NON_NULL, (empty)
            // 2144: argv.offset(i a ... size): typeof(_81) = *mut {l129} *mut {l130} i8
            // 2144: argv.offset(i a ... size):   l129 = UNIQUE | NON_NULL, (empty)
            // 2144: argv.offset(i a ... size):   l130 = UNIQUE | NON_NULL, (empty)
            // 2144: argv: typeof(_82) = *mut {l132} *mut {l133} i8
            // 2144: argv:   l132 = UNIQUE | NON_NULL, (empty)
            // 2144: argv:   l133 = UNIQUE | NON_NULL, (empty)
            // 2144: *argv.offset(i  ... size): typeof(_79 = move _80 as *const i8 (Pointer(MutToConstPointer))) = *const {l1479} i8
            // 2144: *argv.offset(i  ... size):   l1479 = UNIQUE | NON_NULL, (empty)
            b"-b\0" as *const u8 as *const libc::c_char,
            // 2145: b"-b\0" as *con ... _char: typeof(_85) = *const {l137} i8
            // 2145: b"-b\0" as *con ... _char:   l137 = UNIQUE | NON_NULL, (empty)
            // 2145: b"-b\0" as *const u8: typeof(_86) = *const {l139} u8
            // 2145: b"-b\0" as *const u8:   l139 = UNIQUE | NON_NULL, (empty)
            // 2145: b"-b\0": typeof(_87) = *const {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"-b\0":   l141 = UNIQUE | NON_NULL, (empty)
            // 2145: b"-b\0": typeof(_88) = & {l143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"-b\0":   l143 = UNIQUE | NON_NULL, FIXED
            // 2145: b"-b\0" as *con ... _char: typeof(_85 = move _86 as *const i8 (Misc)) = *const {l1483} i8
            // 2145: b"-b\0" as *con ... _char:   l1483 = UNIQUE | NON_NULL, (empty)
            // 2145: b"-b\0": typeof(_87 = &raw const (*_88)) = *const {l1481} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"-b\0":   l1481 = UNIQUE | NON_NULL, (empty)
            // 2145: b"-b\0" as *const u8: typeof(_86 = move _87 as *const u8 (Pointer(ArrayToPointer))) = *const {l1482} u8
            // 2145: b"-b\0" as *const u8:   l1482 = UNIQUE | NON_NULL, (empty)
            // 2145: b"-b\0": typeof(_88 = const b"-b\x00") = & {l1480} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"-b\0":   l1480 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            bar = 1 as libc::c_int;
            // 2148: bar: typeof(_90) = *mut {l146} i32
            // 2148: bar:   l146 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2150: *argv.offset(i  ... size): typeof(_93) = *const {l150} i8
            // 2150: *argv.offset(i  ... size):   l150 = UNIQUE | NON_NULL, (empty)
            // 2150: *argv.offset(i  ... size): typeof(_94) = *mut {l152} i8
            // 2150: *argv.offset(i  ... size):   l152 = UNIQUE | NON_NULL, (empty)
            // 2150: argv.offset(i a ... size): typeof(_95) = *mut {l154} *mut {l155} i8
            // 2150: argv.offset(i a ... size):   l154 = UNIQUE | NON_NULL, (empty)
            // 2150: argv.offset(i a ... size):   l155 = UNIQUE | NON_NULL, (empty)
            // 2150: argv: typeof(_96) = *mut {l157} *mut {l158} i8
            // 2150: argv:   l157 = UNIQUE | NON_NULL, (empty)
            // 2150: argv:   l158 = UNIQUE | NON_NULL, (empty)
            // 2150: *argv.offset(i  ... size): typeof(_93 = move _94 as *const i8 (Pointer(MutToConstPointer))) = *const {l1484} i8
            // 2150: *argv.offset(i  ... size):   l1484 = UNIQUE | NON_NULL, (empty)
            b"-n\0" as *const u8 as *const libc::c_char,
            // 2151: b"-n\0" as *con ... _char: typeof(_99) = *const {l162} i8
            // 2151: b"-n\0" as *con ... _char:   l162 = UNIQUE | NON_NULL, (empty)
            // 2151: b"-n\0" as *const u8: typeof(_100) = *const {l164} u8
            // 2151: b"-n\0" as *const u8:   l164 = UNIQUE | NON_NULL, (empty)
            // 2151: b"-n\0": typeof(_101) = *const {l166} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2151: b"-n\0":   l166 = UNIQUE | NON_NULL, (empty)
            // 2151: b"-n\0": typeof(_102) = & {l168} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2151: b"-n\0":   l168 = UNIQUE | NON_NULL, FIXED
            // 2151: b"-n\0": typeof(_101 = &raw const (*_102)) = *const {l1486} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2151: b"-n\0":   l1486 = UNIQUE | NON_NULL, (empty)
            // 2151: b"-n\0" as *const u8: typeof(_100 = move _101 as *const u8 (Pointer(ArrayToPointer))) = *const {l1487} u8
            // 2151: b"-n\0" as *const u8:   l1487 = UNIQUE | NON_NULL, (empty)
            // 2151: b"-n\0": typeof(_102 = const b"-n\x00") = & {l1485} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2151: b"-n\0":   l1485 = UNIQUE | NON_NULL, (empty)
            // 2151: b"-n\0" as *con ... _char: typeof(_99 = move _100 as *const i8 (Misc)) = *const {l1488} i8
            // 2151: b"-n\0" as *con ... _char:   l1488 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            nowitness = 1 as libc::c_int;
            // 2154: nowitness: typeof(_104) = *mut {l171} i32
            // 2154: nowitness:   l171 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2156: *argv.offset(i  ... size): typeof(_107) = *const {l175} i8
            // 2156: *argv.offset(i  ... size):   l175 = UNIQUE | NON_NULL, (empty)
            // 2156: *argv.offset(i  ... size): typeof(_108) = *mut {l177} i8
            // 2156: *argv.offset(i  ... size):   l177 = UNIQUE | NON_NULL, (empty)
            // 2156: argv.offset(i a ... size): typeof(_109) = *mut {l179} *mut {l180} i8
            // 2156: argv.offset(i a ... size):   l179 = UNIQUE | NON_NULL, (empty)
            // 2156: argv.offset(i a ... size):   l180 = UNIQUE | NON_NULL, (empty)
            // 2156: argv: typeof(_110) = *mut {l182} *mut {l183} i8
            // 2156: argv:   l182 = UNIQUE | NON_NULL, (empty)
            // 2156: argv:   l183 = UNIQUE | NON_NULL, (empty)
            // 2156: *argv.offset(i  ... size): typeof(_107 = move _108 as *const i8 (Pointer(MutToConstPointer))) = *const {l1489} i8
            // 2156: *argv.offset(i  ... size):   l1489 = UNIQUE | NON_NULL, (empty)
            b"--no-reverse\0" as *const u8 as *const libc::c_char,
            // 2157: b"--no-reverse\ ... _char: typeof(_113) = *const {l187} i8
            // 2157: b"--no-reverse\ ... _char:   l187 = UNIQUE | NON_NULL, (empty)
            // 2157: b"--no-reverse\ ... st u8: typeof(_114) = *const {l189} u8
            // 2157: b"--no-reverse\ ... st u8:   l189 = UNIQUE | NON_NULL, (empty)
            // 2157: b"--no-reverse\0": typeof(_115) = *const {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2157: b"--no-reverse\0":   l191 = UNIQUE | NON_NULL, (empty)
            // 2157: b"--no-reverse\0": typeof(_116) = & {l193} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2157: b"--no-reverse\0":   l193 = UNIQUE | NON_NULL, FIXED
            // 2157: b"--no-reverse\0": typeof(_116 = const b"--no-reverse\x00") = & {l1490} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2157: b"--no-reverse\0":   l1490 = UNIQUE | NON_NULL, (empty)
            // 2157: b"--no-reverse\ ... st u8: typeof(_114 = move _115 as *const u8 (Pointer(ArrayToPointer))) = *const {l1492} u8
            // 2157: b"--no-reverse\ ... st u8:   l1492 = UNIQUE | NON_NULL, (empty)
            // 2157: b"--no-reverse\ ... _char: typeof(_113 = move _114 as *const i8 (Misc)) = *const {l1493} i8
            // 2157: b"--no-reverse\ ... _char:   l1493 = UNIQUE | NON_NULL, (empty)
            // 2157: b"--no-reverse\0": typeof(_115 = &raw const (*_116)) = *const {l1491} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 2157: b"--no-reverse\0":   l1491 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            noreverse = 1 as libc::c_int;
            // 2160: noreverse: typeof(_118) = *mut {l196} i32
            // 2160: noreverse:   l196 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2162: *argv.offset(i  ... size): typeof(_121) = *const {l200} i8
            // 2162: *argv.offset(i  ... size):   l200 = UNIQUE | NON_NULL, (empty)
            // 2162: *argv.offset(i  ... size): typeof(_122) = *mut {l202} i8
            // 2162: *argv.offset(i  ... size):   l202 = UNIQUE | NON_NULL, (empty)
            // 2162: argv.offset(i a ... size): typeof(_123) = *mut {l204} *mut {l205} i8
            // 2162: argv.offset(i a ... size):   l204 = UNIQUE | NON_NULL, (empty)
            // 2162: argv.offset(i a ... size):   l205 = UNIQUE | NON_NULL, (empty)
            // 2162: argv: typeof(_124) = *mut {l207} *mut {l208} i8
            // 2162: argv:   l207 = UNIQUE | NON_NULL, (empty)
            // 2162: argv:   l208 = UNIQUE | NON_NULL, (empty)
            // 2162: *argv.offset(i  ... size): typeof(_121 = move _122 as *const i8 (Pointer(MutToConstPointer))) = *const {l1494} i8
            // 2162: *argv.offset(i  ... size):   l1494 = UNIQUE | NON_NULL, (empty)
            b"--no-add\0" as *const u8 as *const libc::c_char,
            // 2163: b"--no-add\0" a ... _char: typeof(_127) = *const {l212} i8
            // 2163: b"--no-add\0" a ... _char:   l212 = UNIQUE | NON_NULL, (empty)
            // 2163: b"--no-add\0" a ... st u8: typeof(_128) = *const {l214} u8
            // 2163: b"--no-add\0" a ... st u8:   l214 = UNIQUE | NON_NULL, (empty)
            // 2163: b"--no-add\0": typeof(_129) = *const {l216} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2163: b"--no-add\0":   l216 = UNIQUE | NON_NULL, (empty)
            // 2163: b"--no-add\0": typeof(_130) = & {l218} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2163: b"--no-add\0":   l218 = UNIQUE | NON_NULL, FIXED
            // 2163: b"--no-add\0": typeof(_130 = const b"--no-add\x00") = & {l1495} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2163: b"--no-add\0":   l1495 = UNIQUE | NON_NULL, (empty)
            // 2163: b"--no-add\0": typeof(_129 = &raw const (*_130)) = *const {l1496} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2163: b"--no-add\0":   l1496 = UNIQUE | NON_NULL, (empty)
            // 2163: b"--no-add\0" a ... st u8: typeof(_128 = move _129 as *const u8 (Pointer(ArrayToPointer))) = *const {l1497} u8
            // 2163: b"--no-add\0" a ... st u8:   l1497 = UNIQUE | NON_NULL, (empty)
            // 2163: b"--no-add\0" a ... _char: typeof(_127 = move _128 as *const i8 (Misc)) = *const {l1498} i8
            // 2163: b"--no-add\0" a ... _char:   l1498 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            addassumptions = 0 as libc::c_int;
            // 2166: addassumptions: typeof(_132) = *mut {l221} i32
            // 2166: addassumptions:   l221 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2168: *argv.offset(i  ... size): typeof(_135) = *const {l225} i8
            // 2168: *argv.offset(i  ... size):   l225 = UNIQUE | NON_NULL, (empty)
            // 2168: *argv.offset(i  ... size): typeof(_136) = *mut {l227} i8
            // 2168: *argv.offset(i  ... size):   l227 = UNIQUE | NON_NULL, (empty)
            // 2168: argv.offset(i a ... size): typeof(_137) = *mut {l229} *mut {l230} i8
            // 2168: argv.offset(i a ... size):   l229 = UNIQUE | NON_NULL, (empty)
            // 2168: argv.offset(i a ... size):   l230 = UNIQUE | NON_NULL, (empty)
            // 2168: argv: typeof(_138) = *mut {l232} *mut {l233} i8
            // 2168: argv:   l232 = UNIQUE | NON_NULL, (empty)
            // 2168: argv:   l233 = UNIQUE | NON_NULL, (empty)
            // 2168: *argv.offset(i  ... size): typeof(_135 = move _136 as *const i8 (Pointer(MutToConstPointer))) = *const {l1499} i8
            // 2168: *argv.offset(i  ... size):   l1499 = UNIQUE | NON_NULL, (empty)
            b"--no-melt\0" as *const u8 as *const libc::c_char,
            // 2169: b"--no-melt\0"  ... _char: typeof(_141) = *const {l237} i8
            // 2169: b"--no-melt\0"  ... _char:   l237 = UNIQUE | NON_NULL, (empty)
            // 2169: b"--no-melt\0"  ... st u8: typeof(_142) = *const {l239} u8
            // 2169: b"--no-melt\0"  ... st u8:   l239 = UNIQUE | NON_NULL, (empty)
            // 2169: b"--no-melt\0": typeof(_143) = *const {l241} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2169: b"--no-melt\0":   l241 = UNIQUE | NON_NULL, (empty)
            // 2169: b"--no-melt\0": typeof(_144) = & {l243} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2169: b"--no-melt\0":   l243 = UNIQUE | NON_NULL, FIXED
            // 2169: b"--no-melt\0": typeof(_143 = &raw const (*_144)) = *const {l1501} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2169: b"--no-melt\0":   l1501 = UNIQUE | NON_NULL, (empty)
            // 2169: b"--no-melt\0"  ... _char: typeof(_141 = move _142 as *const i8 (Misc)) = *const {l1503} i8
            // 2169: b"--no-melt\0"  ... _char:   l1503 = UNIQUE | NON_NULL, (empty)
            // 2169: b"--no-melt\0"  ... st u8: typeof(_142 = move _143 as *const u8 (Pointer(ArrayToPointer))) = *const {l1502} u8
            // 2169: b"--no-melt\0"  ... st u8:   l1502 = UNIQUE | NON_NULL, (empty)
            // 2169: b"--no-melt\0": typeof(_144 = const b"--no-melt\x00") = & {l1500} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 2169: b"--no-melt\0":   l1500 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            nomelt = 1 as libc::c_int;
            // 2172: nomelt: typeof(_146) = *mut {l246} i32
            // 2172: nomelt:   l246 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2174: *argv.offset(i  ... size): typeof(_149) = *const {l250} i8
            // 2174: *argv.offset(i  ... size):   l250 = UNIQUE | NON_NULL, (empty)
            // 2174: *argv.offset(i  ... size): typeof(_150) = *mut {l252} i8
            // 2174: *argv.offset(i  ... size):   l252 = UNIQUE | NON_NULL, (empty)
            // 2174: argv.offset(i a ... size): typeof(_151) = *mut {l254} *mut {l255} i8
            // 2174: argv.offset(i a ... size):   l254 = UNIQUE | NON_NULL, (empty)
            // 2174: argv.offset(i a ... size):   l255 = UNIQUE | NON_NULL, (empty)
            // 2174: argv: typeof(_152) = *mut {l257} *mut {l258} i8
            // 2174: argv:   l257 = UNIQUE | NON_NULL, (empty)
            // 2174: argv:   l258 = UNIQUE | NON_NULL, (empty)
            // 2174: *argv.offset(i  ... size): typeof(_149 = move _150 as *const i8 (Pointer(MutToConstPointer))) = *const {l1504} i8
            // 2174: *argv.offset(i  ... size):   l1504 = UNIQUE | NON_NULL, (empty)
            b"--reduce\0" as *const u8 as *const libc::c_char,
            // 2175: b"--reduce\0" a ... _char: typeof(_155) = *const {l262} i8
            // 2175: b"--reduce\0" a ... _char:   l262 = UNIQUE | NON_NULL, (empty)
            // 2175: b"--reduce\0" a ... st u8: typeof(_156) = *const {l264} u8
            // 2175: b"--reduce\0" a ... st u8:   l264 = UNIQUE | NON_NULL, (empty)
            // 2175: b"--reduce\0": typeof(_157) = *const {l266} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2175: b"--reduce\0":   l266 = UNIQUE | NON_NULL, (empty)
            // 2175: b"--reduce\0": typeof(_158) = & {l268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2175: b"--reduce\0":   l268 = UNIQUE | NON_NULL, FIXED
            // 2175: b"--reduce\0": typeof(_157 = &raw const (*_158)) = *const {l1506} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2175: b"--reduce\0":   l1506 = UNIQUE | NON_NULL, (empty)
            // 2175: b"--reduce\0": typeof(_158 = const b"--reduce\x00") = & {l1505} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 2175: b"--reduce\0":   l1505 = UNIQUE | NON_NULL, (empty)
            // 2175: b"--reduce\0" a ... st u8: typeof(_156 = move _157 as *const u8 (Pointer(ArrayToPointer))) = *const {l1507} u8
            // 2175: b"--reduce\0" a ... st u8:   l1507 = UNIQUE | NON_NULL, (empty)
            // 2175: b"--reduce\0" a ... _char: typeof(_155 = move _156 as *const i8 (Misc)) = *const {l1508} i8
            // 2175: b"--reduce\0" a ... _char:   l1508 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            reduce = 1 as libc::c_int;
            // 2178: reduce: typeof(_160) = *mut {l271} i32
            // 2178: reduce:   l271 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2180: *argv.offset(i  ... size): typeof(_164) = *const {l276} i8
            // 2180: *argv.offset(i  ... size):   l276 = UNIQUE | NON_NULL, (empty)
            // 2180: *argv.offset(i  ... size): typeof(_165) = *mut {l278} i8
            // 2180: *argv.offset(i  ... size):   l278 = UNIQUE | NON_NULL, (empty)
            // 2180: argv.offset(i a ... size): typeof(_166) = *mut {l280} *mut {l281} i8
            // 2180: argv.offset(i a ... size):   l280 = UNIQUE | NON_NULL, (empty)
            // 2180: argv.offset(i a ... size):   l281 = UNIQUE | NON_NULL, (empty)
            // 2180: argv: typeof(_167) = *mut {l283} *mut {l284} i8
            // 2180: argv:   l283 = UNIQUE | NON_NULL, (empty)
            // 2180: argv:   l284 = UNIQUE | NON_NULL, (empty)
            // 2180: *argv.offset(i  ... size): typeof(_164 = move _165 as *const i8 (Pointer(MutToConstPointer))) = *const {l1509} i8
            // 2180: *argv.offset(i  ... size):   l1509 = UNIQUE | NON_NULL, (empty)
            b"--deterministic\0" as *const u8 as *const libc::c_char,
            // 2181: b"--determinist ... _char: typeof(_170) = *const {l288} i8
            // 2181: b"--determinist ... _char:   l288 = UNIQUE | NON_NULL, (empty)
            // 2181: b"--determinist ... st u8: typeof(_171) = *const {l290} u8
            // 2181: b"--determinist ... st u8:   l290 = UNIQUE | NON_NULL, (empty)
            // 2181: b"--deterministic\0": typeof(_172) = *const {l292} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 2181: b"--deterministic\0":   l292 = UNIQUE | NON_NULL, (empty)
            // 2181: b"--deterministic\0": typeof(_173) = & {l294} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 2181: b"--deterministic\0":   l294 = UNIQUE | NON_NULL, FIXED
            // 2181: b"--deterministic\0": typeof(_173 = const b"--deterministic\x00") = & {l1510} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 2181: b"--deterministic\0":   l1510 = UNIQUE | NON_NULL, (empty)
            // 2181: b"--determinist ... _char: typeof(_170 = move _171 as *const i8 (Misc)) = *const {l1513} i8
            // 2181: b"--determinist ... _char:   l1513 = UNIQUE | NON_NULL, (empty)
            // 2181: b"--determinist ... st u8: typeof(_171 = move _172 as *const u8 (Pointer(ArrayToPointer))) = *const {l1512} u8
            // 2181: b"--determinist ... st u8:   l1512 = UNIQUE | NON_NULL, (empty)
            // 2181: b"--deterministic\0": typeof(_172 = &raw const (*_173)) = *const {l1511} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000010)) }]
            // 2181: b"--deterministic\0":   l1511 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2184: *argv.offset(i  ... size): typeof(_176) = *const {l298} i8
                // 2184: *argv.offset(i  ... size):   l298 = UNIQUE | NON_NULL, (empty)
                // 2184: *argv.offset(i  ... size): typeof(_177) = *mut {l300} i8
                // 2184: *argv.offset(i  ... size):   l300 = UNIQUE | NON_NULL, (empty)
                // 2184: argv.offset(i a ... size): typeof(_178) = *mut {l302} *mut {l303} i8
                // 2184: argv.offset(i a ... size):   l302 = UNIQUE | NON_NULL, (empty)
                // 2184: argv.offset(i a ... size):   l303 = UNIQUE | NON_NULL, (empty)
                // 2184: argv: typeof(_179) = *mut {l305} *mut {l306} i8
                // 2184: argv:   l305 = UNIQUE | NON_NULL, (empty)
                // 2184: argv:   l306 = UNIQUE | NON_NULL, (empty)
                // 2184: *argv.offset(i  ... size): typeof(_176 = move _177 as *const i8 (Pointer(MutToConstPointer))) = *const {l1514} i8
                // 2184: *argv.offset(i  ... size):   l1514 = UNIQUE | NON_NULL, (empty)
                b"--det\0" as *const u8 as *const libc::c_char,
                // 2185: b"--det\0" as * ... _char: typeof(_182) = *const {l310} i8
                // 2185: b"--det\0" as * ... _char:   l310 = UNIQUE | NON_NULL, (empty)
                // 2185: b"--det\0" as * ... st u8: typeof(_183) = *const {l312} u8
                // 2185: b"--det\0" as * ... st u8:   l312 = UNIQUE | NON_NULL, (empty)
                // 2185: b"--det\0": typeof(_184) = *const {l314} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 2185: b"--det\0":   l314 = UNIQUE | NON_NULL, (empty)
                // 2185: b"--det\0": typeof(_185) = & {l316} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 2185: b"--det\0":   l316 = UNIQUE | NON_NULL, FIXED
                // 2185: b"--det\0": typeof(_185 = const b"--det\x00") = & {l1515} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 2185: b"--det\0":   l1515 = UNIQUE | NON_NULL, (empty)
                // 2185: b"--det\0" as * ... st u8: typeof(_183 = move _184 as *const u8 (Pointer(ArrayToPointer))) = *const {l1517} u8
                // 2185: b"--det\0" as * ... st u8:   l1517 = UNIQUE | NON_NULL, (empty)
                // 2185: b"--det\0" as * ... _char: typeof(_182 = move _183 as *const i8 (Misc)) = *const {l1518} i8
                // 2185: b"--det\0" as * ... _char:   l1518 = UNIQUE | NON_NULL, (empty)
                // 2185: b"--det\0": typeof(_184 = &raw const (*_185)) = *const {l1516} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 2185: b"--det\0":   l1516 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            deterministic = 1 as libc::c_int;
            // 2188: deterministic: typeof(_187) = *mut {l319} i32
            // 2188: deterministic:   l319 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2190: *argv.offset(i  ... size): typeof(_190) = *const {l323} i8
            // 2190: *argv.offset(i  ... size):   l323 = UNIQUE | NON_NULL, (empty)
            // 2190: *argv.offset(i  ... size): typeof(_191) = *mut {l325} i8
            // 2190: *argv.offset(i  ... size):   l325 = UNIQUE | NON_NULL, (empty)
            // 2190: argv.offset(i a ... size): typeof(_192) = *mut {l327} *mut {l328} i8
            // 2190: argv.offset(i a ... size):   l327 = UNIQUE | NON_NULL, (empty)
            // 2190: argv.offset(i a ... size):   l328 = UNIQUE | NON_NULL, (empty)
            // 2190: argv: typeof(_193) = *mut {l330} *mut {l331} i8
            // 2190: argv:   l330 = UNIQUE | NON_NULL, (empty)
            // 2190: argv:   l331 = UNIQUE | NON_NULL, (empty)
            // 2190: *argv.offset(i  ... size): typeof(_190 = move _191 as *const i8 (Pointer(MutToConstPointer))) = *const {l1519} i8
            // 2190: *argv.offset(i  ... size):   l1519 = UNIQUE | NON_NULL, (empty)
            b"-A\0" as *const u8 as *const libc::c_char,
            // 2191: b"-A\0" as *con ... _char: typeof(_196) = *const {l335} i8
            // 2191: b"-A\0" as *con ... _char:   l335 = UNIQUE | NON_NULL, (empty)
            // 2191: b"-A\0" as *const u8: typeof(_197) = *const {l337} u8
            // 2191: b"-A\0" as *const u8:   l337 = UNIQUE | NON_NULL, (empty)
            // 2191: b"-A\0": typeof(_198) = *const {l339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2191: b"-A\0":   l339 = UNIQUE | NON_NULL, (empty)
            // 2191: b"-A\0": typeof(_199) = & {l341} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2191: b"-A\0":   l341 = UNIQUE | NON_NULL, FIXED
            // 2191: b"-A\0" as *const u8: typeof(_197 = move _198 as *const u8 (Pointer(ArrayToPointer))) = *const {l1522} u8
            // 2191: b"-A\0" as *const u8:   l1522 = UNIQUE | NON_NULL, (empty)
            // 2191: b"-A\0" as *con ... _char: typeof(_196 = move _197 as *const i8 (Misc)) = *const {l1523} i8
            // 2191: b"-A\0" as *con ... _char:   l1523 = UNIQUE | NON_NULL, (empty)
            // 2191: b"-A\0": typeof(_199 = const b"-A\x00") = & {l1520} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2191: b"-A\0":   l1520 = UNIQUE | NON_NULL, (empty)
            // 2191: b"-A\0": typeof(_198 = &raw const (*_199)) = *const {l1521} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2191: b"-A\0":   l1521 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            addassumptions = 2 as libc::c_int;
            // 2194: addassumptions: typeof(_201) = *mut {l344} i32
            // 2194: addassumptions:   l344 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2196: *argv.offset(i  ... size): typeof(_204) = *const {l348} i8
            // 2196: *argv.offset(i  ... size):   l348 = UNIQUE | NON_NULL, (empty)
            // 2196: *argv.offset(i  ... size): typeof(_205) = *mut {l350} i8
            // 2196: *argv.offset(i  ... size):   l350 = UNIQUE | NON_NULL, (empty)
            // 2196: argv.offset(i a ... size): typeof(_206) = *mut {l352} *mut {l353} i8
            // 2196: argv.offset(i a ... size):   l352 = UNIQUE | NON_NULL, (empty)
            // 2196: argv.offset(i a ... size):   l353 = UNIQUE | NON_NULL, (empty)
            // 2196: argv: typeof(_207) = *mut {l355} *mut {l356} i8
            // 2196: argv:   l355 = UNIQUE | NON_NULL, (empty)
            // 2196: argv:   l356 = UNIQUE | NON_NULL, (empty)
            // 2196: *argv.offset(i  ... size): typeof(_204 = move _205 as *const i8 (Pointer(MutToConstPointer))) = *const {l1524} i8
            // 2196: *argv.offset(i  ... size):   l1524 = UNIQUE | NON_NULL, (empty)
            b"-p\0" as *const u8 as *const libc::c_char,
            // 2197: b"-p\0" as *con ... _char: typeof(_210) = *const {l360} i8
            // 2197: b"-p\0" as *con ... _char:   l360 = UNIQUE | NON_NULL, (empty)
            // 2197: b"-p\0" as *const u8: typeof(_211) = *const {l362} u8
            // 2197: b"-p\0" as *const u8:   l362 = UNIQUE | NON_NULL, (empty)
            // 2197: b"-p\0": typeof(_212) = *const {l364} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2197: b"-p\0":   l364 = UNIQUE | NON_NULL, (empty)
            // 2197: b"-p\0": typeof(_213) = & {l366} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2197: b"-p\0":   l366 = UNIQUE | NON_NULL, FIXED
            // 2197: b"-p\0": typeof(_213 = const b"-p\x00") = & {l1525} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2197: b"-p\0":   l1525 = UNIQUE | NON_NULL, (empty)
            // 2197: b"-p\0": typeof(_212 = &raw const (*_213)) = *const {l1526} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2197: b"-p\0":   l1526 = UNIQUE | NON_NULL, (empty)
            // 2197: b"-p\0" as *const u8: typeof(_211 = move _212 as *const u8 (Pointer(ArrayToPointer))) = *const {l1527} u8
            // 2197: b"-p\0" as *const u8:   l1527 = UNIQUE | NON_NULL, (empty)
            // 2197: b"-p\0" as *con ... _char: typeof(_210 = move _211 as *const i8 (Misc)) = *const {l1528} i8
            // 2197: b"-p\0" as *con ... _char:   l1528 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            plain = 1 as libc::c_int;
            // 2200: plain: typeof(_215) = *mut {l369} i32
            // 2200: plain:   l369 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2202: *argv.offset(i  ... size): typeof(_218) = *const {l373} i8
            // 2202: *argv.offset(i  ... size):   l373 = UNIQUE | NON_NULL, (empty)
            // 2202: *argv.offset(i  ... size): typeof(_219) = *mut {l375} i8
            // 2202: *argv.offset(i  ... size):   l375 = UNIQUE | NON_NULL, (empty)
            // 2202: argv.offset(i a ... size): typeof(_220) = *mut {l377} *mut {l378} i8
            // 2202: argv.offset(i a ... size):   l377 = UNIQUE | NON_NULL, (empty)
            // 2202: argv.offset(i a ... size):   l378 = UNIQUE | NON_NULL, (empty)
            // 2202: argv: typeof(_221) = *mut {l380} *mut {l381} i8
            // 2202: argv:   l380 = UNIQUE | NON_NULL, (empty)
            // 2202: argv:   l381 = UNIQUE | NON_NULL, (empty)
            // 2202: *argv.offset(i  ... size): typeof(_218 = move _219 as *const i8 (Pointer(MutToConstPointer))) = *const {l1529} i8
            // 2202: *argv.offset(i  ... size):   l1529 = UNIQUE | NON_NULL, (empty)
            b"-s\0" as *const u8 as *const libc::c_char,
            // 2203: b"-s\0" as *con ... _char: typeof(_224) = *const {l385} i8
            // 2203: b"-s\0" as *con ... _char:   l385 = UNIQUE | NON_NULL, (empty)
            // 2203: b"-s\0" as *const u8: typeof(_225) = *const {l387} u8
            // 2203: b"-s\0" as *const u8:   l387 = UNIQUE | NON_NULL, (empty)
            // 2203: b"-s\0": typeof(_226) = *const {l389} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2203: b"-s\0":   l389 = UNIQUE | NON_NULL, (empty)
            // 2203: b"-s\0": typeof(_227) = & {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2203: b"-s\0":   l391 = UNIQUE | NON_NULL, FIXED
            // 2203: b"-s\0": typeof(_227 = const b"-s\x00") = & {l1530} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2203: b"-s\0":   l1530 = UNIQUE | NON_NULL, (empty)
            // 2203: b"-s\0": typeof(_226 = &raw const (*_227)) = *const {l1531} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2203: b"-s\0":   l1531 = UNIQUE | NON_NULL, (empty)
            // 2203: b"-s\0" as *con ... _char: typeof(_224 = move _225 as *const i8 (Misc)) = *const {l1533} i8
            // 2203: b"-s\0" as *con ... _char:   l1533 = UNIQUE | NON_NULL, (empty)
            // 2203: b"-s\0" as *const u8: typeof(_225 = move _226 as *const u8 (Pointer(ArrayToPointer))) = *const {l1532} u8
            // 2203: b"-s\0" as *const u8:   l1532 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            if !statsfilename.is_null() {
            // 2206: statsfilename: typeof(_231) = *const {l396} i8
            // 2206: statsfilename:   l396 = UNIQUE | NON_NULL, (empty)
                die(b"two '-s' options\0" as *const u8 as *const libc::c_char);
                // 2207: b"two '-s' opti ... _char: typeof(_233) = *const {l399} i8
                // 2207: b"two '-s' opti ... _char:   l399 = UNIQUE | NON_NULL, (empty)
                // 2207: b"two '-s' opti ... st u8: typeof(_234) = *const {l401} u8
                // 2207: b"two '-s' opti ... st u8:   l401 = UNIQUE | NON_NULL, (empty)
                // 2207: b"two '-s' opti ... ns\0": typeof(_235) = *const {l403} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2207: b"two '-s' opti ... ns\0":   l403 = UNIQUE | NON_NULL, (empty)
                // 2207: b"two '-s' opti ... ns\0": typeof(_236) = & {l405} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2207: b"two '-s' opti ... ns\0":   l405 = UNIQUE | NON_NULL, FIXED
                // 2207: b"two '-s' opti ... ns\0": typeof(_236 = const b"two \'-s\' options\x00") = & {l1534} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2207: b"two '-s' opti ... ns\0":   l1534 = UNIQUE | NON_NULL, (empty)
                // 2207: b"two '-s' opti ... ns\0": typeof(_235 = &raw const (*_236)) = *const {l1535} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2207: b"two '-s' opti ... ns\0":   l1535 = UNIQUE | NON_NULL, (empty)
                // 2207: b"two '-s' opti ... st u8: typeof(_234 = move _235 as *const u8 (Pointer(ArrayToPointer))) = *const {l1536} u8
                // 2207: b"two '-s' opti ... st u8:   l1536 = UNIQUE | NON_NULL, (empty)
                // 2207: b"two '-s' opti ... _char: typeof(_233 = move _234 as *const i8 (Misc)) = *const {l1537} i8
                // 2207: b"two '-s' opti ... _char:   l1537 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            if i == argc {
                die(b"argument to '-s' missing\0" as *const u8 as *const libc::c_char);
                // 2211: b"argument to ' ... _char: typeof(_243) = *const {l413} i8
                // 2211: b"argument to ' ... _char:   l413 = UNIQUE | NON_NULL, (empty)
                // 2211: b"argument to ' ... st u8: typeof(_244) = *const {l415} u8
                // 2211: b"argument to ' ... st u8:   l415 = UNIQUE | NON_NULL, (empty)
                // 2211: b"argument to ' ... ng\0": typeof(_245) = *const {l417} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2211: b"argument to ' ... ng\0":   l417 = UNIQUE | NON_NULL, (empty)
                // 2211: b"argument to ' ... ng\0": typeof(_246) = & {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2211: b"argument to ' ... ng\0":   l419 = UNIQUE | NON_NULL, FIXED
                // 2211: b"argument to ' ... _char: typeof(_243 = move _244 as *const i8 (Misc)) = *const {l1541} i8
                // 2211: b"argument to ' ... _char:   l1541 = UNIQUE | NON_NULL, (empty)
                // 2211: b"argument to ' ... ng\0": typeof(_246 = const b"argument to \'-s\' missing\x00") = & {l1538} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2211: b"argument to ' ... ng\0":   l1538 = UNIQUE | NON_NULL, (empty)
                // 2211: b"argument to ' ... st u8: typeof(_244 = move _245 as *const u8 (Pointer(ArrayToPointer))) = *const {l1540} u8
                // 2211: b"argument to ' ... st u8:   l1540 = UNIQUE | NON_NULL, (empty)
                // 2211: b"argument to ' ... ng\0": typeof(_245 = &raw const (*_246)) = *const {l1539} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2211: b"argument to ' ... ng\0":   l1539 = UNIQUE | NON_NULL, (empty)
            }
            statsfilename = *argv.offset(i as isize);
            // 2213: *argv.offset(i  ... size): typeof(_247) = *mut {l421} i8
            // 2213: *argv.offset(i  ... size):   l421 = UNIQUE | NON_NULL, (empty)
            // 2213: argv.offset(i a ... size): typeof(_248) = *mut {l423} *mut {l424} i8
            // 2213: argv.offset(i a ... size):   l423 = UNIQUE | NON_NULL, (empty)
            // 2213: argv.offset(i a ... size):   l424 = UNIQUE | NON_NULL, (empty)
            // 2213: argv: typeof(_249) = *mut {l426} *mut {l427} i8
            // 2213: argv:   l426 = UNIQUE | NON_NULL, (empty)
            // 2213: argv:   l427 = UNIQUE | NON_NULL, (empty)
            // 2213: statsfilename = ... size): typeof(_4 = move _247 as *const i8 (Pointer(MutToConstPointer))) = *const {l1542} i8
            // 2213: statsfilename = ... size):   l1542 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2215: *argv.offset(i  ... size): typeof(_254) = *const {l433} i8
            // 2215: *argv.offset(i  ... size):   l433 = UNIQUE | NON_NULL, (empty)
            // 2215: *argv.offset(i  ... size): typeof(_255) = *mut {l435} i8
            // 2215: *argv.offset(i  ... size):   l435 = UNIQUE | NON_NULL, (empty)
            // 2215: argv.offset(i a ... size): typeof(_256) = *mut {l437} *mut {l438} i8
            // 2215: argv.offset(i a ... size):   l437 = UNIQUE | NON_NULL, (empty)
            // 2215: argv.offset(i a ... size):   l438 = UNIQUE | NON_NULL, (empty)
            // 2215: argv: typeof(_257) = *mut {l440} *mut {l441} i8
            // 2215: argv:   l440 = UNIQUE | NON_NULL, (empty)
            // 2215: argv:   l441 = UNIQUE | NON_NULL, (empty)
            // 2215: *argv.offset(i  ... size): typeof(_254 = move _255 as *const i8 (Pointer(MutToConstPointer))) = *const {l1543} i8
            // 2215: *argv.offset(i  ... size):   l1543 = UNIQUE | NON_NULL, (empty)
            b"-t\0" as *const u8 as *const libc::c_char,
            // 2216: b"-t\0" as *con ... _char: typeof(_260) = *const {l445} i8
            // 2216: b"-t\0" as *con ... _char:   l445 = UNIQUE | NON_NULL, (empty)
            // 2216: b"-t\0" as *const u8: typeof(_261) = *const {l447} u8
            // 2216: b"-t\0" as *const u8:   l447 = UNIQUE | NON_NULL, (empty)
            // 2216: b"-t\0": typeof(_262) = *const {l449} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2216: b"-t\0":   l449 = UNIQUE | NON_NULL, (empty)
            // 2216: b"-t\0": typeof(_263) = & {l451} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2216: b"-t\0":   l451 = UNIQUE | NON_NULL, FIXED
            // 2216: b"-t\0" as *con ... _char: typeof(_260 = move _261 as *const i8 (Misc)) = *const {l1547} i8
            // 2216: b"-t\0" as *con ... _char:   l1547 = UNIQUE | NON_NULL, (empty)
            // 2216: b"-t\0": typeof(_262 = &raw const (*_263)) = *const {l1545} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2216: b"-t\0":   l1545 = UNIQUE | NON_NULL, (empty)
            // 2216: b"-t\0": typeof(_263 = const b"-t\x00") = & {l1544} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2216: b"-t\0":   l1544 = UNIQUE | NON_NULL, (empty)
            // 2216: b"-t\0" as *const u8: typeof(_261 = move _262 as *const u8 (Pointer(ArrayToPointer))) = *const {l1546} u8
            // 2216: b"-t\0" as *const u8:   l1546 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            if !histfilename.is_null() {
            // 2219: histfilename: typeof(_267) = *const {l456} i8
            // 2219: histfilename:   l456 = UNIQUE | NON_NULL, (empty)
                die(b"two '-t' options\0" as *const u8 as *const libc::c_char);
                // 2220: b"two '-t' opti ... _char: typeof(_269) = *const {l459} i8
                // 2220: b"two '-t' opti ... _char:   l459 = UNIQUE | NON_NULL, (empty)
                // 2220: b"two '-t' opti ... st u8: typeof(_270) = *const {l461} u8
                // 2220: b"two '-t' opti ... st u8:   l461 = UNIQUE | NON_NULL, (empty)
                // 2220: b"two '-t' opti ... ns\0": typeof(_271) = *const {l463} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2220: b"two '-t' opti ... ns\0":   l463 = UNIQUE | NON_NULL, (empty)
                // 2220: b"two '-t' opti ... ns\0": typeof(_272) = & {l465} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2220: b"two '-t' opti ... ns\0":   l465 = UNIQUE | NON_NULL, FIXED
                // 2220: b"two '-t' opti ... ns\0": typeof(_272 = const b"two \'-t\' options\x00") = & {l1548} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2220: b"two '-t' opti ... ns\0":   l1548 = UNIQUE | NON_NULL, (empty)
                // 2220: b"two '-t' opti ... ns\0": typeof(_271 = &raw const (*_272)) = *const {l1549} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2220: b"two '-t' opti ... ns\0":   l1549 = UNIQUE | NON_NULL, (empty)
                // 2220: b"two '-t' opti ... _char: typeof(_269 = move _270 as *const i8 (Misc)) = *const {l1551} i8
                // 2220: b"two '-t' opti ... _char:   l1551 = UNIQUE | NON_NULL, (empty)
                // 2220: b"two '-t' opti ... st u8: typeof(_270 = move _271 as *const u8 (Pointer(ArrayToPointer))) = *const {l1550} u8
                // 2220: b"two '-t' opti ... st u8:   l1550 = UNIQUE | NON_NULL, (empty)
            }
            i += 1;
            if i == argc {
                die(b"argument to '-t' missing\0" as *const u8 as *const libc::c_char);
                // 2224: b"argument to ' ... _char: typeof(_279) = *const {l473} i8
                // 2224: b"argument to ' ... _char:   l473 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... st u8: typeof(_280) = *const {l475} u8
                // 2224: b"argument to ' ... st u8:   l475 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ng\0": typeof(_281) = *const {l477} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2224: b"argument to ' ... ng\0":   l477 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ng\0": typeof(_282) = & {l479} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2224: b"argument to ' ... ng\0":   l479 = UNIQUE | NON_NULL, FIXED
                // 2224: b"argument to ' ... ng\0": typeof(_281 = &raw const (*_282)) = *const {l1553} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2224: b"argument to ' ... ng\0":   l1553 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ng\0": typeof(_282 = const b"argument to \'-t\' missing\x00") = & {l1552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                // 2224: b"argument to ' ... ng\0":   l1552 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... st u8: typeof(_280 = move _281 as *const u8 (Pointer(ArrayToPointer))) = *const {l1554} u8
                // 2224: b"argument to ' ... st u8:   l1554 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... _char: typeof(_279 = move _280 as *const i8 (Misc)) = *const {l1555} i8
                // 2224: b"argument to ' ... _char:   l1555 = UNIQUE | NON_NULL, (empty)
            }
            histfilename = *argv.offset(i as isize);
            // 2226: *argv.offset(i  ... size): typeof(_283) = *mut {l481} i8
            // 2226: *argv.offset(i  ... size):   l481 = UNIQUE | NON_NULL, (empty)
            // 2226: argv.offset(i a ... size): typeof(_284) = *mut {l483} *mut {l484} i8
            // 2226: argv.offset(i a ... size):   l483 = UNIQUE | NON_NULL, (empty)
            // 2226: argv.offset(i a ... size):   l484 = UNIQUE | NON_NULL, (empty)
            // 2226: argv: typeof(_285) = *mut {l486} *mut {l487} i8
            // 2226: argv:   l486 = UNIQUE | NON_NULL, (empty)
            // 2226: argv:   l487 = UNIQUE | NON_NULL, (empty)
            // 2226: histfilename =  ... size): typeof(_5 = move _283 as *const i8 (Pointer(MutToConstPointer))) = *const {l1556} i8
            // 2226: histfilename =  ... size):   l1556 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2228: *argv.offset(i  ... size): typeof(_290) = *const {l493} i8
            // 2228: *argv.offset(i  ... size):   l493 = UNIQUE | NON_NULL, (empty)
            // 2228: *argv.offset(i  ... size): typeof(_291) = *mut {l495} i8
            // 2228: *argv.offset(i  ... size):   l495 = UNIQUE | NON_NULL, (empty)
            // 2228: argv.offset(i a ... size): typeof(_292) = *mut {l497} *mut {l498} i8
            // 2228: argv.offset(i a ... size):   l497 = UNIQUE | NON_NULL, (empty)
            // 2228: argv.offset(i a ... size):   l498 = UNIQUE | NON_NULL, (empty)
            // 2228: argv: typeof(_293) = *mut {l500} *mut {l501} i8
            // 2228: argv:   l500 = UNIQUE | NON_NULL, (empty)
            // 2228: argv:   l501 = UNIQUE | NON_NULL, (empty)
            // 2228: *argv.offset(i  ... size): typeof(_290 = move _291 as *const i8 (Pointer(MutToConstPointer))) = *const {l1557} i8
            // 2228: *argv.offset(i  ... size):   l1557 = UNIQUE | NON_NULL, (empty)
            b"--clone\0" as *const u8 as *const libc::c_char,
            // 2229: b"--clone\0" as ... _char: typeof(_296) = *const {l505} i8
            // 2229: b"--clone\0" as ... _char:   l505 = UNIQUE | NON_NULL, (empty)
            // 2229: b"--clone\0" as ... st u8: typeof(_297) = *const {l507} u8
            // 2229: b"--clone\0" as ... st u8:   l507 = UNIQUE | NON_NULL, (empty)
            // 2229: b"--clone\0": typeof(_298) = *const {l509} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2229: b"--clone\0":   l509 = UNIQUE | NON_NULL, (empty)
            // 2229: b"--clone\0": typeof(_299) = & {l511} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2229: b"--clone\0":   l511 = UNIQUE | NON_NULL, FIXED
            // 2229: b"--clone\0" as ... _char: typeof(_296 = move _297 as *const i8 (Misc)) = *const {l1561} i8
            // 2229: b"--clone\0" as ... _char:   l1561 = UNIQUE | NON_NULL, (empty)
            // 2229: b"--clone\0": typeof(_299 = const b"--clone\x00") = & {l1558} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2229: b"--clone\0":   l1558 = UNIQUE | NON_NULL, (empty)
            // 2229: b"--clone\0": typeof(_298 = &raw const (*_299)) = *const {l1559} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 2229: b"--clone\0":   l1559 = UNIQUE | NON_NULL, (empty)
            // 2229: b"--clone\0" as ... st u8: typeof(_297 = move _298 as *const u8 (Pointer(ArrayToPointer))) = *const {l1560} u8
            // 2229: b"--clone\0" as ... st u8:   l1560 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            doclone = 1 as libc::c_int;
            // 2232: doclone: typeof(_301) = *mut {l514} i32
            // 2232: doclone:   l514 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2234: *argv.offset(i  ... size): typeof(_304) = *const {l518} i8
            // 2234: *argv.offset(i  ... size):   l518 = UNIQUE | NON_NULL, (empty)
            // 2234: *argv.offset(i  ... size): typeof(_305) = *mut {l520} i8
            // 2234: *argv.offset(i  ... size):   l520 = UNIQUE | NON_NULL, (empty)
            // 2234: argv.offset(i a ... size): typeof(_306) = *mut {l522} *mut {l523} i8
            // 2234: argv.offset(i a ... size):   l522 = UNIQUE | NON_NULL, (empty)
            // 2234: argv.offset(i a ... size):   l523 = UNIQUE | NON_NULL, (empty)
            // 2234: argv: typeof(_307) = *mut {l525} *mut {l526} i8
            // 2234: argv:   l525 = UNIQUE | NON_NULL, (empty)
            // 2234: argv:   l526 = UNIQUE | NON_NULL, (empty)
            // 2234: *argv.offset(i  ... size): typeof(_304 = move _305 as *const i8 (Pointer(MutToConstPointer))) = *const {l1562} i8
            // 2234: *argv.offset(i  ... size):   l1562 = UNIQUE | NON_NULL, (empty)
            b"--no-flush\0" as *const u8 as *const libc::c_char,
            // 2235: b"--no-flush\0" ... _char: typeof(_310) = *const {l530} i8
            // 2235: b"--no-flush\0" ... _char:   l530 = UNIQUE | NON_NULL, (empty)
            // 2235: b"--no-flush\0" ... st u8: typeof(_311) = *const {l532} u8
            // 2235: b"--no-flush\0" ... st u8:   l532 = UNIQUE | NON_NULL, (empty)
            // 2235: b"--no-flush\0": typeof(_312) = *const {l534} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2235: b"--no-flush\0":   l534 = UNIQUE | NON_NULL, (empty)
            // 2235: b"--no-flush\0": typeof(_313) = & {l536} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2235: b"--no-flush\0":   l536 = UNIQUE | NON_NULL, FIXED
            // 2235: b"--no-flush\0": typeof(_312 = &raw const (*_313)) = *const {l1564} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2235: b"--no-flush\0":   l1564 = UNIQUE | NON_NULL, (empty)
            // 2235: b"--no-flush\0": typeof(_313 = const b"--no-flush\x00") = & {l1563} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2235: b"--no-flush\0":   l1563 = UNIQUE | NON_NULL, (empty)
            // 2235: b"--no-flush\0" ... st u8: typeof(_311 = move _312 as *const u8 (Pointer(ArrayToPointer))) = *const {l1565} u8
            // 2235: b"--no-flush\0" ... st u8:   l1565 = UNIQUE | NON_NULL, (empty)
            // 2235: b"--no-flush\0" ... _char: typeof(_310 = move _311 as *const i8 (Misc)) = *const {l1566} i8
            // 2235: b"--no-flush\0" ... _char:   l1566 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            noflush = 1 as libc::c_int;
            // 2238: noflush: typeof(_315) = *mut {l539} i32
            // 2238: noflush:   l539 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2240: *argv.offset(i  ... size): typeof(_319) = *const {l544} i8
            // 2240: *argv.offset(i  ... size):   l544 = UNIQUE | NON_NULL, (empty)
            // 2240: *argv.offset(i  ... size): typeof(_320) = *mut {l546} i8
            // 2240: *argv.offset(i  ... size):   l546 = UNIQUE | NON_NULL, (empty)
            // 2240: argv.offset(i a ... size): typeof(_321) = *mut {l548} *mut {l549} i8
            // 2240: argv.offset(i a ... size):   l548 = UNIQUE | NON_NULL, (empty)
            // 2240: argv.offset(i a ... size):   l549 = UNIQUE | NON_NULL, (empty)
            // 2240: argv: typeof(_322) = *mut {l551} *mut {l552} i8
            // 2240: argv:   l551 = UNIQUE | NON_NULL, (empty)
            // 2240: argv:   l552 = UNIQUE | NON_NULL, (empty)
            // 2240: *argv.offset(i  ... size): typeof(_319 = move _320 as *const i8 (Pointer(MutToConstPointer))) = *const {l1567} i8
            // 2240: *argv.offset(i  ... size):   l1567 = UNIQUE | NON_NULL, (empty)
            b"-d\0" as *const u8 as *const libc::c_char,
            // 2241: b"-d\0" as *con ... _char: typeof(_325) = *const {l556} i8
            // 2241: b"-d\0" as *con ... _char:   l556 = UNIQUE | NON_NULL, (empty)
            // 2241: b"-d\0" as *const u8: typeof(_326) = *const {l558} u8
            // 2241: b"-d\0" as *const u8:   l558 = UNIQUE | NON_NULL, (empty)
            // 2241: b"-d\0": typeof(_327) = *const {l560} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2241: b"-d\0":   l560 = UNIQUE | NON_NULL, (empty)
            // 2241: b"-d\0": typeof(_328) = & {l562} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2241: b"-d\0":   l562 = UNIQUE | NON_NULL, FIXED
            // 2241: b"-d\0": typeof(_327 = &raw const (*_328)) = *const {l1569} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2241: b"-d\0":   l1569 = UNIQUE | NON_NULL, (empty)
            // 2241: b"-d\0" as *const u8: typeof(_326 = move _327 as *const u8 (Pointer(ArrayToPointer))) = *const {l1570} u8
            // 2241: b"-d\0" as *const u8:   l1570 = UNIQUE | NON_NULL, (empty)
            // 2241: b"-d\0": typeof(_328 = const b"-d\x00") = & {l1568} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2241: b"-d\0":   l1568 = UNIQUE | NON_NULL, (empty)
            // 2241: b"-d\0" as *con ... _char: typeof(_325 = move _326 as *const i8 (Misc)) = *const {l1571} i8
            // 2241: b"-d\0" as *con ... _char:   l1571 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2244: *argv.offset(i  ... size): typeof(_331) = *const {l566} i8
                // 2244: *argv.offset(i  ... size):   l566 = UNIQUE | NON_NULL, (empty)
                // 2244: *argv.offset(i  ... size): typeof(_332) = *mut {l568} i8
                // 2244: *argv.offset(i  ... size):   l568 = UNIQUE | NON_NULL, (empty)
                // 2244: argv.offset(i a ... size): typeof(_333) = *mut {l570} *mut {l571} i8
                // 2244: argv.offset(i a ... size):   l570 = UNIQUE | NON_NULL, (empty)
                // 2244: argv.offset(i a ... size):   l571 = UNIQUE | NON_NULL, (empty)
                // 2244: argv: typeof(_334) = *mut {l573} *mut {l574} i8
                // 2244: argv:   l573 = UNIQUE | NON_NULL, (empty)
                // 2244: argv:   l574 = UNIQUE | NON_NULL, (empty)
                // 2244: *argv.offset(i  ... size): typeof(_331 = move _332 as *const i8 (Pointer(MutToConstPointer))) = *const {l1572} i8
                // 2244: *argv.offset(i  ... size):   l1572 = UNIQUE | NON_NULL, (empty)
                b"--drup\0" as *const u8 as *const libc::c_char,
                // 2245: b"--drup\0" as  ... _char: typeof(_337) = *const {l578} i8
                // 2245: b"--drup\0" as  ... _char:   l578 = UNIQUE | NON_NULL, (empty)
                // 2245: b"--drup\0" as  ... st u8: typeof(_338) = *const {l580} u8
                // 2245: b"--drup\0" as  ... st u8:   l580 = UNIQUE | NON_NULL, (empty)
                // 2245: b"--drup\0": typeof(_339) = *const {l582} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 2245: b"--drup\0":   l582 = UNIQUE | NON_NULL, (empty)
                // 2245: b"--drup\0": typeof(_340) = & {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 2245: b"--drup\0":   l584 = UNIQUE | NON_NULL, FIXED
                // 2245: b"--drup\0" as  ... st u8: typeof(_338 = move _339 as *const u8 (Pointer(ArrayToPointer))) = *const {l1575} u8
                // 2245: b"--drup\0" as  ... st u8:   l1575 = UNIQUE | NON_NULL, (empty)
                // 2245: b"--drup\0": typeof(_339 = &raw const (*_340)) = *const {l1574} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 2245: b"--drup\0":   l1574 = UNIQUE | NON_NULL, (empty)
                // 2245: b"--drup\0" as  ... _char: typeof(_337 = move _338 as *const i8 (Misc)) = *const {l1576} i8
                // 2245: b"--drup\0" as  ... _char:   l1576 = UNIQUE | NON_NULL, (empty)
                // 2245: b"--drup\0": typeof(_340 = const b"--drup\x00") = & {l1573} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 2245: b"--drup\0":   l1573 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            i += 1;
            if i == argc {
                die(
                    b"argument to '%s' missing\0" as *const u8 as *const libc::c_char,
                    // 2251: b"argument to ' ... _char: typeof(_347) = *const {l592} i8
                    // 2251: b"argument to ' ... _char:   l592 = UNIQUE | NON_NULL, (empty)
                    // 2251: b"argument to ' ... st u8: typeof(_348) = *const {l594} u8
                    // 2251: b"argument to ' ... st u8:   l594 = UNIQUE | NON_NULL, (empty)
                    // 2251: b"argument to ' ... ng\0": typeof(_349) = *const {l596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2251: b"argument to ' ... ng\0":   l596 = UNIQUE | NON_NULL, (empty)
                    // 2251: b"argument to ' ... ng\0": typeof(_350) = & {l598} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2251: b"argument to ' ... ng\0":   l598 = UNIQUE | NON_NULL, FIXED
                    // 2251: b"argument to ' ... st u8: typeof(_348 = move _349 as *const u8 (Pointer(ArrayToPointer))) = *const {l1579} u8
                    // 2251: b"argument to ' ... st u8:   l1579 = UNIQUE | NON_NULL, (empty)
                    // 2251: b"argument to ' ... ng\0": typeof(_350 = const b"argument to \'%s\' missing\x00") = & {l1577} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2251: b"argument to ' ... ng\0":   l1577 = UNIQUE | NON_NULL, (empty)
                    // 2251: b"argument to ' ... _char: typeof(_347 = move _348 as *const i8 (Misc)) = *const {l1580} i8
                    // 2251: b"argument to ' ... _char:   l1580 = UNIQUE | NON_NULL, (empty)
                    // 2251: b"argument to ' ... ng\0": typeof(_349 = &raw const (*_350)) = *const {l1578} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2251: b"argument to ' ... ng\0":   l1578 = UNIQUE | NON_NULL, (empty)
                    *argv.offset(i as isize),
                    // 2252: *argv.offset(i  ... size): typeof(_351) = *mut {l600} i8
                    // 2252: *argv.offset(i  ... size):   l600 = UNIQUE | NON_NULL, (empty)
                    // 2252: argv.offset(i a ... size): typeof(_352) = *mut {l602} *mut {l603} i8
                    // 2252: argv.offset(i a ... size):   l602 = UNIQUE | NON_NULL, (empty)
                    // 2252: argv.offset(i a ... size):   l603 = UNIQUE | NON_NULL, (empty)
                    // 2252: argv: typeof(_353) = *mut {l605} *mut {l606} i8
                    // 2252: argv:   l605 = UNIQUE | NON_NULL, (empty)
                    // 2252: argv:   l606 = UNIQUE | NON_NULL, (empty)
                );
            }
            if !druptraceprefix.is_null() {
            // 2255: druptraceprefix: typeof(_359) = *const {l613} i8
            // 2255: druptraceprefix:   l613 = UNIQUE | NON_NULL, (empty)
            // 2255: druptraceprefix: typeof(_360) = *mut {l615} *const {l616} i8
            // 2255: druptraceprefix:   l615 = UNIQUE | NON_NULL, (empty)
            // 2255: druptraceprefix:   l616 = UNIQUE | NON_NULL, (empty)
                die(b"DRUP path prefix set twice\0" as *const u8 as *const libc::c_char);
                // 2256: b"DRUP path pre ... _char: typeof(_362) = *const {l619} i8
                // 2256: b"DRUP path pre ... _char:   l619 = UNIQUE | NON_NULL, (empty)
                // 2256: b"DRUP path pre ... st u8: typeof(_363) = *const {l621} u8
                // 2256: b"DRUP path pre ... st u8:   l621 = UNIQUE | NON_NULL, (empty)
                // 2256: b"DRUP path pre ... ce\0": typeof(_364) = *const {l623} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2256: b"DRUP path pre ... ce\0":   l623 = UNIQUE | NON_NULL, (empty)
                // 2256: b"DRUP path pre ... ce\0": typeof(_365) = & {l625} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2256: b"DRUP path pre ... ce\0":   l625 = UNIQUE | NON_NULL, FIXED
                // 2256: b"DRUP path pre ... st u8: typeof(_363 = move _364 as *const u8 (Pointer(ArrayToPointer))) = *const {l1583} u8
                // 2256: b"DRUP path pre ... st u8:   l1583 = UNIQUE | NON_NULL, (empty)
                // 2256: b"DRUP path pre ... _char: typeof(_362 = move _363 as *const i8 (Misc)) = *const {l1584} i8
                // 2256: b"DRUP path pre ... _char:   l1584 = UNIQUE | NON_NULL, (empty)
                // 2256: b"DRUP path pre ... ce\0": typeof(_365 = const b"DRUP path prefix set twice\x00") = & {l1581} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2256: b"DRUP path pre ... ce\0":   l1581 = UNIQUE | NON_NULL, (empty)
                // 2256: b"DRUP path pre ... ce\0": typeof(_364 = &raw const (*_365)) = *const {l1582} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
                // 2256: b"DRUP path pre ... ce\0":   l1582 = UNIQUE | NON_NULL, (empty)
            }
            druptraceprefix = *argv.offset(i as isize);
            // 2258: *argv.offset(i  ... size): typeof(_366) = *mut {l627} i8
            // 2258: *argv.offset(i  ... size):   l627 = UNIQUE | NON_NULL, (empty)
            // 2258: argv.offset(i a ... size): typeof(_367) = *mut {l629} *mut {l630} i8
            // 2258: argv.offset(i a ... size):   l629 = UNIQUE | NON_NULL, (empty)
            // 2258: argv.offset(i a ... size):   l630 = UNIQUE | NON_NULL, (empty)
            // 2258: argv: typeof(_368) = *mut {l632} *mut {l633} i8
            // 2258: argv:   l632 = UNIQUE | NON_NULL, (empty)
            // 2258: argv:   l633 = UNIQUE | NON_NULL, (empty)
            // 2258: druptraceprefix: typeof(_371) = *mut {l637} *const {l638} i8
            // 2258: druptraceprefix:   l637 = UNIQUE | NON_NULL, (empty)
            // 2258: druptraceprefix:   l638 = UNIQUE | NON_NULL, (empty)
            // 2258: druptraceprefix ... size): typeof((*_371) = move _366 as *const i8 (Pointer(MutToConstPointer))) = *const {l1585} i8
            // 2258: druptraceprefix ... size):   l1585 = UNIQUE | NON_NULL, (empty)
        } else if *(*argv.offset(i as isize)).offset(0 as libc::c_int as isize) as libc::c_int
        // 2259: (*argv.offset(i ... size): typeof(_375) = *mut {l643} i8
        // 2259: (*argv.offset(i ... size):   l643 = UNIQUE | NON_NULL, (empty)
        // 2259: (*argv.offset(i ... ize)): typeof(_376) = *mut {l645} i8
        // 2259: (*argv.offset(i ... ize)):   l645 = UNIQUE | NON_NULL, (empty)
        // 2259: argv.offset(i a ... size): typeof(_377) = *mut {l647} *mut {l648} i8
        // 2259: argv.offset(i a ... size):   l647 = UNIQUE | NON_NULL, (empty)
        // 2259: argv.offset(i a ... size):   l648 = UNIQUE | NON_NULL, (empty)
        // 2259: argv: typeof(_378) = *mut {l650} *mut {l651} i8
        // 2259: argv:   l650 = UNIQUE | NON_NULL, (empty)
        // 2259: argv:   l651 = UNIQUE | NON_NULL, (empty)
            == '-' as i32
        {
            die(
                b"invalid option '%s'\0" as *const u8 as *const libc::c_char,
                // 2263: b"invalid optio ... _char: typeof(_385) = *const {l659} i8
                // 2263: b"invalid optio ... _char:   l659 = UNIQUE | NON_NULL, (empty)
                // 2263: b"invalid optio ... st u8: typeof(_386) = *const {l661} u8
                // 2263: b"invalid optio ... st u8:   l661 = UNIQUE | NON_NULL, (empty)
                // 2263: b"invalid optio ... s'\0": typeof(_387) = *const {l663} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                // 2263: b"invalid optio ... s'\0":   l663 = UNIQUE | NON_NULL, (empty)
                // 2263: b"invalid optio ... s'\0": typeof(_388) = & {l665} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                // 2263: b"invalid optio ... s'\0":   l665 = UNIQUE | NON_NULL, FIXED
                // 2263: b"invalid optio ... _char: typeof(_385 = move _386 as *const i8 (Misc)) = *const {l1589} i8
                // 2263: b"invalid optio ... _char:   l1589 = UNIQUE | NON_NULL, (empty)
                // 2263: b"invalid optio ... s'\0": typeof(_388 = const b"invalid option \'%s\'\x00") = & {l1586} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                // 2263: b"invalid optio ... s'\0":   l1586 = UNIQUE | NON_NULL, (empty)
                // 2263: b"invalid optio ... st u8: typeof(_386 = move _387 as *const u8 (Pointer(ArrayToPointer))) = *const {l1588} u8
                // 2263: b"invalid optio ... st u8:   l1588 = UNIQUE | NON_NULL, (empty)
                // 2263: b"invalid optio ... s'\0": typeof(_387 = &raw const (*_388)) = *const {l1587} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
                // 2263: b"invalid optio ... s'\0":   l1587 = UNIQUE | NON_NULL, (empty)
                *argv.offset(i as isize),
                // 2264: *argv.offset(i  ... size): typeof(_389) = *mut {l667} i8
                // 2264: *argv.offset(i  ... size):   l667 = UNIQUE | NON_NULL, (empty)
                // 2264: argv.offset(i a ... size): typeof(_390) = *mut {l669} *mut {l670} i8
                // 2264: argv.offset(i a ... size):   l669 = UNIQUE | NON_NULL, (empty)
                // 2264: argv.offset(i a ... size):   l670 = UNIQUE | NON_NULL, (empty)
                // 2264: argv: typeof(_391) = *mut {l672} *mut {l673} i8
                // 2264: argv:   l672 = UNIQUE | NON_NULL, (empty)
                // 2264: argv:   l673 = UNIQUE | NON_NULL, (empty)
            );
        } else if isnum(*argv.offset(i as isize)) != 0 {
        // 2266: *argv.offset(i  ... size): typeof(_396) = *const {l679} i8
        // 2266: *argv.offset(i  ... size):   l679 = UNIQUE | NON_NULL, (empty)
        // 2266: *argv.offset(i  ... size): typeof(_397) = *mut {l681} i8
        // 2266: *argv.offset(i  ... size):   l681 = UNIQUE | NON_NULL, (empty)
        // 2266: argv.offset(i a ... size): typeof(_398) = *mut {l683} *mut {l684} i8
        // 2266: argv.offset(i a ... size):   l683 = UNIQUE | NON_NULL, (empty)
        // 2266: argv.offset(i a ... size):   l684 = UNIQUE | NON_NULL, (empty)
        // 2266: argv: typeof(_399) = *mut {l686} *mut {l687} i8
        // 2266: argv:   l686 = UNIQUE | NON_NULL, (empty)
        // 2266: argv:   l687 = UNIQUE | NON_NULL, (empty)
        // 2266: *argv.offset(i  ... size): typeof(_396 = move _397 as *const i8 (Pointer(MutToConstPointer))) = *const {l1590} i8
        // 2266: *argv.offset(i  ... size):   l1590 = UNIQUE | NON_NULL, (empty)
            if nworkers != 0 {
            // 2267: nworkers: typeof(_405) = *mut {l694} i32
            // 2267: nworkers:   l694 = UNIQUE | NON_NULL, (empty)
                die(
                    b"number of workers specified twice: '%d' and '%s'\0" as *const u8
                    // 2269: b"number of wor ... _char: typeof(_407) = *const {l697} i8
                    // 2269: b"number of wor ... _char:   l697 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"number of wor ... st u8: typeof(_408) = *const {l699} u8
                    // 2269: b"number of wor ... st u8:   l699 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"number of wor ... s'\0": typeof(_409) = *const {l701} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 2269: b"number of wor ... s'\0":   l701 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"number of wor ... s'\0": typeof(_410) = & {l703} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 2269: b"number of wor ... s'\0":   l703 = UNIQUE | NON_NULL, FIXED
                    // 2269: b"number of wor ... s'\0": typeof(_410 = const b"number of workers specified twice: \'%d\' and \'%s\'\x00") = & {l1591} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 2269: b"number of wor ... s'\0":   l1591 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"number of wor ... st u8: typeof(_408 = move _409 as *const u8 (Pointer(ArrayToPointer))) = *const {l1593} u8
                    // 2269: b"number of wor ... st u8:   l1593 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"number of wor ... s'\0": typeof(_409 = &raw const (*_410)) = *const {l1592} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
                    // 2269: b"number of wor ... s'\0":   l1592 = UNIQUE | NON_NULL, (empty)
                    // 2269: b"number of wor ... _char: typeof(_407 = move _408 as *const i8 (Misc)) = *const {l1594} i8
                    // 2269: b"number of wor ... _char:   l1594 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    nworkers,
                    // 2271: nworkers: typeof(_412) = *mut {l706} i32
                    // 2271: nworkers:   l706 = UNIQUE | NON_NULL, (empty)
                    *argv.offset(i as isize),
                    // 2272: *argv.offset(i  ... size): typeof(_413) = *mut {l708} i8
                    // 2272: *argv.offset(i  ... size):   l708 = UNIQUE | NON_NULL, (empty)
                    // 2272: argv.offset(i a ... size): typeof(_414) = *mut {l710} *mut {l711} i8
                    // 2272: argv.offset(i a ... size):   l710 = UNIQUE | NON_NULL, (empty)
                    // 2272: argv.offset(i a ... size):   l711 = UNIQUE | NON_NULL, (empty)
                    // 2272: argv: typeof(_415) = *mut {l713} *mut {l714} i8
                    // 2272: argv:   l713 = UNIQUE | NON_NULL, (empty)
                    // 2272: argv:   l714 = UNIQUE | NON_NULL, (empty)
                );
            }
            nworkers = atoi(*argv.offset(i as isize));
            // 2275: *argv.offset(i  ... size): typeof(_419) = *const {l719} i8
            // 2275: *argv.offset(i  ... size):   l719 = UNIQUE | NON_NULL, (empty)
            // 2275: *argv.offset(i  ... size): typeof(_420) = *mut {l721} i8
            // 2275: *argv.offset(i  ... size):   l721 = UNIQUE | NON_NULL, (empty)
            // 2275: argv.offset(i a ... size): typeof(_421) = *mut {l723} *mut {l724} i8
            // 2275: argv.offset(i a ... size):   l723 = UNIQUE | NON_NULL, (empty)
            // 2275: argv.offset(i a ... size):   l724 = UNIQUE | NON_NULL, (empty)
            // 2275: argv: typeof(_422) = *mut {l726} *mut {l727} i8
            // 2275: argv:   l726 = UNIQUE | NON_NULL, (empty)
            // 2275: argv:   l727 = UNIQUE | NON_NULL, (empty)
            // 2275: nworkers: typeof(_425) = *mut {l731} i32
            // 2275: nworkers:   l731 = UNIQUE | NON_NULL, (empty)
            // 2275: *argv.offset(i  ... size): typeof(_419 = move _420 as *const i8 (Pointer(MutToConstPointer))) = *const {l1595} i8
            // 2275: *argv.offset(i  ... size):   l1595 = UNIQUE | NON_NULL, (empty)
            if nworkers <= 0 as libc::c_int {
            // 2276: nworkers: typeof(_428) = *mut {l735} i32
            // 2276: nworkers:   l735 = UNIQUE | NON_NULL, (empty)
                die(
                    b"invalid number of workers argument: '%s'\0" as *const u8
                    // 2278: b"invalid numbe ... _char: typeof(_431) = *const {l739} i8
                    // 2278: b"invalid numbe ... _char:   l739 = UNIQUE | NON_NULL, (empty)
                    // 2278: b"invalid numbe ... st u8: typeof(_432) = *const {l741} u8
                    // 2278: b"invalid numbe ... st u8:   l741 = UNIQUE | NON_NULL, (empty)
                    // 2278: b"invalid numbe ... s'\0": typeof(_433) = *const {l743} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 2278: b"invalid numbe ... s'\0":   l743 = UNIQUE | NON_NULL, (empty)
                    // 2278: b"invalid numbe ... s'\0": typeof(_434) = & {l745} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 2278: b"invalid numbe ... s'\0":   l745 = UNIQUE | NON_NULL, FIXED
                    // 2278: b"invalid numbe ... s'\0": typeof(_433 = &raw const (*_434)) = *const {l1597} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 2278: b"invalid numbe ... s'\0":   l1597 = UNIQUE | NON_NULL, (empty)
                    // 2278: b"invalid numbe ... st u8: typeof(_432 = move _433 as *const u8 (Pointer(ArrayToPointer))) = *const {l1598} u8
                    // 2278: b"invalid numbe ... st u8:   l1598 = UNIQUE | NON_NULL, (empty)
                    // 2278: b"invalid numbe ... s'\0": typeof(_434 = const b"invalid number of workers argument: \'%s\'\x00") = & {l1596} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
                    // 2278: b"invalid numbe ... s'\0":   l1596 = UNIQUE | NON_NULL, (empty)
                    // 2278: b"invalid numbe ... _char: typeof(_431 = move _432 as *const i8 (Misc)) = *const {l1599} i8
                    // 2278: b"invalid numbe ... _char:   l1599 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                    // 2280: *argv.offset(i  ... size): typeof(_435) = *mut {l747} i8
                    // 2280: *argv.offset(i  ... size):   l747 = UNIQUE | NON_NULL, (empty)
                    // 2280: argv.offset(i a ... size): typeof(_436) = *mut {l749} *mut {l750} i8
                    // 2280: argv.offset(i a ... size):   l749 = UNIQUE | NON_NULL, (empty)
                    // 2280: argv.offset(i a ... size):   l750 = UNIQUE | NON_NULL, (empty)
                    // 2280: argv: typeof(_437) = *mut {l752} *mut {l753} i8
                    // 2280: argv:   l752 = UNIQUE | NON_NULL, (empty)
                    // 2280: argv:   l753 = UNIQUE | NON_NULL, (empty)
                );
            }
        } else if !inputname.is_null() {
        // 2283: inputname: typeof(_442) = *mut {l759} i8
        // 2283: inputname:   l759 = UNIQUE | NON_NULL, (empty)
        // 2283: inputname: typeof(_443) = *mut {l761} *mut {l762} i8
        // 2283: inputname:   l761 = UNIQUE | NON_NULL, (empty)
        // 2283: inputname:   l762 = UNIQUE | NON_NULL, (empty)
            die(
                b"two files given: '%s' and '%s'\0" as *const u8 as *const libc::c_char,
                // 2285: b"two files giv ... _char: typeof(_445) = *const {l765} i8
                // 2285: b"two files giv ... _char:   l765 = UNIQUE | NON_NULL, (empty)
                // 2285: b"two files giv ... st u8: typeof(_446) = *const {l767} u8
                // 2285: b"two files giv ... st u8:   l767 = UNIQUE | NON_NULL, (empty)
                // 2285: b"two files giv ... s'\0": typeof(_447) = *const {l769} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2285: b"two files giv ... s'\0":   l769 = UNIQUE | NON_NULL, (empty)
                // 2285: b"two files giv ... s'\0": typeof(_448) = & {l771} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2285: b"two files giv ... s'\0":   l771 = UNIQUE | NON_NULL, FIXED
                // 2285: b"two files giv ... s'\0": typeof(_447 = &raw const (*_448)) = *const {l1601} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2285: b"two files giv ... s'\0":   l1601 = UNIQUE | NON_NULL, (empty)
                // 2285: b"two files giv ... _char: typeof(_445 = move _446 as *const i8 (Misc)) = *const {l1603} i8
                // 2285: b"two files giv ... _char:   l1603 = UNIQUE | NON_NULL, (empty)
                // 2285: b"two files giv ... s'\0": typeof(_448 = const b"two files given: \'%s\' and \'%s\'\x00") = & {l1600} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 2285: b"two files giv ... s'\0":   l1600 = UNIQUE | NON_NULL, (empty)
                // 2285: b"two files giv ... st u8: typeof(_446 = move _447 as *const u8 (Pointer(ArrayToPointer))) = *const {l1602} u8
                // 2285: b"two files giv ... st u8:   l1602 = UNIQUE | NON_NULL, (empty)
                inputname,
                // 2286: inputname: typeof(_449) = *mut {l773} i8
                // 2286: inputname:   l773 = UNIQUE | NON_NULL, (empty)
                // 2286: inputname: typeof(_450) = *mut {l775} *mut {l776} i8
                // 2286: inputname:   l775 = UNIQUE | NON_NULL, (empty)
                // 2286: inputname:   l776 = UNIQUE | NON_NULL, (empty)
                *argv.offset(i as isize),
                // 2287: *argv.offset(i  ... size): typeof(_451) = *mut {l778} i8
                // 2287: *argv.offset(i  ... size):   l778 = UNIQUE | NON_NULL, (empty)
                // 2287: argv.offset(i a ... size): typeof(_452) = *mut {l780} *mut {l781} i8
                // 2287: argv.offset(i a ... size):   l780 = UNIQUE | NON_NULL, (empty)
                // 2287: argv.offset(i a ... size):   l781 = UNIQUE | NON_NULL, (empty)
                // 2287: argv: typeof(_453) = *mut {l783} *mut {l784} i8
                // 2287: argv:   l783 = UNIQUE | NON_NULL, (empty)
                // 2287: argv:   l784 = UNIQUE | NON_NULL, (empty)
            );
        } else {
            inputname = *argv.offset(i as isize);
            // 2290: *argv.offset(i  ... size): typeof(_456) = *mut {l788} i8
            // 2290: *argv.offset(i  ... size):   l788 = UNIQUE | NON_NULL, (empty)
            // 2290: argv.offset(i a ... size): typeof(_457) = *mut {l790} *mut {l791} i8
            // 2290: argv.offset(i a ... size):   l790 = UNIQUE | NON_NULL, (empty)
            // 2290: argv.offset(i a ... size):   l791 = UNIQUE | NON_NULL, (empty)
            // 2290: argv: typeof(_458) = *mut {l793} *mut {l794} i8
            // 2290: argv:   l793 = UNIQUE | NON_NULL, (empty)
            // 2290: argv:   l794 = UNIQUE | NON_NULL, (empty)
            // 2290: inputname: typeof(_461) = *mut {l798} *mut {l799} i8
            // 2290: inputname:   l798 = UNIQUE | NON_NULL, (empty)
            // 2290: inputname:   l799 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    if bar != 0 && isatty(1 as libc::c_int) == 0 {
    // 2295: bar: typeof(_471) = *mut {l810} i32
    // 2295: bar:   l810 = UNIQUE | NON_NULL, (empty)
        die(
            b"progress bar requested but <stdout> not connected to terminal\0" as *const u8
            // 2297: b"progress bar  ... _char: typeof(_476) = *const {l816} i8
            // 2297: b"progress bar  ... _char:   l816 = UNIQUE | NON_NULL, (empty)
            // 2297: b"progress bar  ... st u8: typeof(_477) = *const {l818} u8
            // 2297: b"progress bar  ... st u8:   l818 = UNIQUE | NON_NULL, (empty)
            // 2297: b"progress bar  ... al\0": typeof(_478) = *const {l820} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
            // 2297: b"progress bar  ... al\0":   l820 = UNIQUE | NON_NULL, (empty)
            // 2297: b"progress bar  ... al\0": typeof(_479) = & {l822} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
            // 2297: b"progress bar  ... al\0":   l822 = UNIQUE | NON_NULL, FIXED
            // 2297: b"progress bar  ... al\0": typeof(_479 = const b"progress bar requested but <stdout> not connected to terminal\x00") = & {l1604} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
            // 2297: b"progress bar  ... al\0":   l1604 = UNIQUE | NON_NULL, (empty)
            // 2297: b"progress bar  ... _char: typeof(_476 = move _477 as *const i8 (Misc)) = *const {l1607} i8
            // 2297: b"progress bar  ... _char:   l1607 = UNIQUE | NON_NULL, (empty)
            // 2297: b"progress bar  ... al\0": typeof(_478 = &raw const (*_479)) = *const {l1605} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003e)) }]
            // 2297: b"progress bar  ... al\0":   l1605 = UNIQUE | NON_NULL, (empty)
            // 2297: b"progress bar  ... st u8: typeof(_477 = move _478 as *const u8 (Pointer(ArrayToPointer))) = *const {l1606} u8
            // 2297: b"progress bar  ... st u8:   l1606 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
        );
    }
    if verbose >= 2 as libc::c_int && bar != 0 {
    // 2301: verbose: typeof(_484) = *mut {l828} i32
    // 2301: verbose:   l828 = UNIQUE | NON_NULL, (empty)
    // 2301: bar: typeof(_488) = *mut {l833} i32
    // 2301: bar:   l833 = UNIQUE | NON_NULL, (empty)
        die(
            b"verbosity %d > 1 with '-b'\0" as *const u8 as *const libc::c_char,
            // 2303: b"verbosity %d  ... _char: typeof(_490) = *const {l836} i8
            // 2303: b"verbosity %d  ... _char:   l836 = UNIQUE | NON_NULL, (empty)
            // 2303: b"verbosity %d  ... st u8: typeof(_491) = *const {l838} u8
            // 2303: b"verbosity %d  ... st u8:   l838 = UNIQUE | NON_NULL, (empty)
            // 2303: b"verbosity %d  ... b'\0": typeof(_492) = *const {l840} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2303: b"verbosity %d  ... b'\0":   l840 = UNIQUE | NON_NULL, (empty)
            // 2303: b"verbosity %d  ... b'\0": typeof(_493) = & {l842} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2303: b"verbosity %d  ... b'\0":   l842 = UNIQUE | NON_NULL, FIXED
            // 2303: b"verbosity %d  ... b'\0": typeof(_492 = &raw const (*_493)) = *const {l1609} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2303: b"verbosity %d  ... b'\0":   l1609 = UNIQUE | NON_NULL, (empty)
            // 2303: b"verbosity %d  ... st u8: typeof(_491 = move _492 as *const u8 (Pointer(ArrayToPointer))) = *const {l1610} u8
            // 2303: b"verbosity %d  ... st u8:   l1610 = UNIQUE | NON_NULL, (empty)
            // 2303: b"verbosity %d  ... _char: typeof(_490 = move _491 as *const i8 (Misc)) = *const {l1611} i8
            // 2303: b"verbosity %d  ... _char:   l1611 = UNIQUE | NON_NULL, (empty)
            // 2303: b"verbosity %d  ... b'\0": typeof(_493 = const b"verbosity %d > 1 with \'-b\'\x00") = & {l1608} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001b)) }]
            // 2303: b"verbosity %d  ... b'\0":   l1608 = UNIQUE | NON_NULL, (empty)
            verbose,
            // 2304: verbose: typeof(_495) = *mut {l845} i32
            // 2304: verbose:   l845 = UNIQUE | NON_NULL, (empty)
        );
    }
    if !statsfilename.is_null() && {
    // 2307: statsfilename: typeof(_500) = *const {l851} i8
    // 2307: statsfilename:   l851 = UNIQUE | NON_NULL, (empty)
        statsfile = fopen(statsfilename, b"w\0" as *const u8 as *const libc::c_char);
        // 2308: fopen(statsfile ... char): typeof(_502) = *mut {l854} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2308: fopen(statsfile ... char):   l854 = UNIQUE | NON_NULL, (empty)
        // 2308: statsfilename: typeof(_503) = *const {l856} i8
        // 2308: statsfilename:   l856 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0" as *cons ... _char: typeof(_504) = *const {l858} i8
        // 2308: b"w\0" as *cons ... _char:   l858 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0" as *const u8: typeof(_505) = *const {l860} u8
        // 2308: b"w\0" as *const u8:   l860 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0": typeof(_506) = *const {l862} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2308: b"w\0":   l862 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0": typeof(_507) = & {l864} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2308: b"w\0":   l864 = UNIQUE | NON_NULL, FIXED
        // 2308: statsfile: typeof(_508) = *mut {l866} *mut {l867} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2308: statsfile:   l866 = UNIQUE | NON_NULL, (empty)
        // 2308: statsfile:   l867 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0" as *const u8: typeof(_505 = move _506 as *const u8 (Pointer(ArrayToPointer))) = *const {l1614} u8
        // 2308: b"w\0" as *const u8:   l1614 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0": typeof(_507 = const b"w\x00") = & {l1612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2308: b"w\0":   l1612 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0": typeof(_506 = &raw const (*_507)) = *const {l1613} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2308: b"w\0":   l1613 = UNIQUE | NON_NULL, (empty)
        // 2308: b"w\0" as *cons ... _char: typeof(_504 = move _505 as *const i8 (Misc)) = *const {l1615} i8
        // 2308: b"w\0" as *cons ... _char:   l1615 = UNIQUE | NON_NULL, (empty)
        statsfile.is_null()
        // 2309: statsfile: typeof(_509) = *mut {l869} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2309: statsfile:   l869 = UNIQUE | NON_NULL, (empty)
        // 2309: statsfile: typeof(_510) = *mut {l871} *mut {l872} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2309: statsfile:   l871 = UNIQUE | NON_NULL, (empty)
        // 2309: statsfile:   l872 = UNIQUE | NON_NULL, (empty)
    } {
        die(
            b"can not write to stats file '%s'\0" as *const u8 as *const libc::c_char,
            // 2312: b"can not write ... _char: typeof(_512) = *const {l875} i8
            // 2312: b"can not write ... _char:   l875 = UNIQUE | NON_NULL, (empty)
            // 2312: b"can not write ... st u8: typeof(_513) = *const {l877} u8
            // 2312: b"can not write ... st u8:   l877 = UNIQUE | NON_NULL, (empty)
            // 2312: b"can not write ... s'\0": typeof(_514) = *const {l879} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 2312: b"can not write ... s'\0":   l879 = UNIQUE | NON_NULL, (empty)
            // 2312: b"can not write ... s'\0": typeof(_515) = & {l881} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 2312: b"can not write ... s'\0":   l881 = UNIQUE | NON_NULL, FIXED
            // 2312: b"can not write ... _char: typeof(_512 = move _513 as *const i8 (Misc)) = *const {l1619} i8
            // 2312: b"can not write ... _char:   l1619 = UNIQUE | NON_NULL, (empty)
            // 2312: b"can not write ... s'\0": typeof(_515 = const b"can not write to stats file \'%s\'\x00") = & {l1616} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 2312: b"can not write ... s'\0":   l1616 = UNIQUE | NON_NULL, (empty)
            // 2312: b"can not write ... st u8: typeof(_513 = move _514 as *const u8 (Pointer(ArrayToPointer))) = *const {l1618} u8
            // 2312: b"can not write ... st u8:   l1618 = UNIQUE | NON_NULL, (empty)
            // 2312: b"can not write ... s'\0": typeof(_514 = &raw const (*_515)) = *const {l1617} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
            // 2312: b"can not write ... s'\0":   l1617 = UNIQUE | NON_NULL, (empty)
            statsfilename,
            // 2313: statsfilename: typeof(_516) = *const {l883} i8
            // 2313: statsfilename:   l883 = UNIQUE | NON_NULL, (empty)
        );
    }
    if !histfilename.is_null() && {
    // 2316: histfilename: typeof(_521) = *const {l889} i8
    // 2316: histfilename:   l889 = UNIQUE | NON_NULL, (empty)
        histfile = fopen(histfilename, b"w\0" as *const u8 as *const libc::c_char);
        // 2317: fopen(histfilen ... char): typeof(_523) = *mut {l892} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2317: fopen(histfilen ... char):   l892 = UNIQUE | NON_NULL, (empty)
        // 2317: histfilename: typeof(_524) = *const {l894} i8
        // 2317: histfilename:   l894 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0" as *cons ... _char: typeof(_525) = *const {l896} i8
        // 2317: b"w\0" as *cons ... _char:   l896 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0" as *const u8: typeof(_526) = *const {l898} u8
        // 2317: b"w\0" as *const u8:   l898 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0": typeof(_527) = *const {l900} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2317: b"w\0":   l900 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0": typeof(_528) = & {l902} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2317: b"w\0":   l902 = UNIQUE | NON_NULL, FIXED
        // 2317: histfile: typeof(_529) = *mut {l904} *mut {l905} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2317: histfile:   l904 = UNIQUE | NON_NULL, (empty)
        // 2317: histfile:   l905 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0" as *const u8: typeof(_526 = move _527 as *const u8 (Pointer(ArrayToPointer))) = *const {l1622} u8
        // 2317: b"w\0" as *const u8:   l1622 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0" as *cons ... _char: typeof(_525 = move _526 as *const i8 (Misc)) = *const {l1623} i8
        // 2317: b"w\0" as *cons ... _char:   l1623 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0": typeof(_527 = &raw const (*_528)) = *const {l1621} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2317: b"w\0":   l1621 = UNIQUE | NON_NULL, (empty)
        // 2317: b"w\0": typeof(_528 = const b"w\x00") = & {l1620} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2317: b"w\0":   l1620 = UNIQUE | NON_NULL, (empty)
        histfile.is_null()
        // 2318: histfile: typeof(_530) = *mut {l907} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2318: histfile:   l907 = UNIQUE | NON_NULL, (empty)
        // 2318: histfile: typeof(_531) = *mut {l909} *mut {l910} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2318: histfile:   l909 = UNIQUE | NON_NULL, (empty)
        // 2318: histfile:   l910 = UNIQUE | NON_NULL, (empty)
    } {
        die(
            b"can not write to job run time histogram file '%s'\0" as *const u8
            // 2321: b"can not write ... _char: typeof(_533) = *const {l913} i8
            // 2321: b"can not write ... _char:   l913 = UNIQUE | NON_NULL, (empty)
            // 2321: b"can not write ... st u8: typeof(_534) = *const {l915} u8
            // 2321: b"can not write ... st u8:   l915 = UNIQUE | NON_NULL, (empty)
            // 2321: b"can not write ... s'\0": typeof(_535) = *const {l917} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2321: b"can not write ... s'\0":   l917 = UNIQUE | NON_NULL, (empty)
            // 2321: b"can not write ... s'\0": typeof(_536) = & {l919} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2321: b"can not write ... s'\0":   l919 = UNIQUE | NON_NULL, FIXED
            // 2321: b"can not write ... s'\0": typeof(_535 = &raw const (*_536)) = *const {l1625} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2321: b"can not write ... s'\0":   l1625 = UNIQUE | NON_NULL, (empty)
            // 2321: b"can not write ... s'\0": typeof(_536 = const b"can not write to job run time histogram file \'%s\'\x00") = & {l1624} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000032)) }]
            // 2321: b"can not write ... s'\0":   l1624 = UNIQUE | NON_NULL, (empty)
            // 2321: b"can not write ... st u8: typeof(_534 = move _535 as *const u8 (Pointer(ArrayToPointer))) = *const {l1626} u8
            // 2321: b"can not write ... st u8:   l1626 = UNIQUE | NON_NULL, (empty)
            // 2321: b"can not write ... _char: typeof(_533 = move _534 as *const i8 (Misc)) = *const {l1627} i8
            // 2321: b"can not write ... _char:   l1627 = UNIQUE | NON_NULL, (empty)
                as *const libc::c_char,
            histfilename,
            // 2323: histfilename: typeof(_537) = *const {l921} i8
            // 2323: histfilename:   l921 = UNIQUE | NON_NULL, (empty)
        );
    }
    if verbose != 0 && statsfile.is_null() {
    // 2326: verbose: typeof(_542) = *mut {l927} i32
    // 2326: verbose:   l927 = UNIQUE | NON_NULL, (empty)
    // 2326: statsfile: typeof(_544) = *mut {l930} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2326: statsfile:   l930 = UNIQUE | NON_NULL, (empty)
    // 2326: statsfile: typeof(_545) = *mut {l932} *mut {l933} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2326: statsfile:   l932 = UNIQUE | NON_NULL, (empty)
    // 2326: statsfile:   l933 = UNIQUE | NON_NULL, (empty)
        statsfile = stdout;
        // 2327: stdout: typeof(_546) = *mut {l935} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2327: stdout:   l935 = UNIQUE | NON_NULL, (empty)
        // 2327: stdout: typeof(_547) = *mut {l937} *mut {l938} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2327: stdout:   l937 = UNIQUE | NON_NULL, (empty)
        // 2327: stdout:   l938 = UNIQUE | NON_NULL, (empty)
        // 2327: statsfile: typeof(_548) = *mut {l940} *mut {l941} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2327: statsfile:   l940 = UNIQUE | NON_NULL, (empty)
        // 2327: statsfile:   l941 = UNIQUE | NON_NULL, (empty)
    }
    if verbose != 0 {
    // 2329: verbose: typeof(_552) = *mut {l946} i32
    // 2329: verbose:   l946 = UNIQUE | NON_NULL, (empty)
        lglbnr(
            b"iLingeling Incremental Parallel Lingeling\0" as *const u8 as *const libc::c_char,
            // 2331: b"iLingeling In ... _char: typeof(_554) = *const {l949} i8
            // 2331: b"iLingeling In ... _char:   l949 = UNIQUE | NON_NULL, (empty)
            // 2331: b"iLingeling In ... st u8: typeof(_555) = *const {l951} u8
            // 2331: b"iLingeling In ... st u8:   l951 = UNIQUE | NON_NULL, (empty)
            // 2331: b"iLingeling In ... ng\0": typeof(_556) = *const {l953} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 2331: b"iLingeling In ... ng\0":   l953 = UNIQUE | NON_NULL, (empty)
            // 2331: b"iLingeling In ... ng\0": typeof(_557) = & {l955} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 2331: b"iLingeling In ... ng\0":   l955 = UNIQUE | NON_NULL, FIXED
            // 2331: b"iLingeling In ... ng\0": typeof(_557 = const b"iLingeling Incremental Parallel Lingeling\x00") = & {l1628} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 2331: b"iLingeling In ... ng\0":   l1628 = UNIQUE | NON_NULL, (empty)
            // 2331: b"iLingeling In ... _char: typeof(_554 = move _555 as *const i8 (Misc)) = *const {l1631} i8
            // 2331: b"iLingeling In ... _char:   l1631 = UNIQUE | NON_NULL, (empty)
            // 2331: b"iLingeling In ... ng\0": typeof(_556 = &raw const (*_557)) = *const {l1629} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 2331: b"iLingeling In ... ng\0":   l1629 = UNIQUE | NON_NULL, (empty)
            // 2331: b"iLingeling In ... st u8: typeof(_555 = move _556 as *const u8 (Pointer(ArrayToPointer))) = *const {l1630} u8
            // 2331: b"iLingeling In ... st u8:   l1630 = UNIQUE | NON_NULL, (empty)
            b"c \0" as *const u8 as *const libc::c_char,
            // 2332: b"c \0" as *con ... _char: typeof(_558) = *const {l957} i8
            // 2332: b"c \0" as *con ... _char:   l957 = UNIQUE | NON_NULL, (empty)
            // 2332: b"c \0" as *const u8: typeof(_559) = *const {l959} u8
            // 2332: b"c \0" as *const u8:   l959 = UNIQUE | NON_NULL, (empty)
            // 2332: b"c \0": typeof(_560) = *const {l961} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2332: b"c \0":   l961 = UNIQUE | NON_NULL, (empty)
            // 2332: b"c \0": typeof(_561) = & {l963} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2332: b"c \0":   l963 = UNIQUE | NON_NULL, FIXED
            // 2332: b"c \0": typeof(_560 = &raw const (*_561)) = *const {l1633} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2332: b"c \0":   l1633 = UNIQUE | NON_NULL, (empty)
            // 2332: b"c \0" as *con ... _char: typeof(_558 = move _559 as *const i8 (Misc)) = *const {l1635} i8
            // 2332: b"c \0" as *con ... _char:   l1635 = UNIQUE | NON_NULL, (empty)
            // 2332: b"c \0": typeof(_561 = const b"c \x00") = & {l1632} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2332: b"c \0":   l1632 = UNIQUE | NON_NULL, (empty)
            // 2332: b"c \0" as *const u8: typeof(_559 = move _560 as *const u8 (Pointer(ArrayToPointer))) = *const {l1634} u8
            // 2332: b"c \0" as *const u8:   l1634 = UNIQUE | NON_NULL, (empty)
            stdout,
            // 2333: stdout: typeof(_562) = *mut {l965} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 2333: stdout:   l965 = UNIQUE | NON_NULL, (empty)
            // 2333: stdout: typeof(_563) = *mut {l967} *mut {l968} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 2333: stdout:   l967 = UNIQUE | NON_NULL, (empty)
            // 2333: stdout:   l968 = UNIQUE | NON_NULL, (empty)
        );
        printf(b"c\n\0" as *const u8 as *const libc::c_char);
        // 2335: b"c\n\0" as *co ... _char: typeof(_565) = *const {l971} i8
        // 2335: b"c\n\0" as *co ... _char:   l971 = UNIQUE | NON_NULL, (empty)
        // 2335: b"c\n\0" as *co ... st u8: typeof(_566) = *const {l973} u8
        // 2335: b"c\n\0" as *co ... st u8:   l973 = UNIQUE | NON_NULL, (empty)
        // 2335: b"c\n\0": typeof(_567) = *const {l975} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2335: b"c\n\0":   l975 = UNIQUE | NON_NULL, (empty)
        // 2335: b"c\n\0": typeof(_568) = & {l977} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2335: b"c\n\0":   l977 = UNIQUE | NON_NULL, FIXED
        // 2335: b"c\n\0": typeof(_568 = const b"c\n\x00") = & {l1636} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2335: b"c\n\0":   l1636 = UNIQUE | NON_NULL, (empty)
        // 2335: b"c\n\0": typeof(_567 = &raw const (*_568)) = *const {l1637} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
        // 2335: b"c\n\0":   l1637 = UNIQUE | NON_NULL, (empty)
        // 2335: b"c\n\0" as *co ... st u8: typeof(_566 = move _567 as *const u8 (Pointer(ArrayToPointer))) = *const {l1638} u8
        // 2335: b"c\n\0" as *co ... st u8:   l1638 = UNIQUE | NON_NULL, (empty)
        // 2335: b"c\n\0" as *co ... _char: typeof(_565 = move _566 as *const i8 (Misc)) = *const {l1639} i8
        // 2335: b"c\n\0" as *co ... _char:   l1639 = UNIQUE | NON_NULL, (empty)
        fflush(stdout);
        // 2336: stdout: typeof(_570) = *mut {l980} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2336: stdout:   l980 = UNIQUE | NON_NULL, (empty)
        // 2336: stdout: typeof(_571) = *mut {l982} *mut {l983} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2336: stdout:   l982 = UNIQUE | NON_NULL, (empty)
        // 2336: stdout:   l983 = UNIQUE | NON_NULL, (empty)
    }
    if nworkers == 0 {
    // 2338: nworkers: typeof(_575) = *mut {l988} i32
    // 2338: nworkers:   l988 = UNIQUE | NON_NULL, (empty)
        nworkers = 1 as libc::c_int;
        // 2339: nworkers: typeof(_577) = *mut {l991} i32
        // 2339: nworkers:   l991 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        0 as *mut Worker,
        // 2342: 0 as *mut Worker: typeof(_579) = *mut {l994} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2342: 0 as *mut Worker:   l994 = UNIQUE | NON_NULL, (empty)
        // 2342: 0 as *mut Worker: typeof(_579 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1640} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2342: 0 as *mut Worker:   l1640 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"using %d workers\0" as *const u8 as *const libc::c_char,
        // 2344: b"using %d work ... _char: typeof(_581) = *const {l997} i8
        // 2344: b"using %d work ... _char:   l997 = UNIQUE | NON_NULL, (empty)
        // 2344: b"using %d work ... st u8: typeof(_582) = *const {l999} u8
        // 2344: b"using %d work ... st u8:   l999 = UNIQUE | NON_NULL, (empty)
        // 2344: b"using %d work ... rs\0": typeof(_583) = *const {l1001} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2344: b"using %d work ... rs\0":   l1001 = UNIQUE | NON_NULL, (empty)
        // 2344: b"using %d work ... rs\0": typeof(_584) = & {l1003} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2344: b"using %d work ... rs\0":   l1003 = UNIQUE | NON_NULL, FIXED
        // 2344: b"using %d work ... rs\0": typeof(_583 = &raw const (*_584)) = *const {l1642} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2344: b"using %d work ... rs\0":   l1642 = UNIQUE | NON_NULL, (empty)
        // 2344: b"using %d work ... st u8: typeof(_582 = move _583 as *const u8 (Pointer(ArrayToPointer))) = *const {l1643} u8
        // 2344: b"using %d work ... st u8:   l1643 = UNIQUE | NON_NULL, (empty)
        // 2344: b"using %d work ... _char: typeof(_581 = move _582 as *const i8 (Misc)) = *const {l1644} i8
        // 2344: b"using %d work ... _char:   l1644 = UNIQUE | NON_NULL, (empty)
        // 2344: b"using %d work ... rs\0": typeof(_584 = const b"using %d workers\x00") = & {l1641} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2344: b"using %d work ... rs\0":   l1641 = UNIQUE | NON_NULL, (empty)
        nworkers,
        // 2345: nworkers: typeof(_586) = *mut {l1006} i32
        // 2345: nworkers:   l1006 = UNIQUE | NON_NULL, (empty)
    );
    if !inputname.is_null() {
    // 2347: inputname: typeof(_590) = *mut {l1011} i8
    // 2347: inputname:   l1011 = UNIQUE | NON_NULL, (empty)
    // 2347: inputname: typeof(_591) = *mut {l1013} *mut {l1014} i8
    // 2347: inputname:   l1013 = UNIQUE | NON_NULL, (empty)
    // 2347: inputname:   l1014 = UNIQUE | NON_NULL, (empty)
        inputfile = fopen(inputname, b"r\0" as *const u8 as *const libc::c_char);
        // 2348: fopen(inputname ... char): typeof(_592) = *mut {l1016} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2348: fopen(inputname ... char):   l1016 = UNIQUE | NON_NULL, (empty)
        // 2348: inputname: typeof(_593) = *const {l1018} i8
        // 2348: inputname:   l1018 = UNIQUE | NON_NULL, (empty)
        // 2348: inputname: typeof(_594) = *mut {l1020} i8
        // 2348: inputname:   l1020 = UNIQUE | NON_NULL, (empty)
        // 2348: inputname: typeof(_595) = *mut {l1022} *mut {l1023} i8
        // 2348: inputname:   l1022 = UNIQUE | NON_NULL, (empty)
        // 2348: inputname:   l1023 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0" as *cons ... _char: typeof(_596) = *const {l1025} i8
        // 2348: b"r\0" as *cons ... _char:   l1025 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0" as *const u8: typeof(_597) = *const {l1027} u8
        // 2348: b"r\0" as *const u8:   l1027 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0": typeof(_598) = *const {l1029} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2348: b"r\0":   l1029 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0": typeof(_599) = & {l1031} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2348: b"r\0":   l1031 = UNIQUE | NON_NULL, FIXED
        // 2348: inputfile: typeof(_600) = *mut {l1033} *mut {l1034} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2348: inputfile:   l1033 = UNIQUE | NON_NULL, (empty)
        // 2348: inputfile:   l1034 = UNIQUE | NON_NULL, (empty)
        // 2348: inputname: typeof(_593 = move _594 as *const i8 (Pointer(MutToConstPointer))) = *const {l1645} i8
        // 2348: inputname:   l1645 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0" as *const u8: typeof(_597 = move _598 as *const u8 (Pointer(ArrayToPointer))) = *const {l1648} u8
        // 2348: b"r\0" as *const u8:   l1648 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0": typeof(_598 = &raw const (*_599)) = *const {l1647} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2348: b"r\0":   l1647 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0": typeof(_599 = const b"r\x00") = & {l1646} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2348: b"r\0":   l1646 = UNIQUE | NON_NULL, (empty)
        // 2348: b"r\0" as *cons ... _char: typeof(_596 = move _597 as *const i8 (Misc)) = *const {l1649} i8
        // 2348: b"r\0" as *cons ... _char:   l1649 = UNIQUE | NON_NULL, (empty)
        if inputfile.is_null() {
        // 2349: inputfile: typeof(_603) = *mut {l1038} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2349: inputfile:   l1038 = UNIQUE | NON_NULL, (empty)
        // 2349: inputfile: typeof(_604) = *mut {l1040} *mut {l1041} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2349: inputfile:   l1040 = UNIQUE | NON_NULL, (empty)
        // 2349: inputfile:   l1041 = UNIQUE | NON_NULL, (empty)
            die(
                b"can not read '%s'\0" as *const u8 as *const libc::c_char,
                // 2351: b"can not read  ... _char: typeof(_606) = *const {l1044} i8
                // 2351: b"can not read  ... _char:   l1044 = UNIQUE | NON_NULL, (empty)
                // 2351: b"can not read  ... st u8: typeof(_607) = *const {l1046} u8
                // 2351: b"can not read  ... st u8:   l1046 = UNIQUE | NON_NULL, (empty)
                // 2351: b"can not read  ... s'\0": typeof(_608) = *const {l1048} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 2351: b"can not read  ... s'\0":   l1048 = UNIQUE | NON_NULL, (empty)
                // 2351: b"can not read  ... s'\0": typeof(_609) = & {l1050} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 2351: b"can not read  ... s'\0":   l1050 = UNIQUE | NON_NULL, FIXED
                // 2351: b"can not read  ... st u8: typeof(_607 = move _608 as *const u8 (Pointer(ArrayToPointer))) = *const {l1652} u8
                // 2351: b"can not read  ... st u8:   l1652 = UNIQUE | NON_NULL, (empty)
                // 2351: b"can not read  ... _char: typeof(_606 = move _607 as *const i8 (Misc)) = *const {l1653} i8
                // 2351: b"can not read  ... _char:   l1653 = UNIQUE | NON_NULL, (empty)
                // 2351: b"can not read  ... s'\0": typeof(_609 = const b"can not read \'%s\'\x00") = & {l1650} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 2351: b"can not read  ... s'\0":   l1650 = UNIQUE | NON_NULL, (empty)
                // 2351: b"can not read  ... s'\0": typeof(_608 = &raw const (*_609)) = *const {l1651} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 2351: b"can not read  ... s'\0":   l1651 = UNIQUE | NON_NULL, (empty)
                inputname,
                // 2352: inputname: typeof(_610) = *mut {l1052} i8
                // 2352: inputname:   l1052 = UNIQUE | NON_NULL, (empty)
                // 2352: inputname: typeof(_611) = *mut {l1054} *mut {l1055} i8
                // 2352: inputname:   l1054 = UNIQUE | NON_NULL, (empty)
                // 2352: inputname:   l1055 = UNIQUE | NON_NULL, (empty)
            );
        }
        closeinputfile = 1 as libc::c_int;
    } else {
        inputname = b"<stdin>\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        // 2357: b"<stdin>\0" as ... _char: typeof(_613) = *const {l1058} i8
        // 2357: b"<stdin>\0" as ... _char:   l1058 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0" as ... st u8: typeof(_614) = *const {l1060} u8
        // 2357: b"<stdin>\0" as ... st u8:   l1060 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0": typeof(_615) = *const {l1062} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2357: b"<stdin>\0":   l1062 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0": typeof(_616) = & {l1064} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2357: b"<stdin>\0":   l1064 = UNIQUE | NON_NULL, FIXED
        // 2357: inputname: typeof(_617) = *mut {l1066} *mut {l1067} i8
        // 2357: inputname:   l1066 = UNIQUE | NON_NULL, (empty)
        // 2357: inputname:   l1067 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0": typeof(_616 = const b"<stdin>\x00") = & {l1654} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2357: b"<stdin>\0":   l1654 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0" as ... _char: typeof(_613 = move _614 as *const i8 (Misc)) = *const {l1657} i8
        // 2357: b"<stdin>\0" as ... _char:   l1657 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0": typeof(_615 = &raw const (*_616)) = *const {l1655} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
        // 2357: b"<stdin>\0":   l1655 = UNIQUE | NON_NULL, (empty)
        // 2357: b"<stdin>\0" as ... st u8: typeof(_614 = move _615 as *const u8 (Pointer(ArrayToPointer))) = *const {l1656} u8
        // 2357: b"<stdin>\0" as ... st u8:   l1656 = UNIQUE | NON_NULL, (empty)
        // 2357: inputname = b"< ... _char: typeof((*_617) = move _613 as *mut i8 (Misc)) = *mut {l1658} i8
        // 2357: inputname = b"< ... _char:   l1658 = UNIQUE | NON_NULL, (empty)
        inputfile = stdin;
        // 2358: stdin: typeof(_618) = *mut {l1069} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2358: stdin:   l1069 = UNIQUE | NON_NULL, (empty)
        // 2358: stdin: typeof(_619) = *mut {l1071} *mut {l1072} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2358: stdin:   l1071 = UNIQUE | NON_NULL, (empty)
        // 2358: stdin:   l1072 = UNIQUE | NON_NULL, (empty)
        // 2358: inputfile: typeof(_620) = *mut {l1074} *mut {l1075} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2358: inputfile:   l1074 = UNIQUE | NON_NULL, (empty)
        // 2358: inputfile:   l1075 = UNIQUE | NON_NULL, (empty)
        closeinputfile = 0 as libc::c_int;
    }
    init();
    setsighandlers();
    msg(
        0 as *mut Worker,
        // 2364: 0 as *mut Worker: typeof(_625) = *mut {l1081} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2364: 0 as *mut Worker:   l1081 = UNIQUE | NON_NULL, (empty)
        // 2364: 0 as *mut Worker: typeof(_625 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1659} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2364: 0 as *mut Worker:   l1659 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"parsing %s\0" as *const u8 as *const libc::c_char,
        // 2366: b"parsing %s\0" ... _char: typeof(_627) = *const {l1084} i8
        // 2366: b"parsing %s\0" ... _char:   l1084 = UNIQUE | NON_NULL, (empty)
        // 2366: b"parsing %s\0" ... st u8: typeof(_628) = *const {l1086} u8
        // 2366: b"parsing %s\0" ... st u8:   l1086 = UNIQUE | NON_NULL, (empty)
        // 2366: b"parsing %s\0": typeof(_629) = *const {l1088} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2366: b"parsing %s\0":   l1088 = UNIQUE | NON_NULL, (empty)
        // 2366: b"parsing %s\0": typeof(_630) = & {l1090} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2366: b"parsing %s\0":   l1090 = UNIQUE | NON_NULL, FIXED
        // 2366: b"parsing %s\0": typeof(_629 = &raw const (*_630)) = *const {l1661} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2366: b"parsing %s\0":   l1661 = UNIQUE | NON_NULL, (empty)
        // 2366: b"parsing %s\0" ... st u8: typeof(_628 = move _629 as *const u8 (Pointer(ArrayToPointer))) = *const {l1662} u8
        // 2366: b"parsing %s\0" ... st u8:   l1662 = UNIQUE | NON_NULL, (empty)
        // 2366: b"parsing %s\0" ... _char: typeof(_627 = move _628 as *const i8 (Misc)) = *const {l1663} i8
        // 2366: b"parsing %s\0" ... _char:   l1663 = UNIQUE | NON_NULL, (empty)
        // 2366: b"parsing %s\0": typeof(_630 = const b"parsing %s\x00") = & {l1660} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2366: b"parsing %s\0":   l1660 = UNIQUE | NON_NULL, (empty)
        inputname,
        // 2367: inputname: typeof(_631) = *mut {l1092} i8
        // 2367: inputname:   l1092 = UNIQUE | NON_NULL, (empty)
        // 2367: inputname: typeof(_632) = *mut {l1094} *mut {l1095} i8
        // 2367: inputname:   l1094 = UNIQUE | NON_NULL, (empty)
        // 2367: inputname:   l1095 = UNIQUE | NON_NULL, (empty)
    );
    parse();
    if closeinputfile != 0 {
        fclose(inputfile);
        // 2371: inputfile: typeof(_638) = *mut {l1102} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2371: inputfile:   l1102 = UNIQUE | NON_NULL, (empty)
        // 2371: inputfile: typeof(_639) = *mut {l1104} *mut {l1105} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2371: inputfile:   l1104 = UNIQUE | NON_NULL, (empty)
        // 2371: inputfile:   l1105 = UNIQUE | NON_NULL, (empty)
    }
    msg(
        0 as *mut Worker,
        // 2374: 0 as *mut Worker: typeof(_641) = *mut {l1108} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2374: 0 as *mut Worker:   l1108 = UNIQUE | NON_NULL, (empty)
        // 2374: 0 as *mut Worker: typeof(_641 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1664} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2374: 0 as *mut Worker:   l1664 = UNIQUE | NON_NULL, (empty)
        1 as libc::c_int,
        b"%d variables out of %d used in assumptions which is %.0f%%\0" as *const u8
        // 2376: b"%d variables  ... _char: typeof(_643) = *const {l1111} i8
        // 2376: b"%d variables  ... _char:   l1111 = UNIQUE | NON_NULL, (empty)
        // 2376: b"%d variables  ... st u8: typeof(_644) = *const {l1113} u8
        // 2376: b"%d variables  ... st u8:   l1113 = UNIQUE | NON_NULL, (empty)
        // 2376: b"%d variables  ... %%\0": typeof(_645) = *const {l1115} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
        // 2376: b"%d variables  ... %%\0":   l1115 = UNIQUE | NON_NULL, (empty)
        // 2376: b"%d variables  ... %%\0": typeof(_646) = & {l1117} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
        // 2376: b"%d variables  ... %%\0":   l1117 = UNIQUE | NON_NULL, FIXED
        // 2376: b"%d variables  ... _char: typeof(_643 = move _644 as *const i8 (Misc)) = *const {l1668} i8
        // 2376: b"%d variables  ... _char:   l1668 = UNIQUE | NON_NULL, (empty)
        // 2376: b"%d variables  ... %%\0": typeof(_645 = &raw const (*_646)) = *const {l1666} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
        // 2376: b"%d variables  ... %%\0":   l1666 = UNIQUE | NON_NULL, (empty)
        // 2376: b"%d variables  ... st u8: typeof(_644 = move _645 as *const u8 (Pointer(ArrayToPointer))) = *const {l1667} u8
        // 2376: b"%d variables  ... st u8:   l1667 = UNIQUE | NON_NULL, (empty)
        // 2376: b"%d variables  ... %%\0": typeof(_646 = const b"%d variables out of %d used in assumptions which is %.0f%%\x00") = & {l1665} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003b)) }]
        // 2376: b"%d variables  ... %%\0":   l1665 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        nused,
        // 2378: nused: typeof(_648) = *mut {l1120} i32
        // 2378: nused:   l1120 = UNIQUE | NON_NULL, (empty)
        nvars,
        // 2379: nvars: typeof(_650) = *mut {l1123} i32
        // 2379: nvars:   l1123 = UNIQUE | NON_NULL, (empty)
        if nvars != 0 {
        // 2380: nvars: typeof(_654) = *mut {l1128} i32
        // 2380: nvars:   l1128 = UNIQUE | NON_NULL, (empty)
            100.0f64 * (nused as libc::c_double / nvars as libc::c_double)
            // 2381: nused: typeof(_658) = *mut {l1133} i32
            // 2381: nused:   l1133 = UNIQUE | NON_NULL, (empty)
            // 2381: nvars: typeof(_661) = *mut {l1137} i32
            // 2381: nvars:   l1137 = UNIQUE | NON_NULL, (empty)
        } else {
            0.0f64
        },
    );
    freeze();
    start();
    stop();
    winner = 0 as *mut Worker;
    // 2389: winner = 0 as * ... orker: typeof(_9 = const 0_usize as *mut Worker (PointerFromExposedAddress)) = *mut {l1669} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2389: winner = 0 as * ... orker:   l1669 = UNIQUE | NON_NULL, (empty)
    w = workers;
    // 2390: workers: typeof(_665) = *mut {l1142} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2390: workers:   l1142 = UNIQUE | NON_NULL, (empty)
    // 2390: workers: typeof(_666) = *mut {l1144} *mut {l1145} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2390: workers:   l1144 = UNIQUE | NON_NULL, (empty)
    // 2390: workers:   l1145 = UNIQUE | NON_NULL, (empty)
    while w < workers.offset(nworkers as isize) {
    // 2391: w: typeof(_669) = *mut {l1149} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2391: w:   l1149 = UNIQUE | NON_NULL, (empty)
    // 2391: workers.offset( ... size): typeof(_670) = *mut {l1151} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2391: workers.offset( ... size):   l1151 = UNIQUE | NON_NULL, (empty)
    // 2391: workers: typeof(_671) = *mut {l1153} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2391: workers:   l1153 = UNIQUE | NON_NULL, (empty)
    // 2391: workers: typeof(_672) = *mut {l1155} *mut {l1156} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2391: workers:   l1155 = UNIQUE | NON_NULL, (empty)
    // 2391: workers:   l1156 = UNIQUE | NON_NULL, (empty)
    // 2391: nworkers: typeof(_675) = *mut {l1160} i32
    // 2391: nworkers:   l1160 = UNIQUE | NON_NULL, (empty)
        if (*w).res != 0 {
            winner = w;
            // 2393: w: typeof(_679) = *mut {l1165} DefId(0:297 ~ ilingeling[c969]::Worker)
            // 2393: w:   l1165 = UNIQUE | NON_NULL, (empty)
            if (*w).res == 10 as libc::c_int {
                break;
            }
        }
        w = w.offset(1);
        // 2398: w.offset(1): typeof(_684) = *mut {l1171} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2398: w.offset(1):   l1171 = UNIQUE | NON_NULL, (empty)
        // 2398: w: typeof(_685) = *mut {l1173} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2398: w:   l1173 = UNIQUE | NON_NULL, (empty)
        w;
        // 2399: w: typeof(_686) = *mut {l1175} DefId(0:297 ~ ilingeling[c969]::Worker)
        // 2399: w:   l1175 = UNIQUE | NON_NULL, (empty)
    }
    if !winner.is_null()
    // 2401: winner: typeof(_695) = *mut {l1185} DefId(0:297 ~ ilingeling[c969]::Worker)
    // 2401: winner:   l1185 = UNIQUE | NON_NULL, (empty)
        && {
            res = (*winner).res;
            res == 10 as libc::c_int
        }
        && nowitness == 0
        // 2406: nowitness: typeof(_702) = *mut {l1193} i32
        // 2406: nowitness:   l1193 = UNIQUE | NON_NULL, (empty)
    {
        let mut BYTES: size_t = (nvars as libc::c_ulong)
        // 2408: nvars: typeof(_706) = *mut {l1198} i32
        // 2408: nvars:   l1198 = UNIQUE | NON_NULL, (empty)
            .wrapping_mul(::core::mem::size_of::<libc::c_schar>() as libc::c_ulong);
        vals = malloc(BYTES) as *mut libc::c_schar;
        // 2410: malloc(BYTES): typeof(_709) = *mut {l1202} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2410: malloc(BYTES):   l1202 = UNIQUE | NON_NULL, (empty)
        // 2410: vals: typeof(_711) = *mut {l1205} *mut {l1206} i8
        // 2410: vals:   l1205 = UNIQUE | NON_NULL, (empty)
        // 2410: vals:   l1206 = UNIQUE | NON_NULL, (empty)
        // 2410: vals = malloc(B ... schar: typeof((*_711) = move _709 as *mut i8 (Misc)) = *mut {l1670} i8
        // 2410: vals = malloc(B ... schar:   l1670 = UNIQUE | NON_NULL, (empty)
        if vals.is_null() {
        // 2411: vals: typeof(_714) = *mut {l1210} i8
        // 2411: vals:   l1210 = UNIQUE | NON_NULL, (empty)
        // 2411: vals: typeof(_715) = *mut {l1212} *mut {l1213} i8
        // 2411: vals:   l1212 = UNIQUE | NON_NULL, (empty)
        // 2411: vals:   l1213 = UNIQUE | NON_NULL, (empty)
            die(b"out of memory\0" as *const u8 as *const libc::c_char);
            // 2412: b"out of memory ... _char: typeof(_718) = *const {l1217} i8
            // 2412: b"out of memory ... _char:   l1217 = UNIQUE | NON_NULL, (empty)
            // 2412: b"out of memory ... st u8: typeof(_719) = *const {l1219} u8
            // 2412: b"out of memory ... st u8:   l1219 = UNIQUE | NON_NULL, (empty)
            // 2412: b"out of memory\0": typeof(_720) = *const {l1221} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 2412: b"out of memory\0":   l1221 = UNIQUE | NON_NULL, (empty)
            // 2412: b"out of memory\0": typeof(_721) = & {l1223} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 2412: b"out of memory\0":   l1223 = UNIQUE | NON_NULL, FIXED
            // 2412: b"out of memory ... st u8: typeof(_719 = move _720 as *const u8 (Pointer(ArrayToPointer))) = *const {l1673} u8
            // 2412: b"out of memory ... st u8:   l1673 = UNIQUE | NON_NULL, (empty)
            // 2412: b"out of memory\0": typeof(_720 = &raw const (*_721)) = *const {l1672} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 2412: b"out of memory\0":   l1672 = UNIQUE | NON_NULL, (empty)
            // 2412: b"out of memory\0": typeof(_721 = const b"out of memory\x00") = & {l1671} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
            // 2412: b"out of memory\0":   l1671 = UNIQUE | NON_NULL, (empty)
            // 2412: b"out of memory ... _char: typeof(_718 = move _719 as *const i8 (Misc)) = *const {l1674} i8
            // 2412: b"out of memory ... _char:   l1674 = UNIQUE | NON_NULL, (empty)
            exit(1 as libc::c_int);
        }
        memset(vals as *mut libc::c_void, 0 as libc::c_int, BYTES);
        // 2415: memset(vals as  ... YTES): typeof(_724) = *mut {l1227} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2415: memset(vals as  ... YTES):   l1227 = UNIQUE | NON_NULL, (empty)
        // 2415: vals as *mut li ... _void: typeof(_725) = *mut {l1229} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2415: vals as *mut li ... _void:   l1229 = UNIQUE | NON_NULL, (empty)
        // 2415: vals: typeof(_726) = *mut {l1231} i8
        // 2415: vals:   l1231 = UNIQUE | NON_NULL, (empty)
        // 2415: vals: typeof(_727) = *mut {l1233} *mut {l1234} i8
        // 2415: vals:   l1233 = UNIQUE | NON_NULL, (empty)
        // 2415: vals:   l1234 = UNIQUE | NON_NULL, (empty)
        // 2415: vals as *mut li ... _void: typeof(_725 = move _726 as *mut libc::c_void (Misc)) = *mut {l1675} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2415: vals as *mut li ... _void:   l1675 = UNIQUE | NON_NULL, (empty)
        allocated = allocated.wrapping_add(BYTES);
        // 2416: allocated: typeof(_732) = *mut {l1240} u64
        // 2416: allocated:   l1240 = UNIQUE | NON_NULL, (empty)
        // 2416: allocated: typeof(_734) = *mut {l1243} u64
        // 2416: allocated:   l1243 = UNIQUE | NON_NULL, (empty)
        if allocated > maxallocated {
        // 2417: allocated: typeof(_738) = *mut {l1248} u64
        // 2417: allocated:   l1248 = UNIQUE | NON_NULL, (empty)
        // 2417: maxallocated: typeof(_740) = *mut {l1251} u64
        // 2417: maxallocated:   l1251 = UNIQUE | NON_NULL, (empty)
            maxallocated = allocated;
            // 2418: allocated: typeof(_742) = *mut {l1254} u64
            // 2418: allocated:   l1254 = UNIQUE | NON_NULL, (empty)
            // 2418: maxallocated: typeof(_743) = *mut {l1256} u64
            // 2418: maxallocated:   l1256 = UNIQUE | NON_NULL, (empty)
        }
        i = 1 as libc::c_int;
        while i < nvars {
        // 2421: nvars: typeof(_748) = *mut {l1262} i32
        // 2421: nvars:   l1262 = UNIQUE | NON_NULL, (empty)
            *vals.offset(i as isize) = lglderef((*winner).lgl, i) as libc::c_schar;
            // 2422: (*winner).lgl: typeof(_750) = *mut {l1265} LGL
            // 2422: (*winner).lgl:   l1265 = UNIQUE | NON_NULL, (empty)
            // 2422: vals.offset(i a ... size): typeof(_752) = *mut {l1268} i8
            // 2422: vals.offset(i a ... size):   l1268 = UNIQUE | NON_NULL, (empty)
            // 2422: vals: typeof(_753) = *mut {l1270} i8
            // 2422: vals:   l1270 = UNIQUE | NON_NULL, (empty)
            // 2422: vals: typeof(_754) = *mut {l1272} *mut {l1273} i8
            // 2422: vals:   l1272 = UNIQUE | NON_NULL, (empty)
            // 2422: vals:   l1273 = UNIQUE | NON_NULL, (empty)
            i += 1;
            i;
        }
    }
    resetsighandlers();
    if !statsfile.is_null() {
    // 2428: statsfile: typeof(_766) = *mut {l1286} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2428: statsfile:   l1286 = UNIQUE | NON_NULL, (empty)
    // 2428: statsfile: typeof(_767) = *mut {l1288} *mut {l1289} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2428: statsfile:   l1288 = UNIQUE | NON_NULL, (empty)
    // 2428: statsfile:   l1289 = UNIQUE | NON_NULL, (empty)
        stats();
    }
    if !statsfilename.is_null() {
    // 2431: statsfilename: typeof(_772) = *const {l1295} i8
    // 2431: statsfilename:   l1295 = UNIQUE | NON_NULL, (empty)
        fclose(statsfile);
        // 2432: statsfile: typeof(_774) = *mut {l1298} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2432: statsfile:   l1298 = UNIQUE | NON_NULL, (empty)
        // 2432: statsfile: typeof(_775) = *mut {l1300} *mut {l1301} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2432: statsfile:   l1300 = UNIQUE | NON_NULL, (empty)
        // 2432: statsfile:   l1301 = UNIQUE | NON_NULL, (empty)
    }
    if !histfile.is_null() {
    // 2434: histfile: typeof(_779) = *mut {l1306} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2434: histfile:   l1306 = UNIQUE | NON_NULL, (empty)
    // 2434: histfile: typeof(_780) = *mut {l1308} *mut {l1309} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2434: histfile:   l1308 = UNIQUE | NON_NULL, (empty)
    // 2434: histfile:   l1309 = UNIQUE | NON_NULL, (empty)
        hist();
    }
    if !histfile.is_null() {
    // 2437: histfile: typeof(_785) = *mut {l1315} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2437: histfile:   l1315 = UNIQUE | NON_NULL, (empty)
    // 2437: histfile: typeof(_786) = *mut {l1317} *mut {l1318} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2437: histfile:   l1317 = UNIQUE | NON_NULL, (empty)
    // 2437: histfile:   l1318 = UNIQUE | NON_NULL, (empty)
        fclose(histfile);
        // 2438: histfile: typeof(_788) = *mut {l1321} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2438: histfile:   l1321 = UNIQUE | NON_NULL, (empty)
        // 2438: histfile: typeof(_789) = *mut {l1323} *mut {l1324} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2438: histfile:   l1323 = UNIQUE | NON_NULL, (empty)
        // 2438: histfile:   l1324 = UNIQUE | NON_NULL, (empty)
    }
    if res == 10 as libc::c_int {
        printf(b"s SATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        // 2441: b"s SATISFIABLE ... _char: typeof(_795) = *const {l1331} i8
        // 2441: b"s SATISFIABLE ... _char:   l1331 = UNIQUE | NON_NULL, (empty)
        // 2441: b"s SATISFIABLE ... st u8: typeof(_796) = *const {l1333} u8
        // 2441: b"s SATISFIABLE ... st u8:   l1333 = UNIQUE | NON_NULL, (empty)
        // 2441: b"s SATISFIABLE\n\0": typeof(_797) = *const {l1335} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 2441: b"s SATISFIABLE\n\0":   l1335 = UNIQUE | NON_NULL, (empty)
        // 2441: b"s SATISFIABLE\n\0": typeof(_798) = & {l1337} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 2441: b"s SATISFIABLE\n\0":   l1337 = UNIQUE | NON_NULL, FIXED
        // 2441: b"s SATISFIABLE\n\0": typeof(_797 = &raw const (*_798)) = *const {l1677} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 2441: b"s SATISFIABLE\n\0":   l1677 = UNIQUE | NON_NULL, (empty)
        // 2441: b"s SATISFIABLE ... _char: typeof(_795 = move _796 as *const i8 (Misc)) = *const {l1679} i8
        // 2441: b"s SATISFIABLE ... _char:   l1679 = UNIQUE | NON_NULL, (empty)
        // 2441: b"s SATISFIABLE ... st u8: typeof(_796 = move _797 as *const u8 (Pointer(ArrayToPointer))) = *const {l1678} u8
        // 2441: b"s SATISFIABLE ... st u8:   l1678 = UNIQUE | NON_NULL, (empty)
        // 2441: b"s SATISFIABLE\n\0": typeof(_798 = const b"s SATISFIABLE\n\x00") = & {l1676} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 2441: b"s SATISFIABLE\n\0":   l1676 = UNIQUE | NON_NULL, (empty)
    } else if res == 20 as libc::c_int {
        printf(b"s UNSATISFIABLE\n\0" as *const u8 as *const libc::c_char);
        // 2443: b"s UNSATISFIAB ... _char: typeof(_803) = *const {l1343} i8
        // 2443: b"s UNSATISFIAB ... _char:   l1343 = UNIQUE | NON_NULL, (empty)
        // 2443: b"s UNSATISFIAB ... st u8: typeof(_804) = *const {l1345} u8
        // 2443: b"s UNSATISFIAB ... st u8:   l1345 = UNIQUE | NON_NULL, (empty)
        // 2443: b"s UNSATISFIAB ... \n\0": typeof(_805) = *const {l1347} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2443: b"s UNSATISFIAB ... \n\0":   l1347 = UNIQUE | NON_NULL, (empty)
        // 2443: b"s UNSATISFIAB ... \n\0": typeof(_806) = & {l1349} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2443: b"s UNSATISFIAB ... \n\0":   l1349 = UNIQUE | NON_NULL, FIXED
        // 2443: b"s UNSATISFIAB ... _char: typeof(_803 = move _804 as *const i8 (Misc)) = *const {l1683} i8
        // 2443: b"s UNSATISFIAB ... _char:   l1683 = UNIQUE | NON_NULL, (empty)
        // 2443: b"s UNSATISFIAB ... \n\0": typeof(_805 = &raw const (*_806)) = *const {l1681} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2443: b"s UNSATISFIAB ... \n\0":   l1681 = UNIQUE | NON_NULL, (empty)
        // 2443: b"s UNSATISFIAB ... st u8: typeof(_804 = move _805 as *const u8 (Pointer(ArrayToPointer))) = *const {l1682} u8
        // 2443: b"s UNSATISFIAB ... st u8:   l1682 = UNIQUE | NON_NULL, (empty)
        // 2443: b"s UNSATISFIAB ... \n\0": typeof(_806 = const b"s UNSATISFIABLE\n\x00") = & {l1680} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 2443: b"s UNSATISFIAB ... \n\0":   l1680 = UNIQUE | NON_NULL, (empty)
    } else {
        printf(b"s UNKNOWN\n\0" as *const u8 as *const libc::c_char);
        // 2445: b"s UNKNOWN\n\0 ... _char: typeof(_808) = *const {l1352} i8
        // 2445: b"s UNKNOWN\n\0 ... _char:   l1352 = UNIQUE | NON_NULL, (empty)
        // 2445: b"s UNKNOWN\n\0 ... st u8: typeof(_809) = *const {l1354} u8
        // 2445: b"s UNKNOWN\n\0 ... st u8:   l1354 = UNIQUE | NON_NULL, (empty)
        // 2445: b"s UNKNOWN\n\0": typeof(_810) = *const {l1356} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2445: b"s UNKNOWN\n\0":   l1356 = UNIQUE | NON_NULL, (empty)
        // 2445: b"s UNKNOWN\n\0": typeof(_811) = & {l1358} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2445: b"s UNKNOWN\n\0":   l1358 = UNIQUE | NON_NULL, FIXED
        // 2445: b"s UNKNOWN\n\0": typeof(_810 = &raw const (*_811)) = *const {l1685} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2445: b"s UNKNOWN\n\0":   l1685 = UNIQUE | NON_NULL, (empty)
        // 2445: b"s UNKNOWN\n\0": typeof(_811 = const b"s UNKNOWN\n\x00") = & {l1684} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
        // 2445: b"s UNKNOWN\n\0":   l1684 = UNIQUE | NON_NULL, (empty)
        // 2445: b"s UNKNOWN\n\0 ... _char: typeof(_808 = move _809 as *const i8 (Misc)) = *const {l1687} i8
        // 2445: b"s UNKNOWN\n\0 ... _char:   l1687 = UNIQUE | NON_NULL, (empty)
        // 2445: b"s UNKNOWN\n\0 ... st u8: typeof(_809 = move _810 as *const u8 (Pointer(ArrayToPointer))) = *const {l1686} u8
        // 2445: b"s UNKNOWN\n\0 ... st u8:   l1686 = UNIQUE | NON_NULL, (empty)
    }
    fflush(stdout);
    // 2447: stdout: typeof(_813) = *mut {l1361} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2447: stdout:   l1361 = UNIQUE | NON_NULL, (empty)
    // 2447: stdout: typeof(_814) = *mut {l1363} *mut {l1364} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
    // 2447: stdout:   l1363 = UNIQUE | NON_NULL, (empty)
    // 2447: stdout:   l1364 = UNIQUE | NON_NULL, (empty)
    if !vals.is_null() {
    // 2448: vals: typeof(_818) = *mut {l1369} i8
    // 2448: vals:   l1369 = UNIQUE | NON_NULL, (empty)
    // 2448: vals: typeof(_819) = *mut {l1371} *mut {l1372} i8
    // 2448: vals:   l1371 = UNIQUE | NON_NULL, (empty)
    // 2448: vals:   l1372 = UNIQUE | NON_NULL, (empty)
        i = 1 as libc::c_int;
        while i < nvars {
        // 2450: nvars: typeof(_825) = *mut {l1379} i32
        // 2450: nvars:   l1379 = UNIQUE | NON_NULL, (empty)
            fputs(b"v \0" as *const u8 as *const libc::c_char, stdout);
            // 2451: b"v \0" as *con ... _char: typeof(_827) = *const {l1382} i8
            // 2451: b"v \0" as *con ... _char:   l1382 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0" as *const u8: typeof(_828) = *const {l1384} u8
            // 2451: b"v \0" as *const u8:   l1384 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0": typeof(_829) = *const {l1386} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2451: b"v \0":   l1386 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0": typeof(_830) = & {l1388} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2451: b"v \0":   l1388 = UNIQUE | NON_NULL, FIXED
            // 2451: stdout: typeof(_831) = *mut {l1390} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 2451: stdout:   l1390 = UNIQUE | NON_NULL, (empty)
            // 2451: stdout: typeof(_832) = *mut {l1392} *mut {l1393} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
            // 2451: stdout:   l1392 = UNIQUE | NON_NULL, (empty)
            // 2451: stdout:   l1393 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0" as *const u8: typeof(_828 = move _829 as *const u8 (Pointer(ArrayToPointer))) = *const {l1690} u8
            // 2451: b"v \0" as *const u8:   l1690 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0": typeof(_829 = &raw const (*_830)) = *const {l1689} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2451: b"v \0":   l1689 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0": typeof(_830 = const b"v \x00") = & {l1688} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2451: b"v \0":   l1688 = UNIQUE | NON_NULL, (empty)
            // 2451: b"v \0" as *con ... _char: typeof(_827 = move _828 as *const i8 (Misc)) = *const {l1691} i8
            // 2451: b"v \0" as *con ... _char:   l1691 = UNIQUE | NON_NULL, (empty)
            if (*vals.offset(i as isize) as libc::c_int) < 0 as libc::c_int {
            // 2452: vals.offset(i a ... size): typeof(_837) = *mut {l1399} i8
            // 2452: vals.offset(i a ... size):   l1399 = UNIQUE | NON_NULL, (empty)
            // 2452: vals: typeof(_838) = *mut {l1401} i8
            // 2452: vals:   l1401 = UNIQUE | NON_NULL, (empty)
            // 2452: vals: typeof(_839) = *mut {l1403} *mut {l1404} i8
            // 2452: vals:   l1403 = UNIQUE | NON_NULL, (empty)
            // 2452: vals:   l1404 = UNIQUE | NON_NULL, (empty)
                fputc('-' as i32, stdout);
                // 2453: stdout: typeof(_845) = *mut {l1411} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                // 2453: stdout:   l1411 = UNIQUE | NON_NULL, (empty)
                // 2453: stdout: typeof(_846) = *mut {l1413} *mut {l1414} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
                // 2453: stdout:   l1413 = UNIQUE | NON_NULL, (empty)
                // 2453: stdout:   l1414 = UNIQUE | NON_NULL, (empty)
            }
            printf(b"%d\n\0" as *const u8 as *const libc::c_char, i);
            // 2455: b"%d\n\0" as *c ... _char: typeof(_848) = *const {l1417} i8
            // 2455: b"%d\n\0" as *c ... _char:   l1417 = UNIQUE | NON_NULL, (empty)
            // 2455: b"%d\n\0" as *c ... st u8: typeof(_849) = *const {l1419} u8
            // 2455: b"%d\n\0" as *c ... st u8:   l1419 = UNIQUE | NON_NULL, (empty)
            // 2455: b"%d\n\0": typeof(_850) = *const {l1421} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2455: b"%d\n\0":   l1421 = UNIQUE | NON_NULL, (empty)
            // 2455: b"%d\n\0": typeof(_851) = & {l1423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2455: b"%d\n\0":   l1423 = UNIQUE | NON_NULL, FIXED
            // 2455: b"%d\n\0": typeof(_851 = const b"%d\n\x00") = & {l1692} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2455: b"%d\n\0":   l1692 = UNIQUE | NON_NULL, (empty)
            // 2455: b"%d\n\0": typeof(_850 = &raw const (*_851)) = *const {l1693} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 2455: b"%d\n\0":   l1693 = UNIQUE | NON_NULL, (empty)
            // 2455: b"%d\n\0" as *c ... _char: typeof(_848 = move _849 as *const i8 (Misc)) = *const {l1695} i8
            // 2455: b"%d\n\0" as *c ... _char:   l1695 = UNIQUE | NON_NULL, (empty)
            // 2455: b"%d\n\0" as *c ... st u8: typeof(_849 = move _850 as *const u8 (Pointer(ArrayToPointer))) = *const {l1694} u8
            // 2455: b"%d\n\0" as *c ... st u8:   l1694 = UNIQUE | NON_NULL, (empty)
            i += 1;
            i;
        }
        fputs(b"v 0\n\0" as *const u8 as *const libc::c_char, stdout);
        // 2459: b"v 0\n\0" as * ... _char: typeof(_859) = *const {l1432} i8
        // 2459: b"v 0\n\0" as * ... _char:   l1432 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0" as * ... st u8: typeof(_860) = *const {l1434} u8
        // 2459: b"v 0\n\0" as * ... st u8:   l1434 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0": typeof(_861) = *const {l1436} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 2459: b"v 0\n\0":   l1436 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0": typeof(_862) = & {l1438} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 2459: b"v 0\n\0":   l1438 = UNIQUE | NON_NULL, FIXED
        // 2459: stdout: typeof(_863) = *mut {l1440} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2459: stdout:   l1440 = UNIQUE | NON_NULL, (empty)
        // 2459: stdout: typeof(_864) = *mut {l1442} *mut {l1443} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2459: stdout:   l1442 = UNIQUE | NON_NULL, (empty)
        // 2459: stdout:   l1443 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0": typeof(_861 = &raw const (*_862)) = *const {l1697} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 2459: b"v 0\n\0":   l1697 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0" as * ... _char: typeof(_859 = move _860 as *const i8 (Misc)) = *const {l1699} i8
        // 2459: b"v 0\n\0" as * ... _char:   l1699 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0" as * ... st u8: typeof(_860 = move _861 as *const u8 (Pointer(ArrayToPointer))) = *const {l1698} u8
        // 2459: b"v 0\n\0" as * ... st u8:   l1698 = UNIQUE | NON_NULL, (empty)
        // 2459: b"v 0\n\0": typeof(_862 = const b"v 0\n\x00") = & {l1696} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 2459: b"v 0\n\0":   l1696 = UNIQUE | NON_NULL, (empty)
        fflush(stdout);
        // 2460: stdout: typeof(_866) = *mut {l1446} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2460: stdout:   l1446 = UNIQUE | NON_NULL, (empty)
        // 2460: stdout: typeof(_867) = *mut {l1448} *mut {l1449} DefId(0:214 ~ ilingeling[c969]::_IO_FILE)
        // 2460: stdout:   l1448 = UNIQUE | NON_NULL, (empty)
        // 2460: stdout:   l1449 = UNIQUE | NON_NULL, (empty)
    }
    reset();
    return res;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    // 2466: mut args: typeof(_1) = DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l1} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2466: mut args:   l1 = UNIQUE | NON_NULL, (empty)
    for arg in ::std::env::args() {
    // 2467: ::std::env::args(): typeof(_9) = &mut {l10} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2467: ::std::env::args():   l10 = UNIQUE | NON_NULL, (empty)
    // 2467: ::std::env::args(): typeof(_10) = &mut {l12} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2467: ::std::env::args():   l12 = UNIQUE | NON_NULL, (empty)
    // 2467: ::std::env::args(): typeof(_10 = &mut _5) = &mut {l51} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2467: ::std::env::args():   l51 = UNIQUE | NON_NULL, (empty)
    // 2467: ::std::env::args(): typeof(_9 = &mut (*_10)) = &mut {l52} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2467: ::std::env::args():   l52 = UNIQUE | NON_NULL, (empty)
        args.push(
        // 2468: args.push( (::s ... (), ): typeof(_15) = &mut {l18} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l19} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 2468: args.push( (::s ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 2468: args.push( (::s ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 2468: args.push( (::s ... (), ): typeof(_15 = &mut _1) = &mut {l53} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l54} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 2468: args.push( (::s ... (), ):   l53 = UNIQUE | NON_NULL, (empty)
        // 2468: args.push( (::s ... (), ):   l54 = UNIQUE | NON_NULL, (empty)
            (::std::ffi::CString::new(arg))
            // 2469: (::std::ffi::CS ... raw(): typeof(_16) = *mut {l21} i8
            // 2469: (::std::ffi::CS ... raw():   l21 = UNIQUE | NON_NULL, (empty)
                .expect("Failed to convert argument into CString.")
                // 2470: "Failed to conv ... ing.": typeof(_20) = & {l26} str
                // 2470: "Failed to conv ... ing.":   l26 = UNIQUE | NON_NULL, (empty)
                // 2470: "Failed to conv ... ing.": typeof(_21) = & {l28} str
                // 2470: "Failed to conv ... ing.":   l28 = UNIQUE | NON_NULL, FIXED
                // 2470: "Failed to conv ... ing.": typeof(_20 = &(*_21)) = & {l56} str
                // 2470: "Failed to conv ... ing.":   l56 = UNIQUE | NON_NULL, (empty)
                // 2470: "Failed to conv ... ing.": typeof(_21 = const "Failed to convert argument into CString.") = & {l55} str
                // 2470: "Failed to conv ... ing.":   l55 = UNIQUE | NON_NULL, (empty)
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    // 2474: args.push(::cor ... ut()): typeof(_23) = &mut {l31} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l32} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2474: args.push(::cor ... ut()):   l31 = UNIQUE | NON_NULL, (empty)
    // 2474: args.push(::cor ... ut()):   l32 = UNIQUE | NON_NULL, (empty)
    // 2474: ::core::ptr::nu ... mut(): typeof(_24) = *mut {l34} i8
    // 2474: ::core::ptr::nu ... mut():   l34 = UNIQUE | NON_NULL, (empty)
    // 2474: args.push(::cor ... ut()): typeof(_23 = &mut _1) = &mut {l57} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l58} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2474: args.push(::cor ... ut()):   l57 = UNIQUE | NON_NULL, (empty)
    // 2474: args.push(::cor ... ut()):   l58 = UNIQUE | NON_NULL, (empty)
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            // 2477: args.len(): typeof(_30) = & {l41} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l42} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2477: args.len():   l41 = UNIQUE | NON_NULL, (empty)
            // 2477: args.len():   l42 = UNIQUE | NON_NULL, (empty)
            // 2477: args.len(): typeof(_30 = &_1) = & {l59} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l60} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2477: args.len():   l59 = UNIQUE | NON_NULL, (empty)
            // 2477: args.len():   l60 = UNIQUE | NON_NULL, (empty)
            args.as_mut_ptr() as *mut *mut libc::c_char,
            // 2478: args.as_mut_ptr ... _char: typeof(_32) = *mut {l45} *mut {l46} i8
            // 2478: args.as_mut_ptr ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 2478: args.as_mut_ptr ... _char:   l46 = UNIQUE | NON_NULL, (empty)
            // 2478: args.as_mut_ptr(): typeof(_33) = &mut {l48} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l49} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2478: args.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
            // 2478: args.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 2478: args.as_mut_ptr(): typeof(_33 = &mut _1) = &mut {l61} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l62} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2478: args.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
            // 2478: args.as_mut_ptr():   l62 = UNIQUE | NON_NULL, (empty)
        ) as i32)
    }
}
