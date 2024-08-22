#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, extern_types, label_break_value)]
extern crate libgeling;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type LGL;
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
    fn lglwtrapi(_: *mut LGL, _: *mut FILE);
    fn lglmaxvar(_: *mut LGL) -> libc::c_int;
    fn lglincvar(_: *mut LGL) -> libc::c_int;
    fn lgladd(_: *mut LGL, lit_0: libc::c_int);
    fn lglassume(_: *mut LGL, lit_0: libc::c_int);
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vfprintf(_: *mut FILE, _: *const libc::c_char, _: ::core::ffi::VaList) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn lglookahead(_: *mut LGL) -> libc::c_int;
    fn lglreuse(_: *mut LGL, lit_0: libc::c_int);
    fn lglreusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglusable(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglmelt(_: *mut LGL, lit_0: libc::c_int);
    fn lglfreeze(_: *mut LGL, lit_0: libc::c_int);
    fn lglrepr(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglflushcache(_: *mut LGL);
    fn lglreducecache(_: *mut LGL);
    fn lglchanged(_: *mut LGL) -> libc::c_int;
    fn lglinconsistent(_: *mut LGL) -> libc::c_int;
    fn lglfailed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglfixed(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglderef(_: *mut LGL, lit_0: libc::c_int) -> libc::c_int;
    fn lglsimp(_: *mut LGL, iterations: libc::c_int) -> libc::c_int;
    fn lglsat(_: *mut LGL) -> libc::c_int;
    fn lglfixate(_: *mut LGL);
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn system(__command: *const libc::c_char) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn alarm(__seconds: libc::c_uint) -> libc::c_uint;
    fn dup(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn times(__buffer: *mut tms) -> clock_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn wait(__stat_loc: *mut libc::c_int) -> __pid_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn lglchkclone(_: *mut LGL);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag<'h0,'h1> {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: &'h0 (libc::c_void),
    // 110: overflow_arg_area: typeof(overflow_arg_area) = *mut {g138} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 110: overflow_arg_area:   g138 = UNIQUE | NON_NULL, (empty)
    pub reg_save_area: &'h1 (libc::c_void),
    // 111: reg_save_area: typeof(reg_save_area) = *mut {g139} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 111: reg_save_area:   g139 = UNIQUE | NON_NULL, (empty)
}
pub type size_t = libc::c_ulong;
pub type __gnuc_va_list = __builtin_va_list;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 125: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g140} i8
    // 125: _IO_read_ptr:   g140 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 126: _IO_read_end: typeof(_IO_read_end) = *mut {g141} i8
    // 126: _IO_read_end:   g141 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 127: _IO_read_base: typeof(_IO_read_base) = *mut {g142} i8
    // 127: _IO_read_base:   g142 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 128: _IO_write_base: typeof(_IO_write_base) = *mut {g143} i8
    // 128: _IO_write_base:   g143 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 129: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g144} i8
    // 129: _IO_write_ptr:   g144 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 130: _IO_write_end: typeof(_IO_write_end) = *mut {g145} i8
    // 130: _IO_write_end:   g145 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 131: _IO_buf_base: typeof(_IO_buf_base) = *mut {g146} i8
    // 131: _IO_buf_base:   g146 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 132: _IO_buf_end: typeof(_IO_buf_end) = *mut {g147} i8
    // 132: _IO_buf_end:   g147 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 133: _IO_save_base: typeof(_IO_save_base) = *mut {g148} i8
    // 133: _IO_save_base:   g148 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 134: _IO_backup_base: typeof(_IO_backup_base) = *mut {g149} i8
    // 134: _IO_backup_base:   g149 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 135: _IO_save_end: typeof(_IO_save_end) = *mut {g150} i8
    // 135: _IO_save_end:   g150 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 136: _markers: typeof(_markers) = *mut {g151} _IO_marker
    // 136: _markers:   g151 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 137: _chain: typeof(_chain) = *mut {g152} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 137: _chain:   g152 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 144: _lock: typeof(_lock) = *mut {g153} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 144: _lock:   g153 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 146: _codecvt: typeof(_codecvt) = *mut {g154} _IO_codecvt
    // 146: _codecvt:   g154 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 147: _wide_data: typeof(_wide_data) = *mut {g155} _IO_wide_data
    // 147: _wide_data:   g155 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 148: _freeres_list: typeof(_freeres_list) = *mut {g156} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 148: _freeres_list:   g156 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 149: _freeres_buf: typeof(_freeres_buf) = *mut {g157} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 149: _freeres_buf:   g157 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type va_list = __gnuc_va_list;
pub type pid_t = __pid_t;
pub type clock_t = __clock_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tms {
    pub tms_utime: clock_t,
    pub tms_stime: clock_t,
    pub tms_cutime: clock_t,
    pub tms_cstime: clock_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(libc::c_int) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Data<'h2,'h3,'h4,'h5> {
    pub lgl: *mut LGL,
    // 190: lgl: typeof(lgl) = *mut {g158} LGL
    // 190: lgl:   g158 = UNIQUE | NON_NULL, FIXED
    pub trace: *mut FILE,
    // 191: trace: typeof(trace) = *mut {g159} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 191: trace:   g159 = UNIQUE | NON_NULL, FIXED
    pub available: *mut libc::c_int,
    // 192: available: typeof(available) = *mut {g160} i32
    // 192: available:   g160 = UNIQUE | NON_NULL, FIXED
    pub navailable: libc::c_int,
    pub frozen: *mut libc::c_int,
    // 194: frozen: typeof(frozen) = *mut {g161} i32
    // 194: frozen:   g161 = UNIQUE | NON_NULL, FIXED
    pub nfrozen: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub c: libc::c_int,
    pub print: libc::c_int,
    pub noptsfuzz: libc::c_int,
}
pub type State = Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event<'h2,'h3,'h4,'h5,'h6,'h7> {
    pub state: std::option::Option<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>,
    // 206: state: typeof(state) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g162} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {g163} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 206: state:   g162 = UNIQUE | NON_NULL, FIXED
    // 206: state:   g163 = UNIQUE | NON_NULL, FIXED
    pub rand: libc::c_uint,
    pub remove: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub z: libc::c_uint,
    pub w: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Env<'h8,'h9,'h2,'h3,'h4,'h5,'h6,'h7,'h10> {
    pub file: *mut FILE,
    // 219: file: typeof(file) = *mut {g164} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 219: file:   g164 = UNIQUE | NON_NULL, FIXED
    pub prefix: *const libc::c_char,
    // 220: prefix: typeof(prefix) = *const {g165} i8
    // 220: prefix:   g165 = UNIQUE | NON_NULL, FIXED
    pub round: libc::c_int,
    pub bugs: libc::c_int,
    pub violations: libc::c_int,
    pub print: libc::c_int,
    pub quiet: libc::c_int,
    pub first: libc::c_int,
    pub nodd: libc::c_int,
    pub noptsfuzz: libc::c_int,
    pub alwaysfork: libc::c_int,
    pub terminal: libc::c_int,
    pub forked: libc::c_int,
    pub nevents: libc::c_int,
    pub timeout: libc::c_int,
    pub seed: libc::c_uint,
    pub events: *mut Event<'h2,'h3,'h4,'h5,'h6,'h7>,
    // 235: events: typeof(events) = *mut {g166} DefId(0:314 ~ lglmbt[b165]::Event)
    // 235: events:   g166 = UNIQUE | NON_NULL, FIXED
    pub rng: RNG,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
// 239: mut __nptr: typeof(_1) = *const {g0} i8
// 239: mut __nptr:   g0 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    return strtol(
        __nptr,
        // 241: __nptr: typeof(_4) = *const {l4} i8
        // 241: __nptr:   l4 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        // 242: 0 as *mut libc: ... _char: typeof(_5) = *mut {l6} *mut {g89} i8
        // 242: 0 as *mut libc: ... _char:   l6 = WRITE | UNIQUE, (empty)
        // 242: 0 as *mut libc: ... _char:   g89 = WRITE | OFFSET_ADD, CELL | FIXED
        // 242: 0 as *mut libc: ... _void: typeof(_6) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 242: 0 as *mut libc: ... _void:   l8 = WRITE | UNIQUE, (empty)
        // 242: 0 as *mut libc: ... _void: typeof(_6 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 242: 0 as *mut libc: ... _void:   l11 = WRITE | UNIQUE, (empty)
        // 242: 0 as *mut libc: ... _char: typeof(_5 = move _6 as *mut *mut i8 (Misc)) = *mut {l12} *mut {g89} i8
        // 242: 0 as *mut libc: ... _char:   l12 = WRITE | UNIQUE, (empty)
        // 242: 0 as *mut libc: ... _char:   g89 = WRITE | OFFSET_ADD, CELL | FIXED
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe extern "C" fn initrng(mut seed: libc::c_uint) -> RNG {
    let mut res: RNG = RNG { z: 0, w: 0 };
    res.z = seed.wrapping_mul(1000632769 as libc::c_uint);
    res.w = seed.wrapping_mul(2019164533 as libc::c_uint);
    return res;
}
unsafe extern "C" fn nextrand<'h0>(mut rng: &'h0 mut (RNG)) -> libc::c_uint {
// 252: mut rng: typeof(_1) = *mut {g1} DefId(0:321 ~ lglmbt[b165]::RNG)
// 252: mut rng:   g1 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    (*rng).z = (36969 as libc::c_int as libc::c_uint)
        .wrapping_mul((*rng).z & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add((*rng).z >> 16 as libc::c_int);
    (*rng).w = (18000 as libc::c_int as libc::c_uint)
        .wrapping_mul((*rng).w & 65535 as libc::c_int as libc::c_uint)
        .wrapping_add((*rng).w >> 16 as libc::c_int);
    return ((*rng).z << 16 as libc::c_int).wrapping_add((*rng).w);
}
unsafe fn nextrand_shim(arg0: *mut RNG) -> libc::c_uint {
    {
    let safe_arg0 = &mut *arg0;
    let safe_result = nextrand(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn pick(
    mut rng_ptr: *mut RNG,
    // 262: mut rng_ptr: typeof(_1) = *mut {g2} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 262: mut rng_ptr:   g2 = UNIQUE | NON_NULL, FIXED
    mut from: libc::c_uint,
    mut to: libc::c_uint,
) -> libc::c_int {
    let mut tmp: libc::c_uint = nextrand_shim(rng_ptr);
    // 266: rng_ptr: typeof(_6) = *mut {l6} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 266: rng_ptr:   l6 = UNIQUE | NON_NULL, (empty)
    let mut res: libc::c_int = 0;
    if from <= to && to <= 2147483647 as libc::c_int as libc::c_uint {
    } else {
        __assert_fail(
            b"from <= to && to <= INT_MAX\0" as *const u8 as *const libc::c_char,
            // 271: b"from <= to && ... _char: typeof(_19) = *const {l20} i8
            // 271: b"from <= to && ... _char:   l20 = UNIQUE | NON_NULL, (empty)
            // 271: b"from <= to && ... st u8: typeof(_20) = *const {l22} u8
            // 271: b"from <= to && ... st u8:   l22 = UNIQUE | NON_NULL, (empty)
            // 271: b"from <= to && ... AX\0": typeof(_21) = *const {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 271: b"from <= to && ... AX\0":   l24 = UNIQUE | NON_NULL, (empty)
            // 271: b"from <= to && ... AX\0": typeof(_22) = & {l26} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 271: b"from <= to && ... AX\0":   l26 = UNIQUE | NON_NULL, FIXED
            // 271: b"from <= to && ... AX\0": typeof(_22 = const b"from <= to && to <= INT_MAX\x00") = & {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 271: b"from <= to && ... AX\0":   l103 = UNIQUE | NON_NULL, (empty)
            // 271: b"from <= to && ... _char: typeof(_19 = move _20 as *const i8 (Misc)) = *const {l106} i8
            // 271: b"from <= to && ... _char:   l106 = UNIQUE | NON_NULL, (empty)
            // 271: b"from <= to && ... AX\0": typeof(_21 = &raw const (*_22)) = *const {l104} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
            // 271: b"from <= to && ... AX\0":   l104 = UNIQUE | NON_NULL, (empty)
            // 271: b"from <= to && ... st u8: typeof(_20 = move _21 as *const u8 (Pointer(ArrayToPointer))) = *const {l105} u8
            // 271: b"from <= to && ... st u8:   l105 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 272: b"lglmbt.c\0" a ... _char: typeof(_23) = *const {l28} i8
            // 272: b"lglmbt.c\0" a ... _char:   l28 = UNIQUE | NON_NULL, (empty)
            // 272: b"lglmbt.c\0" a ... st u8: typeof(_24) = *const {l30} u8
            // 272: b"lglmbt.c\0" a ... st u8:   l30 = UNIQUE | NON_NULL, (empty)
            // 272: b"lglmbt.c\0": typeof(_25) = *const {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 272: b"lglmbt.c\0":   l32 = UNIQUE | NON_NULL, (empty)
            // 272: b"lglmbt.c\0": typeof(_26) = & {l34} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 272: b"lglmbt.c\0":   l34 = UNIQUE | NON_NULL, FIXED
            // 272: b"lglmbt.c\0": typeof(_26 = const b"lglmbt.c\x00") = & {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 272: b"lglmbt.c\0":   l107 = UNIQUE | NON_NULL, (empty)
            // 272: b"lglmbt.c\0": typeof(_25 = &raw const (*_26)) = *const {l108} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 272: b"lglmbt.c\0":   l108 = UNIQUE | NON_NULL, (empty)
            // 272: b"lglmbt.c\0" a ... st u8: typeof(_24 = move _25 as *const u8 (Pointer(ArrayToPointer))) = *const {l109} u8
            // 272: b"lglmbt.c\0" a ... st u8:   l109 = UNIQUE | NON_NULL, (empty)
            // 272: b"lglmbt.c\0" a ... _char: typeof(_23 = move _24 as *const i8 (Misc)) = *const {l110} i8
            // 272: b"lglmbt.c\0" a ... _char:   l110 = UNIQUE | NON_NULL, (empty)
            85 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
            // 274: (*::core::mem:: ... ptr(): typeof(_29) = *const {l38} i8
            // 274: (*::core::mem:: ... ptr():   l38 = UNIQUE | NON_NULL, (empty)
            // 274: (*::core::mem:: ... ptr(): typeof(_30) = & {l40} [i8]
            // 274: (*::core::mem:: ... ptr():   l40 = UNIQUE | NON_NULL, FIXED
            // 274: (*::core::mem:: ... ptr(): typeof(_31) = & {l42} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
            // 274: (*::core::mem:: ... ptr():   l42 = UNIQUE | NON_NULL, (empty)
            // 274: ::core::mem::tr ... 0", ): typeof(_32) = & {l44} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
            // 274: ::core::mem::tr ... 0", ):   l44 = UNIQUE | NON_NULL, FIXED
            // 274: (*::core::mem:: ... ptr(): typeof(_31 = &(*_32)) = & {l113} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
            // 274: (*::core::mem:: ... ptr():   l113 = UNIQUE | NON_NULL, (empty)
            // 274: (*::core::mem:: ... ptr(): typeof(_30 = move _31 as &[i8] (Pointer(Unsize))) = & {l114} [i8]
            // 274: (*::core::mem:: ... ptr():   l114 = UNIQUE | NON_NULL, FIXED
                b"int pick(RNG *, unsigned int, unsigned int)\0",
                // 275: b"int pick(RNG  ... t)\0": typeof(_33) = & {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 275: b"int pick(RNG  ... t)\0":   l46 = UNIQUE | NON_NULL, (empty)
                // 275: b"int pick(RNG  ... t)\0": typeof(_34) = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 275: b"int pick(RNG  ... t)\0":   l48 = UNIQUE | NON_NULL, FIXED
                // 275: b"int pick(RNG  ... t)\0": typeof(_33 = &(*_34)) = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 275: b"int pick(RNG  ... t)\0":   l112 = UNIQUE | NON_NULL, (empty)
                // 275: b"int pick(RNG  ... t)\0": typeof(_34 = const b"int pick(RNG *, unsigned int, unsigned int)\x00") = & {l111} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 275: b"int pick(RNG  ... t)\0":   l111 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_4591: {
        if from <= to && to <= 2147483647 as libc::c_int as libc::c_uint {
        } else {
            __assert_fail(
                b"from <= to && to <= INT_MAX\0" as *const u8 as *const libc::c_char,
                // 284: b"from <= to && ... _char: typeof(_46) = *const {l61} i8
                // 284: b"from <= to && ... _char:   l61 = UNIQUE | NON_NULL, (empty)
                // 284: b"from <= to && ... st u8: typeof(_47) = *const {l63} u8
                // 284: b"from <= to && ... st u8:   l63 = UNIQUE | NON_NULL, (empty)
                // 284: b"from <= to && ... AX\0": typeof(_48) = *const {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 284: b"from <= to && ... AX\0":   l65 = UNIQUE | NON_NULL, (empty)
                // 284: b"from <= to && ... AX\0": typeof(_49) = & {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 284: b"from <= to && ... AX\0":   l67 = UNIQUE | NON_NULL, FIXED
                // 284: b"from <= to && ... st u8: typeof(_47 = move _48 as *const u8 (Pointer(ArrayToPointer))) = *const {l117} u8
                // 284: b"from <= to && ... st u8:   l117 = UNIQUE | NON_NULL, (empty)
                // 284: b"from <= to && ... AX\0": typeof(_49 = const b"from <= to && to <= INT_MAX\x00") = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 284: b"from <= to && ... AX\0":   l115 = UNIQUE | NON_NULL, (empty)
                // 284: b"from <= to && ... AX\0": typeof(_48 = &raw const (*_49)) = *const {l116} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
                // 284: b"from <= to && ... AX\0":   l116 = UNIQUE | NON_NULL, (empty)
                // 284: b"from <= to && ... _char: typeof(_46 = move _47 as *const i8 (Misc)) = *const {l118} i8
                // 284: b"from <= to && ... _char:   l118 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 285: b"lglmbt.c\0" a ... _char: typeof(_50) = *const {l69} i8
                // 285: b"lglmbt.c\0" a ... _char:   l69 = UNIQUE | NON_NULL, (empty)
                // 285: b"lglmbt.c\0" a ... st u8: typeof(_51) = *const {l71} u8
                // 285: b"lglmbt.c\0" a ... st u8:   l71 = UNIQUE | NON_NULL, (empty)
                // 285: b"lglmbt.c\0": typeof(_52) = *const {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 285: b"lglmbt.c\0":   l73 = UNIQUE | NON_NULL, (empty)
                // 285: b"lglmbt.c\0": typeof(_53) = & {l75} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 285: b"lglmbt.c\0":   l75 = UNIQUE | NON_NULL, FIXED
                // 285: b"lglmbt.c\0": typeof(_53 = const b"lglmbt.c\x00") = & {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 285: b"lglmbt.c\0":   l119 = UNIQUE | NON_NULL, (empty)
                // 285: b"lglmbt.c\0": typeof(_52 = &raw const (*_53)) = *const {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 285: b"lglmbt.c\0":   l120 = UNIQUE | NON_NULL, (empty)
                // 285: b"lglmbt.c\0" a ... st u8: typeof(_51 = move _52 as *const u8 (Pointer(ArrayToPointer))) = *const {l121} u8
                // 285: b"lglmbt.c\0" a ... st u8:   l121 = UNIQUE | NON_NULL, (empty)
                // 285: b"lglmbt.c\0" a ... _char: typeof(_50 = move _51 as *const i8 (Misc)) = *const {l122} i8
                // 285: b"lglmbt.c\0" a ... _char:   l122 = UNIQUE | NON_NULL, (empty)
                85 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 44], &[libc::c_char; 44]>(
                // 287: (*::core::mem:: ... ptr(): typeof(_56) = *const {l79} i8
                // 287: (*::core::mem:: ... ptr():   l79 = UNIQUE | NON_NULL, (empty)
                // 287: (*::core::mem:: ... ptr(): typeof(_57) = & {l81} [i8]
                // 287: (*::core::mem:: ... ptr():   l81 = UNIQUE | NON_NULL, FIXED
                // 287: (*::core::mem:: ... ptr(): typeof(_58) = & {l83} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 287: (*::core::mem:: ... ptr():   l83 = UNIQUE | NON_NULL, (empty)
                // 287: ::core::mem::tr ... 0", ): typeof(_59) = & {l85} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 287: ::core::mem::tr ... 0", ):   l85 = UNIQUE | NON_NULL, FIXED
                // 287: (*::core::mem:: ... ptr(): typeof(_58 = &(*_59)) = & {l125} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 287: (*::core::mem:: ... ptr():   l125 = UNIQUE | NON_NULL, (empty)
                // 287: (*::core::mem:: ... ptr(): typeof(_57 = move _58 as &[i8] (Pointer(Unsize))) = & {l126} [i8]
                // 287: (*::core::mem:: ... ptr():   l126 = UNIQUE | NON_NULL, FIXED
                    b"int pick(RNG *, unsigned int, unsigned int)\0",
                    // 288: b"int pick(RNG  ... t)\0": typeof(_60) = & {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 288: b"int pick(RNG  ... t)\0":   l87 = UNIQUE | NON_NULL, (empty)
                    // 288: b"int pick(RNG  ... t)\0": typeof(_61) = & {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 288: b"int pick(RNG  ... t)\0":   l89 = UNIQUE | NON_NULL, FIXED
                    // 288: b"int pick(RNG  ... t)\0": typeof(_60 = &(*_61)) = & {l124} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 288: b"int pick(RNG  ... t)\0":   l124 = UNIQUE | NON_NULL, (empty)
                    // 288: b"int pick(RNG  ... t)\0": typeof(_61 = const b"int pick(RNG *, unsigned int, unsigned int)\x00") = & {l123} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 288: b"int pick(RNG  ... t)\0":   l123 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    tmp = tmp.wrapping_rem(
        to.wrapping_sub(from)
            .wrapping_add(1 as libc::c_int as libc::c_uint),
    );
    tmp = tmp.wrapping_add(from);
    res = tmp as libc::c_int;
    return res;
}
unsafe extern "C" fn onabort<'h0>(mut dummy: &'h0 (libc::c_void)) {
// 302: mut dummy: typeof(_1) = *mut {g3} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 302: mut dummy:   g3 = UNIQUE | NON_NULL, (empty)
    exit(42 as libc::c_int);
}
unsafe fn onabort_shim(arg0: *mut libc::c_void) {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_result = onabort(safe_arg0);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn init(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 305: *mut libc::c_void: typeof(_0) = *mut {g29} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 305: *mut libc::c_void:   g29 = UNIQUE | NON_NULL, FIXED
// 305: mut data: typeof(_1) = *mut {g28} DefId(0:299 ~ lglmbt[b165]::Data)
// 305: mut data:   g28 = UNIQUE | NON_NULL, FIXED
    let mut rng: RNG = initrng(r);
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    (*data).m = pick(
        &mut rng,
        // 310: &mut rng: typeof(_9) = *mut {l9} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 310: &mut rng:   l9 = UNIQUE | NON_NULL, (empty)
        // 310: &mut rng: typeof(_10) = &mut {l11} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 310: &mut rng:   l11 = UNIQUE | NON_NULL, (empty)
        // 310: &mut rng: typeof(_9 = &raw mut (*_10)) = *mut {l134} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 310: &mut rng:   l134 = UNIQUE | NON_NULL, (empty)
        // 310: &mut rng: typeof(_10 = &mut _4) = &mut {l133} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 310: &mut rng:   l133 = UNIQUE | NON_NULL, (empty)
        10 as libc::c_int as libc::c_uint,
        200 as libc::c_int as libc::c_uint,
    );
    t = pick(
        &mut rng,
        // 315: &mut rng: typeof(_16) = *mut {l18} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 315: &mut rng:   l18 = UNIQUE | NON_NULL, (empty)
        // 315: &mut rng: typeof(_17) = &mut {l20} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 315: &mut rng:   l20 = UNIQUE | NON_NULL, (empty)
        // 315: &mut rng: typeof(_17 = &mut _4) = &mut {l135} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 315: &mut rng:   l135 = UNIQUE | NON_NULL, (empty)
        // 315: &mut rng: typeof(_16 = &raw mut (*_17)) = *mut {l136} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 315: &mut rng:   l136 = UNIQUE | NON_NULL, (empty)
        390 as libc::c_int as libc::c_uint,
        450 as libc::c_int as libc::c_uint,
    );
    (*data).n = (*data).m * t / 100 as libc::c_int;
    if (*data).print != 0 {
        printf(
            b"cnf %d %d \0" as *const u8 as *const libc::c_char,
            // 322: b"cnf %d %d \0" ... _char: typeof(_35) = *const {l39} i8
            // 322: b"cnf %d %d \0" ... _char:   l39 = UNIQUE | NON_NULL, (empty)
            // 322: b"cnf %d %d \0" ... st u8: typeof(_36) = *const {l41} u8
            // 322: b"cnf %d %d \0" ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
            // 322: b"cnf %d %d \0": typeof(_37) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 322: b"cnf %d %d \0":   l43 = UNIQUE | NON_NULL, (empty)
            // 322: b"cnf %d %d \0": typeof(_38) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 322: b"cnf %d %d \0":   l45 = UNIQUE | NON_NULL, FIXED
            // 322: b"cnf %d %d \0" ... _char: typeof(_35 = move _36 as *const i8 (Misc)) = *const {l140} i8
            // 322: b"cnf %d %d \0" ... _char:   l140 = UNIQUE | NON_NULL, (empty)
            // 322: b"cnf %d %d \0": typeof(_38 = const b"cnf %d %d \x00") = & {l137} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 322: b"cnf %d %d \0":   l137 = UNIQUE | NON_NULL, (empty)
            // 322: b"cnf %d %d \0" ... st u8: typeof(_36 = move _37 as *const u8 (Pointer(ArrayToPointer))) = *const {l139} u8
            // 322: b"cnf %d %d \0" ... st u8:   l139 = UNIQUE | NON_NULL, (empty)
            // 322: b"cnf %d %d \0": typeof(_37 = &raw const (*_38)) = *const {l138} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 322: b"cnf %d %d \0":   l138 = UNIQUE | NON_NULL, (empty)
            (*data).m,
            (*data).n,
        );
        fflush(stdout);
        // 326: stdout: typeof(_42) = *mut {l50} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 326: stdout:   l50 = UNIQUE | NON_NULL, (empty)
        // 326: stdout: typeof(_43) = *mut {l52} *mut {l53} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 326: stdout:   l52 = UNIQUE | NON_NULL, (empty)
        // 326: stdout:   l53 = UNIQUE | NON_NULL, (empty)
    }
    (*data).lgl = lglinit();
    // 328: lglinit(): typeof(_44) = *mut {l55} LGL
    // 328: lglinit():   l55 = UNIQUE | NON_NULL, (empty)
    lglonabort(
        (*data).lgl,
        // 330: (*data).lgl: typeof(_46) = *mut {l58} LGL
        // 330: (*data).lgl:   l58 = UNIQUE | NON_NULL, (empty)
        0 as *mut libc::c_void,
        // 331: 0 as *mut libc: ... _void: typeof(_47) = *mut {l60} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 331: 0 as *mut libc: ... _void:   l60 = UNIQUE | NON_NULL, (empty)
        // 331: 0 as *mut libc: ... _void: typeof(_47 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l141} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 331: 0 as *mut libc: ... _void:   l141 = UNIQUE | NON_NULL, (empty)
        Some(onabort_shim as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        // 332: Some(onabort as ... > ()): typeof(_48) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l62} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 332: Some(onabort as ... > ()):   l62 = UNIQUE | NON_NULL, (empty)
        // 332: onabort as unsa ... -> (): typeof(_49) = fn(*mut {l64} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 332: onabort as unsa ... -> ():   l64 = UNIQUE | NON_NULL, (empty)
        // 332: onabort: typeof(_49 = onabort as unsafe extern "C" fn(*mut libc::c_void) (Pointer(ReifyFnPointer))) = fn(*mut {l142} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()
        // 332: onabort:   l142 = UNIQUE | NON_NULL, (empty)
        // 332: Some(onabort as ... > ()): typeof(_48 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void)>::Some(move _49)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l143} DefId(2:5583 ~ core[480a]::ffi::c_void)) -> ()>
        // 332: Some(onabort as ... > ()):   l143 = UNIQUE | NON_NULL, (empty)
    );
    if !((*data).trace).is_null() {
    // 334: ((*data).trace): typeof(_53) = *mut {l69} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 334: ((*data).trace):   l69 = UNIQUE | NON_NULL, (empty)
        lglwtrapi((*data).lgl, (*data).trace);
        // 335: (*data).lgl: typeof(_55) = *mut {l72} LGL
        // 335: (*data).lgl:   l72 = UNIQUE | NON_NULL, (empty)
        // 335: (*data).trace: typeof(_56) = *mut {l74} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 335: (*data).trace:   l74 = UNIQUE | NON_NULL, (empty)
    }
    (*data).navailable = (*data).m;
    (*data).available = malloc(
    // 338: malloc( ((*data ... g), ): typeof(_58) = *mut {l77} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 338: malloc( ((*data ... g), ):   l77 = UNIQUE | NON_NULL, (empty)
    // 338: (*data).availab ... c_int: typeof(((*_1).2: *mut i32) = move _58 as *mut i32 (Misc)) = *mut {l144} i32
    // 338: (*data).availab ... c_int:   l144 = UNIQUE | NON_NULL, (empty)
        ((*data).navailable as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*data).navailable {
        *((*data).available).offset(i as isize) = i + 1 as libc::c_int;
        // 344: ((*data).availa ... size): typeof(_73) = *mut {l93} i32
        // 344: ((*data).availa ... size):   l93 = UNIQUE | NON_NULL, (empty)
        // 344: ((*data).available): typeof(_74) = *mut {l95} i32
        // 344: ((*data).available):   l95 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    if (*data).noptsfuzz != 0 {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
        // 352: Some( cnf as un ... id, ): typeof(_86) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l108} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l109} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 352: Some( cnf as un ... id, ):   l108 = UNIQUE | NON_NULL, (empty)
        // 352: Some( cnf as un ... id, ):   l109 = UNIQUE | NON_NULL, (empty)
        // 352: Some( cnf as un ... id, ): typeof(_86 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _87)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l147} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l148} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 352: Some( cnf as un ... id, ):   l147 = UNIQUE | NON_NULL, (empty)
        // 352: Some( cnf as un ... id, ):   l148 = UNIQUE | NON_NULL, (empty)
            cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
            // 353: cnf as unsafe e ... _void: typeof(_87) = fn(*mut {l111} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l112} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 353: cnf as unsafe e ... _void:   l111 = UNIQUE | NON_NULL, (empty)
            // 353: cnf as unsafe e ... _void:   l112 = UNIQUE | NON_NULL, (empty)
            // 353: cnf: typeof(_87 = cnf as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l145} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l146} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 353: cnf:   l145 = UNIQUE | NON_NULL, (empty)
            // 353: cnf:   l146 = UNIQUE | NON_NULL, (empty)
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(
        if pick(
        // 360: if pick( &mut r ... id) }: typeof(_88) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l114} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l115} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 360: if pick( &mut r ... id) }:   l114 = UNIQUE | NON_NULL, (empty)
        // 360: if pick( &mut r ... id) }:   l115 = UNIQUE | NON_NULL, (empty)
            &mut rng,
            // 361: &mut rng: typeof(_91) = *mut {l119} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 361: &mut rng:   l119 = UNIQUE | NON_NULL, (empty)
            // 361: &mut rng: typeof(_92) = &mut {l121} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 361: &mut rng:   l121 = UNIQUE | NON_NULL, (empty)
            // 361: &mut rng: typeof(_91 = &raw mut (*_92)) = *mut {l150} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 361: &mut rng:   l150 = UNIQUE | NON_NULL, (empty)
            // 361: &mut rng: typeof(_92 = &mut _4) = &mut {l149} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 361: &mut rng:   l149 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            Some(opts as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
            // 366: opts as unsafe  ... _void: typeof(_97) = fn(*mut {l127} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l128} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 366: opts as unsafe  ... _void:   l127 = UNIQUE | NON_NULL, (empty)
            // 366: opts as unsafe  ... _void:   l128 = UNIQUE | NON_NULL, (empty)
            // 366: Some(opts as un ... void): typeof(_88 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _97)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l153} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l154} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 366: Some(opts as un ... void):   l153 = UNIQUE | NON_NULL, (empty)
            // 366: Some(opts as un ... void):   l154 = UNIQUE | NON_NULL, (empty)
            // 366: opts: typeof(_97 = opts as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l151} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l152} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 366: opts:   l151 = UNIQUE | NON_NULL, (empty)
            // 366: opts:   l152 = UNIQUE | NON_NULL, (empty)
        } else {
            Some(cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
            // 368: cnf as unsafe e ... _void: typeof(_98) = fn(*mut {l130} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l131} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 368: cnf as unsafe e ... _void:   l130 = UNIQUE | NON_NULL, (empty)
            // 368: cnf as unsafe e ... _void:   l131 = UNIQUE | NON_NULL, (empty)
            // 368: Some(cnf as uns ... void): typeof(_88 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _98)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l157} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l158} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 368: Some(cnf as uns ... void):   l157 = UNIQUE | NON_NULL, (empty)
            // 368: Some(cnf as uns ... void):   l158 = UNIQUE | NON_NULL, (empty)
            // 368: cnf: typeof(_98 = cnf as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l155} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l156} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 368: cnf:   l155 = UNIQUE | NON_NULL, (empty)
            // 368: cnf:   l156 = UNIQUE | NON_NULL, (empty)
        },
    );
}
unsafe extern "C" fn inc(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 372: *mut libc::c_void: typeof(_0) = *mut {g5} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 372: *mut libc::c_void:   g5 = UNIQUE | NON_NULL, FIXED
// 372: mut data: typeof(_1) = *mut {g4} DefId(0:299 ~ lglmbt[b165]::Data)
// 372: mut data:   g4 = UNIQUE | NON_NULL, FIXED
    let mut i: libc::c_int = 0;
    let mut oldavailable: libc::c_int = 0;
    let mut newvars: libc::c_int = 0;
    let mut rng: RNG = RNG { z: 0, w: 0 };
    rng = initrng(r);
    if pick(
        &mut rng,
        // 379: &mut rng: typeof(_13) = *mut {l13} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 379: &mut rng:   l13 = UNIQUE | NON_NULL, (empty)
        // 379: &mut rng: typeof(_14) = &mut {l15} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 379: &mut rng:   l15 = UNIQUE | NON_NULL, (empty)
        // 379: &mut rng: typeof(_13 = &raw mut (*_14)) = *mut {l151} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 379: &mut rng:   l151 = UNIQUE | NON_NULL, (empty)
        // 379: &mut rng: typeof(_14 = &mut _7) = &mut {l150} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 379: &mut rng:   l150 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    ) != 0
    {
        lglmaxvar((*data).lgl);
        // 384: (*data).lgl: typeof(_20) = *mut {l22} LGL
        // 384: (*data).lgl:   l22 = UNIQUE | NON_NULL, (empty)
    }
    while pick(
        &mut rng,
        // 387: &mut rng: typeof(_25) = *mut {l28} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 387: &mut rng:   l28 = UNIQUE | NON_NULL, (empty)
        // 387: &mut rng: typeof(_26) = &mut {l30} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 387: &mut rng:   l30 = UNIQUE | NON_NULL, (empty)
        // 387: &mut rng: typeof(_26 = &mut _7) = &mut {l152} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 387: &mut rng:   l152 = UNIQUE | NON_NULL, (empty)
        // 387: &mut rng: typeof(_25 = &raw mut (*_26)) = *mut {l153} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 387: &mut rng:   l153 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
    ) != 0
    {
        lglfixed(
            (*data).lgl,
            // 393: (*data).lgl: typeof(_32) = *mut {l37} LGL
            // 393: (*data).lgl:   l37 = UNIQUE | NON_NULL, (empty)
            pick(
                &mut rng,
                // 395: &mut rng: typeof(_34) = *mut {l40} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 395: &mut rng:   l40 = UNIQUE | NON_NULL, (empty)
                // 395: &mut rng: typeof(_35) = &mut {l42} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 395: &mut rng:   l42 = UNIQUE | NON_NULL, (empty)
                // 395: &mut rng: typeof(_34 = &raw mut (*_35)) = *mut {l155} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 395: &mut rng:   l155 = UNIQUE | NON_NULL, (empty)
                // 395: &mut rng: typeof(_35 = &mut _7) = &mut {l154} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 395: &mut rng:   l154 = UNIQUE | NON_NULL, (empty)
                1 as libc::c_int as libc::c_uint,
                (*data).m as libc::c_uint,
            ),
        );
    }
    while pick(
        &mut rng,
        // 402: &mut rng: typeof(_46) = *mut {l54} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 402: &mut rng:   l54 = UNIQUE | NON_NULL, (empty)
        // 402: &mut rng: typeof(_47) = &mut {l56} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 402: &mut rng:   l56 = UNIQUE | NON_NULL, (empty)
        // 402: &mut rng: typeof(_46 = &raw mut (*_47)) = *mut {l157} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 402: &mut rng:   l157 = UNIQUE | NON_NULL, (empty)
        // 402: &mut rng: typeof(_47 = &mut _7) = &mut {l156} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 402: &mut rng:   l156 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    ) != 0
    {
        lglrepr(
            (*data).lgl,
            // 408: (*data).lgl: typeof(_53) = *mut {l63} LGL
            // 408: (*data).lgl:   l63 = UNIQUE | NON_NULL, (empty)
            pick(
                &mut rng,
                // 410: &mut rng: typeof(_55) = *mut {l66} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 410: &mut rng:   l66 = UNIQUE | NON_NULL, (empty)
                // 410: &mut rng: typeof(_56) = &mut {l68} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 410: &mut rng:   l68 = UNIQUE | NON_NULL, (empty)
                // 410: &mut rng: typeof(_56 = &mut _7) = &mut {l158} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 410: &mut rng:   l158 = UNIQUE | NON_NULL, (empty)
                // 410: &mut rng: typeof(_55 = &raw mut (*_56)) = *mut {l159} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 410: &mut rng:   l159 = UNIQUE | NON_NULL, (empty)
                1 as libc::c_int as libc::c_uint,
                (*data).m as libc::c_uint,
            ),
        );
    }
    newvars = pick(
        &mut rng,
        // 417: &mut rng: typeof(_65) = *mut {l78} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 417: &mut rng:   l78 = UNIQUE | NON_NULL, (empty)
        // 417: &mut rng: typeof(_66) = &mut {l80} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 417: &mut rng:   l80 = UNIQUE | NON_NULL, (empty)
        // 417: &mut rng: typeof(_66 = &mut _7) = &mut {l160} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 417: &mut rng:   l160 = UNIQUE | NON_NULL, (empty)
        // 417: &mut rng: typeof(_65 = &raw mut (*_66)) = *mut {l161} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 417: &mut rng:   l161 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        50 as libc::c_int as libc::c_uint,
    );
    if newvars != 0 {
        if pick(
            &mut rng,
            // 423: &mut rng: typeof(_77) = *mut {l92} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 423: &mut rng:   l92 = UNIQUE | NON_NULL, (empty)
            // 423: &mut rng: typeof(_78) = &mut {l94} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 423: &mut rng:   l94 = UNIQUE | NON_NULL, (empty)
            // 423: &mut rng: typeof(_77 = &raw mut (*_78)) = *mut {l163} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 423: &mut rng:   l163 = UNIQUE | NON_NULL, (empty)
            // 423: &mut rng: typeof(_78 = &mut _7) = &mut {l162} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 423: &mut rng:   l162 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            lglincvar((*data).lgl);
            // 428: (*data).lgl: typeof(_84) = *mut {l101} LGL
            // 428: (*data).lgl:   l101 = UNIQUE | NON_NULL, (empty)
        }
        oldavailable = (*data).navailable;
        (*data).navailable += newvars;
        (*data).available = realloc(
        // 432: realloc( (*data ... g), ): typeof(_88) = *mut {l106} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 432: realloc( (*data ... g), ):   l106 = UNIQUE | NON_NULL, (empty)
        // 432: (*data).availab ... c_int: typeof(((*_1).2: *mut i32) = move _88 as *mut i32 (Misc)) = *mut {l165} i32
        // 432: (*data).availab ... c_int:   l165 = UNIQUE | NON_NULL, (empty)
            (*data).available as *mut libc::c_void,
            // 433: (*data).availab ... _void: typeof(_89) = *mut {l108} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 433: (*data).availab ... _void:   l108 = UNIQUE | NON_NULL, (empty)
            // 433: (*data).available: typeof(_90) = *mut {l110} i32
            // 433: (*data).available:   l110 = UNIQUE | NON_NULL, (empty)
            // 433: (*data).availab ... _void: typeof(_89 = move _90 as *mut libc::c_void (Misc)) = *mut {l164} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 433: (*data).availab ... _void:   l164 = UNIQUE | NON_NULL, (empty)
            ((*data).navailable as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        i = 0 as libc::c_int;
        while i < newvars {
            *((*data).available).offset((oldavailable + i) as isize) =
            // 439: ((*data).availa ... size): typeof(_107) = *mut {l128} i32
            // 439: ((*data).availa ... size):   l128 = UNIQUE | NON_NULL, (empty)
            // 439: ((*data).available): typeof(_108) = *mut {l130} i32
            // 439: ((*data).available):   l130 = UNIQUE | NON_NULL, (empty)
                (*data).m + i + 1 as libc::c_int;
            i += 1;
            i;
        }
        (*data).m += newvars;
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 449: Some( cnf as un ... id, ): typeof(_121) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l144} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l145} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 449: Some( cnf as un ... id, ):   l144 = UNIQUE | NON_NULL, (empty)
    // 449: Some( cnf as un ... id, ):   l145 = UNIQUE | NON_NULL, (empty)
    // 449: Some( cnf as un ... id, ): typeof(_121 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _122)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l168} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l169} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 449: Some( cnf as un ... id, ):   l168 = UNIQUE | NON_NULL, (empty)
    // 449: Some( cnf as un ... id, ):   l169 = UNIQUE | NON_NULL, (empty)
        cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 450: cnf as unsafe e ... _void: typeof(_122) = fn(*mut {l147} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l148} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 450: cnf as unsafe e ... _void:   l147 = UNIQUE | NON_NULL, (empty)
        // 450: cnf as unsafe e ... _void:   l148 = UNIQUE | NON_NULL, (empty)
        // 450: cnf: typeof(_122 = cnf as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l166} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l167} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 450: cnf:   l166 = UNIQUE | NON_NULL, (empty)
        // 450: cnf:   l167 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn opts(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 453: *mut libc::c_void: typeof(_0) = *mut {g27} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 453: *mut libc::c_void:   g27 = UNIQUE | NON_NULL, FIXED
// 453: mut data: typeof(_1) = *mut {g26} DefId(0:299 ~ lglmbt[b165]::Data)
// 453: mut data:   g26 = UNIQUE | NON_NULL, FIXED
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut it: *mut libc::c_void = 0 as *mut libc::c_void;
    // 459: mut it: typeof(_9) = *mut {l9} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 459: mut it:   l9 = UNIQUE | NON_NULL, (empty)
    // 459: 0 as *mut libc: ... _void: typeof(_9 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l462} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 459: 0 as *mut libc: ... _void:   l462 = UNIQUE | NON_NULL, (empty)
    let mut rng: RNG = initrng(r);
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    // 461: mut name: typeof(_12) = *const {l13} i8
    // 461: mut name:   l13 = UNIQUE | NON_NULL, (empty)
    // 461: 0 as *const lib ... _char: typeof(_12 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l463} i8
    // 461: 0 as *const lib ... _char:   l463 = UNIQUE | NON_NULL, (empty)
    if pick(
        &mut rng,
        // 463: &mut rng: typeof(_16) = *mut {l18} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 463: &mut rng:   l18 = UNIQUE | NON_NULL, (empty)
        // 463: &mut rng: typeof(_17) = &mut {l20} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 463: &mut rng:   l20 = UNIQUE | NON_NULL, (empty)
        // 463: &mut rng: typeof(_16 = &raw mut (*_17)) = *mut {l465} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 463: &mut rng:   l465 = UNIQUE | NON_NULL, (empty)
        // 463: &mut rng: typeof(_17 = &mut _10) = &mut {l464} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 463: &mut rng:   l464 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglsetopt(
            (*data).lgl,
            // 469: (*data).lgl: typeof(_23) = *mut {l27} LGL
            // 469: (*data).lgl:   l27 = UNIQUE | NON_NULL, (empty)
            b"plain\0" as *const u8 as *const libc::c_char,
            // 470: b"plain\0" as * ... _char: typeof(_24) = *const {l29} i8
            // 470: b"plain\0" as * ... _char:   l29 = UNIQUE | NON_NULL, (empty)
            // 470: b"plain\0" as * ... st u8: typeof(_25) = *const {l31} u8
            // 470: b"plain\0" as * ... st u8:   l31 = UNIQUE | NON_NULL, (empty)
            // 470: b"plain\0": typeof(_26) = *const {l33} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 470: b"plain\0":   l33 = UNIQUE | NON_NULL, (empty)
            // 470: b"plain\0": typeof(_27) = & {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 470: b"plain\0":   l35 = UNIQUE | NON_NULL, FIXED
            // 470: b"plain\0": typeof(_27 = const b"plain\x00") = & {l466} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 470: b"plain\0":   l466 = UNIQUE | NON_NULL, (empty)
            // 470: b"plain\0" as * ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l468} u8
            // 470: b"plain\0" as * ... st u8:   l468 = UNIQUE | NON_NULL, (empty)
            // 470: b"plain\0" as * ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l469} i8
            // 470: b"plain\0" as * ... _char:   l469 = UNIQUE | NON_NULL, (empty)
            // 470: b"plain\0": typeof(_26 = &raw const (*_27)) = *const {l467} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 470: b"plain\0":   l467 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int,
        );
    } else {
        it = lglfirstopt((*data).lgl);
        // 474: lglfirstopt((*d ... .lgl): typeof(_29) = *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 474: lglfirstopt((*d ... .lgl):   l38 = UNIQUE | NON_NULL, (empty)
        // 474: (*data).lgl: typeof(_30) = *mut {l40} LGL
        // 474: (*data).lgl:   l40 = UNIQUE | NON_NULL, (empty)
        loop {
            it = lglnextopt((*data).lgl, it, &mut name, &mut val, &mut min, &mut max);
            // 476: lglnextopt((*da ...  max): typeof(_33) = *mut {l44} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 476: lglnextopt((*da ...  max):   l44 = UNIQUE | NON_NULL, (empty)
            // 476: (*data).lgl: typeof(_34) = *mut {l46} LGL
            // 476: (*data).lgl:   l46 = UNIQUE | NON_NULL, (empty)
            // 476: it: typeof(_35) = *mut {l48} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 476: it:   l48 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name: typeof(_36) = *mut {l50} *const {l51} i8
            // 476: &mut name:   l50 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name:   l51 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name: typeof(_37) = &mut {l53} *const {l54} i8
            // 476: &mut name:   l53 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name:   l54 = UNIQUE | NON_NULL, (empty)
            // 476: &mut val: typeof(_38) = *mut {l56} i32
            // 476: &mut val:   l56 = UNIQUE | NON_NULL, (empty)
            // 476: &mut val: typeof(_39) = &mut {l58} i32
            // 476: &mut val:   l58 = UNIQUE | NON_NULL, (empty)
            // 476: &mut min: typeof(_40) = *mut {l60} i32
            // 476: &mut min:   l60 = UNIQUE | NON_NULL, (empty)
            // 476: &mut min: typeof(_41) = &mut {l62} i32
            // 476: &mut min:   l62 = UNIQUE | NON_NULL, (empty)
            // 476: &mut max: typeof(_42) = *mut {l64} i32
            // 476: &mut max:   l64 = UNIQUE | NON_NULL, (empty)
            // 476: &mut max: typeof(_43) = &mut {l66} i32
            // 476: &mut max:   l66 = UNIQUE | NON_NULL, (empty)
            // 476: &mut val: typeof(_39 = &mut _6) = &mut {l474} i32
            // 476: &mut val:   l474 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name: typeof(_37 = &mut _12) = &mut {l470} *const {l471} i8
            // 476: &mut name:   l470 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name:   l471 = UNIQUE | NON_NULL, (empty)
            // 476: &mut min: typeof(_41 = &mut _5) = &mut {l476} i32
            // 476: &mut min:   l476 = UNIQUE | NON_NULL, (empty)
            // 476: &mut val: typeof(_38 = &raw mut (*_39)) = *mut {l475} i32
            // 476: &mut val:   l475 = UNIQUE | NON_NULL, (empty)
            // 476: &mut min: typeof(_40 = &raw mut (*_41)) = *mut {l477} i32
            // 476: &mut min:   l477 = UNIQUE | NON_NULL, (empty)
            // 476: &mut max: typeof(_43 = &mut _7) = &mut {l478} i32
            // 476: &mut max:   l478 = UNIQUE | NON_NULL, (empty)
            // 476: &mut max: typeof(_42 = &raw mut (*_43)) = *mut {l479} i32
            // 476: &mut max:   l479 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name: typeof(_36 = &raw mut (*_37)) = *mut {l472} *const {l473} i8
            // 476: &mut name:   l472 = UNIQUE | NON_NULL, (empty)
            // 476: &mut name:   l473 = UNIQUE | NON_NULL, (empty)
            if it.is_null() {
            // 477: it: typeof(_46) = *mut {l70} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 477: it:   l70 = UNIQUE | NON_NULL, (empty)
                break;
            }
            n += 1;
            n;
        }
        if n > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const libc::c_char,
                // 486: b"n > 0\0" as * ... _char: typeof(_56) = *const {l81} i8
                // 486: b"n > 0\0" as * ... _char:   l81 = UNIQUE | NON_NULL, (empty)
                // 486: b"n > 0\0" as * ... st u8: typeof(_57) = *const {l83} u8
                // 486: b"n > 0\0" as * ... st u8:   l83 = UNIQUE | NON_NULL, (empty)
                // 486: b"n > 0\0": typeof(_58) = *const {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 486: b"n > 0\0":   l85 = UNIQUE | NON_NULL, (empty)
                // 486: b"n > 0\0": typeof(_59) = & {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 486: b"n > 0\0":   l87 = UNIQUE | NON_NULL, FIXED
                // 486: b"n > 0\0": typeof(_58 = &raw const (*_59)) = *const {l481} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 486: b"n > 0\0":   l481 = UNIQUE | NON_NULL, (empty)
                // 486: b"n > 0\0": typeof(_59 = const b"n > 0\x00") = & {l480} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                // 486: b"n > 0\0":   l480 = UNIQUE | NON_NULL, (empty)
                // 486: b"n > 0\0" as * ... st u8: typeof(_57 = move _58 as *const u8 (Pointer(ArrayToPointer))) = *const {l482} u8
                // 486: b"n > 0\0" as * ... st u8:   l482 = UNIQUE | NON_NULL, (empty)
                // 486: b"n > 0\0" as * ... _char: typeof(_56 = move _57 as *const i8 (Misc)) = *const {l483} i8
                // 486: b"n > 0\0" as * ... _char:   l483 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 487: b"lglmbt.c\0" a ... _char: typeof(_60) = *const {l89} i8
                // 487: b"lglmbt.c\0" a ... _char:   l89 = UNIQUE | NON_NULL, (empty)
                // 487: b"lglmbt.c\0" a ... st u8: typeof(_61) = *const {l91} u8
                // 487: b"lglmbt.c\0" a ... st u8:   l91 = UNIQUE | NON_NULL, (empty)
                // 487: b"lglmbt.c\0": typeof(_62) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 487: b"lglmbt.c\0":   l93 = UNIQUE | NON_NULL, (empty)
                // 487: b"lglmbt.c\0": typeof(_63) = & {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 487: b"lglmbt.c\0":   l95 = UNIQUE | NON_NULL, FIXED
                // 487: b"lglmbt.c\0" a ... _char: typeof(_60 = move _61 as *const i8 (Misc)) = *const {l487} i8
                // 487: b"lglmbt.c\0" a ... _char:   l487 = UNIQUE | NON_NULL, (empty)
                // 487: b"lglmbt.c\0": typeof(_62 = &raw const (*_63)) = *const {l485} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 487: b"lglmbt.c\0":   l485 = UNIQUE | NON_NULL, (empty)
                // 487: b"lglmbt.c\0": typeof(_63 = const b"lglmbt.c\x00") = & {l484} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 487: b"lglmbt.c\0":   l484 = UNIQUE | NON_NULL, (empty)
                // 487: b"lglmbt.c\0" a ... st u8: typeof(_61 = move _62 as *const u8 (Pointer(ArrayToPointer))) = *const {l486} u8
                // 487: b"lglmbt.c\0" a ... st u8:   l486 = UNIQUE | NON_NULL, (empty)
                148 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                // 489: (*::core::mem:: ... ptr(): typeof(_66) = *const {l99} i8
                // 489: (*::core::mem:: ... ptr():   l99 = UNIQUE | NON_NULL, (empty)
                // 489: (*::core::mem:: ... ptr(): typeof(_67) = & {l101} [i8]
                // 489: (*::core::mem:: ... ptr():   l101 = UNIQUE | NON_NULL, FIXED
                // 489: (*::core::mem:: ... ptr(): typeof(_68) = & {l103} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 489: (*::core::mem:: ... ptr():   l103 = UNIQUE | NON_NULL, (empty)
                // 489: ::core::mem::tr ... 0", ): typeof(_69) = & {l105} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 489: ::core::mem::tr ... 0", ):   l105 = UNIQUE | NON_NULL, FIXED
                // 489: (*::core::mem:: ... ptr(): typeof(_67 = move _68 as &[i8] (Pointer(Unsize))) = & {l491} [i8]
                // 489: (*::core::mem:: ... ptr():   l491 = UNIQUE | NON_NULL, FIXED
                // 489: (*::core::mem:: ... ptr(): typeof(_68 = &(*_69)) = & {l490} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                // 489: (*::core::mem:: ... ptr():   l490 = UNIQUE | NON_NULL, (empty)
                    b"void *opts(Data *, unsigned int)\0",
                    // 490: b"void *opts(Da ... t)\0": typeof(_70) = & {l107} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 490: b"void *opts(Da ... t)\0":   l107 = UNIQUE | NON_NULL, (empty)
                    // 490: b"void *opts(Da ... t)\0": typeof(_71) = & {l109} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 490: b"void *opts(Da ... t)\0":   l109 = UNIQUE | NON_NULL, FIXED
                    // 490: b"void *opts(Da ... t)\0": typeof(_71 = const b"void *opts(Data *, unsigned int)\x00") = & {l488} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 490: b"void *opts(Da ... t)\0":   l488 = UNIQUE | NON_NULL, (empty)
                    // 490: b"void *opts(Da ... t)\0": typeof(_70 = &(*_71)) = & {l489} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 490: b"void *opts(Da ... t)\0":   l489 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_7012: {
            if n > 0 as libc::c_int {
            } else {
                __assert_fail(
                    b"n > 0\0" as *const u8 as *const libc::c_char,
                    // 499: b"n > 0\0" as * ... _char: typeof(_78) = *const {l117} i8
                    // 499: b"n > 0\0" as * ... _char:   l117 = UNIQUE | NON_NULL, (empty)
                    // 499: b"n > 0\0" as * ... st u8: typeof(_79) = *const {l119} u8
                    // 499: b"n > 0\0" as * ... st u8:   l119 = UNIQUE | NON_NULL, (empty)
                    // 499: b"n > 0\0": typeof(_80) = *const {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 499: b"n > 0\0":   l121 = UNIQUE | NON_NULL, (empty)
                    // 499: b"n > 0\0": typeof(_81) = & {l123} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 499: b"n > 0\0":   l123 = UNIQUE | NON_NULL, FIXED
                    // 499: b"n > 0\0" as * ... _char: typeof(_78 = move _79 as *const i8 (Misc)) = *const {l495} i8
                    // 499: b"n > 0\0" as * ... _char:   l495 = UNIQUE | NON_NULL, (empty)
                    // 499: b"n > 0\0": typeof(_81 = const b"n > 0\x00") = & {l492} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 499: b"n > 0\0":   l492 = UNIQUE | NON_NULL, (empty)
                    // 499: b"n > 0\0": typeof(_80 = &raw const (*_81)) = *const {l493} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
                    // 499: b"n > 0\0":   l493 = UNIQUE | NON_NULL, (empty)
                    // 499: b"n > 0\0" as * ... st u8: typeof(_79 = move _80 as *const u8 (Pointer(ArrayToPointer))) = *const {l494} u8
                    // 499: b"n > 0\0" as * ... st u8:   l494 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 500: b"lglmbt.c\0" a ... _char: typeof(_82) = *const {l125} i8
                    // 500: b"lglmbt.c\0" a ... _char:   l125 = UNIQUE | NON_NULL, (empty)
                    // 500: b"lglmbt.c\0" a ... st u8: typeof(_83) = *const {l127} u8
                    // 500: b"lglmbt.c\0" a ... st u8:   l127 = UNIQUE | NON_NULL, (empty)
                    // 500: b"lglmbt.c\0": typeof(_84) = *const {l129} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 500: b"lglmbt.c\0":   l129 = UNIQUE | NON_NULL, (empty)
                    // 500: b"lglmbt.c\0": typeof(_85) = & {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 500: b"lglmbt.c\0":   l131 = UNIQUE | NON_NULL, FIXED
                    // 500: b"lglmbt.c\0": typeof(_84 = &raw const (*_85)) = *const {l497} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 500: b"lglmbt.c\0":   l497 = UNIQUE | NON_NULL, (empty)
                    // 500: b"lglmbt.c\0": typeof(_85 = const b"lglmbt.c\x00") = & {l496} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 500: b"lglmbt.c\0":   l496 = UNIQUE | NON_NULL, (empty)
                    // 500: b"lglmbt.c\0" a ... _char: typeof(_82 = move _83 as *const i8 (Misc)) = *const {l499} i8
                    // 500: b"lglmbt.c\0" a ... _char:   l499 = UNIQUE | NON_NULL, (empty)
                    // 500: b"lglmbt.c\0" a ... st u8: typeof(_83 = move _84 as *const u8 (Pointer(ArrayToPointer))) = *const {l498} u8
                    // 500: b"lglmbt.c\0" a ... st u8:   l498 = UNIQUE | NON_NULL, (empty)
                    148 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 33], &[libc::c_char; 33]>(
                    // 502: (*::core::mem:: ... ptr(): typeof(_88) = *const {l135} i8
                    // 502: (*::core::mem:: ... ptr():   l135 = UNIQUE | NON_NULL, (empty)
                    // 502: (*::core::mem:: ... ptr(): typeof(_89) = & {l137} [i8]
                    // 502: (*::core::mem:: ... ptr():   l137 = UNIQUE | NON_NULL, FIXED
                    // 502: (*::core::mem:: ... ptr(): typeof(_90) = & {l139} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 502: (*::core::mem:: ... ptr():   l139 = UNIQUE | NON_NULL, (empty)
                    // 502: ::core::mem::tr ... 0", ): typeof(_91) = & {l141} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 502: ::core::mem::tr ... 0", ):   l141 = UNIQUE | NON_NULL, FIXED
                    // 502: (*::core::mem:: ... ptr(): typeof(_89 = move _90 as &[i8] (Pointer(Unsize))) = & {l503} [i8]
                    // 502: (*::core::mem:: ... ptr():   l503 = UNIQUE | NON_NULL, FIXED
                    // 502: (*::core::mem:: ... ptr(): typeof(_90 = &(*_91)) = & {l502} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                    // 502: (*::core::mem:: ... ptr():   l502 = UNIQUE | NON_NULL, (empty)
                        b"void *opts(Data *, unsigned int)\0",
                        // 503: b"void *opts(Da ... t)\0": typeof(_92) = & {l143} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                        // 503: b"void *opts(Da ... t)\0":   l143 = UNIQUE | NON_NULL, (empty)
                        // 503: b"void *opts(Da ... t)\0": typeof(_93) = & {l145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                        // 503: b"void *opts(Da ... t)\0":   l145 = UNIQUE | NON_NULL, FIXED
                        // 503: b"void *opts(Da ... t)\0": typeof(_92 = &(*_93)) = & {l501} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                        // 503: b"void *opts(Da ... t)\0":   l501 = UNIQUE | NON_NULL, (empty)
                        // 503: b"void *opts(Da ... t)\0": typeof(_93 = const b"void *opts(Data *, unsigned int)\x00") = & {l500} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000021)) }]
                        // 503: b"void *opts(Da ... t)\0":   l500 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
        };
        m = pick(
            &mut rng,
            // 510: &mut rng: typeof(_95) = *mut {l148} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 510: &mut rng:   l148 = UNIQUE | NON_NULL, (empty)
            // 510: &mut rng: typeof(_96) = &mut {l150} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 510: &mut rng:   l150 = UNIQUE | NON_NULL, (empty)
            // 510: &mut rng: typeof(_96 = &mut _10) = &mut {l504} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 510: &mut rng:   l504 = UNIQUE | NON_NULL, (empty)
            // 510: &mut rng: typeof(_95 = &raw mut (*_96)) = *mut {l505} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 510: &mut rng:   l505 = UNIQUE | NON_NULL, (empty)
            1 as libc::c_int as libc::c_uint,
            10 as libc::c_int as libc::c_uint,
        );
        it = lglfirstopt((*data).lgl);
        // 514: lglfirstopt((*d ... .lgl): typeof(_101) = *mut {l156} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 514: lglfirstopt((*d ... .lgl):   l156 = UNIQUE | NON_NULL, (empty)
        // 514: (*data).lgl: typeof(_102) = *mut {l158} LGL
        // 514: (*data).lgl:   l158 = UNIQUE | NON_NULL, (empty)
        loop {
            it = lglnextopt((*data).lgl, it, &mut name, &mut val, &mut min, &mut max);
            // 516: lglnextopt((*da ...  max): typeof(_103) = *mut {l160} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 516: lglnextopt((*da ...  max):   l160 = UNIQUE | NON_NULL, (empty)
            // 516: (*data).lgl: typeof(_104) = *mut {l162} LGL
            // 516: (*data).lgl:   l162 = UNIQUE | NON_NULL, (empty)
            // 516: it: typeof(_105) = *mut {l164} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 516: it:   l164 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name: typeof(_106) = *mut {l166} *const {l167} i8
            // 516: &mut name:   l166 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name:   l167 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name: typeof(_107) = &mut {l169} *const {l170} i8
            // 516: &mut name:   l169 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name:   l170 = UNIQUE | NON_NULL, (empty)
            // 516: &mut val: typeof(_108) = *mut {l172} i32
            // 516: &mut val:   l172 = UNIQUE | NON_NULL, (empty)
            // 516: &mut val: typeof(_109) = &mut {l174} i32
            // 516: &mut val:   l174 = UNIQUE | NON_NULL, (empty)
            // 516: &mut min: typeof(_110) = *mut {l176} i32
            // 516: &mut min:   l176 = UNIQUE | NON_NULL, (empty)
            // 516: &mut min: typeof(_111) = &mut {l178} i32
            // 516: &mut min:   l178 = UNIQUE | NON_NULL, (empty)
            // 516: &mut max: typeof(_112) = *mut {l180} i32
            // 516: &mut max:   l180 = UNIQUE | NON_NULL, (empty)
            // 516: &mut max: typeof(_113) = &mut {l182} i32
            // 516: &mut max:   l182 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name: typeof(_107 = &mut _12) = &mut {l506} *const {l507} i8
            // 516: &mut name:   l506 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name:   l507 = UNIQUE | NON_NULL, (empty)
            // 516: &mut min: typeof(_110 = &raw mut (*_111)) = *mut {l513} i32
            // 516: &mut min:   l513 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name: typeof(_106 = &raw mut (*_107)) = *mut {l508} *const {l509} i8
            // 516: &mut name:   l508 = UNIQUE | NON_NULL, (empty)
            // 516: &mut name:   l509 = UNIQUE | NON_NULL, (empty)
            // 516: &mut min: typeof(_111 = &mut _5) = &mut {l512} i32
            // 516: &mut min:   l512 = UNIQUE | NON_NULL, (empty)
            // 516: &mut val: typeof(_108 = &raw mut (*_109)) = *mut {l511} i32
            // 516: &mut val:   l511 = UNIQUE | NON_NULL, (empty)
            // 516: &mut max: typeof(_113 = &mut _7) = &mut {l514} i32
            // 516: &mut max:   l514 = UNIQUE | NON_NULL, (empty)
            // 516: &mut max: typeof(_112 = &raw mut (*_113)) = *mut {l515} i32
            // 516: &mut max:   l515 = UNIQUE | NON_NULL, (empty)
            // 516: &mut val: typeof(_109 = &mut _6) = &mut {l510} i32
            // 516: &mut val:   l510 = UNIQUE | NON_NULL, (empty)
            if it.is_null() {
            // 517: it: typeof(_116) = *mut {l186} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 517: it:   l186 = UNIQUE | NON_NULL, (empty)
                break;
            }
            if pick(
                &mut rng,
                // 521: &mut rng: typeof(_121) = *mut {l192} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 521: &mut rng:   l192 = UNIQUE | NON_NULL, (empty)
                // 521: &mut rng: typeof(_122) = &mut {l194} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 521: &mut rng:   l194 = UNIQUE | NON_NULL, (empty)
                // 521: &mut rng: typeof(_121 = &raw mut (*_122)) = *mut {l517} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 521: &mut rng:   l517 = UNIQUE | NON_NULL, (empty)
                // 521: &mut rng: typeof(_122 = &mut _10) = &mut {l516} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 521: &mut rng:   l516 = UNIQUE | NON_NULL, (empty)
                1 as libc::c_int as libc::c_uint,
                m as libc::c_uint,
            ) != 1 as libc::c_int
            {
                continue;
            }
            if strcmp(name, b"check\0" as *const u8 as *const libc::c_char) == 0 {
            // 528: name: typeof(_132) = *const {l205} i8
            // 528: name:   l205 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0" as * ... _char: typeof(_133) = *const {l207} i8
            // 528: b"check\0" as * ... _char:   l207 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0" as * ... st u8: typeof(_134) = *const {l209} u8
            // 528: b"check\0" as * ... st u8:   l209 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0": typeof(_135) = *const {l211} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 528: b"check\0":   l211 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0": typeof(_136) = & {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 528: b"check\0":   l213 = UNIQUE | NON_NULL, FIXED
            // 528: b"check\0": typeof(_135 = &raw const (*_136)) = *const {l519} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 528: b"check\0":   l519 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0": typeof(_136 = const b"check\x00") = & {l518} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 528: b"check\0":   l518 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0" as * ... _char: typeof(_133 = move _134 as *const i8 (Misc)) = *const {l521} i8
            // 528: b"check\0" as * ... _char:   l521 = UNIQUE | NON_NULL, (empty)
            // 528: b"check\0" as * ... st u8: typeof(_134 = move _135 as *const u8 (Pointer(ArrayToPointer))) = *const {l520} u8
            // 528: b"check\0" as * ... st u8:   l520 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"drupligtrace\0" as *const u8 as *const libc::c_char) == 0 {
            // 531: name: typeof(_141) = *const {l219} i8
            // 531: name:   l219 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\ ... _char: typeof(_142) = *const {l221} i8
            // 531: b"drupligtrace\ ... _char:   l221 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\ ... st u8: typeof(_143) = *const {l223} u8
            // 531: b"drupligtrace\ ... st u8:   l223 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\0": typeof(_144) = *const {l225} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 531: b"drupligtrace\0":   l225 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\0": typeof(_145) = & {l227} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 531: b"drupligtrace\0":   l227 = UNIQUE | NON_NULL, FIXED
            // 531: b"drupligtrace\ ... _char: typeof(_142 = move _143 as *const i8 (Misc)) = *const {l525} i8
            // 531: b"drupligtrace\ ... _char:   l525 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\0": typeof(_145 = const b"drupligtrace\x00") = & {l522} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 531: b"drupligtrace\0":   l522 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\ ... st u8: typeof(_143 = move _144 as *const u8 (Pointer(ArrayToPointer))) = *const {l524} u8
            // 531: b"drupligtrace\ ... st u8:   l524 = UNIQUE | NON_NULL, (empty)
            // 531: b"drupligtrace\0": typeof(_144 = &raw const (*_145)) = *const {l523} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 531: b"drupligtrace\0":   l523 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"log\0" as *const u8 as *const libc::c_char) == 0 {
            // 534: name: typeof(_150) = *const {l233} i8
            // 534: name:   l233 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0" as *co ... _char: typeof(_151) = *const {l235} i8
            // 534: b"log\0" as *co ... _char:   l235 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0" as *co ... st u8: typeof(_152) = *const {l237} u8
            // 534: b"log\0" as *co ... st u8:   l237 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0": typeof(_153) = *const {l239} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 534: b"log\0":   l239 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0": typeof(_154) = & {l241} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 534: b"log\0":   l241 = UNIQUE | NON_NULL, FIXED
            // 534: b"log\0" as *co ... _char: typeof(_151 = move _152 as *const i8 (Misc)) = *const {l529} i8
            // 534: b"log\0" as *co ... _char:   l529 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0" as *co ... st u8: typeof(_152 = move _153 as *const u8 (Pointer(ArrayToPointer))) = *const {l528} u8
            // 534: b"log\0" as *co ... st u8:   l528 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0": typeof(_154 = const b"log\x00") = & {l526} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 534: b"log\0":   l526 = UNIQUE | NON_NULL, (empty)
            // 534: b"log\0": typeof(_153 = &raw const (*_154)) = *const {l527} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
            // 534: b"log\0":   l527 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"sleeponabort\0" as *const u8 as *const libc::c_char) == 0 {
            // 537: name: typeof(_159) = *const {l247} i8
            // 537: name:   l247 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\ ... _char: typeof(_160) = *const {l249} i8
            // 537: b"sleeponabort\ ... _char:   l249 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\ ... st u8: typeof(_161) = *const {l251} u8
            // 537: b"sleeponabort\ ... st u8:   l251 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\0": typeof(_162) = *const {l253} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 537: b"sleeponabort\0":   l253 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\0": typeof(_163) = & {l255} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 537: b"sleeponabort\0":   l255 = UNIQUE | NON_NULL, FIXED
            // 537: b"sleeponabort\0": typeof(_163 = const b"sleeponabort\x00") = & {l530} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 537: b"sleeponabort\0":   l530 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\0": typeof(_162 = &raw const (*_163)) = *const {l531} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 537: b"sleeponabort\0":   l531 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\ ... st u8: typeof(_161 = move _162 as *const u8 (Pointer(ArrayToPointer))) = *const {l532} u8
            // 537: b"sleeponabort\ ... st u8:   l532 = UNIQUE | NON_NULL, (empty)
            // 537: b"sleeponabort\ ... _char: typeof(_160 = move _161 as *const i8 (Misc)) = *const {l533} i8
            // 537: b"sleeponabort\ ... _char:   l533 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"exitonabort\0" as *const u8 as *const libc::c_char) == 0 {
            // 540: name: typeof(_168) = *const {l261} i8
            // 540: name:   l261 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0 ... _char: typeof(_169) = *const {l263} i8
            // 540: b"exitonabort\0 ... _char:   l263 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0 ... st u8: typeof(_170) = *const {l265} u8
            // 540: b"exitonabort\0 ... st u8:   l265 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0": typeof(_171) = *const {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 540: b"exitonabort\0":   l267 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0": typeof(_172) = & {l269} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 540: b"exitonabort\0":   l269 = UNIQUE | NON_NULL, FIXED
            // 540: b"exitonabort\0": typeof(_172 = const b"exitonabort\x00") = & {l534} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 540: b"exitonabort\0":   l534 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0 ... _char: typeof(_169 = move _170 as *const i8 (Misc)) = *const {l537} i8
            // 540: b"exitonabort\0 ... _char:   l537 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0": typeof(_171 = &raw const (*_172)) = *const {l535} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 540: b"exitonabort\0":   l535 = UNIQUE | NON_NULL, (empty)
            // 540: b"exitonabort\0 ... st u8: typeof(_170 = move _171 as *const u8 (Pointer(ArrayToPointer))) = *const {l536} u8
            // 540: b"exitonabort\0 ... st u8:   l536 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"verbose\0" as *const u8 as *const libc::c_char) == 0 {
            // 543: name: typeof(_177) = *const {l275} i8
            // 543: name:   l275 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0" as ... _char: typeof(_178) = *const {l277} i8
            // 543: b"verbose\0" as ... _char:   l277 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0" as ... st u8: typeof(_179) = *const {l279} u8
            // 543: b"verbose\0" as ... st u8:   l279 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0": typeof(_180) = *const {l281} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 543: b"verbose\0":   l281 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0": typeof(_181) = & {l283} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 543: b"verbose\0":   l283 = UNIQUE | NON_NULL, FIXED
            // 543: b"verbose\0" as ... st u8: typeof(_179 = move _180 as *const u8 (Pointer(ArrayToPointer))) = *const {l540} u8
            // 543: b"verbose\0" as ... st u8:   l540 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0": typeof(_180 = &raw const (*_181)) = *const {l539} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 543: b"verbose\0":   l539 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0": typeof(_181 = const b"verbose\x00") = & {l538} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 543: b"verbose\0":   l538 = UNIQUE | NON_NULL, (empty)
            // 543: b"verbose\0" as ... _char: typeof(_178 = move _179 as *const i8 (Misc)) = *const {l541} i8
            // 543: b"verbose\0" as ... _char:   l541 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"witness\0" as *const u8 as *const libc::c_char) == 0 {
            // 546: name: typeof(_186) = *const {l289} i8
            // 546: name:   l289 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0" as ... _char: typeof(_187) = *const {l291} i8
            // 546: b"witness\0" as ... _char:   l291 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0" as ... st u8: typeof(_188) = *const {l293} u8
            // 546: b"witness\0" as ... st u8:   l293 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0": typeof(_189) = *const {l295} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 546: b"witness\0":   l295 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0": typeof(_190) = & {l297} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 546: b"witness\0":   l297 = UNIQUE | NON_NULL, FIXED
            // 546: b"witness\0": typeof(_189 = &raw const (*_190)) = *const {l543} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 546: b"witness\0":   l543 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0": typeof(_190 = const b"witness\x00") = & {l542} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 546: b"witness\0":   l542 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0" as ... _char: typeof(_187 = move _188 as *const i8 (Misc)) = *const {l545} i8
            // 546: b"witness\0" as ... _char:   l545 = UNIQUE | NON_NULL, (empty)
            // 546: b"witness\0" as ... st u8: typeof(_188 = move _189 as *const u8 (Pointer(ArrayToPointer))) = *const {l544} u8
            // 546: b"witness\0" as ... st u8:   l544 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if strcmp(name, b"prune\0" as *const u8 as *const libc::c_char) == 0 {
            // 549: name: typeof(_195) = *const {l303} i8
            // 549: name:   l303 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0" as * ... _char: typeof(_196) = *const {l305} i8
            // 549: b"prune\0" as * ... _char:   l305 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0" as * ... st u8: typeof(_197) = *const {l307} u8
            // 549: b"prune\0" as * ... st u8:   l307 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0": typeof(_198) = *const {l309} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 549: b"prune\0":   l309 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0": typeof(_199) = & {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 549: b"prune\0":   l311 = UNIQUE | NON_NULL, FIXED
            // 549: b"prune\0": typeof(_199 = const b"prune\x00") = & {l546} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 549: b"prune\0":   l546 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0" as * ... st u8: typeof(_197 = move _198 as *const u8 (Pointer(ArrayToPointer))) = *const {l548} u8
            // 549: b"prune\0" as * ... st u8:   l548 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0": typeof(_198 = &raw const (*_199)) = *const {l547} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
            // 549: b"prune\0":   l547 = UNIQUE | NON_NULL, (empty)
            // 549: b"prune\0" as * ... _char: typeof(_196 = move _197 as *const i8 (Misc)) = *const {l549} i8
            // 549: b"prune\0" as * ... _char:   l549 = UNIQUE | NON_NULL, (empty)
                continue;
            }
            if pick(
                &mut rng,
                // 553: &mut rng: typeof(_204) = *mut {l317} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 553: &mut rng:   l317 = UNIQUE | NON_NULL, (empty)
                // 553: &mut rng: typeof(_205) = &mut {l319} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 553: &mut rng:   l319 = UNIQUE | NON_NULL, (empty)
                // 553: &mut rng: typeof(_205 = &mut _10) = &mut {l550} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 553: &mut rng:   l550 = UNIQUE | NON_NULL, (empty)
                // 553: &mut rng: typeof(_204 = &raw mut (*_205)) = *mut {l551} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 553: &mut rng:   l551 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                if strcmp(name, b"locsmaxeff\0" as *const u8 as *const libc::c_char) == 0 {
                // 558: name: typeof(_213) = *const {l328} i8
                // 558: name:   l328 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0" ... _char: typeof(_214) = *const {l330} i8
                // 558: b"locsmaxeff\0" ... _char:   l330 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0" ... st u8: typeof(_215) = *const {l332} u8
                // 558: b"locsmaxeff\0" ... st u8:   l332 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0": typeof(_216) = *const {l334} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 558: b"locsmaxeff\0":   l334 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0": typeof(_217) = & {l336} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 558: b"locsmaxeff\0":   l336 = UNIQUE | NON_NULL, FIXED
                // 558: b"locsmaxeff\0": typeof(_217 = const b"locsmaxeff\x00") = & {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 558: b"locsmaxeff\0":   l552 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0" ... st u8: typeof(_215 = move _216 as *const u8 (Pointer(ArrayToPointer))) = *const {l554} u8
                // 558: b"locsmaxeff\0" ... st u8:   l554 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0" ... _char: typeof(_214 = move _215 as *const i8 (Misc)) = *const {l555} i8
                // 558: b"locsmaxeff\0" ... _char:   l555 = UNIQUE | NON_NULL, (empty)
                // 558: b"locsmaxeff\0": typeof(_216 = &raw const (*_217)) = *const {l553} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 558: b"locsmaxeff\0":   l553 = UNIQUE | NON_NULL, (empty)
                    max = 10 as libc::c_int * min;
                }
                if strcmp(name, b"locsrtc\0" as *const u8 as *const libc::c_char) == 0 {
                // 561: name: typeof(_224) = *const {l344} i8
                // 561: name:   l344 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0" as ... _char: typeof(_225) = *const {l346} i8
                // 561: b"locsrtc\0" as ... _char:   l346 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0" as ... st u8: typeof(_226) = *const {l348} u8
                // 561: b"locsrtc\0" as ... st u8:   l348 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0": typeof(_227) = *const {l350} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 561: b"locsrtc\0":   l350 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0": typeof(_228) = & {l352} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 561: b"locsrtc\0":   l352 = UNIQUE | NON_NULL, FIXED
                // 561: b"locsrtc\0" as ... _char: typeof(_225 = move _226 as *const i8 (Misc)) = *const {l559} i8
                // 561: b"locsrtc\0" as ... _char:   l559 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0": typeof(_228 = const b"locsrtc\x00") = & {l556} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 561: b"locsrtc\0":   l556 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0" as ... st u8: typeof(_226 = move _227 as *const u8 (Pointer(ArrayToPointer))) = *const {l558} u8
                // 561: b"locsrtc\0" as ... st u8:   l558 = UNIQUE | NON_NULL, (empty)
                // 561: b"locsrtc\0": typeof(_227 = &raw const (*_228)) = *const {l557} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 561: b"locsrtc\0":   l557 = UNIQUE | NON_NULL, (empty)
                    max = 0 as libc::c_int;
                }
                while pick(
                    &mut rng,
                    // 565: &mut rng: typeof(_234) = *mut {l359} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 565: &mut rng:   l359 = UNIQUE | NON_NULL, (empty)
                    // 565: &mut rng: typeof(_235) = &mut {l361} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 565: &mut rng:   l361 = UNIQUE | NON_NULL, (empty)
                    // 565: &mut rng: typeof(_234 = &raw mut (*_235)) = *mut {l561} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 565: &mut rng:   l561 = UNIQUE | NON_NULL, (empty)
                    // 565: &mut rng: typeof(_235 = &mut _10) = &mut {l560} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 565: &mut rng:   l560 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int as libc::c_uint,
                    1 as libc::c_int as libc::c_uint,
                ) != 0
                    && val < max
                {
                    if val > 2147483647 as libc::c_int / 2 as libc::c_int {
                        break;
                    }
                    if val < 4 as libc::c_int {
                        val += 1;
                        val;
                    } else {
                        val *= 2 as libc::c_int;
                    }
                }
                if val > max {
                    val = max;
                }
            } else {
                while pick(
                    &mut rng,
                    // 586: &mut rng: typeof(_272) = *mut {l399} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 586: &mut rng:   l399 = UNIQUE | NON_NULL, (empty)
                    // 586: &mut rng: typeof(_273) = &mut {l401} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 586: &mut rng:   l401 = UNIQUE | NON_NULL, (empty)
                    // 586: &mut rng: typeof(_272 = &raw mut (*_273)) = *mut {l563} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 586: &mut rng:   l563 = UNIQUE | NON_NULL, (empty)
                    // 586: &mut rng: typeof(_273 = &mut _10) = &mut {l562} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 586: &mut rng:   l562 = UNIQUE | NON_NULL, (empty)
                    0 as libc::c_int as libc::c_uint,
                    1 as libc::c_int as libc::c_uint,
                ) != 0
                    && val > min
                {
                    if val > 0 as libc::c_int {
                        val /= 2 as libc::c_int;
                    } else if val > -(4 as libc::c_int) {
                        val -= 1;
                        val;
                    } else {
                        if val
                            < (-(2147483647 as libc::c_int) - 1 as libc::c_int) / 2 as libc::c_int
                        {
                            break;
                        }
                        val *= 2 as libc::c_int;
                    }
                }
                if val < min {
                    val = min;
                }
            }
            lglsetopt((*data).lgl, name, val);
            // 610: (*data).lgl: typeof(_322) = *mut {l451} LGL
            // 610: (*data).lgl:   l451 = UNIQUE | NON_NULL, (empty)
            // 610: name: typeof(_323) = *const {l453} i8
            // 610: name:   l453 = UNIQUE | NON_NULL, (empty)
        }
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 616: Some( cnf as un ... id, ): typeof(_325) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l456} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l457} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 616: Some( cnf as un ... id, ):   l456 = UNIQUE | NON_NULL, (empty)
    // 616: Some( cnf as un ... id, ):   l457 = UNIQUE | NON_NULL, (empty)
    // 616: Some( cnf as un ... id, ): typeof(_325 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _326)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l566} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l567} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 616: Some( cnf as un ... id, ):   l566 = UNIQUE | NON_NULL, (empty)
    // 616: Some( cnf as un ... id, ):   l567 = UNIQUE | NON_NULL, (empty)
        cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 617: cnf as unsafe e ... _void: typeof(_326) = fn(*mut {l459} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l460} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 617: cnf as unsafe e ... _void:   l459 = UNIQUE | NON_NULL, (empty)
        // 617: cnf as unsafe e ... _void:   l460 = UNIQUE | NON_NULL, (empty)
        // 617: cnf: typeof(_326 = cnf as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l564} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l565} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 617: cnf:   l564 = UNIQUE | NON_NULL, (empty)
        // 617: cnf:   l565 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn cnf(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 620: *mut libc::c_void: typeof(_0) = *mut {g25} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 620: *mut libc::c_void:   g25 = UNIQUE | NON_NULL, FIXED
// 620: mut data: typeof(_1) = *mut {g24} DefId(0:299 ~ lglmbt[b165]::Data)
// 620: mut data:   g24 = UNIQUE | NON_NULL, FIXED
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(if (*data).c < (*data).n {
    // 624: if (*data).c <  ... id) }: typeof(_4) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l4} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 624: if (*data).c <  ... id) }:   l4 = UNIQUE | NON_NULL, (empty)
    // 624: if (*data).c <  ... id) }:   l5 = UNIQUE | NON_NULL, (empty)
        Some(unit as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
        // 625: unit as unsafe  ... _void: typeof(_8) = fn(*mut {l10} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 625: unit as unsafe  ... _void:   l10 = UNIQUE | NON_NULL, (empty)
        // 625: unit as unsafe  ... _void:   l11 = UNIQUE | NON_NULL, (empty)
        // 625: unit: typeof(_8 = unit as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l16} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l17} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 625: unit:   l16 = UNIQUE | NON_NULL, (empty)
        // 625: unit:   l17 = UNIQUE | NON_NULL, (empty)
        // 625: Some(unit as un ... void): typeof(_4 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _8)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l18} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l19} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 625: Some(unit as un ... void):   l18 = UNIQUE | NON_NULL, (empty)
        // 625: Some(unit as un ... void):   l19 = UNIQUE | NON_NULL, (empty)
    } else {
        Some(lkhd as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void)
        // 627: lkhd as unsafe  ... _void: typeof(_9) = fn(*mut {l13} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 627: lkhd as unsafe  ... _void:   l13 = UNIQUE | NON_NULL, (empty)
        // 627: lkhd as unsafe  ... _void:   l14 = UNIQUE | NON_NULL, (empty)
        // 627: Some(lkhd as un ... void): typeof(_4 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _9)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l22} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l23} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 627: Some(lkhd as un ... void):   l22 = UNIQUE | NON_NULL, (empty)
        // 627: Some(lkhd as un ... void):   l23 = UNIQUE | NON_NULL, (empty)
        // 627: lkhd: typeof(_9 = lkhd as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l20} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l21} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 627: lkhd:   l20 = UNIQUE | NON_NULL, (empty)
        // 627: lkhd:   l21 = UNIQUE | NON_NULL, (empty)
    });
}
unsafe extern "C" fn lit(mut data: *mut Data, mut r: *mut RNG) -> libc::c_int {
// 630: mut data: typeof(_1) = *mut {g14} DefId(0:299 ~ lglmbt[b165]::Data)
// 630: mut data:   g14 = UNIQUE | NON_NULL, FIXED
// 630: mut r: typeof(_2) = *mut {g15} DefId(0:321 ~ lglmbt[b165]::RNG)
// 630: mut r:   g15 = UNIQUE | NON_NULL, FIXED
    let mut pos: libc::c_int = pick(
        r,
        // 632: r: typeof(_5) = *mut {l5} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 632: r:   l5 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
    );
    let mut res: libc::c_int = *((*data).available).offset(pos as isize);
    // 636: ((*data).availa ... size): typeof(_14) = *mut {l15} i32
    // 636: ((*data).availa ... size):   l15 = UNIQUE | NON_NULL, (empty)
    // 636: ((*data).available): typeof(_15) = *mut {l17} i32
    // 636: ((*data).available):   l17 = UNIQUE | NON_NULL, (empty)
    if (0 as libc::c_int) < res && res <= (*data).m {
    } else {
        __assert_fail(
            b"0 < res && res <= data->m\0" as *const u8 as *const libc::c_char,
            // 640: b"0 < res && re ... _char: typeof(_28) = *const {l31} i8
            // 640: b"0 < res && re ... _char:   l31 = UNIQUE | NON_NULL, (empty)
            // 640: b"0 < res && re ... st u8: typeof(_29) = *const {l33} u8
            // 640: b"0 < res && re ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
            // 640: b"0 < res && re ... >m\0": typeof(_30) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 640: b"0 < res && re ... >m\0":   l35 = UNIQUE | NON_NULL, (empty)
            // 640: b"0 < res && re ... >m\0": typeof(_31) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 640: b"0 < res && re ... >m\0":   l37 = UNIQUE | NON_NULL, FIXED
            // 640: b"0 < res && re ... >m\0": typeof(_31 = const b"0 < res && res <= data->m\x00") = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 640: b"0 < res && re ... >m\0":   l112 = UNIQUE | NON_NULL, (empty)
            // 640: b"0 < res && re ... _char: typeof(_28 = move _29 as *const i8 (Misc)) = *const {l115} i8
            // 640: b"0 < res && re ... _char:   l115 = UNIQUE | NON_NULL, (empty)
            // 640: b"0 < res && re ... st u8: typeof(_29 = move _30 as *const u8 (Pointer(ArrayToPointer))) = *const {l114} u8
            // 640: b"0 < res && re ... st u8:   l114 = UNIQUE | NON_NULL, (empty)
            // 640: b"0 < res && re ... >m\0": typeof(_30 = &raw const (*_31)) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 640: b"0 < res && re ... >m\0":   l113 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 641: b"lglmbt.c\0" a ... _char: typeof(_32) = *const {l39} i8
            // 641: b"lglmbt.c\0" a ... _char:   l39 = UNIQUE | NON_NULL, (empty)
            // 641: b"lglmbt.c\0" a ... st u8: typeof(_33) = *const {l41} u8
            // 641: b"lglmbt.c\0" a ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
            // 641: b"lglmbt.c\0": typeof(_34) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 641: b"lglmbt.c\0":   l43 = UNIQUE | NON_NULL, (empty)
            // 641: b"lglmbt.c\0": typeof(_35) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 641: b"lglmbt.c\0":   l45 = UNIQUE | NON_NULL, FIXED
            // 641: b"lglmbt.c\0": typeof(_34 = &raw const (*_35)) = *const {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 641: b"lglmbt.c\0":   l117 = UNIQUE | NON_NULL, (empty)
            // 641: b"lglmbt.c\0": typeof(_35 = const b"lglmbt.c\x00") = & {l116} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 641: b"lglmbt.c\0":   l116 = UNIQUE | NON_NULL, (empty)
            // 641: b"lglmbt.c\0" a ... _char: typeof(_32 = move _33 as *const i8 (Misc)) = *const {l119} i8
            // 641: b"lglmbt.c\0" a ... _char:   l119 = UNIQUE | NON_NULL, (empty)
            // 641: b"lglmbt.c\0" a ... st u8: typeof(_33 = move _34 as *const u8 (Pointer(ArrayToPointer))) = *const {l118} u8
            // 641: b"lglmbt.c\0" a ... st u8:   l118 = UNIQUE | NON_NULL, (empty)
            198 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
            // 643: (*::core::mem:: ... ptr(): typeof(_38) = *const {l49} i8
            // 643: (*::core::mem:: ... ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 643: (*::core::mem:: ... ptr(): typeof(_39) = & {l51} [i8]
            // 643: (*::core::mem:: ... ptr():   l51 = UNIQUE | NON_NULL, FIXED
            // 643: (*::core::mem:: ... ptr(): typeof(_40) = & {l53} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 643: (*::core::mem:: ... ptr():   l53 = UNIQUE | NON_NULL, (empty)
            // 643: ::core::mem::tr ... 0", ): typeof(_41) = & {l55} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 643: ::core::mem::tr ... 0", ):   l55 = UNIQUE | NON_NULL, FIXED
            // 643: (*::core::mem:: ... ptr(): typeof(_40 = &(*_41)) = & {l122} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
            // 643: (*::core::mem:: ... ptr():   l122 = UNIQUE | NON_NULL, (empty)
            // 643: (*::core::mem:: ... ptr(): typeof(_39 = move _40 as &[i8] (Pointer(Unsize))) = & {l123} [i8]
            // 643: (*::core::mem:: ... ptr():   l123 = UNIQUE | NON_NULL, FIXED
                b"int lit(Data *, RNG *)\0",
                // 644: b"int lit(Data  ... *)\0": typeof(_42) = & {l57} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 644: b"int lit(Data  ... *)\0":   l57 = UNIQUE | NON_NULL, (empty)
                // 644: b"int lit(Data  ... *)\0": typeof(_43) = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 644: b"int lit(Data  ... *)\0":   l59 = UNIQUE | NON_NULL, FIXED
                // 644: b"int lit(Data  ... *)\0": typeof(_42 = &(*_43)) = & {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 644: b"int lit(Data  ... *)\0":   l121 = UNIQUE | NON_NULL, (empty)
                // 644: b"int lit(Data  ... *)\0": typeof(_43 = const b"int lit(Data *, RNG *)\x00") = & {l120} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 644: b"int lit(Data  ... *)\0":   l120 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_6496: {
        if (0 as libc::c_int) < res && res <= (*data).m {
        } else {
            __assert_fail(
                b"0 < res && res <= data->m\0" as *const u8 as *const libc::c_char,
                // 653: b"0 < res && re ... _char: typeof(_54) = *const {l71} i8
                // 653: b"0 < res && re ... _char:   l71 = UNIQUE | NON_NULL, (empty)
                // 653: b"0 < res && re ... st u8: typeof(_55) = *const {l73} u8
                // 653: b"0 < res && re ... st u8:   l73 = UNIQUE | NON_NULL, (empty)
                // 653: b"0 < res && re ... >m\0": typeof(_56) = *const {l75} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                // 653: b"0 < res && re ... >m\0":   l75 = UNIQUE | NON_NULL, (empty)
                // 653: b"0 < res && re ... >m\0": typeof(_57) = & {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                // 653: b"0 < res && re ... >m\0":   l77 = UNIQUE | NON_NULL, FIXED
                // 653: b"0 < res && re ... _char: typeof(_54 = move _55 as *const i8 (Misc)) = *const {l127} i8
                // 653: b"0 < res && re ... _char:   l127 = UNIQUE | NON_NULL, (empty)
                // 653: b"0 < res && re ... st u8: typeof(_55 = move _56 as *const u8 (Pointer(ArrayToPointer))) = *const {l126} u8
                // 653: b"0 < res && re ... st u8:   l126 = UNIQUE | NON_NULL, (empty)
                // 653: b"0 < res && re ... >m\0": typeof(_56 = &raw const (*_57)) = *const {l125} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                // 653: b"0 < res && re ... >m\0":   l125 = UNIQUE | NON_NULL, (empty)
                // 653: b"0 < res && re ... >m\0": typeof(_57 = const b"0 < res && res <= data->m\x00") = & {l124} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
                // 653: b"0 < res && re ... >m\0":   l124 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 654: b"lglmbt.c\0" a ... _char: typeof(_58) = *const {l79} i8
                // 654: b"lglmbt.c\0" a ... _char:   l79 = UNIQUE | NON_NULL, (empty)
                // 654: b"lglmbt.c\0" a ... st u8: typeof(_59) = *const {l81} u8
                // 654: b"lglmbt.c\0" a ... st u8:   l81 = UNIQUE | NON_NULL, (empty)
                // 654: b"lglmbt.c\0": typeof(_60) = *const {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 654: b"lglmbt.c\0":   l83 = UNIQUE | NON_NULL, (empty)
                // 654: b"lglmbt.c\0": typeof(_61) = & {l85} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 654: b"lglmbt.c\0":   l85 = UNIQUE | NON_NULL, FIXED
                // 654: b"lglmbt.c\0": typeof(_61 = const b"lglmbt.c\x00") = & {l128} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 654: b"lglmbt.c\0":   l128 = UNIQUE | NON_NULL, (empty)
                // 654: b"lglmbt.c\0": typeof(_60 = &raw const (*_61)) = *const {l129} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 654: b"lglmbt.c\0":   l129 = UNIQUE | NON_NULL, (empty)
                // 654: b"lglmbt.c\0" a ... st u8: typeof(_59 = move _60 as *const u8 (Pointer(ArrayToPointer))) = *const {l130} u8
                // 654: b"lglmbt.c\0" a ... st u8:   l130 = UNIQUE | NON_NULL, (empty)
                // 654: b"lglmbt.c\0" a ... _char: typeof(_58 = move _59 as *const i8 (Misc)) = *const {l131} i8
                // 654: b"lglmbt.c\0" a ... _char:   l131 = UNIQUE | NON_NULL, (empty)
                198 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                // 656: (*::core::mem:: ... ptr(): typeof(_64) = *const {l89} i8
                // 656: (*::core::mem:: ... ptr():   l89 = UNIQUE | NON_NULL, (empty)
                // 656: (*::core::mem:: ... ptr(): typeof(_65) = & {l91} [i8]
                // 656: (*::core::mem:: ... ptr():   l91 = UNIQUE | NON_NULL, FIXED
                // 656: (*::core::mem:: ... ptr(): typeof(_66) = & {l93} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 656: (*::core::mem:: ... ptr():   l93 = UNIQUE | NON_NULL, (empty)
                // 656: ::core::mem::tr ... 0", ): typeof(_67) = & {l95} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 656: ::core::mem::tr ... 0", ):   l95 = UNIQUE | NON_NULL, FIXED
                // 656: (*::core::mem:: ... ptr(): typeof(_65 = move _66 as &[i8] (Pointer(Unsize))) = & {l135} [i8]
                // 656: (*::core::mem:: ... ptr():   l135 = UNIQUE | NON_NULL, FIXED
                // 656: (*::core::mem:: ... ptr(): typeof(_66 = &(*_67)) = & {l134} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                // 656: (*::core::mem:: ... ptr():   l134 = UNIQUE | NON_NULL, (empty)
                    b"int lit(Data *, RNG *)\0",
                    // 657: b"int lit(Data  ... *)\0": typeof(_68) = & {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                    // 657: b"int lit(Data  ... *)\0":   l97 = UNIQUE | NON_NULL, (empty)
                    // 657: b"int lit(Data  ... *)\0": typeof(_69) = & {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                    // 657: b"int lit(Data  ... *)\0":   l99 = UNIQUE | NON_NULL, FIXED
                    // 657: b"int lit(Data  ... *)\0": typeof(_68 = &(*_69)) = & {l133} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                    // 657: b"int lit(Data  ... *)\0":   l133 = UNIQUE | NON_NULL, (empty)
                    // 657: b"int lit(Data  ... *)\0": typeof(_69 = const b"int lit(Data *, RNG *)\x00") = & {l132} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                    // 657: b"int lit(Data  ... *)\0":   l132 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    if pick(
        r,
        // 664: r: typeof(_73) = *mut {l104} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 664: r:   l104 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
    ) != 0
    {
        res = -res;
    }
    return res;
}
unsafe extern "C" fn unit(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 673: *mut libc::c_void: typeof(_0) = *mut {g23} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 673: *mut libc::c_void:   g23 = UNIQUE | NON_NULL, FIXED
// 673: mut data: typeof(_1) = *mut {g22} DefId(0:299 ~ lglmbt[b165]::Data)
// 673: mut data:   g22 = UNIQUE | NON_NULL, FIXED
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    // 675: (*data).lgl: typeof(_7) = *mut {l7} LGL
    // 675: (*data).lgl:   l7 = UNIQUE | NON_NULL, (empty)
    // 675: data: typeof(_9) = *mut {l10} DefId(0:299 ~ lglmbt[b165]::Data)
    // 675: data:   l10 = UNIQUE | NON_NULL, (empty)
    // 675: &mut rng: typeof(_10) = *mut {l12} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 675: &mut rng:   l12 = UNIQUE | NON_NULL, (empty)
    // 675: &mut rng: typeof(_11) = &mut {l14} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 675: &mut rng:   l14 = UNIQUE | NON_NULL, (empty)
    // 675: &mut rng: typeof(_11 = &mut _4) = &mut {l40} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 675: &mut rng:   l40 = UNIQUE | NON_NULL, (empty)
    // 675: &mut rng: typeof(_10 = &raw mut (*_11)) = *mut {l41} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 675: &mut rng:   l41 = UNIQUE | NON_NULL, (empty)
    if pick(
        &mut rng,
        // 677: &mut rng: typeof(_15) = *mut {l19} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 677: &mut rng:   l19 = UNIQUE | NON_NULL, (empty)
        // 677: &mut rng: typeof(_16) = &mut {l21} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 677: &mut rng:   l21 = UNIQUE | NON_NULL, (empty)
        // 677: &mut rng: typeof(_16 = &mut _4) = &mut {l42} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 677: &mut rng:   l42 = UNIQUE | NON_NULL, (empty)
        // 677: &mut rng: typeof(_15 = &raw mut (*_16)) = *mut {l43} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 677: &mut rng:   l43 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        99 as libc::c_int as libc::c_uint,
    ) == 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
        // 685: Some( clause as ... id, ): typeof(_22) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l28} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 685: Some( clause as ... id, ):   l28 = UNIQUE | NON_NULL, (empty)
        // 685: Some( clause as ... id, ):   l29 = UNIQUE | NON_NULL, (empty)
        // 685: Some( clause as ... id, ): typeof(_22 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _23)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l46} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l47} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 685: Some( clause as ... id, ):   l46 = UNIQUE | NON_NULL, (empty)
        // 685: Some( clause as ... id, ):   l47 = UNIQUE | NON_NULL, (empty)
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
            // 686: clause as unsaf ... _void: typeof(_23) = fn(*mut {l31} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 686: clause as unsaf ... _void:   l31 = UNIQUE | NON_NULL, (empty)
            // 686: clause as unsaf ... _void:   l32 = UNIQUE | NON_NULL, (empty)
            // 686: clause: typeof(_23 = clause as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l44} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l45} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 686: clause:   l44 = UNIQUE | NON_NULL, (empty)
            // 686: clause:   l45 = UNIQUE | NON_NULL, (empty)
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 692: Some( binary as ... id, ): typeof(_24) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l34} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 692: Some( binary as ... id, ):   l34 = UNIQUE | NON_NULL, (empty)
    // 692: Some( binary as ... id, ):   l35 = UNIQUE | NON_NULL, (empty)
    // 692: Some( binary as ... id, ): typeof(_24 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _25)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l50} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 692: Some( binary as ... id, ):   l50 = UNIQUE | NON_NULL, (empty)
    // 692: Some( binary as ... id, ):   l51 = UNIQUE | NON_NULL, (empty)
        binary as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 693: binary as unsaf ... _void: typeof(_25) = fn(*mut {l37} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 693: binary as unsaf ... _void:   l37 = UNIQUE | NON_NULL, (empty)
        // 693: binary as unsaf ... _void:   l38 = UNIQUE | NON_NULL, (empty)
        // 693: binary: typeof(_25 = binary as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l48} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l49} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 693: binary:   l48 = UNIQUE | NON_NULL, (empty)
        // 693: binary:   l49 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn binary(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 696: *mut libc::c_void: typeof(_0) = *mut {g21} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 696: *mut libc::c_void:   g21 = UNIQUE | NON_NULL, FIXED
// 696: mut data: typeof(_1) = *mut {g20} DefId(0:299 ~ lglmbt[b165]::Data)
// 696: mut data:   g20 = UNIQUE | NON_NULL, FIXED
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    // 698: (*data).lgl: typeof(_7) = *mut {l7} LGL
    // 698: (*data).lgl:   l7 = UNIQUE | NON_NULL, (empty)
    // 698: data: typeof(_9) = *mut {l10} DefId(0:299 ~ lglmbt[b165]::Data)
    // 698: data:   l10 = UNIQUE | NON_NULL, (empty)
    // 698: &mut rng: typeof(_10) = *mut {l12} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 698: &mut rng:   l12 = UNIQUE | NON_NULL, (empty)
    // 698: &mut rng: typeof(_11) = &mut {l14} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 698: &mut rng:   l14 = UNIQUE | NON_NULL, (empty)
    // 698: &mut rng: typeof(_10 = &raw mut (*_11)) = *mut {l41} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 698: &mut rng:   l41 = UNIQUE | NON_NULL, (empty)
    // 698: &mut rng: typeof(_11 = &mut _4) = &mut {l40} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 698: &mut rng:   l40 = UNIQUE | NON_NULL, (empty)
    if pick(
        &mut rng,
        // 700: &mut rng: typeof(_15) = *mut {l19} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 700: &mut rng:   l19 = UNIQUE | NON_NULL, (empty)
        // 700: &mut rng: typeof(_16) = &mut {l21} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 700: &mut rng:   l21 = UNIQUE | NON_NULL, (empty)
        // 700: &mut rng: typeof(_15 = &raw mut (*_16)) = *mut {l43} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 700: &mut rng:   l43 = UNIQUE | NON_NULL, (empty)
        // 700: &mut rng: typeof(_16 = &mut _4) = &mut {l42} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 700: &mut rng:   l42 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    ) == 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
        // 708: Some( clause as ... id, ): typeof(_22) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l28} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 708: Some( clause as ... id, ):   l28 = UNIQUE | NON_NULL, (empty)
        // 708: Some( clause as ... id, ):   l29 = UNIQUE | NON_NULL, (empty)
        // 708: Some( clause as ... id, ): typeof(_22 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _23)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l46} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l47} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 708: Some( clause as ... id, ):   l46 = UNIQUE | NON_NULL, (empty)
        // 708: Some( clause as ... id, ):   l47 = UNIQUE | NON_NULL, (empty)
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
            // 709: clause as unsaf ... _void: typeof(_23) = fn(*mut {l31} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 709: clause as unsaf ... _void:   l31 = UNIQUE | NON_NULL, (empty)
            // 709: clause as unsaf ... _void:   l32 = UNIQUE | NON_NULL, (empty)
            // 709: clause: typeof(_23 = clause as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l44} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l45} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 709: clause:   l44 = UNIQUE | NON_NULL, (empty)
            // 709: clause:   l45 = UNIQUE | NON_NULL, (empty)
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 715: Some( ternary a ... id, ): typeof(_24) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l34} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 715: Some( ternary a ... id, ):   l34 = UNIQUE | NON_NULL, (empty)
    // 715: Some( ternary a ... id, ):   l35 = UNIQUE | NON_NULL, (empty)
    // 715: Some( ternary a ... id, ): typeof(_24 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _25)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l50} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 715: Some( ternary a ... id, ):   l50 = UNIQUE | NON_NULL, (empty)
    // 715: Some( ternary a ... id, ):   l51 = UNIQUE | NON_NULL, (empty)
        ternary as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 716: ternary as unsa ... _void: typeof(_25) = fn(*mut {l37} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 716: ternary as unsa ... _void:   l37 = UNIQUE | NON_NULL, (empty)
        // 716: ternary as unsa ... _void:   l38 = UNIQUE | NON_NULL, (empty)
        // 716: ternary: typeof(_25 = ternary as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l48} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l49} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 716: ternary:   l48 = UNIQUE | NON_NULL, (empty)
        // 716: ternary:   l49 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn ternary(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 719: *mut libc::c_void: typeof(_0) = *mut {g19} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 719: *mut libc::c_void:   g19 = UNIQUE | NON_NULL, FIXED
// 719: mut data: typeof(_1) = *mut {g18} DefId(0:299 ~ lglmbt[b165]::Data)
// 719: mut data:   g18 = UNIQUE | NON_NULL, FIXED
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    // 721: (*data).lgl: typeof(_7) = *mut {l7} LGL
    // 721: (*data).lgl:   l7 = UNIQUE | NON_NULL, (empty)
    // 721: data: typeof(_9) = *mut {l10} DefId(0:299 ~ lglmbt[b165]::Data)
    // 721: data:   l10 = UNIQUE | NON_NULL, (empty)
    // 721: &mut rng: typeof(_10) = *mut {l12} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 721: &mut rng:   l12 = UNIQUE | NON_NULL, (empty)
    // 721: &mut rng: typeof(_11) = &mut {l14} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 721: &mut rng:   l14 = UNIQUE | NON_NULL, (empty)
    // 721: &mut rng: typeof(_10 = &raw mut (*_11)) = *mut {l41} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 721: &mut rng:   l41 = UNIQUE | NON_NULL, (empty)
    // 721: &mut rng: typeof(_11 = &mut _4) = &mut {l40} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 721: &mut rng:   l40 = UNIQUE | NON_NULL, (empty)
    if pick(
        &mut rng,
        // 723: &mut rng: typeof(_15) = *mut {l19} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 723: &mut rng:   l19 = UNIQUE | NON_NULL, (empty)
        // 723: &mut rng: typeof(_16) = &mut {l21} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 723: &mut rng:   l21 = UNIQUE | NON_NULL, (empty)
        // 723: &mut rng: typeof(_16 = &mut _4) = &mut {l42} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 723: &mut rng:   l42 = UNIQUE | NON_NULL, (empty)
        // 723: &mut rng: typeof(_15 = &raw mut (*_16)) = *mut {l43} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 723: &mut rng:   l43 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
    ) != 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
        // 731: Some( clause as ... id, ): typeof(_22) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l28} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 731: Some( clause as ... id, ):   l28 = UNIQUE | NON_NULL, (empty)
        // 731: Some( clause as ... id, ):   l29 = UNIQUE | NON_NULL, (empty)
        // 731: Some( clause as ... id, ): typeof(_22 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _23)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l46} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l47} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 731: Some( clause as ... id, ):   l46 = UNIQUE | NON_NULL, (empty)
        // 731: Some( clause as ... id, ):   l47 = UNIQUE | NON_NULL, (empty)
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
            // 732: clause as unsaf ... _void: typeof(_23) = fn(*mut {l31} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 732: clause as unsaf ... _void:   l31 = UNIQUE | NON_NULL, (empty)
            // 732: clause as unsaf ... _void:   l32 = UNIQUE | NON_NULL, (empty)
            // 732: clause: typeof(_23 = clause as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l44} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l45} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 732: clause:   l44 = UNIQUE | NON_NULL, (empty)
            // 732: clause:   l45 = UNIQUE | NON_NULL, (empty)
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 738: Some( rest as u ... id, ): typeof(_24) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l34} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 738: Some( rest as u ... id, ):   l34 = UNIQUE | NON_NULL, (empty)
    // 738: Some( rest as u ... id, ):   l35 = UNIQUE | NON_NULL, (empty)
    // 738: Some( rest as u ... id, ): typeof(_24 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _25)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l50} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 738: Some( rest as u ... id, ):   l50 = UNIQUE | NON_NULL, (empty)
    // 738: Some( rest as u ... id, ):   l51 = UNIQUE | NON_NULL, (empty)
        rest as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 739: rest as unsafe  ... _void: typeof(_25) = fn(*mut {l37} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 739: rest as unsafe  ... _void:   l37 = UNIQUE | NON_NULL, (empty)
        // 739: rest as unsafe  ... _void:   l38 = UNIQUE | NON_NULL, (empty)
        // 739: rest: typeof(_25 = rest as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l48} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l49} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 739: rest:   l48 = UNIQUE | NON_NULL, (empty)
        // 739: rest:   l49 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn rest(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 742: *mut libc::c_void: typeof(_0) = *mut {g17} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 742: *mut libc::c_void:   g17 = UNIQUE | NON_NULL, FIXED
// 742: mut data: typeof(_1) = *mut {g16} DefId(0:299 ~ lglmbt[b165]::Data)
// 742: mut data:   g16 = UNIQUE | NON_NULL, FIXED
    let mut rng: RNG = initrng(r);
    lgladd((*data).lgl, lit(data, &mut rng));
    // 744: (*data).lgl: typeof(_7) = *mut {l7} LGL
    // 744: (*data).lgl:   l7 = UNIQUE | NON_NULL, (empty)
    // 744: data: typeof(_9) = *mut {l10} DefId(0:299 ~ lglmbt[b165]::Data)
    // 744: data:   l10 = UNIQUE | NON_NULL, (empty)
    // 744: &mut rng: typeof(_10) = *mut {l12} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 744: &mut rng:   l12 = UNIQUE | NON_NULL, (empty)
    // 744: &mut rng: typeof(_11) = &mut {l14} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 744: &mut rng:   l14 = UNIQUE | NON_NULL, (empty)
    // 744: &mut rng: typeof(_11 = &mut _4) = &mut {l40} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 744: &mut rng:   l40 = UNIQUE | NON_NULL, (empty)
    // 744: &mut rng: typeof(_10 = &raw mut (*_11)) = *mut {l41} DefId(0:321 ~ lglmbt[b165]::RNG)
    // 744: &mut rng:   l41 = UNIQUE | NON_NULL, (empty)
    if pick(
        &mut rng,
        // 746: &mut rng: typeof(_15) = *mut {l19} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 746: &mut rng:   l19 = UNIQUE | NON_NULL, (empty)
        // 746: &mut rng: typeof(_16) = &mut {l21} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 746: &mut rng:   l21 = UNIQUE | NON_NULL, (empty)
        // 746: &mut rng: typeof(_16 = &mut _4) = &mut {l42} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 746: &mut rng:   l42 = UNIQUE | NON_NULL, (empty)
        // 746: &mut rng: typeof(_15 = &raw mut (*_16)) = *mut {l43} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 746: &mut rng:   l43 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        3 as libc::c_int as libc::c_uint,
    ) != 0
    {
        return ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
            *mut libc::c_void,
        >(Some(
        // 754: Some( clause as ... id, ): typeof(_22) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l28} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 754: Some( clause as ... id, ):   l28 = UNIQUE | NON_NULL, (empty)
        // 754: Some( clause as ... id, ):   l29 = UNIQUE | NON_NULL, (empty)
        // 754: Some( clause as ... id, ): typeof(_22 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _23)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l46} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l47} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 754: Some( clause as ... id, ):   l46 = UNIQUE | NON_NULL, (empty)
        // 754: Some( clause as ... id, ):   l47 = UNIQUE | NON_NULL, (empty)
            clause as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
            // 755: clause as unsaf ... _void: typeof(_23) = fn(*mut {l31} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 755: clause as unsaf ... _void:   l31 = UNIQUE | NON_NULL, (empty)
            // 755: clause as unsaf ... _void:   l32 = UNIQUE | NON_NULL, (empty)
            // 755: clause: typeof(_23 = clause as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l44} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l45} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 755: clause:   l44 = UNIQUE | NON_NULL, (empty)
            // 755: clause:   l45 = UNIQUE | NON_NULL, (empty)
        ));
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 761: Some( rest as u ... id, ): typeof(_24) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l34} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 761: Some( rest as u ... id, ):   l34 = UNIQUE | NON_NULL, (empty)
    // 761: Some( rest as u ... id, ):   l35 = UNIQUE | NON_NULL, (empty)
    // 761: Some( rest as u ... id, ): typeof(_24 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _25)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l50} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 761: Some( rest as u ... id, ):   l50 = UNIQUE | NON_NULL, (empty)
    // 761: Some( rest as u ... id, ):   l51 = UNIQUE | NON_NULL, (empty)
        rest as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 762: rest as unsafe  ... _void: typeof(_25) = fn(*mut {l37} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 762: rest as unsafe  ... _void:   l37 = UNIQUE | NON_NULL, (empty)
        // 762: rest as unsafe  ... _void:   l38 = UNIQUE | NON_NULL, (empty)
        // 762: rest: typeof(_25 = rest as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l48} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l49} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 762: rest:   l48 = UNIQUE | NON_NULL, (empty)
        // 762: rest:   l49 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn clause(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 765: *mut libc::c_void: typeof(_0) = *mut {g13} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 765: *mut libc::c_void:   g13 = UNIQUE | NON_NULL, FIXED
// 765: mut data: typeof(_1) = *mut {g12} DefId(0:299 ~ lglmbt[b165]::Data)
// 765: mut data:   g12 = UNIQUE | NON_NULL, FIXED
    lgladd((*data).lgl, 0 as libc::c_int);
    // 766: (*data).lgl: typeof(_5) = *mut {l5} LGL
    // 766: (*data).lgl:   l5 = UNIQUE | NON_NULL, (empty)
    (*data).c += 1 as libc::c_int;
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 771: Some( cnf as un ... id, ): typeof(_9) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l10} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 771: Some( cnf as un ... id, ):   l10 = UNIQUE | NON_NULL, (empty)
    // 771: Some( cnf as un ... id, ):   l11 = UNIQUE | NON_NULL, (empty)
    // 771: Some( cnf as un ... id, ): typeof(_9 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _10)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l18} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l19} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 771: Some( cnf as un ... id, ):   l18 = UNIQUE | NON_NULL, (empty)
    // 771: Some( cnf as un ... id, ):   l19 = UNIQUE | NON_NULL, (empty)
        cnf as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 772: cnf as unsafe e ... _void: typeof(_10) = fn(*mut {l13} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 772: cnf as unsafe e ... _void:   l13 = UNIQUE | NON_NULL, (empty)
        // 772: cnf as unsafe e ... _void:   l14 = UNIQUE | NON_NULL, (empty)
        // 772: cnf: typeof(_10 = cnf as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l16} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l17} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 772: cnf:   l16 = UNIQUE | NON_NULL, (empty)
        // 772: cnf:   l17 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn gcd(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if a > 0 as libc::c_int && b > 0 as libc::c_int {
    } else {
        __assert_fail(
            b"a > 0 && b > 0\0" as *const u8 as *const libc::c_char,
            // 780: b"a > 0 && b >  ... _char: typeof(_15) = *const {l15} i8
            // 780: b"a > 0 && b >  ... _char:   l15 = UNIQUE | NON_NULL, (empty)
            // 780: b"a > 0 && b >  ... st u8: typeof(_16) = *const {l17} u8
            // 780: b"a > 0 && b >  ... st u8:   l17 = UNIQUE | NON_NULL, (empty)
            // 780: b"a > 0 && b > 0\0": typeof(_17) = *const {l19} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
            // 780: b"a > 0 && b > 0\0":   l19 = UNIQUE | NON_NULL, (empty)
            // 780: b"a > 0 && b > 0\0": typeof(_18) = & {l21} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
            // 780: b"a > 0 && b > 0\0":   l21 = UNIQUE | NON_NULL, FIXED
            // 780: b"a > 0 && b >  ... _char: typeof(_15 = move _16 as *const i8 (Misc)) = *const {l104} i8
            // 780: b"a > 0 && b >  ... _char:   l104 = UNIQUE | NON_NULL, (empty)
            // 780: b"a > 0 && b > 0\0": typeof(_18 = const b"a > 0 && b > 0\x00") = & {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
            // 780: b"a > 0 && b > 0\0":   l101 = UNIQUE | NON_NULL, (empty)
            // 780: b"a > 0 && b > 0\0": typeof(_17 = &raw const (*_18)) = *const {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
            // 780: b"a > 0 && b > 0\0":   l102 = UNIQUE | NON_NULL, (empty)
            // 780: b"a > 0 && b >  ... st u8: typeof(_16 = move _17 as *const u8 (Pointer(ArrayToPointer))) = *const {l103} u8
            // 780: b"a > 0 && b >  ... st u8:   l103 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 781: b"lglmbt.c\0" a ... _char: typeof(_19) = *const {l23} i8
            // 781: b"lglmbt.c\0" a ... _char:   l23 = UNIQUE | NON_NULL, (empty)
            // 781: b"lglmbt.c\0" a ... st u8: typeof(_20) = *const {l25} u8
            // 781: b"lglmbt.c\0" a ... st u8:   l25 = UNIQUE | NON_NULL, (empty)
            // 781: b"lglmbt.c\0": typeof(_21) = *const {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 781: b"lglmbt.c\0":   l27 = UNIQUE | NON_NULL, (empty)
            // 781: b"lglmbt.c\0": typeof(_22) = & {l29} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 781: b"lglmbt.c\0":   l29 = UNIQUE | NON_NULL, FIXED
            // 781: b"lglmbt.c\0" a ... st u8: typeof(_20 = move _21 as *const u8 (Pointer(ArrayToPointer))) = *const {l107} u8
            // 781: b"lglmbt.c\0" a ... st u8:   l107 = UNIQUE | NON_NULL, (empty)
            // 781: b"lglmbt.c\0": typeof(_22 = const b"lglmbt.c\x00") = & {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 781: b"lglmbt.c\0":   l105 = UNIQUE | NON_NULL, (empty)
            // 781: b"lglmbt.c\0" a ... _char: typeof(_19 = move _20 as *const i8 (Misc)) = *const {l108} i8
            // 781: b"lglmbt.c\0" a ... _char:   l108 = UNIQUE | NON_NULL, (empty)
            // 781: b"lglmbt.c\0": typeof(_21 = &raw const (*_22)) = *const {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 781: b"lglmbt.c\0":   l106 = UNIQUE | NON_NULL, (empty)
            240 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"int gcd(int, int)\0"))
            // 783: (*::core::mem:: ... ptr(): typeof(_25) = *const {l33} i8
            // 783: (*::core::mem:: ... ptr():   l33 = UNIQUE | NON_NULL, (empty)
            // 783: (*::core::mem:: ... ptr(): typeof(_26) = & {l35} [i8]
            // 783: (*::core::mem:: ... ptr():   l35 = UNIQUE | NON_NULL, FIXED
            // 783: (*::core::mem:: ... ptr(): typeof(_27) = & {l37} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: (*::core::mem:: ... ptr():   l37 = UNIQUE | NON_NULL, (empty)
            // 783: ::core::mem::tr ... )\0"): typeof(_28) = & {l39} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: ::core::mem::tr ... )\0"):   l39 = UNIQUE | NON_NULL, FIXED
            // 783: b"int gcd(int,  ... t)\0": typeof(_29) = & {l41} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: b"int gcd(int,  ... t)\0":   l41 = UNIQUE | NON_NULL, (empty)
            // 783: b"int gcd(int,  ... t)\0": typeof(_30) = & {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: b"int gcd(int,  ... t)\0":   l43 = UNIQUE | NON_NULL, FIXED
            // 783: (*::core::mem:: ... ptr(): typeof(_26 = move _27 as &[i8] (Pointer(Unsize))) = & {l112} [i8]
            // 783: (*::core::mem:: ... ptr():   l112 = UNIQUE | NON_NULL, FIXED
            // 783: (*::core::mem:: ... ptr(): typeof(_27 = &(*_28)) = & {l111} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: (*::core::mem:: ... ptr():   l111 = UNIQUE | NON_NULL, (empty)
            // 783: b"int gcd(int,  ... t)\0": typeof(_30 = const b"int gcd(int, int)\x00") = & {l109} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: b"int gcd(int,  ... t)\0":   l109 = UNIQUE | NON_NULL, (empty)
            // 783: b"int gcd(int,  ... t)\0": typeof(_29 = &(*_30)) = & {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 783: b"int gcd(int,  ... t)\0":   l110 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_6163: {
        if a > 0 as libc::c_int && b > 0 as libc::c_int {
        } else {
            __assert_fail(
                b"a > 0 && b > 0\0" as *const u8 as *const libc::c_char,
                // 791: b"a > 0 && b >  ... _char: typeof(_41) = *const {l55} i8
                // 791: b"a > 0 && b >  ... _char:   l55 = UNIQUE | NON_NULL, (empty)
                // 791: b"a > 0 && b >  ... st u8: typeof(_42) = *const {l57} u8
                // 791: b"a > 0 && b >  ... st u8:   l57 = UNIQUE | NON_NULL, (empty)
                // 791: b"a > 0 && b > 0\0": typeof(_43) = *const {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 791: b"a > 0 && b > 0\0":   l59 = UNIQUE | NON_NULL, (empty)
                // 791: b"a > 0 && b > 0\0": typeof(_44) = & {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 791: b"a > 0 && b > 0\0":   l61 = UNIQUE | NON_NULL, FIXED
                // 791: b"a > 0 && b >  ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l116} i8
                // 791: b"a > 0 && b >  ... _char:   l116 = UNIQUE | NON_NULL, (empty)
                // 791: b"a > 0 && b > 0\0": typeof(_43 = &raw const (*_44)) = *const {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 791: b"a > 0 && b > 0\0":   l114 = UNIQUE | NON_NULL, (empty)
                // 791: b"a > 0 && b > 0\0": typeof(_44 = const b"a > 0 && b > 0\x00") = & {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
                // 791: b"a > 0 && b > 0\0":   l113 = UNIQUE | NON_NULL, (empty)
                // 791: b"a > 0 && b >  ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l115} u8
                // 791: b"a > 0 && b >  ... st u8:   l115 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 792: b"lglmbt.c\0" a ... _char: typeof(_45) = *const {l63} i8
                // 792: b"lglmbt.c\0" a ... _char:   l63 = UNIQUE | NON_NULL, (empty)
                // 792: b"lglmbt.c\0" a ... st u8: typeof(_46) = *const {l65} u8
                // 792: b"lglmbt.c\0" a ... st u8:   l65 = UNIQUE | NON_NULL, (empty)
                // 792: b"lglmbt.c\0": typeof(_47) = *const {l67} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 792: b"lglmbt.c\0":   l67 = UNIQUE | NON_NULL, (empty)
                // 792: b"lglmbt.c\0": typeof(_48) = & {l69} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 792: b"lglmbt.c\0":   l69 = UNIQUE | NON_NULL, FIXED
                // 792: b"lglmbt.c\0": typeof(_47 = &raw const (*_48)) = *const {l118} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 792: b"lglmbt.c\0":   l118 = UNIQUE | NON_NULL, (empty)
                // 792: b"lglmbt.c\0" a ... st u8: typeof(_46 = move _47 as *const u8 (Pointer(ArrayToPointer))) = *const {l119} u8
                // 792: b"lglmbt.c\0" a ... st u8:   l119 = UNIQUE | NON_NULL, (empty)
                // 792: b"lglmbt.c\0" a ... _char: typeof(_45 = move _46 as *const i8 (Misc)) = *const {l120} i8
                // 792: b"lglmbt.c\0" a ... _char:   l120 = UNIQUE | NON_NULL, (empty)
                // 792: b"lglmbt.c\0": typeof(_48 = const b"lglmbt.c\x00") = & {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 792: b"lglmbt.c\0":   l117 = UNIQUE | NON_NULL, (empty)
                240 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 18], &[libc::c_char; 18]>(b"int gcd(int, int)\0"))
                // 794: (*::core::mem:: ... ptr(): typeof(_51) = *const {l73} i8
                // 794: (*::core::mem:: ... ptr():   l73 = UNIQUE | NON_NULL, (empty)
                // 794: (*::core::mem:: ... ptr(): typeof(_52) = & {l75} [i8]
                // 794: (*::core::mem:: ... ptr():   l75 = UNIQUE | NON_NULL, FIXED
                // 794: (*::core::mem:: ... ptr(): typeof(_53) = & {l77} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: (*::core::mem:: ... ptr():   l77 = UNIQUE | NON_NULL, (empty)
                // 794: ::core::mem::tr ... )\0"): typeof(_54) = & {l79} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: ::core::mem::tr ... )\0"):   l79 = UNIQUE | NON_NULL, FIXED
                // 794: b"int gcd(int,  ... t)\0": typeof(_55) = & {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: b"int gcd(int,  ... t)\0":   l81 = UNIQUE | NON_NULL, (empty)
                // 794: b"int gcd(int,  ... t)\0": typeof(_56) = & {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: b"int gcd(int,  ... t)\0":   l83 = UNIQUE | NON_NULL, FIXED
                // 794: b"int gcd(int,  ... t)\0": typeof(_56 = const b"int gcd(int, int)\x00") = & {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: b"int gcd(int,  ... t)\0":   l121 = UNIQUE | NON_NULL, (empty)
                // 794: b"int gcd(int,  ... t)\0": typeof(_55 = &(*_56)) = & {l122} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: b"int gcd(int,  ... t)\0":   l122 = UNIQUE | NON_NULL, (empty)
                // 794: (*::core::mem:: ... ptr(): typeof(_53 = &(*_54)) = & {l123} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 794: (*::core::mem:: ... ptr():   l123 = UNIQUE | NON_NULL, (empty)
                // 794: (*::core::mem:: ... ptr(): typeof(_52 = move _53 as &[i8] (Pointer(Unsize))) = & {l124} [i8]
                // 794: (*::core::mem:: ... ptr():   l124 = UNIQUE | NON_NULL, FIXED
                    .as_ptr(),
            );
        }
    };
    while b > 0 as libc::c_int {
        r = a % b;
        a = b;
        b = r;
    }
    return a;
}
unsafe extern "C" fn lkhd(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 806: *mut libc::c_void: typeof(_0) = *mut {g11} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 806: *mut libc::c_void:   g11 = UNIQUE | NON_NULL, FIXED
// 806: mut data: typeof(_1) = *mut {g10} DefId(0:299 ~ lglmbt[b165]::Data)
// 806: mut data:   g10 = UNIQUE | NON_NULL, FIXED
    let mut rng: RNG = initrng(r);
    if pick(
        &mut rng,
        // 809: &mut rng: typeof(_9) = *mut {l9} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 809: &mut rng:   l9 = UNIQUE | NON_NULL, (empty)
        // 809: &mut rng: typeof(_10) = &mut {l11} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 809: &mut rng:   l11 = UNIQUE | NON_NULL, (empty)
        // 809: &mut rng: typeof(_10 = &mut _4) = &mut {l26} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 809: &mut rng:   l26 = UNIQUE | NON_NULL, (empty)
        // 809: &mut rng: typeof(_9 = &raw mut (*_10)) = *mut {l27} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 809: &mut rng:   l27 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglookahead((*data).lgl);
        // 814: (*data).lgl: typeof(_16) = *mut {l18} LGL
        // 814: (*data).lgl:   l18 = UNIQUE | NON_NULL, (empty)
    }
    return ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void>,
        *mut libc::c_void,
    >(Some(
    // 819: Some( sat as un ... id, ): typeof(_17) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l20} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l21} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 819: Some( sat as un ... id, ):   l20 = UNIQUE | NON_NULL, (empty)
    // 819: Some( sat as un ... id, ):   l21 = UNIQUE | NON_NULL, (empty)
    // 819: Some( sat as un ... id, ): typeof(_17 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _18)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l30} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l31} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 819: Some( sat as un ... id, ):   l30 = UNIQUE | NON_NULL, (empty)
    // 819: Some( sat as un ... id, ):   l31 = UNIQUE | NON_NULL, (empty)
        sat as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void,
        // 820: sat as unsafe e ... _void: typeof(_18) = fn(*mut {l23} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l24} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 820: sat as unsafe e ... _void:   l23 = UNIQUE | NON_NULL, (empty)
        // 820: sat as unsafe e ... _void:   l24 = UNIQUE | NON_NULL, (empty)
        // 820: sat: typeof(_18 = sat as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l28} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 820: sat:   l28 = UNIQUE | NON_NULL, (empty)
        // 820: sat:   l29 = UNIQUE | NON_NULL, (empty)
    ));
}
unsafe extern "C" fn sat(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 823: *mut libc::c_void: typeof(_0) = *mut {g9} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 823: *mut libc::c_void:   g9 = UNIQUE | NON_NULL, FIXED
// 823: mut data: typeof(_1) = *mut {g8} DefId(0:299 ~ lglmbt[b165]::Data)
// 823: mut data:   g8 = UNIQUE | NON_NULL, FIXED
    let mut res: libc::c_int = 0;
    let mut freeze: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut pos: libc::c_int = 0;
    let mut delta: libc::c_int = 0;
    let mut lit_0: libc::c_int = 0;
    let mut assumed: *mut libc::c_int = 0 as *mut libc::c_int;
    // 830: mut assumed: typeof(_10) = *mut {l10} i32
    // 830: mut assumed:   l10 = UNIQUE | NON_NULL, (empty)
    // 830: 0 as *mut libc: ... c_int: typeof(_10 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l1164} i32
    // 830: 0 as *mut libc: ... c_int:   l1164 = UNIQUE | NON_NULL, (empty)
    let mut nassumed: libc::c_int = 0;
    let mut szassumed: libc::c_int = 0;
    let mut lgl: *mut LGL = (*data).lgl;
    // 833: mut lgl: typeof(_13) = *mut {l14} LGL
    // 833: mut lgl:   l14 = UNIQUE | NON_NULL, (empty)
    let mut next: State =
    // 834: mut next: typeof(_14) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l16} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l17} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 834: mut next:   l16 = UNIQUE | NON_NULL, (empty)
    // 834: mut next:   l17 = UNIQUE | NON_NULL, (empty)
        Some(release as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void);
        // 835: release as unsa ... _void: typeof(_15) = fn(*mut {l19} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 835: release as unsa ... _void:   l19 = UNIQUE | NON_NULL, (empty)
        // 835: release as unsa ... _void:   l20 = UNIQUE | NON_NULL, (empty)
        // 835: Some(release as ... void): typeof(_14 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _15)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1167} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l1168} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 835: Some(release as ... void):   l1167 = UNIQUE | NON_NULL, (empty)
        // 835: Some(release as ... void):   l1168 = UNIQUE | NON_NULL, (empty)
        // 835: release: typeof(_15 = release as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l1165} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l1166} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 835: release:   l1165 = UNIQUE | NON_NULL, (empty)
        // 835: release:   l1166 = UNIQUE | NON_NULL, (empty)
    let mut rng: RNG = RNG { z: 0, w: 0 };
    rng = initrng(r);
    if pick(
        &mut rng,
        // 839: &mut rng: typeof(_22) = *mut {l28} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 839: &mut rng:   l28 = UNIQUE | NON_NULL, (empty)
        // 839: &mut rng: typeof(_23) = &mut {l30} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 839: &mut rng:   l30 = UNIQUE | NON_NULL, (empty)
        // 839: &mut rng: typeof(_23 = &mut _16) = &mut {l1169} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 839: &mut rng:   l1169 = UNIQUE | NON_NULL, (empty)
        // 839: &mut rng: typeof(_22 = &raw mut (*_23)) = *mut {l1170} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 839: &mut rng:   l1170 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        500 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglchkclone(lgl);
        // 844: lgl: typeof(_29) = *mut {l37} LGL
        // 844: lgl:   l37 = UNIQUE | NON_NULL, (empty)
    }
    freeze = pick(
        &mut rng,
        // 847: &mut rng: typeof(_31) = *mut {l40} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 847: &mut rng:   l40 = UNIQUE | NON_NULL, (empty)
        // 847: &mut rng: typeof(_32) = &mut {l42} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 847: &mut rng:   l42 = UNIQUE | NON_NULL, (empty)
        // 847: &mut rng: typeof(_31 = &raw mut (*_32)) = *mut {l1172} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 847: &mut rng:   l1172 = UNIQUE | NON_NULL, (empty)
        // 847: &mut rng: typeof(_32 = &mut _16) = &mut {l1171} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 847: &mut rng:   l1171 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
    );
    if freeze != 0 {
        if (*data).navailable > 1 as libc::c_int {
            (*data).nfrozen = pick(
                &mut rng,
                // 854: &mut rng: typeof(_44) = *mut {l55} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 854: &mut rng:   l55 = UNIQUE | NON_NULL, (empty)
                // 854: &mut rng: typeof(_45) = &mut {l57} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 854: &mut rng:   l57 = UNIQUE | NON_NULL, (empty)
                // 854: &mut rng: typeof(_45 = &mut _16) = &mut {l1173} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 854: &mut rng:   l1173 = UNIQUE | NON_NULL, (empty)
                // 854: &mut rng: typeof(_44 = &raw mut (*_45)) = *mut {l1174} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 854: &mut rng:   l1174 = UNIQUE | NON_NULL, (empty)
                (((*data).navailable + 9 as libc::c_int) / 10 as libc::c_int) as libc::c_uint,
                (2 as libc::c_int * ((*data).navailable + 2 as libc::c_int) / 3 as libc::c_int)
                    as libc::c_uint,
            );
            (*data).frozen = malloc(
            // 859: malloc( ((*data ... g), ): typeof(_71) = *mut {l84} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 859: malloc( ((*data ... g), ):   l84 = UNIQUE | NON_NULL, (empty)
            // 859: (*data).frozen  ... c_int: typeof(((*_1).4: *mut i32) = move _71 as *mut i32 (Misc)) = *mut {l1175} i32
            // 859: (*data).frozen  ... c_int:   l1175 = UNIQUE | NON_NULL, (empty)
                ((*data).nfrozen as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            delta = pick(
                &mut rng,
                // 864: &mut rng: typeof(_78) = *mut {l92} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 864: &mut rng:   l92 = UNIQUE | NON_NULL, (empty)
                // 864: &mut rng: typeof(_79) = &mut {l94} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 864: &mut rng:   l94 = UNIQUE | NON_NULL, (empty)
                // 864: &mut rng: typeof(_78 = &raw mut (*_79)) = *mut {l1177} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 864: &mut rng:   l1177 = UNIQUE | NON_NULL, (empty)
                // 864: &mut rng: typeof(_79 = &mut _16) = &mut {l1176} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 864: &mut rng:   l1176 = UNIQUE | NON_NULL, (empty)
                1 as libc::c_int as libc::c_uint,
                ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
            );
            while gcd((*data).navailable, delta) != 1 as libc::c_int {
                delta += 1;
                if delta == (*data).navailable {
                    delta = 1 as libc::c_int;
                }
            }
            pos = pick(
                &mut rng,
                // 875: &mut rng: typeof(_103) = *mut {l119} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 875: &mut rng:   l119 = UNIQUE | NON_NULL, (empty)
                // 875: &mut rng: typeof(_104) = &mut {l121} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 875: &mut rng:   l121 = UNIQUE | NON_NULL, (empty)
                // 875: &mut rng: typeof(_103 = &raw mut (*_104)) = *mut {l1179} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 875: &mut rng:   l1179 = UNIQUE | NON_NULL, (empty)
                // 875: &mut rng: typeof(_104 = &mut _16) = &mut {l1178} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 875: &mut rng:   l1178 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
            );
            i = 0 as libc::c_int;
            while i < (*data).nfrozen {
                if 0 as libc::c_int <= pos && pos < (*data).navailable {
                } else {
                    __assert_fail(
                        b"0 <= pos && pos < data->navailable\0" as *const u8 as *const libc::c_char,
                        // 884: b"0 <= pos && p ... _char: typeof(_127) = *const {l145} i8
                        // 884: b"0 <= pos && p ... _char:   l145 = UNIQUE | NON_NULL, (empty)
                        // 884: b"0 <= pos && p ... st u8: typeof(_128) = *const {l147} u8
                        // 884: b"0 <= pos && p ... st u8:   l147 = UNIQUE | NON_NULL, (empty)
                        // 884: b"0 <= pos && p ... le\0": typeof(_129) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                        // 884: b"0 <= pos && p ... le\0":   l149 = UNIQUE | NON_NULL, (empty)
                        // 884: b"0 <= pos && p ... le\0": typeof(_130) = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                        // 884: b"0 <= pos && p ... le\0":   l151 = UNIQUE | NON_NULL, FIXED
                        // 884: b"0 <= pos && p ... st u8: typeof(_128 = move _129 as *const u8 (Pointer(ArrayToPointer))) = *const {l1182} u8
                        // 884: b"0 <= pos && p ... st u8:   l1182 = UNIQUE | NON_NULL, (empty)
                        // 884: b"0 <= pos && p ... _char: typeof(_127 = move _128 as *const i8 (Misc)) = *const {l1183} i8
                        // 884: b"0 <= pos && p ... _char:   l1183 = UNIQUE | NON_NULL, (empty)
                        // 884: b"0 <= pos && p ... le\0": typeof(_130 = const b"0 <= pos && pos < data->navailable\x00") = & {l1180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                        // 884: b"0 <= pos && p ... le\0":   l1180 = UNIQUE | NON_NULL, (empty)
                        // 884: b"0 <= pos && p ... le\0": typeof(_129 = &raw const (*_130)) = *const {l1181} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                        // 884: b"0 <= pos && p ... le\0":   l1181 = UNIQUE | NON_NULL, (empty)
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        // 885: b"lglmbt.c\0" a ... _char: typeof(_131) = *const {l153} i8
                        // 885: b"lglmbt.c\0" a ... _char:   l153 = UNIQUE | NON_NULL, (empty)
                        // 885: b"lglmbt.c\0" a ... st u8: typeof(_132) = *const {l155} u8
                        // 885: b"lglmbt.c\0" a ... st u8:   l155 = UNIQUE | NON_NULL, (empty)
                        // 885: b"lglmbt.c\0": typeof(_133) = *const {l157} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 885: b"lglmbt.c\0":   l157 = UNIQUE | NON_NULL, (empty)
                        // 885: b"lglmbt.c\0": typeof(_134) = & {l159} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 885: b"lglmbt.c\0":   l159 = UNIQUE | NON_NULL, FIXED
                        // 885: b"lglmbt.c\0" a ... st u8: typeof(_132 = move _133 as *const u8 (Pointer(ArrayToPointer))) = *const {l1186} u8
                        // 885: b"lglmbt.c\0" a ... st u8:   l1186 = UNIQUE | NON_NULL, (empty)
                        // 885: b"lglmbt.c\0": typeof(_134 = const b"lglmbt.c\x00") = & {l1184} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 885: b"lglmbt.c\0":   l1184 = UNIQUE | NON_NULL, (empty)
                        // 885: b"lglmbt.c\0": typeof(_133 = &raw const (*_134)) = *const {l1185} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 885: b"lglmbt.c\0":   l1185 = UNIQUE | NON_NULL, (empty)
                        // 885: b"lglmbt.c\0" a ... _char: typeof(_131 = move _132 as *const i8 (Misc)) = *const {l1187} i8
                        // 885: b"lglmbt.c\0" a ... _char:   l1187 = UNIQUE | NON_NULL, (empty)
                        273 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        // 887: (*::core::mem:: ... ptr(): typeof(_137) = *const {l163} i8
                        // 887: (*::core::mem:: ... ptr():   l163 = UNIQUE | NON_NULL, (empty)
                        // 887: (*::core::mem:: ... ptr(): typeof(_138) = & {l165} [i8]
                        // 887: (*::core::mem:: ... ptr():   l165 = UNIQUE | NON_NULL, FIXED
                        // 887: (*::core::mem:: ... ptr(): typeof(_139) = & {l167} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 887: (*::core::mem:: ... ptr():   l167 = UNIQUE | NON_NULL, (empty)
                        // 887: ::core::mem::tr ... 0", ): typeof(_140) = & {l169} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 887: ::core::mem::tr ... 0", ):   l169 = UNIQUE | NON_NULL, FIXED
                        // 887: (*::core::mem:: ... ptr(): typeof(_138 = move _139 as &[i8] (Pointer(Unsize))) = & {l1191} [i8]
                        // 887: (*::core::mem:: ... ptr():   l1191 = UNIQUE | NON_NULL, FIXED
                        // 887: (*::core::mem:: ... ptr(): typeof(_139 = &(*_140)) = & {l1190} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 887: (*::core::mem:: ... ptr():   l1190 = UNIQUE | NON_NULL, (empty)
                            b"void *sat(Data *, unsigned int)\0",
                            // 888: b"void *sat(Dat ... t)\0": typeof(_141) = & {l171} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 888: b"void *sat(Dat ... t)\0":   l171 = UNIQUE | NON_NULL, (empty)
                            // 888: b"void *sat(Dat ... t)\0": typeof(_142) = & {l173} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 888: b"void *sat(Dat ... t)\0":   l173 = UNIQUE | NON_NULL, FIXED
                            // 888: b"void *sat(Dat ... t)\0": typeof(_141 = &(*_142)) = & {l1189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 888: b"void *sat(Dat ... t)\0":   l1189 = UNIQUE | NON_NULL, (empty)
                            // 888: b"void *sat(Dat ... t)\0": typeof(_142 = const b"void *sat(Data *, unsigned int)\x00") = & {l1188} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 888: b"void *sat(Dat ... t)\0":   l1188 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
                'c_6026: {
                    if 0 as libc::c_int <= pos && pos < (*data).navailable {
                    } else {
                        __assert_fail(
                            b"0 <= pos && pos < data->navailable\0" as *const u8
                            // 897: b"0 <= pos && p ... _char: typeof(_153) = *const {l185} i8
                            // 897: b"0 <= pos && p ... _char:   l185 = UNIQUE | NON_NULL, (empty)
                            // 897: b"0 <= pos && p ... st u8: typeof(_154) = *const {l187} u8
                            // 897: b"0 <= pos && p ... st u8:   l187 = UNIQUE | NON_NULL, (empty)
                            // 897: b"0 <= pos && p ... le\0": typeof(_155) = *const {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                            // 897: b"0 <= pos && p ... le\0":   l189 = UNIQUE | NON_NULL, (empty)
                            // 897: b"0 <= pos && p ... le\0": typeof(_156) = & {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                            // 897: b"0 <= pos && p ... le\0":   l191 = UNIQUE | NON_NULL, FIXED
                            // 897: b"0 <= pos && p ... le\0": typeof(_156 = const b"0 <= pos && pos < data->navailable\x00") = & {l1192} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                            // 897: b"0 <= pos && p ... le\0":   l1192 = UNIQUE | NON_NULL, (empty)
                            // 897: b"0 <= pos && p ... le\0": typeof(_155 = &raw const (*_156)) = *const {l1193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000023)) }]
                            // 897: b"0 <= pos && p ... le\0":   l1193 = UNIQUE | NON_NULL, (empty)
                            // 897: b"0 <= pos && p ... st u8: typeof(_154 = move _155 as *const u8 (Pointer(ArrayToPointer))) = *const {l1194} u8
                            // 897: b"0 <= pos && p ... st u8:   l1194 = UNIQUE | NON_NULL, (empty)
                            // 897: b"0 <= pos && p ... _char: typeof(_153 = move _154 as *const i8 (Misc)) = *const {l1195} i8
                            // 897: b"0 <= pos && p ... _char:   l1195 = UNIQUE | NON_NULL, (empty)
                                as *const libc::c_char,
                            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                            // 899: b"lglmbt.c\0" a ... _char: typeof(_157) = *const {l193} i8
                            // 899: b"lglmbt.c\0" a ... _char:   l193 = UNIQUE | NON_NULL, (empty)
                            // 899: b"lglmbt.c\0" a ... st u8: typeof(_158) = *const {l195} u8
                            // 899: b"lglmbt.c\0" a ... st u8:   l195 = UNIQUE | NON_NULL, (empty)
                            // 899: b"lglmbt.c\0": typeof(_159) = *const {l197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 899: b"lglmbt.c\0":   l197 = UNIQUE | NON_NULL, (empty)
                            // 899: b"lglmbt.c\0": typeof(_160) = & {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 899: b"lglmbt.c\0":   l199 = UNIQUE | NON_NULL, FIXED
                            // 899: b"lglmbt.c\0": typeof(_159 = &raw const (*_160)) = *const {l1197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 899: b"lglmbt.c\0":   l1197 = UNIQUE | NON_NULL, (empty)
                            // 899: b"lglmbt.c\0" a ... st u8: typeof(_158 = move _159 as *const u8 (Pointer(ArrayToPointer))) = *const {l1198} u8
                            // 899: b"lglmbt.c\0" a ... st u8:   l1198 = UNIQUE | NON_NULL, (empty)
                            // 899: b"lglmbt.c\0": typeof(_160 = const b"lglmbt.c\x00") = & {l1196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 899: b"lglmbt.c\0":   l1196 = UNIQUE | NON_NULL, (empty)
                            // 899: b"lglmbt.c\0" a ... _char: typeof(_157 = move _158 as *const i8 (Misc)) = *const {l1199} i8
                            // 899: b"lglmbt.c\0" a ... _char:   l1199 = UNIQUE | NON_NULL, (empty)
                            273 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                            // 901: (*::core::mem:: ... ptr(): typeof(_163) = *const {l203} i8
                            // 901: (*::core::mem:: ... ptr():   l203 = UNIQUE | NON_NULL, (empty)
                            // 901: (*::core::mem:: ... ptr(): typeof(_164) = & {l205} [i8]
                            // 901: (*::core::mem:: ... ptr():   l205 = UNIQUE | NON_NULL, FIXED
                            // 901: (*::core::mem:: ... ptr(): typeof(_165) = & {l207} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 901: (*::core::mem:: ... ptr():   l207 = UNIQUE | NON_NULL, (empty)
                            // 901: ::core::mem::tr ... 0", ): typeof(_166) = & {l209} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 901: ::core::mem::tr ... 0", ):   l209 = UNIQUE | NON_NULL, FIXED
                            // 901: (*::core::mem:: ... ptr(): typeof(_164 = move _165 as &[i8] (Pointer(Unsize))) = & {l1203} [i8]
                            // 901: (*::core::mem:: ... ptr():   l1203 = UNIQUE | NON_NULL, FIXED
                            // 901: (*::core::mem:: ... ptr(): typeof(_165 = &(*_166)) = & {l1202} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 901: (*::core::mem:: ... ptr():   l1202 = UNIQUE | NON_NULL, (empty)
                                b"void *sat(Data *, unsigned int)\0",
                                // 902: b"void *sat(Dat ... t)\0": typeof(_167) = & {l211} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                // 902: b"void *sat(Dat ... t)\0":   l211 = UNIQUE | NON_NULL, (empty)
                                // 902: b"void *sat(Dat ... t)\0": typeof(_168) = & {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                // 902: b"void *sat(Dat ... t)\0":   l213 = UNIQUE | NON_NULL, FIXED
                                // 902: b"void *sat(Dat ... t)\0": typeof(_168 = const b"void *sat(Data *, unsigned int)\x00") = & {l1200} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                // 902: b"void *sat(Dat ... t)\0":   l1200 = UNIQUE | NON_NULL, (empty)
                                // 902: b"void *sat(Dat ... t)\0": typeof(_167 = &(*_168)) = & {l1201} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                // 902: b"void *sat(Dat ... t)\0":   l1201 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                };
                lit_0 = *((*data).available).offset(pos as isize);
                // 908: ((*data).availa ... size): typeof(_170) = *mut {l216} i32
                // 908: ((*data).availa ... size):   l216 = UNIQUE | NON_NULL, (empty)
                // 908: ((*data).available): typeof(_171) = *mut {l218} i32
                // 908: ((*data).available):   l218 = UNIQUE | NON_NULL, (empty)
                *((*data).frozen).offset(i as isize) = lit_0;
                // 909: ((*data).frozen ... size): typeof(_175) = *mut {l223} i32
                // 909: ((*data).frozen ... size):   l223 = UNIQUE | NON_NULL, (empty)
                // 909: ((*data).frozen): typeof(_176) = *mut {l225} i32
                // 909: ((*data).frozen):   l225 = UNIQUE | NON_NULL, (empty)
                pos += delta;
                if pos >= (*data).navailable {
                    pos -= (*data).navailable;
                }
                i += 1;
                i;
            }
            i = 0 as libc::c_int;
            while i < (*data).nfrozen {
                lglfreeze(lgl, *((*data).frozen).offset(i as isize));
                // 919: lgl: typeof(_197) = *mut {l247} LGL
                // 919: lgl:   l247 = UNIQUE | NON_NULL, (empty)
                // 919: ((*data).frozen ... size): typeof(_199) = *mut {l250} i32
                // 919: ((*data).frozen ... size):   l250 = UNIQUE | NON_NULL, (empty)
                // 919: ((*data).frozen): typeof(_200) = *mut {l252} i32
                // 919: ((*data).frozen):   l252 = UNIQUE | NON_NULL, (empty)
                i += 1;
                i;
            }
        } else if (*data).navailable == 1 as libc::c_int {
            (*data).nfrozen = 1 as libc::c_int;
            (*data).frozen =
            // 925: (*data).frozen  ... c_int: typeof(((*_1).4: *mut i32) = move _212 as *mut i32 (Misc)) = *mut {l1204} i32
            // 925: (*data).frozen  ... c_int:   l1204 = UNIQUE | NON_NULL, (empty)
                malloc(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) as *mut libc::c_int;
                // 926: malloc(::core:: ... long): typeof(_212) = *mut {l265} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 926: malloc(::core:: ... long):   l265 = UNIQUE | NON_NULL, (empty)
            *((*data).frozen).offset(0 as libc::c_int as isize) =
            // 927: ((*data).frozen ... size): typeof(_220) = *mut {l276} i32
            // 927: ((*data).frozen ... size):   l276 = UNIQUE | NON_NULL, (empty)
            // 927: ((*data).frozen): typeof(_221) = *mut {l278} i32
            // 927: ((*data).frozen):   l278 = UNIQUE | NON_NULL, (empty)
                *((*data).available).offset(0 as libc::c_int as isize);
                // 928: ((*data).availa ... size): typeof(_216) = *mut {l270} i32
                // 928: ((*data).availa ... size):   l270 = UNIQUE | NON_NULL, (empty)
                // 928: ((*data).available): typeof(_217) = *mut {l272} i32
                // 928: ((*data).available):   l272 = UNIQUE | NON_NULL, (empty)
        } else {
            (*data).nfrozen = 0 as libc::c_int;
            (*data).frozen = 0 as *mut libc::c_int;
            // 931: (*data).frozen  ... c_int: typeof(((*_1).4: *mut i32) = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l1205} i32
            // 931: (*data).frozen  ... c_int:   l1205 = UNIQUE | NON_NULL, (empty)
        }
    }
    if (*data).navailable != 0
        && pick(
            &mut rng,
            // 936: &mut rng: typeof(_231) = *mut {l289} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 936: &mut rng:   l289 = UNIQUE | NON_NULL, (empty)
            // 936: &mut rng: typeof(_232) = &mut {l291} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 936: &mut rng:   l291 = UNIQUE | NON_NULL, (empty)
            // 936: &mut rng: typeof(_231 = &raw mut (*_232)) = *mut {l1207} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 936: &mut rng:   l1207 = UNIQUE | NON_NULL, (empty)
            // 936: &mut rng: typeof(_232 = &mut _16) = &mut {l1206} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 936: &mut rng:   l1206 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
        ) == 0
    {
        nassumed = 0 as libc::c_int;
        szassumed = 1 as libc::c_int;
        assumed = malloc(
        // 943: malloc( (szassu ... g), ): typeof(_239) = *mut {l299} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 943: malloc( (szassu ... g), ):   l299 = UNIQUE | NON_NULL, (empty)
        // 943: assumed = mallo ... c_int: typeof(_10 = move _239 as *mut i32 (Misc)) = *mut {l1208} i32
        // 943: assumed = mallo ... c_int:   l1208 = UNIQUE | NON_NULL, (empty)
            (szassumed as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        loop {
            if nassumed == szassumed {
                szassumed *= 2 as libc::c_int;
                assumed = realloc(
                // 950: realloc( assume ... g), ): typeof(_251) = *mut {l312} DefId(2:5583 ~ core[480a]::ffi::c_void)
                // 950: realloc( assume ... g), ):   l312 = UNIQUE | NON_NULL, (empty)
                // 950: assumed = reall ... c_int: typeof(_10 = move _251 as *mut i32 (Misc)) = *mut {l1210} i32
                // 950: assumed = reall ... c_int:   l1210 = UNIQUE | NON_NULL, (empty)
                    assumed as *mut libc::c_void,
                    // 951: assumed as *mut ... _void: typeof(_252) = *mut {l314} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 951: assumed as *mut ... _void:   l314 = UNIQUE | NON_NULL, (empty)
                    // 951: assumed: typeof(_253) = *mut {l316} i32
                    // 951: assumed:   l316 = UNIQUE | NON_NULL, (empty)
                    // 951: assumed as *mut ... _void: typeof(_252 = move _253 as *mut libc::c_void (Misc)) = *mut {l1209} DefId(2:5583 ~ core[480a]::ffi::c_void)
                    // 951: assumed as *mut ... _void:   l1209 = UNIQUE | NON_NULL, (empty)
                    (szassumed as libc::c_ulong)
                        .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
                ) as *mut libc::c_int;
            }
            pos = pick(
                &mut rng,
                // 957: &mut rng: typeof(_260) = *mut {l324} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 957: &mut rng:   l324 = UNIQUE | NON_NULL, (empty)
                // 957: &mut rng: typeof(_261) = &mut {l326} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 957: &mut rng:   l326 = UNIQUE | NON_NULL, (empty)
                // 957: &mut rng: typeof(_261 = &mut _16) = &mut {l1211} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 957: &mut rng:   l1211 = UNIQUE | NON_NULL, (empty)
                // 957: &mut rng: typeof(_260 = &raw mut (*_261)) = *mut {l1212} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 957: &mut rng:   l1212 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
            );
            lit_0 = *((*data).available).offset(pos as isize);
            // 961: ((*data).availa ... size): typeof(_270) = *mut {l336} i32
            // 961: ((*data).availa ... size):   l336 = UNIQUE | NON_NULL, (empty)
            // 961: ((*data).available): typeof(_271) = *mut {l338} i32
            // 961: ((*data).available):   l338 = UNIQUE | NON_NULL, (empty)
            if pick(
                &mut rng,
                // 963: &mut rng: typeof(_277) = *mut {l345} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 963: &mut rng:   l345 = UNIQUE | NON_NULL, (empty)
                // 963: &mut rng: typeof(_278) = &mut {l347} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 963: &mut rng:   l347 = UNIQUE | NON_NULL, (empty)
                // 963: &mut rng: typeof(_278 = &mut _16) = &mut {l1213} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 963: &mut rng:   l1213 = UNIQUE | NON_NULL, (empty)
                // 963: &mut rng: typeof(_277 = &raw mut (*_278)) = *mut {l1214} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 963: &mut rng:   l1214 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                lit_0 = -lit_0;
            }
            lglassume(lgl, lit_0);
            // 970: lgl: typeof(_286) = *mut {l356} LGL
            // 970: lgl:   l356 = UNIQUE | NON_NULL, (empty)
            if nassumed < szassumed {
            } else {
                __assert_fail(
                    b"nassumed < szassumed\0" as *const u8 as *const libc::c_char,
                    // 974: b"nassumed < sz ... _char: typeof(_294) = *const {l365} i8
                    // 974: b"nassumed < sz ... _char:   l365 = UNIQUE | NON_NULL, (empty)
                    // 974: b"nassumed < sz ... st u8: typeof(_295) = *const {l367} u8
                    // 974: b"nassumed < sz ... st u8:   l367 = UNIQUE | NON_NULL, (empty)
                    // 974: b"nassumed < sz ... ed\0": typeof(_296) = *const {l369} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 974: b"nassumed < sz ... ed\0":   l369 = UNIQUE | NON_NULL, (empty)
                    // 974: b"nassumed < sz ... ed\0": typeof(_297) = & {l371} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 974: b"nassumed < sz ... ed\0":   l371 = UNIQUE | NON_NULL, FIXED
                    // 974: b"nassumed < sz ... ed\0": typeof(_296 = &raw const (*_297)) = *const {l1216} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 974: b"nassumed < sz ... ed\0":   l1216 = UNIQUE | NON_NULL, (empty)
                    // 974: b"nassumed < sz ... _char: typeof(_294 = move _295 as *const i8 (Misc)) = *const {l1218} i8
                    // 974: b"nassumed < sz ... _char:   l1218 = UNIQUE | NON_NULL, (empty)
                    // 974: b"nassumed < sz ... ed\0": typeof(_297 = const b"nassumed < szassumed\x00") = & {l1215} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                    // 974: b"nassumed < sz ... ed\0":   l1215 = UNIQUE | NON_NULL, (empty)
                    // 974: b"nassumed < sz ... st u8: typeof(_295 = move _296 as *const u8 (Pointer(ArrayToPointer))) = *const {l1217} u8
                    // 974: b"nassumed < sz ... st u8:   l1217 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 975: b"lglmbt.c\0" a ... _char: typeof(_298) = *const {l373} i8
                    // 975: b"lglmbt.c\0" a ... _char:   l373 = UNIQUE | NON_NULL, (empty)
                    // 975: b"lglmbt.c\0" a ... st u8: typeof(_299) = *const {l375} u8
                    // 975: b"lglmbt.c\0" a ... st u8:   l375 = UNIQUE | NON_NULL, (empty)
                    // 975: b"lglmbt.c\0": typeof(_300) = *const {l377} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 975: b"lglmbt.c\0":   l377 = UNIQUE | NON_NULL, (empty)
                    // 975: b"lglmbt.c\0": typeof(_301) = & {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 975: b"lglmbt.c\0":   l379 = UNIQUE | NON_NULL, FIXED
                    // 975: b"lglmbt.c\0": typeof(_300 = &raw const (*_301)) = *const {l1220} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 975: b"lglmbt.c\0":   l1220 = UNIQUE | NON_NULL, (empty)
                    // 975: b"lglmbt.c\0" a ... _char: typeof(_298 = move _299 as *const i8 (Misc)) = *const {l1222} i8
                    // 975: b"lglmbt.c\0" a ... _char:   l1222 = UNIQUE | NON_NULL, (empty)
                    // 975: b"lglmbt.c\0": typeof(_301 = const b"lglmbt.c\x00") = & {l1219} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 975: b"lglmbt.c\0":   l1219 = UNIQUE | NON_NULL, (empty)
                    // 975: b"lglmbt.c\0" a ... st u8: typeof(_299 = move _300 as *const u8 (Pointer(ArrayToPointer))) = *const {l1221} u8
                    // 975: b"lglmbt.c\0" a ... st u8:   l1221 = UNIQUE | NON_NULL, (empty)
                    312 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 977: (*::core::mem:: ... ptr(): typeof(_304) = *const {l383} i8
                    // 977: (*::core::mem:: ... ptr():   l383 = UNIQUE | NON_NULL, (empty)
                    // 977: (*::core::mem:: ... ptr(): typeof(_305) = & {l385} [i8]
                    // 977: (*::core::mem:: ... ptr():   l385 = UNIQUE | NON_NULL, FIXED
                    // 977: (*::core::mem:: ... ptr(): typeof(_306) = & {l387} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 977: (*::core::mem:: ... ptr():   l387 = UNIQUE | NON_NULL, (empty)
                    // 977: ::core::mem::tr ... 0", ): typeof(_307) = & {l389} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 977: ::core::mem::tr ... 0", ):   l389 = UNIQUE | NON_NULL, FIXED
                    // 977: (*::core::mem:: ... ptr(): typeof(_305 = move _306 as &[i8] (Pointer(Unsize))) = & {l1226} [i8]
                    // 977: (*::core::mem:: ... ptr():   l1226 = UNIQUE | NON_NULL, FIXED
                    // 977: (*::core::mem:: ... ptr(): typeof(_306 = &(*_307)) = & {l1225} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 977: (*::core::mem:: ... ptr():   l1225 = UNIQUE | NON_NULL, (empty)
                        b"void *sat(Data *, unsigned int)\0",
                        // 978: b"void *sat(Dat ... t)\0": typeof(_308) = & {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 978: b"void *sat(Dat ... t)\0":   l391 = UNIQUE | NON_NULL, (empty)
                        // 978: b"void *sat(Dat ... t)\0": typeof(_309) = & {l393} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 978: b"void *sat(Dat ... t)\0":   l393 = UNIQUE | NON_NULL, FIXED
                        // 978: b"void *sat(Dat ... t)\0": typeof(_308 = &(*_309)) = & {l1224} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 978: b"void *sat(Dat ... t)\0":   l1224 = UNIQUE | NON_NULL, (empty)
                        // 978: b"void *sat(Dat ... t)\0": typeof(_309 = const b"void *sat(Data *, unsigned int)\x00") = & {l1223} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 978: b"void *sat(Dat ... t)\0":   l1223 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
            'c_5735: {
                if nassumed < szassumed {
                } else {
                    __assert_fail(
                        b"nassumed < szassumed\0" as *const u8 as *const libc::c_char,
                        // 987: b"nassumed < sz ... _char: typeof(_316) = *const {l401} i8
                        // 987: b"nassumed < sz ... _char:   l401 = UNIQUE | NON_NULL, (empty)
                        // 987: b"nassumed < sz ... st u8: typeof(_317) = *const {l403} u8
                        // 987: b"nassumed < sz ... st u8:   l403 = UNIQUE | NON_NULL, (empty)
                        // 987: b"nassumed < sz ... ed\0": typeof(_318) = *const {l405} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 987: b"nassumed < sz ... ed\0":   l405 = UNIQUE | NON_NULL, (empty)
                        // 987: b"nassumed < sz ... ed\0": typeof(_319) = & {l407} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 987: b"nassumed < sz ... ed\0":   l407 = UNIQUE | NON_NULL, FIXED
                        // 987: b"nassumed < sz ... st u8: typeof(_317 = move _318 as *const u8 (Pointer(ArrayToPointer))) = *const {l1229} u8
                        // 987: b"nassumed < sz ... st u8:   l1229 = UNIQUE | NON_NULL, (empty)
                        // 987: b"nassumed < sz ... ed\0": typeof(_318 = &raw const (*_319)) = *const {l1228} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 987: b"nassumed < sz ... ed\0":   l1228 = UNIQUE | NON_NULL, (empty)
                        // 987: b"nassumed < sz ... ed\0": typeof(_319 = const b"nassumed < szassumed\x00") = & {l1227} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                        // 987: b"nassumed < sz ... ed\0":   l1227 = UNIQUE | NON_NULL, (empty)
                        // 987: b"nassumed < sz ... _char: typeof(_316 = move _317 as *const i8 (Misc)) = *const {l1230} i8
                        // 987: b"nassumed < sz ... _char:   l1230 = UNIQUE | NON_NULL, (empty)
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        // 988: b"lglmbt.c\0" a ... _char: typeof(_320) = *const {l409} i8
                        // 988: b"lglmbt.c\0" a ... _char:   l409 = UNIQUE | NON_NULL, (empty)
                        // 988: b"lglmbt.c\0" a ... st u8: typeof(_321) = *const {l411} u8
                        // 988: b"lglmbt.c\0" a ... st u8:   l411 = UNIQUE | NON_NULL, (empty)
                        // 988: b"lglmbt.c\0": typeof(_322) = *const {l413} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 988: b"lglmbt.c\0":   l413 = UNIQUE | NON_NULL, (empty)
                        // 988: b"lglmbt.c\0": typeof(_323) = & {l415} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 988: b"lglmbt.c\0":   l415 = UNIQUE | NON_NULL, FIXED
                        // 988: b"lglmbt.c\0": typeof(_322 = &raw const (*_323)) = *const {l1232} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 988: b"lglmbt.c\0":   l1232 = UNIQUE | NON_NULL, (empty)
                        // 988: b"lglmbt.c\0": typeof(_323 = const b"lglmbt.c\x00") = & {l1231} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 988: b"lglmbt.c\0":   l1231 = UNIQUE | NON_NULL, (empty)
                        // 988: b"lglmbt.c\0" a ... _char: typeof(_320 = move _321 as *const i8 (Misc)) = *const {l1234} i8
                        // 988: b"lglmbt.c\0" a ... _char:   l1234 = UNIQUE | NON_NULL, (empty)
                        // 988: b"lglmbt.c\0" a ... st u8: typeof(_321 = move _322 as *const u8 (Pointer(ArrayToPointer))) = *const {l1233} u8
                        // 988: b"lglmbt.c\0" a ... st u8:   l1233 = UNIQUE | NON_NULL, (empty)
                        312 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        // 990: (*::core::mem:: ... ptr(): typeof(_326) = *const {l419} i8
                        // 990: (*::core::mem:: ... ptr():   l419 = UNIQUE | NON_NULL, (empty)
                        // 990: (*::core::mem:: ... ptr(): typeof(_327) = & {l421} [i8]
                        // 990: (*::core::mem:: ... ptr():   l421 = UNIQUE | NON_NULL, FIXED
                        // 990: (*::core::mem:: ... ptr(): typeof(_328) = & {l423} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 990: (*::core::mem:: ... ptr():   l423 = UNIQUE | NON_NULL, (empty)
                        // 990: ::core::mem::tr ... 0", ): typeof(_329) = & {l425} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 990: ::core::mem::tr ... 0", ):   l425 = UNIQUE | NON_NULL, FIXED
                        // 990: (*::core::mem:: ... ptr(): typeof(_327 = move _328 as &[i8] (Pointer(Unsize))) = & {l1238} [i8]
                        // 990: (*::core::mem:: ... ptr():   l1238 = UNIQUE | NON_NULL, FIXED
                        // 990: (*::core::mem:: ... ptr(): typeof(_328 = &(*_329)) = & {l1237} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 990: (*::core::mem:: ... ptr():   l1237 = UNIQUE | NON_NULL, (empty)
                            b"void *sat(Data *, unsigned int)\0",
                            // 991: b"void *sat(Dat ... t)\0": typeof(_330) = & {l427} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 991: b"void *sat(Dat ... t)\0":   l427 = UNIQUE | NON_NULL, (empty)
                            // 991: b"void *sat(Dat ... t)\0": typeof(_331) = & {l429} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 991: b"void *sat(Dat ... t)\0":   l429 = UNIQUE | NON_NULL, FIXED
                            // 991: b"void *sat(Dat ... t)\0": typeof(_330 = &(*_331)) = & {l1236} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 991: b"void *sat(Dat ... t)\0":   l1236 = UNIQUE | NON_NULL, (empty)
                            // 991: b"void *sat(Dat ... t)\0": typeof(_331 = const b"void *sat(Data *, unsigned int)\x00") = & {l1235} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 991: b"void *sat(Dat ... t)\0":   l1235 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
            };
            let fresh0 = nassumed;
            nassumed = nassumed + 1;
            *assumed.offset(fresh0 as isize) = lit_0;
            // 999: assumed.offset( ... size): typeof(_336) = *mut {l435} i32
            // 999: assumed.offset( ... size):   l435 = UNIQUE | NON_NULL, (empty)
            // 999: assumed: typeof(_337) = *mut {l437} i32
            // 999: assumed:   l437 = UNIQUE | NON_NULL, (empty)
            if !(pick(
                &mut rng,
                // 1001: &mut rng: typeof(_343) = *mut {l444} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1001: &mut rng:   l444 = UNIQUE | NON_NULL, (empty)
                // 1001: &mut rng: typeof(_344) = &mut {l446} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1001: &mut rng:   l446 = UNIQUE | NON_NULL, (empty)
                // 1001: &mut rng: typeof(_344 = &mut _16) = &mut {l1239} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1001: &mut rng:   l1239 = UNIQUE | NON_NULL, (empty)
                // 1001: &mut rng: typeof(_343 = &raw mut (*_344)) = *mut {l1240} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1001: &mut rng:   l1240 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                10 as libc::c_int as libc::c_uint,
            ) == 0)
            {
                break;
            }
        }
    } else {
        assumed = 0 as *mut libc::c_int;
        // 1010: assumed = 0 as  ... c_int: typeof(_10 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l1241} i32
        // 1010: assumed = 0 as  ... c_int:   l1241 = UNIQUE | NON_NULL, (empty)
        szassumed = 0 as libc::c_int;
        nassumed = szassumed;
    }
    if pick(
        &mut rng,
        // 1015: &mut rng: typeof(_355) = *mut {l458} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1015: &mut rng:   l458 = UNIQUE | NON_NULL, (empty)
        // 1015: &mut rng: typeof(_356) = &mut {l460} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1015: &mut rng:   l460 = UNIQUE | NON_NULL, (empty)
        // 1015: &mut rng: typeof(_356 = &mut _16) = &mut {l1242} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1015: &mut rng:   l1242 = UNIQUE | NON_NULL, (empty)
        // 1015: &mut rng: typeof(_355 = &raw mut (*_356)) = *mut {l1243} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1015: &mut rng:   l1243 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
    ) == 0
    {
        pos = pick(
            &mut rng,
            // 1021: &mut rng: typeof(_362) = *mut {l467} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1021: &mut rng:   l467 = UNIQUE | NON_NULL, (empty)
            // 1021: &mut rng: typeof(_363) = &mut {l469} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1021: &mut rng:   l469 = UNIQUE | NON_NULL, (empty)
            // 1021: &mut rng: typeof(_363 = &mut _16) = &mut {l1244} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1021: &mut rng:   l1244 = UNIQUE | NON_NULL, (empty)
            // 1021: &mut rng: typeof(_362 = &raw mut (*_363)) = *mut {l1245} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1021: &mut rng:   l1245 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            ((*data).navailable - 1 as libc::c_int) as libc::c_uint,
        );
        lit_0 = *((*data).available).offset(pos as isize);
        // 1025: ((*data).availa ... size): typeof(_372) = *mut {l479} i32
        // 1025: ((*data).availa ... size):   l479 = UNIQUE | NON_NULL, (empty)
        // 1025: ((*data).available): typeof(_373) = *mut {l481} i32
        // 1025: ((*data).available):   l481 = UNIQUE | NON_NULL, (empty)
        if pick(
            &mut rng,
            // 1027: &mut rng: typeof(_379) = *mut {l488} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1027: &mut rng:   l488 = UNIQUE | NON_NULL, (empty)
            // 1027: &mut rng: typeof(_380) = &mut {l490} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1027: &mut rng:   l490 = UNIQUE | NON_NULL, (empty)
            // 1027: &mut rng: typeof(_380 = &mut _16) = &mut {l1246} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1027: &mut rng:   l1246 = UNIQUE | NON_NULL, (empty)
            // 1027: &mut rng: typeof(_379 = &raw mut (*_380)) = *mut {l1247} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1027: &mut rng:   l1247 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            1 as libc::c_int as libc::c_uint,
        ) != 0
        {
            lit_0 = -lit_0;
        }
        if pick(
            &mut rng,
            // 1035: &mut rng: typeof(_390) = *mut {l501} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1035: &mut rng:   l501 = UNIQUE | NON_NULL, (empty)
            // 1035: &mut rng: typeof(_391) = &mut {l503} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1035: &mut rng:   l503 = UNIQUE | NON_NULL, (empty)
            // 1035: &mut rng: typeof(_390 = &raw mut (*_391)) = *mut {l1249} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1035: &mut rng:   l1249 = UNIQUE | NON_NULL, (empty)
            // 1035: &mut rng: typeof(_391 = &mut _16) = &mut {l1248} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1035: &mut rng:   l1248 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglresetphase(lgl, lit_0);
            // 1040: lgl: typeof(_397) = *mut {l510} LGL
            // 1040: lgl:   l510 = UNIQUE | NON_NULL, (empty)
        } else {
            lglsetphase(lgl, lit_0);
            // 1042: lgl: typeof(_400) = *mut {l514} LGL
            // 1042: lgl:   l514 = UNIQUE | NON_NULL, (empty)
        }
        if pick(
            &mut rng,
            // 1045: &mut rng: typeof(_404) = *mut {l519} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1045: &mut rng:   l519 = UNIQUE | NON_NULL, (empty)
            // 1045: &mut rng: typeof(_405) = &mut {l521} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1045: &mut rng:   l521 = UNIQUE | NON_NULL, (empty)
            // 1045: &mut rng: typeof(_405 = &mut _16) = &mut {l1250} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1045: &mut rng:   l1250 = UNIQUE | NON_NULL, (empty)
            // 1045: &mut rng: typeof(_404 = &raw mut (*_405)) = *mut {l1251} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1045: &mut rng:   l1251 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            11 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglsetimportant(lgl, lit_0);
            // 1050: lgl: typeof(_411) = *mut {l528} LGL
            // 1050: lgl:   l528 = UNIQUE | NON_NULL, (empty)
        }
    }
    if pick(
        &mut rng,
        // 1054: &mut rng: typeof(_416) = *mut {l534} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1054: &mut rng:   l534 = UNIQUE | NON_NULL, (empty)
        // 1054: &mut rng: typeof(_417) = &mut {l536} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1054: &mut rng:   l536 = UNIQUE | NON_NULL, (empty)
        // 1054: &mut rng: typeof(_416 = &raw mut (*_417)) = *mut {l1253} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1054: &mut rng:   l1253 = UNIQUE | NON_NULL, (empty)
        // 1054: &mut rng: typeof(_417 = &mut _16) = &mut {l1252} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1054: &mut rng:   l1252 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        100 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglchkclone(lgl);
        // 1059: lgl: typeof(_423) = *mut {l543} LGL
        // 1059: lgl:   l543 = UNIQUE | NON_NULL, (empty)
    }
    if pick(
        &mut rng,
        // 1062: &mut rng: typeof(_427) = *mut {l548} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1062: &mut rng:   l548 = UNIQUE | NON_NULL, (empty)
        // 1062: &mut rng: typeof(_428) = &mut {l550} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1062: &mut rng:   l550 = UNIQUE | NON_NULL, (empty)
        // 1062: &mut rng: typeof(_427 = &raw mut (*_428)) = *mut {l1255} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1062: &mut rng:   l1255 = UNIQUE | NON_NULL, (empty)
        // 1062: &mut rng: typeof(_428 = &mut _16) = &mut {l1254} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1062: &mut rng:   l1254 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        66 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglfixate(lgl);
        // 1067: lgl: typeof(_434) = *mut {l557} LGL
        // 1067: lgl:   l557 = UNIQUE | NON_NULL, (empty)
    }
    if pick(
        &mut rng,
        // 1070: &mut rng: typeof(_438) = *mut {l562} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1070: &mut rng:   l562 = UNIQUE | NON_NULL, (empty)
        // 1070: &mut rng: typeof(_439) = &mut {l564} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1070: &mut rng:   l564 = UNIQUE | NON_NULL, (empty)
        // 1070: &mut rng: typeof(_439 = &mut _16) = &mut {l1256} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1070: &mut rng:   l1256 = UNIQUE | NON_NULL, (empty)
        // 1070: &mut rng: typeof(_438 = &raw mut (*_439)) = *mut {l1257} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1070: &mut rng:   l1257 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        20 as libc::c_int as libc::c_uint,
    ) != 0
    {
        res = lglsat(lgl);
        // 1075: lgl: typeof(_445) = *mut {l571} LGL
        // 1075: lgl:   l571 = UNIQUE | NON_NULL, (empty)
    } else {
        res = lglsimp(
            lgl,
            // 1078: lgl: typeof(_447) = *mut {l574} LGL
            // 1078: lgl:   l574 = UNIQUE | NON_NULL, (empty)
            pick(
                &mut rng,
                // 1080: &mut rng: typeof(_449) = *mut {l577} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1080: &mut rng:   l577 = UNIQUE | NON_NULL, (empty)
                // 1080: &mut rng: typeof(_450) = &mut {l579} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1080: &mut rng:   l579 = UNIQUE | NON_NULL, (empty)
                // 1080: &mut rng: typeof(_450 = &mut _16) = &mut {l1258} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1080: &mut rng:   l1258 = UNIQUE | NON_NULL, (empty)
                // 1080: &mut rng: typeof(_449 = &raw mut (*_450)) = *mut {l1259} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1080: &mut rng:   l1259 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                10 as libc::c_int as libc::c_uint,
            ),
        );
    }
    if res == 0 || res == 10 as libc::c_int || res == 20 as libc::c_int {
    } else {
        __assert_fail(
            b"!res || res == 10 || res == 20\0" as *const u8 as *const libc::c_char,
            // 1089: b"!res || res = ... _char: typeof(_468) = *const {l598} i8
            // 1089: b"!res || res = ... _char:   l598 = UNIQUE | NON_NULL, (empty)
            // 1089: b"!res || res = ... st u8: typeof(_469) = *const {l600} u8
            // 1089: b"!res || res = ... st u8:   l600 = UNIQUE | NON_NULL, (empty)
            // 1089: b"!res || res = ... 20\0": typeof(_470) = *const {l602} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1089: b"!res || res = ... 20\0":   l602 = UNIQUE | NON_NULL, (empty)
            // 1089: b"!res || res = ... 20\0": typeof(_471) = & {l604} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1089: b"!res || res = ... 20\0":   l604 = UNIQUE | NON_NULL, FIXED
            // 1089: b"!res || res = ... 20\0": typeof(_470 = &raw const (*_471)) = *const {l1261} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1089: b"!res || res = ... 20\0":   l1261 = UNIQUE | NON_NULL, (empty)
            // 1089: b"!res || res = ... st u8: typeof(_469 = move _470 as *const u8 (Pointer(ArrayToPointer))) = *const {l1262} u8
            // 1089: b"!res || res = ... st u8:   l1262 = UNIQUE | NON_NULL, (empty)
            // 1089: b"!res || res = ... _char: typeof(_468 = move _469 as *const i8 (Misc)) = *const {l1263} i8
            // 1089: b"!res || res = ... _char:   l1263 = UNIQUE | NON_NULL, (empty)
            // 1089: b"!res || res = ... 20\0": typeof(_471 = const b"!res || res == 10 || res == 20\x00") = & {l1260} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1089: b"!res || res = ... 20\0":   l1260 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1090: b"lglmbt.c\0" a ... _char: typeof(_472) = *const {l606} i8
            // 1090: b"lglmbt.c\0" a ... _char:   l606 = UNIQUE | NON_NULL, (empty)
            // 1090: b"lglmbt.c\0" a ... st u8: typeof(_473) = *const {l608} u8
            // 1090: b"lglmbt.c\0" a ... st u8:   l608 = UNIQUE | NON_NULL, (empty)
            // 1090: b"lglmbt.c\0": typeof(_474) = *const {l610} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1090: b"lglmbt.c\0":   l610 = UNIQUE | NON_NULL, (empty)
            // 1090: b"lglmbt.c\0": typeof(_475) = & {l612} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1090: b"lglmbt.c\0":   l612 = UNIQUE | NON_NULL, FIXED
            // 1090: b"lglmbt.c\0": typeof(_475 = const b"lglmbt.c\x00") = & {l1264} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1090: b"lglmbt.c\0":   l1264 = UNIQUE | NON_NULL, (empty)
            // 1090: b"lglmbt.c\0" a ... _char: typeof(_472 = move _473 as *const i8 (Misc)) = *const {l1267} i8
            // 1090: b"lglmbt.c\0" a ... _char:   l1267 = UNIQUE | NON_NULL, (empty)
            // 1090: b"lglmbt.c\0": typeof(_474 = &raw const (*_475)) = *const {l1265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1090: b"lglmbt.c\0":   l1265 = UNIQUE | NON_NULL, (empty)
            // 1090: b"lglmbt.c\0" a ... st u8: typeof(_473 = move _474 as *const u8 (Pointer(ArrayToPointer))) = *const {l1266} u8
            // 1090: b"lglmbt.c\0" a ... st u8:   l1266 = UNIQUE | NON_NULL, (empty)
            328 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
            // 1092: (*::core::mem:: ... ptr(): typeof(_478) = *const {l616} i8
            // 1092: (*::core::mem:: ... ptr():   l616 = UNIQUE | NON_NULL, (empty)
            // 1092: (*::core::mem:: ... ptr(): typeof(_479) = & {l618} [i8]
            // 1092: (*::core::mem:: ... ptr():   l618 = UNIQUE | NON_NULL, FIXED
            // 1092: (*::core::mem:: ... ptr(): typeof(_480) = & {l620} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
            // 1092: (*::core::mem:: ... ptr():   l620 = UNIQUE | NON_NULL, (empty)
            // 1092: ::core::mem::tr ... 0", ): typeof(_481) = & {l622} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
            // 1092: ::core::mem::tr ... 0", ):   l622 = UNIQUE | NON_NULL, FIXED
            // 1092: (*::core::mem:: ... ptr(): typeof(_480 = &(*_481)) = & {l1270} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
            // 1092: (*::core::mem:: ... ptr():   l1270 = UNIQUE | NON_NULL, (empty)
            // 1092: (*::core::mem:: ... ptr(): typeof(_479 = move _480 as &[i8] (Pointer(Unsize))) = & {l1271} [i8]
            // 1092: (*::core::mem:: ... ptr():   l1271 = UNIQUE | NON_NULL, FIXED
                b"void *sat(Data *, unsigned int)\0",
                // 1093: b"void *sat(Dat ... t)\0": typeof(_482) = & {l624} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1093: b"void *sat(Dat ... t)\0":   l624 = UNIQUE | NON_NULL, (empty)
                // 1093: b"void *sat(Dat ... t)\0": typeof(_483) = & {l626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1093: b"void *sat(Dat ... t)\0":   l626 = UNIQUE | NON_NULL, FIXED
                // 1093: b"void *sat(Dat ... t)\0": typeof(_483 = const b"void *sat(Data *, unsigned int)\x00") = & {l1268} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1093: b"void *sat(Dat ... t)\0":   l1268 = UNIQUE | NON_NULL, (empty)
                // 1093: b"void *sat(Dat ... t)\0": typeof(_482 = &(*_483)) = & {l1269} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1093: b"void *sat(Dat ... t)\0":   l1269 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_5469: {
        if res == 0 || res == 10 as libc::c_int || res == 20 as libc::c_int {
        } else {
            __assert_fail(
                b"!res || res == 10 || res == 20\0" as *const u8 as *const libc::c_char,
                // 1102: b"!res || res = ... _char: typeof(_497) = *const {l641} i8
                // 1102: b"!res || res = ... _char:   l641 = UNIQUE | NON_NULL, (empty)
                // 1102: b"!res || res = ... st u8: typeof(_498) = *const {l643} u8
                // 1102: b"!res || res = ... st u8:   l643 = UNIQUE | NON_NULL, (empty)
                // 1102: b"!res || res = ... 20\0": typeof(_499) = *const {l645} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1102: b"!res || res = ... 20\0":   l645 = UNIQUE | NON_NULL, (empty)
                // 1102: b"!res || res = ... 20\0": typeof(_500) = & {l647} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1102: b"!res || res = ... 20\0":   l647 = UNIQUE | NON_NULL, FIXED
                // 1102: b"!res || res = ... _char: typeof(_497 = move _498 as *const i8 (Misc)) = *const {l1275} i8
                // 1102: b"!res || res = ... _char:   l1275 = UNIQUE | NON_NULL, (empty)
                // 1102: b"!res || res = ... 20\0": typeof(_499 = &raw const (*_500)) = *const {l1273} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1102: b"!res || res = ... 20\0":   l1273 = UNIQUE | NON_NULL, (empty)
                // 1102: b"!res || res = ... st u8: typeof(_498 = move _499 as *const u8 (Pointer(ArrayToPointer))) = *const {l1274} u8
                // 1102: b"!res || res = ... st u8:   l1274 = UNIQUE | NON_NULL, (empty)
                // 1102: b"!res || res = ... 20\0": typeof(_500 = const b"!res || res == 10 || res == 20\x00") = & {l1272} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1102: b"!res || res = ... 20\0":   l1272 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1103: b"lglmbt.c\0" a ... _char: typeof(_501) = *const {l649} i8
                // 1103: b"lglmbt.c\0" a ... _char:   l649 = UNIQUE | NON_NULL, (empty)
                // 1103: b"lglmbt.c\0" a ... st u8: typeof(_502) = *const {l651} u8
                // 1103: b"lglmbt.c\0" a ... st u8:   l651 = UNIQUE | NON_NULL, (empty)
                // 1103: b"lglmbt.c\0": typeof(_503) = *const {l653} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1103: b"lglmbt.c\0":   l653 = UNIQUE | NON_NULL, (empty)
                // 1103: b"lglmbt.c\0": typeof(_504) = & {l655} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1103: b"lglmbt.c\0":   l655 = UNIQUE | NON_NULL, FIXED
                // 1103: b"lglmbt.c\0": typeof(_503 = &raw const (*_504)) = *const {l1277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1103: b"lglmbt.c\0":   l1277 = UNIQUE | NON_NULL, (empty)
                // 1103: b"lglmbt.c\0" a ... st u8: typeof(_502 = move _503 as *const u8 (Pointer(ArrayToPointer))) = *const {l1278} u8
                // 1103: b"lglmbt.c\0" a ... st u8:   l1278 = UNIQUE | NON_NULL, (empty)
                // 1103: b"lglmbt.c\0": typeof(_504 = const b"lglmbt.c\x00") = & {l1276} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1103: b"lglmbt.c\0":   l1276 = UNIQUE | NON_NULL, (empty)
                // 1103: b"lglmbt.c\0" a ... _char: typeof(_501 = move _502 as *const i8 (Misc)) = *const {l1279} i8
                // 1103: b"lglmbt.c\0" a ... _char:   l1279 = UNIQUE | NON_NULL, (empty)
                328 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                // 1105: (*::core::mem:: ... ptr(): typeof(_507) = *const {l659} i8
                // 1105: (*::core::mem:: ... ptr():   l659 = UNIQUE | NON_NULL, (empty)
                // 1105: (*::core::mem:: ... ptr(): typeof(_508) = & {l661} [i8]
                // 1105: (*::core::mem:: ... ptr():   l661 = UNIQUE | NON_NULL, FIXED
                // 1105: (*::core::mem:: ... ptr(): typeof(_509) = & {l663} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1105: (*::core::mem:: ... ptr():   l663 = UNIQUE | NON_NULL, (empty)
                // 1105: ::core::mem::tr ... 0", ): typeof(_510) = & {l665} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1105: ::core::mem::tr ... 0", ):   l665 = UNIQUE | NON_NULL, FIXED
                // 1105: (*::core::mem:: ... ptr(): typeof(_508 = move _509 as &[i8] (Pointer(Unsize))) = & {l1283} [i8]
                // 1105: (*::core::mem:: ... ptr():   l1283 = UNIQUE | NON_NULL, FIXED
                // 1105: (*::core::mem:: ... ptr(): typeof(_509 = &(*_510)) = & {l1282} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1105: (*::core::mem:: ... ptr():   l1282 = UNIQUE | NON_NULL, (empty)
                    b"void *sat(Data *, unsigned int)\0",
                    // 1106: b"void *sat(Dat ... t)\0": typeof(_511) = & {l667} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1106: b"void *sat(Dat ... t)\0":   l667 = UNIQUE | NON_NULL, (empty)
                    // 1106: b"void *sat(Dat ... t)\0": typeof(_512) = & {l669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1106: b"void *sat(Dat ... t)\0":   l669 = UNIQUE | NON_NULL, FIXED
                    // 1106: b"void *sat(Dat ... t)\0": typeof(_511 = &(*_512)) = & {l1281} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1106: b"void *sat(Dat ... t)\0":   l1281 = UNIQUE | NON_NULL, (empty)
                    // 1106: b"void *sat(Dat ... t)\0": typeof(_512 = const b"void *sat(Data *, unsigned int)\x00") = & {l1280} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1106: b"void *sat(Dat ... t)\0":   l1280 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    if res == 10 as libc::c_int {
        if (*data).print != 0 {
            printf(b"sat \0" as *const u8 as *const libc::c_char);
            // 1114: b"sat \0" as *c ... _char: typeof(_521) = *const {l679} i8
            // 1114: b"sat \0" as *c ... _char:   l679 = UNIQUE | NON_NULL, (empty)
            // 1114: b"sat \0" as *c ... st u8: typeof(_522) = *const {l681} u8
            // 1114: b"sat \0" as *c ... st u8:   l681 = UNIQUE | NON_NULL, (empty)
            // 1114: b"sat \0": typeof(_523) = *const {l683} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1114: b"sat \0":   l683 = UNIQUE | NON_NULL, (empty)
            // 1114: b"sat \0": typeof(_524) = & {l685} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1114: b"sat \0":   l685 = UNIQUE | NON_NULL, FIXED
            // 1114: b"sat \0": typeof(_523 = &raw const (*_524)) = *const {l1285} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1114: b"sat \0":   l1285 = UNIQUE | NON_NULL, (empty)
            // 1114: b"sat \0" as *c ... st u8: typeof(_522 = move _523 as *const u8 (Pointer(ArrayToPointer))) = *const {l1286} u8
            // 1114: b"sat \0" as *c ... st u8:   l1286 = UNIQUE | NON_NULL, (empty)
            // 1114: b"sat \0": typeof(_524 = const b"sat \x00") = & {l1284} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1114: b"sat \0":   l1284 = UNIQUE | NON_NULL, (empty)
            // 1114: b"sat \0" as *c ... _char: typeof(_521 = move _522 as *const i8 (Misc)) = *const {l1287} i8
            // 1114: b"sat \0" as *c ... _char:   l1287 = UNIQUE | NON_NULL, (empty)
        }
        if pick(
            &mut rng,
            // 1117: &mut rng: typeof(_528) = *mut {l690} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1117: &mut rng:   l690 = UNIQUE | NON_NULL, (empty)
            // 1117: &mut rng: typeof(_529) = &mut {l692} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1117: &mut rng:   l692 = UNIQUE | NON_NULL, (empty)
            // 1117: &mut rng: typeof(_528 = &raw mut (*_529)) = *mut {l1289} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1117: &mut rng:   l1289 = UNIQUE | NON_NULL, (empty)
            // 1117: &mut rng: typeof(_529 = &mut _16) = &mut {l1288} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1117: &mut rng:   l1288 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglinconsistent(lgl);
            // 1122: lgl: typeof(_535) = *mut {l699} LGL
            // 1122: lgl:   l699 = UNIQUE | NON_NULL, (empty)
        }
        if pick(
            &mut rng,
            // 1125: &mut rng: typeof(_539) = *mut {l704} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1125: &mut rng:   l704 = UNIQUE | NON_NULL, (empty)
            // 1125: &mut rng: typeof(_540) = &mut {l706} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1125: &mut rng:   l706 = UNIQUE | NON_NULL, (empty)
            // 1125: &mut rng: typeof(_539 = &raw mut (*_540)) = *mut {l1291} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1125: &mut rng:   l1291 = UNIQUE | NON_NULL, (empty)
            // 1125: &mut rng: typeof(_540 = &mut _16) = &mut {l1290} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1125: &mut rng:   l1290 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            20 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglsetphases(lgl);
            // 1130: lgl: typeof(_546) = *mut {l713} LGL
            // 1130: lgl:   l713 = UNIQUE | NON_NULL, (empty)
        }
        i = pick(
            &mut rng,
            // 1133: &mut rng: typeof(_548) = *mut {l716} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1133: &mut rng:   l716 = UNIQUE | NON_NULL, (empty)
            // 1133: &mut rng: typeof(_549) = &mut {l718} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1133: &mut rng:   l718 = UNIQUE | NON_NULL, (empty)
            // 1133: &mut rng: typeof(_548 = &raw mut (*_549)) = *mut {l1293} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1133: &mut rng:   l1293 = UNIQUE | NON_NULL, (empty)
            // 1133: &mut rng: typeof(_549 = &mut _16) = &mut {l1292} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1133: &mut rng:   l1292 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            (*data).m as libc::c_uint,
        );
        loop {
            let fresh1 = i;
            i = i - 1;
            if !(fresh1 != 0) {
                break;
            }
            lit_0 = pick(
                &mut rng,
                // 1144: &mut rng: typeof(_564) = *mut {l734} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1144: &mut rng:   l734 = UNIQUE | NON_NULL, (empty)
                // 1144: &mut rng: typeof(_565) = &mut {l736} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1144: &mut rng:   l736 = UNIQUE | NON_NULL, (empty)
                // 1144: &mut rng: typeof(_564 = &raw mut (*_565)) = *mut {l1295} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1144: &mut rng:   l1295 = UNIQUE | NON_NULL, (empty)
                // 1144: &mut rng: typeof(_565 = &mut _16) = &mut {l1294} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1144: &mut rng:   l1294 = UNIQUE | NON_NULL, (empty)
                1 as libc::c_int as libc::c_uint,
                (2 as libc::c_int * (*data).m) as libc::c_uint,
            );
            if pick(
                &mut rng,
                // 1149: &mut rng: typeof(_576) = *mut {l748} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1149: &mut rng:   l748 = UNIQUE | NON_NULL, (empty)
                // 1149: &mut rng: typeof(_577) = &mut {l750} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1149: &mut rng:   l750 = UNIQUE | NON_NULL, (empty)
                // 1149: &mut rng: typeof(_577 = &mut _16) = &mut {l1296} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1149: &mut rng:   l1296 = UNIQUE | NON_NULL, (empty)
                // 1149: &mut rng: typeof(_576 = &raw mut (*_577)) = *mut {l1297} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1149: &mut rng:   l1297 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                1 as libc::c_int as libc::c_uint,
            ) != 0
            {
                lit_0 = -lit_0;
            }
            lglderef(lgl, lit_0);
            // 1156: lgl: typeof(_585) = *mut {l759} LGL
            // 1156: lgl:   l759 = UNIQUE | NON_NULL, (empty)
        }
        if pick(
            &mut rng,
            // 1159: &mut rng: typeof(_590) = *mut {l765} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1159: &mut rng:   l765 = UNIQUE | NON_NULL, (empty)
            // 1159: &mut rng: typeof(_591) = &mut {l767} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1159: &mut rng:   l767 = UNIQUE | NON_NULL, (empty)
            // 1159: &mut rng: typeof(_591 = &mut _16) = &mut {l1298} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1159: &mut rng:   l1298 = UNIQUE | NON_NULL, (empty)
            // 1159: &mut rng: typeof(_590 = &raw mut (*_591)) = *mut {l1299} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1159: &mut rng:   l1299 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            30 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglsetphases(lgl);
            // 1164: lgl: typeof(_597) = *mut {l774} LGL
            // 1164: lgl:   l774 = UNIQUE | NON_NULL, (empty)
        }
        if freeze != 0 {
            if (*data).nfrozen <= (*data).navailable {
            } else {
                __assert_fail(
                    b"data->nfrozen <= data->navailable\0" as *const u8 as *const libc::c_char,
                    // 1170: b"data->nfrozen ... _char: typeof(_607) = *const {l785} i8
                    // 1170: b"data->nfrozen ... _char:   l785 = UNIQUE | NON_NULL, (empty)
                    // 1170: b"data->nfrozen ... st u8: typeof(_608) = *const {l787} u8
                    // 1170: b"data->nfrozen ... st u8:   l787 = UNIQUE | NON_NULL, (empty)
                    // 1170: b"data->nfrozen ... le\0": typeof(_609) = *const {l789} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 1170: b"data->nfrozen ... le\0":   l789 = UNIQUE | NON_NULL, (empty)
                    // 1170: b"data->nfrozen ... le\0": typeof(_610) = & {l791} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 1170: b"data->nfrozen ... le\0":   l791 = UNIQUE | NON_NULL, FIXED
                    // 1170: b"data->nfrozen ... st u8: typeof(_608 = move _609 as *const u8 (Pointer(ArrayToPointer))) = *const {l1302} u8
                    // 1170: b"data->nfrozen ... st u8:   l1302 = UNIQUE | NON_NULL, (empty)
                    // 1170: b"data->nfrozen ... le\0": typeof(_610 = const b"data->nfrozen <= data->navailable\x00") = & {l1300} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 1170: b"data->nfrozen ... le\0":   l1300 = UNIQUE | NON_NULL, (empty)
                    // 1170: b"data->nfrozen ... le\0": typeof(_609 = &raw const (*_610)) = *const {l1301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                    // 1170: b"data->nfrozen ... le\0":   l1301 = UNIQUE | NON_NULL, (empty)
                    // 1170: b"data->nfrozen ... _char: typeof(_607 = move _608 as *const i8 (Misc)) = *const {l1303} i8
                    // 1170: b"data->nfrozen ... _char:   l1303 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1171: b"lglmbt.c\0" a ... _char: typeof(_611) = *const {l793} i8
                    // 1171: b"lglmbt.c\0" a ... _char:   l793 = UNIQUE | NON_NULL, (empty)
                    // 1171: b"lglmbt.c\0" a ... st u8: typeof(_612) = *const {l795} u8
                    // 1171: b"lglmbt.c\0" a ... st u8:   l795 = UNIQUE | NON_NULL, (empty)
                    // 1171: b"lglmbt.c\0": typeof(_613) = *const {l797} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1171: b"lglmbt.c\0":   l797 = UNIQUE | NON_NULL, (empty)
                    // 1171: b"lglmbt.c\0": typeof(_614) = & {l799} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1171: b"lglmbt.c\0":   l799 = UNIQUE | NON_NULL, FIXED
                    // 1171: b"lglmbt.c\0" a ... st u8: typeof(_612 = move _613 as *const u8 (Pointer(ArrayToPointer))) = *const {l1306} u8
                    // 1171: b"lglmbt.c\0" a ... st u8:   l1306 = UNIQUE | NON_NULL, (empty)
                    // 1171: b"lglmbt.c\0": typeof(_614 = const b"lglmbt.c\x00") = & {l1304} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1171: b"lglmbt.c\0":   l1304 = UNIQUE | NON_NULL, (empty)
                    // 1171: b"lglmbt.c\0" a ... _char: typeof(_611 = move _612 as *const i8 (Misc)) = *const {l1307} i8
                    // 1171: b"lglmbt.c\0" a ... _char:   l1307 = UNIQUE | NON_NULL, (empty)
                    // 1171: b"lglmbt.c\0": typeof(_613 = &raw const (*_614)) = *const {l1305} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1171: b"lglmbt.c\0":   l1305 = UNIQUE | NON_NULL, (empty)
                    341 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 1173: (*::core::mem:: ... ptr(): typeof(_617) = *const {l803} i8
                    // 1173: (*::core::mem:: ... ptr():   l803 = UNIQUE | NON_NULL, (empty)
                    // 1173: (*::core::mem:: ... ptr(): typeof(_618) = & {l805} [i8]
                    // 1173: (*::core::mem:: ... ptr():   l805 = UNIQUE | NON_NULL, FIXED
                    // 1173: (*::core::mem:: ... ptr(): typeof(_619) = & {l807} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1173: (*::core::mem:: ... ptr():   l807 = UNIQUE | NON_NULL, (empty)
                    // 1173: ::core::mem::tr ... 0", ): typeof(_620) = & {l809} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1173: ::core::mem::tr ... 0", ):   l809 = UNIQUE | NON_NULL, FIXED
                    // 1173: (*::core::mem:: ... ptr(): typeof(_619 = &(*_620)) = & {l1310} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1173: (*::core::mem:: ... ptr():   l1310 = UNIQUE | NON_NULL, (empty)
                    // 1173: (*::core::mem:: ... ptr(): typeof(_618 = move _619 as &[i8] (Pointer(Unsize))) = & {l1311} [i8]
                    // 1173: (*::core::mem:: ... ptr():   l1311 = UNIQUE | NON_NULL, FIXED
                        b"void *sat(Data *, unsigned int)\0",
                        // 1174: b"void *sat(Dat ... t)\0": typeof(_621) = & {l811} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1174: b"void *sat(Dat ... t)\0":   l811 = UNIQUE | NON_NULL, (empty)
                        // 1174: b"void *sat(Dat ... t)\0": typeof(_622) = & {l813} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1174: b"void *sat(Dat ... t)\0":   l813 = UNIQUE | NON_NULL, FIXED
                        // 1174: b"void *sat(Dat ... t)\0": typeof(_622 = const b"void *sat(Data *, unsigned int)\x00") = & {l1308} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1174: b"void *sat(Dat ... t)\0":   l1308 = UNIQUE | NON_NULL, (empty)
                        // 1174: b"void *sat(Dat ... t)\0": typeof(_621 = &(*_622)) = & {l1309} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1174: b"void *sat(Dat ... t)\0":   l1309 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
            'c_5301: {
                if (*data).nfrozen <= (*data).navailable {
                } else {
                    __assert_fail(
                        b"data->nfrozen <= data->navailable\0" as *const u8 as *const libc::c_char,
                        // 1183: b"data->nfrozen ... _char: typeof(_629) = *const {l821} i8
                        // 1183: b"data->nfrozen ... _char:   l821 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"data->nfrozen ... st u8: typeof(_630) = *const {l823} u8
                        // 1183: b"data->nfrozen ... st u8:   l823 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"data->nfrozen ... le\0": typeof(_631) = *const {l825} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                        // 1183: b"data->nfrozen ... le\0":   l825 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"data->nfrozen ... le\0": typeof(_632) = & {l827} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                        // 1183: b"data->nfrozen ... le\0":   l827 = UNIQUE | NON_NULL, FIXED
                        // 1183: b"data->nfrozen ... le\0": typeof(_631 = &raw const (*_632)) = *const {l1313} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                        // 1183: b"data->nfrozen ... le\0":   l1313 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"data->nfrozen ... st u8: typeof(_630 = move _631 as *const u8 (Pointer(ArrayToPointer))) = *const {l1314} u8
                        // 1183: b"data->nfrozen ... st u8:   l1314 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"data->nfrozen ... _char: typeof(_629 = move _630 as *const i8 (Misc)) = *const {l1315} i8
                        // 1183: b"data->nfrozen ... _char:   l1315 = UNIQUE | NON_NULL, (empty)
                        // 1183: b"data->nfrozen ... le\0": typeof(_632 = const b"data->nfrozen <= data->navailable\x00") = & {l1312} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000022)) }]
                        // 1183: b"data->nfrozen ... le\0":   l1312 = UNIQUE | NON_NULL, (empty)
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        // 1184: b"lglmbt.c\0" a ... _char: typeof(_633) = *const {l829} i8
                        // 1184: b"lglmbt.c\0" a ... _char:   l829 = UNIQUE | NON_NULL, (empty)
                        // 1184: b"lglmbt.c\0" a ... st u8: typeof(_634) = *const {l831} u8
                        // 1184: b"lglmbt.c\0" a ... st u8:   l831 = UNIQUE | NON_NULL, (empty)
                        // 1184: b"lglmbt.c\0": typeof(_635) = *const {l833} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 1184: b"lglmbt.c\0":   l833 = UNIQUE | NON_NULL, (empty)
                        // 1184: b"lglmbt.c\0": typeof(_636) = & {l835} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 1184: b"lglmbt.c\0":   l835 = UNIQUE | NON_NULL, FIXED
                        // 1184: b"lglmbt.c\0" a ... _char: typeof(_633 = move _634 as *const i8 (Misc)) = *const {l1319} i8
                        // 1184: b"lglmbt.c\0" a ... _char:   l1319 = UNIQUE | NON_NULL, (empty)
                        // 1184: b"lglmbt.c\0": typeof(_635 = &raw const (*_636)) = *const {l1317} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 1184: b"lglmbt.c\0":   l1317 = UNIQUE | NON_NULL, (empty)
                        // 1184: b"lglmbt.c\0": typeof(_636 = const b"lglmbt.c\x00") = & {l1316} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 1184: b"lglmbt.c\0":   l1316 = UNIQUE | NON_NULL, (empty)
                        // 1184: b"lglmbt.c\0" a ... st u8: typeof(_634 = move _635 as *const u8 (Pointer(ArrayToPointer))) = *const {l1318} u8
                        // 1184: b"lglmbt.c\0" a ... st u8:   l1318 = UNIQUE | NON_NULL, (empty)
                        341 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                        // 1186: (*::core::mem:: ... ptr(): typeof(_639) = *const {l839} i8
                        // 1186: (*::core::mem:: ... ptr():   l839 = UNIQUE | NON_NULL, (empty)
                        // 1186: (*::core::mem:: ... ptr(): typeof(_640) = & {l841} [i8]
                        // 1186: (*::core::mem:: ... ptr():   l841 = UNIQUE | NON_NULL, FIXED
                        // 1186: (*::core::mem:: ... ptr(): typeof(_641) = & {l843} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1186: (*::core::mem:: ... ptr():   l843 = UNIQUE | NON_NULL, (empty)
                        // 1186: ::core::mem::tr ... 0", ): typeof(_642) = & {l845} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1186: ::core::mem::tr ... 0", ):   l845 = UNIQUE | NON_NULL, FIXED
                        // 1186: (*::core::mem:: ... ptr(): typeof(_641 = &(*_642)) = & {l1322} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1186: (*::core::mem:: ... ptr():   l1322 = UNIQUE | NON_NULL, (empty)
                        // 1186: (*::core::mem:: ... ptr(): typeof(_640 = move _641 as &[i8] (Pointer(Unsize))) = & {l1323} [i8]
                        // 1186: (*::core::mem:: ... ptr():   l1323 = UNIQUE | NON_NULL, FIXED
                            b"void *sat(Data *, unsigned int)\0",
                            // 1187: b"void *sat(Dat ... t)\0": typeof(_643) = & {l847} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1187: b"void *sat(Dat ... t)\0":   l847 = UNIQUE | NON_NULL, (empty)
                            // 1187: b"void *sat(Dat ... t)\0": typeof(_644) = & {l849} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1187: b"void *sat(Dat ... t)\0":   l849 = UNIQUE | NON_NULL, FIXED
                            // 1187: b"void *sat(Dat ... t)\0": typeof(_643 = &(*_644)) = & {l1321} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1187: b"void *sat(Dat ... t)\0":   l1321 = UNIQUE | NON_NULL, (empty)
                            // 1187: b"void *sat(Dat ... t)\0": typeof(_644 = const b"void *sat(Data *, unsigned int)\x00") = & {l1320} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                            // 1187: b"void *sat(Dat ... t)\0":   l1320 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
            };
            (*data).navailable = 0 as libc::c_int;
            i = 0 as libc::c_int;
            while i < (*data).nfrozen {
                lit_0 = *((*data).frozen).offset(i as isize);
                // 1196: ((*data).frozen ... size): typeof(_652) = *mut {l858} i32
                // 1196: ((*data).frozen ... size):   l858 = UNIQUE | NON_NULL, (empty)
                // 1196: ((*data).frozen): typeof(_653) = *mut {l860} i32
                // 1196: ((*data).frozen):   l860 = UNIQUE | NON_NULL, (empty)
                let mut current_block_94: u64;
                match i % 5 as libc::c_int {
                    0 => {
                        lglmelt(lgl, lit_0);
                        // 1200: lgl: typeof(_666) = *mut {l874} LGL
                        // 1200: lgl:   l874 = UNIQUE | NON_NULL, (empty)
                        current_block_94 = 11046353700707405348;
                    }
                    1 => {
                        current_block_94 = 11046353700707405348;
                    }
                    2 => {
                        (*data).m += 1;
                        *((*data).available).offset((*data).navailable as isize) = (*data).m;
                        // 1208: ((*data).availa ... size): typeof(_670) = *mut {l879} i32
                        // 1208: ((*data).availa ... size):   l879 = UNIQUE | NON_NULL, (empty)
                        // 1208: ((*data).available): typeof(_671) = *mut {l881} i32
                        // 1208: ((*data).available):   l881 = UNIQUE | NON_NULL, (empty)
                        (*data).navailable += 1;
                        (*data).navailable;
                        current_block_94 = 17485376261910781866;
                    }
                    3 => {
                        lglmelt(lgl, lit_0);
                        // 1214: lgl: typeof(_677) = *mut {l888} LGL
                        // 1214: lgl:   l888 = UNIQUE | NON_NULL, (empty)
                        current_block_94 = 17485376261910781866;
                    }
                    _ => {
                        current_block_94 = 17485376261910781866;
                    }
                }
                match current_block_94 {
                    11046353700707405348 => {
                        *((*data).available).offset((*data).navailable as isize) = lit_0;
                        // 1223: ((*data).availa ... size): typeof(_681) = *mut {l893} i32
                        // 1223: ((*data).availa ... size):   l893 = UNIQUE | NON_NULL, (empty)
                        // 1223: ((*data).available): typeof(_682) = *mut {l895} i32
                        // 1223: ((*data).available):   l895 = UNIQUE | NON_NULL, (empty)
                        (*data).navailable += 1;
                        (*data).navailable;
                    }
                    _ => {}
                }
                i += 1;
                i;
            }
            free((*data).frozen as *mut libc::c_void);
            // 1232: (*data).frozen  ... _void: typeof(_693) = *mut {l907} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1232: (*data).frozen  ... _void:   l907 = UNIQUE | NON_NULL, (empty)
            // 1232: (*data).frozen: typeof(_694) = *mut {l909} i32
            // 1232: (*data).frozen:   l909 = UNIQUE | NON_NULL, (empty)
            // 1232: (*data).frozen  ... _void: typeof(_693 = move _694 as *mut libc::c_void (Misc)) = *mut {l1324} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1232: (*data).frozen  ... _void:   l1324 = UNIQUE | NON_NULL, (empty)
        }
        if freeze >= 2 as libc::c_int {
            (*data).n = (pick(
                &mut rng,
                // 1236: &mut rng: typeof(_702) = *mut {l918} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1236: &mut rng:   l918 = UNIQUE | NON_NULL, (empty)
                // 1236: &mut rng: typeof(_703) = &mut {l920} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1236: &mut rng:   l920 = UNIQUE | NON_NULL, (empty)
                // 1236: &mut rng: typeof(_703 = &mut _16) = &mut {l1325} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1236: &mut rng:   l1325 = UNIQUE | NON_NULL, (empty)
                // 1236: &mut rng: typeof(_702 = &raw mut (*_703)) = *mut {l1326} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1236: &mut rng:   l1326 = UNIQUE | NON_NULL, (empty)
                101 as libc::c_int as libc::c_uint,
                130 as libc::c_int as libc::c_uint,
            ) * (*data).n
                + 99 as libc::c_int)
                / 100 as libc::c_int;
            next = Some(inc as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void);
            // 1242: Some(inc as uns ... void): typeof(_717) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l935} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l936} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 1242: Some(inc as uns ... void):   l935 = UNIQUE | NON_NULL, (empty)
            // 1242: Some(inc as uns ... void):   l936 = UNIQUE | NON_NULL, (empty)
            // 1242: inc as unsafe e ... _void: typeof(_718) = fn(*mut {l938} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l939} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1242: inc as unsafe e ... _void:   l938 = UNIQUE | NON_NULL, (empty)
            // 1242: inc as unsafe e ... _void:   l939 = UNIQUE | NON_NULL, (empty)
            // 1242: Some(inc as uns ... void): typeof(_717 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _718)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1329} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l1330} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 1242: Some(inc as uns ... void):   l1329 = UNIQUE | NON_NULL, (empty)
            // 1242: Some(inc as uns ... void):   l1330 = UNIQUE | NON_NULL, (empty)
            // 1242: inc: typeof(_718 = inc as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l1327} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l1328} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1242: inc:   l1327 = UNIQUE | NON_NULL, (empty)
            // 1242: inc:   l1328 = UNIQUE | NON_NULL, (empty)
        }
        if pick(
            &mut rng,
            // 1245: &mut rng: typeof(_722) = *mut {l944} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1245: &mut rng:   l944 = UNIQUE | NON_NULL, (empty)
            // 1245: &mut rng: typeof(_723) = &mut {l946} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1245: &mut rng:   l946 = UNIQUE | NON_NULL, (empty)
            // 1245: &mut rng: typeof(_723 = &mut _16) = &mut {l1331} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1245: &mut rng:   l1331 = UNIQUE | NON_NULL, (empty)
            // 1245: &mut rng: typeof(_722 = &raw mut (*_723)) = *mut {l1332} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1245: &mut rng:   l1332 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglchanged(lgl);
            // 1250: lgl: typeof(_729) = *mut {l953} LGL
            // 1250: lgl:   l953 = UNIQUE | NON_NULL, (empty)
        }
        if pick(
            &mut rng,
            // 1253: &mut rng: typeof(_732) = *mut {l957} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1253: &mut rng:   l957 = UNIQUE | NON_NULL, (empty)
            // 1253: &mut rng: typeof(_733) = &mut {l959} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1253: &mut rng:   l959 = UNIQUE | NON_NULL, (empty)
            // 1253: &mut rng: typeof(_732 = &raw mut (*_733)) = *mut {l1334} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1253: &mut rng:   l1334 = UNIQUE | NON_NULL, (empty)
            // 1253: &mut rng: typeof(_733 = &mut _16) = &mut {l1333} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1253: &mut rng:   l1333 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            3 as libc::c_int as libc::c_uint,
        ) == 0
        {
            let mut count: libc::c_int = pick(
                &mut rng,
                // 1259: &mut rng: typeof(_739) = *mut {l966} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1259: &mut rng:   l966 = UNIQUE | NON_NULL, (empty)
                // 1259: &mut rng: typeof(_740) = &mut {l968} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1259: &mut rng:   l968 = UNIQUE | NON_NULL, (empty)
                // 1259: &mut rng: typeof(_739 = &raw mut (*_740)) = *mut {l1336} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1259: &mut rng:   l1336 = UNIQUE | NON_NULL, (empty)
                // 1259: &mut rng: typeof(_740 = &mut _16) = &mut {l1335} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1259: &mut rng:   l1335 = UNIQUE | NON_NULL, (empty)
                1 as libc::c_int as libc::c_uint,
                ((*data).m / 10 as libc::c_int) as libc::c_uint,
            );
            let mut i_0: libc::c_int = 0;
            i_0 = 0 as libc::c_int;
            while i_0 < count {
                let mut lit_1: libc::c_int = pick(
                    &mut rng,
                    // 1267: &mut rng: typeof(_757) = *mut {l986} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 1267: &mut rng:   l986 = UNIQUE | NON_NULL, (empty)
                    // 1267: &mut rng: typeof(_758) = &mut {l988} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 1267: &mut rng:   l988 = UNIQUE | NON_NULL, (empty)
                    // 1267: &mut rng: typeof(_757 = &raw mut (*_758)) = *mut {l1338} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 1267: &mut rng:   l1338 = UNIQUE | NON_NULL, (empty)
                    // 1267: &mut rng: typeof(_758 = &mut _16) = &mut {l1337} DefId(0:321 ~ lglmbt[b165]::RNG)
                    // 1267: &mut rng:   l1337 = UNIQUE | NON_NULL, (empty)
                    1 as libc::c_int as libc::c_uint,
                    (*data).m as libc::c_uint,
                );
                if lglusable(lgl, lit_1) == 0 && lglreusable(lgl, lit_1) != 0 {
                // 1271: lgl: typeof(_767) = *mut {l998} LGL
                // 1271: lgl:   l998 = UNIQUE | NON_NULL, (empty)
                // 1271: lgl: typeof(_771) = *mut {l1003} LGL
                // 1271: lgl:   l1003 = UNIQUE | NON_NULL, (empty)
                    lglreuse(lgl, lit_1);
                    // 1272: lgl: typeof(_774) = *mut {l1007} LGL
                    // 1272: lgl:   l1007 = UNIQUE | NON_NULL, (empty)
                }
                i_0 += 1;
                i_0;
            }
        }
    } else if res == 20 as libc::c_int {
        if (*data).print != 0 {
            printf(b"uns \0" as *const u8 as *const libc::c_char);
            // 1280: b"uns \0" as *c ... _char: typeof(_788) = *const {l1022} i8
            // 1280: b"uns \0" as *c ... _char:   l1022 = UNIQUE | NON_NULL, (empty)
            // 1280: b"uns \0" as *c ... st u8: typeof(_789) = *const {l1024} u8
            // 1280: b"uns \0" as *c ... st u8:   l1024 = UNIQUE | NON_NULL, (empty)
            // 1280: b"uns \0": typeof(_790) = *const {l1026} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1280: b"uns \0":   l1026 = UNIQUE | NON_NULL, (empty)
            // 1280: b"uns \0": typeof(_791) = & {l1028} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1280: b"uns \0":   l1028 = UNIQUE | NON_NULL, FIXED
            // 1280: b"uns \0": typeof(_791 = const b"uns \x00") = & {l1339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1280: b"uns \0":   l1339 = UNIQUE | NON_NULL, (empty)
            // 1280: b"uns \0": typeof(_790 = &raw const (*_791)) = *const {l1340} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1280: b"uns \0":   l1340 = UNIQUE | NON_NULL, (empty)
            // 1280: b"uns \0" as *c ... _char: typeof(_788 = move _789 as *const i8 (Misc)) = *const {l1342} i8
            // 1280: b"uns \0" as *c ... _char:   l1342 = UNIQUE | NON_NULL, (empty)
            // 1280: b"uns \0" as *c ... st u8: typeof(_789 = move _790 as *const u8 (Pointer(ArrayToPointer))) = *const {l1341} u8
            // 1280: b"uns \0" as *c ... st u8:   l1341 = UNIQUE | NON_NULL, (empty)
        }
        if nassumed > 0 as libc::c_int {
            i = pick(
                &mut rng,
                // 1284: &mut rng: typeof(_797) = *mut {l1035} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1284: &mut rng:   l1035 = UNIQUE | NON_NULL, (empty)
                // 1284: &mut rng: typeof(_798) = &mut {l1037} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1284: &mut rng:   l1037 = UNIQUE | NON_NULL, (empty)
                // 1284: &mut rng: typeof(_797 = &raw mut (*_798)) = *mut {l1344} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1284: &mut rng:   l1344 = UNIQUE | NON_NULL, (empty)
                // 1284: &mut rng: typeof(_798 = &mut _16) = &mut {l1343} DefId(0:321 ~ lglmbt[b165]::RNG)
                // 1284: &mut rng:   l1343 = UNIQUE | NON_NULL, (empty)
                0 as libc::c_int as libc::c_uint,
                (3 as libc::c_int * nassumed / 2 as libc::c_int) as libc::c_uint,
            );
            loop {
                let fresh2 = i;
                i = i - 1;
                if !(fresh2 > 0 as libc::c_int) {
                    break;
                }
                lglfailed(
                    (*data).lgl,
                    // 1295: (*data).lgl: typeof(_822) = *mut {l1062} LGL
                    // 1295: (*data).lgl:   l1062 = UNIQUE | NON_NULL, (empty)
                    *assumed.offset(pick(
                    // 1296: assumed.offset( ... size): typeof(_824) = *mut {l1065} i32
                    // 1296: assumed.offset( ... size):   l1065 = UNIQUE | NON_NULL, (empty)
                    // 1296: assumed: typeof(_825) = *mut {l1067} i32
                    // 1296: assumed:   l1067 = UNIQUE | NON_NULL, (empty)
                        &mut rng,
                        // 1297: &mut rng: typeof(_828) = *mut {l1071} DefId(0:321 ~ lglmbt[b165]::RNG)
                        // 1297: &mut rng:   l1071 = UNIQUE | NON_NULL, (empty)
                        // 1297: &mut rng: typeof(_829) = &mut {l1073} DefId(0:321 ~ lglmbt[b165]::RNG)
                        // 1297: &mut rng:   l1073 = UNIQUE | NON_NULL, (empty)
                        // 1297: &mut rng: typeof(_828 = &raw mut (*_829)) = *mut {l1346} DefId(0:321 ~ lglmbt[b165]::RNG)
                        // 1297: &mut rng:   l1346 = UNIQUE | NON_NULL, (empty)
                        // 1297: &mut rng: typeof(_829 = &mut _16) = &mut {l1345} DefId(0:321 ~ lglmbt[b165]::RNG)
                        // 1297: &mut rng:   l1345 = UNIQUE | NON_NULL, (empty)
                        0 as libc::c_int as libc::c_uint,
                        (nassumed - 1 as libc::c_int) as libc::c_uint,
                    ) as isize),
                );
            }
        }
        if pick(
            &mut rng,
            // 1305: &mut rng: typeof(_839) = *mut {l1084} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1305: &mut rng:   l1084 = UNIQUE | NON_NULL, (empty)
            // 1305: &mut rng: typeof(_840) = &mut {l1086} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1305: &mut rng:   l1086 = UNIQUE | NON_NULL, (empty)
            // 1305: &mut rng: typeof(_840 = &mut _16) = &mut {l1347} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1305: &mut rng:   l1347 = UNIQUE | NON_NULL, (empty)
            // 1305: &mut rng: typeof(_839 = &raw mut (*_840)) = *mut {l1348} DefId(0:321 ~ lglmbt[b165]::RNG)
            // 1305: &mut rng:   l1348 = UNIQUE | NON_NULL, (empty)
            0 as libc::c_int as libc::c_uint,
            4 as libc::c_int as libc::c_uint,
        ) == 0
        {
            lglinconsistent(lgl);
            // 1310: lgl: typeof(_846) = *mut {l1093} LGL
            // 1310: lgl:   l1093 = UNIQUE | NON_NULL, (empty)
        }
    } else if (*data).print != 0 {
        printf(b"nil \0" as *const u8 as *const libc::c_char);
        // 1313: b"nil \0" as *c ... _char: typeof(_850) = *const {l1098} i8
        // 1313: b"nil \0" as *c ... _char:   l1098 = UNIQUE | NON_NULL, (empty)
        // 1313: b"nil \0" as *c ... st u8: typeof(_851) = *const {l1100} u8
        // 1313: b"nil \0" as *c ... st u8:   l1100 = UNIQUE | NON_NULL, (empty)
        // 1313: b"nil \0": typeof(_852) = *const {l1102} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1313: b"nil \0":   l1102 = UNIQUE | NON_NULL, (empty)
        // 1313: b"nil \0": typeof(_853) = & {l1104} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1313: b"nil \0":   l1104 = UNIQUE | NON_NULL, FIXED
        // 1313: b"nil \0" as *c ... st u8: typeof(_851 = move _852 as *const u8 (Pointer(ArrayToPointer))) = *const {l1351} u8
        // 1313: b"nil \0" as *c ... st u8:   l1351 = UNIQUE | NON_NULL, (empty)
        // 1313: b"nil \0" as *c ... _char: typeof(_850 = move _851 as *const i8 (Misc)) = *const {l1352} i8
        // 1313: b"nil \0" as *c ... _char:   l1352 = UNIQUE | NON_NULL, (empty)
        // 1313: b"nil \0": typeof(_853 = const b"nil \x00") = & {l1349} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1313: b"nil \0":   l1349 = UNIQUE | NON_NULL, (empty)
        // 1313: b"nil \0": typeof(_852 = &raw const (*_853)) = *const {l1350} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
        // 1313: b"nil \0":   l1350 = UNIQUE | NON_NULL, (empty)
    }
    if (*data).print != 0 {
        fflush(stdout);
        // 1316: stdout: typeof(_858) = *mut {l1110} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1316: stdout:   l1110 = UNIQUE | NON_NULL, (empty)
        // 1316: stdout: typeof(_859) = *mut {l1112} *mut {l1113} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1316: stdout:   l1112 = UNIQUE | NON_NULL, (empty)
        // 1316: stdout:   l1113 = UNIQUE | NON_NULL, (empty)
    }
    if pick(
        &mut rng,
        // 1319: &mut rng: typeof(_863) = *mut {l1118} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1319: &mut rng:   l1118 = UNIQUE | NON_NULL, (empty)
        // 1319: &mut rng: typeof(_864) = &mut {l1120} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1319: &mut rng:   l1120 = UNIQUE | NON_NULL, (empty)
        // 1319: &mut rng: typeof(_864 = &mut _16) = &mut {l1353} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1319: &mut rng:   l1353 = UNIQUE | NON_NULL, (empty)
        // 1319: &mut rng: typeof(_863 = &raw mut (*_864)) = *mut {l1354} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1319: &mut rng:   l1354 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        11 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglreducecache(lgl);
        // 1324: lgl: typeof(_870) = *mut {l1127} LGL
        // 1324: lgl:   l1127 = UNIQUE | NON_NULL, (empty)
    } else if pick(
        &mut rng,
        // 1326: &mut rng: typeof(_873) = *mut {l1131} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1326: &mut rng:   l1131 = UNIQUE | NON_NULL, (empty)
        // 1326: &mut rng: typeof(_874) = &mut {l1133} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1326: &mut rng:   l1133 = UNIQUE | NON_NULL, (empty)
        // 1326: &mut rng: typeof(_874 = &mut _16) = &mut {l1355} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1326: &mut rng:   l1355 = UNIQUE | NON_NULL, (empty)
        // 1326: &mut rng: typeof(_873 = &raw mut (*_874)) = *mut {l1356} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1326: &mut rng:   l1356 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        17 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglflushcache(lgl);
        // 1331: lgl: typeof(_880) = *mut {l1140} LGL
        // 1331: lgl:   l1140 = UNIQUE | NON_NULL, (empty)
    }
    if pick(
        &mut rng,
        // 1334: &mut rng: typeof(_884) = *mut {l1145} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1334: &mut rng:   l1145 = UNIQUE | NON_NULL, (empty)
        // 1334: &mut rng: typeof(_885) = &mut {l1147} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1334: &mut rng:   l1147 = UNIQUE | NON_NULL, (empty)
        // 1334: &mut rng: typeof(_884 = &raw mut (*_885)) = *mut {l1358} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1334: &mut rng:   l1358 = UNIQUE | NON_NULL, (empty)
        // 1334: &mut rng: typeof(_885 = &mut _16) = &mut {l1357} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1334: &mut rng:   l1357 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int as libc::c_uint,
        1000 as libc::c_int as libc::c_uint,
    ) == 0
    {
        lglchkclone(lgl);
        // 1339: lgl: typeof(_891) = *mut {l1154} LGL
        // 1339: lgl:   l1154 = UNIQUE | NON_NULL, (empty)
    }
    free(assumed as *mut libc::c_void);
    // 1341: assumed as *mut ... _void: typeof(_893) = *mut {l1157} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1341: assumed as *mut ... _void:   l1157 = UNIQUE | NON_NULL, (empty)
    // 1341: assumed: typeof(_894) = *mut {l1159} i32
    // 1341: assumed:   l1159 = UNIQUE | NON_NULL, (empty)
    // 1341: assumed as *mut ... _void: typeof(_893 = move _894 as *mut libc::c_void (Misc)) = *mut {l1359} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1341: assumed as *mut ... _void:   l1359 = UNIQUE | NON_NULL, (empty)
    return ::core::mem::transmute::<State, *mut libc::c_void>(next);
    // 1342: next: typeof(_895) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1161} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l1162} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1342: next:   l1161 = UNIQUE | NON_NULL, (empty)
    // 1342: next:   l1162 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn release(mut data: *mut Data, mut r: libc::c_uint) -> *mut libc::c_void {
// 1344: *mut libc::c_void: typeof(_0) = *mut {g7} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 1344: *mut libc::c_void:   g7 = UNIQUE | NON_NULL, FIXED
// 1344: mut data: typeof(_1) = *mut {g6} DefId(0:299 ~ lglmbt[b165]::Data)
// 1344: mut data:   g6 = UNIQUE | NON_NULL, FIXED
    lglrelease((*data).lgl);
    // 1345: (*data).lgl: typeof(_5) = *mut {l5} LGL
    // 1345: (*data).lgl:   l5 = UNIQUE | NON_NULL, (empty)
    return 0 as *mut libc::c_void;
    // 1346: 0 as *mut libc: ... _void: typeof(_0 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l7} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1346: 0 as *mut libc: ... _void:   l7 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn rantrav(mut env_0: *mut Env) {
// 1348: mut env_0: typeof(_1) = *mut {g30} DefId(0:327 ~ lglmbt[b165]::Env)
// 1348: mut env_0:   g30 = UNIQUE | NON_NULL, FIXED
    let mut state: State = None;
    // 1349: mut state: typeof(_2) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l2} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l3} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1349: mut state:   l2 = UNIQUE | NON_NULL, (empty)
    // 1349: mut state:   l3 = UNIQUE | NON_NULL, (empty)
    // 1349: None: typeof(_2 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::None) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l98} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l99} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1349: None:   l98 = UNIQUE | NON_NULL, (empty)
    // 1349: None:   l99 = UNIQUE | NON_NULL, (empty)
    let mut next: State = None;
    // 1350: mut next: typeof(_3) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l5} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1350: mut next:   l5 = UNIQUE | NON_NULL, (empty)
    // 1350: mut next:   l6 = UNIQUE | NON_NULL, (empty)
    // 1350: None: typeof(_3 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::None) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l100} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l101} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1350: None:   l100 = UNIQUE | NON_NULL, (empty)
    // 1350: None:   l101 = UNIQUE | NON_NULL, (empty)
    let mut rand: libc::c_uint = 0;
    let mut data: Data = Data {
        lgl: 0 as *mut LGL,
        // 1353: 0 as *mut LGL: typeof(_6) = *mut {l10} LGL
        // 1353: 0 as *mut LGL:   l10 = UNIQUE | NON_NULL, (empty)
        // 1353: 0 as *mut LGL: typeof(_6 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l102} LGL
        // 1353: 0 as *mut LGL:   l102 = UNIQUE | NON_NULL, (empty)
        trace: 0 as *mut FILE,
        // 1354: 0 as *mut FILE: typeof(_7) = *mut {l12} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1354: 0 as *mut FILE:   l12 = UNIQUE | NON_NULL, (empty)
        // 1354: 0 as *mut FILE: typeof(_7 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l103} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1354: 0 as *mut FILE:   l103 = UNIQUE | NON_NULL, (empty)
        available: 0 as *mut libc::c_int,
        // 1355: 0 as *mut libc: ... c_int: typeof(_8) = *mut {l14} i32
        // 1355: 0 as *mut libc: ... c_int:   l14 = UNIQUE | NON_NULL, (empty)
        // 1355: 0 as *mut libc: ... c_int: typeof(_8 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l104} i32
        // 1355: 0 as *mut libc: ... c_int:   l104 = UNIQUE | NON_NULL, (empty)
        navailable: 0,
        frozen: 0 as *mut libc::c_int,
        // 1357: 0 as *mut libc: ... c_int: typeof(_9) = *mut {l16} i32
        // 1357: 0 as *mut libc: ... c_int:   l16 = UNIQUE | NON_NULL, (empty)
        // 1357: 0 as *mut libc: ... c_int: typeof(_9 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l105} i32
        // 1357: 0 as *mut libc: ... c_int:   l105 = UNIQUE | NON_NULL, (empty)
        nfrozen: 0,
        m: 0,
        n: 0,
        c: 0,
        print: 0,
        noptsfuzz: 0,
    };
    memset(
    // 1365: memset( &mut da ... ng, ): typeof(_10) = *mut {l18} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1365: memset( &mut da ... ng, ):   l18 = UNIQUE | NON_NULL, (empty)
        &mut data as *mut Data as *mut libc::c_void,
        // 1366: &mut data as *m ... _void: typeof(_11) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1366: &mut data as *m ... _void:   l20 = UNIQUE | NON_NULL, (empty)
        // 1366: &mut data as *m ...  Data: typeof(_12) = *mut {l22} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1366: &mut data as *m ...  Data:   l22 = UNIQUE | NON_NULL, (empty)
        // 1366: &mut data: typeof(_13) = &mut {l24} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1366: &mut data:   l24 = UNIQUE | NON_NULL, (empty)
        // 1366: &mut data: typeof(_13 = &mut _5) = &mut {l106} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1366: &mut data:   l106 = UNIQUE | NON_NULL, (empty)
        // 1366: &mut data as *m ... _void: typeof(_11 = move _12 as *mut libc::c_void (Misc)) = *mut {l108} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1366: &mut data as *m ... _void:   l108 = UNIQUE | NON_NULL, (empty)
        // 1366: &mut data: typeof(_12 = &raw mut (*_13)) = *mut {l107} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1366: &mut data:   l107 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
        ::core::mem::size_of::<Data>() as libc::c_ulong,
    );
    data.print = (*env_0).print;
    (*env_0).rng.w = (*env_0).seed;
    (*env_0).rng.z = (*env_0).rng.w;
    state = Some(init as unsafe extern "C" fn(*mut Data, libc::c_uint) -> *mut libc::c_void);
    // 1373: Some(init as un ... void): typeof(_20) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l32} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l33} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1373: Some(init as un ... void):   l32 = UNIQUE | NON_NULL, (empty)
    // 1373: Some(init as un ... void):   l33 = UNIQUE | NON_NULL, (empty)
    // 1373: init as unsafe  ... _void: typeof(_21) = fn(*mut {l35} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l36} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1373: init as unsafe  ... _void:   l35 = UNIQUE | NON_NULL, (empty)
    // 1373: init as unsafe  ... _void:   l36 = UNIQUE | NON_NULL, (empty)
    // 1373: init: typeof(_21 = init as unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l109} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l110} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1373: init:   l109 = UNIQUE | NON_NULL, (empty)
    // 1373: init:   l110 = UNIQUE | NON_NULL, (empty)
    // 1373: Some(init as un ... void): typeof(_20 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::Some(move _21)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l111} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l112} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1373: Some(init as un ... void):   l111 = UNIQUE | NON_NULL, (empty)
    // 1373: Some(init as un ... void):   l112 = UNIQUE | NON_NULL, (empty)
    while state.is_some() {
    // 1374: state.is_some(): typeof(_24) = & {l40} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l41} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l42} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1374: state.is_some():   l40 = UNIQUE | NON_NULL, (empty)
    // 1374: state.is_some():   l41 = UNIQUE | NON_NULL, (empty)
    // 1374: state.is_some():   l42 = UNIQUE | NON_NULL, (empty)
    // 1374: state.is_some(): typeof(_24 = &_2) = & {l113} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l114} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l115} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1374: state.is_some():   l113 = UNIQUE | NON_NULL, (empty)
    // 1374: state.is_some():   l114 = UNIQUE | NON_NULL, (empty)
    // 1374: state.is_some():   l115 = UNIQUE | NON_NULL, (empty)
        rand = nextrand_shim(&mut (*env_0).rng);
        // 1375: &mut (*env_0).rng: typeof(_26) = *mut {l45} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1375: &mut (*env_0).rng:   l45 = UNIQUE | NON_NULL, (empty)
        // 1375: &mut (*env_0).rng: typeof(_27) = &mut {l47} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1375: &mut (*env_0).rng:   l47 = UNIQUE | NON_NULL, (empty)
        // 1375: &mut (*env_0).rng: typeof(_26 = &raw mut (*_27)) = *mut {l117} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1375: &mut (*env_0).rng:   l117 = UNIQUE | NON_NULL, (empty)
        // 1375: &mut (*env_0).rng: typeof(_27 = &mut ((*_1).17: RNG)) = &mut {l116} DefId(0:321 ~ lglmbt[b165]::RNG)
        // 1375: &mut (*env_0).rng:   l116 = UNIQUE | NON_NULL, (empty)
        if !((*env_0).file).is_null() {
        // 1376: ((*env_0).file): typeof(_31) = *mut {l52} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1376: ((*env_0).file):   l52 = UNIQUE | NON_NULL, (empty)
            fprintf(
                (*env_0).file,
                // 1378: (*env_0).file: typeof(_33) = *mut {l55} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 1378: (*env_0).file:   l55 = UNIQUE | NON_NULL, (empty)
                b"%p %x\n\0" as *const u8 as *const libc::c_char,
                // 1379: b"%p %x\n\0" as ... _char: typeof(_34) = *const {l57} i8
                // 1379: b"%p %x\n\0" as ... _char:   l57 = UNIQUE | NON_NULL, (empty)
                // 1379: b"%p %x\n\0" as ... st u8: typeof(_35) = *const {l59} u8
                // 1379: b"%p %x\n\0" as ... st u8:   l59 = UNIQUE | NON_NULL, (empty)
                // 1379: b"%p %x\n\0": typeof(_36) = *const {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 1379: b"%p %x\n\0":   l61 = UNIQUE | NON_NULL, (empty)
                // 1379: b"%p %x\n\0": typeof(_37) = & {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 1379: b"%p %x\n\0":   l63 = UNIQUE | NON_NULL, FIXED
                // 1379: b"%p %x\n\0" as ... _char: typeof(_34 = move _35 as *const i8 (Misc)) = *const {l121} i8
                // 1379: b"%p %x\n\0" as ... _char:   l121 = UNIQUE | NON_NULL, (empty)
                // 1379: b"%p %x\n\0": typeof(_36 = &raw const (*_37)) = *const {l119} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 1379: b"%p %x\n\0":   l119 = UNIQUE | NON_NULL, (empty)
                // 1379: b"%p %x\n\0": typeof(_37 = const b"%p %x\n\x00") = & {l118} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                // 1379: b"%p %x\n\0":   l118 = UNIQUE | NON_NULL, (empty)
                // 1379: b"%p %x\n\0" as ... st u8: typeof(_35 = move _36 as *const u8 (Pointer(ArrayToPointer))) = *const {l120} u8
                // 1379: b"%p %x\n\0" as ... st u8:   l120 = UNIQUE | NON_NULL, (empty)
                state,
                // 1380: state: typeof(_38) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l65} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l66} DefId(2:5583 ~ core[480a]::ffi::c_void)>
                // 1380: state:   l65 = UNIQUE | NON_NULL, (empty)
                // 1380: state:   l66 = UNIQUE | NON_NULL, (empty)
                rand,
            );
            fflush((*env_0).file);
            // 1383: (*env_0).file: typeof(_41) = *mut {l70} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
            // 1383: (*env_0).file:   l70 = UNIQUE | NON_NULL, (empty)
        }
        next = ::core::mem::transmute::<*mut libc::c_void, State>(state
        // 1385: ::core::mem::tr ... nd )): typeof(_42) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l72} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l73} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1385: ::core::mem::tr ... nd )):   l72 = UNIQUE | NON_NULL, (empty)
        // 1385: ::core::mem::tr ... nd )):   l73 = UNIQUE | NON_NULL, (empty)
        // 1385: state .expect(" ... and ): typeof(_43) = *mut {l75} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1385: state .expect(" ... and ):   l75 = UNIQUE | NON_NULL, (empty)
        // 1385: state .expect(" ... ter"): typeof(_44) = fn(*mut {l77} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l78} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1385: state .expect(" ... ter"):   l77 = UNIQUE | NON_NULL, (empty)
        // 1385: state .expect(" ... ter"):   l78 = UNIQUE | NON_NULL, (empty)
        // 1385: state: typeof(_45) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l80} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l81} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1385: state:   l80 = UNIQUE | NON_NULL, (empty)
        // 1385: state:   l81 = UNIQUE | NON_NULL, (empty)
            .expect("non-null function pointer")(
            // 1386: "non-null funct ... nter": typeof(_46) = & {l83} str
            // 1386: "non-null funct ... nter":   l83 = UNIQUE | NON_NULL, (empty)
            // 1386: "non-null funct ... nter": typeof(_47) = & {l85} str
            // 1386: "non-null funct ... nter":   l85 = UNIQUE | NON_NULL, FIXED
            // 1386: "non-null funct ... nter": typeof(_47 = const "non-null function pointer") = & {l122} str
            // 1386: "non-null funct ... nter":   l122 = UNIQUE | NON_NULL, (empty)
            // 1386: "non-null funct ... nter": typeof(_46 = &(*_47)) = & {l123} str
            // 1386: "non-null funct ... nter":   l123 = UNIQUE | NON_NULL, (empty)
            &mut data, rand
            // 1387: &mut data: typeof(_48) = *mut {l87} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1387: &mut data:   l87 = UNIQUE | NON_NULL, (empty)
            // 1387: &mut data: typeof(_49) = &mut {l89} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1387: &mut data:   l89 = UNIQUE | NON_NULL, (empty)
            // 1387: &mut data: typeof(_48 = &raw mut (*_49)) = *mut {l125} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1387: &mut data:   l125 = UNIQUE | NON_NULL, (empty)
            // 1387: &mut data: typeof(_49 = &mut _5) = &mut {l124} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1387: &mut data:   l124 = UNIQUE | NON_NULL, (empty)
        ));
        state = next;
        // 1389: next: typeof(_51) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l92} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l93} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1389: next:   l92 = UNIQUE | NON_NULL, (empty)
        // 1389: next:   l93 = UNIQUE | NON_NULL, (empty)
    }
}
unsafe extern "C" fn erase() {
    let mut i: libc::c_int = 0;
    fputc('\r' as i32, stdout);
    // 1394: stdout: typeof(_4) = *mut {l4} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1394: stdout:   l4 = UNIQUE | NON_NULL, (empty)
    // 1394: stdout: typeof(_5) = *mut {l6} *mut {l7} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1394: stdout:   l6 = UNIQUE | NON_NULL, (empty)
    // 1394: stdout:   l7 = UNIQUE | NON_NULL, (empty)
    i = 0 as libc::c_int;
    while i < 79 as libc::c_int {
        fputc(' ' as i32, stdout);
        // 1397: stdout: typeof(_14) = *mut {l17} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1397: stdout:   l17 = UNIQUE | NON_NULL, (empty)
        // 1397: stdout: typeof(_15) = *mut {l19} *mut {l20} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1397: stdout:   l19 = UNIQUE | NON_NULL, (empty)
        // 1397: stdout:   l20 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    fputc('\r' as i32, stdout);
    // 1401: stdout: typeof(_23) = *mut {l29} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1401: stdout:   l29 = UNIQUE | NON_NULL, (empty)
    // 1401: stdout: typeof(_24) = *mut {l31} *mut {l32} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1401: stdout:   l31 = UNIQUE | NON_NULL, (empty)
    // 1401: stdout:   l32 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn isnumstr(mut str: *const libc::c_char) -> libc::c_int {
// 1403: mut str: typeof(_1) = *const {g31} i8
// 1403: mut str:   g31 = UNIQUE | NON_NULL, FIXED
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    // 1404: mut p: typeof(_3) = *const {l3} i8
    // 1404: mut p:   l3 = UNIQUE | NON_NULL, (empty)
    // 1404: 0 as *const lib ... _char: typeof(_3 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l39} i8
    // 1404: 0 as *const lib ... _char:   l39 = UNIQUE | NON_NULL, (empty)
    p = str;
    // 1405: str: typeof(_4) = *const {l5} i8
    // 1405: str:   l5 = UNIQUE | NON_NULL, (empty)
    while *p != 0 {
        if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
        // 1407: (*__ctype_b_loc ... size): typeof(_14) = *const {l16} u16
        // 1407: (*__ctype_b_loc ... size):   l16 = UNIQUE | NON_NULL, (empty)
        // 1407: (*__ctype_b_loc()): typeof(_15) = *const {l18} u16
        // 1407: (*__ctype_b_loc()):   l18 = UNIQUE | NON_NULL, (empty)
        // 1407: __ctype_b_loc(): typeof(_16) = *mut {l20} *const {l21} u16
        // 1407: __ctype_b_loc():   l20 = UNIQUE | NON_NULL, (empty)
        // 1407: __ctype_b_loc():   l21 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            == 0
        {
            return 0 as libc::c_int;
        }
        p = p.offset(1);
        // 1413: p.offset(1): typeof(_24) = *const {l30} i8
        // 1413: p.offset(1):   l30 = UNIQUE | NON_NULL, (empty)
        // 1413: p: typeof(_25) = *const {l32} i8
        // 1413: p:   l32 = UNIQUE | NON_NULL, (empty)
        p;
        // 1414: p: typeof(_26) = *const {l34} i8
        // 1414: p:   l34 = UNIQUE | NON_NULL, (empty)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn die(mut msg: *const libc::c_char, mut args: ...) {
// 1418: mut msg: typeof(_1) = *const {g32} i8
// 1418: mut msg:   g32 = UNIQUE | NON_NULL, FIXED
    let mut ap: ::core::ffi::VaListImpl;
    fputs(
        b"*** lglmbt: \0" as *const u8 as *const libc::c_char,
        // 1421: b"*** lglmbt: \ ... _char: typeof(_6) = *const {l6} i8
        // 1421: b"*** lglmbt: \ ... _char:   l6 = UNIQUE | NON_NULL, (empty)
        // 1421: b"*** lglmbt: \ ... st u8: typeof(_7) = *const {l8} u8
        // 1421: b"*** lglmbt: \ ... st u8:   l8 = UNIQUE | NON_NULL, (empty)
        // 1421: b"*** lglmbt: \0": typeof(_8) = *const {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1421: b"*** lglmbt: \0":   l10 = UNIQUE | NON_NULL, (empty)
        // 1421: b"*** lglmbt: \0": typeof(_9) = & {l12} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1421: b"*** lglmbt: \0":   l12 = UNIQUE | NON_NULL, FIXED
        // 1421: b"*** lglmbt: \ ... st u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l50} u8
        // 1421: b"*** lglmbt: \ ... st u8:   l50 = UNIQUE | NON_NULL, (empty)
        // 1421: b"*** lglmbt: \ ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l51} i8
        // 1421: b"*** lglmbt: \ ... _char:   l51 = UNIQUE | NON_NULL, (empty)
        // 1421: b"*** lglmbt: \0": typeof(_8 = &raw const (*_9)) = *const {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1421: b"*** lglmbt: \0":   l49 = UNIQUE | NON_NULL, (empty)
        // 1421: b"*** lglmbt: \0": typeof(_9 = const b"*** lglmbt: \x00") = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
        // 1421: b"*** lglmbt: \0":   l48 = UNIQUE | NON_NULL, (empty)
        stderr,
        // 1422: stderr: typeof(_10) = *mut {l14} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1422: stderr:   l14 = UNIQUE | NON_NULL, (empty)
        // 1422: stderr: typeof(_11) = *mut {l16} *mut {l17} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1422: stderr:   l16 = UNIQUE | NON_NULL, (empty)
        // 1422: stderr:   l17 = UNIQUE | NON_NULL, (empty)
    );
    ap = args.clone();
    // 1424: args.clone(): typeof(_13) = & {l20} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1424: args.clone():   l20 = UNIQUE | NON_NULL, (empty)
    // 1424: args.clone(): typeof(_13 = &_2) = & {l52} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1424: args.clone():   l52 = UNIQUE | NON_NULL, (empty)
    vfprintf(stderr, msg, ap.as_va_list());
    // 1425: stderr: typeof(_15) = *mut {l23} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1425: stderr:   l23 = UNIQUE | NON_NULL, (empty)
    // 1425: stderr: typeof(_16) = *mut {l25} *mut {l26} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1425: stderr:   l25 = UNIQUE | NON_NULL, (empty)
    // 1425: stderr:   l26 = UNIQUE | NON_NULL, (empty)
    // 1425: msg: typeof(_17) = *const {l28} i8
    // 1425: msg:   l28 = UNIQUE | NON_NULL, (empty)
    // 1425: ap.as_va_list(): typeof(_19) = &mut {l31} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1425: ap.as_va_list():   l31 = UNIQUE | NON_NULL, (empty)
    // 1425: ap.as_va_list(): typeof(_19 = &mut _4) = &mut {l53} DefId(2:46548 ~ core[480a]::ffi::VaListImpl)
    // 1425: ap.as_va_list():   l53 = UNIQUE | NON_NULL, (empty)
    fputc('\n' as i32, stderr);
    // 1426: stderr: typeof(_22) = *mut {l35} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1426: stderr:   l35 = UNIQUE | NON_NULL, (empty)
    // 1426: stderr: typeof(_23) = *mut {l37} *mut {l38} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1426: stderr:   l37 = UNIQUE | NON_NULL, (empty)
    // 1426: stderr:   l38 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 1427: stderr: typeof(_25) = *mut {l41} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1427: stderr:   l41 = UNIQUE | NON_NULL, (empty)
    // 1427: stderr: typeof(_26) = *mut {l43} *mut {l44} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1427: stderr:   l43 = UNIQUE | NON_NULL, (empty)
    // 1427: stderr:   l44 = UNIQUE | NON_NULL, (empty)
    exit(1 as libc::c_int);
}
unsafe extern "C" fn run(
    mut env_0: *mut Env,
    // 1431: mut env_0: typeof(_1) = *mut {g33} DefId(0:327 ~ lglmbt[b165]::Env)
    // 1431: mut env_0:   g33 = UNIQUE | NON_NULL, FIXED
    mut process: Option<unsafe extern "C" fn(*mut Env) -> ()>,
    // 1432: mut process: typeof(_2) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g34} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
    // 1432: mut process:   g34 = UNIQUE | NON_NULL, FIXED
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut saved1: libc::c_int = 0;
    let mut saved2: libc::c_int = 0;
    let mut null: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut id: pid_t = 0;
    (*env_0).forked += 1;
    (*env_0).forked;
    fflush(stdout);
    // 1443: stdout: typeof(_14) = *mut {l14} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1443: stdout:   l14 = UNIQUE | NON_NULL, (empty)
    // 1443: stdout: typeof(_15) = *mut {l16} *mut {l17} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1443: stdout:   l16 = UNIQUE | NON_NULL, (empty)
    // 1443: stdout:   l17 = UNIQUE | NON_NULL, (empty)
    id = fork();
    if id != 0 {
        let mut wid: pid_t = wait(&mut status);
        // 1446: &mut status: typeof(_21) = *mut {l24} i32
        // 1446: &mut status:   l24 = UNIQUE | NON_NULL, (empty)
        // 1446: &mut status: typeof(_22) = &mut {l26} i32
        // 1446: &mut status:   l26 = UNIQUE | NON_NULL, (empty)
        // 1446: &mut status: typeof(_21 = &raw mut (*_22)) = *mut {l510} i32
        // 1446: &mut status:   l510 = UNIQUE | NON_NULL, (empty)
        // 1446: &mut status: typeof(_22 = &mut _5) = &mut {l509} i32
        // 1446: &mut status:   l509 = UNIQUE | NON_NULL, (empty)
        if wid == id {
        } else {
            __assert_fail(
                b"wid == id\0" as *const u8 as *const libc::c_char,
                // 1450: b"wid == id\0"  ... _char: typeof(_29) = *const {l34} i8
                // 1450: b"wid == id\0"  ... _char:   l34 = UNIQUE | NON_NULL, (empty)
                // 1450: b"wid == id\0"  ... st u8: typeof(_30) = *const {l36} u8
                // 1450: b"wid == id\0"  ... st u8:   l36 = UNIQUE | NON_NULL, (empty)
                // 1450: b"wid == id\0": typeof(_31) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1450: b"wid == id\0":   l38 = UNIQUE | NON_NULL, (empty)
                // 1450: b"wid == id\0": typeof(_32) = & {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1450: b"wid == id\0":   l40 = UNIQUE | NON_NULL, FIXED
                // 1450: b"wid == id\0": typeof(_31 = &raw const (*_32)) = *const {l512} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1450: b"wid == id\0":   l512 = UNIQUE | NON_NULL, (empty)
                // 1450: b"wid == id\0"  ... _char: typeof(_29 = move _30 as *const i8 (Misc)) = *const {l514} i8
                // 1450: b"wid == id\0"  ... _char:   l514 = UNIQUE | NON_NULL, (empty)
                // 1450: b"wid == id\0": typeof(_32 = const b"wid == id\x00") = & {l511} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                // 1450: b"wid == id\0":   l511 = UNIQUE | NON_NULL, (empty)
                // 1450: b"wid == id\0"  ... st u8: typeof(_30 = move _31 as *const u8 (Pointer(ArrayToPointer))) = *const {l513} u8
                // 1450: b"wid == id\0"  ... st u8:   l513 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1451: b"lglmbt.c\0" a ... _char: typeof(_33) = *const {l42} i8
                // 1451: b"lglmbt.c\0" a ... _char:   l42 = UNIQUE | NON_NULL, (empty)
                // 1451: b"lglmbt.c\0" a ... st u8: typeof(_34) = *const {l44} u8
                // 1451: b"lglmbt.c\0" a ... st u8:   l44 = UNIQUE | NON_NULL, (empty)
                // 1451: b"lglmbt.c\0": typeof(_35) = *const {l46} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1451: b"lglmbt.c\0":   l46 = UNIQUE | NON_NULL, (empty)
                // 1451: b"lglmbt.c\0": typeof(_36) = & {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1451: b"lglmbt.c\0":   l48 = UNIQUE | NON_NULL, FIXED
                // 1451: b"lglmbt.c\0": typeof(_36 = const b"lglmbt.c\x00") = & {l515} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1451: b"lglmbt.c\0":   l515 = UNIQUE | NON_NULL, (empty)
                // 1451: b"lglmbt.c\0" a ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l517} u8
                // 1451: b"lglmbt.c\0" a ... st u8:   l517 = UNIQUE | NON_NULL, (empty)
                // 1451: b"lglmbt.c\0" a ... _char: typeof(_33 = move _34 as *const i8 (Misc)) = *const {l518} i8
                // 1451: b"lglmbt.c\0" a ... _char:   l518 = UNIQUE | NON_NULL, (empty)
                // 1451: b"lglmbt.c\0": typeof(_35 = &raw const (*_36)) = *const {l516} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1451: b"lglmbt.c\0":   l516 = UNIQUE | NON_NULL, (empty)
                455 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                // 1453: (*::core::mem:: ... ptr(): typeof(_39) = *const {l52} i8
                // 1453: (*::core::mem:: ... ptr():   l52 = UNIQUE | NON_NULL, (empty)
                // 1453: (*::core::mem:: ... ptr(): typeof(_40) = & {l54} [i8]
                // 1453: (*::core::mem:: ... ptr():   l54 = UNIQUE | NON_NULL, FIXED
                // 1453: (*::core::mem:: ... ptr(): typeof(_41) = & {l56} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1453: (*::core::mem:: ... ptr():   l56 = UNIQUE | NON_NULL, (empty)
                // 1453: ::core::mem::tr ... 0", ): typeof(_42) = & {l58} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1453: ::core::mem::tr ... 0", ):   l58 = UNIQUE | NON_NULL, FIXED
                // 1453: (*::core::mem:: ... ptr(): typeof(_40 = move _41 as &[i8] (Pointer(Unsize))) = & {l522} [i8]
                // 1453: (*::core::mem:: ... ptr():   l522 = UNIQUE | NON_NULL, FIXED
                // 1453: (*::core::mem:: ... ptr(): typeof(_41 = &(*_42)) = & {l521} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1453: (*::core::mem:: ... ptr():   l521 = UNIQUE | NON_NULL, (empty)
                    b"int run(Env *, void (*)(Env *))\0",
                    // 1454: b"int run(Env * ... ))\0": typeof(_43) = & {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1454: b"int run(Env * ... ))\0":   l60 = UNIQUE | NON_NULL, (empty)
                    // 1454: b"int run(Env * ... ))\0": typeof(_44) = & {l62} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1454: b"int run(Env * ... ))\0":   l62 = UNIQUE | NON_NULL, FIXED
                    // 1454: b"int run(Env * ... ))\0": typeof(_44 = const b"int run(Env *, void (*)(Env *))\x00") = & {l519} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1454: b"int run(Env * ... ))\0":   l519 = UNIQUE | NON_NULL, (empty)
                    // 1454: b"int run(Env * ... ))\0": typeof(_43 = &(*_44)) = & {l520} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1454: b"int run(Env * ... ))\0":   l520 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_7963: {
            if wid == id {
            } else {
                __assert_fail(
                    b"wid == id\0" as *const u8 as *const libc::c_char,
                    // 1463: b"wid == id\0"  ... _char: typeof(_51) = *const {l70} i8
                    // 1463: b"wid == id\0"  ... _char:   l70 = UNIQUE | NON_NULL, (empty)
                    // 1463: b"wid == id\0"  ... st u8: typeof(_52) = *const {l72} u8
                    // 1463: b"wid == id\0"  ... st u8:   l72 = UNIQUE | NON_NULL, (empty)
                    // 1463: b"wid == id\0": typeof(_53) = *const {l74} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 1463: b"wid == id\0":   l74 = UNIQUE | NON_NULL, (empty)
                    // 1463: b"wid == id\0": typeof(_54) = & {l76} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 1463: b"wid == id\0":   l76 = UNIQUE | NON_NULL, FIXED
                    // 1463: b"wid == id\0"  ... _char: typeof(_51 = move _52 as *const i8 (Misc)) = *const {l526} i8
                    // 1463: b"wid == id\0"  ... _char:   l526 = UNIQUE | NON_NULL, (empty)
                    // 1463: b"wid == id\0": typeof(_53 = &raw const (*_54)) = *const {l524} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 1463: b"wid == id\0":   l524 = UNIQUE | NON_NULL, (empty)
                    // 1463: b"wid == id\0": typeof(_54 = const b"wid == id\x00") = & {l523} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
                    // 1463: b"wid == id\0":   l523 = UNIQUE | NON_NULL, (empty)
                    // 1463: b"wid == id\0"  ... st u8: typeof(_52 = move _53 as *const u8 (Pointer(ArrayToPointer))) = *const {l525} u8
                    // 1463: b"wid == id\0"  ... st u8:   l525 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1464: b"lglmbt.c\0" a ... _char: typeof(_55) = *const {l78} i8
                    // 1464: b"lglmbt.c\0" a ... _char:   l78 = UNIQUE | NON_NULL, (empty)
                    // 1464: b"lglmbt.c\0" a ... st u8: typeof(_56) = *const {l80} u8
                    // 1464: b"lglmbt.c\0" a ... st u8:   l80 = UNIQUE | NON_NULL, (empty)
                    // 1464: b"lglmbt.c\0": typeof(_57) = *const {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1464: b"lglmbt.c\0":   l82 = UNIQUE | NON_NULL, (empty)
                    // 1464: b"lglmbt.c\0": typeof(_58) = & {l84} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1464: b"lglmbt.c\0":   l84 = UNIQUE | NON_NULL, FIXED
                    // 1464: b"lglmbt.c\0": typeof(_58 = const b"lglmbt.c\x00") = & {l527} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1464: b"lglmbt.c\0":   l527 = UNIQUE | NON_NULL, (empty)
                    // 1464: b"lglmbt.c\0": typeof(_57 = &raw const (*_58)) = *const {l528} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1464: b"lglmbt.c\0":   l528 = UNIQUE | NON_NULL, (empty)
                    // 1464: b"lglmbt.c\0" a ... st u8: typeof(_56 = move _57 as *const u8 (Pointer(ArrayToPointer))) = *const {l529} u8
                    // 1464: b"lglmbt.c\0" a ... st u8:   l529 = UNIQUE | NON_NULL, (empty)
                    // 1464: b"lglmbt.c\0" a ... _char: typeof(_55 = move _56 as *const i8 (Misc)) = *const {l530} i8
                    // 1464: b"lglmbt.c\0" a ... _char:   l530 = UNIQUE | NON_NULL, (empty)
                    455 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 1466: (*::core::mem:: ... ptr(): typeof(_61) = *const {l88} i8
                    // 1466: (*::core::mem:: ... ptr():   l88 = UNIQUE | NON_NULL, (empty)
                    // 1466: (*::core::mem:: ... ptr(): typeof(_62) = & {l90} [i8]
                    // 1466: (*::core::mem:: ... ptr():   l90 = UNIQUE | NON_NULL, FIXED
                    // 1466: (*::core::mem:: ... ptr(): typeof(_63) = & {l92} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1466: (*::core::mem:: ... ptr():   l92 = UNIQUE | NON_NULL, (empty)
                    // 1466: ::core::mem::tr ... 0", ): typeof(_64) = & {l94} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1466: ::core::mem::tr ... 0", ):   l94 = UNIQUE | NON_NULL, FIXED
                    // 1466: (*::core::mem:: ... ptr(): typeof(_63 = &(*_64)) = & {l533} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1466: (*::core::mem:: ... ptr():   l533 = UNIQUE | NON_NULL, (empty)
                    // 1466: (*::core::mem:: ... ptr(): typeof(_62 = move _63 as &[i8] (Pointer(Unsize))) = & {l534} [i8]
                    // 1466: (*::core::mem:: ... ptr():   l534 = UNIQUE | NON_NULL, FIXED
                        b"int run(Env *, void (*)(Env *))\0",
                        // 1467: b"int run(Env * ... ))\0": typeof(_65) = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1467: b"int run(Env * ... ))\0":   l96 = UNIQUE | NON_NULL, (empty)
                        // 1467: b"int run(Env * ... ))\0": typeof(_66) = & {l98} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1467: b"int run(Env * ... ))\0":   l98 = UNIQUE | NON_NULL, FIXED
                        // 1467: b"int run(Env * ... ))\0": typeof(_65 = &(*_66)) = & {l532} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1467: b"int run(Env * ... ))\0":   l532 = UNIQUE | NON_NULL, (empty)
                        // 1467: b"int run(Env * ... ))\0": typeof(_66 = const b"int run(Env *, void (*)(Env *))\x00") = & {l531} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1467: b"int run(Env * ... ))\0":   l531 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
        };
    } else {
        saved1 = dup(1 as libc::c_int);
        saved2 = dup(2 as libc::c_int);
        null = open(
            b"/dev/null\0" as *const u8 as *const libc::c_char,
            // 1477: b"/dev/null\0"  ... _char: typeof(_73) = *const {l106} i8
            // 1477: b"/dev/null\0"  ... _char:   l106 = UNIQUE | NON_NULL, (empty)
            // 1477: b"/dev/null\0"  ... st u8: typeof(_74) = *const {l108} u8
            // 1477: b"/dev/null\0"  ... st u8:   l108 = UNIQUE | NON_NULL, (empty)
            // 1477: b"/dev/null\0": typeof(_75) = *const {l110} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1477: b"/dev/null\0":   l110 = UNIQUE | NON_NULL, (empty)
            // 1477: b"/dev/null\0": typeof(_76) = & {l112} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1477: b"/dev/null\0":   l112 = UNIQUE | NON_NULL, FIXED
            // 1477: b"/dev/null\0": typeof(_75 = &raw const (*_76)) = *const {l536} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1477: b"/dev/null\0":   l536 = UNIQUE | NON_NULL, (empty)
            // 1477: b"/dev/null\0": typeof(_76 = const b"/dev/null\x00") = & {l535} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
            // 1477: b"/dev/null\0":   l535 = UNIQUE | NON_NULL, (empty)
            // 1477: b"/dev/null\0"  ... _char: typeof(_73 = move _74 as *const i8 (Misc)) = *const {l538} i8
            // 1477: b"/dev/null\0"  ... _char:   l538 = UNIQUE | NON_NULL, (empty)
            // 1477: b"/dev/null\0"  ... st u8: typeof(_74 = move _75 as *const u8 (Pointer(ArrayToPointer))) = *const {l537} u8
            // 1477: b"/dev/null\0"  ... st u8:   l537 = UNIQUE | NON_NULL, (empty)
            0o1 as libc::c_int,
        );
        close(1 as libc::c_int);
        close(2 as libc::c_int);
        tmp = dup(null);
        if tmp == 1 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                // 1486: b"tmp == 1\0" a ... _char: typeof(_90) = *const {l127} i8
                // 1486: b"tmp == 1\0" a ... _char:   l127 = UNIQUE | NON_NULL, (empty)
                // 1486: b"tmp == 1\0" a ... st u8: typeof(_91) = *const {l129} u8
                // 1486: b"tmp == 1\0" a ... st u8:   l129 = UNIQUE | NON_NULL, (empty)
                // 1486: b"tmp == 1\0": typeof(_92) = *const {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1486: b"tmp == 1\0":   l131 = UNIQUE | NON_NULL, (empty)
                // 1486: b"tmp == 1\0": typeof(_93) = & {l133} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1486: b"tmp == 1\0":   l133 = UNIQUE | NON_NULL, FIXED
                // 1486: b"tmp == 1\0" a ... _char: typeof(_90 = move _91 as *const i8 (Misc)) = *const {l542} i8
                // 1486: b"tmp == 1\0" a ... _char:   l542 = UNIQUE | NON_NULL, (empty)
                // 1486: b"tmp == 1\0": typeof(_93 = const b"tmp == 1\x00") = & {l539} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1486: b"tmp == 1\0":   l539 = UNIQUE | NON_NULL, (empty)
                // 1486: b"tmp == 1\0": typeof(_92 = &raw const (*_93)) = *const {l540} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1486: b"tmp == 1\0":   l540 = UNIQUE | NON_NULL, (empty)
                // 1486: b"tmp == 1\0" a ... st u8: typeof(_91 = move _92 as *const u8 (Pointer(ArrayToPointer))) = *const {l541} u8
                // 1486: b"tmp == 1\0" a ... st u8:   l541 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1487: b"lglmbt.c\0" a ... _char: typeof(_94) = *const {l135} i8
                // 1487: b"lglmbt.c\0" a ... _char:   l135 = UNIQUE | NON_NULL, (empty)
                // 1487: b"lglmbt.c\0" a ... st u8: typeof(_95) = *const {l137} u8
                // 1487: b"lglmbt.c\0" a ... st u8:   l137 = UNIQUE | NON_NULL, (empty)
                // 1487: b"lglmbt.c\0": typeof(_96) = *const {l139} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1487: b"lglmbt.c\0":   l139 = UNIQUE | NON_NULL, (empty)
                // 1487: b"lglmbt.c\0": typeof(_97) = & {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1487: b"lglmbt.c\0":   l141 = UNIQUE | NON_NULL, FIXED
                // 1487: b"lglmbt.c\0" a ... _char: typeof(_94 = move _95 as *const i8 (Misc)) = *const {l546} i8
                // 1487: b"lglmbt.c\0" a ... _char:   l546 = UNIQUE | NON_NULL, (empty)
                // 1487: b"lglmbt.c\0": typeof(_96 = &raw const (*_97)) = *const {l544} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1487: b"lglmbt.c\0":   l544 = UNIQUE | NON_NULL, (empty)
                // 1487: b"lglmbt.c\0": typeof(_97 = const b"lglmbt.c\x00") = & {l543} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1487: b"lglmbt.c\0":   l543 = UNIQUE | NON_NULL, (empty)
                // 1487: b"lglmbt.c\0" a ... st u8: typeof(_95 = move _96 as *const u8 (Pointer(ArrayToPointer))) = *const {l545} u8
                // 1487: b"lglmbt.c\0" a ... st u8:   l545 = UNIQUE | NON_NULL, (empty)
                466 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                // 1489: (*::core::mem:: ... ptr(): typeof(_100) = *const {l145} i8
                // 1489: (*::core::mem:: ... ptr():   l145 = UNIQUE | NON_NULL, (empty)
                // 1489: (*::core::mem:: ... ptr(): typeof(_101) = & {l147} [i8]
                // 1489: (*::core::mem:: ... ptr():   l147 = UNIQUE | NON_NULL, FIXED
                // 1489: (*::core::mem:: ... ptr(): typeof(_102) = & {l149} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1489: (*::core::mem:: ... ptr():   l149 = UNIQUE | NON_NULL, (empty)
                // 1489: ::core::mem::tr ... 0", ): typeof(_103) = & {l151} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1489: ::core::mem::tr ... 0", ):   l151 = UNIQUE | NON_NULL, FIXED
                // 1489: (*::core::mem:: ... ptr(): typeof(_101 = move _102 as &[i8] (Pointer(Unsize))) = & {l550} [i8]
                // 1489: (*::core::mem:: ... ptr():   l550 = UNIQUE | NON_NULL, FIXED
                // 1489: (*::core::mem:: ... ptr(): typeof(_102 = &(*_103)) = & {l549} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1489: (*::core::mem:: ... ptr():   l549 = UNIQUE | NON_NULL, (empty)
                    b"int run(Env *, void (*)(Env *))\0",
                    // 1490: b"int run(Env * ... ))\0": typeof(_104) = & {l153} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1490: b"int run(Env * ... ))\0":   l153 = UNIQUE | NON_NULL, (empty)
                    // 1490: b"int run(Env * ... ))\0": typeof(_105) = & {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1490: b"int run(Env * ... ))\0":   l155 = UNIQUE | NON_NULL, FIXED
                    // 1490: b"int run(Env * ... ))\0": typeof(_105 = const b"int run(Env *, void (*)(Env *))\x00") = & {l547} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1490: b"int run(Env * ... ))\0":   l547 = UNIQUE | NON_NULL, (empty)
                    // 1490: b"int run(Env * ... ))\0": typeof(_104 = &(*_105)) = & {l548} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1490: b"int run(Env * ... ))\0":   l548 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_7887: {
            if tmp == 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 1\0" as *const u8 as *const libc::c_char,
                    // 1499: b"tmp == 1\0" a ... _char: typeof(_112) = *const {l163} i8
                    // 1499: b"tmp == 1\0" a ... _char:   l163 = UNIQUE | NON_NULL, (empty)
                    // 1499: b"tmp == 1\0" a ... st u8: typeof(_113) = *const {l165} u8
                    // 1499: b"tmp == 1\0" a ... st u8:   l165 = UNIQUE | NON_NULL, (empty)
                    // 1499: b"tmp == 1\0": typeof(_114) = *const {l167} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1499: b"tmp == 1\0":   l167 = UNIQUE | NON_NULL, (empty)
                    // 1499: b"tmp == 1\0": typeof(_115) = & {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1499: b"tmp == 1\0":   l169 = UNIQUE | NON_NULL, FIXED
                    // 1499: b"tmp == 1\0": typeof(_114 = &raw const (*_115)) = *const {l552} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1499: b"tmp == 1\0":   l552 = UNIQUE | NON_NULL, (empty)
                    // 1499: b"tmp == 1\0" a ... _char: typeof(_112 = move _113 as *const i8 (Misc)) = *const {l554} i8
                    // 1499: b"tmp == 1\0" a ... _char:   l554 = UNIQUE | NON_NULL, (empty)
                    // 1499: b"tmp == 1\0": typeof(_115 = const b"tmp == 1\x00") = & {l551} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1499: b"tmp == 1\0":   l551 = UNIQUE | NON_NULL, (empty)
                    // 1499: b"tmp == 1\0" a ... st u8: typeof(_113 = move _114 as *const u8 (Pointer(ArrayToPointer))) = *const {l553} u8
                    // 1499: b"tmp == 1\0" a ... st u8:   l553 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1500: b"lglmbt.c\0" a ... _char: typeof(_116) = *const {l171} i8
                    // 1500: b"lglmbt.c\0" a ... _char:   l171 = UNIQUE | NON_NULL, (empty)
                    // 1500: b"lglmbt.c\0" a ... st u8: typeof(_117) = *const {l173} u8
                    // 1500: b"lglmbt.c\0" a ... st u8:   l173 = UNIQUE | NON_NULL, (empty)
                    // 1500: b"lglmbt.c\0": typeof(_118) = *const {l175} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1500: b"lglmbt.c\0":   l175 = UNIQUE | NON_NULL, (empty)
                    // 1500: b"lglmbt.c\0": typeof(_119) = & {l177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1500: b"lglmbt.c\0":   l177 = UNIQUE | NON_NULL, FIXED
                    // 1500: b"lglmbt.c\0" a ... st u8: typeof(_117 = move _118 as *const u8 (Pointer(ArrayToPointer))) = *const {l557} u8
                    // 1500: b"lglmbt.c\0" a ... st u8:   l557 = UNIQUE | NON_NULL, (empty)
                    // 1500: b"lglmbt.c\0" a ... _char: typeof(_116 = move _117 as *const i8 (Misc)) = *const {l558} i8
                    // 1500: b"lglmbt.c\0" a ... _char:   l558 = UNIQUE | NON_NULL, (empty)
                    // 1500: b"lglmbt.c\0": typeof(_119 = const b"lglmbt.c\x00") = & {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1500: b"lglmbt.c\0":   l555 = UNIQUE | NON_NULL, (empty)
                    // 1500: b"lglmbt.c\0": typeof(_118 = &raw const (*_119)) = *const {l556} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1500: b"lglmbt.c\0":   l556 = UNIQUE | NON_NULL, (empty)
                    466 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 1502: (*::core::mem:: ... ptr(): typeof(_122) = *const {l181} i8
                    // 1502: (*::core::mem:: ... ptr():   l181 = UNIQUE | NON_NULL, (empty)
                    // 1502: (*::core::mem:: ... ptr(): typeof(_123) = & {l183} [i8]
                    // 1502: (*::core::mem:: ... ptr():   l183 = UNIQUE | NON_NULL, FIXED
                    // 1502: (*::core::mem:: ... ptr(): typeof(_124) = & {l185} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1502: (*::core::mem:: ... ptr():   l185 = UNIQUE | NON_NULL, (empty)
                    // 1502: ::core::mem::tr ... 0", ): typeof(_125) = & {l187} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1502: ::core::mem::tr ... 0", ):   l187 = UNIQUE | NON_NULL, FIXED
                    // 1502: (*::core::mem:: ... ptr(): typeof(_123 = move _124 as &[i8] (Pointer(Unsize))) = & {l562} [i8]
                    // 1502: (*::core::mem:: ... ptr():   l562 = UNIQUE | NON_NULL, FIXED
                    // 1502: (*::core::mem:: ... ptr(): typeof(_124 = &(*_125)) = & {l561} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1502: (*::core::mem:: ... ptr():   l561 = UNIQUE | NON_NULL, (empty)
                        b"int run(Env *, void (*)(Env *))\0",
                        // 1503: b"int run(Env * ... ))\0": typeof(_126) = & {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1503: b"int run(Env * ... ))\0":   l189 = UNIQUE | NON_NULL, (empty)
                        // 1503: b"int run(Env * ... ))\0": typeof(_127) = & {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1503: b"int run(Env * ... ))\0":   l191 = UNIQUE | NON_NULL, FIXED
                        // 1503: b"int run(Env * ... ))\0": typeof(_127 = const b"int run(Env *, void (*)(Env *))\x00") = & {l559} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1503: b"int run(Env * ... ))\0":   l559 = UNIQUE | NON_NULL, (empty)
                        // 1503: b"int run(Env * ... ))\0": typeof(_126 = &(*_127)) = & {l560} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1503: b"int run(Env * ... ))\0":   l560 = UNIQUE | NON_NULL, (empty)
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
                // 1513: b"tmp == 2\0" a ... _char: typeof(_136) = *const {l201} i8
                // 1513: b"tmp == 2\0" a ... _char:   l201 = UNIQUE | NON_NULL, (empty)
                // 1513: b"tmp == 2\0" a ... st u8: typeof(_137) = *const {l203} u8
                // 1513: b"tmp == 2\0" a ... st u8:   l203 = UNIQUE | NON_NULL, (empty)
                // 1513: b"tmp == 2\0": typeof(_138) = *const {l205} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1513: b"tmp == 2\0":   l205 = UNIQUE | NON_NULL, (empty)
                // 1513: b"tmp == 2\0": typeof(_139) = & {l207} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1513: b"tmp == 2\0":   l207 = UNIQUE | NON_NULL, FIXED
                // 1513: b"tmp == 2\0": typeof(_139 = const b"tmp == 2\x00") = & {l563} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1513: b"tmp == 2\0":   l563 = UNIQUE | NON_NULL, (empty)
                // 1513: b"tmp == 2\0": typeof(_138 = &raw const (*_139)) = *const {l564} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1513: b"tmp == 2\0":   l564 = UNIQUE | NON_NULL, (empty)
                // 1513: b"tmp == 2\0" a ... st u8: typeof(_137 = move _138 as *const u8 (Pointer(ArrayToPointer))) = *const {l565} u8
                // 1513: b"tmp == 2\0" a ... st u8:   l565 = UNIQUE | NON_NULL, (empty)
                // 1513: b"tmp == 2\0" a ... _char: typeof(_136 = move _137 as *const i8 (Misc)) = *const {l566} i8
                // 1513: b"tmp == 2\0" a ... _char:   l566 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1514: b"lglmbt.c\0" a ... _char: typeof(_140) = *const {l209} i8
                // 1514: b"lglmbt.c\0" a ... _char:   l209 = UNIQUE | NON_NULL, (empty)
                // 1514: b"lglmbt.c\0" a ... st u8: typeof(_141) = *const {l211} u8
                // 1514: b"lglmbt.c\0" a ... st u8:   l211 = UNIQUE | NON_NULL, (empty)
                // 1514: b"lglmbt.c\0": typeof(_142) = *const {l213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1514: b"lglmbt.c\0":   l213 = UNIQUE | NON_NULL, (empty)
                // 1514: b"lglmbt.c\0": typeof(_143) = & {l215} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1514: b"lglmbt.c\0":   l215 = UNIQUE | NON_NULL, FIXED
                // 1514: b"lglmbt.c\0" a ... _char: typeof(_140 = move _141 as *const i8 (Misc)) = *const {l570} i8
                // 1514: b"lglmbt.c\0" a ... _char:   l570 = UNIQUE | NON_NULL, (empty)
                // 1514: b"lglmbt.c\0" a ... st u8: typeof(_141 = move _142 as *const u8 (Pointer(ArrayToPointer))) = *const {l569} u8
                // 1514: b"lglmbt.c\0" a ... st u8:   l569 = UNIQUE | NON_NULL, (empty)
                // 1514: b"lglmbt.c\0": typeof(_142 = &raw const (*_143)) = *const {l568} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1514: b"lglmbt.c\0":   l568 = UNIQUE | NON_NULL, (empty)
                // 1514: b"lglmbt.c\0": typeof(_143 = const b"lglmbt.c\x00") = & {l567} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1514: b"lglmbt.c\0":   l567 = UNIQUE | NON_NULL, (empty)
                471 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                // 1516: (*::core::mem:: ... ptr(): typeof(_146) = *const {l219} i8
                // 1516: (*::core::mem:: ... ptr():   l219 = UNIQUE | NON_NULL, (empty)
                // 1516: (*::core::mem:: ... ptr(): typeof(_147) = & {l221} [i8]
                // 1516: (*::core::mem:: ... ptr():   l221 = UNIQUE | NON_NULL, FIXED
                // 1516: (*::core::mem:: ... ptr(): typeof(_148) = & {l223} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1516: (*::core::mem:: ... ptr():   l223 = UNIQUE | NON_NULL, (empty)
                // 1516: ::core::mem::tr ... 0", ): typeof(_149) = & {l225} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1516: ::core::mem::tr ... 0", ):   l225 = UNIQUE | NON_NULL, FIXED
                // 1516: (*::core::mem:: ... ptr(): typeof(_148 = &(*_149)) = & {l573} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1516: (*::core::mem:: ... ptr():   l573 = UNIQUE | NON_NULL, (empty)
                // 1516: (*::core::mem:: ... ptr(): typeof(_147 = move _148 as &[i8] (Pointer(Unsize))) = & {l574} [i8]
                // 1516: (*::core::mem:: ... ptr():   l574 = UNIQUE | NON_NULL, FIXED
                    b"int run(Env *, void (*)(Env *))\0",
                    // 1517: b"int run(Env * ... ))\0": typeof(_150) = & {l227} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1517: b"int run(Env * ... ))\0":   l227 = UNIQUE | NON_NULL, (empty)
                    // 1517: b"int run(Env * ... ))\0": typeof(_151) = & {l229} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1517: b"int run(Env * ... ))\0":   l229 = UNIQUE | NON_NULL, FIXED
                    // 1517: b"int run(Env * ... ))\0": typeof(_150 = &(*_151)) = & {l572} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1517: b"int run(Env * ... ))\0":   l572 = UNIQUE | NON_NULL, (empty)
                    // 1517: b"int run(Env * ... ))\0": typeof(_151 = const b"int run(Env *, void (*)(Env *))\x00") = & {l571} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1517: b"int run(Env * ... ))\0":   l571 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_7844: {
            if tmp == 2 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 2\0" as *const u8 as *const libc::c_char,
                    // 1526: b"tmp == 2\0" a ... _char: typeof(_158) = *const {l237} i8
                    // 1526: b"tmp == 2\0" a ... _char:   l237 = UNIQUE | NON_NULL, (empty)
                    // 1526: b"tmp == 2\0" a ... st u8: typeof(_159) = *const {l239} u8
                    // 1526: b"tmp == 2\0" a ... st u8:   l239 = UNIQUE | NON_NULL, (empty)
                    // 1526: b"tmp == 2\0": typeof(_160) = *const {l241} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1526: b"tmp == 2\0":   l241 = UNIQUE | NON_NULL, (empty)
                    // 1526: b"tmp == 2\0": typeof(_161) = & {l243} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1526: b"tmp == 2\0":   l243 = UNIQUE | NON_NULL, FIXED
                    // 1526: b"tmp == 2\0": typeof(_161 = const b"tmp == 2\x00") = & {l575} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1526: b"tmp == 2\0":   l575 = UNIQUE | NON_NULL, (empty)
                    // 1526: b"tmp == 2\0": typeof(_160 = &raw const (*_161)) = *const {l576} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1526: b"tmp == 2\0":   l576 = UNIQUE | NON_NULL, (empty)
                    // 1526: b"tmp == 2\0" a ... _char: typeof(_158 = move _159 as *const i8 (Misc)) = *const {l578} i8
                    // 1526: b"tmp == 2\0" a ... _char:   l578 = UNIQUE | NON_NULL, (empty)
                    // 1526: b"tmp == 2\0" a ... st u8: typeof(_159 = move _160 as *const u8 (Pointer(ArrayToPointer))) = *const {l577} u8
                    // 1526: b"tmp == 2\0" a ... st u8:   l577 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1527: b"lglmbt.c\0" a ... _char: typeof(_162) = *const {l245} i8
                    // 1527: b"lglmbt.c\0" a ... _char:   l245 = UNIQUE | NON_NULL, (empty)
                    // 1527: b"lglmbt.c\0" a ... st u8: typeof(_163) = *const {l247} u8
                    // 1527: b"lglmbt.c\0" a ... st u8:   l247 = UNIQUE | NON_NULL, (empty)
                    // 1527: b"lglmbt.c\0": typeof(_164) = *const {l249} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1527: b"lglmbt.c\0":   l249 = UNIQUE | NON_NULL, (empty)
                    // 1527: b"lglmbt.c\0": typeof(_165) = & {l251} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1527: b"lglmbt.c\0":   l251 = UNIQUE | NON_NULL, FIXED
                    // 1527: b"lglmbt.c\0": typeof(_165 = const b"lglmbt.c\x00") = & {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1527: b"lglmbt.c\0":   l579 = UNIQUE | NON_NULL, (empty)
                    // 1527: b"lglmbt.c\0" a ... st u8: typeof(_163 = move _164 as *const u8 (Pointer(ArrayToPointer))) = *const {l581} u8
                    // 1527: b"lglmbt.c\0" a ... st u8:   l581 = UNIQUE | NON_NULL, (empty)
                    // 1527: b"lglmbt.c\0" a ... _char: typeof(_162 = move _163 as *const i8 (Misc)) = *const {l582} i8
                    // 1527: b"lglmbt.c\0" a ... _char:   l582 = UNIQUE | NON_NULL, (empty)
                    // 1527: b"lglmbt.c\0": typeof(_164 = &raw const (*_165)) = *const {l580} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1527: b"lglmbt.c\0":   l580 = UNIQUE | NON_NULL, (empty)
                    471 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 1529: (*::core::mem:: ... ptr(): typeof(_168) = *const {l255} i8
                    // 1529: (*::core::mem:: ... ptr():   l255 = UNIQUE | NON_NULL, (empty)
                    // 1529: (*::core::mem:: ... ptr(): typeof(_169) = & {l257} [i8]
                    // 1529: (*::core::mem:: ... ptr():   l257 = UNIQUE | NON_NULL, FIXED
                    // 1529: (*::core::mem:: ... ptr(): typeof(_170) = & {l259} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1529: (*::core::mem:: ... ptr():   l259 = UNIQUE | NON_NULL, (empty)
                    // 1529: ::core::mem::tr ... 0", ): typeof(_171) = & {l261} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1529: ::core::mem::tr ... 0", ):   l261 = UNIQUE | NON_NULL, FIXED
                    // 1529: (*::core::mem:: ... ptr(): typeof(_170 = &(*_171)) = & {l585} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1529: (*::core::mem:: ... ptr():   l585 = UNIQUE | NON_NULL, (empty)
                    // 1529: (*::core::mem:: ... ptr(): typeof(_169 = move _170 as &[i8] (Pointer(Unsize))) = & {l586} [i8]
                    // 1529: (*::core::mem:: ... ptr():   l586 = UNIQUE | NON_NULL, FIXED
                        b"int run(Env *, void (*)(Env *))\0",
                        // 1530: b"int run(Env * ... ))\0": typeof(_172) = & {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1530: b"int run(Env * ... ))\0":   l263 = UNIQUE | NON_NULL, (empty)
                        // 1530: b"int run(Env * ... ))\0": typeof(_173) = & {l265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1530: b"int run(Env * ... ))\0":   l265 = UNIQUE | NON_NULL, FIXED
                        // 1530: b"int run(Env * ... ))\0": typeof(_172 = &(*_173)) = & {l584} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1530: b"int run(Env * ... ))\0":   l584 = UNIQUE | NON_NULL, (empty)
                        // 1530: b"int run(Env * ... ))\0": typeof(_173 = const b"int run(Env *, void (*)(Env *))\x00") = & {l583} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1530: b"int run(Env * ... ))\0":   l583 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
        };
        if (*env_0).timeout > 0 as libc::c_int {
            alarm((*env_0).timeout as libc::c_uint);
        }
        process.expect("non-null function pointer")(env_0);
        // 1539: process.expect( ... ter"): typeof(_182) = fn(*mut {l275} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
        // 1539: process.expect( ... ter"):   l275 = UNIQUE | NON_NULL, (empty)
        // 1539: process: typeof(_183) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l277} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
        // 1539: process:   l277 = UNIQUE | NON_NULL, (empty)
        // 1539: "non-null funct ... nter": typeof(_184) = & {l279} str
        // 1539: "non-null funct ... nter":   l279 = UNIQUE | NON_NULL, (empty)
        // 1539: "non-null funct ... nter": typeof(_185) = & {l281} str
        // 1539: "non-null funct ... nter":   l281 = UNIQUE | NON_NULL, FIXED
        // 1539: env_0: typeof(_186) = *mut {l283} DefId(0:327 ~ lglmbt[b165]::Env)
        // 1539: env_0:   l283 = UNIQUE | NON_NULL, (empty)
        // 1539: "non-null funct ... nter": typeof(_184 = &(*_185)) = & {l588} str
        // 1539: "non-null funct ... nter":   l588 = UNIQUE | NON_NULL, (empty)
        // 1539: "non-null funct ... nter": typeof(_185 = const "non-null function pointer") = & {l587} str
        // 1539: "non-null funct ... nter":   l587 = UNIQUE | NON_NULL, (empty)
        close(null);
        close(2 as libc::c_int);
        tmp = dup(saved2);
        if tmp == 2 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 2\0" as *const u8 as *const libc::c_char,
                // 1546: b"tmp == 2\0" a ... _char: typeof(_199) = *const {l297} i8
                // 1546: b"tmp == 2\0" a ... _char:   l297 = UNIQUE | NON_NULL, (empty)
                // 1546: b"tmp == 2\0" a ... st u8: typeof(_200) = *const {l299} u8
                // 1546: b"tmp == 2\0" a ... st u8:   l299 = UNIQUE | NON_NULL, (empty)
                // 1546: b"tmp == 2\0": typeof(_201) = *const {l301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1546: b"tmp == 2\0":   l301 = UNIQUE | NON_NULL, (empty)
                // 1546: b"tmp == 2\0": typeof(_202) = & {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1546: b"tmp == 2\0":   l303 = UNIQUE | NON_NULL, FIXED
                // 1546: b"tmp == 2\0" a ... st u8: typeof(_200 = move _201 as *const u8 (Pointer(ArrayToPointer))) = *const {l591} u8
                // 1546: b"tmp == 2\0" a ... st u8:   l591 = UNIQUE | NON_NULL, (empty)
                // 1546: b"tmp == 2\0": typeof(_201 = &raw const (*_202)) = *const {l590} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1546: b"tmp == 2\0":   l590 = UNIQUE | NON_NULL, (empty)
                // 1546: b"tmp == 2\0": typeof(_202 = const b"tmp == 2\x00") = & {l589} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1546: b"tmp == 2\0":   l589 = UNIQUE | NON_NULL, (empty)
                // 1546: b"tmp == 2\0" a ... _char: typeof(_199 = move _200 as *const i8 (Misc)) = *const {l592} i8
                // 1546: b"tmp == 2\0" a ... _char:   l592 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1547: b"lglmbt.c\0" a ... _char: typeof(_203) = *const {l305} i8
                // 1547: b"lglmbt.c\0" a ... _char:   l305 = UNIQUE | NON_NULL, (empty)
                // 1547: b"lglmbt.c\0" a ... st u8: typeof(_204) = *const {l307} u8
                // 1547: b"lglmbt.c\0" a ... st u8:   l307 = UNIQUE | NON_NULL, (empty)
                // 1547: b"lglmbt.c\0": typeof(_205) = *const {l309} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1547: b"lglmbt.c\0":   l309 = UNIQUE | NON_NULL, (empty)
                // 1547: b"lglmbt.c\0": typeof(_206) = & {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1547: b"lglmbt.c\0":   l311 = UNIQUE | NON_NULL, FIXED
                // 1547: b"lglmbt.c\0": typeof(_205 = &raw const (*_206)) = *const {l594} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1547: b"lglmbt.c\0":   l594 = UNIQUE | NON_NULL, (empty)
                // 1547: b"lglmbt.c\0": typeof(_206 = const b"lglmbt.c\x00") = & {l593} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1547: b"lglmbt.c\0":   l593 = UNIQUE | NON_NULL, (empty)
                // 1547: b"lglmbt.c\0" a ... _char: typeof(_203 = move _204 as *const i8 (Misc)) = *const {l596} i8
                // 1547: b"lglmbt.c\0" a ... _char:   l596 = UNIQUE | NON_NULL, (empty)
                // 1547: b"lglmbt.c\0" a ... st u8: typeof(_204 = move _205 as *const u8 (Pointer(ArrayToPointer))) = *const {l595} u8
                // 1547: b"lglmbt.c\0" a ... st u8:   l595 = UNIQUE | NON_NULL, (empty)
                480 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                // 1549: (*::core::mem:: ... ptr(): typeof(_209) = *const {l315} i8
                // 1549: (*::core::mem:: ... ptr():   l315 = UNIQUE | NON_NULL, (empty)
                // 1549: (*::core::mem:: ... ptr(): typeof(_210) = & {l317} [i8]
                // 1549: (*::core::mem:: ... ptr():   l317 = UNIQUE | NON_NULL, FIXED
                // 1549: (*::core::mem:: ... ptr(): typeof(_211) = & {l319} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1549: (*::core::mem:: ... ptr():   l319 = UNIQUE | NON_NULL, (empty)
                // 1549: ::core::mem::tr ... 0", ): typeof(_212) = & {l321} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1549: ::core::mem::tr ... 0", ):   l321 = UNIQUE | NON_NULL, FIXED
                // 1549: (*::core::mem:: ... ptr(): typeof(_210 = move _211 as &[i8] (Pointer(Unsize))) = & {l600} [i8]
                // 1549: (*::core::mem:: ... ptr():   l600 = UNIQUE | NON_NULL, FIXED
                // 1549: (*::core::mem:: ... ptr(): typeof(_211 = &(*_212)) = & {l599} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1549: (*::core::mem:: ... ptr():   l599 = UNIQUE | NON_NULL, (empty)
                    b"int run(Env *, void (*)(Env *))\0",
                    // 1550: b"int run(Env * ... ))\0": typeof(_213) = & {l323} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1550: b"int run(Env * ... ))\0":   l323 = UNIQUE | NON_NULL, (empty)
                    // 1550: b"int run(Env * ... ))\0": typeof(_214) = & {l325} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1550: b"int run(Env * ... ))\0":   l325 = UNIQUE | NON_NULL, FIXED
                    // 1550: b"int run(Env * ... ))\0": typeof(_213 = &(*_214)) = & {l598} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1550: b"int run(Env * ... ))\0":   l598 = UNIQUE | NON_NULL, (empty)
                    // 1550: b"int run(Env * ... ))\0": typeof(_214 = const b"int run(Env *, void (*)(Env *))\x00") = & {l597} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1550: b"int run(Env * ... ))\0":   l597 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_7769: {
            if tmp == 2 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 2\0" as *const u8 as *const libc::c_char,
                    // 1559: b"tmp == 2\0" a ... _char: typeof(_221) = *const {l333} i8
                    // 1559: b"tmp == 2\0" a ... _char:   l333 = UNIQUE | NON_NULL, (empty)
                    // 1559: b"tmp == 2\0" a ... st u8: typeof(_222) = *const {l335} u8
                    // 1559: b"tmp == 2\0" a ... st u8:   l335 = UNIQUE | NON_NULL, (empty)
                    // 1559: b"tmp == 2\0": typeof(_223) = *const {l337} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1559: b"tmp == 2\0":   l337 = UNIQUE | NON_NULL, (empty)
                    // 1559: b"tmp == 2\0": typeof(_224) = & {l339} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1559: b"tmp == 2\0":   l339 = UNIQUE | NON_NULL, FIXED
                    // 1559: b"tmp == 2\0" a ... _char: typeof(_221 = move _222 as *const i8 (Misc)) = *const {l604} i8
                    // 1559: b"tmp == 2\0" a ... _char:   l604 = UNIQUE | NON_NULL, (empty)
                    // 1559: b"tmp == 2\0": typeof(_224 = const b"tmp == 2\x00") = & {l601} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1559: b"tmp == 2\0":   l601 = UNIQUE | NON_NULL, (empty)
                    // 1559: b"tmp == 2\0" a ... st u8: typeof(_222 = move _223 as *const u8 (Pointer(ArrayToPointer))) = *const {l603} u8
                    // 1559: b"tmp == 2\0" a ... st u8:   l603 = UNIQUE | NON_NULL, (empty)
                    // 1559: b"tmp == 2\0": typeof(_223 = &raw const (*_224)) = *const {l602} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1559: b"tmp == 2\0":   l602 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1560: b"lglmbt.c\0" a ... _char: typeof(_225) = *const {l341} i8
                    // 1560: b"lglmbt.c\0" a ... _char:   l341 = UNIQUE | NON_NULL, (empty)
                    // 1560: b"lglmbt.c\0" a ... st u8: typeof(_226) = *const {l343} u8
                    // 1560: b"lglmbt.c\0" a ... st u8:   l343 = UNIQUE | NON_NULL, (empty)
                    // 1560: b"lglmbt.c\0": typeof(_227) = *const {l345} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1560: b"lglmbt.c\0":   l345 = UNIQUE | NON_NULL, (empty)
                    // 1560: b"lglmbt.c\0": typeof(_228) = & {l347} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1560: b"lglmbt.c\0":   l347 = UNIQUE | NON_NULL, FIXED
                    // 1560: b"lglmbt.c\0": typeof(_227 = &raw const (*_228)) = *const {l606} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1560: b"lglmbt.c\0":   l606 = UNIQUE | NON_NULL, (empty)
                    // 1560: b"lglmbt.c\0": typeof(_228 = const b"lglmbt.c\x00") = & {l605} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1560: b"lglmbt.c\0":   l605 = UNIQUE | NON_NULL, (empty)
                    // 1560: b"lglmbt.c\0" a ... _char: typeof(_225 = move _226 as *const i8 (Misc)) = *const {l608} i8
                    // 1560: b"lglmbt.c\0" a ... _char:   l608 = UNIQUE | NON_NULL, (empty)
                    // 1560: b"lglmbt.c\0" a ... st u8: typeof(_226 = move _227 as *const u8 (Pointer(ArrayToPointer))) = *const {l607} u8
                    // 1560: b"lglmbt.c\0" a ... st u8:   l607 = UNIQUE | NON_NULL, (empty)
                    480 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 1562: (*::core::mem:: ... ptr(): typeof(_231) = *const {l351} i8
                    // 1562: (*::core::mem:: ... ptr():   l351 = UNIQUE | NON_NULL, (empty)
                    // 1562: (*::core::mem:: ... ptr(): typeof(_232) = & {l353} [i8]
                    // 1562: (*::core::mem:: ... ptr():   l353 = UNIQUE | NON_NULL, FIXED
                    // 1562: (*::core::mem:: ... ptr(): typeof(_233) = & {l355} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1562: (*::core::mem:: ... ptr():   l355 = UNIQUE | NON_NULL, (empty)
                    // 1562: ::core::mem::tr ... 0", ): typeof(_234) = & {l357} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1562: ::core::mem::tr ... 0", ):   l357 = UNIQUE | NON_NULL, FIXED
                    // 1562: (*::core::mem:: ... ptr(): typeof(_233 = &(*_234)) = & {l611} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1562: (*::core::mem:: ... ptr():   l611 = UNIQUE | NON_NULL, (empty)
                    // 1562: (*::core::mem:: ... ptr(): typeof(_232 = move _233 as &[i8] (Pointer(Unsize))) = & {l612} [i8]
                    // 1562: (*::core::mem:: ... ptr():   l612 = UNIQUE | NON_NULL, FIXED
                        b"int run(Env *, void (*)(Env *))\0",
                        // 1563: b"int run(Env * ... ))\0": typeof(_235) = & {l359} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1563: b"int run(Env * ... ))\0":   l359 = UNIQUE | NON_NULL, (empty)
                        // 1563: b"int run(Env * ... ))\0": typeof(_236) = & {l361} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1563: b"int run(Env * ... ))\0":   l361 = UNIQUE | NON_NULL, FIXED
                        // 1563: b"int run(Env * ... ))\0": typeof(_236 = const b"int run(Env *, void (*)(Env *))\x00") = & {l609} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1563: b"int run(Env * ... ))\0":   l609 = UNIQUE | NON_NULL, (empty)
                        // 1563: b"int run(Env * ... ))\0": typeof(_235 = &(*_236)) = & {l610} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1563: b"int run(Env * ... ))\0":   l610 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
        };
        close(1 as libc::c_int);
        tmp = dup(saved1);
        if tmp == 1 as libc::c_int {
        } else {
            __assert_fail(
                b"tmp == 1\0" as *const u8 as *const libc::c_char,
                // 1574: b"tmp == 1\0" a ... _char: typeof(_247) = *const {l373} i8
                // 1574: b"tmp == 1\0" a ... _char:   l373 = UNIQUE | NON_NULL, (empty)
                // 1574: b"tmp == 1\0" a ... st u8: typeof(_248) = *const {l375} u8
                // 1574: b"tmp == 1\0" a ... st u8:   l375 = UNIQUE | NON_NULL, (empty)
                // 1574: b"tmp == 1\0": typeof(_249) = *const {l377} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1574: b"tmp == 1\0":   l377 = UNIQUE | NON_NULL, (empty)
                // 1574: b"tmp == 1\0": typeof(_250) = & {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1574: b"tmp == 1\0":   l379 = UNIQUE | NON_NULL, FIXED
                // 1574: b"tmp == 1\0" a ... _char: typeof(_247 = move _248 as *const i8 (Misc)) = *const {l616} i8
                // 1574: b"tmp == 1\0" a ... _char:   l616 = UNIQUE | NON_NULL, (empty)
                // 1574: b"tmp == 1\0" a ... st u8: typeof(_248 = move _249 as *const u8 (Pointer(ArrayToPointer))) = *const {l615} u8
                // 1574: b"tmp == 1\0" a ... st u8:   l615 = UNIQUE | NON_NULL, (empty)
                // 1574: b"tmp == 1\0": typeof(_249 = &raw const (*_250)) = *const {l614} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1574: b"tmp == 1\0":   l614 = UNIQUE | NON_NULL, (empty)
                // 1574: b"tmp == 1\0": typeof(_250 = const b"tmp == 1\x00") = & {l613} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1574: b"tmp == 1\0":   l613 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1575: b"lglmbt.c\0" a ... _char: typeof(_251) = *const {l381} i8
                // 1575: b"lglmbt.c\0" a ... _char:   l381 = UNIQUE | NON_NULL, (empty)
                // 1575: b"lglmbt.c\0" a ... st u8: typeof(_252) = *const {l383} u8
                // 1575: b"lglmbt.c\0" a ... st u8:   l383 = UNIQUE | NON_NULL, (empty)
                // 1575: b"lglmbt.c\0": typeof(_253) = *const {l385} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1575: b"lglmbt.c\0":   l385 = UNIQUE | NON_NULL, (empty)
                // 1575: b"lglmbt.c\0": typeof(_254) = & {l387} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1575: b"lglmbt.c\0":   l387 = UNIQUE | NON_NULL, FIXED
                // 1575: b"lglmbt.c\0" a ... _char: typeof(_251 = move _252 as *const i8 (Misc)) = *const {l620} i8
                // 1575: b"lglmbt.c\0" a ... _char:   l620 = UNIQUE | NON_NULL, (empty)
                // 1575: b"lglmbt.c\0" a ... st u8: typeof(_252 = move _253 as *const u8 (Pointer(ArrayToPointer))) = *const {l619} u8
                // 1575: b"lglmbt.c\0" a ... st u8:   l619 = UNIQUE | NON_NULL, (empty)
                // 1575: b"lglmbt.c\0": typeof(_254 = const b"lglmbt.c\x00") = & {l617} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1575: b"lglmbt.c\0":   l617 = UNIQUE | NON_NULL, (empty)
                // 1575: b"lglmbt.c\0": typeof(_253 = &raw const (*_254)) = *const {l618} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1575: b"lglmbt.c\0":   l618 = UNIQUE | NON_NULL, (empty)
                486 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                // 1577: (*::core::mem:: ... ptr(): typeof(_257) = *const {l391} i8
                // 1577: (*::core::mem:: ... ptr():   l391 = UNIQUE | NON_NULL, (empty)
                // 1577: (*::core::mem:: ... ptr(): typeof(_258) = & {l393} [i8]
                // 1577: (*::core::mem:: ... ptr():   l393 = UNIQUE | NON_NULL, FIXED
                // 1577: (*::core::mem:: ... ptr(): typeof(_259) = & {l395} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1577: (*::core::mem:: ... ptr():   l395 = UNIQUE | NON_NULL, (empty)
                // 1577: ::core::mem::tr ... 0", ): typeof(_260) = & {l397} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1577: ::core::mem::tr ... 0", ):   l397 = UNIQUE | NON_NULL, FIXED
                // 1577: (*::core::mem:: ... ptr(): typeof(_258 = move _259 as &[i8] (Pointer(Unsize))) = & {l624} [i8]
                // 1577: (*::core::mem:: ... ptr():   l624 = UNIQUE | NON_NULL, FIXED
                // 1577: (*::core::mem:: ... ptr(): typeof(_259 = &(*_260)) = & {l623} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                // 1577: (*::core::mem:: ... ptr():   l623 = UNIQUE | NON_NULL, (empty)
                    b"int run(Env *, void (*)(Env *))\0",
                    // 1578: b"int run(Env * ... ))\0": typeof(_261) = & {l399} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1578: b"int run(Env * ... ))\0":   l399 = UNIQUE | NON_NULL, (empty)
                    // 1578: b"int run(Env * ... ))\0": typeof(_262) = & {l401} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1578: b"int run(Env * ... ))\0":   l401 = UNIQUE | NON_NULL, FIXED
                    // 1578: b"int run(Env * ... ))\0": typeof(_261 = &(*_262)) = & {l622} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1578: b"int run(Env * ... ))\0":   l622 = UNIQUE | NON_NULL, (empty)
                    // 1578: b"int run(Env * ... ))\0": typeof(_262 = const b"int run(Env *, void (*)(Env *))\x00") = & {l621} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1578: b"int run(Env * ... ))\0":   l621 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_7719: {
            if tmp == 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"tmp == 1\0" as *const u8 as *const libc::c_char,
                    // 1587: b"tmp == 1\0" a ... _char: typeof(_269) = *const {l409} i8
                    // 1587: b"tmp == 1\0" a ... _char:   l409 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"tmp == 1\0" a ... st u8: typeof(_270) = *const {l411} u8
                    // 1587: b"tmp == 1\0" a ... st u8:   l411 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"tmp == 1\0": typeof(_271) = *const {l413} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1587: b"tmp == 1\0":   l413 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"tmp == 1\0": typeof(_272) = & {l415} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1587: b"tmp == 1\0":   l415 = UNIQUE | NON_NULL, FIXED
                    // 1587: b"tmp == 1\0": typeof(_271 = &raw const (*_272)) = *const {l626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1587: b"tmp == 1\0":   l626 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"tmp == 1\0" a ... st u8: typeof(_270 = move _271 as *const u8 (Pointer(ArrayToPointer))) = *const {l627} u8
                    // 1587: b"tmp == 1\0" a ... st u8:   l627 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"tmp == 1\0": typeof(_272 = const b"tmp == 1\x00") = & {l625} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1587: b"tmp == 1\0":   l625 = UNIQUE | NON_NULL, (empty)
                    // 1587: b"tmp == 1\0" a ... _char: typeof(_269 = move _270 as *const i8 (Misc)) = *const {l628} i8
                    // 1587: b"tmp == 1\0" a ... _char:   l628 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1588: b"lglmbt.c\0" a ... _char: typeof(_273) = *const {l417} i8
                    // 1588: b"lglmbt.c\0" a ... _char:   l417 = UNIQUE | NON_NULL, (empty)
                    // 1588: b"lglmbt.c\0" a ... st u8: typeof(_274) = *const {l419} u8
                    // 1588: b"lglmbt.c\0" a ... st u8:   l419 = UNIQUE | NON_NULL, (empty)
                    // 1588: b"lglmbt.c\0": typeof(_275) = *const {l421} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1588: b"lglmbt.c\0":   l421 = UNIQUE | NON_NULL, (empty)
                    // 1588: b"lglmbt.c\0": typeof(_276) = & {l423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1588: b"lglmbt.c\0":   l423 = UNIQUE | NON_NULL, FIXED
                    // 1588: b"lglmbt.c\0" a ... _char: typeof(_273 = move _274 as *const i8 (Misc)) = *const {l632} i8
                    // 1588: b"lglmbt.c\0" a ... _char:   l632 = UNIQUE | NON_NULL, (empty)
                    // 1588: b"lglmbt.c\0" a ... st u8: typeof(_274 = move _275 as *const u8 (Pointer(ArrayToPointer))) = *const {l631} u8
                    // 1588: b"lglmbt.c\0" a ... st u8:   l631 = UNIQUE | NON_NULL, (empty)
                    // 1588: b"lglmbt.c\0": typeof(_275 = &raw const (*_276)) = *const {l630} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1588: b"lglmbt.c\0":   l630 = UNIQUE | NON_NULL, (empty)
                    // 1588: b"lglmbt.c\0": typeof(_276 = const b"lglmbt.c\x00") = & {l629} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1588: b"lglmbt.c\0":   l629 = UNIQUE | NON_NULL, (empty)
                    486 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 32], &[libc::c_char; 32]>(
                    // 1590: (*::core::mem:: ... ptr(): typeof(_279) = *const {l427} i8
                    // 1590: (*::core::mem:: ... ptr():   l427 = UNIQUE | NON_NULL, (empty)
                    // 1590: (*::core::mem:: ... ptr(): typeof(_280) = & {l429} [i8]
                    // 1590: (*::core::mem:: ... ptr():   l429 = UNIQUE | NON_NULL, FIXED
                    // 1590: (*::core::mem:: ... ptr(): typeof(_281) = & {l431} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1590: (*::core::mem:: ... ptr():   l431 = UNIQUE | NON_NULL, (empty)
                    // 1590: ::core::mem::tr ... 0", ): typeof(_282) = & {l433} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1590: ::core::mem::tr ... 0", ):   l433 = UNIQUE | NON_NULL, FIXED
                    // 1590: (*::core::mem:: ... ptr(): typeof(_281 = &(*_282)) = & {l635} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                    // 1590: (*::core::mem:: ... ptr():   l635 = UNIQUE | NON_NULL, (empty)
                    // 1590: (*::core::mem:: ... ptr(): typeof(_280 = move _281 as &[i8] (Pointer(Unsize))) = & {l636} [i8]
                    // 1590: (*::core::mem:: ... ptr():   l636 = UNIQUE | NON_NULL, FIXED
                        b"int run(Env *, void (*)(Env *))\0",
                        // 1591: b"int run(Env * ... ))\0": typeof(_283) = & {l435} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1591: b"int run(Env * ... ))\0":   l435 = UNIQUE | NON_NULL, (empty)
                        // 1591: b"int run(Env * ... ))\0": typeof(_284) = & {l437} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1591: b"int run(Env * ... ))\0":   l437 = UNIQUE | NON_NULL, FIXED
                        // 1591: b"int run(Env * ... ))\0": typeof(_284 = const b"int run(Env *, void (*)(Env *))\x00") = & {l633} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1591: b"int run(Env * ... ))\0":   l633 = UNIQUE | NON_NULL, (empty)
                        // 1591: b"int run(Env * ... ))\0": typeof(_283 = &(*_284)) = & {l634} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                        // 1591: b"int run(Env * ... ))\0":   l634 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
        };
        exit(0 as libc::c_int);
    }
    if status & 0x7f as libc::c_int == 0 as libc::c_int {
        res = (status & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        if res == 42 as libc::c_int {
            res = 0 as libc::c_int;
            (*env_0).violations += 1;
            (*env_0).violations;
        } else if (*env_0).print != 0 {
            printf(b"exit %d \0" as *const u8 as *const libc::c_char, res);
            // 1606: b"exit %d \0" a ... _char: typeof(_307) = *const {l461} i8
            // 1606: b"exit %d \0" a ... _char:   l461 = UNIQUE | NON_NULL, (empty)
            // 1606: b"exit %d \0" a ... st u8: typeof(_308) = *const {l463} u8
            // 1606: b"exit %d \0" a ... st u8:   l463 = UNIQUE | NON_NULL, (empty)
            // 1606: b"exit %d \0": typeof(_309) = *const {l465} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1606: b"exit %d \0":   l465 = UNIQUE | NON_NULL, (empty)
            // 1606: b"exit %d \0": typeof(_310) = & {l467} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1606: b"exit %d \0":   l467 = UNIQUE | NON_NULL, FIXED
            // 1606: b"exit %d \0" a ... _char: typeof(_307 = move _308 as *const i8 (Misc)) = *const {l640} i8
            // 1606: b"exit %d \0" a ... _char:   l640 = UNIQUE | NON_NULL, (empty)
            // 1606: b"exit %d \0": typeof(_309 = &raw const (*_310)) = *const {l638} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1606: b"exit %d \0":   l638 = UNIQUE | NON_NULL, (empty)
            // 1606: b"exit %d \0" a ... st u8: typeof(_308 = move _309 as *const u8 (Pointer(ArrayToPointer))) = *const {l639} u8
            // 1606: b"exit %d \0" a ... st u8:   l639 = UNIQUE | NON_NULL, (empty)
            // 1606: b"exit %d \0": typeof(_310 = const b"exit %d \x00") = & {l637} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1606: b"exit %d \0":   l637 = UNIQUE | NON_NULL, (empty)
        }
    } else if ((status & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar as libc::c_int
        >> 1 as libc::c_int
        > 0 as libc::c_int
    {
        if (*env_0).print != 0 {
            printf(b"signal\0" as *const u8 as *const libc::c_char);
            // 1613: b"signal\0" as  ... _char: typeof(_329) = *const {l487} i8
            // 1613: b"signal\0" as  ... _char:   l487 = UNIQUE | NON_NULL, (empty)
            // 1613: b"signal\0" as  ... st u8: typeof(_330) = *const {l489} u8
            // 1613: b"signal\0" as  ... st u8:   l489 = UNIQUE | NON_NULL, (empty)
            // 1613: b"signal\0": typeof(_331) = *const {l491} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1613: b"signal\0":   l491 = UNIQUE | NON_NULL, (empty)
            // 1613: b"signal\0": typeof(_332) = & {l493} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1613: b"signal\0":   l493 = UNIQUE | NON_NULL, FIXED
            // 1613: b"signal\0" as  ... st u8: typeof(_330 = move _331 as *const u8 (Pointer(ArrayToPointer))) = *const {l643} u8
            // 1613: b"signal\0" as  ... st u8:   l643 = UNIQUE | NON_NULL, (empty)
            // 1613: b"signal\0" as  ... _char: typeof(_329 = move _330 as *const i8 (Misc)) = *const {l644} i8
            // 1613: b"signal\0" as  ... _char:   l644 = UNIQUE | NON_NULL, (empty)
            // 1613: b"signal\0": typeof(_331 = &raw const (*_332)) = *const {l642} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1613: b"signal\0":   l642 = UNIQUE | NON_NULL, (empty)
            // 1613: b"signal\0": typeof(_332 = const b"signal\x00") = & {l641} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
            // 1613: b"signal\0":   l641 = UNIQUE | NON_NULL, (empty)
        }
        res = 1 as libc::c_int;
    } else {
        if (*env_0).print != 0 {
            printf(b"unknown\0" as *const u8 as *const libc::c_char);
            // 1618: b"unknown\0" as ... _char: typeof(_338) = *const {l500} i8
            // 1618: b"unknown\0" as ... _char:   l500 = UNIQUE | NON_NULL, (empty)
            // 1618: b"unknown\0" as ... st u8: typeof(_339) = *const {l502} u8
            // 1618: b"unknown\0" as ... st u8:   l502 = UNIQUE | NON_NULL, (empty)
            // 1618: b"unknown\0": typeof(_340) = *const {l504} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1618: b"unknown\0":   l504 = UNIQUE | NON_NULL, (empty)
            // 1618: b"unknown\0": typeof(_341) = & {l506} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1618: b"unknown\0":   l506 = UNIQUE | NON_NULL, FIXED
            // 1618: b"unknown\0" as ... st u8: typeof(_339 = move _340 as *const u8 (Pointer(ArrayToPointer))) = *const {l647} u8
            // 1618: b"unknown\0" as ... st u8:   l647 = UNIQUE | NON_NULL, (empty)
            // 1618: b"unknown\0" as ... _char: typeof(_338 = move _339 as *const i8 (Misc)) = *const {l648} i8
            // 1618: b"unknown\0" as ... _char:   l648 = UNIQUE | NON_NULL, (empty)
            // 1618: b"unknown\0": typeof(_340 = &raw const (*_341)) = *const {l646} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1618: b"unknown\0":   l646 = UNIQUE | NON_NULL, (empty)
            // 1618: b"unknown\0": typeof(_341 = const b"unknown\x00") = & {l645} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
            // 1618: b"unknown\0":   l645 = UNIQUE | NON_NULL, (empty)
        }
        res = 1 as libc::c_int;
    }
    return res;
}
unsafe extern "C" fn printrace(mut env_0: *mut Env) {
// 1624: mut env_0: typeof(_1) = *mut {g35} DefId(0:327 ~ lglmbt[b165]::Env)
// 1624: mut env_0:   g35 = UNIQUE | NON_NULL, FIXED
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    // 1625: mut name: typeof(_2) = *mut {l2} i8
    // 1625: mut name:   l2 = UNIQUE | NON_NULL, (empty)
    // 1625: 0 as *mut libc: ... _char: typeof(_2 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l333} i8
    // 1625: 0 as *mut libc: ... _char:   l333 = UNIQUE | NON_NULL, (empty)
    let mut data: Data = Data {
        lgl: 0 as *mut LGL,
        // 1627: 0 as *mut LGL: typeof(_4) = *mut {l5} LGL
        // 1627: 0 as *mut LGL:   l5 = UNIQUE | NON_NULL, (empty)
        // 1627: 0 as *mut LGL: typeof(_4 = const 0_usize as *mut LGL (PointerFromExposedAddress)) = *mut {l334} LGL
        // 1627: 0 as *mut LGL:   l334 = UNIQUE | NON_NULL, (empty)
        trace: 0 as *mut FILE,
        // 1628: 0 as *mut FILE: typeof(_5) = *mut {l7} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1628: 0 as *mut FILE:   l7 = UNIQUE | NON_NULL, (empty)
        // 1628: 0 as *mut FILE: typeof(_5 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l335} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1628: 0 as *mut FILE:   l335 = UNIQUE | NON_NULL, (empty)
        available: 0 as *mut libc::c_int,
        // 1629: 0 as *mut libc: ... c_int: typeof(_6) = *mut {l9} i32
        // 1629: 0 as *mut libc: ... c_int:   l9 = UNIQUE | NON_NULL, (empty)
        // 1629: 0 as *mut libc: ... c_int: typeof(_6 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l336} i32
        // 1629: 0 as *mut libc: ... c_int:   l336 = UNIQUE | NON_NULL, (empty)
        navailable: 0,
        frozen: 0 as *mut libc::c_int,
        // 1631: 0 as *mut libc: ... c_int: typeof(_7) = *mut {l11} i32
        // 1631: 0 as *mut libc: ... c_int:   l11 = UNIQUE | NON_NULL, (empty)
        // 1631: 0 as *mut libc: ... c_int: typeof(_7 = const 0_usize as *mut i32 (PointerFromExposedAddress)) = *mut {l337} i32
        // 1631: 0 as *mut libc: ... c_int:   l337 = UNIQUE | NON_NULL, (empty)
        nfrozen: 0,
        m: 0,
        n: 0,
        c: 0,
        print: 0,
        noptsfuzz: 0,
    };
    let mut e: *mut Event = 0 as *mut Event;
    // 1639: mut e: typeof(_8) = *mut {l13} DefId(0:314 ~ lglmbt[b165]::Event)
    // 1639: mut e:   l13 = UNIQUE | NON_NULL, (empty)
    // 1639: 0 as *mut Event: typeof(_8 = const 0_usize as *mut Event (PointerFromExposedAddress)) = *mut {l338} DefId(0:314 ~ lglmbt[b165]::Event)
    // 1639: 0 as *mut Event:   l338 = UNIQUE | NON_NULL, (empty)
    let mut i: libc::c_int = 0;
    memset(
    // 1641: memset( &mut da ... ng, ): typeof(_10) = *mut {l16} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1641: memset( &mut da ... ng, ):   l16 = UNIQUE | NON_NULL, (empty)
        &mut data as *mut Data as *mut libc::c_void,
        // 1642: &mut data as *m ... _void: typeof(_11) = *mut {l18} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1642: &mut data as *m ... _void:   l18 = UNIQUE | NON_NULL, (empty)
        // 1642: &mut data as *m ...  Data: typeof(_12) = *mut {l20} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1642: &mut data as *m ...  Data:   l20 = UNIQUE | NON_NULL, (empty)
        // 1642: &mut data: typeof(_13) = &mut {l22} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1642: &mut data:   l22 = UNIQUE | NON_NULL, (empty)
        // 1642: &mut data as *m ... _void: typeof(_11 = move _12 as *mut libc::c_void (Misc)) = *mut {l341} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1642: &mut data as *m ... _void:   l341 = UNIQUE | NON_NULL, (empty)
        // 1642: &mut data: typeof(_12 = &raw mut (*_13)) = *mut {l340} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1642: &mut data:   l340 = UNIQUE | NON_NULL, (empty)
        // 1642: &mut data: typeof(_13 = &mut _3) = &mut {l339} DefId(0:299 ~ lglmbt[b165]::Data)
        // 1642: &mut data:   l339 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
        ::core::mem::size_of::<Data>() as libc::c_ulong,
    );
    data.print = 0 as libc::c_int;
    if !((*env_0).events).is_null() {
    // 1647: ((*env_0).events): typeof(_21) = *mut {l31} DefId(0:314 ~ lglmbt[b165]::Event)
    // 1647: ((*env_0).events):   l31 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"env->events\0" as *const u8 as *const libc::c_char,
            // 1650: b"env->events\0 ... _char: typeof(_24) = *const {l35} i8
            // 1650: b"env->events\0 ... _char:   l35 = UNIQUE | NON_NULL, (empty)
            // 1650: b"env->events\0 ... st u8: typeof(_25) = *const {l37} u8
            // 1650: b"env->events\0 ... st u8:   l37 = UNIQUE | NON_NULL, (empty)
            // 1650: b"env->events\0": typeof(_26) = *const {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1650: b"env->events\0":   l39 = UNIQUE | NON_NULL, (empty)
            // 1650: b"env->events\0": typeof(_27) = & {l41} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1650: b"env->events\0":   l41 = UNIQUE | NON_NULL, FIXED
            // 1650: b"env->events\0 ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l345} i8
            // 1650: b"env->events\0 ... _char:   l345 = UNIQUE | NON_NULL, (empty)
            // 1650: b"env->events\0": typeof(_26 = &raw const (*_27)) = *const {l343} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1650: b"env->events\0":   l343 = UNIQUE | NON_NULL, (empty)
            // 1650: b"env->events\0 ... st u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l344} u8
            // 1650: b"env->events\0 ... st u8:   l344 = UNIQUE | NON_NULL, (empty)
            // 1650: b"env->events\0": typeof(_27 = const b"env->events\x00") = & {l342} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1650: b"env->events\0":   l342 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1651: b"lglmbt.c\0" a ... _char: typeof(_28) = *const {l43} i8
            // 1651: b"lglmbt.c\0" a ... _char:   l43 = UNIQUE | NON_NULL, (empty)
            // 1651: b"lglmbt.c\0" a ... st u8: typeof(_29) = *const {l45} u8
            // 1651: b"lglmbt.c\0" a ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
            // 1651: b"lglmbt.c\0": typeof(_30) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1651: b"lglmbt.c\0":   l47 = UNIQUE | NON_NULL, (empty)
            // 1651: b"lglmbt.c\0": typeof(_31) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1651: b"lglmbt.c\0":   l49 = UNIQUE | NON_NULL, FIXED
            // 1651: b"lglmbt.c\0": typeof(_31 = const b"lglmbt.c\x00") = & {l346} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1651: b"lglmbt.c\0":   l346 = UNIQUE | NON_NULL, (empty)
            // 1651: b"lglmbt.c\0": typeof(_30 = &raw const (*_31)) = *const {l347} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1651: b"lglmbt.c\0":   l347 = UNIQUE | NON_NULL, (empty)
            // 1651: b"lglmbt.c\0" a ... st u8: typeof(_29 = move _30 as *const u8 (Pointer(ArrayToPointer))) = *const {l348} u8
            // 1651: b"lglmbt.c\0" a ... st u8:   l348 = UNIQUE | NON_NULL, (empty)
            // 1651: b"lglmbt.c\0" a ... _char: typeof(_28 = move _29 as *const i8 (Misc)) = *const {l349} i8
            // 1651: b"lglmbt.c\0" a ... _char:   l349 = UNIQUE | NON_NULL, (empty)
            510 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"void printrace(Env *)\0"))
            // 1653: (*::core::mem:: ... ptr(): typeof(_34) = *const {l53} i8
            // 1653: (*::core::mem:: ... ptr():   l53 = UNIQUE | NON_NULL, (empty)
            // 1653: (*::core::mem:: ... ptr(): typeof(_35) = & {l55} [i8]
            // 1653: (*::core::mem:: ... ptr():   l55 = UNIQUE | NON_NULL, FIXED
            // 1653: (*::core::mem:: ... ptr(): typeof(_36) = & {l57} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: (*::core::mem:: ... ptr():   l57 = UNIQUE | NON_NULL, (empty)
            // 1653: ::core::mem::tr ... )\0"): typeof(_37) = & {l59} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: ::core::mem::tr ... )\0"):   l59 = UNIQUE | NON_NULL, FIXED
            // 1653: b"void printrac ... *)\0": typeof(_38) = & {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: b"void printrac ... *)\0":   l61 = UNIQUE | NON_NULL, (empty)
            // 1653: b"void printrac ... *)\0": typeof(_39) = & {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: b"void printrac ... *)\0":   l63 = UNIQUE | NON_NULL, FIXED
            // 1653: b"void printrac ... *)\0": typeof(_38 = &(*_39)) = & {l351} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: b"void printrac ... *)\0":   l351 = UNIQUE | NON_NULL, (empty)
            // 1653: (*::core::mem:: ... ptr(): typeof(_35 = move _36 as &[i8] (Pointer(Unsize))) = & {l353} [i8]
            // 1653: (*::core::mem:: ... ptr():   l353 = UNIQUE | NON_NULL, FIXED
            // 1653: b"void printrac ... *)\0": typeof(_39 = const b"void printrace(Env *)\x00") = & {l350} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: b"void printrac ... *)\0":   l350 = UNIQUE | NON_NULL, (empty)
            // 1653: (*::core::mem:: ... ptr(): typeof(_36 = &(*_37)) = & {l352} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1653: (*::core::mem:: ... ptr():   l352 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_8228: {
        if !((*env_0).events).is_null() {
        // 1658: ((*env_0).events): typeof(_43) = *mut {l68} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1658: ((*env_0).events):   l68 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"env->events\0" as *const u8 as *const libc::c_char,
                // 1661: b"env->events\0 ... _char: typeof(_46) = *const {l72} i8
                // 1661: b"env->events\0 ... _char:   l72 = UNIQUE | NON_NULL, (empty)
                // 1661: b"env->events\0 ... st u8: typeof(_47) = *const {l74} u8
                // 1661: b"env->events\0 ... st u8:   l74 = UNIQUE | NON_NULL, (empty)
                // 1661: b"env->events\0": typeof(_48) = *const {l76} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1661: b"env->events\0":   l76 = UNIQUE | NON_NULL, (empty)
                // 1661: b"env->events\0": typeof(_49) = & {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1661: b"env->events\0":   l78 = UNIQUE | NON_NULL, FIXED
                // 1661: b"env->events\0 ... st u8: typeof(_47 = move _48 as *const u8 (Pointer(ArrayToPointer))) = *const {l356} u8
                // 1661: b"env->events\0 ... st u8:   l356 = UNIQUE | NON_NULL, (empty)
                // 1661: b"env->events\0 ... _char: typeof(_46 = move _47 as *const i8 (Misc)) = *const {l357} i8
                // 1661: b"env->events\0 ... _char:   l357 = UNIQUE | NON_NULL, (empty)
                // 1661: b"env->events\0": typeof(_49 = const b"env->events\x00") = & {l354} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1661: b"env->events\0":   l354 = UNIQUE | NON_NULL, (empty)
                // 1661: b"env->events\0": typeof(_48 = &raw const (*_49)) = *const {l355} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1661: b"env->events\0":   l355 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1662: b"lglmbt.c\0" a ... _char: typeof(_50) = *const {l80} i8
                // 1662: b"lglmbt.c\0" a ... _char:   l80 = UNIQUE | NON_NULL, (empty)
                // 1662: b"lglmbt.c\0" a ... st u8: typeof(_51) = *const {l82} u8
                // 1662: b"lglmbt.c\0" a ... st u8:   l82 = UNIQUE | NON_NULL, (empty)
                // 1662: b"lglmbt.c\0": typeof(_52) = *const {l84} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1662: b"lglmbt.c\0":   l84 = UNIQUE | NON_NULL, (empty)
                // 1662: b"lglmbt.c\0": typeof(_53) = & {l86} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1662: b"lglmbt.c\0":   l86 = UNIQUE | NON_NULL, FIXED
                // 1662: b"lglmbt.c\0" a ... _char: typeof(_50 = move _51 as *const i8 (Misc)) = *const {l361} i8
                // 1662: b"lglmbt.c\0" a ... _char:   l361 = UNIQUE | NON_NULL, (empty)
                // 1662: b"lglmbt.c\0": typeof(_52 = &raw const (*_53)) = *const {l359} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1662: b"lglmbt.c\0":   l359 = UNIQUE | NON_NULL, (empty)
                // 1662: b"lglmbt.c\0": typeof(_53 = const b"lglmbt.c\x00") = & {l358} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1662: b"lglmbt.c\0":   l358 = UNIQUE | NON_NULL, (empty)
                // 1662: b"lglmbt.c\0" a ... st u8: typeof(_51 = move _52 as *const u8 (Pointer(ArrayToPointer))) = *const {l360} u8
                // 1662: b"lglmbt.c\0" a ... st u8:   l360 = UNIQUE | NON_NULL, (empty)
                510 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                // 1664: (*::core::mem:: ... ptr(): typeof(_56) = *const {l90} i8
                // 1664: (*::core::mem:: ... ptr():   l90 = UNIQUE | NON_NULL, (empty)
                // 1664: (*::core::mem:: ... ptr(): typeof(_57) = & {l92} [i8]
                // 1664: (*::core::mem:: ... ptr():   l92 = UNIQUE | NON_NULL, FIXED
                // 1664: (*::core::mem:: ... ptr(): typeof(_58) = & {l94} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1664: (*::core::mem:: ... ptr():   l94 = UNIQUE | NON_NULL, (empty)
                // 1664: ::core::mem::tr ... 0", ): typeof(_59) = & {l96} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1664: ::core::mem::tr ... 0", ):   l96 = UNIQUE | NON_NULL, FIXED
                // 1664: (*::core::mem:: ... ptr(): typeof(_58 = &(*_59)) = & {l364} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1664: (*::core::mem:: ... ptr():   l364 = UNIQUE | NON_NULL, (empty)
                // 1664: (*::core::mem:: ... ptr(): typeof(_57 = move _58 as &[i8] (Pointer(Unsize))) = & {l365} [i8]
                // 1664: (*::core::mem:: ... ptr():   l365 = UNIQUE | NON_NULL, FIXED
                    b"void printrace(Env *)\0",
                    // 1665: b"void printrac ... *)\0": typeof(_60) = & {l98} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1665: b"void printrac ... *)\0":   l98 = UNIQUE | NON_NULL, (empty)
                    // 1665: b"void printrac ... *)\0": typeof(_61) = & {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1665: b"void printrac ... *)\0":   l100 = UNIQUE | NON_NULL, FIXED
                    // 1665: b"void printrac ... *)\0": typeof(_60 = &(*_61)) = & {l363} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1665: b"void printrac ... *)\0":   l363 = UNIQUE | NON_NULL, (empty)
                    // 1665: b"void printrac ... *)\0": typeof(_61 = const b"void printrace(Env *)\x00") = & {l362} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1665: b"void printrac ... *)\0":   l362 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    if !((*env_0).prefix).is_null() {
    // 1671: ((*env_0).prefix): typeof(_65) = *const {l105} i8
    // 1671: ((*env_0).prefix):   l105 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"env->prefix\0" as *const u8 as *const libc::c_char,
            // 1674: b"env->prefix\0 ... _char: typeof(_68) = *const {l109} i8
            // 1674: b"env->prefix\0 ... _char:   l109 = UNIQUE | NON_NULL, (empty)
            // 1674: b"env->prefix\0 ... st u8: typeof(_69) = *const {l111} u8
            // 1674: b"env->prefix\0 ... st u8:   l111 = UNIQUE | NON_NULL, (empty)
            // 1674: b"env->prefix\0": typeof(_70) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1674: b"env->prefix\0":   l113 = UNIQUE | NON_NULL, (empty)
            // 1674: b"env->prefix\0": typeof(_71) = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1674: b"env->prefix\0":   l115 = UNIQUE | NON_NULL, FIXED
            // 1674: b"env->prefix\0 ... st u8: typeof(_69 = move _70 as *const u8 (Pointer(ArrayToPointer))) = *const {l368} u8
            // 1674: b"env->prefix\0 ... st u8:   l368 = UNIQUE | NON_NULL, (empty)
            // 1674: b"env->prefix\0 ... _char: typeof(_68 = move _69 as *const i8 (Misc)) = *const {l369} i8
            // 1674: b"env->prefix\0 ... _char:   l369 = UNIQUE | NON_NULL, (empty)
            // 1674: b"env->prefix\0": typeof(_71 = const b"env->prefix\x00") = & {l366} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1674: b"env->prefix\0":   l366 = UNIQUE | NON_NULL, (empty)
            // 1674: b"env->prefix\0": typeof(_70 = &raw const (*_71)) = *const {l367} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
            // 1674: b"env->prefix\0":   l367 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1675: b"lglmbt.c\0" a ... _char: typeof(_72) = *const {l117} i8
            // 1675: b"lglmbt.c\0" a ... _char:   l117 = UNIQUE | NON_NULL, (empty)
            // 1675: b"lglmbt.c\0" a ... st u8: typeof(_73) = *const {l119} u8
            // 1675: b"lglmbt.c\0" a ... st u8:   l119 = UNIQUE | NON_NULL, (empty)
            // 1675: b"lglmbt.c\0": typeof(_74) = *const {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1675: b"lglmbt.c\0":   l121 = UNIQUE | NON_NULL, (empty)
            // 1675: b"lglmbt.c\0": typeof(_75) = & {l123} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1675: b"lglmbt.c\0":   l123 = UNIQUE | NON_NULL, FIXED
            // 1675: b"lglmbt.c\0" a ... st u8: typeof(_73 = move _74 as *const u8 (Pointer(ArrayToPointer))) = *const {l372} u8
            // 1675: b"lglmbt.c\0" a ... st u8:   l372 = UNIQUE | NON_NULL, (empty)
            // 1675: b"lglmbt.c\0": typeof(_75 = const b"lglmbt.c\x00") = & {l370} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1675: b"lglmbt.c\0":   l370 = UNIQUE | NON_NULL, (empty)
            // 1675: b"lglmbt.c\0" a ... _char: typeof(_72 = move _73 as *const i8 (Misc)) = *const {l373} i8
            // 1675: b"lglmbt.c\0" a ... _char:   l373 = UNIQUE | NON_NULL, (empty)
            // 1675: b"lglmbt.c\0": typeof(_74 = &raw const (*_75)) = *const {l371} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1675: b"lglmbt.c\0":   l371 = UNIQUE | NON_NULL, (empty)
            511 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"void printrace(Env *)\0"))
            // 1677: (*::core::mem:: ... ptr(): typeof(_78) = *const {l127} i8
            // 1677: (*::core::mem:: ... ptr():   l127 = UNIQUE | NON_NULL, (empty)
            // 1677: (*::core::mem:: ... ptr(): typeof(_79) = & {l129} [i8]
            // 1677: (*::core::mem:: ... ptr():   l129 = UNIQUE | NON_NULL, FIXED
            // 1677: (*::core::mem:: ... ptr(): typeof(_80) = & {l131} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: (*::core::mem:: ... ptr():   l131 = UNIQUE | NON_NULL, (empty)
            // 1677: ::core::mem::tr ... )\0"): typeof(_81) = & {l133} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: ::core::mem::tr ... )\0"):   l133 = UNIQUE | NON_NULL, FIXED
            // 1677: b"void printrac ... *)\0": typeof(_82) = & {l135} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: b"void printrac ... *)\0":   l135 = UNIQUE | NON_NULL, (empty)
            // 1677: b"void printrac ... *)\0": typeof(_83) = & {l137} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: b"void printrac ... *)\0":   l137 = UNIQUE | NON_NULL, FIXED
            // 1677: (*::core::mem:: ... ptr(): typeof(_80 = &(*_81)) = & {l376} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: (*::core::mem:: ... ptr():   l376 = UNIQUE | NON_NULL, (empty)
            // 1677: (*::core::mem:: ... ptr(): typeof(_79 = move _80 as &[i8] (Pointer(Unsize))) = & {l377} [i8]
            // 1677: (*::core::mem:: ... ptr():   l377 = UNIQUE | NON_NULL, FIXED
            // 1677: b"void printrac ... *)\0": typeof(_83 = const b"void printrace(Env *)\x00") = & {l374} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: b"void printrac ... *)\0":   l374 = UNIQUE | NON_NULL, (empty)
            // 1677: b"void printrac ... *)\0": typeof(_82 = &(*_83)) = & {l375} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1677: b"void printrac ... *)\0":   l375 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_8192: {
        if !((*env_0).prefix).is_null() {
        // 1682: ((*env_0).prefix): typeof(_87) = *const {l142} i8
        // 1682: ((*env_0).prefix):   l142 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"env->prefix\0" as *const u8 as *const libc::c_char,
                // 1685: b"env->prefix\0 ... _char: typeof(_90) = *const {l146} i8
                // 1685: b"env->prefix\0 ... _char:   l146 = UNIQUE | NON_NULL, (empty)
                // 1685: b"env->prefix\0 ... st u8: typeof(_91) = *const {l148} u8
                // 1685: b"env->prefix\0 ... st u8:   l148 = UNIQUE | NON_NULL, (empty)
                // 1685: b"env->prefix\0": typeof(_92) = *const {l150} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1685: b"env->prefix\0":   l150 = UNIQUE | NON_NULL, (empty)
                // 1685: b"env->prefix\0": typeof(_93) = & {l152} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1685: b"env->prefix\0":   l152 = UNIQUE | NON_NULL, FIXED
                // 1685: b"env->prefix\0 ... st u8: typeof(_91 = move _92 as *const u8 (Pointer(ArrayToPointer))) = *const {l380} u8
                // 1685: b"env->prefix\0 ... st u8:   l380 = UNIQUE | NON_NULL, (empty)
                // 1685: b"env->prefix\0 ... _char: typeof(_90 = move _91 as *const i8 (Misc)) = *const {l381} i8
                // 1685: b"env->prefix\0 ... _char:   l381 = UNIQUE | NON_NULL, (empty)
                // 1685: b"env->prefix\0": typeof(_92 = &raw const (*_93)) = *const {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1685: b"env->prefix\0":   l379 = UNIQUE | NON_NULL, (empty)
                // 1685: b"env->prefix\0": typeof(_93 = const b"env->prefix\x00") = & {l378} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
                // 1685: b"env->prefix\0":   l378 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1686: b"lglmbt.c\0" a ... _char: typeof(_94) = *const {l154} i8
                // 1686: b"lglmbt.c\0" a ... _char:   l154 = UNIQUE | NON_NULL, (empty)
                // 1686: b"lglmbt.c\0" a ... st u8: typeof(_95) = *const {l156} u8
                // 1686: b"lglmbt.c\0" a ... st u8:   l156 = UNIQUE | NON_NULL, (empty)
                // 1686: b"lglmbt.c\0": typeof(_96) = *const {l158} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1686: b"lglmbt.c\0":   l158 = UNIQUE | NON_NULL, (empty)
                // 1686: b"lglmbt.c\0": typeof(_97) = & {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1686: b"lglmbt.c\0":   l160 = UNIQUE | NON_NULL, FIXED
                // 1686: b"lglmbt.c\0": typeof(_96 = &raw const (*_97)) = *const {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1686: b"lglmbt.c\0":   l383 = UNIQUE | NON_NULL, (empty)
                // 1686: b"lglmbt.c\0" a ... _char: typeof(_94 = move _95 as *const i8 (Misc)) = *const {l385} i8
                // 1686: b"lglmbt.c\0" a ... _char:   l385 = UNIQUE | NON_NULL, (empty)
                // 1686: b"lglmbt.c\0": typeof(_97 = const b"lglmbt.c\x00") = & {l382} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1686: b"lglmbt.c\0":   l382 = UNIQUE | NON_NULL, (empty)
                // 1686: b"lglmbt.c\0" a ... st u8: typeof(_95 = move _96 as *const u8 (Pointer(ArrayToPointer))) = *const {l384} u8
                // 1686: b"lglmbt.c\0" a ... st u8:   l384 = UNIQUE | NON_NULL, (empty)
                511 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                // 1688: (*::core::mem:: ... ptr(): typeof(_100) = *const {l164} i8
                // 1688: (*::core::mem:: ... ptr():   l164 = UNIQUE | NON_NULL, (empty)
                // 1688: (*::core::mem:: ... ptr(): typeof(_101) = & {l166} [i8]
                // 1688: (*::core::mem:: ... ptr():   l166 = UNIQUE | NON_NULL, FIXED
                // 1688: (*::core::mem:: ... ptr(): typeof(_102) = & {l168} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1688: (*::core::mem:: ... ptr():   l168 = UNIQUE | NON_NULL, (empty)
                // 1688: ::core::mem::tr ... 0", ): typeof(_103) = & {l170} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1688: ::core::mem::tr ... 0", ):   l170 = UNIQUE | NON_NULL, FIXED
                // 1688: (*::core::mem:: ... ptr(): typeof(_102 = &(*_103)) = & {l388} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1688: (*::core::mem:: ... ptr():   l388 = UNIQUE | NON_NULL, (empty)
                // 1688: (*::core::mem:: ... ptr(): typeof(_101 = move _102 as &[i8] (Pointer(Unsize))) = & {l389} [i8]
                // 1688: (*::core::mem:: ... ptr():   l389 = UNIQUE | NON_NULL, FIXED
                    b"void printrace(Env *)\0",
                    // 1689: b"void printrac ... *)\0": typeof(_104) = & {l172} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1689: b"void printrac ... *)\0":   l172 = UNIQUE | NON_NULL, (empty)
                    // 1689: b"void printrac ... *)\0": typeof(_105) = & {l174} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1689: b"void printrac ... *)\0":   l174 = UNIQUE | NON_NULL, FIXED
                    // 1689: b"void printrac ... *)\0": typeof(_105 = const b"void printrace(Env *)\x00") = & {l386} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1689: b"void printrac ... *)\0":   l386 = UNIQUE | NON_NULL, (empty)
                    // 1689: b"void printrac ... *)\0": typeof(_104 = &(*_105)) = & {l387} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1689: b"void printrac ... *)\0":   l387 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    name = malloc((strlen((*env_0).prefix)).wrapping_add(80 as libc::c_int as libc::c_ulong))
    // 1695: malloc((strlen( ... ong)): typeof(_106) = *mut {l176} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1695: malloc((strlen( ... ong)):   l176 = UNIQUE | NON_NULL, (empty)
    // 1695: (*env_0).prefix: typeof(_109) = *const {l180} i8
    // 1695: (*env_0).prefix:   l180 = UNIQUE | NON_NULL, (empty)
    // 1695: name = malloc(( ... _char: typeof(_2 = move _106 as *mut i8 (Misc)) = *mut {l390} i8
    // 1695: name = malloc(( ... _char:   l390 = UNIQUE | NON_NULL, (empty)
        as *mut libc::c_char;
    sprintf(
        name,
        // 1698: name: typeof(_113) = *mut {l185} i8
        // 1698: name:   l185 = UNIQUE | NON_NULL, (empty)
        b"%s-%u.trace\0" as *const u8 as *const libc::c_char,
        // 1699: b"%s-%u.trace\0 ... _char: typeof(_114) = *const {l187} i8
        // 1699: b"%s-%u.trace\0 ... _char:   l187 = UNIQUE | NON_NULL, (empty)
        // 1699: b"%s-%u.trace\0 ... st u8: typeof(_115) = *const {l189} u8
        // 1699: b"%s-%u.trace\0 ... st u8:   l189 = UNIQUE | NON_NULL, (empty)
        // 1699: b"%s-%u.trace\0": typeof(_116) = *const {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1699: b"%s-%u.trace\0":   l191 = UNIQUE | NON_NULL, (empty)
        // 1699: b"%s-%u.trace\0": typeof(_117) = & {l193} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1699: b"%s-%u.trace\0":   l193 = UNIQUE | NON_NULL, FIXED
        // 1699: b"%s-%u.trace\0 ... st u8: typeof(_115 = move _116 as *const u8 (Pointer(ArrayToPointer))) = *const {l393} u8
        // 1699: b"%s-%u.trace\0 ... st u8:   l393 = UNIQUE | NON_NULL, (empty)
        // 1699: b"%s-%u.trace\0": typeof(_117 = const b"%s-%u.trace\x00") = & {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1699: b"%s-%u.trace\0":   l391 = UNIQUE | NON_NULL, (empty)
        // 1699: b"%s-%u.trace\0": typeof(_116 = &raw const (*_117)) = *const {l392} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1699: b"%s-%u.trace\0":   l392 = UNIQUE | NON_NULL, (empty)
        // 1699: b"%s-%u.trace\0 ... _char: typeof(_114 = move _115 as *const i8 (Misc)) = *const {l394} i8
        // 1699: b"%s-%u.trace\0 ... _char:   l394 = UNIQUE | NON_NULL, (empty)
        (*env_0).prefix,
        // 1700: (*env_0).prefix: typeof(_118) = *const {l195} i8
        // 1700: (*env_0).prefix:   l195 = UNIQUE | NON_NULL, (empty)
        (*env_0).seed,
    );
    data.trace = fopen(name, b"w\0" as *const u8 as *const libc::c_char);
    // 1703: fopen(name, b"w ... char): typeof(_120) = *mut {l198} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1703: fopen(name, b"w ... char):   l198 = UNIQUE | NON_NULL, (empty)
    // 1703: name: typeof(_121) = *const {l200} i8
    // 1703: name:   l200 = UNIQUE | NON_NULL, (empty)
    // 1703: name: typeof(_122) = *mut {l202} i8
    // 1703: name:   l202 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0" as *cons ... _char: typeof(_123) = *const {l204} i8
    // 1703: b"w\0" as *cons ... _char:   l204 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0" as *const u8: typeof(_124) = *const {l206} u8
    // 1703: b"w\0" as *const u8:   l206 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0": typeof(_125) = *const {l208} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1703: b"w\0":   l208 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0": typeof(_126) = & {l210} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1703: b"w\0":   l210 = UNIQUE | NON_NULL, FIXED
    // 1703: b"w\0" as *const u8: typeof(_124 = move _125 as *const u8 (Pointer(ArrayToPointer))) = *const {l398} u8
    // 1703: b"w\0" as *const u8:   l398 = UNIQUE | NON_NULL, (empty)
    // 1703: name: typeof(_121 = move _122 as *const i8 (Pointer(MutToConstPointer))) = *const {l395} i8
    // 1703: name:   l395 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0" as *cons ... _char: typeof(_123 = move _124 as *const i8 (Misc)) = *const {l399} i8
    // 1703: b"w\0" as *cons ... _char:   l399 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0": typeof(_125 = &raw const (*_126)) = *const {l397} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1703: b"w\0":   l397 = UNIQUE | NON_NULL, (empty)
    // 1703: b"w\0": typeof(_126 = const b"w\x00") = & {l396} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1703: b"w\0":   l396 = UNIQUE | NON_NULL, (empty)
    data.noptsfuzz = (*env_0).noptsfuzz;
    if !(data.trace).is_null() {
    // 1705: (data.trace): typeof(_131) = *mut {l216} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1705: (data.trace):   l216 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"data.trace\0" as *const u8 as *const libc::c_char,
            // 1708: b"data.trace\0" ... _char: typeof(_134) = *const {l220} i8
            // 1708: b"data.trace\0" ... _char:   l220 = UNIQUE | NON_NULL, (empty)
            // 1708: b"data.trace\0" ... st u8: typeof(_135) = *const {l222} u8
            // 1708: b"data.trace\0" ... st u8:   l222 = UNIQUE | NON_NULL, (empty)
            // 1708: b"data.trace\0": typeof(_136) = *const {l224} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 1708: b"data.trace\0":   l224 = UNIQUE | NON_NULL, (empty)
            // 1708: b"data.trace\0": typeof(_137) = & {l226} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 1708: b"data.trace\0":   l226 = UNIQUE | NON_NULL, FIXED
            // 1708: b"data.trace\0": typeof(_136 = &raw const (*_137)) = *const {l401} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 1708: b"data.trace\0":   l401 = UNIQUE | NON_NULL, (empty)
            // 1708: b"data.trace\0" ... _char: typeof(_134 = move _135 as *const i8 (Misc)) = *const {l403} i8
            // 1708: b"data.trace\0" ... _char:   l403 = UNIQUE | NON_NULL, (empty)
            // 1708: b"data.trace\0" ... st u8: typeof(_135 = move _136 as *const u8 (Pointer(ArrayToPointer))) = *const {l402} u8
            // 1708: b"data.trace\0" ... st u8:   l402 = UNIQUE | NON_NULL, (empty)
            // 1708: b"data.trace\0": typeof(_137 = const b"data.trace\x00") = & {l400} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 1708: b"data.trace\0":   l400 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1709: b"lglmbt.c\0" a ... _char: typeof(_138) = *const {l228} i8
            // 1709: b"lglmbt.c\0" a ... _char:   l228 = UNIQUE | NON_NULL, (empty)
            // 1709: b"lglmbt.c\0" a ... st u8: typeof(_139) = *const {l230} u8
            // 1709: b"lglmbt.c\0" a ... st u8:   l230 = UNIQUE | NON_NULL, (empty)
            // 1709: b"lglmbt.c\0": typeof(_140) = *const {l232} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1709: b"lglmbt.c\0":   l232 = UNIQUE | NON_NULL, (empty)
            // 1709: b"lglmbt.c\0": typeof(_141) = & {l234} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1709: b"lglmbt.c\0":   l234 = UNIQUE | NON_NULL, FIXED
            // 1709: b"lglmbt.c\0" a ... st u8: typeof(_139 = move _140 as *const u8 (Pointer(ArrayToPointer))) = *const {l406} u8
            // 1709: b"lglmbt.c\0" a ... st u8:   l406 = UNIQUE | NON_NULL, (empty)
            // 1709: b"lglmbt.c\0" a ... _char: typeof(_138 = move _139 as *const i8 (Misc)) = *const {l407} i8
            // 1709: b"lglmbt.c\0" a ... _char:   l407 = UNIQUE | NON_NULL, (empty)
            // 1709: b"lglmbt.c\0": typeof(_141 = const b"lglmbt.c\x00") = & {l404} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1709: b"lglmbt.c\0":   l404 = UNIQUE | NON_NULL, (empty)
            // 1709: b"lglmbt.c\0": typeof(_140 = &raw const (*_141)) = *const {l405} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1709: b"lglmbt.c\0":   l405 = UNIQUE | NON_NULL, (empty)
            516 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(b"void printrace(Env *)\0"))
            // 1711: (*::core::mem:: ... ptr(): typeof(_144) = *const {l238} i8
            // 1711: (*::core::mem:: ... ptr():   l238 = UNIQUE | NON_NULL, (empty)
            // 1711: (*::core::mem:: ... ptr(): typeof(_145) = & {l240} [i8]
            // 1711: (*::core::mem:: ... ptr():   l240 = UNIQUE | NON_NULL, FIXED
            // 1711: (*::core::mem:: ... ptr(): typeof(_146) = & {l242} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: (*::core::mem:: ... ptr():   l242 = UNIQUE | NON_NULL, (empty)
            // 1711: ::core::mem::tr ... )\0"): typeof(_147) = & {l244} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: ::core::mem::tr ... )\0"):   l244 = UNIQUE | NON_NULL, FIXED
            // 1711: b"void printrac ... *)\0": typeof(_148) = & {l246} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: b"void printrac ... *)\0":   l246 = UNIQUE | NON_NULL, (empty)
            // 1711: b"void printrac ... *)\0": typeof(_149) = & {l248} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: b"void printrac ... *)\0":   l248 = UNIQUE | NON_NULL, FIXED
            // 1711: (*::core::mem:: ... ptr(): typeof(_145 = move _146 as &[i8] (Pointer(Unsize))) = & {l411} [i8]
            // 1711: (*::core::mem:: ... ptr():   l411 = UNIQUE | NON_NULL, FIXED
            // 1711: (*::core::mem:: ... ptr(): typeof(_146 = &(*_147)) = & {l410} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: (*::core::mem:: ... ptr():   l410 = UNIQUE | NON_NULL, (empty)
            // 1711: b"void printrac ... *)\0": typeof(_148 = &(*_149)) = & {l409} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: b"void printrac ... *)\0":   l409 = UNIQUE | NON_NULL, (empty)
            // 1711: b"void printrac ... *)\0": typeof(_149 = const b"void printrace(Env *)\x00") = & {l408} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 1711: b"void printrac ... *)\0":   l408 = UNIQUE | NON_NULL, (empty)
                .as_ptr(),
        );
    }
    'c_8104: {
        if !(data.trace).is_null() {
        // 1716: (data.trace): typeof(_153) = *mut {l253} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1716: (data.trace):   l253 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"data.trace\0" as *const u8 as *const libc::c_char,
                // 1719: b"data.trace\0" ... _char: typeof(_156) = *const {l257} i8
                // 1719: b"data.trace\0" ... _char:   l257 = UNIQUE | NON_NULL, (empty)
                // 1719: b"data.trace\0" ... st u8: typeof(_157) = *const {l259} u8
                // 1719: b"data.trace\0" ... st u8:   l259 = UNIQUE | NON_NULL, (empty)
                // 1719: b"data.trace\0": typeof(_158) = *const {l261} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1719: b"data.trace\0":   l261 = UNIQUE | NON_NULL, (empty)
                // 1719: b"data.trace\0": typeof(_159) = & {l263} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1719: b"data.trace\0":   l263 = UNIQUE | NON_NULL, FIXED
                // 1719: b"data.trace\0": typeof(_159 = const b"data.trace\x00") = & {l412} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1719: b"data.trace\0":   l412 = UNIQUE | NON_NULL, (empty)
                // 1719: b"data.trace\0": typeof(_158 = &raw const (*_159)) = *const {l413} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                // 1719: b"data.trace\0":   l413 = UNIQUE | NON_NULL, (empty)
                // 1719: b"data.trace\0" ... st u8: typeof(_157 = move _158 as *const u8 (Pointer(ArrayToPointer))) = *const {l414} u8
                // 1719: b"data.trace\0" ... st u8:   l414 = UNIQUE | NON_NULL, (empty)
                // 1719: b"data.trace\0" ... _char: typeof(_156 = move _157 as *const i8 (Misc)) = *const {l415} i8
                // 1719: b"data.trace\0" ... _char:   l415 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1720: b"lglmbt.c\0" a ... _char: typeof(_160) = *const {l265} i8
                // 1720: b"lglmbt.c\0" a ... _char:   l265 = UNIQUE | NON_NULL, (empty)
                // 1720: b"lglmbt.c\0" a ... st u8: typeof(_161) = *const {l267} u8
                // 1720: b"lglmbt.c\0" a ... st u8:   l267 = UNIQUE | NON_NULL, (empty)
                // 1720: b"lglmbt.c\0": typeof(_162) = *const {l269} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1720: b"lglmbt.c\0":   l269 = UNIQUE | NON_NULL, (empty)
                // 1720: b"lglmbt.c\0": typeof(_163) = & {l271} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1720: b"lglmbt.c\0":   l271 = UNIQUE | NON_NULL, FIXED
                // 1720: b"lglmbt.c\0" a ... st u8: typeof(_161 = move _162 as *const u8 (Pointer(ArrayToPointer))) = *const {l418} u8
                // 1720: b"lglmbt.c\0" a ... st u8:   l418 = UNIQUE | NON_NULL, (empty)
                // 1720: b"lglmbt.c\0": typeof(_163 = const b"lglmbt.c\x00") = & {l416} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1720: b"lglmbt.c\0":   l416 = UNIQUE | NON_NULL, (empty)
                // 1720: b"lglmbt.c\0" a ... _char: typeof(_160 = move _161 as *const i8 (Misc)) = *const {l419} i8
                // 1720: b"lglmbt.c\0" a ... _char:   l419 = UNIQUE | NON_NULL, (empty)
                // 1720: b"lglmbt.c\0": typeof(_162 = &raw const (*_163)) = *const {l417} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1720: b"lglmbt.c\0":   l417 = UNIQUE | NON_NULL, (empty)
                516 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 22], &[libc::c_char; 22]>(
                // 1722: (*::core::mem:: ... ptr(): typeof(_166) = *const {l275} i8
                // 1722: (*::core::mem:: ... ptr():   l275 = UNIQUE | NON_NULL, (empty)
                // 1722: (*::core::mem:: ... ptr(): typeof(_167) = & {l277} [i8]
                // 1722: (*::core::mem:: ... ptr():   l277 = UNIQUE | NON_NULL, FIXED
                // 1722: (*::core::mem:: ... ptr(): typeof(_168) = & {l279} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1722: (*::core::mem:: ... ptr():   l279 = UNIQUE | NON_NULL, (empty)
                // 1722: ::core::mem::tr ... 0", ): typeof(_169) = & {l281} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1722: ::core::mem::tr ... 0", ):   l281 = UNIQUE | NON_NULL, FIXED
                // 1722: (*::core::mem:: ... ptr(): typeof(_168 = &(*_169)) = & {l422} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 1722: (*::core::mem:: ... ptr():   l422 = UNIQUE | NON_NULL, (empty)
                // 1722: (*::core::mem:: ... ptr(): typeof(_167 = move _168 as &[i8] (Pointer(Unsize))) = & {l423} [i8]
                // 1722: (*::core::mem:: ... ptr():   l423 = UNIQUE | NON_NULL, FIXED
                    b"void printrace(Env *)\0",
                    // 1723: b"void printrac ... *)\0": typeof(_170) = & {l283} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1723: b"void printrac ... *)\0":   l283 = UNIQUE | NON_NULL, (empty)
                    // 1723: b"void printrac ... *)\0": typeof(_171) = & {l285} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1723: b"void printrac ... *)\0":   l285 = UNIQUE | NON_NULL, FIXED
                    // 1723: b"void printrac ... *)\0": typeof(_171 = const b"void printrace(Env *)\x00") = & {l420} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1723: b"void printrac ... *)\0":   l420 = UNIQUE | NON_NULL, (empty)
                    // 1723: b"void printrac ... *)\0": typeof(_170 = &(*_171)) = & {l421} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                    // 1723: b"void printrac ... *)\0":   l421 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while i < (*env_0).nevents {
        e = ((*env_0).events).offset(i as isize);
        // 1731: ((*env_0).event ... size): typeof(_178) = *mut {l293} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1731: ((*env_0).event ... size):   l293 = UNIQUE | NON_NULL, (empty)
        // 1731: ((*env_0).events): typeof(_179) = *mut {l295} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1731: ((*env_0).events):   l295 = UNIQUE | NON_NULL, (empty)
        if !((*e).remove != 0) {
            ((*e).state).expect("non-null function pointer")(&mut data, (*e).rand);
            // 1733: ((*e).state).ex ... rand): typeof(_186) = *mut {l303} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1733: ((*e).state).ex ... rand):   l303 = UNIQUE | NON_NULL, (empty)
            // 1733: ((*e).state).ex ... ter"): typeof(_187) = fn(*mut {l305} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l306} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 1733: ((*e).state).ex ... ter"):   l305 = UNIQUE | NON_NULL, (empty)
            // 1733: ((*e).state).ex ... ter"):   l306 = UNIQUE | NON_NULL, (empty)
            // 1733: ((*e).state): typeof(_188) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l308} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l309} DefId(2:5583 ~ core[480a]::ffi::c_void)>
            // 1733: ((*e).state):   l308 = UNIQUE | NON_NULL, (empty)
            // 1733: ((*e).state):   l309 = UNIQUE | NON_NULL, (empty)
            // 1733: "non-null funct ... nter": typeof(_189) = & {l311} str
            // 1733: "non-null funct ... nter":   l311 = UNIQUE | NON_NULL, (empty)
            // 1733: "non-null funct ... nter": typeof(_190) = & {l313} str
            // 1733: "non-null funct ... nter":   l313 = UNIQUE | NON_NULL, FIXED
            // 1733: &mut data: typeof(_191) = *mut {l315} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1733: &mut data:   l315 = UNIQUE | NON_NULL, (empty)
            // 1733: &mut data: typeof(_192) = &mut {l317} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1733: &mut data:   l317 = UNIQUE | NON_NULL, (empty)
            // 1733: &mut data: typeof(_192 = &mut _3) = &mut {l426} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1733: &mut data:   l426 = UNIQUE | NON_NULL, (empty)
            // 1733: &mut data: typeof(_191 = &raw mut (*_192)) = *mut {l427} DefId(0:299 ~ lglmbt[b165]::Data)
            // 1733: &mut data:   l427 = UNIQUE | NON_NULL, (empty)
            // 1733: "non-null funct ... nter": typeof(_189 = &(*_190)) = & {l425} str
            // 1733: "non-null funct ... nter":   l425 = UNIQUE | NON_NULL, (empty)
            // 1733: "non-null funct ... nter": typeof(_190 = const "non-null function pointer") = & {l424} str
            // 1733: "non-null funct ... nter":   l424 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    fclose(data.trace);
    // 1738: data.trace: typeof(_200) = *mut {l326} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1738: data.trace:   l326 = UNIQUE | NON_NULL, (empty)
    free(name as *mut libc::c_void);
    // 1739: name as *mut li ... _void: typeof(_202) = *mut {l329} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1739: name as *mut li ... _void:   l329 = UNIQUE | NON_NULL, (empty)
    // 1739: name: typeof(_203) = *mut {l331} i8
    // 1739: name:   l331 = UNIQUE | NON_NULL, (empty)
    // 1739: name as *mut li ... _void: typeof(_202 = move _203 as *mut libc::c_void (Misc)) = *mut {l428} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1739: name as *mut li ... _void:   l428 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn prwc(mut env_0: *mut Env, mut prefix: *const libc::c_char) {
// 1741: mut env_0: typeof(_1) = *mut {g36} DefId(0:327 ~ lglmbt[b165]::Env)
// 1741: mut env_0:   g36 = UNIQUE | NON_NULL, FIXED
// 1741: mut prefix: typeof(_2) = *const {g37} i8
// 1741: mut prefix:   g37 = UNIQUE | NON_NULL, FIXED
    let mut name: *mut libc::c_char =
    // 1742: mut name: typeof(_3) = *mut {l3} i8
    // 1742: mut name:   l3 = UNIQUE | NON_NULL, (empty)
        malloc((strlen(prefix)).wrapping_add(80 as libc::c_int as libc::c_ulong))
        // 1743: malloc((strlen( ... ong)): typeof(_4) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 1743: malloc((strlen( ... ong)):   l5 = UNIQUE | NON_NULL, (empty)
        // 1743: prefix: typeof(_7) = *const {l9} i8
        // 1743: prefix:   l9 = UNIQUE | NON_NULL, (empty)
        // 1743: malloc((strlen( ... _char: typeof(_3 = move _4 as *mut i8 (Misc)) = *mut {l166} i8
        // 1743: malloc((strlen( ... _char:   l166 = UNIQUE | NON_NULL, (empty)
            as *mut libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut file: *mut FILE = 0 as *mut FILE;
    // 1747: mut file: typeof(_12) = *mut {l15} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1747: mut file:   l15 = UNIQUE | NON_NULL, (empty)
    // 1747: 0 as *mut FILE: typeof(_12 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l167} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1747: 0 as *mut FILE:   l167 = UNIQUE | NON_NULL, (empty)
    sprintf(
        name,
        // 1749: name: typeof(_14) = *mut {l18} i8
        // 1749: name:   l18 = UNIQUE | NON_NULL, (empty)
        b"%s-%u.trace\0" as *const u8 as *const libc::c_char,
        // 1750: b"%s-%u.trace\0 ... _char: typeof(_15) = *const {l20} i8
        // 1750: b"%s-%u.trace\0 ... _char:   l20 = UNIQUE | NON_NULL, (empty)
        // 1750: b"%s-%u.trace\0 ... st u8: typeof(_16) = *const {l22} u8
        // 1750: b"%s-%u.trace\0 ... st u8:   l22 = UNIQUE | NON_NULL, (empty)
        // 1750: b"%s-%u.trace\0": typeof(_17) = *const {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1750: b"%s-%u.trace\0":   l24 = UNIQUE | NON_NULL, (empty)
        // 1750: b"%s-%u.trace\0": typeof(_18) = & {l26} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1750: b"%s-%u.trace\0":   l26 = UNIQUE | NON_NULL, FIXED
        // 1750: b"%s-%u.trace\0": typeof(_18 = const b"%s-%u.trace\x00") = & {l168} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1750: b"%s-%u.trace\0":   l168 = UNIQUE | NON_NULL, (empty)
        // 1750: b"%s-%u.trace\0 ... st u8: typeof(_16 = move _17 as *const u8 (Pointer(ArrayToPointer))) = *const {l170} u8
        // 1750: b"%s-%u.trace\0 ... st u8:   l170 = UNIQUE | NON_NULL, (empty)
        // 1750: b"%s-%u.trace\0": typeof(_17 = &raw const (*_18)) = *const {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000c)) }]
        // 1750: b"%s-%u.trace\0":   l169 = UNIQUE | NON_NULL, (empty)
        // 1750: b"%s-%u.trace\0 ... _char: typeof(_15 = move _16 as *const i8 (Misc)) = *const {l171} i8
        // 1750: b"%s-%u.trace\0 ... _char:   l171 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 1751: prefix: typeof(_19) = *const {l28} i8
        // 1751: prefix:   l28 = UNIQUE | NON_NULL, (empty)
        (*env_0).seed,
    );
    file = fopen(name, b"r\0" as *const u8 as *const libc::c_char);
    // 1754: fopen(name, b"r ... char): typeof(_21) = *mut {l31} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1754: fopen(name, b"r ... char):   l31 = UNIQUE | NON_NULL, (empty)
    // 1754: name: typeof(_22) = *const {l33} i8
    // 1754: name:   l33 = UNIQUE | NON_NULL, (empty)
    // 1754: name: typeof(_23) = *mut {l35} i8
    // 1754: name:   l35 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0" as *cons ... _char: typeof(_24) = *const {l37} i8
    // 1754: b"r\0" as *cons ... _char:   l37 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0" as *const u8: typeof(_25) = *const {l39} u8
    // 1754: b"r\0" as *const u8:   l39 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0": typeof(_26) = *const {l41} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1754: b"r\0":   l41 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0": typeof(_27) = & {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1754: b"r\0":   l43 = UNIQUE | NON_NULL, FIXED
    // 1754: b"r\0" as *const u8: typeof(_25 = move _26 as *const u8 (Pointer(ArrayToPointer))) = *const {l175} u8
    // 1754: b"r\0" as *const u8:   l175 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0" as *cons ... _char: typeof(_24 = move _25 as *const i8 (Misc)) = *const {l176} i8
    // 1754: b"r\0" as *cons ... _char:   l176 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0": typeof(_26 = &raw const (*_27)) = *const {l174} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1754: b"r\0":   l174 = UNIQUE | NON_NULL, (empty)
    // 1754: b"r\0": typeof(_27 = const b"r\x00") = & {l173} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1754: b"r\0":   l173 = UNIQUE | NON_NULL, (empty)
    // 1754: name: typeof(_22 = move _23 as *const i8 (Pointer(MutToConstPointer))) = *const {l172} i8
    // 1754: name:   l172 = UNIQUE | NON_NULL, (empty)
    if !file.is_null() {
    // 1755: file: typeof(_31) = *mut {l48} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1755: file:   l48 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            // 1758: b"file\0" as *c ... _char: typeof(_34) = *const {l52} i8
            // 1758: b"file\0" as *c ... _char:   l52 = UNIQUE | NON_NULL, (empty)
            // 1758: b"file\0" as *c ... st u8: typeof(_35) = *const {l54} u8
            // 1758: b"file\0" as *c ... st u8:   l54 = UNIQUE | NON_NULL, (empty)
            // 1758: b"file\0": typeof(_36) = *const {l56} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1758: b"file\0":   l56 = UNIQUE | NON_NULL, (empty)
            // 1758: b"file\0": typeof(_37) = & {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1758: b"file\0":   l58 = UNIQUE | NON_NULL, FIXED
            // 1758: b"file\0" as *c ... st u8: typeof(_35 = move _36 as *const u8 (Pointer(ArrayToPointer))) = *const {l179} u8
            // 1758: b"file\0" as *c ... st u8:   l179 = UNIQUE | NON_NULL, (empty)
            // 1758: b"file\0": typeof(_36 = &raw const (*_37)) = *const {l178} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1758: b"file\0":   l178 = UNIQUE | NON_NULL, (empty)
            // 1758: b"file\0" as *c ... _char: typeof(_34 = move _35 as *const i8 (Misc)) = *const {l180} i8
            // 1758: b"file\0" as *c ... _char:   l180 = UNIQUE | NON_NULL, (empty)
            // 1758: b"file\0": typeof(_37 = const b"file\x00") = & {l177} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1758: b"file\0":   l177 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1759: b"lglmbt.c\0" a ... _char: typeof(_38) = *const {l60} i8
            // 1759: b"lglmbt.c\0" a ... _char:   l60 = UNIQUE | NON_NULL, (empty)
            // 1759: b"lglmbt.c\0" a ... st u8: typeof(_39) = *const {l62} u8
            // 1759: b"lglmbt.c\0" a ... st u8:   l62 = UNIQUE | NON_NULL, (empty)
            // 1759: b"lglmbt.c\0": typeof(_40) = *const {l64} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1759: b"lglmbt.c\0":   l64 = UNIQUE | NON_NULL, (empty)
            // 1759: b"lglmbt.c\0": typeof(_41) = & {l66} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1759: b"lglmbt.c\0":   l66 = UNIQUE | NON_NULL, FIXED
            // 1759: b"lglmbt.c\0": typeof(_40 = &raw const (*_41)) = *const {l182} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1759: b"lglmbt.c\0":   l182 = UNIQUE | NON_NULL, (empty)
            // 1759: b"lglmbt.c\0" a ... _char: typeof(_38 = move _39 as *const i8 (Misc)) = *const {l184} i8
            // 1759: b"lglmbt.c\0" a ... _char:   l184 = UNIQUE | NON_NULL, (empty)
            // 1759: b"lglmbt.c\0": typeof(_41 = const b"lglmbt.c\x00") = & {l181} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1759: b"lglmbt.c\0":   l181 = UNIQUE | NON_NULL, (empty)
            // 1759: b"lglmbt.c\0" a ... st u8: typeof(_39 = move _40 as *const u8 (Pointer(ArrayToPointer))) = *const {l183} u8
            // 1759: b"lglmbt.c\0" a ... st u8:   l183 = UNIQUE | NON_NULL, (empty)
            532 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
            // 1761: (*::core::mem:: ... ptr(): typeof(_44) = *const {l70} i8
            // 1761: (*::core::mem:: ... ptr():   l70 = UNIQUE | NON_NULL, (empty)
            // 1761: (*::core::mem:: ... ptr(): typeof(_45) = & {l72} [i8]
            // 1761: (*::core::mem:: ... ptr():   l72 = UNIQUE | NON_NULL, FIXED
            // 1761: (*::core::mem:: ... ptr(): typeof(_46) = & {l74} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1761: (*::core::mem:: ... ptr():   l74 = UNIQUE | NON_NULL, (empty)
            // 1761: ::core::mem::tr ... 0", ): typeof(_47) = & {l76} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1761: ::core::mem::tr ... 0", ):   l76 = UNIQUE | NON_NULL, FIXED
            // 1761: (*::core::mem:: ... ptr(): typeof(_46 = &(*_47)) = & {l187} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
            // 1761: (*::core::mem:: ... ptr():   l187 = UNIQUE | NON_NULL, (empty)
            // 1761: (*::core::mem:: ... ptr(): typeof(_45 = move _46 as &[i8] (Pointer(Unsize))) = & {l188} [i8]
            // 1761: (*::core::mem:: ... ptr():   l188 = UNIQUE | NON_NULL, FIXED
                b"void prwc(Env *, const char *)\0",
                // 1762: b"void prwc(Env ... *)\0": typeof(_48) = & {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1762: b"void prwc(Env ... *)\0":   l78 = UNIQUE | NON_NULL, (empty)
                // 1762: b"void prwc(Env ... *)\0": typeof(_49) = & {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1762: b"void prwc(Env ... *)\0":   l80 = UNIQUE | NON_NULL, FIXED
                // 1762: b"void prwc(Env ... *)\0": typeof(_48 = &(*_49)) = & {l186} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1762: b"void prwc(Env ... *)\0":   l186 = UNIQUE | NON_NULL, (empty)
                // 1762: b"void prwc(Env ... *)\0": typeof(_49 = const b"void prwc(Env *, const char *)\x00") = & {l185} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1762: b"void prwc(Env ... *)\0":   l185 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_8363: {
        if !file.is_null() {
        // 1768: file: typeof(_53) = *mut {l85} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1768: file:   l85 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                // 1771: b"file\0" as *c ... _char: typeof(_56) = *const {l89} i8
                // 1771: b"file\0" as *c ... _char:   l89 = UNIQUE | NON_NULL, (empty)
                // 1771: b"file\0" as *c ... st u8: typeof(_57) = *const {l91} u8
                // 1771: b"file\0" as *c ... st u8:   l91 = UNIQUE | NON_NULL, (empty)
                // 1771: b"file\0": typeof(_58) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1771: b"file\0":   l93 = UNIQUE | NON_NULL, (empty)
                // 1771: b"file\0": typeof(_59) = & {l95} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1771: b"file\0":   l95 = UNIQUE | NON_NULL, FIXED
                // 1771: b"file\0" as *c ... st u8: typeof(_57 = move _58 as *const u8 (Pointer(ArrayToPointer))) = *const {l191} u8
                // 1771: b"file\0" as *c ... st u8:   l191 = UNIQUE | NON_NULL, (empty)
                // 1771: b"file\0" as *c ... _char: typeof(_56 = move _57 as *const i8 (Misc)) = *const {l192} i8
                // 1771: b"file\0" as *c ... _char:   l192 = UNIQUE | NON_NULL, (empty)
                // 1771: b"file\0": typeof(_58 = &raw const (*_59)) = *const {l190} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1771: b"file\0":   l190 = UNIQUE | NON_NULL, (empty)
                // 1771: b"file\0": typeof(_59 = const b"file\x00") = & {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1771: b"file\0":   l189 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1772: b"lglmbt.c\0" a ... _char: typeof(_60) = *const {l97} i8
                // 1772: b"lglmbt.c\0" a ... _char:   l97 = UNIQUE | NON_NULL, (empty)
                // 1772: b"lglmbt.c\0" a ... st u8: typeof(_61) = *const {l99} u8
                // 1772: b"lglmbt.c\0" a ... st u8:   l99 = UNIQUE | NON_NULL, (empty)
                // 1772: b"lglmbt.c\0": typeof(_62) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1772: b"lglmbt.c\0":   l101 = UNIQUE | NON_NULL, (empty)
                // 1772: b"lglmbt.c\0": typeof(_63) = & {l103} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1772: b"lglmbt.c\0":   l103 = UNIQUE | NON_NULL, FIXED
                // 1772: b"lglmbt.c\0": typeof(_63 = const b"lglmbt.c\x00") = & {l193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1772: b"lglmbt.c\0":   l193 = UNIQUE | NON_NULL, (empty)
                // 1772: b"lglmbt.c\0" a ... _char: typeof(_60 = move _61 as *const i8 (Misc)) = *const {l196} i8
                // 1772: b"lglmbt.c\0" a ... _char:   l196 = UNIQUE | NON_NULL, (empty)
                // 1772: b"lglmbt.c\0" a ... st u8: typeof(_61 = move _62 as *const u8 (Pointer(ArrayToPointer))) = *const {l195} u8
                // 1772: b"lglmbt.c\0" a ... st u8:   l195 = UNIQUE | NON_NULL, (empty)
                // 1772: b"lglmbt.c\0": typeof(_62 = &raw const (*_63)) = *const {l194} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1772: b"lglmbt.c\0":   l194 = UNIQUE | NON_NULL, (empty)
                532 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 31], &[libc::c_char; 31]>(
                // 1774: (*::core::mem:: ... ptr(): typeof(_66) = *const {l107} i8
                // 1774: (*::core::mem:: ... ptr():   l107 = UNIQUE | NON_NULL, (empty)
                // 1774: (*::core::mem:: ... ptr(): typeof(_67) = & {l109} [i8]
                // 1774: (*::core::mem:: ... ptr():   l109 = UNIQUE | NON_NULL, FIXED
                // 1774: (*::core::mem:: ... ptr(): typeof(_68) = & {l111} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1774: (*::core::mem:: ... ptr():   l111 = UNIQUE | NON_NULL, (empty)
                // 1774: ::core::mem::tr ... 0", ): typeof(_69) = & {l113} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1774: ::core::mem::tr ... 0", ):   l113 = UNIQUE | NON_NULL, FIXED
                // 1774: (*::core::mem:: ... ptr(): typeof(_67 = move _68 as &[i8] (Pointer(Unsize))) = & {l200} [i8]
                // 1774: (*::core::mem:: ... ptr():   l200 = UNIQUE | NON_NULL, FIXED
                // 1774: (*::core::mem:: ... ptr(): typeof(_68 = &(*_69)) = & {l199} [i8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                // 1774: (*::core::mem:: ... ptr():   l199 = UNIQUE | NON_NULL, (empty)
                    b"void prwc(Env *, const char *)\0",
                    // 1775: b"void prwc(Env ... *)\0": typeof(_70) = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 1775: b"void prwc(Env ... *)\0":   l115 = UNIQUE | NON_NULL, (empty)
                    // 1775: b"void prwc(Env ... *)\0": typeof(_71) = & {l117} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 1775: b"void prwc(Env ... *)\0":   l117 = UNIQUE | NON_NULL, FIXED
                    // 1775: b"void prwc(Env ... *)\0": typeof(_71 = const b"void prwc(Env *, const char *)\x00") = & {l197} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 1775: b"void prwc(Env ... *)\0":   l197 = UNIQUE | NON_NULL, (empty)
                    // 1775: b"void prwc(Env ... *)\0": typeof(_70 = &(*_71)) = & {l198} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                    // 1775: b"void prwc(Env ... *)\0":   l198 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    loop {
        ch = getc(file);
        // 1782: file: typeof(_75) = *mut {l122} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1782: file:   l122 = UNIQUE | NON_NULL, (empty)
        if !(ch != -(1 as libc::c_int)) {
            break;
        }
        if ch == '\n' as i32 {
            res += 1;
            res;
        }
    }
    fclose(file);
    // 1791: file: typeof(_90) = *mut {l138} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1791: file:   l138 = UNIQUE | NON_NULL, (empty)
    if (*env_0).quiet == 0 {
        printf(b" %s %d\0" as *const u8 as *const libc::c_char, name, res);
        // 1793: b" %s %d\0" as  ... _char: typeof(_95) = *const {l144} i8
        // 1793: b" %s %d\0" as  ... _char:   l144 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0" as  ... st u8: typeof(_96) = *const {l146} u8
        // 1793: b" %s %d\0" as  ... st u8:   l146 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0": typeof(_97) = *const {l148} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1793: b" %s %d\0":   l148 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0": typeof(_98) = & {l150} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1793: b" %s %d\0":   l150 = UNIQUE | NON_NULL, FIXED
        // 1793: name: typeof(_99) = *mut {l152} i8
        // 1793: name:   l152 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0": typeof(_97 = &raw const (*_98)) = *const {l202} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1793: b" %s %d\0":   l202 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0" as  ... _char: typeof(_95 = move _96 as *const i8 (Misc)) = *const {l204} i8
        // 1793: b" %s %d\0" as  ... _char:   l204 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0" as  ... st u8: typeof(_96 = move _97 as *const u8 (Pointer(ArrayToPointer))) = *const {l203} u8
        // 1793: b" %s %d\0" as  ... st u8:   l203 = UNIQUE | NON_NULL, (empty)
        // 1793: b" %s %d\0": typeof(_98 = const b" %s %d\x00") = & {l201} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1793: b" %s %d\0":   l201 = UNIQUE | NON_NULL, (empty)
        fflush(stdout);
        // 1794: stdout: typeof(_102) = *mut {l156} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1794: stdout:   l156 = UNIQUE | NON_NULL, (empty)
        // 1794: stdout: typeof(_103) = *mut {l158} *mut {l159} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1794: stdout:   l158 = UNIQUE | NON_NULL, (empty)
        // 1794: stdout:   l159 = UNIQUE | NON_NULL, (empty)
    }
    free(name as *mut libc::c_void);
    // 1796: name as *mut li ... _void: typeof(_105) = *mut {l162} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1796: name as *mut li ... _void:   l162 = UNIQUE | NON_NULL, (empty)
    // 1796: name: typeof(_106) = *mut {l164} i8
    // 1796: name:   l164 = UNIQUE | NON_NULL, (empty)
    // 1796: name as *mut li ... _void: typeof(_105 = move _106 as *mut libc::c_void (Misc)) = *mut {l205} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1796: name as *mut li ... _void:   l205 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn dd(
    mut env_0: *mut Env,
    // 1799: mut env_0: typeof(_1) = *mut {g38} DefId(0:327 ~ lglmbt[b165]::Env)
    // 1799: mut env_0:   g38 = UNIQUE | NON_NULL, FIXED
    mut filename: *const libc::c_char,
    // 1800: mut filename: typeof(_2) = *const {g39} i8
    // 1800: mut filename:   g39 = UNIQUE | NON_NULL, FIXED
    mut golden: libc::c_int,
    mut opt: libc::c_int,
) {
    let mut rand: libc::c_uint = 0;
    let mut state: State = None;
    // 1805: mut state: typeof(_6) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l6} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l7} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1805: mut state:   l6 = UNIQUE | NON_NULL, (empty)
    // 1805: mut state:   l7 = UNIQUE | NON_NULL, (empty)
    // 1805: None: typeof(_6 = std::option::Option::<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>::None) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l533} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l534} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 1805: None:   l533 = UNIQUE | NON_NULL, (empty)
    // 1805: None:   l534 = UNIQUE | NON_NULL, (empty)
    let mut file: *mut FILE = 0 as *mut FILE;
    // 1806: mut file: typeof(_7) = *mut {l9} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1806: mut file:   l9 = UNIQUE | NON_NULL, (empty)
    // 1806: 0 as *mut FILE: typeof(_7 = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l535} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1806: 0 as *mut FILE:   l535 = UNIQUE | NON_NULL, (empty)
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    // 1807: mut cmd: typeof(_8) = *mut {l11} i8
    // 1807: mut cmd:   l11 = UNIQUE | NON_NULL, (empty)
    // 1807: 0 as *mut libc: ... _char: typeof(_8 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l536} i8
    // 1807: 0 as *mut libc: ... _char:   l536 = UNIQUE | NON_NULL, (empty)
    let mut i: libc::c_int = 0;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    // 1809: fopen(filename, ... char): typeof(_10) = *mut {l14} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1809: fopen(filename, ... char):   l14 = UNIQUE | NON_NULL, (empty)
    // 1809: filename: typeof(_11) = *const {l16} i8
    // 1809: filename:   l16 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0" as *cons ... _char: typeof(_12) = *const {l18} i8
    // 1809: b"r\0" as *cons ... _char:   l18 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0" as *const u8: typeof(_13) = *const {l20} u8
    // 1809: b"r\0" as *const u8:   l20 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0": typeof(_14) = *const {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1809: b"r\0":   l22 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0": typeof(_15) = & {l24} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1809: b"r\0":   l24 = UNIQUE | NON_NULL, FIXED
    // 1809: b"r\0": typeof(_14 = &raw const (*_15)) = *const {l538} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1809: b"r\0":   l538 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0": typeof(_15 = const b"r\x00") = & {l537} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1809: b"r\0":   l537 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0" as *cons ... _char: typeof(_12 = move _13 as *const i8 (Misc)) = *const {l540} i8
    // 1809: b"r\0" as *cons ... _char:   l540 = UNIQUE | NON_NULL, (empty)
    // 1809: b"r\0" as *const u8: typeof(_13 = move _14 as *const u8 (Pointer(ArrayToPointer))) = *const {l539} u8
    // 1809: b"r\0" as *const u8:   l539 = UNIQUE | NON_NULL, (empty)
    if !file.is_null() {
    // 1810: file: typeof(_19) = *mut {l29} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1810: file:   l29 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            // 1813: b"file\0" as *c ... _char: typeof(_22) = *const {l33} i8
            // 1813: b"file\0" as *c ... _char:   l33 = UNIQUE | NON_NULL, (empty)
            // 1813: b"file\0" as *c ... st u8: typeof(_23) = *const {l35} u8
            // 1813: b"file\0" as *c ... st u8:   l35 = UNIQUE | NON_NULL, (empty)
            // 1813: b"file\0": typeof(_24) = *const {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1813: b"file\0":   l37 = UNIQUE | NON_NULL, (empty)
            // 1813: b"file\0": typeof(_25) = & {l39} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1813: b"file\0":   l39 = UNIQUE | NON_NULL, FIXED
            // 1813: b"file\0": typeof(_25 = const b"file\x00") = & {l541} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1813: b"file\0":   l541 = UNIQUE | NON_NULL, (empty)
            // 1813: b"file\0" as *c ... st u8: typeof(_23 = move _24 as *const u8 (Pointer(ArrayToPointer))) = *const {l543} u8
            // 1813: b"file\0" as *c ... st u8:   l543 = UNIQUE | NON_NULL, (empty)
            // 1813: b"file\0" as *c ... _char: typeof(_22 = move _23 as *const i8 (Misc)) = *const {l544} i8
            // 1813: b"file\0" as *c ... _char:   l544 = UNIQUE | NON_NULL, (empty)
            // 1813: b"file\0": typeof(_24 = &raw const (*_25)) = *const {l542} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1813: b"file\0":   l542 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1814: b"lglmbt.c\0" a ... _char: typeof(_26) = *const {l41} i8
            // 1814: b"lglmbt.c\0" a ... _char:   l41 = UNIQUE | NON_NULL, (empty)
            // 1814: b"lglmbt.c\0" a ... st u8: typeof(_27) = *const {l43} u8
            // 1814: b"lglmbt.c\0" a ... st u8:   l43 = UNIQUE | NON_NULL, (empty)
            // 1814: b"lglmbt.c\0": typeof(_28) = *const {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1814: b"lglmbt.c\0":   l45 = UNIQUE | NON_NULL, (empty)
            // 1814: b"lglmbt.c\0": typeof(_29) = & {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1814: b"lglmbt.c\0":   l47 = UNIQUE | NON_NULL, FIXED
            // 1814: b"lglmbt.c\0": typeof(_28 = &raw const (*_29)) = *const {l546} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1814: b"lglmbt.c\0":   l546 = UNIQUE | NON_NULL, (empty)
            // 1814: b"lglmbt.c\0" a ... st u8: typeof(_27 = move _28 as *const u8 (Pointer(ArrayToPointer))) = *const {l547} u8
            // 1814: b"lglmbt.c\0" a ... st u8:   l547 = UNIQUE | NON_NULL, (empty)
            // 1814: b"lglmbt.c\0": typeof(_29 = const b"lglmbt.c\x00") = & {l545} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1814: b"lglmbt.c\0":   l545 = UNIQUE | NON_NULL, (empty)
            // 1814: b"lglmbt.c\0" a ... _char: typeof(_26 = move _27 as *const i8 (Misc)) = *const {l548} i8
            // 1814: b"lglmbt.c\0" a ... _char:   l548 = UNIQUE | NON_NULL, (empty)
            550 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
            // 1816: (*::core::mem:: ... ptr(): typeof(_32) = *const {l51} i8
            // 1816: (*::core::mem:: ... ptr():   l51 = UNIQUE | NON_NULL, (empty)
            // 1816: (*::core::mem:: ... ptr(): typeof(_33) = & {l53} [i8]
            // 1816: (*::core::mem:: ... ptr():   l53 = UNIQUE | NON_NULL, FIXED
            // 1816: (*::core::mem:: ... ptr(): typeof(_34) = & {l55} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1816: (*::core::mem:: ... ptr():   l55 = UNIQUE | NON_NULL, (empty)
            // 1816: ::core::mem::tr ... 0", ): typeof(_35) = & {l57} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1816: ::core::mem::tr ... 0", ):   l57 = UNIQUE | NON_NULL, FIXED
            // 1816: (*::core::mem:: ... ptr(): typeof(_33 = move _34 as &[i8] (Pointer(Unsize))) = & {l552} [i8]
            // 1816: (*::core::mem:: ... ptr():   l552 = UNIQUE | NON_NULL, FIXED
            // 1816: (*::core::mem:: ... ptr(): typeof(_34 = &(*_35)) = & {l551} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1816: (*::core::mem:: ... ptr():   l551 = UNIQUE | NON_NULL, (empty)
                b"void dd(Env *, const char *, int, int)\0",
                // 1817: b"void dd(Env * ... t)\0": typeof(_36) = & {l59} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1817: b"void dd(Env * ... t)\0":   l59 = UNIQUE | NON_NULL, (empty)
                // 1817: b"void dd(Env * ... t)\0": typeof(_37) = & {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1817: b"void dd(Env * ... t)\0":   l61 = UNIQUE | NON_NULL, FIXED
                // 1817: b"void dd(Env * ... t)\0": typeof(_36 = &(*_37)) = & {l550} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1817: b"void dd(Env * ... t)\0":   l550 = UNIQUE | NON_NULL, (empty)
                // 1817: b"void dd(Env * ... t)\0": typeof(_37 = const b"void dd(Env *, const char *, int, int)\x00") = & {l549} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1817: b"void dd(Env * ... t)\0":   l549 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_8806: {
        if !file.is_null() {
        // 1823: file: typeof(_41) = *mut {l66} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1823: file:   l66 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                // 1826: b"file\0" as *c ... _char: typeof(_44) = *const {l70} i8
                // 1826: b"file\0" as *c ... _char:   l70 = UNIQUE | NON_NULL, (empty)
                // 1826: b"file\0" as *c ... st u8: typeof(_45) = *const {l72} u8
                // 1826: b"file\0" as *c ... st u8:   l72 = UNIQUE | NON_NULL, (empty)
                // 1826: b"file\0": typeof(_46) = *const {l74} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1826: b"file\0":   l74 = UNIQUE | NON_NULL, (empty)
                // 1826: b"file\0": typeof(_47) = & {l76} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1826: b"file\0":   l76 = UNIQUE | NON_NULL, FIXED
                // 1826: b"file\0": typeof(_46 = &raw const (*_47)) = *const {l554} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1826: b"file\0":   l554 = UNIQUE | NON_NULL, (empty)
                // 1826: b"file\0" as *c ... st u8: typeof(_45 = move _46 as *const u8 (Pointer(ArrayToPointer))) = *const {l555} u8
                // 1826: b"file\0" as *c ... st u8:   l555 = UNIQUE | NON_NULL, (empty)
                // 1826: b"file\0" as *c ... _char: typeof(_44 = move _45 as *const i8 (Misc)) = *const {l556} i8
                // 1826: b"file\0" as *c ... _char:   l556 = UNIQUE | NON_NULL, (empty)
                // 1826: b"file\0": typeof(_47 = const b"file\x00") = & {l553} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1826: b"file\0":   l553 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1827: b"lglmbt.c\0" a ... _char: typeof(_48) = *const {l78} i8
                // 1827: b"lglmbt.c\0" a ... _char:   l78 = UNIQUE | NON_NULL, (empty)
                // 1827: b"lglmbt.c\0" a ... st u8: typeof(_49) = *const {l80} u8
                // 1827: b"lglmbt.c\0" a ... st u8:   l80 = UNIQUE | NON_NULL, (empty)
                // 1827: b"lglmbt.c\0": typeof(_50) = *const {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1827: b"lglmbt.c\0":   l82 = UNIQUE | NON_NULL, (empty)
                // 1827: b"lglmbt.c\0": typeof(_51) = & {l84} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1827: b"lglmbt.c\0":   l84 = UNIQUE | NON_NULL, FIXED
                // 1827: b"lglmbt.c\0": typeof(_50 = &raw const (*_51)) = *const {l558} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1827: b"lglmbt.c\0":   l558 = UNIQUE | NON_NULL, (empty)
                // 1827: b"lglmbt.c\0": typeof(_51 = const b"lglmbt.c\x00") = & {l557} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1827: b"lglmbt.c\0":   l557 = UNIQUE | NON_NULL, (empty)
                // 1827: b"lglmbt.c\0" a ... _char: typeof(_48 = move _49 as *const i8 (Misc)) = *const {l560} i8
                // 1827: b"lglmbt.c\0" a ... _char:   l560 = UNIQUE | NON_NULL, (empty)
                // 1827: b"lglmbt.c\0" a ... st u8: typeof(_49 = move _50 as *const u8 (Pointer(ArrayToPointer))) = *const {l559} u8
                // 1827: b"lglmbt.c\0" a ... st u8:   l559 = UNIQUE | NON_NULL, (empty)
                550 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                // 1829: (*::core::mem:: ... ptr(): typeof(_54) = *const {l88} i8
                // 1829: (*::core::mem:: ... ptr():   l88 = UNIQUE | NON_NULL, (empty)
                // 1829: (*::core::mem:: ... ptr(): typeof(_55) = & {l90} [i8]
                // 1829: (*::core::mem:: ... ptr():   l90 = UNIQUE | NON_NULL, FIXED
                // 1829: (*::core::mem:: ... ptr(): typeof(_56) = & {l92} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1829: (*::core::mem:: ... ptr():   l92 = UNIQUE | NON_NULL, (empty)
                // 1829: ::core::mem::tr ... 0", ): typeof(_57) = & {l94} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1829: ::core::mem::tr ... 0", ):   l94 = UNIQUE | NON_NULL, FIXED
                // 1829: (*::core::mem:: ... ptr(): typeof(_56 = &(*_57)) = & {l563} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1829: (*::core::mem:: ... ptr():   l563 = UNIQUE | NON_NULL, (empty)
                // 1829: (*::core::mem:: ... ptr(): typeof(_55 = move _56 as &[i8] (Pointer(Unsize))) = & {l564} [i8]
                // 1829: (*::core::mem:: ... ptr():   l564 = UNIQUE | NON_NULL, FIXED
                    b"void dd(Env *, const char *, int, int)\0",
                    // 1830: b"void dd(Env * ... t)\0": typeof(_58) = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1830: b"void dd(Env * ... t)\0":   l96 = UNIQUE | NON_NULL, (empty)
                    // 1830: b"void dd(Env * ... t)\0": typeof(_59) = & {l98} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1830: b"void dd(Env * ... t)\0":   l98 = UNIQUE | NON_NULL, FIXED
                    // 1830: b"void dd(Env * ... t)\0": typeof(_59 = const b"void dd(Env *, const char *, int, int)\x00") = & {l561} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1830: b"void dd(Env * ... t)\0":   l561 = UNIQUE | NON_NULL, (empty)
                    // 1830: b"void dd(Env * ... t)\0": typeof(_58 = &(*_59)) = & {l562} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1830: b"void dd(Env * ... t)\0":   l562 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    (*env_0).nevents = 0 as libc::c_int;
    while fscanf(
        file,
        // 1838: file: typeof(_65) = *mut {l105} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1838: file:   l105 = UNIQUE | NON_NULL, (empty)
        b"%p %x\n\0" as *const u8 as *const libc::c_char,
        // 1839: b"%p %x\n\0" as ... _char: typeof(_66) = *const {l107} i8
        // 1839: b"%p %x\n\0" as ... _char:   l107 = UNIQUE | NON_NULL, (empty)
        // 1839: b"%p %x\n\0" as ... st u8: typeof(_67) = *const {l109} u8
        // 1839: b"%p %x\n\0" as ... st u8:   l109 = UNIQUE | NON_NULL, (empty)
        // 1839: b"%p %x\n\0": typeof(_68) = *const {l111} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1839: b"%p %x\n\0":   l111 = UNIQUE | NON_NULL, (empty)
        // 1839: b"%p %x\n\0": typeof(_69) = & {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1839: b"%p %x\n\0":   l113 = UNIQUE | NON_NULL, FIXED
        // 1839: b"%p %x\n\0": typeof(_69 = const b"%p %x\n\x00") = & {l565} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1839: b"%p %x\n\0":   l565 = UNIQUE | NON_NULL, (empty)
        // 1839: b"%p %x\n\0" as ... _char: typeof(_66 = move _67 as *const i8 (Misc)) = *const {l568} i8
        // 1839: b"%p %x\n\0" as ... _char:   l568 = UNIQUE | NON_NULL, (empty)
        // 1839: b"%p %x\n\0": typeof(_68 = &raw const (*_69)) = *const {l566} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1839: b"%p %x\n\0":   l566 = UNIQUE | NON_NULL, (empty)
        // 1839: b"%p %x\n\0" as ... st u8: typeof(_67 = move _68 as *const u8 (Pointer(ArrayToPointer))) = *const {l567} u8
        // 1839: b"%p %x\n\0" as ... st u8:   l567 = UNIQUE | NON_NULL, (empty)
        &mut state as *mut State,
        // 1840: &mut state as * ... State: typeof(_70) = *mut {l115} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l116} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l117} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1840: &mut state as * ... State:   l115 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state as * ... State:   l116 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state as * ... State:   l117 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state: typeof(_71) = &mut {l119} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l120} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l121} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1840: &mut state:   l119 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state:   l120 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state:   l121 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state: typeof(_71 = &mut _6) = &mut {l569} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l570} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l571} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1840: &mut state:   l569 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state:   l570 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state:   l571 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state: typeof(_70 = &raw mut (*_71)) = *mut {l572} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l573} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l574} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1840: &mut state:   l572 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state:   l573 = UNIQUE | NON_NULL, (empty)
        // 1840: &mut state:   l574 = UNIQUE | NON_NULL, (empty)
        &mut rand as *mut libc::c_uint,
        // 1841: &mut rand as *m ... _uint: typeof(_72) = *mut {l123} u32
        // 1841: &mut rand as *m ... _uint:   l123 = UNIQUE | NON_NULL, (empty)
        // 1841: &mut rand: typeof(_73) = &mut {l125} u32
        // 1841: &mut rand:   l125 = UNIQUE | NON_NULL, (empty)
        // 1841: &mut rand: typeof(_73 = &mut _5) = &mut {l575} u32
        // 1841: &mut rand:   l575 = UNIQUE | NON_NULL, (empty)
        // 1841: &mut rand: typeof(_72 = &raw mut (*_73)) = *mut {l576} u32
        // 1841: &mut rand:   l576 = UNIQUE | NON_NULL, (empty)
    ) == 2 as libc::c_int
    {
        (*env_0).nevents += 1 as libc::c_int;
    }
    fclose(file);
    // 1846: file: typeof(_81) = *mut {l134} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1846: file:   l134 = UNIQUE | NON_NULL, (empty)
    (*env_0).events = calloc(
    // 1847: calloc( (*env_0 ... ng, ): typeof(_82) = *mut {l136} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1847: calloc( (*env_0 ... ng, ):   l136 = UNIQUE | NON_NULL, (empty)
    // 1847: (*env_0).events ... Event: typeof(((*_1).16: *mut Event) = move _82 as *mut Event (Misc)) = *mut {l577} DefId(0:314 ~ lglmbt[b165]::Event)
    // 1847: (*env_0).events ... Event:   l577 = UNIQUE | NON_NULL, (empty)
        (*env_0).nevents as libc::c_ulong,
        ::core::mem::size_of::<Event>() as libc::c_ulong,
    ) as *mut Event;
    file = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    // 1851: fopen(filename, ... char): typeof(_87) = *mut {l142} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1851: fopen(filename, ... char):   l142 = UNIQUE | NON_NULL, (empty)
    // 1851: filename: typeof(_88) = *const {l144} i8
    // 1851: filename:   l144 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0" as *cons ... _char: typeof(_89) = *const {l146} i8
    // 1851: b"r\0" as *cons ... _char:   l146 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0" as *const u8: typeof(_90) = *const {l148} u8
    // 1851: b"r\0" as *const u8:   l148 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0": typeof(_91) = *const {l150} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1851: b"r\0":   l150 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0": typeof(_92) = & {l152} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1851: b"r\0":   l152 = UNIQUE | NON_NULL, FIXED
    // 1851: b"r\0" as *cons ... _char: typeof(_89 = move _90 as *const i8 (Misc)) = *const {l581} i8
    // 1851: b"r\0" as *cons ... _char:   l581 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0": typeof(_91 = &raw const (*_92)) = *const {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1851: b"r\0":   l579 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0" as *const u8: typeof(_90 = move _91 as *const u8 (Pointer(ArrayToPointer))) = *const {l580} u8
    // 1851: b"r\0" as *const u8:   l580 = UNIQUE | NON_NULL, (empty)
    // 1851: b"r\0": typeof(_92 = const b"r\x00") = & {l578} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 1851: b"r\0":   l578 = UNIQUE | NON_NULL, (empty)
    if !file.is_null() {
    // 1852: file: typeof(_96) = *mut {l157} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1852: file:   l157 = UNIQUE | NON_NULL, (empty)
    } else {
        __assert_fail(
            b"file\0" as *const u8 as *const libc::c_char,
            // 1855: b"file\0" as *c ... _char: typeof(_99) = *const {l161} i8
            // 1855: b"file\0" as *c ... _char:   l161 = UNIQUE | NON_NULL, (empty)
            // 1855: b"file\0" as *c ... st u8: typeof(_100) = *const {l163} u8
            // 1855: b"file\0" as *c ... st u8:   l163 = UNIQUE | NON_NULL, (empty)
            // 1855: b"file\0": typeof(_101) = *const {l165} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1855: b"file\0":   l165 = UNIQUE | NON_NULL, (empty)
            // 1855: b"file\0": typeof(_102) = & {l167} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1855: b"file\0":   l167 = UNIQUE | NON_NULL, FIXED
            // 1855: b"file\0" as *c ... _char: typeof(_99 = move _100 as *const i8 (Misc)) = *const {l585} i8
            // 1855: b"file\0" as *c ... _char:   l585 = UNIQUE | NON_NULL, (empty)
            // 1855: b"file\0" as *c ... st u8: typeof(_100 = move _101 as *const u8 (Pointer(ArrayToPointer))) = *const {l584} u8
            // 1855: b"file\0" as *c ... st u8:   l584 = UNIQUE | NON_NULL, (empty)
            // 1855: b"file\0": typeof(_101 = &raw const (*_102)) = *const {l583} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1855: b"file\0":   l583 = UNIQUE | NON_NULL, (empty)
            // 1855: b"file\0": typeof(_102 = const b"file\x00") = & {l582} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
            // 1855: b"file\0":   l582 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1856: b"lglmbt.c\0" a ... _char: typeof(_103) = *const {l169} i8
            // 1856: b"lglmbt.c\0" a ... _char:   l169 = UNIQUE | NON_NULL, (empty)
            // 1856: b"lglmbt.c\0" a ... st u8: typeof(_104) = *const {l171} u8
            // 1856: b"lglmbt.c\0" a ... st u8:   l171 = UNIQUE | NON_NULL, (empty)
            // 1856: b"lglmbt.c\0": typeof(_105) = *const {l173} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1856: b"lglmbt.c\0":   l173 = UNIQUE | NON_NULL, (empty)
            // 1856: b"lglmbt.c\0": typeof(_106) = & {l175} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1856: b"lglmbt.c\0":   l175 = UNIQUE | NON_NULL, FIXED
            // 1856: b"lglmbt.c\0" a ... st u8: typeof(_104 = move _105 as *const u8 (Pointer(ArrayToPointer))) = *const {l588} u8
            // 1856: b"lglmbt.c\0" a ... st u8:   l588 = UNIQUE | NON_NULL, (empty)
            // 1856: b"lglmbt.c\0" a ... _char: typeof(_103 = move _104 as *const i8 (Misc)) = *const {l589} i8
            // 1856: b"lglmbt.c\0" a ... _char:   l589 = UNIQUE | NON_NULL, (empty)
            // 1856: b"lglmbt.c\0": typeof(_106 = const b"lglmbt.c\x00") = & {l586} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1856: b"lglmbt.c\0":   l586 = UNIQUE | NON_NULL, (empty)
            // 1856: b"lglmbt.c\0": typeof(_105 = &raw const (*_106)) = *const {l587} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1856: b"lglmbt.c\0":   l587 = UNIQUE | NON_NULL, (empty)
            557 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
            // 1858: (*::core::mem:: ... ptr(): typeof(_109) = *const {l179} i8
            // 1858: (*::core::mem:: ... ptr():   l179 = UNIQUE | NON_NULL, (empty)
            // 1858: (*::core::mem:: ... ptr(): typeof(_110) = & {l181} [i8]
            // 1858: (*::core::mem:: ... ptr():   l181 = UNIQUE | NON_NULL, FIXED
            // 1858: (*::core::mem:: ... ptr(): typeof(_111) = & {l183} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1858: (*::core::mem:: ... ptr():   l183 = UNIQUE | NON_NULL, (empty)
            // 1858: ::core::mem::tr ... 0", ): typeof(_112) = & {l185} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1858: ::core::mem::tr ... 0", ):   l185 = UNIQUE | NON_NULL, FIXED
            // 1858: (*::core::mem:: ... ptr(): typeof(_111 = &(*_112)) = & {l592} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1858: (*::core::mem:: ... ptr():   l592 = UNIQUE | NON_NULL, (empty)
            // 1858: (*::core::mem:: ... ptr(): typeof(_110 = move _111 as &[i8] (Pointer(Unsize))) = & {l593} [i8]
            // 1858: (*::core::mem:: ... ptr():   l593 = UNIQUE | NON_NULL, FIXED
                b"void dd(Env *, const char *, int, int)\0",
                // 1859: b"void dd(Env * ... t)\0": typeof(_113) = & {l187} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1859: b"void dd(Env * ... t)\0":   l187 = UNIQUE | NON_NULL, (empty)
                // 1859: b"void dd(Env * ... t)\0": typeof(_114) = & {l189} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1859: b"void dd(Env * ... t)\0":   l189 = UNIQUE | NON_NULL, FIXED
                // 1859: b"void dd(Env * ... t)\0": typeof(_114 = const b"void dd(Env *, const char *, int, int)\x00") = & {l590} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1859: b"void dd(Env * ... t)\0":   l590 = UNIQUE | NON_NULL, (empty)
                // 1859: b"void dd(Env * ... t)\0": typeof(_113 = &(*_114)) = & {l591} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1859: b"void dd(Env * ... t)\0":   l591 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_8721: {
        if !file.is_null() {
        // 1865: file: typeof(_118) = *mut {l194} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1865: file:   l194 = UNIQUE | NON_NULL, (empty)
        } else {
            __assert_fail(
                b"file\0" as *const u8 as *const libc::c_char,
                // 1868: b"file\0" as *c ... _char: typeof(_121) = *const {l198} i8
                // 1868: b"file\0" as *c ... _char:   l198 = UNIQUE | NON_NULL, (empty)
                // 1868: b"file\0" as *c ... st u8: typeof(_122) = *const {l200} u8
                // 1868: b"file\0" as *c ... st u8:   l200 = UNIQUE | NON_NULL, (empty)
                // 1868: b"file\0": typeof(_123) = *const {l202} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1868: b"file\0":   l202 = UNIQUE | NON_NULL, (empty)
                // 1868: b"file\0": typeof(_124) = & {l204} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1868: b"file\0":   l204 = UNIQUE | NON_NULL, FIXED
                // 1868: b"file\0": typeof(_123 = &raw const (*_124)) = *const {l595} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1868: b"file\0":   l595 = UNIQUE | NON_NULL, (empty)
                // 1868: b"file\0": typeof(_124 = const b"file\x00") = & {l594} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
                // 1868: b"file\0":   l594 = UNIQUE | NON_NULL, (empty)
                // 1868: b"file\0" as *c ... _char: typeof(_121 = move _122 as *const i8 (Misc)) = *const {l597} i8
                // 1868: b"file\0" as *c ... _char:   l597 = UNIQUE | NON_NULL, (empty)
                // 1868: b"file\0" as *c ... st u8: typeof(_122 = move _123 as *const u8 (Pointer(ArrayToPointer))) = *const {l596} u8
                // 1868: b"file\0" as *c ... st u8:   l596 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1869: b"lglmbt.c\0" a ... _char: typeof(_125) = *const {l206} i8
                // 1869: b"lglmbt.c\0" a ... _char:   l206 = UNIQUE | NON_NULL, (empty)
                // 1869: b"lglmbt.c\0" a ... st u8: typeof(_126) = *const {l208} u8
                // 1869: b"lglmbt.c\0" a ... st u8:   l208 = UNIQUE | NON_NULL, (empty)
                // 1869: b"lglmbt.c\0": typeof(_127) = *const {l210} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1869: b"lglmbt.c\0":   l210 = UNIQUE | NON_NULL, (empty)
                // 1869: b"lglmbt.c\0": typeof(_128) = & {l212} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1869: b"lglmbt.c\0":   l212 = UNIQUE | NON_NULL, FIXED
                // 1869: b"lglmbt.c\0": typeof(_128 = const b"lglmbt.c\x00") = & {l598} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1869: b"lglmbt.c\0":   l598 = UNIQUE | NON_NULL, (empty)
                // 1869: b"lglmbt.c\0" a ... _char: typeof(_125 = move _126 as *const i8 (Misc)) = *const {l601} i8
                // 1869: b"lglmbt.c\0" a ... _char:   l601 = UNIQUE | NON_NULL, (empty)
                // 1869: b"lglmbt.c\0": typeof(_127 = &raw const (*_128)) = *const {l599} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1869: b"lglmbt.c\0":   l599 = UNIQUE | NON_NULL, (empty)
                // 1869: b"lglmbt.c\0" a ... st u8: typeof(_126 = move _127 as *const u8 (Pointer(ArrayToPointer))) = *const {l600} u8
                // 1869: b"lglmbt.c\0" a ... st u8:   l600 = UNIQUE | NON_NULL, (empty)
                557 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                // 1871: (*::core::mem:: ... ptr(): typeof(_131) = *const {l216} i8
                // 1871: (*::core::mem:: ... ptr():   l216 = UNIQUE | NON_NULL, (empty)
                // 1871: (*::core::mem:: ... ptr(): typeof(_132) = & {l218} [i8]
                // 1871: (*::core::mem:: ... ptr():   l218 = UNIQUE | NON_NULL, FIXED
                // 1871: (*::core::mem:: ... ptr(): typeof(_133) = & {l220} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1871: (*::core::mem:: ... ptr():   l220 = UNIQUE | NON_NULL, (empty)
                // 1871: ::core::mem::tr ... 0", ): typeof(_134) = & {l222} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1871: ::core::mem::tr ... 0", ):   l222 = UNIQUE | NON_NULL, FIXED
                // 1871: (*::core::mem:: ... ptr(): typeof(_133 = &(*_134)) = & {l604} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1871: (*::core::mem:: ... ptr():   l604 = UNIQUE | NON_NULL, (empty)
                // 1871: (*::core::mem:: ... ptr(): typeof(_132 = move _133 as &[i8] (Pointer(Unsize))) = & {l605} [i8]
                // 1871: (*::core::mem:: ... ptr():   l605 = UNIQUE | NON_NULL, FIXED
                    b"void dd(Env *, const char *, int, int)\0",
                    // 1872: b"void dd(Env * ... t)\0": typeof(_135) = & {l224} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1872: b"void dd(Env * ... t)\0":   l224 = UNIQUE | NON_NULL, (empty)
                    // 1872: b"void dd(Env * ... t)\0": typeof(_136) = & {l226} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1872: b"void dd(Env * ... t)\0":   l226 = UNIQUE | NON_NULL, FIXED
                    // 1872: b"void dd(Env * ... t)\0": typeof(_135 = &(*_136)) = & {l603} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1872: b"void dd(Env * ... t)\0":   l603 = UNIQUE | NON_NULL, (empty)
                    // 1872: b"void dd(Env * ... t)\0": typeof(_136 = const b"void dd(Env *, const char *, int, int)\x00") = & {l602} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1872: b"void dd(Env * ... t)\0":   l602 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    i = 0 as libc::c_int;
    while fscanf(
        file,
        // 1880: file: typeof(_141) = *mut {l232} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1880: file:   l232 = UNIQUE | NON_NULL, (empty)
        b"%p %x\n\0" as *const u8 as *const libc::c_char,
        // 1881: b"%p %x\n\0" as ... _char: typeof(_142) = *const {l234} i8
        // 1881: b"%p %x\n\0" as ... _char:   l234 = UNIQUE | NON_NULL, (empty)
        // 1881: b"%p %x\n\0" as ... st u8: typeof(_143) = *const {l236} u8
        // 1881: b"%p %x\n\0" as ... st u8:   l236 = UNIQUE | NON_NULL, (empty)
        // 1881: b"%p %x\n\0": typeof(_144) = *const {l238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1881: b"%p %x\n\0":   l238 = UNIQUE | NON_NULL, (empty)
        // 1881: b"%p %x\n\0": typeof(_145) = & {l240} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1881: b"%p %x\n\0":   l240 = UNIQUE | NON_NULL, FIXED
        // 1881: b"%p %x\n\0": typeof(_145 = const b"%p %x\n\x00") = & {l606} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1881: b"%p %x\n\0":   l606 = UNIQUE | NON_NULL, (empty)
        // 1881: b"%p %x\n\0" as ... st u8: typeof(_143 = move _144 as *const u8 (Pointer(ArrayToPointer))) = *const {l608} u8
        // 1881: b"%p %x\n\0" as ... st u8:   l608 = UNIQUE | NON_NULL, (empty)
        // 1881: b"%p %x\n\0" as ... _char: typeof(_142 = move _143 as *const i8 (Misc)) = *const {l609} i8
        // 1881: b"%p %x\n\0" as ... _char:   l609 = UNIQUE | NON_NULL, (empty)
        // 1881: b"%p %x\n\0": typeof(_144 = &raw const (*_145)) = *const {l607} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
        // 1881: b"%p %x\n\0":   l607 = UNIQUE | NON_NULL, (empty)
        &mut state as *mut State,
        // 1882: &mut state as * ... State: typeof(_146) = *mut {l242} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l243} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l244} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1882: &mut state as * ... State:   l242 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state as * ... State:   l243 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state as * ... State:   l244 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state: typeof(_147) = &mut {l246} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l247} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l248} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1882: &mut state:   l246 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state:   l247 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state:   l248 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state: typeof(_146 = &raw mut (*_147)) = *mut {l613} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l614} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l615} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1882: &mut state:   l613 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state:   l614 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state:   l615 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state: typeof(_147 = &mut _6) = &mut {l610} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l611} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l612} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1882: &mut state:   l610 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state:   l611 = UNIQUE | NON_NULL, (empty)
        // 1882: &mut state:   l612 = UNIQUE | NON_NULL, (empty)
        &mut rand as *mut libc::c_uint,
        // 1883: &mut rand as *m ... _uint: typeof(_148) = *mut {l250} u32
        // 1883: &mut rand as *m ... _uint:   l250 = UNIQUE | NON_NULL, (empty)
        // 1883: &mut rand: typeof(_149) = &mut {l252} u32
        // 1883: &mut rand:   l252 = UNIQUE | NON_NULL, (empty)
        // 1883: &mut rand: typeof(_148 = &raw mut (*_149)) = *mut {l617} u32
        // 1883: &mut rand:   l617 = UNIQUE | NON_NULL, (empty)
        // 1883: &mut rand: typeof(_149 = &mut _5) = &mut {l616} u32
        // 1883: &mut rand:   l616 = UNIQUE | NON_NULL, (empty)
    ) == 2 as libc::c_int
    {
        if i < (*env_0).nevents {
        } else {
            __assert_fail(
                b"i < env->nevents\0" as *const u8 as *const libc::c_char,
                // 1889: b"i < env->neve ... _char: typeof(_157) = *const {l261} i8
                // 1889: b"i < env->neve ... _char:   l261 = UNIQUE | NON_NULL, (empty)
                // 1889: b"i < env->neve ... st u8: typeof(_158) = *const {l263} u8
                // 1889: b"i < env->neve ... st u8:   l263 = UNIQUE | NON_NULL, (empty)
                // 1889: b"i < env->neve ... ts\0": typeof(_159) = *const {l265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1889: b"i < env->neve ... ts\0":   l265 = UNIQUE | NON_NULL, (empty)
                // 1889: b"i < env->neve ... ts\0": typeof(_160) = & {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1889: b"i < env->neve ... ts\0":   l267 = UNIQUE | NON_NULL, FIXED
                // 1889: b"i < env->neve ... _char: typeof(_157 = move _158 as *const i8 (Misc)) = *const {l621} i8
                // 1889: b"i < env->neve ... _char:   l621 = UNIQUE | NON_NULL, (empty)
                // 1889: b"i < env->neve ... ts\0": typeof(_160 = const b"i < env->nevents\x00") = & {l618} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1889: b"i < env->neve ... ts\0":   l618 = UNIQUE | NON_NULL, (empty)
                // 1889: b"i < env->neve ... st u8: typeof(_158 = move _159 as *const u8 (Pointer(ArrayToPointer))) = *const {l620} u8
                // 1889: b"i < env->neve ... st u8:   l620 = UNIQUE | NON_NULL, (empty)
                // 1889: b"i < env->neve ... ts\0": typeof(_159 = &raw const (*_160)) = *const {l619} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 1889: b"i < env->neve ... ts\0":   l619 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1890: b"lglmbt.c\0" a ... _char: typeof(_161) = *const {l269} i8
                // 1890: b"lglmbt.c\0" a ... _char:   l269 = UNIQUE | NON_NULL, (empty)
                // 1890: b"lglmbt.c\0" a ... st u8: typeof(_162) = *const {l271} u8
                // 1890: b"lglmbt.c\0" a ... st u8:   l271 = UNIQUE | NON_NULL, (empty)
                // 1890: b"lglmbt.c\0": typeof(_163) = *const {l273} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1890: b"lglmbt.c\0":   l273 = UNIQUE | NON_NULL, (empty)
                // 1890: b"lglmbt.c\0": typeof(_164) = & {l275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1890: b"lglmbt.c\0":   l275 = UNIQUE | NON_NULL, FIXED
                // 1890: b"lglmbt.c\0": typeof(_163 = &raw const (*_164)) = *const {l623} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1890: b"lglmbt.c\0":   l623 = UNIQUE | NON_NULL, (empty)
                // 1890: b"lglmbt.c\0": typeof(_164 = const b"lglmbt.c\x00") = & {l622} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1890: b"lglmbt.c\0":   l622 = UNIQUE | NON_NULL, (empty)
                // 1890: b"lglmbt.c\0" a ... st u8: typeof(_162 = move _163 as *const u8 (Pointer(ArrayToPointer))) = *const {l624} u8
                // 1890: b"lglmbt.c\0" a ... st u8:   l624 = UNIQUE | NON_NULL, (empty)
                // 1890: b"lglmbt.c\0" a ... _char: typeof(_161 = move _162 as *const i8 (Misc)) = *const {l625} i8
                // 1890: b"lglmbt.c\0" a ... _char:   l625 = UNIQUE | NON_NULL, (empty)
                560 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                // 1892: (*::core::mem:: ... ptr(): typeof(_167) = *const {l279} i8
                // 1892: (*::core::mem:: ... ptr():   l279 = UNIQUE | NON_NULL, (empty)
                // 1892: (*::core::mem:: ... ptr(): typeof(_168) = & {l281} [i8]
                // 1892: (*::core::mem:: ... ptr():   l281 = UNIQUE | NON_NULL, FIXED
                // 1892: (*::core::mem:: ... ptr(): typeof(_169) = & {l283} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1892: (*::core::mem:: ... ptr():   l283 = UNIQUE | NON_NULL, (empty)
                // 1892: ::core::mem::tr ... 0", ): typeof(_170) = & {l285} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1892: ::core::mem::tr ... 0", ):   l285 = UNIQUE | NON_NULL, FIXED
                // 1892: (*::core::mem:: ... ptr(): typeof(_169 = &(*_170)) = & {l628} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1892: (*::core::mem:: ... ptr():   l628 = UNIQUE | NON_NULL, (empty)
                // 1892: (*::core::mem:: ... ptr(): typeof(_168 = move _169 as &[i8] (Pointer(Unsize))) = & {l629} [i8]
                // 1892: (*::core::mem:: ... ptr():   l629 = UNIQUE | NON_NULL, FIXED
                    b"void dd(Env *, const char *, int, int)\0",
                    // 1893: b"void dd(Env * ... t)\0": typeof(_171) = & {l287} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1893: b"void dd(Env * ... t)\0":   l287 = UNIQUE | NON_NULL, (empty)
                    // 1893: b"void dd(Env * ... t)\0": typeof(_172) = & {l289} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1893: b"void dd(Env * ... t)\0":   l289 = UNIQUE | NON_NULL, FIXED
                    // 1893: b"void dd(Env * ... t)\0": typeof(_172 = const b"void dd(Env *, const char *, int, int)\x00") = & {l626} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1893: b"void dd(Env * ... t)\0":   l626 = UNIQUE | NON_NULL, (empty)
                    // 1893: b"void dd(Env * ... t)\0": typeof(_171 = &(*_172)) = & {l627} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1893: b"void dd(Env * ... t)\0":   l627 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
        'c_8661: {
            if i < (*env_0).nevents {
            } else {
                __assert_fail(
                    b"i < env->nevents\0" as *const u8 as *const libc::c_char,
                    // 1902: b"i < env->neve ... _char: typeof(_179) = *const {l297} i8
                    // 1902: b"i < env->neve ... _char:   l297 = UNIQUE | NON_NULL, (empty)
                    // 1902: b"i < env->neve ... st u8: typeof(_180) = *const {l299} u8
                    // 1902: b"i < env->neve ... st u8:   l299 = UNIQUE | NON_NULL, (empty)
                    // 1902: b"i < env->neve ... ts\0": typeof(_181) = *const {l301} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 1902: b"i < env->neve ... ts\0":   l301 = UNIQUE | NON_NULL, (empty)
                    // 1902: b"i < env->neve ... ts\0": typeof(_182) = & {l303} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 1902: b"i < env->neve ... ts\0":   l303 = UNIQUE | NON_NULL, FIXED
                    // 1902: b"i < env->neve ... _char: typeof(_179 = move _180 as *const i8 (Misc)) = *const {l633} i8
                    // 1902: b"i < env->neve ... _char:   l633 = UNIQUE | NON_NULL, (empty)
                    // 1902: b"i < env->neve ... st u8: typeof(_180 = move _181 as *const u8 (Pointer(ArrayToPointer))) = *const {l632} u8
                    // 1902: b"i < env->neve ... st u8:   l632 = UNIQUE | NON_NULL, (empty)
                    // 1902: b"i < env->neve ... ts\0": typeof(_182 = const b"i < env->nevents\x00") = & {l630} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 1902: b"i < env->neve ... ts\0":   l630 = UNIQUE | NON_NULL, (empty)
                    // 1902: b"i < env->neve ... ts\0": typeof(_181 = &raw const (*_182)) = *const {l631} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 1902: b"i < env->neve ... ts\0":   l631 = UNIQUE | NON_NULL, (empty)
                    b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                    // 1903: b"lglmbt.c\0" a ... _char: typeof(_183) = *const {l305} i8
                    // 1903: b"lglmbt.c\0" a ... _char:   l305 = UNIQUE | NON_NULL, (empty)
                    // 1903: b"lglmbt.c\0" a ... st u8: typeof(_184) = *const {l307} u8
                    // 1903: b"lglmbt.c\0" a ... st u8:   l307 = UNIQUE | NON_NULL, (empty)
                    // 1903: b"lglmbt.c\0": typeof(_185) = *const {l309} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1903: b"lglmbt.c\0":   l309 = UNIQUE | NON_NULL, (empty)
                    // 1903: b"lglmbt.c\0": typeof(_186) = & {l311} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1903: b"lglmbt.c\0":   l311 = UNIQUE | NON_NULL, FIXED
                    // 1903: b"lglmbt.c\0" a ... _char: typeof(_183 = move _184 as *const i8 (Misc)) = *const {l637} i8
                    // 1903: b"lglmbt.c\0" a ... _char:   l637 = UNIQUE | NON_NULL, (empty)
                    // 1903: b"lglmbt.c\0": typeof(_186 = const b"lglmbt.c\x00") = & {l634} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1903: b"lglmbt.c\0":   l634 = UNIQUE | NON_NULL, (empty)
                    // 1903: b"lglmbt.c\0": typeof(_185 = &raw const (*_186)) = *const {l635} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                    // 1903: b"lglmbt.c\0":   l635 = UNIQUE | NON_NULL, (empty)
                    // 1903: b"lglmbt.c\0" a ... st u8: typeof(_184 = move _185 as *const u8 (Pointer(ArrayToPointer))) = *const {l636} u8
                    // 1903: b"lglmbt.c\0" a ... st u8:   l636 = UNIQUE | NON_NULL, (empty)
                    560 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                    // 1905: (*::core::mem:: ... ptr(): typeof(_189) = *const {l315} i8
                    // 1905: (*::core::mem:: ... ptr():   l315 = UNIQUE | NON_NULL, (empty)
                    // 1905: (*::core::mem:: ... ptr(): typeof(_190) = & {l317} [i8]
                    // 1905: (*::core::mem:: ... ptr():   l317 = UNIQUE | NON_NULL, FIXED
                    // 1905: (*::core::mem:: ... ptr(): typeof(_191) = & {l319} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1905: (*::core::mem:: ... ptr():   l319 = UNIQUE | NON_NULL, (empty)
                    // 1905: ::core::mem::tr ... 0", ): typeof(_192) = & {l321} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1905: ::core::mem::tr ... 0", ):   l321 = UNIQUE | NON_NULL, FIXED
                    // 1905: (*::core::mem:: ... ptr(): typeof(_191 = &(*_192)) = & {l640} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1905: (*::core::mem:: ... ptr():   l640 = UNIQUE | NON_NULL, (empty)
                    // 1905: (*::core::mem:: ... ptr(): typeof(_190 = move _191 as &[i8] (Pointer(Unsize))) = & {l641} [i8]
                    // 1905: (*::core::mem:: ... ptr():   l641 = UNIQUE | NON_NULL, FIXED
                        b"void dd(Env *, const char *, int, int)\0",
                        // 1906: b"void dd(Env * ... t)\0": typeof(_193) = & {l323} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1906: b"void dd(Env * ... t)\0":   l323 = UNIQUE | NON_NULL, (empty)
                        // 1906: b"void dd(Env * ... t)\0": typeof(_194) = & {l325} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1906: b"void dd(Env * ... t)\0":   l325 = UNIQUE | NON_NULL, FIXED
                        // 1906: b"void dd(Env * ... t)\0": typeof(_194 = const b"void dd(Env *, const char *, int, int)\x00") = & {l638} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1906: b"void dd(Env * ... t)\0":   l638 = UNIQUE | NON_NULL, (empty)
                        // 1906: b"void dd(Env * ... t)\0": typeof(_193 = &(*_194)) = & {l639} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                        // 1906: b"void dd(Env * ... t)\0":   l639 = UNIQUE | NON_NULL, (empty)
                    ))
                    .as_ptr(),
                );
            }
        };
        let ref mut fresh3 = (*((*env_0).events).offset(i as isize)).state;
        // 1912: ref mut fresh3: typeof(_195) = &mut {l327} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l328} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l329} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1912: ref mut fresh3:   l327 = UNIQUE | NON_NULL, FIXED
        // 1912: ref mut fresh3:   l328 = UNIQUE | NON_NULL, (empty)
        // 1912: ref mut fresh3:   l329 = UNIQUE | NON_NULL, (empty)
        // 1912: ((*env_0).event ... size): typeof(_196) = *mut {l331} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1912: ((*env_0).event ... size):   l331 = UNIQUE | NON_NULL, (empty)
        // 1912: ((*env_0).events): typeof(_197) = *mut {l333} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1912: ((*env_0).events):   l333 = UNIQUE | NON_NULL, (empty)
        // 1912: ref mut fresh3: typeof(_195 = &mut ((*_196).0: std::option::Option<unsafe extern "C" fn(*mut Data, u32) -> *mut libc::c_void>)) = &mut {l642} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l643} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l644} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1912: ref mut fresh3:   l642 = UNIQUE | NON_NULL, (empty)
        // 1912: ref mut fresh3:   l643 = UNIQUE | NON_NULL, (empty)
        // 1912: ref mut fresh3:   l644 = UNIQUE | NON_NULL, (empty)
        *fresh3 = state;
        // 1913: state: typeof(_200) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l337} DefId(0:299 ~ lglmbt[b165]::Data), u32) -> *mut {l338} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 1913: state:   l337 = UNIQUE | NON_NULL, (empty)
        // 1913: state:   l338 = UNIQUE | NON_NULL, (empty)
        (*((*env_0).events).offset(i as isize)).rand = rand;
        // 1914: ((*env_0).event ... size): typeof(_202) = *mut {l341} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1914: ((*env_0).event ... size):   l341 = UNIQUE | NON_NULL, (empty)
        // 1914: ((*env_0).events): typeof(_203) = *mut {l343} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1914: ((*env_0).events):   l343 = UNIQUE | NON_NULL, (empty)
        (*((*env_0).events).offset(i as isize)).remove = 0 as libc::c_int;
        // 1915: ((*env_0).event ... size): typeof(_207) = *mut {l348} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1915: ((*env_0).event ... size):   l348 = UNIQUE | NON_NULL, (empty)
        // 1915: ((*env_0).events): typeof(_208) = *mut {l350} DefId(0:314 ~ lglmbt[b165]::Event)
        // 1915: ((*env_0).events):   l350 = UNIQUE | NON_NULL, (empty)
        i += 1;
        i;
    }
    fclose(file);
    // 1919: file: typeof(_217) = *mut {l360} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1919: file:   l360 = UNIQUE | NON_NULL, (empty)
    if i == (*env_0).nevents {
    } else {
        __assert_fail(
            b"i == env->nevents\0" as *const u8 as *const libc::c_char,
            // 1923: b"i == env->nev ... _char: typeof(_224) = *const {l368} i8
            // 1923: b"i == env->nev ... _char:   l368 = UNIQUE | NON_NULL, (empty)
            // 1923: b"i == env->nev ... st u8: typeof(_225) = *const {l370} u8
            // 1923: b"i == env->nev ... st u8:   l370 = UNIQUE | NON_NULL, (empty)
            // 1923: b"i == env->nev ... ts\0": typeof(_226) = *const {l372} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1923: b"i == env->nev ... ts\0":   l372 = UNIQUE | NON_NULL, (empty)
            // 1923: b"i == env->nev ... ts\0": typeof(_227) = & {l374} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1923: b"i == env->nev ... ts\0":   l374 = UNIQUE | NON_NULL, FIXED
            // 1923: b"i == env->nev ... st u8: typeof(_225 = move _226 as *const u8 (Pointer(ArrayToPointer))) = *const {l647} u8
            // 1923: b"i == env->nev ... st u8:   l647 = UNIQUE | NON_NULL, (empty)
            // 1923: b"i == env->nev ... ts\0": typeof(_226 = &raw const (*_227)) = *const {l646} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1923: b"i == env->nev ... ts\0":   l646 = UNIQUE | NON_NULL, (empty)
            // 1923: b"i == env->nev ... _char: typeof(_224 = move _225 as *const i8 (Misc)) = *const {l648} i8
            // 1923: b"i == env->nev ... _char:   l648 = UNIQUE | NON_NULL, (empty)
            // 1923: b"i == env->nev ... ts\0": typeof(_227 = const b"i == env->nevents\x00") = & {l645} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 1923: b"i == env->nev ... ts\0":   l645 = UNIQUE | NON_NULL, (empty)
            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
            // 1924: b"lglmbt.c\0" a ... _char: typeof(_228) = *const {l376} i8
            // 1924: b"lglmbt.c\0" a ... _char:   l376 = UNIQUE | NON_NULL, (empty)
            // 1924: b"lglmbt.c\0" a ... st u8: typeof(_229) = *const {l378} u8
            // 1924: b"lglmbt.c\0" a ... st u8:   l378 = UNIQUE | NON_NULL, (empty)
            // 1924: b"lglmbt.c\0": typeof(_230) = *const {l380} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1924: b"lglmbt.c\0":   l380 = UNIQUE | NON_NULL, (empty)
            // 1924: b"lglmbt.c\0": typeof(_231) = & {l382} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1924: b"lglmbt.c\0":   l382 = UNIQUE | NON_NULL, FIXED
            // 1924: b"lglmbt.c\0": typeof(_230 = &raw const (*_231)) = *const {l650} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1924: b"lglmbt.c\0":   l650 = UNIQUE | NON_NULL, (empty)
            // 1924: b"lglmbt.c\0": typeof(_231 = const b"lglmbt.c\x00") = & {l649} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
            // 1924: b"lglmbt.c\0":   l649 = UNIQUE | NON_NULL, (empty)
            // 1924: b"lglmbt.c\0" a ... st u8: typeof(_229 = move _230 as *const u8 (Pointer(ArrayToPointer))) = *const {l651} u8
            // 1924: b"lglmbt.c\0" a ... st u8:   l651 = UNIQUE | NON_NULL, (empty)
            // 1924: b"lglmbt.c\0" a ... _char: typeof(_228 = move _229 as *const i8 (Misc)) = *const {l652} i8
            // 1924: b"lglmbt.c\0" a ... _char:   l652 = UNIQUE | NON_NULL, (empty)
            567 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
            // 1926: (*::core::mem:: ... ptr(): typeof(_234) = *const {l386} i8
            // 1926: (*::core::mem:: ... ptr():   l386 = UNIQUE | NON_NULL, (empty)
            // 1926: (*::core::mem:: ... ptr(): typeof(_235) = & {l388} [i8]
            // 1926: (*::core::mem:: ... ptr():   l388 = UNIQUE | NON_NULL, FIXED
            // 1926: (*::core::mem:: ... ptr(): typeof(_236) = & {l390} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1926: (*::core::mem:: ... ptr():   l390 = UNIQUE | NON_NULL, (empty)
            // 1926: ::core::mem::tr ... 0", ): typeof(_237) = & {l392} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1926: ::core::mem::tr ... 0", ):   l392 = UNIQUE | NON_NULL, FIXED
            // 1926: (*::core::mem:: ... ptr(): typeof(_235 = move _236 as &[i8] (Pointer(Unsize))) = & {l656} [i8]
            // 1926: (*::core::mem:: ... ptr():   l656 = UNIQUE | NON_NULL, FIXED
            // 1926: (*::core::mem:: ... ptr(): typeof(_236 = &(*_237)) = & {l655} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
            // 1926: (*::core::mem:: ... ptr():   l655 = UNIQUE | NON_NULL, (empty)
                b"void dd(Env *, const char *, int, int)\0",
                // 1927: b"void dd(Env * ... t)\0": typeof(_238) = & {l394} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1927: b"void dd(Env * ... t)\0":   l394 = UNIQUE | NON_NULL, (empty)
                // 1927: b"void dd(Env * ... t)\0": typeof(_239) = & {l396} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1927: b"void dd(Env * ... t)\0":   l396 = UNIQUE | NON_NULL, FIXED
                // 1927: b"void dd(Env * ... t)\0": typeof(_239 = const b"void dd(Env *, const char *, int, int)\x00") = & {l653} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1927: b"void dd(Env * ... t)\0":   l653 = UNIQUE | NON_NULL, (empty)
                // 1927: b"void dd(Env * ... t)\0": typeof(_238 = &(*_239)) = & {l654} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1927: b"void dd(Env * ... t)\0":   l654 = UNIQUE | NON_NULL, (empty)
            ))
            .as_ptr(),
        );
    }
    'c_8568: {
        if i == (*env_0).nevents {
        } else {
            __assert_fail(
                b"i == env->nevents\0" as *const u8 as *const libc::c_char,
                // 1936: b"i == env->nev ... _char: typeof(_246) = *const {l404} i8
                // 1936: b"i == env->nev ... _char:   l404 = UNIQUE | NON_NULL, (empty)
                // 1936: b"i == env->nev ... st u8: typeof(_247) = *const {l406} u8
                // 1936: b"i == env->nev ... st u8:   l406 = UNIQUE | NON_NULL, (empty)
                // 1936: b"i == env->nev ... ts\0": typeof(_248) = *const {l408} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 1936: b"i == env->nev ... ts\0":   l408 = UNIQUE | NON_NULL, (empty)
                // 1936: b"i == env->nev ... ts\0": typeof(_249) = & {l410} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 1936: b"i == env->nev ... ts\0":   l410 = UNIQUE | NON_NULL, FIXED
                // 1936: b"i == env->nev ... ts\0": typeof(_249 = const b"i == env->nevents\x00") = & {l657} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 1936: b"i == env->nev ... ts\0":   l657 = UNIQUE | NON_NULL, (empty)
                // 1936: b"i == env->nev ... _char: typeof(_246 = move _247 as *const i8 (Misc)) = *const {l660} i8
                // 1936: b"i == env->nev ... _char:   l660 = UNIQUE | NON_NULL, (empty)
                // 1936: b"i == env->nev ... st u8: typeof(_247 = move _248 as *const u8 (Pointer(ArrayToPointer))) = *const {l659} u8
                // 1936: b"i == env->nev ... st u8:   l659 = UNIQUE | NON_NULL, (empty)
                // 1936: b"i == env->nev ... ts\0": typeof(_248 = &raw const (*_249)) = *const {l658} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
                // 1936: b"i == env->nev ... ts\0":   l658 = UNIQUE | NON_NULL, (empty)
                b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                // 1937: b"lglmbt.c\0" a ... _char: typeof(_250) = *const {l412} i8
                // 1937: b"lglmbt.c\0" a ... _char:   l412 = UNIQUE | NON_NULL, (empty)
                // 1937: b"lglmbt.c\0" a ... st u8: typeof(_251) = *const {l414} u8
                // 1937: b"lglmbt.c\0" a ... st u8:   l414 = UNIQUE | NON_NULL, (empty)
                // 1937: b"lglmbt.c\0": typeof(_252) = *const {l416} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1937: b"lglmbt.c\0":   l416 = UNIQUE | NON_NULL, (empty)
                // 1937: b"lglmbt.c\0": typeof(_253) = & {l418} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1937: b"lglmbt.c\0":   l418 = UNIQUE | NON_NULL, FIXED
                // 1937: b"lglmbt.c\0": typeof(_253 = const b"lglmbt.c\x00") = & {l661} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1937: b"lglmbt.c\0":   l661 = UNIQUE | NON_NULL, (empty)
                // 1937: b"lglmbt.c\0" a ... st u8: typeof(_251 = move _252 as *const u8 (Pointer(ArrayToPointer))) = *const {l663} u8
                // 1937: b"lglmbt.c\0" a ... st u8:   l663 = UNIQUE | NON_NULL, (empty)
                // 1937: b"lglmbt.c\0": typeof(_252 = &raw const (*_253)) = *const {l662} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                // 1937: b"lglmbt.c\0":   l662 = UNIQUE | NON_NULL, (empty)
                // 1937: b"lglmbt.c\0" a ... _char: typeof(_250 = move _251 as *const i8 (Misc)) = *const {l664} i8
                // 1937: b"lglmbt.c\0" a ... _char:   l664 = UNIQUE | NON_NULL, (empty)
                567 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 39], &[libc::c_char; 39]>(
                // 1939: (*::core::mem:: ... ptr(): typeof(_256) = *const {l422} i8
                // 1939: (*::core::mem:: ... ptr():   l422 = UNIQUE | NON_NULL, (empty)
                // 1939: (*::core::mem:: ... ptr(): typeof(_257) = & {l424} [i8]
                // 1939: (*::core::mem:: ... ptr():   l424 = UNIQUE | NON_NULL, FIXED
                // 1939: (*::core::mem:: ... ptr(): typeof(_258) = & {l426} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1939: (*::core::mem:: ... ptr():   l426 = UNIQUE | NON_NULL, (empty)
                // 1939: ::core::mem::tr ... 0", ): typeof(_259) = & {l428} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1939: ::core::mem::tr ... 0", ):   l428 = UNIQUE | NON_NULL, FIXED
                // 1939: (*::core::mem:: ... ptr(): typeof(_258 = &(*_259)) = & {l667} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                // 1939: (*::core::mem:: ... ptr():   l667 = UNIQUE | NON_NULL, (empty)
                // 1939: (*::core::mem:: ... ptr(): typeof(_257 = move _258 as &[i8] (Pointer(Unsize))) = & {l668} [i8]
                // 1939: (*::core::mem:: ... ptr():   l668 = UNIQUE | NON_NULL, FIXED
                    b"void dd(Env *, const char *, int, int)\0",
                    // 1940: b"void dd(Env * ... t)\0": typeof(_260) = & {l430} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1940: b"void dd(Env * ... t)\0":   l430 = UNIQUE | NON_NULL, (empty)
                    // 1940: b"void dd(Env * ... t)\0": typeof(_261) = & {l432} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1940: b"void dd(Env * ... t)\0":   l432 = UNIQUE | NON_NULL, FIXED
                    // 1940: b"void dd(Env * ... t)\0": typeof(_261 = const b"void dd(Env *, const char *, int, int)\x00") = & {l665} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1940: b"void dd(Env * ... t)\0":   l665 = UNIQUE | NON_NULL, (empty)
                    // 1940: b"void dd(Env * ... t)\0": typeof(_260 = &(*_261)) = & {l666} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000027)) }]
                    // 1940: b"void dd(Env * ... t)\0":   l666 = UNIQUE | NON_NULL, (empty)
                ))
                .as_ptr(),
            );
        }
    };
    (*env_0).prefix = b"bug\0" as *const u8 as *const libc::c_char;
    // 1946: b"bug\0" as *co ... st u8: typeof(_262) = *const {l434} u8
    // 1946: b"bug\0" as *co ... st u8:   l434 = UNIQUE | NON_NULL, (empty)
    // 1946: b"bug\0": typeof(_263) = *const {l436} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1946: b"bug\0":   l436 = UNIQUE | NON_NULL, (empty)
    // 1946: b"bug\0": typeof(_264) = & {l438} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1946: b"bug\0":   l438 = UNIQUE | NON_NULL, FIXED
    // 1946: (*env_0).prefix ... _char: typeof(((*_1).1: *const i8) = move _262 as *const i8 (Misc)) = *const {l672} i8
    // 1946: (*env_0).prefix ... _char:   l672 = UNIQUE | NON_NULL, (empty)
    // 1946: b"bug\0": typeof(_263 = &raw const (*_264)) = *const {l670} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1946: b"bug\0":   l670 = UNIQUE | NON_NULL, (empty)
    // 1946: b"bug\0" as *co ... st u8: typeof(_262 = move _263 as *const u8 (Pointer(ArrayToPointer))) = *const {l671} u8
    // 1946: b"bug\0" as *co ... st u8:   l671 = UNIQUE | NON_NULL, (empty)
    // 1946: b"bug\0": typeof(_264 = const b"bug\x00") = & {l669} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1946: b"bug\0":   l669 = UNIQUE | NON_NULL, (empty)
    run(
        env_0,
        // 1948: env_0: typeof(_266) = *mut {l441} DefId(0:327 ~ lglmbt[b165]::Env)
        // 1948: env_0:   l441 = UNIQUE | NON_NULL, (empty)
        Some(printrace as unsafe extern "C" fn(*mut Env) -> ()),
        // 1949: Some(printrace  ... > ()): typeof(_267) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l443} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
        // 1949: Some(printrace  ... > ()):   l443 = UNIQUE | NON_NULL, (empty)
        // 1949: printrace as un ... -> (): typeof(_268) = fn(*mut {l445} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
        // 1949: printrace as un ... -> ():   l445 = UNIQUE | NON_NULL, (empty)
        // 1949: printrace: typeof(_268 = printrace as unsafe extern "C" fn(*mut Env) (Pointer(ReifyFnPointer))) = fn(*mut {l673} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
        // 1949: printrace:   l673 = UNIQUE | NON_NULL, (empty)
        // 1949: Some(printrace  ... > ()): typeof(_267 = std::option::Option::<unsafe extern "C" fn(*mut Env)>::Some(move _268)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l674} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
        // 1949: Some(printrace  ... > ()):   l674 = UNIQUE | NON_NULL, (empty)
    );
    prwc(env_0, b"bug\0" as *const u8 as *const libc::c_char);
    // 1951: env_0: typeof(_270) = *mut {l448} DefId(0:327 ~ lglmbt[b165]::Env)
    // 1951: env_0:   l448 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0" as *co ... _char: typeof(_271) = *const {l450} i8
    // 1951: b"bug\0" as *co ... _char:   l450 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0" as *co ... st u8: typeof(_272) = *const {l452} u8
    // 1951: b"bug\0" as *co ... st u8:   l452 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0": typeof(_273) = *const {l454} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1951: b"bug\0":   l454 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0": typeof(_274) = & {l456} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1951: b"bug\0":   l456 = UNIQUE | NON_NULL, FIXED
    // 1951: b"bug\0": typeof(_273 = &raw const (*_274)) = *const {l676} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1951: b"bug\0":   l676 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0" as *co ... _char: typeof(_271 = move _272 as *const i8 (Misc)) = *const {l678} i8
    // 1951: b"bug\0" as *co ... _char:   l678 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0" as *co ... st u8: typeof(_272 = move _273 as *const u8 (Pointer(ArrayToPointer))) = *const {l677} u8
    // 1951: b"bug\0" as *co ... st u8:   l677 = UNIQUE | NON_NULL, (empty)
    // 1951: b"bug\0": typeof(_274 = const b"bug\x00") = & {l675} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1951: b"bug\0":   l675 = UNIQUE | NON_NULL, (empty)
    cmd = malloc(100 as libc::c_int as libc::c_ulong) as *mut libc::c_char;
    // 1952: malloc(100 as l ... long): typeof(_275) = *mut {l458} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1952: malloc(100 as l ... long):   l458 = UNIQUE | NON_NULL, (empty)
    // 1952: cmd = malloc(10 ... _char: typeof(_8 = move _275 as *mut i8 (Misc)) = *mut {l679} i8
    // 1952: cmd = malloc(10 ... _char:   l679 = UNIQUE | NON_NULL, (empty)
    if (*env_0).nodd != 0 {
        sprintf(
            cmd,
            // 1955: cmd: typeof(_282) = *mut {l466} i8
            // 1955: cmd:   l466 = UNIQUE | NON_NULL, (empty)
            b"cp bug-%u.trace red-%u.trace\0" as *const u8 as *const libc::c_char,
            // 1956: b"cp bug-%u.tra ... _char: typeof(_283) = *const {l468} i8
            // 1956: b"cp bug-%u.tra ... _char:   l468 = UNIQUE | NON_NULL, (empty)
            // 1956: b"cp bug-%u.tra ... st u8: typeof(_284) = *const {l470} u8
            // 1956: b"cp bug-%u.tra ... st u8:   l470 = UNIQUE | NON_NULL, (empty)
            // 1956: b"cp bug-%u.tra ... ce\0": typeof(_285) = *const {l472} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 1956: b"cp bug-%u.tra ... ce\0":   l472 = UNIQUE | NON_NULL, (empty)
            // 1956: b"cp bug-%u.tra ... ce\0": typeof(_286) = & {l474} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 1956: b"cp bug-%u.tra ... ce\0":   l474 = UNIQUE | NON_NULL, FIXED
            // 1956: b"cp bug-%u.tra ... ce\0": typeof(_286 = const b"cp bug-%u.trace red-%u.trace\x00") = & {l680} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 1956: b"cp bug-%u.tra ... ce\0":   l680 = UNIQUE | NON_NULL, (empty)
            // 1956: b"cp bug-%u.tra ... _char: typeof(_283 = move _284 as *const i8 (Misc)) = *const {l683} i8
            // 1956: b"cp bug-%u.tra ... _char:   l683 = UNIQUE | NON_NULL, (empty)
            // 1956: b"cp bug-%u.tra ... ce\0": typeof(_285 = &raw const (*_286)) = *const {l681} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 1956: b"cp bug-%u.tra ... ce\0":   l681 = UNIQUE | NON_NULL, (empty)
            // 1956: b"cp bug-%u.tra ... st u8: typeof(_284 = move _285 as *const u8 (Pointer(ArrayToPointer))) = *const {l682} u8
            // 1956: b"cp bug-%u.tra ... st u8:   l682 = UNIQUE | NON_NULL, (empty)
            (*env_0).seed,
            (*env_0).seed,
        );
    } else {
        sprintf(
            cmd,
            // 1962: cmd: typeof(_290) = *mut {l479} i8
            // 1962: cmd:   l479 = UNIQUE | NON_NULL, (empty)
            b"./lglddtrace %s bug-%u.trace red-%u.trace\0" as *const u8 as *const libc::c_char,
            // 1963: b"./lglddtrace  ... _char: typeof(_291) = *const {l481} i8
            // 1963: b"./lglddtrace  ... _char:   l481 = UNIQUE | NON_NULL, (empty)
            // 1963: b"./lglddtrace  ... st u8: typeof(_292) = *const {l483} u8
            // 1963: b"./lglddtrace  ... st u8:   l483 = UNIQUE | NON_NULL, (empty)
            // 1963: b"./lglddtrace  ... ce\0": typeof(_293) = *const {l485} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 1963: b"./lglddtrace  ... ce\0":   l485 = UNIQUE | NON_NULL, (empty)
            // 1963: b"./lglddtrace  ... ce\0": typeof(_294) = & {l487} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 1963: b"./lglddtrace  ... ce\0":   l487 = UNIQUE | NON_NULL, FIXED
            // 1963: b"./lglddtrace  ... _char: typeof(_291 = move _292 as *const i8 (Misc)) = *const {l687} i8
            // 1963: b"./lglddtrace  ... _char:   l687 = UNIQUE | NON_NULL, (empty)
            // 1963: b"./lglddtrace  ... ce\0": typeof(_294 = const b"./lglddtrace %s bug-%u.trace red-%u.trace\x00") = & {l684} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 1963: b"./lglddtrace  ... ce\0":   l684 = UNIQUE | NON_NULL, (empty)
            // 1963: b"./lglddtrace  ... ce\0": typeof(_293 = &raw const (*_294)) = *const {l685} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
            // 1963: b"./lglddtrace  ... ce\0":   l685 = UNIQUE | NON_NULL, (empty)
            // 1963: b"./lglddtrace  ... st u8: typeof(_292 = move _293 as *const u8 (Pointer(ArrayToPointer))) = *const {l686} u8
            // 1963: b"./lglddtrace  ... st u8:   l686 = UNIQUE | NON_NULL, (empty)
            if opt != 0 {
            // 1964: if opt != 0 { b ... har }: typeof(_295) = *const {l489} i8
            // 1964: if opt != 0 { b ... har }:   l489 = UNIQUE | NON_NULL, (empty)
                b"-O\0" as *const u8 as *const libc::c_char
                // 1965: b"-O\0" as *const u8: typeof(_298) = *const {l493} u8
                // 1965: b"-O\0" as *const u8:   l493 = UNIQUE | NON_NULL, (empty)
                // 1965: b"-O\0": typeof(_299) = *const {l495} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1965: b"-O\0":   l495 = UNIQUE | NON_NULL, (empty)
                // 1965: b"-O\0": typeof(_300) = & {l497} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1965: b"-O\0":   l497 = UNIQUE | NON_NULL, FIXED
                // 1965: b"-O\0": typeof(_300 = const b"-O\x00") = & {l688} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1965: b"-O\0":   l688 = UNIQUE | NON_NULL, (empty)
                // 1965: b"-O\0" as *const u8: typeof(_298 = move _299 as *const u8 (Pointer(ArrayToPointer))) = *const {l690} u8
                // 1965: b"-O\0" as *const u8:   l690 = UNIQUE | NON_NULL, (empty)
                // 1965: b"-O\0" as *con ... _char: typeof(_295 = move _298 as *const i8 (Misc)) = *const {l691} i8
                // 1965: b"-O\0" as *con ... _char:   l691 = UNIQUE | NON_NULL, (empty)
                // 1965: b"-O\0": typeof(_299 = &raw const (*_300)) = *const {l689} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
                // 1965: b"-O\0":   l689 = UNIQUE | NON_NULL, (empty)
            } else {
                b"\0" as *const u8 as *const libc::c_char
                // 1967: b"\0" as *const u8: typeof(_301) = *const {l499} u8
                // 1967: b"\0" as *const u8:   l499 = UNIQUE | NON_NULL, (empty)
                // 1967: b"\0": typeof(_302) = *const {l501} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                // 1967: b"\0":   l501 = UNIQUE | NON_NULL, (empty)
                // 1967: b"\0": typeof(_303) = & {l503} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                // 1967: b"\0":   l503 = UNIQUE | NON_NULL, FIXED
                // 1967: b"\0": typeof(_303 = const b"\x00") = & {l692} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                // 1967: b"\0":   l692 = UNIQUE | NON_NULL, (empty)
                // 1967: b"\0" as *const u8: typeof(_301 = move _302 as *const u8 (Pointer(ArrayToPointer))) = *const {l694} u8
                // 1967: b"\0" as *const u8:   l694 = UNIQUE | NON_NULL, (empty)
                // 1967: b"\0" as *const ... _char: typeof(_295 = move _301 as *const i8 (Misc)) = *const {l695} i8
                // 1967: b"\0" as *const ... _char:   l695 = UNIQUE | NON_NULL, (empty)
                // 1967: b"\0": typeof(_302 = &raw const (*_303)) = *const {l693} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000001)) }]
                // 1967: b"\0":   l693 = UNIQUE | NON_NULL, (empty)
            },
            (*env_0).seed,
            (*env_0).seed,
        );
    }
    let mut tmp: libc::c_int = system(cmd);
    // 1973: cmd: typeof(_307) = *const {l508} i8
    // 1973: cmd:   l508 = UNIQUE | NON_NULL, (empty)
    // 1973: cmd: typeof(_308) = *mut {l510} i8
    // 1973: cmd:   l510 = UNIQUE | NON_NULL, (empty)
    // 1973: cmd: typeof(_307 = move _308 as *const i8 (Pointer(MutToConstPointer))) = *const {l696} i8
    // 1973: cmd:   l696 = UNIQUE | NON_NULL, (empty)
    prwc(env_0, b"red\0" as *const u8 as *const libc::c_char);
    // 1974: env_0: typeof(_310) = *mut {l513} DefId(0:327 ~ lglmbt[b165]::Env)
    // 1974: env_0:   l513 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0" as *co ... _char: typeof(_311) = *const {l515} i8
    // 1974: b"red\0" as *co ... _char:   l515 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0" as *co ... st u8: typeof(_312) = *const {l517} u8
    // 1974: b"red\0" as *co ... st u8:   l517 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0": typeof(_313) = *const {l519} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1974: b"red\0":   l519 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0": typeof(_314) = & {l521} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1974: b"red\0":   l521 = UNIQUE | NON_NULL, FIXED
    // 1974: b"red\0" as *co ... _char: typeof(_311 = move _312 as *const i8 (Misc)) = *const {l700} i8
    // 1974: b"red\0" as *co ... _char:   l700 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0" as *co ... st u8: typeof(_312 = move _313 as *const u8 (Pointer(ArrayToPointer))) = *const {l699} u8
    // 1974: b"red\0" as *co ... st u8:   l699 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0": typeof(_313 = &raw const (*_314)) = *const {l698} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1974: b"red\0":   l698 = UNIQUE | NON_NULL, (empty)
    // 1974: b"red\0": typeof(_314 = const b"red\x00") = & {l697} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 1974: b"red\0":   l697 = UNIQUE | NON_NULL, (empty)
    free(cmd as *mut libc::c_void);
    // 1975: cmd as *mut lib ... _void: typeof(_316) = *mut {l524} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1975: cmd as *mut lib ... _void:   l524 = UNIQUE | NON_NULL, (empty)
    // 1975: cmd: typeof(_317) = *mut {l526} i8
    // 1975: cmd:   l526 = UNIQUE | NON_NULL, (empty)
    // 1975: cmd as *mut lib ... _void: typeof(_316 = move _317 as *mut libc::c_void (Misc)) = *mut {l701} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1975: cmd as *mut lib ... _void:   l701 = UNIQUE | NON_NULL, (empty)
    free((*env_0).events as *mut libc::c_void);
    // 1976: (*env_0).events ... _void: typeof(_319) = *mut {l529} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1976: (*env_0).events ... _void:   l529 = UNIQUE | NON_NULL, (empty)
    // 1976: (*env_0).events: typeof(_320) = *mut {l531} DefId(0:314 ~ lglmbt[b165]::Event)
    // 1976: (*env_0).events:   l531 = UNIQUE | NON_NULL, (empty)
    // 1976: (*env_0).events ... _void: typeof(_319 = move _320 as *mut libc::c_void (Misc)) = *mut {l702} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 1976: (*env_0).events ... _void:   l702 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn hashmac() -> libc::c_uint {
    let mut file: *mut FILE = fopen(
    // 1979: mut file: typeof(_2) = *mut {l2} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1979: mut file:   l2 = UNIQUE | NON_NULL, (empty)
        b"/sys/class/net/eth0/address\0" as *const u8 as *const libc::c_char,
        // 1980: b"/sys/class/ne ... _char: typeof(_3) = *const {l4} i8
        // 1980: b"/sys/class/ne ... _char:   l4 = UNIQUE | NON_NULL, (empty)
        // 1980: b"/sys/class/ne ... st u8: typeof(_4) = *const {l6} u8
        // 1980: b"/sys/class/ne ... st u8:   l6 = UNIQUE | NON_NULL, (empty)
        // 1980: b"/sys/class/ne ... ss\0": typeof(_5) = *const {l8} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1980: b"/sys/class/ne ... ss\0":   l8 = UNIQUE | NON_NULL, (empty)
        // 1980: b"/sys/class/ne ... ss\0": typeof(_6) = & {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1980: b"/sys/class/ne ... ss\0":   l10 = UNIQUE | NON_NULL, FIXED
        // 1980: b"/sys/class/ne ... ss\0": typeof(_5 = &raw const (*_6)) = *const {l152} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1980: b"/sys/class/ne ... ss\0":   l152 = UNIQUE | NON_NULL, (empty)
        // 1980: b"/sys/class/ne ... _char: typeof(_3 = move _4 as *const i8 (Misc)) = *const {l154} i8
        // 1980: b"/sys/class/ne ... _char:   l154 = UNIQUE | NON_NULL, (empty)
        // 1980: b"/sys/class/ne ... ss\0": typeof(_6 = const b"/sys/class/net/eth0/address\x00") = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001c)) }]
        // 1980: b"/sys/class/ne ... ss\0":   l151 = UNIQUE | NON_NULL, (empty)
        // 1980: b"/sys/class/ne ... st u8: typeof(_4 = move _5 as *const u8 (Pointer(ArrayToPointer))) = *const {l153} u8
        // 1980: b"/sys/class/ne ... st u8:   l153 = UNIQUE | NON_NULL, (empty)
        b"r\0" as *const u8 as *const libc::c_char,
        // 1981: b"r\0" as *cons ... _char: typeof(_7) = *const {l12} i8
        // 1981: b"r\0" as *cons ... _char:   l12 = UNIQUE | NON_NULL, (empty)
        // 1981: b"r\0" as *const u8: typeof(_8) = *const {l14} u8
        // 1981: b"r\0" as *const u8:   l14 = UNIQUE | NON_NULL, (empty)
        // 1981: b"r\0": typeof(_9) = *const {l16} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1981: b"r\0":   l16 = UNIQUE | NON_NULL, (empty)
        // 1981: b"r\0": typeof(_10) = & {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1981: b"r\0":   l18 = UNIQUE | NON_NULL, FIXED
        // 1981: b"r\0" as *cons ... _char: typeof(_7 = move _8 as *const i8 (Misc)) = *const {l158} i8
        // 1981: b"r\0" as *cons ... _char:   l158 = UNIQUE | NON_NULL, (empty)
        // 1981: b"r\0": typeof(_9 = &raw const (*_10)) = *const {l156} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1981: b"r\0":   l156 = UNIQUE | NON_NULL, (empty)
        // 1981: b"r\0": typeof(_10 = const b"r\x00") = & {l155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 1981: b"r\0":   l155 = UNIQUE | NON_NULL, (empty)
        // 1981: b"r\0" as *const u8: typeof(_8 = move _9 as *const u8 (Pointer(ArrayToPointer))) = *const {l157} u8
        // 1981: b"r\0" as *const u8:   l157 = UNIQUE | NON_NULL, (empty)
    );
    let mut mac: [libc::c_uint; 6] = [0; 6];
    let mut res: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if file.is_null() {
    // 1985: file: typeof(_16) = *mut {l25} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 1985: file:   l25 = UNIQUE | NON_NULL, (empty)
        return 0 as libc::c_int as libc::c_uint;
    }
    if fscanf(
        file,
        // 1989: file: typeof(_22) = *mut {l32} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
        // 1989: file:   l32 = UNIQUE | NON_NULL, (empty)
        b"%02x:%02x:%02x:%02x:%02x:%02x\0" as *const u8 as *const libc::c_char,
        // 1990: b"%02x:%02x:%02 ... _char: typeof(_23) = *const {l34} i8
        // 1990: b"%02x:%02x:%02 ... _char:   l34 = UNIQUE | NON_NULL, (empty)
        // 1990: b"%02x:%02x:%02 ... st u8: typeof(_24) = *const {l36} u8
        // 1990: b"%02x:%02x:%02 ... st u8:   l36 = UNIQUE | NON_NULL, (empty)
        // 1990: b"%02x:%02x:%02 ... 2x\0": typeof(_25) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1990: b"%02x:%02x:%02 ... 2x\0":   l38 = UNIQUE | NON_NULL, (empty)
        // 1990: b"%02x:%02x:%02 ... 2x\0": typeof(_26) = & {l40} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1990: b"%02x:%02x:%02 ... 2x\0":   l40 = UNIQUE | NON_NULL, FIXED
        // 1990: b"%02x:%02x:%02 ... 2x\0": typeof(_25 = &raw const (*_26)) = *const {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1990: b"%02x:%02x:%02 ... 2x\0":   l160 = UNIQUE | NON_NULL, (empty)
        // 1990: b"%02x:%02x:%02 ... _char: typeof(_23 = move _24 as *const i8 (Misc)) = *const {l162} i8
        // 1990: b"%02x:%02x:%02 ... _char:   l162 = UNIQUE | NON_NULL, (empty)
        // 1990: b"%02x:%02x:%02 ... 2x\0": typeof(_26 = const b"%02x:%02x:%02x:%02x:%02x:%02x\x00") = & {l159} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
        // 1990: b"%02x:%02x:%02 ... 2x\0":   l159 = UNIQUE | NON_NULL, (empty)
        // 1990: b"%02x:%02x:%02 ... st u8: typeof(_24 = move _25 as *const u8 (Pointer(ArrayToPointer))) = *const {l161} u8
        // 1990: b"%02x:%02x:%02 ... st u8:   l161 = UNIQUE | NON_NULL, (empty)
        mac.as_mut_ptr().offset(0 as libc::c_int as isize),
        // 1991: mac.as_mut_ptr( ... size): typeof(_27) = *mut {l42} u32
        // 1991: mac.as_mut_ptr( ... size):   l42 = UNIQUE | NON_NULL, (empty)
        // 1991: mac.as_mut_ptr(): typeof(_28) = *mut {l44} u32
        // 1991: mac.as_mut_ptr():   l44 = UNIQUE | NON_NULL, (empty)
        // 1991: mac.as_mut_ptr(): typeof(_29) = &mut {l46} [u32]
        // 1991: mac.as_mut_ptr():   l46 = UNIQUE | NON_NULL, FIXED
        // 1991: mac.as_mut_ptr(): typeof(_30) = &mut {l48} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1991: mac.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
        // 1991: mac.as_mut_ptr(): typeof(_30 = &mut _11) = &mut {l163} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1991: mac.as_mut_ptr():   l163 = UNIQUE | NON_NULL, (empty)
        // 1991: mac.as_mut_ptr(): typeof(_29 = move _30 as &mut [u32] (Pointer(Unsize))) = &mut {l164} [u32]
        // 1991: mac.as_mut_ptr():   l164 = UNIQUE | NON_NULL, FIXED
        mac.as_mut_ptr().offset(1 as libc::c_int as isize),
        // 1992: mac.as_mut_ptr( ... size): typeof(_33) = *mut {l52} u32
        // 1992: mac.as_mut_ptr( ... size):   l52 = UNIQUE | NON_NULL, (empty)
        // 1992: mac.as_mut_ptr(): typeof(_34) = *mut {l54} u32
        // 1992: mac.as_mut_ptr():   l54 = UNIQUE | NON_NULL, (empty)
        // 1992: mac.as_mut_ptr(): typeof(_35) = &mut {l56} [u32]
        // 1992: mac.as_mut_ptr():   l56 = UNIQUE | NON_NULL, FIXED
        // 1992: mac.as_mut_ptr(): typeof(_36) = &mut {l58} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1992: mac.as_mut_ptr():   l58 = UNIQUE | NON_NULL, (empty)
        // 1992: mac.as_mut_ptr(): typeof(_35 = move _36 as &mut [u32] (Pointer(Unsize))) = &mut {l166} [u32]
        // 1992: mac.as_mut_ptr():   l166 = UNIQUE | NON_NULL, FIXED
        // 1992: mac.as_mut_ptr(): typeof(_36 = &mut _11) = &mut {l165} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1992: mac.as_mut_ptr():   l165 = UNIQUE | NON_NULL, (empty)
        mac.as_mut_ptr().offset(2 as libc::c_int as isize),
        // 1993: mac.as_mut_ptr( ... size): typeof(_39) = *mut {l62} u32
        // 1993: mac.as_mut_ptr( ... size):   l62 = UNIQUE | NON_NULL, (empty)
        // 1993: mac.as_mut_ptr(): typeof(_40) = *mut {l64} u32
        // 1993: mac.as_mut_ptr():   l64 = UNIQUE | NON_NULL, (empty)
        // 1993: mac.as_mut_ptr(): typeof(_41) = &mut {l66} [u32]
        // 1993: mac.as_mut_ptr():   l66 = UNIQUE | NON_NULL, FIXED
        // 1993: mac.as_mut_ptr(): typeof(_42) = &mut {l68} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1993: mac.as_mut_ptr():   l68 = UNIQUE | NON_NULL, (empty)
        // 1993: mac.as_mut_ptr(): typeof(_42 = &mut _11) = &mut {l167} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1993: mac.as_mut_ptr():   l167 = UNIQUE | NON_NULL, (empty)
        // 1993: mac.as_mut_ptr(): typeof(_41 = move _42 as &mut [u32] (Pointer(Unsize))) = &mut {l168} [u32]
        // 1993: mac.as_mut_ptr():   l168 = UNIQUE | NON_NULL, FIXED
        mac.as_mut_ptr().offset(3 as libc::c_int as isize),
        // 1994: mac.as_mut_ptr( ... size): typeof(_45) = *mut {l72} u32
        // 1994: mac.as_mut_ptr( ... size):   l72 = UNIQUE | NON_NULL, (empty)
        // 1994: mac.as_mut_ptr(): typeof(_46) = *mut {l74} u32
        // 1994: mac.as_mut_ptr():   l74 = UNIQUE | NON_NULL, (empty)
        // 1994: mac.as_mut_ptr(): typeof(_47) = &mut {l76} [u32]
        // 1994: mac.as_mut_ptr():   l76 = UNIQUE | NON_NULL, FIXED
        // 1994: mac.as_mut_ptr(): typeof(_48) = &mut {l78} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1994: mac.as_mut_ptr():   l78 = UNIQUE | NON_NULL, (empty)
        // 1994: mac.as_mut_ptr(): typeof(_47 = move _48 as &mut [u32] (Pointer(Unsize))) = &mut {l170} [u32]
        // 1994: mac.as_mut_ptr():   l170 = UNIQUE | NON_NULL, FIXED
        // 1994: mac.as_mut_ptr(): typeof(_48 = &mut _11) = &mut {l169} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1994: mac.as_mut_ptr():   l169 = UNIQUE | NON_NULL, (empty)
        mac.as_mut_ptr().offset(4 as libc::c_int as isize),
        // 1995: mac.as_mut_ptr( ... size): typeof(_51) = *mut {l82} u32
        // 1995: mac.as_mut_ptr( ... size):   l82 = UNIQUE | NON_NULL, (empty)
        // 1995: mac.as_mut_ptr(): typeof(_52) = *mut {l84} u32
        // 1995: mac.as_mut_ptr():   l84 = UNIQUE | NON_NULL, (empty)
        // 1995: mac.as_mut_ptr(): typeof(_53) = &mut {l86} [u32]
        // 1995: mac.as_mut_ptr():   l86 = UNIQUE | NON_NULL, FIXED
        // 1995: mac.as_mut_ptr(): typeof(_54) = &mut {l88} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1995: mac.as_mut_ptr():   l88 = UNIQUE | NON_NULL, (empty)
        // 1995: mac.as_mut_ptr(): typeof(_54 = &mut _11) = &mut {l171} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1995: mac.as_mut_ptr():   l171 = UNIQUE | NON_NULL, (empty)
        // 1995: mac.as_mut_ptr(): typeof(_53 = move _54 as &mut [u32] (Pointer(Unsize))) = &mut {l172} [u32]
        // 1995: mac.as_mut_ptr():   l172 = UNIQUE | NON_NULL, FIXED
        mac.as_mut_ptr().offset(5 as libc::c_int as isize),
        // 1996: mac.as_mut_ptr( ... size): typeof(_57) = *mut {l92} u32
        // 1996: mac.as_mut_ptr( ... size):   l92 = UNIQUE | NON_NULL, (empty)
        // 1996: mac.as_mut_ptr(): typeof(_58) = *mut {l94} u32
        // 1996: mac.as_mut_ptr():   l94 = UNIQUE | NON_NULL, (empty)
        // 1996: mac.as_mut_ptr(): typeof(_59) = &mut {l96} [u32]
        // 1996: mac.as_mut_ptr():   l96 = UNIQUE | NON_NULL, FIXED
        // 1996: mac.as_mut_ptr(): typeof(_60) = &mut {l98} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1996: mac.as_mut_ptr():   l98 = UNIQUE | NON_NULL, (empty)
        // 1996: mac.as_mut_ptr(): typeof(_59 = move _60 as &mut [u32] (Pointer(Unsize))) = &mut {l174} [u32]
        // 1996: mac.as_mut_ptr():   l174 = UNIQUE | NON_NULL, FIXED
        // 1996: mac.as_mut_ptr(): typeof(_60 = &mut _11) = &mut {l173} [u32; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 1996: mac.as_mut_ptr():   l173 = UNIQUE | NON_NULL, (empty)
    ) == 6 as libc::c_int
    {
        res = mac[5 as libc::c_int as usize];
        res ^= mac[4 as libc::c_int as usize] << 4 as libc::c_int;
        res ^= mac[3 as libc::c_int as usize] << 8 as libc::c_int;
        res ^= mac[2 as libc::c_int as usize] << 16 as libc::c_int;
        res ^= mac[1 as libc::c_int as usize] << 20 as libc::c_int;
        res ^= mac[0 as libc::c_int as usize] << 24 as libc::c_int;
    }
    fclose(file);
    // 2006: file: typeof(_110) = *mut {l149} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2006: file:   l149 = UNIQUE | NON_NULL, (empty)
    return res;
}
static usage: *const libc::c_char = b"usage: lglmbt [ <option> ... ] [ <seed> ]\n\nwhere <option> is one of the following:\n\n  -k | --keep-lines\n  -q | --quiet\n  -f | --first-bug-only\n  -n | --no-delta-debugging\n  -a | --always-fork\n  -O | --optimize-by-delta-debugging-options\n\n  -m <maxruns>\n\0"
    as *const u8 as *const libc::c_char;
static env: Env = Env {
    file: 0 as *const FILE as *mut FILE,
    prefix: 0 as *const libc::c_char,
    round: 0,
    bugs: 0,
    violations: 0,
    print: 0,
    quiet: 0,
    first: 0,
    nodd: 0,
    noptsfuzz: 0,
    alwaysfork: 0,
    terminal: 0,
    forked: 0,
    nevents: 0,
    timeout: 0,
    seed: 0,
    events: 0 as *const Event as *mut Event,
    rng: RNG { z: 0, w: 0 },
};
static start: libc::c_double = 0.;
unsafe extern "C" fn currentime() -> libc::c_double {
    let mut res: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 {
    // 2038: &mut tv: typeof(_8) = *mut {l8} DefId(0:285 ~ lglmbt[b165]::timeval)
    // 2038: &mut tv:   l8 = UNIQUE | NON_NULL, (empty)
    // 2038: &mut tv: typeof(_9) = &mut {l10} DefId(0:285 ~ lglmbt[b165]::timeval)
    // 2038: &mut tv:   l10 = UNIQUE | NON_NULL, (empty)
    // 2038: 0 as *mut libc: ... _void: typeof(_10) = *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2038: 0 as *mut libc: ... _void:   l12 = UNIQUE | NON_NULL, (empty)
    // 2038: &mut tv: typeof(_9 = &mut _4) = &mut {l18} DefId(0:285 ~ lglmbt[b165]::timeval)
    // 2038: &mut tv:   l18 = UNIQUE | NON_NULL, (empty)
    // 2038: 0 as *mut libc: ... _void: typeof(_10 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2038: 0 as *mut libc: ... _void:   l20 = UNIQUE | NON_NULL, (empty)
    // 2038: &mut tv: typeof(_8 = &raw mut (*_9)) = *mut {l19} DefId(0:285 ~ lglmbt[b165]::timeval)
    // 2038: &mut tv:   l19 = UNIQUE | NON_NULL, (empty)
        res = 1e-6f64 * tv.tv_usec as libc::c_double;
        res += tv.tv_sec as libc::c_double;
    }
    return res;
}
unsafe extern "C" fn getime() -> libc::c_double {
    return currentime() - start;
    // 2045: start: typeof(_4) = *mut {l4} f64
    // 2045: start:   l4 = READ | UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn average(mut a: libc::c_double, mut b: libc::c_double) -> libc::c_double {
    return if b != 0. {
        a / b
    } else {
        0 as libc::c_int as libc::c_double
    };
}
unsafe extern "C" fn stats() {
    let mut t: libc::c_double = getime();
    let mut valid: libc::c_int = env.round - env.violations;
    // 2056: env: typeof(_4) = *mut {l4} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2056: env:   l4 = UNIQUE | NON_NULL, (empty)
    // 2056: env: typeof(_6) = *mut {l7} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2056: env:   l7 = UNIQUE | NON_NULL, (empty)
    printf(
        b"[lglmbt] finished after %.2f seconds\n\0" as *const u8 as *const libc::c_char,
        // 2058: b"[lglmbt] fini ... _char: typeof(_9) = *const {l11} i8
        // 2058: b"[lglmbt] fini ... _char:   l11 = UNIQUE | NON_NULL, (empty)
        // 2058: b"[lglmbt] fini ... st u8: typeof(_10) = *const {l13} u8
        // 2058: b"[lglmbt] fini ... st u8:   l13 = UNIQUE | NON_NULL, (empty)
        // 2058: b"[lglmbt] fini ... \n\0": typeof(_11) = *const {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 2058: b"[lglmbt] fini ... \n\0":   l15 = UNIQUE | NON_NULL, (empty)
        // 2058: b"[lglmbt] fini ... \n\0": typeof(_12) = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 2058: b"[lglmbt] fini ... \n\0":   l17 = UNIQUE | NON_NULL, FIXED
        // 2058: b"[lglmbt] fini ... _char: typeof(_9 = move _10 as *const i8 (Misc)) = *const {l91} i8
        // 2058: b"[lglmbt] fini ... _char:   l91 = UNIQUE | NON_NULL, (empty)
        // 2058: b"[lglmbt] fini ... st u8: typeof(_10 = move _11 as *const u8 (Pointer(ArrayToPointer))) = *const {l90} u8
        // 2058: b"[lglmbt] fini ... st u8:   l90 = UNIQUE | NON_NULL, (empty)
        // 2058: b"[lglmbt] fini ... \n\0": typeof(_12 = const b"[lglmbt] finished after %.2f seconds\n\x00") = & {l88} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 2058: b"[lglmbt] fini ... \n\0":   l88 = UNIQUE | NON_NULL, (empty)
        // 2058: b"[lglmbt] fini ... \n\0": typeof(_11 = &raw const (*_12)) = *const {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
        // 2058: b"[lglmbt] fini ... \n\0":   l89 = UNIQUE | NON_NULL, (empty)
        t,
    );
    printf(
        b"[lglmbt] %d rounds = %.0f rounds per second\n\0" as *const u8 as *const libc::c_char,
        // 2062: b"[lglmbt] %d r ... _char: typeof(_15) = *const {l21} i8
        // 2062: b"[lglmbt] %d r ... _char:   l21 = UNIQUE | NON_NULL, (empty)
        // 2062: b"[lglmbt] %d r ... st u8: typeof(_16) = *const {l23} u8
        // 2062: b"[lglmbt] %d r ... st u8:   l23 = UNIQUE | NON_NULL, (empty)
        // 2062: b"[lglmbt] %d r ... \n\0": typeof(_17) = *const {l25} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 2062: b"[lglmbt] %d r ... \n\0":   l25 = UNIQUE | NON_NULL, (empty)
        // 2062: b"[lglmbt] %d r ... \n\0": typeof(_18) = & {l27} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 2062: b"[lglmbt] %d r ... \n\0":   l27 = UNIQUE | NON_NULL, FIXED
        // 2062: b"[lglmbt] %d r ... \n\0": typeof(_18 = const b"[lglmbt] %d rounds = %.0f rounds per second\n\x00") = & {l92} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 2062: b"[lglmbt] %d r ... \n\0":   l92 = UNIQUE | NON_NULL, (empty)
        // 2062: b"[lglmbt] %d r ... _char: typeof(_15 = move _16 as *const i8 (Misc)) = *const {l95} i8
        // 2062: b"[lglmbt] %d r ... _char:   l95 = UNIQUE | NON_NULL, (empty)
        // 2062: b"[lglmbt] %d r ... \n\0": typeof(_17 = &raw const (*_18)) = *const {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002d)) }]
        // 2062: b"[lglmbt] %d r ... \n\0":   l93 = UNIQUE | NON_NULL, (empty)
        // 2062: b"[lglmbt] %d r ... st u8: typeof(_16 = move _17 as *const u8 (Pointer(ArrayToPointer))) = *const {l94} u8
        // 2062: b"[lglmbt] %d r ... st u8:   l94 = UNIQUE | NON_NULL, (empty)
        env.round,
        // 2063: env: typeof(_20) = *mut {l30} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2063: env:   l30 = UNIQUE | NON_NULL, (empty)
        average(env.round as libc::c_double, t),
        // 2064: env: typeof(_24) = *mut {l35} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2064: env:   l35 = UNIQUE | NON_NULL, (empty)
    );
    printf(
        b"[lglmbt] %d violations = %.0f rounds per second\n\0" as *const u8 as *const libc::c_char,
        // 2067: b"[lglmbt] %d v ... _char: typeof(_27) = *const {l39} i8
        // 2067: b"[lglmbt] %d v ... _char:   l39 = UNIQUE | NON_NULL, (empty)
        // 2067: b"[lglmbt] %d v ... st u8: typeof(_28) = *const {l41} u8
        // 2067: b"[lglmbt] %d v ... st u8:   l41 = UNIQUE | NON_NULL, (empty)
        // 2067: b"[lglmbt] %d v ... \n\0": typeof(_29) = *const {l43} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2067: b"[lglmbt] %d v ... \n\0":   l43 = UNIQUE | NON_NULL, (empty)
        // 2067: b"[lglmbt] %d v ... \n\0": typeof(_30) = & {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2067: b"[lglmbt] %d v ... \n\0":   l45 = UNIQUE | NON_NULL, FIXED
        // 2067: b"[lglmbt] %d v ... st u8: typeof(_28 = move _29 as *const u8 (Pointer(ArrayToPointer))) = *const {l98} u8
        // 2067: b"[lglmbt] %d v ... st u8:   l98 = UNIQUE | NON_NULL, (empty)
        // 2067: b"[lglmbt] %d v ... \n\0": typeof(_30 = const b"[lglmbt] %d violations = %.0f rounds per second\n\x00") = & {l96} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2067: b"[lglmbt] %d v ... \n\0":   l96 = UNIQUE | NON_NULL, (empty)
        // 2067: b"[lglmbt] %d v ... \n\0": typeof(_29 = &raw const (*_30)) = *const {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2067: b"[lglmbt] %d v ... \n\0":   l97 = UNIQUE | NON_NULL, (empty)
        // 2067: b"[lglmbt] %d v ... _char: typeof(_27 = move _28 as *const i8 (Misc)) = *const {l99} i8
        // 2067: b"[lglmbt] %d v ... _char:   l99 = UNIQUE | NON_NULL, (empty)
        env.violations,
        // 2068: env: typeof(_32) = *mut {l48} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2068: env:   l48 = UNIQUE | NON_NULL, (empty)
        average(env.violations as libc::c_double, t),
        // 2069: env: typeof(_36) = *mut {l53} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2069: env:   l53 = UNIQUE | NON_NULL, (empty)
    );
    printf(
        b"[lglmbt] %d valid runs = %.0f rounds per second\n\0" as *const u8 as *const libc::c_char,
        // 2072: b"[lglmbt] %d v ... _char: typeof(_39) = *const {l57} i8
        // 2072: b"[lglmbt] %d v ... _char:   l57 = UNIQUE | NON_NULL, (empty)
        // 2072: b"[lglmbt] %d v ... st u8: typeof(_40) = *const {l59} u8
        // 2072: b"[lglmbt] %d v ... st u8:   l59 = UNIQUE | NON_NULL, (empty)
        // 2072: b"[lglmbt] %d v ... \n\0": typeof(_41) = *const {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2072: b"[lglmbt] %d v ... \n\0":   l61 = UNIQUE | NON_NULL, (empty)
        // 2072: b"[lglmbt] %d v ... \n\0": typeof(_42) = & {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2072: b"[lglmbt] %d v ... \n\0":   l63 = UNIQUE | NON_NULL, FIXED
        // 2072: b"[lglmbt] %d v ... \n\0": typeof(_42 = const b"[lglmbt] %d valid runs = %.0f rounds per second\n\x00") = & {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2072: b"[lglmbt] %d v ... \n\0":   l100 = UNIQUE | NON_NULL, (empty)
        // 2072: b"[lglmbt] %d v ... _char: typeof(_39 = move _40 as *const i8 (Misc)) = *const {l103} i8
        // 2072: b"[lglmbt] %d v ... _char:   l103 = UNIQUE | NON_NULL, (empty)
        // 2072: b"[lglmbt] %d v ... \n\0": typeof(_41 = &raw const (*_42)) = *const {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000031)) }]
        // 2072: b"[lglmbt] %d v ... \n\0":   l101 = UNIQUE | NON_NULL, (empty)
        // 2072: b"[lglmbt] %d v ... st u8: typeof(_40 = move _41 as *const u8 (Pointer(ArrayToPointer))) = *const {l102} u8
        // 2072: b"[lglmbt] %d v ... st u8:   l102 = UNIQUE | NON_NULL, (empty)
        valid,
        average(valid as libc::c_double, t),
    );
    printf(
        b"[lglmbt] %d bugs = %.0f bugs per second\n\0" as *const u8 as *const libc::c_char,
        // 2077: b"[lglmbt] %d b ... _char: typeof(_49) = *const {l71} i8
        // 2077: b"[lglmbt] %d b ... _char:   l71 = UNIQUE | NON_NULL, (empty)
        // 2077: b"[lglmbt] %d b ... st u8: typeof(_50) = *const {l73} u8
        // 2077: b"[lglmbt] %d b ... st u8:   l73 = UNIQUE | NON_NULL, (empty)
        // 2077: b"[lglmbt] %d b ... \n\0": typeof(_51) = *const {l75} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 2077: b"[lglmbt] %d b ... \n\0":   l75 = UNIQUE | NON_NULL, (empty)
        // 2077: b"[lglmbt] %d b ... \n\0": typeof(_52) = & {l77} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 2077: b"[lglmbt] %d b ... \n\0":   l77 = UNIQUE | NON_NULL, FIXED
        // 2077: b"[lglmbt] %d b ... \n\0": typeof(_51 = &raw const (*_52)) = *const {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 2077: b"[lglmbt] %d b ... \n\0":   l105 = UNIQUE | NON_NULL, (empty)
        // 2077: b"[lglmbt] %d b ... st u8: typeof(_50 = move _51 as *const u8 (Pointer(ArrayToPointer))) = *const {l106} u8
        // 2077: b"[lglmbt] %d b ... st u8:   l106 = UNIQUE | NON_NULL, (empty)
        // 2077: b"[lglmbt] %d b ... \n\0": typeof(_52 = const b"[lglmbt] %d bugs = %.0f bugs per second\n\x00") = & {l104} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 2077: b"[lglmbt] %d b ... \n\0":   l104 = UNIQUE | NON_NULL, (empty)
        // 2077: b"[lglmbt] %d b ... _char: typeof(_49 = move _50 as *const i8 (Misc)) = *const {l107} i8
        // 2077: b"[lglmbt] %d b ... _char:   l107 = UNIQUE | NON_NULL, (empty)
        env.bugs,
        // 2078: env: typeof(_54) = *mut {l80} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2078: env:   l80 = UNIQUE | NON_NULL, (empty)
        average(env.bugs as libc::c_double, t),
        // 2079: env: typeof(_58) = *mut {l85} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2079: env:   l85 = UNIQUE | NON_NULL, (empty)
    );
}
unsafe extern "C" fn sighandler(mut sig: libc::c_int) {
    fflush(stdout);
    // 2083: stdout: typeof(_4) = *mut {l4} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2083: stdout:   l4 = UNIQUE | NON_NULL, (empty)
    // 2083: stdout: typeof(_5) = *mut {l6} *mut {l7} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2083: stdout:   l6 = UNIQUE | NON_NULL, (empty)
    // 2083: stdout:   l7 = UNIQUE | NON_NULL, (empty)
    fflush(stderr);
    // 2084: stderr: typeof(_7) = *mut {l10} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2084: stderr:   l10 = UNIQUE | NON_NULL, (empty)
    // 2084: stderr: typeof(_8) = *mut {l12} *mut {l13} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2084: stderr:   l12 = UNIQUE | NON_NULL, (empty)
    // 2084: stderr:   l13 = UNIQUE | NON_NULL, (empty)
    printf(
        b"*** lglmbt: caught signal %d in round %d\n\0" as *const u8 as *const libc::c_char,
        // 2086: b"*** lglmbt: c ... _char: typeof(_10) = *const {l16} i8
        // 2086: b"*** lglmbt: c ... _char:   l16 = UNIQUE | NON_NULL, (empty)
        // 2086: b"*** lglmbt: c ... st u8: typeof(_11) = *const {l18} u8
        // 2086: b"*** lglmbt: c ... st u8:   l18 = UNIQUE | NON_NULL, (empty)
        // 2086: b"*** lglmbt: c ... \n\0": typeof(_12) = *const {l20} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 2086: b"*** lglmbt: c ... \n\0":   l20 = UNIQUE | NON_NULL, (empty)
        // 2086: b"*** lglmbt: c ... \n\0": typeof(_13) = & {l22} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 2086: b"*** lglmbt: c ... \n\0":   l22 = UNIQUE | NON_NULL, FIXED
        // 2086: b"*** lglmbt: c ... \n\0": typeof(_13 = const b"*** lglmbt: caught signal %d in round %d\n\x00") = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 2086: b"*** lglmbt: c ... \n\0":   l37 = UNIQUE | NON_NULL, (empty)
        // 2086: b"*** lglmbt: c ... _char: typeof(_10 = move _11 as *const i8 (Misc)) = *const {l40} i8
        // 2086: b"*** lglmbt: c ... _char:   l40 = UNIQUE | NON_NULL, (empty)
        // 2086: b"*** lglmbt: c ... \n\0": typeof(_12 = &raw const (*_13)) = *const {l38} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002a)) }]
        // 2086: b"*** lglmbt: c ... \n\0":   l38 = UNIQUE | NON_NULL, (empty)
        // 2086: b"*** lglmbt: c ... st u8: typeof(_11 = move _12 as *const u8 (Pointer(ArrayToPointer))) = *const {l39} u8
        // 2086: b"*** lglmbt: c ... st u8:   l39 = UNIQUE | NON_NULL, (empty)
        sig,
        env.round,
        // 2088: env: typeof(_16) = *mut {l26} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2088: env:   l26 = UNIQUE | NON_NULL, (empty)
    );
    fflush(stdout);
    // 2090: stdout: typeof(_18) = *mut {l29} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2090: stdout:   l29 = UNIQUE | NON_NULL, (empty)
    // 2090: stdout: typeof(_19) = *mut {l31} *mut {l32} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
    // 2090: stdout:   l31 = UNIQUE | NON_NULL, (empty)
    // 2090: stdout:   l32 = UNIQUE | NON_NULL, (empty)
    stats();
    exit(1 as libc::c_int);
}
unsafe extern "C" fn setsighandlers() {
    signal(
        2 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        11 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        6 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    signal(
        15 as libc::c_int,
        Some(sighandler as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
// 2112: mut argv: typeof(_2) = *mut {g40} *mut {g41} i8
// 2112: mut argv:   g40 = UNIQUE | NON_NULL, FIXED
// 2112: mut argv:   g41 = UNIQUE | NON_NULL, FIXED
    let mut i: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut tmp: libc::c_int = 0;
    let mut prev: libc::c_int = 0;
    let mut opt: libc::c_int = 0;
    let mut mac: libc::c_int = 0;
    let mut pid: libc::c_int = 0;
    start = currentime();
    // 2121: start: typeof(_13) = *mut {l13} f64
    // 2121: start:   l13 = UNIQUE | NON_NULL, (empty)
    memset(
    // 2122: memset( &mut en ... ng, ): typeof(_14) = *mut {l15} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2122: memset( &mut en ... ng, ):   l15 = UNIQUE | NON_NULL, (empty)
        &mut env as *mut Env as *mut libc::c_void,
        // 2123: &mut env as *mu ... _void: typeof(_15) = *mut {l17} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2123: &mut env as *mu ... _void:   l17 = UNIQUE | NON_NULL, (empty)
        // 2123: &mut env as *mut Env: typeof(_16) = *mut {l19} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2123: &mut env as *mut Env:   l19 = UNIQUE | NON_NULL, (empty)
        // 2123: &mut env: typeof(_17) = &mut {l21} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2123: &mut env:   l21 = UNIQUE | NON_NULL, (empty)
        // 2123: env: typeof(_18) = *mut {l23} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2123: env:   l23 = UNIQUE | NON_NULL, (empty)
        // 2123: &mut env as *mu ... _void: typeof(_15 = move _16 as *mut libc::c_void (Misc)) = *mut {l1070} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2123: &mut env as *mu ... _void:   l1070 = UNIQUE | NON_NULL, (empty)
        // 2123: &mut env: typeof(_16 = &raw mut (*_17)) = *mut {l1069} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2123: &mut env:   l1069 = UNIQUE | NON_NULL, (empty)
        // 2123: &mut env: typeof(_17 = &mut (*_18)) = &mut {l1068} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2123: &mut env:   l1068 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
        ::core::mem::size_of::<Env>() as libc::c_ulong,
    );
    max = 2147483647 as libc::c_int;
    prev = 1 as libc::c_int;
    memset(
    // 2129: memset( &mut en ... ng, ): typeof(_24) = *mut {l30} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 2129: memset( &mut en ... ng, ):   l30 = UNIQUE | NON_NULL, (empty)
        &mut env as *mut Env as *mut libc::c_void,
        // 2130: &mut env as *mu ... _void: typeof(_25) = *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2130: &mut env as *mu ... _void:   l32 = UNIQUE | NON_NULL, (empty)
        // 2130: &mut env as *mut Env: typeof(_26) = *mut {l34} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2130: &mut env as *mut Env:   l34 = UNIQUE | NON_NULL, (empty)
        // 2130: &mut env: typeof(_27) = &mut {l36} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2130: &mut env:   l36 = UNIQUE | NON_NULL, (empty)
        // 2130: env: typeof(_28) = *mut {l38} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2130: env:   l38 = UNIQUE | NON_NULL, (empty)
        // 2130: &mut env: typeof(_26 = &raw mut (*_27)) = *mut {l1072} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2130: &mut env:   l1072 = UNIQUE | NON_NULL, (empty)
        // 2130: &mut env: typeof(_27 = &mut (*_28)) = &mut {l1071} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2130: &mut env:   l1071 = UNIQUE | NON_NULL, (empty)
        // 2130: &mut env as *mu ... _void: typeof(_25 = move _26 as *mut libc::c_void (Misc)) = *mut {l1073} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 2130: &mut env as *mu ... _void:   l1073 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
        ::core::mem::size_of::<Env>() as libc::c_ulong,
    );
    env.seed = -(1 as libc::c_int) as libc::c_uint;
    // 2134: env: typeof(_35) = *mut {l46} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2134: env:   l46 = UNIQUE | NON_NULL, (empty)
    env.terminal = isatty(1 as libc::c_int);
    // 2135: env: typeof(_38) = *mut {l50} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2135: env:   l50 = UNIQUE | NON_NULL, (empty)
    env.timeout = 0 as libc::c_int;
    // 2136: env: typeof(_40) = *mut {l53} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2136: env:   l53 = UNIQUE | NON_NULL, (empty)
    opt = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            // 2141: *argv.offset(i  ... size): typeof(_51) = *const {l65} i8
            // 2141: *argv.offset(i  ... size):   l65 = UNIQUE | NON_NULL, (empty)
            // 2141: *argv.offset(i  ... size): typeof(_52) = *mut {l67} i8
            // 2141: *argv.offset(i  ... size):   l67 = UNIQUE | NON_NULL, (empty)
            // 2141: argv.offset(i a ... size): typeof(_53) = *mut {l69} *mut {l70} i8
            // 2141: argv.offset(i a ... size):   l69 = UNIQUE | NON_NULL, (empty)
            // 2141: argv.offset(i a ... size):   l70 = UNIQUE | NON_NULL, (empty)
            // 2141: argv: typeof(_54) = *mut {l72} *mut {l73} i8
            // 2141: argv:   l72 = UNIQUE | NON_NULL, (empty)
            // 2141: argv:   l73 = UNIQUE | NON_NULL, (empty)
            // 2141: *argv.offset(i  ... size): typeof(_51 = move _52 as *const i8 (Pointer(MutToConstPointer))) = *const {l1074} i8
            // 2141: *argv.offset(i  ... size):   l1074 = UNIQUE | NON_NULL, (empty)
            b"-h\0" as *const u8 as *const libc::c_char,
            // 2142: b"-h\0" as *con ... _char: typeof(_57) = *const {l77} i8
            // 2142: b"-h\0" as *con ... _char:   l77 = UNIQUE | NON_NULL, (empty)
            // 2142: b"-h\0" as *const u8: typeof(_58) = *const {l79} u8
            // 2142: b"-h\0" as *const u8:   l79 = UNIQUE | NON_NULL, (empty)
            // 2142: b"-h\0": typeof(_59) = *const {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2142: b"-h\0":   l81 = UNIQUE | NON_NULL, (empty)
            // 2142: b"-h\0": typeof(_60) = & {l83} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2142: b"-h\0":   l83 = UNIQUE | NON_NULL, FIXED
            // 2142: b"-h\0": typeof(_60 = const b"-h\x00") = & {l1075} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2142: b"-h\0":   l1075 = UNIQUE | NON_NULL, (empty)
            // 2142: b"-h\0" as *con ... _char: typeof(_57 = move _58 as *const i8 (Misc)) = *const {l1078} i8
            // 2142: b"-h\0" as *con ... _char:   l1078 = UNIQUE | NON_NULL, (empty)
            // 2142: b"-h\0": typeof(_59 = &raw const (*_60)) = *const {l1076} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2142: b"-h\0":   l1076 = UNIQUE | NON_NULL, (empty)
            // 2142: b"-h\0" as *const u8: typeof(_58 = move _59 as *const u8 (Pointer(ArrayToPointer))) = *const {l1077} u8
            // 2142: b"-h\0" as *const u8:   l1077 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            printf(b"%s\0" as *const u8 as *const libc::c_char, usage);
            // 2145: b"%s\0" as *con ... _char: typeof(_63) = *const {l87} i8
            // 2145: b"%s\0" as *con ... _char:   l87 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0" as *const u8: typeof(_64) = *const {l89} u8
            // 2145: b"%s\0" as *const u8:   l89 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0": typeof(_65) = *const {l91} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"%s\0":   l91 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0": typeof(_66) = & {l93} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"%s\0":   l93 = UNIQUE | NON_NULL, FIXED
            // 2145: usage: typeof(_67) = *const {l95} i8
            // 2145: usage:   l95 = UNIQUE | NON_NULL, (empty)
            // 2145: usage: typeof(_68) = *mut {l97} *const {l98} i8
            // 2145: usage:   l97 = UNIQUE | NON_NULL, (empty)
            // 2145: usage:   l98 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0": typeof(_65 = &raw const (*_66)) = *const {l1080} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"%s\0":   l1080 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0": typeof(_66 = const b"%s\x00") = & {l1079} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2145: b"%s\0":   l1079 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0" as *con ... _char: typeof(_63 = move _64 as *const i8 (Misc)) = *const {l1082} i8
            // 2145: b"%s\0" as *con ... _char:   l1082 = UNIQUE | NON_NULL, (empty)
            // 2145: b"%s\0" as *const u8: typeof(_64 = move _65 as *const u8 (Pointer(ArrayToPointer))) = *const {l1081} u8
            // 2145: b"%s\0" as *const u8:   l1081 = UNIQUE | NON_NULL, (empty)
            exit(0 as libc::c_int);
        } else if strcmp(
            *argv.offset(i as isize),
            // 2148: *argv.offset(i  ... size): typeof(_74) = *const {l105} i8
            // 2148: *argv.offset(i  ... size):   l105 = UNIQUE | NON_NULL, (empty)
            // 2148: *argv.offset(i  ... size): typeof(_75) = *mut {l107} i8
            // 2148: *argv.offset(i  ... size):   l107 = UNIQUE | NON_NULL, (empty)
            // 2148: argv.offset(i a ... size): typeof(_76) = *mut {l109} *mut {l110} i8
            // 2148: argv.offset(i a ... size):   l109 = UNIQUE | NON_NULL, (empty)
            // 2148: argv.offset(i a ... size):   l110 = UNIQUE | NON_NULL, (empty)
            // 2148: argv: typeof(_77) = *mut {l112} *mut {l113} i8
            // 2148: argv:   l112 = UNIQUE | NON_NULL, (empty)
            // 2148: argv:   l113 = UNIQUE | NON_NULL, (empty)
            // 2148: *argv.offset(i  ... size): typeof(_74 = move _75 as *const i8 (Pointer(MutToConstPointer))) = *const {l1083} i8
            // 2148: *argv.offset(i  ... size):   l1083 = UNIQUE | NON_NULL, (empty)
            b"-k\0" as *const u8 as *const libc::c_char,
            // 2149: b"-k\0" as *con ... _char: typeof(_80) = *const {l117} i8
            // 2149: b"-k\0" as *con ... _char:   l117 = UNIQUE | NON_NULL, (empty)
            // 2149: b"-k\0" as *const u8: typeof(_81) = *const {l119} u8
            // 2149: b"-k\0" as *const u8:   l119 = UNIQUE | NON_NULL, (empty)
            // 2149: b"-k\0": typeof(_82) = *const {l121} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2149: b"-k\0":   l121 = UNIQUE | NON_NULL, (empty)
            // 2149: b"-k\0": typeof(_83) = & {l123} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2149: b"-k\0":   l123 = UNIQUE | NON_NULL, FIXED
            // 2149: b"-k\0" as *const u8: typeof(_81 = move _82 as *const u8 (Pointer(ArrayToPointer))) = *const {l1086} u8
            // 2149: b"-k\0" as *const u8:   l1086 = UNIQUE | NON_NULL, (empty)
            // 2149: b"-k\0": typeof(_83 = const b"-k\x00") = & {l1084} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2149: b"-k\0":   l1084 = UNIQUE | NON_NULL, (empty)
            // 2149: b"-k\0": typeof(_82 = &raw const (*_83)) = *const {l1085} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2149: b"-k\0":   l1085 = UNIQUE | NON_NULL, (empty)
            // 2149: b"-k\0" as *con ... _char: typeof(_80 = move _81 as *const i8 (Misc)) = *const {l1087} i8
            // 2149: b"-k\0" as *con ... _char:   l1087 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2152: *argv.offset(i  ... size): typeof(_86) = *const {l127} i8
                // 2152: *argv.offset(i  ... size):   l127 = UNIQUE | NON_NULL, (empty)
                // 2152: *argv.offset(i  ... size): typeof(_87) = *mut {l129} i8
                // 2152: *argv.offset(i  ... size):   l129 = UNIQUE | NON_NULL, (empty)
                // 2152: argv.offset(i a ... size): typeof(_88) = *mut {l131} *mut {l132} i8
                // 2152: argv.offset(i a ... size):   l131 = UNIQUE | NON_NULL, (empty)
                // 2152: argv.offset(i a ... size):   l132 = UNIQUE | NON_NULL, (empty)
                // 2152: argv: typeof(_89) = *mut {l134} *mut {l135} i8
                // 2152: argv:   l134 = UNIQUE | NON_NULL, (empty)
                // 2152: argv:   l135 = UNIQUE | NON_NULL, (empty)
                // 2152: *argv.offset(i  ... size): typeof(_86 = move _87 as *const i8 (Pointer(MutToConstPointer))) = *const {l1088} i8
                // 2152: *argv.offset(i  ... size):   l1088 = UNIQUE | NON_NULL, (empty)
                b"--keep-lines\0" as *const u8 as *const libc::c_char,
                // 2153: b"--keep-lines\ ... _char: typeof(_92) = *const {l139} i8
                // 2153: b"--keep-lines\ ... _char:   l139 = UNIQUE | NON_NULL, (empty)
                // 2153: b"--keep-lines\ ... st u8: typeof(_93) = *const {l141} u8
                // 2153: b"--keep-lines\ ... st u8:   l141 = UNIQUE | NON_NULL, (empty)
                // 2153: b"--keep-lines\0": typeof(_94) = *const {l143} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 2153: b"--keep-lines\0":   l143 = UNIQUE | NON_NULL, (empty)
                // 2153: b"--keep-lines\0": typeof(_95) = & {l145} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 2153: b"--keep-lines\0":   l145 = UNIQUE | NON_NULL, FIXED
                // 2153: b"--keep-lines\ ... st u8: typeof(_93 = move _94 as *const u8 (Pointer(ArrayToPointer))) = *const {l1091} u8
                // 2153: b"--keep-lines\ ... st u8:   l1091 = UNIQUE | NON_NULL, (empty)
                // 2153: b"--keep-lines\0": typeof(_95 = const b"--keep-lines\x00") = & {l1089} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 2153: b"--keep-lines\0":   l1089 = UNIQUE | NON_NULL, (empty)
                // 2153: b"--keep-lines\ ... _char: typeof(_92 = move _93 as *const i8 (Misc)) = *const {l1092} i8
                // 2153: b"--keep-lines\ ... _char:   l1092 = UNIQUE | NON_NULL, (empty)
                // 2153: b"--keep-lines\0": typeof(_94 = &raw const (*_95)) = *const {l1090} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
                // 2153: b"--keep-lines\0":   l1090 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            env.terminal = 0 as libc::c_int;
            // 2156: env: typeof(_97) = *mut {l148} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2156: env:   l148 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2158: *argv.offset(i  ... size): typeof(_101) = *const {l153} i8
            // 2158: *argv.offset(i  ... size):   l153 = UNIQUE | NON_NULL, (empty)
            // 2158: *argv.offset(i  ... size): typeof(_102) = *mut {l155} i8
            // 2158: *argv.offset(i  ... size):   l155 = UNIQUE | NON_NULL, (empty)
            // 2158: argv.offset(i a ... size): typeof(_103) = *mut {l157} *mut {l158} i8
            // 2158: argv.offset(i a ... size):   l157 = UNIQUE | NON_NULL, (empty)
            // 2158: argv.offset(i a ... size):   l158 = UNIQUE | NON_NULL, (empty)
            // 2158: argv: typeof(_104) = *mut {l160} *mut {l161} i8
            // 2158: argv:   l160 = UNIQUE | NON_NULL, (empty)
            // 2158: argv:   l161 = UNIQUE | NON_NULL, (empty)
            // 2158: *argv.offset(i  ... size): typeof(_101 = move _102 as *const i8 (Pointer(MutToConstPointer))) = *const {l1093} i8
            // 2158: *argv.offset(i  ... size):   l1093 = UNIQUE | NON_NULL, (empty)
            b"-q\0" as *const u8 as *const libc::c_char,
            // 2159: b"-q\0" as *con ... _char: typeof(_107) = *const {l165} i8
            // 2159: b"-q\0" as *con ... _char:   l165 = UNIQUE | NON_NULL, (empty)
            // 2159: b"-q\0" as *const u8: typeof(_108) = *const {l167} u8
            // 2159: b"-q\0" as *const u8:   l167 = UNIQUE | NON_NULL, (empty)
            // 2159: b"-q\0": typeof(_109) = *const {l169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2159: b"-q\0":   l169 = UNIQUE | NON_NULL, (empty)
            // 2159: b"-q\0": typeof(_110) = & {l171} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2159: b"-q\0":   l171 = UNIQUE | NON_NULL, FIXED
            // 2159: b"-q\0": typeof(_109 = &raw const (*_110)) = *const {l1095} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2159: b"-q\0":   l1095 = UNIQUE | NON_NULL, (empty)
            // 2159: b"-q\0" as *const u8: typeof(_108 = move _109 as *const u8 (Pointer(ArrayToPointer))) = *const {l1096} u8
            // 2159: b"-q\0" as *const u8:   l1096 = UNIQUE | NON_NULL, (empty)
            // 2159: b"-q\0" as *con ... _char: typeof(_107 = move _108 as *const i8 (Misc)) = *const {l1097} i8
            // 2159: b"-q\0" as *con ... _char:   l1097 = UNIQUE | NON_NULL, (empty)
            // 2159: b"-q\0": typeof(_110 = const b"-q\x00") = & {l1094} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2159: b"-q\0":   l1094 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2162: *argv.offset(i  ... size): typeof(_113) = *const {l175} i8
                // 2162: *argv.offset(i  ... size):   l175 = UNIQUE | NON_NULL, (empty)
                // 2162: *argv.offset(i  ... size): typeof(_114) = *mut {l177} i8
                // 2162: *argv.offset(i  ... size):   l177 = UNIQUE | NON_NULL, (empty)
                // 2162: argv.offset(i a ... size): typeof(_115) = *mut {l179} *mut {l180} i8
                // 2162: argv.offset(i a ... size):   l179 = UNIQUE | NON_NULL, (empty)
                // 2162: argv.offset(i a ... size):   l180 = UNIQUE | NON_NULL, (empty)
                // 2162: argv: typeof(_116) = *mut {l182} *mut {l183} i8
                // 2162: argv:   l182 = UNIQUE | NON_NULL, (empty)
                // 2162: argv:   l183 = UNIQUE | NON_NULL, (empty)
                // 2162: *argv.offset(i  ... size): typeof(_113 = move _114 as *const i8 (Pointer(MutToConstPointer))) = *const {l1098} i8
                // 2162: *argv.offset(i  ... size):   l1098 = UNIQUE | NON_NULL, (empty)
                b"--quiet\0" as *const u8 as *const libc::c_char,
                // 2163: b"--quiet\0" as ... _char: typeof(_119) = *const {l187} i8
                // 2163: b"--quiet\0" as ... _char:   l187 = UNIQUE | NON_NULL, (empty)
                // 2163: b"--quiet\0" as ... st u8: typeof(_120) = *const {l189} u8
                // 2163: b"--quiet\0" as ... st u8:   l189 = UNIQUE | NON_NULL, (empty)
                // 2163: b"--quiet\0": typeof(_121) = *const {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 2163: b"--quiet\0":   l191 = UNIQUE | NON_NULL, (empty)
                // 2163: b"--quiet\0": typeof(_122) = & {l193} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 2163: b"--quiet\0":   l193 = UNIQUE | NON_NULL, FIXED
                // 2163: b"--quiet\0" as ... _char: typeof(_119 = move _120 as *const i8 (Misc)) = *const {l1102} i8
                // 2163: b"--quiet\0" as ... _char:   l1102 = UNIQUE | NON_NULL, (empty)
                // 2163: b"--quiet\0": typeof(_121 = &raw const (*_122)) = *const {l1100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 2163: b"--quiet\0":   l1100 = UNIQUE | NON_NULL, (empty)
                // 2163: b"--quiet\0" as ... st u8: typeof(_120 = move _121 as *const u8 (Pointer(ArrayToPointer))) = *const {l1101} u8
                // 2163: b"--quiet\0" as ... st u8:   l1101 = UNIQUE | NON_NULL, (empty)
                // 2163: b"--quiet\0": typeof(_122 = const b"--quiet\x00") = & {l1099} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000008)) }]
                // 2163: b"--quiet\0":   l1099 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            env.quiet = 1 as libc::c_int;
            // 2166: env: typeof(_124) = *mut {l196} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2166: env:   l196 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2168: *argv.offset(i  ... size): typeof(_128) = *const {l201} i8
            // 2168: *argv.offset(i  ... size):   l201 = UNIQUE | NON_NULL, (empty)
            // 2168: *argv.offset(i  ... size): typeof(_129) = *mut {l203} i8
            // 2168: *argv.offset(i  ... size):   l203 = UNIQUE | NON_NULL, (empty)
            // 2168: argv.offset(i a ... size): typeof(_130) = *mut {l205} *mut {l206} i8
            // 2168: argv.offset(i a ... size):   l205 = UNIQUE | NON_NULL, (empty)
            // 2168: argv.offset(i a ... size):   l206 = UNIQUE | NON_NULL, (empty)
            // 2168: argv: typeof(_131) = *mut {l208} *mut {l209} i8
            // 2168: argv:   l208 = UNIQUE | NON_NULL, (empty)
            // 2168: argv:   l209 = UNIQUE | NON_NULL, (empty)
            // 2168: *argv.offset(i  ... size): typeof(_128 = move _129 as *const i8 (Pointer(MutToConstPointer))) = *const {l1103} i8
            // 2168: *argv.offset(i  ... size):   l1103 = UNIQUE | NON_NULL, (empty)
            b"-f\0" as *const u8 as *const libc::c_char,
            // 2169: b"-f\0" as *con ... _char: typeof(_134) = *const {l213} i8
            // 2169: b"-f\0" as *con ... _char:   l213 = UNIQUE | NON_NULL, (empty)
            // 2169: b"-f\0" as *const u8: typeof(_135) = *const {l215} u8
            // 2169: b"-f\0" as *const u8:   l215 = UNIQUE | NON_NULL, (empty)
            // 2169: b"-f\0": typeof(_136) = *const {l217} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2169: b"-f\0":   l217 = UNIQUE | NON_NULL, (empty)
            // 2169: b"-f\0": typeof(_137) = & {l219} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2169: b"-f\0":   l219 = UNIQUE | NON_NULL, FIXED
            // 2169: b"-f\0" as *con ... _char: typeof(_134 = move _135 as *const i8 (Misc)) = *const {l1107} i8
            // 2169: b"-f\0" as *con ... _char:   l1107 = UNIQUE | NON_NULL, (empty)
            // 2169: b"-f\0": typeof(_136 = &raw const (*_137)) = *const {l1105} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2169: b"-f\0":   l1105 = UNIQUE | NON_NULL, (empty)
            // 2169: b"-f\0": typeof(_137 = const b"-f\x00") = & {l1104} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2169: b"-f\0":   l1104 = UNIQUE | NON_NULL, (empty)
            // 2169: b"-f\0" as *const u8: typeof(_135 = move _136 as *const u8 (Pointer(ArrayToPointer))) = *const {l1106} u8
            // 2169: b"-f\0" as *const u8:   l1106 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2172: *argv.offset(i  ... size): typeof(_140) = *const {l223} i8
                // 2172: *argv.offset(i  ... size):   l223 = UNIQUE | NON_NULL, (empty)
                // 2172: *argv.offset(i  ... size): typeof(_141) = *mut {l225} i8
                // 2172: *argv.offset(i  ... size):   l225 = UNIQUE | NON_NULL, (empty)
                // 2172: argv.offset(i a ... size): typeof(_142) = *mut {l227} *mut {l228} i8
                // 2172: argv.offset(i a ... size):   l227 = UNIQUE | NON_NULL, (empty)
                // 2172: argv.offset(i a ... size):   l228 = UNIQUE | NON_NULL, (empty)
                // 2172: argv: typeof(_143) = *mut {l230} *mut {l231} i8
                // 2172: argv:   l230 = UNIQUE | NON_NULL, (empty)
                // 2172: argv:   l231 = UNIQUE | NON_NULL, (empty)
                // 2172: *argv.offset(i  ... size): typeof(_140 = move _141 as *const i8 (Pointer(MutToConstPointer))) = *const {l1108} i8
                // 2172: *argv.offset(i  ... size):   l1108 = UNIQUE | NON_NULL, (empty)
                b"--first-bug-only\0" as *const u8 as *const libc::c_char,
                // 2173: b"--first-bug-o ... _char: typeof(_146) = *const {l235} i8
                // 2173: b"--first-bug-o ... _char:   l235 = UNIQUE | NON_NULL, (empty)
                // 2173: b"--first-bug-o ... st u8: typeof(_147) = *const {l237} u8
                // 2173: b"--first-bug-o ... st u8:   l237 = UNIQUE | NON_NULL, (empty)
                // 2173: b"--first-bug-o ... ly\0": typeof(_148) = *const {l239} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2173: b"--first-bug-o ... ly\0":   l239 = UNIQUE | NON_NULL, (empty)
                // 2173: b"--first-bug-o ... ly\0": typeof(_149) = & {l241} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2173: b"--first-bug-o ... ly\0":   l241 = UNIQUE | NON_NULL, FIXED
                // 2173: b"--first-bug-o ... _char: typeof(_146 = move _147 as *const i8 (Misc)) = *const {l1112} i8
                // 2173: b"--first-bug-o ... _char:   l1112 = UNIQUE | NON_NULL, (empty)
                // 2173: b"--first-bug-o ... st u8: typeof(_147 = move _148 as *const u8 (Pointer(ArrayToPointer))) = *const {l1111} u8
                // 2173: b"--first-bug-o ... st u8:   l1111 = UNIQUE | NON_NULL, (empty)
                // 2173: b"--first-bug-o ... ly\0": typeof(_148 = &raw const (*_149)) = *const {l1110} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2173: b"--first-bug-o ... ly\0":   l1110 = UNIQUE | NON_NULL, (empty)
                // 2173: b"--first-bug-o ... ly\0": typeof(_149 = const b"--first-bug-only\x00") = & {l1109} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                // 2173: b"--first-bug-o ... ly\0":   l1109 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            env.first = 1 as libc::c_int;
            // 2176: env: typeof(_151) = *mut {l244} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2176: env:   l244 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2178: *argv.offset(i  ... size): typeof(_155) = *const {l249} i8
            // 2178: *argv.offset(i  ... size):   l249 = UNIQUE | NON_NULL, (empty)
            // 2178: *argv.offset(i  ... size): typeof(_156) = *mut {l251} i8
            // 2178: *argv.offset(i  ... size):   l251 = UNIQUE | NON_NULL, (empty)
            // 2178: argv.offset(i a ... size): typeof(_157) = *mut {l253} *mut {l254} i8
            // 2178: argv.offset(i a ... size):   l253 = UNIQUE | NON_NULL, (empty)
            // 2178: argv.offset(i a ... size):   l254 = UNIQUE | NON_NULL, (empty)
            // 2178: argv: typeof(_158) = *mut {l256} *mut {l257} i8
            // 2178: argv:   l256 = UNIQUE | NON_NULL, (empty)
            // 2178: argv:   l257 = UNIQUE | NON_NULL, (empty)
            // 2178: *argv.offset(i  ... size): typeof(_155 = move _156 as *const i8 (Pointer(MutToConstPointer))) = *const {l1113} i8
            // 2178: *argv.offset(i  ... size):   l1113 = UNIQUE | NON_NULL, (empty)
            b"-n\0" as *const u8 as *const libc::c_char,
            // 2179: b"-n\0" as *con ... _char: typeof(_161) = *const {l261} i8
            // 2179: b"-n\0" as *con ... _char:   l261 = UNIQUE | NON_NULL, (empty)
            // 2179: b"-n\0" as *const u8: typeof(_162) = *const {l263} u8
            // 2179: b"-n\0" as *const u8:   l263 = UNIQUE | NON_NULL, (empty)
            // 2179: b"-n\0": typeof(_163) = *const {l265} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2179: b"-n\0":   l265 = UNIQUE | NON_NULL, (empty)
            // 2179: b"-n\0": typeof(_164) = & {l267} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2179: b"-n\0":   l267 = UNIQUE | NON_NULL, FIXED
            // 2179: b"-n\0" as *con ... _char: typeof(_161 = move _162 as *const i8 (Misc)) = *const {l1117} i8
            // 2179: b"-n\0" as *con ... _char:   l1117 = UNIQUE | NON_NULL, (empty)
            // 2179: b"-n\0": typeof(_163 = &raw const (*_164)) = *const {l1115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2179: b"-n\0":   l1115 = UNIQUE | NON_NULL, (empty)
            // 2179: b"-n\0": typeof(_164 = const b"-n\x00") = & {l1114} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2179: b"-n\0":   l1114 = UNIQUE | NON_NULL, (empty)
            // 2179: b"-n\0" as *const u8: typeof(_162 = move _163 as *const u8 (Pointer(ArrayToPointer))) = *const {l1116} u8
            // 2179: b"-n\0" as *const u8:   l1116 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2182: *argv.offset(i  ... size): typeof(_167) = *const {l271} i8
                // 2182: *argv.offset(i  ... size):   l271 = UNIQUE | NON_NULL, (empty)
                // 2182: *argv.offset(i  ... size): typeof(_168) = *mut {l273} i8
                // 2182: *argv.offset(i  ... size):   l273 = UNIQUE | NON_NULL, (empty)
                // 2182: argv.offset(i a ... size): typeof(_169) = *mut {l275} *mut {l276} i8
                // 2182: argv.offset(i a ... size):   l275 = UNIQUE | NON_NULL, (empty)
                // 2182: argv.offset(i a ... size):   l276 = UNIQUE | NON_NULL, (empty)
                // 2182: argv: typeof(_170) = *mut {l278} *mut {l279} i8
                // 2182: argv:   l278 = UNIQUE | NON_NULL, (empty)
                // 2182: argv:   l279 = UNIQUE | NON_NULL, (empty)
                // 2182: *argv.offset(i  ... size): typeof(_167 = move _168 as *const i8 (Pointer(MutToConstPointer))) = *const {l1118} i8
                // 2182: *argv.offset(i  ... size):   l1118 = UNIQUE | NON_NULL, (empty)
                b"--no-delta-debugging\0" as *const u8 as *const libc::c_char,
                // 2183: b"--no-delta-de ... _char: typeof(_173) = *const {l283} i8
                // 2183: b"--no-delta-de ... _char:   l283 = UNIQUE | NON_NULL, (empty)
                // 2183: b"--no-delta-de ... st u8: typeof(_174) = *const {l285} u8
                // 2183: b"--no-delta-de ... st u8:   l285 = UNIQUE | NON_NULL, (empty)
                // 2183: b"--no-delta-de ... ng\0": typeof(_175) = *const {l287} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2183: b"--no-delta-de ... ng\0":   l287 = UNIQUE | NON_NULL, (empty)
                // 2183: b"--no-delta-de ... ng\0": typeof(_176) = & {l289} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2183: b"--no-delta-de ... ng\0":   l289 = UNIQUE | NON_NULL, FIXED
                // 2183: b"--no-delta-de ... ng\0": typeof(_176 = const b"--no-delta-debugging\x00") = & {l1119} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2183: b"--no-delta-de ... ng\0":   l1119 = UNIQUE | NON_NULL, (empty)
                // 2183: b"--no-delta-de ... st u8: typeof(_174 = move _175 as *const u8 (Pointer(ArrayToPointer))) = *const {l1121} u8
                // 2183: b"--no-delta-de ... st u8:   l1121 = UNIQUE | NON_NULL, (empty)
                // 2183: b"--no-delta-de ... ng\0": typeof(_175 = &raw const (*_176)) = *const {l1120} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000015)) }]
                // 2183: b"--no-delta-de ... ng\0":   l1120 = UNIQUE | NON_NULL, (empty)
                // 2183: b"--no-delta-de ... _char: typeof(_173 = move _174 as *const i8 (Misc)) = *const {l1122} i8
                // 2183: b"--no-delta-de ... _char:   l1122 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            env.nodd = 1 as libc::c_int;
            // 2186: env: typeof(_178) = *mut {l292} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2186: env:   l292 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2188: *argv.offset(i  ... size): typeof(_182) = *const {l297} i8
            // 2188: *argv.offset(i  ... size):   l297 = UNIQUE | NON_NULL, (empty)
            // 2188: *argv.offset(i  ... size): typeof(_183) = *mut {l299} i8
            // 2188: *argv.offset(i  ... size):   l299 = UNIQUE | NON_NULL, (empty)
            // 2188: argv.offset(i a ... size): typeof(_184) = *mut {l301} *mut {l302} i8
            // 2188: argv.offset(i a ... size):   l301 = UNIQUE | NON_NULL, (empty)
            // 2188: argv.offset(i a ... size):   l302 = UNIQUE | NON_NULL, (empty)
            // 2188: argv: typeof(_185) = *mut {l304} *mut {l305} i8
            // 2188: argv:   l304 = UNIQUE | NON_NULL, (empty)
            // 2188: argv:   l305 = UNIQUE | NON_NULL, (empty)
            // 2188: *argv.offset(i  ... size): typeof(_182 = move _183 as *const i8 (Pointer(MutToConstPointer))) = *const {l1123} i8
            // 2188: *argv.offset(i  ... size):   l1123 = UNIQUE | NON_NULL, (empty)
            b"-d\0" as *const u8 as *const libc::c_char,
            // 2189: b"-d\0" as *con ... _char: typeof(_188) = *const {l309} i8
            // 2189: b"-d\0" as *con ... _char:   l309 = UNIQUE | NON_NULL, (empty)
            // 2189: b"-d\0" as *const u8: typeof(_189) = *const {l311} u8
            // 2189: b"-d\0" as *const u8:   l311 = UNIQUE | NON_NULL, (empty)
            // 2189: b"-d\0": typeof(_190) = *const {l313} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2189: b"-d\0":   l313 = UNIQUE | NON_NULL, (empty)
            // 2189: b"-d\0": typeof(_191) = & {l315} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2189: b"-d\0":   l315 = UNIQUE | NON_NULL, FIXED
            // 2189: b"-d\0" as *con ... _char: typeof(_188 = move _189 as *const i8 (Misc)) = *const {l1127} i8
            // 2189: b"-d\0" as *con ... _char:   l1127 = UNIQUE | NON_NULL, (empty)
            // 2189: b"-d\0" as *const u8: typeof(_189 = move _190 as *const u8 (Pointer(ArrayToPointer))) = *const {l1126} u8
            // 2189: b"-d\0" as *const u8:   l1126 = UNIQUE | NON_NULL, (empty)
            // 2189: b"-d\0": typeof(_191 = const b"-d\x00") = & {l1124} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2189: b"-d\0":   l1124 = UNIQUE | NON_NULL, (empty)
            // 2189: b"-d\0": typeof(_190 = &raw const (*_191)) = *const {l1125} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2189: b"-d\0":   l1125 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2192: *argv.offset(i  ... size): typeof(_194) = *const {l319} i8
                // 2192: *argv.offset(i  ... size):   l319 = UNIQUE | NON_NULL, (empty)
                // 2192: *argv.offset(i  ... size): typeof(_195) = *mut {l321} i8
                // 2192: *argv.offset(i  ... size):   l321 = UNIQUE | NON_NULL, (empty)
                // 2192: argv.offset(i a ... size): typeof(_196) = *mut {l323} *mut {l324} i8
                // 2192: argv.offset(i a ... size):   l323 = UNIQUE | NON_NULL, (empty)
                // 2192: argv.offset(i a ... size):   l324 = UNIQUE | NON_NULL, (empty)
                // 2192: argv: typeof(_197) = *mut {l326} *mut {l327} i8
                // 2192: argv:   l326 = UNIQUE | NON_NULL, (empty)
                // 2192: argv:   l327 = UNIQUE | NON_NULL, (empty)
                // 2192: *argv.offset(i  ... size): typeof(_194 = move _195 as *const i8 (Pointer(MutToConstPointer))) = *const {l1128} i8
                // 2192: *argv.offset(i  ... size):   l1128 = UNIQUE | NON_NULL, (empty)
                b"--do-not-fuzz-options\0" as *const u8 as *const libc::c_char,
                // 2193: b"--do-not-fuzz ... _char: typeof(_200) = *const {l331} i8
                // 2193: b"--do-not-fuzz ... _char:   l331 = UNIQUE | NON_NULL, (empty)
                // 2193: b"--do-not-fuzz ... st u8: typeof(_201) = *const {l333} u8
                // 2193: b"--do-not-fuzz ... st u8:   l333 = UNIQUE | NON_NULL, (empty)
                // 2193: b"--do-not-fuzz ... ns\0": typeof(_202) = *const {l335} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2193: b"--do-not-fuzz ... ns\0":   l335 = UNIQUE | NON_NULL, (empty)
                // 2193: b"--do-not-fuzz ... ns\0": typeof(_203) = & {l337} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2193: b"--do-not-fuzz ... ns\0":   l337 = UNIQUE | NON_NULL, FIXED
                // 2193: b"--do-not-fuzz ... ns\0": typeof(_202 = &raw const (*_203)) = *const {l1130} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2193: b"--do-not-fuzz ... ns\0":   l1130 = UNIQUE | NON_NULL, (empty)
                // 2193: b"--do-not-fuzz ... _char: typeof(_200 = move _201 as *const i8 (Misc)) = *const {l1132} i8
                // 2193: b"--do-not-fuzz ... _char:   l1132 = UNIQUE | NON_NULL, (empty)
                // 2193: b"--do-not-fuzz ... ns\0": typeof(_203 = const b"--do-not-fuzz-options\x00") = & {l1129} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                // 2193: b"--do-not-fuzz ... ns\0":   l1129 = UNIQUE | NON_NULL, (empty)
                // 2193: b"--do-not-fuzz ... st u8: typeof(_201 = move _202 as *const u8 (Pointer(ArrayToPointer))) = *const {l1131} u8
                // 2193: b"--do-not-fuzz ... st u8:   l1131 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            env.noptsfuzz = 1 as libc::c_int;
            // 2196: env: typeof(_205) = *mut {l340} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2196: env:   l340 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2198: *argv.offset(i  ... size): typeof(_209) = *const {l345} i8
            // 2198: *argv.offset(i  ... size):   l345 = UNIQUE | NON_NULL, (empty)
            // 2198: *argv.offset(i  ... size): typeof(_210) = *mut {l347} i8
            // 2198: *argv.offset(i  ... size):   l347 = UNIQUE | NON_NULL, (empty)
            // 2198: argv.offset(i a ... size): typeof(_211) = *mut {l349} *mut {l350} i8
            // 2198: argv.offset(i a ... size):   l349 = UNIQUE | NON_NULL, (empty)
            // 2198: argv.offset(i a ... size):   l350 = UNIQUE | NON_NULL, (empty)
            // 2198: argv: typeof(_212) = *mut {l352} *mut {l353} i8
            // 2198: argv:   l352 = UNIQUE | NON_NULL, (empty)
            // 2198: argv:   l353 = UNIQUE | NON_NULL, (empty)
            // 2198: *argv.offset(i  ... size): typeof(_209 = move _210 as *const i8 (Pointer(MutToConstPointer))) = *const {l1133} i8
            // 2198: *argv.offset(i  ... size):   l1133 = UNIQUE | NON_NULL, (empty)
            b"-a\0" as *const u8 as *const libc::c_char,
            // 2199: b"-a\0" as *con ... _char: typeof(_215) = *const {l357} i8
            // 2199: b"-a\0" as *con ... _char:   l357 = UNIQUE | NON_NULL, (empty)
            // 2199: b"-a\0" as *const u8: typeof(_216) = *const {l359} u8
            // 2199: b"-a\0" as *const u8:   l359 = UNIQUE | NON_NULL, (empty)
            // 2199: b"-a\0": typeof(_217) = *const {l361} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2199: b"-a\0":   l361 = UNIQUE | NON_NULL, (empty)
            // 2199: b"-a\0": typeof(_218) = & {l363} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2199: b"-a\0":   l363 = UNIQUE | NON_NULL, FIXED
            // 2199: b"-a\0" as *const u8: typeof(_216 = move _217 as *const u8 (Pointer(ArrayToPointer))) = *const {l1136} u8
            // 2199: b"-a\0" as *const u8:   l1136 = UNIQUE | NON_NULL, (empty)
            // 2199: b"-a\0" as *con ... _char: typeof(_215 = move _216 as *const i8 (Misc)) = *const {l1137} i8
            // 2199: b"-a\0" as *con ... _char:   l1137 = UNIQUE | NON_NULL, (empty)
            // 2199: b"-a\0": typeof(_217 = &raw const (*_218)) = *const {l1135} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2199: b"-a\0":   l1135 = UNIQUE | NON_NULL, (empty)
            // 2199: b"-a\0": typeof(_218 = const b"-a\x00") = & {l1134} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2199: b"-a\0":   l1134 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2202: *argv.offset(i  ... size): typeof(_221) = *const {l367} i8
                // 2202: *argv.offset(i  ... size):   l367 = UNIQUE | NON_NULL, (empty)
                // 2202: *argv.offset(i  ... size): typeof(_222) = *mut {l369} i8
                // 2202: *argv.offset(i  ... size):   l369 = UNIQUE | NON_NULL, (empty)
                // 2202: argv.offset(i a ... size): typeof(_223) = *mut {l371} *mut {l372} i8
                // 2202: argv.offset(i a ... size):   l371 = UNIQUE | NON_NULL, (empty)
                // 2202: argv.offset(i a ... size):   l372 = UNIQUE | NON_NULL, (empty)
                // 2202: argv: typeof(_224) = *mut {l374} *mut {l375} i8
                // 2202: argv:   l374 = UNIQUE | NON_NULL, (empty)
                // 2202: argv:   l375 = UNIQUE | NON_NULL, (empty)
                // 2202: *argv.offset(i  ... size): typeof(_221 = move _222 as *const i8 (Pointer(MutToConstPointer))) = *const {l1138} i8
                // 2202: *argv.offset(i  ... size):   l1138 = UNIQUE | NON_NULL, (empty)
                b"--always-fork\0" as *const u8 as *const libc::c_char,
                // 2203: b"--always-fork ... _char: typeof(_227) = *const {l379} i8
                // 2203: b"--always-fork ... _char:   l379 = UNIQUE | NON_NULL, (empty)
                // 2203: b"--always-fork ... st u8: typeof(_228) = *const {l381} u8
                // 2203: b"--always-fork ... st u8:   l381 = UNIQUE | NON_NULL, (empty)
                // 2203: b"--always-fork\0": typeof(_229) = *const {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 2203: b"--always-fork\0":   l383 = UNIQUE | NON_NULL, (empty)
                // 2203: b"--always-fork\0": typeof(_230) = & {l385} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 2203: b"--always-fork\0":   l385 = UNIQUE | NON_NULL, FIXED
                // 2203: b"--always-fork ... _char: typeof(_227 = move _228 as *const i8 (Misc)) = *const {l1142} i8
                // 2203: b"--always-fork ... _char:   l1142 = UNIQUE | NON_NULL, (empty)
                // 2203: b"--always-fork ... st u8: typeof(_228 = move _229 as *const u8 (Pointer(ArrayToPointer))) = *const {l1141} u8
                // 2203: b"--always-fork ... st u8:   l1141 = UNIQUE | NON_NULL, (empty)
                // 2203: b"--always-fork\0": typeof(_229 = &raw const (*_230)) = *const {l1140} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 2203: b"--always-fork\0":   l1140 = UNIQUE | NON_NULL, (empty)
                // 2203: b"--always-fork\0": typeof(_230 = const b"--always-fork\x00") = & {l1139} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000e)) }]
                // 2203: b"--always-fork\0":   l1139 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            env.alwaysfork = 1 as libc::c_int;
            // 2206: env: typeof(_232) = *mut {l388} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2206: env:   l388 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2208: *argv.offset(i  ... size): typeof(_236) = *const {l393} i8
            // 2208: *argv.offset(i  ... size):   l393 = UNIQUE | NON_NULL, (empty)
            // 2208: *argv.offset(i  ... size): typeof(_237) = *mut {l395} i8
            // 2208: *argv.offset(i  ... size):   l395 = UNIQUE | NON_NULL, (empty)
            // 2208: argv.offset(i a ... size): typeof(_238) = *mut {l397} *mut {l398} i8
            // 2208: argv.offset(i a ... size):   l397 = UNIQUE | NON_NULL, (empty)
            // 2208: argv.offset(i a ... size):   l398 = UNIQUE | NON_NULL, (empty)
            // 2208: argv: typeof(_239) = *mut {l400} *mut {l401} i8
            // 2208: argv:   l400 = UNIQUE | NON_NULL, (empty)
            // 2208: argv:   l401 = UNIQUE | NON_NULL, (empty)
            // 2208: *argv.offset(i  ... size): typeof(_236 = move _237 as *const i8 (Pointer(MutToConstPointer))) = *const {l1143} i8
            // 2208: *argv.offset(i  ... size):   l1143 = UNIQUE | NON_NULL, (empty)
            b"-O\0" as *const u8 as *const libc::c_char,
            // 2209: b"-O\0" as *con ... _char: typeof(_242) = *const {l405} i8
            // 2209: b"-O\0" as *con ... _char:   l405 = UNIQUE | NON_NULL, (empty)
            // 2209: b"-O\0" as *const u8: typeof(_243) = *const {l407} u8
            // 2209: b"-O\0" as *const u8:   l407 = UNIQUE | NON_NULL, (empty)
            // 2209: b"-O\0": typeof(_244) = *const {l409} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2209: b"-O\0":   l409 = UNIQUE | NON_NULL, (empty)
            // 2209: b"-O\0": typeof(_245) = & {l411} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2209: b"-O\0":   l411 = UNIQUE | NON_NULL, FIXED
            // 2209: b"-O\0": typeof(_244 = &raw const (*_245)) = *const {l1145} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2209: b"-O\0":   l1145 = UNIQUE | NON_NULL, (empty)
            // 2209: b"-O\0" as *con ... _char: typeof(_242 = move _243 as *const i8 (Misc)) = *const {l1147} i8
            // 2209: b"-O\0" as *con ... _char:   l1147 = UNIQUE | NON_NULL, (empty)
            // 2209: b"-O\0" as *const u8: typeof(_243 = move _244 as *const u8 (Pointer(ArrayToPointer))) = *const {l1146} u8
            // 2209: b"-O\0" as *const u8:   l1146 = UNIQUE | NON_NULL, (empty)
            // 2209: b"-O\0": typeof(_245 = const b"-O\x00") = & {l1144} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2209: b"-O\0":   l1144 = UNIQUE | NON_NULL, (empty)
        ) == 0
            || strcmp(
                *argv.offset(i as isize),
                // 2212: *argv.offset(i  ... size): typeof(_248) = *const {l415} i8
                // 2212: *argv.offset(i  ... size):   l415 = UNIQUE | NON_NULL, (empty)
                // 2212: *argv.offset(i  ... size): typeof(_249) = *mut {l417} i8
                // 2212: *argv.offset(i  ... size):   l417 = UNIQUE | NON_NULL, (empty)
                // 2212: argv.offset(i a ... size): typeof(_250) = *mut {l419} *mut {l420} i8
                // 2212: argv.offset(i a ... size):   l419 = UNIQUE | NON_NULL, (empty)
                // 2212: argv.offset(i a ... size):   l420 = UNIQUE | NON_NULL, (empty)
                // 2212: argv: typeof(_251) = *mut {l422} *mut {l423} i8
                // 2212: argv:   l422 = UNIQUE | NON_NULL, (empty)
                // 2212: argv:   l423 = UNIQUE | NON_NULL, (empty)
                // 2212: *argv.offset(i  ... size): typeof(_248 = move _249 as *const i8 (Pointer(MutToConstPointer))) = *const {l1148} i8
                // 2212: *argv.offset(i  ... size):   l1148 = UNIQUE | NON_NULL, (empty)
                b"--optimize-by-delta-debugging-options\0" as *const u8 as *const libc::c_char,
                // 2213: b"--optimize-by ... _char: typeof(_254) = *const {l427} i8
                // 2213: b"--optimize-by ... _char:   l427 = UNIQUE | NON_NULL, (empty)
                // 2213: b"--optimize-by ... st u8: typeof(_255) = *const {l429} u8
                // 2213: b"--optimize-by ... st u8:   l429 = UNIQUE | NON_NULL, (empty)
                // 2213: b"--optimize-by ... ns\0": typeof(_256) = *const {l431} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 2213: b"--optimize-by ... ns\0":   l431 = UNIQUE | NON_NULL, (empty)
                // 2213: b"--optimize-by ... ns\0": typeof(_257) = & {l433} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 2213: b"--optimize-by ... ns\0":   l433 = UNIQUE | NON_NULL, FIXED
                // 2213: b"--optimize-by ... ns\0": typeof(_257 = const b"--optimize-by-delta-debugging-options\x00") = & {l1149} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 2213: b"--optimize-by ... ns\0":   l1149 = UNIQUE | NON_NULL, (empty)
                // 2213: b"--optimize-by ... st u8: typeof(_255 = move _256 as *const u8 (Pointer(ArrayToPointer))) = *const {l1151} u8
                // 2213: b"--optimize-by ... st u8:   l1151 = UNIQUE | NON_NULL, (empty)
                // 2213: b"--optimize-by ... ns\0": typeof(_256 = &raw const (*_257)) = *const {l1150} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000026)) }]
                // 2213: b"--optimize-by ... ns\0":   l1150 = UNIQUE | NON_NULL, (empty)
                // 2213: b"--optimize-by ... _char: typeof(_254 = move _255 as *const i8 (Misc)) = *const {l1152} i8
                // 2213: b"--optimize-by ... _char:   l1152 = UNIQUE | NON_NULL, (empty)
            ) == 0
        {
            opt = 1 as libc::c_int;
        } else if strcmp(
            *argv.offset(i as isize),
            // 2218: *argv.offset(i  ... size): typeof(_261) = *const {l438} i8
            // 2218: *argv.offset(i  ... size):   l438 = UNIQUE | NON_NULL, (empty)
            // 2218: *argv.offset(i  ... size): typeof(_262) = *mut {l440} i8
            // 2218: *argv.offset(i  ... size):   l440 = UNIQUE | NON_NULL, (empty)
            // 2218: argv.offset(i a ... size): typeof(_263) = *mut {l442} *mut {l443} i8
            // 2218: argv.offset(i a ... size):   l442 = UNIQUE | NON_NULL, (empty)
            // 2218: argv.offset(i a ... size):   l443 = UNIQUE | NON_NULL, (empty)
            // 2218: argv: typeof(_264) = *mut {l445} *mut {l446} i8
            // 2218: argv:   l445 = UNIQUE | NON_NULL, (empty)
            // 2218: argv:   l446 = UNIQUE | NON_NULL, (empty)
            // 2218: *argv.offset(i  ... size): typeof(_261 = move _262 as *const i8 (Pointer(MutToConstPointer))) = *const {l1153} i8
            // 2218: *argv.offset(i  ... size):   l1153 = UNIQUE | NON_NULL, (empty)
            b"-t\0" as *const u8 as *const libc::c_char,
            // 2219: b"-t\0" as *con ... _char: typeof(_267) = *const {l450} i8
            // 2219: b"-t\0" as *con ... _char:   l450 = UNIQUE | NON_NULL, (empty)
            // 2219: b"-t\0" as *const u8: typeof(_268) = *const {l452} u8
            // 2219: b"-t\0" as *const u8:   l452 = UNIQUE | NON_NULL, (empty)
            // 2219: b"-t\0": typeof(_269) = *const {l454} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2219: b"-t\0":   l454 = UNIQUE | NON_NULL, (empty)
            // 2219: b"-t\0": typeof(_270) = & {l456} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2219: b"-t\0":   l456 = UNIQUE | NON_NULL, FIXED
            // 2219: b"-t\0": typeof(_269 = &raw const (*_270)) = *const {l1155} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2219: b"-t\0":   l1155 = UNIQUE | NON_NULL, (empty)
            // 2219: b"-t\0" as *con ... _char: typeof(_267 = move _268 as *const i8 (Misc)) = *const {l1157} i8
            // 2219: b"-t\0" as *con ... _char:   l1157 = UNIQUE | NON_NULL, (empty)
            // 2219: b"-t\0" as *const u8: typeof(_268 = move _269 as *const u8 (Pointer(ArrayToPointer))) = *const {l1156} u8
            // 2219: b"-t\0" as *const u8:   l1156 = UNIQUE | NON_NULL, (empty)
            // 2219: b"-t\0": typeof(_270 = const b"-t\x00") = & {l1154} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2219: b"-t\0":   l1154 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            i += 1;
            if i == argc {
                die(b"argument to '-t' missing (try '-h')\0" as *const u8 as *const libc::c_char);
                // 2224: b"argument to ' ... _char: typeof(_277) = *const {l464} i8
                // 2224: b"argument to ' ... _char:   l464 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... st u8: typeof(_278) = *const {l466} u8
                // 2224: b"argument to ' ... st u8:   l466 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ')\0": typeof(_279) = *const {l468} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2224: b"argument to ' ... ')\0":   l468 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ')\0": typeof(_280) = & {l470} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2224: b"argument to ' ... ')\0":   l470 = UNIQUE | NON_NULL, FIXED
                // 2224: b"argument to ' ... _char: typeof(_277 = move _278 as *const i8 (Misc)) = *const {l1161} i8
                // 2224: b"argument to ' ... _char:   l1161 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... st u8: typeof(_278 = move _279 as *const u8 (Pointer(ArrayToPointer))) = *const {l1160} u8
                // 2224: b"argument to ' ... st u8:   l1160 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ')\0": typeof(_280 = const b"argument to \'-t\' missing (try \'-h\')\x00") = & {l1158} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2224: b"argument to ' ... ')\0":   l1158 = UNIQUE | NON_NULL, (empty)
                // 2224: b"argument to ' ... ')\0": typeof(_279 = &raw const (*_280)) = *const {l1159} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2224: b"argument to ' ... ')\0":   l1159 = UNIQUE | NON_NULL, (empty)
            }
            if isnumstr(*argv.offset(i as isize)) == 0 {
            // 2226: *argv.offset(i  ... size): typeof(_284) = *const {l475} i8
            // 2226: *argv.offset(i  ... size):   l475 = UNIQUE | NON_NULL, (empty)
            // 2226: *argv.offset(i  ... size): typeof(_285) = *mut {l477} i8
            // 2226: *argv.offset(i  ... size):   l477 = UNIQUE | NON_NULL, (empty)
            // 2226: argv.offset(i a ... size): typeof(_286) = *mut {l479} *mut {l480} i8
            // 2226: argv.offset(i a ... size):   l479 = UNIQUE | NON_NULL, (empty)
            // 2226: argv.offset(i a ... size):   l480 = UNIQUE | NON_NULL, (empty)
            // 2226: argv: typeof(_287) = *mut {l482} *mut {l483} i8
            // 2226: argv:   l482 = UNIQUE | NON_NULL, (empty)
            // 2226: argv:   l483 = UNIQUE | NON_NULL, (empty)
            // 2226: *argv.offset(i  ... size): typeof(_284 = move _285 as *const i8 (Pointer(MutToConstPointer))) = *const {l1162} i8
            // 2226: *argv.offset(i  ... size):   l1162 = UNIQUE | NON_NULL, (empty)
                die(
                    b"argument '%s' to '-t' not a number (try '-h')\0" as *const u8
                    // 2228: b"argument '%s' ... _char: typeof(_291) = *const {l488} i8
                    // 2228: b"argument '%s' ... _char:   l488 = UNIQUE | NON_NULL, (empty)
                    // 2228: b"argument '%s' ... st u8: typeof(_292) = *const {l490} u8
                    // 2228: b"argument '%s' ... st u8:   l490 = UNIQUE | NON_NULL, (empty)
                    // 2228: b"argument '%s' ... ')\0": typeof(_293) = *const {l492} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2228: b"argument '%s' ... ')\0":   l492 = UNIQUE | NON_NULL, (empty)
                    // 2228: b"argument '%s' ... ')\0": typeof(_294) = & {l494} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2228: b"argument '%s' ... ')\0":   l494 = UNIQUE | NON_NULL, FIXED
                    // 2228: b"argument '%s' ... _char: typeof(_291 = move _292 as *const i8 (Misc)) = *const {l1166} i8
                    // 2228: b"argument '%s' ... _char:   l1166 = UNIQUE | NON_NULL, (empty)
                    // 2228: b"argument '%s' ... ')\0": typeof(_294 = const b"argument \'%s\' to \'-t\' not a number (try \'-h\')\x00") = & {l1163} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2228: b"argument '%s' ... ')\0":   l1163 = UNIQUE | NON_NULL, (empty)
                    // 2228: b"argument '%s' ... ')\0": typeof(_293 = &raw const (*_294)) = *const {l1164} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2228: b"argument '%s' ... ')\0":   l1164 = UNIQUE | NON_NULL, (empty)
                    // 2228: b"argument '%s' ... st u8: typeof(_292 = move _293 as *const u8 (Pointer(ArrayToPointer))) = *const {l1165} u8
                    // 2228: b"argument '%s' ... st u8:   l1165 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                    // 2230: *argv.offset(i  ... size): typeof(_295) = *mut {l496} i8
                    // 2230: *argv.offset(i  ... size):   l496 = UNIQUE | NON_NULL, (empty)
                    // 2230: argv.offset(i a ... size): typeof(_296) = *mut {l498} *mut {l499} i8
                    // 2230: argv.offset(i a ... size):   l498 = UNIQUE | NON_NULL, (empty)
                    // 2230: argv.offset(i a ... size):   l499 = UNIQUE | NON_NULL, (empty)
                    // 2230: argv: typeof(_297) = *mut {l501} *mut {l502} i8
                    // 2230: argv:   l501 = UNIQUE | NON_NULL, (empty)
                    // 2230: argv:   l502 = UNIQUE | NON_NULL, (empty)
                );
            }
            env.timeout = atoi(*argv.offset(i as isize));
            // 2233: *argv.offset(i  ... size): typeof(_301) = *const {l507} i8
            // 2233: *argv.offset(i  ... size):   l507 = UNIQUE | NON_NULL, (empty)
            // 2233: *argv.offset(i  ... size): typeof(_302) = *mut {l509} i8
            // 2233: *argv.offset(i  ... size):   l509 = UNIQUE | NON_NULL, (empty)
            // 2233: argv.offset(i a ... size): typeof(_303) = *mut {l511} *mut {l512} i8
            // 2233: argv.offset(i a ... size):   l511 = UNIQUE | NON_NULL, (empty)
            // 2233: argv.offset(i a ... size):   l512 = UNIQUE | NON_NULL, (empty)
            // 2233: argv: typeof(_304) = *mut {l514} *mut {l515} i8
            // 2233: argv:   l514 = UNIQUE | NON_NULL, (empty)
            // 2233: argv:   l515 = UNIQUE | NON_NULL, (empty)
            // 2233: env: typeof(_307) = *mut {l519} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2233: env:   l519 = UNIQUE | NON_NULL, (empty)
            // 2233: *argv.offset(i  ... size): typeof(_301 = move _302 as *const i8 (Pointer(MutToConstPointer))) = *const {l1167} i8
            // 2233: *argv.offset(i  ... size):   l1167 = UNIQUE | NON_NULL, (empty)
        } else if strcmp(
            *argv.offset(i as isize),
            // 2235: *argv.offset(i  ... size): typeof(_310) = *const {l523} i8
            // 2235: *argv.offset(i  ... size):   l523 = UNIQUE | NON_NULL, (empty)
            // 2235: *argv.offset(i  ... size): typeof(_311) = *mut {l525} i8
            // 2235: *argv.offset(i  ... size):   l525 = UNIQUE | NON_NULL, (empty)
            // 2235: argv.offset(i a ... size): typeof(_312) = *mut {l527} *mut {l528} i8
            // 2235: argv.offset(i a ... size):   l527 = UNIQUE | NON_NULL, (empty)
            // 2235: argv.offset(i a ... size):   l528 = UNIQUE | NON_NULL, (empty)
            // 2235: argv: typeof(_313) = *mut {l530} *mut {l531} i8
            // 2235: argv:   l530 = UNIQUE | NON_NULL, (empty)
            // 2235: argv:   l531 = UNIQUE | NON_NULL, (empty)
            // 2235: *argv.offset(i  ... size): typeof(_310 = move _311 as *const i8 (Pointer(MutToConstPointer))) = *const {l1168} i8
            // 2235: *argv.offset(i  ... size):   l1168 = UNIQUE | NON_NULL, (empty)
            b"-m\0" as *const u8 as *const libc::c_char,
            // 2236: b"-m\0" as *con ... _char: typeof(_316) = *const {l535} i8
            // 2236: b"-m\0" as *con ... _char:   l535 = UNIQUE | NON_NULL, (empty)
            // 2236: b"-m\0" as *const u8: typeof(_317) = *const {l537} u8
            // 2236: b"-m\0" as *const u8:   l537 = UNIQUE | NON_NULL, (empty)
            // 2236: b"-m\0": typeof(_318) = *const {l539} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2236: b"-m\0":   l539 = UNIQUE | NON_NULL, (empty)
            // 2236: b"-m\0": typeof(_319) = & {l541} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2236: b"-m\0":   l541 = UNIQUE | NON_NULL, FIXED
            // 2236: b"-m\0": typeof(_319 = const b"-m\x00") = & {l1169} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2236: b"-m\0":   l1169 = UNIQUE | NON_NULL, (empty)
            // 2236: b"-m\0": typeof(_318 = &raw const (*_319)) = *const {l1170} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000003)) }]
            // 2236: b"-m\0":   l1170 = UNIQUE | NON_NULL, (empty)
            // 2236: b"-m\0" as *con ... _char: typeof(_316 = move _317 as *const i8 (Misc)) = *const {l1172} i8
            // 2236: b"-m\0" as *con ... _char:   l1172 = UNIQUE | NON_NULL, (empty)
            // 2236: b"-m\0" as *const u8: typeof(_317 = move _318 as *const u8 (Pointer(ArrayToPointer))) = *const {l1171} u8
            // 2236: b"-m\0" as *const u8:   l1171 = UNIQUE | NON_NULL, (empty)
        ) == 0
        {
            i += 1;
            if i == argc {
                die(b"argument to '-m' missing (try '-h')\0" as *const u8 as *const libc::c_char);
                // 2241: b"argument to ' ... _char: typeof(_326) = *const {l549} i8
                // 2241: b"argument to ' ... _char:   l549 = UNIQUE | NON_NULL, (empty)
                // 2241: b"argument to ' ... st u8: typeof(_327) = *const {l551} u8
                // 2241: b"argument to ' ... st u8:   l551 = UNIQUE | NON_NULL, (empty)
                // 2241: b"argument to ' ... ')\0": typeof(_328) = *const {l553} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2241: b"argument to ' ... ')\0":   l553 = UNIQUE | NON_NULL, (empty)
                // 2241: b"argument to ' ... ')\0": typeof(_329) = & {l555} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2241: b"argument to ' ... ')\0":   l555 = UNIQUE | NON_NULL, FIXED
                // 2241: b"argument to ' ... st u8: typeof(_327 = move _328 as *const u8 (Pointer(ArrayToPointer))) = *const {l1175} u8
                // 2241: b"argument to ' ... st u8:   l1175 = UNIQUE | NON_NULL, (empty)
                // 2241: b"argument to ' ... _char: typeof(_326 = move _327 as *const i8 (Misc)) = *const {l1176} i8
                // 2241: b"argument to ' ... _char:   l1176 = UNIQUE | NON_NULL, (empty)
                // 2241: b"argument to ' ... ')\0": typeof(_328 = &raw const (*_329)) = *const {l1174} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2241: b"argument to ' ... ')\0":   l1174 = UNIQUE | NON_NULL, (empty)
                // 2241: b"argument to ' ... ')\0": typeof(_329 = const b"argument to \'-m\' missing (try \'-h\')\x00") = & {l1173} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                // 2241: b"argument to ' ... ')\0":   l1173 = UNIQUE | NON_NULL, (empty)
            }
            if isnumstr(*argv.offset(i as isize)) == 0 {
            // 2243: *argv.offset(i  ... size): typeof(_333) = *const {l560} i8
            // 2243: *argv.offset(i  ... size):   l560 = UNIQUE | NON_NULL, (empty)
            // 2243: *argv.offset(i  ... size): typeof(_334) = *mut {l562} i8
            // 2243: *argv.offset(i  ... size):   l562 = UNIQUE | NON_NULL, (empty)
            // 2243: argv.offset(i a ... size): typeof(_335) = *mut {l564} *mut {l565} i8
            // 2243: argv.offset(i a ... size):   l564 = UNIQUE | NON_NULL, (empty)
            // 2243: argv.offset(i a ... size):   l565 = UNIQUE | NON_NULL, (empty)
            // 2243: argv: typeof(_336) = *mut {l567} *mut {l568} i8
            // 2243: argv:   l567 = UNIQUE | NON_NULL, (empty)
            // 2243: argv:   l568 = UNIQUE | NON_NULL, (empty)
            // 2243: *argv.offset(i  ... size): typeof(_333 = move _334 as *const i8 (Pointer(MutToConstPointer))) = *const {l1177} i8
            // 2243: *argv.offset(i  ... size):   l1177 = UNIQUE | NON_NULL, (empty)
                die(
                    b"argument '%s' to '-m' not a number (try '-h')\0" as *const u8
                    // 2245: b"argument '%s' ... _char: typeof(_340) = *const {l573} i8
                    // 2245: b"argument '%s' ... _char:   l573 = UNIQUE | NON_NULL, (empty)
                    // 2245: b"argument '%s' ... st u8: typeof(_341) = *const {l575} u8
                    // 2245: b"argument '%s' ... st u8:   l575 = UNIQUE | NON_NULL, (empty)
                    // 2245: b"argument '%s' ... ')\0": typeof(_342) = *const {l577} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2245: b"argument '%s' ... ')\0":   l577 = UNIQUE | NON_NULL, (empty)
                    // 2245: b"argument '%s' ... ')\0": typeof(_343) = & {l579} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2245: b"argument '%s' ... ')\0":   l579 = UNIQUE | NON_NULL, FIXED
                    // 2245: b"argument '%s' ... st u8: typeof(_341 = move _342 as *const u8 (Pointer(ArrayToPointer))) = *const {l1180} u8
                    // 2245: b"argument '%s' ... st u8:   l1180 = UNIQUE | NON_NULL, (empty)
                    // 2245: b"argument '%s' ... ')\0": typeof(_343 = const b"argument \'%s\' to \'-m\' not a number (try \'-h\')\x00") = & {l1178} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2245: b"argument '%s' ... ')\0":   l1178 = UNIQUE | NON_NULL, (empty)
                    // 2245: b"argument '%s' ... ')\0": typeof(_342 = &raw const (*_343)) = *const {l1179} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002e)) }]
                    // 2245: b"argument '%s' ... ')\0":   l1179 = UNIQUE | NON_NULL, (empty)
                    // 2245: b"argument '%s' ... _char: typeof(_340 = move _341 as *const i8 (Misc)) = *const {l1181} i8
                    // 2245: b"argument '%s' ... _char:   l1181 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                    *argv.offset(i as isize),
                    // 2247: *argv.offset(i  ... size): typeof(_344) = *mut {l581} i8
                    // 2247: *argv.offset(i  ... size):   l581 = UNIQUE | NON_NULL, (empty)
                    // 2247: argv.offset(i a ... size): typeof(_345) = *mut {l583} *mut {l584} i8
                    // 2247: argv.offset(i a ... size):   l583 = UNIQUE | NON_NULL, (empty)
                    // 2247: argv.offset(i a ... size):   l584 = UNIQUE | NON_NULL, (empty)
                    // 2247: argv: typeof(_346) = *mut {l586} *mut {l587} i8
                    // 2247: argv:   l586 = UNIQUE | NON_NULL, (empty)
                    // 2247: argv:   l587 = UNIQUE | NON_NULL, (empty)
                );
            }
            max = atoi(*argv.offset(i as isize));
            // 2250: *argv.offset(i  ... size): typeof(_350) = *const {l592} i8
            // 2250: *argv.offset(i  ... size):   l592 = UNIQUE | NON_NULL, (empty)
            // 2250: *argv.offset(i  ... size): typeof(_351) = *mut {l594} i8
            // 2250: *argv.offset(i  ... size):   l594 = UNIQUE | NON_NULL, (empty)
            // 2250: argv.offset(i a ... size): typeof(_352) = *mut {l596} *mut {l597} i8
            // 2250: argv.offset(i a ... size):   l596 = UNIQUE | NON_NULL, (empty)
            // 2250: argv.offset(i a ... size):   l597 = UNIQUE | NON_NULL, (empty)
            // 2250: argv: typeof(_353) = *mut {l599} *mut {l600} i8
            // 2250: argv:   l599 = UNIQUE | NON_NULL, (empty)
            // 2250: argv:   l600 = UNIQUE | NON_NULL, (empty)
            // 2250: *argv.offset(i  ... size): typeof(_350 = move _351 as *const i8 (Pointer(MutToConstPointer))) = *const {l1182} i8
            // 2250: *argv.offset(i  ... size):   l1182 = UNIQUE | NON_NULL, (empty)
        } else if isnumstr(*argv.offset(i as isize)) == 0 {
        // 2251: *argv.offset(i  ... size): typeof(_358) = *const {l606} i8
        // 2251: *argv.offset(i  ... size):   l606 = UNIQUE | NON_NULL, (empty)
        // 2251: *argv.offset(i  ... size): typeof(_359) = *mut {l608} i8
        // 2251: *argv.offset(i  ... size):   l608 = UNIQUE | NON_NULL, (empty)
        // 2251: argv.offset(i a ... size): typeof(_360) = *mut {l610} *mut {l611} i8
        // 2251: argv.offset(i a ... size):   l610 = UNIQUE | NON_NULL, (empty)
        // 2251: argv.offset(i a ... size):   l611 = UNIQUE | NON_NULL, (empty)
        // 2251: argv: typeof(_361) = *mut {l613} *mut {l614} i8
        // 2251: argv:   l613 = UNIQUE | NON_NULL, (empty)
        // 2251: argv:   l614 = UNIQUE | NON_NULL, (empty)
        // 2251: *argv.offset(i  ... size): typeof(_358 = move _359 as *const i8 (Pointer(MutToConstPointer))) = *const {l1183} i8
        // 2251: *argv.offset(i  ... size):   l1183 = UNIQUE | NON_NULL, (empty)
            die(
                b"invalid command line option '%s' (try '-h')\0" as *const u8
                // 2253: b"invalid comma ... _char: typeof(_365) = *const {l619} i8
                // 2253: b"invalid comma ... _char:   l619 = UNIQUE | NON_NULL, (empty)
                // 2253: b"invalid comma ... st u8: typeof(_366) = *const {l621} u8
                // 2253: b"invalid comma ... st u8:   l621 = UNIQUE | NON_NULL, (empty)
                // 2253: b"invalid comma ... ')\0": typeof(_367) = *const {l623} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2253: b"invalid comma ... ')\0":   l623 = UNIQUE | NON_NULL, (empty)
                // 2253: b"invalid comma ... ')\0": typeof(_368) = & {l625} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2253: b"invalid comma ... ')\0":   l625 = UNIQUE | NON_NULL, FIXED
                // 2253: b"invalid comma ... ')\0": typeof(_368 = const b"invalid command line option \'%s\' (try \'-h\')\x00") = & {l1184} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2253: b"invalid comma ... ')\0":   l1184 = UNIQUE | NON_NULL, (empty)
                // 2253: b"invalid comma ... _char: typeof(_365 = move _366 as *const i8 (Misc)) = *const {l1187} i8
                // 2253: b"invalid comma ... _char:   l1187 = UNIQUE | NON_NULL, (empty)
                // 2253: b"invalid comma ... ')\0": typeof(_367 = &raw const (*_368)) = *const {l1185} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                // 2253: b"invalid comma ... ')\0":   l1185 = UNIQUE | NON_NULL, (empty)
                // 2253: b"invalid comma ... st u8: typeof(_366 = move _367 as *const u8 (Pointer(ArrayToPointer))) = *const {l1186} u8
                // 2253: b"invalid comma ... st u8:   l1186 = UNIQUE | NON_NULL, (empty)
                    as *const libc::c_char,
                *argv.offset(i as isize),
                // 2255: *argv.offset(i  ... size): typeof(_369) = *mut {l627} i8
                // 2255: *argv.offset(i  ... size):   l627 = UNIQUE | NON_NULL, (empty)
                // 2255: argv.offset(i a ... size): typeof(_370) = *mut {l629} *mut {l630} i8
                // 2255: argv.offset(i a ... size):   l629 = UNIQUE | NON_NULL, (empty)
                // 2255: argv.offset(i a ... size):   l630 = UNIQUE | NON_NULL, (empty)
                // 2255: argv: typeof(_371) = *mut {l632} *mut {l633} i8
                // 2255: argv:   l632 = UNIQUE | NON_NULL, (empty)
                // 2255: argv:   l633 = UNIQUE | NON_NULL, (empty)
            );
        } else {
            env.seed = atoi(*argv.offset(i as isize)) as libc::c_uint;
            // 2258: *argv.offset(i  ... size): typeof(_375) = *const {l638} i8
            // 2258: *argv.offset(i  ... size):   l638 = UNIQUE | NON_NULL, (empty)
            // 2258: *argv.offset(i  ... size): typeof(_376) = *mut {l640} i8
            // 2258: *argv.offset(i  ... size):   l640 = UNIQUE | NON_NULL, (empty)
            // 2258: argv.offset(i a ... size): typeof(_377) = *mut {l642} *mut {l643} i8
            // 2258: argv.offset(i a ... size):   l642 = UNIQUE | NON_NULL, (empty)
            // 2258: argv.offset(i a ... size):   l643 = UNIQUE | NON_NULL, (empty)
            // 2258: argv: typeof(_378) = *mut {l645} *mut {l646} i8
            // 2258: argv:   l645 = UNIQUE | NON_NULL, (empty)
            // 2258: argv:   l646 = UNIQUE | NON_NULL, (empty)
            // 2258: env: typeof(_381) = *mut {l650} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2258: env:   l650 = UNIQUE | NON_NULL, (empty)
            // 2258: *argv.offset(i  ... size): typeof(_375 = move _376 as *const i8 (Pointer(MutToConstPointer))) = *const {l1188} i8
            // 2258: *argv.offset(i  ... size):   l1188 = UNIQUE | NON_NULL, (empty)
        }
        i += 1;
        i;
    }
    env.print = (env.quiet == 0) as libc::c_int;
    // 2263: env: typeof(_389) = *mut {l659} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2263: env:   l659 = UNIQUE | NON_NULL, (empty)
    // 2263: env: typeof(_390) = *mut {l661} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2263: env:   l661 = UNIQUE | NON_NULL, (empty)
    if env.seed != -(1 as libc::c_int) as libc::c_uint && env.alwaysfork == 0 {
    // 2264: env: typeof(_395) = *mut {l667} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2264: env:   l667 = UNIQUE | NON_NULL, (empty)
    // 2264: env: typeof(_402) = *mut {l675} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2264: env:   l675 = UNIQUE | NON_NULL, (empty)
        rantrav(&mut env);
        // 2265: &mut env: typeof(_404) = *mut {l678} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2265: &mut env:   l678 = UNIQUE | NON_NULL, (empty)
        // 2265: &mut env: typeof(_405) = &mut {l680} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2265: &mut env:   l680 = UNIQUE | NON_NULL, (empty)
        // 2265: env: typeof(_406) = *mut {l682} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2265: env:   l682 = UNIQUE | NON_NULL, (empty)
        // 2265: &mut env: typeof(_405 = &mut (*_406)) = &mut {l1189} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2265: &mut env:   l1189 = UNIQUE | NON_NULL, (empty)
        // 2265: &mut env: typeof(_404 = &raw mut (*_405)) = *mut {l1190} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2265: &mut env:   l1190 = UNIQUE | NON_NULL, (empty)
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        // 2266: b"\n\0" as *con ... _char: typeof(_408) = *const {l685} i8
        // 2266: b"\n\0" as *con ... _char:   l685 = UNIQUE | NON_NULL, (empty)
        // 2266: b"\n\0" as *const u8: typeof(_409) = *const {l687} u8
        // 2266: b"\n\0" as *const u8:   l687 = UNIQUE | NON_NULL, (empty)
        // 2266: b"\n\0": typeof(_410) = *const {l689} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2266: b"\n\0":   l689 = UNIQUE | NON_NULL, (empty)
        // 2266: b"\n\0": typeof(_411) = & {l691} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2266: b"\n\0":   l691 = UNIQUE | NON_NULL, FIXED
        // 2266: b"\n\0" as *const u8: typeof(_409 = move _410 as *const u8 (Pointer(ArrayToPointer))) = *const {l1193} u8
        // 2266: b"\n\0" as *const u8:   l1193 = UNIQUE | NON_NULL, (empty)
        // 2266: b"\n\0" as *con ... _char: typeof(_408 = move _409 as *const i8 (Misc)) = *const {l1194} i8
        // 2266: b"\n\0" as *con ... _char:   l1194 = UNIQUE | NON_NULL, (empty)
        // 2266: b"\n\0": typeof(_411 = const b"\n\x00") = & {l1191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2266: b"\n\0":   l1191 = UNIQUE | NON_NULL, (empty)
        // 2266: b"\n\0": typeof(_410 = &raw const (*_411)) = *const {l1192} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 2266: b"\n\0":   l1192 = UNIQUE | NON_NULL, (empty)
    } else {
        mac = hashmac() as libc::c_int;
        pid = getpid();
        setsighandlers();
        env.round = 0 as libc::c_int;
        // 2271: env: typeof(_416) = *mut {l697} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2271: env:   l697 = UNIQUE | NON_NULL, (empty)
        while env.round < max {
        // 2272: env: typeof(_419) = *mut {l701} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2272: env:   l701 = UNIQUE | NON_NULL, (empty)
            if prev & 1 as libc::c_int == 0 {
                prev += 1;
                prev;
            }
            env.seed = mac as libc::c_uint;
            // 2277: env: typeof(_429) = *mut {l712} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2277: env:   l712 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed).wrapping_mul(123301093 as libc::c_int as libc::c_uint);
            // 2278: env: typeof(_432) = *mut {l716} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2278: env:   l716 = UNIQUE | NON_NULL, (empty)
            // 2278: env: typeof(_435) = *mut {l720} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2278: env:   l720 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed as clock_t + times(0 as *mut tms)) as libc::c_uint;
            // 2279: env: typeof(_439) = *mut {l725} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2279: env:   l725 = UNIQUE | NON_NULL, (empty)
            // 2279: 0 as *mut tms: typeof(_441) = *mut {l728} DefId(0:291 ~ lglmbt[b165]::tms)
            // 2279: 0 as *mut tms:   l728 = UNIQUE | NON_NULL, (empty)
            // 2279: env: typeof(_443) = *mut {l731} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2279: env:   l731 = UNIQUE | NON_NULL, (empty)
            // 2279: 0 as *mut tms: typeof(_441 = const 0_usize as *mut tms (PointerFromExposedAddress)) = *mut {l1195} DefId(0:291 ~ lglmbt[b165]::tms)
            // 2279: 0 as *mut tms:   l1195 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed).wrapping_mul(223531513 as libc::c_int as libc::c_uint);
            // 2280: env: typeof(_446) = *mut {l735} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2280: env:   l735 = UNIQUE | NON_NULL, (empty)
            // 2280: env: typeof(_449) = *mut {l739} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2280: env:   l739 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed).wrapping_add(pid as libc::c_uint);
            // 2281: env: typeof(_452) = *mut {l743} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2281: env:   l743 = UNIQUE | NON_NULL, (empty)
            // 2281: env: typeof(_455) = *mut {l747} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2281: env:   l747 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed).wrapping_mul(31752023 as libc::c_int as libc::c_uint);
            // 2282: env: typeof(_458) = *mut {l751} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2282: env:   l751 = UNIQUE | NON_NULL, (empty)
            // 2282: env: typeof(_461) = *mut {l755} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2282: env:   l755 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed).wrapping_add(prev as libc::c_uint);
            // 2283: env: typeof(_464) = *mut {l759} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2283: env:   l759 = UNIQUE | NON_NULL, (empty)
            // 2283: env: typeof(_467) = *mut {l763} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2283: env:   l763 = UNIQUE | NON_NULL, (empty)
            env.seed = (env.seed).wrapping_mul(43376579 as libc::c_int as libc::c_uint);
            // 2284: env: typeof(_470) = *mut {l767} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2284: env:   l767 = UNIQUE | NON_NULL, (empty)
            // 2284: env: typeof(_473) = *mut {l771} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2284: env:   l771 = UNIQUE | NON_NULL, (empty)
            env.seed = env.seed >> 1 as libc::c_int;
            // 2285: env: typeof(_475) = *mut {l774} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2285: env:   l774 = UNIQUE | NON_NULL, (empty)
            // 2285: env: typeof(_478) = *mut {l778} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2285: env:   l778 = UNIQUE | NON_NULL, (empty)
            prev = env.seed as libc::c_int;
            // 2286: env: typeof(_480) = *mut {l781} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2286: env:   l781 = UNIQUE | NON_NULL, (empty)
            if env.quiet == 0 {
            // 2287: env: typeof(_484) = *mut {l786} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2287: env:   l786 = UNIQUE | NON_NULL, (empty)
                if env.terminal != 0 {
                // 2288: env: typeof(_488) = *mut {l791} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2288: env:   l791 = UNIQUE | NON_NULL, (empty)
                    erase();
                }
                printf(
                    b"%d %d \0" as *const u8 as *const libc::c_char,
                    // 2292: b"%d %d \0" as  ... _char: typeof(_491) = *const {l795} i8
                    // 2292: b"%d %d \0" as  ... _char:   l795 = UNIQUE | NON_NULL, (empty)
                    // 2292: b"%d %d \0" as  ... st u8: typeof(_492) = *const {l797} u8
                    // 2292: b"%d %d \0" as  ... st u8:   l797 = UNIQUE | NON_NULL, (empty)
                    // 2292: b"%d %d \0": typeof(_493) = *const {l799} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2292: b"%d %d \0":   l799 = UNIQUE | NON_NULL, (empty)
                    // 2292: b"%d %d \0": typeof(_494) = & {l801} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2292: b"%d %d \0":   l801 = UNIQUE | NON_NULL, FIXED
                    // 2292: b"%d %d \0": typeof(_493 = &raw const (*_494)) = *const {l1197} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2292: b"%d %d \0":   l1197 = UNIQUE | NON_NULL, (empty)
                    // 2292: b"%d %d \0": typeof(_494 = const b"%d %d \x00") = & {l1196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000007)) }]
                    // 2292: b"%d %d \0":   l1196 = UNIQUE | NON_NULL, (empty)
                    // 2292: b"%d %d \0" as  ... st u8: typeof(_492 = move _493 as *const u8 (Pointer(ArrayToPointer))) = *const {l1198} u8
                    // 2292: b"%d %d \0" as  ... st u8:   l1198 = UNIQUE | NON_NULL, (empty)
                    // 2292: b"%d %d \0" as  ... _char: typeof(_491 = move _492 as *const i8 (Misc)) = *const {l1199} i8
                    // 2292: b"%d %d \0" as  ... _char:   l1199 = UNIQUE | NON_NULL, (empty)
                    env.round,
                    // 2293: env: typeof(_496) = *mut {l804} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2293: env:   l804 = UNIQUE | NON_NULL, (empty)
                    env.seed,
                    // 2294: env: typeof(_498) = *mut {l807} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2294: env:   l807 = UNIQUE | NON_NULL, (empty)
                );
                fflush(stdout);
                // 2296: stdout: typeof(_500) = *mut {l810} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2296: stdout:   l810 = UNIQUE | NON_NULL, (empty)
                // 2296: stdout: typeof(_501) = *mut {l812} *mut {l813} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2296: stdout:   l812 = UNIQUE | NON_NULL, (empty)
                // 2296: stdout:   l813 = UNIQUE | NON_NULL, (empty)
            }
            res = run(
                &mut env,
                // 2299: &mut env: typeof(_503) = *mut {l816} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2299: &mut env:   l816 = UNIQUE | NON_NULL, (empty)
                // 2299: &mut env: typeof(_504) = &mut {l818} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2299: &mut env:   l818 = UNIQUE | NON_NULL, (empty)
                // 2299: env: typeof(_505) = *mut {l820} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2299: env:   l820 = UNIQUE | NON_NULL, (empty)
                // 2299: &mut env: typeof(_503 = &raw mut (*_504)) = *mut {l1201} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2299: &mut env:   l1201 = UNIQUE | NON_NULL, (empty)
                // 2299: &mut env: typeof(_504 = &mut (*_505)) = &mut {l1200} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2299: &mut env:   l1200 = UNIQUE | NON_NULL, (empty)
                Some(rantrav as unsafe extern "C" fn(*mut Env) -> ()),
                // 2300: Some(rantrav as ... > ()): typeof(_506) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l822} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
                // 2300: Some(rantrav as ... > ()):   l822 = UNIQUE | NON_NULL, (empty)
                // 2300: rantrav as unsa ... -> (): typeof(_507) = fn(*mut {l824} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
                // 2300: rantrav as unsa ... -> ():   l824 = UNIQUE | NON_NULL, (empty)
                // 2300: rantrav: typeof(_507 = rantrav as unsafe extern "C" fn(*mut Env) (Pointer(ReifyFnPointer))) = fn(*mut {l1202} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
                // 2300: rantrav:   l1202 = UNIQUE | NON_NULL, (empty)
                // 2300: Some(rantrav as ... > ()): typeof(_506 = std::option::Option::<unsafe extern "C" fn(*mut Env)>::Some(move _507)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1203} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
                // 2300: Some(rantrav as ... > ()):   l1203 = UNIQUE | NON_NULL, (empty)
            );
            if res > 0 as libc::c_int {
                let mut name: [libc::c_char; 100] = [0; 100];
                env.bugs += 1;
                // 2304: env: typeof(_513) = *mut {l831} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2304: env:   l831 = UNIQUE | NON_NULL, (empty)
                env.bugs;
                // 2305: env: typeof(_516) = *mut {l835} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2305: env:   l835 = UNIQUE | NON_NULL, (empty)
                sprintf(
                    name.as_mut_ptr(),
                    // 2307: name.as_mut_ptr(): typeof(_518) = *mut {l838} i8
                    // 2307: name.as_mut_ptr():   l838 = UNIQUE | NON_NULL, (empty)
                    // 2307: name.as_mut_ptr(): typeof(_519) = &mut {l840} [i8]
                    // 2307: name.as_mut_ptr():   l840 = UNIQUE | NON_NULL, FIXED
                    // 2307: name.as_mut_ptr(): typeof(_520) = &mut {l842} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                    // 2307: name.as_mut_ptr():   l842 = UNIQUE | NON_NULL, (empty)
                    // 2307: name.as_mut_ptr(): typeof(_519 = move _520 as &mut [i8] (Pointer(Unsize))) = &mut {l1205} [i8]
                    // 2307: name.as_mut_ptr():   l1205 = UNIQUE | NON_NULL, FIXED
                    // 2307: name.as_mut_ptr(): typeof(_520 = &mut _512) = &mut {l1204} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                    // 2307: name.as_mut_ptr():   l1204 = UNIQUE | NON_NULL, (empty)
                    b"/tmp/lglmbt-tmp-%d.trace\0" as *const u8 as *const libc::c_char,
                    // 2308: b"/tmp/lglmbt-t ... _char: typeof(_521) = *const {l844} i8
                    // 2308: b"/tmp/lglmbt-t ... _char:   l844 = UNIQUE | NON_NULL, (empty)
                    // 2308: b"/tmp/lglmbt-t ... st u8: typeof(_522) = *const {l846} u8
                    // 2308: b"/tmp/lglmbt-t ... st u8:   l846 = UNIQUE | NON_NULL, (empty)
                    // 2308: b"/tmp/lglmbt-t ... ce\0": typeof(_523) = *const {l848} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2308: b"/tmp/lglmbt-t ... ce\0":   l848 = UNIQUE | NON_NULL, (empty)
                    // 2308: b"/tmp/lglmbt-t ... ce\0": typeof(_524) = & {l850} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2308: b"/tmp/lglmbt-t ... ce\0":   l850 = UNIQUE | NON_NULL, FIXED
                    // 2308: b"/tmp/lglmbt-t ... st u8: typeof(_522 = move _523 as *const u8 (Pointer(ArrayToPointer))) = *const {l1208} u8
                    // 2308: b"/tmp/lglmbt-t ... st u8:   l1208 = UNIQUE | NON_NULL, (empty)
                    // 2308: b"/tmp/lglmbt-t ... ce\0": typeof(_523 = &raw const (*_524)) = *const {l1207} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2308: b"/tmp/lglmbt-t ... ce\0":   l1207 = UNIQUE | NON_NULL, (empty)
                    // 2308: b"/tmp/lglmbt-t ... _char: typeof(_521 = move _522 as *const i8 (Misc)) = *const {l1209} i8
                    // 2308: b"/tmp/lglmbt-t ... _char:   l1209 = UNIQUE | NON_NULL, (empty)
                    // 2308: b"/tmp/lglmbt-t ... ce\0": typeof(_524 = const b"/tmp/lglmbt-tmp-%d.trace\x00") = & {l1206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                    // 2308: b"/tmp/lglmbt-t ... ce\0":   l1206 = UNIQUE | NON_NULL, (empty)
                    getpid(),
                );
                env.file = fopen(
                // 2311: fopen( name.as_ ... ar, ): typeof(_526) = *mut {l853} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2311: fopen( name.as_ ... ar, ):   l853 = UNIQUE | NON_NULL, (empty)
                // 2311: env: typeof(_535) = *mut {l871} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2311: env:   l871 = UNIQUE | NON_NULL, (empty)
                    name.as_mut_ptr(),
                    // 2312: name.as_mut_ptr(): typeof(_527) = *const {l855} i8
                    // 2312: name.as_mut_ptr():   l855 = UNIQUE | NON_NULL, (empty)
                    // 2312: name.as_mut_ptr(): typeof(_528) = *mut {l857} i8
                    // 2312: name.as_mut_ptr():   l857 = UNIQUE | NON_NULL, (empty)
                    // 2312: name.as_mut_ptr(): typeof(_529) = &mut {l859} [i8]
                    // 2312: name.as_mut_ptr():   l859 = UNIQUE | NON_NULL, FIXED
                    // 2312: name.as_mut_ptr(): typeof(_530) = &mut {l861} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                    // 2312: name.as_mut_ptr():   l861 = UNIQUE | NON_NULL, (empty)
                    // 2312: name.as_mut_ptr(): typeof(_527 = move _528 as *const i8 (Pointer(MutToConstPointer))) = *const {l1212} i8
                    // 2312: name.as_mut_ptr():   l1212 = UNIQUE | NON_NULL, (empty)
                    // 2312: name.as_mut_ptr(): typeof(_530 = &mut _512) = &mut {l1210} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                    // 2312: name.as_mut_ptr():   l1210 = UNIQUE | NON_NULL, (empty)
                    // 2312: name.as_mut_ptr(): typeof(_529 = move _530 as &mut [i8] (Pointer(Unsize))) = &mut {l1211} [i8]
                    // 2312: name.as_mut_ptr():   l1211 = UNIQUE | NON_NULL, FIXED
                    b"w\0" as *const u8 as *const libc::c_char,
                    // 2313: b"w\0" as *cons ... _char: typeof(_531) = *const {l863} i8
                    // 2313: b"w\0" as *cons ... _char:   l863 = UNIQUE | NON_NULL, (empty)
                    // 2313: b"w\0" as *const u8: typeof(_532) = *const {l865} u8
                    // 2313: b"w\0" as *const u8:   l865 = UNIQUE | NON_NULL, (empty)
                    // 2313: b"w\0": typeof(_533) = *const {l867} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2313: b"w\0":   l867 = UNIQUE | NON_NULL, (empty)
                    // 2313: b"w\0": typeof(_534) = & {l869} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2313: b"w\0":   l869 = UNIQUE | NON_NULL, FIXED
                    // 2313: b"w\0" as *const u8: typeof(_532 = move _533 as *const u8 (Pointer(ArrayToPointer))) = *const {l1215} u8
                    // 2313: b"w\0" as *const u8:   l1215 = UNIQUE | NON_NULL, (empty)
                    // 2313: b"w\0" as *cons ... _char: typeof(_531 = move _532 as *const i8 (Misc)) = *const {l1216} i8
                    // 2313: b"w\0" as *cons ... _char:   l1216 = UNIQUE | NON_NULL, (empty)
                    // 2313: b"w\0": typeof(_533 = &raw const (*_534)) = *const {l1214} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2313: b"w\0":   l1214 = UNIQUE | NON_NULL, (empty)
                    // 2313: b"w\0": typeof(_534 = const b"w\x00") = & {l1213} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2313: b"w\0":   l1213 = UNIQUE | NON_NULL, (empty)
                );
                env.print = 0 as libc::c_int;
                // 2315: env: typeof(_537) = *mut {l874} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2315: env:   l874 = UNIQUE | NON_NULL, (empty)
                tmp = run(
                    &mut env,
                    // 2317: &mut env: typeof(_539) = *mut {l877} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2317: &mut env:   l877 = UNIQUE | NON_NULL, (empty)
                    // 2317: &mut env: typeof(_540) = &mut {l879} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2317: &mut env:   l879 = UNIQUE | NON_NULL, (empty)
                    // 2317: env: typeof(_541) = *mut {l881} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2317: env:   l881 = UNIQUE | NON_NULL, (empty)
                    // 2317: &mut env: typeof(_540 = &mut (*_541)) = &mut {l1217} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2317: &mut env:   l1217 = UNIQUE | NON_NULL, (empty)
                    // 2317: &mut env: typeof(_539 = &raw mut (*_540)) = *mut {l1218} DefId(0:327 ~ lglmbt[b165]::Env)
                    // 2317: &mut env:   l1218 = UNIQUE | NON_NULL, (empty)
                    Some(rantrav as unsafe extern "C" fn(*mut Env) -> ()),
                    // 2318: Some(rantrav as ... > ()): typeof(_542) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l883} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
                    // 2318: Some(rantrav as ... > ()):   l883 = UNIQUE | NON_NULL, (empty)
                    // 2318: rantrav as unsa ... -> (): typeof(_543) = fn(*mut {l885} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
                    // 2318: rantrav as unsa ... -> ():   l885 = UNIQUE | NON_NULL, (empty)
                    // 2318: Some(rantrav as ... > ()): typeof(_542 = std::option::Option::<unsafe extern "C" fn(*mut Env)>::Some(move _543)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l1220} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()>
                    // 2318: Some(rantrav as ... > ()):   l1220 = UNIQUE | NON_NULL, (empty)
                    // 2318: rantrav: typeof(_543 = rantrav as unsafe extern "C" fn(*mut Env) (Pointer(ReifyFnPointer))) = fn(*mut {l1219} DefId(0:327 ~ lglmbt[b165]::Env)) -> ()
                    // 2318: rantrav:   l1219 = UNIQUE | NON_NULL, (empty)
                );
                if tmp == res {
                } else {
                    __assert_fail(
                        b"tmp == res\0" as *const u8 as *const libc::c_char,
                        // 2323: b"tmp == res\0" ... _char: typeof(_550) = *const {l893} i8
                        // 2323: b"tmp == res\0" ... _char:   l893 = UNIQUE | NON_NULL, (empty)
                        // 2323: b"tmp == res\0" ... st u8: typeof(_551) = *const {l895} u8
                        // 2323: b"tmp == res\0" ... st u8:   l895 = UNIQUE | NON_NULL, (empty)
                        // 2323: b"tmp == res\0": typeof(_552) = *const {l897} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2323: b"tmp == res\0":   l897 = UNIQUE | NON_NULL, (empty)
                        // 2323: b"tmp == res\0": typeof(_553) = & {l899} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2323: b"tmp == res\0":   l899 = UNIQUE | NON_NULL, FIXED
                        // 2323: b"tmp == res\0" ... _char: typeof(_550 = move _551 as *const i8 (Misc)) = *const {l1224} i8
                        // 2323: b"tmp == res\0" ... _char:   l1224 = UNIQUE | NON_NULL, (empty)
                        // 2323: b"tmp == res\0": typeof(_553 = const b"tmp == res\x00") = & {l1221} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2323: b"tmp == res\0":   l1221 = UNIQUE | NON_NULL, (empty)
                        // 2323: b"tmp == res\0": typeof(_552 = &raw const (*_553)) = *const {l1222} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                        // 2323: b"tmp == res\0":   l1222 = UNIQUE | NON_NULL, (empty)
                        // 2323: b"tmp == res\0" ... st u8: typeof(_551 = move _552 as *const u8 (Pointer(ArrayToPointer))) = *const {l1223} u8
                        // 2323: b"tmp == res\0" ... st u8:   l1223 = UNIQUE | NON_NULL, (empty)
                        b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                        // 2324: b"lglmbt.c\0" a ... _char: typeof(_554) = *const {l901} i8
                        // 2324: b"lglmbt.c\0" a ... _char:   l901 = UNIQUE | NON_NULL, (empty)
                        // 2324: b"lglmbt.c\0" a ... st u8: typeof(_555) = *const {l903} u8
                        // 2324: b"lglmbt.c\0" a ... st u8:   l903 = UNIQUE | NON_NULL, (empty)
                        // 2324: b"lglmbt.c\0": typeof(_556) = *const {l905} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 2324: b"lglmbt.c\0":   l905 = UNIQUE | NON_NULL, (empty)
                        // 2324: b"lglmbt.c\0": typeof(_557) = & {l907} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 2324: b"lglmbt.c\0":   l907 = UNIQUE | NON_NULL, FIXED
                        // 2324: b"lglmbt.c\0" a ... _char: typeof(_554 = move _555 as *const i8 (Misc)) = *const {l1228} i8
                        // 2324: b"lglmbt.c\0" a ... _char:   l1228 = UNIQUE | NON_NULL, (empty)
                        // 2324: b"lglmbt.c\0": typeof(_556 = &raw const (*_557)) = *const {l1226} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 2324: b"lglmbt.c\0":   l1226 = UNIQUE | NON_NULL, (empty)
                        // 2324: b"lglmbt.c\0" a ... st u8: typeof(_555 = move _556 as *const u8 (Pointer(ArrayToPointer))) = *const {l1227} u8
                        // 2324: b"lglmbt.c\0" a ... st u8:   l1227 = UNIQUE | NON_NULL, (empty)
                        // 2324: b"lglmbt.c\0": typeof(_557 = const b"lglmbt.c\x00") = & {l1225} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                        // 2324: b"lglmbt.c\0":   l1225 = UNIQUE | NON_NULL, (empty)
                        751 as libc::c_int as libc::c_uint,
                        (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                        // 2326: (*::core::mem:: ... ptr(): typeof(_560) = *const {l911} i8
                        // 2326: (*::core::mem:: ... ptr():   l911 = UNIQUE | NON_NULL, (empty)
                        // 2326: (*::core::mem:: ... ptr(): typeof(_561) = & {l913} [i8]
                        // 2326: (*::core::mem:: ... ptr():   l913 = UNIQUE | NON_NULL, FIXED
                        // 2326: (*::core::mem:: ... ptr(): typeof(_562) = & {l915} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                        // 2326: (*::core::mem:: ... ptr():   l915 = UNIQUE | NON_NULL, (empty)
                        // 2326: ::core::mem::tr ... 0", ): typeof(_563) = & {l917} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                        // 2326: ::core::mem::tr ... 0", ):   l917 = UNIQUE | NON_NULL, FIXED
                        // 2326: (*::core::mem:: ... ptr(): typeof(_561 = move _562 as &[i8] (Pointer(Unsize))) = & {l1232} [i8]
                        // 2326: (*::core::mem:: ... ptr():   l1232 = UNIQUE | NON_NULL, FIXED
                        // 2326: (*::core::mem:: ... ptr(): typeof(_562 = &(*_563)) = & {l1231} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                        // 2326: (*::core::mem:: ... ptr():   l1231 = UNIQUE | NON_NULL, (empty)
                            b"int main(int, char **)\0",
                            // 2327: b"int main(int, ... *)\0": typeof(_564) = & {l919} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2327: b"int main(int, ... *)\0":   l919 = UNIQUE | NON_NULL, (empty)
                            // 2327: b"int main(int, ... *)\0": typeof(_565) = & {l921} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2327: b"int main(int, ... *)\0":   l921 = UNIQUE | NON_NULL, FIXED
                            // 2327: b"int main(int, ... *)\0": typeof(_565 = const b"int main(int, char **)\x00") = & {l1229} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2327: b"int main(int, ... *)\0":   l1229 = UNIQUE | NON_NULL, (empty)
                            // 2327: b"int main(int, ... *)\0": typeof(_564 = &(*_565)) = & {l1230} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2327: b"int main(int, ... *)\0":   l1230 = UNIQUE | NON_NULL, (empty)
                        ))
                        .as_ptr(),
                    );
                }
                'c_9379: {
                    if tmp == res {
                    } else {
                        __assert_fail(
                            b"tmp == res\0" as *const u8 as *const libc::c_char,
                            // 2336: b"tmp == res\0" ... _char: typeof(_572) = *const {l929} i8
                            // 2336: b"tmp == res\0" ... _char:   l929 = UNIQUE | NON_NULL, (empty)
                            // 2336: b"tmp == res\0" ... st u8: typeof(_573) = *const {l931} u8
                            // 2336: b"tmp == res\0" ... st u8:   l931 = UNIQUE | NON_NULL, (empty)
                            // 2336: b"tmp == res\0": typeof(_574) = *const {l933} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 2336: b"tmp == res\0":   l933 = UNIQUE | NON_NULL, (empty)
                            // 2336: b"tmp == res\0": typeof(_575) = & {l935} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 2336: b"tmp == res\0":   l935 = UNIQUE | NON_NULL, FIXED
                            // 2336: b"tmp == res\0": typeof(_574 = &raw const (*_575)) = *const {l1234} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 2336: b"tmp == res\0":   l1234 = UNIQUE | NON_NULL, (empty)
                            // 2336: b"tmp == res\0" ... _char: typeof(_572 = move _573 as *const i8 (Misc)) = *const {l1236} i8
                            // 2336: b"tmp == res\0" ... _char:   l1236 = UNIQUE | NON_NULL, (empty)
                            // 2336: b"tmp == res\0": typeof(_575 = const b"tmp == res\x00") = & {l1233} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
                            // 2336: b"tmp == res\0":   l1233 = UNIQUE | NON_NULL, (empty)
                            // 2336: b"tmp == res\0" ... st u8: typeof(_573 = move _574 as *const u8 (Pointer(ArrayToPointer))) = *const {l1235} u8
                            // 2336: b"tmp == res\0" ... st u8:   l1235 = UNIQUE | NON_NULL, (empty)
                            b"lglmbt.c\0" as *const u8 as *const libc::c_char,
                            // 2337: b"lglmbt.c\0" a ... _char: typeof(_576) = *const {l937} i8
                            // 2337: b"lglmbt.c\0" a ... _char:   l937 = UNIQUE | NON_NULL, (empty)
                            // 2337: b"lglmbt.c\0" a ... st u8: typeof(_577) = *const {l939} u8
                            // 2337: b"lglmbt.c\0" a ... st u8:   l939 = UNIQUE | NON_NULL, (empty)
                            // 2337: b"lglmbt.c\0": typeof(_578) = *const {l941} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 2337: b"lglmbt.c\0":   l941 = UNIQUE | NON_NULL, (empty)
                            // 2337: b"lglmbt.c\0": typeof(_579) = & {l943} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 2337: b"lglmbt.c\0":   l943 = UNIQUE | NON_NULL, FIXED
                            // 2337: b"lglmbt.c\0" a ... _char: typeof(_576 = move _577 as *const i8 (Misc)) = *const {l1240} i8
                            // 2337: b"lglmbt.c\0" a ... _char:   l1240 = UNIQUE | NON_NULL, (empty)
                            // 2337: b"lglmbt.c\0": typeof(_579 = const b"lglmbt.c\x00") = & {l1237} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 2337: b"lglmbt.c\0":   l1237 = UNIQUE | NON_NULL, (empty)
                            // 2337: b"lglmbt.c\0": typeof(_578 = &raw const (*_579)) = *const {l1238} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
                            // 2337: b"lglmbt.c\0":   l1238 = UNIQUE | NON_NULL, (empty)
                            // 2337: b"lglmbt.c\0" a ... st u8: typeof(_577 = move _578 as *const u8 (Pointer(ArrayToPointer))) = *const {l1239} u8
                            // 2337: b"lglmbt.c\0" a ... st u8:   l1239 = UNIQUE | NON_NULL, (empty)
                            751 as libc::c_int as libc::c_uint,
                            (*::core::mem::transmute::<&[u8; 23], &[libc::c_char; 23]>(
                            // 2339: (*::core::mem:: ... ptr(): typeof(_582) = *const {l947} i8
                            // 2339: (*::core::mem:: ... ptr():   l947 = UNIQUE | NON_NULL, (empty)
                            // 2339: (*::core::mem:: ... ptr(): typeof(_583) = & {l949} [i8]
                            // 2339: (*::core::mem:: ... ptr():   l949 = UNIQUE | NON_NULL, FIXED
                            // 2339: (*::core::mem:: ... ptr(): typeof(_584) = & {l951} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2339: (*::core::mem:: ... ptr():   l951 = UNIQUE | NON_NULL, (empty)
                            // 2339: ::core::mem::tr ... 0", ): typeof(_585) = & {l953} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2339: ::core::mem::tr ... 0", ):   l953 = UNIQUE | NON_NULL, FIXED
                            // 2339: (*::core::mem:: ... ptr(): typeof(_583 = move _584 as &[i8] (Pointer(Unsize))) = & {l1244} [i8]
                            // 2339: (*::core::mem:: ... ptr():   l1244 = UNIQUE | NON_NULL, FIXED
                            // 2339: (*::core::mem:: ... ptr(): typeof(_584 = &(*_585)) = & {l1243} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                            // 2339: (*::core::mem:: ... ptr():   l1243 = UNIQUE | NON_NULL, (empty)
                                b"int main(int, char **)\0",
                                // 2340: b"int main(int, ... *)\0": typeof(_586) = & {l955} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                // 2340: b"int main(int, ... *)\0":   l955 = UNIQUE | NON_NULL, (empty)
                                // 2340: b"int main(int, ... *)\0": typeof(_587) = & {l957} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                // 2340: b"int main(int, ... *)\0":   l957 = UNIQUE | NON_NULL, FIXED
                                // 2340: b"int main(int, ... *)\0": typeof(_587 = const b"int main(int, char **)\x00") = & {l1241} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                // 2340: b"int main(int, ... *)\0":   l1241 = UNIQUE | NON_NULL, (empty)
                                // 2340: b"int main(int, ... *)\0": typeof(_586 = &(*_587)) = & {l1242} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000017)) }]
                                // 2340: b"int main(int, ... *)\0":   l1242 = UNIQUE | NON_NULL, (empty)
                            ))
                            .as_ptr(),
                        );
                    }
                };
                fclose(env.file);
                // 2346: env.file: typeof(_589) = *mut {l960} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2346: env.file:   l960 = UNIQUE | NON_NULL, (empty)
                // 2346: env: typeof(_590) = *mut {l962} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2346: env:   l962 = UNIQUE | NON_NULL, (empty)
                env.file = 0 as *mut FILE;
                // 2347: env: typeof(_591) = *mut {l964} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2347: env:   l964 = UNIQUE | NON_NULL, (empty)
                // 2347: env.file = 0 as ...  FILE: typeof(((*_591).0: *mut _IO_FILE) = const 0_usize as *mut _IO_FILE (PointerFromExposedAddress)) = *mut {l1245} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2347: env.file = 0 as ...  FILE:   l1245 = UNIQUE | NON_NULL, (empty)
                dd(&mut env, name.as_mut_ptr(), res, opt);
                // 2348: &mut env: typeof(_593) = *mut {l967} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2348: &mut env:   l967 = UNIQUE | NON_NULL, (empty)
                // 2348: &mut env: typeof(_594) = &mut {l969} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2348: &mut env:   l969 = UNIQUE | NON_NULL, (empty)
                // 2348: env: typeof(_595) = *mut {l971} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2348: env:   l971 = UNIQUE | NON_NULL, (empty)
                // 2348: name.as_mut_ptr(): typeof(_596) = *const {l973} i8
                // 2348: name.as_mut_ptr():   l973 = UNIQUE | NON_NULL, (empty)
                // 2348: name.as_mut_ptr(): typeof(_597) = *mut {l975} i8
                // 2348: name.as_mut_ptr():   l975 = UNIQUE | NON_NULL, (empty)
                // 2348: name.as_mut_ptr(): typeof(_598) = &mut {l977} [i8]
                // 2348: name.as_mut_ptr():   l977 = UNIQUE | NON_NULL, FIXED
                // 2348: name.as_mut_ptr(): typeof(_599) = &mut {l979} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                // 2348: name.as_mut_ptr():   l979 = UNIQUE | NON_NULL, (empty)
                // 2348: &mut env: typeof(_593 = &raw mut (*_594)) = *mut {l1247} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2348: &mut env:   l1247 = UNIQUE | NON_NULL, (empty)
                // 2348: name.as_mut_ptr(): typeof(_596 = move _597 as *const i8 (Pointer(MutToConstPointer))) = *const {l1250} i8
                // 2348: name.as_mut_ptr():   l1250 = UNIQUE | NON_NULL, (empty)
                // 2348: name.as_mut_ptr(): typeof(_599 = &mut _512) = &mut {l1248} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                // 2348: name.as_mut_ptr():   l1248 = UNIQUE | NON_NULL, (empty)
                // 2348: &mut env: typeof(_594 = &mut (*_595)) = &mut {l1246} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2348: &mut env:   l1246 = UNIQUE | NON_NULL, (empty)
                // 2348: name.as_mut_ptr(): typeof(_598 = move _599 as &mut [i8] (Pointer(Unsize))) = &mut {l1249} [i8]
                // 2348: name.as_mut_ptr():   l1249 = UNIQUE | NON_NULL, FIXED
                unlink(name.as_mut_ptr());
                // 2349: name.as_mut_ptr(): typeof(_603) = *const {l984} i8
                // 2349: name.as_mut_ptr():   l984 = UNIQUE | NON_NULL, (empty)
                // 2349: name.as_mut_ptr(): typeof(_604) = *mut {l986} i8
                // 2349: name.as_mut_ptr():   l986 = UNIQUE | NON_NULL, (empty)
                // 2349: name.as_mut_ptr(): typeof(_605) = &mut {l988} [i8]
                // 2349: name.as_mut_ptr():   l988 = UNIQUE | NON_NULL, FIXED
                // 2349: name.as_mut_ptr(): typeof(_606) = &mut {l990} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                // 2349: name.as_mut_ptr():   l990 = UNIQUE | NON_NULL, (empty)
                // 2349: name.as_mut_ptr(): typeof(_603 = move _604 as *const i8 (Pointer(MutToConstPointer))) = *const {l1253} i8
                // 2349: name.as_mut_ptr():   l1253 = UNIQUE | NON_NULL, (empty)
                // 2349: name.as_mut_ptr(): typeof(_606 = &mut _512) = &mut {l1251} [i8; Const { ty: usize, kind: Value(Leaf(0x0000000000000064)) }]
                // 2349: name.as_mut_ptr():   l1251 = UNIQUE | NON_NULL, (empty)
                // 2349: name.as_mut_ptr(): typeof(_605 = move _606 as &mut [i8] (Pointer(Unsize))) = &mut {l1252} [i8]
                // 2349: name.as_mut_ptr():   l1252 = UNIQUE | NON_NULL, FIXED
                env.print = (env.quiet == 0) as libc::c_int;
                // 2350: env: typeof(_609) = *mut {l994} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2350: env:   l994 = UNIQUE | NON_NULL, (empty)
                // 2350: env: typeof(_610) = *mut {l996} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2350: env:   l996 = UNIQUE | NON_NULL, (empty)
            }
            if env.quiet == 0 {
            // 2352: env: typeof(_614) = *mut {l1001} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2352: env:   l1001 = UNIQUE | NON_NULL, (empty)
                if res != 0 || env.terminal == 0 {
                // 2353: env: typeof(_621) = *mut {l1009} DefId(0:327 ~ lglmbt[b165]::Env)
                // 2353: env:   l1009 = UNIQUE | NON_NULL, (empty)
                    printf(b"\n\0" as *const u8 as *const libc::c_char);
                    // 2354: b"\n\0" as *con ... _char: typeof(_623) = *const {l1012} i8
                    // 2354: b"\n\0" as *con ... _char:   l1012 = UNIQUE | NON_NULL, (empty)
                    // 2354: b"\n\0" as *const u8: typeof(_624) = *const {l1014} u8
                    // 2354: b"\n\0" as *const u8:   l1014 = UNIQUE | NON_NULL, (empty)
                    // 2354: b"\n\0": typeof(_625) = *const {l1016} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2354: b"\n\0":   l1016 = UNIQUE | NON_NULL, (empty)
                    // 2354: b"\n\0": typeof(_626) = & {l1018} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2354: b"\n\0":   l1018 = UNIQUE | NON_NULL, FIXED
                    // 2354: b"\n\0" as *const u8: typeof(_624 = move _625 as *const u8 (Pointer(ArrayToPointer))) = *const {l1256} u8
                    // 2354: b"\n\0" as *const u8:   l1256 = UNIQUE | NON_NULL, (empty)
                    // 2354: b"\n\0": typeof(_625 = &raw const (*_626)) = *const {l1255} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2354: b"\n\0":   l1255 = UNIQUE | NON_NULL, (empty)
                    // 2354: b"\n\0" as *con ... _char: typeof(_623 = move _624 as *const i8 (Misc)) = *const {l1257} i8
                    // 2354: b"\n\0" as *con ... _char:   l1257 = UNIQUE | NON_NULL, (empty)
                    // 2354: b"\n\0": typeof(_626 = const b"\n\x00") = & {l1254} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
                    // 2354: b"\n\0":   l1254 = UNIQUE | NON_NULL, (empty)
                }
                fflush(stdout);
                // 2356: stdout: typeof(_628) = *mut {l1021} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2356: stdout:   l1021 = UNIQUE | NON_NULL, (empty)
                // 2356: stdout: typeof(_629) = *mut {l1023} *mut {l1024} DefId(0:248 ~ lglmbt[b165]::_IO_FILE)
                // 2356: stdout:   l1023 = UNIQUE | NON_NULL, (empty)
                // 2356: stdout:   l1024 = UNIQUE | NON_NULL, (empty)
            }
            if res != 0 && env.first != 0 {
            // 2358: env: typeof(_636) = *mut {l1032} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2358: env:   l1032 = UNIQUE | NON_NULL, (empty)
                break;
            }
            env.round += 1;
            // 2361: env: typeof(_638) = *mut {l1035} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2361: env:   l1035 = UNIQUE | NON_NULL, (empty)
            env.round;
            // 2362: env: typeof(_641) = *mut {l1039} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2362: env:   l1039 = UNIQUE | NON_NULL, (empty)
        }
    }
    if env.quiet == 0 {
    // 2365: env: typeof(_648) = *mut {l1047} DefId(0:327 ~ lglmbt[b165]::Env)
    // 2365: env:   l1047 = UNIQUE | NON_NULL, (empty)
        if env.terminal != 0 {
        // 2366: env: typeof(_652) = *mut {l1052} DefId(0:327 ~ lglmbt[b165]::Env)
        // 2366: env:   l1052 = UNIQUE | NON_NULL, (empty)
            erase();
        }
        printf(
            b"forked %d\n\0" as *const u8 as *const libc::c_char,
            // 2370: b"forked %d\n\0 ... _char: typeof(_655) = *const {l1056} i8
            // 2370: b"forked %d\n\0 ... _char:   l1056 = UNIQUE | NON_NULL, (empty)
            // 2370: b"forked %d\n\0 ... st u8: typeof(_656) = *const {l1058} u8
            // 2370: b"forked %d\n\0 ... st u8:   l1058 = UNIQUE | NON_NULL, (empty)
            // 2370: b"forked %d\n\0": typeof(_657) = *const {l1060} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2370: b"forked %d\n\0":   l1060 = UNIQUE | NON_NULL, (empty)
            // 2370: b"forked %d\n\0": typeof(_658) = & {l1062} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2370: b"forked %d\n\0":   l1062 = UNIQUE | NON_NULL, FIXED
            // 2370: b"forked %d\n\0 ... _char: typeof(_655 = move _656 as *const i8 (Misc)) = *const {l1261} i8
            // 2370: b"forked %d\n\0 ... _char:   l1261 = UNIQUE | NON_NULL, (empty)
            // 2370: b"forked %d\n\0 ... st u8: typeof(_656 = move _657 as *const u8 (Pointer(ArrayToPointer))) = *const {l1260} u8
            // 2370: b"forked %d\n\0 ... st u8:   l1260 = UNIQUE | NON_NULL, (empty)
            // 2370: b"forked %d\n\0": typeof(_658 = const b"forked %d\n\x00") = & {l1258} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2370: b"forked %d\n\0":   l1258 = UNIQUE | NON_NULL, (empty)
            // 2370: b"forked %d\n\0": typeof(_657 = &raw const (*_658)) = *const {l1259} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000b)) }]
            // 2370: b"forked %d\n\0":   l1259 = UNIQUE | NON_NULL, (empty)
            env.forked,
            // 2371: env: typeof(_660) = *mut {l1065} DefId(0:327 ~ lglmbt[b165]::Env)
            // 2371: env:   l1065 = UNIQUE | NON_NULL, (empty)
        );
    }
    stats();
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    // 2378: mut args: typeof(_1) = DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l1} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2378: mut args:   l1 = UNIQUE | NON_NULL, (empty)
    for arg in ::std::env::args() {
    // 2379: ::std::env::args(): typeof(_9) = &mut {l10} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2379: ::std::env::args():   l10 = UNIQUE | NON_NULL, (empty)
    // 2379: ::std::env::args(): typeof(_10) = &mut {l12} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2379: ::std::env::args():   l12 = UNIQUE | NON_NULL, (empty)
    // 2379: ::std::env::args(): typeof(_9 = &mut (*_10)) = &mut {l52} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2379: ::std::env::args():   l52 = UNIQUE | NON_NULL, (empty)
    // 2379: ::std::env::args(): typeof(_10 = &mut _5) = &mut {l51} DefId(1:2460 ~ std[c5ae]::env::Args)
    // 2379: ::std::env::args():   l51 = UNIQUE | NON_NULL, (empty)
        args.push(
        // 2380: args.push( (::s ... (), ): typeof(_15) = &mut {l18} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l19} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 2380: args.push( (::s ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 2380: args.push( (::s ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 2380: args.push( (::s ... (), ): typeof(_15 = &mut _1) = &mut {l53} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l54} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
        // 2380: args.push( (::s ... (), ):   l53 = UNIQUE | NON_NULL, (empty)
        // 2380: args.push( (::s ... (), ):   l54 = UNIQUE | NON_NULL, (empty)
            (::std::ffi::CString::new(arg))
            // 2381: (::std::ffi::CS ... raw(): typeof(_16) = *mut {l21} i8
            // 2381: (::std::ffi::CS ... raw():   l21 = UNIQUE | NON_NULL, (empty)
                .expect("Failed to convert argument into CString.")
                // 2382: "Failed to conv ... ing.": typeof(_20) = & {l26} str
                // 2382: "Failed to conv ... ing.":   l26 = UNIQUE | NON_NULL, (empty)
                // 2382: "Failed to conv ... ing.": typeof(_21) = & {l28} str
                // 2382: "Failed to conv ... ing.":   l28 = UNIQUE | NON_NULL, FIXED
                // 2382: "Failed to conv ... ing.": typeof(_20 = &(*_21)) = & {l56} str
                // 2382: "Failed to conv ... ing.":   l56 = UNIQUE | NON_NULL, (empty)
                // 2382: "Failed to conv ... ing.": typeof(_21 = const "Failed to convert argument into CString.") = & {l55} str
                // 2382: "Failed to conv ... ing.":   l55 = UNIQUE | NON_NULL, (empty)
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    // 2386: args.push(::cor ... ut()): typeof(_23) = &mut {l31} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l32} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2386: args.push(::cor ... ut()):   l31 = UNIQUE | NON_NULL, (empty)
    // 2386: args.push(::cor ... ut()):   l32 = UNIQUE | NON_NULL, (empty)
    // 2386: ::core::ptr::nu ... mut(): typeof(_24) = *mut {l34} i8
    // 2386: ::core::ptr::nu ... mut():   l34 = UNIQUE | NON_NULL, (empty)
    // 2386: args.push(::cor ... ut()): typeof(_23 = &mut _1) = &mut {l57} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l58} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
    // 2386: args.push(::cor ... ut()):   l57 = UNIQUE | NON_NULL, (empty)
    // 2386: args.push(::cor ... ut()):   l58 = UNIQUE | NON_NULL, (empty)
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            // 2389: args.len(): typeof(_30) = & {l41} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l42} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2389: args.len():   l41 = UNIQUE | NON_NULL, (empty)
            // 2389: args.len():   l42 = UNIQUE | NON_NULL, (empty)
            // 2389: args.len(): typeof(_30 = &_1) = & {l59} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l60} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2389: args.len():   l59 = UNIQUE | NON_NULL, (empty)
            // 2389: args.len():   l60 = UNIQUE | NON_NULL, (empty)
            args.as_mut_ptr() as *mut *mut libc::c_char,
            // 2390: args.as_mut_ptr ... _char: typeof(_32) = *mut {l45} *mut {l46} i8
            // 2390: args.as_mut_ptr ... _char:   l45 = UNIQUE | NON_NULL, (empty)
            // 2390: args.as_mut_ptr ... _char:   l46 = UNIQUE | NON_NULL, (empty)
            // 2390: args.as_mut_ptr(): typeof(_33) = &mut {l48} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l49} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2390: args.as_mut_ptr():   l48 = UNIQUE | NON_NULL, (empty)
            // 2390: args.as_mut_ptr():   l49 = UNIQUE | NON_NULL, (empty)
            // 2390: args.as_mut_ptr(): typeof(_33 = &mut _1) = &mut {l61} DefId(5:7780 ~ alloc[3757]::vec::Vec)<*mut {l62} i8, DefId(5:8106 ~ alloc[3757]::alloc::Global)>
            // 2390: args.as_mut_ptr():   l61 = UNIQUE | NON_NULL, (empty)
            // 2390: args.as_mut_ptr():   l62 = UNIQUE | NON_NULL, (empty)
        ) as i32)
    }
}
