#include <iostream>

extern "C" {
#include <font_kit_wrapper.h>
}

int main()
{
    auto font_buffer = find_system_font("OpenSans");

    if (font_buffer.data != nullptr && font_buffer.len > 0)
    {
        std::cout << "Font data is valid." << std::endl;
    }
    else
    {
        std::cout << "Font data is invalid!" << std::endl;
    }

    free_font_buffer(font_buffer);

    return 0;
}
