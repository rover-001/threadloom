use threadloom::{threadloom, render_to_string, create_signal};

#[test]
fn test_macro_event_bindings() {
    let view = threadloom! {
        div(
            id="interactive-box",
            class="p-4 border",
            on_dblclick=|| {},
            on_scroll=|| {},
            on_context_menu=|| {},
            on_copy=|| {},
            on_paste=|| {},
            on_cut=|| {},
            on_drag_start=|| {},
            on_drag_over=|| {},
            on_drag_enter=|| {},
            on_drag_leave=|| {},
            on_drag_end=|| {},
            on_drop=|| {},
            on_key_up=|| {},
            on_wheel=|| {},
        ) {
            span { "Interactive element" }
        }
    };
    let html = render_to_string(&view);
    assert!(html.contains("id=\"interactive-box\""));
    assert!(html.contains("class=\"p-4 border\""));
    assert!(html.contains("<span"));
    assert!(html.contains("Interactive element"));
}

#[test]
fn test_macro_colon_syntax_event_bindings() {
    let view = threadloom! {
        div(
            on:dblclick=|| {},
            on:scroll=|| {},
            on:contextmenu=|| {},
            on:copy=|| {},
            on:paste=|| {},
            on:cut=|| {},
            on:dragstart=|| {},
            on:dragover=|| {},
            on:dragenter=|| {},
            on:dragleave=|| {},
            on:dragend=|| {},
            on:drop=|| {},
        ) {
            "Colon events"
        }
    };
    let html = render_to_string(&view);
    assert!(html.contains("Colon events"));
}

#[test]
fn test_macro_reactive_rendering() {
    let (count, set_count) = create_signal(10);

    let view = threadloom! {
        div(class="counter") {
            span { { count.get().to_string() } }
        }
    };
    let html = render_to_string(&view);
    assert!(html.contains("10"));

    set_count.set(20);
    assert_eq!(count.get(), 20);
}
