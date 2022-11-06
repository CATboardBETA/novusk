test: test.c
	@ echo "Compiling tests..."
	gcc -o test.o test.c -I ../inc -L ../lib -l stdint-intn -l stdint-uintn -l stddef -l stdint -l stdlib -l string
	@ echo "Running tests..."
	@./test.o ; case $$? in \
        0) echo "Tests passed." ;; \
        1) echo "Failed to pass FFI boundary" ;; \
        2) echo "Failed to do int division (stdlib.h::div)" ;; \
        3) echo "Failed to do long int division (stdlib.h::ldiv)" ;; \
        4) echo "Failed to do long long int division (stdlib.h::lldiv)" ;; \
        5) echo "Failed to do int abs (stdlib.h::abs)" ;; \
        6) echo "Failed to do long int abs (stdlib.h::labs)" ;; \
        7) echo "Failed to do long long int abs (stdlib.h::llabs)" ;; \
        8) echo "Failed to convert string to int (stdlib.h::atoi)" ;; \
        9) echo "Failed to convert string to long int (stdlib.h::atol)" ;; \
        10) echo "Failed to convert string to long long int (stdlib.h::atoll)" ;; \
    esac