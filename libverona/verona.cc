#include <verona.h>
#include <cpp/when.h>
#include <iostream>
#include <cstdio>

using namespace verona::rt;
using namespace verona::cpp;

extern "C"
{
  void foo_rust(void *future);

  void marios_print()
  {
    std::cout << "Hello from the external lib\n";
  }

  void runtime_init()
  {
    auto& sched = Scheduler::get();
    Scheduler::set_detect_leaks(true);
    sched.set_fair(true);
    sched.init(2);

    when() << []() { std::cout << "call the verona runtime from Rust\n"; };
  }

  void scheduler_run()
  {
    auto& sched = Scheduler::get();
    sched.run();
  }

  void schedule_task(void *task)
  {
    std::cout << "will schedule a task\n";

    when() << [=]() {
      printf("This is the task to send back to Rust: %lx\n",
          (unsigned long)task);
      foo_rust(task);
    };
  }
}
