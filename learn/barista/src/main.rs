
#[derive(Debug, PartialEq)]
enum Dish {
    Paella,
    Gazpacho,
    Croquetas
}

#[derive(Debug)]
struct Order {
    priority: u8,
    table: u8,
    halal: bool,
    description: String,
    dishes: Vec<Dish>,
}

impl Order {
    // Let's create a new one
    fn new( priority: u8, table: u8, halal: bool, desc: &str, dishes: Vec<Dish>) -> Self {
        let n_of_dishes = dishes.len() as u8;
        Self {
            priority: n_of_dishes,
            description: desc.to_string(),
            halal,
            dishes
        }
    }

    fn order_done (self) -> String {
        format!("Order for {} is done", self.table)
    }

    fn dish_done (&mut self, dish: Dish) {
        self.dishes.retain(|d| d != &dish);
        self.priority -= 1;
    }

    fn change_priotity(&mut self, reputation: &str) {
        if reputation == "good" {
            self.priority += 1;
        } else {
            self.priority -= 1;
        }
    }


}

fn main() {
    let single = vec![Dish::Paella];

    let feast = vec![Dish::Paella, Dish::Croquetas, Dish::Gazpacho];

    let my_order = Order::new(
        1, 
        true,
        "Take your time",
        feast
    );

    println!("{:?}", my_order)

}