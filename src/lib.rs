use std::ffi::{CString, c_char};


#[macro_export]
macro_rules! rustrun_info {
    ($id:ident, $name:ident, $desc:ident) => {

#[unsafe(no_mangle)]
pub extern "C" fn get_plugin_info(which: u8) -> *mut std::ffi::c_char {
    match which {
        0 => std::ffi::CString::new($id).unwrap().into_raw(),
        1 => std::ffi::CString::new($name).unwrap().into_raw(),
        2 => std::ffi::CString::new($desc).unwrap().into_raw(),
        _ => std::ffi::CString::new("").unwrap().into_raw(),
    }
}

    }
}

#[derive(Clone, Debug)]
pub struct SearchState {
    pub query: String,
    pub results: Vec<SearchResult>,
    pub next_result: usize,
    pub finished: bool,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub query_text_display: String,
    pub ico_path: String,
    pub title: String,
    pub subtitle: String,
    pub tooltip: (String, String),
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CSearchResult {
    pub query_text_display: *mut c_char,
    pub ico_path: *mut c_char,
    pub title: *mut c_char,
    pub subtitle: *mut c_char,
    pub tooltip_a: *mut c_char,
    pub tooltip_b: *mut c_char,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CSSearchResult {
    pub query_text_display_length: i32,
    pub query_text_display: *const u16,
    pub ico_path_length: i32,
    pub ico_path: *const u16,
    pub title_length: i32,
    pub title: *const u16,
    pub subtitle_length: i32,
    pub subtitle: *const u16,
    pub tooltip_a_length: i32,
    pub tooltip_a: *const u16,
    pub tooltip_b_length: i32,
    pub tooltip_b: *const u16,
}

#[derive(Clone, Debug)]
pub struct ContextMenuResult {
    pub plugin_name: String,
    pub title: String,
    pub font_family: String,
    pub glyph: String,
    pub accelerator_key: i32,
    pub accelerator_modifiers: i32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CContextMenuResult {
    pub plugin_name: *mut c_char,
    pub title: *mut c_char,
    pub font_family: *mut c_char,
    pub glyph: *mut c_char,
    pub accelerator_key: i32,
    pub accelerator_modifiers: i32,
}

pub fn to_c_str(str: &str) -> *mut c_char {
    let c_str = CString::new(str).unwrap();
    c_str.into_raw()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_c_string(str: *mut c_char) {
    unsafe {
        let x = CString::from_raw(str);
        drop(x);
    }
}

pub unsafe fn take_c_string(str: *mut c_char) -> CString {
    unsafe {
        CString::from_raw(str)
    }
}

pub unsafe fn take_cs_string(utf16_str: *const u16, utf16_len: i32) -> String {
        let slice = unsafe { std::slice::from_raw_parts(utf16_str, utf16_len as usize) };
        String::from_utf16(slice).unwrap()
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SearchResults {
    pub len: usize,
    pub ptr: *mut CSearchResult,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ContextMenuResults {
    pub len: usize,
    pub ptr: *mut CContextMenuResult,
}

#[macro_export]
macro_rules! impl_rustrun {
    ($search:ident, $ctx:ident) => {

#[unsafe(no_mangle)]
pub unsafe extern "C" fn init_search(utf16_str: *const u16, utf16_len: i32) -> SearchResults {
    let query = unsafe {
        take_cs_string(utf16_str, utf16_len)
    };

    let boxx = $search(query)
        .iter()
        .map(|rsr| CSearchResult {
            query_text_display: to_c_str(&rsr.query_text_display),
            ico_path: to_c_str(&rsr.ico_path),
            title: to_c_str(&rsr.title),
            subtitle: to_c_str(&rsr.subtitle),
            tooltip_a: to_c_str(&rsr.tooltip.0),
            tooltip_b: to_c_str(&rsr.tooltip.1),
        })
        .collect::<Vec<CSearchResult>>()
        .into_boxed_slice();

    let len = boxx.len();
    let ptr = Box::<[CSearchResult]>::into_raw(boxx) as *mut CSearchResult;
    SearchResults { len, ptr }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_context_menu(cssr: CSSearchResult) -> ContextMenuResults {
    let sr: SearchResult = unsafe {
        SearchResult {
        query_text_display: take_cs_string(cssr.query_text_display, cssr.query_text_display_length),
        ico_path: take_cs_string(cssr.ico_path, cssr.ico_path_length),
        title: take_cs_string(cssr.title, cssr.title_length),
        subtitle: take_cs_string(cssr.subtitle, cssr.subtitle_length),
        tooltip: (take_cs_string(cssr.tooltip_a, cssr.tooltip_a_length),
        take_cs_string(cssr.tooltip_b, cssr.tooltip_b_length))
        }
    };
    let boxx = $ctx(sr)
        .iter()
        .map(|cosr| CContextMenuResult {
            plugin_name: to_c_str(&cosr.plugin_name),
            title: to_c_str(&cosr.title),
            font_family: to_c_str(&cosr.font_family),
            glyph: to_c_str(&cosr.glyph),
            accelerator_key: cosr.accelerator_key,
            accelerator_modifiers: cosr.accelerator_modifiers,
        })
        .collect::<Vec<CContextMenuResult>>()
        .into_boxed_slice();

    let len = boxx.len();
    let ptr = Box::<[CContextMenuResult]>::into_raw(boxx) as *mut CContextMenuResult;
    ContextMenuResults { len, ptr }
}
       
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_search(srs: SearchResults) {
    let boxx = unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(srs.ptr, srs.len)) };
    drop(boxx)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_search_result(csr: CSearchResult) {
    unsafe {
        free_c_string(csr.query_text_display);
        free_c_string(csr.ico_path);
        free_c_string(csr.title);
        free_c_string(csr.subtitle);
        free_c_string(csr.tooltip_a);
        free_c_string(csr.tooltip_b);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_context_menu_result(cs: CContextMenuResult) {
    unsafe {
        free_c_string(cs.plugin_name);
        free_c_string(cs.title);
        free_c_string(cs.font_family);
        free_c_string(cs.glyph);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_context_menu(css: ContextMenuResults) {
    let boxx = unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(css.ptr, css.len)) };
    drop(boxx)
}