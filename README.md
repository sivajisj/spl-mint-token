
---

## 📁 Project Structure

```

anchor-escrow/
│
├── programs/
│   └── anchor_escrow/
│       ├── src/
│       │   ├── lib.rs              # Main program entry point
│       │   ├── state.rs            # Account and state definitions
│       │   ├── errors.rs           # Custom error definitions
│       │   └── instructions/       # Instruction handlers
│       │       ├── make.rs         # Make escrow (create & deposit Token A)
│       │       ├── take.rs         # Take escrow (deposit Token B & finalize)
│       │       └── refund.rs       # Refund escrow (cancel & refund Token A)
│       └── Cargo.toml
│
├── Anchor.toml                     # Anchor configuration
├── Cargo.toml                      # Workspace dependencies
└── README.md                       # Project documentation

````

---

## 🚀 Features

- **Create Escrow (Make):**  
  User deposits Token A into a vault and defines how much Token B they want in return.

- **Accept Escrow (Take):**  
  Counterparty transfers Token B to the maker and receives Token A from the vault.

- **Refund Escrow (Refund):**  
  If the trade is not accepted, the maker can refund and retrieve Token A.

---

## ⚙️ Prerequisites

Before running this program, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/getting_started/installation.html)
- A local Solana wallet (`solana-keygen new`)

---

## 🧩 Setup & Build

### 1. Clone the Repository
```bash
git clone https://github.com/yourusername/anchor-escrow.git
cd anchor-escrow
````

### 2. Configure Solana Cluster

Use **localnet** for testing:

```bash
solana config set --url localhost
```

### 3. Start Local Validator

```bash
solana-test-validator
```

### 4. Build the Program

In another terminal:

```bash
anchor build
```

This will:

* Compile your program.
* Generate a new program ID.
* Output `.so` and keypair files in `target/deploy`.

---

## 🔑 Updating Program ID

After building, Anchor generates a new program keypair at:

```
target/deploy/anchor_escrow-keypair.json
```

To fix the `DeclaredProgramIdMismatch` error, follow these steps:

1. Open the file:

   ```
   programs/anchor_escrow/src/lib.rs
   ```
2. Update the `declare_id!` line with the new ID shown in your `target/deploy/anchor_escrow-keypair.json` file:

   ```rust
   declare_id!("YOUR_NEW_PROGRAM_ID_HERE");
   ```
3. Then rebuild:

   ```bash
   anchor build
   ```

This ensures your **declared ID** matches your **deployed program ID**.

---

## 🧪 Testing Locally

Run Anchor tests using:

```bash
anchor test
```

This will:

* Spin up a local Solana validator.
* Deploy the program.
* Execute your test scripts in `tests/` folder.

---

## 🧰 Common Error

### ❌ `DeclaredProgramIdMismatch`

**Error Code:** `0x1004`
**Message:**

```
The declared program id does not match the actual program id.
```

**Cause:**
The program ID in `declare_id!()` doesn’t match the one Anchor generated in `target/deploy`.

**Fix:**
Update the ID as shown above under “Updating Program ID”.

---

## 🧠 Learning Resources

* [Anchor Framework Book](https://book.anchor-lang.com/)
* [Solana Developer Docs](https://docs.solana.com/developing)
* [Solana Cookbook](https://solanacookbook.com/)

---

## 📜 License

This project is licensed under the **MIT License**.
Feel free to use and modify for educational or commercial purposes.

---

**Author:** [Your Name or GitHub Handle]
**Program ID:** `22222222222222222222222222222222222222222222` (replace after build)

```

---

Would you like me to include example test cases (in `tests/anchor_escrow.rs`) to show how to call `make`, `take`, and `refund` using Anchor’s `ProgramTest` framework?
```
