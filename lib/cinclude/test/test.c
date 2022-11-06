
// Disambiguation between system and our own definitions of the standard library require us to go back a few
// directories.
#include <stdlib.h>

int main() {
    if (!rust_ffi_test()) return 1;

    struct div_t divResult = {.quot = 2, .rem = 1};
    if (div(5, 2).quot != divResult.quot || div(5, 2).rem != divResult.rem) return 2;
    struct ldiv_t divResult2 = {.quot = 2, .rem = 1};
    if (ldiv(5, 2).quot != divResult2.quot || ldiv(5, 2).rem != divResult2.rem) return 3;
    struct lldiv_t divResult3 = {.quot = 2, .rem = 1};
    if (lldiv(5, 2).quot != divResult3.quot || lldiv(5, 2).rem != divResult3.rem) return 4;

    if (abs(-5) != 5) return 5;
    if (labs(-5) != 5) return 6;
    if (llabs(-5) != 5) return 7;

    if (atoi("5") != 5) return 8;
    if (atol("5") != 5) return 9;
    if (atoll("5") != 5) return 10;

    return 0;
}
