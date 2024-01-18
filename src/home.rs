use alloc::rc::Rc;
use core::{cell::RefCell, cmp};

use web_sys::js_sys::Math::random;
use web_sys_main_loop::{FrameState, MouseButton};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    canvas, life,
    raw_draw::{RawColor, RawDraw},
    Route,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_frame: Rc<RefCell<Vec<Callback<FrameState>>>>,
}

#[function_component]
pub fn HomeView(props: &Props) -> Html {
    html! {
        <>
            <main class="container">
                <hgroup>
                    <h1>{"Welcome!"}</h1>
                    <p>
                        {"This website has been written in "}
                        <a href="https://www.rust-lang.org/" target="_blank">{"Rust"}</a>
                        {" using "}
                        <a href="https://yew.rs/" target="_blank">{"Yew"}</a>
                        {"."}
                    </p>
                </hgroup>

                <GameOfLifeCard on_frame={Rc::clone(&props.on_frame)} />

                <BottomButtonBar />
            </main>
        </>
    }
}

#[function_component]
fn GameOfLifeCard(props: &Props) -> Html {
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = 200;

    let on_frame = Rc::clone(&props.on_frame);
    let on_close = Callback::from(move |()| {
        RefCell::borrow_mut(&on_frame).pop();
    });

    let on_frame = Rc::clone(&props.on_frame);
    let on_context = Callback::from(move |(context, canvas): (_, web_sys::HtmlCanvasElement)| {
        let life = RefCell::new(life::Life::new(WIDTH, HEIGHT));

        for y in 1..HEIGHT - 1 {
            for x in 1..WIDTH - 1 {
                let mut life = RefCell::borrow_mut(&life);
                life.put_buf(x, y, RawColor::from_single(0));
                if random() > 0.5 {
                    life.set(x, y);
                }
            }
        }

        RefCell::borrow_mut(&on_frame).push(Callback::from(move |frame_state: FrameState| {
            let mut life = RefCell::borrow_mut(&life);
            let (x, y) = frame_state.mouse_state.get_position();

            if let Some(buttons) = frame_state.mouse_state.get_pressed_buttons() {
                if buttons.contains(&MouseButton::Left) {
                    let rect = canvas.get_bounding_client_rect();
                    let x = (f64::from(x) - rect.x()) / rect.width();
                    let y = (f64::from(y) - rect.y()) / rect.height();

                    if x > 0.0 && x < 1.0 && y > 0.0 && y < 1.0 {
                        unsafe {
                            let x_start: u32 =
                                (x * f64::from(WIDTH) - 5.0).max(1.0).to_int_unchecked();
                            let y_start: u32 =
                                (y * f64::from(HEIGHT) - 5.0).max(1.0).to_int_unchecked();
                            for y in y_start..cmp::min(y_start + 10, HEIGHT - 1) {
                                for x in x_start..cmp::min(x_start + 10, WIDTH - 1) {
                                    if life.get_buf(x, y).r == 0 {
                                        life.set(x, y);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            canvas::redraw(&context, &*life);
            life.tick();
        }));
    });

    html! {
        <article class="py-8">
            <h1>{"Game of Life"}</h1>
            <p>{"Conceived by John Horton Conway in 1970, the Game of Life is a captivating cellular automaton with the ability to simulate a universal constructor or any other Turing machine."}</p>

            <p>{"My own 'Game of Life' began in the small town of Vitez. The starting point might not seem important, but it laid the foundation for a great journey. In those early days, programming was a foreign concept to me, and I lacked any interest in it. Everything changed when I stepped into highschool. It marked my initiation into a world of competition, where skillful individuals surrounded me. For the first time, I felt a genuine motivation to improve and delve into the realm of programming."}</p>

            <p>{"As my curiosity for programming blossomed, I found myself immersed in a world of algorithms and coding challenges. It was a journey filled with late-night debugging sessions, countless lines of code, and the thrill of solving problems. From mastering the basics to exploring advanced concepts, each coding challenge became a stepping stone in my evolution as a developer. The Game of Life, both Conway's and my own, reflects the dynamic and ever-changing nature of this exciting journey."}</p>

            <canvas::Canvas width={WIDTH} height={HEIGHT} {on_context} {on_close} />
        </article>
    }
}

const _: &str = r#"class="block hidden"#;

#[function_component]
fn BottomButtonBar() -> Html {
    let msg = use_state(|| "Would you like to see something cool?");
    let display = use_state(|| "block");

    let hide_no_button = {
        let msg = msg.clone();
        let display = display.clone();
        Callback::from(move |_| {
            msg.set("Not an option");
            display.set("hidden");
        })
    };

    let go_to_info = {
        let navigator = use_navigator().expect("no navigator found");
        Callback::from(move |_| navigator.push(&Route::Info))
    };

    html! {
        <article>
            <h2>{*msg}</h2>
            <div class="grid">
                <button onclick={go_to_info} role="button" class="my-2">{"Yes"}</button>
                <button onclick={hide_no_button} class={format!("my-2 secondary {}", *display)}>{"No"}</button>
            </div>
        </article>
    }
}
