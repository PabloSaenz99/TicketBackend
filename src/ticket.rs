
use seat::Seat;
// use utils_lib::Getter;

use crate::{event::Event, seat, user::User};

// #[derive(Getter)]
pub struct Ticket<'a> {
	user: &'a User<'a>,
	event: &'a Event<'a>,
	seat: &'a Seat<'a>,
}

impl <'a> Ticket<'a> {
	pub fn new(event: &'a Event<'a>, user: &'a User<'a>, seat: &'a Seat<'a>) -> Ticket<'a>{
		let t = Ticket{event, user, seat };
        // print!("{}", t.describe());
        t
	}
}