use makepad_widgets::*;

live_design! {
    APP_NAVIGATION_FONT = {
        font_size: 12,
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }

    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://makepad-widgets/resources/GoNotoKurrent-Regular.ttf")}
    }

    BACKGROUND_COLOR = #111
    COLOR_DIVIDER = #ddd
    SELECTED_ITEM_COLOR = #fff
    UNSELECTED_ITEM_COLOR= #888
}
