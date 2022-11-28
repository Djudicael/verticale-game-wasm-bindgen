use std::{cell::RefCell, rc::Rc};

use js_sys::Uint8Array;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::Document;

use crate::models::key::Key;
use crate::models::player::{Player, Position};
use crate::models::scaled_canvas::ScaledCanvas;
use crate::models::sprite::Sprite;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub async fn fetch_url_binary(url: String) -> Result<Uint8Array, JsValue> {
    let window = web_sys::window().unwrap(); // Browser window
    let promise = JsFuture::from(window.fetch_with_str(&url)); // File fetch promise
    let result = promise.await?; // Await fulfillment of fetch
    let response: web_sys::Response = result.dyn_into().unwrap(); // Type casting
    let image_data = JsFuture::from(response.array_buffer()?).await?; // Get text
    Ok(Uint8Array::new(&image_data))
}

pub async fn run_canvas(document: &Document) -> Result<(), JsValue> {
    // game-canvas
    let canvas = document.get_element_by_id("game-canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(1024);
    canvas.set_height(576);

    let scaled_canvas =
        ScaledCanvas::new((canvas.width() as f64) / 4., (canvas.height() as f64) / 4.);

    let mut context = Rc::new(RefCell::new(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
    ));
    let out_closure_context = context.clone();

    out_closure_context.borrow_mut().begin_path();

    let animate_f = Rc::new(RefCell::new(None));
    let animate_g = animate_f.clone();

    let player = Rc::new(RefCell::new(Player::new(Position { x: 0., y: 0. })));
    let player_closure = player.clone();

    let keys = Rc::new(RefCell::new(Key::new()));
    let keys_up = keys.clone();
    let keys_animate_closure = keys.clone();

    let binary = fetch_url_binary("http://localhost:8082/img/background.png".to_string()).await?;
    let altbuf = binary.to_vec();
    let image = image::load_from_memory_with_format(&altbuf, image::ImageFormat::Png).unwrap();

    let mut background = Sprite::new(Position { x: 0., y: 0. }, &image);
    // console::log_1(&vite.as_str().into());
    let closure_keydown = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let mut pl1 = player.borrow_mut();
        let mut keys_state = keys.borrow_mut();
        match event.key().as_str() {
            "d" => {
                keys_state.d.pressed = true;
            }
            "q" | "a" => {
                keys_state.q.pressed = true;
            }
            "z" | "w" => {
                pl1.velocity.y = -20.;
            }
            _ => {}
        }
    });
    let closure_keyup = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
        let mut keys_state = keys_up.borrow_mut();
        match event.key().as_str() {
            "d" => {
                keys_state.d.pressed = false;
            }
            "q" | "a" => {
                keys_state.q.pressed = false;
            }
            _ => {}
        }
    });

    window()
        .add_event_listener_with_callback("keydown", closure_keydown.as_ref().unchecked_ref())?;
    window().add_event_listener_with_callback("keyup", closure_keyup.as_ref().unchecked_ref())?;

    *animate_g.borrow_mut() = Some(Closure::new(move || {
        request_animation_frame(animate_f.borrow().as_ref().unwrap());
        let mut context_mutation = context.borrow_mut();
        context_mutation.set_fill_style(&"white".into());
        context_mutation.fill_rect(0., 0., canvas.width() as f64, canvas.height() as f64);
        let mut pl1 = player_closure.borrow_mut();
        let ks = keys_animate_closure.borrow();
        // context_mutation.save();
        context_mutation.scale(4., 4.).unwrap();


        let y_translation = -(background.image.height() as f64) + scaled_canvas.height;
        // let vite = format!("translation {}", y_translation);
        // console::log_1(&vite.into());
        context_mutation.translate(0., y_translation).unwrap_throw();
        background.update(&mut context_mutation).unwrap_throw();
        // context_mutation.restore();

        pl1.update(&mut context_mutation, &canvas);
        pl1.velocity.x = 0.;
        if ks.d.pressed {
            pl1.velocity.x = 1.;
        } else if ks.q.pressed {
            pl1.velocity.x = -1.;
        }
    }));

    closure_keydown.forget();
    closure_keyup.forget();
    request_animation_frame(animate_g.borrow().as_ref().unwrap());
    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}
