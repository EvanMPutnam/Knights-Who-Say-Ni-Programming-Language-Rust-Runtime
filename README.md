# The ni-programming language runtime

The (un)-official programming language used exclusively by the
knights who say ni.

For more context see below video (I claim no ownership):
[![Ni](https://img.youtube.com/vi/zIV4poUZAQo/0.jpg)](https://www.youtube.com/watch?v=zIV4poUZAQo)

# Runtime Description

The Ni rust runtime is almost identical to that of another esoteric programming language, which contains an expletive in
the [title](https://en.wikipedia.org/wiki/Brainfuck).

The runtime interpreter configures a sequential storage array of bytes that has a single data pointer that can be
incremented and decremented. Think of it as a long "tape" of bytes. There is also an instruction pointer that can be
used to perform simple jumps of logic.
Lastly, characters can be read from stdin and placed at the data pointer. They can then be written out to stdout as
ASCII.

## Instruction Set

| Instruction | Description                                                                                                   |
|-------------|---------------------------------------------------------------------------------------------------------------|
| Niii        | If the byte value at the data pointer is zero, jump instruction pointer to end of matching `niii` statement.  |
| niii        | If the byte value at the data pointer is not zero, jump instruction pointer to the matching `Niii` statement. |
| Nii         | Output the byte at the data pointer as ASCII.                                                                 |
| nii         | Accept a byte from stdin and store it at the data pointer.                                                    |
| Ni!         | Increment the byte at the data pointer by 1.                                                                  |
| ni!         | Decrement the byte at the data pointer by 1.                                                                  |
| Ni          | Increment the data pointer by 1.                                                                              |
| ni          | Decrement the data pointer by 1.                                                                              |

There is no support for comments.

## Example Program

The following program says `Hello World!` in Ni.

```
Ni! Ni! Ni! Ni! Ni! Ni! Ni! Ni! Niii Ni Ni! Ni! Ni! Ni!
Niii Ni Ni! Ni! Ni Ni! Ni! Ni! Ni Ni! Ni! Ni! Ni Ni! ni ni ni ni ni! niii
Ni Ni! Ni Ni! Ni ni! Ni Ni Ni!
Niii ni niii ni ni! niii Ni Ni Nii Ni ni! ni! ni!
Nii Ni! Ni! Ni! Ni! Ni! Ni! Ni! Nii Nii Ni! Ni! Ni!
Nii Ni Ni Nii ni ni! Nii ni Nii Ni! Ni! Ni! Nii ni! ni! ni! ni! ni! ni!
Nii ni! ni! ni! ni! ni! ni! ni! ni! Nii Ni Ni Ni! Nii Ni Ni! Ni! Nii 
 
```

## Files

Input files MUST end in either `.ni` or `.nii`.

## Runtime Configuration

### Debugging

Set the environment variable of `NI_DEBUG=true`

### Increase Memory Size

Set the environment variable of `NI_STORAGE=1024` where the number is in bytes.

# Sample Programs

A list of sample programs has been provided in the `sample-programs` directory. Please note that these were originally
bf programs that were translated to `.ni` files with the `bf_to_nil.py` file in the `utils` directory. Proper credit
goes to the original creators linked below.

## [Conways game of life](http://www.linusakesson.net/programming/brainfuck/)

- `sample-programs/conway.ni`
- Enter letters to initialize the board. For example cc will put row, column cc. Hit enter to go to next stage.

## [Sierpinski](http://brainfuck.org)

- `sample-programs/sierpinski.ni`
- Generates the magical sierpinksi triangle.

TODOS

- Environment variable config.
- Tests