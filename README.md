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
