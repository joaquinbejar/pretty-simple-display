use pretty_simple_display::{DebugPretty, DebugSimple, DisplayPretty, DisplaySimple};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DebugPretty, DisplayPretty)]
struct User {
    id: u64,
    name: String,
    email: String,
    roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, DisplaySimple, DebugSimple)]
struct OrderItem {
    product_id: u64,
    quantity: u32,
    price: f64,
}

#[derive(Serialize, Deserialize, DebugPretty, DisplaySimple)]
enum OrderStatus {
    Pending,
    Processing,
    Shipped { tracking_number: String },
    Delivered,
    Cancelled { reason: String },
}

#[derive(Serialize, Deserialize, DebugPretty, DisplayPretty)]
struct Order {
    id: u64,
    user_id: u64,
    items: Vec<OrderItem>,
    status: OrderStatus,
    total: f64,
}

fn main() {
    // Create sample data
    let user = User {
        id: 1,
        name: "Alice Johnson".to_string(),
        email: "alice@example.com".to_string(),
        roles: vec!["admin".to_string(), "user".to_string()],
    };

    let items = vec![
        OrderItem {
            product_id: 101,
            quantity: 2,
            price: 29.99,
        },
        OrderItem {
            product_id: 102,
            quantity: 1,
            price: 15.50,
        },
    ];

    let order = Order {
        id: 12345,
        user_id: 1,
        items: items.clone(),
        status: OrderStatus::Shipped {
            tracking_number: "TRK123456789".to_string(),
        },
        total: 75.48,
    };

    println!("=== User Examples ===");
    println!("User Debug (Pretty): {:?}", user);
    println!("\nUser Display (Pretty): {}", user);

    println!("\n=== Order Item Examples ===");
    println!("OrderItem Debug (Simple): {:?}", &items[0]);
    println!("OrderItem Display (Simple): {}", &items[0]);

    println!("\n=== Order Examples ===");
    println!("Order Debug (Pretty): {:?}", order);
    println!("\nOrder Display (Pretty): {}", order);

    println!("\n=== Order Status Examples ===");
    let pending_status = OrderStatus::Pending;
    let cancelled_status = OrderStatus::Cancelled {
        reason: "Out of stock".to_string(),
    };

    println!("Pending Status Debug (Pretty): {:?}", pending_status);
    println!("Pending Status Display (Simple): {}", pending_status);

    println!("\nCancelled Status Debug (Pretty): {:?}", cancelled_status);
    println!("Cancelled Status Display (Simple): {}", cancelled_status);
}
