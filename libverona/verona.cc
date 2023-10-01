#include <verona.h>
#include <cpp/when.h>
#include <iostream>
#include <cstdio>

using namespace verona::rt;
using namespace verona::cpp;

extern "C"
{
  void poll_future_in_rust(void *future);

  void runtime_init()
  {
    auto& sched = Scheduler::get();
    Scheduler::set_detect_leaks(true);
    sched.set_fair(true);
    sched.init(2);
  }

  void scheduler_run()
  {
    auto& sched = Scheduler::get();
    sched.run();
  }

  void schedule_task(void *task)
  {
    when() << [=]() {
      poll_future_in_rust(task);
    };
  }
}
