use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[repr(u8)]
pub enum Collection {
    Principal = 0,
    PushSubscription = 1,
    Email = 2,
    Mailbox = 3,
    Thread = 4,
    Identity = 5,
    EmailSubmission = 6,
    SieveScript = 7,
}

impl From<u8> for Collection {
    fn from(v: u8) -> Self {
        match v {
            0 => Collection::Principal,
            1 => Collection::PushSubscription,
            2 => Collection::Email,
            3 => Collection::Mailbox,
            4 => Collection::Thread,
            5 => Collection::Identity,
            6 => Collection::EmailSubmission,
            7 => Collection::SieveScript,
            _ => panic!("Invalid collection"),
        }
    }
}

impl From<Collection> for u8 {
    fn from(v: Collection) -> Self {
        v as u8
    }
}

impl Display for Collection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Collection::Principal => write!(f, "principal"),
            Collection::PushSubscription => write!(f, "pushSubscription"),
            Collection::Email => write!(f, "email"),
            Collection::Mailbox => write!(f, "mailbox"),
            Collection::Thread => write!(f, "thread"),
            Collection::Identity => write!(f, "identity"),
            Collection::EmailSubmission => write!(f, "emailSubmission"),
            Collection::SieveScript => write!(f, "sieveScript"),
        }
    }
}