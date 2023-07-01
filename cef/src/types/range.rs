use cef_sys::cef_range_t;

#[derive(Clone, Debug)]
pub struct CefRange {
    pub from: u32,
    pub to: u32,
}
impl CefRange {
    pub(crate) fn from_ptr(raw: *const cef_range_t) -> Self {
        Self::from(unsafe { &*raw })
    }
    pub(crate) fn from(raw: &cef_range_t) -> Self {
        Self {
            from: raw.from,
            to: raw.to,
        }
    }
}
impl Default for CefRange {
    fn default() -> Self {
        Self { from: 0, to: 0 }
    }
}
