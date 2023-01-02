
# Lesson 1 : Numbers & binaries 


## Representation of data  
 In computing, we care about storing and transferring data ( information )

There are different types of data we may want to transfer :
- Boolean ( true / false )
- Integers and decimal numbers 
- Text ( charachters and strings )
- Structured data ( JSON, XML, HTML, ... )
- Binary data ( images, proprietary format)
 
Programming languages help us work with these formats

## Types of computing  
Electronic systems fall into 2 categories: 
- Analogue systems use current to represent data : 
    - E.g, 4-20mA current loop 
- Digital systems use pulses to encode data : 
    - Communication using a stream of 0s and 1s 
    - Each piece of information ( either 0 or 1 ) is called a bit
    - We can store bits in memory ( RAM , SSD ) or transmit them
    
E.g Modern computers are primarily digital systems with some exceptions ( e.g , microphone , audio output, etc.) 

Click on this [link](https://www.youtube.com/watch?v=WxJKXGugfh8) to get more explanation  

## Bits & Multiple bits 

A single bit is the smallest piece of data in a digital system.

It can be either 0 or 1, so it can store anything that has only 2 options. 

A single bit is not enough to encode a number ( such as 957 ) or a piece of text such as "Hello world" that is why we need multiple bits or a chain of bits . 

To make working with bits easier, we put several of them together (E.g 00 , 01 , 10 , 11 ) and each of them together encodes a state (which can be a number or plain text)


## Bytes 

Generally speaking, N bits allow us to encode 2^n states 

The most common value of N is 8, which allows us to encode 2^8 = 256 states

This data type is called a byte ( E.g 00001111 , 11010011 , 10101010 -> These are all bytes)

A single byte is quite restrictive because it only encodes 256 possible states. 

- Put 2 bytes together and you have 2^16 = 65536 possible states (Can store larger number for example screen coordinates ).
A two-byte number is sometimes called a 'short' .

- Put 4 bytes together and you have 2^32 = 4 billion diiferent possibilities ( enough for more purposes)
 
- Put 8 bytes together and you have 2^64  = 1.8 x 10^19 values .

## How to use the bits ? 

Use the bits to store whole non-negative ( unsigned ) numbers : 
    - 8- 16- 32- and 64 bits 

Use them to store whole ( signed , possibly negative ) numbers

- E.g : 
    - Unsigned 16-bit number in [0..65,535]
    - Signed 16-bit number in [-32,768..32,767]