use std::future::Future;
use std::time::Duration;
use std::pin::Pin;

use async_std::task::{self, Poll, Context};
use pin_project_lite::pin_project;


struct DoNothing;

impl Future for DoNothing {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }

}

struct SleepPrint <F> {
    sleep: F,
}

impl<F: Future<Output=()>> Future for SleepPrint<F> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let sleep: Pin<&mut F> = unsafe { self.map_unchecked_mut(|s| &mut s.sleep) };
        match sleep.poll(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                println!("Inside SleepPrint");
                Poll::Ready(())
            },
        }
    }
}

pin_project! {
    struct TwoFutures<Fut1, Fut2> {
        first_done: bool,
        #[pin]
        first: Fut1,
        #[pin]
        second: Fut2,
    }
}

impl <Fut1: Future<Output=()>, Fut2: Future<Output=()>> Future for TwoFutures<Fut1, Fut2> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        if !*this.first_done {
            if let Poll::Ready(_) = this.first.poll(ctx) {
                *this.first_done = true;
            }
        }
        if *this.first_done {
            this.second.poll(ctx)
        } else {
            Poll::Pending
        }
    }
}

async fn sleepus() {
    println!("Sleepus 1");
    task::sleep(Duration::from_millis(500)).await;
    println!("Sleepus 2");
    task::sleep(Duration::from_millis(500)).await;
    println!("Sleepus 3");
}


async fn interruptus() {
    for i in 0..5 {
        println!("Interruptus {}", i);
        task::sleep(Duration::from_millis(1000)).await;
    }
}

fn main() {
    task::block_on(async {
        task::spawn(sleepus());
        interruptus().await;

    });
}
