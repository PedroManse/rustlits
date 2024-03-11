mod views;

fn main() {
	println!("index {:?}\n", views::index());
	println!("list users {:?}", views::list_users());
}
