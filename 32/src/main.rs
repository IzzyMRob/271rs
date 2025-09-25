// Notes for Macros Assignment:
// the macros file is not recognized
// hash functions:
//  deterministically compress long input to shorter output(digest/signature)
//  can hash anything in a computer
//  compression/one-way hash - only make smaller - can't infer og data
//
// measure goodness:
//  infeasable to find collisions
//  signature of ubuntu is always signature of unbutu
//  means you can see if malware is different signature
//  
// math:
//  let M be the size of a hash table
//  a natrual number a : a< M and gcd(a,M)=1
//  aka two positive co-prime integers
//  any integer x can map to:
//   N/M = {0,1,...,M-1}
//  avoid collisions with large number of possible hash values
//
// Sha-1 from 1995 had vulnerabilities
// Sha-2 still used, secure in 2025, digital signatires, version control,
// Sha-3 from 2015 not widly used
// August 2001 - Sha-224, Sha-256, Sha-384, Sha-512, Sha-512/224, Sha
//
// Sha-2 with 512 bit output length
//  accepts bit strings up to length 2^128 -1
//  340 undecillion bits
//  34 septillion terabytes
//
// preprocessing and computation
//  initialize with 512 bits - 8x words of 64 bits each
//  input is padded to length that is a multiple of 1024
//  let M be l bits long
//  k + l + 129 congruent 0 (mod 512)
//
// shift/rotate
//  shift right >>
//  move all values in direction x times
//  shift replaces values with 0 or 1, which is better??
//  rotate wraps around to beginning
//
//  order to impliment:
//   preprocess
//   set message schedule array - 80 working ariables = 8 word sized variables
//   main loop - word level operations
//   update hash value
//
//  set working variables:
//   ABCDEFGH = 
//  iterate:
//
// Izzy Robbins

fn main() {
     let a : [u64; 4] = [0x1111111111110000, 0x1111000011001100, 0x1100110010101010, 0x0123456789ABCDEF];
    println!("*Rotates use a decimal shift value, but print in hexadecimal:\n");
    println!("choice(\n{:016X},\n{:016X},\n{:016X}) = \n--------\n{:016X}\n\n", a[0], a[1], a[2], macros::choice!(a[0], a[1], a[2]));
    println!("median(\n{:016X},\n{:016X},\n{:016X}) = \n--------\n{:016X}\n\n", a[0], a[1], a[2], macros::median!(a[0], a[1], a[2]));
    println!("*Rotates use a decimal shift value, but print in hexadecimal:\n");
    println!("rotate!(\n{:016X}, 04) = \n--------\n{:016X}\n\n", a[3],   macros::rotate!(a[3], 4));
    println!("rotate!(\n{:016X}, 08) = \n--------\n{:016X}\n\n", a[3],   macros::rotate!(a[3], 8));
    println!("rotate!(\n{:016X}, 12) = \n--------\n{:016X}\n\n", a[3],   macros::rotate!(a[3], 12));
    println!("rotate!(\n{:016X}, 02) = \n--------\n{:016X}\n\n", 0x1000, macros::rotate!(0x1000_u64, 2));
    println!("rotate!(\n{:016X}, 30) = \n--------\n{:016X}\n\n", 0x1000, macros::rotate!(0x1000_u64, 30));
}
