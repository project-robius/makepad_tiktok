use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::search_bar::SearchBar;

    IMG_SMILEY_FACE_BW = dep("crate://self/resources/smiley_face_bw.png")
    IMG_PLUS = dep("crate://self/resources/plus.png")
    IMG_AT_SIGN = dep("crate://self/resources/at_sign.png")
    IMG_CAMERA = dep("crate://self/resources/camera.png")
    IMG_HEART = dep("crate://self/resources/heart.png")
    IMG_AVATAR = dep("crate://self/resources/avatars/user1.png")

    FillerX = <View> { width: Fill, height: Fit }
    FillerY = <View> { width: Fit, height: Fill }

    Comment = <View> {
        width: Fill, height: Fit

        content = <View> {
            flow: Right, spacing: 5., padding: 10., align: {y: 0}
            width: Fill, height: Fit

            avatar = <Image> {
                source: (IMG_AVATAR),
                width: 36., height: 36.
            }
            text = <View> {
                flow: Down
                spacing: 10
                width: Fill, height: Fit
                padding: {left: 10., right: 10.}, align: {y: 0.5}

                username = <Label> {
                    text:""
                    draw_text:{
                        text_style: <REGULAR_TEXT>{font_size: 10.},
                        color: #af
                    }
                }

                comment_text = <Label> {
                    text:""
                    draw_text:{
                        text_style: <REGULAR_TEXT>{font_size: 10.},
                        color: #000
                    }
                }

                metadata = <View> {
                    flow: Right
                    width: Fill, height: Fit
                    align: {y: 0.5}

                    timestamp = <Label> {
                        text:""
                        draw_text:{
                            text_style: <REGULAR_TEXT>{font_size: 10.},
                            color: #af
                        }
                    }
                    <FillerX> {}
                    <View> {
                        width: Fit, height: Fit
                        margin: {right: 100.}

                        <Label> {
                            text:"回复"
                            draw_text:{
                                text_style: <REGULAR_TEXT>{font_size: 10.},
                                color: #000
                            }
                        }
                    }
                    <FillerX> {}
                    likes = <View> {
                        flow: Right, spacing: 2.
                        width: Fit, height: Fit
                        align: {y: 0.5}

                        <Image> {
                            source: (IMG_HEART),
                            width: 20., height: 20.
                        }

                        label = <Label> {
                            text:""
                            draw_text:{
                                text_style: <REGULAR_TEXT>{font_size: 10.},
                                color: #af
                            }
                        }
                    }
                }
            }
        }
    }

    Comments = {{Comments}} {
        width: Fill, height: Fill
        flow: Right, spacing: 10., padding: 0.

        list: <PortalList> {
            auto_tail: false
            grab_key_focus: true

            width: Fill, height: Fill
            flow: Down, spacing: 0.

            comment = <Comment> {}
        }
    }

    CommentSection = <View> {
        width: Fill, height: Fill
        flow: Down
        show_bg: true,
        draw_bg: {
            color: #f8
        }

        <View> {
            width: Fill, height: Fit
            margin: 10.
            <FillerX> {}
            <Label> {
                text:"50 条评论"
                draw_text:{
                    text_style: <REGULAR_TEXT>{font_size: 10.},
                    color: #000
                }
            }
            <FillerX> {}
            close_button = <Button> {
                text: "X"
                draw_text:{
                    text_style: <REGULAR_TEXT>{font_size: 10.},
                    color: #af
                }
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return #f8;
                    }
                }

            }
        }
        comments = <Comments> {}
        <View> {
            width: Fill, height: Fit
            flow: Down
            <RoundedView> {
                width: Fill, height: 1.0
                draw_bg: {color: #000}
            }
        }
        <View> {
            width: Fill, height: Fit
            flow: Right, align: {y: 0.5}, padding: 10.
            show_bg: true,
            draw_bg: {
                color: #f8
            }

            <Image> {
                source: (IMG_CAMERA),
                width: 36., height: 36.
            }
            message_input = <SearchBar> {
                show_bg: false
                input = {
                    width: Fill, height: Fit, margin: 0
                    empty_message: " "
                    draw_text:{
                        text_style:<REGULAR_TEXT>{font_size: 11},

                        fn get_color(self) -> vec4 {
                            return #0
                        }
                    }
                }
            }
            <Image> {
                source: (IMG_AT_SIGN),
                width: 36., height: 36.
            }
            <Image> {
                source: (IMG_SMILEY_FACE_BW),
                width: 36., height: 36.
            }
            <Image> {
                source: (IMG_PLUS),
                width: 36., height: 36.
            }
        }
    }
}

#[derive(Live)]
pub struct Comments {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    #[rust]
    comments: Vec<CommentEntry>,
    #[live]
    list: PortalList,
}

impl LiveHook for Comments {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Comments);
    }

    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.comments = vec![
            CommentEntry {
                username: "温柔的晚安".to_string(),
                text: "这可是世首美".to_string(),
                timestamp: "11:23".to_string(),
                likes: 58,
            },
            CommentEntry {
                username: "相见不如怀念＞".to_string(),
                text: "梨花你在干什么".to_string(),
                timestamp: "11:23".to_string(),
                likes: 50,
            },
            CommentEntry {
                username: "无人触及".to_string(),
                text: "我真的成了".to_string(),
                timestamp: "11:22".to_string(),
                likes: 48,
            },
            CommentEntry {
                username: "无时无刻不想着你".to_string(),
                text: "《张老三是哪个吊毛》".to_string(),
                timestamp: "11:22".to_string(),
                likes: 18,
            },
            CommentEntry {
                username: "我我我我我".to_string(),
                text: "为什么没有人说张老三开挂".to_string(),
                timestamp: "10:10".to_string(),
                likes: 33,
            },
            CommentEntry {
                username: "完美情人".to_string(),
                text: "我唐晓耐实名观看".to_string(),
                timestamp: "10:10".to_string(),
                likes: 32,
            },
            CommentEntry {
                username: "我我我我我".to_string(),
                text: "为什么没有人说张老三开挂".to_string(),
                timestamp: "10:10".to_string(),
                likes: 21,
            },
            CommentEntry {
                username: "无人触及".to_string(),
                text: "我真的成了".to_string(),
                timestamp: "10:10".to_string(),
                likes: 80,
            },
            CommentEntry {
                username: "无人触及".to_string(),
                text: "我真的成了".to_string(),
                timestamp: "10:00".to_string(),
                likes: 10,
            },
            CommentEntry {
                username: "完美情人".to_string(),
                text: "我唐晓耐实名观看".to_string(),
                timestamp: "8:23".to_string(),
                likes: 78,
            },
        ];
    }
}

impl Widget for Comments {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let actions = self.list.handle_widget_event(cx, event);
        for action in actions {
            dispatch_action(cx, action);
        }
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.list.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.list.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl Comments {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
        let comment_entries_count = self.comments.len() as u64;

        cx.begin_turtle(walk, self.layout);

        let range_end = if comment_entries_count > 0 {
            comment_entries_count - 1
        } else {
            0
        };
        self.list.set_item_range(cx, 0, range_end);

        while self.list.draw_widget(cx).hook_widget().is_some() {
            while let Some(item_id) = self.list.next_visible_item(cx) {
                if item_id < comment_entries_count {
                    let item_index = item_id as usize;
                    let item_content = &self.comments[item_index];

                    let template = id!(comment);

                    let item = self.list.item(cx, item_id, template[0]).unwrap();

                    item.label(id!(text.comment_text))
                        .set_text(&item_content.text);
                    item.label(id!(text.username))
                        .set_text(&item_content.username);
                    item.label(id!(text.metadata.timestamp))
                        .set_text(&item_content.timestamp);
                    item.label(id!(text.metadata.likes.label))
                        .set_text(&item_content.likes.to_string());

                    item.draw_widget_all(cx);
                }
            }
        }

        cx.end_turtle();
    }
}

struct CommentEntry {
    pub username: String,
    pub text: String,
    pub timestamp: String,
    pub likes: u32,
}
