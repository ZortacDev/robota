use crate::context::Context;
use crate::Result;

pub(crate) trait Job {
    type PublishedData;

    fn new() -> Self;
    fn run(&mut self, context: &Context) -> Result<()>;
}