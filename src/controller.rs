use std::{collections::HashMap, rc::Rc};

use queues::{queue, IsQueue, Queue};
use uuid::Uuid;

use crate::{event::Event, user::User};

pub struct Controller<'a> {
	max_sim_users: u32,
	current_users: HashMap<Uuid, Rc<User<'a>>>,

	event: Event<'a>,
	queue: Queue<Rc<User<'a>>>
}

impl <'a> Controller<'a> {
	pub fn new(event: Event, max_sim_users: u32) -> Controller{
		Controller{max_sim_users, current_users: HashMap::with_capacity(max_sim_users.try_into().unwrap()), event, queue: queue![]}
	}

	pub fn add_user(&mut self, user: Rc<User<'a>>) {
		if self.queue.add(user).is_ok() {
			// if self.current_users.len() < self.max_sim_users.try_into().unwrap() {
			if self.current_users.len().checked_sub(self.max_sim_users.try_into().unwrap()).is_none() {
				self.move_from_queue_to_sale();
			}
		}
	}

	pub fn remove_user_from_sale(&mut self, user: Rc<User<'a>>) {
		match self.current_users.remove(&user.get_id()) {
			Some(old_user) => {
				// self.remaining_tickets =- old_user.get_n_tickets();
				if self.event.get_remaining_tickets().checked_sub(old_user.get_n_tickets().try_into().unwrap()).is_some() {
					self.move_from_queue_to_sale();
				}
			},
			None => todo!(),
		}
	}

	fn remove_user_from_queue(&mut self) -> Result<Rc<User<'a>>, &str>{
		self.queue.remove()
	}

	fn move_from_queue_to_sale(&mut self) {
		match self.remove_user_from_queue() {
			Ok(new_user) => {
				self.current_users.insert(new_user.get_id(), new_user);
			},
			Err(_) => todo!(),
		}
	}
}