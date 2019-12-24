use crate::intcode::{Program,Computer,InvalidInstruction,IOEvent};

use std::collections::HashMap;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::thread;

const INPUT : &'static str = include_str!("../inputs/day23.txt");

struct NetworkNode {
    id:   usize,
    rx:   Receiver<isize>,
    tx:   Sender<isize>
}

struct Network {
    nodes:   Vec<NetworkNode>,
    inputs:  HashMap<usize,Sender<isize>>,
    program: Program
}

impl NetworkNode {
    fn new(id: usize) -> NetworkNode {
        let (tx, rx) = channel();
        NetworkNode {
            id:   id,
            rx:   rx,
            tx:   tx
        }
    }
}

enum OutputState {
    Address,
    X,
    Y
}

impl Network {
    fn new(count: usize, p: &Program) -> Network {
        let nodes : Vec<NetworkNode> = (0..count)
            .map(|id| NetworkNode::new(id))
            .collect();

        let inputs = nodes
            .iter()
            .map(|node| (node.id, node.tx.clone()))
            .collect();

        Network {
            nodes:   nodes,
            inputs:  inputs,
            program: p.clone()
        }
    }

    fn run(mut self) {
        self.boot();

        let inputs  = self.inputs.clone();
        let program = self.program.clone();

        let handles : Vec<thread::JoinHandle<_>> = self.nodes
            .drain(..)
            .map(|node|{
                let map = inputs.clone();
                let program = program.clone();

                thread::spawn(move ||{
                    let n = node;
                    let inputs = map;
                    let mut output_state = OutputState::Address;
                    let mut addr = 0;
                    let mut x = 0;
                    let mut y = 0;
                    let mut p = program;
                    Computer::run(&mut p, |event, _|{
                        match event {
                            IOEvent::Output(value) => {
                                match output_state {
                                    OutputState::Address => {
                                        addr = value;
                                        output_state = OutputState::X;
                                    },
                                    OutputState::X => {
                                        x = value;
                                        output_state = OutputState::Y;
                                    },
                                    OutputState::Y => {
                                        y = value;
                                        output_state = OutputState::Address;
                                        match inputs.get(&(addr as usize)) {
                                            Some(tx) => {
                                                tx.send(x).unwrap();
                                                tx.send(y).unwrap();
                                            },
                                            None => {
                                                println!("attempted to send {},{} to non-existant address {}", x, y, addr);
                                            }
                                        };
                                    }
                                };
                                None
                            },
                            IOEvent::Input => {
                                let next = n.rx.try_recv().unwrap_or(-1);
                                Some(next)
                            }
                        }
                    });
                })
            })
            .collect();

        for h in handles {
            h.join().unwrap();
        };
    }

    fn boot(&mut self) {
        // send each node it's own ID as input to boot up
        self
            .inputs
            .iter()
            .for_each(|(id, input)|{
                input.send(*id as isize).unwrap();
            });
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_solution() {
        let p = Program::parse(INPUT).expect("Failed to parse program");
        let net = Network::new(50, &p);

        net.run();
    }
}
