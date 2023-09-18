use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    ReelActionsButton = <View> {
        width: Fit,
        height: Fit,
        flow: Down,
        spacing: 5.0,
        align: {x: 0.5, y: 1.0},
        button = <Button> {
            width: Fit, height: Fit
            padding: 0.
            icon_walk: {width: 26, height: Fit}
            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    return sdf.result
                }
            }
            draw_icon: {
                color: (SELECTED_ITEM_COLOR);
                brightness: 1;

                fn get_color(self) -> vec4 {
                    return self.color;
                }
            }
        }
        caption = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                color: (SELECTED_ITEM_COLOR);
                text_style: <REGULAR_TEXT> { font_size: 8 }
            }
        }
    }

    ReelActions = {{ReelActions}} {
        height: 500
        width: Fit
        flow: Down
        spacing: 30.0
        align: {x: 0.5, y: 0.5}

        <ReelActionsButton> {
            button = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/heart_icon.svg")
                }
            }
            caption = {
                text: "1234"
            }
        }
        chat = <ReelActionsButton> {
            button = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/chat_icon.svg")
                }
            }
            caption = {
                text: "2234"
            }
        }
        <ReelActionsButton> {
            button = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/star_icon.svg")
                }
            }
            caption = {
                text: "3234"
            }
        }
        <ReelActionsButton> {
            button = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/share_icon.svg")
                }
            }
            caption = {
                text: "4234"
            }
        }
    }
}

#[derive(Clone, WidgetAction, Debug)]
pub enum ReelButtonAction {
    None,
    ShowComments,
}

#[derive(Live)]
pub struct ReelActions {
    #[deref]
    view: View,
}

impl LiveHook for ReelActions {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ReelActions);
    }
}

impl Widget for ReelActions {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let actions = self.view.handle_widget_event(cx, event);

        let chat_button = self.button(id!(chat.button));
        if chat_button.clicked(&actions) {
            let uid = self.widget_uid();
            dispatch_action(cx, WidgetActionItem::new(ReelButtonAction::ShowComments.into(), uid));
        };
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}