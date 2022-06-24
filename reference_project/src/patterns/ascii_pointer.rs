use crate::types::UseAsciiStringPattern;
use interoptopus::ffi_function;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::AsciiPointer;

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_1(x: AsciiPointer) -> u32 {
    x.as_c_str().map(|x| x.to_bytes().len()).unwrap_or(0) as u32
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_2() -> AsciiPointer<'static> {
    AsciiPointer::empty()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_len(x: AsciiPointer, y: UseAsciiStringPattern) -> u32 {
    let x1 = x.as_str().map(|x| x.len()).unwrap_or(0);
    let x2 = y.ascii_string.as_str().map(|x| x.len()).unwrap_or(0);
    (x1 + x2) as u32
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_return_slice() -> FFISlice<'static, UseAsciiStringPattern<'static>> {
    FFISlice::empty()
}

static RV:&[u8] = concat!("Hello world, Καλημέρα κόσμε, コンニチハ", "\0").as_bytes();

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_return_utf8() -> AsciiPointer<'static>
{
    AsciiPointer::from_slice_with_nul(RV).unwrap_or_default()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn pattern_ascii_pointer_param_utf8(x: AsciiPointer) -> u32
{
    let y = x.as_str().unwrap_or("");
    y.chars().count() as u32
}