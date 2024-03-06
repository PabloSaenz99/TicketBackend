use uuid::Uuid;

use crate::ticket::Ticket;

pub struct User<'a> {
	id: Uuid,
	name: String,
	tickets: Vec<Ticket<'a>>
}

impl <'a> User<'a> {
	pub fn new(name: String) -> User<'a>{
		User{ id: Uuid::new_v4(), name, tickets: Vec::new() }
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_id(&self) -> Uuid{
		self.id
	}

	pub fn get_n_tickets(&self) -> usize{
		self.tickets.len()
	}

    pub fn add_ticket(&mut self, ticket: Ticket<'a>) {
        self.tickets.push(ticket);
    }
}