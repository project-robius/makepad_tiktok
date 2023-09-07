use makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::home::header::*;

    VideoReel = <View> {
        width: Fill,
        height: Fill,
        
        <Image> {
            source: dep("crate://self/resources/video_preview_1.png")
            width: Fill,
            height: Fill,
        }
    }

    HomeScreen = <View> {
        width: Fill,
        height: Fill,
        flow: Overlay

        <VideoReel> {}
        <Header> {}
    }
}
