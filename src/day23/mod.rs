use std::collections::VecDeque;
use std::cell::RefCell;
use crate::intcode::{IntCodeIo, IntCode};
use std::rc::Rc;

pub fn step1(input : &str) {
    let controller = NetworkController::new(50);
    let mut computers : Vec<IntCode<ComputerInterface>> = controller.borrow().interfaces.iter()
        .map(|i| IntCode::from_str(input, ComputerInterface::new(i.clone())))
        .collect();

    while !controller.borrow().exit {
        for comp in &mut computers {
            comp.run();
        }
    }
}

pub fn step2(_input : &str) {}

struct NetworkInterface {
    controller : Rc<RefCell<NetworkController>>,
    address : usize,
    queue : VecDeque<(i64, i64)>,
}

impl NetworkInterface {
    fn new(controller : Rc<RefCell<NetworkController>>, address : usize) -> NetworkInterface {
        NetworkInterface {
            controller,
            address,
            queue: VecDeque::new(),
        }
    }
}

struct NetworkController {
    exit: bool,
    interfaces : Vec<Rc<RefCell<NetworkInterface>>>,
}

impl NetworkController {
    fn new(nb : usize) -> Rc<RefCell<NetworkController>> {
        let c = Rc::new(RefCell::new(NetworkController { interfaces: Vec::new(), exit: false }));
        for i in 0..nb {
            c.borrow_mut().interfaces.push(Rc::new(RefCell::new(NetworkInterface::new(c.clone(), i))));
        }
        c
    }

    fn send_packet(&mut self, address : usize, packet : (i64, i64)) {
        if address == 255 {
            println!("{}", packet.1);
            self.exit = true;
            return;
        }
        self.interfaces[address].borrow_mut().queue.push_back(packet);
    }
}

struct ComputerInterface {
    ni: Rc<RefCell<NetworkInterface>>,
    address_sent: bool,
    packet_tail: Option<i64>,
    current_output : (Option<i64>, Option<i64>),
    interrupted : bool,
}

impl ComputerInterface  {
    fn new(ni : Rc<RefCell<NetworkInterface>>) -> ComputerInterface {
        ComputerInterface {
            ni,
            address_sent: false,
            packet_tail: None,
            current_output: (None, None),
            interrupted: false,
        }
    }
}

impl IntCodeIo for ComputerInterface {
    fn input(&mut self) -> Option<i64> {
        if !self.address_sent {
            self.address_sent = true;
            Some(self.ni.borrow().address as i64)
        } else if let Some(t) = self.packet_tail {
            self.packet_tail = None;
            Some(t)
        } else {
            if self.interrupted {
                self.interrupted = false;
                match self.ni.borrow_mut().queue.pop_front() {
                    None => Some(-1),
                    Some((h, t)) => {
                        self.packet_tail = Some(t);
                        Some(h)
                    }
                }
            } else {
                self.interrupted = true;
                None
            }
        }
    }

    fn output(&mut self, val: i64) {
        match self.current_output {
            (None, None) => {
                self.current_output = (Some(val), None);
            },
            (Some(_), None) => {
                self.current_output.1 = Some(val);
            },
            (Some(a), Some(h)) => {
                self.current_output = (None, None);
                self.ni.borrow_mut().controller.borrow_mut().send_packet(a as usize, (h, val));
            }
            _ => panic!()
        }
    }
}