# Phần 1. #

## Khởi tạo ứng dụng: ##

```cmd
cargo init
```

## Tạo khối: ##

```cmd
touch src/block.rs
```

## Rust thêm thư viện serde: ##

```cmd
cargo add serde --features derive
```

## Khai báo cấu trúc khối: ##
```rust
struct Block:
 - timestamp: u128,
 - transactions: String,
 - prev_block_hash: String,
 - hash: String,
 - height: usize,
 - nonce: i32,
```

## Triển khai lệnh trên khối: ##
```rust
impl Block:
 - fn get_prev_hash(&self) -> String //lấy khối ở trước
 - fn get_hash(&self) -> String //láy khối hiện tại
 - fn new_genesis_block() -> Block //tạo khối nguyên thủy
 - fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> //tạo khối mới
 - fn run_proof_if_work(&mut self) -> Result<()> //đào khối
 - fn prepare_hash_data(&self) -> Result<Vec<u8>> //chuân bị khối để khởi tạo
 - fn validate(&self) -> Result<bool> //xác thực khối
```

## Rust thêm thư viện Log failure bincode: ##
```cmd
cargo add log failure bincode
```

## Tạo kiểu báo lỗi: ##
```rust
type Result<T> = std::result::Result<T, failure::Error>;
```

## Tạo xử lý chuỗi khối: ##
```cmd
touch src/blockchain.rs
```

## Rust thêm thư viện sled (DB) lưu chuỗi khối: ##
```cmd
cargo add sled
```

## Khai báo cấu trúc chuỗi khối: ##
```rust
struct Blockchain
 - current_hash: String,
 - db: sled::Db,
```

## Khai báo cấu trúc vòng lặp chuỗi khối: ##
```rust
struct BlockchainIter<'a> 
 - current_hash: String,
 - bc: &'a Blockchain,
```

## Triển khai lệnh trên chuỗi khối: ##
```rust
impl Blockchain:
 - fn new() -> Result<Blockchain> //tạo chuỗi khối
 - fn add_block(&mut self, data: String) -> Result<()> //thêm khối vào chuỗi khối
 - fn iter(&self) -> BlockchainIter //vòng lặp chuỗi khối 
```

## Triển khai lệnh trên vòng lặp chuỗi khối: ##
```rust
impl<'a> Iterator for BlockchainIter<'a>:
 - fn next(&mut self) -> Option<Self::Item> //tạo chuỗi khối mới
```
