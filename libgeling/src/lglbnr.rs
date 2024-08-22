#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    // 28: _IO_read_ptr: typeof(_IO_read_ptr) = *mut {g1462} i8
    // 28: _IO_read_ptr:   g1462 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_end: *mut libc::c_char,
    // 29: _IO_read_end: typeof(_IO_read_end) = *mut {g1463} i8
    // 29: _IO_read_end:   g1463 = UNIQUE | NON_NULL, FIXED
    pub _IO_read_base: *mut libc::c_char,
    // 30: _IO_read_base: typeof(_IO_read_base) = *mut {g1464} i8
    // 30: _IO_read_base:   g1464 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_base: *mut libc::c_char,
    // 31: _IO_write_base: typeof(_IO_write_base) = *mut {g1465} i8
    // 31: _IO_write_base:   g1465 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_ptr: *mut libc::c_char,
    // 32: _IO_write_ptr: typeof(_IO_write_ptr) = *mut {g1466} i8
    // 32: _IO_write_ptr:   g1466 = UNIQUE | NON_NULL, FIXED
    pub _IO_write_end: *mut libc::c_char,
    // 33: _IO_write_end: typeof(_IO_write_end) = *mut {g1467} i8
    // 33: _IO_write_end:   g1467 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_base: *mut libc::c_char,
    // 34: _IO_buf_base: typeof(_IO_buf_base) = *mut {g1468} i8
    // 34: _IO_buf_base:   g1468 = UNIQUE | NON_NULL, FIXED
    pub _IO_buf_end: *mut libc::c_char,
    // 35: _IO_buf_end: typeof(_IO_buf_end) = *mut {g1469} i8
    // 35: _IO_buf_end:   g1469 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_base: *mut libc::c_char,
    // 36: _IO_save_base: typeof(_IO_save_base) = *mut {g1470} i8
    // 36: _IO_save_base:   g1470 = UNIQUE | NON_NULL, FIXED
    pub _IO_backup_base: *mut libc::c_char,
    // 37: _IO_backup_base: typeof(_IO_backup_base) = *mut {g1471} i8
    // 37: _IO_backup_base:   g1471 = UNIQUE | NON_NULL, FIXED
    pub _IO_save_end: *mut libc::c_char,
    // 38: _IO_save_end: typeof(_IO_save_end) = *mut {g1472} i8
    // 38: _IO_save_end:   g1472 = UNIQUE | NON_NULL, FIXED
    pub _markers: *mut _IO_marker,
    // 39: _markers: typeof(_markers) = *mut {g1473} lglbnr::_IO_marker
    // 39: _markers:   g1473 = UNIQUE | NON_NULL, FIXED
    pub _chain: *mut _IO_FILE,
    // 40: _chain: typeof(_chain) = *mut {g1474} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 40: _chain:   g1474 = UNIQUE | NON_NULL, FIXED
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    // 47: _lock: typeof(_lock) = *mut {g1475} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 47: _lock:   g1475 = UNIQUE | NON_NULL, FIXED
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    // 49: _codecvt: typeof(_codecvt) = *mut {g1476} lglbnr::_IO_codecvt
    // 49: _codecvt:   g1476 = UNIQUE | NON_NULL, FIXED
    pub _wide_data: *mut _IO_wide_data,
    // 50: _wide_data: typeof(_wide_data) = *mut {g1477} lglbnr::_IO_wide_data
    // 50: _wide_data:   g1477 = UNIQUE | NON_NULL, FIXED
    pub _freeres_list: *mut _IO_FILE,
    // 51: _freeres_list: typeof(_freeres_list) = *mut {g1478} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 51: _freeres_list:   g1478 = UNIQUE | NON_NULL, FIXED
    pub _freeres_buf: *mut libc::c_void,
    // 52: _freeres_buf: typeof(_freeres_buf) = *mut {g1479} DefId(2:5583 ~ core[480a]::ffi::c_void)
    // 52: _freeres_buf:   g1479 = UNIQUE | NON_NULL, FIXED
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn lglbnr(
    mut name: *const libc::c_char,
    // 61: mut name: typeof(_1) = *const {g0} i8
    // 61: mut name:   g0 = UNIQUE | NON_NULL, FIXED
    mut prefix: *const libc::c_char,
    // 62: mut prefix: typeof(_2) = *const {g1} i8
    // 62: mut prefix:   g1 = UNIQUE | NON_NULL, FIXED
    mut file: *mut FILE,
    // 63: mut file: typeof(_3) = *mut {g2} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 63: mut file:   g2 = UNIQUE | NON_NULL, FIXED
) {
    let mut p: *const libc::c_char = b"-W -Wall -O3 -DNLGLOG -DNDEBUG -DNCHKSOL -DNLGLDRUPLIG -DNLGLYALSAT -DNLGLFILES -DNLGLDEMA\0"
    // 65: mut p: typeof(_4) = *const {l4} i8
    // 65: mut p:   l4 = UNIQUE | NON_NULL, (empty)
    // 65: b"-W -Wall -O3  ... st u8: typeof(_5) = *const {l6} u8
    // 65: b"-W -Wall -O3  ... st u8:   l6 = UNIQUE | NON_NULL, (empty)
    // 65: b"-W -Wall -O3  ... MA\0": typeof(_6) = *const {l8} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000005b)) }]
    // 65: b"-W -Wall -O3  ... MA\0":   l8 = UNIQUE | NON_NULL, (empty)
    // 65: b"-W -Wall -O3  ... MA\0": typeof(_7) = & {l10} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000005b)) }]
    // 65: b"-W -Wall -O3  ... MA\0":   l10 = UNIQUE | NON_NULL, FIXED
    // 65: b"-W -Wall -O3  ... MA\0": typeof(_6 = &raw const (*_7)) = *const {l361} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000005b)) }]
    // 65: b"-W -Wall -O3  ... MA\0":   l361 = UNIQUE | NON_NULL, (empty)
    // 65: b"-W -Wall -O3  ... _char: typeof(_4 = move _5 as *const i8 (Misc)) = *const {l363} i8
    // 65: b"-W -Wall -O3  ... _char:   l363 = UNIQUE | NON_NULL, (empty)
    // 65: b"-W -Wall -O3  ... st u8: typeof(_5 = move _6 as *const u8 (Pointer(ArrayToPointer))) = *const {l362} u8
    // 65: b"-W -Wall -O3  ... st u8:   l362 = UNIQUE | NON_NULL, (empty)
    // 65: b"-W -Wall -O3  ... MA\0": typeof(_7 = const b"-W -Wall -O3 -DNLGLOG -DNDEBUG -DNCHKSOL -DNLGLDRUPLIG -DNLGLYALSAT -DNLGLFILES -DNLGLDEMA\x00") = & {l360} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000005b)) }]
    // 65: b"-W -Wall -O3  ... MA\0":   l360 = UNIQUE | NON_NULL, (empty)
        as *const u8 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    // 67: mut q: typeof(_8) = *const {l12} i8
    // 67: mut q:   l12 = UNIQUE | NON_NULL, (empty)
    // 67: 0 as *const lib ... _char: typeof(_8 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l364} i8
    // 67: 0 as *const lib ... _char:   l364 = UNIQUE | NON_NULL, (empty)
    let mut n: *const libc::c_char = 0 as *const libc::c_char;
    // 68: mut n: typeof(_9) = *const {l14} i8
    // 68: mut n:   l14 = UNIQUE | NON_NULL, (empty)
    // 68: 0 as *const lib ... _char: typeof(_9 = const 0_usize as *const i8 (PointerFromExposedAddress)) = *const {l365} i8
    // 68: 0 as *const lib ... _char:   l365 = UNIQUE | NON_NULL, (empty)
    let mut len: libc::c_int =
        (78 as libc::c_int as libc::c_ulong).wrapping_sub(strlen(prefix)) as libc::c_int;
        // 70: prefix: typeof(_15) = *const {l21} i8
        // 70: prefix:   l21 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 72: file: typeof(_17) = *mut {l24} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 72: file:   l24 = UNIQUE | NON_NULL, (empty)
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        // 73: b"%s%s\n\0" as  ... _char: typeof(_18) = *const {l26} i8
        // 73: b"%s%s\n\0" as  ... _char:   l26 = UNIQUE | NON_NULL, (empty)
        // 73: b"%s%s\n\0" as  ... st u8: typeof(_19) = *const {l28} u8
        // 73: b"%s%s\n\0" as  ... st u8:   l28 = UNIQUE | NON_NULL, (empty)
        // 73: b"%s%s\n\0": typeof(_20) = *const {l30} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 73: b"%s%s\n\0":   l30 = UNIQUE | NON_NULL, (empty)
        // 73: b"%s%s\n\0": typeof(_21) = & {l32} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 73: b"%s%s\n\0":   l32 = UNIQUE | NON_NULL, FIXED
        // 73: b"%s%s\n\0": typeof(_20 = &raw const (*_21)) = *const {l367} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 73: b"%s%s\n\0":   l367 = UNIQUE | NON_NULL, (empty)
        // 73: b"%s%s\n\0" as  ... _char: typeof(_18 = move _19 as *const i8 (Misc)) = *const {l369} i8
        // 73: b"%s%s\n\0" as  ... _char:   l369 = UNIQUE | NON_NULL, (empty)
        // 73: b"%s%s\n\0" as  ... st u8: typeof(_19 = move _20 as *const u8 (Pointer(ArrayToPointer))) = *const {l368} u8
        // 73: b"%s%s\n\0" as  ... st u8:   l368 = UNIQUE | NON_NULL, (empty)
        // 73: b"%s%s\n\0": typeof(_21 = const b"%s%s\n\x00") = & {l366} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 73: b"%s%s\n\0":   l366 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 74: prefix: typeof(_22) = *const {l34} i8
        // 74: prefix:   l34 = UNIQUE | NON_NULL, (empty)
        name,
        // 75: name: typeof(_23) = *const {l36} i8
        // 75: name:   l36 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    // 77: file: typeof(_25) = *mut {l39} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 77: file:   l39 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0" as *c ... _char: typeof(_26) = *const {l41} i8
    // 77: b"%s\n\0" as *c ... _char:   l41 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0" as *c ... st u8: typeof(_27) = *const {l43} u8
    // 77: b"%s\n\0" as *c ... st u8:   l43 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0": typeof(_28) = *const {l45} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 77: b"%s\n\0":   l45 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0": typeof(_29) = & {l47} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 77: b"%s\n\0":   l47 = UNIQUE | NON_NULL, FIXED
    // 77: prefix: typeof(_30) = *const {l49} i8
    // 77: prefix:   l49 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0": typeof(_28 = &raw const (*_29)) = *const {l371} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 77: b"%s\n\0":   l371 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0" as *c ... st u8: typeof(_27 = move _28 as *const u8 (Pointer(ArrayToPointer))) = *const {l372} u8
    // 77: b"%s\n\0" as *c ... st u8:   l372 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0": typeof(_29 = const b"%s\n\x00") = & {l370} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 77: b"%s\n\0":   l370 = UNIQUE | NON_NULL, (empty)
    // 77: b"%s\n\0" as *c ... _char: typeof(_26 = move _27 as *const i8 (Misc)) = *const {l373} i8
    // 77: b"%s\n\0" as *c ... _char:   l373 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 79: file: typeof(_32) = *mut {l52} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 79: file:   l52 = UNIQUE | NON_NULL, (empty)
        b"%sVersion %s %s\n\0" as *const u8 as *const libc::c_char,
        // 80: b"%sVersion %s  ... _char: typeof(_33) = *const {l54} i8
        // 80: b"%sVersion %s  ... _char:   l54 = UNIQUE | NON_NULL, (empty)
        // 80: b"%sVersion %s  ... st u8: typeof(_34) = *const {l56} u8
        // 80: b"%sVersion %s  ... st u8:   l56 = UNIQUE | NON_NULL, (empty)
        // 80: b"%sVersion %s  ... \n\0": typeof(_35) = *const {l58} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 80: b"%sVersion %s  ... \n\0":   l58 = UNIQUE | NON_NULL, (empty)
        // 80: b"%sVersion %s  ... \n\0": typeof(_36) = & {l60} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 80: b"%sVersion %s  ... \n\0":   l60 = UNIQUE | NON_NULL, FIXED
        // 80: b"%sVersion %s  ... \n\0": typeof(_36 = const b"%sVersion %s %s\n\x00") = & {l374} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 80: b"%sVersion %s  ... \n\0":   l374 = UNIQUE | NON_NULL, (empty)
        // 80: b"%sVersion %s  ... \n\0": typeof(_35 = &raw const (*_36)) = *const {l375} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000011)) }]
        // 80: b"%sVersion %s  ... \n\0":   l375 = UNIQUE | NON_NULL, (empty)
        // 80: b"%sVersion %s  ... _char: typeof(_33 = move _34 as *const i8 (Misc)) = *const {l377} i8
        // 80: b"%sVersion %s  ... _char:   l377 = UNIQUE | NON_NULL, (empty)
        // 80: b"%sVersion %s  ... st u8: typeof(_34 = move _35 as *const u8 (Pointer(ArrayToPointer))) = *const {l376} u8
        // 80: b"%sVersion %s  ... st u8:   l376 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 81: prefix: typeof(_37) = *const {l62} i8
        // 81: prefix:   l62 = UNIQUE | NON_NULL, (empty)
        b"1.0.0\0" as *const u8 as *const libc::c_char,
        // 82: b"1.0.0\0" as * ... _char: typeof(_38) = *const {l64} i8
        // 82: b"1.0.0\0" as * ... _char:   l64 = UNIQUE | NON_NULL, (empty)
        // 82: b"1.0.0\0" as * ... st u8: typeof(_39) = *const {l66} u8
        // 82: b"1.0.0\0" as * ... st u8:   l66 = UNIQUE | NON_NULL, (empty)
        // 82: b"1.0.0\0": typeof(_40) = *const {l68} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 82: b"1.0.0\0":   l68 = UNIQUE | NON_NULL, (empty)
        // 82: b"1.0.0\0": typeof(_41) = & {l70} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 82: b"1.0.0\0":   l70 = UNIQUE | NON_NULL, FIXED
        // 82: b"1.0.0\0" as * ... st u8: typeof(_39 = move _40 as *const u8 (Pointer(ArrayToPointer))) = *const {l380} u8
        // 82: b"1.0.0\0" as * ... st u8:   l380 = UNIQUE | NON_NULL, (empty)
        // 82: b"1.0.0\0" as * ... _char: typeof(_38 = move _39 as *const i8 (Misc)) = *const {l381} i8
        // 82: b"1.0.0\0" as * ... _char:   l381 = UNIQUE | NON_NULL, (empty)
        // 82: b"1.0.0\0": typeof(_41 = const b"1.0.0\x00") = & {l378} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 82: b"1.0.0\0":   l378 = UNIQUE | NON_NULL, (empty)
        // 82: b"1.0.0\0": typeof(_40 = &raw const (*_41)) = *const {l379} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 82: b"1.0.0\0":   l379 = UNIQUE | NON_NULL, (empty)
        b"89a167d0d2efe98d983c87b5b84175b40ea55842\0" as *const u8 as *const libc::c_char,
        // 83: b"89a167d0d2efe ... _char: typeof(_42) = *const {l72} i8
        // 83: b"89a167d0d2efe ... _char:   l72 = UNIQUE | NON_NULL, (empty)
        // 83: b"89a167d0d2efe ... st u8: typeof(_43) = *const {l74} u8
        // 83: b"89a167d0d2efe ... st u8:   l74 = UNIQUE | NON_NULL, (empty)
        // 83: b"89a167d0d2efe ... 42\0": typeof(_44) = *const {l76} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 83: b"89a167d0d2efe ... 42\0":   l76 = UNIQUE | NON_NULL, (empty)
        // 83: b"89a167d0d2efe ... 42\0": typeof(_45) = & {l78} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 83: b"89a167d0d2efe ... 42\0":   l78 = UNIQUE | NON_NULL, FIXED
        // 83: b"89a167d0d2efe ... _char: typeof(_42 = move _43 as *const i8 (Misc)) = *const {l385} i8
        // 83: b"89a167d0d2efe ... _char:   l385 = UNIQUE | NON_NULL, (empty)
        // 83: b"89a167d0d2efe ... st u8: typeof(_43 = move _44 as *const u8 (Pointer(ArrayToPointer))) = *const {l384} u8
        // 83: b"89a167d0d2efe ... st u8:   l384 = UNIQUE | NON_NULL, (empty)
        // 83: b"89a167d0d2efe ... 42\0": typeof(_44 = &raw const (*_45)) = *const {l383} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 83: b"89a167d0d2efe ... 42\0":   l383 = UNIQUE | NON_NULL, (empty)
        // 83: b"89a167d0d2efe ... 42\0": typeof(_45 = const b"89a167d0d2efe98d983c87b5b84175b40ea55842\x00") = & {l382} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000029)) }]
        // 83: b"89a167d0d2efe ... 42\0":   l382 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    // 85: file: typeof(_47) = *mut {l81} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 85: file:   l81 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0" as *c ... _char: typeof(_48) = *const {l83} i8
    // 85: b"%s\n\0" as *c ... _char:   l83 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0" as *c ... st u8: typeof(_49) = *const {l85} u8
    // 85: b"%s\n\0" as *c ... st u8:   l85 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0": typeof(_50) = *const {l87} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 85: b"%s\n\0":   l87 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0": typeof(_51) = & {l89} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 85: b"%s\n\0":   l89 = UNIQUE | NON_NULL, FIXED
    // 85: prefix: typeof(_52) = *const {l91} i8
    // 85: prefix:   l91 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0": typeof(_51 = const b"%s\n\x00") = & {l386} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 85: b"%s\n\0":   l386 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0": typeof(_50 = &raw const (*_51)) = *const {l387} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 85: b"%s\n\0":   l387 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0" as *c ... st u8: typeof(_49 = move _50 as *const u8 (Pointer(ArrayToPointer))) = *const {l388} u8
    // 85: b"%s\n\0" as *c ... st u8:   l388 = UNIQUE | NON_NULL, (empty)
    // 85: b"%s\n\0" as *c ... _char: typeof(_48 = move _49 as *const i8 (Misc)) = *const {l389} i8
    // 85: b"%s\n\0" as *c ... _char:   l389 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 87: file: typeof(_54) = *mut {l94} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 87: file:   l94 = UNIQUE | NON_NULL, (empty)
        b"%sCopyright (C) 2010-2016 Armin Biere JKU Linz Austria.\n\0" as *const u8
        // 88: b"%sCopyright ( ... _char: typeof(_55) = *const {l96} i8
        // 88: b"%sCopyright ( ... _char:   l96 = UNIQUE | NON_NULL, (empty)
        // 88: b"%sCopyright ( ... st u8: typeof(_56) = *const {l98} u8
        // 88: b"%sCopyright ( ... st u8:   l98 = UNIQUE | NON_NULL, (empty)
        // 88: b"%sCopyright ( ... \n\0": typeof(_57) = *const {l100} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 88: b"%sCopyright ( ... \n\0":   l100 = UNIQUE | NON_NULL, (empty)
        // 88: b"%sCopyright ( ... \n\0": typeof(_58) = & {l102} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 88: b"%sCopyright ( ... \n\0":   l102 = UNIQUE | NON_NULL, FIXED
        // 88: b"%sCopyright ( ... \n\0": typeof(_58 = const b"%sCopyright (C) 2010-2016 Armin Biere JKU Linz Austria.\n\x00") = & {l390} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 88: b"%sCopyright ( ... \n\0":   l390 = UNIQUE | NON_NULL, (empty)
        // 88: b"%sCopyright ( ... st u8: typeof(_56 = move _57 as *const u8 (Pointer(ArrayToPointer))) = *const {l392} u8
        // 88: b"%sCopyright ( ... st u8:   l392 = UNIQUE | NON_NULL, (empty)
        // 88: b"%sCopyright ( ... _char: typeof(_55 = move _56 as *const i8 (Misc)) = *const {l393} i8
        // 88: b"%sCopyright ( ... _char:   l393 = UNIQUE | NON_NULL, (empty)
        // 88: b"%sCopyright ( ... \n\0": typeof(_57 = &raw const (*_58)) = *const {l391} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000039)) }]
        // 88: b"%sCopyright ( ... \n\0":   l391 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
        prefix,
        // 90: prefix: typeof(_59) = *const {l104} i8
        // 90: prefix:   l104 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(
        file,
        // 93: file: typeof(_61) = *mut {l107} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 93: file:   l107 = UNIQUE | NON_NULL, (empty)
        b"%sAll rights reserved.\n\0" as *const u8 as *const libc::c_char,
        // 94: b"%sAll rights  ... _char: typeof(_62) = *const {l109} i8
        // 94: b"%sAll rights  ... _char:   l109 = UNIQUE | NON_NULL, (empty)
        // 94: b"%sAll rights  ... st u8: typeof(_63) = *const {l111} u8
        // 94: b"%sAll rights  ... st u8:   l111 = UNIQUE | NON_NULL, (empty)
        // 94: b"%sAll rights  ... \n\0": typeof(_64) = *const {l113} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 94: b"%sAll rights  ... \n\0":   l113 = UNIQUE | NON_NULL, (empty)
        // 94: b"%sAll rights  ... \n\0": typeof(_65) = & {l115} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 94: b"%sAll rights  ... \n\0":   l115 = UNIQUE | NON_NULL, FIXED
        // 94: b"%sAll rights  ... st u8: typeof(_63 = move _64 as *const u8 (Pointer(ArrayToPointer))) = *const {l396} u8
        // 94: b"%sAll rights  ... st u8:   l396 = UNIQUE | NON_NULL, (empty)
        // 94: b"%sAll rights  ... \n\0": typeof(_64 = &raw const (*_65)) = *const {l395} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 94: b"%sAll rights  ... \n\0":   l395 = UNIQUE | NON_NULL, (empty)
        // 94: b"%sAll rights  ... _char: typeof(_62 = move _63 as *const i8 (Misc)) = *const {l397} i8
        // 94: b"%sAll rights  ... _char:   l397 = UNIQUE | NON_NULL, (empty)
        // 94: b"%sAll rights  ... \n\0": typeof(_65 = const b"%sAll rights reserved.\n\x00") = & {l394} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000018)) }]
        // 94: b"%sAll rights  ... \n\0":   l394 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 95: prefix: typeof(_66) = *const {l117} i8
        // 95: prefix:   l117 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    // 97: file: typeof(_68) = *mut {l120} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 97: file:   l120 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0" as *c ... _char: typeof(_69) = *const {l122} i8
    // 97: b"%s\n\0" as *c ... _char:   l122 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0" as *c ... st u8: typeof(_70) = *const {l124} u8
    // 97: b"%s\n\0" as *c ... st u8:   l124 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0": typeof(_71) = *const {l126} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 97: b"%s\n\0":   l126 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0": typeof(_72) = & {l128} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 97: b"%s\n\0":   l128 = UNIQUE | NON_NULL, FIXED
    // 97: prefix: typeof(_73) = *const {l130} i8
    // 97: prefix:   l130 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0" as *c ... _char: typeof(_69 = move _70 as *const i8 (Misc)) = *const {l401} i8
    // 97: b"%s\n\0" as *c ... _char:   l401 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0": typeof(_72 = const b"%s\n\x00") = & {l398} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 97: b"%s\n\0":   l398 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0" as *c ... st u8: typeof(_70 = move _71 as *const u8 (Pointer(ArrayToPointer))) = *const {l400} u8
    // 97: b"%s\n\0" as *c ... st u8:   l400 = UNIQUE | NON_NULL, (empty)
    // 97: b"%s\n\0": typeof(_71 = &raw const (*_72)) = *const {l399} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 97: b"%s\n\0":   l399 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 99: file: typeof(_75) = *mut {l133} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 99: file:   l133 = UNIQUE | NON_NULL, (empty)
        b"%sreleased %s\n\0" as *const u8 as *const libc::c_char,
        // 100: b"%sreleased %s ... _char: typeof(_76) = *const {l135} i8
        // 100: b"%sreleased %s ... _char:   l135 = UNIQUE | NON_NULL, (empty)
        // 100: b"%sreleased %s ... st u8: typeof(_77) = *const {l137} u8
        // 100: b"%sreleased %s ... st u8:   l137 = UNIQUE | NON_NULL, (empty)
        // 100: b"%sreleased %s\n\0": typeof(_78) = *const {l139} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 100: b"%sreleased %s\n\0":   l139 = UNIQUE | NON_NULL, (empty)
        // 100: b"%sreleased %s\n\0": typeof(_79) = & {l141} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 100: b"%sreleased %s\n\0":   l141 = UNIQUE | NON_NULL, FIXED
        // 100: b"%sreleased %s\n\0": typeof(_79 = const b"%sreleased %s\n\x00") = & {l402} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 100: b"%sreleased %s\n\0":   l402 = UNIQUE | NON_NULL, (empty)
        // 100: b"%sreleased %s\n\0": typeof(_78 = &raw const (*_79)) = *const {l403} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 100: b"%sreleased %s\n\0":   l403 = UNIQUE | NON_NULL, (empty)
        // 100: b"%sreleased %s ... st u8: typeof(_77 = move _78 as *const u8 (Pointer(ArrayToPointer))) = *const {l404} u8
        // 100: b"%sreleased %s ... st u8:   l404 = UNIQUE | NON_NULL, (empty)
        // 100: b"%sreleased %s ... _char: typeof(_76 = move _77 as *const i8 (Misc)) = *const {l405} i8
        // 100: b"%sreleased %s ... _char:   l405 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 101: prefix: typeof(_80) = *const {l143} i8
        // 101: prefix:   l143 = UNIQUE | NON_NULL, (empty)
        b"Mon Aug 19 16:52:16 PDT 2024\0" as *const u8 as *const libc::c_char,
        // 102: b"Mon Aug 19 16 ... _char: typeof(_81) = *const {l145} i8
        // 102: b"Mon Aug 19 16 ... _char:   l145 = UNIQUE | NON_NULL, (empty)
        // 102: b"Mon Aug 19 16 ... st u8: typeof(_82) = *const {l147} u8
        // 102: b"Mon Aug 19 16 ... st u8:   l147 = UNIQUE | NON_NULL, (empty)
        // 102: b"Mon Aug 19 16 ... 24\0": typeof(_83) = *const {l149} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 102: b"Mon Aug 19 16 ... 24\0":   l149 = UNIQUE | NON_NULL, (empty)
        // 102: b"Mon Aug 19 16 ... 24\0": typeof(_84) = & {l151} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 102: b"Mon Aug 19 16 ... 24\0":   l151 = UNIQUE | NON_NULL, FIXED
        // 102: b"Mon Aug 19 16 ... 24\0": typeof(_83 = &raw const (*_84)) = *const {l407} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 102: b"Mon Aug 19 16 ... 24\0":   l407 = UNIQUE | NON_NULL, (empty)
        // 102: b"Mon Aug 19 16 ... _char: typeof(_81 = move _82 as *const i8 (Misc)) = *const {l409} i8
        // 102: b"Mon Aug 19 16 ... _char:   l409 = UNIQUE | NON_NULL, (empty)
        // 102: b"Mon Aug 19 16 ... 24\0": typeof(_84 = const b"Mon Aug 19 16:52:16 PDT 2024\x00") = & {l406} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 102: b"Mon Aug 19 16 ... 24\0":   l406 = UNIQUE | NON_NULL, (empty)
        // 102: b"Mon Aug 19 16 ... st u8: typeof(_82 = move _83 as *const u8 (Pointer(ArrayToPointer))) = *const {l408} u8
        // 102: b"Mon Aug 19 16 ... st u8:   l408 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(
        file,
        // 105: file: typeof(_86) = *mut {l154} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 105: file:   l154 = UNIQUE | NON_NULL, (empty)
        b"%scompiled %s\n\0" as *const u8 as *const libc::c_char,
        // 106: b"%scompiled %s ... _char: typeof(_87) = *const {l156} i8
        // 106: b"%scompiled %s ... _char:   l156 = UNIQUE | NON_NULL, (empty)
        // 106: b"%scompiled %s ... st u8: typeof(_88) = *const {l158} u8
        // 106: b"%scompiled %s ... st u8:   l158 = UNIQUE | NON_NULL, (empty)
        // 106: b"%scompiled %s\n\0": typeof(_89) = *const {l160} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 106: b"%scompiled %s\n\0":   l160 = UNIQUE | NON_NULL, (empty)
        // 106: b"%scompiled %s\n\0": typeof(_90) = & {l162} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 106: b"%scompiled %s\n\0":   l162 = UNIQUE | NON_NULL, FIXED
        // 106: b"%scompiled %s\n\0": typeof(_89 = &raw const (*_90)) = *const {l411} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 106: b"%scompiled %s\n\0":   l411 = UNIQUE | NON_NULL, (empty)
        // 106: b"%scompiled %s ... st u8: typeof(_88 = move _89 as *const u8 (Pointer(ArrayToPointer))) = *const {l412} u8
        // 106: b"%scompiled %s ... st u8:   l412 = UNIQUE | NON_NULL, (empty)
        // 106: b"%scompiled %s\n\0": typeof(_90 = const b"%scompiled %s\n\x00") = & {l410} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000000f)) }]
        // 106: b"%scompiled %s\n\0":   l410 = UNIQUE | NON_NULL, (empty)
        // 106: b"%scompiled %s ... _char: typeof(_87 = move _88 as *const i8 (Misc)) = *const {l413} i8
        // 106: b"%scompiled %s ... _char:   l413 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 107: prefix: typeof(_91) = *const {l164} i8
        // 107: prefix:   l164 = UNIQUE | NON_NULL, (empty)
        b"Mon Aug 19 16:52:16 PDT 2024\0" as *const u8 as *const libc::c_char,
        // 108: b"Mon Aug 19 16 ... _char: typeof(_92) = *const {l166} i8
        // 108: b"Mon Aug 19 16 ... _char:   l166 = UNIQUE | NON_NULL, (empty)
        // 108: b"Mon Aug 19 16 ... st u8: typeof(_93) = *const {l168} u8
        // 108: b"Mon Aug 19 16 ... st u8:   l168 = UNIQUE | NON_NULL, (empty)
        // 108: b"Mon Aug 19 16 ... 24\0": typeof(_94) = *const {l170} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 108: b"Mon Aug 19 16 ... 24\0":   l170 = UNIQUE | NON_NULL, (empty)
        // 108: b"Mon Aug 19 16 ... 24\0": typeof(_95) = & {l172} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 108: b"Mon Aug 19 16 ... 24\0":   l172 = UNIQUE | NON_NULL, FIXED
        // 108: b"Mon Aug 19 16 ... st u8: typeof(_93 = move _94 as *const u8 (Pointer(ArrayToPointer))) = *const {l416} u8
        // 108: b"Mon Aug 19 16 ... st u8:   l416 = UNIQUE | NON_NULL, (empty)
        // 108: b"Mon Aug 19 16 ... 24\0": typeof(_95 = const b"Mon Aug 19 16:52:16 PDT 2024\x00") = & {l414} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 108: b"Mon Aug 19 16 ... 24\0":   l414 = UNIQUE | NON_NULL, (empty)
        // 108: b"Mon Aug 19 16 ... _char: typeof(_92 = move _93 as *const i8 (Misc)) = *const {l417} i8
        // 108: b"Mon Aug 19 16 ... _char:   l417 = UNIQUE | NON_NULL, (empty)
        // 108: b"Mon Aug 19 16 ... 24\0": typeof(_94 = &raw const (*_95)) = *const {l415} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000001d)) }]
        // 108: b"Mon Aug 19 16 ... 24\0":   l415 = UNIQUE | NON_NULL, (empty)
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    // 110: file: typeof(_97) = *mut {l175} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 110: file:   l175 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0" as *c ... _char: typeof(_98) = *const {l177} i8
    // 110: b"%s\n\0" as *c ... _char:   l177 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0" as *c ... st u8: typeof(_99) = *const {l179} u8
    // 110: b"%s\n\0" as *c ... st u8:   l179 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0": typeof(_100) = *const {l181} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 110: b"%s\n\0":   l181 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0": typeof(_101) = & {l183} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 110: b"%s\n\0":   l183 = UNIQUE | NON_NULL, FIXED
    // 110: prefix: typeof(_102) = *const {l185} i8
    // 110: prefix:   l185 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0" as *c ... st u8: typeof(_99 = move _100 as *const u8 (Pointer(ArrayToPointer))) = *const {l420} u8
    // 110: b"%s\n\0" as *c ... st u8:   l420 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0": typeof(_101 = const b"%s\n\x00") = & {l418} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 110: b"%s\n\0":   l418 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0": typeof(_100 = &raw const (*_101)) = *const {l419} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 110: b"%s\n\0":   l419 = UNIQUE | NON_NULL, (empty)
    // 110: b"%s\n\0" as *c ... _char: typeof(_98 = move _99 as *const i8 (Misc)) = *const {l421} i8
    // 110: b"%s\n\0" as *c ... _char:   l421 = UNIQUE | NON_NULL, (empty)
    fprintf(
        file,
        // 112: file: typeof(_104) = *mut {l188} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 112: file:   l188 = UNIQUE | NON_NULL, (empty)
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        // 113: b"%s%s\n\0" as  ... _char: typeof(_105) = *const {l190} i8
        // 113: b"%s%s\n\0" as  ... _char:   l190 = UNIQUE | NON_NULL, (empty)
        // 113: b"%s%s\n\0" as  ... st u8: typeof(_106) = *const {l192} u8
        // 113: b"%s%s\n\0" as  ... st u8:   l192 = UNIQUE | NON_NULL, (empty)
        // 113: b"%s%s\n\0": typeof(_107) = *const {l194} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 113: b"%s%s\n\0":   l194 = UNIQUE | NON_NULL, (empty)
        // 113: b"%s%s\n\0": typeof(_108) = & {l196} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 113: b"%s%s\n\0":   l196 = UNIQUE | NON_NULL, FIXED
        // 113: b"%s%s\n\0": typeof(_107 = &raw const (*_108)) = *const {l423} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 113: b"%s%s\n\0":   l423 = UNIQUE | NON_NULL, (empty)
        // 113: b"%s%s\n\0" as  ... _char: typeof(_105 = move _106 as *const i8 (Misc)) = *const {l425} i8
        // 113: b"%s%s\n\0" as  ... _char:   l425 = UNIQUE | NON_NULL, (empty)
        // 113: b"%s%s\n\0": typeof(_108 = const b"%s%s\n\x00") = & {l422} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 113: b"%s%s\n\0":   l422 = UNIQUE | NON_NULL, (empty)
        // 113: b"%s%s\n\0" as  ... st u8: typeof(_106 = move _107 as *const u8 (Pointer(ArrayToPointer))) = *const {l424} u8
        // 113: b"%s%s\n\0" as  ... st u8:   l424 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 114: prefix: typeof(_109) = *const {l198} i8
        // 114: prefix:   l198 = UNIQUE | NON_NULL, (empty)
        b"gcc (Ubuntu 13.2.0-23ubuntu4) 13.2.0\0" as *const u8 as *const libc::c_char,
        // 115: b"gcc (Ubuntu 1 ... _char: typeof(_110) = *const {l200} i8
        // 115: b"gcc (Ubuntu 1 ... _char:   l200 = UNIQUE | NON_NULL, (empty)
        // 115: b"gcc (Ubuntu 1 ... st u8: typeof(_111) = *const {l202} u8
        // 115: b"gcc (Ubuntu 1 ... st u8:   l202 = UNIQUE | NON_NULL, (empty)
        // 115: b"gcc (Ubuntu 1 ... .0\0": typeof(_112) = *const {l204} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 115: b"gcc (Ubuntu 1 ... .0\0":   l204 = UNIQUE | NON_NULL, (empty)
        // 115: b"gcc (Ubuntu 1 ... .0\0": typeof(_113) = & {l206} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 115: b"gcc (Ubuntu 1 ... .0\0":   l206 = UNIQUE | NON_NULL, FIXED
        // 115: b"gcc (Ubuntu 1 ... st u8: typeof(_111 = move _112 as *const u8 (Pointer(ArrayToPointer))) = *const {l428} u8
        // 115: b"gcc (Ubuntu 1 ... st u8:   l428 = UNIQUE | NON_NULL, (empty)
        // 115: b"gcc (Ubuntu 1 ... .0\0": typeof(_113 = const b"gcc (Ubuntu 13.2.0-23ubuntu4) 13.2.0\x00") = & {l426} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 115: b"gcc (Ubuntu 1 ... .0\0":   l426 = UNIQUE | NON_NULL, (empty)
        // 115: b"gcc (Ubuntu 1 ... _char: typeof(_110 = move _111 as *const i8 (Misc)) = *const {l429} i8
        // 115: b"gcc (Ubuntu 1 ... _char:   l429 = UNIQUE | NON_NULL, (empty)
        // 115: b"gcc (Ubuntu 1 ... .0\0": typeof(_112 = &raw const (*_113)) = *const {l427} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000025)) }]
        // 115: b"gcc (Ubuntu 1 ... .0\0":   l427 = UNIQUE | NON_NULL, (empty)
    );
    loop {
        fputs(prefix, file);
        // 118: prefix: typeof(_117) = *const {l211} i8
        // 118: prefix:   l211 = UNIQUE | NON_NULL, (empty)
        // 118: file: typeof(_118) = *mut {l213} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 118: file:   l213 = UNIQUE | NON_NULL, (empty)
        q = p;
        // 119: p: typeof(_119) = *const {l215} i8
        // 119: p:   l215 = UNIQUE | NON_NULL, (empty)
        while *q as libc::c_int != 0 && *q as libc::c_int != ' ' as i32 {
            q = q.offset(1);
            // 121: q.offset(1): typeof(_129) = *const {l226} i8
            // 121: q.offset(1):   l226 = UNIQUE | NON_NULL, (empty)
            // 121: q: typeof(_130) = *const {l228} i8
            // 121: q:   l228 = UNIQUE | NON_NULL, (empty)
            q;
            // 122: q: typeof(_131) = *const {l230} i8
            // 122: q:   l230 = UNIQUE | NON_NULL, (empty)
        }
        if *q as libc::c_int != 0 && (q.offset_from(p) as libc::c_long) < len as libc::c_long {
        // 124: q: typeof(_143) = *const {l243} i8
        // 124: q:   l243 = UNIQUE | NON_NULL, (empty)
        // 124: p: typeof(_144) = *const {l245} i8
        // 124: p:   l245 = UNIQUE | NON_NULL, (empty)
            loop {
                n = q.offset(1 as libc::c_int as isize);
                // 126: q.offset(1 as l ... size): typeof(_147) = *const {l249} i8
                // 126: q.offset(1 as l ... size):   l249 = UNIQUE | NON_NULL, (empty)
                // 126: q: typeof(_148) = *const {l251} i8
                // 126: q:   l251 = UNIQUE | NON_NULL, (empty)
                while *n as libc::c_int != 0 && *n as libc::c_int != ' ' as i32 {
                    n = n.offset(1);
                    // 128: n.offset(1): typeof(_160) = *const {l264} i8
                    // 128: n.offset(1):   l264 = UNIQUE | NON_NULL, (empty)
                    // 128: n: typeof(_161) = *const {l266} i8
                    // 128: n:   l266 = UNIQUE | NON_NULL, (empty)
                    n;
                    // 129: n: typeof(_162) = *const {l268} i8
                    // 129: n:   l268 = UNIQUE | NON_NULL, (empty)
                }
                if n.offset_from(p) as libc::c_long >= len as libc::c_long {
                // 131: n: typeof(_170) = *const {l277} i8
                // 131: n:   l277 = UNIQUE | NON_NULL, (empty)
                // 131: p: typeof(_171) = *const {l279} i8
                // 131: p:   l279 = UNIQUE | NON_NULL, (empty)
                    break;
                }
                q = n;
                // 134: n: typeof(_175) = *const {l284} i8
                // 134: n:   l284 = UNIQUE | NON_NULL, (empty)
                if *n == 0 {
                    break;
                }
            }
        }
        while p < q {
        // 140: p: typeof(_181) = *const {l291} i8
        // 140: p:   l291 = UNIQUE | NON_NULL, (empty)
        // 140: q: typeof(_182) = *const {l293} i8
        // 140: q:   l293 = UNIQUE | NON_NULL, (empty)
            let fresh0 = p;
            // 141: fresh0: typeof(_183) = *const {l295} i8
            // 141: fresh0:   l295 = UNIQUE | NON_NULL, (empty)
            p = p.offset(1);
            // 142: p.offset(1): typeof(_184) = *const {l297} i8
            // 142: p.offset(1):   l297 = UNIQUE | NON_NULL, (empty)
            // 142: p: typeof(_185) = *const {l299} i8
            // 142: p:   l299 = UNIQUE | NON_NULL, (empty)
            fputc(*fresh0 as libc::c_int, file);
            // 143: file: typeof(_189) = *mut {l304} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
            // 143: file:   l304 = UNIQUE | NON_NULL, (empty)
        }
        fputc('\n' as i32, file);
        // 145: file: typeof(_195) = *mut {l311} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 145: file:   l311 = UNIQUE | NON_NULL, (empty)
        if *p == 0 {
            break;
        }
        p = p.offset(1);
        // 149: p.offset(1): typeof(_200) = *const {l317} i8
        // 149: p.offset(1):   l317 = UNIQUE | NON_NULL, (empty)
        // 149: p: typeof(_201) = *const {l319} i8
        // 149: p:   l319 = UNIQUE | NON_NULL, (empty)
        p;
        // 150: p: typeof(_202) = *const {l321} i8
        // 150: p:   l321 = UNIQUE | NON_NULL, (empty)
    }
    fprintf(
        file,
        // 153: file: typeof(_204) = *mut {l324} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
        // 153: file:   l324 = UNIQUE | NON_NULL, (empty)
        b"%s%s\n\0" as *const u8 as *const libc::c_char,
        // 154: b"%s%s\n\0" as  ... _char: typeof(_205) = *const {l326} i8
        // 154: b"%s%s\n\0" as  ... _char:   l326 = UNIQUE | NON_NULL, (empty)
        // 154: b"%s%s\n\0" as  ... st u8: typeof(_206) = *const {l328} u8
        // 154: b"%s%s\n\0" as  ... st u8:   l328 = UNIQUE | NON_NULL, (empty)
        // 154: b"%s%s\n\0": typeof(_207) = *const {l330} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 154: b"%s%s\n\0":   l330 = UNIQUE | NON_NULL, (empty)
        // 154: b"%s%s\n\0": typeof(_208) = & {l332} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 154: b"%s%s\n\0":   l332 = UNIQUE | NON_NULL, FIXED
        // 154: b"%s%s\n\0": typeof(_207 = &raw const (*_208)) = *const {l431} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 154: b"%s%s\n\0":   l431 = UNIQUE | NON_NULL, (empty)
        // 154: b"%s%s\n\0" as  ... st u8: typeof(_206 = move _207 as *const u8 (Pointer(ArrayToPointer))) = *const {l432} u8
        // 154: b"%s%s\n\0" as  ... st u8:   l432 = UNIQUE | NON_NULL, (empty)
        // 154: b"%s%s\n\0": typeof(_208 = const b"%s%s\n\x00") = & {l430} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000006)) }]
        // 154: b"%s%s\n\0":   l430 = UNIQUE | NON_NULL, (empty)
        // 154: b"%s%s\n\0" as  ... _char: typeof(_205 = move _206 as *const i8 (Misc)) = *const {l433} i8
        // 154: b"%s%s\n\0" as  ... _char:   l433 = UNIQUE | NON_NULL, (empty)
        prefix,
        // 155: prefix: typeof(_209) = *const {l334} i8
        // 155: prefix:   l334 = UNIQUE | NON_NULL, (empty)
        b"Linux LAPTOP-SOU2J6CG 6.1.21.2-microsoft-standard-WSL2+ x86_64\0" as *const u8
        // 156: b"Linux LAPTOP- ... _char: typeof(_210) = *const {l336} i8
        // 156: b"Linux LAPTOP- ... _char:   l336 = UNIQUE | NON_NULL, (empty)
        // 156: b"Linux LAPTOP- ... st u8: typeof(_211) = *const {l338} u8
        // 156: b"Linux LAPTOP- ... st u8:   l338 = UNIQUE | NON_NULL, (empty)
        // 156: b"Linux LAPTOP- ... 64\0": typeof(_212) = *const {l340} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 156: b"Linux LAPTOP- ... 64\0":   l340 = UNIQUE | NON_NULL, (empty)
        // 156: b"Linux LAPTOP- ... 64\0": typeof(_213) = & {l342} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 156: b"Linux LAPTOP- ... 64\0":   l342 = UNIQUE | NON_NULL, FIXED
        // 156: b"Linux LAPTOP- ... st u8: typeof(_211 = move _212 as *const u8 (Pointer(ArrayToPointer))) = *const {l436} u8
        // 156: b"Linux LAPTOP- ... st u8:   l436 = UNIQUE | NON_NULL, (empty)
        // 156: b"Linux LAPTOP- ... 64\0": typeof(_212 = &raw const (*_213)) = *const {l435} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 156: b"Linux LAPTOP- ... 64\0":   l435 = UNIQUE | NON_NULL, (empty)
        // 156: b"Linux LAPTOP- ... 64\0": typeof(_213 = const b"Linux LAPTOP-SOU2J6CG 6.1.21.2-microsoft-standard-WSL2+ x86_64\x00") = & {l434} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000003f)) }]
        // 156: b"Linux LAPTOP- ... 64\0":   l434 = UNIQUE | NON_NULL, (empty)
        // 156: b"Linux LAPTOP- ... _char: typeof(_210 = move _211 as *const i8 (Misc)) = *const {l437} i8
        // 156: b"Linux LAPTOP- ... _char:   l437 = UNIQUE | NON_NULL, (empty)
            as *const libc::c_char,
    );
    fprintf(file, b"%s\n\0" as *const u8 as *const libc::c_char, prefix);
    // 159: file: typeof(_215) = *mut {l345} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 159: file:   l345 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0" as *c ... _char: typeof(_216) = *const {l347} i8
    // 159: b"%s\n\0" as *c ... _char:   l347 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0" as *c ... st u8: typeof(_217) = *const {l349} u8
    // 159: b"%s\n\0" as *c ... st u8:   l349 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0": typeof(_218) = *const {l351} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 159: b"%s\n\0":   l351 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0": typeof(_219) = & {l353} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 159: b"%s\n\0":   l353 = UNIQUE | NON_NULL, FIXED
    // 159: prefix: typeof(_220) = *const {l355} i8
    // 159: prefix:   l355 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0": typeof(_218 = &raw const (*_219)) = *const {l439} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 159: b"%s\n\0":   l439 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0" as *c ... _char: typeof(_216 = move _217 as *const i8 (Misc)) = *const {l441} i8
    // 159: b"%s\n\0" as *c ... _char:   l441 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0": typeof(_219 = const b"%s\n\x00") = & {l438} [u8; Const { ty: usize, kind: Value(Leaf(0x0000000000000004)) }]
    // 159: b"%s\n\0":   l438 = UNIQUE | NON_NULL, (empty)
    // 159: b"%s\n\0" as *c ... st u8: typeof(_217 = move _218 as *const u8 (Pointer(ArrayToPointer))) = *const {l440} u8
    // 159: b"%s\n\0" as *c ... st u8:   l440 = UNIQUE | NON_NULL, (empty)
    fflush(file);
    // 160: file: typeof(_222) = *mut {l358} DefId(0:1688 ~ libgeling[8679]::lglbnr::_IO_FILE)
    // 160: file:   l358 = UNIQUE | NON_NULL, (empty)
}
#[no_mangle]
pub unsafe extern "C" fn lglversion<'h0>() -> &'h0 (libc::c_char) {
// 163: *const libc::c_char: typeof(_0) = *const {g3} i8
// 163: *const libc::c_char:   g3 = UNIQUE | NON_NULL, (empty)
    return &*(b"1.0.0 89a167d0d2efe98d983c87b5b84175b40ea55842\0") as *const u8 as *const libc::c_char;
    // 164: b"1.0.0 89a167d ... st u8: typeof(_2) = *const {l2} u8
    // 164: b"1.0.0 89a167d ... st u8:   l2 = UNIQUE | NON_NULL, (empty)
    // 164: b"1.0.0 89a167d ... 42\0": typeof(_3) = *const {l4} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
    // 164: b"1.0.0 89a167d ... 42\0":   l4 = UNIQUE | NON_NULL, (empty)
    // 164: b"1.0.0 89a167d ... 42\0": typeof(_4) = & {l6} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
    // 164: b"1.0.0 89a167d ... 42\0":   l6 = UNIQUE | NON_NULL, FIXED
    // 164: b"1.0.0 89a167d ... _char: typeof(_0 = move _2 as *const i8 (Misc)) = *const {l11} i8
    // 164: b"1.0.0 89a167d ... _char:   l11 = UNIQUE | NON_NULL, (empty)
    // 164: b"1.0.0 89a167d ... 42\0": typeof(_3 = &raw const (*_4)) = *const {l9} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
    // 164: b"1.0.0 89a167d ... 42\0":   l9 = UNIQUE | NON_NULL, (empty)
    // 164: b"1.0.0 89a167d ... st u8: typeof(_2 = move _3 as *const u8 (Pointer(ArrayToPointer))) = *const {l10} u8
    // 164: b"1.0.0 89a167d ... st u8:   l10 = UNIQUE | NON_NULL, (empty)
    // 164: b"1.0.0 89a167d ... 42\0": typeof(_4 = const b"1.0.0 89a167d0d2efe98d983c87b5b84175b40ea55842\x00") = & {l8} [u8; Const { ty: usize, kind: Value(Leaf(0x000000000000002f)) }]
    // 164: b"1.0.0 89a167d ... 42\0":   l8 = READ | UNIQUE | OFFSET_ADD | NON_NULL, (empty)
}
