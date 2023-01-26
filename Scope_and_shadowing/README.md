### Question 1 : What is Scope ? and what is shadowing ? 

Scope and shadowing are related but distinct concepts in programming.

Scope refers to the region of the code where a variable, function, or other identifier is visible and can be accessed. A variable that is defined within a particular block of code (such as a function or loop) is said to have local scope, and can only be accessed within that block of code. A variable that is defined outside of any block of code (such as at the module level or in the global scope) is said to have global scope, and can be accessed from anywhere in the program.

Shadowing, on the other hand, refers to the ability to create a new variable with the same name as an existing variable, within a specific scope. When a new variable is created that shadows an existing variable, the new variable takes precedence over the existing variable within the scope where it is defined, and any references to that variable within that scope will refer to the new variable.

For example, if you have a global variable x and inside a function you define a new variable x, the global variable will be shadowed by the local variable inside the function. The local variable is said to shadow the global variable.

It is important to note that shadowing does not affect the lifetime of the original variable, only the accessibility of it within the scope where the new variable is defined.

In summary, scope refers to the region of the code where an identifier can be accessed, while shadowing refers to the ability