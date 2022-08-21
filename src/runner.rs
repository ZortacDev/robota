use crate::context::Context;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::Result;
use crate::job::Job;

pub(crate) trait Runner {
    fn spin(&mut self, context: &Context) -> Result<()>;
}

pub(crate) struct LoopRunner<J: Job> {
    job: J,
    rate: Duration,
    last_exec_time: SystemTime,
}

impl<J: Job> LoopRunner<J> {
    pub(crate) fn new(duration: Duration) -> LoopRunner<J> {
        LoopRunner {
            job: J::new(),
            rate: duration,
            last_exec_time: UNIX_EPOCH,
        }
    }
}

impl<J: Job> Runner for LoopRunner<J> {
    fn spin(&mut self, context: &Context) -> Result<()> {
        let now = SystemTime::now();
        if now.duration_since(self.last_exec_time).unwrap() > self.rate {
            self.last_exec_time = now;
            self.job.run(context)
        } else {
            Ok(())
        }
    }
}