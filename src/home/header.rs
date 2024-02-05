use makepad_widgets::*;
use super::video_reel::VideoReelAction;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*

    ToggleButton = <Button> {
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
            color: (SELECTED_ITEM_COLOR);
            brightness: 1;

            fn get_color(self) -> vec4 {
                return self.color;
            }
        }
    }

    ToggleNetworkButton = <View> {
        width: 45,
        height: Fit,
        flow: Down,
        spacing: 5.0,
        align: {x: 0.5, y: 1.0},
        button = <ToggleButton> {}
        caption = <Label> {
            width: Fit,
            height: Fit,
            draw_text: {
                color: (SELECTED_ITEM_COLOR);
                text_style: <REGULAR_TEXT> { font_size: 7 }
            }
        }
    }

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

    Header = {{Header}} {
        width: Fill,
        height: Fit,
        flow: Right,
        spacing: 5.0,
        padding: 5.0,
        margin: {top: 30}

        <HeaderButton> {
            draw_icon: {
                svg_file: dep("crate://self/resources/plus_icon.svg")
            }
        }

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

        toggle_net_enable = <ToggleNetworkButton> {
            button = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/network_disabled.svg")
                    fn get_color(self) -> vec4 {
                        return #ff5e57
                    }
                }
            }
            caption = {
                text: "local"
            }
        }
        toggle_net_disable = <ToggleNetworkButton> {
            visible: false
            button = {
                draw_icon: {
                    svg_file: dep("crate://self/resources/network_enabled.svg")
                    fn get_color(self) -> vec4 {
                        return #x0be881
                    }
                }
            }
            caption = {
                text: "network"
            }
        }
    }
}

#[derive(Clone, Debug, DefaultNone)]
pub enum HeaderAction {
    None,
    ToggleNetwork,
}

#[derive(Live, LiveHook, Widget)]
pub struct Header {
    #[deref]
    view: View,

    #[rust(true)]
    toggle_ready: bool,
}

impl Widget for Header {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let chat_view = self.view(id!(chat));

        match event.hits(cx, chat_view.area()) {
            Hit::FingerUp(fe) => {
                if fe.was_tap() {
                    let uid = self.widget_uid();
                    cx.widget_action(uid, &scope.path, HeaderAction::ToggleNetwork);
                }
            }
            _ => (),
        }

        if self.toggle_ready {
            let toggle_net_enable = self.view(id!(toggle_net_enable));
            match event.hits(cx, toggle_net_enable.area()) {
                Hit::FingerUp(fe) => {
                    if fe.was_tap() {
                        let uid = self.widget_uid();
                        cx.widget_action(uid, &scope.path, HeaderAction::ToggleNetwork);
                        toggle_net_enable.set_visible(false);

                        let toggle_net_disable = self.view(id!(toggle_net_disable));
                        toggle_net_disable.set_visible(true);
                        toggle_net_disable.label(id!(caption)).set_text("loading");

                        self.toggle_ready = false;
                    }
                }
                _ => (),
            }

            let toggle_net_disable = self.view(id!(toggle_net_disable));
            match event.hits(cx, toggle_net_disable.area()) {
                Hit::FingerUp(fe) => {
                    if fe.was_tap() {
                        let uid = self.widget_uid();
                        cx.widget_action(uid, &scope.path, HeaderAction::ToggleNetwork);
                        toggle_net_disable.set_visible(false);

                        let toggle_net_enable = self.view(id!(toggle_net_enable));
                        toggle_net_enable.set_visible(true);
                        toggle_net_enable.label(id!(caption)).set_text("loading");

                        self.toggle_ready = false;
                    }
                }
                _ => (),
            }
        }

        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.view.draw_walk(cx, scope, walk);
        DrawStep::done()
    }
}

impl MatchEvent for Header {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions) {
        for action in actions {
            match action.downcast_ref().cast() {
                VideoReelAction::NetworkToggleReady => {
                    let toggle_net_enable = self.view(id!(toggle_net_enable));
                    toggle_net_enable.label(id!(caption)).set_text("local");

                    let toggle_net_disable = self.view(id!(toggle_net_disable));
                    toggle_net_disable.label(id!(caption)).set_text("network");

                    self.toggle_ready = true;
                }
                _ => (),
            }
        }
    }
}
