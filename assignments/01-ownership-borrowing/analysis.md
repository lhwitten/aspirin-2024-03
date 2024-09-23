Find in string

Find in string requires lifetimes because we are trying to return references to
local variables. The data from the strings comes from one of two string slices
input into the function, one of these string slices must own the output
variable. We need to specify lifetimes because if data can be owned by one of
multiple things then we could end up deallocating wrong or not deallocating at
all. No dangling pointers. The compiler cannot know how long a piece of data can
live if the lifetime is not specified.

Doubly linked stack

A doubly linked stack is not possible to implement in Rust. A key issue here is
that two variables cannot own each other. If you were trying to push to a doubly
linked stack, you could very easily create a new node and add the downstream
node, and replace the head. The issue comes in when you try to add the link in
the other direction. In order to get to the second node of the doubly linked
stack, you need to traverse it. Currently the second node is owned by the first
making it unreadable and unwritable - we cannot traverse to it then and add the
above node as an uplink.

amit approved explanation!
