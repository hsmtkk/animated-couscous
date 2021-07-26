enum Category {
    Food, // 0
    Book, // 1
    Clothing, // 2
    Misc, // 3
}

struct Item {
    category: Category,
    price: u32,
}

impl Item {
    fn new(category:Category, price:u32) -> Item {
        Item{category, price}
    }
}

fn solve(items:Vec<Item>) -> u32 {
    let mut point = 0;
    for item in items {
        let p: u32 = match item.category {
            Category::Food => { item.price * 5 / 100 },
            Category::Book => { item.price * 3 / 100 },
            Category::Clothing => { item.price * 2 / 100 },
            Category::Misc => { item.price / 100 },
        };
        point += p;
    }
    point
}

#[cfg(test)]
mod tests {
    use super::Item;
    use super::Category;
    #[test]
    fn test_solve0(){
        let want = 95;
        let inputs = vec![
            Item::new(Category::Food, 200),
            Item::new(Category::Book, 240),
            Item::new(Category::Food, 120),
            Item::new(Category::Misc, 460),
            Item::new(Category::Book, 240),
            Item::new(Category::Clothing, 3200),
        ];
        let got = super::solve(inputs);
        assert_eq!(want, got);
    }
}