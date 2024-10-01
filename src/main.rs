#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::error::Error;

slint::include_modules!();

mod bprice;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match bprice::get_bitcoin_price() {
                Ok(price) => {
                    let price = format!("{:.2}",price);
                    ui.set_price(slint::SharedString::from(price));
                },
                Err(_) => {
                    println!("Failed to grab price");
                },
            }
        }
    });

    ui.run()?;
    Ok(())
}
