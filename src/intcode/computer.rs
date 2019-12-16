use super::{Program,ProgramState,InvalidInstruction};

pub struct Computer {
    did_abort: bool
}

pub enum IOEvent {
    Input,
    Output(isize)
}

impl Computer {
    pub fn abort(&mut self) {
        self.did_abort = true;
    }

    pub fn run<IOF>(program: &mut Program, mut io_fn: IOF) -> Result<(), InvalidInstruction>
     where IOF: FnMut(IOEvent, &mut Computer) -> Option<isize> {
        let mut computer   = Computer { did_abort: false };
        let mut next_input = None;

        while !computer.did_abort {
            let state = program.step(next_input)?;
            next_input = None;

            match state {
                ProgramState::Halted           => { return Ok(()) },
                ProgramState::Blocked          => { next_input = io_fn(IOEvent::Input, &mut computer) },
                ProgramState::Running(None)    => {  },
                ProgramState::Running(Some(v)) => { io_fn(IOEvent::Output(v), &mut computer); }
            };
        };

        Ok(())
    }

}
