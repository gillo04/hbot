# HBot
HBot is a coding strategy game where you battle with robots, until only one remains standing. You program robts in an assembly-like language and then let them loose on the playing field.

## State of the project
At the moment, HBot is still in very early development. You can run a few programs and see how robots interact together, but it's not anywhere near what the end goal is.

## Build and run
HBot can be built using [cargo](https://www.rust-lang.org/tools/install), by simply running:
    
    cargo build
    
If you want to build and run HBot at the same time, use:
    
    cargo run
    
Alternatively, you can download one of the releases on the github page, but it's not guaranteed they will be up to date.

# The HBot language
The HBot language is an assembly-like programming language, which is used to program your battle robots. The instructions get executed in order from the first to the last. Once the program counte arrives at the last instruction, it loops back to the first instruction. You can control the flow of the program with branching instructions such as `jmp` and `je`.

To avoid robots behaving in different ways based on their programs are stepped during a battle, some instructions get executed in a specific order: all instructions relating to the motor (rotations and move forewad) get executed before all turret operation (shooting).

## Default commands and registers
Here is a list of the basic instructions the robot can execute and the general purpose registers it has access to. Generally instructions are in the form `<opcode> <destination>, <source>`. In this reference we'll use the following symbols to indicate what operand can be:
- `REG`: register
- `IMM`: immediate value 
- `ID`: identifier (label) 
- `VAL`: either a register, an identifier or an immediate value 

### Instructions
- `nop`: does nothing, waits for a clock cycle.
- `mov REG, VAL`: copies the value in source to the destination register. In case of a label, it copies the address it refers to.
- `add REG, VAL`: adds the value in source to the destination register. 
- `sub REG, VAL`: subtracts (with two's complement) the value in source from the destination register.
- `cmp REG, VAL`: same as the sub instruction, but the result isn't saved in the destination register (only the flags are affected). 
- `and REG, VAL`: performs bitwise and on the value in source and the destination register, storing the result in destination. 
- `or REG, VAL`: performs bitwise or on the value in source and the destination register, storing the result in destination. 
- `xor REG, VAL`: performs bitwise xor on the value in source and the destination register, storing the result in destination. 
- `not REG`: performs bitwise not on the destination register.

- `jmp VAL`: branches to the specified address (absolute address).
- `je VAL`: branches to the specified address if the `e` flag is set.
- `jg VAL`: branches to the specified address if the `g` flag is set.
- `jl VAL`: branches to the specified address if the `l` flag is set.

### Registers
- `a`, `b`, `c`: general purpose registers to be used by the program however it pleases.
- `ip`: read-only register, holds the pointer to the current instruction.
- `flags`: holds ALU flags. Can neither be read or written to. The flags of the ALU are the following:
    - `e`: is set if the last ALU operation (arithmetic or logic) yielded a zero result.
    - `g`: is set if the last ALU operation (arithmetic or logic) yielded a result grater than zero.
    - `l`: is set if the last ALU operation (arithmetic or logic) yielded a result lesser than zero.

## Components
Each component makes available to the robot a new set of commands and registers that can be used in the program. Here is a list of all components and the associated registers and instructions.

### Motor
This component allows the robot to move on the field. It enables the following instructions:
- `fwd`: moves the robot forward of one tile in the direction it is facing.
- `rol`: rotates the robot to it's left of 90 degrees.
- `ror`: rotates the robot to it's right of 90 degrees.
This component doesn't add any registers to the robot.

### Turret
This component allows the robot to shoot at other robots, inflicting damage. It enables the following instructions:
- `sht`: if a bullet is loaded, it shoots in the direction the robot is facing. 
- `rld`: loads a bullet in the turret.

### Sensors
This component allows the robot to get information on it's surroundings.
- `rad`: radar, gets the direction of the closest enemy. It puts a 1 in the `dir` register if you have to turn right to reach it, a 0 if you are facing it and a -1 if you have to turn left to reach it.
- `chk`: checks if an enemy is inside the area of influence. I there's an enemy in the area of influence, it updates the `cx` and `cy` registers to the absolute position on the field of the enemy robot, otherwise it updates them both to -1.
## Example programs
Here are some example robots to get an idea how to write the HBot language.
### space_invader 
Makes the robot go to the right, then shoot forward in a loop, until it hits the right map boundary, then it turns around and does the same going left.
```
rol
main:
  mov a, 10
  ror
  ror
  right:
    fwd
    rol
    sht
    rld
    ror
    sub a, 1
    jg right

  mov a, 10
  rol
  rol
  left:
    fwd
    ror
    sht
    rld
    rol
    sub a, 1
    jg left

jmp main
```
