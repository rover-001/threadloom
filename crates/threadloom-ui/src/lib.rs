#![allow(warnings)]
use std::rc::Rc;
use threadloom_core::{element, fragment, text, View, IntoView};

pub mod components;
pub use components::*;
use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_id() -> String {
    format!("tl-{}", ID_COUNTER.fetch_add(1, Ordering::SeqCst))
}

#[cfg(target_arch = "wasm32")]
pub fn run_animation(script: String) {
    let _ = js_sys::eval(&script);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_animation(_script: String) {}

pub fn apply_animations(id: &str, animate: Option<String>, animate_from: Option<String>, animate_fromto: Option<(String, String)>) {
    if let Some(config) = animate {
        let script = format!("setTimeout(() => {{ if (window.gsap) gsap.to('#{}', {}) }}, 10);", id, config);
        run_animation(script);
    }
    if let Some(config) = animate_from {
        let script = format!("setTimeout(() => {{ if (window.gsap) gsap.from('#{}', {}) }}, 10);", id, config);
        run_animation(script);
    }
    if let Some((from_cfg, to_cfg)) = animate_fromto {
        let script = format!("setTimeout(() => {{ if (window.gsap) gsap.fromTo('#{}', {}, {}) }}, 10);", id, from_cfg, to_cfg);
        run_animation(script);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Ergonomic helper types
// ═══════════════════════════════════════════════════════════════════════════════

/// Wraps an optional callback. Accepts: closure, `Rc<dyn Fn()>`, or `None::<fn()>` / `()`.
#[derive(Default, Clone)]
pub struct Callback(pub Option<Rc<dyn Fn()>>);

impl<F: Fn() + 'static> From<F> for Callback {
    fn from(f: F) -> Self { Callback(Some(Rc::new(f))) }
}
impl From<Rc<dyn Fn()>> for Callback {
    fn from(rc: Rc<dyn Fn()>) -> Self { Callback(Some(rc)) }
}
impl From<Option<Rc<dyn Fn()>>> for Callback {
    fn from(opt: Option<Rc<dyn Fn()>>) -> Self { Callback(opt) }
}
// Allow passing `None::<fn()>` or just `()` for no-op
impl From<()> for Callback {
    fn from(_: ()) -> Self { Callback(None) }
}

/// Wraps an optional callback with 1 argument.
#[derive(Clone)]
pub struct Callback1<T>(pub Option<Rc<dyn Fn(T)>>);

impl<T> Default for Callback1<T> {
    fn default() -> Self { Callback1(None) }
}

impl<T, F: Fn(T) + 'static> From<F> for Callback1<T> {
    fn from(f: F) -> Self { Callback1(Some(Rc::new(f))) }
}
impl<T> From<Rc<dyn Fn(T)>> for Callback1<T> {
    fn from(rc: Rc<dyn Fn(T)>) -> Self { Callback1(Some(rc)) }
}
impl<T> From<Option<Rc<dyn Fn(T)>>> for Callback1<T> {
    fn from(opt: Option<Rc<dyn Fn(T)>>) -> Self { Callback1(opt) }
}
impl<T> From<()> for Callback1<T> {
    fn from(_: ()) -> Self { Callback1(None) }
}

/// Optional CSS class string. Accepts: `&str`, `String`, `()` (none).
#[derive(Default, Clone)]
pub struct OptClass(pub Option<String>);

impl From<&str> for OptClass {
    fn from(s: &str) -> Self { if s.is_empty() { OptClass(None) } else { OptClass(Some(s.to_string())) } }
}
impl From<String> for OptClass {
    fn from(s: String) -> Self { if s.is_empty() { OptClass(None) } else { OptClass(Some(s)) } }
}
impl From<Option<String>> for OptClass {
    fn from(opt: Option<String>) -> Self { OptClass(opt) }
}
impl From<Option<&str>> for OptClass {
    fn from(opt: Option<&str>) -> Self { OptClass(opt.filter(|s| !s.is_empty()).map(|s| s.to_string())) }
}
impl From<()> for OptClass {
    fn from(_: ()) -> Self { OptClass(None) }
}

/// Optional tuple of CSS class strings.
#[derive(Default, Clone)]
pub struct OptTuple(pub Option<(String, String)>);

impl From<(&str, &str)> for OptTuple {
    fn from(t: (&str, &str)) -> Self { OptTuple(Some((t.0.to_string(), t.1.to_string()))) }
}
impl From<(String, String)> for OptTuple {
    fn from(t: (String, String)) -> Self { OptTuple(Some(t)) }
}
impl From<()> for OptTuple {
    fn from(_: ()) -> Self { OptTuple(None) }
}

/// A callback that returns a View.
#[derive(Clone)]
pub struct ViewCallback(pub Option<Rc<dyn Fn() -> View>>);


impl Default for ViewCallback {
    fn default() -> Self { ViewCallback(None) }
}
impl<F: Fn() -> View + 'static> From<F> for ViewCallback {
    fn from(f: F) -> Self { ViewCallback(Some(Rc::new(f))) }
}
impl From<Rc<dyn Fn() -> View>> for ViewCallback {
    fn from(rc: Rc<dyn Fn() -> View>) -> Self { ViewCallback(Some(rc)) }
}
impl From<()> for ViewCallback {
    fn from(_: ()) -> Self { ViewCallback(None) }
}

/// A callback for route middleware that optionally returns a View to render instead of the component.
#[derive(Clone)]
pub struct MiddlewareCallback(pub Option<Rc<dyn Fn() -> Option<View>>>);

impl Default for MiddlewareCallback {
    fn default() -> Self { MiddlewareCallback(None) }
}
impl<F: Fn() -> Option<View> + 'static> From<F> for MiddlewareCallback {
    fn from(f: F) -> Self { MiddlewareCallback(Some(Rc::new(f))) }
}
impl From<Rc<dyn Fn() -> Option<View>>> for MiddlewareCallback {
    fn from(rc: Rc<dyn Fn() -> Option<View>>) -> Self { MiddlewareCallback(Some(rc)) }
}
impl From<()> for MiddlewareCallback {
    fn from(_: ()) -> Self { MiddlewareCallback(None) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use threadloom_core::render_to_string;

    #[test]
    fn test_button_disabled_prop() {
        let btn = Button(ButtonProps {
            label: "Click Me".to_string(),
            disabled: true,
            ..Default::default()
        });
        let html = render_to_string(&btn);
        assert!(html.contains("disabled=\"disabled\""));
        assert!(html.contains("aria-disabled=\"true\""));
        assert!(html.contains("opacity-50"));
        assert!(html.contains("cursor-not-allowed"));
    }

    #[test]
    fn test_alert_component() {
        let alert = Alert(AlertProps {
            title: "Info Notice".to_string(),
            description: "Everything is fine.".to_string(),
            variant: OptClass(Some("info".to_string())),
            ..Default::default()
        });
        let html = render_to_string(&alert);
        assert!(html.contains("role=\"alert\""));
        assert!(html.contains("Info Notice"));
        assert!(html.contains("Everything is fine."));
    }

    #[test]
    fn test_badge_component() {
        let badge = Badge(BadgeProps {
            label: "Beta".to_string(),
            variant: OptClass(Some("outline".to_string())),
            ..Default::default()
        });
        let html = render_to_string(&badge);
        assert!(html.contains("Beta"));
        assert!(html.contains("text-foreground"));
    }

    #[test]
    fn test_divider_component() {
        let divider = Divider(DividerProps {
            my: 6,
            ..Default::default()
        });
        let html = render_to_string(&divider);
        assert!(html.contains("<hr"));
        assert!(html.contains("my-6"));
    }

    #[test]
    fn test_divider_vertical_component() {
        let divider = Divider(DividerProps {
            orientation: "vertical".into(),
            mx: 4,
            ..Default::default()
        });
        let html = render_to_string(&divider);
        assert!(html.contains("role=\"separator\""));
        assert!(html.contains("aria-orientation=\"vertical\""));
        assert!(html.contains("border-l"));
        assert!(html.contains("mx-4"));
    }

    #[test]
    fn test_tooltip_component() {
        let tt = Tooltip(TooltipProps {
            content: "Helpful text".to_string(),
            children: vec![element("button").child(text("Hover me")).into_view()],
            ..Default::default()
        });
        let html = render_to_string(&tt);
        assert!(html.contains("tl-tooltip-wrapper"));
        assert!(html.contains("role=\"tooltip\""));
        assert!(html.contains("Helpful text"));
        assert!(html.contains("Hover me"));
    }

    #[test]
    fn test_tooltip_with_tooltip_text_prop() {
        let tt = Tooltip(TooltipProps {
            tooltip_text: "Legacy text".to_string(),
            children: vec![text("Item")],
            ..Default::default()
        });
        let html = render_to_string(&tt);
        assert!(html.contains("Legacy text"));
    }

    #[test]
    fn test_avatar_component() {
        let av_with_img = Avatar(AvatarProps {
            src: "https://example.com/user.jpg".into(),
            fallback: "JD".to_string(),
            ..Default::default()
        });
        let html = render_to_string(&av_with_img);
        assert!(html.contains("tl-avatar"));
        assert!(html.contains("src=\"https://example.com/user.jpg\""));
        assert!(html.contains("alt=\"JD\""));

        let av_fallback = Avatar(AvatarProps {
            fallback: "AK".to_string(),
            ..Default::default()
        });
        let html_fallback = render_to_string(&av_fallback);
        assert!(html_fallback.contains("tl-avatar-fallback"));
        assert!(html_fallback.contains("AK"));
    }

    #[test]
    fn test_docs_pagination_component() {
        let pag = DocsPagination(DocsPaginationProps {
            prev_title: "Installation".into(),
            prev_href: "/docs/installation".into(),
            next_title: "Client Core".into(),
            next_href: "/docs/client/core".into(),
            ..Default::default()
        });
        let html = render_to_string(&pag);
        assert!(html.contains("aria-label=\"Documentation pagination\""));
        assert!(html.contains("href=\"/docs/installation\""));
        assert!(html.contains("Installation"));
        assert!(html.contains("href=\"/docs/client/core\""));
        assert!(html.contains("Client Core"));
    }

    #[test]
    fn test_progress_component() {
        let prog = Progress(ProgressProps {
            value: 65.0,
            ..Default::default()
        });
        let html = render_to_string(&prog);
        assert!(html.contains("role=\"progressbar\""));
        assert!(html.contains("aria-valuenow=\"65\""));
    }

    #[test]
    fn test_separator_component() {
        let sep = Separator(SeparatorProps {
            orientation: "vertical".into(),
            ..Default::default()
        });
        let html = render_to_string(&sep);
        assert!(html.contains("role=\"separator\""));
        assert!(html.contains("tl-separator-vertical"));
    }

    #[test]
    fn test_card_component() {
        let card = Card(CardProps {
            title: "Settings".to_string(),
            wide: true,
            children: vec![text("Body content")],
            ..Default::default()
        });
        let html = render_to_string(&card);
        assert!(html.contains("tl-card"));
        assert!(html.contains("Settings"));
        assert!(html.contains("md:col-span-2"));
        assert!(html.contains("Body content"));
    }
}
