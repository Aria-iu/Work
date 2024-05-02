#include "inc.h"

int main(void) {
    test_glibc_version();
    perror_test("./test.txt",O_RDWR,O_CREAT);
    return 0;
}
