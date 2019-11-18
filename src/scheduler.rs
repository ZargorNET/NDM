use std::sync::Arc;

use chrono::{DateTime, TimeZone, Utc};
use serenity::prelude::RwLock;

use crate::command_framework::CommandManager;
use crate::safe::Safe;

pub type ScheduleFunction = fn(ScheduleArguments);

pub struct ScheduleArguments {
    pub command_manager: Arc<RwLock<CommandManager>>,
    pub safe: Arc<RwLock<Safe>>,
    pub scheduler: Arc<RwLock<Scheduler>>,
}

pub struct Scheduler {
    schedules: Arc<RwLock<Vec<Schedule>>>,
    start_delay_millis: u64,
}

#[derive(Clone)]
struct Schedule {
    interval: u64,
    function: ScheduleFunction,
    onetime: bool,
    last_executed: DateTime<Utc>,
}

impl Scheduler {
    pub fn new(cmd_handler: Arc<RwLock<CommandManager>>, safe: Arc<RwLock<Safe>>, start_delay_millis: u64) -> Arc<RwLock<Scheduler>> {
        let s: Arc<RwLock<Scheduler>> = Arc::new(RwLock::new(Scheduler {
            schedules: Arc::new(RwLock::new(Vec::new())),
            start_delay_millis
        }));

        s.read().start_schedule(cmd_handler, safe, Arc::clone(&s));
        s
    }

    #[allow(dead_code)]
    pub fn schedule_repeated(&mut self, interval_sec: u64, func: ScheduleFunction) {
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
    pub fn schedule_onetime(&mut self, after_sec: u64, func: ScheduleFunction) {
        let schedules = Arc::clone(&self.schedules);
        let mut schedules = schedules.write();
        schedules.push(Schedule {
            interval: after_sec,
            function: func,
            onetime: true,
            last_executed: Utc.timestamp(0, 0),
        });
    }

    pub fn clear_all(&mut self) {
        let schedules = Arc::clone(&self.schedules);
        let mut schedules = schedules.write();
        schedules.clear();
        schedules.shrink_to_fit();
    }

    fn start_schedule(&self, command_manager: Arc<RwLock<CommandManager>>, safe: Arc<RwLock<Safe>>, scheduler: Arc<RwLock<Scheduler>>) {
        let schedules = Arc::clone(&self.schedules);
        let cmd_manager = Arc::clone(&command_manager);
        let start_delay_millis = self.start_delay_millis;

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(start_delay_millis));

            let schedules = schedules;
            let cmd_manager = cmd_manager;
            let safe = safe;
            let scheduler = scheduler;

            loop {
                let mut schedules = schedules.write();

                let now: DateTime<Utc> = chrono::Utc::now();


                for schedule in schedules.iter_mut() {
                    if now.timestamp() >= (schedule.last_executed + time::Duration::seconds(schedule.interval as i64)).timestamp() {
                        schedule.last_executed = now;

                        let tm_cmd_manager = Arc::clone(&cmd_manager);
                        let tm_safe = Arc::clone(&safe);
                        let tm_scheduler = Arc::clone(&scheduler);

                        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            (schedule.function)(ScheduleArguments {
                                command_manager: tm_cmd_manager,
                                safe: tm_safe,
                                scheduler: tm_scheduler,
                            });
                        })
                        );
                        match res {
                            Ok(_) => {},
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