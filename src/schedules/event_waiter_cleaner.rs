use crate::scheduler::ScheduleArguments;

pub fn clean_waiter(args: ScheduleArguments) {
    args.event_waiter.clean_timeouts(&args);
}