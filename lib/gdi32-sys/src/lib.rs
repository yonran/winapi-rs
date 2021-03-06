// Copyright © 2015, Peter Atashian
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to gdi32.
#![cfg(all(windows, any(target_arch = "x86", target_arch = "x86_64")))]
extern crate winapi;
use winapi::*;
extern "system" {
    // pub fn AbortDoc();
    // pub fn AbortPath();
    // pub fn AddFontMemResourceEx();
    // pub fn AddFontResourceA();
    // pub fn AddFontResourceExA();
    // pub fn AddFontResourceExW();
    // pub fn AddFontResourceW();
    pub fn AngleArc(
        hdc: HDC, X: c_int, Y: c_int, dwRadius: DWORD, eStartAngle: FLOAT, eSweepAngle: FLOAT,
    ) -> BOOL;
    // pub fn AnimatePalette();
    pub fn Arc(
        hdc: HDC, nLeftRect: c_int, nTopRect: c_int, nRightRect: c_int, nBottomRect: c_int,
        nXStartArc: c_int, nYStartArc: c_int, nXEndArc: c_int, nYEndArc: c_int,
    ) -> BOOL;
    pub fn ArcTo(
        hdc: HDC, nLeftRect: c_int, nTopRect: c_int, nRightRect: c_int, nBottomRect: c_int,
        nXRadial1: c_int, nYRadial1: c_int, nXRadial2: c_int, nYRadial2: c_int,
    ) -> BOOL;
    // pub fn BeginPath();
    pub fn BitBlt(
        hdc: HDC, x: c_int, y: c_int, cx: c_int, cy: c_int, hdcSrc: HDC, x1: c_int, y1: c_int,
        rop: DWORD,
    ) -> BOOL;
    // pub fn CancelDC();
    // pub fn CheckColorsInGamut();
    pub fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;
    pub fn Chord(
        hdc: HDC, nLeftRect: c_int, nTopRect: c_int, nRightRect: c_int, nBottomRect: c_int,
        nXRadial1: c_int, nYRadial1: c_int, nXRadial2: c_int, nYRadial2: c_int,
    ) -> BOOL;
    // pub fn CloseEnhMetaFile();
    // pub fn CloseFigure();
    // pub fn CloseMetaFile();
    // pub fn ColorCorrectPalette();
    // pub fn ColorMatchToTarget();
    pub fn CombineRgn(
        hrgnDst: HRGN, hrgnSrc1: HRGN, hrgnSrc2: HRGN, fnCombineMode: c_int,
    ) -> c_int;
    // pub fn CombineTransform();
    // pub fn CopyEnhMetaFileA();
    // pub fn CopyEnhMetaFileW();
    // pub fn CopyMetaFileA();
    // pub fn CopyMetaFileW();
    // pub fn CreateBitmap();
    // pub fn CreateBitmapIndirect();
    // pub fn CreateBrushIndirect();
    // pub fn CreateColorSpaceA();
    // pub fn CreateColorSpaceW();
    pub fn CreateCompatibleBitmap(hdc: HDC, cx: c_int, cy: c_int) -> HBITMAP;
    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;
    pub fn CreateDCA(
        lpszDriver: LPCSTR, lpszDevice: LPCSTR, lpszOutput: LPCSTR, lpInitData: *const DEVMODEA,
    ) -> HDC;
    pub fn CreateDCW(
        lpszDriver: LPCWSTR, lpszDevice: LPCWSTR, lpszOutput: LPCWSTR, lpInitData: *const DEVMODEW,
    ) -> HDC;
    // pub fn CreateDIBPatternBrush();
    // pub fn CreateDIBPatternBrushPt();
    // pub fn CreateDIBSection();
    // pub fn CreateDIBitmap();
    // pub fn CreateDiscardableBitmap();
    // pub fn CreateEllipticRgn();
    // pub fn CreateEllipticRgnIndirect();
    // pub fn CreateEnhMetaFileA();
    // pub fn CreateEnhMetaFileW();
    pub fn CreateFontA(
        cHeight: c_int, cWidth: c_int, cEscapement: c_int, cOrientation: c_int, cWeight: c_int,
        bItalic: DWORD, bUnderline: DWORD, bStrikeOut: DWORD, iCharSet: DWORD,
        iOutPrecision: DWORD, iClipPrecision: DWORD, iQuality: DWORD, iPitchAndFamily: DWORD,
        pszFaceName: LPCSTR,
    ) -> HFONT;
    // pub fn CreateFontIndirectA();
    // pub fn CreateFontIndirectExA();
    // pub fn CreateFontIndirectExW();
    // pub fn CreateFontIndirectW();
    pub fn CreateFontW(
        cHeight: c_int, cWidth: c_int, cEscapement: c_int, cOrientation: c_int, cWeight: c_int,
        bItalic: DWORD, bUnderline: DWORD, bStrikeOut: DWORD, iCharSet: DWORD,
        iOutPrecision: DWORD, iClipPrecision: DWORD, iQuality: DWORD, iPitchAndFamily: DWORD,
        pszFaceName: LPCWSTR,
    ) -> HFONT;
    // pub fn CreateHalftonePalette();
    // pub fn CreateHatchBrush();
    // pub fn CreateICA();
    // pub fn CreateICW();
    // pub fn CreateMetaFileA();
    // pub fn CreateMetaFileW();
    // pub fn CreatePalette();
    // pub fn CreatePatternBrush();
    // pub fn CreatePen();
    // pub fn CreatePenIndirect();
    // pub fn CreatePolyPolygonRgn();
    // pub fn CreatePolygonRgn();
    pub fn CreateRectRgn(
        nLeftRect: c_int, nTopRect: c_int, nRightRect: c_int, nBottomRect: c_int,
    ) -> HRGN;
    // pub fn CreateRectRgnIndirect();
    // pub fn CreateRoundRectRgn();
    // pub fn CreateScalableFontResourceA();
    // pub fn CreateScalableFontResourceW();
    pub fn CreateSolidBrush(color: COLORREF) -> HBRUSH;
    // pub fn D3DKMTAcquireKeyedMutex();
    // pub fn D3DKMTAcquireKeyedMutex2();
    // pub fn D3DKMTCacheHybridQueryValue();
    // pub fn D3DKMTCheckExclusiveOwnership();
    // pub fn D3DKMTCheckMonitorPowerState();
    // pub fn D3DKMTCheckMultiPlaneOverlaySupport();
    // pub fn D3DKMTCheckOcclusion();
    // pub fn D3DKMTCheckSharedResourceAccess();
    // pub fn D3DKMTCheckVidPnExclusiveOwnership();
    // pub fn D3DKMTCloseAdapter();
    // pub fn D3DKMTConfigureSharedResource();
    // pub fn D3DKMTCreateAllocation();
    // pub fn D3DKMTCreateAllocation2();
    // pub fn D3DKMTCreateContext();
    // pub fn D3DKMTCreateDCFromMemory();
    // pub fn D3DKMTCreateDevice();
    // pub fn D3DKMTCreateKeyedMutex();
    // pub fn D3DKMTCreateKeyedMutex2();
    // pub fn D3DKMTCreateOutputDupl();
    // pub fn D3DKMTCreateOverlay();
    // pub fn D3DKMTCreateSynchronizationObject();
    // pub fn D3DKMTCreateSynchronizationObject2();
    // pub fn D3DKMTDestroyAllocation();
    // pub fn D3DKMTDestroyContext();
    // pub fn D3DKMTDestroyDCFromMemory();
    // pub fn D3DKMTDestroyDevice();
    // pub fn D3DKMTDestroyKeyedMutex();
    // pub fn D3DKMTDestroyOutputDupl();
    // pub fn D3DKMTDestroyOverlay();
    // pub fn D3DKMTDestroySynchronizationObject();
    // pub fn D3DKMTEnumAdapters();
    // pub fn D3DKMTEscape();
    // pub fn D3DKMTFlipOverlay();
    // pub fn D3DKMTGetCachedHybridQueryValue();
    // pub fn D3DKMTGetContextInProcessSchedulingPriority();
    // pub fn D3DKMTGetContextSchedulingPriority();
    // pub fn D3DKMTGetDeviceState();
    // pub fn D3DKMTGetDisplayModeList();
    // pub fn D3DKMTGetMultisampleMethodList();
    // pub fn D3DKMTGetOverlayState();
    // pub fn D3DKMTGetPresentHistory();
    // pub fn D3DKMTGetPresentQueueEvent();
    // pub fn D3DKMTGetProcessSchedulingPriorityClass();
    // pub fn D3DKMTGetRuntimeData();
    // pub fn D3DKMTGetScanLine();
    // pub fn D3DKMTGetSharedPrimaryHandle();
    // pub fn D3DKMTGetSharedResourceAdapterLuid();
    // pub fn D3DKMTInvalidateActiveVidPn();
    // pub fn D3DKMTLock();
    // pub fn D3DKMTNetDispGetNextChunkInfo();
    // pub fn D3DKMTNetDispQueryMiracastDisplayDeviceStatus();
    // pub fn D3DKMTNetDispQueryMiracastDisplayDeviceSupport();
    // pub fn D3DKMTNetDispStartMiracastDisplayDevice2();
    // pub fn D3DKMTNetDispStopMiracastDisplayDevice();
    // pub fn D3DKMTOfferAllocations();
    // pub fn D3DKMTOpenAdapterFromDeviceName();
    // pub fn D3DKMTOpenAdapterFromGdiDisplayName();
    // pub fn D3DKMTOpenAdapterFromHdc();
    // pub fn D3DKMTOpenAdapterFromLuid();
    // pub fn D3DKMTOpenKeyedMutex();
    // pub fn D3DKMTOpenKeyedMutex2();
    // pub fn D3DKMTOpenNtHandleFromName();
    // pub fn D3DKMTOpenResource();
    // pub fn D3DKMTOpenResource2();
    // pub fn D3DKMTOpenResourceFromNtHandle();
    // pub fn D3DKMTOpenSyncObjectFromNtHandle();
    // pub fn D3DKMTOpenSynchronizationObject();
    // pub fn D3DKMTOutputDuplGetFrameInfo();
    // pub fn D3DKMTOutputDuplGetMetaData();
    // pub fn D3DKMTOutputDuplGetPointerShapeData();
    // pub fn D3DKMTOutputDuplPresent();
    // pub fn D3DKMTOutputDuplReleaseFrame();
    // pub fn D3DKMTPinDirectFlipResources();
    // pub fn D3DKMTPollDisplayChildren();
    // pub fn D3DKMTPresent();
    // pub fn D3DKMTPresentMultiPlaneOverlay();
    // pub fn D3DKMTQueryAdapterInfo();
    // pub fn D3DKMTQueryAllocationResidency();
    // pub fn D3DKMTQueryRemoteVidPnSourceFromGdiDisplayName();
    // pub fn D3DKMTQueryResourceInfo();
    // pub fn D3DKMTQueryResourceInfoFromNtHandle();
    // pub fn D3DKMTQueryStatistics();
    // pub fn D3DKMTReclaimAllocations();
    // pub fn D3DKMTReleaseKeyedMutex();
    // pub fn D3DKMTReleaseKeyedMutex2();
    // pub fn D3DKMTReleaseProcessVidPnSourceOwners();
    // pub fn D3DKMTRender();
    // pub fn D3DKMTSetAllocationPriority();
    // pub fn D3DKMTSetContextInProcessSchedulingPriority();
    // pub fn D3DKMTSetContextSchedulingPriority();
    // pub fn D3DKMTSetDisplayMode();
    // pub fn D3DKMTSetDisplayPrivateDriverFormat();
    // pub fn D3DKMTSetGammaRamp();
    // pub fn D3DKMTSetProcessSchedulingPriorityClass();
    // pub fn D3DKMTSetQueuedLimit();
    // pub fn D3DKMTSetStereoEnabled();
    // pub fn D3DKMTSetVidPnSourceOwner();
    // pub fn D3DKMTSetVidPnSourceOwner1();
    // pub fn D3DKMTShareObjects();
    // pub fn D3DKMTSharedPrimaryLockNotification();
    // pub fn D3DKMTSharedPrimaryUnLockNotification();
    // pub fn D3DKMTSignalSynchronizationObject();
    // pub fn D3DKMTSignalSynchronizationObject2();
    // pub fn D3DKMTUnlock();
    // pub fn D3DKMTUnpinDirectFlipResources();
    // pub fn D3DKMTUpdateOverlay();
    // pub fn D3DKMTWaitForIdle();
    // pub fn D3DKMTWaitForSynchronizationObject();
    // pub fn D3DKMTWaitForSynchronizationObject2();
    // pub fn D3DKMTWaitForVerticalBlankEvent();
    // pub fn D3DKMTWaitForVerticalBlankEvent2();
    // pub fn DPtoLP();
    // pub fn DeleteColorSpace();
    pub fn DeleteDC(hdc: HDC) -> BOOL;
    // pub fn DeleteEnhMetaFile();
    // pub fn DeleteMetaFile();
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;
    pub fn DescribePixelFormat(
        hdc: HDC, iPixelFormat: c_int, nBytes: UINT, ppfd: LPPIXELFORMATDESCRIPTOR,
    ) -> c_int;
    // pub fn DeviceCapabilitiesExA();
    // pub fn DeviceCapabilitiesExW();
    // pub fn DrawEscape();
    pub fn Ellipse(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    // pub fn EnableEUDC();
    // pub fn EndDoc();
    // pub fn EndFormPage();
    // pub fn EndPage();
    // pub fn EndPath();
    // pub fn EnumEnhMetaFile();
    // pub fn EnumFontFamiliesA();
    // pub fn EnumFontFamiliesExA();
    // pub fn EnumFontFamiliesExW();
    // pub fn EnumFontFamiliesW();
    // pub fn EnumFontsA();
    // pub fn EnumFontsW();
    // pub fn EnumICMProfilesA();
    // pub fn EnumICMProfilesW();
    // pub fn EnumMetaFile();
    // pub fn EnumObjects();
    // pub fn EqualRgn();
    // pub fn Escape();
    // pub fn EudcLoadLinkW();
    // pub fn EudcUnloadLinkW();
    // pub fn ExcludeClipRect();
    // pub fn ExtCreatePen();
    // pub fn ExtCreateRegion();
    // pub fn ExtEscape();
    // pub fn ExtFloodFill();
    pub fn ExtSelectClipRgn(hdc: HDC, hrgn: HRGN, mode: c_int) -> c_int;
    // pub fn ExtTextOutA();
    // pub fn ExtTextOutW();
    // pub fn FillPath();
    // pub fn FillRgn();
    // pub fn FixBrushOrgEx();
    // pub fn FlattenPath();
    // pub fn FloodFill();
    // pub fn FrameRgn();
    // pub fn GdiAlphaBlend();
    // pub fn GdiArtificialDecrementDriver();
    // pub fn GdiComment();
    // pub fn GdiDeleteSpoolFileHandle();
    // pub fn GdiEndDocEMF();
    // pub fn GdiEndPageEMF();
    // pub fn GdiFlush();
    // pub fn GdiGetBatchLimit();
    // pub fn GdiGetDC();
    // pub fn GdiGetDevmodeForPage();
    // pub fn GdiGetPageCount();
    // pub fn GdiGetPageHandle();
    // pub fn GdiGetSpoolFileHandle();
    // pub fn GdiGradientFill();
    // pub fn GdiPlayDCScript();
    // pub fn GdiPlayEMF();
    // pub fn GdiPlayJournal();
    // pub fn GdiPlayPageEMF();
    // pub fn GdiPlayPrivatePageEMF();
    // pub fn GdiPlayScript();
    // pub fn GdiResetDCEMF();
    // pub fn GdiSetBatchLimit();
    // pub fn GdiStartDocEMF();
    // pub fn GdiStartPageEMF();
    // pub fn GdiTransparentBlt();
    pub fn GetArcDirection(hdc: HDC) -> c_int;
    // pub fn GetAspectRatioFilterEx();
    // pub fn GetBitmapBits();
    // pub fn GetBitmapDimensionEx();
    // pub fn GetBkColor();
    pub fn GetBkMode() -> c_int;
    // pub fn GetBoundsRect();
    // pub fn GetBrushOrgEx();
    // pub fn GetCharABCWidthsA();
    // pub fn GetCharABCWidthsFloatA();
    // pub fn GetCharABCWidthsFloatW();
    // pub fn GetCharABCWidthsI();
    // pub fn GetCharABCWidthsW();
    // pub fn GetCharWidth32A();
    // pub fn GetCharWidth32W();
    // pub fn GetCharWidthA();
    // pub fn GetCharWidthFloatA();
    // pub fn GetCharWidthFloatW();
    // pub fn GetCharWidthI();
    // pub fn GetCharWidthW();
    // pub fn GetCharacterPlacementA();
    // pub fn GetCharacterPlacementW();
    // pub fn GetClipBox();
    // pub fn GetClipRgn();
    // pub fn GetColorAdjustment();
    // pub fn GetColorSpace();
    // pub fn GetCurrentObject();
    // pub fn GetCurrentPositionEx();
    // pub fn GetDCBrushColor();
    // pub fn GetDCOrgEx();
    // pub fn GetDCPenColor();
    // pub fn GetDIBColorTable();
    // pub fn GetDIBits();
    // pub fn GetDeviceCaps();
    // pub fn GetDeviceGammaRamp();
    // pub fn GetEnhMetaFileA();
    // pub fn GetEnhMetaFileBits();
    // pub fn GetEnhMetaFileDescriptionA();
    // pub fn GetEnhMetaFileDescriptionW();
    // pub fn GetEnhMetaFileHeader();
    // pub fn GetEnhMetaFilePaletteEntries();
    // pub fn GetEnhMetaFilePixelFormat();
    // pub fn GetEnhMetaFileW();
    // pub fn GetFontAssocStatus();
    // pub fn GetFontData();
    // pub fn GetFontLanguageInfo();
    // pub fn GetFontResourceInfoW();
    // pub fn GetFontUnicodeRanges();
    // pub fn GetGlyphIndicesA();
    // pub fn GetGlyphIndicesW();
    // pub fn GetGlyphOutline();
    // pub fn GetGlyphOutlineA();
    // pub fn GetGlyphOutlineW();
    // pub fn GetGraphicsMode();
    // pub fn GetICMProfileA();
    // pub fn GetICMProfileW();
    // pub fn GetKerningPairs();
    // pub fn GetKerningPairsA();
    // pub fn GetKerningPairsW();
    // pub fn GetLayout();
    // pub fn GetLogColorSpaceA();
    // pub fn GetLogColorSpaceW();
    // pub fn GetMapMode();
    // pub fn GetMetaFileA();
    // pub fn GetMetaFileBitsEx();
    // pub fn GetMetaFileW();
    // pub fn GetMetaRgn();
    // pub fn GetMiterLimit();
    // pub fn GetNearestColor();
    // pub fn GetNearestPaletteIndex();
    // pub fn GetObjectA();
    // pub fn GetObjectType();
    // pub fn GetObjectW();
    // pub fn GetOutlineTextMetricsA();
    // pub fn GetOutlineTextMetricsW();
    // pub fn GetPaletteEntries();
    // pub fn GetPath();
    // pub fn GetPixel();
    // pub fn GetPixelFormat();
    // pub fn GetPolyFillMode();
    // pub fn GetROP2();
    // pub fn GetRandomRgn();
    // pub fn GetRasterizerCaps();
    // pub fn GetRegionData();
    // pub fn GetRelAbs();
    // pub fn GetRgnBox();
    pub fn GetStockObject(i: c_int) -> HGDIOBJ;
    // pub fn GetStretchBltMode();
    // pub fn GetSystemPaletteEntries();
    // pub fn GetSystemPaletteUse();
    // pub fn GetTextAlign();
    // pub fn GetTextCharacterExtra();
    // pub fn GetTextCharset();
    // pub fn GetTextCharsetInfo();
    // pub fn GetTextColor();
    // pub fn GetTextExtentExPointA();
    // pub fn GetTextExtentExPointI();
    // pub fn GetTextExtentExPointW();
    // pub fn GetTextExtentPoint32A();
    // pub fn GetTextExtentPoint32W();
    // pub fn GetTextExtentPointA();
    // pub fn GetTextExtentPointI();
    // pub fn GetTextExtentPointW();
    // pub fn GetTextFaceA();
    // pub fn GetTextFaceW();
    // pub fn GetTextMetricsA();
    // pub fn GetTextMetricsW();
    // pub fn GetViewportExtEx();
    // pub fn GetViewportOrgEx();
    // pub fn GetWinMetaFileBits();
    // pub fn GetWindowExtEx();
    // pub fn GetWindowOrgEx();
    // pub fn GetWorldTransform();
    // pub fn IntersectClipRect();
    // pub fn InvertRgn();
    // pub fn LPtoDP();
    pub fn LineDDA(
        nXStart: c_int, nYStart: c_int, nXEnd: c_int, nYEnd: c_int, lpLineFunc: LINEDDAPROC,
        lpData: LPARAM,
    ) -> BOOL;
    pub fn LineTo(hdc: HDC, nXEnd: c_int, nYEnd: c_int);
    // pub fn MaskBlt();
    // pub fn ModifyWorldTransform();
    pub fn MoveToEx(hdc: HDC, X: c_int, Y: c_int, lpPoint:LPPOINT) -> BOOL;
    // pub fn OffsetClipRgn();
    // pub fn OffsetRgn();
    // pub fn OffsetViewportOrgEx();
    // pub fn OffsetWindowOrgEx();
    // pub fn PaintRgn();
    // pub fn PatBlt();
    // pub fn PathToRegion();
    pub fn Pie(
        hdc: HDC, nLeftRect: c_int, nTopRect: c_int, nBottomRect: c_int, nXRadial1: c_int,
        nYRadial1: c_int, nXRadial2: c_int, nYRadial2: c_int,
    ) -> BOOL;
    // pub fn PlayEnhMetaFile();
    // pub fn PlayEnhMetaFileRecord();
    // pub fn PlayMetaFile();
    // pub fn PlayMetaFileRecord();
    // pub fn PlgBlt();
    pub fn PolyBezier(hdc: HDC, lppt: *const POINT, cPoints: DWORD) -> BOOL;
    pub fn PolyBezierTo(hdc: HDC, lppt: *const POINT, cPoints: DWORD) -> BOOL;
    pub fn PolyDraw(hdc: HDC, lppt: *const POINT, lpbTypes: *const BYTE, cCount: c_int) -> BOOL;
    pub fn PolyPolygon(
        hdc: HDC, lpPoints: *const POINT, lpPolyCounts: *const INT, cCount: DWORD,
    ) -> BOOL;
    pub fn PolyPolyline(
        hdc: HDC, lppt: *const POINT, lpdwPolyPoints: *const DWORD, cCount: DWORD,
    ) -> BOOL;
    // pub fn PolyTextOutA();
    // pub fn PolyTextOutW();
    pub fn Polygon(hdc: HDC, lpPoints: *const POINT, nCount: c_int) -> BOOL;
    pub fn Polyline(hdc: HDC, lppt: *const POINT, cCount: c_int) -> BOOL;
    pub fn PolylineTo(hdc: HDC, lppt: *const POINT, cCount: DWORD) -> BOOL;
    // pub fn PtInRegion();
    // pub fn PtVisible();
    // pub fn RealizePalette();
    // pub fn RectInRegion();
    // pub fn RectVisible();
    pub fn Rectangle(hdc: HDC, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    // pub fn RemoveFontMemResourceEx();
    // pub fn RemoveFontResourceA();
    // pub fn RemoveFontResourceExA();
    // pub fn RemoveFontResourceExW();
    // pub fn RemoveFontResourceW();
    // pub fn ResetDCA();
    // pub fn ResetDCW();
    // pub fn ResizePalette();
    // pub fn RestoreDC();
    pub fn RoundRect(
        hdc: HDC, nLeftRect: c_int, nTopRect: c_int, nRightRect: c_int, nBottomRect: c_int,
        nWidth: c_int, nHeight: c_int,
    ) -> BOOL;
    // pub fn SaveDC();
    // pub fn ScaleViewportExtEx();
    // pub fn ScaleWindowExtEx();
    // pub fn SelectBrushLocal();
    // pub fn SelectClipPath();
    pub fn SelectClipRgn(hdc: HDC, hrgn: HRGN) -> c_int;
    // pub fn SelectFontLocal();
    pub fn SelectObject(hdc: HDC, h: HGDIOBJ) -> HGDIOBJ;
    // pub fn SelectPalette();
    // pub fn SetAbortProc();
    pub fn SetArcDirection(hdc: HDC, ArcDirection: c_int) -> c_int;
    // pub fn SetBitmapBits();
    // pub fn SetBitmapDimensionEx();
    pub fn SetBkColor(hdc: HDC, color: COLORREF) -> COLORREF;
    pub fn SetBkMode(hdc: HDC, iBkMode: c_int) -> c_int;
    // pub fn SetBoundsRect();
    // pub fn SetBrushOrgEx();
    // pub fn SetColorAdjustment();
    // pub fn SetColorSpace();
    pub fn SetDCBrushColor(hdc: HDC, color: COLORREF) -> COLORREF;
    // pub fn SetDCPenColor();
    // pub fn SetDIBColorTable();
    // pub fn SetDIBits();
    // pub fn SetDIBitsToDevice();
    // pub fn SetDeviceGammaRamp();
    // pub fn SetEnhMetaFileBits();
    // pub fn SetFontEnumeration();
    // pub fn SetGraphicsMode();
    // pub fn SetICMMode();
    // pub fn SetICMProfileA();
    // pub fn SetICMProfileW();
    // pub fn SetLayout();
    // pub fn SetMagicColors();
    // pub fn SetMapMode();
    // pub fn SetMapperFlags();
    // pub fn SetMetaFileBitsEx();
    // pub fn SetMetaRgn();
    // pub fn SetMiterLimit();
    // pub fn SetPaletteEntries();
    // pub fn SetPixel();
    pub fn SetPixelFormat(
        hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR,
    ) -> BOOL;
    // pub fn SetPixelV();
    // pub fn SetPolyFillMode();
    // pub fn SetROP2();
    pub fn SetRectRgn(hrgn: HRGN, left: c_int, top: c_int, right: c_int, bottom: c_int) -> BOOL;
    // pub fn SetRelAbs();
    // pub fn SetStretchBltMode();
    // pub fn SetSystemPaletteUse();
    // pub fn SetTextAlign();
    // pub fn SetTextCharacterExtra();
    pub fn SetTextColor(hdc: HDC, color: COLORREF) -> COLORREF;
    // pub fn SetTextJustification();
    // pub fn SetViewportExtEx();
    // pub fn SetViewportOrgEx();
    // pub fn SetWinMetaFileBits();
    // pub fn SetWindowExtEx();
    // pub fn SetWindowOrgEx();
    // pub fn SetWorldTransform();
    // pub fn StartDocA();
    // pub fn StartDocW();
    // pub fn StartFormPage();
    // pub fn StartPage();
    // pub fn StretchBlt();
    // pub fn StretchDIBits();
    // pub fn StrokeAndFillPath();
    // pub fn StrokePath();
    pub fn SwapBuffers(hdc: HDC) -> BOOL;
    pub fn TextOutA(hdc: HDC, x: c_int, y: c_int, lpString: LPCSTR, c: c_int) -> BOOL;
    pub fn TextOutW(hdc: HDC, x: c_int, y: c_int, lpString: LPCWSTR, c: c_int) -> BOOL;
    // pub fn TranslateCharsetInfo();
    // pub fn UnrealizeObject();
    // pub fn UpdateColors();
    // pub fn UpdateICMRegKeyA();
    // pub fn UpdateICMRegKeyW();
    // pub fn WidenPath();
    // pub fn gdiPlaySpoolStream();
}
