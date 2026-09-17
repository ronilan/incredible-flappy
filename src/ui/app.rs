use incredible::*;
use incredible_elements::{App, Link, Text};
use incredible_elements_text_fonts::{BlockCharsStr, BlockKind};
use incredible_helpers_effects::*;
use incredible_helpers_layout::*;
use incredible_helpers_styling::*;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct State {}

pub fn build_theme() {
    theme_rule::<Style>("Link", |s| {
        s.base.decor.underline.set(Some(UnderlineKind::Dotted));
        s.hovered.decor.background.set(Some(Color::from(9)));
        s.hovered.pointer.set(Some(PointerShape::Pointer));
    });

    transform_rule("GradientLabel", |flattened, progress| {
        gradient_color(
            &[
                Color::ansi(4),
                Color::ansi(5),
                Color::ansi(6),
                Color::ansi(4),
            ],
            GradientDirection::Horizontal,
            flattened,
            progress,
        )
    });
}

pub fn build() -> App<State> {
    let app = App::default();

    app.on_window(|el, _state, _event| {
        el.elements_flow_down(1);
        el.elements_to_center();

        if let Some(footer) = el
            .elements
            .cot_w::<Link<State>, _>(|e| e.get_handle() == "footer")
            .first()
        {
            let el_inside = &el.get_inside();
            footer.to_bottom_of(el_inside).to_center_x_of(el_inside);
        };
    });

    let logo = BlockCharsStr::default();
    logo.text("Incredible")
        .kind(BlockKind::Shadow)
        .style_handle("GradientLabel")
        .animation(Some(Animation::new(5000.0, 8.0, 1.0)));

    logo.on_mouse(|el, _state, event| {
        if let Mouse::Down = event.mouse {
            if let Some(mut anim) = el.get_animation() {
                anim.start_time = None;
                el.animation(Some(anim));
            }
        }
    });

    fn logo_effects(el: &BlockCharsStr<State>) {
        decorate_rules::<State, BlockCharsStr<State>>(el, logo_effects);
    }
    effect(&logo, logo_effects);

    let text = Text::default();
    text.faint(Some(true))
        .text("A Rust TUI Framework for the 2nd Quarter of the 21st Century");

    let footer = Link::default();
    footer
        .faint(Some(true))
        .text("Fabriqué au Canada : Made in Canada 🇨🇦")
        .handle("footer")
        .url("https://www.incredible.rs");

    app.add(logo);
    app.add(text);
    app.add(footer);

    build_theme();
    app
}
