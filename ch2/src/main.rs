use std::cell::RefCell;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

struct Context<'a> {
    waker: &'a Waker,
}

impl<'a> Context<'a> {
    fn from_waker(waker: &'a Waker) -> Self {
        Context { waker }
    }

    fn waker(&self) -> &'a Waker {
        &self.waker
    }
}

struct Waker;

impl Waker {
    fn wake(&self) {
        NOTIFY.with(|n| *n.borrow_mut() = true)
    }
}

fn run<F>(mut f: F) -> F::Output
where
    F: Future,
{
    NOTIFY.with(|n| loop {
        if *n.borrow() {
            *n.borrow_mut() = false;
            let ctx = Context::from_waker(&Waker);
            if let Poll::Ready(v) = f.poll(&ctx) {
                return v;
            }
        }
    })
}

enum Poll<T> {
    Ready(T),
    Pending,
}

trait Future {
    type Output;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;
}

#[derive(Default)]
struct MyFuture {
    count: i32,
}

impl Future for MyFuture {
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.count {
            3 => Poll::Ready(self.count),
            _ => {
                self.count += 1;
                ctx.waker.wake();
                Poll::Pending
            }
        }
    }
}

fn main() {
    let f = MyFuture::default();
    println!("Output: {}", run(f));
}
