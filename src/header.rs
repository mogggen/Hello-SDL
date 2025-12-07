// Define states as an enum
#[derive(Debug)]
enum State {
    Initial,
    Intermediate,
    Final,
}

// Define events as an enum
#[derive(Debug)]
enum Event {
    Start,
    Continue,
    Finish,
}

// Define the state machine struct
struct StateMachine {
    state: State,
}

// Implement methods for the state machine
impl StateMachine {
    fn new() -> StateMachine {
        StateMachine { state: State::Initial }
    }

    fn process_event(&mut self, event: Event) {
        match (self.state, event) {
            (State::Initial, Event::Start) => {
                println!("Transitioning from Initial to Intermediate");
                self.state = State::Intermediate;
            }
            (State::Intermediate, Event::Continue) => {
                println!("Continuing in Intermediate state");
            }
            (State::Intermediate, Event::Finish) => {
                println!("Transitioning from Intermediate to Final");
                self.state = State::Final;
            }
            _ => {
                println!("Invalid state transition");
            }
        }
    }

    fn current_state(&self) -> &State {
        &self.state
    }
}

fn main() {
    let mut state_machine = StateMachine::new();

    println!("Current State: {:?}", state_machine.current_state());

    state_machine.process_event(Event::Start);
    println!("Current State: {:?}", state_machine.current_state());

    state_machine.process_event(Event::Continue);
    println!("Current State: {:?}", state_machine.current_state());

    state_machine.process_event(Event::Finish);
    println!("Current State: {:?}", state_machine.current_state());

    state_machine.process_event(Event::Finish); // Invalid transition
    println!("Current State: {:?}", state_machine.current_state());
}
