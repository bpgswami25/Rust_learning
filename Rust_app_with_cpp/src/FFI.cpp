// ...new file...
/*#include <cstdio>

extern "C" void say_hello() {
    std::puts("Hello from C++!");
}*/

#include <iostream>

namespace ffi 
{
    extern "C" void say_hello()
    {
        std::cout<<"Hello from C++ file"<<std::endl;
    }

    extern "C" int add_num(int a, int b)
    {
        return a+b;

    }

}
