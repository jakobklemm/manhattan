use crossbeam_channel::{Sender, Receiver, unbounded};

pub struct Origin<T: Send> {
    management_sender: Sender<Sender<T>>,
    management_receiver: Sender<Receiver<T>>
}

impl<T> Origin<T> where T: Send {
   pub fn spawn(&self) {
       let (rem_s, local_r): (Sender<T>, Receiver<T>) = unbounded();

       let (local_s, rem_r) = (rem_s.clone(), local_r.clone());

       self.management_sender.send(rem_s);
       self.management_receiver.send(rem_r);

       todo!()
   } 
}

pub struct Local<T: Send> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}
