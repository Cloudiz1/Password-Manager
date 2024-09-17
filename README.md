## Password Manager

This is a simple password manager that features AES encryption in CBC operation mode and uses SHA256 to verify logins. Features include:
- encryption
- login screen
    - password of arbitrary length to access credentials
- a toggle to show logins (who knows man just in case)
- adding and removing credentials 
- ability to change login password in the app

## How to Use
On first use, the program generates a `database` directory which contains an initialization vector (IV), a key, a hashed password, and encrypted credentials. The programs prompts for a login password on first use. 
<br> <br>
A random key and IV are generated on first use. The encrypted `logins.txt` will begin as an empty file that is encrypted and the program will add to it as the password manager is used. 
<br> <br>
Simply run `cargo run` to start.

## Future Thingys
- edit credentials instead of deleting and remaking them
- finish that ugly login screen lol
- make errors and prompts pop on screen instead of in a terminal
