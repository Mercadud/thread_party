use thread_party::{participant_state::ParticipantStateBuilder, thread_party::ThreadParty};

#[derive(Clone, Debug)]
struct AnimalProperties {
    breed: String,
}

fn main() {
    println!("running sample");

    // start by creating the ThreadParty 
    // the types are for the types to store stuff into a hashmap usefull for threads that loop
    // last thing is the name of the current thread
    let mut thread_party = ThreadParty::<&str, AnimalProperties>::new("main");

    // add a thread by running this function
    thread_party.add_thread_participant(
        // name of the thread so other threads could send messages to
        "owner1",
        // the method that runs the thread
        // the thread_info param is used to access the thread storage and communicate to other threads
        |thread_info| {
            // you can store things in the threads own personal hashmap
            thread_info.store_data(
                "Jerry",
                AnimalProperties {
                    breed: "Dog".to_string(),
                },
            );

            println!(
                "Jerry is a: {}",
                thread_info.get_data("Jerry").unwrap().breed
            );

            // you can check the message boz to see if u have any incoming messages from other threads
            for i in thread_info.check_message_box() {
                println!("received: {:?}", i);
            }
        },
        // we can ignore that for now
        ParticipantStateBuilder::builder(),
    );

    // lets add another participent
    thread_party.add_thread_participant(
        // different name than any other thread
        "owner2",
        move |thread_info| {
            if !thread_info.shareable_data.contains_key("counter") {

            }
            // send data to another thread
            thread_info.send_data(
                // name of the thread
                // sending it to the first thread I made
                "owner1",
                // what to send
                AnimalProperties {
                    breed: "Labrador".to_string(),
                },
            );

            // same thing but with the main thread
            thread_info.send_data(
                "main",
                AnimalProperties {
                    breed: "Bird".to_string(),
                },
            );
        },
        // we can set the thread to loop at the specified fps
        // side note: set to 0 if you DON'T want it to loop
        // we will run this once every second
        ParticipantStateBuilder::builder().expected_fps(1),
    );

    // check for any messages from other threads
    for i in thread_party.check_message_box() {
        println!("main received: {:?}", i);
    }
}
