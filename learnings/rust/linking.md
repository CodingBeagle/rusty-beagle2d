# Dynamic Library (DLL) Linking

There are multiple ways to link to a DLL.

It can be done:
- Implicitly at link time.
  - Here, the functions off the DLL is linked implicitly to the EXE at link time.
- Explicitly at run time.
  - Here you use functions in the programming language to load the DLL into memory, and then use a function to get memory addresses for named functions you need to invoke.

## More on Implicit Linking

If you link to a DLL implicitly, you need both the .DLL and what is known as an "import library" (in .LIB format). The import library is linked with at link time. *Stubs* for each exported symbol of the DLL gets linked to the program.
These stubs then get updated as the EXE and the DLL gets loaded into memory when the process launches (meaning that the stubs will actually point to the memory addresses of the functions as located in the DLL).

