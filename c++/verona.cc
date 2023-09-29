#include <verona.h>
#include <iostream>

extern "C"
{
  void marios_print()
  {
    std::cout << "Hello from the external lib\n";
  }
}
