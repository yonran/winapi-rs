
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct INITCOMMONCONTROLSEX {
	pub dwSize: ::DWORD,
	pub dwICC: ::DWORD,
}
// Flags for INITCOMMONCONTROLSEX::dwICC
pub const ICC_LISTVIEW_CLASSES: ::DWORD = 0x00000001; // enable WC_LISTVIEW and WC_HEADER
pub const ICC_TREEVIEW_CLASSES: ::DWORD = 0x00000002; // enable WC_TREEVIEW and TOOLTIPS_CLASS
pub const ICC_BAR_CLASSES: ::DWORD = 0x00000004; // toolbar, statusbar, trackbar, tooltips
pub const ICC_TAB_CLASSES: ::DWORD = 0x00000008; // tab, tooltips
pub const ICC_UPDOWN_CLASS: ::DWORD = 0x00000010; // updown
pub const ICC_PROGRESS_CLASS: ::DWORD = 0x00000020; // progress
pub const ICC_HOTKEY_CLASS: ::DWORD = 0x00000040; // hotkey
pub const ICC_ANIMATE_CLASS: ::DWORD = 0x00000080; // animate
pub const ICC_WIN95_CLASSES: ::DWORD = 0x000000FF;
pub const ICC_DATE_CLASSES: ::DWORD = 0x00000100; // month picker, date picker, time picker, updown
pub const ICC_USEREX_CLASSES: ::DWORD = 0x00000200; // comboex
pub const ICC_COOL_CLASSES: ::DWORD = 0x00000400; // rebar (coolbar) control
pub const ICC_INTERNET_CLASSES: ::DWORD = 0x00000800;
pub const ICC_PAGESCROLLER_CLASS: ::DWORD = 0x00001000;   // page scroller
pub const ICC_NATIVEFNTCTL_CLASS: ::DWORD = 0x00002000;   // native font control
pub const ICC_STANDARD_CLASSES: ::DWORD = 0x00004000;
pub const ICC_LINK_CLASS: ::DWORD = 0x00008000;

// If you enable an owner-drawn item, the DRAWITEMSTRUCT::CtlType will be one of:
pub const ODT_HEADER: ::DWORD = 100;
pub const ODT_TAB: ::DWORD = 101;
pub const ODT_LISTVIEW: ::DWORD = 102;

// Common control messages
pub const CCM_SETBKCOLOR: ::UINT = 0x2001;  // color in lParam
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct COLORSCHEME {
	pub dwSize: ::DWORD,
	pub clrBtnHighlight: ::COLORREF,
	pub clrBtnShadow: ::COLORREF,
}
pub const CCM_SETCOLORSCHEME: ::UINT = 0x2002;  // &COLORSCHEME in lParam
pub const CCM_GETCOLORSCHEME: ::UINT = 0x2003;  // &mut COLORSCHEME in lParam
pub const CCM_GETDROPTARGET: ::UINT = 0x2004;
pub const CCM_SETUNICODEFORMAT: ::UINT = 0x2005;
pub const CCM_GETUNICODEFORMAT: ::UINT = 0x2006;
pub const CCM_SETVERSION: ::UINT = 0x2007;
pub const CCM_GETVERSION: ::UINT = 0x2008;
pub const CCM_SETWINDOWTHEME: ::UINT = 0x200b;
pub const CCM_DPISCALE: ::UINT = 0x200c;  // TRUE in wParam

