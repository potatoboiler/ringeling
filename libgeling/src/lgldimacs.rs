#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 53: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g1480} i8
    // 53: _IO_read_ptr:   g1480 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 54: _IO_read_end: typeof(_IO_read_end) = *mut {g1481} i8
    // 54: _IO_read_end:   g1481 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 55: _IO_read_base: typeof(_IO_read_base) = *mut {g1482} i8
    // 55: _IO_read_base:   g1482 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 56: _IO_write_base: typeof(_IO_write_base) = *mut {g1483} i8
    // 56: _IO_write_base:   g1483 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 57: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g1484} i8
    // 57: _IO_write_ptr:   g1484 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 58: _IO_write_end: typeof(_IO_write_end) = *mut {g1485} i8
    // 58: _IO_write_end:   g1485 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 59: _IO_buf_base: typeof(_IO_buf_base) = *mut {g1486} i8
    // 59: _IO_buf_base:   g1486 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 60: _IO_buf_end: typeof(_IO_buf_end) = *mut {g1487} i8
    // 60: _IO_buf_end:   g1487 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 61: _IO_save_base: typeof(_IO_save_base) = *mut {g1488} i8
    // 61: _IO_save_base:   g1488 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 62: _IO_backup_base: typeof(_IO_backup_base) = *mut {g1489} i8
    // 62: _IO_backup_base:   g1489 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 63: _IO_save_end: typeof(_IO_save_end) = *mut {g1490} i8
    // 63: _IO_save_end:   g1490 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 64: _markers: typeof(_markers) = *mut {g1491} lgldimacs::_IO_marker
    // 64: _markers:   g1491 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 65: _chain: typeof(_chain) = *mut {g1492} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 65: _chain:   g1492 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 72: _lock: typeof(_lock) = *mut {g1493} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 72: _lock:   g1493 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 74: _codecvt: typeof(_codecvt) = *mut {g1494} lgldimacs::_IO_codecvt
    // 74: _codecvt:   g1494 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 75: _wide_data: typeof(_wide_data) = *mut {g1495} lgldimacs::_IO_wide_data
    // 75: _wide_data:   g1495 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 76: _freeres_list: typeof(_freeres_list) = *mut {g1496} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 76: _freeres_list:   g1496 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 77: _freeres_buf: typeof(_freeres_buf) = *mut {g1497} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 77: _freeres_buf:   g1497 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LDR<'h0,'h1,'h2,'h10,'h11,'h12,'h13,'h14,'h7,'h8,'h9,'h5,'h6,'h3,'h4> {
    pub mem: lgldimacs::C2RustUnnamed_2<'h10,'h11,'h12,'h13,'h14>,
    pub opt: lgldimacs::C2RustUnnamed_1<'h7,'h8,'h9>,
    pub header: lgldimacs::C2RustUnnamed_0<'h5,'h6>,
    pub add: lgldimacs::C2RustUnnamed<'h3,'h4>,
    pub path: *mut libc::c_char,
    // 91: path: typeof(path) = *mut {g1498} i8
    // 91: path:   g1498 = UNIQUE | NON_NULL, FIXED
    pub errmsg: *mut libc::c_char,
    // 92: errmsg: typeof(errmsg) = *mut {g1499} i8
    // 92: errmsg:   g1499 = UNIQUE | NON_NULL, FIXED
    pub closefile: libc::c_int,
    pub file: *mut FILE,
    // 94: file: typeof(file) = *mut {g1500} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 94: file:   g1500 = UNIQUE | NON_NULL, FIXED
    pub lineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed<'h3,'h4> {
    pub state: *mut libc::c_void,
    // 100: state: typeof(state) = *mut {g1501} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 100: state:   g1501 = UNIQUE | NON_NULL, FIXED
    pub fun: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, i32)>,
    // 101: fun: typeof(fun) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g36} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
    // 101: fun:   g36 = UNIQUE | NON_NULL, FIXED
}
pub type ldradd = Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0<'h5,'h6> {
    pub state: *mut libc::c_void,
    // 107: state: typeof(state) = *mut {g1502} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 107: state:   g1502 = UNIQUE | NON_NULL, FIXED
    pub fun: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, i32, i32)>,
    // 108: fun: typeof(fun) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g33} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
    // 108: fun:   g33 = UNIQUE | NON_NULL, FIXED
}
pub type ldrheader =
    Option<unsafe extern "C" fn(*mut libc::c_void, libc::c_int, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1<'h7,'h8,'h9> {
    pub state: &'h7 (libc::c_void),
    // 115: state: typeof(state) = *mut {g1503} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 115: state:   g1503 = UNIQUE | NON_NULL, (empty)
    pub fun: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, *const i8, i32)>,
    // 116: fun: typeof(fun) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g29} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {g30} i8, i32) -> ()>
    // 116: fun:   g29 = UNIQUE | NON_NULL, (empty)
    // 116: fun:   g30 = UNIQUE | NON_NULL, (empty)
}
pub type ldropt =
    Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2<'h10,'h11,'h12,'h13,'h14> {
    pub state: *mut libc::c_void,
    // 123: state: typeof(state) = *mut {g1504} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 123: state:   g1504 = UNIQUE | NON_NULL, FIXED
    pub alloc: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>,
    // 124: alloc: typeof(alloc) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g1505} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {g1506} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 124: alloc:   g1505 = UNIQUE | NON_NULL, FIXED
    // 124: alloc:   g1506 = UNIQUE | NON_NULL, FIXED
    pub dealloc: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64)>,
    // 125: dealloc: typeof(dealloc) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g1507} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {g1508} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
    // 125: dealloc:   g1507 = UNIQUE | NON_NULL, FIXED
    // 125: dealloc:   g1508 = UNIQUE | NON_NULL, FIXED
}
pub type ldrdealloc =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> ()>;
pub type ldralloc = Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void>;
pub type ldrealloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub parsed: libc::c_int,
    pub specified: libc::c_int,
}
pub const _ISdigit: C2RustUnnamed_4 = 2048;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_4 = 8;
pub const _ISpunct: C2RustUnnamed_4 = 4;
pub const _IScntrl: C2RustUnnamed_4 = 2;
pub const _ISblank: C2RustUnnamed_4 = 1;
pub const _ISgraph: C2RustUnnamed_4 = 32768;
pub const _ISprint: C2RustUnnamed_4 = 16384;
pub const _ISspace: C2RustUnnamed_4 = 8192;
pub const _ISxdigit: C2RustUnnamed_4 = 4096;
pub const _ISalpha: C2RustUnnamed_4 = 1024;
pub const _ISlower: C2RustUnnamed_4 = 512;
pub const _ISupper: C2RustUnnamed_4 = 256;
unsafe extern "C" fn ldrstdalloc<'h0,'h1>(
    mut mem: &'h0 (libc::c_void),
    // 172: mut mem: typeof(_1) = *mut {g4} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 172: mut mem:   g4 = UNIQUE | NON_NULL, (empty)
    mut bytes: size_t,
) -> &'h1 (libc::c_void) {
// 174: *mut libc::c_void: typeof(_0) = *mut {g5} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 174: *mut libc::c_void:   g5 = UNIQUE | NON_NULL, (empty)
    return malloc(bytes);
}
unsafe fn ldrstdalloc_shim(arg0: *mut libc::c_void, arg1: size_t) -> *mut libc::c_void {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_arg1 = arg1;
    let safe_result = ldrstdalloc(safe_arg0,safe_arg1);
    let result = core::ptr::addr_of!(*safe_result).cast_mut();
    result
}
}

unsafe extern "C" fn ldrstdealloc<'h0,'h1>(
    mut mem: &'h0 (libc::c_void),
    // 178: mut mem: typeof(_1) = *mut {g6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 178: mut mem:   g6 = UNIQUE | NON_NULL, (empty)
    mut ptr: &'h1 (libc::c_void),
    // 179: mut ptr: typeof(_2) = *mut {g7} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 179: mut ptr:   g7 = UNIQUE | FREE | NON_NULL, (empty)
    mut bytes: size_t,
) {
    free(ptr);
    // 182: ptr: typeof(_5) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 182: ptr:   l5 = UNIQUE | FREE | NON_NULL, (empty)
}
unsafe fn ldrstdealloc_shim(arg0: *mut libc::c_void, arg1: *mut libc::c_void, arg2: size_t) {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_arg1 = &*arg1.cast_const();
    let safe_arg2 = arg2;
    let safe_result = ldrstdealloc(safe_arg0,safe_arg1,safe_arg2);
    let result = safe_result;
    result
}
}

unsafe extern "C" fn ldrstdrealloc<'h0,'h1,'h2>(
    mut mem: &'h0 (libc::c_void),
    // 185: mut mem: typeof(_1) = *mut {g8} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 185: mut mem:   g8 = UNIQUE | NON_NULL, (empty)
    mut ptr: &'h1 (libc::c_void),
    // 186: mut ptr: typeof(_2) = *mut {g9} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 186: mut ptr:   g9 = UNIQUE | FREE | NON_NULL, (empty)
    mut ob: size_t,
    mut nb: size_t,
) -> &'h2 (libc::c_void) {
// 189: *mut libc::c_void: typeof(_0) = *mut {g10} DefId(2:5583 ~ core[480a]::ffi::c_void)
// 189: *mut libc::c_void:   g10 = UNIQUE | NON_NULL, (empty)
    return realloc(ptr, nb);
    // 190: ptr: typeof(_6) = *mut {l6} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 190: ptr:   l6 = UNIQUE | FREE | NON_NULL, (empty)
}
unsafe fn ldrstdrealloc_shim(arg0: *mut libc::c_void, arg1: *mut libc::c_void, arg2: size_t, arg3: size_t) -> *mut libc::c_void {
    {
    let safe_arg0 = &*arg0.cast_const();
    let safe_arg1 = &*arg1.cast_const();
    let safe_arg2 = arg2;
    let safe_arg3 = arg3;
    let safe_result = ldrstdrealloc(safe_arg0,safe_arg1,safe_arg2,safe_arg3);
    let result = core::ptr::addr_of!(*safe_result).cast_mut();
    result
}
}

#[no_mangle]
pub unsafe extern "C" fn ldrinit() -> *mut LDR {
// 193: *mut LDR: typeof(_0) = *mut {g20} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 193: *mut LDR:   g20 = UNIQUE | NON_NULL, FIXED
    return ldrminit(
        0 as *mut libc::c_void,
        // 195: 0 as *mut libc: ... _void: typeof(_2) = *mut {l2} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 195: 0 as *mut libc: ... _void:   l2 = UNIQUE | NON_NULL, (empty)
        // 195: 0 as *mut libc: ... _void: typeof(_2 = const 0_usize as *mut libc::c_void (PointerFromExposedAddress)) = *mut {l24} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 195: 0 as *mut libc: ... _void:   l24 = UNIQUE | NON_NULL, (empty)
        Some(ldrstdalloc_shim as unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void),
        // 196: Some(ldrstdallo ... void): typeof(_3) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l4} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 196: Some(ldrstdallo ... void):   l4 = UNIQUE | NON_NULL, (empty)
        // 196: Some(ldrstdallo ... void):   l5 = UNIQUE | NON_NULL, (empty)
        // 196: ldrstdalloc as  ... _void: typeof(_4) = fn(*mut {l7} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 196: ldrstdalloc as  ... _void:   l7 = UNIQUE | NON_NULL, (empty)
        // 196: ldrstdalloc as  ... _void:   l8 = UNIQUE | NON_NULL, (empty)
        // 196: Some(ldrstdallo ... void): typeof(_3 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void>::Some(move _4)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l27} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l28} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 196: Some(ldrstdallo ... void):   l27 = UNIQUE | NON_NULL, (empty)
        // 196: Some(ldrstdallo ... void):   l28 = UNIQUE | NON_NULL, (empty)
        // 196: ldrstdalloc: typeof(_4 = lgldimacs::ldrstdalloc as unsafe extern "C" fn(*mut libc::c_void, u64) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l25} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l26} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 196: ldrstdalloc:   l25 = UNIQUE | NON_NULL, (empty)
        // 196: ldrstdalloc:   l26 = UNIQUE | NON_NULL, (empty)
        Some(
        // 197: Some( ldrstdrea ... id, ): typeof(_5) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l10} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 197: Some( ldrstdrea ... id, ):   l10 = UNIQUE | NON_NULL, (empty)
        // 197: Some( ldrstdrea ... id, ):   l11 = UNIQUE | NON_NULL, (empty)
        // 197: Some( ldrstdrea ... id, ):   l12 = UNIQUE | NON_NULL, (empty)
        // 197: Some( ldrstdrea ... id, ): typeof(_5 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void>::Some(move _6)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l33} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l34} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 197: Some( ldrstdrea ... id, ):   l32 = UNIQUE | NON_NULL, (empty)
        // 197: Some( ldrstdrea ... id, ):   l33 = UNIQUE | NON_NULL, (empty)
        // 197: Some( ldrstdrea ... id, ):   l34 = UNIQUE | NON_NULL, (empty)
            ldrstdrealloc_shim
            // 198: ldrstdrealloc a ... _void: typeof(_6) = fn(*mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l15} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l16} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 198: ldrstdrealloc a ... _void:   l14 = UNIQUE | NON_NULL, (empty)
            // 198: ldrstdrealloc a ... _void:   l15 = UNIQUE | NON_NULL, (empty)
            // 198: ldrstdrealloc a ... _void:   l16 = UNIQUE | NON_NULL, (empty)
            // 198: ldrstdrealloc: typeof(_6 = lgldimacs::ldrstdrealloc as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64, u64) -> *mut libc::c_void (Pointer(ReifyFnPointer))) = fn(*mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l30} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {l31} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 198: ldrstdrealloc:   l29 = UNIQUE | NON_NULL, (empty)
            // 198: ldrstdrealloc:   l30 = UNIQUE | NON_NULL, (empty)
            // 198: ldrstdrealloc:   l31 = UNIQUE | NON_NULL, (empty)
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        ),
        Some(
        // 206: Some( ldrstdeal ... (), ): typeof(_7) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l18} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l19} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
        // 206: Some( ldrstdeal ... (), ):   l18 = UNIQUE | NON_NULL, (empty)
        // 206: Some( ldrstdeal ... (), ):   l19 = UNIQUE | NON_NULL, (empty)
        // 206: Some( ldrstdeal ... (), ): typeof(_7 = std::option::Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64)>::Some(move _8)) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l37} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
        // 206: Some( ldrstdeal ... (), ):   l37 = UNIQUE | NON_NULL, (empty)
        // 206: Some( ldrstdeal ... (), ):   l38 = UNIQUE | NON_NULL, (empty)
            ldrstdealloc_shim
            // 207: ldrstdealloc as ... -> (): typeof(_8) = fn(*mut {l21} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l22} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
            // 207: ldrstdealloc as ... -> ():   l21 = UNIQUE | NON_NULL, (empty)
            // 207: ldrstdealloc as ... -> ():   l22 = UNIQUE | NON_NULL, (empty)
            // 207: ldrstdealloc: typeof(_8 = lgldimacs::ldrstdealloc as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, u64) (Pointer(ReifyFnPointer))) = fn(*mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l36} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
            // 207: ldrstdealloc:   l35 = UNIQUE | NON_NULL, (empty)
            // 207: ldrstdealloc:   l36 = UNIQUE | NON_NULL, (empty)
                as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, size_t) -> (),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ldrminit(
    mut state: *mut libc::c_void,
    // 214: mut state: typeof(_1) = *mut {g11} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 214: mut state:   g11 = UNIQUE | NON_NULL, FIXED
    mut alloc: ldralloc,
    // 215: mut alloc: typeof(_2) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g12} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {g13} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 215: mut alloc:   g12 = UNIQUE | NON_NULL, FIXED
    // 215: mut alloc:   g13 = UNIQUE | NON_NULL, FIXED
    mut realloc_0: ldrealloc,
    // 216: mut realloc_0: typeof(_3) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g14} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {g15} DefId(2:5583 ~ core[480a]::ffi::c_void), u64, u64) -> *mut {g16} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 216: mut realloc_0:   g14 = UNIQUE | NON_NULL, FIXED
    // 216: mut realloc_0:   g15 = UNIQUE | NON_NULL, FIXED
    // 216: mut realloc_0:   g16 = UNIQUE | NON_NULL, FIXED
    mut dealloc: ldrdealloc,
    // 217: mut dealloc: typeof(_4) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g17} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {g18} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
    // 217: mut dealloc:   g17 = UNIQUE | NON_NULL, FIXED
    // 217: mut dealloc:   g18 = UNIQUE | NON_NULL, FIXED
) -> *mut LDR {
// 218: *mut LDR: typeof(_0) = *mut {g19} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 218: *mut LDR:   g19 = UNIQUE | NON_NULL, FIXED
    let mut res: *mut LDR = alloc.expect("non-null function pointer")(
    // 219: mut res: typeof(_6) = *mut {l6} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 219: mut res:   l6 = UNIQUE | NON_NULL, (empty)
    // 219: alloc.expect("n ... ng, ): typeof(_7) = *mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 219: alloc.expect("n ... ng, ):   l8 = UNIQUE | NON_NULL, (empty)
    // 219: alloc.expect("n ... ter"): typeof(_8) = fn(*mut {l10} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 219: alloc.expect("n ... ter"):   l10 = UNIQUE | NON_NULL, (empty)
    // 219: alloc.expect("n ... ter"):   l11 = UNIQUE | NON_NULL, (empty)
    // 219: alloc: typeof(_9) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l13} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 219: alloc:   l13 = UNIQUE | NON_NULL, (empty)
    // 219: alloc:   l14 = UNIQUE | NON_NULL, (empty)
    // 219: "non-null funct ... nter": typeof(_10) = & {l16} str
    // 219: "non-null funct ... nter":   l16 = UNIQUE | NON_NULL, (empty)
    // 219: "non-null funct ... nter": typeof(_11) = & {l18} str
    // 219: "non-null funct ... nter":   l18 = UNIQUE | NON_NULL, FIXED
    // 219: "non-null funct ... nter": typeof(_11 = const "non-null function pointer") = & {l46} str
    // 219: "non-null funct ... nter":   l46 = UNIQUE | NON_NULL, (empty)
    // 219: "non-null funct ... nter": typeof(_10 = &(*_11)) = & {l47} str
    // 219: "non-null funct ... nter":   l47 = UNIQUE | NON_NULL, (empty)
    // 219: alloc.expect("n ... t LDR: typeof(_6 = move _7 as *mut lgldimacs::LDR (Misc)) = *mut {l48} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 219: alloc.expect("n ... t LDR:   l48 = UNIQUE | NON_NULL, (empty)
        state,
        // 220: state: typeof(_12) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 220: state:   l20 = UNIQUE | NON_NULL, (empty)
        ::core::mem::size_of::<LDR>() as libc::c_ulong,
    ) as *mut LDR;
    if res.is_null() {
    // 223: res: typeof(_17) = *mut {l26} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 223: res:   l26 = UNIQUE | NON_NULL, (empty)
        return res;
    }
    memset(
    // 226: memset( res as  ... ng, ): typeof(_19) = *mut {l29} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 226: memset( res as  ... ng, ):   l29 = UNIQUE | NON_NULL, (empty)
        res as *mut libc::c_void,
        // 227: res as *mut lib ... _void: typeof(_20) = *mut {l31} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 227: res as *mut lib ... _void:   l31 = UNIQUE | NON_NULL, (empty)
        // 227: res: typeof(_21) = *mut {l33} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 227: res:   l33 = UNIQUE | NON_NULL, (empty)
        // 227: res as *mut lib ... _void: typeof(_20 = move _21 as *mut libc::c_void (Misc)) = *mut {l49} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 227: res as *mut lib ... _void:   l49 = UNIQUE | NON_NULL, (empty)
        0 as libc::c_int,
        ::core::mem::size_of::<LDR>() as libc::c_ulong,
    );
    (*res).mem.state = state;
    // 231: state: typeof(_25) = *mut {l38} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 231: state:   l38 = UNIQUE | NON_NULL, (empty)
    (*res).mem.alloc = alloc;
    // 232: alloc: typeof(_26) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l40} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l41} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 232: alloc:   l40 = UNIQUE | NON_NULL, (empty)
    // 232: alloc:   l41 = UNIQUE | NON_NULL, (empty)
    (*res).mem.dealloc = dealloc;
    // 233: dealloc: typeof(_27) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l43} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l44} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
    // 233: dealloc:   l43 = UNIQUE | NON_NULL, (empty)
    // 233: dealloc:   l44 = UNIQUE | NON_NULL, (empty)
    return res;
}
unsafe extern "C" fn ldrdelstr(mut ldr: *mut LDR, mut str: *mut libc::c_char) {
// 236: mut ldr: typeof(_1) = *mut {g21} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 236: mut ldr:   g21 = UNIQUE | NON_NULL, FIXED
// 236: mut str: typeof(_2) = *mut {g22} i8
// 236: mut str:   g22 = UNIQUE | NON_NULL, FIXED
    if !str.is_null() {
    // 237: str: typeof(_5) = *mut {l5} i8
    // 237: str:   l5 = UNIQUE | NON_NULL, (empty)
        ((*ldr).mem.dealloc).expect("non-null function pointer")(
        // 238: ((*ldr).mem.dea ... ter"): typeof(_7) = fn(*mut {l8} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l9} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
        // 238: ((*ldr).mem.dea ... ter"):   l8 = UNIQUE | NON_NULL, (empty)
        // 238: ((*ldr).mem.dea ... ter"):   l9 = UNIQUE | NON_NULL, (empty)
        // 238: ((*ldr).mem.dealloc): typeof(_8) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l11} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
        // 238: ((*ldr).mem.dealloc):   l11 = UNIQUE | NON_NULL, (empty)
        // 238: ((*ldr).mem.dealloc):   l12 = UNIQUE | NON_NULL, (empty)
        // 238: "non-null funct ... nter": typeof(_9) = & {l14} str
        // 238: "non-null funct ... nter":   l14 = UNIQUE | NON_NULL, (empty)
        // 238: "non-null funct ... nter": typeof(_10) = & {l16} str
        // 238: "non-null funct ... nter":   l16 = UNIQUE | NON_NULL, FIXED
        // 238: "non-null funct ... nter": typeof(_9 = &(*_10)) = & {l33} str
        // 238: "non-null funct ... nter":   l33 = UNIQUE | NON_NULL, (empty)
        // 238: "non-null funct ... nter": typeof(_10 = const "non-null function pointer") = & {l32} str
        // 238: "non-null funct ... nter":   l32 = UNIQUE | NON_NULL, (empty)
            (*ldr).mem.state,
            // 239: (*ldr).mem.state: typeof(_11) = *mut {l18} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 239: (*ldr).mem.state:   l18 = UNIQUE | NON_NULL, (empty)
            str as *mut libc::c_void,
            // 240: str as *mut lib ... _void: typeof(_12) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 240: str as *mut lib ... _void:   l20 = UNIQUE | NON_NULL, (empty)
            // 240: str: typeof(_13) = *mut {l22} i8
            // 240: str:   l22 = UNIQUE | NON_NULL, (empty)
            // 240: str as *mut lib ... _void: typeof(_12 = move _13 as *mut libc::c_void (Misc)) = *mut {l34} DefId(2:5583 ~ core[480a]::ffi::c_void)
            // 240: str as *mut lib ... _void:   l34 = UNIQUE | NON_NULL, (empty)
            (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong),
            // 241: str: typeof(_16) = *const {l26} i8
            // 241: str:   l26 = UNIQUE | NON_NULL, (empty)
            // 241: str: typeof(_17) = *mut {l28} i8
            // 241: str:   l28 = UNIQUE | NON_NULL, (empty)
            // 241: str: typeof(_16 = move _17 as *const i8 (Pointer(MutToConstPointer))) = *const {l35} i8
            // 241: str:   l35 = UNIQUE | NON_NULL, (empty)
        );
    }
}
unsafe extern "C" fn ldrstrdup(
    mut ldr: *mut LDR,
    // 246: mut ldr: typeof(_1) = *mut {g23} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 246: mut ldr:   g23 = UNIQUE | NON_NULL, FIXED
    mut str: *const libc::c_char,
    // 247: mut str: typeof(_2) = *const {g24} i8
    // 247: mut str:   g24 = UNIQUE | NON_NULL, FIXED
) -> *mut libc::c_char {
// 248: *mut libc::c_char: typeof(_0) = *mut {g25} i8
// 248: *mut libc::c_char:   g25 = UNIQUE | NON_NULL, FIXED
    let mut bytes: size_t = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    // 249: str: typeof(_6) = *const {l6} i8
    // 249: str:   l6 = UNIQUE | NON_NULL, (empty)
    let mut res: *mut libc::c_char =
    // 250: mut res: typeof(_9) = *mut {l10} i8
    // 250: mut res:   l10 = UNIQUE | NON_NULL, (empty)
        ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, bytes)
        // 251: ((*ldr).mem.all ... ytes): typeof(_10) = *mut {l12} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 251: ((*ldr).mem.all ... ytes):   l12 = UNIQUE | NON_NULL, (empty)
        // 251: ((*ldr).mem.all ... ter"): typeof(_11) = fn(*mut {l14} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l15} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 251: ((*ldr).mem.all ... ter"):   l14 = UNIQUE | NON_NULL, (empty)
        // 251: ((*ldr).mem.all ... ter"):   l15 = UNIQUE | NON_NULL, (empty)
        // 251: ((*ldr).mem.alloc): typeof(_12) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l17} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l18} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 251: ((*ldr).mem.alloc):   l17 = UNIQUE | NON_NULL, (empty)
        // 251: ((*ldr).mem.alloc):   l18 = UNIQUE | NON_NULL, (empty)
        // 251: "non-null funct ... nter": typeof(_13) = & {l20} str
        // 251: "non-null funct ... nter":   l20 = UNIQUE | NON_NULL, (empty)
        // 251: "non-null funct ... nter": typeof(_14) = & {l22} str
        // 251: "non-null funct ... nter":   l22 = UNIQUE | NON_NULL, FIXED
        // 251: (*ldr).mem.state: typeof(_15) = *mut {l24} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 251: (*ldr).mem.state:   l24 = UNIQUE | NON_NULL, (empty)
        // 251: "non-null funct ... nter": typeof(_14 = const "non-null function pointer") = & {l31} str
        // 251: "non-null funct ... nter":   l31 = UNIQUE | NON_NULL, (empty)
        // 251: ((*ldr).mem.all ... _char: typeof(_9 = move _10 as *mut i8 (Misc)) = *mut {l33} i8
        // 251: ((*ldr).mem.all ... _char:   l33 = UNIQUE | NON_NULL, (empty)
        // 251: "non-null funct ... nter": typeof(_13 = &(*_14)) = & {l32} str
        // 251: "non-null funct ... nter":   l32 = UNIQUE | NON_NULL, (empty)
            as *mut libc::c_char;
    return strcpy(res, str);
    // 253: res: typeof(_17) = *mut {l27} i8
    // 253: res:   l27 = UNIQUE | NON_NULL, (empty)
    // 253: str: typeof(_18) = *const {l29} i8
    // 253: str:   l29 = UNIQUE | NON_NULL, (empty)
}
#[no_mangle]
pub unsafe extern "C" fn ldrelease(mut ldr: *mut LDR) {
// 256: mut ldr: typeof(_1) = *mut {g26} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 256: mut ldr:   g26 = UNIQUE | NON_NULL, FIXED
    if !((*ldr).file).is_null() {
    // 257: ((*ldr).file): typeof(_5) = *mut {l5} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 257: ((*ldr).file):   l5 = UNIQUE | NON_NULL, (empty)
        if (*ldr).closefile == 1 as libc::c_int {
            fclose((*ldr).file);
            // 259: (*ldr).file: typeof(_11) = *mut {l12} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
            // 259: (*ldr).file:   l12 = UNIQUE | NON_NULL, (empty)
        }
        if (*ldr).closefile == 2 as libc::c_int {
            pclose((*ldr).file);
            // 262: (*ldr).file: typeof(_16) = *mut {l18} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
            // 262: (*ldr).file:   l18 = UNIQUE | NON_NULL, (empty)
        }
    }
    ldrdelstr(ldr, (*ldr).errmsg);
    // 265: ldr: typeof(_18) = *mut {l21} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 265: ldr:   l21 = UNIQUE | NON_NULL, (empty)
    // 265: (*ldr).errmsg: typeof(_19) = *mut {l23} i8
    // 265: (*ldr).errmsg:   l23 = UNIQUE | NON_NULL, (empty)
    ldrdelstr(ldr, (*ldr).path);
    // 266: ldr: typeof(_21) = *mut {l26} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 266: ldr:   l26 = UNIQUE | NON_NULL, (empty)
    // 266: (*ldr).path: typeof(_22) = *mut {l28} i8
    // 266: (*ldr).path:   l28 = UNIQUE | NON_NULL, (empty)
    ((*ldr).mem.dealloc).expect("non-null function pointer")(
    // 267: ((*ldr).mem.dea ... ter"): typeof(_24) = fn(*mut {l31} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
    // 267: ((*ldr).mem.dea ... ter"):   l31 = UNIQUE | NON_NULL, (empty)
    // 267: ((*ldr).mem.dea ... ter"):   l32 = UNIQUE | NON_NULL, (empty)
    // 267: ((*ldr).mem.dealloc): typeof(_25) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l34} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l35} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
    // 267: ((*ldr).mem.dealloc):   l34 = UNIQUE | NON_NULL, (empty)
    // 267: ((*ldr).mem.dealloc):   l35 = UNIQUE | NON_NULL, (empty)
    // 267: "non-null funct ... nter": typeof(_26) = & {l37} str
    // 267: "non-null funct ... nter":   l37 = UNIQUE | NON_NULL, (empty)
    // 267: "non-null funct ... nter": typeof(_27) = & {l39} str
    // 267: "non-null funct ... nter":   l39 = UNIQUE | NON_NULL, FIXED
    // 267: "non-null funct ... nter": typeof(_26 = &(*_27)) = & {l50} str
    // 267: "non-null funct ... nter":   l50 = UNIQUE | NON_NULL, (empty)
    // 267: "non-null funct ... nter": typeof(_27 = const "non-null function pointer") = & {l49} str
    // 267: "non-null funct ... nter":   l49 = UNIQUE | NON_NULL, (empty)
        (*ldr).mem.state,
        // 268: (*ldr).mem.state: typeof(_28) = *mut {l41} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 268: (*ldr).mem.state:   l41 = UNIQUE | NON_NULL, (empty)
        ldr as *mut libc::c_void,
        // 269: ldr as *mut lib ... _void: typeof(_29) = *mut {l43} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 269: ldr as *mut lib ... _void:   l43 = UNIQUE | NON_NULL, (empty)
        // 269: ldr: typeof(_30) = *mut {l45} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 269: ldr:   l45 = UNIQUE | NON_NULL, (empty)
        // 269: ldr as *mut lib ... _void: typeof(_29 = move _30 as *mut libc::c_void (Misc)) = *mut {l51} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 269: ldr as *mut lib ... _void:   l51 = UNIQUE | NON_NULL, (empty)
        ::core::mem::size_of::<LDR>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetopt<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17,'h18>(
    mut ldr: &'h0 mut (lgldimacs::LDR<'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15>),
    // 275: mut ldr: typeof(_1) = *mut {g27} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 275: mut ldr:   g27 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    mut state: &'h16 (libc::c_void),
    // 276: mut state: typeof(_2) = *mut {g28} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 276: mut state:   g28 = UNIQUE | NON_NULL, (empty)
    mut fun: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, *const i8, i32)>,
    // 277: mut fun: typeof(_3) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g29} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {g30} i8, i32) -> ()>
    // 277: mut fun:   g29 = UNIQUE | NON_NULL, (empty)
    // 277: mut fun:   g30 = UNIQUE | NON_NULL, (empty)
) {
    (*ldr).opt.fun = fun;
    // 279: fun: typeof(_4) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g29} DefId(2:5583 ~ core[480a]::ffi::c_void), *const {g30} i8, i32) -> ()>
    // 279: fun:   g29 = UNIQUE | NON_NULL, (empty)
    // 279: fun:   g30 = UNIQUE | NON_NULL, (empty)
    (*ldr).opt.state = state;
    // 280: state: typeof(_5) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 280: state:   l5 = UNIQUE | NON_NULL, (empty)
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetheader<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17>(
    mut ldr: &'h0 mut (lgldimacs::LDR<'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15>),
    // 284: mut ldr: typeof(_1) = *mut {g31} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 284: mut ldr:   g31 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    mut state: &'h16 (libc::c_void),
    // 285: mut state: typeof(_2) = *mut {g32} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 285: mut state:   g32 = UNIQUE | NON_NULL, (empty)
    mut fun: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, i32, i32)>,
    // 286: mut fun: typeof(_3) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g33} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
    // 286: mut fun:   g33 = UNIQUE | NON_NULL, FIXED
) {
    (*ldr).header.fun = fun;
    // 288: fun: typeof(_4) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g33} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
    // 288: fun:   g33 = UNIQUE | NON_NULL, FIXED
    (*ldr).header.state = core::ptr::addr_of!(*(state)).cast_mut();
    // 289: state: typeof(_5) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 289: state:   l5 = UNIQUE | NON_NULL, (empty)
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetadd<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17>(
    mut ldr: &'h0 mut (lgldimacs::LDR<'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15>),
    // 293: mut ldr: typeof(_1) = *mut {g34} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 293: mut ldr:   g34 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    mut state: &'h16 (libc::c_void),
    // 294: mut state: typeof(_2) = *mut {g35} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 294: mut state:   g35 = UNIQUE | NON_NULL, (empty)
    mut fun: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, i32)>,
    // 295: mut fun: typeof(_3) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g36} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
    // 295: mut fun:   g36 = UNIQUE | NON_NULL, FIXED
) {
    (*ldr).add.fun = fun;
    // 297: fun: typeof(_4) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {g36} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
    // 297: fun:   g36 = UNIQUE | NON_NULL, FIXED
    (*ldr).add.state = core::ptr::addr_of!(*(state)).cast_mut();
    // 298: state: typeof(_5) = *mut {l5} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 298: state:   l5 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn ldrfilexists(mut path: *const libc::c_char) -> libc::c_int {
// 300: mut path: typeof(_1) = *const {g37} i8
// 300: mut path:   g37 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    return (stat(path, &mut buf) == 0) as libc::c_int;
    // 327: path: typeof(_10) = *const {l10} i8
    // 327: path:   l10 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 327: &mut buf: typeof(_11) = *mut {l12} DefId(0:1806 ~ libgeling[8679]::lgldimacs::stat)
    // 327: &mut buf:   l12 = WRITE | UNIQUE | NON_NULL, (empty)
    // 327: &mut buf: typeof(_12) = &mut {l14} DefId(0:1806 ~ libgeling[8679]::lgldimacs::stat)
    // 327: &mut buf:   l14 = WRITE | UNIQUE | NON_NULL, (empty)
    // 327: &mut buf: typeof(_11 = &raw mut (*_12)) = *mut {l17} DefId(0:1806 ~ libgeling[8679]::lgldimacs::stat)
    // 327: &mut buf:   l17 = WRITE | UNIQUE | NON_NULL, (empty)
    // 327: &mut buf: typeof(_12 = &mut _3) = &mut {l16} DefId(0:1806 ~ libgeling[8679]::lgldimacs::stat)
    // 327: &mut buf:   l16 = WRITE | UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn ldrperr(mut ldr: *mut LDR, mut msg: *const libc::c_char) -> libc::c_int {
// 329: mut ldr: typeof(_1) = *mut {g38} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 329: mut ldr:   g38 = UNIQUE | NON_NULL, FIXED
// 329: mut msg: typeof(_2) = *const {g39} i8
// 329: mut msg:   g39 = UNIQUE | NON_NULL, FIXED
    let mut bytes: size_t = 0;
    let mut len: size_t = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    // 332: mut str: typeof(_6) = *mut {l6} i8
    // 332: mut str:   l6 = UNIQUE | NON_NULL, (empty)
    // 332: 0 as *mut libc: ... _char: typeof(_6 = const 0_usize as *mut i8 (PointerFromExposedAddress)) = *mut {l100} i8
    // 332: 0 as *mut libc: ... _char:   l100 = UNIQUE | NON_NULL, (empty)
    bytes = (strlen(msg))
    // 333: msg: typeof(_10) = *const {l11} i8
    // 333: msg:   l11 = UNIQUE | NON_NULL, (empty)
        .wrapping_add(strlen((*ldr).path))
        // 334: (*ldr).path: typeof(_12) = *const {l14} i8
        // 334: (*ldr).path:   l14 = UNIQUE | NON_NULL, (empty)
        // 334: (*ldr).path: typeof(_13) = *mut {l16} i8
        // 334: (*ldr).path:   l16 = UNIQUE | NON_NULL, (empty)
        // 334: (*ldr).path: typeof(_12 = move _13 as *const i8 (Pointer(MutToConstPointer))) = *const {l101} i8
        // 334: (*ldr).path:   l101 = UNIQUE | NON_NULL, (empty)
        .wrapping_add(20 as libc::c_int as libc::c_ulong);
    str = ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, bytes)
    // 336: ((*ldr).mem.all ... ytes): typeof(_16) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 336: ((*ldr).mem.all ... ytes):   l20 = UNIQUE | NON_NULL, (empty)
    // 336: ((*ldr).mem.all ... ter"): typeof(_17) = fn(*mut {l22} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l23} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 336: ((*ldr).mem.all ... ter"):   l22 = UNIQUE | NON_NULL, (empty)
    // 336: ((*ldr).mem.all ... ter"):   l23 = UNIQUE | NON_NULL, (empty)
    // 336: ((*ldr).mem.alloc): typeof(_18) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l25} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l26} DefId(2:5583 ~ core[480a]::ffi::c_void)>
    // 336: ((*ldr).mem.alloc):   l25 = UNIQUE | NON_NULL, (empty)
    // 336: ((*ldr).mem.alloc):   l26 = UNIQUE | NON_NULL, (empty)
    // 336: "non-null funct ... nter": typeof(_19) = & {l28} str
    // 336: "non-null funct ... nter":   l28 = UNIQUE | NON_NULL, (empty)
    // 336: "non-null funct ... nter": typeof(_20) = & {l30} str
    // 336: "non-null funct ... nter":   l30 = UNIQUE | NON_NULL, FIXED
    // 336: (*ldr).mem.state: typeof(_21) = *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 336: (*ldr).mem.state:   l32 = UNIQUE | NON_NULL, (empty)
    // 336: "non-null funct ... nter": typeof(_19 = &(*_20)) = & {l103} str
    // 336: "non-null funct ... nter":   l103 = UNIQUE | NON_NULL, (empty)
    // 336: "non-null funct ... nter": typeof(_20 = const "non-null function pointer") = & {l102} str
    // 336: "non-null funct ... nter":   l102 = UNIQUE | NON_NULL, (empty)
    // 336: str = ((*ldr).m ... _char: typeof(_6 = move _16 as *mut i8 (Misc)) = *mut {l104} i8
    // 336: str = ((*ldr).m ... _char:   l104 = UNIQUE | NON_NULL, (empty)
        as *mut libc::c_char;
    sprintf(
        str,
        // 339: str: typeof(_24) = *mut {l36} i8
        // 339: str:   l36 = UNIQUE | NON_NULL, (empty)
        b"%s:%d: %s\0" as *const u8 as *const libc::c_char,
        // 340: b"%s:%d: %s\0"  ... _char: typeof(_25) = *const {l38} i8
        // 340: b"%s:%d: %s\0"  ... _char:   l38 = UNIQUE | NON_NULL, (empty)
        // 340: b"%s:%d: %s\0"  ... st u8: typeof(_26) = *const {l40} u8
        // 340: b"%s:%d: %s\0"  ... st u8:   l40 = UNIQUE | NON_NULL, (empty)
        // 340: b"%s:%d: %s\0": typeof(_27) = *const {l42} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 340: b"%s:%d: %s\0":   l42 = UNIQUE | NON_NULL, (empty)
        // 340: b"%s:%d: %s\0": typeof(_28) = & {l44} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 340: b"%s:%d: %s\0":   l44 = UNIQUE | NON_NULL, FIXED
        // 340: b"%s:%d: %s\0"  ... st u8: typeof(_26 = move _27 as *const u8 (Pointer(ArrayToPointer))) = *const {l107} u8
        // 340: b"%s:%d: %s\0"  ... st u8:   l107 = UNIQUE | NON_NULL, (empty)
        // 340: b"%s:%d: %s\0": typeof(_28 = const b"%s:%d: %s\x00") = & {l105} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 340: b"%s:%d: %s\0":   l105 = UNIQUE | NON_NULL, (empty)
        // 340: b"%s:%d: %s\0"  ... _char: typeof(_25 = move _26 as *const i8 (Misc)) = *const {l108} i8
        // 340: b"%s:%d: %s\0"  ... _char:   l108 = UNIQUE | NON_NULL, (empty)
        // 340: b"%s:%d: %s\0": typeof(_27 = &raw const (*_28)) = *const {l106} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000a)) }]
        // 340: b"%s:%d: %s\0":   l106 = UNIQUE | NON_NULL, (empty)
        (*ldr).path,
        // 341: (*ldr).path: typeof(_29) = *mut {l46} i8
        // 341: (*ldr).path:   l46 = UNIQUE | NON_NULL, (empty)
        (*ldr).lineno,
        msg,
        // 343: msg: typeof(_31) = *const {l49} i8
        // 343: msg:   l49 = UNIQUE | NON_NULL, (empty)
    );
    len = (strlen(str)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    // 345: str: typeof(_34) = *const {l53} i8
    // 345: str:   l53 = UNIQUE | NON_NULL, (empty)
    // 345: str: typeof(_35) = *mut {l55} i8
    // 345: str:   l55 = UNIQUE | NON_NULL, (empty)
    // 345: str: typeof(_34 = move _35 as *const i8 (Pointer(MutToConstPointer))) = *const {l109} i8
    // 345: str:   l109 = UNIQUE | NON_NULL, (empty)
    (*ldr).errmsg = strcpy(
    // 346: strcpy( ((*ldr) ... tr, ): typeof(_38) = *mut {l59} i8
    // 346: strcpy( ((*ldr) ... tr, ):   l59 = UNIQUE | NON_NULL, (empty)
        ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, len)
        // 347: ((*ldr).mem.all ... _char: typeof(_39) = *mut {l61} i8
        // 347: ((*ldr).mem.all ... _char:   l61 = UNIQUE | NON_NULL, (empty)
        // 347: ((*ldr).mem.all ...  len): typeof(_40) = *mut {l63} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 347: ((*ldr).mem.all ...  len):   l63 = UNIQUE | NON_NULL, (empty)
        // 347: ((*ldr).mem.all ... ter"): typeof(_41) = fn(*mut {l65} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l66} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 347: ((*ldr).mem.all ... ter"):   l65 = UNIQUE | NON_NULL, (empty)
        // 347: ((*ldr).mem.all ... ter"):   l66 = UNIQUE | NON_NULL, (empty)
        // 347: ((*ldr).mem.alloc): typeof(_42) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l68} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l69} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 347: ((*ldr).mem.alloc):   l68 = UNIQUE | NON_NULL, (empty)
        // 347: ((*ldr).mem.alloc):   l69 = UNIQUE | NON_NULL, (empty)
        // 347: "non-null funct ... nter": typeof(_43) = & {l71} str
        // 347: "non-null funct ... nter":   l71 = UNIQUE | NON_NULL, (empty)
        // 347: "non-null funct ... nter": typeof(_44) = & {l73} str
        // 347: "non-null funct ... nter":   l73 = UNIQUE | NON_NULL, FIXED
        // 347: (*ldr).mem.state: typeof(_45) = *mut {l75} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 347: (*ldr).mem.state:   l75 = UNIQUE | NON_NULL, (empty)
        // 347: "non-null funct ... nter": typeof(_43 = &(*_44)) = & {l111} str
        // 347: "non-null funct ... nter":   l111 = UNIQUE | NON_NULL, (empty)
        // 347: ((*ldr).mem.all ... _char: typeof(_39 = move _40 as *mut i8 (Misc)) = *mut {l112} i8
        // 347: ((*ldr).mem.all ... _char:   l112 = UNIQUE | NON_NULL, (empty)
        // 347: "non-null funct ... nter": typeof(_44 = const "non-null function pointer") = & {l110} str
        // 347: "non-null funct ... nter":   l110 = UNIQUE | NON_NULL, (empty)
            as *mut libc::c_char,
        str,
        // 349: str: typeof(_47) = *const {l78} i8
        // 349: str:   l78 = UNIQUE | NON_NULL, (empty)
        // 349: str: typeof(_48) = *mut {l80} i8
        // 349: str:   l80 = UNIQUE | NON_NULL, (empty)
        // 349: str: typeof(_47 = move _48 as *const i8 (Pointer(MutToConstPointer))) = *const {l113} i8
        // 349: str:   l113 = UNIQUE | NON_NULL, (empty)
    );
    ((*ldr).mem.dealloc).expect("non-null function pointer")(
    // 351: ((*ldr).mem.dea ... ter"): typeof(_50) = fn(*mut {l83} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l84} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
    // 351: ((*ldr).mem.dea ... ter"):   l83 = UNIQUE | NON_NULL, (empty)
    // 351: ((*ldr).mem.dea ... ter"):   l84 = UNIQUE | NON_NULL, (empty)
    // 351: ((*ldr).mem.dealloc): typeof(_51) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l86} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l87} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
    // 351: ((*ldr).mem.dealloc):   l86 = UNIQUE | NON_NULL, (empty)
    // 351: ((*ldr).mem.dealloc):   l87 = UNIQUE | NON_NULL, (empty)
    // 351: "non-null funct ... nter": typeof(_52) = & {l89} str
    // 351: "non-null funct ... nter":   l89 = UNIQUE | NON_NULL, (empty)
    // 351: "non-null funct ... nter": typeof(_53) = & {l91} str
    // 351: "non-null funct ... nter":   l91 = UNIQUE | NON_NULL, FIXED
    // 351: "non-null funct ... nter": typeof(_53 = const "non-null function pointer") = & {l114} str
    // 351: "non-null funct ... nter":   l114 = UNIQUE | NON_NULL, (empty)
    // 351: "non-null funct ... nter": typeof(_52 = &(*_53)) = & {l115} str
    // 351: "non-null funct ... nter":   l115 = UNIQUE | NON_NULL, (empty)
        (*ldr).mem.state,
        // 352: (*ldr).mem.state: typeof(_54) = *mut {l93} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 352: (*ldr).mem.state:   l93 = UNIQUE | NON_NULL, (empty)
        str as *mut libc::c_void,
        // 353: str as *mut lib ... _void: typeof(_55) = *mut {l95} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 353: str as *mut lib ... _void:   l95 = UNIQUE | NON_NULL, (empty)
        // 353: str: typeof(_56) = *mut {l97} i8
        // 353: str:   l97 = UNIQUE | NON_NULL, (empty)
        // 353: str as *mut lib ... _void: typeof(_55 = move _56 as *mut libc::c_void (Misc)) = *mut {l116} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 353: str as *mut lib ... _void:   l116 = UNIQUE | NON_NULL, (empty)
        bytes,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn ldrhas(
    mut str: *const libc::c_char,
    // 359: mut str: typeof(_1) = *const {g40} i8
    // 359: mut str:   g40 = READ | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, FIXED
    mut suffix: *const libc::c_char,
    // 360: mut suffix: typeof(_2) = *const {g41} i8
    // 360: mut suffix:   g41 = READ | UNIQUE | OFFSET_ADD | NON_NULL, FIXED
) -> libc::c_int {
    let mut l: libc::c_int = strlen(str) as libc::c_int;
    // 362: str: typeof(_6) = *const {l6} i8
    // 362: str:   l6 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    let mut k: libc::c_int = strlen(suffix) as libc::c_int;
    // 363: suffix: typeof(_9) = *const {l10} i8
    // 363: suffix:   l10 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    if l < k {
        return 0 as libc::c_int;
    }
    return (strcmp(str.offset(l as isize).offset(-(k as isize)), suffix) == 0) as libc::c_int;
    // 367: str.offset(l as ... ize)): typeof(_17) = *const {l19} i8
    // 367: str.offset(l as ... ize)):   l19 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
    // 367: str.offset(l as ... size): typeof(_18) = *const {l21} i8
    // 367: str.offset(l as ... size):   l21 = READ | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
    // 367: str: typeof(_19) = *const {l23} i8
    // 367: str:   l23 = READ | UNIQUE | OFFSET_ADD | OFFSET_SUB | NON_NULL, (empty)
    // 367: suffix: typeof(_26) = *const {l31} i8
    // 367: suffix:   l31 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
}
unsafe extern "C" fn ldrcmd(
    mut ldr: *mut LDR,
    // 370: mut ldr: typeof(_1) = *mut {g42} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 370: mut ldr:   g42 = UNIQUE | NON_NULL, FIXED
    mut fmt: *const libc::c_char,
    // 371: mut fmt: typeof(_2) = *const {g43} i8
    // 371: mut fmt:   g43 = UNIQUE | NON_NULL, FIXED
    mut name: *const libc::c_char,
    // 372: mut name: typeof(_3) = *const {g44} i8
    // 372: mut name:   g44 = UNIQUE | NON_NULL, FIXED
) -> *mut FILE {
// 373: *mut FILE: typeof(_0) = *mut {g45} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
// 373: *mut FILE:   g45 = UNIQUE | NON_NULL, FIXED
    let mut res: *mut FILE = 0 as *mut FILE;
    // 374: mut res: typeof(_5) = *mut {l5} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 374: mut res:   l5 = UNIQUE | NON_NULL, (empty)
    // 374: 0 as *mut FILE: typeof(_5 = const 0_usize as *mut lgldimacs::_IO_FILE (PointerFromExposedAddress)) = *mut {l76} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 374: 0 as *mut FILE:   l76 = UNIQUE | NON_NULL, (empty)
    let mut len: libc::c_int = (strlen(fmt))
    // 375: fmt: typeof(_10) = *const {l11} i8
    // 375: fmt:   l11 = UNIQUE | NON_NULL, (empty)
        .wrapping_add(strlen(name))
        // 376: name: typeof(_12) = *const {l14} i8
        // 376: name:   l14 = UNIQUE | NON_NULL, (empty)
        .wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    let mut s: *mut libc::c_char =
    // 379: mut s: typeof(_15) = *mut {l18} i8
    // 379: mut s:   l18 = UNIQUE | NON_NULL, (empty)
        ((*ldr).mem.alloc).expect("non-null function pointer")((*ldr).mem.state, len as size_t)
        // 380: ((*ldr).mem.all ... ze_t): typeof(_16) = *mut {l20} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 380: ((*ldr).mem.all ... ze_t):   l20 = UNIQUE | NON_NULL, (empty)
        // 380: ((*ldr).mem.all ... ter"): typeof(_17) = fn(*mut {l22} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l23} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 380: ((*ldr).mem.all ... ter"):   l22 = UNIQUE | NON_NULL, (empty)
        // 380: ((*ldr).mem.all ... ter"):   l23 = UNIQUE | NON_NULL, (empty)
        // 380: ((*ldr).mem.alloc): typeof(_18) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l25} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> *mut {l26} DefId(2:5583 ~ core[480a]::ffi::c_void)>
        // 380: ((*ldr).mem.alloc):   l25 = UNIQUE | NON_NULL, (empty)
        // 380: ((*ldr).mem.alloc):   l26 = UNIQUE | NON_NULL, (empty)
        // 380: "non-null funct ... nter": typeof(_19) = & {l28} str
        // 380: "non-null funct ... nter":   l28 = UNIQUE | NON_NULL, (empty)
        // 380: "non-null funct ... nter": typeof(_20) = & {l30} str
        // 380: "non-null funct ... nter":   l30 = UNIQUE | NON_NULL, FIXED
        // 380: (*ldr).mem.state: typeof(_21) = *mut {l32} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 380: (*ldr).mem.state:   l32 = UNIQUE | NON_NULL, (empty)
        // 380: "non-null funct ... nter": typeof(_19 = &(*_20)) = & {l78} str
        // 380: "non-null funct ... nter":   l78 = UNIQUE | NON_NULL, (empty)
        // 380: "non-null funct ... nter": typeof(_20 = const "non-null function pointer") = & {l77} str
        // 380: "non-null funct ... nter":   l77 = UNIQUE | NON_NULL, (empty)
        // 380: ((*ldr).mem.all ... _char: typeof(_15 = move _16 as *mut i8 (Misc)) = *mut {l79} i8
        // 380: ((*ldr).mem.all ... _char:   l79 = UNIQUE | NON_NULL, (empty)
            as *mut libc::c_char;
    sprintf(s, fmt, name);
    // 382: s: typeof(_25) = *mut {l37} i8
    // 382: s:   l37 = UNIQUE | NON_NULL, (empty)
    // 382: fmt: typeof(_26) = *const {l39} i8
    // 382: fmt:   l39 = UNIQUE | NON_NULL, (empty)
    // 382: name: typeof(_27) = *const {l41} i8
    // 382: name:   l41 = UNIQUE | NON_NULL, (empty)
    res = popen(s, b"r\0" as *const u8 as *const libc::c_char);
    // 383: popen(s, b"r\0" ... char): typeof(_28) = *mut {l43} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 383: popen(s, b"r\0" ... char):   l43 = UNIQUE | NON_NULL, (empty)
    // 383: s: typeof(_29) = *const {l45} i8
    // 383: s:   l45 = UNIQUE | NON_NULL, (empty)
    // 383: s: typeof(_30) = *mut {l47} i8
    // 383: s:   l47 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0" as *cons ... _char: typeof(_31) = *const {l49} i8
    // 383: b"r\0" as *cons ... _char:   l49 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0" as *const u8: typeof(_32) = *const {l51} u8
    // 383: b"r\0" as *const u8:   l51 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0": typeof(_33) = *const {l53} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 383: b"r\0":   l53 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0": typeof(_34) = & {l55} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 383: b"r\0":   l55 = UNIQUE | NON_NULL, FIXED
    // 383: b"r\0": typeof(_33 = &raw const (*_34)) = *const {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 383: b"r\0":   l82 = UNIQUE | NON_NULL, (empty)
    // 383: s: typeof(_29 = move _30 as *const i8 (Pointer(MutToConstPointer))) = *const {l80} i8
    // 383: s:   l80 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0": typeof(_34 = const b"r\x00") = & {l81} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
    // 383: b"r\0":   l81 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0" as *cons ... _char: typeof(_31 = move _32 as *const i8 (Misc)) = *const {l84} i8
    // 383: b"r\0" as *cons ... _char:   l84 = UNIQUE | NON_NULL, (empty)
    // 383: b"r\0" as *const u8: typeof(_32 = move _33 as *const u8 (Pointer(ArrayToPointer))) = *const {l83} u8
    // 383: b"r\0" as *const u8:   l83 = UNIQUE | NON_NULL, (empty)
    ((*ldr).mem.dealloc).expect("non-null function pointer")(
    // 384: ((*ldr).mem.dea ... ter"): typeof(_36) = fn(*mut {l58} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l59} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()
    // 384: ((*ldr).mem.dea ... ter"):   l58 = UNIQUE | NON_NULL, (empty)
    // 384: ((*ldr).mem.dea ... ter"):   l59 = UNIQUE | NON_NULL, (empty)
    // 384: ((*ldr).mem.dealloc): typeof(_37) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l61} DefId(2:5583 ~ core[480a]::ffi::c_void), *mut {l62} DefId(2:5583 ~ core[480a]::ffi::c_void), u64) -> ()>
    // 384: ((*ldr).mem.dealloc):   l61 = UNIQUE | NON_NULL, (empty)
    // 384: ((*ldr).mem.dealloc):   l62 = UNIQUE | NON_NULL, (empty)
    // 384: "non-null funct ... nter": typeof(_38) = & {l64} str
    // 384: "non-null funct ... nter":   l64 = UNIQUE | NON_NULL, (empty)
    // 384: "non-null funct ... nter": typeof(_39) = & {l66} str
    // 384: "non-null funct ... nter":   l66 = UNIQUE | NON_NULL, FIXED
    // 384: "non-null funct ... nter": typeof(_38 = &(*_39)) = & {l86} str
    // 384: "non-null funct ... nter":   l86 = UNIQUE | NON_NULL, (empty)
    // 384: "non-null funct ... nter": typeof(_39 = const "non-null function pointer") = & {l85} str
    // 384: "non-null funct ... nter":   l85 = UNIQUE | NON_NULL, (empty)
        (*ldr).mem.state,
        // 385: (*ldr).mem.state: typeof(_40) = *mut {l68} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 385: (*ldr).mem.state:   l68 = UNIQUE | NON_NULL, (empty)
        s as *mut libc::c_void,
        // 386: s as *mut libc: ... _void: typeof(_41) = *mut {l70} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 386: s as *mut libc: ... _void:   l70 = UNIQUE | NON_NULL, (empty)
        // 386: s: typeof(_42) = *mut {l72} i8
        // 386: s:   l72 = UNIQUE | NON_NULL, (empty)
        // 386: s as *mut libc: ... _void: typeof(_41 = move _42 as *mut libc::c_void (Misc)) = *mut {l87} DefId(2:5583 ~ core[480a]::ffi::c_void)
        // 386: s as *mut libc: ... _void:   l87 = UNIQUE | NON_NULL, (empty)
        len as size_t,
    );
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetpath(mut ldr: *mut LDR, mut path: *const libc::c_char) -> i32 {
// 392: mut ldr: typeof(_1) = *mut {g46} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 392: mut ldr:   g46 = UNIQUE | NON_NULL, FIXED
// 392: mut path: typeof(_2) = *const {g47} i8
// 392: mut path:   g47 = UNIQUE | NON_NULL, FIXED
    (*ldr).path = ldrstrdup(ldr, path);
    // 393: ldrstrdup(ldr, path): typeof(_3) = *mut {l3} i8
    // 393: ldrstrdup(ldr, path):   l3 = UNIQUE | NON_NULL, (empty)
    // 393: ldr: typeof(_4) = *mut {l5} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 393: ldr:   l5 = UNIQUE | NON_NULL, (empty)
    // 393: path: typeof(_5) = *const {l7} i8
    // 393: path:   l7 = UNIQUE | NON_NULL, (empty)
    if ldrfilexists(path) == 0 {
    // 394: path: typeof(_9) = *const {l12} i8
    // 394: path:   l12 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 396: ldr: typeof(_11) = *mut {l15} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 396: ldr:   l15 = UNIQUE | NON_NULL, (empty)
            b"file does not exist\0" as *const u8 as *const libc::c_char,
            // 397: b"file does not ... _char: typeof(_12) = *const {l17} i8
            // 397: b"file does not ... _char:   l17 = UNIQUE | NON_NULL, (empty)
            // 397: b"file does not ... st u8: typeof(_13) = *const {l19} u8
            // 397: b"file does not ... st u8:   l19 = UNIQUE | NON_NULL, (empty)
            // 397: b"file does not ... st\0": typeof(_14) = *const {l21} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 397: b"file does not ... st\0":   l21 = UNIQUE | NON_NULL, (empty)
            // 397: b"file does not ... st\0": typeof(_15) = & {l23} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 397: b"file does not ... st\0":   l23 = UNIQUE | NON_NULL, FIXED
            // 397: b"file does not ... st\0": typeof(_15 = const b"file does not exist\x00") = & {l159} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 397: b"file does not ... st\0":   l159 = UNIQUE | NON_NULL, (empty)
            // 397: b"file does not ... _char: typeof(_12 = move _13 as *const i8 (Misc)) = *const {l162} i8
            // 397: b"file does not ... _char:   l162 = UNIQUE | NON_NULL, (empty)
            // 397: b"file does not ... st\0": typeof(_14 = &raw const (*_15)) = *const {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 397: b"file does not ... st\0":   l160 = UNIQUE | NON_NULL, (empty)
            // 397: b"file does not ... st u8: typeof(_13 = move _14 as *const u8 (Pointer(ArrayToPointer))) = *const {l161} u8
            // 397: b"file does not ... st u8:   l161 = UNIQUE | NON_NULL, (empty)
        );
    }
    (*ldr).closefile = 2 as libc::c_int;
    if ldrhas(path, b".gz\0" as *const u8 as *const libc::c_char) != 0 {
    // 401: path: typeof(_20) = *const {l29} i8
    // 401: path:   l29 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0" as *co ... _char: typeof(_21) = *const {l31} i8
    // 401: b".gz\0" as *co ... _char:   l31 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0" as *co ... st u8: typeof(_22) = *const {l33} u8
    // 401: b".gz\0" as *co ... st u8:   l33 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0": typeof(_23) = *const {l35} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 401: b".gz\0":   l35 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0": typeof(_24) = & {l37} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 401: b".gz\0":   l37 = UNIQUE | NON_NULL, FIXED
    // 401: b".gz\0": typeof(_24 = const b".gz\x00") = & {l163} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 401: b".gz\0":   l163 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0": typeof(_23 = &raw const (*_24)) = *const {l164} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 401: b".gz\0":   l164 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0" as *co ... _char: typeof(_21 = move _22 as *const i8 (Misc)) = *const {l166} i8
    // 401: b".gz\0" as *co ... _char:   l166 = UNIQUE | NON_NULL, (empty)
    // 401: b".gz\0" as *co ... st u8: typeof(_22 = move _23 as *const u8 (Pointer(ArrayToPointer))) = *const {l165} u8
    // 401: b".gz\0" as *co ... st u8:   l165 = UNIQUE | NON_NULL, (empty)
        (*ldr).file = ldrcmd(
        // 402: ldrcmd( ldr, b" ... th, ): typeof(_25) = *mut {l39} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
        // 402: ldrcmd( ldr, b" ... th, ):   l39 = UNIQUE | NON_NULL, (empty)
            ldr,
            // 403: ldr: typeof(_26) = *mut {l41} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 403: ldr:   l41 = UNIQUE | NON_NULL, (empty)
            b"gunzip -c %s\0" as *const u8 as *const libc::c_char,
            // 404: b"gunzip -c %s\ ... _char: typeof(_27) = *const {l43} i8
            // 404: b"gunzip -c %s\ ... _char:   l43 = UNIQUE | NON_NULL, (empty)
            // 404: b"gunzip -c %s\ ... st u8: typeof(_28) = *const {l45} u8
            // 404: b"gunzip -c %s\ ... st u8:   l45 = UNIQUE | NON_NULL, (empty)
            // 404: b"gunzip -c %s\0": typeof(_29) = *const {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 404: b"gunzip -c %s\0":   l47 = UNIQUE | NON_NULL, (empty)
            // 404: b"gunzip -c %s\0": typeof(_30) = & {l49} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 404: b"gunzip -c %s\0":   l49 = UNIQUE | NON_NULL, FIXED
            // 404: b"gunzip -c %s\0": typeof(_29 = &raw const (*_30)) = *const {l168} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 404: b"gunzip -c %s\0":   l168 = UNIQUE | NON_NULL, (empty)
            // 404: b"gunzip -c %s\ ... st u8: typeof(_28 = move _29 as *const u8 (Pointer(ArrayToPointer))) = *const {l169} u8
            // 404: b"gunzip -c %s\ ... st u8:   l169 = UNIQUE | NON_NULL, (empty)
            // 404: b"gunzip -c %s\ ... _char: typeof(_27 = move _28 as *const i8 (Misc)) = *const {l170} i8
            // 404: b"gunzip -c %s\ ... _char:   l170 = UNIQUE | NON_NULL, (empty)
            // 404: b"gunzip -c %s\0": typeof(_30 = const b"gunzip -c %s\x00") = & {l167} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000d)) }]
            // 404: b"gunzip -c %s\0":   l167 = UNIQUE | NON_NULL, (empty)
            path,
            // 405: path: typeof(_31) = *const {l51} i8
            // 405: path:   l51 = UNIQUE | NON_NULL, (empty)
        );
    } else if ldrhas(path, b".bz2\0" as *const u8 as *const libc::c_char) != 0 {
    // 407: path: typeof(_34) = *const {l55} i8
    // 407: path:   l55 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0" as *c ... _char: typeof(_35) = *const {l57} i8
    // 407: b".bz2\0" as *c ... _char:   l57 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0" as *c ... st u8: typeof(_36) = *const {l59} u8
    // 407: b".bz2\0" as *c ... st u8:   l59 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0": typeof(_37) = *const {l61} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 407: b".bz2\0":   l61 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0": typeof(_38) = & {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 407: b".bz2\0":   l63 = UNIQUE | NON_NULL, FIXED
    // 407: b".bz2\0" as *c ... _char: typeof(_35 = move _36 as *const i8 (Misc)) = *const {l174} i8
    // 407: b".bz2\0" as *c ... _char:   l174 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0": typeof(_37 = &raw const (*_38)) = *const {l172} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 407: b".bz2\0":   l172 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0" as *c ... st u8: typeof(_36 = move _37 as *const u8 (Pointer(ArrayToPointer))) = *const {l173} u8
    // 407: b".bz2\0" as *c ... st u8:   l173 = UNIQUE | NON_NULL, (empty)
    // 407: b".bz2\0": typeof(_38 = const b".bz2\x00") = & {l171} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000005)) }]
    // 407: b".bz2\0":   l171 = UNIQUE | NON_NULL, (empty)
        (*ldr).file = ldrcmd(ldr, b"bzcat %s\0" as *const u8 as *const libc::c_char, path);
        // 408: ldrcmd(ldr, b"b ... path): typeof(_39) = *mut {l65} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
        // 408: ldrcmd(ldr, b"b ... path):   l65 = UNIQUE | NON_NULL, (empty)
        // 408: ldr: typeof(_40) = *mut {l67} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 408: ldr:   l67 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0" a ... _char: typeof(_41) = *const {l69} i8
        // 408: b"bzcat %s\0" a ... _char:   l69 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0" a ... st u8: typeof(_42) = *const {l71} u8
        // 408: b"bzcat %s\0" a ... st u8:   l71 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0": typeof(_43) = *const {l73} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 408: b"bzcat %s\0":   l73 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0": typeof(_44) = & {l75} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 408: b"bzcat %s\0":   l75 = UNIQUE | NON_NULL, FIXED
        // 408: path: typeof(_45) = *const {l77} i8
        // 408: path:   l77 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0": typeof(_44 = const b"bzcat %s\x00") = & {l175} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 408: b"bzcat %s\0":   l175 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0" a ... st u8: typeof(_42 = move _43 as *const u8 (Pointer(ArrayToPointer))) = *const {l177} u8
        // 408: b"bzcat %s\0" a ... st u8:   l177 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0" a ... _char: typeof(_41 = move _42 as *const i8 (Misc)) = *const {l178} i8
        // 408: b"bzcat %s\0" a ... _char:   l178 = UNIQUE | NON_NULL, (empty)
        // 408: b"bzcat %s\0": typeof(_43 = &raw const (*_44)) = *const {l176} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 408: b"bzcat %s\0":   l176 = UNIQUE | NON_NULL, (empty)
    } else if ldrhas(path, b".7z\0" as *const u8 as *const libc::c_char) != 0 {
    // 409: path: typeof(_48) = *const {l81} i8
    // 409: path:   l81 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0" as *co ... _char: typeof(_49) = *const {l83} i8
    // 409: b".7z\0" as *co ... _char:   l83 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0" as *co ... st u8: typeof(_50) = *const {l85} u8
    // 409: b".7z\0" as *co ... st u8:   l85 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0": typeof(_51) = *const {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 409: b".7z\0":   l87 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0": typeof(_52) = & {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 409: b".7z\0":   l89 = UNIQUE | NON_NULL, FIXED
    // 409: b".7z\0" as *co ... _char: typeof(_49 = move _50 as *const i8 (Misc)) = *const {l182} i8
    // 409: b".7z\0" as *co ... _char:   l182 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0": typeof(_52 = const b".7z\x00") = & {l179} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 409: b".7z\0":   l179 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0" as *co ... st u8: typeof(_50 = move _51 as *const u8 (Pointer(ArrayToPointer))) = *const {l181} u8
    // 409: b".7z\0" as *co ... st u8:   l181 = UNIQUE | NON_NULL, (empty)
    // 409: b".7z\0": typeof(_51 = &raw const (*_52)) = *const {l180} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 409: b".7z\0":   l180 = UNIQUE | NON_NULL, (empty)
        (*ldr).file = ldrcmd(
        // 410: ldrcmd( ldr, b" ... th, ): typeof(_53) = *mut {l91} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
        // 410: ldrcmd( ldr, b" ... th, ):   l91 = UNIQUE | NON_NULL, (empty)
            ldr,
            // 411: ldr: typeof(_54) = *mut {l93} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 411: ldr:   l93 = UNIQUE | NON_NULL, (empty)
            b"7z x -so %s 2>/dev/null\0" as *const u8 as *const libc::c_char,
            // 412: b"7z x -so %s 2 ... _char: typeof(_55) = *const {l95} i8
            // 412: b"7z x -so %s 2 ... _char:   l95 = UNIQUE | NON_NULL, (empty)
            // 412: b"7z x -so %s 2 ... st u8: typeof(_56) = *const {l97} u8
            // 412: b"7z x -so %s 2 ... st u8:   l97 = UNIQUE | NON_NULL, (empty)
            // 412: b"7z x -so %s 2 ... ll\0": typeof(_57) = *const {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 412: b"7z x -so %s 2 ... ll\0":   l99 = UNIQUE | NON_NULL, (empty)
            // 412: b"7z x -so %s 2 ... ll\0": typeof(_58) = & {l101} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 412: b"7z x -so %s 2 ... ll\0":   l101 = UNIQUE | NON_NULL, FIXED
            // 412: b"7z x -so %s 2 ... _char: typeof(_55 = move _56 as *const i8 (Misc)) = *const {l186} i8
            // 412: b"7z x -so %s 2 ... _char:   l186 = UNIQUE | NON_NULL, (empty)
            // 412: b"7z x -so %s 2 ... ll\0": typeof(_57 = &raw const (*_58)) = *const {l184} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 412: b"7z x -so %s 2 ... ll\0":   l184 = UNIQUE | NON_NULL, (empty)
            // 412: b"7z x -so %s 2 ... st u8: typeof(_56 = move _57 as *const u8 (Pointer(ArrayToPointer))) = *const {l185} u8
            // 412: b"7z x -so %s 2 ... st u8:   l185 = UNIQUE | NON_NULL, (empty)
            // 412: b"7z x -so %s 2 ... ll\0": typeof(_58 = const b"7z x -so %s 2>/dev/null\x00") = & {l183} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 412: b"7z x -so %s 2 ... ll\0":   l183 = UNIQUE | NON_NULL, (empty)
            path,
            // 413: path: typeof(_59) = *const {l103} i8
            // 413: path:   l103 = UNIQUE | NON_NULL, (empty)
        );
    } else if ldrhas(path, b".lzma\0" as *const u8 as *const libc::c_char) != 0 {
    // 415: path: typeof(_62) = *const {l107} i8
    // 415: path:   l107 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0" as * ... _char: typeof(_63) = *const {l109} i8
    // 415: b".lzma\0" as * ... _char:   l109 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0" as * ... st u8: typeof(_64) = *const {l111} u8
    // 415: b".lzma\0" as * ... st u8:   l111 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0": typeof(_65) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
    // 415: b".lzma\0":   l113 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0": typeof(_66) = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
    // 415: b".lzma\0":   l115 = UNIQUE | NON_NULL, FIXED
    // 415: b".lzma\0" as * ... st u8: typeof(_64 = move _65 as *const u8 (Pointer(ArrayToPointer))) = *const {l189} u8
    // 415: b".lzma\0" as * ... st u8:   l189 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0": typeof(_66 = const b".lzma\x00") = & {l187} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
    // 415: b".lzma\0":   l187 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0": typeof(_65 = &raw const (*_66)) = *const {l188} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
    // 415: b".lzma\0":   l188 = UNIQUE | NON_NULL, (empty)
    // 415: b".lzma\0" as * ... _char: typeof(_63 = move _64 as *const i8 (Misc)) = *const {l190} i8
    // 415: b".lzma\0" as * ... _char:   l190 = UNIQUE | NON_NULL, (empty)
        (*ldr).file = ldrcmd(ldr, b"lzcat %s\0" as *const u8 as *const libc::c_char, path);
        // 416: ldrcmd(ldr, b"l ... path): typeof(_67) = *mut {l117} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
        // 416: ldrcmd(ldr, b"l ... path):   l117 = UNIQUE | NON_NULL, (empty)
        // 416: ldr: typeof(_68) = *mut {l119} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 416: ldr:   l119 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0" a ... _char: typeof(_69) = *const {l121} i8
        // 416: b"lzcat %s\0" a ... _char:   l121 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0" a ... st u8: typeof(_70) = *const {l123} u8
        // 416: b"lzcat %s\0" a ... st u8:   l123 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0": typeof(_71) = *const {l125} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 416: b"lzcat %s\0":   l125 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0": typeof(_72) = & {l127} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 416: b"lzcat %s\0":   l127 = UNIQUE | NON_NULL, FIXED
        // 416: path: typeof(_73) = *const {l129} i8
        // 416: path:   l129 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0": typeof(_71 = &raw const (*_72)) = *const {l192} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 416: b"lzcat %s\0":   l192 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0" a ... st u8: typeof(_70 = move _71 as *const u8 (Pointer(ArrayToPointer))) = *const {l193} u8
        // 416: b"lzcat %s\0" a ... st u8:   l193 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0": typeof(_72 = const b"lzcat %s\x00") = & {l191} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000009)) }]
        // 416: b"lzcat %s\0":   l191 = UNIQUE | NON_NULL, (empty)
        // 416: b"lzcat %s\0" a ... _char: typeof(_69 = move _70 as *const i8 (Misc)) = *const {l194} i8
        // 416: b"lzcat %s\0" a ... _char:   l194 = UNIQUE | NON_NULL, (empty)
    } else {
        (*ldr).file = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
        // 418: fopen(path, b"r ... char): typeof(_74) = *mut {l131} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
        // 418: fopen(path, b"r ... char):   l131 = UNIQUE | NON_NULL, (empty)
        // 418: path: typeof(_75) = *const {l133} i8
        // 418: path:   l133 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0" as *cons ... _char: typeof(_76) = *const {l135} i8
        // 418: b"r\0" as *cons ... _char:   l135 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0" as *const u8: typeof(_77) = *const {l137} u8
        // 418: b"r\0" as *const u8:   l137 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0": typeof(_78) = *const {l139} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 418: b"r\0":   l139 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0": typeof(_79) = & {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 418: b"r\0":   l141 = UNIQUE | NON_NULL, FIXED
        // 418: b"r\0": typeof(_79 = const b"r\x00") = & {l195} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 418: b"r\0":   l195 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0" as *const u8: typeof(_77 = move _78 as *const u8 (Pointer(ArrayToPointer))) = *const {l197} u8
        // 418: b"r\0" as *const u8:   l197 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0" as *cons ... _char: typeof(_76 = move _77 as *const i8 (Misc)) = *const {l198} i8
        // 418: b"r\0" as *cons ... _char:   l198 = UNIQUE | NON_NULL, (empty)
        // 418: b"r\0": typeof(_78 = &raw const (*_79)) = *const {l196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000002)) }]
        // 418: b"r\0":   l196 = UNIQUE | NON_NULL, (empty)
        (*ldr).closefile = 1 as libc::c_int;
    }
    if ((*ldr).file).is_null() {
    // 421: ((*ldr).file): typeof(_82) = *mut {l145} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 421: ((*ldr).file):   l145 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 423: ldr: typeof(_84) = *mut {l148} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 423: ldr:   l148 = UNIQUE | NON_NULL, (empty)
            b"can not open file\0" as *const u8 as *const libc::c_char,
            // 424: b"can not open  ... _char: typeof(_85) = *const {l150} i8
            // 424: b"can not open  ... _char:   l150 = UNIQUE | NON_NULL, (empty)
            // 424: b"can not open  ... st u8: typeof(_86) = *const {l152} u8
            // 424: b"can not open  ... st u8:   l152 = UNIQUE | NON_NULL, (empty)
            // 424: b"can not open  ... le\0": typeof(_87) = *const {l154} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 424: b"can not open  ... le\0":   l154 = UNIQUE | NON_NULL, (empty)
            // 424: b"can not open  ... le\0": typeof(_88) = & {l156} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 424: b"can not open  ... le\0":   l156 = UNIQUE | NON_NULL, FIXED
            // 424: b"can not open  ... st u8: typeof(_86 = move _87 as *const u8 (Pointer(ArrayToPointer))) = *const {l201} u8
            // 424: b"can not open  ... st u8:   l201 = UNIQUE | NON_NULL, (empty)
            // 424: b"can not open  ... _char: typeof(_85 = move _86 as *const i8 (Misc)) = *const {l202} i8
            // 424: b"can not open  ... _char:   l202 = UNIQUE | NON_NULL, (empty)
            // 424: b"can not open  ... le\0": typeof(_88 = const b"can not open file\x00") = & {l199} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 424: b"can not open  ... le\0":   l199 = UNIQUE | NON_NULL, (empty)
            // 424: b"can not open  ... le\0": typeof(_87 = &raw const (*_88)) = *const {l200} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000012)) }]
            // 424: b"can not open  ... le\0":   l200 = UNIQUE | NON_NULL, (empty)
        );
    } else {
        return 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetfile<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16>(mut ldr: &'h0 mut (lgldimacs::LDR<'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15>), mut file: &'h16 (FILE)) {
// 431: mut ldr: typeof(_1) = *mut {g48} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 431: mut ldr:   g48 = READ | WRITE | UNIQUE | NON_NULL, (empty)
// 431: mut file: typeof(_2) = *mut {g49} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
// 431: mut file:   g49 = UNIQUE | NON_NULL, (empty)
    (*ldr).file = core::ptr::addr_of!(*(file)).cast_mut();
    // 432: file: typeof(_3) = *mut {l3} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 432: file:   l3 = UNIQUE | NON_NULL, (empty)
    (*ldr).path = core::ptr::addr_of!(*&*(ldrstrdup(
    // 433: ldrstrdup( ldr, ... ar, ): typeof(_4) = *mut {l5} i8
    // 433: ldrstrdup( ldr, ... ar, ):   l5 = UNIQUE | NON_NULL, (empty)
        core::ptr::addr_of!(*&*(ldr)).cast_mut(),
        // 434: ldr: typeof(_5) = *mut {l7} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 434: ldr:   l7 = UNIQUE | NON_NULL, (empty)
        core::ptr::addr_of!(*(&*(b"<unspecified-path>\0") as *const u8 as *const libc::c_char)),
        // 435: b"<unspecified- ... _char: typeof(_6) = *const {l9} i8
        // 435: b"<unspecified- ... _char:   l9 = UNIQUE | NON_NULL, (empty)
        // 435: b"<unspecified- ... st u8: typeof(_7) = *const {l11} u8
        // 435: b"<unspecified- ... st u8:   l11 = UNIQUE | NON_NULL, (empty)
        // 435: b"<unspecified- ... h>\0": typeof(_8) = *const {l13} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 435: b"<unspecified- ... h>\0":   l13 = UNIQUE | NON_NULL, (empty)
        // 435: b"<unspecified- ... h>\0": typeof(_9) = & {l15} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 435: b"<unspecified- ... h>\0":   l15 = UNIQUE | NON_NULL, FIXED
        // 435: b"<unspecified- ... h>\0": typeof(_8 = &raw const (*_9)) = *const {l18} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 435: b"<unspecified- ... h>\0":   l18 = UNIQUE | NON_NULL, (empty)
        // 435: b"<unspecified- ... h>\0": typeof(_9 = const b"<unspecified-path>\x00") = & {l17} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000013)) }]
        // 435: b"<unspecified- ... h>\0":   l17 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
        // 435: b"<unspecified- ... _char: typeof(_6 = move _7 as *const i8 (Misc)) = *const {l20} i8
        // 435: b"<unspecified- ... _char:   l20 = UNIQUE | NON_NULL, (empty)
        // 435: b"<unspecified- ... st u8: typeof(_7 = move _8 as *const u8 (Pointer(ArrayToPointer))) = *const {l19} u8
        // 435: b"<unspecified- ... st u8:   l19 = UNIQUE | NON_NULL, (empty)
    )).cast_const()).cast_mut();
}
#[no_mangle]
pub unsafe extern "C" fn ldrsetnamedfile<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16,'h17>(
    mut ldr: &'h0 mut (lgldimacs::LDR<'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15>),
    // 440: mut ldr: typeof(_1) = *mut {g50} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 440: mut ldr:   g50 = READ | WRITE | UNIQUE | NON_NULL, (empty)
    mut file: &'h16 (FILE),
    // 441: mut file: typeof(_2) = *mut {g51} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 441: mut file:   g51 = UNIQUE | NON_NULL, (empty)
    mut path: &'h17 (libc::c_char),
    // 442: mut path: typeof(_3) = *const {g52} i8
    // 442: mut path:   g52 = UNIQUE | NON_NULL, (empty)
) {
    (*ldr).file = core::ptr::addr_of!(*(file)).cast_mut();
    // 444: file: typeof(_4) = *mut {l4} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 444: file:   l4 = UNIQUE | NON_NULL, (empty)
    (*ldr).path = core::ptr::addr_of!(*&*(ldrstrdup(core::ptr::addr_of!(*&*(ldr)).cast_mut(), core::ptr::addr_of!(*(path)))).cast_const()).cast_mut();
    // 445: ldrstrdup(ldr, path): typeof(_5) = *mut {l6} i8
    // 445: ldrstrdup(ldr, path):   l6 = UNIQUE | NON_NULL, (empty)
    // 445: ldr: typeof(_6) = *mut {l8} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 445: ldr:   l8 = UNIQUE | NON_NULL, (empty)
    // 445: path: typeof(_7) = *const {l10} i8
    // 445: path:   l10 = UNIQUE | NON_NULL, (empty)
}
#[no_mangle]
pub unsafe extern "C" fn ldrerr<'h0,'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15,'h16>(mut ldr: &'h0 (lgldimacs::LDR<'h1,'h2,'h3,'h4,'h5,'h6,'h7,'h8,'h9,'h10,'h11,'h12,'h13,'h14,'h15>)) -> &'h16 (libc::c_char) {
// 448: *const libc::c_char: typeof(_0) = *const {g54} i8
// 448: *const libc::c_char:   g54 = UNIQUE | NON_NULL, (empty)
// 448: mut ldr: typeof(_1) = *mut {g53} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 448: mut ldr:   g53 = READ | UNIQUE | NON_NULL, (empty)
    return &*((*ldr).errmsg).cast_const();
    // 449: (*ldr).errmsg: typeof(_3) = *mut {l3} i8
    // 449: (*ldr).errmsg:   l3 = UNIQUE | NON_NULL, (empty)
    // 449: (*ldr).errmsg: typeof(_0 = move _3 as *const i8 (Pointer(MutToConstPointer))) = *const {l5} i8
    // 449: (*ldr).errmsg:   l5 = UNIQUE | NON_NULL, (empty)
}
unsafe extern "C" fn ldrnext(mut ldr: *mut LDR) -> libc::c_int {
// 451: mut ldr: typeof(_1) = *mut {g55} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 451: mut ldr:   g55 = UNIQUE | NON_NULL, FIXED
    let mut ch: libc::c_int = 0;
    ch = getc((*ldr).file);
    // 453: (*ldr).file: typeof(_5) = *mut {l5} DefId(0:1731 ~ libgeling[8679]::lgldimacs::_IO_FILE)
    // 453: (*ldr).file:   l5 = UNIQUE | NON_NULL, (empty)
    if ch == '\n' as i32 {
        (*ldr).lineno += 1;
        (*ldr).lineno;
    }
    return ch;
}
#[no_mangle]
pub unsafe extern "C" fn ldrparse(mut ldr: *mut LDR) -> libc::c_int {
// 461: mut ldr: typeof(_1) = *mut {g56} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
// 461: mut ldr:   g56 = UNIQUE | NON_NULL, FIXED
    let mut current_block: u64;
    let mut vars: C2RustUnnamed_3 = C2RustUnnamed_3 {
        parsed: 0,
        specified: 0,
    };
    let mut clauses: C2RustUnnamed_3 = C2RustUnnamed_3 {
        parsed: 0,
        specified: 0,
    };
    let mut ch: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut lit: libc::c_int = 0;
    let mut digit: libc::c_int = 0;
    if !((*ldr).errmsg).is_null() {
    // 475: ((*ldr).errmsg): typeof(_13) = *mut {l13} i8
    // 475: ((*ldr).errmsg):   l13 = UNIQUE | NON_NULL, (empty)
        return 0 as libc::c_int;
    }
    loop {
        ch = ldrnext(ldr);
        // 479: ldr: typeof(_18) = *mut {l19} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 479: ldr:   l19 = UNIQUE | NON_NULL, (empty)
        if !(ch == 'c' as i32) {
            break;
        }
        loop {
            ch = ldrnext(ldr);
            // 484: ldr: typeof(_26) = *mut {l28} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 484: ldr:   l28 = UNIQUE | NON_NULL, (empty)
            if !(ch != '\n' as i32) {
                break;
            }
            if ch == -(1 as libc::c_int) {
                return ldrperr(
                    ldr,
                    // 490: ldr: typeof(_39) = *mut {l42} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                    // 490: ldr:   l42 = UNIQUE | NON_NULL, (empty)
                    b"end-of-file in comment before header\0" as *const u8 as *const libc::c_char,
                    // 491: b"end-of-file i ... _char: typeof(_40) = *const {l44} i8
                    // 491: b"end-of-file i ... _char:   l44 = UNIQUE | NON_NULL, (empty)
                    // 491: b"end-of-file i ... st u8: typeof(_41) = *const {l46} u8
                    // 491: b"end-of-file i ... st u8:   l46 = UNIQUE | NON_NULL, (empty)
                    // 491: b"end-of-file i ... er\0": typeof(_42) = *const {l48} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                    // 491: b"end-of-file i ... er\0":   l48 = UNIQUE | NON_NULL, (empty)
                    // 491: b"end-of-file i ... er\0": typeof(_43) = & {l50} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                    // 491: b"end-of-file i ... er\0":   l50 = UNIQUE | NON_NULL, FIXED
                    // 491: b"end-of-file i ... er\0": typeof(_42 = &raw const (*_43)) = *const {l668} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                    // 491: b"end-of-file i ... er\0":   l668 = UNIQUE | NON_NULL, (empty)
                    // 491: b"end-of-file i ... er\0": typeof(_43 = const b"end-of-file in comment before header\x00") = & {l667} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
                    // 491: b"end-of-file i ... er\0":   l667 = UNIQUE | NON_NULL, (empty)
                    // 491: b"end-of-file i ... _char: typeof(_40 = move _41 as *const i8 (Misc)) = *const {l670} i8
                    // 491: b"end-of-file i ... _char:   l670 = UNIQUE | NON_NULL, (empty)
                    // 491: b"end-of-file i ... st u8: typeof(_41 = move _42 as *const u8 (Pointer(ArrayToPointer))) = *const {l669} u8
                    // 491: b"end-of-file i ... st u8:   l669 = UNIQUE | NON_NULL, (empty)
                );
            }
        }
    }
    if ch != 'p' as i32 {
        return ldrperr(
            ldr,
            // 498: ldr: typeof(_49) = *mut {l57} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 498: ldr:   l57 = UNIQUE | NON_NULL, (empty)
            b"expected 'p' or 'c'\0" as *const u8 as *const libc::c_char,
            // 499: b"expected 'p'  ... _char: typeof(_50) = *const {l59} i8
            // 499: b"expected 'p'  ... _char:   l59 = UNIQUE | NON_NULL, (empty)
            // 499: b"expected 'p'  ... st u8: typeof(_51) = *const {l61} u8
            // 499: b"expected 'p'  ... st u8:   l61 = UNIQUE | NON_NULL, (empty)
            // 499: b"expected 'p'  ... c'\0": typeof(_52) = *const {l63} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 499: b"expected 'p'  ... c'\0":   l63 = UNIQUE | NON_NULL, (empty)
            // 499: b"expected 'p'  ... c'\0": typeof(_53) = & {l65} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 499: b"expected 'p'  ... c'\0":   l65 = UNIQUE | NON_NULL, FIXED
            // 499: b"expected 'p'  ... c'\0": typeof(_52 = &raw const (*_53)) = *const {l672} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 499: b"expected 'p'  ... c'\0":   l672 = UNIQUE | NON_NULL, (empty)
            // 499: b"expected 'p'  ... c'\0": typeof(_53 = const b"expected \'p\' or \'c\'\x00") = & {l671} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 499: b"expected 'p'  ... c'\0":   l671 = UNIQUE | NON_NULL, (empty)
            // 499: b"expected 'p'  ... st u8: typeof(_51 = move _52 as *const u8 (Pointer(ArrayToPointer))) = *const {l673} u8
            // 499: b"expected 'p'  ... st u8:   l673 = UNIQUE | NON_NULL, (empty)
            // 499: b"expected 'p'  ... _char: typeof(_50 = move _51 as *const i8 (Misc)) = *const {l674} i8
            // 499: b"expected 'p'  ... _char:   l674 = UNIQUE | NON_NULL, (empty)
        );
    }
    if ldrnext(ldr) != ' ' as i32 {
    // 502: ldr: typeof(_57) = *mut {l70} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 502: ldr:   l70 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 504: ldr: typeof(_60) = *mut {l74} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 504: ldr:   l74 = UNIQUE | NON_NULL, (empty)
            b"expected space after 'p'\0" as *const u8 as *const libc::c_char,
            // 505: b"expected spac ... _char: typeof(_61) = *const {l76} i8
            // 505: b"expected spac ... _char:   l76 = UNIQUE | NON_NULL, (empty)
            // 505: b"expected spac ... st u8: typeof(_62) = *const {l78} u8
            // 505: b"expected spac ... st u8:   l78 = UNIQUE | NON_NULL, (empty)
            // 505: b"expected spac ... p'\0": typeof(_63) = *const {l80} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 505: b"expected spac ... p'\0":   l80 = UNIQUE | NON_NULL, (empty)
            // 505: b"expected spac ... p'\0": typeof(_64) = & {l82} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 505: b"expected spac ... p'\0":   l82 = UNIQUE | NON_NULL, FIXED
            // 505: b"expected spac ... st u8: typeof(_62 = move _63 as *const u8 (Pointer(ArrayToPointer))) = *const {l677} u8
            // 505: b"expected spac ... st u8:   l677 = UNIQUE | NON_NULL, (empty)
            // 505: b"expected spac ... _char: typeof(_61 = move _62 as *const i8 (Misc)) = *const {l678} i8
            // 505: b"expected spac ... _char:   l678 = UNIQUE | NON_NULL, (empty)
            // 505: b"expected spac ... p'\0": typeof(_64 = const b"expected space after \'p\'\x00") = & {l675} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 505: b"expected spac ... p'\0":   l675 = UNIQUE | NON_NULL, (empty)
            // 505: b"expected spac ... p'\0": typeof(_63 = &raw const (*_64)) = *const {l676} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 505: b"expected spac ... p'\0":   l676 = UNIQUE | NON_NULL, (empty)
        );
    }
    if ldrnext(ldr) != 'c' as i32 {
    // 508: ldr: typeof(_68) = *mut {l87} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 508: ldr:   l87 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 510: ldr: typeof(_71) = *mut {l91} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 510: ldr:   l91 = UNIQUE | NON_NULL, (empty)
            b"expected 'c' after 'p '\0" as *const u8 as *const libc::c_char,
            // 511: b"expected 'c'  ... _char: typeof(_72) = *const {l93} i8
            // 511: b"expected 'c'  ... _char:   l93 = UNIQUE | NON_NULL, (empty)
            // 511: b"expected 'c'  ... st u8: typeof(_73) = *const {l95} u8
            // 511: b"expected 'c'  ... st u8:   l95 = UNIQUE | NON_NULL, (empty)
            // 511: b"expected 'c'  ...  '\0": typeof(_74) = *const {l97} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 511: b"expected 'c'  ...  '\0":   l97 = UNIQUE | NON_NULL, (empty)
            // 511: b"expected 'c'  ...  '\0": typeof(_75) = & {l99} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 511: b"expected 'c'  ...  '\0":   l99 = UNIQUE | NON_NULL, FIXED
            // 511: b"expected 'c'  ... st u8: typeof(_73 = move _74 as *const u8 (Pointer(ArrayToPointer))) = *const {l681} u8
            // 511: b"expected 'c'  ... st u8:   l681 = UNIQUE | NON_NULL, (empty)
            // 511: b"expected 'c'  ... _char: typeof(_72 = move _73 as *const i8 (Misc)) = *const {l682} i8
            // 511: b"expected 'c'  ... _char:   l682 = UNIQUE | NON_NULL, (empty)
            // 511: b"expected 'c'  ...  '\0": typeof(_74 = &raw const (*_75)) = *const {l680} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 511: b"expected 'c'  ...  '\0":   l680 = UNIQUE | NON_NULL, (empty)
            // 511: b"expected 'c'  ...  '\0": typeof(_75 = const b"expected \'c\' after \'p \'\x00") = & {l679} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
            // 511: b"expected 'c'  ...  '\0":   l679 = UNIQUE | NON_NULL, (empty)
        );
    }
    if ldrnext(ldr) != 'n' as i32 {
    // 514: ldr: typeof(_79) = *mut {l104} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 514: ldr:   l104 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 516: ldr: typeof(_82) = *mut {l108} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 516: ldr:   l108 = UNIQUE | NON_NULL, (empty)
            b"expected 'n' after 'p c'\0" as *const u8 as *const libc::c_char,
            // 517: b"expected 'n'  ... _char: typeof(_83) = *const {l110} i8
            // 517: b"expected 'n'  ... _char:   l110 = UNIQUE | NON_NULL, (empty)
            // 517: b"expected 'n'  ... st u8: typeof(_84) = *const {l112} u8
            // 517: b"expected 'n'  ... st u8:   l112 = UNIQUE | NON_NULL, (empty)
            // 517: b"expected 'n'  ... c'\0": typeof(_85) = *const {l114} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 517: b"expected 'n'  ... c'\0":   l114 = UNIQUE | NON_NULL, (empty)
            // 517: b"expected 'n'  ... c'\0": typeof(_86) = & {l116} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 517: b"expected 'n'  ... c'\0":   l116 = UNIQUE | NON_NULL, FIXED
            // 517: b"expected 'n'  ... st u8: typeof(_84 = move _85 as *const u8 (Pointer(ArrayToPointer))) = *const {l685} u8
            // 517: b"expected 'n'  ... st u8:   l685 = UNIQUE | NON_NULL, (empty)
            // 517: b"expected 'n'  ... c'\0": typeof(_86 = const b"expected \'n\' after \'p c\'\x00") = & {l683} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 517: b"expected 'n'  ... c'\0":   l683 = UNIQUE | NON_NULL, (empty)
            // 517: b"expected 'n'  ... _char: typeof(_83 = move _84 as *const i8 (Misc)) = *const {l686} i8
            // 517: b"expected 'n'  ... _char:   l686 = UNIQUE | NON_NULL, (empty)
            // 517: b"expected 'n'  ... c'\0": typeof(_85 = &raw const (*_86)) = *const {l684} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
            // 517: b"expected 'n'  ... c'\0":   l684 = UNIQUE | NON_NULL, (empty)
        );
    }
    if ldrnext(ldr) != 'f' as i32 {
    // 520: ldr: typeof(_90) = *mut {l121} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 520: ldr:   l121 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 522: ldr: typeof(_93) = *mut {l125} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 522: ldr:   l125 = UNIQUE | NON_NULL, (empty)
            b"expected 'f' after 'p cn'\0" as *const u8 as *const libc::c_char,
            // 523: b"expected 'f'  ... _char: typeof(_94) = *const {l127} i8
            // 523: b"expected 'f'  ... _char:   l127 = UNIQUE | NON_NULL, (empty)
            // 523: b"expected 'f'  ... st u8: typeof(_95) = *const {l129} u8
            // 523: b"expected 'f'  ... st u8:   l129 = UNIQUE | NON_NULL, (empty)
            // 523: b"expected 'f'  ... n'\0": typeof(_96) = *const {l131} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 523: b"expected 'f'  ... n'\0":   l131 = UNIQUE | NON_NULL, (empty)
            // 523: b"expected 'f'  ... n'\0": typeof(_97) = & {l133} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 523: b"expected 'f'  ... n'\0":   l133 = UNIQUE | NON_NULL, FIXED
            // 523: b"expected 'f'  ... _char: typeof(_94 = move _95 as *const i8 (Misc)) = *const {l690} i8
            // 523: b"expected 'f'  ... _char:   l690 = UNIQUE | NON_NULL, (empty)
            // 523: b"expected 'f'  ... n'\0": typeof(_97 = const b"expected \'f\' after \'p cn\'\x00") = & {l687} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 523: b"expected 'f'  ... n'\0":   l687 = UNIQUE | NON_NULL, (empty)
            // 523: b"expected 'f'  ... st u8: typeof(_95 = move _96 as *const u8 (Pointer(ArrayToPointer))) = *const {l689} u8
            // 523: b"expected 'f'  ... st u8:   l689 = UNIQUE | NON_NULL, (empty)
            // 523: b"expected 'f'  ... n'\0": typeof(_96 = &raw const (*_97)) = *const {l688} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001a)) }]
            // 523: b"expected 'f'  ... n'\0":   l688 = UNIQUE | NON_NULL, (empty)
        );
    }
    if ldrnext(ldr) != ' ' as i32 {
    // 526: ldr: typeof(_101) = *mut {l138} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 526: ldr:   l138 = UNIQUE | NON_NULL, (empty)
        return ldrperr(
            ldr,
            // 528: ldr: typeof(_104) = *mut {l142} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 528: ldr:   l142 = UNIQUE | NON_NULL, (empty)
            b"expected space after 'p cnf'\0" as *const u8 as *const libc::c_char,
            // 529: b"expected spac ... _char: typeof(_105) = *const {l144} i8
            // 529: b"expected spac ... _char:   l144 = UNIQUE | NON_NULL, (empty)
            // 529: b"expected spac ... st u8: typeof(_106) = *const {l146} u8
            // 529: b"expected spac ... st u8:   l146 = UNIQUE | NON_NULL, (empty)
            // 529: b"expected spac ... f'\0": typeof(_107) = *const {l148} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 529: b"expected spac ... f'\0":   l148 = UNIQUE | NON_NULL, (empty)
            // 529: b"expected spac ... f'\0": typeof(_108) = & {l150} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 529: b"expected spac ... f'\0":   l150 = UNIQUE | NON_NULL, FIXED
            // 529: b"expected spac ... _char: typeof(_105 = move _106 as *const i8 (Misc)) = *const {l694} i8
            // 529: b"expected spac ... _char:   l694 = UNIQUE | NON_NULL, (empty)
            // 529: b"expected spac ... st u8: typeof(_106 = move _107 as *const u8 (Pointer(ArrayToPointer))) = *const {l693} u8
            // 529: b"expected spac ... st u8:   l693 = UNIQUE | NON_NULL, (empty)
            // 529: b"expected spac ... f'\0": typeof(_107 = &raw const (*_108)) = *const {l692} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 529: b"expected spac ... f'\0":   l692 = UNIQUE | NON_NULL, (empty)
            // 529: b"expected spac ... f'\0": typeof(_108 = const b"expected space after \'p cnf\'\x00") = & {l691} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
            // 529: b"expected spac ... f'\0":   l691 = UNIQUE | NON_NULL, (empty)
        );
    }
    ch = ldrnext(ldr);
    // 532: ldr: typeof(_110) = *mut {l153} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
    // 532: ldr:   l153 = UNIQUE | NON_NULL, (empty)
    if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
    // 533: (*__ctype_b_loc ... size): typeof(_116) = *const {l160} u16
    // 533: (*__ctype_b_loc ... size):   l160 = UNIQUE | NON_NULL, (empty)
    // 533: (*__ctype_b_loc()): typeof(_117) = *const {l162} u16
    // 533: (*__ctype_b_loc()):   l162 = UNIQUE | NON_NULL, (empty)
    // 533: __ctype_b_loc(): typeof(_118) = *mut {l164} *const {l165} u16
    // 533: __ctype_b_loc():   l164 = UNIQUE | NON_NULL, (empty)
    // 533: __ctype_b_loc():   l165 = UNIQUE | NON_NULL, (empty)
        & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
        == 0
    {
        return ldrperr(
            ldr,
            // 538: ldr: typeof(_125) = *mut {l173} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 538: ldr:   l173 = UNIQUE | NON_NULL, (empty)
            b"expected digit after 'p cnf '\0" as *const u8 as *const libc::c_char,
            // 539: b"expected digi ... _char: typeof(_126) = *const {l175} i8
            // 539: b"expected digi ... _char:   l175 = UNIQUE | NON_NULL, (empty)
            // 539: b"expected digi ... st u8: typeof(_127) = *const {l177} u8
            // 539: b"expected digi ... st u8:   l177 = UNIQUE | NON_NULL, (empty)
            // 539: b"expected digi ...  '\0": typeof(_128) = *const {l179} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
            // 539: b"expected digi ...  '\0":   l179 = UNIQUE | NON_NULL, (empty)
            // 539: b"expected digi ...  '\0": typeof(_129) = & {l181} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
            // 539: b"expected digi ...  '\0":   l181 = UNIQUE | NON_NULL, FIXED
            // 539: b"expected digi ... st u8: typeof(_127 = move _128 as *const u8 (Pointer(ArrayToPointer))) = *const {l697} u8
            // 539: b"expected digi ... st u8:   l697 = UNIQUE | NON_NULL, (empty)
            // 539: b"expected digi ...  '\0": typeof(_128 = &raw const (*_129)) = *const {l696} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
            // 539: b"expected digi ...  '\0":   l696 = UNIQUE | NON_NULL, (empty)
            // 539: b"expected digi ... _char: typeof(_126 = move _127 as *const i8 (Misc)) = *const {l698} i8
            // 539: b"expected digi ... _char:   l698 = UNIQUE | NON_NULL, (empty)
            // 539: b"expected digi ...  '\0": typeof(_129 = const b"expected digit after \'p cnf \'\x00") = & {l695} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001e)) }]
            // 539: b"expected digi ...  '\0":   l695 = UNIQUE | NON_NULL, (empty)
        );
    }
    vars.specified = ch - '0' as i32;
    loop {
        ch = ldrnext(ldr);
        // 544: ldr: typeof(_135) = *mut {l188} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
        // 544: ldr:   l188 = UNIQUE | NON_NULL, (empty)
        if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
        // 545: (*__ctype_b_loc ... size): typeof(_140) = *const {l194} u16
        // 545: (*__ctype_b_loc ... size):   l194 = UNIQUE | NON_NULL, (empty)
        // 545: (*__ctype_b_loc()): typeof(_141) = *const {l196} u16
        // 545: (*__ctype_b_loc()):   l196 = UNIQUE | NON_NULL, (empty)
        // 545: __ctype_b_loc(): typeof(_142) = *mut {l198} *const {l199} u16
        // 545: __ctype_b_loc():   l198 = UNIQUE | NON_NULL, (empty)
        // 545: __ctype_b_loc():   l199 = UNIQUE | NON_NULL, (empty)
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            if (2147483647 as libc::c_int / 10 as libc::c_int) < vars.specified {
                current_block = 14777158775944759451;
                break;
            }
            vars.specified *= 10 as libc::c_int;
            digit = ch - '0' as i32;
            if 2147483647 as libc::c_int - digit < vars.specified {
                current_block = 14777158775944759451;
                break;
            }
            vars.specified += digit;
        } else {
            if ch != ' ' as i32 {
                return ldrperr(
                    ldr,
                    // 563: ldr: typeof(_180) = *mut {l238} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                    // 563: ldr:   l238 = UNIQUE | NON_NULL, (empty)
                    b"expected space after maximum variable index\0" as *const u8
                    // 564: b"expected spac ... _char: typeof(_181) = *const {l240} i8
                    // 564: b"expected spac ... _char:   l240 = UNIQUE | NON_NULL, (empty)
                    // 564: b"expected spac ... st u8: typeof(_182) = *const {l242} u8
                    // 564: b"expected spac ... st u8:   l242 = UNIQUE | NON_NULL, (empty)
                    // 564: b"expected spac ... ex\0": typeof(_183) = *const {l244} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 564: b"expected spac ... ex\0":   l244 = UNIQUE | NON_NULL, (empty)
                    // 564: b"expected spac ... ex\0": typeof(_184) = & {l246} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 564: b"expected spac ... ex\0":   l246 = UNIQUE | NON_NULL, FIXED
                    // 564: b"expected spac ... ex\0": typeof(_184 = const b"expected space after maximum variable index\x00") = & {l699} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 564: b"expected spac ... ex\0":   l699 = UNIQUE | NON_NULL, (empty)
                    // 564: b"expected spac ... st u8: typeof(_182 = move _183 as *const u8 (Pointer(ArrayToPointer))) = *const {l701} u8
                    // 564: b"expected spac ... st u8:   l701 = UNIQUE | NON_NULL, (empty)
                    // 564: b"expected spac ... _char: typeof(_181 = move _182 as *const i8 (Misc)) = *const {l702} i8
                    // 564: b"expected spac ... _char:   l702 = UNIQUE | NON_NULL, (empty)
                    // 564: b"expected spac ... ex\0": typeof(_183 = &raw const (*_184)) = *const {l700} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002c)) }]
                    // 564: b"expected spac ... ex\0":   l700 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
            }
            ch = ldrnext(ldr);
            // 568: ldr: typeof(_186) = *mut {l249} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 568: ldr:   l249 = UNIQUE | NON_NULL, (empty)
            if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
            // 569: (*__ctype_b_loc ... size): typeof(_192) = *const {l256} u16
            // 569: (*__ctype_b_loc ... size):   l256 = UNIQUE | NON_NULL, (empty)
            // 569: (*__ctype_b_loc()): typeof(_193) = *const {l258} u16
            // 569: (*__ctype_b_loc()):   l258 = UNIQUE | NON_NULL, (empty)
            // 569: __ctype_b_loc(): typeof(_194) = *mut {l260} *const {l261} u16
            // 569: __ctype_b_loc():   l260 = UNIQUE | NON_NULL, (empty)
            // 569: __ctype_b_loc():   l261 = UNIQUE | NON_NULL, (empty)
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                == 0
            {
                return ldrperr(
                    ldr,
                    // 574: ldr: typeof(_201) = *mut {l269} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                    // 574: ldr:   l269 = UNIQUE | NON_NULL, (empty)
                    b"expected digit after space after variable index\0" as *const u8
                    // 575: b"expected digi ... _char: typeof(_202) = *const {l271} i8
                    // 575: b"expected digi ... _char:   l271 = UNIQUE | NON_NULL, (empty)
                    // 575: b"expected digi ... st u8: typeof(_203) = *const {l273} u8
                    // 575: b"expected digi ... st u8:   l273 = UNIQUE | NON_NULL, (empty)
                    // 575: b"expected digi ... ex\0": typeof(_204) = *const {l275} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 575: b"expected digi ... ex\0":   l275 = UNIQUE | NON_NULL, (empty)
                    // 575: b"expected digi ... ex\0": typeof(_205) = & {l277} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 575: b"expected digi ... ex\0":   l277 = UNIQUE | NON_NULL, FIXED
                    // 575: b"expected digi ... ex\0": typeof(_204 = &raw const (*_205)) = *const {l704} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 575: b"expected digi ... ex\0":   l704 = UNIQUE | NON_NULL, (empty)
                    // 575: b"expected digi ... ex\0": typeof(_205 = const b"expected digit after space after variable index\x00") = & {l703} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000030)) }]
                    // 575: b"expected digi ... ex\0":   l703 = UNIQUE | NON_NULL, (empty)
                    // 575: b"expected digi ... _char: typeof(_202 = move _203 as *const i8 (Misc)) = *const {l706} i8
                    // 575: b"expected digi ... _char:   l706 = UNIQUE | NON_NULL, (empty)
                    // 575: b"expected digi ... st u8: typeof(_203 = move _204 as *const u8 (Pointer(ArrayToPointer))) = *const {l705} u8
                    // 575: b"expected digi ... st u8:   l705 = UNIQUE | NON_NULL, (empty)
                        as *const libc::c_char,
                );
            }
            clauses.specified = ch - '0' as i32;
            current_block = 15125582407903384992;
            break;
        }
    }
    '_NUMBER_TOO_LARGE: loop {
        match current_block {
            14777158775944759451 => {
                return ldrperr(
                    ldr,
                    // 588: ldr: typeof(_211) = *mut {l284} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                    // 588: ldr:   l284 = UNIQUE | NON_NULL, (empty)
                    b"number too large\0" as *const u8 as *const libc::c_char,
                    // 589: b"number too la ... _char: typeof(_212) = *const {l286} i8
                    // 589: b"number too la ... _char:   l286 = UNIQUE | NON_NULL, (empty)
                    // 589: b"number too la ... st u8: typeof(_213) = *const {l288} u8
                    // 589: b"number too la ... st u8:   l288 = UNIQUE | NON_NULL, (empty)
                    // 589: b"number too la ... ge\0": typeof(_214) = *const {l290} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 589: b"number too la ... ge\0":   l290 = UNIQUE | NON_NULL, (empty)
                    // 589: b"number too la ... ge\0": typeof(_215) = & {l292} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 589: b"number too la ... ge\0":   l292 = UNIQUE | NON_NULL, FIXED
                    // 589: b"number too la ... ge\0": typeof(_215 = const b"number too large\x00") = & {l707} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 589: b"number too la ... ge\0":   l707 = UNIQUE | NON_NULL, (empty)
                    // 589: b"number too la ... ge\0": typeof(_214 = &raw const (*_215)) = *const {l708} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                    // 589: b"number too la ... ge\0":   l708 = UNIQUE | NON_NULL, (empty)
                    // 589: b"number too la ... _char: typeof(_212 = move _213 as *const i8 (Misc)) = *const {l710} i8
                    // 589: b"number too la ... _char:   l710 = UNIQUE | NON_NULL, (empty)
                    // 589: b"number too la ... st u8: typeof(_213 = move _214 as *const u8 (Pointer(ArrayToPointer))) = *const {l709} u8
                    // 589: b"number too la ... st u8:   l709 = UNIQUE | NON_NULL, (empty)
                );
            }
            _ => {
                ch = ldrnext(ldr);
                // 593: ldr: typeof(_217) = *mut {l295} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                // 593: ldr:   l295 = UNIQUE | NON_NULL, (empty)
                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                // 594: (*__ctype_b_loc ... size): typeof(_222) = *const {l301} u16
                // 594: (*__ctype_b_loc ... size):   l301 = UNIQUE | NON_NULL, (empty)
                // 594: (*__ctype_b_loc()): typeof(_223) = *const {l303} u16
                // 594: (*__ctype_b_loc()):   l303 = UNIQUE | NON_NULL, (empty)
                // 594: __ctype_b_loc(): typeof(_224) = *mut {l305} *const {l306} u16
                // 594: __ctype_b_loc():   l305 = UNIQUE | NON_NULL, (empty)
                // 594: __ctype_b_loc():   l306 = UNIQUE | NON_NULL, (empty)
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                    != 0
                {
                    if (2147483647 as libc::c_int / 10 as libc::c_int) < clauses.specified {
                        current_block = 14777158775944759451;
                        continue;
                    }
                    clauses.specified *= 10 as libc::c_int;
                    digit = ch - '0' as i32;
                    if 2147483647 as libc::c_int - digit < clauses.specified {
                        current_block = 14777158775944759451;
                        continue;
                    }
                    clauses.specified += digit;
                    current_block = 15125582407903384992;
                } else {
                    while ch == ' ' as i32 || ch == '\t' as i32 || ch == '\r' as i32 {
                        ch = ldrnext(ldr);
                        // 612: ldr: typeof(_269) = *mut {l352} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                        // 612: ldr:   l352 = UNIQUE | NON_NULL, (empty)
                    }
                    if ch != '\n' as i32 {
                        return ldrperr(
                            ldr,
                            // 616: ldr: typeof(_278) = *mut {l362} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                            // 616: ldr:   l362 = UNIQUE | NON_NULL, (empty)
                            b"expected new line after header\0" as *const u8 as *const libc::c_char,
                            // 617: b"expected new  ... _char: typeof(_279) = *const {l364} i8
                            // 617: b"expected new  ... _char:   l364 = UNIQUE | NON_NULL, (empty)
                            // 617: b"expected new  ... st u8: typeof(_280) = *const {l366} u8
                            // 617: b"expected new  ... st u8:   l366 = UNIQUE | NON_NULL, (empty)
                            // 617: b"expected new  ... er\0": typeof(_281) = *const {l368} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                            // 617: b"expected new  ... er\0":   l368 = UNIQUE | NON_NULL, (empty)
                            // 617: b"expected new  ... er\0": typeof(_282) = & {l370} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                            // 617: b"expected new  ... er\0":   l370 = UNIQUE | NON_NULL, FIXED
                            // 617: b"expected new  ... _char: typeof(_279 = move _280 as *const i8 (Misc)) = *const {l714} i8
                            // 617: b"expected new  ... _char:   l714 = UNIQUE | NON_NULL, (empty)
                            // 617: b"expected new  ... st u8: typeof(_280 = move _281 as *const u8 (Pointer(ArrayToPointer))) = *const {l713} u8
                            // 617: b"expected new  ... st u8:   l713 = UNIQUE | NON_NULL, (empty)
                            // 617: b"expected new  ... er\0": typeof(_281 = &raw const (*_282)) = *const {l712} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                            // 617: b"expected new  ... er\0":   l712 = UNIQUE | NON_NULL, (empty)
                            // 617: b"expected new  ... er\0": typeof(_282 = const b"expected new line after header\x00") = & {l711} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001f)) }]
                            // 617: b"expected new  ... er\0":   l711 = UNIQUE | NON_NULL, (empty)
                        );
                    }
                    if ((*ldr).header.fun).is_some() {
                    // 620: ((*ldr).header. ... ome(): typeof(_285) = & {l374} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l375} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
                    // 620: ((*ldr).header. ... ome():   l374 = UNIQUE | NON_NULL, (empty)
                    // 620: ((*ldr).header. ... ome():   l375 = UNIQUE | NON_NULL, (empty)
                    // 620: ((*ldr).header. ... ome(): typeof(_285 = &(((*_1).2: lgldimacs::C2RustUnnamed_0).1: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, i32, i32)>)) = & {l715} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l716} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
                    // 620: ((*ldr).header. ... ome():   l715 = UNIQUE | NON_NULL, (empty)
                    // 620: ((*ldr).header. ... ome():   l716 = UNIQUE | NON_NULL, (empty)
                        ((*ldr).header.fun).expect("non-null function pointer")(
                        // 621: ((*ldr).header. ... ter"): typeof(_287) = fn(*mut {l378} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()
                        // 621: ((*ldr).header. ... ter"):   l378 = UNIQUE | NON_NULL, (empty)
                        // 621: ((*ldr).header.fun): typeof(_288) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l380} DefId(2:5583 ~ core[480a]::ffi::c_void), i32, i32) -> ()>
                        // 621: ((*ldr).header.fun):   l380 = UNIQUE | NON_NULL, (empty)
                        // 621: "non-null funct ... nter": typeof(_289) = & {l382} str
                        // 621: "non-null funct ... nter":   l382 = UNIQUE | NON_NULL, (empty)
                        // 621: "non-null funct ... nter": typeof(_290) = & {l384} str
                        // 621: "non-null funct ... nter":   l384 = UNIQUE | NON_NULL, FIXED
                        // 621: "non-null funct ... nter": typeof(_289 = &(*_290)) = & {l718} str
                        // 621: "non-null funct ... nter":   l718 = UNIQUE | NON_NULL, (empty)
                        // 621: "non-null funct ... nter": typeof(_290 = const "non-null function pointer") = & {l717} str
                        // 621: "non-null funct ... nter":   l717 = UNIQUE | NON_NULL, (empty)
                            (*ldr).header.state,
                            // 622: (*ldr).header.state: typeof(_291) = *mut {l386} DefId(2:5583 ~ core[480a]::ffi::c_void)
                            // 622: (*ldr).header.state:   l386 = UNIQUE | NON_NULL, (empty)
                            vars.specified,
                            clauses.specified,
                        );
                    }
                    clauses.parsed = 0 as libc::c_int;
                    vars.parsed = clauses.parsed;
                    lit = 0 as libc::c_int;
                    's_198: loop {
                        ch = ldrnext(ldr);
                        // 631: ldr: typeof(_298) = *mut {l394} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                        // 631: ldr:   l394 = UNIQUE | NON_NULL, (empty)
                        if ch == ' ' as i32
                            || ch == '\t' as i32
                            || ch == '\r' as i32
                            || ch == '\n' as i32
                        {
                            continue;
                        }
                        if ch == 'c' as i32 {
                            loop {
                                ch = ldrnext(ldr);
                                // 641: ldr: typeof(_320) = *mut {l417} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                // 641: ldr:   l417 = UNIQUE | NON_NULL, (empty)
                                if !(ch != '\n' as i32) {
                                    break;
                                }
                                if ch == -(1 as libc::c_int) {
                                    return ldrperr(
                                        ldr,
                                        // 647: ldr: typeof(_333) = *mut {l431} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                        // 647: ldr:   l431 = UNIQUE | NON_NULL, (empty)
                                        b"end-of-file in comment after header\0" as *const u8
                                        // 648: b"end-of-file i ... _char: typeof(_334) = *const {l433} i8
                                        // 648: b"end-of-file i ... _char:   l433 = UNIQUE | NON_NULL, (empty)
                                        // 648: b"end-of-file i ... st u8: typeof(_335) = *const {l435} u8
                                        // 648: b"end-of-file i ... st u8:   l435 = UNIQUE | NON_NULL, (empty)
                                        // 648: b"end-of-file i ... er\0": typeof(_336) = *const {l437} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                                        // 648: b"end-of-file i ... er\0":   l437 = UNIQUE | NON_NULL, (empty)
                                        // 648: b"end-of-file i ... er\0": typeof(_337) = & {l439} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                                        // 648: b"end-of-file i ... er\0":   l439 = UNIQUE | NON_NULL, FIXED
                                        // 648: b"end-of-file i ... _char: typeof(_334 = move _335 as *const i8 (Misc)) = *const {l722} i8
                                        // 648: b"end-of-file i ... _char:   l722 = UNIQUE | NON_NULL, (empty)
                                        // 648: b"end-of-file i ... st u8: typeof(_335 = move _336 as *const u8 (Pointer(ArrayToPointer))) = *const {l721} u8
                                        // 648: b"end-of-file i ... st u8:   l721 = UNIQUE | NON_NULL, (empty)
                                        // 648: b"end-of-file i ... er\0": typeof(_337 = const b"end-of-file in comment after header\x00") = & {l719} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                                        // 648: b"end-of-file i ... er\0":   l719 = UNIQUE | NON_NULL, (empty)
                                        // 648: b"end-of-file i ... er\0": typeof(_336 = &raw const (*_337)) = *const {l720} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000024)) }]
                                        // 648: b"end-of-file i ... er\0":   l720 = UNIQUE | NON_NULL, (empty)
                                            as *const libc::c_char,
                                    );
                                }
                            }
                        } else {
                            if ch == -(1 as libc::c_int) {
                                break '_NUMBER_TOO_LARGE;
                            }
                            if ch == '-' as i32 {
                                ch = ldrnext(ldr);
                                // 658: ldr: typeof(_350) = *mut {l453} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                // 658: ldr:   l453 = UNIQUE | NON_NULL, (empty)
                                if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                                // 659: (*__ctype_b_loc ... size): typeof(_356) = *const {l460} u16
                                // 659: (*__ctype_b_loc ... size):   l460 = UNIQUE | NON_NULL, (empty)
                                // 659: (*__ctype_b_loc()): typeof(_357) = *const {l462} u16
                                // 659: (*__ctype_b_loc()):   l462 = UNIQUE | NON_NULL, (empty)
                                // 659: __ctype_b_loc(): typeof(_358) = *mut {l464} *const {l465} u16
                                // 659: __ctype_b_loc():   l464 = UNIQUE | NON_NULL, (empty)
                                // 659: __ctype_b_loc():   l465 = UNIQUE | NON_NULL, (empty)
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    == 0
                                {
                                    return ldrperr(
                                        ldr,
                                        // 664: ldr: typeof(_365) = *mut {l473} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                        // 664: ldr:   l473 = UNIQUE | NON_NULL, (empty)
                                        b"expected digit after '-'\0" as *const u8
                                        // 665: b"expected digi ... _char: typeof(_366) = *const {l475} i8
                                        // 665: b"expected digi ... _char:   l475 = UNIQUE | NON_NULL, (empty)
                                        // 665: b"expected digi ... st u8: typeof(_367) = *const {l477} u8
                                        // 665: b"expected digi ... st u8:   l477 = UNIQUE | NON_NULL, (empty)
                                        // 665: b"expected digi ... -'\0": typeof(_368) = *const {l479} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                        // 665: b"expected digi ... -'\0":   l479 = UNIQUE | NON_NULL, (empty)
                                        // 665: b"expected digi ... -'\0": typeof(_369) = & {l481} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                        // 665: b"expected digi ... -'\0":   l481 = UNIQUE | NON_NULL, FIXED
                                        // 665: b"expected digi ... st u8: typeof(_367 = move _368 as *const u8 (Pointer(ArrayToPointer))) = *const {l725} u8
                                        // 665: b"expected digi ... st u8:   l725 = UNIQUE | NON_NULL, (empty)
                                        // 665: b"expected digi ... -'\0": typeof(_369 = const b"expected digit after \'-\'\x00") = & {l723} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                        // 665: b"expected digi ... -'\0":   l723 = UNIQUE | NON_NULL, (empty)
                                        // 665: b"expected digi ... -'\0": typeof(_368 = &raw const (*_369)) = *const {l724} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000019)) }]
                                        // 665: b"expected digi ... -'\0":   l724 = UNIQUE | NON_NULL, (empty)
                                        // 665: b"expected digi ... _char: typeof(_366 = move _367 as *const i8 (Misc)) = *const {l726} i8
                                        // 665: b"expected digi ... _char:   l726 = UNIQUE | NON_NULL, (empty)
                                            as *const libc::c_char,
                                    );
                                }
                                sign = -(1 as libc::c_int);
                            } else if *(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                            // 670: (*__ctype_b_loc ... size): typeof(_376) = *const {l489} u16
                            // 670: (*__ctype_b_loc ... size):   l489 = UNIQUE | NON_NULL, (empty)
                            // 670: (*__ctype_b_loc()): typeof(_377) = *const {l491} u16
                            // 670: (*__ctype_b_loc()):   l491 = UNIQUE | NON_NULL, (empty)
                            // 670: __ctype_b_loc(): typeof(_378) = *mut {l493} *const {l494} u16
                            // 670: __ctype_b_loc():   l493 = UNIQUE | NON_NULL, (empty)
                            // 670: __ctype_b_loc():   l494 = UNIQUE | NON_NULL, (empty)
                                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                == 0
                            {
                                return ldrperr(
                                    ldr,
                                    // 675: ldr: typeof(_385) = *mut {l502} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                    // 675: ldr:   l502 = UNIQUE | NON_NULL, (empty)
                                    b"expected digit or '-'\0" as *const u8 as *const libc::c_char,
                                    // 676: b"expected digi ... _char: typeof(_386) = *const {l504} i8
                                    // 676: b"expected digi ... _char:   l504 = UNIQUE | NON_NULL, (empty)
                                    // 676: b"expected digi ... st u8: typeof(_387) = *const {l506} u8
                                    // 676: b"expected digi ... st u8:   l506 = UNIQUE | NON_NULL, (empty)
                                    // 676: b"expected digi ... -'\0": typeof(_388) = *const {l508} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 676: b"expected digi ... -'\0":   l508 = UNIQUE | NON_NULL, (empty)
                                    // 676: b"expected digi ... -'\0": typeof(_389) = & {l510} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 676: b"expected digi ... -'\0":   l510 = UNIQUE | NON_NULL, FIXED
                                    // 676: b"expected digi ... -'\0": typeof(_389 = const b"expected digit or \'-\'\x00") = & {l727} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 676: b"expected digi ... -'\0":   l727 = UNIQUE | NON_NULL, (empty)
                                    // 676: b"expected digi ... -'\0": typeof(_388 = &raw const (*_389)) = *const {l728} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
                                    // 676: b"expected digi ... -'\0":   l728 = UNIQUE | NON_NULL, (empty)
                                    // 676: b"expected digi ... st u8: typeof(_387 = move _388 as *const u8 (Pointer(ArrayToPointer))) = *const {l729} u8
                                    // 676: b"expected digi ... st u8:   l729 = UNIQUE | NON_NULL, (empty)
                                    // 676: b"expected digi ... _char: typeof(_386 = move _387 as *const i8 (Misc)) = *const {l730} i8
                                    // 676: b"expected digi ... _char:   l730 = UNIQUE | NON_NULL, (empty)
                                );
                            } else {
                                sign = 1 as libc::c_int;
                            }
                            if clauses.specified == clauses.parsed {
                                return ldrperr(
                                    ldr,
                                    // 683: ldr: typeof(_396) = *mut {l518} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                    // 683: ldr:   l518 = UNIQUE | NON_NULL, (empty)
                                    b"too many clauses\0" as *const u8 as *const libc::c_char,
                                    // 684: b"too many clau ... _char: typeof(_397) = *const {l520} i8
                                    // 684: b"too many clau ... _char:   l520 = UNIQUE | NON_NULL, (empty)
                                    // 684: b"too many clau ... st u8: typeof(_398) = *const {l522} u8
                                    // 684: b"too many clau ... st u8:   l522 = UNIQUE | NON_NULL, (empty)
                                    // 684: b"too many clau ... es\0": typeof(_399) = *const {l524} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 684: b"too many clau ... es\0":   l524 = UNIQUE | NON_NULL, (empty)
                                    // 684: b"too many clau ... es\0": typeof(_400) = & {l526} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 684: b"too many clau ... es\0":   l526 = UNIQUE | NON_NULL, FIXED
                                    // 684: b"too many clau ... st u8: typeof(_398 = move _399 as *const u8 (Pointer(ArrayToPointer))) = *const {l733} u8
                                    // 684: b"too many clau ... st u8:   l733 = UNIQUE | NON_NULL, (empty)
                                    // 684: b"too many clau ... es\0": typeof(_400 = const b"too many clauses\x00") = & {l731} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 684: b"too many clau ... es\0":   l731 = UNIQUE | NON_NULL, (empty)
                                    // 684: b"too many clau ... es\0": typeof(_399 = &raw const (*_400)) = *const {l732} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
                                    // 684: b"too many clau ... es\0":   l732 = UNIQUE | NON_NULL, (empty)
                                    // 684: b"too many clau ... _char: typeof(_397 = move _398 as *const i8 (Misc)) = *const {l734} i8
                                    // 684: b"too many clau ... _char:   l734 = UNIQUE | NON_NULL, (empty)
                                );
                            }
                            lit = ch - '0' as i32;
                            loop {
                                ch = ldrnext(ldr);
                                // 689: ldr: typeof(_406) = *mut {l533} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                // 689: ldr:   l533 = UNIQUE | NON_NULL, (empty)
                                if !(*(*__ctype_b_loc()).offset(ch as isize) as libc::c_int
                                // 690: (*__ctype_b_loc ... size): typeof(_413) = *const {l541} u16
                                // 690: (*__ctype_b_loc ... size):   l541 = UNIQUE | NON_NULL, (empty)
                                // 690: (*__ctype_b_loc()): typeof(_414) = *const {l543} u16
                                // 690: (*__ctype_b_loc()):   l543 = UNIQUE | NON_NULL, (empty)
                                // 690: __ctype_b_loc(): typeof(_415) = *mut {l545} *const {l546} u16
                                // 690: __ctype_b_loc():   l545 = UNIQUE | NON_NULL, (empty)
                                // 690: __ctype_b_loc():   l546 = UNIQUE | NON_NULL, (empty)
                                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                                    != 0)
                                {
                                    break;
                                }
                                if (2147483647 as libc::c_int / 10 as libc::c_int) < lit {
                                    current_block = 14777158775944759451;
                                    break 's_198;
                                }
                                lit *= 10 as libc::c_int;
                                digit = ch - '0' as i32;
                                if 2147483647 as libc::c_int - digit < lit {
                                    current_block = 14777158775944759451;
                                    break 's_198;
                                }
                                lit += digit;
                            }
                            if lit > vars.specified {
                                return ldrperr(
                                    ldr,
                                    // 710: ldr: typeof(_453) = *mut {l585} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
                                    // 710: ldr:   l585 = UNIQUE | NON_NULL, (empty)
                                    b"maximum variable index exceeded\0" as *const u8
                                    // 711: b"maximum varia ... _char: typeof(_454) = *const {l587} i8
                                    // 711: b"maximum varia ... _char:   l587 = UNIQUE | NON_NULL, (empty)
                                    // 711: b"maximum varia ... st u8: typeof(_455) = *const {l589} u8
                                    // 711: b"maximum varia ... st u8:   l589 = UNIQUE | NON_NULL, (empty)
                                    // 711: b"maximum varia ... ed\0": typeof(_456) = *const {l591} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                    // 711: b"maximum varia ... ed\0":   l591 = UNIQUE | NON_NULL, (empty)
                                    // 711: b"maximum varia ... ed\0": typeof(_457) = & {l593} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                    // 711: b"maximum varia ... ed\0":   l593 = UNIQUE | NON_NULL, FIXED
                                    // 711: b"maximum varia ... st u8: typeof(_455 = move _456 as *const u8 (Pointer(ArrayToPointer))) = *const {l737} u8
                                    // 711: b"maximum varia ... st u8:   l737 = UNIQUE | NON_NULL, (empty)
                                    // 711: b"maximum varia ... _char: typeof(_454 = move _455 as *const i8 (Misc)) = *const {l738} i8
                                    // 711: b"maximum varia ... _char:   l738 = UNIQUE | NON_NULL, (empty)
                                    // 711: b"maximum varia ... ed\0": typeof(_457 = const b"maximum variable index exceeded\x00") = & {l735} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                    // 711: b"maximum varia ... ed\0":   l735 = UNIQUE | NON_NULL, (empty)
                                    // 711: b"maximum varia ... ed\0": typeof(_456 = &raw const (*_457)) = *const {l736} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000020)) }]
                                    // 711: b"maximum varia ... ed\0":   l736 = UNIQUE | NON_NULL, (empty)
                                        as *const libc::c_char,
                                );
                            }
                            lit *= sign;
                            if ((*ldr).add.fun).is_some() {
                            // 716: ((*ldr).add.fun ... ome(): typeof(_462) = & {l599} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l600} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
                            // 716: ((*ldr).add.fun ... ome():   l599 = UNIQUE | NON_NULL, (empty)
                            // 716: ((*ldr).add.fun ... ome():   l600 = UNIQUE | NON_NULL, (empty)
                            // 716: ((*ldr).add.fun ... ome(): typeof(_462 = &(((*_1).3: lgldimacs::C2RustUnnamed).1: std::option::Option<unsafe extern "C" fn(*mut libc::c_void, i32)>)) = & {l739} DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l740} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
                            // 716: ((*ldr).add.fun ... ome():   l739 = UNIQUE | NON_NULL, (empty)
                            // 716: ((*ldr).add.fun ... ome():   l740 = UNIQUE | NON_NULL, (empty)
                                ((*ldr).add.fun).expect("non-null function pointer")(
                                // 717: ((*ldr).add.fun ... ter"): typeof(_464) = fn(*mut {l603} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()
                                // 717: ((*ldr).add.fun ... ter"):   l603 = UNIQUE | NON_NULL, (empty)
                                // 717: ((*ldr).add.fun): typeof(_465) = DefId(2:47492 ~ core[480a]::option::Option)<fn(*mut {l605} DefId(2:5583 ~ core[480a]::ffi::c_void), i32) -> ()>
                                // 717: ((*ldr).add.fun):   l605 = UNIQUE | NON_NULL, (empty)
                                // 717: "non-null funct ... nter": typeof(_466) = & {l607} str
                                // 717: "non-null funct ... nter":   l607 = UNIQUE | NON_NULL, (empty)
                                // 717: "non-null funct ... nter": typeof(_467) = & {l609} str
                                // 717: "non-null funct ... nter":   l609 = UNIQUE | NON_NULL, FIXED
                                // 717: "non-null funct ... nter": typeof(_467 = const "non-null function pointer") = & {l741} str
                                // 717: "non-null funct ... nter":   l741 = UNIQUE | NON_NULL, (empty)
                                // 717: "non-null funct ... nter": typeof(_466 = &(*_467)) = & {l742} str
                                // 717: "non-null funct ... nter":   l742 = UNIQUE | NON_NULL, (empty)
                                    (*ldr).add.state,
                                    // 718: (*ldr).add.state: typeof(_468) = *mut {l611} DefId(2:5583 ~ core[480a]::ffi::c_void)
                                    // 718: (*ldr).add.state:   l611 = UNIQUE | NON_NULL, (empty)
                                    lit,
                                );
                            }
                            if lit != 0 {
                                continue;
                            }
                            clauses.parsed += 1;
                            clauses.parsed;
                        }
                    }
                }
            }
        }
    }
    if lit != 0 {
        return ldrperr(
            ldr,
            // 735: ldr: typeof(_480) = *mut {l624} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 735: ldr:   l624 = UNIQUE | NON_NULL, (empty)
            b"zero sentinel missing at end-of-file\0" as *const u8 as *const libc::c_char,
            // 736: b"zero sentinel ... _char: typeof(_481) = *const {l626} i8
            // 736: b"zero sentinel ... _char:   l626 = UNIQUE | NON_NULL, (empty)
            // 736: b"zero sentinel ... st u8: typeof(_482) = *const {l628} u8
            // 736: b"zero sentinel ... st u8:   l628 = UNIQUE | NON_NULL, (empty)
            // 736: b"zero sentinel ... le\0": typeof(_483) = *const {l630} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 736: b"zero sentinel ... le\0":   l630 = UNIQUE | NON_NULL, (empty)
            // 736: b"zero sentinel ... le\0": typeof(_484) = & {l632} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 736: b"zero sentinel ... le\0":   l632 = UNIQUE | NON_NULL, FIXED
            // 736: b"zero sentinel ... le\0": typeof(_483 = &raw const (*_484)) = *const {l744} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 736: b"zero sentinel ... le\0":   l744 = UNIQUE | NON_NULL, (empty)
            // 736: b"zero sentinel ... st u8: typeof(_482 = move _483 as *const u8 (Pointer(ArrayToPointer))) = *const {l745} u8
            // 736: b"zero sentinel ... st u8:   l745 = UNIQUE | NON_NULL, (empty)
            // 736: b"zero sentinel ... _char: typeof(_481 = move _482 as *const i8 (Misc)) = *const {l746} i8
            // 736: b"zero sentinel ... _char:   l746 = UNIQUE | NON_NULL, (empty)
            // 736: b"zero sentinel ... le\0": typeof(_484 = const b"zero sentinel missing at end-of-file\x00") = & {l743} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
            // 736: b"zero sentinel ... le\0":   l743 = UNIQUE | NON_NULL, (empty)
        );
    }
    if clauses.parsed + 1 as libc::c_int == clauses.specified {
        return ldrperr(
            ldr,
            // 741: ldr: typeof(_493) = *mut {l642} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 741: ldr:   l642 = UNIQUE | NON_NULL, (empty)
            b"one clause is missing\0" as *const u8 as *const libc::c_char,
            // 742: b"one clause is ... _char: typeof(_494) = *const {l644} i8
            // 742: b"one clause is ... _char:   l644 = UNIQUE | NON_NULL, (empty)
            // 742: b"one clause is ... st u8: typeof(_495) = *const {l646} u8
            // 742: b"one clause is ... st u8:   l646 = UNIQUE | NON_NULL, (empty)
            // 742: b"one clause is ... ng\0": typeof(_496) = *const {l648} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 742: b"one clause is ... ng\0":   l648 = UNIQUE | NON_NULL, (empty)
            // 742: b"one clause is ... ng\0": typeof(_497) = & {l650} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 742: b"one clause is ... ng\0":   l650 = UNIQUE | NON_NULL, FIXED
            // 742: b"one clause is ... st u8: typeof(_495 = move _496 as *const u8 (Pointer(ArrayToPointer))) = *const {l749} u8
            // 742: b"one clause is ... st u8:   l749 = UNIQUE | NON_NULL, (empty)
            // 742: b"one clause is ... ng\0": typeof(_497 = const b"one clause is missing\x00") = & {l747} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 742: b"one clause is ... ng\0":   l747 = UNIQUE | NON_NULL, (empty)
            // 742: b"one clause is ... ng\0": typeof(_496 = &raw const (*_497)) = *const {l748} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000016)) }]
            // 742: b"one clause is ... ng\0":   l748 = UNIQUE | NON_NULL, (empty)
            // 742: b"one clause is ... _char: typeof(_494 = move _495 as *const i8 (Misc)) = *const {l750} i8
            // 742: b"one clause is ... _char:   l750 = UNIQUE | NON_NULL, (empty)
        );
    }
    if clauses.parsed < clauses.specified {
        return ldrperr(
            ldr,
            // 747: ldr: typeof(_503) = *mut {l657} DefId(0:1768 ~ libgeling[8679]::lgldimacs::LDR)
            // 747: ldr:   l657 = UNIQUE | NON_NULL, (empty)
            b"clauses are missing\0" as *const u8 as *const libc::c_char,
            // 748: b"clauses are m ... _char: typeof(_504) = *const {l659} i8
            // 748: b"clauses are m ... _char:   l659 = UNIQUE | NON_NULL, (empty)
            // 748: b"clauses are m ... st u8: typeof(_505) = *const {l661} u8
            // 748: b"clauses are m ... st u8:   l661 = UNIQUE | NON_NULL, (empty)
            // 748: b"clauses are m ... ng\0": typeof(_506) = *const {l663} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 748: b"clauses are m ... ng\0":   l663 = UNIQUE | NON_NULL, (empty)
            // 748: b"clauses are m ... ng\0": typeof(_507) = & {l665} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 748: b"clauses are m ... ng\0":   l665 = UNIQUE | NON_NULL, FIXED
            // 748: b"clauses are m ... ng\0": typeof(_507 = const b"clauses are missing\x00") = & {l751} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 748: b"clauses are m ... ng\0":   l751 = UNIQUE | NON_NULL, (empty)
            // 748: b"clauses are m ... st u8: typeof(_505 = move _506 as *const u8 (Pointer(ArrayToPointer))) = *const {l753} u8
            // 748: b"clauses are m ... st u8:   l753 = UNIQUE | NON_NULL, (empty)
            // 748: b"clauses are m ... ng\0": typeof(_506 = &raw const (*_507)) = *const {l752} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000014)) }]
            // 748: b"clauses are m ... ng\0":   l752 = UNIQUE | NON_NULL, (empty)
            // 748: b"clauses are m ... _char: typeof(_504 = move _505 as *const i8 (Misc)) = *const {l754} i8
            // 748: b"clauses are m ... _char:   l754 = UNIQUE | NON_NULL, (empty)
        );
    }
    return 1 as libc::c_int;
}
