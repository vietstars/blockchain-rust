# blockchain-rust

## Create first Wallet 
Cargo run createwallet 

## Create second Wallet 
Cargo run createwallet 

## List address
Cargo run listaddresses
```cmd
addresses: 
3A4Kzbf56oJ6CXLBebe4iTKvWEPPzxGF3J
3M7yUwRz8T8a7gajHPxqZTu8i4WvwrRGZP
``` 

## Create Genesis block to Wallet (get 100)
Cargo run create 3A4Kzbf56oJ6CXLBebe4iTKvWEPPzxGF3J

## send 25 coins from 2 wallets
Cargo run send 3A4Kzbf56oJ6CXLBebe4iTKvWEPPzxGF3J 3M7yUwRz8T8a7gajHPxqZTu8i4WvwrRGZP 25

## list chain
Cargo run printchain 


## check balance
Cargo run getbalance 3A4Kzbf56oJ6CXLBebe4iTKvWEPPzxGF3J
```cmd
Balance of '3A4Kzbf56oJ6CXLBebe4iTKvWEPPzxGF3J'; 75 
```

Cargo run getbalance 3M7yUwRz8T8a7gajHPxqZTu8i4WvwrRGZP
```cmd
Balance of '3M7yUwRz8T8a7gajHPxqZTu8i4WvwrRGZP'; 25 
```
