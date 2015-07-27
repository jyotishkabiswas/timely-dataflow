use communication::Data;
use communication::pact::Pipeline;
use communication::observer::Extensions;

use construction::{Stream, GraphBuilder};
use construction::operators::unary::UnaryStreamExt;

pub trait FilterExt<D: Data> {
    fn filter<L: Fn(&D)->bool+'static>(&self, logic: L) -> Self;
}

impl<G: GraphBuilder, D: Data> FilterExt<D> for Stream<G, D> {
    fn filter<L: Fn(&D)->bool+'static>(&self, logic: L) -> Stream<G, D> {
        self.unary_stream(Pipeline, "Filter", move |input, output| {
            while let Some((time, data)) = input.pull() {
                data.retain(|x| logic(x));
                if data.len() > 0 {
                    output.give_message_at(time, data);
                }
            }
        })
    }
}