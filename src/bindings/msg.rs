use core::mem::transmute;
use core::slice;

use cstr_core::CStr;
use ndless_sys::_show_msgbox;

use crate::cstr;
use crate::prelude::*;

#[repr(u32)]
#[derive(Debug)]
pub enum Button {
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

pub fn msg(title: impl Into<String>, msg: impl Into<String>) {
    let title = cstr!(title.into());
    let msg = cstr!(msg.into());
    unsafe {
        _show_msgbox(title.as_ptr(), msg.as_ptr(), 0);
    }
}

pub fn msg_2b(
    title: impl Into<String>,
    msg: impl Into<String>,
    btn1: impl Into<String>,
    btn2: impl Into<String>,
) -> Button {
    let title = cstr!(title.into());
    let msg = cstr!(msg.into());
    let btn1 = cstr!(btn1.into());
    let btn2 = cstr!(btn2.into());
    unsafe {
        transmute(_show_msgbox(
            title.as_ptr(),
            msg.as_ptr(),
            2,
            btn1.as_ptr(),
            btn2.as_ptr(),
        ))
    }
}

pub fn msg_3b(
    title: impl Into<String>,
    msg: impl Into<String>,
    btn1: impl Into<String>,
    btn2: impl Into<String>,
    btn3: impl Into<String>,
) -> Button {
    let title = cstr!(title.into());
    let msg = cstr!(msg.into());
    let btn1 = cstr!(btn1.into());
    let btn2 = cstr!(btn2.into());
    let btn3 = cstr!(btn3.into());
    unsafe {
        transmute(_show_msgbox(
            title.as_ptr(),
            msg.as_ptr(),
            3,
            btn1.as_ptr(),
            btn2.as_ptr(),
            btn3.as_ptr(),
        ))
    }
}

pub fn msg_numeric(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    msg: impl Into<String>,
    min: i32,
    max: i32,
) -> Option<i32> {
    let title = cstr!(title.into());
    let subtitle = cstr!(subtitle.into());
    let msg = cstr!(msg.into());
    let mut num = 0i32;
    match unsafe {
        ndless_sys::show_1numeric_input(
            title.as_ptr(),
            subtitle.as_ptr(),
            msg.as_ptr(),
            &mut num,
            min,
            max,
        )
    } {
        1 => Some(num),
        _ => None,
    }
}

pub fn msg_2numeric(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    msg1: impl Into<String>,
    min1: i32,
    max1: i32,
    msg2: impl Into<String>,
    min2: i32,
    max2: i32,
) -> Option<(i32, i32)> {
    let title = cstr!(title.into());
    let subtitle = cstr!(subtitle.into());
    let msg1 = cstr!(msg1.into());
    let msg2 = cstr!(msg2.into());
    let mut num1 = 0i32;
    let mut num2 = 0i32;
    match unsafe {
        ndless_sys::show_2numeric_input(
            title.as_ptr(),
            subtitle.as_ptr(),
            msg1.as_ptr(),
            &mut num1,
            min1,
            max1,
            msg2.as_ptr(),
            &mut num2,
            min2,
            max2,
        )
    } {
        1 => Some((num1, num2)),
        _ => None,
    }
}

pub fn msg_input(
    title: impl Into<String>,
    msg: impl Into<String>,
    default: impl Into<String>,
) -> Option<String> {
    let title = cstr!(title.into());
    let msg = cstr!(msg.into());
    let default = cstr!(default.into());
    let mut ptr: *mut cty::c_char = core::ptr::null_mut();
    let ret = match unsafe {
        ndless_sys::show_msg_user_input(title.as_ptr(), msg.as_ptr(), default.as_ptr(), &mut ptr)
    } {
        -1 => None,
        len => unsafe {
            Some(
                CStr::from_bytes_with_nul_unchecked(slice::from_raw_parts(
                    ptr as *const u8,
                    len as usize + 1,
                ))
                .to_string_lossy()
                .into_owned(),
            )
        },
    };
    unsafe { ndless_sys::free(ptr as *mut cty::c_void) };
    ret
}
