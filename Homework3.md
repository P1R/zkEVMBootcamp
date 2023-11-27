# Homework 4

Some modular arithmetic

1. Working with the following set of Integers S = {0,1,2,3,4,5,6}

What is

  a) 4 + 4 = (4 + 4) % 7 = **1**
  
  b) 3 x 5 = (3 * 5) % 7 = **1**
  
  c) what is the inverse of 3 under

  i) addition

  To find the additive inverse of 3 in modulo 7 arithmetic, fist we need  to 
  find a number $x$ such that $(3 + x)\mod 7 = 0$.

  The additive inverse of $3$ is therefore $\boxed{4}$, since $3 + 4 = 7$ and 7
  is equivalent to 0 modulo 7.
      
  
  ii) multiplication

  The multiplicative inverse of a number $a$ modulo $m$ is a number $b$ such
  that $a \cdot b \mod m = 1$. For number 3 modulo 7, we need to find a $b$
  such that $3 \cdot b \mod 7 = 1$.

  The multiplicative inverse for 3 modulo 7 is $\boxed{5}$,
  as $3 \cdot 5 = 15$ and $15 \mod 7 = 1$.

2. For S = {0,1,2,3,4,5,6}
Can we consider 'S' and the operation '+' to be a group?

To determine if a set together with a specific operation forms a group, 
we must ensure that the set satisfies four group axioms: **Closure**, 
**Associativity**, **Identity Element**, **Invertibility**

Given the set $ S = {0, 1, 2, 3, 4, 5, 6} $ and assuming the operation is
addition modulo 7, we can check each of the group axioms:

**Closure**: Addition of any two elements in $ S $ will result in another
element in $ S $ when taken modulo 7, thus $ S $ is closed under addition 
modulo 7.

**Associativity**: Addition is associative under regular integer arithmetic,
and this property carries over to addition modulo 7.

**Identity Element**: The identity element for addition is $ 0 $ since adding
$ 0 $ to any element $ a $ does not change $ a $, and $ 0 $ is in the set $S$.

**Invertibility**: Every element $ a $ has an additive inverse $ b $ such that
$ a + b \mod 7 = 0 $. Since our set is a complete residue system modulo 7, each
element indeed has an inverse within the set.

***Considering these properties, $ S $ forms an additive group modulo 7.***

3. What is -13 mod 5? **2**

4. Polynomials

For the polynomial $(x^3 - x^2 + 4x − 12)$ 

Find a positive root

By using the Ruffini method and this [Ruffini](./ruffini) code writen in rust,
we get that a rational root is: **2**


What is the degree of this polynomial? **3**
