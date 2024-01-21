# ethereum-address-grinder 1.0.0
Bruteforce an ethereum address to match certain pattern. By default, it finds an address
that has __6 consequent symbols__ in the beginning, but you can change the source code to match any
pattern you would like.  
By default, the app uses __10 threads__ to find an address. This can also be changed in the source code.  
After the address is found, the private key will be saved into a file named with address. The private
key will be in a hex format.