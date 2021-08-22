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
You can also encrypt the same directory with multiple keys as  
`a ^ b ^ c ^ b ^ c == a`  
The longer the key and the more keys you use the safer the encryption will be.
# Usage
This is a Command Line Interface Application.
It takes 2 or 3.  
`/directory/with/compiled/xor_encryption "/input/directory" "key" ["/output/directory"]`  
if the output directory is not specified  
   it will encrypt the input file/directory in place  
otherwise  
   the input file will not change and an encrypted file will be generated inside the output directory
