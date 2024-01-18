extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;

use web_sys_main_loop::FrameState;
use yew::prelude::*;
use yew_router::prelude::*;

mod canvas;
mod home;
mod info;
mod life;
mod raw_draw;

#[derive(Routable, Clone, Copy, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/info")]
    Info,
}

fn switch(route: Route, on_frame: Rc<RefCell<Vec<Callback<FrameState>>>>) -> Html {
    match route {
        Route::Home => html! { <home::HomeView on_frame={on_frame} /> },
        Route::Info => html! { <info::InfoView /> },
    }
}

#[function_component]
fn App() -> Html {
    let on_frame = Rc::new(RefCell::new(vec![]));

    let switcher = {
        let callbacks = Rc::clone(&on_frame);
        Callback::from(move |r| switch(r, Rc::clone(&callbacks)))
    };

    let window = web_sys::window().expect("Can't get the window instance");

    web_sys_main_loop::start(&window, move |frame_state| {
        for callback in RefCell::borrow(&on_frame).iter() {
            callback.emit(FrameState {
                pressed_keys: frame_state.pressed_keys.clone(),
                mouse_state: frame_state.mouse_state,
            });
        }
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={switcher}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
