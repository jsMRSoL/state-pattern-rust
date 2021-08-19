use std::cell::RefCell;

enum State {
    StartState,
    StopState,
}

impl State {
    fn do_action<'a>(&'a self, context: &'a Context<'a>) {
	match self {
	    Self::StartState => self.do_start_action(context),
	    Self::StopState => self.do_stop_action(context),
	}
	
    }

    fn do_start_action<'a>(&'a self, context: &'a Context<'a>) {
	println!("Player is in start state.");
	context.set_state(self)
    }

    fn do_stop_action<'a>(&'a self, context: &'a Context<'a>) {
	println!("Player is in start state.");
	context.set_state(self)
    }

    fn to_string(&self) -> String {
	match self {
	    Self::StartState => "Start State".to_string(),
	    Self::StopState => "Stop State".to_string(),
	}
    }
   
}

struct Context<'a> {
    state: RefCell<Option<&'a State>>,
}

impl<'a> Context<'a> {
    fn new() -> Self {
	Self {
	    state: RefCell::new(None),
	}
    }

    fn set_state(&self, new_state: &'a State) {
	*self.state.borrow_mut() = Some(new_state);
	// let mut st = self.state.borrow_mut();
	// *st = Some(new_state);
    }

    fn get_state(&self) -> &'a State {
	self.state.borrow().unwrap()
	// let st = self.state.borrow();
	// st.unwrap()
    }
}
fn main() {
    let context = Context::new();

    let start_state = State::StartState;
    start_state.do_action(&context);

    println!("{}", context.get_state().to_string());

    let stop_state = State::StopState;
    stop_state.do_action(&context);

    println!("{}", context.get_state().to_string());
}
