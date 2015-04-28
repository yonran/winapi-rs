
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

//4794
// Tree view styles (dwStyle param of CreateWindow/CreateWindowEx)
pub const TVS_HASBUTTONS: ::DWORD = 0x0001;
pub const TVS_HASLINES: ::DWORD = 0x0002;
pub const TVS_LINESATROOT: ::DWORD = 0x0004;
pub const TVS_EDITLABELS: ::DWORD = 0x0008;
pub const TVS_DISABLEDRAGDROP: ::DWORD = 0x0010;
pub const TVS_SHOWSELALWAYS: ::DWORD = 0x0020;
pub const TVS_RTLREADING: ::DWORD = 0x0040;

pub const TVS_NOTOOLTIPS: ::DWORD = 0x0080;
pub const TVS_CHECKBOXES: ::DWORD = 0x0100;
pub const TVS_TRACKSELECT: ::DWORD = 0x0200;
pub const TVS_SINGLEEXPAND: ::DWORD = 0x0400;
pub const TVS_INFOTIP: ::DWORD = 0x0800;
pub const TVS_FULLROWSELECT: ::DWORD = 0x1000;
pub const TVS_NOSCROLL: ::DWORD = 0x2000;
pub const TVS_NONEVENHEIGHT: ::DWORD = 0x4000;
pub const TVS_NOHSCROLL: ::DWORD = 0x8000;  // TVS_NOSCROLL overrides this;

// Tree View extended styles (dwExStyle param of CreateWindowEx)
// xp and above
pub const TVS_EX_NOSINGLECOLLAPSE: ::DWORD = 0x0001;
// vista and above:
pub const TVS_EX_MULTISELECT: ::DWORD = 0x0002; // msdn says not supported; do not use...
pub const TVS_EX_DOUBLEBUFFER: ::DWORD = 0x0004;
pub const TVS_EX_NOINDENTSTATE: ::DWORD = 0x0008;
pub const TVS_EX_RICHTOOLTIP: ::DWORD = 0x0010;
pub const TVS_EX_AUTOHSCROLL: ::DWORD = 0x0020;
pub const TVS_EX_FADEINOUTEXPANDOS: ::DWORD = 0x0040;
pub const TVS_EX_PARTIALCHECKBOXES: ::DWORD = 0x0080;
pub const TVS_EX_EXCLUSIONCHECKBOXES: ::DWORD = 0x0100;
pub const TVS_EX_DIMMEDCHECKBOXES: ::DWORD = 0x0200;
pub const TVS_EX_DRAWIMAGEASYNC: ::DWORD = 0x0400;

struct _TREEITEM {do_not_construct: ::std::marker::PhantomData<()>}
pub type HTREEITEM = *const _TREEITEM;

//4834
// Tree View control Item Flags (TVITEMEXW::mask)
pub const TVIF_TEXT: ::UINT = 0x0001;
pub const TVIF_IMAGE: ::UINT = 0x0002;
pub const TVIF_PARAM: ::UINT = 0x0004;
pub const TVIF_STATE: ::UINT = 0x0008;
pub const TVIF_HANDLE: ::UINT = 0x0010;
pub const TVIF_SELECTEDIMAGE: ::UINT = 0x0020;
pub const TVIF_CHILDREN: ::UINT = 0x0040;
pub const TVIF_INTEGRAL: ::UINT = 0x0080;
// IE 6
pub const TVIF_STATEEX: ::UINT = 0x0100;
pub const TVIF_EXPANDEDIMAGE: ::UINT = 0x0200;

// Tree View control Item States
pub const TVIS_SELECTED: ::UINT = 0x0002;
pub const TVIS_CUT: ::UINT = 0x0004;
pub const TVIS_DROPHILITED: ::UINT = 0x0008;
pub const TVIS_BOLD: ::UINT = 0x0010;
pub const TVIS_EXPANDED: ::UINT = 0x0020;
pub const TVIS_EXPANDEDONCE: ::UINT = 0x0040;
pub const TVIS_EXPANDPARTIAL: ::UINT = 0x0080;

pub const TVIS_OVERLAYMASK: ::UINT = 0x0F00;
pub const TVIS_STATEIMAGEMASK: ::UINT = 0xF000;
pub const TVIS_USERMASK: ::UINT = 0xF000;

// IE 6
pub const TVIS_EX_FLAT: ::UINT = 0x0001;
// Vista
pub const TVIS_EX_DISABLED: ::UINT = 0x0002;

pub const TVIS_EX_ALL: ::UINT = 0x0002;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVITEMEXW {
	pub mask: ::UINT,
	pub hItem: HTREEITEM,
	pub state: ::UINT,
	pub stateMask: ::UINT,
	pub pszText: ::LPCWSTR,
	pub cchTextMax: ::c_int,
	pub iImage: ::c_int,
	pub iSelectedImage: ::c_int,
	pub cChildren: ::c_int,
	pub lParam: ::LPARAM,
	pub iIntegral: ::c_int,
	// IE 6
	pub uStateEx: ::UINT,
	pub hwnd: ::HWND,
	pub iExpandedImage: ::c_int,
	// Win7
	pub iReserved: ::c_int,
}
// make it easier to construct: TVITEMEXW {<interesting fields>, ..Default::default()}
impl ::std::default::Default for TVITEMEXW {
	fn default () -> TVITEMEXW {
		TVITEMEXW {mask:0, hItem: ::std::ptr::null(), state:0, stateMask:0, pszText: ::std::ptr::null_mut(), cchTextMax:0, iImage:0, iSelectedImage:0, cChildren:0, lParam:0, iIntegral:0, uStateEx:0, hwnd: ::std::ptr::null_mut(), iExpandedImage:0, iReserved:0}
	}
}


pub const TVI_ROOT: HTREEITEM = -0x10000 as isize as HTREEITEM;
pub const TVI_FIRST: HTREEITEM = -0x0FFFF as isize as HTREEITEM;
pub const TVI_LAST: HTREEITEM = -0x0FFFE as isize as HTREEITEM;
pub const TVI_SORT: HTREEITEM = -0x0FFFD as isize as HTREEITEM;

//5000
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct TVINSERTSTRUCTW {
	pub hParent: HTREEITEM,
	pub hInsertAfter: HTREEITEM,
	pub itemex: TVITEMEXW,
}
pub const TV_FIRST: ::UINT = 0x1100;
pub const TVM_INSERTITEMA: ::UINT = 0x1100;  // *const TVINSERTSTRUCTA in lParam
pub const TVM_INSERTITEMW: ::UINT = (TV_FIRST + 50);  // *const TVINSERTSTRUCTW in lParam
