# XOR-Encryption
Encrypt files and directories with the XOR cipher algorithm
# How this works
The XOR operator (^) is applied to every byte in a file. Which looks something like this  
`01101011`  
   XOR  
`10010010`  
    ==  
`11111001`  
so `107 ^ 146 == 249`  
Also  
`a ^ b == c`  
`c ^ b == a`  
Therefore `(a ^ b) ^ b == a`  
Every operation can be reversed with the same key/password. Here the input file (a) will get encrypted with the key (b) and the result will be the same directory but encrypted (c).
If c gets encrypted with b then the output will always be a.
