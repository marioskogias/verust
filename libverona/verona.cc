#include <verona.h>
#include <cpp/when.h>
#include <iostream>

using namespace verona::rt;
using namespace verona::cpp;

extern "C"
{
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
}
