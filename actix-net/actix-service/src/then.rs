use super::{Service, ServiceFactory};
use alloc::rc::Rc;
use core::future::Future;
use core::marker::PhantomData;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_core::ready;
use pin_project_lite::pin_project;

// Rc用来多个ThenService可以共享同一对服务。
pub(crate) struct ThenService<A, B, Req>(Rc<(A, B)>, PhantomData<Req>);

impl<A, B, Req> ThenService<A, B, Req> {
    /// Create new `.then()` combinator
    pub(crate) fn new(a: A, b: B) -> ThenService<A, B, Req>
    where
        A: Service<Req>,
        B: Service<Result<A::Response, A::Error>, Error = A::Error>,
    {
        Self(Rc::new((a, b)), PhantomData)
    }
}

// 浅拷贝
impl<A, B, Req> Clone for ThenService<A, B, Req> {
    fn clone(&self) -> Self {
        ThenService(self.0.clone(), PhantomData)
    }
}

impl<A, B, Req> Service<Req> for ThenService<A, B, Req>
where
    A: Service<Req>,
    B: Service<Result<A::Response, A::Error>, Error = A::Error>,
{
    type Response = B::Response;
    type Error = B::Error;
    type Future = ThenServiceResponse<A, B, Req>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let (a, b) = &*self.0;
        let not_ready = a.poll_ready(ctx)?.is_ready();
        if !b.poll_ready(ctx)?.is_ready() || not_ready {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }

    fn call(&self, req: Req) -> Self::Future {
        ThenServiceResponse {
            state: State::A {
                fut: self.0 .0.call(req),
                b: Some(self.0.clone()),
            },
        }
    }
}

pin_project! {

pub(crate) struct ThenServiceResponse<A,B,Req>
where
    A: Service<Req>,
    B: Service<Result<A::Response,A::Error>>,
{
    #[pin]
    state: State<A,B,Req>,
}
}

pin_project! {
#[project=StateProj]
enum State<A, B, Req>
where
    A: Service<Req>,
    B: Service<Result<A::Response, A::Error>>,
{
    A { #[pin] fut: A::Future, b: Option<Rc(A, B)> },
    B { #[pin] fut: B::Future },
}
}

impl<A, B, Req> Future for ThenServiceResponse<A, B, Req>
where
    A: Service<Req>,
    B: Service<Result<A::Response, A::Error>>,
{
    type Output = Result<B::Response, B::Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.as_mut().project();

        match this.state.as_mut().project() {
            StateProj::A { fut, b } => {
                let res = ready!(fut.poll(cx));
                let b = b.take().unwrap();
                let fut = b.1.call(res);
                this.state.set(State::B { fut });
                self.poll(cx)
            }
            StateProj::B { fut } => fut.poll(cx),
        }
    }
}


#[cfg(test)]
mod tests{
    use alloc::rc::Rc;
    use core::cell::Cell;
    use core::future::Ready;
    use core::task::{Context, Poll};
    // use actix_rt::net::Ready;
    use crate::Service;

    #[derive(Clone)]
    struct Srv1(Rc<Cell<usize>>);

    impl Service<Result<&'static str, &'static str>> for Srv1 {
        type Response = &'static str;
        type Error = ();
        type Future = Ready<Result<Self::Response, Self::Error>>;

        fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.0.set(self.0.get() + 1);
            Poll::Ready(Ok(()))
        }

        fn call(&self, req: Result<&'static str, &'static str>) -> Self::Future {
            match req {
                Ok(msg) => Ready(Some(Ok(msg))),
                Err(_) => Ready(Some(Err(()))),
            }
        }
    }

    #[test]
    fn it_works(){
        let inner = Rc::new(Cell::new(0 as usize));
        let s = Srv1(inner.clone());




    }

}
