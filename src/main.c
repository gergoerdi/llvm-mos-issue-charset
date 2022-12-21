#include <stdint.h>

extern void select_and_load_file(uint8_t* mem);
extern void make_charset();

int main ()
{
    uint8_t mem[4 * 1024 - 0x200];
    select_and_load_file(mem);

    make_charset();
    return 0;
}
