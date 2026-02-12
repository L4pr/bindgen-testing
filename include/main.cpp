#include <iostream>
#include <vector>
#include <cstring>

// Include the header that cbindgen generated for you
#include "rust_core.h"

int main() {
    std::cout << "--- C++ Calling Rust ---" << std::endl;

    // 1. Create the Rust object (Opaque Pointer)
    // C++ doesn't know what's inside, it just holds the handle.
    Calculator* my_calc = calc_create("Renzo's Calc", 500);

    if (my_calc == nullptr) {
        std::cerr << "Failed to create calculator!" << std::endl;
        return 1;
    }

    // 2. Call methods
    std::cout << "Adding 50..." << std::endl;
    calc_add(my_calc, 50);

    // 3. Get a string from Rust
    // Rust allocated this string, so we must ask Rust to free it later.
    char* summary = calc_get_summary(my_calc);

    std::cout << "Rust says: " << summary << std::endl;

    // 4. Cleanup
    // Free the string
    calc_free_string(summary);

    // Free the object
    calc_destroy(my_calc);

    std::cout << "--- Done ---" << std::endl;
    return 0;
}