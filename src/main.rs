mod server;
mod routes;

use server::servidor;

fn main(){
 servidor().expect("Failed to start server");
}
