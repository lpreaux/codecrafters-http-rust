use std::ops::Deref;
use crate::http::header::accept::AcceptHeader;
use crate::http::header::header_base::HttpHeader;
use crate::http::header::host::HostHeader;
use crate::http::header::user_agent::UserAgentHeader;

pub enum HttpHeaderKind {
    Host(Box<HostHeader>),
    UserAgent(Box<UserAgentHeader>),
    Accept(Box<AcceptHeader>),
}

// Implementation of deref to automatically forward method calls
impl Deref for HttpHeaderKind {
    type Target = dyn HttpHeader;

    fn deref(&self) -> &Self::Target {
        match self {
            HttpHeaderKind::Host(host) => host.as_ref(),
            HttpHeaderKind::UserAgent(user_agent) => user_agent.as_ref(),
            HttpHeaderKind::Accept(accept) => accept.as_ref(),
        }
    }
}

// Implement HttpHeader for HttpHeaderKind if you need specific methods
impl HttpHeader for HttpHeaderKind {
    // Implement methods that require customization
}
