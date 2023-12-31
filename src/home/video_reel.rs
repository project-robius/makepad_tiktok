use crate::home::reel_actions::ReelButtonAction;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

const MEDIA_HEIGHT: f64 = 800.0;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::home::reel_actions::*;

    SEAGULLS = dep("crate://self/resources/seagulls.mp4")
    TRAIN = dep("crate://self/resources/train.mp4")
    DANCE = dep("crate://self/resources/dance.mp4")
    CAT = dep("crate://self/resources/cat.mp4")
    CAT2 = dep("crate://self/resources/cat2.mp4")

    MEDIA_WIDTH: 400.0
    MEDIA_HEIGHT: 800.0

    VideoReelItem = <View> {
        width: (MEDIA_WIDTH), height: (MEDIA_HEIGHT)
        flow: Overlay
        video = <Video> {
            source: (SEAGULLS)
            width: Fill,
            height: Fill,
            is_looping: true
            hold_to_pause: true
        }
        <View> {
            width: Fill, height: Fill
            flow: Down
            <View> {width: Fit, height: 300}
            <View> {
                width: Fill, height: Fill
                flow: Right
                <View> {width: 250, height: Fit}
                actions = <ReelActions> {
                    width: Fill, height: Fit
                    padding: 20
                }
            }
        }
    }

    VideoReel = {{VideoReel}} {
        width: Fill
        height: Fill
        flow: Overlay,
        align: {x: 0.0, y: 0.0}

        item1 = <VideoReelItem> {
            video = {
                source: Dependency { path: (TRAIN) }
                autoplay: true
            }
        }

        item2 = <VideoReelItem> {
            video = {
                source: Dependency { path: (CAT) }
            }
        }

        item3 = <VideoReelItem> {
            video = {
                source: Dependency { path: (SEAGULLS) }
            }
        }

        item4 = <VideoReelItem> {
            video = {
                source: Dependency { path: (DANCE) }
            }
        }

        item5 = <VideoReelItem> {
            video = {
                source: Dependency { path: (CAT2) }
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

#[derive(Live, Widget)]
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
    media_containers: Vec<ViewRef>,
    #[rust(0)]
    current_media_index: i32,
    #[rust(0)]
    previous_media_index: i32,

    #[rust]
    dragging: bool,
    #[rust]
    offset_from_drag: f64,
    #[rust]
    init_drag_time: f64,

    #[rust(true)]
    change_video_enabled: bool,

    #[rust(VideoReelDirection::Forward)]
    direction: VideoReelDirection,
}

impl LiveHook for VideoReel {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.media_containers = vec![
            self.view(id!(item1)),
            self.view(id!(item2)),
            self.view(id!(item3)),
            self.view(id!(item4)),
            self.view(id!(item5)),
        ];

        self.begin_media();

        self.next_view = cx.new_next_frame();
        self.animator_play(cx, id!(carrousel.initial));
    }
}

impl Widget for VideoReel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.match_event(cx, event);

        self.control_animation(cx, event);
        self.handle_mouse_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let _ = self.view.draw_walk(cx, scope, walk);
        self.next_view = cx.new_next_frame();
        DrawStep::done()
    }
}

impl MatchEvent for VideoReel {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions) {
        for action in actions {
            match action.downcast_ref().cast() {
                ReelButtonAction::ShowComments => {
                    self.change_video_enabled = false;
                }
                _ => (),
            }
        }
    }
}

impl VideoReel {
    fn control_animation(&mut self, cx: &mut Cx, event: &Event) {
        if let Some(_ne) = self.next_view.is_event(event) {
            if !self.dragging {
                if self.animator_handle_event(cx, event).is_animating() {
                    self.update_media_positions(cx);
                    self.redraw(cx);
                } else {
                    self.fire_next_animation(cx);
                }
            }
            self.next_view = cx.new_next_frame();
        }

        if let Event::NextFrame(_) = event {
            self.next_view = cx.new_next_frame();
        }
    }

    fn get_active_containers(&mut self) -> (ViewRef, ViewRef) {
        let prev_index = self.previous_media_index as usize;
        let curr_index = self.current_media_index as usize;
        (
            self.media_containers[prev_index].clone(),
            self.media_containers[curr_index].clone(),
        )
    }

    fn update_media_positions(&mut self, cx: &mut Cx) {
        if self.animator.animator_in_state(cx, id!(carrousel.display)) {
            let direction = self.direction;
            let (mut prev_media, mut current_media) = self.get_active_containers();

            match direction {
                VideoReelDirection::Forward => {
                    let offset = (self.offset - self.offset_from_drag).max(0.0);
                    Self::set_vertical_position(&mut current_media, offset, cx);
                    Self::set_vertical_position(&mut prev_media, offset - MEDIA_HEIGHT, cx);
                }
                VideoReelDirection::Backward => {
                    let offset = (self.offset + self.offset_from_drag).max(0.0);
                    Self::set_vertical_position(&mut current_media, -offset, cx);
                    Self::set_vertical_position(&mut prev_media, MEDIA_HEIGHT - offset, cx);
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
            let (previous_image, _) = self.get_active_containers();
            previous_image.set_visible(false);
        }
    }

    // TODO rename
    fn set_vertical_position(media_ref: &mut ViewRef, offset: f64, cx: &mut Cx) {
        media_ref.apply_over(cx, live! {margin: {top: (offset) }});
    }

    fn handle_mouse_event(&mut self, cx: &mut Cx, event: &Event) {
        if self.animator.is_track_animating(cx, id!(carrousel))
            && self.animator.animator_in_state(cx, id!(carrousel.display))
        {
            return;
        }

        match event.hits_with_capture_overload(cx, self.view.area(), true) {
            Hit::FingerDown(fe) => {
                self.last_abs = fe.abs.y;
                self.init_drag_time = fe.time;
                self.offset_from_drag = 0.0;
            }
            Hit::FingerMove(fe) => {
                let time_elapsed = fe.time - self.init_drag_time;
                if self.change_video_enabled && time_elapsed > 0.15 {
                    let delta = self.last_abs - fe.abs.y;
                    let (_, mut current_media) = self.get_active_containers();

                    self.dragging = true;
                    if fe.abs.y > self.last_abs {
                        let upcoming_image_index = (self.current_media_index - 1)
                            .rem_euclid(self.media_containers.len() as i32);
                        let mut upcoming_image =
                            self.media_containers[upcoming_image_index as usize].clone();
                        Self::set_vertical_position(&mut upcoming_image, -MEDIA_HEIGHT - delta, cx);

                        upcoming_image.set_visible(true);
                    } else {
                        let upcoming_image_index = (self.current_media_index + 1)
                            .rem_euclid(self.media_containers.len() as i32);
                        let mut upcoming_image =
                            self.media_containers[upcoming_image_index as usize].clone();
                        Self::set_vertical_position(&mut upcoming_image, MEDIA_HEIGHT - delta, cx);
                        upcoming_image.set_visible(true);
                    }

                    self.offset = -delta;
                    Self::set_vertical_position(&mut current_media, -delta, cx);
                    self.redraw(cx);
                }
            }
            Hit::FingerUp(fe) => {
                if self.change_video_enabled
                    && fe.is_over
                    && (fe.abs.y - self.last_abs).abs() > 10.0
                {
                    self.previous_media_index = self.current_media_index;

                    if fe.abs.y > self.last_abs {
                        self.setup_next_animation(VideoReelDirection::Backward);
                    } else {
                        self.setup_next_animation(VideoReelDirection::Forward);
                    };
                    self.update_media(cx);

                    self.offset_from_drag = -self.offset;
                    self.animator_play(cx, id!(carrousel.restart));
                }
                self.dragging = false;
            }
            _ => {}
        }
    }

    fn setup_next_animation(&mut self, direction: VideoReelDirection) {
        self.direction = direction;
        self.previous_media_index = self.current_media_index;
        match direction {
            VideoReelDirection::Forward => {
                self.current_media_index =
                    (self.current_media_index + 1).rem_euclid(self.media_containers.len() as i32);
            }
            VideoReelDirection::Backward => {
                self.current_media_index =
                    (self.current_media_index - 1).rem_euclid(self.media_containers.len() as i32);
            }
        }
    }

    fn begin_media(&mut self) {
        let (_, current_media) = self.get_active_containers();
        current_media.set_visible(true);

        for (i, media) in self.media_containers.iter().enumerate() {
            if i != self.current_media_index as usize {
                media.set_visible(false);
            }
        }
    }

    fn update_media(&mut self, cx: &mut Cx) {
        for (i, media) in self.media_containers.iter().enumerate() {
            if i == self.current_media_index as usize {
                media.set_visible(true);
                media.video(id!(video)).begin_playback(cx);
            } else if i == self.previous_media_index as usize {
                // keep previous visible so it doesn't dissapear on transition
                media.set_visible(true);
                media.video(id!(video)).stop_and_cleanup_resources(cx);
            } else {
                media.set_visible(false);
            }
        }
    }
}

impl VideoReelRef {
    pub fn comments_dismissed(&self) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.change_video_enabled = true;
        }
    }
}
