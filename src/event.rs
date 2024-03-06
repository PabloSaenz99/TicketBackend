pub struct Event<'a> {
	name: &'a str,
	total_tickets: u32,
	remaining_tickets: u32
}

impl <'a> Event<'a> {
	pub fn new(name: &str, total_tickets: u32) -> Event{
		Event{name, total_tickets, remaining_tickets: total_tickets}
	}

	pub fn get_name(&self) -> &str {
		self.name
	}

	pub fn get_total_tickets(&self) -> u32 {
		self.total_tickets
	}

	pub fn get_remaining_tickets(&self) -> u32 {
		self.remaining_tickets
	}

	pub fn set_remaining_tickets(&mut self, tickets: u32) {
		self.remaining_tickets = tickets;
	}
}