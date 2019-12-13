use super::{Program,ProgramState,InvalidInstruction};

pub struct Computer { }

pub enum IOEvent {
    Input,
    Output(isize)
}

impl Computer {
    pub fn run<IOF>(program: &mut Program, mut io_fn: IOF) -> Result<(), InvalidInstruction>
     where IOF: FnMut(IOEvent) -> Option<isize> {

        let mut next_input = None;

        loop {
            let state = program.step(next_input)?;
            next_input = None;

            match state {
                ProgramState::Halted           => { return Ok(()) },
                ProgramState::Blocked          => { next_input = io_fn(IOEvent::Input) },
                ProgramState::Running(None)    => {  },
                ProgramState::Running(Some(v)) => { io_fn(IOEvent::Output(v)); }
            };
        };
    }

}
