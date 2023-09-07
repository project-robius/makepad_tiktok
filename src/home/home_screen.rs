use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::home::header::*;
    import crate::home::video_reel::*;

    HomeScreen = <View> {
        width: Fill,
        height: Fill,
        flow: Overlay

        <VideoReel> {}
        <Header> {}
    }
}
