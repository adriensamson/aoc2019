use crate::intcode::{IntCode, IntCodeIo};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn step1(input: &str) {
    let controller = Part1NetworkController::new(50);
    let mut computers: Vec<IntCode<ComputerInterface>> = controller
        .borrow()
        .interfaces
        .iter()
        .map(|i| {
            IntCode::from_str(
                input,
                ComputerInterface::new(controller.clone(), i.borrow().address),
            )
        })
        .collect();

    while !controller.borrow().exit {
        for comp in &mut computers {
            comp.run();
        }
    }
}

pub fn step2(input: &str) {
    let controller = Part2NetworkController::new(50);
    let mut computers: Vec<IntCode<ComputerInterface>> = controller
        .borrow()
        .interfaces
        .iter()
        .map(|(_, i)| {
            IntCode::from_str(
                input,
                ComputerInterface::new(controller.clone(), i.borrow().address),
            )
        })
        .collect();

    while !controller.borrow().exit {
        for comp in &mut computers {
            comp.run();
        }
    }
}

struct NetworkInterface {
    address: usize,
    queue: VecDeque<(i64, i64)>,
}

impl NetworkInterface {
    fn new(address: usize) -> NetworkInterface {
        NetworkInterface {
            address,
            queue: VecDeque::new(),
        }
    }
}

trait NetworkController {
    fn get_packet(&mut self, address: usize) -> Option<(i64, i64)>;
    fn send_packet(&mut self, from: usize, to: usize, packet: (i64, i64));
}

struct Part1NetworkController {
    exit: bool,
    interfaces: Vec<Rc<RefCell<NetworkInterface>>>,
}

impl Part1NetworkController {
    fn new(nb: usize) -> Rc<RefCell<Part1NetworkController>> {
        let c = Rc::new(RefCell::new(Part1NetworkController {
            interfaces: Vec::new(),
            exit: false,
        }));
        for i in 0..nb {
            c.borrow_mut()
                .interfaces
                .push(Rc::new(RefCell::new(NetworkInterface::new(i))));
        }
        c
    }
}

impl NetworkController for Part1NetworkController {
    fn get_packet(&mut self, address: usize) -> Option<(i64, i64)> {
        self.interfaces[address].borrow_mut().queue.pop_front()
    }

    fn send_packet(&mut self, _from: usize, to: usize, packet: (i64, i64)) {
        if to == 255 {
            println!("{}", packet.1);
            self.exit = true;
            return;
        }
        self.interfaces[to].borrow_mut().queue.push_back(packet);
    }
}

struct ComputerInterface {
    nc: Rc<RefCell<dyn NetworkController>>,
    address: usize,
    address_sent: bool,
    packet_tail: Option<i64>,
    current_output: (Option<i64>, Option<i64>),
    interrupted: bool,
}

impl ComputerInterface {
    fn new(nc: Rc<RefCell<dyn NetworkController>>, address: usize) -> ComputerInterface {
        ComputerInterface {
            nc,
            address,
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
            Some(self.address as i64)
        } else if let Some(t) = self.packet_tail {
            self.packet_tail = None;
            Some(t)
        } else if self.interrupted {
            self.interrupted = false;
            match self.nc.borrow_mut().get_packet(self.address) {
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

    fn output(&mut self, val: i64) {
        match self.current_output {
            (None, None) => {
                self.current_output = (Some(val), None);
            }
            (Some(_), None) => {
                self.current_output.1 = Some(val);
            }
            (Some(a), Some(h)) => {
                self.current_output = (None, None);
                self.nc
                    .borrow_mut()
                    .send_packet(self.address, a as usize, (h, val));
            }
            _ => panic!(),
        }
    }
}

struct Part2NetworkController {
    exit: bool,
    interfaces: Vec<(bool, Rc<RefCell<NetworkInterface>>)>,
    nat: Option<(i64, i64)>,
    last_nat_sent: Option<(i64, i64)>,
}

impl Part2NetworkController {
    fn new(nb: usize) -> Rc<RefCell<Part2NetworkController>> {
        let c = Rc::new(RefCell::new(Part2NetworkController {
            interfaces: Vec::new(),
            exit: false,
            nat: None,
            last_nat_sent: None,
        }));
        for i in 0..nb {
            c.borrow_mut()
                .interfaces
                .push((false, Rc::new(RefCell::new(NetworkInterface::new(i)))));
        }
        c
    }

    fn check_send_nat(&mut self) {
        if let Some(nat) = self.nat {
            if self
                .interfaces
                .iter()
                .all(|(w, ni)| *w && ni.borrow().queue.is_empty())
            {
                if self.last_nat_sent != None && nat.1 == self.last_nat_sent.unwrap().1 {
                    println!("{}", self.nat.unwrap().1);
                    self.exit = true;
                    return;
                }
                self.last_nat_sent = self.nat;
                self.interfaces[0].0 = false;
                self.interfaces[0]
                    .1
                    .borrow_mut()
                    .queue
                    .push_back(self.nat.unwrap());
            }
        }
    }
}

impl NetworkController for Part2NetworkController {
    fn get_packet(&mut self, address: usize) -> Option<(i64, i64)> {
        self.check_send_nat();
        let p = self.interfaces[address].1.borrow_mut().queue.pop_front();
        self.interfaces[address].0 = p == None;
        p
    }

    fn send_packet(&mut self, from: usize, to: usize, packet: (i64, i64)) {
        self.interfaces[from].0 = false;
        if to == 255 {
            self.nat = Some(packet);
            return;
        }
        self.interfaces[to].1.borrow_mut().queue.push_back(packet);
    }
}
