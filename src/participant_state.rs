use std::{
    collections::HashMap,
    hash::Hash,
    sync::mpsc::{Receiver, Sender},
};

use crate::connection::Connection;

pub struct ParticipantState<K, V>
where
    K: Hash + Eq,
{
    pub(crate) thread_loop: bool,
    pub(crate) expected_fps: u32,
    pub shareable_data: HashMap<K, V>,
    pub(crate) senders: Vec<Connection<V>>,
    pub(crate) receiver: Receiver<V>,
}

impl<K, V: Clone> ParticipantState<K, V>
where
    K: Hash + Eq,
{
    pub fn end_loop(&mut self) {
        self.thread_loop = false;
    }

    pub fn get_data(&self, locator: K) -> Option<&V> {
        self.shareable_data.get(&locator)
    }

    pub fn get_mut_data(&mut self, locator: K) -> Option<&mut V> {
        self.shareable_data.get_mut(&locator)
    }

    pub fn remove_data(&mut self, locator: K) -> Option<V> {
        self.shareable_data.remove(&locator)
    }

    pub fn store_data(&mut self, locator: K, data: V) {
        self.shareable_data.insert(locator, data);
    }

    pub fn send_data(&self, participant_name: &str, data: V) {
        for i in &self.senders {
            if i.receiver_name.to_string() == participant_name {
                i.sender_channel.send(data.clone()).unwrap();
            }
        }
    }

    pub fn check_message_box(&mut self) -> &Receiver<V> {
        &self.receiver
    }
}

pub struct ParticipantStateBuilder {
    pub(crate) expected_fps: u32,
}

impl Default for ParticipantStateBuilder {
    fn default() -> Self {
        Self { expected_fps: 0 }
    }
}

impl ParticipantStateBuilder {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn expected_fps(mut self, fps: u32) -> Self {
        self.expected_fps = fps;
        self
    }
}
