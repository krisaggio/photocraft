//! A finger navigates, it does not paint (#10264). On Windows winit reports a finger and a pen
//! alike as `WindowEvent::Touch`, and egui-winit turns each into an emulated pointer, so the first
//! finger of a two-finger pinch or pan left a stroke before the second one landed. While a finger
//! is on the screen (the platform's `finger_contact` service, from the `WM_POINTER` type) and
//! Preferences › Enhanced Controls › Finger Painting is off, [`drop_finger_pointer`] removes
//! those emulated pointer events. The `Event::Touch` events stay, so pinch, twist and pan still
//! read them.

use egui::{Event, PointerButton, RawInput};

/// Drops the primary-button presses and the pointer moves of this frame. Call from
/// `raw_input_hook` only when a finger is down.
pub fn drop_finger_pointer(raw: &mut RawInput) {
    raw.events.retain(|e| !matches!(e, Event::PointerMoved(_) | Event::PointerButton { button: PointerButton::Primary, .. }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Modifiers, TouchPhase, pos2};

    #[test]
    fn a_fingers_pointer_events_go_and_its_touch_events_stay() {
        let p = pos2(10.0, 10.0);
        let touch = Event::Touch { device_id: egui::TouchDeviceId(1), id: egui::TouchId(1), phase: TouchPhase::Start, pos: p, force: None };
        let mut raw = RawInput {
            events: vec![
                Event::PointerMoved(p),
                Event::PointerButton { pos: p, button: PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE },
                touch.clone(),
                Event::PointerButton { pos: p, button: PointerButton::Secondary, pressed: true, modifiers: Modifiers::NONE },
                Event::PointerGone,
            ],
            ..Default::default()
        };
        drop_finger_pointer(&mut raw);
        assert_eq!(raw.events.len(), 3);
        assert!(raw.events.contains(&touch));
        assert!(raw.events.contains(&Event::PointerGone));
    }

    #[test]
    fn a_stroke_needs_a_pointer_so_a_filtered_finger_cannot_start_one() {
        let ctx = egui::Context::default();
        let p = pos2(10.0, 10.0);
        let mut raw = RawInput {
            events: vec![Event::PointerMoved(p), Event::PointerButton { pos: p, button: PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE }],
            ..Default::default()
        };
        drop_finger_pointer(&mut raw);
        ctx.run_ui(raw, |ui| assert!(!ui.input(|i| i.pointer.primary_down())));
    }
}
