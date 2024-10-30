use aspirin_eats::db::AspirinEatsDb;
use aspirin_eats::error::*;
use aspirin_eats::food::*;
use aspirin_eats::http::*;
use std::env;
use std::io::prelude::*;
use std::io::stdin;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::FromStr;

/// Change this path to match where you want to store the database file
const DB_PATH: &str = "/home/lwitten/aspirin-2024-03/assignments/04-networking/aspirin_eats.db";

fn read_stream(mut a_stream: TcpStream, mut buf: [u8; 1024]) -> Option<String> {
    match a_stream.read(&mut buf) {
        Ok(num_bytes) => {
            if num_bytes == 0 {
                println!("Connection closed by the server.");
                return None;
            }
            // Print the received message (convert buffer to string)
            let response = String::from_utf8_lossy(&buf[..num_bytes]).to_string();

            return Some(response);
        }
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return None;
        }
    }
}

fn process_database(db: &AspirinEatsDb, http_request: String) -> String {
    let request = HttpRequest::from_str(&http_request).expect("Failed to parse HTTP request");

    match (request.method.as_deref(), request.path.as_deref()) {
        (Some("GET"), Some("/orders")) => handle_get_all_orders(db),
        (Some("GET"), Some(path)) if path.starts_with("/orders/") => {
            handle_get_order_by_id(db, path)
        }
        (Some("POST"), Some("/orders")) => handle_post_order(db, request.body),
        (Some("DELETE"), Some("/orders")) => handle_delete_all_orders(db),
        (Some("DELETE"), Some(path)) if path.starts_with("/orders/") => {
            handle_delete_order_by_id(db, path)
        }
        (Some("GET"), Some("/")) => handle_welcome_message(),
        _ => handle_not_found(),
    }
}

// Handle GET /orders
fn handle_get_all_orders(db: &AspirinEatsDb) -> String {
    match db.get_all_orders() {
        Ok(orders) => {
            HttpResponse::new(200, "OK", &serde_json::to_string(&orders).unwrap()).to_string()
        }
        Err(_) => {
            HttpResponse::new(500, "Internal Server Error", "Failed to retrieve orders").to_string()
        }
    }
}

// Handle GET /orders/{id}
fn handle_get_order_by_id(db: &AspirinEatsDb, path: &str) -> String {
    let id_str = path.trim_start_matches("/orders/");
    if let Ok(id) = id_str.parse::<i64>() {
        match db.get_order(id) {
            Ok(Some(order)) => {
                HttpResponse::new(200, "OK", &serde_json::to_string(&order).unwrap()).to_string()
            }
            Ok(None) => HttpResponse::new(404, "Not Found", "Order not found").to_string(),
            Err(_) => HttpResponse::new(500, "Internal Server Error", "Failed to retrieve order")
                .to_string(),
        }
    } else {
        HttpResponse::new(400, "Bad Request", "Invalid order ID").to_string()
    }
}

// Handle POST /orders
fn handle_post_order(db: &AspirinEatsDb, body: Option<String>) -> String {
    if let Some(body) = body {
        match serde_json::from_str::<OrderRequest>(&body) {
            Ok(order_request) => {
                let new_order: Order = order_request.into();
                match db.add_order(new_order) {
                    Ok(_) => {
                        HttpResponse::new(201, "Created", "Order added successfully").to_string()
                    }
                    Err(_) => {
                        HttpResponse::new(500, "Internal Server Error", "Failed to add order")
                            .to_string()
                    }
                }
            }
            Err(_) => HttpResponse::new(400, "Bad Request", "Invalid order format").to_string(),
        }
    } else {
        HttpResponse::new(400, "Bad Request", "Missing request body").to_string()
    }
}

// Handle DELETE /orders
fn handle_delete_all_orders(db: &AspirinEatsDb) -> String {
    match db.reset_orders() {
        Ok(_) => HttpResponse::new(200, "OK", "All orders deleted").to_string(),
        Err(_) => {
            HttpResponse::new(500, "Internal Server Error", "Failed to delete orders").to_string()
        }
    }
}

// Handle DELETE /orders/{id}
fn handle_delete_order_by_id(db: &AspirinEatsDb, path: &str) -> String {
    let id_str = path.trim_start_matches("/orders/");
    if let Ok(id) = id_str.parse::<i64>() {
        match db.remove_order(id) {
            Ok(_) => HttpResponse::new(200, "OK", "Order deleted").to_string(),
            Err(_) => HttpResponse::new(500, "Internal Server Error", "Failed to delete order")
                .to_string(),
        }
    } else {
        HttpResponse::new(400, "Bad Request", "Invalid order ID").to_string()
    }
}

// Handle GET /
fn handle_welcome_message() -> String {
    HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!").to_string()
}

// Handle 404 Not Found
fn handle_not_found() -> String {
    HttpResponse::new(404, "Not Found", "Resource not found").to_string()
}

fn main() {
    let db = AspirinEatsDb::from_path(DB_PATH).expect("Failed to open database");

    //from reverse proxy
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Usage: {} <proxy-from> <proxy-to>", args[0]);
        std::process::exit(2);
    }

    let proxy_addr = &args[1];
    let origin_addr = &args[2];

    //tcp trial code

    let listener = TcpListener::bind(origin_addr).expect("failed bind");
    let mut buf = [0; 1024];
    let proxy_addr = &args[1];
    let origin_addr = &args[2];

    //server code
    loop {
        // listen for proxy
        for stream in listener.incoming() {
            match stream {
                Ok(mut a_stream) => {
                    //read from proxy
                    let incoming = match read_stream(
                        a_stream.try_clone().expect("Failed to clone stream"),
                        buf,
                    ) {
                        Some(incoming) => {
                            println!("incoming request is: {}", incoming);
                            incoming // Keep the incoming string
                        }
                        None => {
                            println!("No response incoming");
                            continue; // Skip to the next iteration if inside a loop
                        }
                    };

                    // process incoming

                    //end of working stream
                    //echo response to user

                    let outgoing = process_database(&db, incoming);

                    //writeback to proxy
                    if let Err(e) = a_stream.write(outgoing.as_bytes()) {
                        eprintln!("Failed to write back to user: {}", e);
                        continue;
                    }
                }
                Err(e) => {
                    panic!("couldn't get a valid stream")
                }
            }
        }
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use aspirin_eats::db::AspirinEatsDb;
    use aspirin_eats::food::{Bun, Burger, MenuItem, OrderRequest, Patty, Topping};

    fn setup_test_db() -> AspirinEatsDb {
        AspirinEatsDb::in_memory().unwrap()
    }

    // Helper function to easily create an order request for testing
    fn create_test_order_request() -> String {
        r#"
        {
            "customer": "John Doe",
            "food": [
                {
                    "Burger": {
                        "bun": "Sesame",
                        "patty": "Beef",
                        "toppings": ["Cheese", "Bacon"]
                    }
                },
                "Fries",
                "Drink"
            ]
        }"#
        .to_string()
    }

    /// Test the process_database function (integration tests)
    #[test]
    fn test_process_database_welcome_message() {
        let db = setup_test_db();
        let request = "GET / HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, request);
        assert_eq!(response, "HTTP/1.1 200 OK\r\n\r\nWelcome to Aspirin Eats!");
    }

    #[test]
    fn test_process_database_add_order() {
        let db = setup_test_db();
        let request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        let response = process_database(&db, request);
        assert_eq!(
            response,
            "HTTP/1.1 201 Created\r\n\r\nOrder added successfully"
        );
    }

    #[test]
    fn test_process_database_get_all_orders() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Get all orders
        let get_request = "GET /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, get_request);

        assert!(response.contains("\"customer\":\"John Doe\""));
        assert!(response.contains("\"Fries\""));
    }

    #[test]
    fn test_process_database_get_order_by_id() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Get the order by ID (assuming it's ID 1)
        let get_request = "GET /orders/1 HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, get_request);

        assert!(response.contains("\"customer\":\"John Doe\""));
        assert!(response.contains("\"Burger\""));
    }

    #[test]
    fn test_process_database_delete_all_orders() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Delete all orders
        let delete_request = "DELETE /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, delete_request);
        assert_eq!(response, "HTTP/1.1 200 OK\r\n\r\nAll orders deleted");

        // Ensure the orders are deleted
        let get_request = "GET /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, get_request);
        assert!(response.contains("[]")); // Should return an empty list
    }

    #[test]
    fn test_process_database_delete_order_by_id() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Delete the order by ID (assuming it's ID 1)
        let delete_request =
            "DELETE /orders/1 HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, delete_request);
        assert_eq!(response, "HTTP/1.1 200 OK\r\n\r\nOrder deleted");

        // Ensure the order is deleted
        let get_request = "GET /orders/1 HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, get_request);
        assert!(response.contains("Order not found"));
    }

    #[test]
    fn test_process_database_invalid_request() {
        let db = setup_test_db();

        // Invalid path
        let invalid_path_request =
            "GET /invalid_path HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, invalid_path_request);
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nResource not found");

        // Invalid method
        let invalid_method_request =
            "PUT /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, invalid_method_request);
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nResource not found");
    }

    // Unit tests for individual helper functions
    #[test]
    fn test_handle_get_all_orders() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Test handle_get_all_orders
        let response = handle_get_all_orders(&db);
        assert!(response.contains("\"customer\":\"John Doe\""));
        assert!(response.contains("\"Fries\""));
    }

    #[test]
    fn test_handle_get_order_by_id() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Test handle_get_order_by_id with valid ID
        let response = handle_get_order_by_id(&db, "/orders/1");
        assert!(response.contains("\"customer\":\"John Doe\""));

        // Test handle_get_order_by_id with invalid ID
        let response = handle_get_order_by_id(&db, "/orders/999");
        assert!(response.contains("Order not found"));
    }

    #[test]
    fn test_handle_post_order() {
        let db = setup_test_db();

        // Test handle_post_order with valid request body
        let body = Some(create_test_order_request());
        let response = handle_post_order(&db, body);
        assert_eq!(
            response,
            "HTTP/1.1 201 Created\r\n\r\nOrder added successfully"
        );

        // Test handle_post_order with invalid request body
        let invalid_body = Some("invalid json".to_string());
        let response = handle_post_order(&db, invalid_body);
        assert_eq!(
            response,
            "HTTP/1.1 400 Bad Request\r\n\r\nInvalid order format"
        );
    }

    #[test]
    fn test_handle_delete_all_orders() {
        let db = setup_test_db();

        // Add an order first
        let add_request = format!(
            "POST /orders HTTP/1.1\r\nHost: localhost:8080\r\nContent-Type: application/json\r\n\r\n{}",
            create_test_order_request()
        );
        process_database(&db, add_request); // Add order

        // Test handle_delete_all_orders
        let response = handle_delete_all_orders(&db);
        assert_eq!(response, "HTTP/1.1 200 OK\r\n\r\nAll orders deleted");

        // Ensure the orders are deleted
        let get_request = "GET /orders HTTP/1.1\r\nHost: localhost:8080\r\n\r\n".to_string();
        let response = process_database(&db, get_request);
        assert!(response.contains("[]")); // Should return an empty list
    }

    #[test]
    fn test_handle_welcome_message() {
        let response = handle_welcome_message();
        assert_eq!(response, "HTTP/1.1 200 OK\r\n\r\nWelcome to Aspirin Eats!");
    }

    #[test]
    fn test_handle_not_found() {
        let response = handle_not_found();
        assert_eq!(response, "HTTP/1.1 404 Not Found\r\n\r\nResource not found");
    }
}
