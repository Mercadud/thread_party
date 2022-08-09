use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Connection<V>
{
    pub(crate) sender_channel: Sender<V>,
    pub(crate) receiver_name: String,
}
