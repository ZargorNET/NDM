use std::sync::Arc;

use chrono::{DateTime, TimeZone, Utc};
use serenity::CacheAndHttp;
use serenity::prelude::RwLock;

use crate::command_framework::CommandManager;
use crate::util::eventwaiter::Eventwaiter;
use crate::util::safe::Safe;

pub type ScheduleFunction = fn(ScheduleArguments);
pub type ArcScheduler = Arc<Scheduler>;

pub struct ScheduleArguments {
    pub command_manager: Arc<RwLock<CommandManager>>,
    pub safe: Arc<RwLock<Safe>>,
    pub scheduler: Arc<Scheduler>,
    pub serenity: Arc<CacheAndHttp>,
    pub event_waiter: Arc<Eventwaiter>,
}

pub struct Scheduler {
    schedules: Arc<RwLock<Vec<Schedule>>>,
}

#[derive(Clone)]
struct Schedule {
    interval: u64,
    function: ScheduleFunction,
    onetime: bool,
    last_executed: DateTime<Utc>,
}

impl Scheduler {
    pub fn new(cmd_handler: Arc<RwLock<CommandManager>>, safe: Arc<RwLock<Safe>>, serenity: Arc<CacheAndHttp>, event_waiter: Arc<Eventwaiter>) -> ArcScheduler {
        let s = Arc::new(Scheduler {
            schedules: Arc::new(RwLock::new(Vec::new())),
        });

        s.start_schedule(cmd_handler, safe, Arc::clone(&s), Arc::clone(&serenity), Arc::clone(&event_waiter));
        s
    }

    #[allow(dead_code)]
    pub fn schedule_repeated(&self, interval_sec: u64, func: ScheduleFunction) {
        let schedules = Arc::clone(&self.schedules);
        let mut schedules = schedules.write();
        schedules.push(Schedule {
            interval: interval_sec,
            function: func,
            onetime: false,
            last_executed: Utc.timestamp(0, 0),
        });
    }

    #[allow(dead_code)]
    pub fn schedule_onetime(&self, after_sec: u64, func: ScheduleFunction) {
        let schedules = Arc::clone(&self.schedules);
        let mut schedules = schedules.write();
        schedules.push(Schedule {
            interval: after_sec,
            function: func,
            onetime: true,
            last_executed: Utc.timestamp(0, 0),
        });
    }

    pub fn clear_all(&self) {
        let schedules = Arc::clone(&self.schedules);
        let mut schedules = schedules.write();
        schedules.clear();
        schedules.shrink_to_fit();
    }

    fn start_schedule(&self, command_manager: Arc<RwLock<CommandManager>>, safe: Arc<RwLock<Safe>>, scheduler: Arc<Scheduler>, serenity: Arc<CacheAndHttp>, event_waiter: Arc<Eventwaiter>) {
        let schedules = Arc::clone(&self.schedules);
        let cmd_manager = Arc::clone(&command_manager);

        std::thread::spawn(move || {
            let schedules = schedules;
            let cmd_manager = cmd_manager;
            let safe = safe;
            let scheduler = scheduler;
            let serenity = serenity;

            loop {
                let mut schedules = schedules.write();

                let now: DateTime<Utc> = chrono::Utc::now();


                for schedule in schedules.iter_mut() {
                    if now.timestamp() >= (schedule.last_executed + time::Duration::seconds(schedule.interval as i64)).timestamp() {
                        schedule.last_executed = now;

                        let tm_cmd_manager = Arc::clone(&cmd_manager);
                        let tm_safe = Arc::clone(&safe);
                        let tm_scheduler = Arc::clone(&scheduler);
                        let tm_serenity = Arc::clone(&serenity);
                        let tm_event_waiter = Arc::clone(&event_waiter);

                        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            (schedule.function)(ScheduleArguments {
                                command_manager: tm_cmd_manager,
                                safe: tm_safe,
                                scheduler: tm_scheduler,
                                serenity: tm_serenity,
                                event_waiter: tm_event_waiter,
                            });
                        })
                        );
                        match res {
                            Ok(_) => {}
                            Err(_e) => error!("SCHEDULER: caught unwind from scheduler thread")
                        }
                    }
                }

                schedules.retain(|s| !s.onetime || s.last_executed.timestamp() == 0);

                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }
}