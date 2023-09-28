# blockchain-rust

## create first Account (ben) (default 100 coins)
Cargo run create "ben" 

## send 50 coins from ben to master account
Cargo run send ben master 40

## send 25 coins from ben to vietstars account
Cargo run send ben vietstars 25

## send 25 coins from ben to vietstar.nt account
Cargo run send ben vietstar.nt 25

## send 15 coins from master to ben account
Cargo run send master ben 15

## list chain
Cargo run printchain 


## check balance
Cargo run getbalance ben
```cmd
Balance of 'ben'; 25 
```

Cargo run getbalance master
```cmd
Balance of 'master'; 25 
```

Cargo run getbalance vietstars
```cmd
Balance of 'vietstars'; 25 
```

Cargo run getbalance vietstar.nt
```cmd
Balance of 'vietstar.nt'; 25 
```
