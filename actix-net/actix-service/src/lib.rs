//! 给出了 Service trait和 ServiceFactory trait的描述。
//! 以及从Box Rc Arc （RefCell）等相互转化（调用）。
//!

#![no_std]

mod then;

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, sync::Arc};
use core::{
    cell::RefCell,
    future::Future,
    task::{self, Context, Poll},
};

/// 定义了Service trait，用来表示统一的Service需要的模型。
/// 一个Service需要一个Req，返回Response或者Error。
///
/// poll_ready 方法的作用是检查服务是否准备好处理请求。
/// call 方法的作用是处理请求并异步返回响应。
pub trait Service<Req> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    /// Returns `Ready` when the service is able to process requests.
    /// 如果服务处于满负载状态，则返回 `Pending`，并在服务再次准备好时通知任务。这个函数预期在任务中被调用。
    ///
    ///  # 注意事项
    /// 1. `poll_ready` 可能会在不同于 `call` 的任务中被调用。
    /// 2. 在服务链的情况下，所有服务的 `.poll_ready()` 会被同时调用
    fn poll_ready(&self, ctx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>>;

    /// Process the request and return the response asynchronously.
    fn call(&self, req: Req) -> Self::Future;
}

/// 创建 `Service` 的工厂。
///
/// 这对于需要生成新 `Service` 的情况很有用。一种情况是 TCP 服务器监听器：监听器接受新连接，
/// 使用 `ServiceFactory` trait 为每个连接构建一个新的 `Service`，
/// 并使用这个新 `Service` 处理该新连接上的入站请求。
///
/// `Config` 是一个服务工厂配置类型。
///
/// 简单的工厂可以使用 [`fn_factory`] 或 [`fn_factory_with_config`] 来减少样板代码。
pub trait ServiceFactory<Req> {
    /// Responses given by the created services.
    type Response;

    /// Errors produced by the created services.
    type Error;

    /// Service factory configuration.
    type Config;

    /// The kind of `Service` created by this factory.
    type Service: Service<Req, Response = Self::Response, Error = Self::Error>;

    /// Errors potentially raised while building a service.
    type InitError;

    /// The future of the `Service` instance.g
    type Future: Future<Output = Result<Self::Service, Self::InitError>>;

    /// Create and return a new service asynchronously.
    fn new_service(&self, cfg: Self::Config) -> Self::Future;
}

impl<'a, S, Req> Service<Req> for &'a mut S
where
    S: Service<Req>,
    S: 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        (**self).poll_ready(ctx)
    }

    fn call(&self, req: Req) -> Self::Future {
        (**self).call(req)
    }
}

impl<'a, S, Req> Service<Req> for &'a S
where
    S: Service<Req>,
    S: 'a,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        (**self).poll_ready(ctx)
    }

    fn call(&self, req: Req) -> Self::Future {
        (**self).call(req)
    }
}

/*
    使用 ?Sized，
    可以支持如 Box<dyn Service<Req>> 和 Rc<dyn Service<Req>> 这样的类型，
    这些类型在运行时确定其大小。
*/

impl<Req, S> Service<Req> for Box<S>
where
    S: Service<Req>,
    S: ?Sized,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        (**self).poll_ready(ctx)
    }

    fn call(&self, req: Req) -> Self::Future {
        (**self).call(req)
    }
}

/*
    该实现被弃用的原因在于 Service trait 现在接收共享引用。
    这意味着 poll_ready 和 call 方法可以通过共享引用调用，而不需要借用可变引用。
*/

impl<Req, S> Service<Req> for RefCell<S>
where
    S: Service<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.borrow().poll_ready(ctx)
    }

    fn call(&self, req: Req) -> Self::Future {
        self.borrow().call(req)
    }
}

impl<S, Req> ServiceFactory<Req> for Rc<S>
where
    S: ServiceFactory<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Config = S::Config;
    type Service = S::Service;
    type InitError = S::InitError;
    type Future = S::Future;

    fn new_service(&self, cfg: Self::Config) -> Self::Future {
        self.as_ref().new_service(cfg)
    }
}

impl<S, Req> ServiceFactory<Req> for Arc<S>
where
    S: ServiceFactory<Req>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Config = S::Config;
    type Service = S::Service;
    type InitError = S::InitError;
    type Future = S::Future;

    fn new_service(&self, cfg: S::Config) -> S::Future {
        self.as_ref().new_service(cfg)
    }
}

/// 这个trait描述了可以拿走所有权，转换为一个Service<Req>的类要实现的特征。
pub trait IntoService<S, Req>
where
    S: Service<Req>,
{
    fn into_service(self) -> S;
}

/// 同上
pub trait IntoServiceFactory<SF, Req>
where
    SF: ServiceFactory<Req>,
{
    /// Convert `Self` to a `ServiceFactory`
    fn into_factory(self) -> SF;
}

impl<S, Req> IntoService<S, Req> for S
where
    S: Service<Req>,
{
    fn into_service(self) -> S {
        self
    }
}

impl<SF, Req> IntoServiceFactory<SF, Req> for SF
where
    SF: ServiceFactory<Req>,
{
    fn into_factory(self) -> SF {
        self
    }
}

pub fn into_service<I, S, Req>(tp: I) -> S
where
    I: IntoService<S, Req>,
    S: Service<Req>,
{
    tp.into_service()
}
