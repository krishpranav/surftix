//!
//! @file: user_agent.rs
//! COPYRIGHT 2023 KRISNA PRANAV
//!
//!

use fake_useragent::{Browsers, UserAgentsBuilder};

pub fn random_user_agent() -> String {
    UserAgentsBuilder::new()
        .cache(false)
        .dir("/tmp")
        .thread(1)
        .set_browsers(
            Browsers::new()
                .set_chrome()
                .set_safari()
                .set_edge()
                .set_firefox()
                .set_mozilla(),
        )
        .build()
        .random()
        .to_string()
}
