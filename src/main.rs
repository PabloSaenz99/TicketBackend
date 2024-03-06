use std::rc::Rc;
use controller::Controller;
use event::Event;
use seat::Seat;
use ticket::Ticket;
use user::User;

pub mod ticket;
pub mod user;
pub mod event;
pub mod controller;
pub mod seat;

fn main() {

    Ticket::new(&Event::new("name", 10), &User::new("user".to_string()), &Seat::new("grade1", 1, 1));
	/* let event = Event::new("U2", 1000000);
	let mut controller = Controller::new(event, 5);
	for i in 0..1000 {
		controller.add_user(Rc::new(User::new(format!("{}{}", "User", i))));
	} */
}
