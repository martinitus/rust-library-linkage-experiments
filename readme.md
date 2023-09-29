# How does linkage and dynamic loading of libraries effect static global variables?

This repo contains 3 libraries and one executable linking to those libraries.
The dependency graph looks as follows:

```
      ┌─────┐
      │     │
   ┌──┤  A  ├─┐
   │  │     │ │
   │  └─┬───┘ │
   │    │     │
┌──▼──┐ │ ┌───▼─┐
│     │ │ │     │
│  B  │ │ │  C  │
│     │ │ │     │
└┬────┘ │ └───┬─┘
 │      │     │
 │ ┌────▼───┐ │
 │ │        │ │
 └─►  exe   │◄┘
   │        │
   └────────┘
```

I.e.: 
- B links against A
- C links against A
- exe links against A
- exe links against B
- exe links against C

Library A contains a static global variable and a minimal api to read and write its value.
This value will be changed and read from the executable direcltly via A, and indirectly via B or C.
Further, the executable loads B via libloading at runtime and changes the value in A via the dynamically loaded B.

The output on my machine (windows) is as follows:

```
exe sees: 0
b sees: 0
c sees: 0
================
setting to 1 directly via a    
exe sees: 1
b sees: 1
c sees: 1
================
setting to 2 via b
exe sees: 2
b sees: 2
c sees: 2
=======DYNAMIC LOADING=========
b-dyn-loaded sees: 0
setting to 3via dyn-loaded-b   
exe sees: 2
b sees: 2
b-dyn-loaded sees: 3
c sees: 2
```

__This shows, that even with dynamic linking, the dynamically loaded A (via B) still has its own memory region for the 
static variable.__


