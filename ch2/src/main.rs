use std::cell::RefCell;

thread_local!(static NOTIFY: RefCell<bool> = RefCell::new(true));

mod task {
    use crate::NOTIFY;

    pub struct Context<'a> {
        waker: &'a Waker,
    }

    impl<'a> Context<'a> {
        pub fn from_waker(waker: &'a Waker) -> Self {
            Context { waker }
        }

        pub fn waker(&self) -> &'a Waker {
            &self.waker
        }
    }

    pub struct Waker;

    impl Waker {
        pub fn wake(&self) {
            NOTIFY.with(|n| *n.borrow_mut() = true)
        }
    }
}

use crate::task::*;

mod future {
    use crate::task::*;

    pub enum Poll<T> {
        Ready(T),
        Pending,
    }

    pub trait Future {
        type Output;

        fn poll(&mut self, ctx: &Context) -> Poll<Self::Output>;
    }
}

use crate::future::*;

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
                ctx.waker().wake();
                Poll::Pending
            }
        }
    }
}

struct AddOneFuture<T>(T);

impl<T> Future for AddOneFuture<T>
where
    T: Future,
    T::Output: std::ops::Add<i32, Output = i32>,
{
    type Output = i32;

    fn poll(&mut self, ctx: &Context) -> Poll<Self::Output> {
        match self.0.poll(ctx) {
            Poll::Ready(v) => Poll::Ready(v + 1),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn main() {
    let f = MyFuture::default();
    println!("Output: {}", run(AddOneFuture(f)));
}
