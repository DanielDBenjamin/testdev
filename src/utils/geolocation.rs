#[derive(Debug, Clone, Copy)]
pub struct BrowserLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub accuracy: Option<f64>,
}

#[cfg(target_arch = "wasm32")]
pub async fn get_current_location() -> Result<BrowserLocation, String> {
    use futures::channel::oneshot;
    use leptos::wasm_bindgen::prelude::*;
    use leptos::wasm_bindgen::JsCast;

    let window = web_sys::window().ok_or_else(|| "No window available".to_string())?;
    let navigator = window.navigator();
    let geolocation = navigator
        .geolocation()
        .map_err(|_| "Geolocation not supported".to_string())?;

    let mut options = web_sys::PositionOptions::new();
    options.set_enable_high_accuracy(true);
    options.set_timeout(10_000);
    options.set_maximum_age(0);

    let (sender, receiver) = oneshot::channel();
    let sender = std::rc::Rc::new(std::cell::RefCell::new(Some(sender)));

    let success_sender = sender.clone();
    let success_callback = Closure::once(move |position: web_sys::Position| {
        let coords = position.coords();
        if let Some(tx) = success_sender.borrow_mut().take() {
            let location = BrowserLocation {
                latitude: coords.latitude(),
                longitude: coords.longitude(),
                accuracy: Some(coords.accuracy()),
            };
            let _ = tx.send(Ok(location));
        }
    });

    let error_sender = sender.clone();
    let error_callback = Closure::once(move |error: web_sys::PositionError| {
        if let Some(tx) = error_sender.borrow_mut().take() {
            let _ = tx.send(Err(error.message()));
        }
    });

    geolocation
        .get_current_position_with_error_callback_and_options(
            success_callback.as_ref().unchecked_ref(),
            Some(error_callback.as_ref().unchecked_ref()),
            &options,
        )
        .map_err(|_| "Failed to request position".to_string())?;

    success_callback.forget();
    error_callback.forget();

    receiver
        .await
        .map_err(|_| "Location request cancelled".to_string())?
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn get_current_location() -> Result<BrowserLocation, String> {
    Err("Geolocation is only available in the browser".to_string())
}
