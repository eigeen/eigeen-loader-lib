set_project("cpp_example")

add_requires("fmt")

target("cpp_example")
    set_kind("shared")
    set_languages("c++17")

    add_packages("fmt")

    -- include
    add_includedirs("../../include")
    -- static library
    add_links("../../include/eigeen_loader.dll.lib")

    add_files("plugin.cpp")