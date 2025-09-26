#include <iostream>

extern "C"{
    void callFromRustToCPP();
    void callFromCPPToRust();
}

extern "C" void callFromRustToCPP(){
    std::cout<<"called CPP function from rust file"<<std::endl;

    callFromCPPToRust();
}
