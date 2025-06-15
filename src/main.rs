mod models;

use models::item::Item;
use models::basebox::BoxConstraints;

fn main() {
    let items = vec![
        Item { id: 1, width: 100, height: 200, depth: 150, weight: 500, price: 1000 },
        Item { id: 2, width: 80, height: 120, depth: 100, weight: 300, price: 700 },
    ];

    for item in &items {
        println!("{:?}", item);
    }

    let constraints = BoxConstraints {
        max_width: 300,
        max_height: 300,
        max_depth: 300,
        max_weight: 2000,
        max_price: 3000,
    };

    println!("Box constraints: {:?}", constraints);
}
