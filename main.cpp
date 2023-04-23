#include <iostream>

extern "C" {
#include <font_kit_wrapper.h>
}

int main() {
    auto font_buffer = find_system_font("ArialMT");

    if (font_buffer.data != nullptr && font_buffer.len > 0) {
        std::cout << "Font data is valid." << std::endl;
    }

    free_font_buffer(font_buffer);

    return 0;
}
