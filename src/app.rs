use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*
    import crate::home::home_screen::*

    AppTab = <RadioButton> {
        height: Fill
        width: Fit
        align: {x: 0.0, y: 0.0}

        draw_radio: {
            radio_type: Tab,
            color_active: (BACKGROUND_COLOR),
            color_inactive: (BACKGROUND_COLOR),
        }
        draw_text: {
            color_selected: (SELECTED_ITEM_COLOR),
            color_unselected: (UNSELECTED_ITEM_COLOR),
            color_unselected_hover: (SELECTED_ITEM_COLOR),
            text_style: <APP_NAVIGATION_FONT> {}
        }
    }

    CenterTab = <View> {
        height: Fill
        width: Fit
        align: {x: 0.5, y: 0.5}

        <RoundedView> {
            width: 34.,
            height: 28.
            flow: Right,
            align: {x: 0.5, y: 0.5}

            draw_bg: {
                border_color: (SELECTED_ITEM_COLOR)
                color: (BACKGROUND_COLOR)
                border_width: 1.5
                radius: 4.
            }

            <Label> {
                width: Fit,
                height: Fit

                text: "+"
                draw_text: {
                    color: (SELECTED_ITEM_COLOR)
                    text_style: <REGULAR_TEXT> { font_size: 20 }
                }
            }

        }
    }

    App = {{App}} {
        ui: <DesktopWindow> {
            window: {position: vec2(0, 0), inner_size: vec2(400, 800)},
            pass: {clear_color: #2A}
            block_signal_event: true;

            application_pages = <View> {
                margin: 0
                padding: 0

                tab1_frame = <HomeScreen> {visible: true}
                tab2_frame = <View> {visible: false}
                tab3_frame = <View> {visible: false}
                tab4_frame = <View> {visible: false}
                tab5_frame = <View> {visible: false}
            }

            horizontal_divider = <View> {
                width: Fill,
                height: 5,
                margin: 0.0,
                padding: 0.0, spacing: 0.0
                show_bg: true
                draw_bg: {
                    color: (COLOR_DIVIDER)
                }
            }

            mobile_menu = <View> {
                width: Fill
                height: 80
                padding: {top: 10, bottom: 40, left: 10, right: 10}
                flow: Right
                spacing: 6.0
                show_bg: true
                draw_bg: {
                    color: (BACKGROUND_COLOR)
                }

                mobile_modes = <View> {
                    tab1 = <AppTab> {
                        animator: {selected = {default: on}}
                        label: "首页"
                        width: Fill
                        flow: Down
                        spacing: 5.0
                        align: {x: 0.5, y: 0.5}

                        icon_walk: {width: 20, height: 20}
                    }
                    tab2 = <AppTab> {
                        label: "朋友",
                        width: Fill
                        flow: Down
                        spacing: 5.0
                        align: {x: 0.5, y: 0.5}

                        icon_walk: {width: 20, height: 20}
                    }
                    tab3 = <CenterTab> {
                        width: Fill
                        flow: Down
                        spacing: 5.0
                        align: {x: 0.5, y: 0.5}
                    }
                    tab4 = <AppTab> {
                        label: "消息",
                        width: Fill
                        flow: Down
                        spacing: 5.0
                        align: {x: 0.5, y: 0.5}

                        icon_walk: {width: 20, height: 20}
                    }
                    tab5 = <AppTab> {
                        label: "我",
                        width: Fill
                        flow: Down
                        spacing: 5.0
                        align: {x: 0.5, y: 0.5}

                        icon_walk: {width: 20, height: 20}
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::shared::styles::live_design(cx);
        crate::home::home_screen::live_design(cx);
        crate::home::header::live_design(cx);
        crate::home::video_reel::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            return self.ui.draw_widget_all(&mut Cx2d::new(cx, event));
        }

        let actions = self.ui.handle_widget_event(cx, event);

        self.ui.radio_button_set(ids!(
            mobile_modes.tab1,
            mobile_modes.tab2,
            mobile_modes.tab3,
            mobile_modes.tab4,
            mobile_modes.tab5,
        ))
        .selected_to_visible(
            cx,
            &self.ui,
            &actions,
            ids!(
                application_pages.tab1_frame,
                application_pages.tab2_frame,
                application_pages.tab3_frame,
                application_pages.tab4_frame,
                application_pages.tab5_frame,
            ),
        );
    }
}
