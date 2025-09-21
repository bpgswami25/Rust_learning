#include <iostream>

namespace ffi 
{
    extern "C" void say_hello()
    {
        std::cout<<"Hello from C++ Lib"<<std::endl;
    }

    extern "C" int add(int a, int b)
    {
        return a+b;

    }

}
