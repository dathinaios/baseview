use std::cell::Cell;

use cocoa::base::{id, nil};
use objc::{class, msg_send, sel, sel_impl};

use crate::MouseCursor;

pub(super) unsafe fn set_cursor(cursor: MouseCursor, cursor_hidden: &Cell<bool>) {
    debug_assert!({
        let is_main: bool = msg_send![class!(NSThread), isMainThread];
        is_main
    });

    let should_hide = matches!(cursor, MouseCursor::Hidden);

    if should_hide {
        if !cursor_hidden.get() {
            let transparent = transparent_cursor();
            let _: () = msg_send![transparent, set];
            cursor_hidden.set(true);
        }
        return;
    }

    cursor_hidden.set(false);

    let ns_cursor = to_ns_cursor(cursor);
    let _: () = msg_send![ns_cursor, set];
}

pub(super) unsafe fn to_ns_cursor(cursor: MouseCursor) -> id {
    match cursor {
        MouseCursor::Default
        | MouseCursor::Move
        | MouseCursor::AllScroll
        | MouseCursor::Cell => {
            msg_send![class!(NSCursor), arrowCursor]
        }

        MouseCursor::Hand => {
            msg_send![class!(NSCursor), openHandCursor]
        }

        MouseCursor::HandGrabbing => {
            msg_send![class!(NSCursor), closedHandCursor]
        }

        MouseCursor::Text => {
            msg_send![class!(NSCursor), IBeamCursor]
        }

        MouseCursor::VerticalText => {
            msg_send![class!(NSCursor), IBeamCursorForVerticalLayout]
        }

        MouseCursor::Copy => {
            msg_send![class!(NSCursor), dragCopyCursor]
        }

        MouseCursor::Alias => {
            msg_send![class!(NSCursor), dragLinkCursor]
        }

        MouseCursor::NotAllowed | MouseCursor::PtrNotAllowed => {
            msg_send![class!(NSCursor), operationNotAllowedCursor]
        }

        MouseCursor::Crosshair => {
            msg_send![class!(NSCursor), crosshairCursor]
        }

        MouseCursor::EResize => {
            msg_send![class!(NSCursor), resizeRightCursor]
        }

        MouseCursor::NResize => {
            msg_send![class!(NSCursor), resizeUpCursor]
        }

        MouseCursor::WResize => {
            msg_send![class!(NSCursor), resizeLeftCursor]
        }

        MouseCursor::SResize => {
            msg_send![class!(NSCursor), resizeDownCursor]
        }

        MouseCursor::EwResize | MouseCursor::ColResize => {
            msg_send![class!(NSCursor), resizeLeftRightCursor]
        }

        MouseCursor::NsResize | MouseCursor::RowResize => {
            msg_send![class!(NSCursor), resizeUpDownCursor]
        }

        // No public equivalents — fall back safely
        MouseCursor::Help
        | MouseCursor::ZoomIn
        | MouseCursor::ZoomOut
        | MouseCursor::NeResize
        | MouseCursor::NwResize
        | MouseCursor::SeResize
        | MouseCursor::SwResize
        | MouseCursor::NeswResize
        | MouseCursor::NwseResize
        | MouseCursor::Working
        | MouseCursor::PtrWorking => {
            msg_send![class!(NSCursor), arrowCursor]
        }

        MouseCursor::Hidden => unreachable!(),
    }
}

unsafe fn transparent_cursor() -> id {
    use cocoa::foundation::{NSPoint, NSSize};
    use cocoa::appkit::NSImage;

    let size = NSSize::new(1.0, 1.0);
    let image: id = msg_send![class!(NSImage), alloc];
    let image: id = msg_send![image, initWithSize: size];

    let hotspot = NSPoint::new(0.0, 0.0);

    let cursor: id = msg_send![class!(NSCursor), alloc];
    msg_send![cursor, initWithImage: image hotSpot: hotspot]
}
