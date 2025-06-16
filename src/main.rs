mod models;
mod packer;
// use cfg_if::cfg_if;

// use gelf_logger::{GelfLogger, GelfLoggerBuilder};
use log::{info};
// use log::Level;
use models::item::Item;
use models::basebox::BoxConstraints;
use packer::packer::pack_items;

fn main() {
    // log::set_logger(console_log)
    // console_log::init_with_level(Level::Trace);
    //     let number = match number_str.parse::<i32>() {
    //     Ok(number)  => number,
    //     Err(e) => return Err(e),
    // };
    let items = vec![
        Item { id: 1, width: 100, height: 200, depth: 150, weight: 500, price: 1000 },
        Item { id: 2, width: 80, height: 100, depth: 100, weight: 300, price: 500 },
        Item { id: 3, width: 90, height: 150, depth: 100, weight: 700, price: 900 },
        Item { id: 4, width: 120, height: 200, depth: 120, weight: 600, price: 800 },
    ];

    info!("Сервіс запущено");
    let constraints = BoxConstraints {
        max_width: 300,
        max_height: 400,
        max_depth: 300,
        max_weight: 2000,
        max_price: 3000,
    };

    let packed = pack_items(items, constraints);

    for (i, b) in packed.iter().enumerate() {
        println!("Box {}: {:?}", i + 1, b.items.iter().map(|it| it.id).collect::<Vec<_>>());
    }
}