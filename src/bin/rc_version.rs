use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::to_string_pretty;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Serialize)]
struct Transaction {
    id: u16,
    transaction_date: DateTime<Utc>,
}

#[derive(Debug)]
struct Agent {
    id: u16,
    transactions_closed: i32,
    transactions_open: i32,
    transaction_queue: VecDeque<Transaction>,
}

impl Agent {
    fn new() -> Self {
        let mut rng = rand::thread_rng();

        Agent {
            id: rng.gen_range(0..=1000),
            transactions_closed: 0,
            transactions_open: 0,
            transaction_queue: VecDeque::from([
                Transaction {
                    id: 0001,
                    transaction_date: Utc::now(),
                },
                Transaction {
                    id: 0001,
                    transaction_date: Utc::now(),
                },
            ]),
        }
    }
}

impl Serialize for Agent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Agent", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("transactions_closed", &self.transactions_closed)?;
        state.serialize_field("transactions_open", &self.transactions_open)?;
        state.serialize_field("transaction_queue", &self.transaction_queue)?;
        state.end()
    }
}

fn main() {
    let agent = Rc::new(Agent::new());
    //Format output to JSON
    let json_agent = to_string_pretty(&*agent).expect("Failed to serialize agent");
    println!("Default agent: {}", json_agent);

    println!("Creating RC clone 1 for agent {}", agent.id);
    let agent_rc_clone1 = Rc::clone(&agent);
    println!("Creating RC clone 2 for agent {}", agent.id);
    let agent_rc_clone2 = Rc::clone(&agent);

    let json_agent_clone1 =
        serde_json::to_string_pretty(&*agent_rc_clone1).expect("Failed to serialize agent");
    println!("Default agent: {}", json_agent_clone1);
    let json_agent_clone2 =
        serde_json::to_string_pretty(&*agent_rc_clone2).expect("Failed to serialize agent");
    println!("Default agent: {}", json_agent_clone2);

    println!("Reference Count: {}", Rc::strong_count(&agent));
}
