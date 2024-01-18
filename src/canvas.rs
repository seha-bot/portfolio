use web_sys::wasm_bindgen::{self, JsCast};
use yew::prelude::*;

use crate::raw_draw;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub width: u32,
    pub height: u32,
    pub on_context: Callback<(
        web_sys::CanvasRenderingContext2d,
        web_sys::HtmlCanvasElement,
    )>,
    pub on_close: Callback<()>,
}

pub fn redraw<'buf, T: raw_draw::RawDraw<'buf>>(
    context: &web_sys::CanvasRenderingContext2d,
    raw_draw: &'buf T,
) {
    let image_data = web_sys::ImageData::new_with_u8_clamped_array(
        wasm_bindgen::Clamped(raw_draw.buf()),
        raw_draw.width(),
    )
    .expect("can't convert a buffer to image data");

    context
        .put_image_data(&image_data, 0.0, 0.0)
        .expect("can't put image data to canvas");
}

fn init_canvas(
    element_ref: &NodeRef,
    width: u32,
    height: u32,
) -> Option<(
    web_sys::HtmlCanvasElement,
    web_sys::CanvasRenderingContext2d,
)> {
    let canvas = element_ref.cast::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .ok()?;

    canvas.set_width(width);
    canvas.set_height(height);
    Some((canvas, context))
}

#[function_component]
pub fn Canvas(props: &Props) -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        let width = props.width;
        let height = props.height;
        let on_context = props.on_context.clone();
        let on_close = props.on_close.clone();
        use_effect(move || {
            let (canvas, context) =
                init_canvas(&canvas_ref, width, height).expect("failed to init canvas");
            on_context.emit((context, canvas));

            move || on_close.emit(())
        });
    };

    html! {
        <canvas class="w-4/5 mx-auto block" style="width=500;image-rendering: pixelated" ref={canvas_ref} />
    }
}
