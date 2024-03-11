# zktsim

## PLONKish arithmetization table

| Gate inputs and output | Wire assignments | Gate definition | Expected input and output |
| ---------------------- | ---------------- | --------------- | ------------------------- |

**Gate inputs and output subtable**

| i_e_g                | e_g         | g         | l_idx            | l_val            | r_idx             | r_val             | o_idx        | o_val        |
| -------------------- | ----------- | --------- | ---------------- | ---------------- | ----------------- | ----------------- | ------------ | ------------ |
| Fixed                | Advice      | Advice    | Advice           | Advice           | Advice            | Advice            | Advice       | Advice       |
| Internal enable gate | Enable gate | Gate type | Left input index | Left input value | Right input index | Right input value | Output index | Output value |

**Wire assignments subtable**

| i_e_w                           | idx        | val        |
| ------------------------------- | ---------- | ---------- |
| Fixed                           | Fixed      | Advice     |
| Internal enable wire assignment | Wire index | Wire value |

**Gate definition subtable**

| i_e_g_def                      | g_def               | l_def            | r_def             | o_def                  |
| ------------------------------ | ------------------- | ---------------- | ----------------- | ---------------------- |
| Fixed                          | Fixed               | Fixed            | Fixed             | Fixed                  |
| Internal enable gate defintion | Gate type to define | Left input value | Right input value | Resultant output value |

**Expected input and output subtable**

| e_i_o        | i_o_val               |
| ------------ | --------------------- |
| Instance     | Instance              |
| Enable value | Input or output value |

## Constraints

* Logic gates satisfied
  
  ```
  (i_e_g * e_g, g, l_val, r_val, o_val) 
      ∈ (i_e_g_def, g_def, l_def, r_def, o_def);
  ```

* Wire assignments satisfied
  
  ```
  (i_e_g * e_g, l_idx, l_val) ∈ (i_e_w, idx, val);
  (i_e_g * e_g, r_idx, r_val) ∈ (i_e_w, idx, val);
  (i_e_g * e_g, o_idx, o_val) ∈ (i_e_w, idx, val);
  ```

* Input/output constraints satisfied
  
  ```
  e_i_o * (val - i_o_val) == 0;
  ```

## Hash function - MiMC7 CBC encryption

* Block size = 1 field element = 255 bits (BLS12-381 scalar field size)
* Num rounds = `ceil(log(2**255, 7))` = 91
* One block corresponds to 4 circuit netlist rows
* Gate encoded as 3 bits and wire indexes encoded as 20 bits
  * Size of 4 circuit netlist rows = `(3 + 20 * 3) * 4` = 252 bits
* Gate value already constrained to 3 bits because of the lookup in the Gate Definition Table
* Wire indexes already constrained to be less than the circuit hyperparameter W; thus W must be `<= 2**20`

**MiMC7 CBC Arithmetization Table**

| s                          | k      | iv                   | x_in         | x_0       | ...                 | x_91       | x_out         |
| -------------------------- | ------ | -------------------- | ------------ | --------- | ------------------- | ---------- | ------------- |
| Selector                   | Advice | Advice               | Advice       | Advice    | Advice              | Advice     | Advice        |
| Enable cipher every 4 rows | Key    | Initialization value | Cipher input | Aux input | Intermediate values | Aux output | Cipher output |

**Constraints**

* Round function
  
  ```
  s * ((x_[i] + c_[i] + k) ** 7 - x_[i+1]) == 0 for i in 0..91;
  ```

* Cipher input/output
  
  ```
  s * (x_in + iv - x_0) == 0;
  s * (x_91 + k - x_out) == 0;
  ```

* Key copy 
  
  ```
  // same k copied throughout the column 
  ```

* Initialization value copy
  
  ```
  // iv == 0 for the first row
  // iv == (x_out from the previous row) for the remaining rows
  ```

## Hashing the circuit netlist

* Sample a random key K
* Hash K using the [LP231Ap](https://github.com/zcash/zcash/issues/2233#issuecomment-291840857) construction with the MiMC block cipher
* Expose public this hashed value of K 
* Encrypt the circuit netlist with MiMC7 CBC encryption using the key K
* Expose public the encrypted circuit netlist

**Arithmetization**

* Circuit netlist encryption
  
  * Instantitate MiMC7 CBC encryption arithmetization table
  
  * Set `e_c = i_e_g`
  
  * Constrain 
    
    ```
    // s_i_e := selector input encode; enabled every range(0, G, 4)
    l0 := g + l_idx * 2**3 + r_idx * 2**23 + o_idx * 2**43;
    l1 := g[+1] + l_idx[+1] * 2**3 + r_idx[+1] * 2**23 + o_idx[+1] * 2**43;
    l2 := g[+2] + l_idx[+2] * 2**3 + r_idx[+2] * 2**23 + o_idx[+2] * 2**43;
    l2 := g[+3] + l_idx[+3] * 2**3 + r_idx[+3] * 2**23 + o_idx[+3] * 2**43;
    s_i_e * (l0 + l1 * 2**63 + l2 * 2**126 + l3 * 2**189 - x_in) == 0;
    ```
