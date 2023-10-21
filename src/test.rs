use std::thread;
use tokio::sync::oneshot::{channel, Receiver, Sender};

trait Message {
    type ReturnType;
}

trait Handle<M: Message> {
    fn handle(&mut self, msg: M) -> M::ReturnType;
}

trait EnvelopeProxy<A: Actor> {
    fn handle(&mut self, act: &mut A);
}

struct Envelope<A: Actor>(Box<dyn EnvelopeProxy<A>>);

struct SyncEnvelopeProxy<M>
where
    M: Message + Send,
    M::ReturnType: Send,
{
    msg: Option<M>,
    tx: Sender<M::ReturnType>,
}

impl<A, M> EnvelopeProxy<A> for SyncEnvelopeProxy<M>
where
    M: Message + Send + 'static,
    M::Result: Send,
    A: Actor + Handle<M>,
{
    fn handle(&mut self, act: &mut A) {
        if let Some(msg) = self.msg.take() {
            let fut = <A as Handle<M>>::handle(act, msg);
            let _ = self.tx.send(fut);
        }
    }
}

struct Addr<A: Actor> {
    tx: Sender<Envelope<A>>,
}

impl<A> Addr<A> {
    fn new(tx: Sender<Envelope<A>>) -> Self {
        Self {
            tx
        }
    }
}

impl<A: Actor> Addr<A> {
    fn send<M: Message>(&self, msg: M) {}
}

trait Actor {
    fn start(&self, tx: Sender<Envelope<Self>>) -> Addr<Self> {
        Addr::new(tx)
    }
}

struct TestActor {}

impl Actor for TestActor {}

struct TestMessage(String);

impl Message for TestMessage {
    type ReturnType = ();
}

impl Handle<TestMessage> for TestActor {
    fn handle(&mut self, msg: TestMessage) -> Self::ReturnType {
        println!("{}", msg.0);
        return ();
    }
}

fn main() {
    let thrd = thread::spawn(move || {

    });
    let (tx, rx) = channel();
    let a = TestActor {}.start(tx);
    let m = TestMessage(String::from("42"));

    let _ = a.send(m);

    thrd.join();
}
