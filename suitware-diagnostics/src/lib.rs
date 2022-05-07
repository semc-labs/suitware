use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Ping;

pub mod basic {
    //! A basic initial diagnostic. Checks that the messaging system is up and
    //! running.

    use super::Ping;
    use actix::prelude::*;
    use suitware_util::ok;

    pub struct Basic;

    impl Actor for Basic {
        type Context = Context<Self>;
    }

    impl Handler<Ping> for Basic {
        type Result = ();

        fn handle(&mut self, _msg: Ping, _ctx: &mut Self::Context) {
            ok!("basic")
        }
    }
}
