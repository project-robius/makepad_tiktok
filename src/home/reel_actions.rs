use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

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
        height: Fit
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

#[derive(Clone, Debug, DefaultNone)]
pub enum ReelButtonAction {
    None,
    ShowComments,
}

#[derive(Live, LiveHook, Widget)]
pub struct ReelActions {
    #[deref]
    view: View,
}

impl Widget for ReelActions {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let chat_view = self.view(id!(chat));

        match event.hits(cx, chat_view.area()) {
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    let uid = self.widget_uid();
                    cx.widget_action(uid, &scope.path, ReelButtonAction::ShowComments);
                }
            }
            _ => (),
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.view.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
}
