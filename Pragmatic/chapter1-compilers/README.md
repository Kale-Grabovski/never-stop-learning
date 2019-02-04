Compilers are generally structured as a series of phases. The first few phases— 
scanning, parsing, and semantic analysis—serve to analyze the source program. 
Collectively these phases are known as the compiler’s front end. The final few 
phases—target code generation and machine-specific code improvement—are 
known as the back end. They serve to build a target program—preferably a fast 
one—whose semantics match those of the source. Between the front end and the 
back end, a good compiler performs extensive machine-independent code imrovement; the phases of this “middle end” typically comprise the bulk of the 
code of the compiler, and account for most of its execution time.
