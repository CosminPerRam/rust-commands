# rust-commands
Attempting to create a command completion system.  

Ex:  
Structure: `console <clear|close>`  
Input: `c`  
Completion: `console`  

ex:  
Structure: `add_u8 <u8> <u8>`  
Input: `add_u8 15`  
Completion: `any u8` (the second u8)  
Input: `add_u8 256`  
Completion: `<none>` (cause the first u8 is not a valid u8)  
