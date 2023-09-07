use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*

    HeaderButton = <Button> {
        width: Fit, height: Fit
        padding: 0.
        icon_walk: {width: 20, height: Fit}
        draw_bg: {
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                return sdf.result
            }
        }
        draw_icon: {
            color: #000;
            brightness: 0.8;
        }
    }

    HeaderItem = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10.0,
        align: {x: 0.5, y: 1.0},

        label = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                color: (UNSELECTED_ITEM_COLOR),
                text_style: <REGULAR_TEXT> {}
            }
        }
        underline = <View> {
            width: 30,
            height: 2,
            margin: 0.0,
            padding: 0.0, spacing: 0.0
            show_bg: true
            draw_bg: {
                color: (UNSELECTED_ITEM_COLOR)
            }
        }
    }

    Header = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 10.0,
        padding: 10.0,
        margin: {top: 30}

        <HeaderButton> {
            draw_icon: {
                svg_file: dep("crate://self/resources/plus_icon.svg")
            }
        }

        <HeaderItem> { label = { text: "探索" } }
        <HeaderItem> { label = { text: "北京" } }
        <HeaderItem> { label = { text: "关注" } }
        <HeaderItem> { label = { text: "商城" } }
        <HeaderItem> {
            label = {
                draw_text: {
                    color: (SELECTED_ITEM_COLOR),
                }
                text: "推荐"
            }
            underline = {
                draw_bg: {
                    color: (SELECTED_ITEM_COLOR)
                }
            }
        }

        <HeaderButton> {
            draw_icon: {
                svg_file: dep("crate://self/resources/search_icon.svg")
            }
        }
    }
}
