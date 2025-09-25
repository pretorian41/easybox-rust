use crate::models::item::Item;
use crate::models::basebox::{BoxConstraints, BoxInstance};

pub fn pack_items(items: Vec<Item>, constraints: BoxConstraints) -> Vec<BoxInstance> {
    let mut boxes: Vec<BoxInstance> = Vec::new();

    for item in items {
        if let Some(existing_box) = boxes.iter_mut().find(|b| b.can_add(&item)) {
            existing_box.add(item); // move item у наявну коробку
        } else {
            let mut new_box = BoxInstance::new(BoxConstraints { ..constraints });
            new_box.add(item); // move item у нову коробку
            boxes.push(new_box);
        }
    }

    boxes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::item::Item;
    use crate::models::basebox::BoxConstraints;

    #[test]
    fn packs_items_into_one_box_if_possible() {
        let items = vec![
            Item { id: ("1".to_string()), width: 100, height: 100, depth: 100, weight: 200, price: 300 },
            Item { id: ("2".to_string()), width: 90, height: 100, depth: 90, weight: 100, price: 200 },
        ];

        let constraints = BoxConstraints {
            max_width: 200,
            max_height: 300,
            max_depth: 200,
            max_weight: 500,
            max_price: 1000,
        };

        let result = pack_items(items, constraints);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].items.len(), 2);
    }

    #[test]
    fn splits_items_into_two_boxes_when_needed() {
        let items = vec![
            Item { id: ("1".to_string()), width: 100, height: 200, depth: 100, weight: 500, price: 500 },
            Item { id: ("2".to_string()), width: 100, height: 200, depth: 100, weight: 600, price: 600 },
        ];

        let constraints = BoxConstraints {
            max_width: 200,
            max_height: 300,
            max_depth: 200,
            max_weight: 1000,
            max_price: 1000,
        };

        let result = pack_items(items, constraints);
        assert_eq!(result.len(), 2);
    }
}