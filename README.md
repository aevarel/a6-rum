# Overview

Authors: Kai Maffucci and Aeyva Rubelo

This took about 20 hours total to complete, with a couple down to analyzing, a couple for designing, and the rest for implementation and debugging. 

Thanks to Dr. Noah Daniels for his rumdump lab from CSC 411, as we used the lab's code for reading the UM binary file and loading it into memory. 

Our UM is fully functional as far as we know, as it was able to run all of the binaries from here: git clone https://github.com/ndaniels/rum-binaries.git

Notable departures:
- The `halt` function is no longer in use and is replaced with a simple process::exit call
- 

# Architecture

## Registers

We have an 8-item long array of u32s called `r` which emulates virtual registers r[0]-r[7]. 

## Memory

A 2d vector (a vector of vectors) of u32s called `m` represents virtual memory segments. Identifiers are the index used to determine the location of the inner vector. We also have a vector of u32s `saved_ids` which stores segment IDs we can re-use and need to store. 

## Main program loop

The program logic is stored within main.rs in a loop. We have a program counter `pc` which points to the current instruction at offset m[0][pc], which is the current/next instruction being executed. In this loop we extract the opcode to determine which operation to perform. Most operations are contained within their own designated functions for that specific operation. These functions are passed mutable references to r, m, etc as needed. The state of the UM is altered by the side effects of each of these functions. So these other functions are all tethered to main, but not to each other. The one execption of that is word.rs, which we use to extract the opcode and register IDs from each instruction word. 

# 50 Million Instructions

# Error Code Table

| Error Code # | Opcode # | Referenced Operation | Usage/Indication |
|--------------|----------|---------------------|-----------------|
|140              |n/A          |n/A                     |User called non-existent opcode (Usage: 0<= opcode# <= 13)                 |
|     |           |                      |                 |
|              |          |                     |                 |
