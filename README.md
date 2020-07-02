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
# Usage
This is a Command Line Interface Application.
It takes 3 or 4 arguments.  
`C:\directory\with\xor_encryption.exe "C:\input\directory" "C:\output\directory" "key" [-m/-d]`  
`-m` will encrypt on the spot. The encoded files will only be moved to the output directory temporarily and the encoded files will later be moved to the input directory. 
The temporary output directory will be deleted.  
`-d` will keep the encrypted files in the output directory and will delete everything in the input directory.  
` ` if left as blank nothing will be deleted.  
