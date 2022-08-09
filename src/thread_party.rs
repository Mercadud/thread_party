use std::{
    collections::HashMap,
    hash::Hash,
    sync::mpsc::{self, Receiver, Sender},
    thread::{self, JoinHandle},
    time::Duration, any::Any,
};

use spin_sleep::LoopHelper;

use crate::{
    connection::Connection,
    participant_state::{ParticipantState, ParticipantStateBuilder},
};

type V = Box<dyn Any + 'static>;

pub trait Shareable: Any {}


pub struct ThreadParty<K>
{
    thread_name: String,
    data: HashMap<K, Box<dyn 'static + Shareable>>,
    receiver: Receiver<V>,
    threads: Vec<ThreadParticipant>,
    connections: Vec<Connection<V>>,
}

impl<K> ThreadParty<K> {
    pub fn new(main_thread_name: &str) -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            thread_name: main_thread_name.to_string(),
            data: HashMap::new(),
            receiver: rx,
            threads: Vec::new(),
            connections: vec![Connection {
                sender_channel: tx,
                receiver_name: main_thread_name.to_string(),
            }],
        }
    }

    pub fn add_thread_participant<F>(
        &mut self,
        thread_name: &str,
        thread_method: F,
        thread_config: ParticipantStateBuilder,
    ) where
        F: Fn(&mut ParticipantState<K, V>) + std::marker::Send + std::marker::Sync + 'static,
        K: Hash + Eq,
    {
        let (rx, receiver) = mpsc::channel();
        let mut senders = Vec::new();
        senders.extend_from_slice(&self.connections[0..]);
        // senders.clone_from_slice(&self.connections[0..]);

        self.connections.push(Connection {
            sender_channel: rx,
            receiver_name: thread_name.to_string(),
        });

        self.threads.push(ThreadParticipant {
            thread_name: thread_name.to_string(),
            thread: thread::spawn(move || {
                let mut participant_state = ParticipantState {
                    thread_loop: thread_config.expected_fps != 0,
                    expected_fps: thread_config.expected_fps,
                    shareable_data: HashMap::new(),
                    senders,
                    receiver,
                };

                if participant_state.thread_loop {
                    let mut looper = LoopHelper::builder()
                        .report_interval(Duration::from_millis(
                            (1000 / participant_state.expected_fps) as u64,
                        ))
                        .build_with_target_rate(participant_state.expected_fps);

                    while participant_state.thread_loop {
                        looper.loop_start();

                        thread_method(&mut participant_state);

                        looper.loop_sleep();
                    }
                } else {
                    thread_method(&mut participant_state);
                }
            }),
        })
    }

    pub fn check_message_box(&mut self) -> &Receiver<V> {
        &self.receiver
    }
}

impl<K> Drop for ThreadParty<K> {
    fn drop(&mut self) {
        while !self.threads.is_empty() {
            let participant = self.threads.pop().unwrap();
            participant.thread.join().unwrap();
        }
    }
}

pub struct ThreadParticipant {
    thread_name: String,
    thread: JoinHandle<()>,
}
