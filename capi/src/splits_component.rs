//! The Splits Component is the main component for visualizing all the split
//! times. Each segment is shown in a tabular fashion showing the segment icon,
//! segment name, the delta compared to the chosen comparison, and the split
//! time. The list provides scrolling functionality, so not every segment needs
//! to be shown all the time.

use livesplit_core::component::splits::Component as SplitsComponent;
use livesplit_core::{GeneralLayoutSettings, Timer};
use super::{acc, acc_mut, alloc, output_vec, own, own_drop, Json};
use splits_component_state::OwnedSplitsComponentState;
use component::OwnedComponent;

/// type
pub type OwnedSplitsComponent = *mut SplitsComponent;

/// Creates a new Splits Component.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_new() -> OwnedSplitsComponent {
    alloc(SplitsComponent::new())
}

/// drop
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_drop(this: OwnedSplitsComponent) {
    own_drop(this);
}

/// Converts the component into a generic component suitable for using with a
/// layout.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_into_generic(
    this: OwnedSplitsComponent,
) -> OwnedComponent {
    alloc(own(this).into())
}

/// Encodes the component's state information as JSON.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state_as_json(
    this: *mut SplitsComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> Json {
    output_vec(|o| {
        acc_mut(this)
            .state(acc(timer), acc(layout_settings))
            .write_json(o)
            .unwrap();
    })
}

/// Calculates the component's state based on the timer and layout settings
/// provided.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_state(
    this: *mut SplitsComponent,
    timer: *const Timer,
    layout_settings: *const GeneralLayoutSettings,
) -> OwnedSplitsComponentState {
    alloc(acc_mut(this).state(acc(timer), acc(layout_settings)))
}

/// Scrolls up the window of the segments that are shown. Doesn't move the
/// scroll window if it reaches the top of the segments.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_up(this: *mut SplitsComponent) {
    acc_mut(this).scroll_up();
}

/// Scrolls down the window of the segments that are shown. Doesn't move the
/// scroll window if it reaches the bottom of the segments.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_scroll_down(this: *mut SplitsComponent) {
    acc_mut(this).scroll_down();
}

/// The amount of segments to show in the list at any given time. If this is
/// set to 0, all the segments are shown. If this is set to a number lower
/// than the total amount of segments, only a certain window of all the
/// segments is shown. This window can scroll up or down.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_visual_split_count(
    this: *mut SplitsComponent,
    count: usize,
) {
    acc_mut(this).settings_mut().visual_split_count = count;
}

/// If there's more segments than segments that are shown, the window
/// showing the segments automatically scrolls up and down when the current
/// segment changes. This count determines the minimum number of future
/// segments to be shown in this scrolling window when it automatically
/// scrolls.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_split_preview_count(
    this: *mut SplitsComponent,
    count: usize,
) {
    acc_mut(this).settings_mut().split_preview_count = count;
}

/// If not every segment is shown in the scrolling window of segments, then
/// this determines whether the final segment is always to be shown, as it
/// contains valuable information about the total duration of the chosen
/// comparison, which is often the runner's Personal Best.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_always_show_last_split(
    this: *mut SplitsComponent,
    always_show_last_split: bool,
) {
    acc_mut(this).settings_mut().always_show_last_split = always_show_last_split;
}

/// If the last segment is to always be shown, this determines whether to
/// show a more pronounced separator in front of the last segment, if it is
/// not directly adjacent to the segment shown right before it in the
/// scrolling window.
#[no_mangle]
pub unsafe extern "C" fn SplitsComponent_set_separator_last_split(
    this: *mut SplitsComponent,
    separator_last_split: bool,
) {
    acc_mut(this).settings_mut().separator_last_split = separator_last_split;
}
