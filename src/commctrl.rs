
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

//330
// WM_NOTIFY codes (::c_int.code)
pub const NM_FIRST:   ::UINT = 0; // generic to all controls
pub const NM_LAST:    ::UINT = 0xffffff9d;  // 0-99
pub const LVN_FIRST:  ::UINT = 0xffffff9c;  // 0-100 listview
pub const LVN_LAST:   ::UINT = 0xffffff39;  // 0-199
// Property sheet reserved      (0-200) -  (0-299) - see prsht.h
pub const HDN_FIRST:  ::UINT = 0xfffffed4;  // 0-300 header
pub const HDN_LAST:   ::UINT = 0xfffffe71;  // 0-399
pub const TVN_FIRST:  ::UINT = 0xfffffe70;  // 0-400 treeview

pub const TVN_LAST:   ::UINT = 0xfffffe0d;  // 0-499
pub const TTN_FIRST:  ::UINT = 0xfffffdf8;  // 0-520 tooltips
pub const TTN_LAST:   ::UINT = 0xfffffddb;  // 0-549
pub const TCN_FIRST:  ::UINT = 0xfffffdda;  // 0-550 tab control
pub const TCN_LAST:   ::UINT = 0xfffffdbc;  // 0-580
// Shell reserved               (0-580) - (0-589)
pub const CDN_FIRST:  ::UINT = 0xfffffda7;  // 0-601 common dialog (new)
pub const CDN_LAST:   ::UINT = 0xfffffd45;  // 0-699
pub const TBN_FIRST:  ::UINT = 0xfffffd44;  // 0-700 toolbar
pub const TBN_LAST:   ::UINT = 0xfffffd30;  // 0-720
pub const UDN_FIRST:  ::UINT = 0xfffffd2f;  // 0-721 updown
pub const UDN_LAST:   ::UINT = 0xfffffd27;  // 0-729
pub const DTN_FIRST:  ::UINT = 0xfffffd1c;  // 0-740 datetimepick
pub const DTN_LAST:   ::UINT = 0xfffffd17;  // 0-745 DTN_FIRST - 5
pub const MCN_FIRST:  ::UINT = 0xfffffd16;  // 0-746 monthcal
pub const MCN_LAST:   ::UINT = 0xfffffd10;  // 0-752 MCN_FIRST - 6
pub const DTN_FIRST2: ::UINT = 0xfffffd0f;  // 0-753 datetimepick2
pub const DTN_LAST2:  ::UINT = 0xfffffce1;  // 0-799
pub const CBEN_FIRST: ::UINT = 0xfffffce0;  // 0-800 combo box ex
pub const CBEN_LAST:  ::UINT = 0xfffffcc2;  // 0-830
pub const RBN_FIRST:  ::UINT = 0xfffffcc1;  // 0-831 rebar
pub const RBN_LAST:   ::UINT = 0xfffffca5;  // 0-859
pub const IPN_FIRST:  ::UINT = 0xfffffca4;  // 0-860 internet address
pub const IPN_LAST:   ::UINT = 0xfffffc91;  // 0-879
pub const SBN_FIRST:  ::UINT = 0xfffffc90;  // 0-880 status bar
pub const SBN_LAST:   ::UINT = 0xfffffc7d;  // 0-899
pub const PGN_FIRST:  ::UINT = 0xfffffc7c;  // 0-900 Pager Control
pub const PGN_LAST:   ::UINT = 0xfffffc4a;  // 0-950
pub const WMN_FIRST:  ::UINT = 0xfffffc18;  // 0-1000
pub const WMN_LAST:   ::UINT = 0xfffffb50;  // 0-1200
pub const BCN_FIRST:  ::UINT = 0xfffffb1e;  // 0-1250
pub const BCN_LAST:   ::UINT = 0xfffffaba;  // 0-1350
pub const TRBN_FIRST: ::UINT = 0xfffffa23;  // 0-1501 trackbar
pub const TRBN_LAST:  ::UINT = 0xfffffa11;  // 0-1519
pub const MSGF_COMMCTRL_BEGINDRAG: ::UINT = 0x4200;
pub const MSGF_COMMCTRL_SIZEHEADER: ::UINT = 0x4201;
pub const MSGF_COMMCTRL_DRAGSELECT: ::UINT = 0x4202;
pub const MSGF_COMMCTRL_TOOLBARCUST: ::UINT = 0x4203;


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
//2989
pub const MAX_LINKID_TEXT: usize = 48;
pub const L_MAX_URL_LENGTH: usize = (2048 + 32 + 3 /*sizeof("://") */);

//3028
#[repr(C)] #[derive(Copy)]
pub struct LITEM {
    pub mask: ::UINT,
    pub iLink: ::c_int,
    pub state: ::UINT,
    pub stateMask: ::UINT,
    pub szID: [::WCHAR; MAX_LINKID_TEXT],
    pub szUrl: [::WCHAR; L_MAX_URL_LENGTH],
}
pub type PLITEM = *mut LITEM;
impl ::std::clone::Clone for LITEM {
    fn clone(&self) -> Self {
        let mut clone = LITEM {
            mask: self.mask,
            iLink: self.iLink,
            state: self.state,
            stateMask: self.stateMask,
            szID: [0; MAX_LINKID_TEXT],
            szUrl: [0; L_MAX_URL_LENGTH],
        };
        for i in 0..clone.szID.len() {
            clone.szID[i] = self.szID[i];
        }
        for i in 0..clone.szUrl.len() {
            clone.szUrl[i] = self.szUrl[i];
        }
        clone
    }
}
impl ::std::fmt::Debug for LITEM {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(f.write_fmt(format_args!("LVITEM {{ mask: {}, iLink: {}, state: {}, stateMask: {}, szID: {:?}, szUrl: {:?} }}",
            self.mask, self.iLink, self.state, self.stateMask, &self.szID[..], &self.szUrl[..])));
        Ok(())
    }
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LHITTESTINFO {
    pub pt: ::POINT,
    pub item: LITEM,
}
pub type PLHITTESTINFO = *mut LHITTESTINFO;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLINK {
    pub hdr: ::NMHDR,
    pub item: LITEM,
}
pub type PNMLINK = *mut NMLINK;

//3078
// List View Styles (dwStyle param of CreateWindow/CreateWindowEx)
pub const LVS_ICON: ::DWORD = 0x0000;
pub const LVS_REPORT: ::DWORD = 0x0001;
pub const LVS_SMALLICON: ::DWORD = 0x0002;
pub const LVS_LIST: ::DWORD = 0x0003;
pub const LVS_TYPEMASK: ::DWORD = 0x0003;
pub const LVS_SINGLESEL: ::DWORD = 0x0004;
pub const LVS_SHOWSELALWAYS: ::DWORD = 0x0008;
pub const LVS_SORTASCENDING: ::DWORD = 0x0010;
pub const LVS_SORTDESCENDING: ::DWORD = 0x0020;
pub const LVS_SHAREIMAGELISTS: ::DWORD = 0x0040;
pub const LVS_NOLABELWRAP: ::DWORD = 0x0080;
pub const LVS_AUTOARRANGE: ::DWORD = 0x0100;
pub const LVS_EDITLABELS: ::DWORD = 0x0200;
pub const LVS_OWNERDATA: ::DWORD = 0x1000;
pub const LVS_NOSCROLL: ::DWORD = 0x2000;

pub const LVS_TYPESTYLEMASK: ::DWORD = 0xfc00;

pub const LVS_ALIGNTOP: ::DWORD = 0x0000;
pub const LVS_ALIGNLEFT: ::DWORD = 0x0800;
pub const LVS_ALIGNMASK: ::DWORD = 0x0c00;

pub const LVS_OWNERDRAWFIXED: ::DWORD = 0x0400;
pub const LVS_NOCOLUMNHEADER: ::DWORD = 0x4000;
pub const LVS_NOSORTHEADER: ::DWORD = 0x8000;

//3140
// List View Item Flags: flags for LVITEMW::mask
pub const LVIF_TEXT: ::UINT = 0x00000001;
pub const LVIF_IMAGE: ::UINT = 0x00000002;
pub const LVIF_PARAM: ::UINT = 0x00000004;
pub const LVIF_STATE: ::UINT = 0x00000008;
pub const LVIF_INDENT: ::UINT = 0x00000010;
pub const LVIF_NORECOMPUTE: ::UINT = 0x00000800;
// xp
pub const LVIF_GROUPID: ::UINT = 0x00000100;
pub const LVIF_COLUMNS: ::UINT = 0x00000200;
// vista
pub const LVIF_COLFMT: ::UINT = 0x00010000; // The piColFmt member is valid in addition to puColumns

// List View Item State: bits for LVITEMW::state
pub const LVIS_FOCUSED: ::UINT = 0x0001;
pub const LVIS_SELECTED: ::UINT = 0x0002;
pub const LVIS_CUT: ::UINT = 0x0004;
pub const LVIS_DROPHILITED: ::UINT = 0x0008;
pub const LVIS_GLOW: ::UINT = 0x0010;
pub const LVIS_ACTIVATING: ::UINT = 0x0020;

pub const LVIS_OVERLAYMASK: ::UINT = 0x0F00;
pub const LVIS_STATEIMAGEMASK: ::UINT = 0xF000;

//3191
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVITEMA {
	pub mask: ::UINT,
	pub iItem: ::c_int,
	pub iSubItem: ::c_int,
	pub state: ::UINT,
	pub stateMask: ::UINT,
	pub pszText: ::LPSTR,
	pub cchTextMax: ::c_int,
	pub iImage: ::c_int,
	pub lParam: ::LPARAM,
	pub iIndent: ::c_int,
	// XP
	pub iGroupId: ::c_int,
	pub cColumns: ::UINT, // tile view columns
	pub puColumns: ::PUINT,
	// Vista
	pub piColFmt: *mut ::c_int,
	pub iGroup: ::c_int, // readonly. only valid for owner data.
}
pub type LPLVITEMA = *mut LVITEMA;
// make it easier to construct: LVITEMA {<interesting fields>, ..Default::default()}
impl ::std::default::Default for LVITEMA {
	fn default () -> LVITEMA {
		LVITEMA {
			mask:0, iItem: 0, iSubItem:0, state:0, stateMask:0, pszText: ::std::ptr::null_mut(), cchTextMax:0, iImage:0, lParam:0, iIndent:0, iGroupId:0, cColumns:0, puColumns: ::std::ptr::null_mut(), piColFmt: ::std::ptr::null_mut(), iGroup:0
		}
	}
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVITEMW {
	pub mask: ::UINT,
	pub iItem: ::c_int,
	pub iSubItem: ::c_int,
	pub state: ::UINT,
	pub stateMask: ::UINT,
	pub pszText: ::LPWSTR,
	pub cchTextMax: ::c_int,
	pub iImage: ::c_int,
	pub lParam: ::LPARAM,
	pub iIndent: ::c_int,
	// XP
	pub iGroupId: ::c_int,
	pub cColumns: ::UINT, // tile view columns
	pub puColumns: ::PUINT,
	// Vista
	pub piColFmt: *mut ::c_int,
	pub iGroup: ::c_int, // readonly. only valid for owner data.
}
pub type LPLVITEMW = *mut LVITEMW;
// make it easier to construct: LVITEMW {<interesting fields>, ..Default::default()}
impl ::std::default::Default for LVITEMW {
	fn default () -> LVITEMW {
		LVITEMW {
			mask:0, iItem: 0, iSubItem:0, state:0, stateMask:0, pszText: ::std::ptr::null_mut(), cchTextMax:0, iImage:0, lParam:0, iIndent:0, iGroupId:0, cColumns:0, puColumns: ::std::ptr::null_mut(), piColFmt: ::std::ptr::null_mut(), iGroup:0
		}
	}
}

//3518
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVCOLUMNA {
	pub mask: ::UINT,
	pub fmt: ::c_int,
	pub cx: ::c_int,
	pub pszText: ::LPSTR,
	pub cchTextMax: ::c_int,
	pub iSubItem: ::c_int,
	pub iImage: ::c_int,
	pub iOrder: ::c_int,
	// vista
	pub cxMin: ::c_int,       // min snap point
	pub cxDefault: ::c_int,   // default snap point
	pub cxIdeal: ::c_int,     // read only. ideal may not eqaul current width if auto sized (LVS_EX_AUTOSIZECOLUMNS) to a lesser width.
}
pub type LPLVCOLUMNA = *mut LVCOLUMNA;
// make it easier to construct: LVCOLUMNA {<interesting fields>, ..Default::default()}
impl ::std::default::Default for LVCOLUMNA {
	fn default () -> LVCOLUMNA {
		LVCOLUMNA {mask:0, fmt:0, cx:0, pszText: ::std::ptr::null_mut(),cchTextMax:0, iSubItem:0, iImage:0, iOrder:0, cxMin:0, cxDefault:0, cxIdeal:0}
	}
}

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct LVCOLUMNW {
	pub mask: ::UINT,
	pub fmt: ::c_int,
	pub cx: ::c_int,
	pub pszText: ::LPWSTR,
	pub cchTextMax: ::c_int,
	pub iSubItem: ::c_int,
	pub iImage: ::c_int,
	pub iOrder: ::c_int,
	// vista
	pub cxMin: ::c_int,       // min snap point
	pub cxDefault: ::c_int,   // default snap point
	pub cxIdeal: ::c_int,     // read only. ideal may not eqaul current width if auto sized (LVS_EX_AUTOSIZECOLUMNS) to a lesser width.
}
pub type LPLVCOLUMNW = *mut LVCOLUMNW;
// make it easier to construct: LVCOLUMNW {<interesting fields>, ..Default::default()}
impl ::std::default::Default for LVCOLUMNW {
	fn default () -> LVCOLUMNW {
		LVCOLUMNW {mask:0, fmt:0, cx:0, pszText: ::std::ptr::null_mut(),cchTextMax:0, iSubItem:0, iImage:0, iOrder:0, cxMin:0, cxDefault:0, cxIdeal:0}
	}
}


pub const LVM_FIRST: ::UINT = 0x1000;      // ListView messages
pub const LVM_GETITEMCOUNT: ::UINT = (LVM_FIRST + 4);  // returns c_int
pub const LVM_GETITEMA: ::UINT = (LVM_FIRST + 5);  // *mut LVITEMA in lParam
pub const LVM_GETITEMW: ::UINT = (LVM_FIRST + 75);  // *mut LVITEMW in lParam
pub const LVM_SETITEMA: ::UINT = (LVM_FIRST + 6);  // *const LVITEMA in lParam
pub const LVM_SETITEMW: ::UINT = (LVM_FIRST + 76);  // *const LVITEMW in lParam
pub const LVM_INSERTITEMA: ::UINT = (LVM_FIRST + 7);
pub const LVM_INSERTITEMW: ::UINT = (LVM_FIRST + 77);
pub const LVM_DELETEITEM: ::UINT = (LVM_FIRST + 8);
pub const LVM_DELETEALLITEMS: ::UINT = (LVM_FIRST + 9);
pub const LVM_GETCALLBACKMASK: ::UINT = (LVM_FIRST + 10);
pub const LVM_SETCALLBACKMASK: ::UINT = (LVM_FIRST + 11);

//3338
pub const LVM_GETNEXTITEM: ::UINT = (LVM_FIRST + 12);
//3378
pub const LVM_FINDITEMA: ::UINT = (LVM_FIRST + 13);
pub const LVM_FINDITEMW: ::UINT = (LVM_FIRST + 83);
//3395
pub const LVM_GETITEMRECT: ::UINT = (LVM_FIRST + 14);
pub const LVM_SETITEMPOSITION: ::UINT = (LVM_FIRST + 15);
pub const LVM_GETITEMPOSITION: ::UINT = (LVM_FIRST + 16);
pub const LVM_GETSTRINGWIDTHA: ::UINT = (LVM_FIRST + 17);
pub const LVM_GETSTRINGWIDTHW: ::UINT = (LVM_FIRST + 87);
//3461
pub const LVM_HITTEST: ::UINT = (LVM_FIRST + 18);
pub const LVM_ENSUREVISIBLE: ::UINT = (LVM_FIRST + 19);
pub const LVM_SCROLL: ::UINT = (LVM_FIRST + 20);
pub const LVM_REDRAWITEMS: ::UINT = (LVM_FIRST + 21);

//3489
pub const LVM_ARRANGE: ::UINT = (LVM_FIRST + 22);
pub const LVM_EDITLABELA: ::UINT = (LVM_FIRST + 23);
pub const LVM_EDITLABELW: ::UINT = (LVM_FIRST + 118);
pub const LVM_GETEDITCONTROL: ::UINT = (LVM_FIRST + 24);

//3563
// List View Column Flags in LVCOLUMN::mask
pub const LVCF_FMT: ::UINT = 0x0001;
pub const LVCF_WIDTH: ::UINT = 0x0002;
pub const LVCF_TEXT: ::UINT = 0x0004;
pub const LVCF_SUBITEM: ::UINT = 0x0008;
pub const LVCF_IMAGE: ::UINT = 0x0010;
pub const LVCF_ORDER: ::UINT = 0x0020;
// Vista
pub const LVCF_MINWIDTH: ::UINT = 0x0040;
pub const LVCF_DEFAULTWIDTH: ::UINT = 0x0080;
pub const LVCF_IDEALWIDTH: ::UINT = 0x0100;


// LVCFMT_ flags up to FFFF are shared with the header control (HDF_ flags).
// Flags above FFFF are listview-specific.
// List View Column Formats in LVCOLUMN::fmt
pub const LVCFMT_LEFT: ::c_int = 0x0000; // Same as HDF_LEFT
pub const LVCFMT_RIGHT: ::c_int = 0x0001; // Same as HDF_RIGHT
pub const LVCFMT_CENTER: ::c_int = 0x0002; // Same as HDF_CENTER
pub const LVCFMT_JUSTIFYMASK: ::c_int = 0x0003; // Same as HDF_JUSTIFYMASK

pub const LVCFMT_IMAGE: ::c_int = 0x0800; // Same as HDF_IMAGE
pub const LVCFMT_BITMAP_ON_RIGHT: ::c_int = 0x1000; // Same as HDF_BITMAP_ON_RIGHT
pub const LVCFMT_COL_HAS_IMAGES: ::c_int = 0x8000; // Same as HDF_OWNERDRAW

// Vista
pub const LVCFMT_FIXED_WIDTH: ::c_int = 0x00100; // Can't resize the column; same as HDF_FIXEDWIDTH
pub const LVCFMT_NO_DPI_SCALE: ::c_int = 0x40000; // If not set, CCM_DPISCALE will govern scaling up fixed width
pub const LVCFMT_FIXED_RATIO: ::c_int = 0x80000; // Width will augment with the row height

// The following flags
pub const LVCFMT_LINE_BREAK: ::c_int = 0x100000; // Move to the top of the next list of columns
pub const LVCFMT_FILL: ::c_int = 0x200000; // Fill the remainder of the tile area. Might have a title.
pub const LVCFMT_WRAP: ::c_int = 0x400000; // This sub-item can be wrapped.
pub const LVCFMT_NO_TITLE: ::c_int = 0x800000; // This sub-item doesn't have an title.
pub const LVCFMT_TILE_PLACEMENTMASK: ::c_int = (LVCFMT_LINE_BREAK | LVCFMT_FILL);

pub const LVCFMT_SPLITBUTTON: ::c_int = 0x1000000; // Column is a split button; same as HDF_SPLITBUTTON


//3602
pub const LVM_GETCOLUMNA: ::UINT = (LVM_FIRST + 25);  // iCol in wParam, *mut LVCOLUMNA in lParam
pub const LVM_GETCOLUMNW: ::UINT = (LVM_FIRST + 95);  // iCol in wParam, *mut LVCOLUMNW in lParam
pub const LVM_SETCOLUMNA: ::UINT = (LVM_FIRST + 26);  // iCol in wParam, *const LVCOLUMNA in lParam
pub const LVM_SETCOLUMNW: ::UINT = (LVM_FIRST + 96);  // iCol in wParam, *const LVCOLUMNW in lParam
pub const LVM_INSERTCOLUMNA: ::UINT = (LVM_FIRST + 27);  // iCol in wParam, *const LVCOLUMNA in lParam
pub const LVM_INSERTCOLUMNW: ::UINT = (LVM_FIRST + 97);  // iCol in wParam, *const LVCOLUMNW in lParam
pub const LVM_DELETECOLUMN: ::UINT = (LVM_FIRST + 28);  // iCol in wParam
pub const LVM_GETCOLUMNWIDTH: ::UINT = (LVM_FIRST + 29);  // iCol in wParam

//4433
pub struct NMLISTVIEW {
    pub hdr: ::NMHDR,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub uNewState: ::UINT,
    pub uOldState: ::UINT,
    pub uChanged: ::UINT,
    pub ptAction: ::POINT,
    pub lParam: ::LPARAM,
}
pub type LPNMLISTVIEW = *mut NMLISTVIEW;
// legacy name pub type LPNM_LISTVIEW = LPNMLISTVIEW;
// legacy name pub type NM_LISTVIEW = NMLISTVIEW;


//4551
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVODSTATECHANGE {
    pub hdr: ::NMHDR,
    pub iFrom: ::c_int,
    pub iTo: ::c_int,
    pub uNewState: ::UINT,
    pub uOldState: ::UINT,
}
pub type LPNMLVODSTATECHANGE = *mut NMLVODSTATECHANGE;
// legacy name pub type PNM_ODSTATECHANGE = LPNMLVODSTATECHANGE;
// legacy name pub type LPNM_ODSTATECHANGE = LPNMLVODSTATECHANGE;
// legacy name pub type NM_ODSTATECHANGE = NMLVODSTATECHANGE;

//4565
pub const LVN_ITEMCHANGING: ::UINT = (LVN_FIRST-0);
pub const LVN_ITEMCHANGED: ::UINT = (LVN_FIRST-1);
pub const LVN_INSERTITEM: ::UINT = (LVN_FIRST-2);
pub const LVN_DELETEITEM: ::UINT = (LVN_FIRST-3);
pub const LVN_DELETEALLITEMS: ::UINT = (LVN_FIRST-4);
pub const LVN_BEGINLABELEDITA: ::UINT = (LVN_FIRST-5);
pub const LVN_BEGINLABELEDITW: ::UINT = (LVN_FIRST-75);
pub const LVN_ENDLABELEDITA: ::UINT = (LVN_FIRST-6);
pub const LVN_ENDLABELEDITW: ::UINT = (LVN_FIRST-76);
pub const LVN_COLUMNCLICK: ::UINT = (LVN_FIRST-8);
pub const LVN_BEGINDRAG: ::UINT = (LVN_FIRST-9);
pub const LVN_BEGINRDRAG: ::UINT = (LVN_FIRST-11);

pub const LVN_ODCACHEHINT: ::UINT = (LVN_FIRST-13);
pub const LVN_ODFINDITEMA: ::UINT = (LVN_FIRST-52);
pub const LVN_ODFINDITEMW: ::UINT = (LVN_FIRST-79);

pub const LVN_ITEMACTIVATE: ::UINT = (LVN_FIRST-14);
pub const LVN_ODSTATECHANGED: ::UINT = (LVN_FIRST-15);
pub const LVN_HOTTRACK: ::UINT = (LVN_FIRST-21);

pub const LVN_GETDISPINFOA: ::UINT = (LVN_FIRST-50);
pub const LVN_GETDISPINFOW: ::UINT = (LVN_FIRST-77);
pub const LVN_SETDISPINFOA: ::UINT = (LVN_FIRST-51);
pub const LVN_SETDISPINFOW: ::UINT = (LVN_FIRST-78);
// LVITEM::mask
pub const LVIF_DI_SETITEM: ::UINT = 0x1000;

//4619
#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVDISPINFOA {
    pub hdr: ::NMHDR,
    pub item: LVITEMA,
}
pub type LPNMLVDISPINFOA = *mut NMLVDISPINFOA;
// legacy name pub type LV_DISPINFOA = NMLVDISPINFOA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVDISPINFOW {
    pub hdr: ::NMHDR,
    pub item: LVITEMW,
}
pub type LPNMLVDISPINFOW = *mut NMLVDISPINFOW;
// legacy name pub type LV_DISPINFOW = NMLVDISPINFOW;
pub const LVN_KEYDOWN: ::UINT = (LVN_FIRST-55);

#[repr(C, packed)] #[derive(Clone, Copy, Debug)]
pub struct NMLVKEYDOWN {
    pub hdr: ::NMHDR,
    pub wVKey: ::WORD,
    pub flags: ::UINT,
}
pub type LPNMLVKEYDOWN = *mut NMLVKEYDOWN;
// legacy name pub type LV_KEYDOWN = NMLVKEYDOWN;

pub const LVN_MARQUEEBEGIN: ::UINT = (LVN_FIRST-56);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVLINK {
    pub hdr: ::NMHDR,
    pub link: LITEM,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
}
pub type PNMLVLINK = *mut NMLVLINK;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVGETINFOTIPA {

    pub hdr: ::NMHDR,
    pub dwFlags: ::DWORD,
    pub pszText: ::LPSTR,
    pub cchTextMax: ::c_int,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPNMLVGETINFOTIPA = *mut NMLVGETINFOTIPA;

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVGETINFOTIPW {

    pub hdr: ::NMHDR,
    pub dwFlags: ::DWORD,
    pub pszText: ::LPWSTR,
    pub cchTextMax: ::c_int,
    pub iItem: ::c_int,
    pub iSubItem: ::c_int,
    pub lParam: ::LPARAM,
}
pub type LPNMLVGETINFOTIPW = *mut NMLVGETINFOTIPW;

// NMLVGETINFOTIPA.dwFlag values
pub const LVGIT_UNFOLDED: ::UINT = 0x0001;

pub const LVN_GETINFOTIPA: ::UINT = (LVN_FIRST-57);
pub const LVN_GETINFOTIPW: ::UINT = (LVN_FIRST-58);

//4707
//  LVN_INCREMENTALSEARCH gives the app the opportunity to customize
//  incremental search.  For example, if the items are numeric,
//  the app can do numerical search instead of string search.
//
//  ListView notifies the app with NMLVFINDITEM.
//  The app sets pnmfi->lvfi.lParam to the result of the incremental search,
//  or to LVNSCH_DEFAULT if ListView should do the default search,
//  or to LVNSCH_ERROR to fail the search and just beep,
//  or to LVNSCH_IGNORE to stop all ListView processing.
//
//  The return value is not used.

pub const LVNSCH_DEFAULT: ::LPARAM = -1;
pub const LVNSCH_ERROR: ::LPARAM = -2;
pub const LVNSCH_IGNORE: ::LPARAM = -3;

pub const LVN_INCREMENTALSEARCHA: ::UINT = (LVN_FIRST-62);
pub const LVN_INCREMENTALSEARCHW: ::UINT = (LVN_FIRST-63);

pub const LVN_COLUMNDROPDOWN: ::UINT = (LVN_FIRST-64);
pub const LVN_COLUMNOVERFLOWCLICK: ::UINT = (LVN_FIRST-66);

#[repr(C)] #[derive(Clone, Copy, Debug)]
pub struct NMLVSCROLL {
    pub hdr: ::NMHDR,
    pub dx: ::c_int,
    pub dy: ::c_int,
}
pub type LPNMLVSCROLL = *mut NMLVSCROLL;

pub const LVN_BEGINSCROLL: ::UINT = (LVN_FIRST-80);
pub const LVN_ENDSCROLL: ::UINT = (LVN_FIRST-81);

pub const LVN_LINKCLICK: ::UINT = (LVN_FIRST-84);
pub const EMF_CENTERED: ::UINT = 0x00000001;  // render markup centered in the listview area

#[repr(C)] #[derive(Copy)]
pub struct NMLVEMPTYMARKUP {
    pub hdr: ::NMHDR,
    // out params from client back to listview
    pub dwFlags: ::DWORD,                      // EMF_*
    pub szMarkup: [::WCHAR; L_MAX_URL_LENGTH],   // markup displayed
}
impl ::std::clone::Clone for NMLVEMPTYMARKUP {
    fn clone(&self) -> Self {
        let mut clone = NMLVEMPTYMARKUP {
            hdr: self.hdr,
            dwFlags: self.dwFlags,
            szMarkup: [0; L_MAX_URL_LENGTH],
        };
        for i in 0..clone.szMarkup.len() {
            clone.szMarkup[i] = self.szMarkup[i];
        }
        clone
    }
}
impl ::std::fmt::Debug for NMLVEMPTYMARKUP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        try!(f.write_fmt(format_args!("NMLVEMPTYMARKUP {{ hdr: {:?}, dwFlags: {:?}, szMarkup: {:?} }}",
            self.hdr, self.dwFlags, &self.szMarkup as &[::WCHAR])));
        Ok(())
    }
}

pub const LVN_GETEMPTYMARKUP: ::UINT = (LVN_FIRST-87);


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
