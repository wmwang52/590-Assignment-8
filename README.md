Team Members:
Cora Rogers, Dylan Nicks, William Wang

## How to Run

1. **Navigate to the Project Directory**
   ```bash
   cd A8
   ```

2. **Compile the Program**
   ```bash
   rustc main.rs
   ```

3. **Run the Program**
   ```bash
   ./main
   ```

---

## Commands to Test the Program

### Arithmetic Operations (`serv1`)
- Perform addition:
  ```plaintext
  add 5 3
  ```
  Output: `(serv1) 5 add 3 = 8`

- Perform subtraction:
  ```plaintext
  sub 10 4
  ```
  Output: `(serv1) 10 sub 4 = 6`

- Perform multiplication:
  ```plaintext
  mult 7 6
  ```
  Output: `(serv1) 7 mult 6 = 42`

- Perform division:
  ```plaintext
  div 9 3
  ```
  Output: `(serv1) 9 div 3 = 3`

- Handle division by zero:
  ```plaintext
  div 9 0
  ```
  Output: `(serv1) Error: Division by zero`

- Negation:
  ```plaintext
  neg 5
  ```
  Output: `(serv1) neg 5 = -5`

- Square root:
  ```plaintext
  sqrt 16
  ```
  Output: `(serv1) sqrt 16 = 4`

---

### List Operations (`serv2`)
- Sum of integers:
  ```plaintext
  [1,2,3,4]
  ```
  Output: `(serv2) Sum of numbers: 10`

- Product of floats:
  ```plaintext
  [1.5,2.0,3.0]
  ```
  Output: `(serv2) Product of numbers: 9`

---

### Error Handling (`serv3`)
- Handle errors:
  ```plaintext
  {error, "Invalid input"}
  ```
  Output: `(serv3) Error: Invalid input`

- Handle unknown messages:
  ```plaintext
  random_message
  ```
  Output: `(serv3) Not handled: Unknown("random_message")`

---

### Halt the Program
To terminate the program, enter:
```plaintext
all_done
```
Output:
```plaintext
Sending shutdown signal...
Waiting for servers to shut down...
(serv1) Halting.
(serv2) Halting.
(serv3) Halting. Unprocessed messages count: [number]
All servers have shut down. Main process exiting.
```
