use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

const IMAGE_HEIGHT: f64 = 800.0;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    VIDEO_PLACEHOLDER_1_IMG = dep("crate://self/resources/video_preview_1.png")
    VIDEO_PLACEHOLDER_2_IMG = dep("crate://self/resources/video_preview_2.png")
    VIDEO_PLACEHOLDER_3_IMG = dep("crate://self/resources/video_preview_3.png")

    IMAGE_WIDTH: 400.0
    IMAGE_HEIGHT: 800.0
    
    VideoReel = {{VideoReel}} {
        width: Fill
        height: Fill
        flow: Overlay,
        align: {x: 0.0, y: 0.0}

        image1 = <View> {
            width: (IMAGE_WIDTH), height: (IMAGE_HEIGHT)
            <Image> {
                width: Fill
                height: Fill
                source: (VIDEO_PLACEHOLDER_1_IMG)
            }
        }

        image2 = <View> {
            width: (IMAGE_WIDTH), height: (IMAGE_HEIGHT)
            <Image> {
                width: Fill
                height: Fill
                source: (VIDEO_PLACEHOLDER_2_IMG)
            }
        }

        image3 = <View> {
            width: (IMAGE_WIDTH), height: (IMAGE_HEIGHT)
            <Image> {
                width: Fill
                height: Fill
                source: (VIDEO_PLACEHOLDER_3_IMG)
            }
        }

        offset: 0

        animator: {
            carrousel = {
                default: display,
                display = {
                    from: {all: Forward {duration: 0.4}}
                    apply: {offset: 0.0}
                }

                initial = {
                    from: {all: Snap}
                    apply: {offset: 0.0}
                }

                restart = {
                    from: {all: Snap}
                    apply: {offset: 800.0}
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum VideoReelDirection {
    Forward,
    Backward,
}

#[derive(Live)]
pub struct VideoReel {
    #[deref]
    view: View,

    #[live]
    offset: f64,

    #[animator]
    animator: Animator,

    #[rust]
    next_view: NextFrame,

    #[rust]
    last_abs: f64,

    #[rust]
    image_containers: Vec<ViewRef>,

    #[rust(0)]
    current_image_index: i32,

    #[rust(0)]
    previous_image_index: i32,

    #[rust(VideoReelDirection::Forward)]
    direction: VideoReelDirection,
}

impl LiveHook for VideoReel {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, VideoReel);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.image_containers = vec![
            self.view(id!(image1)),
            self.view(id!(image2)),
            self.view(id!(image3)),
        ];

        self.reset_images_visibility();

        self.next_view = cx.new_next_frame();
        self.animator_play(cx, id!(carrousel.initial));
    }
}

impl Widget for VideoReel {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.control_animation(cx, event);
        self.handle_mouse_event(cx, event);
    }

    fn walk(&self) -> Walk {
        self.view.walk()
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        self.next_view = cx.new_next_frame();
        WidgetDraw::done()
    }
}

impl VideoReel {
    fn control_animation(&mut self, cx: &mut Cx, event: &Event) {
        if let Some(_ne) = self.next_view.is_event(event) {
            if self.animator_handle_event(cx, event).is_animating() {
                self.update_image_positions(cx);
                self.redraw(cx);
            } else {
                self.fire_next_animation(cx);
            }
            self.next_view = cx.new_next_frame();
        }

        if let Event::NextFrame(_) = event {
            self.next_view = cx.new_next_frame();
        }
    }

    fn get_active_images_containers(&mut self) -> (ViewRef, ViewRef) {
        let prev_index = self.previous_image_index as usize;
        let curr_index = self.current_image_index as usize;
        (
            self.image_containers[prev_index].clone(),
            self.image_containers[curr_index].clone()
        )
    }

    fn update_image_positions(&mut self, cx: &mut Cx) {
        if self.animator.animator_in_state(cx, id!(carrousel.display)) {
            let direction = self.direction;
            let offset = self.offset;

            let (mut prev_image, mut current_image) = self.get_active_images_containers();

            match direction {
                VideoReelDirection::Forward => {
                    Self::set_vertical_position(&mut current_image, offset, cx);
                    Self::set_vertical_position(&mut prev_image, offset - IMAGE_HEIGHT, cx);
                },
                VideoReelDirection::Backward => {
                    Self::set_vertical_position(&mut current_image, -offset, cx);
                    Self::set_vertical_position(&mut prev_image, IMAGE_HEIGHT - offset, cx);
                }
            }
        }
    }

    fn fire_next_animation(&mut self, cx: &mut Cx) {
        if self.animator.animator_in_state(cx, id!(carrousel.restart)) {
            // Fires the animation of the carrousel again
            self.animator_play(cx, id!(carrousel.display));
        } else if self.animator.animator_in_state(cx, id!(carrousel.display)) {
            // Begins the period of time where the carrousel is stopped
            let (previous_image, _) = self.get_active_images_containers();
            previous_image.set_visible(false);
        }
    }

    // TODO rename
    fn set_vertical_position(image_ref: &mut ViewRef, offset: f64, cx: &mut Cx) {
        image_ref.apply_over(cx, live!{margin: {top: (offset) }});
    }

    fn handle_mouse_event(&mut self, cx: &mut Cx, event: &Event) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerDown(fe) => {
                self.last_abs = fe.abs.y;
            }
            Hit::FingerUp(fe) => {
                if fe.is_over && (fe.abs.y - self.last_abs).abs() > 10.0 {
                    self.previous_image_index = self.current_image_index;

                    let initial_offset = if fe.abs.y > self.last_abs {
                        self.setup_next_animation(VideoReelDirection::Backward);
                        IMAGE_HEIGHT
                    } else {
                        self.setup_next_animation(VideoReelDirection::Forward);
                        -IMAGE_HEIGHT
                    };

                    let (_, mut current_image) = self.get_active_images_containers();
                    Self::set_vertical_position(&mut current_image, initial_offset, cx);
                    self.reset_images_visibility();

                    self.animator_play(cx, id!(carrousel.restart));
                }
            },
            _ => {}
        }
    }

    fn setup_next_animation(&mut self, direction: VideoReelDirection) {
        self.direction = direction;
        self.previous_image_index = self.current_image_index;
        match direction {
            VideoReelDirection::Forward => {
                self.current_image_index = (self.current_image_index + 1).rem_euclid(self.image_containers.len() as i32);
            },
            VideoReelDirection::Backward => {
                self.current_image_index = (self.current_image_index - 1).rem_euclid(self.image_containers.len() as i32);
            }
        }
    }

    fn reset_images_visibility(&mut self) {
        for (i, image) in self.image_containers.iter().enumerate() {
            if i == self.current_image_index as usize || i == self.previous_image_index as usize {
                image.set_visible(true);
            } else {
                image.set_visible(false);
            }
        }
    }
}
