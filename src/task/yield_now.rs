use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

pub async fn yield_now() {
    struct YieldNowFuture {
        yielded: bool,
    }

    impl Future for YieldNowFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
            if self.yielded {
                Poll::Ready(())
            } else {
                self.yielded = true;
                Poll::Pending
            }
        }
    }

    YieldNowFuture { yielded: false }.await;
}
