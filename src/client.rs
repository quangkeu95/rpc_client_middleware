use std::sync::Arc;

use solana_client::rpc_client::RpcClient;

use crate::Middleware;

/// A `RpcClientBuilder` is used to build a [`RpcClientWithMiddleware`]
///
/// [`RpcClientWithMiddleware`]: create::RpcClientWithMiddleware
pub struct RpcClientBuilder {
    client: RpcClient,
    middleware_stack: Vec<Arc<dyn Middleware>>,
}

impl RpcClientBuilder {
    pub fn new(client: RpcClient) -> Self {
        Self {
            client,
            middleware_stack: Vec::new(),
        }
    }

    /// Convenience method to attach middleware.
    ///
    /// If you need to keep a reference to the middleware after attaching, use [`with_arc`].
    ///
    /// [`with_arc`]: Self::with_arc
    pub fn with<M>(self, middleware: M) -> Self
    where
        M: Middleware,
    {
        self.with_arc(Arc::new(middleware))
    }

    /// Add middleware to the chain. [`with`] is more ergonomic if you don't need the `Arc`.
    ///
    /// [`with`]: Self::with
    pub fn with_arc(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middleware_stack.push(middleware);
        self
    }

    pub fn build(self) -> RpcClientWithMiddleware {
        RpcClientWithMiddleware {}
    }
}

/// `RpcClientWithMiddleware` is a wrapper around [`RpcClient`] which runs middleware on every request.
///
/// [`RpcClient`]: solana_client::rpc_client::RpcClient
#[derive(Clone)]
pub struct RpcClientWithMiddleware {
    inner: RpcClient,
    middleware_stack: Box<[Arc<dyn Middleware>]>,
}
